#![allow(dead_code)] // XXX

pub(crate) struct NameBuffer(pub(crate) [u16; 255 + 1]); // https://docs.microsoft.com/en-us/windows/win32/sysinfo/registry-element-size-limits
impl NameBuffer {
    pub fn len(&self) -> u32 { 255 + 1 }
}
impl Default for NameBuffer {
    fn default() -> Self {
        Self([ // 16 x 16 = 256 = 255 + 1
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ])
    }
}

#[cfg(windows)] use winapi::shared::minwindef::HKEY;
#[cfg(windows)] use winapi::shared::winerror::*;
#[cfg(windows)] use winapi::um::winreg::*;
#[cfg(windows)] use winapi::um::winnt::*;

#[cfg(not(windows))] type HKEY      = *mut std::ffi::c_void;
#[cfg(not(windows))] type REGSAM    = u32;

use std::io;
use std::ops::Drop;
use std::ptr::null_mut;



/// [RegOpenKeyExW ulOptions values](https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeyexw)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Options(u32);
impl Options {
    pub const NONE      : Options = Options(0);
    pub const OPEN_LINK : Options = Options(win0!(REG_OPTION_OPEN_LINK));
}



/// [Registry Key Security and Access Rights](https://docs.microsoft.com/en-us/windows/win32/sysinfo/registry-key-security-and-access-rights)
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SAM(u32);
impl SAM {
    pub const ALL_ACCESS : SAM = SAM(win0!(KEY_ALL_ACCESS));
    pub const READ_ONLY  : SAM = SAM(win0!(KEY_QUERY_VALUE | KEY_ENUMERATE_SUB_KEYS));
    // TODO: saner read-only options
}



pub struct Key(HKEY);

impl Key {
    /// Takeover ownership of a give hkey
    ///
    /// ### SAFETY
    ///
    /// * `hkey` must be a valid [HKEY] for the duration of [Key]'s lifetime (e.g. it must be defined behavior to pass it to [Reg*])
    /// * `hkey` will be [RegCloseKey]ed when [Key] is [Drop]ped, so don't free it yourself (to avoid double-free bugs!)
    ///
    /// [RegCloseKey]:      https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regclosekey
    /// [Reg*]:             https://docs.microsoft.com/en-us/windows/win32/api/winreg/
    pub(crate) unsafe fn own(hkey: HKEY) -> Self { Self(hkey) }

    /// [RegOpenKeyExW]
    ///
    /// ### SAFETY
    ///
    /// * `hkey` must be a valid [HKEY] (e.g. it must be defined behavior to pass it to [Reg*])
    /// * `sub_key` must be `'\0'`-terminated or this function will panic
    ///
    /// [RegOpenKeyExW]:    https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeyexw
    /// [Reg*]:             https://docs.microsoft.com/en-us/windows/win32/api/winreg/
    pub(crate) unsafe fn open(hkey: HKEY, sub_key: &[u16], options: Options, sam_desired: SAM) -> io::Result<Self> {
        Self::open_ex_w_impl(hkey, sub_key, options, sam_desired)
    }

    /// [RegOpenKeyExW]
    ///
    /// * `sub_key` must be `'\0'`-terminated or this function will panic
    ///
    /// [RegOpenKeyExW]:    https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regopenkeyexw
    pub(crate) fn subkey(&self, sub_key: &[u16], options: Options, sam_desired: SAM) -> io::Result<Self> {
        unsafe { Self::open_ex_w_impl(self.0, sub_key, options, sam_desired) }
    }

    /// [RegEnumKeyExW] but simplified ala [RegEnumKeyW]
    ///
    /// [RegEnumKeyExW]:    https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumkeyexw
    /// [RegEnumKeyW]:      https://docs.microsoft.com/en-us/windows/win32/api/winreg/nf-winreg-regenumkeyw
    pub(crate) fn enum_key_w<'s>(&self, index: u32, name: &'s mut NameBuffer) -> io::Result<Option<&'s [u16]>> {
        self.enum_key_w_impl(index, name)
    }

    /// HKEY_CLASSES_ROOT
    pub(crate) fn hkcr(sub_key: &[u16], options: Options, sam_desired: SAM) -> io::Result<Self> { unsafe { Self::open(win0!(HKEY_CLASSES_ROOT),    sub_key, options, sam_desired) } }

    /// HKEY_CURRENT_CONFIG
    pub(crate) fn hkcc(sub_key: &[u16], options: Options, sam_desired: SAM) -> io::Result<Self> { unsafe { Self::open(win0!(HKEY_CURRENT_CONFIG),  sub_key, options, sam_desired) } }

    /// HKEY_CURRENT_USER
    pub(crate) fn hkcu(sub_key: &[u16], options: Options, sam_desired: SAM) -> io::Result<Self> { unsafe { Self::open(win0!(HKEY_CURRENT_USER),    sub_key, options, sam_desired) } }

    /// HKEY_LOCAL_MACHINE
    pub(crate) fn hklm(sub_key: &[u16], options: Options, sam_desired: SAM) -> io::Result<Self> { unsafe { Self::open(win0!(HKEY_LOCAL_MACHINE),   sub_key, options, sam_desired) } }

    /// HKEY_USERS
    pub(crate) fn hku (sub_key: &[u16], options: Options, sam_desired: SAM) -> io::Result<Self> { unsafe { Self::open(win0!(HKEY_USERS),           sub_key, options, sam_desired) } }
}

#[cfg(not(windows))] impl Key {
    unsafe fn open_ex_w_impl(_hkey: HKEY, _sub_key: &[u16], _options: Options, _sam_desired: SAM) -> io::Result<Self> {
        Err(io::Error::new(io::ErrorKind::Other, "registry not implemented on this platform"))
    }

    fn enum_key_w_impl<'s>(&self, _index: u32, _name: &'s mut NameBuffer) -> io::Result<Option<&'s [u16]>> {
        Err(io::Error::new(io::ErrorKind::Other, "registry not implemented on this platform"))
    }
}

#[cfg(windows)] impl Key {
    unsafe fn open_ex_w_impl(hkey: HKEY, sub_key: &[u16], options: Options, sam_desired: SAM) -> io::Result<Self> {
        assert!(sub_key.last() == Some(&0), "`sub_key` must be null terminated - use wchar::wch_c!(\"...\")!");
        let mut result = null_mut();
        let status = RegOpenKeyExW(hkey, sub_key.as_ptr(), options.0, sam_desired.0, &mut result);
        match status as u32 {
            ERROR_SUCCESS   => Ok(Self::own(result)),
            _               => Err(io::Error::from_raw_os_error(status)),
        }
    }

    fn enum_key_w_impl<'s>(&self, index: u32, name: &'s mut NameBuffer) -> io::Result<Option<&'s [u16]>> {
        let mut len = name.len();
        let status = unsafe { RegEnumKeyExW(self.0, index, name.0.as_mut_ptr(), &mut len, null_mut(), null_mut(), null_mut(), null_mut()) };
        match status as u32 {
            ERROR_SUCCESS       => Ok(Some(&name.0[..(len as usize)])),
            ERROR_NO_MORE_ITEMS => Ok(None),
            _                   => Err(io::Error::from_raw_os_error(status)),
        }
    }
}

impl Drop for Key {
    fn drop(&mut self) {
        #[cfg(windows)] {
            let status = unsafe { RegCloseKey(self.0) } as u32;
            assert_eq!(status, ERROR_SUCCESS);
        }
    }
}
