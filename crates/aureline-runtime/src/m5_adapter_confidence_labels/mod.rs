//! Adapter-confidence labels, heuristic-fallback banners, and the
//! no-lower-confidence-overwrite contract shared by the task center, test
//! trees, coverage/flaky views, pipeline overlays, notebook run history, and the
//! support/export, CLI/headless, and AI evidence surfaces.
//!
//! The canonical task-event envelope ([`crate::m5_task_event_envelope_bus`])
//! already carries a source kind, an adapter-priority rank, a confidence level,
//! and a downgrade flag on every record, and the frozen policy layer
//! ([`crate::m5_task_event_adapter_policy`]) fixes the native-first priority
//! ladder, the per-source confidence ceilings, and the closed downgrade
//! vocabulary. This module turns those parser-internal facts into a
//! *user-visible contract*: it binds one [`ConfidenceLabel`] (source class chip,
//! confidence chip, and a heuristic-fallback banner with its reason) to every
//! claimed surface, and it arbitrates the case the docs care most about — two
//! sources describing the same lifecycle slot or artifact — so a weaker or
//! imported emission may *enrich* context but can never *silently overwrite*
//! native, BSP, or Bazel BEP/BES truth.
//!
//! It reuses the [`crate::build_test_event_interoperability`] source-kind,
//! confidence, lifecycle, severity, and promotion vocabulary and the
//! [`crate::m5_task_event_adapter_policy`] priority ladder, confidence ceilings,
//! and [`DowngradeReason`] fallback vocabulary rather than minting parallel
//! tokens. Beyond those it adds exactly two reusable vocabularies that desktop,
//! CLI/headless, AI, and support flows can all inspect: the [`OverwriteDecision`]
//! (with its [`OverwriteReason`]) recorded for every challenger claim, and the
//! [`SourceQualityChange`] recorded for every claim subject.
//!
//! Four invariants keep confidence preservation honest:
//!
//! - **Source class and confidence stay two chips, never one badge.** Every
//!   surface binding renders the source class and the confidence as distinct
//!   cues, so the guardrail against compressing them into one badge or a generic
//!   "partial" label is enforced, not assumed.
//! - **Heuristic rows always carry a banner.** A heuristic-parser emission shows
//!   a heuristic-fallback banner naming its [`DowngradeReason`] on every claimed
//!   surface; a native/BSP/BEP/structured emission shows neither.
//! - **Weaker sources cannot overwrite stronger truth.** When more than one
//!   source describes the same subject, the strongest claim is authoritative and
//!   every weaker claim that attempts to overwrite it is visibly
//!   [`BlockedLowerConfidence`](OverwriteDecision::BlockedLowerConfidence); a
//!   weaker claim that never asserted authority is kept as inspectable
//!   [`EnrichedContextOnly`](OverwriteDecision::EnrichedContextOnly) context.
//! - **Lineage is never dropped to resolve a conflict.** Every claim a subject
//!   ever saw is retained, so a reviewer can replay the raw-to-authoritative
//!   chain through export, CLI/headless, and AI evidence without losing
//!   provenance.
//!
//! The reviewer-facing contract lives at
//! [`/docs/m5/adapter-confidence-and-fallback.md`](../../../docs/m5/adapter-confidence-and-fallback.md);
//! the machine-readable boundary lives at
//! [`/schemas/tooling/adapter-confidence-audit.schema.json`](../../../schemas/tooling/adapter-confidence-audit.schema.json).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::build_test_event_interoperability::{
    BuildTestEventConfidence, BuildTestEventKind, BuildTestEventSourceKind,
    BuildTestInteropFindingSeverity, BuildTestInteropPromotionState,
};
use crate::m5_task_event_adapter_policy::{
    canonical_confidence_ceiling, canonical_priority_rank, source_is_authoritative, DowngradeReason,
};

/// Stable record-kind tag for [`AdapterConfidenceAudit`].
pub const ADAPTER_CONFIDENCE_AUDIT_RECORD_KIND: &str = "m5_adapter_confidence_audit";

/// Stable record-kind tag for [`AdapterConfidenceAuditSupportExport`].
pub const ADAPTER_CONFIDENCE_AUDIT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_adapter_confidence_audit_support_export";

/// Stable record-kind tag for [`AdapterConfidenceCliHeadlessView`].
pub const ADAPTER_CONFIDENCE_AUDIT_CLI_HEADLESS_RECORD_KIND: &str =
    "m5_adapter_confidence_audit_cli_headless";

/// Stable record-kind tag for [`AdapterConfidenceAiEvidenceView`].
pub const ADAPTER_CONFIDENCE_AUDIT_AI_EVIDENCE_RECORD_KIND: &str =
    "m5_adapter_confidence_audit_ai_evidence";

/// Integer schema version for the adapter-confidence audit.
pub const ADAPTER_CONFIDENCE_AUDIT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the adapter-confidence-audit boundary schema.
pub const ADAPTER_CONFIDENCE_AUDIT_SCHEMA_REF: &str =
    "schemas/tooling/adapter-confidence-audit.schema.json";

/// Repo-relative path of the per-event task-event envelope boundary schema.
pub const ADAPTER_CONFIDENCE_AUDIT_ENVELOPE_SCHEMA_REF: &str =
    "schemas/tooling/task-event-envelope.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const ADAPTER_CONFIDENCE_AUDIT_DOC_REF: &str = "docs/m5/adapter-confidence-and-fallback.md";

/// Repo-relative path of the frozen adapter-policy baseline this lane consumes.
pub const ADAPTER_CONFIDENCE_AUDIT_POLICY_BASELINE_REF: &str =
    "artifacts/m5/tooling/event-interop-baseline/baseline.json";

/// Repo-relative path of the protected fixture corpus directory.
pub const ADAPTER_CONFIDENCE_AUDIT_FIXTURE_DIR: &str =
    "fixtures/tooling/m5/confidence-preservation";

/// Repo-relative path of the checked-in audit artifact.
pub const ADAPTER_CONFIDENCE_AUDIT_PACKET_ARTIFACT_REF: &str =
    "artifacts/m5/tooling/adapter-confidence-audit/packet.json";

/// Stable audit id minted by the seed.
pub const ADAPTER_CONFIDENCE_AUDIT_ID: &str = "tooling:m5:adapter-confidence-audit:v1";

/// Stable support-export id minted by the seed inspector.
pub const ADAPTER_CONFIDENCE_AUDIT_SUPPORT_EXPORT_ID: &str =
    "support-export:tooling:m5:adapter-confidence-audit";

/// Stable CLI/headless view id minted by the seed inspector.
pub const ADAPTER_CONFIDENCE_AUDIT_CLI_HEADLESS_ID: &str =
    "cli-headless:tooling:m5:adapter-confidence-audit";

/// Stable AI evidence view id minted by the seed inspector.
pub const ADAPTER_CONFIDENCE_AUDIT_AI_EVIDENCE_ID: &str =
    "ai-evidence:tooling:m5:adapter-confidence-audit";

