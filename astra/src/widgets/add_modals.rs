use egui::{Button, ComboBox, Id, Ui};
use egui_modal::Modal;
use indexmap::IndexMap;

use crate::model::{KeyedListModel, KeyedViewItem, ListModel, ViewItem};

use super::indexed_model_drop_down;

#[derive(Debug, Clone, Copy)]
pub enum AddModalCommand {
    Add,
    Insert(usize),
    Duplicate(usize),
}

impl AddModalCommand {
    fn act<M, I>(&self, model: &mut M, key: String) -> bool
    where
        M: KeyedListModel<I>,
        I: KeyedViewItem + Default + Clone,
    {
        match self {
            Self::Add => {
                let mut item = I::default();
                item.set_key(key);
                model.add(item);
                true
            }
            Self::Insert(index) => {
                if *index <= model.len() {
                    let mut item = I::default();
                    item.set_key(key);
                    model.insert(*index, item);
                    true
                } else {
                    false
                }
            }
            Self::Duplicate(index) => {
                if let Some(mut item) = model.item(*index).cloned() {
                    item.set_key(key);
                    model.insert(*index + 1, item);
                    true
                } else {
                    false
                }
            }
        }
    }
}

pub fn keyed_add_modal_content<M, I>(
    id_source: &str,
    modal: &Modal,
    ui: &mut Ui,
    model: &mut M,
    command: AddModalCommand,
) -> bool
where
    M: KeyedListModel<I>,
    I: KeyedViewItem + Default + Clone,
{
    let mut changed = false;
    let id = Id::new(id_source).with("add_modal");
    let mut item_id = ui.memory_mut(|mem| {
        mem.data
            .get_persisted_mut_or_default::<String>(id)
            .to_owned()
    });
    let valid = !item_id.is_empty() && !model.contains(&item_id);
    ui.horizontal_top(|ui| {
        ui.label("ID");
        ui.vertical(|ui| {
            ui.text_edit_singleline(&mut item_id);
            if !valid && !item_id.is_empty() {
                ui.colored_label(ui.visuals().error_fg_color, "ID must be unique.");
            }
        });
    });
    modal.buttons(ui, |ui| {
        modal.button(ui, "Close");
        if ui.add_enabled(valid, Button::new("Add")).clicked() {
            changed = command.act(model, item_id.clone());
            modal.close();
            item_id = String::new();
        }
    });
    ui.memory_mut(|mem| mem.data.insert_persisted(id, item_id));
    changed
}

pub fn list_select_modal<M, I, D>(
    id_source: &str,
    modal: &Modal,
    ui: &mut Ui,
    model: &mut M,
    dependencies: &D,
    source_index: usize,
) -> bool
where
    M: ListModel<I>,
    I: ViewItem<Dependencies = D>,
{
    let mut changed = false;
    let id = Id::new(id_source).with("list_select_modal");
    let mut selected =
        ui.memory_mut(|mem| *mem.data.get_persisted_mut_or_default::<Option<usize>>(id));
    modal.title(ui, "Select an entry");
    ui.add(indexed_model_drop_down(model, dependencies, &mut selected));
    modal.buttons(ui, |ui| {
        modal.button(ui, "Close");
        if ui
            .add_enabled(selected.is_some(), Button::new("Ok"))
            .clicked()
        {
            model.copy(source_index, selected.unwrap());
            modal.close();
            selected = None;
            changed = true;
        }
    });
    ui.memory_mut(|mem| mem.data.insert_persisted(id, selected));
    changed
}

#[derive(Debug, Clone, Copy)]
pub enum GroupModalCommand {
    Add,
    Insert(usize),
    Duplicate(usize),
    Edit(usize),
}

