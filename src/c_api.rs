use serde::Serialize;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

#[derive(Serialize)]
pub struct LibLaikaEntry {
    pub message: String,
}

#[no_mangle]
pub extern "C" fn liblaika_hello() {
    crate::liblaika::liblaika_hello();
}

#[no_mangle]
pub extern "C" fn liblaika_initialize_logger(
    dir_path: *const c_char,
    ecc_len: usize,
) -> *mut crate::logger::Logger {
    // Convert C string to Rust string
    let dir_path_rs = unsafe {
        if dir_path.is_null() {
            return ptr::null_mut();
        }
        CStr::from_ptr(dir_path).to_string_lossy().into_owned()
    };

    // Attempt to initialize the logger
    match crate::logger::Logger::initialize_logger(&dir_path_rs, ecc_len) {
        Ok(logger) => Box::into_raw(Box::new(logger)), // Return a pointer to the logger
        Err(_) => ptr::null_mut(),                     // Return null pointer on error
    }
}

#[no_mangle]
pub extern "C" fn liblaika_log(logger: *mut crate::logger::Logger, message: *const c_char) -> bool {
    // Convert C string to Rust string
    let message_rs = unsafe {
        if message.is_null() {
            return false;
        }
        CStr::from_ptr(message).to_string_lossy().into_owned()
    };

    // Check for null logger
    if logger.is_null() {
        return false;
    }

    // Log the message
    let logger = unsafe { &mut *logger };

    // Create a log entry to serialize
    let entry = LibLaikaEntry {
        message: message_rs,
    };

    match logger.log(&entry) {
        Ok(_) => true,
        Err(_) => false,
    }
}

#[no_mangle]
pub extern "C" fn liblaika_destroy_logger(logger: *mut crate::logger::Logger) {
    if !logger.is_null() {
        // Convert raw pointer back to Box and let it drop
        unsafe { drop(Box::from_raw(logger)) };
    }
}
