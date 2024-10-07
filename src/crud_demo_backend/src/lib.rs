mod datatypes;
#[cfg(test)]
mod tests;

use datatypes::{Id, OrderedMap};
use std::cell::RefCell;

#[ic_cdk::pre_upgrade]
fn pre_upgrade() {
    STORAGE.with(|storage| {
        if let Err(e) = ic_cdk::storage::stable_save((storage,)) {
            ic_cdk::trap(&format!("Failed to save state: {e:?}"));
        }
    });
}

#[ic_cdk::post_upgrade]
fn post_upgrade() {
    let (storage,) = ic_cdk::storage::stable_restore::<(OrderedMap,)>().unwrap_or_default();
    STORAGE.with(|s| *s.borrow_mut() = storage);
}

thread_local! {
    static STORAGE: RefCell<OrderedMap> = RefCell::new(OrderedMap::new());
}

/// Get a single TODO item by its ID.
#[ic_cdk::query]
pub(crate) fn get_todo(id: Id) -> Option<String> {
    STORAGE.with(|storage| storage.borrow().get(&id).cloned())
}

/// Get all TODO items, with paging.
#[ic_cdk::query]
pub(crate) fn get_all_todos(offset: u64, limit: u64) -> Vec<(Id, String)> {
    STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|(id, value)| (id.clone(), value.clone()))
            .collect()
    })
}

/// Add a new TODO item.
#[ic_cdk::update]
pub(crate) fn create_todo(value: String) -> Id {
    let id = Id::new();
    STORAGE.with(|storage| {
        storage.borrow_mut().insert(id.clone(), value);
    });
    id
}

/// Delete a TODO item by its ID.
#[ic_cdk::update]
pub(crate) fn delete_todo(id: Id) {
    STORAGE.with(|storage| {
        storage.borrow_mut().remove(&id);
    });
}

/// Update a TODO item by its ID.
#[ic_cdk::update]
pub(crate) fn update_todo(id: Id, value: String) {
    STORAGE.with(|storage| {
        storage.borrow_mut().insert(id, value);
    });
}
