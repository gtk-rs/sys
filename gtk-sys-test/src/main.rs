#![allow(bad_style, unused_imports)]

extern crate gtk_sys;
extern crate libc;

use gtk_sys::xlib::*;
use gtk_sys::*;
use libc::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
