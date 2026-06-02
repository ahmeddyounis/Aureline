//! Unit tests for the local-history export/replay lineage projection.

use super::*;

fn make_support_export() -> LocalHistoryExportReplaySupportExportInputs {
    LocalHistoryExportReplaySupportExportInputs::metadata_safe_baseline(
        LocalHistoryExportReplaySupportExportPosture::MetadataSafeExport,
    )
}

#[allow(clippy::too_many_arguments)]
fn packet(
    packet_id: &str,
    packet_kind: ExportPacketKind,
    body_class: BodyAvailabilityClass,
    body_override_disclosure_ref: Option<String>,
    encoding_class: EncodingFidelityClass,
) -> PacketObservation {
    PacketObservation {
        packet_id: packet_id.to_owned(),
        packet_kind,
        packet_ref: format!("pkt:{packet_id}"),
        body_availability_class: body_class,
        body_override_disclosure_ref,
        encoding_fidelity_class: encoding_class,
        encoding_preserved: true,
        newline_preserved: true,
        bom_preserved: true,
        restore_of_ref: format!("restore-of:{packet_id}"),
        mutation_journal_ref: format!("mj:{packet_id}"),
        actor_class: "typing".to_owned(),
        integrity_hash: format!("blake3:{packet_id}"),
        support_export: make_support_export(),
        captured_at: "mono:1700000700".to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn replay_path(
    replay_id: &str,
    replay_kind: ReplayPathKind,
    packet_id: &str,
    compare_state: Option<CompareToDiskState>,
    rerun_posture: ReplayRerunPosture,
    discloses_modified: bool,
) -> ReplayPathObservation {
    let mutates = replay_kind.mutates_workspace();
    ReplayPathObservation {
        replay_path_id: replay_id.to_owned(),
        label: replay_id.to_owned(),
        replay_path_kind: replay_kind,
        packet_id: packet_id.to_owned(),
        compare_to_disk_state: compare_state,
        discloses_disk_modified_state: discloses_modified,
        rerun_posture,
        commit_action_id: if mutates {
            format!("action.{replay_id}.commit")
        } else {
            String::new()
        },
        commit_disclosure_id: if mutates {
            format!("disclosure.{replay_id}.commit")
        } else {
            String::new()
        },
        preserves_encoding_fidelity: true,
        preserves_restore_provenance: true,
        verifies_integrity_hash: true,
        support_export: make_support_export(),
        captured_at: "mono:1700000700".to_owned(),
    }
}

fn baseline_inputs() -> LocalHistoryExportReplayInputs {
    let packets = vec![
        packet(
            "entry.0",
            ExportPacketKind::LocalHistoryEntryExport,
            BodyAvailabilityClass::MetadataOnly,
            None,
            EncodingFidelityClass::Utf8Lf,
        ),
        packet(
            "group.0",
            ExportPacketKind::LocalHistoryGroupExport,
            BodyAvailabilityClass::MetadataOnly,
            None,
            EncodingFidelityClass::Utf8Lf,
        ),
        packet(
            "restore.0",
            ExportPacketKind::RestoreCheckpointExport,
            BodyAvailabilityClass::MetadataOnly,
            None,
            EncodingFidelityClass::Utf8Crlf,
        ),
        packet(
            "compare.0",
            ExportPacketKind::CompareToDiskDiffExport,
            BodyAvailabilityClass::MetadataOnly,
            None,
            EncodingFidelityClass::Utf8Lf,
        ),
        packet(
            "support.0",
            ExportPacketKind::SupportBundleLocalHistorySection,
            BodyAvailabilityClass::MetadataOnly,
            None,
            EncodingFidelityClass::Utf8Lf,
        ),
    ];
    let replay_paths = vec![
        replay_path(
            "replay.restore_from_packet",
            ReplayPathKind::RestoreFromPacket,
            "entry.0",
            None,
            ReplayRerunPosture::ExplicitUserActionRequired,
            true,
        ),
        replay_path(
            "replay.compare_to_disk",
            ReplayPathKind::CompareToDiskReplay,
            "compare.0",
            Some(CompareToDiskState::InSyncWithPacket),
            ReplayRerunPosture::TerminalNoFurtherRun,
            true,
        ),
        replay_path(
            "replay.entry_inspect",
            ReplayPathKind::EntryInspectReplay,
            "entry.0",
            None,
            ReplayRerunPosture::TerminalNoFurtherRun,
            true,
        ),
        replay_path(
            "replay.group_inspect",
            ReplayPathKind::GroupInspectReplay,
            "group.0",
            None,
            ReplayRerunPosture::TerminalNoFurtherRun,
            true,
        ),
        replay_path(
            "replay.support_bundle",
            ReplayPathKind::SupportBundleReplay,
            "support.0",
            None,
            ReplayRerunPosture::TerminalNoFurtherRun,
            true,
        ),
    ];

    LocalHistoryExportReplayInputs {
        workspace_ref: "workspace-local-history-0001".to_owned(),
        producer_ref: "producer-aureline-0001".to_owned(),
        corpus_ref: "local-history-export-replay-corpus-0001".to_owned(),
        captured_at: "mono:1700000700".to_owned(),
        packets,
        replay_paths,
    }
}

#[test]
fn clean_inputs_project_stable_record() {
    let inputs = baseline_inputs();
    let record = project_local_history_export_replay_lineage("posture.clean", &inputs);
    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(record.is_support_export_safe());
    assert_eq!(
        record.record_kind,
        LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_RECORD_KIND
    );
    assert_eq!(
        record.schema_ref,
        LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_SCHEMA_REF
    );
    assert!(record.packet_coverage.all_required_packet_kinds_present);
    assert!(
        record
            .replay_path_coverage
            .all_required_replay_path_kinds_present
    );
    assert!(record.compare_to_disk_honesty.all_compare_paths_have_state);
    assert!(
        record
            .compare_to_disk_honesty
            .no_disk_modified_silently_clean
    );
    assert!(record.body_export_safety.all_overrides_have_disclosure);
    assert!(record.body_export_safety.no_raw_body_by_default);
    assert!(record.encoding_fidelity.all_packets_preserve_encoding);
    assert!(record.encoding_fidelity.all_packets_preserve_newline);
    assert!(record.encoding_fidelity.all_packets_preserve_bom);
    assert!(
        record
            .encoding_fidelity
            .all_replays_preserve_encoding_fidelity
    );
    assert!(record.restore_provenance.all_packets_carry_restore_of_ref);
    assert!(
        record
            .restore_provenance
            .all_packets_carry_mutation_journal_ref
    );
    assert!(record.restore_provenance.all_packets_carry_actor_class);
    assert!(
        record
            .restore_provenance
            .all_replays_preserve_restore_provenance
    );
    assert!(record.no_silent_rerun.all_replays_safe_rerun_posture);
    assert!(
        record
            .no_silent_rerun
            .all_mutating_replays_have_commit_metadata
    );
    assert!(record.integrity_hash_pinning.all_packets_pin_integrity_hash);
    assert!(
        record
            .integrity_hash_pinning
            .all_replays_verify_integrity_hash
    );
    assert_eq!(record.inspection_hooks.len(), 6);
    assert!(record
        .producer_attribution
        .integrity_hash
        .starts_with("lher:"));
}

#[test]
fn missing_required_packet_kind_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs
        .packets
        .retain(|p| p.packet_kind != ExportPacketKind::CompareToDiskDiffExport);
    let record =
        project_local_history_export_replay_lineage("posture.missing_compare_packet", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::RequiredPacketKindMissing));
}

