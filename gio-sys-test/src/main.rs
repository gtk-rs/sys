#![allow(bad_style)]

extern crate gio_sys;
extern crate libc;

use gio_sys::*;
use libc::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
