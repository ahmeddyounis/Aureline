//! M05-939 surface certification over the frozen M5 shared-component-state-taxonomy /
//! interactive-state / selection-or-lock-state / degraded-state-application component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix`])
//! defines the four reusable shared-taxonomy, interactive-state, selection-or-lock-state, and
//! degraded-state-application components, the M05-933..936 primitive lanes narrow each one, the
//! M05-937 consumer lane
//! ([`crate::add_shared_shell_command_search_review_settings_provider_test_and_support_consumers_so_state_taxonomy_components_keep_label_recovery_and_accessibility_parity_across_claimed_m5_profiles`])
//! proves they are reusable across the claimed shell / command / search / review / settings /
//! provider / test / support consumers, and the M05-938 accessibility / auto-narrowing capstone
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_state_owner_block_reason_or_recovery_truth_is_missing_or_stale_across_claimed_m5_components`])
//! certifies keyboard / screen-reader / CLI / export parity per family, this closing capstone
//! *certifies* that the shared component-state taxonomy truth holds on every claimed M5 control,
//! collection, and recovery surface — and auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user interacts with, selects on, or recovers from (the
//! control affordance, the dense collection, the blocked-action prompt, the settings / capability
//! sheet, the activity / recovery view, the command palette, the support / export bundle, and the
//! CLI), not on component family or primitive lane. Each [`StateSurfaceCertificationRow`] certifies
//! one surface across six truth axes — visual, keyboard, screen-reader, CLI/export, degraded-state,
//! and state-boundary provenance — and either passes (green), auto-narrows its state claim to the
//! weakest supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind a
//! full-truth claim inherited from a healthier state lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps an `ExactStateTruth` / `ReviewableStateGuidance` claim while one of its
//! truth axes is not current — the state's cause is unresolved, the lock / read-only / disabled
//! owner is unresolved, the degraded / warning / error recovery is unavailable, or the
//! accessibility / export proof is stale — is over-claiming and blocks; a surface that discloses
//! the reduction by narrowing its state claim (with a bound reason and a frozen downgrade trigger)
//! is honestly yellow. State truth never loses lineage: a narrowed surface always preserves its
//! state-cause / owner / block-reason / recovery lineage continuity rather than dropping it between
//! a control, a dense collection, and a recovery view. The always-on CLI/export axis must always
//! stay certified, so support and automation can reconstruct the same typed-state / cause / owner /
//! block-reason / recovery truth from the same component identity the user saw.
//!
//! Every row cites exactly one canonical shared-component-state proof bundle
//! ([`STATE_CERT_CANONICAL_BUNDLE_REF`]) — the frozen component matrix release proof — rather than
//! cloning per-surface evidence. The packet is metadata-only: raw state copy, captured surface
//! bodies, and credentials never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-shared-component-state-taxonomy-certification.schema.json`](../../../../schemas/ui/m5-shared-component-state-taxonomy-certification.schema.json).
//! The contract doc is
//! [`docs/design-system/m5_shared_component_state_taxonomy_certification_contract.md`](../../../../docs/design-system/m5_shared_component_state_taxonomy_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_shell_command_search_review_settings_provider_test_and_support_consumers_so_state_taxonomy_components_keep_label_recovery_and_accessibility_parity_across_claimed_m5_profiles as consumers;
use crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_state_owner_block_reason_or_recovery_truth_is_missing_or_stale_across_claimed_m5_components as a11y;
use a11y::M5StateComponentClaim;
use matrix::{M5ComponentStateDowngradeTrigger, M5SharedComponentStateFamily};

/// Schema version stamped on the M05-939 certification packet.
pub const STATE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`StateSurfaceCertificationPacket`].
pub const STATE_CERT_RECORD_KIND: &str = "m5_shared_component_state_taxonomy_certification_packet";

/// Stable record-kind tag carried by each [`StateSurfaceCertificationRow`].
pub const STATE_CERT_ROW_RECORD_KIND: &str = "m5_shared_component_state_taxonomy_certification_row";

