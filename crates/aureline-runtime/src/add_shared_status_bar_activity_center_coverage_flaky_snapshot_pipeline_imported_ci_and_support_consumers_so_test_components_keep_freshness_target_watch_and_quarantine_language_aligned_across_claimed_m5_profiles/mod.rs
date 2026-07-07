//! Shared status-bar, activity-center, coverage, flaky, snapshot, pipeline,
//! imported-CI, and support consumers for the frozen M5 test-explorer / watch /
//! triage components.
//!
//! This module is the M05-913 consumer-adoption lane over the frozen M5
//! test-explorer / watch / triage component matrix
//! ([`crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix`])
//! and the 909-912 primitive resolvers (test-tree row, inline result marker,
//! session-summary bar / watch-mode banner, and failure-triage panel /
//! quarantine-review sheet / environment-matrix card). Where the freeze matrix
//! defines the seven reusable component families and 909-912 resolve their
//! per-surface truth, this lane proves those families are reusable *primitives*
//! rather than one test-tree pane by adopting them across the claimed M5 test
//! consumer surfaces beyond the primary test tree:
//!
//! 1. the status-bar test summary and the durable activity center (day-to-day
//!    editor),
//! 2. coverage, flaky, and snapshot intelligence (quality intelligence),
//! 3. pipeline overlays and imported-CI views (pipeline / imported), and
//! 4. support / export packets (support).
//!
//! Each [`TestConsumerRow`] points back to exactly one canonical component
//! family (its primitive schema + release-proof packet) instead of cloning
//! surface-local test vocabulary, and every consumer keeps the identical
//! controlled label families for result freshness, target class, watch state,
//! quarantine semantics, and imported-versus-live result origin, plus one shared
//! state lexicon so `failed`, `rerun failed`, and `quarantined` mean the same
//! thing on every surface.
//!
//! When a consumer's evidence is weaker than a full local-live claim — the
//! result was imported, the target compatibility drifted, watch fidelity
//! degraded, or quarantine visibility is restricted by scope / policy — the row
//! **auto-narrows** its visible claim language and discloses the reduction with
//! an auto-narrow banner naming the reason and a recovery hint, rather than
//! renaming or dropping the governed state or letting an imported result read as
//! a local rerun.
//!
//! The packet is metadata-only: raw runner output, assertion diffs, stack
//! frames, credentials, and provider payloads never cross this boundary; the
//! packet carries only typed class tokens, opaque summary / evidence refs,
//! booleans, and redacted labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-test-component-consumer.schema.json`](../../../../schemas/ui/m5-test-component-consumer.schema.json).
//! The contract doc is
//! [`docs/testing/m5_test_component_consumer_contract.md`](../../../../docs/testing/m5_test_component_consumer_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix::{
    M5TestExplorerWatchTriageComponentFamily, M5TestResultOrigin,
    M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_DOC_REF,
    M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF,
};
use crate::implement_failure_triage_panels_quarantine_review_sheets_and_environment_matrix_cards_with_assertion_diff_summaries_recent_attempts_env_build_runtime_deltas_owner_expiry_release_impact_and_rerun_debug_review_parity_across_claimed_m5_quality_surfaces::{
    M5_QUALITY_TRIAGE_STATUS_ARTIFACT_REF, M5_QUALITY_TRIAGE_STATUS_ENVIRONMENT_SCHEMA_REF,
    M5_QUALITY_TRIAGE_STATUS_QUARANTINE_SCHEMA_REF, M5_QUALITY_TRIAGE_STATUS_TRIAGE_SCHEMA_REF,
};
use crate::implement_inline_result_markers_with_live_versus_imported_versus_stale_stability_chips_open_recent_attempts_and_target_env_shorthand_across_claimed_m5_editors_and_notebook_views::{
    M5_INLINE_RESULT_MARKER_ARTIFACT_REF, M5_INLINE_RESULT_MARKER_SCHEMA_REF,
};
use crate::implement_session_summary_bars_and_watch_mode_banners_with_exact_selection_running_backlog_retry_counts_live_reduced_polling_unavailable_state_last_successful_cycle_and_recover_pause_truth_across_claimed_m5_test_lanes::{
    M5_SESSION_WATCH_STATUS_ARTIFACT_REF, M5_SESSION_WATCH_STATUS_SESSION_SCHEMA_REF,
    M5_SESSION_WATCH_STATUS_WATCH_SCHEMA_REF,
};
use crate::implement_test_tree_rows_with_suite_template_case_notebook_imported_result_distinction_parameterized_counts_freshness_target_chip_and_mute_quarantine_truth_across_claimed_m5_test_surfaces::{
    M5_TEST_TREE_ROW_ARTIFACT_REF, M5_TEST_TREE_ROW_SCHEMA_REF,
};

/// Schema version stamped on the M05-913 consumer packet.
pub const TEST_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`TestConsumerPacket`].
pub const TEST_CONSUMER_RECORD_KIND: &str = "m5_test_component_consumer_packet";

/// Stable record-kind tag carried by each [`TestConsumerRow`].
pub const TEST_CONSUMER_ROW_RECORD_KIND: &str = "m5_test_component_consumer_row";

/// Repo-relative path of the boundary schema.
pub const TEST_CONSUMER_SCHEMA_REF: &str = "schemas/ui/m5-test-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const TEST_CONSUMER_DOC_REF: &str = "docs/testing/m5_test_component_consumer_contract.md";

