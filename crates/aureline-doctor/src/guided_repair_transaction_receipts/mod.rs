//! Guided repair transactions and reversibility receipts for the M5 failure
//! families.
//!
//! This module owns the canonical packet that turns each guided repair into an
//! auditable **repair transaction receipt**. A receipt records the transaction's
//! full declaration *before mutation begins* — the
//! [`RepairTransactionReceipt::repair_id`], the
//! [`RepairTransactionReceipt::initiating_findings`] that justify it, the
//! [`RepairTransactionReceipt::failure_family`] it belongs to, the impacted
//! state classes, the preconditions, the disclosed
//! [`RepairTransactionReceipt::host_boundary`], the
//! [`CheckpointDisclosure`] (or its explicit absence), the verification plan, and
//! the [`ReversalClass`] — and then the staged outcome of the
//! review/dry-run/checkpoint/apply/verify and (when needed)
//! rollback-or-compensate flow.
//!
//! The receipt's terminal [`CompletionState`] is never a generic
//! success/failure: it distinguishes *fixed*, *partially repaired*,
//! *reduced-but-not-resolved*, *verification-inconclusive*, *exact rollback*, and
//! *compensating rollback*, and links back to the initiating findings, the
//! checkpoint, the affected objects, the verification results, and the
//! support/export paths.
//!
//! Two guardrails are enforced by construction and re-checked by
//! [`RepairTransactionReceipt::validate`]:
//!
//! - **No hidden reset of durable user state.** A transaction that mutates
//!   durable user state must either carry a checkpoint or be an explicitly
//!   guarded irreversible repair that offers support/export paths — it can never
//!   silently wipe durable state.
//! - **No false promise of reversibility.** When no checkpoint exists, the
//!   receipt says so, its reversal class may not claim clean/snapshot
//!   reversibility, it offers support/export paths, and it can never claim an
//!   *exact* rollback (exact rollback requires a checkpoint).
//!
//! The packet also pins **cross-surface parity**: each receipt records the
//! [`ParitySurface`]s it renders on and the locale-invariant
//! [`RepairTransactionReceipt::machine_meaning_keys`], so the desktop repair
//! receipt, the CLI/headless row, the support export, the incident packet, and
//! the public-truth surface carry the same repair id, failure family, completion
//! state, and reversal class without localized prose changing machine meaning.
//!
//! The packet is checked in at
//! `artifacts/doctor/m5/project-doctor-repair-transaction-receipts.json` and
//! embedded here, so this typed consumer and any CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no credential bodies, raw provider payloads, or mount/port/tunnel
//! secrets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported repair-transaction-receipts packet schema version.
pub const PROJECT_DOCTOR_REPAIR_TRANSACTION_RECEIPTS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const PROJECT_DOCTOR_REPAIR_TRANSACTION_RECEIPTS_RECORD_KIND: &str =
    "project_doctor_repair_transaction_receipts";

/// Repo-relative path to the checked-in packet.
pub const PROJECT_DOCTOR_REPAIR_TRANSACTION_RECEIPTS_PATH: &str =
    "artifacts/doctor/m5/project-doctor-repair-transaction-receipts.json";

/// Repo-relative path to the boundary schema.
pub const PROJECT_DOCTOR_REPAIR_TRANSACTION_RECEIPTS_SCHEMA_REF: &str =
    "schemas/doctor/project-doctor-repair-transaction-receipts.schema.json";

/// Repo-relative path to the companion document.
pub const PROJECT_DOCTOR_REPAIR_TRANSACTION_RECEIPTS_DOC_REF: &str =
    "docs/doctor/m5/project-doctor-repair-transaction-receipts.md";

/// Stable finding-code prefix every initiating finding must use.
pub const DOCTOR_FINDING_PREFIX: &str = "doctor.finding.";

/// Stable repair-id prefix every transaction must use.
pub const DOCTOR_REPAIR_PREFIX: &str = "repair.";

/// Stable receipt-id prefix every receipt must use.
pub const RECEIPT_ID_PREFIX: &str = "receipt:";

/// Canonical, locale-invariant machine-meaning keys every receipt must carry, so
/// localized prose can never silently change what a surface means.
pub const REQUIRED_MACHINE_MEANING_KEYS: [&str; 4] = [
    "repair_id",
    "failure_family",
    "completion_state",
    "reversal_class",
];

/// Embedded checked-in packet JSON.
pub const PROJECT_DOCTOR_REPAIR_TRANSACTION_RECEIPTS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/doctor/m5/project-doctor-repair-transaction-receipts.json"
));

/// Generic, non-actionable detail tokens that may never stand in for an explicit
/// repair-unavailable or block reason.
const GENERIC_DETAIL_TOKENS: [&str; 9] = [
    "unavailable",
    "error",
    "failed",
    "failure",
    "unknown",
    "unknown_error",
    "generic_failure",
    "n_a",
    "na",
];

// ---------------------------------------------------------------------------
// Closed vocabularies
// ---------------------------------------------------------------------------

/// The M5 failure family a repair transaction addresses.
///
/// Each family is a distinct, named recovery lane added in M5 rather than folded
/// into a generic "repair" bucket.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureFamily {
    /// Notebook kernel/runtime session state.
    NotebookKernel,
    /// Request/API auth and environment drift.
    RequestApi,
    /// Database connection/target drift.
    DatabaseConnection,
    /// Profiler/replay storage or symbol problems.
    ProfilerReplay,
    /// Remote preview-route expiry.
    PreviewRoute,
    /// Sync/offboarding/device-registry state.
    SyncOffboarding,
    /// Companion handoff integrity.
    CompanionHandoff,
    /// Incident packet handoff integrity.
    IncidentPacket,
}

impl FailureFamily {
    /// Every failure family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::NotebookKernel,
        Self::RequestApi,
        Self::DatabaseConnection,
        Self::ProfilerReplay,
        Self::PreviewRoute,
        Self::SyncOffboarding,
        Self::CompanionHandoff,
        Self::IncidentPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookKernel => "notebook_kernel",
            Self::RequestApi => "request_api",
            Self::DatabaseConnection => "database_connection",
            Self::ProfilerReplay => "profiler_replay",
            Self::PreviewRoute => "preview_route",
            Self::SyncOffboarding => "sync_offboarding",
            Self::CompanionHandoff => "companion_handoff",
            Self::IncidentPacket => "incident_packet",
        }
    }
}

