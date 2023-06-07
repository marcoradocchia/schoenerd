<div align="center">
  <h1 align="center"><code>schoenerd</code></h1>

  ![GitHub releases](https://img.shields.io/github/downloads/marcoradocchia/schoenerd/total?color=%23a9b665&logo=github)
  ![GitHub source size](https://img.shields.io/github/languages/code-size/marcoradocchia/schoenerd?color=ea6962&logo=github)
  ![GitHub open issues](https://img.shields.io/github/issues-raw/marcoradocchia/schoenerd?color=%23d8a657&logo=github)
  ![GitHub open pull requests](https://img.shields.io/github/issues-pr-raw/marcoradocchia/schoenerd?color=%2389b482&logo=github)
  ![GitHub sponsors](https://img.shields.io/github/sponsors/marcoradocchia?color=%23d3869b&logo=github)
  <!-- TODO: uncomment after publishing -->
  <!-- ![Crates.io downloads](https://img.shields.io/crates/d/schoenerd?label=crates.io%20downloads&logo=rust) -->
  <!-- ![Crates.io version](https://img.shields.io/crates/v/schoenerd?logo=rust&color=%23d8a657) -->
  ![GitHub license](https://img.shields.io/github/license/marcoradocchia/schoenerd?color=%23e78a4e)
</div>
  

<!-- TODO: uncomment after publishing -->
<!-- <a href="https://repology.org/project/schoenerd/versions"> -->
<!--   <img src="https://repology.org/badge/vertical-allrepos/schoenerd.svg" alt="Packaging status" align="right"> -->
<!-- </a> -->

Schoener's D index calculator for niche overlap.

![Schoener D Index](./assets/schoenerd.svg)

# Installation

## Build from source[^1]

The following building instructions build and install `schoenerd` from the
git `master` branch. They assume the Rust `stable` toolchain (or any
version `>= 1.70.0`) installed on the system[^2], as well as the
[`just`](https://github.com/casey/just) command runner.

```sh
git clone https://github.com/marcoradocchia/schoenerd
cd schoenerd
just build
sudo just install
```

By default, installation prefix is set to `/usr/local`. To use a different
installation prefix, specify it via the `PREFIX` environment variable as
`PREFIX=<prefix> sudo just install`, where `<prefix>` is a placeholder for the
desired path.

## GitHub releases

Pre-compiled binary of the
[latest release](https://github.com/marcoradocchia/schoenerd/releases/latest)
[^3], as well as shell completion scripts[^4] and manpage are available in 
GitHub releases.

## Cargo

Installing with Rust's `cargo` package manager will place the `schoenerd`
binary in `$CARGO_HOME/bin`, but will ignore shell completions and manpage.

### Master Branch

Build and install with `cargo` from git master branch:

```sh
cargo install --git https://github.com/marcoradocchia/schoenerd --branch master
```

### [crates.io](https://crates.io/crates/schoenerd)

Build and install with `cargo` from latest release:

```sh
cargo install schoenerd
```

## Arch User Repository

For Arch Linux users, packages are available in the **A**rch **U**ser
**R**epository:
- [`schoenerd`](https://aur.archlinux.org/packages/schoenerd): pre-compiled
    binary[^3] of the latest release;
- [`schoenerd-git`](https://aur.archlinux.org/packages/schoenerd-git): builds
    from source, from git master branch.

# Usage

<!-- TODO -->

## Input format

<!-- TODO -->

## Help

Below is listed the output of `schoenerd -h`:

```
schoenerd v0.1.0 - Marco Radocchia <marco.radocchia@outlook.com>, Gaia Di Francescantonio <gaiadfa@virgilio.it>
Schoener's D index calculator for niche overlap.

Usage:
  schoenerd [OPTIONS]

Options:
  -i, --input <FILE>
          Input CSV file path
  -f, --input-field-delimiter <CHAR>
          Input CSV field delimiter
  -t, --input-record-terminator <CHAR>
          Input CSV record terminator
  -c, --input-quote-character <CHAR>
          Input CSV quote character
  -o, --output <FILE>
          Output CSV file path
  -F, --output-field-delimiter <CHAR>
          Output CSV field delimiter
  -T, --output-record-terminator <CHAR>
          Output CSV record terminator
  -C, --output-quote-character <CHAR>
          Output CSV quote character
  -n, --disable-output-headers
          Disable output headers
  -s, --sort <DIRECTION>
          Sort output by D index value [possible values: descending, ascending]
  -p, --pretty-table
          Display output as a pretty table on stdout
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```

# License

[GPLv3](LICENSE)

[^1]: Compilation should work on all platforms (Linux, Window, MacOs, ...) as 
        long as `rust` and `just` are correctly installed, however installation
        recipes in the `justfile` are currently Linux only
[^2]: [Install using `rustup`](https://www.rust-lang.org/tools/install)
[^3]: Currently `x86_64` only
[^4]: Currently `zsh`, `bash` & `fish`