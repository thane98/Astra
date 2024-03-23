use std::borrow::Cow;

use astra_types::{Effect, EffectBook, EffectSequence};
use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, id_field, model_drop_down, sheet_retriever, CachedView, EditorState,
    KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem,
};

sheet_retriever!(Effect, EffectBook, effects, IndexMap<String, Effect>);

impl ViewItem for Effect {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.eid)
    }
}

impl KeyedViewItem for Effect {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.eid)
    }

    fn set_key(&mut self, key: String) {
        self.eid = key;
    }
}

sheet_retriever!(EffectSequence, EffectBook, effect_sequences, IndexMap<String, EffectSequence>);

impl ViewItem for EffectSequence {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.sequence)
    }
}

impl KeyedViewItem for EffectSequence {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.sequence)
    }

    fn set_key(&mut self, key: String) {
        self.sequence = key;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Effects,
    Sequences,
}

pub struct EffectEditor {
    tab: Tab,
    effect: EffectSheet,
    sequence: EffectSequenceSheet,
    effect_cache: CachedView<EffectSheetRetriever, EffectBook, Effect>,
    effect_content: ListEditorContent<IndexMap<String, Effect>, Effect>,
    sequence_content: ListEditorContent<IndexMap<String, EffectSequence>, EffectSequence>,
}

impl EffectEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Effects,
            effect: state.effect.clone(),
            sequence: state.effect_sequence.clone(),
            effect_cache: CachedView::new(state.effect.clone(), state),
            effect_content: ListEditorContent::new("effect"),
            sequence_content: ListEditorContent::new("effect_sequence"),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Effects, "Effects");
            ui.selectable_value(&mut self.tab, Tab::Sequences, "Sequences");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.effect_cache.refresh(state);
        match self.tab {
            Tab::Effects => {
                self.effect_content.left_panel(ctx, &self.effect, state);
                self.effect.write(|data| {
                    self.effect_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("effects", selection)
                            .new_section("")
                            .field("EID", |ui, d| ui.add(id_field(&mut d.eid)))
                            .default_field("File Path", |d| &mut d.file_path)
                            .default_field("Sound Label", |d| &mut d.sound_label)
                            .default_field("Ty", |d| &mut d.ty)
                            .default_field("Resident", |d| &mut d.resident)
                            .default_field("Delay Time", |d| &mut d.delay_time)
                            .default_field("Wait Time", |d| &mut d.wait_time)
                            .default_field("Shake Time", |d| &mut d.shake_time)
                            .default_field("Shake Magnitude", |d| &mut d.shake_magnitude)
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::Sequences => {
                self.sequence_content.left_panel(ctx, &self.sequence, state);
                self.sequence.write(|data| {
                    self.sequence_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("effect_sequences", selection)
                            .new_section("")
                            .field("Sequence", |ui, d| ui.add(id_field(&mut d.sequence)))
                            .field("Active", |ui, d| {
                                ui.add(model_drop_down(self.effect_cache.get(), &(), &mut d.active))
                            })
                            .field("Shoot", |ui, d| {
                                ui.add(model_drop_down(self.effect_cache.get(), &(), &mut d.shoot))
                            })
                            .field("Hit", |ui, d| {
                                ui.add(model_drop_down(self.effect_cache.get(), &(), &mut d.hit))
                            })
                            .show(ui)
                            .changed()
                    })
                });
            }
        }
    }
}
