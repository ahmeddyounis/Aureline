//! Canonical per-record truth for M5 Problems rows: source-task correlation,
//! structured-versus-heuristic confidence labels, raw-output backlinks, and
//! rerun/jump parity.
//!
//! Where [`crate::m5_execution_evidence_causality_matrix`] froze the *lane*
//! matrix — one row per Problems/output/execution-evidence **surface family** —
//! this module freezes the **individual Problems row**. Each [`ProblemRecord`] is
//! a single run-derived finding bound to its source tool/run refs, its file/span
//! anchor, its structured-versus-heuristic parse class, its confidence tier and
//! raw-output backlink, the editor decoration / timeline entry / source task /
//! owning output channel it is correlated with, and the freshness/stale/
//! superseded state of the run it came from. The record re-derives, rather than
//! trusts, an effective [`ProblemRecordStatus`] and a per-action
//! [`ActionAvailability`] for the three canonical actions (jump to source, open
//! owning output, rerun or inspect the originating task/session) so a stale,
//! superseded, downgraded, or lineage-broken finding can never read as a clean,
//! fully actionable row.
//!
//! The record speaks the **same** frozen vocabulary as the causality matrix
//! (`ProblemSourceKind`, `ConfidenceTier`, `FreshnessState`, `OriginClass`,
//! `OutputChannelClass`, `ClaimPosture`, `ProofCurrency`) rather than forking a
//! private bottom-panel truth model. Reuse the canonical task-event envelopes,
//! diagnostic IDs, activity rows, run objects, and output channels already landed
//! earlier; this module binds them to one inspectable, reopenable Problems row.
//!
//! Re-derivation rules ([`ProblemRecord::narrow`]):
//!
//! * A finding from a **native structured diagnostic**, **normalized task event**,
//!   **imported provider annotation**, or **heuristic text parse** keeps its
//!   origin inspectable: structured and heuristic origins stay distinct, and a
//!   heuristic parse keeps an explicit confidence tier plus a raw-output backlink.
//! * Every row can **jump to source**, **open owning output**, and **rerun or
//!   inspect** the originating task/session *when allowed*; remote/imported origins
//!   inspect read-only and never offer a live local rerun, and an authority-gated
//!   rerun is surfaced as gated, not silently dropped.
//! * Findings from **stale runs**, **superseded retries**, or **downgraded
//!   mappings** stay visibly classified ([`ProblemRecordStatus::NarrowedActionable`])
//!   until dismissed or replaced by current evidence — never silently dropped and
//!   never silently upgraded to fresh certainty.
//! * A finding that conflates structured/heuristic origin, loses its source-tool
//!   ref, drops a heuristic raw-output backlink, leaves a superseded retry
//!   unmarked, or lets an imported overlay claim live local authority floors to
//!   [`ProblemRecordStatus::RawEvidenceOnly`] and keeps a raw-output backlink
//!   rather than rendering a clean-but-false actionable row. Labs/unadvertised
//!   rows make no public claim and are never widened.
//!
//! [`M5ProblemRecordSetPacket::validate`] confirms the packet is well-formed and
//! honest: header/identity/redaction/freshness are present, every claimed
//! problem-source kind is represented, overlay rows name their provider, a floored
//! row keeps a raw-output fallback, at least one row demonstrates the
//! auto-narrowing rule, and no raw boundary material crosses the export.
//!
//! Raw stdout/stderr bytes, command lines, provider log bodies, env bodies,
//! absolute paths, URLs, and secrets never cross this boundary; the packet carries
//! only typed class tokens, line/column numbers, booleans, opaque ids, and
//! redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/tooling/m5-problem-records.schema.json`](../../../../schemas/tooling/m5-problem-records.schema.json).
//! The contract doc is
//! [`docs/tooling/m5-problem-records.md`](../../../../docs/tooling/m5-problem-records.md).
//! The canonical support export is
//! [`artifacts/tooling/m5-problem-records/support_export.json`](../../../../artifacts/tooling/m5-problem-records/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/tooling/m5-problem-records/`](../../../../fixtures/tooling/m5-problem-records/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_execution_evidence_causality_matrix::{
    json_contains_forbidden_boundary_material, label_is_generic, parse_rfc3339_to_epoch_seconds,
    ClaimPosture, ConfidenceTier, FreshnessState, LaneVerification, OriginClass,
    OutputChannelClass, ProblemSourceKind, ProofCurrency, VerificationFreshness,
};

/// Stable record-kind tag carried by [`M5ProblemRecordSetPacket`].
pub const M5_PROBLEM_RECORDS_RECORD_KIND: &str = "m5_problem_record_set_packet";

/// Schema version for the problem-record set.
pub const M5_PROBLEM_RECORDS_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_PROBLEM_RECORDS_TAXONOMY_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_PROBLEM_RECORDS_SCHEMA_REF: &str = "schemas/tooling/m5-problem-records.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PROBLEM_RECORDS_DOC_REF: &str = "docs/tooling/m5-problem-records.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_PROBLEM_RECORDS_SUPPORT_EXPORT_REF: &str =
    "artifacts/tooling/m5-problem-records/support_export.json";

/// Repo-relative path of the generated certification report.
pub const M5_PROBLEM_RECORDS_REPORT_REF: &str = "artifacts/tooling/m5-problem-records/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_PROBLEM_RECORDS_FIXTURE_DIR: &str = "fixtures/tooling/m5-problem-records";

/// Allowed packet redaction-class tokens (mirrors the causality matrix).
const REDACTION_CLASS_TOKENS: [&str; 4] = [
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
];

// --------------------------------------------------------------------------- //
// Frozen record taxonomies (mirror the boundary schema).
// --------------------------------------------------------------------------- //

/// Problem severity, independent of source confidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProblemSeverity {
    /// Informational finding.
    Info,
    /// Warning finding.
    Warning,
    /// Error finding.
    Error,
    /// Fatal finding.
    Fatal,
}

impl ProblemSeverity {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
        }
    }
}

/// Authority a Problems row has to rerun or inspect its originating task/session.
///
/// This models the "when allowed" gate in the rerun/inspect action: a local task
/// can be rerun in place, an authority-gated route must be surfaced as gated, and
/// a remote/imported origin can only be inspected read-only — it never offers a
/// live local rerun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunAuthority {
    /// The originating task can be rerun locally without elevation.
    LocalRerunGranted,
    /// Rerun is permitted but gated behind explicit authority/confirmation.
    RequiresElevatedAuthority,
    /// Remote/imported origin: inspect read-only; no live local rerun.
    RemoteInspectReadOnly,
    /// Policy denies rerun and inspection of the originating session.
    DeniedPolicy,
    /// The row has no rerunnable/inspectable originating task.
    NotApplicable,
}

impl RerunAuthority {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalRerunGranted => "local_rerun_granted",
            Self::RequiresElevatedAuthority => "requires_elevated_authority",
            Self::RemoteInspectReadOnly => "remote_inspect_read_only",
            Self::DeniedPolicy => "denied_policy",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// The three canonical actions every Problems row offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProblemAction {
    /// Jump to the file/span the finding anchors to.
    JumpToSource,
    /// Open the output channel that produced the finding.
    OpenOwningOutput,
    /// Rerun or inspect the originating task/session.
    RerunOrInspectOriginator,
}

impl ProblemAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::JumpToSource,
        Self::OpenOwningOutput,
        Self::RerunOrInspectOriginator,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JumpToSource => "jump_to_source",
            Self::OpenOwningOutput => "open_owning_output",
            Self::RerunOrInspectOriginator => "rerun_or_inspect_originator",
        }
    }
}

/// The re-derived availability of a single [`ProblemAction`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionAvailability {
    /// The action is offered and works against current evidence.
    Available,
    /// The action is offered but gated behind explicit authority/confirmation.
    GatedRequiresAuthority,
    /// The action resolves to a read-only inspection of imported/remote evidence.
    ReadOnlyInspectOnly,
    /// The action cannot be offered because its target lineage is missing.
    Unavailable,
    /// The action does not apply to this row.
    NotApplicable,
}

