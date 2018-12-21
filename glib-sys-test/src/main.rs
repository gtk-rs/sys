#![allow(bad_style)]

extern crate glib_sys;
extern crate libc;

use glib_sys::*;
use libc::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
