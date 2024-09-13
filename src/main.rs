use clap::Parser;
use color_eyre::eyre::{ContextCompat, Result, WrapErr};
use std::cmp::Reverse;
use std::env::VarError;
use std::path::PathBuf;
use std::time::SystemTime;

/// Get the most recent downloads
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Optional, how many downloads to get
    #[arg(default_value_t = 1)]
    number: usize,
}

pub fn find_most_recent_download(
    dl_dir: PathBuf,
) -> color_eyre::Result<Vec<(SystemTime, PathBuf)>> {
    let mut downloads: Vec<(SystemTime, PathBuf)> = Vec::new();

    for i in std::fs::read_dir(dl_dir)? {
        let i = i?;
        let Ok(meta) = i.metadata() else {
            eprintln!(
                "couldn't read file metadata of {}; skipping",
                i.path().display()
            );
            continue;
        };

        if meta.is_file() {
            let Ok(created) = meta.created() else {
                eprintln!(
                    "couldn't read creation time of {}; skipping",
                    i.path().display()
                );
                continue;
            };

            downloads.push((created, i.path()))
        }
    }

    downloads.sort_by_key(|i| Reverse(i.0));

    Ok(downloads)
}

pub fn find_downloads_dir(home: PathBuf) -> color_eyre::Result<PathBuf> {
    let mut fallback_dl_dir = home.join("Downloads");
    if !fallback_dl_dir.exists() {
        fallback_dl_dir = home.join("dl");
    }
    let dl_dir = match std::env::var("XDG_DOWNLOAD_DIR") {
        Ok(i) => PathBuf::from(i),
        Err(VarError::NotPresent) => fallback_dl_dir,
        Err(e) => {
            return Err(e.into());
        }
    };
    Ok(dl_dir)
}
fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Cli::parse();

    let home = home::home_dir().wrap_err("find home directory")?;
    let dl_dir = find_downloads_dir(home).wrap_err("find downloads directory")?;
    let recent_downloads = find_most_recent_download(dl_dir).wrap_err("find recent downloads")?;

    for (_time, path) in recent_downloads.into_iter().take(args.number) {
        println!("{}", path.display())
    }

    Ok(())
}
