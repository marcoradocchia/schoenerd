<div align="center">
  <h1 align="center"><code>schoenerd</code></h1>

  ![GitHub releases](https://img.shields.io/github/downloads/marcoradocchia/schoenerd/total?color=%23a9b665&logo=github)
  ![GitHub source size](https://img.shields.io/github/languages/code-size/marcoradocchia/schoenerd?color=ea6962&logo=github)
  ![GitHub open issues](https://img.shields.io/github/issues-raw/marcoradocchia/schoenerd?color=%23d8a657&logo=github)
  ![GitHub open pull requests](https://img.shields.io/github/issues-pr-raw/marcoradocchia/schoenerd?color=%2389b482&logo=github)
  ![GitHub sponsors](https://img.shields.io/github/sponsors/marcoradocchia?color=%23d3869b&logo=github)
  ![Crates.io downloads](https://img.shields.io/crates/d/schoenerd?label=crates.io%20downloads&logo=rust)
  ![Crates.io version](https://img.shields.io/crates/v/schoenerd?logo=rust&color=%23d8a657)
  ![GitHub license](https://img.shields.io/github/license/marcoradocchia/schoenerd?color=%23e78a4e)
</div>
  

<a href="https://repology.org/project/schoenerd/versions">
  <img src="https://repology.org/badge/vertical-allrepos/schoenerd.svg" alt="Packaging status" align="right">
  <img src="https://repology.org/badge/vertical-allrepos/schoenerd-bin.svg" alt="Packaging status" align="right">
  <img src="https://repology.org/badge/vertical-allrepos/schoenerd-git.svg" alt="Packaging status" align="right">
</a>

Schoener's D index calculator for niche overlap.

```math
D_{ij} = 1 - \frac{1}{2} \sum_{k = 1}^{S} \left\lvert P_{ik} - P_{jk} \right\rvert
```

`schoenerd` is a CLI program designed to compute the _Schoener's D Index_ for
niche overlap between pairs of species[^1] exploiting a common resource.

# Installation

## Build from source[^2]

The following building instructions compile and install `schoenerd` from the
git `master` branch. They assume the Rust `stable` toolchain (or any
version `>= 1.70.0`) installed on the system[^3], as well as the
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

<!-- WARNING: what about BSD? -->
> **NOTE**: The `install` recipe in the `justfile` is Linux-specific and not
>        available on other platforms. In order to install `schoenerd` on a
>        non-Linux platform, manual installation of the build artifacts
>        (binary, completions, manpage, etc.) is required.

## GitHub releases

Pre-compiled binary[^4] of the
[latest release](https://github.com/marcoradocchia/schoenerd/releases/latest),
 as well as shell completion scripts[^5] and manpage are available in
GitHub releases.

## Cargo

Installing with Rust's `cargo` package manager is available on any
platform (Linux, Windows, macOS, etc.).

> **NOTE**: `cargo` installs the `schoenerd` binary in `$CARGO_HOME/bin`,
>         however it ignores shell completions and manpage. If you need either
>         of those, consider using one of the other installation options.

### Master Branch

Build and install with `cargo` on any platform from git master branch:

```sh
cargo install --git https://github.com/marcoradocchia/schoenerd --branch master
```

### [crates.io](https://crates.io/crates/schoenerd)

Build and install with `cargo` on any platform from latest release:

```sh
cargo install schoenerd
```

## Arch User Repository

For Arch Linux users, packages are available in the **A**rch **U**ser
**R**epository:
- [`schoenerd`](https://aur.archlinux.org/packages/schoenerd):
        builds from source, from latest release;
- [`schoenerd-bin`](https://aur.archlinux.org/packages/schoenerd-bin):
        pre-compiled binary[^4] of the latest release;
- [`schoenerd-git`](https://aur.archlinux.org/packages/schoenerd-git): builds
        from source, from git master branch.

> **WARNING**: `schoenerd`, `schoenerd-bin` & `schoenerd-git` AUR packages are
>        mutually exclusive and conflict with each other.


You may install one of the above packages using an AUR helper, such as
[`yay`](https://github.com/Jguer/yay):

```sh
yay -S schoenerd-bin # or `schoenerd`, or `schoenerd-git`
```

or [`paru`](https://github.com/Morganamilo/paru):

```sh
paru -S schoenerd-bin # or `schoenerd`, or `schoenerd-git`
```

# Usage

`schoenerd` accepts input data either from stdin or input file
(using the `-i/--input` option) and produces output either to stdout or output
file (using the `-o/--output` option). This allows `schoenerd` to be used as a
standalone program or inside a UNIX pipeline, which makes it suitable for
usage along with other programs or inside scripts.

## Input format

Input data must be formatted as a **C**omma **S**eparated **V**alue table.
Take as an example the data table below:

|                             | **_Bombus lapidarius_** | **_Bombus mesomelas_** | **_Bombus pascuorum_** | **_Bombus pratorum_** |
| --------------------------- | ----------------------- | ---------------------- | ---------------------- | --------------------- |
| **_Carduus chrysacanthus_** | 0                       | 0                      | 0                      | 1                     |
| **_Carlina acaulis_**       | 0                       | 0                      | 1                      | 0                     |
| **_Stachys germanica_**     | 18                      | 5                      | 14                     | 134                   |
| **_Trifolium pratense_**    | 0                       | 1                      | 0                      | 1                     |
| **_Trifolium repens_**      | 0                       | 4                      | 0                      | 6                     |
| **_Oxytropis campestris_**  | 3                       | 153                    | 0                      | 53                    |

If one wanted to calculate the Schoener's D Index for each pair of pollinators,
column headers of the table should contain each pollinator species, while row
headers should contain each of the plant species representing the available
resources. Each intersection cell must represent a non-negative integer value
describing the interaction between each plant/pollinator pair
(e.g. _number of visits_).

`schoenerd` requires such a table to be serialized in a CSV format, like below:

```
,Bombus pratorum,Bombus lapidarius,Bombus mesomelas,Bombus pascuorum
Carduus chrysacanthus,1,0,0,0
Carlina acaulis,0,0,0,1
Stachys germanica,134,18,5,14
Trifolium pratense,1,0,1,0
Trifolium repens,6,0,4,0
Oxytropis campestris,53,3,153,0
```

## Output format

By default `schoenerd` produces CSV formatted output as shown below:

```
FIRST SPECIES,SECOND SPECIES,D INDEX
Bombus pratorum,Bombus lapidarius,0.8300366300366301
Bombus pratorum,Bombus mesomelas,0.3321378008494573
Bombus pratorum,Bombus pascuorum,0.6871794871794872
Bombus lapidarius,Bombus mesomelas,0.17353198948290982
Bombus lapidarius,Bombus pascuorum,0.8571428571428571
Bombus mesomelas,Bombus pascuorum,0.030674846625766916
```

Each record (line) of the output data contains three fields described by the
CSV headers: first two fields contain the two pollinator species of which the
D index is reported in the third field.

### Pretty table

Using the `-p/--pretty-table` flag the output is formatted into a table for
quick data visualization. Using the same input above, the produced result is
the following:

```
┌───────────────────┬───────────────────┬──────────────────────┐
│ FIRST SPECIES     │ SECOND SPECIES    │ D INDEX              │
├───────────────────┼───────────────────┼──────────────────────┤
│ Bombus pratorum   │ Bombus lapidarius │ 0.8300366300366301   │
├───────────────────┼───────────────────┼──────────────────────┤
│ Bombus pratorum   │ Bombus mesomelas  │ 0.3321378008494573   │
├───────────────────┼───────────────────┼──────────────────────┤
│ Bombus pratorum   │ Bombus pascuorum  │ 0.6871794871794872   │
├───────────────────┼───────────────────┼──────────────────────┤
│ Bombus lapidarius │ Bombus mesomelas  │ 0.17353198948290982  │
├───────────────────┼───────────────────┼──────────────────────┤
│ Bombus lapidarius │ Bombus pascuorum  │ 0.8571428571428571   │
├───────────────────┼───────────────────┼──────────────────────┤
│ Bombus mesomelas  │ Bombus pascuorum  │ 0.030674846625766916 │
└───────────────────┴───────────────────┴──────────────────────┘
```

## Help

Output of `schoenerd -h` listed below:

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

This project is licensed under [GPLv3](LICENSE).

[^1]: Schoener, T.W. (1968) Anolis lizards of Bimini: resource parti tioning in
         a complex fauna. Ecology, 49, 704–726.
[^2]: `schoenerd` compiles on all platforms (Linux, Windows, macOs, ...) as
        long as `rust` and `just` are correctly installed, however installation
        recipes in the `justfile` are currently Linux only.
[^3]: [Install using `rustup`](https://www.rust-lang.org/tools/install).
[^4]: Currently `x86_64-linux-gnu` only.
[^5]: Currently `zsh`, `bash` & `fish`.
