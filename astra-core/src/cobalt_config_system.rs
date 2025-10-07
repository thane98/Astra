use std::path::Path;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::CobaltFileSystemProxy;

// https://github.com/Raytwo/Cobalt/blob/master/crates/mods/src/manager.rs#L14C12-L14C22
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ModConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub author: String,
    #[serde(default)]
    pub dependencies: Vec<String>,
    pub repository: Option<String>,
}

pub struct CobaltConfigSystem {
    file_system: Arc<CobaltFileSystemProxy>,
    config: Option<ModConfig>,
}

impl CobaltConfigSystem {
    pub fn load(file_system: Arc<CobaltFileSystemProxy>) -> Result<Self> {
        Ok(Self {
            config: if let Some(raw_config) = file_system.read_cobalt_config() {
                serde_yaml::from_str(&raw_config).context("Failed to parse Cobalt config.yaml")?
            } else {
                None
            },
            file_system,
        })
    }

    pub fn save(&self, backup_root: &Path) -> Result<()> {
        if let Some(config) = &self.config {
            info!("Saving Cobalt config...");
            self.file_system.save_cobalt_config(&serde_yaml::to_string(&config)?, backup_root)?;
        } else {
            info!("NOT saving Cobalt config since the project does not have one.")
        }
        Ok(())
    }

    pub fn create_config(&mut self) -> Result<()> {
        if self.config.is_some() {
            bail!("Cannot create a new config.yaml since one already exists");
        }
        self.config = Some(ModConfig::default());
        Ok(())
    }

    pub fn get_config_mut(&mut self) -> Option<&mut ModConfig> {
        self.config.as_mut()
    }
}
