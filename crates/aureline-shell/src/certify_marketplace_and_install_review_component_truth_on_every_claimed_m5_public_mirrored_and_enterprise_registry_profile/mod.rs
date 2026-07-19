//! M05-1107 surface certification over the frozen M5 marketplace-result-row /
//! detail-fact-grid / compatibility-label / permission-manifest / activation-budget /
//! install-review / publisher-continuity / installed-state-diagnostics component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix`])
//! defines the eight reusable marketplace-result-row, marketplace-detail-fact-grid,
//! compatibility-label-strip, permission-manifest-summary, activation-budget-band,
//! install-update-disable-rollback-review-sheet, publisher-continuity-row, and
//! installed-state-diagnostics-card components, the M05-1101..1105 implement lanes narrow
//! each one, and the M05-1106 accessibility lane
//! ([`crate::add_keyboard_screen_reader_high_zoom_reduced_motion_cli_export_parity_and_automatic_claim_narrowing_when_manifest_publisher_compatibility_or_activation_budget_evidence_weakens_across_claimed_m5_marketplace_components`])
//! proves keyboard / screen-reader / high-zoom / reduced-motion / CLI-export parity and
//! per-family auto-narrowing, this closing capstone *certifies* that the shared component
//! truth holds on every claimed M5 registry / install operating profile — and auto-narrows
//! any profile that cannot sustain it.
//!
//! It is keyed on the claimed **registry profile** a user, operator, or support engineer
//! reads marketplace and install-review truth through (a public verified registry, a
//! mirrored / offline-continuity registry, an enterprise / private registry, a reviewed
//! side-load, a stale-compatibility listing, an over-budget / throttled artifact, a
//! rollback whose compatibility is unverifiable, and a transferred-publisher listing), not
//! on component family or implement lane. Each [`MarketplaceInstallProfileCertificationRow`]
//! certifies one profile across six truth axes — visual, keyboard, screen-reader,
//! CLI/export, degraded-state, and registry-install-truth behavior — and either passes
//! (green), auto-narrows its install / listing claim to the weakest supported ceiling
//! (yellow), or is blocked (red) when a degraded axis is hidden behind a fresh
//! install-ready claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile
//! that keeps an `InstallReadyResult` / `ReviewableListingResult` claim while one of its
//! truth axes is not current is over-claiming and blocks; a profile that discloses the
//! reduction by narrowing its claim (with a bound reason and a frozen downgrade trigger)
//! is honestly yellow. Only a public first-party verified registry profile may certify an
//! `InstallReadyResult` claim — a mirrored, enterprise, side-load, or degraded profile that
//! keeps an install-ready claim is over-reaching and blocks. The always-on CLI/export axis
//! must always stay certified so support and automation can reconstruct the certified
//! registry source class, compatibility range, host / runtime model, permission posture,
//! transitive widening, activation-budget band, disable scope, rollback compatibility,
//! publisher continuity, and quarantine history from the same artifact identity the user
//! saw.
//!
//! The B131 guardrails are enforced per row: no profile may hide permission widening or
//! activation cost, hide a publisher transfer / disable scope / rollback incompatibility,
//! collapse the registry source class across public / mirrored / enterprise, or present an
//! incompatible or over-budget artifact as ready to install. A profile that breaches any
//! guardrail blocks (red).
//!
//! Every row cites exactly one canonical marketplace-install proof bundle
//! ([`MARKETPLACE_INSTALL_CERT_CANONICAL_BUNDLE_REF`]) — the frozen marketplace-install
//! component matrix proof — rather than cloning per-profile evidence. The packet is
//! metadata-only: raw manifest bodies, permission tokens, and activation-budget payloads
//! never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-marketplace-install-component-certification.schema.json`](../../../../schemas/ui/m5-marketplace-install-component-certification.schema.json).
//! The contract doc is
//! [`docs/marketplace/m5_marketplace_install_component_certification_contract.md`](../../../../docs/marketplace/m5_marketplace_install_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_keyboard_screen_reader_high_zoom_reduced_motion_cli_export_parity_and_automatic_claim_narrowing_when_manifest_publisher_compatibility_or_activation_budget_evidence_weakens_across_claimed_m5_marketplace_components as a11y;
use crate::freeze_the_m5_marketplace_result_row_marketplace_detail_fact_grid_compatibility_permission_activation_install_review_publisher_continuity_and_diagnostics_component_matrix as matrix;
use a11y::M5MarketplaceComponentClaim;
use matrix::{M5MarketplaceInstallComponentFamily, M5MarketplaceInstallDowngradeTrigger};

/// Schema version stamped on the M05-1107 certification packet.
pub const MARKETPLACE_INSTALL_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`MarketplaceInstallProfileCertificationPacket`].
pub const MARKETPLACE_INSTALL_CERT_RECORD_KIND: &str =
    "m5_marketplace_install_component_certification_packet";

