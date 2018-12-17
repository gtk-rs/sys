#![allow(bad_style)]

extern crate pango_sys;
extern crate libc;

use pango_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));

