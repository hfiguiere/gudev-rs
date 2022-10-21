// Generated by gir (https://github.com/gtk-rs/gir @ e0d8d8d645b1)
// from gir (https://github.com/gtk-rs/gir.git @ e0d8d8d645b1)
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 3ff4d3275258)
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
    /// [`Client`][crate::Client] is used to query information about devices on a Linux
    /// system from the Linux kernel and the udev device
    /// manager.
    ///
    /// Device information is retrieved from the kernel (through the
    /// `<literal>`sysfs`</literal>` filesystem) and the udev daemon (through a
    /// `<literal>`tmpfs`</literal>` filesystem) and presented through
    /// [`Device`][crate::Device] objects. This means that no blocking IO ever happens
    /// (in both cases, we are essentially just reading data from kernel
    /// memory) and as such there are no asynchronous versions of the
    /// provided methods.
    ///
    /// To get [`Device`][crate::Device] objects, use
    /// [`ClientExt::query_by_subsystem()`][crate::prelude::ClientExt::query_by_subsystem()],
    /// [`ClientExt::query_by_device_number()`][crate::prelude::ClientExt::query_by_device_number()],
    /// [`ClientExt::query_by_device_file()`][crate::prelude::ClientExt::query_by_device_file()],
    /// [`ClientExt::query_by_sysfs_path()`][crate::prelude::ClientExt::query_by_sysfs_path()],
    /// [`ClientExt::query_by_subsystem_and_name()`][crate::prelude::ClientExt::query_by_subsystem_and_name()]
    /// or the [`Enumerator`][crate::Enumerator] type.
    ///
    /// To listen to uevents, connect to the `signal::Client::uevent` signal.
    ///
    /// # Implements
    ///
    /// [`ClientExt`][trait@crate::prelude::ClientExt]
    // rustdoc-stripper-ignore-next-stop
    /// [`Client`][crate::Client] is used to query information about devices on a Linux
    /// system from the Linux kernel and the udev device
    /// manager.
    ///
    /// Device information is retrieved from the kernel (through the
    /// `<literal>`sysfs`</literal>` filesystem) and the udev daemon (through a
    /// `<literal>`tmpfs`</literal>` filesystem) and presented through
    /// [`Device`][crate::Device] objects. This means that no blocking IO ever happens
    /// (in both cases, we are essentially just reading data from kernel
    /// memory) and as such there are no asynchronous versions of the
    /// provided methods.
    ///
    /// To get [`Device`][crate::Device] objects, use
    /// [`ClientExt::query_by_subsystem()`][crate::prelude::ClientExt::query_by_subsystem()],
    /// [`ClientExt::query_by_device_number()`][crate::prelude::ClientExt::query_by_device_number()],
    /// [`ClientExt::query_by_device_file()`][crate::prelude::ClientExt::query_by_device_file()],
    /// [`ClientExt::query_by_sysfs_path()`][crate::prelude::ClientExt::query_by_sysfs_path()],
    /// [`ClientExt::query_by_subsystem_and_name()`][crate::prelude::ClientExt::query_by_subsystem_and_name()]
    /// or the [`Enumerator`][crate::Enumerator] type.
    ///
    /// To listen to uevents, connect to the `signal::Client::uevent` signal.
    ///
    /// # Implements
    ///
    /// [`ClientExt`][trait@crate::prelude::ClientExt]
    #[doc(alias = "GUdevClient")]
    pub struct Client(Object<ffi::GUdevClient, ffi::GUdevClientClass>);

    match fn {
        type_ => || ffi::g_udev_client_get_type(),
    }
}

impl Client {
    pub const NONE: Option<&'static Client> = None;

