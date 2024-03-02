use std::marker::PhantomData;

use egui::panel::Side;
use egui::{Button, CentralPanel, Id, SidePanel, TextEdit, Ui};
use egui_modal::Modal;

use crate::model::{SheetHandle, SheetRetriever};
use crate::{blank_slate, list_view, FilterProxyBuilder, ListModel, ViewItem};

use super::{list_select_modal, AddModalCommand};

pub struct ListEditorContent<M, I> {
    phantom: PhantomData<I>,
    id_source: &'static str,
    add_modal_fn: Option<Box<dyn Fn(&str, &Modal, &mut Ui, &mut M, AddModalCommand) -> bool>>,
    filter_proxy: FilterProxyBuilder,
    selection: Option<usize>,
    prev_model_revision: Option<usize>,
    add_command: Option<AddModalCommand>,
    copy_index: Option<usize>,
}

impl<M, I> ListEditorContent<M, I>
where
    M: ListModel<I>,
    I: ViewItem + Default + Clone,
{
    pub fn new(id_source: &'static str) -> Self {
        Self {
            id_source,
            selection: None,
            add_modal_fn: None,
            prev_model_revision: None,
            add_command: None,
            copy_index: None,
            filter_proxy: FilterProxyBuilder::new(),
            phantom: PhantomData,
        }
    }

    pub fn with_add_modal_content(
        self,
        renderer: impl Fn(&str, &Modal, &mut Ui, &mut M, AddModalCommand) -> bool + 'static,
    ) -> Self {
        Self {
            add_modal_fn: Some(Box::new(renderer)),
            ..self
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selection = index;
    }

    pub fn selection(&self) -> Option<usize> {
        self.selection
    }

    pub fn side_panel<R, B>(
        &mut self,
        ctx: &egui::Context,
        model: &SheetHandle<R, B, M>,
        dependencies: &I::Dependencies,
    ) where
        R: SheetRetriever<B, M>,
    {
        // TODO: Fix out of bounds selection
        let modal = Modal::new(ctx, self.id_source);
        if let (Some(modal_fn), Some(add_command)) = (&self.add_modal_fn, self.add_command) {
            modal.show(|ui| {
                model.write(|data| modal_fn(self.id_source, &modal, ui, data, add_command))
            });
        }

        let copy_modal_id = format!("{}_copy_modal", self.id_source);
        let copy_modal = Modal::new(ctx, &copy_modal_id);
        if let Some(source_index) = self.copy_index {
            copy_modal.show(|ui| {
                model.write(|data| {
                    list_select_modal(
                        &copy_modal_id,
                        &copy_modal,
                        ui,
                        data,
                        dependencies,
                        source_index,
                    )
                });
            });
        }

        SidePanel::new(Side::Left, Id::new(self.id_source).with("side_panel"))
            .default_width(300.)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.menu_button("…", |ui| {
                        if ui.button("➕ Add Item").clicked() {
                            if self.add_modal_fn.is_some() {
                                self.add_command = Some(AddModalCommand::Add);
                                modal.open();
                            } else {
                                model.write(|data| {
                                    data.add(I::default());
                                    true
                                });
                            }
                            ui.close_menu();
                        }

                        let has_selection = self.selection.is_some();
                        if ui
                            .add_enabled(has_selection, Button::new("⮩ Insert Below"))
                            .clicked()
                        {
                            let selection = self.selection.unwrap();
                            if self.add_modal_fn.is_some() {
                                self.add_command = Some(AddModalCommand::Insert(selection + 1));
                                modal.open();
                            } else {
                                model.write(|data| {
                                    data.insert(selection + 1, I::default());
                                    true
                                });
                            }
                            ui.close_menu();
                        }
                        if ui
                            .add_enabled(has_selection, Button::new("🗐 Duplicate"))
                            .clicked()
                        {
                            let selection = self.selection.unwrap();
                            if self.add_modal_fn.is_some() {
                                self.add_command = Some(AddModalCommand::Duplicate(selection));
                                modal.open();
                            } else {
                                model.write(|data| {
                                    if let Some(item) = data.item(selection).cloned() {
                                        data.insert(selection + 1, item);
                                        true
                                    } else {
                                        false
                                    }
                                })
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui
                            .add_enabled(has_selection, Button::new("🗐 Copy To"))
                            .clicked()
                        {
                            self.copy_index = self.selection;
                            copy_modal.open();
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui
                            .add_enabled(has_selection, Button::new("⏶ Move Up"))
                            .clicked()
                        {
                            model.write(|data| {
                                let selection = self.selection.unwrap();
                                if selection > 0 && selection < data.len() {
                                    data.swap_items(selection, selection - 1);
                                    self.selection = Some(selection - 1);
                                    self.filter_proxy.request_refresh();
                                    return true;
                                }
                                false
                            });
                            ui.close_menu();
                        }
                        if ui
                            .add_enabled(has_selection, Button::new("⏷ Move Down"))
                            .clicked()
                        {
                            model.write(|data| {
                                let selection = self.selection.unwrap();
                                if selection < data.len() - 1 {
                                    data.swap_items(selection, selection + 1);
                                    self.selection = Some(selection + 1);
                                    self.filter_proxy.request_refresh();
                                    return true;
                                }
                                false
                            });
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui
                            .add_enabled(has_selection, Button::new("❎ Delete Item"))
                            .clicked()
                        {
                            model.write(|data| {
                                data.remove(self.selection.unwrap());
                                if self.selection.unwrap() >= data.len() {
                                    self.selection = None;
                                }
                                true
                            });
                            ui.close_menu();
                        }
                    });
                    self.filter_proxy.with_filter_expr(|filter| {
                        ui.add(TextEdit::singleline(filter).desired_width(f32::INFINITY))
                            .changed()
                    });
                });
                let changed = if let Some(revision) = self.prev_model_revision {
                    if revision < model.revision_number() {
                        self.prev_model_revision = Some(model.revision_number());
                        true
                    } else {
                        false
                    }
                } else {
                    self.prev_model_revision = Some(model.revision_number());
                    true
                };
                model.read(|data| {
                    // TODO: Find a better way to make sure the model is initialized.
                    let _ = self.filter_proxy.model(changed, data, dependencies);

                    let mut proxy_index = self
                        .selection
                        .and_then(|source_index| self.filter_proxy.proxy_index(source_index));
                    ui.add(list_view(
                        20.,
                        &self.filter_proxy.model(changed, data, dependencies),
                        dependencies,
                        &mut proxy_index,
                    ));
                    self.selection = proxy_index
                        .and_then(|proxy_index| self.filter_proxy.source_index(proxy_index, data));
                });
            });
    }

    pub fn content(
        &mut self,
        ctx: &egui::Context,
        model: &mut M,
        add_content: impl FnOnce(&mut Ui, &mut I) -> bool,
    ) -> bool {
        let item = self.selection.and_then(|index| model.item_mut(index));
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
}
