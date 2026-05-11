//! Fixture-driven integration tests for the representation-labeled safe
//! preview and copy/export card.
//!
//! Each fixture under `fixtures/preview/representation_cases/` exercises one
//! of the three M1 lanes (risky text, oversized artifact, generated content)
//! or the named failure drill. The test loads the fixture, builds the
//! canonical [`aureline_preview::SafePreviewRecord`] from its typed input,
//! projects the shell snapshot, and asserts the fixture's expected vocabulary
//! verbatim.

use std::path::PathBuf;

use serde::Deserialize;

use aureline_content_safety::detect_suspicious_content;
use aureline_preview::{
    build_generated_content_preview, build_oversized_artifact_preview,
    build_risky_text_preview, GeneratedContentInput, OversizedArtifactInput, RiskyTextInput,
    SafePreviewInvariantViolation, SafePreviewRecord,
};
use aureline_shell::safe_preview_card::{
    SafePreviewCardSnapshot, SafePreviewRowStatus, SafePreviewSectionId,
};

fn fixtures_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("preview")
        .join("representation_cases")
}

fn load_case(file_name: &str) -> serde_json::Value {
    let path = fixtures_dir().join(file_name);
    let bytes = std::fs::read(&path)
        .unwrap_or_else(|err| panic!("failed to read fixture {}: {err}", path.display()));
    serde_json::from_slice(&bytes)
        .unwrap_or_else(|err| panic!("invalid JSON in fixture {}: {err}", path.display()))
}

#[derive(Debug, Deserialize)]
struct RiskyTextSubject {
    preview_id: String,
    source_subject_ref: String,
    source_surface_family: String,
    trust_class: String,
    risky_text_payload: String,
}

#[derive(Debug, Deserialize)]
struct OversizedSubject {
    preview_id: String,
    source_subject_ref: String,
    source_surface_family: String,
    trust_class: String,
    total_byte_count: u64,
    visible_byte_count: u64,
    visible_line_count: u64,
    omitted_bytes_estimate: u64,
    omitted_line_count_estimate: u64,
}

#[derive(Debug, Deserialize)]
struct GeneratedSubject {
    preview_id: String,
    source_subject_ref: String,
    source_surface_family: String,
    generator_id: String,
    citation_anchor_refs: Vec<String>,
    canonical_source_subject_ref: Option<String>,
}

fn parse_trust_class(token: &str) -> aureline_content_safety::TrustClass {
    match token {
        "RawText" => aureline_content_safety::TrustClass::RawText,
        "SanitizedRich" => aureline_content_safety::TrustClass::SanitizedRich,
        "TrustedLocalActive" => aureline_content_safety::TrustClass::TrustedLocalActive,
        "IsolatedRemoteActive" => aureline_content_safety::TrustClass::IsolatedRemoteActive,
        other => panic!("unknown trust class token: {other}"),
    }
}

fn build_risky_record_from_fixture(value: &serde_json::Value) -> SafePreviewRecord {
    let subject: RiskyTextSubject = serde_json::from_value(value["subject"].clone()).unwrap();
    build_risky_text_preview(RiskyTextInput {
        preview_id: subject.preview_id,
        source_subject_ref: subject.source_subject_ref,
        source_surface_family: subject.source_surface_family,
        trust_class: parse_trust_class(&subject.trust_class),
        detection: detect_suspicious_content(&subject.risky_text_payload),
    })
}

fn build_oversized_record_from_fixture(value: &serde_json::Value) -> SafePreviewRecord {
    let subject: OversizedSubject = serde_json::from_value(value["subject"].clone()).unwrap();
    build_oversized_artifact_preview(OversizedArtifactInput {
        preview_id: subject.preview_id,
        source_subject_ref: subject.source_subject_ref,
        source_surface_family: subject.source_surface_family,
        trust_class: parse_trust_class(&subject.trust_class),
        total_byte_count: subject.total_byte_count,
        visible_byte_count: subject.visible_byte_count,
        visible_line_count: subject.visible_line_count,
        omitted_bytes_estimate: subject.omitted_bytes_estimate,
        omitted_line_count_estimate: subject.omitted_line_count_estimate,
    })
}

fn build_generated_record_from_fixture(value: &serde_json::Value) -> SafePreviewRecord {
    let subject: GeneratedSubject = serde_json::from_value(value["subject"].clone()).unwrap();
    build_generated_content_preview(GeneratedContentInput {
        preview_id: subject.preview_id,
        source_subject_ref: subject.source_subject_ref,
        source_surface_family: subject.source_surface_family,
        generator_id: subject.generator_id,
        citation_anchor_refs: subject.citation_anchor_refs,
        canonical_source_subject_ref: subject.canonical_source_subject_ref,
    })
}

