//! Canonical per-entry truth for reused task/problem/output chronology rows: one
//! durable run-lifecycle event — start, progress, retry, cancel, failure, or
//! completion — replayed across the activity center, the history/timeline, issue
//! packets, support bundles, and AI-evidence packets without each surface
//! re-summarising what ran.
//!
//! Where [`crate::m5_execution_evidence_causality_matrix`] froze the *lane* matrix
//! (one row per Problems/output/execution-evidence **surface family**),
//! [`crate::m5_problem_records_source_task_correlation_and_rerun_jump_parity`] froze
//! the **individual Problems row**, and
//! [`crate::m5_execution_evidence_projection_overlays`] froze the **projected
//! overlay**, this module freezes the **individual chronology entry**. A chronology
//! entry is one durable run-lifecycle event written once and *reused* — the same
//! object surfaces in the activity center, in the history/timeline, in an exported
//! issue packet, in a support bundle, and in an AI-evidence packet. Each
//! [`ChronologyEntry`] binds its actor/action/object/outcome grammar to the canonical
//! task/run/channel/problem objects, the provider/adapter and target scope it ran
//! against, its retry lineage, the evidence freshness/stale/superseded state, the
//! confidence tier, and the reopen-to-origin target — so a failure shown in three
//! places points to one canonical run/channel/problem id rather than three
//! rephrasings.
//!
//! The entry speaks the **same** frozen vocabulary as the causality matrix
//! ([`ClaimPosture`], [`OriginClass`], [`ConfidenceTier`], [`FreshnessState`],
//! [`ReopenTarget`], [`ProofCurrency`], [`VerificationFreshness`]) rather than forking
//! a private chronology truth model. Reuse the canonical task-event envelopes,
//! diagnostic ids, run/channel refs, and activity rows already landed earlier; this
//! module binds them onto one inspectable, reopenable chronology row.
//!
//! Re-derivation rules ([`ChronologyEntry::narrow`]):
//!
//! * Every entry keeps its **actor/action/object/outcome grammar, provider/adapter,
//!   target scope, and retry lineage** intact, and its canonical **run/channel/problem
//!   ids reopenable on demand** on every surface it is reused on. A failure in the
//!   activity center, a support bundle, and an AI-evidence packet must resolve to the
//!   same run/channel/problem id, not three surface-local restatements.
//! * Every entry carries an explicit **freshness label**, a **confidence tier**, and a
//!   marked **superseded** state; a stale or superseded entry stays visibly classified
//!   rather than reading as a fresh, fully actionable row.
//! * **Imported/remote/pipeline** origins are reused only as a read-only overlay; they
//!   are attributable and reopenable but never claim live local authority, and a reuse
//!   surface may never render a claim wider than the entry's effective claim.
//! * An **exported** packet (issue / support / AI evidence) stays self-contained:
//!   reviewable and reopenable without the originating UI state.
//! * An entry that flattens its grammar, provider/adapter, target scope, retry
//!   lineage, or canonical ids; lets two surfaces disagree about the canonical id;
//!   hides lineage from a surface; drops a heuristic raw-output backlink; loses its
//!   reopen path; lets a surface overclaim; lets an imported chronology claim live; or
//!   exports a packet that needs the UI to be reviewable floors to
//!   [`ChronologyClaim::Unreconstructable`] and keeps a raw-output / keyboard fallback
//!   rather than rendering a clean-but-false row. Stale/labelled gaps hold a
//!   first-party entry at [`ChronologyClaim::Narrowed`] (still reopenable).
//!   Labs/unadvertised entries make no public claim and are never widened.
//!
//! [`M5ChronologyReuseSetPacket::validate`] confirms the packet is well-formed and
//! honest: header/identity/redaction/freshness are present, every lifecycle phase and
//! every reuse surface is represented, overlay entries name their provider, retry
//! entries carry retry lineage, the recorded action matches the recorded outcome, no
//! reuse surface overclaims its entry, a floored entry keeps a raw fallback, at least
//! one entry demonstrates the auto-narrowing rule, and no raw boundary material crosses
//! the export. Downstream activity-center, history, issue-export, support-export, and
//! AI-evidence surfaces ingest this packet rather than inventing a parallel chronology
//! model.
//!
//! Raw stdout/stderr bytes, command lines, provider log bodies, env bodies, absolute
//! paths, URLs, and secrets never cross this boundary; the packet carries only typed
//! class tokens, opaque ids, booleans, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/tooling/m5-chronology-reuse.schema.json`](../../../../schemas/tooling/m5-chronology-reuse.schema.json).
//! The contract doc is
//! [`docs/tooling/m5-chronology-reuse.md`](../../../../docs/tooling/m5-chronology-reuse.md).
//! The canonical support export is
//! [`artifacts/tooling/m5-chronology-reuse/support_export.json`](../../../../artifacts/tooling/m5-chronology-reuse/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/tooling/m5-chronology-reuse/`](../../../../fixtures/tooling/m5-chronology-reuse/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_execution_evidence_causality_matrix::{
    json_contains_forbidden_boundary_material, label_is_generic, parse_rfc3339_to_epoch_seconds,
    ClaimPosture, ConfidenceTier, FreshnessState, OriginClass, ProofCurrency, ReopenTarget,
    VerificationFreshness,
};

/// Stable record-kind tag carried by [`M5ChronologyReuseSetPacket`].
pub const M5_CHRONOLOGY_REUSE_RECORD_KIND: &str = "m5_chronology_reuse_set_packet";

/// Schema version for the chronology-reuse set.
pub const M5_CHRONOLOGY_REUSE_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_CHRONOLOGY_REUSE_TAXONOMY_VERSION: u32 = 1;

/// Stable id of the canonical chronology-reuse packet.
pub const M5_CHRONOLOGY_REUSE_PACKET_ID: &str = "m5-chronology-reuse:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_CHRONOLOGY_REUSE_SCHEMA_REF: &str = "schemas/tooling/m5-chronology-reuse.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CHRONOLOGY_REUSE_DOC_REF: &str = "docs/tooling/m5-chronology-reuse.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_CHRONOLOGY_REUSE_SUPPORT_EXPORT_REF: &str =
    "artifacts/tooling/m5-chronology-reuse/support_export.json";

/// Repo-relative path of the generated certification report.
pub const M5_CHRONOLOGY_REUSE_REPORT_REF: &str = "artifacts/tooling/m5-chronology-reuse/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_CHRONOLOGY_REUSE_FIXTURE_DIR: &str = "fixtures/tooling/m5-chronology-reuse";

/// Allowed packet redaction-class tokens.
const REDACTION_CLASS_TOKENS: [&str; 4] = [
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
];

/// Deterministic seed timestamp for the canonical packet and report.
const SEED_AS_OF: &str = "2026-06-21T00:00:00Z";

// --------------------------------------------------------------------------- //
// Frozen chronology taxonomies (mirror the boundary schema).
// --------------------------------------------------------------------------- //

/// The lifecycle position a chronology entry records. This is the *action* in the
/// actor/action/object/outcome grammar: a run starts, reports progress, is retried,
/// is cancelled, fails, or completes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyPhase {
    /// A run started.
    RunStarted,
    /// A run reported progress.
    RunProgress,
    /// A run was retried (a new attempt of an earlier run).
    RunRetried,
    /// A run was cancelled.
    RunCancelled,
    /// A run failed.
    RunFailed,
    /// A run completed successfully.
    RunCompleted,
}

impl ChronologyPhase {
    /// Every lifecycle phase, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RunStarted,
        Self::RunProgress,
        Self::RunRetried,
        Self::RunCancelled,
        Self::RunFailed,
        Self::RunCompleted,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunStarted => "run_started",
            Self::RunProgress => "run_progress",
            Self::RunRetried => "run_retried",
            Self::RunCancelled => "run_cancelled",
            Self::RunFailed => "run_failed",
            Self::RunCompleted => "run_completed",
        }
    }

    /// The single outcome this action is consistent with, so a recorded action and a
    /// recorded outcome can never silently disagree.
    pub const fn consistent_outcome(self) -> ChronologyOutcome {
        match self {
            Self::RunStarted | Self::RunProgress => ChronologyOutcome::InProgress,
            Self::RunRetried => ChronologyOutcome::Retried,
            Self::RunCancelled => ChronologyOutcome::Cancelled,
            Self::RunFailed => ChronologyOutcome::Failed,
            Self::RunCompleted => ChronologyOutcome::Succeeded,
        }
    }

    /// Whether this phase records a fresh attempt of an earlier run, so retry lineage
    /// (attempt index and a prior-run ref) must be present.
    pub const fn is_retry(self) -> bool {
        matches!(self, Self::RunRetried)
    }
}

/// Who or what performed the action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyActorKind {
    /// A human user.
    User,
    /// An AI agent.
    AiAgent,
    /// Headless automation / a scheduled job.
    Automation,
    /// An extension.
    Extension,
    /// A remote provider / pipeline adapter.
    ProviderAdapter,
    /// The system / runtime itself.
    System,
}

