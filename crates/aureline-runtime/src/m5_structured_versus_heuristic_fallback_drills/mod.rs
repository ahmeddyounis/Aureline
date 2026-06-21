//! Canonical per-case truth for the structured-native versus heuristic-fallback proof
//! corpus: one parse-evidence case — a native structured diagnostic, a normalized task
//! event, an imported provider annotation, or a heuristic text parse — exercised through
//! a failure drill (malformed output, stale run, superseded retry, reconnect, lost
//! channel, partial export, imported evidence, or output-channel virtualization) and
//! projected onto the claimed M5 tooling profiles (Problems panel, output channel,
//! terminal runner, debug console, notebook output, pipeline overlay, AI-tool evidence,
//! support export).
//!
//! Where [`crate::m5_execution_evidence_causality_matrix`] froze the *lane* matrix,
//! [`crate::m5_problem_records_source_task_correlation_and_rerun_jump_parity`] froze the
//! **individual Problems row**, [`crate::m5_execution_evidence_projection_overlays`]
//! froze the **projected overlay**, [`crate::m5_task_problem_output_chronology_reuse`]
//! froze the **chronology entry**, and
//! [`crate::m5_output_channel_virtualization_trust_and_freshness`] froze the **output
//! channel**, this module freezes the **parse-evidence drill case**: the proof that the
//! structured-native versus heuristic distinction, the confidence label, and the
//! raw-output backlink survive malformed output, heuristic parsing, stale retries,
//! imported provider evidence, and reconnect-heavy workflows — and that a failure in
//! causal linking or confidence labeling automatically narrows the affected profile
//! claims.
//!
//! Each [`FallbackDrillCase`] speaks the **same** frozen vocabulary as the causality
//! matrix ([`ClaimPosture`], [`OriginClass`], [`ProblemSourceKind`],
//! [`OutputChannelClass`], [`ConfidenceTier`], [`FreshnessState`], [`ReopenTarget`],
//! [`ProofCurrency`], [`VerificationFreshness`]) rather than forking a private
//! parse-evidence model. Reuse the canonical task-event envelopes, diagnostic ids,
//! run/channel refs, and provider objects already landed earlier; this module binds them
//! onto one inspectable, reopenable drill case.
//!
//! Re-derivation rules ([`FallbackDrillCase::narrow`]):
//!
//! * A **heuristic** case (heuristic problem source or a heuristic confidence tier) must
//!   render **visibly distinct from native/structured evidence on every claimed
//!   profile** and keep a **raw-output backlink**. A heuristic case that reads as
//!   structured, drops its backlink, or hides its tier floors to
//!   [`FallbackClaim::Unreconstructable`] and keeps a raw/keyboard fallback rather than a
//!   clean-but-false row.
//! * Every case keeps its **problem-source class, run/step/provider/channel lineage, and
//!   stable channel id** intact and reopenable on demand on every profile; a failure
//!   floors.
//! * The **failure drills** are honest: a reconnect or lost-channel drill that drops the
//!   evidence/backlinks floors; a partial export that cannot be reviewed without the
//!   originating UI floors; an imported/remote/pipeline origin that claims live local
//!   authority floors below its read-only overlay.
//! * Stale, superseded, and missing freshness states and stale/missing verification
//!   proofs **narrow** a first-party case (still reopenable) rather than reading as
//!   fresh; an output-channel virtualization drill that loses stream-first paging,
//!   search, or copy/export narrows.
//! * A profile that renders a claim **wider** than the case's effective claim floors as
//!   an overclaim. Imported/remote/pipeline origins reuse read-only only.
//!   Labs/unadvertised cases make no public claim and are never widened.
//!
//! [`M5FallbackEvidenceDrillSetPacket::validate`] confirms the packet is well-formed and
//! honest: header/identity/redaction/freshness are present, every claimable problem
//! source, every failure drill, and every claimed profile is represented, at least one
//! heuristic case and one auto-narrowing case are present, overlay cases name their
//! provider, real-channel cases name their channel, heuristic cases keep a backlink ref,
//! a floored case keeps a raw fallback, no profile overclaims its case, and no raw
//! boundary material crosses the export. Downstream Problems, output-channel, terminal,
//! debug, notebook, pipeline, AI-tool, and support surfaces ingest this packet rather
//! than inventing a parallel structured-versus-heuristic model.
//!
//! Raw stdout/stderr bytes, command lines, provider log bodies, env bodies, absolute
//! paths, URLs, and secrets never cross this boundary; the packet carries only typed
//! class tokens, opaque ids, booleans, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/tooling/m5-fallback-evidence-drills.schema.json`](../../../../schemas/tooling/m5-fallback-evidence-drills.schema.json).
//! The contract doc is
//! [`docs/tooling/m5-fallback-evidence-drills.md`](../../../../docs/tooling/m5-fallback-evidence-drills.md).
//! The canonical support export is
//! [`artifacts/tooling/m5-fallback-evidence-drills/support_export.json`](../../../../artifacts/tooling/m5-fallback-evidence-drills/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/tooling/m5-fallback-evidence-drills/`](../../../../fixtures/tooling/m5-fallback-evidence-drills/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_execution_evidence_causality_matrix::{
    json_contains_forbidden_boundary_material, label_is_generic, parse_rfc3339_to_epoch_seconds,
    ClaimPosture, ConfidenceTier, FreshnessState, OriginClass, OutputChannelClass,
    ProblemSourceKind, ProofCurrency, ReopenTarget, VerificationFreshness,
};

/// Stable record-kind tag carried by [`M5FallbackEvidenceDrillSetPacket`].
pub const M5_FALLBACK_EVIDENCE_DRILL_RECORD_KIND: &str = "m5_fallback_evidence_drill_set_packet";

/// Schema version for the fallback-evidence drill set.
pub const M5_FALLBACK_EVIDENCE_DRILL_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_FALLBACK_EVIDENCE_DRILL_TAXONOMY_VERSION: u32 = 1;

/// Stable id of the canonical fallback-evidence drill packet.
pub const M5_FALLBACK_EVIDENCE_DRILL_PACKET_ID: &str = "m5-fallback-evidence-drills:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_FALLBACK_EVIDENCE_DRILL_SCHEMA_REF: &str =
    "schemas/tooling/m5-fallback-evidence-drills.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_FALLBACK_EVIDENCE_DRILL_DOC_REF: &str = "docs/tooling/m5-fallback-evidence-drills.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_FALLBACK_EVIDENCE_DRILL_SUPPORT_EXPORT_REF: &str =
    "artifacts/tooling/m5-fallback-evidence-drills/support_export.json";

/// Repo-relative path of the generated certification report.
pub const M5_FALLBACK_EVIDENCE_DRILL_REPORT_REF: &str =
    "artifacts/tooling/m5-fallback-evidence-drills/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_FALLBACK_EVIDENCE_DRILL_FIXTURE_DIR: &str =
    "fixtures/tooling/m5-fallback-evidence-drills";

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
// Frozen drill / profile taxonomies (mirror the boundary schema).
// --------------------------------------------------------------------------- //

/// The failure drill a parse-evidence case is exercised through. This is the *scenario*
/// axis (orthogonal to the problem-source axis): a native baseline, malformed output, a
/// stale run, a superseded retry, a reconnect, a lost channel, a partial export, imported
/// provider evidence, or an output-channel virtualization stress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackDrillKind {
    /// A native structured diagnostic baseline.
    NativeStructured,
    /// A normalized task event projected into a finding.
    NormalizedTaskEvent,
    /// A heuristic parser over raw output text.
    HeuristicTextParse,
    /// Imported provider evidence (CI / SARIF-like annotation).
    ImportedEvidence,
    /// Malformed/garbled output a heuristic parser must degrade on.
    MalformedOutput,
    /// A stale run whose evidence aged past the freshness window.
    StaleRun,
    /// A superseded retry: a newer attempt replaced this run.
    SupersededRetry,
    /// A reconnect-heavy workflow that must not drop evidence.
    Reconnect,
    /// A lost output channel that must not drop evidence.
    LostChannel,
    /// A partial export that must stay reviewable without the originating UI.
    PartialExport,
    /// An output-channel virtualization / search / copy-export / reopen stress.
    ChannelVirtualization,
}

impl FallbackDrillKind {
    /// Every failure drill, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::NativeStructured,
        Self::NormalizedTaskEvent,
        Self::HeuristicTextParse,
        Self::ImportedEvidence,
        Self::MalformedOutput,
        Self::StaleRun,
        Self::SupersededRetry,
        Self::Reconnect,
        Self::LostChannel,
        Self::PartialExport,
        Self::ChannelVirtualization,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeStructured => "native_structured",
            Self::NormalizedTaskEvent => "normalized_task_event",
            Self::HeuristicTextParse => "heuristic_text_parse",
            Self::ImportedEvidence => "imported_evidence",
            Self::MalformedOutput => "malformed_output",
            Self::StaleRun => "stale_run",
            Self::SupersededRetry => "superseded_retry",
            Self::Reconnect => "reconnect",
            Self::LostChannel => "lost_channel",
            Self::PartialExport => "partial_export",
            Self::ChannelVirtualization => "channel_virtualization",
        }
    }
}

