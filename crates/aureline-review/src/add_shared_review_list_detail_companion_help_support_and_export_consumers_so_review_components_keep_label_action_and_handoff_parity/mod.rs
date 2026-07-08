//! Shared review-list / detail / companion / help / support / export consumers
//! that keep review-request and merge-readiness components at label, action, and
//! handoff parity across every claimed M5 profile.
//!
//! This module is the closing consumer-adoption lane for the seven reusable review
//! components frozen in
//! [`crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix`]
//! and implemented by the review-request-row, checks-summary-card,
//! merge-readiness / merge-queue / stack-dependency, and
//! pending-review-tray / approval-invalidation-banner lanes. It binds each shared
//! component to the desktop list, detail pane, browser companion triage, Help
//! surface, support packet, and exported evidence that render it, and proves — by
//! fixtures, not screenshots — that the same review object presents the same
//! provider, queue, readiness, and staleness language wherever it appears.
//!
//! The core honesty axes are two. First, parity: for a given review object, every
//! consumer surface must present identical parity facet values — the same label,
//! the same primary action, the same queue/readiness/status language, and the same
//! handoff reason. A surface may narrow how much it shows when provider freshness
//! degrades, but it may never reword the underlying language per surface, flatten
//! provider-managed queue state into a local estimate, hide approval invalidation
//! behind a generic warning pill, or force raw-provider navigation for ordinary
//! triage. Second, disclosure: when a surface narrows, it must do so through an
//! explicit narrow banner that names the reason, the preserved facets, and the next
//! action — browser handoff and local-continue fallbacks stay explicit rather than
//! collapsing the object out of view.
//!
//! Component reuse is proven rather than inferred: every one of the seven shared
//! components must be adopted by at least two distinct consumers, and Help, support,
//! and exported-evidence consumers must point at the canonical component contracts
//! by id. The provider-freshness vocabulary is reused directly from the frozen
//! matrix ([`M5ReviewComponentStaleProviderState`]) and the component identity from
//! [`M5ReviewComponent`], so freshness downgrades and component identity read the
//! same everywhere.
//!
//! The packet references upstream component contracts by id rather than embedding
//! their content. Raw provider responses, credentials, and live provider payloads
//! stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-review-component-consumer.schema.json`](../../../../schemas/ui/m5-review-component-consumer.schema.json).
//! The contract doc is
//! [`docs/review/m5/add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity.md`](../../../../docs/review/m5/add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-review-component-consumers/`](../../../../fixtures/ui/m5-review-component-consumers/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix::{
    M5ReviewComponent, M5ReviewComponentStaleProviderState,
};

/// Stable record-kind tag carried by [`ReviewComponentConsumerPacket`].
pub const REVIEW_COMPONENT_CONSUMER_RECORD_KIND: &str = "review_component_consumer_parity_truth";

/// Schema version for review-component consumer parity records.
pub const REVIEW_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const REVIEW_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-review-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const REVIEW_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/review/m5/add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity.md";

/// Repo-relative path of the frozen component matrix these consumers adopt.
pub const REVIEW_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-review-request-check-queue-component-matrix.schema.json";

/// Repo-relative path of the review-request-row component contract.
pub const REVIEW_COMPONENT_CONSUMER_REVIEW_REQUEST_ROW_CONTRACT_REF: &str =
    "schemas/ui/m5-review-request-row.schema.json";

/// Repo-relative path of the checks-summary-card component contract.
pub const REVIEW_COMPONENT_CONSUMER_CHECKS_SUMMARY_CARD_CONTRACT_REF: &str =
    "schemas/ui/m5-checks-summary-card.schema.json";

/// Repo-relative path of the combined merge-readiness / merge-queue / stack-dependency contract.
pub const REVIEW_COMPONENT_CONSUMER_MERGE_READINESS_PANEL_CONTRACT_REF: &str =
    "schemas/ui/m5-merge-readiness-panel.schema.json";