impl ChronologyActorKind {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => "user",
            Self::AiAgent => "ai_agent",
            Self::Automation => "automation",
            Self::Extension => "extension",
            Self::ProviderAdapter => "provider_adapter",
            Self::System => "system",
        }
    }
}

/// The object the action was performed on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyObjectKind {
    /// A task run.
    TaskRun,
    /// A test run.
    TestRun,
    /// A debug session.
    DebugSession,
    /// A notebook run.
    NotebookRun,
    /// A pipeline run.
    PipelineRun,
    /// A build task.
    BuildTask,
    /// An output channel.
    OutputChannel,
}

impl ChronologyObjectKind {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskRun => "task_run",
            Self::TestRun => "test_run",
            Self::DebugSession => "debug_session",
            Self::NotebookRun => "notebook_run",
            Self::PipelineRun => "pipeline_run",
            Self::BuildTask => "build_task",
            Self::OutputChannel => "output_channel",
        }
    }
}

/// The outcome an action resolved to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyOutcome {
    /// Still running.
    InProgress,
    /// Completed successfully.
    Succeeded,
    /// Failed.
    Failed,
    /// Cancelled.
    Cancelled,
    /// Retried (a new attempt was minted).
    Retried,
    /// Superseded by a newer run.
    Superseded,
}

impl ChronologyOutcome {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InProgress => "in_progress",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::Retried => "retried",
            Self::Superseded => "superseded",
        }
    }
}

/// A surface a chronology entry is reused on, away from where it was first written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologySurface {
    /// The activity center.
    ActivityCenter,
    /// The history / timeline.
    HistoryTimeline,
    /// An exported issue packet.
    IssuePacket,
    /// An exported support bundle.
    SupportBundle,
    /// An exported AI-evidence packet.
    AiEvidencePacket,
}

impl ChronologySurface {
    /// Every reuse surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ActivityCenter,
        Self::HistoryTimeline,
        Self::IssuePacket,
        Self::SupportBundle,
        Self::AiEvidencePacket,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActivityCenter => "activity_center",
            Self::HistoryTimeline => "history_timeline",
            Self::IssuePacket => "issue_packet",
            Self::SupportBundle => "support_bundle",
            Self::AiEvidencePacket => "ai_evidence_packet",
        }
    }

    /// Whether this surface is an exported packet, which must stay self-contained:
    /// reviewable and reopenable without the originating live UI state.
    pub const fn is_export(self) -> bool {
        matches!(
            self,
            Self::IssuePacket | Self::SupportBundle | Self::AiEvidencePacket
        )
    }
}

// --------------------------------------------------------------------------- //
// Derived chronology-claim ladder and narrowing reasons.
// --------------------------------------------------------------------------- //

/// The effective claim a chronology entry renders when reused. A higher rank asserts
/// more authority, so a narrowed or floored entry must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyClaim {
    /// Grammar/lineage/reopen broken or canonical ids disagree; the entry surfaces a
    /// raw-output backlink or keyboard fallback instead of a clean-but-false row.
    #[serde(rename = "chronology_unreconstructable")]
    Unreconstructable,
    /// Remote/pipeline/imported chronology; attributable and reopenable but never
    /// claims live local authority.
    #[serde(rename = "chronology_read_only_overlay")]
    ReadOnlyOverlay,
    /// A first-party entry held below reused by a stale/labelled gap, but lineage stays
    /// reopenable.
    #[serde(rename = "chronology_narrowed")]
    Narrowed,
    /// Full first-party chronology preserved, fresh, grammar/ids intact, reopenable —
    /// reused faithfully across every surface.
    #[serde(rename = "chronology_reused")]
    Reused,
    /// Labs/unadvertised; makes no public claim and is never widened.
    #[serde(rename = "chronology_labs_not_claimed")]
    LabsNotClaimed,
}

impl ChronologyClaim {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unreconstructable => "chronology_unreconstructable",
            Self::ReadOnlyOverlay => "chronology_read_only_overlay",
            Self::Narrowed => "chronology_narrowed",
            Self::Reused => "chronology_reused",
            Self::LabsNotClaimed => "chronology_labs_not_claimed",
        }
    }

    /// Monotonic rank, or `None` for the non-claiming Labs token.
    pub const fn rank(self) -> Option<u8> {
        match self {
            Self::Unreconstructable => Some(0),
            Self::ReadOnlyOverlay => Some(1),
            Self::Narrowed => Some(2),
            Self::Reused => Some(3),
            Self::LabsNotClaimed => None,
        }
    }

    /// Whether rendering `rendered` would overclaim relative to this effective claim. A
    /// reuse surface must never render wider than the entry's effective claim; the Labs
    /// token may only render as itself.
    pub fn overclaims_as(self, rendered: ChronologyClaim) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            _ => self != rendered,
        }
    }
}

/// A reason a chronology entry fails to hold its headline claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronologyNarrowingReason {
    /// Actor/action/object/outcome grammar flattened away from the entry.
    #[serde(rename = "grammar_flattened")]
    GrammarFlattened,
    /// Provider/adapter identity flattened away from the entry.
    #[serde(rename = "provider_adapter_flattened")]
    ProviderAdapterFlattened,
    /// Target scope (target / build-toolchain / host-target) flattened.
    #[serde(rename = "target_scope_flattened")]
    TargetScopeFlattened,
    /// Retry lineage flattened.
    #[serde(rename = "retry_lineage_flattened")]
    RetryLineageFlattened,
    /// Canonical run/channel/problem ids flattened away from the entry.
    #[serde(rename = "canonical_id_flattened")]
    CanonicalIdFlattened,
    /// Two reuse surfaces disagree about the canonical run/channel/problem id.
    #[serde(rename = "canonical_id_divergence")]
    CanonicalIdDivergence,
    /// Lineage cannot be revealed on demand on some reuse surface.
    #[serde(rename = "lineage_not_visible")]
    LineageNotVisible,
    /// Heuristic entry without a raw-output backlink.
    #[serde(rename = "raw_output_backlink_missing")]
    RawBacklinkMissing,
    /// Reopen-to-origin lost; only a keyboard fallback remains.
    #[serde(rename = "reopen_target_lost")]
    ReopenTargetLost,
    /// An exported packet cannot be reviewed without the originating UI state.
    #[serde(rename = "export_not_self_contained")]
    ExportNotSelfContained,
    /// A reuse surface renders a claim wider than the effective claim.
    #[serde(rename = "surface_overclaims")]
    SurfaceOverclaims,
    /// Imported/remote/pipeline chronology claims live local authority.
    #[serde(rename = "imported_chronology_claims_live")]
    ImportedChronologyClaimsLive,
    /// Evidence missing.
    #[serde(rename = "evidence_missing")]
    EvidenceMissing,
    /// Evidence freshness state not surfaced.
    #[serde(rename = "freshness_unlabeled")]
    FreshnessUnlabeled,
    /// Confidence tier not surfaced.
    #[serde(rename = "confidence_unlabeled")]
    ConfidenceUnlabeled,
    /// Superseded-by-newer-run state not marked.
    #[serde(rename = "superseded_state_not_marked")]
    SupersededNotMarked,
    /// First-party chronology entry stale.
    #[serde(rename = "evidence_stale")]
    StaleEvidence,
    /// Verification proof stale or window elapsed.
    #[serde(rename = "verification_proof_stale")]
    StaleProof,
    /// Verification proof missing.
    #[serde(rename = "verification_proof_missing")]
    MissingProof,
}

