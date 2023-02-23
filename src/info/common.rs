use ogl33::{glGetString, GLubyte, GL_VENDOR};

use super::InfoError;

pub fn ipv4_to_int(s: &str) -> u32
{
    let mut s = s.split('.');
    let a = s
        .next()
        .unwrap_or_default()
        .trim()
        .parse::<u32>()
        .unwrap_or_default();
    let b = s
        .nth(1)
        .unwrap_or_default()
        .trim()
        .parse::<u32>()
        .unwrap_or_default()
        << 8;
    let c = s
        .nth(2)
        .unwrap_or_default()
        .trim()
        .parse::<u32>()
        .unwrap_or_default()
        << 16;
    let d = s
        .nth(3)
        .unwrap_or_default()
        .trim()
        .parse::<u32>()
        .unwrap_or_default()
        << 24;
    (a + b + c + d).to_be()
}

pub fn int_to_ipv4(i: u32) -> String
{
    let i = i.to_le();
    let a = i & 0xFF;
    let b = (i >> 8) & 0xFF;
    let c = (i >> 16) & 0xFF;
    let d = (i >> 24) & 0xFF;

    format!("{a}.{b}.{c}.{d}")
}

pub fn get_gpu_name_gl() -> Result<(), InfoError> {
    let vendor_name: *const GLubyte = unsafe { glGetString(GL_VENDOR) };
    if vendor_name.is_null()
    {
        return Err(InfoError::General("OPENGL: failed to return gpu vendor name".to_string()));
    }

    Ok(())
}