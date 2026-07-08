//! M05-947 surface certification over the frozen M5 badge-family matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`])
//! defines the six controlled badge families — support class, evidence freshness,
//! lifecycle, channel, deployment scope, and compatibility state — the M05-941..944
//! primitive lanes narrow each render vocabulary, the M05-945 consumer lane adopts
//! them across product surfaces, and the M05-946 accessibility lane
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_badge_freshness_lifecycle_deployment_support_or_compatibility_posture_is_stale_limited_imported_or_policy_blocked_across_claimed_m5_surfaces`])
//! proves keyboard / screen-reader / CLI-export parity and per-dimension
//! auto-narrowing, this closing capstone *certifies* that the shared badge truth
//! holds on every claimed M5 badge-bearing surface — and auto-narrows any surface
//! that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user reads a badge on (marketplace,
//! help/about, settings, onboarding, diagnostics, runtime/deployment, support/export,
//! and CLI/headless), not on badge family or primitive lane. Each
//! [`BadgeSurfaceCertificationRow`] certifies one surface across six truth axes —
//! visual, keyboard, screen-reader, CLI/export, degraded-state, and axis-separation —
//! and either passes (green), auto-narrows its badge-support claim to the weakest
//! supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden
//! behind a full-truth claim inherited from a healthier surface.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `FullClaim`/`Supported` badge claim while one of its truth
//! axes is not current is over-claiming and blocks; a surface that discloses the
//! reduction by narrowing its badge-support claim (with a bound reason and a frozen
//! downgrade trigger) is honestly yellow. The always-on CLI/export axis must always
//! stay certified, so support and automation can reconstruct the certified
//! support-class / freshness / lifecycle / channel / deployment / compatibility truth
//! from the same badge identity the user saw. The axis-separation axis certifies that
//! the six badge cues stay distinct — no badge implies another, and Certified never
//! implies Fresh.
//!
//! The M05-947 delta over the release-publication certification capstone it clones is
//! **badge-meaning preservation**: the badge's axis meaning (support class ≠ freshness
//! ≠ lifecycle ≠ channel ≠ deployment ≠ compatibility, plus its explanation drawer,
//! downgrade rule, and filter key) must never be collapsed or dropped between the
//! marketplace, help, diagnostics, and exported evidence. A row that loses badge
//! meaning blocks.
//!
//! Every row cites exactly one canonical release-proof bundle
//! ([`BADGE_CERT_CANONICAL_BUNDLE_REF`]) — the frozen badge-family matrix release
//! proof — rather than cloning per-surface evidence. The packet is metadata-only: raw
//! badge material, signing keys, and evidence cursors never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-badge-family-certification.schema.json`](../../../../schemas/ui/m5-badge-family-certification.schema.json).
//! The contract doc is
//! [`docs/release/m5_badge_family_certification_contract.md`](../../../../docs/release/m5_badge_family_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_badge_freshness_lifecycle_deployment_support_or_compatibility_posture_is_stale_limited_imported_or_policy_blocked_across_claimed_m5_surfaces as a11y;
use a11y::M5BadgeSupportClaim;
use matrix::{M5BadgeDowngradeTrigger, M5BadgeFamily};

/// Schema version stamped on the M05-947 certification packet.
pub const BADGE_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`BadgeSurfaceCertificationPacket`].
pub const BADGE_CERT_RECORD_KIND: &str = "m5_badge_family_certification_packet";

/// Stable record-kind tag carried by each [`BadgeSurfaceCertificationRow`].
pub const BADGE_CERT_ROW_RECORD_KIND: &str = "m5_badge_family_certification_row";

/// Repo-relative path of the boundary schema.
pub const BADGE_CERT_SCHEMA_REF: &str = "schemas/ui/m5-badge-family-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const BADGE_CERT_DOC_REF: &str = "docs/release/m5_badge_family_certification_contract.md";

/// Repo-relative path of the frozen badge-family matrix schema the certified surfaces
/// render.
pub const BADGE_CERT_MATRIX_REF: &str = matrix::M5_BADGE_FAMILY_SCHEMA_REF;

/// The one canonical release-proof bundle every certified surface cites as its
/// first-resolved badge truth. All eight surfaces point back to it rather than cloning
/// per-surface evidence.
pub const BADGE_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_BADGE_FAMILY_ARTIFACT_REF;

