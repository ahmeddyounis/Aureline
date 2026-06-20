//! Conformance dump for the efficiency-state surfaces.
//!
//! Emits, for every representative posture, the canonical efficiency-state
//! snapshot together with the diagnostics and support-export projections that
//! derive from it. The output backs the checked-in fixtures under
//! `fixtures/efficiency/states/` so status, diagnostics, and support surfaces
//! provably share one object.

use aureline_shell::efficiency::surfaces::{
    seeded_efficiency_state_snapshots, EfficiencyDiagnosticsProjection,
    EfficiencyStateSupportExport,
};

fn main() {
    let cases = seeded_efficiency_state_snapshots()
        .into_iter()
        .map(|snapshot| {
            let diagnostics = EfficiencyDiagnosticsProjection::from_snapshot(&snapshot);
            let support_export = EfficiencyStateSupportExport::from_snapshot(&snapshot);
            serde_json::json!({
                "workspace_id": snapshot.workspace_id,
                "snapshot": snapshot,
                "diagnostics": diagnostics,
                "support_export": support_export,
            })
        })
        .collect::<Vec<_>>();
    println!(
        "{}",
        serde_json::to_string_pretty(&cases).expect("efficiency-state surfaces serialize")
    );
}
