pub extern crate libc;

#[macro_use]
mod macros;

pub mod coreopts;

#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "utf8")]
pub mod utf8;
#[cfg(feature = "encoding")]
pub mod encoding;
#[cfg(feature = "parse_time")]
pub mod parse_time;

#[cfg(all(unix, feature = "utmpx"))]
pub mod utmpx;
#[cfg(all(unix, feature = "c_types"))]
pub mod c_types;
#[cfg(all(unix, feature = "process"))]
pub mod process;
#[cfg(all(unix, feature = "signals"))]
pub mod signals;

#[cfg(all(windows, feature = "wide"))]
pub mod wide;
