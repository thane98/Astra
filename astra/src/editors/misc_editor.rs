use std::borrow::Cow;

use astra_types::{
    AmiiboBook, AmiiboData, EndRollBook, EndRollData, GroundAttribute, GroundAttributeBook,
    JukeboxBook, JukeboxData, KeyHelpData, KeyHelpDataBook, MapHistory, MapHistoryBook, RangeBook,
    RangeData, SoundEvent, SoundEventBook, VibrationBook, VibrationDefineData,
};
use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editable_list, editor_tab_strip, id_field, keyed_add_modal_content, model_drop_down,
    msbt_key_value_multiline, msbt_key_value_singleline, sheet_retriever, standard_keyed_display,
    EditorState, GroupEditorContent, GroupViewItem, KeyedViewItem, ListEditorContent, PropertyGrid,
    ViewItem,
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

sheet_retriever!(EndRollData, EndRollBook, end_roll_data, Vec<EndRollData>);

impl ViewItem for EndRollData {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        let mut text = self.text_1.clone();
        text.push_str(&self.text_2);
        text.push_str(&self.text_3);
        if text.is_empty() {
            Cow::Borrowed("{empty}")
        } else {
            Cow::Owned(text)
        }
    }
}

sheet_retriever!(GroundAttribute, GroundAttributeBook, ground_attributes, IndexMap<String, GroundAttribute>);

impl ViewItem for GroundAttribute {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.label)
    }
}

impl KeyedViewItem for GroundAttribute {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.label)
    }

    fn set_key(&mut self, key: String) {
        self.label = key;
    }
}

sheet_retriever!(JukeboxData, JukeboxBook, jukebox_data, IndexMap<String, JukeboxData>);

impl ViewItem for JukeboxData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.event_name)
    }
}

impl KeyedViewItem for JukeboxData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.event_name)
    }

    fn set_key(&mut self, key: String) {
        self.event_name = key;
    }
}

sheet_retriever!(KeyHelpData, KeyHelpDataBook, key_help_data, IndexMap<String, Vec<KeyHelpData>>);

impl GroupViewItem for IndexMap<String, Vec<KeyHelpData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for KeyHelpData {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, mid, mid)
    }
}

sheet_retriever!(SoundEvent, SoundEventBook, sound_events, IndexMap<String, SoundEvent>);

impl ViewItem for SoundEvent {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.movie_file_name)
    }
}

impl KeyedViewItem for SoundEvent {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.movie_file_name)
    }

    fn set_key(&mut self, key: String) {
        self.movie_file_name = key;
    }
}

sheet_retriever!(VibrationDefineData, VibrationBook, vibration_data, IndexMap<String, VibrationDefineData>);

impl ViewItem for VibrationDefineData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.event_name)
    }
}

impl KeyedViewItem for VibrationDefineData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.event_name)
    }

    fn set_key(&mut self, key: String) {
        self.event_name = key;
    }
}

sheet_retriever!(MapHistory, MapHistoryBook, history, IndexMap<String, MapHistory>);

impl ViewItem for MapHistory {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.mhid)
    }
}

impl KeyedViewItem for MapHistory {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.mhid)
    }

    fn set_key(&mut self, key: String) {
        self.mhid = key;
    }
}

sheet_retriever!(RangeData, RangeBook, ranges, IndexMap<String, Vec<RangeData>>);

impl GroupViewItem for IndexMap<String, Vec<RangeData>> {
    type Dependencies = EditorState;

    fn text<'a>(key: &'a str, _: &'a Self::Dependencies) -> Cow<'a, str> {
        key.into()
    }
}

