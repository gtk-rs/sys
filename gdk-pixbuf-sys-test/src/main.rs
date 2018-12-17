#![allow(bad_style)]

extern crate gdk_pixbuf_sys;
extern crate libc;

use gdk_pixbuf_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));

