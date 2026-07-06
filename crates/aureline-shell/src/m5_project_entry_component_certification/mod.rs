//! M05-843 surface certification over the frozen M5 project-entry component
//! matrix.
//!
//! Where the freeze matrix ([`crate::m5_project_entry_components`]) defines the
//! ten reusable start-center quick-action, recent-work, workspace-switcher,
//! restore-prompt, entry-chooser, entry-review, destination-collision,
//! post-entry-handoff, admission-checkpoint, and archetype-readiness cards,
//! rows, and sheets, and the M05-842 consumer lane
//! ([`crate::add_shared_start_center_system_open_deep_link_and_cli_headless_project_entry_component_consumers`])
//! adopts them across five consumer classes, this closing capstone *certifies*
//! that the shared components behave consistently on every claimed M5
//! project-entry surface.
//!
//! It is keyed on the claimed **surface** (Start Center, command palette,
//! system-open, deep-link, CLI/headless, template/prebuild, clone, import, and
//! restore), not on family or consumer group. Each [`EntrySurfaceCertificationRow`]
//! certifies one surface across five truth axes — profile/remote badge parity,
//! restore class, trust posture, first-useful-work routing, and (always-on)
//! export parity — and either passes (green), auto-narrows its interactive claim
//! to the weakest supported ceiling (yellow), or is blocked (red) when a degraded
//! axis is hidden behind a full-truth claim inherited from a healthier lane.
//!
//! The invariant is: **a degraded axis must produce a visible tier narrowing**.
//! A surface that keeps a full-entry claim while one of its truth axes is not
//! current is over-claiming and blocks; a surface that discloses the reduction by
//! narrowing its claim (with a bound reason and a downgrade trigger) is honestly
//! yellow. The always-on export-parity axis must always stay certified, so
//! support and automation can reconstruct the certified surface state from the
//! same object identity the user saw.
//!
//! Every row cites exactly one canonical release-proof bundle
//! ([`ENTRY_CERT_CANONICAL_BUNDLE_REF`]) — the frozen project-entry component
//! release proof — rather than cloning per-surface evidence. The packet is
//! metadata-only: raw file paths, clone URLs, credentials, remote hosts, and
//! device identifiers never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-project-entry-component-certification.schema.json`](../../../../schemas/ui/m5-project-entry-component-certification.schema.json).
//! The contract doc is
//! [`docs/opening-projects/m5_project_entry_component_certification_contract.md`](../../../../docs/opening-projects/m5_project_entry_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_start_center_system_open_deep_link_and_cli_headless_project_entry_component_consumers as entry_consumers;
use entry_consumers::{CopyExportParity, M5ProjectEntryComponentFamily};

/// Schema version stamped on the M05-843 certification packet.
pub const ENTRY_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`EntrySurfaceCertificationPacket`].
pub const ENTRY_CERT_RECORD_KIND: &str = "m5_project_entry_component_certification_packet";

/// Stable record-kind tag carried by each [`EntrySurfaceCertificationRow`].
pub const ENTRY_CERT_ROW_RECORD_KIND: &str = "m5_project_entry_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const ENTRY_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-project-entry-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const ENTRY_CERT_DOC_REF: &str =
    "docs/opening-projects/m5_project_entry_component_certification_contract.md";

/// Repo-relative path of the frozen project-entry component matrix fixture the
/// certified surfaces render.
pub const ENTRY_CERT_MATRIX_REF: &str =
    crate::m5_project_entry_components::M5_PROJECT_ENTRY_COMPONENT_FIXTURE_REF;

/// The one canonical release-proof bundle every certified surface cites as its
/// first-resolved component truth. All nine surfaces point back to it rather
/// than cloning per-surface evidence.
pub const ENTRY_CERT_CANONICAL_BUNDLE_REF: &str =
    "artifacts/release/m5-project-entry-component-proof/packet.json";

