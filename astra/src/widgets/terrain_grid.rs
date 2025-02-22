use std::borrow::Cow;
use std::collections::HashSet;

use astra_formats::{TerrainData, UString};
use egui::{Button, Color32, Grid, ScrollArea, Ui, Vec2};

use crate::model::ViewItem;
use crate::util::get_tile_color;
use crate::{AppConfig, EditorState, ListModel};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum TerrainBrush {
    #[default]
    Stamp,
    Fill,
}

pub struct TerrainGridResult {
    pub changed: bool,
    pub hovered_tile: Option<String>,
    pub selected_tile: Option<String>,
}

pub fn terrain_grid(
    ui: &mut Ui,
    terrain: &mut TerrainData,
    selected_tile_index: Option<usize>,
    state: &EditorState,
    config: &AppConfig,
    brush: TerrainBrush,
) -> TerrainGridResult {
    let mut changed = vec![];
    let mut hovered_tile = None;
    let mut selected_tile = None;
    ScrollArea::both()
        .id_source("chapter_terrain_scroll")
        .show(ui, |ui| {
            ui.spacing_mut().button_padding = Vec2::ZERO;
            ui.spacing_mut().item_spacing = Vec2::new(1., 1.);
            Grid::new("chapter_terrain_grid").show(ui, |ui| {
                state.terrain.read(|data| {
                    for row in (0..(terrain.height as usize)).rev() {
                        for col in 0..(terrain.width as usize) {
                            let tid = terrain.terrains.get(row * 32 + col);
                            let (tile_name, fill) = tid
                                .and_then(|tid| data.get(tid.as_str()))
                                .map(|tile| (tile.text(state), get_tile_color(tile, config)))
                                .unwrap_or_else(|| (Cow::Borrowed("???"), Color32::from_gray(0)));

                            let response =
                                ui.add_sized([48., 48.], Button::new("").rounding(0.).fill(fill));
                            if response.clicked() {
                                if let Some(tid) = tid {
                                    match brush {
                                        TerrainBrush::Stamp => changed.push((row, col)),
                                        TerrainBrush::Fill => get_bucket_fill_tiles(
                                            &mut changed,
                                            &terrain.terrains,
                                            tid.as_str(),
                                            row,
                                            col,
                                        ),
                                    }
                                }
                            }
                            if response.secondary_clicked() {
                                selected_tile = tid.map(|v| v.to_string());
                            }
                            if response.hovered() {
                                egui::show_tooltip(ui.ctx(), ui.id(), |ui| {
                                    ui.label(tile_name.clone());
                                });
                                hovered_tile = Some(tile_name.into_owned());
                            }
                        }
                        ui.end_row();
                    }
                });
            });
        });
    if !changed.is_empty() {
        let new_tid = state.terrain.read(|data| {
            selected_tile_index
                .and_then(|index| data.item(index))
                .map(|tile| tile.tid.clone())
        });
        if let Some(tid) = new_tid {
            for (row, col) in &changed {
                let index = row * 32 + col;
                if index < terrain.terrains.len() {
                    terrain.terrains[index] = UString(tid.clone());
                }
            }
        }
    }
    TerrainGridResult {
        changed: changed.is_empty(),
        hovered_tile,
        selected_tile,
    }
}

fn get_bucket_fill_tiles(
    output: &mut Vec<(usize, usize)>,
    tiles: &[UString],
    target_tid: &str,
    row: usize,
    col: usize,
) {
    // Normally, a bucket / flood fill algorithm doesn't need this because you make changes as you go.
    // We don't do that because of the borrow checker, so we use a set to track checked coordinates
    // and exit recursion when the coordinate has already been checked.
    let mut checked = HashSet::new();
    get_bucket_fill_tiles_recursive(&mut checked, output, tiles, target_tid, row, col);
}

fn get_bucket_fill_tiles_recursive(
    checked: &mut HashSet<(usize, usize)>,
    output: &mut Vec<(usize, usize)>,
    tiles: &[UString],
    target_tid: &str,
    row: usize,
    col: usize,
) {
    // Base case: already visited
    if checked.contains(&(row, col)) {
        return;
    }

    checked.insert((row, col));

    // Base case: out of bounds
    if let Some(tid) = tiles.get(row * 32 + col) {
        // Base case: different tile
        if tid.as_str() == target_tid {
            // General case: note the tile and recurse to adjacent tiles
            output.push((row, col));
            get_bucket_fill_tiles_recursive(checked, output, tiles, target_tid, row - 1, col);
            get_bucket_fill_tiles_recursive(checked, output, tiles, target_tid, row + 1, col);
            get_bucket_fill_tiles_recursive(checked, output, tiles, target_tid, row, col + 1);
            get_bucket_fill_tiles_recursive(checked, output, tiles, target_tid, row, col - 1);
        }
    }
}
