use super::*;

pub fn os_info() -> Result<(String, crate::printing::OsArt), InfoError>
{
    let name = os_name()?;
    let art = os_art(&name)?;
    Ok((name, art))
}

pub fn os_name() -> Result<String, InfoError>
{
    let mut release_b = String::new();

    let os_name = match File::open(ETC_OS_RELEASE)
        .and_then(|mut f| f.read_to_string(&mut release_b))
    {
        Ok(_) =>
        {
            let name = release_b
                .split('\n')
                .find(|line| line.starts_with("NAME"))
                .unwrap_or_default()
                .split('=')
                .last()
                .unwrap_or_default()
                .replace('"', "");
            name.trim().to_string()
        }
        Err(_) =>
        {
            if File::open(ETC_LSB_RELEASE)
                .and_then(|mut f| f.read_to_string(&mut release_b))
                .is_err()
            {
                return Err(InfoError::FileRead {
                    path: ETC_LSB_RELEASE.to_string(),
                });
            }

            let name = release_b
                .split('\n')
                .find(|line| line.starts_with("DISTRIB_DESCRIPTION"))
                .unwrap_or_default()
                .split('=')
                .last()
                .unwrap_or_default()
                .replace('"', "");
            name.trim().to_string()
        }
    };

    Ok(os_name)
}

pub fn os_art(name: &str) -> Result<crate::printing::OsArt, InfoError>
{
    use printing::OsArt;
    Ok(match name
    {
        "Arch Linux" => OsArt::ArchLinux,
        "Alpine Linux" => OsArt::AlpineLinux,
        "Debian GNU/Linux" => OsArt::Debian,
        _ => OsArt::Unknown,
    })
}
