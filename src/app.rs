use crate::{
    cli::Cli,
    data::InteractionData,
    input::Input,
    output::{Output, SortDirection},
    Result,
};

/// App instance.
pub struct App {
    /// Configured data reader.
    input: Input,
    /// Output data format.
    output: Output,
    /// Output sorting strategy (sort by D index value).
    sort: Option<SortDirection>,
}

impl App {
    /// Inizialize and construct a new [`App`] instance.
    pub fn new(cli: Cli) -> Result<Self> {
        let input = Input::new(
            cli.input,
            cli.input_field_delimiter,
            cli.input_record_terminator,
            cli.input_quote_character,
        )?;

        let output = match cli.pretty_table {
            true => Output::Table,
            false => Output::CSV {
                path: cli.output,
                delimiter: cli.output_field_delimiter,
                terminator: cli.output_record_terminator,
                quote: cli.output_quote_character,
            },
        };

        Ok(App {
            input,
            output,
            sort: cli.sort,
        })
    }

    /// Run the [`App`].
    pub fn run(self) -> Result<()> {
        let mut data = InteractionData::parse(self.input)?;
        data.normalize();

        let mut schoener_indexes = data.schoener_indexes();
        if let Some(direction) = self.sort {
            schoener_indexes.sort(direction);
        }

        self.output.finalize(schoener_indexes)
    }
}
