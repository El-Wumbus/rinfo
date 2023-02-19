use std::sync::Mutex;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use super::*;

/// Get cpu information on linux platforms using procfs
mod cpu;
pub use cpu::*;

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
    Ok(("NONE".to_string(), crate::printing::OsArt::Unknown))
}
pub fn hostname_info() -> Result<String, InfoError>
{
   Ok("NONE".to_string())
}

pub fn motherboard_info() -> Result<String, InfoError>
{
    Ok("NONE".to_string())
}

pub fn memory_info() -> Result<Memory, InfoError>
{
    Ok(Memory::default())
}

pub fn caller_info() -> Result<Caller, InfoError>
{
    Ok(Caller::default())
}