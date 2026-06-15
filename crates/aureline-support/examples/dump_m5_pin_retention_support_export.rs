//! Dumps the metadata-safe M5 pin / retention support export as pretty JSON.
//! Used to regenerate the golden fixture at
//! `fixtures/storage/m5_pin_retention/support_export.golden.json`.

use aureline_support::m5_pin_retention::current_pin_retention_manager_corpus;

fn main() {
    let corpus = current_pin_retention_manager_corpus().expect("corpus parses");
    let export =
        corpus.support_export("support_export.m5_pin_retention.v1", "2026-06-14T00:00:00Z");
    println!(
        "{}",
        serde_json::to_string_pretty(&export).expect("serialize support export")
    );
}
