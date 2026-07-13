//! M05-1139 surface certification over the frozen M5 badge-chip-pill / popover / dialog-sheet /
//! banner-inline-notice / toast / empty-state / loading-state / consequence-block decision-feedback
//! component matrix.
//!
//! Where the freeze matrix ([`crate::m5_decision_feedback_component_matrix`]) defines the eight
//! reusable badge-chip-pill, popover, dialog-sheet, banner-inline-notice, toast, empty-state,
//! loading-state, and consequence-block families, the M05-1133..1136 implement lanes narrow each
//! one, the M05-1137 shared consumer lane aligns their vocabulary, and the M05-1138 accessibility
//! lane
//! ([`crate::m5_decision_feedback_accessibility_parity_and_narrowing_when_decision_feedback_truth_is_stale`])
//! proves keyboard / screen-reader / high-zoom / reduced-motion / CLI-export parity and per-family
//! auto-narrowing, this closing capstone *certifies* that the shared decision-feedback truth holds
//! on every claimed M5 shell / entry / trust / review / repair / notification operating profile —
//! and auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a user, reviewer, or support engineer reads, dismisses,
//! reopens, or exports a reusable decision / feedback primitive through (a live, first-party trusted
//! decision surface; a reviewable decision structure; a stale-severity badge surface; an
//! unscoped-notice surface; an unanchored-popover surface; a toast-only durable surface; a
//! spinner-loading surface; and a partial-recovery consequence surface), not on component family or
//! implement lane. Each [`DecisionFeedbackProfileCertificationRow`] certifies one profile across
//! eight truth axes — visual, keyboard, screen-reader, high-zoom-reflow, reduced-motion, CLI/export,
//! degraded-state, and decision-feedback-component-truth behavior — and either passes (green),
//! auto-narrows its decision claim to the weakest supported ceiling (yellow), or is blocked (red)
//! when a degraded axis is hidden behind a fresh trusted claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that
//! keeps a `TrustedDecisionSurface` / `ReviewableDecisionSurface` claim while one of its truth axes
//! is not current is over-claiming and blocks; a profile that discloses the reduction by narrowing
//! its claim (with a bound reason and a frozen downgrade trigger) is honestly yellow. Only a live,
//! first-party trusted decision profile may certify a `TrustedDecisionSurface` claim — a reviewable,
//! stale-severity, unscoped, unanchored, toast-only, spinner, or partial-recovery profile that keeps
//! a trusted claim is over-reaching and blocks. The always-on CLI/export axis must always stay
//! certified so support and automation can reconstruct the disposition, severity meaning, notice
//! scope, focus-return anchor, durable-object linkage, partial-capability fidelity, and
//! recovery/rollback posture from the same component identity the user saw.
//!
//! The B135 guardrails are enforced per row: no profile may rely on color alone for badge / banner /
//! inline-notice meaning, let a popover carry the only critical workflow instruction, use generic
//! Yes/No copy in a high-risk dialog, represent long-running or reviewable work as toast-only truth,
//! blank a useful pane during loading, or use a full-screen spinner where partial capability exists.
//! A profile that breaches any guardrail blocks (red).
//!
//! Every row cites exactly one canonical decision-feedback proof bundle
//! ([`DECISION_FEEDBACK_CERT_CANONICAL_BUNDLE_REF`]) — the frozen decision-feedback component matrix
//! proof — rather than cloning per-profile evidence. The packet is metadata-only: raw field values,
//! copy payloads, credentials, secrets, and endpoint refs never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-decision-feedback-component-surface-certification.schema.json`](../../../../schemas/ui/m5-decision-feedback-component-surface-certification.schema.json).
//! The contract doc is
//! [`docs/components/m5_decision_feedback_component_surface_certification_contract.md`](../../../../docs/components/m5_decision_feedback_component_surface_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_decision_feedback_accessibility_parity_and_narrowing_when_decision_feedback_truth_is_stale as a11y;
use crate::m5_decision_feedback_component_matrix as matrix;
use a11y::M5DecisionFeedbackA11yClaim;
use matrix::{M5DecisionFeedbackDowngradeTrigger, M5DecisionFeedbackFamily};

/// Schema version stamped on the M05-1139 certification packet.
pub const DECISION_FEEDBACK_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`DecisionFeedbackProfileCertificationPacket`].
pub const DECISION_FEEDBACK_CERT_RECORD_KIND: &str =
    "m5_decision_feedback_component_surface_certification_packet";

/// Stable record-kind tag carried by each [`DecisionFeedbackProfileCertificationRow`].
pub const DECISION_FEEDBACK_CERT_ROW_RECORD_KIND: &str =
    "m5_decision_feedback_component_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const DECISION_FEEDBACK_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-decision-feedback-component-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const DECISION_FEEDBACK_CERT_DOC_REF: &str =
    "docs/components/m5_decision_feedback_component_surface_certification_contract.md";

