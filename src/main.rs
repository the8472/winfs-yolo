
use std::ffi::CString;
use std::{mem, ptr};
use std::ptr::{null, null_mut};

use windows_sys::core::*;
use windows_sys::Win32::Foundation::{GENERIC_READ, GENERIC_WRITE, INVALID_HANDLE_VALUE};
use windows_sys::Win32::System::IO::DeviceIoControl;
use windows_sys::Win32::Storage::FileSystem::{CreateFileA, FILE_SHARE_READ, FILE_SHARE_WRITE, OPEN_EXISTING};


const IOCTL_DISK_GET_CACHE_SETTINGS: u32 = 0x740e0;
const IOCTL_DISK_SET_CACHE_SETTINGS: u32 = 0x7c0e4;

#[repr(C)]
struct DiskCacheSettings {
    pub version: u32,
    pub state: u32,
    pub is_power_protected: u8,
}

fn main() {
    unsafe {
        // r#"\\?\Device\Harddisk0\DR0"#
        // PhysicalDisk0
        let hd1 = CString::new(r#"\\.\PHYSICALDRIVE0"#).unwrap();
        let handle = CreateFileA(
            hd1.as_ptr().cast(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            null(),
            OPEN_EXISTING,
            0,
            null_mut()
        );

        if handle == INVALID_HANDLE_VALUE {
            let error_code = windows_sys::Win32::Foundation::GetLastError();
            panic!("Failed to open device DR0, {:x}", error_code);
        }

        let mut settings: DiskCacheSettings = mem::zeroed();
        let mut lpbytesreturned: u32 = 0;

        let r = DeviceIoControl(handle, IOCTL_DISK_GET_CACHE_SETTINGS, null(), 0, ptr::from_mut(&mut settings).cast(), size_of::<DiskCacheSettings>() as u32, &mut lpbytesreturned, null_mut());
        if r == 0 {
            panic!("Failed to get cache settings");
        }

        // Enable YOLO mode
        settings.is_power_protected = 1;

        let r = DeviceIoControl(handle, IOCTL_DISK_SET_CACHE_SETTINGS, ptr::from_ref(&settings).cast(), size_of::<DiskCacheSettings>() as u32, null_mut(), 0, &mut lpbytesreturned, null_mut());
        if r == 0 {
            panic!("Failed to set cache settings");
        }
        
        // verify that it sticks

        let r = DeviceIoControl(handle, IOCTL_DISK_GET_CACHE_SETTINGS, null(), 0, ptr::from_mut(&mut settings).cast(), size_of::<DiskCacheSettings>() as u32, &mut lpbytesreturned, null_mut());

        if r == 0 {
            panic!("Failed to get cache settings after setting");
        }

        if settings.is_power_protected != 1 {
            panic!("Failed to enable YOLO mode, is_power_protected is not set");
        } else {
            println!("YOLO mode enabled successfully!");
        }
    }
}
