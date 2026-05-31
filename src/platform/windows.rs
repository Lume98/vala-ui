use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr::{null, null_mut};

use windows_sys::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{
    COLOR_WINDOW, DEFAULT_GUI_FONT, GetStockObject, UpdateWindow,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    BN_CLICKED, BS_PUSHBUTTON, CREATESTRUCTW, CW_USEDEFAULT, CreateWindowExW, DefWindowProcW,
    DestroyWindow, DispatchMessageW, GWLP_USERDATA, GetClientRect, GetMessageW, HMENU, IDC_ARROW,
    LoadCursorW, MSG, PostQuitMessage, RegisterClassW, SW_SHOW, SendMessageW, SetWindowLongPtrW,
    ShowWindow, TranslateMessage, WM_COMMAND, WM_DESTROY, WM_NCCREATE, WM_NCDESTROY, WM_SETFONT,
    WNDCLASSW, WS_CHILD, WS_OVERLAPPEDWINDOW, WS_TABSTOP, WS_VISIBLE,
};

use crate::{ActionHandler, Element, Error, Result, Window};

const CLASS_NAME: &str = "RustUiWindow";
const ROOT_MARGIN: i32 = 16;
const COLUMN_GAP: i32 = 8;
const TEXT_HEIGHT: i32 = 24;
const BUTTON_HEIGHT: i32 = 32;
const FIRST_CONTROL_ID: i32 = 1000;
const ERROR_CLASS_ALREADY_EXISTS: u32 = 1410;

struct WindowsState {
    root: Element,
    actions: HashMap<i32, String>,
    next_control_id: i32,
    action_handler: Option<ActionHandler>,
}

impl WindowsState {
    fn new(root: Element, action_handler: Option<ActionHandler>) -> Self {
        Self {
            root,
            actions: HashMap::new(),
            next_control_id: FIRST_CONTROL_ID,
            action_handler,
        }
    }

    fn register_action(&mut self, action: impl Into<String>) -> i32 {
        let control_id = self.next_control_id;
        self.next_control_id += 1;
        self.actions.insert(control_id, action.into());
        control_id
    }
}

pub(crate) fn run_window(window: Window) -> Result<()> {
    let (title, width, height, root, action_handler) = window.into_parts();
    let instance = unsafe { GetModuleHandleW(null()) };

    if instance.is_null() {
        return Err(Error::windows_api("GetModuleHandleW"));
    }

    register_window_class(instance)?;

    let class_name = to_wide_null(CLASS_NAME);
    let title = to_wide_null(title);
    let state = Box::new(WindowsState::new(root, action_handler));
    let state_ptr = Box::into_raw(state);

    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            title.as_ptr(),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            null_mut(),
            null_mut(),
            instance,
            state_ptr.cast::<c_void>(),
        )
    };

    if hwnd.is_null() {
        unsafe {
            drop(Box::from_raw(state_ptr));
        }
        return Err(Error::windows_api("CreateWindowExW"));
    }

    let render_result = unsafe {
        let root = (*state_ptr).root.clone();
        render_root(hwnd, instance, &root, &mut *state_ptr)
    };

    if let Err(error) = render_result {
        unsafe {
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0);
            DestroyWindow(hwnd);
            drop(Box::from_raw(state_ptr));
        }
        return Err(error);
    }

    unsafe {
        ShowWindow(hwnd, SW_SHOW);
        UpdateWindow(hwnd);
    }

    run_message_loop()
}

fn register_window_class(instance: HINSTANCE) -> Result<()> {
    let class_name = to_wide_null(CLASS_NAME);
    let window_class = WNDCLASSW {
        style: 0,
        lpfnWndProc: Some(window_proc),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: instance,
        hIcon: null_mut(),
        hCursor: unsafe { LoadCursorW(null_mut(), IDC_ARROW) },
        hbrBackground: (COLOR_WINDOW + 1) as isize as _,
        lpszMenuName: null(),
        lpszClassName: class_name.as_ptr(),
    };

    let atom = unsafe { RegisterClassW(&window_class) };

    if atom == 0 {
        let code = unsafe { windows_sys::Win32::Foundation::GetLastError() };

        if code != ERROR_CLASS_ALREADY_EXISTS {
            return Err(Error::WindowsApi {
                operation: "RegisterClassW",
                code,
            });
        }
    }

    Ok(())
}

fn render_root(
    parent: HWND,
    instance: HINSTANCE,
    root: &Element,
    state: &mut WindowsState,
) -> Result<()> {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    if unsafe { GetClientRect(parent, &mut rect) } == 0 {
        return Err(Error::windows_api("GetClientRect"));
    }

    let width = (rect.right - rect.left - ROOT_MARGIN * 2).max(1);
    render_element(
        parent,
        instance,
        root,
        ROOT_MARGIN,
        ROOT_MARGIN,
        width,
        state,
    )?;
    Ok(())
}

