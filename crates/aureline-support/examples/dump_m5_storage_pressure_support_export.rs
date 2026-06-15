//! Dumps the metadata-safe M5 storage-pressure banner support export as pretty
//! JSON. Used to regenerate the golden fixture at
//! `fixtures/storage/m5_storage_pressure/support_export.golden.json`.

use aureline_support::m5_storage_pressure::current_storage_pressure_banner_corpus;

fn main() {
    let corpus = current_storage_pressure_banner_corpus().expect("corpus parses");
    let export = corpus.support_export(
        "support_export.m5_storage_pressure.v1",
        "2026-06-14T00:00:00Z",
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&export).expect("serialize support export")
    );
}
