//! M05-1067 surface certification over the frozen M5 adaptive-efficiency
//! component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix`])
//! defines the eight reusable power-state-indicator, throttled-subsystem-row,
//! background-work-row, background-work-banner, per-workspace-override-sheet,
//! override-policy-note-row, resume-summary-card, and stale-result-continuity-note
//! components, the M05-1061..1064 implement lanes narrow each one, the M05-1066
//! consumer lane
//! ([`crate::add_shell_status_activity_center_notebook_preview_docs_browser_pipeline_incident_and_support_export_consumers_so_adaptive_efficiency_components_keep_source_state_backlog_and_stale_result_language_aligned`])
//! adopts them, and the M05-1065 accessibility lane
//! ([`crate::implement_keyboard_screen_reader_reduced_motion_high_contrast_cli_export_and_support_packet_parity_and_adaptive_efficiency_component_claim_auto_narrowing`])
//! proves keyboard / screen-reader / reduced-motion / high-contrast / CLI-export
//! parity and per-family auto-narrowing, this closing capstone *certifies* that the
//! shared component truth holds on every claimed M5 adaptive-efficiency surface — and
//! auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user, operator, or support engineer reads
//! adaptive-efficiency truth through (shell status bar, activity center, work-content
//! canvas, policy-aware settings, incident diagnostics, docs/help, support/export, and
//! CLI/headless), not on component family or implement lane. Each
//! [`EfficiencySurfaceCertificationRow`] certifies one surface across six truth
//! axes — visual, keyboard, screen-reader, CLI/export, degraded-state, and
//! efficiency-truth behavior — and either passes (green), auto-narrows its
//! efficiency-support claim to the weakest supported ceiling (yellow), or is blocked
//! (red) when a degraded axis is hidden behind a full-truth claim inherited from a
//! healthier surface.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A
//! surface that keeps a `FullTruth` / `ResolvedTruth` claim while one of its truth
//! axes is not current is over-claiming and blocks; a surface that discloses the
//! reduction by narrowing its efficiency-support claim (with a bound reason and a
//! frozen downgrade trigger) is honestly yellow. The always-on CLI/export axis must
//! always stay certified, so support and automation can reconstruct the certified
//! source-of-change / slowed-versus-paused / override / resumed-backlog /
//! stale-result truth from the same object identity the user saw.
//!
//! Every row cites exactly one canonical efficiency-proof bundle
//! ([`EFFICIENCY_CERT_CANONICAL_BUNDLE_REF`]) — the frozen adaptive-efficiency
//! component matrix proof — rather than cloning per-surface evidence. The packet is
//! metadata-only: raw device telemetry, credentials, and policy secrets never cross
//! this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-efficiency-component-certification.schema.json`](../../../../schemas/ui/m5-efficiency-component-certification.schema.json).
//! The contract doc is
//! [`docs/help/m5_efficiency_component_certification_contract.md`](../../../../docs/help/m5_efficiency_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_reduced_motion_high_contrast_cli_export_and_support_packet_parity_and_adaptive_efficiency_component_claim_auto_narrowing as a11y;
use a11y::M5EfficiencyAccessClaim;
use matrix::{M5EfficiencyComponentFamily, M5EfficiencyDowngradeTrigger};

/// Schema version stamped on the M05-1067 certification packet.
pub const EFFICIENCY_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`EfficiencySurfaceCertificationPacket`].
pub const EFFICIENCY_CERT_RECORD_KIND: &str = "m5_efficiency_component_certification_packet";

/// Stable record-kind tag carried by each [`EfficiencySurfaceCertificationRow`].
pub const EFFICIENCY_CERT_ROW_RECORD_KIND: &str = "m5_efficiency_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const EFFICIENCY_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-efficiency-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const EFFICIENCY_CERT_DOC_REF: &str =
    "docs/help/m5_efficiency_component_certification_contract.md";

/// Repo-relative path of the frozen adaptive-efficiency component matrix schema the
/// certified surfaces render.
pub const EFFICIENCY_CERT_MATRIX_REF: &str = matrix::M5_EFFICIENCY_COMPONENT_SCHEMA_REF;

/// The one canonical efficiency-proof bundle every certified surface cites as its
/// first-resolved component truth. All eight surfaces point back to it rather than
/// cloning per-surface evidence.
pub const EFFICIENCY_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_EFFICIENCY_COMPONENT_ARTIFACT_REF;

