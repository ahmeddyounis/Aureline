//! Canonical in-process truth for the M5 Problems / output-channel /
//! execution-evidence causality matrix.
//!
//! Problems rows, output channels, channel headers, execution-evidence
//! projections, and evidence-bundle exports are one causal system, not three
//! loosely related panes. A user investigating a failure must be able to answer
//! **what ran, what produced this message, how certain the parser was, what
//! run/provider/channel it came from, and how to reopen the originating
//! evidence** without stitching raw logs together by hand.
//!
//! Where [`crate::stabilize_problem_records_output_channels_and_execution_evidence`]
//! froze the per-record canonical task-event, problem, output-channel, output-chunk,
//! and evidence *objects* (stable IDs, stream-first virtualization, retention, and
//! export bundles), this module binds them into one bounded **causality-lane
//! matrix**. Each [`CausalityLaneRow`] is a claimed (or Labs) tooling causality
//! lane: a problem record, output channel, execution-evidence projection, or
//! evidence-bundle export bound to its origin
//! run/step/provider/channel/build-toolchain/host-target identity, its
//! problem-source kind, its confidence tier, its evidence freshness/stale/
//! superseded state, and its reopen-to-origin target.
//!
//! The matrix *re-derives* an effective causal claim per lane that never reads
//! wider than the evidence supports ([`CausalityLaneRow::narrow`]): structured and
//! heuristic origins stay distinct and a heuristic parse keeps a raw-output
//! backlink; run/step/provider/channel/build-toolchain/host-target identity and the
//! original adapter survive into every overlay; large logs stay stream-first,
//! searchable, and exportable; stale and superseded state stay visible; imported
//! provider/remote evidence is a read-only overlay that never claims live local
//! authority; and the canonical evidence stays reopenable. A first-party lane with
//! a stale/missing/labelled gap holds at [`CausalClaim::Narrowed`] (still
//! reopenable); a lane that flattens lineage/channel identity, loses its reopen
//! path, ships an incomplete bundle, has missing evidence, or lets an imported
//! overlay claim live truth floors at [`CausalClaim::Unreconstructable`] and keeps a
//! raw-output backlink or keyboard fallback rather than a clean-but-false causal
//! claim. Remote/pipeline/imported origins certify only as
//! [`CausalClaim::ReadOnlyOverlay`]; Labs/unadvertised lanes make no public claim
//! and are never widened.
//!
//! [`M5ExecutionEvidenceCausalityMatrixPacket::validate`] confirms the packet is
//! well-formed and honest: header/identity/redaction/freshness are present, every
//! surface family and origin class is represented, overlay lanes name their
//! provider, a floored lane keeps a reopen fallback, at least one lane demonstrates
//! the auto-narrowing rule, and no raw boundary material crosses the export.
//! Downstream Problems, output, diagnostics, AI evidence, support export, review,
//! CLI/headless, and docs surfaces ingest this packet rather than inventing a
//! parallel causal model.
//!
//! Raw stdout/stderr bytes, command lines, provider log bodies, env bodies,
//! absolute paths, URLs, and secrets never cross this boundary; the packet carries
//! only typed class tokens, booleans, opaque ids, and redaction-aware reviewable
//! labels.
//!
//! The boundary schema is
//! [`schemas/tooling/m5-execution-evidence.schema.json`](../../../../schemas/tooling/m5-execution-evidence.schema.json).
//! The contract doc is
//! [`docs/tooling/m5-execution-evidence.md`](../../../../docs/tooling/m5-execution-evidence.md).
//! The canonical support export is
//! [`artifacts/tooling/m5-execution-evidence/support_export.json`](../../../../artifacts/tooling/m5-execution-evidence/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/tooling/m5-execution-evidence/`](../../../../fixtures/tooling/m5-execution-evidence/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ExecutionEvidenceCausalityMatrixPacket`].
pub const M5_EXECUTION_EVIDENCE_CAUSALITY_RECORD_KIND: &str =
    "m5_execution_evidence_causality_matrix_packet";

/// Schema version for the execution-evidence causality matrix.
pub const M5_EXECUTION_EVIDENCE_CAUSALITY_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen enum vocabularies.
pub const M5_EXECUTION_EVIDENCE_CAUSALITY_TAXONOMY_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_EXECUTION_EVIDENCE_CAUSALITY_SCHEMA_REF: &str =
    "schemas/tooling/m5-execution-evidence.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_EXECUTION_EVIDENCE_CAUSALITY_DOC_REF: &str = "docs/tooling/m5-execution-evidence.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const M5_EXECUTION_EVIDENCE_CAUSALITY_SUPPORT_EXPORT_REF: &str =
    "artifacts/tooling/m5-execution-evidence/support_export.json";

/// Repo-relative path of the generated causal-claim matrix.
pub const M5_EXECUTION_EVIDENCE_CAUSALITY_MATRIX_REF: &str =
    "artifacts/tooling/m5-execution-evidence/matrix.json";

/// Repo-relative path of the generated certification report.
pub const M5_EXECUTION_EVIDENCE_CAUSALITY_REPORT_REF: &str =
    "artifacts/tooling/m5-execution-evidence/report.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const M5_EXECUTION_EVIDENCE_CAUSALITY_FIXTURE_DIR: &str =
    "fixtures/tooling/m5-execution-evidence";

/// Allowed packet redaction-class tokens.
const REDACTION_CLASS_TOKENS: [&str; 4] = [
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
];

// --------------------------------------------------------------------------- //
// Frozen lane taxonomies (mirror the boundary schema).
// --------------------------------------------------------------------------- //

/// Which Problems/output/execution-evidence surface family a lane backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceFamily {
    /// Problems panel row.
    ProblemsPanel,
    /// Output channel / channel header.
    OutputChannel,
    /// Execution-evidence projection over the editor, timeline, or review.
    ExecutionEvidenceProjection,
    /// Evidence-bundle export.
    EvidenceBundleExport,
}

impl SurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProblemsPanel,
        Self::OutputChannel,
        Self::ExecutionEvidenceProjection,
        Self::EvidenceBundleExport,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProblemsPanel => "problems_panel",
            Self::OutputChannel => "output_channel",
            Self::ExecutionEvidenceProjection => "execution_evidence_projection",
            Self::EvidenceBundleExport => "evidence_bundle_export",
        }
    }
}

