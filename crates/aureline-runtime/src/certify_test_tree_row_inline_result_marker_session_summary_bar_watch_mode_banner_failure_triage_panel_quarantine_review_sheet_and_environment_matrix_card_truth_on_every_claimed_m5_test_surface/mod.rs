//! M05-915 surface certification over the frozen M5 test-explorer / watch / triage component
//! matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix`])
//! defines the seven reusable test-tree-row, inline-result-marker, session-summary-bar,
//! watch-mode-banner, failure-triage-panel, quarantine-review-sheet, and environment-matrix-card
//! components, the M05-909..912 primitive lanes narrow each one, the M05-913 consumer lane
//! ([`crate::add_shared_status_bar_activity_center_coverage_flaky_snapshot_pipeline_imported_ci_and_support_consumers_so_test_components_keep_freshness_target_watch_and_quarantine_language_aligned_across_claimed_m5_profiles`])
//! proves they are reusable across the claimed status-bar / activity / coverage / flaky /
//! snapshot / pipeline / imported-CI / support consumers, and the M05-914 accessibility /
//! auto-narrowing capstone
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_result_evidence_is_imported_or_stale_watch_fidelity_is_reduced_selection_widens_or_quarantine_state_is_expired_or_policy_blocked_across_claimed_m5_test_components`])
//! certifies keyboard / screen-reader / CLI / export parity per family, this closing capstone
//! *certifies* that the shared test-explorer / watch / triage component truth holds on every
//! claimed M5 test surface — and auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user reruns, debugs, suppresses, exports, or
//! reviews failing tests from (the test-explorer tree, editor / notebook inline markers, the
//! status-bar / session summary, the watch-mode banner, the failure-triage panel, the
//! quarantine-review sheet, imported-CI views, and CLI / export), not on component family or
//! primitive lane. Each [`TestSurfaceCertificationRow`] certifies one surface across six truth
//! axes — visual, keyboard, screen-reader, CLI/export, degraded-state, and test-intelligence /
//! suppression provenance — and either passes (green), auto-narrows its test claim to the
//! weakest supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden
//! behind a full-truth claim inherited from a healthier test lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `TrustedLiveResult` / `ReviewableResult` claim while one of its truth
//! axes is not current — the result evidence is imported or stale, watch fidelity is reduced,
//! the rerun selection widened, or the quarantine state is expired or policy-blocked — is
//! over-claiming and blocks; a surface that discloses the reduction by narrowing its test
//! claim (with a bound reason and a frozen downgrade trigger) is honestly yellow. Rerun /
//! debug / triage never loses lineage: a narrowed surface always preserves its result /
//! attempt / retry lineage continuity rather than dropping it between an imported or stale
//! reading and a live local rerun. The always-on CLI/export axis must always stay certified, so
//! support and automation can reconstruct the same identity / freshness / imported-versus-live /
//! target / environment / watch-fidelity / retry-lineage / quarantine-ownership / release-impact
//! truth from the same test identity the user saw.
//!
//! Every row cites exactly one canonical test-explorer / watch / triage component proof bundle
//! ([`TEST_CERT_CANONICAL_BUNDLE_REF`]) — the frozen component matrix release proof — rather
//! than cloning per-surface evidence. The packet is metadata-only: raw assertion diffs, log
//! bodies, redacted evidence contents, and credentials never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-test-explorer-watch-triage-component-certification.schema.json`](../../../../schemas/ui/m5-test-explorer-watch-triage-component-certification.schema.json).
//! The contract doc is
//! [`docs/testing/m5_test_explorer_watch_triage_component_certification_contract.md`](../../../../docs/testing/m5_test_explorer_watch_triage_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_status_bar_activity_center_coverage_flaky_snapshot_pipeline_imported_ci_and_support_consumers_so_test_components_keep_freshness_target_watch_and_quarantine_language_aligned_across_claimed_m5_profiles as consumers;
use crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_result_evidence_is_imported_or_stale_watch_fidelity_is_reduced_selection_widens_or_quarantine_state_is_expired_or_policy_blocked_across_claimed_m5_test_components as a11y;
use a11y::M5TestComponentClaim;
use matrix::{M5TestDowngradeTrigger, M5TestExplorerWatchTriageComponentFamily};

