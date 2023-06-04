use crate::{error::Error, Result};
use csv::{Reader, ReaderBuilder, Terminator, Trim};
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
        Ok(self
            .reader
            .headers()
            .map_err(Error::InputHeaders)?
            .into_iter()
            .skip(1)
            .map(String::from)
            .collect::<Vec<String>>())
    }

    /// Returns a vector of plants species names and interaction data.
    pub fn plants_interactions(&mut self) -> Result<(Vec<String>, Vec<f64>)> {
        let mut plants = Vec::new();
        let mut interactions = Vec::new();

        for (line, record) in self.reader.records().enumerate() {
            let record = record.map_err(Error::Record)?;
            let mut fields = record.into_iter();

            let plant = fields
                .next()
                .ok_or(Error::PlantFieldMissing { line: line + 2 })?
                .to_string();

            if plants.contains(&plant) {
                return Err(Error::DuplicatePlant {
                    line: line + 2,
                    plant,
                });
            }

            plants.push(plant);

            for interaction in fields {
                let value =
                    interaction
                        .parse::<f64>()
                        .map_err(|error| Error::InvalidFieldValue {
                            value: interaction.to_string(),
                            error,
                        })?;

                interactions.push(value);
            }
        }

        Ok((plants, interactions))
    }
}
