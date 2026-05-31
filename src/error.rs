use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    PlatformUnsupported,
    WindowsApi { operation: &'static str, code: u32 },
}

impl Error {
    #[cfg(target_os = "windows")]
    pub(crate) fn windows_api(operation: &'static str) -> Self {
        Self::WindowsApi {
            operation,
            code: unsafe { windows_sys::Win32::Foundation::GetLastError() },
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PlatformUnsupported => {
                write!(formatter, "this platform is not supported")
            }
            Self::WindowsApi { operation, code } => {
                write!(formatter, "{operation} failed with Windows error {code}")
            }
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
