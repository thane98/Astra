use std::borrow::Cow;

use egui::Ui;
use indexmap::IndexMap;

use crate::{
    editor_tab_strip, id_field, keyed_add_modal_content, model_drop_down,
    msbt_key_value_singleline, sheet_retriever, standard_keyed_display, CachedView, EditorState,
    KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem,
};

use astra_types::{
    ProfileCardBook, ProfileCardCategorizedComponent, ProfileCardCategorizedImageComponent,
    ProfileCardColorComponent, ProfileCardDefaultCommentData, ProfileCardFavoriteMapData,
    ProfileCardImageComponent, ProfileCardNameComponent,
};

sheet_retriever!(ProfileCardImageComponent, ProfileCardBook, bg, IndexMap<String, ProfileCardImageComponent>);

impl ViewItem for ProfileCardImageComponent {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!("{} ({})", self.id, self.image))
    }
}

impl KeyedViewItem for ProfileCardImageComponent {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(ProfileCardColorComponent, ProfileCardBook, text_colors, IndexMap<String, ProfileCardColorComponent>);

impl ViewItem for ProfileCardColorComponent {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for ProfileCardColorComponent {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(ProfileCardCategorizedImageComponent, ProfileCardBook, stamp_data_2, IndexMap<String, ProfileCardCategorizedImageComponent>);

impl ViewItem for ProfileCardCategorizedImageComponent {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Owned(format!("{} ({})", self.id, self.image))
    }
}

impl KeyedViewItem for ProfileCardCategorizedImageComponent {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(ProfileCardNameComponent, ProfileCardBook, title, IndexMap<String, ProfileCardNameComponent>);

impl ViewItem for ProfileCardNameComponent {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, id, name)
    }
}

impl KeyedViewItem for ProfileCardNameComponent {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(ProfileCardFavoriteMapData, ProfileCardBook, favorite_map, IndexMap<String, ProfileCardFavoriteMapData>);

impl ViewItem for ProfileCardFavoriteMapData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for ProfileCardFavoriteMapData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(ProfileCardCategorizedComponent, ProfileCardBook, comment, IndexMap<String, ProfileCardCategorizedComponent>);

impl ViewItem for ProfileCardCategorizedComponent {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }
}

impl KeyedViewItem for ProfileCardCategorizedComponent {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.id)
    }

    fn set_key(&mut self, key: String) {
        self.id = key;
    }
}

sheet_retriever!(ProfileCardDefaultCommentData, ProfileCardBook, default_comment, IndexMap<String, ProfileCardDefaultCommentData>);

impl ViewItem for ProfileCardDefaultCommentData {
    type Dependencies = EditorState;

    fn text(&self, _dependencies: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.language)
    }
}

