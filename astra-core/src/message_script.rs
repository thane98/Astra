use std::path::Path;
use std::sync::Arc;

use crate::LocalizedFileSystem;

use anyhow::Result;
use astra_formats::MessageBundle;
use parking_lot::RwLock;

pub struct OpenMessageScript(Arc<RwLock<OpenMessageScriptInner>>);

impl Clone for OpenMessageScript {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl OpenMessageScript {
    pub fn load(file_system: &LocalizedFileSystem, path: String) -> Result<Self> {
        OpenMessageScriptInner::load(file_system, path)
            .map(|script| Self(Arc::new(RwLock::new(script))))
    }

    pub fn save(&self, file_system: &LocalizedFileSystem, backup_root: &Path) -> Result<()> {
        self.0.write().save(file_system, backup_root)
    }

    pub fn path(&self) -> String {
        self.0.read().path.clone()
    }

    pub fn access(&self, consumer: impl FnOnce(&mut String) -> bool) {
        let mut script = self.0.write();
        if consumer(&mut script.script) {
            script.dirty = true;
        }
    }
}

struct OpenMessageScriptInner {
    pub script: String,
    pub dirty: bool,
    bundle: MessageBundle,
    pub path: String,
}

impl OpenMessageScriptInner {
    pub fn load(file_system: &LocalizedFileSystem, path: String) -> Result<Self> {
        let contents = file_system.read(&path, false)?;
        let mut bundle = MessageBundle::from_slice(&contents)?;
        Ok(Self {
            script: bundle.take_script()?,
            bundle,
            path,
            dirty: false,
        })
    }

    pub fn save(&mut self, file_system: &LocalizedFileSystem, backup_root: &Path) -> Result<()> {
        if self.dirty {
            file_system.backup(&self.path, backup_root, false)?;
            self.bundle.replace_script(&self.script)?;
            let raw_bundle = self.bundle.serialize()?;
            // Clear out the data after building the bundle to avoid a memory leak.
            self.bundle.replace_script("")?;
            file_system.write(&self.path, &raw_bundle, false)?;
        }
        Ok(())
    }
}