impl GroupModalCommand {
    fn act<I>(&self, model: &mut IndexMap<String, I>, key: String) -> bool
    where
        I: Default + Clone,
    {
        match self {
            Self::Add => {
                model.insert(key, I::default());
                true
            }
            Self::Insert(index) => {
                if *index <= model.len() {
                    model.insert(key, I::default());
                    model.move_index(model.len() - 1, *index);
                    true
                } else {
                    false
                }
            }
            Self::Duplicate(index) => {
                if let Some(data) = model.get_index(*index).map(|(_, v)| v).cloned() {
                    model.insert(key, data);
                    model.move_index(model.len() - 1, *index + 1);
                    true
                } else {
                    false
                }
            }
            Self::Edit(index) => {
                if *index < model.len() {
                    let (_, group) = model.shift_remove_index(*index).unwrap();
                    model.insert(key, group);
                    model.move_index(model.len() - 1, *index);
                    true
                } else {
                    false
                }
            }
        }
    }
}

pub fn group_add_modal_content<I>(
    id_source: &str,
    modal: &Modal,
    ui: &mut Ui,
    model: &mut IndexMap<String, I>,
    command: GroupModalCommand,
) -> bool
where
    I: Default + Clone,
{
    let mut changed = false;
    let id = Id::new(id_source).with("group_add_modal");
    let mut item_id = ui.memory_mut(|mem| {
        mem.data
            .get_persisted_mut_or_default::<String>(id)
            .to_owned()
    });
    let valid = !item_id.is_empty() && !model.contains_key(&item_id);
    ui.horizontal_top(|ui| {
        ui.label("ID");
        ui.vertical(|ui| {
            ui.text_edit_singleline(&mut item_id);
            if !valid && !item_id.is_empty() {
                ui.colored_label(ui.visuals().error_fg_color, "ID must be unique.");
            }
        });
    });
    modal.buttons(ui, |ui| {
        modal.button(ui, "Close");
        if ui.add_enabled(valid, Button::new("Add")).clicked() {
            changed = command.act(model, item_id.clone());
            modal.close();
            item_id = String::new();
        }
    });
    ui.memory_mut(|mem| mem.data.insert_persisted(id, item_id));
    changed
}

pub fn group_copy_modal_content<I, D>(
    id_source: &str,
    modal: &Modal,
    ui: &mut Ui,
    model: &mut IndexMap<String, Vec<I>>,
    dependencies: &D,
    copy_source: &(String, usize),
) -> bool
where
    I: ViewItem<Dependencies = D> + Clone,
{
    let mut changed = false;
    let id = Id::new(id_source).with("group_copy_modal");
    let mut selection = ui.memory_mut(|mem| {
        mem.data
            .get_persisted_mut_or_default::<Option<(String, usize)>>(id)
            .clone()
    });
    modal.title(ui, "Select an entry");
    ComboBox::from_id_source(id.with("group_copy_modal_combo_box"))
        .width(400.)
        .selected_text(
            selection
                .as_ref()
                .map(|(group, index)| format!("{}, Index {}", group, index))
                .unwrap_or_default(),
        )
        .show_ui(ui, |ui| {
            for (group, items) in model.iter_mut() {
                ui.heading(group);
                for (i, item) in items.iter_mut().enumerate() {
                    let selected =
                        selection.as_ref().map(|(g, i)| (g.as_str(), *i)) == Some((&group, i));
                    if ui
                        .selectable_label(selected, item.text(dependencies))
                        .clicked()
                    {
                        selection = Some((group.clone(), i));
                    }
                }
            }
        });
    modal.buttons(ui, |ui| {
        modal.button(ui, "Close");
        if ui
            .add_enabled(selection.is_some(), Button::new("Ok"))
            .clicked()
        {
            let (group, index) = selection.clone().unwrap();
            if let Some(source) = model
                .get(&copy_source.0)
                .and_then(|group| group.get(copy_source.1))
                .cloned()
            {
                if let Some(dest) = model.get_mut(&group) {
                    if index < dest.len() {
                        dest[index] = source;
                        changed = true;
                    }
                }
            }
            modal.close();
            selection = None;
        }
    });
    ui.memory_mut(|mem| mem.data.insert_persisted(id, selection));
    changed
}