impl ActionAvailability {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::GatedRequiresAuthority => "gated_requires_authority",
            Self::ReadOnlyInspectOnly => "read_only_inspect_only",
            Self::Unavailable => "unavailable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

// --------------------------------------------------------------------------- //
// Derived status ladder and downgrade reasons.
// --------------------------------------------------------------------------- //

/// The effective status a Problems row renders. A higher rank asserts more
/// actionable certainty, so a narrowed or floored row must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProblemRecordStatus {
    /// Source/lineage broken or origin dishonest; the row surfaces a raw-output
    /// backlink instead of a clean-but-false actionable claim.
    #[serde(rename = "raw_evidence_only")]
    RawEvidenceOnly,
    /// Remote/pipeline/imported finding; inspectable and reopenable but never a
    /// live local actionable row.
    #[serde(rename = "read_only_imported")]
    ReadOnlyImported,
    /// A first-party row held below fully actionable by a stale/superseded/
    /// downgraded/uncorrelated gap, but still jumpable and inspectable.
    #[serde(rename = "narrowed_actionable")]
    NarrowedActionable,
    /// A first-party row with an honest origin, current evidence, full
    /// correlations, and every applicable action available.
    #[serde(rename = "actionable")]
    Actionable,
    /// Labs/unadvertised; makes no public actionability claim and is never widened.
    #[serde(rename = "labs_not_claimed")]
    LabsNotClaimed,
}

impl ProblemRecordStatus {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawEvidenceOnly => "raw_evidence_only",
            Self::ReadOnlyImported => "read_only_imported",
            Self::NarrowedActionable => "narrowed_actionable",
            Self::Actionable => "actionable",
            Self::LabsNotClaimed => "labs_not_claimed",
        }
    }

    /// Monotonic rank used to compare statuses, or `None` for the non-claiming Labs
    /// token (which never participates in widening/narrowing comparisons).
    pub const fn rank(self) -> Option<u8> {
        match self {
            Self::RawEvidenceOnly => Some(0),
            Self::ReadOnlyImported => Some(1),
            Self::NarrowedActionable => Some(2),
            Self::Actionable => Some(3),
            Self::LabsNotClaimed => None,
        }
    }

    /// Whether rendering `rendered` on a public surface would overclaim relative to
    /// this effective status. A projection must never render a Problems row wider
    /// than its effective status; the Labs token may only render as itself.
    pub fn overclaims_as(self, rendered: ProblemRecordStatus) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            _ => self != rendered,
        }
    }
}

/// A reason a Problems row fails to hold its headline status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProblemDowngradeReason {
    /// Structured vs heuristic origin not distinct.
    #[serde(rename = "origin_kind_flattened")]
    OriginFlattened,
    /// Heuristic parse without a raw-output backlink.
    #[serde(rename = "raw_output_backlink_missing")]
    RawBacklinkMissing,
    /// The source tool/run ref is missing, so the originator cannot be reopened.
    #[serde(rename = "source_ref_missing")]
    SourceRefMissing,
    /// Underlying run evidence is missing.
    #[serde(rename = "evidence_missing")]
    EvidenceMissing,
    /// A superseded retry is not visibly marked, which would upgrade certainty.
    #[serde(rename = "superseded_state_not_marked")]
    SupersededNotMarked,
    /// An imported/remote overlay claims live local authority.
    #[serde(rename = "imported_overlay_claims_live")]
    ImportedOverlayClaimsLive,
    /// The confidence tier / label is not surfaced.
    #[serde(rename = "confidence_unlabeled")]
    ConfidenceUnlabeled,
    /// The file/span anchor is missing, so jump-to-source is unavailable.
    #[serde(rename = "anchor_missing")]
    AnchorMissing,
    /// The owning output channel ref is missing, so open-owning-output is lost.
    #[serde(rename = "owning_channel_missing")]
    OwningChannelMissing,
    /// The originating task is not correlated, so rerun/inspect is unavailable.
    #[serde(rename = "source_task_uncorrelated")]
    SourceTaskUncorrelated,
    /// The editor decoration for an anchored finding is not correlated.
    #[serde(rename = "editor_decoration_uncorrelated")]
    EditorDecorationUncorrelated,
    /// The timeline entry for the finding is not correlated.
    #[serde(rename = "timeline_uncorrelated")]
    TimelineUncorrelated,
    /// The finding comes from a stale run.
    #[serde(rename = "stale_run")]
    StaleRun,
    /// The finding is superseded by a newer run (kept visibly classified).
    #[serde(rename = "superseded_by_newer_run")]
    Superseded,
    /// The finding's anchor is not anchored to the current revision.
    #[serde(rename = "anchor_unanchored")]
    Unanchored,
    /// The provenance mapping was downgraded to lower certainty.
    #[serde(rename = "downgraded_mapping")]
    DowngradedMapping,
    /// Verification proof stale or window elapsed.
    #[serde(rename = "verification_proof_stale")]
    StaleProof,
    /// Verification proof missing.
    #[serde(rename = "verification_proof_missing")]
    MissingProof,
}

impl ProblemDowngradeReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OriginFlattened => "origin_kind_flattened",
            Self::RawBacklinkMissing => "raw_output_backlink_missing",
            Self::SourceRefMissing => "source_ref_missing",
            Self::EvidenceMissing => "evidence_missing",
            Self::SupersededNotMarked => "superseded_state_not_marked",
            Self::ImportedOverlayClaimsLive => "imported_overlay_claims_live",
            Self::ConfidenceUnlabeled => "confidence_unlabeled",
            Self::AnchorMissing => "anchor_missing",
            Self::OwningChannelMissing => "owning_channel_missing",
            Self::SourceTaskUncorrelated => "source_task_uncorrelated",
            Self::EditorDecorationUncorrelated => "editor_decoration_uncorrelated",
            Self::TimelineUncorrelated => "timeline_uncorrelated",
            Self::StaleRun => "stale_run",
            Self::Superseded => "superseded_by_newer_run",
            Self::Unanchored => "anchor_unanchored",
            Self::DowngradedMapping => "downgraded_mapping",
            Self::StaleProof => "verification_proof_stale",
            Self::MissingProof => "verification_proof_missing",
        }
    }

    /// Whether this reason floors a row to [`ProblemRecordStatus::RawEvidenceOnly`].
    /// Each floor reason breaks the "stay reopenable / never flatten origin / never
    /// silently upgrade certainty" contract outright rather than merely aging out.
    pub const fn is_floor(self) -> bool {
        matches!(
            self,
            Self::OriginFlattened
                | Self::RawBacklinkMissing
                | Self::SourceRefMissing
                | Self::EvidenceMissing
                | Self::SupersededNotMarked
                | Self::ImportedOverlayClaimsLive
        )
    }

    /// Deterministic ordering index so recorded reason lists are stable across
    /// runs. Floor reasons sort first so the headline trigger is the most severe.
    const fn order_index(self) -> u8 {
        match self {
            Self::OriginFlattened => 0,
            Self::SourceRefMissing => 1,
            Self::RawBacklinkMissing => 2,
            Self::SupersededNotMarked => 3,
            Self::ImportedOverlayClaimsLive => 4,
            Self::EvidenceMissing => 5,
            Self::ConfidenceUnlabeled => 6,
            Self::AnchorMissing => 7,
            Self::OwningChannelMissing => 8,
            Self::SourceTaskUncorrelated => 9,
            Self::EditorDecorationUncorrelated => 10,
            Self::TimelineUncorrelated => 11,
            Self::Superseded => 12,
            Self::StaleRun => 13,
            Self::Unanchored => 14,
            Self::DowngradedMapping => 15,
            Self::StaleProof => 16,
            Self::MissingProof => 17,
        }
    }

    /// Reviewer label with underscores expanded.
    fn display(self) -> String {
        self.as_str().replace('_', " ")
    }
}

/// Sort reasons by their canonical order and drop duplicates.
fn order_reasons(mut reasons: Vec<ProblemDowngradeReason>) -> Vec<ProblemDowngradeReason> {
    reasons.sort_by_key(|reason| reason.order_index());
    reasons.dedup();
    reasons
}

// --------------------------------------------------------------------------- //
// Record sub-objects.
// --------------------------------------------------------------------------- //

