use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

use astra_core::{Astra, OpenMessageArchive};
use parking_lot::RwLock;

#[derive(Debug, Clone)]
struct KeyData {
    value: String,
    archive: usize,
}

pub struct MessageDbWrapper(Rc<RefCell<MessageDb>>);

impl MessageDbWrapper {
    pub fn new(db: MessageDb) -> Self {
        Self(Rc::new(RefCell::new(db)))
    }

    pub fn message(&self, key: &str) -> Option<String> {
        self.0.borrow().message(key).map(|v| v.to_string())
    }

    pub fn with_message_mut(
        &self,
        key: &str,
        default_archive: &str,
        consumer: impl FnOnce(Option<&mut String>) -> bool,
    ) {
        self.0
            .borrow_mut()
            .with_message_mut(key, default_archive, consumer)
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

        if let Some(data) = self.retrieve_data(key, default_archive) {
            let changed = consumer(Some(&mut data.value));
            if changed {
                let data = data.clone();
                if let Some(archive) = self.archives.get(data.archive) {
                    archive.write(|message_map| {
                        message_map.insert(key.to_string(), data.value);
                        true
                    });
                }
            }
        } else {
            consumer(None);
        }
    }

    fn retrieve_data(&mut self, key: &str, default_archive: &str) -> Option<&mut KeyData> {
        if !self.messages.contains_key(key) {
            let data = KeyData {
                archive: *self
                    .cobalt_msbt
                    .as_deref()
                    .and_then(|archive| self.archives_by_name.get(archive))
                    .or_else(|| self.archives_by_name.get(default_archive))?,
                value: String::new(),
            };
            self.messages.insert(key.to_string(), data);
        }
        return self.messages.get_mut(key);
    }
}
