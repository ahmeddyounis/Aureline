//! M05-931 surface certification over the frozen M5 contextual-tip-card / migration-bridge-card
//! / sequence-help-strip / why-unavailable-explanation-row / source-language-fallback component
//! matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix`])
//! defines the five reusable contextual-tip-card, migration-bridge-card, sequence-help-strip,
//! why-unavailable-explanation-row, and source-language-fallback components, the M05-925..928
//! primitive lanes narrow each one, the M05-929 consumer lane
//! ([`crate::add_shared_onboarding_help_importer_keybinding_modal_command_doc_consumers_so_contextual_teaching_components_keep_mapping_enablement_source_language_truth_aligned_across_claimed_m5_profiles`])
//! proves they are reusable across the claimed onboarding / importer / keybinding / command-doc /
//! help / localized-support consumers, and the M05-930 accessibility / auto-narrowing capstone
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_tips_are_snoozed_bridges_are_partial_sequences_are_unsupported_or_fallback_content_is_stale_across_claimed_m5_teaching_components`])
//! certifies keyboard / screen-reader / CLI / export parity per family, this closing capstone
//! *certifies* that the shared contextual-teaching component truth holds on every claimed M5
//! onboarding, migration, and command-help surface — and auto-narrows any surface that cannot
//! sustain it.
//!
//! It is keyed on the claimed **surface** a user learns, switches, or hits a blocked state on
//! (the first-run onboarding flow, the migration importer review, the command palette / docs, the
//! keybinding help, the modal sequence overlay, the localized support packet, the support /
//! export bundle, and the CLI), not on component family or primitive lane. Each
//! [`TeachingSurfaceCertificationRow`] certifies one surface across six truth axes — visual,
//! keyboard, screen-reader, CLI/export, degraded-state, and teaching-boundary provenance — and
//! either passes (green), auto-narrows its teaching claim to the weakest supported ceiling
//! (yellow), or is blocked (red) when a degraded axis is hidden behind a full-truth claim
//! inherited from a healthier teaching lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps an `ExactTeaching` / `ReviewableGuidance` claim while one of its truth
//! axes is not current — the contextual tip is snoozed, the migration bridge is only partial, the
//! command sequence is unsupported in the current context, the localized fallback content is
//! stale, or the command-binding / migration-mapping / blocked-action / source-language boundary
//! is unstated — is over-claiming and blocks; a surface that discloses the reduction by narrowing
//! its teaching claim (with a bound reason and a frozen downgrade trigger) is honestly yellow.
//! Teaching truth never loses lineage: a narrowed surface always preserves its command-binding /
//! migration-mapping / blocked-action / source-language lineage continuity rather than dropping it
//! between an in-place tip, a migration bridge, and a localized help fallback. The always-on
//! CLI/export axis must always stay certified, so support and automation can reconstruct the same
//! tip-trigger / command-binding / migration-mapping / sequence-state / blocked-action /
//! source-language-citation truth from the same command identity the user saw.
//!
//! Every row cites exactly one canonical contextual-teaching component proof bundle
//! ([`TEACHING_CERT_CANONICAL_BUNDLE_REF`]) — the frozen component matrix release proof — rather
//! than cloning per-surface evidence. The packet is metadata-only: raw teaching copy, captured
//! source-language bodies, imported migration payloads, and credentials never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-contextual-teaching-component-certification.schema.json`](../../../../schemas/ui/m5-contextual-teaching-component-certification.schema.json).
//! The contract doc is
//! [`docs/help/m5_contextual_teaching_component_certification_contract.md`](../../../../docs/help/m5_contextual_teaching_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_onboarding_help_importer_keybinding_modal_command_doc_consumers_so_contextual_teaching_components_keep_mapping_enablement_source_language_truth_aligned_across_claimed_m5_profiles as consumers;
use crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_tips_are_snoozed_bridges_are_partial_sequences_are_unsupported_or_fallback_content_is_stale_across_claimed_m5_teaching_components as a11y;
use a11y::M5TeachingComponentClaim;
use matrix::{M5ContextualTeachingComponentFamily, M5TeachingDowngradeTrigger};

