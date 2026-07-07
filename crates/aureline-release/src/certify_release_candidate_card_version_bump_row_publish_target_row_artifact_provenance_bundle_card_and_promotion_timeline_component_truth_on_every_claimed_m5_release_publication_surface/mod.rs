//! M05-867 surface certification over the frozen M5 release-center / publication
//! component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix`])
//! defines the six reusable release-candidate-card, version-bump-row,
//! publish-target-row, artifact-provenance-bundle-card, promotion-timeline-step,
//! and rollback/revocation-row components, the M05-861..864 primitive lanes
//! narrow each one, the M05-865 consumer lane adopts them, and the M05-866
//! accessibility lane
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_publication_component_claim_auto_narrowing`])
//! proves keyboard / screen-reader / CLI-export parity and per-family
//! auto-narrowing, this closing capstone *certifies* that the shared component
//! truth holds on every claimed M5 release-publication surface — and auto-narrows
//! any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user proposes, publishes, promotes,
//! mirrors, evaluates, or exports a release through (release center, update
//! center, About/help, docs, enterprise evaluation, mirror/offline, CLI/headless,
//! and support/export), not on component family or primitive lane. Each
//! [`PublicationSurfaceCertificationRow`] certifies one surface across six truth
//! axes — visual, keyboard, screen-reader, CLI/export, degraded-state, and
//! rollback/revocation behavior — and either passes (green), auto-narrows its
//! publication-support claim to the weakest supported ceiling (yellow), or is
//! blocked (red) when a degraded axis is hidden behind a full-truth claim
//! inherited from a healthier surface.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `Certified`/`Supported` claim while one of its truth
//! axes is not current is over-claiming and blocks; a surface that discloses the
//! reduction by narrowing its publication-support claim (with a bound reason and
//! a frozen downgrade trigger) is honestly yellow. The always-on CLI/export axis
//! must always stay certified, so support and automation can reconstruct the
//! certified candidate / target / provenance / timeline truth from the same
//! object identity the user saw.
//!
//! Every row cites exactly one canonical release-proof bundle
//! ([`PUBLICATION_CERT_CANONICAL_BUNDLE_REF`]) — the frozen release-center
//! component release proof — rather than cloning per-surface evidence. The packet
//! is metadata-only: raw artifacts, signing keys, publish credentials, and mirror
//! cursors never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-release-publication-component-certification.schema.json`](../../../../schemas/ui/m5-release-publication-component-certification.schema.json).
//! The contract doc is
//! [`docs/release/m5_release_publication_component_certification_contract.md`](../../../../docs/release/m5_release_publication_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_release_candidate_card_version_bump_row_publish_target_row_artifact_provenance_bundle_card_and_promotion_timeline_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_publication_component_claim_auto_narrowing as a11y;
use a11y::M5PublicationSupportClaim;
use matrix::{M5ReleaseCenterComponentFamily, M5ReleaseCenterDowngradeTrigger};

/// Schema version stamped on the M05-867 certification packet.
pub const PUBLICATION_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`PublicationSurfaceCertificationPacket`].
pub const PUBLICATION_CERT_RECORD_KIND: &str =
    "m5_release_publication_component_certification_packet";

/// Stable record-kind tag carried by each [`PublicationSurfaceCertificationRow`].
pub const PUBLICATION_CERT_ROW_RECORD_KIND: &str =
    "m5_release_publication_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const PUBLICATION_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-release-publication-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const PUBLICATION_CERT_DOC_REF: &str =
    "docs/release/m5_release_publication_component_certification_contract.md";

/// Repo-relative path of the frozen release-center component matrix schema the
/// certified surfaces render.
pub const PUBLICATION_CERT_MATRIX_REF: &str = matrix::M5_RELEASE_CENTER_SCHEMA_REF;

/// The one canonical release-proof bundle every certified surface cites as its
/// first-resolved component truth. All eight surfaces point back to it rather than
/// cloning per-surface evidence.
pub const PUBLICATION_CERT_CANONICAL_BUNDLE_REF: &str =
    "artifacts/release/m5-release-center-component-proof/support_export.json";

/// The M05-866 accessibility support export the certification builds on. Recorded
/// as a supporting evidence ref on every row.
pub const PUBLICATION_CERT_A11Y_BUNDLE_REF: &str =
    "artifacts/release/m5-publication-component-accessibility-fallback-proof/support_export.json";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const PUBLICATION_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-release-publication-component-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const PUBLICATION_CERT_CSV_REF: &str =
    "artifacts/release/m5-release-publication-component-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const PUBLICATION_CERT_REPORT_REF: &str =
    "artifacts/release/m5-release-publication-component-certification-proof/report.md";