impl KeyedViewItem for ProfileCardDefaultCommentData {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.language)
    }

    fn set_key(&mut self, key: String) {
        self.language = key;
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tab {
    Background,
    Frame,
    Lettering,
    TextColor,
    StampData1,
    StampData2,
    Title,
    FavoriteCharacter,
    FavoriteMap,
    Comment,
    FavoriteMapEditorTheme,
    DefaultComment,
}

pub struct ProfileCardEditor {
    tab: Tab,
    bg: ProfileCardImageComponentSheet,
    frames: ProfileCardImageComponentSheet,
    lettering: ProfileCardImageComponentSheet,
    text_colors: ProfileCardColorComponentSheet,
    stamp_data_1: ProfileCardImageComponentSheet,
    stamp_data_2: ProfileCardCategorizedImageComponentSheet,
    title: ProfileCardNameComponentSheet,
    favorite_character: ProfileCardNameComponentSheet,
    favorite_map: ProfileCardFavoriteMapDataSheet,
    comment: ProfileCardCategorizedComponentSheet,
    favorite_map_editor_theme: ProfileCardCategorizedComponentSheet,
    default_comment: ProfileCardDefaultCommentDataSheet,
    comment_cache: CachedView<
        ProfileCardCategorizedComponentSheetRetriever,
        ProfileCardBook,
        ProfileCardCategorizedComponent,
    >,
    bg_content: ListEditorContent<
        IndexMap<String, ProfileCardImageComponent>,
        ProfileCardImageComponent,
        EditorState,
    >,
    frames_content: ListEditorContent<
        IndexMap<String, ProfileCardImageComponent>,
        ProfileCardImageComponent,
        EditorState,
    >,
    lettering_content: ListEditorContent<
        IndexMap<String, ProfileCardImageComponent>,
        ProfileCardImageComponent,
        EditorState,
    >,
    text_colors_content: ListEditorContent<
        IndexMap<String, ProfileCardColorComponent>,
        ProfileCardColorComponent,
        EditorState,
    >,
    stamp_data_1_content: ListEditorContent<
        IndexMap<String, ProfileCardImageComponent>,
        ProfileCardImageComponent,
        EditorState,
    >,
    stamp_data_2_content: ListEditorContent<
        IndexMap<String, ProfileCardCategorizedImageComponent>,
        ProfileCardCategorizedImageComponent,
        EditorState,
    >,
    title_content: ListEditorContent<
        IndexMap<String, ProfileCardNameComponent>,
        ProfileCardNameComponent,
        EditorState,
    >,
    favorite_character_content: ListEditorContent<
        IndexMap<String, ProfileCardNameComponent>,
        ProfileCardNameComponent,
        EditorState,
    >,
    favorite_map_content: ListEditorContent<
        IndexMap<String, ProfileCardFavoriteMapData>,
        ProfileCardFavoriteMapData,
        EditorState,
    >,
    comment_content: ListEditorContent<
        IndexMap<String, ProfileCardCategorizedComponent>,
        ProfileCardCategorizedComponent,
        EditorState,
    >,
    favorite_map_editor_theme_content: ListEditorContent<
        IndexMap<String, ProfileCardCategorizedComponent>,
        ProfileCardCategorizedComponent,
        EditorState,
    >,
    default_comment_content: ListEditorContent<
        IndexMap<String, ProfileCardDefaultCommentData>,
        ProfileCardDefaultCommentData,
        EditorState,
    >,
}

impl ProfileCardEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            tab: Tab::Background,
            bg: state.profile_card_bg.clone(),
            frames: state.profile_card_frames.clone(),
            lettering: state.profile_card_lettering.clone(),
            text_colors: state.profile_card_text_colors.clone(),
            stamp_data_1: state.profile_card_stamp_data_1.clone(),
            stamp_data_2: state.profile_card_stamp_data_2.clone(),
            title: state.profile_card_title.clone(),
            favorite_character: state.profile_card_favorite_character.clone(),
            favorite_map: state.profile_card_favorite_map.clone(),
            comment: state.profile_card_comment.clone(),
            favorite_map_editor_theme: state.profile_card_favorite_map_editor_theme.clone(),
            default_comment: state.profile_card_default_comment.clone(),
            comment_cache: CachedView::new(state.profile_card_comment.clone(), state),
            bg_content: ListEditorContent::new("bg_editor")
                .with_add_modal_content(keyed_add_modal_content),
            frames_content: ListEditorContent::new("frames_editor")
                .with_add_modal_content(keyed_add_modal_content),
            lettering_content: ListEditorContent::new("lettering_editor")
                .with_add_modal_content(keyed_add_modal_content),
            text_colors_content: ListEditorContent::new("text_colors_editor")
                .with_add_modal_content(keyed_add_modal_content),
            stamp_data_1_content: ListEditorContent::new("stamp_data_1_editor")
                .with_add_modal_content(keyed_add_modal_content),
            stamp_data_2_content: ListEditorContent::new("stamp_data_2_editor")
                .with_add_modal_content(keyed_add_modal_content),
            title_content: ListEditorContent::new("title_editor")
                .with_add_modal_content(keyed_add_modal_content),
            favorite_character_content: ListEditorContent::new("favorite_character_editor")
                .with_add_modal_content(keyed_add_modal_content),
            favorite_map_content: ListEditorContent::new("favorite_map_editor")
                .with_add_modal_content(keyed_add_modal_content),
            comment_content: ListEditorContent::new("comment_editor")
                .with_add_modal_content(keyed_add_modal_content),
            favorite_map_editor_theme_content: ListEditorContent::new(
                "favorite_map_editor_theme_editor",
            )
            .with_add_modal_content(keyed_add_modal_content),
            default_comment_content: ListEditorContent::new("default_comment_editor")
                .with_add_modal_content(keyed_add_modal_content),
        }
    }

    pub fn tab_strip(&mut self, ui: &mut Ui) {
        editor_tab_strip(ui, |ui| {
            ui.selectable_value(&mut self.tab, Tab::Background, "Background");
            ui.selectable_value(&mut self.tab, Tab::Frame, "Frame");
            ui.selectable_value(&mut self.tab, Tab::Lettering, "Lettering");
            ui.selectable_value(&mut self.tab, Tab::TextColor, "Text Color");
            ui.selectable_value(&mut self.tab, Tab::StampData1, "Stamp Data (1)");
            ui.selectable_value(&mut self.tab, Tab::StampData2, "Stamp Data (2)");
            ui.selectable_value(&mut self.tab, Tab::Title, "Title");
            ui.selectable_value(&mut self.tab, Tab::FavoriteCharacter, "Favorite Character");
            ui.selectable_value(&mut self.tab, Tab::FavoriteMap, "Favorite Map");
            ui.selectable_value(&mut self.tab, Tab::Comment, "Comment");
            ui.selectable_value(
                &mut self.tab,
                Tab::FavoriteMapEditorTheme,
                "Favorite Map Editor Theme",
            );
            ui.selectable_value(&mut self.tab, Tab::DefaultComment, "Default Comment");
        });
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.comment_cache.refresh(state);

        match self.tab {
            Tab::Background | Tab::Frame | Tab::Lettering | Tab::StampData1 => {
                let (id, sheet, content) = match self.tab {
                    Tab::Background => ("bg", &mut self.bg, &mut self.bg_content),
                    Tab::Frame => ("frame", &mut self.frames, &mut self.frames_content),
                    Tab::Lettering => (
                        "lettering",
                        &mut self.lettering,
                        &mut self.lettering_content,
                    ),
                    Tab::StampData1 => (
                        "stamp_data_1",
                        &mut self.stamp_data_1,
                        &mut self.stamp_data_1_content,
                    ),
                    _ => unimplemented!(),
                };
                content.left_panel(ctx, sheet, state);
                sheet.write(|data| {
                    content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new(id, selection)
                            .new_section("")
                            .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                            .default_field("Image", |d| &mut d.image)
                            .default_field("Condition", |d| &mut d.condition)
                            .default_field("Arg", |d| &mut d.arg)
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::TextColor => {
                self.text_colors_content
                    .left_panel(ctx, &self.text_colors, state);
                self.text_colors.write(|data| {
                    self.text_colors_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("text_colors", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .default_field("Color", |d| &mut d.color)
                                .default_field("Condition", |d| &mut d.condition)
                                .default_field("Arg", |d| &mut d.arg)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::StampData2 => {
                self.stamp_data_2_content
                    .left_panel(ctx, &self.stamp_data_2, state);
                self.stamp_data_2.write(|data| {
                    self.stamp_data_2_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("stamp_data_2", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .default_field("Image", |d| &mut d.image)
                                .default_field("Category", |d| &mut d.category)
                                .default_field("Condition", |d| &mut d.condition)
                                .default_field("Arg", |d| &mut d.arg)
                                .show(ui)
                                .changed()
                        })
                });
            }
            Tab::Title | Tab::FavoriteCharacter => {
                let (id, sheet, content) = match self.tab {
                    Tab::Title => ("title", &mut self.title, &mut self.title_content),
                    Tab::FavoriteCharacter => (
                        "favorite_character",
                        &mut self.favorite_character,
                        &mut self.favorite_character_content,
                    ),
                    _ => unimplemented!(),
                };
                content.left_panel(ctx, sheet, state);
                sheet.write(|data| {
                    content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new(id, selection)
                            .new_section("")
                            .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                            .field("Name", |ui, d| {
                                msbt_key_value_singleline!(ui, state, "person", d.name)
                            })
                            .default_field("Condition", |d| &mut d.condition)
                            .default_field("Arg", |d| &mut d.arg)
                            .show(ui)
                            .changed()
                    })
                });
            }

            Tab::FavoriteMap => {
                self.favorite_map_content
                    .left_panel(ctx, &self.favorite_map, state);
                self.favorite_map.write(|data| {
                    self.favorite_map_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("favorite_map", selection)
                                .new_section("")
                                .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                                .field("Chapter", |ui, d| {
                                    state.chapter.read(|data| {
                                        ui.add(model_drop_down(data, state, &mut d.cid))
                                    })
                                })
                                .default_field("Condition", |d| &mut d.condition)
                                .default_field("Arg", |d| &mut d.arg)
                                .show(ui)
                                .changed()
                        })
                });
            }

            Tab::Comment | Tab::FavoriteMapEditorTheme => {
                let (id, sheet, content) = match self.tab {
                    Tab::Comment => ("comment", &mut self.comment, &mut self.comment_content),
                    Tab::FavoriteMapEditorTheme => (
                        "favorite_map_editor_theme",
                        &mut self.favorite_map_editor_theme,
                        &mut self.favorite_map_editor_theme_content,
                    ),
                    _ => unimplemented!(),
                };
                content.left_panel(ctx, sheet, state);
                sheet.write(|data| {
                    content.content(ctx, data, |ui, selection| {
                        PropertyGrid::new(id, selection)
                            .new_section("")
                            .field("ID", |ui, d| ui.add(id_field(&mut d.id)))
                            .field("Name", |ui, d| {
                                msbt_key_value_singleline!(ui, state, "profilecard", d.name)
                            })
                            .default_field("Category", |d| &mut d.category)
                            .default_field("Condition", |d| &mut d.condition)
                            .default_field("Arg", |d| &mut d.arg)
                            .show(ui)
                            .changed()
                    })
                });
            }
            Tab::DefaultComment => {
                self.default_comment_content
                    .left_panel(ctx, &self.default_comment, state);
                self.default_comment.write(|data| {
                    self.default_comment_content
                        .content(ctx, data, |ui, selection| {
                            PropertyGrid::new("default_comment", selection)
                                .new_section("")
                                .field("Language", |ui, d| ui.add(id_field(&mut d.language)))
                                .field("Comment 1", |ui, d| {
                                    ui.add(model_drop_down(
                                        self.comment_cache.get(),
                                        &(),
                                        &mut d.id_1,
                                    ))
                                })
                                .field("Comment 2", |ui, d| {
                                    ui.add(model_drop_down(
                                        self.comment_cache.get(),
                                        &(),
                                        &mut d.id_2,
                                    ))
                                })
                                .field("Comment 3", |ui, d| {
                                    ui.add(model_drop_down(
                                        self.comment_cache.get(),
                                        &(),
                                        &mut d.id_3,
                                    ))
                                })
                                .show(ui)
                                .changed()
                        })
                });
            }
        }
    }
}
