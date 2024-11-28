use std::borrow::Cow;

use egui::Ui;
use indexmap::IndexMap;

use crate::{
    bond_fragment_field, editable_list, editor_tab_strip, id_field, iron_field_i8,
    keyed_add_modal_content, model_drop_down, msbt_key_value_multiline, msbt_key_value_singleline,
    sheet_retriever, silver_field, standard_keyed_display, steel_field, CachedView, DropDownModal,
    EditorState, GroupEditorContent, GroupViewItem, KeyedViewItem, ListEditorContent,
    ModelDropDown, PropertyGrid, ViewItem,
};

use astra_types::{
    AnimalData, HubAnimalBonus, HubAnimalBonusGroup, HubAreaBook, HubAreaData, HubCrystalData,
    HubDemoBook, HubDemoData, HubDisposBook, HubFacilityData, HubFortuneTellingBook,
    HubFortuneTellingData, HubIngredientBonus, HubIngredientBonusGroup, HubInvestmentBook,
    HubItemBonus, HubMapIconBook, HubMapIconData, HubMaterialBonus, HubMyRoomBook, HubMyRoomData,
    HubNationData, HubResourceBook, HubResourceData, HubSpawn, HubSpawnRandomSet, HubTalkBook,
    HubTalkData, HubTalkFacilityData, HubTalkRelativeData, HubUnityBehavior, IngredientData, Item,
    Person,
};

sheet_retriever!(HubAreaData, HubAreaBook, hub_area_data, IndexMap<String, HubAreaData>);

impl ViewItem for HubAreaData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, aid, mid)
    }
}

impl KeyedViewItem for HubAreaData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.aid)
    }

    fn set_key(&mut self, key: String) {
        self.aid = key;
    }
}

sheet_retriever!(HubFacilityData, HubAreaBook, hub_facility_data, IndexMap<String, HubFacilityData>);

impl ViewItem for HubFacilityData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, aid, mid)
    }

    fn decorated(_: crate::DecorationKind<'_>) -> bool {
        true
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        _: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies
            .texture_cache
            .borrow_mut()
            .get_hub_icon(&self.icon_name)
            .map(|texture| (texture, 0.5))
    }
}

impl KeyedViewItem for HubFacilityData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.aid)
    }

    fn set_key(&mut self, key: String) {
        self.aid = key;
    }
}

sheet_retriever!(HubDemoData, HubDemoBook, hub_demo_data, IndexMap<String, HubDemoData>);

impl ViewItem for HubDemoData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, locator, mid)
    }
}

impl KeyedViewItem for HubDemoData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.mid)
    }

    fn set_key(&mut self, key: String) {
        self.mid = key;
    }
}

sheet_retriever!(HubSpawn, HubDisposBook, spawns, IndexMap<String, Vec<HubSpawn>>);

impl GroupViewItem for IndexMap<String, Vec<HubSpawn>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for HubSpawn {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(if self.hid.is_empty() {
            &self.locator
        } else {
            &self.hid
        })
    }
}

sheet_retriever!(HubSpawnRandomSet, HubDisposBook, random_sets, IndexMap<String, Vec<HubSpawnRandomSet>>);

impl GroupViewItem for IndexMap<String, Vec<HubSpawnRandomSet>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for HubSpawnRandomSet {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        if self.id.starts_with("PID_") {
            dependencies
                .person
                .read(|data| {
                    data.get(&self.id)
                        .map(|person| person.text(dependencies).to_string())
                })
                .map(Cow::Owned)
                .unwrap_or(Cow::Borrowed(&self.id))
        } else if self.id.starts_with("GID_") {
            dependencies
                .god
                .read(|data| {
                    data.get(&self.id)
                        .map(|god| god.text(dependencies).to_string())
                })
                .map(Cow::Owned)
                .unwrap_or(Cow::Borrowed(&self.id))
        } else if self.id.starts_with("IID_") {
            dependencies
                .item
                .read(|data| {
                    data.get(&self.id)
                        .map(|item| item.text(dependencies).to_string())
                })
                .map(Cow::Owned)
                .unwrap_or(Cow::Borrowed(&self.id))
        } else {
            Cow::Borrowed(&self.id)
        }
    }
}

sheet_retriever!(HubUnityBehavior, HubDisposBook, unity_behavior, IndexMap<String, Vec<HubUnityBehavior>>);

impl GroupViewItem for IndexMap<String, Vec<HubUnityBehavior>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for HubUnityBehavior {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!("{} + {}", self.move_type, self.body_name))
    }
}

sheet_retriever!(HubFortuneTellingData, HubFortuneTellingBook, fortune_telling_data, IndexMap<String, HubFortuneTellingData>);

impl ViewItem for HubFortuneTellingData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!("{} ({})", self.texture_name, self.id))
    }
}

