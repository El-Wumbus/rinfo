use std::sync::Mutex;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use super::*;

/// Get cpu information on linux platforms using procfs
mod cpu;
pub use cpu::*;

mod memory;
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