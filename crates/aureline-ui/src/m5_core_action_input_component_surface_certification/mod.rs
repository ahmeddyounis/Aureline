//! M05-1131 surface certification over the frozen M5 button / icon-button / split-button /
//! text-field / search-field / combobox / checkbox-radio-switch / segmented-control
//! core-action-input component matrix.
//!
//! Where the freeze matrix ([`crate::m5_core_action_input_component_matrix`]) defines the eight
//! reusable button, icon-button, split-button, text-field, search-field, combobox, toggle, and
//! segmented-control families, the M05-1125..1128 implement lanes narrow each one, the M05-1129
//! shared consumer lane aligns their vocabulary, and the M05-1130 accessibility lane
//! ([`crate::m5_core_action_input_accessibility_parity_and_narrowing_when_control_truth_is_stale`])
//! proves keyboard / screen-reader / high-zoom / reduced-motion / CLI-export parity and per-family
//! auto-narrowing, this closing capstone *certifies* that the shared control truth holds on every
//! claimed M5 forms / settings / search / entry / review / repair operating profile — and
//! auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a user, reviewer, or support engineer operates a reusable
//! action / input control through (a live, first-party trusted control surface; a reviewable
//! control structure; an unbound-command surface; an unlabeled-icon surface; a riskier-split-default
//! surface; a stale-validation field; an unverified-toggle control; and a partial-retention search
//! field), not on component family or implement lane. Each
//! [`CoreControlProfileCertificationRow`] certifies one profile across eight truth axes — visual,
//! keyboard, screen-reader, high-zoom-reflow, reduced-motion, CLI/export, degraded-state, and
//! control-component-truth behavior — and either passes (green), auto-narrows its control claim to
//! the weakest supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind
//! a fresh trusted claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that
//! keeps a `TrustedControl` / `ReviewableControl` claim while one of its truth axes is not current
//! is over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with
//! a bound reason and a frozen downgrade trigger) is honestly yellow. Only a live, first-party
//! trusted control profile may certify a `TrustedControl` claim — a reviewable, unbound, unlabeled,
//! riskier-default, stale, unverified, or partial-retention profile that keeps a trusted claim is
//! over-reaching and blocks. The always-on CLI/export axis must always stay certified so support and
//! automation can reconstruct the interaction state, command binding, accessible name, default
//! safety, validation, value source, toggle semantics, and retention posture from the same
//! component identity the user saw.
//!
//! The B134 guardrails are enforced per row: no profile may use placeholder text as the only label,
//! let a loading control relabel or resize its action out of attribution, leave an icon-only
//! destructive action unlabeled, blur a switch with a deferred checkbox, let a split button default
//! to a riskier alternate, or hide locked / degraded semantics behind generic disabled chrome. A
//! profile that breaches any guardrail blocks (red).
//!
//! Every row cites exactly one canonical core-action-input proof bundle
//! ([`CORE_ACTION_INPUT_CERT_CANONICAL_BUNDLE_REF`]) — the frozen core-action-input component matrix
//! proof — rather than cloning per-profile evidence. The packet is metadata-only: raw field values,
//! option payloads, credentials, secrets, and endpoint refs never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-core-action-input-component-surface-certification.schema.json`](../../../../schemas/ui/m5-core-action-input-component-surface-certification.schema.json).
//! The contract doc is
//! [`docs/components/m5_core_action_input_component_surface_certification_contract.md`](../../../../docs/components/m5_core_action_input_component_surface_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_core_action_input_accessibility_parity_and_narrowing_when_control_truth_is_stale as a11y;
use crate::m5_core_action_input_component_matrix as matrix;
use a11y::M5CoreControlClaim;
use matrix::{M5CoreControlDowngradeTrigger, M5CoreControlFamily};

/// Schema version stamped on the M05-1131 certification packet.
pub const CORE_ACTION_INPUT_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`CoreControlProfileCertificationPacket`].
pub const CORE_ACTION_INPUT_CERT_RECORD_KIND: &str =
    "m5_core_action_input_component_surface_certification_packet";

/// Stable record-kind tag carried by each [`CoreControlProfileCertificationRow`].
pub const CORE_ACTION_INPUT_CERT_ROW_RECORD_KIND: &str =
    "m5_core_action_input_component_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const CORE_ACTION_INPUT_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-core-action-input-component-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const CORE_ACTION_INPUT_CERT_DOC_REF: &str =
    "docs/components/m5_core_action_input_component_surface_certification_contract.md";