/// The M05-842 consumer support export the certification builds on. Recorded as
/// a supporting evidence ref on every row.
pub const ENTRY_CERT_CONSUMER_BUNDLE_REF: &str =
    "artifacts/release/m5-project-entry-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const ENTRY_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-project-entry-component-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const ENTRY_CERT_CSV_REF: &str =
    "artifacts/release/m5-project-entry-component-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const ENTRY_CERT_REPORT_REF: &str =
    "artifacts/release/m5-project-entry-component-certification-proof/report.md";

/// The nine claimed M5 project-entry surfaces this capstone certifies. Keyed on
/// the surface a user actually enters through, not on the reusable component
/// family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProjectEntryCertifiedSurface {
    /// The Start Center home surface.
    StartCenter,
    /// The command palette entry surface.
    CommandPalette,
    /// The system-open / file-association intake surface.
    SystemOpen,
    /// The protocol / deep-link / browser-mobile handoff surface.
    DeepLink,
    /// The CLI / headless entry surface.
    CliHeadless,
    /// The template / prebuild start-of-work surface.
    TemplatePrebuild,
    /// The repository clone surface.
    Clone,
    /// The portable-state / handoff-packet import surface.
    Import,
    /// The session / crash-recovery restore surface.
    Restore,
}

impl M5ProjectEntryCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5ProjectEntryCertifiedSurface; 9] = [
        M5ProjectEntryCertifiedSurface::StartCenter,
        M5ProjectEntryCertifiedSurface::CommandPalette,
        M5ProjectEntryCertifiedSurface::SystemOpen,
        M5ProjectEntryCertifiedSurface::DeepLink,
        M5ProjectEntryCertifiedSurface::CliHeadless,
        M5ProjectEntryCertifiedSurface::TemplatePrebuild,
        M5ProjectEntryCertifiedSurface::Clone,
        M5ProjectEntryCertifiedSurface::Import,
        M5ProjectEntryCertifiedSurface::Restore,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartCenter => "start_center",
            Self::CommandPalette => "command_palette",
            Self::SystemOpen => "system_open",
            Self::DeepLink => "deep_link",
            Self::CliHeadless => "cli_headless",
            Self::TemplatePrebuild => "template_prebuild",
            Self::Clone => "clone",
            Self::Import => "import",
            Self::Restore => "restore",
        }
    }

    /// True when the surface can reach a remote host (clone, import, deep-link,
    /// CLI/headless), so its profile/remote badge axis carries live-vs-cached
    /// truth rather than a local-only constant.
    pub const fn is_remote_capable(self) -> bool {
        matches!(
            self,
            Self::DeepLink | Self::CliHeadless | Self::Clone | Self::Import
        )
    }
}

/// The five truth axes a certified surface is scored on. The first four carry
/// the compatibility notes the spec calls out (profile/remote badges, restore
/// classes, trust posture, first-useful-work routing); the fifth, export
/// parity, is always-on and must stay certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryCertificationAxis {
    /// Profile / remote badge parity (managed, remote, cached-only, offline).
    ProfileRemoteBadge,
    /// Restore-fidelity class parity (exact, compatible, layout-only, drafts,
    /// evidence-only, no-restore).
    RestoreClass,
    /// Trust posture parity (root identity, trust class, host/auth posture).
    TrustPosture,
    /// First-useful-work routing parity (attributable routing + same-weight
    /// plain-open path).
    FirstUsefulWorkRouting,
    /// Export / support parity (always-on): the certified surface state is
    /// copyable as text / JSON / Markdown for support and automation.
    ExportParity,
}

impl EntryCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [EntryCertificationAxis; 5] = [
        EntryCertificationAxis::ProfileRemoteBadge,
        EntryCertificationAxis::RestoreClass,
        EntryCertificationAxis::TrustPosture,
        EntryCertificationAxis::FirstUsefulWorkRouting,
        EntryCertificationAxis::ExportParity,
    ];

    /// The always-on export-parity axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::ExportParity)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProfileRemoteBadge => "profile_remote_badge",
            Self::RestoreClass => "restore_class",
            Self::TrustPosture => "trust_posture",
            Self::FirstUsefulWorkRouting => "first_useful_work_routing",
            Self::ExportParity => "export_parity",
        }
    }
}

