// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Cancellable;
use Error;
use ffi;
#[cfg(feature = "futures")]
use futures_core;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "futures")]
use std::boxed::Box as Box_;
use std::fmt;
use std::ptr;

glib_wrapper! {
    pub struct ProxyResolver(Interface<ffi::GProxyResolver>);

    match fn {
        get_type => || ffi::g_proxy_resolver_get_type(),
    }
}

impl ProxyResolver {
    pub fn get_default() -> Option<ProxyResolver> {
        unsafe {
            from_glib_none(ffi::g_proxy_resolver_get_default())
        }
    }
}

pub const NONE_PROXY_RESOLVER: Option<&ProxyResolver> = None;

pub trait ProxyResolverExt: 'static {
    fn is_supported(&self) -> bool;

    fn lookup<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>>(&self, uri: &str, cancellable: Q) -> Result<Vec<GString>, Error>;

    fn lookup_async<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<Vec<GString>, Error>) + Send + 'static>(&self, uri: &str, cancellable: Q, callback: R);

    #[cfg(feature = "futures")]
    fn lookup_async_future(&self, uri: &str) -> Box_<futures_core::Future<Item = (Self, Vec<GString>), Error = (Self, Error)>> where Self: Sized + Clone;
}

impl<O: IsA<ProxyResolver>> ProxyResolverExt for O {
    fn is_supported(&self) -> bool {
        unsafe {
            from_glib(ffi::g_proxy_resolver_is_supported(self.as_ref().to_glib_none().0))
        }
    }

    fn lookup<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>>(&self, uri: &str, cancellable: Q) -> Result<Vec<GString>, Error> {
        let cancellable = cancellable.into();
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_proxy_resolver_lookup(self.as_ref().to_glib_none().0, uri.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn lookup_async<'a, P: IsA<Cancellable> + 'a, Q: Into<Option<&'a P>>, R: FnOnce(Result<Vec<GString>, Error>) + Send + 'static>(&self, uri: &str, cancellable: Q, callback: R) {
        let cancellable = cancellable.into();
        let user_data: Box<Box<R>> = Box::new(Box::new(callback));
        unsafe extern "C" fn lookup_async_trampoline<R: FnOnce(Result<Vec<GString>, Error>) + Send + 'static>(_source_object: *mut gobject_ffi::GObject, res: *mut ffi::GAsyncResult, user_data: glib_ffi::gpointer)
        {
            let mut error = ptr::null_mut();
            let ret = ffi::g_proxy_resolver_lookup_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) };
            let callback: Box<Box<R>> = Box::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = lookup_async_trampoline::<R>;
        unsafe {
            ffi::g_proxy_resolver_lookup_async(self.as_ref().to_glib_none().0, uri.to_glib_none().0, cancellable.map(|p| p.as_ref()).to_glib_none().0, Some(callback), Box::into_raw(user_data) as *mut _);
        }
    }

    #[cfg(feature = "futures")]
    fn lookup_async_future(&self, uri: &str) -> Box_<futures_core::Future<Item = (Self, Vec<GString>), Error = (Self, Error)>> where Self: Sized + Clone {
        use GioFuture;
        use fragile::Fragile;

        let uri = String::from(uri);
        GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            let send = Fragile::new(send);
            let obj_clone = Fragile::new(obj.clone());
            obj.lookup_async(
                 &uri,
                 Some(&cancellable),
                 move |res| {
                     let obj = obj_clone.into_inner();
                     let res = res.map(|v| (obj.clone(), v)).map_err(|v| (obj.clone(), v));
                     let _ = send.into_inner().send(res);
                 },
            );

            cancellable
        })
    }
}

impl fmt::Display for ProxyResolver {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ProxyResolver")
    }
}