/// Repo-relative path of the boundary schema.
pub const STATE_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-shared-component-state-taxonomy-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const STATE_CERT_DOC_REF: &str =
    "docs/design-system/m5_shared_component_state_taxonomy_certification_contract.md";

/// Repo-relative path of the frozen shared-component-state matrix schema the certified surfaces
/// render.
pub const STATE_CERT_MATRIX_REF: &str = matrix::M5_SHARED_COMPONENT_STATE_SCHEMA_REF;

/// The one canonical shared-component-state proof bundle every certified surface cites as its
/// first-resolved component truth. All eight surfaces point back to it rather than cloning
/// per-surface evidence.
pub const STATE_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_SHARED_COMPONENT_STATE_ARTIFACT_REF;

/// The M05-937 consumer-adoption support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const STATE_CERT_CONSUMER_BUNDLE_REF: &str =
    consumers::M5_STATE_COMPONENT_CONSUMER_ARTIFACT_REF;

/// The M05-938 accessibility / auto-narrowing support export whose keyboard / screen-reader /
/// CLI / export parity this capstone builds on. Recorded as a supporting evidence ref on every row.
pub const STATE_CERT_A11Y_BUNDLE_REF: &str = a11y::STATE_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const STATE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-shared-component-state-taxonomy-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const STATE_CERT_CSV_REF: &str =
    "artifacts/release/m5-shared-component-state-taxonomy-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const STATE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-shared-component-state-taxonomy-certification/report.md";

/// The eight claimed M5 control / collection / recovery surfaces this capstone certifies. Keyed on
/// the surface a user actually interacts with, selects on, or recovers from, not on the reusable
/// component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SharedComponentStateCertifiedSurface {
    /// The interactive control affordance surface (buttons, toggles, inputs, pane affordances).
    ControlAffordance,
    /// The dense collection surface (tabs, trees, lists, tables) that carries selection / current /
    /// lock state.
    DenseCollection,
    /// The blocked-action prompt surface (locked / disabled / read-only affordances that must name
    /// their owner and block reason).
    BlockedActionPrompt,
    /// The settings / capability sheet surface.
    SettingsCapabilitySheet,
    /// The activity / recovery view surface (degraded / warning / error blocks with consequence and
    /// recovery).
    ActivityRecoveryView,
    /// The command palette / command-docs surface.
    CommandPalette,
    /// The support / export bundle surface.
    SupportExport,
    /// The CLI / headless surface.
    CliHeadless,
}

impl M5SharedComponentStateCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5SharedComponentStateCertifiedSurface; 8] = [
        M5SharedComponentStateCertifiedSurface::ControlAffordance,
        M5SharedComponentStateCertifiedSurface::DenseCollection,
        M5SharedComponentStateCertifiedSurface::BlockedActionPrompt,
        M5SharedComponentStateCertifiedSurface::SettingsCapabilitySheet,
        M5SharedComponentStateCertifiedSurface::ActivityRecoveryView,
        M5SharedComponentStateCertifiedSurface::CommandPalette,
        M5SharedComponentStateCertifiedSurface::SupportExport,
        M5SharedComponentStateCertifiedSurface::CliHeadless,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlAffordance => "control_affordance",
            Self::DenseCollection => "dense_collection",
            Self::BlockedActionPrompt => "blocked_action_prompt",
            Self::SettingsCapabilitySheet => "settings_capability_sheet",
            Self::ActivityRecoveryView => "activity_recovery_view",
            Self::CommandPalette => "command_palette",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity dimensions the
/// spec requires verifying — visual, keyboard, screen-reader, CLI/export, degraded-state, and
/// state-boundary provenance. The CLI/export axis is always-on and must stay certified for every
/// surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateCertificationAxis {
    /// Visual parity: the typed state, its cause, the lock / read-only / disabled owner and block
    /// reason, and its consequence / recovery are shown on the primary surface, never color-only.
    Visual,
    /// Keyboard-reach parity: the same state truth and its recovery actions are reachable without a
    /// pointer.
    Keyboard,
    /// Screen-reader parity: the same state is announced non-visually, never relying on color or a
    /// status glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is reconstructable as text /
    /// JSON / Markdown for support and automation, from the same component identity.
    CliExport,
    /// Degraded-state parity: a missing-cause, unresolved-owner, unavailable-recovery, or
    /// stale-proof state honestly downgrades an `ExactStateTruth` / `ReviewableStateGuidance` claim
    /// to a weaker state tier.
    DegradedState,
    /// State-boundary provenance parity: the typed state, its cause, the lock / read-only /
    /// disabled owner and block reason, and its consequence / recovery stay explicit before any
    /// state is presented — never inheriting a healthier lane's state truth, never collapsing
    /// current into selected, masking a lock as a plain disabled, showing pending as generic
    /// loading, or omitting a degraded consequence / recovery, and never dropping state-cause /
    /// owner / block-reason / recovery lineage between a control, a dense collection, and a recovery
    /// view.
    StateBoundaryProvenance,
}

