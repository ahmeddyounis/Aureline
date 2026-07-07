//! M05-907 surface certification over the frozen M5 support-intake / escalation component
//! matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix`])
//! defines the five reusable support-scenario-picker-row, issue-report-builder-step,
//! escalation-packet-summary, handoff-timeline-row, and unsafe-fix-blocked-note components,
//! the M05-901..904 primitive lanes narrow each one, the M05-905 consumer lane
//! ([`crate::add_shared_doctor_safe_mode_bisect_support_center_docs_help_and_export_consumers_so_support_intake_components_keep_scenario_code_repair_lineage_and_redaction_parity_across_claimed_m5_profiles`])
//! proves they are reusable across the claimed Doctor / safe-mode / bisect / support-center /
//! docs-help / export consumers, and the M05-906 accessibility / auto-narrowing capstone
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_scenario_classification_is_uncertain_evidence_classes_are_omitted_destination_is_local_only_or_repair_guidance_is_policy_blocked_across_claimed_m5_support_components`])
//! certifies keyboard / screen-reader / CLI / export parity per family, this closing capstone
//! *certifies* that the shared support-intake / escalation component truth holds on every
//! claimed M5 supportability surface — and auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user starts diagnosis from, reviews a suggested
//! repair on, builds a report on, or escalates a case from (Doctor results, safe mode,
//! extension bisect, the support center, Help / docs, the support-bundle preview, the CLI,
//! and support / export), not on component family or primitive lane. Each
//! [`SupportSurfaceCertificationRow`] certifies one surface across six truth axes — visual,
//! keyboard, screen-reader, CLI/export, degraded-state, and support-intake / escalation
//! provenance — and either passes (green), auto-narrows its support claim to the weakest
//! supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind a
//! full-truth claim inherited from a healthier support lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `ReadyToEscalate` / `ReviewableCase` claim while one of its truth
//! axes is not current — the scenario classification is uncertain, one or more evidence
//! classes were omitted, the packet destination is local-only, the approved-repair guidance
//! is policy-blocked, or the next-human-step / owner continuity is unstated — is over-claiming
//! and blocks; a surface that discloses the reduction by narrowing its support claim (with a
//! bound reason and a frozen downgrade trigger) is honestly yellow. Escalation never loses
//! lineage: a narrowed case always preserves its scenario-code / Doctor-finding / packet
//! lineage continuity rather than dropping it between local diagnosis and human handoff. The
//! always-on CLI/export axis must always stay certified, so support and automation can
//! reconstruct the same scenario-code / incident-scope / evidence-class / finding-lineage /
//! approved-repair / packet-destination / next-human-step truth from the same support
//! identity the user saw.
//!
//! Every row cites exactly one canonical support-intake / escalation component proof bundle
//! ([`SUPPORT_CERT_CANONICAL_BUNDLE_REF`]) — the frozen component matrix release proof —
//! rather than cloning per-surface evidence. The packet is metadata-only: raw logs, report
//! bodies, redacted evidence contents, and credentials never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-support-intake-escalation-component-certification.schema.json`](../../../../schemas/ui/m5-support-intake-escalation-component-certification.schema.json).
//! The contract doc is
//! [`docs/support/m5_support_intake_escalation_component_certification_contract.md`](../../../../docs/support/m5_support_intake_escalation_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_doctor_safe_mode_bisect_support_center_docs_help_and_export_consumers_so_support_intake_components_keep_scenario_code_repair_lineage_and_redaction_parity_across_claimed_m5_profiles as consumers;
// The frozen matrix module name carries "unsafe_fix_blocked_note"; aliasing it to `matrix`
// is a deliberate shorthand, not an unsafe-name removal.
#[allow(clippy::unsafe_removed_from_name)]
use crate::freeze_the_m5_support_scenario_picker_row_issue_report_builder_step_escalation_packet_summary_handoff_timeline_row_and_unsafe_fix_blocked_note_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_scenario_classification_is_uncertain_evidence_classes_are_omitted_destination_is_local_only_or_repair_guidance_is_policy_blocked_across_claimed_m5_support_components as a11y;
use a11y::M5SupportIntakeClaim;
use matrix::{M5SupportDowngradeTrigger, M5SupportIntakeEscalationComponentFamily};

