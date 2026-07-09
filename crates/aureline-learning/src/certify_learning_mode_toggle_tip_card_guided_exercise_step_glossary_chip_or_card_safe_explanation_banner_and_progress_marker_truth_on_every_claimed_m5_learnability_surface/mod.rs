//! M05-1011 closing surface certification over the frozen M5 learning-mode-toggle / tip-card /
//! guided-exercise-step / glossary-chip-or-card / safe-explanation-banner / progress-marker
//! component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix`])
//! defines the six reusable learning-mode-toggle, tip-card, guided-exercise-step,
//! glossary-chip-or-card, safe-explanation-banner, and progress-marker components, the M05-1005..1007
//! primitive lanes narrow each one, the M05-1009 consumer lane
//! ([`crate::add_shared_onboarding_migration_contextual_help_docs_browser_feature_family_tour_companion_handoff_and_support_export_consumers_so_learning_components_keep_citation_privacy_and_progress_language_aligned_across_claimed_m5_profiles`])
//! proves they are reusable across the claimed onboarding / migration / contextual-help /
//! docs-browser / tour / companion-handoff / support-export consumers, and the M05-1010
//! accessibility / auto-narrowing capstone
//! ([`crate::implement_keyboard_screen_reader_localization_export_parity_and_automatic_narrowing_when_citations_are_stale_pack_freshness_drifted_progress_portability_is_blocked_or_explain_do_boundaries_cannot_be_proven_across_claimed_m5_learning_components`])
//! certifies keyboard / screen-reader / localization / export parity per family, this closing
//! capstone *certifies* that the shared learning-component truth holds on every claimed M5
//! learnability surface — and auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user actually learns on (the first-run onboarding
//! flow, the feature-family tour, the docs / glossary browser, the support / export bundle, the
//! guided-exercise practice surface, the contextual-help surface, the educational-AI companion, and
//! the CLI / headless surface), not on component family or primitive lane. Each
//! [`LearningSurfaceCertificationRow`] certifies one surface across six truth axes — visual,
//! keyboard, screen-reader, export, degraded-state, and learning-boundary provenance — and either
//! passes (green), auto-narrows its learning claim to the weakest supported ceiling (yellow), or is
//! blocked (red) when a degraded axis is hidden behind a full-truth claim inherited from a healthier
//! learning lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps an `ExactLearning` / `ReviewableGuidance` claim while one of its truth axes
//! is not current — the glossary citation is stale, the exercise pack has drifted, the
//! explain-versus-do boundary cannot be proven, or progress portability is blocked — is
//! over-claiming and blocks; a surface that discloses the reduction by narrowing its learning claim
//! (with a bound reason and a frozen downgrade trigger) is honestly yellow. Learning truth never
//! loses lineage: a narrowed surface always preserves its cited-source / command-binding /
//! progress-ownership / explain-versus-do lineage continuity rather than dropping it between a tip,
//! a glossary chip, and an exported progress record. The always-on export axis must always stay
//! certified, so support and automation can reconstruct the same learning-mode / tip / exercise /
//! citation / explanation / progress truth from the same component identity the user saw. No
//! certified surface may widen trust or mutating authority: learnability stays opt-in,
//! citation-backed, command-backed, and privacy-bounded, and explain stays separate from do.
//!
//! Every row cites exactly one canonical learning-component proof bundle
//! ([`LEARNING_CERT_CANONICAL_BUNDLE_REF`]) — the frozen component matrix release proof — rather
//! than cloning per-surface evidence. The packet is metadata-only: raw learning copy, captured
//! glossary bodies, exercise payloads, imported progress state, and credentials never cross this
//! boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-learning-component-certification.schema.json`](../../../../schemas/ui/m5-learning-component-certification.schema.json).
//! The contract doc is
//! [`docs/help/m5_learning_component_certification_contract.md`](../../../../docs/help/m5_learning_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_onboarding_migration_contextual_help_docs_browser_feature_family_tour_companion_handoff_and_support_export_consumers_so_learning_components_keep_citation_privacy_and_progress_language_aligned_across_claimed_m5_profiles as consumers;
use crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_localization_export_parity_and_automatic_narrowing_when_citations_are_stale_pack_freshness_drifted_progress_portability_is_blocked_or_explain_do_boundaries_cannot_be_proven_across_claimed_m5_learning_components as a11y;
use a11y::M5LearningComponentClaim;
use matrix::{M5LearningComponentFamily, M5LearningDowngradeTrigger};