    /// Constructs a [`Client`][crate::Client] object that can be used to query
    /// information about devices. Connect to the `signal::Client::uevent`
    /// signal to listen for uevents. Note that signals are emitted in the
    /// <link linkend="g-main-context-push-thread-default">thread-default main loop`</link>`
    /// of the thread that you call this constructor from.
    /// ## `subsystems`
    /// A [`None`] terminated string array of subsystems to listen for uevents on, [`None`] to not listen on uevents at all, or an empty array to listen to uevents on all subsystems. See the documentation for the `property::Client::subsystems` property for details on this parameter.
    ///
    /// # Returns
    ///
    /// A new [`Client`][crate::Client] object. Free with `g_object_unref()`.
    // rustdoc-stripper-ignore-next-stop
    /// Constructs a [`Client`][crate::Client] object that can be used to query
    /// information about devices. Connect to the `signal::Client::uevent`
    /// signal to listen for uevents. Note that signals are emitted in the
    /// <link linkend="g-main-context-push-thread-default">thread-default main loop`</link>`
    /// of the thread that you call this constructor from.
    /// ## `subsystems`
    /// A [`None`] terminated string array of subsystems to listen for uevents on, [`None`] to not listen on uevents at all, or an empty array to listen to uevents on all subsystems. See the documentation for the `property::Client::subsystems` property for details on this parameter.
    ///
    /// # Returns
    ///
    /// A new [`Client`][crate::Client] object. Free with `g_object_unref()`.
    #[doc(alias = "g_udev_client_new")]
    pub fn new(subsystems: &[&str]) -> Client {
        unsafe { from_glib_full(ffi::g_udev_client_new(subsystems.to_glib_none().0)) }
    }
}

