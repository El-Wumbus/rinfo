use crate::info::*;

use windows::Win32::System::SystemInformation::*;

pub fn cpu_info() -> Result<Cpu, InfoError>
{
    let mut system_info = SYSTEM_INFO::default();
    unsafe {
        GetSystemInfo(&mut system_info);
    }
    let (_cores, _threads) = core_thread_count(system_info)?;
    // todo!()
    Ok(Cpu::default())
}

fn core_thread_count(system_info: SYSTEM_INFO) -> Result<(usize, usize), InfoError>
{
    let threads = system_info.dwNumberOfProcessors as usize;

    // TODO: Figure out the mess that is the windows api and get the number of
    // physical cores.

    Ok((0, threads))
}
