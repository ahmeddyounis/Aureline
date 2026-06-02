//! Fixture generator helper for the local-history export/replay
//! lineage replay gate.
//!
//! Only runs when `LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_GEN_FIXTURES=1`
//! is set in the environment. Emits the canonical fixture JSON files
//! into `fixtures/workspace/m4/local_history_export_replay_lineage/`
//! so the replay gate has a deterministic, checked-in corpus.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    default_local_history_export_replay_inspection_hooks,
    project_local_history_export_replay_lineage_with_hooks, BodyAvailabilityClass,
    CompareToDiskState, EncodingFidelityClass, ExportPacketKind, LocalHistoryExportReplayInputs,
    LocalHistoryExportReplayInspectionHook, LocalHistoryExportReplayInspectionHookClass,
    LocalHistoryExportReplayLineageRecord, LocalHistoryExportReplaySupportExportInputs,
    LocalHistoryExportReplaySupportExportPosture, PacketObservation, ReplayPathKind,
    ReplayPathObservation, ReplayRerunPosture,
};
use serde::Serialize;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/local_history_export_replay_lineage")
}

fn support() -> LocalHistoryExportReplaySupportExportInputs {
    LocalHistoryExportReplaySupportExportInputs::metadata_safe_baseline(
        LocalHistoryExportReplaySupportExportPosture::MetadataSafeExport,
    )
}