/// Schema version stamped on the M05-1011 certification packet.
pub const LEARNING_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`LearningSurfaceCertificationPacket`].
pub const LEARNING_CERT_RECORD_KIND: &str = "m5_learning_component_certification_packet";

/// Stable record-kind tag carried by each [`LearningSurfaceCertificationRow`].
pub const LEARNING_CERT_ROW_RECORD_KIND: &str = "m5_learning_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const LEARNING_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-learning-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const LEARNING_CERT_DOC_REF: &str = "docs/help/m5_learning_component_certification_contract.md";

/// Repo-relative path of the frozen learning-component matrix schema the certified surfaces render.
pub const LEARNING_CERT_MATRIX_REF: &str = matrix::M5_LEARNING_COMPONENT_SCHEMA_REF;

/// The one canonical learning-component proof bundle every certified surface cites as its
/// first-resolved component truth. All eight surfaces point back to it rather than cloning
/// per-surface evidence.
pub const LEARNING_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_LEARNING_COMPONENT_ARTIFACT_REF;

/// The M05-1009 consumer-adoption support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const LEARNING_CERT_CONSUMER_BUNDLE_REF: &str =
    consumers::M5_LEARNING_COMPONENT_CONSUMER_ARTIFACT_REF;

/// The M05-1010 accessibility / auto-narrowing support export whose keyboard / screen-reader /
/// localization / export parity this capstone builds on. Recorded as a supporting evidence ref on
/// every row.
pub const LEARNING_CERT_A11Y_BUNDLE_REF: &str = a11y::LEARNING_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const LEARNING_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-learning-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const LEARNING_CERT_CSV_REF: &str =
    "artifacts/release/m5-learning-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const LEARNING_CERT_REPORT_REF: &str =
    "artifacts/release/m5-learning-component-certification/report.md";

/// The eight claimed M5 learnability surfaces this capstone certifies. Keyed on the surface a user
/// actually learns on, not on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningCertifiedSurface {
    /// The first-run onboarding flow surface.
    FirstRunOnboarding,
    /// The feature-family tour surface.
    FeatureFamilyTour,
    /// The docs / glossary browser surface.
    DocsGlossaryBrowser,
    /// The support / export bundle surface.
    SupportExport,
    /// The guided-exercise practice surface.
    GuidedExercisePractice,
    /// The contextual-help surface.
    ContextualHelp,
    /// The educational-AI companion surface.
    EducationalAiCompanion,
    /// The CLI / headless surface.
    CliHeadless,
}

