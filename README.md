[![Continuous Deployment](https://github.com/DevCleaner/devcleaner/actions/workflows/cd.yml/badge.svg)](https://github.com/DevCleaner/devcleaner/actions/workflows/cd.yml)
[![Continuous Integration](https://github.com/DevCleaner/devcleaner/actions/workflows/ci.yml/badge.svg)](https://github.com/DevCleaner/devcleaner/actions/workflows/ci.yml)
![Licence](https://img.shields.io/badge/license-MIT-blueviolet.svg)
![LOC](https://tokei.rs/b1/github/DevCleaner/devcleaner?category=code)
[![crates.io link](https://img.shields.io/crates/v/devcleaner.svg)](https://crates.io/crates/devcleaner)
![Release](https://img.shields.io/github/v/release/DevCleaner/devcleaner?color=%23c694ff)
[![Coverage](https://coveralls.io/repos/github/DevCleaner/devcleaner/badge.svg?branch=main)](https://coveralls.io/github/DevCleaner/devcleaner?branch=main)
[![GitHub Downloads](https://img.shields.io/github/downloads/DevCleaner/devcleaner/total.svg?label=GitHub%20downloads)](https://github.com/DevCleaner/devcleaner/releases)
![Crate.io downloads](https://img.shields.io/crates/d/devcleaner?label=Crate%20downloads)

[![Follow Yuniel Acosta Pérez (yacosta738)](https://img.shields.io/twitter/follow/yacosta738?label=Follow%20Yuniel%20Acosta%20Pérez%20%28yacosta738%29&style=social)](https://twitter.com/intent/follow?screen_name=yacosta738)

```
 ██████████                          █████████  ████                                                  
░░███░░░░███                        ███░░░░░███░░███                                                  
 ░███   ░░███  ██████  █████ █████ ███     ░░░  ░███   ██████   ██████   ████████    ██████  ████████ 
 ░███    ░███ ███░░███░░███ ░░███ ░███          ░███  ███░░███ ░░░░░███ ░░███░░███  ███░░███░░███░░███
 ░███    ░███░███████  ░███  ░███ ░███          ░███ ░███████   ███████  ░███ ░███ ░███████  ░███ ░░░ 
 ░███    ███ ░███░░░   ░░███ ███  ░░███     ███ ░███ ░███░░░   ███░░███  ░███ ░███ ░███░░░   ░███     
 ██████████  ░░██████   ░░█████    ░░█████████  █████░░██████ ░░████████ ████ █████░░██████  █████    
░░░░░░░░░░    ░░░░░░     ░░░░░      ░░░░░░░░░  ░░░░░  ░░░░░░   ░░░░░░░░ ░░░░ ░░░░░  ░░░░░░  ░░░░░ 
```


# DevCleaner
A tool to search for node_modules a clean the projects selected

## Installation

### Homebrew (Mac & Linux)

```bash
brew tap DevCleaner/devcleaner
brew install devcleaner

# If you need to be more specific, use:
brew install DevCleaner/devcleaner/devcleaner
```

To upgrade

```bash
brew upgrade devcleaner
```

### Scoop (Windows - Recommended way)

```bash
scoop bucket add kdash-bucket https://github.com/DevCleaner/scoop-devcleaner

scoop install devcleaner
```

### Chocolatey (Windows)

Choco package located [here](https://chocolatey.org/packages/devcleaner).
Since validation of the package takes forever, it may take a long while to become available after a release. I would recommend using Scoop instead for Windows.

```bash
choco install kdash

# Version number may be required for newer releases, if available:
choco install devcleaner --version=0.1.0
```

To upgrade

```bash
choco upgrade devcleaner --version=0.1.0
```

### Install script

Run the below command to install the latest binary. Run with sudo if you dont have write access to /usr/local/bin. Else the script will install to current directory

```sh
curl https://raw.githubusercontent.com/DevCleaner/devcleaner/main/deployment/getLatest.sh | bash
```

### Manual

Binaries for macOS, Linux and Windows are available on the [releases](https://github.com/DevCleaner/devcleaner/releases) page

1. Download the latest [binary](https://github.com/DevCleaner/devcleaner/releases) for your OS.
1. For Linux/macOS:
   1. `cd` to the file you just downloaded and run `tar -C /usr/local/bin -xzf downloaded-file-name`. Use sudo if required.
   1. Run with `devcleaner`
1. For Windows:
   1. Use 7-Zip or TarTool to unpack the tar file.
   1. Run the executable file `devcleaner.exe`


### Cargo

If you have Cargo installed then you install KDash from crates.io

```bash
cargo install devcleaner
```

> Note: On Debian/Ubuntu you might need to install `libxcb-xfixes0-dev` and `libxcb-shape0-dev`. On Fedora `libxcb` and `libxcb-devel` would be needed.

> Note: On Linux you might need to have package `xorg-dev` (Debian/Ubuntu) or `xorg-x11-server-devel` (Fedora) or equivalent installed for the copy to clipboard features to work

> Note: If you are getting compilation error from openSSL. Make sure perl and perl-core are installed for your OS.

You can also clone the repo and run `cargo run` or `make` to build and run the app

## USAGE:

```bash
devcleaner
```

Press `?` while running the app to see keybindings

## FLAGS:

- `-h, --help`: Prints help information
- `-V, --version`: Prints version information
- `-p, --path`: Set the path to scan for the criteria.
- `-c, --criteria`: Set the criteria to search.


## Licence

MIT

## Authors

- [Yuniel Acosta Pérez](https://blastkode.com/)
