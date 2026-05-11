use super::*;
use aureline_content_safety::{detect_suspicious_content, TrustClass};

fn risky_text_input() -> RiskyTextInput {
    // U+202E RIGHT-TO-LEFT OVERRIDE plus zero-width joiner.
    let text = "let \u{202E}admin\u{200D}user = 1;";
    RiskyTextInput {
        preview_id: "preview:risky_text:src/lib.rs#config_loader".to_owned(),
        source_subject_ref: "buffer:file:src/lib.rs#identifier:config_loader".to_owned(),
        source_surface_family: "editor".to_owned(),
        trust_class: TrustClass::RawText,
        detection: detect_suspicious_content(text),
    }
}

fn oversized_input() -> OversizedArtifactInput {
    OversizedArtifactInput {
        preview_id: "preview:oversized:log:tail.session".to_owned(),
        source_subject_ref: "capture:terminal_log:tail.session".to_owned(),
        source_surface_family: "output_viewer".to_owned(),
        trust_class: TrustClass::SanitizedRich,
        total_byte_count: 12_500_000,
        visible_byte_count: 64_000,
        visible_line_count: 1_024,
        omitted_bytes_estimate: 12_436_000,
        omitted_line_count_estimate: 199_232,
    }
}

fn generated_input() -> GeneratedContentInput {
    GeneratedContentInput {
        preview_id: "preview:generated:ai_change_summary:001".to_owned(),
        source_subject_ref: "ai:change_summary:proposal:001".to_owned(),
        source_surface_family: "rich_preview".to_owned(),
        generator_id: "composer-seed/m1".to_owned(),
        citation_anchor_refs: vec![
            "anchor:src/router.rs#fn:dispatch:lines:42-58".to_owned(),
            "anchor:docs/router.md#section:routing".to_owned(),
        ],
        canonical_source_subject_ref: Some("buffer:file:src/router.rs#fn:dispatch".to_owned()),
    }
}

#[test]
fn protected_walk_risky_text_offers_raw_and_escaped_paired() {
    let record = build_risky_text_preview(risky_text_input());
    assert_eq!(record.content_class_token, "risky_text");
    assert_eq!(record.currently_visible_representation_token, "escaped");
    assert!(record.suspicious_finding_count >= 2);

    let raw = record
        .copy_export_options
        .iter()
        .find(|o| o.action_id == "copy_raw")
        .expect("copy_raw present");
    let escaped = record
        .copy_export_options
        .iter()
        .find(|o| o.action_id == "copy_escaped")
        .expect("copy_escaped present");
    assert!(raw.must_offer_also.iter().any(|s| s == "copy_escaped"));
    assert!(escaped.must_offer_also.iter().any(|s| s == "copy_raw"));
    assert!(record
        .copy_export_options
        .iter()
        .any(|o| o.action_id == "export_metadata_only"));

    assert!(record.validate().is_empty());
}

#[test]
fn risky_text_clean_input_does_not_require_findings() {
    let mut input = risky_text_input();
    input.detection = detect_suspicious_content("let id = 1;");
    let record = build_risky_text_preview(input);
    assert_eq!(record.suspicious_finding_count, 0);
    assert_eq!(record.currently_visible_representation_token, "raw");
    // Even on clean input the paired actions remain so the chrome cannot
    // strip them on a per-frame basis.
    assert!(record.validate().is_empty());
}

#[test]
fn failure_drill_risky_text_unlabeled_rendered_copy_is_rejected() {
    let mut record = build_risky_text_preview(risky_text_input());
    // The wedge replaces the escaped option with a mislabeled copy_rendered
    // that pretends to carry raw bytes — the named failure drill where
    // rendered output could silently masquerade as raw or escaped.
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
    assert!(violations.iter().any(|v| matches!(
        v,
        SafePreviewInvariantViolation::MissingPairedAction { missing_action_id, .. } if missing_action_id == "copy_escaped"
    )));
    assert!(violations
        .iter()
        .any(|v| matches!(v, SafePreviewInvariantViolation::UnlabeledRenderedCopy { .. })));
}

#[test]
fn risky_text_must_carry_representation_label_disclosure() {
    let mut record = build_risky_text_preview(risky_text_input());
    if let Some(option) = record
        .copy_export_options
        .iter_mut()
        .find(|o| o.action_id == "copy_raw")
    {
        option.required_disclosure_fields.retain(|f| f != "representation_label");
    }
    let violations = record.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, SafePreviewInvariantViolation::MissingRepresentationLabel { .. })));
}

#[test]
fn oversized_windowed_preview_names_scope_and_omission() {
    let record = build_oversized_artifact_preview(oversized_input());
    assert_eq!(record.content_class_token, "oversized_artifact");
    let copy = record
        .copy_export_options
        .iter()
        .find(|o| o.action_id == "copy_rendered")
        .expect("copy_rendered present");
    assert_eq!(copy.scope_class, "visible_rows_or_events");
    assert!(copy
        .transforms_applied
        .iter()
        .any(|t| t == "truncated_or_windowed"));
    assert_eq!(
        copy.omission_summary.omitted_bytes_estimate,
        Some(12_436_000)
    );
    assert!(record.validate().is_empty());
}