/// Schema version stamped on the M05-931 certification packet.
pub const TEACHING_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`TeachingSurfaceCertificationPacket`].
pub const TEACHING_CERT_RECORD_KIND: &str = "m5_contextual_teaching_component_certification_packet";

/// Stable record-kind tag carried by each [`TeachingSurfaceCertificationRow`].
pub const TEACHING_CERT_ROW_RECORD_KIND: &str =
    "m5_contextual_teaching_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const TEACHING_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-contextual-teaching-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const TEACHING_CERT_DOC_REF: &str =
    "docs/help/m5_contextual_teaching_component_certification_contract.md";

/// Repo-relative path of the frozen contextual-teaching component matrix schema the certified
/// surfaces render.
pub const TEACHING_CERT_MATRIX_REF: &str = matrix::M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF;

/// The one canonical contextual-teaching component proof bundle every certified surface cites as
/// its first-resolved component truth. All eight surfaces point back to it rather than cloning
/// per-surface evidence.
pub const TEACHING_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_CONTEXTUAL_TEACHING_COMPONENT_ARTIFACT_REF;

/// The M05-929 consumer-adoption support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const TEACHING_CERT_CONSUMER_BUNDLE_REF: &str =
    consumers::M5_TEACHING_COMPONENT_CONSUMER_ARTIFACT_REF;

/// The M05-930 accessibility / auto-narrowing support export whose keyboard / screen-reader /
/// CLI / export parity this capstone builds on. Recorded as a supporting evidence ref on every
/// row.
pub const TEACHING_CERT_A11Y_BUNDLE_REF: &str = a11y::TEACHING_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const TEACHING_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-contextual-teaching-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const TEACHING_CERT_CSV_REF: &str =
    "artifacts/release/m5-contextual-teaching-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const TEACHING_CERT_REPORT_REF: &str =
    "artifacts/release/m5-contextual-teaching-component-certification/report.md";

/// The eight claimed M5 onboarding / migration / command-help surfaces this capstone certifies.
/// Keyed on the surface a user actually learns, switches, or hits a blocked state on, not on the
/// reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContextualTeachingHelpCertifiedSurface {
    /// The first-run onboarding flow surface.
    FirstRunOnboarding,
    /// The migration importer review surface.
    MigrationImporterReview,
    /// The command palette / command-docs surface.
    CommandPaletteDocs,
    /// The keybinding / leader-overlay help surface.
    KeybindingHelp,
    /// The modal command-sequence overlay surface.
    ModalSequenceOverlay,
    /// The localized support / help packet surface.
    LocalizedSupport,
    /// The support / export bundle surface.
    SupportExport,
    /// The CLI / headless surface.
    CliHeadless,
}

impl M5ContextualTeachingHelpCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5ContextualTeachingHelpCertifiedSurface; 8] = [
        M5ContextualTeachingHelpCertifiedSurface::FirstRunOnboarding,
        M5ContextualTeachingHelpCertifiedSurface::MigrationImporterReview,
        M5ContextualTeachingHelpCertifiedSurface::CommandPaletteDocs,
        M5ContextualTeachingHelpCertifiedSurface::KeybindingHelp,
        M5ContextualTeachingHelpCertifiedSurface::ModalSequenceOverlay,
        M5ContextualTeachingHelpCertifiedSurface::LocalizedSupport,
        M5ContextualTeachingHelpCertifiedSurface::SupportExport,
        M5ContextualTeachingHelpCertifiedSurface::CliHeadless,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstRunOnboarding => "first_run_onboarding",
            Self::MigrationImporterReview => "migration_importer_review",
            Self::CommandPaletteDocs => "command_palette_docs",
            Self::KeybindingHelp => "keybinding_help",
            Self::ModalSequenceOverlay => "modal_sequence_overlay",
            Self::LocalizedSupport => "localized_support",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity dimensions
/// the spec requires verifying — visual, keyboard, screen-reader, CLI/export, degraded-state, and
/// teaching-boundary provenance. The CLI/export axis is always-on and must stay certified for
/// every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeachingCertificationAxis {
    /// Visual parity: the tip trigger, command binding, migration mapping, sequence state,
    /// blocked-action owner / reason / next-safe-action, and source-language citation are shown on
    /// the primary surface.
    Visual,
    /// Keyboard-reach parity: the same teaching / migration / blocked-action truth and its actions
    /// are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color or a
    /// status glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is reconstructable as text /
    /// JSON / Markdown for support and automation, from the same command identity.
    CliExport,
    /// Degraded-state parity: a snoozed tip, a partial migration bridge, an unsupported command
    /// sequence, or stale localized fallback content honestly downgrades an `ExactTeaching` /
    /// `ReviewableGuidance` claim to a weaker teaching tier.
    DegradedState,
    /// Teaching-boundary provenance parity: the tip trigger, command binding, migration mapping,
    /// sequence state, blocked-action owner / reason / next-safe-action, and source-language
    /// citation stay explicit before any teach, migrate, or blocked-action explanation — never
    /// inheriting a healthier lane's teaching truth, never masking a snoozed tip, partial bridge,
    /// unsupported sequence, stale fallback, or unstated boundary as an exact-teaching surface, and
    /// never dropping command-binding / migration-mapping / blocked-action / source-language
    /// lineage between an in-place tip, a migration bridge, and a localized help fallback.
    TeachingBoundaryProvenance,
}

