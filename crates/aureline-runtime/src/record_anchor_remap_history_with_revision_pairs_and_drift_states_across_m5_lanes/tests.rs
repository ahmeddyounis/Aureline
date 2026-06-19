use super::*;

const MINTED_AT: &str = "2026-06-19T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[allow(clippy::too_many_arguments)]
fn entry(
    label: &str,
    sequence: u32,
    lane: AnchorDriftLaneClass,
    old_anchor: Option<&str>,
    new_anchor: Option<&str>,
    basis: AnchorRemapEvidenceBasisClass,
    from_rev: &str,
    to_rev: &str,
    actor: AnchorRemapActorClass,
) -> AnchorRemapHistoryEntry {
    AnchorRemapHistoryEntry::new(AnchorRemapHistoryEntryInput {
        entry_id: format!("entry:{label}:{sequence}"),
        sequence,
        drift_lane_class: lane,
        old_anchor_ref: old_anchor.map(str::to_owned),
        new_anchor_ref: new_anchor.map(str::to_owned),
        evidence_basis_class: basis,
        evidence_basis_ref: format!("evidence:{label}:{sequence}"),
        revision_pair: RevisionPair::new(from_rev, to_rev),
        actor_class: actor,
        actor_tool_ref: format!("actor:{label}"),
        produced_at: MINTED_AT.to_owned(),
        export_safe_summary: format!("Remap entry {sequence} for {label}."),
    })
}

/// A file-edit history: the anchor is created exact, then a later edit moves it so
/// it only contextually survives.
fn file_edit_history() -> AnchorRemapHistory {
    AnchorRemapHistory::new(AnchorRemapHistoryInput {
        history_id: "history:test:file-edit:0001".to_owned(),
        anchor_family_id: "anchor-family:test:file-edit:0001".to_owned(),
        diagnostic_id: "diagnostic:test:file-edit:0001".to_owned(),
        entries: vec![
            entry(
                "file-edit",
                0,
                AnchorDriftLaneClass::FileEdit,
                None,
                Some("anchor:file-edit:rev0"),
                AnchorRemapEvidenceBasisClass::ExactRangePreserved,
                "rev:file-edit:0",
                "rev:file-edit:0",
                AnchorRemapActorClass::EditorEditTracker,
            ),
            entry(
                "file-edit",
                1,
                AnchorDriftLaneClass::FileEdit,
                Some("anchor:file-edit:rev0"),
                Some("anchor:file-edit:rev1"),
                AnchorRemapEvidenceBasisClass::SurroundingContextMatch,
                "rev:file-edit:0",
                "rev:file-edit:1",
                AnchorRemapActorClass::EditorEditTracker,
            ),
        ],
        export_safe_summary: "File edit moved the anchor; it now only contextually survives."
            .to_owned(),
    })
}

/// An imported-snapshot history: a snapshot-only static mapping that is then
/// revalidated against a later local revision.
fn imported_snapshot_history() -> AnchorRemapHistory {
    AnchorRemapHistory::new(AnchorRemapHistoryInput {
        history_id: "history:test:imported:0001".to_owned(),
        anchor_family_id: "anchor-family:test:imported:0001".to_owned(),
        diagnostic_id: "diagnostic:test:imported:0001".to_owned(),
        entries: vec![
            entry(
                "imported",
                0,
                AnchorDriftLaneClass::ImportedSnapshotComparison,
                None,
                Some("anchor:imported:static"),
                AnchorRemapEvidenceBasisClass::ImportedStaticLocation,
                "rev:imported:snapshot",
                "rev:imported:snapshot",
                AnchorRemapActorClass::ImportedScanComparator,
            ),
            entry(
                "imported",
                1,
                AnchorDriftLaneClass::ImportedSnapshotComparison,
                Some("anchor:imported:static"),
                Some("anchor:imported:mapped"),
                AnchorRemapEvidenceBasisClass::SurroundingContextMatch,
                "rev:imported:snapshot",
                "rev:imported:local-1",
                AnchorRemapActorClass::ImportedScanComparator,
            ),
        ],
        export_safe_summary:
            "Imported snapshot mapped a static location onto a later local revision.".to_owned(),
    })
}

/// A genesis-only exact history: the anchor exists and has never moved.
fn stable_exact_history() -> AnchorRemapHistory {
    AnchorRemapHistory::new(AnchorRemapHistoryInput {
        history_id: "history:test:stable:0001".to_owned(),
        anchor_family_id: "anchor-family:test:stable:0001".to_owned(),
        diagnostic_id: "diagnostic:test:stable:0001".to_owned(),
        entries: vec![entry(
            "stable",
            0,
            AnchorDriftLaneClass::FileEdit,
            None,
            Some("anchor:stable:rev0"),
            AnchorRemapEvidenceBasisClass::ExactRangePreserved,
            "rev:stable:0",
            "rev:stable:0",
            AnchorRemapActorClass::EditorEditTracker,
        )],
        export_safe_summary: "Stable anchor, never moved.".to_owned(),
    })
}