/// Repo-relative path of the combined pending-review-tray / approval-invalidation contract.
pub const REVIEW_COMPONENT_CONSUMER_PENDING_REVIEW_TRAY_CONTRACT_REF: &str =
    "schemas/ui/m5-pending-review-tray.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const REVIEW_COMPONENT_CONSUMER_FIXTURE_DIR: &str = "fixtures/ui/m5-review-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const REVIEW_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/review/m5/add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const REVIEW_COMPONENT_CONSUMER_SUMMARY_REF: &str =
    "artifacts/review/m5/add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity.md";

/// Canonical component contract that a consumer must point at for a given component.
///
/// Each of the seven shared components resolves to the checked-in schema of the lane
/// that implemented it: the review-request-row, checks-summary-card, combined
/// merge-readiness panel (which also governs merge-queue entries and
/// stack-dependency chips), and combined pending-review tray (which also governs
/// approval-invalidation banners).
pub const fn component_canonical_schema_ref(component: M5ReviewComponent) -> &'static str {
    match component {
        M5ReviewComponent::ReviewRequestRow => {
            REVIEW_COMPONENT_CONSUMER_REVIEW_REQUEST_ROW_CONTRACT_REF
        }
        M5ReviewComponent::ChecksSummaryCard => {
            REVIEW_COMPONENT_CONSUMER_CHECKS_SUMMARY_CARD_CONTRACT_REF
        }
        M5ReviewComponent::MergeReadinessPanel
        | M5ReviewComponent::MergeQueueEntry
        | M5ReviewComponent::StackDependencyChip => {
            REVIEW_COMPONENT_CONSUMER_MERGE_READINESS_PANEL_CONTRACT_REF
        }
        M5ReviewComponent::PendingReviewTray | M5ReviewComponent::ApprovalInvalidationBanner => {
            REVIEW_COMPONENT_CONSUMER_PENDING_REVIEW_TRAY_CONTRACT_REF
        }
    }
}

/// Consumer surface that must reuse the shared review components at full parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentConsumer {
    /// Desktop review list.
    DesktopList,
    /// Review detail pane.
    DetailPane,
    /// Browser companion triage queue.
    CompanionTriage,
    /// Help / About surface.
    HelpSurface,
    /// Support packet.
    SupportExport,
    /// Exported review evidence.
    ExportedEvidence,
}

impl ReviewComponentConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopList,
        Self::DetailPane,
        Self::CompanionTriage,
        Self::HelpSurface,
        Self::SupportExport,
        Self::ExportedEvidence,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopList => "desktop_list",
            Self::DetailPane => "detail_pane",
            Self::CompanionTriage => "companion_triage",
            Self::HelpSurface => "help_surface",
            Self::SupportExport => "support_export",
            Self::ExportedEvidence => "exported_evidence",
        }
    }

    /// Whether this consumer is a Help, support, or exported-evidence surface that
    /// must point at the canonical component contracts by id.
    pub const fn is_help_support_or_export(self) -> bool {
        matches!(
            self,
            Self::HelpSurface | Self::SupportExport | Self::ExportedEvidence
        )
    }
}

/// A parity facet whose value must stay identical across surfaces for one object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentParityFacet {
    /// The primary label for the component.
    Label,
    /// The primary action offered by the component.
    PrimaryAction,
    /// The queue / readiness / status language shown on the component.
    QueueReadinessStatusLanguage,
    /// The handoff reason shown when the component hands off.
    HandoffReason,
}

impl ReviewComponentParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Label,
        Self::PrimaryAction,
        Self::QueueReadinessStatusLanguage,
        Self::HandoffReason,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Label => "label",
            Self::PrimaryAction => "primary_action",
            Self::QueueReadinessStatusLanguage => "queue_readiness_status_language",
            Self::HandoffReason => "handoff_reason",
        }
    }
}

/// How much of a shared component a consumer renders.
///
/// Narrowing changes how much is shown, never the underlying parity language: a
/// narrowed surface still carries the same label, primary action, status language,
/// and handoff reason, and discloses the narrowing through an explicit banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentRenderMode {
    /// Full parity; provider truth is fresh.
    FullParity,
    /// Freshness is narrowed; provider truth is refreshing, stale, or in conflict.
    FreshnessNarrowed,
    /// Browser handoff is required; the provider is unreachable.
    HandoffRequired,
    /// Local-continue fallback; the surface continues from local-only truth.
    LocalContinueFallback,
}

