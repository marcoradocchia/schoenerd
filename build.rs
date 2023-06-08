use clap::CommandFactory;
use clap_complete::{self, shells::Shell};
use clap_mangen::Man;
use std::{env, error, fmt, fs, io, process::ExitCode};

include!("src/cli.rs");

const BINARY_NAME: &str = "schoenerd";
const MAN_FILENAME: &str = "schoenerd.1";
const SUPPORTED_SHELLS: [Shell; 3] = [Shell::Bash, Shell::Fish, Shell::Zsh];

#[derive(Debug)]
enum BuildError {
    OutDirVar,
    CreateDir { path: PathBuf, error: io::Error },
    CompletionGen { shell: Shell, error: io::Error },
    ManualRender(io::Error),
    WriteManualFile { path: PathBuf, error: io::Error },
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::OutDirVar => f.write_str("OUT_DIR environment variable not set"),
            BuildError::CreateDir { path, error } => write!(
                f,
                "unable to create '{path}' directory: {error}",
                path = path.display()
            ),
            BuildError::CompletionGen { shell, error } => {
                write!(f, "unable to generate {shell} shell completions: {error}")
            }
            BuildError::ManualRender(error) => {
                write!(f, "unable to render man page: {error}")
            }
            BuildError::WriteManualFile { path, error } => write!(
                f,
                "unable to write man file to '{path}': {error}",
                path = path.display()
            ),
        }
    }
}

impl error::Error for BuildError {}

fn build() -> Result<(), BuildError> {
    let out_dir = match env::var_os("OUT_DIR") {
        Some(path) => PathBuf::from(path),
        None => return Err(BuildError::OutDirVar),
    };

    // ---- COMPLETIONS ----
    let comp_path = out_dir.join("completions");

    fs::create_dir_all(&comp_path).map_err(|error| BuildError::CreateDir {
        path: comp_path.clone(),
        error,
    })?;

    let mut cmd = Cli::command();

    for shell in SUPPORTED_SHELLS {
        let comp_path = clap_complete::generate_to(shell, &mut cmd, BINARY_NAME, &comp_path)
            .map_err(|error| BuildError::CompletionGen { shell, error })?;

        println!(
            "cargo:warning=completions file for {shell} shell generated at {path}",
            path = comp_path.display()
        );
    }

    // ---- MANUAL ----
    let mut man_path = out_dir.join("man");
    fs::create_dir_all(&man_path).map_err(|error| BuildError::CreateDir {
        path: man_path.clone(),
        error,
    })?;

    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer).map_err(BuildError::ManualRender)?;

    man_path.push(MAN_FILENAME);
    fs::write(&man_path, buffer).map_err(|error| BuildError::WriteManualFile {
        path: man_path.clone(),
        error,
    })?;

    println!(
        "cargo:warning=manual file generated at {path}",
        path = man_path.display()
    );

    Ok(())
}

fn main() -> ExitCode {
    if let Err(error) = build() {
        eprintln!("error: {error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}
