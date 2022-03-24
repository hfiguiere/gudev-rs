//! Rust bindings for the gudev library.
//!
//! [Libgudev](https://gitlab.gnome.org/GNOME/libgudev) is a library providing GObject bindings for libudev.
//!
//! # Usage
//!
//! Add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! gudev = "0.12"
//! ```
//!
//! Next:
//!
//! ```rust
//! use gudev::prelude::*;
//! use gudev::Client;
//! use gudev::Device;
//!
//! fn main() {
//!     // Get a list of all block devices
//!     let devices = Client::new(&[]).query_by_subsystem(Some("block"));
//!
//!     for device in devices {
//!         print_device(&device);
//!     }
//! }
//!
//! fn print_device(device: &Device) {
//!     println!(
//!         "{} (subsystem={}, sysname={}, devtype={})",
//!         device.sysfs_path().unwrap_or_else(|| "---".into()),
//!         device.subsystem().unwrap_or_else(|| "---".into()),
//!         device.device_file().unwrap_or_else(|| "---".into()),
//!         device.devtype().unwrap_or_else(|| "---".into()),
//!     );
//! }
//! ```
//! 
pub use ffi;

pub type DeviceNumber = ffi::GUdevDeviceNumber;

#[allow(unused_imports)]
mod auto;

pub use auto::*;

pub mod prelude {
    pub use super::auto::traits::*;
}