/// Repo-relative path of the frozen test-explorer / watch / triage component
/// matrix these consumers adopt.
pub const TEST_CONSUMER_MATRIX_REF: &str = M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen-matrix contract doc these consumers adopt.
pub const TEST_CONSUMER_MATRIX_DOC_REF: &str = M5_TEST_EXPLORER_WATCH_TRIAGE_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const TEST_CONSUMER_FIXTURE_DIR: &str = "fixtures/ui/m5-test-component-consumers";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const TEST_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-test-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const TEST_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-test-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const TEST_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-test-component-consumer-proof/report.md";

/// The controlled label families a consumer must preserve identically across
/// every surface. These are the track-invariant truth pillars of the
/// test-explorer / watch / triage components: result freshness, target class,
/// watch state, quarantine semantics, and imported-versus-live result origin.
/// The union of every row's `preserved_label_families` must cover this set.
pub const REQUIRED_LABEL_FAMILIES: [&str; 5] = [
    "result_freshness",
    "target_class",
    "watch_state",
    "quarantine_semantics",
    "result_origin",
];

/// The single shared state lexicon every consumer keeps verbatim so a red mark,
/// a re-run of only the failing selection, and a suppressed test read the same on
/// every surface. This is the AC anchor that imported-CI and local-live consumers
/// stop diverging on what `failed`, `rerun failed`, or `quarantined` means.
pub const SHARED_STATE_LEXICON: [&str; 3] = ["failed", "rerun_failed", "quarantined"];

/// The canonical primitive schema that defines a component family's contract.
/// Consumers must point at this schema instead of inventing a surface-local one.
pub const fn family_canonical_schema_ref(
    family: M5TestExplorerWatchTriageComponentFamily,
) -> &'static str {
    use M5TestExplorerWatchTriageComponentFamily::*;
    match family {
        TestTreeRow => M5_TEST_TREE_ROW_SCHEMA_REF,
        InlineResultMarker => M5_INLINE_RESULT_MARKER_SCHEMA_REF,
        SessionSummaryBar => M5_SESSION_WATCH_STATUS_SESSION_SCHEMA_REF,
        WatchModeBanner => M5_SESSION_WATCH_STATUS_WATCH_SCHEMA_REF,
        FailureTriagePanel => M5_QUALITY_TRIAGE_STATUS_TRIAGE_SCHEMA_REF,
        QuarantineReviewSheet => M5_QUALITY_TRIAGE_STATUS_QUARANTINE_SCHEMA_REF,
        EnvironmentMatrixCard => M5_QUALITY_TRIAGE_STATUS_ENVIRONMENT_SCHEMA_REF,
    }
}

/// The canonical release-proof packet that defines a component family's first
/// resolved truth. Consumers point back to this packet rather than cloning it.
pub const fn family_canonical_packet_ref(
    family: M5TestExplorerWatchTriageComponentFamily,
) -> &'static str {
    use M5TestExplorerWatchTriageComponentFamily::*;
    match family {
        TestTreeRow => M5_TEST_TREE_ROW_ARTIFACT_REF,
        InlineResultMarker => M5_INLINE_RESULT_MARKER_ARTIFACT_REF,
        // The session-summary bar and watch-mode banner are two halves of the
        // same 911 session/watch status primitive.
        SessionSummaryBar | WatchModeBanner => M5_SESSION_WATCH_STATUS_ARTIFACT_REF,
        // Failure-triage, quarantine-review, and environment-matrix are the three
        // halves of the same 912 quality-triage status primitive.
        FailureTriagePanel | QuarantineReviewSheet | EnvironmentMatrixCard => {
            M5_QUALITY_TRIAGE_STATUS_ARTIFACT_REF
        }
    }
}

/// A short human-readable label for a component family, for the Markdown report.
pub const fn family_label(family: M5TestExplorerWatchTriageComponentFamily) -> &'static str {
    use M5TestExplorerWatchTriageComponentFamily::*;
    match family {
        TestTreeRow => "Test-tree row",
        InlineResultMarker => "Inline result marker",
        SessionSummaryBar => "Session-summary bar",
        WatchModeBanner => "Watch-mode banner",
        FailureTriagePanel => "Failure-triage panel",
        QuarantineReviewSheet => "Quarantine-review sheet",
        EnvironmentMatrixCard => "Environment-matrix card",
    }
}

/// True when a result origin is not a local-live run and therefore may never
/// claim full local-live certainty.
pub const fn origin_is_imported(origin: M5TestResultOrigin) -> bool {
    matches!(
        origin,
        M5TestResultOrigin::ImportedCi
            | M5TestResultOrigin::ImportedTeammate
            | M5TestResultOrigin::ReplayedSnapshot
    )
}

/// The four claimed M5 test consumer classes that must each adopt at least one
/// canonical component family beyond the primary test tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedTestConsumerClass {
    /// The day-to-day editor surfaces: status-bar summary + activity center.
    DayToDayEditor,
    /// Coverage, flaky, and snapshot intelligence surfaces.
    QualityIntelligence,
    /// Pipeline overlays and imported-CI views.
    PipelineImported,
    /// Support / export packets.
    SupportExport,
}

impl SharedTestConsumerClass {
    /// Every consumer class that must be present for cross-surface reuse.
    pub const ALL: [SharedTestConsumerClass; 4] = [
        SharedTestConsumerClass::DayToDayEditor,
        SharedTestConsumerClass::QualityIntelligence,
        SharedTestConsumerClass::PipelineImported,
        SharedTestConsumerClass::SupportExport,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DayToDayEditor => "day_to_day_editor",
            Self::QualityIntelligence => "quality_intelligence",
            Self::PipelineImported => "pipeline_imported",
            Self::SupportExport => "support_export",
        }
    }
}

