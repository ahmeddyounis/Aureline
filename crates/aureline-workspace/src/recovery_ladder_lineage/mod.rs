//! Recovery-ladder sequencing lineage: the governed, export-safe
//! projection that hardens how Aureline walks through crash-loop safe
//! mode, suspect-extension quarantine, open-without-restore,
//! cache/index repair, restricted reopen, and (optionally) typed repair
//! and support-export handoff — and proves that the recovery sequence
//! never silently replays privileged work, never drops user state
//! without disclosure, and never narrows trust into an unreviewable
//! state.
//!
//! Where the cache / storage-class lineage proves the storage layer
//! underneath caches and durable state, and the trust-gating lineage
//! proves that privileged surfaces are gated under restricted
//! workspaces, this projection proves the *ordered recovery sequence*
//! the user, support, and the IDE itself walk when something goes
//! wrong: which rungs exist, which trigger reaches each rung, which
//! no-rerun posture each rung declares, which reversibility class each
//! rung admits, which user-state preservation posture each rung
//! commits to, which inspection / repair hooks fire before any
//! destructive rung commits, and which support-export projection each
//! rung ships.
//!
//! The projection ingests a live [`RecoveryLadderInputs`] envelope
//! verbatim (one [`RecoveryRungObservation`] per ladder rung plus the
//! controlled inspection-hook table) and produces a lineage record
//! that proves the contract claims the stable line is anchored on:
//!
//! - **Rung-coverage truth.** Every required rung ships a row bound to
//!   one closed [`RecoveryRungKind`] (`crash_loop_safe_mode`,
//!   `safe_mode_quarantine`, `open_without_restore`,
//!   `cache_index_repair`, `restricted_reopen`). Optional terminal
//!   rungs (`typed_repair_flow`, `support_export_handoff`) ride on top
//!   without changing the required set.
//! - **Sequence truth.** Every required rung carries the canonical
//!   step ordinal so the ladder cannot ship out-of-order or skip past
//!   a less-invasive rung silently.
//! - **No-rerun honesty.** Every rung declares one closed
//!   [`NoRerunPosture`]; rungs that touch privileged or mutating
//!   surfaces (anything beyond `crash_loop_safe_mode`) must declare
//!   `explicit_user_action_required` and reference both an action id
//!   and a disclosure id so resume / reconnect / recovery cannot
//!   silently replay terminals, tasks, debug sessions, or AI apply.
//! - **User-state preservation truth.** Every rung that can lose user
//!   state declares one closed [`UserStatePreservationPosture`]; any
//!   posture beyond `preserved` requires an `export_before_repair`
//!   disclosure plus the rollback-checkpoint inspection hook.
//! - **Reversibility truth.** Every rung declares one closed
//!   [`ReversibilityClass`]; irreversible rungs require a disclosure
//!   id and the rollback-checkpoint inspection hook.
//! - **Support-export honesty.** Each row's support-export projection
//!   preserves rung kind, trigger class, no-rerun posture, user-state
//!   preservation posture, reversibility class, claimed step ordinal,
//!   and disclosure id while excluding raw secrets, approval tickets,
//!   delegated credentials, and live authority handles.
//! - **Pre-action inspection-hook honesty.** A controlled set of
//!   pre-action inspection / repair hooks (`inspect_ladder_state`,
//!   `compare_before_action`, `export_before_repair`,
//!   `rollback_checkpoint`, `export`, `repair`) must be reachable so
//!   destructive recovery rungs stay reviewable.
//! - **Producer attribution.** Each record carries the producer ref,
//!   the schema version, the capture timestamp, and an integrity hash
//!   derived from the input identities so replay and support
//!   pipelines can pin the source before applying.
//! - **Lineage and export honesty.** The record sets
//!   `raw_payload_excluded = true` and carries only opaque refs to the
//!   source corpus, workspace, and producer.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for [`RecoveryLadderLineageRecord`].
pub const RECOVERY_LADDER_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the recovery-ladder lineage record.
pub const RECOVERY_LADDER_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/recovery_ladder_lineage.schema.json";

/// Stable record-kind tag for the recovery-ladder lineage record.
pub const RECOVERY_LADDER_LINEAGE_RECORD_KIND: &str = "recovery_ladder_lineage_record";

// ---------------------------------------------------------------------------
// Closed vocabularies.
// ---------------------------------------------------------------------------

/// Closed vocabulary for the rungs Aureline walks through on the
/// recovery ladder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryRungKind {
    /// Repeated crash / hang has tripped the runtime into safe mode.
    CrashLoopSafeMode,
    /// Suspect extensions / runtime quarantined to isolate regressions.
    SafeModeQuarantine,
    /// Workspace opened cold without resuming saved session state.
    OpenWithoutRestore,
    /// Caches and derived indexes rebuilt from authoritative sources.
    CacheIndexRepair,
    /// Workspace reopened under restricted (read-only) trust posture.
    RestrictedReopen,
    /// Typed repair transaction with explicit preview / rollback.
    TypedRepairFlow,
    /// Terminal handoff to support with a metadata-safe packet.
    SupportExportHandoff,
}