#[allow(clippy::too_many_arguments)]
fn make_packet(
    packet_id: &str,
    kind: ExportPacketKind,
    body_class: BodyAvailabilityClass,
    body_override_disclosure_ref: Option<&str>,
    encoding: EncodingFidelityClass,
    restore_of_ref: &str,
    mutation_journal_ref: &str,
    actor_class: &str,
    integrity_hash: &str,
    captured_at: &str,
) -> PacketObservation {
    PacketObservation {
        packet_id: packet_id.to_owned(),
        packet_kind: kind,
        packet_ref: format!("pkt:{packet_id}"),
        body_availability_class: body_class,
        body_override_disclosure_ref: body_override_disclosure_ref.map(str::to_owned),
        encoding_fidelity_class: encoding,
        encoding_preserved: true,
        newline_preserved: true,
        bom_preserved: true,
        restore_of_ref: restore_of_ref.to_owned(),
        mutation_journal_ref: mutation_journal_ref.to_owned(),
        actor_class: actor_class.to_owned(),
        integrity_hash: integrity_hash.to_owned(),
        support_export: support(),
        captured_at: captured_at.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn make_replay(
    replay_id: &str,
    label: &str,
    kind: ReplayPathKind,
    packet_id: &str,
    compare_state: Option<CompareToDiskState>,
    discloses_modified: bool,
    rerun_posture: ReplayRerunPosture,
    captured_at: &str,
) -> ReplayPathObservation {
    let mutates = kind.mutates_workspace();
    ReplayPathObservation {
        replay_path_id: replay_id.to_owned(),
        label: label.to_owned(),
        replay_path_kind: kind,
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
        support_export: support(),
        captured_at: captured_at.to_owned(),
    }
}

fn baseline_packets(captured_at: &str) -> Vec<PacketObservation> {
    vec![
        make_packet(
            "entry.editor_save.0",
            ExportPacketKind::LocalHistoryEntryExport,
            BodyAvailabilityClass::MetadataOnly,
            None,
            EncodingFidelityClass::Utf8Lf,
            "restore-of:entry.editor_save.0",
            "mj:entry.editor_save.0",
            "user_keystroke",
            "blake3:entry-editor-save-0",
            captured_at,
        ),
        make_packet(
            "group.formatter_save.0",
            ExportPacketKind::LocalHistoryGroupExport,
            BodyAvailabilityClass::MetadataOnly,
            None,
            EncodingFidelityClass::Utf8Lf,
            "restore-of:group.formatter_save.0",
            "mj:group.formatter_save.0",
            "formatter",
            "blake3:group-formatter-save-0",
            captured_at,
        ),
        make_packet(
            "restore.checkpoint.crlf.0",
            ExportPacketKind::RestoreCheckpointExport,
            BodyAvailabilityClass::MetadataOnly,
            None,
            EncodingFidelityClass::Utf8Crlf,
            "restore-of:restore.checkpoint.crlf.0",
            "mj:restore.checkpoint.crlf.0",
            "restore_rollback_runner",
            "blake3:restore-checkpoint-0",
            captured_at,
        ),
        make_packet(
            "compare.diff.in_sync.0",
            ExportPacketKind::CompareToDiskDiffExport,
            BodyAvailabilityClass::MetadataOnly,
            None,
            EncodingFidelityClass::Utf8Lf,
            "restore-of:compare.diff.in_sync.0",
            "mj:compare.diff.in_sync.0",
            "user_keystroke",
            "blake3:compare-diff-0",
            captured_at,
        ),
        make_packet(
            "support.bundle.section.0",
            ExportPacketKind::SupportBundleLocalHistorySection,
            BodyAvailabilityClass::MetadataOnly,
            None,
            EncodingFidelityClass::Utf8Lf,
            "restore-of:support.bundle.section.0",
            "mj:support.bundle.section.0",
            "support_export_collector",
            "blake3:support-bundle-section-0",
            captured_at,
        ),
    ]
}

fn baseline_replay_paths(captured_at: &str) -> Vec<ReplayPathObservation> {
    vec![
        make_replay(
            "replay.restore_from_packet.entry.0",
            "Restore buffer from entry packet",
            ReplayPathKind::RestoreFromPacket,
            "entry.editor_save.0",
            None,
            true,
            ReplayRerunPosture::ExplicitUserActionRequired,
            captured_at,
        ),
        make_replay(
            "replay.compare_to_disk.entry.0",
            "Compare entry packet to disk",
            ReplayPathKind::CompareToDiskReplay,
            "compare.diff.in_sync.0",
            Some(CompareToDiskState::InSyncWithPacket),
            true,
            ReplayRerunPosture::TerminalNoFurtherRun,
            captured_at,
        ),
        make_replay(
            "replay.entry_inspect.0",
            "Inspect entry packet",
            ReplayPathKind::EntryInspectReplay,
            "entry.editor_save.0",
            None,
            true,
            ReplayRerunPosture::TerminalNoFurtherRun,
            captured_at,
        ),
        make_replay(
            "replay.group_inspect.0",
            "Inspect group packet",
            ReplayPathKind::GroupInspectReplay,
            "group.formatter_save.0",
            None,
            true,
            ReplayRerunPosture::TerminalNoFurtherRun,
            captured_at,
        ),
        make_replay(
            "replay.support_bundle.0",
            "Replay support bundle section",
            ReplayPathKind::SupportBundleReplay,
            "support.bundle.section.0",
            None,
            true,
            ReplayRerunPosture::TerminalNoFurtherRun,
            captured_at,
        ),
    ]
}

fn base_inputs(
    workspace_ref: &str,
    corpus_ref: &str,
    captured_at: &str,
    packets: Vec<PacketObservation>,
    replay_paths: Vec<ReplayPathObservation>,
) -> LocalHistoryExportReplayInputs {
    LocalHistoryExportReplayInputs {
        workspace_ref: workspace_ref.to_owned(),
        producer_ref: "producer-aureline-fixtures-0001".to_owned(),
        corpus_ref: corpus_ref.to_owned(),
        captured_at: captured_at.to_owned(),
        packets,
        replay_paths,
    }
}

#[derive(Debug, Serialize)]
struct FixtureEnvelope<'a> {
    posture_id: &'a str,
    inputs: &'a LocalHistoryExportReplayInputs,
    inspection_hooks: &'a Vec<LocalHistoryExportReplayInspectionHook>,
    expected: &'a LocalHistoryExportReplayLineageRecord,
}

fn write_fixture(
    name: &str,
    posture_id: &str,
    inputs: LocalHistoryExportReplayInputs,
    inspection_hooks: Vec<LocalHistoryExportReplayInspectionHook>,
) {
    let record = project_local_history_export_replay_lineage_with_hooks(
        posture_id,
        &inputs,
        inspection_hooks.clone(),
    );
    let envelope = FixtureEnvelope {
        posture_id,
        inputs: &inputs,
        inspection_hooks: &inspection_hooks,
        expected: &record,
    };
    let path = fixtures_dir().join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(&envelope).expect("envelope serializes");
    std::fs::write(&path, json + "\n").expect("fixture write");
    eprintln!("wrote {}", path.display());
}

#[test]
fn generate_fixtures() {
    if std::env::var("LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_GEN_FIXTURES")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    std::fs::create_dir_all(fixtures_dir()).expect("ensure fixture dir");

    // Baseline Stable: every required packet kind + every required
    // replay path, in-sync compare-to-disk, metadata-only bodies.
    write_fixture(
        "baseline_local_history_export_replay_stable",
        "posture:baseline_local_history_export_replay",
        base_inputs(
            "workspace-rust-service-0001",
            "local-history-export-replay-corpus-baseline-0001",
            "mono:1700000700",
            baseline_packets("mono:1700000700"),
            baseline_replay_paths("mono:1700000700"),
        ),
        default_local_history_export_replay_inspection_hooks(),
    );

    // Extended Stable: adds a raw-body packet with an explicit
    // override disclosure ref and a disk_modified_since_packet
    // compare-to-disk replay path that discloses the modification.
    let mut extended_packets = baseline_packets("mono:1700000710");
    extended_packets.push(make_packet(
        "entry.raw_body_override.0",
        ExportPacketKind::LocalHistoryEntryExport,
        BodyAvailabilityClass::RawBodyWithDisclosure,
        Some("disclosure.entry.raw_body_override.0"),
        EncodingFidelityClass::Utf8Lf,
        "restore-of:entry.raw_body_override.0",
        "mj:entry.raw_body_override.0",
        "user_keystroke",
        "blake3:entry-raw-body-override-0",
        "mono:1700000710",
    ));
    let mut extended_replays = baseline_replay_paths("mono:1700000710");
    if let Some(compare) = extended_replays
        .iter_mut()
        .find(|r| r.replay_path_kind == ReplayPathKind::CompareToDiskReplay)
    {
        compare.compare_to_disk_state = Some(CompareToDiskState::DiskModifiedSincePacket);
        compare.discloses_disk_modified_state = true;
    }
    extended_replays.push(make_replay(
        "replay.compare_to_disk.recovered.0",
        "Compare decoded-recovered packet to disk",
        ReplayPathKind::CompareToDiskReplay,
        "entry.raw_body_override.0",
        Some(CompareToDiskState::PacketDecodedRecovered),
        true,
        ReplayRerunPosture::TerminalNoFurtherRun,
        "mono:1700000710",
    ));
    write_fixture(
        "extended_disk_modified_disclosed_stable",
        "posture:extended_disk_modified_disclosed",
        base_inputs(
            "workspace-rust-service-0001",
            "local-history-export-replay-corpus-extended-0001",
            "mono:1700000710",
            extended_packets,
            extended_replays,
        ),
        default_local_history_export_replay_inspection_hooks(),
    );

    // Narrowed: a compare-to-disk replay reports disk_modified_since_packet
    // but does not disclose the modification — must narrow with
    // `disk_modified_silently_treated_as_clean`.
    let mut silent_replays = baseline_replay_paths("mono:1700000720");
    if let Some(compare) = silent_replays
        .iter_mut()
        .find(|r| r.replay_path_kind == ReplayPathKind::CompareToDiskReplay)
    {
        compare.compare_to_disk_state = Some(CompareToDiskState::DiskModifiedSincePacket);
        compare.discloses_disk_modified_state = false;
    }
    write_fixture(
        "disk_modified_silent_narrowed",
        "posture:disk_modified_silent",
        base_inputs(
            "workspace-rust-service-0001",
            "local-history-export-replay-corpus-disk-modified-silent-0001",
            "mono:1700000720",
            baseline_packets("mono:1700000720"),
            silent_replays,
        ),
        default_local_history_export_replay_inspection_hooks(),
    );

    // Narrowed: a restore-from-packet replay declares
    // `silent_rerun_permitted` (forbidden on Stable rows).
    let mut silent_rerun_replays = baseline_replay_paths("mono:1700000730");
    if let Some(restore) = silent_rerun_replays
        .iter_mut()
        .find(|r| r.replay_path_kind == ReplayPathKind::RestoreFromPacket)
    {
        restore.rerun_posture = ReplayRerunPosture::SilentRerunPermitted;
    }
    write_fixture(
        "restore_silent_rerun_narrowed",
        "posture:restore_silent_rerun",
        base_inputs(
            "workspace-rust-service-0001",
            "local-history-export-replay-corpus-silent-rerun-0001",
            "mono:1700000730",
            baseline_packets("mono:1700000730"),
            silent_rerun_replays,
        ),
        default_local_history_export_replay_inspection_hooks(),
    );

    // Narrowed: required `compare_before_replay` inspection hook is
    // unavailable on this posture.
    let narrowed_inputs = base_inputs(
        "workspace-rust-service-0001",
        "local-history-export-replay-corpus-narrowed-hook-0001",
        "mono:1700000740",
        baseline_packets("mono:1700000740"),
        baseline_replay_paths("mono:1700000740"),
    );
    let mut narrowed_hooks = default_local_history_export_replay_inspection_hooks();
    for hook in &mut narrowed_hooks {
        if hook.hook_class == LocalHistoryExportReplayInspectionHookClass::CompareBeforeReplay {
            hook.available = false;
            hook.disclosure = "Compare-before-replay unavailable on this posture.".to_owned();
        }
    }
    write_fixture(
        "missing_compare_before_replay_hook_narrowed",
        "posture:missing_compare_before_replay_hook",
        narrowed_inputs,
        narrowed_hooks,
    );
}
