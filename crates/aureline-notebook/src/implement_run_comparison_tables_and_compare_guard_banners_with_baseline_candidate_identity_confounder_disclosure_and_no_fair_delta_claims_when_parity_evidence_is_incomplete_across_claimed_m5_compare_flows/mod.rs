//! Two reusable M5 experiment components — the run comparison table and the compare guard
//! banner — so a user can tell whether two results are actually comparable, which baseline and
//! candidate runs are being compared, and what code, data, environment, and hardware factors
//! differ, before trusting a metric delta, exporting a comparison, or escalating a result: the
//! run comparison table names its baseline and candidate run identities, metric values, delta,
//! threshold state, confidence note, comparator type, and explicit code / data / environment /
//! hardware difference summaries, derives a fairness class (`fair`, `caveated`, `unfair`, or
//! `unproven`) from the frozen comparability state, and offers first-class open-baseline /
//! open-candidate / export-comparison actions; the compare guard banner discloses what is
//! comparable, partially comparable, or not comparable — including which lineage fields are
//! missing, which environment / data / code factors changed, and what was redacted — derives a
//! guard comparability class from the frozen guard state, and offers a first-class
//! open-full-lineage action so the comparability guard is never silently bypassed.
//!
//! Aureline's frozen experiment-component matrix
//! ([`crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix`])
//! names the run comparison table and the compare guard banner as two governed component
//! families and freezes their controlled vocabulary — the comparison axis classes
//! (`metric_delta`, `param_diff`, `dataset_diff`, `env_diff`, `code_revision_diff`,
//! `artifact_diff`) and comparability states (`comparable`, `comparable_with_caveats`,
//! `not_comparable`, `confounded`, `insufficient_overlap`, `unknown_comparability`) a table
//! binds; the compare guard reasons (`dataset_mismatch`, `environment_drift`,
//! `code_revision_gap`, `metric_definition_change`, `sample_size_imbalance`,
//! `confounder_present`) and guard states (`comparison_permitted`, `comparison_caveated`,
//! `comparison_blocked`, `guard_acknowledged`, `guard_overridden_by_choice`, `guard_unavailable`)
//! a banner binds; the one controlled disposition vocabulary — including the four reproducibility
//! trust labels (`reproducible`, `likely_reproducible`, `needs_rerun`, `context_incomplete`);
//! the surface families; the deployment lines; the consumer surfaces; the accessibility routes;
//! the required labels; and the downgrade triggers. This module *implements* that contract as two
//! co-equal component vectors so a claimed M5 notebook, experiment-dashboard, comparison,
//! lineage, share-review, or CLI surface can project a comparison table and a guard banner that
//! keep the same truth.
//!
//! The module has two derived resolvers:
//!
//! 1. [`resolve_run_comparison`] — takes a comparison table's comparability state and derives its
//!    fairness class (fair baseline, caveated baseline, unfair baseline, or unproven baseline),
//!    whether the comparison is a fair baseline, and which notes the table must carry — so a
//!    not-comparable, confounded, or insufficiently-overlapping comparison can never read as a
//!    fair apples-to-apples baseline, and the code / data / environment / hardware differences
//!    always stay beside the delta.
//! 2. [`resolve_compare_guard`] — takes a compare guard banner's guard state and derives its
//!    guard comparability class (permitted, partially comparable, overridden, blocked, or
//!    unavailable), whether the guard permits a fair comparison, and which notes the banner must
//!    carry — so a blocked or overridden guard is never silently bypassed and a comparison is
//!    never permitted apples-to-apples when the parity evidence is incomplete.
//!
//! A single controls packet — [`RunComparisonTableCompareGuardBannerControlsPacket`] — binds one
//! vector of comparison tables and one vector of guard banners to the same comparison-axis /
//! comparability, guard-reason / guard-state, deep-link, and non-visual accessibility vocabulary,
//! so baseline / candidate identity and comparability truth stay explicit across desktop,
//! headless / export, and support consumers.
//!
//! The comparison axis class ([`M5ComparisonAxisClass`]), comparability state
//! ([`M5ComparabilityState`]), compare guard reason ([`M5CompareGuardReason`]), compare guard
//! state ([`M5CompareGuardState`]), disposition ([`M5ExperimentDisposition`]), surface family
//! ([`M5ExperimentSurfaceFamily`]), deployment line ([`M5ExperimentDeploymentLine`]), consumer
//! surface ([`M5ExperimentConsumerSurface`]), accessibility route
//! ([`M5ExperimentAccessibilityRoute`]), required label ([`M5ExperimentRequiredLabel`]), and
//! downgrade trigger ([`M5ExperimentDowngradeTrigger`]) are reused verbatim from the frozen
//! matrix. This module mints new vocabulary only for what that matrix left implicit about the two
//! components themselves: the derived fairness and guard-comparability classes, the bounded
//! comparison-table and guard-banner actions, and the deep-link kinds. No M5 experiment surface
//! invents a second comparison-table or guard-banner grammar, and no table or banner invents a
//! comparison-specific comparability, redaction, or trust-label exception.
//!
//! Raw metric payloads, pasted paths, credentials, and private endpoints stay outside the export
//! boundary; every context line, deep-link reference, and component identity is carried only as
//! an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_run_comparison_table_compare_guard_banner_controls,
    seeded_run_comparison_table_compare_guard_banner_controls_compare_guard_banner_blocked,
    seeded_run_comparison_table_compare_guard_banner_controls_comparison_table_not_comparable,
    RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_PACKET_ID,
};

// The comparison axis classes and comparability states, the compare guard reasons and guard
// states, the disposition vocabulary, and the surface / deployment / consumer / accessibility /
// label / downgrade vocabularies are frozen once, in the experiment-component matrix. This lane
// reuses them verbatim so it never invents a parallel comparison-table or guard-banner
// vocabulary.
pub use crate::freeze_the_m5_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_component_matrix::{
    M5ComparabilityState, M5ComparisonAxisClass, M5CompareGuardReason, M5CompareGuardState,
    M5ExperimentAccessibilityRoute, M5ExperimentComponentFamily, M5ExperimentConsumerSurface,
    M5ExperimentDeploymentLine, M5ExperimentDisposition, M5ExperimentDowngradeTrigger,
    M5ExperimentRequiredLabel, M5ExperimentSurfaceFamily, M5_COMPARE_GUARD_BANNER_SCHEMA_REF,
    M5_EXPERIMENT_COMPONENT_DOC_REF, M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
    M5_RUN_COMPARISON_TABLE_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by
/// [`RunComparisonTableCompareGuardBannerControlsPacket`].
pub const RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_RECORD_KIND: &str =
    "implement_m5_run_comparison_tables_and_compare_guard_banners_with_baseline_candidate_identity_confounder_disclosure_and_no_fair_delta_claims_when_parity_evidence_is_incomplete_across_claimed_m5_compare_flows";

/// Schema version for M5 run-comparison-table / compare-guard-banner control records.
pub const RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the controls boundary schema.
pub const RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_REF: &str =
    "schemas/ui/m5-run-comparison-table-compare-guard-banner-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_DOC_REF: &str =
    "docs/notebooks/m5_run_comparison_table_compare_guard_banner_controls.md";

/// Repo-relative path of the protected fixture directory.
pub const RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-run-comparison-table-compare-guard-banner-controls";

/// Repo-relative path of the checked support-export artifact.
pub const RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_ARTIFACT_REF: &str =
    "artifacts/release/m5-run-comparison-table-compare-guard-banner-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_CSV_REF: &str =
    "artifacts/release/m5-run-comparison-table-compare-guard-banner-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_REPORT_REF: &str =
    "artifacts/design/m5-run-comparison-table-compare-guard-banner.md";

// ---- shared deep-link vocabulary ----------------------------------------

/// The kind of stable deep link an experiment component binds its next step against, so a
/// comparison table or guard banner never routes through an ephemeral overlay — every next step
/// is a stable run, notebook, dataset-catalog, or docs reference the user can reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkKind {
    /// A stable experiment-run object reference.
    RunObject,
    /// A stable notebook / cell location.
    NotebookLocation,
    /// A stable dataset-catalog anchor.
    DatasetCatalogAnchor,
    /// A stable docs anchor.
    DocsAnchor,
    /// No deep link is bound (the component names that it routes nowhere).
    NoDeepLink,
}

