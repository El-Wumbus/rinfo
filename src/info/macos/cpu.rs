
use super::*;

pub fn cpu_info() -> Result<Cpu, InfoError>
{
    let uptime = (uptime()? * 1000.0) as u64;
    let name = cpu_name()?;
    let clock_rate = cpu_frequency()?;
    let (cores, threads) = cpu_count()?;

    Ok(Cpu {
        uptime,
        name,
        clock_rate,
        cores,
        threads,
    })
}

fn uptime() -> Result<f64, InfoError>
{
    let uptime = unsafe {macos_uptime()};  
    if uptime == -1.0
    {
        return Err(InfoError::General("Unable to get uptime".to_string()));
    }
    Ok(uptime)
}

fn cpu_frequency() -> Result<f64, InfoError>
{
    let mut frequency:u64 = 0;
    if unsafe {macos_cpu_frequency(&mut frequency as *mut u64)} < 0
    {
        return Err(InfoError::General("".to_string()));
    }

    Ok(frequency as f64)
}

fn cpu_name() -> Result<String, InfoError>
{

    let mut buffer = [0x0; 2048];

    if unsafe {
        macos_cpu_name(buffer.as_mut_ptr(), buffer.len())
    } < 0
    {
        return Err(InfoError::General("Unable to get CPU name".to_string()));
    }

    let buffer = buffer.map(|char| char as u8);

    let s = std::str::from_utf8(&buffer).expect("INVALID UTF-8");

    Ok(s.trim().to_string())
}

fn cpu_count() -> Result<(usize, usize), InfoError>
{
    let mut buffer = MacOsCpuCount { core_count:0, thread_count: 0 };

    if unsafe {
        macos_cpu_count(&mut buffer)
    } < 0
    {
        return Err(InfoError::General("Unable to get CPU name".to_string()));
    }

    Ok((buffer.core_count, buffer.thread_count))
}