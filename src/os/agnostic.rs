#![allow(unused_unsafe)]

use core::ffi::c_void;
use std::ptr;

pub use inline::*;

#[cfg(unix)]
mod inline {
    use super::*;

    pub fn allocate_executable_buffer(size: usize) -> *mut c_void {
        unimplemented!()
    }

    pub unsafe fn deallocate_buffer(data: *mut c_void, _size: usize) -> Result<(), ()> {
        unimplemented!()
    }
}

#[cfg(windows)]
mod inline {
    use jit_rs_sys::bindings::{
        Windows::Win32::System::Diagnostics::Debug::FlushInstructionCache,
        Windows::Win32::System::Memory::MEM_COMMIT,
        Windows::Win32::System::Memory::MEM_RELEASE,
        Windows::Win32::System::Memory::MEM_RESERVE,
        Windows::Win32::System::Memory::PAGE_EXECUTE_READWRITE,
        Windows::Win32::System::Memory::VirtualAlloc,
        Windows::Win32::System::Memory::VirtualFree,
        Windows::Win32::System::Threading::GetCurrentProcess,
    };

    use super::*;

    pub fn allocate_executable_buffer(size: usize) -> *mut c_void {
        let buffer = unsafe { VirtualAlloc(ptr::null_mut(), size, MEM_COMMIT | MEM_RESERVE, PAGE_EXECUTE_READWRITE) };
        buffer
    }

    pub unsafe fn deallocate_buffer(data: *mut c_void, _size: usize) -> Result<(), ()> {
        let ok = unsafe { VirtualFree(data, 0, MEM_RELEASE) };

        // Convert bool to Result<(), ()>
        ok.as_bool().then(|| ()).ok_or(())
    }

    pub unsafe fn flush_instruction_cache(data: *mut c_void, size: usize) -> Result<(), ()> {
        let ok = unsafe { FlushInstructionCache(GetCurrentProcess(), data, size) };

        // Convert bool to Result<(), ()>
        ok.as_bool().then(|| ()).ok_or(())
    }
}
