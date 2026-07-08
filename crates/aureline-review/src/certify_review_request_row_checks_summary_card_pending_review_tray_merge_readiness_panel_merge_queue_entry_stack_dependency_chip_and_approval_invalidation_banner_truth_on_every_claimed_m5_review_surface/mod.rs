//! Surface certification of review-request-row, checks-summary-card,
//! pending-review-tray, merge-readiness-panel, merge-queue-entry,
//! stack-dependency-chip, and approval-invalidation-banner truth on every claimed
//! M5 review surface.
//!
//! This module is the closing certification capstone over the seven shared review
//! components frozen in
//! [`crate::freeze_the_m5_review_request_check_and_merge_queue_component_matrix`],
//! implemented by the review-request-row, checks-summary-card, merge-readiness /
//! merge-queue / stack-dependency, and pending-review-tray /
//! approval-invalidation-banner lanes, adopted by the shared consumers in
//! [`crate::add_shared_review_list_detail_companion_help_support_and_export_consumers_so_review_components_keep_label_action_and_handoff_parity`],
//! and proven across assistive, headless, and exported forms by
//! [`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_provider_freshness_queue_authority_or_approval_lineage_is_stale_or_missing_across_claimed_m5_review_components`].
//!
//! Where the implement lanes ship the components and the consumer lane proves
//! label / action / handoff parity, this lane certifies the release claim: that on
//! every claimed M5 review surface — desktop list, detail pane, companion queue,
//! help surface, support export, exported review packet, headless CLI, and
//! diagnostics — the same controlled component truth is presented with no hidden
//! provider drift. Each certified surface row scores six certification axes
//! ([`ReviewComponentCertificationAxis`]): the visual, keyboard, screen-reader, and
//! CLI/export axes that every claim must always pass, the degraded-state axis that
//! narrows a claim when provider freshness, queue authority, approval lineage, or a
//! stack relation weakens, and the provider/local-provenance axis that keeps the
//! certification honest — a certified surface never implies its provider-backed
//! truth is fresh or authoritative.
//!
//! A surface earns [`ReviewComponentSurfaceClaimStatus::CertifiedParity`] only when
//! its certified claim equals its claimed claim, no axis narrows, and component
//! truth is preserved. It narrows to
//! [`ReviewComponentSurfaceClaimStatus::NarrowedParity`] the moment an axis narrows
//! or the certified claim drops below the claimed one, and it fails to
//! [`ReviewComponentSurfaceClaimStatus::ParityBlocked`] whenever the provider /
//! local distinction, queue owner, check class, approval invalidation, or freshness
//! truth is flattened out of the export. That last rule is the delta of this
//! capstone: certification may narrow a claim but may never drop the component's
//! meaning.
//!
//! The packet references upstream component, consumer, and accessibility contracts
//! by id rather than embedding their content. Raw provider responses, credentials,
//! and live provider payloads stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-review-request-check-queue-component-certification.schema.json`](../../../../schemas/ui/m5-review-request-check-queue-component-certification.schema.json).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{M5ReviewComponent, ReviewComponentClaimTier};

/// Stable record-kind tag carried by [`ReviewComponentCertificationPacket`].
pub const M5_REVIEW_COMPONENT_CERTIFICATION_RECORD_KIND: &str =
    "m5_review_component_surface_certification_truth";

/// Schema version for review-component surface certification records.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/ui/m5-review-request-check-queue-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_DOC_REF: &str =
    "docs/review/m5/certify_review_request_row_checks_summary_card_pending_review_tray_merge_readiness_panel_merge_queue_entry_stack_dependency_chip_and_approval_invalidation_banner_truth_on_every_claimed_m5_review_surface.md";

/// Repo-relative path of the frozen component matrix this certification builds on.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-review-request-check-queue-component-matrix.schema.json";

/// Repo-relative path of the shared-consumer parity contract this certification builds on.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_CONSUMER_CONTRACT_REF: &str =
    "schemas/ui/m5-review-component-consumer.schema.json";

/// Repo-relative path of the accessibility / headless / export parity contract this certification builds on.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF: &str =
    "schemas/ui/m5-review-component-accessibility-parity.schema.json";

/// Repo-relative path of the review-request-row component contract.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_REVIEW_REQUEST_ROW_CONTRACT_REF: &str =
    "schemas/ui/m5-review-request-row.schema.json";

/// Repo-relative path of the checks-summary-card component contract.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_CHECKS_SUMMARY_CARD_CONTRACT_REF: &str =
    "schemas/ui/m5-checks-summary-card.schema.json";

/// Repo-relative path of the combined merge-readiness / merge-queue / stack-dependency contract.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_MERGE_READINESS_PANEL_CONTRACT_REF: &str =
    "schemas/ui/m5-merge-readiness-panel.schema.json";

/// Repo-relative path of the combined pending-review-tray / approval-invalidation contract.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_PENDING_REVIEW_TRAY_CONTRACT_REF: &str =
    "schemas/ui/m5-pending-review-tray.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/ui/m5-review-request-check-queue-component-certification";

