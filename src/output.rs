use crate::{data::SchoenerIndexes, Error, Result};
use clap::ValueEnum;
use csv::{Terminator, WriterBuilder};
use prettytable::{format, Attr, Cell, Row, Table};
use std::{fs::OpenOptions, io, path::PathBuf};

/// Output D index value sort direction.
#[derive(Debug, ValueEnum, Clone)]
pub enum SortDirection {
    Descending,
    Ascending,
}

/// Output dafa format.
pub enum Output {
    /// CSV output format (terminal or file).
    CSV {
        path: Option<PathBuf>,
        delimiter: Option<char>,
        terminator: Option<char>,
        quote: Option<char>,
    },
    /// Terminal pretty table output.
    Table,
}

impl Output {
    /// Finalizes [`Output`].
    pub fn finalize(self, schoener_indexes: SchoenerIndexes) -> Result<()> {
        match self {
            Output::CSV {
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
                            .map_err(|error| Error::OutputFile { path, error })?;

                        schoener_indexes.output_csv(builder.from_writer(file))
                    }
                    None => schoener_indexes.output_csv(builder.from_writer(io::stdout())),
                }
            }
            Output::Table => {
                let mut table = Table::new();
                let mut headers = Row::empty();
                headers.add_cell(Cell::new("First Species").with_style(Attr::Bold));
                headers.add_cell(Cell::new("Second Species").with_style(Attr::Bold));
                headers.add_cell(Cell::new("D index").with_style(Attr::Bold));

                table.set_titles(headers);
                table.set_format(
                    format::FormatBuilder::new()
                        .column_separator('│')
                        .borders('│')
                        .separators(
                            &[format::LinePosition::Top],
                            format::LineSeparator::new('─', '┬', '┌', '┐'),
                        )
                        .separators(
                            &[format::LinePosition::Intern],
                            format::LineSeparator::new('─', '┼', '├', '┤'),
                        )
                        .separators(
                            &[format::LinePosition::Bottom],
                            format::LineSeparator::new('─', '┴', '└', '┘'),
                        )
                        .padding(1, 1)
                        .build(),
                );

                schoener_indexes.output_table(table)
            }
        }
    }
}
