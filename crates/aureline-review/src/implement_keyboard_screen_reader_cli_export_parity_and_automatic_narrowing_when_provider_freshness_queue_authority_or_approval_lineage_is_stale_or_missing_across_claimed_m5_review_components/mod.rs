//! Keyboard, screen-reader, CLI, and export parity plus automatic claim narrowing
//! for the seven shared M5 review components.
//!
//! This module is the accessibility / headless / export capstone over the review
//! components frozen in
//! [`crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix`],
//! implemented by the review-request-row, checks-summary-card, merge-readiness /
//! merge-queue / stack-dependency, and pending-review-tray /
//! approval-invalidation-banner lanes, and adopted by the shared consumers in
//! [`crate::add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity`].
//! Where the consumer lane proves label / action / handoff parity across desktop
//! surfaces, this lane proves the harder claim: that review-request, checks-summary,
//! merge-readiness, queue, and approval-invalidation state is exposed just as
//! honestly in assistive, headless, and exported forms as it is on desktop — and
//! that a claim-bearing component automatically narrows the moment its
//! provider-backed truth stops being trustworthy.
//!
//! The honesty axes are two. First, parity across forms: every claimed component
//! must expose a keyboard label, a screen-reader label, a CLI enum token, an export
//! enum token, and a human-readable explanation field, and must render on the
//! desktop, the headless CLI, and the support export alike. No component may be
//! pointer-only, export-opaque, or semantically stronger on the desktop than it is
//! in CLI or support output.
//!
//! Second, automatic narrowing: each component carries a claim about how
//! trustworthy its provider-backed truth is, drawn from
//! [`ReviewComponentClaimTier`]. When provider freshness goes stale, when queue
//! authority drops to a local estimate, when approval lineage is missing, or when
//! an out-of-scope action requires a browser handoff, the claim must narrow to the
//! ceiling permitted by that condition ([`ReviewComponentClaimCondition::permitted_ceiling`]),
//! disclose the narrowing through an explicit trigger and next action, keep the
//! browser handoff explicit, and preserve local-only continuation. A component may
//! never keep asserting a full provider-backed claim while one of those conditions
//! holds.
//!
//! The packet references upstream component and consumer contracts by id rather than
//! embedding their content. Raw provider responses, credentials, and live provider
//! payloads stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-review-component-accessibility-parity.schema.json`](../../../../schemas/ui/m5-review-component-accessibility-parity.schema.json).
//! The contract doc is
//! [`docs/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_freshness_queue_authority_or_approval_lineage_is_stale_or_missing_across_claimed_m5_review_components.md`](../../../../docs/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_freshness_queue_authority_or_approval_lineage_is_stale_or_missing_across_claimed_m5_review_components.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-review-component-accessibility-parity/`](../../../../fixtures/ui/m5-review-component-accessibility-parity/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix::M5ReviewComponent;

/// Stable record-kind tag carried by [`ReviewComponentAccessibilityPacket`].
pub const REVIEW_COMPONENT_ACCESSIBILITY_RECORD_KIND: &str =
    "review_component_accessibility_parity_truth";

/// Schema version for review-component accessibility parity records.
pub const REVIEW_COMPONENT_ACCESSIBILITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const REVIEW_COMPONENT_ACCESSIBILITY_SCHEMA_REF: &str =
    "schemas/ui/m5-review-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const REVIEW_COMPONENT_ACCESSIBILITY_DOC_REF: &str =
    "docs/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_freshness_queue_authority_or_approval_lineage_is_stale_or_missing_across_claimed_m5_review_components.md";

/// Repo-relative path of the frozen component matrix these claims exercise.
pub const REVIEW_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-review-request-check-queue-component-matrix.schema.json";

/// Repo-relative path of the shared-consumer parity contract this capstone extends.
pub const REVIEW_COMPONENT_ACCESSIBILITY_CONSUMER_CONTRACT_REF: &str =
    "schemas/ui/m5-review-component-consumer.schema.json";

/// Repo-relative path of the review-request-row component contract.
pub const REVIEW_COMPONENT_ACCESSIBILITY_REVIEW_REQUEST_ROW_CONTRACT_REF: &str =
    "schemas/ui/m5-review-request-row.schema.json";

/// Repo-relative path of the checks-summary-card component contract.
pub const REVIEW_COMPONENT_ACCESSIBILITY_CHECKS_SUMMARY_CARD_CONTRACT_REF: &str =
    "schemas/ui/m5-checks-summary-card.schema.json";

/// Repo-relative path of the combined merge-readiness / merge-queue / stack-dependency contract.
pub const REVIEW_COMPONENT_ACCESSIBILITY_MERGE_READINESS_PANEL_CONTRACT_REF: &str =
    "schemas/ui/m5-merge-readiness-panel.schema.json";