/// The interactive-claim ceiling a surface asserts (and the weakest ceiling it
/// is certified down to when an axis is not current). Ranked so certification
/// can only narrow a claim, never widen it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryClaimTier {
    /// Full interactive entry: open / clone / import / restore directly.
    FullEntry,
    /// Reviewed entry: the verb is staged behind an explicit review before any
    /// write, clone, import, restore, or scope widening.
    ReviewedEntry,
    /// Inspect-only: read every governed label but take no action.
    InspectOnly,
    /// Export-only: reconstruct the surface state from an export packet.
    ExportOnly,
}

impl EntryClaimTier {
    /// Every claim tier, in declaration (descending capability) order.
    pub const ALL: [EntryClaimTier; 4] = [
        EntryClaimTier::FullEntry,
        EntryClaimTier::ReviewedEntry,
        EntryClaimTier::InspectOnly,
        EntryClaimTier::ExportOnly,
    ];

    /// Capability rank; higher is more capable. Certification may lower this but
    /// never raise it above the claimed tier.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::FullEntry => 4,
            Self::ReviewedEntry => 3,
            Self::InspectOnly => 2,
            Self::ExportOnly => 1,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullEntry => "full_entry",
            Self::ReviewedEntry => "reviewed_entry",
            Self::InspectOnly => "inspect_only",
            Self::ExportOnly => "export_only",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds
    /// to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth
    /// claim inherited from a healthier lane.
    UndisclosedDrift,
}

impl AxisCertificationState {
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
/// author — always recomputed from the axis outcomes and tier narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClaimStatus {
    /// Full standing: every axis certified, claimed tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows
    /// visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, export parity drops,
    /// or the narrowing is inconsistent.
    Red,
}

impl SurfaceClaimStatus {
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

/// One axis outcome on one certified surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: EntryCertificationAxis,
    /// The certification state of the axis.
    pub state: AxisCertificationState,
    /// The compatibility note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The visible downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<String>,
}

impl EntryAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a visible
    ///   trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no
    ///   visible trigger (that is exactly what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            AxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            AxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                let trigger_ok = self
                    .downgrade_trigger
                    .as_deref()
                    .is_some_and(|t| !t.trim().is_empty());
                reason_ok && trigger_ok
            }
            AxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a surface applies when a truth axis is not
