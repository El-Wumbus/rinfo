use regex::Regex;
use std::rc::Rc;

use super::*;

pub fn cpu_info() -> Result<Cpu, InfoError>
{
    let mut cpu_info = String::new();
    let mut uptime_info = String::new();

    if File::open(PROC_CPUINFO)
        .and_then(|mut f| f.read_to_string(&mut cpu_info))
        .is_err()
    {
        return Err(InfoError::FileRead {
            path: PROC_CPUINFO.to_string(),
        });
    }

    if File::open(PROC_UPTIME)
        .and_then(|mut f| f.read_to_string(&mut uptime_info))
        .is_err()
    {
        return Err(InfoError::FileRead {
            path: PROC_UPTIME.to_string(),
        });
    }

    let cpu_info = Rc::new(cpu_info);
    let uptime_info = Rc::new(uptime_info);

    let name = cpu_name(Rc::clone(&cpu_info))?;
    let (cores, threads) = cpu_count(Rc::clone(&cpu_info));
    let clock_rate = cpu_clock(Rc::clone(&cpu_info))?;
    let uptime = cpu_uptime(Rc::clone(&uptime_info)) as u128;

    Ok(Cpu {
        name,
        uptime,
        cores,
        threads,
        clock_rate,
    })
}


fn cpu_clock(cpu_info: Rc<String>) -> Result<f64, InfoError>
{
    let re = Regex::new(r"^cpu MHz\s*:\s*(\d+(\.\d+)?)$").unwrap();
    let cpu_clock_line = cpu_info
        .lines()
        .find(|line| re.is_match(line))
        .ok_or_else(|| {
            InfoError::General("Couldn't find CPU clock speed in CPU info".to_owned())
        })?;
    let clock_speed_str = cpu_clock_line
        .split(':')
        .nth(1)
        .ok_or_else(|| {
            InfoError::General("Couldn't find CPU clock speed value in CPU info".to_owned())
        })?
        .trim();
    clock_speed_str
        .parse()
        .map_err(|e| InfoError::General(format!("Couldn't parse CPU clock speed: {e}")))
}

/// Get the core and thread count `(core, thread)`
fn cpu_count(cpu_info: Rc<String>) -> (usize, usize)
{
    let cpu_info_file = cpu_info.split('\n');
    let processors = cpu_info_file
        .clone()
        .filter(|line| line.starts_with("processor"))
        .count();

    let cores_unfiltered = cpu_info_file.filter(|line| line.starts_with("core id"));

    let mut cores = 0;
    let mut tmp: Vec<&str> = Vec::new();

    for core in cores_unfiltered
    {
        if !tmp.contains(&core)
        {
            tmp.push(core);
            cores += 1;
        }
    }

    (cores, processors)
}

/// Get the cpu model name
fn cpu_name(cpu_info: Rc<String>) -> Result<String, InfoError>
{
    let model_name_line = cpu_info
        .lines()
        .find(|line| line.starts_with("model name"))
        .ok_or_else(|| {
            InfoError::General("Couldn't find line that starts with 'model name'".to_string())
        })?;

    let model_name = model_name_line
        .split(':')
        .nth(1)
        .ok_or_else(|| InfoError::General("Invalid model name format".to_string()))?
        .trim();

    Ok(model_name.to_string())
}

/// Returns the cpu uptime (in seconds)
fn cpu_uptime(uptime: Rc<String>) -> u64
{
    let mut uptime = uptime.split(' ');
    let uptime = uptime
        .next()
        .unwrap_or_default()
        .parse::<f64>()
        .unwrap_or_default();

    (uptime * 1000.0) as u64
}