/// Stable identifiers binding a problem record to the tool/run that produced it.
/// Lineage is reconstructed from these refs, never inferred from freeform display
/// text. Absent refs serialize as `null` so the schema's required keys stay
/// present. No absolute paths, command lines, or raw log bodies cross this block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemSourceRefs {
    /// Execution-context ref (required).
    pub execution_context_ref: String,
    /// The producing tool/adapter ref (language server, runner, parser, importer).
    pub source_tool_ref: Option<String>,
    /// Owning run ref.
    pub run_ref: Option<String>,
    /// Step ref within the run.
    pub step_ref: Option<String>,
    /// Provider ref (required for remote/pipeline/imported overlays).
    pub provider_ref: Option<String>,
    /// Build/toolchain ref.
    pub build_toolchain_ref: Option<String>,
    /// Host/target ref.
    pub host_target_ref: Option<String>,
    /// Task-event envelope ref the finding was projected from.
    pub task_event_envelope_ref: Option<String>,
    /// Backlink into the raw-output chunk that produced the finding.
    pub raw_output_backlink_ref: Option<String>,
}

/// A file/span anchor for a finding. Carries an opaque, workspace-relative file
/// ref plus line/column numbers — never an absolute path or raw source text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileSpanAnchor {
    /// Opaque workspace-relative file ref, or `null` when the finding is file-less.
    pub file_ref: Option<String>,
    /// One-based start line, or `null`.
    pub start_line: Option<u32>,
    /// One-based start column, or `null`.
    pub start_column: Option<u32>,
    /// One-based end line, or `null`.
    pub end_line: Option<u32>,
    /// One-based end column, or `null`.
    pub end_column: Option<u32>,
    /// Owning-symbol ref, or `null`.
    pub symbol_ref: Option<String>,
    /// Whether the anchor still resolves against the current revision.
    pub anchored_to_current_revision: bool,
}

impl FileSpanAnchor {
    /// Whether the anchor names a concrete file/span jump-to-source can resolve.
    pub fn is_present(&self) -> bool {
        opt_present(&self.file_ref) && self.start_line.is_some()
    }
}

/// How a Problems row is correlated to the rest of the surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemCorrelations {
    /// Editor decoration (gutter/squiggle) ref for the anchored finding.
    pub editor_decoration_ref: Option<String>,
    /// Activity-center timeline entry ref.
    pub timeline_entry_ref: Option<String>,
    /// The source task ref that can be rerun or inspected.
    pub source_task_ref: Option<String>,
    /// The owning output channel ref.
    pub owning_output_channel_ref: Option<String>,
    /// The owning output channel class.
    pub owning_output_channel_class: OutputChannelClass,
    /// Authority the row has to rerun/inspect the originating task/session.
    pub rerun_authority: RerunAuthority,
}

/// The origin/confidence honesty invariants a row re-derives rather than trusting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemEvidence {
    /// Structured and heuristic origins stay distinct.
    pub structured_vs_heuristic_distinct: bool,
    /// A heuristic parse keeps a raw-output backlink.
    pub raw_output_backlink_present: bool,
    /// The confidence tier/label is visible on the row.
    pub confidence_label_visible: bool,
    /// The producing tool/run lineage survives onto the row.
    pub preserves_source_run_lineage: bool,
    /// A superseded retry is visibly marked.
    pub superseded_state_marked: bool,
    /// An imported/remote finding stays read-only.
    pub imported_overlay_read_only: bool,
    /// The provenance mapping was downgraded to lower certainty.
    pub mapping_downgraded: bool,
}

// --------------------------------------------------------------------------- //
// Record row + derivation.
// --------------------------------------------------------------------------- //

/// One Problems-panel row: a run-derived finding bound to its source, anchor,
/// correlations, and freshness/stale/superseded state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemRecord {
    /// Stable problem id.
    pub problem_id: String,
    /// Human-readable, redaction-safe label summary.
    pub label_summary: String,
    /// Whether the row is publicly claimed.
    pub claim_posture: ClaimPosture,
    /// How the run/evidence originated.
    pub origin_class: OriginClass,
    /// Whether the finding came from a native structured diagnostic, normalized
    /// task event, imported provider annotation, or heuristic text parse.
    pub parse_class: ProblemSourceKind,
    /// Finding severity, independent of source confidence.
    pub severity: ProblemSeverity,
    /// Declared confidence tier.
    pub declared_confidence_tier: ConfidenceTier,
    /// Declared freshness/anchoring state of the run the finding came from.
    pub declared_freshness_state: FreshnessState,
    /// Source tool/run identity block.
    pub source: ProblemSourceRefs,
    /// File/span anchor block.
    pub anchor: FileSpanAnchor,
    /// Correlation block.
    pub correlations: ProblemCorrelations,
    /// Origin/confidence honesty block.
    pub evidence: ProblemEvidence,
    /// Certification-proof block (reuses the causality-matrix proof currency).
    pub verification: LaneVerification,
}

/// The per-action availability projection for a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionAvailabilitySet {
    /// Whether jump-to-source is available.
    pub jump_to_source: ActionAvailability,
    /// Whether open-owning-output is available.
    pub open_owning_output: ActionAvailability,
    /// Whether rerun-or-inspect-originator is available.
    pub rerun_or_inspect_originator: ActionAvailability,
}

impl ActionAvailabilitySet {
    /// The availability of one [`ProblemAction`].
    pub fn for_action(&self, action: ProblemAction) -> ActionAvailability {
        match action {
            ProblemAction::JumpToSource => self.jump_to_source,
            ProblemAction::OpenOwningOutput => self.open_owning_output,
            ProblemAction::RerunOrInspectOriginator => self.rerun_or_inspect_originator,
        }
    }
}

/// The re-derived decision for one Problems row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemRecordDecision {
    /// The headline status the row is eligible to claim from its posture/origin.
    pub claimed_status: ProblemRecordStatus,
    /// The effective status after re-derivation; never wider than the evidence.
    pub effective_status: ProblemRecordStatus,
    /// Ordered, de-duplicated downgrade reasons.
    pub active_downgrade_reasons: Vec<ProblemDowngradeReason>,
    /// Whether the effective status ranks below the claimed status.
    pub narrowed: bool,
    /// The re-derived per-action availability projection.
    pub actions: ActionAvailabilitySet,
}

impl ProblemRecordDecision {
    /// The headline downgrade trigger, when narrowed: the most severe reason.
    pub fn downgrade_trigger(&self) -> Option<ProblemDowngradeReason> {
        if self.narrowed {
            self.active_downgrade_reasons.first().copied()
        } else {
            None
        }
    }

    /// Whether a public surface rendering `rendered` for this row would overclaim.
    pub fn surface_overclaims(&self, rendered: ProblemRecordStatus) -> bool {
        self.effective_status.overclaims_as(rendered)
    }
}

