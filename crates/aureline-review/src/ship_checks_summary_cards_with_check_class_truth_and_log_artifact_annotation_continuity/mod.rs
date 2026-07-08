//! Checks-summary cards with required/optional/skipped/suppressed/timed-out/
//! stale/not-evaluated-here truth plus direct log/artifact/annotation continuity.
//!
//! This module narrows the `checks_summary_card` component frozen in
//! [`crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix`]
//! into an implemented, export-safe card contract. Every
//! [`ChecksSummaryCard`] keeps each check's disposition explicit — required,
//! optional, skipped, suppressed, timed out, stale, or not-evaluated-here — so the
//! reader never sees a single green/red gate number where richer per-check
//! evidence exists. Each check links to its logs, artifacts, and annotations, and
//! every one of those links carries the originating review and check identity, so
//! navigation stays anchored across open, reopen, and export paths.
//!
//! Provider outage and stale-sync degradations are preserved rather than collapsed:
//! a degraded provider still lets ordinary triage continue from the local diff,
//! cached annotations, or exported evidence via an explicit local-continue path,
//! and an unreachable provider keeps its browser-handoff boundary explicit. The
//! whole review lane is never collapsed just because one provider went stale.
//!
//! The same card contract is reused by the review workspace, review lists,
//! companion queues, handoff packets, CLI/headless output, diagnostics, Help/About,
//! and support exports, so there is no hidden provider-specific meaning. The
//! provider-freshness vocabulary is reused directly from the frozen matrix
//! ([`M5ReviewComponentStaleProviderState`]) so freshness downgrades read the same
//! everywhere.
//!
//! The packet references upstream review-workspace, pipeline-run, log-view,
//! artifact-card, and annotation contracts by id rather than embedding their
//! content. Raw check logs, raw artifact bytes, raw annotation payloads,
//! credentials, and live provider responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-checks-summary-card.schema.json`](../../../../schemas/ui/m5-checks-summary-card.schema.json).
//! The contract doc is
//! [`docs/review/m5/ship_checks_summary_cards_with_check_class_truth_and_log_artifact_annotation_continuity.md`](../../../../docs/review/m5/ship_checks_summary_cards_with_check_class_truth_and_log_artifact_annotation_continuity.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-checks-summary-cards/`](../../../../fixtures/ui/m5-checks-summary-cards/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix::M5ReviewComponentStaleProviderState;

/// Stable record-kind tag carried by [`ChecksSummaryCardPacket`].
pub const CHECKS_SUMMARY_CARD_RECORD_KIND: &str =
    "checks_summary_card_check_class_and_evidence_continuity";

/// Schema version for checks-summary card records.
pub const CHECKS_SUMMARY_CARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const CHECKS_SUMMARY_CARD_SCHEMA_REF: &str = "schemas/ui/m5-checks-summary-card.schema.json";

/// Repo-relative path of the checks-summary card contract doc.
pub const CHECKS_SUMMARY_CARD_DOC_REF: &str =
    "docs/review/m5/ship_checks_summary_cards_with_check_class_truth_and_log_artifact_annotation_continuity.md";

/// Repo-relative path of the frozen component matrix this card implements.
pub const CHECKS_SUMMARY_CARD_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-review-request-check-queue-component-matrix.schema.json";

/// Repo-relative path of the review-workspace contract that supplies review identity.
pub const CHECKS_SUMMARY_CARD_REVIEW_WORKSPACE_CONTRACT_REF: &str =
    "schemas/review/review_workspace.schema.json";

/// Repo-relative path of the pipeline-run contract that supplies check identity.
pub const CHECKS_SUMMARY_CARD_PIPELINE_RUN_CONTRACT_REF: &str =
    "schemas/ci/pipeline_run_row.schema.json";

/// Repo-relative path of the log-view contract that anchors log links.
pub const CHECKS_SUMMARY_CARD_LOG_VIEW_CONTRACT_REF: &str = "schemas/ci/log_view.schema.json";

