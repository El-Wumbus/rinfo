use crate::info::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use windows::Win32::Networking::WinSock::{
    self, WSAGetLastError, WSAEFAULT, WSAEINPROGRESS, WSANOTINITIALISED,
};

pub mod cpu;
pub use cpu::*;

pub mod memory;
pub use memory::*;

lazy_static! {
    /// Has the initialization function ran?
    pub static ref INITIALIZED: Mutex<bool> = Mutex::new(false);
}

pub fn init() -> Result<(), InfoError>
{
    // Setup WSA
    let wsa_version = 2 & 0xFF | (2 & 0xFF) << 8;
    dbg!(wsa_version);
    let mut wsa_data = WinSock::WSADATA::default();
    unsafe { WinSock::WSAStartup(wsa_version, &mut wsa_data) };
    Ok(())
}

/// Has the `init()` run?
pub fn initialized() -> bool { *INITIALIZED.lock().unwrap() }

pub fn os_info() -> Result<(String, crate::printing::OsArt), InfoError>
{
    Ok(("String".to_string(), crate::printing::OsArt::Windows))
}

pub fn motherboard_info() -> Result<String, InfoError> { Ok(String::new()) }

/// Get the computer's hostname
pub fn hostname_info() -> Result<String, InfoError>
{
    let mut bytes = [0x0; 1024];
    let r = unsafe { WinSock::gethostname(&mut bytes) };
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


    let s = match std::str::from_utf8(&bytes)
    {
        Ok(s) => s,
        Err(e) => return Err(InfoError::General(format!("Invalid UTF-8 sequence: {e}"))),
    };

    Ok(s.trim().to_string())
}

pub fn caller_info() -> Result<Caller, InfoError>
{
    Ok(Caller {
        name: String::new(),
        shell: String::new(),
    })
}
