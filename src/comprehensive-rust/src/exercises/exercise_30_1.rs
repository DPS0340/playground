// TODO: remove this when you're done with your implementation.
#![allow(unused_imports, unused_variables, dead_code)]

mod ffi {
    use std::os::raw::{c_char, c_int};
    #[cfg(not(target_os = "macos"))]
    use std::os::raw::{c_long, c_ulong, c_ushort};

    // Opaque type. See https://doc.rust-lang.org/nomicon/ffi.html.
    #[repr(C)]
    pub struct DIR {
        _data: [u8; 0],
        _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
    }

    // Layout as per readdir(3) and definitions in /usr/include/x86_64-linux-gnu.
    #[cfg(not(target_os = "macos"))]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: c_long,
        pub d_off: c_ulong,
        pub d_reclen: c_ushort,
        pub d_type: c_char,
        pub d_name: [c_char; 256],
    }

    // Layout as per man entry for dirent
    #[cfg(target_os = "macos")]
    #[repr(C)]
    pub struct dirent {
        pub d_ino: u64,
        pub d_seekoff: u64,
        pub d_reclen: u16,
        pub d_namlen: u16,
        pub d_type: u8,
        pub d_name: [c_char; 1024],
    }

    extern "C" {
        pub fn opendir(s: *const c_char) -> *mut DIR;
        pub fn readdir(s: *mut DIR) -> *const dirent;
        pub fn closedir(s: *mut DIR) -> c_int;
    }
}

use std::ffi::{CStr, CString, OsStr, OsString};
use std::os::raw::c_char;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::prelude::OsStringExt;
use std::panic;
use std::str::FromStr;

use self::ffi::{closedir, opendir, readdir};

#[derive(Debug)]
struct DirectoryIterator {
    path: CString,
    dir: *mut ffi::DIR,
}

impl DirectoryIterator {
    fn new(path: &str) -> Result<DirectoryIterator, String> {
        // Call opendir and return a Ok value if that worked,
        // otherwise return Err with a message.
        match panic::catch_unwind(|| unsafe {
            let path = CString::new(path.as_bytes()).unwrap();
            DirectoryIterator {
                path: path.clone(),
                dir: opendir(path.as_ptr()),
            }
        }) {
            Ok(e) => Ok(e),
            Err(e) => Err("opendir() failed.".to_string()),
        }
    }
}

impl Iterator for DirectoryIterator {
    type Item = OsString;
    fn next(&mut self) -> Option<OsString> {
        // Keep calling readdir until we get a NULL pointer back.
        let e = unsafe { readdir(self.dir) };
        if e.is_null() {
            return None;
        }
        let v = unsafe {
            let i8s = (*e).d_name.as_slice();
            std::mem::transmute::<&[i8], &[u8]>(i8s).to_vec()
        };

        let oss = OsString::from_vec(v);
        Some(oss)
    }
}

impl Drop for DirectoryIterator {
    fn drop(&mut self) {
        // Call closedir as needed.
        unsafe {
            closedir(self.dir);
        }
    }
}

fn main() -> Result<(), String> {
    let iter = DirectoryIterator::new(".")?;
    println!("files: {:#?}", iter.collect::<Vec<_>>());
    Ok(())
}

#[test]
fn test() {
    assert!(main().is_ok());
}
