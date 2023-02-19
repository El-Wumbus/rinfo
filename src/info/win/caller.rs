use super::*;
pub fn caller_info() -> Result<Caller, InfoError>
{
    Ok(Caller {
        name: caller_name()?,
        shell: caller_shell()?,
    })
}

fn get_ppid(_pid: u32) -> Result<u32, InfoError>
{
    let info = &mut PROCESSENTRY32::default();
    info.dwSize = size_of::<PROCESSENTRY32>() as u32;
    let pid = unsafe { GetCurrentProcessId() };
    let mut ppid = 0;
    let mut err = Ok(());
    let snapshot = match unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, pid) }
    {
        Ok(h) => h,
        Err(e) => return Err(InfoError::General(format!("CreateToolhelp32Snapshot: {e}"))),
    };

    match unsafe { Process32First(snapshot, info) }.as_bool()
    {
        true => (),
        false =>
        {
            let e = match unsafe { GetLastError() }
            {
                ERROR_NO_MORE_FILES =>
                {
                    format!("Process32First: Snapshot doesn't contain process for '{pid}'")
                }
                e => format!("Process32First: Code {}", e.0),
            };
            err = Err(InfoError::General(e));
        }
    };

    if err.is_err()
    {
        unsafe { CloseHandle(snapshot) };
        return Err(err.err().unwrap());
    }

    loop
    {
        if info.th32ProcessID == pid
        {
            ppid = info.th32ParentProcessID;
            break;
        }

        if !unsafe { Process32Next(snapshot, info) }.as_bool()
        {
            break;
        }
    }

    unsafe { CloseHandle(snapshot) };
    Ok(ppid)
}

fn get_pid_info(pid: u32) -> Result<(PROCESSENTRY32, MODULEENTRY32), InfoError>
{
    let info = &mut PROCESSENTRY32::default();
    let module_info = &mut MODULEENTRY32::default();
    info.dwSize = size_of::<PROCESSENTRY32>() as u32;
    module_info.dwSize = size_of::<MODULEENTRY32>() as u32;
    let mut err = Ok(());

    let snapshot =
        match unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS | TH32CS_SNAPMODULE, pid) }
        {
            Ok(h) => h,
            Err(e) => return Err(InfoError::General(format!("CreateToolhelp32Snapshot: {e}"))),
        };

    match unsafe { Module32First(snapshot, module_info) }.as_bool()
    {
        true => (),
        false =>
        {
            let e = match unsafe { GetLastError() }
            {
                ERROR_NO_MORE_FILES =>
                {
                    format!("Module32First: Snapshot doesn't contain process for '{pid}'")
                }
                e => format!("Module32First: Code {}", e.0),
            };
            err = Err(InfoError::General(e));
        }
    };

    unsafe { CloseHandle(snapshot) };
    if err.is_err()
    {
        return Err(err.err().unwrap());
    }

    Ok((*info, *module_info))
}

fn caller_shell() -> Result<String, InfoError>
{
    use windows::Win32::System::Threading;

    if let Ok(x) = std::env::var("0")
    {
        return Ok(x);
    }
    else if let Ok(x) = std::env::var("shell_id")
    {
        return Ok(x);
    }

    // The current process pid
    let pid = unsafe { Threading::GetCurrentProcessId() };
    let ppid = get_ppid(pid)?;
    let (_, pp_module_info) = get_pid_info(ppid)?;

    // Map the `CHAR`s to their underlying `u8`s
    let exe = pp_module_info.szModule.map(|ch| ch.0);

    // Construct a string from the buffer
    let s = match std::str::from_utf8(&exe)
    {
        Ok(s) => s,
        Err(e) => return Err(InfoError::General(format!("Invalid UTF-8 sequence: {e}"))),
    };
    if let Some(s) = PathBuf::from(s).file_name()
    {
        return Ok(match PathBuf::from(s).file_stem()
        {
            Some(s) => s.to_string_lossy(),
            None => s.to_string_lossy(),
        }
        .to_string());
    }


    Ok(s.to_string())
}

/// Get the username of the caller
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
