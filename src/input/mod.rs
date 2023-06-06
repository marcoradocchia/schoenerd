mod field;

use crate::{error::Error, Result};
use csv::{Reader, ReaderBuilder, Terminator, Trim};
pub use field::{InteractionField, ParseFieldError, ParseFieldErrorKind};
use std::{
    fs::OpenOptions,
    io::{self, IsTerminal, Read},
    path::PathBuf,
};

/// [`Input`] builder.
pub struct InputBuilder {
    builder: ReaderBuilder,
}

impl InputBuilder {
    /// Constructs a new [`InputBuilder`].
    fn new(reader_builder: ReaderBuilder) -> InputBuilder {
        InputBuilder {
            builder: reader_builder,
        }
    }

    /// Builds a new [`Input`] from file.
    fn file(self, path: PathBuf) -> Result<Input> {
        let file = OpenOptions::new()
            .read(true)
            .open(&path)
            .map_err(|error| Error::InputFile { path, error })?;

        Ok(Input {
            reader: self.builder.from_reader(Box::new(file)),
        })
    }

    /// Builds a new [`Input`] from stdin.
    fn stdin(self) -> Result<Input> {
        let stdin = io::stdin();
        if stdin.is_terminal() {
            return Err(Error::MissingInput);
        }

        Ok(Input {
            reader: self.builder.from_reader(Box::new(stdin)),
        })
    }
}

/// Configured input data reader.
pub struct Input {
    reader: Reader<Box<dyn Read>>,
}

impl Input {
    /// Constructs a new [`Input`] instance.
    pub fn new(
        input: Option<PathBuf>,
        delimiter: Option<char>,
        terminator: Option<char>,
        quote: Option<char>,
    ) -> Result<Input> {
        let mut builder = ReaderBuilder::new();
        builder.trim(Trim::All);
        builder.has_headers(true);

        if let Some(delimiter) = delimiter {
            builder.delimiter(delimiter as u8);
        }

        if let Some(terminator) = terminator {
            builder.terminator(Terminator::Any(terminator as u8));
        }

        if let Some(quote) = quote {
            builder.quote(quote as u8);
        }

        let builder = InputBuilder::new(builder);
        match input {
            Some(path) => builder.file(path),
            None => builder.stdin(),
        }
    }

    /// Returns a vector of pollinators species names.
    pub fn pollinators(&mut self) -> Result<Vec<String>> {
        let pollinators = self
            .reader
            .headers()
            .map_err(Error::ParseHeaders)?
            .into_iter()
            .skip(1)
            .map(String::from)
            .collect::<Vec<String>>();

        if pollinators.len() < 2 {
            return Err(Error::InsufficientInputColumns { pollinators });
        }

        Ok(pollinators)
    }

    /// Returns vectors of plants species names and interaction data respectively.
    pub fn plants_interactions(&mut self) -> Result<(Vec<String>, Vec<f64>)> {
        let mut plants = Vec::<String>::new();
        let mut interactions = Vec::new();

        for (line_idx, record) in self.reader.records().enumerate() {
            let record = record.map_err(Error::ReadRecord)?;
            let mut fields = record.into_iter();

            let plant = match fields.next() {
                Some(plant) => plant.to_string(),
                None => continue,
            };

            if plant.is_empty() {
                return Err(Error::MissingPlantIdent {
                    line_nr: line_idx + 2,
                });
            }

            // WARNING: worst case is pretty slow for large number of records.
            // Also valid input will always cause the iterator reach the end, so valid input is worst case.
            // Maybe use HashSet instad of vector and later convert it?
            if let Some(prev_occ_line) = plants.iter().position(|element| element == &plant) {
                return Err(Error::DuplicatePlantIdent {
                    prev_occ_line_nr: prev_occ_line + 2,
                    line_nr: line_idx + 2,
                    plant,
                });
            }

            plants.push(plant);

            for (field_idx, field_value) in fields.enumerate() {
                let interaction =
                    InteractionField::parse(field_value).map_err(|kind| ParseFieldError {
                        field_nr: field_idx + 2,
                        line_nr: line_idx + 2,
                        field_value: field_value.to_string(),
                        kind,
                    })?;

                interactions.push(interaction);
            }
        }

        if plants.is_empty() || interactions.is_empty() {
            return Err(Error::MissingData);
        }

        Ok((plants, interactions))
    }
}
