use comics_list_tauri::db::{
    create_publisher, create_series as db_create_series, delete_series as db_delete_series,
    list_series_paginated, update_series as db_update_series,
};

use crate::common::with_test_db;

#[test]
fn create_series_persists_and_lists_data() {
    with_test_db(|conn| {
        let publisher_id = create_publisher(conn, "Comics").expect("create publisher");

        db_create_series(
            conn,
            "Spawn",
            Some("Prima edizione"),
            Some(publisher_id),
            Some(1990),
            None,
            Some("note"),
            "2026-01-01T00:00:00Z",
        )
        .expect("create first series");

        db_create_series(
            conn,
            "Batman",
            None,
            Some(publisher_id),
            None,
            None,
            None,
            "2026-01-01T00:00:00Z",
        )
        .expect("create second series");

        let first_page = list_series_paginated(conn, None, 1, 0).expect("list first page");
        assert_eq!(first_page.items.len(), 1);
        assert_eq!(first_page.pagination.page, 1);
        assert_eq!(first_page.pagination.page_size, 1);
        assert_eq!(first_page.pagination.total_items, 2);
        assert_eq!(first_page.pagination.total_pages, 2);

        let first_item = &first_page.items[0];
        assert!(first_item.publisher.is_some());
        assert_eq!(
            first_item.publisher.as_ref().expect("publisher").id,
            publisher_id
        );
    });
}

#[test]
fn list_series_filters_by_title_and_ignores_empty_query() {
    with_test_db(|conn| {
        db_create_series(
            conn,
            "Spawn",
            Some("Prima edizione"),
            None,
            Some(1990),
            None,
            None,
            "2026-01-01T00:00:00Z",
        )
        .expect("create Spawn");
        db_create_series(
            conn,
            "Batman",
            Some("Detective Comics"),
            None,
            Some(1939),
            None,
            None,
            "2026-01-01T00:00:00Z",
        )
        .expect("create Batman");

        let all = list_series_paginated(conn, None, 10, 0).expect("list all series");
        assert_eq!(all.pagination.total_items, 2);

        let filtered = list_series_paginated(conn, Some("bat"), 10, 0).expect("filter series");
        assert_eq!(filtered.items.len(), 1);
        assert_eq!(filtered.items[0].title, "Batman");

        let empty_query =
            list_series_paginated(conn, Some("   "), 10, 0).expect("list with empty query");
        assert_eq!(empty_query.pagination.total_items, 2);
    });
}

#[test]
fn update_series_updates_existing_row() {
    with_test_db(|conn| {
        let publisher_id = create_publisher(conn, "Comics").expect("create publisher");

        let first_series_id = db_create_series(
            conn,
            "Spawn",
            Some("Prima edizione"),
            Some(publisher_id),
            Some(1990),
            None,
            Some("note"),
            "2026-01-01T00:00:00Z",
        )
        .expect("create first series");

        let updated_id = db_update_series(
            conn,
            first_series_id,
            "Spawn Deluxe",
            Some("Edizione aggiornata"),
            Some(publisher_id),
            Some(1991),
            Some(1999),
            Some("updated"),
            "2026-01-02T00:00:00Z",
        )
        .expect("update series");
        assert_eq!(updated_id, first_series_id);
    });
}

#[test]
fn delete_existing_series() {
    with_test_db(|conn| {
        let publisher_id = create_publisher(conn, "Comics").expect("create publisher");

        let first_series_id = db_create_series(
            conn,
            "Spawn",
            Some("Prima edizione"),
            Some(publisher_id),
            Some(1990),
            None,
            Some("note"),
            "2026-01-01T00:00:00Z",
        )
        .expect("create first series");

        db_delete_series(conn, first_series_id).expect("delete series");
    });
}
