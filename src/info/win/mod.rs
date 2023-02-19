use crate::info::*;
use lazy_static::lazy_static;
use std::sync::Mutex;
use windows::Win32::{
    Foundation::GetLastError,
    Networking::WinSock::{self, WSAGetLastError, WSAEFAULT, WSAEINPROGRESS, WSANOTINITIALISED},
    System::WindowsProgramming::GetUserNameA,
};

const MAX_USERLEN: usize = 257;

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
    Ok(("String".to_string(), crate::printing::OsArt::Windows))
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

pub fn caller_info() -> Result<Caller, InfoError>
{
    Ok(Caller {
        name: caller_name()?,
        shell: String::new(),
    })
}

// Get the username of the caller
fn caller_name() -> Result<String, InfoError>
{
    // Create character buffer
    let mut buffer = [0x0; 1024];
    let mut written = buffer.len() as u32;

    let r = unsafe { GetUserNameA(windows::core::PSTR(buffer.as_mut_ptr()), &mut written) };
    if !r.as_bool()
    {
        let r = unsafe { GetLastError() }.0;
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
    }

    // Construct a string from the buffer
    let s = match std::str::from_utf8(&buffer)
    {
        Ok(s) => s,
        Err(e) => return Err(InfoError::General(format!("Invalid UTF-8 sequence: {e}"))),
    };

    Ok(s.to_string())
}