impl RecoveryRungKind {
    /// Returns the stable snake_case token for this rung kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashLoopSafeMode => "crash_loop_safe_mode",
            Self::SafeModeQuarantine => "safe_mode_quarantine",
            Self::OpenWithoutRestore => "open_without_restore",
            Self::CacheIndexRepair => "cache_index_repair",
            Self::RestrictedReopen => "restricted_reopen",
            Self::TypedRepairFlow => "typed_repair_flow",
            Self::SupportExportHandoff => "support_export_handoff",
        }
    }

    /// Returns the canonical step ordinal for this rung — least
    /// invasive (`1`) to most invasive (`7`).
    pub const fn canonical_step_ordinal(self) -> u32 {
        match self {
            Self::CrashLoopSafeMode => 1,
            Self::SafeModeQuarantine => 2,
            Self::OpenWithoutRestore => 3,
            Self::CacheIndexRepair => 4,
            Self::RestrictedReopen => 5,
            Self::TypedRepairFlow => 6,
            Self::SupportExportHandoff => 7,
        }
    }

    /// True when the rung is one of the required ladder rungs (every
    /// claimed Stable corpus must seed all of them).
    pub const fn is_required(self) -> bool {
        matches!(
            self,
            Self::CrashLoopSafeMode
                | Self::SafeModeQuarantine
                | Self::OpenWithoutRestore
                | Self::CacheIndexRepair
                | Self::RestrictedReopen
        )
    }
}

/// Closed list of rungs every recovery-ladder lineage record must
/// seed in canonical order.
pub const REQUIRED_RECOVERY_RUNGS: [RecoveryRungKind; 5] = [
    RecoveryRungKind::CrashLoopSafeMode,
    RecoveryRungKind::SafeModeQuarantine,
    RecoveryRungKind::OpenWithoutRestore,
    RecoveryRungKind::CacheIndexRepair,
    RecoveryRungKind::RestrictedReopen,
];

/// Closed trigger-class vocabulary — the named reason a rung is
/// reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RungTriggerClass {
    /// Runtime detected a repeated crash / hang loop.
    RepeatedCrashLoop,
    /// Extension or runtime regression suspected after safe-mode
    /// stabilizes.
    ExtensionRegressionSuspected,
    /// Resume / restore state cannot be applied safely.
    ResumeStateUnsafe,
    /// Derived store (cache or index) is inconsistent with authority.
    DerivedStoreInconsistent,
    /// Workspace trust posture cannot be verified for full execution.
    TrustPostureUnverified,
    /// Typed repair flow requested by the user or support.
    TypedRepairRequested,
    /// Support / incident escalation initiated handoff.
    SupportEscalationInitiated,
}

impl RungTriggerClass {
    /// Returns the stable snake_case token for this trigger class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepeatedCrashLoop => "repeated_crash_loop",
            Self::ExtensionRegressionSuspected => "extension_regression_suspected",
            Self::ResumeStateUnsafe => "resume_state_unsafe",
            Self::DerivedStoreInconsistent => "derived_store_inconsistent",
            Self::TrustPostureUnverified => "trust_posture_unverified",
            Self::TypedRepairRequested => "typed_repair_requested",
            Self::SupportEscalationInitiated => "support_escalation_initiated",
        }
    }
}

/// Closed no-rerun posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoRerunPosture {
    /// The rung commits only after an explicit user action with a
    /// disclosure.
    ExplicitUserActionRequired,
    /// The rung continues automatically only after a rollback
    /// checkpoint is captured and the rung does not touch privileged
    /// or mutating surfaces.
    AutoContinueAfterCheckpoint,
    /// The rung is terminal (no further automatic run will fire).
    TerminalNoFurtherRun,
}

impl NoRerunPosture {
    /// Returns the stable snake_case token for this no-rerun posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitUserActionRequired => "explicit_user_action_required",
            Self::AutoContinueAfterCheckpoint => "auto_continue_after_checkpoint",
            Self::TerminalNoFurtherRun => "terminal_no_further_run",
        }
    }

    /// True when the posture lets the rung touch privileged or
    /// mutating surfaces (only `explicit_user_action_required` does).
    pub const fn safe_for_privileged_rung(self) -> bool {
        matches!(
            self,
            Self::ExplicitUserActionRequired | Self::TerminalNoFurtherRun
        )
    }
}

/// Closed user-state preservation posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserStatePreservationPosture {
    /// Rung does not touch user state.
    Preserved,
    /// Rung touches user state only after an export prompt fires.
    PreservedAfterExportPrompt,
    /// Rung drops user state but discloses it and captures a rollback
    /// checkpoint before committing.
    DroppedWithDisclosure,
}

impl UserStatePreservationPosture {
    /// Returns the stable snake_case token for this preservation
    /// posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::PreservedAfterExportPrompt => "preserved_after_export_prompt",
            Self::DroppedWithDisclosure => "dropped_with_disclosure",
        }
    }

    /// True when the posture requires an `export_before_repair`
    /// disclosure plus the rollback-checkpoint hook.
    pub const fn requires_export_before_repair(self) -> bool {
        matches!(
            self,
            Self::PreservedAfterExportPrompt | Self::DroppedWithDisclosure
        )
    }
}

/// Closed reversibility-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReversibilityClass {
    /// Rung is fully reversible.
    Reversible,
    /// Rung is reversible only when paired with a captured rollback
    /// checkpoint.
    ReversibleWithCheckpoint,
    /// Rung is irreversible and the user is informed via disclosure
    /// before commit.
    IrreversibleWithDisclosure,
}

impl ReversibilityClass {
    /// Returns the stable snake_case token for this reversibility
    /// class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reversible => "reversible",
            Self::ReversibleWithCheckpoint => "reversible_with_checkpoint",
            Self::IrreversibleWithDisclosure => "irreversible_with_disclosure",
        }
    }

    /// True when the class requires a captured rollback checkpoint
    /// before commit.
    pub const fn requires_rollback_checkpoint(self) -> bool {
        matches!(self, Self::ReversibleWithCheckpoint)
    }

    /// True when the class requires an explicit disclosure id.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::IrreversibleWithDisclosure)
    }
}