impl fmt::Display for FailureFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// The host/boundary a repair transaction touches, disclosed before apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostBoundary {
    /// The local workspace on the user's machine.
    LocalWorkspace,
    /// A remote host the workspace is attached to.
    RemoteHost,
    /// A container the workspace runs in.
    Container,
    /// A devcontainer the workspace runs in.
    Devcontainer,
    /// A preview/forwarding tunnel.
    Tunnel,
    /// A managed external service.
    ManagedService,
}

impl HostBoundary {
    /// Every host boundary, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalWorkspace,
        Self::RemoteHost,
        Self::Container,
        Self::Devcontainer,
        Self::Tunnel,
        Self::ManagedService,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::RemoteHost => "remote_host",
            Self::Container => "container",
            Self::Devcontainer => "devcontainer",
            Self::Tunnel => "tunnel",
            Self::ManagedService => "managed_service",
        }
    }
}

/// The kind of checkpoint a transaction captured before apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointKind {
    /// A transactional snapshot reversed by rolling back the transaction.
    TransactionalSnapshot,
    /// A filesystem/state snapshot reversed by restoring it.
    FilesystemSnapshot,
    /// An exported copy of the prior state, reversed by re-import.
    StateExport,
    /// No checkpoint was captured.
    None,
}

impl CheckpointKind {
    /// Every checkpoint kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::TransactionalSnapshot,
        Self::FilesystemSnapshot,
        Self::StateExport,
        Self::None,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TransactionalSnapshot => "transactional_snapshot",
            Self::FilesystemSnapshot => "filesystem_snapshot",
            Self::StateExport => "state_export",
            Self::None => "none",
        }
    }

    /// Whether this kind names a real, captured checkpoint.
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// How a repair transaction can be undone.
///
/// The class is declared before apply so a user can reason about reversibility
/// up front, and it must agree with the captured checkpoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReversalClass {
    /// The repair reverses cleanly by rolling back its transaction.
    ReversibleTransactional,
    /// The repair reverses by restoring a captured snapshot/export.
    ReversibleWithSnapshot,
    /// The repair has no exact undo; only compensating actions can recover.
    CompensatingOnly,
    /// The repair is irreversible and is gated behind an explicit guard.
    IrreversibleGuarded,
}

impl ReversalClass {
    /// Every reversal class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReversibleTransactional,
        Self::ReversibleWithSnapshot,
        Self::CompensatingOnly,
        Self::IrreversibleGuarded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReversibleTransactional => "reversible_transactional",
            Self::ReversibleWithSnapshot => "reversible_with_snapshot",
            Self::CompensatingOnly => "compensating_only",
            Self::IrreversibleGuarded => "irreversible_guarded",
        }
    }

    /// Whether this class promises an *exact* reversal backed by a checkpoint.
    pub const fn requires_checkpoint(self) -> bool {
        matches!(
            self,
            Self::ReversibleTransactional | Self::ReversibleWithSnapshot
        )
    }
}

/// A stage in the guided repair transaction flow.
///
/// The stages run in [`RepairStage::order`] order. [`RepairStage::Rollback`] and
/// [`RepairStage::Compensate`] are the two mutually exclusive terminal recovery
/// stages and share the final order slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairStage {
    /// Declare and review the transaction before any mutation.
    Review,
    /// Simulate the repair without mutating state.
    DryRun,
    /// Capture the checkpoint that backs reversal.
    Checkpoint,
    /// Apply the mutation.
    Apply,
    /// Verify the mutation against the verification plan.
    Verify,
    /// Reverse the mutation exactly from a checkpoint.
    Rollback,
    /// Recover with compensating actions when no exact reversal exists.
    Compensate,
}

impl RepairStage {
    /// Every stage, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Review,
        Self::DryRun,
        Self::Checkpoint,
        Self::Apply,
        Self::Verify,
        Self::Rollback,
        Self::Compensate,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Review => "review",
            Self::DryRun => "dry_run",
            Self::Checkpoint => "checkpoint",
            Self::Apply => "apply",
            Self::Verify => "verify",
            Self::Rollback => "rollback",
            Self::Compensate => "compensate",
        }
    }

    /// The canonical position of this stage in the flow. The two terminal
    /// recovery stages share the final slot because at most one of them runs.
    pub const fn order(self) -> u8 {
        match self {
            Self::Review => 0,
            Self::DryRun => 1,
            Self::Checkpoint => 2,
            Self::Apply => 3,
            Self::Verify => 4,
            Self::Rollback | Self::Compensate => 5,
        }
    }
}

/// The outcome of a single executed stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageStatus {
    /// The stage completed cleanly.
    Passed,
    /// The stage completed for only part of its scope.
    Partial,
    /// The stage failed.
    Failed,
    /// The stage was deliberately skipped.
    Skipped,
    /// The stage ran but could not reach a verdict.
    Inconclusive,
}

impl StageStatus {
    /// Every stage status, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Passed,
        Self::Partial,
        Self::Failed,
        Self::Skipped,
        Self::Inconclusive,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Partial => "partial",
            Self::Failed => "failed",
            Self::Skipped => "skipped",
            Self::Inconclusive => "inconclusive",
        }
    }
}

/// The verdict of a single verification-plan check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationOutcome {
    /// The check confirmed the repair.
    Passed,
    /// The check shows the finding persists.
    Failed,
    /// The check could not reach a verdict.
    Inconclusive,
}

impl VerificationOutcome {
    /// Every verification outcome, in declaration order.
    pub const ALL: [Self; 3] = [Self::Passed, Self::Failed, Self::Inconclusive];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Inconclusive => "inconclusive",
        }
    }
}

/// The terminal completion state of a repair transaction.
///
/// Every state is a distinct, named outcome rather than a generic
/// success/failure toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletionState {
    /// The repair applied and verified cleanly.
    Fixed,
    /// The repair applied to part of its scope only.
    PartiallyRepaired,
    /// The repair applied but the finding persists in reduced form.
    ReducedButNotResolved,
    /// Verification ran but could not confirm the repair.
    VerificationInconclusive,
    /// The repair was reversed exactly from a checkpoint.
    RolledBackExact,
    /// The repair was reversed by compensating actions (no exact checkpoint).
    RolledBackCompensating,
}

