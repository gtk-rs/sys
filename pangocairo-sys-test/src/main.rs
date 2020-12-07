#![allow(bad_style, unused_macros)]

extern crate libc;
extern crate pango_cairo_sys;

use pango_cairo_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