/// Repo-relative path of the frozen decision-feedback component matrix schema the certified profiles
/// render.
pub const DECISION_FEEDBACK_CERT_MATRIX_REF: &str =
    matrix::M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF;

/// The one canonical decision-feedback proof bundle every certified profile cites as its
/// first-resolved component truth. All eight profiles point back to it rather than cloning
/// per-profile evidence.
pub const DECISION_FEEDBACK_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_DECISION_FEEDBACK_COMPONENT_ARTIFACT_REF;

/// The M05-1138 accessibility support export the certification builds on. Recorded as a supporting
/// evidence ref on every row.
pub const DECISION_FEEDBACK_CERT_A11Y_BUNDLE_REF: &str = a11y::DECISION_FEEDBACK_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const DECISION_FEEDBACK_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-decision-feedback-component-surface-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const DECISION_FEEDBACK_CERT_CSV_REF: &str =
    "artifacts/release/m5-decision-feedback-component-surface-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const DECISION_FEEDBACK_CERT_REPORT_REF: &str =
    "artifacts/release/m5-decision-feedback-component-surface-certification-proof/report.md";

/// Stable packet id for the checked-in certification bundle.
pub const DECISION_FEEDBACK_CERT_PACKET_ID: &str =
    "m5-decision-feedback-component-surface-certification:stable:0001";

/// The eight claimed M5 shell / entry / trust / review / repair / notification operating profiles
/// this capstone certifies. Keyed on the profile a user, reviewer, or support engineer reads a
/// decision / feedback primitive through, not on the reusable component family it renders. Only a
/// live, first-party trusted decision profile may certify a trusted decision surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionFeedbackCertifiedProfile {
    /// A live, first-party, fully-current decision surface — a dialog / sheet and consequence block
    /// naming the trusted, scoped, rationale-carrying, blast-radius-explicit decision state exactly
    /// right now.
    LiveTrustedDecisionSurface,
    /// A reviewable decision structure: a self-sufficient, inspectable empty state a user can
    /// review, never itself an authoritative, action-driving decision surface.
    ReviewableDecisionStructure,
    /// A badge / chip / pill surface whose severity evidence is stale; the claim narrows to a
    /// severity-unverified projection with last-known meaning preserved, never a fresh, color-only
    /// severity shown as authoritative.
    StaleSeverityBadgeSurface,
    /// A banner / inline-notice surface whose scope cannot be confirmed; the claim narrows to a
    /// scope-unverified projection that keeps the last-known scope explicit, never an unscoped notice
    /// shown as global truth.
    UnscopedNoticeSurface,
    /// A popover surface whose safe focus-return anchor cannot be confirmed; the claim narrows to a
    /// focus-return-unverified projection that keeps the anchor and content inspectable, never a
    /// popover that strands focus or carries the only critical instruction.
    UnanchoredPopoverSurface,
    /// A toast surface whose durable-object linkage is missing; the claim narrows to a
    /// durable-object-unverified projection that discloses the missing durable back-link, never a
    /// durable outcome shown as toast-only truth.
    ToastOnlyDurableSurface,
    /// A loading-state surface that can only prove partial capability; the claim narrows to a
    /// partial-capability-unverified projection that preserves the useful partial data, never a
    /// full-screen spinner that blanks a useful pane.
    SpinnerLoadingSurface,
    /// A consequence-block surface that can only disclose a partial / redacted recovery / rollback
    /// posture; the claim narrows to a recovery-path-disclosed projection disclosing the partial
    /// recovery posture.
    PartialRecoveryConsequenceSurface,
}

impl M5DecisionFeedbackCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5DecisionFeedbackCertifiedProfile; 8] = [
        M5DecisionFeedbackCertifiedProfile::LiveTrustedDecisionSurface,
        M5DecisionFeedbackCertifiedProfile::ReviewableDecisionStructure,
        M5DecisionFeedbackCertifiedProfile::StaleSeverityBadgeSurface,
        M5DecisionFeedbackCertifiedProfile::UnscopedNoticeSurface,
        M5DecisionFeedbackCertifiedProfile::UnanchoredPopoverSurface,
        M5DecisionFeedbackCertifiedProfile::ToastOnlyDurableSurface,
        M5DecisionFeedbackCertifiedProfile::SpinnerLoadingSurface,
        M5DecisionFeedbackCertifiedProfile::PartialRecoveryConsequenceSurface,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrustedDecisionSurface => "live_trusted_decision_surface",
            Self::ReviewableDecisionStructure => "reviewable_decision_structure",
            Self::StaleSeverityBadgeSurface => "stale_severity_badge_surface",
            Self::UnscopedNoticeSurface => "unscoped_notice_surface",
            Self::UnanchoredPopoverSurface => "unanchored_popover_surface",
            Self::ToastOnlyDurableSurface => "toast_only_durable_surface",
            Self::SpinnerLoadingSurface => "spinner_loading_surface",
            Self::PartialRecoveryConsequenceSurface => "partial_recovery_consequence_surface",
        }
    }

    /// True only for the live, first-party trusted decision surface profile. A trusted decision
    /// surface may be certified on this profile alone; every other profile is at most a reviewable
    /// decision structure or a narrowed projection.
    pub const fn is_live_trusted_decision_surface(self) -> bool {
        matches!(self, Self::LiveTrustedDecisionSurface)
    }
}

