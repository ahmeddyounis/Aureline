//! Dumps the derived runtime instances, their materialization parity, and
//! the fixture corpus.
//!
//! With no argument it prints `{ "instances": ..., "materializations": ...,
//! "exports": ..., "fixtures": [...] }` for human inspection. Pass
//! `instances`, `materializations`, `exports`, or `fixtures` to print only
//! that view. The `fixtures` form is the one written, per fixture, under
//! `fixtures/env/runtime-materialization/`.

use aureline_env::{
    export_runtime_materialization, seeded_runtime_instances,
    seeded_runtime_materialization_fixtures, seeded_runtime_materializations,
};

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let value = match mode.as_str() {
        "instances" => serde_json::to_value(seeded_runtime_instances()),
        "materializations" => serde_json::to_value(seeded_runtime_materializations()),
        "exports" => serde_json::to_value(
            seeded_runtime_materializations()
                .iter()
                .map(export_runtime_materialization)
                .collect::<Vec<_>>(),
        ),
        "fixtures" => serde_json::to_value(seeded_runtime_materialization_fixtures()),
        _ => serde_json::to_value(serde_json::json!({
            "instances": seeded_runtime_instances(),
            "materializations": seeded_runtime_materializations(),
            "exports": seeded_runtime_materializations()
                .iter()
                .map(export_runtime_materialization)
                .collect::<Vec<_>>(),
            "fixtures": seeded_runtime_materialization_fixtures(),
        })),
    }
    .expect("runtime instances, materializations, and fixtures serialize");
    println!(
        "{}",
        serde_json::to_string_pretty(&value).expect("pretty JSON renders")
    );
}
