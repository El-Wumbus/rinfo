#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};
use structopt::StructOpt;
mod info;
mod printing;
use info::*;
use std::io::Write;

/// Conditionally append the result of a function to the string to be later
/// printed.
macro_rules! add_info {
    ($vec:expr, $cond:expr, $func:expr) => {
        if $cond
        {
            let result = $func();
            let info = InfoError::report(result);
            write!($vec, "\n{}", info).unwrap();
        }
    };
}

#[derive(
    Debug, StructOpt, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default,
)]
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

    /// Don't print local IP address
    #[structopt(short = "i", long)]
    omit_ip: bool,

    /// Print character art above information
    #[structopt(short = "v", long)]
    vertical_art: bool,
}

impl Config
{
    /// Combine mutliple configs in a way that prioritizes the `other`'s true
    /// fields over `self`'s false ones.
    pub fn combine(&mut self, other: Self)
    {
        self.omit_art |= !self.omit_art && other.omit_art;
        self.omit_caller |= !self.omit_caller && other.omit_caller;
        self.omit_cpu |= !self.omit_cpu && other.omit_cpu;
        self.omit_hostname |= !self.omit_hostname && other.omit_hostname;
        self.omit_motherboard |= !self.omit_motherboard && other.omit_motherboard;
        self.omit_os |= !self.omit_os && other.omit_os;
        self.vertical_art |= !self.vertical_art && other.vertical_art;
        self.omit_ip |= !self.omit_ip && other.omit_ip;
    }
}

fn main()
{
    // Load configuration
    let mut config = Config::default();
    if let Some(config_dir) = dirs::config_dir()
    {
        let config_file = config_dir.join("SBII").join("rinfo.toml");
        if let Ok(contents) = std::fs::read_to_string(config_file)
        {
            match toml::from_str(&contents)
            {
                Ok(config_from_file) =>
                {
                    config.combine(config_from_file);
                }
                Err(e) =>
                {
                    eprintln!("Couldn't parse config file: {e:?}");
                }
            }
        }
    }

    config.combine(Config::from_args());


    // Build information string
    let mut info_vec = Vec::new();
    let os = InfoError::report(OperatingSystem::read());

    add_info!(info_vec, !config.omit_cpu, &Cpu::read);
    add_info!(info_vec, !config.omit_ram, &Memory::read);
    add_info!(info_vec, !config.omit_motherboard, &BaseBoard::read);
    add_info!(info_vec, !config.omit_ip, &Net::read);
    add_info!(info_vec, !config.omit_hostname, &Host::read);
    add_info!(info_vec, !config.omit_caller, &Caller::read);
    add_info!(info_vec, !config.omit_os, &|| Ok(os.clone()));

    let info_str = String::from_utf8_lossy(&info_vec).trim_start().to_string(); // We `trim_start()` to trim the leading newline

    // Print information
    if config.omit_art
    {
        println!("{info_str}");
    }
    else if config.vertical_art
    {
        println!("{}\n{info_str}", os.art);
    }
    else
    {
        printing::print_with_logo(os.art, &info_str);
    }
}
