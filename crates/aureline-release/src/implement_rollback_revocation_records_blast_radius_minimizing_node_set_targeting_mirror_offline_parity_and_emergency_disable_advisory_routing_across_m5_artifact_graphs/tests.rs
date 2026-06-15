use super::*;

use crate::release_center_model::{ArtifactGraphConsistency, RollbackOrRevocationKind};

fn register() -> M5ArtifactGraphRecoveryRegister {
    current_m5_artifact_graph_recovery_register().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, M5_ARTIFACT_GRAPH_RECOVERY_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_ARTIFACT_GRAPH_RECOVERY_RECORD_KIND);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "register must validate cleanly: {violations:#?}"
    );
    assert!(!r.rows.is_empty());
}

#[test]
fn embedded_json_matches_builder() {
    // The checked-in JSON must be exactly what the in-code builder produces, so the
    // embedded consumer and the artifact never drift.
    assert_eq!(register(), build_m5_artifact_graph_recovery_register());
}

#[test]
fn builder_validates_cleanly() {
    assert_eq!(
        build_m5_artifact_graph_recovery_register().validate(),
        Vec::new()
    );
}

#[test]
fn covers_every_family_kind() {
    let r = register();
    for kind in M5ArtifactFamilyKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "family kind {} must have at least one ledger",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_candidate() {
    let r = register();
    assert!(!r.release_blocking_candidate_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_rows()
        .iter()
        .map(|row| row.candidate_ref.as_str())
        .collect();
    for declared in &r.release_blocking_candidate_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking ledger"
        );
    }
}

#[test]
fn register_narrows_at_least_one_family() {
    let r = register();
    assert!(
        !r.rows_narrowed().is_empty(),
        "the register must narrow at least one family below the cutline"
    );
}

#[test]
fn every_narrowing_reason_has_a_stop_rule() {
    let r = register();
    let covered: std::collections::BTreeSet<NarrowingReason> = r
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in NarrowingReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn contained_families_deliver_to_hosted_mirror_and_offline_at_parity() {
    // Acceptance: hosted, mirrored, and offline customers receive the current
    // rollback/revocation/advisory truth for claimed families.
    let r = register();
    for row in r.rows_contained() {
        for channel in DeliveryChannel::ALL {
            assert_eq!(
                row.channel_parity.channel_state(channel),
                Some(ChannelDeliveryState::Current),
                "contained family {} must deliver to {}",
                row.entry_id,
                channel.as_str()
            );
        }
        assert!(
            row.channel_parity.channels_at_parity(&row.record_ids()),
            "contained family {} channels must be at recovery parity",
            row.entry_id
        );
    }
}

#[test]
fn every_recovery_record_targets_the_smallest_node_set() {
    // Acceptance/guardrail: records target the smallest affected node set, classify
    // every node, and never over-revoke an installable node.
    let r = register();
    for row in &r.rows {
        for record in &row.recovery_records {
            assert!(
                row.record_blast_radius_scoped(record),
                "ledger {} record {} must scope its blast radius to the node set",
                row.entry_id,
                record.record_id
            );
            assert!(
                row.record_preserves_unaffected(record),
                "ledger {} record {} must preserve every unaffected node",
                row.entry_id,
                record.record_id
            );
            assert!(
                !row.record_overrevokes(record),
                "ledger {} record {} must not over-revoke a preservable node",
                row.entry_id,
                record.record_id
            );
        }
    }
}

#[test]
fn unaffected_nodes_remain_installable() {
    // Acceptance: unaffected artifact nodes remain installable where the graph model
    // allows it. Every family keeps at least one node installable after the action.
    let r = register();
    for row in &r.rows {
        assert!(
            row.affected_node_set
                .iter()
                .any(|node| node.installable_after_action),
            "ledger {} must preserve at least one installable node",
            row.entry_id
        );
        // Every installable node must be in each record's preserved set.
        for record in &row.recovery_records {
            for node in &row.affected_node_set {
                if node.installable_after_action {
                    assert!(
                        record.unaffected_artifact_refs.contains(&node.artifact_ref),
                        "ledger {} record {} must list installable node {} as preserved",
                        row.entry_id,
                        record.record_id,
                        node.artifact_ref
                    );
                }
            }
        }
    }
}

#[test]
fn emergency_disable_rides_the_same_record_model() {
    // Emergency-disable bundles and security advisories ride the same auditable
    // record model as an ordinary rollback.
    let r = register();
    let total_emergency: usize = r
        .rows
        .iter()
        .map(|row| row.emergency_disable_records().len())
        .sum();
    assert!(
        total_emergency > 0,
        "the register must capture at least one emergency-disable record"
    );
    // A contained family's emergency-disable is reconciled and routes an advisory.
    for row in r.rows_contained() {
        for record in row.emergency_disable_records() {
            assert!(FamilyRecoveryLedger::record_emergency_reconciled(record));
            assert!(
                !FamilyRecoveryLedger::record_advisory_missing(record),
                "a contained emergency-disable must route an advisory"
            );
        }
    }
}

#[test]
fn advisory_export_replays_kind_blast_radius_advisory_and_channels() {
    // Acceptance: support/advisory exports replay the recovery action and prove the
    // hosted/mirrored/offline channels received the same truth.
    let r = register();
    let projection = r.support_export_projection();
    let contained = projection
        .rows
        .iter()
        .find(|row| row.publishes_stable)
        .expect("a contained ledger exists");
    assert!(!contained.replay.is_empty());
    assert_eq!(contained.channel_states.len(), DeliveryChannel::ALL.len());
    assert!(contained.channels_at_parity);
    for entry in &contained.replay {
        assert!(
            !entry.advisory_refs.is_empty()
                || entry.kind == RollbackOrRevocationKind::Rollback
                || entry.kind == RollbackOrRevocationKind::Repin,
            "a withdrawal action must replay its routed advisory"
        );
        assert!(
            entry.affected_artifact_count > 0,
            "replay must record the affected node count"
        );
    }
}

#[test]
fn summary_counts_match_rows() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.entries_contained + r.summary.entries_narrowed,
        r.rows.len()
    );
    assert!(r.summary.total_emergency_disable_records > 0);
    assert_eq!(
        r.summary.records_rollback
            + r.summary.records_revoke
            + r.summary.records_yank
            + r.summary.records_repin
            + r.summary.records_emergency_disable,
        r.summary.total_recovery_records
    );
}