impl StateCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [StateCertificationAxis; 6] = [
        StateCertificationAxis::Visual,
        StateCertificationAxis::Keyboard,
        StateCertificationAxis::ScreenReader,
        StateCertificationAxis::CliExport,
        StateCertificationAxis::DegradedState,
        StateCertificationAxis::StateBoundaryProvenance,
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
            Self::StateBoundaryProvenance => "state_boundary_provenance",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim inherited from
    /// a healthier surface.
    UndisclosedDrift,
}

impl StateAxisCertificationState {
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
pub enum StateSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed state tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, lineage is
    /// dropped, or the narrowing is inconsistent.
    Red,
}

impl StateSurfaceClaimStatus {
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

/// The copy / export parity a certified surface preserves. The CLI/export axis certifies only when
/// this offers text / JSON / Markdown reconstruction and prohibits a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The typed-state / cause / owner / block-reason / recovery fields the surface preserves in
    /// export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl StateCertExportParity {
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
pub struct StateAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: StateCertificationAxis,
    /// The certification state of the axis.
    pub state: StateAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5ComponentStateDowngradeTrigger>,
}

impl StateAxisOutcome {
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
            StateAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            StateAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            StateAxisCertificationState::UndisclosedDrift => {
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
pub struct StateClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: StateCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5StateComponentClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5StateComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
    /// True when the narrowed surface still preserves its state-cause / owner / block-reason /
    /// recovery lineage continuity rather than dropping it between a control, a dense collection,
    /// and a recovery view.
    pub preserves_lineage_continuity: bool,
}

/// One certified M5 control / collection / recovery surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateSurfaceCertificationRow {
    /// Record kind; must equal [`STATE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`STATE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5SharedComponentStateCertifiedSurface,
    /// The state-claim ceiling the surface asserts.
    pub claimed_claim: M5StateComponentClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5StateComponentClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5SharedComponentStateFamily>,
    /// One outcome per [`StateCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<StateAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<StateClaimAutoNarrow>,
    /// True when this surface never drops its state-cause / owner / block-reason / recovery lineage
    /// continuity between a control, a dense collection, and a recovery view.
    pub lineage_preserved: bool,
    /// The one canonical shared-component-state proof bundle this surface cites. Must equal
    /// [`STATE_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: StateSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: StateCertExportParity,
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

impl StateSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: StateCertificationAxis) -> Option<&StateAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<StateCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && StateCertificationAxis::ALL.iter().all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes.iter().all(StateAxisOutcome::well_formed)
    }

    /// True when the surface narrows its state claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<StateCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == StateAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether a narrowed surface preserves its state-cause / owner / block-reason / recovery
    /// lineage continuity rather than dropping it. A non-narrowed surface trivially preserves
    /// lineage; a narrowed one must say so.
    pub fn preserves_lineage_continuity(&self) -> bool {
        match &self.claim_auto_narrow {
            Some(narrow) => self.lineage_preserved && narrow.preserves_lineage_continuity,
            None => self.lineage_preserved,
        }
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, CLI/export parity must
    /// always certify, state truth must never drop lineage, and the narrowing must be consistent.
    pub fn derive_status(&self) -> StateSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != STATE_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.preserves_lineage_continuity()
        {
            return StateSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return StateSurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(StateCertificationAxis::CliExport) {
            Some(o) if o.state == StateAxisCertificationState::Certified => {}
            _ => return StateSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == StateAxisCertificationState::UndisclosedDrift)
        {
            return StateSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return StateSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return StateSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                    || !narrow.preserves_lineage_continuity
                {
                    return StateSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return StateSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return StateSurfaceClaimStatus::Red;
        }

        StateSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == STATE_CERT_ROW_RECORD_KIND
            && self.schema_version == STATE_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-939 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateSurfaceCertificationSummary {
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

/// Constructor input for [`StateSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StateSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<StateSurfaceCertificationRow>,
}

/// Checked-in M05-939 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<StateSurfaceCertificationRow>,
    pub summary: StateSurfaceCertificationSummary,
}

impl StateSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: StateSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: STATE_CERT_SCHEMA_VERSION,
            record_kind: STATE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: StateSurfaceCertificationSummary {
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
    pub fn represented_surfaces(&self) -> BTreeSet<M5SharedComponentStateCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5SharedComponentStateFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5SharedComponentStateCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5SharedComponentStateFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(StateCertificationAxis::CliExport)
                .is_some_and(|o| o.state == StateAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> StateSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == StateSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == StateSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == StateSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(StateSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_preserved = self
            .rows
            .iter()
            .all(StateSurfaceCertificationRow::preserves_lineage_continuity);

        StateSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == STATE_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(StateSurfaceCertificationRow::covers_all_axes),
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
    pub fn validate(&self) -> Vec<StateCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != STATE_CERT_SCHEMA_VERSION {
            violations.push(StateCertificationViolation::SchemaVersion {
                expected: STATE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != STATE_CERT_RECORD_KIND {
            violations.push(StateCertificationViolation::RecordKind {
                expected: STATE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(StateCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != STATE_CERT_CANONICAL_BUNDLE_REF {
            violations.push(StateCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(StateCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(StateCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(StateCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(StateCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != STATE_CERT_CANONICAL_BUNDLE_REF {
                violations.push(StateCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(StateCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(StateCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // State truth must never drop lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(StateCertificationViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(StateCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(StateCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == StateSurfaceClaimStatus::Red {
                violations.push(StateCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(StateCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(StateCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(StateCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(StateCertificationViolation::RawStateMaterialInExport);
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
        out.push_str("# M5 Shared-Component-State Taxonomy Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5SharedComponentStateCertifiedSurface::ALL.len(),
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
pub fn current_m5_shared_component_state_taxonomy_certification_export(
) -> Result<StateSurfaceCertificationPacket, StateCertificationArtifactError> {
    let packet: StateSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shared-component-state-taxonomy-certification/support_export.json"
    )))
    .map_err(StateCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(StateCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum StateCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<StateCertificationViolation>),
}

impl fmt::Display for StateCertificationArtifactError {
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

impl Error for StateCertificationArtifactError {}

/// Validation failure for M05-939 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StateCertificationViolation {
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
    RawStateMaterialInExport,
}

impl fmt::Display for StateCertificationViolation {
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
                    "packet does not cite the canonical shared-component-state proof bundle"
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
                    "row {id} does not cite the one canonical shared-component-state proof bundle"
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
                    "row {id} drops state-cause / owner / block-reason / recovery lineage continuity (a narrowed surface must preserve its lineage between a control, a dense collection, and a recovery view)"
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
                    "not every claimed M5 control / collection / recovery surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen shared-component-state family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawStateMaterialInExport => {
                write!(f, "export contains raw state material")
            }
        }
    }
}

impl Error for StateCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&StateAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != StateAxisCertificationState::Certified,
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
            | "disabled"
            | "read-only"
            | "read_only"
            | "locked"
            | "pending"
            | "loading"
            | "no owner"
            | "no_owner"
            | "no cause"
            | "no_cause"
            | "no recovery"
            | "no_recovery"
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

/// Builds the canonical, checked-in M05-939 certification packet. Certifies all eight claimed M5
/// control / collection / recovery surfaces: four deliver their claim (green) and four auto-narrow
/// a not-current truth axis to a weaker state ceiling (yellow). No surface hides drift (red), and
/// no surface drops state-cause / owner / block-reason / recovery lineage.
pub fn seeded_m5_shared_component_state_taxonomy_certification_packet(
) -> StateSurfaceCertificationPacket {
    StateSurfaceCertificationPacket::new(StateSurfaceCertificationPacketInput {
        packet_id: "m5-shared-component-state-taxonomy-certification:stable:0001".to_owned(),
        as_of: "2026-07-08T00:00:00Z".to_owned(),
        matrix_ref: STATE_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: STATE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:shared-component-state-taxonomy-certification:{id}"),
        STATE_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        STATE_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> StateCertExportParity {
    StateCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: StateCertificationAxis) -> &'static str {
    match axis {
        StateCertificationAxis::Visual => {
            "typed state, its cause, the lock/read-only/disabled owner and block reason, and its consequence/recovery shown on-surface, never color-only"
        }
        StateCertificationAxis::Keyboard => {
            "the same state truth and its recovery actions are keyboard-reachable"
        }
        StateCertificationAxis::ScreenReader => {
            "the same state is announced non-visually, never color/glyph-only"
        }
        StateCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support and automation from the same component identity"
        }
        StateCertificationAxis::DegradedState => {
            "a missing-cause, unresolved-owner, unavailable-recovery, or stale-proof state honestly downgrades the ExactStateTruth/ReviewableStateGuidance claim"
        }
        StateCertificationAxis::StateBoundaryProvenance => {
            "typed state, its cause, the lock/read-only/disabled owner and block reason, and its consequence/recovery stay explicit before any state is presented; current never collapses into selected, a lock never masks as plain disabled, pending never reads as generic loading, and the boundary never drops state-cause/owner/block-reason/recovery lineage"
        }
    }
}

fn seed_certified(axis: StateCertificationAxis) -> StateAxisOutcome {
    StateAxisOutcome {
        axis,
        state: StateAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: StateCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5ComponentStateDowngradeTrigger,
) -> StateAxisOutcome {
    StateAxisOutcome {
        axis,
        state: StateAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<StateAxisOutcome> {
    StateCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: StateCertificationAxis,
    outcome: StateAxisOutcome,
) -> Vec<StateAxisOutcome> {
    StateCertificationAxis::ALL
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
    surface: M5SharedComponentStateCertifiedSurface,
    claimed_claim: M5StateComponentClaim,
    certified_claim: M5StateComponentClaim,
    consumed_families: &[M5SharedComponentStateFamily],
    axis_outcomes: Vec<StateAxisOutcome>,
    claim_auto_narrow: Option<StateClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> StateSurfaceCertificationRow {
    let mut row = StateSurfaceCertificationRow {
        record_kind: STATE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: STATE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        lineage_preserved: true,
        canonical_bundle_ref: STATE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: StateSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            STATE_CERT_MATRIX_REF.to_owned(),
            STATE_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-08T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: StateCertificationAxis,
    from_claim: M5StateComponentClaim,
    to_claim: M5StateComponentClaim,
    label: &str,
) -> StateClaimAutoNarrow {
    StateClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
        preserves_lineage_continuity: true,
    }
}

fn seeded_rows() -> Vec<StateSurfaceCertificationRow> {
    use M5ComponentStateDowngradeTrigger as Trig;
    use M5SharedComponentStateCertifiedSurface as S;
    use M5SharedComponentStateFamily::*;
    use M5StateComponentClaim::*;
    use StateCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:control-affordance",
            S::ControlAffordance,
            ExactStateTruth,
            ExactStateTruth,
            &[InteractiveState, SharedComponentStateTaxonomy],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "typed_state"],
            &[
                "the interactive-state control keeps default/hover/focus-visible/pressed distinct with no color-only and no layout-shift treatment",
                "the shared taxonomy keeps every governed state name and its precedence rules explicit behind the control affordance",
                "keyboard/screen-reader reach preserved for the control affordance and its state cues",
                "provenance: an interactive control never encodes a state by color alone or hides its keyboard focus route",
            ],
        ),
        seed_row(
            "cert:dense-collection",
            S::DenseCollection,
            ExactStateTruth,
            ExactStateTruth,
            &[SelectionOrLockState, SharedComponentStateTaxonomy],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "selection_state"],
            &[
                "the selection-or-lock collection keeps current and selected distinct and never masks a lock as a plain disabled row",
                "the shared taxonomy keeps read-only-over-disabled and locked-over-disabled precedence explicit across the dense tab/tree/list/table lineage",
                "keyboard/screen-reader reach preserved for the collection's selection and lock states",
                "provenance: a dense collection never collapses current into selected and never hides a lock owner",
            ],
        ),
        seed_row(
            "cert:command-palette",
            S::CommandPalette,
            ExactStateTruth,
            ExactStateTruth,
            &[DegradedStateApplication, SharedComponentStateTaxonomy],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "recovery_class"],
            &[
                "the degraded-state-application block keeps loading and pending distinct and names consequence and recovery for warning/error states in the palette",
                "the shared taxonomy keeps pending-never-as-loading precedence explicit for command results",
                "keyboard/screen-reader reach preserved for the palette's degraded and pending states",
                "provenance: the command palette never shows pending as generic loading or omits a degraded consequence/recovery",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            ReviewableStateGuidance,
            ReviewableStateGuidance,
            &[DegradedStateApplication, SharedComponentStateTaxonomy],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "state_cause"],
            &[
                "support export reconstructs typed-state / cause / owner / block-reason / recovery truth from the same component identity",
                "the degraded-state-application block keeps its consequence and next-safe-action explicit in the exported packet",
                "the shared taxonomy keeps every governed state name and precedence rule explicit in the exported record",
                "provenance: a state export never carries raw state copy or captured surface bodies",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:settings-capability-sheet",
            S::SettingsCapabilitySheet,
            ExactStateTruth,
            CauseNarrowedProjection,
            &[SharedComponentStateTaxonomy],
            seed_certified_except(
                Ax::StateBoundaryProvenance,
                seed_narrowed(
                    Ax::StateBoundaryProvenance,
                    "the disabled/unavailable capability could not resolve its cause and cannot claim an exact live state",
                    "The settings capability sheet resolves a capability whose state cause could not be resolved, so the ExactStateTruth claim narrows to cause-narrowed-projection instead of presenting a silent state with no reason",
                    Trig::StateCauseUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::StateBoundaryProvenance,
                ExactStateTruth,
                CauseNarrowedProjection,
                "Capability state cause unresolved: the settings sheet shows a cause-narrowed explanation of why the capability is off rather than a silent disabled toggle",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the settings sheet keeps the unresolved-cause explanation explicit rather than a bare disabled control",
                "the shared taxonomy keeps the governed state name explicit while the cause stays narrowed",
                "state-boundary: ExactStateTruth narrows to cause-narrowed-projection (auto-narrowed)",
                "known compatibility note: cause-narrowed behavior — a capability with an unresolved cause never reads as an exact live state",
            ],
        ),
        seed_row(
            "cert:blocked-action-prompt",
            S::BlockedActionPrompt,
            ExactStateTruth,
            OwnerNarrowedProjection,
            &[SelectionOrLockState],
            seed_certified_except(
                Ax::StateBoundaryProvenance,
                seed_narrowed(
                    Ax::StateBoundaryProvenance,
                    "the lock/read-only/disabled owner could not be resolved and the prompt cannot claim an exact live state",
                    "The blocked-action prompt resolves a locked affordance whose owner could not be resolved, so the ExactStateTruth claim narrows to owner-narrowed-projection instead of masking the lock as a plain silent disabled control",
                    Trig::LockOwnerMasked,
                ),
            ),
            Some(seed_narrow(
                Ax::StateBoundaryProvenance,
                ExactStateTruth,
                OwnerNarrowedProjection,
                "Lock owner unresolved: the blocked-action prompt shows an owner-narrowed explanation that the affordance is locked rather than presenting it as a plain disabled control",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the blocked-action prompt keeps the locked-not-disabled distinction explicit rather than a silent disabled affordance",
                "the selection-or-lock component keeps its read-only inspectability while the owner stays narrowed",
                "state-boundary: ExactStateTruth narrows to owner-narrowed-projection (auto-narrowed)",
                "known compatibility note: owner-narrowed behavior — a lock with an unresolved owner never reads as a plain disabled control",
            ],
        ),
        seed_row(
            "cert:activity-recovery-view",
            S::ActivityRecoveryView,
            ExactStateTruth,
            RecoveryNarrowedProjection,
            &[DegradedStateApplication],
            seed_certified_except(
                Ax::StateBoundaryProvenance,
                seed_narrowed(
                    Ax::StateBoundaryProvenance,
                    "the degraded/warning/error state's recovery could not be preserved and cannot claim a healthy live state",
                    "The activity recovery view resolves a degraded state whose recovery path could not be preserved, so the ExactStateTruth claim narrows to recovery-narrowed-projection instead of implying the surface is healthy — its named consequence stays explicit",
                    Trig::ConsequenceOrRecoveryOmitted,
                ),
            ),
            Some(seed_narrow(
                Ax::StateBoundaryProvenance,
                ExactStateTruth,
                RecoveryNarrowedProjection,
                "Recovery unavailable: the activity recovery view names the consequence and shows a recovery-narrowed explanation rather than implying the surface is a healthy live state",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the activity recovery view keeps the named consequence and what-still-works explicit rather than a silent degraded surface",
                "the degraded-state-application component keeps its warning-vs-error distinction explicit while recovery stays narrowed",
                "state-boundary: ExactStateTruth narrows to recovery-narrowed-projection (auto-narrowed)",
                "known compatibility note: recovery-narrowed behavior — a degraded state with unavailable recovery never reads as a healthy live state",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            ExactStateTruth,
            StaleProofProjection,
            &[SharedComponentStateTaxonomy],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the accessibility / export proof for this surface has gone stale and only its identity, state, and keyboard route remain current",
                    "The CLI-headless surface resolves a stale accessibility/export proof, so the ExactStateTruth claim narrows to stale-proof-projection instead of presenting a stale proof as a current exact-truth state",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ExactStateTruth,
                StaleProofProjection,
                "Proof stale in headless context: the CLI surface keeps its identity, state, and keyboard route and shows the proof is stale rather than presenting it as a current exact-truth state",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the CLI-headless surface keeps its typed state and keyboard route reachable while the proof stays stale",
                "the shared taxonomy keeps the governed state name explicit in the headless export while the proof is stale",
                "degraded-state: ExactStateTruth narrows to stale-proof-projection (auto-narrowed)",
                "known compatibility note: stale-proof behavior — a surface with a stale proof never reads as a current exact-truth state",
            ],
        ),
    ]
}