#[test]
fn missing_required_replay_path_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs
        .replay_paths
        .retain(|r| r.replay_path_kind != ReplayPathKind::CompareToDiskReplay);
    let record =
        project_local_history_export_replay_lineage("posture.missing_compare_replay", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::RequiredReplayPathKindMissing));
}

#[test]
fn replay_referencing_unknown_packet_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.replay_paths[0].packet_id = "nonexistent".to_owned();
    let record = project_local_history_export_replay_lineage("posture.unknown_packet", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::ReplayReferencesUnknownPacket));
}

#[test]
fn compare_path_without_state_narrows_record() {
    let mut inputs = baseline_inputs();
    let compare = inputs
        .replay_paths
        .iter_mut()
        .find(|r| r.replay_path_kind == ReplayPathKind::CompareToDiskReplay)
        .expect("seeded");
    compare.compare_to_disk_state = None;
    let record = project_local_history_export_replay_lineage("posture.no_state", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::CompareToDiskStateMissing));
}

#[test]
fn disk_modified_silent_narrows_record() {
    let mut inputs = baseline_inputs();
    let compare = inputs
        .replay_paths
        .iter_mut()
        .find(|r| r.replay_path_kind == ReplayPathKind::CompareToDiskReplay)
        .expect("seeded");
    compare.compare_to_disk_state = Some(CompareToDiskState::DiskModifiedSincePacket);
    compare.discloses_disk_modified_state = false;
    let record =
        project_local_history_export_replay_lineage("posture.disk_modified_silent", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record.stable_qualification.narrow_reasons.contains(
        &LocalHistoryExportReplayLineageNarrowReason::DiskModifiedSilentlyTreatedAsClean
    ));
}