impl CompletionState {
    /// Every completion state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Fixed,
        Self::PartiallyRepaired,
        Self::ReducedButNotResolved,
        Self::VerificationInconclusive,
        Self::RolledBackExact,
        Self::RolledBackCompensating,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fixed => "fixed",
            Self::PartiallyRepaired => "partially_repaired",
            Self::ReducedButNotResolved => "reduced_but_not_resolved",
            Self::VerificationInconclusive => "verification_inconclusive",
            Self::RolledBackExact => "rolled_back_exact",
            Self::RolledBackCompensating => "rolled_back_compensating",
        }
    }

    /// Whether this state reflects a transaction that was reversed.
    pub const fn is_rollback(self) -> bool {
        matches!(self, Self::RolledBackExact | Self::RolledBackCompensating)
    }
}

/// A surface a receipt must render on with identical machine meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParitySurface {
    /// Desktop repair receipt pane.
    DesktopReceipt,
    /// Interactive CLI receipt row.
    CliRow,
    /// Headless JSON row.
    HeadlessJson,
    /// Support export row.
    SupportExport,
    /// Incident packet row.
    IncidentPacket,
    /// Public-truth/release surface row.
    PublicTruth,
}

impl ParitySurface {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopReceipt,
        Self::CliRow,
        Self::HeadlessJson,
        Self::SupportExport,
        Self::IncidentPacket,
        Self::PublicTruth,
    ];

    /// The core surfaces every receipt must render on to be cross-surface stable.
    pub const CORE: [Self; 4] = [
        Self::DesktopReceipt,
        Self::CliRow,
        Self::HeadlessJson,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopReceipt => "desktop_receipt",
            Self::CliRow => "cli_row",
            Self::HeadlessJson => "headless_json",
            Self::SupportExport => "support_export",
            Self::IncidentPacket => "incident_packet",
            Self::PublicTruth => "public_truth",
        }
    }
}

// ---------------------------------------------------------------------------
// Records
// ---------------------------------------------------------------------------

/// The checkpoint a transaction captured (or its explicit absence) before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointDisclosure {
    /// Whether a checkpoint was captured.
    pub present: bool,
    /// The kind of checkpoint (`none` iff [`CheckpointDisclosure::present`] is
    /// false).
    pub checkpoint_kind: CheckpointKind,
    /// Opaque, redaction-safe checkpoint reference (empty iff absent).
    #[serde(default)]
    pub checkpoint_ref: String,
}

impl CheckpointDisclosure {
    /// Whether the disclosure is internally consistent: present iff a real kind
    /// and a non-empty ref.
    pub fn is_consistent(&self) -> bool {
        let has_ref = !self.checkpoint_ref.trim().is_empty();
        self.present == self.checkpoint_kind.is_present() && self.present == has_ref
    }
}

/// One executed stage and its outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageRecord {
    /// The stage that ran.
    pub stage: RepairStage,
    /// The stage outcome.
    pub status: StageStatus,
    /// Reviewer-safe note about the stage.
    pub note: String,
}

/// One verification-plan check result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationResult {
    /// Stable check id.
    pub check_id: String,
    /// The verdict.
    pub outcome: VerificationOutcome,
}

/// One guided repair transaction receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepairTransactionReceipt {
    /// Stable receipt id (must start with [`RECEIPT_ID_PREFIX`]).
    pub receipt_id: String,
    /// Stable repair id (must start with [`DOCTOR_REPAIR_PREFIX`]).
    pub repair_id: String,
    /// The failure family this transaction addresses.
    pub failure_family: FailureFamily,
    /// Finding codes that initiated the transaction (at least one, each using
    /// [`DOCTOR_FINDING_PREFIX`]).
    pub initiating_findings: Vec<String>,
    /// State classes the transaction may change (at least one).
    pub impacted_state_classes: Vec<String>,
    /// Preconditions checked before apply (at least one).
    pub preconditions: Vec<String>,
    /// Disclosed host/boundary the transaction touches.
    pub host_boundary: HostBoundary,
    /// Opaque, redaction-safe boundary scope reference (mount/port/tunnel/etc.).
    pub boundary_scope_ref: String,
    /// Whether the transaction mutates durable user state.
    pub mutates_durable_user_state: bool,
    /// The checkpoint (or explicit absence) backing reversal.
    pub checkpoint: CheckpointDisclosure,
    /// The declared reversal class.
    pub reversal_class: ReversalClass,
    /// The verification plan checked after apply (at least one item).
    pub verification_plan: Vec<String>,
    /// The executed stages and their outcomes (in flow order).
    pub stages: Vec<StageRecord>,
    /// The terminal completion state.
    pub completion_state: CompletionState,
    /// Whether partial-success handling applies to this receipt.
    pub partial_success: bool,
    /// Affected objects the transaction touched (at least one opaque ref).
    pub affected_objects: Vec<String>,
    /// Verification results linked from the receipt (at least one).
    pub verification_results: Vec<VerificationResult>,
    /// Support/export paths the receipt links to (at least one).
    pub support_paths: Vec<String>,
    /// Surfaces this receipt renders on with identical machine meaning.
    pub parity_surfaces: Vec<ParitySurface>,
    /// Locale-invariant JSON keys whose values may never change with locale.
    pub machine_meaning_keys: Vec<String>,
    /// Human-readable explanation (additive, localizable prose).
    pub explanation: String,
    /// Redaction class (must be metadata-safe).
    pub redaction_class: String,
    /// Whether raw private material is excluded (must be true).
    pub raw_private_material_excluded: bool,
    /// Reviewer-safe summary.
    pub summary: String,
}

impl RepairTransactionReceipt {
    /// Whether the receipt renders on every core surface, so desktop, CLI,
    /// headless, and support reason about the same machine meaning.
    pub fn is_cross_surface_stable(&self) -> bool {
        ParitySurface::CORE
            .iter()
            .all(|surface| self.parity_surfaces.contains(surface))
    }

    /// The recorded stage matching `stage`, if any.
    pub fn stage(&self, stage: RepairStage) -> Option<&StageRecord> {
        self.stages.iter().find(|record| record.stage == stage)
    }

    /// Whether a stage was recorded.
    pub fn has_stage(&self, stage: RepairStage) -> bool {
        self.stage(stage).is_some()
    }

