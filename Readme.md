# appx - manage [appx packages]

[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/appx.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/appx)
[![crates.io](https://img.shields.io/crates/v/appx.svg)](https://crates.io/crates/appx)
[![docs.rs](https://docs.rs/appx/badge.svg)](https://docs.rs/appx)
[![License](https://img.shields.io/crates/l/appx.svg)](https://github.com/MaulingMonkey/appx)
[![Build Status](https://github.com/MaulingMonkey/appx/workflows/Rust/badge.svg)](https://github.com/MaulingMonkey/appx/actions?query=workflow%3Arust)
<!-- [![dependency status](https://deps.rs/repo/github/MaulingMonkey/appx/status.svg)](https://deps.rs/repo/github/MaulingMonkey/appx) -->

Find, enumerate, and install [appx packages]

Intended to be an unholy conglomeration of:
* direct registry access
* `powershell ...` command line
* [winrt]?



<h2 name="todo">TODO</h2>

* [winrt] support for app sandbox compatability / progress monitoring?
* [makeappx] support?
    * Commands: `pack`, `unpack`, `bundle`, `unbundle`, `encrypt`, `decrypt`, ...
    * Binary location?
    * Mapping file generation?
    * [signtool] support?

<h2 name="license">License</h2>

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.



<h2 name="contribution">Contribution</h2>

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.



<!-- references -->

[appx packages]:        https://docs.microsoft.com/en-us/windows/msix/package/packaging-uwp-apps
[winrt]:                https://docs.rs/winrt/
[makeappx]:             https://docs.microsoft.com/en-us/windows/win32/appxpkg/make-appx-package--makeappx-exe-
[signtool]:             https://docs.microsoft.com/en-us/windows/win32/seccrypto/signtool