/// The eight claimed M5 release-publication surfaces this capstone certifies.
/// Keyed on the surface a user actually proposes, publishes, promotes, mirrors,
/// evaluates, or exports a release through, not on the reusable component family
/// it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReleasePublicationCertifiedSurface {
    /// The release-center UI where candidates are proposed and promoted.
    ReleaseCenter,
    /// The update center where installed clients see and apply releases.
    UpdateCenter,
    /// The About / Help surface.
    AboutHelp,
    /// The docs portal.
    Docs,
    /// The enterprise-evaluation pack.
    EnterpriseEvaluation,
    /// The mirror / offline console.
    MirrorOffline,
    /// The CLI / headless surface.
    CliHeadless,
    /// The support / export bundle surface.
    SupportExport,
}

impl M5ReleasePublicationCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5ReleasePublicationCertifiedSurface; 8] = [
        M5ReleasePublicationCertifiedSurface::ReleaseCenter,
        M5ReleasePublicationCertifiedSurface::UpdateCenter,
        M5ReleasePublicationCertifiedSurface::AboutHelp,
        M5ReleasePublicationCertifiedSurface::Docs,
        M5ReleasePublicationCertifiedSurface::EnterpriseEvaluation,
        M5ReleasePublicationCertifiedSurface::MirrorOffline,
        M5ReleasePublicationCertifiedSurface::CliHeadless,
        M5ReleasePublicationCertifiedSurface::SupportExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::UpdateCenter => "update_center",
            Self::AboutHelp => "about_help",
            Self::Docs => "docs",
            Self::EnterpriseEvaluation => "enterprise_evaluation",
            Self::MirrorOffline => "mirror_offline",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the
/// parity dimensions the spec requires verifying — visual, keyboard,
/// screen-reader, CLI/export, degraded-state, and rollback/revocation behavior.
/// The CLI/export axis is always-on and must stay certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationCertificationAxis {
    /// Visual parity: candidate scope / blocker freshness, target visibility /
    /// mutability / auth source, provenance, and rollout ring are shown on the
    /// primary surface.
    Visual,
    /// Keyboard-reach parity: the same candidate / target / provenance / timeline
    /// truth and its actions are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never
    /// relying on color or a badge glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is
    /// reconstructable as text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: stale evidence, a partial signature / attestation, a
    /// masked target-auth posture, or an unverified mirror honestly downgrades a
    /// `Certified` / `Supported` claim to degraded / provisional / unverified /
    /// policy-blocked.
    DegradedState,
    /// Rollback / revocation parity: a rollback or revocation's blast radius and
    /// revocation scope are stated before any promotion or emergency action, never
    /// reading like a generic status change.
    RollbackRevocation,
}

impl PublicationCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [PublicationCertificationAxis; 6] = [
        PublicationCertificationAxis::Visual,
        PublicationCertificationAxis::Keyboard,
        PublicationCertificationAxis::ScreenReader,
        PublicationCertificationAxis::CliExport,
        PublicationCertificationAxis::DegradedState,
        PublicationCertificationAxis::RollbackRevocation,
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
            Self::RollbackRevocation => "rollback_revocation",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to
    /// a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth
    /// claim inherited from a healthier surface.
    UndisclosedDrift,
}

impl PublicationAxisCertificationState {
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
pub enum PublicationSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed publication-support tier
    /// delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops,
    /// or the narrowing is inconsistent.
    Red,
}

impl PublicationSurfaceClaimStatus {
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
pub struct PublicationCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The candidate / target / provenance / timeline fields the surface preserves
    /// in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl PublicationCertExportParity {
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
pub struct PublicationAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: PublicationCertificationAxis,
    /// The certification state of the axis.
    pub state: PublicationAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5ReleaseCenterDowngradeTrigger>,
}