/// Repo-relative path of the combined pending-review-tray / approval-invalidation contract.
pub const REVIEW_COMPONENT_ACCESSIBILITY_PENDING_REVIEW_TRAY_CONTRACT_REF: &str =
    "schemas/ui/m5-pending-review-tray.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const REVIEW_COMPONENT_ACCESSIBILITY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-review-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact.
pub const REVIEW_COMPONENT_ACCESSIBILITY_ARTIFACT_REF: &str =
    "artifacts/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_freshness_queue_authority_or_approval_lineage_is_stale_or_missing_across_claimed_m5_review_components/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const REVIEW_COMPONENT_ACCESSIBILITY_SUMMARY_REF: &str =
    "artifacts/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_freshness_queue_authority_or_approval_lineage_is_stale_or_missing_across_claimed_m5_review_components.md";

/// Canonical component contract that a row must point at for a given component.
///
/// Each of the seven shared components resolves to the checked-in schema of the lane
/// that implemented it: the review-request-row, checks-summary-card, combined
/// merge-readiness panel (which also governs merge-queue entries and
/// stack-dependency chips), and combined pending-review tray (which also governs
/// approval-invalidation banners).
pub const fn component_canonical_schema_ref(component: M5ReviewComponent) -> &'static str {
    match component {
        M5ReviewComponent::ReviewRequestRow => {
            REVIEW_COMPONENT_ACCESSIBILITY_REVIEW_REQUEST_ROW_CONTRACT_REF
        }
        M5ReviewComponent::ChecksSummaryCard => {
            REVIEW_COMPONENT_ACCESSIBILITY_CHECKS_SUMMARY_CARD_CONTRACT_REF
        }
        M5ReviewComponent::MergeReadinessPanel
        | M5ReviewComponent::MergeQueueEntry
        | M5ReviewComponent::StackDependencyChip => {
            REVIEW_COMPONENT_ACCESSIBILITY_MERGE_READINESS_PANEL_CONTRACT_REF
        }
        M5ReviewComponent::PendingReviewTray | M5ReviewComponent::ApprovalInvalidationBanner => {
            REVIEW_COMPONENT_ACCESSIBILITY_PENDING_REVIEW_TRAY_CONTRACT_REF
        }
    }
}

/// The condition governing how trustworthy a component's provider-backed claim is.
///
/// [`ProviderFresh`](Self::ProviderFresh) is the baseline where the full
/// provider-backed claim is permitted. The other four are the weakening conditions
/// named by the spec: stale provider freshness, queue authority dropping to a local
/// estimate, missing approval lineage, and a required browser handoff for an
/// out-of-scope action. Each weakening condition pins the claim to a ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentClaimCondition {
    /// Provider truth is fresh; the full provider-backed claim is permitted.
    ProviderFresh,
    /// Provider-backed freshness has gone stale relative to what it gates.
    ProviderFreshnessStale,
    /// Queue authority has dropped to a local estimate.
    QueueAuthorityLocalEstimate,
    /// Approval lineage is missing and cannot be verified.
    ApprovalLineageMissing,
    /// An out-of-scope action requires a browser handoff.
    BrowserHandoffRequired,
}

impl ReviewComponentClaimCondition {
    /// Every condition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProviderFresh,
        Self::ProviderFreshnessStale,
        Self::QueueAuthorityLocalEstimate,
        Self::ApprovalLineageMissing,
        Self::BrowserHandoffRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderFresh => "provider_fresh",
            Self::ProviderFreshnessStale => "provider_freshness_stale",
            Self::QueueAuthorityLocalEstimate => "queue_authority_local_estimate",
            Self::ApprovalLineageMissing => "approval_lineage_missing",
            Self::BrowserHandoffRequired => "browser_handoff_required",
        }
    }

    /// Whether this condition weakens the provider-backed claim (everything but fresh).
    pub const fn is_weakening(self) -> bool {
        !matches!(self, Self::ProviderFresh)
    }

    /// The strongest claim tier this condition still permits.
    pub const fn permitted_ceiling(self) -> ReviewComponentClaimTier {
        match self {
            Self::ProviderFresh => ReviewComponentClaimTier::ProviderBacked,
            Self::ProviderFreshnessStale => ReviewComponentClaimTier::LocallyReviewable,
            Self::QueueAuthorityLocalEstimate => ReviewComponentClaimTier::EstimateOnly,
            Self::ApprovalLineageMissing => ReviewComponentClaimTier::ApprovalUnverified,
            Self::BrowserHandoffRequired => ReviewComponentClaimTier::HandoffRequired,
        }
    }

    /// The downgrade trigger a weakening condition must disclose, if any.
    pub const fn default_trigger(self) -> Option<ReviewComponentAccessibilityDowngradeTrigger> {
        match self {
            Self::ProviderFresh => None,
            Self::ProviderFreshnessStale => {
                Some(ReviewComponentAccessibilityDowngradeTrigger::ProviderFreshnessStale)
            }
            Self::QueueAuthorityLocalEstimate => Some(
                ReviewComponentAccessibilityDowngradeTrigger::QueueAuthorityDroppedToLocalEstimate,
            ),
            Self::ApprovalLineageMissing => {
                Some(ReviewComponentAccessibilityDowngradeTrigger::ApprovalLineageMissing)
            }
            Self::BrowserHandoffRequired => {
                Some(ReviewComponentAccessibilityDowngradeTrigger::BrowserHandoffRequired)
            }
        }
    }

    /// The next action a weakening condition's narrow disclosure must offer.
    pub const fn next_action(self) -> ReviewComponentClaimNextAction {
        match self {
            Self::ProviderFresh | Self::ProviderFreshnessStale => {
                ReviewComponentClaimNextAction::RefreshProviderFreshness
            }
            Self::QueueAuthorityLocalEstimate => {
                ReviewComponentClaimNextAction::ReconcileQueueAuthority
            }
            Self::ApprovalLineageMissing => ReviewComponentClaimNextAction::RestoreApprovalLineage,
            Self::BrowserHandoffRequired => ReviewComponentClaimNextAction::OpenBrowserHandoff,
        }
    }
}

