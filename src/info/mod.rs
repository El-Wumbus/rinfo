use thiserror::Error;
use std::path::Path;
use crate::printing;

#[cfg(target_os = "linux")]
pub mod linux as system


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone)]
pub enum InfoError
{
    #[error("MissingFileError: '{}' isn't an existing file", path.display())]
    MissingFile {path: PathBuf},

    #[error("ReadError: couldn't read file '{path}'")]
    FileRead {path: String},

    #[error("Error: Unexpected Error")]
    #[default]
    General,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
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
    pub clock_rate: usize,
}

impl Cpu
{
    pub fn read() -> Result<Self, InfoError>
    {
        if !system::INITIALIZED.lock()
        {
            system::init()?;
        }

        system::cpu_info()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Memory
{
    /// Total memory in Mib
    pub total: f64,

    /// Available memory in Mib
    pub available: f64,

    /// Used memory in Mib
    pub used: f64,
}

impl Memory
{
    pub fn read() -> Result<Self, InfoError>
    {
        if !system::INITIALIZED.lock()
        {
            system::init()?;
        }

        system::cpu_info()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub struct OperatingSystem
{
    pub name: String,
    pub kind: OsKind,
    pub art: printing::OsArt,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Default)]
pub enum OsKind
{
    Linux
    {
        kernel: String,
        kernel_version: String,
    },
    Windows
    {
        /// Update version
        version: String,

        /// The release (windows 11, windows 10, etc.)
        release: String,
    },

    MacOs
    {
        version: String,
    },
    Bsd,
    Unix,
    
    #[default]
    Unknown,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Caller
{
    /// The user running this program
    name: String,

    /// The shell running the program
    shell: String,
}