impl PublicationAxisOutcome {
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
            PublicationAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            PublicationAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            PublicationAxisCertificationState::UndisclosedDrift => {
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
pub struct PublicationClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: PublicationCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5PublicationSupportClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5PublicationSupportClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 release-publication surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationSurfaceCertificationRow {
    /// Record kind; must equal [`PUBLICATION_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PUBLICATION_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5ReleasePublicationCertifiedSurface,
    /// The publication-support claim ceiling the surface asserts.
    pub claimed_claim: M5PublicationSupportClaim,
    /// The weakest supported claim the surface is certified down to. Must be no
    /// stronger than `claimed_claim`.
    pub certified_claim: M5PublicationSupportClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5ReleaseCenterComponentFamily>,
    /// One outcome per [`PublicationCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<PublicationAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<PublicationClaimAutoNarrow>,
    /// The one canonical release-proof bundle this surface cites. Must equal
    /// [`PUBLICATION_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: PublicationSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: PublicationCertExportParity,
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

impl PublicationSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: PublicationCertificationAxis) -> Option<&PublicationAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<PublicationCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && PublicationCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(PublicationAxisOutcome::well_formed)
    }

    /// True when the surface narrows its publication-support claim below what it
    /// asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<PublicationCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == PublicationAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the
    /// heart of the capstone: a degraded axis must produce a visible claim
    /// narrowing, CLI/export parity must always certify, and the narrowing must be
    /// consistent.
    pub fn derive_status(&self) -> PublicationSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != PUBLICATION_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return PublicationSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return PublicationSurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(PublicationCertificationAxis::CliExport) {
            Some(o) if o.state == PublicationAxisCertificationState::Certified => {}
            _ => return PublicationSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == PublicationAxisCertificationState::UndisclosedDrift)
        {
            return PublicationSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return PublicationSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return PublicationSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return PublicationSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return PublicationSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a
        // hidden overclaim inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return PublicationSurfaceClaimStatus::Red;
        }

        PublicationSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == PUBLICATION_CERT_ROW_RECORD_KIND
            && self.schema_version == PUBLICATION_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-867 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationSurfaceCertificationSummary {
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

/// Constructor input for [`PublicationSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<PublicationSurfaceCertificationRow>,
}

/// Checked-in M05-867 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<PublicationSurfaceCertificationRow>,
    pub summary: PublicationSurfaceCertificationSummary,
}

