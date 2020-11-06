use crate::WString;

use std::ops::Deref;
use std::fmt::{self, Display, Formatter};



/// e.g. `NcsiUwpApp_8wekyb3d8bbwe` -
/// Refers to a family or set of appx packages/files.
///
/// This is comprised of the following underscore separated fields:
///
/// | Field             | Example               | Example |
/// | ----------------- | --------------------- | ------- |
/// | `Name`            | `NcsiUwpApp`          | `CanonicalGroupLimited.Ubuntu20.04onWindows`
/// | `PublisherId`     | `8wekyb3d8bbwe`       | `79rhkp1fndgsc`
///
/// To refer to specific individual packages within the set, see [PackageFullName](crate::PackageFullName)
///
/// ### Examples
///
/// * `NcsiUwpApp_8wekyb3d8bbwe`
/// * `CanonicalGroupLimited.UbuntuonWindows_79rhkp1fndgsc`
/// * `CanonicalGroupLimited.Ubuntu16.04onWindows_79rhkp1fndgsc`
/// * `CanonicalGroupLimited.Ubuntu18.04onWindows_79rhkp1fndgsc`
/// * `CanonicalGroupLimited.Ubuntu20.04onWindows_79rhkp1fndgsc`
///
/// ### Corresponds to
///
/// `powershell Get-AppxPackage ^| Format-Table -Property PackageFamilyName`<br>
/// `HKCR\Local Settings\Software\Microsoft\Windows\CurrentVersion\AppModel\Repository\Families\...`<br>
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)] pub struct PackageFamilyName(pub(crate) WString);

impl Deref              for PackageFamilyName { fn deref(&self) -> &Self::Target { &self.0 } type Target = WString; }
impl Display            for PackageFamilyName { fn fmt(&self, fmt: &mut Formatter) -> fmt::Result { Display::fmt(&self.0.display(), fmt) } }
impl From<&String  >    for PackageFamilyName { fn from(value: &String  ) -> Self { Self(WString::from(value)) } }
impl From< String  >    for PackageFamilyName { fn from(value:  String  ) -> Self { Self(WString::from(value)) } }
impl From<&str     >    for PackageFamilyName { fn from(value: &str     ) -> Self { Self(WString::from(value)) } }
impl From<&   [u16]>    for PackageFamilyName { fn from(value: &   [u16]) -> Self { Self(WString::from(value)) } }
impl From<&Vec<u16>>    for PackageFamilyName { fn from(value: &Vec<u16>) -> Self { Self(WString::from(value)) } }
impl From< Vec<u16>>    for PackageFamilyName { fn from(value:  Vec<u16>) -> Self { Self(WString::from(value)) } }

impl PackageFamilyName {
    pub fn new(pfn: impl Into<Self>) -> Self { pfn.into() }

    /// Family `Name`
    ///
    /// ### Examples
    ///
    /// * `NcsiUwpApp`
    /// * `CanonicalGroupLimited.Ubuntu20.04onWindows`
    pub fn name(&self) -> &[u16] {
        match self.0.units().iter().position(|&cu| cu == u16::from(b'_')) {
            Some(u) => &self.0.units()[..u],
            None    => self.0.units(),
        }
    }

    /// Family `PublisherId`
    ///
    /// ### Examples
    ///
    /// * `8wekyb3d8bbwe`
    /// * `79rhkp1fndgsc`
    pub fn publisher_id(&self) -> &[u16] {
        match self.0.units().iter().position(|&cu| cu == u16::from(b'_')) {
            Some(u) => &self.0.units()[(u+1)..],
            None    => &[],
        }
    }
}

#[test] fn test_pfn() {
    let pfn = PackageFamilyName::from("NcsiUwpApp_8wekyb3d8bbwe");
    assert_eq!(pfn.name(),          wchar::wch!("NcsiUwpApp"));
    assert_eq!(pfn.publisher_id(),  wchar::wch!("8wekyb3d8bbwe"));

    let pfn = PackageFamilyName::from("CanonicalGroupLimited.UbuntuonWindows_79rhkp1fndgsc");
    assert_eq!(pfn.name(),          wchar::wch!("CanonicalGroupLimited.UbuntuonWindows"));
    assert_eq!(pfn.publisher_id(),  wchar::wch!("79rhkp1fndgsc"));
}