/// Repo-relative path of the checked support-export artifact.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/review/m5/certify_review_request_row_checks_summary_card_pending_review_tray_merge_readiness_panel_merge_queue_entry_stack_dependency_chip_and_approval_invalidation_banner_truth_on_every_claimed_m5_review_surface/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/review/m5/certify_review_request_row_checks_summary_card_pending_review_tray_merge_readiness_panel_merge_queue_entry_stack_dependency_chip_and_approval_invalidation_banner_truth_on_every_claimed_m5_review_surface.md";

/// Repo-relative path of the release-proof support export.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_RELEASE_PROOF_ARTIFACT_REF: &str =
    "artifacts/release/m5-review-request-check-queue-certification-proof/support_export.json";

/// Repo-relative path of the release-proof certification matrix CSV.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_RELEASE_PROOF_MATRIX_REF: &str =
    "artifacts/release/m5-review-request-check-queue-certification-proof/matrix.csv";

/// Repo-relative path of the release-proof report.
pub const M5_REVIEW_COMPONENT_CERTIFICATION_RELEASE_PROOF_REPORT_REF: &str =
    "artifacts/release/m5-review-request-check-queue-certification-proof/report.md";

/// Canonical component contract that a certified surface row must cite for a
/// component it presents.
///
/// Each of the seven shared components resolves to the checked-in schema of the
/// lane that implemented it: the review-request-row, checks-summary-card, combined
/// merge-readiness panel (which also governs merge-queue entries and
/// stack-dependency chips), and combined pending-review tray (which also governs
/// approval-invalidation banners).
pub const fn certification_component_canonical_schema_ref(
    component: M5ReviewComponent,
) -> &'static str {
    match component {
        M5ReviewComponent::ReviewRequestRow => {
            M5_REVIEW_COMPONENT_CERTIFICATION_REVIEW_REQUEST_ROW_CONTRACT_REF
        }
        M5ReviewComponent::ChecksSummaryCard => {
            M5_REVIEW_COMPONENT_CERTIFICATION_CHECKS_SUMMARY_CARD_CONTRACT_REF
        }
        M5ReviewComponent::MergeReadinessPanel
        | M5ReviewComponent::MergeQueueEntry
        | M5ReviewComponent::StackDependencyChip => {
            M5_REVIEW_COMPONENT_CERTIFICATION_MERGE_READINESS_PANEL_CONTRACT_REF
        }
        M5ReviewComponent::PendingReviewTray | M5ReviewComponent::ApprovalInvalidationBanner => {
            M5_REVIEW_COMPONENT_CERTIFICATION_PENDING_REVIEW_TRAY_CONTRACT_REF
        }
    }
}

/// A claimed M5 review surface whose component truth this packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewComponentCertifiedSurface {
    /// Desktop review list (list of review-request rows).
    DesktopReviewList,
    /// Review detail pane (opened review, checks, merge readiness, stack).
    ReviewDetailPane,
    /// Companion review queue (pending review trays and merge-queue triage).
    CompanionReviewQueue,
    /// Help / About review surface.
    HelpReviewSurface,
    /// Support export bundle.
    SupportExport,
    /// Exported review packet (offline / publish-later review pack).
    ExportedReviewPacket,
    /// Headless CLI review output.
    CliHeadless,
    /// Diagnostics review surface.
    Diagnostics,
}

impl M5ReviewComponentCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DesktopReviewList,
        Self::ReviewDetailPane,
        Self::CompanionReviewQueue,
        Self::HelpReviewSurface,
        Self::SupportExport,
        Self::ExportedReviewPacket,
        Self::CliHeadless,
        Self::Diagnostics,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopReviewList => "desktop_review_list",
            Self::ReviewDetailPane => "review_detail_pane",
            Self::CompanionReviewQueue => "companion_review_queue",
            Self::HelpReviewSurface => "help_review_surface",
            Self::SupportExport => "support_export",
            Self::ExportedReviewPacket => "exported_review_packet",
            Self::CliHeadless => "cli_headless",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// A certification axis scored on every certified surface row.
///
/// The first four axes are always-on: a claimed component must always pass them on
/// every surface. [`DegradedState`](Self::DegradedState) narrows a claim when
/// provider freshness, queue authority, approval lineage, or a stack relation
/// weakens. [`ProviderLocalProvenance`](Self::ProviderLocalProvenance) is the
/// certification-specific separation axis: it keeps the provider-backed-vs-local
/// distinction explicit so a certified surface never implies its provider truth is
/// fresh or authoritative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentCertificationAxis {
    /// Visual rendering carries the controlled component truth.
    Visual,
    /// Keyboard reach and operation carry the controlled component truth.
    Keyboard,
    /// Screen-reader labelling carries the controlled component truth.
    ScreenReader,
    /// CLI and export forms carry the controlled component truth.
    CliExport,
    /// Degraded provider / queue / approval / stack state narrows the claim honestly.
    DegradedState,
    /// The provider-backed-vs-local distinction stays explicit; certified never implies fresh.
    ProviderLocalProvenance,
}

