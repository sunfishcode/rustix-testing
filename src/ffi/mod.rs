//! Utilities related to FFI bindings.

#[cfg(not(feature = "std"))]
pub use alloc::ffi::{FromBytesWithNulError, NulError, ZStr, ZString};

#[cfg(feature = "std")]
pub use std::ffi::{CStr as ZStr, CString as ZString, FromBytesWithNulError, NulError};

/// Workaround for `CStr`/`ZStr` differences.
///
/// `CStr` has a method `to_string_lossy()` which returns `CString`.
/// `ZStr` lives in core, and `ZString` lives in alloc, so `ZStr`
/// can't define a corresponding method. So we define it as an
/// extension trait here.
#[cfg(not(feature = "std"))]
pub trait ZStrExt {
    /// Like `CStr::to_string_lossy`.
    fn to_string_lossy(&self) -> alloc::borrow::Cow<'_, str>;
}

/// Workaround for `CStr`/`ZStr` differences.
///
/// `CStr` has a method `to_string_lossy()` which returns `CString`.
/// `ZStr` lives in core, and `ZString` lives in alloc, so `ZStr`
/// can't define a corresponding method. So we define it as an
/// extension trait here.
#[cfg(feature = "std")]
pub trait ZStrExt {}

#[cfg(not(feature = "std"))]
impl ZStrExt for ZStr {
    #[inline]
    fn to_string_lossy(&self) -> alloc::borrow::Cow<'_, str> {
        ZString::to_string_lossy(self)
    }
}
