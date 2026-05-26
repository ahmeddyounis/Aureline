//! Local-history export/replay packet and compare-to-disk lineage:
//! the governed, export-safe projection that proves how local-history
//! export packets and their replay/compare-to-disk flows preserve
//! source fidelity, restore provenance, and no-rerun semantics across
//! crashes, migrations, external changes, and policy-bound workflows.
//!
//! The projection ingests a live [`LocalHistoryExportReplayInputs`]
//! envelope verbatim (one [`PacketObservation`] per governed export
//! packet, one [`ReplayPathObservation`] per replay/compare-to-disk
//! path, plus the controlled inspection-hook table) and produces a
//! lineage record that proves the contract claims the stable line is
//! anchored on:
//!
//! - **Packet-kind coverage truth.** Every governed packet kind
//!   (`local_history_entry_export`, `local_history_group_export`,
//!   `restore_checkpoint_export`, `compare_to_disk_diff_export`,
//!   `support_bundle_local_history_section`) ships a row bound to one
//!   closed [`ExportPacketKind`]; the optional replay envelopes ride
//!   on top without changing the required set.
//! - **Replay-path coverage truth.** Every required replay path
//!   (`restore_from_packet`, `compare_to_disk_replay`,
//!   `entry_inspect_replay`, `group_inspect_replay`,
//!   `support_bundle_replay`) ships a row bound to one closed
//!   [`ReplayPathKind`].
//! - **Compare-to-disk honesty.** Every compare-to-disk replay path
//!   distinguishes one closed [`CompareToDiskState`] and never
//!   silently treats `disk_modified_since_packet` as a clean match.
//! - **Body-export safety.** Every packet declares one closed
//!   [`BodyAvailabilityClass`] and the default posture is
//!   `metadata_only`; `raw_body_with_disclosure` requires an explicit
//!   override disclosure.
//! - **Encoding/newline fidelity.** Every packet preserves the source
//!   encoding, newline mode, and BOM posture so a replay that lands
//!   on disk reproduces the exact bytes the source captured.
//! - **Restore-provenance preservation.** Every packet preserves a
//!   non-empty restore-of ref, mutation-journal ref, and actor-class
//!   token so a replay never severs the lineage chain.
//! - **No-silent-rerun honesty.** Every replay path declares
//!   `explicit_user_action_required` or `terminal_no_further_run`
//!   with both a commit action id and a commit disclosure id; a
//!   `silent_rerun_permitted` posture is forbidden on Stable rows.
//! - **Integrity-hash pinning.** Every packet pins a non-empty
//!   integrity hash that replay/compare-to-disk paths can verify
//!   before applying.
//! - **Pre-action inspection-hook honesty.** A controlled set of
//!   pre-action inspection / repair hooks (`inspect_packet`,
//!   `compare_before_replay`, `preview_replay`, `export_packet`,
//!   `rollback_replay`, `repair_packet`) is reachable before any
//!   destructive replay/cleanup commits.
//! - **Support-export honesty.** Each row's support-export projection
//!   preserves the packet kind, replay path class, packet ref,
//!   compare-to-disk class, body-availability class, declared
//!   encoding/newline, restore-of ref, mutation-journal ref, and
//!   integrity hash while excluding raw secrets, raw body bytes,
//!   approval tickets, delegated credentials, and live authority
//!   handles.
//! - **Producer attribution.** Each record carries the producer ref,
//!   the schema version, the capture timestamp, and an integrity
//!   hash derived from the input identities so replay and support
//!   pipelines can pin the source before applying.
//! - **Lineage and export honesty.** The record sets
//!   `raw_payload_excluded = true` and carries only opaque refs to
//!   the source workspace, corpus, and producer.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for [`LocalHistoryExportReplayLineageRecord`].
pub const LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the local-history export/replay lineage record.
pub const LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/local_history_export_replay_lineage.schema.json";

/// Stable record-kind tag for the local-history export/replay lineage record.
pub const LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_RECORD_KIND: &str =
    "local_history_export_replay_lineage_record";

// ---------------------------------------------------------------------------
// Closed vocabularies.
// ---------------------------------------------------------------------------

/// Closed vocabulary for the local-history export packet kinds governed
/// by this lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportPacketKind {
    /// One local-history entry exported as a metadata-safe packet.
    LocalHistoryEntryExport,
    /// A local-history group exported as a metadata-safe packet.
    LocalHistoryGroupExport,
    /// A named restore checkpoint exported as a metadata-safe packet.
    RestoreCheckpointExport,
    /// A compare-to-disk diff packet pinned for inspection or replay.
    CompareToDiskDiffExport,
    /// The local-history section of a support bundle.
    SupportBundleLocalHistorySection,
    /// Optional input envelope used to drive an external replay.
    ReplayInputEnvelope,
    /// Optional output envelope captured by a replay run.
    ReplayOutputEnvelope,
}

impl ExportPacketKind {
    /// Returns the stable snake_case token for this packet kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalHistoryEntryExport => "local_history_entry_export",
            Self::LocalHistoryGroupExport => "local_history_group_export",
            Self::RestoreCheckpointExport => "restore_checkpoint_export",
            Self::CompareToDiskDiffExport => "compare_to_disk_diff_export",
            Self::SupportBundleLocalHistorySection => "support_bundle_local_history_section",
            Self::ReplayInputEnvelope => "replay_input_envelope",
            Self::ReplayOutputEnvelope => "replay_output_envelope",
        }
    }

    /// True when this packet kind is part of the required set every
    /// Stable corpus must seed.
    pub const fn is_required(self) -> bool {
        matches!(
            self,
            Self::LocalHistoryEntryExport
                | Self::LocalHistoryGroupExport
                | Self::RestoreCheckpointExport
                | Self::CompareToDiskDiffExport
                | Self::SupportBundleLocalHistorySection
        )
    }
}

/// Closed list of packet kinds every lineage record must seed.
pub const REQUIRED_EXPORT_PACKET_KINDS: [ExportPacketKind; 5] = [
    ExportPacketKind::LocalHistoryEntryExport,
    ExportPacketKind::LocalHistoryGroupExport,
    ExportPacketKind::RestoreCheckpointExport,
    ExportPacketKind::CompareToDiskDiffExport,
    ExportPacketKind::SupportBundleLocalHistorySection,
];

/// Closed vocabulary for the replay/compare-to-disk path kinds
/// governed by this lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayPathKind {
    /// Restore a buffer or workspace from an exported packet.
    RestoreFromPacket,
    /// Compare a pinned packet to the current on-disk contents.
    CompareToDiskReplay,
    /// Inspect a single entry packet (read-only).
    EntryInspectReplay,
    /// Inspect a group packet (read-only).
    GroupInspectReplay,
    /// Replay the local-history section of a support bundle for
    /// diagnostic inspection.
    SupportBundleReplay,
}

impl ReplayPathKind {
    /// Returns the stable snake_case token for this replay path kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreFromPacket => "restore_from_packet",
            Self::CompareToDiskReplay => "compare_to_disk_replay",
            Self::EntryInspectReplay => "entry_inspect_replay",
            Self::GroupInspectReplay => "group_inspect_replay",
            Self::SupportBundleReplay => "support_bundle_replay",
        }
    }

    /// True when this replay path mutates buffers or workspace state
    /// (and therefore must declare an explicit no-silent-rerun posture
    /// with both a commit action id and a commit disclosure id).
    pub const fn mutates_workspace(self) -> bool {
        matches!(self, Self::RestoreFromPacket)
    }

    /// True when this replay path performs a compare-to-disk
    /// comparison (and therefore must declare a non-empty
    /// compare-to-disk state).
    pub const fn is_compare_to_disk(self) -> bool {
        matches!(self, Self::CompareToDiskReplay)
    }
}

