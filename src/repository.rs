//! `HKCR\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository`<br>
//! [Windows::Management::Deployment::PackageManager](https://docs.microsoft.com/en-us/uwp/api/windows.management.deployment.packagemanager?view=winrt-19041)

// TODO: winrt alternatives to most of these APIs might be nice

use crate::{PackageFamilyName, PackageFullName, WString};
use crate::reg;

use std::io;
use std::path::Path;



/// Get the [PackageFamilyName]s installed on this computer
///
/// ### Examples
///
/// ```rust
/// for fam in appx::repository::families().unwrap() {
///     println!("{}", fam);
/// }
/// ```
pub fn families() -> io::Result<impl Iterator<Item = PackageFamilyName>> { imp::families() }

/// Get the [PackageFullName]s installed on this computer
///
/// ### Examples
///
/// ```rust
/// for pkg in appx::repository::packages().unwrap() {
///     println!("{}", pkg);
/// }
/// ```
pub fn packages() -> io::Result<impl Iterator<Item = PackageFullName>> { imp::packages() }

/// Check if the [PackageFamilyName] appears on this computer
pub fn has_family(fam: impl Into<PackageFamilyName>) -> bool {
    if !cfg!(windows) { return false; }
    let fam = fam.into();
    let key = reg::Key::hkcr(winstr0!(r"Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Families"), reg::Options::NONE, reg::SAM::READ_ONLY);
    key.ok().and_then(|key| key.subkey(fam.units0(), reg::Options::NONE, reg::SAM::READ_ONLY).ok()).is_some()
}

/// Check if the [PackageFullName] appears on this computer
pub fn has_package(pfn: impl Into<PackageFullName>) -> bool {
    if !cfg!(windows) { return false; }
    let pfn = pfn.into();
    let key = reg::Key::hkcr(winstr0!(r"Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Packages"), reg::Options::NONE, reg::SAM::READ_ONLY);
    key.ok().and_then(|key| key.subkey(pfn.units0(), reg::Options::NONE, reg::SAM::READ_ONLY).ok()).is_some()
}

/// `powershell Add-AppxPackage -Path [path]` or equivalent - install `path` as an appx package
///
/// Might use [winrt](https://docs.rs/winrt/) in the future, possibly behind a feature for WinRT app compatability
pub fn add_appx_package(path: impl AsRef<Path>) -> io::Result<()> {
    if !cfg!(windows) { return Err(io::Error::new(io::ErrorKind::Other, "add_appx_package: not implemented on this platform")); }

    let path = path.as_ref();
    if !path.exists() { return Err(io::Error::new(io::ErrorKind::NotFound, "add_appx_package: `path` does not exist")); }
    let mut cmd = std::process::Command::new("powershell");
    cmd.arg("Add-AppxPackage");
    cmd.arg("-Path").arg(path);
    let status = cmd.status().map_err(|err| io::Error::new(err.kind(), "add_appx_package: `powershell Add-AppxPackage -Path ...` failed to launch"))?;
    match status.code() {
        Some(0) => Ok(()),
        Some(n) => Err(io::Error::new(io::ErrorKind::Other, format!("add_appx_package: `powershell Add-AppxPackage -Path ...` failed (exit code {})", n))),
        None    => Err(io::Error::new(io::ErrorKind::Other,         "add_appx_package: `powershell Add-AppxPackage -Path ...` failed (signal)")),
    }
}



#[cfg(not(windows))] mod imp {
    use super::*;
    pub(super) fn families() -> io::Result<impl Iterator<Item = PackageFamilyName>> { Ok(None.into_iter()) }
    pub(super) fn packages() -> io::Result<impl Iterator<Item = PackageFullName  >> { Ok(None.into_iter()) }
}



#[cfg(windows)] mod imp {
    use super::*;

    pub(super) fn families() -> io::Result<impl Iterator<Item = PackageFamilyName>> {
        Ok(FamiliesIter {
            key: reg::Key::hkcr(winstr0!(r"Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Families"), reg::Options::NONE, reg::SAM::READ_ONLY)?,
            index: 0,
        })
    }

    struct FamiliesIter {
        key:    reg::Key,
        index:  u32,
    }

    impl Iterator for FamiliesIter {
        type Item = PackageFamilyName;
        fn next(&mut self) -> Option<Self::Item> {
            let mut pfn = Default::default();
            let pfn = self.key.enum_key_w(self.index, &mut pfn).ok()??;
            self.index += 1;
            Some(PackageFamilyName(WString::new(pfn)))
        }
    }

    pub(super) fn packages() -> io::Result<impl Iterator<Item = PackageFullName>> {
        Ok(PackagesIter {
            key: reg::Key::hkcr(winstr0!(r"Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Packages"), reg::Options::NONE, reg::SAM::READ_ONLY)?,
            index: 0,
        })
    }

    struct PackagesIter {
        key:    reg::Key,
        index:  u32,
    }

    impl Iterator for PackagesIter {
        type Item = PackageFullName;
        fn next(&mut self) -> Option<Self::Item> {
            let mut pfn = Default::default();
            let pfn = self.key.enum_key_w(self.index, &mut pfn).ok()??;
            self.index += 1;
            Some(PackageFullName(WString::new(pfn)))
        }
    }
}
