#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::unreadable_literal)]
#![allow(clippy::cognitive_complexity)]
#![allow(clippy::useless_transmute)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[no_mangle]
pub extern "C" fn fatalError(msg: *const std::os::raw::c_char) {
    let c_str = unsafe {
        std::ffi::CString::from_raw(msg as *mut std::os::raw::c_char)
    };
    let message = c_str.to_string_lossy();
    eprintln!("Fatal Error: {message}");
    std::process::exit(1);
}

#[no_mangle]
pub extern "C" fn newPrimObject(dataSize: std::os::raw::c_uint) -> ObjRef {
    let layout = std::alloc::Layout::from_size_align(
        (dataSize as usize) + std::mem::size_of::<std::os::raw::c_uint>(),
        std::mem::align_of::<ObjRef>(),
    )
    .expect("Failed to create memory layout");

    let obj_ref_ptr: ObjRef = unsafe { std::alloc::alloc(layout) as ObjRef };

    if obj_ref_ptr.is_null() {
        fatalError(
            std::ffi::CString::new(
                "Error: failed to allocate memory for ObjRef",
            )
            .unwrap()
            .into_raw(),
        );
    }

    unsafe {
        (*obj_ref_ptr).size = dataSize;
    }

    obj_ref_ptr
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