/// Closed list of replay path kinds every lineage record must seed.
pub const REQUIRED_REPLAY_PATH_KINDS: [ReplayPathKind; 5] = [
    ReplayPathKind::RestoreFromPacket,
    ReplayPathKind::CompareToDiskReplay,
    ReplayPathKind::EntryInspectReplay,
    ReplayPathKind::GroupInspectReplay,
    ReplayPathKind::SupportBundleReplay,
];

/// Closed vocabulary for the state a compare-to-disk replay reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareToDiskState {
    /// The packet matches the on-disk bytes exactly.
    InSyncWithPacket,
    /// The disk content has changed since the packet was minted.
    DiskModifiedSincePacket,
    /// The packet was decoded via a recovery path and is annotated as
    /// such so consumers know the bytes are not lossless.
    PacketDecodedRecovered,
    /// The packet redacts the body and only carries metadata; the
    /// compare-to-disk surface must say so rather than silently
    /// signaling a mismatch.
    PacketRedacted,
    /// Compare-to-disk is unavailable because the source lives only
    /// on a remote workspace.
    CompareUnavailableRemoteOnly,
    /// The packet is local-only (no canonical disk file to compare
    /// against, e.g. an untitled buffer).
    LocalOnlyPacket,
}

impl CompareToDiskState {
    /// Returns the stable snake_case token for this compare-to-disk
    /// state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSyncWithPacket => "in_sync_with_packet",
            Self::DiskModifiedSincePacket => "disk_modified_since_packet",
            Self::PacketDecodedRecovered => "packet_decoded_recovered",
            Self::PacketRedacted => "packet_redacted",
            Self::CompareUnavailableRemoteOnly => "compare_unavailable_remote_only",
            Self::LocalOnlyPacket => "local_only_packet",
        }
    }

    /// True when this state must be disclosed (rather than silently
    /// treated as a clean match) before any destructive replay.
    pub const fn requires_user_disclosure(self) -> bool {
        matches!(
            self,
            Self::DiskModifiedSincePacket
                | Self::PacketDecodedRecovered
                | Self::PacketRedacted
                | Self::CompareUnavailableRemoteOnly
        )
    }
}

/// Closed vocabulary for the body-availability posture of a packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyAvailabilityClass {
    /// Only metadata-safe fields ship; no body bytes, no object refs.
    MetadataOnly,
    /// A content-addressed body object ref ships with an explicit
    /// override disclosure.
    BodyObjectRefWithDisclosure,
    /// Raw body bytes ship with an explicit override disclosure (only
    /// from a high-friction override path).
    RawBodyWithDisclosure,
    /// The body is intentionally excluded by policy (redaction,
    /// trust, license).
    BodyExcludedByPolicy,
}

impl BodyAvailabilityClass {
    /// Returns the stable snake_case token for this body class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::BodyObjectRefWithDisclosure => "body_object_ref_with_disclosure",
            Self::RawBodyWithDisclosure => "raw_body_with_disclosure",
            Self::BodyExcludedByPolicy => "body_excluded_by_policy",
        }
    }

    /// True when this class requires an explicit override disclosure
    /// ref before being shipped.
    pub const fn requires_override_disclosure(self) -> bool {
        matches!(
            self,
            Self::BodyObjectRefWithDisclosure | Self::RawBodyWithDisclosure
        )
    }

    /// True when this class is safe by default (no override needed).
    pub const fn is_default_safe(self) -> bool {
        matches!(self, Self::MetadataOnly | Self::BodyExcludedByPolicy)
    }
}

/// Closed vocabulary for the no-rerun posture a replay path declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayRerunPosture {
    /// The replay path requires an explicit user commit action before
    /// it can run.
    ExplicitUserActionRequired,
    /// The replay path is terminal: it does not re-fire after the
    /// captured run.
    TerminalNoFurtherRun,
    /// The replay path may re-fire silently — forbidden on Stable.
    SilentRerunPermitted,
}

impl ReplayRerunPosture {
    /// Returns the stable snake_case token for this rerun posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitUserActionRequired => "explicit_user_action_required",
            Self::TerminalNoFurtherRun => "terminal_no_further_run",
            Self::SilentRerunPermitted => "silent_rerun_permitted",
        }
    }

    /// True when this posture is safe to ship on a Stable row.
    pub const fn safe_for_stable(self) -> bool {
        matches!(
            self,
            Self::ExplicitUserActionRequired | Self::TerminalNoFurtherRun
        )
    }
}

/// Closed vocabulary for the encoding/newline fidelity class a packet
/// preserves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EncodingFidelityClass {
    /// UTF-8 with LF newlines, no BOM.
    Utf8Lf,
    /// UTF-8 with CRLF newlines, no BOM.
    Utf8Crlf,
    /// UTF-8 with LF newlines and a UTF-8 BOM.
    Utf8BomLf,
    /// UTF-8 with CRLF newlines and a UTF-8 BOM.
    Utf8BomCrlf,
    /// A declared non-UTF-8 encoding (e.g. Windows-1252, Shift-JIS)
    /// preserved verbatim.
    NonUtf8DeclaredEncoding,
    /// A binary or large-file packet handled in binary-safe mode.
    BinarySafe,
}

impl EncodingFidelityClass {
    /// Returns the stable snake_case token for this encoding class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Utf8Lf => "utf8_lf",
            Self::Utf8Crlf => "utf8_crlf",
            Self::Utf8BomLf => "utf8_bom_lf",
            Self::Utf8BomCrlf => "utf8_bom_crlf",
            Self::NonUtf8DeclaredEncoding => "non_utf8_declared_encoding",
            Self::BinarySafe => "binary_safe",
        }
    }
}

/// Closed vocabulary for pre-action inspection / repair hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryExportReplayInspectionHookClass {
    /// Open the packet inspector with the packet's kind, packet ref,
    /// encoding/newline class, integrity hash, and body-availability
    /// class.
    InspectPacket,
    /// Compare the packet to the current on-disk contents before any
    /// destructive replay commits.
    CompareBeforeReplay,
    /// Preview the replay's effects (restored bytes, encoded line
    /// endings, restore-provenance changes) before the apply.
    PreviewReplay,
    /// Export the packet (support-safe) so the same replay can be
    /// driven elsewhere.
    ExportPacket,
    /// Roll a destructive replay back to the previous packet
    /// identity.
    RollbackReplay,
    /// Open the typed repair sheet for a damaged packet (rebuild
    /// integrity hash, refetch object body, re-mint compare-to-disk
    /// diff).
    RepairPacket,
}

impl LocalHistoryExportReplayInspectionHookClass {
    /// Returns the stable snake_case token for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectPacket => "inspect_packet",
            Self::CompareBeforeReplay => "compare_before_replay",
            Self::PreviewReplay => "preview_replay",
            Self::ExportPacket => "export_packet",
            Self::RollbackReplay => "rollback_replay",
            Self::RepairPacket => "repair_packet",
        }
    }
}

/// One pre-action inspection / repair hook offered before a
/// destructive replay / cleanup commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryExportReplayInspectionHook {
    /// Hook class.
    pub hook_class: LocalHistoryExportReplayInspectionHookClass,
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Whether the hook is reachable on this posture.
    pub available: bool,
    /// Disclosure shown when the hook is offered.
    pub disclosure: String,
}

