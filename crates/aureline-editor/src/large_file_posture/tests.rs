//! Unit tests for the large-file posture projection.

use super::*;
use crate::large_file_mode::{
    default_limited_mode_capabilities, LimitedModeActivationTrigger, LimitedModeCapabilityState,
    LimitedModeEditPolicyClass, LimitedModeFileRecord, LimitedModeOverrideAction,
    LimitedModeSafePreviewClass, LimitedModeWritePolicyClass, LIMITED_MODE_FILE_RECORD_KIND,
    LIMITED_MODE_FILE_SCHEMA_REF, LIMITED_MODE_FILE_SCHEMA_VERSION,
};

/// A clean limited-mode record for an oversized text file: read-only, whole-file
/// participants blocked, range-only reviewed writes available, canonical target
/// resolved, explicit override route with disclosure.
fn clean_limited_mode() -> LimitedModeFileRecord {
    LimitedModeFileRecord {
        record_kind: LIMITED_MODE_FILE_RECORD_KIND.to_owned(),
        limited_mode_file_schema_version: LIMITED_MODE_FILE_SCHEMA_VERSION,
        schema_ref: LIMITED_MODE_FILE_SCHEMA_REF.to_owned(),
        limited_mode_file_id: "limited.clean".to_owned(),
        workspace_ref: "workspace.fixture.clean".to_owned(),
        document_ref: "doc.clean".to_owned(),
        canonical_uri: "file:///workspace/big.log".to_owned(),
        bytes_on_disk: 256 * 1024 * 1024,
        activation_trigger_class: Some(LimitedModeActivationTrigger::SizeThreshold),
        activation_reason: "File size exceeds the large-file threshold.".to_owned(),
        safe_preview_class: LimitedModeSafePreviewClass::PagedRawTextPreview,
        edit_policy_class: LimitedModeEditPolicyClass::ReadOnlyByDefault,
        write_policy_class: LimitedModeWritePolicyClass::WholeFileParticipantsBlocked,
        override_action: LimitedModeOverrideAction {
            action_id: "open_anyway".to_owned(),
            label: "Open anyway".to_owned(),
            disclosure: "Opens the normal editor path and may be slow or memory intensive."
                .to_owned(),
        },
        capabilities: default_limited_mode_capabilities(),
        raw_payload_excluded: true,
        support_summary: "Limited mode with safe preview and constrained writes.".to_owned(),
    }
}

/// A clean classification observation for an oversized (non-binary) text file.
fn text_classification() -> LargeFileClassificationObservation {
    LargeFileClassificationObservation {
        mode: "large_file".to_owned(),
        trigger: Some("size_threshold".to_owned()),
        reason: "File size exceeds the large-file threshold.".to_owned(),
        bytes_on_disk: 256 * 1024 * 1024,
        sniff_bytes: 64 * 1024,
        has_null_bytes: false,
        max_line_length_in_sniff: 120,
        bom_kind: None,
        non_printable_per_mille: 1,
        looks_binary: false,
        looks_minified: false,
        matches_pack_suffix: false,
    }
}

/// A binary classification observation (NUL bytes, high non-printable ratio).
fn binary_classification() -> LargeFileClassificationObservation {
    LargeFileClassificationObservation {
        mode: "large_file".to_owned(),
        trigger: Some("classification".to_owned()),
        reason: "File appears to be binary or otherwise unsafe for full processing.".to_owned(),
        bytes_on_disk: 4 * 1024 * 1024,
        sniff_bytes: 64 * 1024,
        has_null_bytes: true,
        max_line_length_in_sniff: 64 * 1024,
        bom_kind: None,
        non_printable_per_mille: 480,
        looks_binary: true,
        looks_minified: false,
        matches_pack_suffix: false,
    }
}

fn set_capability(record: &mut LimitedModeFileRecord, id: &str, state: LimitedModeCapabilityState) {
    for cap in &mut record.capabilities {
        if cap.capability_id == id {
            cap.state = state;
        }
    }
}

#[test]
fn clean_text_large_file_is_stable_and_export_safe() {
    let record = project_large_file_posture(
        "posture.clean",
        &text_classification(),
        &clean_limited_mode(),
    );

    assert!(record.is_stable_qualified());
    assert!(record.is_support_export_safe());
    assert!(record.preview_fidelity.byte_faithful_read);
    assert!(record.preview_fidelity.source_fidelity_proven);
    assert!(record.write_posture.whole_file_participants_blocked);
    assert!(record.write_posture.restricted_write_proven);
    assert_eq!(record.canonical_uri, "file:///workspace/big.log");
    assert!(record.reduced_capability_count > 0);
    assert_eq!(record.inspection_hooks.len(), 4);
}

#[test]
fn binary_file_with_binary_safe_preview_is_stable() {
    let mut limited = clean_limited_mode();
    limited.safe_preview_class = LimitedModeSafePreviewClass::BinarySafePreview;

    let record = project_large_file_posture("posture.binary", &binary_classification(), &limited);

    assert!(record.activation.looks_binary);
    assert_eq!(
        record.preview_fidelity.safe_preview_class,
        "binary_safe_preview"
    );
    assert!(record.preview_fidelity.binary_safe_preview_selected);
    assert!(record.is_stable_qualified());
}

