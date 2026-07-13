//! M05-1171 surface certification over the frozen M5 platform-convention / shortcut-notation /
//! file-path-reveal / theme-contrast-live-change / credential-store-wording / input-method platform-fit
//! matrix.
//!
//! Where the freeze matrix ([`crate::m5_platform_fit_matrix`]) defines the six governed platform-fit
//! families, the M05-1165..1168 implement lanes narrow each one, the M05-1169 shared-consumer lane aligns
//! their grammar across surfaces, and the M05-1170 accessibility lane
//! ([`crate::m5_platform_fit_accessibility_parity_and_narrowing_when_platform_convention_native_affordance_or_input_method_truth_is_stale`])
//! proves keyboard / screen-reader / high-zoom / high-contrast / localization / CLI-export parity and
//! per-family auto-narrowing, this closing capstone *certifies* that the shared platform-fit truth holds on
//! every claimed M5 desktop operating profile — and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a user, reviewer, or support engineer reads a shortcut-notation,
//! window/menu, file-path/reveal, live-appearance, credential-store, or input-method surface through (a live,
//! first-party trusted platform-fit surface; a reviewable platform-fit structure; a disclosed
//! path-terminology profile; an unverified appearance-response profile; an unverified credential-wording
//! profile; and an unverified input-fidelity profile), not on platform-fit family or implement lane. Each
//! [`PlatformFitProfileCertificationRow`] certifies one profile across nine truth axes — visual, keyboard,
//! screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and
//! platform-fit-component-truth behavior — and either passes (green), auto-narrows its platform-fit claim to
//! the weakest supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind a fresh
//! trusted claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `TrustedPlatformFitSurface` / `ReviewablePlatformFitSurface` claim while one of its truth axes is not
//! current is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with a
//! bound reason and a frozen downgrade trigger) is honestly yellow. Only a live, first-party trusted
//! platform-fit profile may certify a `TrustedPlatformFitSurface` claim — a reviewable, disclosed-path,
//! unverified-appearance, unverified-credential, or unverified-input profile that keeps a trusted claim is
//! over-reaching and blocks. The always-on CLI/export axis must always stay certified so support and
//! automation can reconstruct the canonical host platform, shortcut notation, path/reveal verb, appearance
//! posture, credential-store wording, input-method fidelity, and registry reference from the same
//! platform-fit truth the user saw.
//!
//! The B139 hard invariants are enforced per row: no profile may let platform-specific wording change command
//! or permission meaning, hide a primary action only in OS chrome (menus / title bars), silently fall back to
//! plaintext secret storage, let an input method corrupt text or trust fidelity, or produce a screenshot or
//! docs page that mislabels a shortcut or path/reveal verb. A profile that breaches any invariant blocks
//! (red).
//!
//! Every row cites exactly one canonical platform-fit proof bundle
//! ([`PLATFORM_FIT_CERT_CANONICAL_BUNDLE_REF`]) — the frozen platform-fit matrix proof — rather than cloning
//! per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets, bearer tokens,
//! endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/platform/m5-platform-fit-surface-certification.schema.json`](../../../../schemas/platform/m5-platform-fit-surface-certification.schema.json).
//! The contract doc is
//! [`docs/platform/m5_platform_fit_surface_certification.md`](../../../../docs/platform/m5_platform_fit_surface_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_platform_fit_accessibility_parity_and_narrowing_when_platform_convention_native_affordance_or_input_method_truth_is_stale as a11y;
use crate::m5_platform_fit_matrix as matrix;
use a11y::M5PlatformFitA11yClaim;
use matrix::{M5PlatformFitDowngradeTrigger, M5PlatformFitFamily};

/// Schema version stamped on the M05-1171 certification packet.
pub const PLATFORM_FIT_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`PlatformFitProfileCertificationPacket`].
pub const PLATFORM_FIT_CERT_RECORD_KIND: &str = "m5_platform_fit_surface_certification_packet";

/// Stable record-kind tag carried by each [`PlatformFitProfileCertificationRow`].
pub const PLATFORM_FIT_CERT_ROW_RECORD_KIND: &str = "m5_platform_fit_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const PLATFORM_FIT_CERT_SCHEMA_REF: &str =
    "schemas/platform/m5-platform-fit-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const PLATFORM_FIT_CERT_DOC_REF: &str =
    "docs/platform/m5_platform_fit_surface_certification.md";