/// Schema version stamped on the M05-915 certification packet.
pub const TEST_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`TestSurfaceCertificationPacket`].
pub const TEST_CERT_RECORD_KIND: &str =
    "m5_test_explorer_watch_triage_component_certification_packet";

/// Stable record-kind tag carried by each [`TestSurfaceCertificationRow`].
pub const TEST_CERT_ROW_RECORD_KIND: &str =
    "m5_test_explorer_watch_triage_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const TEST_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-test-explorer-watch-triage-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const TEST_CERT_DOC_REF: &str =
    "docs/testing/m5_test_explorer_watch_triage_component_certification_contract.md";

/// Repo-relative path of the frozen test-explorer / watch / triage component matrix schema the
/// certified surfaces render.
pub const TEST_CERT_MATRIX_REF: &str = matrix::M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF;

/// The one canonical test-explorer / watch / triage component proof bundle every certified
/// surface cites as its first-resolved component truth. All eight surfaces point back to it
/// rather than cloning per-surface evidence.
pub const TEST_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_ARTIFACT_REF;

/// The M05-913 consumer-adoption test export the certification builds on. Recorded as a
/// supporting evidence ref on every row.
pub const TEST_CERT_CONSUMER_BUNDLE_REF: &str = consumers::TEST_CONSUMER_ARTIFACT_REF;

/// The M05-914 accessibility / auto-narrowing test export whose keyboard / screen-reader / CLI
/// / export parity this capstone builds on. Recorded as a supporting evidence ref on every row.
pub const TEST_CERT_A11Y_BUNDLE_REF: &str = a11y::TEST_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const TEST_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-test-explorer-watch-triage-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const TEST_CERT_CSV_REF: &str =
    "artifacts/release/m5-test-explorer-watch-triage-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const TEST_CERT_REPORT_REF: &str =
    "artifacts/release/m5-test-explorer-watch-triage-component-certification/report.md";

/// The eight claimed M5 test surfaces this capstone certifies. Keyed on the surface a user
/// actually reruns, debugs, suppresses, exports, or reviews failing tests from, not on the
/// reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestExplorerWatchTriageCertifiedSurface {
    /// The test-explorer tree surface.
    TestExplorerTree,
    /// The editor / notebook inline-marker surface.
    EditorNotebookMarkers,
    /// The status-bar / session-summary surface.
    StatusBarSessionSummary,
    /// The watch-mode banner surface.
    WatchBanner,
    /// The failure-triage panel surface.
    TriagePanel,
    /// The quarantine-review sheet surface.
    QuarantineReviewSheet,
    /// The imported-CI results view surface.
    ImportedCiView,
    /// The CLI / export consumer surface.
    CliExport,
}

impl M5TestExplorerWatchTriageCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5TestExplorerWatchTriageCertifiedSurface; 8] = [
        M5TestExplorerWatchTriageCertifiedSurface::TestExplorerTree,
        M5TestExplorerWatchTriageCertifiedSurface::EditorNotebookMarkers,
        M5TestExplorerWatchTriageCertifiedSurface::StatusBarSessionSummary,
        M5TestExplorerWatchTriageCertifiedSurface::WatchBanner,
        M5TestExplorerWatchTriageCertifiedSurface::TriagePanel,
        M5TestExplorerWatchTriageCertifiedSurface::QuarantineReviewSheet,
        M5TestExplorerWatchTriageCertifiedSurface::ImportedCiView,
        M5TestExplorerWatchTriageCertifiedSurface::CliExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestExplorerTree => "test_explorer_tree",
            Self::EditorNotebookMarkers => "editor_notebook_markers",
            Self::StatusBarSessionSummary => "status_bar_session_summary",
            Self::WatchBanner => "watch_banner",
            Self::TriagePanel => "triage_panel",
            Self::QuarantineReviewSheet => "quarantine_review_sheet",
            Self::ImportedCiView => "imported_ci_view",
            Self::CliExport => "cli_export",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity
/// dimensions the spec requires verifying — visual, keyboard, screen-reader, CLI/export,
/// degraded-state, and test-intelligence / suppression provenance. The CLI/export axis is
/// always-on and must stay certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestCertificationAxis {
    /// Visual parity: test identity class, freshness, imported/live state, target and
    /// environment, watch fidelity, retry lineage, and mute / quarantine ownership are shown on
    /// the primary surface.
    Visual,
    /// Keyboard-reach parity: the same identity / freshness / origin / target / watch / lineage
    /// / quarantine truth and its rerun / debug / review controls are reachable without a
    /// pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color
    /// or a status glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is reconstructable as
    /// text / JSON / Markdown for support and automation, from the same test identity.
    CliExport,
    /// Degraded-state parity: imported or stale evidence, reduced watch fidelity, a widened
    /// rerun selection, or an expired / policy-blocked quarantine honestly downgrades a
    /// `TrustedLiveResult` / `ReviewableResult` claim to a weaker test tier.
    DegradedState,
    /// Test-intelligence / suppression provenance parity: test identity class, freshness,
    /// imported/live state, target and environment, watch fidelity, retry lineage, mute /
    /// quarantine ownership, and release impact stay explicit before any rerun, debug,
    /// suppression, export, or review — never inheriting a healthier lane's test truth, never
    /// masking imported or stale evidence, reduced watch fidelity, a widened selection, or an
    /// expired / policy-blocked quarantine as a trusted live result, and never dropping result /
    /// attempt / retry lineage between an imported or stale reading and a live local rerun.
    TestIntelligenceAndSuppressionProvenance,
}

impl TestCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [TestCertificationAxis; 6] = [
        TestCertificationAxis::Visual,
        TestCertificationAxis::Keyboard,
        TestCertificationAxis::ScreenReader,
        TestCertificationAxis::CliExport,
        TestCertificationAxis::DegradedState,
        TestCertificationAxis::TestIntelligenceAndSuppressionProvenance,
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
            Self::TestIntelligenceAndSuppressionProvenance => {
                "test_intelligence_and_suppression_provenance"
            }
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible
    /// claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim inherited
    /// from a healthier surface.
    UndisclosedDrift,
}

impl TestAxisCertificationState {
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
pub enum TestSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed test tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, lineage is
    /// dropped, or the narrowing is inconsistent.
    Red,
}

impl TestSurfaceClaimStatus {
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
pub struct TestCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The identity / freshness / origin / target / watch / lineage / quarantine fields the
    /// surface preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl TestCertExportParity {
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
pub struct TestCertAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: TestCertificationAxis,
    /// The certification state of the axis.
    pub state: TestAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5TestDowngradeTrigger>,
}