/// Stable record-kind tag carried by each [`MarketplaceInstallProfileCertificationRow`].
pub const MARKETPLACE_INSTALL_CERT_ROW_RECORD_KIND: &str =
    "m5_marketplace_install_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const MARKETPLACE_INSTALL_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-marketplace-install-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const MARKETPLACE_INSTALL_CERT_DOC_REF: &str =
    "docs/marketplace/m5_marketplace_install_component_certification_contract.md";

/// Repo-relative path of the frozen marketplace-install component matrix schema the
/// certified profiles render.
pub const MARKETPLACE_INSTALL_CERT_MATRIX_REF: &str =
    matrix::M5_MARKETPLACE_INSTALL_COMPONENT_SCHEMA_REF;

/// The one canonical marketplace-install proof bundle every certified profile cites as its
/// first-resolved component truth. All eight profiles point back to it rather than cloning
/// per-profile evidence.
pub const MARKETPLACE_INSTALL_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_MARKETPLACE_INSTALL_COMPONENT_ARTIFACT_REF;

/// The M05-1106 accessibility support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const MARKETPLACE_INSTALL_CERT_A11Y_BUNDLE_REF: &str =
    a11y::MARKETPLACE_INSTALL_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const MARKETPLACE_INSTALL_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-marketplace-install-component-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const MARKETPLACE_INSTALL_CERT_CSV_REF: &str =
    "artifacts/release/m5-marketplace-install-component-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const MARKETPLACE_INSTALL_CERT_REPORT_REF: &str =
    "artifacts/release/m5-marketplace-install-component-certification-proof/report.md";

/// The eight claimed M5 registry / install operating profiles this capstone certifies.
/// Keyed on the profile a user, operator, or support engineer reads marketplace and
/// install-review truth through, not on the reusable component family it renders. Only a
/// public first-party verified registry profile may certify an install-ready result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MarketplaceInstallCertifiedProfile {
    /// A public, first-party verified registry listing — fully identified, source-attributed,
    /// compatible, permission-clear, and publisher-continuous; the only profile that may read
    /// as exactly ready to install right now.
    PublicVerifiedRegistry,
    /// A mirrored / offline-continuity registry where the source class is mirrored — reviewable
    /// under the named mirror, never a fresh public install-ready reading.
    MirroredRegistry,
    /// An enterprise / private registry governed by an operator — reviewable under the named
    /// enterprise source class, never a blanket public install-ready reading.
    EnterpriseRegistry,
    /// A reviewed side-loaded artifact whose source class is named side-load — reviewable, never
    /// presented as a verified public install.
    SideLoadReviewedRegistry,
    /// A listing whose compatibility evidence is stale; the claim narrows to a
    /// compatibility-unverified projection that preserves the last-known compatibility range.
    StaleCompatibilityRegistry,
    /// An artifact that is over budget / throttled; the claim narrows to an activation-budget
    /// projection that names the over-budget band rather than a cost-free install.
    OverBudgetThrottledRegistry,
    /// A rollback whose compatibility evidence is unverifiable; the claim narrows to a
    /// rollback-unverified projection that names the rollback limits and disable scope.
    RollbackUnverifiableRegistry,
    /// A listing whose publisher continuity is transferred / unverifiable; the claim narrows to
    /// a publisher-continuity projection that names the transfer.
    TransferredPublisherRegistry,
}

impl M5MarketplaceInstallCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5MarketplaceInstallCertifiedProfile; 8] = [
        M5MarketplaceInstallCertifiedProfile::PublicVerifiedRegistry,
        M5MarketplaceInstallCertifiedProfile::MirroredRegistry,
        M5MarketplaceInstallCertifiedProfile::EnterpriseRegistry,
        M5MarketplaceInstallCertifiedProfile::SideLoadReviewedRegistry,
        M5MarketplaceInstallCertifiedProfile::StaleCompatibilityRegistry,
        M5MarketplaceInstallCertifiedProfile::OverBudgetThrottledRegistry,
        M5MarketplaceInstallCertifiedProfile::RollbackUnverifiableRegistry,
        M5MarketplaceInstallCertifiedProfile::TransferredPublisherRegistry,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicVerifiedRegistry => "public_verified_registry",
            Self::MirroredRegistry => "mirrored_registry",
            Self::EnterpriseRegistry => "enterprise_registry",
            Self::SideLoadReviewedRegistry => "side_load_reviewed_registry",
            Self::StaleCompatibilityRegistry => "stale_compatibility_registry",
            Self::OverBudgetThrottledRegistry => "over_budget_throttled_registry",
            Self::RollbackUnverifiableRegistry => "rollback_unverifiable_registry",
            Self::TransferredPublisherRegistry => "transferred_publisher_registry",
        }
    }

    /// True only for the public first-party verified registry profile. An install-ready result
    /// may be certified on this profile alone; every other profile is at most a reviewable
    /// listing result or a narrowed projection.
    pub const fn is_public_first_party(self) -> bool {
        matches!(self, Self::PublicVerifiedRegistry)
    }
}

