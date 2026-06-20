//! Dumps the declarative workspace templates, their why-this-template
//! inspections, metadata-first exports, and the fixture corpus.
//!
//! With no argument it prints `{ "templates": ..., "inspections": ...,
//! "exports": ..., "fixtures": [...] }` for human inspection. Pass
//! `templates`, `inspections`, `exports`, or `fixtures` to print only that
//! view. The `fixtures` form is the one written, per fixture, under
//! `fixtures/env/workspace-template/`.

use aureline_env::{
    export_template_metadata, inspect_template, seeded_workspace_template_fixtures,
    seeded_workspace_templates,
};

fn main() {
    let mode = std::env::args().nth(1).unwrap_or_default();
    let templates = seeded_workspace_templates();
    let value = match mode.as_str() {
        "templates" => serde_json::to_value(&templates),
        "inspections" => {
            serde_json::to_value(templates.iter().map(inspect_template).collect::<Vec<_>>())
        }
        "exports" => serde_json::to_value(
            templates
                .iter()
                .map(export_template_metadata)
                .collect::<Vec<_>>(),
        ),
        "fixtures" => serde_json::to_value(seeded_workspace_template_fixtures()),
        _ => serde_json::to_value(serde_json::json!({
            "templates": &templates,
            "inspections": templates.iter().map(inspect_template).collect::<Vec<_>>(),
            "exports": templates.iter().map(export_template_metadata).collect::<Vec<_>>(),
            "fixtures": seeded_workspace_template_fixtures(),
        })),
    }
    .expect("workspace templates, inspections, and fixtures serialize");
    println!(
        "{}",
        serde_json::to_string_pretty(&value).expect("pretty JSON renders")
    );
}