/// current. Present iff the certified tier is strictly below the claimed tier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: EntryCertificationAxis,
    /// The claimed tier the surface would deliver at full parity.
    pub from_tier: EntryClaimTier,
    /// The weakest supported tier the surface is certified down to.
    pub to_tier: EntryClaimTier,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 project-entry surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntrySurfaceCertificationRow {
    /// Record kind; must equal [`ENTRY_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ENTRY_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5ProjectEntryCertifiedSurface,
    /// The interactive-claim ceiling the surface asserts.
    pub claimed_tier: EntryClaimTier,
    /// The weakest supported ceiling the surface is certified down to. Must be
    /// no more capable than `claimed_tier`.
    pub certified_tier: EntryClaimTier,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5ProjectEntryComponentFamily>,
    /// One outcome per [`EntryCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<EntryAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_tier` is below
    /// `claimed_tier`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<EntryClaimAutoNarrow>,
    /// The one canonical release-proof bundle this surface cites. Must equal
    /// [`ENTRY_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: SurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: CopyExportParity,
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

impl EntrySurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: EntryCertificationAxis) -> Option<&EntryAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<EntryCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && EntryCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes.iter().all(EntryAxisOutcome::well_formed)
    }

    /// True when the surface narrows its interactive claim below what it asserts.
    pub fn is_tier_narrowed(&self) -> bool {
        self.certified_tier.capability_rank() < self.claimed_tier.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<EntryCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == AxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the surface verdict from its axes and tier narrowing. This is the
    /// heart of the capstone: a degraded axis must produce a visible tier
    /// narrowing, export parity must always certify, and the narrowing must be
    /// consistent.
    pub fn derive_status(&self) -> SurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != ENTRY_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return SurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never widen it.
        if self.certified_tier.capability_rank() > self.claimed_tier.capability_rank() {
            return SurfaceClaimStatus::Red;
        }

        // The always-on export-parity axis must stay certified.
        match self.axis(EntryCertificationAxis::ExportParity) {
            Some(o) if o.state == AxisCertificationState::Certified => {}
            _ => return SurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == AxisCertificationState::UndisclosedDrift)
        {
            return SurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let tier_narrowed = self.is_tier_narrowed();

        match (&self.claim_auto_narrow, tier_narrowed) {
            // Spurious narrowing structure without a tier reduction.
            (Some(_), false) => return SurfaceClaimStatus::Red,
            // A tier reduction with no disclosed narrowing structure.
            (None, true) => return SurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_tier != self.claimed_tier
                    || narrow.to_tier != self.certified_tier
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return SurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if tier_narrowed {
            // A disclosed, consistently-bound narrowing.
            return SurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a
        // hidden overclaim inheriting a healthier lane's truth.
        if !narrowed.is_empty() {
            return SurfaceClaimStatus::Red;
        }

        SurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == ENTRY_CERT_ROW_RECORD_KIND
            && self.schema_version == ENTRY_CERT_SCHEMA_VERSION
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
            claimed = self.claimed_tier.as_str(),
            certified = self.certified_tier.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
        )
    }
}

/// Rolled-up summary of an M05-843 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntrySurfaceCertificationSummary {
    pub row_count: usize,
    pub surface_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_surfaces_present: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub every_axis_covered_on_every_row: bool,
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`EntrySurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntrySurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<EntrySurfaceCertificationRow>,
}

/// Checked-in M05-843 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntrySurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<EntrySurfaceCertificationRow>,
    pub summary: EntrySurfaceCertificationSummary,
}

impl EntrySurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: EntrySurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: ENTRY_CERT_SCHEMA_VERSION,
            record_kind: ENTRY_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: EntrySurfaceCertificationSummary {
                row_count: 0,
                surface_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_surfaces_present: false,
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
    pub fn represented_surfaces(&self) -> BTreeSet<M5ProjectEntryCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5ProjectEntryCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether an export-parity axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(EntryCertificationAxis::ExportParity)
                .is_some_and(|o| o.state == AxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> EntrySurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == SurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == SurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == SurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self.rows.iter().all(EntrySurfaceCertificationRow::status_is_fresh);

        EntrySurfaceCertificationSummary {
            row_count: self.rows.len(),
            surface_count: surfaces.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_surfaces_present: self.all_surfaces_present(),
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == ENTRY_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(EntrySurfaceCertificationRow::covers_all_axes),
            narrowed_surface_count: self
                .rows
                .iter()
                .filter(|r| r.is_tier_narrowed())
                .count(),
            report_clean: all_publishable && all_fresh && self.all_surfaces_present(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<EntryCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != ENTRY_CERT_SCHEMA_VERSION {
            violations.push(EntryCertificationViolation::SchemaVersion {
                expected: ENTRY_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != ENTRY_CERT_RECORD_KIND {
            violations.push(EntryCertificationViolation::RecordKind {
                expected: ENTRY_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(EntryCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != ENTRY_CERT_CANONICAL_BUNDLE_REF {
            violations.push(EntryCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(EntryCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(EntryCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(EntryCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(EntryCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != ENTRY_CERT_CANONICAL_BUNDLE_REF {
                violations.push(EntryCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // Export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(EntryCertificationAxis::ExportParity)
                    .is_none_or_state_not_certified()
            {
                violations.push(EntryCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never widen a claim.
            if row.certified_tier.capability_rank() > row.claimed_tier.capability_rank() {
                violations.push(EntryCertificationViolation::CertifiedTierExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(EntryCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == SurfaceClaimStatus::Red {
                violations.push(EntryCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(EntryCertificationViolation::SurfaceCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(EntryCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(EntryCertificationViolation::RawBoundaryMaterialInExport);
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
            "row_id,surface,claimed_tier,certified_tier,status,narrowed_axes,binding_axis\n",
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
                claimed = row.claimed_tier.as_str(),
                certified = row.certified_tier.as_str(),
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
        out.push_str("# M5 Project-Entry Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Canonical bundle: `{}`\n", self.canonical_bundle_ref));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5ProjectEntryCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
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
pub fn current_m5_project_entry_component_certification_export(
) -> Result<EntrySurfaceCertificationPacket, EntryCertificationArtifactError> {
    let packet: EntrySurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-project-entry-component-certification-proof/support_export.json"
    )))
    .map_err(EntryCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(EntryCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum EntryCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<EntryCertificationViolation>),
}

impl fmt::Display for EntryCertificationArtifactError {
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

impl Error for EntryCertificationArtifactError {}

/// Validation failure for M05-843 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntryCertificationViolation {
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
    CertifiedTierExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for EntryCertificationViolation {
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
                write!(f, "packet does not cite the canonical release-proof bundle")
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
                write!(f, "row {id} does not cite the one canonical release-proof bundle")
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::CertifiedTierExceedsClaim { id } => {
                write!(f, "row {id} certifies a tier more capable than the claimed one")
            }
            Self::StatusDerivationStale { id } => {
                write!(f, "row {id} stored status disagrees with a fresh derivation")
            }
            Self::SurfaceBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a full claim, \
export parity dropped, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(f, "not every claimed M5 project-entry surface is certified exactly once")
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for EntryCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&EntryAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != AxisCertificationState::Certified,
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
    if lower.contains("get started") {
        return true;
    }
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
            | "read only"
            | "read-only"
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
// Seed builder — the one source of truth shared by the tests, the example
// binary, and the on-disk support export so all three stay byte-aligned.
// --------------------------------------------------------------------------

/// Builds the canonical, checked-in M05-843 certification packet. Certifies all
/// nine claimed M5 project-entry surfaces: four deliver their claim (green) and
/// five auto-narrow a not-current truth axis to a reviewed-entry ceiling
/// (yellow). No surface hides drift (red).
pub fn seeded_m5_project_entry_component_certification_packet() -> EntrySurfaceCertificationPacket {
    EntrySurfaceCertificationPacket::new(EntrySurfaceCertificationPacketInput {
        packet_id: "m5-project-entry-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-06T00:00:00Z".to_owned(),
        matrix_ref: ENTRY_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: ENTRY_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:project-entry-certification:{id}"),
        ENTRY_CERT_CONSUMER_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: EntryCertificationAxis) -> &'static str {
    match axis {
        EntryCertificationAxis::ProfileRemoteBadge => {
            "profile and remote badges match the live workspace descriptor"
        }
        EntryCertificationAxis::RestoreClass => {
            "restore-fidelity class matches the recovery checkpoint"
        }
        EntryCertificationAxis::TrustPosture => {
            "root identity, trust class, and host/auth posture are current"
        }
        EntryCertificationAxis::FirstUsefulWorkRouting => {
            "first-useful-work routing is attributable with a same-weight plain-open path"
        }
        EntryCertificationAxis::ExportParity => {
            "surface state exports as text / JSON / Markdown for support replay"
        }
    }
}

fn seed_certified(axis: EntryCertificationAxis) -> EntryAxisOutcome {
    EntryAxisOutcome {
        axis,
        state: AxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: EntryCertificationAxis,
    note: &str,
    reason: &str,
    trigger: &str,
) -> EntryAxisOutcome {
    EntryAxisOutcome {
        axis,
        state: AxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger.to_owned()),
    }
}

fn seed_all_certified() -> Vec<EntryAxisOutcome> {
    EntryCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: EntryCertificationAxis,
    outcome: EntryAxisOutcome,
) -> Vec<EntryAxisOutcome> {
    EntryCertificationAxis::ALL
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
    surface: M5ProjectEntryCertifiedSurface,
    claimed_tier: EntryClaimTier,
    certified_tier: EntryClaimTier,
    consumed_families: &[M5ProjectEntryComponentFamily],
    axis_outcomes: Vec<EntryAxisOutcome>,
    claim_auto_narrow: Option<EntryClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> EntrySurfaceCertificationRow {
    let mut row = EntrySurfaceCertificationRow {
        record_kind: ENTRY_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: ENTRY_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_tier,
        certified_tier,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        canonical_bundle_ref: ENTRY_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: SurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes.iter().map(|n| (*n).to_owned()).collect(),
        source_refs: vec![ENTRY_CERT_MATRIX_REF.to_owned(), ENTRY_CERT_SCHEMA_REF.to_owned()],
        observed_at: "2026-07-06T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: EntryCertificationAxis,
    from_tier: EntryClaimTier,
    to_tier: EntryClaimTier,
    label: &str,
) -> EntryClaimAutoNarrow {
    EntryClaimAutoNarrow {
        binding_axis,
        from_tier,
        to_tier,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<EntrySurfaceCertificationRow> {
    use EntryCertificationAxis::*;
    use EntryClaimTier::*;
    use M5ProjectEntryCertifiedSurface as S;
    use M5ProjectEntryComponentFamily::*;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:start-center",
            S::StartCenter,
            FullEntry,
            FullEntry,
            &[
                StartCenterQuickActionCard,
                RecentWorkRow,
                RestorePromptCard,
                WorkspaceSwitcherEntry,
            ],
            seed_all_certified(),
            None,
            &["surface", "claimed_tier", "certified_tier", "status"],
            &[
                "profile/remote badges: local + managed profiles current",
                "restore classes: exact/compatible/layout/drafts/evidence/none all surfaced",
                "trust posture: root identity and trust class shown before open",
                "first-useful-work routing: attributable with same-weight plain open",
            ],
        ),
        seed_row(
            "cert:command-palette",
            S::CommandPalette,
            FullEntry,
            FullEntry,
            &[EntryChooserRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_tier", "certified_tier", "status"],
            &[
                "profile/remote badges: mirror the Start Center descriptor",
                "restore classes: restore chooser row keeps fidelity class",
                "trust posture: open/clone/import/restore stay distinct verbs",
                "first-useful-work routing: plain open routes to ordinary editing",
            ],
        ),
        seed_row(
            "cert:import",
            S::Import,
            FullEntry,
            FullEntry,
            &[EntryReviewSheet, PostEntryHandoffCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_tier", "certified_tier", "status"],
            &[
                "profile/remote badges: import source profile is current",
                "restore classes: imported handoff packet keeps restore fidelity",
                "trust posture: write scope and side effects reviewed before import",
                "first-useful-work routing: post-entry handoff routes attributably",
            ],
        ),
        seed_row(
            "cert:cli-headless",
            S::CliHeadless,
            ExportOnly,
            ExportOnly,
            &[ArchetypeReadinessRow, AdmissionCheckpointCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_tier", "certified_tier", "status"],
            &[
                "profile/remote badges: emitted as typed tokens for scripts",
                "restore classes: readiness bucket emitted without auto-install",
                "trust posture: root identity and trust class emitted, not widened",
                "first-useful-work routing: archetype/readiness emitted for the shell to act on",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:system-open",
            S::SystemOpen,
            FullEntry,
            ReviewedEntry,
            &[EntryReviewSheet, EntryChooserRow],
            seed_certified_except(
                TrustPosture,
                seed_narrowed(
                    TrustPosture,
                    "OS-handed-off target trust is not yet established",
                    "System-open trust posture is not current until the literal target and host/auth posture are reviewed",
                    "review_required_before_open",
                ),
            ),
            Some(seed_narrow(
                TrustPosture,
                FullEntry,
                ReviewedEntry,
                "Review-required system open: confirm the literal target, resulting mode, and write scope before Aureline opens what the OS handed off",
            )),
            &["surface", "claimed_tier", "certified_tier", "status", "binding_axis"],
            &[
                "profile/remote badges: local intake profile current",
                "restore classes: not applicable to a fresh system open",
                "trust posture: not current until reviewed (auto-narrowed)",
                "first-useful-work routing: plain open preserved post-review",
            ],
        ),
        seed_row(
            "cert:deep-link",
            S::DeepLink,
            FullEntry,
            ReviewedEntry,
            &[EntryReviewSheet, PostEntryHandoffCard],
            seed_certified_except(
                TrustPosture,
                seed_narrowed(
                    TrustPosture,
                    "deep-link protocol target trust is unestablished",
                    "Deep-link trust posture is not current until the protocol target and recovery path are reviewed",
                    "review_required_before_open",
                ),
            ),
            Some(seed_narrow(
                TrustPosture,
                FullEntry,
                ReviewedEntry,
                "Review-required deep link: the protocol target, resulting mode, and recovery path are shown before Aureline opens or writes anything",
            )),
            &["surface", "claimed_tier", "certified_tier", "status", "binding_axis"],
            &[
                "profile/remote badges: remote handoff origin shown",
                "restore classes: preserved when a review link resumes a session",
                "trust posture: not current until reviewed (auto-narrowed)",
                "first-useful-work routing: review-link route attributable",
            ],
        ),
        seed_row(
            "cert:clone",
            S::Clone,
            FullEntry,
            ReviewedEntry,
            &[EntryChooserRow, DestinationCollisionSheet],
            seed_certified_except(
                ProfileRemoteBadge,
                seed_narrowed(
                    ProfileRemoteBadge,
                    "remote profile badge parity lags the origin host",
                    "Clone remote/profile badge parity is not current until the origin host and auth posture are reviewed",
                    "review_required_before_clone",
                ),
            ),
            Some(seed_narrow(
                ProfileRemoteBadge,
                FullEntry,
                ReviewedEntry,
                "Review-required clone: confirm the origin host, auth posture, and destination before Aureline materializes a working tree",
            )),
            &["surface", "claimed_tier", "certified_tier", "status", "binding_axis"],
            &[
                "profile/remote badges: not current until reviewed (auto-narrowed)",
                "restore classes: not applicable to a fresh clone",
                "trust posture: destination-collision safe actions preserved",
                "first-useful-work routing: post-clone routing attributable",
            ],
        ),
        seed_row(
            "cert:restore",
            S::Restore,
            FullEntry,
            ReviewedEntry,
            &[RestorePromptCard],
            seed_certified_except(
                RestoreClass,
                seed_narrowed(
                    RestoreClass,
                    "restore fidelity is partial (compatible, not exact)",
                    "Restore class parity is not current: only a compatible restore is available, not an exact one",
                    "review_required_before_restore",
                ),
            ),
            Some(seed_narrow(
                RestoreClass,
                FullEntry,
                ReviewedEntry,
                "Reviewed restore: only a compatible restore is offered; confirm the layout and recovered drafts before resuming",
            )),
            &["surface", "claimed_tier", "certified_tier", "status", "binding_axis"],
            &[
                "profile/remote badges: prior workspace profile shown",
                "restore classes: only compatible restore available (auto-narrowed)",
                "trust posture: resulting mode and write scope shown before restore",
                "first-useful-work routing: recovered drafts routed explicitly",
            ],
        ),
        seed_row(
            "cert:template-prebuild",
            S::TemplatePrebuild,
            FullEntry,
            ReviewedEntry,
            &[PostEntryHandoffCard, AdmissionCheckpointCard],
            seed_certified_except(
                FirstUsefulWorkRouting,
                seed_narrowed(
                    FirstUsefulWorkRouting,
                    "prebuild snapshot routing is provisional",
                    "First-useful-work routing is not current until the prebuild snapshot freshness is confirmed",
                    "review_required_before_open",
                ),
            ),
            Some(seed_narrow(
                FirstUsefulWorkRouting,
                FullEntry,
                ReviewedEntry,
                "Reviewed template/prebuild open: confirm the prebuild snapshot freshness and first-useful-work route before starting",
            )),
            &["surface", "claimed_tier", "certified_tier", "status", "binding_axis"],
            &[
                "profile/remote badges: template starter profile shown",
                "restore classes: not applicable to a fresh template open",
                "trust posture: admission checkpoint keeps continue-without available",
                "first-useful-work routing: provisional until snapshot confirmed (auto-narrowed)",
            ],
        ),
    ]
}
