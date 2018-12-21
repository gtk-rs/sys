#![allow(bad_style)]

extern crate gdk_sys;
extern crate glib_sys;
extern crate libc;

use gdk_sys::*;
use glib_sys::gboolean;
use libc::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