fn parse_section_id(token: &str) -> SafePreviewSectionId {
    match token {
        "prototype_label" => SafePreviewSectionId::PrototypeLabel,
        "header" => SafePreviewSectionId::Header,
        "currently_visible" => SafePreviewSectionId::CurrentlyVisible,
        "body_extent" => SafePreviewSectionId::BodyExtent,
        "copy_export_options" => SafePreviewSectionId::CopyExportOptions,
        "claim_limits" => SafePreviewSectionId::ClaimLimits,
        "invariants" => SafePreviewSectionId::Invariants,
        other => panic!("unknown section id token: {other}"),
    }
}

fn assert_common_record_fields(record: &SafePreviewRecord, fixture: &serde_json::Value) {
    let expected = &fixture["expected_record"];
    if let Some(token) = expected["prototype_label_token"].as_str() {
        assert_eq!(record.prototype_label_token, token);
    }
    if let Some(token) = expected["content_class_token"].as_str() {
        assert_eq!(record.content_class_token, token);
    }
    if let Some(token) = expected["trust_class_token"].as_str() {
        assert_eq!(record.trust_class_token, token);
    }
    if let Some(token) = expected["origin_class_token"].as_str() {
        assert_eq!(record.origin_class_token, token);
    }
    if let Some(token) = expected["currently_visible_representation_token"].as_str() {
        assert_eq!(record.currently_visible_representation_token, token);
    }
    if let Some(actions) = expected["expected_action_ids"].as_array() {
        let observed: Vec<String> = record
            .copy_export_options
            .iter()
            .map(|o| o.action_id.clone())
            .collect();
        for action in actions {
            let token = action.as_str().expect("action id is string");
            assert!(
                observed.iter().any(|a| a == token),
                "expected action id {token} not found in {observed:?}",
            );
        }
    }
    if let Some(violations) = expected["expected_invariant_violation_tokens"].as_array() {
        let actual = record.validate();
        let actual_tokens: Vec<&str> = actual.iter().map(|v| v.token()).collect();
        for token in violations {
            let token = token.as_str().expect("violation token is string");
            assert!(
                actual_tokens.contains(&token),
                "expected violation token {token} not present in {actual_tokens:?}",
            );
        }
    }
}

fn assert_common_snapshot_fields(snapshot: &SafePreviewCardSnapshot, fixture: &serde_json::Value) {
    let expected = &fixture["expected_snapshot"];
    if let Some(sections) = expected["expected_section_ids"].as_array() {
        let observed: Vec<SafePreviewSectionId> =
            snapshot.sections.iter().map(|s| s.section_id).collect();
        for section in sections {
            let token = section.as_str().expect("section id is string");
            assert!(
                observed.contains(&parse_section_id(token)),
                "expected section {token} not present in {observed:?}"
            );
        }
    }
    if let Some(expected_invariants) = expected["expected_has_invariant_violations"].as_bool() {
        assert_eq!(snapshot.has_invariant_violations, expected_invariants);
    }
    if let Some(count) = expected["expected_copy_export_option_count"].as_u64() {
        assert_eq!(snapshot.copy_export_option_count as u64, count);
    }
}

#[test]
fn protected_walk_risky_text_fixture_drives_full_card() {
    let fixture = load_case("01_risky_text_bidi_identifier.json");
    let record = build_risky_record_from_fixture(&fixture);

    let expected = &fixture["expected_record"];
    if let Some(min) = expected["expected_minimum_suspicious_findings"].as_u64() {
        assert!(record.suspicious_finding_count as u64 >= min);
    }
    if let Some(pairs) = expected["must_offer_also_pairs"].as_array() {
        for pair in pairs {
            let action_id = pair["action_id"].as_str().unwrap();
            let peer = pair["expected_peer"].as_str().unwrap();
            let option = record
                .copy_export_options
                .iter()
                .find(|o| o.action_id == action_id)
                .expect("paired option present");
            assert!(option.must_offer_also.iter().any(|p| p == peer));
        }
    }
    assert_common_record_fields(&record, &fixture);

    let snapshot = SafePreviewCardSnapshot::project(&record);
    assert_common_snapshot_fields(&snapshot, &fixture);
}

