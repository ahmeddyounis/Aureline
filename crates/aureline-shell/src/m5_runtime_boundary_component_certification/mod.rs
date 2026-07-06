//! M05-859 surface certification over the frozen M5 runtime-boundary component
//! matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix`])
//! defines the six reusable terminal-tab, remote-target-pill,
//! environment-status-strip, toolchain-pin-row, presence-avatar-stack, and
//! repair-action-card components, the four M05-853..857 primitive lanes narrow
//! each one, and the M05-858 accessibility lane
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_runtime_boundary_claim_auto_narrowing`])
//! proves keyboard / screen-reader / CLI-export parity and auto-narrowing, this
//! closing capstone *certifies* that the shared component truth holds on every
//! claimed M5 runtime-collaboration-recovery surface — and auto-narrows any
//! surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user runs, shares, switches,
//! repairs, or exports execution state through (terminal, notebook console,
//! request console, preview server, debug, run/test, collaboration, Doctor,
//! support, and export), not on family or primitive lane. Each
//! [`RuntimeSurfaceCertificationRow`] certifies one surface across six truth
//! axes — visual, keyboard, screen-reader, CLI/export, degraded-state, and
//! restore-no-rerun behavior — and either passes (green), auto-narrows its
//! runtime-support claim to the weakest supported ceiling (yellow), or is
//! blocked (red) when a degraded axis is hidden behind a full-truth claim
//! inherited from a healthier lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `Live`/`Ready` claim while one of its truth axes is
//! not current is over-claiming and blocks; a surface that discloses the
//! reduction by narrowing its runtime-support claim (with a bound reason and a
//! frozen downgrade trigger) is honestly yellow. The always-on CLI/export axis
//! must always stay certified, so support and automation can reconstruct the
//! certified host / runtime / role / reversal truth from the same object
//! identity the user saw.
//!
//! Every row cites exactly one canonical release-proof bundle
//! ([`RUNTIME_CERT_CANONICAL_BUNDLE_REF`]) — the frozen runtime-boundary
//! component release proof — rather than cloning per-surface evidence. The
//! packet is metadata-only: raw file paths, remote hosts, credentials, and
//! device identifiers never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-runtime-boundary-component-certification.schema.json`](../../../../schemas/ui/m5-runtime-boundary-component-certification.schema.json).
//! The contract doc is
//! [`docs/runtime/m5_runtime_boundary_component_certification_contract.md`](../../../../docs/runtime/m5_runtime_boundary_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_runtime_boundary_claim_auto_narrowing as a11y;
use a11y::M5RuntimeSupportClaim;
use matrix::{M5RuntimeBoundaryComponentFamily, M5RuntimeBoundaryDowngradeTrigger};

/// Schema version stamped on the M05-859 certification packet.
pub const RUNTIME_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`RuntimeSurfaceCertificationPacket`].
pub const RUNTIME_CERT_RECORD_KIND: &str = "m5_runtime_boundary_component_certification_packet";

/// Stable record-kind tag carried by each [`RuntimeSurfaceCertificationRow`].
pub const RUNTIME_CERT_ROW_RECORD_KIND: &str = "m5_runtime_boundary_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const RUNTIME_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-runtime-boundary-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const RUNTIME_CERT_DOC_REF: &str =
    "docs/runtime/m5_runtime_boundary_component_certification_contract.md";

/// Repo-relative path of the frozen runtime-boundary component matrix schema the
/// certified surfaces render.
pub const RUNTIME_CERT_MATRIX_REF: &str = matrix::M5_RUNTIME_BOUNDARY_SCHEMA_REF;

/// The one canonical release-proof bundle every certified surface cites as its
/// first-resolved component truth. All ten surfaces point back to it rather than
/// cloning per-surface evidence.
pub const RUNTIME_CERT_CANONICAL_BUNDLE_REF: &str =
    "artifacts/release/m5-runtime-boundary-proof/support_export.json";

/// The M05-858 accessibility support export the certification builds on.
/// Recorded as a supporting evidence ref on every row.
pub const RUNTIME_CERT_A11Y_BUNDLE_REF: &str =
    "artifacts/release/m5-runtime-boundary-component-accessibility-fallback-proof/support_export.json";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const RUNTIME_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-runtime-boundary-component-certification-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const RUNTIME_CERT_CSV_REF: &str =
    "artifacts/release/m5-runtime-boundary-component-certification-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const RUNTIME_CERT_REPORT_REF: &str =
    "artifacts/release/m5-runtime-boundary-component-certification-proof/report.md";

