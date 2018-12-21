#![allow(bad_style)]

extern crate glib_sys;
extern crate gobject_sys;
extern crate libc;

use glib_sys::GType;
use gobject_sys::*;
use libc::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));

