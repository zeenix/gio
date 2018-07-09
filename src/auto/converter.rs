// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Converter(Object<ffi::GConverter, ffi::GConverterIface>);

    match fn {
        get_type => || ffi::g_converter_get_type(),
    }
}

pub trait ConverterExt {
    fn reset(&self);
}

impl<O: IsA<Converter>> ConverterExt for O {
    fn reset(&self) {
        unsafe {
            ffi::g_converter_reset(self.to_glib_none().0);
        }
    }
}