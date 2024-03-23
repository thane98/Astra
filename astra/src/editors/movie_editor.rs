use std::borrow::Cow;


use indexmap::IndexMap;

use crate::{
    id_field, msbt_key_value_multiline, msbt_key_value_singleline, sheet_retriever, standard_keyed_display, EditorState, KeyedViewItem, ListEditorContent, PropertyGrid, ViewItem
};

use astra_types::{Movie, MovieBook};

sheet_retriever!(Movie, MovieBook, movies, IndexMap<String, Movie>);

impl ViewItem for Movie {
    type Dependencies = EditorState;

    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str> {
        standard_keyed_display!(self, dependencies, movie_file_name, name)
    }
}

impl KeyedViewItem for Movie {
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.movie_file_name)
    }

    fn set_key(&mut self, key: String) {
        self.movie_file_name = key;
    }
}

pub struct MovieEditor {
    movies: MovieSheet,
    movies_content: ListEditorContent<IndexMap<String, Movie>, Movie>,
}

impl MovieEditor {
    pub fn new(state: &EditorState) -> Self {
        Self {
            movies: state.movies.clone(),
            movies_content: ListEditorContent::new("movies_editor"),
        }
    }

    pub fn show(&mut self, ctx: &egui::Context, state: &EditorState) {
        self.movies_content.left_panel(ctx, &self.movies, state);

        self.movies.write(|data| {
            self.movies_content.content(ctx, data, |ui, selection| {
                PropertyGrid::new("movies", selection)
                    .new_section("")
                    .field("Movie File Name", |ui, d| ui.add(id_field(&mut d.movie_file_name)))
                    .field("Name", |ui, d| msbt_key_value_singleline!(ui, state, "moviename", d.name))
                    .field("Help", |ui, d| msbt_key_value_multiline!(ui, state, "moviename", d.help))
                    .default_field("Condition", |d| &mut d.condition)
                    .default_field("No", |d| &mut d.no)
                    .default_field("Before Sound Event Name 1", |d| {
                        &mut d.before_sound_event_name_1
                    })
                    .default_field("Before Sound Event Name 2", |d| {
                        &mut d.before_sound_event_name_2
                    })
                    .default_field("Before Sound Event Name 3", |d| {
                        &mut d.before_sound_event_name_3
                    })
                    .default_field("After Sound Event Name 1", |d| {
                        &mut d.after_sound_event_name_1
                    })
                    .default_field("After Sound Event Name 2", |d| {
                        &mut d.after_sound_event_name_2
                    })
                    .default_field("After Sound Event Name 3", |d| {
                        &mut d.after_sound_event_name_3
                    })
                    .default_field("Mess File Name", |d| &mut d.mess_file_name)
                    .default_field("Dlc Directory Name", |d| &mut d.dlc_directory_name)
                    .show(ui)
                    .changed()
            })
        });
    }
}
