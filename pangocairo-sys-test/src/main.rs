#![allow(bad_style, unused_macros)]

extern crate pango_cairo_sys;
extern crate libc;

use pango_cairo_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));

