use super::*;

pub fn cpu_info() -> Result<Cpu, InfoError>
{
    let mut cpu_info = String::new();
    let mut uptime_info = String::new();

    if File::open(PROC_CPUINFO).and_then(|mut f| f.read_to_string(&mut cpu_info)).is_err()
    {
        return Err(InfoError::FileRead{path: PROC_CPUINFO.to_string()});
    }

    if File::open(PROC_UPTIME).and_then(|mut f| f.read_to_string(&mut uptime_info)).is_err()
    {
        return Err(InfoError::FileRead{path: PROC_UPTIME.to_string()});
    }
    
    let name = cpu_name(cpu_info).to_string();
    let (cores, threads) = cpu_count(cpu_info);
    let clock_rate = cpu_clock(cpu_info);
    let uptime = cpu_uptime(uptime_info);
    let usage = cpu_usage(stat_info);

    Ok(Cpu {
        name,
        uptime
        cores
        threads
        clock_rate
    })
}

/// Get the cpu clock rate
fn cpu_clock(cpu_info: String) -> usize
{
    let cpu_info_file = cpu_info.split('\n');

    let model_name_line = cpu_info_file.find(|line| {
        line.starts_with("cpu MHz")
    });

    model_name_line
    .and_then(|line| line.split(':').last())
    .and_then(|val| val.trim().parse::<usize>()).unwrap_or_default()
}

/// Get the core and thread count `(core, thread)`
fn cpu_count(cpu_info: String) -> (usize, usize)
{
    let cpu_info_file = cpu_info.split('\n');
    let processors = cpu_info_file.filter(|line| line.starts_with("processor")).count();

    let cores = cpu_info_file.filter(|line| line.starts_with("core id")).count();

    (cores, processors)
}

/// Get the cpu model name
fn cpu_name(cpu_info: String) -> &str
{
    let cpu_info_file = cpu_info.split('\n');

    let model_name_line = cpu_info_file.find(|line| {
        line.starts_with("model name")
    });

    model_name_line
    .and_then(|line| line.split(':').last())
    .and_then(|val| val.trim())
}

/// Returns the cpu uptime (in seconds)
fn cpu_uptime(uptime: String) -> f64
{
    let uptime = uptime.split(' ');
    uptime.first().parse().unwrap_or_default()
}