/// Trait containing all [`struct@Client`] methods.
///
/// # Implementors
///
/// [`Client`][struct@crate::Client]
// rustdoc-stripper-ignore-next-stop
/// Trait containing all [`struct@Client`] methods.
///
/// # Implementors
///
/// [`Client`][struct@crate::Client]
pub trait ClientExt: 'static {
    /// Looks up a device for a device file.
    /// ## `device_file`
    /// A device file.
    ///
    /// # Returns
    ///
    /// A [`Device`][crate::Device] object or [`None`]
    /// if the device was not found. Free with `g_object_unref()`.
    // rustdoc-stripper-ignore-next-stop
    /// Looks up a device for a device file.
    /// ## `device_file`
    /// A device file.
    ///
    /// # Returns
    ///
    /// A [`Device`][crate::Device] object or [`None`]
    /// if the device was not found. Free with `g_object_unref()`.
    #[doc(alias = "g_udev_client_query_by_device_file")]
    fn query_by_device_file(&self, device_file: &str) -> Option<Device>;

    /// Looks up a device for a type and device number.
    /// ## `type_`
    /// A value from the [`DeviceType`][crate::DeviceType] enumeration.
    /// ## `number`
    /// A device number.
    ///
    /// # Returns
    ///
    /// A [`Device`][crate::Device] object or [`None`]
    /// if the device was not found. Free with `g_object_unref()`.
    // rustdoc-stripper-ignore-next-stop
    /// Looks up a device for a type and device number.
    /// ## `type_`
    /// A value from the [`DeviceType`][crate::DeviceType] enumeration.
    /// ## `number`
    /// A device number.
    ///
    /// # Returns
    ///
    /// A [`Device`][crate::Device] object or [`None`]
    /// if the device was not found. Free with `g_object_unref()`.
    #[doc(alias = "g_udev_client_query_by_device_number")]
    fn query_by_device_number(&self, type_: DeviceType, number: DeviceNumber) -> Option<Device>;

    /// Gets all devices belonging to `subsystem`.
    /// ## `subsystem`
    /// The subsystem to get devices for or [`None`] to get all devices.
    ///
    /// # Returns
    ///
    /// A
    /// list of [`Device`][crate::Device] objects. The caller should free the result by
    /// using `g_object_unref()` on each element in the list and then
    /// `g_list_free()` on the list.
    // rustdoc-stripper-ignore-next-stop
    /// Gets all devices belonging to `subsystem`.
    /// ## `subsystem`
    /// The subsystem to get devices for or [`None`] to get all devices.
    ///
    /// # Returns
    ///
    /// A
    /// list of [`Device`][crate::Device] objects. The caller should free the result by
    /// using `g_object_unref()` on each element in the list and then
    /// `g_list_free()` on the list.
    #[doc(alias = "g_udev_client_query_by_subsystem")]
    fn query_by_subsystem(&self, subsystem: Option<&str>) -> Vec<Device>;

    /// Looks up a device for a subsystem and name.
    /// ## `subsystem`
    /// A subsystem name.
    /// ## `name`
    /// The name of the device.
    ///
    /// # Returns
    ///
    /// A [`Device`][crate::Device] object or [`None`]
    /// if the device was not found. Free with `g_object_unref()`.
    // rustdoc-stripper-ignore-next-stop
    /// Looks up a device for a subsystem and name.
    /// ## `subsystem`
    /// A subsystem name.
    /// ## `name`
    /// The name of the device.
    ///
    /// # Returns
    ///
    /// A [`Device`][crate::Device] object or [`None`]
    /// if the device was not found. Free with `g_object_unref()`.
    #[doc(alias = "g_udev_client_query_by_subsystem_and_name")]
    fn query_by_subsystem_and_name(&self, subsystem: &str, name: &str) -> Option<Device>;

    /// Looks up a device for a sysfs path.
    /// ## `sysfs_path`
    /// A sysfs path.
    ///
    /// # Returns
    ///
    /// A [`Device`][crate::Device] object or [`None`]
    /// if the device was not found. Free with `g_object_unref()`.
    // rustdoc-stripper-ignore-next-stop
    /// Looks up a device for a sysfs path.
    /// ## `sysfs_path`
    /// A sysfs path.
    ///
    /// # Returns
    ///
    /// A [`Device`][crate::Device] object or [`None`]
    /// if the device was not found. Free with `g_object_unref()`.
    #[doc(alias = "g_udev_client_query_by_sysfs_path")]
    fn query_by_sysfs_path(&self, sysfs_path: &str) -> Option<Device>;

    /// The subsystems to listen for uevents on.
    ///
    /// To listen for only a specific DEVTYPE for a given SUBSYSTEM, use
    /// "subsystem/devtype". For example, to only listen for uevents
    /// where SUBSYSTEM is usb and DEVTYPE is usb_interface, use
    /// "usb/usb_interface".
    ///
    /// If this property is [`None`], then no events will be reported. If
    /// it's the empty array, events from all subsystems will be
    /// reported.
    // rustdoc-stripper-ignore-next-stop
    /// The subsystems to listen for uevents on.
    ///
    /// To listen for only a specific DEVTYPE for a given SUBSYSTEM, use
    /// "subsystem/devtype". For example, to only listen for uevents
    /// where SUBSYSTEM is usb and DEVTYPE is usb_interface, use
    /// "usb/usb_interface".
    ///
    /// If this property is [`None`], then no events will be reported. If
    /// it's the empty array, events from all subsystems will be
    /// reported.
    fn subsystems(&self) -> Vec<glib::GString>;

    /// Emitted when `client` receives an uevent.
    ///
    /// Note that while you'll have access to all the device's properties and attributes
    /// for the majority of actions, only the sysfs path will be available when the device
    /// is removed.
    ///
    /// Also note that the action is an arbitrary string, controlled by device drivers. Other
    /// values than those listed is possible, but unlikely.
    ///
    /// This signal is emitted in the
    /// <link linkend="g-main-context-push-thread-default">thread-default main loop`</link>`
    /// of the thread that `client` was created in.
    /// ## `action`
    /// The action for the uevent e.g. "add", "remove", "change", "move",
    ///  "online" or "offline"
    /// ## `device`
    /// Details about the [`Device`][crate::Device] the event is for.
    // rustdoc-stripper-ignore-next-stop
    /// Emitted when `client` receives an uevent.
    ///
    /// Note that while you'll have access to all the device's properties and attributes
    /// for the majority of actions, only the sysfs path will be available when the device
    /// is removed.
    ///
    /// Also note that the action is an arbitrary string, controlled by device drivers. Other
    /// values than those listed is possible, but unlikely.
    ///
    /// This signal is emitted in the
    /// <link linkend="g-main-context-push-thread-default">thread-default main loop`</link>`
    /// of the thread that `client` was created in.
    /// ## `action`
    /// The action for the uevent e.g. "add", "remove", "change", "move",
    ///  "online" or "offline"
    /// ## `device`
    /// Details about the [`Device`][crate::Device] the event is for.
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