/// Whether a lane is publicly claimed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimPosture {
    /// Claimed at stable maturity.
    ClaimedStable,
    /// Claimed at beta maturity.
    ClaimedBeta,
    /// Labs/unadvertised; makes no causal-authority claim and is never widened.
    LabsUnadvertised,
}

impl ClaimPosture {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimedStable => "claimed_stable",
            Self::ClaimedBeta => "claimed_beta",
            Self::LabsUnadvertised => "labs_unadvertised",
        }
    }
}

/// How the run/evidence originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginClass {
    /// Local task run.
    LocalTask,
    /// Local test run.
    LocalTest,
    /// Local debug session.
    LocalDebugSession,
    /// Notebook cell run.
    NotebookRun,
    /// Headless automation run.
    HeadlessAutomation,
    /// Extension-owned run.
    ExtensionOwnedRun,
    /// AI-triggered run.
    AiTriggeredRun,
    /// Remote-linked run surfaced as a read-only overlay.
    RemoteLinkedRun,
    /// Pipeline/provider run surfaced as a read-only overlay.
    PipelineProviderRun,
    /// Imported provider evidence surfaced as a read-only overlay.
    ImportedProviderEvidence,
}

impl OriginClass {
    /// Every origin class, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::LocalTask,
        Self::LocalTest,
        Self::LocalDebugSession,
        Self::NotebookRun,
        Self::HeadlessAutomation,
        Self::ExtensionOwnedRun,
        Self::AiTriggeredRun,
        Self::RemoteLinkedRun,
        Self::PipelineProviderRun,
        Self::ImportedProviderEvidence,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalTask => "local_task",
            Self::LocalTest => "local_test",
            Self::LocalDebugSession => "local_debug_session",
            Self::NotebookRun => "notebook_run",
            Self::HeadlessAutomation => "headless_automation",
            Self::ExtensionOwnedRun => "extension_owned_run",
            Self::AiTriggeredRun => "ai_triggered_run",
            Self::RemoteLinkedRun => "remote_linked_run",
            Self::PipelineProviderRun => "pipeline_provider_run",
            Self::ImportedProviderEvidence => "imported_provider_evidence",
        }
    }

    /// Whether this origin is an inherently read-only overlay: it can never claim
    /// live local causal authority, only an attributable read-only overlay.
    pub const fn is_overlay(self) -> bool {
        matches!(
            self,
            Self::RemoteLinkedRun | Self::PipelineProviderRun | Self::ImportedProviderEvidence
        )
    }
}

/// Appendix BI.1 problem-source class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProblemSourceKind {
    /// Structured language-server / compiler diagnostic.
    StructuredLanguageDiagnostic,
    /// Normalized task event projected into a finding.
    NormalizedTaskEvent,
    /// Heuristic parser/problem-matcher over raw output.
    HeuristicOutputParse,
    /// Imported provider annotation (CI / SARIF-like).
    ImportedProviderAnnotation,
    /// The lane produces no problem record.
    NotApplicable,
}

impl ProblemSourceKind {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredLanguageDiagnostic => "structured_language_diagnostic",
            Self::NormalizedTaskEvent => "normalized_task_event",
            Self::HeuristicOutputParse => "heuristic_output_parse",
            Self::ImportedProviderAnnotation => "imported_provider_annotation",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this source is a heuristic parse, which must keep a raw-output
    /// backlink and an explicit heuristic confidence tier.
    pub const fn is_heuristic(self) -> bool {
        matches!(self, Self::HeuristicOutputParse)
    }
}

/// Appendix BI.2 output-channel / evidence class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputChannelClass {
    /// Task / test / debug output channel.
    TaskTestDebugOutput,
    /// Extension / AI-tool output channel.
    ExtensionAiToolOutput,
    /// Remote / provider / imported output channel.
    RemoteProviderImportedOutput,
    /// Evidence bundle channel.
    EvidenceBundle,
    /// The lane owns no output channel.
    NotApplicable,
}

impl OutputChannelClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskTestDebugOutput => "task_test_debug_output",
            Self::ExtensionAiToolOutput => "extension_ai_tool_output",
            Self::RemoteProviderImportedOutput => "remote_provider_imported_output",
            Self::EvidenceBundle => "evidence_bundle",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this class names a real channel that must carry a stable channel ref.
    pub const fn is_real_channel(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }
}

/// Confidence taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceTier {
    /// Full confidence from a structured source.
    StructuredFull,
    /// High-confidence heuristic parse.
    HeuristicHigh,
    /// Medium-confidence heuristic parse.
    HeuristicMedium,
    /// Low-confidence heuristic parse.
    HeuristicLow,
    /// Imported provider mapping quality.
    ProviderMapped,
    /// Unmapped; a first-class state, not an invisible gap.
    UnmappedRequiresReview,
}

impl ConfidenceTier {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredFull => "structured_full",
            Self::HeuristicHigh => "heuristic_high",
            Self::HeuristicMedium => "heuristic_medium",
            Self::HeuristicLow => "heuristic_low",
            Self::ProviderMapped => "provider_mapped",
            Self::UnmappedRequiresReview => "unmapped_requires_review",
        }
    }

    /// Whether this tier is one of the explicit heuristic tiers.
    pub const fn is_heuristic_tier(self) -> bool {
        matches!(
            self,
            Self::HeuristicHigh | Self::HeuristicMedium | Self::HeuristicLow
        )
    }
}

/// The evidence projection's own freshness / anchoring state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    /// Live and current.
    Live,
    /// Cached within the freshness window.
    CachedWithinWindow,
    /// Stale beyond the freshness window.
    StaleExpired,
    /// Superseded by a newer run.
    SupersededByNewerRun,
    /// Not anchored to the current revision.
    Unanchored,
    /// Evidence missing.
    Missing,
}

impl FreshnessState {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::CachedWithinWindow => "cached_within_window",
            Self::StaleExpired => "stale_expired",
            Self::SupersededByNewerRun => "superseded_by_newer_run",
            Self::Unanchored => "unanchored",
            Self::Missing => "missing",
        }
    }
}

