//! M05-1123 surface certification over the frozen M5 editor-tab / gutter /
//! diagnostic-decoration / code-action-chip / diff-view / review-thread /
//! AI-message-card / evidence-timeline editor-inline component matrix.
//!
//! Where the freeze matrix ([`crate::m5_editor_inline_component_matrix`]) defines the eight
//! reusable editor-tab, gutter, diagnostic-decoration, code-action-chip, diff-view,
//! review-thread, AI-message-card, and evidence-timeline components, the M05-1117..1120
//! implement lanes narrow each one, the M05-1121 shared consumer lane aligns their
//! vocabulary, and the M05-1122 accessibility lane
//! ([`crate::m5_editor_inline_accessibility_parity_and_narrowing_when_evidence_truth_is_stale`])
//! proves keyboard / screen-reader / high-zoom / reduced-motion / CLI-export parity and
//! per-family auto-narrowing, this closing capstone *certifies* that the shared inline
//! component truth holds on every claimed M5 editor / review / AI operating profile — and
//! auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a user, reviewer, or support engineer reads inline
//! editor / review / AI truth through (a live, first-party trusted inline surface; a
//! reviewable inline structure; a drifted-anchor surface; a stale-severity decoration; an
//! inferred-fix chip; a stale-confidence message; an unverified-approval thread; and a
//! partial-evidence timeline), not on component family or implement lane. Each
//! [`EditorInlineProfileCertificationRow`] certifies one profile across eight truth axes —
//! visual, keyboard, screen-reader, high-zoom-reflow, reduced-motion, CLI/export,
//! degraded-state, and inline-component-truth behavior — and either passes (green),
//! auto-narrows its inline claim to the weakest supported ceiling (yellow), or is blocked
//! (red) when a degraded axis is hidden behind a fresh trusted claim inherited from a
//! healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile
//! that keeps a `TrustedInlineResult` / `ReviewableInlineResult` claim while one of its truth
//! axes is not current is over-claiming and blocks; a profile that discloses the reduction by
//! narrowing its claim (with a bound reason and a frozen downgrade trigger) is honestly
//! yellow. Only a live, first-party trusted inline profile may certify a `TrustedInlineResult`
//! claim — a reviewable, drifted, stale, inferred, unverified, or partial profile that keeps a
//! trusted claim is over-reaching and blocks. The always-on CLI/export axis must always stay
//! certified so support and automation can reconstruct the inline state, anchor durability,
//! severity / source, fix posture, confidence / source context, approval / outdated-versus-
//! resolved state, and evidence lineage from the same component identity the user saw.
//!
//! The B133 guardrails are enforced per row: no profile may encode tab / marker / diagnostic
//! state by color alone, let a comment anchor or AI evidence pointer silently drift, blur
//! outdated and resolved review state, present an inferred fix as exact, or hide an evidence
//! timeline in an opaque log. A profile that breaches any guardrail blocks (red).
//!
//! Every row cites exactly one canonical editor-inline proof bundle
//! ([`EDITOR_INLINE_CERT_CANONICAL_BUNDLE_REF`]) — the frozen editor-inline component matrix
//! proof — rather than cloning per-profile evidence. The packet is metadata-only: raw editor
//! buffers, diff bodies, comment payloads, AI message bodies, credentials, and endpoint refs
//! never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-editor-inline-component-surface-certification.schema.json`](../../../../schemas/ui/m5-editor-inline-component-surface-certification.schema.json).
//! The contract doc is
//! [`docs/editor/m5_editor_inline_component_surface_certification_contract.md`](../../../../docs/editor/m5_editor_inline_component_surface_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_editor_inline_accessibility_parity_and_narrowing_when_evidence_truth_is_stale as a11y;
use crate::m5_editor_inline_component_matrix as matrix;
use a11y::M5EditorInlineComponentClaim;
use matrix::{M5EditorInlineComponentFamily, M5EditorInlineDowngradeTrigger};

/// Schema version stamped on the M05-1123 certification packet.
pub const EDITOR_INLINE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`EditorInlineProfileCertificationPacket`].
pub const EDITOR_INLINE_CERT_RECORD_KIND: &str =
    "m5_editor_inline_component_surface_certification_packet";