impl ProblemRecord {
    /// Whether this row is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, ClaimPosture::LabsUnadvertised)
    }

    /// Whether this row is an inherently read-only overlay origin.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin_class.is_overlay()
    }

    /// The headline status this row is eligible to claim.
    pub fn claimed_status(&self) -> ProblemRecordStatus {
        if self.is_labs() {
            ProblemRecordStatus::LabsNotClaimed
        } else if self.is_overlay_origin() {
            ProblemRecordStatus::ReadOnlyImported
        } else {
            ProblemRecordStatus::Actionable
        }
    }

    /// Re-derive the per-action availability projection from the row's refs and
    /// rerun authority — independent of the declared status.
    pub fn action_availability(&self) -> ActionAvailabilitySet {
        let jump_to_source = if !self.anchor.is_present()
            || matches!(self.declared_freshness_state, FreshnessState::Missing)
        {
            ActionAvailability::Unavailable
        } else {
            ActionAvailability::Available
        };

        let open_owning_output = if !self
            .correlations
            .owning_output_channel_class
            .is_real_channel()
        {
            ActionAvailability::NotApplicable
        } else if opt_present(&self.correlations.owning_output_channel_ref) {
            ActionAvailability::Available
        } else {
            ActionAvailability::Unavailable
        };

        let rerun_or_inspect_originator = match self.correlations.rerun_authority {
            RerunAuthority::LocalRerunGranted => {
                if opt_present(&self.correlations.source_task_ref) {
                    ActionAvailability::Available
                } else {
                    ActionAvailability::Unavailable
                }
            }
            RerunAuthority::RequiresElevatedAuthority => ActionAvailability::GatedRequiresAuthority,
            RerunAuthority::RemoteInspectReadOnly => ActionAvailability::ReadOnlyInspectOnly,
            RerunAuthority::DeniedPolicy => ActionAvailability::Unavailable,
            RerunAuthority::NotApplicable => ActionAvailability::NotApplicable,
        };

        ActionAvailabilitySet {
            jump_to_source,
            open_owning_output,
            rerun_or_inspect_originator,
        }
    }

    /// Every downgrade reason this row fails to hold its headline status.
    ///
    /// `stale_window` is true when the packet verification window has elapsed by the
    /// evaluation time, which ages out a row resting on a current proof.
    pub fn record_reasons(&self, stale_window: bool) -> Vec<ProblemDowngradeReason> {
        let ev = &self.evidence;
        let overlay = self.is_overlay_origin();
        let mut reasons: Vec<ProblemDowngradeReason> = Vec::new();

        // Origin honesty: structured vs heuristic must stay distinct.
        if !ev.structured_vs_heuristic_distinct {
            reasons.push(ProblemDowngradeReason::OriginFlattened);
        }

        // A heuristic parse must keep a raw-output backlink and an explicit tier.
        if self.parse_class.is_heuristic() {
            if !ev.raw_output_backlink_present || !opt_present(&self.source.raw_output_backlink_ref)
            {
                reasons.push(ProblemDowngradeReason::RawBacklinkMissing);
            }
            if !self.declared_confidence_tier.is_heuristic_tier() || !ev.confidence_label_visible {
                reasons.push(ProblemDowngradeReason::ConfidenceUnlabeled);
            }
        } else if !ev.confidence_label_visible {
            reasons.push(ProblemDowngradeReason::ConfidenceUnlabeled);
        }

        // Source identity: the producing tool/run lineage must reach the row, else
        // the originator cannot be reopened.
        if !opt_present(&self.source.source_tool_ref) || !ev.preserves_source_run_lineage {
            reasons.push(ProblemDowngradeReason::SourceRefMissing);
        }

        // File/span anchor: jump-to-source needs a concrete anchor.
        if !self.anchor.is_present() {
            reasons.push(ProblemDowngradeReason::AnchorMissing);
        } else if !self.anchor.anchored_to_current_revision
            && !matches!(self.declared_freshness_state, FreshnessState::Unanchored)
        {
            // An anchor that no longer resolves but whose freshness does not already
            // declare it unanchored is a hidden drift.
            reasons.push(ProblemDowngradeReason::Unanchored);
        }

        // Owning output channel: a real channel must carry a stable channel ref.
        if self
            .correlations
            .owning_output_channel_class
            .is_real_channel()
            && !opt_present(&self.correlations.owning_output_channel_ref)
        {
            reasons.push(ProblemDowngradeReason::OwningChannelMissing);
        }

        // Originating task correlation: rerun/inspect needs a source task unless the
        // route is explicitly not-applicable.
        if !matches!(
            self.correlations.rerun_authority,
            RerunAuthority::NotApplicable
        ) && !opt_present(&self.correlations.source_task_ref)
        {
            reasons.push(ProblemDowngradeReason::SourceTaskUncorrelated);
        }

        // Editor decoration: an anchored finding should decorate the editor.
        if self.anchor.is_present() && !opt_present(&self.correlations.editor_decoration_ref) {
            reasons.push(ProblemDowngradeReason::EditorDecorationUncorrelated);
        }

        // Timeline correlation.
        if !opt_present(&self.correlations.timeline_entry_ref) {
            reasons.push(ProblemDowngradeReason::TimelineUncorrelated);
        }

        // Freshness / superseded / missing / unanchored. Stale and superseded stay
        // visibly classified rather than dropped or silently upgraded.
        match self.declared_freshness_state {
            FreshnessState::Missing => reasons.push(ProblemDowngradeReason::EvidenceMissing),
            FreshnessState::SupersededByNewerRun => {
                if ev.superseded_state_marked {
                    reasons.push(ProblemDowngradeReason::Superseded);
                } else {
                    reasons.push(ProblemDowngradeReason::SupersededNotMarked);
                }
            }
            FreshnessState::Unanchored => reasons.push(ProblemDowngradeReason::Unanchored),
            FreshnessState::StaleExpired if !overlay => {
                reasons.push(ProblemDowngradeReason::StaleRun);
            }
            _ => {}
        }

        // Downgraded provenance mapping stays visibly classified.
        if ev.mapping_downgraded {
            reasons.push(ProblemDowngradeReason::DowngradedMapping);
        }

        // Certification-proof currency.
        match self.verification.proof_currency {
            ProofCurrency::MissingProof => reasons.push(ProblemDowngradeReason::MissingProof),
            ProofCurrency::StaleExpired | ProofCurrency::RequiresReview => {
                reasons.push(ProblemDowngradeReason::StaleProof);
            }
            ProofCurrency::VerifiedCurrent | ProofCurrency::CachedWithinWindow if stale_window => {
                reasons.push(ProblemDowngradeReason::StaleProof);
            }
            _ => {}
        }

        // Imported/remote overlays must stay read-only.
        if overlay && !ev.imported_overlay_read_only {
            reasons.push(ProblemDowngradeReason::ImportedOverlayClaimsLive);
        }

        order_reasons(reasons)
    }

    /// Re-derive the effective status, reasons, narrowed flag, and action set.
    pub fn narrow(&self, stale_window: bool) -> ProblemRecordDecision {
        let claimed = self.claimed_status();
        let actions = self.action_availability();

        if matches!(claimed, ProblemRecordStatus::LabsNotClaimed) {
            return ProblemRecordDecision {
                claimed_status: ProblemRecordStatus::LabsNotClaimed,
                effective_status: ProblemRecordStatus::LabsNotClaimed,
                active_downgrade_reasons: Vec::new(),
                narrowed: false,
                actions,
            };
        }

        let reasons = self.record_reasons(stale_window);
        let floored = reasons.iter().any(|reason| reason.is_floor());

        let effective = if floored {
            ProblemRecordStatus::RawEvidenceOnly
        } else if !reasons.is_empty() {
            // An imported overlay is already the minimal honest status: any further
            // gap means we can no longer certify even the read-only overlay.
            if matches!(claimed, ProblemRecordStatus::ReadOnlyImported) {
                ProblemRecordStatus::RawEvidenceOnly
            } else {
                ProblemRecordStatus::NarrowedActionable
            }
        } else {
            claimed
        };

        let narrowed = matches!(
            (effective.rank(), claimed.rank()),
            (Some(eff), Some(claim)) if eff < claim
        );

        ProblemRecordDecision {
            claimed_status: claimed,
            effective_status: effective,
            active_downgrade_reasons: reasons,
            narrowed,
            actions,
        }
    }

    /// The effective confidence tier: a floored row cannot assert a tier beyond
    /// unmapped/needs-review.
    pub fn effective_confidence(&self, effective: ProblemRecordStatus) -> ConfidenceTier {
        if matches!(effective, ProblemRecordStatus::RawEvidenceOnly) {
            ConfidenceTier::UnmappedRequiresReview
        } else {
            self.declared_confidence_tier
        }
    }

    /// A precise, non-generic reviewer label for a narrowed/floored row.
    pub fn narrowed_label(&self, decision: &ProblemRecordDecision) -> Option<String> {
        if !decision.narrowed {
            return None;
        }
        let trigger = decision
            .downgrade_trigger()
            .map_or_else(|| "narrowed".to_owned(), ProblemDowngradeReason::display);
        let claimed = decision.claimed_status.as_str();
        let effective = decision.effective_status;
        let label = if matches!(effective, ProblemRecordStatus::RawEvidenceOnly) {
            format!(
                "Floored to {} below the {claimed} row: {trigger}; the raw-output backlink stays reopenable rather than rendering a clean-but-false actionable row",
                effective.as_str()
            )
        } else {
            format!(
                "Held at {} below the {claimed} row: {trigger}; the finding stays jumpable and inspectable until current evidence replaces it",
                effective.as_str()
            )
        };
        Some(label)
    }

    /// Whether a non-labs row that floors keeps a raw-output fallback rather than
    /// hiding lineage behind a clean-but-false actionable claim.
    fn floored_row_keeps_fallback(&self, effective: ProblemRecordStatus) -> bool {
        if !matches!(effective, ProblemRecordStatus::RawEvidenceOnly) {
            return true;
        }
        ev_backlink_present(self)
    }

    /// Structural row checks that hold independently of the narrowing derivation.
    fn structural_violations(&self, out: &mut Vec<M5ProblemRecordsViolation>) {
        if self.problem_id.trim().is_empty()
            || self.label_summary.trim().is_empty()
            || self.source.execution_context_ref.trim().is_empty()
        {
            out.push(M5ProblemRecordsViolation::RowMissingIdentity);
        }
        if self.is_overlay_origin() && !opt_present(&self.source.provider_ref) {
            out.push(M5ProblemRecordsViolation::OverlayMissingProviderRef);
        }
    }
}

