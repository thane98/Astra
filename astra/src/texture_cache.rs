use std::collections::HashMap;

use astra_core::Astra;
use astra_formats::image::DynamicImage;
use egui::{ColorImage, TextureHandle, TextureOptions};

pub struct TextureCache {
    system_cache: HashMap<String, TextureHandle>,
    godring_cache: HashMap<String, TextureHandle>,
    facethumb_cache: HashMap<String, TextureHandle>,
    item_icon_cache: HashMap<String, TextureHandle>,
    skill_icon_cache: HashMap<String, TextureHandle>,
    unit_icon_cache: HashMap<String, TextureHandle>,
    versus_cache: HashMap<String, TextureHandle>,
    hub_icons_cache: HashMap<String, TextureHandle>,
    hub_cafe_icons_cache: HashMap<String, TextureHandle>,
    fish_cache: HashMap<String, TextureHandle>,
    achievement_cache: HashMap<String, TextureHandle>,
    notebook_cache: HashMap<String, TextureHandle>,
    relay_stamp_cache: HashMap<String, TextureHandle>,
    map_status_cache: HashMap<String, TextureHandle>,
}

impl TextureCache {
    pub fn new(ctx: egui::Context, astra: &mut Astra) -> Self {
        Self {
            system_cache: Self::build_cache(
                &ctx,
                astra.consume_sprite_atlas("system").unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
            godring_cache: Self::build_cache(
                &ctx,
                astra.consume_sprite_atlas("godring").unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
            facethumb_cache: Self::build_cache(
                &ctx,
                astra.consume_sprite_atlas("facethumb").unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
            item_icon_cache: Self::build_cache(
                &ctx,
                astra.consume_sprite_atlas("item").unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
            skill_icon_cache: Self::build_cache(
                &ctx,
                astra.consume_sprite_atlas("skill").unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
            unit_icon_cache: Self::build_cache(
                &ctx,
                astra.consume_sprite_atlas("units").unwrap_or_default(),
                TextureOptions::NEAREST,
            ),
            versus_cache: Self::build_cache(
                &ctx,
                astra.consume_sprite_atlas("versus").unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
            hub_icons_cache: Self::build_cache(
                &ctx,
                astra.consume_sprite_atlas("hub_icons").unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
            hub_cafe_icons_cache: Self::build_cache(
                &ctx,
                astra
                    .consume_sprite_atlas("hub_cafe_icons")
                    .unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
            fish_cache: Self::build_cache(
                &ctx,
                astra.consume_sprite_atlas("fishing").unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
            achievement_cache: Self::build_cache(
                &ctx,
                astra
                    .consume_sprite_atlas("achievement")
                    .unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
            notebook_cache: Self::build_cache(
                &ctx,
                astra.consume_sprite_atlas("notebook").unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
            relay_stamp_cache: Self::build_cache(
                &ctx,
                astra.consume_sprite_atlas("relaystamp").unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
            map_status_cache: Self::build_cache(
                &ctx,
                astra.consume_sprite_atlas("mapstatus").unwrap_or_default(),
                TextureOptions::LINEAR,
            ),
        }
    }

    fn build_cache(
        ctx: &egui::Context,
        backend_sprites: HashMap<String, DynamicImage>,
        texture_options: TextureOptions,
    ) -> HashMap<String, TextureHandle> {
        backend_sprites
            .into_iter()
            .map(|(key, image)| {
                (
                    key.clone(),
                    ctx.load_texture(
                        key,
                        ColorImage::from_rgba_unmultiplied(
                            [image.width() as _, image.height() as _],
                            image.as_bytes(),
                        ),
                        texture_options,
                    ),
                )
            })
            .collect()
    }

    pub fn get_system(&mut self, key: &str) -> Option<TextureHandle> {
        self.system_cache.get(key).cloned()
    }

    pub fn get_godring(&mut self, key: &str) -> Option<TextureHandle> {
        self.godring_cache.get(key).cloned()
    }

    pub fn get_facethumb(&mut self, key: &str) -> Option<TextureHandle> {
        self.facethumb_cache
            .get(&format!("{}_DLC", key))
            .or_else(|| self.facethumb_cache.get(key))
            .cloned()
    }

    pub fn get_item(&mut self, key: &str) -> Option<TextureHandle> {
        self.item_icon_cache.get(key).cloned()
    }

    pub fn get_skill(&mut self, key: &str) -> Option<TextureHandle> {
        self.skill_icon_cache.get(key).cloned()
    }

    pub fn get_unit(
        &mut self,
        unit_icon_id: &str,
        job_icon_id: &str,
        weapon_icon_id: &str,
    ) -> Option<TextureHandle> {
        let key = format!("{}_{}_{}", unit_icon_id, job_icon_id, weapon_icon_id);
        self.unit_icon_cache.get(&key).cloned()
    }

    pub fn get_versus(&mut self, key: &str) -> Option<TextureHandle> {
        self.versus_cache.get(key).cloned()
    }

    pub fn get_hub_icon(&mut self, key: &str) -> Option<TextureHandle> {
        self.hub_icons_cache.get(key).cloned()
    }

    pub fn get_hub_cafe_icon(&mut self, key: &str) -> Option<TextureHandle> {
        self.hub_cafe_icons_cache.get(key).cloned()
    }

    pub fn get_fish(&mut self, key: &str) -> Option<TextureHandle> {
        self.fish_cache.get(key).cloned()
    }

    pub fn get_achievement(&mut self, key: &str) -> Option<TextureHandle> {
        self.achievement_cache.get(key).cloned()
    }

    pub fn get_notebook(&mut self, key: &str) -> Option<TextureHandle> {
        self.notebook_cache.get(key).cloned()
    }

    pub fn get_relay_stamp(&mut self, key: &str) -> Option<TextureHandle> {
        self.relay_stamp_cache.get(key).cloned()
    }

    pub fn get_map_status(&mut self, key: &str) -> Option<TextureHandle> {
        self.map_status_cache.get(key).cloned()
    }
}
