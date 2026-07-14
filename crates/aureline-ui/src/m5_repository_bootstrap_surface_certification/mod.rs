//! M05-1195 surface certification over the frozen M5 open-local / clone-remote / open-archive /
//! import-bundle / resume-snapshot repository-bootstrap matrix.
//!
//! Where the freeze matrix ([`crate::m5_repository_bootstrap_matrix`]) defines the five governed project-entry
//! acquisition families, the M05-1189..1192 implement lanes resolve each one, the M05-1193 shared-consumer lane
//! aligns their grammar across surfaces, and the M05-1194 accessibility lane
//! ([`crate::m5_repository_bootstrap_accessibility_parity_and_narrowing_when_checkout_plan_trust_stage_mirror_signer_continuity_or_bootstrap_evidence_is_stale_or_partial`])
//! proves keyboard / screen-reader / high-zoom / high-contrast / localization / CLI-export parity and
//! per-family auto-narrowing, this closing capstone *certifies* that the shared repository-bootstrap truth holds
//! on every claimed M5 project-entry profile — and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a user, reviewer, admin, or support engineer reads a source-locator,
//! checkout-plan, credential-posture, staged-trust, bootstrap-evidence, or post-open-queue surface through (a
//! live, first-party trusted acquisition surface; a reviewable acquisition structure; a disclosed checkout-plan
//! profile; an unverified trust-stage profile; and an unverified bootstrap-evidence profile), not on the
//! acquisition family or implement lane. Each [`RepositoryBootstrapProfileCertificationRow`] certifies one
//! profile across nine truth axes — visual, keyboard, screen-reader, high-zoom-reflow, high-contrast,
//! localization, CLI/export, degraded-state, and repository-bootstrap-component-truth behavior — and either
//! passes (green), auto-narrows its acquisition claim to the weakest supported ceiling (yellow), or is blocked
//! (red) when a degraded axis is hidden behind a fresh trusted claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `TrustedAcquisitionSurface` / `ReviewableAcquisitionSurface` claim while one of its truth axes is not current
//! is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with a bound
//! reason and a frozen downgrade trigger) is honestly yellow. Only a live, first-party trusted acquisition
//! surface profile may certify a `TrustedAcquisitionSurface` claim — a reviewable, disclosed-checkout-plan,
//! unverified-trust-stage, or unverified-bootstrap-evidence profile that keeps a trusted claim is over-reaching
//! and blocks. The always-on CLI/export axis must always stay certified so support and automation can
//! reconstruct the canonical source locator, checkout plan, credential posture, evidence packet, staged-trust
//! rule, post-open queue, and registry reference from the same repository-bootstrap truth the operator saw.
//!
//! The B142 hard invariants are enforced per row: no profile may rewrite clone into open because a local
//! checkout already exists, run repo-owned actions (hooks, tasks, extensions, package restores, submodule or
//! LFS hydration, generator installs) implicitly during acquisition, lose signer or mirror provenance across an
//! offline or mirrored fetch, strand partial acquisition without Resume / Discard / read-only choices, or hide
//! the bootstrap credential posture behind generic connected-state copy. A profile that breaches any invariant
//! blocks (red).
//!
//! Every row cites exactly one canonical repository-bootstrap proof bundle
//! ([`REPOSITORY_BOOTSTRAP_CERT_CANONICAL_BUNDLE_REF`]) — the frozen repository-bootstrap matrix proof — rather
//! than cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets, bearer
//! tokens, endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/workspaces/m5-repository-bootstrap-surface-certification.schema.json`](../../../../schemas/workspaces/m5-repository-bootstrap-surface-certification.schema.json).
//! The contract doc is
//! [`docs/workspaces/m5_repository_bootstrap_surface_certification.md`](../../../../docs/workspaces/m5_repository_bootstrap_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_repository_bootstrap_accessibility_parity_and_narrowing_when_checkout_plan_trust_stage_mirror_signer_continuity_or_bootstrap_evidence_is_stale_or_partial as a11y;
use crate::m5_repository_bootstrap_matrix as matrix;
use a11y::M5RepositoryBootstrapA11yClaim;
use matrix::{M5RepositoryBootstrapDowngradeTrigger, M5RepositoryBootstrapFamily};

/// Schema version stamped on the M05-1195 certification packet.
pub const REPOSITORY_BOOTSTRAP_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`RepositoryBootstrapProfileCertificationPacket`].
pub const REPOSITORY_BOOTSTRAP_CERT_RECORD_KIND: &str =
    "m5_repository_bootstrap_surface_certification_packet";

