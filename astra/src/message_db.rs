use std::collections::HashMap;
use std::sync::Arc;

use astra_core::{Astra, OpenMessageArchive};
use astra_formats::MsbtToken;
use bimap::BiHashMap;
use parking_lot::RwLock;

use crate::{AppConfig, GodDataSheet, PersonSheet};

#[derive(Debug, Clone)]
struct KeyData {
    value: String,
    archive: usize,
}

#[derive(Clone)]
pub struct MessageDbWrapper(Arc<RwLock<MessageDb>>);

impl MessageDbWrapper {
    pub fn new(db: MessageDb) -> Self {
        Self(Arc::new(RwLock::new(db)))
    }

    pub fn message(&self, key: &str) -> Option<String> {
        self.0.read().message(key).map(|v| v.to_string())
    }

    pub fn with_message_mut(
        &self,
        key: &str,
        default_archive: &str,
        consumer: impl FnOnce(Option<&mut String>) -> bool,
    ) {
        self.0
            .write()
            .with_message_mut(key, default_archive, consumer)
    }

    pub fn build_translations(
        &self,
        person: &PersonSheet,
        god: &GodDataSheet,
        config: &AppConfig,
    ) -> BiHashMap<String, String> {
        let mut translations = BiHashMap::new();
        god.read(|data| {
            for god in data.values() {
                let key = god.gid.strip_prefix("GID_").unwrap_or_default();
                if !key.is_empty() {
                    if let Some(message) = self.message(&god.mid) {
                        if !translations.contains_left(key)
                            && !translations.contains_right(&message)
                        {
                            translations.insert(key.to_string(), message);
                        }
                    }
                }
            }
            for gid in &config.override_translation_keys_god {
                let key = gid.strip_prefix("GID_").unwrap_or_default();
                let message = data.get(gid).and_then(|god| self.message(&god.mid));
                if let Some(message) = message {
                    if !key.is_empty() && !message.is_empty() {
                        translations.insert(key.to_string(), message);
                    }
                }
            }
        });
        person.read(|data| {
            for person in data.values() {
                let key = person.pid.strip_prefix("PID_").unwrap_or_default();
                if !key.is_empty() {
                    if let Some(message) = self.message(&person.name) {
                        if !translations.contains_left(key)
                            && !translations.contains_right(&message)
                        {
                            translations.insert(key.to_string(), message);
                        }
                    }
                }
            }
            for pid in &config.override_translation_keys_god {
                let key = pid.strip_prefix("PID_").unwrap_or_default();
                let message = data.get(pid).and_then(|person| self.message(&person.name));
                if let Some(message) = message {
                    if !key.is_empty() && !message.is_empty() {
                        translations.insert(key.to_string(), message);
                    }
                }
            }
        });
        translations
    }

    pub fn translate_script(
        &self,
        script: &str,
        translations: &BiHashMap<String, String>,
    ) -> Option<String> {
        let mut entries = astra_formats::parse_astra_script(script).ok()?;
        for entry in entries.values_mut() {
            for token in entry {
                match token {
                    MsbtToken::Window { speaker, .. } => {
                        if let Some(translation) = translations.get_by_left(speaker).cloned() {
                            *speaker = translation;
                        }
                    }
                    MsbtToken::Animation { target, .. } => {
                        if let Some(translation) = translations.get_by_left(target).cloned() {
                            *target = translation;
                        }
                    }
                    MsbtToken::Alias { actual, .. } => {
                        if let Some(translation) = translations.get_by_left(actual).cloned() {
                            *actual = translation;
                        }
                    }
                    _ => {}
                }
            }
        }

        // TODO: Add a utility to astra_formats so we don't have to parse repeatedly.
        let script = astra_formats::pack_msbt_entries(&entries);
        astra_formats::parse_msbt_script(&script).ok()
    }

    pub fn untranslate_script(
        &self,
        script: &str,
        translations: &BiHashMap<String, String>,
    ) -> Option<String> {
        let mut entries = astra_formats::parse_astra_script(script).ok()?;
        for entry in entries.values_mut() {
            for token in entry {
                match token {
                    MsbtToken::Window { speaker, .. } => {
                        if let Some(translation) = translations.get_by_right(speaker).cloned() {
                            *speaker = translation;
                        }
                    }
                    MsbtToken::Animation { target, .. } => {
                        if let Some(translation) = translations.get_by_right(target).cloned() {
                            *target = translation;
                        }
                    }
                    MsbtToken::Alias { actual, .. } => {
                        if let Some(translation) = translations.get_by_right(actual).cloned() {
                            *actual = translation;
                        }
                    }
                    _ => {}
                }
            }
        }

        // TODO: Add a utility to astra_formats so we don't have to parse repeatedly.
        let script = astra_formats::pack_msbt_entries(&entries);
        astra_formats::parse_msbt_script(&script).ok()
    }
}

pub struct MessageDb {
    messages: HashMap<String, KeyData>,
    archives: Vec<OpenMessageArchive>,
    archives_by_name: HashMap<String, usize>,
    cobalt_msbt: Option<String>,
}

impl MessageDb {
    pub fn new(astra: Arc<RwLock<Astra>>) -> Self {
        let mut archives = vec![];
        let mut archives_by_name = HashMap::new();
        let mut messages = HashMap::new();
        let astra = astra.read();
        for archive_id in astra.list_archives() {
            if let Some(archive) = astra.get_archive(archive_id) {
                archive.read(|data| {
                    for (key, value) in data.clone() {
                        messages.insert(
                            key,
                            KeyData {
                                value,
                                archive: archives.len(),
                            },
                        );
                    }
                });
                archives_by_name.insert(archive_id.clone(), archives.len());
                archives.push(archive.clone());
            }
        }
        Self {
            messages,
            archives,
            archives_by_name,
            cobalt_msbt: astra.cobalt_msbt(),
        }
    }

    pub fn message(&self, key: &str) -> Option<&str> {
        self.messages.get(key).map(|data| data.value.as_str())
    }

    pub fn with_message_mut(
        &mut self,
        key: &str,
        default_archive: &str,
        consumer: impl FnOnce(Option<&mut String>) -> bool,
    ) {
        if key.is_empty() {
            consumer(None);
            return;
        }

        if let Some(mut data) = self.retrieve_data(key, default_archive) {
            let changed = consumer(Some(&mut data.value));
            if changed {
                let archive_index = self
                    .cobalt_msbt
                    .as_deref()
                    .and_then(|archive| self.archives_by_name.get(archive))
                    .copied()
                    .unwrap_or(data.archive);
                if let Some(archive) = self.archives.get(archive_index) {
                    data.archive = archive_index;
                    archive.write(|message_map| {
                        message_map.insert(key.to_string(), data.value.clone());
                        self.messages.insert(key.to_string(), data);
                        true
                    });
                }
            }
        } else {
            consumer(None);
        }
    }

    fn retrieve_data(&mut self, key: &str, default_archive: &str) -> Option<KeyData> {
        if !self.messages.contains_key(key) {
            let data = KeyData {
                archive: *self
                    .cobalt_msbt
                    .as_deref()
                    .and_then(|archive| self.archives_by_name.get(archive))
                    .or_else(|| self.archives_by_name.get(default_archive))?,
                value: String::new(),
            };
            return Some(data);
        }
        self.messages.get_mut(key).cloned()
    }
}
