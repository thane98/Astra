use std::borrow::Cow;
use std::marker::PhantomData;

use egui::TextureHandle;
use indexmap::IndexMap;

use crate::{DecorationKind, KeyedViewItem, SheetHandle, SheetRetriever, ViewItem};

#[derive(Clone)]
pub struct CacheItem<I> {
    key: String,
    text: String,
    drop_down_decoration: Option<(TextureHandle, f32)>,
    phantom: PhantomData<I>,
}

impl<I> CacheItem<I>
where
    I: KeyedViewItem,
{
    /// Cache the given model item.
    pub fn cache(item: &I, dependencies: &I::Dependencies) -> Self {
        Self {
            key: item.key().to_string(),
            text: item.text(dependencies).to_string(),
            drop_down_decoration: item.decoration(dependencies, DecorationKind::DropDown),
            phantom: PhantomData,
        }
    }
}

impl<I> ViewItem for CacheItem<I>
where
    I: ViewItem,
{
    type Dependencies = ();

    fn text(&self, _: &Self::Dependencies) -> Cow<'_, str> {
        Cow::Borrowed(&self.text)
    }

    fn decorated(kind: DecorationKind<'_>) -> bool {
        I::decorated(kind)
    }

    fn decoration(
        &self,
        _: &Self::Dependencies,
        kind: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        if let DecorationKind::DropDown = kind {
            self.drop_down_decoration.clone()
        } else {
            None
        }
    }
}

impl<I> KeyedViewItem for CacheItem<I>
where
    I: ViewItem,
{
    fn key(&self) -> Cow<'_, str> {
        Cow::Borrowed(&self.key)
    }

    fn set_key(&mut self, _: String) {
        unimplemented!("Modify the source item instead")
    }
}

/// A cached view over [KeyedViewItem]. This is useful for models with self references.
pub struct CachedModel<I> {
    cache: IndexMap<String, CacheItem<I>>,
}

impl<I> CachedModel<I>
where
    I: KeyedViewItem,
{
    /// Create a cache containing the given items.
    pub fn cache(items: &IndexMap<String, I>, dependencies: &I::Dependencies) -> Self {
        Self {
            cache: items
                .iter()
                .map(|(key, item)| (key.clone(), CacheItem::cache(item, dependencies)))
                .collect(),
        }
    }

    /// Retrieve the cached model.
    pub fn model(&self) -> &IndexMap<String, CacheItem<I>> {
        &self.cache
    }
}

/// A [CachedModel] manager that rebuilds the cache whenever its [SheetHandle] changes.
/// Currently uses coarse grained detection so every update is a FULL rebuild.
pub struct CachedView<R, B, I> {
    cache: CachedModel<I>,
    sheet: SheetHandle<R, B, IndexMap<String, I>>,
    revision_number: usize,
}

impl<R, B, I> CachedView<R, B, I>
where
    R: SheetRetriever<B, IndexMap<String, I>>,
    I: KeyedViewItem,
{
    /// Build a [CachedModel] from the given sheet.
    pub fn new(
        sheet: SheetHandle<R, B, IndexMap<String, I>>,
        dependencies: &I::Dependencies,
    ) -> Self {
        let cache = sheet.read(|data| CachedModel::cache(data, dependencies));
        let prev_edit_count = sheet.revision_number();
        Self {
            cache,
            sheet,
            revision_number: prev_edit_count,
        }
    }

    /// Refresh the [CachedModel] if there are any updates.
    pub fn refresh(&mut self, dependencies: &I::Dependencies) {
        if self.sheet.revision_number() > self.revision_number {
            let cache = self
                .sheet
                .read(|data| CachedModel::cache(data, dependencies));
            self.cache = cache;
            self.revision_number = self.sheet.revision_number();
        }
    }

    /// Retrieve the model.
    pub fn get(&self) -> &IndexMap<String, CacheItem<I>> {
        self.cache.model()
    }
}
