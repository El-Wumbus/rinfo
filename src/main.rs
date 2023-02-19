#![allow(non_camel_case_types)]
use std::fs::read_to_string;

use serde::{Deserialize, Serialize};
use structopt::StructOpt;
mod info;
mod printing;

#[derive(Debug, StructOpt, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
#[structopt(name = "rinfo", about = "Get information about your system")]
#[serde(rename_all = "camelCase")]
struct Config
{
    /// Don't print CPU information
    #[structopt(short = "c", long)]
    omit_cpu: bool,

    /// Don't print RAM information
    #[structopt(short = "r", long)]
    omit_ram: bool,

    /// Don't print motherboard information
    #[structopt(short = "m", long)]
    omit_motherboard: bool,

    /// Don't print caller (USER, SHELL) information
    #[structopt(short = "p", long)]
    omit_caller: bool,

    /// Don't print the system hostname
    #[structopt(short = "n", long)]
    omit_hostname: bool,

    /// Don't print operating system information
    #[structopt(short = "o", long)]
    omit_os: bool,

    /// Don't print character art
    #[structopt(short = "a", long)]
    omit_art: bool,

    /// Print character art above information
    #[structopt(short = "v", long)]
    vertical_art: bool,
}

impl Config
{
    pub fn combine(&mut self, other: Self)
    {
        if !self.omit_art && other.omit_art
        {
            self.omit_art = true
        }
        if !self.omit_caller && other.omit_caller
        {
            self.omit_caller = true
        }
        if !self.omit_cpu && other.omit_cpu
        {
            self.omit_cpu = true
        }
        if !self.omit_hostname && other.omit_hostname
        {
            self.omit_hostname = true
        }
        if !self.omit_motherboard && other.omit_motherboard
        {
            self.omit_motherboard = true
        }
        if !self.omit_os && other.omit_os
        {
            self.omit_os = true
        }
        if !self.vertical_art && other.vertical_art
        {
            self.vertical_art = true
        }
    }
}

fn main()
{
    let mut config = Config::default();

    if let Some(config_dir) = dirs::config_dir()
    {
        let config_file = config_dir.join("SBII").join("rinfo.toml");
        if config_file.is_file()
        {
            if let Ok(contents) = read_to_string(config_file)
            {
                match toml::from_str(&contents)
                {
                    Ok(x) => config = x,
                    Err(e) => eprintln!("Couldn't load config file: {e}"),
                }
            }
        }
    }

    config.combine(Config::from_args());

    let info = info::Info::read().unwrap();
    let mut info_str = String::new();

    if !config.omit_cpu
    {
        info_str.push_str(&info.cpu.to_string());
    }

    if !config.omit_ram
    {
        info_str.push_str(&format!("\n{}", info.memory));
    }

    if !config.omit_motherboard
    {
        info_str.push_str(&format!("\nBOARD: {}", info.motherboard_name));
    }

    if !config.omit_hostname
    {
        info_str.push_str(&format!("\nHOST: {}", info.hostname));
    }

    if !config.omit_caller
    {
        info_str.push_str(&format!("\n{}", info.user));
    }

    if !config.omit_os
    {
        info_str.push_str(&format!("\n{}", info.os));
    }

    if config.omit_art
    {
        println!("{info_str}");
    }
    else if config.vertical_art
    {
        println!("{}\n{info_str}", info.os.art);
    }
    else
    {
        printing::print_with_logo(info.os.art, &info_str);
    }
}
