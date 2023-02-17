use super::*;

pub fn memory_info() -> Result<Memory, InfoError>
{
    let mut meminfo_file = String::new();

    if File::open(PROC_MEMINFO)
        .and_then(|mut f| f.read_to_string(&mut meminfo_file))
        .is_err()
    {
        return Err(InfoError::FileRead {
            path: PROC_MEMINFO.to_string(),
        });
    }

    let mut meminfo = meminfo_file.split('\n');

    let meminfo_total = match meminfo.find(|line| line.starts_with("MemTotal"))
    {
        Some(x) => x,
        None =>
        {
            return Err(InfoError::General(
                "Couldn't find line that starts with 'MemTotal'".to_string(),
            ))
        }
    };

    let total = match meminfo_total
        .split(':')
        .last()
        .unwrap_or_default()
        .trim()
        .split(' ')
        .next()
        .unwrap_or_default()
        .trim()
        .parse::<f32>()
    {
        Ok(x) => x,
        Err(e) =>
        {
            return Err(InfoError::FileParseError {
                path: PROC_MEMINFO.to_string(),
                reason: e.to_string(),
            })
        }
    } / 1049.0;
    let meminfo_available = match meminfo.find(|line| line.starts_with("MemAvailable"))
    {
        Some(x) => x,
        None =>
        {
            return Err(InfoError::General(
                "Couldn't find line that starts with 'MemAvailable'".to_string(),
            ))
        }
    };

    let available = match meminfo_available
        .split(':')
        .last()
        .unwrap_or_default()
        .trim()
        .split(' ')
        .next()
        .unwrap_or_default()
        .trim()
        .parse::<f32>()
    {
        Ok(x) => x,
        Err(e) =>
        {
            return Err(InfoError::FileParseError {
                path: PROC_MEMINFO.to_string(),
                reason: e.to_string(),
            })
        }
    } / 1049.0;

    let used = total - available;

    Ok(Memory {
        total,
        available,
        used,
    })
}