impl ReviewComponentRenderMode {
    /// Every render mode, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullParity,
        Self::FreshnessNarrowed,
        Self::HandoffRequired,
        Self::LocalContinueFallback,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::FreshnessNarrowed => "freshness_narrowed",
            Self::HandoffRequired => "handoff_required",
            Self::LocalContinueFallback => "local_continue_fallback",
        }
    }

    /// Whether this mode narrows below full parity.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }
}

/// Why a surface narrowed its rendering of a shared component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentNarrowReason {
    /// Provider-backed freshness degraded (refreshing, stale, or in conflict).
    ProviderFreshnessDegraded,
    /// The provider is unreachable and browser handoff is required.
    BrowserHandoffRequired,
    /// A local-continue fallback is engaged while provider freshness is degraded.
    LocalContinueEngaged,
}

impl ReviewComponentNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderFreshnessDegraded => "provider_freshness_degraded",
            Self::BrowserHandoffRequired => "browser_handoff_required",
            Self::LocalContinueEngaged => "local_continue_engaged",
        }
    }
}

/// The next action a narrow banner offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentNarrowNextAction {
    /// Refresh provider freshness.
    RefreshProviderFreshness,
    /// Open the browser handoff.
    OpenBrowserHandoff,
    /// Continue reviewing locally.
    ContinueLocalReview,
}

impl ReviewComponentNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshProviderFreshness => "refresh_provider_freshness",
            Self::OpenBrowserHandoff => "open_browser_handoff",
            Self::ContinueLocalReview => "continue_local_review",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentParityState {
    /// All parity facets are preserved and shown in full.
    FacetsPreserved,
    /// All parity facets are preserved, and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl ReviewComponentParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentConsumerDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Provider-backed freshness has gone stale relative to what it gates.
    ProviderFreshnessStale,
    /// An approval invalidation is pending and unresolved.
    ApprovalInvalidationPending,
    /// Browser handoff for provider deep links is unavailable.
    BrowserHandoffUnavailable,
    /// A local-continue fallback is unavailable while provider freshness is degraded.
    LocalContinueUnavailable,
    /// Parity drift was detected between surfaces for the same object.
    ParityDriftDetected,
    /// Consumer trust narrowed.
    TrustNarrowing,
    /// An upstream shared component narrowed.
    UpstreamComponentNarrowed,
}

impl ReviewComponentConsumerDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderFreshnessStale,
        Self::ApprovalInvalidationPending,
        Self::BrowserHandoffUnavailable,
        Self::LocalContinueUnavailable,
        Self::ParityDriftDetected,
        Self::TrustNarrowing,
        Self::UpstreamComponentNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderFreshnessStale => "provider_freshness_stale",
            Self::ApprovalInvalidationPending => "approval_invalidation_pending",
            Self::BrowserHandoffUnavailable => "browser_handoff_unavailable",
            Self::LocalContinueUnavailable => "local_continue_unavailable",
            Self::ParityDriftDetected => "parity_drift_detected",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamComponentNarrowed => "upstream_component_narrowed",
        }
    }
}

/// The parity facet values a shared component presents for one review object.
///
/// These four values must be identical across every consumer surface that shows the
/// same review object. A surface may narrow how much it renders, but it may never
/// reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentParityFacetValues {
    /// Primary label (never reworded per surface).
    pub label: String,
    /// Primary action (identical across surfaces).
    pub primary_action: String,
    /// Queue / readiness / status language (identical across surfaces).
    pub queue_readiness_status_language: String,
    /// Handoff reason (identical across surfaces).
    pub handoff_reason: String,
}

impl ReviewComponentParityFacetValues {
    /// Whether every parity facet value is present.
    pub fn all_present(&self) -> bool {
        !self.label.trim().is_empty()
            && !self.primary_action.trim().is_empty()
            && !self.queue_readiness_status_language.trim().is_empty()
            && !self.handoff_reason.trim().is_empty()
    }
}

