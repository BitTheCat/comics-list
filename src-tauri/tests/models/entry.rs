use comics_list_tauri::db::{
    create_entry, create_series, delete_entry, list_entries_paginated, update_entry, next_issue_number
};

use crate::common::with_test_db;

#[test]
fn create_entry_persists_and_lists_data() {
    with_test_db(|conn| {
        let series_id = create_series(
            conn,
            "Spawn",
            None,
            None,
            Some(1990),
            None,
            None,
            "2026-01-01T00:00:00Z",
        )
        .expect("create series");

        let entry_id = create_entry(
            conn,
            series_id,
            "1",
            Some("The Beginning"),
            Some(1990),
            Some(1),
            Some("cover.png"),
            Some("Marvel,Hero"),
            Some("NM"),
            "2026-01-01T00:00:00Z",
        )
        .expect("create entry");

        let page = list_entries_paginated(conn, series_id, None, 10, 0).expect("list entries");
        assert_eq!(page.items.len(), 1);
        assert_eq!(page.pagination.total_items, 1);

        let entry = &page.items[0];
        assert_eq!(entry.id, entry_id);
        assert_eq!(entry.series_id, series_id);
        assert_eq!(entry.issue_number, "1");
        assert_eq!(entry.title.as_deref(), Some("The Beginning"));
        assert_eq!(entry.pub_year, Some(1990));
        assert_eq!(entry.pub_month, Some(1));
        assert_eq!(entry.cover_path.as_deref(), Some("cover.png"));
        assert_eq!(entry.tags, vec!["Marvel", "Hero"]);
        assert_eq!(entry.condition.as_deref(), Some("NM"));
    });
}

#[test]
fn update_entry_updates_existing_row() {
    with_test_db(|conn| {
        let series_id = create_series(
            conn,
            "Spawn",
            None,
            None,
            Some(1990),
            None,
            None,
            "2026-01-01T00:00:00Z",
        )
        .expect("create series");
        let entry_id = create_entry(
            conn,
            series_id,
            "1",
            Some("The Beginning"),
            Some(1990),
            Some(1),
            None,
            Some("Marvel"),
            Some("NM"),
            "2026-01-01T00:00:00Z",
        )
        .expect("create entry");

        let updated_id = update_entry(
            conn,
            entry_id,
            series_id,
            "1A",
            Some("The Return"),
            Some(1991),
            Some(2),
            Some("updated.png"),
            Some("Image,Hero"),
            Some("VF"),
            "2026-01-02T00:00:00Z",
        )
        .expect("update entry");
        assert_eq!(updated_id, entry_id);

        let page =
            list_entries_paginated(conn, series_id, None, 10, 0).expect("list updated entry");
        let entry = &page.items[0];
        assert_eq!(entry.issue_number, "1A");
        assert_eq!(entry.title.as_deref(), Some("The Return"));
        assert_eq!(entry.pub_year, Some(1991));
        assert_eq!(entry.pub_month, Some(2));
        assert_eq!(entry.tags, vec!["Image", "Hero"]);
        assert_eq!(entry.condition.as_deref(), Some("VF"));
    });
}

#[test]
fn delete_existing_entry() {
    with_test_db(|conn| {
        let series_id = create_series(
            conn,
            "Spawn",
            None,
            None,
            Some(1990),
            None,
            None,
            "2026-01-01T00:00:00Z",
        )
        .expect("create series");
        let entry_id = create_entry(
            conn,
            series_id,
            "1",
            Some("The Beginning"),
            None,
            None,
            None,
            None,
            None,
            "2026-01-01T00:00:00Z",
        )
        .expect("create entry");

        delete_entry(conn, entry_id).expect("delete entry");

        let page = list_entries_paginated(conn, series_id, None, 10, 0)
            .expect("list entries after delete");
        assert!(page.items.is_empty());
        assert_eq!(page.pagination.total_items, 0);
    });
}

#[test]
fn list_entries_filters_by_title_tags_and_condition() {
    with_test_db(|conn| {
        let series_id = create_series(
            conn,
            "Spawn",
            None,
            None,
            Some(1990),
            None,
            None,
            "2026-01-01T00:00:00Z",
        )
        .expect("create series");

        create_entry(
            conn,
            series_id,
            "1",
            Some("The Beginning"),
            Some(1990),
            Some(1),
            None,
            Some("Marvel,Hero"),
            Some("NM"),
            "2026-01-01T00:00:00Z",
        )
        .expect("create first entry");
        create_entry(
            conn,
            series_id,
            "2",
            Some("The Return"),
            Some(1991),
            Some(2),
            None,
            Some("Image"),
            Some("VF"),
            "2026-01-01T00:00:00Z",
        )
        .expect("create second entry");

        let all = list_entries_paginated(conn, series_id, None, 10, 0).expect("list all entries");
        assert_eq!(all.pagination.total_items, 2);

        let title_match = list_entries_paginated(conn, series_id, Some("return"), 10, 0)
            .expect("filter by title");
        assert_eq!(title_match.items.len(), 1);
        assert_eq!(title_match.items[0].issue_number, "2");

        let tag_match =
            list_entries_paginated(conn, series_id, Some("hero"), 10, 0).expect("filter by tag");
        assert_eq!(tag_match.items.len(), 1);
        assert_eq!(tag_match.items[0].tags, vec!["Marvel", "Hero"]);

        let condition_match = list_entries_paginated(conn, series_id, Some("vf"), 10, 0)
            .expect("filter by condition");
        assert_eq!(condition_match.items.len(), 1);
        assert_eq!(condition_match.items[0].condition.as_deref(), Some("VF"));

        let empty_query = list_entries_paginated(conn, series_id, Some("   "), 10, 0)
            .expect("list with empty query");
        assert_eq!(empty_query.pagination.total_items, 2);
    });
}

#[test]
fn test_api_retrieve_next_issue_number() {
    with_test_db(|conn| {
        let series_id = create_series(conn, "Test Series", None, None, None, None, None, "2024-01-01T00:00:00Z").expect("create series");
        let next_issue = next_issue_number(conn, series_id).expect("retrieve next issue");
        assert_eq!(next_issue, "1");

        create_entry(conn, series_id, "4", None, None, None, None, None, None, "2024-01-01T00:00:00Z")
            .expect("create entry");

        let next_issue = next_issue_number(conn, series_id).expect("retrieve next issue");
        assert_eq!(next_issue, "5");

        create_entry(conn, series_id, "10a", None, None, None, None, None, None, "2024-01-01T00:00:00Z")
            .expect("create entry");
        let next_issue = next_issue_number(conn, series_id).expect("retrieve next issue");
        assert_eq!(next_issue, "11");
        
    });
    
}
