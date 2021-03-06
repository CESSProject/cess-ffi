use std::ptr;

use drop_struct_macro_derive::DropStructMacro;
// `CodeAndMessage` is the trait implemented by `code_and_message_impl
use ffi_toolkit::{code_and_message_impl, free_c_str, CodeAndMessage, FCPResponseStatus};

#[repr(C)]
#[derive(DropStructMacro)]
pub struct cess_GpuDeviceResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *const libc::c_char,
    pub devices_len: libc::size_t,
    pub devices_ptr: *const *const libc::c_char,
}

impl Default for cess_GpuDeviceResponse {
    fn default() -> Self {
        Self {
            error_msg: ptr::null(),
            status_code: FCPResponseStatus::FCPNoError,
            devices_len: 0,
            devices_ptr: ptr::null(),
        }
    }
}

code_and_message_impl!(cess_GpuDeviceResponse);

#[no_mangle]
pub unsafe extern "C" fn cess_destroy_gpu_device_response(ptr: *mut cess_GpuDeviceResponse) {
    let _ = Box::from_raw(ptr);
}

#[repr(C)]
#[derive(DropStructMacro)]
pub struct cess_InitLogFdResponse {
    pub status_code: FCPResponseStatus,
    pub error_msg: *const libc::c_char,
}

impl Default for cess_InitLogFdResponse {
    fn default() -> Self {
        Self {
            error_msg: ptr::null(),
            status_code: FCPResponseStatus::FCPNoError,
        }
    }
}

code_and_message_impl!(cess_InitLogFdResponse);

#[no_mangle]
pub unsafe extern "C" fn cess_destroy_init_log_fd_response(ptr: *mut cess_InitLogFdResponse) {
    let _ = Box::from_raw(ptr);
}
