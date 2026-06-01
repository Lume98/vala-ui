use std::collections::HashMap;
use std::ffi::c_void;
use std::mem::MaybeUninit;
use std::ptr::{null, null_mut};

use windows_sys::Win32::Foundation::{COLORREF, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows_sys::Win32::Graphics::Gdi::{
    CLEARTYPE_QUALITY, CLIP_DEFAULT_PRECIS, COLOR_WINDOW, CreateFontW, CreateSolidBrush,
    DEFAULT_CHARSET, DEFAULT_GUI_FONT, DeleteObject, FF_SWISS, FW_SEMIBOLD, GetStockObject, HBRUSH,
    HDC, HFONT, NULL_BRUSH, OUT_DEFAULT_PRECIS, SetBkColor, SetBkMode, SetTextColor, TRANSPARENT,
    UpdateWindow, VARIABLE_PITCH,
};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use windows_sys::Win32::UI::Controls::EM_SETCUEBANNER;
use windows_sys::Win32::UI::WindowsAndMessaging::{
    BM_SETCHECK, BN_CLICKED, BS_AUTOCHECKBOX, BS_DEFPUSHBUTTON, BS_PUSHBUTTON, CREATESTRUCTW,
    CW_USEDEFAULT, CreateWindowExW, DefWindowProcW, DestroyWindow, DispatchMessageW,
    ES_AUTOHSCROLL, ES_READONLY, GWLP_USERDATA, GetClientRect, GetMessageW, HMENU, IDC_ARROW,
    LoadCursorW, MSG, MoveWindow, PostQuitMessage, RegisterClassW, SW_SHOW, SendMessageW,
    SetWindowLongPtrW, ShowWindow, TranslateMessage, WM_COMMAND, WM_CTLCOLORBTN, WM_CTLCOLOREDIT,
    WM_CTLCOLORSTATIC, WM_DESTROY, WM_NCCREATE, WM_NCDESTROY, WM_SETFONT, WM_SIZE, WNDCLASSW,
    WS_BORDER, WS_CHILD, WS_DISABLED, WS_OVERLAPPEDWINDOW, WS_TABSTOP, WS_VISIBLE,
};

use crate::{ActionHandler, Element, Error, Result, Window};

const CLASS_NAME: &str = "RustUiWindow";
const ROOT_MARGIN: i32 = 24;
const COLUMN_GAP: i32 = 10;
const TEXT_HEIGHT: i32 = 24;
const HEADING_HEIGHT: i32 = 38;
const INPUT_HEIGHT: i32 = 32;
const BUTTON_HEIGHT: i32 = 34;
const CHECKBOX_HEIGHT: i32 = 28;
const DIVIDER_HEIGHT: i32 = 1;
const FIRST_CONTROL_ID: i32 = 1000;
const ERROR_CLASS_ALREADY_EXISTS: u32 = 1410;

struct WindowsState {
    root: Element,
    controls: Vec<RenderedControl>,
    control_styles: HashMap<HWND, ControlVisualStyle>,
    actions: HashMap<i32, String>,
    next_control_id: i32,
    action_handler: Option<ActionHandler>,
    heading_font: HFONT,
    fonts: Vec<HFONT>,
}

struct RenderedControl {
    hwnd: HWND,
    height: i32,
}

#[derive(Clone, Copy)]
struct ControlVisualStyle {
    color: Option<COLORREF>,
    background: Option<COLORREF>,
    brush: HBRUSH,
}

impl WindowsState {
    fn new(root: Element, action_handler: Option<ActionHandler>) -> Self {
        Self {
            root,
            controls: Vec::new(),
            control_styles: HashMap::new(),
            actions: HashMap::new(),
            next_control_id: FIRST_CONTROL_ID,
            action_handler,
            heading_font: create_heading_font(),
            fonts: Vec::new(),
        }
    }

    fn register_action(&mut self, action: impl Into<String>) -> i32 {
        let control_id = self.next_control_id;
        self.next_control_id += 1;
        self.actions.insert(control_id, action.into());
        control_id
    }

    fn register_control(&mut self, hwnd: HWND, height: i32, element: &Element) {
        self.controls.push(RenderedControl { hwnd, height });

        if let Some(style) = control_visual_style(element) {
            self.control_styles.insert(hwnd, style);
        }
    }

    fn register_font(&mut self, font: HFONT) {
        if !font.is_null() {
            self.fonts.push(font);
        }
    }
}

impl Drop for WindowsState {
    fn drop(&mut self) {
        for style in self.control_styles.values() {
            if !style.brush.is_null() {
                unsafe {
                    DeleteObject(style.brush as _);
                }
            }
        }

        for font in self.fonts.drain(..) {
            unsafe {
                DeleteObject(font as _);
            }
        }

        if !self.heading_font.is_null() {
            unsafe {
                DeleteObject(self.heading_font as _);
            }
        }
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
        "column" => {
            let (x, width) = layout_bounds(element, x, width);
            render_column(parent, instance, element, x, y, width, state)
        }
        "form" => {
            let (x, width) = layout_bounds(element, x, width);
            render_column(parent, instance, element, x, y, width, state)
        }
        "text" | "label" => {
            let (x, width) = layout_bounds(element, x, width);
            let height = control_height(element, TEXT_HEIGHT);
            let hwnd = create_control(
                "STATIC",
                element.text_content().unwrap_or_default(),
                parent,
                instance,
                x,
                y,
                width,
                height,
                WS_CHILD | WS_VISIBLE,
                null_mut(),
            )?;
            apply_text_style(hwnd, element, state);
            state.register_control(hwnd, height, element);

            Ok(height)
        }
        "heading" => {
            let (x, width) = layout_bounds(element, x, width);
            let height = control_height(element, HEADING_HEIGHT);
            let hwnd = create_control(
                "STATIC",
                element.text_content().unwrap_or_default(),
                parent,
                instance,
                x,
                y,
                width,
                height,
                WS_CHILD | WS_VISIBLE,
                null_mut(),
            )?;
            if !apply_text_style(hwnd, element, state) {
                set_control_font(hwnd, state.heading_font);
            }
            state.register_control(hwnd, height, element);

            Ok(height)
        }
        "spacer" => Ok(spacer_height(element)),
        "divider" => {
            let (x, width) = layout_bounds(element, x, width);
            let height = control_height(element, DIVIDER_HEIGHT);
            let hwnd = create_control(
                "STATIC",
                "",
                parent,
                instance,
                x,
                y,
                width,
                height,
                WS_CHILD | WS_VISIBLE | WS_BORDER,
                null_mut(),
            )?;
            state.register_control(hwnd, height, element);

            Ok(height)
        }
        "input" => {
            let (x, width) = layout_bounds(element, x, width);
            let height = control_height(element, INPUT_HEIGHT);
            let input_style = if true_attribute(element, "readonly") {
                ES_READONLY as u32
            } else {
                0
            };
            let hwnd = create_control(
                "EDIT",
                element.text_content().unwrap_or_default(),
                parent,
                instance,
                x,
                y,
                width,
                height,
                WS_CHILD
                    | WS_VISIBLE
                    | WS_TABSTOP
                    | WS_BORDER
                    | ES_AUTOHSCROLL as u32
                    | input_style
                    | disabled_style(element),
                null_mut(),
            )?;
            apply_text_style(hwnd, element, state);

            if let Some(placeholder) = attribute_value(element, "placeholder") {
                set_input_placeholder(hwnd, placeholder);
            }

            state.register_control(hwnd, height, element);

            Ok(height)
        }
        "checkbox" => {
            let (x, width) = layout_bounds(element, x, width);
            let height = control_height(element, CHECKBOX_HEIGHT);
            let control_id = action_attribute(element)
                .map(|action| state.register_action(action))
                .unwrap_or_default();
            let hwnd = create_control(
                "BUTTON",
                element.text_content().unwrap_or_default(),
                parent,
                instance,
                x,
                y,
                width,
                height,
                WS_CHILD
                    | WS_VISIBLE
                    | WS_TABSTOP
                    | BS_AUTOCHECKBOX as u32
                    | disabled_style(element),
                control_id as isize as HMENU,
            )?;
            apply_text_style(hwnd, element, state);

            if true_attribute(element, "checked") {
                unsafe {
                    SendMessageW(hwnd, BM_SETCHECK, 1, 0);
                }
            }

            state.register_control(hwnd, height, element);

            Ok(height)
        }
        "button" => {
            let (x, width) = layout_bounds(element, x, width);
            let height = control_height(element, BUTTON_HEIGHT);
            let control_id = action_attribute(element)
                .map(|action| state.register_action(action))
                .unwrap_or_default();
            let button_style = if true_attribute(element, "default")
                || attribute_value(element, "variant") == Some("primary")
            {
                BS_DEFPUSHBUTTON as u32
            } else {
                BS_PUSHBUTTON as u32
            };

            let hwnd = create_control(
                "BUTTON",
                element.text_content().unwrap_or_default(),
                parent,
                instance,
                x,
                y,
                width,
                height,
                WS_CHILD | WS_VISIBLE | WS_TABSTOP | button_style | disabled_style(element),
                control_id as isize as HMENU,
            )?;
            apply_text_style(hwnd, element, state);
            state.register_control(hwnd, height, element);

            Ok(height)
        }
        _ => {
            let (x, width) = layout_bounds(element, x, width);
            render_column(parent, instance, element, x, y, width, state)
        }
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
    let gap = gap(element);

    for child in element.children() {
        let height = render_element(parent, instance, child, x, current_y, width, state)?;
        current_y += height + gap;
    }

    Ok((current_y - y - gap).max(0))
}

fn relayout_root(parent: HWND, root: &Element, controls: &[RenderedControl]) {
    let mut rect = RECT {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };

    if unsafe { GetClientRect(parent, &mut rect) } == 0 {
        return;
    }

    let width = (rect.right - rect.left - ROOT_MARGIN * 2).max(1);
    let mut controls = controls.iter();
    layout_element(root, ROOT_MARGIN, ROOT_MARGIN, width, &mut controls);
}

fn layout_element<'a>(
    element: &Element,
    x: i32,
    y: i32,
    width: i32,
    controls: &mut impl Iterator<Item = &'a RenderedControl>,
) -> i32 {
    match element.name() {
        "column" => {
            let (x, width) = layout_bounds(element, x, width);
            layout_column(element, x, y, width, controls)
        }
        "form" => {
            let (x, width) = layout_bounds(element, x, width);
            layout_column(element, x, y, width, controls)
        }
        "spacer" => spacer_height(element),
        "heading" | "text" | "label" | "divider" | "input" | "checkbox" | "button" => {
            let (x, width) = layout_bounds(element, x, width);
            match controls.next() {
                Some(control) => {
                    unsafe {
                        MoveWindow(control.hwnd, x, y, width, control.height, 1);
                    }

                    control.height
                }
                None => 0,
            }
        }
        _ => layout_column(element, x, y, width, controls),
    }
}

fn layout_column<'a>(
    element: &Element,
    x: i32,
    y: i32,
    width: i32,
    controls: &mut impl Iterator<Item = &'a RenderedControl>,
) -> i32 {
    let mut current_y = y;
    let gap = gap(element);

    for child in element.children() {
        let height = layout_element(child, x, current_y, width, controls);
        current_y += height + gap;
    }

    (current_y - y - gap).max(0)
}

