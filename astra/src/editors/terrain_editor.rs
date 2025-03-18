use astra_types::{TerrainBook, TerrainData};
use egui::DragValue;
use indexmap::IndexMap;

use crate::widgets::{
    id_field, keyed_add_modal_content, terrain_destroyer_drop_down, terrain_prohibition_drop_down,
};
use crate::{
    model_drop_down, msbt_key_value_singleline, rgb_color_picker, CachedView, EditorState,
    ListEditorContent, PropertyGrid, TerrainDataSheet, TerrainDataSheetRetriever,
};

pub struct TerrainDataEditor {
    terrain: TerrainDataSheet,
    content: ListEditorContent<IndexMap<String, TerrainData>, TerrainData, EditorState>,
    cache: CachedView<TerrainDataSheetRetriever, TerrainBook, TerrainData>,
}

impl TerrainDataEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            terrain: state.terrain.clone(),
            content: ListEditorContent::new("terrain_editor")
                .with_add_modal_content(keyed_add_modal_content),
            cache: CachedView::new(state.terrain.clone(), state),
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.content.select(index);
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState) {
        self.content.left_panel(ctx, &self.terrain, state);

        self.cache.refresh(state);

        self.terrain.write(|data| {
            self.content.content(ctx, data, |ui, terraindata| {
                PropertyGrid::new("terrain", terraindata)
                    .new_section("Data")
                    .field("TID", |ui, tile| ui.add(id_field(&mut tile.tid)))
                    .field("Name", |ui, tile| {
                        msbt_key_value_singleline!(ui, state, "gamedata", tile.name)
                    })
                    .field("Cost Name", |ui, tile| {
                        ui.text_edit_singleline(&mut tile.cost_name)
                    })
                    .field("Layer", |ui, tile| ui.add(DragValue::new(&mut tile.layer)))
                    .field("Prohibition", |ui, tile| {
                        ui.add(terrain_prohibition_drop_down(&mut tile.prohibition))
                    })
                    .field("Sight", |ui, tile| ui.add(DragValue::new(&mut tile.sight)))
                    .field("Destroyer", |ui, tile| {
                        ui.add(terrain_destroyer_drop_down(&mut tile.destroyer))
                    })
                    .field("HP (N)", |ui, tile| ui.add(DragValue::new(&mut tile.hp_n)))
                    .field("HP (H)", |ui, tile| ui.add(DragValue::new(&mut tile.hp_h)))
                    .field("HP (L)", |ui, tile| ui.add(DragValue::new(&mut tile.hp_l)))
                    .field("Defense", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.defense))
                    })
                    .field("Avoid", |ui, tile| ui.add(DragValue::new(&mut tile.avoid)))
                    .field("Player Defense", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.player_defense))
                    })
                    .field("Enemy Defense", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.enemy_defense))
                    })
                   .field("Player Avoid", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.player_avoid))
                    })
                    .field("Enemy Avoid", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.enemy_avoid))
                    })
                    .field("Heal", |ui, tile| ui.add(DragValue::new(&mut tile.heal)))
                    .field("Life", |ui, tile| ui.add(DragValue::new(&mut tile.life)))
                    .field("Move Cost", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.move_cost))
                    })
                    .field("Fly Cost", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.fly_cost))
                    })
                    .field("Move First", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.move_first))
                    })
                    .field("Offset", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.offset))
                    })
                    .field("Put Effect", |ui, tile| {
                        ui.text_edit_singleline(&mut tile.put_effect)
                    })
                    .field("Minimap", |ui, tile| {
                        ui.text_edit_singleline(&mut tile.minimap)
                    })
                    .field("Cannon Skill", |ui, tile| {
                        state.skill.read(|data| {
                            ui.add(model_drop_down(data, state, &mut tile.cannon_skill))
                        })
                    })
                    .field("Cannon Shells (N)", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.cannon_shells_n))
                    })
                    .field("Cannon Shells (H)", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.cannon_shells_h))
                    })
                    .field("Cannon Shells (L)", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.cannon_shells_l))
                    })
                    .field("Change TID", |ui, tile| {
                        ui.add(model_drop_down(self.cache.get(), &(), &mut tile.change_tid))
                    })
                    .field("Change Encount", |ui, tile| {
                        ui.add(model_drop_down(
                            self.cache.get(),
                            &(),
                            &mut tile.change_encount,
                        ))
                    })
                    .field("Command", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.command))
                    })
                    .field("Height", |ui, tile| {
                        ui.add(DragValue::new(&mut tile.height))
                    })
                    .field("Color (RGB)", |ui, tile| {
                        ui.add(rgb_color_picker(
                            &mut tile.color_r,
                            &mut tile.color_g,
                            &mut tile.color_b,
                        ))
                    })
                    .show(ui)
                    .changed()
            })
        });
    }
}
