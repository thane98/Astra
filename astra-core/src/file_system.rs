use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, bail, Context, Result};
use astra_formats::{Book, TextBundle};
use indexmap::IndexMap;
use normpath::PathExt;
use quick_xml::events::Event;
use quick_xml::{Reader, Writer};
use tracing::{error, info, warn};

use crate::OpenBook;

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

    pub fn localization_dir(&self) -> PathBuf {
        Path::new(&self.country_dir).join(&self.language_dir)
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

#[derive(Debug)]
pub enum FileSystemLayer {
    Directory(DirectoryFileSystemLayer),
    Network(NetworkFileSystemLayer),
}

impl FileSystemLayer {
    pub fn directory(root: impl Into<PathBuf>) -> Result<Self> {
        Ok(FileSystemLayer::Directory(DirectoryFileSystemLayer::new(
            root,
        )?))
    }

    pub fn network(ip: &str) -> Result<Self> {
        Ok(FileSystemLayer::Network(NetworkFileSystemLayer::new(ip)?))
    }

    pub fn read<T: AsRef<Path>>(&self, path_in_rom: T) -> Result<Vec<u8>> {
        match self {
            FileSystemLayer::Directory(directory) => directory.read(path_in_rom),
            FileSystemLayer::Network(network) => network.read(path_in_rom),
        }
    }

    pub fn write<T: AsRef<Path>>(&self, path_in_rom: T, contents: &[u8]) -> Result<()> {
        match self {
            FileSystemLayer::Directory(directory) => directory.write(path_in_rom, contents),
            _ => bail!("Layer does not support this operation"),
        }
    }

    pub fn list_files<T: AsRef<Path>>(
        &self,
        path_in_rom: T,
        glob: &str,
    ) -> Result<HashSet<PathBuf>> {
        match self {
            FileSystemLayer::Directory(directory) => directory.list_files(path_in_rom, glob),
            FileSystemLayer::Network(network) => network.list_files(path_in_rom, glob),
        }
    }

    pub fn backup<T: AsRef<Path>, U: AsRef<Path>>(
        &self,
        path_in_rom: T,
        backup_root: U,
    ) -> Result<()> {
        match self {
            FileSystemLayer::Directory(directory) => directory.backup(path_in_rom, backup_root),
            _ => bail!("Layer does not support this operation"),
        }
    }

    pub fn exists<T: AsRef<Path>>(&self, path_in_rom: T) -> Result<bool> {
        match self {
            FileSystemLayer::Directory(directory) => directory.exists(path_in_rom),
            FileSystemLayer::Network(network) => network.exists(path_in_rom),
        }
    }

    pub fn root(&self) -> &Path {
        match self {
            FileSystemLayer::Directory(directory) => directory.root(),
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug)]
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
        if self.exists(&path_in_rom)? {
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

    pub fn exists<T: AsRef<Path>>(&self, path_in_rom: T) -> Result<bool> {
        Ok(self.root.join(path_in_rom).exists())
    }

    pub fn root(&self) -> &Path {
        self.root.as_path()
    }
}

#[derive(Debug)]
pub struct NetworkFileSystemLayer {
    addr: SocketAddr,
}

impl NetworkFileSystemLayer {
    pub fn new(ip: &str) -> Result<Self> {
        let addr = SocketAddr::from_str(ip)?;

        Ok(Self { addr })
    }

    fn with_connection<R>(&self, function: impl Fn(&mut TcpStream) -> Result<R>) -> Result<R> {
        let mut stream = TcpStream::connect_timeout(&self.addr, Duration::from_secs(10))?;
        stream.set_write_timeout(Some(Duration::from_secs(10)))?;
        stream.set_read_timeout(Some(Duration::from_secs(10)))?;

        let result = match function(&mut stream) {
            Ok(r) => Ok(r),
            Err(err) => {
                error!("Networked request failed with error '{:?}'", err);
                Err(err)
            }
        };

        stream.shutdown(std::net::Shutdown::Both)?;

        result
    }

    pub fn read<T: AsRef<Path>>(&self, path_in_rom: T) -> Result<Vec<u8>> {
        let path = path_in_rom.as_ref().to_string_lossy().to_string();

        info!("Requesting file {} from remote server...", path);

        self.with_connection(|stream| {
            stream.write_all(&[1])?;
            stream.write_all(path.as_bytes())?;
            stream.write_all("\n".as_bytes())?;

            let mut result_buffer = [0u8; 1];
            stream.read_exact(&mut result_buffer)?;
            let mut size_buffer = [0u8; 8];
            stream.read_exact(&mut size_buffer)?;
            let size = u64::from_be_bytes(size_buffer);
            let mut buffer = vec![0; size as usize];
            stream.read_exact(&mut buffer)?;
            if result_buffer[0] == 1 {
                bail!("{}", String::from_utf8_lossy(&buffer));
            }

            info!("Got file of size {}", size);
            Ok(buffer)
        })
        .with_context(|| format!("Failed to read file {} from server {}", path, self.addr))
    }

    pub fn exists<T: AsRef<Path>>(&self, path_in_rom: T) -> Result<bool> {
        let path = path_in_rom.as_ref().to_string_lossy().to_string();

        info!("Checking if file {} exists on remote server...", path);

        self.with_connection(|stream| {
            stream.write_all(&[0])?;
            stream.write_all(path.as_bytes())?;
            stream.write_all("\n".as_bytes())?;

            let mut buffer = [0u8; 1];
            stream.read_exact(&mut buffer)?;

            Ok(buffer[0] == 1)
        })
        .with_context(|| {
            format!(
                "Failed to check for path '{}' on server {}",
                path, self.addr
            )
        })
    }

    pub fn list_files<T: AsRef<Path>>(
        &self,
        path_in_rom: T,
        glob: &str,
    ) -> Result<HashSet<PathBuf>> {
        let path = path_in_rom.as_ref().to_string_lossy().to_string();

        info!("Listing files under path {} on remote server...", path);

        self.with_connection(|mut stream| {
            stream.write_all(&[2])?;
            stream.write_all(path.as_bytes())?;
            stream.write_all("\n".as_bytes())?;
            stream.write_all(glob.as_bytes())?;
            stream.write_all("\n".as_bytes())?;

            let mut result_buffer = [0u8; 1];
            stream.read_exact(&mut result_buffer)?;
            let mut size_buffer = [0u8; 8];
            stream.read_exact(&mut size_buffer)?;
            let size = u64::from_be_bytes(size_buffer);
            if result_buffer[0] == 0 {
                info!("Server found {} files", size);

                let mut paths = HashSet::new();
                let mut reader = BufReader::new(&mut stream);
                for _ in 0..size {
                    let mut line = String::new();
                    reader.read_line(&mut line)?;
                    paths.insert(PathBuf::from(line.trim()));
                }
                Ok(paths)
            } else {
                let mut buffer = vec![0; size as usize];
                stream.read_exact(&mut buffer)?;
                bail!("{}", String::from_utf8_lossy(&buffer))
            }
        })
        .with_context(|| {
            format!(
                "Failed to list files for path '{}' on server {}",
                path, self.addr
            )
        })
    }
}

#[derive(Debug)]
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
        let path = path_in_rom.as_ref();
        for layer in &self.layers {
            if layer.exists(path)? {
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
            if layer.exists(path)? {
                all_layers.extend(layer.list_files(path, glob)?);
            } else {
                warn!(
                    "Not listing files in layer {:?} because path {} does not exist in it",
                    layer,
                    path.display()
                );
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
        if self.layers[0].exists(path)? {
            info!(
                "Backuping up file {} to folder {}",
                path.display(),
                backup_root.display()
            );
            return self.layers[0].backup(path, backup_root);
        }
        Ok(())
    }

    pub fn exists<T: AsRef<Path>>(&self, path_in_rom: T) -> Result<bool> {
        let path = path_in_rom.as_ref();
        for layer in &self.layers {
            if layer.exists(path)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub fn root(&self) -> &Path {
        self.layers[0].root()
    }
}

#[derive(Debug)]
pub struct LocalizedFileSystem {
    pub path_localizer: PathLocalizer,
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
        if localized {
            self.file_system
                .exists(self.path_localizer.localize(path_in_rom)?)
        } else {
            self.file_system.exists(path_in_rom)
        }
    }

    pub fn root(&self) -> &Path {
        self.file_system.root()
    }
}

pub enum BundlePersistFormat {
    Cobalt {
        path: PathBuf,
    },
    Vanilla {
        bundle_path: PathBuf,
        bundle: TextBundle,
    },
}

impl Debug for BundlePersistFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cobalt { path } => f.debug_struct("Cobalt").field("path", path).finish(),
            Self::Vanilla { bundle_path, .. } => f
                .debug_struct("Vanilla")
                .field("bundle_path", bundle_path)
                .finish(),
        }
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

    pub fn list_scripts(&self) -> Result<BTreeSet<String>> {
        let mut scripts = self.main_file_system.list_files(
            r"StreamingAssets\aa\Switch\fe_assets_scripts",
            "*.txt.bundle",
            false,
        )?;
        if let Some(cobalt) = &self.cobalt_file_system {
            scripts.extend(
                cobalt
                    .list_files("scripts", "*.txt")
                    .ok()
                    .unwrap_or_default(),
            );
        }
        Ok(scripts
            .into_iter()
            .map(|script| {
                script
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .trim_end_matches(".txt")
                    .to_string()
            })
            .collect())
    }

    pub fn read_script(&self, script_file_name: &str) -> Result<(PathBuf, BundlePersistFormat)> {
        let path_in_cobalt = Path::new("scripts")
            .join(script_file_name)
            .with_extension("txt");
        let base_path =
            Path::new(r"StreamingAssets\aa\Switch\fe_assets_scripts").join(script_file_name);
        let script_path = base_path.with_extension("lua");

        // Try reading an existing script from the Cobalt FS.
        if let Some(cobalt) = &self.cobalt_file_system {
            // For backwards compatibility, copy old scripts to the Cobalt FS.
            if !cobalt.exists(&path_in_cobalt)?
                && self.main_file_system.exists(&script_path, false)?
            {
                info!(
                    "Found legacy script at {} - copying to {}",
                    script_path.display(),
                    path_in_cobalt.display()
                );
                let raw = self.main_file_system.read(&script_path, false)?;
                cobalt.write(&path_in_cobalt, &raw)?;
                info!(
                    "Successfully copied script to '{}'",
                    path_in_cobalt.display()
                );
            }
            if cobalt.exists(&path_in_cobalt)? {
                info!(
                    "Found script in Cobalt folder at {}",
                    path_in_cobalt.display()
                );
                return Ok((
                    cobalt.root.join(&path_in_cobalt),
                    BundlePersistFormat::Cobalt {
                        path: path_in_cobalt,
                    },
                ));
            }
        }

        // Load the bundle.
        let bundle_path = base_path.with_extension("txt.bundle");
        info!("Loading script from bundle '{}'", bundle_path.display());

        let raw_bundle = self.main_file_system.read(&bundle_path, false)?;
        let mut bundle = TextBundle::from_slice(&raw_bundle)?;
        let script_contents = bundle.take_raw()?;

        // If this is a Cobalt project, save the script in the Cobalt FS.
        if let Some(cobalt) = &self.cobalt_file_system {
            info!(
                "Loaded the script from its bundle. Saving to the Cobalt folder at '{}'",
                path_in_cobalt.display()
            );
            cobalt.write(&path_in_cobalt, &script_contents)?;
            info!(
                "Successfully wrote script to path '{}' in Cobalt",
                path_in_cobalt.display()
            );
            return Ok((
                cobalt.root.join(&path_in_cobalt),
                BundlePersistFormat::Cobalt {
                    path: path_in_cobalt,
                },
            ));
        }

        // If this is a normal project, save the lua file to the output folder.
        info!("Saving unbundled script to path {}", script_path.display());
        if !self.main_file_system.exists(&script_path, false)? {
            self.main_file_system
                .write(&script_path, &script_contents, false)?;
        }
        Ok((
            self.main_file_system.root().join(script_path),
            BundlePersistFormat::Vanilla {
                bundle_path,
                bundle,
            },
        ))
    }

    pub fn save_script<P: AsRef<Path>, P2: AsRef<Path>>(
        &self,
        absolute_script_path: P,
        persist_format: &mut BundlePersistFormat,
        backup_root: P2,
    ) -> Result<()> {
        if let BundlePersistFormat::Vanilla {
            bundle_path,
            bundle,
        } = persist_format
        {
            info!("Re-bundling script to {}", bundle_path.display());
            self.main_file_system
                .backup(&bundle_path, backup_root, false)?;
            let script_contents = std::fs::read(absolute_script_path)?;
            bundle.replace_raw(script_contents)?;
            self.main_file_system
                .write(bundle_path, &bundle.serialize()?, false)?;
            bundle.replace_raw(vec![])?;
        }
        Ok(())
    }

    pub fn read_book<PathType, DataType>(
        &self,
        path: PathType,
        xml_name: &str,
    ) -> Result<OpenBook<DataType>>
    where
        PathType: AsRef<Path>,
        DataType: TryFrom<Book, Error = anyhow::Error>,
    {
        // Try to read a Cobalt XML.
        if let Some(cobalt) = &self.cobalt_file_system {
            let path_in_cobalt = Self::format_cobalt_xml_path(&path, Some(xml_name));
            if cobalt.exists(&path_in_cobalt)? {
                info!(
                    "Loading book from Cobalt folder at {}",
                    path_in_cobalt.display()
                );
                return cobalt
                    .read(&path_in_cobalt)
                    .and_then(|raw| Book::from_string(&String::from_utf8_lossy(&raw)))
                    .and_then(|book| DataType::try_from(book))
                    .map(|data| {
                        OpenBook::new(
                            data,
                            BundlePersistFormat::Cobalt {
                                path: path_in_cobalt,
                            },
                        )
                    });
            }
        }

        // Read a normal bundle.
        let path_in_rom = Path::new(r"StreamingAssets\aa\Switch\fe_assets_gamedata\")
            .join(&path)
            .with_extension("xml.bundle");
        info!("Loading bundled book from path {}", path_in_rom.display());

        let raw = self.main_file_system.read(&path_in_rom, false)?;
        let mut bundle = TextBundle::from_slice(&raw)?;
        let book = Book::from_string(&bundle.take_string()?)?;
        let data = DataType::try_from(book)?;
        Ok(OpenBook::new(
            data,
            if self.cobalt_file_system.is_some() {
                BundlePersistFormat::Cobalt {
                    path: Self::format_cobalt_xml_path(&path, Some(xml_name)),
                }
            } else {
                BundlePersistFormat::Vanilla {
                    bundle_path: path_in_rom,
                    bundle,
                }
            },
        ))
    }

    // TODO: Delete this.
    fn format_cobalt_xml_path<P: AsRef<Path>>(path: P, xml_name: Option<&str>) -> PathBuf {
        let path = Path::new("xml").join(path);
        if let Some(xml_name) = xml_name {
            path.with_file_name(xml_name).with_extension("xml")
        } else if let Some(file_name) = path.file_name() {
            let mut file_name = file_name.to_string_lossy().into_owned();
            let capitalized = format!("{}{file_name}", file_name.remove(0).to_uppercase());
            path.with_file_name(capitalized).with_extension("xml")
        } else {
            path.with_extension("xml")
        }
    }

    pub fn save_book<PathType, DataType>(
        &self,
        book_data: &DataType,
        persist_format: &mut BundlePersistFormat,
        backup_root: PathType,
    ) -> Result<()>
    where
        PathType: AsRef<Path>,
        for<'a> &'a DataType: Into<Book>,
    {
        // Serialize the book.
        let book: Book = book_data.into();
        let mut raw_book = vec![0xEF, 0xBB, 0xBF];
        let pretty_xml = prettify_xml(&book.serialize()?)?;
        raw_book.extend(pretty_xml.as_bytes());

        match (persist_format, &self.cobalt_file_system) {
            (BundlePersistFormat::Cobalt { path }, Some(cobalt)) => {
                info!("Saving book to Cobalt folder at {}", path.display());
                cobalt.backup(&path, backup_root)?;
                cobalt.write(&path, &raw_book)?;
            }
            // TODO: Technically, there is a case where we could receive vanilla data and save as Cobalt.
            //       This should never happen, but we could support it anyway.
            (
                BundlePersistFormat::Vanilla {
                    bundle_path,
                    bundle,
                },
                None,
            ) => {
                // Happy path: straight to the layered FS output.
                info!("Saving book to bundle at {}", bundle_path.display());
                self.main_file_system
                    .backup(&bundle_path, backup_root, false)?;
                bundle.replace_raw(raw_book)?;
                self.main_file_system
                    .write(bundle_path, &bundle.serialize()?, false)?;
                bundle.replace_raw(vec![])?; // Avoid holding the book blob in memory while it's unused.
            }
            _ => bail!("Cannot save a Cobalt book because Cobalt's file system is not configured."),
        }
        Ok(())
    }

    pub fn read_cobalt_msbt<P: AsRef<Path>>(
        &self,
        path: P,
    ) -> Result<Option<IndexMap<String, String>>> {
        let path: &Path = path.as_ref();
        if let Some(fs) = &self.cobalt_file_system {
            let cobalt_path = self.to_cobalt_msbt_path(path)?;
            if fs.exists(cobalt_path.as_path())? {
                info!("Loading Cobalt MSBT from path {}", cobalt_path.display());
                let raw = fs.read(&cobalt_path)?;
                let script = String::from_utf8_lossy(&raw);
                let messages = astra_formats::convert_astra_script_to_entries(&script)?;
                return Ok(Some(messages));
            }
        }
        Ok(None)
    }

    pub fn save_msbt<P: AsRef<Path>>(
        &self,
        path: P,
        msbt: &IndexMap<String, String>,
    ) -> Result<()> {
        if let Some(fs) = &self.cobalt_file_system {
            let p: &Path = path.as_ref();
            let cobalt_path = self.to_cobalt_msbt_path(p)?;
            info!("Saving MSBT to Cobalt folder at {}", cobalt_path.display());
            let script = astra_formats::convert_entries_to_astra_script(msbt)?;
            fs.write(cobalt_path, script.as_bytes())?;
        } else {
            bail!("Expected Cobalt folder but the project does not support it")
        }
        Ok(())
    }

    pub fn backup_msbt<T: AsRef<Path>, U: AsRef<Path>>(
        &self,
        path: T,
        backup_root: U,
    ) -> Result<()> {
        if let Some(fs) = &self.cobalt_file_system {
            let cobalt_path = self.to_cobalt_msbt_path(path.as_ref())?;
            fs.backup(cobalt_path, backup_root)?;
        } else {
            bail!("Expected Cobalt folder but the project does not support it")
        }
        Ok(())
    }

    fn to_cobalt_msbt_path(&self, path: &Path) -> Result<PathBuf> {
        info!("Attempting to convert RomFS path '{}' to Cobalt path", path.display());
        let file_stem = path
            .file_name()
            .map(|name| {
                name.to_string_lossy()
                    .trim_end_matches(".bytes.bundle")
                    .to_string()
            })
            .ok_or_else(|| {
                anyhow!(
                    "Could not convert path '{}' to a Cobalt path",
                    path.display()
                )
            })?;
        Ok(self.cobalt_msbt_dir().join(format!("{}.txt", file_stem)))
    }

    fn cobalt_msbt_dir(&self) -> PathBuf {
        Path::new("msbt")
            .join("message")
            .join(&self.path_localizer.country_dir)
            .join(&self.path_localizer.language_dir)
    }

    pub fn is_cobalt_project(&self) -> bool {
        self.cobalt_file_system.is_some()
    }
}

// Borrowed from Raytwo
fn prettify_xml(xml: &str) -> Result<String> {
    let mut reader = Reader::from_str(xml);
    reader.trim_text(true);

    let mut writer = Writer::new_with_indent(Vec::new(), b'\t', 1);

    loop {
        match reader.read_event()? {
            Event::Eof => break, // exits the loop when reaching end of file
            event => {
                writer.write_event(event)?;
            }
        }
    }

    Ok(std::str::from_utf8(&writer.into_inner())?.to_string())
}
