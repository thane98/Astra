use std::borrow::Cow;

use astra_formats::{TerrainData, UString};
use egui::{Button, Color32, Grid, ScrollArea, Ui, Vec2};

use crate::model::ViewItem;
use crate::util::get_tile_color;
use crate::{AppConfig, EditorState, ListModel};

pub struct TerrainGridResult {
    pub changed: bool,
    pub hovered_tile: Option<String>,
}

pub fn terrain_grid(
    ui: &mut Ui,
    terrain: &mut TerrainData,
    selected_tile_index: Option<usize>,
    state: &EditorState,
    config: &AppConfig,
) -> TerrainGridResult {
    let mut changed = None;
    let mut hovered_tile = None;
    ScrollArea::both()
        .id_source("chapter_terrain_scroll")
        .show(ui, |ui| {
            ui.spacing_mut().button_padding = Vec2::ZERO;
            ui.spacing_mut().item_spacing = Vec2::new(1., 1.);
            Grid::new("chapter_terrain_grid").show(ui, |ui| {
                state.terrain.read(|data| {
                    for row in (0..(terrain.height as usize)).rev() {
                        for col in 0..(terrain.width as usize) {
                            let (tile_name, fill) = terrain
                                .terrains
                                .get(row * 32 + col)
                                .and_then(|tid| data.get(tid.as_str()))
                                .map(|tile| {
                                    (
                                        tile.text(state),
                                        get_tile_color(tile, config),
                                    )
                                })
                                .unwrap_or_else(|| (Cow::Borrowed("???"), Color32::from_gray(0)));

                            let response =
                                ui.add_sized([48., 48.], Button::new("").rounding(0.).fill(fill));
                            if response.clicked() {
                                let new_tid = selected_tile_index
                                    .and_then(|index| data.item(index))
                                    .map(|tile| tile.tid.clone());
                                if let Some(tid) = new_tid {
                                    changed = Some((row, col, tid));
                                }
                            }
                            if response.hovered() {
                                hovered_tile = Some(tile_name.into_owned());
                            }
                        }
                        ui.end_row();
                    }
                });
            });
        });
    if let Some((row, col, tid)) = changed {
        let index = row * 32 + col;
        if index < terrain.terrains.len() {
            terrain.terrains[index] = UString(tid);
        }
        return TerrainGridResult {
            changed: true,
            hovered_tile,
        };
    }
    TerrainGridResult {
        changed: false,
        hovered_tile,
    }
}
