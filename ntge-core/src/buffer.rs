#[repr(C)]
#[derive(Debug)]
#[cfg(target_os = "ios")]
pub struct Buffer {
    pub data: *const u8, // memory should handled by pointer holder
    pub len: usize,
}

#[cfg(target_os = "ios")]
impl Buffer {
    pub unsafe fn to_bytes(&self) -> Vec<u8> {
        std::slice::from_raw_parts(self.data, self.len).to_vec()
    }
}

#[no_mangle]
#[cfg(target_os = "ios")]
pub unsafe extern "C" fn c_buffer_destroy(buffer: Buffer) {
    let slice = std::slice::from_raw_parts_mut(buffer.data as *mut u8, buffer.len);
    let _: Box<[u8]> = Box::from_raw(slice);
}