/// Repo-relative path of the frozen platform-fit matrix schema the certified profiles render.
pub const PLATFORM_FIT_CERT_MATRIX_REF: &str = matrix::M5_PLATFORM_FIT_MATRIX_SCHEMA_REF;

/// The one canonical platform-fit proof bundle every certified profile cites as its first-resolved
/// platform-fit truth. All six profiles point back to it rather than cloning per-profile evidence.
pub const PLATFORM_FIT_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_PLATFORM_FIT_ARTIFACT_REF;

/// The M05-1170 accessibility support export the certification builds on. Recorded as a supporting evidence
/// ref on every row.
pub const PLATFORM_FIT_CERT_A11Y_BUNDLE_REF: &str = a11y::PLATFORM_FIT_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const PLATFORM_FIT_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-platform-fit-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const PLATFORM_FIT_CERT_CSV_REF: &str =
    "artifacts/release/m5-platform-fit-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const PLATFORM_FIT_CERT_REPORT_REF: &str =
    "artifacts/release/m5-platform-fit-surface-certification.md";

/// Stable packet id for the checked-in certification bundle.
pub const PLATFORM_FIT_CERT_PACKET_ID: &str = "m5-platform-fit-surface-certification:stable:0001";

/// The six claimed M5 desktop platform-fit operating profiles this capstone certifies. Keyed on the profile
/// a user, reviewer, or support engineer reads a shortcut-notation, window/menu, file-path/reveal,
/// live-appearance, credential-store, or input-method surface through, not on the reusable platform-fit
/// family it renders. Only a live, first-party trusted platform-fit profile may certify a trusted
/// platform-fit surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlatformFitCertifiedProfile {
    /// A live, first-party, fully-current platform-fit surface — a registry-bound, host-correct,
    /// live-appearance, truthful-credential-wording, text-faithful shell rendering the trusted platform-fit
    /// convention exactly right now.
    LiveTrustedPlatformFitSurface,
    /// A reviewable platform-fit structure: a self-sufficient, inspectable shortcut-notation / registry
    /// reference a user can review, never itself an authoritative, live-rendering platform-fit surface.
    ReviewablePlatformFitStructure,
    /// A file / path / reveal / save terminology surface whose localization can only be partially disclosed;
    /// the claim narrows to a path-terminology-disclosed projection that discloses the partial localization
    /// alongside the last-known host-correct verb, never a mislabeled path / reveal verb shown as host-correct
    /// when its localization is incomplete.
    DisclosedPathTerminologyProfile,
    /// A theme / contrast / accent / text-scale surface whose live-apply cannot be confirmed; the claim
    /// narrows to an appearance-response-unverified projection that keeps the last-known appearance posture
    /// explicit, never a theme or contrast change shown as applied live when it may not have applied or
    /// explained its fallback.
    UnverifiedAppearanceResponseProfile,
    /// A credential-store wording surface whose truthful, non-leaky posture cannot be confirmed; the claim
    /// narrows to a credential-wording-unverified projection that keeps the last-known credential-store
    /// wording explicit, never a credential-store message shown as truthful when it may hide a
    /// plaintext-storage fallback.
    UnverifiedCredentialWordingProfile,
    /// An input-method surface whose IME / dead-key / AltGr / dictation / emoji / layout text and trust
    /// fidelity cannot be confirmed; the claim narrows to an input-fidelity-unverified projection that keeps
    /// the last-known input-method state explicit, never an input flow shown as faithful when it may corrupt
    /// text or trust semantics.
    UnverifiedInputFidelityProfile,
}