impl PublicationSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: PublicationSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: PUBLICATION_CERT_SCHEMA_VERSION,
            record_kind: PUBLICATION_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: PublicationSurfaceCertificationSummary {
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
    pub fn represented_surfaces(&self) -> BTreeSet<M5ReleasePublicationCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ReleaseCenterComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5ReleasePublicationCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface —
    /// proof the full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5ReleaseCenterComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(PublicationCertificationAxis::CliExport)
                .is_some_and(|o| o.state == PublicationAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> PublicationSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == PublicationSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == PublicationSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == PublicationSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(PublicationSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();

        PublicationSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == PUBLICATION_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(PublicationSurfaceCertificationRow::covers_all_axes),
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_surfaces && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<PublicationCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != PUBLICATION_CERT_SCHEMA_VERSION {
            violations.push(PublicationCertificationViolation::SchemaVersion {
                expected: PUBLICATION_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != PUBLICATION_CERT_RECORD_KIND {
            violations.push(PublicationCertificationViolation::RecordKind {
                expected: PUBLICATION_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(PublicationCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != PUBLICATION_CERT_CANONICAL_BUNDLE_REF {
            violations.push(PublicationCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(PublicationCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(PublicationCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(PublicationCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(PublicationCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != PUBLICATION_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    PublicationCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(PublicationCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(
                    PublicationCertificationViolation::ExportParityNotCertified {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    PublicationCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(PublicationCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == PublicationSurfaceClaimStatus::Red {
                violations.push(PublicationCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(PublicationCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(PublicationCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(PublicationCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(PublicationCertificationViolation::RawReleaseMaterialInExport);
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
        out.push_str("# M5 Release-Publication Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5ReleasePublicationCertifiedSurface::ALL.len(),
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
pub fn current_m5_release_publication_component_certification_export(
) -> Result<PublicationSurfaceCertificationPacket, PublicationCertificationArtifactError> {
    let packet: PublicationSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-release-publication-component-certification-proof/support_export.json"
    )))
    .map_err(PublicationCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PublicationCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum PublicationCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PublicationCertificationViolation>),
}

impl fmt::Display for PublicationCertificationArtifactError {
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

impl Error for PublicationCertificationArtifactError {}

/// Validation failure for M05-867 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicationCertificationViolation {
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
    RawReleaseMaterialInExport,
}

impl fmt::Display for PublicationCertificationViolation {
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
                write!(f, "packet does not cite the canonical release-proof bundle")
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
                    "row {id} does not cite the one canonical release-proof bundle"
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
                    "not every claimed M5 release-publication surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen release-center component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawReleaseMaterialInExport => {
                write!(f, "export contains raw release material")
            }
        }
    }
}

impl Error for PublicationCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&PublicationAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != PublicationAxisCertificationState::Certified,
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

/// Builds the canonical, checked-in M05-867 certification packet. Certifies all
/// eight claimed M5 release-publication surfaces: four deliver their claim (green)
/// and four auto-narrow a not-current truth axis to a weaker publication-support
/// ceiling (yellow). No surface hides drift (red).
pub fn seeded_m5_release_publication_component_certification_packet(
) -> PublicationSurfaceCertificationPacket {
    PublicationSurfaceCertificationPacket::new(PublicationSurfaceCertificationPacketInput {
        packet_id: "m5-release-publication-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-06T00:00:00Z".to_owned(),
        matrix_ref: PUBLICATION_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: PUBLICATION_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:release-publication-certification:{id}"),
        PUBLICATION_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> PublicationCertExportParity {
    PublicationCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: PublicationCertificationAxis) -> &'static str {
    match axis {
        PublicationCertificationAxis::Visual => {
            "candidate scope, blocker freshness, target visibility/mutability/auth source, provenance, and rollout ring shown on-surface"
        }
        PublicationCertificationAxis::Keyboard => {
            "the same candidate/target/provenance/timeline actions are keyboard-reachable"
        }
        PublicationCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/badge-only"
        }
        PublicationCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support replay"
        }
        PublicationCertificationAxis::DegradedState => {
            "stale evidence, partial signature/attestation, masked auth, or unverified mirror honestly downgrades the Certified/Supported claim"
        }
        PublicationCertificationAxis::RollbackRevocation => {
            "rollback/revocation blast radius and revocation scope stated before any promotion or emergency action"
        }
    }
}

fn seed_certified(axis: PublicationCertificationAxis) -> PublicationAxisOutcome {
    PublicationAxisOutcome {
        axis,
        state: PublicationAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: PublicationCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5ReleaseCenterDowngradeTrigger,
) -> PublicationAxisOutcome {
    PublicationAxisOutcome {
        axis,
        state: PublicationAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<PublicationAxisOutcome> {
    PublicationCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: PublicationCertificationAxis,
    outcome: PublicationAxisOutcome,
) -> Vec<PublicationAxisOutcome> {
    PublicationCertificationAxis::ALL
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
    surface: M5ReleasePublicationCertifiedSurface,
    claimed_claim: M5PublicationSupportClaim,
    certified_claim: M5PublicationSupportClaim,
    consumed_families: &[M5ReleaseCenterComponentFamily],
    axis_outcomes: Vec<PublicationAxisOutcome>,
    claim_auto_narrow: Option<PublicationClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> PublicationSurfaceCertificationRow {
    let mut row = PublicationSurfaceCertificationRow {
        record_kind: PUBLICATION_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: PUBLICATION_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        canonical_bundle_ref: PUBLICATION_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: PublicationSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            PUBLICATION_CERT_MATRIX_REF.to_owned(),
            PUBLICATION_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-06T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: PublicationCertificationAxis,
    from_claim: M5PublicationSupportClaim,
    to_claim: M5PublicationSupportClaim,
    label: &str,
) -> PublicationClaimAutoNarrow {
    PublicationClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<PublicationSurfaceCertificationRow> {
    use M5PublicationSupportClaim::*;
    use M5ReleaseCenterComponentFamily::*;
    use M5ReleaseCenterDowngradeTrigger as Trig;
    use M5ReleasePublicationCertifiedSurface as S;
    use PublicationCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:release-center",
            S::ReleaseCenter,
            Certified,
            Certified,
            &[ReleaseCandidateCard, VersionBumpRow, PromotionTimelineStep],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "candidate_scope"],
            &[
                "release-candidate card names candidate scope and blocker freshness",
                "version-bump row names the derived public-surface impact",
                "promotion-timeline step names its rollout ring and stage state",
                "rollback/revocation: candidate promotion states its reversible window before any move",
            ],
        ),
        seed_row(
            "cert:about-help",
            S::AboutHelp,
            Supported,
            Supported,
            &[ReleaseCandidateCard, ArtifactProvenanceBundleCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "provenance"],
            &[
                "About/help card names the running build's candidate identity",
                "provenance bundle names signature, attestation, SBOM, and digest lineage",
                "keyboard/screen-reader reach preserved for the provenance rows",
                "rollback/revocation: the installed build's revocation scope stays inspectable",
            ],
        ),
        seed_row(
            "cert:docs",
            S::Docs,
            Supported,
            Supported,
            &[VersionBumpRow, PublishTargetRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "auth_source"],
            &[
                "docs render the version-bump row's public-surface impact identically",
                "publish-target row names target visibility, mutability, and auth source",
                "export preserves the target-class and auth-source truth",
                "rollback/revocation: docs never present a mutable target as an immutable step",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            Certified,
            Certified,
            &[ArtifactProvenanceBundleCard, RollbackRevocationRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "revocation_scope"],
            &[
                "support export reconstructs candidate/target/provenance/timeline truth",
                "rollback/revocation row names blast radius and revocation scope",
                "text / JSON / Markdown reconstruction certified for support replay",
                "rollback/revocation: an emergency action is never captured as a generic status change",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:update-center",
            S::UpdateCenter,
            Certified,
            Provisional,
            &[PromotionTimelineStep, PublishTargetRow],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "promotion mirror verification aged out",
                    "The promotion step's mirror verification proof has gone stale and is re-establishing, so the Certified claim narrows to provisional rather than presenting last-known mirror state as current",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                Certified,
                Provisional,
                "Provisional update: the promotion's mirror verification proof is stale and re-establishing; the rollout ring and target shown are last-known",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "promotion-timeline step keeps rollout ring and stage state visible through the stale window",
                "publish-target row keeps target visibility/mutability/auth source visible",
                "degraded-state: Certified narrows to provisional (auto-narrowed)",
                "rollback/revocation: the last-known-good target stays reachable while proof re-establishes",
            ],
        ),
        seed_row(
            "cert:enterprise-evaluation",
            S::EnterpriseEvaluation,
            Certified,
            Unverified,
            &[ArtifactProvenanceBundleCard, ReleaseCandidateCard],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "provenance signature/attestation could not be verified on this pack",
                    "The evaluation pack's artifact signature and attestation could not be verified from the bundled evidence, so the Certified claim narrows to unverified instead of showing an unproven signature as clean",
                    Trig::SignatureOrAttestationOverclaimed,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                Certified,
                Unverified,
                "Unverified provenance: the evaluation pack could not verify the artifact signature/attestation; digest lineage is shown but trust is not asserted",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "provenance bundle keeps the digest set and lineage visible without asserting trust",
                "release-candidate card keeps candidate scope and blocker freshness visible",
                "degraded-state: Certified narrows to unverified (auto-narrowed)",
                "rollback/revocation: revocation scope stays explicit even when signature is unproven",
            ],
        ),
        seed_row(
            "cert:mirror-offline",
            S::MirrorOffline,
            Supported,
            Provisional,
            &[PromotionTimelineStep, ArtifactProvenanceBundleCard],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "offline mirror parity proof is stale",
                    "The mirror console's offline parity proof has aged past its freshness window and is re-verifying, so the Supported claim narrows to provisional rather than implying the mirror is confirmed current",
                    Trig::ProofStale,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                Supported,
                Provisional,
                "Provisional mirror: the offline parity proof is stale and re-verifying; the mirrored digests shown are last-known, not confirmed current",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "promotion-timeline step keeps rollout ring and immutable-digest joins visible",
                "provenance bundle keeps mirror refs and digest lineage visible",
                "degraded-state: Supported narrows to provisional (auto-narrowed)",
                "rollback/revocation: the mirror's revocation scope stays reconstructable offline",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            Certified,
            Degraded,
            &[RollbackRevocationRow, PublishTargetRow],
            seed_certified_except(
                Ax::RollbackRevocation,
                seed_narrowed(
                    Ax::RollbackRevocation,
                    "rollback blast radius is only partially resolved for the headless target",
                    "The headless rollback's affected node set is only partially resolved, so the Certified claim narrows to degraded instead of understating the blast radius as fully bounded",
                    Trig::RollbackBlastRadiusUnderstated,
                ),
            ),
            Some(seed_narrow(
                Ax::RollbackRevocation,
                Certified,
                Degraded,
                "Degraded rollback scope: the headless target's affected node set is partially resolved; the bounded portion and the unresolved remainder are both shown",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "rollback/revocation row keeps blast radius and revocation scope explicit in the CLI output",
                "publish-target row keeps target class and auth source in the structured output",
                "rollback/revocation: Certified narrows to degraded (auto-narrowed)",
                "CLI/export parity certified so automation can replay the headless decision",
            ],
        ),
    ]
}