/// Repo-relative path of the frozen core-action-input component matrix schema the certified
/// profiles render.
pub const CORE_ACTION_INPUT_CERT_MATRIX_REF: &str = matrix::M5_CORE_CONTROL_COMPONENT_SCHEMA_REF;

/// The one canonical core-action-input proof bundle every certified profile cites as its
/// first-resolved component truth. All eight profiles point back to it rather than cloning
/// per-profile evidence.
pub const CORE_ACTION_INPUT_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_CORE_CONTROL_COMPONENT_ARTIFACT_REF;

/// The M05-1130 accessibility support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const CORE_ACTION_INPUT_CERT_A11Y_BUNDLE_REF: &str = a11y::CORE_ACTION_INPUT_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const CORE_ACTION_INPUT_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-core-action-input-component-surface-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const CORE_ACTION_INPUT_CERT_CSV_REF: &str =
    "artifacts/release/m5-core-action-input-component-surface-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const CORE_ACTION_INPUT_CERT_REPORT_REF: &str =
    "artifacts/release/m5-core-action-input-component-surface-certification-proof/report.md";

/// Stable packet id for the checked-in certification bundle.
pub const CORE_ACTION_INPUT_CERT_PACKET_ID: &str =
    "m5-core-action-input-component-surface-certification:stable:0001";

/// The eight claimed M5 forms / settings / search / entry / review / repair operating profiles this
/// capstone certifies. Keyed on the profile a user, reviewer, or support engineer operates a
/// control through, not on the reusable component family it renders. Only a live, first-party
/// trusted control profile may certify a trusted control.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CoreControlCertifiedProfile {
    /// A live, first-party, fully-current control surface — a button and segmented control naming
    /// the trusted, command-bound, safe-by-default, accessibly-named control state exactly right
    /// now.
    LiveTrustedControlSurface,
    /// A reviewable control structure: a self-sufficient, inspectable combobox a user can review,
    /// never itself an authoritative mutation-ready control surface.
    ReviewableControlStructure,
    /// A button surface whose command binding is stale / missing; the claim narrows to a
    /// command-binding-unverified projection with last-known identity preserved.
    UnboundCommandSurface,
    /// An icon-only surface with no confirmed accessible name; the claim narrows to an
    /// accessible-name-unverified projection naming the last-known glyph / action.
    UnlabeledIconSurface,
    /// A split-button surface whose safe default cannot be confirmed; the claim narrows to a
    /// default-safety-unverified projection that keeps the safe default explicit, never letting a
    /// riskier alternate become the default.
    RiskierSplitDefaultSurface,
    /// A text field whose validation anchor is stale; the claim narrows to a validation-unverified
    /// projection disclosing the last-known validation state.
    StaleValidationField,
    /// A toggle control whose immediate-versus-deferred semantic is unverified; the claim narrows to
    /// a toggle-semantics-unverified projection keeping the last-known toggle semantics.
    UnverifiedToggleControl,
    /// A search field that can only disclose a partial / redacted retention posture; the claim
    /// narrows to a retention-disclosed projection disclosing the partial retention posture.
    PartialRetentionSearchField,
}

impl M5CoreControlCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5CoreControlCertifiedProfile; 8] = [
        M5CoreControlCertifiedProfile::LiveTrustedControlSurface,
        M5CoreControlCertifiedProfile::ReviewableControlStructure,
        M5CoreControlCertifiedProfile::UnboundCommandSurface,
        M5CoreControlCertifiedProfile::UnlabeledIconSurface,
        M5CoreControlCertifiedProfile::RiskierSplitDefaultSurface,
        M5CoreControlCertifiedProfile::StaleValidationField,
        M5CoreControlCertifiedProfile::UnverifiedToggleControl,
        M5CoreControlCertifiedProfile::PartialRetentionSearchField,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrustedControlSurface => "live_trusted_control_surface",
            Self::ReviewableControlStructure => "reviewable_control_structure",
            Self::UnboundCommandSurface => "unbound_command_surface",
            Self::UnlabeledIconSurface => "unlabeled_icon_surface",
            Self::RiskierSplitDefaultSurface => "riskier_split_default_surface",
            Self::StaleValidationField => "stale_validation_field",
            Self::UnverifiedToggleControl => "unverified_toggle_control",
            Self::PartialRetentionSearchField => "partial_retention_search_field",
        }
    }

    /// True only for the live, first-party trusted control surface profile. A trusted control may be
    /// certified on this profile alone; every other profile is at most a reviewable control or a
    /// narrowed projection.
    pub const fn is_live_trusted_control(self) -> bool {
        matches!(self, Self::LiveTrustedControlSurface)
    }
}

