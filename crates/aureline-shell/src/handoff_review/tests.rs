use super::*;
use std::collections::BTreeSet;

fn build_context_export(class: BuildContextExportClass, ref_suffix: &str) -> BuildContextExport {
    BuildContextExport {
        export_class: class,
        export_block_ref: format!("build_context_export:{ref_suffix}.v1"),
        export_block_schema_version: 1,
        redacted_for_audience: class,
        raw_screenshots_excluded: true,
        raw_secrets_excluded: true,
        export_summary: "Build identity refs only; no raw screenshots or secrets.".to_owned(),
    }
}

fn public_issue_target() -> HandoffTargetReview {
    HandoffTargetReview {
        handoff_target_review_schema_version: HANDOFF_TARGET_REVIEW_SCHEMA_VERSION,
        record_kind: HANDOFF_TARGET_REVIEW_RECORD_KIND.to_owned(),
        target_id: "handoff_target:public_issue".to_owned(),
        route_class: HandoffRouteClass::PublicIssue,
        visibility_class: TargetVisibilityClass::OfficialPublic,
        destination_identity_ref: "about_destination:issue.tracker.public".to_owned(),
        destination_label: "Public issue tracker".to_owned(),
        network_browser_requirement_class: NetworkBrowserRequirement::SystemBrowserPublicBrowse,
        data_exit_boundary_class: DataExitBoundary::MetadataSafeObjectRefs,
        safe_fallback_refs: vec!["about_destination:local.fallback.docs_pack".to_owned()],
        build_context_exports: vec![build_context_export(
            BuildContextExportClass::PublicIssueTemplateBlock,
            "public_issue",
        )],
        headline_label: "File a public issue".to_owned(),
        target_summary: "Opens the public issue tracker in the system browser.".to_owned(),
        contract_doc_ref: HANDOFF_AND_REPRO_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

fn public_repro_packet() -> ReproPacketPreview {
    ReproPacketPreview {
        repro_packet_preview_schema_version: REPRO_PACKET_PREVIEW_SCHEMA_VERSION,
        record_kind: REPRO_PACKET_PREVIEW_RECORD_KIND.to_owned(),
        packet_id: "repro_packet_preview:public_issue".to_owned(),
        redaction_posture_class: RedactionPostureClass::FullyRedactedPublicSafe,
        selected_diagnostics: vec![
            DiagnosticSelection {
                kind_class: DiagnosticKindClass::BuildIdentity,
                included: true,
                summary: "Build identity refs.".to_owned(),
            },
            DiagnosticSelection {
                kind_class: DiagnosticKindClass::ReproStepsText,
                included: true,
                summary: "Reproduction steps the user wrote.".to_owned(),
            },
        ],
        attachments: vec![ReproAttachment {
            kind_class: AttachmentKindClass::BuildContextExportBlock,
            attachment_ref: "build_context_export:public_issue.v1".to_owned(),
            redaction_applied: true,
            summary: "Redacted build-context export block.".to_owned(),
        }],
        anchor_identity: ExactAnchorIdentity {
            anchor_ref: "anchor:editor.line.412".to_owned(),
            object_ref: "object:source.file.lib_rs".to_owned(),
            anchor_label: "lib.rs line 412".to_owned(),
        },
        preview_confirmed_before_share: true,
        raw_secrets_excluded: true,
        raw_screenshots_excluded: true,
        headline_label: "Reproduction packet".to_owned(),
        packet_summary: "Build identity, repro steps, and one redacted export block.".to_owned(),
        contract_doc_ref: HANDOFF_AND_REPRO_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

fn opened_continuity() -> DraftContinuity {
    DraftContinuity {
        handoff_outcome_class: HandoffOutcomeClass::OpenedInSystemBrowser,
        intent_preserved: true,
        silent_loss: false,
        preserved_draft_text_ref: Some("draft_text:public_issue.body".to_owned()),
        preserved_attachment_refs: vec!["build_context_export:public_issue.v1".to_owned()],
        preserved_visibility_class: TargetVisibilityClass::OfficialPublic,
        preserved_redaction_posture_class: RedactionPostureClass::FullyRedactedPublicSafe,
        available_actions: vec![
            PreservationActionClass::ExportPacket,
            PreservationActionClass::SaveDraftLocal,
        ],
        selected_fallback_ref: None,
        continuity_summary: "Handoff opened in the system browser after preview confirmation."
            .to_owned(),
    }
}

fn baseline_sheet() -> HandoffReviewSheet {
    HandoffReviewSheet {
        handoff_review_sheet_schema_version: HANDOFF_REVIEW_SHEET_SCHEMA_VERSION,
        record_kind: HANDOFF_REVIEW_SHEET_RECORD_KIND.to_owned(),
        sheet_id: "handoff_review_sheet:public_issue".to_owned(),
        sheet_summary: "Public issue handoff review sheet.".to_owned(),
        target_review: public_issue_target(),
        repro_packet_preview: public_repro_packet(),
        draft_continuity: opened_continuity(),
        contract_doc_ref: HANDOFF_AND_REPRO_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

#[test]
fn baseline_sheet_validates() {
    baseline_sheet()
        .validate()
        .expect("baseline sheet validates");
}

#[test]
fn security_route_cannot_target_public() {
    let mut sheet = baseline_sheet();
    sheet.target_review.route_class = HandoffRouteClass::SecurityDisclosure;
    // visibility stays official_public — a coercion the contract must reject.
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::RouteVisibilityMismatch { .. }
    ));
}

#[test]
fn security_disclosure_requires_security_channel_and_payloads() {
    let mut sheet = baseline_sheet();
    sheet.target_review.route_class = HandoffRouteClass::SecurityDisclosure;
    sheet.target_review.visibility_class = TargetVisibilityClass::SecurityDisclosure;
    sheet.target_review.network_browser_requirement_class =
        NetworkBrowserRequirement::EncryptedSecurityChannel;
    sheet.target_review.data_exit_boundary_class = DataExitBoundary::SecurityPayloadsOnly;
    sheet.repro_packet_preview.redaction_posture_class = RedactionPostureClass::SecurityChannelOnly;
    sheet.draft_continuity.preserved_visibility_class = TargetVisibilityClass::SecurityDisclosure;
    sheet.draft_continuity.preserved_redaction_posture_class =
        RedactionPostureClass::SecurityChannelOnly;
    sheet
        .validate()
        .expect("well-formed security disclosure validates");

    // A security disclosure that tries to ride the public browser is rejected.
    sheet.target_review.network_browser_requirement_class =
        NetworkBrowserRequirement::SystemBrowserPublicBrowse;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::VisibilityNetworkMismatch { .. }
    ));
}

#[test]
fn public_target_rejects_security_scoped_redaction() {
    let mut sheet = baseline_sheet();
    sheet.repro_packet_preview.redaction_posture_class = RedactionPostureClass::SecurityChannelOnly;
    sheet.draft_continuity.preserved_redaction_posture_class =
        RedactionPostureClass::SecurityChannelOnly;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::RedactionPostureUnsafeForVisibility { .. }
    ));
}

