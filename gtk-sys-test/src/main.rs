#![allow(bad_style)]

extern crate gtk_sys;
extern crate libc;

use gtk_sys::*;
use gtk_sys::xlib::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));