/// The eight truth axes a certified profile is scored on. These are exactly the parity dimensions
/// the spec requires verifying — visual, keyboard, screen-reader, high-zoom reflow, reduced-motion,
/// CLI/export, degraded-state, and decision-feedback-component-truth behavior. The CLI/export axis
/// is always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionFeedbackCertificationAxis {
    /// Visual parity: disposition, severity meaning, notice scope, rationale, next action, and
    /// durability are shown on the primary surface without relying on color alone.
    Visual,
    /// Keyboard-reach parity: the same decision truth and its bounded local actions are reachable and
    /// operable without a pointer, never hover-only.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color,
    /// motion, or a chrome glyph alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at high zoom rather than clipping the
    /// disposition, severity, scope, rationale, or recovery copy.
    HighZoomReflow,
    /// Reduced-motion parity: the same truth is legible and usable with reduced motion, never
    /// motion-only.
    ReducedMotion,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale severity evidence, unconfirmed notice scope, unanchored focus
    /// return, missing durable-object linkage, partial capability, or partial recovery posture
    /// honestly downgrades a `TrustedDecisionSurface` / `ReviewableDecisionSurface` claim rather than
    /// reading as a fresh, authoritative decision surface.
    DegradedState,
    /// Decision-feedback-component-truth parity: disposition, severity meaning, notice scope,
    /// rationale, focus-return anchor, durable-object linkage, partial-capability fidelity, and the
    /// recovery / rollback posture stay explicit and never collapse into generic
    /// something-went-wrong chrome, encode meaning by color alone, let a popover carry the only
    /// critical instruction, use generic Yes/No copy in a high-risk dialog, represent durable work as
    /// toast-only truth, blank a useful pane during loading, or use a full-screen spinner where
    /// partial capability exists.
    DecisionFeedbackComponentTruth,
}

impl DecisionFeedbackCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [DecisionFeedbackCertificationAxis; 8] = [
        DecisionFeedbackCertificationAxis::Visual,
        DecisionFeedbackCertificationAxis::Keyboard,
        DecisionFeedbackCertificationAxis::ScreenReader,
        DecisionFeedbackCertificationAxis::HighZoomReflow,
        DecisionFeedbackCertificationAxis::ReducedMotion,
        DecisionFeedbackCertificationAxis::CliExport,
        DecisionFeedbackCertificationAxis::DegradedState,
        DecisionFeedbackCertificationAxis::DecisionFeedbackComponentTruth,
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
            Self::DecisionFeedbackComponentTruth => "decision_feedback_component_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionFeedbackAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a
    /// healthier profile.
    UndisclosedDrift,
}

impl DecisionFeedbackAxisCertificationState {
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
pub enum DecisionFeedbackProfileClaimStatus {
    /// Full standing: every axis certified, every guardrail held, claimed decision tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a guardrail breaks, CLI/export parity
    /// drops, a non-live profile claims a trusted decision surface, or the narrowing is inconsistent.
    Red,
}

impl DecisionFeedbackProfileClaimStatus {
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

/// The six B135 guardrails carried on every certified profile. All six must hold — a breach blocks
/// the profile (red). Each field is `true` only when the profile *breaks* the guardrail, so a clean
/// profile carries all-false.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackCertGuardrails {
    /// True if the profile relies on color alone for badge / banner / inline-notice meaning. Must be
    /// false.
    pub relies_on_color_alone_for_meaning: bool,
    /// True if the profile lets a popover carry the only critical workflow instruction. Must be
    /// false.
    pub lets_popover_carry_only_critical_instruction: bool,
    /// True if the profile uses generic Yes/No confirmation copy in a high-risk dialog. Must be
    /// false.
    pub uses_generic_yes_no_in_high_risk_dialog: bool,
    /// True if the profile represents long-running or reviewable work as toast-only truth. Must be
    /// false.
    pub represents_durable_work_as_toast_only: bool,
    /// True if the profile blanks a useful pane during loading. Must be false.
    pub blanks_useful_pane_during_loading: bool,
    /// True if the profile uses a full-screen spinner where partial capability exists. Must be false.
    pub uses_full_screen_spinner_when_partial_capable: bool,
}

impl DecisionFeedbackCertGuardrails {
    /// A clean profile: every guardrail held.
    pub const CLEAN: Self = Self {
        relies_on_color_alone_for_meaning: false,
        lets_popover_carry_only_critical_instruction: false,
        uses_generic_yes_no_in_high_risk_dialog: false,
        represents_durable_work_as_toast_only: false,
        blanks_useful_pane_during_loading: false,
        uses_full_screen_spinner_when_partial_capable: false,
    };