/// The eight truth axes a certified profile is scored on. These are exactly the parity dimensions
/// the spec requires verifying — visual, keyboard, screen-reader, high-zoom reflow, reduced-motion,
/// CLI/export, degraded-state, and control-component-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreControlCertificationAxis {
    /// Visual parity: interaction state, command binding, accessible name, default safety,
    /// validation, value source, toggle semantics, and selected mode are shown on the primary
    /// surface without relying on color alone.
    Visual,
    /// Keyboard-reach parity: the same control truth and its bounded local actions are reachable and
    /// operable without a pointer, never hover-only.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color,
    /// motion, or a chrome glyph alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at high zoom rather than clipping the
    /// state, label, command, or validation copy.
    HighZoomReflow,
    /// Reduced-motion parity: the same truth is legible and usable with reduced motion, never
    /// motion-only.
    ReducedMotion,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale command binding, missing accessible name, unconfirmed default
    /// safety, stale validation anchor, unverified toggle semantic, or partial retention posture
    /// honestly downgrades a `TrustedControl` / `ReviewableControl` claim rather than reading as a
    /// fresh, authoritative control.
    DegradedState,
    /// Control-component-truth parity: interaction state, command binding, accessible name, default
    /// safety, validation, value source, toggle semantics, selected mode, and the locked / read-only
    /// / degraded distinction stay explicit and never collapse into generic disabled chrome, encode
    /// state by color alone, use placeholder text as the only label, let a loading control relabel
    /// its action, leave an icon-only destructive action unlabeled, blur a switch with a deferred
    /// checkbox, or let a split button default to a riskier alternate.
    ControlComponentTruth,
}

impl CoreControlCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [CoreControlCertificationAxis; 8] = [
        CoreControlCertificationAxis::Visual,
        CoreControlCertificationAxis::Keyboard,
        CoreControlCertificationAxis::ScreenReader,
        CoreControlCertificationAxis::HighZoomReflow,
        CoreControlCertificationAxis::ReducedMotion,
        CoreControlCertificationAxis::CliExport,
        CoreControlCertificationAxis::DegradedState,
        CoreControlCertificationAxis::ControlComponentTruth,
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
            Self::ReducedMotion => "reduced_motion",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::ControlComponentTruth => "control_component_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreControlAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a
    /// healthier profile.
    UndisclosedDrift,
}

impl CoreControlAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the author — always
/// recomputed from the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoreControlProfileClaimStatus {
    /// Full standing: every axis certified, every guardrail held, claimed control tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a guardrail breaks, CLI/export parity
    /// drops, a non-live profile claims a trusted control, or the narrowing is inconsistent.
    Red,
}

impl CoreControlProfileClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the profile is publishable as certified (green or disclosed yellow); red profiles
    /// block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The six B134 guardrails carried on every certified profile. All six must hold — a breach blocks
/// the profile (red). Each field is `true` only when the profile *breaks* the guardrail, so a clean
/// profile carries all-false.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlCertGuardrails {
    /// True if the profile uses placeholder text as the only label. Must be false.
    pub uses_placeholder_as_label: bool,
    /// True if the profile lets a loading control relabel its action or resize enough to lose
    /// attribution. Must be false.
    pub lets_loading_relabel_or_resize_action: bool,
    /// True if the profile leaves an icon-only destructive action unlabeled. Must be false.
    pub leaves_icon_only_destructive_unlabeled: bool,
    /// True if the profile blurs a switch with a deferred checkbox. Must be false.
    pub blurs_switch_with_deferred_checkbox: bool,
    /// True if the profile lets a split button default to a riskier alternate. Must be false.
    pub lets_split_default_to_riskier_alternate: bool,
    /// True if the profile hides locked / degraded semantics behind generic disabled chrome. Must be
    /// false.
    pub hides_locked_or_degraded_behind_disabled: bool,
}

