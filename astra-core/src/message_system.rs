use std::collections::{BTreeSet, HashMap};
use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use astra_formats::indexmap::IndexMap;
use astra_formats::MessageBundle;
use indexmap::IndexSet;
use parking_lot::RwLock;
use tracing::{info, warn};

use crate::message_script::OpenMessageScript;
use crate::{CobaltFileSystemProxy, LocalizedFileSystem};

pub struct MessageSystem {
    archives: HashMap<String, OpenMessageArchive>,
    scripts: HashMap<String, OpenMessageScript>,
    file_system: Arc<LocalizedFileSystem>,
    cobalt: Arc<CobaltFileSystemProxy>,
}

impl MessageSystem {
    pub fn load(
        file_system: Arc<LocalizedFileSystem>,
        cobalt: Arc<CobaltFileSystemProxy>,
    ) -> Result<Self> {
        let targets = vec![
            (
                "accessories",
                "StreamingAssets/aa/Switch/fe_assets_message/accessories.bytes.bundle",
            ),
            (
                "achieve",
                "StreamingAssets/aa/Switch/fe_assets_message/achieve.bytes.bundle",
            ),
            (
                "bondsring",
                "StreamingAssets/aa/Switch/fe_assets_message/bondsring.bytes.bundle",
            ),
            (
                "cook",
                "StreamingAssets/aa/Switch/fe_assets_message/cook.bytes.bundle",
            ),
            (
                "friendlist",
                "StreamingAssets/aa/Switch/fe_assets_message/friendlist.bytes.bundle",
            ),
            (
                "friendlist_ex",
                "StreamingAssets/aa/Switch/fe_assets_message/friendlist_ex.bytes.bundle",
            ),
            (
                "gamedata",
                "StreamingAssets/aa/Switch/fe_assets_message/gamedata.bytes.bundle",
            ),
            (
                "hub",
                "StreamingAssets/aa/Switch/fe_assets_message/hub.bytes.bundle",
            ),
            (
                "hubcommon",
                "StreamingAssets/aa/Switch/fe_assets_message/hubcommon.bytes.bundle",
            ),
            (
                "hubcommon_p0",
                "StreamingAssets/aa/Switch/fe_assets_message/hubcommon_p0.bytes.bundle",
            ),
            (
                "hubcommon_p1",
                "StreamingAssets/aa/Switch/fe_assets_message/hubcommon_p1.bytes.bundle",
            ),
            (
                "hubcommon_p2",
                "StreamingAssets/aa/Switch/fe_assets_message/hubcommon_p2.bytes.bundle",
            ),
            (
                "hubcommon_p3",
                "StreamingAssets/aa/Switch/fe_assets_message/hubcommon_p3.bytes.bundle",
            ),
            (
                "item",
                "StreamingAssets/aa/Switch/fe_assets_message/item.bytes.bundle",
            ),
            (
                "job",
                "StreamingAssets/aa/Switch/fe_assets_message/job.bytes.bundle",
            ),
            (
                "maphistory",
                "StreamingAssets/aa/Switch/fe_assets_message/maphistory.bytes.bundle",
            ),
            (
                "moviename",
                "StreamingAssets/aa/Switch/fe_assets_message/moviename.bytes.bundle",
            ),
            (
                "musicname",
                "StreamingAssets/aa/Switch/fe_assets_message/musicname.bytes.bundle",
            ),
            (
                "network",
                "StreamingAssets/aa/Switch/fe_assets_message/network.bytes.bundle",
            ),
            (
                "skill",
                "StreamingAssets/aa/Switch/fe_assets_message/skill.bytes.bundle",
            ),
            (
                "system",
                "StreamingAssets/aa/Switch/fe_assets_message/system.bytes.bundle",
            ),
            (
                "patch0",
                "StreamingAssets/aa/Switch/fe_assets_message/patch0.bytes.bundle",
            ),
            (
                "patch1",
                "StreamingAssets/aa/Switch/fe_assets_message/patch1.bytes.bundle",
            ),
            (
                "patch2",
                "StreamingAssets/aa/Switch/fe_assets_message/patch2.bytes.bundle",
            ),
            (
                "patch3",
                "StreamingAssets/aa/Switch/fe_assets_message/patch3.bytes.bundle",
            ),
            (
                "person",
                "StreamingAssets/aa/Switch/fe_assets_message/person.bytes.bundle",
            ),
            (
                "profilecard",
                "StreamingAssets/aa/Switch/fe_assets_message/profilecard.bytes.bundle",
            ),
            (
                "tutorial",
                "StreamingAssets/aa/Switch/fe_assets_message/tutorial.bytes.bundle",
            ),
            (
                "tutorial_p0",
                "StreamingAssets/aa/Switch/fe_assets_message/tutorial_p0.bytes.bundle",
            ),
            (
                "tutorial_p1",
                "StreamingAssets/aa/Switch/fe_assets_message/tutorial_p1.bytes.bundle",
            ),
            (
                "tutorial_p2",
                "StreamingAssets/aa/Switch/fe_assets_message/tutorial_p2.bytes.bundle",
            ),
            (
                "tutorial_p3",
                "StreamingAssets/aa/Switch/fe_assets_message/tutorial_p3.bytes.bundle",
            ),
        ];
        let mut archives = HashMap::new();
        for (key, path) in targets {
            let archive = OpenMessageArchive::load(&file_system, &cobalt, path.to_string())
                .with_context(|| format!("failed to read archive {}", path))?;
            archives.insert(key.to_string(), archive);
        }
        Ok(Self {
            scripts: HashMap::new(),
            archives,
            file_system,
            cobalt,
        })
    }

