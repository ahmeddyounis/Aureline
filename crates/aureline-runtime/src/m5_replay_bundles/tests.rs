//! Inline unit coverage for the replay bundle: seed stability, the
//! dual-retention join integrity, the redaction-honoring evidence joins, and
//! the replay robustness drills.

use super::*;

fn seed() -> ReplayBundle {
    seeded_replay_bundle()
}

#[test]
fn seed_materializes_stable() {
    let bundle = seed();
    assert!(
        bundle.validate().is_empty(),
        "seed must validate clean: {:?}",
        bundle.validate()
    );
    assert_eq!(
        bundle.promotion_state,
        BuildTestInteropPromotionState::Stable
    );
    assert_eq!(bundle.record_kind, REPLAY_BUNDLE_RECORD_KIND);
    assert_eq!(bundle.schema_version, REPLAY_BUNDLE_SCHEMA_VERSION);
}

#[test]
fn seed_carries_all_three_retention_classes() {
    let bundle = seed();
    assert_eq!(
        bundle.retention_class_tokens(),
        vec![
            "metadata_digest_only",
            "redacted_reference",
            "support_approval_required",
        ]
    );
}

#[test]
fn every_event_joins_to_exactly_one_lineage_entry() {
    let bundle = seed();
    for event in &bundle.events {
        let entry = bundle
            .lineage_for(&event.raw_payload_ref)
            .expect("event joins to a lineage entry");
        assert_eq!(entry.source_kind, event.source_kind);
        assert_eq!(entry.retention_class, event.raw_payload_retention_class);
        assert!(entry.referencing_event_ids.contains(&event.event_id));
    }
    // No orphan lineage entries.
    assert_eq!(bundle.raw_lineage.len(), bundle.events.len());
}

#[test]
fn lineage_stays_typed_and_bounded() {
    let bundle = seed();
    for entry in &bundle.raw_lineage {
        assert_eq!(
            entry.retained_byte_bound,
            retention_byte_bound(entry.retention_class)
        );
        assert!(entry.payload_byte_len <= entry.retained_byte_bound);
        assert!(!entry.payload_digest.is_empty());
    }
}

#[test]
fn support_and_ai_joins_gate_approval_only_payloads() {
    let bundle = seed();
    for surface in [
        ReplayJoinSurface::SupportBundle,
        ReplayJoinSurface::IncidentPacket,
        ReplayJoinSurface::AiEvidence,
    ] {
        let view = bundle.evidence_join(surface, "view", "2026-06-17T00:01:00Z");
        assert!(view.honors_redaction());
        assert_eq!(
            view.gated_payload_count,
            1,
            "{} gates one payload",
            surface.as_str()
        );
        // The gated row keeps provenance (digest + source) but no resolvable ref.
        let gated = view
            .lineage_rows
            .iter()
            .find(|row| !row.disclosed)
            .expect("a gated row exists");
        assert_eq!(gated.retention_class, "support_approval_required");
        assert!(gated.raw_payload_ref.starts_with("<gated:"));
        assert!(!gated.payload_digest.is_empty());
    }
}

#[test]
fn replay_join_discloses_every_payload() {
    let bundle = seed();
    let view = bundle.evidence_join(ReplayJoinSurface::Replay, "view", "2026-06-17T00:01:00Z");
    assert_eq!(view.gated_payload_count, 0);
    assert_eq!(view.disclosed_payload_count, bundle.raw_lineage.len());
}

#[test]
fn normalized_rows_never_repeat_the_raw_reference() {
    let bundle = seed();
    let view = bundle.evidence_join(
        ReplayJoinSurface::AiEvidence,
        "view",
        "2026-06-17T00:01:00Z",
    );
    for row in &view.normalized_rows {
        assert!(
            !row.explanation.contains("raw:"),
            "explanation leaks a raw ref"
        );
        assert!(!row.adapter_id.is_empty(), "provenance is preserved");
    }
}

