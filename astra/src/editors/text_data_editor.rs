use std::collections::BTreeSet;
use std::sync::Arc;

use astra_core::{Astra, OpenMessageScript};
use bimap::BiHashMap;
use egui::{CentralPanel, ScrollArea, SidePanel, TextEdit};
use parking_lot::RwLock;

use crate::{blank_slate, AppConfig, EditorState, MessageDbWrapper, MsbtScriptEditor};

struct Selection {
    raw_selection: String,
    script: OpenMessageScript,
    translation: Option<String>,
}

pub struct TextDataEditor {
    astra: Arc<RwLock<Astra>>,
    scripts: BTreeSet<String>,
    selection: Option<Selection>,
    translations: BiHashMap<String, String>,
    changed: bool,
    search: String,
    person_revision_number: usize,
    god_revision_number: usize,
}

impl TextDataEditor {
    pub fn new(state: &EditorState) -> Self {
        let scripts = state.astra.read().list_msbt_scripts();
        Self {
            astra: state.astra.clone(),
            scripts,
            selection: None,
            search: Default::default(),
            translations: Default::default(),
            changed: false,
            person_revision_number: state.person.revision_number(),
            god_revision_number: state.god.revision_number(),
        }
    }

    pub fn on_leave(&mut self, state: &EditorState) {
        if self.changed {
            if let Some(selection) = &self.selection {
                if let Some(translation) = &selection.translation {
                    untranslate(
                        translation,
                        &self.translations,
                        &selection.script,
                        &state.message_db,
                    );
                }
            }
            self.changed = false;
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &mut EditorState, config: &AppConfig) {
        self.left_panel(ctx, state);

        if self.translations.is_empty()
            || state.person.revision_number() > self.person_revision_number
            || state.god.revision_number() > state.person.revision_number()
        {
            self.translations =
                state
                    .message_db
                    .build_translations(&state.person, &state.god, config);
            self.person_revision_number = state.person.revision_number();
            self.god_revision_number = state.god.revision_number();
        }

        CentralPanel::default().show(ctx, |ui| match self.selection.as_mut() {
            Some(selection) => {
                let mut script_editor = MsbtScriptEditor::new("text_data_script_editor");
                if let Some(translation) = &mut selection.translation {
                    let changed = script_editor
                        .on_focus_lost(|script| {
                            untranslate(
                                script,
                                &self.translations,
                                &selection.script,
                                &state.message_db,
                            );
                            self.changed = false;
                        })
                        .show(ui, translation);
                    self.changed = changed;
                } else {
                    selection
                        .script
                        .access(|script| script_editor.show(ui, script));
                }
            }
            None => blank_slate(ui),
        });
    }

    fn left_panel(&mut self, ctx: &egui::Context, state: &EditorState) {
        SidePanel::left("text_data_editor_side_panel").show(ctx, |ui| {
            ui.add(TextEdit::singleline(&mut self.search).desired_width(f32::INFINITY));
            ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let search = self.search.to_lowercase();
                    for script_name in &self.scripts {
                        if search.is_empty() || script_name.contains(&search) {
                            let selected = self
                                .selection
                                .as_ref()
                                .map(|s| s.raw_selection == *script_name)
                                .unwrap_or_default();
                            if ui.selectable_label(selected, script_name).clicked() {
                                let mut astra = self.astra.write();
                                // TODO: Show an error message.
                                let script = astra.open_msbt_script(script_name).ok();
                                if let Some(script) = script {
                                    self.selection = Some(Selection {
                                        translation: self.translate(&script, state),
                                        raw_selection: script_name.to_owned(),
                                        script,
                                    });
                                }
                            }
                        }
                    }
                });
        });
    }

    fn translate(&self, script: &OpenMessageScript, state: &EditorState) -> Option<String> {
        let mut translation = None;
        script.access(|script| {
            translation = state
                .message_db
                .translate_script(script, &self.translations);
            false
        });
        translation
    }
}

fn untranslate(
    translated_script: &str,
    translations: &BiHashMap<String, String>,
    script: &OpenMessageScript,
    message_db: &MessageDbWrapper,
) {
    script.access(|script| {
        if let Some(untranslated_script) =
            message_db.untranslate_script(translated_script, translations)
        {
            *script = untranslated_script;
            return true;
        }
        false
    });
}
