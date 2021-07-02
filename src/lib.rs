use std::mem::size_of;

mod os;

/// executes arbitrary machine code
pub unsafe fn execute_bytes<R>(bytes: &[u8]) -> R {
    let bytes_n = bytes.len() * size_of::<u8>();

    let mut buffer = os::buffer::Buffer::new(os::agnostic::allocate_executable_buffer(bytes_n) as *mut u8, bytes_n);

    buffer.as_slice_mut().copy_from_slice(bytes);

    let (data, _, _) = buffer.get_raw_parts();
    let fnptr: extern "C" fn() -> R = std::mem::transmute(data as *const ());

    fnptr()
}