/// The concrete M5 test consumer surface a component is embedded in. Each surface
/// belongs to exactly one [`SharedTestConsumerClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SharedTestConsumerSurface {
    /// The status-bar test summary.
    StatusBarSummary,
    /// The durable activity center / run history.
    ActivityCenter,
    /// The coverage-intelligence overlay.
    CoverageIntelligence,
    /// The flaky-test intelligence view.
    FlakyIntelligence,
    /// The snapshot / golden review.
    SnapshotReview,
    /// A CI / pipeline overlay.
    PipelineOverlay,
    /// An imported-CI results view.
    ImportedCiView,
    /// A support / export packet.
    SupportPacket,
}

impl SharedTestConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [SharedTestConsumerSurface; 8] = [
        SharedTestConsumerSurface::StatusBarSummary,
        SharedTestConsumerSurface::ActivityCenter,
        SharedTestConsumerSurface::CoverageIntelligence,
        SharedTestConsumerSurface::FlakyIntelligence,
        SharedTestConsumerSurface::SnapshotReview,
        SharedTestConsumerSurface::PipelineOverlay,
        SharedTestConsumerSurface::ImportedCiView,
        SharedTestConsumerSurface::SupportPacket,
    ];

    /// The consumer class this surface belongs to.
    pub const fn consumer_class(self) -> SharedTestConsumerClass {
        match self {
            Self::StatusBarSummary | Self::ActivityCenter => {
                SharedTestConsumerClass::DayToDayEditor
            }
            Self::CoverageIntelligence | Self::FlakyIntelligence | Self::SnapshotReview => {
                SharedTestConsumerClass::QualityIntelligence
            }
            Self::PipelineOverlay | Self::ImportedCiView => {
                SharedTestConsumerClass::PipelineImported
            }
            Self::SupportPacket => SharedTestConsumerClass::SupportExport,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatusBarSummary => "status_bar_summary",
            Self::ActivityCenter => "activity_center",
            Self::CoverageIntelligence => "coverage_intelligence",
            Self::FlakyIntelligence => "flaky_intelligence",
            Self::SnapshotReview => "snapshot_review",
            Self::PipelineOverlay => "pipeline_overlay",
            Self::ImportedCiView => "imported_ci_view",
            Self::SupportPacket => "support_packet",
        }
    }
}

/// Why a consumer auto-narrows its visible claim language below full local-live
/// certainty. These are the four spec-named auto-narrow conditions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestClaimNarrowReason {
    /// The result was imported (CI, teammate, or replayed snapshot) rather than
    /// produced by a local-live run.
    ResultsImported,
    /// The target / environment compatibility drifted from what produced the
    /// result.
    TargetCompatibilityDrift,
    /// Watch fidelity degraded (reduced, polling, or unavailable).
    WatchFidelityDegraded,
    /// Quarantine visibility is restricted by scope or policy.
    QuarantineVisibilityRestricted,
}

impl M5TestClaimNarrowReason {
    /// Every auto-narrow reason, in declaration order.
    pub const ALL: [M5TestClaimNarrowReason; 4] = [
        M5TestClaimNarrowReason::ResultsImported,
        M5TestClaimNarrowReason::TargetCompatibilityDrift,
        M5TestClaimNarrowReason::WatchFidelityDegraded,
        M5TestClaimNarrowReason::QuarantineVisibilityRestricted,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResultsImported => "results_imported",
            Self::TargetCompatibilityDrift => "target_compatibility_drift",
            Self::WatchFidelityDegraded => "watch_fidelity_degraded",
            Self::QuarantineVisibilityRestricted => "quarantine_visibility_restricted",
        }
    }

    /// The honest, non-generic claim phrase this narrowing shows.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::ResultsImported => "imported result — not a local rerun",
            Self::TargetCompatibilityDrift => "target compatibility drifted from this result",
            Self::WatchFidelityDegraded => "watch fidelity degraded — not live",
            Self::QuarantineVisibilityRestricted => "quarantine visibility restricted by policy",
        }
    }

    /// The recovery hint the narrowed consumer offers to restore full certainty.
    pub const fn recovery(self) -> &'static str {
        match self {
            Self::ResultsImported => "rerun locally to produce a live result",
            Self::TargetCompatibilityDrift => "rerun on the matching target to re-verify",
            Self::WatchFidelityDegraded => "recover the watch session for live fidelity",
            Self::QuarantineVisibilityRestricted => "open the quarantine-review sheet with scope",
        }
    }
}

/// Whether the consumer preserves the canonical component's controlled labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityState {
    /// Full identity / freshness / origin / target / watch / quarantine label
    /// parity; a full local-live claim.
    Preserved,
    /// The claim was auto-narrowed and disclosed, but the labels are still
    /// preserved.
    DisclosedNarrowed,
    /// A label was renamed, flattened, or dropped (red; blocks review).
    RenamedOrDropped,
}

impl LabelParityState {
    /// Returns true when no controlled label is renamed or dropped.
    pub const fn keeps_labels(self) -> bool {
        !matches!(self, Self::RenamedOrDropped)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::RenamedOrDropped => "renamed_or_dropped",
        }
    }
}