#[test]
fn body_override_without_disclosure_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.packets[0].body_availability_class = BodyAvailabilityClass::BodyObjectRefWithDisclosure;
    inputs.packets[0].body_override_disclosure_ref = None;
    let record = project_local_history_export_replay_lineage("posture.no_body_override", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::BodyOverrideDisclosureMissing));
}

#[test]
fn body_raw_with_disclosure_stays_stable_when_disclosure_present() {
    let mut inputs = baseline_inputs();
    inputs.packets[0].body_availability_class = BodyAvailabilityClass::RawBodyWithDisclosure;
    inputs.packets[0].body_override_disclosure_ref = Some("disclosure.raw_body.entry.0".to_owned());
    let record = project_local_history_export_replay_lineage("posture.raw_body_safe", &inputs);
    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
}

#[test]
fn replay_silent_rerun_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.replay_paths[0].rerun_posture = ReplayRerunPosture::SilentRerunPermitted;
    let record = project_local_history_export_replay_lineage("posture.silent_rerun", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::ReplayRerunSilentForbidden));
}

#[test]
fn mutating_replay_missing_commit_action_narrows_record() {
    let mut inputs = baseline_inputs();
    let restore = inputs
        .replay_paths
        .iter_mut()
        .find(|r| r.replay_path_kind == ReplayPathKind::RestoreFromPacket)
        .expect("seeded");
    restore.commit_action_id = String::new();
    let record = project_local_history_export_replay_lineage("posture.no_commit_action", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::ReplayCommitActionMetadataMissing));
}

#[test]
fn encoding_not_preserved_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.packets[0].encoding_preserved = false;
    let record = project_local_history_export_replay_lineage("posture.no_encoding", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::EncodingFidelityNotPreserved));
}

#[test]
fn newline_not_preserved_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.packets[0].newline_preserved = false;
    let record = project_local_history_export_replay_lineage("posture.no_newline", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::EncodingFidelityNotPreserved));
}

#[test]
fn replay_dropping_encoding_fidelity_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.replay_paths[0].preserves_encoding_fidelity = false;
    let record = project_local_history_export_replay_lineage("posture.no_replay_encoding", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::EncodingFidelityNotPreserved));
}

#[test]
fn empty_restore_of_ref_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.packets[0].restore_of_ref = String::new();
    let record = project_local_history_export_replay_lineage("posture.no_restore_of", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::RestoreProvenanceNotPreserved));
}