/// A claimed M5 tooling profile a parse-evidence case is rendered onto. Acceptance is
/// per-profile: a heuristic case must read distinctly, and a narrowed/floored case must
/// not overclaim, on *every* profile it is rendered on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolingProfile {
    /// The Problems panel.
    ProblemsPanel,
    /// An output channel.
    OutputChannel,
    /// The terminal task runner.
    TerminalRunner,
    /// The debug console.
    DebugConsole,
    /// Notebook cell output.
    NotebookOutput,
    /// A pipeline/provider overlay.
    PipelineOverlay,
    /// An AI-tool evidence packet.
    AiToolEvidence,
    /// A support export.
    SupportExport,
}

impl ToolingProfile {
    /// Every claimed profile, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ProblemsPanel,
        Self::OutputChannel,
        Self::TerminalRunner,
        Self::DebugConsole,
        Self::NotebookOutput,
        Self::PipelineOverlay,
        Self::AiToolEvidence,
        Self::SupportExport,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProblemsPanel => "problems_panel",
            Self::OutputChannel => "output_channel",
            Self::TerminalRunner => "terminal_runner",
            Self::DebugConsole => "debug_console",
            Self::NotebookOutput => "notebook_output",
            Self::PipelineOverlay => "pipeline_overlay",
            Self::AiToolEvidence => "ai_tool_evidence",
            Self::SupportExport => "support_export",
        }
    }

    /// Whether this profile is an exported packet that must stay self-contained.
    pub const fn is_export(self) -> bool {
        matches!(self, Self::AiToolEvidence | Self::SupportExport)
    }
}

// --------------------------------------------------------------------------- //
// Derived fallback-claim ladder and narrowing reasons.
// --------------------------------------------------------------------------- //

/// The effective claim a parse-evidence case renders. A higher rank asserts more
/// authority, so a narrowed or floored case must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackClaim {
    /// Distinctness/lineage/reopen broken or a drill dropped evidence; the case surfaces
    /// a raw-output backlink or keyboard fallback instead of a clean-but-false row.
    #[serde(rename = "fallback_unreconstructable")]
    Unreconstructable,
    /// Remote/pipeline/imported parse evidence; attributable and reopenable but never
    /// claims live local authority.
    #[serde(rename = "fallback_read_only_overlay")]
    ReadOnlyOverlay,
    /// A first-party case held below certified by a stale/labelled gap, but lineage stays
    /// reopenable.
    #[serde(rename = "fallback_narrowed")]
    Narrowed,
    /// Structured-native or a clearly-distinct heuristic fallback, fresh, lineage and
    /// confidence intact, reopenable.
    #[serde(rename = "fallback_certified")]
    Certified,
    /// Labs/unadvertised; makes no public claim and is never widened.
    #[serde(rename = "fallback_labs_not_claimed")]
    LabsNotClaimed,
}

impl FallbackClaim {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unreconstructable => "fallback_unreconstructable",
            Self::ReadOnlyOverlay => "fallback_read_only_overlay",
            Self::Narrowed => "fallback_narrowed",
            Self::Certified => "fallback_certified",
            Self::LabsNotClaimed => "fallback_labs_not_claimed",
        }
    }

    /// Monotonic rank, or `None` for the non-claiming Labs token.
    pub const fn rank(self) -> Option<u8> {
        match self {
            Self::Unreconstructable => Some(0),
            Self::ReadOnlyOverlay => Some(1),
            Self::Narrowed => Some(2),
            Self::Certified => Some(3),
            Self::LabsNotClaimed => None,
        }
    }

    /// Whether rendering `rendered` would overclaim relative to this effective claim. A
    /// profile must never render wider than the case's effective claim; the Labs token
    /// may only render as itself.
    pub fn overclaims_as(self, rendered: FallbackClaim) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            _ => self != rendered,
        }
    }
}

/// A reason a parse-evidence case fails to hold its headline claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackNarrowingReason {
    /// Problem-source class (structured / normalized / heuristic / imported) flattened.
    #[serde(rename = "source_kind_flattened")]
    SourceKindFlattened,
    /// A heuristic parse not visibly distinct from native/structured evidence.
    #[serde(rename = "heuristic_indistinct_from_structured")]
    HeuristicIndistinct,
    /// Run/step/provider/channel lineage flattened, or hidden on some profile.
    #[serde(rename = "run_channel_lineage_flattened")]
    LineageFlattened,
    /// Output channel lost its stable canonical channel id.
    #[serde(rename = "channel_identity_flattened")]
    ChannelIdentityFlattened,
    /// Two profiles disagree about the canonical run/channel/problem id.
    #[serde(rename = "canonical_id_divergence")]
    CanonicalIdDivergence,
    /// Heuristic parse without a raw-output backlink.
    #[serde(rename = "raw_output_backlink_missing")]
    RawBacklinkMissing,
    /// Reopen-to-origin lost; only a keyboard fallback remains.
    #[serde(rename = "reopen_target_lost")]
    ReopenTargetLost,
    /// A reconnect / lost-channel drill dropped the evidence or its backlinks.
    #[serde(rename = "reconnect_drops_evidence")]
    ReconnectDropsEvidence,
    /// A partial export cannot be reviewed without the originating UI state.
    #[serde(rename = "partial_export_incomplete")]
    PartialExportIncomplete,
    /// A profile renders a claim wider than the effective claim.
    #[serde(rename = "surface_overclaims")]
    SurfaceOverclaims,
    /// Imported/remote/pipeline evidence claims live local authority.
    #[serde(rename = "imported_overlay_claims_live")]
    ImportedOverlayClaimsLive,
    /// Evidence missing.
    #[serde(rename = "evidence_missing")]
    EvidenceMissing,
    /// Confidence tier not surfaced.
    #[serde(rename = "confidence_unlabeled")]
    ConfidenceUnlabeled,
    /// Evidence freshness state not surfaced.
    #[serde(rename = "freshness_unlabeled")]
    FreshnessUnlabeled,
    /// Superseded-by-newer-run state not marked.
    #[serde(rename = "superseded_state_not_marked")]
    SupersededNotMarked,
    /// Large log not stream-first / bounded-memory in the virtualization drill.
    #[serde(rename = "virtualization_not_stream_first")]
    VirtualizationNotStreamFirst,
    /// Output channel not searchable in the virtualization drill.
    #[serde(rename = "search_unavailable")]
    SearchUnavailable,
    /// Output channel copy/export unavailable in the virtualization drill.
    #[serde(rename = "copy_export_unavailable")]
    CopyExportUnavailable,
    /// First-party parse evidence stale.
    #[serde(rename = "evidence_stale")]
    StaleEvidence,
    /// Verification proof stale or window elapsed.
    #[serde(rename = "verification_proof_stale")]
    StaleProof,
    /// Verification proof missing.
    #[serde(rename = "verification_proof_missing")]
    MissingProof,
}

impl FallbackNarrowingReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceKindFlattened => "source_kind_flattened",
            Self::HeuristicIndistinct => "heuristic_indistinct_from_structured",
            Self::LineageFlattened => "run_channel_lineage_flattened",
            Self::ChannelIdentityFlattened => "channel_identity_flattened",
            Self::CanonicalIdDivergence => "canonical_id_divergence",
            Self::RawBacklinkMissing => "raw_output_backlink_missing",
            Self::ReopenTargetLost => "reopen_target_lost",
            Self::ReconnectDropsEvidence => "reconnect_drops_evidence",
            Self::PartialExportIncomplete => "partial_export_incomplete",
            Self::SurfaceOverclaims => "surface_overclaims",
            Self::ImportedOverlayClaimsLive => "imported_overlay_claims_live",
            Self::EvidenceMissing => "evidence_missing",
            Self::ConfidenceUnlabeled => "confidence_unlabeled",
            Self::FreshnessUnlabeled => "freshness_unlabeled",
            Self::SupersededNotMarked => "superseded_state_not_marked",
            Self::VirtualizationNotStreamFirst => "virtualization_not_stream_first",
            Self::SearchUnavailable => "search_unavailable",
            Self::CopyExportUnavailable => "copy_export_unavailable",
            Self::StaleEvidence => "evidence_stale",
            Self::StaleProof => "verification_proof_stale",
            Self::MissingProof => "verification_proof_missing",
        }
    }

    /// Whether this reason floors a case to [`FallbackClaim::Unreconstructable`]. Each
    /// floor reason breaks the "stay distinct / stay reopenable / never flatten lineage /
    /// never drop evidence / never masquerade as live" contract outright rather than
    /// merely aging out.
    pub const fn is_floor(self) -> bool {
        matches!(
            self,
            Self::SourceKindFlattened
                | Self::HeuristicIndistinct
                | Self::LineageFlattened
                | Self::ChannelIdentityFlattened
                | Self::CanonicalIdDivergence
                | Self::RawBacklinkMissing
                | Self::ReopenTargetLost
                | Self::ReconnectDropsEvidence
                | Self::PartialExportIncomplete
                | Self::SurfaceOverclaims
                | Self::ImportedOverlayClaimsLive
                | Self::EvidenceMissing
        )
    }

    /// Deterministic ordering index so recorded reason lists are stable across runs.
    /// Floor reasons sort first so the headline trigger is the most severe.
    const fn order_index(self) -> u8 {
        match self {
            Self::SourceKindFlattened => 0,
            Self::HeuristicIndistinct => 1,
            Self::LineageFlattened => 2,
            Self::ChannelIdentityFlattened => 3,
            Self::CanonicalIdDivergence => 4,
            Self::RawBacklinkMissing => 5,
            Self::ReopenTargetLost => 6,
            Self::ReconnectDropsEvidence => 7,
            Self::PartialExportIncomplete => 8,
            Self::SurfaceOverclaims => 9,
            Self::ImportedOverlayClaimsLive => 10,
            Self::EvidenceMissing => 11,
            Self::ConfidenceUnlabeled => 12,
            Self::FreshnessUnlabeled => 13,
            Self::SupersededNotMarked => 14,
            Self::VirtualizationNotStreamFirst => 15,
            Self::SearchUnavailable => 16,
            Self::CopyExportUnavailable => 17,
            Self::StaleEvidence => 18,
            Self::StaleProof => 19,
            Self::MissingProof => 20,
        }
    }
}

