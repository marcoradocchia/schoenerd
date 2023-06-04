use std::{error, fmt, io, num::ParseFloatError, path::PathBuf};

/// Top level errors.
#[derive(Debug)]
pub enum Error {
    /// Unable to access input file.
    InputFile { path: PathBuf, error: io::Error },
    /// Input file was not provided and no input was piped via stdin.
    MissingInput,
    /// Unable to read input headers.
    InputHeaders(csv::Error),
    /// Unable to read input record.
    Record(csv::Error),
    /// Unable to parse numeric input field as f64.
    InvalidFieldValue {
        value: String,
        error: ParseFloatError,
    },
    /// Duplicate plant in input record.
    DuplicatePlant { line: usize, plant: String },
    /// Plant field is missing.
    PlantFieldMissing { line: usize },
    /// Unable to create output file.
    OutputFile { path: PathBuf, error: io::Error },
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
            Error::InputHeaders(error) => write!(f, "unable to retrieve input headers: {error}"),
            Error::Record(error) => write!(f, "unable to retrieve input record: {error}"),
            Error::InvalidFieldValue { value, error } => write!(
                f,
                "unable to parse input field value '{value}' as `f64`: {error}"
            ),
            Error::DuplicatePlant { line, plant } => {
                write!(f, "duplicate input plant record at line {line}: '{plant}'")
            }
            Error::PlantFieldMissing { line } => {
                write!(f, "plant field is missing for record at line {line}")
            }
            Error::OutputFile { path, error } => write!(
                f,
                "unable to create output file '{}': {error}",
                path.display()
            ),
            Error::OutputData(error) => {
                write!(f, "unable to write output data: {error}")
            }
        }
    }
}

impl error::Error for Error {}
