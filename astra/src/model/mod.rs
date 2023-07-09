mod cached_view;
mod config;
mod sheet;
mod theme;

pub use cached_view::*;
pub use config::*;
use itertools::Itertools;
pub use sheet::*;
pub use theme::*;

use std::borrow::Cow;

use egui::TextureHandle;
use indexmap::IndexMap;

/// Where the decoration will be displayed. Used to provide context when requesting a decoration from an item.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecorationKind<'a> {
    List,
    DropDown,
    Other(&'a str),
}

/// An item that could be rendered in a view. Typically part of a collection of similar items stored in a model.
pub trait ViewItem: Clone {
    type Dependencies;

    /// Indicates whether ANY item of this type could be decorated.
    /// If not, widgets may skip allocating space for decorations and use a simpler layout.
    #[allow(unused)]
    fn decorated(kind: DecorationKind<'_>) -> bool {
        false
    }

    /// Retrieve the display text for this item using the given dependencies.
    #[allow(unused)]
    fn text(&self, dependencies: &Self::Dependencies) -> Cow<'_, str>;

    /// Retrieve the decoration for this item and the recommended scale to display it with.
    /// The [DecorationKind] may be used to provide different decorations based on the context.
    #[allow(unused)]
    fn decoration(
        &self,
        dependencies: &Self::Dependencies,
        kind: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        None
    }
}

/// A [ViewItem] that has a unique ID distinguishing it from other items.
pub trait KeyedViewItem: ViewItem {
    /// Retrieve the key from this item.
    fn key(&self) -> Cow<'_, str>;

    fn set_key(&mut self, key: String);
}

/// An array-like of [ViewItem] that can be rendered in collection widgets.
pub trait ListModel<I> {
    /// The number of items in this model.
    fn len(&self) -> usize;

    /// Retrieve the item at the given index (if in bounds)
    fn item(&self, index: usize) -> Option<&I>;

    /// Retrieve a mutable reference to an item if the index is in bounds.
    fn item_mut(&mut self, index: usize) -> Option<&mut I>;

    /// Add an item to the end of this model (if supported)
    fn add(&mut self, item: I);

    /// Insert an item at the specified index (if in bounds)
    fn insert(&mut self, index: usize, item: I);

    /// Remove the item at the given index (if in bounds)
    fn remove(&mut self, index: usize);

    /// Swap items at the given indices (if in bounds)
    fn swap(&mut self, a: usize, b: usize);

    /// Copy the contents of index `a` to index `b`.
    fn copy(&mut self, a: usize, b: usize);
}

