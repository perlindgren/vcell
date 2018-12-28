// Minimal allocator for &[u8]
//
use cstr_core::CStr;

const SIZE: usize = 0x1000;
static mut HEAP: [u8; SIZE] = [0; SIZE];
static mut CURR: usize = 0;

// copy a slice of bytes into static memory
pub fn static_bytes(b: &[u8]) -> &'static [u8] {
    unsafe {
        if CURR + (b.len() as usize) > SIZE {
            panic!("OOM");
        };

        let slice = &mut HEAP[CURR..CURR + b.len() as usize];
        slice.copy_from_slice(b);
        CURR += b.len();

        slice
    }
}

pub fn check(b: &[u8]) -> () {
    unsafe {
        let start: usize = HEAP.as_ptr() as usize;
        let end: usize = HEAP[CURR..].as_ptr() as usize;

        let b_start: usize = b.as_ptr() as usize;
        let b_end: usize = b_start + b.len();

        assert! { start <= b_start && b_end <= end };
    }
}
