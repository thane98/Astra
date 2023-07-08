use astra_types::AssetDef;

use crate::widgets::model_drop_down;
use crate::{
    editable_list, f32_drag, i8_drag, rgb_color_picker, AssetTableSheet, EditorState,
    ListEditorContent, PropertyGrid,
};

pub struct AssetTableEditor {
    asset_table: AssetTableSheet,
    content: ListEditorContent<Vec<AssetDef>, AssetDef>,
}

impl AssetTableEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            asset_table: state.asset_table.clone(),
            content: ListEditorContent::new("asset_def_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.content.side_panel(ctx, &self.asset_table, &());

        self.asset_table.write(|data| {
            self.content.content(ctx, data, |ui, assettable| {
                PropertyGrid::new("asset_def", assettable)
                    .new_section("Core")
                    .field("Out", |ui, def| ui.text_edit_singleline(&mut def.out))
                    .field("Comment", |ui, def| {
                        ui.text_edit_singleline(&mut def.comment)
                    })
                    .field("Preset Name", |ui, def| {
                        ui.text_edit_singleline(&mut def.preset_name)
                    })
                    .field("Mode", |ui, def| ui.add(i8_drag(&mut def.mode)))
                    .field("Conditions", |ui, def| {
                        ui.add(editable_list(&mut def.conditions, |_, value, ui| {
                            ui.text_edit_singleline(value)
                        }))
                    })
                    .new_section("Models")
                    .field("Body Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.body_model)
                    })
                    .field("Dress Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.dress_model)
                    })
                    .field("Head Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.head_model)
                    })
                    .field("Hair Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.hair_model)
                    })
                    .field("Ride Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.ride_model)
                    })
                    .field("Ride Dress Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.ride_dress_model)
                    })
                    .new_section("Accessories")
                    .field("Accessory 1 Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_1_model)
                    })
                    .field("Accessory 1 Locator", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_1_locator)
                    })
                    .field("Accessory 2 Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_2_model)
                    })
                    .field("Accessory 2 Locator", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_2_locator)
                    })
                    .field("Accessory 3 Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_3_model)
                    })
                    .field("Accessory 3 Locator", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_3_locator)
                    })
                    .field("Accessory 4 Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_4_model)
                    })
                    .field("Accessory 4 Locator", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_4_locator)
                    })
                    .field("Accessory 5 Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_5_model)
                    })
                    .field("Accessory 5 Locator", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_5_locator)
                    })
                    .field("Accessory 6 Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_6_model)
                    })
                    .field("Accessory 6 Locator", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_6_locator)
                    })
                    .field("Accessory 7 Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_7_model)
                    })
                    .field("Accessory 7 Locator", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_1_locator)
                    })
                    .field("Accessory 8 Model", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_8_model)
                    })
                    .field("Accessory 8 Locator", |ui, def| {
                        ui.text_edit_singleline(&mut def.acc_8_locator)
                    })
                    .new_section("Colors")
                    .field("Hair Color", |ui, def| {
                        ui.add(rgb_color_picker(
                            &mut def.hair_r,
                            &mut def.hair_g,
                            &mut def.hair_b,
                        ))
                    })
                    .field("Grad Color", |ui, def| {
                        ui.add(rgb_color_picker(
                            &mut def.grad_r,
                            &mut def.grad_g,
                            &mut def.grad_b,
                        ))
                    })
                    .field("Skin Color", |ui, def| {
                        ui.add(rgb_color_picker(
                            &mut def.skin_r,
                            &mut def.skin_g,
                            &mut def.skin_b,
                        ))
                    })
                    .field("Toon Color", |ui, def| {
                        ui.add(rgb_color_picker(
                            &mut def.toon_r,
                            &mut def.toon_g,
                            &mut def.toon_b,
                        ))
                    })
                    .field("Mask Color 100", |ui, def| {
                        ui.add(rgb_color_picker(
                            &mut def.mask_color_100_r,
                            &mut def.mask_color_100_g,
                            &mut def.mask_color_100_b,
                        ))
                    })
                    .field("Mask Color 75", |ui, def| {
                        ui.add(rgb_color_picker(
                            &mut def.mask_color_075_r,
                            &mut def.mask_color_075_g,
                            &mut def.mask_color_075_b,
                        ))
                    })
                    .field("Mask Color 50", |ui, def| {
                        ui.add(rgb_color_picker(
                            &mut def.mask_color_050_r,
                            &mut def.mask_color_050_g,
                            &mut def.mask_color_050_b,
                        ))
                    })
                    .field("Mask Color 25", |ui, def| {
                        ui.add(rgb_color_picker(
                            &mut def.mask_color_025_r,
                            &mut def.mask_color_025_g,
                            &mut def.mask_color_025_b,
                        ))
                    })
                    .new_section("Animation")
                    .field("Body Anim", |ui, def| {
                        state.anim_set.read(|data| {
                            ui.add(model_drop_down(data, &(), &mut def.body_anim))
                        })
                    })
                    .field("Info Anim", |ui, def| {
                        state.anim_set.read(|data| {
                            ui.add(model_drop_down(data, &(), &mut def.info_anim))
                        })
                    })
                    .field("Talk Anim", |ui, def| {
                        state.anim_set.read(|data| {
                            ui.add(model_drop_down(data, &(), &mut def.talk_anim))
                        })
                    })
                    .field("Demo Anim", |ui, def| {
                        state.anim_set.read(|data| {
                            ui.add(model_drop_down(data, &(), &mut def.demo_anim))
                        })
                    })
                    .field("Hub Anim", |ui, def| {
                        state.anim_set.read(|data| {
                            ui.add(model_drop_down(data, &(), &mut def.hub_anim))
                        })
                    })
                    .new_section("Scale")
                    .field("Scale (All)", |ui, def| {
                        ui.add(f32_drag(&mut def.scale_all))
                    })
                    .field("Scale (Head)", |ui, def| {
                        ui.add(f32_drag(&mut def.scale_head))
                    })
                    .field("Scale (Neck)", |ui, def| {
                        ui.add(f32_drag(&mut def.scale_neck))
                    })
                    .field("Scale (Torso)", |ui, def| {
                        ui.add(f32_drag(&mut def.scale_torso))
                    })
                    .field("Scale (Shoulders)", |ui, def| {
                        ui.add(f32_drag(&mut def.scale_shoulders))
                    })
                    .field("Scale (Arms)", |ui, def| {
                        ui.add(f32_drag(&mut def.scale_arms))
                    })
                    .field("Scale (Hands)", |ui, def| {
                        ui.add(f32_drag(&mut def.scale_hands))
                    })
                    .field("Scale (Legs)", |ui, def| {
                        ui.add(f32_drag(&mut def.scale_legs))
                    })
                    .field("Scale (Feet)", |ui, def| {
                        ui.add(f32_drag(&mut def.scale_feet))
                    })
                    .new_section("Volume")
                    .field("Volume (Arms)", |ui, def| {
                        ui.add(f32_drag(&mut def.volume_arms))
                    })
                    .field("Volume (Legs)", |ui, def| {
                        ui.add(f32_drag(&mut def.volume_legs))
                    })
                    .field("Volume (Bust)", |ui, def| {
                        ui.add(f32_drag(&mut def.volume_bust))
                    })
                    .field("Volume (Abdomen)", |ui, def| {
                        ui.add(f32_drag(&mut def.volume_abdomen))
                    })
                    .field("Volume (Torso)", |ui, def| {
                        ui.add(f32_drag(&mut def.volume_torso))
                    })
                    .field("Volume Scale (Arms)", |ui, def| {
                        ui.add(f32_drag(&mut def.volume_scale_arms))
                    })
                    .field("Volume Scale (Legs)", |ui, def| {
                        ui.add(f32_drag(&mut def.volume_scale_legs))
                    })
                    .new_section("Map Scale")
                    .field("Map Scale (All)", |ui, def| {
                        ui.add(f32_drag(&mut def.map_scale_all))
                    })
                    .field("Map Scale (Head)", |ui, def| {
                        ui.add(f32_drag(&mut def.map_scale_head))
                    })
                    .field("Map Scale (Wings)", |ui, def| {
                        ui.add(f32_drag(&mut def.map_scale_wing))
                    })
                    .new_section("Uncategorized")
                    .field("Voice", |ui, def| ui.text_edit_singleline(&mut def.voice))
                    .field("Foot Step", |ui, def| {
                        ui.text_edit_singleline(&mut def.foot_step)
                    })
                    .field("Material", |ui, def| {
                        ui.text_edit_singleline(&mut def.material)
                    })
                    .field("Left Hand", |ui, def| {
                        ui.text_edit_singleline(&mut def.left_hand)
                    })
                    .field("Right Hand", |ui, def| {
                        ui.text_edit_singleline(&mut def.right_hand)
                    })
                    .field("Trail", |ui, def| ui.text_edit_singleline(&mut def.trail))
                    .field("Magic", |ui, def| ui.text_edit_singleline(&mut def.magic))
                    .show(ui)
                    .changed()
            })
        });
    }
}
