use std::{env, ffi::c_int, mem::size_of, os::raw::c_char, ptr::null};

use libc::{c_void, proc_name, sysctl, CTL_KERN, KERN_PROCARGS};

use super::*;

fn caller_user() -> Result<String, InfoError> {
    let user;

    if let Ok(name) = env::var("USER") {
        user = name;
    } else if let Ok(name) = env::var("USERNAME") {
        user = name;
    } else {
        let uid = unsafe { libc::geteuid() };
        user = match uname_from_uid(uid) {
            Some(x) => x,
            None => {
                return Err(InfoError::General(format!(
                    "Unable to get username from UID '{uid}'"
                )))
            }
        };
    };

    Ok(user)
}

fn caller_shell() -> Result<String, InfoError> {
    let shell = match env::var("0") {
        Ok(x) => PathBuf::from(x)
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        Err(_) => {
            let mut buffer: [c_char; 2048] = [0x0; 2048];
            let ppid = std::os::unix::process::parent_id();

            if unsafe { macos_get_caller(ppid as usize, &mut buffer as *mut i8, 2048) } < 0 {
                return Err(InfoError::General("Couln't get calling shell".to_string()));
            }

            let buffer = buffer.map(|char| char as u8);
            let shell = std::str::from_utf8(&buffer).unwrap().trim();
            match shell.strip_prefix('-') {
                Some(s) => s,
                None => shell,
            }
            .to_string()
        }
    };

    Ok(shell)
}

pub fn caller_info() -> Result<Caller, InfoError> {
    let name = caller_user()?;
    Ok(Caller {
        name,
        shell: caller_shell()?,
    })
}