/// Whether the row retains a raw-output backlink (ref present and flagged).
fn ev_backlink_present(record: &ProblemRecord) -> bool {
    record.evidence.raw_output_backlink_present
        && opt_present(&record.source.raw_output_backlink_ref)
}

/// Whether an optional ref is present and non-empty.
fn opt_present(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|inner| !inner.trim().is_empty())
}

// --------------------------------------------------------------------------- //
// Packet.
// --------------------------------------------------------------------------- //

/// Constructor input for an [`M5ProblemRecordSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProblemRecordSetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-record Problems rows.
    pub records: Vec<ProblemRecord>,
}

/// Export-safe M5 Problems-record set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProblemRecordSetPacket {
    /// Record kind; must equal [`M5_PROBLEM_RECORDS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PROBLEM_RECORDS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal [`M5_PROBLEM_RECORDS_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-record Problems rows.
    pub records: Vec<ProblemRecord>,
}

/// The distribution of effective statuses across a record set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusDistribution {
    /// Rows effective at [`ProblemRecordStatus::Actionable`].
    pub actionable: usize,
    /// Rows effective at [`ProblemRecordStatus::NarrowedActionable`].
    pub narrowed: usize,
    /// Rows effective at [`ProblemRecordStatus::ReadOnlyImported`].
    pub read_only_imported: usize,
    /// Rows effective at [`ProblemRecordStatus::RawEvidenceOnly`].
    pub raw_evidence_only: usize,
    /// Rows effective at [`ProblemRecordStatus::LabsNotClaimed`].
    pub labs: usize,
}

impl M5ProblemRecordSetPacket {
    /// Builds a Problems-record set packet, sealing the record-kind, schema, and
    /// taxonomy version constants.
    pub fn new(input: M5ProblemRecordSetInput) -> Self {
        Self {
            record_kind: M5_PROBLEM_RECORDS_RECORD_KIND.to_owned(),
            schema_version: M5_PROBLEM_RECORDS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_PROBLEM_RECORDS_TAXONOMY_VERSION,
            redaction_class_token: input.redaction_class_token,
            verification_freshness: input.verification_freshness,
            records: input.records,
        }
    }

    /// Whether the packet verification window has elapsed by `as_of`.
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