impl M5LearningCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5LearningCertifiedSurface; 8] = [
        M5LearningCertifiedSurface::FirstRunOnboarding,
        M5LearningCertifiedSurface::FeatureFamilyTour,
        M5LearningCertifiedSurface::DocsGlossaryBrowser,
        M5LearningCertifiedSurface::SupportExport,
        M5LearningCertifiedSurface::GuidedExercisePractice,
        M5LearningCertifiedSurface::ContextualHelp,
        M5LearningCertifiedSurface::EducationalAiCompanion,
        M5LearningCertifiedSurface::CliHeadless,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstRunOnboarding => "first_run_onboarding",
            Self::FeatureFamilyTour => "feature_family_tour",
            Self::DocsGlossaryBrowser => "docs_glossary_browser",
            Self::SupportExport => "support_export",
            Self::GuidedExercisePractice => "guided_exercise_practice",
            Self::ContextualHelp => "contextual_help",
            Self::EducationalAiCompanion => "educational_ai_companion",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity dimensions the
/// spec requires verifying — visual, keyboard, screen-reader, export, degraded-state, and
/// learning-boundary provenance. The export axis is always-on and must stay certified for every
/// surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningCertificationAxis {
    /// Visual parity: the learning-mode / tip / exercise state, cited source, source-class
    /// freshness, explain-versus-do boundary, and progress ownership are shown on the primary
    /// surface.
    Visual,
    /// Keyboard-reach parity: the same learning / citation / progress truth and its actions are
    /// reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color or a
    /// status glyph alone.
    ScreenReader,
    /// Export parity (always-on): the certified surface state is reconstructable as text / JSON /
    /// Markdown for support and automation, from the same component identity.
    Export,
    /// Degraded-state parity: a stale glossary citation, a drifted exercise pack, an unprovable
    /// explain-versus-do boundary, or blocked progress portability honestly downgrades an
    /// `ExactLearning` / `ReviewableGuidance` claim to a weaker learning tier.
    DegradedState,
    /// Learning-boundary provenance parity: the cited source, source-class freshness,
    /// explain-versus-do boundary, and progress ownership stay explicit before any explain,
    /// exercise, or apply — never inheriting a healthier lane's learning truth, never masking a
    /// stale citation, drifted pack, unprovable boundary, or blocked progress as an exact-learning
    /// surface, never widening trust or mutating authority, and never dropping cited-source /
    /// command-binding / progress-ownership / explain-versus-do lineage between a tip, a glossary
    /// chip, and an exported progress record.
    LearningBoundaryProvenance,
}

impl LearningCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [LearningCertificationAxis; 6] = [
        LearningCertificationAxis::Visual,
        LearningCertificationAxis::Keyboard,
        LearningCertificationAxis::ScreenReader,
        LearningCertificationAxis::Export,
        LearningCertificationAxis::DegradedState,
        LearningCertificationAxis::LearningBoundaryProvenance,
    ];

    /// The always-on export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::Export)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::Export => "export",
            Self::DegradedState => "degraded_state",
            Self::LearningBoundaryProvenance => "learning_boundary_provenance",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim inherited from
    /// a healthier surface.
    UndisclosedDrift,
}

impl LearningAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole surface. Never asserted by the author — always
/// recomputed from the axis outcomes and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearningSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed learning tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, export parity drops, lineage is dropped,
    /// authority is widened, or the narrowing is inconsistent.
    Red,
}

impl LearningSurfaceClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the surface is certifiable as shipped (green or disclosed yellow); red surfaces
    /// block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The copy / export parity a certified surface preserves. The export axis certifies only when this
/// offers text / JSON / Markdown reconstruction and prohibits a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The learning-mode / tip / exercise / citation / explanation / progress fields the surface
    /// preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl LearningCertExportParity {
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

/// One axis outcome on one certified surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: LearningCertificationAxis,
    /// The certification state of the axis.
    pub state: LearningAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5LearningDowngradeTrigger>,
}

impl LearningAxisOutcome {
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
            LearningAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            LearningAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            LearningAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a surface applies when a truth axis is not current. Present iff the
/// certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: LearningCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5LearningComponentClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5LearningComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
    /// True when the narrowed surface still preserves its cited-source / command-binding /
    /// progress-ownership / explain-versus-do lineage continuity rather than dropping it between a
    /// tip, a glossary chip, and an exported progress record.
    pub preserves_lineage_continuity: bool,
}

/// One certified M5 learnability surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningSurfaceCertificationRow {
    /// Record kind; must equal [`LEARNING_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`LEARNING_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5LearningCertifiedSurface,
    /// The learning-claim ceiling the surface asserts.
    pub claimed_claim: M5LearningComponentClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5LearningComponentClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5LearningComponentFamily>,
    /// One outcome per [`LearningCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<LearningAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<LearningClaimAutoNarrow>,
    /// True when this surface never drops its cited-source / command-binding / progress-ownership /
    /// explain-versus-do lineage continuity between a tip, a glossary chip, and an exported progress
    /// record.
    pub lineage_preserved: bool,
    /// True iff this surface widens trust or mutating authority beyond what the frozen learning
    /// component allows. A certified surface MUST keep this false: learnability stays opt-in,
    /// citation-backed, command-backed, and privacy-bounded, and explain stays separate from do.
    pub widens_learning_authority: bool,
    /// The one canonical learning proof bundle this surface cites. Must equal
    /// [`LEARNING_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: LearningSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: LearningCertExportParity,
    /// The compatibility notes captured for this surface.
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

