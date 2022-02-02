pub type DeviceNumber = ffi::GUdevDeviceNumber;

#[allow(unused_imports)]
mod auto;

pub use auto::*;

pub mod prelude {
    pub use super::auto::traits::*;
}
