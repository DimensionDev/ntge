#[repr(C)]
#[derive(Debug)]
pub struct Buffer {
    pub data: *mut u8, // memory should handled by pointer holder
    pub len: usize,
}

impl Buffer {
    pub unsafe fn to_bytes(&self) -> Vec<u8> {
        std::slice::from_raw_parts(self.data, self.len).to_vec()
    }
}

// impl Drop for Buffer {
//     fn drop(&mut self) {
//         println!("{:?} is being deallocated", self);
//     }
// }