/// Closed support-export posture vocabulary — recovery rungs must
/// always emit a support-safe projection (no `local_only` posture).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoverySupportExportPosture {
    /// Rung ships a metadata-safe projection in the support packet.
    MetadataSafeExport,
    /// Rung withholds its state from the packet until manual review.
    HeldRecord,
}

impl RecoverySupportExportPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeExport => "metadata_safe_export",
            Self::HeldRecord => "held_record",
        }
    }
}

/// Class of pre-action inspection / repair hook offered before any
/// destructive recovery rung commits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLadderInspectionHookClass {
    /// Open the recovery-ladder inspector and surface the current
    /// rung.
    InspectLadderState,
    /// Compare the current rung's projected effect against the
    /// pre-rung baseline.
    CompareBeforeAction,
    /// Export the user-visible state before a destructive repair fires.
    ExportBeforeRepair,
    /// Capture a one-step rollback checkpoint before committing a
    /// repair.
    RollbackCheckpoint,
    /// Export the recovery-ladder lineage record (support-safe).
    Export,
    /// Open the typed repair sheet for the current rung.
    Repair,
}

impl RecoveryLadderInspectionHookClass {
    /// Returns the stable snake_case token for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectLadderState => "inspect_ladder_state",
            Self::CompareBeforeAction => "compare_before_action",
            Self::ExportBeforeRepair => "export_before_repair",
            Self::RollbackCheckpoint => "rollback_checkpoint",
            Self::Export => "export",
            Self::Repair => "repair",
        }
    }
}