    /// Whether the receipt offers at least one support/export path.
    pub fn offers_support_path(&self) -> bool {
        self.support_paths.iter().any(|p| !p.trim().is_empty())
    }

    /// Whether durable-state mutation is safely guarded: a checkpoint exists, or
    /// the repair is an explicitly guarded irreversible repair offering support
    /// paths. Used to enforce "no hidden reset of durable user state".
    pub fn durable_state_is_guarded(&self) -> bool {
        if !self.mutates_durable_user_state {
            return true;
        }
        self.checkpoint.present
            || (self.reversal_class == ReversalClass::IrreversibleGuarded
                && self.offers_support_path())
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDoctorRepairTransactionReceiptsSummary {
    /// Number of receipts.
    pub receipt_count: usize,
    /// Receipts with a captured checkpoint.
    pub receipts_with_checkpoint: usize,
    /// Receipts disclosing no checkpoint.
    pub receipts_without_checkpoint: usize,
    /// Receipts that mutate durable user state.
    pub durable_state_receipts: usize,
    /// Receipts whose terminal state is a rollback/compensate.
    pub rolled_back_receipts: usize,
    /// Receipts that render stably across the core surfaces.
    pub cross_surface_stable_receipts: usize,
    /// Receipts in completion state `fixed`.
    pub fixed_receipts: usize,
    /// Receipts in completion state `partially_repaired`.
    pub partially_repaired_receipts: usize,
    /// Receipts in completion state `reduced_but_not_resolved`.
    pub reduced_receipts: usize,
    /// Receipts in completion state `verification_inconclusive`.
    pub verification_inconclusive_receipts: usize,
    /// Receipts in completion state `rolled_back_exact`.
    pub rolled_back_exact_receipts: usize,
    /// Receipts in completion state `rolled_back_compensating`.
    pub rolled_back_compensating_receipts: usize,
}

/// A redaction-safe export row projected from a receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorRepairTransactionReceiptsExportRow {
    /// Receipt id.
    pub receipt_id: String,
    /// Repair id.
    pub repair_id: String,
    /// Failure-family token.
    pub failure_family: String,
    /// Initiating finding codes.
    pub initiating_findings: Vec<String>,
    /// Host-boundary token.
    pub host_boundary: String,
    /// Opaque boundary scope ref.
    pub boundary_scope_ref: String,
    /// Whether a checkpoint was captured.
    pub checkpoint_present: bool,
    /// Checkpoint-kind token.
    pub checkpoint_kind: String,
    /// Opaque checkpoint ref (empty when absent).
    pub checkpoint_ref: String,
    /// Reversal-class token.
    pub reversal_class: String,
    /// Whether the transaction mutates durable user state.
    pub mutates_durable_user_state: bool,
    /// Completion-state token.
    pub completion_state: String,
    /// Whether partial-success handling applies.
    pub partial_success: bool,
    /// Affected object refs.
    pub affected_objects: Vec<String>,
    /// Support/export paths.
    pub support_paths: Vec<String>,
    /// Whether the receipt renders stably across the core surfaces.
    pub cross_surface_stable: bool,
    /// Human-readable explanation.
    pub explanation: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectDoctorRepairTransactionReceiptsExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<ProjectDoctorRepairTransactionReceiptsExportRow>,
    /// Receipts with a captured checkpoint.
    pub with_checkpoint_count: usize,
    /// Receipts disclosing no checkpoint.
    pub without_checkpoint_count: usize,
    /// Receipts whose terminal state is a rollback/compensate.
    pub rolled_back_count: usize,
    /// Receipts that render stably across the core surfaces.
    pub cross_surface_stable_count: usize,
}

/// The typed repair-transaction-receipts packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectDoctorRepairTransactionReceipts {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed failure-family vocabulary.
    pub failure_families: Vec<FailureFamily>,
    /// Closed host-boundary vocabulary.
    pub host_boundaries: Vec<HostBoundary>,
    /// Closed checkpoint-kind vocabulary.
    pub checkpoint_kinds: Vec<CheckpointKind>,
    /// Closed reversal-class vocabulary.
    pub reversal_classes: Vec<ReversalClass>,
    /// Closed repair-stage vocabulary.
    pub repair_stages: Vec<RepairStage>,
    /// Closed stage-status vocabulary.
    pub stage_statuses: Vec<StageStatus>,
    /// Closed verification-outcome vocabulary.
    pub verification_outcomes: Vec<VerificationOutcome>,
    /// Closed completion-state vocabulary.
    pub completion_states: Vec<CompletionState>,
    /// Closed parity-surface vocabulary.
    pub parity_surfaces: Vec<ParitySurface>,
    /// Repair transaction receipts.
    #[serde(default)]
    pub receipts: Vec<RepairTransactionReceipt>,
    /// Summary counts.
    pub summary: ProjectDoctorRepairTransactionReceiptsSummary,
}

impl ProjectDoctorRepairTransactionReceipts {
    /// Receipts in a given completion state.
    pub fn receipts_in_state(
        &self,
        state: CompletionState,
    ) -> impl Iterator<Item = &RepairTransactionReceipt> {
        self.receipts
            .iter()
            .filter(move |r| r.completion_state == state)
    }

    /// Recomputes the summary block from the receipts.
    pub fn computed_summary(&self) -> ProjectDoctorRepairTransactionReceiptsSummary {
        let count_state = |state: CompletionState| {
            self.receipts
                .iter()
                .filter(|r| r.completion_state == state)
                .count()
        };
        ProjectDoctorRepairTransactionReceiptsSummary {
            receipt_count: self.receipts.len(),
            receipts_with_checkpoint: self
                .receipts
                .iter()
                .filter(|r| r.checkpoint.present)
                .count(),
            receipts_without_checkpoint: self
                .receipts
                .iter()
                .filter(|r| !r.checkpoint.present)
                .count(),
            durable_state_receipts: self
                .receipts
                .iter()
                .filter(|r| r.mutates_durable_user_state)
                .count(),
            rolled_back_receipts: self
                .receipts
                .iter()
                .filter(|r| r.completion_state.is_rollback())
                .count(),
            cross_surface_stable_receipts: self
                .receipts
                .iter()
                .filter(|r| r.is_cross_surface_stable())
                .count(),
            fixed_receipts: count_state(CompletionState::Fixed),
            partially_repaired_receipts: count_state(CompletionState::PartiallyRepaired),
            reduced_receipts: count_state(CompletionState::ReducedButNotResolved),
            verification_inconclusive_receipts: count_state(
                CompletionState::VerificationInconclusive,
            ),
            rolled_back_exact_receipts: count_state(CompletionState::RolledBackExact),
            rolled_back_compensating_receipts: count_state(CompletionState::RolledBackCompensating),
        }
    }