    /// True when every guardrail holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.relies_on_color_alone_for_meaning
            && !self.lets_popover_carry_only_critical_instruction
            && !self.uses_generic_yes_no_in_high_risk_dialog
            && !self.represents_durable_work_as_toast_only
            && !self.blanks_useful_pane_during_loading
            && !self.uses_full_screen_spinner_when_partial_capable
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when
/// this offers text / JSON / Markdown reconstruction and prohibits a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The disposition / severity-meaning / notice-scope / rationale / durable-object /
    /// partial-capability / recovery-posture fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl DecisionFeedbackCertExportParity {
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
pub struct DecisionFeedbackAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: DecisionFeedbackCertificationAxis,
    /// The certification state of the axis.
    pub state: DecisionFeedbackAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5DecisionFeedbackDowngradeTrigger>,
}

impl DecisionFeedbackAxisOutcome {
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
            DecisionFeedbackAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            DecisionFeedbackAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            DecisionFeedbackAxisCertificationState::UndisclosedDrift => {
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
pub struct DecisionFeedbackClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: DecisionFeedbackCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5DecisionFeedbackA11yClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5DecisionFeedbackA11yClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 shell / entry / trust / review / repair / notification decision profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackProfileCertificationRow {
    /// Record kind; must equal [`DECISION_FEEDBACK_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DECISION_FEEDBACK_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5DecisionFeedbackCertifiedProfile,
    /// The decision claim ceiling the profile asserts.
    pub claimed_claim: M5DecisionFeedbackA11yClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5DecisionFeedbackA11yClaim,
    /// The frozen component families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5DecisionFeedbackFamily>,
    /// One outcome per [`DecisionFeedbackCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<DecisionFeedbackAxisOutcome>,
    /// The B135 guardrails; all must hold.
    pub guardrails: DecisionFeedbackCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<DecisionFeedbackClaimAutoNarrow>,
    /// The one canonical decision-feedback proof bundle this profile cites. Must equal
    /// [`DECISION_FEEDBACK_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: DecisionFeedbackProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: DecisionFeedbackCertExportParity,
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

impl DecisionFeedbackProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(
        &self,
        axis: DecisionFeedbackCertificationAxis,
    ) -> Option<&DecisionFeedbackAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<DecisionFeedbackCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && DecisionFeedbackCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(DecisionFeedbackAxisOutcome::well_formed)
    }

    /// True when the profile narrows its decision claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<DecisionFeedbackCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == DecisionFeedbackAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, guardrails, and claim narrowing. This is the heart
    /// of the capstone: a degraded axis must produce a visible claim narrowing, only a live
    /// first-party profile may certify a trusted decision surface, every guardrail must hold,
    /// CLI/export parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> DecisionFeedbackProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != DECISION_FEEDBACK_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return DecisionFeedbackProfileClaimStatus::Red;
        }

