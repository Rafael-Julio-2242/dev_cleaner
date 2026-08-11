mod clean_config;
mod default_matches;

use anyhow::Result;
use bytesize::ByteSize;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::{path::{Path, PathBuf}, time::Duration};
use walkdir::{DirEntry, WalkDir};

use crate::clean_config::CleanConfig;

pub const CONFIG_FILE_NAME: &'static str = "clean.toml";


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Folder Path.
    path: PathBuf,

    /// Run without erasing real files.
    #[arg(short, long)]
    dry_run: bool,

    /// Run on recursive mode. Default = true
    #[arg(short, long)]
    recursive: bool,

    /// Display the Path of each found Folder / File that is matched to be deleted
    #[arg(short = 'p', long)]
    display_path: bool,
    
}

enum MatchedPath {
    File { entry: DirEntry, size: u64 },
    Folder { entry: DirEntry, size: u64 }
}

impl MatchedPath {
    pub fn size(&self) -> u64 {
        match self {
            Self::File { size, .. } | Self::Folder { size , ..} => *size
        }
    }

    pub fn erase(&self, pb: &mut ProgressBar) -> Result<()> {
        match self {
            Self::File { entry, size } => {
                std::fs::remove_file(entry.path())?;
                pb.inc(*size);
            },
            Self::Folder { entry, .. } => {
                let files: Vec<(PathBuf, u64)> = WalkDir::new(entry.path())
                    .min_depth(1)
                    .into_iter()
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_file())
                    .map(|e| {
                        let size = e.metadata().map(|m| m.len())
                            .unwrap_or(0);
                        (e.into_path(), size)
                    })
                    .collect();

                for (path, size) in files {
                    std::fs::remove_file(&path)?;
                    pb.inc(size);
                }

                std::fs::remove_dir_all(entry.path())?;
            }
            
        }

        Ok(())
    }

    pub fn get_path(&self) -> &Path {
        match self {
            Self::File { entry, .. } => entry.path(),
            Self::Folder { entry, .. } => entry.path(),
        }
    }

    pub fn display_path(&self) {
        match self {
            Self::File { entry, .. } => println!("Matched Path: {} {}", entry.path().display(), ByteSize(self.size())),
            Self::Folder { entry, .. } => println!("Matched Path: {} {}", entry.path().display(), ByteSize(self.size())),            
        }
    }
}


fn dir_size(path: &PathBuf) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}

fn validate_match(path: &Path, match_paths: &Vec<String>, not_match_paths: &Vec<String>) -> bool {
    let segments: Vec<&str> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str())
        .collect();
    
    
    for n_m_path in not_match_paths {
        if segments.iter().any(|s| s == n_m_path || s.ends_with(n_m_path)) {
            return false;
        }
    }

    for m_path in match_paths {
        if segments.iter().any(|s| s == m_path || s.ends_with(m_path)) {
            return true;
        }
    }
    false
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = args.path;

    let mut matched_paths: Vec<MatchedPath> = vec![];

    let mut config = CleanConfig::default();

    
    config.configure(path.clone(), CONFIG_FILE_NAME)?;

    if args.dry_run {
        config.settings.dry_run = args.dry_run;
    }
    
    if args.recursive {
        config.settings.recursive = args.recursive;
    }

    if args.display_path {
        config.settings.display_path = args.display_path;
    }

    let mut walker = WalkDir::new(path);

    if !config.settings.recursive {
        walker = walker.min_depth(1).max_depth(1);
    }

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Scanning...");
    spinner.enable_steady_tick(Duration::from_millis(100));

    for entry_res in walker {
        let entry = entry_res?;
        let path_str = match entry.path().to_str() {
            Some(v) => v,
            None => continue
        };

        if path_str.contains(CONFIG_FILE_NAME) {
            continue;
        }

        let matched = validate_match(entry.path(), &config.targets.include, &config.targets.exclude);

        if matched {
            
            let mut already_matched = false;

            // Validar se já foi inserido.

            for mp in &matched_paths {
                let mp_path: &Path = mp.get_path();


                match mp_path.to_str() {
                    Some(v) => {
                        if path_str.contains(v) {
                            already_matched = true;
                            break;
                        }
                    },
                    None => continue
                }

            } 

            if already_matched { continue; }

            if entry.path().is_file() {
                matched_paths.push(MatchedPath::File {
                    entry: entry.clone(),
                    size: dir_size(&entry.clone().into_path()),
                });
            } else {
                matched_paths.push(MatchedPath::Folder {
                    entry: entry.clone(),
                    size: dir_size(&entry.clone().into_path()),
                });
            }

        }
    }

    spinner.finish_with_message("Scanning complete.");

    let mut full_erase_size: u64 = 0;


    matched_paths.iter().for_each(|mp| {
        if config.settings.display_path {
            mp.display_path();
        }

        full_erase_size += mp.size();
    });

    if full_erase_size == 0 {
        println!("Nothing to erase.");
        println!("Exiting.....");
        return Ok(())
    }

    println!("AMOUNT OF SIZE TO CLEAR - {}", ByteSize(full_erase_size));

    if config.settings.dry_run {
        return Ok(());
    }

    println!("Start Cleaning Process...");
    
    let mut pb = ProgressBar::new(full_erase_size);
    
    pb.set_style(
        ProgressStyle::with_template("{bar:40} {bytes}/{total_bytes}")?
    );
    
    for mp in matched_paths.iter() {
        mp.erase(&mut pb)?;
    }
    
    pb.finish_with_message("Process finished.");
    
    Ok(())
}