impl TeachingCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [TeachingCertificationAxis; 6] = [
        TeachingCertificationAxis::Visual,
        TeachingCertificationAxis::Keyboard,
        TeachingCertificationAxis::ScreenReader,
        TeachingCertificationAxis::CliExport,
        TeachingCertificationAxis::DegradedState,
        TeachingCertificationAxis::TeachingBoundaryProvenance,
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
            Self::TeachingBoundaryProvenance => "teaching_boundary_provenance",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TeachingAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim inherited
    /// from a healthier surface.
    UndisclosedDrift,
}

impl TeachingAxisCertificationState {
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
pub enum TeachingSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed teaching tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, lineage is
    /// dropped, or the narrowing is inconsistent.
    Red,
}

impl TeachingSurfaceClaimStatus {
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

/// The copy / export parity a certified surface preserves. The CLI/export axis certifies only
/// when this offers text / JSON / Markdown reconstruction and prohibits a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The tip-trigger / command-binding / migration-mapping / sequence-state / blocked-action /
    /// source-language fields the surface preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl TeachingCertExportParity {
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

/// One axis outcome on one certified surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: TeachingCertificationAxis,
    /// The certification state of the axis.
    pub state: TeachingAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5TeachingDowngradeTrigger>,
}

impl TeachingAxisOutcome {
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
            TeachingAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            TeachingAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            TeachingAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a surface applies when a truth axis is not current. Present iff
/// the certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: TeachingCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5TeachingComponentClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5TeachingComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
    /// True when the narrowed surface still preserves its command-binding / migration-mapping /
    /// blocked-action / source-language lineage continuity rather than dropping it between an
    /// in-place tip, a migration bridge, and a localized help fallback.
    pub preserves_lineage_continuity: bool,
}

/// One certified M5 onboarding / migration / command-help surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingSurfaceCertificationRow {
    /// Record kind; must equal [`TEACHING_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`TEACHING_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5ContextualTeachingHelpCertifiedSurface,
    /// The teaching-claim ceiling the surface asserts.
    pub claimed_claim: M5TeachingComponentClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5TeachingComponentClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5ContextualTeachingComponentFamily>,
    /// One outcome per [`TeachingCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<TeachingAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<TeachingClaimAutoNarrow>,
    /// True when this surface never drops its command-binding / migration-mapping / blocked-action
    /// / source-language lineage continuity between an in-place tip, a migration bridge, and a
    /// localized help fallback.
    pub lineage_preserved: bool,
    /// The one canonical contextual-teaching proof bundle this surface cites. Must equal
    /// [`TEACHING_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: TeachingSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: TeachingCertExportParity,
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

