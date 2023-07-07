use std::path::PathBuf;

use anyhow::{bail, Result};
use astra_core::{AstraProject, PathLocalizer};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use crate::Theme;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AppConfig {
    pub projects: Vec<ProjectDef>,
    #[serde(default)]
    pub active_project: Option<usize>,
    #[serde(default)]
    pub theme: Theme,
    #[serde(default)]
    pub script_editor_process: String,
    #[serde(default)]
    pub script_editor_command_args: String,
}

impl AppConfig {
    pub fn get_active_project(&self) -> Option<&ProjectDef> {
        self.active_project
            .and_then(|index| self.projects.get(index))
    }

    pub fn load() -> Result<Self> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "thane98", "astra") {
            let config_path = proj_dirs.config_dir().join("config.yml");
            if !config_path.exists() {
                Ok(Self::default())
            } else {
                let config_contents = std::fs::read_to_string(&config_path)?;
                Ok(serde_yaml::from_str(&config_contents)?)
            }
        } else {
            bail!("unable to determine a home directory")
        }
    }

    pub fn save(&self) -> Result<()> {
        if let Some(proj_dirs) = ProjectDirs::from("com", "thane98", "astra") {
            let config_dir_path = proj_dirs.config_dir();
            if !config_dir_path.exists() {
                std::fs::create_dir_all(config_dir_path)?;
            }
            std::fs::write(
                config_dir_path.join("config.yml"),
                serde_yaml::to_string(self)?,
            )?;
            Ok(())
        } else {
            bail!("unable to determine a home directory")
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProjectDef {
    pub name: String,
    pub romfs_path: String,
    pub output_mode: ProjectOutputMode,
    pub active_country_dir_name: String,
    pub active_language_dir_name: String,
}

impl ProjectDef {
    pub fn is_valid(&self) -> bool {
        self.output_mode.is_valid()
            && !(self.name.is_empty()
                || self.romfs_path.is_empty()
                || self.active_country_dir_name.is_empty()
                || self.active_language_dir_name.is_empty())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProjectOutputMode {
    Standard(String),
    Cobalt {
        data_path: String,
        patch_path: String,
        output_msbt: Option<String>,
    },
}

impl ProjectOutputMode {
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Standard(path) => !path.is_empty(),
            Self::Cobalt {
                data_path,
                patch_path,
                output_msbt: _,
            } => !(data_path.is_empty() || patch_path.is_empty()),
        }
    }
}

impl Default for ProjectOutputMode {
    fn default() -> Self {
        Self::Cobalt {
            data_path: String::new(),
            patch_path: String::new(),
            output_msbt: None,
        }
    }
}

impl From<ProjectDef> for AstraProject {
    fn from(value: ProjectDef) -> Self {
        let (output_dir, cobalt_dir, cobalt_msbt) = match value.output_mode {
            ProjectOutputMode::Standard(output_dir) => (output_dir.into(), None, None),
            ProjectOutputMode::Cobalt {
                data_path,
                patch_path,
                output_msbt,
            } => (
                data_path.into(),
                Some(patch_path.into()),
                output_msbt,
            ),
        };
        Self {
            backup_dir: PathBuf::from("Backups"),
            romfs_dir: value.romfs_path.into(),
            output_dir,
            cobalt_dir,
            cobalt_msbt,
            localization: PathLocalizer::new(
                value.active_country_dir_name,
                value.active_language_dir_name,
            ),
        }
    }
}