impl KeyedViewItem for HubFortuneTellingData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(HubNationData, HubInvestmentBook, nation_data, IndexMap<String, HubNationData>);

impl ViewItem for HubNationData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, id, name)
    }

    fn decorated(_: crate::DecorationKind<'_>) -> bool {
        true
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        _: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        let mut cache = dependencies.texture_cache.borrow_mut();
        cache
            .get_hub_cafe_icon(&self.symbol_texture)
            .or_else(|| cache.get_hub_cafe_icon("Other"))
            .map(|texture| (texture, 0.5))
    }
}

impl KeyedViewItem for HubNationData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(HubMaterialBonus, HubInvestmentBook, material_bonuses, IndexMap<String, Vec<HubMaterialBonus>>);

impl GroupViewItem for IndexMap<String, Vec<HubMaterialBonus>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for HubMaterialBonus {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!("{} Gold Invested", self.cost))
    }
}

sheet_retriever!(HubIngredientBonus, HubInvestmentBook, ingredient_bonuses, IndexMap<String, Vec<HubIngredientBonus>>);

impl GroupViewItem for IndexMap<String, Vec<HubIngredientBonus>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for HubIngredientBonus {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .ingredient
            .read(|data| {
                data.get(&self.foodstuff)
                    .map(|d| d.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.foodstuff))
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        IngredientData::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.ingredient.read(|data| {
            data.get(&self.foodstuff)
                .and_then(|d| d.decoration(dependencies, kind))
        })
    }
}

sheet_retriever!(HubAnimalBonus, HubInvestmentBook, animal_bonuses, IndexMap<String, Vec<HubAnimalBonus>>);

impl GroupViewItem for IndexMap<String, Vec<HubAnimalBonus>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for HubAnimalBonus {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .animal
            .read(|data| {
                data.get(&self.anid)
                    .map(|animal| animal.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.anid))
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        AnimalData::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.animal.read(|data| {
            data.get(&self.anid)
                .and_then(|d| d.decoration(dependencies, kind))
        })
    }
}

sheet_retriever!(HubItemBonus, HubInvestmentBook, item_bonuses, IndexMap<String, Vec<HubItemBonus>>);

impl GroupViewItem for IndexMap<String, Vec<HubItemBonus>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for HubItemBonus {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .item
            .read(|data| {
                data.get(&self.item_id)
                    .map(|item| item.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.item_id))
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        Item::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.item.read(|data| {
            data.get(&self.item_id)
                .and_then(|item| item.decoration(dependencies, kind))
        })
    }
}

sheet_retriever!(HubIngredientBonusGroup, HubInvestmentBook, ingredient_bonus_groups, IndexMap<String, Vec<HubIngredientBonusGroup>>);

impl GroupViewItem for IndexMap<String, Vec<HubIngredientBonusGroup>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for HubIngredientBonusGroup {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .ingredient
            .read(|data| {
                data.get(&self.foodstuff)
                    .map(|d| d.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.foodstuff))
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        IngredientData::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.ingredient.read(|data| {
            data.get(&self.foodstuff)
                .and_then(|d| d.decoration(dependencies, kind))
        })
    }
}

sheet_retriever!(HubAnimalBonusGroup, HubInvestmentBook, animal_bonus_groups, IndexMap<String, Vec<HubAnimalBonusGroup>>);

impl GroupViewItem for IndexMap<String, Vec<HubAnimalBonusGroup>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for HubAnimalBonusGroup {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .animal
            .read(|data| {
                data.get(&self.animal_id)
                    .map(|item| item.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.animal_id))
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        AnimalData::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.animal.read(|data| {
            data.get(&self.animal_id)
                .and_then(|d| d.decoration(dependencies, kind))
        })
    }
}

sheet_retriever!(HubMapIconData, HubMapIconBook, map_icon_data, IndexMap<String, HubMapIconData>);

impl ViewItem for HubMapIconData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.icon_name)
    }

    fn decorated(_: crate::DecorationKind<'_>) -> bool {
        true
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        _: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies
            .texture_cache
            .borrow_mut()
            .get_hub_icon(&self.icon_name)
            .map(|texture| (texture, 0.5))
    }
}

impl KeyedViewItem for HubMapIconData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.icon_name)
    }

    fn set_key(&mut self, key: String) {
        self.icon_name = key;
    }
}

sheet_retriever!(HubMyRoomData, HubMyRoomBook, my_room_data, IndexMap<String, HubMyRoomData>);

impl ViewItem for HubMyRoomData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .person
            .read(|data| {
                data.get(&self.pid)
                    .map(|person| person.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.pid))
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        Person::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.person.read(|data| {
            data.get(&self.pid)
                .and_then(|d| d.decoration(dependencies, kind))
        })
    }
}

impl KeyedViewItem for HubMyRoomData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.pid)
    }

    fn set_key(&mut self, key: String) {
        self.pid = key;
    }
}