/// Schema version stamped on the M05-907 certification packet.
pub const SUPPORT_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`SupportSurfaceCertificationPacket`].
pub const SUPPORT_CERT_RECORD_KIND: &str =
    "m5_support_intake_escalation_component_certification_packet";

/// Stable record-kind tag carried by each [`SupportSurfaceCertificationRow`].
pub const SUPPORT_CERT_ROW_RECORD_KIND: &str =
    "m5_support_intake_escalation_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const SUPPORT_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-support-intake-escalation-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const SUPPORT_CERT_DOC_REF: &str =
    "docs/support/m5_support_intake_escalation_component_certification_contract.md";

/// Repo-relative path of the frozen support-intake / escalation component matrix schema the
/// certified surfaces render.
pub const SUPPORT_CERT_MATRIX_REF: &str = matrix::M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_SCHEMA_REF;

/// The one canonical support-intake / escalation component proof bundle every certified
/// surface cites as its first-resolved component truth. All eight surfaces point back to it
/// rather than cloning per-surface evidence.
pub const SUPPORT_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_ARTIFACT_REF;

/// The M05-905 consumer-adoption support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const SUPPORT_CERT_CONSUMER_BUNDLE_REF: &str =
    consumers::M5_SUPPORT_INTAKE_ESCALATION_COMPONENT_CONSUMER_ARTIFACT_REF;

/// The M05-906 accessibility / auto-narrowing support export whose keyboard / screen-reader
/// / CLI / export parity this capstone builds on. Recorded as a supporting evidence ref on
/// every row.
pub const SUPPORT_CERT_A11Y_BUNDLE_REF: &str =
    a11y::SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const SUPPORT_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-support-intake-escalation-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const SUPPORT_CERT_CSV_REF: &str =
    "artifacts/release/m5-support-intake-escalation-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const SUPPORT_CERT_REPORT_REF: &str =
    "artifacts/release/m5-support-intake-escalation-component-certification/report.md";

/// The eight claimed M5 supportability surfaces this capstone certifies. Keyed on the surface
/// a user actually starts diagnosis from, reviews a repair on, builds a report on, or
/// escalates a case from, not on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportIntakeEscalationCertifiedSurface {
    /// The Project Doctor results surface.
    DoctorResults,
    /// The safe-mode runtime surface.
    SafeMode,
    /// The extension-bisect surface.
    ExtensionBisect,
    /// The support-center surface.
    SupportCenter,
    /// The Help / docs surface.
    DocsHelp,
    /// The support-bundle preview surface.
    SupportBundlePreview,
    /// The CLI / headless surface.
    CliHeadless,
    /// The support / export bundle surface.
    SupportExport,
}

