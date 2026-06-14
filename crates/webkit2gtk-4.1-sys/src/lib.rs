//! Rust FFI bindings for webkit2gtk-4.1
//! 
//! This crate provides raw FFI bindings for the webkit2gtk-4.1 library,
//! which is a newer version of WebKitGTK with better Wayland support.

pub fn webkit_get_major_version() -> i32 {
    unsafe { webkit2gtk_ffi::webkit_get_major_version() }
}

pub fn webkit_get_minor_version() -> i32 {
    unsafe { webkit2gtk_ffi::webkit_get_minor_version() }
}

pub fn webkit_get_micro_version() -> i32 {
    unsafe { webkit2gtk_ffi::webkit_get_micro_version() }
}
