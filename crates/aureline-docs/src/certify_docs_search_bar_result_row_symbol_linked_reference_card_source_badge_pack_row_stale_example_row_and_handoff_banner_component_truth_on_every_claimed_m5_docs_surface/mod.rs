//! M05-875 surface certification over the frozen M5 docs-browser component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`])
//! defines the eight reusable docs-search-bar, docs-scope-switcher, docs-result-row,
//! symbol-linked-reference-card, docs-source/version-badge, docs-pack-row,
//! stale-example-finding-row, and browser-handoff-banner components, the
//! M05-869..873 primitive lanes narrow each one, and the M05-874 accessibility lane
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_docs_browser_component_claim_auto_narrowing`])
//! proves keyboard / screen-reader / CLI-export parity and per-family
//! auto-narrowing, this closing capstone *certifies* that the shared docs-browser
//! component truth holds on every claimed M5 docs / help / onboarding / AI surface —
//! and auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user searches, opens, compares, cites,
//! or exports documentation through (docs browser, onboarding, glossary, AI
//! citations, support/help, mirror/offline, CLI/headless, and support/export), not
//! on component family or primitive lane. Each [`DocsSurfaceCertificationRow`]
//! certifies one surface across six truth axes — visual, keyboard, screen-reader,
//! CLI/export, degraded-state, and source/handoff provenance — and either passes
//! (green), auto-narrows its docs-support claim to the weakest supported ceiling
//! (yellow), or is blocked (red) when a degraded axis is hidden behind a full-truth
//! claim inherited from a healthier surface.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `CurrentAuthoritative` / `SupportedReference` claim while
//! one of its truth axes is not current is over-claiming and blocks; a surface that
//! discloses the reduction by narrowing its docs-support claim (with a bound reason
//! and a frozen downgrade trigger) is honestly yellow. The always-on CLI/export axis
//! must always stay certified, so support and automation can reconstruct the same
//! corpus / source / version / pack / handoff truth from the same object identity the
//! user saw.
//!
//! Every row cites exactly one canonical docs-browser proof bundle
//! ([`DOCS_CERT_CANONICAL_BUNDLE_REF`]) — the frozen docs-browser component matrix
//! release proof — rather than cloning per-surface evidence. The packet is
//! metadata-only: raw docs bodies, provider tokens, and mirror cursors never cross
//! this boundary.
//!
//! The boundary schema is
//! [`schemas/docs/m5-docs-browser-component-certification.schema.json`](../../../../schemas/docs/m5-docs-browser-component-certification.schema.json).
//! The contract doc is
//! [`docs/docs/m5/m5_docs_browser_component_certification_contract.md`](../../../../docs/docs/m5/m5_docs_browser_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_docs_browser_component_claim_auto_narrowing as a11y;
use a11y::M5DocsSupportClaim;
use matrix::{M5DocsBrowserComponentFamily, M5DocsDowngradeTrigger};

/// Schema version stamped on the M05-875 certification packet.
pub const DOCS_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`DocsSurfaceCertificationPacket`].
pub const DOCS_CERT_RECORD_KIND: &str = "m5_docs_browser_component_certification_packet";

/// Stable record-kind tag carried by each [`DocsSurfaceCertificationRow`].
pub const DOCS_CERT_ROW_RECORD_KIND: &str = "m5_docs_browser_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const DOCS_CERT_SCHEMA_REF: &str =
    "schemas/docs/m5-docs-browser-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const DOCS_CERT_DOC_REF: &str =
    "docs/docs/m5/m5_docs_browser_component_certification_contract.md";

/// Repo-relative path of the frozen docs-browser component matrix schema the
/// certified surfaces render.
pub const DOCS_CERT_MATRIX_REF: &str = matrix::M5_DOCS_BROWSER_SCHEMA_REF;

/// The one canonical docs-browser proof bundle every certified surface cites as its
/// first-resolved component truth. All eight surfaces point back to it rather than
/// cloning per-surface evidence.
pub const DOCS_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_DOCS_BROWSER_ARTIFACT_REF;

/// The M05-874 accessibility support export the certification builds on. Recorded as
/// a supporting evidence ref on every row.
pub const DOCS_CERT_A11Y_BUNDLE_REF: &str = a11y::DOCS_BROWSER_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const DOCS_CERT_ARTIFACT_REF: &str =
    "artifacts/docs/m5/m5-docs-browser-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const DOCS_CERT_CSV_REF: &str =
    "artifacts/docs/m5/m5-docs-browser-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const DOCS_CERT_REPORT_REF: &str =
    "artifacts/docs/m5/m5-docs-browser-component-certification/report.md";

/// The eight claimed M5 docs / help / onboarding / AI surfaces this capstone
/// certifies. Keyed on the surface a user actually searches, opens, compares, cites,
/// or exports documentation through, not on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsBrowserCertifiedSurface {
    /// The docs-browser UI where a user searches, opens, and compares documentation.
    DocsBrowser,
    /// The onboarding / learning tour that embeds docs references.
    Onboarding,
    /// The glossary surface that resolves terms to docs and symbols.
    Glossary,
    /// The AI-citation / evidence panel that cites documentation.
    AiCitations,
    /// The support / help surface.
    SupportHelp,
    /// The mirror / offline docs console.
    MirrorOffline,
    /// The CLI / headless docs surface.
    CliHeadless,
    /// The support / export bundle surface.
    SupportExport,
}

impl M5DocsBrowserCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5DocsBrowserCertifiedSurface; 8] = [
        M5DocsBrowserCertifiedSurface::DocsBrowser,
        M5DocsBrowserCertifiedSurface::Onboarding,
        M5DocsBrowserCertifiedSurface::Glossary,
        M5DocsBrowserCertifiedSurface::AiCitations,
        M5DocsBrowserCertifiedSurface::SupportHelp,
        M5DocsBrowserCertifiedSurface::MirrorOffline,
        M5DocsBrowserCertifiedSurface::CliHeadless,
        M5DocsBrowserCertifiedSurface::SupportExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::Onboarding => "onboarding",
            Self::Glossary => "glossary",
            Self::AiCitations => "ai_citations",
            Self::SupportHelp => "support_help",
            Self::MirrorOffline => "mirror_offline",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader,
/// CLI/export, degraded-state, and source/handoff provenance. The CLI/export axis is
/// always-on and must stay certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsCertificationAxis {
    /// Visual parity: corpus class, provider / source, version / package scope, symbol
    /// anchor, project-doc override reason, and freshness are shown on the primary
    /// surface.
    Visual,
    /// Keyboard-reach parity: the same corpus / source / version / symbol / pack /
    /// handoff truth and its actions are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on
    /// color or a badge glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is reconstructable
    /// as text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a cached / mirrored result, a version-adjacent match, an
    /// unverified symbol linkage, or a stale example honestly downgrades a
    /// `CurrentAuthoritative` / `SupportedReference` claim to a weaker docs-support
    /// tier.
    DegradedState,
    /// Source / handoff provenance parity: source class, version adjacency, mirror
    /// freshness, pack pin / offline / quarantine state, and the browser-handoff reason
    /// stay explicit, never inheriting a healthier surface's provenance or flattening a
    /// handoff into a bare URL jump.
    SourceAndHandoffProvenance,
}

impl DocsCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [DocsCertificationAxis; 6] = [
        DocsCertificationAxis::Visual,
        DocsCertificationAxis::Keyboard,
        DocsCertificationAxis::ScreenReader,
        DocsCertificationAxis::CliExport,
        DocsCertificationAxis::DegradedState,
        DocsCertificationAxis::SourceAndHandoffProvenance,
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
            Self::SourceAndHandoffProvenance => "source_and_handoff_provenance",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a
    /// visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim
    /// inherited from a healthier surface.
    UndisclosedDrift,
}

impl DocsAxisCertificationState {
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
pub enum DocsSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed docs-support tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, or
    /// the narrowing is inconsistent.
    Red,
}

impl DocsSurfaceClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the surface is certifiable as shipped (green or disclosed yellow);
    /// red surfaces block the release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The copy / export parity a certified surface preserves. The CLI/export axis
/// certifies only when this offers text / JSON / Markdown reconstruction and prohibits
/// a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The corpus / source / version / symbol / pack / handoff fields the surface
    /// preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl DocsCertExportParity {
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
pub struct DocsAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: DocsCertificationAxis,
    /// The certification state of the axis.
    pub state: DocsAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5DocsDowngradeTrigger>,
}

impl DocsAxisOutcome {
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
            DocsAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            DocsAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            DocsAxisCertificationState::UndisclosedDrift => {
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
pub struct DocsClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: DocsCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5DocsSupportClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5DocsSupportClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 docs / help / onboarding / AI surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSurfaceCertificationRow {
    /// Record kind; must equal [`DOCS_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5DocsBrowserCertifiedSurface,
    /// The docs-support claim ceiling the surface asserts.
    pub claimed_claim: M5DocsSupportClaim,
    /// The weakest supported claim the surface is certified down to. Must be no
    /// stronger than `claimed_claim`.
    pub certified_claim: M5DocsSupportClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5DocsBrowserComponentFamily>,
    /// One outcome per [`DocsCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<DocsAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<DocsClaimAutoNarrow>,
    /// The one canonical docs-browser proof bundle this surface cites. Must equal
    /// [`DOCS_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: DocsSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: DocsCertExportParity,
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

