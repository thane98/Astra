use std::borrow::Cow;

use egui::{Grid, Ui};
use egui_extras::Size;
use indexmap::IndexMap;

use crate::{
    editable_list, editor_tab_strip, id_field, keyed_add_modal_content, model_drop_down,
    msbt_key_value_multiline, msbt_key_value_singleline, optional_image, rgb_color_picker,
    ring_rank_drop_down, sheet_retriever, standard_keyed_display, stat_column_headers_no_sight,
    stats_row_no_sight, DecorationKind, EditorState, GroupEditorContent, GroupViewItem,
    KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem,
};

use astra_types::{RingBook, RingCleaningVoiceBook, RingData, RingPolishVoiceData};

sheet_retriever!(RingData, RingBook, ring_data, IndexMap<String, RingData>);

impl ViewItem for RingData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, rnid, name)
    }

    fn decorated(kind: DecorationKind<'_>) -> bool {
        matches!(
            kind,
            DecorationKind::DropDown | DecorationKind::List | DecorationKind::Other("portrait")
        )
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        match kind {
            DecorationKind::DropDown | DecorationKind::List => {
                let mut cache = dependencies.texture_cache.borrow_mut();
                let texture = cache.get_godring(match self.rank {
                    1 => "CommonSilver",
                    2 => "CommonGold",
                    3 => "CommonPlatinum",
                    _ => "CommonBronze",
                })?;
                Some((texture, 0.5))
            }
            DecorationKind::Other("portrait") => {
                let mut cache = dependencies.texture_cache.borrow_mut();
                let texture = cache.get_facethumb(&self.icon)?;
                Some((texture, 1.))
            }
            _ => None,
        }
    }
}

impl KeyedViewItem for RingData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.rnid)
    }

    fn set_key(&mut self, key: String) {
        self.rnid = key;
    }
}

sheet_retriever!(RingPolishVoiceData, RingCleaningVoiceBook, ring_data, IndexMap<String, Vec<RingPolishVoiceData>>);

impl GroupViewItem for IndexMap<String, Vec<RingPolishVoiceData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for RingPolishVoiceData {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!("Label {}", self.label))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    RingData,
    RingPolishVoiceData,
}

pub struct RingEditor {
    tab: Tab,
    ring_data: RingDataSheet,
    ring_polish_voice: RingPolishVoiceDataSheet,
    ring_data_content: ListEditorContent<IndexMap<String, RingData>, RingData, EditorState>,
    ring_polish_voice_content: GroupEditorContent,
}

impl RingEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::RingData,
            ring_data: state.ring_data.clone(),
            ring_polish_voice: state.ring_polish_voice.clone(),
            ring_data_content: ListEditorContent::new("ring_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            ring_polish_voice_content: GroupEditorContent::new("ring_data_editor"),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::RingData, "Bond Rings");
            ui.selectable_value(&mut self.tab, Tab::RingPolishVoiceData, "Polish Voice");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::RingData => {
                self.ring_data_content
                    .left_panel(ctx, &self.ring_data, state);
                self.ring_data.write(|data| {
                    self.ring_data_content.content(ctx, data, |ui, selection| {
                        let mut changed = false;
                        egui_grid::GridBuilder::new()
                            .new_row(Size::exact(100.))
                            .cell(Size::relative(0.2).at_least(215.))
                            .cell(Size::remainder())
                            .new_row(Size::remainder())
                            .cell(Size::remainder())
                            .show(ui, |mut grid| {
                                grid.cell(|ui| {
                                    ui.group(|ui| {
                                        ui.add(optional_image(
                                            selection.decoration(
                                                state,
                                                DecorationKind::Other("portrait"),
                                            ),
                                            [200., 90.],
                                        ));
                                    });
                                });
                                grid.cell(|ui| {
                                    ui.group(|ui| {
                                        Grid::new("ring_grid").show(ui, |ui| {
                                            stat_column_headers_no_sight(ui);
                                            ui.label("Bonus Stats");
                                            stats_row_no_sight!(ui, selection, enhance, changed);
                                        });
                                        ui.allocate_space(ui.available_size());
                                    });
                                });
                                grid.cell(|ui| {
                                    changed |= PropertyGrid::new("ring_data", selection)
                                        .new_section("")
                                        .field("RNID", |ui, d| ui.add(id_field(&mut d.rnid)))
                                        .field("Name", |ui, d| {
                                            msbt_key_value_singleline!(
                                                ui,
                                                state,
                                                "bondsring",
                                                d.name
                                            )
                                        })
                                        .field("Help", |ui, d| {
                                            msbt_key_value_multiline!(
                                                ui,
                                                state,
                                                "bondsring",
                                                d.help
                                            )
                                        })
                                        .field("GID", |ui, d| {
                                            state.god.read(|data| {
                                                ui.add(model_drop_down(data, state, &mut d.gid))
                                            })
                                        })
                                        .default_field("Ring Model", |d| &mut d.ring_model)
                                        .field("Rank", |ui, d| {
                                            ui.add(ring_rank_drop_down(&mut d.rank))
                                        })
                                        .default_field("Icon", |d| &mut d.icon)
                                        .field("Equip Skills", |ui, d| {
                                            state.skill.read(|data| {
                                                ui.add(editable_list(
                                                    &mut d.equip_sids,
                                                    |_, d, ui| {
                                                        ui.add(model_drop_down(data, state, d))
                                                    },
                                                ))
                                            })
                                        })
                                        .default_field("Is Single Rank", |d| &mut d.is_single_rank)
                                        .field("Jewel Color (RGB)", |ui, d| {
                                            ui.add(rgb_color_picker(
                                                &mut d.jewel_color_r,
                                                &mut d.jewel_color_g,
                                                &mut d.jewel_color_b,
                                            ))
                                        })
                                        .field("Rim Color (RGB)", |ui, d| {
                                            ui.add(rgb_color_picker(
                                                &mut d.rim_color_r,
                                                &mut d.rim_color_g,
                                                &mut d.rim_color_b,
                                            ))
                                        })
                                        .show(ui)
                                        .changed();
                                });
                            });

                        changed
                    })
                });
            }

            Tab::RingPolishVoiceData => {
                self.ring_polish_voice_content
                    .left_panel(ctx, &self.ring_polish_voice, state);
                self.ring_polish_voice.write(|data| {
                    self.ring_polish_voice_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("ring_polish_voice", selection)
                                .new_section("")
                                .default_field("Label", |d| &mut d.label)
                                .default_field("Play Situation", |d| &mut d.play_situation)
                                .default_field("Is Play Completed", |d| &mut d.is_play_completed)
                                .default_field("Unit Face Anim", |d| &mut d.unit_face_anim)
                                .default_field("God Face Anim", |d| &mut d.god_face_anim)
                                .show(ui)
                                .changed()
                        })
                });
            }
        }
    }
}