/// The M05-946 accessibility support export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const BADGE_CERT_A11Y_BUNDLE_REF: &str = a11y::BADGE_A11Y_FALLBACK_ARTIFACT_REF;

/// The M05-945 consumer-adoption support export, recorded as supporting evidence.
pub const BADGE_CERT_CONSUMER_BUNDLE_REF: &str =
    "artifacts/release/m5-badge-family-consumer-proof/support_export.json";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const BADGE_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-badge-family-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const BADGE_CERT_CSV_REF: &str =
    "artifacts/release/m5-badge-family-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const BADGE_CERT_REPORT_REF: &str =
    "artifacts/release/m5-badge-family-certification-proof/report.md";

/// The eight claimed M5 badge-bearing surfaces this capstone certifies. Keyed on the
/// surface a user actually reads a badge on, not on the reusable badge family it
/// renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeCertifiedSurface {
    /// The marketplace UI where capabilities are browsed and installed.
    Marketplace,
    /// The help / About surface.
    HelpAbout,
    /// The settings UI.
    Settings,
    /// The onboarding flow.
    Onboarding,
    /// The diagnostics surface.
    Diagnostics,
    /// The runtime / deployment summary surface.
    RuntimeDeployment,
    /// The support / export bundle surface.
    SupportExport,
    /// The CLI / headless surface.
    CliHeadless,
}

impl M5BadgeCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5BadgeCertifiedSurface; 8] = [
        M5BadgeCertifiedSurface::Marketplace,
        M5BadgeCertifiedSurface::HelpAbout,
        M5BadgeCertifiedSurface::Settings,
        M5BadgeCertifiedSurface::Onboarding,
        M5BadgeCertifiedSurface::Diagnostics,
        M5BadgeCertifiedSurface::RuntimeDeployment,
        M5BadgeCertifiedSurface::SupportExport,
        M5BadgeCertifiedSurface::CliHeadless,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Marketplace => "marketplace",
            Self::HelpAbout => "help_about",
            Self::Settings => "settings",
            Self::Onboarding => "onboarding",
            Self::Diagnostics => "diagnostics",
            Self::RuntimeDeployment => "runtime_deployment",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader,
/// CLI/export, degraded-state, and axis-separation behavior. The CLI/export axis is
/// always-on and must stay certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeCertificationAxis {
    /// Visual parity: the badge's value, axis name, and explanation-drawer affordance
    /// are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same badge truth and its explanation drawer are
    /// reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the badge's axis name and typed value are announced
    /// non-visually, never relying on color or a glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified badge state is reconstructable as
    /// text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a stale, limited, imported, or policy-blocked badge
    /// dimension honestly downgrades a `FullClaim` / `Supported` claim to a weaker
    /// ceiling rather than presenting last-known posture as current.
    DegradedState,
    /// Axis-separation parity: the six badge cues stay distinct — no badge merges into,
    /// implies, or stands in for another, and Certified never implies Fresh.
    AxisSeparation,
}

impl BadgeCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [BadgeCertificationAxis; 6] = [
        BadgeCertificationAxis::Visual,
        BadgeCertificationAxis::Keyboard,
        BadgeCertificationAxis::ScreenReader,
        BadgeCertificationAxis::CliExport,
        BadgeCertificationAxis::DegradedState,
        BadgeCertificationAxis::AxisSeparation,
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
            Self::AxisSeparation => "axis_separation",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a
    /// visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim
    /// inherited from a healthier surface.
    UndisclosedDrift,
}

impl BadgeAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole surface. Never asserted by the author
/// — always recomputed from the axis outcomes and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BadgeSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed badge-support tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops,
    /// badge meaning is dropped, or the narrowing is inconsistent.
    Red,
}