/// A component's claim about how trustworthy its provider-backed truth is.
///
/// Ordered strongest to weakest. [`ProviderBacked`](Self::ProviderBacked) is the
/// only tier that asserts live provider-backed truth; the rest are the honest
/// fallbacks a weakening condition narrows to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentClaimTier {
    /// Live provider-backed truth.
    ProviderBacked,
    /// Reviewable in full from local truth while provider backing is degraded.
    LocallyReviewable,
    /// Queue authority is a local estimate, not the provider's own ordering.
    EstimateOnly,
    /// Approval lineage cannot be verified.
    ApprovalUnverified,
    /// The action is out of scope in-product and requires a browser handoff.
    HandoffRequired,
}

impl ReviewComponentClaimTier {
    /// Every tier, in declaration order (strongest first).
    pub const ALL: [Self; 5] = [
        Self::ProviderBacked,
        Self::LocallyReviewable,
        Self::EstimateOnly,
        Self::ApprovalUnverified,
        Self::HandoffRequired,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderBacked => "provider_backed",
            Self::LocallyReviewable => "locally_reviewable",
            Self::EstimateOnly => "estimate_only",
            Self::ApprovalUnverified => "approval_unverified",
            Self::HandoffRequired => "handoff_required",
        }
    }

    /// Strength rank, higher is stronger. Used for the ceiling comparison.
    pub const fn rank(self) -> u8 {
        match self {
            Self::ProviderBacked => 5,
            Self::LocallyReviewable => 4,
            Self::EstimateOnly => 3,
            Self::ApprovalUnverified => 2,
            Self::HandoffRequired => 1,
        }
    }

    /// Whether this tier asserts live provider-backed truth.
    pub const fn asserts_provider_backed(self) -> bool {
        matches!(self, Self::ProviderBacked)
    }
}

/// A rendering form the claim must reach with identical semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentRenderingSurface {
    /// The full desktop surface.
    DesktopFull,
    /// The headless CLI.
    CliHeadless,
    /// The support export.
    SupportExport,
}

impl ReviewComponentRenderingSurface {
    /// Every rendering surface, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopFull, Self::CliHeadless, Self::SupportExport];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// The next action a narrow disclosure offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentClaimNextAction {
    /// Refresh provider freshness.
    RefreshProviderFreshness,
    /// Reconcile queue authority against the provider.
    ReconcileQueueAuthority,
    /// Restore or recompute the approval lineage.
    RestoreApprovalLineage,
    /// Open the browser handoff.
    OpenBrowserHandoff,
    /// Continue reviewing locally.
    ContinueLocalReview,
}

impl ReviewComponentClaimNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshProviderFreshness => "refresh_provider_freshness",
            Self::ReconcileQueueAuthority => "reconcile_queue_authority",
            Self::RestoreApprovalLineage => "restore_approval_lineage",
            Self::OpenBrowserHandoff => "open_browser_handoff",
            Self::ContinueLocalReview => "continue_local_review",
        }
    }
}

/// Downgrade trigger that can narrow this accessibility lane below its full claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentAccessibilityDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Provider-backed freshness has gone stale relative to what it gates.
    ProviderFreshnessStale,
    /// Queue authority has dropped to a local estimate.
    QueueAuthorityDroppedToLocalEstimate,
    /// Approval lineage is missing.
    ApprovalLineageMissing,
    /// A browser handoff is required for an out-of-scope action.
    BrowserHandoffRequired,
    /// A claim was overstated relative to its permitted ceiling.
    ClaimOverstated,
    /// Parity across desktop, CLI, or export was dropped.
    ParityDropped,
    /// Consumer trust narrowed.
    TrustNarrowing,
}

impl ReviewComponentAccessibilityDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProviderFreshnessStale,
        Self::QueueAuthorityDroppedToLocalEstimate,
        Self::ApprovalLineageMissing,
        Self::BrowserHandoffRequired,
        Self::ClaimOverstated,
        Self::ParityDropped,
        Self::TrustNarrowing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderFreshnessStale => "provider_freshness_stale",
            Self::QueueAuthorityDroppedToLocalEstimate => {
                "queue_authority_dropped_to_local_estimate"
            }
            Self::ApprovalLineageMissing => "approval_lineage_missing",
            Self::BrowserHandoffRequired => "browser_handoff_required",
            Self::ClaimOverstated => "claim_overstated",
            Self::ParityDropped => "parity_dropped",
            Self::TrustNarrowing => "trust_narrowing",
        }
    }
}

/// The disclosures an accessibility row must carry, derived from its condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewComponentClaimResolution {
    /// The strongest claim tier the condition permits.
    pub permitted_ceiling: ReviewComponentClaimTier,
    /// Whether the condition requires an explicit narrow disclosure.
    pub requires_narrowing: bool,
    /// The downgrade trigger the narrow disclosure must name, if any.
    pub expected_trigger: Option<ReviewComponentAccessibilityDowngradeTrigger>,
    /// The next action the narrow disclosure must offer.
    pub expected_next_action: ReviewComponentClaimNextAction,
    /// Whether the row must carry an explicit browser-handoff note.
    pub needs_browser_handoff_note: bool,
    /// Whether the row must carry an explicit local-continue note.
    pub needs_local_continue_note: bool,
}

/// Resolves the claim narrowing an accessibility row must carry from its condition.
///
/// Fresh provider truth keeps the full provider-backed claim. Each weakening
/// condition pins the claim to a ceiling, demands an explicit narrow disclosure
/// naming its trigger and next action, and preserves a local-continue path so the
/// reviewer's work never vanishes. A required browser handoff additionally demands
/// an explicit handoff note rather than forcing raw-provider navigation.
pub const fn resolve_review_component_claim_narrowing(
    condition: ReviewComponentClaimCondition,
) -> ReviewComponentClaimResolution {
    ReviewComponentClaimResolution {
        permitted_ceiling: condition.permitted_ceiling(),
        requires_narrowing: condition.is_weakening(),
        expected_trigger: condition.default_trigger(),
        expected_next_action: condition.next_action(),
        needs_browser_handoff_note: matches!(
            condition,
            ReviewComponentClaimCondition::BrowserHandoffRequired
        ),
        needs_local_continue_note: condition.is_weakening(),
    }
}

/// The explicit narrow disclosure a claim-narrowed row shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentClaimNarrowing {
    /// The downgrade trigger the narrowing discloses.
    pub trigger: ReviewComponentAccessibilityDowngradeTrigger,
    /// The claim tier the narrowing pins the component to.
    pub narrowed_to: ReviewComponentClaimTier,
    /// Note naming the truth preserved through the narrowing (never omitted).
    pub preserved_truth_note: String,
    /// The next action offered.
    pub next_action: ReviewComponentClaimNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// One accessibility row: a claimed component under one condition, exposed across
/// keyboard, screen-reader, CLI, and export forms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentAccessibilityRow {
    /// Stable row id.
    pub row_id: String,
    /// Which shared component this row claims.
    pub component: M5ReviewComponent,
    /// The condition governing the claim.
    pub condition: ReviewComponentClaimCondition,
    /// The claim tier the component effectively asserts.
    pub effective_claim: ReviewComponentClaimTier,
    /// Keyboard reach / operation label (never empty).
    pub keyboard_label: String,
    /// Screen-reader label (never empty).
    pub screen_reader_label: String,
    /// CLI enum token (never empty).
    pub cli_enum_token: String,
    /// Export enum token (never empty).
    pub export_enum_token: String,
    /// Human-readable explanation field (never empty).
    pub explanation_field: String,
    /// The rendering surfaces this row reaches (must cover all three).
    pub rendering_surfaces: Vec<ReviewComponentRenderingSurface>,
    /// The explicit narrow disclosure; required and complete when the claim narrows.
    pub narrowing: Option<ReviewComponentClaimNarrowing>,
    /// Local-continue note; required and non-empty when the disclosure demands it.
    pub local_continue_note: String,
    /// Browser-handoff note; required and non-empty when the disclosure demands it.
    pub browser_handoff_note: String,
    /// Guardrail: this component is reachable only by pointer.
    pub is_pointer_only: bool,
    /// Guardrail: this component omits itself from the export.
    pub is_export_opaque: bool,
    /// Guardrail: this component claims more on the desktop than in CLI or export.
    pub desktop_stronger_than_cli: bool,
    /// Source contract refs this row points at.
    pub source_contract_refs: Vec<String>,
}

impl ReviewComponentAccessibilityRow {
    /// The disclosures this row must carry, derived from its condition.
    pub const fn resolution(&self) -> ReviewComponentClaimResolution {
        resolve_review_component_claim_narrowing(self.condition)
    }

