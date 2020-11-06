use crate::WString;

use std::ops::Deref;
use std::fmt::{self, Display, Formatter};



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
    /// * `split.scale-100`
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

#[test] fn test_pfn() {
    let pfn = PackageFullName::from("NcsiUwpApp_1000.19041.423.0_neutral_neutral_8wekyb3d8bbwe");
    assert_eq!(pfn.name(),          wchar::wch!("NcsiUwpApp"));
    assert_eq!(pfn.version(),       wchar::wch!("1000.19041.423.0"));
    assert_eq!(pfn.architecture(),  wchar::wch!("neutral"));
    assert_eq!(pfn.field4(),        wchar::wch!("neutral"));
    assert_eq!(pfn.publisher_id(),  wchar::wch!("8wekyb3d8bbwe"));

    let pfn = PackageFullName::from("CanonicalGroupLimited.UbuntuonWindows_2004.2020.812.0_x64__79rhkp1fndgsc");
    assert_eq!(pfn.name(),          wchar::wch!("CanonicalGroupLimited.UbuntuonWindows"));
    assert_eq!(pfn.version(),       wchar::wch!("2004.2020.812.0"));
    assert_eq!(pfn.architecture(),  wchar::wch!("x64"));
    assert_eq!(pfn.field4(),        wchar::wch!(""));
    assert_eq!(pfn.publisher_id(),  wchar::wch!("79rhkp1fndgsc"));
}
