use crate::printing;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

    pub hostname: String,

    pub motherboard: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Caller
{
    /// The user running this program
    name: String,

    /// The shell running the program
    shell: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Cpu
{
    /// Cpu name
    pub name: String,

    /// Core count
    pub cores: usize,

    /// Thread count
    pub threads: usize,

    /// Cpu clock rate in Megahertz
    pub clock_rate: usize,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub struct Memory
{
    /// Total memory in megabytes
    pub total: usize,

    /// Available memory in megabytes
    pub available: usize,

    /// Used memory in megabytes
    pub used: usize,
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