/// One pre-action inspection / repair hook offered before a recovery
/// rung commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderInspectionHook {
    /// Hook class.
    pub hook_class: RecoveryLadderInspectionHookClass,
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
pub fn default_recovery_ladder_inspection_hooks() -> Vec<RecoveryLadderInspectionHook> {
    vec![
        RecoveryLadderInspectionHook {
            hook_class: RecoveryLadderInspectionHookClass::InspectLadderState,
            action_id: "recovery_ladder.inspect_ladder_state".to_owned(),
            label: "Inspect recovery ladder".to_owned(),
            available: true,
            disclosure:
                "Opens the recovery-ladder inspector with the current rung, trigger class, declared no-rerun posture, and the captured inspection hooks before any rung commits."
                    .to_owned(),
        },
        RecoveryLadderInspectionHook {
            hook_class: RecoveryLadderInspectionHookClass::CompareBeforeAction,
            action_id: "recovery_ladder.compare_before_action".to_owned(),
            label: "Compare before action".to_owned(),
            available: true,
            disclosure:
                "Renders the diff between the current workspace state and the projected post-rung state so the user can review what a recovery rung will change before it fires."
                    .to_owned(),
        },
        RecoveryLadderInspectionHook {
            hook_class: RecoveryLadderInspectionHookClass::ExportBeforeRepair,
            action_id: "recovery_ladder.export_before_repair".to_owned(),
            label: "Export before repair".to_owned(),
            available: true,
            disclosure:
                "Exports the user-visible workspace state into a support-safe artifact before any destructive repair commits, so state can be restored if the rung misfires."
                    .to_owned(),
        },
        RecoveryLadderInspectionHook {
            hook_class: RecoveryLadderInspectionHookClass::RollbackCheckpoint,
            action_id: "recovery_ladder.rollback_checkpoint".to_owned(),
            label: "Capture rollback checkpoint".to_owned(),
            available: true,
            disclosure:
                "Captures a one-step rollback checkpoint so the user can revert a recovery rung if a downstream surface relied on the pre-rung state."
                    .to_owned(),
        },
        RecoveryLadderInspectionHook {
            hook_class: RecoveryLadderInspectionHookClass::Export,
            action_id: "recovery_ladder.export".to_owned(),
            label: "Export recovery-ladder lineage".to_owned(),
            available: true,
            disclosure:
                "Exports this recovery-ladder lineage record for support without raw secrets, approval tickets, or delegated credentials."
                    .to_owned(),
        },
        RecoveryLadderInspectionHook {
            hook_class: RecoveryLadderInspectionHookClass::Repair,
            action_id: "recovery_ladder.repair".to_owned(),
            label: "Open typed repair sheet".to_owned(),
            available: true,
            disclosure:
                "Opens the typed repair sheet for the current rung and surfaces the preview, blast-radius disclosure, checkpoint, and reversal semantics rather than firing the repair as a shortcut."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Input envelope.
// ---------------------------------------------------------------------------

/// Metadata-safe support-export projection input for a recovery rung.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoverySupportExportInputs {
    pub posture: RecoverySupportExportPosture,
    pub includes_rung_kind: bool,
    pub includes_trigger_class: bool,
    pub includes_no_rerun_posture: bool,
    pub includes_user_state_posture: bool,
    pub includes_reversibility_class: bool,
    pub includes_step_ordinal: bool,
    pub includes_disclosure_id: bool,
    pub raw_secrets_excluded: bool,
    pub approval_tickets_excluded: bool,
    pub delegated_credentials_excluded: bool,
    pub live_authority_handles_excluded: bool,
}

impl RecoverySupportExportInputs {
    /// Returns the metadata-safe baseline for a given posture.
    pub const fn metadata_safe_baseline(posture: RecoverySupportExportPosture) -> Self {
        Self {
            posture,
            includes_rung_kind: true,
            includes_trigger_class: true,
            includes_no_rerun_posture: true,
            includes_user_state_posture: true,
            includes_reversibility_class: true,
            includes_step_ordinal: true,
            includes_disclosure_id: true,
            raw_secrets_excluded: true,
            approval_tickets_excluded: true,
            delegated_credentials_excluded: true,
            live_authority_handles_excluded: true,
        }
    }
}

/// One observation of a recovery-ladder rung at a captured moment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRungObservation {
    /// Stable rung id (route-style, e.g. `ladder.safe_mode_quarantine`).
    pub rung_id: String,
    /// Human-readable title.
    pub title: String,
    /// Closed rung kind.
    pub rung_kind: RecoveryRungKind,
    /// Declared canonical step ordinal (1-based).
    pub declared_step_ordinal: u32,
    /// Closed trigger class.
    pub trigger_class: RungTriggerClass,
    /// Stable id of the trigger disclosure.
    pub trigger_disclosure_id: String,
    /// Declared no-rerun posture.
    pub no_rerun_posture: NoRerunPosture,
    /// Stable id of the action that commits this rung (required for
    /// `explicit_user_action_required` rungs).
    pub commit_action_id: String,
    /// Stable id of the disclosure paired with the commit action.
    pub commit_disclosure_id: String,
    /// Whether this rung touches privileged or mutating surfaces
    /// (terminals, tasks, debug, AI apply, privileged extensions).
    pub touches_privileged_surface: bool,
    /// Declared user-state preservation posture.
    pub user_state_preservation: UserStatePreservationPosture,
    /// Stable id of the export-before-repair disclosure (required
    /// when `user_state_preservation` is not `preserved`).
    pub export_before_repair_disclosure_id: String,
    /// Declared reversibility class.
    pub reversibility: ReversibilityClass,
    /// Stable id of the rollback checkpoint captured before commit
    /// (required when reversibility is `reversible_with_checkpoint`).
    pub rollback_checkpoint_id: String,
    /// Stable id of the irreversibility disclosure (required when
    /// reversibility is `irreversible_with_disclosure`).
    pub irreversibility_disclosure_id: String,
    /// Support-export projection.
    pub support_export: RecoverySupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Input envelope ingested by the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderInputs {
    /// Opaque workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque producer ref.
    pub producer_ref: String,
    /// Opaque corpus ref.
    pub corpus_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Captured rung observations.
    pub rungs: Vec<RecoveryRungObservation>,
}

// ---------------------------------------------------------------------------
// Narrow reasons + qualification.
// ---------------------------------------------------------------------------

/// Named reason a recovery-ladder lineage record narrows below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryLadderLineageNarrowReason {
    /// The captured input had no rung observations.
    CorpusEmpty,
    /// A required rung kind is missing from the corpus.
    RequiredRungMissing,
    /// A rung declared a step ordinal that disagrees with its
    /// canonical position.
    RungSequenceUnordered,
    /// A rung declared a trigger class without referencing a
    /// trigger-disclosure id.
    RungTriggerMissingDisclosure,
    /// A rung that touches privileged surfaces declared a no-rerun
    /// posture other than `explicit_user_action_required`.
    NoRerunPostureUnsafe,
    /// An `explicit_user_action_required` rung is missing the commit
    /// action id or commit disclosure id.
    ExplicitActionMetadataMissing,
    /// A rung lossy to user state did not reference an
    /// `export_before_repair` disclosure id.
    UserStateLossUndisclosed,
    /// A `reversible_with_checkpoint` rung did not reference a
    /// rollback-checkpoint id.
    ReversibilityCheckpointMissing,
    /// An `irreversible_with_disclosure` rung did not reference an
    /// irreversibility-disclosure id.
    IrreversibleRungMissingDisclosure,
    /// A required pre-action inspection / repair hook is unavailable.
    InspectionHookUnavailable,
    /// A support-export projection drops a required field.
    SupportExportFieldsDropped,
    /// Raw secrets, approval tickets, delegated credentials, or live
    /// authority handles slipped into a support-export projection.
    SupportExportRedactionUnsafe,
    /// Producer attribution is incomplete (producer ref or
    /// captured-at empty).
    ProducerAttributionIncomplete,
    /// Workspace ref or corpus ref is empty (would break support
    /// export).
    LineageExportUnsafe,
}

impl RecoveryLadderLineageNarrowReason {
    /// Returns the stable snake_case token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusEmpty => "corpus_empty",
            Self::RequiredRungMissing => "required_rung_missing",
            Self::RungSequenceUnordered => "rung_sequence_unordered",
            Self::RungTriggerMissingDisclosure => "rung_trigger_missing_disclosure",
            Self::NoRerunPostureUnsafe => "no_rerun_posture_unsafe",
            Self::ExplicitActionMetadataMissing => "explicit_action_metadata_missing",
            Self::UserStateLossUndisclosed => "user_state_loss_undisclosed",
            Self::ReversibilityCheckpointMissing => "reversibility_checkpoint_missing",
            Self::IrreversibleRungMissingDisclosure => "irreversible_rung_missing_disclosure",
            Self::InspectionHookUnavailable => "inspection_hook_unavailable",
            Self::SupportExportFieldsDropped => "support_export_fields_dropped",
            Self::SupportExportRedactionUnsafe => "support_export_redaction_unsafe",
            Self::ProducerAttributionIncomplete => "producer_attribution_incomplete",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a recovery-ladder lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderLineageQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not
    /// qualified.
    pub narrow_reasons: Vec<RecoveryLadderLineageNarrowReason>,
}

// ---------------------------------------------------------------------------
// Pillar projections.
// ---------------------------------------------------------------------------

/// One recovery-rung row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryRungRow {
    /// Stable rung id.
    pub rung_id: String,
    /// Rung title.
    pub title: String,
    /// Rung kind.
    pub rung_kind: RecoveryRungKind,
    /// Declared step ordinal.
    pub declared_step_ordinal: u32,
    /// Canonical step ordinal re-derived from the rung kind.
    pub canonical_step_ordinal: u32,
    /// True when the declared step ordinal matches the canonical one.
    pub step_ordinal_matches: bool,
    /// Trigger class.
    pub trigger_class: RungTriggerClass,
    /// Trigger disclosure id.
    pub trigger_disclosure_id: String,
    /// No-rerun posture.
    pub no_rerun_posture: NoRerunPosture,
    /// Commit action id.
    pub commit_action_id: String,
    /// Commit disclosure id.
    pub commit_disclosure_id: String,
    /// Whether the rung touches privileged surfaces.
    pub touches_privileged_surface: bool,
    /// User-state preservation posture.
    pub user_state_preservation: UserStatePreservationPosture,
    /// Export-before-repair disclosure id.
    pub export_before_repair_disclosure_id: String,
    /// Reversibility class.
    pub reversibility: ReversibilityClass,
    /// Rollback checkpoint id.
    pub rollback_checkpoint_id: String,
    /// Irreversibility disclosure id.
    pub irreversibility_disclosure_id: String,
    /// Support-export posture.
    pub support_export_posture: RecoverySupportExportPosture,
    /// True when this rung is required by the contract.
    pub is_required: bool,
}

