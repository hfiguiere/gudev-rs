// Generated by gir (https://github.com/gtk-rs/gir @ 0e476ab5c1de)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ cfc0305f903b)
// DO NOT EDIT

use crate::{DeviceNumber, DeviceType};
use glib::{prelude::*, translate::*};

glib::wrapper! {
    #[doc(alias = "GUdevDevice")]
    pub struct Device(Object<ffi::GUdevDevice, ffi::GUdevDeviceClass>);

    match fn {
        type_ => || ffi::g_udev_device_get_type(),
    }
}

impl Device {
    pub const NONE: Option<&'static Device> = None;
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::Device>> Sealed for T {}
}

pub trait DeviceExt: IsA<Device> + sealed::Sealed + 'static {
    #[doc(alias = "g_udev_device_get_action")]
    #[doc(alias = "get_action")]
    fn action(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_udev_device_get_action(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_device_file")]
    #[doc(alias = "get_device_file")]
    fn device_file(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_udev_device_get_device_file(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_device_file_symlinks")]
    #[doc(alias = "get_device_file_symlinks")]
    fn device_file_symlinks(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_udev_device_get_device_file_symlinks(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_device_number")]
    #[doc(alias = "get_device_number")]
    fn device_number(&self) -> DeviceNumber {
        unsafe { ffi::g_udev_device_get_device_number(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_udev_device_get_device_type")]
    #[doc(alias = "get_device_type")]
    fn device_type(&self) -> DeviceType {
        unsafe {
            from_glib(ffi::g_udev_device_get_device_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_devtype")]
    #[doc(alias = "get_devtype")]
    fn devtype(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_udev_device_get_devtype(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_driver")]
    #[doc(alias = "get_driver")]
    fn driver(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_udev_device_get_driver(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_is_initialized")]
    #[doc(alias = "get_is_initialized")]
    fn is_initialized(&self) -> bool {
        unsafe {
            from_glib(ffi::g_udev_device_get_is_initialized(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_name")]
    #[doc(alias = "get_name")]
    fn name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::g_udev_device_get_name(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_udev_device_get_number")]
    #[doc(alias = "get_number")]
    fn number(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_udev_device_get_number(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_parent")]
    #[doc(alias = "get_parent")]
    #[must_use]
    fn parent(&self) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_device_get_parent(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_parent_with_subsystem")]
    #[doc(alias = "get_parent_with_subsystem")]
    #[must_use]
    fn parent_with_subsystem(&self, subsystem: &str, devtype: Option<&str>) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_device_get_parent_with_subsystem(
                self.as_ref().to_glib_none().0,
                subsystem.to_glib_none().0,
                devtype.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_property")]
    #[doc(alias = "get_property")]
    fn property(&self, key: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_udev_device_get_property(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_property_as_boolean")]
    #[doc(alias = "get_property_as_boolean")]
    fn property_as_boolean(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::g_udev_device_get_property_as_boolean(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_property_as_double")]
    #[doc(alias = "get_property_as_double")]
    fn property_as_double(&self, key: &str) -> f64 {
        unsafe {
            ffi::g_udev_device_get_property_as_double(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "g_udev_device_get_property_as_int")]
    #[doc(alias = "get_property_as_int")]
    fn property_as_int(&self, key: &str) -> i32 {
        unsafe {
            ffi::g_udev_device_get_property_as_int(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "g_udev_device_get_property_as_strv")]
    #[doc(alias = "get_property_as_strv")]
    fn property_as_strv(&self, key: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_udev_device_get_property_as_strv(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_property_as_uint64")]
    #[doc(alias = "get_property_as_uint64")]
    fn property_as_uint64(&self, key: &str) -> u64 {
        unsafe {
            ffi::g_udev_device_get_property_as_uint64(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "g_udev_device_get_property_keys")]
    #[doc(alias = "get_property_keys")]
    fn property_keys(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_udev_device_get_property_keys(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_seqnum")]
    #[doc(alias = "get_seqnum")]
    fn seqnum(&self) -> u64 {
        unsafe { ffi::g_udev_device_get_seqnum(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_udev_device_get_subsystem")]
    #[doc(alias = "get_subsystem")]
    fn subsystem(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_udev_device_get_subsystem(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr")]
    #[doc(alias = "get_sysfs_attr")]
    fn sysfs_attr(&self, name: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_udev_device_get_sysfs_attr(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr_as_boolean")]
    #[doc(alias = "get_sysfs_attr_as_boolean")]
    fn sysfs_attr_as_boolean(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_udev_device_get_sysfs_attr_as_boolean(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr_as_boolean_uncached")]
    #[doc(alias = "get_sysfs_attr_as_boolean_uncached")]
    fn sysfs_attr_as_boolean_uncached(&self, name: &str) -> bool {
        unsafe {
            from_glib(ffi::g_udev_device_get_sysfs_attr_as_boolean_uncached(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr_as_double")]
    #[doc(alias = "get_sysfs_attr_as_double")]
    fn sysfs_attr_as_double(&self, name: &str) -> f64 {
        unsafe {
            ffi::g_udev_device_get_sysfs_attr_as_double(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr_as_double_uncached")]
    #[doc(alias = "get_sysfs_attr_as_double_uncached")]
    fn sysfs_attr_as_double_uncached(&self, name: &str) -> f64 {
        unsafe {
            ffi::g_udev_device_get_sysfs_attr_as_double_uncached(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr_as_int")]
    #[doc(alias = "get_sysfs_attr_as_int")]
    fn sysfs_attr_as_int(&self, name: &str) -> i32 {
        unsafe {
            ffi::g_udev_device_get_sysfs_attr_as_int(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr_as_int_uncached")]
    #[doc(alias = "get_sysfs_attr_as_int_uncached")]
    fn sysfs_attr_as_int_uncached(&self, name: &str) -> i32 {
        unsafe {
            ffi::g_udev_device_get_sysfs_attr_as_int_uncached(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr_as_strv")]
    #[doc(alias = "get_sysfs_attr_as_strv")]
    fn sysfs_attr_as_strv(&self, name: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_udev_device_get_sysfs_attr_as_strv(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr_as_strv_uncached")]
    #[doc(alias = "get_sysfs_attr_as_strv_uncached")]
    fn sysfs_attr_as_strv_uncached(&self, name: &str) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(
                ffi::g_udev_device_get_sysfs_attr_as_strv_uncached(
                    self.as_ref().to_glib_none().0,
                    name.to_glib_none().0,
                ),
            )
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr_as_uint64")]
    #[doc(alias = "get_sysfs_attr_as_uint64")]
    fn sysfs_attr_as_uint64(&self, name: &str) -> u64 {
        unsafe {
            ffi::g_udev_device_get_sysfs_attr_as_uint64(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr_as_uint64_uncached")]
    #[doc(alias = "get_sysfs_attr_as_uint64_uncached")]
    fn sysfs_attr_as_uint64_uncached(&self, name: &str) -> u64 {
        unsafe {
            ffi::g_udev_device_get_sysfs_attr_as_uint64_uncached(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            )
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr_keys")]
    #[doc(alias = "get_sysfs_attr_keys")]
    fn sysfs_attr_keys(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_udev_device_get_sysfs_attr_keys(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_attr_uncached")]
    #[doc(alias = "get_sysfs_attr_uncached")]
    fn sysfs_attr_uncached(&self, name: &str) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_udev_device_get_sysfs_attr_uncached(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_sysfs_path")]
    #[doc(alias = "get_sysfs_path")]
    fn sysfs_path(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::g_udev_device_get_sysfs_path(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_tags")]
    #[doc(alias = "get_tags")]
    fn tags(&self) -> Vec<glib::GString> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::g_udev_device_get_tags(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_get_usec_since_initialized")]
    #[doc(alias = "get_usec_since_initialized")]
    fn usec_since_initialized(&self) -> u64 {
        unsafe { ffi::g_udev_device_get_usec_since_initialized(self.as_ref().to_glib_none().0) }
    }

    #[doc(alias = "g_udev_device_has_property")]
    fn has_property(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::g_udev_device_has_property(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_has_sysfs_attr")]
    fn has_sysfs_attr(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::g_udev_device_has_sysfs_attr(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "g_udev_device_has_sysfs_attr_uncached")]
    fn has_sysfs_attr_uncached(&self, key: &str) -> bool {
        unsafe {
            from_glib(ffi::g_udev_device_has_sysfs_attr_uncached(
                self.as_ref().to_glib_none().0,
                key.to_glib_none().0,
            ))
        }
    }
}

impl<O: IsA<Device>> DeviceExt for O {}