/// Sort reasons by their canonical order and drop duplicates.
fn order_reasons(mut reasons: Vec<FallbackNarrowingReason>) -> Vec<FallbackNarrowingReason> {
    reasons.sort_by_key(|reason| reason.order_index());
    reasons.dedup();
    reasons
}

// --------------------------------------------------------------------------- //
// Case sub-objects.
// --------------------------------------------------------------------------- //

/// Stable identifiers binding a parse-evidence case to its canonical objects. Refs carry
/// opaque ids only; absent refs serialize as `null` so the schema's required keys stay
/// present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackLinks {
    /// Execution-context ref (required).
    pub execution_context_ref: String,
    /// Canonical run ref.
    pub run_ref: Option<String>,
    /// Canonical step ref.
    pub step_ref: Option<String>,
    /// Source task ref.
    pub task_ref: Option<String>,
    /// Owning output-channel ref (required for a real channel).
    pub channel_ref: Option<String>,
    /// Correlated Problems-record ref.
    pub problem_ref: Option<String>,
    /// Generated-artifact ref.
    pub artifact_ref: Option<String>,
    /// Provider ref (required for remote/pipeline/imported origins).
    pub provider_ref: Option<String>,
    /// Adapter ref.
    pub adapter_ref: Option<String>,
    /// Raw-output backlink ref (required for a heuristic case).
    pub raw_output_backlink_ref: Option<String>,
}

/// The output-channel virtualization profile exercised by the virtualization drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChannelVirtualization {
    /// A large log stays stream-first (no full materialization into shell memory).
    pub stream_first: bool,
    /// The channel content is searchable.
    pub searchable: bool,
    /// The channel content can be copied/exported.
    pub copy_exportable: bool,
    /// Channel memory stays bounded under a large log.
    pub bounded_memory: bool,
}

/// The integrity invariants every case re-derives rather than trusting a grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackIntegrity {
    /// The problem-source class survives into the rendered case.
    pub preserves_source_kind: bool,
    /// A heuristic case is rendered visibly distinct from native/structured evidence.
    pub heuristic_visibly_distinct_from_structured: bool,
    /// Run/step/provider/channel lineage survives into the rendered case.
    pub preserves_run_channel_lineage: bool,
    /// The output channel keeps its stable canonical channel id.
    pub channel_identity_stable: bool,
    /// The confidence tier is surfaced rather than hidden.
    pub confidence_label_visible: bool,
    /// A heuristic case keeps a raw-output backlink.
    pub raw_output_backlink_present: bool,
    /// The freshness state is surfaced rather than hidden.
    pub freshness_state_labeled: bool,
    /// Superseded state stays marked.
    pub superseded_state_marked: bool,
    /// A reconnect / lost-channel drill keeps the evidence and its backlinks.
    pub reconnect_preserves_evidence: bool,
    /// A partial export stays reviewable/reopenable without the originating UI.
    pub partial_export_self_contained: bool,
    /// Imported/remote/pipeline evidence stays read-only.
    pub imported_evidence_read_only: bool,
}

/// Certification-proof currency for a case (distinct from the evidence's own freshness
/// state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackVerification {
    /// Currency of the certification proof.
    pub proof_currency: ProofCurrency,
    /// Proof ref, or `null` when no proof anchors the case.
    pub proof_ref: Option<String>,
}

/// One claimed tooling profile that renders a parse-evidence case, with the claim it
/// shows, whether it renders the heuristic distinction, whether it can reveal lineage,
/// and the canonical ids it points at.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileBinding {
    /// The claimed tooling profile.
    pub profile: ToolingProfile,
    /// The claim this profile renders.
    pub rendered_claim: FallbackClaim,
    /// Whether this profile renders the heuristic-versus-structured distinction.
    pub fallback_visibly_distinct: bool,
    /// Whether the origin lineage is revealable here.
    pub lineage_visible: bool,
    /// Whether this rendering is read-only.
    pub read_only: bool,
    /// Backlink to the canonical case this profile renders.
    pub source_case_ref: String,
    /// Canonical run ref this profile points at, or `null` when not shown.
    pub bound_run_ref: Option<String>,
    /// Canonical channel ref this profile points at, or `null` when not shown.
    pub bound_channel_ref: Option<String>,
    /// Canonical problem ref this profile points at, or `null` when not shown.
    pub bound_problem_ref: Option<String>,
}

// --------------------------------------------------------------------------- //
// Case + derivation.
// --------------------------------------------------------------------------- //

/// One claimed (or Labs) parse-evidence drill case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackDrillCase {
    /// Stable case id.
    pub case_id: String,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Whether the case is publicly claimed.
    pub claim_posture: ClaimPosture,
    /// How the run/evidence originated.
    pub origin_class: OriginClass,
    /// Problem-source class (the structured-versus-heuristic axis).
    pub problem_source_kind: ProblemSourceKind,
    /// Output-channel / evidence class.
    pub output_channel_class: OutputChannelClass,
    /// The failure drill this case is exercised through.
    pub drill_kind: FallbackDrillKind,
    /// Declared confidence tier.
    pub declared_confidence_tier: ConfidenceTier,
    /// Declared freshness state.
    pub declared_freshness_state: FreshnessState,
    /// Declared reopen target.
    pub declared_reopen_target: ReopenTarget,
    /// Canonical-object link block.
    pub links: FallbackLinks,
    /// Output-channel virtualization block.
    pub virtualization: ChannelVirtualization,
    /// Integrity invariant block.
    pub integrity: FallbackIntegrity,
    /// Certification-proof block.
    pub verification: FallbackVerification,
    /// Profiles that render this case.
    pub profiles: Vec<ProfileBinding>,
}

/// The re-derived fallback decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackDecision {
    /// The headline claim the case is eligible to make.
    pub claimed_fallback_claim: FallbackClaim,
    /// The effective claim after re-derivation; never wider than the evidence.
    pub effective_fallback_claim: FallbackClaim,
    /// Ordered, de-duplicated reasons the case fails to hold its headline.
    pub active_narrowing_reasons: Vec<FallbackNarrowingReason>,
    /// Whether the effective claim ranks below the claimed claim.
    pub narrowed: bool,
}

impl FallbackDecision {
    /// The headline downgrade trigger, when narrowed: the most severe reason.
    pub fn downgrade_trigger(&self) -> Option<FallbackNarrowingReason> {
        if self.narrowed {
            self.active_narrowing_reasons.first().copied()
        } else {
            None
        }
    }

    /// Whether a profile rendering `rendered` for this case would overclaim.
    pub fn surface_overclaims(&self, rendered: FallbackClaim) -> bool {
        self.effective_fallback_claim.overclaims_as(rendered)
    }
}

/// Map (claimed, reasons) onto an effective claim.
fn derive_effective(claimed: FallbackClaim, reasons: &[FallbackNarrowingReason]) -> FallbackClaim {
    if reasons.iter().any(|reason| reason.is_floor()) {
        FallbackClaim::Unreconstructable
    } else if reasons.is_empty() {
        claimed
    } else if matches!(claimed, FallbackClaim::ReadOnlyOverlay) {
        // An overlay is already the minimal honest claim: any other gap means we can no
        // longer certify even the read-only overlay, so it floors.
        FallbackClaim::Unreconstructable
    } else {
        FallbackClaim::Narrowed
    }
}

/// Whether two optional refs that are both present disagree.
fn refs_diverge(a: &Option<String>, b: &Option<String>) -> bool {
    match (opt_str(a), opt_str(b)) {
        (Some(left), Some(right)) => left != right,
        _ => false,
    }
}

