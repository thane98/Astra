use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use astra_formats::Book;
use astra_types::{
    AchievementBook, AiBook, AmiiboBook, AnimSetBook, AnimalBook, ArenaBook, AssetTableBook,
    CalculatorBook, ChapterBook, ChartBook, CookBook, DisposBook, DragonRidePresetParamDataBook,
    DragonRidePrizeListBook, DragonRideTargetPatternBook, EffectBook, EncountBook, EndRollBook,
    FishingFishBook, FriendListBook, GodBook, GroundAttributeBook, HubAreaBook,
    HubDemoBook, HubDisposBook, HubFortuneTellingBook, HubInvestmentBook, HubMapIconBook,
    HubMyRoomBook, HubResourceBook, HubTalkBook, ItemBook, JobBook, JukeboxBook, KeyHelpDataBook,
    KillBonusBook, LaterTalkBook, MapEditorBook, MapHistoryBook, MascotBook, MovieBook,
    MuscleExerciseDataBook, MusicBook, ParamsBook, PersonBook, PhotographSpotBook, ProfileCardBook,
    RangeBook, RelayBook, RelianceBook, RingBook, RingCleaningVoiceBook, ShopBook, SkillBook,
    SoundEventBook, TerrainBook, TitleBook, TutorialBook, VibrationBook,
};
use parking_lot::RwLock;
use tracing::info;

use crate::{BundlePersistFormat, CobaltFileSystemProxy};

pub struct BookSystem {
    file_system: Arc<CobaltFileSystemProxy>,
    dispos: HashMap<String, OpenBook<DisposBook>>,
    pub(crate) achieve: OpenBook<AchievementBook>,
    pub(crate) ai: OpenBook<AiBook>,
    pub(crate) amiibo: OpenBook<AmiiboBook>,
    pub(crate) anim_set: OpenBook<AnimSetBook>,
    pub(crate) animal: OpenBook<AnimalBook>,
    pub(crate) arena: OpenBook<ArenaBook>,
    pub(crate) asset_table: OpenBook<AssetTableBook>,
    pub(crate) calculator: OpenBook<CalculatorBook>,
    pub(crate) chapter: OpenBook<ChapterBook>,
    pub(crate) chart: OpenBook<ChartBook>,
    pub(crate) cook: OpenBook<CookBook>,
    pub(crate) dragon_ride_preset_param: OpenBook<DragonRidePresetParamDataBook>,
    pub(crate) dragon_ride_prize_list: OpenBook<DragonRidePrizeListBook>,
    pub(crate) dragon_ride_target_pattern: OpenBook<DragonRideTargetPatternBook>,
    pub(crate) effect: OpenBook<EffectBook>,
    pub(crate) encount: OpenBook<EncountBook>,
    pub(crate) end_roll: OpenBook<EndRollBook>,
    pub(crate) fishing: OpenBook<FishingFishBook>,
    pub(crate) friend_list: OpenBook<FriendListBook>,
    pub(crate) god: OpenBook<GodBook>,
    pub(crate) ground_attribute: OpenBook<GroundAttributeBook>,
    pub(crate) hub_area: OpenBook<HubAreaBook>,
    pub(crate) hub_demo: OpenBook<HubDemoBook>,
    pub(crate) hub_dispos: OpenBook<HubDisposBook>,
    pub(crate) hub_fortune_telling: OpenBook<HubFortuneTellingBook>,
    pub(crate) hub_investment: OpenBook<HubInvestmentBook>,
    pub(crate) hub_map_icon: OpenBook<HubMapIconBook>,
    pub(crate) hub_my_room: OpenBook<HubMyRoomBook>,
    pub(crate) hub_resource: OpenBook<HubResourceBook>,
    pub(crate) hub_talk: OpenBook<HubTalkBook>,
    pub(crate) item: OpenBook<ItemBook>,
    pub(crate) job: OpenBook<JobBook>,
    pub(crate) jukebox: OpenBook<JukeboxBook>,
    pub(crate) key_help: OpenBook<KeyHelpDataBook>,
    pub(crate) kill_bonus: OpenBook<KillBonusBook>,
    pub(crate) later_talk: OpenBook<LaterTalkBook>,
    pub(crate) map_editor: OpenBook<MapEditorBook>,
    pub(crate) map_history: OpenBook<MapHistoryBook>,
    pub(crate) mascot: OpenBook<MascotBook>,
    pub(crate) movie: OpenBook<MovieBook>,
    pub(crate) music: OpenBook<MusicBook>,
    pub(crate) muscle_exercise: OpenBook<MuscleExerciseDataBook>,
    pub(crate) param: OpenBook<ParamsBook>,
    pub(crate) person: OpenBook<PersonBook>,
    pub(crate) photograph: OpenBook<PhotographSpotBook>,
    pub(crate) profile_card: OpenBook<ProfileCardBook>,
    pub(crate) range: OpenBook<RangeBook>,
    pub(crate) relay: OpenBook<RelayBook>,
    pub(crate) reliance: OpenBook<RelianceBook>,
    pub(crate) ring: OpenBook<RingBook>,
    pub(crate) ring_cleaning_voice: OpenBook<RingCleaningVoiceBook>,
    pub(crate) shop: OpenBook<ShopBook>,
    pub(crate) skill: OpenBook<SkillBook>,
    pub(crate) sound_event: OpenBook<SoundEventBook>,
    pub(crate) terrain: OpenBook<TerrainBook>,
    pub(crate) title: OpenBook<TitleBook>,
    pub(crate) tutorial: OpenBook<TutorialBook>,
    pub(crate) vibration: OpenBook<VibrationBook>,
}