/// The copy / export parity a consumer keeps for the adopted component: the
/// governed labels must be copyable as text / JSON / Markdown, and a
/// screenshot-only export is prohibited (it would lose the machine-readable
/// identity and state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportParity {
    /// The copy formats the consumer offers (must include text / json /
    /// markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The export fields the consumer preserves.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl CopyExportParity {
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

/// The auto-narrow banner a consumer shows when its claim is weaker than a full
/// local-live claim. It names every narrow reason and a recovery hint so the
/// reduction is disclosed rather than silently applied.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AutoNarrowBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// The visible, non-generic banner label.
    pub visible_label: String,
    /// The narrow-reason tokens; must equal the row's `claim_narrow_reasons`.
    #[serde(default)]
    pub reasons: Vec<String>,
    /// The recovery hint(s) that would restore full certainty.
    #[serde(default)]
    pub recovery_hints: Vec<String>,
}

/// One consumer adopting one canonical test-explorer / watch / triage component
/// family on one M5 test consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestConsumerRow {
    /// Record kind; must equal [`TEST_CONSUMER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`TEST_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer class.
    pub consumer_class: SharedTestConsumerClass,
    /// The concrete consumer surface; must belong to `consumer_class`.
    pub consumer_surface: SharedTestConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5TestExplorerWatchTriageComponentFamily,
    /// The canonical primitive schema for the family. Must equal
    /// `family_canonical_schema_ref(component_family)`.
    pub canonical_family_schema_ref: String,
    /// The canonical release-proof packet(s) this consumer points back to. Must
    /// contain `family_canonical_packet_ref(component_family)`.
    #[serde(default)]
    pub canonical_packet_refs: Vec<String>,
    /// True when the consumer references the canonical family rather than
    /// cloning surface-local test prose.
    pub references_canonical_not_local_prose: bool,
    /// The imported-versus-live origin of the result this consumer renders.
    pub result_origin: M5TestResultOrigin,
    /// The controlled label families the consumer preserves verbatim (subset of
    /// [`REQUIRED_LABEL_FAMILIES`]).
    #[serde(default)]
    pub preserved_label_families: Vec<String>,
    /// The shared state lexicon the consumer keeps identical to every other
    /// surface (must equal [`SHARED_STATE_LEXICON`]).
    #[serde(default)]
    pub shared_state_lexicon: Vec<String>,
    /// The reasons this consumer auto-narrows its claim, if any.
    #[serde(default)]
    pub claim_narrow_reasons: Vec<M5TestClaimNarrowReason>,
    /// Whether the consumer keeps the controlled labels.
    pub label_parity: LabelParityState,
    /// The auto-narrow banner, present only when the consumer narrows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_narrow_banner: Option<AutoNarrowBanner>,
    /// The copy / export parity of the adopted component.
    pub copy_export: CopyExportParity,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the adoption was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl TestConsumerRow {
    /// Returns true when the consumer auto-narrows its claim.
    pub fn is_narrowed(&self) -> bool {
        !self.claim_narrow_reasons.is_empty()
    }

    /// The surface's declared class matches the row's declared class.
    pub fn surface_class_consistent(&self) -> bool {
        self.consumer_surface.consumer_class() == self.consumer_class
    }

    /// AC1 (canonical): the consumer points back to exactly one canonical family
    /// — the declared schema matches the family, a release-proof packet is
    /// referenced, and no surface-local prose is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == family_canonical_schema_ref(self.component_family)
            && self
                .canonical_packet_refs
                .iter()
                .any(|p| p == family_canonical_packet_ref(self.component_family))
            && self.references_canonical_not_local_prose
    }

    /// AC1 (parity): the consumer preserves the family's controlled label
    /// families and the shared state lexicon rather than renaming or omitting
    /// them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_label_families.is_empty()
            && self
                .preserved_label_families
                .iter()
                .all(|f| REQUIRED_LABEL_FAMILIES.contains(&f.as_str()))
            && self
                .shared_state_lexicon
                .iter()
                .map(String::as_str)
                .eq(SHARED_STATE_LEXICON)
    }

    /// AC2 (imported-versus-live truth): an imported-origin consumer must carry
    /// the `ResultsImported` narrow reason and never claim a full local-live
    /// parity; a local-live consumer must never claim its result was imported.
    pub fn origin_claim_consistent(&self) -> bool {
        let claims_imported = self
            .claim_narrow_reasons
            .contains(&M5TestClaimNarrowReason::ResultsImported);
        if origin_is_imported(self.result_origin) {
            claims_imported && self.label_parity != LabelParityState::Preserved
        } else if self.result_origin == M5TestResultOrigin::LiveLocal {
            !claims_imported
        } else {
            // Synthetic / unknown origins never assert an imported claim.
            !claims_imported
        }
    }

    /// AC2 (auto-narrow disclosure): a narrowed consumer discloses the reduction
    /// with an auto-narrow banner whose reasons match the row and that carries a
    /// recovery hint; a full consumer carries no banner.
    pub fn discloses_narrowing(&self) -> bool {
        if self.is_narrowed() {
            let expected: Vec<String> = self
                .claim_narrow_reasons
                .iter()
                .map(|r| r.as_str().to_owned())
                .collect();
            match &self.auto_narrow_banner {
                None => return false,
                Some(banner) => {
                    if banner.banner_id.trim().is_empty()
                        || banner.visible_label.trim().is_empty()
                        || label_is_generic(&banner.visible_label)
                        || banner.reasons != expected
                        || banner.recovery_hints.is_empty()
                        || banner.recovery_hints.iter().any(|h| h.trim().is_empty())
                    {
                        return false;
                    }
                }
            }
            // A narrowed consumer that keeps every label is disclosed-narrowed,
            // never plain preserved.
            if self.label_parity == LabelParityState::Preserved {
                return false;
            }
        } else if self.auto_narrow_banner.is_some() {
            // A full-claim consumer must not carry a spurious banner.
            return false;
        } else if self.label_parity != LabelParityState::Preserved {
            // A consumer with no narrow reasons is a full local-live claim.
            return false;
        }
        true
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == TEST_CONSUMER_ROW_RECORD_KIND
            && self.schema_version == TEST_CONSUMER_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_family_schema_ref.trim().is_empty()
            && !self.canonical_packet_refs.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.source_refs.is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        let reasons = if self.claim_narrow_reasons.is_empty() {
            "full".to_owned()
        } else {
            self.claim_narrow_reasons
                .iter()
                .map(|r| r.as_str())
                .collect::<Vec<_>>()
                .join("+")
        };
        format!(
            "surface={surface} class={class} family={family} origin={origin} \
label_parity={label_parity} narrow={reasons}",
            surface = self.consumer_surface.as_str(),
            class = self.consumer_class.as_str(),
            family = self.component_family.as_str(),
            origin = self.result_origin.as_str(),
            label_parity = self.label_parity.as_str(),
            reasons = reasons,
        )
    }
}

