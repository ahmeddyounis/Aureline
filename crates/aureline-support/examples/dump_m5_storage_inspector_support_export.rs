//! Dumps the metadata-safe M5 storage-inspector support export as pretty JSON.
//! Used to regenerate the golden fixture at
//! `fixtures/storage/m5_storage_inspector/support_export.golden.json`.

use aureline_support::m5_storage_inspector::current_storage_inspector_corpus;

fn main() {
    let corpus = current_storage_inspector_corpus().expect("corpus parses");
    let export = corpus.support_export(
        "support_export.m5_storage_inspector.v1",
        "2026-06-14T00:00:00Z",
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&export).expect("serialize support export")
    );
}
