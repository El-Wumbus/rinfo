use std::mem::size_of;

use crate::info::*;
use windows::Win32::System::SystemInformation::*;

pub fn memory_info() -> Result<Memory, InfoError>
{
    let mut memory_info = MEMORYSTATUSEX::default();

    unsafe {
        memory_info.dwLength = size_of::<MEMORYSTATUSEX>() as u32;
        if !GlobalMemoryStatusEx(&mut memory_info).as_bool()
        {
            return Err(InfoError::General(String::from("GlobalMemoryStatusEx failed.")));
        }
    }
    let (total, available) = (memory_info.ullTotalPhys, memory_info.ullAvailPhys);
    Ok(Memory
    {
        total,
        available,
        used: total - available,
    })
}