impl BookSystem {
    pub fn load(file_system: Arc<CobaltFileSystemProxy>) -> Result<Self> {
        Ok(Self {
            achieve: OpenBook::load(&file_system, "achieve".into(), "Achieve")
                .context("Failed to load achieve")?,
            ai: OpenBook::load(&file_system, "ai".into(), "AI").context("Failed to load AI")?,
            amiibo: OpenBook::load(&file_system, "amiibolist".into(), "AmiiboList")
                .context("Failed to load amiibo")?,
            animal: OpenBook::load(&file_system, "animal".into(), "Animal")
                .context("Failed to load animal")?,
            arena: OpenBook::load(&file_system, "arena".into(), "Arena")
                .context("Failed to load arena")?,
            asset_table: OpenBook::load(&file_system, "assettable".into(), "AssetTable")
                .context("Failed to load asset table")?,
            anim_set: OpenBook::load(&file_system, "animset".into(), "AnimSet")
                .context("Failed to load anim set")?,
            calculator: OpenBook::load(&file_system, "calculator".into(), "Calculator")
                .context("Failed to load chapter")?,
            chapter: OpenBook::load(&file_system, "chapter".into(), "Chapter")
                .context("Failed to load chapter")?,
            chart: OpenBook::load(&file_system, "chart".into(), "Chart")
                .context("Failed to load chart")?,
            cook: OpenBook::load(&file_system, "cook".into(), "Cook")
                .context("Failed to load cook")?,
            dragon_ride_preset_param: OpenBook::load(
                &file_system,
                "dragonridepresetparamdata".into(),
                "DragonRidePresetParamData",
            )
            .context("Failed to load dragon ride preset param data")?,
            dragon_ride_prize_list: OpenBook::load(
                &file_system,
                "dragonrideprizelist".into(),
                "DragonRidePrizeList",
            )
            .context("Failed to load dragon ride prize list")?,
            dragon_ride_target_pattern: OpenBook::load(
                &file_system,
                "dragonridetargetpattern".into(),
                "DragonRideTargetPattern",
            )
            .context("Failed to load dragon ride target pattern")?,
            effect: OpenBook::load(&file_system, "effect".into(), "Effect")
                .context("Failed to load effect")?,
            encount: OpenBook::load(&file_system, "encount".into(), "Encount")
                .context("Failed to load encount")?,
            end_roll: OpenBook::load(&file_system, "endroll".into(), "EndRoll")
                .context("Failed to load end roll")?,
            fishing: OpenBook::load(&file_system, "fishingfishdata".into(), "FishingFishData")
                .context("Failed to load fishing")?,
            friend_list: OpenBook::load(&file_system, "friendlist".into(), "FriendList")
                .context("Failed to load friend list")?,
            ground_attribute: OpenBook::load(
                &file_system,
                "groundattribute".into(),
                "GroundAttributeBook",
            )
            .context("Failed to load ground attribute")?,
            god: OpenBook::load(&file_system, "god".into(), "God").context("Failed to load god")?,
            hub_area: OpenBook::load(&file_system, "hubarea".into(), "HubArea")
                .context("Failed to load hub area")?,
            hub_demo: OpenBook::load(&file_system, "hubdemo".into(), "HubDemo")
                .context("Failed to load hub demo")?,
            hub_dispos: OpenBook::load(&file_system, "hubdispos".into(), "HubDispos")
                .context("Failed to load hub dispos")?,
            hub_fortune_telling: OpenBook::load(
                &file_system,
                "hubfortunetelling".into(),
                "HubFortuneTelling",
            )
            .context("Failed to load hub fortune telling")?,
            hub_investment: OpenBook::load(&file_system, "hubinvestment".into(), "HubInvestment")
                .context("Failed to load hub investment")?,
            hub_map_icon: OpenBook::load(&file_system, "hubmapicon".into(), "HubMapIcon")
                .context("Failed to load hub map icon")?,
            hub_my_room: OpenBook::load(&file_system, "hubmyroom".into(), "HubMyRoom")
                .context("Failed to load hub myroom")?,
            hub_resource: OpenBook::load(&file_system, "hubresource".into(), "HubResource")
                .context("Failed to load hub resource")?,
            hub_talk: OpenBook::load(&file_system, "hubtalk".into(), "HubTalk")
                .context("Failed to load hub talk")?,
            item: OpenBook::load(&file_system, "item".into(), "Item")
                .context("Failed to load item")?,
            job: OpenBook::load(&file_system, "job".into(), "Job").context("Failed to load job")?,
            jukebox: OpenBook::load(&file_system, "jukebox".into(), "Jukebox")
                .context("Failed to load jukebox")?,
            key_help: OpenBook::load(&file_system, "keyhelpdata".into(), "KeyHelpData")
                .context("Failed to load key help")?,
            kill_bonus: OpenBook::load(&file_system, "killbonus".into(), "KillBonus")
                .context("Failed to load kill bonus")?,
            later_talk: OpenBook::load(&file_system, "latertalk".into(), "LaterTalk")
                .context("Failed to load later talk")?,
            map_editor: OpenBook::load(&file_system, "mapeditor".into(), "MapEditor")
                .context("Failed to load map editor")?,
            map_history: OpenBook::load(&file_system, "maphistory".into(), "MapHistory")
                .context("Failed to load map history")?,
            mascot: OpenBook::load(&file_system, "mascot".into(), "Mascot")
                .context("Failed to load mascot")?,
            movie: OpenBook::load(&file_system, "movie".into(), "Movie")
                .context("Failed to load movie")?,
            music: OpenBook::load(&file_system, "music".into(), "Music")
                .context("Failed to load music")?,
            muscle_exercise: OpenBook::load(
                &file_system,
                "muscleexercisedata".into(),
                "MuscleExerciseData",
            )
            .context("Failed to load muscle exercise data")?,
            param: OpenBook::load(&file_system, "params".into(), "Params")
                .context("Failed to load param")?,
            person: OpenBook::load(&file_system, "person".into(), "Person")
                .context("Failed to load person")?,
            photograph: OpenBook::load(&file_system, "photographspot".into(), "PhotographSpot")
                .context("Failed to load photograph")?,
            profile_card: OpenBook::load(&file_system, "profilecard".into(), "ProfileCard")
                .context("Failed to load profile card")?,
            range: OpenBook::load(&file_system, "range".into(), "Range")
                .context("Failed to load person")?,
            relay: OpenBook::load(&file_system, "relay".into(), "Relay")
                .context("Failed to load relay")?,
            reliance: OpenBook::load(&file_system, "reliance".into(), "Reliance")
                .context("Failed to load reliance")?,
            ring: OpenBook::load(&file_system, "ring".into(), "Ring")
                .context("Failed to load ring")?,
            ring_cleaning_voice: OpenBook::load(
                &file_system,
                "ringcleaningvoice".into(),
                "RingCleaningVoice",
            )
            .context("Failed to load ring cleaning voice")?,
            shop: OpenBook::load(&file_system, "shop".into(), "Shop")
                .context("Failed to load shop")?,
            skill: OpenBook::load(&file_system, "skill".into(), "Skill")
                .context("Failed to load skill")?,
            sound_event: OpenBook::load(&file_system, "soundevent".into(), "SoundEvent")
                .context("Failed to load sound event")?,
            terrain: OpenBook::load(&file_system, "terrain".into(), "Terrain")
                .context("Failed to load terrain")?,
            title: OpenBook::load(&file_system, "title".into(), "Title")
                .context("Failed to load title")?,
            tutorial: OpenBook::load(&file_system, "tutorial".into(), "Tutorial")
                .context("Failed to load tutorial")?,
            vibration: OpenBook::load(&file_system, "vibration".into(), "Vibration")
                .context("Failed to load vibration")?,
            dispos: HashMap::new(),
            file_system,
        })
    }

