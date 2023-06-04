mod app;
mod cli;
mod data;
mod error;
mod input;
mod output;

use app::App;
use cli::Cli;
use error::Error;
use std::process::ExitCode;

type Result<T> = std::result::Result<T, Error>;

#[inline(always)]
fn run() -> Result<()> {
    let cli = Cli::parse_args();
    App::new(cli)?.run()
}

fn main() -> ExitCode {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

// TODO: custom Result type implementing the Try traiit (nightly only) in order to use the ?
// operator on it and the Termination trait in order to produce error messages on process exit
// when the body of the main function returns an error.
// fn main() -> Result<()> {
//     let cli = Cli::parse_args();
//     App::new(cli)?.run()
// }