/// Repo-relative path of the artifact-card contract that anchors artifact links.
pub const CHECKS_SUMMARY_CARD_ARTIFACT_CARD_CONTRACT_REF: &str =
    "schemas/ci/pipeline_artifact_card.schema.json";

/// Repo-relative path of the annotation-row contract that anchors annotation links.
pub const CHECKS_SUMMARY_CARD_ANNOTATION_CONTRACT_REF: &str =
    "schemas/ci/pipeline_annotation_row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const CHECKS_SUMMARY_CARD_FIXTURE_DIR: &str = "fixtures/ui/m5-checks-summary-cards";

/// Repo-relative path of the checked support-export artifact.
pub const CHECKS_SUMMARY_CARD_ARTIFACT_REF: &str =
    "artifacts/review/m5/ship_checks_summary_cards_with_check_class_truth_and_log_artifact_annotation_continuity/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const CHECKS_SUMMARY_CARD_SUMMARY_REF: &str =
    "artifacts/review/m5/ship_checks_summary_cards_with_check_class_truth_and_log_artifact_annotation_continuity.md";

/// Disposition class of a single check on a checks-summary card.
///
/// This is the core honesty axis. A card must let the reader tell required,
/// optional, skipped, suppressed, timed-out, stale, and not-evaluated-here checks
/// apart from the card alone, rather than flattening every check into one pass/fail
/// number.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckClass {
    /// A required check that gates merge readiness.
    Required,
    /// An optional check that does not gate merge readiness.
    Optional,
    /// A check that was intentionally skipped for this change.
    Skipped,
    /// A check whose result was suppressed by policy or configuration.
    Suppressed,
    /// A check that timed out before returning a verdict.
    TimedOut,
    /// A check whose provider-backed result is stale relative to the head it gates.
    Stale,
    /// A check that is not evaluated in this local/offline context.
    NotEvaluatedHere,
}

impl CheckClass {
    /// Every check class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Required,
        Self::Optional,
        Self::Skipped,
        Self::Suppressed,
        Self::TimedOut,
        Self::Stale,
        Self::NotEvaluatedHere,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Optional => "optional",
            Self::Skipped => "skipped",
            Self::Suppressed => "suppressed",
            Self::TimedOut => "timed_out",
            Self::Stale => "stale",
            Self::NotEvaluatedHere => "not_evaluated_here",
        }
    }

    /// Whether a check of this class gates merge readiness.
    ///
    /// Only a required check gates; every other class is informational and must
    /// never be silently folded into a pass/fail gate number.
    pub const fn is_gating(self) -> bool {
        matches!(self, Self::Required)
    }

    /// Whether a check of this class must carry an explicit evaluation reason.
    ///
    /// A required or optional check has a plain pass/fail verdict, but every
    /// anomalous class must explain why it is not a plain verdict so it is never
    /// collapsed into one.
    pub const fn needs_evaluation_reason(self) -> bool {
        matches!(
            self,
            Self::Skipped
                | Self::Suppressed
                | Self::TimedOut
                | Self::Stale
                | Self::NotEvaluatedHere
        )
    }

    /// Whether a check of this class must not be shown as a plain pass.
    ///
    /// Stale and not-evaluated-here checks are the ones most at risk of being read
    /// as passing; they must always stay visibly distinct.
    pub const fn never_shown_as_pass(self) -> bool {
        matches!(self, Self::Stale | Self::NotEvaluatedHere | Self::TimedOut)
    }
}

/// Kind of evidence a check links to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckEvidenceKind {
    /// A check log stream.
    Log,
    /// A build/test artifact.
    Artifact,
    /// An inline code annotation.
    Annotation,
}

impl CheckEvidenceKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Log => "log",
            Self::Artifact => "artifact",
            Self::Annotation => "annotation",
        }
    }
}

/// A direct action a checks-summary card or check exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecksSummaryAction {
    /// Open the check log inside the Aureline workspace.
    OpenLog,
    /// Open a check artifact inside the workspace.
    OpenArtifact,
    /// Open an inline annotation inside the workspace.
    OpenAnnotation,
    /// Rerun the check where reruns are allowed.
    RerunCheck,
    /// Cancel the check where cancellation is allowed.
    CancelCheck,
    /// Continue reviewing locally while provider freshness is degraded.
    ContinueLocalReview,
    /// Hand off to the provider in the browser.
    OpenProviderInBrowser,
    /// Export the check evidence packet.
    ExportCheckEvidence,
}

