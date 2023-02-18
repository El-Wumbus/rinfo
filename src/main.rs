mod info;
mod printing;

fn main()
{
    let info = info::Info::read().unwrap();

    let mut info_str = String::new();

    info_str.push_str(&info.cpu.to_string());
    info_str.push_str(&format!("\n{}", info.memory));
    info_str.push_str(&format!("\nBOARD: {}", info.motherboard_name));
    info_str.push_str(&format!("\nHOST: {}", info.hostname));
    info_str.push_str(&format!("\n{}", info.user));
    info_str.push_str(&format!("\n{}", info.os));

    printing::print_with_logo(info.os.art, &info_str);
}
