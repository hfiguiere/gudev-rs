extern crate glib;
extern crate glib_sys;
extern crate gobject_sys;
extern crate gudev_sys as ffi;
extern crate libc;

pub use gudev_sys::GUdevDeviceNumber as DeviceNumber;

pub use auto::*;
mod auto;
