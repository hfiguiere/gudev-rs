
extern crate libc;
#[macro_use]
extern crate glib;
extern crate gobject_sys;
extern crate glib_sys;
extern crate gudev_sys;

pub use gudev_sys::GUdevDeviceNumber as DeviceNumber;

pub use auto::*;
mod auto;
