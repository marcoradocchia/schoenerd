use crate::{
    cli::Cli,
    data::InteractionData,
    input::Input,
    output::{Output, OutputFormat},
    Result,
};

/// App instance.
pub struct App {
    /// Configured data reader.
    input: Input,
    /// Output data format.
    output: Output,
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

        let output = Output::new(
            !cli.disable_output_headers,
            cli.sort,
            match cli.pretty_table {
                true => OutputFormat::Table,
                false => OutputFormat::Csv {
                    path: cli.output,
                    delimiter: cli.output_field_delimiter,
                    terminator: cli.output_record_terminator,
                    quote: cli.output_quote_character,
                },
            },
        );

        Ok(App { input, output })
    }

    /// Run the [`App`].
    pub fn run(self) -> Result<()> {
        let mut data = InteractionData::parse(self.input)?;
        data.normalize();

        let schoener_indexes = data.schoener_indexes();
        self.output.finalize(schoener_indexes)
    }
}
