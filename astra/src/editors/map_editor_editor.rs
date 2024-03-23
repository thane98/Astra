use std::borrow::Cow;

use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, id_field, model_drop_down, sheet_retriever, CachedView, EditorState,
    KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem,
};

use astra_types::{MapEditorBook, MapEditorCategory, MapEditorObject};

sheet_retriever!(MapEditorObject, MapEditorBook, objects, IndexMap<String, MapEditorObject>);

impl ViewItem for MapEditorObject {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.object_name)
    }

    fn decorated(kind: crate::DecorationKind<'_>) -> bool {
        MapEditorCategory::decorated(kind)
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies.map_editor_categories.read(|data| {
            data.get(&self.category)
                .and_then(|category| category.decoration(dependencies, kind))
        })
    }
}

impl KeyedViewItem for MapEditorObject {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.object_name)
    }

    fn set_key(&mut self, key: String) {
        self.object_name = key;
    }
}

sheet_retriever!(MapEditorCategory, MapEditorBook, categories, IndexMap<String, MapEditorCategory>);

impl ViewItem for MapEditorCategory {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.cid)
    }

    fn decorated(_: crate::DecorationKind<'_>) -> bool {
        true
    }

    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        _: crate::DecorationKind<'_>,
    ) -> Option<(egui::TextureHandle, f32)> {
        dependencies
            .texture_cache
            .borrow_mut()
            .get_versus(&self.icon_name)
            .map(|texture| (texture, 0.5))
    }
}

impl KeyedViewItem for MapEditorCategory {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.cid)
    }

    fn set_key(&mut self, key: String) {
        self.cid = key;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    MapEditorObject,
    MapEditorCategory,
}

pub struct MapEditorEditor {
    tab: Tab,
    objects: MapEditorObjectSheet,
    categories: MapEditorCategorySheet,
    categories_cache: CachedView<MapEditorCategorySheetRetriever, MapEditorBook, MapEditorCategory>,
    objects_content: ListEditorContent<IndexMap<String, MapEditorObject>, MapEditorObject>,
    categories_content: ListEditorContent<IndexMap<String, MapEditorCategory>, MapEditorCategory>,
}

impl MapEditorEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::MapEditorObject,
            objects: state.map_editor_objects.clone(),
            categories: state.map_editor_categories.clone(),
            categories_cache: CachedView::new(state.map_editor_categories.clone(), state),
            objects_content: ListEditorContent::new("objects_editor"),
            categories_content: ListEditorContent::new("categories_editor"),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::MapEditorObject, "Objects");
            ui.selectable_value(&mut self.tab, Tab::MapEditorCategory, "Categories");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.categories_cache.refresh(state);

        match self.tab {
            Tab::MapEditorObject => {
                self.objects_content.left_panel(ctx, &self.objects, state);
                self.objects.write(|data| {
                    self.objects_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("objects", selection)
                            .new_section("")
                            .field("Object Name", |ui, d| ui.add(id_field(&mut d.object_name)))
                            .default_field("Sound Event", |d| &mut d.sound_event)
                            .field("Category", |ui, d| {
                                ui.add(model_drop_down(
                                    self.categories_cache.get(),
                                    &(),
                                    &mut d.category,
                                ))
                            })
                            .show(ui)
                            .changed()
                    })
                });
            }

            Tab::MapEditorCategory => {
                self.categories_content
                    .left_panel(ctx, &self.categories, state);
                self.categories.write(|data| {
                    self.categories_content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new("categories", selection)
                            .new_section("")
                            .field("Category ID", |ui, d| ui.add(id_field(&mut d.cid)))
                            .default_field("Count Max", |d| &mut d.count_max)
                            .default_field("Icon Name", |d| &mut d.icon_name)
                            .show(ui)
                            .changed()
                    })
                });
            }
        }
    }
}
