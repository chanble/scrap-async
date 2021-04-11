#[macro_use]
extern crate cfg_if;
extern crate libc;

#[cfg(quartz)] extern crate block;
#[cfg(quartz)] mod quartz;

#[cfg(x11)] mod x11;

#[cfg(dxgi)] extern crate winapi;
#[cfg(dxgi)] mod dxgi;

mod common;
pub use common::*;
