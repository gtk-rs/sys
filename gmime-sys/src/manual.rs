#[allow(unused_imports)]
use libc::{c_int, c_ushort, c_void};

pub type _GMimeFilterPrivate = c_void;
pub type _GMimeFilterGZipPrivate = c_void;
pub type _GMimeParserPrivate = c_void;
pub type _GMimeStreamFilterPrivate = c_void;
pub type _UrlScanner = c_void;
pub type _cat_node = c_void;


// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct _cat_node {
//     pub next: *mut _cat_node,
//     pub stream: *mut GMimeStream,
//     pub position: i64,
//     pub id: i32
// }
