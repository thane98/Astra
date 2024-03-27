use std::borrow::Cow;

use astra_types::{AnimalBook, AnimalData};
use indexmap::IndexMap;

use crate::{
    id_field, keyed_add_modal_content, model_drop_down, msbt_key_value_multiline,
    msbt_key_value_singleline, sheet_retriever, standard_keyed_display, EditorState, KeyedViewItem,
    ListEditorContent, PropertyGrid, ViewItem,
};

sheet_retriever!(Animal, AnimalBook, animals, IndexMap<String, AnimalData>);

impl ViewItem for AnimalData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, anid, name)
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
            .get_system(&self.icon_name)
            .map(|texture| (texture, 0.5))
    }
}

impl KeyedViewItem for AnimalData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.anid)
    }

    fn set_key(&mut self, key: String) {
        self.anid = key;
    }
}

pub struct AnimalEditor {
    sheet: AnimalSheet,
    content: ListEditorContent<IndexMap<String, AnimalData>, AnimalData, EditorState>,
}

impl AnimalEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            sheet: state.animal.clone(),
            content: ListEditorContent::new("animal")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.content.left_panel(ctx, &self.sheet, state);

        self.sheet.write(|data| {
            self.content.content(ctx, data, |ui, selection| {
                PropertyGrid::new("animal", selection)
                    .new_section("")
                    .field("ANID", |ui, d| ui.add(id_field(&mut d.anid)))
                    .field("Name", |ui, d| {
                        msbt_key_value_singleline!(ui, state, "person", d.name)
                    })
                    .field("Name", |ui, d| {
                        msbt_key_value_multiline!(ui, state, "person", d.help)
                    })
                    .default_field("Icon Name", |d| &mut d.icon_name)
                    .default_field("Category", |d| &mut d.category)
                    .default_field("Radius", |d| &mut d.radius)
                    .default_field("Nid", |d| &mut d.nid)
                    .field("Person", |ui, d| {
                        state
                            .person
                            .read(|data| ui.add(model_drop_down(data, state, &mut d.pid)))
                    })
                    .field("Item", |ui, d| {
                        state
                            .item
                            .read(|data| ui.add(model_drop_down(data, state, &mut d.item)))
                    })
                    .default_field("Rare", |d| &mut d.rare)
                    .default_field("M001", |d| &mut d.m_001)
                    .default_field("M002", |d| &mut d.m_002)
                    .default_field("M003", |d| &mut d.m_003)
                    .default_field("M004", |d| &mut d.m_004)
                    .default_field("M005", |d| &mut d.m_005)
                    .default_field("M006", |d| &mut d.m_006)
                    .default_field("M007", |d| &mut d.m_007)
                    .default_field("M008", |d| &mut d.m_008)
                    .default_field("M009", |d| &mut d.m_009)
                    .default_field("M010", |d| &mut d.m_010)
                    .default_field("M011", |d| &mut d.m_011)
                    .default_field("M012", |d| &mut d.m_012)
                    .default_field("M013", |d| &mut d.m_013)
                    .default_field("M014", |d| &mut d.m_014)
                    .default_field("M015", |d| &mut d.m_015)
                    .default_field("M016", |d| &mut d.m_016)
                    .default_field("M017", |d| &mut d.m_017)
                    .default_field("M018", |d| &mut d.m_018)
                    .default_field("M019", |d| &mut d.m_019)
                    .default_field("M020", |d| &mut d.m_020)
                    .default_field("M021", |d| &mut d.m_021)
                    .default_field("M022", |d| &mut d.m_022)
                    .default_field("M023", |d| &mut d.m_023)
                    .default_field("M024", |d| &mut d.m_024)
                    .default_field("M025", |d| &mut d.m_025)
                    .default_field("M026", |d| &mut d.m_026)
                    .default_field("S001", |d| &mut d.s_001)
                    .default_field("S002", |d| &mut d.s_002)
                    .default_field("S003", |d| &mut d.s_003)
                    .default_field("S004", |d| &mut d.s_004)
                    .default_field("S005", |d| &mut d.s_005)
                    .default_field("S006", |d| &mut d.s_006)
                    .default_field("S007", |d| &mut d.s_007)
                    .default_field("S008", |d| &mut d.s_008)
                    .default_field("S009", |d| &mut d.s_009)
                    .default_field("S010", |d| &mut d.s_010)
                    .default_field("S011", |d| &mut d.s_011)
                    .default_field("S012", |d| &mut d.s_012)
                    .default_field("S013", |d| &mut d.s_013)
                    .default_field("S014", |d| &mut d.s_014)
                    .default_field("S015", |d| &mut d.s_015)
                    .default_field("G001", |d| &mut d.g_001)
                    .default_field("G002", |d| &mut d.g_002)
                    .default_field("G003", |d| &mut d.g_003)
                    .default_field("G004", |d| &mut d.g_004)
                    .default_field("G005", |d| &mut d.g_005)
                    .default_field("G006", |d| &mut d.g_006)
                    .default_field("E001", |d| &mut d.e_001)
                    .default_field("E002", |d| &mut d.e_002)
                    .default_field("E003", |d| &mut d.e_003)
                    .default_field("E004", |d| &mut d.e_004)
                    .default_field("E005", |d| &mut d.e_005)
                    .default_field("E006", |d| &mut d.e_006)
                    .show(ui)
                    .changed()
            })
        });
    }
}