#[test]
fn opening_browser_without_preview_confirmation_is_rejected() {
    let mut sheet = baseline_sheet();
    sheet.repro_packet_preview.preview_confirmed_before_share = false;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::SharedWithoutPreviewConfirmation { .. }
    ));
}

#[test]
fn blocked_handoff_must_preserve_draft_and_offer_export_and_save() {
    let mut sheet = baseline_sheet();
    sheet.draft_continuity.handoff_outcome_class = HandoffOutcomeClass::BrowserBlocked;
    // Preserved draft + export/save still present from the baseline: this is the
    // success path for a blocked handoff.
    sheet
        .validate()
        .expect("blocked handoff with preserved draft validates");

    // Dropping the export action turns it into silent loss territory.
    sheet.draft_continuity.available_actions = vec![PreservationActionClass::Discard];
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::BlockedHandoffMissingPreservationActions { .. }
    ));
}

#[test]
fn blocked_handoff_dropping_intent_is_rejected() {
    let mut sheet = baseline_sheet();
    sheet.draft_continuity.handoff_outcome_class = HandoffOutcomeClass::Offline;
    sheet.draft_continuity.intent_preserved = false;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::BlockedHandoffDroppedIntent { .. }
    ));
}

#[test]
fn blocked_handoff_without_draft_text_is_rejected() {
    let mut sheet = baseline_sheet();
    sheet.draft_continuity.handoff_outcome_class = HandoffOutcomeClass::PolicyDenied;
    sheet.draft_continuity.preserved_draft_text_ref = None;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::BlockedHandoffMissingDraftText { .. }
    ));
}

