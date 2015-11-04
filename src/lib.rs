
#![deny(missing_docs,
        missing_debug_implementations,
        missing_copy_implementations,
        trivial_casts,
        trivial_numeric_casts,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]

//! Rust FFI helpers for working with win32 API's "Unicode" functions that uses "wide" strings.


mod wcstr;
mod wcstring;
mod error;

pub use error::{NulError, NoNulError};
pub use wcstr::WCStr;
pub use wcstring::WCString;
