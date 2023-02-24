use crate::printing;
use humansize::{FormatSize, BINARY};
use std::path::PathBuf;
use thiserror::Error;
pub mod common;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "linux")]
pub use linux as system;

#[cfg(target_os = "windows")]
pub mod win;

#[cfg(target_os = "windows")]
pub use win as system;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "macos")]
pub use macos as system;


#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
#[allow(dead_code)]
pub enum InfoError
{
    #[error("MissingFileError: '{}' isn't an existing file", path.display())]
    MissingFile
    {
        path: PathBuf
    },

    #[error("ReadError: couldn't read file '{path}'")]
    FileRead
    {
        path: String
    },

    #[error("FileParseError: couldn't parse from file '{path}': {reason}")]
    FileParseError
    {
        path: String, reason: String
    },

    #[error("SysctlError: couldn't get '{name}'")]
    Sysctl
    {
        name: String
    },

    #[error("Error: Unexpected Error: {0}")]
    General(String),
}

impl InfoError
{
    pub fn report<T>(e: Result<T, Self>) -> T
    {
        match e
        {
            Err(e) =>
            {
                let code = match e
                {
                    Self::FileParseError { path: _, reason: _ } => 65,
                    Self::Sysctl { name: _ } => 71,
                    Self::MissingFile { path: _ } => 72,
                    Self::FileRead { path: _ } => 74,
                    _ => 64,
                };

                eprintln!("{e}");
                std::process::exit(code);
            }
            Ok(x) => x,
        }
    }
}

pub trait Information
{
    fn read() -> Result<Self, InfoError>
    where
        Self: Sized;
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct Cpu
{
    /// Cpu name
    pub name: String,

    /// Cpu uptime in milliseconds
    pub uptime: u128,

    /// Core count
    pub cores: usize,

    /// Thread count
    pub threads: usize,

    /// Cpu clock rate in Megahertz
    pub clock_rate: f64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default, Copy)]
pub struct Memory
{
    /// Total memory in Bytes
    pub total: u64,

    /// Available memory in Bytes
    pub available: u64,

    /// Used memory in Bytes
    pub used: u64,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default, Copy)]
pub enum OsKind
{
    Linux,
    Windows,
    MacOs,
    FreeBsd,

    #[default]
    Unknown,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default)]
pub struct Net
{
    /// The local IP address used to access the internet
    local_ip: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default)]
/// Information relating to the Baseboard (Motherboard)
pub struct BaseBoard
{
    /// The name of the board
    pub model: String,

    /// The board vendor
    pub vendor: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default)]
pub struct Host
{
    /// Name of the host PC
    pub hostname: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default)]
pub struct Caller
{
    /// The user running this program
    name: String,

    /// The shell running the program
    shell: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct OperatingSystem
{
    /// Display name of the OS
    pub name: String,

    /// The kind of OS
    pub kind: OsKind,

    /// The OS art
    pub art: printing::OsArt,
}

impl Information for Net
{
    fn read() -> Result<Self, InfoError>
    {
        if !system::initialized()
        {
            system::init()?;
        }

        system::net_info()
    }
}

impl Information for BaseBoard
{
    fn read() -> Result<Self, InfoError>
    {
        if !system::initialized()
        {
            system::init()?;
        }

        system::motherboard_info()
    }
}

impl Information for Host
{
    fn read() -> Result<Self, InfoError>
    {
        if !system::initialized()
        {
            system::init()?;
        }

        system::hostname_info()
    }
}

impl Information for Cpu
{
    fn read() -> Result<Self, InfoError>
    {
        if !system::initialized()
        {
            system::init()?;
        }

        system::cpu_info()
    }
}

impl Information for Memory
{
    fn read() -> Result<Self, InfoError>
    {
        if !system::initialized()
        {
            system::init()?;
        }

        system::memory_info()
    }
}

impl Information for OperatingSystem
{
    // TODO: FINISH
    fn read() -> Result<Self, InfoError>
    {
        let kind = OsKind::read()?;
        let (name, art) = system::os_info()?;

        Ok(Self { name, kind, art })
    }
}

impl Information for OsKind
{
    fn read() -> Result<Self, InfoError>
    {
        Ok(match std::env::consts::OS
        {
            "linux" => Self::Linux,
            "windows" => Self::Windows,
            "macos" => Self::MacOs,
            "freebsd" => Self::FreeBsd,

            _ => Self::default(),
        })
    }
}

impl Information for Caller
{
    fn read() -> Result<Self, InfoError> { system::caller_info() }
}

impl std::fmt::Display for BaseBoard
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let vendor = if self.vendor.is_empty()
        {
            self.vendor.clone()
        }
        else
        {
            format!(" ({})", self.vendor)
        };

        write!(f, "BOARD {}{vendor}", self.model)
    }
}

impl std::fmt::Display for Host
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "HOST: {}", self.hostname)
    }
}

impl std::fmt::Display for Cpu
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        use chrono::Duration;
        use chrono_humanize::{Accuracy, HumanTime, Tense};
        let uptime = HumanTime::from(Duration::milliseconds(-(self.uptime as i64)));
        write!(
            f,
            "CPU: {}@{:.2}GHz ({} cores, {} threads)\nUPTIME: {}",
            self.name,
            self.clock_rate / 1000.0,
            self.cores,
            self.threads,
            uptime.to_text_en(Accuracy::Precise, Tense::Present)
        )
    }
}

impl std::fmt::Display for Memory
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(
            f,
            "RAM: {}/{} ({} available)",
            self.used.format_size(BINARY),
            self.total.format_size(BINARY),
            self.available.format_size(BINARY),
        )
    }
}

impl std::fmt::Display for OperatingSystem
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "OS: {} ({})", self.name, self.kind)
    }
}

impl std::fmt::Display for OsKind
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        let s = match self
        {
            Self::Linux => "linux",
            Self::Windows => "windows",
            Self::MacOs => "macos",
            Self::FreeBsd => "freebsd",

            _ => "Unknown",
        };
        write!(f, "{s}")
    }
}

impl std::fmt::Display for Net
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "LAN: {} (IPV4)", self.local_ip)
    }
}

impl std::fmt::Display for Caller
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "USER: {}\nSHELL: {}", self.name, self.shell)
    }
}
