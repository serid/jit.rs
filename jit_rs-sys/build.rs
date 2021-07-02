fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    windows::build! {
        // Windows::Win32::System::Diagnostics::ToolHelp::CreateToolhelp32Snapshot,
        // Windows::Win32::Foundation::HANDLE,
        // Windows::Win32::Foundation::HINSTANCE,
        // Windows::Win32::System::ProcessStatus::K32EnumProcesses,
        // Windows::Win32::System::ProcessStatus::K32GetModuleBaseNameW,
    }
    ;
}