impl ChronologyNarrowingReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GrammarFlattened => "grammar_flattened",
            Self::ProviderAdapterFlattened => "provider_adapter_flattened",
            Self::TargetScopeFlattened => "target_scope_flattened",
            Self::RetryLineageFlattened => "retry_lineage_flattened",
            Self::CanonicalIdFlattened => "canonical_id_flattened",
            Self::CanonicalIdDivergence => "canonical_id_divergence",
            Self::LineageNotVisible => "lineage_not_visible",
            Self::RawBacklinkMissing => "raw_output_backlink_missing",
            Self::ReopenTargetLost => "reopen_target_lost",
            Self::ExportNotSelfContained => "export_not_self_contained",
            Self::SurfaceOverclaims => "surface_overclaims",
            Self::ImportedChronologyClaimsLive => "imported_chronology_claims_live",
            Self::EvidenceMissing => "evidence_missing",
            Self::FreshnessUnlabeled => "freshness_unlabeled",
            Self::ConfidenceUnlabeled => "confidence_unlabeled",
            Self::SupersededNotMarked => "superseded_state_not_marked",
            Self::StaleEvidence => "evidence_stale",
            Self::StaleProof => "verification_proof_stale",
            Self::MissingProof => "verification_proof_missing",
        }
    }

    /// Whether this reason floors an entry to [`ChronologyClaim::Unreconstructable`].
    /// Each floor reason breaks the "stay reopenable / never flatten grammar or
    /// lineage / never masquerade as live / stay self-contained" contract outright
    /// rather than merely aging out.
    pub const fn is_floor(self) -> bool {
        matches!(
            self,
            Self::GrammarFlattened
                | Self::ProviderAdapterFlattened
                | Self::TargetScopeFlattened
                | Self::RetryLineageFlattened
                | Self::CanonicalIdFlattened
                | Self::CanonicalIdDivergence
                | Self::LineageNotVisible
                | Self::RawBacklinkMissing
                | Self::ReopenTargetLost
                | Self::ExportNotSelfContained
                | Self::SurfaceOverclaims
                | Self::ImportedChronologyClaimsLive
                | Self::EvidenceMissing
        )
    }

    /// Deterministic ordering index so recorded reason lists are stable across runs.
    /// Floor reasons sort first so the headline trigger is the most severe.
    const fn order_index(self) -> u8 {
        match self {
            Self::GrammarFlattened => 0,
            Self::ProviderAdapterFlattened => 1,
            Self::TargetScopeFlattened => 2,
            Self::RetryLineageFlattened => 3,
            Self::CanonicalIdFlattened => 4,
            Self::CanonicalIdDivergence => 5,
            Self::LineageNotVisible => 6,
            Self::ReopenTargetLost => 7,
            Self::RawBacklinkMissing => 8,
            Self::ExportNotSelfContained => 9,
            Self::SurfaceOverclaims => 10,
            Self::ImportedChronologyClaimsLive => 11,
            Self::EvidenceMissing => 12,
            Self::FreshnessUnlabeled => 13,
            Self::ConfidenceUnlabeled => 14,
            Self::SupersededNotMarked => 15,
            Self::StaleEvidence => 16,
            Self::StaleProof => 17,
            Self::MissingProof => 18,
        }
    }
}

/// Sort reasons by their canonical order and drop duplicates.
fn order_reasons(mut reasons: Vec<ChronologyNarrowingReason>) -> Vec<ChronologyNarrowingReason> {
    reasons.sort_by_key(|reason| reason.order_index());
    reasons.dedup();
    reasons
}

// --------------------------------------------------------------------------- //
// Chronology sub-objects.
// --------------------------------------------------------------------------- //

/// The actor/action/object/outcome grammar a chronology entry records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyActorAction {
    /// Who or what performed the action.
    pub actor_kind: ChronologyActorKind,
    /// Stable actor ref, or `null` when the actor is anonymous/system.
    pub actor_ref: Option<String>,
    /// The lifecycle action (start/progress/retry/cancel/fail/complete).
    pub action: ChronologyPhase,
    /// The object the action was performed on.
    pub object_kind: ChronologyObjectKind,
    /// The outcome the action resolved to.
    pub outcome: ChronologyOutcome,
}

/// Stable identifiers binding a chronology entry to its canonical objects. Reuse is
/// reconstructed from these refs, never inferred from freeform display text. Absent
/// refs serialize as `null` so the schema's required keys stay present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyLinks {
    /// Execution-context ref (required).
    pub execution_context_ref: String,
    /// Canonical run ref.
    pub run_ref: Option<String>,
    /// Canonical step ref.
    pub step_ref: Option<String>,
    /// Source task ref.
    pub task_ref: Option<String>,
    /// Owning output-channel ref.
    pub channel_ref: Option<String>,
    /// Correlated Problems-record ref.
    pub problem_ref: Option<String>,
    /// Generated-artifact ref.
    pub artifact_ref: Option<String>,
    /// Provider ref (required for remote/pipeline/imported entries).
    pub provider_ref: Option<String>,
    /// Adapter ref.
    pub adapter_ref: Option<String>,
    /// Raw-output backlink ref.
    pub raw_output_backlink_ref: Option<String>,
}

/// The target scope a run executed against.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyScope {
    /// Target-scope ref (workspace / workset / target).
    pub target_scope_ref: Option<String>,
    /// Build/toolchain ref.
    pub build_toolchain_ref: Option<String>,
    /// Host/target ref.
    pub host_target_ref: Option<String>,
}

/// The retry lineage of an attempt: which attempt this is and the run it retries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetryLineage {
    /// 1-based attempt index; a fresh run is attempt 1.
    pub attempt_index: u32,
    /// The run ref this attempt retries, or `null` for a first attempt.
    pub retry_of_run_ref: Option<String>,
    /// The immediately preceding attempt's entry ref, or `null` for a first attempt.
    pub previous_attempt_ref: Option<String>,
}

/// The chronology-integrity invariants every entry re-derives rather than trusting a
/// grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyIntegrity {
    /// Actor/action/object/outcome grammar survives into the reused entry.
    pub preserves_actor_action_object_outcome: bool,
    /// Provider/adapter identity survives into the reused entry.
    pub preserves_provider_adapter: bool,
    /// Target scope survives into the reused entry.
    pub preserves_target_scope: bool,
    /// Retry lineage survives into the reused entry.
    pub preserves_retry_lineage: bool,
    /// Canonical run/channel/problem ids survive into the reused entry.
    pub preserves_canonical_ids: bool,
    /// Lineage can be revealed on demand on every reuse surface.
    pub lineage_visible_on_demand: bool,
    /// The freshness state is surfaced rather than hidden.
    pub freshness_state_labeled: bool,
    /// The confidence tier is surfaced rather than hidden.
    pub confidence_label_visible: bool,
    /// Superseded state stays marked.
    pub superseded_state_marked: bool,
    /// Imported chronology stays read-only.
    pub imported_chronology_read_only: bool,
    /// A heuristic entry keeps a raw-output backlink.
    pub raw_output_backlink_present: bool,
    /// An exported packet stays reviewable/reopenable without the originating UI.
    pub export_self_contained: bool,
}

/// Certification-proof currency for an entry (distinct from the evidence's own
/// freshness state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyVerification {
    /// Currency of the certification proof.
    pub proof_currency: ProofCurrency,
    /// Proof ref, or `null` when no proof anchors the entry.
    pub proof_ref: Option<String>,
}

/// One surface that reuses a chronology entry, with the claim it shows, whether it can
/// reveal the origin lineage on demand, and the canonical ids it points at.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologySurfaceBinding {
    /// The reuse surface.
    pub surface: ChronologySurface,
    /// The claim this surface renders.
    pub rendered_claim: ChronologyClaim,
    /// Whether the origin grammar/lineage is revealable here.
    pub lineage_visible: bool,
    /// Whether this reuse is read-only.
    pub read_only: bool,
    /// Backlink to the canonical entry this surface reuses.
    pub source_entry_ref: String,
    /// Canonical run ref this surface points at, or `null` when not shown.
    pub bound_run_ref: Option<String>,
    /// Canonical channel ref this surface points at, or `null` when not shown.
    pub bound_channel_ref: Option<String>,
    /// Canonical problem ref this surface points at, or `null` when not shown.
    pub bound_problem_ref: Option<String>,
}

// --------------------------------------------------------------------------- //
// Chronology entry + derivation.
// --------------------------------------------------------------------------- //

/// One claimed (or Labs) chronology entry, reused across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Whether the entry is publicly claimed.
    pub claim_posture: ClaimPosture,
    /// How the run/evidence originated.
    pub origin_class: OriginClass,
    /// Declared confidence tier.
    pub declared_confidence_tier: ConfidenceTier,
    /// Declared freshness state.
    pub declared_freshness_state: FreshnessState,
    /// Declared reopen target.
    pub declared_reopen_target: ReopenTarget,
    /// Actor/action/object/outcome grammar.
    pub grammar: ChronologyActorAction,
    /// Canonical-object link block.
    pub links: ChronologyLinks,
    /// Target-scope block.
    pub scope: ChronologyScope,
    /// Retry-lineage block.
    pub retry_lineage: RetryLineage,
    /// Chronology-integrity invariant block.
    pub integrity: ChronologyIntegrity,
    /// Certification-proof block.
    pub verification: ChronologyVerification,
    /// Surfaces that reuse this entry.
    pub bindings: Vec<ChronologySurfaceBinding>,
}

/// The re-derived chronology decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyDecision {
    /// The headline claim the entry is eligible to make.
    pub claimed_chronology_claim: ChronologyClaim,
    /// The effective claim after re-derivation; never wider than the evidence.
    pub effective_chronology_claim: ChronologyClaim,
    /// Ordered, de-duplicated reasons the entry fails to hold its headline.
    pub active_narrowing_reasons: Vec<ChronologyNarrowingReason>,
    /// Whether the effective claim ranks below the claimed claim.
    pub narrowed: bool,
}

impl ChronologyDecision {
    /// The headline downgrade trigger, when narrowed: the most severe reason.
    pub fn downgrade_trigger(&self) -> Option<ChronologyNarrowingReason> {
        if self.narrowed {
            self.active_narrowing_reasons.first().copied()
        } else {
            None
        }
    }

