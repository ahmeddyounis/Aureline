//! Dumps the metadata-safe M5 clear-data review support export as pretty JSON.
//! Used to regenerate the golden fixture at
//! `fixtures/storage/m5_clear_data_review/support_export.golden.json`.

use aureline_support::m5_clear_data_review::current_clear_data_review_corpus;

fn main() {
    let corpus = current_clear_data_review_corpus().expect("corpus parses");
    let export = corpus.support_export(
        "support_export.m5_clear_data_review.v1",
        "2026-06-14T00:00:00Z",
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&export).expect("serialize support export")
    );
}
