#[cfg(target_os = "ios")]
use base58_monero::{decode, encode};

#[cfg(target_os = "ios")]
use crate::buffer::Buffer;

#[cfg(target_os = "ios")]
use crate::strings;

#[cfg(target_os = "ios")]
use std::os::raw::c_char;

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_base58_utils_encode(
    input_buffer: Buffer,
    encoded_output: *mut *mut c_char,
) -> i32 {
    let data = input_buffer.to_bytes();
    match encode(&data) {
        Ok(text) => {
            let result = strings::string_to_c_char(text);
            *encoded_output = result;
            return 0;
        }
        Err(_) => {
            return 1;
        }
    }
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_base58_utils_decode(encoded_input: *const c_char) -> Buffer {
    let text = strings::c_char_to_string(encoded_input);
    match decode(&text) {
        Ok(bytes) => {
            let slice = bytes.into_boxed_slice();
            let data = slice.as_ptr();
            let len = slice.len();
            std::mem::forget(slice);
            Buffer { data, len }
        }
        Err(_) => Buffer {
            data: std::ptr::null_mut(),
            len: 0,
        },
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_encode_hello_world() {
        let input = b"Hello, World!";
        let encoded = base58_monero::encode(input).unwrap();
        assert_eq!(encoded, "D7LMXYjYZ7cDaGe8bS");
    }
}
