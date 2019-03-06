# Oracle
[![asciicast](https://asciinema.org/a/r0ZsIgR5rlrOg2eyY9GeFiOta.png)](https://asciinema.org/a/r0ZsIgR5rlrOg2eyY9GeFiOta)

## Requirements
Needs rust installed and cargo make

### Install Rust
```bash
curl -sf -L https://static.rust-lang.org/rustup.sh | sh
```
### Install Cargo Make
```bash
cargo install cargo-make
```

## Build Oracle
```bash
git clone https://github.com/FriedPandaFries/oracle.git
cd oracle
cargo make --makefile Build.toml build
```

### Or
###### One line version
```bash
git clone https://github.com/FriedPandaFries/oracle.git && cd oracle && cargo make --makefile Build.toml build
```

## Goals
- Be extremely fast
- Support all Unix Distributions

## TODOs
- [x] Be extremely fast
- [ ] Support 50 Linix Distros
- [x] Create a method to download Distros
- [ ] Make distros list print in columns based on terminal width and height

## Support
| # |  Distribution |Ascii| Download   | Detect |
|---|---------------|-----|------------|--------|
|  1|Aix            | yes | no         |   no   |
|  2|Alpine         | yes | no         |   no   |
|  3|Anarchy        | yes | no         |   no   |
|  4|Android        | yes | no		   |   yes  |
|  5|Antergos       | yes | no         |   no   |
|  6|Antix          | yes | no         |   no   |
|  7|AOSC           | yes | no         |   no   |
|  8|Apricity       | yes | no         |   no   |
|  9|Arch Linux     | yes | no         |   yes  |
| 10|Archobox       | yes | no         |   no   |
| 11|Archlabs       | yes | no         |   no   |
| 12|Archmerge      | yes | no         |   no   |
| 13|Arch Xferience | yes | no         |   no   |
| 14|Artix          | yes | no         |   no   |
| 15|Arya           | yes | no         |   no   |
| 16|Blag           | yes | no         |   no   |
| 17|Bitrig         | yes | no         |   no   |
| 18|Elementary     | yes | no		   |   yes  |
| 19|Fedora         | yes | no         |   yes  |
| 20|Kali           | yes | no         |   yes  |
| 21|Kubuntu        | yes |<b> yes </b>|   yes  |
| 22|Linux Mint     | yes | no		   |   yes  |
| 23|Lubuntu        | yes |<b> yes </b>|   yes  |
| 24|Manjaro        | yes | no         |   yes  |
| 25|Neon           | yes | no         |   yes  |
| 26|Qubes          | yes | no         |   yes  |
| 27|Raspbian       | yes | no         |   yes  |
| 28|RHEL		    | yes | no		   |   yes  |
| 29|Ubuntu         | yes |<b> yes </b>|   yes  |
| 30|Ubuntu Budgie  | yes | no         |   yes  |
| 31|Ubuntu MATE    | yes |<b> yes </b>|   yes  |
| 32|Ubuntu Studio  | yes |<b> yes </b>|   yes  |
| 33|Void Linux	    | yes | no		   |   yes  |
| 34|Xubuntu	    | yes |<b> yes </b>|   yes  |