fn render_element(
    parent: HWND,
    instance: HINSTANCE,
    element: &Element,
    x: i32,
    y: i32,
    width: i32,
    state: &mut WindowsState,
) -> Result<i32> {
    match element.name() {
        "column" => render_column(parent, instance, element, x, y, width, state),
        "text" => {
            create_control(
                "STATIC",
                element.text_content().unwrap_or_default(),
                parent,
                instance,
                x,
                y,
                width,
                TEXT_HEIGHT,
                WS_CHILD | WS_VISIBLE,
                null_mut(),
            )?;

            Ok(TEXT_HEIGHT)
        }
        "button" => {
            let control_id = action_attribute(element)
                .map(|action| state.register_action(action))
                .unwrap_or_default();

            create_control(
                "BUTTON",
                element.text_content().unwrap_or_default(),
                parent,
                instance,
                x,
                y,
                width,
                BUTTON_HEIGHT,
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | BS_PUSHBUTTON as u32,
                control_id as isize as HMENU,
            )?;

            Ok(BUTTON_HEIGHT)
        }
        _ => render_column(parent, instance, element, x, y, width, state),
    }
}

fn render_column(
    parent: HWND,
    instance: HINSTANCE,
    element: &Element,
    x: i32,
    y: i32,
    width: i32,
    state: &mut WindowsState,
) -> Result<i32> {
    let mut current_y = y;

    for child in element.children() {
        let height = render_element(parent, instance, child, x, current_y, width, state)?;
        current_y += height + COLUMN_GAP;
    }

    Ok((current_y - y - COLUMN_GAP).max(0))
}

#[allow(clippy::too_many_arguments)]
fn create_control(
    class_name: &str,
    text: &str,
    parent: HWND,
    instance: HINSTANCE,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    style: u32,
    menu: HMENU,
) -> Result<HWND> {
    let class_name = to_wide_null(class_name);
    let text = to_wide_null(text);

    let hwnd = unsafe {
        CreateWindowExW(
            0,
            class_name.as_ptr(),
            text.as_ptr(),
            style,
            x,
            y,
            width,
            height,
            parent,
            menu,
            instance,
            null(),
        )
    };

    if hwnd.is_null() {
        return Err(Error::windows_api("CreateWindowExW"));
    }

    unsafe {
        SendMessageW(
            hwnd,
            WM_SETFONT,
            GetStockObject(DEFAULT_GUI_FONT) as WPARAM,
            1,
        );
    }

    Ok(hwnd)
}

fn run_message_loop() -> Result<()> {
    let mut message = MaybeUninit::<MSG>::zeroed();

    loop {
        let result = unsafe { GetMessageW(message.as_mut_ptr(), null_mut(), 0, 0) };

        if result == -1 {
            return Err(Error::windows_api("GetMessageW"));
        }

        if result == 0 {
            return Ok(());
        }

        unsafe {
            TranslateMessage(message.as_ptr());
            DispatchMessageW(message.as_ptr());
        }
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match message {
        WM_NCCREATE => {
            let create_struct = lparam as *const CREATESTRUCTW;

            if !create_struct.is_null() {
                unsafe {
                    SetWindowLongPtrW(
                        hwnd,
                        GWLP_USERDATA,
                        (*create_struct).lpCreateParams as isize,
                    );
                }
            }

            unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
        }
        WM_COMMAND => {
            let notification = ((wparam >> 16) & 0xffff) as u16;
            let control_id = (wparam & 0xffff) as i32;

            if notification == BN_CLICKED as u16 {
                if let Some(state) = unsafe { state_from_window(hwnd) } {
                    if let Some(action) = state.actions.get(&control_id).cloned() {
                        if let Some(handler) = state.action_handler.as_mut() {
                            handler(&action);
                        }
                    }
                }

                return 0;
            }

            unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
        }
        WM_DESTROY => {
            unsafe {
                PostQuitMessage(0);
            }
            0
        }
        WM_NCDESTROY => {
            let state_ptr =
                unsafe { SetWindowLongPtrW(hwnd, GWLP_USERDATA, 0) } as *mut WindowsState;

            if !state_ptr.is_null() {
                unsafe {
                    drop(Box::from_raw(state_ptr));
                }
            }

            unsafe { DefWindowProcW(hwnd, message, wparam, lparam) }
        }
        _ => unsafe { DefWindowProcW(hwnd, message, wparam, lparam) },
    }
}

unsafe fn state_from_window(hwnd: HWND) -> Option<&'static mut WindowsState> {
    let state_ptr = unsafe {
        windows_sys::Win32::UI::WindowsAndMessaging::GetWindowLongPtrW(hwnd, GWLP_USERDATA)
    } as *mut WindowsState;

    unsafe { state_ptr.as_mut() }
}

fn action_attribute(element: &Element) -> Option<&str> {
    element
        .attributes()
        .iter()
        .find(|attribute| attribute.name() == "on_click")
        .map(|attribute| attribute.value())
}

fn to_wide_null(value: impl AsRef<str>) -> Vec<u16> {
    value
        .as_ref()
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect()
}
