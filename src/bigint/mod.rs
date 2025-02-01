#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::useless_transmute)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::alloc::{alloc, Layout};
use std::os::raw::c_void;
use std::os::raw::{c_char, c_uchar, c_uint};
use std::ptr;
use std::ptr::NonNull;

#[repr(C)]
pub struct ObjRef {
    size: c_uint,
    data: [c_uchar; 1], // Placeholder for the flexible array
}

impl ObjRef {
    pub fn new(data_size: usize) -> Result<NonNull<ObjRef>, &'static str> {
        let layout = Layout::from_size_align(
            std::mem::size_of::<ObjRef>() + data_size - 1,
            std::mem::align_of::<ObjRef>(),
        )
        .map_err(|_| "Invalid memory layout")?;

        let ptr = unsafe { alloc(layout) as *mut ObjRef };

        if ptr.is_null() {
            return Err("newPrimObject() got no memory");
        }

        unsafe {
            (*ptr).size = data_size as c_uint;
        }

        Ok(unsafe { NonNull::new_unchecked(ptr) })
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }

    pub fn data_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    pub fn data_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
}

#[no_mangle]
pub extern "C" fn fatalError(msg: *const c_char) {
    let c_str = unsafe { std::ffi::CString::from_raw(msg as *mut c_char) };
    let message = c_str.to_string_lossy();
    eprintln!("Fatal Error: {message}");
    std::process::exit(1);
}

#[no_mangle]
pub extern "C" fn newPrimObject(data_size: c_uint) -> *mut c_void {
    match ObjRef::new(data_size as usize) {
        Ok(obj_ref) => obj_ref.as_ptr() as *mut c_void,
        Err(err) => {
            eprintln!("{err}");
            ptr::null_mut()
        }
    }
}

/// `getPrimObjectDataPointer` function in Rust
/// # Safety
/// The caller must ensure:
/// - `obj` is a valid, non-null pointer to a properly initialized `ObjRef`.
/// - The `ObjRef` must not be accessed elsewhere during the lifetime of this call.
#[no_mangle]
pub unsafe extern "C" fn getPrimObjectDataPointer(
    obj: *mut ObjRef,
) -> *mut c_uchar {
    if obj.is_null() {
        return ptr::null_mut();
    }

    let obj_ref = unsafe { &mut *obj };
    obj_ref.data_mut_ptr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bigint_lib() {
        unsafe {
            bigFromInt(2);
            println!("{bip:#?}");
        }
    }
}
