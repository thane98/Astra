use egui::collapsing_header::CollapsingState;
use egui::{Button, CentralPanel, Id, ScrollArea, SidePanel, TextEdit, Ui};
use egui_modal::Modal;
use indexmap::IndexMap;

use crate::{blank_slate, SheetHandle, SheetRetriever, ViewItem};

use super::{group_add_modal_content, group_copy_modal_content, GroupModalCommand};

struct GroupCommand {
    group: String,
    index: usize,
    kind: GroupCommandKind,
}

enum GroupCommandKind {
    AddEntry,
    Remove,
    MoveUp,
    MoveDown,
}

struct GroupEntryCommand {
    group: String,
    index: usize,
    kind: GroupEntryCommandKind,
}

enum GroupEntryCommandKind {
    Insert,
    Remove,
    MoveUp,
    MoveDown,
}

impl GroupCommand {
    pub fn act<I>(self, data: &mut IndexMap<String, Vec<I>>) -> bool
    where
        I: Default,
    {
        match self.kind {
            GroupCommandKind::AddEntry => {
                if let Some(group) = data.get_mut(&self.group) {
                    group.push(I::default());
                    true
                } else {
                    false
                }
            }
            GroupCommandKind::Remove => data.remove(&self.group).is_some(),
            GroupCommandKind::MoveUp => {
                if self.index > 0 && self.index < data.len() {
                    data.swap_indices(self.index, self.index - 1);
                    true
                } else {
                    false
                }
            }
            GroupCommandKind::MoveDown => {
                if !data.is_empty() && self.index < data.len() - 1 {
                    data.swap_indices(self.index, self.index + 1);
                    true
                } else {
                    false
                }
            }
        }
    }
}

impl GroupEntryCommand {
    pub fn act<I>(self, data: &mut IndexMap<String, Vec<I>>) -> bool
    where
        I: Default,
    {
        let group = if let Some(group) = data.get_mut(&self.group) {
            group
        } else {
            return false;
        };
        match self.kind {
            GroupEntryCommandKind::Remove => {
                if self.index < group.len() {
                    group.remove(self.index);
                    true
                } else {
                    false
                }
            }
            GroupEntryCommandKind::MoveUp => {
                if self.index > 0 && self.index < group.len() {
                    group.swap(self.index, self.index - 1);
                    true
                } else {
                    false
                }
            }
            GroupEntryCommandKind::MoveDown => {
                if !group.is_empty() && self.index < group.len() - 1 {
                    group.swap(self.index, self.index + 1);
                    true
                } else {
                    false
                }
            }
            GroupEntryCommandKind::Insert => {
                if self.index < group.len() {
                    group.insert(self.index + 1, I::default());
                    true
                } else {
                    false
                }
            }
        }
    }
}

pub struct GroupEditorContent {
    selection: Option<(String, usize)>,
    search: String,
    modal_command: Option<GroupModalCommand>,
    copy_source: Option<(String, usize)>,
    id_source: &'static str,
}

impl GroupEditorContent {
    pub fn new(id_source: &'static str) -> Self {
        Self {
            id_source,
            selection: None,
            modal_command: None,
            copy_source: None,
            search: String::new(),
        }
    }

    pub fn selection_mut(&mut self) -> &mut Option<(String, usize)> {
        &mut self.selection
    }