/// The M05-1065 accessibility support export the certification builds on. Recorded as
/// a supporting evidence ref on every row.
pub const EFFICIENCY_CERT_A11Y_BUNDLE_REF: &str = a11y::EFFICIENCY_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const EFFICIENCY_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-efficiency-component-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EFFICIENCY_CERT_CSV_REF: &str =
    "artifacts/release/m5-efficiency-component-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const EFFICIENCY_CERT_REPORT_REF: &str =
    "artifacts/release/m5-efficiency-component-certification-proof/report.md";

/// The eight claimed M5 adaptive-efficiency surfaces this capstone certifies. Keyed on
/// the surface a user, operator, or support engineer reads adaptive-efficiency truth
/// through, not on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyCertifiedSurface {
    /// The desktop shell status bar where the power-state indicator and throttled
    /// subsystems read.
    ShellStatusBar,
    /// The activity center where background-work rows and banners read.
    ActivityCenter,
    /// The work-content canvas (notebook / preview / pipeline / graph) where slowed or
    /// paused work reads.
    WorkContentCanvas,
    /// The policy-aware settings surface where per-workspace override sheets and policy
    /// notes read.
    PolicyAwareSettings,
    /// The incident / diagnostics console where constrained-state truth reads.
    IncidentDiagnostics,
    /// The docs / help surface.
    DocsHelp,
    /// The support / export bundle surface.
    SupportExport,
    /// The CLI / headless surface.
    CliHeadless,
}

impl M5EfficiencyCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5EfficiencyCertifiedSurface; 8] = [
        M5EfficiencyCertifiedSurface::ShellStatusBar,
        M5EfficiencyCertifiedSurface::ActivityCenter,
        M5EfficiencyCertifiedSurface::WorkContentCanvas,
        M5EfficiencyCertifiedSurface::PolicyAwareSettings,
        M5EfficiencyCertifiedSurface::IncidentDiagnostics,
        M5EfficiencyCertifiedSurface::DocsHelp,
        M5EfficiencyCertifiedSurface::SupportExport,
        M5EfficiencyCertifiedSurface::CliHeadless,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellStatusBar => "shell_status_bar",
            Self::ActivityCenter => "activity_center",
            Self::WorkContentCanvas => "work_content_canvas",
            Self::PolicyAwareSettings => "policy_aware_settings",
            Self::IncidentDiagnostics => "incident_diagnostics",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader,
/// CLI/export, degraded-state, and efficiency-truth behavior. The CLI/export axis is
/// always-on and must stay certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EfficiencyCertificationAxis {
    /// Visual parity: source of change, active efficiency state, slowed-versus-paused
    /// work, what still works, override availability, policy owner, resumed backlog, and
    /// stale-result continuity are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same efficiency truth and its actions (inspect,
    /// override, resume) are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on
    /// color or an indicator glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is reconstructable
    /// as text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale, deferred, or partial adaptive-efficiency reading
    /// honestly downgrades a `FullTruth` / `ResolvedTruth` claim rather than reading
    /// current.
    DegradedState,
    /// Efficiency-truth parity: source of change, slowed-versus-paused work, override
    /// availability, policy owner, resumed-work backlog, and stale-result continuity
    /// stay explicit and never collapse into one generic low-power warning, hide paused
    /// work behind a toast, present a blocked override as available, or clear
    /// stale-result context on resume.
    EfficiencyTruth,
}

impl EfficiencyCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [EfficiencyCertificationAxis; 6] = [
        EfficiencyCertificationAxis::Visual,
        EfficiencyCertificationAxis::Keyboard,
        EfficiencyCertificationAxis::ScreenReader,
        EfficiencyCertificationAxis::CliExport,
        EfficiencyCertificationAxis::DegradedState,
        EfficiencyCertificationAxis::EfficiencyTruth,
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
            Self::EfficiencyTruth => "efficiency_truth",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EfficiencyAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a
    /// visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim
    /// inherited from a healthier surface.
    UndisclosedDrift,
}

impl EfficiencyAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole surface. Never asserted by the
/// author — always recomputed from the axis outcomes and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EfficiencySurfaceClaimStatus {
    /// Full standing: every axis certified, claimed efficiency-support tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, or
    /// the narrowing is inconsistent.
    Red,
}

impl EfficiencySurfaceClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the surface is publishable as certified (green or disclosed yellow);
    /// red surfaces block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The copy / export parity a certified surface preserves. The CLI/export axis
