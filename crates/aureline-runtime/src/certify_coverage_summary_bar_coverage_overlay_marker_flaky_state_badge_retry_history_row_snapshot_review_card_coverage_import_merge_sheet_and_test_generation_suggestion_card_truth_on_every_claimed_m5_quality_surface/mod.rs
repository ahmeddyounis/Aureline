//! M05-1035 surface certification over the frozen M5 test-intelligence component matrix.
//!
//! Where the freeze matrix
//! ([`crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix`])
//! defines the seven reusable coverage-summary-bar, coverage-overlay-marker, flaky-state-badge,
//! retry-history-row, snapshot-review-card, coverage-import-merge-sheet, and
//! test-generation-suggestion-card components, the M05-1029..1032 primitive lanes narrow each
//! one, the M05-1033 consumer lane
//! ([`crate::add_shared_editor_gutter_test_tree_pr_review_cli_summary_support_export_and_imported_ci_consumers_so_test_intelligence_components_keep_scope_freshness_and_baseline_language_aligned_across_claimed_m5_profiles`])
//! proves they are reusable across the claimed editor / test-tree / PR-review / CLI / imported-CI
//! / support consumers, and the M05-1034 accessibility / auto-narrowing capstone
//! ([`crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_included_run_provenance_branch_coverage_flaky_windows_baseline_identity_or_sandbox_validation_is_partial_or_stale_across_claimed_m5_test_intelligence_components`])
//! certifies keyboard / screen-reader / CLI / export parity per family, this closing capstone
//! *certifies* that the shared coverage / flaky / snapshot / generated-test component truth holds
//! on every claimed M5 quality surface — and auto-narrows any surface that cannot sustain it.
//!
//! It is keyed on the claimed **surface** a user inspects test evidence from before trusting a
//! green bar, a flaky verdict, a snapshot baseline, or a generated test (the coverage-report view,
//! the editor / notebook gutter overlay, the flaky dashboard, the retry-history panel, the
//! snapshot-review pane, the coverage import / merge sheet, the AI test-generation review, and
//! CLI / export), not on component family or primitive lane. Each
//! [`TestIntelSurfaceCertificationRow`] certifies one surface across six truth axes — visual,
//! keyboard, screen-reader, CLI/export, degraded-state, and evidence-provenance / assumption
//! boundary — and either passes (green), auto-narrows its evidence claim to the weakest supported
//! ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind a full-truth claim
//! inherited from a healthier evidence lane.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**.
//! A surface that keeps a `VerifiedCurrentEvidence` / `ReviewableEvidence` claim while one of its
//! truth axes is not current — the included-run provenance is imported or stale, branch / condition
//! coverage is partial, the flaky evidence window is insufficient, the snapshot / merge baseline
//! identity is unverified, or the generated test's sandbox validation is unproven — is
//! over-claiming and blocks; a surface that discloses the reduction by narrowing its evidence claim
//! (with a bound reason and a frozen downgrade trigger) is honestly yellow. Coverage / flake /
//! snapshot / generated-test review never loses its evidence path: a narrowed surface always
//! preserves a durable path back to the raw report, the rerun / open-logs action, or the
//! diff-first rollback rather than collapsing a shard omission into a single percentage or bundling
//! generated changes into one opaque apply. The always-on CLI/export axis must always stay
//! certified, so support and automation can reconstruct the same included-run / line-versus-branch /
//! provenance / classifier-confidence / baseline-identity / raw-fallback / generated-assumption
//! truth from the same test identity the user saw.
//!
//! Every row cites exactly one canonical test-intelligence component proof bundle
//! ([`TEST_INTEL_CERT_CANONICAL_BUNDLE_REF`]) — the frozen component matrix release proof — rather
//! than cloning per-surface evidence. The packet is metadata-only: raw assertion diffs, coverage
//! report bodies, snapshot / golden artifact contents, generated-test source, and credentials never
//! cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-test-intelligence-component-certification.schema.json`](../../../../schemas/ui/m5-test-intelligence-component-certification.schema.json).
//! The contract doc is
//! [`docs/testing/m5_test_intelligence_component_certification_contract.md`](../../../../docs/testing/m5_test_intelligence_component_certification_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_shared_editor_gutter_test_tree_pr_review_cli_summary_support_export_and_imported_ci_consumers_so_test_intelligence_components_keep_scope_freshness_and_baseline_language_aligned_across_claimed_m5_profiles as consumers;
use crate::freeze_the_m5_coverage_summary_bar_coverage_overlay_marker_flaky_state_badge_retry_history_row_snapshot_review_card_coverage_import_merge_sheet_and_test_generation_suggestion_card_component_matrix as matrix;
use crate::implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_included_run_provenance_branch_coverage_flaky_windows_baseline_identity_or_sandbox_validation_is_partial_or_stale_across_claimed_m5_test_intelligence_components as a11y;
use a11y::M5IntelComponentClaim;
use matrix::{M5TestIntelligenceComponentFamily, M5TestIntelligenceDowngradeTrigger};