impl DeepLinkKind {
    /// Every deep-link kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RunObject,
        Self::NotebookLocation,
        Self::DatasetCatalogAnchor,
        Self::DocsAnchor,
        Self::NoDeepLink,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunObject => "run_object",
            Self::NotebookLocation => "notebook_location",
            Self::DatasetCatalogAnchor => "dataset_catalog_anchor",
            Self::DocsAnchor => "docs_anchor",
            Self::NoDeepLink => "no_deep_link",
        }
    }

    /// True when this kind names a resolvable deep-link target.
    pub const fn is_resolvable(self) -> bool {
        !matches!(self, Self::NoDeepLink)
    }
}

// ---- run-comparison-table vocabulary ------------------------------------

/// Derived fairness class a run comparison table may present.
///
/// This is the comparability honesty axis: the class is derived from the frozen comparability
/// state, never asserted, so a not-comparable, confounded, or insufficiently-overlapping
/// comparison can never present as a fair apples-to-apples baseline and a user can always tell
/// how trustworthy a metric delta is before acting on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComparisonFairnessClass {
    /// The two runs are a fair apples-to-apples baseline.
    FairBaseline,
    /// The two runs are comparable only with caveats.
    CaveatedBaseline,
    /// The two runs are not a fair baseline (not comparable or confounded).
    UnfairBaseline,
    /// Comparability is unproven (insufficient overlap or unknown).
    UnprovenBaseline,
}

impl ComparisonFairnessClass {
    /// Every fairness class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FairBaseline,
        Self::CaveatedBaseline,
        Self::UnfairBaseline,
        Self::UnprovenBaseline,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FairBaseline => "fair_baseline",
            Self::CaveatedBaseline => "caveated_baseline",
            Self::UnfairBaseline => "unfair_baseline",
            Self::UnprovenBaseline => "unproven_baseline",
        }
    }

    /// True only when the comparison is a fair apples-to-apples baseline.
    pub const fn is_fair_baseline(self) -> bool {
        matches!(self, Self::FairBaseline)
    }
}

/// One keyboard-complete default action a run comparison table offers, so a table never hides its
/// open / export path behind a pointer-only gesture. `OpenBaselineRun`, `OpenCurrentRun`, and
/// `ExportComparison` are always offered so both compared runs are actionable — and never
/// anonymous — before any trust decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunComparisonAction {
    /// Open the baseline run (always available).
    OpenBaselineRun,
    /// Open the current / candidate run (always available).
    OpenCurrentRun,
    /// Export the comparison metadata only (always available).
    ExportComparison,
    /// Open the full lineage of both runs.
    OpenFullLineage,
    /// Open the stable run / notebook / dataset / docs deep link.
    OpenDeepLink,
    /// Copy the stable comparison id.
    CopyComparisonId,
}

impl RunComparisonAction {
    /// Every comparison-table action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenBaselineRun,
        Self::OpenCurrentRun,
        Self::ExportComparison,
        Self::OpenFullLineage,
        Self::OpenDeepLink,
        Self::CopyComparisonId,
    ];

    /// The default actions every keyboard-complete comparison table must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::OpenBaselineRun,
        Self::OpenCurrentRun,
        Self::ExportComparison,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenBaselineRun => "open_baseline_run",
            Self::OpenCurrentRun => "open_current_run",
            Self::ExportComparison => "export_comparison",
            Self::OpenFullLineage => "open_full_lineage",
            Self::OpenDeepLink => "open_deep_link",
            Self::CopyComparisonId => "copy_comparison_id",
        }
    }
}

/// Disclosures a run comparison table must carry, derived from the comparability state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ComparisonFairnessDisclosure {
    /// The derived fairness class this table may present.
    pub fairness_class: ComparisonFairnessClass,
    /// Whether the comparison is a fair apples-to-apples baseline.
    pub is_fair_baseline: bool,
    /// Whether the table must carry an explicit comparable-with-caveats note.
    pub needs_caveat_note: bool,
    /// Whether the table must carry an explicit not-comparable note.
    pub needs_not_comparable_note: bool,
    /// Whether the table must carry an explicit confounder note.
    pub needs_confounder_note: bool,
    /// Whether the table must carry an explicit insufficient-overlap note.
    pub needs_insufficient_overlap_note: bool,
    /// Whether the table must carry an explicit unknown-comparability note.
    pub needs_unknown_comparability_note: bool,
}

/// Resolves the fairness truth a run comparison table may present.
///
/// A `comparable` comparison is a fair baseline. A `comparable_with_caveats` comparison is a
/// caveated baseline (must carry an explicit caveat note). A `not_comparable` comparison is an
/// unfair baseline (must carry an explicit not-comparable note) and a `confounded` comparison is
/// an unfair baseline (must carry an explicit confounder note). An `insufficient_overlap`
/// comparison is unproven (must carry an explicit insufficient-overlap note) and an
/// `unknown_comparability` comparison is unproven (must carry an explicit unknown note), so a
/// comparison whose parity evidence is incomplete can never read as a fair baseline.
pub fn resolve_run_comparison(state: M5ComparabilityState) -> ComparisonFairnessDisclosure {
    use ComparisonFairnessClass as Fair;
    use M5ComparabilityState as State;

    let fairness_class = match state {
        State::Comparable => Fair::FairBaseline,
        State::ComparableWithCaveats => Fair::CaveatedBaseline,
        State::NotComparable | State::Confounded => Fair::UnfairBaseline,
        State::InsufficientOverlap | State::UnknownComparability => Fair::UnprovenBaseline,
    };

    ComparisonFairnessDisclosure {
        fairness_class,
        is_fair_baseline: fairness_class.is_fair_baseline(),
        needs_caveat_note: matches!(fairness_class, Fair::CaveatedBaseline),
        needs_not_comparable_note: matches!(state, State::NotComparable),
        needs_confounder_note: matches!(state, State::Confounded),
        needs_insufficient_overlap_note: matches!(state, State::InsufficientOverlap),
        needs_unknown_comparability_note: matches!(state, State::UnknownComparability),
    }
}