impl TeachingSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: TeachingCertificationAxis) -> Option<&TeachingAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<TeachingCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && TeachingCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(TeachingAxisOutcome::well_formed)
    }

    /// True when the surface narrows its teaching claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<TeachingCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == TeachingAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether a narrowed surface preserves its command-binding / migration-mapping /
    /// blocked-action / source-language lineage continuity rather than dropping it. A non-narrowed
    /// surface trivially preserves lineage; a narrowed one must say so.
    pub fn preserves_lineage_continuity(&self) -> bool {
        match &self.claim_auto_narrow {
            Some(narrow) => self.lineage_preserved && narrow.preserves_lineage_continuity,
            None => self.lineage_preserved,
        }
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, CLI/export parity must
    /// always certify, teaching truth must never drop lineage, and the narrowing must be
    /// consistent.
    pub fn derive_status(&self) -> TeachingSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != TEACHING_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.preserves_lineage_continuity()
        {
            return TeachingSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return TeachingSurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(TeachingCertificationAxis::CliExport) {
            Some(o) if o.state == TeachingAxisCertificationState::Certified => {}
            _ => return TeachingSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == TeachingAxisCertificationState::UndisclosedDrift)
        {
            return TeachingSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return TeachingSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return TeachingSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                    || !narrow.preserves_lineage_continuity
                {
                    return TeachingSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return TeachingSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return TeachingSurfaceClaimStatus::Red;
        }

        TeachingSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == TEACHING_CERT_ROW_RECORD_KIND
            && self.schema_version == TEACHING_CERT_SCHEMA_VERSION
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
narrowed_axes={narrowed} lineage_preserved={preserved}",
            surface = self.surface.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
            preserved = self.lineage_preserved,
        )
    }
}

