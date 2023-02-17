// Generated by gir (https://github.com/gtk-rs/gir @ 43b9242f6689)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 3ff4d3275258)
// DO NOT EDIT

use glib::{translate::*, value::FromValue, value::ToValue, StaticType, Type};
use std::fmt;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Clone, Copy)]
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
        write!(
            f,
            "DeviceType::{}",
            match *self {
                Self::None => "None",
                Self::Block => "Block",
                Self::Char => "Char",
                _ => "Unknown",
            }
        )
    }
}

#[doc(hidden)]
impl IntoGlib for DeviceType {
    type GlibType = ffi::GUdevDeviceType;

    #[inline]
    fn into_glib(self) -> ffi::GUdevDeviceType {
        match self {
            Self::None => ffi::G_UDEV_DEVICE_TYPE_NONE,
            Self::Block => ffi::G_UDEV_DEVICE_TYPE_BLOCK,
            Self::Char => ffi::G_UDEV_DEVICE_TYPE_CHAR,
            Self::__Unknown(value) => value,
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::GUdevDeviceType> for DeviceType {
    #[inline]
    unsafe fn from_glib(value: ffi::GUdevDeviceType) -> Self {
        match value {
            ffi::G_UDEV_DEVICE_TYPE_NONE => Self::None,
            ffi::G_UDEV_DEVICE_TYPE_BLOCK => Self::Block,
            ffi::G_UDEV_DEVICE_TYPE_CHAR => Self::Char,
            value => Self::__Unknown(value),
        }
    }
}

impl StaticType for DeviceType {
    #[inline]
    fn static_type() -> Type {
        unsafe { from_glib(ffi::g_udev_device_type_get_type()) }
    }
}

impl glib::HasParamSpec for DeviceType {
    type ParamSpec = glib::ParamSpecEnum;
    type SetValue = Self;
    type BuilderFn = fn(&str, Self) -> glib::ParamSpecEnumBuilder<Self>;

    fn param_spec_builder() -> Self::BuilderFn {
        |name, default_value| Self::ParamSpec::builder_with_default(name, default_value)
    }
}

impl glib::value::ValueType for DeviceType {
    type Type = Self;
}

unsafe impl<'a> FromValue<'a> for DeviceType {
    type Checker = glib::value::GenericValueTypeChecker<Self>;

    #[inline]
    unsafe fn from_value(value: &'a glib::Value) -> Self {
        from_glib(glib::gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl ToValue for DeviceType {
    #[inline]
    fn to_value(&self) -> glib::Value {
        let mut value = glib::Value::for_value_type::<Self>();
        unsafe {
            glib::gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, self.into_glib());
        }
        value
    }

    #[inline]
    fn value_type(&self) -> glib::Type {
        Self::static_type()
    }
}

impl From<DeviceType> for glib::Value {
    #[inline]
    fn from(v: DeviceType) -> Self {
        ToValue::to_value(&v)
    }
}