/// Where reopen-to-origin lands.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenTarget {
    /// The owning run.
    OwningRun,
    /// The output channel.
    OutputChannel,
    /// A generated artifact.
    GeneratedArtifact,
    /// The provider's run page.
    ProviderRunPage,
    /// The raw-output backlink.
    RawOutputBacklink,
    /// An editor anchor.
    EditorAnchor,
    /// Lineage could not be reconstructed; only a raw/keyboard fallback remains.
    NoneKeyboardFallback,
}

impl ReopenTarget {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwningRun => "owning_run",
            Self::OutputChannel => "output_channel",
            Self::GeneratedArtifact => "generated_artifact",
            Self::ProviderRunPage => "provider_run_page",
            Self::RawOutputBacklink => "raw_output_backlink",
            Self::EditorAnchor => "editor_anchor",
            Self::NoneKeyboardFallback => "none_keyboard_fallback",
        }
    }

    /// Whether this target is a minimal raw/keyboard fallback that a floored lane
    /// keeps reopenable rather than rendering a clean-but-false causal claim.
    pub const fn is_raw_fallback(self) -> bool {
        matches!(self, Self::RawOutputBacklink | Self::NoneKeyboardFallback)
    }

    /// Reviewer label with underscores expanded.
    fn display(self) -> &'static str {
        match self {
            Self::RawOutputBacklink => "raw-output backlink",
            Self::OwningRun => "owning run",
            Self::OutputChannel => "output channel",
            Self::GeneratedArtifact => "generated artifact",
            Self::ProviderRunPage => "provider run page",
            Self::EditorAnchor => "editor anchor",
            Self::NoneKeyboardFallback => "none keyboard fallback",
        }
    }
}

/// Currency of the certification proof that a lane preserves the causal chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofCurrency {
    /// Verified and current.
    VerifiedCurrent,
    /// Cached within the window.
    CachedWithinWindow,
    /// Imported and current for an overlay.
    ImportedCurrent,
    /// Stale beyond the window.
    StaleExpired,
    /// Missing proof.
    MissingProof,
    /// Requires review.
    RequiresReview,
}

impl ProofCurrency {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCurrent => "verified_current",
            Self::CachedWithinWindow => "cached_within_window",
            Self::ImportedCurrent => "imported_current",
            Self::StaleExpired => "stale_expired",
            Self::MissingProof => "missing_proof",
            Self::RequiresReview => "requires_review",
        }
    }
}

// --------------------------------------------------------------------------- //
// Derived causal-claim ladder and narrowing reasons.
// --------------------------------------------------------------------------- //

/// The effective causal claim a lane renders. A higher rank asserts more causal
/// authority, so a narrowed or floored lane must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CausalClaim {
    /// Lineage/channel/reopen broken or evidence missing; the lane surfaces a
    /// raw-output backlink or keyboard fallback instead of a clean-but-false claim.
    #[serde(rename = "causal_chain_unreconstructable")]
    Unreconstructable,
    /// Remote/pipeline/imported evidence; attributable and reopenable but never
    /// claims live local authority.
    #[serde(rename = "evidence_read_only_overlay")]
    ReadOnlyOverlay,
    /// A first-party lane held below certified by a stale/missing/labelled gap, but
    /// lineage stays reopenable.
    #[serde(rename = "causal_chain_narrowed")]
    Narrowed,
    /// Full first-party causal chain preserved, fresh, confidence honest,
    /// reopenable.
    #[serde(rename = "causal_chain_certified")]
    Certified,
    /// Labs/unadvertised; makes no public causal claim and is never widened.
    #[serde(rename = "causal_evidence_labs_not_claimed")]
    LabsNotClaimed,
}

impl CausalClaim {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unreconstructable => "causal_chain_unreconstructable",
            Self::ReadOnlyOverlay => "evidence_read_only_overlay",
            Self::Narrowed => "causal_chain_narrowed",
            Self::Certified => "causal_chain_certified",
            Self::LabsNotClaimed => "causal_evidence_labs_not_claimed",
        }
    }

    /// Monotonic rank used to compare claims, or `None` for the non-claiming Labs
    /// token (which never participates in widening/narrowing comparisons).
    pub const fn rank(self) -> Option<u8> {
        match self {
            Self::Unreconstructable => Some(0),
            Self::ReadOnlyOverlay => Some(1),
            Self::Narrowed => Some(2),
            Self::Certified => Some(3),
            Self::LabsNotClaimed => None,
        }
    }

    /// Whether rendering `rendered` on a public surface would overclaim relative to
    /// this effective claim. A projection must never render wider than the lane's
    /// effective claim; the Labs token may only render as itself.
    pub fn overclaims_as(self, rendered: CausalClaim) -> bool {
        match (self.rank(), rendered.rank()) {
            (Some(effective), Some(shown)) => shown > effective,
            // Labs renders only as Labs; anything ranked over a Labs lane, or a Labs
            // token rendered over a ranked lane, is an overclaim.
            _ => self != rendered,
        }
    }
}

/// A reason a lane fails to hold its headline causal claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// Structured vs heuristic origin not distinct.
    #[serde(rename = "origin_kind_flattened")]
    OriginFlattened,
    /// Heuristic parse without a raw-output backlink.
    #[serde(rename = "raw_output_backlink_missing")]
    RawBacklinkMissing,
    /// Heuristic confidence tier not surfaced.
    #[serde(rename = "confidence_unlabeled")]
    ConfidenceUnlabeled,
    /// Run/step/provider/channel or origin-adapter lineage flattened.
    #[serde(rename = "run_channel_lineage_flattened")]
    LineageFlattened,
    /// Output channel lost its stable canonical channel ref.
    #[serde(rename = "channel_identity_flattened")]
    ChannelIdentityFlattened,
    /// Build/toolchain or host/target identity not visible.
    #[serde(rename = "build_or_host_target_missing")]
    BuildHostTargetMissing,
    /// Large log not stream-first/searchable/exportable.
    #[serde(rename = "stream_not_virtualized")]
    StreamNotVirtualized,
    /// Reopen-to-origin lost; only a keyboard fallback remains.
    #[serde(rename = "reopen_target_lost")]
    ReopenTargetLost,
    /// Evidence-bundle export missing the minimum reopen identity.
    #[serde(rename = "export_packet_incomplete")]
    ExportPacketIncomplete,
    /// Evidence missing.
    #[serde(rename = "evidence_missing")]
    EvidenceMissing,
    /// Superseded-by-newer-run state not marked.
    #[serde(rename = "superseded_state_not_marked")]
    SupersededNotMarked,
    /// Evidence unanchored to the current revision.
    #[serde(rename = "evidence_unanchored")]
    Unanchored,
    /// First-party evidence projection stale.
    #[serde(rename = "evidence_stale")]
    StaleEvidence,
    /// Verification proof stale or window elapsed.
    #[serde(rename = "verification_proof_stale")]
    StaleProof,
    /// Verification proof missing.
    #[serde(rename = "verification_proof_missing")]
    MissingProof,
    /// Imported/remote overlay claims live local authority.
    #[serde(rename = "imported_overlay_claims_live")]
    ImportedOverlayClaimsLive,
}

