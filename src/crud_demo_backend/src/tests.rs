use crate::{create_todo, delete_todo, get_all_todos, get_todo, update_todo};

#[test]
fn test_set_get() {
    let id = create_todo("Alice".to_string());
    assert_eq!(get_todo(id), Some("Alice".to_string()));
}

#[test]
fn test_get_all_with_paging() {
    let id1 = create_todo("Alice".to_string());
    let id2 = create_todo("Bob".to_string());
    let id3 = create_todo("Charlie".to_string());

    assert_eq!(
        get_all_todos(0, 2),
        vec![
            (id1.clone(), "Alice".to_string()),
            (id2.clone(), "Bob".to_string())
        ]
    );
    assert_eq!(
        get_all_todos(1, 2),
        vec![
            (id2.clone(), "Bob".to_string()),
            (id3.clone(), "Charlie".to_string())
        ]
    );
}

#[test]
fn test_delete() {
    let id = create_todo("Alice".to_string());
    delete_todo(id.clone());
    assert_eq!(get_todo(id), None);
}

#[test]
fn test_update() {
    let id = create_todo("Alice".to_string());
    update_todo(id.clone(), "Bob".to_string());
    assert_eq!(get_todo(id), Some("Bob".to_string()));
}