/// The ten claimed M5 runtime-collaboration-recovery surfaces this capstone
/// certifies. Keyed on the surface a user actually runs, shares, switches,
/// repairs, or exports execution state through, not on the reusable component
/// family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RuntimeCertifiedSurface {
    /// The integrated terminal.
    Terminal,
    /// The notebook console / kernel surface.
    NotebookConsole,
    /// The request / REST console surface.
    RequestConsole,
    /// The preview server surface.
    PreviewServer,
    /// The debug session surface.
    Debug,
    /// The run / test runner surface.
    RunTest,
    /// The collaboration / shared-session surface.
    Collaboration,
    /// The Project Doctor / guided-repair surface.
    Doctor,
    /// The support-bundle surface.
    Support,
    /// The export / portable-state surface.
    Export,
}

impl M5RuntimeCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5RuntimeCertifiedSurface; 10] = [
        M5RuntimeCertifiedSurface::Terminal,
        M5RuntimeCertifiedSurface::NotebookConsole,
        M5RuntimeCertifiedSurface::RequestConsole,
        M5RuntimeCertifiedSurface::PreviewServer,
        M5RuntimeCertifiedSurface::Debug,
        M5RuntimeCertifiedSurface::RunTest,
        M5RuntimeCertifiedSurface::Collaboration,
        M5RuntimeCertifiedSurface::Doctor,
        M5RuntimeCertifiedSurface::Support,
        M5RuntimeCertifiedSurface::Export,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Terminal => "terminal",
            Self::NotebookConsole => "notebook_console",
            Self::RequestConsole => "request_console",
            Self::PreviewServer => "preview_server",
            Self::Debug => "debug",
            Self::RunTest => "run_test",
            Self::Collaboration => "collaboration",
            Self::Doctor => "doctor",
            Self::Support => "support",
            Self::Export => "export",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the
/// parity dimensions the spec requires verifying — visual, keyboard,
/// screen-reader, CLI/export, degraded-state, and restore-no-rerun behavior. The
/// CLI/export axis is always-on and must stay certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeCertificationAxis {
    /// Visual parity: host boundary, resolved runtime/toolchain, collaboration
    /// role, and reversal class are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same boundary/status/role/reversal truth and
    /// its actions are reachable without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never
    /// relying on color or avatar imagery alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is
    /// reconstructable as text / JSON / Markdown for support and automation.
    CliExport,
    /// Degraded-state parity: a weakened host / runtime / role / reversal posture
    /// honestly downgrades a `Live`/`Ready` claim to degraded / reconnecting /
    /// restored / policy-blocked.
    DegradedState,
    /// Restore-no-rerun parity: a restored session preserves boundary and status
    /// truth without silently re-running work.
    RestoreNoRerun,
}

impl RuntimeCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [RuntimeCertificationAxis; 6] = [
        RuntimeCertificationAxis::Visual,
        RuntimeCertificationAxis::Keyboard,
        RuntimeCertificationAxis::ScreenReader,
        RuntimeCertificationAxis::CliExport,
        RuntimeCertificationAxis::DegradedState,
        RuntimeCertificationAxis::RestoreNoRerun,
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
            Self::RestoreNoRerun => "restore_no_rerun",
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to
    /// a visible claim narrowing.
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
/// author — always recomputed from the axis outcomes and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClaimStatus {
    /// Full standing: every axis certified, claimed runtime-support tier
    /// delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity
    /// drops, or the narrowing is inconsistent.
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

/// The copy / export parity a certified surface preserves. The CLI/export axis
/// certifies only when this offers text / JSON / Markdown reconstruction and
/// prohibits a screenshot-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The boundary/status/role/reversal fields the surface preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl RuntimeExportParity {
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
pub struct RuntimeAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: RuntimeCertificationAxis,
    /// The certification state of the axis.
    pub state: AxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5RuntimeBoundaryDowngradeTrigger>,
}

