//! Replay gate for the workset / scope UX lineage record.
//!
//! Each fixture under
//! `fixtures/workspace/m4/workset_scope_ux_lineage/` carries the
//! posture input (a [`WorksetScopeUxInputs`] envelope plus the
//! inspection-hook set) and the expected projected lineage record.
//! This gate re-projects each input and asserts the result equals
//! the checked-in `expected`, so the projection cannot drift from
//! the canonical checked-in record without failing CI. It also
//! proves every fixture stays support-export safe and that the
//! corpus covers Stable plus narrowed-below-Stable postures.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    project_workset_scope_ux_lineage_with_hooks, workset_scope_ux_lineage_lines,
    WorksetScopeUxInputs, WorksetScopeUxInspectionHook, WorksetScopeUxLineageRecord,
    WORKSET_SCOPE_UX_LINEAGE_RECORD_KIND, WORKSET_SCOPE_UX_LINEAGE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageFixture {
    posture_id: String,
    inputs: WorksetScopeUxInputs,
    inspection_hooks: Vec<WorksetScopeUxInspectionHook>,
    expected: WorksetScopeUxLineageRecord,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/workset_scope_ux_lineage")
}

fn load_fixtures() -> Vec<(String, LineageFixture)> {
    let dir = fixtures_dir();
    let mut out = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("fixture directory must exist") {
        let path = entry.expect("fixture entry must read").path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let raw = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
        let fixture: LineageFixture = serde_json::from_str(&raw)
            .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()));
        out.push((path.display().to_string(), fixture));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    assert!(!out.is_empty(), "expected at least one lineage fixture");
    out
}

#[test]
fn projection_replays_each_fixture_exactly() {
    for (name, fixture) in load_fixtures() {
        let projected = project_workset_scope_ux_lineage_with_hooks(
            fixture.posture_id.clone(),
            &fixture.inputs,
            fixture.inspection_hooks.clone(),
        );
        assert_eq!(
            projected, fixture.expected,
            "projection drifted from checked-in record for fixture {name}"
        );

        let roundtrip: WorksetScopeUxLineageRecord =
            serde_json::from_str(&serde_json::to_string(&projected).expect("record serializes"))
                .expect("record round-trips");
        assert_eq!(roundtrip, projected, "record must round-trip for {name}");
    }
}

#[test]
fn every_fixture_is_support_export_safe_and_well_formed() {
    for (name, fixture) in load_fixtures() {
        let record = &fixture.expected;
        assert!(
            record.is_support_export_safe(),
            "fixture {name} must be support-export safe"
        );
        assert_eq!(record.record_kind, WORKSET_SCOPE_UX_LINEAGE_RECORD_KIND);
        assert_eq!(record.schema_ref, WORKSET_SCOPE_UX_LINEAGE_SCHEMA_REF);
        assert!(
            record.raw_payload_excluded,
            "fixture {name} excludes raw payload"
        );
        assert!(
            !record.workspace_ref.is_empty(),
            "fixture {name} must carry a workspace ref"
        );
        assert!(
            !record.corpus_ref.is_empty(),
            "fixture {name} must carry a corpus ref"
        );

        let lines = workset_scope_ux_lineage_lines(record);
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Workset/scope UX lineage")),
            "fixture {name} must render the header line"
        );
        assert!(
            lines.iter().any(|line| line == "Scopes:"),
            "fixture {name} must render scopes"
        );
        assert!(
            lines.iter().any(|line| line == "Surfaces:"),
            "fixture {name} must render surfaces"
        );
        assert!(
            lines.iter().any(|line| line.contains("Outside-vs-omitted")),
            "fixture {name} must render outside-vs-omitted disclosure"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Slice-ref propagation")),
            "fixture {name} must render slice-ref propagation"
        );
        assert!(
            lines.iter().any(|line| line == "Widen previews:"),
            "fixture {name} must render widen previews"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Widen-preservation truth")),
            "fixture {name} must render widen-preservation truth"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Policy-limited disclosure")),
            "fixture {name} must render policy-limited disclosure"
        );
        assert!(
            lines.iter().any(|line| line == "Inspection hooks:"),
            "fixture {name} must render inspection hooks"
        );
    }
}

#[test]
fn corpus_covers_stable_and_narrowed_postures() {
    let fixtures = load_fixtures();
    let any_stable = fixtures
        .iter()
        .any(|(_, fixture)| fixture.expected.stable_qualification.qualified);
    let any_narrowed = fixtures
        .iter()
        .any(|(_, fixture)| !fixture.expected.stable_qualification.qualified);
    assert!(any_stable, "corpus must include a Stable-qualified posture");
    assert!(
        any_narrowed,
        "corpus must include a narrowed-below-Stable posture"
    );
}

#[test]
fn corpus_proves_widen_preview_and_hook_paths_are_narrowed() {
    // The corpus must demonstrate that a missing widen-preview
    // disclosure narrows the record below Stable and that a missing
    // inspection hook narrows the record below Stable.
    let fixtures = load_fixtures();
    let saw_preview_narrow = fixtures.iter().any(|(_, fixture)| {
        fixture
            .expected
            .stable_qualification
            .narrow_reasons
            .iter()
            .any(|reason| reason.as_str() == "widen_preview_field_missing")
    });
    assert!(
        saw_preview_narrow,
        "corpus must include a fixture proving a missing widen-preview disclosure narrows the record"
    );

    let saw_hook_narrow = fixtures.iter().any(|(_, fixture)| {
        fixture
            .expected
            .stable_qualification
            .narrow_reasons
            .iter()
            .any(|reason| reason.as_str() == "inspection_hook_unavailable")
    });
    assert!(
        saw_hook_narrow,
        "corpus must include a fixture proving a missing inspection hook narrows the record"
    );
}

#[test]
fn corpus_proves_policy_limited_admin_redacted_view_stays_stable() {
    // Every Stable corpus must include a policy-limited view whose
    // admin-policy narrowing cause refuses to expose the hidden
    // member list — and remains Stable in that posture.
    let fixtures = load_fixtures();
    let saw_admin_redacted_stable = fixtures.iter().any(|(_, fixture)| {
        fixture.expected.stable_qualification.qualified
            && fixture
                .expected
                .scope_coverage
                .scope_rows
                .iter()
                .any(|row| {
                    matches!(
                        row.narrowing_cause,
                        Some(aureline_workspace::WorksetScopeUxNarrowingCause::AdminPolicy)
                    ) && !row.hidden_member_list_visible
                })
    });
    assert!(
        saw_admin_redacted_stable,
        "corpus must include a Stable fixture with an admin-policy policy_limited_view that redacts the hidden member list"
    );
}
