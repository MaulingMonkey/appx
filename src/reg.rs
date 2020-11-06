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

use std::convert::*;
use std::ffi::OsString;
use std::io;
use std::ops::Drop;
use std::path::PathBuf;
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

    pub(crate) fn get_value_dword(&self, sub_key: Option<&[u16]>, value: Option<&[u16]>) -> io::Result<u32> {
        let mut r = [0u32];
        self.get_value_w_impl(sub_key, value, win0!(RRF_RT_REG_DWORD), None, &mut r[..])?;
        let [r] = r;
        Ok(r)
    }

    pub(crate) fn get_value_qword(&self, sub_key: Option<&[u16]>, value: Option<&[u16]>) -> io::Result<u64> {
        let mut r = [0u64];
        self.get_value_w_impl(sub_key, value, win0!(RRF_RT_REG_QWORD), None, &mut r[..])?;
        let [r] = r;
        Ok(r)
    }

    pub(crate) fn get_value_string(&self, sub_key: Option<&[u16]>, value: Option<&[u16]>, buf: &mut [u16]) -> io::Result<String> {
        self.get_value_w_impl(sub_key, value, win0!(RRF_RT_REG_SZ), None, buf).map(Self::trim_u16).map(String::from_utf16_lossy)
    }

    pub(crate) fn get_value_os_string(&self, sub_key: Option<&[u16]>, value: Option<&[u16]>, buf: &mut [u16]) -> io::Result<OsString> {
        self.get_value_w_impl(sub_key, value, win0!(RRF_RT_REG_SZ), None, buf).map(Self::trim_u16).map(Self::os_string_from_wide)
    }

    pub(crate) fn get_value_pathbuf(&self, sub_key: Option<&[u16]>, value: Option<&[u16]>, buf: &mut [u16]) -> io::Result<PathBuf> {
        self.get_value_os_string(sub_key, value, buf).map(PathBuf::from)
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

impl Key {
    fn trim_u16(v: &[u16]) -> &[u16] { if v.last() != Some(&0) { v } else { &v[..v.len()-1] } }
}

#[cfg(not(windows))] impl Key {
    unsafe fn open_ex_w_impl(_hkey: HKEY, _sub_key: &[u16], _options: Options, _sam_desired: SAM) -> io::Result<Self> {
        Err(io::Error::new(io::ErrorKind::Other, "registry not implemented on this platform"))
    }

    fn enum_key_w_impl<'s>(&self, _index: u32, _name: &'s mut NameBuffer) -> io::Result<Option<&'s [u16]>> {
        Err(io::Error::new(io::ErrorKind::Other, "registry not implemented on this platform"))
    }

    fn get_value_w_impl<'v, T>(&self, _sub_key: Option<&[u16]>, _value: Option<&[u16]>, _flags: u32, _ty: Option<&mut u32>, _data: &'v mut [T]) -> io::Result<&'v [T]> {
        Err(io::Error::new(io::ErrorKind::Other, "registry not implemented on this platform"))
    }

    fn os_string_from_wide(buf: &[u16]) -> OsString {
        OsString::from(String::from_utf16_lossy(buf))
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

    fn get_value_w_impl<'v, T>(&self, sub_key: Option<&[u16]>, value: Option<&[u16]>, flags: u32, ty: Option<&mut u32>, data: &'v mut [T]) -> io::Result<&'v [T]> {
        let sub_key = sub_key.map_or(null_mut(), |sk| { assert!(sk.last() == Some(&0), "`sub_key` must be null terminated - use wchar::wch_c!(\"...\")!"); sk.as_ptr() as *mut _ });
        let value   = value  .map_or(null_mut(), |v | { assert!(v .last() == Some(&0),   "`value` must be null terminated - use wchar::wch_c!(\"...\")!"); v .as_ptr() as *mut _ });
        let ty      = ty     .map_or(null_mut(), |ty| ty);

        let mut len = u32::try_from(std::mem::size_of_val(data)).map_err(|_| io::Error::new(io::ErrorKind::Other, "RegGetValueW cannot read that much data"))?;
        let status = unsafe { RegGetValueW(self.0, sub_key, value, flags, ty, data.as_mut_ptr().cast(), &mut len) };
        match status as u32 {
            ERROR_SUCCESS   => Ok(&data[..((len as usize)/std::mem::size_of::<T>())]),
            // XXX: Grow `data` on ERROR_MORE_DATA?
            _               => Err(io::Error::from_raw_os_error(status))
        }
    }

    fn os_string_from_wide(buf: &[u16]) -> OsString {
        use std::os::windows::ffi::*;
        OsString::from_wide(buf)
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
