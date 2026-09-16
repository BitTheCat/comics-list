use comics_list_tauri::db::{create_publisher, list_publishers};

use crate::common::with_test_db;

#[test]
fn create_new_publisher() {
    with_test_db(|conn| {
        create_publisher(conn, "Comics").expect("insert comics");

        let all = list_publishers(conn, None).expect("list all publishers");
        assert_eq!(all.len(), 1);

        let filtered = list_publishers(conn, Some("mics")).expect("list filtered publishers");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Comics");
    });
}

#[test]
fn list_publishers_supports_optional_name_filter() {
    with_test_db(|conn| {
        create_publisher(conn, "Comics").expect("insert comics");
        create_publisher(conn, "Mics 1").expect("insert mics 1");

        let all = list_publishers(conn, None).expect("list all publishers");
        assert_eq!(all.len(), 2);

        let filtered = list_publishers(conn, Some("comics")).expect("list filtered publishers");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "Comics");
    });
}
