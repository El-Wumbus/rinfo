use crate::info::*;
use std::mem::size_of;
use windows::Win32::{
    Foundation::{GetLastError, ERROR_INSUFFICIENT_BUFFER},
    System::SystemInformation::*,
};

pub fn cpu_info() -> Result<Cpu, InfoError>
{
    let mut system_info = SYSTEM_INFO::default();
    unsafe {
        GetSystemInfo(&mut system_info);
    }
    let (cores, threads) = core_thread_count(system_info)?;
    // todo!()
    Ok(Cpu::default())
}

fn core_thread_count(system_info: SYSTEM_INFO) -> Result<(usize, usize), InfoError>
{
    let threads = system_info.dwNumberOfProcessors as usize;

    // TODO: Figure out the mess that is the windows api and get the number of
    // physical cores.

    Ok((threads, threads))
}