impl M5PlatformFitCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5PlatformFitCertifiedProfile; 6] = [
        M5PlatformFitCertifiedProfile::LiveTrustedPlatformFitSurface,
        M5PlatformFitCertifiedProfile::ReviewablePlatformFitStructure,
        M5PlatformFitCertifiedProfile::DisclosedPathTerminologyProfile,
        M5PlatformFitCertifiedProfile::UnverifiedAppearanceResponseProfile,
        M5PlatformFitCertifiedProfile::UnverifiedCredentialWordingProfile,
        M5PlatformFitCertifiedProfile::UnverifiedInputFidelityProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrustedPlatformFitSurface => "live_trusted_platform_fit_surface",
            Self::ReviewablePlatformFitStructure => "reviewable_platform_fit_structure",
            Self::DisclosedPathTerminologyProfile => "disclosed_path_terminology_profile",
            Self::UnverifiedAppearanceResponseProfile => "unverified_appearance_response_profile",
            Self::UnverifiedCredentialWordingProfile => "unverified_credential_wording_profile",
            Self::UnverifiedInputFidelityProfile => "unverified_input_fidelity_profile",
        }
    }

    /// True only for the live, first-party trusted platform-fit surface profile. A trusted platform-fit
    /// surface may be certified on this profile alone; every other profile is at most a reviewable
    /// platform-fit structure or a narrowed projection.
    pub const fn is_live_trusted_platform_fit_surface(self) -> bool {
        matches!(self, Self::LiveTrustedPlatformFitSurface)
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and platform-fit-component-truth behavior. The CLI/export axis is always-on
/// and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformFitCertificationAxis {
    /// Visual parity: canonical host platform, shortcut notation, path / reveal verb, appearance posture,
    /// credential-store wording, input-method fidelity, and registry reference are shown on the primary
    /// surface without relying on an OS-chrome-only affordance or a mislabeled screenshot alone.
    Visual,
    /// Keyboard-reach parity: the same platform-fit truth and its bound commands are reachable and operable
    /// without a pointer, never hover-only, with stable command IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on an OS-chrome-only
    /// affordance, a mislabeled screenshot, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the
    /// shortcut notation, path / reveal verb, appearance posture, credential-store wording, or registry
    /// reference.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping
    /// the shortcut notation, path / reveal verb, or credential-store wording.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales and input methods,
    /// never mislabeling a path / reveal verb or corrupting composed text when a locale or IME is incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a partially-localized path terminology, an unconfirmed live appearance response,
    /// an unconfirmed credential-store wording, or an unconfirmed input fidelity honestly downgrades a
    /// `TrustedPlatformFitSurface` / `ReviewablePlatformFitSurface` claim rather than reading as a fresh,
    /// authoritative platform-fit surface.
    DegradedState,
    /// Platform-fit-component-truth parity: canonical host platform, shortcut notation, path / reveal verb,
    /// appearance posture, credential-store wording, input-method fidelity, and registry reference stay
    /// explicit and never let platform wording change command or permission meaning, hide a primary action
    /// only in OS chrome, silently fall back to plaintext secret storage, let an input method corrupt text or
    /// trust fidelity, or produce a screenshot or docs page that mislabels a shortcut or path / reveal verb.
    PlatformFitComponentTruth,
}

impl PlatformFitCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [PlatformFitCertificationAxis; 9] = [
        PlatformFitCertificationAxis::Visual,
        PlatformFitCertificationAxis::Keyboard,
        PlatformFitCertificationAxis::ScreenReader,
        PlatformFitCertificationAxis::HighZoomReflow,
        PlatformFitCertificationAxis::HighContrast,
        PlatformFitCertificationAxis::Localization,
        PlatformFitCertificationAxis::CliExport,
        PlatformFitCertificationAxis::DegradedState,
        PlatformFitCertificationAxis::PlatformFitComponentTruth,
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
            Self::PlatformFitComponentTruth => "platform_fit_component_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformFitAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl PlatformFitAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the author — always recomputed
/// from the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformFitProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed platform-fit tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops,
    /// a non-live profile claims a trusted platform-fit surface, or the narrowing is inconsistent.
    Red,
}

impl PlatformFitProfileClaimStatus {
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

/// The five B139 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile
/// carries all-false.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitCertGuardrails {
    /// True if the profile lets platform-specific wording change command or permission meaning. Must be
    /// false.
    pub lets_platform_wording_change_command_or_permission_meaning: bool,
    /// True if the profile hides a primary action only in OS chrome (menus / title bars). Must be false.
    pub hides_a_primary_action_only_in_os_chrome: bool,
    /// True if the profile silently falls back to plaintext secret storage. Must be false.
    pub falls_back_to_plaintext_secret_storage_silently: bool,
    /// True if the profile lets an input method corrupt text or trust fidelity. Must be false.
    pub lets_an_input_method_corrupt_text_or_trust_fidelity: bool,
    /// True if the profile produces a screenshot or docs page that mislabels a shortcut or path / reveal
    /// verb. Must be false.
    pub produces_a_screenshot_or_docs_page_that_mislabels_a_shortcut_or_path_verb: bool,
}