/// Claimed M5 surface that renders adapter-confidence labels and banners.
///
/// The five product surfaces — task center, test trees, coverage/flaky views,
/// pipeline overlays, and notebook run history — render rows the user reads
/// directly. The three export surfaces — support/export, CLI/headless, and AI
/// evidence — re-project the same labels so confidence preservation survives the
/// trust boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceLabelSurface {
    /// Task center timeline and task headers.
    TaskCenter,
    /// Test explorer trees and inline test results.
    TestTree,
    /// Coverage overlays and flaky/snapshot views.
    CoverageFlaky,
    /// Pipeline / run-control overlays.
    PipelineOverlay,
    /// Notebook run history and kernel-backed test cells.
    NotebookRunHistory,
    /// Support and release export packets.
    SupportExport,
    /// CLI / headless stable JSON surface.
    CliHeadless,
    /// AI explanations and evidence callouts.
    AiEvidence,
}

impl ConfidenceLabelSurface {
    /// Every claimed surface in stable declaration order.
    pub const ALL: [Self; 8] = [
        Self::TaskCenter,
        Self::TestTree,
        Self::CoverageFlaky,
        Self::PipelineOverlay,
        Self::NotebookRunHistory,
        Self::SupportExport,
        Self::CliHeadless,
        Self::AiEvidence,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaskCenter => "task_center",
            Self::TestTree => "test_tree",
            Self::CoverageFlaky => "coverage_flaky",
            Self::PipelineOverlay => "pipeline_overlay",
            Self::NotebookRunHistory => "notebook_run_history",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
            Self::AiEvidence => "ai_evidence",
        }
    }

    /// True when the surface re-projects labels across the runtime trust boundary.
    pub const fn is_export(self) -> bool {
        matches!(
            self,
            Self::SupportExport | Self::CliHeadless | Self::AiEvidence
        )
    }
}

/// How the authoritative truth for a claim subject changed under arbitration.
///
/// This is the reusable source-quality-change vocabulary the spec requires:
/// desktop, CLI/headless, AI, and support flows all read these tokens instead of
/// inferring a quality shift from rendered text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceQualityChange {
    /// Authority stayed at the same source class with no contested overwrite.
    HeldAuthoritative,
    /// A stronger source took over from a weaker prior authority.
    UpgradedToAuthoritative,
    /// The prior authority dropped and only a weaker/heuristic source remains.
    DowngradedToFallback,
    /// A weaker source attempted to overwrite and was refused; authority held.
    OverwriteBlocked,
    /// A weaker source added inspectable context without changing authority.
    EnrichedWithoutOverwrite,
}

impl SourceQualityChange {
    /// Every source-quality change in stable declaration order.
    pub const ALL: [Self; 5] = [
        Self::HeldAuthoritative,
        Self::UpgradedToAuthoritative,
        Self::DowngradedToFallback,
        Self::OverwriteBlocked,
        Self::EnrichedWithoutOverwrite,
    ];

    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeldAuthoritative => "held_authoritative",
            Self::UpgradedToAuthoritative => "upgraded_to_authoritative",
            Self::DowngradedToFallback => "downgraded_to_fallback",
            Self::OverwriteBlocked => "overwrite_blocked",
            Self::EnrichedWithoutOverwrite => "enriched_without_overwrite",
        }
    }
}

/// Outcome of arbitrating one claim against the authoritative claim for a subject.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverwriteDecision {
    /// The claim is the strongest for the subject and carries authority.
    AcceptedAuthoritative,
    /// A weaker claim that never asserted authority; kept as visible context.
    EnrichedContextOnly,
    /// A weaker claim that attempted to overwrite stronger truth; refused.
    BlockedLowerConfidence,
}

impl OverwriteDecision {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AcceptedAuthoritative => "accepted_authoritative",
            Self::EnrichedContextOnly => "enriched_context_only",
            Self::BlockedLowerConfidence => "blocked_lower_confidence",
        }
    }
}

/// Why a non-authoritative claim could not overwrite the authoritative claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverwriteReason {
    /// The claim's source class ranks below the authority on the priority ladder.
    WeakerSourceClass,
    /// Same source class as the authority but a lower confidence tier.
    LowerConfidenceTier,
    /// The claim never asserted authority, so it enriches rather than overwrites.
    NeverClaimedAuthority,
}

impl OverwriteReason {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WeakerSourceClass => "weaker_source_class",
            Self::LowerConfidenceTier => "lower_confidence_tier",
            Self::NeverClaimedAuthority => "never_claimed_authority",
        }
    }
}

/// Whether a claim subject is a run lifecycle slot or a published artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimSubjectKind {
    /// A lifecycle slot (a queued/started/finished/diagnostic/test position).
    LifecycleSlot,
    /// A published artifact (a coverage report, a build output, a bundle).
    Artifact,
}

impl ClaimSubjectKind {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LifecycleSlot => "lifecycle_slot",
            Self::Artifact => "artifact",
        }
    }
}

/// Numeric weight used to compare confidence levels (higher is stronger).
const fn confidence_weight(confidence: BuildTestEventConfidence) -> u8 {
    match confidence {
        BuildTestEventConfidence::High => 4,
        BuildTestEventConfidence::MediumHigh => 3,
        BuildTestEventConfidence::Medium => 2,
        BuildTestEventConfidence::Low => 1,
    }
}

/// One adapter-confidence label: a source class chip, a confidence chip, and a
/// heuristic-fallback banner.
///
/// The three cues stay separate fields on purpose. Surfaces render the source
/// class and the confidence as distinct chips so neither the guardrail against a
/// single merged badge nor the guardrail against a generic "partial" label can be
/// silently broken.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfidenceLabel {
    /// Source class shown as its own chip.
    pub source_kind: BuildTestEventSourceKind,
    /// Confidence shown as its own chip.
    pub confidence: BuildTestEventConfidence,
    /// True when this label must carry a heuristic-fallback banner.
    pub heuristic_fallback_banner: bool,
    /// Fallback reason behind the banner, present iff the banner is shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<DowngradeReason>,
}

impl ConfidenceLabel {
    /// Builds the label for a source/confidence pair, deriving the banner from
    /// whether the source is a heuristic fallback.
    pub fn new(
        source_kind: BuildTestEventSourceKind,
        confidence: BuildTestEventConfidence,
    ) -> Self {
        let heuristic = source_kind.is_heuristic();
        Self {
            source_kind,
            confidence,
            heuristic_fallback_banner: heuristic,
            fallback_reason: heuristic.then_some(DowngradeReason::HeuristicFallback),
        }
    }

    /// Stable token for the source-class chip.
    pub const fn source_chip(&self) -> &'static str {
        self.source_kind.as_str()
    }

    /// Stable token for the confidence chip.
    pub const fn confidence_chip(&self) -> &'static str {
        self.confidence.as_str()
    }

    /// True when the source is a first-party / negotiated-protocol authority.
    pub const fn is_authoritative(&self) -> bool {
        source_is_authoritative(self.source_kind)
    }

    /// Human-readable banner text, present iff the label is a heuristic fallback.
    pub fn banner_text(&self) -> Option<String> {
        self.fallback_reason.map(|reason| {
            format!(
                "heuristic fallback — {} source, {} confidence ({})",
                self.source_kind.as_str(),
                self.confidence.as_str(),
                reason.as_str(),
            )
        })
    }

    /// True when the banner flag and the fallback reason agree with the source.
    fn banner_consistent(&self) -> bool {
        let heuristic = self.source_kind.is_heuristic();
        self.heuristic_fallback_banner == heuristic && self.fallback_reason.is_some() == heuristic
    }

    /// True when the confidence stays at or below the source's canonical ceiling.
    fn within_ceiling(&self) -> bool {
        confidence_weight(self.confidence)
            <= confidence_weight(canonical_confidence_ceiling(self.source_kind))
    }
}

