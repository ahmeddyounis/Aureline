//! Fixture replay for the public handoff-review and reproduction-packet preview
//! corpus published under `fixtures/public/m3/handoff_and_repro_preview/`.
//!
//! Positive cases MUST validate end-to-end; negative cases MUST fail validation
//! with a typed [`HandoffReviewValidationError`].

use std::fs;
use std::path::{Path, PathBuf};

use aureline_shell::handoff_review::{
    HandoffOutcomeClass, HandoffReviewSheet, HandoffReviewValidationError, HandoffRouteClass,
    RedactionPostureClass, TargetVisibilityClass,
};

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/public/m3/handoff_and_repro_preview")
}

fn load_sheet(rel: &str) -> HandoffReviewSheet {
    let path = fixture_root().join(rel);
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("failed to read directory {}: {err}", dir.display()))
    {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            out.push(path);
        }
    }
    out.sort();
    out
}

#[test]
fn all_positive_fixtures_validate() {
    let dir = fixture_root().join("positive");
    let files = json_files(&dir);
    assert!(
        !files.is_empty(),
        "expected positive fixtures under {}",
        dir.display()
    );
    for path in files {
        let body = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
        let sheet: HandoffReviewSheet = serde_json::from_str(&body)
            .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()));
        sheet.validate().unwrap_or_else(|err| {
            panic!(
                "positive fixture {} should validate but failed: {err}",
                path.display()
            )
        });
        let text = sheet.render_plaintext();
        assert!(
            text.contains(&sheet.sheet_id),
            "plaintext export must mention sheet id ({})",
            path.display()
        );
    }
}

#[test]
fn all_negative_fixtures_fail_validation() {
    let dir = fixture_root().join("negative");
    let files = json_files(&dir);
    assert!(
        !files.is_empty(),
        "expected negative fixtures under {}",
        dir.display()
    );
    for path in files {
        let body = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
        let sheet: HandoffReviewSheet = serde_json::from_str(&body)
            .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()));
        assert!(
            sheet.validate().is_err(),
            "negative fixture {} should fail validation but passed",
            path.display()
        );
    }
}

#[test]
fn public_issue_positive_shares_after_preview() {
    let sheet = load_sheet("positive/public_issue_handoff.json");
    sheet.validate().expect("public issue positive validates");
    assert_eq!(
        sheet.target_review.route_class,
        HandoffRouteClass::PublicIssue
    );
    assert_eq!(
        sheet.target_review.visibility_class,
        TargetVisibilityClass::OfficialPublic
    );
    assert!(
        sheet.repro_packet_preview.preview_confirmed_before_share,
        "a shared public issue must confirm the preview first"
    );
    assert_eq!(
        sheet.draft_continuity.handoff_outcome_class,
        HandoffOutcomeClass::OpenedInSystemBrowser
    );
}

#[test]
fn security_disclosure_positive_stays_private() {
    let sheet = load_sheet("positive/security_disclosure_private_channel.json");
    sheet.validate().expect("security disclosure positive validates");
    assert_eq!(
        sheet.target_review.visibility_class,
        TargetVisibilityClass::SecurityDisclosure
    );
    assert!(
        !sheet.target_review.visibility_class.is_public(),
        "a security disclosure must never resolve to a public target"
    );
    assert_eq!(
        sheet.repro_packet_preview.redaction_posture_class,
        RedactionPostureClass::SecurityChannelOnly
    );
}

#[test]
fn offline_positive_preserves_draft_and_offers_export_and_save() {
    let sheet = load_sheet("positive/offline_blocked_preserves_draft.json");
    sheet.validate().expect("offline positive validates");
    let d = &sheet.draft_continuity;
    assert!(d.handoff_outcome_class.is_blocked());
    assert!(d.intent_preserved, "blocked handoff must preserve intent");
    assert!(!d.silent_loss, "blocked handoff must not silently lose work");
    assert!(
        d.preserved_draft_text_ref.is_some(),
        "blocked handoff must preserve the draft text"
    );
    assert_eq!(
        d.preserved_visibility_class, sheet.target_review.visibility_class,
        "preserved target class must match the chosen target"
    );
    assert_eq!(
        d.preserved_redaction_posture_class, sheet.repro_packet_preview.redaction_posture_class,
        "preserved redaction posture must match the previewed packet"
    );
    use aureline_shell::handoff_review::PreservationActionClass;
    assert!(d
        .available_actions
        .contains(&PreservationActionClass::ExportPacket));
    assert!(d
        .available_actions
        .contains(&PreservationActionClass::SaveDraftLocal));
}

#[test]
fn negative_security_route_coerced_to_public_is_typed_error() {
    let sheet = load_sheet("negative/security_route_coerced_to_public.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::RouteVisibilityMismatch { .. }
    ));
}

#[test]
fn negative_public_target_with_security_redaction_is_typed_error() {
    let sheet = load_sheet("negative/public_target_with_security_redaction.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::RedactionPostureUnsafeForVisibility { .. }
    ));
}

#[test]
fn negative_blocked_handoff_discard_only_is_typed_error() {
    let sheet = load_sheet("negative/blocked_handoff_discard_only.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::BlockedHandoffMissingPreservationActions { .. }
    ));
}

#[test]
fn negative_shared_without_preview_confirmation_is_typed_error() {
    let sheet = load_sheet("negative/shared_without_preview_confirmation.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::SharedWithoutPreviewConfirmation { .. }
    ));
}

#[test]
fn negative_missing_safe_fallback_is_typed_error() {
    let sheet = load_sheet("negative/missing_safe_fallback.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::MissingSafeFallback { .. }
    ));
}

#[test]
fn negative_preserved_visibility_mismatch_is_typed_error() {
    let sheet = load_sheet("negative/preserved_visibility_mismatch.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::PreservedVisibilityMismatch { .. }
    ));
}

#[test]
fn negative_handoff_missing_build_context_export_is_typed_error() {
    let sheet = load_sheet("negative/handoff_missing_build_context_export.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::MissingBuildContextExport { .. }
    ));
}

#[test]
fn negative_unredacted_attachment_is_typed_error() {
    let sheet = load_sheet("negative/unredacted_attachment.json");
    let err = sheet.validate().unwrap_err();
    assert!(matches!(
        err,
        HandoffReviewValidationError::AttachmentNotRedacted { .. }
    ));
}
