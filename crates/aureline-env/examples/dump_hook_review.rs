//! Dumps the declared lifecycle hooks, the per-scenario review packets, their
//! metadata-first exports, the failure / recovery drills, and the fixture
//! corpus.
//!
//! With no argument it prints `{ "hooks": ..., "packets": ..., "exports": ...,
//! "drills": ..., "fixtures": [...] }` for human inspection. Pass `hooks`,
//! `packets`, `exports`, `drills`, or `fixtures` to print only that view. The
//! `fixtures` form is the one written, per fixture, under
//! `fixtures/env/hook-review/`.

use aureline_env::{
    export_hook_review, seeded_hook_review_drills, seeded_hook_review_fixtures,
    seeded_hook_review_packets, seeded_lifecycle_hooks,
};

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let value = match mode.as_str() {
        "hooks" => serde_json::to_value(seeded_lifecycle_hooks()),
        "packets" => serde_json::to_value(seeded_hook_review_packets()),
        "exports" => serde_json::to_value(
            seeded_hook_review_packets()
                .iter()
                .map(export_hook_review)
                .collect::<Vec<_>>(),
        ),
        "drills" => serde_json::to_value(seeded_hook_review_drills()),
        "fixtures" => serde_json::to_value(seeded_hook_review_fixtures()),
        _ => serde_json::to_value(serde_json::json!({
            "hooks": seeded_lifecycle_hooks(),
            "packets": seeded_hook_review_packets(),
            "exports": seeded_hook_review_packets()
                .iter()
                .map(export_hook_review)
                .collect::<Vec<_>>(),
            "drills": seeded_hook_review_drills(),
            "fixtures": seeded_hook_review_fixtures(),
        })),
    }
    .expect("hooks, packets, exports, drills, and fixtures serialize");
    println!(
        "{}",
        serde_json::to_string_pretty(&value).expect("pretty JSON renders")
    );
}