    /// Whether a surface rendering `rendered` for this entry would overclaim.
    pub fn surface_overclaims(&self, rendered: ChronologyClaim) -> bool {
        self.effective_chronology_claim.overclaims_as(rendered)
    }
}

/// Map (claimed, reasons) onto an effective claim.
fn derive_effective(
    claimed: ChronologyClaim,
    reasons: &[ChronologyNarrowingReason],
) -> ChronologyClaim {
    if reasons.iter().any(|reason| reason.is_floor()) {
        ChronologyClaim::Unreconstructable
    } else if reasons.is_empty() {
        claimed
    } else if matches!(claimed, ChronologyClaim::ReadOnlyOverlay) {
        // An overlay is already the minimal honest claim: any other gap means we can no
        // longer certify even the read-only overlay, so it floors.
        ChronologyClaim::Unreconstructable
    } else {
        ChronologyClaim::Narrowed
    }
}

/// Whether two optional refs that are both present disagree.
fn refs_diverge(a: &Option<String>, b: &Option<String>) -> bool {
    match (opt_str(a), opt_str(b)) {
        (Some(left), Some(right)) => left != right,
        _ => false,
    }
}

impl ChronologyEntry {
    /// The recorded lifecycle phase (the grammar's action).
    pub fn phase(&self) -> ChronologyPhase {
        self.grammar.action
    }

    /// Whether this entry is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, ClaimPosture::LabsUnadvertised)
    }

    /// Whether this entry is an inherently read-only overlay origin.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin_class.is_overlay()
    }

    /// The headline chronology claim this entry is eligible to make.
    pub fn claimed_claim(&self) -> ChronologyClaim {
        if self.is_labs() {
            ChronologyClaim::LabsNotClaimed
        } else if self.is_overlay_origin() {
            ChronologyClaim::ReadOnlyOverlay
        } else {
            ChronologyClaim::Reused
        }
    }

    /// Whether this entry's confidence tier is one of the explicit heuristic tiers,
    /// which must keep a raw-output backlink.
    fn is_heuristic(&self) -> bool {
        self.declared_confidence_tier.is_heuristic_tier()
    }

    /// Whether any reuse surface points at a canonical id that disagrees with the
    /// entry's own canonical run/channel/problem id. This is the cross-surface
    /// consistency check: a failure shown in three places must resolve to one
    /// canonical run/channel/problem id, not three restatements.
    fn has_canonical_id_divergence(&self) -> bool {
        self.bindings.iter().any(|b| {
            refs_diverge(&b.bound_run_ref, &self.links.run_ref)
                || refs_diverge(&b.bound_channel_ref, &self.links.channel_ref)
                || refs_diverge(&b.bound_problem_ref, &self.links.problem_ref)
        })
    }

    /// Reasons that hold independently of how the reuse surfaces render — the intrinsic
    /// grammar/lineage/freshness gaps.
    fn intrinsic_reasons(&self, stale_window: bool) -> Vec<ChronologyNarrowingReason> {
        use ChronologyNarrowingReason as R;
        let integ = &self.integrity;
        let overlay = self.is_overlay_origin();
        let mut reasons: Vec<R> = Vec::new();

        // Grammar / provider-adapter / target-scope / retry-lineage / canonical ids.
        if !integ.preserves_actor_action_object_outcome {
            reasons.push(R::GrammarFlattened);
        }
        if !integ.preserves_provider_adapter {
            reasons.push(R::ProviderAdapterFlattened);
        }
        if !integ.preserves_target_scope {
            reasons.push(R::TargetScopeFlattened);
        }
        if !integ.preserves_retry_lineage {
            reasons.push(R::RetryLineageFlattened);
        }
        if !integ.preserves_canonical_ids {
            reasons.push(R::CanonicalIdFlattened);
        }
        if self.has_canonical_id_divergence() {
            reasons.push(R::CanonicalIdDivergence);
        }

        // Lineage on-demand visibility across every reuse surface.
        if !integ.lineage_visible_on_demand || self.bindings.iter().any(|b| !b.lineage_visible) {
            reasons.push(R::LineageNotVisible);
        }

        // A heuristic entry must keep a raw-output backlink and a tier label.
        if self.is_heuristic() && !integ.raw_output_backlink_present {
            reasons.push(R::RawBacklinkMissing);
        }
        if !integ.confidence_label_visible {
            reasons.push(R::ConfidenceUnlabeled);
        }

        // Freshness must be labelled.
        if !integ.freshness_state_labeled {
            reasons.push(R::FreshnessUnlabeled);
        }

        // Reopen-to-origin must survive.
        if matches!(
            self.declared_reopen_target,
            ReopenTarget::NoneKeyboardFallback
        ) {
            reasons.push(R::ReopenTargetLost);
        }

        // Exported packets must stay self-contained.
        if !integ.export_self_contained && self.bindings.iter().any(|b| b.surface.is_export()) {
            reasons.push(R::ExportNotSelfContained);
        }

        // Evidence freshness / superseded / missing.
        match self.declared_freshness_state {
            FreshnessState::Missing => reasons.push(R::EvidenceMissing),
            FreshnessState::SupersededByNewerRun if !integ.superseded_state_marked => {
                reasons.push(R::SupersededNotMarked);
            }
            // An overlay snapshot is expected to be cached/stale; a first-party live
            // surface showing a stale entry has aged out of currency.
            FreshnessState::StaleExpired if !overlay => {
                reasons.push(R::StaleEvidence);
            }
            _ => {}
        }

        // Certification-proof currency (distinct from the evidence's own freshness).
        match self.verification.proof_currency {
            ProofCurrency::MissingProof => reasons.push(R::MissingProof),
            ProofCurrency::StaleExpired | ProofCurrency::RequiresReview => {
                reasons.push(R::StaleProof);
            }
            ProofCurrency::VerifiedCurrent | ProofCurrency::CachedWithinWindow if stale_window => {
                reasons.push(R::StaleProof);
            }
            _ => {}
        }

        // Imported/remote/pipeline chronology must stay read-only.
        if overlay && !integ.imported_chronology_read_only {
            reasons.push(R::ImportedChronologyClaimsLive);
        }

        reasons
    }

    /// Every reason this entry fails to hold its headline claim, including a reuse
    /// surface that overclaims relative to the intrinsic effective claim.
    pub fn entry_reasons(&self, stale_window: bool) -> Vec<ChronologyNarrowingReason> {
        let claimed = self.claimed_claim();
        let mut reasons = self.intrinsic_reasons(stale_window);
        let intrinsic = derive_effective(claimed, &reasons);
        if self
            .bindings
            .iter()
            .any(|b| intrinsic.overclaims_as(b.rendered_claim))
        {
            reasons.push(ChronologyNarrowingReason::SurfaceOverclaims);
        }
        order_reasons(reasons)
    }

    /// Re-derive the effective chronology claim, reasons, and narrowed flag.
    pub fn narrow(&self, stale_window: bool) -> ChronologyDecision {
        let claimed = self.claimed_claim();

        // Labs/unadvertised entries make no public claim, so they never accrue
        // governance narrowing; they hold their non-claiming token.
        if matches!(claimed, ChronologyClaim::LabsNotClaimed) {
            return ChronologyDecision {
                claimed_chronology_claim: ChronologyClaim::LabsNotClaimed,
                effective_chronology_claim: ChronologyClaim::LabsNotClaimed,
                active_narrowing_reasons: Vec::new(),
                narrowed: false,
            };
        }

        let reasons = self.entry_reasons(stale_window);
        let effective = derive_effective(claimed, &reasons);
        let narrowed = matches!(
            (effective.rank(), claimed.rank()),
            (Some(eff), Some(claim)) if eff < claim
        );

        ChronologyDecision {
            claimed_chronology_claim: claimed,
            effective_chronology_claim: effective,
            active_narrowing_reasons: reasons,
            narrowed,
        }
    }

    /// The effective confidence tier: a floored entry cannot assert a tier beyond
    /// unmapped/needs-review.
    pub fn effective_confidence(&self, effective: ChronologyClaim) -> ConfidenceTier {
        if matches!(effective, ChronologyClaim::Unreconstructable) {
            ConfidenceTier::UnmappedRequiresReview
        } else {
            self.declared_confidence_tier
        }
    }

    /// A precise, non-generic reviewer label for a narrowed/floored entry.
    pub fn narrowed_label(&self, decision: &ChronologyDecision) -> Option<String> {
        if !decision.narrowed {
            return None;
        }
        let trigger = decision
            .downgrade_trigger()
            .map_or("narrowed", ChronologyNarrowingReason::as_str)
            .replace('_', " ");
        let reopen = self.declared_reopen_target.as_str().replace('_', " ");
        let claimed = decision.claimed_chronology_claim.as_str();
        let effective = decision.effective_chronology_claim;
        let label = if matches!(effective, ChronologyClaim::Unreconstructable) {
            format!(
                "Floored to {} below the {claimed} claim: {trigger}; the {reopen} stays reopenable rather than reusing a clean-but-false row",
                effective.as_str()
            )
        } else {
            format!(
                "Held at {} below the {claimed} claim: {trigger}; lineage stays reopenable via the {reopen} until re-verified",
                effective.as_str()
            )
        };
        Some(label)
    }

    /// Whether a non-labs entry that floors keeps a reopen fallback rather than hiding
    /// lineage behind a clean-but-false claim.
    fn floored_keeps_fallback(&self, effective: ChronologyClaim) -> bool {
        if !matches!(effective, ChronologyClaim::Unreconstructable) {
            return true;
        }
        self.declared_reopen_target.is_raw_fallback()
            || self.integrity.raw_output_backlink_present
            || opt_present(&self.links.raw_output_backlink_ref)
    }

    /// Whether any reuse surface renders wider than the entry's effective claim.
    fn surface_overclaims(&self, effective: ChronologyClaim) -> bool {
        self.bindings
            .iter()
            .any(|b| effective.overclaims_as(b.rendered_claim))
    }

    /// Structural checks that hold independently of the narrowing derivation.
    fn structural_violations(&self, out: &mut Vec<M5ChronologyReuseViolation>) {
        if self.entry_id.trim().is_empty()
            || self.label_summary.trim().is_empty()
            || self.links.execution_context_ref.trim().is_empty()
        {
            out.push(M5ChronologyReuseViolation::EntryMissingIdentity);
        }
        if self.is_overlay_origin() && !opt_present(&self.links.provider_ref) {
            out.push(M5ChronologyReuseViolation::OverlayMissingProviderRef);
        }
        if self.bindings.is_empty() {
            out.push(M5ChronologyReuseViolation::EntryMissingBinding);
        }
        for binding in &self.bindings {
            if binding.source_entry_ref.trim().is_empty() {
                out.push(M5ChronologyReuseViolation::BindingMissingSourceRef);
            }
        }
        // A recorded action and a recorded outcome can never silently disagree.
        if self.grammar.outcome != self.phase().consistent_outcome() {
            out.push(M5ChronologyReuseViolation::PhaseOutcomeMismatch);
        }
        // A retry entry must carry retry lineage: a later attempt index and a prior-run
        // ref, so a rerun never reads as a first attempt.
        if self.phase().is_retry()
            && (self.retry_lineage.attempt_index < 2
                || !opt_present(&self.retry_lineage.retry_of_run_ref))
        {
            out.push(M5ChronologyReuseViolation::RetryEntryMissingLineage);
        }
    }
}

