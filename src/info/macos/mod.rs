use super::*;
use std::{
    ffi::{c_char, c_int},
    fs::File,
    io::Read,
    mem::size_of,
    ptr::null,
    sync::Mutex,
};


const OS_DISPLAY_NAME_FILE: &str = "/System/Library/CoreServices/Setup \
                                    Assistant.app/Contents/Resources/en.lproj/OSXSoftwareLicense.\
                                    rtf";
const MODEL_NAME_PLIST_FILE: &str = "/System/Library/PrivateFrameworks/ServerInformation.\
                                     framework/Versions/A/Resources/en.lproj/SIMachineAttributes.\
                                     plist";

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Get cpu information on linux platforms using procfs
mod cpu;
pub use cpu::*;

mod memory;
use libc::{
    c_void, connect, getsockname, in_addr, sockaddr, sockaddr_in, socket, sysctl, AF_INET, CTL_HW,
    CTL_KERN, HW_MODEL, KERN_HOSTNAME, SOCK_DGRAM,
};
pub use memory::*;

mod caller;
pub use caller::*;

use lazy_static::lazy_static;

lazy_static! {
    /// Has the initialization function ran?
    pub static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

pub fn init() -> Result<(), InfoError>
{
    *INITIALIZED.lock().unwrap() = true;
    Ok(())
}

/// Has the `init()` run?
pub fn initialized() -> bool { *INITIALIZED.lock().unwrap() }

pub fn os_info() -> Result<(String, crate::printing::OsArt), InfoError>
{
    let mut os_display_name_file = String::new();

    if File::open(OS_DISPLAY_NAME_FILE)
        .and_then(|mut f| f.read_to_string(&mut os_display_name_file))
        .is_err()
    {
        return Err(InfoError::FileRead {
            path: OS_DISPLAY_NAME_FILE.to_string(),
        });
    }

    let mut os_display_name_file = os_display_name_file.split('\n');
    let os_display_name_line = os_display_name_file
        .find(|line| line.contains("SOFTWARE LICENSE AGREEMENT FOR macOS"))
        .unwrap_or_default();
    let os_display_name = os_display_name_line
        .split(' ')
        .last()
        .unwrap_or("UNKNOWN")
        .replace('\\', "")
        .trim()
        .to_owned();

    Ok((
        format!("MacOS {}", os_display_name),
        crate::printing::OsArt::MacOS,
    ))
}

pub fn hostname_info() -> Result<String, InfoError>
{
    let mut buffer = [0x0 as c_char; 2048];
    let mut mib: [c_int; 2] = [CTL_KERN, KERN_HOSTNAME];

    // Get the hostname
    if unsafe {
        sysctl(
            &mut mib as *mut c_int,
            2,
            &mut buffer as *mut i8 as *mut c_void,
            &mut size_of::<[c_char; 2048]>(),
            null::<usize>() as *mut c_void,
            0,
        )
    } < 0
    {
        return Err(InfoError::Sysctl {
            name: "kern.hostname".to_string(),
        });
    }
    let buffer = buffer.map(|char| char as u8);
    Ok(std::str::from_utf8(&buffer).unwrap().trim().to_string())
}

pub fn motherboard_info() -> Result<String, InfoError>
{
    let mut buffer = [0x0 as c_char; 2048];
    let mut mib: [c_int; 2] = [CTL_HW, HW_MODEL];

    // Get the model name
    if unsafe {
        sysctl(
            &mut mib as *mut c_int,
            2,
            &mut buffer as *mut i8 as *mut c_void,
            &mut size_of::<[c_char; 2048]>(),
            null::<usize>() as *mut c_void,
            0,
        )
    } < 0
    {
        return Err(InfoError::Sysctl {
            name: "hw.model".to_string(),
        });
    }
    let buffer = buffer.map(|char| char as u8);
    let mut model = std::str::from_utf8(&buffer)
        .unwrap()
        .replace('\0', "")
        .trim()
        .to_string();

    if let Ok(file) = plist::Value::from_file(MODEL_NAME_PLIST_FILE)
    {
        let file = file.as_dictionary().unwrap();
        if file.contains_key(&model)
        {
            let file = file.get(&model).unwrap().as_dictionary().unwrap();
            let file = file.get("_LOCALIZABLE_").unwrap().as_dictionary().unwrap();
            let file = file.get("marketingModel").unwrap().as_string().unwrap();
            model = file.trim().to_string();
        }
    };

    Ok(model)
}

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
        sin_family: AF_INET as u8,
        sin_port: PORT.to_be(),
        sin_addr: in_addr {
            s_addr: common::ipv4_to_int(IP),
        },
        sin_zero: [0; 8],
        sin_len: 8,
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
        sin_len: 8,
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

    unsafe { close(sock) };

    let s = format!("{} (IPV4)", common::int_to_ipv4(name.sin_addr.s_addr));

    Ok(s)
}
