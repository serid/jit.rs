use std::ffi::c_void;
use std::mem::size_of;
use std::slice;

/// Like `Box<[T]>` but for os buffers allocated by `VirtualAlloc` or `mmap`
pub struct Buffer<T> {
    data: *mut T,
    // size of allocated buffer
    size: usize,
    // number of elements in the buffer. `length * size_of::<T>() <= size`
    length: usize,
}

impl<T> Buffer<T> {
    pub unsafe fn new(data: *mut T, size: usize) -> Buffer<T> {
        Buffer {
            data,
            size,
            length: size / size_of::<T>(),
        }
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.data as *const T, self.length) }
    }

    pub fn as_slice_mut(&mut self) -> &mut [T] {
        unsafe { slice::from_raw_parts_mut(self.data, self.length) }
    }

    pub fn get_raw_parts(&self) -> (*mut T, usize, usize) {
        (self.data, self.size, self.length)
    }
}

impl<T> Drop for Buffer<T> {
    fn drop(&mut self) {
        let _result = unsafe { super::agnostic::deallocate_buffer(self.data as *mut c_void, self.size) };
    }
}