/// The six truth axes a certified profile is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader, CLI/export,
/// degraded-state, and registry-install-truth behavior. The CLI/export axis is always-on
/// and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceInstallCertificationAxis {
    /// Visual parity: registry source class, compatibility range, host / runtime model,
    /// permission posture, transitive widening, activation-budget band, disable scope, rollback
    /// compatibility, publisher continuity, and quarantine history are shown on the primary
    /// surface.
    Visual,
    /// Keyboard-reach parity: the same marketplace / install truth and its actions (open
    /// detail, review install, inspect permissions, disable / rollback) are reachable without a
    /// pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color
    /// or a chrome glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as text
    /// / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale compatibility signal, partial permission manifest, stale
    /// activation budget, unverifiable rollback, transferred publisher, or partial quarantine
    /// history honestly downgrades an `InstallReadyResult` / `ReviewableListingResult` claim
    /// rather than reading as a fresh, verified public install.
    DegradedState,
    /// Registry-install-truth parity: registry source class, compatibility range, permission
    /// posture, transitive widening, activation-budget band, disable scope, rollback
    /// compatibility, publisher continuity, and quarantine history stay explicit and never
    /// collapse into generic chrome wording, hide permission widening or activation cost, hide a
    /// publisher transfer / disable scope / rollback incompatibility, collapse the source class
    /// across public / mirrored / enterprise, or present an incompatible or over-budget artifact
    /// as ready to install.
    RegistryInstallTruth,
}

impl MarketplaceInstallCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [MarketplaceInstallCertificationAxis; 6] = [
        MarketplaceInstallCertificationAxis::Visual,
        MarketplaceInstallCertificationAxis::Keyboard,
        MarketplaceInstallCertificationAxis::ScreenReader,
        MarketplaceInstallCertificationAxis::CliExport,
        MarketplaceInstallCertificationAxis::DegradedState,
        MarketplaceInstallCertificationAxis::RegistryInstallTruth,
    ];

    /// The always-on CLI/export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::CliExport)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::RegistryInstallTruth => "registry_install_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceInstallAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible
    /// claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind an install-ready claim
    /// inherited from a healthier profile.
    UndisclosedDrift,
}

impl MarketplaceInstallAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the author —
/// always recomputed from the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketplaceInstallProfileClaimStatus {
    /// Full standing: every axis certified, every guardrail held, claimed install / listing
    /// tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a guardrail breaks, CLI/export parity
    /// drops, a non-public profile claims install-ready, or the narrowing is inconsistent.
    Red,
}

impl MarketplaceInstallProfileClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the profile is publishable as certified (green or disclosed yellow); red
    /// profiles block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The four B131 guardrails carried on every certified profile. All four must hold — a
/// breach blocks the profile (red). Each field is `true` only when the profile *breaks* the
/// guardrail, so a clean profile carries all-false.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceInstallCertGuardrails {
    /// True if the profile hides permission widening or activation cost behind compact chrome.
    /// Must be false.
    pub hides_permission_widening_or_activation_cost: bool,
    /// True if the profile hides a publisher transfer, disable scope, or rollback
    /// incompatibility. Must be false.
    pub hides_publisher_transfer_disable_scope_or_rollback_incompatibility: bool,
    /// True if the profile collapses the registry source class across public / mirrored /
    /// enterprise. Must be false.
    pub collapses_registry_source_class_across_public_mirrored_enterprise: bool,
    /// True if the profile presents an incompatible or over-budget artifact as ready to install.
    /// Must be false.
    pub presents_incompatible_or_over_budget_as_ready: bool,
}

impl MarketplaceInstallCertGuardrails {
    /// A clean profile: every guardrail held.
    pub const CLEAN: Self = Self {
        hides_permission_widening_or_activation_cost: false,
        hides_publisher_transfer_disable_scope_or_rollback_incompatibility: false,
        collapses_registry_source_class_across_public_mirrored_enterprise: false,
        presents_incompatible_or_over_budget_as_ready: false,
    };

    /// True when every guardrail holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.hides_permission_widening_or_activation_cost
            && !self.hides_publisher_transfer_disable_scope_or_rollback_incompatibility
            && !self.collapses_registry_source_class_across_public_mirrored_enterprise
            && !self.presents_incompatible_or_over_budget_as_ready
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies
/// only when this offers text / JSON / Markdown reconstruction and prohibits a
/// screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceInstallCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The source-class / compatibility / host / permission / activation-budget / disable-scope /
    /// rollback / publisher / quarantine fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl MarketplaceInstallCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a screenshot-only
    /// export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// One axis outcome on one certified profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceInstallAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: MarketplaceInstallCertificationAxis,
    /// The certification state of the axis.
    pub state: MarketplaceInstallAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5MarketplaceInstallDowngradeTrigger>,
}

