
extern crate libc;
#[macro_use]
extern crate glib;
extern crate glib_sys as glib_ffi;
extern crate gudev_sys as ffi;

/// No-op
macro_rules! callback_guard {
    () => ()
}

pub use ffi::GUdevDeviceNumber as DeviceNumber;

pub use auto::*;
mod auto;