/// Stable record-kind tag carried by each [`RepositoryBootstrapProfileCertificationRow`].
pub const REPOSITORY_BOOTSTRAP_CERT_ROW_RECORD_KIND: &str =
    "m5_repository_bootstrap_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const REPOSITORY_BOOTSTRAP_CERT_SCHEMA_REF: &str =
    "schemas/workspaces/m5-repository-bootstrap-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const REPOSITORY_BOOTSTRAP_CERT_DOC_REF: &str =
    "docs/workspaces/m5_repository_bootstrap_surface_certification.md";

/// Repo-relative path of the frozen repository-bootstrap matrix schema the certified profiles render.
pub const REPOSITORY_BOOTSTRAP_CERT_MATRIX_REF: &str =
    matrix::M5_REPOSITORY_BOOTSTRAP_MATRIX_SCHEMA_REF;

/// The one canonical repository-bootstrap proof bundle every certified profile cites as its first-resolved
/// repository-bootstrap truth. All five profiles point back to it rather than cloning per-profile evidence.
pub const REPOSITORY_BOOTSTRAP_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_REPOSITORY_BOOTSTRAP_ARTIFACT_REF;

/// The M05-1194 accessibility support export the certification builds on. Recorded as a supporting evidence ref
/// on every row.
pub const REPOSITORY_BOOTSTRAP_CERT_A11Y_BUNDLE_REF: &str =
    a11y::REPOSITORY_BOOTSTRAP_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const REPOSITORY_BOOTSTRAP_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-repository-bootstrap-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const REPOSITORY_BOOTSTRAP_CERT_CSV_REF: &str =
    "artifacts/release/m5-repository-bootstrap-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const REPOSITORY_BOOTSTRAP_CERT_REPORT_REF: &str =
    "artifacts/release/m5-repository-bootstrap-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const REPOSITORY_BOOTSTRAP_CERT_FIXTURE_DIR: &str =
    "fixtures/workspaces/m5-repository-bootstrap-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const REPOSITORY_BOOTSTRAP_CERT_PACKET_ID: &str =
    "m5-repository-bootstrap-surface-certification:stable:0001";

/// The five claimed M5 project-entry operating profiles this capstone certifies. Keyed on the profile a user,
/// reviewer, admin, or support engineer reads a source-locator, checkout-plan, credential-posture, staged-trust,
/// bootstrap-evidence, or post-open-queue surface through, not on the reusable acquisition family it renders.
/// Only a live, first-party trusted acquisition profile may certify a trusted acquisition surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RepositoryBootstrapCertifiedProfile {
    /// A live, first-party, fully-current acquisition surface — a registry-bound, credential-posture-disclosed,
    /// checkout-plan-visible, trust-staged, evidence-backed open-local acquisition rendering the trusted
    /// acquisition claim exactly right now.
    LiveTrustedAcquisitionSurface,
    /// A reviewable acquisition structure: a self-sufficient, inspectable repository-bootstrap projection (a
    /// source-locator / checkout-plan / registry reference) an admin can review, never itself an authoritative,
    /// live-resolving acquisition surface.
    ReviewableAcquisitionStructure,
    /// An open-archive / checkout-plan surface whose checkout-plan proof can only be partially disclosed; the
    /// claim narrows to a checkout-plan-disclosed projection that discloses the partial checkout plan alongside
    /// the resolved topology, never a collapsed checkout shown as fully hydrated when its plan proof is
    /// incomplete.
    DisclosedCheckoutPlanProfile,
    /// An import-bundle staged-trust surface whose staged-trust fence cannot be confirmed; the claim narrows to a
    /// trust-stage-unverified projection that keeps the last-known deferred-repo-action posture explicit, never a
    /// bootstrap shown as fully trusted when a repo-owned action may have run implicitly or trust may have
    /// widened before browse-safe metadata.
    UnverifiedTrustStageProfile,
    /// A resume-snapshot bootstrap-evidence surface whose signer / mirror provenance or bootstrap-evidence has
    /// aged out or is policy-blocked; the claim narrows to a bootstrap-evidence-unverified projection that keeps
    /// the last-known partial-acquisition posture explicit, never a partial acquisition shown as a healthy full
    /// checkout when its evidence has aged out or its provenance is lost.
    UnverifiedBootstrapEvidenceProfile,
}