impl ViewItem for RangeData {
    type Dependencies = EditorState;

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!(
            "{} {} {} {} {} {} {} {}",
            self.value_1.unwrap_or_default(),
            self.value_2.unwrap_or_default(),
            self.value_3.unwrap_or_default(),
            self.value_4.unwrap_or_default(),
            self.value_5.unwrap_or_default(),
            self.value_6.unwrap_or_default(),
            self.value_7.unwrap_or_default(),
            self.value_8.unwrap_or_default(),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Amiibo,
    Credits,
    GroundAttribute,
    Jukebox,
    KeyHelp,
    MapHistory,
    Range,
    SoundEvent,
    Vibration,
}

pub struct MiscEditor {
    tab: Tab,
    amiibo: AmiiboSheet,
    end_roll_data: EndRollDataSheet,
    ground_attributes: GroundAttributeSheet,
    jukebox_data: JukeboxDataSheet,
    key_help_data: KeyHelpDataSheet,
    history: MapHistorySheet,
    ranges: RangeDataSheet,
    sound_events: SoundEventSheet,
    vibration_data: VibrationDefineDataSheet,
    amiibo_content: ListEditorContent<IndexMap<String, AmiiboData>, AmiiboData, ()>,
    end_roll_data_content: ListEditorContent<Vec<EndRollData>, EndRollData, EditorState>,
    ground_attributes_content:
        ListEditorContent<IndexMap<String, GroundAttribute>, GroundAttribute, EditorState>,
    jukebox_data_content:
        ListEditorContent<IndexMap<String, JukeboxData>, JukeboxData, EditorState>,
    key_help_data_content: GroupEditorContent,
    history_content: ListEditorContent<IndexMap<String, MapHistory>, MapHistory, EditorState>,
    ranges_content: GroupEditorContent,
    sound_events_content: ListEditorContent<IndexMap<String, SoundEvent>, SoundEvent, EditorState>,
    vibration_data_content:
        ListEditorContent<IndexMap<String, VibrationDefineData>, VibrationDefineData, EditorState>,
}

impl MiscEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Amiibo,
            amiibo: state.amiibo.clone(),
            end_roll_data: state.end_roll_data.clone(),
            ground_attributes: state.ground_attributes.clone(),
            jukebox_data: state.jukebox_data.clone(),
            key_help_data: state.key_help_data.clone(),
            history: state.map_history.clone(),
            ranges: state.ranges.clone(),
            sound_events: state.sound_events.clone(),
            vibration_data: state.vibration_data.clone(),
            amiibo_content: ListEditorContent::new("amiibo_editor")
                .with_add_modal_content(keyed_add_modal_content),
            end_roll_data_content: ListEditorContent::new("end_roll_data_editor"),
            ground_attributes_content: ListEditorContent::new("ground_attributes_editor")
                .with_add_modal_content(keyed_add_modal_content),
            jukebox_data_content: ListEditorContent::new("jukebox_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
            key_help_data_content: GroupEditorContent::new("key_help_data_editor"),
            history_content: ListEditorContent::new("history_editor")
                .with_add_modal_content(keyed_add_modal_content),
            ranges_content: GroupEditorContent::new("ranges_editor"),
            sound_events_content: ListEditorContent::new("sound_events_editor")
                .with_add_modal_content(keyed_add_modal_content),
            vibration_data_content: ListEditorContent::new("vibration_data_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Amiibo, "Amiibo");
            ui.selectable_value(&mut self.tab, Tab::Credits, "Credits");
            ui.selectable_value(&mut self.tab, Tab::GroundAttribute, "Ground Attribute");
            ui.selectable_value(&mut self.tab, Tab::Jukebox, "Jukebox");
            ui.selectable_value(&mut self.tab, Tab::KeyHelp, "Key Help");
            ui.selectable_value(&mut self.tab, Tab::MapHistory, "Map History");
            ui.selectable_value(&mut self.tab, Tab::Range, "Range");
            ui.selectable_value(&mut self.tab, Tab::SoundEvent, "Sound Event");
            ui.selectable_value(&mut self.tab, Tab::Vibration, "Vibration");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        match self.tab {
            Tab::Amiibo => {
                self.amiibo_content.left_panel(ctx, &self.amiibo, &());

                self.amiibo.write(|data| {
                    self.amiibo_content.content(ctx, data, |ui, selection| {
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
            Tab::Credits => {
                self.end_roll_data_content
                    .left_panel(ctx, &self.end_roll_data, state);

                self.end_roll_data.write(|data| {
                    self.end_roll_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("end_roll_data", selection)
                                .new_section("")
                                .default_field("Ty", |d| &mut d.ty)
                                .default_field("Text 1", |d| &mut d.text_1)
                                .default_field("Text 2", |d| &mut d.text_2)
                                .default_field("Text 3", |d| &mut d.text_3)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::GroundAttribute => {
                self.ground_attributes_content
                    .left_panel(ctx, &self.ground_attributes, state);

                self.ground_attributes.write(|data| {
                    self.ground_attributes_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("ground_attributes", selection)
                                .new_section("")
                                .field("Label", |ui, d| ui.add(id_field(&mut d.label)))
                                .default_field("Name", |d| &mut d.name)
                                .default_field("Sound", |d| &mut d.sound)
                                .default_field("Particle", |d| &mut d.particle)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::Jukebox => {
                self.jukebox_data_content
                    .left_panel(ctx, &self.jukebox_data, state);

                self.jukebox_data.write(|data| {
                    self.jukebox_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("jukebox_data", selection)
                                .new_section("")
                                .field("Event Name", |ui, d| ui.add(id_field(&mut d.event_name)))
                                // TOOD: Figure out what MSBT has this
                                .default_field("Name", |d| &mut d.name)
                                .default_field("Condition", |d| &mut d.condition)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::KeyHelp => {
                self.key_help_data_content
                    .left_panel(ctx, &self.key_help_data, state);

                self.key_help_data.write(|data| {
                    self.key_help_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("key_help_data", selection)
                                .new_section("")
                                .default_field("Button Index", |d| &mut d.button_index)
                                .field("MID", |ui, d| {
                                    msbt_key_value_multiline!(ui, state, "system", d.mid)
                                })
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::MapHistory => {
                self.history_content.left_panel(ctx, &self.history, state);

                self.history.write(|data| {
                    self.history_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("history", selection)
                            .new_section("")
                            .field("MHID", |ui, d| ui.add(id_field(&mut d.mhid)))
                            .field("Action", |ui, d| {
                                msbt_key_value_singleline!(ui, state, "maphistory", d.action)
                            })
                            .default_field("Priority", |d| &mut d.priority)
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::Range => {
                self.ranges_content.left_panel(ctx, &self.ranges, state);

                self.ranges.write(|data| {
                    self.ranges_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("ranges", selection)
                            .new_section("")
                            .default_field("Value 1", |d| &mut d.value_1)
                            .default_field("Value 2", |d| &mut d.value_2)
                            .default_field("Value 3", |d| &mut d.value_3)
                            .default_field("Value 4", |d| &mut d.value_4)
                            .default_field("Value 5", |d| &mut d.value_5)
                            .default_field("Value 6", |d| &mut d.value_6)
                            .default_field("Value 7", |d| &mut d.value_7)
                            .default_field("Value 8", |d| &mut d.value_8)
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::SoundEvent => {
                self.sound_events_content
                    .left_panel(ctx, &self.sound_events, state);

                self.sound_events.write(|data| {
                    self.sound_events_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("sound_events", selection)
                                .new_section("")
                                .field("Movie File Name", |ui, d| {
                                    ui.add(id_field(&mut d.movie_file_name))
                                })
                                .default_field("Event Name 1", |d| &mut d.event_name_1)
                                .default_field("Event Name 2", |d| &mut d.event_name_2)
                                .default_field("Event Name 3", |d| &mut d.event_name_3)
                                .default_field("Event Name 4", |d| &mut d.event_name_4)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::Vibration => {
                self.vibration_data_content
                    .left_panel(ctx, &self.vibration_data, state);

                self.vibration_data.write(|data| {
                    self.vibration_data_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("vibration_data", selection)
                                .new_section("")
                                .field("Event Name", |ui, d| ui.add(id_field(&mut d.event_name)))
                                .default_field("Vibration File Name", |d| {
                                    &mut d.vibration_file_name
                                })
                                .default_field("Amplitude Magnitude", |d| {
                                    &mut d.amplitude_magnitude
                                })
                                .show(ui)
                                .changed()
                        })
                });
            }
        }
    }
}