fn layout_bounds(element: &Element, x: i32, width: i32) -> (i32, i32) {
    let available_width = width;
    let mut width = attribute_i32(element, "width").unwrap_or(available_width);

    if let Some(min_width) = attribute_i32(element, "min_width") {
        width = width.max(min_width);
    }

    if let Some(max_width) = attribute_i32(element, "max_width") {
        width = width.min(max_width);
    }

    width = width.max(1);

    let x = match attribute_value(element, "align") {
        Some("center") => x + (available_width - width).max(0) / 2,
        Some("end") => x + (available_width - width).max(0),
        _ => x,
    };

    (x, width)
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

fn set_control_font(hwnd: HWND, font: HFONT) {
    if font.is_null() {
        return;
    }

    unsafe {
        SendMessageW(hwnd, WM_SETFONT, font as WPARAM, 1);
    }
}

fn disabled_style(element: &Element) -> u32 {
    if true_attribute(element, "disabled") {
        WS_DISABLED
    } else {
        0
    }
}

fn create_heading_font() -> HFONT {
    create_font(22, FW_SEMIBOLD as i32)
}

fn apply_text_style(hwnd: HWND, element: &Element, state: &mut WindowsState) -> bool {
    if attribute_value(element, "font_size").is_none()
        && attribute_value(element, "font_weight").is_none()
    {
        return false;
    }

    let font_size = attribute_value(element, "font_size")
        .and_then(|value| value.parse().ok())
        .unwrap_or_else(|| default_font_size(element));

    let font = create_font(font_size, font_weight(element));
    set_control_font(hwnd, font);
    state.register_font(font);
    true
}

fn create_font(font_size: i32, font_weight: i32) -> HFONT {
    let face_name = to_wide_null("Segoe UI");

    unsafe {
        CreateFontW(
            -font_size,
            0,
            0,
            0,
            font_weight,
            0,
            0,
            0,
            DEFAULT_CHARSET as u32,
            OUT_DEFAULT_PRECIS as u32,
            CLIP_DEFAULT_PRECIS as u32,
            CLEARTYPE_QUALITY as u32,
            (VARIABLE_PITCH | FF_SWISS) as u32,
            face_name.as_ptr(),
        )
    }
}

fn set_input_placeholder(hwnd: HWND, placeholder: &str) {
    let placeholder = to_wide_null(placeholder);

    unsafe {
        SendMessageW(hwnd, EM_SETCUEBANNER, 0, placeholder.as_ptr() as LPARAM);
    }
}

fn apply_control_colors(hwnd: HWND, hdc: HDC, state: &WindowsState) -> Option<HBRUSH> {
    let style = state.control_styles.get(&hwnd)?;

    if let Some(color) = style.color {
        unsafe {
            SetTextColor(hdc, color);
        }
    }

    if let Some(background) = style.background {
        unsafe {
            SetBkColor(hdc, background);
        }

        return Some(style.brush);
    }

    unsafe {
        SetBkMode(hdc, TRANSPARENT as i32);
    }

    Some(unsafe { GetStockObject(NULL_BRUSH) as HBRUSH })
}

fn control_visual_style(element: &Element) -> Option<ControlVisualStyle> {
    let color = attribute_value(element, "color").and_then(colorref_from_hex);
    let background = attribute_value(element, "background").and_then(colorref_from_hex);

    if color.is_none() && background.is_none() {
        return None;
    }

    let brush = background
        .map(|color| unsafe { CreateSolidBrush(color) })
        .unwrap_or(null_mut());

    Some(ControlVisualStyle {
        color,
        background,
        brush,
    })
}

fn colorref_from_hex(value: &str) -> Option<COLORREF> {
    let value = value.strip_prefix('#')?;

    if value.len() != 6 {
        return None;
    }

    let red = u8::from_str_radix(&value[0..2], 16).ok()? as COLORREF;
    let green = u8::from_str_radix(&value[2..4], 16).ok()? as COLORREF;
    let blue = u8::from_str_radix(&value[4..6], 16).ok()? as COLORREF;

    Some(red | (green << 8) | (blue << 16))
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
        WM_SIZE => {
            if let Some(state) = unsafe { state_from_window(hwnd) } {
                relayout_root(hwnd, &state.root, &state.controls);
            }

            0
        }
        WM_CTLCOLORSTATIC | WM_CTLCOLOREDIT | WM_CTLCOLORBTN => {
            if let Some(state) = unsafe { state_from_window(hwnd) } {
                if let Some(brush) = apply_control_colors(lparam as HWND, wparam as HDC, state) {
                    return brush as LRESULT;
                }
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
    attribute_value(element, "on_click").or_else(|| attribute_value(element, "on_toggle"))
}

fn true_attribute(element: &Element, name: &str) -> bool {
    attribute_value(element, name) == Some("true")
}

fn control_height(element: &Element, default_height: i32) -> i32 {
    attribute_i32(element, "height")
        .unwrap_or(default_height)
        .max(1)
}

fn gap(element: &Element) -> i32 {
    attribute_i32(element, "gap").unwrap_or(COLUMN_GAP).max(0)
}

fn font_weight(element: &Element) -> i32 {
    match attribute_value(element, "font_weight") {
        Some("bold") => 700,
        Some("semibold") => FW_SEMIBOLD as i32,
        _ => 400,
    }
}

fn default_font_size(element: &Element) -> i32 {
    match element.name() {
        "heading" => 22,
        _ => 14,
    }
}

fn spacer_height(element: &Element) -> i32 {
    attribute_i32(element, "height")
        .unwrap_or(COLUMN_GAP)
        .max(0)
}

fn attribute_i32(element: &Element, name: &str) -> Option<i32> {
    attribute_value(element, name).and_then(|value| value.parse().ok())
}

fn attribute_value<'a>(element: &'a Element, name: &str) -> Option<&'a str> {
    element
        .attributes()
        .iter()
        .find(|attribute| attribute.name() == name)
        .map(|attribute| attribute.value())
}

fn to_wide_null(value: impl AsRef<str>) -> Vec<u16> {
    value
        .as_ref()
        .encode_utf16()
        .chain(std::iter::once(0))
        .collect()
}
