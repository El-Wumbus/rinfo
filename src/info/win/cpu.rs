use crate::info::*;
use std::mem::size_of;
use windows::Win32::System::SystemInformation::*;

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
    const FIND_CORES_BY_RELATIONSHIP: i32 = 0;
    let threads = system_info.dwNumberOfProcessors as usize;

    // let mut logical_processor_information = SYSTEM_LOGICAL_PROCESSOR_INFORMATION_EX::default();
    // let mut size = 0;
    // let find_cores = LOGICAL_PROCESSOR_RELATIONSHIP(FIND_CORES_BY_RELATIONSHIP);

    // if !unsafe {
    //     GetLogicalProcessorInformationEx(
    //         find_cores,
    //         Some(&mut logical_processor_information),
    //         &mut size,
    //     )
    // }
    // .as_bool()
    // {
    //     return Err(InfoError::General(String::from(
    //         "GetLogicalProcessorInformationEx failed.",
    //     )));
    // }

    // unsafe {dbg!(logical_processor_information.Anonymous.Processor)};

    Ok((threads, threads))
}
