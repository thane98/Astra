use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use astra_formats::{MonoBehavior, TerrainBundle, TerrainData};
use parking_lot::RwLock;

use crate::LocalizedFileSystem;

pub struct TerrainSystem {
    file_system: Arc<LocalizedFileSystem>,
    terrain: HashMap<String, OpenTerrain>,
}

impl TerrainSystem {
    pub fn load(file_system: Arc<LocalizedFileSystem>) -> Result<Self> {
        Ok(Self {
            terrain: HashMap::new(),
            file_system,
        })
    }

    pub fn open(&mut self, terrain_name: &str) -> Result<OpenTerrain> {
        if let Some(terrain) = self.terrain.get(terrain_name).cloned() {
            Ok(terrain)
        } else {
            let path = Path::new(r"StreamingAssets\aa\Switch\fe_assets_gamedata\terrains")
                .join(terrain_name.to_lowercase())
                .with_extension("bundle");
            let open_bundle = OpenTerrain::load(&self.file_system, path)?;
            let terrain = open_bundle.clone();
            self.terrain.insert(terrain_name.to_string(), open_bundle);
            Ok(terrain)
        }
    }

    pub fn save(&self, backup_root: &Path) -> Result<()> {
        for terrain in self.terrain.values() {
            terrain.save(&self.file_system, backup_root)?;
        }
        Ok(())
    }
}

pub struct OpenTerrain(Arc<RwLock<OpenTerrainInner>>);

impl Clone for OpenTerrain {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl OpenTerrain {
    pub fn load(file_system: &LocalizedFileSystem, path: PathBuf) -> Result<Self> {
        OpenTerrainInner::load(file_system, path)
            .map(|terrain| Self(Arc::new(RwLock::new(terrain))))
    }

    pub fn save(&self, file_system: &LocalizedFileSystem, backup_root: &Path) -> Result<()> {
        self.0.write().save(file_system, backup_root)
    }

    pub fn read<R>(&self, consumer: impl FnOnce(&TerrainData) -> R) -> R {
        consumer(&self.0.read().data.data)
    }

    pub fn write(&self, consumer: impl FnOnce(&mut TerrainData) -> bool) {
        let mut terrain = self.0.write();
        if consumer(&mut terrain.data.data) {
            terrain.dirty = true;
        }
    }
}

struct OpenTerrainInner {
    bundle: TerrainBundle,
    path: PathBuf,
    pub dirty: bool,
    pub data: MonoBehavior<TerrainData>,
}

impl OpenTerrainInner {
    pub fn load(file_system: &LocalizedFileSystem, path: PathBuf) -> Result<Self> {
        let raw_bundle = file_system.read(&path, false)?;
        let mut bundle = TerrainBundle::from_slice(&raw_bundle)?;
        let data = bundle.take_data()?;
        Ok(Self {
            data,
            bundle,
            path,
            dirty: false,
        })
    }

    pub fn save(&mut self, file_system: &LocalizedFileSystem, backup_root: &Path) -> Result<()> {
        if self.dirty {
            file_system.backup(&self.path, backup_root, false)?;
            self.bundle.replace_data(self.data.clone())?;
            file_system.write(&self.path, &self.bundle.serialize()?, false)?;
            self.bundle.replace_data(Default::default())?;
        }
        Ok(())
    }
}