/// Rolled-up summary of an M05-913 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestConsumerSummary {
    pub row_count: usize,
    pub consumer_class_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_rows_origin_claim_consistent: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_have_copy_export: bool,
    pub day_to_day_editor_present: bool,
    pub quality_intelligence_present: bool,
    pub pipeline_imported_present: bool,
    pub support_export_present: bool,
    pub label_family_coverage_complete: bool,
    pub all_narrow_reasons_demonstrated: bool,
    pub imported_and_live_both_present: bool,
    pub shared_lexicon_uniform: bool,
    pub families_reused_across_classes: usize,
}

/// Constructor input for [`TestConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestConsumerPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<TestConsumerRow>,
}

/// Checked-in M05-913 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<TestConsumerRow>,
    pub summary: TestConsumerSummary,
}

impl TestConsumerPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: TestConsumerPacketInput) -> Self {
        let mut packet = Self {
            schema_version: TEST_CONSUMER_SCHEMA_VERSION,
            record_kind: TEST_CONSUMER_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: TestConsumerSummary {
                row_count: 0,
                consumer_class_count: 0,
                consumer_surface_count: 0,
                component_family_count: 0,
                all_rows_point_to_canonical_family: false,
                all_rows_preserve_labels: false,
                all_rows_origin_claim_consistent: false,
                all_narrowed_rows_disclose: false,
                all_rows_have_copy_export: false,
                day_to_day_editor_present: false,
                quality_intelligence_present: false,
                pipeline_imported_present: false,
                support_export_present: false,
                label_family_coverage_complete: false,
                all_narrow_reasons_demonstrated: false,
                imported_and_live_both_present: false,
                shared_lexicon_uniform: false,
                families_reused_across_classes: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5TestExplorerWatchTriageComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// The union of every row's preserved label families.
    pub fn covered_label_families(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.preserved_label_families.iter().cloned())
            .collect()
    }

    /// The narrow reasons demonstrated by some row.
    pub fn demonstrated_narrow_reasons(&self) -> BTreeSet<M5TestClaimNarrowReason> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_narrow_reasons.iter().copied())
            .collect()
    }

    /// The count of component families adopted by two or more distinct consumer
    /// classes — the strongest evidence that a family is a reusable primitive.
    pub fn families_reused_across_classes(&self) -> usize {
        M5TestExplorerWatchTriageComponentFamily::ALL
            .iter()
            .filter(|family| {
                let classes: BTreeSet<SharedTestConsumerClass> = self
                    .rows
                    .iter()
                    .filter(|r| r.component_family == **family)
                    .map(|r| r.consumer_class)
                    .collect();
                classes.len() >= 2
            })
            .count()
    }

    /// Whether at least one imported-origin auto-narrowing row and one
    /// local-live row are both present (AC2: the two stop diverging).
    pub fn imported_and_live_both_present(&self) -> bool {
        let imported = self
            .rows
            .iter()
            .any(|r| origin_is_imported(r.result_origin) && r.is_narrowed());
        let live = self
            .rows
            .iter()
            .any(|r| r.result_origin == M5TestResultOrigin::LiveLocal);
        imported && live
    }

    /// Whether every row carries the identical shared state lexicon.
    pub fn shared_lexicon_uniform(&self) -> bool {
        self.rows.iter().all(|r| {
            r.shared_state_lexicon
                .iter()
                .map(String::as_str)
                .eq(SHARED_STATE_LEXICON)
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> TestConsumerSummary {
        let mut classes = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        let mut families = BTreeSet::new();
        for row in &self.rows {
            classes.insert(row.consumer_class);
            surfaces.insert(row.consumer_surface);
            families.insert(row.component_family);
        }

        let has_class = |c: SharedTestConsumerClass| classes.contains(&c);
        let covered = self.covered_label_families();
        let demonstrated = self.demonstrated_narrow_reasons();

        TestConsumerSummary {
            row_count: self.rows.len(),
            consumer_class_count: classes.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(TestConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self.rows.iter().all(TestConsumerRow::preserves_labels),
            all_rows_origin_claim_consistent: self
                .rows
                .iter()
                .all(TestConsumerRow::origin_claim_consistent),
            all_narrowed_rows_disclose: self.rows.iter().all(TestConsumerRow::discloses_narrowing),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_complete()),
            day_to_day_editor_present: has_class(SharedTestConsumerClass::DayToDayEditor),
            quality_intelligence_present: has_class(SharedTestConsumerClass::QualityIntelligence),
            pipeline_imported_present: has_class(SharedTestConsumerClass::PipelineImported),
            support_export_present: has_class(SharedTestConsumerClass::SupportExport),
            label_family_coverage_complete: REQUIRED_LABEL_FAMILIES
                .iter()
                .all(|f| covered.contains(*f)),
            all_narrow_reasons_demonstrated: M5TestClaimNarrowReason::ALL
                .iter()
                .all(|r| demonstrated.contains(r)),
            imported_and_live_both_present: self.imported_and_live_both_present(),
            shared_lexicon_uniform: self.shared_lexicon_uniform(),
            families_reused_across_classes: self.families_reused_across_classes(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<TestConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != TEST_CONSUMER_SCHEMA_VERSION {
            violations.push(TestConsumerViolation::SchemaVersion {
                expected: TEST_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != TEST_CONSUMER_RECORD_KIND {
            violations.push(TestConsumerViolation::RecordKind {
                expected: TEST_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(TestConsumerViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_classes = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(TestConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_classes.insert(row.consumer_class);

            if !row.is_complete() {
                violations.push(TestConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }
            if !row.surface_class_consistent() {
                violations.push(TestConsumerViolation::SurfaceClassMismatch {
                    id: row.row_id.clone(),
                });
            }
            if !row.points_to_canonical_family() {
                violations.push(TestConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }
            if !row.preserves_labels() {
                violations.push(TestConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }
            if !row.origin_claim_consistent() {
                violations.push(TestConsumerViolation::OriginClaimDivergent {
                    id: row.row_id.clone(),
                });
            }
            if !row.discloses_narrowing() {
                violations.push(TestConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }
            if !row.copy_export.is_complete() {
                violations.push(TestConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }
        }

        // Cross-surface reuse spans all four claimed consumer classes.
        for class in SharedTestConsumerClass::ALL {
            if !seen_classes.contains(&class) {
                violations.push(TestConsumerViolation::MissingConsumerClass { class });
            }
        }

        // Every frozen family is adopted by at least one consumer.
        let families = self.represented_families();
        for family in M5TestExplorerWatchTriageComponentFamily::ALL {
            if !families.contains(&family) {
                violations.push(TestConsumerViolation::MissingFamilyCoverage { family });
            }
        }

        // AC1: at least one family is reused across two or more consumer classes.
        if self.families_reused_across_classes() == 0 {
            violations.push(TestConsumerViolation::NoFamilyReusedAcrossClasses);
        }

        // AC1: the controlled label families are collectively preserved.
        let covered = self.covered_label_families();
        for family in REQUIRED_LABEL_FAMILIES {
            if !covered.contains(family) {
                violations.push(TestConsumerViolation::MissingLabelFamily {
                    family: family.to_owned(),
                });
            }
        }

        // AC2: every auto-narrow condition is demonstrated somewhere.
        let demonstrated = self.demonstrated_narrow_reasons();
        for reason in M5TestClaimNarrowReason::ALL {
            if !demonstrated.contains(&reason) {
                violations.push(TestConsumerViolation::NarrowReasonNotDemonstrated { reason });
            }
        }

        // AC2: an imported auto-narrowing consumer and a local-live consumer both
        // exist so the two stop diverging on shared state meaning.
        if !self.imported_and_live_both_present() {
            violations.push(TestConsumerViolation::ImportedAndLiveNotBothPresent);
        }

        // AC1: the shared state lexicon is one truth on every surface.
        if !self.shared_lexicon_uniform() {
            violations.push(TestConsumerViolation::SharedLexiconDivergent);
        }

        if self.summary != self.computed_summary() {
            violations.push(TestConsumerViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("consumer packet serializes"),
        ) {
            violations.push(TestConsumerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("consumer packet serializes")
    }

    /// Deterministic CSV of the adoption rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,consumer_class,consumer_surface,component_family,result_origin,label_parity,narrow_reasons\n",
        );
        for row in &self.rows {
            let reasons = if row.claim_narrow_reasons.is_empty() {
                "full".to_owned()
            } else {
                row.claim_narrow_reasons
                    .iter()
                    .map(|r| r.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            };
            out.push_str(&format!(
                "{id},{class},{surface},{family},{origin},{label_parity},{reasons}\n",
                id = row.row_id,
                class = row.consumer_class.as_str(),
                surface = row.consumer_surface.as_str(),
                family = row.component_family.as_str(),
                origin = row.result_origin.as_str(),
                label_parity = row.label_parity.as_str(),
                reasons = reasons,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Test-Explorer / Watch / Triage Component Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Rows: {} across {} consumer classes and {} / {} frozen families\n",
            self.summary.row_count,
            self.summary.consumer_class_count,
            self.represented_families().len(),
            M5TestExplorerWatchTriageComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Families reused across classes: {}\n",
            self.summary.families_reused_across_classes,
        ));
        out.push_str(&format!(
            "- Imported + local-live both present: {}\n",
            self.summary.imported_and_live_both_present,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                family_label(row.component_family),
                row.chip_tokens()
            ));
        }
        out
    }
}

/// Reads and validates the checked-in consumer export.
pub fn current_m5_test_component_consumers_export(
) -> Result<TestConsumerPacket, TestConsumerArtifactError> {
    let packet: TestConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-test-component-consumer-proof/support_export.json"
    )))
    .map_err(TestConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TestConsumerArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in consumer export.
#[derive(Debug)]
pub enum TestConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TestConsumerViolation>),
}

impl fmt::Display for TestConsumerArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => write!(f, "consumer export parse failed: {error}"),
            Self::Validation(violations) => write!(
                f,
                "consumer export failed validation: {} violation(s)",
                violations.len()
            ),
        }
    }
}

impl Error for TestConsumerArtifactError {}

/// Validation failure for M05-913 consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestConsumerViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    SurfaceClassMismatch {
        id: String,
    },
    NotCanonicalFamily {
        id: String,
    },
    LabelParityBroken {
        id: String,
    },
    OriginClaimDivergent {
        id: String,
    },
    NarrowedWithoutDisclosure {
        id: String,
    },
    MissingCopyExportParity {
        id: String,
    },
    MissingConsumerClass {
        class: SharedTestConsumerClass,
    },
    MissingFamilyCoverage {
        family: M5TestExplorerWatchTriageComponentFamily,
    },
    NoFamilyReusedAcrossClasses,
    MissingLabelFamily {
        family: String,
    },
    NarrowReasonNotDemonstrated {
        reason: M5TestClaimNarrowReason,
    },
    ImportedAndLiveNotBothPresent,
    SharedLexiconDivergent,
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for TestConsumerViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema version mismatch: expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete consumer row: {id}"),
            Self::SurfaceClassMismatch { id } => {
                write!(f, "row {id} declares a surface that does not belong to its consumer class")
            }
            Self::NotCanonicalFamily { id } => {
                write!(f, "row {id} does not point back to exactly one canonical component family")
            }
            Self::LabelParityBroken { id } => write!(
                f,
                "row {id} renames or drops a canonical freshness, target-class, watch-state, \
quarantine-semantics, result-origin, or shared-lexicon label"
            ),
            Self::OriginClaimDivergent { id } => write!(
                f,
                "row {id} lets an imported result read as local-live (or vice versa)"
            ),
            Self::NarrowedWithoutDisclosure { id } => {
                write!(f, "row {id} auto-narrows without an auto-narrow banner naming its reasons")
            }
            Self::MissingCopyExportParity { id } => {
                write!(f, "row {id} is missing text / JSON / Markdown copy-export parity")
            }
            Self::MissingConsumerClass { class } => {
                write!(f, "consumer class {class:?} is not adopted in the packet")
            }
            Self::MissingFamilyCoverage { family } => {
                write!(f, "component family {family:?} is not adopted in the packet")
            }
            Self::NoFamilyReusedAcrossClasses => {
                write!(f, "no component family is adopted across two or more consumer classes")
            }
            Self::MissingLabelFamily { family } => {
                write!(f, "controlled label family {family} is not preserved anywhere")
            }
            Self::NarrowReasonNotDemonstrated { reason } => {
                write!(f, "auto-narrow reason {reason:?} is not demonstrated by any consumer")
            }
            Self::ImportedAndLiveNotBothPresent => write!(
                f,
                "the packet must carry both an imported auto-narrowing consumer and a local-live consumer"
            ),
            Self::SharedLexiconDivergent => {
                write!(f, "a consumer diverges from the shared failed / rerun-failed / quarantined lexicon")
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => write!(f, "export contains raw boundary material"),
        }
    }
}

impl Error for TestConsumerViolation {}

/// Whether a banner label is a generic non-answer rather than a precise label.
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
            | "imported"
            | "stale"
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

/// Builds the canonical, checked-in consumer packet. This is the one source of
/// truth shared by the tests, the bin, and the on-disk support export so all
/// three stay byte-aligned.
pub fn seeded_m5_test_component_consumers_packet() -> TestConsumerPacket {
    TestConsumerPacket::new(TestConsumerPacketInput {
        packet_id: "m5-test-component-consumers:stable:0001".to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: TEST_CONSUMER_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:test-component-consumer:{id}")]
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn labels(families: &[&str]) -> Vec<String> {
    families.iter().map(|f| (*f).to_owned()).collect()
}

fn lexicon() -> Vec<String> {
    SHARED_STATE_LEXICON
        .iter()
        .map(|t| (*t).to_owned())
        .collect()
}

fn banner(id: &str, label: &str, reasons: &[M5TestClaimNarrowReason]) -> AutoNarrowBanner {
    AutoNarrowBanner {
        banner_id: id.to_owned(),
        visible_label: label.to_owned(),
        reasons: reasons.iter().map(|r| r.as_str().to_owned()).collect(),
        recovery_hints: reasons.iter().map(|r| r.recovery().to_owned()).collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    consumer_surface: SharedTestConsumerSurface,
    component_family: M5TestExplorerWatchTriageComponentFamily,
    result_origin: M5TestResultOrigin,
    label_families: &[&str],
    export_fields: &[&str],
    narrow_reasons: &[M5TestClaimNarrowReason],
    banner_label: &str,
) -> TestConsumerRow {
    let is_narrowed = !narrow_reasons.is_empty();
    let (label_parity, auto_narrow_banner) = if is_narrowed {
        (
            LabelParityState::DisclosedNarrowed,
            Some(banner(
                &format!("banner:{row_id}"),
                banner_label,
                narrow_reasons,
            )),
        )
    } else {
        (LabelParityState::Preserved, None)
    };
    TestConsumerRow {
        record_kind: TEST_CONSUMER_ROW_RECORD_KIND.to_owned(),
        schema_version: TEST_CONSUMER_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer_class: consumer_surface.consumer_class(),
        consumer_surface,
        component_family,
        canonical_family_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_packet_refs: vec![family_canonical_packet_ref(component_family).to_owned()],
        references_canonical_not_local_prose: true,
        result_origin,
        preserved_label_families: labels(label_families),
        shared_state_lexicon: lexicon(),
        claim_narrow_reasons: narrow_reasons.to_vec(),
        label_parity,
        auto_narrow_banner,
        copy_export: copy_export(export_fields),
        source_refs: vec![
            TEST_CONSUMER_MATRIX_REF.to_owned(),
            family_canonical_schema_ref(component_family).to_owned(),
        ],
        observed_at: "2026-07-07T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn seeded_rows() -> Vec<TestConsumerRow> {
    use M5TestClaimNarrowReason::*;
    use M5TestExplorerWatchTriageComponentFamily::*;
    use M5TestResultOrigin::*;
    use SharedTestConsumerSurface::*;

    vec![
        // --- Day-to-day editor: status bar + activity center ----------------
        row(
            "consumer:status-bar:session-summary-bar",
            StatusBarSummary,
            SessionSummaryBar,
            LiveLocal,
            &["result_freshness", "watch_state", "result_origin"],
            &["session_id", "result_freshness", "watch_state", "result_origin"],
            &[],
            "",
        ),
        row(
            "consumer:activity-center:test-tree-row",
            ActivityCenter,
            TestTreeRow,
            LiveLocal,
            &["result_freshness", "quarantine_semantics", "result_origin"],
            &["item_id", "result_freshness", "quarantine_state", "result_origin"],
            &[],
            "",
        ),
        // --- Quality intelligence: coverage / flaky / snapshot --------------
        row(
            "consumer:coverage:inline-result-marker",
            CoverageIntelligence,
            InlineResultMarker,
            ImportedCi,
            &["result_freshness", "result_origin"],
            &["marker_id", "result_freshness", "result_origin"],
            &[ResultsImported],
            "Imported CI coverage — not a local rerun; rerun locally to produce a live result",
        ),
        row(
            "consumer:flaky:failure-triage-panel",
            FlakyIntelligence,
            FailureTriagePanel,
            LiveLocal,
            &["result_freshness", "quarantine_semantics", "result_origin"],
            &["panel_id", "failure_category", "quarantine_state", "result_origin"],
            &[],
            "",
        ),
        row(
            "consumer:snapshot:environment-matrix-card",
            SnapshotReview,
            EnvironmentMatrixCard,
            LiveLocal,
            &["target_class", "result_freshness", "result_origin"],
            &["card_id", "target_class", "result_freshness", "result_origin"],
            &[TargetCompatibilityDrift],
            "Snapshot target compatibility drifted — rerun on the matching target to re-verify",
        ),
        // --- Pipeline / imported CI -----------------------------------------
        row(
            "consumer:pipeline:watch-mode-banner",
            PipelineOverlay,
            WatchModeBanner,
            ImportedCi,
            &["watch_state", "result_freshness", "result_origin"],
            &["banner_id", "watch_state", "result_freshness", "result_origin"],
            &[ResultsImported, WatchFidelityDegraded],
            "Pipeline watch is imported and degraded — not live; recover the watch session for live fidelity",
        ),
        row(
            "consumer:imported-ci:test-tree-row",
            ImportedCiView,
            TestTreeRow,
            ImportedCi,
            &["result_freshness", "quarantine_semantics", "result_origin"],
            &["item_id", "result_freshness", "quarantine_state", "result_origin"],
            &[ResultsImported],
            "Imported CI results — not a local rerun; rerun locally to produce a live result",
        ),
        row(
            "consumer:imported-ci:session-summary-bar",
            ImportedCiView,
            SessionSummaryBar,
            ImportedTeammate,
            &["result_freshness", "watch_state", "result_origin"],
            &["session_id", "result_freshness", "watch_state", "result_origin"],
            &[ResultsImported],
            "Imported teammate session — not a local rerun; rerun locally to produce a live result",
        ),
        // --- Support / export packets ---------------------------------------
        row(
            "consumer:support:quarantine-review-sheet",
            SupportPacket,
            QuarantineReviewSheet,
            LiveLocal,
            &["quarantine_semantics", "target_class", "result_origin"],
            &["sheet_id", "quarantine_state", "owner", "release_impact", "result_origin"],
            &[QuarantineVisibilityRestricted],
            "Quarantine visibility restricted by policy — open the quarantine-review sheet with scope",
        ),
        row(
            "consumer:support:failure-triage-panel",
            SupportPacket,
            FailureTriagePanel,
            ReplayedSnapshot,
            &["result_freshness", "quarantine_semantics", "result_origin"],
            &["panel_id", "failure_category", "quarantine_state", "result_origin"],
            &[ResultsImported],
            "Replayed snapshot triage — not a local rerun; rerun locally to produce a live result",
        ),
        row(
            "consumer:support:environment-matrix-card",
            SupportPacket,
            EnvironmentMatrixCard,
            ImportedTeammate,
            &["target_class", "result_freshness", "result_origin"],
            &["card_id", "target_class", "result_freshness", "result_origin"],
            &[ResultsImported, TargetCompatibilityDrift],
            "Imported teammate matrix with drifted target — rerun on the matching target to re-verify",
        ),
    ]
}
