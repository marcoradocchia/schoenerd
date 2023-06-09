use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// CLI help template.
const HELP_TEMPLATE: &str = r#"
{name} v{version} - {author}
{about}

{usage-heading}
{tab}{usage}

{all-args}"#;

/// Output D index value sort direction.
#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum SortDirection {
    Descending,
    Ascending,
}

/// Schoener's D index calculator for niche overlap.
#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
    help_template = HELP_TEMPLATE,
)]
pub struct Cli {
    /// Input CSV file path.
    #[arg(short, long, value_name = "FILE")]
    pub input: Option<PathBuf>,
    /// Input CSV field delimiter.
    #[arg(short = 'f', long, value_name = "CHAR")]
    pub input_field_delimiter: Option<char>,
    /// Input CSV record terminator.
    #[arg(short = 't', long, value_name = "CHAR")]
    pub input_record_terminator: Option<char>,
    /// Input CSV quote character.
    #[arg(short = 'c', long, value_name = "CHAR")]
    pub input_quote_character: Option<char>,

    /// Output CSV file path.
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,
    /// Output CSV field delimiter.
    #[arg(short = 'F', long, value_name = "CHAR")]
    pub output_field_delimiter: Option<char>,
    /// Output CSV record terminator.
    #[arg(short = 'T', long, value_name = "CHAR")]
    pub output_record_terminator: Option<char>,
    /// Output CSV quote character.
    #[arg(short = 'C', long, value_name = "CHAR")]
    pub output_quote_character: Option<char>,

    /// Disable output headers.
    #[arg(short = 'n', long)]
    pub disable_output_headers: bool,
    /// Sort output by D index value.
    #[arg(short, long, value_enum, value_name = "DIRECTION")]
    pub sort: Option<SortDirection>,
    /// Display output as a pretty table on stdout.
    #[arg(
        short,
        long,
        conflicts_with_all = [
            "output",
            "output_field_delimiter",
            "output_record_terminator",
            "output_quote_character"
        ]
    )]
    pub pretty_table: bool,
}

impl Cli {
    /// Parse CLI arguments/option/flags.
    pub fn parse_args() -> Self {
        Cli::parse()
    }
}