/// The explicit banner a narrowed surface shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentNarrowBanner {
    /// Why the surface narrowed.
    pub reason: ReviewComponentNarrowReason,
    /// Note naming the preserved parity facets (never omitted).
    pub preserved_facets_note: String,
    /// The next action offered.
    pub next_action: ReviewComponentNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its provider freshness.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewComponentRenderDisclosure {
    /// The render mode the freshness state requires.
    pub expected_mode: ReviewComponentRenderMode,
    /// The narrow reason the render mode requires, if any.
    pub narrow_reason: Option<ReviewComponentNarrowReason>,
    /// Whether the binding must carry an explicit narrow banner.
    pub needs_narrow_banner: bool,
    /// Whether the binding must carry an explicit local-continue note.
    pub needs_local_continue_note: bool,
    /// Whether the binding must carry an explicit browser-handoff boundary.
    pub needs_browser_handoff_boundary: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its freshness.
///
/// Fresh provider truth renders at full parity. Refreshing, stale, or conflicting
/// provider truth narrows freshness while keeping every parity facet. An unreachable
/// provider forces a browser-handoff boundary, and a local-only continuation engages
/// the local-continue fallback. In every degraded case except plain refreshing the
/// binding must offer a local-continue path, so the reviewer's work never vanishes.
pub fn resolve_review_component_render_disclosure(
    provider_freshness: M5ReviewComponentStaleProviderState,
) -> ReviewComponentRenderDisclosure {
    let (expected_mode, narrow_reason) = match provider_freshness {
        M5ReviewComponentStaleProviderState::ProviderFresh => {
            (ReviewComponentRenderMode::FullParity, None)
        }
        M5ReviewComponentStaleProviderState::ProviderRefreshing
        | M5ReviewComponentStaleProviderState::ProviderStale
        | M5ReviewComponentStaleProviderState::ProviderConflict => (
            ReviewComponentRenderMode::FreshnessNarrowed,
            Some(ReviewComponentNarrowReason::ProviderFreshnessDegraded),
        ),
        M5ReviewComponentStaleProviderState::ProviderUnreachable => (
            ReviewComponentRenderMode::HandoffRequired,
            Some(ReviewComponentNarrowReason::BrowserHandoffRequired),
        ),
        M5ReviewComponentStaleProviderState::LocalOnlyContinuation => (
            ReviewComponentRenderMode::LocalContinueFallback,
            Some(ReviewComponentNarrowReason::LocalContinueEngaged),
        ),
    };

    // A local-continue path is required in every degraded case except plain
    // refreshing, matching the tray/banner disclosure rules of the implement lanes.
    let needs_local_continue_note = matches!(
        provider_freshness,
        M5ReviewComponentStaleProviderState::ProviderStale
            | M5ReviewComponentStaleProviderState::ProviderUnreachable
            | M5ReviewComponentStaleProviderState::ProviderConflict
            | M5ReviewComponentStaleProviderState::LocalOnlyContinuation
    );
    let needs_browser_handoff_boundary = matches!(
        provider_freshness,
        M5ReviewComponentStaleProviderState::ProviderUnreachable
    );

    ReviewComponentRenderDisclosure {
        expected_mode,
        narrow_reason,
        needs_narrow_banner: expected_mode.is_narrowed(),
        needs_local_continue_note,
        needs_browser_handoff_boundary,
    }
}

/// The parity state a render mode requires.
pub const fn parity_state_for_mode(mode: ReviewComponentRenderMode) -> ReviewComponentParityState {
    match mode {
        ReviewComponentRenderMode::FullParity => ReviewComponentParityState::FacetsPreserved,
        ReviewComponentRenderMode::FreshnessNarrowed
        | ReviewComponentRenderMode::HandoffRequired
        | ReviewComponentRenderMode::LocalContinueFallback => {
            ReviewComponentParityState::FacetsDisclosedNarrowed
        }
    }
}

/// One consumer binding: a shared component rendered on one consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable review-object id (shared across surfaces that show the same object).
    pub review_object_id: String,
    /// Human-readable review-object identity.
    pub review_object_label: String,
    /// Which shared component this binding renders.
    pub component: M5ReviewComponent,
    /// Which consumer surface renders it.
    pub consumer: ReviewComponentConsumer,
    /// Provider-freshness state, reused from the frozen component matrix.
    pub provider_freshness: M5ReviewComponentStaleProviderState,
    /// How much of the component this surface renders.
    pub render_mode: ReviewComponentRenderMode,
    /// The parity facet values presented (identical across surfaces for one object).
    pub parity_facets: ReviewComponentParityFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: ReviewComponentParityState,
    /// The explicit narrow banner; required and complete when the binding narrows.
    pub narrow_banner: Option<ReviewComponentNarrowBanner>,
    /// Local-continue note; required and non-empty when the disclosure demands it.
    pub local_continue_note: String,
    /// Browser-handoff boundary; required and non-empty when the disclosure demands it.
    pub browser_handoff_boundary: String,
    /// Guardrail: this surface forces raw-provider navigation for ordinary triage.
    pub forces_raw_provider_navigation_for_triage: bool,
    /// Guardrail: this surface flattens provider-managed queue state into a local estimate.
    pub flattens_provider_state_into_local_estimate: bool,
    /// Guardrail: this surface hides approval invalidation behind a generic warning pill.
    pub hides_approval_invalidation_behind_generic_pill: bool,
    /// Guardrail: this surface rewords the parity labels per surface.
    pub rewords_labels_per_surface: bool,
    /// Guardrail: this surface drops the handoff reason or the local-continue fallback.
    pub drops_handoff_reason_or_local_continue: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl ReviewComponentConsumerBinding {
    /// Disclosures this binding must carry, derived from its provider freshness.
    pub fn disclosure(&self) -> ReviewComponentRenderDisclosure {
        resolve_review_component_render_disclosure(self.provider_freshness)
    }

    /// Whether this binding renders below full parity.
    pub fn is_narrowed(&self) -> bool {
        self.render_mode.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub fn guardrails_hold(&self) -> bool {
        !self.forces_raw_provider_navigation_for_triage
            && !self.flattens_provider_state_into_local_estimate
            && !self.hides_approval_invalidation_behind_generic_pill
            && !self.rewords_labels_per_surface
            && !self.drops_handoff_reason_or_local_continue
    }

    /// Whether this binding points at the canonical component schema and matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = component_canonical_schema_ref(self.component);
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self.source_contract_refs.iter().any(|reference| {
                reference == REVIEW_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF
            })
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentConsumerTrustReview {
    /// Component reuse is proven by fixtures rather than inferred from screenshots.
    pub component_reuse_proven_by_fixtures: bool,
    /// The same review object presents the same language across surfaces.
    pub same_object_same_language_across_surfaces: bool,
    /// Provider-managed state is never flattened into a local estimate.
    pub provider_state_never_flattened_to_local_estimate: bool,
    /// Approval invalidation is never hidden behind a generic warning pill.
    pub approval_invalidation_never_hidden_behind_generic_pill: bool,
    /// Primary actions are identical across surfaces.
    pub primary_actions_identical_across_surfaces: bool,
    /// Queue / readiness / status language is identical across surfaces.
    pub queue_readiness_status_language_identical_across_surfaces: bool,
    /// Browser handoff is kept explicit.
    pub browser_handoff_kept_explicit: bool,
    /// Local continuation is preserved when provider freshness is degraded.
    pub local_continue_preserved_under_degraded_freshness: bool,
    /// Ordinary triage never forces raw-provider navigation.
    pub no_forced_raw_provider_navigation_for_triage: bool,
    /// Help, support, and export consumers point at the canonical contracts.
    pub help_support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl ReviewComponentConsumerTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.component_reuse_proven_by_fixtures
            && self.same_object_same_language_across_surfaces
            && self.provider_state_never_flattened_to_local_estimate
            && self.approval_invalidation_never_hidden_behind_generic_pill
            && self.primary_actions_identical_across_surfaces
            && self.queue_readiness_status_language_identical_across_surfaces
            && self.browser_handoff_kept_explicit
            && self.local_continue_preserved_under_degraded_freshness
            && self.no_forced_raw_provider_navigation_for_triage
            && self.help_support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentConsumerProjection {
    /// The desktop list reuses the shared components.
    pub desktop_list_reuses_shared_components: bool,
    /// The detail pane reuses the shared components.
    pub detail_pane_reuses_shared_components: bool,
    /// Companion triage reuses the shared components.
    pub companion_triage_reuses_shared_components: bool,
    /// The Help surface reuses the shared components.
    pub help_surface_reuses_shared_components: bool,
    /// The support export reuses the shared components.
    pub support_export_reuses_shared_components: bool,
    /// Exported evidence reuses the shared components.
    pub exported_evidence_reuses_shared_components: bool,
    /// Every component is adopted by two or more consumers.
    pub every_component_adopted_by_two_or_more_consumers: bool,
    /// Parity facets are identical for the same review object.
    pub parity_facets_identical_for_same_object: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export preserves provider and queue identity.
    pub export_preserves_provider_and_queue_identity: bool,
}

impl ReviewComponentConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.desktop_list_reuses_shared_components
            && self.detail_pane_reuses_shared_components
            && self.companion_triage_reuses_shared_components
            && self.help_surface_reuses_shared_components
            && self.support_export_reuses_shared_components
            && self.exported_evidence_reuses_shared_components
            && self.every_component_adopted_by_two_or_more_consumers
            && self.parity_facets_identical_for_same_object
            && self.narrowing_disclosed_not_hidden
            && self.export_preserves_provider_and_queue_identity
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ReviewComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ReviewComponentConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ReviewComponentConsumerDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<ReviewComponentConsumer>,
    /// Trust review block.
    pub trust_review: ReviewComponentConsumerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ReviewComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ReviewComponentConsumerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe review-component consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentConsumerPacket {
    /// Record kind; must equal [`REVIEW_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`REVIEW_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ReviewComponentConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ReviewComponentConsumerDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<ReviewComponentConsumer>,
    /// Trust review block.
    pub trust_review: ReviewComponentConsumerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ReviewComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ReviewComponentConsumerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ReviewComponentConsumerPacket {
    /// Builds a review-component consumer packet from stable-lane input.
    pub fn new(input: ReviewComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: REVIEW_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: REVIEW_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            consumer_bindings: input.consumer_bindings,
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

    /// Validates the review-component consumer parity invariants.
    pub fn validate(&self) -> Vec<ReviewComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != REVIEW_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(ReviewComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != REVIEW_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(ReviewComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ReviewComponentConsumerViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ReviewComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ReviewComponentConsumerViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(ReviewComponentConsumerViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ReviewComponentConsumerViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ReviewComponentConsumerViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("review-component consumer packet serializes"),
        ) {
            violations.push(ReviewComponentConsumerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("review-component consumer packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str("# Shared Review-Component Consumers: Label, Action, and Handoff Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed)\n",
            self.consumer_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: component `{}` on `{}`, mode `{}`\n",
                binding.review_object_label,
                binding.binding_id,
                binding.component.as_str(),
                binding.consumer.as_str(),
                binding.render_mode.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in review-component consumer export.
#[derive(Debug)]
pub enum ReviewComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ReviewComponentConsumerViolation>),
}

impl fmt::Display for ReviewComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "review-component consumer export parse failed: {error}"
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
                    "review-component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ReviewComponentConsumerArtifactError {}

/// Validation failures emitted by [`ReviewComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReviewComponentConsumerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No consumer bindings are present.
    ConsumerBindingsMissing,
    /// A consumer binding is incomplete.
    BindingIncomplete,
    /// A binding's parity facet values are incomplete.
    ParityFacetIncomplete,
    /// A binding's render mode does not match its provider freshness.
    RenderModeMismatch,
    /// A binding's parity state does not match its render mode.
    ParityStateMismatch,
    /// Two surfaces show the same review object with different parity language.
    ParityDriftAcrossSurfaces,
    /// A shared component is not adopted by at least two distinct consumers.
    ReviewComponentReuseUnproven,
    /// A Help, support, or export binding does not point at the canonical contracts.
    HelpSupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow banner.
    NarrowBannerMissing,
    /// A narrow banner's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow banner is missing its preserved-facets note.
    NarrowBannerPreservedFacetsMissing,
    /// A narrow banner is missing its next-action copy.
    NarrowNextActionMissing,
    /// A binding that must preserve a local-continue path is missing its note.
    LocalContinueNoteMissing,
    /// A binding that needs an explicit browser-handoff boundary is missing it.
    BrowserHandoffBoundaryMissing,
    /// A binding forces raw-provider navigation for ordinary triage.
    ForcedRawProviderNavigation,
    /// A binding flattens provider-managed queue state into a local estimate.
    ProviderStateFlattenedToLocalEstimate,
    /// A binding hides approval invalidation behind a generic warning pill.
    ApprovalInvalidationHiddenBehindGenericPill,
    /// A binding rewords the parity labels per surface.
    LabelsRewordedPerSurface,
    /// A binding drops the handoff reason or the local-continue fallback.
    HandoffOrLocalContinueDropped,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared component appears among the bindings.
    ComponentCoverageMissing,
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

impl ReviewComponentConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::ParityFacetIncomplete => "parity_facet_incomplete",
            Self::RenderModeMismatch => "render_mode_mismatch",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::ParityDriftAcrossSurfaces => "parity_drift_across_surfaces",
            Self::ReviewComponentReuseUnproven => "review_component_reuse_unproven",
            Self::HelpSupportExportReferenceMissing => "help_support_export_reference_missing",
            Self::NarrowBannerMissing => "narrow_banner_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowBannerPreservedFacetsMissing => "narrow_banner_preserved_facets_missing",
            Self::NarrowNextActionMissing => "narrow_next_action_missing",
            Self::LocalContinueNoteMissing => "local_continue_note_missing",
            Self::BrowserHandoffBoundaryMissing => "browser_handoff_boundary_missing",
            Self::ForcedRawProviderNavigation => "forced_raw_provider_navigation",
            Self::ProviderStateFlattenedToLocalEstimate => {
                "provider_state_flattened_to_local_estimate"
            }
            Self::ApprovalInvalidationHiddenBehindGenericPill => {
                "approval_invalidation_hidden_behind_generic_pill"
            }
            Self::LabelsRewordedPerSurface => "labels_reworded_per_surface",
            Self::HandoffOrLocalContinueDropped => "handoff_or_local_continue_dropped",
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable review-component consumer export.
pub fn current_review_component_consumer_export(
) -> Result<ReviewComponentConsumerPacket, ReviewComponentConsumerArtifactError> {
    let packet: ReviewComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity/support_export.json"
    )))
    .map_err(ReviewComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ReviewComponentConsumerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ReviewComponentConsumerPacket,
    violations: &mut Vec<ReviewComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        REVIEW_COMPONENT_CONSUMER_SCHEMA_REF,
        REVIEW_COMPONENT_CONSUMER_DOC_REF,
        REVIEW_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF,
        REVIEW_COMPONENT_CONSUMER_REVIEW_REQUEST_ROW_CONTRACT_REF,
        REVIEW_COMPONENT_CONSUMER_CHECKS_SUMMARY_CARD_CONTRACT_REF,
        REVIEW_COMPONENT_CONSUMER_MERGE_READINESS_PANEL_CONTRACT_REF,
        REVIEW_COMPONENT_CONSUMER_PENDING_REVIEW_TRAY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ReviewComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &ReviewComponentConsumerPacket,
    violations: &mut Vec<ReviewComponentConsumerViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(ReviewComponentConsumerViolation::ConsumerBindingsMissing);
        return;
    }

    // Parity: the parity facet values must be identical for every binding that
    // renders the same review object.
    let mut object_facets: BTreeMap<&str, &ReviewComponentParityFacetValues> = BTreeMap::new();
    let mut parity_drift_reported = false;

    // Reuse: each component must be adopted by at least two distinct consumers.
    let mut component_consumers: BTreeMap<M5ReviewComponent, BTreeSet<ReviewComponentConsumer>> =
        BTreeMap::new();
    let mut seen_consumers: BTreeSet<ReviewComponentConsumer> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5ReviewComponent> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.review_object_id.trim().is_empty()
            || binding.review_object_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(ReviewComponentConsumerViolation::BindingIncomplete);
        }
        if !binding.parity_facets.all_present() {
            violations.push(ReviewComponentConsumerViolation::ParityFacetIncomplete);
        }

        let disclosure = binding.disclosure();

        if binding.render_mode != disclosure.expected_mode {
            violations.push(ReviewComponentConsumerViolation::RenderModeMismatch);
        }
        if binding.parity_state != parity_state_for_mode(binding.render_mode) {
            violations.push(ReviewComponentConsumerViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_banner {
            match &binding.narrow_banner {
                None => {
                    violations.push(ReviewComponentConsumerViolation::NarrowBannerMissing);
                }
                Some(banner) => {
                    if Some(banner.reason) != disclosure.narrow_reason {
                        violations.push(ReviewComponentConsumerViolation::NarrowReasonMismatch);
                    }
                    if banner.preserved_facets_note.trim().is_empty() {
                        violations.push(
                            ReviewComponentConsumerViolation::NarrowBannerPreservedFacetsMissing,
                        );
                    }
                    if banner.next_action_label.trim().is_empty() {
                        violations.push(ReviewComponentConsumerViolation::NarrowNextActionMissing);
                    }
                }
            }
        } else if binding.narrow_banner.is_some() {
            // A full-parity binding must not carry a narrow banner.
            violations.push(ReviewComponentConsumerViolation::NarrowBannerMissing);
        }

        if disclosure.needs_local_continue_note && binding.local_continue_note.trim().is_empty() {
            violations.push(ReviewComponentConsumerViolation::LocalContinueNoteMissing);
        }
        if disclosure.needs_browser_handoff_boundary
            && binding.browser_handoff_boundary.trim().is_empty()
        {
            violations.push(ReviewComponentConsumerViolation::BrowserHandoffBoundaryMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.forces_raw_provider_navigation_for_triage {
            violations.push(ReviewComponentConsumerViolation::ForcedRawProviderNavigation);
        }
        if binding.flattens_provider_state_into_local_estimate {
            violations
                .push(ReviewComponentConsumerViolation::ProviderStateFlattenedToLocalEstimate);
        }
        if binding.hides_approval_invalidation_behind_generic_pill {
            violations.push(
                ReviewComponentConsumerViolation::ApprovalInvalidationHiddenBehindGenericPill,
            );
        }
        if binding.rewords_labels_per_surface {
            violations.push(ReviewComponentConsumerViolation::LabelsRewordedPerSurface);
        }
        if binding.drops_handoff_reason_or_local_continue {
            violations.push(ReviewComponentConsumerViolation::HandoffOrLocalContinueDropped);
        }

        // Help / support / export consumers must point at the canonical contracts.
        if binding.consumer.is_help_support_or_export() && !binding.points_at_canonical_contracts()
        {
            violations.push(ReviewComponentConsumerViolation::HelpSupportExportReferenceMissing);
        }

        // Parity drift accumulation.
        match object_facets.get(binding.review_object_id.as_str()) {
            None => {
                object_facets.insert(binding.review_object_id.as_str(), &binding.parity_facets);
            }
            Some(existing) => {
                if **existing != binding.parity_facets && !parity_drift_reported {
                    violations.push(ReviewComponentConsumerViolation::ParityDriftAcrossSurfaces);
                    parity_drift_reported = true;
                }
            }
        }

        component_consumers
            .entry(binding.component)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_components.insert(binding.component);
    }

    // Coverage: every consumer and every component must appear.
    for consumer in ReviewComponentConsumer::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(ReviewComponentConsumerViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for component in M5ReviewComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(ReviewComponentConsumerViolation::ComponentCoverageMissing);
            break;
        }
    }

    // Reuse: every present component must be adopted by two or more distinct consumers.
    for consumers in component_consumers.values() {
        if consumers.len() < 2 {
            violations.push(ReviewComponentConsumerViolation::ReviewComponentReuseUnproven);
            break;
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