/// Schema version stamped on the M05-1035 certification packet.
pub const TEST_INTEL_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`TestIntelSurfaceCertificationPacket`].
pub const TEST_INTEL_CERT_RECORD_KIND: &str = "m5_test_intelligence_component_certification_packet";

/// Stable record-kind tag carried by each [`TestIntelSurfaceCertificationRow`].
pub const TEST_INTEL_CERT_ROW_RECORD_KIND: &str =
    "m5_test_intelligence_component_certification_row";

/// Repo-relative path of the boundary schema.
pub const TEST_INTEL_CERT_SCHEMA_REF: &str =
    "schemas/ui/m5-test-intelligence-component-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const TEST_INTEL_CERT_DOC_REF: &str =
    "docs/testing/m5_test_intelligence_component_certification_contract.md";

/// Repo-relative path of the frozen test-intelligence component matrix schema the certified
/// surfaces render.
pub const TEST_INTEL_CERT_MATRIX_REF: &str = matrix::M5_TEST_INTELLIGENCE_COMPONENT_SCHEMA_REF;

/// The one canonical test-intelligence component proof bundle every certified surface cites as its
/// first-resolved component truth. All eight surfaces point back to it rather than cloning
/// per-surface evidence.
pub const TEST_INTEL_CERT_CANONICAL_BUNDLE_REF: &str =
    matrix::M5_TEST_INTELLIGENCE_COMPONENT_ARTIFACT_REF;

/// The M05-1033 consumer-adoption export the certification builds on. Recorded as a supporting
/// evidence ref on every row.
pub const TEST_INTEL_CERT_CONSUMER_BUNDLE_REF: &str = consumers::TEST_INTEL_CONSUMER_ARTIFACT_REF;

/// The M05-1034 accessibility / auto-narrowing export whose keyboard / screen-reader / CLI /
/// export parity this capstone builds on. Recorded as a supporting evidence ref on every row.
pub const TEST_INTEL_CERT_A11Y_BUNDLE_REF: &str =
    a11y::TEST_INTEL_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const TEST_INTEL_CERT_ARTIFACT_REF: &str =
    "artifacts/release/m5-test-intelligence-component-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const TEST_INTEL_CERT_CSV_REF: &str =
    "artifacts/release/m5-test-intelligence-component-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const TEST_INTEL_CERT_REPORT_REF: &str =
    "artifacts/release/m5-test-intelligence-component-certification/report.md";

/// The eight claimed M5 quality surfaces this capstone certifies. Keyed on the surface a user
/// actually inspects coverage, flake, snapshot, or generated-test evidence from before trusting it,
/// not on the reusable component family it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestIntelligenceCertifiedSurface {
    /// The dedicated coverage-report / coverage-summary view.
    CoverageReportView,
    /// The editor / notebook gutter coverage-overlay surface.
    EditorGutterOverlay,
    /// The flaky-state dashboard surface.
    FlakyDashboard,
    /// The retry-history panel surface.
    RetryHistoryPanel,
    /// The snapshot / golden review pane surface.
    SnapshotReviewPane,
    /// The coverage import / merge sheet surface.
    CoverageImportMerge,
    /// The AI test-generation suggestion review surface.
    GeneratedTestReview,
    /// The CLI / export consumer surface.
    CliExport,
}

impl M5TestIntelligenceCertifiedSurface {
    /// Every certified surface, in declaration order.
    pub const ALL: [M5TestIntelligenceCertifiedSurface; 8] = [
        M5TestIntelligenceCertifiedSurface::CoverageReportView,
        M5TestIntelligenceCertifiedSurface::EditorGutterOverlay,
        M5TestIntelligenceCertifiedSurface::FlakyDashboard,
        M5TestIntelligenceCertifiedSurface::RetryHistoryPanel,
        M5TestIntelligenceCertifiedSurface::SnapshotReviewPane,
        M5TestIntelligenceCertifiedSurface::CoverageImportMerge,
        M5TestIntelligenceCertifiedSurface::GeneratedTestReview,
        M5TestIntelligenceCertifiedSurface::CliExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoverageReportView => "coverage_report_view",
            Self::EditorGutterOverlay => "editor_gutter_overlay",
            Self::FlakyDashboard => "flaky_dashboard",
            Self::RetryHistoryPanel => "retry_history_panel",
            Self::SnapshotReviewPane => "snapshot_review_pane",
            Self::CoverageImportMerge => "coverage_import_merge",
            Self::GeneratedTestReview => "generated_test_review",
            Self::CliExport => "cli_export",
        }
    }
}