/// Stable record-kind tag carried by each [`EditorInlineProfileCertificationRow`].
pub const EDITOR_INLINE_CERT_ROW_RECORD_KIND: &str =
    "m5_editor_inline_component_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const EDITOR_INLINE_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-editor-inline-component-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const EDITOR_INLINE_CERT_DOC_REF: &str =
    "docs/editor/m5_editor_inline_component_surface_certification_contract.md";

/// Repo-relative path of the frozen editor-inline component matrix schema the certified
/// profiles render.
pub const EDITOR_INLINE_CERT_MATRIX_REF: &str = matrix::M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF;

/// The one canonical editor-inline proof bundle every certified profile cites as its
/// first-resolved component truth. All eight profiles point back to it rather than cloning
/// per-profile evidence.
pub const EDITOR_INLINE_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_EDITOR_INLINE_COMPONENT_ARTIFACT_REF;

/// The M05-1122 accessibility support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const EDITOR_INLINE_CERT_A11Y_BUNDLE_REF: &str = a11y::EDITOR_INLINE_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const EDITOR_INLINE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-editor-inline-component-surface-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EDITOR_INLINE_CERT_CSV_REF: &str =
    "artifacts/release/m5-editor-inline-component-surface-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const EDITOR_INLINE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-editor-inline-component-surface-certification-proof/report.md";

/// The eight claimed M5 editor / review / AI operating profiles this capstone certifies.
/// Keyed on the profile a user, reviewer, or support engineer reads inline truth through, not
/// on the reusable component family it renders. Only a live, first-party trusted inline
/// profile may certify a trusted inline result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineCertifiedProfile {
    /// A live, first-party, fully-current inline surface — an editor tab and code-action chip
    /// naming the trusted, durably-anchored, apply-ready inline state exactly right now.
    LiveTrustedInlineSurface,
    /// A reviewable inline structure: a self-sufficient read-only diff / gutter a user can
    /// review, never itself an authoritative trusted apply surface.
    ReviewableInlineStructure,
    /// A diagnostic / review surface whose comment / diagnostic anchor durability is stale;
    /// the claim narrows to an anchor-unverified projection with last-known identity preserved.
    DriftedAnchorSurface,
    /// A diagnostic decoration whose severity / source attribution is stale; the claim narrows
    /// to a severity-unverified projection naming the last-known severity.
    StaleSeverityDecoration,
    /// A code-action chip whose fix is only inferred; the claim narrows to a
    /// fix-posture-unverified projection that names it an inferred fix, never an exact change.
    InferredFixChip,
    /// An AI message card whose confidence / source context is stale; the claim narrows to a
    /// confidence-unverified projection disclosing the last-known confidence.
    StaleConfidenceMessage,
    /// A review thread whose approval / outdated-versus-resolved state is unverified; the claim
    /// narrows to an approval-unverified projection keeping the last-known thread state.
    UnverifiedApprovalThread,
    /// An evidence timeline whose lineage is only partial / redacted; the claim narrows to an
    /// evidence-lineage projection disclosing the partial / redacted lineage.
    PartialEvidenceTimeline,
}

impl M5EditorInlineCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5EditorInlineCertifiedProfile; 8] = [
        M5EditorInlineCertifiedProfile::LiveTrustedInlineSurface,
        M5EditorInlineCertifiedProfile::ReviewableInlineStructure,
        M5EditorInlineCertifiedProfile::DriftedAnchorSurface,
        M5EditorInlineCertifiedProfile::StaleSeverityDecoration,
        M5EditorInlineCertifiedProfile::InferredFixChip,
        M5EditorInlineCertifiedProfile::StaleConfidenceMessage,
        M5EditorInlineCertifiedProfile::UnverifiedApprovalThread,
        M5EditorInlineCertifiedProfile::PartialEvidenceTimeline,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrustedInlineSurface => "live_trusted_inline_surface",
            Self::ReviewableInlineStructure => "reviewable_inline_structure",
            Self::DriftedAnchorSurface => "drifted_anchor_surface",
            Self::StaleSeverityDecoration => "stale_severity_decoration",
            Self::InferredFixChip => "inferred_fix_chip",
            Self::StaleConfidenceMessage => "stale_confidence_message",
            Self::UnverifiedApprovalThread => "unverified_approval_thread",
            Self::PartialEvidenceTimeline => "partial_evidence_timeline",
        }
    }

    /// True only for the live, first-party trusted inline surface profile. A trusted inline
    /// result may be certified on this profile alone; every other profile is at most a
    /// reviewable inline result or a narrowed projection.
    pub const fn is_live_trusted_inline(self) -> bool {
        matches!(self, Self::LiveTrustedInlineSurface)
    }
}