impl PlatformFitCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        lets_platform_wording_change_command_or_permission_meaning: false,
        hides_a_primary_action_only_in_os_chrome: false,
        falls_back_to_plaintext_secret_storage_silently: false,
        lets_an_input_method_corrupt_text_or_trust_fidelity: false,
        produces_a_screenshot_or_docs_page_that_mislabels_a_shortcut_or_path_verb: false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.lets_platform_wording_change_command_or_permission_meaning
            && !self.hides_a_primary_action_only_in_os_chrome
            && !self.falls_back_to_plaintext_secret_storage_silently
            && !self.lets_an_input_method_corrupt_text_or_trust_fidelity
            && !self.produces_a_screenshot_or_docs_page_that_mislabels_a_shortcut_or_path_verb
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The host-platform / shortcut-notation / path-verb / appearance-posture / credential-wording /
    /// input-fidelity / registry-reference fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl PlatformFitCertExportParity {
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
pub struct PlatformFitAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: PlatformFitCertificationAxis,
    /// The certification state of the axis.
    pub state: PlatformFitAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5PlatformFitDowngradeTrigger>,
}

impl PlatformFitAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible trigger (that is
    ///   exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            PlatformFitAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            PlatformFitAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            PlatformFitAxisCertificationState::UndisclosedDrift => {
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
pub struct PlatformFitClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: PlatformFitCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5PlatformFitA11yClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5PlatformFitA11yClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 desktop platform-fit profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitProfileCertificationRow {
    /// Record kind; must equal [`PLATFORM_FIT_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PLATFORM_FIT_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5PlatformFitCertifiedProfile,
    /// The platform-fit claim ceiling the profile asserts.
    pub claimed_claim: M5PlatformFitA11yClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5PlatformFitA11yClaim,
    /// The frozen platform-fit families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5PlatformFitFamily>,
    /// One outcome per [`PlatformFitCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<PlatformFitAxisOutcome>,
    /// The B139 hard invariants; all must hold.
    pub guardrails: PlatformFitCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<PlatformFitClaimAutoNarrow>,
    /// The one canonical platform-fit proof bundle this profile cites. Must equal
    /// [`PLATFORM_FIT_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: PlatformFitProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: PlatformFitCertExportParity,
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

impl PlatformFitProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: PlatformFitCertificationAxis) -> Option<&PlatformFitAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<PlatformFitCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && PlatformFitCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(PlatformFitAxisOutcome::well_formed)
    }

    /// True when the profile narrows its platform-fit claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<PlatformFitCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == PlatformFitAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a live first-party profile may
    /// certify a trusted platform-fit surface, every hard invariant must hold, CLI/export parity must always
    /// certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> PlatformFitProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != PLATFORM_FIT_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return PlatformFitProfileClaimStatus::Red;
        }

        // Every B139 hard invariant must hold.
        if !self.guardrails.all_held() {
            return PlatformFitProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return PlatformFitProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a trusted platform-fit surface.
        if self.certified_claim.asserts_trusted_surface()
            && !self.profile.is_live_trusted_platform_fit_surface()
        {
            return PlatformFitProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(PlatformFitCertificationAxis::CliExport) {
            Some(o) if o.state == PlatformFitAxisCertificationState::Certified => {}
            _ => return PlatformFitProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == PlatformFitAxisCertificationState::UndisclosedDrift)
        {
            return PlatformFitProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return PlatformFitProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return PlatformFitProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return PlatformFitProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return PlatformFitProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return PlatformFitProfileClaimStatus::Red;
        }

        PlatformFitProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == PLATFORM_FIT_CERT_ROW_RECORD_KIND
            && self.schema_version == PLATFORM_FIT_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1171 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitProfileCertificationSummary {
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

/// Constructor input for [`PlatformFitProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformFitProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<PlatformFitProfileCertificationRow>,
}

/// Checked-in M05-1171 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformFitProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<PlatformFitProfileCertificationRow>,
    pub summary: PlatformFitProfileCertificationSummary,
}

impl PlatformFitProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: PlatformFitProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: PLATFORM_FIT_CERT_SCHEMA_VERSION,
            record_kind: PLATFORM_FIT_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: PlatformFitProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5PlatformFitCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Platform-fit families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5PlatformFitFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5PlatformFitCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen platform-fit family is certified on at least one profile — proof the full matrix
    /// runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5PlatformFitFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(PlatformFitCertificationAxis::CliExport)
                .is_some_and(|o| o.state == PlatformFitAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> PlatformFitProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == PlatformFitProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == PlatformFitProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == PlatformFitProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(PlatformFitProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        PlatformFitProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == PLATFORM_FIT_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(PlatformFitProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<PlatformFitCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PLATFORM_FIT_CERT_SCHEMA_VERSION {
            violations.push(PlatformFitCertificationViolation::SchemaVersion {
                expected: PLATFORM_FIT_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != PLATFORM_FIT_CERT_RECORD_KIND {
            violations.push(PlatformFitCertificationViolation::RecordKind {
                expected: PLATFORM_FIT_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(PlatformFitCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != PLATFORM_FIT_CERT_CANONICAL_BUNDLE_REF {
            violations.push(PlatformFitCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(PlatformFitCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(PlatformFitCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(PlatformFitCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(PlatformFitCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != PLATFORM_FIT_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    PlatformFitCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B139 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(PlatformFitCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a live first-party profile may certify a trusted platform-fit surface.
            if row.certified_claim.asserts_trusted_surface()
                && !row.profile.is_live_trusted_platform_fit_surface()
            {
                violations.push(
                    PlatformFitCertificationViolation::NonLiveProfileClaimsTrustedSurface {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(PlatformFitCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    PlatformFitCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    PlatformFitCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(PlatformFitCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == PlatformFitProfileClaimStatus::Red {
                violations.push(PlatformFitCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(PlatformFitCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen platform-fit family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(PlatformFitCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(PlatformFitCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(PlatformFitCertificationViolation::RawPlatformFitMaterialInExport);
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
        out.push_str("# M5 Platform-Fit Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5PlatformFitCertifiedProfile::ALL.len(),
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
pub fn current_m5_platform_fit_surface_certification_export(
) -> Result<PlatformFitProfileCertificationPacket, PlatformFitCertificationArtifactError> {
    let packet: PlatformFitProfileCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-platform-fit-surface-certification/support_export.json"
        )))
        .map_err(PlatformFitCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PlatformFitCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum PlatformFitCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PlatformFitCertificationViolation>),
}

impl fmt::Display for PlatformFitCertificationArtifactError {
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

impl Error for PlatformFitCertificationArtifactError {}

/// Validation failure for M05-1171 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlatformFitCertificationViolation {
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
    RawPlatformFitMaterialInExport,
}

impl fmt::Display for PlatformFitCertificationViolation {
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
                    "packet does not cite the canonical platform-fit proof bundle"
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
                    "row {id} does not cite the one canonical platform-fit proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B139 hard invariant: platform-specific wording changing command or \
permission meaning; a primary action hidden only in OS chrome; secret storage silently falling back to \
plaintext; an input method corrupting text or trust fidelity; or a screenshot or docs page mislabeling a \
shortcut or path / reveal verb"
                )
            }
            Self::NonLiveProfileClaimsTrustedSurface { id } => {
                write!(
                    f,
                    "row {id} certifies a trusted platform-fit surface on a non-live first-party profile"
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
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh trusted claim, a \
hard invariant broke, CLI/export parity dropped, a non-live profile claimed a trusted platform-fit \
surface, or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 platform-fit profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen platform-fit family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawPlatformFitMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for PlatformFitCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&PlatformFitAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != PlatformFitAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the platform-fit
/// generics the spec forbids collapsing distinct platform-convention, shortcut-notation, path-terminology,
/// appearance-response, credential-wording, and input-fidelity truth into (whole-label matches so a full
/// sentence naming a concrete host platform, verb, or registry reference is not flagged).
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
            | "platform"
            | "convention"
            | "shortcut"
            | "notation"
            | "path"
            | "reveal"
            | "verb"
            | "theme"
            | "contrast"
            | "appearance"
            | "credential"
            | "wording"
            | "input"
            | "ime"
            | "dictation"
            | "layout"
            | "host"
            | "menu"
            | "window"
            | "chrome"
            | "registry reference"
            | "host platform"
            | "shortcut notation"
            | "path verb"
            | "reveal verb"
            | "more"
            | "…"
            | "..."
            | "overflow"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the platform-fit matrix
/// and M05-1170 heuristic (no `secret` token) so the reused
/// [`M5PlatformFitDowngradeTrigger::SecretStorageFellBackToPlaintextSilently`] narrowing serializes cleanly.
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

/// Builds the canonical, checked-in M05-1171 certification packet. Certifies all six claimed M5 desktop
/// platform-fit profiles: two deliver their claim (green) and four auto-narrow a not-current truth axis to a
/// weaker platform-fit ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_platform_fit_surface_certification_packet() -> PlatformFitProfileCertificationPacket
{
    PlatformFitProfileCertificationPacket::new(PlatformFitProfileCertificationPacketInput {
        packet_id: PLATFORM_FIT_CERT_PACKET_ID.to_owned(),
        as_of: "2026-07-13T00:00:00Z".to_owned(),
        matrix_ref: PLATFORM_FIT_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: PLATFORM_FIT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:platform-fit-surface-certification:{id}"),
        PLATFORM_FIT_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> PlatformFitCertExportParity {
    PlatformFitCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: PlatformFitCertificationAxis) -> &'static str {
    match axis {
        PlatformFitCertificationAxis::Visual => {
            "canonical host platform, shortcut notation, path / reveal verb, appearance posture, credential-store wording, input-method fidelity, and registry reference shown on-surface without an OS-chrome-only affordance or a mislabeled screenshot alone"
        }
        PlatformFitCertificationAxis::Keyboard => {
            "the same platform-fit role, registry reference, and bound commands are keyboard-reachable with stable command IDs, never hover-only"
        }
        PlatformFitCertificationAxis::ScreenReader => {
            "the same platform-fit truth is announced non-visually, never an OS-chrome-only / mislabeled-screenshot / unlabeled-control-only cue"
        }
        PlatformFitCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the shortcut notation, path / reveal verb, appearance posture, credential-store wording, or registry reference"
        }
        PlatformFitCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the shortcut notation, path / reveal verb, or credential-store wording"
        }
        PlatformFitCertificationAxis::Localization => {
            "the same truth stays host-correct and text-faithful across locales and input methods without mislabeling a path / reveal verb or corrupting composed text"
        }
        PlatformFitCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        PlatformFitCertificationAxis::DegradedState => {
            "a partially-localized path terminology, an unconfirmed live appearance response, an unconfirmed credential-store wording, or an unconfirmed input fidelity honestly downgrades the TrustedPlatformFitSurface/ReviewablePlatformFitSurface claim rather than reading as a fresh authoritative platform-fit surface"
        }
        PlatformFitCertificationAxis::PlatformFitComponentTruth => {
            "canonical host platform, shortcut notation, path / reveal verb, appearance posture, credential-store wording, input-method fidelity, and registry reference stay explicit and never let platform wording change command or permission meaning, hide a primary action only in OS chrome, fall back to plaintext secret storage silently, let an input method corrupt text or trust fidelity, or produce a screenshot or docs page that mislabels a shortcut or path / reveal verb"
        }
    }
}

fn seed_certified(axis: PlatformFitCertificationAxis) -> PlatformFitAxisOutcome {
    PlatformFitAxisOutcome {
        axis,
        state: PlatformFitAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: PlatformFitCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5PlatformFitDowngradeTrigger,
) -> PlatformFitAxisOutcome {
    PlatformFitAxisOutcome {
        axis,
        state: PlatformFitAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<PlatformFitAxisOutcome> {
    PlatformFitCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: PlatformFitCertificationAxis,
    outcome: PlatformFitAxisOutcome,
) -> Vec<PlatformFitAxisOutcome> {
    PlatformFitCertificationAxis::ALL
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
    profile: M5PlatformFitCertifiedProfile,
    claimed_claim: M5PlatformFitA11yClaim,
    certified_claim: M5PlatformFitA11yClaim,
    consumed_families: &[M5PlatformFitFamily],
    axis_outcomes: Vec<PlatformFitAxisOutcome>,
    claim_auto_narrow: Option<PlatformFitClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> PlatformFitProfileCertificationRow {
    let mut row = PlatformFitProfileCertificationRow {
        record_kind: PLATFORM_FIT_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: PLATFORM_FIT_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: PlatformFitCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: PLATFORM_FIT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: PlatformFitProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            PLATFORM_FIT_CERT_MATRIX_REF.to_owned(),
            PLATFORM_FIT_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-13T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: PlatformFitCertificationAxis,
    from_claim: M5PlatformFitA11yClaim,
    to_claim: M5PlatformFitA11yClaim,
    label: &str,
) -> PlatformFitClaimAutoNarrow {
    PlatformFitClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<PlatformFitProfileCertificationRow> {
    use M5PlatformFitA11yClaim::*;
    use M5PlatformFitCertifiedProfile as P;
    use M5PlatformFitDowngradeTrigger as Trig;
    use M5PlatformFitFamily::*;
    use PlatformFitCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-trusted-platform-fit-surface",
            P::LiveTrustedPlatformFitSurface,
            TrustedPlatformFitSurface,
            TrustedPlatformFitSurface,
            &[PlatformConvention],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "host_platform",
            ],
            &[
                "macOS profile: the shell keeps window controls, menu-bar behavior, and system chrome host-correct rather than hiding a primary action only in OS chrome (menus / title bars)",
                "the trusted platform-fit surface keeps stable command IDs while platform labels and shortcut notation adapt from the one shortcut-notation registry across macOS / Windows / Linux",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered platform-fit surface",
                "platform-fit-component-truth: a live first-party platform-fit surface is the only profile that certifies a trusted platform-fit surface",
            ],
        ),
        seed_row(
            "cert:reviewable-platform-fit-structure",
            P::ReviewablePlatformFitStructure,
            ReviewablePlatformFitSurface,
            ReviewablePlatformFitSurface,
            &[ShortcutNotation],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "shortcut_notation",
            ],
            &[
                "Windows / Linux profile: the shortcut notation stays bound to the single shortcut-notation registry (⌘/⌥/⌃/⇧ on macOS, Ctrl/Alt/Shift accelerators on Windows and Linux) with stable command IDs rather than a per-platform notation copied by hand",
                "the reviewable platform-fit structure keeps its menu, palette, inspector, help, and onboarding shortcut labels inspectable rather than an OS-chrome-only or mislabeled-screenshot cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable platform-fit structure",
                "platform-fit-component-truth: a reviewable platform-fit structure never certifies a live trusted, authoritative platform-fit claim",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:disclosed-path-terminology-profile",
            P::DisclosedPathTerminologyProfile,
            ReviewablePlatformFitSurface,
            PathTerminologyDisclosedProjection,
            &[FilePathReveal],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "the file / path / reveal / save terminology can only be partially localized for this locale so a fully host-correct localized verb cannot be certified",
                    "The file / path / reveal / save terminology can only be partially localized for this locale, so the ReviewablePlatformFitSurface claim narrows to a path-terminology-disclosed projection and the shell discloses the partial localization alongside the last-known host-correct verb rather than presenting a mislabeled path / reveal verb as host-correct when its localization is incomplete",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewablePlatformFitSurface,
                PathTerminologyDisclosedProjection,
                "Path terminology disclosed partial: the localized reveal / open / save verb is only partially proven for this locale so it is disclosed alongside the last-known host-correct verb and no mislabeled path verb is shown as host-correct",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "localized profile: the shell keeps the last-known host-correct reveal verb (Reveal in Finder / Show in Explorer / Show in Files) explicit and marks the localized terminology as disclosed-partial rather than presenting an incompletely-localized verb as host-correct",
                "the file-path-reveal surface keeps its host separator and reveal / open / save verbs legible while the localization is disclosed as partial",
                "localization: ReviewablePlatformFitSurface narrows to a path-terminology-disclosed projection (auto-narrowed)",
                "platform-fit-component-truth: a path / reveal verb is never mislabeled as host-correct when its localization is incomplete",
            ],
        ),
        seed_row(
            "cert:unverified-appearance-response-profile",
            P::UnverifiedAppearanceResponseProfile,
            ReviewablePlatformFitSurface,
            AppearanceResponseUnverifiedProjection,
            &[ThemeContrastLiveChange],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the theme / contrast / accent / text-scale change's live-apply cannot be confirmed so a live-applied appearance response cannot be certified",
                    "The theme / contrast / accent / text-scale change's live-apply cannot be confirmed, so the ReviewablePlatformFitSurface claim narrows to an appearance-response-unverified projection and the shell keeps the last-known appearance posture explicit rather than presenting a theme or contrast change as applied live when it may not have applied or explained its fallback",
                    Trig::ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewablePlatformFitSurface,
                AppearanceResponseUnverifiedProjection,
                "Appearance response unverified: the live-apply of the theme / contrast / accent / text-scale change cannot be confirmed so the last-known appearance posture stays explicit and no change is shown as applied live without an explained fallback",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "restart-required profile: the shell keeps its last-known appearance posture explicit and marks the live-apply as unverified rather than presenting a theme or contrast change as applied live when a restart-required fallback was not explained",
                "the theme-contrast surface keeps its light / dark / high-contrast and accent / text-scale posture legible while the live-apply is disclosed as unverified",
                "degraded-state: ReviewablePlatformFitSurface narrows to an appearance-response-unverified projection (auto-narrowed)",
                "platform-fit-component-truth: a theme or contrast change either applies live or explains its fallback",
            ],
        ),
        seed_row(
            "cert:unverified-credential-wording-profile",
            P::UnverifiedCredentialWordingProfile,
            ReviewablePlatformFitSurface,
            CredentialWordingUnverifiedProjection,
            &[CredentialStoreWording],
            seed_certified_except(
                Ax::PlatformFitComponentTruth,
                seed_narrowed(
                    Ax::PlatformFitComponentTruth,
                    "the credential-store wording's truthful, non-leaky posture cannot be confirmed so a truthful credential-store message cannot be certified",
                    "The credential-store wording's truthful, non-leaky posture cannot be confirmed, so the ReviewablePlatformFitSurface claim narrows to a credential-wording-unverified projection and the shell keeps the last-known credential-store wording explicit rather than presenting a credential-store message as truthful when it may hide a plaintext-storage fallback",
                    Trig::SecretStorageFellBackToPlaintextSilently,
                ),
            ),
            Some(seed_narrow(
                Ax::PlatformFitComponentTruth,
                ReviewablePlatformFitSurface,
                CredentialWordingUnverifiedProjection,
                "Credential wording unverified: the truthful, non-leaky posture of the credential-store copy cannot be confirmed so the last-known credential-store wording stays explicit and no message hides a plaintext-storage fallback",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "managed profile: the shell keeps its last-known credential-store wording (system secure store vs disclosed encrypted-file fallback) explicit and marks the truthfulness as unverified rather than presenting a credential-store message as truthful when it may hide a plaintext-storage fallback",
                "the credential-store surface keeps its settings-panel / auth-dialog / support-diagnostics wording legible while the truthful, non-leaky posture is disclosed as unverified",
                "platform-fit-component-truth: ReviewablePlatformFitSurface narrows to a credential-wording-unverified projection (auto-narrowed)",
                "platform-fit-component-truth: credential-store wording stays truthful and non-leaky and never hides a plaintext-storage fallback",
            ],
        ),
        seed_row(
            "cert:unverified-input-fidelity-profile",
            P::UnverifiedInputFidelityProfile,
            ReviewablePlatformFitSurface,
            InputFidelityUnverifiedProjection,
            &[InputMethod],
            seed_certified_except(
                Ax::PlatformFitComponentTruth,
                seed_narrowed(
                    Ax::PlatformFitComponentTruth,
                    "the IME / dead-key / AltGr / dictation / emoji / layout text and trust fidelity cannot be confirmed so a faithful input flow cannot be certified",
                    "The IME / dead-key / AltGr / dictation / emoji / layout text and trust fidelity cannot be confirmed, so the ReviewablePlatformFitSurface claim narrows to an input-fidelity-unverified projection and the shell keeps the last-known input-method state explicit rather than presenting an input flow as faithful when it may corrupt text or trust semantics",
                    Trig::InputMethodCorruptedTextOrTrust,
                ),
            ),
            Some(seed_narrow(
                Ax::PlatformFitComponentTruth,
                ReviewablePlatformFitSurface,
                InputFidelityUnverifiedProjection,
                "Input fidelity unverified: the text and trust fidelity of IME / dead-key / AltGr / dictation / emoji / layout input cannot be confirmed so the last-known input-method state stays explicit and no input flow is shown as faithful when it may corrupt composed text",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "IME-heavy profile: the shell keeps its last-known input-method state explicit and marks the text fidelity as unverified rather than presenting an IME / dead-key / dictation flow as faithful when composed text may be corrupted",
                "the input-method surface keeps its macOS / Windows-TSF / Linux-IBus composition, dead-key, AltGr, dictation, emoji, and layout-switch behavior legible while the text and trust fidelity is disclosed as unverified",
                "platform-fit-component-truth: ReviewablePlatformFitSurface narrows to an input-fidelity-unverified projection (auto-narrowed)",
                "platform-fit-component-truth: IME, dead keys, AltGr, dictation, emoji, and layout switching never corrupt text fidelity or trust semantics",
            ],
        ),
    ]
}
