//! M05-1059 surface certification over the frozen M5 governance-dashboard
//! component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix`])
//! defines the nine reusable fitness-dashboard-tile, governance-report-row,
//! waiver-expiry-queue-item, release-gate-banner, mitigation-note-card,
//! service-ownership-card, on-call-strip, decision-right-card, and
//! milestone-dashboard-row components, the M05-1053..1056 implement lanes narrow
//! each one, the M05-1057 consumer lane adopts them, and the M05-1058
//! accessibility lane
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_evidence_freshness_waiver_expiry_owner_coverage_support_class_or_decision_right_truth_is_stale_or_partial_across_claimed_m5_governance_dashboard_components`])
//! proves keyboard / screen-reader / CLI-export parity and per-family
//! auto-narrowing, this closing capstone *certifies* that the shared component
//! truth holds on every claimed M5 assurance, operator, and shiproom surface — and
//! auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user or operator reads readiness,
//! ownership, or a ship/no-ship decision through (assurance center, operator
//! overview, release center, shiproom, service health, support/export, docs/help,
//! and CLI/headless), not on component family or implement lane. Each
//! [`GovernanceSurfaceCertificationRow`] certifies one surface across six truth
//! axes — visual, keyboard, screen-reader, CLI/export, degraded-state, and
//! governance-truth behavior — and either passes (green), auto-narrows its
//! governance-support claim to the weakest supported ceiling (yellow), or is
//! blocked (red) when a degraded axis is hidden behind a full-truth claim
//! inherited from a healthier surface.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `GovernedPass` / `GovernedResolved` claim while one of
//! its truth axes is not current is over-claiming and blocks; a surface that
//! discloses the reduction by narrowing its governance-support claim (with a bound
//! reason and a frozen downgrade trigger) is honestly yellow. The always-on
//! CLI/export axis must always stay certified, so support and automation can
//! reconstruct the certified fitness / ownership / waiver / decision truth from the
//! same object identity the user saw.
//!
//! Every row cites exactly one canonical governance-proof bundle
//! ([`GOVERNANCE_CERT_CANONICAL_BUNDLE_REF`]) — the frozen governance-dashboard
//! component matrix proof — rather than cloning per-surface evidence. The packet is
//! metadata-only: raw evidence, waiver credentials, owner contact detail, and
//! escalation secrets never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-governance-dashboard-component-certification.schema.json`](../../../../schemas/ui/m5-governance-dashboard-component-certification.schema.json).
//! The contract doc is
//! [`docs/help/m5_governance_dashboard_component_certification_contract.md`](../../../../docs/help/m5_governance_dashboard_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_fitness_dashboard_tile_governance_report_row_waiver_expiry_queue_item_release_gate_banner_mitigation_note_card_service_ownership_card_on_call_strip_decision_right_card_and_milestone_dashboard_row_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_evidence_freshness_waiver_expiry_owner_coverage_support_class_or_decision_right_truth_is_stale_or_partial_across_claimed_m5_governance_dashboard_components as a11y;
use a11y::M5GovernanceSupportClaim;
use matrix::{M5GovernanceDashboardComponentFamily, M5GovernanceDowngradeTrigger};

/// Schema version stamped on the M05-1059 certification packet.
pub const GOVERNANCE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`GovernanceSurfaceCertificationPacket`].
pub const GOVERNANCE_CERT_RECORD_KIND: &str =
    "m5_governance_dashboard_component_certification_packet";

/// Stable record-kind tag carried by each [`GovernanceSurfaceCertificationRow`].
pub const GOVERNANCE_CERT_ROW_RECORD_KIND: &str =
    "m5_governance_dashboard_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const GOVERNANCE_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-governance-dashboard-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const GOVERNANCE_CERT_DOC_REF: &str =
    "docs/help/m5_governance_dashboard_component_certification_contract.md";

/// Repo-relative path of the frozen governance-dashboard component matrix schema
/// the certified surfaces render.
pub const GOVERNANCE_CERT_MATRIX_REF: &str = matrix::M5_GOVERNANCE_DASHBOARD_SCHEMA_REF;

