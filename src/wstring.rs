use std::ffi::OsString;
use std::fmt::{self, Debug, Formatter};
use std::path::PathBuf;
#[cfg(windows)] use std::os::windows::prelude::*;

use imp::WStringCore;



/// A wide UTF16-ish `\0` terminated string
///
/// ### Differences vs [OsString]
///
/// * Remains wide in-memory to reduce conversions/encodings
/// * Null terminated
/// * UTF16-ish even on !windows
///
/// ### Differences vs [CString](std::ffi::CString)
///
/// * Remains wide in-memory to reduce conversions/encodings
/// * UTF16-ish
///
/// ### Differences vs [widestring](http://docs.rs/widestring)::*
///
/// * Technically 0 `unsafe`
/// * The one effectively unsafe bit (`unsafe_empty_or_0_terminated_buffer` access
///   potentailly invalidating `WString::as_ptr()`'s guarantees) is carefully limited.
#[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WString(WStringCore);

impl WString {
    /// Construct a [WString] from (`&`){`String`,`str`,`[u16]`,`Vec<u16>`}
    pub fn new(src: impl Into<WString>) -> Self { src.into() }

    /// Construct a [WString] from [code units](https://unicode.org/glossary/#code_unit) of a UTF16-ish string.  An implicit `\0` will be added.
    pub fn from_code_units(src: impl Iterator<Item = u16>) -> Self { Self::from_code_units_vec(src.collect::<Vec<u16>>()) }

    /// Construct a [WString] from [code units](https://unicode.org/glossary/#code_unit) of a UTF16-ish string.  An implicit `\0` will be added.
    pub fn from_code_units_vec(src: impl Into<Vec<u16>>) -> Self { Self(WStringCore::from_code_units_vec(src.into())) }

    /// Get the [code units](https://unicode.org/glossary/#code_unit) of a UTF16-ish string, without the `\0`
    pub fn units(&self) -> &[u16] { let buf = self.units0(); &buf[..buf.len()-1] }

    /// Get the [code units](https://unicode.org/glossary/#code_unit) of a UTF16-ish string, **including** the `\0`
    pub fn units0(&self) -> &[u16] { self.0.units0() }

    /// Length of the string in [code units](https://unicode.org/glossary/#code_unit), without the `\0`-terminator
    pub fn len(&self) -> usize { self.units().len() }

    /// Length of the string in [code units](https://unicode.org/glossary/#code_unit), **including** the `\0`-terminator
    pub fn len0(&self) -> usize { self.units0().len() }

    /// Convert the UTF16-ish string into an [OsString] ([WTF8](https://simonsapin.github.io/wtf-8/) on windows, something messier and possibly lossy on unix)
    pub fn to_os_string(&self) -> OsString { self.to_os_string_impl() }

    /// Get a pointer to a `\0`-terminated buffer for this string
    pub fn as_ptr0(&self) -> *const u16 { self.units0().as_ptr() }

    /// Make this string [Display](std::fmt::Display)-friendly
    pub fn display<'s>(&'s self) -> impl std::fmt::Display + 's { Display(self) }
}

struct Display<'s>(&'s WString);
impl std::fmt::Display for Display<'_> {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        std::fmt::Display::fmt(&PathBuf::from(self.0).display(), fmt)
    }
}

impl Debug for WString {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        Debug::fmt(&OsString::from(self), fmt)
    }
}

impl From<&WString> for OsString { fn from(value: &WString) -> OsString { value.to_os_string() } }
impl From< WString> for OsString { fn from(value:  WString) -> OsString { value.to_os_string() } }
impl From<&WString> for PathBuf  { fn from(value: &WString) -> PathBuf  { PathBuf::from(value.to_os_string()) } }
impl From< WString> for PathBuf  { fn from(value:  WString) -> PathBuf  { PathBuf::from(value.to_os_string()) } }

impl From<&String  > for WString { fn from(value: &String  ) -> Self { Self::from_code_units(value.encode_utf16()) } }
impl From< String  > for WString { fn from(value:  String  ) -> Self { Self::from_code_units(value.encode_utf16()) } }
impl From<&str     > for WString { fn from(value: &str     ) -> Self { Self::from_code_units(value.encode_utf16()) } }
impl From<&   [u16]> for WString { fn from(value: &   [u16]) -> Self { Self::from_code_units(value.iter().copied()) } }
impl From<&Vec<u16>> for WString { fn from(value: &Vec<u16>) -> Self { Self::from_code_units(value.iter().copied()) } }
impl From< Vec<u16>> for WString { fn from(value:  Vec<u16>) -> Self { Self::from_code_units_vec(value) } }

#[cfg(windows)] impl WString {
    fn to_os_string_impl(&self) -> OsString { OsString::from_wide(self.units()) }
}

#[cfg(not(windows))] impl WString {
    fn to_os_string_impl(&self) -> OsString { OsString::from(String::from_utf16_lossy(self.units0())) } // best effort
}

/// This module exists for easier auditing/code reviews, by limiting access to:
/// [imp::WStringCore::unsafe_empty_or_0_terminated_buffer_do_not_directly_access]
mod imp {
    /// Implementation of [WString](super::WString)
    #[derive(Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub(super) struct WStringCore {
        /// ### Safety
        ///
        /// Users of `WString::as_ptr()` rely on `0`-terminated buffers.  This makes
        /// the constraints on the buffer incredibly important.  This can be either:
        ///
        /// * Empty (in which case a static `&[0]` should be referenced instead)
        /// * `0`-terminated
        ///
        /// Since such an awkward contraint is easy to fuck up - both in relying on
        /// it, and in creating it - I've given this buffer a loud and obnoxious
        /// name to encorage writing as little code as possible against it.  Only
        /// two methods should need access:
        ///
        /// * `WString::from_code_units_vec` to construct it
        /// * `WString::units0` to read it sanely
        ///
        /// Use them, not this member!
        unsafe_empty_or_0_terminated_buffer_do_not_directly_access: Vec<u16>
    }

    impl WStringCore {
        /// Construct the data from a series UTF16-ish [code units](https://unicode.org/glossary/#code_unit).  A `\0` will be added.
        pub fn from_code_units_vec(mut src: Vec<u16>) -> Self {
            if !src.is_empty() { src.push(0); }
            // Safety:  We *just* 0-terminated `src` if it wasn't empty, so this should be correct
            Self { unsafe_empty_or_0_terminated_buffer_do_not_directly_access: src }
        }

        /// Get the [code units](https://unicode.org/glossary/#code_unit) of a UTF16-ish string, **including** the `\0`
        pub fn units0(&self) -> &[u16] {
            // Safety:  We check if it's empty.  If it's not, it *should* be 0-terminated, which we debug_assert.
            let buf = &self.unsafe_empty_or_0_terminated_buffer_do_not_directly_access;
            if !buf.is_empty() {
                debug_assert!(buf.ends_with(&[0]), "SOUNDNESS BUG:  WString buffer should be 0-terminated at all time if not empty!");
                &buf[..]
            } else {
                &[0]
            }
        }
    }
}
