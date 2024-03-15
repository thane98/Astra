mod atlas_system;
mod book_system;
mod file_system;
mod message_system;
mod script_system;
mod terrain_system;

use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;

pub use anyhow as error;
use astra_types::{
    AnimSetBook, AssetTableBook, ChapterBook, DisposBook, GodBook, ItemBook, JobBook, ParamsBook,
    PersonBook, RelianceBook, ShopBook, SkillBook, TerrainBook,
};
use error::Context;
pub use image;
pub use parking_lot;

use atlas_system::AtlasSystem;
use book_system::BookSystem;
pub use book_system::OpenBook;
pub use file_system::*;
use image::DynamicImage;
use message_system::MessageSystem;
pub use message_system::{OpenMessageArchive, OpenMessageScript};
use script_system::ScriptSystem;
pub use terrain_system::OpenTerrain;
use terrain_system::TerrainSystem;

#[derive(Debug)]
pub enum RomSource {
    Directory(PathBuf),
    Network(String),
}

#[derive(Debug)]
pub struct AstraProject {
    pub backup_dir: PathBuf,
    pub rom_source: RomSource,
    pub output_dir: PathBuf,
    pub cobalt_dir: Option<PathBuf>,
    pub cobalt_msbt: Option<String>,
    pub localization: PathLocalizer,
}

pub struct Astra {
    backup_root: PathBuf,
    cobalt_msbt: Option<String>,
    atlas_system: AtlasSystem,
    book_system: BookSystem,
    message_system: MessageSystem,
    script_system: ScriptSystem,
    terrain_system: TerrainSystem,
}

impl Astra {
    pub fn load(project: AstraProject) -> Result<Self> {
        let file_system = Arc::new(LocalizedFileSystem::new(
            LayeredFileSystem::new(vec![
                FileSystemLayer::directory(project.output_dir)?,
                match &project.rom_source {
                    RomSource::Directory(directory) => FileSystemLayer::directory(directory)?,
                    RomSource::Network(ip) => FileSystemLayer::network(ip)?,
                },
            ])?,
            project.localization,
        ));
        let cobalt_proxy = Arc::new(CobaltFileSystemProxy::new(
            file_system.clone(),
            project.cobalt_dir,
        )?);
        Ok(Self {
            backup_root: project.backup_dir,
            cobalt_msbt: project.cobalt_msbt,
            atlas_system: AtlasSystem::load(&file_system)
                .context("Failed to load sprite atlases")?,
            book_system: BookSystem::load(cobalt_proxy.clone())
                .context("Failed to load books (fe_assets_gamedata)")?,
            script_system: ScriptSystem::new(cobalt_proxy.clone()),
            message_system: MessageSystem::load(file_system.clone(), cobalt_proxy)
                .context("Failed to load text data (MSBT)")?,
            terrain_system: TerrainSystem::load(file_system)
                .context("Failed to initialize terrain system")?,
        })
    }

    pub fn save(&self) -> Result<()> {
        let time = chrono::offset::Local::now().to_rfc3339().replace(':', "_");
        let backup_path = self.backup_root.join(time);
        self.book_system.save(backup_path.as_path())?;
        self.message_system.save(backup_path.as_path())?;
        self.script_system.save(backup_path.as_path())?;
        self.terrain_system.save(backup_path.as_path())?;
        Ok(())
    }

    pub fn cobalt_msbt(&self) -> Option<String> {
        self.cobalt_msbt
            .as_deref()
            .and_then(|path| Path::new(path).file_name())
            .map(|file_name| file_name.to_string_lossy().to_string())
    }

    pub fn open_script(
        &mut self,
        script_name: &str,
        editor_program: &str,
        editor_args: &str,
    ) -> Result<()> {
        self.script_system
            .open(script_name, editor_program, editor_args)
    }

    pub fn forget_script(&mut self, script_name: &str) {
        self.script_system.forget(script_name)
    }

    pub fn list_scripts(&self) -> impl Iterator<Item = &String> {
        self.script_system.list()
    }

    pub fn list_archives(&self) -> impl Iterator<Item = &String> {
        self.message_system.archives()
    }

    pub fn list_msbt_scripts(&self) -> BTreeSet<String> {
        self.message_system.scripts()
    }

    pub fn get_archive(&self, archive_id: &str) -> Option<&OpenMessageArchive> {
        self.message_system.get(archive_id)
    }

    pub fn open_msbt_script(&mut self, archive_name: &str) -> Result<OpenMessageScript> {
        self.message_system.open_script(archive_name)
    }

    pub fn consume_sprite_atlas(
        &mut self,
        atlas_id: &str,
    ) -> Option<HashMap<String, DynamicImage>> {
        self.atlas_system.take_sprites(atlas_id)
    }

    pub fn get_chapter_terrain(&mut self, terrain_name: &str) -> Option<OpenTerrain> {
        self.terrain_system.open(terrain_name).ok() // TODO: Log the error
    }

    pub fn get_dispos(&mut self, dispos_name: &str) -> Option<OpenBook<DisposBook>> {
        self.book_system.open_dispos(dispos_name).ok() // TODO: Log the error
    }

    pub fn get_asset_table_book(&self) -> OpenBook<AssetTableBook> {
        self.book_system.asset_table.clone()
    }

    pub fn get_anim_set_book(&self) -> OpenBook<AnimSetBook> {
        self.book_system.anim_set.clone()
    }

    pub fn get_god_book(&self) -> OpenBook<GodBook> {
        self.book_system.god.clone()
    }

    pub fn get_person_book(&self) -> OpenBook<PersonBook> {
        self.book_system.person.clone()
    }

    pub fn get_job_book(&self) -> OpenBook<JobBook> {
        self.book_system.job.clone()
    }

    pub fn get_item_book(&self) -> OpenBook<ItemBook> {
        self.book_system.item.clone()
    }

    pub fn get_chapter_book(&self) -> OpenBook<ChapterBook> {
        self.book_system.chapter.clone()
    }

    pub fn get_shop_book(&self) -> OpenBook<ShopBook> {
        self.book_system.shop.clone()
    }

    pub fn get_skill_book(&self) -> OpenBook<SkillBook> {
        self.book_system.skill.clone()
    }

    pub fn get_terrain_book(&self) -> OpenBook<TerrainBook> {
        self.book_system.terrain.clone()
    }

    pub fn get_param_book(&self) -> OpenBook<ParamsBook> {
        self.book_system.param.clone()
    }

    pub fn get_reliance_book(&self) -> OpenBook<RelianceBook> {
        self.book_system.reliance.clone()
    }
}
