use crate::reg;
use crate::WString;

use std::fmt::{self, Display, Formatter};
use std::io;
use std::ops::Deref;
use std::path::PathBuf;



/// e.g. `NcsiUwpApp_1000.19041.423.0_neutral_neutral_8wekyb3d8bbwe` -
/// A specific appx package.
///
/// This is comprised of the following underscore separated fields:
///
/// | Field             | Example               | Example |
/// | ----------------- | --------------------- | ------- |
/// | `Name`            | `NcsiUwpApp`          | `CanonicalGroupLimited.Ubuntu20.04onWindows`
/// | `Version`         | `1000.19041.423.0`    | `2004.2020.812.0`
/// | `Architecture`    | `neutral`             | `x64`
/// | ???               | `neutral`             | (blank), `split.scale-100`
/// | `PublisherId`     | `8wekyb3d8bbwe`       | `79rhkp1fndgsc`
///
/// ### Examples
///
/// * `NcsiUwpApp_1000.19041.423.0_neutral_neutral_8wekyb3d8bbwe`
/// * `CanonicalGroupLimited.UbuntuonWindows_2004.2020.812.0_x64__79rhkp1fndgsc`
/// * `CanonicalGroupLimited.Ubuntu16.04onWindows_1604.2020.824.0_x64__79rhkp1fndgsc`
/// * `CanonicalGroupLimited.Ubuntu18.04onWindows_1804.2020.824.0_x64__79rhkp1fndgsc`
/// * `CanonicalGroupLimited.Ubuntu20.04onWindows_2004.2020.812.0_x64__79rhkp1fndgsc`
/// * `CanonicalGroupLimited.Ubuntu20.04onWindows_2004.2020.812.0_neutral_split.scale-100_79rhkp1fndgsc`
///
/// ### Corresponds to
///
/// `powershell Get-AppxPackage ^| Format-Table -Property PackageFullName`<br>
/// `HKCR\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Packages\...`<br>
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct PackageFullName(pub(crate) WString);

impl Deref              for PackageFullName { fn deref(&self) -> &Self::Target { &self.0 } type Target = WString; }
impl Display            for PackageFullName { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.0.display(), fmt) } }
impl From<&String  >    for PackageFullName { fn from(value: &String  ) -> Self { Self(WString::from(value)) } }
impl From< String  >    for PackageFullName { fn from(value:  String  ) -> Self { Self(WString::from(value)) } }
impl From<&str     >    for PackageFullName { fn from(value: &str     ) -> Self { Self(WString::from(value)) } }
impl From<&   [u16]>    for PackageFullName { fn from(value: &   [u16]) -> Self { Self(WString::from(value)) } }
impl From<&Vec<u16>>    for PackageFullName { fn from(value: &Vec<u16>) -> Self { Self(WString::from(value)) } }
impl From< Vec<u16>>    for PackageFullName { fn from(value:  Vec<u16>) -> Self { Self(WString::from(value)) } }

impl PackageFullName {
    pub fn new(pfn: impl Into<Self>) -> Self { pfn.into() }

    /// Package `Name`
    ///
    /// ### Examples
    ///
    /// * `NcsiUwpApp`
    /// * `CanonicalGroupLimited.Ubuntu20.04onWindows`
    pub fn name(&self) -> &[u16] { self.field(0) }

    /// Package `Version`
    ///
    /// ### Examples
    ///
    /// * `1000.19041.423.0`
    /// * `2004.2020.812.0`
    pub fn version(&self) -> &[u16] { self.field(1) }

    /// Package `Architecture`
    ///
    /// ### Examples
    ///
    /// * `x64`
    /// * `neutral`
    pub fn architecture(&self) -> &[u16] { self.field(2) }

    /// ???
    ///
    /// ### Examples
    ///
    /// * (blank)
    /// * `neutral`         (`ResourceId`?)
    /// * `split.scale-100` (**not** `ResourceId`)
    #[allow(dead_code)] // TODO: name
    fn field4(&self) -> &[u16] { self.field(3) }

    /// Package `PublisherId`
    ///
    /// ### Examples
    ///
    /// * `8wekyb3d8bbwe`
    /// * `79rhkp1fndgsc`
    pub fn publisher_id(&self) -> &[u16] { self.field(4) }

    fn field(&self, n: usize) -> &[u16] { self.0.units().splitn(5, |&cu| cu == u16::from(b'_')).skip(n).next().unwrap_or(&[]) }
}

impl PackageFullName {
    fn key_packages() -> io::Result<reg::Key> { reg::Key::hkcr(winstr0!(r"Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Packages"), reg::Options::NONE, reg::SAM::READ_ONLY) }