#[test]
fn binary_file_without_binary_safe_preview_narrows() {
    // Binary content but the record kept the paged raw-text preview: a binary
    // file rendered as garbled text is not binary-safe.
    let record = project_large_file_posture(
        "posture.binary_unsafe",
        &binary_classification(),
        &clean_limited_mode(),
    );

    assert!(!record.preview_fidelity.binary_safe_preview_selected);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LargeFilePostureNarrowReason::PreviewNotBinarySafe));
}

#[test]
fn whole_file_load_allowed_narrows_byte_faithful_read() {
    let mut limited = clean_limited_mode();
    set_capability(
        &mut limited,
        "whole_file_load_into_ram",
        LimitedModeCapabilityState::Allowed,
    );

    let record = project_large_file_posture("posture.load", &text_classification(), &limited);

    assert!(!record.preview_fidelity.byte_faithful_read);
    assert!(!record.preview_fidelity.source_fidelity_proven);
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LargeFilePostureNarrowReason::SourceReadNotByteFaithful));
}

#[test]
fn whole_file_write_participant_narrows_restricted_write() {
    let mut limited = clean_limited_mode();
    set_capability(
        &mut limited,
        "save_participant_whole_file",
        LimitedModeCapabilityState::Allowed,
    );

    let record = project_large_file_posture("posture.write", &text_classification(), &limited);

    assert!(!record.write_posture.whole_file_participants_blocked);
    assert!(!record.write_posture.restricted_write_proven);
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LargeFilePostureNarrowReason::WholeFileWriteNotRestricted));
}

#[test]
fn unresolved_canonical_target_narrows() {
    let mut limited = clean_limited_mode();
    limited.canonical_uri = "   ".to_owned();

    let record = project_large_file_posture("posture.target", &text_classification(), &limited);

    assert!(!record.write_posture.canonical_target_resolved);
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LargeFilePostureNarrowReason::CanonicalTargetUnresolved));
}

#[test]
fn undisclosed_override_route_narrows() {
    let mut limited = clean_limited_mode();
    limited.override_action.disclosure = "   ".to_owned();

    let record = project_large_file_posture("posture.override", &text_classification(), &limited);

    assert!(!record.write_posture.override_disclosed);
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LargeFilePostureNarrowReason::OverrideRouteNotDisclosed));
}

#[test]
fn missing_checkpoint_hook_narrows_destructive_action() {
    let mut hooks = default_large_file_inspection_hooks();
    for hook in &mut hooks {
        if hook.hook_class == InspectionHookClass::Checkpoint {
            hook.available = false;
        }
    }

    let record = project_large_file_posture_with_hooks(
        "posture.no_checkpoint",
        &text_classification(),
        &clean_limited_mode(),
        hooks,
    );

    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LargeFilePostureNarrowReason::DestructiveActionNoCheckpoint));
}

#[test]
fn raw_payload_included_narrows_export_safety() {
    let mut limited = clean_limited_mode();
    limited.raw_payload_excluded = false;

    let record = project_large_file_posture("posture.export", &text_classification(), &limited);

    // The projected record always excludes the raw payload, but it inherits the
    // source record's export safety: an unsafe source narrows the posture.
    assert!(record.raw_payload_excluded);
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LargeFilePostureNarrowReason::PostureExportUnsafe));
}

#[test]
fn observation_from_live_classifier_round_trips() {
    use crate::large_file::{classify_file, ClassificationPolicy};
    use std::io::Write;

    let mut tmp = std::env::temp_dir();
    tmp.push(format!(
        "aureline_large_file_posture_test_{}.bin",
        std::process::id()
    ));
    {
        let mut file = std::fs::File::create(&tmp).expect("temp file creates");
        file.write_all(&[0u8, 1, 2, 3, 0, 5, 6, 7])
            .expect("temp file writes");
    }
    let policy = ClassificationPolicy::default();
    let decision = classify_file(&tmp, &policy).expect("classify succeeds");
    let observation = LargeFileClassificationObservation::from_classification_decision(&decision);
    let _ = std::fs::remove_file(&tmp);

    assert!(observation.is_large_file());
    assert!(observation.looks_binary);
    assert!(observation.has_null_bytes);
    assert_eq!(observation.trigger.as_deref(), Some("classification"));
}

#[test]
fn lines_render_every_pillar() {
    let record = project_large_file_posture(
        "posture.lines",
        &text_classification(),
        &clean_limited_mode(),
    );
    let lines = large_file_posture_lines(&record);

    assert!(lines.iter().any(|l| l.contains("Large-file posture")));
    assert!(lines.iter().any(|l| l.contains("Evaluated capabilities:")));
    assert!(lines.iter().any(|l| l.contains("Inspection hooks:")));
    assert!(lines.iter().any(|l| l.contains("whole_file_load_into_ram")));
    assert!(lines.iter().any(|l| l.contains("compare")));
}

#[test]
fn record_round_trips_through_json() {
    let record = project_large_file_posture(
        "posture.json",
        &text_classification(),
        &clean_limited_mode(),
    );
    let json = serde_json::to_string(&record).expect("record serializes");
    let restored: LargeFilePostureRecord =
        serde_json::from_str(&json).expect("record deserializes");
    assert_eq!(record, restored);
}