#[test]
fn silent_loss_is_never_allowed() {
    let mut sheet = baseline_sheet();
    sheet.draft_continuity.silent_loss = true;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::SilentLossNotAllowed { .. }
    ));
}

#[test]
fn preserved_target_class_must_match_chosen_target() {
    let mut sheet = baseline_sheet();
    sheet.draft_continuity.preserved_visibility_class = TargetVisibilityClass::Community;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::PreservedVisibilityMismatch { .. }
    ));
}

#[test]
fn preserved_redaction_must_match_packet() {
    let mut sheet = baseline_sheet();
    sheet.draft_continuity.preserved_redaction_posture_class =
        RedactionPostureClass::MetadataRefsOnly;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::PreservedRedactionMismatch { .. }
    ));
}

#[test]
fn target_must_offer_safe_fallback() {
    let mut sheet = baseline_sheet();
    sheet.target_review.safe_fallback_refs.clear();
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::MissingSafeFallback { .. }
    ));
}

#[test]
fn selected_fallback_must_be_offered() {
    let mut sheet = baseline_sheet();
    sheet.draft_continuity.handoff_outcome_class = HandoffOutcomeClass::Offline;
    sheet.draft_continuity.selected_fallback_ref =
        Some("about_destination:not.on.target".to_owned());
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::SelectedFallbackNotOffered { .. }
    ));
}

#[test]
fn handoff_lane_requires_build_context_export() {
    let mut sheet = baseline_sheet();
    sheet.target_review.build_context_exports.clear();
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::MissingBuildContextExport { .. }
    ));
}

#[test]
fn repro_packet_must_select_a_diagnostic() {
    let mut sheet = baseline_sheet();
    for diagnostic in &mut sheet.repro_packet_preview.selected_diagnostics {
        diagnostic.included = false;
    }
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::NoDiagnosticsSelected { .. }
    ));
}

#[test]
fn raw_payload_must_be_excluded() {
    let mut sheet = baseline_sheet();
    sheet.repro_packet_preview.raw_secrets_excluded = false;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::RawPayloadNotExcluded { .. }
    ));
}

#[test]
fn unredacted_attachment_is_rejected() {
    let mut sheet = baseline_sheet();
    sheet.repro_packet_preview.attachments[0].redaction_applied = false;
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::AttachmentNotRedacted { .. }
    ));
}

#[test]
fn raw_url_in_destination_ref_is_rejected() {
    let mut sheet = baseline_sheet();
    sheet.target_review.destination_identity_ref = "https://example.com/issues".to_owned();
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::RawRefLeak { .. }
    ));
}

#[test]
fn raw_email_in_anchor_ref_is_rejected() {
    let mut sheet = baseline_sheet();
    sheet.repro_packet_preview.anchor_identity.object_ref = "security@example.com".to_owned();
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::RawRefLeak { .. }
    ));
}

#[test]
fn render_plaintext_is_stable_and_mentions_key_axes() {
    let sheet = baseline_sheet();
    sheet.validate().expect("sheet validates");
    let block = sheet.render_plaintext();
    assert!(block.contains("handoff_review_sheet:public_issue"));
    assert!(block.contains("visibility=official_public"));
    assert!(block.contains("repro_packet_preview:public_issue"));
    assert!(block.contains("preserved visibility=official_public"));
    assert!(block.contains("actions: export_packet, save_draft_local"));
}

#[test]
fn validate_sheets_rejects_duplicate_ids() {
    let sheet = baseline_sheet();
    let err = validate_sheets(&[sheet.clone(), sheet]).unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::MalformedSheetId { .. }
    ));
}

#[test]
fn all_visibility_labels_are_distinct() {
    let classes = [
        TargetVisibilityClass::OfficialPublic,
        TargetVisibilityClass::OfficialPrivate,
        TargetVisibilityClass::SecurityDisclosure,
        TargetVisibilityClass::Community,
        TargetVisibilityClass::ThirdPartyVendor,
    ];
    let labels: BTreeSet<&str> = classes.iter().map(|c| c.label()).collect();
    assert_eq!(
        labels.len(),
        classes.len(),
        "visibility labels must be distinct"
    );
}