impl ReviewComponentCertificationAxis {
    /// Every axis, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Visual,
        Self::Keyboard,
        Self::ScreenReader,
        Self::CliExport,
        Self::DegradedState,
        Self::ProviderLocalProvenance,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::ProviderLocalProvenance => "provider_local_provenance",
        }
    }

    /// Whether this axis must always be certified on every claimed surface.
    pub const fn is_always_on(self) -> bool {
        matches!(
            self,
            Self::Visual | Self::Keyboard | Self::ScreenReader | Self::CliExport
        )
    }
}

/// The certification state of a single axis on a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentAxisCertificationState {
    /// The axis is fully certified on this surface.
    Certified,
    /// The axis is certified but narrowed (an honest fallback is disclosed).
    NarrowedCertified,
    /// The axis is not certified on this surface (it is honestly out of scope here).
    NotCertifiedHere,
}

impl ReviewComponentAxisCertificationState {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedCertified => "narrowed_certified",
            Self::NotCertifiedHere => "not_certified_here",
        }
    }
}

/// The certification status a surface row earns.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentSurfaceClaimStatus {
    /// Green: certified claim equals claimed claim, no axis narrows, truth preserved.
    CertifiedParity,
    /// Yellow: certification is narrowed but component truth is preserved.
    NarrowedParity,
    /// Red: component truth was flattened out of this surface.
    ParityBlocked,
}

impl ReviewComponentSurfaceClaimStatus {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedParity => "certified_parity",
            Self::NarrowedParity => "narrowed_parity",
            Self::ParityBlocked => "parity_blocked",
        }
    }

    /// Whether the surface is fully certified (green).
    pub const fn is_green(self) -> bool {
        matches!(self, Self::CertifiedParity)
    }

    /// Whether the surface is blocked (red).
    pub const fn is_red(self) -> bool {
        matches!(self, Self::ParityBlocked)
    }
}

/// Downgrade trigger that can narrow a certified surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComponentCertificationDowngradeTrigger {
    /// Proof packet has gone stale relative to its freshness SLO.
    ProofStale,
    /// An upstream evidence packet failed validation or is missing.
    EvidencePacketInvalid,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Provider-backed freshness has gone stale relative to what it gates.
    ProviderFreshnessStale,
    /// Queue authority has dropped to a local estimate.
    QueueAuthorityLocalEstimate,
    /// Approval lineage is missing and cannot be verified.
    ApprovalLineageMissing,
    /// An out-of-scope action requires a browser handoff.
    BrowserHandoffRequired,
    /// A stack / restack relation drifted and is unresolved.
    StackDriftUnresolved,
    /// Consumer or workspace trust narrowed.
    TrustNarrowing,
    /// An upstream dependency row narrowed.
    UpstreamDependencyNarrowed,
}

impl ReviewComponentCertificationDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::EvidencePacketInvalid,
        Self::PolicyBlocked,
        Self::ProviderFreshnessStale,
        Self::QueueAuthorityLocalEstimate,
        Self::ApprovalLineageMissing,
        Self::BrowserHandoffRequired,
        Self::StackDriftUnresolved,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::EvidencePacketInvalid => "evidence_packet_invalid",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProviderFreshnessStale => "provider_freshness_stale",
            Self::QueueAuthorityLocalEstimate => "queue_authority_local_estimate",
            Self::ApprovalLineageMissing => "approval_lineage_missing",
            Self::BrowserHandoffRequired => "browser_handoff_required",
            Self::StackDriftUnresolved => "stack_drift_unresolved",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Derives the certification status of a surface from its claims and axis narrowing.
///
/// Component truth is the hard gate: if the provider / local distinction, queue
/// owner, check class, approval invalidation, or freshness truth is flattened, the
/// surface is [`ReviewComponentSurfaceClaimStatus::ParityBlocked`] regardless of
/// the claim tiers. Otherwise a certified claim below the claimed one, or any
/// narrowed axis, narrows the surface to
/// [`ReviewComponentSurfaceClaimStatus::NarrowedParity`]; only a full, un-narrowed
/// claim earns [`ReviewComponentSurfaceClaimStatus::CertifiedParity`].
pub const fn derive_review_component_surface_claim_status(
    claimed: ReviewComponentClaimTier,
    certified: ReviewComponentClaimTier,
    component_truth_preserved: bool,
    has_narrowed_axes: bool,
) -> ReviewComponentSurfaceClaimStatus {
    if !component_truth_preserved {
        ReviewComponentSurfaceClaimStatus::ParityBlocked
    } else if certified.rank() < claimed.rank() || has_narrowed_axes {
        ReviewComponentSurfaceClaimStatus::NarrowedParity
    } else {
        ReviewComponentSurfaceClaimStatus::CertifiedParity
    }
}

/// One axis outcome on a certified surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentCertAxisOutcome {
    /// The certification axis scored.
    pub axis: ReviewComponentCertificationAxis,
    /// The state the axis earned on this surface.
    pub state: ReviewComponentAxisCertificationState,
    /// Human-readable note explaining the outcome (never empty).
    pub note: String,
}