/// The six truth axes a certified surface is scored on. These are exactly the parity dimensions the
/// spec requires verifying — visual, keyboard, screen-reader, CLI/export, degraded-state, and
/// evidence-provenance / assumption boundary. The CLI/export axis is always-on and must stay
/// certified for every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestIntelCertificationAxis {
    /// Visual parity: included-run scope, line-versus-branch metric, local/imported/cached/stale
    /// provenance, classifier confidence, baseline identity, raw/text fallback, and generated-test
    /// assumption boundaries are shown on the primary surface.
    Visual,
    /// Keyboard-reach parity: the same scope / metric / provenance / confidence / baseline /
    /// fallback / assumption truth and its rerun / open-logs / accept / apply controls are reachable
    /// without a pointer.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on color or a
    /// status glyph alone.
    ScreenReader,
    /// CLI / export parity (always-on): the certified surface state is reconstructable as text /
    /// JSON / Markdown for support and automation, from the same test identity.
    CliExport,
    /// Degraded-state parity: imported / stale / cached evidence or a sandbox validation that could
    /// not run honestly downgrades a `VerifiedCurrentEvidence` / `ReviewableEvidence` claim to a
    /// weaker evidence tier.
    DegradedState,
    /// Evidence-provenance / assumption-boundary parity: included-run scope, line-versus-branch
    /// coverage, local-versus-imported-versus-cached-versus-stale provenance, classifier confidence,
    /// artifact baseline identity, raw / text fallback, and generated-test assumption boundaries
    /// stay explicit before any trust, rerun, accept, or apply — never inheriting a healthier lane's
    /// evidence truth, never hiding a shard omission behind a single percentage, never reading one
    /// intermittent failure as confirmed flakiness, and never bundling generated changes into one
    /// opaque apply — and a narrowed surface never drops its raw-report / rerun / rollback continuity.
    EvidenceProvenanceAndAssumptionBoundary,
}

impl TestIntelCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [TestIntelCertificationAxis; 6] = [
        TestIntelCertificationAxis::Visual,
        TestIntelCertificationAxis::Keyboard,
        TestIntelCertificationAxis::ScreenReader,
        TestIntelCertificationAxis::CliExport,
        TestIntelCertificationAxis::DegradedState,
        TestIntelCertificationAxis::EvidenceProvenanceAndAssumptionBoundary,
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
            Self::EvidenceProvenanceAndAssumptionBoundary => {
                "evidence_provenance_and_assumption_boundary"
            }
        }
    }
}

/// The certification state of one truth axis on one surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestIntelAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim
    /// narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the surface hides it behind a full-truth claim inherited from
    /// a healthier surface.
    UndisclosedDrift,
}

impl TestIntelAxisCertificationState {
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
pub enum TestIntelSurfaceClaimStatus {
    /// Full standing: every axis certified, claimed evidence tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, CLI/export parity drops, evidence
    /// continuity is dropped, or the narrowing is inconsistent.
    Red,
}

impl TestIntelSurfaceClaimStatus {
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
pub struct TestIntelCertExportParity {
    /// The copy formats the surface offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The included-run / metric / provenance / confidence / baseline / fallback / assumption fields
    /// the surface preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl TestIntelCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a screenshot-only export.
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
pub struct TestIntelCertAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: TestIntelCertificationAxis,
    /// The certification state of the axis.
    pub state: TestIntelAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5TestIntelligenceDowngradeTrigger>,
}

impl TestIntelCertAxisOutcome {
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
            TestIntelAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            TestIntelAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            TestIntelAxisCertificationState::UndisclosedDrift => {
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
pub struct TestIntelClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: TestIntelCertificationAxis,
    /// The claim the surface would deliver at full parity.
    pub from_claim: M5IntelComponentClaim,
    /// The weakest supported claim the surface is certified down to.
    pub to_claim: M5IntelComponentClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
    /// True when the narrowed surface still preserves a durable path back to the raw report, the
    /// rerun / open-logs action, or the diff-first rollback rather than collapsing a shard omission
    /// into a single percentage or bundling generated changes into one opaque apply.
    pub preserves_evidence_continuity: bool,
}

/// One certified M5 quality surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestIntelSurfaceCertificationRow {
    /// Record kind; must equal [`TEST_INTEL_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`TEST_INTEL_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified surface.
    pub surface: M5TestIntelligenceCertifiedSurface,
    /// The evidence-claim ceiling the surface asserts.
    pub claimed_claim: M5IntelComponentClaim,
    /// The weakest supported claim the surface is certified down to. Must be no stronger than
    /// `claimed_claim`.
    pub certified_claim: M5IntelComponentClaim,
    /// The frozen component families this surface renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5TestIntelligenceComponentFamily>,
    /// One outcome per [`TestIntelCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<TestIntelCertAxisOutcome>,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<TestIntelClaimAutoNarrow>,
    /// True when this surface never drops its raw-report / rerun / rollback evidence continuity
    /// between an imported or stale reading and a fresh current-run signal.
    pub evidence_continuity_preserved: bool,
    /// The one canonical test-intelligence proof bundle this surface cites. Must equal
    /// [`TEST_INTEL_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: TestIntelSurfaceClaimStatus,
    /// The copy / export parity of the certified surface state.
    pub export_parity: TestIntelCertExportParity,
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

impl TestIntelSurfaceCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: TestIntelCertificationAxis) -> Option<&TestIntelCertAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<TestIntelCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && TestIntelCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(TestIntelCertAxisOutcome::well_formed)
    }