impl RuntimeAxisOutcome {
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
            AxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            AxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
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
/// current. Present iff the certified claim is strictly weaker than the claimed
/// one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: RuntimeCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5RuntimeSupportClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5RuntimeSupportClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 runtime-collaboration-recovery surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeSurfaceCertificationRow {
    /// Record kind; must equal [`RUNTIME_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RUNTIME_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5RuntimeCertifiedSurface,
    /// The runtime-support claim ceiling the surface asserts.
    pub claimed_claim: M5RuntimeSupportClaim,
    /// The weakest supported claim the surface is certified down to. Must be no
    /// stronger than `claimed_claim`.
    pub certified_claim: M5RuntimeSupportClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5RuntimeBoundaryComponentFamily>,
    /// One outcome per [`RuntimeCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<RuntimeAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<RuntimeClaimAutoNarrow>,
    /// The one canonical release-proof bundle this surface cites. Must equal
    /// [`RUNTIME_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: SurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: RuntimeExportParity,
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

impl RuntimeSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: RuntimeCertificationAxis) -> Option<&RuntimeAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<RuntimeCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && RuntimeCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(RuntimeAxisOutcome::well_formed)
    }

    /// True when the surface narrows its runtime-support claim below what it
    /// asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<RuntimeCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == AxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the
    /// heart of the capstone: a degraded axis must produce a visible claim
    /// narrowing, CLI/export parity must always certify, and the narrowing must
    /// be consistent.
    pub fn derive_status(&self) -> SurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != RUNTIME_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return SurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return SurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(RuntimeCertificationAxis::CliExport) {
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
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return SurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return SurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
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

        if claim_narrowed {
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
        self.record_kind == RUNTIME_CERT_ROW_RECORD_KIND
            && self.schema_version == RUNTIME_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-859 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeSurfaceCertificationSummary {
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

/// Constructor input for [`RuntimeSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<RuntimeSurfaceCertificationRow>,
}

/// Checked-in M05-859 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<RuntimeSurfaceCertificationRow>,
    pub summary: RuntimeSurfaceCertificationSummary,
}

impl RuntimeSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: RuntimeSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: RUNTIME_CERT_SCHEMA_VERSION,
            record_kind: RUNTIME_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: RuntimeSurfaceCertificationSummary {
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
    pub fn represented_surfaces(&self) -> BTreeSet<M5RuntimeCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5RuntimeBoundaryComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5RuntimeCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one
    /// surface — proof the full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5RuntimeBoundaryComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(RuntimeCertificationAxis::CliExport)
                .is_some_and(|o| o.state == AxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> RuntimeSurfaceCertificationSummary {
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
        let all_fresh = self
            .rows
            .iter()
            .all(RuntimeSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();

        RuntimeSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == RUNTIME_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(RuntimeSurfaceCertificationRow::covers_all_axes),
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_surfaces && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<RuntimeCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != RUNTIME_CERT_SCHEMA_VERSION {
            violations.push(RuntimeCertificationViolation::SchemaVersion {
                expected: RUNTIME_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != RUNTIME_CERT_RECORD_KIND {
            violations.push(RuntimeCertificationViolation::RecordKind {
                expected: RUNTIME_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(RuntimeCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != RUNTIME_CERT_CANONICAL_BUNDLE_REF {
            violations.push(RuntimeCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(RuntimeCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(RuntimeCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(RuntimeCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(RuntimeCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != RUNTIME_CERT_CANONICAL_BUNDLE_REF {
                violations.push(RuntimeCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(RuntimeCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(RuntimeCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(RuntimeCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(RuntimeCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == SurfaceClaimStatus::Red {
                violations.push(RuntimeCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(RuntimeCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(RuntimeCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(RuntimeCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(RuntimeCertificationViolation::RawBoundaryMaterialInExport);
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
        out.push_str("# M5 Runtime-Boundary Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5RuntimeCertifiedSurface::ALL.len(),
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
pub fn current_m5_runtime_boundary_component_certification_export(
) -> Result<RuntimeSurfaceCertificationPacket, RuntimeCertificationArtifactError> {
    let packet: RuntimeSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-runtime-boundary-component-certification-proof/support_export.json"
    )))
    .map_err(RuntimeCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RuntimeCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum RuntimeCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RuntimeCertificationViolation>),
}

impl fmt::Display for RuntimeCertificationArtifactError {
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

impl Error for RuntimeCertificationArtifactError {}

/// Validation failure for M05-859 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuntimeCertificationViolation {
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
    RawBoundaryMaterialInExport,
}

impl fmt::Display for RuntimeCertificationViolation {
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
                    "not every claimed M5 runtime surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(f, "not every frozen runtime-boundary component family is certified on some surface")
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for RuntimeCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&RuntimeAxisOutcome> {
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
// Seed builder — the one source of truth shared by the tests and the on-disk
// support export so both stay byte-aligned.
// --------------------------------------------------------------------------

/// Builds the canonical, checked-in M05-859 certification packet. Certifies all
/// ten claimed M5 runtime-collaboration-recovery surfaces: six deliver their
/// claim (green) and four auto-narrow a not-current truth axis to a weaker
/// runtime-support ceiling (yellow). No surface hides drift (red).
pub fn seeded_m5_runtime_boundary_component_certification_packet(
) -> RuntimeSurfaceCertificationPacket {
    RuntimeSurfaceCertificationPacket::new(RuntimeSurfaceCertificationPacketInput {
        packet_id: "m5-runtime-boundary-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-06T00:00:00Z".to_owned(),
        matrix_ref: RUNTIME_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: RUNTIME_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:runtime-boundary-certification:{id}"),
        RUNTIME_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> RuntimeExportParity {
    RuntimeExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: RuntimeCertificationAxis) -> &'static str {
    match axis {
        RuntimeCertificationAxis::Visual => {
            "host boundary, resolved runtime/toolchain, role, and reversal class shown on-surface"
        }
        RuntimeCertificationAxis::Keyboard => {
            "the same boundary/status/role/reversal actions are keyboard-reachable"
        }
        RuntimeCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/avatar-only"
        }
        RuntimeCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support replay"
        }
        RuntimeCertificationAxis::DegradedState => {
            "a weakened posture honestly downgrades the Live/Ready claim"
        }
        RuntimeCertificationAxis::RestoreNoRerun => {
            "a restored session preserves boundary/status truth without re-running work"
        }
    }
}

fn seed_certified(axis: RuntimeCertificationAxis) -> RuntimeAxisOutcome {
    RuntimeAxisOutcome {
        axis,
        state: AxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: RuntimeCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5RuntimeBoundaryDowngradeTrigger,
) -> RuntimeAxisOutcome {
    RuntimeAxisOutcome {
        axis,
        state: AxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<RuntimeAxisOutcome> {
    RuntimeCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: RuntimeCertificationAxis,
    outcome: RuntimeAxisOutcome,
) -> Vec<RuntimeAxisOutcome> {
    RuntimeCertificationAxis::ALL
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
    surface: M5RuntimeCertifiedSurface,
    claimed_claim: M5RuntimeSupportClaim,
    certified_claim: M5RuntimeSupportClaim,
    consumed_families: &[M5RuntimeBoundaryComponentFamily],
    axis_outcomes: Vec<RuntimeAxisOutcome>,
    claim_auto_narrow: Option<RuntimeClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> RuntimeSurfaceCertificationRow {
    let mut row = RuntimeSurfaceCertificationRow {
        record_kind: RUNTIME_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: RUNTIME_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        canonical_bundle_ref: RUNTIME_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: SurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            RUNTIME_CERT_MATRIX_REF.to_owned(),
            RUNTIME_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-06T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: RuntimeCertificationAxis,
    from_claim: M5RuntimeSupportClaim,
    to_claim: M5RuntimeSupportClaim,
    label: &str,
) -> RuntimeClaimAutoNarrow {
    RuntimeClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<RuntimeSurfaceCertificationRow> {
    use M5RuntimeBoundaryComponentFamily::*;
    use M5RuntimeBoundaryDowngradeTrigger as Trig;
    use M5RuntimeCertifiedSurface as S;
    use M5RuntimeSupportClaim::*;
    use RuntimeCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:terminal",
            S::Terminal,
            Live,
            Live,
            &[TerminalTab, RemoteTargetPill, EnvironmentStatusStrip],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "host_boundary"],
            &[
                "terminal tab shows session title, host boundary, and shell-integration quality",
                "remote target pill names the host boundary and live connection state",
                "environment strip names the winning runtime source",
                "restore-no-rerun: a restored terminal keeps its transcript without re-executing",
            ],
        ),
        seed_row(
            "cert:notebook-console",
            S::NotebookConsole,
            Live,
            Live,
            &[TerminalTab, EnvironmentStatusStrip],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "runtime_source"],
            &[
                "notebook console header carries kernel host boundary and integration quality",
                "environment strip names the resolved kernel runtime source",
                "keyboard/screen-reader reach preserved for the kernel status strip",
                "restore-no-rerun: reopened notebook keeps cell outputs without re-running",
            ],
        ),
        seed_row(
            "cert:request-console",
            S::RequestConsole,
            Ready,
            Ready,
            &[EnvironmentStatusStrip, RemoteTargetPill],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "runtime_source"],
            &[
                "request console strip names the resolved runtime and target environment",
                "remote target pill names the host boundary for the request target",
                "export preserves the resolved-environment and target truth",
                "restore-no-rerun: reopened console does not silently re-send requests",
            ],
        ),
        seed_row(
            "cert:preview-server",
            S::PreviewServer,
            Ready,
            Ready,
            &[RemoteTargetPill, EnvironmentStatusStrip],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "host_boundary"],
            &[
                "preview server pill names the host boundary and connection state",
                "environment strip names the resolved preview runtime source",
                "keyboard/screen-reader reach preserved for the preview status strip",
                "restore-no-rerun: reopened preview reflects last-known state, not a re-launch",
            ],
        ),
        seed_row(
            "cert:run-test",
            S::RunTest,
            Ready,
            Ready,
            &[EnvironmentStatusStrip, ToolchainPinRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "toolchain"],
            &[
                "run/test strip names the resolved runtime source",
                "toolchain pin row explains why the winning toolchain won",
                "export preserves the winning-scope and pin truth",
                "restore-no-rerun: reopened run view shows prior results, not a re-run",
            ],
        ),
        seed_row(
            "cert:export",
            S::Export,
            Restored,
            Restored,
            &[TerminalTab, RepairActionCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "reversal_class"],
            &[
                "export packet reconstructs host boundary, runtime, role, and reversal truth",
                "restored-not-live claim is stated up front for the exported surface",
                "text / JSON / Markdown reconstruction certified for support replay",
                "restore-no-rerun: reconstructed surface never re-executes captured work",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:debug",
            S::Debug,
            Live,
            Degraded,
            &[TerminalTab, EnvironmentStatusStrip],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "debug adapter session liveness is partial",
                    "Debug session liveness is partial: the adapter is attached but not streaming a fully live session, so the Live claim narrows to degraded",
                    Trig::SessionLivenessAmbiguous,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                Live,
                Degraded,
                "Degraded debug session: the adapter is attached but session liveness is partial; the host and runtime shown are last-known",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "terminal tab keeps host boundary and integration quality through the degrade",
                "environment strip keeps the resolved runtime source visible",
                "degraded-state: Live narrows to degraded (auto-narrowed)",
                "restore-no-rerun: reattaching does not re-run the debugged program",
            ],
        ),
        seed_row(
            "cert:collaboration",
            S::Collaboration,
            Live,
            Reconnecting,
            &[PresenceAvatarStack, RemoteTargetPill],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "collaboration link dropped and is re-establishing",
                    "Collaboration presence is last-known while the shared-session link re-establishes, so the Live claim narrows to reconnecting rather than masking the role",
                    Trig::CollaborationRoleMasked,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                Live,
                Reconnecting,
                "Reconnecting collaboration: presence and role are last-known while the shared-session link re-establishes",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "presence stack keeps who is present and who is presenting through the drop",
                "remote target pill keeps the host boundary visible",
                "degraded-state: Live narrows to reconnecting (auto-narrowed)",
                "restore-no-rerun: local fallback preserves who was present and in control",
            ],
        ),
        seed_row(
            "cert:doctor",
            S::Doctor,
            Ready,
            PolicyBlocked,
            &[RepairActionCard, EnvironmentStatusStrip],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "repair reversal is policy-gated on this host",
                    "The proposed repair's exact reversal is not guaranteed under the current policy, so the Ready claim narrows to policy-blocked instead of overstating reversibility",
                    Trig::ReversibilityOverstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                Ready,
                PolicyBlocked,
                "Policy-blocked repair: exact reversal is not guaranteed under the current policy; the blast radius and what stays untouched are shown before any approval",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "repair card shows blast radius and what will change before approval",
                "environment strip keeps the resolved runtime source visible",
                "degraded-state: Ready narrows to policy-blocked (auto-narrowed)",
                "restore-no-rerun: blocked repair applies nothing, so there is no work to undo",
            ],
        ),
        seed_row(
            "cert:support",
            S::Support,
            Ready,
            Restored,
            &[RepairActionCard, TerminalTab],
            seed_certified_except(
                Ax::RestoreNoRerun,
                seed_narrowed(
                    Ax::RestoreNoRerun,
                    "support bundle reconstructs from a captured snapshot",
                    "The support surface reconstructs host / runtime / role / reversal truth from a captured snapshot, not a live session, so the Ready claim narrows to restored",
                    Trig::SessionLivenessAmbiguous,
                ),
            ),
            Some(seed_narrow(
                Ax::RestoreNoRerun,
                Ready,
                Restored,
                "Restored support view: host, runtime, role, and reversal truth are reconstructed from a captured snapshot, not a live session",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "support bundle preserves the exact boundary/status/role/reversal fields",
                "terminal transcript is reconstructed read-only in the bundle",
                "restore-no-rerun: the reconstructed bundle never re-executes captured work (auto-narrowed)",
                "CLI/export parity certified so automation can replay the bundle",
            ],
        ),
    ]
}
