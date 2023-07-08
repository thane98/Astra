use astra_types::{Accessory, ItemBook};
use indexmap::IndexMap;

use crate::{
    model_drop_down, msbt_key_value_multiline, msbt_key_value_singleline, AccessorySheet,
    AccessorySheetRetriever, CachedView, EditorState, ListEditorContent, PropertyGrid,
    keyed_add_modal_content, id_field
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

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState) {
        self.content.side_panel(ctx, &self.accessory, state);

        self.cache.refresh(state);

        self.accessory.write(|data| {
            self.content.content(ctx, data, |ui, accessory| {
                PropertyGrid::new("accessory", accessory)
                    .new_section("Data")
                    .field("AID", |ui, acc| {
                        ui.add(id_field(&mut acc.aid))
                    })
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
                    .default_field("Price", |acc| &mut acc.price)
                    .default_field("Iron", |acc| &mut acc.iron)
                    .default_field("Steel", |acc| &mut acc.steel)
                    .default_field("Silver", |acc| &mut acc.silver)
                    .default_field("Mask", |acc| &mut acc.mask)
                    .show(ui)
                    .changed()
            })
        });
    }
}
