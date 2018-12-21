#![allow(bad_style)]

extern crate atk_sys;
extern crate libc;

use atk_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
