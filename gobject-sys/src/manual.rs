// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[allow(unused_imports)]
use libc::{c_char, c_uint};
#[allow(unused_imports)]
use {GObject, GType, GValue};

extern "C" {
    #[cfg(any(feature = "v2_54", feature = "dox"))]
    pub fn g_object_new_with_properties(object_type: GType, n_properties: c_uint, names: *mut *const c_char, values: *const GValue) -> *mut GObject;
    #[cfg(any(feature = "v2_54", feature = "dox"))]
    pub fn g_object_getv(object: *mut GObject, n_properties: c_uint, names: *mut *const c_char, values: *mut GValue);
    #[cfg(any(feature = "v2_54", feature = "dox"))]
    pub fn g_object_setv(object: *mut GObject, n_properties: c_uint, names: *mut *const c_char, values: *const GValue);
}
