//! Miscellaneous utilities.

use std::{
    ffi::{CStr, OsStr},
    iter,
    os::{raw::c_char, windows::prelude::OsStrExt},
    slice, str,
};
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::{FARPROC, HINSTANCE},
        System::LibraryLoader::GetProcAddress,
    },
};

/// Helper to convert a string pointer to a [`prim@str`].
#[inline]
pub unsafe fn str_from_cstr<'a>(ptr: *const c_char) -> Option<&'a str> {
    if ptr.is_null() {
        None
    } else {
        CStr::from_ptr(ptr).to_str().ok()
    }
}

/// Helper to convert a string pointer and a length to a [`prim@str`].
///
/// The pointer needs to be non-null. Panics if the string is invalid UTF-8.
#[inline]
#[allow(unused)]
pub unsafe fn str_from_cstr_len<'a>(ptr: *const c_char, len: u64) -> &'a str {
    let slice = slice::from_raw_parts(ptr as *const u8, len as usize);
    str::from_utf8(slice).expect("cstr with invalid utf8")
}

/// Strips the `':'` prefix from an account name if present.
#[inline]
pub fn strip_account_prefix(account_name: &str) -> &str {
    account_name.strip_prefix(':').unwrap_or(account_name)
}

/// Helper to retrieve an exported function.
/// Name needs to be null-terminated.
#[inline]
pub unsafe fn exported_proc(handle: HINSTANCE, name: &'static str) -> FARPROC {
    GetProcAddress(handle, PCSTR(name.as_ptr()))
}

/// Helper to convert a string to a Windows wide char string.
#[inline]
pub unsafe fn str_to_wide(string: impl AsRef<str>) -> Vec<u16> {
    OsStr::new(string.as_ref())
        .encode_wide()
        .chain(iter::once(0))
        .collect()
}