/// Rung-sequence coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RungSequenceCoverageSummary {
    /// All rung rows carried by the corpus.
    pub rung_rows: Vec<RecoveryRungRow>,
    /// True when every required rung is present.
    pub all_required_rungs_present: bool,
    /// True when every required rung carries its canonical step
    /// ordinal.
    pub all_required_steps_ordered: bool,
}

/// No-rerun honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoRerunHonestySummary {
    /// True when every rung that touches privileged surfaces declares
    /// `explicit_user_action_required` (or `terminal_no_further_run`
    /// when no further run is possible) — never
    /// `auto_continue_after_checkpoint`.
    pub all_privileged_rungs_safe: bool,
    /// True when every `explicit_user_action_required` rung references
    /// both a commit action id and a commit disclosure id.
    pub all_explicit_rungs_have_metadata: bool,
}

/// User-state preservation truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserStatePreservationSummary {
    /// Number of rungs that could lose user state.
    pub user_state_lossy_rung_count: usize,
    /// True when every lossy rung references an
    /// `export_before_repair` disclosure id.
    pub all_lossy_rungs_have_export_disclosure: bool,
}

/// Reversibility truth posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReversibilityTruthSummary {
    /// Number of rungs declared `reversible_with_checkpoint`.
    pub checkpointed_rung_count: usize,
    /// True when every `reversible_with_checkpoint` rung references a
    /// rollback-checkpoint id.
    pub all_checkpointed_rungs_have_checkpoint_id: bool,
    /// Number of rungs declared `irreversible_with_disclosure`.
    pub irreversible_rung_count: usize,
    /// True when every `irreversible_with_disclosure` rung references
    /// an irreversibility-disclosure id.
    pub all_irreversible_rungs_have_disclosure_id: bool,
}

/// Trigger-disclosure honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriggerDisclosureSummary {
    /// True when every rung references a non-empty trigger disclosure
    /// id.
    pub all_rungs_have_trigger_disclosure: bool,
}

/// Support-export honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoverySupportExportHonestySummary {
    /// True when every rung's support-export projection preserves the
    /// required recovery-ladder fields.
    pub all_rungs_preserve_fields: bool,
    /// True when every rung redacts raw secrets.
    pub all_rungs_redact_raw_secrets: bool,
    /// True when every rung excludes approval tickets.
    pub all_rungs_exclude_approval_tickets: bool,
    /// True when every rung excludes delegated credentials.
    pub all_rungs_exclude_delegated_credentials: bool,
    /// True when every rung excludes live authority handles.
    pub all_rungs_exclude_live_authority_handles: bool,
}

/// Producer-attribution posture for replay safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderProducerAttributionSummary {
    /// Opaque producer build / instance ref.
    pub producer_ref: String,
    /// Schema version pinned by the input.
    pub schema_version: u32,
    /// Opaque integrity hash derived from the input identities.
    pub integrity_hash: String,
    /// Input capture timestamp.
    pub captured_at: String,
    /// True when producer attribution fields are non-empty.
    pub producer_attribution_complete: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe recovery-ladder lineage record per posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryLadderLineageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub recovery_ladder_lineage_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable posture id.
    pub posture_id: String,
    /// Workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque ref to the corpus the projection ingested.
    pub corpus_ref: String,
    /// Producer attribution pillar.
    pub producer_attribution: RecoveryLadderProducerAttributionSummary,
    /// Rung-sequence coverage pillar.
    pub rung_sequence_coverage: RungSequenceCoverageSummary,
    /// Trigger-disclosure pillar.
    pub trigger_disclosure: TriggerDisclosureSummary,
    /// No-rerun honesty pillar.
    pub no_rerun_honesty: NoRerunHonestySummary,
    /// User-state preservation pillar.
    pub user_state_preservation: UserStatePreservationSummary,
    /// Reversibility truth pillar.
    pub reversibility_truth: ReversibilityTruthSummary,
    /// Support-export honesty pillar.
    pub support_export_honesty: RecoverySupportExportHonestySummary,
    /// Pre-action inspection / repair hooks.
    pub inspection_hooks: Vec<RecoveryLadderInspectionHook>,
    /// Stable-qualification posture with named narrow reasons.
    pub stable_qualification: RecoveryLadderLineageQualification,
    /// Whether the record is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl RecoveryLadderLineageRecord {
    /// Returns true when the record is metadata-safe for support
    /// export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == RECOVERY_LADDER_LINEAGE_SCHEMA_REF
            && self.record_kind == RECOVERY_LADDER_LINEAGE_RECORD_KIND
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
        class: RecoveryLadderInspectionHookClass,
    ) -> Option<&RecoveryLadderInspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed recovery-ladder lineage record from a live
