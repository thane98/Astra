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
    AchievementBook, AiBook, AmiiboBook, AnimSetBook, AnimalBook, ArenaBook, AssetTableBook,
    CalculatorBook, ChapterBook, ChartBook, CookBook, DisposBook, DragonRidePresetParamDataBook,
    DragonRidePrizeListBook, DragonRideTargetPatternBook, EffectBook, EncountBook, EndRollBook,
    FishingFishBook, FriendListBook, GodBook, GroundAttributeBook, HubAreaBook, HubDemoBook,
    HubDisposBook, HubFortuneTellingBook, HubInvestmentBook, HubMapIconBook, HubMyRoomBook,
    HubResourceBook, HubTalkBook, ItemBook, JobBook, JukeboxBook, KeyHelpDataBook, KillBonusBook,
    LaterTalkBook, MapEditorBook, MapHistoryBook, MascotBook, MovieBook, MuscleExerciseDataBook,
    MusicBook, ParamsBook, PersonBook, PhotographSpotBook, ProfileCardBook, RangeBook, RelayBook,
    RelianceBook, RingBook, RingCleaningVoiceBook, ShopBook, SkillBook, SoundEventBook,
    TerrainBook, TitleBook, TutorialBook, VibrationBook,
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

    pub fn get_achieve_book(&self) -> OpenBook<AchievementBook> {
        self.book_system.achieve.clone()
    }

    pub fn get_ai_book(&self) -> OpenBook<AiBook> {
        self.book_system.ai.clone()
    }

    pub fn get_amiibo_book(&self) -> OpenBook<AmiiboBook> {
        self.book_system.amiibo.clone()
    }

    pub fn get_anim_set_book(&self) -> OpenBook<AnimSetBook> {
        self.book_system.anim_set.clone()
    }

    pub fn get_animal_book(&self) -> OpenBook<AnimalBook> {
        self.book_system.animal.clone()
    }

    pub fn get_arena_book(&self) -> OpenBook<ArenaBook> {
        self.book_system.arena.clone()
    }

    pub fn get_asset_table_book(&self) -> OpenBook<AssetTableBook> {
        self.book_system.asset_table.clone()
    }

    pub fn get_calculator_book(&self) -> OpenBook<CalculatorBook> {
        self.book_system.calculator.clone()
    }

    pub fn get_chapter_book(&self) -> OpenBook<ChapterBook> {
        self.book_system.chapter.clone()
    }

    pub fn get_chart_book(&self) -> OpenBook<ChartBook> {
        self.book_system.chart.clone()
    }

    pub fn get_cook_book(&self) -> OpenBook<CookBook> {
        self.book_system.cook.clone()
    }

    pub fn get_dragon_ride_preset_param_book(&self) -> OpenBook<DragonRidePresetParamDataBook> {
        self.book_system.dragon_ride_preset_param.clone()
    }

    pub fn get_dragon_ride_prize_list_book(&self) -> OpenBook<DragonRidePrizeListBook> {
        self.book_system.dragon_ride_prize_list.clone()
    }

    pub fn get_dragon_ride_target_pattern_book(&self) -> OpenBook<DragonRideTargetPatternBook> {
        self.book_system.dragon_ride_target_pattern.clone()
    }

    pub fn get_effect_book(&self) -> OpenBook<EffectBook> {
        self.book_system.effect.clone()
    }

    pub fn get_encount_book(&self) -> OpenBook<EncountBook> {
        self.book_system.encount.clone()
    }

    pub fn get_end_roll_book(&self) -> OpenBook<EndRollBook> {
        self.book_system.end_roll.clone()
    }

    pub fn get_fishing_book(&self) -> OpenBook<FishingFishBook> {
        self.book_system.fishing.clone()
    }

    pub fn get_friend_list_book(&self) -> OpenBook<FriendListBook> {
        self.book_system.friend_list.clone()
    }

    pub fn get_god_book(&self) -> OpenBook<GodBook> {
        self.book_system.god.clone()
    }

    pub fn get_ground_attribute_book(&self) -> OpenBook<GroundAttributeBook> {
        self.book_system.ground_attribute.clone()
    }

    pub fn get_hub_area_book(&self) -> OpenBook<HubAreaBook> {
        self.book_system.hub_area.clone()
    }

    pub fn get_hub_demo_book(&self) -> OpenBook<HubDemoBook> {
        self.book_system.hub_demo.clone()
    }

    pub fn get_hub_dispos_book(&self) -> OpenBook<HubDisposBook> {
        self.book_system.hub_dispos.clone()
    }

    pub fn get_hub_fortune_telling_book(&self) -> OpenBook<HubFortuneTellingBook> {
        self.book_system.hub_fortune_telling.clone()
    }

    pub fn get_hub_investment_book(&self) -> OpenBook<HubInvestmentBook> {
        self.book_system.hub_investment.clone()
    }

    pub fn get_hub_map_icon_book(&self) -> OpenBook<HubMapIconBook> {
        self.book_system.hub_map_icon.clone()
    }

    pub fn get_hub_my_room_book(&self) -> OpenBook<HubMyRoomBook> {
        self.book_system.hub_my_room.clone()
    }

    pub fn get_hub_resource_book(&self) -> OpenBook<HubResourceBook> {
        self.book_system.hub_resource.clone()
    }

    pub fn get_hub_talk_book(&self) -> OpenBook<HubTalkBook> {
        self.book_system.hub_talk.clone()
    }

    pub fn get_item_book(&self) -> OpenBook<ItemBook> {
        self.book_system.item.clone()
    }

    pub fn get_job_book(&self) -> OpenBook<JobBook> {
        self.book_system.job.clone()
    }

    pub fn get_jukebox_book(&self) -> OpenBook<JukeboxBook> {
        self.book_system.jukebox.clone()
    }

    pub fn get_key_help_book(&self) -> OpenBook<KeyHelpDataBook> {
        self.book_system.key_help.clone()
    }

    pub fn get_kill_bonus_book(&self) -> OpenBook<KillBonusBook> {
        self.book_system.kill_bonus.clone()
    }

    pub fn get_later_talk_book(&self) -> OpenBook<LaterTalkBook> {
        self.book_system.later_talk.clone()
    }

    pub fn get_map_editor_book(&self) -> OpenBook<MapEditorBook> {
        self.book_system.map_editor.clone()
    }

    pub fn get_map_history_book(&self) -> OpenBook<MapHistoryBook> {
        self.book_system.map_history.clone()
    }

    pub fn get_mascot_book(&self) -> OpenBook<MascotBook> {
        self.book_system.mascot.clone()
    }

    pub fn get_movie_book(&self) -> OpenBook<MovieBook> {
        self.book_system.movie.clone()
    }

    pub fn get_music_book(&self) -> OpenBook<MusicBook> {
        self.book_system.music.clone()
    }

    pub fn get_muscle_exercise_book(&self) -> OpenBook<MuscleExerciseDataBook> {
        self.book_system.muscle_exercise.clone()
    }

    pub fn get_param_book(&self) -> OpenBook<ParamsBook> {
        self.book_system.param.clone()
    }

    pub fn get_person_book(&self) -> OpenBook<PersonBook> {
        self.book_system.person.clone()
    }

    pub fn get_photograph_book(&self) -> OpenBook<PhotographSpotBook> {
        self.book_system.photograph.clone()
    }

    pub fn get_profile_card_book(&self) -> OpenBook<ProfileCardBook> {
        self.book_system.profile_card.clone()
    }

    pub fn get_range_book(&self) -> OpenBook<RangeBook> {
        self.book_system.range.clone()
    }

    pub fn get_relay_book(&self) -> OpenBook<RelayBook> {
        self.book_system.relay.clone()
    }

    pub fn get_reliance_book(&self) -> OpenBook<RelianceBook> {
        self.book_system.reliance.clone()
    }

    pub fn get_ring_book(&self) -> OpenBook<RingBook> {
        self.book_system.ring.clone()
    }

    pub fn get_ring_cleaning_voice_book(&self) -> OpenBook<RingCleaningVoiceBook> {
        self.book_system.ring_cleaning_voice.clone()
    }

    pub fn get_shop_book(&self) -> OpenBook<ShopBook> {
        self.book_system.shop.clone()
    }

    pub fn get_skill_book(&self) -> OpenBook<SkillBook> {
        self.book_system.skill.clone()
    }

    pub fn get_sound_event_book(&self) -> OpenBook<SoundEventBook> {
        self.book_system.sound_event.clone()
    }

    pub fn get_terrain_book(&self) -> OpenBook<TerrainBook> {
        self.book_system.terrain.clone()
    }

    pub fn get_title_book(&self) -> OpenBook<TitleBook> {
        self.book_system.title.clone()
    }

    pub fn get_tutorial_book(&self) -> OpenBook<TutorialBook> {
        self.book_system.tutorial.clone()
    }

    pub fn get_vibration_book(&self) -> OpenBook<VibrationBook> {
        self.book_system.vibration.clone()
    }
}