impl NarrowingReason {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OriginFlattened => "origin_kind_flattened",
            Self::RawBacklinkMissing => "raw_output_backlink_missing",
            Self::ConfidenceUnlabeled => "confidence_unlabeled",
            Self::LineageFlattened => "run_channel_lineage_flattened",
            Self::ChannelIdentityFlattened => "channel_identity_flattened",
            Self::BuildHostTargetMissing => "build_or_host_target_missing",
            Self::StreamNotVirtualized => "stream_not_virtualized",
            Self::ReopenTargetLost => "reopen_target_lost",
            Self::ExportPacketIncomplete => "export_packet_incomplete",
            Self::EvidenceMissing => "evidence_missing",
            Self::SupersededNotMarked => "superseded_state_not_marked",
            Self::Unanchored => "evidence_unanchored",
            Self::StaleEvidence => "evidence_stale",
            Self::StaleProof => "verification_proof_stale",
            Self::MissingProof => "verification_proof_missing",
            Self::ImportedOverlayClaimsLive => "imported_overlay_claims_live",
        }
    }

    /// Whether this reason floors a lane to [`CausalClaim::Unreconstructable`].
    /// Each floor reason breaks the "stay reopenable / never flatten lineage /
    /// never masquerade as live" contract outright rather than merely aging out.
    pub const fn is_floor(self) -> bool {
        matches!(
            self,
            Self::RawBacklinkMissing
                | Self::LineageFlattened
                | Self::ChannelIdentityFlattened
                | Self::ReopenTargetLost
                | Self::ExportPacketIncomplete
                | Self::EvidenceMissing
                | Self::ImportedOverlayClaimsLive
        )
    }

    /// Deterministic ordering index so recorded reason lists are stable across
    /// runs. Floor reasons sort first so the headline trigger is the most severe.
    const fn order_index(self) -> u8 {
        match self {
            Self::LineageFlattened => 0,
            Self::ChannelIdentityFlattened => 1,
            Self::ReopenTargetLost => 2,
            Self::RawBacklinkMissing => 3,
            Self::ExportPacketIncomplete => 4,
            Self::EvidenceMissing => 5,
            Self::ImportedOverlayClaimsLive => 6,
            Self::OriginFlattened => 7,
            Self::ConfidenceUnlabeled => 8,
            Self::BuildHostTargetMissing => 9,
            Self::StreamNotVirtualized => 10,
            Self::SupersededNotMarked => 11,
            Self::Unanchored => 12,
            Self::StaleEvidence => 13,
            Self::StaleProof => 14,
            Self::MissingProof => 15,
        }
    }
}

/// Sort reasons by their canonical order and drop duplicates.
fn order_reasons(mut reasons: Vec<NarrowingReason>) -> Vec<NarrowingReason> {
    reasons.sort_by_key(|reason| reason.order_index());
    reasons.dedup();
    reasons
}

// --------------------------------------------------------------------------- //
// Lane sub-objects.
// --------------------------------------------------------------------------- //

/// Stable identifiers binding a lane to its origin. Lineage is reconstructed from
/// these refs, never inferred from freeform display text. Absent refs serialize as
/// `null` so the schema's required keys stay present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneIdentity {
    /// Execution-context ref (required).
    pub execution_context_ref: String,
    /// Owning run ref.
    pub run_ref: Option<String>,
    /// Step ref.
    pub step_ref: Option<String>,
    /// Provider ref (required for remote/pipeline/imported overlays).
    pub provider_ref: Option<String>,
    /// Output-channel ref.
    pub channel_ref: Option<String>,
    /// Build/toolchain ref.
    pub build_toolchain_ref: Option<String>,
    /// Host/target ref.
    pub host_target_ref: Option<String>,
    /// Task-event envelope ref.
    pub task_event_envelope_ref: Option<String>,
    /// Problem record id.
    pub problem_record_id: Option<String>,
    /// Evidence bundle id.
    pub evidence_bundle_id: Option<String>,
}

/// The causal-chain invariants every lane re-derives rather than trusting a grade.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CausalChain {
    /// Structured and heuristic origins stay distinct.
    pub structured_vs_heuristic_distinct: bool,
    /// A heuristic parse keeps a raw-output backlink.
    pub raw_output_backlink_present: bool,
    /// The original adapter survives.
    pub preserves_origin_adapter: bool,
    /// Run/step/provider/channel identity survives.
    pub preserves_run_step_provider_channel: bool,
    /// Build/toolchain and host/target identity survive.
    pub preserves_build_toolchain_host_target: bool,
    /// The confidence label is visible.
    pub confidence_label_visible: bool,
    /// Large logs stay stream-first, searchable, and exportable.
    pub stream_first_searchable_exportable: bool,
    /// Overlays never flatten the original lineage.
    pub overlay_preserves_lineage: bool,
    /// Superseded state stays marked.
    pub superseded_state_marked: bool,
    /// Imported overlays stay read-only.
    pub imported_overlay_read_only: bool,
}

/// Minimum evidence-bundle export identity: a bundle must be attributable and
/// reopenable without the original UI state or a live provider session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportPacket {
    /// Bundle id present.
    pub bundle_id_present: bool,
    /// Included event refs present.
    pub included_event_refs_present: bool,
    /// Included channel refs present.
    pub included_channel_refs_present: bool,
    /// Included evidence refs present.
    pub included_evidence_refs_present: bool,
    /// Redaction profile present.
    pub redaction_profile_present: bool,
    /// Retention state present.
    pub retention_state_present: bool,
    /// Reopen refs present.
    pub reopen_refs_present: bool,
}

