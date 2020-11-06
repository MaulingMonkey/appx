#![deny(unreachable_patterns)]  // common bug when missing ERROR_*

#![cfg_attr(not(windows), allow(dead_code))]
#![cfg_attr(not(windows), allow(unused_imports))]
#![cfg_attr(not(windows), allow(unused_macros))]



#[macro_use] mod macros;

mod family;                 pub use family::PackageFamilyName;
mod package;                pub use package::PackageFullName;
mod reg;
pub mod repository;
mod wstring;                pub use wstring::WString;
