use std::{cmp::Reverse, env::VarError, path::PathBuf, time::SystemTime};

use color_eyre::eyre::{OptionExt, Result, WrapErr};

fn find_recent_downloads(dl_dir: PathBuf) -> color_eyre::Result<Vec<(SystemTime, PathBuf)>> {
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

fn find_downloads_dir(home: PathBuf) -> color_eyre::Result<PathBuf> {
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

pub fn recent_downloads() -> Result<Vec<PathBuf>> {
    let home = home::home_dir().ok_or_eyre("find home directory")?;
    let dl_dir = find_downloads_dir(home).wrap_err("find downloads directory")?;
    let recent_downloads = find_recent_downloads(dl_dir).wrap_err("find recent downloads")?;

    Ok(recent_downloads.into_iter().map(|i| i.1).collect())
}