fn guardrails() -> AnchorRemapGuardrails {
    AnchorRemapGuardrails {
        drift_never_silently_dropped: true,
        same_remap_vocabulary_across_lanes: true,
        history_is_append_only: true,
        history_is_exportable: true,
        imported_static_supported_for_snapshot_only: true,
        no_silent_anchor_repair: true,
        revision_pair_recorded_per_remap: true,
    }
}

fn consumer_projection() -> AnchorRemapConsumerProjection {
    AnchorRemapConsumerProjection {
        editor_shows_remap_state: true,
        problems_shows_remap_state: true,
        review_shows_remap_history: true,
        cli_shows_remap_state: true,
        support_export_preserves_history: true,
    }
}

fn packet_with(histories: Vec<AnchorRemapHistory>) -> AnchorRemapHistorySetPacket {
    AnchorRemapHistorySetPacket::new(AnchorRemapHistorySetPacketInput {
        packet_id: "packet:test:0001".to_owned(),
        set_label: "Test anchor-remap history set".to_owned(),
        workspace_id: "workspace:test".to_owned(),
        histories,
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: refs(&[
            M5_ANCHOR_REMAP_HISTORY_SET_SCHEMA_REF,
            M5_ANCHOR_REMAP_HISTORY_SET_DOC_REF,
            M5_ANCHOR_REMAP_HISTORY_SET_ARTIFACT_REF,
            CANONICAL_DIAGNOSTIC_RECORD_SET_SCHEMA_REF,
        ]),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

// ---- Checked artifact ----------------------------------------------------

#[test]
fn checked_export_validates() {
    let packet = current_m5_anchor_remap_history_set_export()
        .expect("checked anchor-remap history set export validates");
    assert!(packet.validate().is_empty());
    assert!(packet.histories.len() >= 5);
}

#[test]
fn checked_export_round_trips() {
    let packet = current_m5_anchor_remap_history_set_export().expect("export validates");
    let json = packet.export_safe_json();
    let parsed: AnchorRemapHistorySetPacket =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn checked_export_covers_all_drift_lanes() {
    let packet = current_m5_anchor_remap_history_set_export().expect("export validates");
    assert!(packet.covers_all_drift_lanes());
    for lane in AnchorDriftLaneClass::ALL {
        assert!(
            packet.represented_drift_lanes().contains(&lane),
            "lane {lane:?} missing"
        );
    }
}

#[test]
fn checked_export_covers_every_remap_state() {
    let packet = current_m5_anchor_remap_history_set_export().expect("export validates");
    for state in [
        DiagnosticAnchorRemapStateClass::Exact,
        DiagnosticAnchorRemapStateClass::Contextual,
        DiagnosticAnchorRemapStateClass::Stale,
        DiagnosticAnchorRemapStateClass::Unmapped,
        DiagnosticAnchorRemapStateClass::ImportedStatic,
    ] {
        assert!(
            packet.represented_states().contains(&state),
            "state {state:?} missing"
        );
    }
}

#[test]
fn checked_export_support_export_preserves_history() {
    let packet = current_m5_anchor_remap_history_set_export().expect("export validates");
    assert!(packet.support_export.preserves(&packet.histories));
    assert!(!packet.support_export.raw_source_content_included);
    assert!(!packet.support_export.raw_payload_included);
    for history in &packet.histories {
        assert!(packet
            .support_export
            .history_refs
            .contains(&history.history_id));
    }
}

#[test]
fn markdown_summary_names_histories_and_entries() {
    let packet = current_m5_anchor_remap_history_set_export().expect("export validates");
    let summary = packet.render_markdown_summary();
    assert!(summary.contains("M5 Anchor-Remap History Set"));
    assert!(summary.contains("Drift lanes covered:"));
    assert!(summary.contains("imported_static"));
}

// ---- Entry / history truth -----------------------------------------------

#[test]
fn evidence_basis_determines_state_and_imported_flag() {
    let exact = entry(
        "basis",
        0,
        AnchorDriftLaneClass::FileEdit,
        None,
        Some("anchor:x"),
        AnchorRemapEvidenceBasisClass::ExactRangePreserved,
        "r0",
        "r0",
        AnchorRemapActorClass::EditorEditTracker,
    );
    assert_eq!(
        exact.remap_state_class,
        DiagnosticAnchorRemapStateClass::Exact
    );
    assert!(exact.maps_cleanly());
    assert!(!exact.imported_static);

    let imported = entry(
        "basis",
        0,
        AnchorDriftLaneClass::ImportedReplayComparison,
        None,
        Some("anchor:imported"),
        AnchorRemapEvidenceBasisClass::ImportedStaticLocation,
        "r0",
        "r0",
        AnchorRemapActorClass::ReplayComparator,
    );
    assert_eq!(
        imported.remap_state_class,
        DiagnosticAnchorRemapStateClass::ImportedStatic
    );
    assert!(imported.imported_static);
    assert!(imported.imported_static_consistency());
    assert!(imported.requires_disclosure());
}

#[test]
fn history_is_append_only_and_continuous() {
    let history = file_edit_history();
    assert!(history.is_append_only());
    assert!(history.sequence_is_monotonic());
    assert!(history.revisions_are_continuous());
    assert!(history.anchor_chain_is_continuous());
    assert!(history.no_silent_repair());
    assert!(history.current_state_consistent());
    assert!(history.drift_lanes_consistent());
    assert!(history.is_structurally_complete());
    assert!(history.demonstrates_explicit_drift());
    assert_eq!(
        history.current_state_class,
        DiagnosticAnchorRemapStateClass::Contextual
    );
}

#[test]
fn history_projects_into_shared_anchor_remap() {
    let history = file_edit_history();
    let remap = history
        .current_anchor_remap()
        .expect("non-empty history projects an anchor remap");
    assert_eq!(remap.anchor_family_id, history.anchor_family_id);
    assert_eq!(
        remap.remap_state_class,
        DiagnosticAnchorRemapStateClass::Contextual
    );
    assert_eq!(
        remap.current_anchor_ref.as_deref(),
        Some("anchor:file-edit:rev1")
    );
    assert_eq!(
        remap.current_revision_ref.as_deref(),
        Some("rev:file-edit:1")
    );
    assert!(remap.requires_disclosure());
}

#[test]
fn surface_projections_expose_state_and_history() {
    let packet = packet_with(vec![file_edit_history()]);
    for history in &packet.histories {
        for surface_class in REMAP_EXPOSURE_SURFACES {
            let projection = packet
                .projection_for(&history.history_id, surface_class)
                .expect("required surface projection exists");
            assert!(projection.is_honest(history));
            assert_eq!(projection.entry_count, history.entries.len());
            assert!(projection.only_contextually_survives);
            assert!(!projection.maps_cleanly);
        }
    }
}

// ---- Validation: append-only and silent repair ---------------------------

#[test]
fn broken_sequence_is_flagged() {
    let mut history = file_edit_history();
    history.entries[1].sequence = 5;
    assert!(!history.is_append_only());
    let violations = packet_with(vec![history]).validate();
    assert!(violations.contains(&AnchorRemapViolation::HistoryNotAppendOnly));
}

#[test]
fn broken_revision_continuity_is_flagged() {
    let mut history = file_edit_history();
    history.entries[1].revision_pair.from_revision_ref = "rev:wrong".to_owned();
    assert!(!history.revisions_are_continuous());
    let violations = packet_with(vec![history]).validate();
    assert!(violations.contains(&AnchorRemapViolation::HistoryNotAppendOnly));
}

#[test]
fn broken_anchor_chain_is_flagged() {
    let mut history = file_edit_history();
    history.entries[1].old_anchor_ref = Some("anchor:wrong".to_owned());
    assert!(!history.anchor_chain_is_continuous());
    let violations = packet_with(vec![history]).validate();
    assert!(violations.contains(&AnchorRemapViolation::AnchorChainBroken));
}

#[test]
fn silent_anchor_repair_is_flagged() {
    let mut history = file_edit_history();
    // Claim an exact mapping while the evidence basis only supports contextual.
    history.entries[1].remap_state_class = DiagnosticAnchorRemapStateClass::Exact;
    assert!(!history.no_silent_repair());
    let violations = packet_with(vec![history]).validate();
    assert!(violations.contains(&AnchorRemapViolation::SilentAnchorRepair));
}

#[test]
fn inconsistent_current_state_is_flagged() {
    let mut history = file_edit_history();
    history.current_state_class = DiagnosticAnchorRemapStateClass::Stale;
    assert!(!history.current_state_consistent());
    let violations = packet_with(vec![history]).validate();
    assert!(violations.contains(&AnchorRemapViolation::CurrentStateInconsistent));
}

#[test]
fn inconsistent_imported_static_flag_is_flagged() {
    let mut history = imported_snapshot_history();
    history.entries[0].imported_static = false;
    assert!(!history.entries[0].imported_static_consistency());
    let violations = packet_with(vec![history]).validate();
    assert!(violations.contains(&AnchorRemapViolation::ImportedStaticInconsistent));
}

#[test]
fn imported_static_on_non_imported_lane_is_flagged() {
    let mut history = imported_snapshot_history();
    // An imported-static state can only come from an imported lane.
    history.entries[0].drift_lane_class = AnchorDriftLaneClass::FileEdit;
    assert!(!history.entries[0].imported_static_consistency());
    let violations = packet_with(vec![history]).validate();
    assert!(violations.contains(&AnchorRemapViolation::ImportedStaticInconsistent));
}

#[test]
fn unmapped_entry_with_anchor_is_flagged() {
    let mut history = file_edit_history();
    // Force an unmapped state but leave a current anchor present.
    history.entries[1].evidence_basis_class = AnchorRemapEvidenceBasisClass::NoMappingFound;
    history.entries[1].remap_state_class = DiagnosticAnchorRemapStateClass::Unmapped;
    // new_anchor_ref still Some -> inconsistent.
    assert!(!history.entries[1].anchor_consistency());
    let violations = packet_with(vec![history]).validate();
    assert!(violations.contains(&AnchorRemapViolation::AnchorRefInconsistent));
}

#[test]
fn missing_explicit_drift_proof_is_flagged() {
    let violations = packet_with(vec![stable_exact_history()]).validate();
    assert!(violations.contains(&AnchorRemapViolation::ExplicitDriftProofMissing));
}

// ---- Validation: projections and export ----------------------------------

#[test]
fn missing_surface_projection_is_flagged() {
    let mut packet = packet_with(vec![file_edit_history()]);
    packet
        .surface_projections
        .retain(|projection| projection.surface_class != DiagnosticSurfaceClass::Review);
    let violations = packet.validate();
    assert!(violations.contains(&AnchorRemapViolation::SurfaceProjectionMissing));
}

#[test]
fn projection_dropping_history_is_flagged() {
    let mut packet = packet_with(vec![file_edit_history()]);
    let projection = packet
        .surface_projections
        .iter_mut()
        .find(|projection| projection.surface_class == DiagnosticSurfaceClass::Editor)
        .expect("editor projection exists");
    projection.exposes_remap_history = false;
    let violations = packet.validate();
    assert!(violations.contains(&AnchorRemapViolation::SurfaceProjectionDropsHistory));
}

#[test]
fn raw_content_in_support_export_is_flagged() {
    let mut packet = packet_with(vec![file_edit_history()]);
    packet.support_export.raw_payload_included = true;
    let violations = packet.validate();
    assert!(violations.contains(&AnchorRemapViolation::SupportExportIncludesRawContent));
}

#[test]
fn lossy_support_export_is_flagged() {
    let mut packet = packet_with(vec![file_edit_history()]);
    packet.support_export.history_trails.clear();
    let violations = packet.validate();
    assert!(violations.contains(&AnchorRemapViolation::SupportExportLossy));
}

// ---- Validation: packet-level invariants ---------------------------------

#[test]
fn empty_history_set_is_flagged() {
    let violations = packet_with(Vec::new()).validate();
    assert!(violations.contains(&AnchorRemapViolation::NoHistories));
}

#[test]
fn missing_source_contract_is_flagged() {
    let mut packet = packet_with(vec![file_edit_history()]);
    packet
        .source_contract_refs
        .retain(|r| r != CANONICAL_DIAGNOSTIC_RECORD_SET_SCHEMA_REF);
    let violations = packet.validate();
    assert!(violations.contains(&AnchorRemapViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_are_flagged() {
    let mut packet = packet_with(vec![file_edit_history()]);
    packet.guardrails.no_silent_anchor_repair = false;
    let violations = packet.validate();
    assert!(violations.contains(&AnchorRemapViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_is_flagged() {
    let mut packet = packet_with(vec![file_edit_history()]);
    packet.consumer_projection.support_export_preserves_history = false;
    let violations = packet.validate();
    assert!(violations.contains(&AnchorRemapViolation::ConsumerProjectionIncomplete));
}

#[test]
fn forbidden_boundary_material_is_flagged() {
    let mut packet = packet_with(vec![file_edit_history()]);
    packet.set_label = "leaked password material".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&AnchorRemapViolation::RawBoundaryMaterialInExport));
}

#[test]
fn complete_packet_has_no_violations() {
    let packet = packet_with(vec![file_edit_history(), imported_snapshot_history()]);
    assert!(packet.validate().is_empty());
}