impl ExportPacket {
    /// Whether every minimum-identity field is present.
    pub const fn is_complete(self) -> bool {
        self.bundle_id_present
            && self.included_event_refs_present
            && self.included_channel_refs_present
            && self.included_evidence_refs_present
            && self.redaction_profile_present
            && self.retention_state_present
            && self.reopen_refs_present
    }
}

/// Certification-proof currency for a lane (distinct from the evidence's own
/// freshness state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneVerification {
    /// Currency of the certification proof.
    pub proof_currency: ProofCurrency,
    /// Proof ref, or `null` when no proof anchors the lane.
    pub proof_ref: Option<String>,
}

/// Evidence freshness window for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationFreshness {
    /// Freshness SLO in hours; zero is invalid.
    pub verification_freshness_slo_hours: u32,
    /// Timestamp of the last verification refresh.
    pub last_verification_refresh: String,
    /// Whether a lane auto-narrows once the window elapses.
    pub auto_downgrade_on_stale: bool,
}

// --------------------------------------------------------------------------- //
// Lane row + derivation.
// --------------------------------------------------------------------------- //

/// One claimed (or Labs) tooling causality lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CausalityLaneRow {
    /// Stable lane id.
    pub lane_id: String,
    /// Surface family this lane primarily backs.
    pub surface_family: SurfaceFamily,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Whether the lane is publicly claimed.
    pub claim_posture: ClaimPosture,
    /// How the run/evidence originated.
    pub origin_class: OriginClass,
    /// Problem-source kind.
    pub problem_source_kind: ProblemSourceKind,
    /// Output-channel / evidence class.
    pub output_channel_class: OutputChannelClass,
    /// Declared confidence tier.
    pub declared_confidence_tier: ConfidenceTier,
    /// Declared freshness state.
    pub declared_freshness_state: FreshnessState,
    /// Declared reopen target.
    pub declared_reopen_target: ReopenTarget,
    /// Stable identity block.
    pub identity: LaneIdentity,
    /// Causal-chain invariant block.
    pub causal_chain: CausalChain,
    /// Evidence-bundle export identity block.
    pub export_packet: ExportPacket,
    /// Certification-proof block.
    pub verification: LaneVerification,
}

/// The re-derived causal decision for one lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneCausalDecision {
    /// The headline claim the lane is eligible to make from its posture/origin.
    pub claimed_causality_claim: CausalClaim,
    /// The effective claim after re-derivation; never wider than the evidence.
    pub effective_causality_claim: CausalClaim,
    /// Ordered, de-duplicated reasons the lane fails to hold its headline claim.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// Whether the effective claim ranks below the claimed claim.
    pub narrowed: bool,
}

impl LaneCausalDecision {
    /// The headline downgrade trigger, when narrowed: the most severe reason.
    pub fn downgrade_trigger(&self) -> Option<NarrowingReason> {
        if self.narrowed {
            self.active_narrowing_reasons.first().copied()
        } else {
            None
        }
    }

    /// Whether a public surface rendering `rendered` for this lane would overclaim.
    pub fn surface_overclaims(&self, rendered: CausalClaim) -> bool {
        self.effective_causality_claim.overclaims_as(rendered)
    }
}

impl CausalityLaneRow {
    /// Whether this lane is Labs/unadvertised.
    pub fn is_labs(&self) -> bool {
        matches!(self.claim_posture, ClaimPosture::LabsUnadvertised)
    }

    /// Whether this lane is an inherently read-only overlay origin.
    pub fn is_overlay_origin(&self) -> bool {
        self.origin_class.is_overlay()
    }

    /// The headline causal claim this lane is eligible to make.
    pub fn claimed_claim(&self) -> CausalClaim {
        if self.is_labs() {
            CausalClaim::LabsNotClaimed
        } else if self.is_overlay_origin() {
            CausalClaim::ReadOnlyOverlay
        } else {
            CausalClaim::Certified
        }
    }

    /// Every causal-chain reason this lane fails to hold its headline claim.
    ///
    /// `stale_window` is true when the matrix verification window has elapsed by the
    /// evaluation time, which ages out a lane resting on a current proof.
    pub fn lane_reasons(&self, stale_window: bool) -> Vec<NarrowingReason> {
        let cc = &self.causal_chain;
        let overlay = self.is_overlay_origin();
        let mut reasons: Vec<NarrowingReason> = Vec::new();

        // Origin honesty: structured vs heuristic must stay distinct.
        if !cc.structured_vs_heuristic_distinct {
            reasons.push(NarrowingReason::OriginFlattened);
        }

        // A heuristic parse must keep a raw-output backlink and an explicit tier.
        if self.problem_source_kind.is_heuristic() {
            if !cc.raw_output_backlink_present {
                reasons.push(NarrowingReason::RawBacklinkMissing);
            }
            if !self.declared_confidence_tier.is_heuristic_tier() || !cc.confidence_label_visible {
                reasons.push(NarrowingReason::ConfidenceUnlabeled);
            }
        } else if !cc.confidence_label_visible {
            reasons.push(NarrowingReason::ConfidenceUnlabeled);
        }

        // Lineage: origin adapter + run/step/provider/channel + overlay lineage.
        if !(cc.preserves_origin_adapter
            && cc.preserves_run_step_provider_channel
            && cc.overlay_preserves_lineage)
        {
            reasons.push(NarrowingReason::LineageFlattened);
        }
        if !cc.preserves_build_toolchain_host_target {
            reasons.push(NarrowingReason::BuildHostTargetMissing);
        }

        // Channel identity: a real channel must carry a stable channel ref.
        if self.output_channel_class.is_real_channel() && !opt_present(&self.identity.channel_ref) {
            reasons.push(NarrowingReason::ChannelIdentityFlattened);
        }
        if matches!(self.surface_family, SurfaceFamily::OutputChannel)
            && !cc.stream_first_searchable_exportable
        {
            reasons.push(NarrowingReason::StreamNotVirtualized);
        }

        // Reopen-to-origin must survive.
        if matches!(
            self.declared_reopen_target,
            ReopenTarget::NoneKeyboardFallback
        ) {
            reasons.push(NarrowingReason::ReopenTargetLost);
        }

        // Evidence-bundle export minimums.
        if matches!(self.surface_family, SurfaceFamily::EvidenceBundleExport)
            && !self.export_packet.is_complete()
        {
            reasons.push(NarrowingReason::ExportPacketIncomplete);
        }

        // Evidence freshness / superseded / missing / unanchored.
        match self.declared_freshness_state {
            FreshnessState::Missing => reasons.push(NarrowingReason::EvidenceMissing),
            FreshnessState::SupersededByNewerRun if !cc.superseded_state_marked => {
                reasons.push(NarrowingReason::SupersededNotMarked);
            }
            FreshnessState::Unanchored => reasons.push(NarrowingReason::Unanchored),
            // An overlay snapshot is expected to be cached/stale; a first-party live
            // surface showing a stale projection has aged out of currency.
            FreshnessState::StaleExpired if !overlay => {
                reasons.push(NarrowingReason::StaleEvidence);
            }
            _ => {}
        }

        // Certification-proof currency (distinct from the evidence's own freshness).
        match self.verification.proof_currency {
            ProofCurrency::MissingProof => reasons.push(NarrowingReason::MissingProof),
            ProofCurrency::StaleExpired | ProofCurrency::RequiresReview => {
                reasons.push(NarrowingReason::StaleProof);
            }
            ProofCurrency::VerifiedCurrent | ProofCurrency::CachedWithinWindow if stale_window => {
                reasons.push(NarrowingReason::StaleProof);
            }
            _ => {}
        }

        // Imported/remote overlays must stay read-only.
        if overlay && !cc.imported_overlay_read_only {
            reasons.push(NarrowingReason::ImportedOverlayClaimsLive);
        }

        order_reasons(reasons)
    }