    pub fn open_dispos(&mut self, dispos_name: &str) -> Result<OpenBook<DisposBook>> {
        if let Some(dispos) = self.dispos.get(dispos_name) {
            Ok((*dispos).clone())
        } else {
            let path = Path::new("dispos").join(dispos_name.to_lowercase());
            let book = OpenBook::load(&self.file_system, path, &dispos_name.to_uppercase())?;
            self.dispos.insert(dispos_name.to_string(), book.clone());
            Ok(book)
        }
    }

    pub fn save(&self, backup_root: &Path) -> Result<()> {
        self.achieve.save(&self.file_system, backup_root)?;
        self.ai.save(&self.file_system, backup_root)?;
        self.amiibo.save(&self.file_system, backup_root)?;
        self.anim_set.save(&self.file_system, backup_root)?;
        self.animal.save(&self.file_system, backup_root)?;
        self.arena.save(&self.file_system, backup_root)?;
        self.asset_table.save(&self.file_system, backup_root)?;
        self.calculator.save(&self.file_system, backup_root)?;
        self.chapter.save(&self.file_system, backup_root)?;
        self.chart.save(&self.file_system, backup_root)?;
        self.cook.save(&self.file_system, backup_root)?;
        self.dragon_ride_preset_param.save(&self.file_system, backup_root)?;
        self.dragon_ride_prize_list.save(&self.file_system, backup_root)?;
        self.dragon_ride_target_pattern.save(&self.file_system, backup_root)?;
        self.effect.save(&self.file_system, backup_root)?;
        self.encount.save(&self.file_system, backup_root)?;
        self.end_roll.save(&self.file_system, backup_root)?;
        self.fishing.save(&self.file_system, backup_root)?;
        self.friend_list.save(&self.file_system, backup_root)?;
        self.god.save(&self.file_system, backup_root)?;
        self.ground_attribute.save(&self.file_system, backup_root)?;
        self.hub_area.save(&self.file_system, backup_root)?;
        self.hub_demo.save(&self.file_system, backup_root)?;
        self.hub_dispos.save(&self.file_system, backup_root)?;
        self.hub_fortune_telling.save(&self.file_system, backup_root)?;
        self.hub_investment.save(&self.file_system, backup_root)?;
        self.hub_map_icon.save(&self.file_system, backup_root)?;
        self.hub_my_room.save(&self.file_system, backup_root)?;
        self.hub_resource.save(&self.file_system, backup_root)?;
        self.hub_talk.save(&self.file_system, backup_root)?;
        self.person.save(&self.file_system, backup_root)?;
        self.item.save(&self.file_system, backup_root)?;
        self.job.save(&self.file_system, backup_root)?;
        self.jukebox.save(&self.file_system, backup_root)?;
        self.key_help.save(&self.file_system, backup_root)?;
        self.kill_bonus.save(&self.file_system, backup_root)?;
        self.later_talk.save(&self.file_system, backup_root)?;
        self.map_editor.save(&self.file_system, backup_root)?;
        self.map_history.save(&self.file_system, backup_root)?;
        self.mascot.save(&self.file_system, backup_root)?;
        self.movie.save(&self.file_system, backup_root)?;
        self.music.save(&self.file_system, backup_root)?;
        self.muscle_exercise.save(&self.file_system, backup_root)?;
        self.param.save(&self.file_system, backup_root)?;
        self.photograph.save(&self.file_system, backup_root)?;
        self.profile_card.save(&self.file_system, backup_root)?;
        self.range.save(&self.file_system, backup_root)?;
        self.relay.save(&self.file_system, backup_root)?;
        self.reliance.save(&self.file_system, backup_root)?;
        self.ring.save(&self.file_system, backup_root)?;
        self.ring_cleaning_voice.save(&self.file_system, backup_root)?;
        self.shop.save(&self.file_system, backup_root)?;
        self.skill.save(&self.file_system, backup_root)?;
        self.sound_event.save(&self.file_system, backup_root)?;
        self.terrain.save(&self.file_system, backup_root)?;
        self.title.save(&self.file_system, backup_root)?;
        self.tutorial.save(&self.file_system, backup_root)?;
        self.vibration.save(&self.file_system, backup_root)?;
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
    pub fn new(data: T, persist_format: BundlePersistFormat) -> Self {
        info!("Creating book with persist format {:?}", persist_format);
        Self(Arc::new(RwLock::new(OpenBookInner {
            data,
            persist_format,
            dirty: false,
        })))
    }

    pub fn read<R>(&self, consumer: impl FnOnce(&T) -> R) -> R {
        consumer(&self.0.read().data)
    }

    pub fn write<R>(&self, consumer: impl FnOnce(&mut T) -> R) -> R {
        consumer(&mut self.0.write().data)
    }

    pub fn mark_dirty(&self) {
        self.0.write().dirty = true;
    }
}

impl<T> OpenBook<T>
where
    T: TryFrom<Book, Error = anyhow::Error>,
    for<'a> &'a T: Into<Book>,
{
    pub fn load(
        file_system: &CobaltFileSystemProxy,
        path: PathBuf,
        xml_name: &str,
    ) -> Result<Self> {
        info!("Loading path={} xml_name={}", path.display(), xml_name);
        file_system.read_book(path, xml_name)
    }

    pub fn save(&self, file_system: &CobaltFileSystemProxy, backup_root: &Path) -> Result<()> {
        self.0.write().save(file_system, backup_root)
    }
}

struct OpenBookInner<T> {
    pub dirty: bool,
    pub data: T,
    pub persist_format: BundlePersistFormat,
}

impl<T> OpenBookInner<T>
where
    for<'a> &'a T: Into<Book>,
{
    fn save(&mut self, file_system: &CobaltFileSystemProxy, backup_root: &Path) -> Result<()> {
        if self.dirty {
            info!("Saving book to {:?}", self.persist_format);
            file_system.save_book(&self.data, &mut self.persist_format, backup_root)?;
            self.dirty = false;
        } else {
            info!(
                "Skipping write since book has not been modified: {:?}",
                self.persist_format
            );
        }
        Ok(())
    }
}
