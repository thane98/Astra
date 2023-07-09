use std::collections::{BTreeSet, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use astra_formats::{Book, MessageMap, TextBundle};
use indexmap::IndexMap;
use normpath::PathExt;

#[derive(Debug, Clone)]
pub struct PathLocalizer {
    country_dir: String,
    language_dir: String,
}

impl PathLocalizer {
    pub fn new(country_dir: String, language_dir: String) -> Self {
        Self {
            country_dir,
            language_dir,
        }
    }

    pub fn localize<T: AsRef<Path>>(&self, path_in_rom: T) -> Result<PathBuf> {
        let path_in_rom = path_in_rom.as_ref();
        if let (Some(parent), Some(file)) = (path_in_rom.parent(), path_in_rom.file_name()) {
            let mut path = parent.to_path_buf();
            path.push(&self.country_dir);
            path.push(&self.language_dir);
            path.push(file);
            Ok(path)
        } else {
            bail!(
                "could not determine file name and parent of path '{}'",
                path_in_rom.display()
            )
        }
    }
}

impl Default for PathLocalizer {
    fn default() -> Self {
        Self {
            country_dir: String::from("us"),
            language_dir: String::from("usen"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum FileSystemLayer {
    Directory(DirectoryFileSystemLayer),
}

impl FileSystemLayer {
    pub fn directory(root: impl Into<PathBuf>) -> Result<Self> {
        Ok(FileSystemLayer::Directory(DirectoryFileSystemLayer::new(
            root,
        )?))
    }

    pub fn read<T: AsRef<Path>>(&self, path_in_rom: T) -> Result<Vec<u8>> {
        match self {
            FileSystemLayer::Directory(directory) => directory.read(path_in_rom),
        }
    }

    pub fn write<T: AsRef<Path>>(&self, path_in_rom: T, contents: &[u8]) -> Result<()> {
        match self {
            FileSystemLayer::Directory(directory) => directory.write(path_in_rom, contents),
        }
    }

    pub fn list_files<T: AsRef<Path>>(
        &self,
        path_in_rom: T,
        glob: &str,
    ) -> Result<HashSet<PathBuf>> {
        match self {
            FileSystemLayer::Directory(directory) => directory.list_files(path_in_rom, glob),
        }
    }

    pub fn backup<T: AsRef<Path>, U: AsRef<Path>>(
        &self,
        path_in_rom: T,
        backup_root: U,
    ) -> Result<()> {
        match self {
            FileSystemLayer::Directory(directory) => directory.backup(path_in_rom, backup_root),
        }
    }

    pub fn exists<T: AsRef<Path>>(&self, path_in_rom: T) -> bool {
        match self {
            FileSystemLayer::Directory(directory) => directory.exists(path_in_rom),
        }
    }

    pub fn root(&self) -> &Path {
        match self {
            FileSystemLayer::Directory(directory) => directory.root(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct DirectoryFileSystemLayer {
    root: PathBuf,
}

impl DirectoryFileSystemLayer {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self> {
        let root: PathBuf = root.into().normalize()?.into();
        if !root.is_dir() {
            bail!("path '{}' is not a directory", root.display());
        }
        Ok(DirectoryFileSystemLayer { root })
    }

    pub fn read<T: AsRef<Path>>(&self, path_in_rom: T) -> Result<Vec<u8>> {
        let full_path = self.root.join(path_in_rom);
        if !full_path.is_file() {
            bail!("path '{}' is not a file", full_path.display());
        }
        let contents = std::fs::read(&full_path)
            .with_context(|| format!("failed to read file at path '{}'", full_path.display()))?;
        Ok(contents)
    }

    pub fn write<T: AsRef<Path>>(&self, path_in_rom: T, contents: &[u8]) -> Result<()> {
        let full_path = self.root.join(path_in_rom);
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent).with_context(|| {
                format!(
                    "failed to create directories to write file '{}'",
                    full_path.display()
                )
            })?;
        }
        std::fs::write(&full_path, contents)
            .with_context(|| format!("failed to write file at path '{}'", full_path.display()))?;
        Ok(())
    }

    pub fn list_files<T: AsRef<Path>>(
        &self,
        path_in_rom: T,
        glob: &str,
    ) -> Result<HashSet<PathBuf>> {
        let full_path = self.root.join(&path_in_rom).normalize()?;
        let full_path = full_path.as_path();
        if !full_path.is_dir() {
            bail!(
                "cannot list files at path '{}' because it is not a directory",
                full_path.display()
            );
        }
        let mut paths = HashSet::new();
        let root_component_count = self.root.components().count();
        for path in glob::glob(&full_path.join(glob).to_string_lossy())? {
            let path = path?;
            let entry_relative_to_root: PathBuf = path.iter().skip(root_component_count).collect();
            paths.insert(entry_relative_to_root);
        }
        Ok(paths)
    }

    pub fn backup<T: AsRef<Path>, U: AsRef<Path>>(
        &self,
        path_in_rom: T,
        backup_root: U,
    ) -> Result<()> {
        if self.exists(&path_in_rom) {
            let full_path = backup_root.as_ref().join(&path_in_rom);
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent).with_context(|| {
                    format!(
                        "failed to create directories to write file '{}'",
                        full_path.display()
                    )
                })?;
            }
            std::fs::copy(
                self.root.join(&path_in_rom),
                backup_root.as_ref().join(&path_in_rom),
            )?;
        }
        Ok(())
    }

    pub fn exists<T: AsRef<Path>>(&self, path_in_rom: T) -> bool {
        self.root.join(path_in_rom).exists()
    }

    pub fn root(&self) -> &Path {
        self.root.as_path()
    }
}

#[derive(Clone, Debug)]
pub struct LayeredFileSystem {
    layers: Vec<FileSystemLayer>,
}

impl LayeredFileSystem {
    pub fn new(layers: Vec<FileSystemLayer>) -> Result<Self> {
        if layers.is_empty() {
            bail!("file system must have at least one layer");
        }
        Ok(LayeredFileSystem { layers })
    }

    pub fn read<T: AsRef<Path>>(&self, path_in_rom: T) -> Result<Vec<u8>> {
        // TODO: Check if it's a file, not just if it exists
        let path = path_in_rom.as_ref();
        for layer in &self.layers {
            if layer.exists(path) {
                return layer.read(path);
            }
        }
        bail!("file '{}' does not exist in any layer", path.display());
    }

    pub fn write<T: AsRef<Path>>(&self, path_in_rom: T, contents: &[u8]) -> Result<()> {
        self.layers[0].write(path_in_rom, contents)
    }

    pub fn list_files<T: AsRef<Path>>(
        &self,
        path_in_rom: T,
        glob: &str,
    ) -> Result<HashSet<PathBuf>> {
        let path = path_in_rom.as_ref();
        let mut all_layers = HashSet::new();
        for layer in &self.layers {
            if layer.exists(path) {
                all_layers.extend(layer.list_files(path, glob)?);
            }
        }
        Ok(all_layers)
    }

    pub fn backup<T: AsRef<Path>, U: AsRef<Path>>(
        &self,
        path_in_rom: T,
        backup_root: U,
    ) -> Result<()> {
        let path = path_in_rom.as_ref();
        let backup_root = backup_root.as_ref();
        for layer in &self.layers {
            if layer.exists(path) {
                return layer.backup(path, backup_root);
            }
        }
        Ok(())
    }

    pub fn exists<T: AsRef<Path>>(&self, path_in_rom: T) -> bool {
        let path = path_in_rom.as_ref();
        for layer in &self.layers {
            if layer.exists(path) {
                return true;
            }
        }
        false
    }

    pub fn root(&self) -> &Path {
        self.layers[0].root()
    }
}

#[derive(Debug)]
pub struct LocalizedFileSystem {
    path_localizer: PathLocalizer,
    file_system: LayeredFileSystem,
}

impl LocalizedFileSystem {
    pub fn new(file_system: LayeredFileSystem, path_localizer: PathLocalizer) -> Self {
        Self {
            path_localizer,
            file_system,
        }
    }

    pub fn get_country_dirs(&self) -> Result<BTreeSet<String>> {
        Ok(self
            .list_files("fe_assets_message", "*", false)?
            .into_iter()
            .filter(|path| path.is_dir())
            .filter_map(|path| path.file_name().map(|p| p.to_string_lossy().to_string()))
            .collect())
    }

    pub fn get_language_dirs(&self, country_dir: &str) -> Result<BTreeSet<String>> {
        let target = Path::new("fe_assets_message").join(country_dir);
        Ok(self
            .list_files(target, "*", false)?
            .into_iter()
            .filter_map(|path| path.file_name().map(|p| p.to_string_lossy().to_string()))
            .collect())
    }

    pub fn read<T: AsRef<Path>>(&self, path_in_rom: T, localized: bool) -> Result<Vec<u8>> {
        let path = path_in_rom.as_ref();
        if localized {
            self.file_system.read(self.path_localizer.localize(path)?)
        } else {
            self.file_system.read(path)
        }
    }

    pub fn write<T: AsRef<Path>>(
        &self,
        path_in_rom: T,
        contents: &[u8],
        localized: bool,
    ) -> Result<()> {
        if localized {
            self.file_system
                .write(self.path_localizer.localize(path_in_rom)?, contents)
        } else {
            self.file_system.write(path_in_rom, contents)
        }
    }

    pub fn list_files<T: AsRef<Path>>(
        &self,
        path_in_rom: T,
        glob: &str,
        localized: bool,
    ) -> Result<HashSet<PathBuf>> {
        if localized {
            self.file_system
                .list_files(self.path_localizer.localize(path_in_rom)?, glob)
        } else {
            self.file_system.list_files(path_in_rom, glob)
        }
    }

    pub fn backup<T: AsRef<Path>, U: AsRef<Path>>(
        &self,
        path_in_rom: T,
        backup_root: U,
        localized: bool,
    ) -> Result<()> {
        if localized {
            self.file_system
                .backup(self.path_localizer.localize(path_in_rom)?, backup_root)
        } else {
            self.file_system.backup(path_in_rom, backup_root)
        }
    }

    pub fn exists<T: AsRef<Path>>(&self, path_in_rom: T, localized: bool) -> Result<bool> {
        Ok(if localized {
            self.file_system
                .exists(self.path_localizer.localize(path_in_rom)?)
        } else {
            self.file_system.exists(path_in_rom)
        })
    }

    pub fn root(&self) -> &Path {
        self.file_system.root()
    }
}

/// Support for [Cobalt](https://github.com/Raytwo/Cobalt) files.
/// Writes Cobalt patch files where supported and regular bundles otherwise.
// TODO: Refactor the file_system setup so this doesn't have to be a special case.
pub struct CobaltFileSystemProxy {
    main_file_system: Arc<LocalizedFileSystem>,
    cobalt_file_system: Option<DirectoryFileSystemLayer>,
    path_localizer: PathLocalizer,
}

impl CobaltFileSystemProxy {
    pub fn new(
        main_file_system: Arc<LocalizedFileSystem>,
        cobalt_root: Option<PathBuf>,
    ) -> Result<Self> {
        Ok(Self {
            cobalt_file_system: if let Some(root) = cobalt_root {
                Some(DirectoryFileSystemLayer::new(root)?)
            } else {
                None
            },
            path_localizer: main_file_system.path_localizer.clone(),
            main_file_system,
        })
    }

    pub fn read_book<P: AsRef<Path>>(&self, path: P) -> Result<(Book, Option<TextBundle>)> {
        if let Some(fs) = &self.cobalt_file_system {
            let path_in_cobalt = Self::format_cobalt_xml_path(&path);
            if Self::supports_cobalt_xml_patching(path.as_ref()) && fs.exists(&path_in_cobalt) {
                return fs
                    .read(path_in_cobalt)
                    .and_then(|data| Book::from_string(&String::from_utf8_lossy(&data)))
                    .map(|book| (book, None));
            }
        }

        let path_in_rom = Path::new(r"StreamingAssets\aa\Switch\fe_assets_gamedata\")
            .join(&path)
            .with_extension("xml.bundle");
        let raw = self.main_file_system.read(path_in_rom, false)?;
        let mut bundle = TextBundle::from_slice(&raw)?;
        let book = Book::from_string(&bundle.take_string()?)?;
        Ok((book, Some(bundle)))
    }

    pub fn read_cobalt_msbts(&self) -> Result<Vec<(PathBuf, IndexMap<String, String>)>> {
        let mut files = vec![];
        if let Some(fs) = &self.cobalt_file_system {
            if fs.exists(self.cobalt_msbt_dir()) {
                for path in fs.list_files(self.cobalt_msbt_dir(), "*")? {
                    let raw = fs.read(&path)?;
                    let mut message_map = MessageMap::from_slice(&raw).with_context(|| {
                        format!("failed to read Cobalt MSBT at path {}", path.display())
                    })?;
                    // TODO: Push this into astra_formats
                    let mut out = IndexMap::new();
                    let raw = std::mem::take(&mut message_map.messages);
                    for (k, v) in raw {
                        out.insert(
                            k,
                            astra_formats::parse_msbt_entry(&v).with_context(|| {
                                format!("failed to read Cobalt archive {}", path.display())
                            })?,
                        );
                    }
                    files.push((path, out))
                }
            }
        }
        Ok(files)
    }

    pub fn save_book<P: AsRef<Path>>(
        &self,
        path: P,
        book: &Book,
        bundle: Option<&mut TextBundle>,
    ) -> Result<()> {
        let mut raw_book = vec![0xEF, 0xBB, 0xBF];
        raw_book.extend(book.serialize()?.as_bytes());
        if let Some(fs) = &self.cobalt_file_system {
            if Self::supports_cobalt_xml_patching(path.as_ref()) {
                fs.write(Self::format_cobalt_xml_path(path), &raw_book)?;
                return Ok(());
            }
        }

        if let Some(bundle) = bundle {
            let path_in_rom = Path::new(r"StreamingAssets\aa\Switch\fe_assets_gamedata\")
                .join(&path)
                .with_extension("xml.bundle");
            bundle.replace_raw(raw_book)?;
            self.main_file_system
                .write(path_in_rom, &bundle.serialize()?, false)?;
            bundle.replace_raw(vec![])?; // Avoid holding the book blob in memory when it's not being used.
            Ok(())
        } else {
            bail!("Must provide a bundle to save books when Cobalt is not enabled.");
        }
    }

    pub fn save_msbt<P: AsRef<Path>>(
        &self,
        path: P,
        msbt: &IndexMap<String, String>,
    ) -> Result<()> {
        if let Some(fs) = &self.cobalt_file_system {
            let mut messages = IndexMap::new();
            for (k, v) in msbt {
                let msbt_tokens = astra_formats::parse_astra_script_entry(&v)?;
                messages.insert(k.clone(), astra_formats::pack_msbt_entry(&msbt_tokens));
            }
            let mut message_map = MessageMap::default();
            message_map.messages = messages;
            fs.write(path, &message_map.rehash_and_serialize()?)?;
        } else {
            bail!("Expected Cobalt folder but the project does not support it")
        }
        Ok(())
    }

    pub fn backup_xml<T: AsRef<Path>, U: AsRef<Path>>(
        &self,
        path: T,
        backup_root: U,
        use_cobalt_path: bool,
    ) -> Result<()> {
        if use_cobalt_path && Self::supports_cobalt_xml_patching(path.as_ref()) {
            let path = Self::format_cobalt_xml_path(path);
            if let Some(fs) = &self.cobalt_file_system {
                fs.backup(path, backup_root)?;
            } else {
                bail!("Expected Cobalt patch but the project does not support it")
            }
        } else {
            let path = Path::new(r"StreamingAssets\aa\Switch\fe_assets_gamedata\")
                .join(&path)
                .with_extension("xml.bundle");
            self.main_file_system.backup(path, backup_root, false)?;
        }
        Ok(())
    }

    pub fn backup_msbt<T: AsRef<Path>, U: AsRef<Path>>(
        &self,
        path: T,
        backup_root: U,
    ) -> Result<()> {
        if let Some(fs) = &self.cobalt_file_system {
            fs.backup(path, backup_root)?;
        } else {
            bail!("Expected Cobalt folder but the project does not support it")
        }
        Ok(())
    }

    fn cobalt_msbt_dir(&self) -> PathBuf {
        Path::new("msbt")
            .join("message")
            .join(&self.path_localizer.country_dir)
            .join(&self.path_localizer.language_dir)
    }

    fn format_cobalt_xml_path<P: AsRef<Path>>(path: P) -> PathBuf {
        let path = Path::new("xml").join(path);
        if let Some(file_name) = path.file_name() {
            let mut file_name = file_name.to_string_lossy().into_owned();
            let capitalized = format!("{}{file_name}", file_name.remove(0).to_uppercase());
            path.with_file_name(capitalized).with_extension("xml")
        } else {
            path.with_extension("xml")
        }
    }

    fn supports_cobalt_xml_patching(path: &Path) -> bool {
        !path.starts_with("dispos")
            && !path
                .file_stem()
                .map(|stem| match stem.to_string_lossy().as_ref() {
                    "reliance" | "terrain" => false,
                    _ => true,
                })
                .unwrap_or_default()
    }
}
