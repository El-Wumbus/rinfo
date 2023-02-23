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
        if e.is_err()
        {
            let e = e.err().unwrap();
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

        e.unwrap()
    }
}


pub struct Net
{
    local_ip: String,
}

impl std::fmt::Display for Net
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "LAN: {} (IPV4)", self.local_ip)
    }
}

impl Net
{
    pub fn read() -> Result<Self, InfoError>
    {
        if !system::initialized()
        {
            system::init()?;
        }

        system::net_info()
    }
}

pub struct BaseBoard
{
    pub model: String,
    pub vendor: String,
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

impl BaseBoard
{
    pub fn read() -> Result<Self, InfoError>
    {
        if !system::initialized()
        {
            system::init()?;
        }

        system::motherboard_info()
    }
}

pub struct Host
{
    pub hostname: String,
}

impl std::fmt::Display for Host
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "HOST: {}", self.hostname)
    }
}

impl Host
{
    pub fn read() -> Result<Self, InfoError>
    {
        if !system::initialized()
        {
            system::init()?;
        }

        system::hostname_info()
    }
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

impl Cpu
{
    pub fn read() -> Result<Self, InfoError>
    {
        if !system::initialized()
        {
            system::init()?;
        }

        system::cpu_info()
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Default)]
pub struct Memory
{
    /// Total memory in Bytes
    pub total: u64,

    /// Available memory in Bytes
    pub available: u64,

    /// Used memory in Bytes
    pub used: u64,
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

impl Memory
{
    pub fn read() -> Result<Self, InfoError>
    {
        if !system::initialized()
        {
            system::init()?;
        }

        system::memory_info()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub struct OperatingSystem
{
    pub name: String,
    pub kind: OsKind,
    pub art: printing::OsArt,
}

impl std::fmt::Display for OperatingSystem
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "OS: {} ({})", self.name, self.kind)
    }
}

impl OperatingSystem
{
    // TODO: FINISH
    pub fn read() -> Result<Self, InfoError>
    {
        let kind = OsKind::read();
        let (name, art) = system::os_info()?;

        Ok(Self { name, kind, art })
    }
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

impl OsKind
{
    pub fn read() -> Self
    {
        match std::env::consts::OS
        {
            "linux" => Self::Linux,
            "windows" => Self::Windows,
            "macos" => Self::MacOs,
            "freebsd" => Self::FreeBsd,

            _ => Self::default(),
        }
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default)]
pub struct Caller
{
    /// The user running this program
    name: String,

    /// The shell running the program
    shell: String,
}

impl std::fmt::Display for Caller
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "USER: {}\nSHELL: {}", self.name, self.shell)
    }
}

impl Caller
{
    pub fn read() -> Result<Self, InfoError> { system::caller_info() }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default)]
pub struct Gpu
{
    model: String,
    vendor: String,
}

impl std::fmt::Display for Gpu
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "GPU: {} ({})", self.model, self.vendor)
    }
}

impl Gpu
{
    pub fn read() -> Result<Self, InfoError> { system::gpu_info() }
}