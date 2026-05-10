//! Protected-walk integration test for safe mode, recovery-ladder stubs,
//! suspicious-content detection, and representation-labeled safe preview.
//!
//! Walks the failure drill and the protected walk named in the spec:
//!
//!   trigger suspicious content or broken startup
//!     -> enter safe mode or a recovery step
//!     -> confirm safe preview and recovery labels stay honest.
//!
//! The test exercises the live shell-side projections rather than mock-only
//! adapters: it runs the shared suspicious-content detector against a save
//! payload, materializes the save-review annotation, materializes the safe-
//! mode profile and crash-loop containment record, and confirms that the
//! representation-labeled copy/export paths and the recovery offers stay
//! honest end-to-end.

use aureline_shell::recovery::{
    annotate_save_review_with_suspicious_content, materialize_safe_mode_profile,
    write_safe_mode_profile_log, CrashLoopOfferKey, CrashLoopReasonClass, RecoveryLadderRung,
    SafeModeEntryReason,
};
use aureline_shell::recovery::crash_loop::{
    materialize_crash_loop_containment, write_crash_loop_containment_log,
};

#[test]
fn suspicious_save_payload_offers_safe_mode_and_representation_labels() {
    // Local buffer with a bidi control inside an identifier.
    let local = "fn pay_to(\u{202E}user: &str) {}\n".as_bytes();

    let annotation = annotate_save_review_with_suspicious_content(
        "case:save:protected_walk",
        local,
        "standard_inline",
        "review_hunk",
        16,
    )
    .expect("suspicious content must be detected on the protected walk");

    // The detector outcome is sanitize on the annotation layer; bytes stay raw.
    assert_eq!(annotation.detector_outcome, "sanitize");
    assert!(annotation.finding_count >= 1);
    assert!(annotation
        .case_record
        .findings
        .iter()
        .any(|f| f.content_class == "bidi_control"));

    // Representation-labeled copy/export pair must be present and reachable.
    let copy_raw = annotation
        .representation_transfers
        .iter()
        .find(|t| t.action_id == "copy_raw")
        .expect("copy_raw must be offered");
    let copy_escaped = annotation
        .representation_transfers
        .iter()
        .find(|t| t.action_id == "copy_escaped")
        .expect("copy_escaped must be offered");
    assert!(copy_raw.must_offer_also.iter().any(|s| s == "copy_escaped"));
    assert!(copy_escaped.must_offer_also.iter().any(|s| s == "copy_raw"));
    assert!(copy_raw.required_disclosure_fields.iter().any(|d| d == "warning_state"));

    // Escaped preview escapes the bidi codepoint instead of replaying it.
    assert!(annotation
        .escaped_preview_lines
        .iter()
        .any(|line| line.contains("\\u{202E}")));

    // Status line documents the safe-mode hand-off.
    let status = annotation.status_line();
    assert!(status.contains("safe_mode_offered=true"));
    assert!(status.contains("cmd:workspace.enter_safe_mode"));
}

#[test]
fn safe_mode_profile_preserves_state_and_disables_extensions() {
    let dir = tempfile::tempdir().expect("tempdir");

    let profile = materialize_safe_mode_profile(SafeModeEntryReason::SuspiciousContentDetected);

    // Auto-restore and extension activation must be off; state must be preserved.
    assert!(profile.auto_restore_forbidden);
    assert!(profile.trust_widening_forbidden_without_review);
    assert!(profile
        .disabled_or_narrowed_capabilities
        .iter()
        .any(|c| c == "extension_auto_activation"));
    assert!(profile
        .preserved_state_classes
        .iter()
        .any(|c| c == "user_authored_files"));
    assert!(profile
        .preserved_state_classes
        .iter()
        .any(|c| c == "session_restore_store"));

    // The profile must offer the export path so the user can hand off
    // evidence without leaving safe mode.
    assert!(profile.offers("export_escalation_packet"));

    write_safe_mode_profile_log(dir.path(), &profile).expect("write profile");
    let contents =
        std::fs::read_to_string(dir.path().join("safe_mode_profile_latest.json")).unwrap();
    assert!(contents.contains("\"safe_mode_profile_record\""));
    assert!(contents.contains("\"suspicious_content_detected\""));
}

#[test]
fn crash_loop_containment_exposes_first_class_offers_and_safe_mode_profile() {
    let dir = tempfile::tempdir().expect("tempdir");
    let record =
        materialize_crash_loop_containment(CrashLoopReasonClass::StrikeBudgetExceeded);

    for required in [
        CrashLoopOfferKey::OpenSafeMode,
        CrashLoopOfferKey::DisableSuspectExtensionOrRuntime,
        CrashLoopOfferKey::OpenWithoutRestore,
        CrashLoopOfferKey::ExportEvidence,
    ] {
        assert!(
            record.first_class(required),
            "{} must be a first-class crash-loop offer",
            required.as_str()
        );
    }

    // Cache/index repair must be visible but gated.
    assert!(record.exposes(CrashLoopOfferKey::RepairCacheOrIndex));
    assert!(!record.first_class(CrashLoopOfferKey::RepairCacheOrIndex));

    // Containment record carries the safe-mode profile so the surface and
    // the diagnostics packet share one truth.
    assert_eq!(record.safe_mode_profile.entry_reason_class, "crash_loop_detected");
    assert!(record.never_deletes_state);
    assert!(record.auto_rerun_forbidden);

    // The associated_rung mapping pairs first-class offers with their
    // canonical recovery-ladder rung.
    assert_eq!(
        CrashLoopOfferKey::OpenSafeMode.associated_rung(),
        RecoveryLadderRung::SafeMode
    );
    assert_eq!(
        CrashLoopOfferKey::ExportEvidence.associated_rung(),
        RecoveryLadderRung::ExportEvidence
    );

    write_crash_loop_containment_log(dir.path(), &record).expect("write");
    let contents =
        std::fs::read_to_string(dir.path().join("crash_loop_containment_latest.json")).unwrap();
    assert!(contents.contains("\"crash_loop_containment_record\""));
    assert!(contents.contains("\"open_safe_mode\""));
    assert!(contents.contains("\"export_evidence\""));
}