/// One certified surface row: a claimed M5 review surface and the component truth
/// it presents, scored across the six certification axes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentCertifiedSurfaceRow {
    /// Stable row id.
    pub row_id: String,
    /// The claimed M5 review surface.
    pub surface: M5ReviewComponentCertifiedSurface,
    /// The shared components this surface presents (non-empty).
    pub components_present: Vec<M5ReviewComponent>,
    /// The claim tier the surface claims for its components.
    pub claimed_claim: ReviewComponentClaimTier,
    /// The claim tier the certification actually earns.
    pub certified_claim: ReviewComponentClaimTier,
    /// The certification status the surface earns.
    pub status: ReviewComponentSurfaceClaimStatus,
    /// Per-axis outcomes; must cover all six axes.
    pub axis_outcomes: Vec<ReviewComponentCertAxisOutcome>,
    /// The axes that narrowed on this surface (subset of the axis outcomes).
    pub narrowed_axes: Vec<ReviewComponentCertificationAxis>,
    /// The downgrade trigger disclosed when the surface narrows.
    pub downgrade_trigger: Option<ReviewComponentCertificationDowngradeTrigger>,
    /// Delta invariant: the component's provider / local, queue, check-class,
    /// approval-invalidation, and freshness truth is preserved (never flattened).
    pub component_truth_preserved: bool,
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
    /// Source contract refs this row points at.
    pub source_contract_refs: Vec<String>,
}

impl ReviewComponentCertifiedSurfaceRow {
    /// The status this row should carry, derived from its claims and narrowing.
    pub fn derived_status(&self) -> ReviewComponentSurfaceClaimStatus {
        derive_review_component_surface_claim_status(
            self.claimed_claim,
            self.certified_claim,
            self.component_truth_preserved,
            !self.narrowed_axes.is_empty(),
        )
    }

    /// Whether the recorded status matches the derived one.
    pub fn status_is_consistent(&self) -> bool {
        self.status == self.derived_status()
    }

    /// Whether every axis is scored on this row.
    pub fn covers_all_axes(&self) -> bool {
        ReviewComponentCertificationAxis::ALL.iter().all(|axis| {
            self.axis_outcomes
                .iter()
                .any(|outcome| outcome.axis == *axis)
        })
    }

    /// Whether every parity / export field is present.
    pub fn parity_fields_present(&self) -> bool {
        !self.keyboard_label.trim().is_empty()
            && !self.screen_reader_label.trim().is_empty()
            && !self.cli_enum_token.trim().is_empty()
            && !self.export_enum_token.trim().is_empty()
            && !self.explanation_field.trim().is_empty()
    }

    /// Whether the certified claim stays at or below the claimed one.
    pub fn certified_claim_within_claimed(&self) -> bool {
        self.certified_claim.rank() <= self.claimed_claim.rank()
    }

    /// Whether the narrowed axes agree with the axis outcomes marked narrowed.
    pub fn narrowed_axes_consistent(&self) -> bool {
        let narrowed: BTreeSet<ReviewComponentCertificationAxis> =
            self.narrowed_axes.iter().copied().collect();
        for outcome in &self.axis_outcomes {
            let marked_narrowed =
                outcome.state == ReviewComponentAxisCertificationState::NarrowedCertified;
            if marked_narrowed != narrowed.contains(&outcome.axis) {
                return false;
            }
        }
        true
    }

    /// Whether this row cites the canonical matrix and each present component's schema.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_REVIEW_COMPONENT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF) {
            return false;
        }
        self.components_present.iter().all(|component| {
            refs.contains(certification_component_canonical_schema_ref(*component))
        })
    }
}

/// Aggregate certification summary across all surface rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentCertificationSummary {
    /// Total certified surface rows.
    pub total_rows: u32,
    /// Count of green (fully certified) surfaces.
    pub certified_count: u32,
    /// Count of yellow (narrowed) surfaces.
    pub narrowed_count: u32,
    /// Count of red (blocked) surfaces.
    pub blocked_count: u32,
    /// True when every surface preserves component truth (no red).
    pub all_rows_preserve_component_truth: bool,
    /// True when all eight claimed surfaces are covered.
    pub all_surfaces_covered: bool,
    /// True when all seven shared components appear across the surfaces.
    pub all_components_covered: bool,
    /// Human-readable certification note.
    pub certification_note: String,
}

