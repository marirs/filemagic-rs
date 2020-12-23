//! This crate provides bindings for `libmagic`, which recognizes the
//! type of data contained in a file (or buffer).
//!
//! You should be already familiar with `libmagic`'s CLI command `file`:
//! ```sh
//! $ file rust-logo-128x128-blk.png
//! rust-logo-128x128-blk.png: PNG image data, 128 x 128, 8-bit/color RGBA, non-interlaced
//! ```
//!
//! # Usage example
//!
//! Here's an example of using this crate:
//!
//! ```no_run
//! use filemagic::Magic;
//!
//! fn main() {
//!    let test_file = "path/to/file";
//!    let cookie = Magic::open(Default::default()).expect("error");
//!     cookie.load::<String>(&[]).expect("error");
//!     let magic = cookie.file(&test_file).expect("error in magic");
//!     println!("magic= {}", magic);
//! }
//! ```
#![crate_type = "lib"]
#[macro_use]
extern crate bitflags;
#[macro_use]
pub mod macros;

extern crate libc;
use libc::{c_char, size_t};

mod api;

pub mod version;
pub use version::version;

pub mod flags;
pub use flags::Flags;

#[cfg(test)]
mod tests;

use std::{
    error,
    ffi::{CStr, CString},
    fmt::Display,
    path::Path,
    ptr, str,
};

fn db_filenames<P: AsRef<Path>>(filenames: &[P]) -> *const c_char {
    match filenames.len() {
        0 => ptr::null(),
        // FIXME: This is just plain wrong. I'm surprised it works at all..
        1 => CString::new(filenames[0].as_ref().to_string_lossy().into_owned())
            .unwrap()
            .into_raw(),
        _ => unimplemented!(),
    }
}

/// The error type used in this crate
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct FileMagicError {
    pub desc: String,
}

impl error::Error for FileMagicError {
    fn description(&self) -> &str {
        "internal libmagic error"
    }
}

impl Display for FileMagicError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.desc)
    }
}

/// Configuration of which `Flags` and magic databases to use
pub struct Magic {
    magic: *const api::Magic,
}

impl Drop for Magic {
    /// Closes the magic database and deallocates any resources used
    fn drop(&mut self) {
        unsafe { api::magic_close(self.magic) }
    }
}

impl Magic {
    fn last_error(&self) -> Option<FileMagicError> {
        let cookie = self.magic;

        unsafe {
            let e = api::magic_error(cookie);
            if e.is_null() {
                None
            } else {
                let slice = CStr::from_ptr(e).to_bytes();
                Some(self::FileMagicError {
                    desc: str::from_utf8(slice).unwrap().to_string(),
                })
            }
        }
    }

    fn magic_failure(&self) -> FileMagicError {
        match self.last_error() {
            Some(e) => e,
            None => self::FileMagicError {
                desc: "unknown error".to_string(),
            },
        }
    }

    /// Returns a textual explanation of the last error, if any
    ///
    /// You should not need to call this, since you can use the `FileMagicError` in
    /// the `Result` returned by the other functions.
    // TODO: Remove this entirely?
    pub fn error(&self) -> Option<String> {
        unsafe {
            let str = api::magic_error(self.magic);
            if str.is_null() {
                None
            } else {
                let slice = CStr::from_ptr(str).to_bytes();
                Some(str::from_utf8(slice).unwrap().to_string())
            }
        }
    }

    /// Returns a textual description of the contents of the `filename`
    pub fn file<P: AsRef<Path>>(&self, filename: P) -> Result<String, FileMagicError> {
        let cookie = self.magic;
        let f = CString::new(filename.as_ref().to_string_lossy().into_owned())
            .unwrap()
            .into_raw();
        unsafe {
            let str = api::magic_file(cookie, f);
            if str.is_null() {
                Err(self.magic_failure())
            } else {
                let slice = CStr::from_ptr(str).to_bytes();
                Ok(str::from_utf8(slice).unwrap().to_string())
            }
        }
    }

    /// Returns a textual description of the contents of the `buffer`
    pub fn buffer(&self, buffer: &[u8]) -> Result<String, FileMagicError> {
        let buffer_len = buffer.len() as size_t;
        let pbuffer = buffer.as_ptr();
        unsafe {
            let str = api::magic_buffer(self.magic, pbuffer, buffer_len);
            if str.is_null() {
                Err(self.magic_failure())
            } else {
                let slice = CStr::from_ptr(str).to_bytes();
                Ok(str::from_utf8(slice).unwrap().to_string())
            }
        }
    }

    /// Check the validity of entries in the database `filenames`
    pub fn check<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), FileMagicError> {
        let cookie = self.magic;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = api::magic_check(cookie, db_filenames);
        }
        if 0 == ret { Ok(()) } else { Err(self.magic_failure()) }
    }

    /// Compiles the given database `filenames` for faster access
    ///
    /// The compiled files created are named from the `basename` of each file argument with '.mgc' appended to it.
    pub fn compile<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), FileMagicError> {
        let cookie = self.magic;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = api::magic_compile(cookie, db_filenames);
        }
        if 0 == ret { Ok(()) } else { Err(self.magic_failure()) }
    }

    /// Dumps all magic entries in the given database `filenames` in a human readable format
    pub fn list<P: AsRef<Path>>(&self, filenames: &[P]) -> Result<(), FileMagicError> {
        let cookie = self.magic;
        let db_filenames = db_filenames(filenames);
        let ret;

        unsafe {
            ret = api::magic_list(cookie, db_filenames);
        }
        if 0 == ret { Ok(()) } else { Err(self.magic_failure()) }
    }

    /// Sets the flags to use
    ///
    /// Overwrites any previously set flags, e.g. those from `load()`.
    // TODO: libmagic itself has to magic_getflags, but we could remember them in Cookie?
    pub fn set_flags(&self, flags: Flags) -> bool {
        unsafe { api::magic_setflags(self.magic, flags.bits()) != -1 }
    }

    /// Creates a new configuration, `flags` specify how other functions should behave
    ///
    /// This does not `load()` any databases yet.
    pub fn open(flags: Flags) -> Result<Magic, FileMagicError> {
        let cookie;
        unsafe {
            cookie = api::magic_open((flags | Flags::ERROR).bits());
        }
        if cookie.is_null() {
            Err(self::FileMagicError {
                desc: "errno".to_string(),
            })
        } else {
            Ok(Magic { magic: cookie })
        }
    }

    /// Loads the given database `filenames` for further queries
    ///
    /// Adds '.mgc'	to the database	files as appropriate.
    pub fn load<P: AsRef<Path>>(&self, magic_databases: &[P]) -> Result<(), FileMagicError> {
        let cookie = self.magic;
        let db_filenames = db_filenames(magic_databases);
        let ret;

        unsafe {
            ret = api::magic_load(cookie, db_filenames);
        }
        if 0 == ret {
            Ok(())
        } else {
            Err(self.magic_failure())
        }
    }
}
