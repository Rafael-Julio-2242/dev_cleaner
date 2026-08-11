use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

use crate::default_matches::DEFAULT_MATCHES;

#[derive(Deserialize)]
pub struct Settings {
    pub recursive: bool,
    pub dry_run: bool,
    pub display_path: bool,
}

#[derive(Deserialize)]
pub struct Targets {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

#[derive(Deserialize)]
pub struct CleanConfig {
    pub settings: Settings,
    pub targets: Targets,
}

impl CleanConfig {
    pub fn default() -> CleanConfig {
        CleanConfig {
            settings: Settings {
                dry_run: false,
                recursive: true,
                display_path: false,
            },
            targets: Targets {
                exclude: vec![],
                include: DEFAULT_MATCHES.map(|v| { v.to_string() }).to_vec(),
            },
        }
    }

    pub fn configure(&mut self, folder_path: PathBuf, c_file_name: &str) -> Result<()> {
        let config_path = folder_path.join(c_file_name);

        let file_clean_config: Option<CleanConfig> = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: CleanConfig = toml::from_str(&content)?;
            Some(config)
        } else {
            None
        };

        if let Some(f_config) = file_clean_config {
            self.settings = f_config.settings;

            if f_config.targets.include.len() > 0 {
                self.targets.include = f_config.targets.include
            }

            if f_config.targets.exclude.len() > 0 {
                self.targets.exclude = f_config.targets.exclude
            }
        }

        Ok(())
    }
}