impl DocsSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: DocsCertificationAxis) -> Option<&DocsAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<DocsCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && DocsCertificationAxis::ALL.iter().all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes.iter().all(DocsAxisOutcome::well_formed)
    }

    /// True when the surface narrows its docs-support claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<DocsCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == DocsAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart
    /// of the capstone: a degraded axis must produce a visible claim narrowing,
    /// CLI/export parity must always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> DocsSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != DOCS_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return DocsSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return DocsSurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(DocsCertificationAxis::CliExport) {
            Some(o) if o.state == DocsAxisCertificationState::Certified => {}
            _ => return DocsSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == DocsAxisCertificationState::UndisclosedDrift)
        {
            return DocsSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return DocsSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return DocsSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return DocsSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return DocsSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return DocsSurfaceClaimStatus::Red;
        }

        DocsSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == DOCS_CERT_ROW_RECORD_KIND
            && self.schema_version == DOCS_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-875 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSurfaceCertificationSummary {
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

/// Constructor input for [`DocsSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<DocsSurfaceCertificationRow>,
}

/// Checked-in M05-875 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<DocsSurfaceCertificationRow>,
    pub summary: DocsSurfaceCertificationSummary,
}

impl DocsSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: DocsSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: DOCS_CERT_SCHEMA_VERSION,
            record_kind: DOCS_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: DocsSurfaceCertificationSummary {
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
    pub fn represented_surfaces(&self) -> BTreeSet<M5DocsBrowserCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5DocsBrowserComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5DocsBrowserCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface —
    /// proof the full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5DocsBrowserComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(DocsCertificationAxis::CliExport)
                .is_some_and(|o| o.state == DocsAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> DocsSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == DocsSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == DocsSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == DocsSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(DocsSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();

        DocsSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == DOCS_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(DocsSurfaceCertificationRow::covers_all_axes),
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_surfaces && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<DocsCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != DOCS_CERT_SCHEMA_VERSION {
            violations.push(DocsCertificationViolation::SchemaVersion {
                expected: DOCS_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != DOCS_CERT_RECORD_KIND {
            violations.push(DocsCertificationViolation::RecordKind {
                expected: DOCS_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(DocsCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != DOCS_CERT_CANONICAL_BUNDLE_REF {
            violations.push(DocsCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(DocsCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(DocsCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(DocsCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(DocsCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != DOCS_CERT_CANONICAL_BUNDLE_REF {
                violations.push(DocsCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(DocsCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(DocsCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(DocsCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(DocsCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == DocsSurfaceClaimStatus::Red {
                violations.push(DocsCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(DocsCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(DocsCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(DocsCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(DocsCertificationViolation::RawDocsMaterialInExport);
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
        out.push_str("# M5 Docs-Browser Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5DocsBrowserCertifiedSurface::ALL.len(),
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
pub fn current_m5_docs_browser_component_certification_export(
) -> Result<DocsSurfaceCertificationPacket, DocsCertificationArtifactError> {
    let packet: DocsSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/m5-docs-browser-component-certification/support_export.json"
    )))
    .map_err(DocsCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DocsCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum DocsCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DocsCertificationViolation>),
}

impl fmt::Display for DocsCertificationArtifactError {
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

impl Error for DocsCertificationArtifactError {}

/// Validation failure for M05-875 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocsCertificationViolation {
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
    RawDocsMaterialInExport,
}

impl fmt::Display for DocsCertificationViolation {
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
                    "packet does not cite the canonical docs-browser proof bundle"
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
                    "row {id} does not cite the one canonical docs-browser proof bundle"
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
                    "not every claimed M5 docs / help / onboarding / AI surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen docs-browser component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawDocsMaterialInExport => {
                write!(f, "export contains raw docs material")
            }
        }
    }
}

impl Error for DocsCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&DocsAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != DocsAxisCertificationState::Certified,
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

/// Builds the canonical, checked-in M05-875 certification packet. Certifies all eight
/// claimed M5 docs / help / onboarding / AI surfaces: four deliver their claim (green)
/// and four auto-narrow a not-current truth axis to a weaker docs-support ceiling
/// (yellow). No surface hides drift (red).
pub fn seeded_m5_docs_browser_component_certification_packet() -> DocsSurfaceCertificationPacket {
    DocsSurfaceCertificationPacket::new(DocsSurfaceCertificationPacketInput {
        packet_id: "m5-docs-browser-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-06T00:00:00Z".to_owned(),
        matrix_ref: DOCS_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: DOCS_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:docs-browser-certification:{id}"),
        DOCS_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> DocsCertExportParity {
    DocsCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: DocsCertificationAxis) -> &'static str {
    match axis {
        DocsCertificationAxis::Visual => {
            "corpus class, provider/source, version/package scope, symbol anchor, project-doc override reason, and freshness shown on-surface"
        }
        DocsCertificationAxis::Keyboard => {
            "the same corpus/source/version/symbol/pack/handoff truth and its actions are keyboard-reachable"
        }
        DocsCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/badge-only"
        }
        DocsCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support replay"
        }
        DocsCertificationAxis::DegradedState => {
            "a cached/mirrored result, version-adjacent match, unverified linkage, or stale example honestly downgrades the CurrentAuthoritative/SupportedReference claim"
        }
        DocsCertificationAxis::SourceAndHandoffProvenance => {
            "source class, version adjacency, mirror freshness, pack pin/offline/quarantine state, and the browser-handoff reason stay explicit"
        }
    }
}

fn seed_certified(axis: DocsCertificationAxis) -> DocsAxisOutcome {
    DocsAxisOutcome {
        axis,
        state: DocsAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: DocsCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5DocsDowngradeTrigger,
) -> DocsAxisOutcome {
    DocsAxisOutcome {
        axis,
        state: DocsAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<DocsAxisOutcome> {
    DocsCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: DocsCertificationAxis,
    outcome: DocsAxisOutcome,
) -> Vec<DocsAxisOutcome> {
    DocsCertificationAxis::ALL
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
    surface: M5DocsBrowserCertifiedSurface,
    claimed_claim: M5DocsSupportClaim,
    certified_claim: M5DocsSupportClaim,
    consumed_families: &[M5DocsBrowserComponentFamily],
    axis_outcomes: Vec<DocsAxisOutcome>,
    claim_auto_narrow: Option<DocsClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> DocsSurfaceCertificationRow {
    let mut row = DocsSurfaceCertificationRow {
        record_kind: DOCS_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: DOCS_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        canonical_bundle_ref: DOCS_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: DocsSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            DOCS_CERT_MATRIX_REF.to_owned(),
            DOCS_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-06T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: DocsCertificationAxis,
    from_claim: M5DocsSupportClaim,
    to_claim: M5DocsSupportClaim,
    label: &str,
) -> DocsClaimAutoNarrow {
    DocsClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<DocsSurfaceCertificationRow> {
    use DocsCertificationAxis as Ax;
    use M5DocsBrowserCertifiedSurface as S;
    use M5DocsBrowserComponentFamily::*;
    use M5DocsDowngradeTrigger as Trig;
    use M5DocsSupportClaim::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:docs-browser",
            S::DocsBrowser,
            CurrentAuthoritative,
            CurrentAuthoritative,
            &[DocsSearchBar, DocsScopeSwitcher, DocsResultRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "corpus_class"],
            &[
                "docs search bar names the corpus classes it searches",
                "docs scope switcher names the version / package scope in effect",
                "docs result row names its match state and any project-doc override reason",
                "source/handoff: the browser surface never presents a mirrored result as a live provider read",
            ],
        ),
        seed_row(
            "cert:onboarding",
            S::Onboarding,
            SupportedReference,
            SupportedReference,
            &[DocsResultRow, SymbolLinkedReferenceCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "symbol_anchor"],
            &[
                "onboarding embeds docs result rows with their source/version badge intact",
                "symbol-linked reference cards keep the initiating file and symbol anchor",
                "keyboard/screen-reader reach preserved for the embedded reference cards",
                "source/handoff: an onboarding handoff always explains its destination reason and return path",
            ],
        ),
        seed_row(
            "cert:glossary",
            S::Glossary,
            SupportedReference,
            SupportedReference,
            &[DocsSourceVersionBadge, SymbolLinkedReferenceCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "provider_source"],
            &[
                "glossary terms resolve through the docs source/version badge, naming provider and freshness",
                "symbol-linked reference cards keep exact/nearby/project/keyword linkage explicit",
                "export preserves the provider, version scope, and symbol-anchor truth",
                "source/handoff: the glossary never flattens a version-adjacent match into an exact one",
            ],
        ),
        seed_row(
            "cert:support-export",
            S::SupportExport,
            CurrentAuthoritative,
            CurrentAuthoritative,
            &[DocsResultRow, DocsHandoffBanner],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "handoff_reason"],
            &[
                "support export reconstructs corpus/source/version/pack/handoff truth from the same object identity",
                "docs handoff banner names why Aureline handed off, the privacy exposure, and the return path",
                "text / JSON / Markdown reconstruction certified for support replay",
                "source/handoff: a handoff is never captured as a raw URL jump that strips source/version context",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:ai-citations",
            S::AiCitations,
            CurrentAuthoritative,
            UnverifiedReference,
            &[SymbolLinkedReferenceCard, DocsSourceVersionBadge],
            seed_certified_except(
                Ax::SourceAndHandoffProvenance,
                seed_narrowed(
                    Ax::SourceAndHandoffProvenance,
                    "AI-cited symbol card resolved by keyword fallback",
                    "The AI citation's symbol-linked reference resolved only by keyword fallback rather than an exact or nearby symbol anchor, so the CurrentAuthoritative claim narrows to unverified instead of presenting a keyword guess as a proven linkage",
                    Trig::SymbolAnchorUnresolvedHidden,
                ),
            ),
            Some(seed_narrow(
                Ax::SourceAndHandoffProvenance,
                CurrentAuthoritative,
                UnverifiedReference,
                "Unverified linkage: the cited symbol resolved by keyword fallback; the source/version badge is shown but the exact symbol anchor is not confirmed",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "symbol-linked reference card keeps the initiating file/symbol anchor visible with a keyword-fallback disclosure",
                "docs source/version badge keeps provider and freshness visible without asserting exact linkage",
                "source/handoff: CurrentAuthoritative narrows to unverified (auto-narrowed)",
                "known compatibility note: keyword-fallback citations never inherit an exact-symbol authoritative label",
            ],
        ),
        seed_row(
            "cert:support-help",
            S::SupportHelp,
            SupportedReference,
            CachedReference,
            &[DocsHandoffBanner, DocsResultRow],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "referenced docs result served from cache",
                    "The help surface's referenced docs result was served from a cached / mirrored copy rather than a live provider read, so the SupportedReference claim narrows to cached instead of presenting last-known content as current",
                    Trig::MirroredOrCachedShownAsLive,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                SupportedReference,
                CachedReference,
                "Cached reference: the help result is a last-known cached copy, not a live provider read; the source/version badge shows the cache freshness",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "docs handoff banner keeps the destination reason, privacy exposure, and return path explicit",
                "docs result row keeps its cached freshness reading visible",
                "degraded-state: SupportedReference narrows to cached (auto-narrowed)",
                "known compatibility note: mirror freshness — cached help results never read as live",
            ],
        ),
        seed_row(
            "cert:mirror-offline",
            S::MirrorOffline,
            SupportedReference,
            CachedReference,
            &[DocsPackRow, DocsSourceVersionBadge],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "offline mirror pack content is cached",
                    "The offline console's docs-pack content is a mirrored / cached copy while the provider is unreachable, so the SupportedReference claim narrows to cached rather than implying the mirror is confirmed current",
                    Trig::MirroredOrCachedShownAsLive,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                SupportedReference,
                CachedReference,
                "Cached mirror: the offline docs pack is a mirrored last-known copy, not a live provider read; the pack row shows its mirror / offline state",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "docs-pack row keeps its pin / mirror / offline / quarantine state distinct",
                "docs source/version badge keeps provider and mirror freshness visible",
                "degraded-state: SupportedReference narrows to cached (auto-narrowed)",
                "known compatibility note: mirror freshness — offline packs are reconstructable but never read as live",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            CurrentAuthoritative,
            PolicyBlockedReference,
            &[StaleExampleFindingRow, DocsPackRow],
            seed_certified_except(
                Ax::SourceAndHandoffProvenance,
                seed_narrowed(
                    Ax::SourceAndHandoffProvenance,
                    "a docs pack in the CLI corpus is quarantined",
                    "A docs pack in the headless corpus is quarantined by policy, so the CurrentAuthoritative claim narrows to policy-blocked instead of presenting a quarantined pack as a trusted authoritative source",
                    Trig::QuarantinedPackShownAsTrusted,
                ),
            ),
            Some(seed_narrow(
                Ax::SourceAndHandoffProvenance,
                CurrentAuthoritative,
                PolicyBlockedReference,
                "Policy-blocked pack: a docs pack in this corpus is quarantined; the CLI output shows the quarantine state and the stale-example findings rather than asserting trust",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "stale-example finding row keeps its version-anchored drift explicit in the CLI output",
                "docs-pack row keeps its quarantine state explicit in the structured output",
                "source/handoff: CurrentAuthoritative narrows to policy-blocked (auto-narrowed)",
                "known compatibility note: pack quarantine — a quarantined pack never reads as trusted in headless output",
            ],
        ),
    ]
}
