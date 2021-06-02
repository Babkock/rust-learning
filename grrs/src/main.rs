use structopt::StructOpt;
use anyhow::{Context, Result};

pub fn main() -> Result<()> {
	let args = grrs::Cli::from_args();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    grrs::find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}