/// One source's claim about a single subject's confidence label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfidenceClaim {
    /// Stable claim id, unique within the subject.
    pub claim_id: String,
    /// Adapter that produced the claim.
    pub adapter_id: String,
    /// The confidence label this source asserts.
    pub label: ConfidenceLabel,
    /// Adapter priority rank (must equal the source's canonical rank).
    pub priority_rank: u8,
    /// True when this source asserts itself as the authoritative emission.
    pub attempts_overwrite: bool,
    /// Capture time in the producing context.
    pub observed_at: String,
    /// Pointer to the retained raw adapter payload.
    pub raw_payload_ref: String,
}

impl ConfidenceClaim {
    fn is_bound(&self) -> bool {
        !self.claim_id.trim().is_empty()
            && !self.adapter_id.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.raw_payload_ref.trim().is_empty()
    }

    /// Sort key for arbitration: strongest first (lowest rank, then highest
    /// confidence, then lexically smallest claim id for determinism).
    fn strength_key(&self) -> (u8, std::cmp::Reverse<u8>, &str) {
        (
            self.priority_rank,
            std::cmp::Reverse(confidence_weight(self.label.confidence)),
            self.claim_id.as_str(),
        )
    }
}

/// Identity of the lifecycle slot or artifact several sources may describe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimSubject {
    /// Stable subject id.
    pub subject_id: String,
    /// Workspace or workset identity.
    pub workspace_id: String,
    /// Build target, task, test suite, or debug-configuration identity.
    pub target_id: String,
    /// Canonical lifecycle kind the subject sits on.
    pub event_kind: BuildTestEventKind,
    /// Whether the subject is a lifecycle slot or a published artifact.
    pub subject_kind: ClaimSubjectKind,
    /// Source class that was authoritative before this resolution, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_authoritative_source: Option<BuildTestEventSourceKind>,
}

impl ClaimSubject {
    fn is_bound(&self) -> bool {
        !self.subject_id.trim().is_empty()
            && !self.workspace_id.trim().is_empty()
            && !self.target_id.trim().is_empty()
    }
}

/// One challenger's arbitration outcome against the authoritative claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverwriteDecisionRow {
    /// Claim this decision applies to.
    pub claim_id: String,
    /// Arbitration decision.
    pub decision: OverwriteDecision,
    /// Reason, present for every non-authoritative claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<OverwriteReason>,
}

/// Input for one claim subject: the subject plus every claim that described it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimSubjectResolutionInput {
    /// Subject identity.
    pub subject: ClaimSubject,
    /// Every claim that described the subject, in producer order.
    #[serde(default)]
    pub claims: Vec<ConfidenceClaim>,
}

/// A resolved claim subject: its retained claims plus the derived authority,
/// per-claim overwrite decisions, and source-quality change.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimSubjectResolution {
    /// Subject identity.
    pub subject: ClaimSubject,
    /// Every claim that described the subject, retained for lineage.
    #[serde(default)]
    pub claims: Vec<ConfidenceClaim>,
    /// Claim id of the authoritative (winning) claim.
    pub authoritative_claim_id: String,
    /// Source class currently authoritative for the subject.
    pub current_authoritative_source: BuildTestEventSourceKind,
    /// Confidence currently authoritative for the subject.
    pub current_confidence: BuildTestEventConfidence,
    /// Per-claim arbitration outcomes.
    #[serde(default)]
    pub overwrite_decisions: Vec<OverwriteDecisionRow>,
    /// How authority changed relative to the prior authoritative source.
    pub source_quality_change: SourceQualityChange,
}

/// Canonical arbitration of a subject and its claims.
///
/// Returns the authoritative claim id, the authoritative source/confidence, the
/// per-claim decisions, and the source-quality change. Both materialization and
/// validation call this so stored derived fields can never drift unnoticed.
fn canonical_resolution(
    subject: &ClaimSubject,
    claims: &[ConfidenceClaim],
) -> Option<(
    String,
    BuildTestEventSourceKind,
    BuildTestEventConfidence,
    Vec<OverwriteDecisionRow>,
    SourceQualityChange,
)> {
    let authoritative = claims
        .iter()
        .min_by(|a, b| a.strength_key().cmp(&b.strength_key()))?;
    let authoritative_id = authoritative.claim_id.clone();
    let current_source = authoritative.label.source_kind;
    let current_confidence = authoritative.label.confidence;

    let mut decisions = Vec::with_capacity(claims.len());
    let mut any_blocked = false;
    let mut any_enriched = false;
    for claim in claims {
        if claim.claim_id == authoritative_id {
            decisions.push(OverwriteDecisionRow {
                claim_id: claim.claim_id.clone(),
                decision: OverwriteDecision::AcceptedAuthoritative,
                reason: None,
            });
            continue;
        }
        // A non-authoritative claim is weaker on the (rank, confidence) order.
        let reason = if claim.priority_rank > authoritative.priority_rank {
            OverwriteReason::WeakerSourceClass
        } else {
            OverwriteReason::LowerConfidenceTier
        };
        let (decision, reason) = if claim.attempts_overwrite {
            any_blocked = true;
            (OverwriteDecision::BlockedLowerConfidence, reason)
        } else {
            any_enriched = true;
            (
                OverwriteDecision::EnrichedContextOnly,
                OverwriteReason::NeverClaimedAuthority,
            )
        };
        decisions.push(OverwriteDecisionRow {
            claim_id: claim.claim_id.clone(),
            decision,
            reason: Some(reason),
        });
    }

    let change = derive_source_quality_change(
        subject.prior_authoritative_source,
        current_source,
        any_blocked,
        any_enriched,
    );
    Some((
        authoritative_id,
        current_source,
        current_confidence,
        decisions,
        change,
    ))
}

/// Derives the source-quality change from the prior versus current authority.
fn derive_source_quality_change(
    prior: Option<BuildTestEventSourceKind>,
    current: BuildTestEventSourceKind,
    any_blocked: bool,
    any_enriched: bool,
) -> SourceQualityChange {
    if let Some(prior) = prior {
        let prior_rank = canonical_priority_rank(prior);
        let current_rank = canonical_priority_rank(current);
        if current_rank < prior_rank {
            return SourceQualityChange::UpgradedToAuthoritative;
        }
        if current_rank > prior_rank {
            return SourceQualityChange::DowngradedToFallback;
        }
    }
    if any_blocked {
        SourceQualityChange::OverwriteBlocked
    } else if any_enriched {
        SourceQualityChange::EnrichedWithoutOverwrite
    } else {
        SourceQualityChange::HeldAuthoritative
    }
}

impl ClaimSubjectResolution {
    fn resolve(input: ClaimSubjectResolutionInput) -> Self {
        let ClaimSubjectResolutionInput { subject, claims } = input;
        match canonical_resolution(&subject, &claims) {
            Some((authoritative_claim_id, source, confidence, decisions, change)) => Self {
                subject,
                claims,
                authoritative_claim_id,
                current_authoritative_source: source,
                current_confidence: confidence,
                overwrite_decisions: decisions,
                source_quality_change: change,
            },
            None => Self {
                subject,
                claims,
                authoritative_claim_id: String::new(),
                // No claims: fall back to the weakest source so an empty subject
                // cannot masquerade as authoritative native truth.
                current_authoritative_source: BuildTestEventSourceKind::HeuristicParser,
                current_confidence: BuildTestEventConfidence::Low,
                overwrite_decisions: Vec::new(),
                source_quality_change: SourceQualityChange::HeldAuthoritative,
            },
        }
    }

