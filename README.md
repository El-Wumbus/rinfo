# RINFO

[PKGBUILD]: https://github.com/Sir-Bobert-II/rinfo/raw/master/packages/PKGBUILD

Get basic information about your system from the command-line.
`rinfo` is a rust "rewrite" of `qinfo`. Unlike `qinfo`, `rinfo` is cross-platform
(Linux and Windows currently supported). `rinfo` is in beta, and it will stay there
until the information collection methods have stabilized.

## Usage

`rinfo` has configurable output. This can be achived through command-line flags or
with a configuration file. The default output looks similar to the following:

```txt
                   -`                     CPU: AMD Ryzen 5 5600X 6-Core Processor@4.64GHz (6 cores, 12 threads)
                  .o+`                    UPTIME: 5 days, 3 hours, 19 minutes, 26 seconds and 640 ms
                 `ooo/                    RAM: 11.31 GiB/31.27 GiB (19.96 GiB available)
                `+oooo:                   BOARD: B550 GAMING X V2 (Gigabyte Technology Co., Ltd.)
               `+oooooo:                  HOST: Basestation
               -+oooooo+:                 USER: decator
             `/:-:++oooo+:                SHELL: zsh
            `/++++/+++++++:               OS: Arch Linux (linux)
           `/++++++++++++++:              
          `/+++ooooooooooooo/`            
         ./ooosssso++osssssso+`           
        .oossssso-````/ossssss+`          
       -osssssso.      :ssssssso.         
      :osssssss/        osssso+++.        
     /ossssssss/        +ssssooo/-        
   `/ossssso+/:-        -:/+osssso+-      
  `+sso+:-`                 `.-/+oso:     
 `++:.                           `-/+\    
 .`                                 ` .   
```

### Flags

```txt
rinfo 0.1.0
Get information about your system

USAGE:
    rinfo [FLAGS]

FLAGS:
    -h, --help                Prints help information
    -a, --omit-art            Don't print character art
    -p, --omit-caller         Don't print caller (USER, SHELL) information
    -c, --omit-cpu            Don't print CPU information
    -n, --omit-hostname       Don't print the system hostname
    -m, --omit-motherboard    Don't print motherboard information
    -o, --omit-os             Don't print operating system information
    -r, --omit-ram            Don't print RAM information
    -V, --version             Prints version information
    -v, --vertical-art        Print character art above information
```

#### Example

```txt
$ rinfo -ap --omit-os
CPU: AMD Ryzen 5 5600X 6-Core Processor@2.06GHz (6 cores, 12 threads)
UPTIME: 5 days, 3 hours, 5 minutes, 1 second and 390 ms
RAM: 11.81 GiB/31.27 GiB (19.46 GiB available)
BOARD: B550 GAMING X V2 (Gigabyte Technology Co., Ltd.)
HOST: Basestation
```

### Config File

Depending on your OS, the configuration file will be in a different location:

* *Linux* - `$XDG_CONFIG_HOME/SBII/rinfo.toml` or `$HOME/.config/SBII/rinfo.toml`
(e.g. `/home/awesomeguy420/.config/SBII/rinfo.toml`)
* *Windows* - `{FOLDERID_RoamingAppData}\SBII\rinfo.toml`
(e.g. `C:\Users\CoolGuy69\AppData\Roaming\SBII\rinfo.toml`)

The configuration file uses the `TOML` format, an example of one is seen below.

```toml
omitCpu = false
omitRam = false
omitMotherboard = true
omitCaller = false
omitHostname = false
omitOs = false
omitArt = false
verticalArt = true
```

By default, any flags passed to the program will take precidence over the configuration.
This meaning, with the above configuration, the output of `rinfo --omit-art` won't contian the art
depite the configuration file specifing otherwise.

## Installing

When there's a full release of `rinfo`, there will a windows installer and built packages. Until then,
manual compilation will have to suffice (exept on Arch Linux, theres a [pkgbuild][PKGBUILD]).

### Linux

[Arch Linux](#arch-lnux) does have differing instructions, skip there if needed.

1. Clone the repository
   
    ```sh
    git clone https://github.com/Sir-Bobert-II/rinfo
    ```

2. Build

   ```sh
   cd rinfo
   cargo build --release
   ```

3. Install
    ```sh
    install -Dvm755 target/release/rinfo /usr/bin/rinfo
    ```

#### Arch Lnux

1. Download the [`PKGBUILD`][PKGBUILD]
    
    ```bash
    curl -LO https://github.com/Sir-Bobert-II/rinfo/raw/master/packages/PKGBUILD
    ```

2. Build and install
    
    ```zsh
    makepkg -si
    ```