/// A run comparison table naming its baseline and candidate run identities, comparison axis,
/// metric values, delta, threshold state, confidence note, comparator type, explicit code / data
/// / environment / hardware difference summaries, derived fairness class, comparability state,
/// bounded open / export actions, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunComparisonTable {
    /// Frozen component this control implements; must be `run_comparison_table`.
    pub component: M5ExperimentComponentFamily,
    /// Stable comparison-table id.
    pub table_id: String,
    /// Human-readable comparison label; required and non-empty.
    pub comparison_label: String,
    /// Comparison axis class, reused from the frozen matrix.
    pub comparison_axis: M5ComparisonAxisClass,
    /// Comparability state, reused from the frozen matrix.
    pub comparability_state: M5ComparabilityState,
    /// Derived fairness class (must equal the resolved class).
    pub fairness_class: ComparisonFairnessClass,
    /// Whether the table claims a fair baseline (must equal the derived truth).
    pub claims_fair_baseline: bool,
    /// Stable baseline run id; always required so no baseline is anonymous.
    pub baseline_run_id: String,
    /// Human-readable baseline run label; always required.
    pub baseline_run_label: String,
    /// Stable candidate / current run id; always required so no candidate is anonymous.
    pub candidate_run_id: String,
    /// Human-readable candidate / current run label; always required.
    pub candidate_run_label: String,
    /// Metric value note (baseline vs current); always required so the compared values stay
    /// explicit.
    pub metric_value_note: String,
    /// Delta note; always required so the metric delta stays explicit.
    pub delta_note: String,
    /// Threshold state note; always required so whether the delta crosses a threshold stays
    /// explicit.
    pub threshold_state_note: String,
    /// Confidence note; always required so the confidence in the delta stays explicit.
    pub confidence_note: String,
    /// Comparator type note; always required so how the delta was computed stays explicit.
    pub comparator_type_note: String,
    /// Code revision difference summary; always required so a code change beside the delta stays
    /// visible.
    pub code_difference_note: String,
    /// Data snapshot difference summary; always required so a data change beside the delta stays
    /// visible.
    pub data_difference_note: String,
    /// Environment fingerprint difference summary; always required so an environment change
    /// beside the delta stays visible.
    pub environment_difference_note: String,
    /// Hardware / profile-class difference summary; always required so a hardware change beside
    /// the delta stays visible.
    pub hardware_difference_note: String,
    /// Comparable-with-caveats note; required when the comparison is caveated.
    pub caveat_note: String,
    /// Not-comparable note; required when the comparison is not comparable.
    pub not_comparable_note: String,
    /// Confounder note; required when the comparison is confounded.
    pub confounder_note: String,
    /// Insufficient-overlap note; required when the comparison has insufficient overlap.
    pub insufficient_overlap_note: String,
    /// Unknown-comparability note; required when the comparability is unknown.
    pub unknown_comparability_note: String,
    /// Comparability / parity note; always required so the comparability truth stays explicit.
    pub comparability_and_parity_note: String,
    /// Context note; always required so the table names what to check before trusting the delta.
    pub context_note: String,
    /// Kind of stable deep link this table binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include open-baseline / open-current / export).
    pub table_actions: Vec<RunComparisonAction>,
    /// Dispositions this table binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ExperimentDisposition>,
    /// Downgrade triggers this table can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Mandatory labels this table can show (must include the mandatory labels).
    pub required_labels: Vec<M5ExperimentRequiredLabel>,
    /// Claimed M5 surface families that render this table.
    pub surface_families: Vec<M5ExperimentSurfaceFamily>,
    /// Deployment lines this table keeps the same truth across.
    pub deployment_lines: Vec<M5ExperimentDeploymentLine>,
    /// Non-visual accessibility routes this table offers.
    pub accessibility_routes: Vec<M5ExperimentAccessibilityRoute>,
    /// Experiment subsystems that consume this table's projection.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this table.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks provenance or sensitivity state. MUST be `false`.
    pub masks_provenance_or_sensitivity_state: bool,
    /// Hard invariant: never hides the baseline or candidate identity. MUST be `false`.
    pub hides_baseline_or_candidate_identity: bool,
    /// Hard invariant: never hides the code / data / environment / hardware differences beside
    /// the delta. MUST be `false`.
    pub hides_difference_factors_beside_delta: bool,
    /// Hard invariant: never implies apples-to-apples without parity. MUST be `false`.
    pub implies_apples_to_apples_without_parity: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl RunComparisonTable {
    /// Fairness disclosures this table must carry, derived from its comparability state.
    pub fn fairness_disclosure(&self) -> ComparisonFairnessDisclosure {
        resolve_run_comparison(self.comparability_state)
    }

    /// Whether the table offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<RunComparisonAction> = self.table_actions.iter().copied().collect();
        RunComparisonAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the table declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ExperimentRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ExperimentRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the table offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.table_actions
            .contains(&RunComparisonAction::OpenDeepLink)
    }
}

// ---- compare-guard-banner vocabulary ------------------------------------

/// Derived guard comparability class a compare guard banner may present.
///
/// This is the guard honesty axis: the class is derived from the frozen guard state, never
/// asserted, so a blocked or overridden guard can never present as a permitted comparison and a
/// user can always tell whether a comparison the guard is watching is permitted, partially
/// comparable, overridden by explicit choice, blocked, or unavailable before trusting a delta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardComparabilityClass {
    /// The guard permits an apples-to-apples comparison.
    ComparablePermitted,
    /// The guard permits the comparison only partially (caveated or acknowledged).
    PartiallyComparable,
    /// The guard was overridden by explicit choice.
    OverriddenComparison,
    /// The guard blocks the comparison.
    NotComparableBlocked,
    /// The guard is unavailable (comparability cannot be established).
    GuardUnavailable,
}

impl GuardComparabilityClass {
    /// Every guard comparability class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ComparablePermitted,
        Self::PartiallyComparable,
        Self::OverriddenComparison,
        Self::NotComparableBlocked,
        Self::GuardUnavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComparablePermitted => "comparable_permitted",
            Self::PartiallyComparable => "partially_comparable",
            Self::OverriddenComparison => "overridden_comparison",
            Self::NotComparableBlocked => "not_comparable_blocked",
            Self::GuardUnavailable => "guard_unavailable",
        }
    }

    /// True only when the guard permits a fair apples-to-apples comparison.
    pub const fn permits_fair_comparison(self) -> bool {
        matches!(self, Self::ComparablePermitted)
    }
}

/// One keyboard-complete default action a compare guard banner offers, so a banner never hides
/// its open-full-lineage path behind a pointer-only gesture. `OpenFullLineage` and
/// `ReviewComparability` are always offered so the comparability guard can always be inspected
/// before any comparison is trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareGuardAction {
    /// Open the full lineage of the guarded comparison (always available).
    OpenFullLineage,
    /// Review what is comparable, partially comparable, or not comparable (always available).
    ReviewComparability,
    /// View the changed environment / data / code factors.
    ViewChangedFactors,
    /// Acknowledge the guard without overriding it.
    AcknowledgeGuard,
    /// Open the stable run / notebook / dataset / docs deep link.
    OpenDeepLink,
    /// Copy the stable guard id.
    CopyGuardId,
}

impl CompareGuardAction {
    /// Every guard-banner action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenFullLineage,
        Self::ReviewComparability,
        Self::ViewChangedFactors,
        Self::AcknowledgeGuard,
        Self::OpenDeepLink,
        Self::CopyGuardId,
    ];

    /// The default actions every keyboard-complete guard banner must offer.
    pub const MANDATORY: [Self; 2] = [Self::OpenFullLineage, Self::ReviewComparability];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenFullLineage => "open_full_lineage",
            Self::ReviewComparability => "review_comparability",
            Self::ViewChangedFactors => "view_changed_factors",
            Self::AcknowledgeGuard => "acknowledge_guard",
            Self::OpenDeepLink => "open_deep_link",
            Self::CopyGuardId => "copy_guard_id",
        }
    }
}

/// Disclosures a compare guard banner must carry, derived from the guard state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GuardComparabilityDisclosure {
    /// The derived guard comparability class this banner may present.
    pub guard_class: GuardComparabilityClass,
    /// Whether the guard permits a fair apples-to-apples comparison.
    pub permits_fair_comparison: bool,
    /// Whether the comparison is only partially comparable.
    pub is_partially_comparable: bool,
    /// Whether the guard blocks the comparison.
    pub is_blocked: bool,
    /// Whether the banner must carry an explicit partial-comparability note.
    pub needs_partial_comparability_note: bool,
    /// Whether the banner must carry an explicit override warning.
    pub needs_override_warning: bool,
    /// Whether the banner must carry an explicit blocked note.
    pub needs_blocked_note: bool,
    /// Whether the banner must carry an explicit guard-unavailable note.
    pub needs_guard_unavailable_note: bool,
}