    /// Produces an export projection that downstream surfaces — Help/About,
    /// docs/help, support exports, incident packets, and release/public-truth
    /// packets — render instead of restating receipt text by hand.
    pub fn export_projection(&self) -> ProjectDoctorRepairTransactionReceiptsExportProjection {
        let rows = self
            .receipts
            .iter()
            .map(|receipt| ProjectDoctorRepairTransactionReceiptsExportRow {
                receipt_id: receipt.receipt_id.clone(),
                repair_id: receipt.repair_id.clone(),
                failure_family: receipt.failure_family.as_str().to_owned(),
                initiating_findings: receipt.initiating_findings.clone(),
                host_boundary: receipt.host_boundary.as_str().to_owned(),
                boundary_scope_ref: receipt.boundary_scope_ref.clone(),
                checkpoint_present: receipt.checkpoint.present,
                checkpoint_kind: receipt.checkpoint.checkpoint_kind.as_str().to_owned(),
                checkpoint_ref: receipt.checkpoint.checkpoint_ref.clone(),
                reversal_class: receipt.reversal_class.as_str().to_owned(),
                mutates_durable_user_state: receipt.mutates_durable_user_state,
                completion_state: receipt.completion_state.as_str().to_owned(),
                partial_success: receipt.partial_success,
                affected_objects: receipt.affected_objects.clone(),
                support_paths: receipt.support_paths.clone(),
                cross_surface_stable: receipt.is_cross_surface_stable(),
                explanation: receipt.explanation.clone(),
            })
            .collect();
        ProjectDoctorRepairTransactionReceiptsExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            with_checkpoint_count: self
                .receipts
                .iter()
                .filter(|r| r.checkpoint.present)
                .count(),
            without_checkpoint_count: self
                .receipts
                .iter()
                .filter(|r| !r.checkpoint.present)
                .count(),
            rolled_back_count: self
                .receipts
                .iter()
                .filter(|r| r.completion_state.is_rollback())
                .count(),
            cross_surface_stable_count: self
                .receipts
                .iter()
                .filter(|r| r.is_cross_surface_stable())
                .count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<ProjectDoctorRepairTransactionReceiptsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen = BTreeSet::new();
        for receipt in &self.receipts {
            if !seen.insert(receipt.receipt_id.clone()) {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::DuplicateReceiptId {
                        receipt_id: receipt.receipt_id.clone(),
                    },
                );
            }
            self.validate_receipt(receipt, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(ProjectDoctorRepairTransactionReceiptsViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(
        &self,
        violations: &mut Vec<ProjectDoctorRepairTransactionReceiptsViolation>,
    ) {
        if self.schema_version != PROJECT_DOCTOR_REPAIR_TRANSACTION_RECEIPTS_SCHEMA_VERSION {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != PROJECT_DOCTOR_REPAIR_TRANSACTION_RECEIPTS_RECORD_KIND {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("schema_ref", &self.schema_ref),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::EmptyField {
                        id: "<packet>".to_owned(),
                        field_name: field,
                    },
                );
            }
        }
        for (field, ok) in [
            (
                "failure_families",
                self.failure_families == FailureFamily::ALL.to_vec(),
            ),
            (
                "host_boundaries",
                self.host_boundaries == HostBoundary::ALL.to_vec(),
            ),
            (
                "checkpoint_kinds",
                self.checkpoint_kinds == CheckpointKind::ALL.to_vec(),
            ),
            (
                "reversal_classes",
                self.reversal_classes == ReversalClass::ALL.to_vec(),
            ),
            (
                "repair_stages",
                self.repair_stages == RepairStage::ALL.to_vec(),
            ),
            (
                "stage_statuses",
                self.stage_statuses == StageStatus::ALL.to_vec(),
            ),
            (
                "verification_outcomes",
                self.verification_outcomes == VerificationOutcome::ALL.to_vec(),
            ),
            (
                "completion_states",
                self.completion_states == CompletionState::ALL.to_vec(),
            ),
            (
                "parity_surfaces",
                self.parity_surfaces == ParitySurface::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::ClosedVocabularyMismatch {
                        field,
                    },
                );
            }
        }
    }

    fn validate_receipt(
        &self,
        receipt: &RepairTransactionReceipt,
        violations: &mut Vec<ProjectDoctorRepairTransactionReceiptsViolation>,
    ) {
        let id = receipt.receipt_id.clone();

        for (field, value) in [
            ("receipt_id", &receipt.receipt_id),
            ("repair_id", &receipt.repair_id),
            ("boundary_scope_ref", &receipt.boundary_scope_ref),
            ("explanation", &receipt.explanation),
            ("summary", &receipt.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::EmptyField {
                        id: id.clone(),
                        field_name: field,
                    },
                );
            }
        }

        if !receipt.receipt_id.starts_with(RECEIPT_ID_PREFIX) {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::ReceiptIdPrefix {
                    receipt_id: id.clone(),
                },
            );
        }
        if !receipt.repair_id.starts_with(DOCTOR_REPAIR_PREFIX) {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::RepairIdPrefix {
                    receipt_id: id.clone(),
                },
            );
        }

        // Every transaction must be justified by at least one prefixed finding.
        if receipt.initiating_findings.is_empty() {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::NoInitiatingFinding {
                    receipt_id: id.clone(),
                },
            );
        }
        for finding in &receipt.initiating_findings {
            if !finding.starts_with(DOCTOR_FINDING_PREFIX) {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::FindingCodePrefix {
                        receipt_id: id.clone(),
                        finding: finding.clone(),
                    },
                );
            }
        }

        // The declaration must be complete before mutation: impacted state
        // classes, preconditions, and a verification plan are all required.
        for (field, empty) in [
            (
                "impacted_state_classes",
                receipt.impacted_state_classes.is_empty(),
            ),
            ("preconditions", receipt.preconditions.is_empty()),
            ("verification_plan", receipt.verification_plan.is_empty()),
            ("affected_objects", receipt.affected_objects.is_empty()),
            (
                "verification_results",
                receipt.verification_results.is_empty(),
            ),
            ("stages", receipt.stages.is_empty()),
        ] {
            if empty {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::RequiredListEmpty {
                        receipt_id: id.clone(),
                        field_name: field,
                    },
                );
            }
        }

        // Receipts always link a support/export path, so a user is never left
        // without a recovery route.
        if !receipt.offers_support_path() {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::NoSupportPath {
                    receipt_id: id.clone(),
                },
            );
        }

        self.validate_checkpoint(receipt, &mut *violations);
        self.validate_stages(receipt, &mut *violations);
        self.validate_completion(receipt, &mut *violations);
        self.validate_guardrails(receipt, &mut *violations);
        self.validate_parity_and_safety(receipt, &mut *violations);
    }

    fn validate_checkpoint(
        &self,
        receipt: &RepairTransactionReceipt,
        violations: &mut Vec<ProjectDoctorRepairTransactionReceiptsViolation>,
    ) {
        let id = receipt.receipt_id.clone();
        if !receipt.checkpoint.is_consistent() {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::CheckpointInconsistent {
                    receipt_id: id.clone(),
                },
            );
        }

        // Reversal class and checkpoint must agree: a class that promises an exact
        // reversal must be backed by the matching checkpoint kind.
        match receipt.reversal_class {
            ReversalClass::ReversibleTransactional => {
                if receipt.checkpoint.checkpoint_kind != CheckpointKind::TransactionalSnapshot {
                    violations.push(
                        ProjectDoctorRepairTransactionReceiptsViolation::ReversalCheckpointMismatch {
                            receipt_id: id.clone(),
                            reversal_class: receipt.reversal_class.as_str(),
                            checkpoint_kind: receipt.checkpoint.checkpoint_kind.as_str(),
                        },
                    );
                }
            }
            ReversalClass::ReversibleWithSnapshot => {
                if !matches!(
                    receipt.checkpoint.checkpoint_kind,
                    CheckpointKind::FilesystemSnapshot | CheckpointKind::StateExport
                ) {
                    violations.push(
                        ProjectDoctorRepairTransactionReceiptsViolation::ReversalCheckpointMismatch {
                            receipt_id: id.clone(),
                            reversal_class: receipt.reversal_class.as_str(),
                            checkpoint_kind: receipt.checkpoint.checkpoint_kind.as_str(),
                        },
                    );
                }
            }
            ReversalClass::CompensatingOnly | ReversalClass::IrreversibleGuarded => {}
        }
    }

    fn validate_stages(
        &self,
        receipt: &RepairTransactionReceipt,
        violations: &mut Vec<ProjectDoctorRepairTransactionReceiptsViolation>,
    ) {
        let id = receipt.receipt_id.clone();

        // Every transaction begins with review and reaches apply and verify.
        for required in [RepairStage::Review, RepairStage::Apply, RepairStage::Verify] {
            if !receipt.has_stage(required) {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::MissingRequiredStage {
                        receipt_id: id.clone(),
                        stage: required.as_str(),
                    },
                );
            }
        }

        // Stages run in canonical order with no duplicates; rollback and
        // compensate are mutually exclusive.
        let mut last_order: Option<u8> = None;
        let mut seen = BTreeSet::new();
        for record in &receipt.stages {
            if !seen.insert(record.stage) {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::DuplicateStage {
                        receipt_id: id.clone(),
                        stage: record.stage.as_str(),
                    },
                );
            }
            if let Some(prev) = last_order {
                if record.stage.order() <= prev {
                    violations.push(
                        ProjectDoctorRepairTransactionReceiptsViolation::StagesOutOfOrder {
                            receipt_id: id.clone(),
                            stage: record.stage.as_str(),
                        },
                    );
                }
            }
            last_order = Some(record.stage.order());
        }
        if receipt.has_stage(RepairStage::Rollback) && receipt.has_stage(RepairStage::Compensate) {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::RollbackAndCompensate {
                    receipt_id: id.clone(),
                },
            );
        }

        // The checkpoint stage is present iff a checkpoint was captured.
        if receipt.has_stage(RepairStage::Checkpoint) != receipt.checkpoint.present {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::CheckpointStageMismatch {
                    receipt_id: id.clone(),
                },
            );
        }
    }

    fn validate_completion(
        &self,
        receipt: &RepairTransactionReceipt,
        violations: &mut Vec<ProjectDoctorRepairTransactionReceiptsViolation>,
    ) {
        let id = receipt.receipt_id.clone();
        let verify_status = receipt.stage(RepairStage::Verify).map(|s| s.status);
        let apply_status = receipt.stage(RepairStage::Apply).map(|s| s.status);
        let has_inconclusive_result = receipt
            .verification_results
            .iter()
            .any(|r| r.outcome == VerificationOutcome::Inconclusive);
        let has_failed_result = receipt
            .verification_results
            .iter()
            .any(|r| r.outcome == VerificationOutcome::Failed);
        let all_results_passed = !receipt.verification_results.is_empty()
            && receipt
                .verification_results
                .iter()
                .all(|r| r.outcome == VerificationOutcome::Passed);

        let bad = |violations: &mut Vec<_>| {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::CompletionStateInconsistent {
                    receipt_id: id.clone(),
                    completion_state: receipt.completion_state.as_str(),
                },
            );
        };

        match receipt.completion_state {
            CompletionState::Fixed => {
                if verify_status != Some(StageStatus::Passed) || !all_results_passed {
                    bad(violations);
                }
                if receipt.has_stage(RepairStage::Rollback)
                    || receipt.has_stage(RepairStage::Compensate)
                {
                    bad(violations);
                }
            }
            CompletionState::PartiallyRepaired => {
                if apply_status != Some(StageStatus::Partial) || !receipt.partial_success {
                    bad(violations);
                }
            }
            CompletionState::ReducedButNotResolved => {
                if !receipt.partial_success || !has_failed_result {
                    bad(violations);
                }
            }
            CompletionState::VerificationInconclusive => {
                if verify_status != Some(StageStatus::Inconclusive) || !has_inconclusive_result {
                    bad(violations);
                }
            }
            CompletionState::RolledBackExact => {
                if !receipt.has_stage(RepairStage::Rollback) || !receipt.checkpoint.present {
                    bad(violations);
                }
            }
            CompletionState::RolledBackCompensating => {
                if !receipt.has_stage(RepairStage::Compensate) {
                    bad(violations);
                }
            }
        }
    }

    fn validate_guardrails(
        &self,
        receipt: &RepairTransactionReceipt,
        violations: &mut Vec<ProjectDoctorRepairTransactionReceiptsViolation>,
    ) {
        let id = receipt.receipt_id.clone();

        // No hidden reset of durable user state: durable mutation must be backed
        // by a checkpoint or an explicitly guarded irreversible repair that offers
        // support paths.
        if !receipt.durable_state_is_guarded() {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::UnguardedDurableMutation {
                    receipt_id: id.clone(),
                },
            );
        }

        if !receipt.checkpoint.present {
            // No checkpoint: the receipt may not claim clean/snapshot
            // reversibility, must offer support/export paths, and may never claim
            // an exact rollback.
            if receipt.reversal_class.requires_checkpoint() {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::ReversibilityWithoutCheckpoint {
                        receipt_id: id.clone(),
                        reversal_class: receipt.reversal_class.as_str(),
                    },
                );
            }
            if !receipt.offers_support_path() {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::NoCheckpointWithoutSupportPath {
                        receipt_id: id.clone(),
                    },
                );
            }
            if receipt.completion_state == CompletionState::RolledBackExact {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::ExactRollbackWithoutCheckpoint {
                        receipt_id: id.clone(),
                    },
                );
            }
        }
    }

    fn validate_parity_and_safety(
        &self,
        receipt: &RepairTransactionReceipt,
        violations: &mut Vec<ProjectDoctorRepairTransactionReceiptsViolation>,
    ) {
        let id = receipt.receipt_id.clone();

        if receipt.parity_surfaces.is_empty() {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::ParitySurfacesEmpty {
                    receipt_id: id.clone(),
                },
            );
        }
        if !receipt.is_cross_surface_stable() {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::NotCrossSurfaceStable {
                    receipt_id: id.clone(),
                },
            );
        }

        for required in REQUIRED_MACHINE_MEANING_KEYS {
            if !receipt.machine_meaning_keys.iter().any(|k| k == required) {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::MissingMachineMeaningKey {
                        receipt_id: id.clone(),
                        key: required,
                    },
                );
            }
        }

        if receipt.redaction_class != "metadata_safe_default" {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::RedactionNotSafe {
                    receipt_id: id.clone(),
                },
            );
        }
        if !receipt.raw_private_material_excluded {
            violations.push(
                ProjectDoctorRepairTransactionReceiptsViolation::RawMaterialPresent {
                    receipt_id: id.clone(),
                },
            );
        }

        // Verification-result check ids must be present and non-generic.
        for result in &receipt.verification_results {
            let check = result.check_id.trim();
            if check.is_empty()
                || GENERIC_DETAIL_TOKENS.contains(&check.to_ascii_lowercase().as_str())
            {
                violations.push(
                    ProjectDoctorRepairTransactionReceiptsViolation::GenericVerificationCheck {
                        receipt_id: id.clone(),
                    },
                );
            }
        }
    }
}