    /// Re-derive the decision for every row, paired with its problem id.
    pub fn decisions(&self) -> Vec<(String, ProblemRecordDecision)> {
        let stale_window = self.stale_window();
        self.records
            .iter()
            .map(|record| (record.problem_id.clone(), record.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective statuses.
    pub fn status_distribution(&self) -> StatusDistribution {
        let stale_window = self.stale_window();
        let mut dist = StatusDistribution {
            actionable: 0,
            narrowed: 0,
            read_only_imported: 0,
            raw_evidence_only: 0,
            labs: 0,
        };
        for record in &self.records {
            match record.narrow(stale_window).effective_status {
                ProblemRecordStatus::Actionable => dist.actionable += 1,
                ProblemRecordStatus::NarrowedActionable => dist.narrowed += 1,
                ProblemRecordStatus::ReadOnlyImported => dist.read_only_imported += 1,
                ProblemRecordStatus::RawEvidenceOnly => dist.raw_evidence_only += 1,
                ProblemRecordStatus::LabsNotClaimed => dist.labs += 1,
            }
        }
        dist
    }

    /// Count of rows whose effective status ranks below their claimed status.
    pub fn narrowed_record_count(&self) -> usize {
        let stale_window = self.stale_window();
        self.records
            .iter()
            .filter(|record| record.narrow(stale_window).narrowed)
            .count()
    }

    /// Origin classes represented by some row.
    pub fn represented_origin_classes(&self) -> BTreeSet<OriginClass> {
        self.records
            .iter()
            .map(|record| record.origin_class)
            .collect()
    }

    /// Problem-source kinds represented by some row.
    pub fn represented_source_kinds(&self) -> BTreeSet<ProblemSourceKind> {
        self.records
            .iter()
            .map(|record| record.parse_class)
            .collect()
    }

    /// Validate the per-record Problems invariants.
    pub fn validate(&self) -> Vec<M5ProblemRecordsViolation> {
        use M5ProblemRecordsViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_PROBLEM_RECORDS_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_PROBLEM_RECORDS_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_PROBLEM_RECORDS_TAXONOMY_VERSION {
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
        if self.records.is_empty() {
            violations.push(V::EmptyRecords);
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for record in &self.records {
            if !seen.insert(record.problem_id.as_str()) {
                violations.push(V::DuplicateProblemId);
            }
        }

        // Every claimed problem-source kind must be represented so a user can always
        // inspect the four origins (native diagnostic, normalized task event,
        // imported provider, heuristic parse).
        let kinds = self.represented_source_kinds();
        for kind in [
            ProblemSourceKind::StructuredLanguageDiagnostic,
            ProblemSourceKind::NormalizedTaskEvent,
            ProblemSourceKind::HeuristicOutputParse,
            ProblemSourceKind::ImportedProviderAnnotation,
        ] {
            if !kinds.contains(&kind) {
                violations.push(V::ProblemSourceKindMissing);
                break;
            }
        }

        let stale_window = self.stale_window();
        let mut demonstrates_narrowing = false;
        for record in &self.records {
            record.structural_violations(&mut violations);
            let decision = record.narrow(stale_window);
            if decision.narrowed {
                demonstrates_narrowing = true;
                if decision.downgrade_trigger().is_none()
                    || record
                        .narrowed_label(&decision)
                        .map_or(true, |label| label_is_generic(&label))
                {
                    violations.push(V::NarrowedRowMissingLabelOrTrigger);
                }
            }
            if !record.floored_row_keeps_fallback(decision.effective_status) {
                violations.push(V::FlooredRowLosesFallback);
            }
        }
        if !demonstrates_narrowing {
            violations.push(V::DowngradedRowCaseMissing);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("problem-record set packet serializes"),
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
        serde_json::to_string_pretty(self).expect("problem-record set packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.status_distribution();
        let mut out = String::new();
        out.push_str("# M5 Problem Records — source-task correlation and rerun/jump parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Rows: {}\n", self.records.len()));
        out.push_str(&format!(
            "- Effective: {} actionable, {} narrowed, {} read-only imported, {} raw-evidence-only, {} labs\n\n",
            dist.actionable, dist.narrowed, dist.read_only_imported, dist.raw_evidence_only, dist.labs
        ));

        out.push_str(
            "| Row | Origin | Parse | Claimed | Effective | Confidence | Jump | Output | Rerun |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
        for record in &self.records {
            let decision = record.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
                record.problem_id,
                record.origin_class.as_str(),
                record.parse_class.as_str(),
                decision.claimed_status.as_str(),
                decision.effective_status.as_str(),
                record
                    .effective_confidence(decision.effective_status)
                    .as_str(),
                decision.actions.jump_to_source.as_str(),
                decision.actions.open_owning_output.as_str(),
                decision.actions.rerun_or_inspect_originator.as_str(),
            ));
        }

        out.push('\n');
        for record in &self.records {
            let decision = record.narrow(stale_window);
            if let Some(label) = record.narrowed_label(&decision) {
                out.push_str(&format!(
                    "- Narrowed: `{}` — {}\n",
                    record.problem_id, label
                ));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum M5ProblemRecordsArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5ProblemRecordsViolation>),
}

impl fmt::Display for M5ProblemRecordsArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(f, "problem-record set support export parse error: {err}")
            }
            Self::Validation(violations) => write!(
                f,
                "problem-record set support export failed validation: {violations:?}"
            ),
        }
    }
}

impl Error for M5ProblemRecordsArtifactError {}

/// Invariant violations reported by [`M5ProblemRecordSetPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProblemRecordsViolation {
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
    /// The packet carries no records.
    EmptyRecords,
    /// Two rows share a problem id.
    DuplicateProblemId,
    /// A required problem-source kind is unrepresented.
    ProblemSourceKindMissing,
    /// A row is missing its problem id, label, or execution-context ref.
    RowMissingIdentity,
    /// An overlay-origin row does not name its provider.
    OverlayMissingProviderRef,
    /// A floored row lost its raw-output reopen fallback.
    FlooredRowLosesFallback,
    /// A narrowed row is missing its precise label or trigger.
    NarrowedRowMissingLabelOrTrigger,
    /// No row demonstrates the auto-narrowing rule.
    DowngradedRowCaseMissing,
    /// Export-safe JSON carried forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ProblemRecordsViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::WrongTaxonomyVersion => "wrong_taxonomy_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidRedactionClass => "invalid_redaction_class",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::EmptyRecords => "empty_records",
            Self::DuplicateProblemId => "duplicate_problem_id",
            Self::ProblemSourceKindMissing => "problem_source_kind_missing",
            Self::RowMissingIdentity => "row_missing_identity",
            Self::OverlayMissingProviderRef => "overlay_missing_provider_ref",
            Self::FlooredRowLosesFallback => "floored_row_loses_fallback",
            Self::NarrowedRowMissingLabelOrTrigger => "narrowed_row_missing_label_or_trigger",
            Self::DowngradedRowCaseMissing => "downgraded_row_case_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Loads and validates the checked-in canonical support export.
///
/// This is the canonical entry point downstream Problems, output, diagnostics, AI
/// evidence, support export, review, CLI/headless, and docs surfaces use to ingest
/// the frozen Problems-record set instead of cloning provider-local state.
///
/// # Errors
///
/// Returns [`M5ProblemRecordsArtifactError`] when the artifact cannot be parsed or
/// fails validation.
pub fn current_m5_problem_record_set(
) -> Result<M5ProblemRecordSetPacket, M5ProblemRecordsArtifactError> {
    let packet: M5ProblemRecordSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/tooling/m5-problem-records/support_export.json"
    )))
    .map_err(M5ProblemRecordsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ProblemRecordsArtifactError::Validation(violations))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// Stable packet id for the canonical seed.
pub const M5_PROBLEM_RECORDS_PACKET_ID: &str = "m5-problem-records:stable:0001";

/// Mint/evaluation timestamp for the canonical seed.
const SEED_AS_OF: &str = "2026-06-21T00:00:00Z";

/// Builds the canonical seed packet. The checked-in support export is the byte
/// image of this builder; the `dump_m5_problem_records` example regenerates it and
/// a test asserts the two stay identical so the artifact never drifts from Rust.
pub fn seeded_problem_record_set() -> M5ProblemRecordSetPacket {
    M5ProblemRecordSetPacket::new(M5ProblemRecordSetInput {
        packet_id: M5_PROBLEM_RECORDS_PACKET_ID.to_owned(),
        label: "M5 Problems records — source-task correlation and rerun/jump parity".to_owned(),
        as_of: SEED_AS_OF.to_owned(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        verification_freshness: VerificationFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_AS_OF.to_owned(),
            auto_downgrade_on_stale: true,
        },
        records: seed_records(),
    })
}

fn seed_records() -> Vec<ProblemRecord> {
    vec![
        // 1. A clean local structured language diagnostic.
        ProblemRecord {
            problem_id: "problem:local-structured-diagnostic:0001".to_owned(),
            label_summary:
                "Local compiler/language-server error anchored to a file/span and owning symbol."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            parse_class: ProblemSourceKind::StructuredLanguageDiagnostic,
            severity: ProblemSeverity::Error,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            source: ProblemSourceRefs {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                source_tool_ref: Some("tool.local.language-server".to_owned()),
                run_ref: Some("run.local.diagnostic.0001".to_owned()),
                step_ref: None,
                provider_ref: None,
                build_toolchain_ref: Some("toolchain.local.rustc.pinned".to_owned()),
                host_target_ref: Some("host.local.desktop.primary".to_owned()),
                task_event_envelope_ref: Some("task-event.local.diagnostic.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.diagnostic.0001".to_owned()),
            },
            anchor: anchor("file.local.src.lib", 42, 5, 42, 18, "symbol.local.fn.parse"),
            correlations: ProblemCorrelations {
                editor_decoration_ref: Some("decoration.local.diagnostic.0001".to_owned()),
                timeline_entry_ref: Some("timeline.local.diagnostic.0001".to_owned()),
                source_task_ref: Some("task.local.analysis.0001".to_owned()),
                owning_output_channel_ref: None,
                owning_output_channel_class: OutputChannelClass::NotApplicable,
                rerun_authority: RerunAuthority::LocalRerunGranted,
            },
            evidence: clean_evidence(),
            verification: verified("proof.local.diagnostic.0001"),
        },
        // 2. A clean local test failure projected from a normalized task event.
        ProblemRecord {
            problem_id: "problem:local-test-normalized-event:0001".to_owned(),
            label_summary:
                "Local test failure projected from a normalized task event with full run lineage."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTest,
            parse_class: ProblemSourceKind::NormalizedTaskEvent,
            severity: ProblemSeverity::Error,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            source: ProblemSourceRefs {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                source_tool_ref: Some("tool.local.test-runner".to_owned()),
                run_ref: Some("run.local.test.0001".to_owned()),
                step_ref: Some("step.local.test.case.0001".to_owned()),
                provider_ref: None,
                build_toolchain_ref: Some("toolchain.local.rustc.pinned".to_owned()),
                host_target_ref: Some("host.local.desktop.primary".to_owned()),
                task_event_envelope_ref: Some("task-event.local.test.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.test.0001".to_owned()),
            },
            anchor: anchor("file.local.tests.parse", 17, 1, 17, 40, "symbol.local.test.roundtrip"),
            correlations: ProblemCorrelations {
                editor_decoration_ref: Some("decoration.local.test.0001".to_owned()),
                timeline_entry_ref: Some("timeline.local.test.0001".to_owned()),
                source_task_ref: Some("task.local.test.0001".to_owned()),
                owning_output_channel_ref: Some("channel.local.test.0001".to_owned()),
                owning_output_channel_class: OutputChannelClass::TaskTestDebugOutput,
                rerun_authority: RerunAuthority::LocalRerunGranted,
            },
            evidence: clean_evidence(),
            verification: verified("proof.local.test.0001"),
        },
        // 3. A clean local heuristic parse with an explicit tier and raw backlink.
        ProblemRecord {
            problem_id: "problem:local-heuristic-parse:0001".to_owned(),
            label_summary:
                "Heuristic problem-matcher finding over raw build output, kept distinct from structured diagnostics."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            parse_class: ProblemSourceKind::HeuristicOutputParse,
            severity: ProblemSeverity::Warning,
            declared_confidence_tier: ConfidenceTier::HeuristicMedium,
            declared_freshness_state: FreshnessState::Live,
            source: ProblemSourceRefs {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                source_tool_ref: Some("tool.local.problem-matcher".to_owned()),
                run_ref: Some("run.local.build.0001".to_owned()),
                step_ref: None,
                provider_ref: None,
                build_toolchain_ref: Some("toolchain.local.node.pinned".to_owned()),
                host_target_ref: Some("host.local.desktop.primary".to_owned()),
                task_event_envelope_ref: Some("task-event.local.build.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.build.0001".to_owned()),
            },
            anchor: anchor("file.local.src.bundle", 3, 1, 3, 12, "symbol.local.module.bundle"),
            correlations: ProblemCorrelations {
                editor_decoration_ref: Some("decoration.local.build.0001".to_owned()),
                timeline_entry_ref: Some("timeline.local.build.0001".to_owned()),
                source_task_ref: Some("task.local.build.0001".to_owned()),
                owning_output_channel_ref: Some("channel.local.build.0001".to_owned()),
                owning_output_channel_class: OutputChannelClass::TaskTestDebugOutput,
                rerun_authority: RerunAuthority::LocalRerunGranted,
            },
            evidence: clean_evidence(),
            verification: verified("proof.local.build.0001"),
        },
        // 4. An imported provider annotation: inspect read-only, never live local.
        ProblemRecord {
            problem_id: "problem:imported-provider-annotation:0001".to_owned(),
            label_summary:
                "Imported CI annotation surfaced read-only with provider mapping and reopenable provider run."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::ImportedProviderEvidence,
            parse_class: ProblemSourceKind::ImportedProviderAnnotation,
            severity: ProblemSeverity::Error,
            declared_confidence_tier: ConfidenceTier::ProviderMapped,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            source: ProblemSourceRefs {
                execution_context_ref: "exec-context.imported.ci.primary".to_owned(),
                source_tool_ref: Some("tool.imported.ci-annotation".to_owned()),
                run_ref: Some("run.imported.ci.0001".to_owned()),
                step_ref: Some("step.imported.ci.job.0001".to_owned()),
                provider_ref: Some("provider.ci.actions".to_owned()),
                build_toolchain_ref: Some("toolchain.imported.ci.pinned".to_owned()),
                host_target_ref: Some("host.imported.ci.runner".to_owned()),
                task_event_envelope_ref: Some("task-event.imported.ci.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.imported.ci.0001".to_owned()),
            },
            anchor: anchor("file.local.src.api", 88, 9, 88, 30, "symbol.local.fn.handler"),
            correlations: ProblemCorrelations {
                editor_decoration_ref: Some("decoration.imported.ci.0001".to_owned()),
                timeline_entry_ref: Some("timeline.imported.ci.0001".to_owned()),
                source_task_ref: Some("task.imported.ci.0001".to_owned()),
                owning_output_channel_ref: Some("channel.imported.ci.0001".to_owned()),
                owning_output_channel_class: OutputChannelClass::RemoteProviderImportedOutput,
                rerun_authority: RerunAuthority::RemoteInspectReadOnly,
            },
            evidence: clean_evidence(),
            verification: LaneVerification {
                proof_currency: ProofCurrency::ImportedCurrent,
                proof_ref: Some("proof.imported.ci.0001".to_owned()),
            },
        },
        // 5. A pipeline/provider run finding: read-only overlay.
        ProblemRecord {
            problem_id: "problem:pipeline-provider-run:0001".to_owned(),
            label_summary:
                "Pipeline/provider run failure surfaced as a read-only overlay with its provider run reopenable."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedBeta,
            origin_class: OriginClass::PipelineProviderRun,
            parse_class: ProblemSourceKind::NormalizedTaskEvent,
            severity: ProblemSeverity::Error,
            declared_confidence_tier: ConfidenceTier::ProviderMapped,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            source: ProblemSourceRefs {
                execution_context_ref: "exec-context.pipeline.primary".to_owned(),
                source_tool_ref: Some("tool.pipeline.runner".to_owned()),
                run_ref: Some("run.pipeline.0001".to_owned()),
                step_ref: Some("step.pipeline.stage.0001".to_owned()),
                provider_ref: Some("provider.pipeline.hosted".to_owned()),
                build_toolchain_ref: Some("toolchain.pipeline.pinned".to_owned()),
                host_target_ref: Some("host.pipeline.runner".to_owned()),
                task_event_envelope_ref: Some("task-event.pipeline.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.pipeline.0001".to_owned()),
            },
            anchor: anchor("file.local.src.worker", 12, 1, 12, 25, "symbol.local.fn.worker"),
            correlations: ProblemCorrelations {
                editor_decoration_ref: Some("decoration.pipeline.0001".to_owned()),
                timeline_entry_ref: Some("timeline.pipeline.0001".to_owned()),
                source_task_ref: Some("task.pipeline.0001".to_owned()),
                owning_output_channel_ref: Some("channel.pipeline.0001".to_owned()),
                owning_output_channel_class: OutputChannelClass::RemoteProviderImportedOutput,
                rerun_authority: RerunAuthority::RemoteInspectReadOnly,
            },
            evidence: clean_evidence(),
            verification: LaneVerification {
                proof_currency: ProofCurrency::ImportedCurrent,
                proof_ref: Some("proof.pipeline.0001".to_owned()),
            },
        },
        // 6. A notebook-run finding superseded by a newer run, kept visibly marked.
        ProblemRecord {
            problem_id: "problem:notebook-superseded:0001".to_owned(),
            label_summary:
                "Notebook-cell failure superseded by a newer run, kept visibly classified until replaced."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::NotebookRun,
            parse_class: ProblemSourceKind::NormalizedTaskEvent,
            severity: ProblemSeverity::Error,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::SupersededByNewerRun,
            source: ProblemSourceRefs {
                execution_context_ref: "exec-context.local.notebook.primary".to_owned(),
                source_tool_ref: Some("tool.local.notebook-kernel".to_owned()),
                run_ref: Some("run.local.notebook.0001".to_owned()),
                step_ref: Some("step.local.notebook.cell.0001".to_owned()),
                provider_ref: None,
                build_toolchain_ref: Some("toolchain.local.python.pinned".to_owned()),
                host_target_ref: Some("host.local.desktop.primary".to_owned()),
                task_event_envelope_ref: Some("task-event.local.notebook.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.notebook.0001".to_owned()),
            },
            anchor: anchor("file.local.notebook.cells", 6, 1, 6, 20, "symbol.local.cell.train"),
            correlations: ProblemCorrelations {
                editor_decoration_ref: Some("decoration.local.notebook.0001".to_owned()),
                timeline_entry_ref: Some("timeline.local.notebook.0001".to_owned()),
                source_task_ref: Some("task.local.notebook.0001".to_owned()),
                owning_output_channel_ref: Some("channel.local.notebook.0001".to_owned()),
                owning_output_channel_class: OutputChannelClass::TaskTestDebugOutput,
                rerun_authority: RerunAuthority::LocalRerunGranted,
            },
            evidence: ProblemEvidence {
                superseded_state_marked: true,
                ..clean_evidence()
            },
            verification: verified("proof.local.notebook.0001"),
        },
        // 7. A headless-automation finding from a stale run.
        ProblemRecord {
            problem_id: "problem:headless-stale-run:0001".to_owned(),
            label_summary:
                "Headless-automation heuristic finding from a stale run, kept visibly classified as stale."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::HeadlessAutomation,
            parse_class: ProblemSourceKind::HeuristicOutputParse,
            severity: ProblemSeverity::Warning,
            declared_confidence_tier: ConfidenceTier::HeuristicLow,
            declared_freshness_state: FreshnessState::StaleExpired,
            source: ProblemSourceRefs {
                execution_context_ref: "exec-context.headless.primary".to_owned(),
                source_tool_ref: Some("tool.headless.problem-matcher".to_owned()),
                run_ref: Some("run.headless.0001".to_owned()),
                step_ref: None,
                provider_ref: None,
                build_toolchain_ref: Some("toolchain.headless.pinned".to_owned()),
                host_target_ref: Some("host.headless.runner".to_owned()),
                task_event_envelope_ref: Some("task-event.headless.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.headless.0001".to_owned()),
            },
            anchor: anchor("file.local.src.cli", 21, 1, 21, 15, "symbol.local.fn.main"),
            correlations: ProblemCorrelations {
                editor_decoration_ref: Some("decoration.headless.0001".to_owned()),
                timeline_entry_ref: Some("timeline.headless.0001".to_owned()),
                source_task_ref: Some("task.headless.0001".to_owned()),
                owning_output_channel_ref: Some("channel.headless.0001".to_owned()),
                owning_output_channel_class: OutputChannelClass::TaskTestDebugOutput,
                rerun_authority: RerunAuthority::LocalRerunGranted,
            },
            evidence: clean_evidence(),
            verification: verified("proof.headless.0001"),
        },
        // 8. A local finding whose provenance mapping was downgraded.
        ProblemRecord {
            problem_id: "problem:local-downgraded-mapping:0001".to_owned(),
            label_summary:
                "Local heuristic finding whose provenance mapping was downgraded to lower certainty."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            parse_class: ProblemSourceKind::HeuristicOutputParse,
            severity: ProblemSeverity::Info,
            declared_confidence_tier: ConfidenceTier::HeuristicLow,
            declared_freshness_state: FreshnessState::Live,
            source: ProblemSourceRefs {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                source_tool_ref: Some("tool.local.problem-matcher".to_owned()),
                run_ref: Some("run.local.lint.0001".to_owned()),
                step_ref: None,
                provider_ref: None,
                build_toolchain_ref: Some("toolchain.local.node.pinned".to_owned()),
                host_target_ref: Some("host.local.desktop.primary".to_owned()),
                task_event_envelope_ref: Some("task-event.local.lint.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.lint.0001".to_owned()),
            },
            anchor: anchor("file.local.src.style", 9, 3, 9, 24, "symbol.local.fn.render"),
            correlations: ProblemCorrelations {
                editor_decoration_ref: Some("decoration.local.lint.0001".to_owned()),
                timeline_entry_ref: Some("timeline.local.lint.0001".to_owned()),
                source_task_ref: Some("task.local.lint.0001".to_owned()),
                owning_output_channel_ref: Some("channel.local.lint.0001".to_owned()),
                owning_output_channel_class: OutputChannelClass::TaskTestDebugOutput,
                rerun_authority: RerunAuthority::LocalRerunGranted,
            },
            evidence: ProblemEvidence {
                mapping_downgraded: true,
                ..clean_evidence()
            },
            verification: verified("proof.local.lint.0001"),
        },
        // 9. An extension-owned finding whose rerun route is authority-gated. The row
        //    stays actionable; only the rerun action is surfaced as gated.
        ProblemRecord {
            problem_id: "problem:extension-gated-rerun:0001".to_owned(),
            label_summary:
                "Extension-owned task failure whose rerun is permitted but gated behind explicit authority."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::ExtensionOwnedRun,
            parse_class: ProblemSourceKind::NormalizedTaskEvent,
            severity: ProblemSeverity::Warning,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            source: ProblemSourceRefs {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                source_tool_ref: Some("tool.extension.task".to_owned()),
                run_ref: Some("run.extension.0001".to_owned()),
                step_ref: None,
                provider_ref: None,
                build_toolchain_ref: Some("toolchain.local.node.pinned".to_owned()),
                host_target_ref: Some("host.local.desktop.primary".to_owned()),
                task_event_envelope_ref: Some("task-event.extension.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.extension.0001".to_owned()),
            },
            anchor: anchor("file.local.src.plugin", 30, 1, 30, 16, "symbol.local.fn.activate"),
            correlations: ProblemCorrelations {
                editor_decoration_ref: Some("decoration.extension.0001".to_owned()),
                timeline_entry_ref: Some("timeline.extension.0001".to_owned()),
                source_task_ref: Some("task.extension.0001".to_owned()),
                owning_output_channel_ref: Some("channel.extension.0001".to_owned()),
                owning_output_channel_class: OutputChannelClass::ExtensionAiToolOutput,
                rerun_authority: RerunAuthority::RequiresElevatedAuthority,
            },
            evidence: clean_evidence(),
            verification: verified("proof.extension.0001"),
        },
        // 10. A local finding whose source-tool lineage was lost: floors but keeps a
        //     raw-output backlink as the reopen fallback.
        ProblemRecord {
            problem_id: "problem:local-lineage-lost-floored:0001".to_owned(),
            label_summary:
                "Local heuristic finding whose source-tool lineage was lost; floored to a raw-output backlink."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            parse_class: ProblemSourceKind::HeuristicOutputParse,
            severity: ProblemSeverity::Warning,
            declared_confidence_tier: ConfidenceTier::HeuristicLow,
            declared_freshness_state: FreshnessState::Live,
            source: ProblemSourceRefs {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                source_tool_ref: None,
                run_ref: None,
                step_ref: None,
                provider_ref: None,
                build_toolchain_ref: None,
                host_target_ref: Some("host.local.desktop.primary".to_owned()),
                task_event_envelope_ref: None,
                raw_output_backlink_ref: Some("raw.local.orphan.0001".to_owned()),
            },
            anchor: anchor("file.local.src.orphan", 2, 1, 2, 10, "symbol.local.fn.orphan"),
            correlations: ProblemCorrelations {
                editor_decoration_ref: Some("decoration.local.orphan.0001".to_owned()),
                timeline_entry_ref: Some("timeline.local.orphan.0001".to_owned()),
                source_task_ref: None,
                owning_output_channel_ref: Some("channel.local.orphan.0001".to_owned()),
                owning_output_channel_class: OutputChannelClass::TaskTestDebugOutput,
                rerun_authority: RerunAuthority::NotApplicable,
            },
            evidence: ProblemEvidence {
                preserves_source_run_lineage: false,
                ..clean_evidence()
            },
            verification: LaneVerification {
                proof_currency: ProofCurrency::RequiresReview,
                proof_ref: None,
            },
        },
        // 11. A Labs cross-run correlation row: makes no public claim.
        ProblemRecord {
            problem_id: "problem:labs-cross-run-correlation:0001".to_owned(),
            label_summary:
                "Labs cross-run correlation experiment; makes no public actionability claim and is never widened."
                    .to_owned(),
            claim_posture: ClaimPosture::LabsUnadvertised,
            origin_class: OriginClass::AiTriggeredRun,
            parse_class: ProblemSourceKind::NormalizedTaskEvent,
            severity: ProblemSeverity::Info,
            declared_confidence_tier: ConfidenceTier::UnmappedRequiresReview,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            source: ProblemSourceRefs {
                execution_context_ref: "exec-context.labs.primary".to_owned(),
                source_tool_ref: Some("tool.labs.correlator".to_owned()),
                run_ref: None,
                step_ref: None,
                provider_ref: None,
                build_toolchain_ref: None,
                host_target_ref: None,
                task_event_envelope_ref: None,
                raw_output_backlink_ref: Some("raw.labs.0001".to_owned()),
            },
            anchor: FileSpanAnchor {
                file_ref: None,
                start_line: None,
                start_column: None,
                end_line: None,
                end_column: None,
                symbol_ref: None,
                anchored_to_current_revision: false,
            },
            correlations: ProblemCorrelations {
                editor_decoration_ref: None,
                timeline_entry_ref: None,
                source_task_ref: None,
                owning_output_channel_ref: None,
                owning_output_channel_class: OutputChannelClass::NotApplicable,
                rerun_authority: RerunAuthority::NotApplicable,
            },
            evidence: clean_evidence(),
            verification: LaneVerification {
                proof_currency: ProofCurrency::RequiresReview,
                proof_ref: None,
            },
        },
    ]
}

/// A fully honest evidence block; perturbations toggle one field at a time.
fn clean_evidence() -> ProblemEvidence {
    ProblemEvidence {
        structured_vs_heuristic_distinct: true,
        raw_output_backlink_present: true,
        confidence_label_visible: true,
        preserves_source_run_lineage: true,
        superseded_state_marked: true,
        imported_overlay_read_only: true,
        mapping_downgraded: false,
    }
}

/// A current verification proof anchored to `proof_ref`.
fn verified(proof_ref: &str) -> LaneVerification {
    LaneVerification {
        proof_currency: ProofCurrency::VerifiedCurrent,
        proof_ref: Some(proof_ref.to_owned()),
    }
}

/// A file/span anchor that resolves against the current revision.
fn anchor(
    file_ref: &str,
    start_line: u32,
    start_column: u32,
    end_line: u32,
    end_column: u32,
    symbol_ref: &str,
) -> FileSpanAnchor {
    FileSpanAnchor {
        file_ref: Some(file_ref.to_owned()),
        start_line: Some(start_line),
        start_column: Some(start_column),
        end_line: Some(end_line),
        end_column: Some(end_column),
        symbol_ref: Some(symbol_ref.to_owned()),
        anchored_to_current_revision: true,
    }
}