/// Returns the default pre-action inspection / repair hook table.
pub fn default_local_history_export_replay_inspection_hooks(
) -> Vec<LocalHistoryExportReplayInspectionHook> {
    vec![
        LocalHistoryExportReplayInspectionHook {
            hook_class: LocalHistoryExportReplayInspectionHookClass::InspectPacket,
            action_id: "local_history_export_replay.inspect_packet".to_owned(),
            label: "Inspect packet".to_owned(),
            available: true,
            disclosure:
                "Opens the packet inspector with the packet's kind, ref, encoding/newline class, integrity hash, body-availability class, and restore provenance before any replay commits."
                    .to_owned(),
        },
        LocalHistoryExportReplayInspectionHook {
            hook_class: LocalHistoryExportReplayInspectionHookClass::CompareBeforeReplay,
            action_id: "local_history_export_replay.compare_before_replay".to_owned(),
            label: "Compare packet to disk".to_owned(),
            available: true,
            disclosure:
                "Renders the typed compare-to-disk view between the pinned packet and the current on-disk bytes so the user can review modifications before any replay applies."
                    .to_owned(),
        },
        LocalHistoryExportReplayInspectionHook {
            hook_class: LocalHistoryExportReplayInspectionHookClass::PreviewReplay,
            action_id: "local_history_export_replay.preview_replay".to_owned(),
            label: "Preview replay implications".to_owned(),
            available: true,
            disclosure:
                "Previews the bytes the replay will restore, the encoding/newline class it will write, and the restore-provenance changes that will land before any apply commits."
                    .to_owned(),
        },
        LocalHistoryExportReplayInspectionHook {
            hook_class: LocalHistoryExportReplayInspectionHookClass::ExportPacket,
            action_id: "local_history_export_replay.export_packet".to_owned(),
            label: "Export packet".to_owned(),
            available: true,
            disclosure:
                "Exports the current packet (support-safe) so the exact packet identity, integrity hash, and compare-to-disk state can be replayed elsewhere."
                    .to_owned(),
        },
        LocalHistoryExportReplayInspectionHook {
            hook_class: LocalHistoryExportReplayInspectionHookClass::RollbackReplay,
            action_id: "local_history_export_replay.rollback_replay".to_owned(),
            label: "Roll back replay".to_owned(),
            available: true,
            disclosure:
                "Reverts the most recent destructive replay back to the previous packet identity, preserving restore provenance, encoding, and integrity hashes."
                    .to_owned(),
        },
        LocalHistoryExportReplayInspectionHook {
            hook_class: LocalHistoryExportReplayInspectionHookClass::RepairPacket,
            action_id: "local_history_export_replay.repair_packet".to_owned(),
            label: "Open typed repair sheet".to_owned(),
            available: true,
            disclosure:
                "Opens the typed repair sheet for the packet (rebuild integrity hash, refetch object body, re-mint compare-to-disk diff) rather than firing a repair as a shortcut."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Input envelope.
// ---------------------------------------------------------------------------

/// Metadata-safe support-export projection input for a packet or
/// replay row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryExportReplaySupportExportInputs {
    /// Whether the row ships a metadata-safe export or holds for
    /// manual review.
    pub posture: LocalHistoryExportReplaySupportExportPosture,
    pub includes_packet_kind: bool,
    pub includes_replay_path_class: bool,
    pub includes_packet_ref: bool,
    pub includes_compare_to_disk_class: bool,
    pub includes_body_availability_class: bool,
    pub includes_encoding_fidelity_class: bool,
    pub includes_restore_of_ref: bool,
    pub includes_mutation_journal_ref: bool,
    pub includes_integrity_hash: bool,
    pub raw_secrets_excluded: bool,
    pub raw_body_bytes_excluded: bool,
    pub approval_tickets_excluded: bool,
    pub delegated_credentials_excluded: bool,
    pub live_authority_handles_excluded: bool,
}

impl LocalHistoryExportReplaySupportExportInputs {
    /// Returns the metadata-safe baseline for a given posture.
    pub const fn metadata_safe_baseline(
        posture: LocalHistoryExportReplaySupportExportPosture,
    ) -> Self {
        Self {
            posture,
            includes_packet_kind: true,
            includes_replay_path_class: true,
            includes_packet_ref: true,
            includes_compare_to_disk_class: true,
            includes_body_availability_class: true,
            includes_encoding_fidelity_class: true,
            includes_restore_of_ref: true,
            includes_mutation_journal_ref: true,
            includes_integrity_hash: true,
            raw_secrets_excluded: true,
            raw_body_bytes_excluded: true,
            approval_tickets_excluded: true,
            delegated_credentials_excluded: true,
            live_authority_handles_excluded: true,
        }
    }
}

/// Closed support-export posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryExportReplaySupportExportPosture {
    /// Row ships a metadata-safe projection in the support packet.
    MetadataSafeExport,
    /// Row withholds its state until manual review.
    HeldRecord,
}

impl LocalHistoryExportReplaySupportExportPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeExport => "metadata_safe_export",
            Self::HeldRecord => "held_record",
        }
    }
}

/// One observation of a governed export packet at a captured moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketObservation {
    /// Stable packet identity every consumer preserves.
    pub packet_id: String,
    /// Closed packet kind.
    pub packet_kind: ExportPacketKind,
    /// Opaque packet ref (e.g. `pkt:local_history.entry:abc`).
    pub packet_ref: String,
    /// Body-availability class.
    pub body_availability_class: BodyAvailabilityClass,
    /// Optional override-disclosure ref (required when the class
    /// requires one).
    pub body_override_disclosure_ref: Option<String>,
    /// Encoding/newline fidelity class.
    pub encoding_fidelity_class: EncodingFidelityClass,
    /// True when the packet preserves the source encoding verbatim.
    pub encoding_preserved: bool,
    /// True when the packet preserves the source newline mode
    /// verbatim.
    pub newline_preserved: bool,
    /// True when the packet preserves the source BOM posture
    /// verbatim.
    pub bom_preserved: bool,
    /// Opaque restore-of ref (the entry/group/checkpoint the packet
    /// derives from).
    pub restore_of_ref: String,
    /// Opaque mutation-journal ref (the journal entry/group the
    /// packet attests to).
    pub mutation_journal_ref: String,
    /// Original actor-class token.
    pub actor_class: String,
    /// Non-empty integrity hash pinned for replay/verification.
    pub integrity_hash: String,
    /// Support-export projection for the packet row.
    pub support_export: LocalHistoryExportReplaySupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One observation of a replay / compare-to-disk path at a captured
/// moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayPathObservation {
    /// Stable replay path id.
    pub replay_path_id: String,
    /// Human-readable label.
    pub label: String,
    /// Closed replay path kind.
    pub replay_path_kind: ReplayPathKind,
    /// The stable packet id the replay path consumes.
    pub packet_id: String,
    /// Closed compare-to-disk state (required for compare-to-disk
    /// paths, optional otherwise).
    pub compare_to_disk_state: Option<CompareToDiskState>,
    /// True when the surface discloses the compare-to-disk state to
    /// the user before any destructive replay.
    pub discloses_disk_modified_state: bool,
    /// Closed no-rerun posture.
    pub rerun_posture: ReplayRerunPosture,
    /// Stable id of the commit action that gates this replay
    /// (required when the path mutates workspace state).
    pub commit_action_id: String,
    /// Stable id of the disclosure paired with the commit action
    /// (required when the path mutates workspace state).
    pub commit_disclosure_id: String,
    /// True when the replay path preserves the encoding/newline class
    /// from the packet onto disk.
    pub preserves_encoding_fidelity: bool,
    /// True when the replay path preserves the restore provenance
    /// (restore_of_ref + mutation_journal_ref + actor_class) on the
    /// replayed entry.
    pub preserves_restore_provenance: bool,
    /// True when the replay path verifies the packet's integrity hash
    /// before applying.
    pub verifies_integrity_hash: bool,
    /// Support-export projection for the replay row.
    pub support_export: LocalHistoryExportReplaySupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Input envelope ingested by the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryExportReplayInputs {
    /// Opaque workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque producer ref.
    pub producer_ref: String,
    /// Opaque corpus ref.
    pub corpus_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Captured packet observations.
    pub packets: Vec<PacketObservation>,
    /// Captured replay-path observations.
    pub replay_paths: Vec<ReplayPathObservation>,
}