/// Resolves the guard truth a compare guard banner may present.
///
/// A `comparison_permitted` guard is comparable-permitted. A `comparison_caveated` or
/// `guard_acknowledged` guard is partially comparable (must carry an explicit partial note). A
/// `guard_overridden_by_choice` guard is overridden (must carry an explicit override warning). A
/// `comparison_blocked` guard is blocked (must carry an explicit blocked note). A
/// `guard_unavailable` guard is unavailable (must carry an explicit unavailable note), so a
/// blocked or overridden comparison can never read as a permitted apples-to-apples comparison.
pub fn resolve_compare_guard(state: M5CompareGuardState) -> GuardComparabilityDisclosure {
    use GuardComparabilityClass as Guard;
    use M5CompareGuardState as State;

    let guard_class = match state {
        State::ComparisonPermitted => Guard::ComparablePermitted,
        State::ComparisonCaveated | State::GuardAcknowledged => Guard::PartiallyComparable,
        State::GuardOverriddenByChoice => Guard::OverriddenComparison,
        State::ComparisonBlocked => Guard::NotComparableBlocked,
        State::GuardUnavailable => Guard::GuardUnavailable,
    };

    GuardComparabilityDisclosure {
        guard_class,
        permits_fair_comparison: guard_class.permits_fair_comparison(),
        is_partially_comparable: matches!(guard_class, Guard::PartiallyComparable),
        is_blocked: matches!(guard_class, Guard::NotComparableBlocked),
        needs_partial_comparability_note: matches!(guard_class, Guard::PartiallyComparable),
        needs_override_warning: matches!(state, State::GuardOverriddenByChoice),
        needs_blocked_note: matches!(state, State::ComparisonBlocked),
        needs_guard_unavailable_note: matches!(state, State::GuardUnavailable),
    }
}

/// A compare guard banner naming its guard reason, guard state, derived guard comparability
/// class, what is comparable / partially comparable / not comparable, which lineage fields are
/// missing, which environment / data / code factors changed, what was redacted, bounded
/// open-full-lineage / review actions, and a stable deep link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareGuardBanner {
    /// Frozen component this control implements; must be `compare_guard_banner`.
    pub component: M5ExperimentComponentFamily,
    /// Stable guard-banner id.
    pub banner_id: String,
    /// Human-readable guard-banner label; required and non-empty.
    pub banner_label: String,
    /// Compare guard reason, reused from the frozen matrix.
    pub guard_reason: M5CompareGuardReason,
    /// Compare guard state, reused from the frozen matrix.
    pub guard_state: M5CompareGuardState,
    /// Derived guard comparability class (must equal the resolved class).
    pub guard_class: GuardComparabilityClass,
    /// Whether the banner claims the guard permits a fair comparison (must equal derived truth).
    pub claims_permits_fair_comparison: bool,
    /// Comparability disclosure note (what is comparable / partially / not); always required.
    pub comparability_disclosure_note: String,
    /// Missing-lineage-fields note; always required so which lineage fields are missing stays
    /// explicit.
    pub missing_lineage_fields_note: String,
    /// Changed-factors note; always required so which environment / data / code factors changed
    /// stays explicit.
    pub changed_factors_note: String,
    /// Redaction note; always required so what was redacted before compare / share stays
    /// explicit.
    pub redaction_note: String,
    /// Guard reason note; always required so why the comparison is guarded stays explicit.
    pub guard_reason_note: String,
    /// Partial-comparability note; required when the comparison is only partially comparable.
    pub partial_comparability_note: String,
    /// Override warning; required when the guard was overridden by explicit choice.
    pub override_warning: String,
    /// Blocked note; required when the guard blocks the comparison.
    pub blocked_note: String,
    /// Guard-unavailable note; required when the guard is unavailable.
    pub guard_unavailable_note: String,
    /// Comparability / parity note; always required so the comparability truth stays explicit.
    pub comparability_and_parity_note: String,
    /// Context note; always required so the banner names what to check before trusting a compare.
    pub context_note: String,
    /// Kind of stable deep link this banner binds its next step against.
    pub deep_link_kind: DeepLinkKind,
    /// Opaque stable deep-link reference; required when the kind resolves.
    pub deep_link_ref: String,
    /// Keyboard-complete default actions (must include open-full-lineage / review).
    pub banner_actions: Vec<CompareGuardAction>,
    /// Dispositions this banner binds (required, matching the frozen matrix vocabulary).
    pub dispositions: Vec<M5ExperimentDisposition>,
    /// Downgrade triggers this banner can name (required, matching the frozen matrix).
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Mandatory labels this banner can show (must include the mandatory labels).
    pub required_labels: Vec<M5ExperimentRequiredLabel>,
    /// Claimed M5 surface families that render this banner.
    pub surface_families: Vec<M5ExperimentSurfaceFamily>,
    /// Deployment lines this banner keeps the same truth across.
    pub deployment_lines: Vec<M5ExperimentDeploymentLine>,
    /// Non-visual accessibility routes this banner offers.
    pub accessibility_routes: Vec<M5ExperimentAccessibilityRoute>,
    /// Experiment subsystems that consume this banner's projection.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this banner.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks provenance or sensitivity state. MUST be `false`.
    pub masks_provenance_or_sensitivity_state: bool,
    /// Hard invariant: never hides the baseline or candidate identity. MUST be `false`.
    pub hides_baseline_or_candidate_identity: bool,
    /// Hard invariant: never hides the code / data / environment / hardware differences beside
    /// the delta. MUST be `false`.
    pub hides_difference_factors_beside_delta: bool,
    /// Hard invariant: never implies apples-to-apples without parity. MUST be `false`.
    pub implies_apples_to_apples_without_parity: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
}

impl CompareGuardBanner {
    /// Guard disclosures this banner must carry, derived from its guard state.
    pub fn guard_disclosure(&self) -> GuardComparabilityDisclosure {
        resolve_compare_guard(self.guard_state)
    }

    /// Whether the banner offers every mandatory keyboard-complete action.
    fn declares_mandatory_actions(&self) -> bool {
        let present: BTreeSet<CompareGuardAction> = self.banner_actions.iter().copied().collect();
        CompareGuardAction::MANDATORY
            .iter()
            .all(|action| present.contains(action))
    }

