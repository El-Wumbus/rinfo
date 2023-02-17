use super::*;

pub fn caller_info() -> Result<Caller, InfoError>
{
    Ok(Caller {
        name: caller_user()?,
        shell: caller_shell()?,
    })
}


fn caller_shell() -> Result<String, InfoError>
{
    let shell = match env::var("0")
    {
        Ok(x) =>
        {
            PathBuf::from(x)
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_string()
        }
        Err(_) =>
        {
            let ppid = std::os::unix::process::parent_id();
            match std::fs::read_link(format!("/proc/{ppid}/exe"))
            {
                Ok(x) =>
                {
                    x.to_string_lossy()
                        .split('/')
                        .last()
                        .unwrap_or_default()
                        .to_string()
                }
                Err(_) =>
                {
                    return Err(InfoError::General(format!(
                        "Couldn't readlink '/proc/{ppid}/exe'"
                    )))
                }
            }
        }
    };

    Ok(shell)
}


fn caller_user() -> Result<String, InfoError>
{
    let user = match env::var("USER")
    {
        Ok(x) => x,
        Err(_) =>
        {
            let uid = unsafe { libc::geteuid() };
            match uname_from_uid(uid)
            {
                Some(x) => x,
                None =>
                {
                    return Err(InfoError::General(format!(
                        "Unable to get username from UID '{uid}'"
                    )))
                }
            }
        }
    };

    Ok(user)
}