/// The one canonical governance-proof bundle every certified surface cites as its
/// first-resolved component truth. All eight surfaces point back to it rather than
/// cloning per-surface evidence.
pub const GOVERNANCE_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_GOVERNANCE_DASHBOARD_ARTIFACT_REF;

/// The M05-1058 accessibility support export the certification builds on. Recorded
/// as a supporting evidence ref on every row.
pub const GOVERNANCE_CERT_A11Y_BUNDLE_REF: &str = a11y::GOVERNANCE_A11Y_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const GOVERNANCE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-governance-dashboard-component-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const GOVERNANCE_CERT_CSV_REF: &str =
    "artifacts/release/m5-governance-dashboard-component-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const GOVERNANCE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-governance-dashboard-component-certification-proof/report.md";

/// The eight claimed M5 assurance / operator / shiproom surfaces this capstone
/// certifies. Keyed on the surface a user or operator reads readiness, ownership,
/// or a ship/no-ship decision through, not on the reusable component family it
/// renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceCertifiedSurface {
    /// The assurance-center dashboards where fitness and governance readiness read.
    AssuranceCenter,
    /// The operator overview boards where ownership and on-call coverage read.
    OperatorOverview,
    /// The release-center truth surface where gates and milestones read.
    ReleaseCenter,
    /// The shiproom packet where decision rights and blockers are routed.
    Shiproom,
    /// The service-health surface where ownership and escalation read.
    ServiceHealth,
    /// The support / export bundle surface.
    SupportExport,
    /// The docs / help surface.
    DocsHelp,
    /// The CLI / headless surface.
    CliHeadless,
}

impl M5GovernanceCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5GovernanceCertifiedSurface; 8] = [
        M5GovernanceCertifiedSurface::AssuranceCenter,
        M5GovernanceCertifiedSurface::OperatorOverview,
        M5GovernanceCertifiedSurface::ReleaseCenter,
        M5GovernanceCertifiedSurface::Shiproom,
        M5GovernanceCertifiedSurface::ServiceHealth,
        M5GovernanceCertifiedSurface::SupportExport,
        M5GovernanceCertifiedSurface::DocsHelp,
        M5GovernanceCertifiedSurface::CliHeadless,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssuranceCenter => "assurance_center",
            Self::OperatorOverview => "operator_overview",
            Self::ReleaseCenter => "release_center",
            Self::Shiproom => "shiproom",
            Self::ServiceHealth => "service_health",
            Self::SupportExport => "support_export",
            Self::DocsHelp => "docs_help",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the
/// parity dimensions the spec requires verifying — visual, keyboard,
/// screen-reader, CLI/export, degraded-state, and governance-truth behavior. The
/// CLI/export axis is always-on and must stay certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceCertificationAxis {
    /// Visual parity: fitness reading / provenance, readiness state, waiver expiry,
    /// owner / backup coverage, on-call / escalation route, decision forum, and
    /// blocker / waiver counts are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same fitness / ownership / waiver / decision truth
    /// and its actions are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying
    /// on color or a badge glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is
    /// reconstructable as text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: stale evidence or a partial proof honestly downgrades a
    /// `GovernedPass` / `GovernedResolved` claim to degraded / provisional rather than
    /// reading current.
    DegradedState,
    /// Governance-truth parity: waiver expiry, owner / backup coverage, on-call gap,
    /// mitigation language, and decision-right authority stay explicit and never read
    /// as a clean pass when the underlying lane is waived, ownerless, forumless, or
    /// jargon-hidden.
    GovernanceTruth,
}

impl GovernanceCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [GovernanceCertificationAxis; 6] = [
        GovernanceCertificationAxis::Visual,
        GovernanceCertificationAxis::Keyboard,
        GovernanceCertificationAxis::ScreenReader,
        GovernanceCertificationAxis::CliExport,
        GovernanceCertificationAxis::DegradedState,
        GovernanceCertificationAxis::GovernanceTruth,
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
            Self::GovernanceTruth => "governance_truth",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to
    /// a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth
    /// claim inherited from a healthier surface.
    UndisclosedDrift,
}