#[test]
fn replay_dropping_provenance_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.replay_paths[0].preserves_restore_provenance = false;
    let record =
        project_local_history_export_replay_lineage("posture.no_replay_provenance", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::RestoreProvenanceNotPreserved));
}

#[test]
fn empty_integrity_hash_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.packets[0].integrity_hash = String::new();
    let record = project_local_history_export_replay_lineage("posture.no_integrity", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::IntegrityHashNotPinned));
}

#[test]
fn replay_not_verifying_integrity_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.replay_paths[0].verifies_integrity_hash = false;
    let record =
        project_local_history_export_replay_lineage("posture.no_replay_integrity", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::IntegrityHashNotPinned));
}

#[test]
fn missing_inspection_hook_narrows_record() {
    let inputs = baseline_inputs();
    let mut hooks = default_local_history_export_replay_inspection_hooks();
    for hook in &mut hooks {
        if hook.hook_class == LocalHistoryExportReplayInspectionHookClass::CompareBeforeReplay {
            hook.available = false;
        }
    }
    let record = project_local_history_export_replay_lineage_with_hooks(
        "posture.no_compare_hook",
        &inputs,
        hooks,
    );
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::InspectionHookUnavailable));
}

#[test]
fn support_export_dropping_field_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.packets[0].support_export.includes_integrity_hash = false;
    let record = project_local_history_export_replay_lineage("posture.support_dropped", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::SupportExportFieldsDropped));
}

#[test]
fn support_export_raw_body_leak_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.packets[0].support_export.raw_body_bytes_excluded = false;
    let record = project_local_history_export_replay_lineage("posture.support_raw_body", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::SupportExportRedactionUnsafe));
}

#[test]
fn empty_workspace_ref_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.workspace_ref = String::new();
    let record = project_local_history_export_replay_lineage("posture.no_workspace", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::LineageExportUnsafe));
}

#[test]
fn empty_corpus_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.packets.clear();
    inputs.replay_paths.clear();
    let record = project_local_history_export_replay_lineage("posture.empty_corpus", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::CorpusEmpty));
}

#[test]
fn producer_attribution_incomplete_narrows_record() {
    let mut inputs = baseline_inputs();
    inputs.producer_ref = String::new();
    let record = project_local_history_export_replay_lineage("posture.no_producer", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&LocalHistoryExportReplayLineageNarrowReason::ProducerAttributionIncomplete));
}

#[test]
fn lines_projection_renders_required_sections() {
    let inputs = baseline_inputs();
    let record = project_local_history_export_replay_lineage("posture.lines", &inputs);
    let lines = local_history_export_replay_lineage_lines(&record);
    assert!(lines
        .iter()
        .any(|line| line.contains("Local-history export/replay lineage")));
    assert!(lines.iter().any(|line| line.contains("packet_coverage")));
    assert!(lines.iter().any(|line| line == "Packets:"));
    assert!(lines
        .iter()
        .any(|line| line.contains("replay_path_coverage")));
    assert!(lines.iter().any(|line| line == "Replay paths:"));
    assert!(lines
        .iter()
        .any(|line| line.contains("Compare-to-disk honesty")));
    assert!(lines.iter().any(|line| line.contains("Body-export safety")));
    assert!(lines.iter().any(|line| line.contains("Encoding fidelity")));
    assert!(lines.iter().any(|line| line.contains("Restore provenance")));
    assert!(lines.iter().any(|line| line.contains("No-silent-rerun")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Integrity-hash pinning")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Support-export honesty")));
    assert!(lines.iter().any(|line| line == "Inspection hooks:"));
}

#[test]
fn record_round_trips_through_json() {
    let inputs = baseline_inputs();
    let record = project_local_history_export_replay_lineage("posture.round_trip", &inputs);
    let serialized = serde_json::to_string(&record).expect("record must serialize");
    let parsed: LocalHistoryExportReplayLineageRecord =
        serde_json::from_str(&serialized).expect("record must deserialize");
    assert_eq!(record, parsed);
}
