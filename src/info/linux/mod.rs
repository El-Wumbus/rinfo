use super::common;
use crate::info::*;
use lazy_static::lazy_static;
use libc::{connect, getsockname, in_addr, sockaddr, sockaddr_in, socket, AF_INET, SOCK_DGRAM};
use std::{env, fs::File, io::Read, mem::size_of, path::PathBuf, sync::Mutex};

lazy_static! {
    /// Has the initialization function ran?
    pub static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

const PROC_CPUINFO: &str = "/proc/cpuinfo";
const PROC_UPTIME: &str = "/proc/uptime";
const PROC_MEMINFO: &str = "/proc/meminfo";
const PROC_HOSTNAME: &str = "/proc/sys/kernel/hostname";
const SYS_BOARD_VENDOR: &str = "/sys/devices/virtual/dmi/id/board_vendor";
const SYS_BOARD_NAME: &str = "/sys/devices/virtual/dmi/id/board_name";
const ETC_OS_RELEASE: &str = "/etc/os-release";
const ETC_LSB_RELEASE: &str = "/etc/lsb-release";

/// Get cpu information on linux platforms using procfs
mod cpu;
pub use cpu::*;

/// Get memory information on linux platforms using procfs
mod memory;
pub use memory::*;

/// Get information on the caller of the program
mod caller;
pub use caller::*;

/// Get information about the operating system
mod operating_system;
pub use operating_system::*;

/// Perform any initalization and pre-checks required
pub fn init() -> Result<(), InfoError>
{
    let cpu_info = PathBuf::from(PROC_CPUINFO);
    let uptime_info = PathBuf::from(PROC_UPTIME);
    let memory_info = PathBuf::from(PROC_MEMINFO);
    let (sys_board_name, sys_board_vendor) = (
        PathBuf::from(SYS_BOARD_NAME),
        PathBuf::from(SYS_BOARD_VENDOR),
    );
    let lsb_release_info = PathBuf::from(ETC_LSB_RELEASE);
    let os_release_info = PathBuf::from(ETC_OS_RELEASE);

    // Ensure the files that we need exist
    if !cpu_info.is_file()
    {
        return Err(InfoError::MissingFile { path: cpu_info });
    }

    if !uptime_info.is_file()
    {
        return Err(InfoError::MissingFile { path: uptime_info });
    }

    if !memory_info.is_file()
    {
        return Err(InfoError::MissingFile { path: memory_info });
    }

    if !sys_board_name.is_file()
    {
        return Err(InfoError::MissingFile {
            path: sys_board_name,
        });
    }

    if !sys_board_vendor.is_file()
    {
        return Err(InfoError::MissingFile {
            path: sys_board_vendor,
        });
    }

    if !lsb_release_info.is_file() && !os_release_info.is_file()
    {
        return Err(InfoError::MissingFile {
            path: lsb_release_info,
        });
    }

    *INITIALIZED.lock().unwrap() = true;

    Ok(())
}

/// Has the `init()` run?
pub fn initialized() -> bool { *INITIALIZED.lock().unwrap() }

fn uname_from_uid(uid: u32) -> Option<String>
{
    use std::ffi::CStr;
    use std::mem;
    use std::ptr;

    let mut result = ptr::null_mut();
    let amt = match unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) }
    {
        n if n < 0 => 512,
        n => n as usize,
    };
    let mut buf = Vec::with_capacity(amt);
    let mut passwd: libc::passwd = unsafe { mem::zeroed() };

    match unsafe {
        libc::getpwuid_r(
            uid,
            &mut passwd,
            buf.as_mut_ptr(),
            buf.capacity() as libc::size_t,
            &mut result,
        )
    }
    {
        0 if !result.is_null() =>
        {
            let ptr = passwd.pw_name as *const _;
            let username = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_owned();
            Some(username)
        }
        _ => None,
    }
}

pub fn hostname_info() -> Result<String, InfoError>
{
    let mut hostname = String::new();

    if File::open(PROC_HOSTNAME)
        .and_then(|mut f| f.read_to_string(&mut hostname))
        .is_err()
    {
        return Err(InfoError::FileRead {
            path: PROC_HOSTNAME.to_string(),
        });
    }

    Ok(hostname.trim().to_string())
}

pub fn motherboard_info() -> Result<String, InfoError>
{
    let mut vendor = String::new();

    if File::open(SYS_BOARD_VENDOR)
        .and_then(|mut f| f.read_to_string(&mut vendor))
        .is_err()
    {
        return Err(InfoError::FileRead {
            path: SYS_BOARD_VENDOR.to_string(),
        });
    }

    let mut name = String::new();

    if File::open(SYS_BOARD_NAME)
        .and_then(|mut f| f.read_to_string(&mut name))
        .is_err()
    {
        return Err(InfoError::FileRead {
            path: SYS_BOARD_NAME.to_string(),
        });
    }

    Ok(format!("{} ({})", name.trim(), vendor.trim()))
}


pub fn ip_info() -> Result<String, InfoError>
{
    const IP: &str = "1.1.1.1";
    const PORT: u16 = 53;

    let sock = unsafe { socket(AF_INET, SOCK_DGRAM, 0) };
    if sock == -1
    {
        return Err(InfoError::General("Unable to create socket".to_string()));
    }

    let serv = sockaddr_in {
        sin_family: AF_INET as u16,
        sin_port: PORT.to_be(),
        sin_addr: in_addr {
            s_addr: common::ipv4_to_int(IP),
        },
        sin_zero: [0; 8],
    };

    if unsafe {
        connect(
            sock,
            &serv as *const sockaddr_in as *const sockaddr,
            size_of::<sockaddr_in>() as u32,
        )
    } == -1
    {
        return Err(InfoError::General(format!("Unable to connect to '{IP}'")));
    }

    let mut name = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };

    if unsafe {
        getsockname(
            sock,
            &mut name as *mut sockaddr_in as *mut sockaddr,
            &mut (size_of::<sockaddr_in>() as u32),
        )
    } == -1
    {
        return Err(InfoError::General("Unable to get socket name".to_string()));
    }

    let s = format!("{} (IPV4)", common::int_to_ipv4(name.sin_addr.s_addr));

    Ok(s)
}
