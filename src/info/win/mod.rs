use crate::info::*;
use lazy_static::lazy_static;
use std::{collections::HashMap, mem::size_of, path::PathBuf, sync::Mutex};
use winreg::enums::*;
use winreg::RegKey;

use windows::Win32::{
    Foundation::{CloseHandle, GetLastError, ERROR_NO_MORE_FILES},
    Networking::WinSock::{self, WSAGetLastError, WSAEFAULT, WSAEINPROGRESS, WSANOTINITIALISED},
    System::{
        Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Module32First, Process32First, Process32Next, MODULEENTRY32,
            PROCESSENTRY32, TH32CS_SNAPMODULE, TH32CS_SNAPPROCESS,
        },
        Threading::GetCurrentProcessId,
        WindowsProgramming::GetUserNameA,
    },
};


pub mod cpu;
pub use cpu::*;

pub mod memory;
pub use memory::*;

pub mod caller;
pub use caller::*;

lazy_static! {
    /// Has the initialization function ran?
    pub static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

pub fn init() -> Result<(), InfoError>
{
    const WSA_VERSION_2_2: u16 = 2 & 0xFF | (2 & 0xFF) << 8;

    // Setup WSA
    let mut wsa_data = WinSock::WSADATA::default();
    unsafe { WinSock::WSAStartup(WSA_VERSION_2_2, &mut wsa_data) };
    Ok(())
}

/// Has the `init()` run?
pub fn initialized() -> bool { *INITIALIZED.lock().unwrap() }

pub fn os_info() -> Result<(String, crate::printing::OsArt), InfoError>
{
    let version_map = HashMap::from([
        ("10.00", "10/11"),
        ("6.30", "8.1"),
        ("6.20", "8"),
        ("6.10", "7"),
        ("6.00", "Vista"),
        ("5.20", "Server 2003"),
        ("5.10", "XP"),
    ]);

    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm
        .open_subkey("SOFTWARE\\Microsoft\\Windows NT\\CurrentVersion")
        .unwrap();
    let major_version_num: u32 = cur_ver.get_value("CurrentMajorVersionNumber").unwrap();
    let minor_version_num: u32 = cur_ver.get_value("CurrentMinorVersionNumber").unwrap();
    let info = cur_ver.query_info().unwrap();
    let version_string = format!("{major_version_num}.{minor_version_num:02}");
    let os_str = if version_map.contains_key(&*version_string)
    {
        format!("Windows {}", version_map.get(&*version_string).unwrap())
    }
    else
    {
        "Windows".to_string()
    };
    let os_art = match &*os_str
    {
        "Windows 10/11" => crate::printing::OsArt::Windows1011, 
        _ => crate::printing::OsArt::Windows,
    };
    Ok((os_str, os_art))
}

pub fn motherboard_info() -> Result<String, InfoError> { Ok(String::new()) }

/// Get the computer's hostname
pub fn hostname_info() -> Result<String, InfoError>
{
    // Create a character buffer
    let mut buffer = [0x0; 1024];
    let r = unsafe { WinSock::gethostname(&mut buffer) };
    if r != 0
    {
        let err = unsafe { WSAGetLastError() };

        let m = match err
        {
            WSAEFAULT => "gethostname failed: WSAEFAULT",
            WSANOTINITIALISED => "gethostname failed: WSASTARTUP NOT RAN",
            WSAEINPROGRESS => "gethostname failed: IN PROGRESS",
            _ => "gethostname failed",
        };

        return Err(InfoError::General(m.to_string()));
    }


    let s = match std::str::from_utf8(&buffer)
    {
        Ok(s) => s,
        Err(e) => return Err(InfoError::General(format!("Invalid UTF-8 sequence: {e}"))),
    };

    Ok(s.trim().to_string())
}