    /// Stable, support-safe explanation derived only from canonical fields.
    pub fn explain(&self) -> String {
        format!(
            "{} ({}) is authoritative {} at {} confidence via {}; source quality {}",
            self.subject.subject_id,
            self.subject.subject_kind.as_str(),
            self.current_authoritative_source.as_str(),
            self.current_confidence.as_str(),
            self.authoritative_claim_id,
            self.source_quality_change.as_str(),
        )
    }
}

/// Binding proving one claimed surface renders the adapter-confidence label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceLabelBinding {
    /// Claimed surface.
    pub surface: ConfidenceLabelSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// True when the surface reads the canonical label, not rendered text.
    pub reads_canonical_label: bool,
    /// True when the surface renders the source class as its own chip.
    pub shows_source_class_chip: bool,
    /// True when the surface renders confidence as its own chip.
    pub shows_confidence_chip: bool,
    /// True when source class and confidence are not collapsed into one badge.
    pub keeps_source_and_confidence_distinct: bool,
    /// True when the surface shows a heuristic-fallback banner for fallback rows.
    pub shows_heuristic_fallback_banner: bool,
    /// True when the surface surfaces the fallback reason from the vocabulary.
    pub shows_fallback_reason: bool,
    /// True when the surface keeps the no-lower-confidence-overwrite decision visible.
    pub shows_overwrite_decision: bool,
    /// True when the surface keeps the full claim lineage inspectable.
    pub keeps_lineage_inspectable: bool,
    /// Count of confidence labels this surface references (derived).
    pub observed_label_count: usize,
}

impl SurfaceLabelBinding {
    /// True when the source-class and confidence chips are not merged.
    fn keeps_chips_distinct(&self) -> bool {
        self.reads_canonical_label
            && self.shows_source_class_chip
            && self.shows_confidence_chip
            && self.keeps_source_and_confidence_distinct
    }

    /// True when fallback rows keep their banner and reason.
    fn keeps_banner(&self) -> bool {
        self.shows_heuristic_fallback_banner && self.shows_fallback_reason
    }

    /// True when overwrite decisions and lineage stay inspectable.
    fn keeps_lineage(&self) -> bool {
        self.shows_overwrite_decision && self.keeps_lineage_inspectable
    }
}

/// Closed validation finding vocabulary for the adapter-confidence audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceAuditFindingKind {
    /// Record kind does not match the frozen tag.
    WrongRecordKind,
    /// Schema version does not match the frozen version.
    WrongSchemaVersion,
    /// Required identity or schema-ref field is missing.
    MissingIdentity,
    /// The audit carries no surface bindings.
    NoSurfaceBindings,
    /// A required surface binding is absent.
    SurfaceBindingMissing,
    /// A binding merges source class and confidence into one badge.
    SurfaceCollapsesSourceAndConfidence,
    /// A binding hides the heuristic-fallback banner or its reason.
    SurfaceHidesFallbackBanner,
    /// A binding drops the overwrite decision or the claim lineage.
    SurfaceDropsLineage,
    /// The audit carries no claim subjects.
    NoClaimSubjects,
    /// A subject carries no claims.
    SubjectHasNoClaims,
    /// A claim has incomplete identity.
    ClaimIdentityIncomplete,
    /// A claim's priority rank disagrees with its source kind.
    ClaimPriorityMismatch,
    /// A claim's confidence exceeds its source's ceiling.
    ClaimConfidenceOverclaim,
    /// A label's banner flag or reason disagrees with its source.
    LabelBannerInconsistent,
    /// The stored authoritative claim or its source/confidence is wrong.
    AuthoritativeClaimMismatch,
    /// A weaker overwrite-attempting claim was not blocked.
    LowerConfidenceOverwriteAccepted,
    /// A stored overwrite decision disagrees with canonical arbitration.
    OverwriteDecisionInconsistent,
    /// A stored source-quality change disagrees with the derived change.
    SourceQualityChangeMismatch,
    /// A decision references a claim that the subject no longer retains.
    LineageDropped,
    /// Stored promotion state disagrees with the derived state.
    PromotionStateMismatch,
}