    /// Re-derive the effective causal claim, reasons, and narrowed flag.
    pub fn narrow(&self, stale_window: bool) -> LaneCausalDecision {
        let claimed = self.claimed_claim();

        // Labs/unadvertised lanes make no public claim, so they never accrue
        // governance narrowing; they hold their non-claiming token.
        if matches!(claimed, CausalClaim::LabsNotClaimed) {
            return LaneCausalDecision {
                claimed_causality_claim: CausalClaim::LabsNotClaimed,
                effective_causality_claim: CausalClaim::LabsNotClaimed,
                active_narrowing_reasons: Vec::new(),
                narrowed: false,
            };
        }

        let reasons = self.lane_reasons(stale_window);
        let floored = reasons.iter().any(|reason| reason.is_floor());

        let effective = if floored {
            CausalClaim::Unreconstructable
        } else if !reasons.is_empty() {
            // An overlay is already the minimal honest claim: if anything else is
            // off we can no longer certify even the read-only overlay, so we floor
            // it. A first-party lane holds at narrowed (still reopenable).
            if matches!(claimed, CausalClaim::ReadOnlyOverlay) {
                CausalClaim::Unreconstructable
            } else {
                CausalClaim::Narrowed
            }
        } else {
            claimed
        };

        let narrowed = matches!(
            (effective.rank(), claimed.rank()),
            (Some(eff), Some(claim)) if eff < claim
        );

        LaneCausalDecision {
            claimed_causality_claim: claimed,
            effective_causality_claim: effective,
            active_narrowing_reasons: reasons,
            narrowed,
        }
    }

    /// The effective confidence tier: a floored lane cannot assert a tier beyond
    /// unmapped/needs-review.
    pub fn effective_confidence(&self, effective: CausalClaim) -> ConfidenceTier {
        if matches!(effective, CausalClaim::Unreconstructable) {
            ConfidenceTier::UnmappedRequiresReview
        } else {
            self.declared_confidence_tier
        }
    }

    /// A precise, non-generic reviewer label for a narrowed/floored lane.
    pub fn narrowed_label(&self, decision: &LaneCausalDecision) -> Option<String> {
        if !decision.narrowed {
            return None;
        }
        let trigger = decision
            .downgrade_trigger()
            .map_or("narrowed", NarrowingReason::as_str)
            .replace('_', " ");
        let reopen = self.declared_reopen_target.display();
        let claimed = decision.claimed_causality_claim.as_str();
        let effective = decision.effective_causality_claim;
        let label = if matches!(effective, CausalClaim::Unreconstructable) {
            format!(
                "Floored to {} below the {claimed} claim: {trigger}; the {reopen} stays reopenable rather than rendering a clean-but-false causal claim",
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

    /// Whether a non-labs lane that floors keeps a reopen fallback rather than
    /// hiding lineage behind a clean-but-false claim.
    fn floored_lane_keeps_fallback(&self, effective: CausalClaim) -> bool {
        if !matches!(effective, CausalClaim::Unreconstructable) {
            return true;
        }
        self.declared_reopen_target.is_raw_fallback()
            || self.causal_chain.raw_output_backlink_present
    }

    /// Structural row checks that hold independently of the narrowing derivation.
    fn structural_violations(&self, out: &mut Vec<M5ExecutionEvidenceCausalityViolation>) {
        if self.lane_id.trim().is_empty()
            || self.label_summary.trim().is_empty()
            || self.identity.execution_context_ref.trim().is_empty()
        {
            out.push(M5ExecutionEvidenceCausalityViolation::RowMissingIdentity);
        }
        if self.is_overlay_origin() && !opt_present(&self.identity.provider_ref) {
            out.push(M5ExecutionEvidenceCausalityViolation::OverlayMissingProviderRef);
        }
    }
}

/// Whether an optional ref is present and non-empty.
fn opt_present(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|inner| !inner.trim().is_empty())
}

// --------------------------------------------------------------------------- //
// Packet.
// --------------------------------------------------------------------------- //

/// Constructor input for an [`M5ExecutionEvidenceCausalityMatrixPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionEvidenceCausalityMatrixInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-lane causality rows.
    pub rows: Vec<CausalityLaneRow>,
}

/// Export-safe M5 execution-evidence causality matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExecutionEvidenceCausalityMatrixPacket {
    /// Record kind; must equal [`M5_EXECUTION_EVIDENCE_CAUSALITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_EXECUTION_EVIDENCE_CAUSALITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub label: String,
    /// Evaluation/mint timestamp (RFC 3339).
    pub as_of: String,
    /// Taxonomy version; must equal
    /// [`M5_EXECUTION_EVIDENCE_CAUSALITY_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Evidence freshness window.
    pub verification_freshness: VerificationFreshness,
    /// Per-lane causality rows.
    pub rows: Vec<CausalityLaneRow>,
}