/// Whether an optional ref is present and non-empty.
fn opt_present(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|inner| !inner.trim().is_empty())
}

/// The trimmed string content of an optional ref, when present and non-empty.
fn opt_str(value: &Option<String>) -> Option<&str> {
    value
        .as_ref()
        .map(|inner| inner.trim())
        .filter(|inner| !inner.is_empty())
}

// --------------------------------------------------------------------------- //
// Packet.
// --------------------------------------------------------------------------- //

/// Constructor input for an [`M5ChronologyReuseSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologyReuseSetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-entry rows.
    pub entries: Vec<ChronologyEntry>,
}

/// Export-safe M5 chronology-reuse set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ChronologyReuseSetPacket {
    /// Record kind; must equal [`M5_CHRONOLOGY_REUSE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CHRONOLOGY_REUSE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal [`M5_CHRONOLOGY_REUSE_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-entry rows.
    pub entries: Vec<ChronologyEntry>,
}

/// The distribution of effective chronology claims across a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChronologyClaimDistribution {
    /// Entries effective at [`ChronologyClaim::Reused`].
    pub reused: usize,
    /// Entries effective at [`ChronologyClaim::Narrowed`].
    pub narrowed: usize,
    /// Entries effective at [`ChronologyClaim::ReadOnlyOverlay`].
    pub overlay: usize,
    /// Entries effective at [`ChronologyClaim::Unreconstructable`].
    pub unreconstructable: usize,
    /// Entries effective at [`ChronologyClaim::LabsNotClaimed`].
    pub labs: usize,
}

impl M5ChronologyReuseSetPacket {
    /// Builds a chronology-reuse packet, sealing the record-kind, schema, and taxonomy
    /// version constants.
    pub fn new(input: M5ChronologyReuseSetInput) -> Self {
        Self {
            record_kind: M5_CHRONOLOGY_REUSE_RECORD_KIND.to_owned(),
            schema_version: M5_CHRONOLOGY_REUSE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_CHRONOLOGY_REUSE_TAXONOMY_VERSION,
            redaction_class_token: input.redaction_class_token,
            verification_freshness: input.verification_freshness,
            entries: input.entries,
        }
    }

    /// Whether the verification window has elapsed by `as_of`.
    pub fn freshness_stale_at(&self, as_of: &str) -> bool {
        if !self.verification_freshness.auto_downgrade_on_stale {
            return false;
        }
        let last =
            parse_rfc3339_to_epoch_seconds(&self.verification_freshness.last_verification_refresh);
        let now = parse_rfc3339_to_epoch_seconds(as_of);
        match (last, now) {
            (Some(last), Some(now)) => {
                now - last
                    > i64::from(self.verification_freshness.verification_freshness_slo_hours) * 3600
            }
            _ => false,
        }
    }

    /// Whether the window has elapsed by the packet's own `as_of`.
    pub fn stale_window(&self) -> bool {
        self.freshness_stale_at(&self.as_of)
    }