impl ConfidenceAuditFindingKind {
    /// Stable token used in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::NoSurfaceBindings => "no_surface_bindings",
            Self::SurfaceBindingMissing => "surface_binding_missing",
            Self::SurfaceCollapsesSourceAndConfidence => "surface_collapses_source_and_confidence",
            Self::SurfaceHidesFallbackBanner => "surface_hides_fallback_banner",
            Self::SurfaceDropsLineage => "surface_drops_lineage",
            Self::NoClaimSubjects => "no_claim_subjects",
            Self::SubjectHasNoClaims => "subject_has_no_claims",
            Self::ClaimIdentityIncomplete => "claim_identity_incomplete",
            Self::ClaimPriorityMismatch => "claim_priority_mismatch",
            Self::ClaimConfidenceOverclaim => "claim_confidence_overclaim",
            Self::LabelBannerInconsistent => "label_banner_inconsistent",
            Self::AuthoritativeClaimMismatch => "authoritative_claim_mismatch",
            Self::LowerConfidenceOverwriteAccepted => "lower_confidence_overwrite_accepted",
            Self::OverwriteDecisionInconsistent => "overwrite_decision_inconsistent",
            Self::SourceQualityChangeMismatch => "source_quality_change_mismatch",
            Self::LineageDropped => "lineage_dropped",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the audit validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfidenceAuditValidationFinding {
    /// Closed finding kind.
    pub finding_kind: ConfidenceAuditFindingKind,
    /// Finding severity.
    pub severity: BuildTestInteropFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ConfidenceAuditValidationFinding {
    fn blocker(finding_kind: ConfidenceAuditFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: BuildTestInteropFindingSeverity::Blocker,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`AdapterConfidenceAudit::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterConfidenceAuditInput {
    /// Stable audit id.
    pub audit_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Per-surface label bindings.
    #[serde(default)]
    pub surface_bindings: Vec<SurfaceLabelBinding>,
    /// Claim subjects with their retained claims.
    #[serde(default)]
    pub subjects: Vec<ClaimSubjectResolutionInput>,
}

/// Canonical adapter-confidence audit: surface bindings plus resolved subjects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterConfidenceAudit {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable audit id.
    pub audit_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Adapter-confidence-audit boundary schema ref.
    pub audit_schema_ref: String,
    /// Per-event envelope boundary schema ref.
    pub envelope_schema_ref: String,
    /// Reviewer contract doc ref.
    pub doc_ref: String,
    /// Frozen adapter-policy baseline this lane consumes.
    pub policy_baseline_ref: String,
    /// Per-surface label bindings.
    #[serde(default)]
    pub surface_bindings: Vec<SurfaceLabelBinding>,
    /// Resolved claim subjects.
    #[serde(default)]
    pub subjects: Vec<ClaimSubjectResolution>,
    /// Order-invariant digest of the audit's claims.
    pub label_digest: String,
    /// Derived promotion state.
    pub promotion_state: BuildTestInteropPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ConfidenceAuditValidationFinding>,
}

impl AdapterConfidenceAudit {
    /// Materializes an audit, resolving each subject and recording the validation
    /// findings and derived promotion state.
    pub fn materialize(input: AdapterConfidenceAuditInput) -> Self {
        let subjects: Vec<ClaimSubjectResolution> = input
            .subjects
            .into_iter()
            .map(ClaimSubjectResolution::resolve)
            .collect();
        let total_labels: usize = subjects.iter().map(|s| s.claims.len()).sum();
        let surface_bindings = input
            .surface_bindings
            .into_iter()
            .map(|mut binding| {
                binding.observed_label_count = total_labels;
                binding
            })
            .collect();

        let mut audit = Self {
            record_kind: ADAPTER_CONFIDENCE_AUDIT_RECORD_KIND.to_owned(),
            schema_version: ADAPTER_CONFIDENCE_AUDIT_SCHEMA_VERSION,
            audit_id: input.audit_id,
            generated_at: input.generated_at,
            audit_schema_ref: ADAPTER_CONFIDENCE_AUDIT_SCHEMA_REF.to_owned(),
            envelope_schema_ref: ADAPTER_CONFIDENCE_AUDIT_ENVELOPE_SCHEMA_REF.to_owned(),
            doc_ref: ADAPTER_CONFIDENCE_AUDIT_DOC_REF.to_owned(),
            policy_baseline_ref: ADAPTER_CONFIDENCE_AUDIT_POLICY_BASELINE_REF.to_owned(),
            surface_bindings,
            subjects,
            label_digest: String::new(),
            promotion_state: BuildTestInteropPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        audit.label_digest = audit.compute_label_digest();
        audit.refresh_findings();
        audit
    }

    /// Recomputes the validation findings and promotion state from current state.
    ///
    /// Mutators that write dishonest derived fields call this so a tampered audit
    /// reports the same blockers the validator would raise on disk. The promotion
    /// self-check is skipped here because this method is what sets it.
    pub fn refresh_findings(&mut self) {
        let findings = self.derive_findings(false);
        self.promotion_state = promotion_state_for_findings(&findings);
        self.validation_findings = findings;
    }

    /// Re-validates the audit against the frozen invariants, including that the
    /// stored promotion state agrees with the derived findings.
    pub fn validate(&self) -> Vec<ConfidenceAuditValidationFinding> {
        self.derive_findings(true)
    }

    /// Returns true when no blocker-level finding is present.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    }

    fn derive_findings(&self, check_promotion: bool) -> Vec<ConfidenceAuditValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != ADAPTER_CONFIDENCE_AUDIT_RECORD_KIND {
            findings.push(ConfidenceAuditValidationFinding::blocker(
                ConfidenceAuditFindingKind::WrongRecordKind,
                "audit record kind does not match the frozen tag",
            ));
        }
        if self.schema_version != ADAPTER_CONFIDENCE_AUDIT_SCHEMA_VERSION {
            findings.push(ConfidenceAuditValidationFinding::blocker(
                ConfidenceAuditFindingKind::WrongSchemaVersion,
                "audit schema version does not match the frozen version",
            ));
        }
        if self.audit_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.audit_schema_ref.trim().is_empty()
            || self.envelope_schema_ref.trim().is_empty()
            || self.doc_ref.trim().is_empty()
            || self.policy_baseline_ref.trim().is_empty()
        {
            findings.push(ConfidenceAuditValidationFinding::blocker(
                ConfidenceAuditFindingKind::MissingIdentity,
                "audit is missing an identity or schema-ref field",
            ));
        }
        if self.label_digest != self.compute_label_digest() {
            findings.push(ConfidenceAuditValidationFinding::blocker(
                ConfidenceAuditFindingKind::MissingIdentity,
                "audit label digest does not match the claim history",
            ));
        }

        self.check_surface_bindings(&mut findings);
        self.check_subjects(&mut findings);

        if check_promotion {
            let derived_state = promotion_state_for_findings(&findings);
            if self.promotion_state != derived_state {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::PromotionStateMismatch,
                    "stored promotion state disagrees with the derived findings",
                ));
            }
        }

        findings
    }

    fn check_surface_bindings(&self, findings: &mut Vec<ConfidenceAuditValidationFinding>) {
        if self.surface_bindings.is_empty() {
            findings.push(ConfidenceAuditValidationFinding::blocker(
                ConfidenceAuditFindingKind::NoSurfaceBindings,
                "audit carries no surface bindings",
            ));
            return;
        }
        let present: BTreeSet<ConfidenceLabelSurface> = self
            .surface_bindings
            .iter()
            .map(|binding| binding.surface)
            .collect();
        for surface in ConfidenceLabelSurface::ALL {
            if !present.contains(&surface) {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::SurfaceBindingMissing,
                    format!("surface {} has no label binding", surface.as_str()),
                ));
            }
        }
        for binding in &self.surface_bindings {
            if binding.binding_ref.trim().is_empty() {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::MissingIdentity,
                    format!("binding for {} has no ref", binding.surface.as_str()),
                ));
            }
            if !binding.keeps_chips_distinct() {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::SurfaceCollapsesSourceAndConfidence,
                    format!(
                        "surface {} merges source class and confidence",
                        binding.surface.as_str()
                    ),
                ));
            }
            if !binding.keeps_banner() {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::SurfaceHidesFallbackBanner,
                    format!(
                        "surface {} hides the heuristic-fallback banner",
                        binding.surface.as_str()
                    ),
                ));
            }
            if !binding.keeps_lineage() {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::SurfaceDropsLineage,
                    format!(
                        "surface {} drops the overwrite decision or claim lineage",
                        binding.surface.as_str()
                    ),
                ));
            }
        }
    }

    fn check_subjects(&self, findings: &mut Vec<ConfidenceAuditValidationFinding>) {
        if self.subjects.is_empty() {
            findings.push(ConfidenceAuditValidationFinding::blocker(
                ConfidenceAuditFindingKind::NoClaimSubjects,
                "audit carries no claim subjects",
            ));
            return;
        }
        for resolution in &self.subjects {
            self.check_subject(resolution, findings);
        }
    }

    fn check_subject(
        &self,
        resolution: &ClaimSubjectResolution,
        findings: &mut Vec<ConfidenceAuditValidationFinding>,
    ) {
        let subject_id = resolution.subject.subject_id.as_str();
        if !resolution.subject.is_bound() {
            findings.push(ConfidenceAuditValidationFinding::blocker(
                ConfidenceAuditFindingKind::MissingIdentity,
                format!("subject {subject_id} has incomplete identity"),
            ));
        }
        if resolution.claims.is_empty() {
            findings.push(ConfidenceAuditValidationFinding::blocker(
                ConfidenceAuditFindingKind::SubjectHasNoClaims,
                format!("subject {subject_id} carries no claims"),
            ));
            return;
        }

        for claim in &resolution.claims {
            if !claim.is_bound() {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::ClaimIdentityIncomplete,
                    format!("a claim on {subject_id} has incomplete identity"),
                ));
            }
            if claim.priority_rank != canonical_priority_rank(claim.label.source_kind) {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::ClaimPriorityMismatch,
                    format!(
                        "claim {} on {subject_id} has a non-canonical priority rank",
                        claim.claim_id
                    ),
                ));
            }
            if !claim.label.within_ceiling() {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::ClaimConfidenceOverclaim,
                    format!(
                        "claim {} on {subject_id} overclaims confidence for its source",
                        claim.claim_id
                    ),
                ));
            }
            if !claim.label.banner_consistent() {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::LabelBannerInconsistent,
                    format!(
                        "claim {} on {subject_id} has an inconsistent fallback banner",
                        claim.claim_id
                    ),
                ));
            }
        }

        // Every decision must reference a retained claim; otherwise lineage was
        // dropped to resolve a conflict (an explicit out-of-scope guardrail).
        let claim_ids: BTreeSet<&str> = resolution
            .claims
            .iter()
            .map(|c| c.claim_id.as_str())
            .collect();
        for decision in &resolution.overwrite_decisions {
            if !claim_ids.contains(decision.claim_id.as_str()) {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::LineageDropped,
                    format!(
                        "decision on {subject_id} references dropped claim {}",
                        decision.claim_id
                    ),
                ));
            }
        }

        let Some((auth_id, source, confidence, canonical_decisions, change)) =
            canonical_resolution(&resolution.subject, &resolution.claims)
        else {
            return;
        };

        if resolution.authoritative_claim_id != auth_id
            || resolution.current_authoritative_source != source
            || resolution.current_confidence != confidence
        {
            findings.push(ConfidenceAuditValidationFinding::blocker(
                ConfidenceAuditFindingKind::AuthoritativeClaimMismatch,
                format!("subject {subject_id} names the wrong authoritative claim"),
            ));
        }

        // Compare stored decisions against canonical arbitration. A weaker,
        // overwrite-attempting claim that is not blocked is the core invariant
        // breach; any other mismatch is a generic inconsistency.
        for canonical in &canonical_decisions {
            let stored = resolution
                .overwrite_decisions
                .iter()
                .find(|row| row.claim_id == canonical.claim_id);
            let Some(stored) = stored else {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::OverwriteDecisionInconsistent,
                    format!(
                        "subject {subject_id} is missing a decision for claim {}",
                        canonical.claim_id
                    ),
                ));
                continue;
            };
            if stored.decision == canonical.decision && stored.reason == canonical.reason {
                continue;
            }
            if canonical.decision == OverwriteDecision::BlockedLowerConfidence
                && stored.decision != OverwriteDecision::BlockedLowerConfidence
            {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::LowerConfidenceOverwriteAccepted,
                    format!(
                        "subject {subject_id} let weaker claim {} overwrite authoritative truth",
                        canonical.claim_id
                    ),
                ));
            } else {
                findings.push(ConfidenceAuditValidationFinding::blocker(
                    ConfidenceAuditFindingKind::OverwriteDecisionInconsistent,
                    format!(
                        "subject {subject_id} stores a non-canonical decision for claim {}",
                        canonical.claim_id
                    ),
                ));
            }
        }

        if resolution.source_quality_change != change {
            findings.push(ConfidenceAuditValidationFinding::blocker(
                ConfidenceAuditFindingKind::SourceQualityChangeMismatch,
                format!("subject {subject_id} stores a non-canonical source-quality change"),
            ));
        }
    }

    fn compute_label_digest(&self) -> String {
        let mut tokens: Vec<String> = Vec::new();
        for resolution in &self.subjects {
            for claim in &resolution.claims {
                tokens.push(format!(
                    "{}|{}|{}|{}",
                    resolution.subject.subject_id,
                    claim.claim_id,
                    claim.label.source_kind.as_str(),
                    claim.label.confidence.as_str(),
                ));
            }
        }
        tokens.sort();
        let refs: Vec<&str> = tokens.iter().map(String::as_str).collect();
        label_digest(&refs)
    }

    /// Builds an export-safe support packet carrying the exact audit.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> AdapterConfidenceAuditSupportExport {
        AdapterConfidenceAuditSupportExport {
            record_kind: ADAPTER_CONFIDENCE_AUDIT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ADAPTER_CONFIDENCE_AUDIT_SCHEMA_VERSION,
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            audit_id_ref: self.audit_id.clone(),
            label_digest: self.label_digest.clone(),
            audit: self.clone(),
        }
    }

    /// Builds the CLI/headless stable view consumers read without parsing logs.
    pub fn cli_headless_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> AdapterConfidenceCliHeadlessView {
        let rows = self.label_rows();
        AdapterConfidenceCliHeadlessView {
            record_kind: ADAPTER_CONFIDENCE_AUDIT_CLI_HEADLESS_RECORD_KIND.to_owned(),
            schema_version: ADAPTER_CONFIDENCE_AUDIT_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            audit_id_ref: self.audit_id.clone(),
            label_digest: self.label_digest.clone(),
            rows,
        }
    }

    /// Builds the AI evidence view that keeps source, confidence, banner,
    /// decision, and lineage inspectable for evidence callouts.
    pub fn ai_evidence_view(
        &self,
        view_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> AdapterConfidenceAiEvidenceView {
        let subjects = self
            .subjects
            .iter()
            .map(AiEvidenceSubjectRow::from_resolution)
            .collect();
        AdapterConfidenceAiEvidenceView {
            record_kind: ADAPTER_CONFIDENCE_AUDIT_AI_EVIDENCE_RECORD_KIND.to_owned(),
            schema_version: ADAPTER_CONFIDENCE_AUDIT_SCHEMA_VERSION,
            view_id: view_id.into(),
            generated_at: generated_at.into(),
            audit_id_ref: self.audit_id.clone(),
            label_digest: self.label_digest.clone(),
            subjects,
        }
    }

    fn label_rows(&self) -> Vec<AdapterConfidenceCliHeadlessRow> {
        let mut rows: Vec<AdapterConfidenceCliHeadlessRow> = Vec::new();
        for resolution in &self.subjects {
            let decision_for = |claim_id: &str| {
                resolution
                    .overwrite_decisions
                    .iter()
                    .find(|row| row.claim_id == claim_id)
            };
            for claim in &resolution.claims {
                let decision = decision_for(&claim.claim_id);
                rows.push(AdapterConfidenceCliHeadlessRow {
                    subject_id: resolution.subject.subject_id.clone(),
                    subject_kind: resolution.subject.subject_kind.as_str().to_owned(),
                    claim_id: claim.claim_id.clone(),
                    source_kind: claim.label.source_chip().to_owned(),
                    confidence: claim.label.confidence_chip().to_owned(),
                    heuristic_fallback_banner: claim.label.heuristic_fallback_banner,
                    fallback_reason: claim.label.fallback_reason.map(|r| r.as_str().to_owned()),
                    overwrite_decision: decision
                        .map(|row| row.decision.as_str().to_owned())
                        .unwrap_or_default(),
                    overwrite_reason: decision
                        .and_then(|row| row.reason)
                        .map(|r| r.as_str().to_owned()),
                    is_authoritative: claim.claim_id == resolution.authoritative_claim_id,
                    source_quality_change: resolution.source_quality_change.as_str().to_owned(),
                    raw_payload_ref: claim.raw_payload_ref.clone(),
                });
            }
        }
        rows.sort_by(|a, b| {
            (a.subject_id.as_str(), a.claim_id.as_str())
                .cmp(&(b.subject_id.as_str(), b.claim_id.as_str()))
        });
        rows
    }

    /// Sorted, distinct surface tokens present in the bindings.
    pub fn surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for binding in &self.surface_bindings {
            set.insert(binding.surface);
        }
        set.into_iter()
            .map(ConfidenceLabelSurface::as_str)
            .collect()
    }

    /// Sorted, distinct source-kind tokens present in the claim history.
    pub fn source_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for resolution in &self.subjects {
            for claim in &resolution.claims {
                set.insert(claim.label.source_kind);
            }
        }
        set.into_iter()
            .map(BuildTestEventSourceKind::as_str)
            .collect()
    }

    /// Sorted, distinct source-quality-change tokens across the subjects.
    pub fn source_quality_change_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for resolution in &self.subjects {
            set.insert(resolution.source_quality_change);
        }
        set.into_iter().map(SourceQualityChange::as_str).collect()
    }

    /// Sorted, distinct overwrite-decision tokens across the subjects.
    pub fn overwrite_decision_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for resolution in &self.subjects {
            for decision in &resolution.overwrite_decisions {
                set.insert(decision.decision);
            }
        }
        set.into_iter().map(OverwriteDecision::as_str).collect()
    }

    /// Sorted, distinct fallback-reason tokens on the labels.
    pub fn fallback_reason_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for resolution in &self.subjects {
            for claim in &resolution.claims {
                if let Some(reason) = claim.label.fallback_reason {
                    set.insert(reason);
                }
            }
        }
        set.into_iter().map(DowngradeReason::as_str).collect()
    }

    /// Compact, support-safe one-line summaries of each subject's resolution.
    pub fn compact_lines(&self) -> Vec<String> {
        self.subjects
            .iter()
            .map(ClaimSubjectResolution::explain)
            .collect()
    }
}

/// CLI/headless row: one confidence label projected for a stable JSON surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterConfidenceCliHeadlessRow {
    /// Subject the claim describes.
    pub subject_id: String,
    /// Subject kind token.
    pub subject_kind: String,
    /// Claim id.
    pub claim_id: String,
    /// Source-class chip token.
    pub source_kind: String,
    /// Confidence chip token.
    pub confidence: String,
    /// True when the row carries a heuristic-fallback banner.
    pub heuristic_fallback_banner: bool,
    /// Fallback reason token, present iff the banner is shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<String>,
    /// Overwrite decision token.
    pub overwrite_decision: String,
    /// Overwrite reason token, present for non-authoritative claims.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overwrite_reason: Option<String>,
    /// True when the claim is authoritative for its subject.
    pub is_authoritative: bool,
    /// Source-quality-change token for the subject.
    pub source_quality_change: String,
    /// Retained raw-payload reference.
    pub raw_payload_ref: String,
}

/// CLI/headless stable view of the audit's confidence labels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterConfidenceCliHeadlessView {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// View timestamp.
    pub generated_at: String,
    /// Audit id ref.
    pub audit_id_ref: String,
    /// Order-invariant label digest of the source audit.
    pub label_digest: String,
    /// Rows in deterministic order.
    #[serde(default)]
    pub rows: Vec<AdapterConfidenceCliHeadlessRow>,
}

impl AdapterConfidenceCliHeadlessView {
    /// True when every row keeps the source class and confidence as two cues and
    /// names a non-empty overwrite decision.
    pub fn every_row_keeps_label(&self) -> bool {
        self.rows.iter().all(|row| {
            !row.source_kind.trim().is_empty()
                && !row.confidence.trim().is_empty()
                && !row.overwrite_decision.trim().is_empty()
                && (!row.heuristic_fallback_banner || row.fallback_reason.is_some())
        })
    }
}

/// AI-evidence projection of one subject's resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvidenceSubjectRow {
    /// Subject id.
    pub subject_id: String,
    /// Subject kind token.
    pub subject_kind: String,
    /// Authoritative claim id.
    pub authoritative_claim_id: String,
    /// Authoritative source-class token.
    pub authoritative_source: String,
    /// Authoritative confidence token.
    pub authoritative_confidence: String,
    /// Source-quality-change token.
    pub source_quality_change: String,
    /// Retained claim lineage with per-claim decisions.
    #[serde(default)]
    pub claims: Vec<AiEvidenceClaimRow>,
    /// Support-safe explanation derived from canonical fields.
    pub explanation: String,
}

impl AiEvidenceSubjectRow {
    fn from_resolution(resolution: &ClaimSubjectResolution) -> Self {
        let claims = resolution
            .claims
            .iter()
            .map(|claim| {
                let decision = resolution
                    .overwrite_decisions
                    .iter()
                    .find(|row| row.claim_id == claim.claim_id);
                AiEvidenceClaimRow {
                    claim_id: claim.claim_id.clone(),
                    source_kind: claim.label.source_chip().to_owned(),
                    confidence: claim.label.confidence_chip().to_owned(),
                    heuristic_fallback_banner: claim.label.heuristic_fallback_banner,
                    fallback_reason: claim.label.fallback_reason.map(|r| r.as_str().to_owned()),
                    overwrite_decision: decision
                        .map(|row| row.decision.as_str().to_owned())
                        .unwrap_or_default(),
                    overwrite_reason: decision
                        .and_then(|row| row.reason)
                        .map(|r| r.as_str().to_owned()),
                    raw_payload_ref: claim.raw_payload_ref.clone(),
                }
            })
            .collect();
        Self {
            subject_id: resolution.subject.subject_id.clone(),
            subject_kind: resolution.subject.subject_kind.as_str().to_owned(),
            authoritative_claim_id: resolution.authoritative_claim_id.clone(),
            authoritative_source: resolution.current_authoritative_source.as_str().to_owned(),
            authoritative_confidence: resolution.current_confidence.as_str().to_owned(),
            source_quality_change: resolution.source_quality_change.as_str().to_owned(),
            claims,
            explanation: resolution.explain(),
        }
    }
}

/// AI-evidence projection of one retained claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiEvidenceClaimRow {
    /// Claim id.
    pub claim_id: String,
    /// Source-class token.
    pub source_kind: String,
    /// Confidence token.
    pub confidence: String,
    /// True when the claim carries a heuristic-fallback banner.
    pub heuristic_fallback_banner: bool,
    /// Fallback reason token, present iff the banner is shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_reason: Option<String>,
    /// Overwrite decision token.
    pub overwrite_decision: String,
    /// Overwrite reason token, present for non-authoritative claims.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overwrite_reason: Option<String>,
    /// Retained raw-payload reference.
    pub raw_payload_ref: String,
}

/// AI evidence view of the audit: per-subject resolutions with retained lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterConfidenceAiEvidenceView {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable view id.
    pub view_id: String,
    /// View timestamp.
    pub generated_at: String,
    /// Audit id ref.
    pub audit_id_ref: String,
    /// Order-invariant label digest of the source audit.
    pub label_digest: String,
    /// Per-subject evidence rows.
    #[serde(default)]
    pub subjects: Vec<AiEvidenceSubjectRow>,
}

impl AdapterConfidenceAiEvidenceView {
    /// True when every subject keeps its full claim lineage inspectable.
    pub fn keeps_lineage(&self) -> bool {
        self.subjects.iter().all(|subject| {
            !subject.claims.is_empty()
                && subject.claims.iter().all(|claim| {
                    !claim.source_kind.trim().is_empty()
                        && !claim.overwrite_decision.trim().is_empty()
                        && !claim.raw_payload_ref.trim().is_empty()
                })
        })
    }
}

/// Export-safe support packet carrying the exact audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdapterConfidenceAuditSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Audit id ref.
    pub audit_id_ref: String,
    /// Order-invariant label digest of the source audit.
    pub label_digest: String,
    /// The exact audit.
    pub audit: AdapterConfidenceAudit,
}

impl AdapterConfidenceAuditSupportExport {
    /// True when the audit validates clean, so the export preserves confidence
    /// semantics across the trust boundary without leaking raw payload bodies.
    pub fn is_export_safe(&self) -> bool {
        self.audit.is_stable()
            && self.audit_id_ref == self.audit.audit_id
            && self.label_digest == self.audit.label_digest
    }
}

/// Order-stable FNV-1a 64-bit digest of a sequence of label tokens.
fn label_digest(tokens_in_order: &[&str]) -> String {
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash = OFFSET;
    for token in tokens_in_order {
        for byte in token.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(PRIME);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

fn promotion_state_for_findings(
    findings: &[ConfidenceAuditValidationFinding],
) -> BuildTestInteropPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Blocker)
    {
        BuildTestInteropPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == BuildTestInteropFindingSeverity::Warning)
    {
        BuildTestInteropPromotionState::NarrowedBelowStable
    } else {
        BuildTestInteropPromotionState::Stable
    }
}

/// Builds the canonical stable adapter-confidence audit input.
pub fn current_stable_adapter_confidence_audit_input() -> AdapterConfidenceAuditInput {
    AdapterConfidenceAuditInput {
        audit_id: ADAPTER_CONFIDENCE_AUDIT_ID.to_owned(),
        generated_at: "2026-06-17T00:00:00Z".to_owned(),
        surface_bindings: canonical_surface_bindings(),
        subjects: canonical_subjects(),
    }
}

/// Materializes the canonical stable adapter-confidence audit.
pub fn seeded_adapter_confidence_audit() -> AdapterConfidenceAudit {
    AdapterConfidenceAudit::materialize(current_stable_adapter_confidence_audit_input())
}

/// Validates an audit and returns an `Ok(())` / findings result.
pub fn validate_adapter_confidence_audit(
    audit: &AdapterConfidenceAudit,
) -> Result<(), Vec<ConfidenceAuditValidationFinding>> {
    let findings = audit.validate();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn canonical_surface_bindings() -> Vec<SurfaceLabelBinding> {
    ConfidenceLabelSurface::ALL
        .into_iter()
        .map(|surface| SurfaceLabelBinding {
            surface,
            binding_ref: format!("binding:tooling:m5:adapter-confidence:{}", surface.as_str()),
            reads_canonical_label: true,
            shows_source_class_chip: true,
            shows_confidence_chip: true,
            keeps_source_and_confidence_distinct: true,
            shows_heuristic_fallback_banner: true,
            shows_fallback_reason: true,
            shows_overwrite_decision: true,
            keeps_lineage_inspectable: true,
            // Overwritten by `materialize`.
            observed_label_count: 0,
        })
        .collect()
}

fn claim(
    claim_id: &str,
    adapter: &str,
    source: BuildTestEventSourceKind,
    confidence: BuildTestEventConfidence,
    attempts_overwrite: bool,
) -> ConfidenceClaim {
    ConfidenceClaim {
        claim_id: claim_id.to_owned(),
        adapter_id: format!("adapter:{adapter}"),
        label: ConfidenceLabel::new(source, confidence),
        priority_rank: canonical_priority_rank(source),
        attempts_overwrite,
        observed_at: "2026-06-17T00:00:00Z".to_owned(),
        raw_payload_ref: format!("raw:{claim_id}"),
    }
}

fn subject(
    subject_id: &str,
    target_id: &str,
    event_kind: BuildTestEventKind,
    subject_kind: ClaimSubjectKind,
    prior: Option<BuildTestEventSourceKind>,
    claims: Vec<ConfidenceClaim>,
) -> ClaimSubjectResolutionInput {
    ClaimSubjectResolutionInput {
        subject: ClaimSubject {
            subject_id: subject_id.to_owned(),
            workspace_id: "workspace:checkout".to_owned(),
            target_id: target_id.to_owned(),
            event_kind,
            subject_kind,
            prior_authoritative_source: prior,
        },
        claims,
    }
}

fn canonical_subjects() -> Vec<ClaimSubjectResolutionInput> {
    use BuildTestEventConfidence::{High, Low, MediumHigh};
    use BuildTestEventKind::{
        ArtifactPublished, DiagnosticEmitted, TaskFinished, TestCaseFinished,
    };
    use BuildTestEventSourceKind::{HeuristicParser, Native, StructuredOutput};
    use ClaimSubjectKind::{Artifact, LifecycleSlot};

    vec![
        // Test finish: native truth holds; a heuristic re-report of the same slot
        // attempts to overwrite it and is blocked — the core no-lower-confidence
        // overwrite case.
        subject(
            "subject:test:finish",
            "target:checkout:test",
            TestCaseFinished,
            LifecycleSlot,
            Some(Native),
            vec![
                claim(
                    "claim:test:finish:native",
                    "aureline-test",
                    Native,
                    High,
                    true,
                ),
                claim(
                    "claim:test:finish:heuristic",
                    "problem-matcher",
                    HeuristicParser,
                    Low,
                    true,
                ),
            ],
        ),
        // Coverage artifact: imported structured output is authoritative; a
        // heuristic adds context without ever asserting authority.
        subject(
            "subject:coverage:artifact",
            "target:checkout:coverage",
            ArtifactPublished,
            Artifact,
            Some(StructuredOutput),
            vec![
                claim(
                    "claim:coverage:structured",
                    "lcov-import",
                    StructuredOutput,
                    MediumHigh,
                    true,
                ),
                claim(
                    "claim:coverage:heuristic",
                    "coverage-scraper",
                    HeuristicParser,
                    Low,
                    false,
                ),
            ],
        ),
        // Pipeline diagnostic: the native source dropped this run, so a heuristic
        // is authoritative-but-flagged and the quality visibly downgrades.
        subject(
            "subject:pipeline:diagnostic",
            "target:checkout:pipeline",
            DiagnosticEmitted,
            LifecycleSlot,
            Some(Native),
            vec![claim(
                "claim:pipeline:heuristic",
                "problem-matcher",
                HeuristicParser,
                Low,
                true,
            )],
        ),
        // Task finished: a native source took over from a prior heuristic
        // authority; the old heuristic is retained as inspectable context.
        subject(
            "subject:task:finished",
            "target:checkout:build",
            TaskFinished,
            LifecycleSlot,
            Some(HeuristicParser),
            vec![
                claim("claim:task:native", "aureline-task", Native, High, true),
                claim(
                    "claim:task:heuristic",
                    "problem-matcher",
                    HeuristicParser,
                    Low,
                    false,
                ),
            ],
        ),
        // Notebook test: native truth with no challenger; authority holds steady.
        subject(
            "subject:notebook:test",
            "target:checkout:notebook",
            TestCaseFinished,
            LifecycleSlot,
            Some(Native),
            vec![claim(
                "claim:notebook:native",
                "aureline-notebook",
                Native,
                High,
                true,
            )],
        ),
    ]
}