    /// Whether the banner declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ExperimentRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ExperimentRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the banner offers a deep-link-opening action.
    fn offers_deep_link_action(&self) -> bool {
        self.banner_actions
            .contains(&CompareGuardAction::OpenDeepLink)
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance comparison / guard review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareReview {
    /// The comparison table names its baseline and candidate runs.
    pub comparison_table_shows_baseline_and_candidate: bool,
    /// The comparison table names its delta and its difference factors.
    pub comparison_table_shows_delta_and_difference_factors: bool,
    /// The comparison table offers open-baseline / open-current / export.
    pub comparison_table_offers_open_runs_and_export: bool,
    /// The guard banner names its comparability disclosure and guard reason.
    pub compare_guard_banner_shows_comparability_and_reason: bool,
    /// The guard banner offers open-full-lineage and review.
    pub compare_guard_banner_offers_full_lineage_and_review: bool,
    /// Fairness and guard comparability are derived from state, never asserted.
    pub fairness_and_comparability_derived_never_asserted: bool,
    /// An unfair or unproven comparison is never shown as a fair baseline.
    pub unfair_or_unproven_never_shown_as_fair: bool,
    /// The code / data / environment / hardware differences stay beside the delta.
    pub differences_shown_beside_delta: bool,
    /// Apples-to-apples is never implied without parity evidence.
    pub apples_to_apples_never_implied_without_parity: bool,
    /// The reproducibility trust labels are used consistently across compare surfaces.
    pub reproducibility_trust_labels_used_consistently: bool,
    /// Every next step names one stable run / notebook / dataset / docs deep link.
    pub every_next_step_names_stable_deep_link: bool,
    /// Tables and banners reuse Aureline's existing comparability vocabulary.
    pub reuses_existing_comparability_vocabulary: bool,
    /// Provenance and sensitivity state stays visible.
    pub provenance_and_sensitivity_state_visible: bool,
    /// Changed factors and missing lineage fields stay visible.
    pub changed_factors_and_missing_lineage_visible: bool,
    /// Redaction state stays visible before a compare or share.
    pub redaction_state_visible_before_share: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The components keep the same truth across every deployment line.
    pub components_stable_across_deployment_lines: bool,
    /// The components stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl CompareReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.comparison_table_shows_baseline_and_candidate
            && self.comparison_table_shows_delta_and_difference_factors
            && self.comparison_table_offers_open_runs_and_export
            && self.compare_guard_banner_shows_comparability_and_reason
            && self.compare_guard_banner_offers_full_lineage_and_review
            && self.fairness_and_comparability_derived_never_asserted
            && self.unfair_or_unproven_never_shown_as_fair
            && self.differences_shown_beside_delta
            && self.apples_to_apples_never_implied_without_parity
            && self.reproducibility_trust_labels_used_consistently
            && self.every_next_step_names_stable_deep_link
            && self.reuses_existing_comparability_vocabulary
            && self.provenance_and_sensitivity_state_visible
            && self.changed_factors_and_missing_lineage_visible
            && self.redaction_state_visible_before_share
            && self.no_surface_invents_alternate_state_label
            && self.components_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareConsumerProjection {
    /// The comparison surface reads a single canonical source.
    pub comparison_ui_reads_single_source: bool,
    /// The compare-guard surface reads a single canonical source.
    pub compare_guard_surface_reads_single_source: bool,
    /// The baseline and candidate runs are visible before a trust decision.
    pub baseline_and_candidate_visible_before_trust: bool,
    /// The difference factors are visible before a trust decision.
    pub difference_factors_visible_before_trust: bool,
    /// Support export shows component truth.
    pub support_export_shows_component_truth: bool,
    /// Help / docs shows component truth.
    pub help_docs_shows_component_truth: bool,
}

impl CompareConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.comparison_ui_reads_single_source
            && self.compare_guard_surface_reads_single_source
            && self.baseline_and_candidate_visible_before_trust
            && self.difference_factors_visible_before_trust
            && self.support_export_shows_component_truth
            && self.help_docs_shows_component_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for
/// [`RunComparisonTableCompareGuardBannerControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunComparisonTableCompareGuardBannerControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Run comparison tables.
    pub comparison_tables: Vec<RunComparisonTable>,
    /// Compare guard banners.
    pub guard_banners: Vec<CompareGuardBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Comparison / guard review block.
    pub compare_review: CompareReview,
    /// Consumer projection block.
    pub consumer_projection: CompareConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CompareProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe run-comparison-table / compare-guard-banner controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunComparisonTableCompareGuardBannerControlsPacket {
    /// Record kind; must equal
    /// [`RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Run comparison tables.
    pub comparison_tables: Vec<RunComparisonTable>,
    /// Compare guard banners.
    pub guard_banners: Vec<CompareGuardBanner>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ExperimentDowngradeTrigger>,
    /// Consumer surfaces that must reuse these components.
    pub consumer_surfaces: Vec<M5ExperimentConsumerSurface>,
    /// Comparison / guard review block.
    pub compare_review: CompareReview,
    /// Consumer projection block.
    pub consumer_projection: CompareConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CompareProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RunComparisonTableCompareGuardBannerControlsPacket {
    /// Builds a run-comparison-table / compare-guard-banner controls packet from stable-lane
    /// input.
    pub fn new(input: RunComparisonTableCompareGuardBannerControlsPacketInput) -> Self {
        Self {
            record_kind: RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_RECORD_KIND.to_owned(),
            schema_version: RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            comparison_tables: input.comparison_tables,
            guard_banners: input.guard_banners,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            compare_review: input.compare_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the run-comparison-table / compare-guard-banner control invariants.
    pub fn validate(&self) -> Vec<RunComparisonTableCompareGuardBannerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_RECORD_KIND {
            violations.push(RunComparisonTableCompareGuardBannerViolation::WrongRecordKind);
        }
        if self.schema_version != RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_VERSION {
            violations.push(RunComparisonTableCompareGuardBannerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RunComparisonTableCompareGuardBannerViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(RunComparisonTableCompareGuardBannerViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_comparison_tables(self, &mut violations);
        validate_guard_banners(self, &mut violations);
        validate_trust_label_coverage(self, &mut violations);

        if !self.compare_review.all_hold() {
            violations.push(RunComparisonTableCompareGuardBannerViolation::CompareReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("run comparison compare guard packet serializes"),
        ) {
            violations.push(RunComparisonTableCompareGuardBannerViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("run comparison compare guard packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str("component,id,state,axis_or_reason,derived,safe_flag,deep_link_kind\n");
        for table in &self.comparison_tables {
            let disclosure = table.fairness_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "run_comparison_table",
                csv_field(&table.table_id),
                table.comparability_state.as_str(),
                table.comparison_axis.as_str(),
                disclosure.fairness_class.as_str(),
                disclosure.is_fair_baseline,
                table.deep_link_kind.as_str(),
            ));
        }
        for banner in &self.guard_banners {
            let disclosure = banner.guard_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                "compare_guard_banner",
                csv_field(&banner.banner_id),
                banner.guard_state.as_str(),
                banner.guard_reason.as_str(),
                disclosure.guard_class.as_str(),
                disclosure.permits_fair_comparison,
                banner.deep_link_kind.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let unfair = self
            .comparison_tables
            .iter()
            .filter(|table| !table.fairness_disclosure().is_fair_baseline)
            .count();
        let guarded = self
            .guard_banners
            .iter()
            .filter(|banner| !banner.guard_disclosure().permits_fair_comparison)
            .count();

        let mut out = String::new();
        out.push_str("# Run comparison tables and compare guard banners\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Run comparison tables: {} ({} not a fair baseline)\n",
            self.comparison_tables.len(),
            unfair
        ));
        out.push_str(&format!(
            "- Compare guard banners: {} ({} do not permit a fair comparison)\n",
            self.guard_banners.len(),
            guarded
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Run comparison tables\n\n");
        for table in &self.comparison_tables {
            let disclosure = table.fairness_disclosure();
            out.push_str(&format!(
                "- **{}** — axis `{}`, comparability `{}` → `{}`, baseline `{}` vs candidate `{}`, deep link `{}`\n",
                table.comparison_label,
                table.comparison_axis.as_str(),
                table.comparability_state.as_str(),
                disclosure.fairness_class.as_str(),
                table.baseline_run_id,
                table.candidate_run_id,
                table.deep_link_kind.as_str(),
            ));
        }

        out.push_str("\n## Compare guard banners\n\n");
        for banner in &self.guard_banners {
            let disclosure = banner.guard_disclosure();
            out.push_str(&format!(
                "- **{}** — reason `{}`, guard `{}` → `{}`, permits-fair `{}`, deep link `{}`\n",
                banner.banner_label,
                banner.guard_reason.as_str(),
                banner.guard_state.as_str(),
                disclosure.guard_class.as_str(),
                disclosure.permits_fair_comparison,
                banner.deep_link_kind.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in run-comparison-table / compare-guard-banner export.
#[derive(Debug)]
pub enum RunComparisonTableCompareGuardBannerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RunComparisonTableCompareGuardBannerViolation>),
}

impl fmt::Display for RunComparisonTableCompareGuardBannerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "run comparison compare guard export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "run comparison compare guard export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RunComparisonTableCompareGuardBannerArtifactError {}

/// Validation failures emitted by
/// [`RunComparisonTableCompareGuardBannerControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunComparisonTableCompareGuardBannerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No run comparison tables are present.
    ComparisonTablesMissing,
    /// A run comparison table is incomplete.
    ComparisonTableIncomplete,
    /// A run comparison table carries the wrong frozen component class.
    ComparisonTableWrongComponentClass,
    /// A comparison table misrepresents its derived fairness class.
    FairnessMisrepresented,
    /// A comparison table does not name its baseline or candidate run.
    BaselineOrCandidateMissing,
    /// A comparison table does not name its metric value, delta, threshold, confidence, or
    /// comparator type.
    MetricOrDeltaMissing,
    /// A comparison table does not name its code / data / environment / hardware differences.
    DifferenceFactorsMissing,
    /// A caveated comparison does not name its caveat.
    CaveatNoteMissing,
    /// A not-comparable comparison does not name its not-comparable state.
    NotComparableNoteMissing,
    /// A confounded comparison does not name its confounder.
    ConfounderNoteMissing,
    /// An insufficient-overlap comparison does not name its insufficient overlap.
    InsufficientOverlapNoteMissing,
    /// An unknown-comparability comparison does not name its unknown comparability.
    UnknownComparabilityNoteMissing,
    /// A comparison table does not name its comparability / parity truth.
    ComparabilityAndParityNoteMissing,
    /// A comparison table omits a mandatory open / export action.
    ComparisonTableActionsIncomplete,
    /// The comparison tables do not cover every derived fairness class.
    FairnessClassCoverageMissing,
    /// The comparison tables do not cover every comparison axis class.
    ComparisonAxisCoverageMissing,
    /// The comparison tables do not cover every comparability state.
    ComparabilityStateCoverageMissing,
    /// No compare guard banners are present.
    GuardBannersMissing,
    /// A compare guard banner is incomplete.
    GuardBannerIncomplete,
    /// A compare guard banner carries the wrong frozen component class.
    GuardBannerWrongComponentClass,
    /// A guard banner misrepresents its derived guard comparability class.
    GuardClassMisrepresented,
    /// A guard banner does not name its comparability disclosure.
    ComparabilityDisclosureMissing,
    /// A guard banner does not name its missing lineage fields.
    MissingLineageFieldsNoteMissing,
    /// A guard banner does not name its changed factors.
    ChangedFactorsNoteMissing,
    /// A guard banner does not name its redaction state.
    GuardRedactionNoteMissing,
    /// A guard banner does not name its guard reason.
    GuardReasonNoteMissing,
    /// A partially comparable guard does not name its partial comparability.
    PartialComparabilityNoteMissing,
    /// An overridden guard does not name its override warning.
    OverrideWarningMissing,
    /// A blocked guard does not name its blocked state.
    BlockedNoteMissing,
    /// An unavailable guard does not name its unavailable state.
    GuardUnavailableNoteMissing,
    /// A guard banner omits a mandatory open-full-lineage / review action.
    GuardBannerActionsIncomplete,
    /// The guard banners do not cover every derived guard comparability class.
    GuardClassCoverageMissing,
    /// The guard banners do not cover every compare guard reason.
    GuardReasonCoverageMissing,
    /// The guard banners do not cover every compare guard state.
    GuardStateCoverageMissing,
    /// The components do not cover every reproducibility trust label.
    TrustLabelCoverageMissing,
    /// A component does not name its context.
    ContextNoteMissing,
    /// A component offers a deep-link action but its deep link does not resolve exactly.
    DeepLinkUnresolved,
    /// A component names a deep-link kind but not its stable reference.
    DeepLinkRefMissing,
    /// A component does not bind any disposition.
    DispositionsMissing,
    /// A component does not declare its downgrade triggers.
    DowngradeTriggersMissing,
    /// A component does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A component does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A component masks its provenance or sensitivity state.
    ProvenanceOrSensitivityStateMasked,
    /// A component hides the baseline or candidate identity.
    BaselineOrCandidateIdentityHidden,
    /// A component hides the difference factors beside the delta.
    DifferenceFactorsHidden,
    /// A component implies apples-to-apples without parity evidence.
    ApplesToApplesImpliedWithoutParity,
    /// A component invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Comparison / guard review does not satisfy required invariants.
    CompareReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl RunComparisonTableCompareGuardBannerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ComparisonTablesMissing => "comparison_tables_missing",
            Self::ComparisonTableIncomplete => "comparison_table_incomplete",
            Self::ComparisonTableWrongComponentClass => "comparison_table_wrong_component_class",
            Self::FairnessMisrepresented => "fairness_misrepresented",
            Self::BaselineOrCandidateMissing => "baseline_or_candidate_missing",
            Self::MetricOrDeltaMissing => "metric_or_delta_missing",
            Self::DifferenceFactorsMissing => "difference_factors_missing",
            Self::CaveatNoteMissing => "caveat_note_missing",
            Self::NotComparableNoteMissing => "not_comparable_note_missing",
            Self::ConfounderNoteMissing => "confounder_note_missing",
            Self::InsufficientOverlapNoteMissing => "insufficient_overlap_note_missing",
            Self::UnknownComparabilityNoteMissing => "unknown_comparability_note_missing",
            Self::ComparabilityAndParityNoteMissing => "comparability_and_parity_note_missing",
            Self::ComparisonTableActionsIncomplete => "comparison_table_actions_incomplete",
            Self::FairnessClassCoverageMissing => "fairness_class_coverage_missing",
            Self::ComparisonAxisCoverageMissing => "comparison_axis_coverage_missing",
            Self::ComparabilityStateCoverageMissing => "comparability_state_coverage_missing",
            Self::GuardBannersMissing => "guard_banners_missing",
            Self::GuardBannerIncomplete => "guard_banner_incomplete",
            Self::GuardBannerWrongComponentClass => "guard_banner_wrong_component_class",
            Self::GuardClassMisrepresented => "guard_class_misrepresented",
            Self::ComparabilityDisclosureMissing => "comparability_disclosure_missing",
            Self::MissingLineageFieldsNoteMissing => "missing_lineage_fields_note_missing",
            Self::ChangedFactorsNoteMissing => "changed_factors_note_missing",
            Self::GuardRedactionNoteMissing => "guard_redaction_note_missing",
            Self::GuardReasonNoteMissing => "guard_reason_note_missing",
            Self::PartialComparabilityNoteMissing => "partial_comparability_note_missing",
            Self::OverrideWarningMissing => "override_warning_missing",
            Self::BlockedNoteMissing => "blocked_note_missing",
            Self::GuardUnavailableNoteMissing => "guard_unavailable_note_missing",
            Self::GuardBannerActionsIncomplete => "guard_banner_actions_incomplete",
            Self::GuardClassCoverageMissing => "guard_class_coverage_missing",
            Self::GuardReasonCoverageMissing => "guard_reason_coverage_missing",
            Self::GuardStateCoverageMissing => "guard_state_coverage_missing",
            Self::TrustLabelCoverageMissing => "trust_label_coverage_missing",
            Self::ContextNoteMissing => "context_note_missing",
            Self::DeepLinkUnresolved => "deep_link_unresolved",
            Self::DeepLinkRefMissing => "deep_link_ref_missing",
            Self::DispositionsMissing => "dispositions_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ProvenanceOrSensitivityStateMasked => "provenance_or_sensitivity_state_masked",
            Self::BaselineOrCandidateIdentityHidden => "baseline_or_candidate_identity_hidden",
            Self::DifferenceFactorsHidden => "difference_factors_hidden",
            Self::ApplesToApplesImpliedWithoutParity => "apples_to_apples_implied_without_parity",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::CompareReviewIncomplete => "compare_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable run-comparison-table / compare-guard-banner export.
pub fn current_run_comparison_table_compare_guard_banner_export() -> Result<
    RunComparisonTableCompareGuardBannerControlsPacket,
    RunComparisonTableCompareGuardBannerArtifactError,
> {
    let packet: RunComparisonTableCompareGuardBannerControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-run-comparison-table-compare-guard-banner-proof/support_export.json"
        )))
        .map_err(RunComparisonTableCompareGuardBannerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RunComparisonTableCompareGuardBannerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &RunComparisonTableCompareGuardBannerControlsPacket,
    violations: &mut Vec<RunComparisonTableCompareGuardBannerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_REF,
        RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_DOC_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_DOC_REF,
        M5_RUN_COMPARISON_TABLE_SCHEMA_REF,
        M5_COMPARE_GUARD_BANNER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(RunComparisonTableCompareGuardBannerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_comparison_tables(
    packet: &RunComparisonTableCompareGuardBannerControlsPacket,
    violations: &mut Vec<RunComparisonTableCompareGuardBannerViolation>,
) {
    if packet.comparison_tables.is_empty() {
        violations.push(RunComparisonTableCompareGuardBannerViolation::ComparisonTablesMissing);
        return;
    }

    let mut fairness_classes: BTreeSet<ComparisonFairnessClass> = BTreeSet::new();
    let mut axes: BTreeSet<M5ComparisonAxisClass> = BTreeSet::new();
    let mut states: BTreeSet<M5ComparabilityState> = BTreeSet::new();

    for table in &packet.comparison_tables {
        let disclosure = table.fairness_disclosure();
        fairness_classes.insert(disclosure.fairness_class);
        axes.insert(table.comparison_axis);
        states.insert(table.comparability_state);

        if table.table_id.trim().is_empty()
            || table.comparison_label.trim().is_empty()
            || table.fields_shown.is_empty()
            || table.surface_families.is_empty()
            || table.deployment_lines.is_empty()
            || table.consumer_surfaces.is_empty()
            || table.source_contract_refs.is_empty()
        {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::ComparisonTableIncomplete);
        }
        if table.component != M5ExperimentComponentFamily::RunComparisonTable {
            violations.push(
                RunComparisonTableCompareGuardBannerViolation::ComparisonTableWrongComponentClass,
            );
        }
        if table.fairness_class != disclosure.fairness_class
            || table.claims_fair_baseline != disclosure.is_fair_baseline
        {
            violations.push(RunComparisonTableCompareGuardBannerViolation::FairnessMisrepresented);
        }
        if table.baseline_run_id.trim().is_empty()
            || table.baseline_run_label.trim().is_empty()
            || table.candidate_run_id.trim().is_empty()
            || table.candidate_run_label.trim().is_empty()
        {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::BaselineOrCandidateMissing);
        }
        if table.metric_value_note.trim().is_empty()
            || table.delta_note.trim().is_empty()
            || table.threshold_state_note.trim().is_empty()
            || table.confidence_note.trim().is_empty()
            || table.comparator_type_note.trim().is_empty()
        {
            violations.push(RunComparisonTableCompareGuardBannerViolation::MetricOrDeltaMissing);
        }
        if table.code_difference_note.trim().is_empty()
            || table.data_difference_note.trim().is_empty()
            || table.environment_difference_note.trim().is_empty()
            || table.hardware_difference_note.trim().is_empty()
        {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::DifferenceFactorsMissing);
        }
        if disclosure.needs_caveat_note && table.caveat_note.trim().is_empty() {
            violations.push(RunComparisonTableCompareGuardBannerViolation::CaveatNoteMissing);
        }
        if disclosure.needs_not_comparable_note && table.not_comparable_note.trim().is_empty() {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::NotComparableNoteMissing);
        }
        if disclosure.needs_confounder_note && table.confounder_note.trim().is_empty() {
            violations.push(RunComparisonTableCompareGuardBannerViolation::ConfounderNoteMissing);
        }
        if disclosure.needs_insufficient_overlap_note
            && table.insufficient_overlap_note.trim().is_empty()
        {
            violations.push(
                RunComparisonTableCompareGuardBannerViolation::InsufficientOverlapNoteMissing,
            );
        }
        if disclosure.needs_unknown_comparability_note
            && table.unknown_comparability_note.trim().is_empty()
        {
            violations.push(
                RunComparisonTableCompareGuardBannerViolation::UnknownComparabilityNoteMissing,
            );
        }
        if table.comparability_and_parity_note.trim().is_empty() {
            violations.push(
                RunComparisonTableCompareGuardBannerViolation::ComparabilityAndParityNoteMissing,
            );
        }
        if !table.declares_mandatory_actions() {
            violations.push(
                RunComparisonTableCompareGuardBannerViolation::ComparisonTableActionsIncomplete,
            );
        }
        validate_deep_link(
            table.offers_deep_link_action(),
            table.deep_link_kind,
            &table.deep_link_ref,
            &table.context_note,
            violations,
        );
        validate_common_control(
            &table.dispositions,
            &table.downgrade_triggers,
            table.declares_mandatory_labels(),
            &table.accessibility_routes,
            ControlInvariants {
                masks_provenance_or_sensitivity_state: table.masks_provenance_or_sensitivity_state,
                hides_baseline_or_candidate_identity: table.hides_baseline_or_candidate_identity,
                hides_difference_factors_beside_delta: table.hides_difference_factors_beside_delta,
                implies_apples_to_apples_without_parity: table
                    .implies_apples_to_apples_without_parity,
                invents_alternate_state_label: table.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in ComparisonFairnessClass::ALL {
        if !fairness_classes.contains(&required) {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::FairnessClassCoverageMissing);
            break;
        }
    }
    for required in M5ComparisonAxisClass::ALL {
        if !axes.contains(&required) {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::ComparisonAxisCoverageMissing);
            break;
        }
    }
    for required in M5ComparabilityState::ALL {
        if !states.contains(&required) {
            violations.push(
                RunComparisonTableCompareGuardBannerViolation::ComparabilityStateCoverageMissing,
            );
            break;
        }
    }
}

fn validate_guard_banners(
    packet: &RunComparisonTableCompareGuardBannerControlsPacket,
    violations: &mut Vec<RunComparisonTableCompareGuardBannerViolation>,
) {
    if packet.guard_banners.is_empty() {
        violations.push(RunComparisonTableCompareGuardBannerViolation::GuardBannersMissing);
        return;
    }

    let mut guard_classes: BTreeSet<GuardComparabilityClass> = BTreeSet::new();
    let mut reasons: BTreeSet<M5CompareGuardReason> = BTreeSet::new();
    let mut states: BTreeSet<M5CompareGuardState> = BTreeSet::new();

    for banner in &packet.guard_banners {
        let disclosure = banner.guard_disclosure();
        guard_classes.insert(disclosure.guard_class);
        reasons.insert(banner.guard_reason);
        states.insert(banner.guard_state);

        if banner.banner_id.trim().is_empty()
            || banner.banner_label.trim().is_empty()
            || banner.fields_shown.is_empty()
            || banner.surface_families.is_empty()
            || banner.deployment_lines.is_empty()
            || banner.consumer_surfaces.is_empty()
            || banner.source_contract_refs.is_empty()
        {
            violations.push(RunComparisonTableCompareGuardBannerViolation::GuardBannerIncomplete);
        }
        if banner.component != M5ExperimentComponentFamily::CompareGuardBanner {
            violations.push(
                RunComparisonTableCompareGuardBannerViolation::GuardBannerWrongComponentClass,
            );
        }
        if banner.guard_class != disclosure.guard_class
            || banner.claims_permits_fair_comparison != disclosure.permits_fair_comparison
        {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::GuardClassMisrepresented);
        }
        if banner.comparability_disclosure_note.trim().is_empty() {
            violations.push(
                RunComparisonTableCompareGuardBannerViolation::ComparabilityDisclosureMissing,
            );
        }
        if banner.missing_lineage_fields_note.trim().is_empty() {
            violations.push(
                RunComparisonTableCompareGuardBannerViolation::MissingLineageFieldsNoteMissing,
            );
        }
        if banner.changed_factors_note.trim().is_empty() {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::ChangedFactorsNoteMissing);
        }
        if banner.redaction_note.trim().is_empty() {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::GuardRedactionNoteMissing);
        }
        if banner.guard_reason_note.trim().is_empty() {
            violations.push(RunComparisonTableCompareGuardBannerViolation::GuardReasonNoteMissing);
        }
        if disclosure.needs_partial_comparability_note
            && banner.partial_comparability_note.trim().is_empty()
        {
            violations.push(
                RunComparisonTableCompareGuardBannerViolation::PartialComparabilityNoteMissing,
            );
        }
        if disclosure.needs_override_warning && banner.override_warning.trim().is_empty() {
            violations.push(RunComparisonTableCompareGuardBannerViolation::OverrideWarningMissing);
        }
        if disclosure.needs_blocked_note && banner.blocked_note.trim().is_empty() {
            violations.push(RunComparisonTableCompareGuardBannerViolation::BlockedNoteMissing);
        }
        if disclosure.needs_guard_unavailable_note
            && banner.guard_unavailable_note.trim().is_empty()
        {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::GuardUnavailableNoteMissing);
        }
        if banner.comparability_and_parity_note.trim().is_empty() {
            violations.push(
                RunComparisonTableCompareGuardBannerViolation::ComparabilityAndParityNoteMissing,
            );
        }
        if !banner.declares_mandatory_actions() {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::GuardBannerActionsIncomplete);
        }
        validate_deep_link(
            banner.offers_deep_link_action(),
            banner.deep_link_kind,
            &banner.deep_link_ref,
            &banner.context_note,
            violations,
        );
        validate_common_control(
            &banner.dispositions,
            &banner.downgrade_triggers,
            banner.declares_mandatory_labels(),
            &banner.accessibility_routes,
            ControlInvariants {
                masks_provenance_or_sensitivity_state: banner.masks_provenance_or_sensitivity_state,
                hides_baseline_or_candidate_identity: banner.hides_baseline_or_candidate_identity,
                hides_difference_factors_beside_delta: banner.hides_difference_factors_beside_delta,
                implies_apples_to_apples_without_parity: banner
                    .implies_apples_to_apples_without_parity,
                invents_alternate_state_label: banner.invents_alternate_state_label,
            },
            violations,
        );
    }

    for required in GuardComparabilityClass::ALL {
        if !guard_classes.contains(&required) {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::GuardClassCoverageMissing);
            break;
        }
    }
    for required in M5CompareGuardReason::ALL {
        if !reasons.contains(&required) {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::GuardReasonCoverageMissing);
            break;
        }
    }
    for required in M5CompareGuardState::ALL {
        if !states.contains(&required) {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::GuardStateCoverageMissing);
            break;
        }
    }
}

/// Validates that the four reproducibility trust labels are used consistently — the acceptance
/// criterion that `reproducible`, `likely_reproducible`, `needs_rerun`, and `context_incomplete`
/// remain controlled trust labels across compare surfaces, exports, and support evidence.
fn validate_trust_label_coverage(
    packet: &RunComparisonTableCompareGuardBannerControlsPacket,
    violations: &mut Vec<RunComparisonTableCompareGuardBannerViolation>,
) {
    let mut labels: BTreeSet<M5ExperimentDisposition> = BTreeSet::new();
    for table in &packet.comparison_tables {
        labels.extend(table.dispositions.iter().copied());
    }
    for banner in &packet.guard_banners {
        labels.extend(banner.dispositions.iter().copied());
    }
    for required in [
        M5ExperimentDisposition::Reproducible,
        M5ExperimentDisposition::LikelyReproducible,
        M5ExperimentDisposition::NeedsRerun,
        M5ExperimentDisposition::ContextIncomplete,
    ] {
        if !labels.contains(&required) {
            violations
                .push(RunComparisonTableCompareGuardBannerViolation::TrustLabelCoverageMissing);
            return;
        }
    }
}

/// Validates the context and stable deep-link truth shared by both component vectors.
///
/// A component that offers a deep-link action must name a resolvable deep-link kind, a component
/// that names a resolvable kind must carry its stable reference, and every component must name its
/// context — so a next step is never an ephemeral overlay or hidden route.
fn validate_deep_link(
    offers_deep_link_action: bool,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    context_note: &str,
    violations: &mut Vec<RunComparisonTableCompareGuardBannerViolation>,
) {
    if context_note.trim().is_empty() {
        violations.push(RunComparisonTableCompareGuardBannerViolation::ContextNoteMissing);
    }
    if offers_deep_link_action && !deep_link_kind.is_resolvable() {
        violations.push(RunComparisonTableCompareGuardBannerViolation::DeepLinkUnresolved);
    }
    if deep_link_kind.is_resolvable() && deep_link_ref.trim().is_empty() {
        violations.push(RunComparisonTableCompareGuardBannerViolation::DeepLinkRefMissing);
    }
}

/// The five hard-invariant bools every component must keep `false`.
struct ControlInvariants {
    masks_provenance_or_sensitivity_state: bool,
    hides_baseline_or_candidate_identity: bool,
    hides_difference_factors_beside_delta: bool,
    implies_apples_to_apples_without_parity: bool,
    invents_alternate_state_label: bool,
}

/// Validates the axes shared by both component vectors.
fn validate_common_control(
    dispositions: &[M5ExperimentDisposition],
    downgrade_triggers: &[M5ExperimentDowngradeTrigger],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5ExperimentAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<RunComparisonTableCompareGuardBannerViolation>,
) {
    if dispositions.is_empty() {
        violations.push(RunComparisonTableCompareGuardBannerViolation::DispositionsMissing);
    }
    if downgrade_triggers.is_empty() {
        violations.push(RunComparisonTableCompareGuardBannerViolation::DowngradeTriggersMissing);
    }
    if !declares_mandatory_labels {
        violations.push(RunComparisonTableCompareGuardBannerViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5ExperimentAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(RunComparisonTableCompareGuardBannerViolation::AccessibilityRouteMissing);
    }
    if invariants.masks_provenance_or_sensitivity_state {
        violations.push(
            RunComparisonTableCompareGuardBannerViolation::ProvenanceOrSensitivityStateMasked,
        );
    }
    if invariants.hides_baseline_or_candidate_identity {
        violations
            .push(RunComparisonTableCompareGuardBannerViolation::BaselineOrCandidateIdentityHidden);
    }
    if invariants.hides_difference_factors_beside_delta {
        violations.push(RunComparisonTableCompareGuardBannerViolation::DifferenceFactorsHidden);
    }
    if invariants.implies_apples_to_apples_without_parity {
        violations.push(
            RunComparisonTableCompareGuardBannerViolation::ApplesToApplesImpliedWithoutParity,
        );
    }
    if invariants.invents_alternate_state_label {
        violations.push(RunComparisonTableCompareGuardBannerViolation::AlternateStateLabelInvented);
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