#[test]
fn protected_walk_oversized_fixture_carries_window_scope_and_omission() {
    let fixture = load_case("02_oversized_log_capture_windowed.json");
    let record = build_oversized_record_from_fixture(&fixture);

    let expected = &fixture["expected_record"];
    let copy = record
        .copy_export_options
        .iter()
        .find(|o| o.action_id == "copy_rendered")
        .expect("copy_rendered present");
    if let Some(scope) = expected["expected_windowed_option_scope_class"].as_str() {
        assert_eq!(copy.scope_class, scope);
    }
    if let Some(transforms) = expected["expected_windowed_option_transforms_include"].as_array() {
        for t in transforms {
            let token = t.as_str().unwrap();
            assert!(
                copy.transforms_applied.iter().any(|x| x == token),
                "expected transform {token} not present"
            );
        }
    }
    if let Some(reasons) = expected["expected_windowed_option_omission_reasons_include"].as_array()
    {
        for r in reasons {
            let token = r.as_str().unwrap();
            assert!(copy.omission_summary.reasons.iter().any(|x| x == token));
        }
    }
    if let Some(estimate) = expected["expected_windowed_option_omitted_bytes_estimate"].as_u64() {
        assert_eq!(
            copy.omission_summary.omitted_bytes_estimate,
            Some(estimate)
        );
    }
    assert_common_record_fields(&record, &fixture);

    let snapshot = SafePreviewCardSnapshot::project(&record);
    let body = snapshot
        .section(SafePreviewSectionId::BodyExtent)
        .expect("body_extent section");
    if let Some(expected_rows) =
        fixture["expected_snapshot"]["expected_body_extent_row_ids"].as_array()
    {
        for row_id in expected_rows {
            let token = row_id.as_str().unwrap();
            assert!(body.rows.iter().any(|r| r.row_id == token), "missing {token}");
        }
    }
    assert_common_snapshot_fields(&snapshot, &fixture);
}

#[test]
fn protected_walk_generated_fixture_requires_citation_anchors_for_copy_raw() {
    let fixture = load_case("03_generated_change_summary.json");
    let record = build_generated_record_from_fixture(&fixture);

    let copy_raw = record
        .copy_export_options
        .iter()
        .find(|o| o.action_id == "copy_raw")
        .expect("copy_raw canonical-source option present");
    assert!(!copy_raw.citation_anchor_refs.is_empty());

    let copy_rendered = record
        .copy_export_options
        .iter()
        .find(|o| o.action_id == "copy_rendered")
        .expect("copy_rendered present");
    if let Some(fields) =
        fixture["expected_record"]["expected_copy_rendered_required_disclosure_fields_include"]
            .as_array()
    {
        for field in fields {
            let token = field.as_str().unwrap();
            assert!(copy_rendered
                .required_disclosure_fields
                .iter()
                .any(|f| f == token));
        }
    }
    assert_common_record_fields(&record, &fixture);

    let snapshot = SafePreviewCardSnapshot::project(&record);
    assert_common_snapshot_fields(&snapshot, &fixture);
}

#[test]
fn failure_drill_unlabeled_rendered_copy_fixture_surfaces_typed_violations() {
    let fixture = load_case("04_failure_drill_unlabeled_rendered_copy.json");
    let mut record = build_risky_record_from_fixture(&fixture);

    // Inject the named failure: a buggy preview replaces the paired
    // copy_escaped option with a copy_rendered button that pretends to
    // carry raw bytes.
    if let Some(option) = record
        .copy_export_options
        .iter_mut()
        .find(|o| o.action_id == "copy_escaped")
    {
        option.action_id = "copy_rendered".to_owned();
        option.representation_class = "raw".to_owned();
        option.must_offer_also = Vec::new();
    }

    let violations = record.validate();
    assert!(
        !violations.is_empty(),
        "named failure drill must surface invariant violations",
    );
    let observed: Vec<&str> = violations.iter().map(|v| v.token()).collect();
    if let Some(expected_tokens) =
        fixture["expected_record"]["expected_invariant_violation_tokens"].as_array()
    {
        for token in expected_tokens {
            let token = token.as_str().unwrap();
            assert!(
                observed.contains(&token),
                "expected violation token {token} not present in {observed:?}",
            );
        }
    }
    assert!(violations.iter().any(|v| matches!(
        v,
        SafePreviewInvariantViolation::UnlabeledRenderedCopy { .. }
    )));

    let snapshot = SafePreviewCardSnapshot::project(&record);
    assert!(snapshot.has_invariant_violations);
    let invariants = snapshot
        .section(SafePreviewSectionId::Invariants)
        .expect("invariants section");
    let blocked_tokens: Vec<String> = invariants
        .rows
        .iter()
        .filter(|row| row.status == SafePreviewRowStatus::Blocked)
        .filter_map(|row| row.value_token.clone())
        .collect();
    if let Some(expected_tokens) =
        fixture["expected_snapshot"]["expected_blocked_violation_tokens_include"].as_array()
    {
        for token in expected_tokens {
            let token = token.as_str().unwrap();
            assert!(
                blocked_tokens.iter().any(|t| t == token),
                "expected blocked token {token} not present in {blocked_tokens:?}",
            );
        }
    }
}