    pub fn archives(&self) -> impl Iterator<Item = &String> {
        self.archives.keys()
    }

    pub fn scripts(&self) -> BTreeSet<String> {
        let mut scripts = BTreeSet::new();
        let localized_path = self.file_system.path_localizer.localization_dir();
        self.list_scripts_in_dir(&mut scripts, &localized_path);
        let puppet_path = Path::new("pu").join("puppet");
        self.list_scripts_in_dir(&mut scripts, &puppet_path);
        let sound_path = Path::new("so").join("sound");
        self.list_scripts_in_dir(&mut scripts, &sound_path);
        scripts
    }

    fn list_scripts_in_dir(&self, out: &mut BTreeSet<String>, dir: &Path) {
        let root = Path::new(r"StreamingAssets/aa/Switch/fe_assets_message").join(dir);
        info!("Listing scripts under ROM path {}", root.display());
        match self.file_system.list_files(&root, "*.bytes.bundle", false) {
            Ok(listing) => {
                for archive in listing {
                    let file_name = archive
                        .file_name()
                        .map(|f| f.to_string_lossy().to_string())
                        .unwrap_or_default();
                    let file_stem = file_name.strip_suffix(".bytes.bundle").unwrap_or_default();
                    if !self.archives.contains_key(file_stem) {
                        out.insert(dir.join(file_stem).to_string_lossy().to_string());
                    }
                }
            }
            Err(err) => {
                warn!(
                    "Encountered error while listing files in path {}: {:?}",
                    root.display(),
                    err
                );
            }
        }
    }

    pub fn open_script(&mut self, archive_name: &str) -> Result<OpenMessageScript> {
        // TODO: Do not allow opening a script which is already opened as an archive
        if let Some(script) = self.scripts.get(archive_name).cloned() {
            Ok(script)
        } else {
            let path = Path::new(r"StreamingAssets/aa/Switch/fe_assets_message")
                .join(archive_name)
                .with_extension("bytes.bundle");
            let script = OpenMessageScript::load(
                &self.file_system,
                path.to_string_lossy().to_string(), // TODO: Just take a PathBuf?
            )?;
            self.scripts
                .insert(archive_name.to_string(), script.clone());
            Ok(script)
        }
    }

