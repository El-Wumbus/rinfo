use std::{ffi::c_int, mem::size_of, ptr::null};

use libc::{c_void, difftime, sysctl, time, time_t, timeval, CTL_KERN, KERN_BOOTTIME};

use super::*;

pub fn cpu_info() -> Result<Cpu, InfoError> {
    let uptime = uptime()?;
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

fn uptime() -> Result<u128, InfoError> {
    // Initialize the structure
    let mut boottime: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    // Get hw.boottime
    let mut mib: [c_int; 2] = [CTL_KERN, KERN_BOOTTIME];
    if unsafe {
        sysctl(
            &mut mib as *mut c_int,
            2,
            &mut boottime as *mut timeval as *mut c_void,
            &mut size_of::<timeval>(),
            null::<usize>() as *mut c_void,
            0,
        )
    } < 0
    {
        return Err(InfoError::Sysctl {
            name: "hw.boottime".to_string(),
        });
    }
    
    // // Get the time elapsed (assuming the current time hasn't been changed)
    let bsec: time_t = boottime.tv_sec;
    let csec: time_t = unsafe { time(null::<i64>() as *mut i64) };
    let uptime = (unsafe { difftime(csec, bsec) } * 1000.0) as u128;

    Ok(uptime)
}

fn cpu_frequency() -> Result<f64, InfoError> {
    let mut frequency: u64 = 0;
    if unsafe { macos_cpu_frequency(&mut frequency as *mut u64) } < 0 {
        return Err(InfoError::General("".to_string()));
    }

    Ok(frequency as f64)
}

fn cpu_name() -> Result<String, InfoError> {
    let mut buffer = [0x0; 2048];

    if unsafe { macos_cpu_name(buffer.as_mut_ptr(), buffer.len()) } < 0 {
        return Err(InfoError::General("Unable to get CPU name".to_string()));
    }

    let buffer = buffer.map(|char| char as u8);

    let s = std::str::from_utf8(&buffer).expect("INVALID UTF-8");

    Ok(s.trim().to_string())
}

fn cpu_count() -> Result<(usize, usize), InfoError> {
    let mut buffer = MacOsCpuCount {
        core_count: 0,
        thread_count: 0,
    };

    if unsafe { macos_cpu_count(&mut buffer) } < 0 {
        return Err(InfoError::General("Unable to get CPU name".to_string()));
    }

    Ok((buffer.core_count, buffer.thread_count))
}
