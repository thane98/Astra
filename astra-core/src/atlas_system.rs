use std::collections::HashMap;

use anyhow::Result;
use astra_formats::{AtlasBundle, SpriteAtlasWrapper};
use image::{DynamicImage, GenericImageView, RgbaImage};

use crate::LocalizedFileSystem;

pub struct AtlasSystem {
    pub atlases: HashMap<String, SpriteAtlasWrapper>,
}

impl AtlasSystem {
    pub fn load(file_system: &LocalizedFileSystem) -> Result<Self> {
        let entries =
            vec![
            // ("system", "StreamingAssets/aa/Switch/fe_assets_ui/icon/system/system.bundle"),
            ("item", "StreamingAssets/aa/Switch/fe_assets_ui/icon/item/item.bundle"),
            ("skill", "StreamingAssets/aa/Switch/fe_assets_ui/icon/skill/skill.bundle"),
            (
                "facethumb",
                "StreamingAssets/aa/Switch/fe_assets_ui/common/unitlist/facethumb/facethumb.bundle",
            ),
            ("unit_indexes", "StreamingAssets/aa/Switch/fe_assets_ui/icon/unit/unitindexes.bundle"),
            (
                "unit_palettes",
                "StreamingAssets/aa/Switch/fe_assets_ui/icon/unit/unitpallettes.bundle",
            ),
        ];
        let mut atlases = HashMap::new();
        for (key, path) in entries {
            let raw_bundle = file_system.read(path, false)?;
            let atlas =
                AtlasBundle::from_slice(&raw_bundle).and_then(|bundle| bundle.extract_data())?;
            atlases.insert(key.to_owned(), atlas);
        }
        Ok(Self { atlases })
    }

    pub fn take_sprites(&mut self, atlas_id: &str) -> Option<HashMap<String, DynamicImage>> {
        if atlas_id == "units" {
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
            Some(rendered_sprites)
        } else {
            self.atlases
                .remove(atlas_id)
                .map(|atlas| atlas.unwrap_sprites())
        }
    }

    // pub fn get_sprite(&self, atlas_id: &str, sprite_name: &str) -> Option<DynamicImage> {
    //     self.atlases
    //         .get(atlas_id)
    //         .and_then(|atlas| atlas.get_sprite(sprite_name))
    // }

    // pub fn get_unit(&self, unit_name: &str) -> Option<DynamicImage> {
    //     let palette_name = unit_name.split('_').next()?;
    //     let palette = self.get_sprite("unit_palettes", palette_name)?;
    //     let index = self.get_sprite("unit_indexes", unit_name)?;
    //     Some(DynamicImage::ImageRgba8(RgbaImage::from_fn(
    //         index.width(),
    //         index.height(),
    //         |x, y| {
    //             palette
    //                 .get_pixel(index.get_pixel(x, y).0[0] as u32 * 2, 0)
    //                 .to_owned()
    //         },
    //     )))
    // }
}