impl M5RepositoryBootstrapCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5RepositoryBootstrapCertifiedProfile; 5] = [
        M5RepositoryBootstrapCertifiedProfile::LiveTrustedAcquisitionSurface,
        M5RepositoryBootstrapCertifiedProfile::ReviewableAcquisitionStructure,
        M5RepositoryBootstrapCertifiedProfile::DisclosedCheckoutPlanProfile,
        M5RepositoryBootstrapCertifiedProfile::UnverifiedTrustStageProfile,
        M5RepositoryBootstrapCertifiedProfile::UnverifiedBootstrapEvidenceProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrustedAcquisitionSurface => "live_trusted_acquisition_surface",
            Self::ReviewableAcquisitionStructure => "reviewable_acquisition_structure",
            Self::DisclosedCheckoutPlanProfile => "disclosed_checkout_plan_profile",
            Self::UnverifiedTrustStageProfile => "unverified_trust_stage_profile",
            Self::UnverifiedBootstrapEvidenceProfile => "unverified_bootstrap_evidence_profile",
        }
    }

    /// True only for the live, first-party trusted acquisition surface profile. A trusted acquisition surface may
    /// be certified on this profile alone; every other profile is at most a reviewable acquisition structure or a
    /// narrowed projection.
    pub const fn is_live_trusted_acquisition_surface(self) -> bool {
        matches!(self, Self::LiveTrustedAcquisitionSurface)
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and repository-bootstrap-component-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryBootstrapCertificationAxis {
    /// Visual parity: canonical source locator, checkout plan, credential posture, evidence packet, staged-trust
    /// rule, post-open queue, and registry reference are shown on the primary surface without relying on a
    /// shell-chrome-only affordance or a mislabeled screenshot alone.
    Visual,
    /// Keyboard-reach parity: the same repository-bootstrap truth and its bound operations are reachable and
    /// operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a shell-chrome-only
    /// affordance, a mislabeled screenshot, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the source
    /// locator, checkout plan, credential posture, evidence packet, or registry reference.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping the
    /// source locator, checkout plan, or credential posture.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling an
    /// acquisition verb, checkout mode, or credential-posture class when a locale is incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a partially-disclosed checkout plan, an unconfirmed staged-trust fence, or an
    /// aged-out / policy-blocked bootstrap-evidence proof honestly downgrades a
    /// `TrustedAcquisitionSurface` / `ReviewableAcquisitionSurface` claim rather than reading as a fresh,
    /// authoritative acquisition surface.
    DegradedState,
    /// Repository-bootstrap-component-truth parity: canonical source locator, checkout plan, credential posture,
    /// evidence packet, staged-trust rule, post-open queue, and registry reference stay explicit and never let an
    /// acquisition rewrite clone into open over an existing checkout, run repo-owned actions implicitly during
    /// acquisition, lose signer or mirror provenance across an offline or mirrored fetch, strand partial
    /// acquisition without Resume / Discard / read-only choices, or hide the bootstrap credential posture behind
    /// generic connected-state copy.
    RepositoryBootstrapComponentTruth,
}

impl RepositoryBootstrapCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [RepositoryBootstrapCertificationAxis; 9] = [
        RepositoryBootstrapCertificationAxis::Visual,
        RepositoryBootstrapCertificationAxis::Keyboard,
        RepositoryBootstrapCertificationAxis::ScreenReader,
        RepositoryBootstrapCertificationAxis::HighZoomReflow,
        RepositoryBootstrapCertificationAxis::HighContrast,
        RepositoryBootstrapCertificationAxis::Localization,
        RepositoryBootstrapCertificationAxis::CliExport,
        RepositoryBootstrapCertificationAxis::DegradedState,
        RepositoryBootstrapCertificationAxis::RepositoryBootstrapComponentTruth,
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
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrast => "high_contrast",
            Self::Localization => "localization",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::RepositoryBootstrapComponentTruth => "repository_bootstrap_component_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryBootstrapAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl RepositoryBootstrapAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the author — always recomputed from
/// the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryBootstrapProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed acquisition tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-live profile claims a trusted acquisition surface, or the narrowing is inconsistent.
    Red,
}

impl RepositoryBootstrapProfileClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the profile is publishable as certified (green or disclosed yellow); red profiles block the
    /// release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The five B142 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile carries
/// all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapCertGuardrails {
    /// True if the profile rewrites clone into open because a local checkout already exists. Must be false.
    pub rewrites_clone_into_open_when_local_checkout_already_exists: bool,
    /// True if the profile runs repo-owned actions (hooks, tasks, extensions, restores, generators) implicitly
    /// during acquisition. Must be false.
    pub runs_repo_owned_actions_implicitly_during_acquisition: bool,
    /// True if the profile loses signer or mirror provenance across an offline or mirrored fetch. Must be false.
    pub loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches: bool,
    /// True if the profile strands partial acquisition without Resume / Discard / read-only choices. Must be
    /// false.
    pub strands_partial_acquisition_without_resume_discard_or_readonly_choices: bool,
    /// True if the profile hides the bootstrap credential posture behind generic connected-state copy. Must be
    /// false.
    pub hides_bootstrap_credential_posture_behind_generic_connected_state_copy: bool,
}