impl CoreControlCertGuardrails {
    /// A clean profile: every guardrail held.
    pub const CLEAN: Self = Self {
        uses_placeholder_as_label: false,
        lets_loading_relabel_or_resize_action: false,
        leaves_icon_only_destructive_unlabeled: false,
        blurs_switch_with_deferred_checkbox: false,
        lets_split_default_to_riskier_alternate: false,
        hides_locked_or_degraded_behind_disabled: false,
    };

    /// True when every guardrail holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.uses_placeholder_as_label
            && !self.lets_loading_relabel_or_resize_action
            && !self.leaves_icon_only_destructive_unlabeled
            && !self.blurs_switch_with_deferred_checkbox
            && !self.lets_split_default_to_riskier_alternate
            && !self.hides_locked_or_degraded_behind_disabled
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when
/// this offers text / JSON / Markdown reconstruction and prohibits a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The interaction-state / command-binding / accessible-name / default-safety / validation /
    /// value-source / toggle-semantics / retention fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl CoreControlCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a screenshot-only export.
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
pub struct CoreControlAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: CoreControlCertificationAxis,
    /// The certification state of the axis.
    pub state: CoreControlAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5CoreControlDowngradeTrigger>,
}

impl CoreControlAxisOutcome {
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
            CoreControlAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            CoreControlAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            CoreControlAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current. Present iff the
/// certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: CoreControlCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5CoreControlClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5CoreControlClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 forms / settings / search / entry / review / repair control profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlProfileCertificationRow {
    /// Record kind; must equal [`CORE_ACTION_INPUT_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CORE_ACTION_INPUT_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5CoreControlCertifiedProfile,
    /// The control claim ceiling the profile asserts.
    pub claimed_claim: M5CoreControlClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5CoreControlClaim,
    /// The frozen component families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5CoreControlFamily>,
    /// One outcome per [`CoreControlCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<CoreControlAxisOutcome>,
    /// The B134 guardrails; all must hold.
    pub guardrails: CoreControlCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<CoreControlClaimAutoNarrow>,
    /// The one canonical core-action-input proof bundle this profile cites. Must equal
    /// [`CORE_ACTION_INPUT_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: CoreControlProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: CoreControlCertExportParity,
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

impl CoreControlProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: CoreControlCertificationAxis) -> Option<&CoreControlAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<CoreControlCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && CoreControlCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(CoreControlAxisOutcome::well_formed)
    }

    /// True when the profile narrows its control claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<CoreControlCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == CoreControlAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, guardrails, and claim narrowing. This is the heart
    /// of the capstone: a degraded axis must produce a visible claim narrowing, only a live
    /// first-party profile may certify a trusted control, every guardrail must hold, CLI/export
    /// parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> CoreControlProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != CORE_ACTION_INPUT_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return CoreControlProfileClaimStatus::Red;
        }

        // Every B134 guardrail must hold.
        if !self.guardrails.all_held() {
            return CoreControlProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return CoreControlProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a trusted control.
        if self.certified_claim.asserts_trusted_control() && !self.profile.is_live_trusted_control()
        {
            return CoreControlProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(CoreControlCertificationAxis::CliExport) {
            Some(o) if o.state == CoreControlAxisCertificationState::Certified => {}
            _ => return CoreControlProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == CoreControlAxisCertificationState::UndisclosedDrift)
        {
            return CoreControlProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return CoreControlProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return CoreControlProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return CoreControlProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return CoreControlProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier profile's truth.
        if !narrowed.is_empty() {
            return CoreControlProfileClaimStatus::Red;
        }

        CoreControlProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == CORE_ACTION_INPUT_CERT_ROW_RECORD_KIND
            && self.schema_version == CORE_ACTION_INPUT_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1131 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlProfileCertificationSummary {
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

/// Constructor input for [`CoreControlProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CoreControlProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<CoreControlProfileCertificationRow>,
}

/// Checked-in M05-1131 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoreControlProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<CoreControlProfileCertificationRow>,
    pub summary: CoreControlProfileCertificationSummary,
}

impl CoreControlProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: CoreControlProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: CORE_ACTION_INPUT_CERT_SCHEMA_VERSION,
            record_kind: CORE_ACTION_INPUT_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: CoreControlProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5CoreControlCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Component families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5CoreControlFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5CoreControlCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen component family is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5CoreControlFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(CoreControlCertificationAxis::CliExport)
                .is_some_and(|o| o.state == CoreControlAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> CoreControlProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == CoreControlProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == CoreControlProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == CoreControlProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(CoreControlProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        CoreControlProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == CORE_ACTION_INPUT_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(CoreControlProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<CoreControlCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != CORE_ACTION_INPUT_CERT_SCHEMA_VERSION {
            violations.push(CoreControlCertificationViolation::SchemaVersion {
                expected: CORE_ACTION_INPUT_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != CORE_ACTION_INPUT_CERT_RECORD_KIND {
            violations.push(CoreControlCertificationViolation::RecordKind {
                expected: CORE_ACTION_INPUT_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(CoreControlCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != CORE_ACTION_INPUT_CERT_CANONICAL_BUNDLE_REF {
            violations.push(CoreControlCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(CoreControlCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(CoreControlCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(CoreControlCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(CoreControlCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != CORE_ACTION_INPUT_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    CoreControlCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B134 guardrail must hold.
            if !row.guardrails.all_held() {
                violations.push(CoreControlCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a live first-party profile may certify a trusted control.
            if row.certified_claim.asserts_trusted_control()
                && !row.profile.is_live_trusted_control()
            {
                violations.push(
                    CoreControlCertificationViolation::NonLiveProfileClaimsTrustedControl {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(CoreControlCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    CoreControlCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    CoreControlCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(CoreControlCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == CoreControlProfileClaimStatus::Red {
                violations.push(CoreControlCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(CoreControlCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen component family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(CoreControlCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(CoreControlCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(CoreControlCertificationViolation::RawControlMaterialInExport);
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
        out.push_str("# M5 Core-Action-Input Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5CoreControlCertifiedProfile::ALL.len(),
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
pub fn current_m5_core_action_input_component_surface_certification_export(
) -> Result<CoreControlProfileCertificationPacket, CoreControlCertificationArtifactError> {
    let packet: CoreControlProfileCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-core-action-input-component-surface-certification-proof/support_export.json"
    )))
    .map_err(CoreControlCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CoreControlCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum CoreControlCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CoreControlCertificationViolation>),
}

impl fmt::Display for CoreControlCertificationArtifactError {
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

impl Error for CoreControlCertificationArtifactError {}

/// Validation failure for M05-1131 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreControlCertificationViolation {
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
    NonLiveProfileClaimsTrustedControl { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawControlMaterialInExport,
}

impl fmt::Display for CoreControlCertificationViolation {
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
                    "packet does not cite the canonical core-action-input proof bundle"
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
                    "row {id} does not cite the one canonical core-action-input proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B134 guardrail: placeholder-as-label, a loading control that \
relabels or resizes its action, an unlabeled icon-only destructive action, a switch blurred with a \
deferred checkbox, a split button defaulting to a riskier alternate, or locked / degraded semantics \
hidden behind generic disabled chrome"
                )
            }
            Self::NonLiveProfileClaimsTrustedControl { id } => {
                write!(
                    f,
                    "row {id} certifies a trusted control on a non-live first-party profile"
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
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh trusted \
claim, a guardrail broke, CLI/export parity dropped, a non-live profile claimed a trusted control, \
or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 core-action-input profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen core-action-input component family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawControlMaterialInExport => {
                write!(
                    f,
                    "export contains a raw field value, option payload, credential, or secret material"
                )
            }
        }
    }
}

impl Error for CoreControlCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&CoreControlAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != CoreControlAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the control /
/// input generics the spec forbids collapsing distinct interaction-state, command, value-source,
/// validation, and toggle-semantics truth into (whole-label matches so a full sentence naming a
/// concrete state, command, or validation posture is not flagged).
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
            | "disabled"
            | "locked"
            | "read only"
            | "read-only"
            | "partial"
            | "cached"
            | "trusted"
            | "reviewable"
            | "button"
            | "icon button"
            | "split button"
            | "text field"
            | "search field"
            | "combobox"
            | "toggle"
            | "checkbox"
            | "radio"
            | "switch"
            | "segmented control"
            | "command"
            | "command binding"
            | "accessible name"
            | "default safety"
            | "validation"
            | "value source"
            | "toggle semantics"
            | "retention"
            | "more"
            | "…"
            | "..."
            | "overflow"
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

/// Builds the canonical, checked-in M05-1131 certification packet. Certifies all eight claimed M5
/// forms / settings / search / entry / review / repair profiles: two deliver their claim (green)
/// and six auto-narrow a not-current truth axis to a weaker control ceiling (yellow). No profile
/// hides drift or breaks a guardrail (red).
pub fn seeded_m5_core_action_input_component_surface_certification_packet(
) -> CoreControlProfileCertificationPacket {
    CoreControlProfileCertificationPacket::new(CoreControlProfileCertificationPacketInput {
        packet_id: CORE_ACTION_INPUT_CERT_PACKET_ID.to_owned(),
        as_of: "2026-07-12T00:00:00Z".to_owned(),
        matrix_ref: CORE_ACTION_INPUT_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: CORE_ACTION_INPUT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:core-action-input-component-certification:{id}"),
        CORE_ACTION_INPUT_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> CoreControlCertExportParity {
    CoreControlCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: CoreControlCertificationAxis) -> &'static str {
    match axis {
        CoreControlCertificationAxis::Visual => {
            "interaction state, command binding, accessible name, default safety, validation, value source, toggle semantics, and selected mode shown on-surface without color alone"
        }
        CoreControlCertificationAxis::Keyboard => {
            "the same control state, command, label, validation, value source, toggle semantics, and bounded local actions are keyboard-reachable, never hover-only"
        }
        CoreControlCertificationAxis::ScreenReader => {
            "the same control / input truth is announced non-visually, never color/motion/glyph-only"
        }
        CoreControlCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at high zoom without clipping the state, label, command, or validation copy"
        }
        CoreControlCertificationAxis::ReducedMotion => {
            "the same truth stays legible and usable with reduced motion, never motion-only"
        }
        CoreControlCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        CoreControlCertificationAxis::DegradedState => {
            "a stale command binding, missing accessible name, unconfirmed default safety, stale validation anchor, unverified toggle semantic, or partial retention posture honestly downgrades the TrustedControl/ReviewableControl claim rather than reading as a fresh authoritative control"
        }
        CoreControlCertificationAxis::ControlComponentTruth => {
            "interaction state, command binding, accessible name, default safety, validation, value source, toggle semantics, selected mode, and the locked / read-only / degraded distinction stay explicit and never collapse into generic disabled chrome, encode state by color alone, use placeholder text as the only label, let a loading control relabel its action, leave an icon-only destructive action unlabeled, blur a switch with a deferred checkbox, or let a split button default to a riskier alternate"
        }
    }
}

fn seed_certified(axis: CoreControlCertificationAxis) -> CoreControlAxisOutcome {
    CoreControlAxisOutcome {
        axis,
        state: CoreControlAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: CoreControlCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5CoreControlDowngradeTrigger,
) -> CoreControlAxisOutcome {
    CoreControlAxisOutcome {
        axis,
        state: CoreControlAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<CoreControlAxisOutcome> {
    CoreControlCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: CoreControlCertificationAxis,
    outcome: CoreControlAxisOutcome,
) -> Vec<CoreControlAxisOutcome> {
    CoreControlCertificationAxis::ALL
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
    profile: M5CoreControlCertifiedProfile,
    claimed_claim: M5CoreControlClaim,
    certified_claim: M5CoreControlClaim,
    consumed_families: &[M5CoreControlFamily],
    axis_outcomes: Vec<CoreControlAxisOutcome>,
    claim_auto_narrow: Option<CoreControlClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> CoreControlProfileCertificationRow {
    let mut row = CoreControlProfileCertificationRow {
        record_kind: CORE_ACTION_INPUT_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: CORE_ACTION_INPUT_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: CoreControlCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: CORE_ACTION_INPUT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: CoreControlProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            CORE_ACTION_INPUT_CERT_MATRIX_REF.to_owned(),
            CORE_ACTION_INPUT_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-12T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: CoreControlCertificationAxis,
    from_claim: M5CoreControlClaim,
    to_claim: M5CoreControlClaim,
    label: &str,
) -> CoreControlClaimAutoNarrow {
    CoreControlClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<CoreControlProfileCertificationRow> {
    use CoreControlCertificationAxis as Ax;
    use M5CoreControlCertifiedProfile as P;
    use M5CoreControlClaim::*;
    use M5CoreControlDowngradeTrigger as Trig;
    use M5CoreControlFamily::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-trusted-control-surface",
            P::LiveTrustedControlSurface,
            TrustedControl,
            TrustedControl,
            &[Button, SegmentedControl],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "interaction_state"],
            &[
                "button names its permanent label, stable emphasis, and command binding without relying on color alone, and its loading state never relabels or resizes the action out of attribution",
                "segmented control stays a small mode / view toggle naming the selected mode, never stealth navigation",
                "keyboard / screen-reader / high-zoom / reduced-motion reach preserved for the button and the segmented control",
                "control-component-truth: a live first-party control surface is the only profile that certifies a trusted control",
            ],
        ),
        seed_row(
            "cert:reviewable-control-structure",
            P::ReviewableControlStructure,
            ReviewableControl,
            ReviewableControl,
            &[Combobox],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "value_source"],
            &[
                "combobox preserves its filterability and source-of-value truth and names its selected value without collapsing the option provenance",
                "the combobox permanent label and validation state stay legible rather than placeholder-only",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable structure",
                "control-component-truth: a reviewable read-only combobox never certifies a live trusted mutation-ready claim",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:unbound-command-surface",
            P::UnboundCommandSurface,
            ReviewableControl,
            CommandBindingUnverifiedProjection,
            &[Button],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the button's command binding is stale / missing so a freshly-bound, ready-to-invoke action cannot be certified",
                    "The button's command binding is stale / missing, so the ReviewableControl claim narrows to a command-binding-unverified projection and the control preserves its last-known label / action rather than presenting an unbound button as a freshly-bound, ready-to-invoke action",
                    Trig::CommandBindingUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableControl,
                CommandBindingUnverifiedProjection,
                "Command binding unverified: the binding is stale so the last-known label / action is preserved and the button never reads as freshly bound",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "button preserves its last-known label and marks the command binding as unverified rather than presenting an unbound action as ready to invoke",
                "the button keeps its loading behavior honest — no relabel, no width loss — while the command binding is disclosed as unverified",
                "degraded-state: ReviewableControl narrows to a command-binding-unverified projection (auto-narrowed)",
                "control-component-truth: an unbound button never silently reads as a freshly-bound, ready-to-invoke action",
            ],
        ),
        seed_row(
            "cert:unlabeled-icon-surface",
            P::UnlabeledIconSurface,
            ReviewableControl,
            AccessibleNameUnverifiedProjection,
            &[IconButton],
            seed_certified_except(
                Ax::ControlComponentTruth,
                seed_narrowed(
                    Ax::ControlComponentTruth,
                    "the icon-only control has no confirmed accessible name so an accessibly-named, safe-to-operate icon action cannot be certified",
                    "The icon-only control has no confirmed accessible name, so the ReviewableControl claim narrows to an accessible-name-unverified projection and the control preserves its last-known glyph / action rather than presenting an unlabeled destructive action as a safe, accessibly-named control",
                    Trig::IconOnlyDestructiveUnlabeled,
                ),
            ),
            Some(seed_narrow(
                Ax::ControlComponentTruth,
                ReviewableControl,
                AccessibleNameUnverifiedProjection,
                "Accessible name unverified: the icon action has no confirmed name so the last-known glyph / action is preserved and it never reads as a safe, named control",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "icon-only control keeps its last-known glyph / action visible and marks the accessible name as unverified rather than showing an unlabeled destructive action as safe",
                "the control keeps its emphasis and locked / disabled distinction while the accessible name is disclosed as unverified",
                "control-component-truth: ReviewableControl narrows to an accessible-name-unverified projection (auto-narrowed)",
                "control-component-truth: an icon-only destructive action is never left unlabeled and shown as safe",
            ],
        ),
        seed_row(
            "cert:riskier-split-default-surface",
            P::RiskierSplitDefaultSurface,
            ReviewableControl,
            DefaultSafetyUnverifiedProjection,
            &[SplitButton],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the split button's safe default cannot be confirmed so a safe-by-default split action cannot be certified",
                    "The split button's safe default cannot be confirmed, so the ReviewableControl claim narrows to a default-safety-unverified projection and the control keeps the safe default explicit rather than letting a riskier alternate quietly become the default action",
                    Trig::SplitDefaultedToRiskierAlternate,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableControl,
                DefaultSafetyUnverifiedProjection,
                "Default safety unverified: the safe default cannot be confirmed so the safe default stays explicit and a riskier alternate never becomes the default",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "split button keeps its safe default explicit and marks the default safety as unverified rather than promoting a riskier alternate to the default action",
                "the split menu keeps its alternate actions visible and attributed while the default safety is disclosed as unverified",
                "degraded-state: ReviewableControl narrows to a default-safety-unverified projection (auto-narrowed)",
                "control-component-truth: a riskier split alternate never quietly becomes the default",
            ],
        ),
        seed_row(
            "cert:stale-validation-field",
            P::StaleValidationField,
            ReviewableControl,
            ValidationUnverifiedProjection,
            &[TextField],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the text field's validation anchor is stale so a freshly-validated field cannot be certified",
                    "The text field's validation anchor is stale, so the ReviewableControl claim narrows to a validation-unverified projection and the field discloses its last-known validation state rather than presenting a stale field as freshly validated",
                    Trig::ValidationStateUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableControl,
                ValidationUnverifiedProjection,
                "Validation unverified: the validation anchor is stale so the last-known validation state is disclosed and the field never reads as freshly validated",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "text field keeps its permanent label and discloses its last-known validation state, marking the validation as unverified rather than presenting a stale field as freshly validated",
                "the field keeps its label permanent — never placeholder-only — while the validation anchor is disclosed as stale",
                "degraded-state: ReviewableControl narrows to a validation-unverified projection (auto-narrowed)",
                "control-component-truth: placeholder text never replaces the permanent label, and a stale validation never reads as fresh",
            ],
        ),
        seed_row(
            "cert:unverified-toggle-control",
            P::UnverifiedToggleControl,
            ReviewableControl,
            ToggleSemanticsUnverifiedProjection,
            &[ToggleControl],
            seed_certified_except(
                Ax::ControlComponentTruth,
                seed_narrowed(
                    Ax::ControlComponentTruth,
                    "the immediate-versus-deferred toggle semantic is unverified so a semantics-clear switch / checkbox cannot be certified",
                    "The immediate-versus-deferred toggle semantic is unverified, so the ReviewableControl claim narrows to a toggle-semantics-unverified projection and the control keeps its last-known toggle semantics rather than blurring a switch with a deferred checkbox",
                    Trig::SwitchAndDeferredCheckboxBlurred,
                ),
            ),
            Some(seed_narrow(
                Ax::ControlComponentTruth,
                ReviewableControl,
                ToggleSemanticsUnverifiedProjection,
                "Toggle semantics unverified: the immediate-versus-deferred behavior is unverified so the last-known toggle semantics are kept and a switch is never blurred with a deferred checkbox",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "toggle control keeps its last-known switch / checkbox / radio semantics and marks the immediate-versus-deferred behavior as unverified rather than blurring a switch with a deferred checkbox",
                "the control keeps its distinct on / off / mixed state legible while the toggle semantics are disclosed as unverified",
                "control-component-truth: ReviewableControl narrows to a toggle-semantics-unverified projection (auto-narrowed)",
                "control-component-truth: a switch is never blurred with a deferred checkbox",
            ],
        ),
        seed_row(
            "cert:partial-retention-search-field",
            P::PartialRetentionSearchField,
            ReviewableControl,
            RetentionDisclosedProjection,
            &[SearchField],
            seed_certified_except(
                Ax::ControlComponentTruth,
                seed_narrowed(
                    Ax::ControlComponentTruth,
                    "the search field can only disclose a partial / redacted retention posture so a fully-private, no-retention field cannot be certified",
                    "The search field can only disclose a partial / redacted retention / privacy posture, so the ReviewableControl claim narrows to a retention-disclosed projection and the field discloses the partial retention posture inspectably rather than presenting itself as a fully-private, no-retention field",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::ControlComponentTruth,
                ReviewableControl,
                RetentionDisclosedProjection,
                "Retention disclosed partial: the retention / privacy posture is partial so the field discloses it inspectably and never reads as fully private with no retention",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "search field discloses its partial / redacted retention posture and preserves its clear / submit / validation truth rather than claiming to be a fully-private, no-retention field",
                "the field keeps its clear and submit affordances and permanent label while the retention posture is disclosed as partial",
                "control-component-truth: ReviewableControl narrows to a retention-disclosed projection (auto-narrowed)",
                "control-component-truth: a partial retention posture is disclosed honestly, never presented as fully private",
            ],
        ),
    ]
}