sheet_retriever!(HubResourceData, HubResourceBook, resources, IndexMap<String, HubResourceData>);

impl ViewItem for HubResourceData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }
}

impl KeyedViewItem for HubResourceData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.name)
    }

    fn set_key(&mut self, key: String) {
        self.name = key;
    }
}

sheet_retriever!(HubTalkData, HubTalkBook, talk_data, IndexMap<String, HubTalkData>);

impl ViewItem for HubTalkData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.krid)
    }
}

impl KeyedViewItem for HubTalkData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.krid)
    }

    fn set_key(&mut self, key: String) {
        self.krid = key;
    }
}

sheet_retriever!(HubTalkRelativeData, HubTalkBook, relative_data, IndexMap<String, HubTalkRelativeData>);

impl ViewItem for HubTalkRelativeData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .person
            .read(|data| {
                data.get(&self.pid)
                    .map(|person| person.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.pid))
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        Person::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.person.read(|data| {
            data.get(&self.pid)
                .and_then(|d| d.decoration(dependencies, kind))
        })
    }
}

impl KeyedViewItem for HubTalkRelativeData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.pid)
    }

    fn set_key(&mut self, key: String) {
        self.pid = key;
    }
}

sheet_retriever!(HubTalkFacilityData, HubTalkBook, talk_facility_data, IndexMap<String, HubTalkFacilityData>);

impl ViewItem for HubTalkFacilityData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .person
            .read(|data| {
                data.get(&self.pid)
                    .map(|person| person.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.pattern))
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        Person::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.person.read(|data| {
            data.get(&self.pid)
                .and_then(|d| d.decoration(dependencies, kind))
        })
    }
}

impl KeyedViewItem for HubTalkFacilityData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.pattern)
    }

    fn set_key(&mut self, key: String) {
        self.pattern = key;
    }
}

sheet_retriever!(HubCrystalData, HubTalkBook, crystal_data, IndexMap<String, HubCrystalData>);

impl ViewItem for HubCrystalData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        dependencies
            .chapter
            .read(|data| {
                data.get(&format!("CID_{}", self.cid))
                    .map(|chapter| chapter.text(dependencies).to_string())
            })
            .map(Cow::Owned)
            .unwrap_or(Cow::Borrowed(&self.cid))
    }
}

impl KeyedViewItem for HubCrystalData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.cid)
    }

    fn set_key(&mut self, key: String) {
        self.cid = key;
    }
}

fn area_key_transform(key: &str) -> String {
    let mut id = String::from("AID_");
    id.push_str(key);
    id
}

fn area_key_reverse_transform(key: &str) -> String {
    key.trim_start_matches("AID_").to_owned()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Area,
    Facility,
    Demo,
    Spawn,
    SpawnSet,
    UnityBehavior,
    Fortune,
    Nation,
    MaterialBonus,
    IngredientBonus,
    AnimalBonus,
    ItemBonus,
    IngredientBonusGroup,
    AnimalBonusGroup,
    MapIconData,
    MyRoomData,
    TalkData,
    TalkRelativeData,
    TalkFacilityData,
    CrystalData,
}

pub struct HubAreaEditor {
    tab: Tab,
    hub_area_data: HubAreaDataSheet,
    hub_facility_data: HubFacilityDataSheet,
    hub_demo_data: HubDemoDataSheet,
    spawns: HubSpawnSheet,
    random_sets: HubSpawnRandomSetSheet,
    unity_behavior: HubUnityBehaviorSheet,
    fortune_telling_data: HubFortuneTellingDataSheet,
    nation_data: HubNationDataSheet,
    material_bonuses: HubMaterialBonusSheet,
    ingredient_bonuses: HubIngredientBonusSheet,
    animal_bonuses: HubAnimalBonusSheet,
    item_bonuses: HubItemBonusSheet,
    ingredient_bonus_groups: HubIngredientBonusGroupSheet,
    animal_bonus_groups: HubAnimalBonusGroupSheet,
    map_icon_data: HubMapIconDataSheet,
    my_room_data: HubMyRoomDataSheet,
    talk_data: HubTalkDataSheet,
    relative_data: HubTalkRelativeDataSheet,
    talk_facility_data: HubTalkFacilityDataSheet,
    crystal_data: HubCrystalDataSheet,
    facility_data_cache: CachedView<HubFacilityDataSheetRetriever, HubAreaBook, HubFacilityData>,
    hub_area_data_content:
        ListEditorContent<IndexMap<String, HubAreaData>, HubAreaData, EditorState>,
    hub_facility_data_content:
        ListEditorContent<IndexMap<String, HubFacilityData>, HubFacilityData, EditorState>,
    hub_demo_data_content:
        ListEditorContent<IndexMap<String, HubDemoData>, HubDemoData, EditorState>,
    spawns_content: GroupEditorContent,
    random_sets_content: GroupEditorContent,
    unity_behavior_content: GroupEditorContent,
    fortune_telling_data_content: ListEditorContent<
        IndexMap<String, HubFortuneTellingData>,
        HubFortuneTellingData,
        EditorState,
    >,
    nation_data_content:
        ListEditorContent<IndexMap<String, HubNationData>, HubNationData, EditorState>,
    material_bonuses_content: GroupEditorContent,
    ingredient_bonuses_content: GroupEditorContent,
    animal_bonuses_content: GroupEditorContent,
    item_bonuses_content: GroupEditorContent,
    ingredient_bonus_groups_content: GroupEditorContent,
    animal_bonus_groups_content: GroupEditorContent,
    map_icon_data_content:
        ListEditorContent<IndexMap<String, HubMapIconData>, HubMapIconData, EditorState>,
    my_room_data_content:
        ListEditorContent<IndexMap<String, HubMyRoomData>, HubMyRoomData, EditorState>,
    talk_data_content: ListEditorContent<IndexMap<String, HubTalkData>, HubTalkData, EditorState>,
    relative_data_content:
        ListEditorContent<IndexMap<String, HubTalkRelativeData>, HubTalkRelativeData, EditorState>,
    talk_facility_data_content:
        ListEditorContent<IndexMap<String, HubTalkFacilityData>, HubTalkFacilityData, EditorState>,
    crystal_data_content:
        ListEditorContent<IndexMap<String, HubCrystalData>, HubCrystalData, EditorState>,
}

