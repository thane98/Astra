use astra_types::{Accessory, ItemBook};
use indexmap::IndexMap;

use crate::{
    gold_field, id_field, iron_field_i8, keyed_add_modal_content, model_drop_down,
    msbt_key_value_multiline, msbt_key_value_singleline, silver_field, steel_field, AccessorySheet,
    AccessorySheetRetriever, CachedView, EditorState, ListEditorContent, PropertyGrid,
};

pub struct AccessoryEditor {
    accessory: AccessorySheet,
    content: ListEditorContent<IndexMap<String, Accessory>, Accessory>,
    cache: CachedView<AccessorySheetRetriever, ItemBook, Accessory>,
}

impl AccessoryEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            accessory: state.accessory.clone(),
            cache: CachedView::new(state.accessory.clone(), state),
            content: ListEditorContent::new("accessory_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.content.select(index);
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState) {
        self.content.left_panel(ctx, &self.accessory, state);

        self.cache.refresh(state);

        self.accessory.write(|data| {
            self.content.content(ctx, data, |ui, accessory| {
                PropertyGrid::new("accessory", accessory)
                    .new_section("Data")
                    .field("AID", |ui, acc| ui.add(id_field(&mut acc.aid)))
                    .field("Name", |ui, acc| {
                        msbt_key_value_singleline!(ui, state, "accessory", acc.name)
                    })
                    .field("Help", |ui, acc| {
                        msbt_key_value_multiline!(ui, state, "accessory", acc.help)
                    })
                    .field("Name (M)", |ui, acc| {
                        msbt_key_value_singleline!(ui, state, "accessory", acc.name_m)
                    })
                    .field("Help (M)", |ui, acc| {
                        msbt_key_value_multiline!(ui, state, "accessory", acc.help_m)
                    })
                    .field("Name (F)", |ui, acc| {
                        msbt_key_value_singleline!(ui, state, "accessory", acc.name_f)
                    })
                    .field("Help (F)", |ui, acc| {
                        msbt_key_value_multiline!(ui, state, "accessory", acc.help_f)
                    })
                    .default_field("First", |acc| &mut acc.first)
                    .default_field("Amiibo", |acc| &mut acc.amiibo)
                    .default_field("Asset", |acc| &mut acc.asset)
                    .field("Condition", |ui, acc| {
                        state.chapter.read(|data| {
                            ui.add(model_drop_down(data, state, &mut acc.condtion_cid))
                        })
                    })
                    .field("GID", |ui, acc| {
                        state
                            .god
                            .read(|data| ui.add(model_drop_down(data, state, &mut acc.gid)))
                    })
                    .field("Price", |ui, acc| gold_field(ui, state, &mut acc.price))
                    .field("Iron", |ui, d| iron_field_i8(ui, state, &mut d.iron))
                    .field("Steel", |ui, d| steel_field(ui, state, &mut d.steel))
                    .field("Silver", |ui, d| silver_field(ui, state, &mut d.silver))
                    .default_field("Mask", |acc| &mut acc.mask)
                    .show(ui)
                    .changed()
            })
        });
    }
}