impl ReviewComponentCertificationSummary {
    /// Recomputes the summary from a surface row set.
    pub fn from_rows(rows: &[ReviewComponentCertifiedSurfaceRow]) -> Self {
        let mut certified = 0u32;
        let mut narrowed = 0u32;
        let mut blocked = 0u32;
        let mut seen_surfaces: BTreeSet<M5ReviewComponentCertifiedSurface> = BTreeSet::new();
        let mut seen_components: BTreeSet<M5ReviewComponent> = BTreeSet::new();
        for row in rows {
            match row.status {
                ReviewComponentSurfaceClaimStatus::CertifiedParity => certified += 1,
                ReviewComponentSurfaceClaimStatus::NarrowedParity => narrowed += 1,
                ReviewComponentSurfaceClaimStatus::ParityBlocked => blocked += 1,
            }
            seen_surfaces.insert(row.surface);
            for component in &row.components_present {
                seen_components.insert(*component);
            }
        }
        let all_surfaces_covered = M5ReviewComponentCertifiedSurface::ALL
            .iter()
            .all(|surface| seen_surfaces.contains(surface));
        let all_components_covered = M5ReviewComponent::ALL
            .iter()
            .all(|component| seen_components.contains(component));
        let all_preserve = blocked == 0;
        let certification_note = if all_preserve {
            format!(
                "{certified} surface(s) certified, {narrowed} narrowed; all preserve component truth"
            )
        } else {
            format!("{blocked} surface(s) blocked: component truth was flattened")
        };
        Self {
            total_rows: rows.len() as u32,
            certified_count: certified,
            narrowed_count: narrowed,
            blocked_count: blocked,
            all_rows_preserve_component_truth: all_preserve,
            all_surfaces_covered,
            all_components_covered,
            certification_note,
        }
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentCertificationTrustReview {
    /// Every claimed surface presents the same controlled component truth.
    pub same_component_truth_on_every_surface: bool,
    /// Provider identity and base/head or stack relation stay explicit.
    pub provider_identity_and_relation_explicit: bool,
    /// Check class and queue owner stay explicit, never flattened to one verdict.
    pub check_class_and_queue_owner_explicit: bool,
    /// Local-versus-provider estimate stays distinct from provider-owned truth.
    pub local_versus_provider_distinct: bool,
    /// Approval invalidation stays explicit, never hidden behind a generic pill.
    pub approval_invalidation_explicit: bool,
    /// Provider freshness stays explicit; certified never implies fresh.
    pub certified_never_implies_fresh: bool,
    /// Browser handoff stays explicit and never forces raw-provider navigation for triage.
    pub browser_handoff_explicit_no_forced_navigation: bool,
    /// Local-only continuation is preserved when provider freshness is degraded.
    pub local_continuation_preserved: bool,
    /// Certification narrows a claim rather than dropping the component's meaning.
    pub narrows_instead_of_dropping_meaning: bool,
    /// A surface that flattens component truth blocks its certification.
    pub flattened_truth_blocks_certification: bool,
}

impl ReviewComponentCertificationTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.same_component_truth_on_every_surface
            && self.provider_identity_and_relation_explicit
            && self.check_class_and_queue_owner_explicit
            && self.local_versus_provider_distinct
            && self.approval_invalidation_explicit
            && self.certified_never_implies_fresh
            && self.browser_handoff_explicit_no_forced_navigation
            && self.local_continuation_preserved
            && self.narrows_instead_of_dropping_meaning
            && self.flattened_truth_blocks_certification
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentCertificationConsumerProjection {
    /// Desktop review list shows the certified component truth.
    pub desktop_review_list_shows_certification: bool,
    /// Review detail pane shows the certified component truth.
    pub review_detail_pane_shows_certification: bool,
    /// Companion review queue shows the certified component truth.
    pub companion_review_queue_shows_certification: bool,
    /// Help / About review surface shows the certified component truth.
    pub help_review_surface_shows_certification: bool,
    /// Support export shows the certified component truth.
    pub support_export_shows_certification: bool,
    /// Exported review packet shows the certified component truth.
    pub exported_review_packet_shows_certification: bool,
    /// CLI / headless shows the certified component truth.
    pub cli_headless_shows_certification: bool,
    /// Diagnostics shows the certified component truth.
    pub diagnostics_shows_certification: bool,
    /// Narrowed surfaces are visibly labelled rather than silently downgraded.
    pub narrowed_surfaces_visibly_labelled: bool,
}

impl ReviewComponentCertificationConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.desktop_review_list_shows_certification
            && self.review_detail_pane_shows_certification
            && self.companion_review_queue_shows_certification
            && self.help_review_surface_shows_certification
            && self.support_export_shows_certification
            && self.exported_review_packet_shows_certification
            && self.cli_headless_shows_certification
            && self.diagnostics_shows_certification
            && self.narrowed_surfaces_visibly_labelled
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentCertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the certification.
    pub auto_narrow_on_stale: bool,
}

/// Per-surface observation fed to [`ReviewComponentCertificationPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewComponentCertObservation {
    /// Surface the observation applies to.
    pub surface: M5ReviewComponentCertifiedSurface,
    /// True when the surface's provider backing is currently fresh.
    pub provider_fresh: bool,
    /// True when the surface still preserves component truth.
    pub component_truth_preserved: bool,
}