#[test]
fn join_counts_reflect_retention_posture() {
    let bundle = seed();
    let total = bundle.raw_lineage.len();
    for projection in &bundle.join_projections {
        let expected = match projection.surface {
            ReplayJoinSurface::Replay => total,
            // One approval-gated payload is not citable by the export surfaces.
            _ => total - 1,
        };
        assert_eq!(projection.citable_payload_count, expected);
    }
}

#[test]
fn all_four_robustness_cases_are_stable() {
    let bundle = seed();
    assert_eq!(
        bundle.failure_mode_tokens(),
        vec![
            "truncation",
            "duplicate_delivery",
            "adapter_drift",
            "export_import_round_trip",
        ]
    );
    for case in &bundle.robustness_cases {
        assert!(case.stable, "{} must be stable", case.failure_mode.as_str());
        assert_eq!(case.replay_digest_before, case.replay_digest_after);
        assert_eq!(
            case.recovery_posture,
            case.failure_mode.canonical_recovery()
        );
    }
}

#[test]
fn support_export_round_trips_and_stays_safe() {
    let bundle = seed();
    let export = bundle.support_export(REPLAY_BUNDLE_SUPPORT_EXPORT_ID, "2026-06-17T00:01:00Z");
    assert!(export.is_export_safe());
    let json = serde_json::to_string(&export).expect("serialize");
    let parsed: ReplayBundleSupportExport = serde_json::from_str(&json).expect("parse");
    assert_eq!(parsed, export);
    assert!(parsed.bundle.is_stable());
    assert_eq!(parsed.bundle.replay_digest, bundle.replay_digest);
}

#[test]
fn cli_headless_view_joins_every_row() {
    let bundle = seed();
    let view = bundle.cli_headless_view(REPLAY_BUNDLE_CLI_HEADLESS_ID, "2026-06-17T00:01:00Z");
    assert!(view.every_row_joins());
    assert_eq!(view.rows.len(), bundle.events.len());
    assert_eq!(view.replay_digest, bundle.replay_digest);
}

#[test]
fn missing_lineage_entry_blocks_stable() {
    let mut input = current_stable_replay_bundle_input();
    input
        .raw_lineage
        .retain(|entry| !entry.raw_payload_ref.ends_with("event:task:queued"));
    let bundle = ReplayBundle::materialize(input);
    assert_eq!(
        bundle.promotion_state,
        BuildTestInteropPromotionState::BlocksStable
    );
    assert!(bundle
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ReplayBundleFindingKind::LineageEntryMissing));
}

#[test]
fn unbounded_raw_payload_blocks_stable() {
    let mut input = current_stable_replay_bundle_input();
    if let Some(entry) = input
        .raw_lineage
        .iter_mut()
        .find(|entry| entry.retention_class == RawPayloadRetentionClass::MetadataDigestOnly)
    {
        entry.payload_byte_len = 1_000_000;
    }
    let bundle = ReplayBundle::materialize(input);
    assert!(bundle
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ReplayBundleFindingKind::RawPayloadUnbounded));
}

#[test]
fn exposing_approval_gated_payload_blocks_stable() {
    let mut input = current_stable_replay_bundle_input();
    for entry in &mut input.raw_lineage {
        if entry.retention_class == RawPayloadRetentionClass::SupportApprovalRequired {
            entry.support_export_safe = true;
            entry.ai_evidence_safe = true;
        }
    }
    let bundle = ReplayBundle::materialize(input);
    assert!(bundle
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ReplayBundleFindingKind::RetentionPostureMismatch));
}

#[test]
fn join_dropping_redaction_blocks_stable() {
    let mut input = current_stable_replay_bundle_input();
    for projection in &mut input.join_projections {
        if projection.surface == ReplayJoinSurface::AiEvidence {
            projection.honors_retention_redaction = false;
        }
    }
    let bundle = ReplayBundle::materialize(input);
    assert!(bundle
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == ReplayBundleFindingKind::JoinProjectionDropsTruth));
}