/// The distribution of effective causal claims across a matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimDistribution {
    /// Lanes effective at [`CausalClaim::Certified`].
    pub certified: usize,
    /// Lanes effective at [`CausalClaim::Narrowed`].
    pub narrowed: usize,
    /// Lanes effective at [`CausalClaim::ReadOnlyOverlay`].
    pub overlay: usize,
    /// Lanes effective at [`CausalClaim::Unreconstructable`].
    pub unreconstructable: usize,
    /// Lanes effective at [`CausalClaim::LabsNotClaimed`].
    pub labs: usize,
}

impl M5ExecutionEvidenceCausalityMatrixPacket {
    /// Builds a causality matrix packet, sealing the record-kind, schema, and
    /// taxonomy version constants.
    pub fn new(input: M5ExecutionEvidenceCausalityMatrixInput) -> Self {
        Self {
            record_kind: M5_EXECUTION_EVIDENCE_CAUSALITY_RECORD_KIND.to_owned(),
            schema_version: M5_EXECUTION_EVIDENCE_CAUSALITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            taxonomy_version: M5_EXECUTION_EVIDENCE_CAUSALITY_TAXONOMY_VERSION,
            redaction_class_token: input.redaction_class_token,
            verification_freshness: input.verification_freshness,
            rows: input.rows,
        }
    }

    /// Whether the matrix verification window has elapsed by `as_of`.
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

    /// Re-derive the causal decision for every lane, paired with its lane id.
    pub fn decisions(&self) -> Vec<(String, LaneCausalDecision)> {
        let stale_window = self.stale_window();
        self.rows
            .iter()
            .map(|row| (row.lane_id.clone(), row.narrow(stale_window)))
            .collect()
    }

    /// The distribution of effective causal claims.
    pub fn claim_distribution(&self) -> ClaimDistribution {
        let stale_window = self.stale_window();
        let mut dist = ClaimDistribution {
            certified: 0,
            narrowed: 0,
            overlay: 0,
            unreconstructable: 0,
            labs: 0,
        };
        for row in &self.rows {
            match row.narrow(stale_window).effective_causality_claim {
                CausalClaim::Certified => dist.certified += 1,
                CausalClaim::Narrowed => dist.narrowed += 1,
                CausalClaim::ReadOnlyOverlay => dist.overlay += 1,
                CausalClaim::Unreconstructable => dist.unreconstructable += 1,
                CausalClaim::LabsNotClaimed => dist.labs += 1,
            }
        }
        dist
    }

    /// Count of lanes whose effective claim ranks below their claimed claim.
    pub fn narrowed_lane_count(&self) -> usize {
        let stale_window = self.stale_window();
        self.rows
            .iter()
            .filter(|row| row.narrow(stale_window).narrowed)
            .count()
    }

    /// Surface families represented by some lane.
    pub fn represented_surface_families(&self) -> BTreeSet<SurfaceFamily> {
        self.rows.iter().map(|row| row.surface_family).collect()
    }

    /// Origin classes represented by some lane.
    pub fn represented_origin_classes(&self) -> BTreeSet<OriginClass> {
        self.rows.iter().map(|row| row.origin_class).collect()
    }

    /// Validate the execution-evidence causality invariants.
    pub fn validate(&self) -> Vec<M5ExecutionEvidenceCausalityViolation> {
        use M5ExecutionEvidenceCausalityViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_EXECUTION_EVIDENCE_CAUSALITY_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_EXECUTION_EVIDENCE_CAUSALITY_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.taxonomy_version != M5_EXECUTION_EVIDENCE_CAUSALITY_TAXONOMY_VERSION {
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
        if self.rows.is_empty() {
            violations.push(V::EmptyRows);
        }

        let mut seen: BTreeSet<&str> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.lane_id.as_str()) {
                violations.push(V::DuplicateLaneId);
            }
        }

        let surfaces = self.represented_surface_families();
        if SurfaceFamily::ALL.iter().any(|f| !surfaces.contains(f)) {
            violations.push(V::SurfaceFamilyMissing);
        }
        let origins = self.represented_origin_classes();
        if OriginClass::ALL.iter().any(|o| !origins.contains(o)) {
            violations.push(V::OriginClassMissing);
        }

        let stale_window = self.stale_window();
        let mut demonstrates_narrowing = false;
        for row in &self.rows {
            row.structural_violations(&mut violations);
            let decision = row.narrow(stale_window);
            if decision.narrowed {
                demonstrates_narrowing = true;
                if decision.downgrade_trigger().is_none()
                    || row
                        .narrowed_label(&decision)
                        .map_or(true, |label| label_is_generic(&label))
                {
                    violations.push(V::NarrowedRowMissingLabelOrTrigger);
                }
            }
            if !row.floored_lane_keeps_fallback(decision.effective_causality_claim) {
                violations.push(V::FlooredLaneLosesFallback);
            }
        }
        if !demonstrates_narrowing {
            violations.push(V::DowngradedRowCaseMissing);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("execution-evidence causality packet serializes"),
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
        serde_json::to_string_pretty(self).expect("execution-evidence causality packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stale_window = self.stale_window();
        let dist = self.claim_distribution();
        let mut out = String::new();
        out.push_str("# M5 Execution-Evidence Causality Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Lanes: {}\n", self.rows.len()));
        out.push_str(&format!(
            "- Effective: {} certified, {} narrowed, {} read-only overlay, {} unreconstructable, {} labs\n\n",
            dist.certified, dist.narrowed, dist.overlay, dist.unreconstructable, dist.labs
        ));

        out.push_str("| Lane | Surface | Origin | Claimed | Effective | Confidence |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- |\n");
        for row in &self.rows {
            let decision = row.narrow(stale_window);
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                row.lane_id,
                row.surface_family.as_str(),
                row.origin_class.as_str(),
                decision.claimed_causality_claim.as_str(),
                decision.effective_causality_claim.as_str(),
                row.effective_confidence(decision.effective_causality_claim)
                    .as_str(),
            ));
        }

        out.push('\n');
        for row in &self.rows {
            let decision = row.narrow(stale_window);
            if let Some(label) = row.narrowed_label(&decision) {
                out.push_str(&format!("- Narrowed: `{}` — {}\n", row.lane_id, label));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum M5ExecutionEvidenceCausalityArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<M5ExecutionEvidenceCausalityViolation>),
}