/// Constructor input for [`ReviewComponentCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewComponentCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified surface rows.
    pub surface_rows: Vec<ReviewComponentCertifiedSurfaceRow>,
    /// Aggregate certification summary.
    pub summary: ReviewComponentCertificationSummary,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ReviewComponentCertificationDowngradeTrigger>,
    /// Trust review block.
    pub trust_review: ReviewComponentCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ReviewComponentCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ReviewComponentCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe review-component surface certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewComponentCertificationPacket {
    /// Record kind; must equal [`M5_REVIEW_COMPONENT_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_REVIEW_COMPONENT_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified surface rows.
    pub surface_rows: Vec<ReviewComponentCertifiedSurfaceRow>,
    /// Aggregate certification summary.
    pub summary: ReviewComponentCertificationSummary,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ReviewComponentCertificationDowngradeTrigger>,
    /// Trust review block.
    pub trust_review: ReviewComponentCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ReviewComponentCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ReviewComponentCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ReviewComponentCertificationPacket {
    /// Builds a review-component surface certification packet from stable-lane input.
    pub fn new(input: ReviewComponentCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_REVIEW_COMPONENT_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_REVIEW_COMPONENT_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            surface_rows: input.surface_rows,
            summary: input.summary,
            downgrade_triggers: input.downgrade_triggers,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows surfaces whose provider backing is no longer fresh and blocks
    /// surfaces that flatten component truth, then recomputes the summary.
    ///
    /// This is the downgrade automation: a surface reported with a flattened
    /// component truth blocks (red); a still-green surface whose provider backing
    /// went stale narrows its provider-backed claim to locally-reviewable, marks
    /// the provider/local-provenance axis narrowed, and discloses the stale
    /// trigger. Observations for surfaces not present in the packet are ignored;
    /// surfaces without an observation are left unchanged.
    pub fn apply_downgrade_automation(&mut self, observations: &[ReviewComponentCertObservation]) {
        for row in &mut self.surface_rows {
            let Some(observation) = observations.iter().find(|obs| obs.surface == row.surface)
            else {
                continue;
            };
            if !observation.component_truth_preserved {
                row.component_truth_preserved = false;
            } else if !observation.provider_fresh
                && row.status == ReviewComponentSurfaceClaimStatus::CertifiedParity
            {
                if row.certified_claim.rank() > ReviewComponentClaimTier::LocallyReviewable.rank() {
                    row.certified_claim = ReviewComponentClaimTier::LocallyReviewable;
                }
                if !row
                    .narrowed_axes
                    .contains(&ReviewComponentCertificationAxis::ProviderLocalProvenance)
                {
                    row.narrowed_axes
                        .push(ReviewComponentCertificationAxis::ProviderLocalProvenance);
                }
                for outcome in &mut row.axis_outcomes {
                    if outcome.axis == ReviewComponentCertificationAxis::ProviderLocalProvenance {
                        outcome.state = ReviewComponentAxisCertificationState::NarrowedCertified;
                        outcome.note =
                            "Provider freshness went stale; the claim narrows to locally reviewable and the provider/local distinction stays explicit"
                                .to_owned();
                    }
                }
                row.downgrade_trigger =
                    Some(ReviewComponentCertificationDowngradeTrigger::ProviderFreshnessStale);
            }
            row.status = row.derived_status();
        }
        self.summary = ReviewComponentCertificationSummary::from_rows(&self.surface_rows);
    }

    /// Validates the review-component surface certification invariants.
    pub fn validate(&self) -> Vec<ReviewComponentCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_REVIEW_COMPONENT_CERTIFICATION_RECORD_KIND {
            violations.push(ReviewComponentCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_REVIEW_COMPONENT_CERTIFICATION_SCHEMA_VERSION {
            violations.push(ReviewComponentCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ReviewComponentCertificationViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ReviewComponentCertificationViolation::DowngradeTriggersMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_summary(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(ReviewComponentCertificationViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ReviewComponentCertificationViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ReviewComponentCertificationViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("review-component certification packet serializes"),
        ) {
            violations.push(ReviewComponentCertificationViolation::RawReviewMaterialInExport);
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
            .expect("review-component certification packet serializes")
    }

    /// Deterministic certification matrix CSV for release proof.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,component_truth_preserved\n",
        );
        for row in &self.surface_rows {
            let narrowed = row
                .narrowed_axes
                .iter()
                .map(|axis| axis.as_str())
                .collect::<Vec<_>>()
                .join("|");
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.row_id,
                row.surface.as_str(),
                row.claimed_claim.as_str(),
                row.certified_claim.as_str(),
                row.status.as_str(),
                narrowed,
                row.component_truth_preserved,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Review-Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} certified, {} narrowed, {} blocked)\n",
            self.summary.total_rows,
            self.summary.certified_count,
            self.summary.narrowed_count,
            self.summary.blocked_count,
        ));
        out.push_str(&format!(
            "- All surfaces preserve component truth: {}\n",
            self.summary.all_rows_preserve_component_truth
        ));
        out.push_str(&format!("- Note: {}\n", self.summary.certification_note));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Certified surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: `{}` (claimed `{}`, certified `{}`)\n",
                row.surface.as_str(),
                row.row_id,
                row.status.as_str(),
                row.claimed_claim.as_str(),
                row.certified_claim.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in review-component certification export.
#[derive(Debug)]
pub enum ReviewComponentCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ReviewComponentCertificationViolation>),
}

impl fmt::Display for ReviewComponentCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "review-component certification export parse failed: {error}"
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
                    "review-component certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ReviewComponentCertificationArtifactError {}

/// Validation failures emitted by [`ReviewComponentCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReviewComponentCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No surface rows are present.
    SurfaceRowsMissing,
    /// A surface row is incomplete.
    RowIncomplete,
    /// A surface row lists no components.
    ComponentsMissingOnRow,
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
    /// A row does not score all six certification axes.
    AxisCoverageMissing,
    /// An axis outcome is missing its explanatory note.
    AxisNoteMissing,
    /// A certified claim exceeds the claimed claim it certifies.
    CertifiedClaimExceedsClaimed,
    /// The recorded status does not agree with the derived one.
    StatusMismatch,
    /// The narrowed-axis list disagrees with the axis outcomes marked narrowed.
    NarrowedAxesInconsistent,
    /// A narrowed surface is missing its disclosed downgrade trigger.
    NarrowingWithoutTrigger,
    /// A surface flattened the component's provider / local / queue / approval / freshness truth.
    ReviewComponentTruthDropped,
    /// A row does not cite the canonical matrix and component contracts.
    CanonicalContractReferenceMissing,
    /// Not every claimed surface appears among the rows.
    SurfaceCoverageMissing,
    /// Not every shared component appears across the surfaces.
    ComponentCoverageMissing,
    /// The summary does not agree with the surface rows.
    SummaryMismatch,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// Export contains raw review boundary material.
    RawReviewMaterialInExport,
}

impl ReviewComponentCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::SurfaceRowsMissing => "surface_rows_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::ComponentsMissingOnRow => "components_missing_on_row",
            Self::KeyboardLabelMissing => "keyboard_label_missing",
            Self::ScreenReaderLabelMissing => "screen_reader_label_missing",
            Self::CliEnumTokenMissing => "cli_enum_token_missing",
            Self::ExportEnumTokenMissing => "export_enum_token_missing",
            Self::ExplanationFieldMissing => "explanation_field_missing",
            Self::AxisCoverageMissing => "axis_coverage_missing",
            Self::AxisNoteMissing => "axis_note_missing",
            Self::CertifiedClaimExceedsClaimed => "certified_claim_exceeds_claimed",
            Self::StatusMismatch => "status_mismatch",
            Self::NarrowedAxesInconsistent => "narrowed_axes_inconsistent",
            Self::NarrowingWithoutTrigger => "narrowing_without_trigger",
            Self::ReviewComponentTruthDropped => "review_component_truth_dropped",
            Self::CanonicalContractReferenceMissing => "canonical_contract_reference_missing",
            Self::SurfaceCoverageMissing => "surface_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::SummaryMismatch => "summary_mismatch",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RawReviewMaterialInExport => "raw_review_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable review-component certification export.
pub fn current_review_component_certification_export(
) -> Result<ReviewComponentCertificationPacket, ReviewComponentCertificationArtifactError> {
    let packet: ReviewComponentCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/certify_review_request_row_checks_summary_card_pending_review_tray_merge_readiness_panel_merge_queue_entry_stack_dependency_chip_and_approval_invalidation_banner_truth_on_every_claimed_m5_review_surface/support_export.json"
    )))
    .map_err(ReviewComponentCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ReviewComponentCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Canonical trust review block with every invariant satisfied.
pub fn canonical_trust_review() -> ReviewComponentCertificationTrustReview {
    ReviewComponentCertificationTrustReview {
        same_component_truth_on_every_surface: true,
        provider_identity_and_relation_explicit: true,
        check_class_and_queue_owner_explicit: true,
        local_versus_provider_distinct: true,
        approval_invalidation_explicit: true,
        certified_never_implies_fresh: true,
        browser_handoff_explicit_no_forced_navigation: true,
        local_continuation_preserved: true,
        narrows_instead_of_dropping_meaning: true,
        flattened_truth_blocks_certification: true,
    }
}

/// Canonical consumer projection block with every surface projecting certification truth.
pub fn canonical_consumer_projection() -> ReviewComponentCertificationConsumerProjection {
    ReviewComponentCertificationConsumerProjection {
        desktop_review_list_shows_certification: true,
        review_detail_pane_shows_certification: true,
        companion_review_queue_shows_certification: true,
        help_review_surface_shows_certification: true,
        support_export_shows_certification: true,
        exported_review_packet_shows_certification: true,
        cli_headless_shows_certification: true,
        diagnostics_shows_certification: true,
        narrowed_surfaces_visibly_labelled: true,
    }
}

/// Canonical source contract refs that every certification export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        M5_REVIEW_COMPONENT_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_REVIEW_COMPONENT_CERTIFICATION_DOC_REF.to_owned(),
        M5_REVIEW_COMPONENT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF.to_owned(),
        M5_REVIEW_COMPONENT_CERTIFICATION_CONSUMER_CONTRACT_REF.to_owned(),
        M5_REVIEW_COMPONENT_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF.to_owned(),
        M5_REVIEW_COMPONENT_CERTIFICATION_REVIEW_REQUEST_ROW_CONTRACT_REF.to_owned(),
        M5_REVIEW_COMPONENT_CERTIFICATION_CHECKS_SUMMARY_CARD_CONTRACT_REF.to_owned(),
        M5_REVIEW_COMPONENT_CERTIFICATION_MERGE_READINESS_PANEL_CONTRACT_REF.to_owned(),
        M5_REVIEW_COMPONENT_CERTIFICATION_PENDING_REVIEW_TRAY_CONTRACT_REF.to_owned(),
    ]
}

