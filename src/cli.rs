use crate::output::SortDirection;
use clap::Parser;
use std::path::PathBuf;

/// CLI help template.
const HELP_TEMPLATE: &str = r#"
{name} v{version} - {author}
{about}

{usage-heading}
{tab}{usage}

{all-args}"#;

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
    #[arg(
        short,
        long,
        value_name = "FILE",
        long_help = "Input must be must be presented in CSV format with the \
         columns being the pollinators and the rows representing the plants."
    )]
    pub input: Option<PathBuf>,
    /// Input CSV field delimiter.
    #[arg(short = 'f', long, value_name = "CHAR")]
    pub input_field_delimiter: Option<char>,
    /// Input CSV record (line) terminator.
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
    /// Output CSV record (line) terminator.
    #[arg(short = 'T', long, value_name = "CHAR")]
    pub output_record_terminator: Option<char>,
    /// Output CSV quote character.
    #[arg(short = 'C', long, value_name = "CHAR")]
    pub output_quote_character: Option<char>,

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