    pub fn left_panel<R, B, I>(
        &mut self,
        ctx: &egui::Context,
        model: &SheetHandle<R, B, IndexMap<String, Vec<I>>>,
        dependencies: &I::Dependencies,
    ) where
        R: SheetRetriever<B, IndexMap<String, Vec<I>>>,
        I: ViewItem + Default + Clone,
    {
        let modal = Modal::new(ctx, self.id_source);
        if let Some(modal_command) = self.modal_command {
            modal.show(|ui| {
                model.write(|data| {
                    group_add_modal_content(self.id_source, &modal, ui, data, modal_command)
                });
            });
        }

        let copy_modal = Modal::new(ctx, self.id_source);
        if let Some(copy_source) = &self.copy_source {
            copy_modal.show(|ui| {
                model.write(|data| {
                    group_copy_modal_content(
                        &format!("{}_copy_modal", self.id_source),
                        &copy_modal,
                        ui,
                        data,
                        dependencies,
                        copy_source,
                    )
                });
            });
        }

        SidePanel::left(Id::new(self.id_source).with("left_panel")).show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui
                    .add(Button::new("+").min_size([30., 0.].into()))
                    .clicked()
                {
                    self.modal_command = Some(GroupModalCommand::Add);
                    modal.open();
                }
                ui.add(TextEdit::singleline(&mut self.search).desired_width(f32::INFINITY))
                    .changed();
            });

            let mut group_command = None;
            let mut group_entry_command = None;

            let search = self.search.to_lowercase();
            ScrollArea::both()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    model.read(|data| {
                        for (i, (group, items)) in data.iter().enumerate() {
                            if !group.to_lowercase().contains(&search) {
                                continue;
                            }
                            let id =
                                ui.make_persistent_id(format!("{}_{}", &self.id_source, group));
                            CollapsingState::load_with_default_open(ctx, id, false)
                                .show_header(ui, |ui| {
                                    ui.label(group);
                                    if !group_command.is_some() {
                                        group_command =
                                            self.group_command_menu(&modal, ui, group, i);
                                    }
                                })
                                .body(|ui| {
                                    for (i, entry) in items.iter().enumerate() {
                                        let selected =
                                            self.selection.as_ref().map(|(g, i)| (g.as_str(), *i))
                                                == Some((&group, i));
                                        ui.horizontal_top(|ui| {
                                            // TODO: Show decorations
                                            if ui
                                                .selectable_label(
                                                    selected,
                                                    entry.text(dependencies),
                                                )
                                                .clicked()
                                            {
                                                self.selection = Some((group.clone(), i));
                                            }
                                            if !group_entry_command.is_some() {
                                                group_entry_command = self
                                                    .group_entry_command_menu(
                                                        ui,
                                                        &copy_modal,
                                                        group,
                                                        i,
                                                    );
                                            }
                                        });
                                    }
                                });
                        }
                    });
                });

            model.write(|data| {
                let mut changed = false;
                if let Some(command) = group_command {
                    changed |= command.act(data);
                }
                if let Some(command) = group_entry_command {
                    changed |= command.act(data);
                }
                changed
            });

            self.selection = std::mem::take(&mut self.selection).and_then(|(group, index)| {
                model.read(|data| {
                    if let Some(group_items) = data.get(&group) {
                        if group_items.is_empty() {
                            None
                        } else if index >= group_items.len() {
                            Some((group, index - 1))
                        } else {
                            Some((group, index))
                        }
                    } else {
                        None
                    }
                })
            });
        });
    }

    fn group_command_menu(
        &mut self,
        modal: &Modal,
        ui: &mut Ui,
        group: &str,
        index: usize,
    ) -> Option<GroupCommand> {
        let mut command_kind = None;
        ui.menu_button("…", |ui| {
            if ui.button("✏ Edit Name").clicked() {
                self.modal_command = Some(GroupModalCommand::Edit(index));
                modal.open();
                ui.close_menu();
            }
            ui.separator();
            if ui.button("➕ Add Entry").clicked() {
                command_kind = Some(GroupCommandKind::AddEntry);
                ui.close_menu();
            }
            if ui.button("⮩ Insert Below").clicked() {
                self.modal_command = Some(GroupModalCommand::Insert(index + 1));
                modal.open();
                ui.close_menu();
            }
            if ui.button("🗐 Duplicate").clicked() {
                self.modal_command = Some(GroupModalCommand::Duplicate(index));
                modal.open();
                ui.close_menu();
            }
            ui.separator();
            if ui.button("⏶ Move Up").clicked() {
                command_kind = Some(GroupCommandKind::MoveUp);
                ui.close_menu();
            }
            if ui.button("⏷ Move Down").clicked() {
                command_kind = Some(GroupCommandKind::MoveDown);
                ui.close_menu();
            }
            ui.separator();
            if ui.button("❎ Delete Group").clicked() {
                command_kind = Some(GroupCommandKind::Remove);
                ui.close_menu();
            }
        });
        command_kind.map(|kind| GroupCommand {
            group: group.to_string(),
            index,
            kind,
        })
    }

    fn group_entry_command_menu(
        &mut self,
        ui: &mut Ui,
        copy_modal: &Modal,
        group: &str,
        index: usize,
    ) -> Option<GroupEntryCommand> {
        let mut command_kind = None;
        ui.menu_button("…", |ui| {
            if ui.button("⮩ Insert Below").clicked() {
                command_kind = Some(GroupEntryCommandKind::Insert);
                ui.close_menu();
            }
            ui.separator();
            if ui.button("🗐 Copy To").clicked() {
                self.copy_source = Some((group.to_string(), index));
                copy_modal.open();
                ui.close_menu();
            }
            ui.separator();
            if ui.button("⏶ Move Up").clicked() {
                command_kind = Some(GroupEntryCommandKind::MoveUp);
                ui.close_menu();
            }
            if ui.button("⏷ Move Down").clicked() {
                command_kind = Some(GroupEntryCommandKind::MoveDown);
                ui.close_menu();
            }
            ui.separator();
            if ui.button("❎ Delete Item").clicked() {
                command_kind = Some(GroupEntryCommandKind::Remove);
                ui.close_menu();
            }
        });
        command_kind.map(|kind| GroupEntryCommand {
            group: group.to_string(),
            index,
            kind,
        })
    }

    pub fn content<I>(
        &self,
        ctx: &egui::Context,
        model: &mut IndexMap<String, Vec<I>>,
        add_content: impl FnOnce(&mut Ui, &mut I) -> bool,
    ) -> bool {
        let item = self.selection.as_ref().and_then(|(group, index)| {
            model.get_mut(group).and_then(|group| group.get_mut(*index))
        });
        CentralPanel::default()
            .show(ctx, |ui| match item {
                Some(item) => add_content(ui, item),
                None => {
                    blank_slate(ui);
                    false
                }
            })
            .inner
    }

    pub fn right_panel<I>(
        &self,
        ctx: &egui::Context,
        model: &mut IndexMap<String, Vec<I>>,
        add_content: impl FnOnce(&mut Ui, &mut I) -> bool,
    ) -> bool {
        let item = self.selection.as_ref().and_then(|(group, index)| {
            model.get_mut(group).and_then(|group| group.get_mut(*index))
        });
        if let Some(item) = item {
            SidePanel::right(Id::new(self.id_source).with("right_panel"))
                .show(ctx, |ui| add_content(ui, item))
                .inner
        } else {
            false
        }
    }
}