/// [`RecoveryLadderInputs`] envelope using the default inspection-hook
/// set.
pub fn project_recovery_ladder_lineage(
    posture_id: impl Into<String>,
    inputs: &RecoveryLadderInputs,
) -> RecoveryLadderLineageRecord {
    project_recovery_ladder_lineage_with_hooks(
        posture_id,
        inputs,
        default_recovery_ladder_inspection_hooks(),
    )
}

/// Like [`project_recovery_ladder_lineage`] but with an explicit
/// inspection-hook set (for testing degraded-hook postures).
pub fn project_recovery_ladder_lineage_with_hooks(
    posture_id: impl Into<String>,
    inputs: &RecoveryLadderInputs,
    inspection_hooks: Vec<RecoveryLadderInspectionHook>,
) -> RecoveryLadderLineageRecord {
    let posture_id: String = posture_id.into();

    let rung_sequence_coverage = project_rung_sequence_coverage(inputs);
    let trigger_disclosure = project_trigger_disclosure(&rung_sequence_coverage);
    let no_rerun_honesty = project_no_rerun_honesty(&rung_sequence_coverage);
    let user_state_preservation = project_user_state_preservation(&rung_sequence_coverage);
    let reversibility_truth = project_reversibility_truth(&rung_sequence_coverage);
    let support_export_honesty = project_support_export_honesty(inputs);
    let producer_attribution = project_producer_attribution(inputs);

    let mut narrow_reasons = Vec::new();

    if inputs.rungs.is_empty() {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::CorpusEmpty);
    }
    if !rung_sequence_coverage.all_required_rungs_present {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::RequiredRungMissing);
    }
    if !rung_sequence_coverage.all_required_steps_ordered {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::RungSequenceUnordered);
    }
    if !trigger_disclosure.all_rungs_have_trigger_disclosure {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::RungTriggerMissingDisclosure);
    }
    if !no_rerun_honesty.all_privileged_rungs_safe {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::NoRerunPostureUnsafe);
    }
    if !no_rerun_honesty.all_explicit_rungs_have_metadata {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::ExplicitActionMetadataMissing);
    }
    if !user_state_preservation.all_lossy_rungs_have_export_disclosure {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::UserStateLossUndisclosed);
    }
    if !reversibility_truth.all_checkpointed_rungs_have_checkpoint_id {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::ReversibilityCheckpointMissing);
    }
    if !reversibility_truth.all_irreversible_rungs_have_disclosure_id {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::IrreversibleRungMissingDisclosure);
    }

    let required_hooks = [
        RecoveryLadderInspectionHookClass::InspectLadderState,
        RecoveryLadderInspectionHookClass::CompareBeforeAction,
        RecoveryLadderInspectionHookClass::ExportBeforeRepair,
        RecoveryLadderInspectionHookClass::RollbackCheckpoint,
        RecoveryLadderInspectionHookClass::Export,
        RecoveryLadderInspectionHookClass::Repair,
    ];
    if !required_hooks
        .iter()
        .all(|required| hook_available(&inspection_hooks, *required))
    {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::InspectionHookUnavailable);
    }

    collect_support_export_narrows(&support_export_honesty, &mut narrow_reasons);

    if !producer_attribution.producer_attribution_complete {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::ProducerAttributionIncomplete);
    }

    if inputs.workspace_ref.trim().is_empty() || inputs.corpus_ref.trim().is_empty() {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = RecoveryLadderLineageQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        &rung_sequence_coverage,
        &no_rerun_honesty,
        &user_state_preservation,
        &stable_qualification,
    );

    RecoveryLadderLineageRecord {
        record_kind: RECOVERY_LADDER_LINEAGE_RECORD_KIND.to_owned(),
        recovery_ladder_lineage_schema_version: RECOVERY_LADDER_LINEAGE_SCHEMA_VERSION,
        schema_ref: RECOVERY_LADDER_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id,
        workspace_ref: inputs.workspace_ref.clone(),
        corpus_ref: inputs.corpus_ref.clone(),
        producer_attribution,
        rung_sequence_coverage,
        trigger_disclosure,
        no_rerun_honesty,
        user_state_preservation,
        reversibility_truth,
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

fn project_rung_sequence_coverage(inputs: &RecoveryLadderInputs) -> RungSequenceCoverageSummary {
    let rung_rows: Vec<RecoveryRungRow> = inputs.rungs.iter().map(project_rung_row).collect();
    let observed: BTreeSet<_> = rung_rows.iter().map(|row| row.rung_kind).collect();
    let all_required_rungs_present = REQUIRED_RECOVERY_RUNGS
        .iter()
        .all(|required| observed.contains(required));
    let all_required_steps_ordered = rung_rows
        .iter()
        .filter(|row| row.is_required)
        .all(|row| row.step_ordinal_matches);
    RungSequenceCoverageSummary {
        rung_rows,
        all_required_rungs_present,
        all_required_steps_ordered,
    }
}

fn project_rung_row(rung: &RecoveryRungObservation) -> RecoveryRungRow {
    let canonical_step_ordinal = rung.rung_kind.canonical_step_ordinal();
    let step_ordinal_matches = rung.declared_step_ordinal == canonical_step_ordinal;
    RecoveryRungRow {
        rung_id: rung.rung_id.clone(),
        title: rung.title.clone(),
        rung_kind: rung.rung_kind,
        declared_step_ordinal: rung.declared_step_ordinal,
        canonical_step_ordinal,
        step_ordinal_matches,
        trigger_class: rung.trigger_class,
        trigger_disclosure_id: rung.trigger_disclosure_id.clone(),
        no_rerun_posture: rung.no_rerun_posture,
        commit_action_id: rung.commit_action_id.clone(),
        commit_disclosure_id: rung.commit_disclosure_id.clone(),
        touches_privileged_surface: rung.touches_privileged_surface,
        user_state_preservation: rung.user_state_preservation,
        export_before_repair_disclosure_id: rung.export_before_repair_disclosure_id.clone(),
        reversibility: rung.reversibility,
        rollback_checkpoint_id: rung.rollback_checkpoint_id.clone(),
        irreversibility_disclosure_id: rung.irreversibility_disclosure_id.clone(),
        support_export_posture: rung.support_export.posture,
        is_required: rung.rung_kind.is_required(),
    }
}

fn project_trigger_disclosure(coverage: &RungSequenceCoverageSummary) -> TriggerDisclosureSummary {
    let all_rungs_have_trigger_disclosure = coverage
        .rung_rows
        .iter()
        .all(|row| !row.trigger_disclosure_id.trim().is_empty());
    TriggerDisclosureSummary {
        all_rungs_have_trigger_disclosure,
    }
}

fn project_no_rerun_honesty(coverage: &RungSequenceCoverageSummary) -> NoRerunHonestySummary {
    let mut all_privileged_rungs_safe = true;
    let mut all_explicit_rungs_have_metadata = true;
    for row in &coverage.rung_rows {
        if row.touches_privileged_surface && !row.no_rerun_posture.safe_for_privileged_rung() {
            all_privileged_rungs_safe = false;
        }
        if row.no_rerun_posture == NoRerunPosture::ExplicitUserActionRequired
            && (row.commit_action_id.trim().is_empty()
                || row.commit_disclosure_id.trim().is_empty())
        {
            all_explicit_rungs_have_metadata = false;
        }
    }
    NoRerunHonestySummary {
        all_privileged_rungs_safe,
        all_explicit_rungs_have_metadata,
    }
}

fn project_user_state_preservation(
    coverage: &RungSequenceCoverageSummary,
) -> UserStatePreservationSummary {
    let mut user_state_lossy_rung_count = 0usize;
    let mut all_lossy_rungs_have_export_disclosure = true;
    for row in &coverage.rung_rows {
        if row.user_state_preservation.requires_export_before_repair() {
            user_state_lossy_rung_count += 1;
            if row.export_before_repair_disclosure_id.trim().is_empty() {
                all_lossy_rungs_have_export_disclosure = false;
            }
        }
    }
    UserStatePreservationSummary {
        user_state_lossy_rung_count,
        all_lossy_rungs_have_export_disclosure,
    }
}

fn project_reversibility_truth(
    coverage: &RungSequenceCoverageSummary,
) -> ReversibilityTruthSummary {
    let mut checkpointed_rung_count = 0usize;
    let mut all_checkpointed_rungs_have_checkpoint_id = true;
    let mut irreversible_rung_count = 0usize;
    let mut all_irreversible_rungs_have_disclosure_id = true;
    for row in &coverage.rung_rows {
        match row.reversibility {
            ReversibilityClass::Reversible => {}
            ReversibilityClass::ReversibleWithCheckpoint => {
                checkpointed_rung_count += 1;
                if row.rollback_checkpoint_id.trim().is_empty() {
                    all_checkpointed_rungs_have_checkpoint_id = false;
                }
            }
            ReversibilityClass::IrreversibleWithDisclosure => {
                irreversible_rung_count += 1;
                if row.irreversibility_disclosure_id.trim().is_empty() {
                    all_irreversible_rungs_have_disclosure_id = false;
                }
            }
        }
    }
    ReversibilityTruthSummary {
        checkpointed_rung_count,
        all_checkpointed_rungs_have_checkpoint_id,
        irreversible_rung_count,
        all_irreversible_rungs_have_disclosure_id,
    }
}

fn project_support_export_honesty(
    inputs: &RecoveryLadderInputs,
) -> RecoverySupportExportHonestySummary {
    let mut all_rungs_preserve_fields = true;
    let mut all_rungs_redact_raw_secrets = true;
    let mut all_rungs_exclude_approval_tickets = true;
    let mut all_rungs_exclude_delegated_credentials = true;
    let mut all_rungs_exclude_live_authority_handles = true;

    for rung in &inputs.rungs {
        let support = rung.support_export;
        if !(support.includes_rung_kind
            && support.includes_trigger_class
            && support.includes_no_rerun_posture
            && support.includes_user_state_posture
            && support.includes_reversibility_class
            && support.includes_step_ordinal
            && support.includes_disclosure_id)
        {
            all_rungs_preserve_fields = false;
        }
        if !support.raw_secrets_excluded {
            all_rungs_redact_raw_secrets = false;
        }
        if !support.approval_tickets_excluded {
            all_rungs_exclude_approval_tickets = false;
        }
        if !support.delegated_credentials_excluded {
            all_rungs_exclude_delegated_credentials = false;
        }
        if !support.live_authority_handles_excluded {
            all_rungs_exclude_live_authority_handles = false;
        }
    }

    RecoverySupportExportHonestySummary {
        all_rungs_preserve_fields,
        all_rungs_redact_raw_secrets,
        all_rungs_exclude_approval_tickets,
        all_rungs_exclude_delegated_credentials,
        all_rungs_exclude_live_authority_handles,
    }
}

fn project_producer_attribution(
    inputs: &RecoveryLadderInputs,
) -> RecoveryLadderProducerAttributionSummary {
    let integrity_hash = compute_integrity_hash(inputs);
    let producer_attribution_complete =
        !inputs.producer_ref.trim().is_empty() && !inputs.captured_at.trim().is_empty();
    RecoveryLadderProducerAttributionSummary {
        producer_ref: inputs.producer_ref.clone(),
        schema_version: RECOVERY_LADDER_LINEAGE_SCHEMA_VERSION,
        integrity_hash,
        captured_at: inputs.captured_at.clone(),
        producer_attribution_complete,
    }
}

fn collect_support_export_narrows(
    summary: &RecoverySupportExportHonestySummary,
    narrow_reasons: &mut Vec<RecoveryLadderLineageNarrowReason>,
) {
    if !summary.all_rungs_preserve_fields {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::SupportExportFieldsDropped);
    }
    if !(summary.all_rungs_redact_raw_secrets
        && summary.all_rungs_exclude_approval_tickets
        && summary.all_rungs_exclude_delegated_credentials
        && summary.all_rungs_exclude_live_authority_handles)
    {
        narrow_reasons.push(RecoveryLadderLineageNarrowReason::SupportExportRedactionUnsafe);
    }
}

