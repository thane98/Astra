use std::borrow::Cow;

use astra_types::{AmiiboBook, AmiiboData};
use indexmap::IndexMap;

use crate::{
    editable_list, id_field, model_drop_down, sheet_retriever, EditorState, KeyedViewItem,
    ListEditorContent, PropertyGrid, ViewItem,
};

sheet_retriever!(Amiibo, AmiiboBook, amiibo, IndexMap<String, AmiiboData>);

impl ViewItem for AmiiboData {
    type Dependencies = ();

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.numbering_id)
    }
}

impl KeyedViewItem for AmiiboData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.numbering_id)
    }

    fn set_key(&mut self, key: String) {
        self.numbering_id = key;
    }
}

pub struct AmiiboEditor {
    sheet: AmiiboSheet,
    content: ListEditorContent<IndexMap<String, AmiiboData>, AmiiboData>,
}

impl AmiiboEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            sheet: state.amiibo.clone(),
            content: ListEditorContent::new("amiibo_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.content.left_panel(ctx, &self.sheet, &());

        self.sheet.write(|data| {
            self.content.content(ctx, data, |ui, selection| {
                PropertyGrid::new("amiibo", selection)
                    .new_section("")
                    .field("Numbering Id", |ui, d| {
                        ui.add(id_field(&mut d.numbering_id))
                    })
                    .field("Items", |ui, d| {
                        ui.add(editable_list(&mut d.items, |_, item, ui| {
                            state
                                .item
                                .read(|data| ui.add(model_drop_down(data, state, item)))
                        }))
                    })
                    .default_field("Aid", |d| &mut d.aid)
                    .default_field("Bgm", |d| &mut d.bgm)
                    .default_field("Ticket Num", |d| &mut d.ticket_num)
                    .default_field("Kizuna Num", |d| &mut d.kizuna_num)
                    .show(ui)
                    .changed()
            })
        });
    }
}
