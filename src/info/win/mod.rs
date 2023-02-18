use crate::info::*;
use lazy_static::lazy_static;
use std::{env, sync::Mutex};
use windows::Win32::System::SystemInformation::MEMORYSTATUS;

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
    Ok(())
}

/// Has the `init()` run?
pub fn initialized() -> bool { *INITIALIZED.lock().unwrap() }

pub fn os_info() -> Result<(String, crate::printing::OsArt), InfoError>
{
    Ok(("String".to_string(), crate::printing::OsArt::Windows))
}

pub fn motherboard_info() -> Result<String, InfoError>
{
    Ok(String::new())
}

pub fn hostname_info() -> Result<String, InfoError>
{
    Ok(String::new())
}

pub fn caller_info() -> Result<Caller, InfoError>
{
    Ok(Caller {
        name: String::new(),
        shell: String::new(),
    })
}