#[test]
fn failure_drill_oversized_scope_overclaim_is_rejected() {
    let mut record = build_oversized_artifact_preview(oversized_input());
    for option in &mut record.copy_export_options {
        if option.action_id == "copy_rendered" {
            // Pretend the rendered view covers the full materialized set
            // even though only a slice is visible.
            option.scope_class = "loaded_materialized_set".to_owned();
            option
                .transforms_applied
                .retain(|t| t != "truncated_or_windowed");
        }
        // Strip the omission estimate everywhere so the support-export
        // path also overclaims.
        option.omission_summary.omitted_bytes_estimate = None;
        option.omission_summary.omitted_line_count_estimate = None;
    }
    let violations = record.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, SafePreviewInvariantViolation::OversizedScopeOverclaim)));
    assert!(violations
        .iter()
        .any(|v| matches!(v, SafePreviewInvariantViolation::OversizedMissingWindowTransform)));
    assert!(violations
        .iter()
        .any(|v| matches!(v, SafePreviewInvariantViolation::OversizedMissingOmittedBytes)));
}

#[test]
fn oversized_full_body_does_not_require_window_transform() {
    let mut input = oversized_input();
    input.visible_byte_count = input.total_byte_count;
    input.omitted_bytes_estimate = 0;
    input.omitted_line_count_estimate = 0;
    let record = build_oversized_artifact_preview(input);
    assert!(record.validate().is_empty());
}

#[test]
fn generated_preview_pins_origin_and_currently_visible_representation() {
    let record = build_generated_content_preview(generated_input());
    assert_eq!(record.origin_class_token, "generated");
    assert_eq!(record.currently_visible_representation_token, "generated");
    let copy_rendered = record
        .copy_export_options
        .iter()
        .find(|o| o.action_id == "copy_rendered")
        .expect("copy_rendered present");
    assert!(copy_rendered
        .required_disclosure_fields
        .iter()
        .any(|f| f == "citation_anchors"));
    let copy_raw = record
        .copy_export_options
        .iter()
        .find(|o| o.action_id == "copy_raw")
        .expect("copy_raw for canonical source present");
    assert!(!copy_raw.citation_anchor_refs.is_empty());
    assert!(record.validate().is_empty());
}

#[test]
fn failure_drill_generated_copy_raw_without_citation_is_rejected() {
    let mut record = build_generated_content_preview(generated_input());
    if let Some(option) = record
        .copy_export_options
        .iter_mut()
        .find(|o| o.action_id == "copy_raw")
    {
        option.citation_anchor_refs.clear();
    }
    let violations = record.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, SafePreviewInvariantViolation::GeneratedCopyRawWithoutCitation { .. })));
}

#[test]
fn failure_drill_generated_origin_mislabel_is_rejected() {
    let mut record = build_generated_content_preview(generated_input());
    record.origin_class_token = "user_authored_or_imported".to_owned();
    record.currently_visible_representation_token = "raw".to_owned();
    let violations = record.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, SafePreviewInvariantViolation::GeneratedOriginMismatch { .. })));
    assert!(violations
        .iter()
        .any(|v| matches!(v, SafePreviewInvariantViolation::GeneratedVisibleMismatch { .. })));
}

#[test]
fn generated_without_canonical_source_omits_copy_raw_option() {
    let mut input = generated_input();
    input.canonical_source_subject_ref = None;
    input.citation_anchor_refs = Vec::new();
    let record = build_generated_content_preview(input);
    assert!(record
        .copy_export_options
        .iter()
        .all(|o| o.action_id != "copy_raw"));
    assert!(record.validate().is_empty());
}

#[test]
fn render_plaintext_quotes_every_lineage_field_in_stable_order() {
    let record = build_risky_text_preview(risky_text_input());
    let text = record.render_plaintext();
    assert!(text.contains("m1_prototype_safe_preview_and_copy_export"));
    assert!(text.contains("content_class=risky_text"));
    assert!(text.contains("trust_class=RawText"));
    assert!(text.contains("currently_visible=escaped"));
    assert!(text.contains("copy_export_options:"));
    assert!(text.contains("copy_raw"));
    assert!(text.contains("copy_escaped"));
    assert!(text.contains("export_metadata_only"));
    assert!(text.contains("must_offer_also="));
    assert!(text.contains("claim_limits:"));
    assert!(text.contains("bounded_prototype_only"));
}

#[test]
fn record_round_trips_through_serde_json() {
    let record = build_oversized_artifact_preview(oversized_input());
    let json = serde_json::to_string(&record).expect("serialize");
    let back: SafePreviewRecord = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(back.preview_id, record.preview_id);
    assert_eq!(back.copy_export_options.len(), record.copy_export_options.len());
    assert_eq!(back.validate(), record.validate());
}

#[test]
fn missing_copy_export_options_is_reported() {
    let mut record = build_risky_text_preview(risky_text_input());
    record.copy_export_options.clear();
    let violations = record.validate();
    assert!(violations.contains(&SafePreviewInvariantViolation::NoCopyExportOptions));
}