impl RepositoryBootstrapCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        rewrites_clone_into_open_when_local_checkout_already_exists: false,
        runs_repo_owned_actions_implicitly_during_acquisition: false,
        loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches: false,
        strands_partial_acquisition_without_resume_discard_or_readonly_choices: false,
        hides_bootstrap_credential_posture_behind_generic_connected_state_copy: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.rewrites_clone_into_open_when_local_checkout_already_exists
            && !self.runs_repo_owned_actions_implicitly_during_acquisition
            && !self.loses_signer_or_mirror_provenance_across_offline_or_mirrored_fetches
            && !self.strands_partial_acquisition_without_resume_discard_or_readonly_choices
            && !self.hides_bootstrap_credential_posture_behind_generic_connected_state_copy
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this offers
/// text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The source-locator / checkout-plan / credential-posture / evidence-packet / staged-trust /
    /// post-open-queue / registry-reference fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl RepositoryBootstrapCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a raw-payload-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.raw_payload_only_prohibited
    }
}

/// One axis outcome on one certified profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: RepositoryBootstrapCertificationAxis,
    /// The certification state of the axis.
    pub state: RepositoryBootstrapAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5RepositoryBootstrapDowngradeTrigger>,
}

impl RepositoryBootstrapAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible trigger (that is exactly
    ///   what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            RepositoryBootstrapAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            RepositoryBootstrapAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            RepositoryBootstrapAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current. Present iff the certified
/// claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: RepositoryBootstrapCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5RepositoryBootstrapA11yClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5RepositoryBootstrapA11yClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 project-entry profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapProfileCertificationRow {
    /// Record kind; must equal [`REPOSITORY_BOOTSTRAP_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`REPOSITORY_BOOTSTRAP_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5RepositoryBootstrapCertifiedProfile,
    /// The acquisition claim ceiling the profile asserts.
    pub claimed_claim: M5RepositoryBootstrapA11yClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5RepositoryBootstrapA11yClaim,
    /// The frozen acquisition families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5RepositoryBootstrapFamily>,
    /// One outcome per [`RepositoryBootstrapCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<RepositoryBootstrapAxisOutcome>,
    /// The B142 hard invariants; all must hold.
    pub guardrails: RepositoryBootstrapCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<RepositoryBootstrapClaimAutoNarrow>,
    /// The one canonical repository-bootstrap proof bundle this profile cites. Must equal
    /// [`REPOSITORY_BOOTSTRAP_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: RepositoryBootstrapProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: RepositoryBootstrapCertExportParity,
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

impl RepositoryBootstrapProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: RepositoryBootstrapCertificationAxis,
    ) -> Option<&RepositoryBootstrapAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<RepositoryBootstrapCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && RepositoryBootstrapCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(RepositoryBootstrapAxisOutcome::well_formed)
    }

    /// True when the profile narrows its acquisition claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<RepositoryBootstrapCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == RepositoryBootstrapAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a live first-party profile may
    /// certify a trusted acquisition surface, every hard invariant must hold, CLI/export parity must always
    /// certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> RepositoryBootstrapProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != REPOSITORY_BOOTSTRAP_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return RepositoryBootstrapProfileClaimStatus::Red;
        }

        // Every B142 hard invariant must hold.
        if !self.guardrails.all_held() {
            return RepositoryBootstrapProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return RepositoryBootstrapProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a trusted acquisition surface.
        if self.certified_claim.asserts_trusted_surface()
            && !self.profile.is_live_trusted_acquisition_surface()
        {
            return RepositoryBootstrapProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(RepositoryBootstrapCertificationAxis::CliExport) {
            Some(o) if o.state == RepositoryBootstrapAxisCertificationState::Certified => {}
            _ => return RepositoryBootstrapProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == RepositoryBootstrapAxisCertificationState::UndisclosedDrift)
        {
            return RepositoryBootstrapProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return RepositoryBootstrapProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return RepositoryBootstrapProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return RepositoryBootstrapProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return RepositoryBootstrapProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return RepositoryBootstrapProfileClaimStatus::Red;
        }

        RepositoryBootstrapProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == REPOSITORY_BOOTSTRAP_CERT_ROW_RECORD_KIND
            && self.schema_version == REPOSITORY_BOOTSTRAP_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1195 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapProfileCertificationSummary {
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

/// Constructor input for [`RepositoryBootstrapProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryBootstrapProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<RepositoryBootstrapProfileCertificationRow>,
}

/// Checked-in M05-1195 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepositoryBootstrapProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<RepositoryBootstrapProfileCertificationRow>,
    pub summary: RepositoryBootstrapProfileCertificationSummary,
}

impl RepositoryBootstrapProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: RepositoryBootstrapProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: REPOSITORY_BOOTSTRAP_CERT_SCHEMA_VERSION,
            record_kind: REPOSITORY_BOOTSTRAP_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: RepositoryBootstrapProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5RepositoryBootstrapCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Acquisition families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5RepositoryBootstrapFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5RepositoryBootstrapCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen acquisition family is certified on at least one profile — proof the full matrix runs
    /// across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5RepositoryBootstrapFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(RepositoryBootstrapCertificationAxis::CliExport)
                .is_some_and(|o| o.state == RepositoryBootstrapAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> RepositoryBootstrapProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == RepositoryBootstrapProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == RepositoryBootstrapProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == RepositoryBootstrapProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(RepositoryBootstrapProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        RepositoryBootstrapProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == REPOSITORY_BOOTSTRAP_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(RepositoryBootstrapProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<RepositoryBootstrapCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != REPOSITORY_BOOTSTRAP_CERT_SCHEMA_VERSION {
            violations.push(RepositoryBootstrapCertificationViolation::SchemaVersion {
                expected: REPOSITORY_BOOTSTRAP_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != REPOSITORY_BOOTSTRAP_CERT_RECORD_KIND {
            violations.push(RepositoryBootstrapCertificationViolation::RecordKind {
                expected: REPOSITORY_BOOTSTRAP_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(RepositoryBootstrapCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != REPOSITORY_BOOTSTRAP_CERT_CANONICAL_BUNDLE_REF {
            violations.push(RepositoryBootstrapCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(RepositoryBootstrapCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(RepositoryBootstrapCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    RepositoryBootstrapCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    RepositoryBootstrapCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != REPOSITORY_BOOTSTRAP_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    RepositoryBootstrapCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B142 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(
                    RepositoryBootstrapCertificationViolation::GuardrailViolated {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Only a live first-party profile may certify a trusted acquisition surface.
            if row.certified_claim.asserts_trusted_surface()
                && !row.profile.is_live_trusted_acquisition_surface()
            {
                violations.push(
                    RepositoryBootstrapCertificationViolation::NonLiveProfileClaimsTrustedSurface {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(RepositoryBootstrapCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    RepositoryBootstrapCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    RepositoryBootstrapCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    RepositoryBootstrapCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == RepositoryBootstrapProfileClaimStatus::Red {
                violations.push(RepositoryBootstrapCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(RepositoryBootstrapCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen acquisition family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(RepositoryBootstrapCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(RepositoryBootstrapCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(
                RepositoryBootstrapCertificationViolation::RawRepositoryBootstrapMaterialInExport,
            );
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
        out.push_str("# M5 Repository-Bootstrap Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5RepositoryBootstrapCertifiedProfile::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Invariants held: {}\n",
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
pub fn current_m5_repository_bootstrap_surface_certification_export() -> Result<
    RepositoryBootstrapProfileCertificationPacket,
    RepositoryBootstrapCertificationArtifactError,
> {
    let packet: RepositoryBootstrapProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-repository-bootstrap-surface-certification/support_export.json"
        )))
        .map_err(RepositoryBootstrapCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RepositoryBootstrapCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum RepositoryBootstrapCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RepositoryBootstrapCertificationViolation>),
}

impl fmt::Display for RepositoryBootstrapCertificationArtifactError {
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

impl Error for RepositoryBootstrapCertificationArtifactError {}

/// Validation failure for M05-1195 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryBootstrapCertificationViolation {
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
    NonLiveProfileClaimsTrustedSurface { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawRepositoryBootstrapMaterialInExport,
}

impl fmt::Display for RepositoryBootstrapCertificationViolation {
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
                    "packet does not cite the canonical repository-bootstrap proof bundle"
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
                    "row {id} does not cite the one canonical repository-bootstrap proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B142 hard invariant: rewriting clone into open when a local checkout \
already exists; running repo-owned actions implicitly during acquisition; losing signer or mirror provenance \
across an offline or mirrored fetch; stranding partial acquisition without Resume / Discard / read-only \
choices; or hiding the bootstrap credential posture behind generic connected-state copy"
                )
            }
            Self::NonLiveProfileClaimsTrustedSurface { id } => {
                write!(
                    f,
                    "row {id} certifies a trusted acquisition surface on a non-live first-party profile"
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
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh trusted claim, a hard \
invariant broke, CLI/export parity dropped, a non-live profile claimed a trusted acquisition surface, or the \
narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 project-entry profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen acquisition family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawRepositoryBootstrapMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for RepositoryBootstrapCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&RepositoryBootstrapAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != RepositoryBootstrapAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the repository-bootstrap
/// generics the spec forbids collapsing distinct source-locator, checkout-plan, credential-posture, staged-trust,
/// and bootstrap-evidence truth into (whole-label matches so a full sentence naming a concrete locator, checkout
/// mode, or registry reference is not flagged).
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
            | "something went wrong"
            | "degraded"
            | "narrowed"
            | "reduced"
            | "stale"
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "pending"
            | "loading"
            | "partial"
            | "cached"
            | "trusted"
            | "reviewable"
            | "acquisition"
            | "repository"
            | "bootstrap"
            | "source"
            | "locator"
            | "source locator"
            | "checkout"
            | "plan"
            | "checkout plan"
            | "credential"
            | "posture"
            | "credential posture"
            | "evidence"
            | "evidence packet"
            | "trust"
            | "stage"
            | "trust stage"
            | "staged trust"
            | "queue"
            | "post-open queue"
            | "signer"
            | "mirror"
            | "provenance"
            | "clone"
            | "open"
            | "archive"
            | "import"
            | "bundle"
            | "resume"
            | "snapshot"
            | "registry reference"
            | "more"
            | "…"
            | "..."
            | "overflow"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the repository-bootstrap
/// matrix and M05-1194 heuristic so the reused [`M5RepositoryBootstrapDowngradeTrigger`] narrowings serialize
/// cleanly — the repository-bootstrap grammar carries only typed class tokens and opaque refs, never raw secret
/// values or endpoints.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
                || lower.contains("://")
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

/// Builds the canonical, checked-in M05-1195 certification packet. Certifies all five claimed M5 project-entry
/// profiles: two deliver their claim (green) and three auto-narrow a not-current truth axis to a weaker
/// acquisition ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_repository_bootstrap_surface_certification_packet(
) -> RepositoryBootstrapProfileCertificationPacket {
    RepositoryBootstrapProfileCertificationPacket::new(
        RepositoryBootstrapProfileCertificationPacketInput {
            packet_id: REPOSITORY_BOOTSTRAP_CERT_PACKET_ID.to_owned(),
            as_of: "2026-07-14T00:00:00Z".to_owned(),
            matrix_ref: REPOSITORY_BOOTSTRAP_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: REPOSITORY_BOOTSTRAP_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:repository-bootstrap-surface-certification:{id}"),
        REPOSITORY_BOOTSTRAP_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> RepositoryBootstrapCertExportParity {
    RepositoryBootstrapCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: RepositoryBootstrapCertificationAxis) -> &'static str {
    match axis {
        RepositoryBootstrapCertificationAxis::Visual => {
            "canonical source locator, checkout plan, credential posture, evidence packet, staged-trust rule, post-open queue, and registry reference shown on-surface without a shell-chrome-only affordance or a mislabeled screenshot alone"
        }
        RepositoryBootstrapCertificationAxis::Keyboard => {
            "the same repository-bootstrap role, registry reference, and bound operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        RepositoryBootstrapCertificationAxis::ScreenReader => {
            "the same repository-bootstrap truth is announced non-visually, never a shell-chrome-only / mislabeled-screenshot / unlabeled-control-only cue"
        }
        RepositoryBootstrapCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the source locator, checkout plan, credential posture, evidence packet, or registry reference"
        }
        RepositoryBootstrapCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the source locator, checkout plan, or credential posture"
        }
        RepositoryBootstrapCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling an acquisition verb, checkout mode, or credential-posture class"
        }
        RepositoryBootstrapCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        RepositoryBootstrapCertificationAxis::DegradedState => {
            "a partially-disclosed checkout plan, an unconfirmed staged-trust fence, or an aged-out / policy-blocked bootstrap-evidence proof honestly downgrades the TrustedAcquisitionSurface/ReviewableAcquisitionSurface claim rather than reading as a fresh authoritative acquisition surface"
        }
        RepositoryBootstrapCertificationAxis::RepositoryBootstrapComponentTruth => {
            "canonical source locator, checkout plan, credential posture, evidence packet, staged-trust rule, post-open queue, and registry reference stay explicit and never let an acquisition rewrite clone into open over an existing checkout, run repo-owned actions implicitly during acquisition, lose signer or mirror provenance across an offline or mirrored fetch, strand partial acquisition without Resume / Discard / read-only choices, or hide the bootstrap credential posture behind generic connected-state copy"
        }
    }
}

fn seed_certified(axis: RepositoryBootstrapCertificationAxis) -> RepositoryBootstrapAxisOutcome {
    RepositoryBootstrapAxisOutcome {
        axis,
        state: RepositoryBootstrapAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: RepositoryBootstrapCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5RepositoryBootstrapDowngradeTrigger,
) -> RepositoryBootstrapAxisOutcome {
    RepositoryBootstrapAxisOutcome {
        axis,
        state: RepositoryBootstrapAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<RepositoryBootstrapAxisOutcome> {
    RepositoryBootstrapCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: RepositoryBootstrapCertificationAxis,
    outcome: RepositoryBootstrapAxisOutcome,
) -> Vec<RepositoryBootstrapAxisOutcome> {
    RepositoryBootstrapCertificationAxis::ALL
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
    profile: M5RepositoryBootstrapCertifiedProfile,
    claimed_claim: M5RepositoryBootstrapA11yClaim,
    certified_claim: M5RepositoryBootstrapA11yClaim,
    consumed_families: &[M5RepositoryBootstrapFamily],
    axis_outcomes: Vec<RepositoryBootstrapAxisOutcome>,
    claim_auto_narrow: Option<RepositoryBootstrapClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> RepositoryBootstrapProfileCertificationRow {
    let mut row = RepositoryBootstrapProfileCertificationRow {
        record_kind: REPOSITORY_BOOTSTRAP_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: REPOSITORY_BOOTSTRAP_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: RepositoryBootstrapCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: REPOSITORY_BOOTSTRAP_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: RepositoryBootstrapProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            REPOSITORY_BOOTSTRAP_CERT_MATRIX_REF.to_owned(),
            REPOSITORY_BOOTSTRAP_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-14T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: RepositoryBootstrapCertificationAxis,
    from_claim: M5RepositoryBootstrapA11yClaim,
    to_claim: M5RepositoryBootstrapA11yClaim,
    label: &str,
) -> RepositoryBootstrapClaimAutoNarrow {
    RepositoryBootstrapClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<RepositoryBootstrapProfileCertificationRow> {
    use M5RepositoryBootstrapA11yClaim::*;
    use M5RepositoryBootstrapCertifiedProfile as P;
    use M5RepositoryBootstrapDowngradeTrigger as Trig;
    use M5RepositoryBootstrapFamily::*;
    use RepositoryBootstrapCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-trusted-acquisition-surface",
            P::LiveTrustedAcquisitionSurface,
            TrustedAcquisitionSurface,
            TrustedAcquisitionSurface,
            &[OpenLocal],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "source_locator",
            ],
            &[
                "open-local profile: an existing local checkout is opened in place rather than recloned over it, and the source locator, checkout plan, and credential posture stay visible and attributable rather than merged into an opaque connected-state blob",
                "the trusted acquisition surface keeps stable operation IDs while the source locator, checkout plan, and credential posture bind to the one repository-bootstrap registry across acquisition-engine / shell / diagnostics / support",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered acquisition surface",
                "repository-bootstrap-component-truth: a live first-party acquisition surface is the only profile that certifies a trusted acquisition surface",
            ],
        ),
        seed_row(
            "cert:reviewable-acquisition-structure",
            P::ReviewableAcquisitionStructure,
            ReviewableAcquisitionSurface,
            ReviewableAcquisitionSurface,
            &[CloneRemote],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "checkout_plan",
            ],
            &[
                "clone-remote profile: the checkout cost, topology, and credential posture stay bound to the single repository-bootstrap registry and are shown before the fetch rather than a per-surface description copied by hand",
                "the reviewable acquisition structure keeps its source-locator, checkout-plan, credential-posture, and registry labels inspectable rather than a shell-chrome-only or mislabeled-screenshot cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable acquisition structure",
                "repository-bootstrap-component-truth: a reviewable acquisition structure never certifies a live trusted, authoritative acquisition claim and never rewrites clone into open over an existing checkout",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:disclosed-checkout-plan-profile",
            P::DisclosedCheckoutPlanProfile,
            ReviewableAcquisitionSurface,
            CheckoutPlanDisclosedProjection,
            &[OpenArchive],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "the open-archive checkout-plan proof can only be partially disclosed for this profile so a fully hydrated checkout cannot be certified as proven",
                    "The open-archive checkout-plan proof can only be partially disclosed, so the ReviewableAcquisitionSurface claim narrows to a checkout-plan-disclosed projection and the acquisition discloses the partial checkout plan alongside the resolved topology rather than presenting a collapsed checkout as fully hydrated when its plan proof is incomplete",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableAcquisitionSurface,
                CheckoutPlanDisclosedProjection,
                "Checkout plan disclosed partial: the open-archive checkout is only partially proven so it is disclosed alongside the resolved topology and no collapsed checkout is shown as fully hydrated",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "open-archive profile: the acquisition verifies the archive digest and extraction plan first and keeps the resolved topology explicit, marking the checkout-plan proof as disclosed-partial rather than deleting checkout structure silently when a dependency is missing",
                "the open-archive surface keeps its checkout plan and resolved topology legible while the plan proof is disclosed as partial",
                "localization: ReviewableAcquisitionSurface narrows to a checkout-plan-disclosed projection (auto-narrowed)",
                "repository-bootstrap-component-truth: a missing dependency never rewrites the archive open into a clone or runs repo-owned actions implicitly — the resolved topology is preserved",
            ],
        ),
        seed_row(
            "cert:unverified-trust-stage-profile",
            P::UnverifiedTrustStageProfile,
            ReviewableAcquisitionSurface,
            TrustStageUnverifiedProjection,
            &[ImportBundle],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the import-bundle staged-trust fence cannot be confirmed so a fully trusted bootstrap cannot be certified",
                    "The import-bundle staged-trust fence cannot be confirmed, so the ReviewableAcquisitionSurface claim narrows to a trust-stage-unverified projection and the acquisition keeps the last-known deferred-repo-action posture explicit rather than presenting a bootstrap as fully trusted when a repo-owned action may have run implicitly or trust may have widened before browse-safe metadata",
                    Trig::StagedTrustRuleUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableAcquisitionSurface,
                TrustStageUnverifiedProjection,
                "Trust stage unverified: the staged-trust fence cannot be confirmed so the last-known deferred-repo-action posture stays explicit and no bootstrap is shown as fully trusted when a repo-owned action may have run implicitly",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "import-bundle profile: the acquisition keeps its staged-trust posture explicit and marks the trust fence as unverified rather than silently running hooks, repo tasks, extensions, package restores, submodule or LFS hydration, or generator installs, and never widens trust before browse-safe metadata",
                "the import-bundle surface keeps its deferred repo-owned actions and post-open queue legible while the staged-trust fence is disclosed as unverified",
                "degraded-state: ReviewableAcquisitionSurface narrows to a trust-stage-unverified projection (auto-narrowed)",
                "repository-bootstrap-component-truth: an acquisition never runs repo-owned actions implicitly and never overclaims full trust when only browse-safe metadata was computed",
            ],
        ),
        seed_row(
            "cert:unverified-bootstrap-evidence-profile",
            P::UnverifiedBootstrapEvidenceProfile,
            ReviewableAcquisitionSurface,
            BootstrapEvidenceUnverifiedProjection,
            &[ResumeSnapshot],
            seed_certified_except(
                Ax::RepositoryBootstrapComponentTruth,
                seed_narrowed(
                    Ax::RepositoryBootstrapComponentTruth,
                    "the signer / mirror provenance or bootstrap-evidence has aged out or is policy-blocked so a healthy full checkout cannot be certified",
                    "The signer / mirror provenance or bootstrap-evidence has aged out or is policy-blocked, so the ReviewableAcquisitionSurface claim narrows to a bootstrap-evidence-unverified projection and the acquisition keeps the last-known partial-acquisition posture explicit rather than presenting a partial acquisition as a healthy full checkout or losing signer or mirror provenance when its evidence has aged out or become policy-blocked",
                    Trig::LostSignerOrMirrorProvenanceAcrossOfflineOrMirroredFetches,
                ),
            ),
            Some(seed_narrow(
                Ax::RepositoryBootstrapComponentTruth,
                ReviewableAcquisitionSurface,
                BootstrapEvidenceUnverifiedProjection,
                "Bootstrap evidence unverified: the signer / mirror provenance or bootstrap-evidence has aged out or is policy-blocked so the last-known partial-acquisition posture stays explicit and no partial acquisition is shown as a healthy full checkout when its evidence is stale",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "resume-snapshot profile: the acquisition keeps its last-known partial-acquisition posture and Resume / Discard / read-only choices explicit and marks the bootstrap evidence as unverified rather than stranding partial acquisition or presenting it as a healthy full checkout",
                "the resume-snapshot surface keeps its partial-acquisition checkpoint and post-open queue legible while the bootstrap evidence is disclosed as unverified",
                "repository-bootstrap-component-truth: ReviewableAcquisitionSurface narrows to a bootstrap-evidence-unverified projection (auto-narrowed)",
                "repository-bootstrap-component-truth: an offline or mirrored fetch never loses signer or mirror provenance, and no acquisition claim outpaces the resolved bootstrap evidence",
            ],
        ),
    ]
}