impl TestCertAxisOutcome {
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
            TestAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            TestAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            TestAxisCertificationState::UndisclosedDrift => {
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
pub struct TestClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: TestCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5TestComponentClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5TestComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
    /// True when the narrowed surface still preserves its result / attempt / retry lineage
    /// continuity rather than dropping it between an imported or stale reading and a live local
    /// rerun.
    pub preserves_lineage_continuity: bool,
}

/// One certified M5 test surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestSurfaceCertificationRow {
    /// Record kind; must equal [`TEST_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`TEST_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5TestExplorerWatchTriageCertifiedSurface,
    /// The test-claim ceiling the surface asserts.
    pub claimed_claim: M5TestComponentClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5TestComponentClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5TestExplorerWatchTriageComponentFamily>,
    /// One outcome per [`TestCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<TestCertAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than
    /// `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<TestClaimAutoNarrow>,
    /// True when this surface never drops its result / attempt / retry lineage continuity
    /// between an imported or stale reading and a live local rerun.
    pub lineage_preserved: bool,
    /// The one canonical test-explorer / watch / triage proof bundle this surface cites. Must
    /// equal [`TEST_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: TestSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: TestCertExportParity,
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

impl TestSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: TestCertificationAxis) -> Option<&TestCertAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<TestCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && TestCertificationAxis::ALL.iter().all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(TestCertAxisOutcome::well_formed)
    }

    /// True when the surface narrows its test claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<TestCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == TestAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether a narrowed surface preserves its result / attempt / retry lineage continuity
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
    /// always certify, rerun / debug / triage must never drop lineage, and the narrowing must be
    /// consistent.
    pub fn derive_status(&self) -> TestSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != TEST_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.preserves_lineage_continuity()
        {
            return TestSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return TestSurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(TestCertificationAxis::CliExport) {
            Some(o) if o.state == TestAxisCertificationState::Certified => {}
            _ => return TestSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == TestAxisCertificationState::UndisclosedDrift)
        {
            return TestSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return TestSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return TestSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                    || !narrow.preserves_lineage_continuity
                {
                    return TestSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return TestSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden
        // overclaim inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return TestSurfaceClaimStatus::Red;
        }

        TestSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == TEST_CERT_ROW_RECORD_KIND
            && self.schema_version == TEST_CERT_SCHEMA_VERSION
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

/// Rolled-up summary of an M05-915 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestSurfaceCertificationSummary {
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

/// Constructor input for [`TestSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<TestSurfaceCertificationRow>,
}

/// Checked-in M05-915 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<TestSurfaceCertificationRow>,
    pub summary: TestSurfaceCertificationSummary,
}