impl MarketplaceInstallAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible trigger
    ///   (that is exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            MarketplaceInstallAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            MarketplaceInstallAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            MarketplaceInstallAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current. Present
/// iff the certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceInstallClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: MarketplaceInstallCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5MarketplaceComponentClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5MarketplaceComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 registry / install profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceInstallProfileCertificationRow {
    /// Record kind; must equal [`MARKETPLACE_INSTALL_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MARKETPLACE_INSTALL_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5MarketplaceInstallCertifiedProfile,
    /// The install / listing claim ceiling the profile asserts.
    pub claimed_claim: M5MarketplaceComponentClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5MarketplaceComponentClaim,
    /// The frozen component families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5MarketplaceInstallComponentFamily>,
    /// One outcome per [`MarketplaceInstallCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<MarketplaceInstallAxisOutcome>,
    /// The B131 guardrails; all must hold.
    pub guardrails: MarketplaceInstallCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<MarketplaceInstallClaimAutoNarrow>,
    /// The one canonical marketplace-install proof bundle this profile cites. Must equal
    /// [`MARKETPLACE_INSTALL_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: MarketplaceInstallProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: MarketplaceInstallCertExportParity,
    /// The compatibility notes captured for this profile.
    #[serde(default)]
    pub compatibility_notes: Vec<String>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the certification was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl MarketplaceInstallProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: MarketplaceInstallCertificationAxis,
    ) -> Option<&MarketplaceInstallAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<MarketplaceInstallCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && MarketplaceInstallCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(MarketplaceInstallAxisOutcome::well_formed)
    }

    /// True when the profile narrows its install / listing claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<MarketplaceInstallCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == MarketplaceInstallAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, guardrails, and claim narrowing. This is the
    /// heart of the capstone: a degraded axis must produce a visible claim narrowing, only a
    /// public first-party profile may certify install-ready, every guardrail must hold,
    /// CLI/export parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> MarketplaceInstallProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != MARKETPLACE_INSTALL_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return MarketplaceInstallProfileClaimStatus::Red;
        }

        // Every B131 guardrail must hold.
        if !self.guardrails.all_held() {
            return MarketplaceInstallProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return MarketplaceInstallProfileClaimStatus::Red;
        }

        // Only a public first-party profile may certify an install-ready result.
        if self.certified_claim.asserts_install_ready_result()
            && !self.profile.is_public_first_party()
        {
            return MarketplaceInstallProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(MarketplaceInstallCertificationAxis::CliExport) {
            Some(o) if o.state == MarketplaceInstallAxisCertificationState::Certified => {}
            _ => return MarketplaceInstallProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == MarketplaceInstallAxisCertificationState::UndisclosedDrift)
        {
            return MarketplaceInstallProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return MarketplaceInstallProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return MarketplaceInstallProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return MarketplaceInstallProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return MarketplaceInstallProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier profile's truth.
        if !narrowed.is_empty() {
            return MarketplaceInstallProfileClaimStatus::Red;
        }

        MarketplaceInstallProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == MARKETPLACE_INSTALL_CERT_ROW_RECORD_KIND
            && self.schema_version == MARKETPLACE_INSTALL_CERT_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_bundle_ref.trim().is_empty()
            && !self.consumed_families.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.compatibility_notes.is_empty()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "profile={profile} claimed={claimed} certified={certified} status={status} \
narrowed_axes={narrowed}",
            profile = self.profile.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
        )
    }
}