    /// Re-derive the decision for every entry, paired with its id.
    pub fn decisions(&self) -> Vec<(String, ChronologyDecision)> {
        let stale_window = self.stale_window();
        self.entries
            .iter()
            .map(|e| (e.entry_id.clone(), e.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective chronology claims.
    pub fn claim_distribution(&self) -> ChronologyClaimDistribution {
        let stale_window = self.stale_window();
        let mut dist = ChronologyClaimDistribution {
            reused: 0,
            narrowed: 0,
            overlay: 0,
            unreconstructable: 0,
            labs: 0,
        };
        for e in &self.entries {
            match e.narrow(stale_window).effective_chronology_claim {
                ChronologyClaim::Reused => dist.reused += 1,
                ChronologyClaim::Narrowed => dist.narrowed += 1,
                ChronologyClaim::ReadOnlyOverlay => dist.overlay += 1,
                ChronologyClaim::Unreconstructable => dist.unreconstructable += 1,
                ChronologyClaim::LabsNotClaimed => dist.labs += 1,
            }
        }
        dist
    }

    /// Count of entries whose effective claim ranks below their claimed claim.
    pub fn narrowed_entry_count(&self) -> usize {
        let stale_window = self.stale_window();
        self.entries
            .iter()
            .filter(|e| e.narrow(stale_window).narrowed)
            .count()
    }

    /// Lifecycle phases represented by some entry.
    pub fn represented_phases(&self) -> BTreeSet<ChronologyPhase> {
        self.entries.iter().map(|e| e.phase()).collect()
    }

    /// Reuse surfaces represented by some entry.
    pub fn represented_surfaces(&self) -> BTreeSet<ChronologySurface> {
        self.entries
            .iter()
            .flat_map(|e| e.bindings.iter().map(|b| b.surface))
            .collect()
    }

    /// Validate the chronology-reuse invariants.
    pub fn validate(&self) -> Vec<M5ChronologyReuseViolation> {
        use M5ChronologyReuseViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_CHRONOLOGY_REUSE_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_CHRONOLOGY_REUSE_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_CHRONOLOGY_REUSE_TAXONOMY_VERSION {
            violations.push(V::WrongTaxonomyVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            violations.push(V::MissingIdentity);
        }
        if !REDACTION_CLASS_TOKENS.contains(&self.redaction_class_token.as_str()) {
            violations.push(V::InvalidRedactionClass);
        }
        if self.verification_freshness.verification_freshness_slo_hours == 0
            || self
                .verification_freshness
                .last_verification_refresh
                .trim()
                .is_empty()
        {
            violations.push(V::EvidenceFreshnessIncomplete);
        }
        if self.entries.is_empty() {
            violations.push(V::EmptyEntries);
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for e in &self.entries {
            if !seen.insert(e.entry_id.as_str()) {
                violations.push(V::DuplicateEntryId);
            }
        }

        let phases = self.represented_phases();
        if ChronologyPhase::ALL.iter().any(|p| !phases.contains(p)) {
            violations.push(V::ChronologyPhaseMissing);
        }
        let surfaces = self.represented_surfaces();
        if ChronologySurface::ALL.iter().any(|s| !surfaces.contains(s)) {
            violations.push(V::ChronologySurfaceMissing);
        }

        let stale_window = self.stale_window();
        let mut demonstrates_narrowing = false;
        for e in &self.entries {
            e.structural_violations(&mut violations);
            let decision = e.narrow(stale_window);
            if decision.narrowed {
                demonstrates_narrowing = true;
                if decision.downgrade_trigger().is_none()
                    || e.narrowed_label(&decision)
                        .map_or(true, |label| label_is_generic(&label))
                {
                    violations.push(V::NarrowedEntryMissingLabelOrTrigger);
                }
            }
            if !e.floored_keeps_fallback(decision.effective_chronology_claim) {
                violations.push(V::FlooredEntryLosesFallback);
            }
            if e.surface_overclaims(decision.effective_chronology_claim) {
                violations.push(V::BindingSurfaceOverclaims);
            }
        }
        if !demonstrates_narrowing {
            violations.push(V::DowngradedEntryCaseMissing);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("chronology-reuse packet serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("chronology-reuse packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.claim_distribution();
        let mut out = String::new();
        out.push_str("# M5 Task/Problem/Output Chronology Reuse\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Entries: {}\n", self.entries.len()));
        out.push_str(&format!(
            "- Effective: {} reused, {} narrowed, {} read-only overlay, {} unreconstructable, {} labs\n\n",
            dist.reused, dist.narrowed, dist.overlay, dist.unreconstructable, dist.labs
        ));

        out.push_str("| Entry | Phase | Outcome | Origin | Claimed | Effective | Confidence |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for e in &self.entries {
            let decision = e.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} |\n",
                e.entry_id,
                e.phase().as_str(),
                e.grammar.outcome.as_str(),
                e.origin_class.as_str(),
                decision.claimed_chronology_claim.as_str(),
                decision.effective_chronology_claim.as_str(),
                e.effective_confidence(decision.effective_chronology_claim)
                    .as_str(),
            ));
        }

        out.push('\n');
        for e in &self.entries {
            let decision = e.narrow(stale_window);
            if let Some(label) = e.narrowed_label(&decision) {
                out.push_str(&format!("- Narrowed: `{}` — {}\n", e.entry_id, label));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or validate.
#[derive(Debug)]
pub enum M5ChronologyReuseArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5ChronologyReuseViolation>),
}

impl fmt::Display for M5ChronologyReuseArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(f, "chronology-reuse support export parse error: {err}")
            }
            Self::Validation(violations) => write!(
                f,
                "chronology-reuse support export failed validation: {violations:?}"
            ),
        }
    }
}

impl Error for M5ChronologyReuseArtifactError {}

/// Invariant violations reported by [`M5ChronologyReuseSetPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChronologyReuseViolation {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Taxonomy version is wrong.
    WrongTaxonomyVersion,
    /// Packet identity fields are missing.
    MissingIdentity,
    /// Redaction-class token is not one of the allowed values.
    InvalidRedactionClass,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// The packet carries no entries.
    EmptyEntries,
    /// Two entries share an id.
    DuplicateEntryId,
    /// A required lifecycle phase is unrepresented.
    ChronologyPhaseMissing,
    /// A required reuse surface is unrepresented.
    ChronologySurfaceMissing,
    /// An entry is missing its id, label, or execution-context ref.
    EntryMissingIdentity,
    /// An overlay-origin entry does not name its provider.
    OverlayMissingProviderRef,
    /// An entry is reused on no surface.
    EntryMissingBinding,
    /// A binding is missing its source-entry backlink.
    BindingMissingSourceRef,
    /// A recorded action and a recorded outcome disagree.
    PhaseOutcomeMismatch,
    /// A retry entry is missing its retry lineage.
    RetryEntryMissingLineage,
    /// A floored entry lost its raw-output / keyboard reopen fallback.
    FlooredEntryLosesFallback,
    /// A narrowed entry is missing its precise label or trigger.
    NarrowedEntryMissingLabelOrTrigger,
    /// A reuse surface renders wider than the entry's effective claim.
    BindingSurfaceOverclaims,
    /// No entry demonstrates the auto-narrowing rule.
    DowngradedEntryCaseMissing,
    /// Export-safe JSON carried forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ChronologyReuseViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::WrongTaxonomyVersion => "wrong_taxonomy_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidRedactionClass => "invalid_redaction_class",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::EmptyEntries => "empty_entries",
            Self::DuplicateEntryId => "duplicate_entry_id",
            Self::ChronologyPhaseMissing => "chronology_phase_missing",
            Self::ChronologySurfaceMissing => "chronology_surface_missing",
            Self::EntryMissingIdentity => "entry_missing_identity",
            Self::OverlayMissingProviderRef => "overlay_missing_provider_ref",
            Self::EntryMissingBinding => "entry_missing_binding",
            Self::BindingMissingSourceRef => "binding_missing_source_ref",
            Self::PhaseOutcomeMismatch => "phase_outcome_mismatch",
            Self::RetryEntryMissingLineage => "retry_entry_missing_lineage",
            Self::FlooredEntryLosesFallback => "floored_entry_loses_fallback",
            Self::NarrowedEntryMissingLabelOrTrigger => "narrowed_entry_missing_label_or_trigger",
            Self::BindingSurfaceOverclaims => "binding_surface_overclaims",
            Self::DowngradedEntryCaseMissing => "downgraded_entry_case_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Loads and validates the checked-in canonical support export.
///
/// This is the canonical entry point downstream activity-center, history, issue-export,
/// support-export, and AI-evidence surfaces use to ingest the frozen chronology set
/// instead of cloning surface-local run history.
///
/// # Errors
///
/// Returns [`M5ChronologyReuseArtifactError`] when the artifact cannot be parsed or
/// fails validation.
pub fn current_m5_chronology_reuse_set(
) -> Result<M5ChronologyReuseSetPacket, M5ChronologyReuseArtifactError> {
    let packet: M5ChronologyReuseSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/tooling/m5-chronology-reuse/support_export.json"
    )))
    .map_err(M5ChronologyReuseArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ChronologyReuseArtifactError::Validation(violations))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// The canonical seeded chronology-reuse set: the in-crate source of truth the
/// checked-in support export and report are regenerated from.
pub fn seeded_chronology_reuse_set() -> M5ChronologyReuseSetPacket {
    M5ChronologyReuseSetPacket::new(M5ChronologyReuseSetInput {
        packet_id: M5_CHRONOLOGY_REUSE_PACKET_ID.to_owned(),
        label: "M5 task/problem/output chronology reuse — one durable run-lifecycle grammar across activity, history, issue, support, and AI evidence".to_owned(),
        as_of: SEED_AS_OF.to_owned(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        verification_freshness: VerificationFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_AS_OF.to_owned(),
            auto_downgrade_on_stale: true,
        },
        entries: seed_entries(),
    })
}

/// A clean first-party integrity block.
fn clean_integrity() -> ChronologyIntegrity {
    ChronologyIntegrity {
        preserves_actor_action_object_outcome: true,
        preserves_provider_adapter: true,
        preserves_target_scope: true,
        preserves_retry_lineage: true,
        preserves_canonical_ids: true,
        lineage_visible_on_demand: true,
        freshness_state_labeled: true,
        confidence_label_visible: true,
        superseded_state_marked: true,
        imported_chronology_read_only: true,
        raw_output_backlink_present: true,
        export_self_contained: true,
    }
}

/// A verified-current proof block.
fn verified(proof_ref: &str) -> ChronologyVerification {
    ChronologyVerification {
        proof_currency: ProofCurrency::VerifiedCurrent,
        proof_ref: Some(proof_ref.to_owned()),
    }
}

/// A first-attempt retry-lineage block.
fn first_attempt() -> RetryLineage {
    RetryLineage {
        attempt_index: 1,
        retry_of_run_ref: None,
        previous_attempt_ref: None,
    }
}

/// Bindings that reuse `claim` cleanly across the named surfaces, each echoing the
/// entry's canonical run/channel/problem ids.
fn bindings(
    source_ref: &str,
    claim: ChronologyClaim,
    surfaces: &[ChronologySurface],
    read_only: bool,
    run_ref: Option<&str>,
    channel_ref: Option<&str>,
    problem_ref: Option<&str>,
) -> Vec<ChronologySurfaceBinding> {
    surfaces
        .iter()
        .map(|&surface| ChronologySurfaceBinding {
            surface,
            rendered_claim: claim,
            lineage_visible: true,
            read_only,
            source_entry_ref: source_ref.to_owned(),
            bound_run_ref: run_ref.map(str::to_owned),
            bound_channel_ref: channel_ref.map(str::to_owned),
            bound_problem_ref: problem_ref.map(str::to_owned),
        })
        .collect()
}

