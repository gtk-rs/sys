#![allow(bad_style)]

extern crate gdk_pixbuf_sys;
extern crate libc;

use gdk_pixbuf_sys::*;
use libc::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));

