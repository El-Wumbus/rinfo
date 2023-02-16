use crate::info::*;
use std::{path::PathBuf, sync::Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

const PROC_CPUINFO: &str = "/proc/cpuinfo";
const PROC_UPTIME: &str = "/proc/uptime";
const PROC_MEMINFO: &str = "/proc/meminfo";

/// Get cpu information on linux platforms using procfs
mod cpu;
pub use cpu::*;

/// Perform any initalization and pre-checks required
pub fn init() -> Result<(), InfoError>
{
    let cpu_info = PathBuf::from(PROC_CPUINFO);
    let uptime_info = PathBuf::from(PROC_UPTIME);

    // Ensure the files that we need exist
    if !cpu_info.is_file()
    {
        return Err(InfoError::MissingFile{path: cpu_info});
    }

    if !uptime_info.is_file()
    {
        return Err(InfoError::MissingFile{path: uptime_info});   
    }

    let mut state = INITIALIZED.lock().unwrap();
    *state = true;

    Ok(())
}

pub memory_info() -> Result<Memory, InfoError>
{
    let mut meminfo_file = String::new();

    if File::open(PROC_MEMINFO).and_then(|mut f| f.read_to_string(&mut meminfo_file)).is_err()
    {
        return Err(InfoError::FileRead{path: PROC_MEMINFO.to_string()});
    }

    let meminfo = meminfo_file.split('\n');

    let meminfo_total = meminfo.find(|line| {
        line.starts_with("MemTotal")
    });

    let total =
    meminfo_total
        .and_then(|line| line.split(':').last())
        .and_then(|val| val.trim().parse::<f64>() / 1049)
        .unwrap_or_default();

    let meminfo_available = meminfo.find(|line| {
        line.starts_with("MemAvailable")
    });

    let available =
    meminfo_available
        .and_then(|line| line.split(':').last().trim().split(' ').next())
        .and_then(|val| val.trim().parse::<f64>() / 1049)
        .unwrap_or_default();

    let used = total - available;

    Ok(Memory {
        total,
        available,
        used,
    })
}