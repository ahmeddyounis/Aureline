//! Inline tests for the M5 runbook governance matrix and operator scenarios.

use super::*;

fn canonical() -> M5RunbookGovernancePacket {
    seeded_m5_runbook_governance_packet()
}

#[test]
fn canonical_packet_validates() {
    let packet = canonical();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_RUNBOOK_GOVERNANCE_PACKET_ID);
    assert_eq!(packet.record_kind, M5_RUNBOOK_GOVERNANCE_RECORD_KIND);
}

#[test]
fn every_object_class_is_governed_with_owner_consumer_and_proof() {
    let packet = canonical();
    for class in RunbookObjectClass::ALL {
        let contract = packet
            .object_contract(class)
            .unwrap_or_else(|| panic!("object class {} ungoverned", class.as_str()));
        assert!(!contract.owner_role.is_empty());
        assert!(!contract.proof_ref.is_empty());
        assert!(!contract.governed_vocab.is_empty());
        // Each contract cites its object class's source-of-truth schema.
        assert_eq!(contract.schema_ref, class.schema_ref());
    }
    // The four governed schemas exist as distinct source refs.
    let schema_refs: std::collections::BTreeSet<&str> = packet
        .object_contracts
        .iter()
        .map(|c| c.schema_ref.as_str())
        .collect();
    assert!(schema_refs.contains(M5_RUNBOOK_SOURCE_SCHEMA_REF));
    assert!(schema_refs.contains(M5_RUNBOOK_STEP_SCHEMA_REF));
    assert!(schema_refs.contains(M5_RUNBOOK_EXECUTION_SCHEMA_REF));
}

#[test]
fn canonical_is_all_governed_green() {
    let packet = canonical();
    assert!(!packet.surface_claims.is_empty());
    for surface in &packet.surface_claims {
        assert_eq!(
            surface.status,
            RunbookSurfaceStatus::Mapped,
            "surface {} not mapped",
            surface.surface_id
        );
        assert_eq!(surface.signal, RunbookSignal::Green);
        assert!(surface.is_governed());
        assert_eq!(surface.effective_class, RunbookClaimClass::Stable);
        assert!(surface.gaps.is_empty());
        assert!(!surface.bound_object_classes.is_empty());
    }
    assert!(!packet.blocks_stable_promotion());
    let matrix = packet.matrix();
    assert_eq!(matrix.green_count, packet.surface_claims.len() as u32);
    assert_eq!(matrix.yellow_count, 0);
    assert_eq!(matrix.red_count, 0);
    assert_eq!(matrix.total_objects, RunbookObjectClass::ALL.len() as u32);
}

#[test]
fn every_object_class_is_bound_by_some_surface() {
    let packet = canonical();
    let bound: std::collections::BTreeSet<RunbookObjectClass> = packet
        .surface_claims
        .iter()
        .flat_map(|s| s.bound_object_classes.iter().copied())
        .collect();
    for class in RunbookObjectClass::ALL {
        assert!(bound.contains(&class), "object {} unbound", class.as_str());
    }
}

#[test]
fn stale_proof_drill_auto_narrows_without_blocking() {
    let packet = seeded_m5_runbook_governance_packet_stale_proof_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(!packet.blocks_stable_promotion());

    let mut narrowed_any = false;
    let mut governed_any = false;
    for surface in &packet.surface_claims {
        let binds_stale = surface
            .bound_object_classes
            .contains(&RunbookObjectClass::ControlPlaneHandoff);
        if binds_stale {
            narrowed_any = true;
            assert_eq!(surface.status, RunbookSurfaceStatus::Provisional);
            assert!(surface.is_narrowed());
            assert_eq!(surface.effective_class, RunbookClaimClass::Beta);
            assert!(surface
                .gaps
                .iter()
                .any(|g| g.gap_kind == RunbookGapKind::ProofStale));
        } else {
            governed_any = true;
            assert!(surface.is_governed());
        }
    }
    assert!(narrowed_any && governed_any);
}

#[test]
fn missing_proof_drill_blocks_stable_promotion() {
    let packet = seeded_m5_runbook_governance_packet_missing_proof_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.blocks_stable_promotion());

    let blocked = packet.blocked_surface_ids();
    assert!(!blocked.is_empty());
    for surface in &packet.surface_claims {
        let binds_missing = surface
            .bound_object_classes
            .contains(&RunbookObjectClass::ArchivalExport);
        if binds_missing {
            assert_eq!(surface.status, RunbookSurfaceStatus::Unmapped);
            assert_eq!(surface.signal, RunbookSignal::Red);
            assert!(surface.is_blocked());
            assert_eq!(surface.effective_class, RunbookClaimClass::Held);
            assert!(surface
                .gaps
                .iter()
                .any(|g| g.gap_kind == RunbookGapKind::ProofMissing));
        }
    }
}

