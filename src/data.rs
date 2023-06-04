use crate::{error::Error, input::Input, output::SortDirection, Result};
use csv::{ByteRecord, Writer};
use prettytable::{Attr, Cell, Row, Table};
use std::io::Write;

/// Schoener D indexes.
#[derive(Debug)]
pub struct SchoenerIndexes<'a>(Vec<SchoenerD<'a>>);

impl<'a> SchoenerIndexes<'a> {
    /// Constructs a new [`SchoenerIndexes`] instance with given capacity.
    fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    /// Adds a new [`SchoenerD`].
    fn add_index(&mut self, firs_species: &'a str, second_species: &'a str, index: f64) {
        self.0
            .push(SchoenerD::new(firs_species, second_species, index))
    }

    /// Sorts [`SchoenerIndexes`] entries by D index value by given sorting direction.
    pub fn sort(&mut self, direction: SortDirection) {
        match direction {
            SortDirection::Ascending => self.0.sort_by(|a, b| a.index.total_cmp(&b.index)),
            SortDirection::Descending => self.0.sort_by(|a, b| b.index.total_cmp(&a.index)),
        }
    }

    /// Consumes the [`SchoenerIndexes`] producing a CSV.
    pub fn output_csv<W: Write>(self, mut writer: Writer<W>) -> Result<()> {
        for index in self.0 {
            writer.write_byte_record(&index.to_byte_record()).unwrap();
        }
        writer.flush().map_err(Error::OutputData)
    }

    /// Consumes the [`SchoenerIndexes`] producing a pretty table on stdout.
    pub fn output_table(self, mut table: Table) -> Result<()> {
        for index in self.0 {
            table.add_row(index.to_table_row());
        }
        table.print_tty(false).map_err(Error::OutputData)?;

        Ok(())
    }
}

/// Schoener's D index.
#[derive(Debug)]
pub struct SchoenerD<'a> {
    first_species: &'a str,
    second_species: &'a str,
    index: f64,
}

impl<'a> SchoenerD<'a> {
    /// Constructs a new [`SchoenerD`].
    #[inline(always)]
    fn new(first_species: &'a str, second_species: &'a str, index: f64) -> Self {
        Self {
            first_species,
            second_species,
            index,
        }
    }

    /// Returns a [`ByteRecord`] CSV data record.
    #[inline]
    pub fn to_byte_record(self) -> ByteRecord {
        let mut record = ByteRecord::new();
        record.push_field(self.first_species.as_bytes());
        record.push_field(self.second_species.as_bytes());
        record.push_field(self.index.to_string().as_bytes());
        record
    }

    /// Returns a table [`Row`] data.
    #[inline]
    pub fn to_table_row(self) -> Row {
        let mut row = Row::empty();
        row.add_cell(Cell::new(self.first_species).with_style(Attr::Italic(true)));
        row.add_cell(Cell::new(self.second_species).with_style(Attr::Italic(true)));
        row.add_cell(Cell::new(&self.index.to_string()).with_style(Attr::Blink));
        row
    }
}

/// Interaction data.
#[derive(Debug)]
pub struct InteractionData {
    /// Pollinator species names
    /// (its length represents the number of columns of the `interactions` matrix).
    pollinators: Vec<String>,
    /// Plant species names
    /// (its length represents the number of rows of the `interactions` matrix).
    plants: Vec<String>,
    /// Interaction matrix.
    interactions: Vec<f64>,
}

impl InteractionData {
    /// Parses [`InteractionData`] from [`Input`].
    pub fn parse(mut input: Input) -> Result<Self> {
        let pollinators = input.pollinators()?;
        let (plants, interactions) = input.plants_interactions()?;

        Ok(Self {
            pollinators,
            plants,
            interactions,
        })
    }

    /// Normalize [`InteractionData`] as relative frequencies per species
    /// (column totals).
    pub fn normalize(&mut self) {
        let cols = self.pollinators.len();

        let factors = (0..cols)
            .map(|idx| {
                self.interactions
                    .iter()
                    .skip(idx)
                    .step_by(cols)
                    .sum::<f64>()
            })
            .collect::<Vec<f64>>();

        for (idx, value) in self.interactions.iter_mut().enumerate() {
            *value /= factors[idx % cols];
        }
    }

    /// Calculates the Schoener's D index, consuming [`InteractionData`].
    pub fn schoener_indexes(&self) -> SchoenerIndexes {
        let cols = self.pollinators.len();
        let rows = self.plants.len();

        let mut schoener_indexes = SchoenerIndexes::with_capacity(rows * cols / 2); // FIXME: this capacity is wrong
        for i in 0..cols {
            for j in i + 1..cols {
                let sum = (0..rows)
                    .map(|row| {
                        let k = row * cols;
                        (self.interactions[i + k] - self.interactions[j + k]).abs()
                    })
                    .sum::<f64>();

                schoener_indexes.add_index(
                    &self.pollinators[i],
                    &self.pollinators[j],
                    1.0 - 0.5 * sum,
                );
            }
        }

        schoener_indexes
    }
}