/// The eight truth axes a certified profile is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader, high-zoom
/// reflow, reduced-motion, CLI/export, degraded-state, and inline-component-truth behavior.
/// The CLI/export axis is always-on and must stay certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorInlineCertificationAxis {
    /// Visual parity: inline state, anchor durability, severity / source, fix posture,
    /// confidence / source context, approval / resolution, and evidence lineage are shown on
    /// the primary surface without relying on color alone.
    Visual,
    /// Keyboard-reach parity: the same inline truth and its bounded local actions are
    /// reachable and operable without a pointer, never hover-only.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color,
    /// motion, or a chrome glyph alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at high zoom rather than
    /// clipping the state, anchor, severity, or fix posture.
    HighZoomReflow,
    /// Reduced-motion parity: the same truth is legible and usable with reduced motion, never
    /// motion-only.
    ReducedMotion,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a drifted anchor, stale severity, inferred fix, stale
    /// confidence, unverified approval, or partial evidence lineage honestly downgrades a
    /// `TrustedInlineResult` / `ReviewableInlineResult` claim rather than reading as a fresh,
    /// authoritative inline result.
    DegradedState,
    /// Inline-component-truth parity: inline state, anchor durability, severity / source, fix
    /// posture, confidence / source context, approval / outdated-versus-resolved state, and
    /// evidence lineage stay explicit and never collapse into generic chrome wording, encode
    /// state by color alone, let anchors or evidence pointers drift, blur outdated and resolved
    /// state, present an inferred fix as exact, or hide an evidence timeline in an opaque log.
    InlineComponentTruth,
}

impl EditorInlineCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [EditorInlineCertificationAxis; 8] = [
        EditorInlineCertificationAxis::Visual,
        EditorInlineCertificationAxis::Keyboard,
        EditorInlineCertificationAxis::ScreenReader,
        EditorInlineCertificationAxis::HighZoomReflow,
        EditorInlineCertificationAxis::ReducedMotion,
        EditorInlineCertificationAxis::CliExport,
        EditorInlineCertificationAxis::DegradedState,
        EditorInlineCertificationAxis::InlineComponentTruth,
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
            Self::InlineComponentTruth => "inline_component_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorInlineAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible
    /// claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited
    /// from a healthier profile.
    UndisclosedDrift,
}

impl EditorInlineAxisCertificationState {
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
pub enum EditorInlineProfileClaimStatus {
    /// Full standing: every axis certified, every guardrail held, claimed inline tier
    /// delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a guardrail breaks, CLI/export
    /// parity drops, a non-live profile claims a trusted result, or the narrowing is
    /// inconsistent.
    Red,
}

impl EditorInlineProfileClaimStatus {
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

/// The five B133 guardrails carried on every certified profile. All five must hold — a breach
/// blocks the profile (red). Each field is `true` only when the profile *breaks* the
/// guardrail, so a clean profile carries all-false.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineCertGuardrails {
    /// True if the profile encodes tab / marker / diagnostic state by color alone. Must be
    /// false.
    pub encodes_state_by_color_alone: bool,
    /// True if the profile lets a comment anchor or AI evidence pointer silently drift. Must
    /// be false.
    pub lets_anchor_or_evidence_pointer_drift: bool,
    /// True if the profile blurs outdated and resolved review state. Must be false.
    pub blurs_outdated_and_resolved_review_state: bool,
    /// True if the profile presents an inferred fix as exact. Must be false.
    pub presents_inferred_fix_as_exact: bool,
    /// True if the profile hides an evidence timeline in an opaque log. Must be false.
    pub hides_evidence_timeline_in_opaque_log: bool,
}

impl EditorInlineCertGuardrails {
    /// A clean profile: every guardrail held.
    pub const CLEAN: Self = Self {
        encodes_state_by_color_alone: false,
        lets_anchor_or_evidence_pointer_drift: false,
        blurs_outdated_and_resolved_review_state: false,
        presents_inferred_fix_as_exact: false,
        hides_evidence_timeline_in_opaque_log: false,
    };