    /// True when the surface narrows its evidence claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<TestIntelCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == TestIntelAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Whether a narrowed surface preserves its raw-report / rerun / rollback evidence continuity
    /// rather than dropping it. A non-narrowed surface trivially preserves continuity; a narrowed
    /// one must say so.
    pub fn preserves_evidence_continuity(&self) -> bool {
        match &self.claim_auto_narrow {
            Some(narrow) => {
                self.evidence_continuity_preserved && narrow.preserves_evidence_continuity
            }
            None => self.evidence_continuity_preserved,
        }
    }

    /// Derives the surface verdict from its axes and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, CLI/export parity must
    /// always certify, coverage / flake / snapshot / generated-test review must never drop evidence
    /// continuity, and the narrowing must be consistent.
    pub fn derive_status(&self) -> TestIntelSurfaceClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != TEST_INTEL_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
            || !self.preserves_evidence_continuity()
        {
            return TestIntelSurfaceClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return TestIntelSurfaceClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(TestIntelCertificationAxis::CliExport) {
            Some(o) if o.state == TestIntelAxisCertificationState::Certified => {}
            _ => return TestIntelSurfaceClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == TestIntelAxisCertificationState::UndisclosedDrift)
        {
            return TestIntelSurfaceClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return TestIntelSurfaceClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return TestIntelSurfaceClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                    || !narrow.preserves_evidence_continuity
                {
                    return TestIntelSurfaceClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return TestIntelSurfaceClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim
        // inheriting a healthier surface's truth.
        if !narrowed.is_empty() {
            return TestIntelSurfaceClaimStatus::Red;
        }

        TestIntelSurfaceClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == TEST_INTEL_CERT_ROW_RECORD_KIND
            && self.schema_version == TEST_INTEL_CERT_SCHEMA_VERSION
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
narrowed_axes={narrowed} evidence_continuity_preserved={preserved}",
            surface = self.surface.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
            preserved = self.evidence_continuity_preserved,
        )
    }
}

/// Rolled-up summary of an M05-1035 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestIntelSurfaceCertificationSummary {
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
    pub all_evidence_continuity_preserved: bool,
    pub narrowed_surface_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`TestIntelSurfaceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestIntelSurfaceCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<TestIntelSurfaceCertificationRow>,
}

/// Checked-in M05-1035 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestIntelSurfaceCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<TestIntelSurfaceCertificationRow>,
    pub summary: TestIntelSurfaceCertificationSummary,
}

impl TestIntelSurfaceCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: TestIntelSurfaceCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: TEST_INTEL_CERT_SCHEMA_VERSION,
            record_kind: TEST_INTEL_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: TestIntelSurfaceCertificationSummary {
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
                all_evidence_continuity_preserved: false,
                narrowed_surface_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Surfaces represented by some row in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5TestIntelligenceCertifiedSurface> {
        self.rows.iter().map(|r| r.surface).collect()
    }

    /// Component families rendered by some certified surface in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5TestIntelligenceComponentFamily> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified surface appears exactly once.
    pub fn all_surfaces_present(&self) -> bool {
        let surfaces = self.represented_surfaces();
        surfaces.len() == self.rows.len()
            && M5TestIntelligenceCertifiedSurface::ALL
                .iter()
                .all(|s| surfaces.contains(s))
    }

    /// Whether every frozen component family is certified on at least one surface — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5TestIntelligenceComponentFamily::ALL
            .iter()
            .all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(TestIntelCertificationAxis::CliExport)
                .is_some_and(|o| o.state == TestIntelAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> TestIntelSurfaceCertificationSummary {
        let surfaces = self.represented_surfaces();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == TestIntelSurfaceClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == TestIntelSurfaceClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == TestIntelSurfaceClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(TestIntelSurfaceCertificationRow::status_is_fresh);
        let all_surfaces = self.all_surfaces_present();
        let all_families = self.all_families_covered();
        let all_preserved = self
            .rows
            .iter()
            .all(TestIntelSurfaceCertificationRow::preserves_evidence_continuity);

        TestIntelSurfaceCertificationSummary {
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
                .all(|r| r.canonical_bundle_ref == TEST_INTEL_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(TestIntelSurfaceCertificationRow::covers_all_axes),
            all_evidence_continuity_preserved: all_preserved,
            narrowed_surface_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable
                && all_fresh
                && all_surfaces
                && all_families
                && all_preserved,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<TestIntelCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != TEST_INTEL_CERT_SCHEMA_VERSION {
            violations.push(TestIntelCertificationViolation::SchemaVersion {
                expected: TEST_INTEL_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != TEST_INTEL_CERT_RECORD_KIND {
            violations.push(TestIntelCertificationViolation::RecordKind {
                expected: TEST_INTEL_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(TestIntelCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != TEST_INTEL_CERT_CANONICAL_BUNDLE_REF {
            violations.push(TestIntelCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(TestIntelCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(TestIntelCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(TestIntelCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(TestIntelCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != TEST_INTEL_CERT_CANONICAL_BUNDLE_REF {
                violations.push(TestIntelCertificationViolation::RowMissingCanonicalBundle {
                    id: row.row_id.clone(),
                });
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(TestIntelCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(TestIntelCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Coverage / flake / snapshot / generated-test review must never drop evidence continuity.
            if !row.preserves_evidence_continuity() {
                violations.push(TestIntelCertificationViolation::EvidenceContinuityDropped {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    TestIntelCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(TestIntelCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) surface must not ship in a clean packet.
            if row.derived_status == TestIntelSurfaceClaimStatus::Red {
                violations.push(TestIntelCertificationViolation::SurfaceBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed surface must be certified exactly once.
        if !self.all_surfaces_present() {
            violations.push(TestIntelCertificationViolation::SurfaceCoverageIncomplete);
        }

        // Every frozen component family must be certified on some surface.
        if !self.all_families_covered() {
            violations.push(TestIntelCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(TestIntelCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(TestIntelCertificationViolation::RawTestMaterialInExport);
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
            "row_id,surface,claimed_claim,certified_claim,status,narrowed_axes,binding_axis,evidence_continuity_preserved\n",
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
                preserved = row.evidence_continuity_preserved,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Test-Intelligence Component Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.surface_count,
            M5TestIntelligenceCertifiedSurface::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Evidence continuity preserved on every surface: {}\n",
            self.summary.all_evidence_continuity_preserved
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
pub fn current_m5_test_intelligence_component_certification_export(
) -> Result<TestIntelSurfaceCertificationPacket, TestIntelCertificationArtifactError> {
    let packet: TestIntelSurfaceCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-test-intelligence-component-certification/support_export.json"
    )))
    .map_err(TestIntelCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TestIntelCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum TestIntelCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TestIntelCertificationViolation>),
}

impl fmt::Display for TestIntelCertificationArtifactError {
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

impl Error for TestIntelCertificationArtifactError {}

/// Validation failure for M05-1035 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestIntelCertificationViolation {
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
    EvidenceContinuityDropped { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    SurfaceBlocked { id: String },
    SurfaceCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawTestMaterialInExport,
}

impl fmt::Display for TestIntelCertificationViolation {
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
                    "packet does not cite the canonical test-intelligence component proof bundle"
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
                    "row {id} does not cite the one canonical test-intelligence component proof bundle"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::EvidenceContinuityDropped { id } => {
                write!(
                    f,
                    "row {id} drops raw-report / rerun / rollback evidence continuity (a narrowed surface must preserve a durable path back to the raw report, the rerun / open-logs action, or the diff-first rollback rather than collapsing a shard omission into a single percentage or bundling generated changes into one opaque apply)"
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
CLI/export parity dropped, evidence continuity was dropped, or the narrowing is inconsistent"
                )
            }
            Self::SurfaceCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 quality surface is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen test-intelligence component family is certified on some surface"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawTestMaterialInExport => {
                write!(f, "export contains raw test material")
            }
        }
    }
}

impl Error for TestIntelCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&TestIntelCertAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != TestIntelAxisCertificationState::Certified,
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
            | "unvalidated"
            | "unconfirmed"
            | "partial"
            | "generated"
            | "flaky"
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

/// Builds the canonical, checked-in M05-1035 certification packet. Certifies all eight claimed M5
/// quality surfaces: three deliver their claim (green) and five auto-narrow a not-current truth axis
/// to a weaker evidence ceiling (yellow). No surface hides drift (red), and no surface drops its
/// raw-report / rerun / rollback evidence continuity.
pub fn seeded_m5_test_intelligence_component_certification_packet(
) -> TestIntelSurfaceCertificationPacket {
    TestIntelSurfaceCertificationPacket::new(TestIntelSurfaceCertificationPacketInput {
        packet_id: "m5-test-intelligence-component-certification:stable:0001".to_owned(),
        as_of: "2026-07-09T00:00:00Z".to_owned(),
        matrix_ref: TEST_INTEL_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: TEST_INTEL_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:test-intelligence-component-certification:{id}"),
        TEST_INTEL_CERT_CONSUMER_BUNDLE_REF.to_owned(),
        TEST_INTEL_CERT_A11Y_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> TestIntelCertExportParity {
    TestIntelCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn seed_certified_note(axis: TestIntelCertificationAxis) -> &'static str {
    match axis {
        TestIntelCertificationAxis::Visual => {
            "included-run scope, line-versus-branch metric, local/imported/cached/stale provenance, classifier confidence, baseline identity, raw/text fallback, and generated-test assumption boundaries shown on-surface"
        }
        TestIntelCertificationAxis::Keyboard => {
            "the same scope/metric/provenance/confidence/baseline/fallback/assumption truth and its rerun/open-logs/accept/apply controls are keyboard-reachable"
        }
        TestIntelCertificationAxis::ScreenReader => {
            "the same truth is announced non-visually, never color/glyph-only"
        }
        TestIntelCertificationAxis::CliExport => {
            "surface state exports as text / JSON / Markdown for support from the same test identity"
        }
        TestIntelCertificationAxis::DegradedState => {
            "imported or stale or cached evidence, or a sandbox validation that could not run, honestly downgrades the VerifiedCurrentEvidence/ReviewableEvidence claim"
        }
        TestIntelCertificationAxis::EvidenceProvenanceAndAssumptionBoundary => {
            "included-run scope, line-versus-branch coverage, provenance class, classifier confidence, baseline identity, raw/text fallback, and generated-test assumption boundaries stay explicit before any trust, rerun, accept, or apply; a shard omission never hides behind a single percentage and generated changes never bundle into one opaque apply"
        }
    }
}

fn seed_certified(axis: TestIntelCertificationAxis) -> TestIntelCertAxisOutcome {
    TestIntelCertAxisOutcome {
        axis,
        state: TestIntelAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: TestIntelCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5TestIntelligenceDowngradeTrigger,
) -> TestIntelCertAxisOutcome {
    TestIntelCertAxisOutcome {
        axis,
        state: TestIntelAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<TestIntelCertAxisOutcome> {
    TestIntelCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: TestIntelCertificationAxis,
    outcome: TestIntelCertAxisOutcome,
) -> Vec<TestIntelCertAxisOutcome> {
    TestIntelCertificationAxis::ALL
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
    surface: M5TestIntelligenceCertifiedSurface,
    claimed_claim: M5IntelComponentClaim,
    certified_claim: M5IntelComponentClaim,
    consumed_families: &[M5TestIntelligenceComponentFamily],
    axis_outcomes: Vec<TestIntelCertAxisOutcome>,
    claim_auto_narrow: Option<TestIntelClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> TestIntelSurfaceCertificationRow {
    let mut row = TestIntelSurfaceCertificationRow {
        record_kind: TEST_INTEL_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: TEST_INTEL_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        surface,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        claim_auto_narrow,
        evidence_continuity_preserved: true,
        canonical_bundle_ref: TEST_INTEL_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: TestIntelSurfaceClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            TEST_INTEL_CERT_MATRIX_REF.to_owned(),
            TEST_INTEL_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-09T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: TestIntelCertificationAxis,
    from_claim: M5IntelComponentClaim,
    to_claim: M5IntelComponentClaim,
    label: &str,
) -> TestIntelClaimAutoNarrow {
    TestIntelClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
        preserves_evidence_continuity: true,
    }
}

fn seeded_rows() -> Vec<TestIntelSurfaceCertificationRow> {
    use M5IntelComponentClaim::*;
    use M5TestIntelligenceCertifiedSurface as S;
    use M5TestIntelligenceComponentFamily::*;
    use M5TestIntelligenceDowngradeTrigger as Trig;
    use TestIntelCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:editor-gutter-overlay",
            S::EditorGutterOverlay,
            VerifiedCurrentEvidence,
            VerifiedCurrentEvidence,
            &[CoverageOverlayMarker, CoverageSummaryBar],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "line_versus_branch"],
            &[
                "the coverage-overlay marker keeps its gutter state and changed-line emphasis explicit while distinguishing line from branch coverage",
                "the coverage-summary bar keeps its included-run scope and line/branch metric explicit rather than collapsing shards into one percentage",
                "keyboard/screen-reader reach preserved for the overlay marker and the summary bar's open-report affordance",
                "provenance: a live gutter overlay never leaves its included-run scope or line-versus-branch dimension implicit",
            ],
        ),
        seed_row(
            "cert:retry-history-panel",
            S::RetryHistoryPanel,
            ReviewableEvidence,
            ReviewableEvidence,
            &[RetryHistoryRow, FlakyStateBadge],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "classifier_confidence"],
            &[
                "the retry-history row keeps its ordered attempt outcomes, env/build/runtime deltas, and rerun scope explicit for a reviewer",
                "the flaky-state badge keeps its classification and classifier confidence explicit rather than reading one intermittent failure as confirmed flakiness",
                "keyboard/screen-reader reach preserved for the retry panel and its rerun/open-logs controls",
                "provenance: a retry review keeps its ordered attempts and classifier confidence explicit before any confirmed-flaky verdict",
            ],
        ),
        seed_row(
            "cert:cli-export",
            S::CliExport,
            ReviewableEvidence,
            ReviewableEvidence,
            &[TestGenerationSuggestionCard, SnapshotReviewCard],
            seed_all_certified(),
            None,
            &["surface", "claimed_claim", "certified_claim", "status", "generated_assumptions"],
            &[
                "CLI/export reconstructs included-run/metric/provenance/confidence/baseline/fallback/assumption truth from the same test identity",
                "the test-generation suggestion card keeps its assumption summary and helper/fixture/snapshot separation explicit in the exported packet rather than one opaque apply",
                "the snapshot-review card keeps its baseline identity, artifact count, and raw/text fallback explicit in the exported packet",
                "provenance: an exported packet never emits raw assertion diffs, coverage report bodies, snapshot artifact contents, or generated-test source",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:coverage-report-view",
            S::CoverageReportView,
            VerifiedCurrentEvidence,
            PartialConditionEvidence,
            &[CoverageSummaryBar, CoverageImportMergeSheet],
            seed_certified_except(
                Ax::EvidenceProvenanceAndAssumptionBoundary,
                seed_narrowed(
                    Ax::EvidenceProvenanceAndAssumptionBoundary,
                    "the report resolves line coverage only; branch / condition coverage is partial for the included runs",
                    "The coverage-report view resolves a summary whose branch / condition coverage is partial, so the VerifiedCurrentEvidence claim narrows to partial-condition-evidence instead of implying the percentage reflects full line-and-branch coverage",
                    Trig::LineVersusBranchUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::EvidenceProvenanceAndAssumptionBoundary,
                VerifiedCurrentEvidence,
                PartialConditionEvidence,
                "Branch coverage partial: the report names that only line coverage is complete and keeps the open-report path to the per-file branch detail rather than implying full coverage",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the coverage-summary bar keeps its included-run scope explicit and distinguishes line from branch coverage rather than implying both",
                "the coverage-import / merge sheet keeps its merged shard scope explicit while branch detail stays partial",
                "evidence-provenance: VerifiedCurrentEvidence narrows to partial-condition-evidence (auto-narrowed)",
                "known compatibility note: partial-branch behavior — a line-only percentage never reads as full line-and-branch coverage",
            ],
        ),
        seed_row(
            "cert:coverage-import-merge",
            S::CoverageImportMerge,
            VerifiedCurrentEvidence,
            ImportedOrStaleEvidence,
            &[CoverageImportMergeSheet, CoverageOverlayMarker],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the merged coverage was imported from an external CI artifact rather than produced by a fresh local run",
                    "The coverage import / merge sheet renders coverage merged from an imported CI artifact, so the VerifiedCurrentEvidence claim narrows to imported-or-stale-evidence instead of implying the merged number reflects a fresh current-run result",
                    Trig::ProvenanceClassUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                VerifiedCurrentEvidence,
                ImportedOrStaleEvidence,
                "Imported coverage: the sheet names the external CI source and its included-shard scope and keeps the raw-report path rather than implying a fresh local run",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the coverage-import / merge sheet keeps its imported source and included-shard scope explicit and discloses any omitted shard rather than hiding it behind one percentage",
                "the coverage-overlay marker keeps its gutter state explicit while the merged coverage stays imported",
                "degraded-state: VerifiedCurrentEvidence narrows to imported-or-stale-evidence (auto-narrowed)",
                "known compatibility note: imported-coverage behavior — an imported merge never reads as a fresh current-run result",
            ],
        ),
        seed_row(
            "cert:flaky-dashboard",
            S::FlakyDashboard,
            ReviewableEvidence,
            UnconfirmedFlakyEvidence,
            &[FlakyStateBadge, RetryHistoryRow],
            seed_certified_except(
                Ax::EvidenceProvenanceAndAssumptionBoundary,
                seed_narrowed(
                    Ax::EvidenceProvenanceAndAssumptionBoundary,
                    "the flaky evidence window is insufficient — a single intermittent failure has not been reproduced across enough runs to confirm flakiness",
                    "The flaky dashboard resolves a badge whose flaky evidence window is insufficient, so the ReviewableEvidence claim narrows to unconfirmed-flaky-evidence instead of reading one intermittent failure as confirmed flakiness",
                    Trig::FlakyConfidenceOverstated,
                ),
            ),
            Some(seed_narrow(
                Ax::EvidenceProvenanceAndAssumptionBoundary,
                ReviewableEvidence,
                UnconfirmedFlakyEvidence,
                "Flakiness unconfirmed: the badge names the insufficient reproduction window and keeps the retry-history path to the ordered attempts rather than implying confirmed flakiness",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the flaky-state badge keeps its classifier confidence and reproduction window explicit rather than asserting confirmed flakiness",
                "the retry-history row keeps its ordered attempt outcomes explicit while the flaky verdict stays unconfirmed",
                "evidence-provenance: ReviewableEvidence narrows to unconfirmed-flaky-evidence (auto-narrowed)",
                "known compatibility note: unconfirmed-flaky behavior — one intermittent failure never reads as confirmed flakiness",
            ],
        ),
        seed_row(
            "cert:snapshot-review-pane",
            S::SnapshotReviewPane,
            ReviewableEvidence,
            UnverifiedBaselineEvidence,
            &[SnapshotReviewCard, CoverageOverlayMarker],
            seed_certified_except(
                Ax::EvidenceProvenanceAndAssumptionBoundary,
                seed_narrowed(
                    Ax::EvidenceProvenanceAndAssumptionBoundary,
                    "the snapshot / golden baseline identity or shard scope is unverified — the artifact the diff compares against cannot be confirmed as the intended baseline",
                    "The snapshot-review pane resolves a card whose baseline identity is unverified, so the ReviewableEvidence claim narrows to unverified-baseline-evidence instead of implying a trusted baseline behind a blind Accept-all",
                    Trig::SnapshotBaselineUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::EvidenceProvenanceAndAssumptionBoundary,
                ReviewableEvidence,
                UnverifiedBaselineEvidence,
                "Baseline unverified: the card names the unconfirmed baseline identity and artifact count and keeps the raw/text fallback path rather than collapsing to a blind Accept-all",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the snapshot-review card keeps its baseline identity, artifact count, and scope explicit rather than collapsing acceptance to a blind Accept-all",
                "the coverage-overlay marker keeps its changed-line emphasis explicit while the snapshot baseline stays unverified",
                "evidence-provenance: ReviewableEvidence narrows to unverified-baseline-evidence (auto-narrowed)",
                "known compatibility note: unverified-baseline behavior — an unconfirmed baseline never reads as a trusted baseline",
            ],
        ),
        seed_row(
            "cert:generated-test-review",
            S::GeneratedTestReview,
            ReviewableEvidence,
            UnvalidatedGeneratedEvidence,
            &[TestGenerationSuggestionCard, FlakyStateBadge],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the generated test's sandbox validation could not run, so its assumptions are unproven",
                    "The generated-test review resolves a suggestion whose sandbox validation is unproven, so the ReviewableEvidence claim narrows to unvalidated-generated-evidence instead of implying a validated generated test behind one opaque apply",
                    Trig::GeneratedAssumptionHidden,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableEvidence,
                UnvalidatedGeneratedEvidence,
                "Generated test unvalidated: the card names its unproven sandbox validation and separates assertion, helper/fixture, and snapshot churn with a diff-first rollback rather than one opaque apply",
            )),
            &["surface", "claimed_claim", "certified_claim", "status", "binding_axis"],
            &[
                "the test-generation suggestion card keeps its assumption summary explicit and separates assertion, helper/fixture, and snapshot changes into distinct review classes",
                "the flaky-state badge keeps its classification explicit while the generated test stays unvalidated",
                "degraded-state: ReviewableEvidence narrows to unvalidated-generated-evidence (auto-narrowed)",
                "known compatibility note: unvalidated-generated behavior — an unproven sandbox validation never reads as a validated generated test, and generated changes never bundle into one opaque apply",
            ],
        ),
    ]
}