// ---------------------------------------------------------------------------
// Narrow reasons + qualification.
// ---------------------------------------------------------------------------

/// Named reason a local-history export/replay lineage record narrows
/// below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryExportReplayLineageNarrowReason {
    /// The captured input had no packets or no replay paths.
    CorpusEmpty,
    /// A required packet kind is missing from the corpus.
    RequiredPacketKindMissing,
    /// A required replay-path kind is missing from the corpus.
    RequiredReplayPathKindMissing,
    /// A replay path references a packet id not present in the
    /// corpus.
    ReplayReferencesUnknownPacket,
    /// A compare-to-disk replay path is missing its compare-to-disk
    /// state.
    CompareToDiskStateMissing,
    /// A compare-to-disk replay surfaced a `disk_modified_since_packet`
    /// (or other user-disclosure state) without disclosing it to the
    /// user.
    DiskModifiedSilentlyTreatedAsClean,
    /// A packet declares a body-availability class that requires an
    /// override disclosure but no override-disclosure ref ships.
    BodyOverrideDisclosureMissing,
    /// A packet ships raw body bytes by default without an explicit
    /// override.
    BodyRawByDefault,
    /// A replay path declares `silent_rerun_permitted` (forbidden on
    /// Stable rows).
    ReplayRerunSilentForbidden,
    /// A workspace-mutating replay is missing its commit action id
    /// or commit disclosure id.
    ReplayCommitActionMetadataMissing,
    /// A packet does not preserve the source encoding/newline/BOM
    /// posture, or a replay path does not preserve the encoding
    /// fidelity class.
    EncodingFidelityNotPreserved,
    /// A packet or replay path does not preserve the restore
    /// provenance.
    RestoreProvenanceNotPreserved,
    /// A packet ships without an integrity hash, or a replay path
    /// does not verify the integrity hash.
    IntegrityHashNotPinned,
    /// A required pre-action inspection / repair hook is unavailable.
    InspectionHookUnavailable,
    /// A support-export projection drops a required field.
    SupportExportFieldsDropped,
    /// Raw secrets, raw body bytes, approval tickets, delegated
    /// credentials, or live authority handles slipped into a
    /// support-export projection.
    SupportExportRedactionUnsafe,
    /// Producer attribution is incomplete (producer ref or
    /// captured-at empty).
    ProducerAttributionIncomplete,
    /// Workspace ref or corpus ref is empty (would break support
    /// export).
    LineageExportUnsafe,
}

impl LocalHistoryExportReplayLineageNarrowReason {
    /// Returns the stable snake_case token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusEmpty => "corpus_empty",
            Self::RequiredPacketKindMissing => "required_packet_kind_missing",
            Self::RequiredReplayPathKindMissing => "required_replay_path_kind_missing",
            Self::ReplayReferencesUnknownPacket => "replay_references_unknown_packet",
            Self::CompareToDiskStateMissing => "compare_to_disk_state_missing",
            Self::DiskModifiedSilentlyTreatedAsClean => "disk_modified_silently_treated_as_clean",
            Self::BodyOverrideDisclosureMissing => "body_override_disclosure_missing",
            Self::BodyRawByDefault => "body_raw_by_default",
            Self::ReplayRerunSilentForbidden => "replay_rerun_silent_forbidden",
            Self::ReplayCommitActionMetadataMissing => "replay_commit_action_metadata_missing",
            Self::EncodingFidelityNotPreserved => "encoding_fidelity_not_preserved",
            Self::RestoreProvenanceNotPreserved => "restore_provenance_not_preserved",
            Self::IntegrityHashNotPinned => "integrity_hash_not_pinned",
            Self::InspectionHookUnavailable => "inspection_hook_unavailable",
            Self::SupportExportFieldsDropped => "support_export_fields_dropped",
            Self::SupportExportRedactionUnsafe => "support_export_redaction_unsafe",
            Self::ProducerAttributionIncomplete => "producer_attribution_incomplete",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a local-history export/replay
/// lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryExportReplayLineageQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not
    /// qualified.
    pub narrow_reasons: Vec<LocalHistoryExportReplayLineageNarrowReason>,
}

// ---------------------------------------------------------------------------
// Pillar projections.
// ---------------------------------------------------------------------------

/// One packet row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketRow {
    pub packet_id: String,
    pub packet_kind: ExportPacketKind,
    pub packet_ref: String,
    pub body_availability_class: BodyAvailabilityClass,
    pub body_override_disclosure_ref: Option<String>,
    pub encoding_fidelity_class: EncodingFidelityClass,
    pub encoding_preserved: bool,
    pub newline_preserved: bool,
    pub bom_preserved: bool,
    pub restore_of_ref: String,
    pub mutation_journal_ref: String,
    pub actor_class: String,
    pub integrity_hash: String,
    pub support_export_posture: LocalHistoryExportReplaySupportExportPosture,
    pub is_required: bool,
}

/// One replay-path row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayPathRow {
    pub replay_path_id: String,
    pub label: String,
    pub replay_path_kind: ReplayPathKind,
    pub packet_id: String,
    pub compare_to_disk_state: Option<CompareToDiskState>,
    pub discloses_disk_modified_state: bool,
    pub rerun_posture: ReplayRerunPosture,
    pub commit_action_id: String,
    pub commit_disclosure_id: String,
    pub preserves_encoding_fidelity: bool,
    pub preserves_restore_provenance: bool,
    pub verifies_integrity_hash: bool,
    pub support_export_posture: LocalHistoryExportReplaySupportExportPosture,
    pub mutates_workspace: bool,
    pub is_compare_to_disk: bool,
    pub is_required: bool,
}

/// Packet-kind coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketCoverageSummary {
    pub packet_rows: Vec<PacketRow>,
    pub all_required_packet_kinds_present: bool,
}

/// Replay-path coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayPathCoverageSummary {
    pub replay_path_rows: Vec<ReplayPathRow>,
    pub all_required_replay_path_kinds_present: bool,
}

/// Compare-to-disk honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareToDiskHonestySummary {
    pub compare_to_disk_path_count: usize,
    pub all_compare_paths_have_state: bool,
    pub no_disk_modified_silently_clean: bool,
}

/// Body-export safety posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BodyExportSafetySummary {
    pub all_overrides_have_disclosure: bool,
    pub no_raw_body_by_default: bool,
}

/// Encoding/newline fidelity posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncodingFidelitySummary {
    pub all_packets_preserve_encoding: bool,
    pub all_packets_preserve_newline: bool,
    pub all_packets_preserve_bom: bool,
    pub all_replays_preserve_encoding_fidelity: bool,
}

/// Restore-provenance posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProvenanceSummary {
    pub all_packets_carry_restore_of_ref: bool,
    pub all_packets_carry_mutation_journal_ref: bool,
    pub all_packets_carry_actor_class: bool,
    pub all_replays_preserve_restore_provenance: bool,
}

