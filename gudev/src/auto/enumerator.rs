// Generated by gir (https://github.com/gtk-rs/gir @ 43b9242f6689)
// from
// from gir-files (https://github.com/gtk-rs/gir-files.git @ 3ff4d3275258)
// DO NOT EDIT

use crate::{Client, Device};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GUdevEnumerator")]
    pub struct Enumerator(Object<ffi::GUdevEnumerator, ffi::GUdevEnumeratorClass>);

    match fn {
        type_ => || ffi::g_udev_enumerator_get_type(),
    }
}

impl Enumerator {
    pub const NONE: Option<&'static Enumerator> = None;

    #[doc(alias = "g_udev_enumerator_new")]
    pub fn new(client: &impl IsA<Client>) -> Enumerator {
        unsafe { from_glib_full(ffi::g_udev_enumerator_new(client.as_ref().to_glib_none().0)) }
    }
}

pub trait EnumeratorExt: 'static {
    #[doc(alias = "g_udev_enumerator_add_match_is_initialized")]
    #[must_use]
    fn add_match_is_initialized(&self) -> Option<Enumerator>;

    #[doc(alias = "g_udev_enumerator_add_match_name")]
    #[must_use]
    fn add_match_name(&self, name: &str) -> Option<Enumerator>;

    #[doc(alias = "g_udev_enumerator_add_match_property")]
    #[must_use]
    fn add_match_property(&self, name: &str, value: &str) -> Option<Enumerator>;

    #[doc(alias = "g_udev_enumerator_add_match_subsystem")]
    #[must_use]
    fn add_match_subsystem(&self, subsystem: &str) -> Option<Enumerator>;

    #[doc(alias = "g_udev_enumerator_add_match_sysfs_attr")]
    #[must_use]
    fn add_match_sysfs_attr(&self, name: &str, value: &str) -> Option<Enumerator>;

    #[doc(alias = "g_udev_enumerator_add_match_tag")]
    #[must_use]
    fn add_match_tag(&self, tag: &str) -> Option<Enumerator>;

    #[doc(alias = "g_udev_enumerator_add_nomatch_subsystem")]
    #[must_use]
    fn add_nomatch_subsystem(&self, subsystem: &str) -> Option<Enumerator>;

    #[doc(alias = "g_udev_enumerator_add_nomatch_sysfs_attr")]
    #[must_use]
    fn add_nomatch_sysfs_attr(&self, name: &str, value: &str) -> Option<Enumerator>;

    #[doc(alias = "g_udev_enumerator_add_sysfs_path")]
    #[must_use]
    fn add_sysfs_path(&self, sysfs_path: &str) -> Option<Enumerator>;

    #[doc(alias = "g_udev_enumerator_execute")]
    fn execute(&self) -> Vec<Device>;

    fn client(&self) -> Option<Client>;
}

impl<O: IsA<Enumerator>> EnumeratorExt for O {
    fn add_match_is_initialized(&self) -> Option<Enumerator> {
        unsafe {
            from_glib_none(ffi::g_udev_enumerator_add_match_is_initialized(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn add_match_name(&self, name: &str) -> Option<Enumerator> {
        unsafe {
            from_glib_none(ffi::g_udev_enumerator_add_match_name(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
            ))
        }
    }

    fn add_match_property(&self, name: &str, value: &str) -> Option<Enumerator> {
        unsafe {
            from_glib_none(ffi::g_udev_enumerator_add_match_property(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn add_match_subsystem(&self, subsystem: &str) -> Option<Enumerator> {
        unsafe {
            from_glib_none(ffi::g_udev_enumerator_add_match_subsystem(
                self.as_ref().to_glib_none().0,
                subsystem.to_glib_none().0,
            ))
        }
    }

    fn add_match_sysfs_attr(&self, name: &str, value: &str) -> Option<Enumerator> {
        unsafe {
            from_glib_none(ffi::g_udev_enumerator_add_match_sysfs_attr(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn add_match_tag(&self, tag: &str) -> Option<Enumerator> {
        unsafe {
            from_glib_none(ffi::g_udev_enumerator_add_match_tag(
                self.as_ref().to_glib_none().0,
                tag.to_glib_none().0,
            ))
        }
    }

    fn add_nomatch_subsystem(&self, subsystem: &str) -> Option<Enumerator> {
        unsafe {
            from_glib_none(ffi::g_udev_enumerator_add_nomatch_subsystem(
                self.as_ref().to_glib_none().0,
                subsystem.to_glib_none().0,
            ))
        }
    }

    fn add_nomatch_sysfs_attr(&self, name: &str, value: &str) -> Option<Enumerator> {
        unsafe {
            from_glib_none(ffi::g_udev_enumerator_add_nomatch_sysfs_attr(
                self.as_ref().to_glib_none().0,
                name.to_glib_none().0,
                value.to_glib_none().0,
            ))
        }
    }

    fn add_sysfs_path(&self, sysfs_path: &str) -> Option<Enumerator> {
        unsafe {
            from_glib_none(ffi::g_udev_enumerator_add_sysfs_path(
                self.as_ref().to_glib_none().0,
                sysfs_path.to_glib_none().0,
            ))
        }
    }

    fn execute(&self) -> Vec<Device> {
        unsafe {
            FromGlibPtrContainer::from_glib_full(ffi::g_udev_enumerator_execute(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn client(&self) -> Option<Client> {
        glib::ObjectExt::property(self.as_ref(), "client")
    }
}

impl fmt::Display for Enumerator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Enumerator")
    }
}
