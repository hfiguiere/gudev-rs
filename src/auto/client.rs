// This file was generated by gir (https://github.com/gtk-rs/gir @ 907cd81)
// from gir-files (https://github.com/gtk-rs/gir-files @ e03533e)
// DO NOT EDIT

use crate::Device;
use crate::DeviceNumber;
use crate::DeviceType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Client(Object<ffi::GUdevClient, ffi::GUdevClientClass>);

    match fn {
        get_type => || ffi::g_udev_client_get_type(),
    }
}

impl Client {
    #[doc(alias = "g_udev_client_new")]
    pub fn new(subsystems: &[&str]) -> Client {
        unsafe {
            from_glib_full(ffi::g_udev_client_new(subsystems.to_glib_none().0))
        }
    }
}

pub const NONE_CLIENT: Option<&Client> = None;

pub trait ClientExt: 'static {
    #[doc(alias = "g_udev_client_query_by_device_file")]
    fn query_by_device_file(&self, device_file: &str) -> Option<Device>;

    #[doc(alias = "g_udev_client_query_by_device_number")]
    fn query_by_device_number(&self, type_: DeviceType, number: DeviceNumber) -> Option<Device>;

    #[doc(alias = "g_udev_client_query_by_subsystem")]
    fn query_by_subsystem(&self, subsystem: Option<&str>) -> Vec<Device>;

    #[doc(alias = "g_udev_client_query_by_subsystem_and_name")]
    fn query_by_subsystem_and_name(&self, subsystem: &str, name: &str) -> Option<Device>;

    #[doc(alias = "g_udev_client_query_by_sysfs_path")]
    fn query_by_sysfs_path(&self, sysfs_path: &str) -> Option<Device>;

    fn get_property_subsystems(&self) -> Vec<glib::GString>;

    fn connect_uevent<F: Fn(&Self, &str, &Device) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Client>> ClientExt for O {
    fn query_by_device_file(&self, device_file: &str) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_device_file(self.as_ref().to_glib_none().0, device_file.to_glib_none().0))
        }
    }

    fn query_by_device_number(&self, type_: DeviceType, number: DeviceNumber) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_device_number(self.as_ref().to_glib_none().0, type_.to_glib(), number))
        }
    }

    fn query_by_subsystem(&self, subsystem: Option<&str>) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_udev_client_query_by_subsystem(self.as_ref().to_glib_none().0, subsystem.to_glib_none().0))
        }
    }

    fn query_by_subsystem_and_name(&self, subsystem: &str, name: &str) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_subsystem_and_name(self.as_ref().to_glib_none().0, subsystem.to_glib_none().0, name.to_glib_none().0))
        }
    }

    fn query_by_sysfs_path(&self, sysfs_path: &str) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_sysfs_path(self.as_ref().to_glib_none().0, sysfs_path.to_glib_none().0))
        }
    }

    fn get_property_subsystems(&self) -> Vec<glib::GString> {
        unsafe {
            let mut value = glib::Value::from_type(<Vec<glib::GString> as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut glib::gobject_ffi::GObject, b"subsystems\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().expect("Return Value for property `subsystems` getter").unwrap()
        }
    }

    fn connect_uevent<F: Fn(&Self, &str, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn uevent_trampoline<P, F: Fn(&P, &str, &Device) + 'static>(this: *mut ffi::GUdevClient, action: *mut libc::c_char, device: *mut ffi::GUdevDevice, f: glib::ffi::gpointer)
            where P: IsA<Client>
        {
            let f: &F = &*(f as *const F);
            f(&Client::from_glib_borrow(this).unsafe_cast_ref(), &glib::GString::from_glib_borrow(action), &from_glib_borrow(device))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"uevent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(uevent_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Client")
    }
}
