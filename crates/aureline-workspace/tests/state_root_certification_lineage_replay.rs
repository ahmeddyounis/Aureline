//! Replay gate for the state-root certification lineage record.
//!
//! Each fixture under
//! `fixtures/workspace/m4/state_root_certification_lineage/` carries
//! the posture input (a [`StateRootCertificationInputs`] envelope plus
//! the inspection-hook set) and the expected projected lineage
//! record. This gate re-projects each input and asserts the result
//! equals the checked-in `expected`, so the projection cannot drift
//! from the canonical checked-in record without failing CI. It also
//! proves every fixture stays support-export safe and that the
//! corpus covers Stable plus narrowed-below-Stable postures.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    project_state_root_certification_lineage_with_hooks,
    state_root_certification_lineage_lines, StateRootCertificationInputs,
    StateRootCertificationLineageRecord, StateRootInspectionHook,
    STATE_ROOT_CERTIFICATION_LINEAGE_RECORD_KIND,
    STATE_ROOT_CERTIFICATION_LINEAGE_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct LineageFixture {
    posture_id: String,
    inputs: StateRootCertificationInputs,
    inspection_hooks: Vec<StateRootInspectionHook>,
    expected: StateRootCertificationLineageRecord,
}

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/state_root_certification_lineage")
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
        let projected = project_state_root_certification_lineage_with_hooks(
            fixture.posture_id.clone(),
            &fixture.inputs,
            fixture.inspection_hooks.clone(),
        );
        assert_eq!(
            projected, fixture.expected,
            "projection drifted from checked-in record for fixture {name}"
        );

        let roundtrip: StateRootCertificationLineageRecord = serde_json::from_str(
            &serde_json::to_string(&projected).expect("record serializes"),
        )
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
        assert_eq!(record.record_kind, STATE_ROOT_CERTIFICATION_LINEAGE_RECORD_KIND);
        assert_eq!(record.schema_ref, STATE_ROOT_CERTIFICATION_LINEAGE_SCHEMA_REF);
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

        let lines = state_root_certification_lineage_lines(record);
        assert!(
            lines
                .iter()
                .any(|line| line.contains("State-root certification lineage")),
            "fixture {name} must render the header line"
        );
        assert!(
            lines.iter().any(|line| line == "Resource audits:"),
            "fixture {name} must render resource audits"
        );
        assert!(
            lines.iter().any(|line| line == "Audit surfaces:"),
            "fixture {name} must render audit surfaces"
        );
        assert!(
            lines.iter().any(|line| line.contains("Audit honesty")),
            "fixture {name} must render audit honesty"
        );
        assert!(
            lines.iter().any(|line| line.contains("Preservation")),
            "fixture {name} must render preservation"
        );
        assert!(
            lines.iter().any(|line| line.contains("No-silent-rerun")),
            "fixture {name} must render no-silent-rerun"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Audit-transaction pinning")),
            "fixture {name} must render audit-transaction pinning"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("Audit-surface reachability")),
            "fixture {name} must render audit-surface reachability"
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
fn corpus_proves_silent_dirty_and_silent_rerun_are_narrowed() {
    let fixtures = load_fixtures();

    let saw_silent_dirty = fixtures.iter().any(|(_, fixture)| {
        fixture
            .expected
            .stable_qualification
            .narrow_reasons
            .iter()
            .any(|reason| reason.as_str() == "audit_disclosure_missing")
    });
    assert!(
        saw_silent_dirty,
        "corpus must include a fixture proving a dirty audit without disclosure narrows"
    );

    let saw_silent_rerun = fixtures.iter().any(|(_, fixture)| {
        fixture
            .expected
            .stable_qualification
            .narrow_reasons
            .iter()
            .any(|reason| reason.as_str() == "rerun_silent_forbidden")
    });
    assert!(
        saw_silent_rerun,
        "corpus must include a fixture proving silent_rerun_permitted narrows the record"
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
fn corpus_proves_dirty_with_disclosure_stays_stable() {
    use aureline_workspace::{AuditFindingClass, AuditRedactionClass};

    let fixtures = load_fixtures();
    let saw_dirty_stable = fixtures.iter().any(|(_, fixture)| {
        fixture.expected.stable_qualification.qualified
            && fixture
                .expected
                .resource_class_coverage
                .resource_audit_rows
                .iter()
                .any(|row| {
                    matches!(row.audit_finding, AuditFindingClass::AuditDirtyWithDisclosure)
                        && row
                            .audit_disclosure_ref
                            .as_ref()
                            .is_some_and(|value| !value.is_empty())
                        && matches!(row.redaction_class, AuditRedactionClass::RedactedWithDisclosure)
                        && row
                            .redaction_disclosure_ref
                            .as_ref()
                            .is_some_and(|value| !value.is_empty())
                        && !row.cleanup_surface_refs.is_empty()
                        && !row.inspection_hook_refs.is_empty()
                })
    });
    assert!(
        saw_dirty_stable,
        "corpus must include a Stable fixture with a dirty-with-disclosure audit carrying explicit disclosure refs and cleanup preconditions"
    );
}