/// certifies only when this offers text / JSON / Markdown reconstruction and prohibits
/// a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The source-of-change / disposition / override / backlog / stale-result fields the
    /// surface preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl EfficiencyCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a
    /// screenshot-only export.
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
pub struct EfficiencyAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: EfficiencyCertificationAxis,
    /// The certification state of the axis.
    pub state: EfficiencyAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5EfficiencyDowngradeTrigger>,
}

impl EfficiencyAxisOutcome {
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
            EfficiencyAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            EfficiencyAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            EfficiencyAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a surface applies when a truth axis is not current.
/// Present iff the certified claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: EfficiencyCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5EfficiencyAccessClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5EfficiencyAccessClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 adaptive-efficiency surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencySurfaceCertificationRow {
    /// Record kind; must equal [`EFFICIENCY_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EFFICIENCY_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5EfficiencyCertifiedSurface,
    /// The efficiency-support claim ceiling the surface asserts.
    pub claimed_claim: M5EfficiencyAccessClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger
    /// than `claimed_claim`.
    pub certified_claim: M5EfficiencyAccessClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5EfficiencyComponentFamily>,
    /// One outcome per [`EfficiencyCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<EfficiencyAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<EfficiencyClaimAutoNarrow>,
    /// The one canonical efficiency-proof bundle this surface cites. Must equal
    /// [`EFFICIENCY_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: EfficiencySurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: EfficiencyCertExportParity,
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

impl EfficiencySurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: EfficiencyCertificationAxis) -> Option<&EfficiencyAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<EfficiencyCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && EfficiencyCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes.iter().all(EfficiencyAxisOutcome::well_formed)
    }

    /// True when the surface narrows its efficiency-support claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<EfficiencyCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == EfficiencyAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart
    /// of the capstone: a degraded axis must produce a visible claim narrowing,
    /// CLI/export parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> EfficiencySurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != EFFICIENCY_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return EfficiencySurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return EfficiencySurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(EfficiencyCertificationAxis::CliExport) {
            Some(o) if o.state == EfficiencyAxisCertificationState::Certified => {}
            _ => return EfficiencySurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == EfficiencyAxisCertificationState::UndisclosedDrift)
        {
            return EfficiencySurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return EfficiencySurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return EfficiencySurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return EfficiencySurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return EfficiencySurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return EfficiencySurfaceClaimStatus::Red;
        }

        EfficiencySurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == EFFICIENCY_CERT_ROW_RECORD_KIND
            && self.schema_version == EFFICIENCY_CERT_SCHEMA_VERSION
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
narrowed_axes={narrowed}",
            surface = self.surface.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
        )
    }
}