impl HubAreaEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Area,
            hub_area_data: state.hub_area_data.clone(),
            hub_facility_data: state.hub_facility_data.clone(),
            hub_demo_data: state.hub_demo_data.clone(),
            spawns: state.hub_spawns.clone(),
            random_sets: state.hub_random_sets.clone(),
            unity_behavior: state.hub_unity_behavior.clone(),
            fortune_telling_data: state.hub_fortune_telling_data.clone(),
            nation_data: state.hub_nation_data.clone(),
            material_bonuses: state.hub_material_bonuses.clone(),
            ingredient_bonuses: state.hub_ingredient_bonuses.clone(),
            animal_bonuses: state.hub_animal_bonuses.clone(),
            item_bonuses: state.hub_item_bonuses.clone(),
            ingredient_bonus_groups: state.hub_ingredient_bonus_groups.clone(),
            animal_bonus_groups: state.hub_animal_bonus_groups.clone(),
            map_icon_data: state.hub_map_icon_data.clone(),
            my_room_data: state.hub_my_room_data.clone(),
            talk_data: state.hub_talk_data.clone(),
            relative_data: state.hub_relative_data.clone(),
            talk_facility_data: state.hub_talk_facility_data.clone(),
            crystal_data: state.hub_crystal_data.clone(),
            facility_data_cache: CachedView::new(state.hub_facility_data.clone(), state),
            hub_area_data_content: ListEditorContent::new("hub_area_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            hub_facility_data_content: ListEditorContent::new("hub_facility_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            hub_demo_data_content: ListEditorContent::new("hub_demo_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            spawns_content: GroupEditorContent::new("spawns_editor"),
            random_sets_content: GroupEditorContent::new("random_sets_editor"),
            unity_behavior_content: GroupEditorContent::new("unity_behavior_editor"),
            fortune_telling_data_content: ListEditorContent::new("fortune_telling_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            nation_data_content: ListEditorContent::new("nation_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            material_bonuses_content: GroupEditorContent::new("material_bonuses_editor"),
            ingredient_bonuses_content: GroupEditorContent::new("ingredient_bonuses_editor"),
            animal_bonuses_content: GroupEditorContent::new("animal_bonuses_editor"),
            item_bonuses_content: GroupEditorContent::new("item_bonuses_editor"),
            ingredient_bonus_groups_content: GroupEditorContent::new(
                "ingredient_bonus_groups_editor",
            ),
            animal_bonus_groups_content: GroupEditorContent::new("animal_bonus_groups_editor"),
            map_icon_data_content: ListEditorContent::new("map_icon_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            my_room_data_content: ListEditorContent::new("my_room_data_editor")
                .with_add_modal_content(DropDownModal::new(state.person.clone())),
            talk_data_content: ListEditorContent::new("talk_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            relative_data_content: ListEditorContent::new("relative_data_editor")
                .with_add_modal_content(DropDownModal::new(state.person.clone())),
            talk_facility_data_content: ListEditorContent::new("talk_facility_data_editor")
                .with_add_modal_content(DropDownModal::new(state.person.clone())),
            crystal_data_content: ListEditorContent::new("crystal_data_editor")
                .with_add_modal_content(DropDownModal::new(state.chapter.clone())),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Area, "Area");
            ui.selectable_value(&mut self.tab, Tab::Facility, "Facility");
            ui.selectable_value(&mut self.tab, Tab::Demo, "Demo");
            ui.selectable_value(&mut self.tab, Tab::Spawn, "Spawn");
            ui.selectable_value(&mut self.tab, Tab::SpawnSet, "Spawn Set");
            ui.selectable_value(&mut self.tab, Tab::UnityBehavior, "Unity Behavior");
            ui.selectable_value(&mut self.tab, Tab::Fortune, "Fortune Telling");
            ui.selectable_value(&mut self.tab, Tab::Nation, "Nation Data");
            ui.selectable_value(&mut self.tab, Tab::MaterialBonus, "Material Bonus");
            ui.selectable_value(&mut self.tab, Tab::IngredientBonus, "Ingredient Bonus");
            ui.selectable_value(&mut self.tab, Tab::AnimalBonus, "Animal Bonus");
            ui.selectable_value(&mut self.tab, Tab::ItemBonus, "Item Bonus");
            ui.selectable_value(
                &mut self.tab,
                Tab::IngredientBonusGroup,
                "Ingredient Bonus Group",
            );
            ui.selectable_value(&mut self.tab, Tab::AnimalBonusGroup, "Animal Bonus Group");
            ui.selectable_value(&mut self.tab, Tab::MapIconData, "Map Icon");
            ui.selectable_value(&mut self.tab, Tab::MyRoomData, "My Room");
            ui.selectable_value(&mut self.tab, Tab::TalkData, "Talk");
            ui.selectable_value(&mut self.tab, Tab::TalkRelativeData, "Talk Relative");
            ui.selectable_value(&mut self.tab, Tab::TalkFacilityData, "Talk Facility");
            ui.selectable_value(&mut self.tab, Tab::CrystalData, "Crystal");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.facility_data_cache.refresh(state);

        match self.tab {
            Tab::Area => {
                self.hub_area_data_content
                    .left_panel(ctx, &self.hub_area_data, state);
                self.hub_area_data.write(|data| {
                    self.hub_area_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("hub_area_data", selection)
                                .new_section("")
                                .field("AID", |ui, d| ui.add(id_field(&mut d.aid)))
                                .field("MID", |ui, d| {
                                    msbt_key_value_singleline!(ui, state, "hub", d.mid)
                                })
                                .field("Help", |ui, d| {
                                    msbt_key_value_multiline!(ui, state, "hub", d.mid_h)
                                })
                                .field("Condition Chapter", |ui, d| {
                                    state.chapter.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.condition_cid))
                                    })
                                })
                                .default_field("Scene Name", |d| &mut d.scene_name)
                                .default_field("Locator Name", |d| &mut d.locator_name)
                                .default_field("Map Point No", |d| &mut d.map_point_no)
                                .field("Facilities", |ui, d| {
                                    let cache = self.facility_data_cache.get();
                                    ui.add(editable_list(&mut d.facility_aid_list, |_, d, ui| {
                                        ui.add(model_drop_down(cache, &(), d))
                                    }))
                                })
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::Facility => {
                self.hub_facility_data_content
                    .left_panel(ctx, &self.hub_facility_data, state);
                self.hub_facility_data.write(|data| {
                    self.hub_facility_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("hub_facility_data", selection)
                                .new_section("")
                                .field("AID", |ui, d| ui.add(id_field(&mut d.aid)))
                                .field("MID", |ui, d| {
                                    msbt_key_value_singleline!(ui, state, "hub", d.mid)
                                })
                                .field("Condition Chapter", |ui, d| {
                                    state.chapter.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.condition_cid))
                                    })
                                })
                                .default_field("Icon Name", |d| &mut d.icon_name)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::Demo => {
                self.hub_demo_data_content
                    .left_panel(ctx, &self.hub_demo_data, state);
                self.hub_demo_data.write(|data| {
                    self.hub_demo_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("hub_demo_data", selection)
                                .new_section("")
                                .field("Locator", |ui, d| ui.add(id_field(&mut d.locator)))
                                .field("MID", |ui, d| {
                                    msbt_key_value_singleline!(ui, state, "hub", d.mid)
                                })
                                .default_field("Camera Name", |d| &mut d.camera_name)
                                .default_field("Tutorial", |d| &mut d.tutorial)
                                .default_field("Condition", |d| &mut d.condition)
                                .default_field("Timezone", |d| &mut d.timezone)
                                .default_field("Flag Name", |d| &mut d.flag_name)
                                .default_field("Manual Culling Name", |d| {
                                    &mut d.manual_culling_name
                                })
                                .default_field("Lod Bias", |d| &mut d.lod_bias)
                                .default_field("Is Disabled Lod Crossfade Anime", |d| {
                                    &mut d.is_disabled_lod_crossfade_anime
                                })
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::Spawn => {
                self.spawns_content.left_panel(ctx, &self.spawns, state);
                self.spawns.write(|data| {
                    self.spawns_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("spawns", selection)
                            .new_section("")
                            .field("HID", |ui, d| ui.add(id_field(&mut d.hid)))
                            .default_field("Locator", |d| &mut d.locator)
                            .default_field("Parent Locator", |d| &mut d.parent_locator)
                            .default_field("Is Must Child", |d| &mut d.is_must_child)
                            .default_field("Fade Distance", |d| &mut d.fade_distance)
                            .default_field("Priority", |d| &mut d.priority)
                            .field("Chapter", |ui, d| {
                                state.chapter.read(|data| {
                                    ui.add(model_drop_down(data, state, &mut d.chapter))
                                })
                            })
                            .default_field("Phase", |d| &mut d.phase)
                            .default_field("Timezone Flag", |d| &mut d.timezone_flag)
                            .default_field("Flag Name", |d| &mut d.flag_name)
                            .default_field("Any Condition", |d| &mut d.any_condition)
                            .default_field("Content Type", |d| &mut d.content_type)
                            .default_field("Aid", |d| &mut d.aid)
                            .default_field("Talk Pattern", |d| &mut d.talk_pattern)
                            .field("MID", |ui, d| {
                                msbt_key_value_singleline!(ui, state, "hub", d.main_label)
                            })
                            .field("Help", |ui, d| {
                                msbt_key_value_multiline!(ui, state, "hub", d.help_label)
                            })
                            .default_field("Script Name", |d| &mut d.script_name)
                            .default_field("Access Type", |d| &mut d.access_type)
                            .default_field("Idle Body Name", |d| &mut d.idle_body_name)
                            .default_field("Idle Face Name", |d| &mut d.idle_face_name)
                            .default_field("Idle Type", |d| &mut d.idle_type)
                            .default_field("Disabled Anim", |d| &mut d.disabled_anim)
                            .default_field("Disabled Talk", |d| &mut d.disabled_talk)
                            .default_field("Ignore Story", |d| &mut d.ignore_story)
                            .default_field("Bind", |d| &mut d.bind)
                            .default_field("Dispos Type", |d| &mut d.dispos_type)
                            .default_field("Access Angle", |d| &mut d.access_angle)
                            .default_field("Move Name", |d| &mut d.move_name)
                            .field("Area", |ui, d| {
                                state.hub_area_data.read(|data| {
                                    ModelDropDown::default()
                                        .transform(&area_key_transform, &area_key_reverse_transform)
                                        .show(ui, data, state, &mut d.area)
                                })
                            })
                            .default_field("Layer", |d| &mut d.layer)
                            .default_field("Disabled Mini Map", |d| &mut d.disabled_mini_map)
                            .default_field("Weight", |d| &mut d.weight)
                            .default_field("Optimize Type", |d| &mut d.optimize_type)
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::SpawnSet => {
                self.random_sets_content
                    .left_panel(ctx, &self.random_sets, state);
                self.random_sets.write(|data| {
                    self.random_sets_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("random_sets", selection)
                                .new_section("")
                                .field("Id", |ui, d| {
                                    ui.vertical(|ui| {
                                        let mut response = state.person.read(|data| {
                                            ui.add(model_drop_down(data, state, &mut d.id))
                                        });
                                        response |= state.god.read(|data| {
                                            ui.add(model_drop_down(data, state, &mut d.id))
                                        });
                                        response |= state.item.read(|data| {
                                            ui.add(model_drop_down(data, state, &mut d.id))
                                        });
                                        response |= ui.text_edit_singleline(&mut d.id);
                                        response
                                    })
                                    .inner
                                })
                                .default_field("Rate", |d| &mut d.rate)
                                .default_field("Count", |d| &mut d.count)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::UnityBehavior => {
                self.unity_behavior_content
                    .left_panel(ctx, &self.unity_behavior, state);
                self.unity_behavior.write(|data| {
                    self.unity_behavior_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("unity_behavior", selection)
                                .new_section("")
                                .default_field("Move Type", |d| &mut d.move_type)
                                .default_field("Locator", |d| &mut d.locator)
                                .default_field("Body Name", |d| &mut d.body_name)
                                .default_field("Face Name", |d| &mut d.face_name)
                                .default_field("Is Turn", |d| &mut d.is_turn)
                                .default_field("Move Sec", |d| &mut d.move_sec)
                                .default_field("Move Speed", |d| &mut d.move_speed)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::Fortune => {
                self.fortune_telling_data_content.left_panel(
                    ctx,
                    &self.fortune_telling_data,
                    state,
                );
                self.fortune_telling_data.write(|data| {
                    self.fortune_telling_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("fortune_telling_data", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .default_field("Texture Name", |d| &mut d.texture_name)
                                .default_field("Primary Text", |d| &mut d.primary_text)
                                .default_field("Primary Text Ex", |d| &mut d.primary_text_ex)
                                .default_field("Reverse Text", |d| &mut d.reverse_text)
                                .default_field("Reverse Text Ex", |d| &mut d.reverse_text_ex)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::Nation => {
                self.nation_data_content
                    .left_panel(ctx, &self.nation_data, state);
                self.nation_data.write(|data| {
                    self.nation_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("nation_data", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .field("Name", |ui, d| {
                                    msbt_key_value_singleline!(ui, state, "hub", d.name)
                                })
                                .field("Chapter", |ui, d| {
                                    state.chapter.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.chapter))
                                    })
                                })
                                .default_field("Is Not Level", |d| &mut d.is_not_level)
                                .default_field("Symbol Texture", |d| &mut d.symbol_texture)
                                .default_field("Level Info", |d| &mut d.level_info)
                                .default_field("Foodstuff Info", |d| &mut d.foodstuff_info)
                                .default_field("Animal Info", |d| &mut d.animal_info)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::MaterialBonus => {
                self.material_bonuses_content
                    .left_panel(ctx, &self.material_bonuses, state);
                self.material_bonuses.write(|data| {
                    self.material_bonuses_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("material_bonuses", selection)
                                .new_section("")
                                .default_field("Cost", |d| &mut d.cost)
                                .field("Bonus Name", |ui, d| {
                                    msbt_key_value_singleline!(ui, state, "hub", d.bonus_name)
                                })
                                .default_field("Bonus Item", |d| &mut d.bonus_item)
                                .default_field("Bonus Food", |d| &mut d.bonus_food)
                                .default_field("Bonus Animal", |d| &mut d.bonus_animal)
                                .field("Bonus Accessory", |ui, d| {
                                    state.accessory.read(|data| {
                                        ui.add(model_drop_down(
                                            data,
                                            state,
                                            &mut d.bonus_accessory_aid,
                                        ))
                                    })
                                })
                                .field("Bonus Iron", |ui, d| {
                                    iron_field_i8(ui, state, &mut d.bonus_iron)
                                })
                                .field("Bonus Steel", |ui, d| {
                                    steel_field(ui, state, &mut d.bonus_steel)
                                })
                                .field("Bonus Silver", |ui, d| {
                                    silver_field(ui, state, &mut d.bonus_silver)
                                })
                                .field("Bonus Bond Fragments", |ui, d| {
                                    bond_fragment_field(ui, state, &mut d.bonus_piece_of_bond)
                                })
                                .field("Iron", |ui, d| iron_field_i8(ui, state, &mut d.iron))
                                .field("Steel", |ui, d| steel_field(ui, state, &mut d.steel))
                                .field("Silver", |ui, d| silver_field(ui, state, &mut d.silver))
                                .field("Bond Fragments", |ui, d| {
                                    bond_fragment_field(ui, state, &mut d.piece_of_bond)
                                })
                                .default_field("Gold Enemy Rate", |d| &mut d.gold_enemy_rate)
                                .default_field("Exp Enemy Rate", |d| &mut d.exp_enemy_rate)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::IngredientBonus => {
                self.ingredient_bonuses_content
                    .left_panel(ctx, &self.ingredient_bonuses, state);
                self.ingredient_bonuses.write(|data| {
                    self.ingredient_bonuses_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("ingredient_bonuses", selection)
                                .new_section("")
                                .field("Foodstuff", |ui, d| {
                                    state.item.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.foodstuff))
                                    })
                                })
                                .default_field("Lv 1", |d| &mut d.lv_1)
                                .default_field("Lv 2", |d| &mut d.lv_2)
                                .default_field("Lv 3", |d| &mut d.lv_3)
                                .default_field("Lv 4", |d| &mut d.lv_4)
                                .default_field("Lv 5", |d| &mut d.lv_5)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::AnimalBonus => {
                self.animal_bonuses_content
                    .left_panel(ctx, &self.animal_bonuses, state);
                self.animal_bonuses.write(|data| {
                    self.animal_bonuses_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("animal_bonuses", selection)
                                .new_section("")
                                .field("Animal", |ui, d| {
                                    state.animal.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.anid))
                                    })
                                })
                                .default_field("Appear Rate Lv 1", |d| &mut d.appear_rate_lv_1)
                                .default_field("Appear Rate Lv 2", |d| &mut d.appear_rate_lv_2)
                                .default_field("Appear Rate Lv 3", |d| &mut d.appear_rate_lv_3)
                                .default_field("Appear Rate Lv 4", |d| &mut d.appear_rate_lv_4)
                                .default_field("Appear Rate Lv 5", |d| &mut d.appear_rate_lv_5)
                                .default_field("Capture Level", |d| &mut d.capture_level)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::ItemBonus => {
                self.item_bonuses_content
                    .left_panel(ctx, &self.item_bonuses, state);
                self.item_bonuses.write(|data| {
                    self.item_bonuses_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("item_bonuses", selection)
                                .new_section("")
                                .field("Item", |ui, d| {
                                    state.item.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.item_id))
                                    })
                                })
                                .default_field("Num", |d| &mut d.num)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::IngredientBonusGroup => {
                self.ingredient_bonus_groups_content.left_panel(
                    ctx,
                    &self.ingredient_bonus_groups,
                    state,
                );
                self.ingredient_bonus_groups.write(|data| {
                    self.ingredient_bonus_groups_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("ingredient_bonus_groups", selection)
                                .new_section("")
                                .field("Foodstuff", |ui, d| {
                                    state.item.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.foodstuff))
                                    })
                                })
                                .default_field("Num", |d| &mut d.num)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::AnimalBonusGroup => {
                self.animal_bonus_groups_content
                    .left_panel(ctx, &self.animal_bonus_groups, state);
                self.animal_bonus_groups.write(|data| {
                    self.animal_bonus_groups_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("animal_bonus_groups", selection)
                                .new_section("")
                                .field("Animal", |ui, d| {
                                    state.animal.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.animal_id))
                                    })
                                })
                                .default_field("Num", |d| &mut d.num)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::MapIconData => {
                self.map_icon_data_content
                    .left_panel(ctx, &self.map_icon_data, state);
                self.map_icon_data.write(|data| {
                    self.map_icon_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("map_icon_data", selection)
                                .new_section("")
                                .field("Dispos Name", |ui, d| ui.add(id_field(&mut d.dispos_name)))
                                .default_field("Icon Name", |d| &mut d.icon_name)
                                .default_field("Large Scale", |d| &mut d.large_scale)
                                .default_field("Small Scale", |d| &mut d.small_scale)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::MyRoomData => {
                self.my_room_data_content
                    .left_panel(ctx, &self.my_room_data, state);
                self.my_room_data.write(|data| {
                    self.my_room_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("my_room_data", selection)
                                .new_section("")
                                .field("PID", |ui, d| ui.add(id_field(&mut d.pid)))
                                .default_field("C 1", |d| &mut d.c_1)
                                .default_field("C 2", |d| &mut d.c_2)
                                .default_field("B 1", |d| &mut d.b_1)
                                .default_field("B 2", |d| &mut d.b_2)
                                .default_field("A 1", |d| &mut d.a_1)
                                .default_field("A 2", |d| &mut d.a_2)
                                .default_field("S 1", |d| &mut d.s_1)
                                .default_field("S 2", |d| &mut d.s_2)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::TalkData => {
                self.talk_data_content
                    .left_panel(ctx, &self.talk_data, state);
                self.talk_data.write(|data| {
                    self.talk_data_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("talk_data", selection)
                            .new_section("")
                            .field("KRID", |ui, d| ui.add(id_field(&mut d.krid)))
                            .default_field("Count", |d| &mut d.count)
                            .default_field("Args 0", |d| &mut d.args_0)
                            .default_field("Args 1", |d| &mut d.args_1)
                            .field("Item", |ui, d| {
                                state
                                    .item
                                    .read(|data| ui.add(model_drop_down(data, state, &mut d.item)))
                            })
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::TalkRelativeData => {
                self.relative_data_content
                    .left_panel(ctx, &self.relative_data, state);
                self.relative_data.write(|data| {
                    self.relative_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("relative_data", selection)
                                .new_section("")
                                .field("PID", |ui, d| ui.add(id_field(&mut d.pid)))
                                .field("Person 1", |ui, d| {
                                    state.person.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.pid_1))
                                    })
                                })
                                .field("Person 2", |ui, d| {
                                    state.person.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.pid_2))
                                    })
                                })
                                .field("Person 3", |ui, d| {
                                    state.person.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.pid_3))
                                    })
                                })
                                .field("Person 4", |ui, d| {
                                    state.person.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.pid_4))
                                    })
                                })
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::TalkFacilityData => {
                self.talk_facility_data_content
                    .left_panel(ctx, &self.talk_facility_data, state);
                self.talk_facility_data.write(|data| {
                    self.talk_facility_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("talk_facility_data", selection)
                                .new_section("")
                                .field("Pattern", |ui, d| ui.add(id_field(&mut d.pattern)))
                                .field("Person", |ui, d| {
                                    state.person.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.pid))
                                    })
                                })
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::CrystalData => {
                self.crystal_data_content
                    .left_panel(ctx, &self.crystal_data, state);
                self.crystal_data.write(|data| {
                    self.crystal_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("crystal_data", selection)
                                .new_section("")
                                .field("CID", |ui, d| ui.add(id_field(&mut d.cid)))
                                .default_field("Count", |d| &mut d.count)
                                .show(ui)
                                .changed()
                        })
                });
            }
        }
    }
}
