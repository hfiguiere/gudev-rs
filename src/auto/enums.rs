// This file was generated by gir (https://github.com/gtk-rs/gir @ 907cd81)
// from gir-files (https://github.com/gtk-rs/gir-files @ be1cc90)
// DO NOT EDIT

use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::StaticType;
use glib::Type;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[derive(Clone, Copy)]
#[non_exhaustive]
#[doc(alias = "GUdevDeviceType")]
pub enum DeviceType {
    #[doc(alias = "G_UDEV_DEVICE_TYPE_NONE")]
    None,
    #[doc(alias = "G_UDEV_DEVICE_TYPE_BLOCK")]
    Block,
    #[doc(alias = "G_UDEV_DEVICE_TYPE_CHAR")]
    Char,
#[doc(hidden)]
    __Unknown(i32),
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceType::{}", match *self {
            DeviceType::None => "None",
            DeviceType::Block => "Block",
            DeviceType::Char => "Char",
            _ => "Unknown",
        })
    }
}

#[doc(hidden)]
impl ToGlib for DeviceType {
    type GlibType = ffi::GUdevDeviceType;

    fn to_glib(&self) -> ffi::GUdevDeviceType {
        match *self {
            DeviceType::None => ffi::G_UDEV_DEVICE_TYPE_NONE,
            DeviceType::Block => ffi::G_UDEV_DEVICE_TYPE_BLOCK,
            DeviceType::Char => ffi::G_UDEV_DEVICE_TYPE_CHAR,
            DeviceType::__Unknown(value) => value,
}
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GUdevDeviceType> for DeviceType {
    unsafe fn from_glib(value: ffi::GUdevDeviceType) -> Self {
        match value {
            0 => DeviceType::None,
            98 => DeviceType::Block,
            99 => DeviceType::Char,
            value => DeviceType::__Unknown(value),
}
    }
}

impl StaticType for DeviceType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_udev_device_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for DeviceType {
    unsafe fn from_value_optional(value: &glib::Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for DeviceType {
    unsafe fn from_value(value: &glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for DeviceType {
    unsafe fn set_value(value: &mut glib::Value, this: &Self) {
        glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

