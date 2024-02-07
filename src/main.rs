use anyhow::Result;
use cli::cli;

mod cli;
mod projects_file;

fn main() -> Result<()> {
    cli()
}