fn seed_entries() -> Vec<ChronologyEntry> {
    vec![
        // 1. Run started — local task — reused.
        ChronologyEntry {
            entry_id: "chronology:run-started-local-task:0001".to_owned(),
            label_summary:
                "A user started a local task run; the start event is reused in the activity center and the history timeline."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OwningRun,
            grammar: ChronologyActorAction {
                actor_kind: ChronologyActorKind::User,
                actor_ref: Some("actor.user.primary".to_owned()),
                action: ChronologyPhase::RunStarted,
                object_kind: ChronologyObjectKind::TaskRun,
                outcome: ChronologyOutcome::InProgress,
            },
            links: ChronologyLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.task.build.0001".to_owned()),
                step_ref: Some("step.local.task.build.0001".to_owned()),
                task_ref: Some("task.local.build.0001".to_owned()),
                channel_ref: Some("channel.local.task.0001".to_owned()),
                problem_ref: None,
                artifact_ref: None,
                provider_ref: None,
                adapter_ref: Some("adapter.local.task.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.task.build.0001".to_owned()),
            },
            scope: ChronologyScope {
                target_scope_ref: Some("scope.workspace.primary".to_owned()),
                build_toolchain_ref: Some("toolchain.local.cargo.0001".to_owned()),
                host_target_ref: Some("host.local.primary".to_owned()),
            },
            retry_lineage: first_attempt(),
            integrity: clean_integrity(),
            verification: verified("proof.local.task.start.0001"),
            bindings: bindings(
                "chronology:run-started-local-task:0001",
                ChronologyClaim::Reused,
                &[
                    ChronologySurface::ActivityCenter,
                    ChronologySurface::HistoryTimeline,
                ],
                false,
                Some("run.local.task.build.0001"),
                Some("channel.local.task.0001"),
                None,
            ),
        },
        // 2. Run progress — local test — reused.
        ChronologyEntry {
            entry_id: "chronology:run-progress-local-test:0001".to_owned(),
            label_summary:
                "A local test run reported progress; the progress event is reused in the activity center and the history timeline."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTest,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OwningRun,
            grammar: ChronologyActorAction {
                actor_kind: ChronologyActorKind::System,
                actor_ref: None,
                action: ChronologyPhase::RunProgress,
                object_kind: ChronologyObjectKind::TestRun,
                outcome: ChronologyOutcome::InProgress,
            },
            links: ChronologyLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.test.suite.0001".to_owned()),
                step_ref: Some("step.local.test.suite.0001".to_owned()),
                task_ref: Some("task.local.test.0001".to_owned()),
                channel_ref: Some("channel.local.test.0001".to_owned()),
                problem_ref: None,
                artifact_ref: None,
                provider_ref: None,
                adapter_ref: Some("adapter.local.test.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.test.suite.0001".to_owned()),
            },
            scope: ChronologyScope {
                target_scope_ref: Some("scope.workspace.primary".to_owned()),
                build_toolchain_ref: Some("toolchain.local.cargo.0001".to_owned()),
                host_target_ref: Some("host.local.primary".to_owned()),
            },
            retry_lineage: first_attempt(),
            integrity: clean_integrity(),
            verification: verified("proof.local.test.progress.0001"),
            bindings: bindings(
                "chronology:run-progress-local-test:0001",
                ChronologyClaim::Reused,
                &[
                    ChronologySurface::ActivityCenter,
                    ChronologySurface::HistoryTimeline,
                ],
                false,
                Some("run.local.test.suite.0001"),
                Some("channel.local.test.0001"),
                None,
            ),
        },
        // 3. Run retried — local task — reused, with retry lineage.
        ChronologyEntry {
            entry_id: "chronology:run-retried-local-task:0001".to_owned(),
            label_summary:
                "A user retried a failed local task; the retry event carries attempt-2 lineage and is reused in the history timeline and the AI-evidence packet."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OwningRun,
            grammar: ChronologyActorAction {
                actor_kind: ChronologyActorKind::User,
                actor_ref: Some("actor.user.primary".to_owned()),
                action: ChronologyPhase::RunRetried,
                object_kind: ChronologyObjectKind::TaskRun,
                outcome: ChronologyOutcome::Retried,
            },
            links: ChronologyLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.task.build.0002".to_owned()),
                step_ref: Some("step.local.task.build.0002".to_owned()),
                task_ref: Some("task.local.build.0001".to_owned()),
                channel_ref: Some("channel.local.task.0002".to_owned()),
                problem_ref: None,
                artifact_ref: None,
                provider_ref: None,
                adapter_ref: Some("adapter.local.task.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.task.build.0002".to_owned()),
            },
            scope: ChronologyScope {
                target_scope_ref: Some("scope.workspace.primary".to_owned()),
                build_toolchain_ref: Some("toolchain.local.cargo.0001".to_owned()),
                host_target_ref: Some("host.local.primary".to_owned()),
            },
            retry_lineage: RetryLineage {
                attempt_index: 2,
                retry_of_run_ref: Some("run.local.task.build.0001".to_owned()),
                previous_attempt_ref: Some("chronology:run-started-local-task:0001".to_owned()),
            },
            integrity: clean_integrity(),
            verification: verified("proof.local.task.retry.0001"),
            bindings: bindings(
                "chronology:run-retried-local-task:0001",
                ChronologyClaim::Reused,
                &[
                    ChronologySurface::HistoryTimeline,
                    ChronologySurface::AiEvidencePacket,
                ],
                false,
                Some("run.local.task.build.0002"),
                Some("channel.local.task.0002"),
                None,
            ),
        },
        // 4. Run cancelled — local task — reused.
        ChronologyEntry {
            entry_id: "chronology:run-cancelled-local-task:0001".to_owned(),
            label_summary:
                "A user cancelled a local task run; the cancel event is reused in the activity center and an exported support bundle."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OwningRun,
            grammar: ChronologyActorAction {
                actor_kind: ChronologyActorKind::User,
                actor_ref: Some("actor.user.primary".to_owned()),
                action: ChronologyPhase::RunCancelled,
                object_kind: ChronologyObjectKind::TaskRun,
                outcome: ChronologyOutcome::Cancelled,
            },
            links: ChronologyLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.task.watch.0001".to_owned()),
                step_ref: Some("step.local.task.watch.0001".to_owned()),
                task_ref: Some("task.local.watch.0001".to_owned()),
                channel_ref: Some("channel.local.task.0003".to_owned()),
                problem_ref: None,
                artifact_ref: None,
                provider_ref: None,
                adapter_ref: Some("adapter.local.task.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.task.watch.0001".to_owned()),
            },
            scope: ChronologyScope {
                target_scope_ref: Some("scope.workspace.primary".to_owned()),
                build_toolchain_ref: Some("toolchain.local.cargo.0001".to_owned()),
                host_target_ref: Some("host.local.primary".to_owned()),
            },
            retry_lineage: first_attempt(),
            integrity: clean_integrity(),
            verification: verified("proof.local.task.cancel.0001"),
            bindings: bindings(
                "chronology:run-cancelled-local-task:0001",
                ChronologyClaim::Reused,
                &[
                    ChronologySurface::ActivityCenter,
                    ChronologySurface::SupportBundle,
                ],
                false,
                Some("run.local.task.watch.0001"),
                Some("channel.local.task.0003"),
                None,
            ),
        },
        // 5. Run failed — local test — reused across every surface with one canonical
        //    run/channel/problem id (the cross-surface failure).
        ChronologyEntry {
            entry_id: "chronology:run-failed-local-test:0001".to_owned(),
            label_summary:
                "A local test run failed; the same failure is reused in the activity center, the history timeline, an issue packet, a support bundle, and an AI-evidence packet pointing at one canonical run, channel, and problem id."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTest,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OutputChannel,
            grammar: ChronologyActorAction {
                actor_kind: ChronologyActorKind::System,
                actor_ref: None,
                action: ChronologyPhase::RunFailed,
                object_kind: ChronologyObjectKind::TestRun,
                outcome: ChronologyOutcome::Failed,
            },
            links: ChronologyLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.test.failing.0001".to_owned()),
                step_ref: Some("step.local.test.failing.0001".to_owned()),
                task_ref: Some("task.local.test.0001".to_owned()),
                channel_ref: Some("channel.local.test.0002".to_owned()),
                problem_ref: Some("problem.local.test.assert.0001".to_owned()),
                artifact_ref: Some("artifact.local.test.report.0001".to_owned()),
                provider_ref: None,
                adapter_ref: Some("adapter.local.test.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.test.failing.0001".to_owned()),
            },
            scope: ChronologyScope {
                target_scope_ref: Some("scope.workspace.primary".to_owned()),
                build_toolchain_ref: Some("toolchain.local.cargo.0001".to_owned()),
                host_target_ref: Some("host.local.primary".to_owned()),
            },
            retry_lineage: first_attempt(),
            integrity: clean_integrity(),
            verification: verified("proof.local.test.fail.0001"),
            bindings: bindings(
                "chronology:run-failed-local-test:0001",
                ChronologyClaim::Reused,
                &[
                    ChronologySurface::ActivityCenter,
                    ChronologySurface::HistoryTimeline,
                    ChronologySurface::IssuePacket,
                    ChronologySurface::SupportBundle,
                    ChronologySurface::AiEvidencePacket,
                ],
                false,
                Some("run.local.test.failing.0001"),
                Some("channel.local.test.0002"),
                Some("problem.local.test.assert.0001"),
            ),
        },
        // 6. Run completed — notebook — reused.
        ChronologyEntry {
            entry_id: "chronology:run-completed-notebook:0001".to_owned(),
            label_summary:
                "A notebook run completed successfully; the completion event is reused in the history timeline and an exported support bundle."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::NotebookRun,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OwningRun,
            grammar: ChronologyActorAction {
                actor_kind: ChronologyActorKind::User,
                actor_ref: Some("actor.user.primary".to_owned()),
                action: ChronologyPhase::RunCompleted,
                object_kind: ChronologyObjectKind::NotebookRun,
                outcome: ChronologyOutcome::Succeeded,
            },
            links: ChronologyLinks {
                execution_context_ref: "exec-context.local.notebook.primary".to_owned(),
                run_ref: Some("run.local.notebook.cell.0001".to_owned()),
                step_ref: Some("step.local.notebook.cell.0001".to_owned()),
                task_ref: Some("task.local.notebook.0001".to_owned()),
                channel_ref: Some("channel.local.notebook.0001".to_owned()),
                problem_ref: None,
                artifact_ref: Some("artifact.local.notebook.output.0001".to_owned()),
                provider_ref: None,
                adapter_ref: Some("adapter.local.notebook.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.notebook.cell.0001".to_owned()),
            },
            scope: ChronologyScope {
                target_scope_ref: Some("scope.notebook.primary".to_owned()),
                build_toolchain_ref: Some("toolchain.local.python.0001".to_owned()),
                host_target_ref: Some("host.local.primary".to_owned()),
            },
            retry_lineage: first_attempt(),
            integrity: clean_integrity(),
            verification: verified("proof.local.notebook.complete.0001"),
            bindings: bindings(
                "chronology:run-completed-notebook:0001",
                ChronologyClaim::Reused,
                &[
                    ChronologySurface::HistoryTimeline,
                    ChronologySurface::SupportBundle,
                ],
                false,
                Some("run.local.notebook.cell.0001"),
                Some("channel.local.notebook.0001"),
                None,
            ),
        },
        // 7. Run failed — pipeline/provider — read-only overlay.
        ChronologyEntry {
            entry_id: "chronology:run-failed-pipeline-provider:0001".to_owned(),
            label_summary:
                "A pipeline-provider run failed; the failure is reused read-only in an issue packet, a support bundle, and an AI-evidence packet, attributable but never claiming live local authority."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::PipelineProviderRun,
            declared_confidence_tier: ConfidenceTier::ProviderMapped,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::ProviderRunPage,
            grammar: ChronologyActorAction {
                actor_kind: ChronologyActorKind::ProviderAdapter,
                actor_ref: Some("actor.provider.ci.0001".to_owned()),
                action: ChronologyPhase::RunFailed,
                object_kind: ChronologyObjectKind::PipelineRun,
                outcome: ChronologyOutcome::Failed,
            },
            links: ChronologyLinks {
                execution_context_ref: "exec-context.remote.pipeline.primary".to_owned(),
                run_ref: Some("run.pipeline.provider.0001".to_owned()),
                step_ref: Some("step.pipeline.provider.0001".to_owned()),
                task_ref: Some("task.pipeline.provider.0001".to_owned()),
                channel_ref: Some("channel.pipeline.provider.0001".to_owned()),
                problem_ref: Some("problem.pipeline.provider.0001".to_owned()),
                artifact_ref: Some("artifact.pipeline.provider.0001".to_owned()),
                provider_ref: Some("provider.pipeline.ci.0001".to_owned()),
                adapter_ref: Some("adapter.pipeline.ci.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.pipeline.provider.0001".to_owned()),
            },
            scope: ChronologyScope {
                target_scope_ref: Some("scope.pipeline.provider.0001".to_owned()),
                build_toolchain_ref: Some("toolchain.pipeline.provider.0001".to_owned()),
                host_target_ref: Some("host.pipeline.provider.0001".to_owned()),
            },
            retry_lineage: first_attempt(),
            integrity: clean_integrity(),
            verification: ChronologyVerification {
                proof_currency: ProofCurrency::ImportedCurrent,
                proof_ref: Some("proof.pipeline.provider.0001".to_owned()),
            },
            bindings: bindings(
                "chronology:run-failed-pipeline-provider:0001",
                ChronologyClaim::ReadOnlyOverlay,
                &[
                    ChronologySurface::IssuePacket,
                    ChronologySurface::SupportBundle,
                    ChronologySurface::AiEvidencePacket,
                ],
                true,
                Some("run.pipeline.provider.0001"),
                Some("channel.pipeline.provider.0001"),
                Some("problem.pipeline.provider.0001"),
            ),
        },
        // 8. Run completed — local task with a heuristic perf verdict — narrowed by a
        //    stale verification proof.
        ChronologyEntry {
            entry_id: "chronology:run-completed-perf-local:0001".to_owned(),
            label_summary:
                "A local task run completed with a heuristic perf verdict whose verification proof has gone stale, so it is held below reused while staying reopenable in the history timeline and the AI-evidence packet."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            declared_confidence_tier: ConfidenceTier::HeuristicHigh,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::GeneratedArtifact,
            grammar: ChronologyActorAction {
                actor_kind: ChronologyActorKind::Automation,
                actor_ref: Some("actor.automation.bench.0001".to_owned()),
                action: ChronologyPhase::RunCompleted,
                object_kind: ChronologyObjectKind::BuildTask,
                outcome: ChronologyOutcome::Succeeded,
            },
            links: ChronologyLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.bench.perf.0001".to_owned()),
                step_ref: Some("step.local.bench.perf.0001".to_owned()),
                task_ref: Some("task.local.bench.0001".to_owned()),
                channel_ref: Some("channel.local.bench.0001".to_owned()),
                problem_ref: None,
                artifact_ref: Some("artifact.local.perf.trace.0001".to_owned()),
                provider_ref: None,
                adapter_ref: Some("adapter.local.bench.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.bench.perf.0001".to_owned()),
            },
            scope: ChronologyScope {
                target_scope_ref: Some("scope.workspace.primary".to_owned()),
                build_toolchain_ref: Some("toolchain.local.cargo.0001".to_owned()),
                host_target_ref: Some("host.local.primary".to_owned()),
            },
            retry_lineage: first_attempt(),
            integrity: clean_integrity(),
            verification: ChronologyVerification {
                proof_currency: ProofCurrency::StaleExpired,
                proof_ref: Some("proof.local.perf.0001".to_owned()),
            },
            bindings: bindings(
                "chronology:run-completed-perf-local:0001",
                ChronologyClaim::Narrowed,
                &[
                    ChronologySurface::HistoryTimeline,
                    ChronologySurface::AiEvidencePacket,
                ],
                false,
                Some("run.local.bench.perf.0001"),
                Some("channel.local.bench.0001"),
                None,
            ),
        },
        // 9. Run progress — Labs notebook — makes no public claim.
        ChronologyEntry {
            entry_id: "chronology:run-progress-labs:0001".to_owned(),
            label_summary:
                "A Labs notebook run reported progress; the entry is unadvertised, makes no public claim, and is never widened."
                    .to_owned(),
            claim_posture: ClaimPosture::LabsUnadvertised,
            origin_class: OriginClass::NotebookRun,
            declared_confidence_tier: ConfidenceTier::HeuristicMedium,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::RawOutputBacklink,
            grammar: ChronologyActorAction {
                actor_kind: ChronologyActorKind::Extension,
                actor_ref: Some("actor.extension.labs.0001".to_owned()),
                action: ChronologyPhase::RunProgress,
                object_kind: ChronologyObjectKind::NotebookRun,
                outcome: ChronologyOutcome::InProgress,
            },
            links: ChronologyLinks {
                execution_context_ref: "exec-context.local.notebook.labs".to_owned(),
                run_ref: Some("run.local.notebook.labs.0001".to_owned()),
                step_ref: None,
                task_ref: None,
                channel_ref: Some("channel.local.notebook.labs.0001".to_owned()),
                problem_ref: None,
                artifact_ref: None,
                provider_ref: None,
                adapter_ref: Some("adapter.local.notebook.labs.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.notebook.labs.0001".to_owned()),
            },
            scope: ChronologyScope {
                target_scope_ref: Some("scope.notebook.labs".to_owned()),
                build_toolchain_ref: None,
                host_target_ref: Some("host.local.primary".to_owned()),
            },
            retry_lineage: first_attempt(),
            integrity: clean_integrity(),
            verification: ChronologyVerification {
                proof_currency: ProofCurrency::RequiresReview,
                proof_ref: None,
            },
            bindings: bindings(
                "chronology:run-progress-labs:0001",
                ChronologyClaim::LabsNotClaimed,
                &[ChronologySurface::HistoryTimeline],
                false,
                Some("run.local.notebook.labs.0001"),
                Some("channel.local.notebook.labs.0001"),
                None,
            ),
        },
    ]
}
