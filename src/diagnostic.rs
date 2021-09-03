use std::ffi::OsStr;
use winapi::um::debugapi;

pub fn output_debug_string(message: impl AsRef<OsStr>) {
    match widestring::WideCString::from_os_str(message) {
        Err(_) => return,
        Ok(message_string) => {
            unsafe { debugapi::OutputDebugStringW(message_string.as_ptr()); }
        }
    }
}