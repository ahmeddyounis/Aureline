//! Emits the downgraded semantic-claims matrix as JSON to stdout.
//!
//! Run with:
//!
//! ```
//! cargo run -p aureline-language --example dump_downgraded_semantic_claims_matrix
//! ```

use aureline_language::provider_arbitration::{
    build_downgraded_semantic_claims_matrix, current_provider_arbitration_proof_corpus,
};

fn main() {
    let corpus = current_provider_arbitration_proof_corpus().expect("proof corpus parses");
    let matrix = build_downgraded_semantic_claims_matrix(
        &corpus,
        "language:downgraded_semantic_claims_matrix:2026-05-19:01",
        "2026-05-19T19:00:00Z",
    );
    let json = serde_json::to_string_pretty(&matrix).expect("matrix serializes to JSON");
    println!("{json}");
}