impl GovernanceAxisCertificationState {
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
pub enum GovernanceSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed governance-support tier
    /// delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops,
    /// or the narrowing is inconsistent.
    Red,
}

impl GovernanceSurfaceClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the surface is publishable as certified (green or disclosed
    /// yellow); red surfaces block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The copy / export parity a certified surface preserves. The CLI/export axis
/// certifies only when this offers text / JSON / Markdown reconstruction and
/// prohibits a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The fitness / ownership / waiver / decision fields the surface preserves in
    /// export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl GovernanceCertExportParity {
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
pub struct GovernanceAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: GovernanceCertificationAxis,
    /// The certification state of the axis.
    pub state: GovernanceAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5GovernanceDowngradeTrigger>,
}

impl GovernanceAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no
    ///   visible trigger (that is exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            GovernanceAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            GovernanceAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            GovernanceAxisCertificationState::UndisclosedDrift => {
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
pub struct GovernanceClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: GovernanceCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5GovernanceSupportClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5GovernanceSupportClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 governance-dashboard surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceSurfaceCertificationRow {
    /// Record kind; must equal [`GOVERNANCE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`GOVERNANCE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5GovernanceCertifiedSurface,
    /// The governance-support claim ceiling the surface asserts.
    pub claimed_claim: M5GovernanceSupportClaim,
    /// The weakest supported claim the surface is certified down to. Must be no
    /// stronger than `claimed_claim`.
    pub certified_claim: M5GovernanceSupportClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5GovernanceDashboardComponentFamily>,
    /// One outcome per [`GovernanceCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<GovernanceAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<GovernanceClaimAutoNarrow>,
    /// The one canonical governance-proof bundle this surface cites. Must equal
    /// [`GOVERNANCE_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: GovernanceSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: GovernanceCertExportParity,
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

impl GovernanceSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: GovernanceCertificationAxis) -> Option<&GovernanceAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<GovernanceCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && GovernanceCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(GovernanceAxisOutcome::well_formed)
    }

    /// True when the surface narrows its governance-support claim below what it
    /// asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<GovernanceCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == GovernanceAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the
    /// heart of the capstone: a degraded axis must produce a visible claim
    /// narrowing, CLI/export parity must always certify, and the narrowing must be
    /// consistent.
    pub fn derive_status(&self) -> GovernanceSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != GOVERNANCE_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return GovernanceSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return GovernanceSurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(GovernanceCertificationAxis::CliExport) {
            Some(o) if o.state == GovernanceAxisCertificationState::Certified => {}
            _ => return GovernanceSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == GovernanceAxisCertificationState::UndisclosedDrift)
        {
            return GovernanceSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return GovernanceSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return GovernanceSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return GovernanceSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return GovernanceSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a
        // hidden overclaim inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return GovernanceSurfaceClaimStatus::Red;
        }

        GovernanceSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == GOVERNANCE_CERT_ROW_RECORD_KIND
            && self.schema_version == GOVERNANCE_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-1059 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceSurfaceCertificationSummary {
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

/// Constructor input for [`GovernanceSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernanceSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<GovernanceSurfaceCertificationRow>,
}

/// Checked-in M05-1059 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<GovernanceSurfaceCertificationRow>,
    pub summary: GovernanceSurfaceCertificationSummary,
}

