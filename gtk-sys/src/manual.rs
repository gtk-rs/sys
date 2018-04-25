// Copyright 2013-2017, The Gtk-rs Project Developers.
// See the COPYRIGHT file at the top-level directory of this distribution.
// Licensed under the MIT license, see the LICENSE file or <http://opensource.org/licenses/MIT>

use libc::{c_char, c_int};
use {GtkIconInfo, GtkIconLookupFlags, GtkIconTheme};

pub mod xlib {
    pub type Window = i32;
}

extern "C" {
    pub fn gtk_icon_theme_choose_icon(icon_theme: *mut GtkIconTheme, icon_names: *mut *const c_char, size: c_int, flags: GtkIconLookupFlags) -> *mut GtkIconInfo;
    #[cfg(any(feature = "v3_10", feature = "dox"))]
    pub fn gtk_icon_theme_choose_icon_for_scale(icon_theme: *mut GtkIconTheme, icon_names: *mut *const c_char, size: c_int, scale: c_int, flags: GtkIconLookupFlags) -> *mut GtkIconInfo;
    pub fn gtk_icon_theme_get_search_path(icon_theme: *mut GtkIconTheme, path: *mut *mut *mut c_char, n_elements: *mut c_int);
    pub fn gtk_icon_theme_set_search_path(icon_theme: *mut GtkIconTheme, path: *mut *const c_char, n_elements: c_int);
}