impl TestSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: TestSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: TEST_CERT_SCHEMA_VERSION,
            record_kind: TEST_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: TestSurfaceCertificationSummary {
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
    pub fn represented_surfaces(&self) -> BTreeSet<M5TestExplorerWatchTriageCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5TestExplorerWatchTriageComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5TestExplorerWatchTriageCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof the
    /// full matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5TestExplorerWatchTriageComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(TestCertificationAxis::CliExport)
                .is_some_and(|o| o.state == TestAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> TestSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == TestSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == TestSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == TestSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(TestSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_preserved = self
            .rows
            .iter()
            .all(TestSurfaceCertificationRow::preserves_lineage_continuity);

        TestSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == TEST_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(TestSurfaceCertificationRow::covers_all_axes),
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
    pub fn validate(&self) -> Vec<TestCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != TEST_CERT_SCHEMA_VERSION {
            violations.push(TestCertificationViolation::SchemaVersion {
                expected: TEST_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != TEST_CERT_RECORD_KIND {
            violations.push(TestCertificationViolation::RecordKind {
                expected: TEST_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(TestCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != TEST_CERT_CANONICAL_BUNDLE_REF {
            violations.push(TestCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(TestCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(TestCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(TestCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(TestCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != TEST_CERT_CANONICAL_BUNDLE_REF {
                violations.push(TestCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(TestCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(TestCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Rerun / debug / triage must never drop lineage.
            if !row.preserves_lineage_continuity() {
                violations.push(TestCertificationViolation::LineageDropped {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(TestCertificationViolation::CertifiedClaimExceedsClaim {
                    id: row.row_id.clone(),
                });
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(TestCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == TestSurfaceClaimStatus::Red {
                violations.push(TestCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(TestCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(TestCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(TestCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(TestCertificationViolation::RawTestMaterialInExport);
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
        out.push_str("# M5 Test-Explorer / Watch / Triage Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5TestExplorerWatchTriageCertifiedSurface::ALL.len(),
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
pub fn current_m5_test_explorer_watch_triage_component_certification_export(
) -> Result<TestSurfaceCertificationPacket, TestCertificationArtifactError> {
    let packet: TestSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-test-explorer-watch-triage-component-certification/support_export.json"
    )))
    .map_err(TestCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TestCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum TestCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TestCertificationViolation>),
}

impl fmt::Display for TestCertificationArtifactError {
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

impl Error for TestCertificationArtifactError {}

/// Validation failure for M05-915 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestCertificationViolation {
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
    RawTestMaterialInExport,
}

impl fmt::Display for TestCertificationViolation {
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
                    "packet does not cite the canonical test-explorer / watch / triage proof bundle"
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
                    "row {id} does not cite the one canonical test-explorer / watch / triage proof bundle"
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
                    "row {id} drops result / attempt / retry lineage continuity (a narrowed surface must preserve its lineage between an imported or stale reading and a live local rerun)"
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
                    "not every claimed M5 test surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen test-explorer / watch / triage component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawTestMaterialInExport => {
                write!(f, "export contains raw test material")
            }
        }
    }
}

impl Error for TestCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&TestCertAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != TestAxisCertificationState::Certified,
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
            | "reduced watch"
            | "stale"
            | "cached"
            | "unverified"
            | "offline"
            | "blocked"
            | "policy blocked"
            | "policy_blocked"
            | "paused"
            | "interrupted"
            | "incomplete"
            | "uncertain"
            | "imported"
            | "widened"
            | "widened selection"
            | "restricted"
            | "quarantined"
            | "expired"
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

/// Builds the canonical, checked-in M05-915 certification packet. Certifies all eight claimed
/// M5 test surfaces: four deliver their claim (green) and four auto-narrow a not-current truth
/// axis to a weaker test ceiling (yellow). No surface hides drift (red), and no surface drops
/// result / attempt / retry lineage on rerun, debug, or triage.
pub fn seeded_m5_test_explorer_watch_triage_component_certification_packet(
) -> TestSurfaceCertificationPacket {
    TestSurfaceCertificationPacket::new(TestSurfaceCertificationPacketInput {
        packet_id: "m5-test-explorer-watch-triage-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: TEST_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: TEST_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:test-explorer-watch-triage-certification:{id}"),
        TEST_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        TEST_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> TestCertExportParity {
    TestCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: TestCertificationAxis) -> &'static str {
    match axis {
        TestCertificationAxis::Visual => {
            "test identity class, freshness, imported/live state, target and environment, watch fidelity, retry lineage, and mute/quarantine ownership shown on-surface"
        }
        TestCertificationAxis::Keyboard => {
            "the same identity/freshness/origin/target/watch/lineage/quarantine truth and its rerun/debug/review controls are keyboard-reachable"
        }
        TestCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only"
        }
        TestCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support from the same test identity"
        }
        TestCertificationAxis::DegradedState => {
            "imported or stale evidence, reduced watch fidelity, a widened rerun selection, or an expired/policy-blocked quarantine honestly downgrades the TrustedLiveResult/ReviewableResult claim"
        }
        TestCertificationAxis::TestIntelligenceAndSuppressionProvenance => {
            "test identity class, freshness, imported/live state, target and environment, watch fidelity, retry lineage, mute/quarantine ownership, and release impact stay explicit before any rerun, debug, suppression, export, or review; rerun/debug/triage never drops lineage"
        }
    }
}

fn seed_certified(axis: TestCertificationAxis) -> TestCertAxisOutcome {
    TestCertAxisOutcome {
        axis,
        state: TestAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: TestCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5TestDowngradeTrigger,
) -> TestCertAxisOutcome {
    TestCertAxisOutcome {
        axis,
        state: TestAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<TestCertAxisOutcome> {
    TestCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: TestCertificationAxis,
    outcome: TestCertAxisOutcome,
) -> Vec<TestCertAxisOutcome> {
    TestCertificationAxis::ALL
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
    surface: M5TestExplorerWatchTriageCertifiedSurface,
    claimed_claim: M5TestComponentClaim,
    certified_claim: M5TestComponentClaim,
    consumed_families: &[M5TestExplorerWatchTriageComponentFamily],
    axis_outcomes: Vec<TestCertAxisOutcome>,
    claim_auto_narrow: Option<TestClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> TestSurfaceCertificationRow {
    let mut row = TestSurfaceCertificationRow {
        record_kind: TEST_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: TEST_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        lineage_preserved: true,
        canonical_bundle_ref: TEST_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: TestSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            TEST_CERT_MATRIX_REF.to_owned(),
            TEST_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-07T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: TestCertificationAxis,
    from_claim: M5TestComponentClaim,
    to_claim: M5TestComponentClaim,
    label: &str,
) -> TestClaimAutoNarrow {
    TestClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
        preserves_lineage_continuity: true,
    }
}

fn seeded_rows() -> Vec<TestSurfaceCertificationRow> {
    use M5TestComponentClaim::*;
    use M5TestDowngradeTrigger as Trig;
    use M5TestExplorerWatchTriageCertifiedSurface as S;
    use M5TestExplorerWatchTriageComponentFamily::*;
    use TestCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:test-explorer-tree",
            S::TestExplorerTree,
            TrustedLiveResult,
            TrustedLiveResult,
            &[TestTreeRow, EnvironmentMatrixCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "identity_class"],
            &[
                "the test-tree row keeps its identity class, imported/live origin, freshness, and target/environment chip explicit before a rerun",
                "the environment-matrix card keeps its target class and environment lane explicit rather than implying a single equivalent run",
                "keyboard/screen-reader reach preserved for the test-tree row and the environment-matrix card",
                "provenance: a live tree row never leaves its identity class or imported/live origin implicit before rerun",
            ],
        ),
        seed_row(
            "cert:editor-notebook-markers",
            S::EditorNotebookMarkers,
            TrustedLiveResult,
            TrustedLiveResult,
            &[InlineResultMarker, TestTreeRow],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "result_freshness"],
            &[
                "the inline result marker keeps its verdict, freshness, and live/imported origin explicit in the editor and notebook gutter",
                "the test-tree row keeps its identity class and mute/quarantine truth explicit alongside the marker",
                "keyboard/screen-reader reach preserved for the inline marker and its recent-attempts affordance",
                "provenance: an editor/notebook marker never reads an imported result as a live local one",
            ],
        ),
        seed_row(
            "cert:triage-panel",
            S::TriagePanel,
            ReviewableResult,
            ReviewableResult,
            &[FailureTriagePanel, EnvironmentMatrixCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "retry_lineage"],
            &[
                "the failure-triage panel keeps its assertion/diff summary, recent attempts, and classifier confidence explicit for a reviewer to read",
                "the environment-matrix card keeps its env/build/runtime deltas explicit rather than asserting a safe equivalence",
                "keyboard/screen-reader reach preserved for the triage panel and its evidence-gated rerun/debug controls",
                "provenance: a triage review keeps its retry lineage explicit rather than collapsing attempts into one",
            ],
        ),
        seed_row(
            "cert:cli-export",
            S::CliExport,
            ReviewableResult,
            ReviewableResult,
            &[SessionSummaryBar, QuarantineReviewSheet],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "quarantine_ownership"],
            &[
                "CLI/export reconstructs identity/freshness/origin/target/watch/lineage/quarantine truth from the same test identity",
                "the session-summary bar keeps its exact selection, running backlog, and retry counts explicit in the exported summary",
                "the quarantine-review sheet keeps its ownership, expiry, and release impact explicit in the exported packet",
                "provenance: an exported packet never emits raw assertion diffs, log bodies, or redacted evidence contents",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:imported-ci-view",
            S::ImportedCiView,
            TrustedLiveResult,
            ImportedOrStaleResult,
            &[InlineResultMarker, TestTreeRow],
            seed_certified_except(
                Ax::TestIntelligenceAndSuppressionProvenance,
                seed_narrowed(
                    Ax::TestIntelligenceAndSuppressionProvenance,
                    "the results shown were imported from an external CI run rather than produced by a live local rerun",
                    "The imported-CI view renders results replayed from an external CI run, so the TrustedLiveResult claim narrows to imported-or-stale-result instead of implying the marks reflect a current local rerun",
                    Trig::ResultOriginUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::TestIntelligenceAndSuppressionProvenance,
                TrustedLiveResult,
                ImportedOrStaleResult,
                "Imported from CI: the marks were replayed from an external run; the inline marker shows they are not a live local result until rerun",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the inline result marker keeps the imported-CI origin explicit and offers a live local rerun rather than implying currency",
                "the test-tree row keeps its identity class explicit while the result stays imported",
                "test-intelligence/suppression: TrustedLiveResult narrows to imported-or-stale-result (auto-narrowed)",
                "known compatibility note: imported-evidence behavior — an imported-CI mark never reads as a live local result",
            ],
        ),
        seed_row(
            "cert:watch-banner",
            S::WatchBanner,
            TrustedLiveResult,
            ReducedWatchResult,
            &[WatchModeBanner, SessionSummaryBar],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the watcher fell back to polling and can no longer guarantee live re-run fidelity",
                    "The watch-mode banner reports the watcher fell back from live re-run to polling, so the TrustedLiveResult claim narrows to reduced-watch-result instead of implying every save still triggers a live re-run",
                    Trig::WatchFidelityUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                TrustedLiveResult,
                ReducedWatchResult,
                "Watch degraded to polling: the banner names why live re-run fidelity dropped and preserves the last successful cycle rather than implying continuous coverage",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the watch-mode banner keeps the reduced-fidelity reason explicit and preserves the last successful cycle",
                "the session-summary bar keeps its running backlog explicit while watch fidelity is reduced",
                "degraded-state: TrustedLiveResult narrows to reduced-watch-result (auto-narrowed)",
                "known compatibility note: reduced-watch behavior — a polling watcher never reads as continuous live coverage",
            ],
        ),
        seed_row(
            "cert:status-bar-session-summary",
            S::StatusBarSessionSummary,
            TrustedLiveResult,
            WidenedSelectionResult,
            &[SessionSummaryBar, WatchModeBanner],
            seed_certified_except(
                Ax::TestIntelligenceAndSuppressionProvenance,
                seed_narrowed(
                    Ax::TestIntelligenceAndSuppressionProvenance,
                    "the rerun scope widened beyond the exact selection the user chose",
                    "The status-bar session summary resolves a rerun scope wider than the exact selection, so the TrustedLiveResult claim narrows to widened-selection-result instead of implying only the chosen tests will rerun",
                    Trig::RerunScopeWidened,
                ),
            ),
            Some(seed_narrow(
                Ax::TestIntelligenceAndSuppressionProvenance,
                TrustedLiveResult,
                WidenedSelectionResult,
                "Rerun scope widened: the session summary names the exact widened scope rather than implying only the picked tests rerun",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the session-summary bar keeps the widened rerun scope explicit rather than implying the exact selection",
                "the watch-mode banner keeps its watch state explicit while the selection is widened",
                "test-intelligence/suppression: TrustedLiveResult narrows to widened-selection-result (auto-narrowed)",
                "known compatibility note: widened-selection behavior — a widened rerun never reads as the exact chosen selection",
            ],
        ),
        seed_row(
            "cert:quarantine-review-sheet",
            S::QuarantineReviewSheet,
            TrustedLiveResult,
            RestrictedQuarantineResult,
            &[QuarantineReviewSheet, FailureTriagePanel],
            seed_certified_except(
                Ax::TestIntelligenceAndSuppressionProvenance,
                seed_narrowed(
                    Ax::TestIntelligenceAndSuppressionProvenance,
                    "the quarantine has expired or its release impact is policy-restricted from this surface",
                    "The quarantine-review sheet resolves an expired or policy-restricted quarantine, so the TrustedLiveResult claim narrows to restricted-quarantine-result instead of implying the mute still hides the test from release with full visibility",
                    Trig::QuarantineReleaseImpactHidden,
                ),
            ),
            Some(seed_narrow(
                Ax::TestIntelligenceAndSuppressionProvenance,
                TrustedLiveResult,
                RestrictedQuarantineResult,
                "Quarantine restricted: the sheet keeps the mute visible and names its expiry and release impact rather than implying unrestricted suppression",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the quarantine-review sheet keeps the mute always visible with its owner, expiry, and release impact explicit rather than silently hiding it",
                "the failure-triage panel keeps its recent attempts explicit while the quarantine stays restricted",
                "test-intelligence/suppression: TrustedLiveResult narrows to restricted-quarantine-result (auto-narrowed)",
                "known compatibility note: restricted-quarantine behavior — an expired or policy-restricted quarantine never reads as full release suppression",
            ],
        ),
    ]
}