impl M5SupportIntakeEscalationCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5SupportIntakeEscalationCertifiedSurface; 8] = [
        M5SupportIntakeEscalationCertifiedSurface::DoctorResults,
        M5SupportIntakeEscalationCertifiedSurface::SafeMode,
        M5SupportIntakeEscalationCertifiedSurface::ExtensionBisect,
        M5SupportIntakeEscalationCertifiedSurface::SupportCenter,
        M5SupportIntakeEscalationCertifiedSurface::DocsHelp,
        M5SupportIntakeEscalationCertifiedSurface::SupportBundlePreview,
        M5SupportIntakeEscalationCertifiedSurface::CliHeadless,
        M5SupportIntakeEscalationCertifiedSurface::SupportExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DoctorResults => "doctor_results",
            Self::SafeMode => "safe_mode",
            Self::ExtensionBisect => "extension_bisect",
            Self::SupportCenter => "support_center",
            Self::DocsHelp => "docs_help",
            Self::SupportBundlePreview => "support_bundle_preview",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader, CLI/export,
/// degraded-state, and support-intake / escalation provenance. The CLI/export axis is
/// always-on and must stay certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportCertificationAxis {
    /// Visual parity: scenario family, incident scope, selected / omitted evidence classes,
    /// Doctor finding lineage, approved repair class, packet destination, and next human step
    /// are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same scenario / scope / evidence / finding / repair /
    /// destination / next-step truth and its controls are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color
    /// or a status glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is reconstructable as
    /// text / JSON / Markdown for support and automation, from the same support identity.
    CliExport,
    /// Degraded-state parity: an uncertain scenario, an omitted evidence class, a local-only
    /// destination, or a policy-blocked repair honestly downgrades a `ReadyToEscalate` /
    /// `ReviewableCase` claim to a weaker support tier.
    DegradedState,
    /// Support-intake / escalation provenance parity: scenario family, incident scope,
    /// selected / omitted evidence classes, Doctor finding lineage, approved repair class,
    /// packet destination, and next human step stay explicit before any diagnosis start,
    /// suggested-repair review, report build, or escalation — never inheriting a healthier
    /// lane's support truth, never masking an uncertain scenario, omitted evidence,
    /// local-only destination, policy-blocked repair, or unstated next step as a
    /// ready-to-escalate case, and never dropping scenario / finding / packet lineage between
    /// local diagnosis and human handoff.
    SupportIntakeAndEscalationProvenance,
}

impl SupportCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [SupportCertificationAxis; 6] = [
        SupportCertificationAxis::Visual,
        SupportCertificationAxis::Keyboard,
        SupportCertificationAxis::ScreenReader,
        SupportCertificationAxis::CliExport,
        SupportCertificationAxis::DegradedState,
        SupportCertificationAxis::SupportIntakeAndEscalationProvenance,
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
            Self::SupportIntakeAndEscalationProvenance => {
                "support_intake_and_escalation_provenance"
            }
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible
    /// claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim inherited
    /// from a healthier surface.
    UndisclosedDrift,
}

impl SupportAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole surface. Never asserted by the author —
/// always recomputed from the axis outcomes and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed support tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, lineage is
    /// dropped, or the narrowing is inconsistent.
    Red,
}

impl SupportSurfaceClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the surface is certifiable as shipped (green or disclosed yellow); red
    /// surfaces block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The copy / export parity a certified surface preserves. The CLI/export axis certifies only
/// when this offers text / JSON / Markdown reconstruction and prohibits a screenshot-only
/// export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The scenario / scope / evidence / finding / repair / destination / next-step fields the
    /// surface preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl SupportCertExportParity {
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
pub struct SupportAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: SupportCertificationAxis,
    /// The certification state of the axis.
    pub state: SupportAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5SupportDowngradeTrigger>,
}

