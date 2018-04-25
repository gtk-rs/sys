// Copyright 2013-2018, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

#[allow(unused_imports)]
use libc::{c_char, c_double, c_int, c_ushort, c_void, size_t};
use {GKeyFile, gboolean};

#[cfg(windows)]
pub type GPid = *mut c_void;

#[cfg(not(windows))]
pub type GPid = c_int;

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(all(windows,target_arch="x86_64"))]
pub struct GPollFD {
    pub fd: i64,
    pub events: c_ushort,
    pub revents: c_ushort,
}

#[repr(C)]
#[derive(Copy, Clone)]
#[cfg(not(all(windows,target_arch="x86_64")))]
pub struct GPollFD {
    pub fd: c_int,
    pub events: c_ushort,
    pub revents: c_ushort,
}

extern "C" {
    pub fn g_key_file_set_boolean_list(key_file: *mut GKeyFile, group_name: *const c_char, key: *const c_char, list: *mut gboolean, length: size_t);
    pub fn g_key_file_set_double_list(key_file: *mut GKeyFile, group_name: *const c_char, key: *const c_char, list: *mut c_double, length: size_t);
    pub fn g_key_file_set_integer_list(key_file: *mut GKeyFile, group_name: *const c_char, key: *const c_char, list: *mut c_int, length: size_t);
    pub fn g_key_file_set_locale_string_list(key_file: *mut GKeyFile, group_name: *const c_char, key: *const c_char, locale: *const c_char, list: *const *const c_char, length: size_t);
    pub fn g_key_file_set_string_list(key_file: *mut GKeyFile, group_name: *const c_char, key: *const c_char, list: *const *const c_char, length: size_t);
}