/// Rolled-up summary of an M05-931 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingSurfaceCertificationSummary {
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
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`TeachingSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TeachingSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<TeachingSurfaceCertificationRow>,
}

/// Checked-in M05-931 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TeachingSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<TeachingSurfaceCertificationRow>,
    pub summary: TeachingSurfaceCertificationSummary,
}

impl TeachingSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: TeachingSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: TEACHING_CERT_SCHEMA_VERSION,
            record_kind: TEACHING_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: TeachingSurfaceCertificationSummary {
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
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5ContextualTeachingHelpCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ContextualTeachingComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5ContextualTeachingHelpCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5ContextualTeachingComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(TeachingCertificationAxis::CliExport)
                .is_some_and(|o| o.state == TeachingAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> TeachingSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == TeachingSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == TeachingSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == TeachingSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(TeachingSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_preserved = self
            .rows
            .iter()
            .all(TeachingSurfaceCertificationRow::preserves_lineage_continuity);

        TeachingSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == TEACHING_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(TeachingSurfaceCertificationRow::covers_all_axes),
            all_lineage_preserved: all_preserved,
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable
                && all_fresh
                && all_surfaces
                && all_families
                && all_preserved,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<TeachingCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != TEACHING_CERT_SCHEMA_VERSION {
            violations.push(TeachingCertificationViolation::SchemaVersion {
                expected: TEACHING_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != TEACHING_CERT_RECORD_KIND {
            violations.push(TeachingCertificationViolation::RecordKind {
                expected: TEACHING_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(TeachingCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != TEACHING_CERT_CANONICAL_BUNDLE_REF {
            violations.push(TeachingCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(TeachingCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(TeachingCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(TeachingCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(TeachingCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != TEACHING_CERT_CANONICAL_BUNDLE_REF {
                violations.push(TeachingCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(TeachingCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(TeachingCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Teaching truth must never drop lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(TeachingCertificationViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(TeachingCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(TeachingCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == TeachingSurfaceClaimStatus::Red {
                violations.push(TeachingCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(TeachingCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(TeachingCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(TeachingCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(TeachingCertificationViolation::RawTeachingMaterialInExport);
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
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,binding_axis,lineage_preserved\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{surface},{claimed},{certified},{status},{narrowed},{binding},{preserved}\n",
                id = row.row_id,
                surface = row.surface.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
                preserved = row.lineage_preserved,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Contextual-Teaching Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5ContextualTeachingHelpCertifiedSurface::ALL.len(),
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
pub fn current_m5_contextual_teaching_component_certification_export(
) -> Result<TeachingSurfaceCertificationPacket, TeachingCertificationArtifactError> {
    let packet: TeachingSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-contextual-teaching-component-certification/support_export.json"
    )))
    .map_err(TeachingCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TeachingCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum TeachingCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TeachingCertificationViolation>),
}

impl fmt::Display for TeachingCertificationArtifactError {
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

impl Error for TeachingCertificationArtifactError {}

/// Validation failure for M05-931 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeachingCertificationViolation {
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
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawTeachingMaterialInExport,
}

impl fmt::Display for TeachingCertificationViolation {
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
                    "packet does not cite the canonical contextual-teaching component proof bundle"
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
                    "row {id} does not cite the one canonical contextual-teaching component proof bundle"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::LineageDropped { id } => {
                write!(
                    f,
                    "row {id} drops command-binding / migration-mapping / blocked-action / source-language lineage continuity (a narrowed surface must preserve its lineage between an in-place tip, a migration bridge, and a localized help fallback)"
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
CLI/export parity dropped, lineage was dropped, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 onboarding / migration / command-help surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen contextual-teaching component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawTeachingMaterialInExport => {
                write!(f, "export contains raw teaching material")
            }
        }
    }
}

impl Error for TeachingCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&TeachingAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != TeachingAxisCertificationState::Certified,
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
            | "interrupted"
            | "incomplete"
            | "uncertain"
            | "snoozed"
            | "partial"
            | "no binding"
            | "no_binding"
            | "source language"
            | "source_language"
            | "unmapped"
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

/// Builds the canonical, checked-in M05-931 certification packet. Certifies all eight claimed M5
/// onboarding / migration / command-help surfaces: four deliver their claim (green) and four
/// auto-narrow a not-current truth axis to a weaker teaching ceiling (yellow). No surface hides
/// drift (red), and no surface drops command-binding / migration-mapping / blocked-action /
/// source-language lineage.
pub fn seeded_m5_contextual_teaching_component_certification_packet(
) -> TeachingSurfaceCertificationPacket {
    TeachingSurfaceCertificationPacket::new(TeachingSurfaceCertificationPacketInput {
        packet_id: "m5-contextual-teaching-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: TEACHING_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: TEACHING_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:contextual-teaching-component-certification:{id}"),
        TEACHING_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        TEACHING_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> TeachingCertExportParity {
    TeachingCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: TeachingCertificationAxis) -> &'static str {
    match axis {
        TeachingCertificationAxis::Visual => {
            "tip trigger, command binding, migration mapping, sequence state, blocked-action owner/reason/next-safe-action, and source-language citation shown on-surface"
        }
        TeachingCertificationAxis::Keyboard => {
            "the same teaching/migration/blocked-action truth and its actions are keyboard-reachable"
        }
        TeachingCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only"
        }
        TeachingCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support and automation from the same command identity"
        }
        TeachingCertificationAxis::DegradedState => {
            "a snoozed tip, a partial bridge, an unsupported sequence, or stale localized fallback honestly downgrades the ExactTeaching/ReviewableGuidance claim"
        }
        TeachingCertificationAxis::TeachingBoundaryProvenance => {
            "tip trigger, command binding, migration mapping, sequence state, blocked-action owner/reason/next-safe-action, and source-language citation stay explicit before any teach, migrate, or blocked-action explanation; the boundary never drops command-binding/migration-mapping/blocked-action/source-language lineage"
        }
    }
}

fn seed_certified(axis: TeachingCertificationAxis) -> TeachingAxisOutcome {
    TeachingAxisOutcome {
        axis,
        state: TeachingAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: TeachingCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5TeachingDowngradeTrigger,
) -> TeachingAxisOutcome {
    TeachingAxisOutcome {
        axis,
        state: TeachingAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<TeachingAxisOutcome> {
    TeachingCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: TeachingCertificationAxis,
    outcome: TeachingAxisOutcome,
) -> Vec<TeachingAxisOutcome> {
    TeachingCertificationAxis::ALL
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
    surface: M5ContextualTeachingHelpCertifiedSurface,
    claimed_claim: M5TeachingComponentClaim,
    certified_claim: M5TeachingComponentClaim,
    consumed_families: &[M5ContextualTeachingComponentFamily],
    axis_outcomes: Vec<TeachingAxisOutcome>,
    claim_auto_narrow: Option<TeachingClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> TeachingSurfaceCertificationRow {
    let mut row = TeachingSurfaceCertificationRow {
        record_kind: TEACHING_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: TEACHING_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        lineage_preserved: true,
        canonical_bundle_ref: TEACHING_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: TeachingSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            TEACHING_CERT_MATRIX_REF.to_owned(),
            TEACHING_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-07T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: TeachingCertificationAxis,
    from_claim: M5TeachingComponentClaim,
    to_claim: M5TeachingComponentClaim,
    label: &str,
) -> TeachingClaimAutoNarrow {
    TeachingClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
        preserves_lineage_continuity: true,
    }
}

fn seeded_rows() -> Vec<TeachingSurfaceCertificationRow> {
    use M5ContextualTeachingComponentFamily::*;
    use M5ContextualTeachingHelpCertifiedSurface as S;
    use M5TeachingComponentClaim::*;
    use M5TeachingDowngradeTrigger as Trig;
    use TeachingCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:first-run-onboarding",
            S::FirstRunOnboarding,
            ExactTeaching,
            ExactTeaching,
            &[ContextualTipCard, SequenceHelpStrip],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "command_binding"],
            &[
                "the contextual tip card keeps its why-now trigger, concrete next action, and stable command binding explicit before the tip teaches in place",
                "the sequence-help strip keeps its current mode, next-key guidance, and cancel hint explicit while the onboarding sequence runs",
                "keyboard/screen-reader reach preserved for the tip card and the sequence-help strip",
                "provenance: a first-run tip never teaches a command it cannot name or run",
            ],
        ),
        seed_row(
            "cert:command-palette-docs",
            S::CommandPaletteDocs,
            ExactTeaching,
            ExactTeaching,
            &[SequenceHelpStrip, WhyUnavailableExplanationRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "sequence_state"],
            &[
                "the sequence-help strip keeps its current mode, valid next keys, and example command explicit in the palette",
                "the why-unavailable explanation row keeps its blocked-action owner, reason, and next safe action explicit rather than a bare disabled state",
                "keyboard/screen-reader reach preserved for the sequence-help strip and the why-unavailable row",
                "provenance: the command palette never shows a blocked action without naming who blocked it and what to do next",
            ],
        ),
        seed_row(
            "cert:keybinding-help",
            S::KeybindingHelp,
            ExactTeaching,
            ExactTeaching,
            &[SequenceHelpStrip, ContextualTipCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "leader_key"],
            &[
                "the sequence-help strip keeps its current leader, valid next keys, and cancel key explicit in the keybinding help",
                "the contextual tip card keeps its command binding explicit so a keybinding tip never teaches an unbound chord",
                "keyboard/screen-reader reach preserved for the sequence-help strip and the tip card",
                "provenance: keybinding help never presents a chord whose command it cannot resolve",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            ReviewableGuidance,
            ReviewableGuidance,
            &[SourceLanguageFallback, ContextualTipCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "source_language_citation"],
            &[
                "support export reconstructs tip-trigger/command-binding/migration-mapping/sequence-state/blocked-action/source-language-citation truth from the same command identity",
                "the source-language fallback surface keeps its canonical citation explicit in the exported packet rather than severing it",
                "the contextual tip card keeps its command binding explicit in the exported teaching record",
                "provenance: a teaching export never carries raw teaching copy, captured source-language bodies, or imported migration payloads",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:migration-importer-review",
            S::MigrationImporterReview,
            ExactTeaching,
            PartialBridgeProjection,
            &[MigrationBridgeCard],
            seed_certified_except(
                Ax::TeachingBoundaryProvenance,
                seed_narrowed(
                    Ax::TeachingBoundaryProvenance,
                    "the migration bridge maps the imported behavior only partially and cannot claim an exact native mapping",
                    "The migration importer review resolves a migration bridge that maps the imported behavior only partially, so the ExactTeaching claim narrows to partial-bridge-projection instead of implying the old path maps exactly onto a native command",
                    Trig::MigrationMappingUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::TeachingBoundaryProvenance,
                ExactTeaching,
                PartialBridgeProjection,
                "Bridge maps only part of the imported behavior: the migration bridge card shows the old path bridges partially onto the native command rather than implying an exact mapping",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the migration bridge card keeps the partial-mapping scope and its unsupported edge cases explicit and offers a view-mapping action",
                "the migration bridge card keeps its imported source tool and old-path/new-command mapping explicit while the bridge stays partial",
                "teaching-boundary: ExactTeaching narrows to partial-bridge-projection (auto-narrowed)",
                "known compatibility note: partial-bridge behavior — a partial migration bridge never reads as an exact native mapping",
            ],
        ),
        seed_row(
            "cert:modal-sequence-overlay",
            S::ModalSequenceOverlay,
            ExactTeaching,
            UnsupportedSequenceProjection,
            &[SequenceHelpStrip],
            seed_certified_except(
                Ax::TeachingBoundaryProvenance,
                seed_narrowed(
                    Ax::TeachingBoundaryProvenance,
                    "the command sequence has no committed binding in the current context and cannot be run from here",
                    "The modal sequence overlay resolves a command sequence with no committed binding in the current context, so the ExactTeaching claim narrows to unsupported-sequence-projection instead of implying the sequence can be completed here",
                    Trig::SequenceHelpStateUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::TeachingBoundaryProvenance,
                ExactTeaching,
                UnsupportedSequenceProjection,
                "Sequence not bound in this context: the sequence-help strip shows the current mode and cancel key rather than implying the next key completes a runnable command",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the sequence-help strip keeps the unsupported-here reason and cancel key explicit rather than silently offering a dead-end key",
                "the sequence-help strip keeps its current mode and open-full-cheat-sheet action explicit while the sequence stays unsupported",
                "teaching-boundary: ExactTeaching narrows to unsupported-sequence-projection (auto-narrowed)",
                "known compatibility note: unsupported-sequence behavior — an unsupported command sequence never reads as a runnable exact-teaching path",
            ],
        ),
        seed_row(
            "cert:localized-support",
            S::LocalizedSupport,
            ExactTeaching,
            StaleFallbackProjection,
            &[SourceLanguageFallback],
            seed_certified_except(
                Ax::TeachingBoundaryProvenance,
                seed_narrowed(
                    Ax::TeachingBoundaryProvenance,
                    "the localized help content is stale / source-language only and cannot claim a current localized teaching state",
                    "The localized support packet resolves stale, source-language-only help content, so the ExactTeaching claim narrows to stale-fallback-projection instead of implying the localized help is current — its canonical citation stays preserved",
                    Trig::SourceLanguageFallbackUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::TeachingBoundaryProvenance,
                ExactTeaching,
                StaleFallbackProjection,
                "Localized help is source-language only: the source-language fallback surface preserves its canonical citation and shows the content is a source-language fallback rather than a current localized teaching state",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the source-language fallback surface keeps its canonical citation preserved and its source-language-only reason explicit",
                "the source-language fallback surface keeps its fallback state and request-localization action explicit while the content stays stale",
                "teaching-boundary: ExactTeaching narrows to stale-fallback-projection (auto-narrowed)",
                "known compatibility note: stale-fallback behavior — source-language-only help never reads as current localized teaching and never severs its canonical citation",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            ExactTeaching,
            SnoozedTipProjection,
            &[ContextualTipCard],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the contextual tip is snoozed / suppressed in the headless context and only its stable command binding remains",
                    "The CLI-headless surface resolves a snoozed contextual tip with only its stable command binding available, so the ExactTeaching claim narrows to snoozed-tip-projection instead of presenting a suppressed tip as an active live tip",
                    Trig::TipCommandBindingUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ExactTeaching,
                SnoozedTipProjection,
                "Tip snoozed in headless context: the contextual tip card keeps its stable command binding available and shows the tip is snoozed rather than presenting it as an active live tip",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the contextual tip card keeps its stable command binding explicit and honors the snoozed / quiet-hours state rather than re-surfacing a live tip",
                "the contextual tip card keeps its command reference reachable in the headless export while the tip stays snoozed",
                "degraded-state: ExactTeaching narrows to snoozed-tip-projection (auto-narrowed)",
                "known compatibility note: snoozed-tip behavior — a snoozed contextual tip never reads as an active live teaching tip",
            ],
        ),
    ]
}