impl FallbackDrillCase {
    /// Whether this case is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, ClaimPosture::LabsUnadvertised)
    }

    /// Whether this case is an inherently read-only overlay origin.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin_class.is_overlay()
    }

    /// Whether this case is a heuristic fallback: a heuristic problem source or one of the
    /// explicit heuristic confidence tiers. A heuristic case must read distinctly and keep
    /// a raw-output backlink.
    pub fn is_heuristic(&self) -> bool {
        self.problem_source_kind.is_heuristic() || self.declared_confidence_tier.is_heuristic_tier()
    }

    /// The headline fallback claim this case is eligible to make.
    pub fn claimed_claim(&self) -> FallbackClaim {
        if self.is_labs() {
            FallbackClaim::LabsNotClaimed
        } else if self.is_overlay_origin() {
            FallbackClaim::ReadOnlyOverlay
        } else {
            FallbackClaim::Certified
        }
    }

    /// Whether any profile points at a canonical id that disagrees with the case's own
    /// canonical run/channel/problem id.
    fn has_canonical_id_divergence(&self) -> bool {
        self.profiles.iter().any(|p| {
            refs_diverge(&p.bound_run_ref, &self.links.run_ref)
                || refs_diverge(&p.bound_channel_ref, &self.links.channel_ref)
                || refs_diverge(&p.bound_problem_ref, &self.links.problem_ref)
        })
    }

    /// Reasons that hold independently of how the profiles render — the intrinsic
    /// distinctness/lineage/freshness gaps.
    fn intrinsic_reasons(&self, stale_window: bool) -> Vec<FallbackNarrowingReason> {
        use FallbackNarrowingReason as R;
        let integ = &self.integrity;
        let virt = &self.virtualization;
        let heuristic = self.is_heuristic();
        let overlay = self.is_overlay_origin();
        let mut reasons: Vec<R> = Vec::new();

        // Source-kind distinctness: the structured-versus-heuristic axis.
        if !integ.preserves_source_kind {
            reasons.push(R::SourceKindFlattened);
        }
        // A heuristic case must read distinctly on every profile.
        if heuristic
            && (!integ.heuristic_visibly_distinct_from_structured
                || self.profiles.iter().any(|p| !p.fallback_visibly_distinct))
        {
            reasons.push(R::HeuristicIndistinct);
        }

        // Lineage on-demand visibility across every profile.
        if !integ.preserves_run_channel_lineage || self.profiles.iter().any(|p| !p.lineage_visible)
        {
            reasons.push(R::LineageFlattened);
        }
        if self.output_channel_class.is_real_channel() && !integ.channel_identity_stable {
            reasons.push(R::ChannelIdentityFlattened);
        }
        if self.has_canonical_id_divergence() {
            reasons.push(R::CanonicalIdDivergence);
        }

        // A heuristic case must keep a raw-output backlink and a tier label.
        if heuristic && !integ.raw_output_backlink_present {
            reasons.push(R::RawBacklinkMissing);
        }
        if !integ.confidence_label_visible {
            reasons.push(R::ConfidenceUnlabeled);
        }
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

        // Reconnect / lost-channel drills must not drop the evidence.
        if !integ.reconnect_preserves_evidence {
            reasons.push(R::ReconnectDropsEvidence);
        }
        // A partial export must stay self-contained.
        if !integ.partial_export_self_contained {
            reasons.push(R::PartialExportIncomplete);
        }

        // Output-channel virtualization drill (non-floor: a degraded channel narrows).
        if !virt.stream_first || !virt.bounded_memory {
            reasons.push(R::VirtualizationNotStreamFirst);
        }
        if !virt.searchable {
            reasons.push(R::SearchUnavailable);
        }
        if !virt.copy_exportable {
            reasons.push(R::CopyExportUnavailable);
        }

        // Evidence freshness / superseded / missing.
        match self.declared_freshness_state {
            FreshnessState::Missing => reasons.push(R::EvidenceMissing),
            FreshnessState::SupersededByNewerRun if !integ.superseded_state_marked => {
                reasons.push(R::SupersededNotMarked);
            }
            // An overlay snapshot is expected to be cached/stale; a first-party live
            // surface showing a stale case has aged out of currency.
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

        // Imported/remote/pipeline evidence must stay read-only.
        if overlay && !integ.imported_evidence_read_only {
            reasons.push(R::ImportedOverlayClaimsLive);
        }

        reasons
    }

    /// Every reason this case fails to hold its headline claim, including a profile that
    /// overclaims relative to the intrinsic effective claim.
    pub fn case_reasons(&self, stale_window: bool) -> Vec<FallbackNarrowingReason> {
        let claimed = self.claimed_claim();
        let mut reasons = self.intrinsic_reasons(stale_window);
        let intrinsic = derive_effective(claimed, &reasons);
        if self
            .profiles
            .iter()
            .any(|p| intrinsic.overclaims_as(p.rendered_claim))
        {
            reasons.push(FallbackNarrowingReason::SurfaceOverclaims);
        }
        order_reasons(reasons)
    }

    /// Re-derive the effective fallback claim, reasons, and narrowed flag.
    pub fn narrow(&self, stale_window: bool) -> FallbackDecision {
        let claimed = self.claimed_claim();

        // Labs/unadvertised cases make no public claim, so they never accrue governance
        // narrowing; they hold their non-claiming token.
        if matches!(claimed, FallbackClaim::LabsNotClaimed) {
            return FallbackDecision {
                claimed_fallback_claim: FallbackClaim::LabsNotClaimed,
                effective_fallback_claim: FallbackClaim::LabsNotClaimed,
                active_narrowing_reasons: Vec::new(),
                narrowed: false,
            };
        }

        let reasons = self.case_reasons(stale_window);
        let effective = derive_effective(claimed, &reasons);
        let narrowed = matches!(
            (effective.rank(), claimed.rank()),
            (Some(eff), Some(claim)) if eff < claim
        );

        FallbackDecision {
            claimed_fallback_claim: claimed,
            effective_fallback_claim: effective,
            active_narrowing_reasons: reasons,
            narrowed,
        }
    }

    /// The effective confidence tier: a floored case cannot assert a tier beyond
    /// unmapped/needs-review.
    pub fn effective_confidence(&self, effective: FallbackClaim) -> ConfidenceTier {
        if matches!(effective, FallbackClaim::Unreconstructable) {
            ConfidenceTier::UnmappedRequiresReview
        } else {
            self.declared_confidence_tier
        }
    }

    /// A precise, non-generic reviewer label for a narrowed/floored case.
    pub fn narrowed_label(&self, decision: &FallbackDecision) -> Option<String> {
        if !decision.narrowed {
            return None;
        }
        let trigger = decision
            .downgrade_trigger()
            .map_or("narrowed", FallbackNarrowingReason::as_str)
            .replace('_', " ");
        let reopen = self.declared_reopen_target.as_str().replace('_', " ");
        let claimed = decision.claimed_fallback_claim.as_str();
        let effective = decision.effective_fallback_claim;
        let label = if matches!(effective, FallbackClaim::Unreconstructable) {
            format!(
                "Floored to {} below the {claimed} claim: {trigger}; the {reopen} stays reopenable rather than rendering a clean-but-false row",
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

    /// Whether a non-labs case that floors keeps a reopen fallback rather than hiding
    /// lineage behind a clean-but-false claim.
    fn floored_keeps_fallback(&self, effective: FallbackClaim) -> bool {
        if !matches!(effective, FallbackClaim::Unreconstructable) {
            return true;
        }
        self.declared_reopen_target.is_raw_fallback()
            || self.integrity.raw_output_backlink_present
            || opt_present(&self.links.raw_output_backlink_ref)
    }

    /// Whether any profile renders wider than the case's effective claim.
    fn surface_overclaims(&self, effective: FallbackClaim) -> bool {
        self.profiles
            .iter()
            .any(|p| effective.overclaims_as(p.rendered_claim))
    }

    /// Structural checks that hold independently of the narrowing derivation.
    fn structural_violations(&self, out: &mut Vec<M5FallbackEvidenceDrillViolation>) {
        use M5FallbackEvidenceDrillViolation as V;
        if self.case_id.trim().is_empty()
            || self.label_summary.trim().is_empty()
            || self.links.execution_context_ref.trim().is_empty()
        {
            out.push(V::CaseMissingIdentity);
        }
        if self.is_overlay_origin() && !opt_present(&self.links.provider_ref) {
            out.push(V::OverlayMissingProviderRef);
        }
        if self.output_channel_class.is_real_channel() && !opt_present(&self.links.channel_ref) {
            out.push(V::RealChannelMissingChannelRef);
        }
        if self.is_heuristic() && !opt_present(&self.links.raw_output_backlink_ref) {
            out.push(V::HeuristicMissingBacklinkRef);
        }
        if self.profiles.is_empty() {
            out.push(V::CaseMissingProfile);
        }
        for profile in &self.profiles {
            if profile.source_case_ref.trim().is_empty() {
                out.push(V::ProfileMissingSourceRef);
            }
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

/// Constructor input for an [`M5FallbackEvidenceDrillSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FallbackEvidenceDrillSetInput {
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
    /// Per-case rows.
    pub cases: Vec<FallbackDrillCase>,
}

/// Export-safe M5 structured-versus-heuristic fallback drill set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FallbackEvidenceDrillSetPacket {
    /// Record kind; must equal [`M5_FALLBACK_EVIDENCE_DRILL_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FALLBACK_EVIDENCE_DRILL_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal [`M5_FALLBACK_EVIDENCE_DRILL_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-case rows.
    pub cases: Vec<FallbackDrillCase>,
}

/// The distribution of effective fallback claims across a set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackClaimDistribution {
    /// Cases effective at [`FallbackClaim::Certified`].
    pub certified: usize,
    /// Cases effective at [`FallbackClaim::Narrowed`].
    pub narrowed: usize,
    /// Cases effective at [`FallbackClaim::ReadOnlyOverlay`].
    pub overlay: usize,
    /// Cases effective at [`FallbackClaim::Unreconstructable`].
    pub unreconstructable: usize,
    /// Cases effective at [`FallbackClaim::LabsNotClaimed`].
    pub labs: usize,
}

impl M5FallbackEvidenceDrillSetPacket {
    /// Builds a fallback-evidence drill packet, sealing the record-kind, schema, and
    /// taxonomy version constants.
    pub fn new(input: M5FallbackEvidenceDrillSetInput) -> Self {
        Self {
            record_kind: M5_FALLBACK_EVIDENCE_DRILL_RECORD_KIND.to_owned(),
            schema_version: M5_FALLBACK_EVIDENCE_DRILL_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_FALLBACK_EVIDENCE_DRILL_TAXONOMY_VERSION,
            redaction_class_token: input.redaction_class_token,
            verification_freshness: input.verification_freshness,
            cases: input.cases,
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

    /// Re-derive the decision for every case, paired with its id.
    pub fn decisions(&self) -> Vec<(String, FallbackDecision)> {
        let stale_window = self.stale_window();
        self.cases
            .iter()
            .map(|c| (c.case_id.clone(), c.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective fallback claims.
    pub fn claim_distribution(&self) -> FallbackClaimDistribution {
        let stale_window = self.stale_window();
        let mut dist = FallbackClaimDistribution {
            certified: 0,
            narrowed: 0,
            overlay: 0,
            unreconstructable: 0,
            labs: 0,
        };
        for c in &self.cases {
            match c.narrow(stale_window).effective_fallback_claim {
                FallbackClaim::Certified => dist.certified += 1,
                FallbackClaim::Narrowed => dist.narrowed += 1,
                FallbackClaim::ReadOnlyOverlay => dist.overlay += 1,
                FallbackClaim::Unreconstructable => dist.unreconstructable += 1,
                FallbackClaim::LabsNotClaimed => dist.labs += 1,
            }
        }
        dist
    }

    /// Count of cases whose effective claim ranks below their claimed claim.
    pub fn narrowed_case_count(&self) -> usize {
        let stale_window = self.stale_window();
        self.cases
            .iter()
            .filter(|c| c.narrow(stale_window).narrowed)
            .count()
    }

    /// Problem-source kinds represented by some case.
    pub fn represented_source_kinds(&self) -> BTreeSet<ProblemSourceKind> {
        self.cases.iter().map(|c| c.problem_source_kind).collect()
    }

    /// Failure drills represented by some case.
    pub fn represented_drills(&self) -> BTreeSet<FallbackDrillKind> {
        self.cases.iter().map(|c| c.drill_kind).collect()
    }

    /// Tooling profiles represented by some case.
    pub fn represented_profiles(&self) -> BTreeSet<ToolingProfile> {
        self.cases
            .iter()
            .flat_map(|c| c.profiles.iter().map(|p| p.profile))
            .collect()
    }

    /// Validate the fallback-evidence drill invariants.
    pub fn validate(&self) -> Vec<M5FallbackEvidenceDrillViolation> {
        use M5FallbackEvidenceDrillViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_FALLBACK_EVIDENCE_DRILL_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_FALLBACK_EVIDENCE_DRILL_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_FALLBACK_EVIDENCE_DRILL_TAXONOMY_VERSION {
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
        if self.cases.is_empty() {
            violations.push(V::EmptyCases);
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for c in &self.cases {
            if !seen.insert(c.case_id.as_str()) {
                violations.push(V::DuplicateCaseId);
            }
        }

        // Every claimable problem-source class must be exercised.
        let sources = self.represented_source_kinds();
        let required_sources = [
            ProblemSourceKind::StructuredLanguageDiagnostic,
            ProblemSourceKind::NormalizedTaskEvent,
            ProblemSourceKind::HeuristicOutputParse,
            ProblemSourceKind::ImportedProviderAnnotation,
        ];
        if required_sources.iter().any(|s| !sources.contains(s)) {
            violations.push(V::ProblemSourceMissing);
        }
        let drills = self.represented_drills();
        if FallbackDrillKind::ALL.iter().any(|d| !drills.contains(d)) {
            violations.push(V::DrillKindMissing);
        }
        let profiles = self.represented_profiles();
        if ToolingProfile::ALL.iter().any(|p| !profiles.contains(p)) {
            violations.push(V::ProfileMissing);
        }
        if !self.cases.iter().any(FallbackDrillCase::is_heuristic) {
            violations.push(V::HeuristicCaseMissing);
        }

        let stale_window = self.stale_window();
        let mut demonstrates_narrowing = false;
        for c in &self.cases {
            c.structural_violations(&mut violations);
            let decision = c.narrow(stale_window);
            if decision.narrowed {
                demonstrates_narrowing = true;
                if decision.downgrade_trigger().is_none()
                    || c.narrowed_label(&decision)
                        .map_or(true, |label| label_is_generic(&label))
                {
                    violations.push(V::NarrowedCaseMissingLabelOrTrigger);
                }
            }
            if !c.floored_keeps_fallback(decision.effective_fallback_claim) {
                violations.push(V::FlooredCaseLosesFallback);
            }
            if c.surface_overclaims(decision.effective_fallback_claim) {
                violations.push(V::ProfileOverclaims);
            }
        }
        if !demonstrates_narrowing {
            violations.push(V::DowngradedCaseMissing);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("fallback-evidence drill packet serializes"),
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
        serde_json::to_string_pretty(self).expect("fallback-evidence drill packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.claim_distribution();
        let mut out = String::new();
        out.push_str("# M5 Structured-vs-Heuristic Fallback Evidence Drills\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Cases: {}\n", self.cases.len()));
        out.push_str(&format!(
            "- Effective: {} certified, {} narrowed, {} read-only overlay, {} unreconstructable, {} labs\n\n",
            dist.certified, dist.narrowed, dist.overlay, dist.unreconstructable, dist.labs
        ));

        out.push_str(
            "| Case | Source | Drill | Channel | Origin | Claimed | Effective | Confidence |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- | --- |\n");
        for c in &self.cases {
            let decision = c.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
                c.case_id,
                c.problem_source_kind.as_str(),
                c.drill_kind.as_str(),
                c.output_channel_class.as_str(),
                c.origin_class.as_str(),
                decision.claimed_fallback_claim.as_str(),
                decision.effective_fallback_claim.as_str(),
                c.effective_confidence(decision.effective_fallback_claim)
                    .as_str(),
            ));
        }

        out.push('\n');
        for c in &self.cases {
            let decision = c.narrow(stale_window);
            if let Some(label) = c.narrowed_label(&decision) {
                out.push_str(&format!("- Narrowed: `{}` — {}\n", c.case_id, label));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or validate.
#[derive(Debug)]
pub enum M5FallbackEvidenceDrillArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5FallbackEvidenceDrillViolation>),
}

impl fmt::Display for M5FallbackEvidenceDrillArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(
                    f,
                    "fallback-evidence drill support export parse error: {err}"
                )
            }
            Self::Validation(violations) => write!(
                f,
                "fallback-evidence drill support export failed validation: {violations:?}"
            ),
        }
    }
}

impl Error for M5FallbackEvidenceDrillArtifactError {}

/// Invariant violations reported by [`M5FallbackEvidenceDrillSetPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FallbackEvidenceDrillViolation {
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
    /// The packet carries no cases.
    EmptyCases,
    /// Two cases share an id.
    DuplicateCaseId,
    /// A claimable problem-source class is unexercised.
    ProblemSourceMissing,
    /// A failure drill is unexercised.
    DrillKindMissing,
    /// A claimed tooling profile is unrepresented.
    ProfileMissing,
    /// No case exercises a heuristic fallback.
    HeuristicCaseMissing,
    /// A case is missing its id, label, or execution-context ref.
    CaseMissingIdentity,
    /// An overlay-origin case does not name its provider.
    OverlayMissingProviderRef,
    /// A real-channel case does not name its channel.
    RealChannelMissingChannelRef,
    /// A heuristic case does not name a raw-output backlink ref.
    HeuristicMissingBacklinkRef,
    /// A case is rendered on no profile.
    CaseMissingProfile,
    /// A profile is missing its source-case backlink.
    ProfileMissingSourceRef,
    /// A floored case lost its raw-output / keyboard reopen fallback.
    FlooredCaseLosesFallback,
    /// A narrowed case is missing its precise label or trigger.
    NarrowedCaseMissingLabelOrTrigger,
    /// A profile renders wider than the case's effective claim.
    ProfileOverclaims,
    /// No case demonstrates the auto-narrowing rule.
    DowngradedCaseMissing,
    /// Export-safe JSON carried forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl M5FallbackEvidenceDrillViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::WrongTaxonomyVersion => "wrong_taxonomy_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidRedactionClass => "invalid_redaction_class",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::EmptyCases => "empty_cases",
            Self::DuplicateCaseId => "duplicate_case_id",
            Self::ProblemSourceMissing => "problem_source_missing",
            Self::DrillKindMissing => "drill_kind_missing",
            Self::ProfileMissing => "profile_missing",
            Self::HeuristicCaseMissing => "heuristic_case_missing",
            Self::CaseMissingIdentity => "case_missing_identity",
            Self::OverlayMissingProviderRef => "overlay_missing_provider_ref",
            Self::RealChannelMissingChannelRef => "real_channel_missing_channel_ref",
            Self::HeuristicMissingBacklinkRef => "heuristic_missing_backlink_ref",
            Self::CaseMissingProfile => "case_missing_profile",
            Self::ProfileMissingSourceRef => "profile_missing_source_ref",
            Self::FlooredCaseLosesFallback => "floored_case_loses_fallback",
            Self::NarrowedCaseMissingLabelOrTrigger => "narrowed_case_missing_label_or_trigger",
            Self::ProfileOverclaims => "profile_overclaims",
            Self::DowngradedCaseMissing => "downgraded_case_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Loads and validates the checked-in canonical support export.
///
/// This is the canonical entry point downstream Problems, output-channel, terminal,
/// debug, notebook, pipeline, AI-tool, and support surfaces use to ingest the frozen
/// structured-versus-heuristic drill set instead of cloning surface-local fallback logic.
///
/// # Errors
///
/// Returns [`M5FallbackEvidenceDrillArtifactError`] when the artifact cannot be parsed or
/// fails validation.
pub fn current_m5_fallback_evidence_drill_set(
) -> Result<M5FallbackEvidenceDrillSetPacket, M5FallbackEvidenceDrillArtifactError> {
    let packet: M5FallbackEvidenceDrillSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/tooling/m5-fallback-evidence-drills/support_export.json"
    )))
    .map_err(M5FallbackEvidenceDrillArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5FallbackEvidenceDrillArtifactError::Validation(violations))
    }
}

// --------------------------------------------------------------------------- //
// Canonical seed.
// --------------------------------------------------------------------------- //

/// The canonical seeded fallback-evidence drill set: the in-crate source of truth the
/// checked-in support export and report are regenerated from.
pub fn seeded_fallback_evidence_drill_set() -> M5FallbackEvidenceDrillSetPacket {
    M5FallbackEvidenceDrillSetPacket::new(M5FallbackEvidenceDrillSetInput {
        packet_id: M5_FALLBACK_EVIDENCE_DRILL_PACKET_ID.to_owned(),
        label: "M5 structured-native versus heuristic-fallback proof corpus across local, remote, notebook, extension, AI-tool, and provider channels".to_owned(),
        as_of: SEED_AS_OF.to_owned(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        verification_freshness: VerificationFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_AS_OF.to_owned(),
            auto_downgrade_on_stale: true,
        },
        cases: seed_cases(),
    })
}

/// A clean first-party integrity block.
fn clean_integrity() -> FallbackIntegrity {
    FallbackIntegrity {
        preserves_source_kind: true,
        heuristic_visibly_distinct_from_structured: true,
        preserves_run_channel_lineage: true,
        channel_identity_stable: true,
        confidence_label_visible: true,
        raw_output_backlink_present: true,
        freshness_state_labeled: true,
        superseded_state_marked: true,
        reconnect_preserves_evidence: true,
        partial_export_self_contained: true,
        imported_evidence_read_only: true,
    }
}

/// A clean output-channel virtualization block.
fn clean_virtualization() -> ChannelVirtualization {
    ChannelVirtualization {
        stream_first: true,
        searchable: true,
        copy_exportable: true,
        bounded_memory: true,
    }
}

/// A verified-current proof block.
fn verified(proof_ref: &str) -> FallbackVerification {
    FallbackVerification {
        proof_currency: ProofCurrency::VerifiedCurrent,
        proof_ref: Some(proof_ref.to_owned()),
    }
}

/// An imported-current proof block for an overlay.
fn imported(proof_ref: &str) -> FallbackVerification {
    FallbackVerification {
        proof_currency: ProofCurrency::ImportedCurrent,
        proof_ref: Some(proof_ref.to_owned()),
    }
}

/// Profiles that render `claim` cleanly on the named profiles, each echoing the case's
/// canonical run/channel/problem ids.
#[allow(clippy::too_many_arguments)]
fn profiles(
    source_ref: &str,
    claim: FallbackClaim,
    list: &[ToolingProfile],
    read_only: bool,
    run_ref: Option<&str>,
    channel_ref: Option<&str>,
    problem_ref: Option<&str>,
) -> Vec<ProfileBinding> {
    list.iter()
        .map(|&profile| ProfileBinding {
            profile,
            rendered_claim: claim,
            fallback_visibly_distinct: true,
            lineage_visible: true,
            read_only,
            source_case_ref: source_ref.to_owned(),
            bound_run_ref: run_ref.map(str::to_owned),
            bound_channel_ref: channel_ref.map(str::to_owned),
            bound_problem_ref: problem_ref.map(str::to_owned),
        })
        .collect()
}

fn seed_cases() -> Vec<FallbackDrillCase> {
    use FallbackClaim::{Certified, LabsNotClaimed, Narrowed, ReadOnlyOverlay};
    use FallbackDrillKind as D;
    use ToolingProfile as P;

    vec![
        // 1. Native structured language diagnostic — Problems / output / debug — certified.
        FallbackDrillCase {
            case_id: "fallback:native-structured-language-problems:0001".to_owned(),
            label_summary:
                "A native structured language-server diagnostic renders identically in the Problems panel, output channel, and debug console with full lineage and a structured confidence tier."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTest,
            problem_source_kind: ProblemSourceKind::StructuredLanguageDiagnostic,
            output_channel_class: OutputChannelClass::TaskTestDebugOutput,
            drill_kind: D::NativeStructured,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::EditorAnchor,
            links: FallbackLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.test.diag.0001".to_owned()),
                step_ref: Some("step.local.test.diag.0001".to_owned()),
                task_ref: Some("task.local.test.0001".to_owned()),
                channel_ref: Some("channel.local.test.0001".to_owned()),
                problem_ref: Some("problem.local.lang.0001".to_owned()),
                artifact_ref: None,
                provider_ref: None,
                adapter_ref: Some("adapter.local.language.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.test.diag.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: verified("proof.local.native.0001"),
            profiles: profiles(
                "fallback:native-structured-language-problems:0001",
                Certified,
                &[P::ProblemsPanel, P::OutputChannel, P::DebugConsole],
                false,
                Some("run.local.test.diag.0001"),
                Some("channel.local.test.0001"),
                Some("problem.local.lang.0001"),
            ),
        },
        // 2. Normalized task event — terminal / output — certified.
        FallbackDrillCase {
            case_id: "fallback:normalized-task-event-terminal:0001".to_owned(),
            label_summary:
                "A normalized task event is projected into a finding shown in the terminal runner and output channel with structured confidence."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            problem_source_kind: ProblemSourceKind::NormalizedTaskEvent,
            output_channel_class: OutputChannelClass::TaskTestDebugOutput,
            drill_kind: D::NormalizedTaskEvent,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OwningRun,
            links: FallbackLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.task.build.0001".to_owned()),
                step_ref: Some("step.local.task.build.0001".to_owned()),
                task_ref: Some("task.local.build.0001".to_owned()),
                channel_ref: Some("channel.local.task.0001".to_owned()),
                problem_ref: Some("problem.local.task.0001".to_owned()),
                artifact_ref: None,
                provider_ref: None,
                adapter_ref: Some("adapter.local.task.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.task.build.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: verified("proof.local.normalized.0001"),
            profiles: profiles(
                "fallback:normalized-task-event-terminal:0001",
                Certified,
                &[P::TerminalRunner, P::OutputChannel],
                false,
                Some("run.local.task.build.0001"),
                Some("channel.local.task.0001"),
                Some("problem.local.task.0001"),
            ),
        },
        // 3. Heuristic text parse — Problems / terminal — certified, visibly distinct.
        FallbackDrillCase {
            case_id: "fallback:heuristic-parse-terminal:0001".to_owned(),
            label_summary:
                "A heuristic output-matcher parse renders visibly distinct from structured evidence in the Problems panel and terminal runner, keeping a high heuristic tier and a raw-output backlink."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            problem_source_kind: ProblemSourceKind::HeuristicOutputParse,
            output_channel_class: OutputChannelClass::TaskTestDebugOutput,
            drill_kind: D::HeuristicTextParse,
            declared_confidence_tier: ConfidenceTier::HeuristicHigh,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::RawOutputBacklink,
            links: FallbackLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.task.lint.0001".to_owned()),
                step_ref: Some("step.local.task.lint.0001".to_owned()),
                task_ref: Some("task.local.lint.0001".to_owned()),
                channel_ref: Some("channel.local.task.0002".to_owned()),
                problem_ref: Some("problem.local.heuristic.0001".to_owned()),
                artifact_ref: None,
                provider_ref: None,
                adapter_ref: Some("adapter.local.matcher.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.task.lint.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: verified("proof.local.heuristic.0001"),
            profiles: profiles(
                "fallback:heuristic-parse-terminal:0001",
                Certified,
                &[P::ProblemsPanel, P::TerminalRunner],
                false,
                Some("run.local.task.lint.0001"),
                Some("channel.local.task.0002"),
                Some("problem.local.heuristic.0001"),
            ),
        },
        // 4. Malformed output — heuristic degrades but stays distinct — certified.
        FallbackDrillCase {
            case_id: "fallback:malformed-output-heuristic:0001".to_owned(),
            label_summary:
                "A heuristic parser degrades on malformed output to a medium tier, stays visibly distinct from structured evidence, and keeps a raw-output backlink in the Problems panel and output channel."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            problem_source_kind: ProblemSourceKind::HeuristicOutputParse,
            output_channel_class: OutputChannelClass::TaskTestDebugOutput,
            drill_kind: D::MalformedOutput,
            declared_confidence_tier: ConfidenceTier::HeuristicMedium,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::RawOutputBacklink,
            links: FallbackLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.task.garbled.0001".to_owned()),
                step_ref: Some("step.local.task.garbled.0001".to_owned()),
                task_ref: Some("task.local.garbled.0001".to_owned()),
                channel_ref: Some("channel.local.task.0003".to_owned()),
                problem_ref: Some("problem.local.heuristic.0002".to_owned()),
                artifact_ref: None,
                provider_ref: None,
                adapter_ref: Some("adapter.local.matcher.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.task.garbled.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: verified("proof.local.malformed.0001"),
            profiles: profiles(
                "fallback:malformed-output-heuristic:0001",
                Certified,
                &[P::ProblemsPanel, P::OutputChannel],
                false,
                Some("run.local.task.garbled.0001"),
                Some("channel.local.task.0003"),
                Some("problem.local.heuristic.0002"),
            ),
        },
        // 5. Imported provider annotation — pipeline / support / AI — read-only overlay.
        FallbackDrillCase {
            case_id: "fallback:imported-provider-annotation:0001".to_owned(),
            label_summary:
                "An imported provider annotation reused in a pipeline overlay, a support export, and an AI-tool packet is attributable and reopenable but never claims live local authority."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::ImportedProviderEvidence,
            problem_source_kind: ProblemSourceKind::ImportedProviderAnnotation,
            output_channel_class: OutputChannelClass::RemoteProviderImportedOutput,
            drill_kind: D::ImportedEvidence,
            declared_confidence_tier: ConfidenceTier::ProviderMapped,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::ProviderRunPage,
            links: FallbackLinks {
                execution_context_ref: "exec-context.remote.provider.primary".to_owned(),
                run_ref: Some("run.provider.imported.0001".to_owned()),
                step_ref: Some("step.provider.imported.0001".to_owned()),
                task_ref: Some("task.provider.imported.0001".to_owned()),
                channel_ref: Some("channel.provider.imported.0001".to_owned()),
                problem_ref: Some("problem.provider.imported.0001".to_owned()),
                artifact_ref: Some("artifact.provider.imported.0001".to_owned()),
                provider_ref: Some("provider.ci.imported.0001".to_owned()),
                adapter_ref: Some("adapter.provider.imported.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.provider.imported.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: imported("proof.provider.imported.0001"),
            profiles: profiles(
                "fallback:imported-provider-annotation:0001",
                ReadOnlyOverlay,
                &[P::PipelineOverlay, P::SupportExport, P::AiToolEvidence],
                true,
                Some("run.provider.imported.0001"),
                Some("channel.provider.imported.0001"),
                Some("problem.provider.imported.0001"),
            ),
        },
        // 6. Pipeline reconnect — overlay holds through reconnect.
        FallbackDrillCase {
            case_id: "fallback:pipeline-reconnect:0001".to_owned(),
            label_summary:
                "A pipeline-provider run reconnects after a dropped connection; the overlay keeps its evidence and backlinks and stays a read-only overlay in the pipeline overlay and output channel."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::PipelineProviderRun,
            problem_source_kind: ProblemSourceKind::NormalizedTaskEvent,
            output_channel_class: OutputChannelClass::RemoteProviderImportedOutput,
            drill_kind: D::Reconnect,
            declared_confidence_tier: ConfidenceTier::ProviderMapped,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::ProviderRunPage,
            links: FallbackLinks {
                execution_context_ref: "exec-context.remote.pipeline.primary".to_owned(),
                run_ref: Some("run.pipeline.reconnect.0001".to_owned()),
                step_ref: Some("step.pipeline.reconnect.0001".to_owned()),
                task_ref: Some("task.pipeline.reconnect.0001".to_owned()),
                channel_ref: Some("channel.pipeline.reconnect.0001".to_owned()),
                problem_ref: Some("problem.pipeline.reconnect.0001".to_owned()),
                artifact_ref: None,
                provider_ref: Some("provider.pipeline.ci.0001".to_owned()),
                adapter_ref: Some("adapter.pipeline.ci.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.pipeline.reconnect.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: imported("proof.pipeline.reconnect.0001"),
            profiles: profiles(
                "fallback:pipeline-reconnect:0001",
                ReadOnlyOverlay,
                &[P::PipelineOverlay, P::OutputChannel],
                true,
                Some("run.pipeline.reconnect.0001"),
                Some("channel.pipeline.reconnect.0001"),
                Some("problem.pipeline.reconnect.0001"),
            ),
        },
        // 7. Notebook heuristic parse, stale run — narrowed but reopenable.
        FallbackDrillCase {
            case_id: "fallback:notebook-heuristic-stale:0001".to_owned(),
            label_summary:
                "A notebook heuristic parse whose run aged past the freshness window is held below certified while staying visibly distinct and reopenable in the notebook output and output channel."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::NotebookRun,
            problem_source_kind: ProblemSourceKind::HeuristicOutputParse,
            output_channel_class: OutputChannelClass::TaskTestDebugOutput,
            drill_kind: D::StaleRun,
            declared_confidence_tier: ConfidenceTier::HeuristicHigh,
            declared_freshness_state: FreshnessState::StaleExpired,
            declared_reopen_target: ReopenTarget::RawOutputBacklink,
            links: FallbackLinks {
                execution_context_ref: "exec-context.local.notebook.primary".to_owned(),
                run_ref: Some("run.local.notebook.parse.0001".to_owned()),
                step_ref: Some("step.local.notebook.parse.0001".to_owned()),
                task_ref: Some("task.local.notebook.0001".to_owned()),
                channel_ref: Some("channel.local.notebook.0001".to_owned()),
                problem_ref: Some("problem.local.notebook.0001".to_owned()),
                artifact_ref: None,
                provider_ref: None,
                adapter_ref: Some("adapter.local.notebook.matcher.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.notebook.parse.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: verified("proof.local.notebook.0001"),
            profiles: profiles(
                "fallback:notebook-heuristic-stale:0001",
                Narrowed,
                &[P::NotebookOutput, P::OutputChannel],
                false,
                Some("run.local.notebook.parse.0001"),
                Some("channel.local.notebook.0001"),
                Some("problem.local.notebook.0001"),
            ),
        },
        // 8. Superseded retry, marked — stays certified because the state is visible.
        FallbackDrillCase {
            case_id: "fallback:superseded-retry-marked:0001".to_owned(),
            label_summary:
                "A superseded retry whose superseded state is clearly marked stays certified in the Problems panel and terminal runner rather than reading as a fresh, fully actionable row."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTest,
            problem_source_kind: ProblemSourceKind::NormalizedTaskEvent,
            output_channel_class: OutputChannelClass::TaskTestDebugOutput,
            drill_kind: D::SupersededRetry,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::SupersededByNewerRun,
            declared_reopen_target: ReopenTarget::OwningRun,
            links: FallbackLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.test.superseded.0001".to_owned()),
                step_ref: Some("step.local.test.superseded.0001".to_owned()),
                task_ref: Some("task.local.test.0002".to_owned()),
                channel_ref: Some("channel.local.test.0002".to_owned()),
                problem_ref: Some("problem.local.test.0002".to_owned()),
                artifact_ref: None,
                provider_ref: None,
                adapter_ref: Some("adapter.local.test.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.test.superseded.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: verified("proof.local.superseded.0001"),
            profiles: profiles(
                "fallback:superseded-retry-marked:0001",
                Certified,
                &[P::ProblemsPanel, P::TerminalRunner],
                false,
                Some("run.local.test.superseded.0001"),
                Some("channel.local.test.0002"),
                Some("problem.local.test.0002"),
            ),
        },
        // 9. Extension / AI-tool heuristic — certified, visibly distinct.
        FallbackDrillCase {
            case_id: "fallback:extension-ai-tool-heuristic:0001".to_owned(),
            label_summary:
                "An extension-owned AI-tool heuristic parse renders visibly distinct from structured evidence in the AI-tool evidence packet and output channel, keeping a medium tier and a backlink."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::ExtensionOwnedRun,
            problem_source_kind: ProblemSourceKind::HeuristicOutputParse,
            output_channel_class: OutputChannelClass::ExtensionAiToolOutput,
            drill_kind: D::HeuristicTextParse,
            declared_confidence_tier: ConfidenceTier::HeuristicMedium,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::RawOutputBacklink,
            links: FallbackLinks {
                execution_context_ref: "exec-context.local.extension.primary".to_owned(),
                run_ref: Some("run.local.extension.ai.0001".to_owned()),
                step_ref: Some("step.local.extension.ai.0001".to_owned()),
                task_ref: Some("task.local.extension.0001".to_owned()),
                channel_ref: Some("channel.local.extension.0001".to_owned()),
                problem_ref: Some("problem.local.extension.0001".to_owned()),
                artifact_ref: None,
                provider_ref: None,
                adapter_ref: Some("adapter.local.extension.matcher.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.extension.ai.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: verified("proof.local.extension.0001"),
            profiles: profiles(
                "fallback:extension-ai-tool-heuristic:0001",
                Certified,
                &[P::AiToolEvidence, P::OutputChannel],
                false,
                Some("run.local.extension.ai.0001"),
                Some("channel.local.extension.0001"),
                Some("problem.local.extension.0001"),
            ),
        },
        // 10. Output-channel virtualization stress — certified.
        FallbackDrillCase {
            case_id: "fallback:channel-virtualization-large-log:0001".to_owned(),
            label_summary:
                "A large structured output log stays stream-first, searchable, copy/exportable, and reopenable under a virtualization stress in the output channel and terminal runner without full materialization."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            problem_source_kind: ProblemSourceKind::StructuredLanguageDiagnostic,
            output_channel_class: OutputChannelClass::TaskTestDebugOutput,
            drill_kind: D::ChannelVirtualization,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::Live,
            declared_reopen_target: ReopenTarget::OutputChannel,
            links: FallbackLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.task.largelog.0001".to_owned()),
                step_ref: Some("step.local.task.largelog.0001".to_owned()),
                task_ref: Some("task.local.largelog.0001".to_owned()),
                channel_ref: Some("channel.local.task.0004".to_owned()),
                problem_ref: None,
                artifact_ref: Some("artifact.local.largelog.0001".to_owned()),
                provider_ref: None,
                adapter_ref: Some("adapter.local.task.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.task.largelog.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: verified("proof.local.virtualization.0001"),
            profiles: profiles(
                "fallback:channel-virtualization-large-log:0001",
                Certified,
                &[P::OutputChannel, P::TerminalRunner],
                false,
                Some("run.local.task.largelog.0001"),
                Some("channel.local.task.0004"),
                None,
            ),
        },
        // 11. Partial export — support bundle stays self-contained — certified.
        FallbackDrillCase {
            case_id: "fallback:partial-export-support-bundle:0001".to_owned(),
            label_summary:
                "A partially exported evidence bundle keeps the minimum reopen identity and stays reviewable without the originating UI in the support export and AI-tool evidence packet."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTest,
            problem_source_kind: ProblemSourceKind::StructuredLanguageDiagnostic,
            output_channel_class: OutputChannelClass::EvidenceBundle,
            drill_kind: D::PartialExport,
            declared_confidence_tier: ConfidenceTier::StructuredFull,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::GeneratedArtifact,
            links: FallbackLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.test.export.0001".to_owned()),
                step_ref: Some("step.local.test.export.0001".to_owned()),
                task_ref: Some("task.local.test.0003".to_owned()),
                channel_ref: Some("channel.local.evidence.0001".to_owned()),
                problem_ref: Some("problem.local.test.0003".to_owned()),
                artifact_ref: Some("artifact.local.evidence.bundle.0001".to_owned()),
                provider_ref: None,
                adapter_ref: Some("adapter.local.test.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.test.export.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: verified("proof.local.export.0001"),
            profiles: profiles(
                "fallback:partial-export-support-bundle:0001",
                Certified,
                &[P::SupportExport, P::AiToolEvidence],
                false,
                Some("run.local.test.export.0001"),
                Some("channel.local.evidence.0001"),
                Some("problem.local.test.0003"),
            ),
        },
        // 12. Heuristic verdict, stale verification proof — narrowed but reopenable.
        FallbackDrillCase {
            case_id: "fallback:heuristic-stale-proof:0001".to_owned(),
            label_summary:
                "A high-confidence heuristic verdict whose verification proof has gone stale is held below certified while staying visibly distinct and reopenable in the output channel and AI-tool evidence packet."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::LocalTask,
            problem_source_kind: ProblemSourceKind::HeuristicOutputParse,
            output_channel_class: OutputChannelClass::TaskTestDebugOutput,
            drill_kind: D::HeuristicTextParse,
            declared_confidence_tier: ConfidenceTier::HeuristicHigh,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::RawOutputBacklink,
            links: FallbackLinks {
                execution_context_ref: "exec-context.local.workspace.primary".to_owned(),
                run_ref: Some("run.local.task.perf.0001".to_owned()),
                step_ref: Some("step.local.task.perf.0001".to_owned()),
                task_ref: Some("task.local.perf.0001".to_owned()),
                channel_ref: Some("channel.local.task.0005".to_owned()),
                problem_ref: Some("problem.local.heuristic.0003".to_owned()),
                artifact_ref: Some("artifact.local.perf.trace.0001".to_owned()),
                provider_ref: None,
                adapter_ref: Some("adapter.local.matcher.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.local.task.perf.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: FallbackVerification {
                proof_currency: ProofCurrency::StaleExpired,
                proof_ref: Some("proof.local.perf.0001".to_owned()),
            },
            profiles: profiles(
                "fallback:heuristic-stale-proof:0001",
                Narrowed,
                &[P::OutputChannel, P::AiToolEvidence],
                false,
                Some("run.local.task.perf.0001"),
                Some("channel.local.task.0005"),
                Some("problem.local.heuristic.0003"),
            ),
        },
        // 13. Lost remote channel — overlay holds through a lost channel.
        FallbackDrillCase {
            case_id: "fallback:lost-channel-remote:0001".to_owned(),
            label_summary:
                "A remote-linked run whose output channel is lost mid-stream keeps its evidence and backlinks and stays a read-only overlay in the output channel and pipeline overlay."
                    .to_owned(),
            claim_posture: ClaimPosture::ClaimedStable,
            origin_class: OriginClass::RemoteLinkedRun,
            problem_source_kind: ProblemSourceKind::NormalizedTaskEvent,
            output_channel_class: OutputChannelClass::RemoteProviderImportedOutput,
            drill_kind: D::LostChannel,
            declared_confidence_tier: ConfidenceTier::ProviderMapped,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::ProviderRunPage,
            links: FallbackLinks {
                execution_context_ref: "exec-context.remote.linked.primary".to_owned(),
                run_ref: Some("run.remote.linked.0001".to_owned()),
                step_ref: Some("step.remote.linked.0001".to_owned()),
                task_ref: Some("task.remote.linked.0001".to_owned()),
                channel_ref: Some("channel.remote.linked.0001".to_owned()),
                problem_ref: Some("problem.remote.linked.0001".to_owned()),
                artifact_ref: None,
                provider_ref: Some("provider.remote.linked.0001".to_owned()),
                adapter_ref: Some("adapter.remote.linked.0001".to_owned()),
                raw_output_backlink_ref: Some("raw.remote.linked.0001".to_owned()),
            },
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: imported("proof.remote.linked.0001"),
            profiles: profiles(
                "fallback:lost-channel-remote:0001",
                ReadOnlyOverlay,
                &[P::OutputChannel, P::PipelineOverlay],
                true,
                Some("run.remote.linked.0001"),
                Some("channel.remote.linked.0001"),
                Some("problem.remote.linked.0001"),
            ),
        },
        // 14. Labs heuristic notebook — makes no public claim.
        FallbackDrillCase {
            case_id: "fallback:labs-heuristic-notebook:0001".to_owned(),
            label_summary:
                "A Labs notebook heuristic parse is unadvertised, makes no public claim, and is never widened or narrowed."
                    .to_owned(),
            claim_posture: ClaimPosture::LabsUnadvertised,
            origin_class: OriginClass::NotebookRun,
            problem_source_kind: ProblemSourceKind::HeuristicOutputParse,
            output_channel_class: OutputChannelClass::TaskTestDebugOutput,
            drill_kind: D::HeuristicTextParse,
            declared_confidence_tier: ConfidenceTier::HeuristicMedium,
            declared_freshness_state: FreshnessState::CachedWithinWindow,
            declared_reopen_target: ReopenTarget::RawOutputBacklink,
            links: FallbackLinks {
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
            virtualization: clean_virtualization(),
            integrity: clean_integrity(),
            verification: FallbackVerification {
                proof_currency: ProofCurrency::RequiresReview,
                proof_ref: None,
            },
            profiles: profiles(
                "fallback:labs-heuristic-notebook:0001",
                LabsNotClaimed,
                &[P::NotebookOutput],
                false,
                Some("run.local.notebook.labs.0001"),
                Some("channel.local.notebook.labs.0001"),
                None,
            ),
        },
    ]
}
