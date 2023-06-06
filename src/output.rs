use crate::{cli::SortDirection, data::SchoenerIndexes, Error, Result};
use csv::{Terminator, WriterBuilder};
use prettytable::{
    format::{FormatBuilder, LinePosition, LineSeparator},
    Table,
};
use std::{fs::OpenOptions, io, path::PathBuf};

/// Output headers.
pub const OUTPUT_HEADERS: [&str; 3] = ["FIRST SPECIES", "SECOND SPECIES", "D INDEX"];

/// [`Output`] dafa format.
pub enum OutputFormat {
    /// CSV output format (terminal or file).
    Csv {
        path: Option<PathBuf>,
        delimiter: Option<char>,
        terminator: Option<char>,
        quote: Option<char>,
    },
    /// Terminal pretty table output.
    Table,
}

/// Configured output.
pub struct Output {
    headers: bool,
    sort: Option<SortDirection>,
    format: OutputFormat,
}

impl Output {
    /// Constructs a new [`Output`] instance.
    pub fn new(headers: bool, sort: Option<SortDirection>, format: OutputFormat) -> Self {
        Output {
            headers,
            sort,
            format,
        }
    }

    /// Finalizes [`Output`].
    pub fn finalize(self, mut schoener_indexes: SchoenerIndexes) -> Result<()> {
        if let Some(direction) = self.sort {
            schoener_indexes.sort(direction);
        }

        match self.format {
            OutputFormat::Csv {
                path,
                delimiter,
                terminator,
                quote,
            } => {
                let mut builder = WriterBuilder::new();

                if let Some(delimiter) = delimiter {
                    builder.delimiter(delimiter as u8);
                }

                if let Some(terminator) = terminator {
                    builder.terminator(Terminator::Any(terminator as u8));
                }

                if let Some(quote) = quote {
                    builder.quote(quote as u8);
                }

                match path {
                    Some(path) => {
                        let file = OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(&path)
                            .map_err(|error| Error::OutputFile {
                                path: path.clone(),
                                error,
                            })?;

                        let writer = builder.from_writer(file);
                        schoener_indexes.output_csv(writer, self.headers)?;

                        Ok(())
                    }
                    None => {
                        let writer = builder.from_writer(io::stdout());
                        schoener_indexes.output_csv(writer, self.headers)?;

                        Ok(())
                    }
                }
            }
            OutputFormat::Table => {
                let mut table = Table::new();
                let format = FormatBuilder::new()
                    .column_separator('│')
                    .borders('│')
                    .separators(&[LinePosition::Top], LineSeparator::new('─', '┬', '┌', '┐'))
                    .separators(
                        &[LinePosition::Intern],
                        LineSeparator::new('─', '┼', '├', '┤'),
                    )
                    .separators(
                        &[LinePosition::Bottom],
                        LineSeparator::new('─', '┴', '└', '┘'),
                    )
                    .padding(1, 1)
                    .build();

                table.set_format(format);
                schoener_indexes.output_table(table, self.headers)?;

                Ok(())
            }
        }
    }
}