impl<I> ListModel<I> for Vec<I>
where
    I: ViewItem,
{
    fn len(&self) -> usize {
        self.len()
    }

    fn item(&self, index: usize) -> Option<&I> {
        self.get(index)
    }

    fn item_mut(&mut self, index: usize) -> Option<&mut I> {
        self.get_mut(index)
    }

    fn add(&mut self, item: I) {
        self.push(item);
    }

    fn insert(&mut self, index: usize, item: I) {
        if index <= self.len() {
            self.insert(index, item);
        }
    }

    fn remove(&mut self, index: usize) {
        if index < self.len() {
            self.remove(index);
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        if a < self.len() && b < self.len() {
            self.swap(a, b);
        }
    }

    fn copy(&mut self, a: usize, b: usize) {
        if a < self.len() && b < self.len() {
            self[b] = self[a].clone();
        }
    }
}

impl<I> ListModel<I> for IndexMap<String, I>
where
    I: KeyedViewItem,
{
    fn len(&self) -> usize {
        self.len()
    }

    fn item(&self, index: usize) -> Option<&I> {
        self.get_index(index).map(|(_, v)| v)
    }

    fn item_mut(&mut self, index: usize) -> Option<&mut I> {
        self.get_index_mut(index).map(|(_, v)| v)
    }

    fn add(&mut self, item: I) {
        let key = item.key();
        if !self.contains_key(key.as_ref()) {
            self.insert(key.into_owned(), item);
        }
    }

    fn insert(&mut self, index: usize, item: I) {
        if index <= self.len() {
            self.add(item);
            self.move_index(self.len() - 1, index);
        }
    }

    fn remove(&mut self, index: usize) {
        if index < self.len() {
            self.shift_remove_index(index);
        }
    }

    fn swap(&mut self, a: usize, b: usize) {
        if a < self.len() && b < self.len() {
            self.swap_indices(a, b);
        }
    }

    fn copy(&mut self, a: usize, b: usize) {
        if let Some(key) = self.get_index(b).map(|(k, _)| k).cloned() {
            if let Some(mut a) = self.get_index(a).map(|(_, v)| v).cloned() {
                a.set_key(key.clone());
                self.insert(key, a);
            }
        }
    }
}

/// A [ListModel] of items which have a unique ID.
pub trait KeyedListModel<I>: ListModel<I> {
    /// Retrieve the kind of a [ViewItem] from its key.
    fn index_of(&self, key: &str) -> Option<usize>;

    fn item_keyed(&self, key: &str) -> Option<&I> {
        self.index_of(key).and_then(|index| self.item(index))
    }

    fn contains(&self, key: &str) -> bool {
        self.index_of(key).is_some()
    }
}

impl<I> KeyedListModel<I> for IndexMap<String, I>
where
    I: KeyedViewItem,
{
    fn index_of(&self, key: &str) -> Option<usize> {
        self.get_index_of(key)
    }
}

/// A utility for showing filtered data on a UI by generating a "proxy model".
pub struct FilterProxyBuilder {
    filter_expr: String,
    requires_refresh: bool,
    proxy_indices: Vec<usize>,
}

impl FilterProxyBuilder {
    pub fn new() -> Self {
        Self {
            filter_expr: String::new(),
            requires_refresh: true,
            proxy_indices: vec![],
        }
    }

    /// Retrieve the index of an item in the source model from its index in the proxy model.
    pub fn source_index<M, I>(&self, index: usize, source_model: &M) -> Option<usize>
    where
        M: ListModel<I>,
    {
        // The client might call this without refreshing after a source model change (but they shouldn't).
        // Compare against the source model's length to be safe.
        self.proxy_indices
            .get(index)
            .and_then(|real_index| (*real_index < source_model.len()).then_some(*real_index))
    }

    /// Retrieve the proxy index of an item from its index in the source model.
    pub fn proxy_index(&self, source_index: usize) -> Option<usize> {
        self.proxy_indices
            .iter()
            .find_position(|index| **index == source_index)
            .map(|(proxy_index, _)| proxy_index)
    }

    /// Access this proxy's filter expression. The function must return true if the filter changes.
    pub fn with_filter_expr(&mut self, func: impl FnOnce(&mut String) -> bool) {
        if func(&mut self.filter_expr) {
            self.requires_refresh = true;
        }
    }

    /// Build a filtered view of a source model using this proxy's filter expression.
    /// Will ONLY trigger a refresh when required. There are three cases for this:
    /// * First time building a proxy (detected automatically)
    /// * Filter expression was altered (detected automatically)
    /// * Source model was changed. YOU must tell the proxy when this happened.
    pub fn model<'a, M, I, D>(
        &'a mut self,
        requires_refresh: bool,
        model: &'a M,
        dependencies: &D,
    ) -> FilterProxyModel<'_, M>
    where
        M: ListModel<I>,
        I: ViewItem<Dependencies = D>,
    {
        if self.requires_refresh || requires_refresh {
            self.requires_refresh = false;
            self.proxy_indices.clear();
            for i in 0..model.len() {
                let matches_filter = model
                    .item(i)
                    .map(|item| {
                        item.text(dependencies)
                            .to_lowercase()
                            .contains(&self.filter_expr.to_lowercase())
                    })
                    .unwrap_or_default();
                if matches_filter {
                    self.proxy_indices.push(i);
                }
            }
        }

        FilterProxyModel {
            proxy_indices: &self.proxy_indices,
            model,
        }
    }
}

/// A filtered view of a [ListModel] generated by a [FilterProxyBuilder].
pub struct FilterProxyModel<'a, M> {
    proxy_indices: &'a [usize],
    model: &'a M,
}

impl<'a, M, I> ListModel<I> for FilterProxyModel<'a, M>
where
    M: ListModel<I>,
{
    fn len(&self) -> usize {
        self.proxy_indices.len()
    }

    fn item(&self, index: usize) -> Option<&I> {
        self.proxy_indices
            .get(index)
            .and_then(|source_index| self.model.item(*source_index))
    }

    fn item_mut(&mut self, _: usize) -> Option<&mut I> {
        unimplemented!("modify the source model instead")
    }

    fn add(&mut self, _item: I) {
        unimplemented!("modify the source model instead")
    }

    fn insert(&mut self, _: usize, _: I) {
        unimplemented!("modify the source model instead")
    }

    fn remove(&mut self, _index: usize) {
        unimplemented!("modify the source model instead")
    }

    fn swap(&mut self, _: usize, _: usize) {
        unimplemented!("modify the source model instead")
    }

    fn copy(&mut self, _: usize, _: usize) {
        unimplemented!("modify the source model instead")
    }
}

/// A trait to retrieve display info for a group.
pub trait GroupViewItem {
    type Dependencies;

    /// Retrieve the display text for a group item with the given key.
    fn text<'a>(key: &'a str, dependencies: &'a Self::Dependencies) -> Cow<'a, str>;

    /// Indicates whether ANY item of this type could be decorated.
    /// If not, widgets may skip allocating space for decorations and use a simpler layout.
    #[allow(unused)]
    fn decorated(kind: DecorationKind<'_>) -> bool {
        false
    }

    #[allow(unused)]
    fn decoration(
        key: &str,
        dependencies: &Self::Dependencies,
        kind: DecorationKind<'_>,
    ) -> Option<(TextureHandle, f32)> {
        None
    }
}
