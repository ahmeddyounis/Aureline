//! Dumps the typed environment capsules, their why-this-environment
//! inspections, and the fixture corpus.
//!
//! With no argument it prints `{ "capsules": ..., "inspections": ...,
//! "fixtures": [...] }` for human inspection. Pass `capsules`,
//! `inspections`, `exports`, or `fixtures` to print only that view. The
//! `fixtures` form is the one written, per fixture, under
//! `fixtures/env/environment-capsule/`.

use aureline_env::{
    export_capsule_metadata, inspect_environment, seeded_environment_capsule_fixtures,
    seeded_environment_capsules,
};

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let capsules = seeded_environment_capsules();
    let value = match mode.as_str() {
        "capsules" => serde_json::to_value(&capsules),
        "inspections" => {
            serde_json::to_value(capsules.iter().map(inspect_environment).collect::<Vec<_>>())
        }
        "exports" => serde_json::to_value(
            capsules
                .iter()
                .map(export_capsule_metadata)
                .collect::<Vec<_>>(),
        ),
        "fixtures" => serde_json::to_value(seeded_environment_capsule_fixtures()),
        _ => serde_json::to_value(serde_json::json!({
            "capsules": &capsules,
            "inspections": capsules.iter().map(inspect_environment).collect::<Vec<_>>(),
            "exports": capsules.iter().map(export_capsule_metadata).collect::<Vec<_>>(),
            "fixtures": seeded_environment_capsule_fixtures(),
        })),
    }
    .expect("environment capsules, inspections, and fixtures serialize");
    println!(
        "{}",
        serde_json::to_string_pretty(&value).expect("pretty JSON renders")
    );
}