impl ChecksSummaryAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenLog => "open_log",
            Self::OpenArtifact => "open_artifact",
            Self::OpenAnnotation => "open_annotation",
            Self::RerunCheck => "rerun_check",
            Self::CancelCheck => "cancel_check",
            Self::ContinueLocalReview => "continue_local_review",
            Self::OpenProviderInBrowser => "open_provider_in_browser",
            Self::ExportCheckEvidence => "export_check_evidence",
        }
    }

    /// Whether this action stays inside the product rather than forcing raw-provider navigation.
    pub const fn is_in_product(self) -> bool {
        !matches!(self, Self::OpenProviderInBrowser)
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecksSummaryCardDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Provider-backed freshness has gone stale relative to what it gates.
    ProviderFreshnessStale,
    /// A required check timed out before returning a verdict.
    CheckTimedOut,
    /// A check result was suppressed by policy.
    CheckEvaluationSuppressed,
    /// Browser handoff for provider deep links is unavailable.
    BrowserHandoffUnavailable,
    /// Card trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified checks-summary boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl ChecksSummaryCardDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderFreshnessStale,
        Self::CheckTimedOut,
        Self::CheckEvaluationSuppressed,
        Self::BrowserHandoffUnavailable,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderFreshnessStale => "provider_freshness_stale",
            Self::CheckTimedOut => "check_timed_out",
            Self::CheckEvaluationSuppressed => "check_evaluation_suppressed",
            Self::BrowserHandoffUnavailable => "browser_handoff_unavailable",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must reuse this card contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecksSummaryCardConsumerSurface {
    /// Review workspace.
    ReviewWorkspace,
    /// Review list.
    ReviewList,
    /// Browser companion queue.
    CompanionQueue,
    /// Browser/provider handoff packet.
    HandoffPacket,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
    /// Checks-summary drawer.
    ChecksSummaryDrawer,
}

impl ChecksSummaryCardConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ReviewWorkspace,
        Self::ReviewList,
        Self::CompanionQueue,
        Self::HandoffPacket,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
        Self::ChecksSummaryDrawer,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewWorkspace => "review_workspace",
            Self::ReviewList => "review_list",
            Self::CompanionQueue => "companion_queue",
            Self::HandoffPacket => "handoff_packet",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
            Self::ChecksSummaryDrawer => "checks_summary_drawer",
        }
    }
}

/// A single evidence link a check exposes.
///
/// Every link must preserve both the originating review and the originating check
/// identity so log/artifact/annotation navigation stays anchored across open,
/// reopen, and export paths.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckEvidenceLink {
    /// Kind of evidence linked.
    pub kind: CheckEvidenceKind,
    /// Human-readable link label.
    pub label: String,
    /// Originating review identity carried by the link.
    pub review_id_ref: String,
    /// Originating check identity carried by the link.
    pub check_id_ref: String,
}

impl CheckEvidenceLink {
    /// Whether this link preserves both review and check identity.
    pub fn preserves_identity(&self) -> bool {
        !self.review_id_ref.trim().is_empty() && !self.check_id_ref.trim().is_empty()
    }
}

/// One check entry on a checks-summary card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CheckEntry {
    /// Stable check id.
    pub check_id: String,
    /// Human-readable check name.
    pub check_name_label: String,
    /// Disposition class of the check.
    pub check_class: CheckClass,
    /// Evaluation reason; required and non-empty for anomalous classes.
    pub evaluation_reason: String,
    /// Evidence links (logs, artifacts, annotations) the check exposes.
    pub evidence_links: Vec<CheckEvidenceLink>,
    /// Direct actions the check exposes, in display order.
    pub actions: Vec<ChecksSummaryAction>,
}

impl CheckEntry {
    /// Whether this check exposes at least one in-product action for ordinary triage.
    pub fn has_in_product_action(&self) -> bool {
        self.actions.iter().any(|action| action.is_in_product())
    }
}