    // `CapabilityCount` - parse w/ `CapabilitySids` instead?
    // pub fn capability_count(&self) -> io::Result<u32> { ... }
    
    // `CapabilitySids` - format?
    // pub fn capability_sids(&self) -> io::Result<Vec<u8>> { ... }
    
    /// `DisplayName`
    pub fn display_name(&self) -> io::Result<String> { key_packages()?.get_value_string(Some(self.units0()), Some(winstr0!("DisplayName")), &mut temp_vec(64 * 1024)) }

    /// `OSMaxVersionTested`
    pub fn os_max_version_tested(&self) -> io::Result<u64> { key_packages()?.get_value_qword(Some(self.units0()), Some(winstr0!("OSMaxVersionTested"))) }

    /// `OSMinVersion`
    pub fn os_min_version(&self) -> io::Result<u64> { key_packages()?.get_value_qword(Some(self.units0()), Some(winstr0!("OSMinVersion"))) }

    // `PackageId`
    // pub fn package_id(&self) -> io::Result<OsString> { ... } - is a reg key, but already implied from the PFN

    /// `PackageRootFolder` (registry)
    /// `InstallLocation` (powershell)
    pub fn package_root_folder(&self) -> io::Result<PathBuf> { Self::key_packages()?.get_value_pathbuf(Some(self.units0()), Some(winstr0!("PackageRootFolder")), &mut temp_vec(64 * 1024)) }

    /// `PackageRootFolder` (registry)
    /// `InstallLocation` (powershell)
    pub fn install_location(&self) -> io::Result<PathBuf> { self.package_root_folder() }

    // `PackageSid` - don't have REG_BINARY APIs (yet)
    //pub fn package_sid(&self) -> io::Result<Vec<u8>> {
    //}

    /// `SupportedUsers`
    pub fn supported_users(&self) -> io::Result<u32> { key_packages()?.get_value_dword(Some(self.units0()), Some(winstr0!("SupportedUsers"))) }
}

fn key_packages() -> io::Result<reg::Key> { reg::Key::hkcr(winstr0!(r"Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Packages"), reg::Options::NONE, reg::SAM::READ_ONLY) }

fn temp_vec<T: Copy + Default>(n: usize) -> Vec<T> {
    let mut v = Vec::new();
    v.resize(n, Default::default());
    v
}

#[test] fn test_pfn() {
    let pfn = PackageFullName::from("NcsiUwpApp_1000.19041.423.0_neutral_neutral_8wekyb3d8bbwe");
    assert_eq!(pfn.name(),          wchar::wch!("NcsiUwpApp"));
    assert_eq!(pfn.version(),       wchar::wch!("1000.19041.423.0"));
    assert_eq!(pfn.architecture(),  wchar::wch!("neutral"));
    assert_eq!(pfn.field4(),        wchar::wch!("neutral"));
    assert_eq!(pfn.publisher_id(),  wchar::wch!("8wekyb3d8bbwe"));

    if pfn.display_name().is_ok() { 
        assert_eq!(pfn.display_name().unwrap(),                 "NcsiUwpApp");
        assert_eq!(pfn.package_root_folder().unwrap(),          PathBuf::from(r"C:\WINDOWS\SystemApps\NcsiUwpApp_8wekyb3d8bbwe"));
        assert_eq!(pfn.install_location().unwrap(),             PathBuf::from(r"C:\WINDOWS\SystemApps\NcsiUwpApp_8wekyb3d8bbwe"));
        assert!(   pfn.supported_users().unwrap_or(0)           >= 1);

        if std::env::var("COMPUTERNAME").map_or(false, |cn| cn == "SACRILEGE") {
            assert!(pfn.os_max_version_tested().unwrap_or(0)    >= 0x03E8_4A61_01A7_0000);
            assert!(pfn.os_min_version().unwrap_or(0)           >= 0x000A_0000_0000_0000);
        }
    }

    let pfn = PackageFullName::from("CanonicalGroupLimited.UbuntuonWindows_2004.2020.812.0_x64__79rhkp1fndgsc");
    assert_eq!(pfn.name(),          wchar::wch!("CanonicalGroupLimited.UbuntuonWindows"));
    assert_eq!(pfn.version(),       wchar::wch!("2004.2020.812.0"));
    assert_eq!(pfn.architecture(),  wchar::wch!("x64"));
    assert_eq!(pfn.field4(),        wchar::wch!(""));
    assert_eq!(pfn.publisher_id(),  wchar::wch!("79rhkp1fndgsc"));
}
