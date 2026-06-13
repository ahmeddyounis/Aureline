//! Fixture-driven tests for the bounded notebook alpha lane.

use std::collections::BTreeSet;
use std::path::Path;

use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize)]
struct NotebookAlphaFixture {
    lane: NotebookAlphaLaneRecord,
    expect: NotebookAlphaFixtureExpect,
}

#[derive(Debug, Deserialize)]
struct NotebookAlphaFixtureExpect {
    valid: bool,
    required_violation_tokens: Vec<String>,
    required_export_scopes: Vec<String>,
    required_plaintext_snippets: Vec<String>,
}

fn fixture(name: &str) -> NotebookAlphaFixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/notebook/notebook_trust_diff_alpha")
        .join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_yaml::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {} must parse: {err}", path.display()))
}

#[test]
fn protected_lane_publishes_required_objects_and_scope_truth() {
    let fixture = fixture("protected_trust_diff_repair_export.yaml");
    let lane = fixture.lane.normalized();
    let violations = lane.validate();
    let violation_tokens: Vec<&str> = violations
        .iter()
        .map(NotebookAlphaViolation::token)
        .collect();

    assert!(
        fixture.expect.valid,
        "fixture expectation should describe a valid protected lane"
    );
    assert!(
        violations.is_empty(),
        "unexpected violations: {violation_tokens:?} {violations:?}"
    );
    assert!(lane.has_required_object_graph());
    assert!(lane.preserves_unknown_metadata_and_attachments());
    assert!(lane.distinguishes_export_scopes());
    assert!(lane.outputs_do_not_masquerade_as_live());
    assert!(lane.has_diff_or_repair_affordance());

    let scopes: BTreeSet<_> = lane
        .export_scopes
        .iter()
        .map(|scope| scope.scope_class.as_str())
        .collect();
    for required_scope in &fixture.expect.required_export_scopes {
        assert!(
            scopes.contains(required_scope.as_str()),
            "missing export scope {required_scope}"
        );
    }

    let support_export = lane.support_export();
    assert_eq!(
        support_export.record_kind,
        NOTEBOOK_ALPHA_SUPPORT_EXPORT_RECORD_KIND
    );
    assert!(support_export.violation_tokens.is_empty());
    assert!(support_export
        .object_refs
        .iter()
        .any(|object_ref| object_ref.starts_with("document:")));
    assert!(support_export
        .object_refs
        .iter()
        .any(|object_ref| object_ref.starts_with("cell:")));
    assert!(support_export
        .object_refs
        .iter()
        .any(|object_ref| object_ref.starts_with("kernel_session:")));
    assert!(support_export
        .object_refs
        .iter()
        .any(|object_ref| object_ref.starts_with("output_record:")));
    assert!(support_export
        .object_refs
        .iter()
        .any(|object_ref| object_ref.starts_with("review_session:")));
    assert!(support_export
        .object_refs
        .iter()
        .any(|object_ref| object_ref.starts_with("paired_export:")));
    assert!(support_export
        .identity_tokens
        .iter()
        .any(|token| token == "family:notebook_document"));
    assert!(support_export
        .identity_tokens
        .iter()
        .any(|token| token == "root:local"));
    assert!(support_export
        .identity_tokens
        .iter()
        .any(|token| token == "write_posture:editable_canonical_source"));

    for snippet in &fixture.expect.required_plaintext_snippets {
        assert!(
            support_export.plaintext_summary.contains(snippet),
            "support export plaintext should contain {snippet:?}\n{}",
            support_export.plaintext_summary
        );
    }
}

#[test]
fn failure_drill_stale_imported_output_cannot_masquerade_as_live() {
    let fixture = fixture("failure_stale_imported_output_claimed_live.yaml");
    let lane = fixture.lane.normalized();
    let violations = lane.validate();
    let observed: BTreeSet<_> = violations
        .iter()
        .map(|violation| violation.token())
        .collect();

    assert!(!fixture.expect.valid);
    assert!(!violations.is_empty());
    for required in &fixture.expect.required_violation_tokens {
        assert!(
            observed.contains(required.as_str()),
            "expected violation {required}, observed {observed:?}"
        );
    }
    assert!(!lane.outputs_do_not_masquerade_as_live());

    let export = lane.support_export();
    for required in &fixture.expect.required_violation_tokens {
        assert!(
            export
                .violation_tokens
                .iter()
                .any(|token| token == required),
            "support export should carry violation {required}"
        );
    }
}
