use crate::printing;
use humansize::{format_size, DECIMAL};
use std::path::PathBuf;
use thiserror::Error;

pub mod linux;

#[cfg(target_os = "linux")]
use linux as system;

pub use system::{hostname_info, motherboard_info};

use self::linux::os_info;

#[derive(Error, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
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

    #[error("Error: Unexpected Error: {0}")]
    General(String),
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Info
{
    /// Cpu information
    pub cpu: Cpu,

    /// RAM info
    pub memory: Memory,

    /// OS info
    pub os: OperatingSystem,

    /// The user who's calling the program
    pub user: Caller,

    /// The Hostname
    pub hostname: String,

    pub motherboard_name: String,
}

impl Info
{
    pub fn read() -> Result<Self, InfoError>
    {
        let cpu = Cpu::read()?;
        let memory = Memory::read()?;
        let os = OperatingSystem::read()?;
        let user = Caller::read()?;
        let hostname = hostname_info()?;
        let motherboard_name = motherboard_info()?;

        Ok(Self {
            cpu,
            memory,
            os,
            user,
            hostname,
            motherboard_name,
        })
    }
}


#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub struct Cpu
{
    /// Cpu name
    pub name: String,

    /// Cpu uptime in seconds
    pub uptime: f64,

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
        write!(
            f,
            "CPU: {}@{:.2}GHz ({} cores, {} threads)",
            self.name,
            self.clock_rate / 1000.0,
            self.cores,
            self.threads
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

#[derive(Debug, PartialEq, PartialOrd, Clone)]
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
            format_size(self.used, DECIMAL),
            format_size(self.total, DECIMAL),
            format_size(self.available, DECIMAL),
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
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
        let (name, art) = os_info()?;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
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