impl BadgeSurfaceClaimStatus {
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
pub struct BadgeCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The badge fields the surface preserves in export (value, axis name, explanation,
    /// downgrade reason, filter key).
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl BadgeCertExportParity {
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
pub struct BadgeAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: BadgeCertificationAxis,
    /// The certification state of the axis.
    pub state: BadgeAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5BadgeDowngradeTrigger>,
}

impl BadgeAxisOutcome {
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
            BadgeAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            BadgeAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            BadgeAxisCertificationState::UndisclosedDrift => {
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
pub struct BadgeClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: BadgeCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5BadgeSupportClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5BadgeSupportClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 badge-bearing surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeSurfaceCertificationRow {
    /// Record kind; must equal [`BADGE_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BADGE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5BadgeCertifiedSurface,
    /// The badge-support claim ceiling the surface asserts.
    pub claimed_claim: M5BadgeSupportClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger
    /// than `claimed_claim`.
    pub certified_claim: M5BadgeSupportClaim,
    /// The frozen badge families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5BadgeFamily>,
    /// One outcome per [`BadgeCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<BadgeAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<BadgeClaimAutoNarrow>,
    /// True when the badge's axis meaning (support class ≠ freshness ≠ lifecycle ≠
    /// channel ≠ deployment ≠ compatibility, plus its explanation drawer, downgrade
    /// rule, and filter key) is preserved across this surface and its export. A row that
    /// drops badge meaning cannot certify. This is the M05-947 delta over the cloned
    /// publication-component certification.
    pub badge_meaning_preserved: bool,
    /// The one canonical release-proof bundle this surface cites. Must equal
    /// [`BADGE_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: BadgeSurfaceClaimStatus,
    /// The copy / export parity of the certified badge state.
    pub export_parity: BadgeCertExportParity,
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

impl BadgeSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: BadgeCertificationAxis) -> Option<&BadgeAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<BadgeCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && BadgeCertificationAxis::ALL.iter().all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes.iter().all(BadgeAxisOutcome::well_formed)
    }

    /// True when the surface narrows its badge-support claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<BadgeCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == BadgeAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether this surface preserves the badge's axis meaning end to end (the M05-947
    /// delta invariant).
    pub fn preserves_badge_meaning(&self) -> bool {
        self.badge_meaning_preserved
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart
    /// of the capstone: a degraded axis must produce a visible claim narrowing,
    /// CLI/export parity must always certify, badge meaning must be preserved, and the
    /// narrowing must be consistent.
    pub fn derive_status(&self) -> BadgeSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != BADGE_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.badge_meaning_preserved
        {
            return BadgeSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return BadgeSurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(BadgeCertificationAxis::CliExport) {
            Some(o) if o.state == BadgeAxisCertificationState::Certified => {}
            _ => return BadgeSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == BadgeAxisCertificationState::UndisclosedDrift)
        {
            return BadgeSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return BadgeSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return BadgeSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return BadgeSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return BadgeSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return BadgeSurfaceClaimStatus::Red;
        }

        BadgeSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == BADGE_CERT_ROW_RECORD_KIND
            && self.schema_version == BADGE_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-947 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeSurfaceCertificationSummary {
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
    pub all_rows_preserve_badge_meaning: bool,
    pub every_axis_covered_on_every_row: bool,
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`BadgeSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BadgeSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<BadgeSurfaceCertificationRow>,
}

/// Checked-in M05-947 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BadgeSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<BadgeSurfaceCertificationRow>,
    pub summary: BadgeSurfaceCertificationSummary,
}

impl BadgeSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: BadgeSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: BADGE_CERT_SCHEMA_VERSION,
            record_kind: BADGE_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: BadgeSurfaceCertificationSummary {
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
                all_rows_preserve_badge_meaning: false,
                every_axis_covered_on_every_row: false,
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5BadgeCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Badge families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5BadgeFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5BadgeCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen badge family is certified on at least one surface — proof
    /// the full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5BadgeFamily::ALL.iter().all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(BadgeCertificationAxis::CliExport)
                .is_some_and(|o| o.state == BadgeAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Whether every row preserves badge meaning (the M05-947 delta invariant).
    pub fn all_rows_preserve_badge_meaning(&self) -> bool {
        self.rows.iter().all(|r| r.preserves_badge_meaning())
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> BadgeSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == BadgeSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == BadgeSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == BadgeSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(BadgeSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_badge_meaning = self.all_rows_preserve_badge_meaning();

        BadgeSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == BADGE_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_rows_preserve_badge_meaning: all_badge_meaning,
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(BadgeSurfaceCertificationRow::covers_all_axes),
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable
                && all_fresh
                && all_surfaces
                && all_families
                && all_badge_meaning,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<BadgeCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != BADGE_CERT_SCHEMA_VERSION {
            violations.push(BadgeCertificationViolation::SchemaVersion {
                expected: BADGE_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != BADGE_CERT_RECORD_KIND {
            violations.push(BadgeCertificationViolation::RecordKind {
                expected: BADGE_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(BadgeCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != BADGE_CERT_CANONICAL_BUNDLE_REF {
            violations.push(BadgeCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(BadgeCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(BadgeCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(BadgeCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(BadgeCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != BADGE_CERT_CANONICAL_BUNDLE_REF {
                violations.push(BadgeCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(BadgeCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(BadgeCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Badge meaning must be preserved end to end (M05-947 delta).
            if !row.preserves_badge_meaning() {
                violations.push(BadgeCertificationViolation::BadgeMeaningDropped {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(BadgeCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(BadgeCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == BadgeSurfaceClaimStatus::Red {
                violations.push(BadgeCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(BadgeCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen badge family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(BadgeCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(BadgeCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(BadgeCertificationViolation::RawBadgeMaterialInExport);
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
        out.push_str("# M5 Badge-Family Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5BadgeCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Badge meaning preserved: {}\n",
            self.summary.all_rows_preserve_badge_meaning
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
pub fn current_m5_badge_family_certification_export(
) -> Result<BadgeSurfaceCertificationPacket, BadgeCertificationArtifactError> {
    let packet: BadgeSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-badge-family-certification-proof/support_export.json"
    )))
    .map_err(BadgeCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BadgeCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum BadgeCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BadgeCertificationViolation>),
}

impl fmt::Display for BadgeCertificationArtifactError {
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

impl Error for BadgeCertificationArtifactError {}

/// Validation failure for M05-947 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BadgeCertificationViolation {
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
    BadgeMeaningDropped { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawBadgeMaterialInExport,
}

impl fmt::Display for BadgeCertificationViolation {
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
                write!(f, "packet does not cite the canonical badge-family bundle")
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
                    "row {id} does not cite the one canonical badge-family bundle"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::BadgeMeaningDropped { id } => {
                write!(
                    f,
                    "row {id} drops badge meaning: an axis, explanation drawer, downgrade rule, or filter key is collapsed between product, docs, diagnostics, and export"
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
CLI/export parity dropped, badge meaning was dropped, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 badge-bearing surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen badge family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBadgeMaterialInExport => {
                write!(f, "export contains raw badge material")
            }
        }
    }
}

impl Error for BadgeCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&BadgeAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != BadgeAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Rejects a
/// disclosure that is just a bare badge value token (which would imply an axis rather
/// than explain the narrowing).
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
            | "certified"
            | "fresh"
            | "supported"
            | "deprecated"
            | "beta"
            | "preview"
            | "imported"
            | "limited"
            | "provisional"
            | "policy_blocked"
            | "policy blocked"
            | "compatible"
            | "mismatch"
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

/// Builds the canonical, checked-in M05-947 certification packet. Certifies all eight
/// claimed M5 badge-bearing surfaces: four deliver their claim (green) and four
/// auto-narrow a not-current truth axis to a weaker badge-support ceiling (yellow). No
/// surface hides drift (red).
pub fn seeded_m5_badge_family_certification_packet() -> BadgeSurfaceCertificationPacket {
    BadgeSurfaceCertificationPacket::new(BadgeSurfaceCertificationPacketInput {
        packet_id: "m5-badge-family-certification:stable:0001".to_owned(),
        as_of: "2026-07-08T00:00:00Z".to_owned(),
        matrix_ref: BADGE_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: BADGE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:badge-family-certification:{id}"),
        BADGE_CERT_A11Y_BUNDLE_REF.to_owned(),
        BADGE_CERT_CONSUMER_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> BadgeCertExportParity {
    BadgeCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: BadgeCertificationAxis) -> &'static str {
    match axis {
        BadgeCertificationAxis::Visual => {
            "each badge shows its typed value, axis name, and explanation-drawer affordance on-surface"
        }
        BadgeCertificationAxis::Keyboard => {
            "the same badge truth and its explanation drawer are keyboard-reachable"
        }
        BadgeCertificationAxis::ScreenReader => {
            "the badge axis name and typed value are announced non-visually, never color/glyph-only"
        }
        BadgeCertificationAxis::CliExport => {
            "badge state exports as text / JSON / Markdown for support replay"
        }
        BadgeCertificationAxis::DegradedState => {
            "a stale, limited, imported, or policy-blocked dimension honestly narrows the FullClaim/Supported claim"
        }
        BadgeCertificationAxis::AxisSeparation => {
            "the six badge cues stay distinct — no badge implies another, and Certified never implies Fresh"
        }
    }
}

fn seed_certified(axis: BadgeCertificationAxis) -> BadgeAxisOutcome {
    BadgeAxisOutcome {
        axis,
        state: BadgeAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: BadgeCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5BadgeDowngradeTrigger,
) -> BadgeAxisOutcome {
    BadgeAxisOutcome {
        axis,
        state: BadgeAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<BadgeAxisOutcome> {
    BadgeCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: BadgeCertificationAxis,
    outcome: BadgeAxisOutcome,
) -> Vec<BadgeAxisOutcome> {
    BadgeCertificationAxis::ALL
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
    surface: M5BadgeCertifiedSurface,
    claimed_claim: M5BadgeSupportClaim,
    certified_claim: M5BadgeSupportClaim,
    consumed_families: &[M5BadgeFamily],
    axis_outcomes: Vec<BadgeAxisOutcome>,
    claim_auto_narrow: Option<BadgeClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> BadgeSurfaceCertificationRow {
    let mut row = BadgeSurfaceCertificationRow {
        record_kind: BADGE_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: BADGE_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        badge_meaning_preserved: true,
        canonical_bundle_ref: BADGE_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: BadgeSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            BADGE_CERT_MATRIX_REF.to_owned(),
            BADGE_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-08T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: BadgeCertificationAxis,
    from_claim: M5BadgeSupportClaim,
    to_claim: M5BadgeSupportClaim,
    label: &str,
) -> BadgeClaimAutoNarrow {
    BadgeClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<BadgeSurfaceCertificationRow> {
    use BadgeCertificationAxis as Ax;
    use M5BadgeCertifiedSurface as S;
    use M5BadgeDowngradeTrigger as Trig;
    use M5BadgeFamily::*;
    use M5BadgeSupportClaim::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:marketplace",
            S::Marketplace,
            FullClaim,
            FullClaim,
            &[SupportClass, EvidenceFreshness],
            seed_all_certified(),
            None,
            &["surface", "value_state", "axis_name", "explanation_drawer", "filter_key"],
            &[
                "support-class badge names how supported the capability is",
                "evidence-freshness badge names how fresh the proof is, separately from support class",
                "both badges open the same explanation drawer and stay separately filterable",
                "axis-separation: a Certified support class never implies Fresh evidence",
            ],
        ),
        seed_row(
            "cert:help-about",
            S::HelpAbout,
            Supported,
            Supported,
            &[Lifecycle, Channel],
            seed_all_certified(),
            None,
            &["surface", "value_state", "axis_name", "explanation_drawer", "filter_key"],
            &[
                "lifecycle badge names the maturity stage of the running build",
                "channel badge names which release channel it rides, distinct from lifecycle",
                "help/about export preserves both axis names and their explanation drawers",
                "axis-separation: a Stable lifecycle never implies a Stable channel",
            ],
        ),
        seed_row(
            "cert:settings",
            S::Settings,
            Supported,
            Supported,
            &[DeploymentScope],
            seed_all_certified(),
            None,
            &["surface", "value_state", "axis_name", "explanation_drawer", "filter_key"],
            &[
                "deployment-scope badge names where the capability runs / is available",
                "the badge keeps its explanation drawer and filter key in settings",
                "keyboard and screen-reader reach preserved for the scope badge",
                "axis-separation: a local-only deployment never implies an experimental lifecycle",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            FullClaim,
            FullClaim,
            &[CompatibilityState, SupportClass],
            seed_all_certified(),
            None,
            &["surface", "value_state", "axis_name", "downgrade_reason", "filter_key"],
            &[
                "support export reconstructs each badge's axis name, value, and downgrade reason",
                "compatibility-state badge names host compatibility, distinct from support class",
                "text / JSON / Markdown reconstruction certified for support replay",
                "axis-separation: exported evidence never loses a badge's axis meaning",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:onboarding",
            S::Onboarding,
            FullClaim,
            Provisional,
            &[SupportClass, Lifecycle],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "support-class evidence proof aged out and is re-establishing",
                    "The onboarding surface's support-class evidence proof has gone stale and is re-verifying, so the FullClaim narrows to provisional rather than presenting last-known support posture as current",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                FullClaim,
                Provisional,
                "Provisional support: the support-class evidence proof is stale and re-establishing; the class shown is last-known, not confirmed current",
            )),
            &["surface", "value_state", "axis_name", "downgrade_reason", "binding_axis"],
            &[
                "support-class badge keeps its value and axis name visible through the stale window",
                "lifecycle badge stays certified and separate from the narrowed support class",
                "degraded-state: FullClaim narrows to provisional (auto-narrowed)",
                "axis-separation: only the support-class claim narrows; freshness and lifecycle are untouched",
            ],
        ),
        seed_row(
            "cert:diagnostics",
            S::Diagnostics,
            FullClaim,
            Limited,
            &[CompatibilityState, EvidenceFreshness],
            seed_certified_except(
                Ax::AxisSeparation,
                seed_narrowed(
                    Ax::AxisSeparation,
                    "compatibility and freshness badges render in a compact diagnostics cluster",
                    "The diagnostics surface renders the compatibility and freshness cues in a compact cluster, so the compatibility claim narrows to limited and each badge keeps its own axis name and drawer rather than letting one imply the other",
                    Trig::AxisMergedIntoAnother,
                ),
            ),
            Some(seed_narrow(
                Ax::AxisSeparation,
                FullClaim,
                Limited,
                "Limited compatibility: shown compactly beside freshness; each badge keeps its own axis name, value, and drawer so neither implies the other",
            )),
            &["surface", "value_state", "axis_name", "explanation_drawer", "binding_axis"],
            &[
                "compatibility-state badge keeps its value and axis name in the compact cluster",
                "evidence-freshness badge stays a distinct cue with its own drawer",
                "axis-separation: compatibility narrows to limited so no badge implies another (auto-narrowed)",
                "diagnostics export preserves each badge's separate meaning",
            ],
        ),
        seed_row(
            "cert:runtime-deployment",
            S::RuntimeDeployment,
            Supported,
            Provisional,
            &[DeploymentScope, Channel],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "deployment-scope freshness proof is stale and re-verifying",
                    "The runtime surface's deployment-scope freshness proof has aged past its window and is re-verifying, so the Supported claim narrows to provisional rather than implying the deployment scope is confirmed current",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                Supported,
                Provisional,
                "Provisional scope: the deployment-scope freshness proof is stale and re-verifying; the scope shown is last-known, not confirmed current",
            )),
            &["surface", "value_state", "axis_name", "downgrade_reason", "binding_axis"],
            &[
                "deployment-scope badge keeps its value and axis name through the stale window",
                "channel badge stays certified and separate from the narrowed deployment scope",
                "degraded-state: Supported narrows to provisional (auto-narrowed)",
                "axis-separation: only the deployment-scope claim narrows; channel is untouched",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            FullClaim,
            Limited,
            &[CompatibilityState, SupportClass],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the headless surface renders the explanation drawer as an exportable text block, not inline",
                    "The headless surface cannot render the compatibility badge's explanation drawer inline, so the FullClaim narrows to limited and points to the exportable explanation instead of implying the drawer was shown",
                    Trig::ExplanationDrawerMissing,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                FullClaim,
                Limited,
                "Limited compatibility: the headless surface exports the explanation drawer as text rather than rendering it inline; the value and axis name are still shown",
            )),
            &["surface", "value_state", "axis_name", "downgrade_reason", "binding_axis"],
            &[
                "compatibility-state badge keeps its value and axis name in the CLI output",
                "support-class badge stays certified and separate from the narrowed compatibility",
                "degraded-state: FullClaim narrows to limited (auto-narrowed)",
                "CLI/export parity certified so automation can replay the headless badge state",
            ],
        ),
    ]
}