fn validate_source_contracts(
    packet: &ReviewComponentCertificationPacket,
    violations: &mut Vec<ReviewComponentCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_REVIEW_COMPONENT_CERTIFICATION_SCHEMA_REF,
        M5_REVIEW_COMPONENT_CERTIFICATION_DOC_REF,
        M5_REVIEW_COMPONENT_CERTIFICATION_COMPONENT_MATRIX_CONTRACT_REF,
        M5_REVIEW_COMPONENT_CERTIFICATION_CONSUMER_CONTRACT_REF,
        M5_REVIEW_COMPONENT_CERTIFICATION_ACCESSIBILITY_CONTRACT_REF,
        M5_REVIEW_COMPONENT_CERTIFICATION_REVIEW_REQUEST_ROW_CONTRACT_REF,
        M5_REVIEW_COMPONENT_CERTIFICATION_CHECKS_SUMMARY_CARD_CONTRACT_REF,
        M5_REVIEW_COMPONENT_CERTIFICATION_MERGE_READINESS_PANEL_CONTRACT_REF,
        M5_REVIEW_COMPONENT_CERTIFICATION_PENDING_REVIEW_TRAY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ReviewComponentCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &ReviewComponentCertificationPacket,
    violations: &mut Vec<ReviewComponentCertificationViolation>,
) {
    if packet.surface_rows.is_empty() {
        violations.push(ReviewComponentCertificationViolation::SurfaceRowsMissing);
        return;
    }

    let mut seen_surfaces: BTreeSet<M5ReviewComponentCertifiedSurface> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5ReviewComponent> = BTreeSet::new();

    for row in &packet.surface_rows {
        if row.row_id.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(ReviewComponentCertificationViolation::RowIncomplete);
        }
        if row.components_present.is_empty() {
            violations.push(ReviewComponentCertificationViolation::ComponentsMissingOnRow);
        }

        if row.keyboard_label.trim().is_empty() {
            violations.push(ReviewComponentCertificationViolation::KeyboardLabelMissing);
        }
        if row.screen_reader_label.trim().is_empty() {
            violations.push(ReviewComponentCertificationViolation::ScreenReaderLabelMissing);
        }
        if row.cli_enum_token.trim().is_empty() {
            violations.push(ReviewComponentCertificationViolation::CliEnumTokenMissing);
        }
        if row.export_enum_token.trim().is_empty() {
            violations.push(ReviewComponentCertificationViolation::ExportEnumTokenMissing);
        }
        if row.explanation_field.trim().is_empty() {
            violations.push(ReviewComponentCertificationViolation::ExplanationFieldMissing);
        }

        if !row.covers_all_axes() {
            violations.push(ReviewComponentCertificationViolation::AxisCoverageMissing);
        }
        if row
            .axis_outcomes
            .iter()
            .any(|outcome| outcome.note.trim().is_empty())
        {
            violations.push(ReviewComponentCertificationViolation::AxisNoteMissing);
        }

        // AC2 core: a certified claim may never exceed the claim it certifies.
        if !row.certified_claim_within_claimed() {
            violations.push(ReviewComponentCertificationViolation::CertifiedClaimExceedsClaimed);
        }

        if !row.narrowed_axes_consistent() {
            violations.push(ReviewComponentCertificationViolation::NarrowedAxesInconsistent);
        }

        // A narrowed surface must disclose its downgrade trigger.
        if !row.narrowed_axes.is_empty() && row.downgrade_trigger.is_none() {
            violations.push(ReviewComponentCertificationViolation::NarrowingWithoutTrigger);
        }

        // Delta: certification may narrow a claim but never drop component truth.
        if !row.component_truth_preserved {
            violations.push(ReviewComponentCertificationViolation::ReviewComponentTruthDropped);
        }

        // The recorded status must agree with the derived one.
        if !row.status_is_consistent() {
            violations.push(ReviewComponentCertificationViolation::StatusMismatch);
        }

        if !row.points_at_canonical_contracts() {
            violations
                .push(ReviewComponentCertificationViolation::CanonicalContractReferenceMissing);
        }

        seen_surfaces.insert(row.surface);
        for component in &row.components_present {
            seen_components.insert(*component);
        }
    }

    for surface in M5ReviewComponentCertifiedSurface::ALL {
        if !seen_surfaces.contains(&surface) {
            violations.push(ReviewComponentCertificationViolation::SurfaceCoverageMissing);
            break;
        }
    }
    for component in M5ReviewComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(ReviewComponentCertificationViolation::ComponentCoverageMissing);
            break;
        }
    }
}

fn validate_summary(
    packet: &ReviewComponentCertificationPacket,
    violations: &mut Vec<ReviewComponentCertificationViolation>,
) {
    let recomputed = ReviewComponentCertificationSummary::from_rows(&packet.surface_rows);
    if recomputed != packet.summary {
        violations.push(ReviewComponentCertificationViolation::SummaryMismatch);
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
