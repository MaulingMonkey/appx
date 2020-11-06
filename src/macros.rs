#[cfg(    windows )] macro_rules! win0 { ( $windows:expr ) => { $windows }; }
#[cfg(not(windows))] macro_rules! win0 { ( $windows:expr ) => { 0 as _ }; }

#[cfg(    windows )] macro_rules! winstr0 { ( $windows:literal ) => { wchar::wch_c!($windows) }; }
#[cfg(not(windows))] macro_rules! winstr0 { ( $windows:literal ) => { &[0u16] }; }