impl fmt::Display for M5ExecutionEvidenceCausalityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(
                    f,
                    "execution-evidence causality support export parse error: {err}"
                )
            }
            Self::Validation(violations) => write!(
                f,
                "execution-evidence causality support export failed validation: {violations:?}"
            ),
        }
    }
}

impl Error for M5ExecutionEvidenceCausalityArtifactError {}

/// Invariant violations reported by
/// [`M5ExecutionEvidenceCausalityMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionEvidenceCausalityViolation {
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
    /// The packet carries no rows.
    EmptyRows,
    /// Two rows share a lane id.
    DuplicateLaneId,
    /// A required surface family is unrepresented.
    SurfaceFamilyMissing,
    /// A required origin class is unrepresented.
    OriginClassMissing,
    /// A row is missing its lane id, label, or execution-context ref.
    RowMissingIdentity,
    /// An overlay-origin row does not name its provider.
    OverlayMissingProviderRef,
    /// A floored lane lost its raw-output / keyboard reopen fallback.
    FlooredLaneLosesFallback,
    /// A narrowed row is missing its precise label or trigger.
    NarrowedRowMissingLabelOrTrigger,
    /// No lane demonstrates the auto-narrowing rule.
    DowngradedRowCaseMissing,
    /// Export-safe JSON carried forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ExecutionEvidenceCausalityViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::WrongTaxonomyVersion => "wrong_taxonomy_version",
            Self::MissingIdentity => "missing_identity",
            Self::InvalidRedactionClass => "invalid_redaction_class",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::EmptyRows => "empty_rows",
            Self::DuplicateLaneId => "duplicate_lane_id",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::OriginClassMissing => "origin_class_missing",
            Self::RowMissingIdentity => "row_missing_identity",
            Self::OverlayMissingProviderRef => "overlay_missing_provider_ref",
            Self::FlooredLaneLosesFallback => "floored_lane_loses_fallback",
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
/// the frozen causality matrix instead of cloning provider-local state.
///
/// # Errors
///
/// Returns [`M5ExecutionEvidenceCausalityArtifactError`] when the artifact cannot
/// be parsed or fails validation.
pub fn current_m5_execution_evidence_causality_matrix(
) -> Result<M5ExecutionEvidenceCausalityMatrixPacket, M5ExecutionEvidenceCausalityArtifactError> {
    let packet: M5ExecutionEvidenceCausalityMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/tooling/m5-execution-evidence/support_export.json"
        )))
        .map_err(M5ExecutionEvidenceCausalityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ExecutionEvidenceCausalityArtifactError::Validation(
            violations,
        ))
    }
}

// --------------------------------------------------------------------------- //
// Helpers.
// --------------------------------------------------------------------------- //

/// Whether a degraded label is a generic non-answer rather than a precise label.
pub(crate) fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    matches!(
        trimmed.to_lowercase().as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "downgraded"
            | "unverified"
            | "narrowed"
            | "stale"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
pub(crate) fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Parse an RFC 3339 UTC timestamp into seconds since the Unix epoch.
///
/// Supports the `Z`/`z` zulu suffix and `±HH:MM` / `±HHMM` numeric offsets, with an
/// optional fractional-second component that is truncated. Returns `None` for
/// malformed input rather than panicking.
pub(crate) fn parse_rfc3339_to_epoch_seconds(value: &str) -> Option<i64> {
    let s = value.trim();
    let bytes = s.as_bytes();
    if s.len() < 19 {
        return None;
    }
    let year: i64 = s.get(0..4)?.parse().ok()?;
    if *bytes.get(4)? != b'-' {
        return None;
    }
    let month: u32 = s.get(5..7)?.parse().ok()?;
    if *bytes.get(7)? != b'-' {
        return None;
    }
    let day: u32 = s.get(8..10)?.parse().ok()?;
    match bytes.get(10)? {
        b'T' | b't' | b' ' => {}
        _ => return None,
    }
    let hour: i64 = s.get(11..13)?.parse().ok()?;
    if *bytes.get(13)? != b':' {
        return None;
    }
    let minute: i64 = s.get(14..16)?.parse().ok()?;
    if *bytes.get(16)? != b':' {
        return None;
    }
    let second: i64 = s.get(17..19)?.parse().ok()?;

    let mut rest = &s[19..];
    if let Some(stripped) = rest.strip_prefix('.') {
        let digits = stripped.bytes().take_while(u8::is_ascii_digit).count();
        rest = &stripped[digits..];
    }

    let offset_seconds = if rest.is_empty() || rest == "Z" || rest == "z" {
        0
    } else {
        let sign = match rest.bytes().next()? {
            b'+' => 1,
            b'-' => -1,
            _ => return None,
        };
        let off = &rest[1..];
        let (oh, om) = if let Some((hh, mm)) = off.split_once(':') {
            (hh.parse::<i64>().ok()?, mm.parse::<i64>().ok()?)
        } else if off.len() == 4 {
            (
                off.get(0..2)?.parse::<i64>().ok()?,
                off.get(2..4)?.parse::<i64>().ok()?,
            )
        } else {
            return None;
        };
        sign * (oh * 3600 + om * 60)
    };

    if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    let days = days_from_civil(year, month, day);
    Some(days * 86_400 + hour * 3600 + minute * 60 + second - offset_seconds)
}

/// Days since 1970-01-01 for a proleptic-Gregorian date (Hinnant's algorithm).
fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let m = i64::from(month);
    let mp = if m > 2 { m - 3 } else { m + 9 };
    let doy = (153 * mp + 2) / 5 + i64::from(day) - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}