/// A validation violation for the repair-transaction-receipts packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectDoctorRepairTransactionReceiptsViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Record or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A required list field is empty.
    RequiredListEmpty {
        /// Receipt id.
        receipt_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A receipt id appears more than once.
    DuplicateReceiptId {
        /// Duplicate receipt id.
        receipt_id: String,
    },
    /// A receipt id does not use the receipt prefix.
    ReceiptIdPrefix {
        /// Receipt id.
        receipt_id: String,
    },
    /// A repair id does not use the repair prefix.
    RepairIdPrefix {
        /// Receipt id.
        receipt_id: String,
    },
    /// A receipt names no initiating finding.
    NoInitiatingFinding {
        /// Receipt id.
        receipt_id: String,
    },
    /// An initiating finding code does not use the Doctor finding prefix.
    FindingCodePrefix {
        /// Receipt id.
        receipt_id: String,
        /// Offending finding code.
        finding: String,
    },
    /// A receipt offers no support/export path.
    NoSupportPath {
        /// Receipt id.
        receipt_id: String,
    },
    /// A checkpoint disclosure is internally inconsistent.
    CheckpointInconsistent {
        /// Receipt id.
        receipt_id: String,
    },
    /// The reversal class and checkpoint kind disagree.
    ReversalCheckpointMismatch {
        /// Receipt id.
        receipt_id: String,
        /// Reversal-class token.
        reversal_class: &'static str,
        /// Checkpoint-kind token.
        checkpoint_kind: &'static str,
    },
    /// A required stage was not recorded.
    MissingRequiredStage {
        /// Receipt id.
        receipt_id: String,
        /// Missing stage token.
        stage: &'static str,
    },
    /// A stage appears more than once.
    DuplicateStage {
        /// Receipt id.
        receipt_id: String,
        /// Duplicate stage token.
        stage: &'static str,
    },
    /// Stages are not recorded in canonical order.
    StagesOutOfOrder {
        /// Receipt id.
        receipt_id: String,
        /// Offending stage token.
        stage: &'static str,
    },
    /// Both rollback and compensate stages are present.
    RollbackAndCompensate {
        /// Receipt id.
        receipt_id: String,
    },
    /// The checkpoint stage presence disagrees with the captured checkpoint.
    CheckpointStageMismatch {
        /// Receipt id.
        receipt_id: String,
    },
    /// The completion state disagrees with the stages/results.
    CompletionStateInconsistent {
        /// Receipt id.
        receipt_id: String,
        /// Completion-state token.
        completion_state: &'static str,
    },
    /// A durable-state mutation is neither checkpointed nor guarded.
    UnguardedDurableMutation {
        /// Receipt id.
        receipt_id: String,
    },
    /// A receipt without a checkpoint claims clean/snapshot reversibility.
    ReversibilityWithoutCheckpoint {
        /// Receipt id.
        receipt_id: String,
        /// Reversal-class token.
        reversal_class: &'static str,
    },
    /// A receipt without a checkpoint offers no support/export path.
    NoCheckpointWithoutSupportPath {
        /// Receipt id.
        receipt_id: String,
    },
    /// A receipt claims an exact rollback without a checkpoint.
    ExactRollbackWithoutCheckpoint {
        /// Receipt id.
        receipt_id: String,
    },
    /// A receipt declares no parity surface.
    ParitySurfacesEmpty {
        /// Receipt id.
        receipt_id: String,
    },
    /// A receipt does not render stably across the core surfaces.
    NotCrossSurfaceStable {
        /// Receipt id.
        receipt_id: String,
    },
    /// A receipt omits a required locale-invariant machine-meaning key.
    MissingMachineMeaningKey {
        /// Receipt id.
        receipt_id: String,
        /// Missing key.
        key: &'static str,
    },
    /// A receipt's redaction class is not metadata-safe.
    RedactionNotSafe {
        /// Receipt id.
        receipt_id: String,
    },
    /// A receipt does not exclude raw private material.
    RawMaterialPresent {
        /// Receipt id.
        receipt_id: String,
    },
    /// A verification result hides behind a generic/empty check id.
    GenericVerificationCheck {
        /// Receipt id.
        receipt_id: String,
    },
    /// The summary counts disagree with the receipts.
    SummaryMismatch,
}