impl GovernanceSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: GovernanceSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: GOVERNANCE_CERT_SCHEMA_VERSION,
            record_kind: GOVERNANCE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: GovernanceSurfaceCertificationSummary {
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
    pub fn represented_surfaces(&self) -> BTreeSet<M5GovernanceCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5GovernanceDashboardComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5GovernanceCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface —
    /// proof the full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5GovernanceDashboardComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(GovernanceCertificationAxis::CliExport)
                .is_some_and(|o| o.state == GovernanceAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> GovernanceSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == GovernanceSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == GovernanceSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == GovernanceSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(GovernanceSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();

        GovernanceSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == GOVERNANCE_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(GovernanceSurfaceCertificationRow::covers_all_axes),
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_surfaces && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<GovernanceCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != GOVERNANCE_CERT_SCHEMA_VERSION {
            violations.push(GovernanceCertificationViolation::SchemaVersion {
                expected: GOVERNANCE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != GOVERNANCE_CERT_RECORD_KIND {
            violations.push(GovernanceCertificationViolation::RecordKind {
                expected: GOVERNANCE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(GovernanceCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != GOVERNANCE_CERT_CANONICAL_BUNDLE_REF {
            violations.push(GovernanceCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(GovernanceCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(GovernanceCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(GovernanceCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(GovernanceCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != GOVERNANCE_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    GovernanceCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(GovernanceCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(GovernanceCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    GovernanceCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(GovernanceCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == GovernanceSurfaceClaimStatus::Red {
                violations.push(GovernanceCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(GovernanceCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(GovernanceCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(GovernanceCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(GovernanceCertificationViolation::RawGovernanceMaterialInExport);
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
        out.push_str("# M5 Governance-Dashboard Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5GovernanceCertifiedSurface::ALL.len(),
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
pub fn current_m5_governance_dashboard_component_certification_export(
) -> Result<GovernanceSurfaceCertificationPacket, GovernanceCertificationArtifactError> {
    let packet: GovernanceSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-governance-dashboard-component-certification-proof/support_export.json"
    )))
    .map_err(GovernanceCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(GovernanceCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum GovernanceCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<GovernanceCertificationViolation>),
}

impl fmt::Display for GovernanceCertificationArtifactError {
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

impl Error for GovernanceCertificationArtifactError {}

/// Validation failure for M05-1059 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceCertificationViolation {
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
    RawGovernanceMaterialInExport,
}

impl fmt::Display for GovernanceCertificationViolation {
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
                    "packet does not cite the canonical governance-proof bundle"
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
                    "row {id} does not cite the one canonical governance-proof bundle"
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
                    "not every claimed M5 assurance/operator/shiproom surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen governance-dashboard component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawGovernanceMaterialInExport => {
                write!(f, "export contains raw governance material")
            }
        }
    }
}

impl Error for GovernanceCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&GovernanceAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != GovernanceAxisCertificationState::Certified,
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
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "waived"
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

/// Builds the canonical, checked-in M05-1059 certification packet. Certifies all
/// eight claimed M5 assurance / operator / shiproom surfaces: four deliver their
/// claim (green) and four auto-narrow a not-current truth axis to a weaker
/// governance-support ceiling (yellow). No surface hides drift (red).
pub fn seeded_m5_governance_dashboard_component_certification_packet(
) -> GovernanceSurfaceCertificationPacket {
    GovernanceSurfaceCertificationPacket::new(GovernanceSurfaceCertificationPacketInput {
        packet_id: "m5-governance-dashboard-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-10T00:00:00Z".to_owned(),
        matrix_ref: GOVERNANCE_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: GOVERNANCE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:governance-dashboard-certification:{id}"),
        GOVERNANCE_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> GovernanceCertExportParity {
    GovernanceCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: GovernanceCertificationAxis) -> &'static str {
    match axis {
        GovernanceCertificationAxis::Visual => {
            "fitness reading/provenance, readiness state, waiver expiry, owner/backup coverage, on-call/escalation route, decision forum, and blocker/waiver counts shown on-surface"
        }
        GovernanceCertificationAxis::Keyboard => {
            "the same fitness/ownership/waiver/decision actions are keyboard-reachable"
        }
        GovernanceCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/badge-only"
        }
        GovernanceCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support replay"
        }
        GovernanceCertificationAxis::DegradedState => {
            "stale evidence or a partial proof honestly downgrades the GovernedPass/GovernedResolved claim"
        }
        GovernanceCertificationAxis::GovernanceTruth => {
            "waiver expiry, owner/backup coverage, on-call gap, mitigation language, and decision-right authority stay explicit and never read as a clean pass when unresolved"
        }
    }
}

fn seed_certified(axis: GovernanceCertificationAxis) -> GovernanceAxisOutcome {
    GovernanceAxisOutcome {
        axis,
        state: GovernanceAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: GovernanceCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5GovernanceDowngradeTrigger,
) -> GovernanceAxisOutcome {
    GovernanceAxisOutcome {
        axis,
        state: GovernanceAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<GovernanceAxisOutcome> {
    GovernanceCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: GovernanceCertificationAxis,
    outcome: GovernanceAxisOutcome,
) -> Vec<GovernanceAxisOutcome> {
    GovernanceCertificationAxis::ALL
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
    surface: M5GovernanceCertifiedSurface,
    claimed_claim: M5GovernanceSupportClaim,
    certified_claim: M5GovernanceSupportClaim,
    consumed_families: &[M5GovernanceDashboardComponentFamily],
    axis_outcomes: Vec<GovernanceAxisOutcome>,
    claim_auto_narrow: Option<GovernanceClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> GovernanceSurfaceCertificationRow {
    let mut row = GovernanceSurfaceCertificationRow {
        record_kind: GOVERNANCE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: GOVERNANCE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        canonical_bundle_ref: GOVERNANCE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: GovernanceSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            GOVERNANCE_CERT_MATRIX_REF.to_owned(),
            GOVERNANCE_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-10T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: GovernanceCertificationAxis,
    from_claim: M5GovernanceSupportClaim,
    to_claim: M5GovernanceSupportClaim,
    label: &str,
) -> GovernanceClaimAutoNarrow {
    GovernanceClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<GovernanceSurfaceCertificationRow> {
    use GovernanceCertificationAxis as Ax;
    use M5GovernanceCertifiedSurface as S;
    use M5GovernanceDashboardComponentFamily::*;
    use M5GovernanceDowngradeTrigger as Trig;
    use M5GovernanceSupportClaim::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:assurance-center",
            S::AssuranceCenter,
            GovernedPass,
            GovernedPass,
            &[FitnessDashboardTile, GovernanceReportRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "evidence_freshness"],
            &[
                "fitness dashboard tile names its metric reading and corpus/profile provenance",
                "governance report row names lane readiness and its evidence freshness",
                "keyboard/screen-reader reach preserved for the fitness and report rows",
                "governance-truth: a stale or waived lane never reads as a clean assurance pass",
            ],
        ),
        seed_row(
            "cert:release-center",
            S::ReleaseCenter,
            GovernedPass,
            GovernedPass,
            &[ReleaseGateBanner, MilestoneDashboardRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "gate_decision"],
            &[
                "release-gate banner names its ship/no-ship decision and plain-language reason",
                "milestone dashboard row names its exit-gate state and blocker/waiver counts",
                "export preserves the gate decision and milestone gate truth",
                "governance-truth: a forumless or blocked milestone never reads as resolved",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            GovernedResolved,
            GovernedResolved,
            &[MitigationNoteCard, WaiverExpiryQueueItem],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "waiver_expiry"],
            &[
                "mitigation note card carries user-facing mitigation in reusable plain language",
                "waiver-expiry queue item names when each waiver lapses",
                "text / JSON / Markdown reconstruction certified for support replay",
                "governance-truth: mitigation is never hidden behind internal jargon in export",
            ],
        ),
        seed_row(
            "cert:docs-help",
            S::DocsHelp,
            GovernedResolved,
            GovernedResolved,
            &[GovernanceReportRow, FitnessDashboardTile],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "provenance"],
            &[
                "docs render the governance report row's readiness and evidence identically",
                "fitness tile provenance is documented, never implied as current",
                "export preserves the corpus/profile provenance truth",
                "governance-truth: docs never present a stale reading as a clean pass",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:operator-overview",
            S::OperatorOverview,
            GovernedPass,
            Provisional,
            &[ServiceOwnershipCard, OnCallStrip],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "service-ownership backup-coverage proof aged out",
                    "The operator board's owner/backup coverage freshness proof has gone stale and is re-establishing, so the GovernedPass claim narrows to provisional rather than presenting last-known coverage as current",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                GovernedPass,
                Provisional,
                "Provisional ownership: the owner/backup coverage proof is stale and re-establishing; the coverage and on-call route shown are last-known",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "service-ownership card keeps owner and backup coverage visible through the stale window",
                "on-call strip keeps the escalation route and named responder visible",
                "degraded-state: GovernedPass narrows to provisional (auto-narrowed)",
                "governance-truth: the ownerless/backup-missing risk stays explicit while proof re-establishes",
            ],
        ),
        seed_row(
            "cert:service-health",
            S::ServiceHealth,
            GovernedPass,
            Degraded,
            &[ServiceOwnershipCard, OnCallStrip],
            seed_certified_except(
                Ax::GovernanceTruth,
                seed_narrowed(
                    Ax::GovernanceTruth,
                    "on-call coverage has a disclosed gap for one rotation",
                    "The service-health surface has an on-call gap in one rotation, so the GovernedPass claim narrows to degraded and the gap plus its escalation route are shown rather than presenting the rotation as fully covered",
                    Trig::OnCallGapHidden,
                ),
            ),
            Some(seed_narrow(
                Ax::GovernanceTruth,
                GovernedPass,
                Degraded,
                "Degraded on-call coverage: one rotation has a gap; the uncovered window and the escalation route to page are both shown",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "service-ownership card keeps the accountable owner visible",
                "on-call strip names the covered rotations and the gap explicitly",
                "governance-truth: GovernedPass narrows to degraded (auto-narrowed)",
                "CLI/export parity certified so operators can replay the on-call gap decision",
            ],
        ),
        seed_row(
            "cert:shiproom",
            S::Shiproom,
            GovernedPass,
            Degraded,
            &[DecisionRightCard, MilestoneDashboardRow],
            seed_certified_except(
                Ax::GovernanceTruth,
                seed_narrowed(
                    Ax::GovernanceTruth,
                    "the forum shown is advisory; the authoritative decision has not resolved",
                    "The shiproom packet's next move needs an authoritative forum that has not yet resolved, so the GovernedPass claim narrows to degraded and the forum is labelled advisory rather than letting it read as authoritative approval",
                    Trig::AdvisoryForumReadsAuthoritative,
                ),
            ),
            Some(seed_narrow(
                Ax::GovernanceTruth,
                GovernedPass,
                Degraded,
                "Degraded decision right: the forum shown is advisory only; the authoritative forum that can approve the next move has not yet resolved it",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "decision-right card names the forum and marks it advisory, not authoritative",
                "milestone dashboard row keeps the exit-gate state and blocker counts visible",
                "governance-truth: GovernedPass narrows to degraded (auto-narrowed)",
                "shiproom never routes a move as approved when only an advisory forum has spoken",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            GovernedResolved,
            WaiverGated,
            &[WaiverExpiryQueueItem, DecisionRightCard],
            seed_certified_except(
                Ax::GovernanceTruth,
                seed_narrowed(
                    Ax::GovernanceTruth,
                    "an expiring waiver holds a lane surfaced in this headless export",
                    "A lane in the headless export is held by an active, expiring waiver, so the GovernedResolved claim narrows to waiver-gated instead of letting the waived lane read as a clean pass",
                    Trig::WaiverExpiryHidden,
                ),
            ),
            Some(seed_narrow(
                Ax::GovernanceTruth,
                GovernedResolved,
                WaiverGated,
                "Waiver-gated lane: an active waiver is expiring soon; the lane is held by exception and does not read as a clean pass",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "waiver-expiry queue item names the waiver's lapse time in the CLI output",
                "decision-right card keeps the authoritative forum explicit in the structured output",
                "governance-truth: GovernedResolved narrows to waiver-gated (auto-narrowed)",
                "CLI/export parity certified so automation can replay the waiver-gated decision",
            ],
        ),
    ]
}