    /// True when every guardrail holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.encodes_state_by_color_alone
            && !self.lets_anchor_or_evidence_pointer_drift
            && !self.blurs_outdated_and_resolved_review_state
            && !self.presents_inferred_fix_as_exact
            && !self.hides_evidence_timeline_in_opaque_log
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only
/// when this offers text / JSON / Markdown reconstruction and prohibits a screenshot-only
/// export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The inline-state / anchor / severity / fix-posture / confidence / approval /
    /// evidence-lineage fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl EditorInlineCertExportParity {
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
pub struct EditorInlineAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: EditorInlineCertificationAxis,
    /// The certification state of the axis.
    pub state: EditorInlineAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5EditorInlineDowngradeTrigger>,
}

impl EditorInlineAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible
    ///   trigger (that is exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            EditorInlineAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            EditorInlineAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            EditorInlineAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current. Present iff
/// the certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: EditorInlineCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5EditorInlineComponentClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5EditorInlineComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 editor / review / AI inline profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineProfileCertificationRow {
    /// Record kind; must equal [`EDITOR_INLINE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EDITOR_INLINE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5EditorInlineCertifiedProfile,
    /// The inline claim ceiling the profile asserts.
    pub claimed_claim: M5EditorInlineComponentClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5EditorInlineComponentClaim,
    /// The frozen component families this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5EditorInlineComponentFamily>,
    /// One outcome per [`EditorInlineCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<EditorInlineAxisOutcome>,
    /// The B133 guardrails; all must hold.
    pub guardrails: EditorInlineCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<EditorInlineClaimAutoNarrow>,
    /// The one canonical editor-inline proof bundle this profile cites. Must equal
    /// [`EDITOR_INLINE_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: EditorInlineProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: EditorInlineCertExportParity,
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

impl EditorInlineProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: EditorInlineCertificationAxis) -> Option<&EditorInlineAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<EditorInlineCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && EditorInlineCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(EditorInlineAxisOutcome::well_formed)
    }

    /// True when the profile narrows its inline claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<EditorInlineCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == EditorInlineAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, guardrails, and claim narrowing. This is the
    /// heart of the capstone: a degraded axis must produce a visible claim narrowing, only a
    /// live first-party profile may certify a trusted inline result, every guardrail must hold,
    /// CLI/export parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> EditorInlineProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != EDITOR_INLINE_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return EditorInlineProfileClaimStatus::Red;
        }

        // Every B133 guardrail must hold.
        if !self.guardrails.all_held() {
            return EditorInlineProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return EditorInlineProfileClaimStatus::Red;
        }

        // Only a live first-party profile may certify a trusted inline result.
        if self.certified_claim.asserts_trusted_inline_result()
            && !self.profile.is_live_trusted_inline()
        {
            return EditorInlineProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(EditorInlineCertificationAxis::CliExport) {
            Some(o) if o.state == EditorInlineAxisCertificationState::Certified => {}
            _ => return EditorInlineProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == EditorInlineAxisCertificationState::UndisclosedDrift)
        {
            return EditorInlineProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return EditorInlineProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return EditorInlineProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return EditorInlineProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return EditorInlineProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier profile's truth.
        if !narrowed.is_empty() {
            return EditorInlineProfileClaimStatus::Red;
        }

        EditorInlineProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == EDITOR_INLINE_CERT_ROW_RECORD_KIND
            && self.schema_version == EDITOR_INLINE_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1123 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineProfileCertificationSummary {
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

/// Constructor input for [`EditorInlineProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorInlineProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<EditorInlineProfileCertificationRow>,
}

/// Checked-in M05-1123 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<EditorInlineProfileCertificationRow>,
    pub summary: EditorInlineProfileCertificationSummary,
}

impl EditorInlineProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: EditorInlineProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: EDITOR_INLINE_CERT_SCHEMA_VERSION,
            record_kind: EDITOR_INLINE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: EditorInlineProfileCertificationSummary {
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
    pub fn represented_profiles(&self) -> BTreeSet<M5EditorInlineCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Component families rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5EditorInlineComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5EditorInlineCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen component family is certified on at least one profile — proof the
    /// full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5EditorInlineComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(EditorInlineCertificationAxis::CliExport)
                .is_some_and(|o| o.state == EditorInlineAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> EditorInlineProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == EditorInlineProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == EditorInlineProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == EditorInlineProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(EditorInlineProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        EditorInlineProfileCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == EDITOR_INLINE_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(EditorInlineProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<EditorInlineCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EDITOR_INLINE_CERT_SCHEMA_VERSION {
            violations.push(EditorInlineCertificationViolation::SchemaVersion {
                expected: EDITOR_INLINE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EDITOR_INLINE_CERT_RECORD_KIND {
            violations.push(EditorInlineCertificationViolation::RecordKind {
                expected: EDITOR_INLINE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(EditorInlineCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != EDITOR_INLINE_CERT_CANONICAL_BUNDLE_REF {
            violations.push(EditorInlineCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(EditorInlineCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(EditorInlineCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(EditorInlineCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(EditorInlineCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != EDITOR_INLINE_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    EditorInlineCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B133 guardrail must hold.
            if !row.guardrails.all_held() {
                violations.push(EditorInlineCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a live first-party profile may certify a trusted inline result.
            if row.certified_claim.asserts_trusted_inline_result()
                && !row.profile.is_live_trusted_inline()
            {
                violations.push(
                    EditorInlineCertificationViolation::NonLiveProfileClaimsTrustedResult {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(EditorInlineCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    EditorInlineCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    EditorInlineCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(EditorInlineCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == EditorInlineProfileClaimStatus::Red {
                violations.push(EditorInlineCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(EditorInlineCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen component family must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(EditorInlineCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(EditorInlineCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(EditorInlineCertificationViolation::RawInlineMaterialInExport);
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
        out.push_str("# M5 Editor-Inline Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5EditorInlineCertifiedProfile::ALL.len(),
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
pub fn current_m5_editor_inline_component_surface_certification_export(
) -> Result<EditorInlineProfileCertificationPacket, EditorInlineCertificationArtifactError> {
    let packet: EditorInlineProfileCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-editor-inline-component-surface-certification-proof/support_export.json"
    )))
    .map_err(EditorInlineCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(EditorInlineCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum EditorInlineCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<EditorInlineCertificationViolation>),
}

impl fmt::Display for EditorInlineCertificationArtifactError {
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

impl Error for EditorInlineCertificationArtifactError {}

/// Validation failure for M05-1123 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorInlineCertificationViolation {
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
    NonLiveProfileClaimsTrustedResult { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawInlineMaterialInExport,
}

impl fmt::Display for EditorInlineCertificationViolation {
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
                    "packet does not cite the canonical editor-inline proof bundle"
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
                    "row {id} does not cite the one canonical editor-inline proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B133 guardrail: tab/marker/diagnostic state by color alone, \
a silently-drifting comment anchor or AI evidence pointer, blurred outdated/resolved review state, \
an inferred fix presented as exact, or an evidence timeline hidden in an opaque log"
                )
            }
            Self::NonLiveProfileClaimsTrustedResult { id } => {
                write!(
                    f,
                    "row {id} certifies a trusted inline result on a non-live first-party profile"
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
claim, a guardrail broke, CLI/export parity dropped, a non-live profile claimed a trusted result, \
or the narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 editor-inline profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen editor-inline component family is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawInlineMaterialInExport => {
                write!(
                    f,
                    "export contains a raw editor buffer, diff body, comment payload, AI message body, or credential material"
                )
            }
        }
    }
}

impl Error for EditorInlineCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&EditorInlineAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != EditorInlineAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the
/// editor / review / AI generics the spec forbids collapsing distinct inline-state, anchor,
/// severity, fix-posture, confidence, approval, and evidence-lineage truth into (whole-label
/// matches so a full sentence naming a concrete state, anchor, or fix posture is not flagged).
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
            | "partial"
            | "cached"
            | "trusted"
            | "reviewable"
            | "editor tab"
            | "gutter"
            | "diagnostic"
            | "code action"
            | "diff"
            | "review thread"
            | "ai message"
            | "evidence timeline"
            | "anchor"
            | "severity"
            | "fix"
            | "fix posture"
            | "confidence"
            | "approval"
            | "evidence"
            | "evidence lineage"
            | "inferred"
            | "outdated"
            | "resolved"
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

/// Builds the canonical, checked-in M05-1123 certification packet. Certifies all eight claimed
/// M5 editor / review / AI profiles: two deliver their claim (green) and six auto-narrow a
/// not-current truth axis to a weaker inline ceiling (yellow). No profile hides drift or breaks
/// a guardrail (red).
pub fn seeded_m5_editor_inline_component_surface_certification_packet(
) -> EditorInlineProfileCertificationPacket {
    EditorInlineProfileCertificationPacket::new(EditorInlineProfileCertificationPacketInput {
        packet_id: "m5-editor-inline-component-surface-certification:stable:0001".to_owned(),
        as_of: "2026-07-12T00:00:00Z".to_owned(),
        matrix_ref: EDITOR_INLINE_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: EDITOR_INLINE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:editor-inline-component-certification:{id}"),
        EDITOR_INLINE_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> EditorInlineCertExportParity {
    EditorInlineCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: EditorInlineCertificationAxis) -> &'static str {
    match axis {
        EditorInlineCertificationAxis::Visual => {
            "inline state, anchor durability, severity / source, fix posture, confidence / source context, approval / resolution, and evidence lineage shown on-surface without color alone"
        }
        EditorInlineCertificationAxis::Keyboard => {
            "the same inline state, anchor, severity, fix posture, confidence, approval, evidence lineage, and bounded local actions are keyboard-reachable, never hover-only"
        }
        EditorInlineCertificationAxis::ScreenReader => {
            "the same inline / review / AI truth is announced non-visually, never color/motion/glyph-only"
        }
        EditorInlineCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at high zoom without clipping the state, anchor, severity, or fix posture"
        }
        EditorInlineCertificationAxis::ReducedMotion => {
            "the same truth stays legible and usable with reduced motion, never motion-only"
        }
        EditorInlineCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        EditorInlineCertificationAxis::DegradedState => {
            "a drifted anchor, stale severity, inferred fix, stale confidence, unverified approval, or partial evidence lineage honestly downgrades the TrustedInlineResult/ReviewableInlineResult claim rather than reading as a fresh authoritative inline result"
        }
        EditorInlineCertificationAxis::InlineComponentTruth => {
            "inline state, anchor durability, severity / source, fix posture, confidence / source context, approval / outdated-versus-resolved state, and evidence lineage stay explicit and never collapse into generic chrome, encode state by color alone, let anchors or evidence pointers drift, blur outdated and resolved state, present an inferred fix as exact, or hide an evidence timeline in an opaque log"
        }
    }
}

fn seed_certified(axis: EditorInlineCertificationAxis) -> EditorInlineAxisOutcome {
    EditorInlineAxisOutcome {
        axis,
        state: EditorInlineAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: EditorInlineCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5EditorInlineDowngradeTrigger,
) -> EditorInlineAxisOutcome {
    EditorInlineAxisOutcome {
        axis,
        state: EditorInlineAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<EditorInlineAxisOutcome> {
    EditorInlineCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: EditorInlineCertificationAxis,
    outcome: EditorInlineAxisOutcome,
) -> Vec<EditorInlineAxisOutcome> {
    EditorInlineCertificationAxis::ALL
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
    profile: M5EditorInlineCertifiedProfile,
    claimed_claim: M5EditorInlineComponentClaim,
    certified_claim: M5EditorInlineComponentClaim,
    consumed_families: &[M5EditorInlineComponentFamily],
    axis_outcomes: Vec<EditorInlineAxisOutcome>,
    claim_auto_narrow: Option<EditorInlineClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> EditorInlineProfileCertificationRow {
    let mut row = EditorInlineProfileCertificationRow {
        record_kind: EDITOR_INLINE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: EDITOR_INLINE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: EditorInlineCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: EDITOR_INLINE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: EditorInlineProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            EDITOR_INLINE_CERT_MATRIX_REF.to_owned(),
            EDITOR_INLINE_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-12T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: EditorInlineCertificationAxis,
    from_claim: M5EditorInlineComponentClaim,
    to_claim: M5EditorInlineComponentClaim,
    label: &str,
) -> EditorInlineClaimAutoNarrow {
    EditorInlineClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<EditorInlineProfileCertificationRow> {
    use EditorInlineCertificationAxis as Ax;
    use M5EditorInlineCertifiedProfile as P;
    use M5EditorInlineComponentClaim::*;
    use M5EditorInlineComponentFamily::*;
    use M5EditorInlineDowngradeTrigger as Trig;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:live-trusted-inline-surface",
            P::LiveTrustedInlineSurface,
            TrustedInlineResult,
            TrustedInlineResult,
            &[EditorTab, CodeActionChip],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "inline_state"],
            &[
                "editor tab names the open-document context and its modified / read-only / preview / shared / generated / remote item state without relying on color alone",
                "code-action chip names an exact, durably-anchored, apply-ready fix rather than an inferred change",
                "keyboard / screen-reader / high-zoom / reduced-motion reach preserved for the tab and the chip",
                "inline-component-truth: a live first-party inline surface is the only profile that certifies a trusted inline result",
            ],
        ),
        seed_row(
            "cert:reviewable-inline-structure",
            P::ReviewableInlineStructure,
            ReviewableInlineResult,
            ReviewableInlineResult,
            &[DiffView, Gutter],
            seed_all_certified(),
            None,
            &["profile", "claimed_claim", "certified_claim", "status", "change_kind"],
            &[
                "diff view names added / removed / modified / moved / conflicted change kinds without collapsing them",
                "gutter layers breakpoint, change-marker, and fold markers next to the code without relying on color alone",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable structure",
                "inline-component-truth: a reviewable read-only diff / gutter never certifies a live trusted apply claim",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:drifted-anchor-surface",
            P::DriftedAnchorSurface,
            ReviewableInlineResult,
            AnchorUnverifiedProjection,
            &[DiagnosticDecoration, ReviewThread],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the comment / diagnostic anchor durability signal is stale so a durably-anchored result cannot be certified",
                    "The comment / diagnostic anchor durability signal is stale, so the ReviewableInlineResult claim narrows to an anchor-unverified projection and the surface preserves its last-known anchor identity rather than presenting a drifted anchor as a durably-anchored result",
                    Trig::AnchorStateUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableInlineResult,
                AnchorUnverifiedProjection,
                "Anchor unverified: the durability signal is stale so the last-known anchor identity is preserved and the surface never reads as durably anchored",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "diagnostic decoration preserves its last-known anchored range and marks the anchor as unverified rather than presenting a drifted anchor as durable",
                "review thread keeps its comment identity while the anchor durability is disclosed as unverified",
                "degraded-state: ReviewableInlineResult narrows to an anchor-unverified projection (auto-narrowed)",
                "inline-component-truth: a drifted comment / diagnostic anchor never silently reads as durably anchored",
            ],
        ),
        seed_row(
            "cert:stale-severity-decoration",
            P::StaleSeverityDecoration,
            ReviewableInlineResult,
            SeverityUnverifiedProjection,
            &[DiagnosticDecoration, Gutter],
            seed_certified_except(
                Ax::InlineComponentTruth,
                seed_narrowed(
                    Ax::InlineComponentTruth,
                    "the diagnostic severity / source attribution and freshness are stale so a freshly-verified diagnostic cannot be certified",
                    "The diagnostic severity / source attribution is stale, so the ReviewableInlineResult claim narrows to a severity-unverified projection and the decoration preserves its last-known severity rather than presenting a stale problem as a freshly-verified diagnostic",
                    Trig::DiagnosticFreshnessUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::InlineComponentTruth,
                ReviewableInlineResult,
                SeverityUnverifiedProjection,
                "Severity unverified: the severity / source freshness is stale so the last-known severity is preserved and the decoration never reads as freshly verified",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "diagnostic decoration keeps its last-known severity visible and marks the source / freshness as unverified rather than showing a stale problem as fresh",
                "gutter keeps its marker layering while the diagnostic severity is disclosed as unverified",
                "inline-component-truth: ReviewableInlineResult narrows to a severity-unverified projection (auto-narrowed)",
                "inline-component-truth: a stale diagnostic severity never reads as a freshly-verified problem",
            ],
        ),
        seed_row(
            "cert:inferred-fix-chip",
            P::InferredFixChip,
            ReviewableInlineResult,
            FixPostureUnverifiedProjection,
            &[CodeActionChip],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the code-action fix posture is only inferred so an exact, safe-to-apply change cannot be certified",
                    "The code-action fix is only inferred, so the ReviewableInlineResult claim narrows to a fix-posture-unverified projection and the chip names it an inferred fix rather than presenting an inferred change as an exact, safe-to-apply edit",
                    Trig::InferredFixShownAsExact,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableInlineResult,
                FixPostureUnverifiedProjection,
                "Fix posture inferred: the change is only inferred so the chip names it an inferred fix and never reads as an exact, safe-to-apply edit",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "code-action chip names the fix as inferred and marks it not-safe-to-apply rather than presenting it as an exact change",
                "the chip keeps its applied / reverted / blocked state explicit while the fix posture is disclosed as inferred",
                "degraded-state: ReviewableInlineResult narrows to a fix-posture-unverified projection (auto-narrowed)",
                "inline-component-truth: an inferred fix never masquerades as an exact, safe-to-apply change",
            ],
        ),
        seed_row(
            "cert:stale-confidence-message",
            P::StaleConfidenceMessage,
            ReviewableInlineResult,
            ConfidenceUnverifiedProjection,
            &[AiMessageCard],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the AI message confidence / source context is stale so a fully-verified answer cannot be certified",
                    "The AI message confidence / source context is stale, so the ReviewableInlineResult claim narrows to a confidence-unverified projection and the card discloses its last-known confidence rather than presenting a stale answer as fully verified",
                    Trig::AiConfidenceUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableInlineResult,
                ConfidenceUnverifiedProjection,
                "Confidence unverified: the confidence / source context is stale so the last-known confidence is disclosed and the card never reads as fully verified",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "AI message card discloses its last-known confidence and source context and marks the answer as confidence-unverified rather than presenting it as fully verified",
                "the card keeps its available-actions truth while the confidence is disclosed as stale",
                "degraded-state: ReviewableInlineResult narrows to a confidence-unverified projection (auto-narrowed)",
                "inline-component-truth: a stale AI confidence never reads as a fully-verified answer",
            ],
        ),
        seed_row(
            "cert:unverified-approval-thread",
            P::UnverifiedApprovalThread,
            ReviewableInlineResult,
            ApprovalUnverifiedProjection,
            &[ReviewThread],
            seed_certified_except(
                Ax::InlineComponentTruth,
                seed_narrowed(
                    Ax::InlineComponentTruth,
                    "the review approval / outdated-versus-resolved state is unverified so a resolved, approved review cannot be certified",
                    "The review approval / outdated-versus-resolved state is unverified, so the ReviewableInlineResult claim narrows to an approval-unverified projection and the thread keeps its last-known state rather than blurring outdated and resolved into a settled, approved review",
                    Trig::OutdatedAndResolvedBlurred,
                ),
            ),
            Some(seed_narrow(
                Ax::InlineComponentTruth,
                ReviewableInlineResult,
                ApprovalUnverifiedProjection,
                "Approval unverified: the outdated-versus-resolved state is unverified so the last-known thread state is kept and the thread never reads as a resolved, approved review",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "review thread keeps its last-known thread state and marks the approval as unverified rather than presenting an outdated thread as resolved",
                "the thread keeps its comment-anchor durability while the approval state is disclosed as unverified",
                "inline-component-truth: ReviewableInlineResult narrows to an approval-unverified projection (auto-narrowed)",
                "inline-component-truth: outdated and resolved review state are never blurred together",
            ],
        ),
        seed_row(
            "cert:partial-evidence-timeline",
            P::PartialEvidenceTimeline,
            ReviewableInlineResult,
            EvidenceLineageProjection,
            &[EvidenceTimeline],
            seed_certified_except(
                Ax::InlineComponentTruth,
                seed_narrowed(
                    Ax::InlineComponentTruth,
                    "the evidence lineage is only partial / redacted so a fully-captured evidence trail cannot be certified",
                    "The evidence timeline lineage is only partial / redacted, so the ReviewableInlineResult claim narrows to an evidence-lineage projection and the timeline discloses the partial / redacted lineage in an inspectable structure rather than hiding it in an opaque log",
                    Trig::EvidenceTimelineOpaqueLog,
                ),
            ),
            Some(seed_narrow(
                Ax::InlineComponentTruth,
                ReviewableInlineResult,
                EvidenceLineageProjection,
                "Evidence lineage partial: the lineage is partial / redacted so the timeline discloses the partial lineage inspectably and never reads as a fully-captured trail",
            )),
            &["profile", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "evidence timeline discloses the partial / redacted lineage in an inspectable, collapsible structure rather than an opaque log",
                "the timeline keeps its export-safe evidence identity while the lineage is disclosed as partial",
                "inline-component-truth: ReviewableInlineResult narrows to an evidence-lineage projection (auto-narrowed)",
                "inline-component-truth: a partial / redacted evidence timeline is never hidden in an opaque log",
            ],
        ),
    ]
}
