use std::collections::HashMap;
use std::fmt::Debug;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use astra_formats::{Book, TextBundle};
use astra_types::{
    AnimSetBook, AssetTableBook, ChapterBook, DisposBook, GodBook, ItemBook, JobBook, ParamsBook,
    PersonBook, RelianceBook, ShopBook, SkillBook, TerrainBook,
};
use parking_lot::RwLock;

use crate::CobaltFileSystemProxy;

pub struct BookSystem {
    file_system: Arc<CobaltFileSystemProxy>,
    dispos: HashMap<String, OpenBook<DisposBook>>,
    pub(crate) asset_table: OpenBook<AssetTableBook>,
    pub(crate) anim_set: OpenBook<AnimSetBook>,
    pub(crate) person: OpenBook<PersonBook>,
    pub(crate) job: OpenBook<JobBook>,
    pub(crate) item: OpenBook<ItemBook>,
    pub(crate) skill: OpenBook<SkillBook>,
    pub(crate) chapter: OpenBook<ChapterBook>,
    pub(crate) god: OpenBook<GodBook>,
    pub(crate) param: OpenBook<ParamsBook>,
    pub(crate) reliance: OpenBook<RelianceBook>,
    pub(crate) terrain: OpenBook<TerrainBook>,
    pub(crate) shop: OpenBook<ShopBook>,
}

impl BookSystem {
    pub fn load(file_system: Arc<CobaltFileSystemProxy>) -> Result<Self> {
        Ok(Self {
            asset_table: OpenBook::load(&file_system, "assettable".into(), Some("AssetTable"))
                .context("Failed to load asset table")?,
            anim_set: OpenBook::load(&file_system, "animset".into(), Some("AnimSet"))
                .context("Failed to load anim set")?,
            chapter: OpenBook::load(&file_system, "chapter".into(), Some("Chapter"))
                .context("Failed to load chapter")?,
            god: OpenBook::load(&file_system, "god".into(), Some("God"))
                .context("Failed to load god")?,
            person: OpenBook::load(&file_system, "person".into(), Some("Person"))
                .context("Failed to load person")?,
            job: OpenBook::load(&file_system, "job".into(), Some("Job"))
                .context("Failed to load job")?,
            item: OpenBook::load(&file_system, "item".into(), Some("Item"))
                .context("Failed to load item")?,
            param: OpenBook::load(&file_system, "params".into(), Some("Params"))
                .context("Failed to load param")?,
            reliance: OpenBook::load(&file_system, "reliance".into(), Some("Reliance"))
                .context("Failed to load reliance")?,
            shop: OpenBook::load(&file_system, "shop".into(), Some("Shop"))
                .context("Failed to load shop")?,
            skill: OpenBook::load(&file_system, "skill".into(), Some("Skill"))
                .context("Failed to load skill")?,
            terrain: OpenBook::load(&file_system, "terrain".into(), Some("Terrain"))
                .context("Failed to load terrain")?,
            dispos: HashMap::new(),
            file_system,
        })
    }

    pub fn open_dispos(&mut self, dispos_name: &str) -> Result<OpenBook<DisposBook>> {
        if let Some(dispos) = self.dispos.get(dispos_name) {
            Ok((*dispos).clone())
        } else {
            let path = Path::new("dispos").join(dispos_name.to_lowercase());
            let book = OpenBook::load(&self.file_system, path, None)?;
            self.dispos.insert(dispos_name.to_string(), book.clone());
            Ok(book)
        }
    }

    pub fn save(&self, backup_root: &Path) -> Result<()> {
        self.asset_table.save(&self.file_system, backup_root)?;
        self.anim_set.save(&self.file_system, backup_root)?;
        self.chapter.save(&self.file_system, backup_root)?;
        self.god.save(&self.file_system, backup_root)?;
        self.person.save(&self.file_system, backup_root)?;
        self.job.save(&self.file_system, backup_root)?;
        self.item.save(&self.file_system, backup_root)?;
        self.param.save(&self.file_system, backup_root)?;
        self.reliance.save(&self.file_system, backup_root)?;
        self.shop.save(&self.file_system, backup_root)?;
        self.skill.save(&self.file_system, backup_root)?;
        self.terrain.save(&self.file_system, backup_root)?;
        for book in self.dispos.values() {
            book.save(&self.file_system, backup_root)?;
        }
        Ok(())
    }
}

pub struct OpenBook<T>(Arc<RwLock<OpenBookInner<T>>>);

impl<T> Clone for OpenBook<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> OpenBook<T> {
    pub fn read<R>(&self, consumer: impl FnOnce(&T) -> R) -> R {
        consumer(&self.0.read().book)
    }

    pub fn write<R>(&self, consumer: impl FnOnce(&mut T) -> R) -> R {
        consumer(&mut self.0.write().book)
    }

    pub fn mark_dirty(&self) {
        self.0.write().dirty = true;
    }
}

impl<T> OpenBook<T>
where
    T: TryFrom<Book>,
    <T as TryFrom<Book>>::Error: Debug,
    for<'a> &'a T: Into<Book>,
{
    pub fn load(
        file_system: &CobaltFileSystemProxy,
        path: PathBuf,
        xml_name: Option<&str>,
    ) -> Result<Self> {
        Ok(Self(Arc::new(RwLock::new(OpenBookInner::load(
            file_system,
            path,
            xml_name,
        )?))))
    }

    pub fn save(&self, file_system: &CobaltFileSystemProxy, backup_root: &Path) -> Result<()> {
        self.0.write().save(file_system, backup_root)
    }
}

struct OpenBookInner<T> {
    bundle: Option<TextBundle>,
    path: PathBuf,
    xml_name: Option<String>,
    pub dirty: bool,
    pub book: T,
}

impl<T> OpenBookInner<T>
where
    T: TryFrom<Book>,
    <T as TryFrom<Book>>::Error: Debug,
    for<'a> &'a T: Into<Book>,
{
    pub fn load(
        file_system: &CobaltFileSystemProxy,
        path: PathBuf,
        xml_name: Option<&str>,
    ) -> Result<Self> {
        let (book, bundle) = file_system.read_book(&path, xml_name)?;
        let xml_name = if let Some(bundle) = &bundle {
            Some(bundle.get_asset_name()?)
        } else {
            None
        };
        Ok(Self {
            bundle,
            path,
            xml_name,
            dirty: false,
            book: T::try_from(book).map_err(|err| anyhow!("{:?}", err))?,
        })
    }

    pub fn save(&mut self, file_system: &CobaltFileSystemProxy, backup_root: &Path) -> Result<()> {
        if self.dirty {
            file_system.backup_xml(
                &self.path,
                backup_root,
                self.bundle.is_none(),
                self.xml_name.as_deref(),
            )?;
            let book: Book = (&self.book).into();
            file_system.save_book(
                &self.path,
                &book,
                self.bundle.as_mut(),
                self.xml_name.as_deref(),
            )?;
            self.dirty = false;
        }
        Ok(())
    }
}