impl LearningSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: LearningCertificationAxis) -> Option<&LearningAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<LearningCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && LearningCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(LearningAxisOutcome::well_formed)
    }

    /// True when the surface narrows its learning claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<LearningCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == LearningAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether a narrowed surface preserves its cited-source / command-binding / progress-ownership
    /// / explain-versus-do lineage continuity rather than dropping it. A non-narrowed surface
    /// trivially preserves lineage; a narrowed one must say so.
    pub fn preserves_lineage_continuity(&self) -> bool {
        match &self.claim_auto_narrow {
            Some(narrow) => self.lineage_preserved && narrow.preserves_lineage_continuity,
            None => self.lineage_preserved,
        }
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, export parity must always
    /// certify, learning truth must never drop lineage or widen authority, and the narrowing must
    /// be consistent.
    pub fn derive_status(&self) -> LearningSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != LEARNING_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.preserves_lineage_continuity()
            || self.widens_learning_authority
        {
            return LearningSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return LearningSurfaceClaimStatus::Red;
        }

        // The always-on export axis must stay certified.
        match self.axis(LearningCertificationAxis::Export) {
            Some(o) if o.state == LearningAxisCertificationState::Certified => {}
            _ => return LearningSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == LearningAxisCertificationState::UndisclosedDrift)
        {
            return LearningSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return LearningSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return LearningSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                    || !narrow.preserves_lineage_continuity
                {
                    return LearningSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return LearningSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return LearningSurfaceClaimStatus::Red;
        }

        LearningSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == LEARNING_CERT_ROW_RECORD_KIND
            && self.schema_version == LEARNING_CERT_SCHEMA_VERSION
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
            "surface={surface} claimed={claimed} certified={certified} status={status} \
narrowed_axes={narrowed} lineage_preserved={preserved} widens_authority={widens}",
            surface = self.surface.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
            preserved = self.lineage_preserved,
            widens = self.widens_learning_authority,
        )
    }
}