    pub fn save(&self, backup_root: &Path) -> Result<()> {
        for archive in self.archives.values() {
            archive.save(&self.file_system, &self.cobalt, backup_root)?;
        }
        for script in self.scripts.values() {
            script.save(&self.file_system, backup_root)?;
        }
        Ok(())
    }

    pub fn get(&self, archive_id: &str) -> Option<&OpenMessageArchive> {
        self.archives.get(archive_id)
    }
}

pub struct OpenMessageArchive(Arc<RwLock<OpenMessageArchiveInner>>);

impl Clone for OpenMessageArchive {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl OpenMessageArchive {
    pub fn load(
        file_system: &LocalizedFileSystem,
        cobalt: &CobaltFileSystemProxy,
        path: String,
    ) -> Result<Self> {
        OpenMessageArchiveInner::load(file_system, cobalt, path)
            .map(|archive| Self(Arc::new(RwLock::new(archive))))
    }

    pub fn save(
        &self,
        file_system: &LocalizedFileSystem,
        cobalt: &CobaltFileSystemProxy,
        backup_root: &Path,
    ) -> Result<()> {
        self.0.write().save(file_system, cobalt, backup_root)
    }

    pub fn path(&self) -> String {
        self.0.read().path.clone()
    }

    pub fn read<R>(&self, consumer: impl FnOnce(&IndexMap<String, String>) -> R) -> R {
        consumer(&self.0.read().message_map)
    }

    pub fn put(&self, key: String, value: String) {
        let mut archive = self.0.write();
        archive.put(key, value);
    }
}

struct OpenMessageArchiveInner {
    message_map: IndexMap<String, String>,
    altered_keys: IndexSet<String>,
    bundle: MessageBundle,
    path: String,
}

impl OpenMessageArchiveInner {
    pub fn load(
        file_system: &LocalizedFileSystem,
        cobalt: &CobaltFileSystemProxy,
        path: String,
    ) -> Result<Self> {
        let contents = file_system.read(&path, true)?;
        let mut bundle = MessageBundle::from_slice(&contents)?;

        let mut message_map = bundle.take_entries()?;
        let mut altered_keys = IndexSet::new();
        if let Some(messages) = cobalt.read_cobalt_msbt(&path)? {
            altered_keys.extend(messages.keys().cloned());
            message_map.extend(messages);
        }

        Ok(Self {
            message_map,
            bundle,
            path,
            altered_keys,
        })
    }

    pub fn save(
        &mut self,
        file_system: &LocalizedFileSystem,
        cobalt: &CobaltFileSystemProxy,
        backup_root: &Path,
    ) -> Result<()> {
        if !self.altered_keys.is_empty() {
            if cobalt.is_cobalt_project() {
                let mut changes = IndexMap::new();
                for k in &self.altered_keys {
                    let value = self
                        .message_map
                        .get(k)
                        .cloned()
                        .ok_or_else(|| anyhow!("Failed to find altered key '{}'", k))?;
                    changes.insert(k.to_string(), value);
                }
                cobalt.backup_msbt(&self.path, backup_root)?;
                cobalt.save_msbt(&self.path, &changes)?;
            } else {
                file_system.backup(&self.path, backup_root, true)?;
                self.bundle.replace_entries(self.message_map.clone())?;
                let raw_bundle = self.bundle.serialize()?;
                // Clear out data after building the bundle to avoid a memory leak.
                self.bundle.replace_entries(IndexMap::new())?;
                file_system.write(&self.path, &raw_bundle, true)?;
            }
        } else {
            info!(
                "Skipping updates to message archive '{}' since no edits were made.",
                self.path
            );
        }
        Ok(())
    }

    pub fn put(&mut self, key: String, value: String) {
        self.altered_keys.insert(key.clone());
        self.message_map.insert(key, value);
    }
}
