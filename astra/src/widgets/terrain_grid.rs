use astra_formats::{TerrainData, UString};
use egui::{Button, Color32, Grid, ScrollArea, Ui, Vec2};

use crate::{EditorState, ListModel};

pub fn terrain_grid(
    ui: &mut Ui,
    terrain: &mut TerrainData,
    selected_tile_index: Option<usize>,
    state: &EditorState,
) -> bool {
    let mut change = None;
    ScrollArea::both()
        .id_source("chapter_terrain_scroll")
        .show(ui, |ui| {
            ui.spacing_mut().button_padding = Vec2::ZERO;
            ui.spacing_mut().item_spacing = Vec2::new(1., 1.);
            Grid::new("chapter_terrain_grid").show(ui, |ui| {
                state.terrain.read(|data| {
                    for row in (0..(terrain.height as usize)).rev() {
                        for col in 0..(terrain.width as usize) {
                            let fill = terrain
                                .terrains
                                .get(row * 32 + col)
                                .and_then(|tid| data.get(tid.as_str()))
                                .map(|tile| {
                                    Color32::from_rgb(
                                        tile.color_r.unwrap_or_default(),
                                        tile.color_g.unwrap_or_default(),
                                        tile.color_b.unwrap_or_default(),
                                    )
                                })
                                .unwrap_or_else(|| Color32::from_gray(0));

                            let response =
                                ui.add_sized([48., 48.], Button::new("").rounding(0.).fill(fill));
                            if response.clicked() {
                                let new_tid = selected_tile_index
                                    .and_then(|index| data.item(index))
                                    .map(|tile| tile.tid.clone());
                                if let Some(tid) = new_tid {
                                    change = Some((row, col, tid));
                                }
                            }
                        }
                        ui.end_row();
                    }
                });
            });
        });
    if let Some((row, col, tid)) = change {
        let index = row * 32 + col;
        if index < terrain.terrains.len() {
            terrain.terrains[index] = UString(tid);
        }
        return true;
    }
    false
}