/// Disclosures a card must carry, derived from its provider freshness and evidence depth.
///
/// This is the resolver output that anchors the honesty invariants: a card with
/// richer per-check evidence never flattens to a single pass/fail verdict, a
/// degraded provider preserves a local-continue path, and an unreachable provider
/// keeps its browser-handoff boundary explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChecksSummaryCardDisclosure {
    /// Whether the card must not collapse into a single pass/fail verdict.
    pub must_not_flatten_to_single_verdict: bool,
    /// Whether the card must carry an explicit browser-handoff boundary.
    pub needs_browser_handoff_boundary: bool,
    /// Whether the card must preserve a local-continue fallback.
    pub needs_local_continue_fallback: bool,
}

/// Resolves the disclosures a card must carry from its provider freshness and evidence depth.
///
/// The distinction is derived, never asserted directly: a card that carries richer
/// per-check evidence must not be flattened into a single verdict, a stale,
/// unreachable, conflicting, or local-only-continued provider always forces a
/// local-continue fallback, and an unreachable provider always forces an explicit
/// handoff boundary. Stale-sync therefore degrades one card without collapsing the
/// whole review lane.
pub fn resolve_checks_summary_card_disclosure(
    provider_freshness: M5ReviewComponentStaleProviderState,
    has_richer_evidence: bool,
) -> ChecksSummaryCardDisclosure {
    let freshness_forces_local_continue = matches!(
        provider_freshness,
        M5ReviewComponentStaleProviderState::ProviderStale
            | M5ReviewComponentStaleProviderState::ProviderUnreachable
            | M5ReviewComponentStaleProviderState::ProviderConflict
            | M5ReviewComponentStaleProviderState::LocalOnlyContinuation
    );
    let freshness_forces_handoff = matches!(
        provider_freshness,
        M5ReviewComponentStaleProviderState::ProviderUnreachable
    );
    ChecksSummaryCardDisclosure {
        must_not_flatten_to_single_verdict: has_richer_evidence,
        needs_browser_handoff_boundary: freshness_forces_handoff,
        needs_local_continue_fallback: freshness_forces_local_continue,
    }
}

/// One checks-summary card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChecksSummaryCard {
    /// Stable card id.
    pub card_id: String,
    /// Human-readable originating review identity.
    pub review_id_label: String,
    /// Human-readable provider or local-object identity.
    pub provider_identity_label: String,
    /// Provider-freshness state, reused from the frozen component matrix.
    pub provider_freshness: M5ReviewComponentStaleProviderState,
    /// Whether the card presents a single pass/fail verdict; must be false when richer evidence exists.
    pub presents_single_verdict: bool,
    /// Human-readable headline verdict label shown alongside the per-check breakdown.
    pub headline_verdict_label: String,
    /// Per-check entries, in display order.
    pub checks: Vec<CheckEntry>,
    /// Browser-handoff boundary; required and non-empty when the disclosure demands it.
    pub browser_handoff_boundary: String,
    /// Local-continue fallback; required and non-empty when the disclosure demands it.
    pub local_continue_fallback: String,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
}

impl ChecksSummaryCard {
    /// Whether this card carries richer per-check evidence than a single verdict can honestly convey.
    ///
    /// A card carries richer evidence when it holds more than one check, links to
    /// any log/artifact/annotation, or shows any anomalous check class. Such a card
    /// must never be collapsed into one pass/fail number.
    pub fn has_richer_evidence(&self) -> bool {
        self.checks.len() >= 2
            || self.checks.iter().any(|check| {
                !check.evidence_links.is_empty() || check.check_class.needs_evaluation_reason()
            })
    }

    /// Disclosures this card must carry, derived from its freshness and evidence depth.
    pub fn disclosure(&self) -> ChecksSummaryCardDisclosure {
        resolve_checks_summary_card_disclosure(self.provider_freshness, self.has_richer_evidence())
    }

