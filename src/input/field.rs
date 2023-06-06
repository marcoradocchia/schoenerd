use std::{error, fmt, num};

trait IsInteger {
    fn is_integer(&self) -> bool;
}

impl IsInteger for f64 {
    /// Checks wheter the `f64` represents an integer number.
    fn is_integer(&self) -> bool {
        self.trunc() == *self
    }
}

/// [`ParseFieldError`] kinds.
#[derive(Debug)]
pub enum ParseFieldErrorKind {
    /// Unable to parse f64.
    ParseFloat(num::ParseFloatError),
    /// Field value is a negative number.
    IsNegative,
    /// Field value does not represent an integer number.
    NotAnInteger,
}

impl fmt::Display for ParseFieldErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseFieldErrorKind::ParseFloat(error) => error.fmt(f),
            ParseFieldErrorKind::IsNegative => f.write_str("value must be a positive number"),
            ParseFieldErrorKind::NotAnInteger => f.write_str("value must be positive"),
        }
    }
}

/// Errors that may occur while parsing interaction field data.
#[derive(Debug)]
pub struct ParseFieldError {
    pub line_nr: usize,
    pub field_nr: usize,
    pub field_value: String,
    pub kind: ParseFieldErrorKind,
}

impl fmt::Display for ParseFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "unable to parse input field value '{field_value}' \
                (line {line_nr}, field {field_nr}): {kind}",
            field_value = self.field_value,
            line_nr = self.line_nr,
            field_nr = self.field_nr,
            kind = self.kind
        )
    }
}

impl error::Error for ParseFieldError {}

/// Interaction field.
pub struct InteractionField;

impl InteractionField {
    pub fn parse(field_value: &str) -> Result<f64, ParseFieldErrorKind> {
        let interaction = field_value
            .parse::<f64>()
            .map_err(ParseFieldErrorKind::ParseFloat)?;

        if interaction.is_sign_negative() {
            return Err(ParseFieldErrorKind::IsNegative);
        }

        if !interaction.is_integer() {
            return Err(ParseFieldErrorKind::NotAnInteger);
        }

        Ok(interaction)
    }
}