impl SupportAxisOutcome {
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
            SupportAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            SupportAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            SupportAxisCertificationState::UndisclosedDrift => {
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
pub struct SupportClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: SupportCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5SupportIntakeClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5SupportIntakeClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
    /// True when the narrowed case still preserves its scenario / finding / packet lineage
    /// continuity rather than dropping it between local diagnosis and human handoff.
    pub preserves_lineage_continuity: bool,
}

/// One certified M5 supportability surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportSurfaceCertificationRow {
    /// Record kind; must equal [`SUPPORT_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SUPPORT_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5SupportIntakeEscalationCertifiedSurface,
    /// The support-claim ceiling the surface asserts.
    pub claimed_claim: M5SupportIntakeClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5SupportIntakeClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5SupportIntakeEscalationComponentFamily>,
    /// One outcome per [`SupportCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<SupportAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<SupportClaimAutoNarrow>,
    /// True when this surface never drops its scenario / finding / packet lineage continuity
    /// between local diagnosis and human handoff.
    pub lineage_preserved: bool,
    /// The one canonical support-intake / escalation proof bundle this surface cites. Must
    /// equal [`SUPPORT_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: SupportSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: SupportCertExportParity,
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

impl SupportSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: SupportCertificationAxis) -> Option<&SupportAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<SupportCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && SupportCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(SupportAxisOutcome::well_formed)
    }

    /// True when the surface narrows its support claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<SupportCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == SupportAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether a narrowed case preserves its scenario / finding / packet lineage continuity
    /// rather than dropping it. A non-narrowed surface trivially preserves lineage; a narrowed
    /// one must say so.
    pub fn preserves_lineage_continuity(&self) -> bool {
        match &self.claim_auto_narrow {
            Some(narrow) => self.lineage_preserved && narrow.preserves_lineage_continuity,
            None => self.lineage_preserved,
        }
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, CLI/export parity must
    /// always certify, escalation must never drop lineage, and the narrowing must be
    /// consistent.
    pub fn derive_status(&self) -> SupportSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != SUPPORT_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.preserves_lineage_continuity()
        {
            return SupportSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return SupportSurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(SupportCertificationAxis::CliExport) {
            Some(o) if o.state == SupportAxisCertificationState::Certified => {}
            _ => return SupportSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == SupportAxisCertificationState::UndisclosedDrift)
        {
            return SupportSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return SupportSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return SupportSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                    || !narrow.preserves_lineage_continuity
                {
                    return SupportSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return SupportSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return SupportSurfaceClaimStatus::Red;
        }

        SupportSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == SUPPORT_CERT_ROW_RECORD_KIND
            && self.schema_version == SUPPORT_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-907 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportSurfaceCertificationSummary {
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

/// Constructor input for [`SupportSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SupportSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<SupportSurfaceCertificationRow>,
}

/// Checked-in M05-907 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<SupportSurfaceCertificationRow>,
    pub summary: SupportSurfaceCertificationSummary,
}

impl SupportSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: SupportSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: SUPPORT_CERT_SCHEMA_VERSION,
            record_kind: SUPPORT_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: SupportSurfaceCertificationSummary {
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
    pub fn represented_surfaces(&self) -> BTreeSet<M5SupportIntakeEscalationCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5SupportIntakeEscalationComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5SupportIntakeEscalationCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof the
    /// full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5SupportIntakeEscalationComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(SupportCertificationAxis::CliExport)
                .is_some_and(|o| o.state == SupportAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> SupportSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == SupportSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == SupportSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == SupportSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(SupportSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_preserved = self
            .rows
            .iter()
            .all(SupportSurfaceCertificationRow::preserves_lineage_continuity);

        SupportSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == SUPPORT_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(SupportSurfaceCertificationRow::covers_all_axes),
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
    pub fn validate(&self) -> Vec<SupportCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != SUPPORT_CERT_SCHEMA_VERSION {
            violations.push(SupportCertificationViolation::SchemaVersion {
                expected: SUPPORT_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != SUPPORT_CERT_RECORD_KIND {
            violations.push(SupportCertificationViolation::RecordKind {
                expected: SUPPORT_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(SupportCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != SUPPORT_CERT_CANONICAL_BUNDLE_REF {
            violations.push(SupportCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(SupportCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(SupportCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(SupportCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(SupportCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != SUPPORT_CERT_CANONICAL_BUNDLE_REF {
                violations.push(SupportCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(SupportCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(SupportCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Escalation must never drop lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(SupportCertificationViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(SupportCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(SupportCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == SupportSurfaceClaimStatus::Red {
                violations.push(SupportCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(SupportCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(SupportCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(SupportCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(SupportCertificationViolation::RawSupportMaterialInExport);
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
        out.push_str("# M5 Support-Intake / Escalation Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5SupportIntakeEscalationCertifiedSurface::ALL.len(),
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
pub fn current_m5_support_intake_escalation_component_certification_export(
) -> Result<SupportSurfaceCertificationPacket, SupportCertificationArtifactError> {
    let packet: SupportSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-support-intake-escalation-component-certification/support_export.json"
    )))
    .map_err(SupportCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(SupportCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum SupportCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<SupportCertificationViolation>),
}

impl fmt::Display for SupportCertificationArtifactError {
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

impl Error for SupportCertificationArtifactError {}

/// Validation failure for M05-907 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SupportCertificationViolation {
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
    RawSupportMaterialInExport,
}

impl fmt::Display for SupportCertificationViolation {
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
                    "packet does not cite the canonical support-intake / escalation proof bundle"
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
                    "row {id} does not cite the one canonical support-intake / escalation proof bundle"
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
                    "row {id} drops scenario / finding / packet lineage continuity (a narrowed case must preserve its lineage between local diagnosis and human handoff)"
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
                    "not every claimed M5 supportability surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen support-intake / escalation component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawSupportMaterialInExport => {
                write!(f, "export contains raw support material")
            }
        }
    }
}

impl Error for SupportCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&SupportAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != SupportAxisCertificationState::Certified,
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
            | "unclassified"
            | "local only"
            | "local_only"
            | "policy blocked"
            | "policy_blocked"
            | "omitted"
            | "evidence omitted"
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

/// Builds the canonical, checked-in M05-907 certification packet. Certifies all eight claimed
/// M5 supportability surfaces: four deliver their claim (green) and four auto-narrow a
/// not-current truth axis to a weaker support ceiling (yellow). No surface hides drift (red),
/// and no surface drops scenario / finding / packet lineage on escalation.
pub fn seeded_m5_support_intake_escalation_component_certification_packet(
) -> SupportSurfaceCertificationPacket {
    SupportSurfaceCertificationPacket::new(SupportSurfaceCertificationPacketInput {
        packet_id: "m5-support-intake-escalation-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: SUPPORT_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: SUPPORT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:support-intake-escalation-certification:{id}"),
        SUPPORT_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        SUPPORT_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> SupportCertExportParity {
    SupportCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: SupportCertificationAxis) -> &'static str {
    match axis {
        SupportCertificationAxis::Visual => {
            "scenario family, incident scope, selected/omitted evidence classes, Doctor finding lineage, approved repair class, packet destination, and next human step shown on-surface"
        }
        SupportCertificationAxis::Keyboard => {
            "the same scenario/scope/evidence/finding/repair/destination/next-step truth and its controls are keyboard-reachable"
        }
        SupportCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only"
        }
        SupportCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support from the same support identity"
        }
        SupportCertificationAxis::DegradedState => {
            "an uncertain scenario, an omitted evidence class, a local-only destination, or a policy-blocked repair honestly downgrades the ReadyToEscalate/ReviewableCase claim"
        }
        SupportCertificationAxis::SupportIntakeAndEscalationProvenance => {
            "scenario family, incident scope, selected/omitted evidence classes, Doctor finding lineage, approved repair class, packet destination, and next human step stay explicit before any diagnosis start, repair review, report build, or escalation; escalation never drops lineage"
        }
    }
}

fn seed_certified(axis: SupportCertificationAxis) -> SupportAxisOutcome {
    SupportAxisOutcome {
        axis,
        state: SupportAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: SupportCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5SupportDowngradeTrigger,
) -> SupportAxisOutcome {
    SupportAxisOutcome {
        axis,
        state: SupportAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<SupportAxisOutcome> {
    SupportCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: SupportCertificationAxis,
    outcome: SupportAxisOutcome,
) -> Vec<SupportAxisOutcome> {
    SupportCertificationAxis::ALL
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
    surface: M5SupportIntakeEscalationCertifiedSurface,
    claimed_claim: M5SupportIntakeClaim,
    certified_claim: M5SupportIntakeClaim,
    consumed_families: &[M5SupportIntakeEscalationComponentFamily],
    axis_outcomes: Vec<SupportAxisOutcome>,
    claim_auto_narrow: Option<SupportClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> SupportSurfaceCertificationRow {
    let mut row = SupportSurfaceCertificationRow {
        record_kind: SUPPORT_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: SUPPORT_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        lineage_preserved: true,
        canonical_bundle_ref: SUPPORT_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: SupportSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            SUPPORT_CERT_MATRIX_REF.to_owned(),
            SUPPORT_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-07T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: SupportCertificationAxis,
    from_claim: M5SupportIntakeClaim,
    to_claim: M5SupportIntakeClaim,
    label: &str,
) -> SupportClaimAutoNarrow {
    SupportClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
        preserves_lineage_continuity: true,
    }
}

fn seeded_rows() -> Vec<SupportSurfaceCertificationRow> {
    use M5SupportDowngradeTrigger as Trig;
    use M5SupportIntakeClaim::*;
    use M5SupportIntakeEscalationCertifiedSurface as S;
    use M5SupportIntakeEscalationComponentFamily::*;
    use SupportCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:doctor-results",
            S::DoctorResults,
            ReadyToEscalate,
            ReadyToEscalate,
            &[SupportScenarioPickerRow, EscalationPacketSummary],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "scenario_code"],
            &[
                "the support-scenario picker row maps the Doctor symptom to a stable scenario family, incident scope, and bound Doctor finding family before diagnosis starts",
                "the escalation-packet summary keeps its packet destination and next human step explicit once a case is assembled",
                "keyboard/screen-reader reach preserved for the scenario picker row and the escalation-packet summary",
                "provenance: a Doctor-driven case never leaves the scenario code or its Doctor finding lineage implicit before escalation",
            ],
        ),
        seed_row(
            "cert:support-center",
            S::SupportCenter,
            ReadyToEscalate,
            ReadyToEscalate,
            &[IssueReportBuilderStep, HandoffTimelineRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "next_human_step"],
            &[
                "the issue-report builder step keeps its selected and omitted evidence classes explicit with no hidden upload requirement",
                "the handoff-timeline row keeps its handoff stage, owner, and next human step explicit",
                "keyboard/screen-reader reach preserved for the report builder step and the handoff-timeline row",
                "provenance: a support-center case never loses the next human step or the owner accountable for it",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            ReviewableCase,
            ReviewableCase,
            &[EscalationPacketSummary, HandoffTimelineRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "packet_destination"],
            &[
                "support export reconstructs scenario-code/scope/evidence-class/finding-lineage/approved-repair/destination/next-step truth from the same support identity",
                "the escalation-packet summary keeps its packet destination and redaction state explicit with no hidden sharing",
                "the handoff-timeline row keeps its handoff stage and next human step explicit in the exported packet",
                "provenance: a support packet never exports raw logs, report bodies, or redacted evidence contents",
            ],
        ),
        seed_row(
            "cert:safe-mode",
            S::SafeMode,
            ReviewableCase,
            ReviewableCase,
            &[SupportScenarioPickerRow, HandoffTimelineRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "incident_scope"],
            &[
                "the support-scenario picker row keeps the safe-mode scenario family and incident scope explicit before diagnosis starts",
                "the handoff-timeline row keeps its handoff stage and next human step explicit while running under safe mode",
                "keyboard/screen-reader reach preserved for the scenario picker row and the handoff-timeline row",
                "provenance: a safe-mode case never leaves its scenario code or the next human step implicit",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:extension-bisect",
            S::ExtensionBisect,
            ReadyToEscalate,
            UnclassifiedScenario,
            &[SupportScenarioPickerRow, IssueReportBuilderStep],
            seed_certified_except(
                Ax::SupportIntakeAndEscalationProvenance,
                seed_narrowed(
                    Ax::SupportIntakeAndEscalationProvenance,
                    "the bisect symptom has not yet mapped to a stable scenario family and scope",
                    "The extension-bisect session has not resolved the symptom to a stable scenario family or Doctor finding, so the ReadyToEscalate claim narrows to unclassified-scenario instead of escalating a case whose scenario code is still uncertain",
                    Trig::ScenarioOrScopeUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::SupportIntakeAndEscalationProvenance,
                ReadyToEscalate,
                UnclassifiedScenario,
                "Scenario still resolving: the bisect has not mapped the symptom to a Doctor finding family; the scenario picker row shows it must be classified before the case is trusted",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the support-scenario picker row keeps the unmapped-scenario reason explicit and offers continued classification",
                "the issue-report builder step keeps its selected evidence classes explicit while the scenario is still uncertain",
                "support-intake/escalation: ReadyToEscalate narrows to unclassified-scenario (auto-narrowed)",
                "known compatibility note: uncertain-scenario behavior — an unmapped bisect scenario never reads as escalation-ready",
            ],
        ),
        seed_row(
            "cert:docs-help",
            S::DocsHelp,
            ReviewableCase,
            EvidenceIncompleteCase,
            &[IssueReportBuilderStep, EscalationPacketSummary],
            seed_certified_except(
                Ax::SupportIntakeAndEscalationProvenance,
                seed_narrowed(
                    Ax::SupportIntakeAndEscalationProvenance,
                    "one or more evidence classes were omitted from the report the docs flow assembled",
                    "The docs/help issue-report flow omitted one or more evidence classes the reviewer needs, so the ReviewableCase claim narrows to evidence-incomplete-case instead of implying the case carries the full evidence a reviewer can read",
                    Trig::EvidenceClassMasked,
                ),
            ),
            Some(seed_narrow(
                Ax::SupportIntakeAndEscalationProvenance,
                ReviewableCase,
                EvidenceIncompleteCase,
                "Evidence incomplete: the docs-help report omitted one or more evidence classes; the report builder step names exactly which class is missing rather than implying a full case",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the issue-report builder step keeps the omitted evidence class explicit rather than hiding the gap",
                "the escalation-packet summary keeps the case marked evidence-incomplete rather than reviewable",
                "support-intake/escalation: ReviewableCase narrows to evidence-incomplete-case (auto-narrowed)",
                "known compatibility note: evidence-omitted behavior — an evidence-incomplete docs-help case never reads as a full reviewable case",
            ],
        ),
        seed_row(
            "cert:support-bundle-preview",
            S::SupportBundlePreview,
            ReadyToEscalate,
            LocalOnlyDiagnosis,
            &[EscalationPacketSummary, HandoffTimelineRow],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the previewed packet destination is local-only and cannot be shared or uploaded",
                    "The support-bundle preview resolves a local-only packet destination, so the ReadyToEscalate claim narrows to local-only-diagnosis instead of implying the case can be shared or uploaded to a human reviewer",
                    Trig::PacketDestinationUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReadyToEscalate,
                LocalOnlyDiagnosis,
                "Local-only destination: the previewed support bundle stays on this device; the escalation-packet summary shows it cannot be shared rather than offering an upload",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the escalation-packet summary keeps the local-only destination explicit rather than implying a share",
                "the handoff-timeline row keeps its next human step explicit while the case stays a local self-diagnosis",
                "degraded-state: ReadyToEscalate narrows to local-only-diagnosis (auto-narrowed)",
                "known compatibility note: local-only destination — a local-only support bundle never reads as escalation-ready",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            ReadyToEscalate,
            PolicyBlockedRepair,
            &[UnsafeFixBlockedNote, SupportScenarioPickerRow],
            seed_certified_except(
                Ax::SupportIntakeAndEscalationProvenance,
                seed_narrowed(
                    Ax::SupportIntakeAndEscalationProvenance,
                    "the suggested repair is policy-blocked and no approved repair path can proceed",
                    "The CLI-headless repair review surfaces a policy-blocked fix with no approved repair class, so the ReadyToEscalate claim narrows to policy-blocked-repair instead of implying an approved repair can proceed",
                    Trig::ApprovedRepairClassMasked,
                ),
            ),
            Some(seed_narrow(
                Ax::SupportIntakeAndEscalationProvenance,
                ReadyToEscalate,
                PolicyBlockedRepair,
                "Repair policy-blocked: the CLI fix has no approved repair class; the unsafe-fix blocked note explains why the action is blocked rather than implying a safe repair",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the unsafe-fix blocked note keeps its block reason and the absent approved repair class explicit rather than implying a destructive repair",
                "the support-scenario picker row keeps its scenario code explicit while the repair stays blocked",
                "support-intake/escalation: ReadyToEscalate narrows to policy-blocked-repair (auto-narrowed)",
                "known compatibility note: policy-blocked repair — a policy-blocked CLI fix never reads as an approved repair",
            ],
        ),
    ]
}