/// Rolled-up summary of an M05-1067 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencySurfaceCertificationSummary {
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
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`EfficiencySurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EfficiencySurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<EfficiencySurfaceCertificationRow>,
}

/// Checked-in M05-1067 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencySurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<EfficiencySurfaceCertificationRow>,
    pub summary: EfficiencySurfaceCertificationSummary,
}

impl EfficiencySurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: EfficiencySurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: EFFICIENCY_CERT_SCHEMA_VERSION,
            record_kind: EFFICIENCY_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: EfficiencySurfaceCertificationSummary {
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
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5EfficiencyCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5EfficiencyComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5EfficiencyCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface —
    /// proof the full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5EfficiencyComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(EfficiencyCertificationAxis::CliExport)
                .is_some_and(|o| o.state == EfficiencyAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> EfficiencySurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == EfficiencySurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == EfficiencySurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == EfficiencySurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(EfficiencySurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();

        EfficiencySurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == EFFICIENCY_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(EfficiencySurfaceCertificationRow::covers_all_axes),
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_surfaces && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<EfficiencyCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EFFICIENCY_CERT_SCHEMA_VERSION {
            violations.push(EfficiencyCertificationViolation::SchemaVersion {
                expected: EFFICIENCY_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EFFICIENCY_CERT_RECORD_KIND {
            violations.push(EfficiencyCertificationViolation::RecordKind {
                expected: EFFICIENCY_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(EfficiencyCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != EFFICIENCY_CERT_CANONICAL_BUNDLE_REF {
            violations.push(EfficiencyCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(EfficiencyCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(EfficiencyCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(EfficiencyCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(EfficiencyCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != EFFICIENCY_CERT_CANONICAL_BUNDLE_REF {
                violations.push(EfficiencyCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(EfficiencyCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(EfficiencyCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(EfficiencyCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(EfficiencyCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == EfficiencySurfaceClaimStatus::Red {
                violations.push(EfficiencyCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(EfficiencyCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(EfficiencyCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(EfficiencyCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(EfficiencyCertificationViolation::RawEfficiencyMaterialInExport);
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
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,binding_axis\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{surface},{claimed},{certified},{status},{narrowed},{binding}\n",
                id = row.row_id,
                surface = row.surface.as_str(),
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
        out.push_str("# M5 Adaptive-Efficiency Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Canonical bundle: `{}`\n", self.canonical_bundle_ref));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5EfficiencyCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
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
pub fn current_m5_efficiency_component_certification_export(
) -> Result<EfficiencySurfaceCertificationPacket, EfficiencyCertificationArtifactError> {
    let packet: EfficiencySurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-efficiency-component-certification-proof/support_export.json"
    )))
    .map_err(EfficiencyCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(EfficiencyCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum EfficiencyCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<EfficiencyCertificationViolation>),
}

impl fmt::Display for EfficiencyCertificationArtifactError {
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

impl Error for EfficiencyCertificationArtifactError {}

/// Validation failure for M05-1067 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EfficiencyCertificationViolation {
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
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawEfficiencyMaterialInExport,
}

impl fmt::Display for EfficiencyCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema version mismatch: expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::WrongCanonicalBundle => {
                write!(f, "packet does not cite the canonical efficiency-proof bundle")
            }
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::AxisCoverageIncomplete { id } => {
                write!(f, "row {id} does not score every certification axis exactly once")
            }
            Self::MalformedAxisOutcome { id } => {
                write!(
                    f,
                    "row {id} has an axis outcome whose disclosure fields disagree with its state"
                )
            }
            Self::RowMissingCanonicalBundle { id } => {
                write!(f, "row {id} does not cite the one canonical efficiency-proof bundle")
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::CertifiedClaimExceedsClaim { id } => {
                write!(f, "row {id} certifies a claim stronger than the claimed one")
            }
            Self::StatusDerivationStale { id } => {
                write!(f, "row {id} stored status disagrees with a fresh derivation")
            }
            Self::SurfaceBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a full claim, \
CLI/export parity dropped, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 adaptive-efficiency surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen adaptive-efficiency component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawEfficiencyMaterialInExport => {
                write!(f, "export contains raw efficiency material")
            }
        }
    }
}

impl Error for EfficiencyCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&EfficiencyAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != EfficiencyAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes
/// the adaptive-efficiency generics the spec forbids collapsing distinct pressure
/// sources into (whole-label matches so a full sentence naming "battery saver" as the
/// source of change is not flagged).
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
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "paused"
            | "slowed"
            | "slowed down"
            | "throttled"
            | "low power"
            | "low-power mode"
            | "power saver"
            | "battery saver"
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

/// Builds the canonical, checked-in M05-1067 certification packet. Certifies all eight
/// claimed M5 adaptive-efficiency surfaces: four deliver their claim (green) and four
/// auto-narrow a not-current truth axis to a weaker efficiency-support ceiling
/// (yellow). No surface hides drift (red).
pub fn seeded_m5_efficiency_component_certification_packet() -> EfficiencySurfaceCertificationPacket
{
    EfficiencySurfaceCertificationPacket::new(EfficiencySurfaceCertificationPacketInput {
        packet_id: "m5-efficiency-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-10T00:00:00Z".to_owned(),
        matrix_ref: EFFICIENCY_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: EFFICIENCY_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:efficiency-component-certification:{id}"),
        EFFICIENCY_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> EfficiencyCertExportParity {
    EfficiencyCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: EfficiencyCertificationAxis) -> &'static str {
    match axis {
        EfficiencyCertificationAxis::Visual => {
            "source of change, active efficiency state, slowed-versus-paused work, what still works, override availability, policy owner, resumed backlog, and stale-result continuity shown on-surface"
        }
        EfficiencyCertificationAxis::Keyboard => {
            "the same inspect / override / resume actions are keyboard-reachable"
        }
        EfficiencyCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only"
        }
        EfficiencyCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support replay"
        }
        EfficiencyCertificationAxis::DegradedState => {
            "a stale, deferred, or partial reading honestly downgrades the FullTruth/ResolvedTruth claim"
        }
        EfficiencyCertificationAxis::EfficiencyTruth => {
            "source of change, slowed-versus-paused work, override availability, policy owner, resumed backlog, and stale-result continuity stay explicit and never collapse into one generic warning"
        }
    }
}

fn seed_certified(axis: EfficiencyCertificationAxis) -> EfficiencyAxisOutcome {
    EfficiencyAxisOutcome {
        axis,
        state: EfficiencyAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: EfficiencyCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5EfficiencyDowngradeTrigger,
) -> EfficiencyAxisOutcome {
    EfficiencyAxisOutcome {
        axis,
        state: EfficiencyAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<EfficiencyAxisOutcome> {
    EfficiencyCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: EfficiencyCertificationAxis,
    outcome: EfficiencyAxisOutcome,
) -> Vec<EfficiencyAxisOutcome> {
    EfficiencyCertificationAxis::ALL
        .iter()
        .copied()
        .map(|a| if a == axis { outcome.clone() } else { seed_certified(a) })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn seed_row(
    row_id: &str,
    surface: M5EfficiencyCertifiedSurface,
    claimed_claim: M5EfficiencyAccessClaim,
    certified_claim: M5EfficiencyAccessClaim,
    consumed_families: &[M5EfficiencyComponentFamily],
    axis_outcomes: Vec<EfficiencyAxisOutcome>,
    claim_auto_narrow: Option<EfficiencyClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> EfficiencySurfaceCertificationRow {
    let mut row = EfficiencySurfaceCertificationRow {
        record_kind: EFFICIENCY_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: EFFICIENCY_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        canonical_bundle_ref: EFFICIENCY_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: EfficiencySurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes.iter().map(|n| (*n).to_owned()).collect(),
        source_refs: vec![
            EFFICIENCY_CERT_MATRIX_REF.to_owned(),
            EFFICIENCY_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-10T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: EfficiencyCertificationAxis,
    from_claim: M5EfficiencyAccessClaim,
    to_claim: M5EfficiencyAccessClaim,
    label: &str,
) -> EfficiencyClaimAutoNarrow {
    EfficiencyClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<EfficiencySurfaceCertificationRow> {
    use EfficiencyCertificationAxis as Ax;
    use M5EfficiencyAccessClaim::*;
    use M5EfficiencyCertifiedSurface as S;
    use M5EfficiencyComponentFamily::*;
    use M5EfficiencyDowngradeTrigger as Trig;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:shell-status-bar",
            S::ShellStatusBar,
            FullTruth,
            FullTruth,
            &[PowerStateIndicator, ThrottledSubsystemRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "source_of_change"],
            &[
                "power-state indicator names the source of change (battery saver / thermal / user low-power / policy cap) and the active efficiency state",
                "throttled-subsystem row names which subsystem is slowed and what still works",
                "keyboard/screen-reader reach preserved for the indicator and subsystem rows",
                "efficiency-truth: distinct pressure sources never collapse into one generic low-power warning",
            ],
        ),
        seed_row(
            "cert:activity-center",
            S::ActivityCenter,
            FullTruth,
            FullTruth,
            &[BackgroundWorkRow, BackgroundWorkBanner],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "work_disposition"],
            &[
                "background-work row names one job's slowed-versus-paused disposition and resume condition",
                "background-work banner names aggregate paused/slowed work durably, not toast-only",
                "export preserves the affected work class and slowed-versus-paused truth",
                "efficiency-truth: paused work is never hidden behind toast-only messaging",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            ResolvedTruth,
            ResolvedTruth,
            &[ResumeSummaryCard, StaleResultContinuityNote],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "resume_backlog"],
            &[
                "resume-summary card carries the resumed-work backlog and next safe action in reusable language",
                "stale-result continuity note names which results are still stale after resume",
                "text / JSON / Markdown reconstruction certified for support replay",
                "efficiency-truth: stale-result context is never cleared merely because work resumed",
            ],
        ),
        seed_row(
            "cert:docs-help",
            S::DocsHelp,
            ResolvedTruth,
            ResolvedTruth,
            &[OverridePolicyNoteRow, PowerStateIndicator],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "policy_owner"],
            &[
                "docs render the override-policy note row's policy owner identically",
                "power-state indicator documentation names each source of change, never implied as current live state",
                "export preserves the policy-owner and source-of-change truth",
                "efficiency-truth: docs never present a documented example as a live reading",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:work-content-canvas",
            S::WorkContentCanvas,
            FullTruth,
            Degraded,
            &[ThrottledSubsystemRow, BackgroundWorkRow],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the canvas's live power/thermal-adaptation proof aged out and is re-establishing",
                    "The notebook/pipeline canvas's live power-thermal adaptation proof has gone stale and is re-establishing, so the FullTruth claim narrows to degraded and the slowed-work reading shown is last-known rather than presented as current",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                FullTruth,
                Degraded,
                "Degraded adaptation reading: the live power/thermal proof is stale and re-establishing; the slowed-work state shown for this canvas is last-known",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "throttled-subsystem row keeps the affected subsystem and what-still-works visible through the stale window",
                "background-work row keeps the slowed-versus-paused disposition visible",
                "degraded-state: FullTruth narrows to degraded (auto-narrowed)",
                "efficiency-truth: the source of change stays explicit while the proof re-establishes",
            ],
        ),
        seed_row(
            "cert:policy-aware-settings",
            S::PolicyAwareSettings,
            FullTruth,
            PolicyBlocked,
            &[PerWorkspaceOverrideSheet, OverridePolicyNoteRow],
            seed_certified_except(
                Ax::EfficiencyTruth,
                seed_narrowed(
                    Ax::EfficiencyTruth,
                    "an admin policy cap blocks the override on this workspace",
                    "An admin policy cap blocks the override on this workspace, so the FullTruth claim narrows to policy-blocked and the sheet shows the blocking policy and its owner rather than presenting the override as available",
                    Trig::OverrideAvailabilityUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::EfficiencyTruth,
                FullTruth,
                PolicyBlocked,
                "Policy-blocked override: an admin policy cap holds this workspace; the blocking policy and its owner are shown and the override does not read as available",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "per-workspace override sheet names the current mode, allowed ceilings, and reset path",
                "override-policy note row names the accountable policy owner behind the cap",
                "efficiency-truth: FullTruth narrows to policy-blocked (auto-narrowed)",
                "an override blocked by policy is never presented as available",
            ],
        ),
        seed_row(
            "cert:incident-diagnostics",
            S::IncidentDiagnostics,
            FullTruth,
            Deferred,
            &[BackgroundWorkBanner, ThrottledSubsystemRow],
            seed_certified_except(
                Ax::EfficiencyTruth,
                seed_narrowed(
                    Ax::EfficiencyTruth,
                    "reindex work is paused under an incident throttle and shown from its last-known backlog",
                    "The diagnostics console shows reindex work paused under an incident throttle from its last-known backlog, so the FullTruth claim narrows to deferred and the paused work and its resume condition stay explicit rather than reading as live progress",
                    Trig::PausedWorkToastOnly,
                ),
            ),
            Some(seed_narrow(
                Ax::EfficiencyTruth,
                FullTruth,
                Deferred,
                "Deferred work: reindex is paused under the incident throttle; the paused work and its resume condition are shown durably from the last-known backlog, not as live progress",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "background-work banner keeps the aggregate paused work durable and reviewable",
                "throttled-subsystem row names the subsystem held by the incident throttle",
                "efficiency-truth: FullTruth narrows to deferred (auto-narrowed)",
                "paused work is shown durably, never behind a dismissed toast",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            ResolvedTruth,
            StaleShown,
            &[StaleResultContinuityNote, ResumeSummaryCard],
            seed_certified_except(
                Ax::EfficiencyTruth,
                seed_narrowed(
                    Ax::EfficiencyTruth,
                    "the headless export surfaces a stale result kept visible pending refresh",
                    "The headless export surfaces a stale result deliberately kept visible pending refresh, so the ResolvedTruth claim narrows to stale-shown and stale-result continuity plus the next safe action are preserved rather than cleared",
                    Trig::StaleResultContinuityCleared,
                ),
            ),
            Some(seed_narrow(
                Ax::EfficiencyTruth,
                ResolvedTruth,
                StaleShown,
                "Stale-shown result: a prior result is deliberately kept visible pending refresh; stale-result continuity and the next safe action are preserved in the headless output",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "stale-result continuity note names which results remain stale in the CLI output",
                "resume-summary card keeps the resumed-work backlog explicit in the structured output",
                "efficiency-truth: ResolvedTruth narrows to stale-shown (auto-narrowed)",
                "CLI/export parity certified so automation can replay the stale-result continuity",
            ],
        ),
    ]
}
