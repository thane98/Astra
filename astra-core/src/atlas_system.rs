use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use astra_formats::{AtlasBundle, SpriteAtlasWrapper};
use image::io::Reader;
use image::{DynamicImage, GenericImageView, RgbaImage};
use tracing::warn;

use crate::{CobaltFileSystemProxy, LocalizedFileSystem};

pub struct AtlasSystem {
    pub atlases: HashMap<String, SpriteAtlasWrapper>,
    pub cobalt_icons: HashMap<String, HashMap<String, DynamicImage>>,
}

struct AtlasInfo {
    key: &'static str,
    rom_path: &'static str,
    cobalt: Option<CobaltAtlasInfo>,
}

struct CobaltAtlasInfo {
    path: &'static str,
}

impl AtlasSystem {
    pub fn load(
        file_system: &LocalizedFileSystem,
        cobalt_file_system: &CobaltFileSystemProxy,
    ) -> Result<Self> {
        let entries = vec![
            AtlasInfo {
                key: "achievement",
                rom_path: "fe_assets_ui/icon/achievement/achievement.bundle",
                cobalt: None,
            },
            AtlasInfo {
                key: "hub_icons",
                rom_path: "fe_assets_ui/hub/minimap/textures/minimap.bundle",
                cobalt: None,
            },
            AtlasInfo {
                key: "hub_cafe_icons",
                rom_path: "fe_assets_ui/hub/cafeterrace/cafe/textures/cafe.bundle",
                cobalt: None,
            },
            AtlasInfo {
                key: "system",
                rom_path: "fe_assets_ui/icon/system/system.bundle",
                cobalt: None,
            },
            AtlasInfo {
                key: "fishing",
                rom_path: "fe_assets_ui/hub/fishing/textures/fishing.bundle",
                cobalt: None,
            },
            AtlasInfo {
                key: "godring",
                rom_path: "fe_assets_ui/icon/godring/godring.bundle",
                cobalt: Some(CobaltAtlasInfo {
                    path: "icon/emblem/godring",
                }),
            },
            AtlasInfo {
                key: "item",
                rom_path: "fe_assets_ui/icon/item/item.bundle",
                cobalt: Some(CobaltAtlasInfo { path: "icon/item" }),
            },
            AtlasInfo {
                key: "notebook",
                rom_path: "fe_assets_ui/hub/notebook/stamps/allstamps.bundle",
                cobalt: None,
            },
            AtlasInfo {
                key: "skill",
                rom_path: "fe_assets_ui/icon/skill/skill.bundle",
                cobalt: Some(CobaltAtlasInfo { path: "icon/skill" }),
            },
            AtlasInfo {
                key: "facethumb",
                rom_path: "fe_assets_ui/common/unitlist/facethumb/facethumb.bundle",
                cobalt: Some(CobaltAtlasInfo {
                    path: "icon/facethumb",
                }),
            },
            AtlasInfo {
                key: "unit_indexes",
                rom_path: "fe_assets_ui/icon/unit/unitindexes.bundle",
                cobalt: Some(CobaltAtlasInfo { path: "icon/job" }),
            },
            AtlasInfo {
                key: "unit_palettes",
                rom_path: "fe_assets_ui/icon/unit/unitpallettes.bundle",
                cobalt: None,
            },
            AtlasInfo {
                key: "versus",
                rom_path: "fe_assets_ui/network/versus/textures/versus.bundle",
                cobalt: None,
            },
            AtlasInfo {
                key: "relaystamp",
                rom_path: "fe_assets_ui/network/relaystamp/relaystamp.bundle",
                cobalt: None,
            },
            AtlasInfo {
                key: "mapstatus",
                rom_path: "fe_assets_map/textures/mapstatus/mapstatus.bundle",
                cobalt: Some(CobaltAtlasInfo {
                    path: "icon/mapstatus",
                }),
            },
        ];
        let mut atlases = HashMap::new();
        let mut cobalt_icons = HashMap::new();
        for entry in entries {
            let AtlasInfo {
                key,
                rom_path,
                cobalt,
            } = entry;
            let full_path = format!("StreamingAssets/aa/Switch/{}", rom_path);
            let raw_bundle = file_system.read(&full_path, false)?;
            let atlas = AtlasBundle::from_slice(&raw_bundle)
                .and_then(|bundle| bundle.extract_data())
                .with_context(|| format!("Failed to load sprites from {}", full_path))?;
            atlases.insert(key.to_string(), atlas);

            if let Some(cobalt) = cobalt {
                match cobalt_file_system.list_cobalt_icons(cobalt.path) {
                    Ok(paths) => {
                        let images = load_cobalt_icons(paths);
                        cobalt_icons.insert(key.to_string(), images);
                    }
                    Err(err) => warn!(
                        "Failed to read Cobalt icons for atlas '{}' error: {:?}",
                        key, err
                    ),
                }
            }
        }
        Ok(Self {
            atlases,
            cobalt_icons,
        })
    }

    pub fn take_sprites(&mut self, atlas_id: &str) -> Option<HashMap<String, DynamicImage>> {
        if atlas_id == "units" {
            let cobalt_sprites = self.cobalt_icons.remove("unit_indexes").unwrap_or_default();
            let indexes = self.take_sprites("unit_indexes")?;
            let palettes = self.take_sprites("unit_palettes")?;
            let mut rendered_sprites = HashMap::new();
            for (unit_name, index) in indexes {
                if let Some(palette_name) = unit_name.split('_').next() {
                    if let Some(palette) = palettes.get(palette_name) {
                        let image = DynamicImage::ImageRgba8(RgbaImage::from_fn(
                            index.width(),
                            index.height(),
                            |x, y| {
                                palette
                                    .get_pixel(index.get_pixel(x, y).0[0] as u32 * 2, 0)
                                    .to_owned()
                            },
                        ));
                        rendered_sprites.insert(unit_name, image);
                    }
                }
            }
            rendered_sprites.extend(cobalt_sprites);
            Some(rendered_sprites)
        } else {
            self.atlases.remove(atlas_id).map(|atlas| {
                let mut sprites = atlas.unwrap_sprites();
                sprites.extend(self.cobalt_icons.remove(atlas_id).unwrap_or_default());
                sprites
            })
        }
    }
}

fn load_cobalt_icons(paths: HashSet<PathBuf>) -> HashMap<String, DynamicImage> {
    let mut icons = HashMap::new();
    for path in paths {
        let icon_name = path
            .file_stem()
            .map(|file_name| file_name.to_string_lossy().to_string());
        let icon_name = match icon_name {
            Some(name) => name,
            None => {
                warn!("Could not extract file stem from path {}", path.display());
                continue;
            }
        };
        match load_cobalt_icon(path.as_path()) {
            Ok(icon) => {
                icons.insert(icon_name, icon);
            }
            Err(err) => {
                warn!(
                    "Failed to decode Cobalt icon from path {} due to error: {:?}",
                    path.display(),
                    err
                );
            }
        }
    }
    icons
}

fn load_cobalt_icon(path: &Path) -> Result<DynamicImage> {
    let image = Reader::open(path)?.decode()?;
    Ok(image)
}