        // Every B135 guardrail must hold.
        if !self.guardrails.all_held() {
            return DecisionFeedbackProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return DecisionFeedbackProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a trusted decision surface.
        if self.certified_claim.asserts_trusted_surface()
            && !self.profile.is_live_trusted_decision_surface()
        {
            return DecisionFeedbackProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(DecisionFeedbackCertificationAxis::CliExport) {
            Some(o) if o.state == DecisionFeedbackAxisCertificationState::Certified => {}
            _ => return DecisionFeedbackProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == DecisionFeedbackAxisCertificationState::UndisclosedDrift)
        {
            return DecisionFeedbackProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return DecisionFeedbackProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return DecisionFeedbackProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return DecisionFeedbackProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return DecisionFeedbackProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier profile's truth.
        if !narrowed.is_empty() {
            return DecisionFeedbackProfileClaimStatus::Red;
        }

        DecisionFeedbackProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == DECISION_FEEDBACK_CERT_ROW_RECORD_KIND
            && self.schema_version == DECISION_FEEDBACK_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1139 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackProfileCertificationSummary {
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

/// Constructor input for [`DecisionFeedbackProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionFeedbackProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<DecisionFeedbackProfileCertificationRow>,
}

/// Checked-in M05-1139 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionFeedbackProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<DecisionFeedbackProfileCertificationRow>,
    pub summary: DecisionFeedbackProfileCertificationSummary,
}

impl DecisionFeedbackProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: DecisionFeedbackProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: DECISION_FEEDBACK_CERT_SCHEMA_VERSION,
            record_kind: DECISION_FEEDBACK_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: DecisionFeedbackProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5DecisionFeedbackCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Component families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5DecisionFeedbackFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5DecisionFeedbackCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen component family is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5DecisionFeedbackFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(DecisionFeedbackCertificationAxis::CliExport)
                .is_some_and(|o| o.state == DecisionFeedbackAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> DecisionFeedbackProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == DecisionFeedbackProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == DecisionFeedbackProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == DecisionFeedbackProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(DecisionFeedbackProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        DecisionFeedbackProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == DECISION_FEEDBACK_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(DecisionFeedbackProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<DecisionFeedbackCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != DECISION_FEEDBACK_CERT_SCHEMA_VERSION {
            violations.push(DecisionFeedbackCertificationViolation::SchemaVersion {
                expected: DECISION_FEEDBACK_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != DECISION_FEEDBACK_CERT_RECORD_KIND {
            violations.push(DecisionFeedbackCertificationViolation::RecordKind {
                expected: DECISION_FEEDBACK_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(DecisionFeedbackCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != DECISION_FEEDBACK_CERT_CANONICAL_BUNDLE_REF {
            violations.push(DecisionFeedbackCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(DecisionFeedbackCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(DecisionFeedbackCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(
                    DecisionFeedbackCertificationViolation::AxisCoverageIncomplete {
                        id: row.row_id.clone(),
                    },
                );
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(
                    DecisionFeedbackCertificationViolation::MalformedAxisOutcome {
                        id: row.row_id.clone(),
                    },
                );
            }

            if row.canonical_bundle_ref != DECISION_FEEDBACK_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    DecisionFeedbackCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B135 guardrail must hold.
            if !row.guardrails.all_held() {
                violations.push(DecisionFeedbackCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a live first-party profile may certify a trusted decision surface.
            if row.certified_claim.asserts_trusted_surface()
                && !row.profile.is_live_trusted_decision_surface()
            {
                violations.push(
                    DecisionFeedbackCertificationViolation::NonLiveProfileClaimsTrustedSurface {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(DecisionFeedbackCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    DecisionFeedbackCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    DecisionFeedbackCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(
                    DecisionFeedbackCertificationViolation::StatusDerivationStale {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == DecisionFeedbackProfileClaimStatus::Red {
                violations.push(DecisionFeedbackCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(DecisionFeedbackCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen component family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(DecisionFeedbackCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(DecisionFeedbackCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(DecisionFeedbackCertificationViolation::RawComponentMaterialInExport);
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
        out.push_str("# M5 Decision-Feedback Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5DecisionFeedbackCertifiedProfile::ALL.len(),
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
pub fn current_m5_decision_feedback_component_surface_certification_export(
) -> Result<DecisionFeedbackProfileCertificationPacket, DecisionFeedbackCertificationArtifactError>
{
    let packet: DecisionFeedbackProfileCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-decision-feedback-component-surface-certification-proof/support_export.json"
    )))
    .map_err(DecisionFeedbackCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DecisionFeedbackCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum DecisionFeedbackCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DecisionFeedbackCertificationViolation>),
}

impl fmt::Display for DecisionFeedbackCertificationArtifactError {
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

impl Error for DecisionFeedbackCertificationArtifactError {}

/// Validation failure for M05-1139 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecisionFeedbackCertificationViolation {
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
    RawComponentMaterialInExport,
}

impl fmt::Display for DecisionFeedbackCertificationViolation {
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
                    "packet does not cite the canonical decision-feedback proof bundle"
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
                    "row {id} does not cite the one canonical decision-feedback proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B135 guardrail: color-alone meaning, a popover carrying the \
only critical instruction, generic Yes/No copy in a high-risk dialog, durable work shown as \
toast-only truth, a useful pane blanked during loading, or a full-screen spinner where partial \
capability exists"
                )
            }
            Self::NonLiveProfileClaimsTrustedSurface { id } => {
                write!(
                    f,
                    "row {id} certifies a trusted decision surface on a non-live first-party profile"
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
claim, a guardrail broke, CLI/export parity dropped, a non-live profile claimed a trusted decision \
surface, or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 decision-feedback profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen decision-feedback component family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawComponentMaterialInExport => {
                write!(
                    f,
                    "export contains a raw field value, copy payload, credential, or secret material"
                )
            }
        }
    }
}

impl Error for DecisionFeedbackCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&DecisionFeedbackAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != DecisionFeedbackAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the decision /
/// feedback generics the spec forbids collapsing distinct disposition, severity, scope, focus-return,
/// durability, capability, and recovery truth into (whole-label matches so a full sentence naming a
/// concrete state, scope, or recovery posture is not flagged).
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
            | "dismissed"
            | "acknowledged"
            | "partial"
            | "cached"
            | "trusted"
            | "reviewable"
            | "badge"
            | "chip"
            | "pill"
            | "popover"
            | "dialog"
            | "sheet"
            | "banner"
            | "inline notice"
            | "notice"
            | "toast"
            | "empty state"
            | "loading state"
            | "consequence"
            | "consequence block"
            | "severity"
            | "scope"
            | "focus return"
            | "durable object"
            | "recovery"
            | "recovery path"
            | "rollback"
            | "blast radius"
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

/// Builds the canonical, checked-in M05-1139 certification packet. Certifies all eight claimed M5
/// shell / entry / trust / review / repair / notification profiles: two deliver their claim (green)
/// and six auto-narrow a not-current truth axis to a weaker decision ceiling (yellow). No profile
/// hides drift or breaks a guardrail (red).
pub fn seeded_m5_decision_feedback_component_surface_certification_packet(
) -> DecisionFeedbackProfileCertificationPacket {
    DecisionFeedbackProfileCertificationPacket::new(
        DecisionFeedbackProfileCertificationPacketInput {
            packet_id: DECISION_FEEDBACK_CERT_PACKET_ID.to_owned(),
            as_of: "2026-07-13T00:00:00Z".to_owned(),
            matrix_ref: DECISION_FEEDBACK_CERT_MATRIX_REF.to_owned(),
            canonical_bundle_ref: DECISION_FEEDBACK_CERT_CANONICAL_BUNDLE_REF.to_owned(),
            rows: seeded_rows(),
        },
    )
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:decision-feedback-component-certification:{id}"),
        DECISION_FEEDBACK_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> DecisionFeedbackCertExportParity {
    DecisionFeedbackCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: DecisionFeedbackCertificationAxis) -> &'static str {
    match axis {
        DecisionFeedbackCertificationAxis::Visual => {
            "disposition, severity meaning, notice scope, rationale, next action, and durability shown on-surface without color alone"
        }
        DecisionFeedbackCertificationAxis::Keyboard => {
            "the same decision state, severity, scope, rationale, recovery posture, and bounded local actions are keyboard-reachable, never hover-only"
        }
        DecisionFeedbackCertificationAxis::ScreenReader => {
            "the same decision / feedback truth is announced non-visually, never color/motion/glyph-only"
        }
        DecisionFeedbackCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at high zoom without clipping the disposition, severity, scope, rationale, or recovery copy"
        }
        DecisionFeedbackCertificationAxis::ReducedMotion => {
            "the same truth stays legible and usable with reduced motion, never motion-only"
        }
        DecisionFeedbackCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        DecisionFeedbackCertificationAxis::DegradedState => {
            "a stale severity evidence, unconfirmed notice scope, unanchored focus return, missing durable-object linkage, partial capability, or partial recovery posture honestly downgrades the TrustedDecisionSurface/ReviewableDecisionSurface claim rather than reading as a fresh authoritative decision surface"
        }
        DecisionFeedbackCertificationAxis::DecisionFeedbackComponentTruth => {
            "disposition, severity meaning, notice scope, rationale, focus-return anchor, durable-object linkage, partial-capability fidelity, and the recovery / rollback posture stay explicit and never collapse into generic something-went-wrong chrome, encode meaning by color alone, let a popover carry the only critical instruction, use generic Yes/No copy in a high-risk dialog, represent durable work as toast-only truth, blank a useful pane during loading, or use a full-screen spinner where partial capability exists"
        }
    }
}

fn seed_certified(axis: DecisionFeedbackCertificationAxis) -> DecisionFeedbackAxisOutcome {
    DecisionFeedbackAxisOutcome {
        axis,
        state: DecisionFeedbackAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: DecisionFeedbackCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5DecisionFeedbackDowngradeTrigger,
) -> DecisionFeedbackAxisOutcome {
    DecisionFeedbackAxisOutcome {
        axis,
        state: DecisionFeedbackAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<DecisionFeedbackAxisOutcome> {
    DecisionFeedbackCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: DecisionFeedbackCertificationAxis,
    outcome: DecisionFeedbackAxisOutcome,
) -> Vec<DecisionFeedbackAxisOutcome> {
    DecisionFeedbackCertificationAxis::ALL
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
    profile: M5DecisionFeedbackCertifiedProfile,
    claimed_claim: M5DecisionFeedbackA11yClaim,
    certified_claim: M5DecisionFeedbackA11yClaim,
    consumed_families: &[M5DecisionFeedbackFamily],
    axis_outcomes: Vec<DecisionFeedbackAxisOutcome>,
    claim_auto_narrow: Option<DecisionFeedbackClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> DecisionFeedbackProfileCertificationRow {
    let mut row = DecisionFeedbackProfileCertificationRow {
        record_kind: DECISION_FEEDBACK_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: DECISION_FEEDBACK_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: DecisionFeedbackCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: DECISION_FEEDBACK_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: DecisionFeedbackProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            DECISION_FEEDBACK_CERT_MATRIX_REF.to_owned(),
            DECISION_FEEDBACK_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-13T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: DecisionFeedbackCertificationAxis,
    from_claim: M5DecisionFeedbackA11yClaim,
    to_claim: M5DecisionFeedbackA11yClaim,
    label: &str,
) -> DecisionFeedbackClaimAutoNarrow {
    DecisionFeedbackClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<DecisionFeedbackProfileCertificationRow> {
    use DecisionFeedbackCertificationAxis as Ax;
    use M5DecisionFeedbackA11yClaim::*;
    use M5DecisionFeedbackCertifiedProfile as P;
    use M5DecisionFeedbackDowngradeTrigger as Trig;
    use M5DecisionFeedbackFamily::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-trusted-decision-surface",
            P::LiveTrustedDecisionSurface,
            TrustedDecisionSurface,
            TrustedDecisionSurface,
            &[DialogSheet, ConsequenceBlock],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "disposition"],
            &[
                "dialog / sheet names its rationale, named scope, and explicit action labels without relying on generic Yes/No, and returns focus to a safe anchor on reopen",
                "consequence block names its blast radius and rollback / help posture rather than a color-only or toast-only outcome",
                "keyboard / screen-reader / high-zoom / reduced-motion reach preserved for the dialog and the consequence block",
                "decision-feedback-component-truth: a live first-party decision surface is the only profile that certifies a trusted decision surface",
            ],
        ),
        seed_row(
            "cert:reviewable-decision-structure",
            P::ReviewableDecisionStructure,
            ReviewableDecisionSurface,
            ReviewableDecisionSurface,
            &[EmptyState],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "purpose"],
            &[
                "empty state explains its purpose, current emptiness, and best next action without decorative filler or a blank pane",
                "the empty-state card keeps its purpose and next-action copy legible rather than a generic something-went-wrong placeholder",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable structure",
                "decision-feedback-component-truth: a reviewable read-only empty state never certifies a live trusted, action-driving claim",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:stale-severity-badge-surface",
            P::StaleSeverityBadgeSurface,
            ReviewableDecisionSurface,
            SeverityUnverifiedProjection,
            &[BadgeChipPill],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the badge / chip / pill severity evidence is stale so a fresh, plain-language severity meaning cannot be certified",
                    "The badge severity evidence is stale, so the ReviewableDecisionSurface claim narrows to a severity-unverified projection and the primitive preserves its last-known plain-language meaning rather than presenting a fresh, color-only severity as authoritative",
                    Trig::StateTaxonomyDrifted,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableDecisionSurface,
                SeverityUnverifiedProjection,
                "Severity unverified: the severity evidence is stale so the last-known plain-language meaning is preserved and the badge never reads as a fresh, color-only severity",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "badge preserves its last-known plain-language meaning and marks the severity as unverified rather than presenting a stale, color-only severity as authoritative",
                "the badge expands into plain language off-hover while the severity evidence is disclosed as stale",
                "degraded-state: ReviewableDecisionSurface narrows to a severity-unverified projection (auto-narrowed)",
                "decision-feedback-component-truth: a stale severity is never shown as a fresh, color-only badge",
            ],
        ),
        seed_row(
            "cert:unscoped-notice-surface",
            P::UnscopedNoticeSurface,
            ReviewableDecisionSurface,
            ScopeUnverifiedProjection,
            &[BannerInlineNotice],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the banner / inline notice's scope cannot be confirmed so a scoped, global-truth notice cannot be certified",
                    "The banner / inline notice's scope cannot be confirmed, so the ReviewableDecisionSurface claim narrows to a scope-unverified projection and the notice keeps its last-known scope explicit rather than presenting an unscoped notice as global truth",
                    Trig::ScopeUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableDecisionSurface,
                ScopeUnverifiedProjection,
                "Scope unverified: the notice scope cannot be confirmed so the last-known scope stays explicit and the notice never reads as unscoped global truth",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "banner keeps its last-known scope and what-still-works copy explicit and marks the scope as unverified rather than presenting an unscoped notice as global truth",
                "the notice keeps its scoped cause and primary next-action visible while the scope is disclosed as unverified",
                "degraded-state: ReviewableDecisionSurface narrows to a scope-unverified projection (auto-narrowed)",
                "decision-feedback-component-truth: an unscoped notice never silently reads as global truth or a color-only alert",
            ],
        ),
        seed_row(
            "cert:unanchored-popover-surface",
            P::UnanchoredPopoverSurface,
            ReviewableDecisionSurface,
            FocusReturnUnverifiedProjection,
            &[Popover],
            seed_certified_except(
                Ax::DecisionFeedbackComponentTruth,
                seed_narrowed(
                    Ax::DecisionFeedbackComponentTruth,
                    "the popover's safe focus-return anchor cannot be confirmed so a focus-anchored, lightweight secondary popover cannot be certified",
                    "The popover's safe focus-return anchor cannot be confirmed, so the ReviewableDecisionSurface claim narrows to a focus-return-unverified projection and the popover keeps its anchor and content inspectable rather than stranding focus or carrying the only critical instruction",
                    Trig::PopoverCarriedOnlyCriticalInstruction,
                ),
            ),
            Some(seed_narrow(
                Ax::DecisionFeedbackComponentTruth,
                ReviewableDecisionSurface,
                FocusReturnUnverifiedProjection,
                "Focus return unverified: the safe focus-return anchor cannot be confirmed so the anchor and content stay inspectable and the popover never strands focus or carries the only critical instruction",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "popover keeps its anchor and content inspectable and marks the focus return as unverified rather than stranding focus or carrying the only critical instruction",
                "the popover stays a lightweight secondary control while the focus-return anchor is disclosed as unverified",
                "decision-feedback-component-truth: ReviewableDecisionSurface narrows to a focus-return-unverified projection (auto-narrowed)",
                "decision-feedback-component-truth: a popover never carries the only critical workflow instruction",
            ],
        ),
        seed_row(
            "cert:toast-only-durable-surface",
            P::ToastOnlyDurableSurface,
            ReviewableDecisionSurface,
            DurableObjectUnverifiedProjection,
            &[Toast],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the toast's durable-object linkage is missing so a durable, back-linked acknowledgement cannot be certified",
                    "The toast's durable-object linkage is missing, so the ReviewableDecisionSurface claim narrows to a durable-object-unverified projection and the toast discloses the missing durable back-link rather than presenting a durable outcome as toast-only truth",
                    Trig::DurableWorkShownAsToastOnly,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableDecisionSurface,
                DurableObjectUnverifiedProjection,
                "Durable object unverified: the durable-object linkage is missing so the missing durable back-link is disclosed and the toast never reads as the only durable truth",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "toast discloses the missing durable back-link and marks the durable-object linkage as unverified rather than presenting a durable outcome as toast-only truth",
                "the toast stays an acknowledgement carrying one bounded action while the durable-object linkage is disclosed as missing",
                "degraded-state: ReviewableDecisionSurface narrows to a durable-object-unverified projection (auto-narrowed)",
                "decision-feedback-component-truth: a durable outcome is never represented as toast-only truth",
            ],
        ),
        seed_row(
            "cert:spinner-loading-surface",
            P::SpinnerLoadingSurface,
            ReviewableDecisionSurface,
            PartialCapabilityUnverifiedProjection,
            &[LoadingState],
            seed_certified_except(
                Ax::DecisionFeedbackComponentTruth,
                seed_narrowed(
                    Ax::DecisionFeedbackComponentTruth,
                    "the loading state can only prove partial capability so a fully-ready, non-blanking loading state cannot be certified",
                    "The loading state can only prove partial capability, so the ReviewableDecisionSurface claim narrows to a partial-capability-unverified projection and the loading state preserves the useful partial data rather than blanking a useful pane behind a full-screen spinner",
                    Trig::FullScreenSpinnerWhenPartialCapable,
                ),
            ),
            Some(seed_narrow(
                Ax::DecisionFeedbackComponentTruth,
                ReviewableDecisionSurface,
                PartialCapabilityUnverifiedProjection,
                "Partial capability unverified: only partial capability is provable so the useful partial data is preserved and no useful pane is blanked behind a full-screen spinner",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "loading state preserves the useful partial data and marks the capability as partial rather than blanking a useful pane behind a full-screen spinner",
                "the loading state distinguishes skeleton / retained-content / partial-streaming treatments while the capability is disclosed as partial",
                "decision-feedback-component-truth: ReviewableDecisionSurface narrows to a partial-capability-unverified projection (auto-narrowed)",
                "decision-feedback-component-truth: a useful pane is never blanked and a full-screen spinner is never used where partial capability exists",
            ],
        ),
        seed_row(
            "cert:partial-recovery-consequence-surface",
            P::PartialRecoveryConsequenceSurface,
            ReviewableDecisionSurface,
            RecoveryPathDisclosedProjection,
            &[ConsequenceBlock],
            seed_certified_except(
                Ax::DecisionFeedbackComponentTruth,
                seed_narrowed(
                    Ax::DecisionFeedbackComponentTruth,
                    "the consequence block can only disclose a partial / redacted recovery / rollback posture so a fully-reversible, no-consequence block cannot be certified",
                    "The consequence block can only disclose a partial / redacted recovery / rollback posture, so the ReviewableDecisionSurface claim narrows to a recovery-path-disclosed projection and the block discloses the partial recovery posture inspectably rather than presenting itself as a fully-reversible, no-consequence block",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::DecisionFeedbackComponentTruth,
                ReviewableDecisionSurface,
                RecoveryPathDisclosedProjection,
                "Recovery path disclosed partial: the recovery / rollback posture is partial so the block discloses it inspectably and never reads as fully reversible with no consequence",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "consequence block discloses its partial / redacted recovery posture and preserves its named blast radius and rollback / help posture rather than claiming to be a fully-reversible, no-consequence block",
                "the block keeps its named blast radius and rollback / help hooks while the recovery posture is disclosed as partial",
                "decision-feedback-component-truth: ReviewableDecisionSurface narrows to a recovery-path-disclosed projection (auto-narrowed)",
                "decision-feedback-component-truth: a partial recovery posture is disclosed honestly, never presented as fully reversible",
            ],
        ),
    ]
}
