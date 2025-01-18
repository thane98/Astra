use std::collections::{HashMap, HashSet};
use std::net::SocketAddrV4;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use anyhow::{bail, Result};
use astra_core::{AstraProject, PathLocalizer, RomSource};
use directories::ProjectDirs;
use egui::Color32;
use maplit::hashmap;
use serde::{Deserialize, Serialize};

use crate::Theme;

fn default_show_network_warning() -> bool {
    true
}

fn default_terrain_brightness() -> f32 {
    0.7
}

fn default_override_translation_keys_person() -> HashSet<String> {
    let mut overrides = HashSet::new();
    overrides.insert("PID_イル".to_string());
    overrides.insert("PID_エル".to_string());
    overrides
}

fn default_override_translation_keys_god() -> HashSet<String> {
    Default::default()
}

fn default_tile_color_overrides() -> HashMap<String, Color32> {
    hashmap! {
        "TID_大柱".into() => Color32::from_rgb(128, 128, 0),
        "TID_階段".into() => Color32::from_rgb(160, 160, 160),
        "TID_壁".into() => Color32::from_rgb(128, 128, 0),
        "TID_柱".into() => Color32::from_rgb(128, 128, 128),
        "TID_大柱".into() => Color32::from_rgb(128, 128, 128),
        "TID_崩れた床".into() => Color32::from_rgb(128, 128, 0),
        "TID_進入不可".into() => Color32::BLACK,
        "TID_林".into() => Color32::from_rgb(32, 100, 32),
        "TID_橋".into() => Color32::BROWN,
        "TID_海".into() => Color32::BLUE,
        "TID_海_全戦禁".into() => Color32::BLUE,
        "TID_海_影無".into() => Color32::BLUE,
    }
}

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
    #[serde(default = "default_terrain_brightness")]
    pub terrain_brightness: f32,
    #[serde(default = "default_tile_color_overrides")]
    pub tile_color_overrides: HashMap<String, Color32>,
    #[serde(default)]
    pub cobalt_path: String,
    #[serde(default = "default_show_network_warning")]
    pub show_network_warning: bool,
    #[serde(default = "default_override_translation_keys_person")]
    pub override_translation_keys_person: HashSet<String>,
    #[serde(default = "default_override_translation_keys_god")]
    pub override_translation_keys_god: HashSet<String>,
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

    pub fn has_configured_script_editor(&self) -> bool {
        !self.script_editor_process.is_empty() && !self.script_editor_command_args.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum RomSourceDef {
    Directory { romfs_path: String },
    Network { romfs_ip: String },
}

impl RomSourceDef {
    pub fn is_valid(&self) -> bool {
        match self {
            Self::Directory { romfs_path } => Path::new(romfs_path).is_dir(),
            Self::Network { romfs_ip } => SocketAddrV4::from_str(romfs_ip).is_ok(),
        }
    }
}

impl Default for RomSourceDef {
    fn default() -> Self {
        Self::Directory {
            romfs_path: Default::default(),
        }
    }
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ProjectDef {
    pub name: String,
    #[serde(flatten)]
    pub rom_source: RomSourceDef,
    pub output_mode: ProjectOutputMode,
    pub active_country_dir_name: String,
    pub active_language_dir_name: String,
}

impl ProjectDef {
    pub fn is_valid_for_new_cobalt_project(&self, config: &AppConfig) -> bool {
        self.rom_source.is_valid()
            && !self.name.is_empty()
            && !self.active_country_dir_name.is_empty()
            && !self.active_language_dir_name.is_empty()
            && Path::new(&config.cobalt_path).is_dir()
    }

    pub fn is_valid(&self) -> bool {
        self.output_mode.is_valid()
            && self.rom_source.is_valid()
            && !(self.name.is_empty()
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
            } => (data_path.into(), Some(patch_path.into()), output_msbt),
        };
        Self {
            backup_dir: PathBuf::from("Backups"),
            rom_source: match value.rom_source {
                RomSourceDef::Directory { romfs_path } => {
                    RomSource::Directory(PathBuf::from(romfs_path))
                }
                RomSourceDef::Network { romfs_ip } => RomSource::Network(romfs_ip),
            },
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