/// No-silent-rerun posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSilentRerunSummary {
    pub all_replays_safe_rerun_posture: bool,
    pub all_mutating_replays_have_commit_metadata: bool,
}

/// Integrity-hash pinning posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IntegrityHashPinningSummary {
    pub all_packets_pin_integrity_hash: bool,
    pub all_replays_verify_integrity_hash: bool,
}

/// Support-export honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryExportReplaySupportExportHonestySummary {
    pub all_rows_preserve_fields: bool,
    pub all_rows_exclude_raw_secrets: bool,
    pub all_rows_exclude_raw_body_bytes: bool,
    pub all_rows_exclude_approval_tickets: bool,
    pub all_rows_exclude_delegated_credentials: bool,
    pub all_rows_exclude_live_authority_handles: bool,
}

/// Producer-attribution posture for replay safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryExportReplayProducerAttributionSummary {
    pub producer_ref: String,
    pub schema_version: u32,
    pub integrity_hash: String,
    pub captured_at: String,
    pub producer_attribution_complete: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe local-history export/replay lineage record
/// per posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryExportReplayLineageRecord {
    pub record_kind: String,
    pub local_history_export_replay_lineage_schema_version: u32,
    pub schema_ref: String,
    pub posture_id: String,
    pub workspace_ref: String,
    pub corpus_ref: String,
    pub producer_attribution: LocalHistoryExportReplayProducerAttributionSummary,
    pub packet_coverage: PacketCoverageSummary,
    pub replay_path_coverage: ReplayPathCoverageSummary,
    pub compare_to_disk_honesty: CompareToDiskHonestySummary,
    pub body_export_safety: BodyExportSafetySummary,
    pub encoding_fidelity: EncodingFidelitySummary,
    pub restore_provenance: RestoreProvenanceSummary,
    pub no_silent_rerun: NoSilentRerunSummary,
    pub integrity_hash_pinning: IntegrityHashPinningSummary,
    pub support_export_honesty: LocalHistoryExportReplaySupportExportHonestySummary,
    pub inspection_hooks: Vec<LocalHistoryExportReplayInspectionHook>,
    pub stable_qualification: LocalHistoryExportReplayLineageQualification,
    pub raw_payload_excluded: bool,
    pub summary: String,
}

impl LocalHistoryExportReplayLineageRecord {
    /// Returns true when the record is metadata-safe for support
    /// export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_SCHEMA_REF
            && self.record_kind == LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_RECORD_KIND
            && !self.workspace_ref.trim().is_empty()
            && !self.corpus_ref.trim().is_empty()
    }

    /// Returns true when the record proves the contract on the
    /// claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }

    /// Returns the inspection hook of the given class, when present.
    pub fn inspection_hook(
        &self,
        class: LocalHistoryExportReplayInspectionHookClass,
    ) -> Option<&LocalHistoryExportReplayInspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed local-history export/replay lineage record
/// from a live [`LocalHistoryExportReplayInputs`] envelope using the
/// default inspection-hook set.
pub fn project_local_history_export_replay_lineage(
    posture_id: impl Into<String>,
    inputs: &LocalHistoryExportReplayInputs,
) -> LocalHistoryExportReplayLineageRecord {
    project_local_history_export_replay_lineage_with_hooks(
        posture_id,
        inputs,
        default_local_history_export_replay_inspection_hooks(),
    )
}