/// Rolled-up summary of an M05-1011 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningSurfaceCertificationSummary {
    pub row_count: usize,
    pub surface_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_surfaces_present: bool,
    pub all_families_covered: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub every_axis_covered_on_every_row: bool,
    pub all_lineage_preserved: bool,
    pub no_surface_widens_authority: bool,
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`LearningSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearningSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<LearningSurfaceCertificationRow>,
}

/// Checked-in M05-1011 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearningSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<LearningSurfaceCertificationRow>,
    pub summary: LearningSurfaceCertificationSummary,
}

impl LearningSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: LearningSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: LEARNING_CERT_SCHEMA_VERSION,
            record_kind: LEARNING_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: LearningSurfaceCertificationSummary {
                row_count: 0,
                surface_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_surfaces_present: false,
                all_families_covered: false,
                all_rows_publishable: false,
                all_status_fresh: false,
                all_rows_cite_canonical_bundle: false,
                all_rows_export_parity_certified: false,
                every_axis_covered_on_every_row: false,
                all_lineage_preserved: false,
                no_surface_widens_authority: false,
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5LearningCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5LearningComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5LearningCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5LearningComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether an export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(LearningCertificationAxis::Export)
                .is_some_and(|o| o.state == LearningAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> LearningSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == LearningSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == LearningSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == LearningSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(LearningSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_preserved = self
            .rows
            .iter()
            .all(LearningSurfaceCertificationRow::preserves_lineage_continuity);
        let no_widen = self.rows.iter().all(|r| !r.widens_learning_authority);

        LearningSurfaceCertificationSummary {
            row_count: self.rows.len(),
            surface_count: surfaces.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_surfaces_present: all_surfaces,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == LEARNING_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(LearningSurfaceCertificationRow::covers_all_axes),
            all_lineage_preserved: all_preserved,
            no_surface_widens_authority: no_widen,
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable
                && all_fresh
                && all_surfaces
                && all_families
                && all_preserved
                && no_widen,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<LearningCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != LEARNING_CERT_SCHEMA_VERSION {
            violations.push(LearningCertificationViolation::SchemaVersion {
                expected: LEARNING_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != LEARNING_CERT_RECORD_KIND {
            violations.push(LearningCertificationViolation::RecordKind {
                expected: LEARNING_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(LearningCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != LEARNING_CERT_CANONICAL_BUNDLE_REF {
            violations.push(LearningCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(LearningCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(LearningCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(LearningCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(LearningCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != LEARNING_CERT_CANONICAL_BUNDLE_REF {
                violations.push(LearningCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // Export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(LearningCertificationAxis::Export)
                    .is_none_or_state_not_certified()
            {
                violations.push(LearningCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Learning truth must never drop lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(LearningCertificationViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // No certified surface may widen trust or mutating authority.
            if row.widens_learning_authority {
                violations.push(LearningCertificationViolation::LearningAuthorityWidened {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(LearningCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(LearningCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == LearningSurfaceClaimStatus::Red {
                violations.push(LearningCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(LearningCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(LearningCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(LearningCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(LearningCertificationViolation::RawLearningMaterialInExport);
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
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,binding_axis,lineage_preserved,widens_authority\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{surface},{claimed},{certified},{status},{narrowed},{binding},{preserved},{widens}\n",
                id = row.row_id,
                surface = row.surface.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
                preserved = row.lineage_preserved,
                widens = row.widens_learning_authority,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Learning Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5LearningCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Lineage preserved on every surface: {}\n",
            self.summary.all_lineage_preserved
        ));
        out.push_str(&format!(
            "- No surface widens authority: {}\n",
            self.summary.no_surface_widens_authority
        ));
        out.push_str(&format!(
            "- Auto-narrowed surfaces: {}\n",
            self.summary.narrowed_surface_count,
        ));
        out.push_str(&format!("- Report clean: {}\n", self.summary.report_clean));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in certification export.
pub fn current_m5_learning_component_certification_export(
) -> Result<LearningSurfaceCertificationPacket, LearningCertificationArtifactError> {
    let packet: LearningSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-learning-component-certification/support_export.json"
    )))
    .map_err(LearningCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(LearningCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum LearningCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<LearningCertificationViolation>),
}

impl fmt::Display for LearningCertificationArtifactError {
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

impl Error for LearningCertificationArtifactError {}

/// Validation failure for M05-1011 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LearningCertificationViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    WrongCanonicalBundle,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    AxisCoverageIncomplete { id: String },
    MalformedAxisOutcome { id: String },
    RowMissingCanonicalBundle { id: String },
    ExportParityNotCertified { id: String },
    LineageDropped { id: String },
    LearningAuthorityWidened { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawLearningMaterialInExport,
}

impl fmt::Display for LearningCertificationViolation {
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
                    "packet does not cite the canonical learning-component proof bundle"
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
                    "row {id} does not cite the one canonical learning-component proof bundle"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::LineageDropped { id } => {
                write!(
                    f,
                    "row {id} drops cited-source / command-binding / progress-ownership / explain-versus-do lineage continuity (a narrowed surface must preserve its lineage between a tip, a glossary chip, and an exported progress record)"
                )
            }
            Self::LearningAuthorityWidened { id } => {
                write!(
                    f,
                    "row {id} widens trust or mutating authority beyond the frozen learning component (learnability must stay opt-in, citation-backed, command-backed, and privacy-bounded, with explain separate from do)"
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
            Self::SurfaceBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a full claim, \
export parity dropped, lineage was dropped, authority was widened, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 learnability surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen learning-component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawLearningMaterialInExport => {
                write!(f, "export contains raw learning material")
            }
        }
    }
}

impl Error for LearningCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&LearningAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != LearningAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure.
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
            | "fallback"
            | "reduced"
            | "stale"
            | "cached"
            | "unverified"
            | "offline"
            | "blocked"
            | "paused"
            | "snoozed"
            | "interrupted"
            | "incomplete"
            | "uncertain"
            | "partial"
            | "uncited"
            | "unprovable"
            | "not installed"
            | "not_installed"
            | "local only"
            | "local_only"
            | "no citation"
            | "no_citation"
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

/// Builds the canonical, checked-in M05-1011 certification packet. Certifies all eight claimed M5
/// learnability surfaces: four deliver their claim (green) and four auto-narrow a not-current truth
/// axis to a weaker learning ceiling (yellow). No surface hides drift (red), no surface widens
/// authority, and no surface drops cited-source / command-binding / progress-ownership /
/// explain-versus-do lineage.
pub fn seeded_m5_learning_component_certification_packet() -> LearningSurfaceCertificationPacket {
    LearningSurfaceCertificationPacket::new(LearningSurfaceCertificationPacketInput {
        packet_id: "m5-learning-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-09T00:00:00Z".to_owned(),
        matrix_ref: LEARNING_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: LEARNING_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:learning-component-certification:{id}"),
        LEARNING_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        LEARNING_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> LearningCertExportParity {
    LearningCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: LearningCertificationAxis) -> &'static str {
    match axis {
        LearningCertificationAxis::Visual => {
            "learning-mode/tip/exercise state, cited source, source-class freshness, explain-versus-do boundary, and progress ownership shown on-surface"
        }
        LearningCertificationAxis::Keyboard => {
            "the same learning/citation/progress truth and its actions are keyboard-reachable"
        }
        LearningCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only"
        }
        LearningCertificationAxis::Export => {
            "surface state exports as text / JSON / Markdown for support and automation from the same component identity"
        }
        LearningCertificationAxis::DegradedState => {
            "a stale glossary citation, a drifted exercise pack, an unprovable explain-versus-do boundary, or blocked progress portability honestly downgrades the ExactLearning/ReviewableGuidance claim"
        }
        LearningCertificationAxis::LearningBoundaryProvenance => {
            "cited source, source-class freshness, explain-versus-do boundary, and progress ownership stay explicit before any explain, exercise, or apply; the boundary never widens authority and never drops cited-source/command-binding/progress-ownership/explain-versus-do lineage"
        }
    }
}

fn seed_certified(axis: LearningCertificationAxis) -> LearningAxisOutcome {
    LearningAxisOutcome {
        axis,
        state: LearningAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: LearningCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5LearningDowngradeTrigger,
) -> LearningAxisOutcome {
    LearningAxisOutcome {
        axis,
        state: LearningAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<LearningAxisOutcome> {
    LearningCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: LearningCertificationAxis,
    outcome: LearningAxisOutcome,
) -> Vec<LearningAxisOutcome> {
    LearningCertificationAxis::ALL
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
    surface: M5LearningCertifiedSurface,
    claimed_claim: M5LearningComponentClaim,
    certified_claim: M5LearningComponentClaim,
    consumed_families: &[M5LearningComponentFamily],
    axis_outcomes: Vec<LearningAxisOutcome>,
    claim_auto_narrow: Option<LearningClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> LearningSurfaceCertificationRow {
    let mut row = LearningSurfaceCertificationRow {
        record_kind: LEARNING_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: LEARNING_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        lineage_preserved: true,
        widens_learning_authority: false,
        canonical_bundle_ref: LEARNING_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: LearningSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            LEARNING_CERT_MATRIX_REF.to_owned(),
            LEARNING_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-09T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: LearningCertificationAxis,
    from_claim: M5LearningComponentClaim,
    to_claim: M5LearningComponentClaim,
    label: &str,
) -> LearningClaimAutoNarrow {
    LearningClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
        preserves_lineage_continuity: true,
    }
}

fn seeded_rows() -> Vec<LearningSurfaceCertificationRow> {
    use LearningCertificationAxis as Ax;
    use M5LearningCertifiedSurface as S;
    use M5LearningComponentClaim::*;
    use M5LearningComponentFamily::*;
    use M5LearningDowngradeTrigger as Trig;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:first-run-onboarding",
            S::FirstRunOnboarding,
            ExactLearning,
            ExactLearning,
            &[LearningModeToggle, TipCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "command_binding"],
            &[
                "the learning-mode toggle keeps its learning state, scope, and stable command/docs deep link explicit before it enables in place",
                "the tip card keeps its why-now context and stable command binding explicit rather than re-surfacing a suppressed tip",
                "keyboard/screen-reader reach preserved for the learning-mode toggle and the tip card",
                "provenance: a first-run learning surface never toggles a mode or teaches a command it cannot name or run, and never widens authority",
            ],
        ),
        seed_row(
            "cert:feature-family-tour",
            S::FeatureFamilyTour,
            ExactLearning,
            ExactLearning,
            &[TipCard, LearningModeToggle],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "learning_mode_state"],
            &[
                "the tip card keeps its why-now relevance and concrete next action explicit while the tour runs",
                "the learning-mode toggle keeps its user/workspace scope and reset path explicit during the tour",
                "keyboard/screen-reader reach preserved for the tip card and the learning-mode toggle",
                "provenance: the tour never presents a mode or tip whose command it cannot resolve",
            ],
        ),
        seed_row(
            "cert:docs-glossary-browser",
            S::DocsGlossaryBrowser,
            ReviewableGuidance,
            ReviewableGuidance,
            &[GlossaryChipOrCard, ProgressMarker],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "cited_source"],
            &[
                "the glossary chip/card keeps its cited file/symbol/doc source and source-class freshness explicit and current",
                "the progress marker keeps its user-owned, default-local progress and resume/reset/export paths explicit",
                "keyboard/screen-reader reach preserved for the glossary chip/card and the progress marker",
                "provenance: the docs/glossary browser never severs a glossary citation and never shares progress beyond its owned scope",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            ReviewableGuidance,
            ReviewableGuidance,
            &[ProgressMarker, LearningModeToggle],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "progress_ownership"],
            &[
                "support export reconstructs learning-mode/tip/exercise/citation/explanation/progress truth from the same component identity",
                "the progress marker keeps its user-owned, privacy-bounded progress explicit in the exported record rather than leaking it",
                "the learning-mode toggle keeps its learning state and command binding explicit in the exported learning record",
                "provenance: a learning export never carries raw learning copy, captured glossary bodies, exercise payloads, or imported progress state",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:guided-exercise-practice",
            S::GuidedExercisePractice,
            ExactLearning,
            StalePackProjection,
            &[GuidedExerciseStep],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the exercise pack has drifted / gone stale and cannot claim a current guided-exercise step state",
                    "The guided-exercise practice surface resolves a drifted exercise pack, so the ExactLearning claim narrows to stale-pack-projection instead of implying the guided step matches a current pack",
                    Trig::ExerciseStepStateUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ExactLearning,
                StalePackProjection,
                "Exercise pack has drifted: the guided-exercise step shows its cached-pack state and offers reset/skip rather than implying the step matches a current exercise pack",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the guided-exercise step keeps its target object, success criteria, and stale-pack state explicit and offers hint/reveal/reset/skip",
                "the guided-exercise step keeps its sandbox-or-preview mutation preference explicit while the pack stays stale",
                "degraded-state: ExactLearning narrows to stale-pack-projection (auto-narrowed)",
                "known compatibility note: stale-pack behavior — a drifted exercise pack never reads as a current guided-exercise step",
            ],
        ),
        seed_row(
            "cert:contextual-help",
            S::ContextualHelp,
            ExactLearning,
            UncitedGlossaryProjection,
            &[GlossaryChipOrCard],
            seed_certified_except(
                Ax::LearningBoundaryProvenance,
                seed_narrowed(
                    Ax::LearningBoundaryProvenance,
                    "the glossary citation is stale / cannot be resolved and cannot claim a current cited-source glossary state",
                    "The contextual-help surface resolves a glossary chip whose citation is stale, so the ExactLearning claim narrows to uncited-glossary-projection instead of implying the definition is currently cited-source backed",
                    Trig::GlossaryCitationSevered,
                ),
            ),
            Some(seed_narrow(
                Ax::LearningBoundaryProvenance,
                ExactLearning,
                UncitedGlossaryProjection,
                "Glossary citation is stale: the glossary chip shows the definition is uncited/source-unresolved and offers a request-citation path rather than implying a current cited source",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the glossary chip keeps its uncited/stale-citation reason explicit rather than presenting a definition as currently source-backed",
                "the glossary chip keeps its source-class and open-source-when-available path explicit while the citation stays stale",
                "learning-boundary: ExactLearning narrows to uncited-glossary-projection (auto-narrowed)",
                "known compatibility note: uncited-glossary behavior — a stale glossary citation never reads as a currently cited-source definition",
            ],
        ),
        seed_row(
            "cert:educational-ai-companion",
            S::EducationalAiCompanion,
            ReviewableGuidance,
            UnprovableBoundaryProjection,
            &[SafeExplanationBanner],
            seed_certified_except(
                Ax::LearningBoundaryProvenance,
                seed_narrowed(
                    Ax::LearningBoundaryProvenance,
                    "the explain-versus-do boundary cannot be proven for this explanation and cannot claim a provable no-hidden-apply guarantee",
                    "The educational-AI companion resolves a safe-explanation banner whose explain-versus-do boundary cannot be proven, so the ReviewableGuidance claim narrows to unprovable-boundary-projection instead of implying the explanation can safely act",
                    Trig::ExplanationApplyBoundaryUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::LearningBoundaryProvenance,
                ReviewableGuidance,
                UnprovableBoundaryProjection,
                "Explain-versus-do boundary unprovable: the safe-explanation banner stays explain-only and shows no apply action rather than implying it can mutate live state",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the safe-explanation banner keeps explain separate from do and offers no apply action while the boundary stays unprovable",
                "the safe-explanation banner keeps its cited source explicit and never mutates live state without the same preview/approval model as ordinary work",
                "learning-boundary: ReviewableGuidance narrows to unprovable-boundary-projection (auto-narrowed)",
                "known compatibility note: unprovable-boundary behavior — an explanation whose explain-versus-do boundary cannot be proven never reads as a safe live-apply surface",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            ExactLearning,
            BlockedProgressProjection,
            &[ProgressMarker],
            seed_certified_except(
                Ax::LearningBoundaryProvenance,
                seed_narrowed(
                    Ax::LearningBoundaryProvenance,
                    "progress portability is blocked in the headless context and cannot claim a current resume/export-ready progress state",
                    "The CLI-headless surface resolves a progress marker whose portability is blocked, so the ExactLearning claim narrows to blocked-progress-projection instead of implying progress can be resumed or exported here — its user-owned, default-local ownership stays preserved",
                    Trig::ProgressOwnershipUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::LearningBoundaryProvenance,
                ExactLearning,
                BlockedProgressProjection,
                "Progress portability blocked: the progress marker preserves its user-owned, default-local ownership and shows resume/export is unavailable here rather than implying portable progress",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the progress marker keeps its user-owned, default-local ownership explicit and honors the blocked-portability state rather than implying a syncable/exportable record",
                "the progress marker keeps its completed/remaining truth reachable in the headless export while portability stays blocked",
                "learning-boundary: ExactLearning narrows to blocked-progress-projection (auto-narrowed)",
                "known compatibility note: blocked-progress behavior — blocked progress portability never reads as resume/export-ready progress and never shares beyond its owned scope",
            ],
        ),
    ]
}