#[test]
fn publication_decision_matches_computed() {
    let r = register();
    assert_eq!(r.publication.decision, r.computed_publication_decision());
    assert_eq!(
        r.publication.blocking_rule_ids,
        r.computed_blocking_rule_ids()
    );
    assert_eq!(
        r.publication.blocking_claim_ids,
        r.computed_blocking_entry_ids()
    );
}

#[test]
fn narrowed_family_surfaces_its_gaps_in_export() {
    let r = register();
    let projection = r.support_export_projection();
    let narrowed = projection
        .rows
        .iter()
        .find(|row| !row.publishes_stable)
        .expect("a narrowed family exists");
    assert!(
        !narrowed.active_narrowing_reasons.is_empty(),
        "a narrowed family must surface its narrowing reasons"
    );
    assert!(
        !narrowed.channels_at_parity
            || narrowed
                .active_narrowing_reasons
                .contains(&NarrowingReason::GraphConsistencyBroken),
        "a narrowed family must surface a concrete recovery gap"
    );
}

#[test]
fn export_projection_mirrors_rows() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.rows.len());
    for (row, proj) in r.rows.iter().zip(&projection.rows) {
        assert_eq!(row.entry_id, proj.entry_id);
        assert_eq!(row.publishes_stable(), proj.publishes_stable);
        assert_eq!(row.recovery_records.len(), proj.recovery_record_count);
        assert_eq!(
            row.emergency_disable_records().len(),
            proj.emergency_disable_count
        );
    }
}

#[test]
fn validate_flags_a_contained_family_with_active_gap() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a contained family exists");
    row.active_narrowing_reasons
        .push(NarrowingReason::ProofPacketMissing);
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ArtifactGraphRecoveryViolation::HeldWithActiveGap { .. }
    )));
}

#[test]
fn validate_flags_an_over_revoked_preservable_node() {
    // Guardrail: a record may not over-revoke a node the graph model marks
    // installable. Pulling a preserved node into the affected set must trip it.
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a contained family exists");
    let preserved_ref = row
        .affected_node_set
        .iter()
        .find(|node| node.installable_after_action)
        .map(|node| node.artifact_ref.clone())
        .expect("a preserved node exists");
    row.recovery_records[0]
        .affected_artifact_refs
        .push(preserved_ref);
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ArtifactGraphRecoveryViolation::OverRevokedPreservableNode { .. }
    )));
}

#[test]
fn validate_flags_emergency_truth_withheld_from_offline() {
    // Guardrail: an emergency-bearing family may not withhold the truth from the
    // offline channel while the hosted channel already has it.
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| {
            row.publishes_stable()
                && row.carries_routed_advisory()
                && row.channel_parity.channel_state(DeliveryChannel::Hosted)
                    == Some(ChannelDeliveryState::Current)
        })
        .expect("a contained advisory-bearing family exists");
    let offline = row
        .channel_parity
        .channels
        .iter_mut()
        .find(|c| c.channel == DeliveryChannel::Offline)
        .expect("offline channel exists");
    offline.delivery_state = ChannelDeliveryState::Undelivered;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ArtifactGraphRecoveryViolation::EmergencyTruthWithheldFromChannel {
            channel: DeliveryChannel::Offline,
            ..
        }
    )));
}

#[test]
fn validate_flags_a_held_family_with_broken_graph() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a contained family exists");
    row.recovery_records[0].artifact_graph_consistency = ArtifactGraphConsistency::Broken;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ArtifactGraphRecoveryViolation::RecoveryGapWithoutReason {
            reason: NarrowingReason::GraphConsistencyBroken,
            ..
        }
    )));
}

#[test]
fn validate_flags_a_contained_family_without_signoff() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a contained family exists");
    row.owner_signoff.signed_off = false;
    row.owner_signoff.signed_at = None;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ArtifactGraphRecoveryViolation::HeldWithoutSignoff { .. }
    )));
}

#[test]
fn validate_flags_a_mirror_delivery_gap_on_a_held_family() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a contained family exists");
    let mirror = row
        .channel_parity
        .channels
        .iter_mut()
        .find(|c| c.channel == DeliveryChannel::Mirrored)
        .expect("mirror channel exists");
    mirror.delivery_state = ChannelDeliveryState::Undelivered;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ArtifactGraphRecoveryViolation::HeldWithoutChannelParity { .. }
            | M5ArtifactGraphRecoveryViolation::RecoveryGapWithoutReason {
                reason: NarrowingReason::MirrorParityMissing,
                ..
            }
    )));
}
