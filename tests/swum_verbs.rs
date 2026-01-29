#![allow(clippy::unwrap_used)]

use semmap::swum;

#[test]
fn swum_update_verbs() {
    assert_eq!(swum::expand_identifier("update_cache"), "Updates cache.");
    assert_eq!(swum::expand_identifier("sync_data"), "Updates data.");
    assert_eq!(swum::expand_identifier("refresh_token"), "Updates token.");
}

#[test]
fn swum_delete_verbs() {
    assert_eq!(swum::expand_identifier("delete_user"), "Removes user.");
    assert_eq!(swum::expand_identifier("remove_file"), "Removes file.");
    assert_eq!(swum::expand_identifier("drop_table"), "Removes table.");
    assert_eq!(swum::expand_identifier("clear_buffer"), "Removes buffer.");
}

#[test]
fn swum_render_verbs() {
    assert_eq!(
        swum::expand_identifier("render_page"),
        "Formats page for output."
    );
    assert_eq!(
        swum::expand_identifier("format_string"),
        "Formats string for output."
    );
    assert_eq!(
        swum::expand_identifier("display_info"),
        "Formats info for output."
    );
}

#[test]
fn swum_convert_verbs() {
    assert_eq!(swum::expand_identifier("convert_json"), "Converts json.");
    assert_eq!(swum::expand_identifier("transform_data"), "Converts data.");
    assert_eq!(swum::expand_identifier("map_values"), "Converts values.");
}

#[test]
fn swum_handle_verbs() {
    assert_eq!(
        swum::expand_identifier("handle_request"),
        "Processes request."
    );
    assert_eq!(
        swum::expand_identifier("process_payment"),
        "Processes payment."
    );
    assert_eq!(swum::expand_identifier("run_job"), "Processes job.");
}

#[test]
fn swum_find_verbs() {
    assert_eq!(swum::expand_identifier("find_item"), "Finds item.");
    assert_eq!(swum::expand_identifier("search_db"), "Finds db.");
    assert_eq!(swum::expand_identifier("lookup_key"), "Finds key.");
}

#[test]
fn swum_test_verbs() {
    assert_eq!(swum::expand_identifier("test_function"), "Tests function.");
    assert_eq!(
        swum::expand_identifier("spec_validation"),
        "Tests validation."
    );
}
