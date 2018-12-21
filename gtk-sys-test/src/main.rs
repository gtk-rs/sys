#![allow(bad_style, unused_imports)]

extern crate gtk_sys;
extern crate libc;

use libc::*;
use gtk_sys::*;
use gtk_sys::xlib::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));