/// Rolled-up summary of an M05-1107 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceInstallProfileCertificationSummary {
    pub row_count: usize,
    pub profile_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_profiles_present: bool,
    pub all_families_covered: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub all_guardrails_held: bool,
    pub every_axis_covered_on_every_row: bool,
    pub narrowed_profile_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`MarketplaceInstallProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarketplaceInstallProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<MarketplaceInstallProfileCertificationRow>,
}

/// Checked-in M05-1107 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketplaceInstallProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<MarketplaceInstallProfileCertificationRow>,
    pub summary: MarketplaceInstallProfileCertificationSummary,
}

impl MarketplaceInstallProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: MarketplaceInstallProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: MARKETPLACE_INSTALL_CERT_SCHEMA_VERSION,
            record_kind: MARKETPLACE_INSTALL_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: MarketplaceInstallProfileCertificationSummary {
                row_count: 0,
                profile_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_profiles_present: false,
                all_families_covered: false,
                all_rows_publishable: false,
                all_status_fresh: false,
                all_rows_cite_canonical_bundle: false,
                all_rows_export_parity_certified: false,
                all_guardrails_held: false,
                every_axis_covered_on_every_row: false,
                narrowed_profile_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Profiles represented by some row in this packet.
    pub fn represented_profiles(&self) -> BTreeSet<M5MarketplaceInstallCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Component families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5MarketplaceInstallComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5MarketplaceInstallCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen component family is certified on at least one profile — proof the
    /// full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5MarketplaceInstallComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(MarketplaceInstallCertificationAxis::CliExport)
                .is_some_and(|o| o.state == MarketplaceInstallAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> MarketplaceInstallProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == MarketplaceInstallProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == MarketplaceInstallProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == MarketplaceInstallProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(MarketplaceInstallProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        MarketplaceInstallProfileCertificationSummary {
            row_count: self.rows.len(),
            profile_count: profiles.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_profiles_present: all_profiles,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == MARKETPLACE_INSTALL_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(MarketplaceInstallProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<MarketplaceInstallCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != MARKETPLACE_INSTALL_CERT_SCHEMA_VERSION {
            violations.push(MarketplaceInstallCertificationViolation::SchemaVersion {
                expected: MARKETPLACE_INSTALL_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != MARKETPLACE_INSTALL_CERT_RECORD_KIND {
            violations.push(MarketplaceInstallCertificationViolation::RecordKind {
                expected: MARKETPLACE_INSTALL_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(MarketplaceInstallCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != MARKETPLACE_INSTALL_CERT_CANONICAL_BUNDLE_REF {
            violations.push(MarketplaceInstallCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(MarketplaceInstallCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(MarketplaceInstallCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    MarketplaceInstallCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    MarketplaceInstallCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != MARKETPLACE_INSTALL_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    MarketplaceInstallCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B131 guardrail must hold.
            if !row.guardrails.all_held() {
                violations.push(
                    MarketplaceInstallCertificationViolation::GuardrailViolated {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Only a public first-party profile may certify an install-ready result.
            if row.certified_claim.asserts_install_ready_result()
                && !row.profile.is_public_first_party()
            {
                violations.push(
                    MarketplaceInstallCertificationViolation::NonPublicProfileClaimsInstallReady {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(MarketplaceInstallCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    MarketplaceInstallCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    MarketplaceInstallCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    MarketplaceInstallCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == MarketplaceInstallProfileClaimStatus::Red {
                violations.push(MarketplaceInstallCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(MarketplaceInstallCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen component family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(MarketplaceInstallCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(MarketplaceInstallCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations
                .push(MarketplaceInstallCertificationViolation::RawMarketplaceMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("certification packet serializes")
    }

    /// Deterministic CSV of the certification rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,profile,claimed_claim,certified_claim,status,narrowed_axes,binding_axis\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{profile},{claimed},{certified},{status},{narrowed},{binding}\n",
                id = row.row_id,
                profile = row.profile.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Marketplace / Install-Review Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5MarketplaceInstallCertifiedProfile::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Guardrails held: {}\n",
            self.summary.all_guardrails_held
        ));
        out.push_str(&format!(
            "- Auto-narrowed profiles: {}\n",
            self.summary.narrowed_profile_count,
        ));
        out.push_str(&format!("- Report clean: {}\n", self.summary.report_clean));
        out.push_str("\n## Profiles\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in certification export.
pub fn current_m5_marketplace_install_component_certification_export() -> Result<
    MarketplaceInstallProfileCertificationPacket,
    MarketplaceInstallCertificationArtifactError,
> {
    let packet: MarketplaceInstallProfileCertificationPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-marketplace-install-component-certification-proof/support_export.json"
        )
    ))
    .map_err(MarketplaceInstallCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(MarketplaceInstallCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum MarketplaceInstallCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<MarketplaceInstallCertificationViolation>),
}

impl fmt::Display for MarketplaceInstallCertificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "certification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for MarketplaceInstallCertificationArtifactError {}

/// Validation failure for M05-1107 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarketplaceInstallCertificationViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    WrongCanonicalBundle,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    AxisCoverageIncomplete { id: String },
    MalformedAxisOutcome { id: String },
    RowMissingCanonicalBundle { id: String },
    GuardrailViolated { id: String },
    NonPublicProfileClaimsInstallReady { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawMarketplaceMaterialInExport,
}

impl fmt::Display for MarketplaceInstallCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::WrongCanonicalBundle => {
                write!(
                    f,
                    "packet does not cite the canonical marketplace-install proof bundle"
                )
            }
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::AxisCoverageIncomplete { id } => {
                write!(
                    f,
                    "row {id} does not score every certification axis exactly once"
                )
            }
            Self::MalformedAxisOutcome { id } => {
                write!(
                    f,
                    "row {id} has an axis outcome whose disclosure fields disagree with its state"
                )
            }
            Self::RowMissingCanonicalBundle { id } => {
                write!(
                    f,
                    "row {id} does not cite the one canonical marketplace-install proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B131 guardrail: hidden permission widening or activation cost, \
hidden publisher transfer / disable scope / rollback incompatibility, collapsed registry source \
class, or an incompatible / over-budget artifact presented as install-ready"
                )
            }
            Self::NonPublicProfileClaimsInstallReady { id } => {
                write!(
                    f,
                    "row {id} certifies an install-ready result on a non-public first-party profile"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::CertifiedClaimExceedsClaim { id } => {
                write!(
                    f,
                    "row {id} certifies a claim stronger than the claimed one"
                )
            }
            Self::StatusDerivationStale { id } => {
                write!(
                    f,
                    "row {id} stored status disagrees with a fresh derivation"
                )
            }
            Self::ProfileBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh install-ready \
claim, a guardrail broke, CLI/export parity dropped, a non-public profile claimed install-ready, or \
the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 registry / install profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen marketplace-install component family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawMarketplaceMaterialInExport => {
                write!(f, "export contains raw manifest / credential material")
            }
        }
    }
}

impl Error for MarketplaceInstallCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&MarketplaceInstallAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != MarketplaceInstallAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the
/// marketplace / install generics the spec forbids collapsing distinct source-class,
/// compatibility, permission, activation-budget, rollback, and publisher-continuity truth
/// into (whole-label matches so a full sentence naming a concrete source class, compatibility
/// range, or publisher transfer is not flagged).
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "reduced"
            | "stale"
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "loading"
            | "public"
            | "mirrored"
            | "enterprise"
            | "side-load"
            | "sideload"
            | "side load"
            | "incompatible"
            | "over budget"
            | "over-budget"
            | "throttled"
            | "quarantined"
            | "transferred"
            | "install"
            | "install-ready"
            | "ready"
            | "compatible"
            | "permission"
            | "rollback"
            | "disable"
            | "publisher"
            | "success"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// --------------------------------------------------------------------------
// Seed builder — the one source of truth shared by the tests and the on-disk
// support export so both stay byte-aligned.
// --------------------------------------------------------------------------

/// Builds the canonical, checked-in M05-1107 certification packet. Certifies all eight
/// claimed M5 registry / install profiles: four deliver their claim (green) and four
/// auto-narrow a not-current truth axis to a weaker install / listing ceiling (yellow). No
/// profile hides drift or breaks a guardrail (red).
pub fn seeded_m5_marketplace_install_component_certification_packet(
) -> MarketplaceInstallProfileCertificationPacket {
    MarketplaceInstallProfileCertificationPacket::new(
        MarketplaceInstallProfileCertificationPacketInput {
            packet_id: "m5-marketplace-install-component-certification:stable:0001".to_owned(),
            as_of: "2026-07-11T00:00:00Z".to_owned(),
            matrix_ref: MARKETPLACE_INSTALL_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: MARKETPLACE_INSTALL_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:marketplace-install-component-certification:{id}"),
        MARKETPLACE_INSTALL_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> MarketplaceInstallCertExportParity {
    MarketplaceInstallCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: MarketplaceInstallCertificationAxis) -> &'static str {
    match axis {
        MarketplaceInstallCertificationAxis::Visual => {
            "registry source class, compatibility range, host / runtime model, permission posture, transitive widening, activation-budget band, disable scope, rollback compatibility, publisher continuity, and quarantine history shown on-surface"
        }
        MarketplaceInstallCertificationAxis::Keyboard => {
            "the same open-detail / review-install / inspect-permissions / disable-rollback actions are keyboard-reachable"
        }
        MarketplaceInstallCertificationAxis::ScreenReader => {
            "the same marketplace / install truth is announced non-visually, never color/glyph-only"
        }
        MarketplaceInstallCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        MarketplaceInstallCertificationAxis::DegradedState => {
            "a stale compatibility signal, partial permission manifest, stale activation budget, unverifiable rollback, transferred publisher, or partial quarantine history honestly downgrades the InstallReadyResult/ReviewableListingResult claim rather than reading as a fresh verified public install"
        }
        MarketplaceInstallCertificationAxis::RegistryInstallTruth => {
            "registry source class, compatibility range, permission posture, transitive widening, activation-budget band, disable scope, rollback compatibility, publisher continuity, and quarantine history stay explicit and never collapse into generic chrome, hide permission widening or activation cost, hide a publisher transfer / disable scope / rollback incompatibility, collapse the source class across public / mirrored / enterprise, or present an incompatible or over-budget artifact as install-ready"
        }
    }
}

fn seed_certified(axis: MarketplaceInstallCertificationAxis) -> MarketplaceInstallAxisOutcome {
    MarketplaceInstallAxisOutcome {
        axis,
        state: MarketplaceInstallAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: MarketplaceInstallCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5MarketplaceInstallDowngradeTrigger,
) -> MarketplaceInstallAxisOutcome {
    MarketplaceInstallAxisOutcome {
        axis,
        state: MarketplaceInstallAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<MarketplaceInstallAxisOutcome> {
    MarketplaceInstallCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: MarketplaceInstallCertificationAxis,
    outcome: MarketplaceInstallAxisOutcome,
) -> Vec<MarketplaceInstallAxisOutcome> {
    MarketplaceInstallCertificationAxis::ALL
        .iter()
        .copied()
        .map(|a| {
            if a == axis {
                outcome.clone()
            } else {
                seed_certified(a)
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn seed_row(
    row_id: &str,
    profile: M5MarketplaceInstallCertifiedProfile,
    claimed_claim: M5MarketplaceComponentClaim,
    certified_claim: M5MarketplaceComponentClaim,
    consumed_families: &[M5MarketplaceInstallComponentFamily],
    axis_outcomes: Vec<MarketplaceInstallAxisOutcome>,
    claim_auto_narrow: Option<MarketplaceInstallClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> MarketplaceInstallProfileCertificationRow {
    let mut row = MarketplaceInstallProfileCertificationRow {
        record_kind: MARKETPLACE_INSTALL_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: MARKETPLACE_INSTALL_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: MarketplaceInstallCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: MARKETPLACE_INSTALL_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: MarketplaceInstallProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            MARKETPLACE_INSTALL_CERT_MATRIX_REF.to_owned(),
            MARKETPLACE_INSTALL_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-11T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: MarketplaceInstallCertificationAxis,
    from_claim: M5MarketplaceComponentClaim,
    to_claim: M5MarketplaceComponentClaim,
    label: &str,
) -> MarketplaceInstallClaimAutoNarrow {
    MarketplaceInstallClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<MarketplaceInstallProfileCertificationRow> {
    use M5MarketplaceComponentClaim::*;
    use M5MarketplaceInstallCertifiedProfile as P;
    use M5MarketplaceInstallComponentFamily::*;
    use M5MarketplaceInstallDowngradeTrigger as Trig;
    use MarketplaceInstallCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:public-verified-registry",
            P::PublicVerifiedRegistry,
            InstallReadyResult,
            InstallReadyResult,
            &[MarketplaceResultRow, PublisherContinuityRow],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "registry_source_class"],
            &[
                "marketplace result row names the public registry source class, the compatibility range, and the continuous publisher for the listed artifact",
                "publisher-continuity row names a continuous, verified publisher with no transfer so the listing reads as first-party",
                "keyboard/screen-reader reach preserved for the result row and the publisher-continuity row",
                "registry-install-truth: a public first-party verified registry is the only profile that certifies an install-ready result",
            ],
        ),
        seed_row(
            "cert:mirrored-registry",
            P::MirroredRegistry,
            ReviewableListingResult,
            ReviewableListingResult,
            &[MarketplaceDetailFactGrid, CompatibilityLabelStrip],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "mirror_source"],
            &[
                "marketplace detail fact grid names the mirrored source class alongside compatibility, host model, permission posture, and activation budget in one place",
                "compatibility-label strip names the compatibility range and host / runtime model against the mirrored copy",
                "text / JSON / Markdown reconstruction certified for support replay",
                "registry-install-truth: a mirrored registry stays reviewable and never reads as a fresh public install-ready result",
            ],
        ),
        seed_row(
            "cert:enterprise-registry",
            P::EnterpriseRegistry,
            ReviewableListingResult,
            ReviewableListingResult,
            &[PermissionManifestSummary, InstallUpdateDisableRollbackReviewSheet],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "enterprise_source"],
            &[
                "permission-manifest summary names the permission posture and any transitive widening against the enterprise source class",
                "install-update-disable-rollback review sheet names the disable scope and rollback compatibility before any mutation",
                "export preserves the enterprise source class and permission posture",
                "registry-install-truth: an enterprise registry is reviewed under its named source class and never implies a public install-ready result",
            ],
        ),
        seed_row(
            "cert:side-load-reviewed-registry",
            P::SideLoadReviewedRegistry,
            ReviewableListingResult,
            ReviewableListingResult,
            &[InstalledStateDiagnosticsCard, ActivationBudgetBand],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "side_load_source"],
            &[
                "installed-state diagnostics card names the side-load source class and the full quarantine history alongside the activation health",
                "activation-budget band names the within-budget band so the reviewed side-load never reads as cost-free",
                "text / JSON / Markdown reconstruction certified so support can replay the side-load review",
                "registry-install-truth: a reviewed side-load names its source class explicitly and never reads as a verified public install",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:stale-compatibility-registry",
            P::StaleCompatibilityRegistry,
            ReviewableListingResult,
            CompatibilityUnverifiedProjection,
            &[CompatibilityLabelStrip, MarketplaceResultRow],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the compatibility signal is stale so a current compatibility range cannot be certified",
                    "The compatibility signal for this listing is stale, so the ReviewableListingResult claim narrows to a compatibility-unverified projection and the compatibility-label strip preserves the last-known compatibility range rather than presenting the artifact as freshly compatible and ready to install",
                    Trig::CompatibilityRangeUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableListingResult,
                CompatibilityUnverifiedProjection,
                "Compatibility unverified: the compatibility signal is stale; the last-known compatibility range and host model are preserved and the artifact is never shown as freshly ready to install",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "compatibility-label strip preserves the last-known compatibility range and host model through the stale signal",
                "marketplace result row keeps the source class and the stale-evidence disclosure visible",
                "degraded-state: ReviewableListingResult narrows to a compatibility-unverified projection (auto-narrowed)",
                "registry-install-truth: a stale compatibility signal never masquerades as a ready install",
            ],
        ),
        seed_row(
            "cert:over-budget-throttled-registry",
            P::OverBudgetThrottledRegistry,
            ReviewableListingResult,
            ActivationBudgetProjection,
            &[ActivationBudgetBand, InstalledStateDiagnosticsCard],
            seed_certified_except(
                Ax::RegistryInstallTruth,
                seed_narrowed(
                    Ax::RegistryInstallTruth,
                    "the artifact is over budget / throttled so a cost-free install cannot be certified",
                    "The artifact is over its activation budget and throttled, so the ReviewableListingResult claim narrows to an activation-budget projection and the activation-budget band names the over-budget band and the throttling rather than presenting the install as cost-free",
                    Trig::ActivationCostHidden,
                ),
            ),
            Some(seed_narrow(
                Ax::RegistryInstallTruth,
                ReviewableListingResult,
                ActivationBudgetProjection,
                "Activation budget: the artifact is over budget and throttled; the over-budget band and the throttling are named and the install is never presented as cost-free",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "activation-budget band names the over-budget band and the throttling state rather than a cost-free install",
                "installed-state diagnostics card names the activation health so the over-budget cost is never hidden behind a healthy card",
                "registry-install-truth: ReviewableListingResult narrows to an activation-budget projection (auto-narrowed)",
                "registry-install-truth: an over-budget / throttled artifact is never presented as ready to install",
            ],
        ),
        seed_row(
            "cert:rollback-unverifiable-registry",
            P::RollbackUnverifiableRegistry,
            ReviewableListingResult,
            RollbackUnverifiedProjection,
            &[InstallUpdateDisableRollbackReviewSheet, MarketplaceDetailFactGrid],
            seed_certified_except(
                Ax::RegistryInstallTruth,
                seed_narrowed(
                    Ax::RegistryInstallTruth,
                    "the rollback's compatibility evidence is unverifiable so a clean revert cannot be certified",
                    "The rollback's compatibility evidence is unverifiable, so the ReviewableListingResult claim narrows to a rollback-unverified projection and the install-update-disable-rollback review sheet names the rollback limits and the disable scope before mutation rather than implying a clean revert",
                    Trig::RollbackIncompatibilityHidden,
                ),
            ),
            Some(seed_narrow(
                Ax::RegistryInstallTruth,
                ReviewableListingResult,
                RollbackUnverifiedProjection,
                "Rollback unverified: the rollback compatibility evidence is unverifiable; the rollback limits and the disable scope are named before mutation and a clean revert is never implied",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "install-update-disable-rollback review sheet names the disable scope and the rollback limits before any mutation",
                "marketplace detail fact grid keeps the source class and the rollback disclosure visible together",
                "registry-install-truth: ReviewableListingResult narrows to a rollback-unverified projection (auto-narrowed)",
                "registry-install-truth: an unverifiable rollback never reads as a clean-revert result and the disable scope is never hidden",
            ],
        ),
        seed_row(
            "cert:transferred-publisher-registry",
            P::TransferredPublisherRegistry,
            ReviewableListingResult,
            PublisherContinuityProjection,
            &[PublisherContinuityRow, MarketplaceResultRow],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the publisher continuity is transferred / unverifiable so a continuous publisher cannot be certified",
                    "The publisher continuity for this listing is transferred and unverifiable, so the ReviewableListingResult claim narrows to a publisher-continuity projection and the publisher-continuity row names the transfer rather than presenting the artifact as a continuous-publisher install",
                    Trig::PublisherTransferHidden,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableListingResult,
                PublisherContinuityProjection,
                "Publisher continuity: the publisher has transferred and continuity is unverifiable; the transfer is named and the artifact is never presented as a continuous-publisher install",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "publisher-continuity row names the publisher transfer so the listing never reads as a continuous first-party publisher",
                "marketplace result row keeps the source class and the transfer disclosure visible",
                "degraded-state: ReviewableListingResult narrows to a publisher-continuity projection (auto-narrowed)",
                "registry-install-truth: a transferred / unverifiable publisher is never presented as continuous",
            ],
        ),
    ]
}
