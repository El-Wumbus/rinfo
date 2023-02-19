use std::rc::Rc;

use super::*;
use windows::Win32::System::SystemInformation::*;
use wmi::{COMLibrary, Variant, WMIConnection};

pub fn cpu_info() -> Result<Cpu, InfoError>
{
    let com_con = COMLibrary::new().unwrap();
    let wmi_con = Rc::new(WMIConnection::new(com_con.into()).unwrap());

    let mut system_info = SYSTEM_INFO::default();

    unsafe {
        GetSystemInfo(&mut system_info);
    }

    let uptime = unsafe { GetTickCount64() } as usize;
    let (cores, threads) = core_thread_count(system_info, Rc::clone(&wmi_con))?;
    let (name, clock_rate) = cpu_name_clock(Rc::clone(&wmi_con))?;

    Ok(Cpu {
        name,
        uptime,
        cores,
        threads,
        clock_rate,
    })
}

fn cpu_name_clock(wmi_con: Rc<WMIConnection>) -> Result<(String, f64), InfoError>
{
    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query("SELECT Name,CurrentClockSpeed FROM Win32_Processor")
        .unwrap();
    let mut name = "UNAVAILABLE";
    let mut clock = 0.0;

    if !results.is_empty()
    {
        match results.first().unwrap().get("Name")
        {
            None =>
            {
                return Err(InfoError::General(
                    "WMI: 'Win32_Processor' failed: 'Name' not found".to_string(),
                ))
            }
            Some(x) =>
            {
                match x
                {
                    Variant::String(x) => name = &x.trim(),
                    _ => (),
                }
            }
        };
        match results.first().unwrap().get("CurrentClockSpeed")
        {
            None =>
            {
                return Err(InfoError::General(
                    "WMI: 'Win32_Processor' failed: 'CurrentClockSpeed' not found".to_string(),
                ))
            }
            Some(x) =>
            {
                match x
                {
                    Variant::UI4(x) => clock = f64::from(*x),
                    _ => (),
                }
            }
        };
    }
    else
    {
        return Err(InfoError::General(
            "WMI: 'Name, CurrentClockSpeed from Win32_Processor' failed: empty result".to_string(),
        ));
    }

    Ok((name.to_string(), clock))
}

fn core_thread_count(
    system_info: SYSTEM_INFO,
    wmi_con: Rc<WMIConnection>,
) -> Result<(usize, usize), InfoError>
{
    let threads = system_info.dwNumberOfProcessors as usize;
    let mut cores = threads;

    let results: Vec<HashMap<String, Variant>> = wmi_con
        .raw_query("SELECT NumberOfCores FROM Win32_Processor")
        .unwrap();

    if !results.is_empty()
    {
        cores = match results.first().unwrap().get("NumberOfCores")
        {
            None =>
            {
                return Err(InfoError::General(
                    "WMI: 'NumberOfCores from Win32_Processor' failed: 'NumberOfCores' not found"
                        .to_string(),
                ))
            }
            Some(x) =>
            {
                match x
                {
                    Variant::UI4(x) => *x as usize,
                    _ => 0,
                }
            }
        };
    }
    else
    {
        return Err(InfoError::General(
            "WMI: 'NumberOfCores from Win32_Processor' failed: empty result".to_string(),
        ));
    }

    Ok((cores, threads))
}