/// Like [`project_local_history_export_replay_lineage`] but with an
/// explicit inspection-hook set (for testing degraded-hook postures).
pub fn project_local_history_export_replay_lineage_with_hooks(
    posture_id: impl Into<String>,
    inputs: &LocalHistoryExportReplayInputs,
    inspection_hooks: Vec<LocalHistoryExportReplayInspectionHook>,
) -> LocalHistoryExportReplayLineageRecord {
    let posture_id: String = posture_id.into();

    let packet_coverage = project_packet_coverage(inputs);
    let replay_path_coverage = project_replay_path_coverage(inputs);
    let compare_to_disk_honesty = project_compare_to_disk_honesty(&replay_path_coverage);
    let body_export_safety = project_body_export_safety(&packet_coverage);
    let encoding_fidelity = project_encoding_fidelity(&packet_coverage, &replay_path_coverage);
    let restore_provenance = project_restore_provenance(&packet_coverage, &replay_path_coverage);
    let no_silent_rerun = project_no_silent_rerun(&replay_path_coverage);
    let integrity_hash_pinning =
        project_integrity_hash_pinning(&packet_coverage, &replay_path_coverage);
    let support_export_honesty = project_support_export_honesty(inputs);
    let producer_attribution = project_producer_attribution(inputs);

    let known_packet_ids: BTreeSet<&str> = packet_coverage
        .packet_rows
        .iter()
        .map(|row| row.packet_id.as_str())
        .collect();

    let mut narrow_reasons = Vec::new();

    if inputs.packets.is_empty() || inputs.replay_paths.is_empty() {
        narrow_reasons.push(LocalHistoryExportReplayLineageNarrowReason::CorpusEmpty);
    }
    if !packet_coverage.all_required_packet_kinds_present {
        narrow_reasons
            .push(LocalHistoryExportReplayLineageNarrowReason::RequiredPacketKindMissing);
    }
    if !replay_path_coverage.all_required_replay_path_kinds_present {
        narrow_reasons
            .push(LocalHistoryExportReplayLineageNarrowReason::RequiredReplayPathKindMissing);
    }
    if replay_path_coverage
        .replay_path_rows
        .iter()
        .any(|row| !known_packet_ids.contains(row.packet_id.as_str()))
    {
        narrow_reasons
            .push(LocalHistoryExportReplayLineageNarrowReason::ReplayReferencesUnknownPacket);
    }
    if !compare_to_disk_honesty.all_compare_paths_have_state {
        narrow_reasons.push(LocalHistoryExportReplayLineageNarrowReason::CompareToDiskStateMissing);
    }
    if !compare_to_disk_honesty.no_disk_modified_silently_clean {
        narrow_reasons
            .push(LocalHistoryExportReplayLineageNarrowReason::DiskModifiedSilentlyTreatedAsClean);
    }
    if !body_export_safety.all_overrides_have_disclosure {
        narrow_reasons
            .push(LocalHistoryExportReplayLineageNarrowReason::BodyOverrideDisclosureMissing);
    }
    if !body_export_safety.no_raw_body_by_default {
        narrow_reasons.push(LocalHistoryExportReplayLineageNarrowReason::BodyRawByDefault);
    }
    if !no_silent_rerun.all_replays_safe_rerun_posture {
        narrow_reasons
            .push(LocalHistoryExportReplayLineageNarrowReason::ReplayRerunSilentForbidden);
    }
    if !no_silent_rerun.all_mutating_replays_have_commit_metadata {
        narrow_reasons
            .push(LocalHistoryExportReplayLineageNarrowReason::ReplayCommitActionMetadataMissing);
    }
    if !(encoding_fidelity.all_packets_preserve_encoding
        && encoding_fidelity.all_packets_preserve_newline
        && encoding_fidelity.all_packets_preserve_bom
        && encoding_fidelity.all_replays_preserve_encoding_fidelity)
    {
        narrow_reasons
            .push(LocalHistoryExportReplayLineageNarrowReason::EncodingFidelityNotPreserved);
    }
    if !(restore_provenance.all_packets_carry_restore_of_ref
        && restore_provenance.all_packets_carry_mutation_journal_ref
        && restore_provenance.all_packets_carry_actor_class
        && restore_provenance.all_replays_preserve_restore_provenance)
    {
        narrow_reasons
            .push(LocalHistoryExportReplayLineageNarrowReason::RestoreProvenanceNotPreserved);
    }
    if !(integrity_hash_pinning.all_packets_pin_integrity_hash
        && integrity_hash_pinning.all_replays_verify_integrity_hash)
    {
        narrow_reasons.push(LocalHistoryExportReplayLineageNarrowReason::IntegrityHashNotPinned);
    }

    let required_hooks = [
        LocalHistoryExportReplayInspectionHookClass::InspectPacket,
        LocalHistoryExportReplayInspectionHookClass::CompareBeforeReplay,
        LocalHistoryExportReplayInspectionHookClass::PreviewReplay,
        LocalHistoryExportReplayInspectionHookClass::ExportPacket,
        LocalHistoryExportReplayInspectionHookClass::RollbackReplay,
        LocalHistoryExportReplayInspectionHookClass::RepairPacket,
    ];
    if !required_hooks
        .iter()
        .all(|required| hook_available(&inspection_hooks, *required))
    {
        narrow_reasons
            .push(LocalHistoryExportReplayLineageNarrowReason::InspectionHookUnavailable);
    }

    collect_support_export_narrows(&support_export_honesty, &mut narrow_reasons);

    if !producer_attribution.producer_attribution_complete {
        narrow_reasons
            .push(LocalHistoryExportReplayLineageNarrowReason::ProducerAttributionIncomplete);
    }

    if inputs.workspace_ref.trim().is_empty() || inputs.corpus_ref.trim().is_empty() {
        narrow_reasons.push(LocalHistoryExportReplayLineageNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = LocalHistoryExportReplayLineageQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        &packet_coverage,
        &replay_path_coverage,
        &compare_to_disk_honesty,
        &stable_qualification,
    );

    LocalHistoryExportReplayLineageRecord {
        record_kind: LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_RECORD_KIND.to_owned(),
        local_history_export_replay_lineage_schema_version:
            LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_SCHEMA_VERSION,
        schema_ref: LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id,
        workspace_ref: inputs.workspace_ref.clone(),
        corpus_ref: inputs.corpus_ref.clone(),
        producer_attribution,
        packet_coverage,
        replay_path_coverage,
        compare_to_disk_honesty,
        body_export_safety,
        encoding_fidelity,
        restore_provenance,
        no_silent_rerun,
        integrity_hash_pinning,
        support_export_honesty,
        inspection_hooks,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Pillar builders.
// ---------------------------------------------------------------------------

fn project_packet_coverage(inputs: &LocalHistoryExportReplayInputs) -> PacketCoverageSummary {
    let packet_rows: Vec<PacketRow> = inputs.packets.iter().map(project_packet_row).collect();
    let observed: BTreeSet<_> = packet_rows.iter().map(|row| row.packet_kind).collect();
    let all_required_packet_kinds_present = REQUIRED_EXPORT_PACKET_KINDS
        .iter()
        .all(|required| observed.contains(required));
    PacketCoverageSummary {
        packet_rows,
        all_required_packet_kinds_present,
    }
}

fn project_packet_row(observation: &PacketObservation) -> PacketRow {
    PacketRow {
        packet_id: observation.packet_id.clone(),
        packet_kind: observation.packet_kind,
        packet_ref: observation.packet_ref.clone(),
        body_availability_class: observation.body_availability_class,
        body_override_disclosure_ref: observation.body_override_disclosure_ref.clone(),
        encoding_fidelity_class: observation.encoding_fidelity_class,
        encoding_preserved: observation.encoding_preserved,
        newline_preserved: observation.newline_preserved,
        bom_preserved: observation.bom_preserved,
        restore_of_ref: observation.restore_of_ref.clone(),
        mutation_journal_ref: observation.mutation_journal_ref.clone(),
        actor_class: observation.actor_class.clone(),
        integrity_hash: observation.integrity_hash.clone(),
        support_export_posture: observation.support_export.posture,
        is_required: observation.packet_kind.is_required(),
    }
}

fn project_replay_path_coverage(
    inputs: &LocalHistoryExportReplayInputs,
) -> ReplayPathCoverageSummary {
    let replay_path_rows: Vec<ReplayPathRow> = inputs
        .replay_paths
        .iter()
        .map(project_replay_path_row)
        .collect();
    let observed: BTreeSet<_> = replay_path_rows
        .iter()
        .map(|row| row.replay_path_kind)
        .collect();
    let all_required_replay_path_kinds_present = REQUIRED_REPLAY_PATH_KINDS
        .iter()
        .all(|required| observed.contains(required));
    ReplayPathCoverageSummary {
        replay_path_rows,
        all_required_replay_path_kinds_present,
    }
}

fn project_replay_path_row(observation: &ReplayPathObservation) -> ReplayPathRow {
    ReplayPathRow {
        replay_path_id: observation.replay_path_id.clone(),
        label: observation.label.clone(),
        replay_path_kind: observation.replay_path_kind,
        packet_id: observation.packet_id.clone(),
        compare_to_disk_state: observation.compare_to_disk_state,
        discloses_disk_modified_state: observation.discloses_disk_modified_state,
        rerun_posture: observation.rerun_posture,
        commit_action_id: observation.commit_action_id.clone(),
        commit_disclosure_id: observation.commit_disclosure_id.clone(),
        preserves_encoding_fidelity: observation.preserves_encoding_fidelity,
        preserves_restore_provenance: observation.preserves_restore_provenance,
        verifies_integrity_hash: observation.verifies_integrity_hash,
        support_export_posture: observation.support_export.posture,
        mutates_workspace: observation.replay_path_kind.mutates_workspace(),
        is_compare_to_disk: observation.replay_path_kind.is_compare_to_disk(),
        is_required: true,
    }
}

fn project_compare_to_disk_honesty(
    coverage: &ReplayPathCoverageSummary,
) -> CompareToDiskHonestySummary {
    let mut count = 0usize;
    let mut all_states = true;
    let mut all_disclosed = true;
    for row in &coverage.replay_path_rows {
        if !row.is_compare_to_disk {
            continue;
        }
        count += 1;
        let state = match row.compare_to_disk_state {
            Some(state) => state,
            None => {
                all_states = false;
                continue;
            }
        };
        if state.requires_user_disclosure() && !row.discloses_disk_modified_state {
            all_disclosed = false;
        }
    }
    CompareToDiskHonestySummary {
        compare_to_disk_path_count: count,
        all_compare_paths_have_state: all_states,
        no_disk_modified_silently_clean: all_disclosed,
    }
}

fn project_body_export_safety(coverage: &PacketCoverageSummary) -> BodyExportSafetySummary {
    let mut all_overrides = true;
    let mut no_raw_default = true;
    for row in &coverage.packet_rows {
        if row.body_availability_class.requires_override_disclosure() {
            let has_disclosure = row
                .body_override_disclosure_ref
                .as_ref()
                .is_some_and(|value| !value.trim().is_empty());
            if !has_disclosure {
                all_overrides = false;
            }
        }
        if matches!(row.body_availability_class, BodyAvailabilityClass::RawBodyWithDisclosure)
            && row
                .body_override_disclosure_ref
                .as_ref()
                .map_or(true, |value| value.trim().is_empty())
        {
            no_raw_default = false;
        }
    }
    BodyExportSafetySummary {
        all_overrides_have_disclosure: all_overrides,
        no_raw_body_by_default: no_raw_default,
    }
}

fn project_encoding_fidelity(
    packet_coverage: &PacketCoverageSummary,
    replay_coverage: &ReplayPathCoverageSummary,
) -> EncodingFidelitySummary {
    let mut enc = true;
    let mut nl = true;
    let mut bom = true;
    for row in &packet_coverage.packet_rows {
        if !row.encoding_preserved {
            enc = false;
        }
        if !row.newline_preserved {
            nl = false;
        }
        if !row.bom_preserved {
            bom = false;
        }
    }
    let replays_ok = replay_coverage
        .replay_path_rows
        .iter()
        .all(|row| row.preserves_encoding_fidelity);
    EncodingFidelitySummary {
        all_packets_preserve_encoding: enc,
        all_packets_preserve_newline: nl,
        all_packets_preserve_bom: bom,
        all_replays_preserve_encoding_fidelity: replays_ok,
    }
}

fn project_restore_provenance(
    packet_coverage: &PacketCoverageSummary,
    replay_coverage: &ReplayPathCoverageSummary,
) -> RestoreProvenanceSummary {
    let mut restore_ok = true;
    let mut journal_ok = true;
    let mut actor_ok = true;
    for row in &packet_coverage.packet_rows {
        if row.restore_of_ref.trim().is_empty() {
            restore_ok = false;
        }
        if row.mutation_journal_ref.trim().is_empty() {
            journal_ok = false;
        }
        if row.actor_class.trim().is_empty() {
            actor_ok = false;
        }
    }
    let replays_ok = replay_coverage
        .replay_path_rows
        .iter()
        .all(|row| row.preserves_restore_provenance);
    RestoreProvenanceSummary {
        all_packets_carry_restore_of_ref: restore_ok,
        all_packets_carry_mutation_journal_ref: journal_ok,
        all_packets_carry_actor_class: actor_ok,
        all_replays_preserve_restore_provenance: replays_ok,
    }
}

fn project_no_silent_rerun(coverage: &ReplayPathCoverageSummary) -> NoSilentRerunSummary {
    let mut posture_ok = true;
    let mut commit_ok = true;
    for row in &coverage.replay_path_rows {
        if !row.rerun_posture.safe_for_stable() {
            posture_ok = false;
        }
        if row.mutates_workspace
            && (row.commit_action_id.trim().is_empty()
                || row.commit_disclosure_id.trim().is_empty())
        {
            commit_ok = false;
        }
    }
    NoSilentRerunSummary {
        all_replays_safe_rerun_posture: posture_ok,
        all_mutating_replays_have_commit_metadata: commit_ok,
    }
}

fn project_integrity_hash_pinning(
    packet_coverage: &PacketCoverageSummary,
    replay_coverage: &ReplayPathCoverageSummary,
) -> IntegrityHashPinningSummary {
    let packets_ok = packet_coverage
        .packet_rows
        .iter()
        .all(|row| !row.integrity_hash.trim().is_empty());
    let replays_ok = replay_coverage
        .replay_path_rows
        .iter()
        .all(|row| row.verifies_integrity_hash);
    IntegrityHashPinningSummary {
        all_packets_pin_integrity_hash: packets_ok,
        all_replays_verify_integrity_hash: replays_ok,
    }
}

fn project_support_export_honesty(
    inputs: &LocalHistoryExportReplayInputs,
) -> LocalHistoryExportReplaySupportExportHonestySummary {
    let mut preserve_fields = true;
    let mut redact_secrets = true;
    let mut exclude_body = true;
    let mut exclude_approvals = true;
    let mut exclude_credentials = true;
    let mut exclude_authority = true;

    let supports = inputs
        .packets
        .iter()
        .map(|p| p.support_export)
        .chain(inputs.replay_paths.iter().map(|r| r.support_export));

    for support in supports {
        if !(support.includes_packet_kind
            && support.includes_replay_path_class
            && support.includes_packet_ref
            && support.includes_compare_to_disk_class
            && support.includes_body_availability_class
            && support.includes_encoding_fidelity_class
            && support.includes_restore_of_ref
            && support.includes_mutation_journal_ref
            && support.includes_integrity_hash)
        {
            preserve_fields = false;
        }
        if !support.raw_secrets_excluded {
            redact_secrets = false;
        }
        if !support.raw_body_bytes_excluded {
            exclude_body = false;
        }
        if !support.approval_tickets_excluded {
            exclude_approvals = false;
        }
        if !support.delegated_credentials_excluded {
            exclude_credentials = false;
        }
        if !support.live_authority_handles_excluded {
            exclude_authority = false;
        }
    }

    LocalHistoryExportReplaySupportExportHonestySummary {
        all_rows_preserve_fields: preserve_fields,
        all_rows_exclude_raw_secrets: redact_secrets,
        all_rows_exclude_raw_body_bytes: exclude_body,
        all_rows_exclude_approval_tickets: exclude_approvals,
        all_rows_exclude_delegated_credentials: exclude_credentials,
        all_rows_exclude_live_authority_handles: exclude_authority,
    }
}

fn project_producer_attribution(
    inputs: &LocalHistoryExportReplayInputs,
) -> LocalHistoryExportReplayProducerAttributionSummary {
    let integrity_hash = compute_integrity_hash(inputs);
    let producer_attribution_complete =
        !inputs.producer_ref.trim().is_empty() && !inputs.captured_at.trim().is_empty();
    LocalHistoryExportReplayProducerAttributionSummary {
        producer_ref: inputs.producer_ref.clone(),
        schema_version: LOCAL_HISTORY_EXPORT_REPLAY_LINEAGE_SCHEMA_VERSION,
        integrity_hash,
        captured_at: inputs.captured_at.clone(),
        producer_attribution_complete,
    }
}

fn collect_support_export_narrows(
    summary: &LocalHistoryExportReplaySupportExportHonestySummary,
    narrow_reasons: &mut Vec<LocalHistoryExportReplayLineageNarrowReason>,
) {
    if !summary.all_rows_preserve_fields {
        narrow_reasons.push(LocalHistoryExportReplayLineageNarrowReason::SupportExportFieldsDropped);
    }
    if !(summary.all_rows_exclude_raw_secrets
        && summary.all_rows_exclude_raw_body_bytes
        && summary.all_rows_exclude_approval_tickets
        && summary.all_rows_exclude_delegated_credentials
        && summary.all_rows_exclude_live_authority_handles)
    {
        narrow_reasons.push(LocalHistoryExportReplayLineageNarrowReason::SupportExportRedactionUnsafe);
    }
}

fn compute_integrity_hash(inputs: &LocalHistoryExportReplayInputs) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    let header = [
        inputs.workspace_ref.as_str(),
        inputs.producer_ref.as_str(),
        inputs.corpus_ref.as_str(),
        inputs.captured_at.as_str(),
    ];
    for input in header {
        for byte in input.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for packet in &inputs.packets {
        for byte in packet.packet_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(packet.packet_kind.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(packet.encoding_fidelity_class.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for replay in &inputs.replay_paths {
        for byte in replay.replay_path_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(replay.replay_path_kind.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(replay.rerun_posture.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("lher:{hash:016x}")
}

fn hook_available(
    hooks: &[LocalHistoryExportReplayInspectionHook],
    class: LocalHistoryExportReplayInspectionHookClass,
) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn build_summary(
    packet_coverage: &PacketCoverageSummary,
    replay_coverage: &ReplayPathCoverageSummary,
    compare_to_disk_honesty: &CompareToDiskHonestySummary,
    qualification: &LocalHistoryExportReplayLineageQualification,
) -> String {
    if qualification.qualified {
        format!(
            "Local-history export/replay lineage proven Stable: packets={packets} replays={replays} compare_paths={compare}.",
            packets = packet_coverage.packet_rows.len(),
            replays = replay_coverage.replay_path_rows.len(),
            compare = compare_to_disk_honesty.compare_to_disk_path_count,
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Local-history export/replay lineage narrowed below Stable (packets={packets} replays={replays} compare_paths={compare}): {reasons}.",
            packets = packet_coverage.packet_rows.len(),
            replays = replay_coverage.replay_path_rows.len(),
            compare = compare_to_disk_honesty.compare_to_disk_path_count,
            reasons = reasons.join(", "),
        )
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection (for headless emitter / shell status surface).
// ---------------------------------------------------------------------------

/// Returns the human-readable projection of a local-history
/// export/replay lineage record. The same projection is consumed by
/// the workspace local-history status surface, the headless CLI
/// emitter, Help/About, and support export.
pub fn local_history_export_replay_lineage_lines(
    record: &LocalHistoryExportReplayLineageRecord,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Local-history export/replay lineage — {} ({})",
        record.posture_id, record.stable_qualification.level
    ));
    lines.push(format!(
        "workspace={} corpus={} producer={} integrity_hash={} captured_at={}",
        record.workspace_ref,
        record.corpus_ref,
        record.producer_attribution.producer_ref,
        record.producer_attribution.integrity_hash,
        record.producer_attribution.captured_at,
    ));
    lines.push(format!(
        "packet_coverage: packets={} required_present={}",
        record.packet_coverage.packet_rows.len(),
        record.packet_coverage.all_required_packet_kinds_present,
    ));
    lines.push("Packets:".to_owned());
    for row in &record.packet_coverage.packet_rows {
        let override_ref = row
            .body_override_disclosure_ref
            .as_deref()
            .unwrap_or("none");
        lines.push(format!(
            "  - {kind} {id} ref={packet_ref} body={body} override={override_ref} encoding={enc} enc_preserved={enc_ok} nl_preserved={nl_ok} bom_preserved={bom_ok} restore_of={restore_of} mutation_journal={mutation} actor={actor} integrity={integrity} required={required} support_export={support}",
            kind = row.packet_kind.as_str(),
            id = row.packet_id,
            packet_ref = row.packet_ref,
            body = row.body_availability_class.as_str(),
            override_ref = override_ref,
            enc = row.encoding_fidelity_class.as_str(),
            enc_ok = row.encoding_preserved,
            nl_ok = row.newline_preserved,
            bom_ok = row.bom_preserved,
            restore_of = row.restore_of_ref,
            mutation = row.mutation_journal_ref,
            actor = row.actor_class,
            integrity = row.integrity_hash,
            required = row.is_required,
            support = row.support_export_posture.as_str(),
        ));
    }
    lines.push(format!(
        "replay_path_coverage: paths={} required_present={}",
        record.replay_path_coverage.replay_path_rows.len(),
        record.replay_path_coverage.all_required_replay_path_kinds_present,
    ));
    lines.push("Replay paths:".to_owned());
    for row in &record.replay_path_coverage.replay_path_rows {
        let compare_state = row
            .compare_to_disk_state
            .map(|state| state.as_str())
            .unwrap_or("none");
        lines.push(format!(
            "  - {kind} {id} packet={packet} compare_state={compare_state} discloses_modified={discloses} rerun={rerun} commit_action={commit_action} commit_disclosure={commit_disclosure} preserves_encoding={enc} preserves_provenance={prov} verifies_integrity={integ} mutates={mutates} compare_path={compare_path} required={required} support_export={support}",
            kind = row.replay_path_kind.as_str(),
            id = row.replay_path_id,
            packet = row.packet_id,
            compare_state = compare_state,
            discloses = row.discloses_disk_modified_state,
            rerun = row.rerun_posture.as_str(),
            commit_action = row.commit_action_id,
            commit_disclosure = row.commit_disclosure_id,
            enc = row.preserves_encoding_fidelity,
            prov = row.preserves_restore_provenance,
            integ = row.verifies_integrity_hash,
            mutates = row.mutates_workspace,
            compare_path = row.is_compare_to_disk,
            required = row.is_required,
            support = row.support_export_posture.as_str(),
        ));
    }
    lines.push(format!(
        "Compare-to-disk honesty: count={count} have_state={state} no_silent_modified={silent}",
        count = record.compare_to_disk_honesty.compare_to_disk_path_count,
        state = record.compare_to_disk_honesty.all_compare_paths_have_state,
        silent = record.compare_to_disk_honesty.no_disk_modified_silently_clean,
    ));
    lines.push(format!(
        "Body-export safety: overrides_disclosed={overrides} no_raw_default={raw}",
        overrides = record.body_export_safety.all_overrides_have_disclosure,
        raw = record.body_export_safety.no_raw_body_by_default,
    ));
    lines.push(format!(
        "Encoding fidelity: encoding={enc} newline={nl} bom={bom} replays={replays}",
        enc = record.encoding_fidelity.all_packets_preserve_encoding,
        nl = record.encoding_fidelity.all_packets_preserve_newline,
        bom = record.encoding_fidelity.all_packets_preserve_bom,
        replays = record.encoding_fidelity.all_replays_preserve_encoding_fidelity,
    ));
    lines.push(format!(
        "Restore provenance: restore_of={r} mutation_journal={m} actor={a} replays={replays}",
        r = record.restore_provenance.all_packets_carry_restore_of_ref,
        m = record
            .restore_provenance
            .all_packets_carry_mutation_journal_ref,
        a = record.restore_provenance.all_packets_carry_actor_class,
        replays = record
            .restore_provenance
            .all_replays_preserve_restore_provenance,
    ));
    lines.push(format!(
        "No-silent-rerun: posture={posture} commit_metadata={commit}",
        posture = record.no_silent_rerun.all_replays_safe_rerun_posture,
        commit = record
            .no_silent_rerun
            .all_mutating_replays_have_commit_metadata,
    ));
    lines.push(format!(
        "Integrity-hash pinning: packets={p} replays={r}",
        p = record.integrity_hash_pinning.all_packets_pin_integrity_hash,
        r = record
            .integrity_hash_pinning
            .all_replays_verify_integrity_hash,
    ));
    lines.push(format!(
        "Support-export honesty: preserve_fields={fields} exclude_secrets={secrets} exclude_body={body} exclude_approvals={approvals} exclude_credentials={credentials} exclude_authority={authority}",
        fields = record.support_export_honesty.all_rows_preserve_fields,
        secrets = record.support_export_honesty.all_rows_exclude_raw_secrets,
        body = record.support_export_honesty.all_rows_exclude_raw_body_bytes,
        approvals = record
            .support_export_honesty
            .all_rows_exclude_approval_tickets,
        credentials = record
            .support_export_honesty
            .all_rows_exclude_delegated_credentials,
        authority = record
            .support_export_honesty
            .all_rows_exclude_live_authority_handles,
    ));
    lines.push("Inspection hooks:".to_owned());
    for hook in &record.inspection_hooks {
        lines.push(format!(
            "  {class} [{id}] available={available} — {label}",
            class = hook.hook_class.as_str(),
            id = hook.action_id,
            available = hook.available,
            label = hook.label,
        ));
    }
    if !record.stable_qualification.qualified {
        let reasons: Vec<&str> = record
            .stable_qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        lines.push(format!("Narrowed below Stable: {}", reasons.join(", ")));
    }
    lines.push(record.summary.clone());
    lines
}

#[cfg(test)]
mod tests;