fn compute_integrity_hash(inputs: &RecoveryLadderInputs) -> String {
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
    for rung in &inputs.rungs {
        for byte in rung.rung_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(rung.rung_kind.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(rung.declared_step_ordinal as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(rung.trigger_class.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(rung.no_rerun_posture.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(rung.user_state_preservation.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(rung.reversibility.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("rll:{hash:016x}")
}

fn hook_available(
    hooks: &[RecoveryLadderInspectionHook],
    class: RecoveryLadderInspectionHookClass,
) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn build_summary(
    coverage: &RungSequenceCoverageSummary,
    no_rerun: &NoRerunHonestySummary,
    user_state: &UserStatePreservationSummary,
    qualification: &RecoveryLadderLineageQualification,
) -> String {
    if qualification.qualified {
        format!(
            "Recovery-ladder lineage proven Stable: rungs={total} privileged_safe={safe} lossy_rungs={lossy}.",
            total = coverage.rung_rows.len(),
            safe = no_rerun.all_privileged_rungs_safe,
            lossy = user_state.user_state_lossy_rung_count,
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Recovery-ladder lineage narrowed below Stable (rungs={total}): {reasons}.",
            total = coverage.rung_rows.len(),
            reasons = reasons.join(", "),
        )
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection (for headless emitter / shell status surface).
// ---------------------------------------------------------------------------

/// Returns the human-readable projection of a recovery-ladder lineage
/// record. The same projection is consumed by the workspace recovery
/// status surface, the headless CLI emitter, Help/About, and support
/// export.
pub fn recovery_ladder_lineage_lines(record: &RecoveryLadderLineageRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Recovery-ladder lineage — {} ({})",
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
        "rung_sequence_coverage: rungs={} required_present={} required_ordered={}",
        record.rung_sequence_coverage.rung_rows.len(),
        record.rung_sequence_coverage.all_required_rungs_present,
        record.rung_sequence_coverage.all_required_steps_ordered,
    ));
    lines.push("Recovery rungs:".to_owned());
    for row in &record.rung_sequence_coverage.rung_rows {
        lines.push(format!(
            "  - {kind} {id} step_declared={declared} step_canonical={canonical} step_matches={matches} trigger={trigger} no_rerun={no_rerun} privileged={privileged} user_state={user_state} reversibility={rev} required={required} support_export={posture}",
            kind = row.rung_kind.as_str(),
            id = row.rung_id,
            declared = row.declared_step_ordinal,
            canonical = row.canonical_step_ordinal,
            matches = row.step_ordinal_matches,
            trigger = row.trigger_class.as_str(),
            no_rerun = row.no_rerun_posture.as_str(),
            privileged = row.touches_privileged_surface,
            user_state = row.user_state_preservation.as_str(),
            rev = row.reversibility.as_str(),
            required = row.is_required,
            posture = row.support_export_posture.as_str(),
        ));
    }
    lines.push(format!(
        "Trigger disclosure: all_have_disclosure={}",
        record.trigger_disclosure.all_rungs_have_trigger_disclosure,
    ));
    lines.push(format!(
        "No-rerun honesty: all_privileged_safe={safe} all_explicit_have_metadata={meta}",
        safe = record.no_rerun_honesty.all_privileged_rungs_safe,
        meta = record.no_rerun_honesty.all_explicit_rungs_have_metadata,
    ));
    lines.push(format!(
        "User-state preservation: lossy_rungs={count} all_have_export_disclosure={disclosure}",
        count = record.user_state_preservation.user_state_lossy_rung_count,
        disclosure = record
            .user_state_preservation
            .all_lossy_rungs_have_export_disclosure,
    ));
    lines.push(format!(
        "Reversibility truth: checkpointed_rungs={cp} all_have_checkpoint_id={cp_id} irreversible_rungs={irr} all_have_disclosure_id={irr_id}",
        cp = record.reversibility_truth.checkpointed_rung_count,
        cp_id = record
            .reversibility_truth
            .all_checkpointed_rungs_have_checkpoint_id,
        irr = record.reversibility_truth.irreversible_rung_count,
        irr_id = record
            .reversibility_truth
            .all_irreversible_rungs_have_disclosure_id,
    ));
    lines.push(format!(
        "Support-export honesty: preserve_fields={fields} redact_secrets={secrets} exclude_approvals={approvals} exclude_credentials={credentials} exclude_authority={authority}",
        fields = record.support_export_honesty.all_rungs_preserve_fields,
        secrets = record.support_export_honesty.all_rungs_redact_raw_secrets,
        approvals = record
            .support_export_honesty
            .all_rungs_exclude_approval_tickets,
        credentials = record
            .support_export_honesty
            .all_rungs_exclude_delegated_credentials,
        authority = record
            .support_export_honesty
            .all_rungs_exclude_live_authority_handles,
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
