use crate::input;
use std::{error, fmt, io, path::PathBuf};

/// Top level errors.
#[derive(Debug)]
pub enum Error {
    /// Unable to access input file.
    InputFile { path: PathBuf, error: io::Error },
    /// Input file was not provided and no input was piped via stdin.
    MissingInput,
    /// Unable to parse headers from input data.
    ParseHeaders(csv::Error),
    /// Insufficient number of columns (pollinators) in input data:
    /// required at least two species of pollinators.
    InsufficientInputColumns { pollinators: Vec<String> },
    /// Unable to read input CSV record.
    ReadRecord(csv::Error),
    /// Unable to parse numeric input field as f64.
    InvalidFieldValue(input::ParseFieldError),
    /// Missing plant identifier in input record.
    MissingPlantIdent {
        /// Missing plant line number.
        line_nr: usize,
    },
    /// Duplicate plant in input record.
    DuplicatePlantIdent {
        /// Previous occurrence line number.
        prev_occ_line_nr: usize,
        /// Duplicate line number.
        line_nr: usize,
        /// Plant ident.
        plant: String,
    },
    /// Missing interaction (input) data.
    MissingData,
    /// Unable to create output file.
    OutputFile { path: PathBuf, error: io::Error },
    /// Unable to write output record.
    WriteRecord(csv::Error),
    /// Unable to write output data.
    OutputData(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InputFile { path, error } => {
                write!(f, "unable access input file '{}': {error}", path.display())
            }
            Error::MissingInput => {
                f.write_str("expected either input file or piped input from stdin")
            }
            Error::ParseHeaders(error) => write!(f, "unable to retrieve input headers: {error}"),
            Error::InsufficientInputColumns { pollinators } => {
                let field = pollinators.len();
                write!(
                    f,
                    "insufficient number of columns in input data: required \
                        at least two species of pollinators, found {field}"
                )?;

                if field > 0 {
                    write!(f, " ('{}')", pollinators[0])?;
                }

                Ok(())
            }
            Error::ReadRecord(error) => write!(f, "invalid input record: {error}"),
            Error::InvalidFieldValue(error) => error.fmt(f),
            Error::MissingPlantIdent { line_nr } => {
                write!(f, "missing plant identifier at line {line_nr}")
            }
            Error::DuplicatePlantIdent {
                line_nr: line,
                prev_occ_line_nr,
                plant,
            } => {
                write!(
                    f,
                    "duplicate plant ('{plant}') at line {line}, \
                        previous occurrence at line {prev_occ_line_nr}"
                )
            }
            Error::MissingData => f.write_str(
                "missing plant-pollinator interaction data, \
                    required at least one record",
            ),
            Error::OutputFile { path, error } => write!(
                f,
                "unable to create output file '{}': {error}",
                path.display()
            ),
            Error::WriteRecord(error) => write!(f, "unable to write output record: {error}"),
            Error::OutputData(error) => {
                write!(f, "unable to write output data: {error}")
            }
        }
    }
}

impl error::Error for Error {}

impl From<input::ParseFieldError> for Error {
    fn from(error: input::ParseFieldError) -> Self {
        Error::InvalidFieldValue(error)
    }
}