    /// Whether this card exposes at least one in-product action for ordinary triage.
    pub fn has_in_product_action(&self) -> bool {
        self.checks.iter().any(CheckEntry::has_in_product_action)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChecksSummaryCardTrustReview {
    /// Required and optional checks are visibly distinct.
    pub required_optional_distinct: bool,
    /// Skipped, suppressed, timed-out, stale, and not-evaluated-here checks are visibly distinct.
    pub anomalous_check_states_distinct: bool,
    /// Checks are never flattened into one pass/fail number when richer evidence exists.
    pub checks_never_flattened_when_richer_evidence: bool,
    /// Log/artifact/annotation links preserve the originating review and check identity.
    pub log_artifact_annotation_identity_preserved: bool,
    /// A provider outage preserves a local-continue path instead of collapsing the card.
    pub provider_outage_preserves_local_continuation: bool,
    /// Stale sync degrades one card and never collapses the whole review lane.
    pub stale_sync_never_collapses_review_lane: bool,
    /// Rerun and cancel are offered only where they are allowed.
    pub rerun_cancel_only_where_allowed: bool,
    /// Ordinary triage never forces raw-provider navigation.
    pub no_forced_raw_provider_navigation_for_triage: bool,
    /// Not-evaluated-here and stale checks are never shown as a plain pass.
    pub not_evaluated_or_stale_never_shown_as_pass: bool,
    /// One card contract is reused with no hidden provider-specific meaning.
    pub one_card_contract_no_hidden_provider_meaning: bool,
    /// Downgrade narrows the claim rather than hiding the card.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified cards automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl ChecksSummaryCardTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.required_optional_distinct
            && self.anomalous_check_states_distinct
            && self.checks_never_flattened_when_richer_evidence
            && self.log_artifact_annotation_identity_preserved
            && self.provider_outage_preserves_local_continuation
            && self.stale_sync_never_collapses_review_lane
            && self.rerun_cancel_only_where_allowed
            && self.no_forced_raw_provider_navigation_for_triage
            && self.not_evaluated_or_stale_never_shown_as_pass
            && self.one_card_contract_no_hidden_provider_meaning
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChecksSummaryCardConsumerProjection {
    /// The review workspace reuses one card contract.
    pub review_workspace_reuses_one_card_contract: bool,
    /// Review lists reuse one card contract.
    pub review_list_reuses_one_card_contract: bool,
    /// Companion queues reuse one card contract.
    pub companion_queue_reuses_one_card_contract: bool,
    /// The card distinguishes all check classes.
    pub card_distinguishes_all_check_classes: bool,
    /// Evidence links preserve the originating review and check identity.
    pub evidence_links_preserve_review_and_check_identity: bool,
    /// CLI / headless shows card truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows card truth.
    pub support_export_shows_truth: bool,
    /// Diagnostics shows card truth.
    pub diagnostics_shows_truth: bool,
    /// Help / About shows card truth.
    pub help_about_shows_truth: bool,
    /// Export preserves check identity across reopen paths.
    pub export_preserves_check_identity: bool,
}

impl ChecksSummaryCardConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.review_workspace_reuses_one_card_contract
            && self.review_list_reuses_one_card_contract
            && self.companion_queue_reuses_one_card_contract
            && self.card_distinguishes_all_check_classes
            && self.evidence_links_preserve_review_and_check_identity
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.diagnostics_shows_truth
            && self.help_about_shows_truth
            && self.export_preserves_check_identity
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChecksSummaryCardProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ChecksSummaryCardPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksSummaryCardPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Checks-summary cards.
    pub cards: Vec<ChecksSummaryCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ChecksSummaryCardDowngradeTrigger>,
    /// Consumer surfaces that must reuse this card contract.
    pub consumer_surfaces: Vec<ChecksSummaryCardConsumerSurface>,
    /// Trust review block.
    pub trust_review: ChecksSummaryCardTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ChecksSummaryCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ChecksSummaryCardProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe checks-summary card packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChecksSummaryCardPacket {
    /// Record kind; must equal [`CHECKS_SUMMARY_CARD_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CHECKS_SUMMARY_CARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Checks-summary cards.
    pub cards: Vec<ChecksSummaryCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ChecksSummaryCardDowngradeTrigger>,
    /// Consumer surfaces that must reuse this card contract.
    pub consumer_surfaces: Vec<ChecksSummaryCardConsumerSurface>,
    /// Trust review block.
    pub trust_review: ChecksSummaryCardTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ChecksSummaryCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ChecksSummaryCardProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ChecksSummaryCardPacket {
    /// Builds a checks-summary card packet from stable-lane input.
    pub fn new(input: ChecksSummaryCardPacketInput) -> Self {
        Self {
            record_kind: CHECKS_SUMMARY_CARD_RECORD_KIND.to_owned(),
            schema_version: CHECKS_SUMMARY_CARD_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            cards: input.cards,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the checks-summary card invariants.
    pub fn validate(&self) -> Vec<ChecksSummaryCardViolation> {
        let mut violations = Vec::new();

        if self.record_kind != CHECKS_SUMMARY_CARD_RECORD_KIND {
            violations.push(ChecksSummaryCardViolation::WrongRecordKind);
        }
        if self.schema_version != CHECKS_SUMMARY_CARD_SCHEMA_VERSION {
            violations.push(ChecksSummaryCardViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ChecksSummaryCardViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ChecksSummaryCardViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ChecksSummaryCardViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_cards(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(ChecksSummaryCardViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ChecksSummaryCardViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ChecksSummaryCardViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("checks-summary card packet serializes"),
        ) {
            violations.push(ChecksSummaryCardViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("checks-summary card packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let total_checks: usize = self.cards.iter().map(|card| card.checks.len()).sum();
        let flattened = self
            .cards
            .iter()
            .filter(|card| card.presents_single_verdict)
            .count();

        let mut out = String::new();
        out.push_str("# Checks-Summary Cards: Check-Class and Evidence Continuity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Cards: {} ({} checks total, {} presenting a single verdict)\n",
            self.cards.len(),
            total_checks,
            flattened
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Cards\n\n");
        for card in &self.cards {
            out.push_str(&format!(
                "- **{}** [`{}`]: {} — {} checks, provider freshness `{}`\n",
                card.review_id_label,
                card.provider_freshness.as_str(),
                card.provider_identity_label,
                card.checks.len(),
                card.provider_freshness.as_str()
            ));
            for check in &card.checks {
                out.push_str(&format!(
                    "  - `{}` [{}] — {} evidence link(s)\n",
                    check.check_name_label,
                    check.check_class.as_str(),
                    check.evidence_links.len()
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in checks-summary card export.
#[derive(Debug)]
pub enum ChecksSummaryCardArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ChecksSummaryCardViolation>),
}

impl fmt::Display for ChecksSummaryCardArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "checks-summary card export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "checks-summary card export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ChecksSummaryCardArtifactError {}

/// Validation failures emitted by [`ChecksSummaryCardPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChecksSummaryCardViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No cards are present.
    CardsMissing,
    /// A card is incomplete.
    CardIncomplete,
    /// A check is incomplete.
    CheckIncomplete,
    /// A card flattens richer per-check evidence into a single pass/fail verdict.
    ChecksFlattenedToSingleVerdict,
    /// An anomalous check is missing its explicit evaluation reason.
    CheckEvaluationReasonMissing,
    /// An evidence link does not preserve the originating review and check identity.
    EvidenceIdentityNotPreserved,
    /// A card that must preserve a local-continue fallback is missing it.
    LocalContinueFallbackMissing,
    /// A card that needs an explicit browser-handoff boundary is missing it.
    BrowserHandoffBoundaryMissing,
    /// A card forces raw-provider navigation for ordinary triage.
    ForcedRawProviderNavigation,
    /// The card set does not cover every check class.
    CheckClassCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ChecksSummaryCardViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::CardsMissing => "cards_missing",
            Self::CardIncomplete => "card_incomplete",
            Self::CheckIncomplete => "check_incomplete",
            Self::ChecksFlattenedToSingleVerdict => "checks_flattened_to_single_verdict",
            Self::CheckEvaluationReasonMissing => "check_evaluation_reason_missing",
            Self::EvidenceIdentityNotPreserved => "evidence_identity_not_preserved",
            Self::LocalContinueFallbackMissing => "local_continue_fallback_missing",
            Self::BrowserHandoffBoundaryMissing => "browser_handoff_boundary_missing",
            Self::ForcedRawProviderNavigation => "forced_raw_provider_navigation",
            Self::CheckClassCoverageMissing => "check_class_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable checks-summary card export.
pub fn current_checks_summary_card_export(
) -> Result<ChecksSummaryCardPacket, ChecksSummaryCardArtifactError> {
    let packet: ChecksSummaryCardPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/ship_checks_summary_cards_with_check_class_truth_and_log_artifact_annotation_continuity/support_export.json"
    )))
    .map_err(ChecksSummaryCardArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ChecksSummaryCardArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ChecksSummaryCardPacket,
    violations: &mut Vec<ChecksSummaryCardViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        CHECKS_SUMMARY_CARD_SCHEMA_REF,
        CHECKS_SUMMARY_CARD_DOC_REF,
        CHECKS_SUMMARY_CARD_COMPONENT_MATRIX_CONTRACT_REF,
        CHECKS_SUMMARY_CARD_REVIEW_WORKSPACE_CONTRACT_REF,
        CHECKS_SUMMARY_CARD_PIPELINE_RUN_CONTRACT_REF,
        CHECKS_SUMMARY_CARD_LOG_VIEW_CONTRACT_REF,
        CHECKS_SUMMARY_CARD_ARTIFACT_CARD_CONTRACT_REF,
        CHECKS_SUMMARY_CARD_ANNOTATION_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ChecksSummaryCardViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_cards(
    packet: &ChecksSummaryCardPacket,
    violations: &mut Vec<ChecksSummaryCardViolation>,
) {
    if packet.cards.is_empty() {
        violations.push(ChecksSummaryCardViolation::CardsMissing);
        return;
    }

    let mut present: BTreeSet<CheckClass> = BTreeSet::new();

    for card in &packet.cards {
        if card.card_id.trim().is_empty()
            || card.review_id_label.trim().is_empty()
            || card.provider_identity_label.trim().is_empty()
            || card.headline_verdict_label.trim().is_empty()
            || card.checks.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(ChecksSummaryCardViolation::CardIncomplete);
        }

        let disclosure = card.disclosure();

        if disclosure.must_not_flatten_to_single_verdict && card.presents_single_verdict {
            violations.push(ChecksSummaryCardViolation::ChecksFlattenedToSingleVerdict);
        }
        if disclosure.needs_browser_handoff_boundary
            && card.browser_handoff_boundary.trim().is_empty()
        {
            violations.push(ChecksSummaryCardViolation::BrowserHandoffBoundaryMissing);
        }
        if disclosure.needs_local_continue_fallback
            && card.local_continue_fallback.trim().is_empty()
        {
            violations.push(ChecksSummaryCardViolation::LocalContinueFallbackMissing);
        }
        if !card.has_in_product_action() {
            violations.push(ChecksSummaryCardViolation::ForcedRawProviderNavigation);
        }

        for check in &card.checks {
            present.insert(check.check_class);

            if check.check_id.trim().is_empty()
                || check.check_name_label.trim().is_empty()
                || check.actions.is_empty()
            {
                violations.push(ChecksSummaryCardViolation::CheckIncomplete);
            }
            if check.check_class.needs_evaluation_reason()
                && check.evaluation_reason.trim().is_empty()
            {
                violations.push(ChecksSummaryCardViolation::CheckEvaluationReasonMissing);
            }
            for link in &check.evidence_links {
                if link.label.trim().is_empty() || !link.preserves_identity() {
                    violations.push(ChecksSummaryCardViolation::EvidenceIdentityNotPreserved);
                }
            }
        }
    }

    for required in CheckClass::ALL {
        if !present.contains(&required) {
            violations.push(ChecksSummaryCardViolation::CheckClassCoverageMissing);
            return;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
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