#[test]
fn waiver_narrows_but_never_hides_true_status() {
    let packet = seeded_m5_runbook_governance_packet_waived_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    // The waiver turns the block into a narrow: promotion is no longer blocked.
    assert!(!packet.blocks_stable_promotion());

    let mut waived_any = false;
    for surface in &packet.surface_claims {
        let binds_missing = surface
            .bound_object_classes
            .contains(&RunbookObjectClass::ArchivalExport);
        if binds_missing {
            waived_any = true;
            // The true status stays red even though the gate narrows.
            assert_eq!(surface.status, RunbookSurfaceStatus::Unmapped);
            assert_eq!(surface.signal, RunbookSignal::Red);
            assert!(surface.is_narrowed());
            assert_eq!(surface.effective_class, RunbookClaimClass::Beta);
            assert!(!surface.waivers.is_empty());
            assert!(surface
                .gaps
                .iter()
                .filter(|g| g.gap_kind == RunbookGapKind::ProofMissing)
                .all(|g| g.waived));
        }
    }
    assert!(waived_any);
}

#[test]
fn unmapped_object_blocks_via_recompute() {
    // A surface that binds an object class absent from the contract inventory is blocked.
    let mut contracts = seeded_m5_runbook_governance_packet().object_contracts;
    contracts.retain(|c| c.object_class != RunbookObjectClass::ArchivalExport);
    let mut surface = seeded_m5_runbook_governance_packet()
        .surface("support-runbook-export")
        .cloned()
        .expect("surface exists");
    surface.recompute(&contracts);
    assert!(surface.is_blocked());
    assert_eq!(surface.status, RunbookSurfaceStatus::Unmapped);
    assert!(surface
        .gaps
        .iter()
        .any(|g| g.gap_kind == RunbookGapKind::ObjectMappingMissing));
}

#[test]
fn vocabulary_set_is_canonical_and_review_holds() {
    let packet = canonical();
    assert!(packet.vocabulary_set.matches_canonical());
    assert!(packet.conformance_review.all_hold());
    assert!(packet.consumer_projection.all_hold());
}

#[test]
fn release_gate_aggregate_matches_rows() {
    let packet = seeded_m5_runbook_governance_packet_missing_proof_blocked();
    let blocked: std::collections::BTreeSet<&str> =
        packet.blocked_surface_ids().into_iter().collect();
    let gate_blocked: std::collections::BTreeSet<&str> = packet
        .release_gate
        .blocked_surface_ids
        .iter()
        .map(String::as_str)
        .collect();
    assert_eq!(blocked, gate_blocked);
    assert!(packet.release_gate.blocks_stable_promotion);
}

#[test]
fn all_operator_scenarios_validate() {
    let records = seeded_operator_scenario_records();
    assert_eq!(records.len(), 4);
    for record in &records {
        assert!(record.validate().is_empty(), "{:?}", record.validate());
        assert!(record.attributable);
        assert!(record.no_hidden_mutate_channel);
        assert!(!record.archival_export.raw_content_exported);
    }
}

#[test]
fn deviation_lineage_is_recorded_and_attributable() {
    let record = super::seed::failover_deviation_lineage();
    assert!(record.validate().is_empty(), "{:?}", record.validate());
    assert_eq!(record.deviation_lineage.len(), 2);
    for note in &record.deviation_lineage {
        assert!(note.deviation_class.is_deviation());
        assert!(note.attributable);
        assert!(!note.approver_role.is_empty());
    }
    let classes: Vec<DeviationClass> = record
        .deviation_lineage
        .iter()
        .map(|n| n.deviation_class)
        .collect();
    assert!(classes.contains(&DeviationClass::StepSkipped));
    assert!(classes.contains(&DeviationClass::StepAddedAdHoc));
}

#[test]
fn deviation_notes_are_durable_and_inspectable() {
    // A deviation carries its own id, affected steps, actor, time, and summary, so it is a
    // standalone inspectable record rather than generic completion copy.
    let record = super::seed::failover_deviation_lineage();
    for note in &record.deviation_lineage {
        assert!(note.validate().is_empty(), "{:?}", note.validate());
        assert!(!note.deviation_id.is_empty());
        assert!(!note.actor_ref.is_empty());
        assert!(!note.recorded_at.is_empty());
        assert!(note.affected_step_ids.contains(&note.from_step_id));
        assert!(note
            .summary_message_id
            .starts_with(M5_RUNBOOK_MESSAGE_ID_PREFIX));
    }
    // The ad-hoc rollback deviation names more than one affected step.
    let adhoc = record
        .deviation_lineage
        .iter()
        .find(|n| n.deviation_class == DeviationClass::StepAddedAdHoc)
        .expect("ad-hoc deviation present");
    assert!(adhoc.affected_step_ids.len() >= 2);
}

#[test]
fn a_recorded_deviation_without_actor_is_unattributable() {
    let mut note = super::seed::failover_deviation_lineage().deviation_lineage[0].clone();
    note.actor_ref = String::new();
    assert!(note
        .validate()
        .contains(&M5RunbookGovernanceViolation::UnattributableDeviation));
}

