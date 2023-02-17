// Generated by gir (https://github.com/gtk-rs/gir @ 43b9242f6689)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 3ff4d3275258)
// DO NOT EDIT

use crate::{Device, DeviceNumber, DeviceType};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GUdevClient")]
    pub struct Client(Object<ffi::GUdevClient, ffi::GUdevClientClass>);

    match fn {
        type_ => || ffi::g_udev_client_get_type(),
    }
}

impl Client {
    pub const NONE: Option<&'static Client> = None;

    #[doc(alias = "g_udev_client_new")]
    pub fn new(subsystems: &[&str]) -> Client {
        unsafe { from_glib_full(ffi::g_udev_client_new(subsystems.to_glib_none().0)) }
    }
}

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

    fn subsystems(&self) -> Vec<glib::GString>;

    #[doc(alias = "uevent")]
    fn connect_uevent<F: Fn(&Self, &str, &Device) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Client>> ClientExt for O {
    fn query_by_device_file(&self, device_file: &str) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_device_file(
                self.as_ref().to_glib_none().0,
                device_file.to_glib_none().0,
            ))
        }
    }

    fn query_by_device_number(&self, type_: DeviceType, number: DeviceNumber) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_device_number(
                self.as_ref().to_glib_none().0,
                type_.into_glib(),
                number,
            ))
        }
    }

    fn query_by_subsystem(&self, subsystem: Option<&str>) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_udev_client_query_by_subsystem(
                self.as_ref().to_glib_none().0,
                subsystem.to_glib_none().0,
            ))
        }
    }

    fn query_by_subsystem_and_name(&self, subsystem: &str, name: &str) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_subsystem_and_name(
                self.as_ref().to_glib_none().0,
                subsystem.to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn query_by_sysfs_path(&self, sysfs_path: &str) -> Option<Device> {
        unsafe {
            from_glib_full(ffi::g_udev_client_query_by_sysfs_path(
                self.as_ref().to_glib_none().0,
                sysfs_path.to_glib_none().0,
            ))
        }
    }

    fn subsystems(&self) -> Vec<glib::GString> {
        glib::ObjectExt::property(self.as_ref(), "subsystems")
    }

    fn connect_uevent<F: Fn(&Self, &str, &Device) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn uevent_trampoline<
            P: IsA<Client>,
            F: Fn(&P, &str, &Device) + 'static,
        >(
            this: *mut ffi::GUdevClient,
            action: *mut libc::c_char,
            device: *mut ffi::GUdevDevice,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                Client::from_glib_borrow(this).unsafe_cast_ref(),
                &glib::GString::from_glib_borrow(action),
                &from_glib_borrow(device),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"uevent\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    uevent_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Client")
    }
}
