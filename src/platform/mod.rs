#[cfg(target_os = "windows")]
mod windows;

use crate::{Result, Window};

#[cfg(target_os = "windows")]
pub(crate) fn run_window(window: Window) -> Result<()> {
    windows::run_window(window)
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn run_window(_window: Window) -> Result<()> {
    Err(crate::Error::PlatformUnsupported)
}
