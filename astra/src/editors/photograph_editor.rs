use std::borrow::Cow;

use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editable_list, editor_tab_strip, id_field, model_drop_down, sheet_retriever, EditorState, GroupEditorContent, GroupViewItem, KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem
};

use astra_types::{PhotographPose, PhotographSpot, PhotographSpotBook};

sheet_retriever!(PhotographSpot, PhotographSpotBook, spots, IndexMap<String, PhotographSpot>);

impl ViewItem for PhotographSpot {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.mid)
    }
}

impl KeyedViewItem for PhotographSpot {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.mid)
    }

    fn set_key(&mut self, key: String) {
        self.mid = key;
    }
}

sheet_retriever!(PhotographPose, PhotographSpotBook, poses, IndexMap<String, Vec<PhotographPose>>);

impl GroupViewItem for IndexMap<String, Vec<PhotographPose>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for PhotographPose {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.pause_name)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    PhotographSpot,
    PhotographPose,
}

pub struct PhotographSpotEditor {
    tab: Tab,
    spots: PhotographSpotSheet,
    poses: PhotographPoseSheet,
    spots_content: ListEditorContent<IndexMap<String, PhotographSpot>, PhotographSpot>,
    poses_content: GroupEditorContent,
}

impl PhotographSpotEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::PhotographSpot,
            spots: state.photograph_spots.clone(),
            poses: state.photograph_poses.clone(),
            spots_content: ListEditorContent::new("spots_editor"),
            poses_content: GroupEditorContent::new("poses_editor"),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::PhotographSpot, "Spot");
            ui.selectable_value(&mut self.tab, Tab::PhotographPose, "Pose");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::PhotographSpot => {
                self.spots_content.left_panel(ctx, &self.spots, state);
                self.spots.write(|data| {
                    self.spots_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("spots", selection)
                            .new_section("")
                            .field("MID", |ui, d| ui.add(id_field(&mut d.mid)))
                            .default_field("Name", |d| &mut d.name)
                            .field("Condition Chapter", |ui, d| state.chapter.read(|data| {
                                ui.add(model_drop_down(data, state, &mut d.condition_cid))
                            }))
                            .default_field("Locator Count", |d| &mut d.locator_count)
                            .default_field("Pause Group Name List 1", |d| {
                                &mut d.pause_group_name_list_1
                            })
                            .default_field("Pause Group Name List 2", |d| {
                                &mut d.pause_group_name_list_2
                            })
                            .default_field("Pause Group Name List 3", |d| {
                                &mut d.pause_group_name_list_3
                            })
                            .default_field("Pause Group Name List 4", |d| {
                                &mut d.pause_group_name_list_4
                            })
                            .show(ui)
                            .changed()
                    })
                });
            }

            Tab::PhotographPose => {
                self.poses_content.left_panel(ctx, &self.poses, state);
                self.poses.write(|data| {
                    self.poses_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("poses", selection)
                            .new_section("")
                            .default_field("Pause Name", |d| &mut d.pause_name)
                            .default_field("Mid", |d| &mut d.mid)
                            .default_field("No", |d| &mut d.no)
                            .default_field("Anime Frame", |d| &mut d.anime_frame)
                            .default_field("Face Anime", |d| &mut d.face_anime)
                            .field("Characters", |ui, d| state.person.read(|data| {
                                ui.add(editable_list(&mut d.chara_id_list, |_, d, ui| {
                                    ui.add(model_drop_down(data, state, d))
                                }))
                            }))
                            .show(ui)
                            .changed()
                    })
                });
            }
        }
    }
}