#[test]
fn an_incomplete_deviation_note_is_rejected() {
    let mut note = super::seed::failover_deviation_lineage().deviation_lineage[0].clone();
    note.affected_step_ids.clear();
    assert!(note
        .validate()
        .contains(&M5RunbookGovernanceViolation::DeviationNoteIncomplete));
}

#[test]
fn archived_execution_stays_joinable_after_session() {
    // Every operator scenario archives an export-safe record joinable to other evidence
    // families through stable ids, with no raw payload retained.
    for record in seeded_operator_scenario_records() {
        let archive = &record.archival_export;
        assert!(archive.validate().is_empty(), "{:?}", archive.validate());
        assert!(archive.archived);
        assert!(!archive.archived_at.is_empty());
        assert!(archive.lineage_recoverable_from_metadata_only);
        assert!(!archive.raw_content_exported);
        assert!(
            archive.lineage_joins.has_any_join(),
            "{} has no joins",
            record.execution_id
        );
    }
    // The failover scenario joins all four evidence families.
    let failover = super::seed::failover_deviation_lineage();
    assert_eq!(
        failover.archival_export.lineage_joins.joined_families(),
        vec!["incident", "rollout", "review", "support_bundle"]
    );
}

#[test]
fn an_archival_object_retaining_raw_content_is_rejected() {
    let mut archive = super::seed::restart_pipeline_governed().archival_export;
    archive.raw_content_exported = true;
    assert!(archive
        .validate()
        .contains(&M5RunbookGovernanceViolation::RawBoundaryMaterialInExport));

    let mut archive = super::seed::restart_pipeline_governed().archival_export;
    archive.archived_at = String::new();
    assert!(archive
        .validate()
        .contains(&M5RunbookGovernanceViolation::ArchivalRecordIncomplete));
}

#[test]
fn console_handoff_stays_attributable_and_returns_to_plane() {
    let record = super::seed::vendor_console_handoff();
    assert!(record.validate().is_empty(), "{:?}", record.validate());
    let handoff_step = record
        .executed_steps
        .iter()
        .find(|s| s.step.step_class == RunbookStepClass::ConsoleHandoff)
        .expect("handoff step exists");
    let handoff = handoff_step
        .handoff
        .as_ref()
        .expect("handoff packet exists");
    assert!(handoff.boundary_class.leaves_governed_plane());
    assert!(!handoff.attribution_ref.is_empty());
    assert!(handoff.returns_to_governed_plane);
    assert!(!handoff.creates_hidden_mutate_channel);
}

#[test]
fn companion_scenario_stays_within_scope_with_no_mutating_steps() {
    let record = super::seed::companion_within_scope();
    assert!(record.validate().is_empty(), "{:?}", record.validate());
    assert!(record.companion_driven);
    for result in &record.executed_steps {
        assert!(!result.step.mutating, "companion drove a mutating step");
        assert!(result.step.companion_permitted);
    }
    assert!(record.no_hidden_mutate_channel);
}

#[test]
fn a_mutating_step_without_approval_is_a_hidden_mutate_channel() {
    let bad = RunbookStepDescriptor {
        record_kind: M5_RUNBOOK_STEP_RECORD_KIND.to_owned(),
        schema_version: M5_RUNBOOK_OBJECT_SCHEMA_VERSION,
        step_id: "bad.mutate".to_owned(),
        step_label: "Mutate with no approval".to_owned(),
        step_class: RunbookStepClass::Mitigate,
        approval_scope: RunbookApprovalScope::NoApprovalReadOnly,
        control_plane_boundary: ControlPlaneBoundaryClass::InAppGoverned,
        mutating: true,
        expected_evidence_outputs: vec!["receipt".to_owned()],
        companion_permitted: false,
        detail_message_id: format!("{}step.bad", M5_RUNBOOK_MESSAGE_ID_PREFIX),
    };
    assert!(bad
        .validate()
        .contains(&M5RunbookGovernanceViolation::HiddenMutateChannel));
}

#[test]
fn round_trips_through_json() {
    let packet = canonical();
    let json = packet.export_safe_json();
    let parsed: M5RunbookGovernancePacket = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(parsed, packet);
    assert!(parsed.validate().is_empty());

    let matrix_json = packet.matrix_json();
    let parsed_matrix: M5RunbookGovernanceMatrix =
        serde_json::from_str(&matrix_json).expect("matrix round-trips");
    assert_eq!(parsed_matrix, packet.matrix());
}

#[test]
fn markdown_summary_names_objects_and_surfaces() {
    let summary = canonical().render_markdown_summary();
    assert!(summary.contains("Governed runbook objects"));
    assert!(summary.contains("source_descriptor"));
    assert!(summary.contains("control_plane_handoff"));
    assert!(summary.contains("incident-runbook-pane"));
}
