use std::ffi::c_void;
use std::mem::size_of;

mod os;

/// executes arbitrary machine code
pub unsafe fn execute_bytes<R>(bytes: &[u8]) -> R {
    let bytes_n = bytes.len() * size_of::<u8>();

    let mut buffer = os::buffer::Buffer::new(os::agnostic::allocate_executable_buffer(bytes_n) as *mut u8, bytes_n);

    buffer.as_slice_mut().copy_from_slice(bytes);

    let (data, size, _) = buffer.get_raw_parts();
    let fnptr: extern "C" fn() -> R = std::mem::transmute(data);

    let _result = os::agnostic::flush_instruction_cache(data as *mut c_void, size);

    fnptr()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten() {
        // Machine code that returns int value 10
        let bytes = [
            0xb8, 0x0a, 0x00, 0x00, 0x00, // mov  eax,10
            0xc3 // ret
        ];

        // Execute code stored in `bytes`
        let r = unsafe { execute_bytes::<i32>(&bytes) };
        assert_eq!(r, 10);
    }

    #[test]
    fn two_plus_two_is_four() {
        let bytes = [
            0xb8, 0x02, 0x00, 0x00, 0x00, // mov eax,2
            0x01, 0xc0, // add eax,eax
            0xc3 // ret
        ];

        // Execute code stored in `bytes`
        let r = unsafe { execute_bytes::<i32>(&bytes) };
        assert_eq!(r, 4);
    }
}