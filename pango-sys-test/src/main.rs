#![allow(bad_style)]

extern crate libc;
extern crate pango_sys;

use libc::*;
use pango_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
