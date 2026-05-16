use core::ffi::c_char;
use core::fmt;

use crate::ffi;
use crate::util::take_owned_string;

/// Errors returned by the CoreImage bridge.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CIError {
    /// Caller supplied an invalid argument (for example a path with an interior NUL byte).
    InvalidArgument(String),
    /// A CoreImage API returned `nil` where a concrete object was required.
    NullResult(String),
    /// The requested operation is unavailable on this macOS or hardware configuration.
    Unsupported(String),
    /// Image encoding or filesystem I/O failed.
    Io(String),
    /// CoreImage or Foundation returned a framework error.
    Framework(String),
    /// Catch-all for unmapped bridge statuses.
    Unknown { code: i32, message: String },
}

impl fmt::Display for CIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(message) => write!(f, "invalid argument: {message}"),
            Self::NullResult(message) => write!(f, "unexpected nil result: {message}"),
            Self::Unsupported(message) => write!(f, "unsupported operation: {message}"),
            Self::Io(message) => write!(f, "image I/O failed: {message}"),
            Self::Framework(message) => write!(f, "CoreImage framework error: {message}"),
            Self::Unknown { code, message } => write!(f, "CoreImage error {code}: {message}"),
        }
    }
}

impl std::error::Error for CIError {}

impl CIError {
    pub(crate) unsafe fn from_swift(status: i32, error_str: *mut c_char) -> Self {
        let message = take_owned_string(error_str).unwrap_or_default();
        match status {
            ffi::status::INVALID_ARGUMENT => Self::InvalidArgument(message),
            ffi::status::NULL_RESULT => Self::NullResult(message),
            ffi::status::UNSUPPORTED => Self::Unsupported(message),
            ffi::status::IO => Self::Io(message),
            ffi::status::FRAMEWORK => Self::Framework(message),
            code => Self::Unknown { code, message },
        }
    }
}