impl fmt::Display for ProjectDoctorRepairTransactionReceiptsViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::RequiredListEmpty {
                receipt_id,
                field_name,
            } => {
                write!(
                    f,
                    "receipt {receipt_id} has empty required list {field_name}"
                )
            }
            Self::DuplicateReceiptId { receipt_id } => {
                write!(f, "duplicate receipt id {receipt_id}")
            }
            Self::ReceiptIdPrefix { receipt_id } => {
                write!(
                    f,
                    "receipt {receipt_id} id does not start with {RECEIPT_ID_PREFIX}"
                )
            }
            Self::RepairIdPrefix { receipt_id } => {
                write!(
                    f,
                    "receipt {receipt_id} repair id does not start with {DOCTOR_REPAIR_PREFIX}"
                )
            }
            Self::NoInitiatingFinding { receipt_id } => {
                write!(f, "receipt {receipt_id} names no initiating finding")
            }
            Self::FindingCodePrefix {
                receipt_id,
                finding,
            } => {
                write!(
                    f,
                    "receipt {receipt_id} finding {finding} does not start with {DOCTOR_FINDING_PREFIX}"
                )
            }
            Self::NoSupportPath { receipt_id } => {
                write!(f, "receipt {receipt_id} offers no support/export path")
            }
            Self::CheckpointInconsistent { receipt_id } => {
                write!(
                    f,
                    "receipt {receipt_id} checkpoint disclosure is internally inconsistent"
                )
            }
            Self::ReversalCheckpointMismatch {
                receipt_id,
                reversal_class,
                checkpoint_kind,
            } => {
                write!(
                    f,
                    "receipt {receipt_id} reversal class {reversal_class} does not match checkpoint kind {checkpoint_kind}"
                )
            }
            Self::MissingRequiredStage { receipt_id, stage } => {
                write!(f, "receipt {receipt_id} is missing required stage {stage}")
            }
            Self::DuplicateStage { receipt_id, stage } => {
                write!(
                    f,
                    "receipt {receipt_id} records stage {stage} more than once"
                )
            }
            Self::StagesOutOfOrder { receipt_id, stage } => {
                write!(f, "receipt {receipt_id} records stage {stage} out of order")
            }
            Self::RollbackAndCompensate { receipt_id } => {
                write!(
                    f,
                    "receipt {receipt_id} records both rollback and compensate stages"
                )
            }
            Self::CheckpointStageMismatch { receipt_id } => {
                write!(
                    f,
                    "receipt {receipt_id} checkpoint stage presence disagrees with the captured checkpoint"
                )
            }
            Self::CompletionStateInconsistent {
                receipt_id,
                completion_state,
            } => {
                write!(
                    f,
                    "receipt {receipt_id} completion state {completion_state} disagrees with its stages/results"
                )
            }
            Self::UnguardedDurableMutation { receipt_id } => {
                write!(
                    f,
                    "receipt {receipt_id} mutates durable user state without a checkpoint or guarded reversal"
                )
            }
            Self::ReversibilityWithoutCheckpoint {
                receipt_id,
                reversal_class,
            } => {
                write!(
                    f,
                    "receipt {receipt_id} claims reversal class {reversal_class} but has no checkpoint"
                )
            }
            Self::NoCheckpointWithoutSupportPath { receipt_id } => {
                write!(
                    f,
                    "receipt {receipt_id} has no checkpoint and offers no support/export path"
                )
            }
            Self::ExactRollbackWithoutCheckpoint { receipt_id } => {
                write!(
                    f,
                    "receipt {receipt_id} claims an exact rollback without a checkpoint"
                )
            }
            Self::ParitySurfacesEmpty { receipt_id } => {
                write!(f, "receipt {receipt_id} declares no parity surface")
            }
            Self::NotCrossSurfaceStable { receipt_id } => {
                write!(
                    f,
                    "receipt {receipt_id} is not stable across desktop, CLI, headless, and support"
                )
            }
            Self::MissingMachineMeaningKey { receipt_id, key } => {
                write!(f, "receipt {receipt_id} omits machine-meaning key {key}")
            }
            Self::RedactionNotSafe { receipt_id } => {
                write!(
                    f,
                    "receipt {receipt_id} redaction_class must be metadata_safe_default"
                )
            }
            Self::RawMaterialPresent { receipt_id } => {
                write!(
                    f,
                    "receipt {receipt_id} raw_private_material_excluded must be true"
                )
            }
            Self::GenericVerificationCheck { receipt_id } => {
                write!(
                    f,
                    "receipt {receipt_id} has a verification result with a generic/empty check id"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the receipts")
            }
        }
    }
}

impl Error for ProjectDoctorRepairTransactionReceiptsViolation {}

/// Loads the embedded repair-transaction-receipts packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`ProjectDoctorRepairTransactionReceipts`].
pub fn current_project_doctor_repair_transaction_receipts(
) -> Result<ProjectDoctorRepairTransactionReceipts, serde_json::Error> {
    serde_json::from_str(PROJECT_DOCTOR_REPAIR_TRANSACTION_RECEIPTS_JSON)
}

#[cfg(test)]
mod tests;
