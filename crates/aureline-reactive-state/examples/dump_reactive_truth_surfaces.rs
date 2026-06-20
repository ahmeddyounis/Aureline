//! Regenerates the checked-in reactive-truth-surfaces artifacts and
//! fixtures from the seeded packet so the on-disk evidence can never drift
//! from the canonical engine.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_reactive_state::reactive_truth_surfaces::{
    render_reactive_truth_surfaces_audit_plaintext, seeded_reactive_truth_surfaces_fixtures,
    seeded_reactive_truth_surfaces_packet, ReactiveTruthCueFixture,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

/// Pretty JSON with sorted keys, matching the house artifact style.
fn sorted_json<T: serde::Serialize>(value: &T) -> String {
    let value = serde_json::to_value(value).expect("value serializes");
    serde_json::to_string_pretty(&value).expect("pretty prints")
}

fn fixture_slug(fixture: &ReactiveTruthCueFixture) -> String {
    fixture
        .fixture_id
        .rsplit(':')
        .next()
        .expect("fixture id has a tail")
        .to_owned()
}

fn report_markdown() -> String {
    let packet = seeded_reactive_truth_surfaces_packet();
    let mut out = String::new();
    out.push_str("# Reactive-truth surfaces — evidence report\n\n");
    out.push_str(
        "Every derived M5 surface ships one canonical reactive-truth cue — source\n\
         authority, freshness, completeness, invalidation reason, backpressure, the\n\
         narrowed claim, and an action gate — instead of feature-local stale-state\n\
         prose. The cue layer is implemented in\n\
         [`crates/aureline-reactive-state/src/reactive_truth_surfaces/mod.rs`](../../crates/aureline-reactive-state/src/reactive_truth_surfaces/mod.rs)\n\
         and serialized to\n\
         [`artifacts/state/reactive_truth_surfaces.json`](./reactive_truth_surfaces.json).\n\n",
    );
    out.push_str(
        "It is derived from the canonical governance matrix in\n\
         [`artifacts/state/m5_reactive_governance.json`](./m5_reactive_governance.json),\n\
         so the gate, invalidation reason, and resubscribe cue can never fork the\n\
         narrowing engine.\n\n",
    );
    out.push_str("## Invariants\n\n");
    for invariant in &packet.invariants {
        out.push_str(&format!("- {invariant}\n"));
    }
    out.push_str("\n## Per-surface action gating\n\n");
    out.push_str(
        "| surface | authority | view class | healthy claim | healthy gate | gated rules |\n",
    );
    out.push_str("| --- | --- | --- | --- | --- | --- |\n");
    for audit in &packet.surfaces {
        out.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
            audit.surface_class.as_str(),
            audit.authority_class.as_str(),
            audit.view_class.as_str(),
            audit.healthy_claim.as_str(),
            audit.healthy_action_gate.as_str(),
            audit.gated_narrowing_rules.len(),
        ));
    }
    out.push_str("\n## Deterministic audit projection\n\n```\n");
    out.push_str(&render_reactive_truth_surfaces_audit_plaintext(&packet));
    out.push_str("\n```\n");
    out
}

fn manifest_yaml(fixtures: &[ReactiveTruthCueFixture]) -> String {
    let mut out = String::new();
    out.push_str("schema_ref: schemas/state/reactive_truth_surfaces.schema.json\n");
    out.push_str("doc_ref: docs/state/reactive_truth_surfaces.md\n");
    out.push_str("packet_ref: artifacts/state/reactive_truth_surfaces.json\n");
    out.push_str("report_ref: artifacts/state/reactive_truth_surfaces.md\n");
    out.push_str("fixtures:\n");
    for fixture in fixtures {
        out.push_str(&format!(
            "  - fixtures/state/reactive_truth_surfaces/{}.json\n",
            fixture_slug(fixture)
        ));
    }
    out
}

fn main() {
    let root = repo_root();
    let packet = seeded_reactive_truth_surfaces_packet();
    let fixtures = seeded_reactive_truth_surfaces_fixtures();

    let packet_path = root.join("artifacts/state/reactive_truth_surfaces.json");
    fs::write(&packet_path, sorted_json(&packet) + "\n").expect("write packet");

    let report_path = root.join("artifacts/state/reactive_truth_surfaces.md");
    fs::write(&report_path, report_markdown()).expect("write report");

    let fixture_dir = root.join("fixtures/state/reactive_truth_surfaces");
    fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    for fixture in &fixtures {
        let path = fixture_dir.join(format!("{}.json", fixture_slug(fixture)));
        fs::write(&path, sorted_json(fixture) + "\n").expect("write fixture");
    }
    fs::write(fixture_dir.join("manifest.yaml"), manifest_yaml(&fixtures)).expect("write manifest");

    println!(
        "regenerated reactive-truth surfaces: {} audit rows, {} fixtures",
        packet.surfaces.len(),
        fixtures.len()
    );
}