    /// Whether this row narrows below the full provider-backed claim.
    pub const fn is_narrowed(&self) -> bool {
        self.condition.is_weakening()
    }

    /// Whether this row reaches all three rendering surfaces.
    pub fn covers_all_rendering_surfaces(&self) -> bool {
        ReviewComponentRenderingSurface::ALL
            .iter()
            .all(|surface| self.rendering_surfaces.contains(surface))
    }

    /// Whether every accessibility field is present.
    pub fn accessibility_fields_present(&self) -> bool {
        !self.keyboard_label.trim().is_empty()
            && !self.screen_reader_label.trim().is_empty()
            && !self.cli_enum_token.trim().is_empty()
            && !self.export_enum_token.trim().is_empty()
            && !self.explanation_field.trim().is_empty()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.is_pointer_only && !self.is_export_opaque && !self.desktop_stronger_than_cli
    }

    /// Whether this row points at the canonical component schema and matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = component_canonical_schema_ref(self.component);
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self.source_contract_refs.iter().any(|reference| {
                reference == REVIEW_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF
            })
    }

    /// Whether the effective claim is honest under the row's condition: it never
    /// exceeds the permitted ceiling, and a weakening condition narrows the claim
    /// down to exactly that ceiling.
    pub fn claim_is_honest(&self) -> bool {
        let resolution = self.resolution();
        let ceiling = resolution.permitted_ceiling;
        if self.effective_claim.rank() > ceiling.rank() {
            return false;
        }
        if resolution.requires_narrowing {
            self.effective_claim == ceiling
                && self
                    .narrowing
                    .as_ref()
                    .is_some_and(|narrowing| narrowing.narrowed_to == ceiling)
        } else {
            self.effective_claim == ceiling && self.narrowing.is_none()
        }
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentAccessibilityTrustReview {
    /// Every claim is keyboard-reachable.
    pub keyboard_reachable_on_every_claim: bool,
    /// Every claim carries a screen-reader label.
    pub screen_reader_labeled_on_every_claim: bool,
    /// Every claim exposes a CLI enum token.
    pub cli_enum_exposed_on_every_claim: bool,
    /// Every claim exposes an export enum token.
    pub export_enum_exposed_on_every_claim: bool,
    /// Every claim carries an explanation field.
    pub explanation_field_present_on_every_claim: bool,
    /// No component is pointer-only.
    pub no_component_pointer_only: bool,
    /// No component is export-opaque.
    pub no_component_export_opaque: bool,
    /// No component claims more on the desktop than in CLI or export.
    pub desktop_never_stronger_than_cli: bool,
    /// The claim narrows whenever provider backing weakens.
    pub claim_narrows_when_provider_backing_weakens: bool,
    /// Provider-backed truth is never overstated while a weakening condition holds.
    pub provider_backed_never_overstated_under_weakening: bool,
    /// Browser handoff is kept explicit.
    pub browser_handoff_kept_explicit: bool,
    /// Local continuation is preserved when provider backing is degraded.
    pub local_continue_preserved_under_degraded_backing: bool,
}

impl ReviewComponentAccessibilityTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.keyboard_reachable_on_every_claim
            && self.screen_reader_labeled_on_every_claim
            && self.cli_enum_exposed_on_every_claim
            && self.export_enum_exposed_on_every_claim
            && self.explanation_field_present_on_every_claim
            && self.no_component_pointer_only
            && self.no_component_export_opaque
            && self.desktop_never_stronger_than_cli
            && self.claim_narrows_when_provider_backing_weakens
            && self.provider_backed_never_overstated_under_weakening
            && self.browser_handoff_kept_explicit
            && self.local_continue_preserved_under_degraded_backing
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentAccessibilityProjection {
    /// Keyboard and screen-reader labels are exposed.
    pub exposes_keyboard_and_screen_reader_labels: bool,
    /// CLI and export enums are exposed.
    pub exposes_cli_and_export_enums: bool,
    /// Explanation fields are exposed.
    pub exposes_explanation_fields: bool,
    /// The claim auto-narrows on stale provider freshness.
    pub auto_narrows_on_stale_freshness: bool,
    /// The claim auto-narrows when queue authority drops to a local estimate.
    pub auto_narrows_on_local_estimate_queue_authority: bool,
    /// The claim auto-narrows when approval lineage is missing.
    pub auto_narrows_on_missing_approval_lineage: bool,
    /// The claim auto-narrows when a browser handoff is required.
    pub auto_narrows_on_required_browser_handoff: bool,
    /// Desktop, CLI, and export semantics are identical.
    pub desktop_cli_export_semantics_identical: bool,
    /// Narrowing prevents overstated provider-backed truth.
    pub narrowing_prevents_overstated_provider_truth: bool,
    /// Every component is reachable non-visually.
    pub every_component_reachable_non_visually: bool,
}

impl ReviewComponentAccessibilityProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.exposes_keyboard_and_screen_reader_labels
            && self.exposes_cli_and_export_enums
            && self.exposes_explanation_fields
            && self.auto_narrows_on_stale_freshness
            && self.auto_narrows_on_local_estimate_queue_authority
            && self.auto_narrows_on_missing_approval_lineage
            && self.auto_narrows_on_required_browser_handoff
            && self.desktop_cli_export_semantics_identical
            && self.narrowing_prevents_overstated_provider_truth
            && self.every_component_reachable_non_visually
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentAccessibilityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ReviewComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewComponentAccessibilityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Accessibility rows.
    pub accessibility_rows: Vec<ReviewComponentAccessibilityRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ReviewComponentAccessibilityDowngradeTrigger>,
    /// Rendering surfaces this packet covers.
    pub rendering_surfaces: Vec<ReviewComponentRenderingSurface>,
    /// Trust review block.
    pub trust_review: ReviewComponentAccessibilityTrustReview,
    /// Consumer projection block.
    pub projection: ReviewComponentAccessibilityProjection,
    /// Proof freshness block.
    pub proof_freshness: ReviewComponentAccessibilityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe review-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentAccessibilityPacket {
    /// Record kind; must equal [`REVIEW_COMPONENT_ACCESSIBILITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`REVIEW_COMPONENT_ACCESSIBILITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Accessibility rows.
    pub accessibility_rows: Vec<ReviewComponentAccessibilityRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ReviewComponentAccessibilityDowngradeTrigger>,
    /// Rendering surfaces this packet covers.
    pub rendering_surfaces: Vec<ReviewComponentRenderingSurface>,
    /// Trust review block.
    pub trust_review: ReviewComponentAccessibilityTrustReview,
    /// Consumer projection block.
    pub projection: ReviewComponentAccessibilityProjection,
    /// Proof freshness block.
    pub proof_freshness: ReviewComponentAccessibilityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ReviewComponentAccessibilityPacket {
    /// Builds a review-component accessibility packet from stable-lane input.
    pub fn new(input: ReviewComponentAccessibilityPacketInput) -> Self {
        Self {
            record_kind: REVIEW_COMPONENT_ACCESSIBILITY_RECORD_KIND.to_owned(),
            schema_version: REVIEW_COMPONENT_ACCESSIBILITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            accessibility_rows: input.accessibility_rows,
            downgrade_triggers: input.downgrade_triggers,
            rendering_surfaces: input.rendering_surfaces,
            trust_review: input.trust_review,
            projection: input.projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the review-component accessibility parity invariants.
    pub fn validate(&self) -> Vec<ReviewComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != REVIEW_COMPONENT_ACCESSIBILITY_RECORD_KIND {
            violations.push(ReviewComponentAccessibilityViolation::WrongRecordKind);
        }
        if self.schema_version != REVIEW_COMPONENT_ACCESSIBILITY_SCHEMA_VERSION {
            violations.push(ReviewComponentAccessibilityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ReviewComponentAccessibilityViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ReviewComponentAccessibilityViolation::DowngradeTriggersMissing);
        }
        if self.rendering_surfaces.is_empty() {
            violations.push(ReviewComponentAccessibilityViolation::RenderingSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(ReviewComponentAccessibilityViolation::TrustReviewIncomplete);
        }
        if !self.projection.all_hold() {
            violations.push(ReviewComponentAccessibilityViolation::ProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ReviewComponentAccessibilityViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("review-component accessibility packet serializes"),
        ) {
            violations.push(ReviewComponentAccessibilityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("review-component accessibility packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .accessibility_rows
            .iter()
            .filter(|row| row.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str("# Review-Component Accessibility, Headless, and Export Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Accessibility rows: {} ({} claim-narrowed)\n",
            self.accessibility_rows.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Accessibility rows\n\n");
        for row in &self.accessibility_rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: condition `{}`, claim `{}`\n",
                row.component.as_str(),
                row.row_id,
                row.condition.as_str(),
                row.effective_claim.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in review-component accessibility export.
#[derive(Debug)]
pub enum ReviewComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ReviewComponentAccessibilityViolation>),
}

impl fmt::Display for ReviewComponentAccessibilityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "review-component accessibility export parse failed: {error}"
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
                    "review-component accessibility export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ReviewComponentAccessibilityArtifactError {}

/// Validation failures emitted by [`ReviewComponentAccessibilityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReviewComponentAccessibilityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No accessibility rows are present.
    AccessibilityRowsMissing,
    /// An accessibility row is incomplete.
    RowIncomplete,
    /// A row is missing its keyboard label.
    KeyboardLabelMissing,
    /// A row is missing its screen-reader label.
    ScreenReaderLabelMissing,
    /// A row is missing its CLI enum token.
    CliEnumTokenMissing,
    /// A row is missing its export enum token.
    ExportEnumTokenMissing,
    /// A row is missing its explanation field.
    ExplanationFieldMissing,
    /// A row does not reach all three rendering surfaces.
    RenderingSurfaceCoverageMissing,
    /// A component is reachable only by pointer.
    PointerOnlyComponent,
    /// A component omits itself from the export.
    ExportOpaqueComponent,
    /// A component claims more on the desktop than in CLI or export.
    DesktopStrongerThanCli,
    /// A row's effective claim exceeds the ceiling its condition permits.
    ClaimCeilingExceeded,
    /// A weakening condition is missing its explicit narrow disclosure.
    ClaimNarrowingMissing,
    /// A baseline condition unexpectedly carries a narrow disclosure.
    ClaimNarrowingUnexpected,
    /// A narrow disclosure pins the claim to the wrong tier.
    NarrowedToMismatch,
    /// A narrow disclosure names the wrong trigger.
    NarrowTriggerMismatch,
    /// A narrow disclosure offers the wrong next action.
    NarrowNextActionMismatch,
    /// A narrow disclosure is missing its preserved-truth note.
    NarrowPreservedTruthMissing,
    /// A narrow disclosure is missing its next-action copy.
    NarrowNextActionMissing,
    /// A row that needs an explicit browser-handoff note is missing it.
    BrowserHandoffNoteMissing,
    /// A row that must preserve a local-continue path is missing its note.
    LocalContinueNoteMissing,
    /// A row does not point at the canonical component and matrix contracts.
    CanonicalContractReferenceMissing,
    /// Not every shared component appears among the rows.
    ComponentCoverageMissing,
    /// Not every claim condition appears among the rows.
    ConditionCoverageMissing,
    /// Not every claim tier appears as an effective claim.
    ClaimTierCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No rendering surfaces are present.
    RenderingSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ReviewComponentAccessibilityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::AccessibilityRowsMissing => "accessibility_rows_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::KeyboardLabelMissing => "keyboard_label_missing",
            Self::ScreenReaderLabelMissing => "screen_reader_label_missing",
            Self::CliEnumTokenMissing => "cli_enum_token_missing",
            Self::ExportEnumTokenMissing => "export_enum_token_missing",
            Self::ExplanationFieldMissing => "explanation_field_missing",
            Self::RenderingSurfaceCoverageMissing => "rendering_surface_coverage_missing",
            Self::PointerOnlyComponent => "pointer_only_component",
            Self::ExportOpaqueComponent => "export_opaque_component",
            Self::DesktopStrongerThanCli => "desktop_stronger_than_cli",
            Self::ClaimCeilingExceeded => "claim_ceiling_exceeded",
            Self::ClaimNarrowingMissing => "claim_narrowing_missing",
            Self::ClaimNarrowingUnexpected => "claim_narrowing_unexpected",
            Self::NarrowedToMismatch => "narrowed_to_mismatch",
            Self::NarrowTriggerMismatch => "narrow_trigger_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowPreservedTruthMissing => "narrow_preserved_truth_missing",
            Self::NarrowNextActionMissing => "narrow_next_action_missing",
            Self::BrowserHandoffNoteMissing => "browser_handoff_note_missing",
            Self::LocalContinueNoteMissing => "local_continue_note_missing",
            Self::CanonicalContractReferenceMissing => "canonical_contract_reference_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::ConditionCoverageMissing => "condition_coverage_missing",
            Self::ClaimTierCoverageMissing => "claim_tier_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RenderingSurfacesMissing => "rendering_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ProjectionIncomplete => "projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable review-component accessibility export.
pub fn current_review_component_accessibility_export(
) -> Result<ReviewComponentAccessibilityPacket, ReviewComponentAccessibilityArtifactError> {
    let packet: ReviewComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_freshness_queue_authority_or_approval_lineage_is_stale_or_missing_across_claimed_m5_review_components/support_export.json"
    )))
    .map_err(ReviewComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ReviewComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &ReviewComponentAccessibilityPacket,
    violations: &mut Vec<ReviewComponentAccessibilityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        REVIEW_COMPONENT_ACCESSIBILITY_SCHEMA_REF,
        REVIEW_COMPONENT_ACCESSIBILITY_DOC_REF,
        REVIEW_COMPONENT_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF,
        REVIEW_COMPONENT_ACCESSIBILITY_CONSUMER_CONTRACT_REF,
        REVIEW_COMPONENT_ACCESSIBILITY_REVIEW_REQUEST_ROW_CONTRACT_REF,
        REVIEW_COMPONENT_ACCESSIBILITY_CHECKS_SUMMARY_CARD_CONTRACT_REF,
        REVIEW_COMPONENT_ACCESSIBILITY_MERGE_READINESS_PANEL_CONTRACT_REF,
        REVIEW_COMPONENT_ACCESSIBILITY_PENDING_REVIEW_TRAY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ReviewComponentAccessibilityViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &ReviewComponentAccessibilityPacket,
    violations: &mut Vec<ReviewComponentAccessibilityViolation>,
) {
    if packet.accessibility_rows.is_empty() {
        violations.push(ReviewComponentAccessibilityViolation::AccessibilityRowsMissing);
        return;
    }

    let mut seen_components: BTreeSet<M5ReviewComponent> = BTreeSet::new();
    let mut seen_conditions: BTreeSet<ReviewComponentClaimCondition> = BTreeSet::new();
    let mut seen_tiers: BTreeSet<ReviewComponentClaimTier> = BTreeSet::new();

    for row in &packet.accessibility_rows {
        if row.row_id.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(ReviewComponentAccessibilityViolation::RowIncomplete);
        }

        if row.keyboard_label.trim().is_empty() {
            violations.push(ReviewComponentAccessibilityViolation::KeyboardLabelMissing);
        }
        if row.screen_reader_label.trim().is_empty() {
            violations.push(ReviewComponentAccessibilityViolation::ScreenReaderLabelMissing);
        }
        if row.cli_enum_token.trim().is_empty() {
            violations.push(ReviewComponentAccessibilityViolation::CliEnumTokenMissing);
        }
        if row.export_enum_token.trim().is_empty() {
            violations.push(ReviewComponentAccessibilityViolation::ExportEnumTokenMissing);
        }
        if row.explanation_field.trim().is_empty() {
            violations.push(ReviewComponentAccessibilityViolation::ExplanationFieldMissing);
        }

        if !row.covers_all_rendering_surfaces() {
            violations.push(ReviewComponentAccessibilityViolation::RenderingSurfaceCoverageMissing);
        }

        // AC1 guardrails: parity across desktop, CLI, and export.
        if row.is_pointer_only {
            violations.push(ReviewComponentAccessibilityViolation::PointerOnlyComponent);
        }
        if row.is_export_opaque {
            violations.push(ReviewComponentAccessibilityViolation::ExportOpaqueComponent);
        }
        if row.desktop_stronger_than_cli {
            violations.push(ReviewComponentAccessibilityViolation::DesktopStrongerThanCli);
        }

        let resolution = row.resolution();
        let ceiling = resolution.permitted_ceiling;

        // AC2 core: a claim may never exceed the ceiling its condition permits.
        if row.effective_claim.rank() > ceiling.rank() {
            violations.push(ReviewComponentAccessibilityViolation::ClaimCeilingExceeded);
        }

        // Narrow-disclosure presence and completeness.
        if resolution.requires_narrowing {
            match &row.narrowing {
                None => {
                    violations.push(ReviewComponentAccessibilityViolation::ClaimNarrowingMissing);
                }
                Some(narrowing) => {
                    if narrowing.narrowed_to != ceiling {
                        violations.push(ReviewComponentAccessibilityViolation::NarrowedToMismatch);
                    }
                    if Some(narrowing.trigger) != resolution.expected_trigger {
                        violations
                            .push(ReviewComponentAccessibilityViolation::NarrowTriggerMismatch);
                    }
                    if narrowing.next_action != resolution.expected_next_action {
                        violations
                            .push(ReviewComponentAccessibilityViolation::NarrowNextActionMismatch);
                    }
                    if narrowing.preserved_truth_note.trim().is_empty() {
                        violations.push(
                            ReviewComponentAccessibilityViolation::NarrowPreservedTruthMissing,
                        );
                    }
                    if narrowing.next_action_label.trim().is_empty() {
                        violations
                            .push(ReviewComponentAccessibilityViolation::NarrowNextActionMissing);
                    }
                }
            }
        } else if row.narrowing.is_some() {
            violations.push(ReviewComponentAccessibilityViolation::ClaimNarrowingUnexpected);
        }

        if resolution.needs_browser_handoff_note && row.browser_handoff_note.trim().is_empty() {
            violations.push(ReviewComponentAccessibilityViolation::BrowserHandoffNoteMissing);
        }
        if resolution.needs_local_continue_note && row.local_continue_note.trim().is_empty() {
            violations.push(ReviewComponentAccessibilityViolation::LocalContinueNoteMissing);
        }

        if !row.points_at_canonical_contracts() {
            violations
                .push(ReviewComponentAccessibilityViolation::CanonicalContractReferenceMissing);
        }

        seen_components.insert(row.component);
        seen_conditions.insert(row.condition);
        seen_tiers.insert(row.effective_claim);
    }

    // Coverage: every component, every condition, and every claim tier must appear.
    for component in M5ReviewComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(ReviewComponentAccessibilityViolation::ComponentCoverageMissing);
            break;
        }
    }
    for condition in ReviewComponentClaimCondition::ALL {
        if !seen_conditions.contains(&condition) {
            violations.push(ReviewComponentAccessibilityViolation::ConditionCoverageMissing);
            break;
        }
    }
    for tier in ReviewComponentClaimTier::ALL {
        if !seen_tiers.contains(&tier) {
            violations.push(ReviewComponentAccessibilityViolation::ClaimTierCoverageMissing);
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
