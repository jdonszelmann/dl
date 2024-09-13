use clap::Parser;
use color_eyre::Result;
use find_dl::recent_downloads;

mod shared;

/// Get the most recent downloads
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Optional, how many downloads to get
    #[arg(default_value_t = 1)]
    number: usize,
}


fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    for path in recent_downloads()?.into_iter().take(args.number) {
        println!("{}", path.display())
    }

    Ok(())
}
