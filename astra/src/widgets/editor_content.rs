use std::marker::PhantomData;

use egui::panel::Side;
use egui::{Button, CentralPanel, Id, SidePanel, TextEdit, Ui};
use egui_modal::Modal;

use crate::model::{SheetHandle, SheetRetriever};
use crate::{
    blank_slate, list_view, AddModalRenderer, FilterProxyBuilder, ListModel, ViewItem,
    ADD_SHORTCUT, COPY_TO_SHORTCUT, DELETE_SHORTCUT, DUPLICATE_SHORTCUT, INSERT_SHORTCUT,
    MOVE_DOWN_SHORTCUT, MOVE_UP_SHORTCUT,
};

use super::{list_select_modal, AddModalCommand};

pub struct ListEditorContent<M, I, D> {
    phantom: PhantomData<I>,
    id_source: &'static str,
    add_modal_renderer: Option<Box<dyn AddModalRenderer<M, I, D>>>,
    filter_proxy: FilterProxyBuilder,
    selection: Option<usize>,
    prev_model_revision: Option<usize>,
    add_command: Option<AddModalCommand>,
    copy_index: Option<usize>,
}

impl<M, I, D> ListEditorContent<M, I, D>
where
    M: ListModel<I>,
    I: ViewItem<Dependencies = D> + Default + Clone,
{
    pub fn new(id_source: &'static str) -> Self {
        Self {
            id_source,
            selection: None,
            add_modal_renderer: None,
            prev_model_revision: None,
            add_command: None,
            copy_index: None,
            filter_proxy: FilterProxyBuilder::new(),
            phantom: Default::default(),
        }
    }

    pub fn with_add_modal_content(
        self,
        renderer: impl AddModalRenderer<M, I, D> + 'static,
    ) -> Self {
        Self {
            add_modal_renderer: Some(Box::new(renderer)),
            ..self
        }
    }

    pub fn select(&mut self, index: Option<usize>) {
        self.selection = index;
    }

    pub fn selection(&self) -> Option<usize> {
        self.selection
    }

    pub fn left_panel<R, B>(
        &mut self,
        ctx: &egui::Context,
        model: &SheetHandle<R, B, M>,
        dependencies: &D,
    ) where
        R: SheetRetriever<B, M>,
    {
        // TODO: Fix out of bounds selection
        let add_modal = Modal::new(ctx, self.id_source);
        if let (Some(add_modal_renderer), Some(add_command)) =
            (&mut self.add_modal_renderer, self.add_command)
        {
            add_modal.show(|ui| {
                model.write(|data| {
                    add_modal_renderer.show(
                        self.id_source,
                        &add_modal,
                        ui,
                        data,
                        dependencies,
                        add_command,
                    )
                })
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
                        "Copy To",
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
                            self.add_item(model, &add_modal);
                            ui.close_menu();
                        }

                        let has_selection = self.selection.is_some();
                        if ui
                            .add_enabled(has_selection, Button::new("⮩ Insert Below"))
                            .clicked()
                        {
                            self.insert_item(model, &add_modal);
                            ui.close_menu();
                        }
                        if ui
                            .add_enabled(has_selection, Button::new("🗐 Duplicate"))
                            .clicked()
                        {
                            self.duplicate_item(model, &add_modal);
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
                            self.move_item_up(model);
                            ui.close_menu();
                        }
                        if ui
                            .add_enabled(has_selection, Button::new("⏷ Move Down"))
                            .clicked()
                        {
                            self.move_item_down(model);
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui
                            .add_enabled(has_selection, Button::new("❎ Delete Item"))
                            .clicked()
                        {
                            self.delete_item(model);
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
                        &self.filter_proxy.model(changed, data, dependencies),
                        dependencies,
                        &mut proxy_index,
                    ));
                    self.selection = proxy_index
                        .and_then(|proxy_index| self.filter_proxy.source_index(proxy_index, data));
                });

                let no_widgets_focused = ctx.memory(|mem| mem.focus().is_none());
                if no_widgets_focused {
                    if ui.input_mut(|input| input.consume_shortcut(&ADD_SHORTCUT)) {
                        self.add_item(model, &add_modal);
                    }
                    if ui.input_mut(|input| input.consume_shortcut(&INSERT_SHORTCUT)) {
                        self.insert_item(model, &add_modal);
                    }
                    if ui.input_mut(|input| input.consume_shortcut(&DUPLICATE_SHORTCUT)) {
                        self.duplicate_item(model, &add_modal);
                    }
                    if ui.input_mut(|input| input.consume_shortcut(&COPY_TO_SHORTCUT)) {
                        self.copy_index = self.selection;
                        copy_modal.open();
                    }
                    if ui.input_mut(|input| input.consume_shortcut(&MOVE_UP_SHORTCUT)) {
                        self.move_item_up(model);
                    }
                    if ui.input_mut(|input| input.consume_shortcut(&MOVE_DOWN_SHORTCUT)) {
                        self.move_item_down(model);
                    }
                    if ui.input_mut(|input| input.consume_shortcut(&DELETE_SHORTCUT)) {
                        self.delete_item(model);
                    }
                }
            });
    }

    pub fn add_item<R, B>(&mut self, model: &SheetHandle<R, B, M>, add_modal: &Modal)
    where
        R: SheetRetriever<B, M>,
    {
        if self.add_modal_renderer.is_some() {
            self.add_command = Some(AddModalCommand::Add);
            add_modal.open();
        } else {
            model.write(|data| {
                data.add(I::default());
                true
            });
        }
    }

    pub fn insert_item<R, B>(&mut self, model: &SheetHandle<R, B, M>, add_modal: &Modal)
    where
        R: SheetRetriever<B, M>,
    {
        if let Some(selection) = self.selection {
            if self.add_modal_renderer.is_some() {
                self.add_command = Some(AddModalCommand::Insert(selection + 1));
                add_modal.open();
            } else {
                model.write(|data| {
                    data.insert(selection + 1, I::default());
                    true
                });
            }
        }
    }

    pub fn duplicate_item<R, B>(&mut self, model: &SheetHandle<R, B, M>, add_modal: &Modal)
    where
        R: SheetRetriever<B, M>,
    {
        if let Some(selection) = self.selection {
            if self.add_modal_renderer.is_some() {
                self.add_command = Some(AddModalCommand::Duplicate(selection));
                add_modal.open();
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
        }
    }

    pub fn move_item_up<R, B>(&mut self, model: &SheetHandle<R, B, M>)
    where
        R: SheetRetriever<B, M>,
    {
        model.write(|data| {
            if let Some(selection) = self.selection {
                if selection > 0 && selection < data.len() {
                    data.swap_items(selection, selection - 1);
                    self.selection = Some(selection - 1);
                    self.filter_proxy.request_refresh();
                    return true;
                }
            }
            false
        });
    }

    pub fn move_item_down<R, B>(&mut self, model: &SheetHandle<R, B, M>)
    where
        R: SheetRetriever<B, M>,
    {
        model.write(|data| {
            if let Some(selection) = self.selection {
                if selection < data.len() - 1 {
                    data.swap_items(selection, selection + 1);
                    self.selection = Some(selection + 1);
                    self.filter_proxy.request_refresh();
                    return true;
                }
            }
            false
        });
    }

    pub fn delete_item<R, B>(&mut self, model: &SheetHandle<R, B, M>)
    where
        R: SheetRetriever<B, M>,
    {
        model.write(|data| {
            if let Some(selection) = self.selection {
                data.remove(selection);
                if selection >= data.len() {
                    self.selection = None;
                }
                return true;
            }
            false
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
