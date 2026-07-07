//! One reusable M5 test-explorer primitive — the test-tree row — so a user can tell,
//! from the row alone, exactly what will rerun or debug and with what certainty, before
//! any action leaves the tree.
//!
//! Aureline's frozen test-explorer / watch / triage component matrix
//! ([`crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix`])
//! names the test-tree row as one governed component family and freezes its controlled
//! vocabulary — the test identity classes, the imported/live result origins, the result
//! freshness states, the current-state verdicts, the test target classes and environment
//! lanes, the quarantine ownership classes and release impacts, plus the surface
//! families, the deployment lines, the consumer surfaces, the accessibility routes, the
//! qualification classes, and the downgrade triggers. This module *implements* that
//! contract as one reusable resolver so a user can tell — from the tree row alone —
//! which item class the row represents (a suite, a parameterized template, a concrete
//! case, a notebook-backed item, an imported result, or a not-yet-discovered
//! placeholder), its stable item identity, its current state and last-result freshness,
//! its imported-versus-live result origin, its target/environment shorthand, its
//! parameterized-case count, and its mute/quarantine state and release impact — and,
//! above all, exactly what selection will rerun or debug, without ever letting an
//! imported or partial-discovery item inherit the same visual certainty as a current
//! live result.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_test_tree_row`] — takes one item's tree item class, identity class,
//!    result origin, result freshness, current verdict, target class, environment lane,
//!    quarantine ownership and release impact, parameterized-case count, mute flag,
//!    opaque item label, and opaque stable item identity, and produces one
//!    [`M5ResolvedTestTreeRow`] carrying the derived row posture (a quarantined,
//!    partial-discovery, imported-evidence, stale-result, suite-aggregate, or
//!    live-concrete row), the exact rerun scope the row will run, whether the row can be
//!    rerun or debugged, whether it carries reduced (imported/partial) certainty rather
//!    than live certainty, and the bounded reveal-identity / rerun / debug /
//!    review-quarantine / export actions. It never masks the item's identity class or
//!    imported/live origin, never hides a quarantine's release impact, never overstates
//!    imported or partial-discovery certainty as live, and never silently widens the
//!    rerun scope.
//!
//! A single parity matrix — [`M5TestTreeRowPacket`] — binds one row per claimed M5 test
//! surface consumer (the test-explorer tree, the editor-gutter tree, the run-panel tree,
//! the headless/CLI tree, and the test-report export) to the shared tree-row anatomy, the
//! same item classes, identity classes, result origins, freshness states, row postures,
//! rerun scopes, bounded actions, export fields, and non-visual accessibility routes, so
//! the identity / origin / freshness / rerun-scope / quarantine vocabulary stays identical
//! across desktop, headless/export, and report consumers.
//!
//! The test identity class ([`M5TestIdentityClass`]), result origin
//! ([`M5TestResultOrigin`]), result freshness ([`M5TestResultFreshness`]), current
//! verdict ([`M5InlineMarkerVerdict`]), target class ([`M5TestTargetClass`]), environment
//! lane ([`M5TestEnvironmentLane`]), quarantine ownership ([`M5QuarantineOwnership`]),
//! release impact ([`M5TestReleaseImpact`]), surface family ([`M5TestSurfaceFamily`]),
//! deployment line ([`M5TestDeploymentLine`]), consumer surface
//! ([`M5TestConsumerSurface`]), accessibility route ([`M5TestAccessibilityRoute`]),
//! qualification class ([`M5TestQualificationClass`]), and downgrade trigger
//! ([`M5TestDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the tree row
//! itself: the tree item class, its test-surface consumers, its anatomy parts, its
//! derived row posture, its rerun scope, its bounded actions, and its export fields. No
//! M5 test surface invents a second tree-row grammar.
//!
//! Raw log bodies, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every item label and item identity is carried only as an opaque,
//! export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_test_tree_row_headless_cli_tree_beta_narrowed, seeded_m5_test_tree_row_packet,
    seeded_m5_test_tree_row_run_panel_tree_preview_narrowed, M5_TEST_TREE_ROW_PACKET_ID,
};

// The test identity class, result origin, freshness, verdict, target class, environment
// lane, quarantine ownership, release impact, surface family, deployment line, consumer
// surface, accessibility route, qualification class, and downgrade triggers are frozen
// once, in the test-explorer / watch / triage component matrix. This primitive reuses
// them verbatim so it never invents a parallel test-tree vocabulary.
pub use crate::freeze_the_m5_test_tree_row_inline_result_marker_session_summary_bar_watch_mode_banner_failure_triage_panel_quarantine_review_sheet_and_environment_matrix_card_component_matrix::{
    M5InlineMarkerVerdict, M5QuarantineOwnership, M5TestAccessibilityRoute, M5TestConsumerSurface,
    M5TestDeploymentLine, M5TestDowngradeTrigger, M5TestEnvironmentLane, M5TestIdentityClass,
    M5TestQualificationClass, M5TestReleaseImpact, M5TestResultFreshness, M5TestResultOrigin,
    M5TestSurfaceFamily, M5TestTargetClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5TestTreeRowPacket`].
pub const M5_TEST_TREE_ROW_RECORD_KIND: &str =
    "implement_m5_test_tree_rows_with_suite_template_case_notebook_imported_result_distinction_parameterized_counts_freshness_target_chip_and_mute_quarantine_truth_across_claimed_m5_test_surfaces";

/// Schema version for M5 test-tree-row records.
pub const M5_TEST_TREE_ROW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the tree-row boundary schema.
pub const M5_TEST_TREE_ROW_SCHEMA_REF: &str = "schemas/ui/m5-test-tree-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_TEST_TREE_ROW_DOC_REF: &str = "docs/testing/m5_test_tree_row_primitive.md";

/// Repo-relative path of the frozen test-explorer / watch / triage component matrix this
/// primitive narrows from.
pub const M5_TEST_TREE_ROW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-test-explorer-watch-triage-component-matrix.schema.json";

/// Repo-relative path of the test-item-identity contract this primitive binds its
/// identity / origin truth against.
pub const M5_TEST_TREE_ROW_TEST_ITEM_IDENTITY_REF: &str =
    "schemas/testing/test_item_identity.schema.json";

/// Repo-relative path of the quarantine-record contract this primitive binds its
/// mute/quarantine truth against.
pub const M5_TEST_TREE_ROW_QUARANTINE_RECORD_REF: &str =
    "schemas/testing/test_quarantine_record.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_TEST_TREE_ROW_FIXTURE_DIR: &str = "fixtures/ui/m5-test-tree-row-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TEST_TREE_ROW_ARTIFACT_REF: &str =
    "artifacts/release/m5-test-tree-row-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_TEST_TREE_ROW_CSV_REF: &str =
    "artifacts/release/m5-test-tree-row-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_TEST_TREE_ROW_REPORT_REF: &str = "artifacts/design/m5-test-tree-row-primitive.md";

/// Controlled tree item class — what kind of test object a tree row represents, so a row
/// never leaves its class implicit and a suite, a parameterized template, a concrete
/// case, a notebook-backed item, an imported result, and a not-yet-discovered placeholder
/// are never collapsed into one undifferentiated node. This is the distinction the
/// acceptance criteria require the row to make explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestTreeItemClass {
    /// A suite / container that fans out to child cases when rerun.
    Suite,
    /// A parameterized test template that fans out to its parameter variants.
    Template,
    /// A single concrete, runnable test case.
    ConcreteCase,
    /// A test backed by a notebook cell / document.
    NotebookBackedItem,
    /// An imported result row that references an external run.
    ImportedResult,
    /// A partial-discovery placeholder not yet resolved to a concrete case.
    PartialDiscoveryPlaceholder,
}

impl M5TestTreeItemClass {
    /// Every tree item class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Suite,
        Self::Template,
        Self::ConcreteCase,
        Self::NotebookBackedItem,
        Self::ImportedResult,
        Self::PartialDiscoveryPlaceholder,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Suite => "suite",
            Self::Template => "template",
            Self::ConcreteCase => "concrete_case",
            Self::NotebookBackedItem => "notebook_backed_item",
            Self::ImportedResult => "imported_result",
            Self::PartialDiscoveryPlaceholder => "partial_discovery_placeholder",
        }
    }

    /// True when this item class is a directly runnable / debuggable concrete leaf.
    pub const fn is_concrete_leaf(self) -> bool {
        matches!(self, Self::ConcreteCase | Self::NotebookBackedItem)
    }
}

/// One claimed M5 test-surface consumer that renders the shared test-tree row. These are
/// the consumers the acceptance criteria name — the test-explorer tree, the editor-gutter
/// tree, the run-panel tree, the headless/CLI tree, and the test-report export — so the
/// same tree-row grammar works across every claimed test surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestTreeConsumerSurface {
    /// The test-explorer tree surface.
    TestExplorerTree,
    /// The editor-gutter tree surface.
    EditorGutterTree,
    /// The run-panel tree surface.
    RunPanelTree,
    /// The headless / CLI tree surface.
    HeadlessCliTree,
    /// The test-report export surface.
    TestReportExport,
}

impl M5TestTreeConsumerSurface {
    /// Every claimed test-surface consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TestExplorerTree,
        Self::EditorGutterTree,
        Self::RunPanelTree,
        Self::HeadlessCliTree,
        Self::TestReportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestExplorerTree => "test_explorer_tree",
            Self::EditorGutterTree => "editor_gutter_tree",
            Self::RunPanelTree => "run_panel_tree",
            Self::HeadlessCliTree => "headless_cli_tree",
            Self::TestReportExport => "test_report_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::TestExplorerTree => "Test Explorer Tree",
            Self::EditorGutterTree => "Editor Gutter Tree",
            Self::RunPanelTree => "Run Panel Tree",
            Self::HeadlessCliTree => "Headless / CLI Tree",
            Self::TestReportExport => "Test Report Export",
        }
    }
}

/// The derived posture of a test-tree row — the resolver's verdict about how much
/// certainty the row carries and what should draw attention. Computed in a fixed
/// honesty-first order, so a quarantined, partial-discovery, imported, or stale row never
/// reads with the same visual certainty as a current live result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestTreeRowPosture {
    /// The item is muted / quarantined; its ownership and release impact head the row.
    QuarantinedRow,
    /// The item is a partial-discovery placeholder or has an ambiguous / unknown identity;
    /// what will rerun is not yet certain.
    PartialDiscoveryRow,
    /// The item's result is imported from an external run, not produced live here; it
    /// carries reduced certainty.
    ImportedEvidenceRow,
    /// The item's live-local result is stale, outdated, or expired relative to its source.
    StaleResultRow,
    /// The item is a suite or parameterized template that fans out when rerun.
    SuiteAggregateRow,
    /// A concrete, live-local, fresh test case — the highest-certainty row.
    LiveConcreteRow,
}

impl M5TestTreeRowPosture {
    /// Every row posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::QuarantinedRow,
        Self::PartialDiscoveryRow,
        Self::ImportedEvidenceRow,
        Self::StaleResultRow,
        Self::SuiteAggregateRow,
        Self::LiveConcreteRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuarantinedRow => "quarantined_row",
            Self::PartialDiscoveryRow => "partial_discovery_row",
            Self::ImportedEvidenceRow => "imported_evidence_row",
            Self::StaleResultRow => "stale_result_row",
            Self::SuiteAggregateRow => "suite_aggregate_row",
            Self::LiveConcreteRow => "live_concrete_row",
        }
    }

    /// True only for a concrete, live-local, fresh row — the one posture that may present
    /// full live certainty. Imported and partial rows deliberately never qualify.
    pub const fn shows_live_certainty(self) -> bool {
        matches!(self, Self::LiveConcreteRow)
    }

    /// True when the row carries reduced (imported or partial-discovery) certainty and
    /// must therefore not read as a current live result.
    pub const fn carries_reduced_certainty(self) -> bool {
        matches!(self, Self::ImportedEvidenceRow | Self::PartialDiscoveryRow)
    }

    /// True when the row needs operator attention before it is trusted as green.
    pub const fn needs_attention(self) -> bool {
        !matches!(self, Self::LiveConcreteRow | Self::SuiteAggregateRow)
    }
}

/// Controlled rerun scope — exactly what selection a rerun / debug from this row will run,
/// so the acceptance-criterion promise that a user can tell what will rerun from the row
/// alone is explicit and never silently widened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestTreeRerunScope {
    /// Reruns the whole suite subtree.
    WholeSuite,
    /// Reruns every parameter variant of a template.
    ParameterizedGroup,
    /// Reruns exactly this one concrete case.
    SingleCase,
    /// Reruns the notebook-backed item's cells.
    NotebookCells,
    /// The imported result is not locally rerunnable; only replay / re-import applies.
    ImportedReplayOnly,
    /// Nothing concrete has been discovered yet to rerun.
    NothingConcreteYet,
}

impl M5TestTreeRerunScope {
    /// Every rerun scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WholeSuite,
        Self::ParameterizedGroup,
        Self::SingleCase,
        Self::NotebookCells,
        Self::ImportedReplayOnly,
        Self::NothingConcreteYet,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WholeSuite => "whole_suite",
            Self::ParameterizedGroup => "parameterized_group",
            Self::SingleCase => "single_case",
            Self::NotebookCells => "notebook_cells",
            Self::ImportedReplayOnly => "imported_replay_only",
            Self::NothingConcreteYet => "nothing_concrete_yet",
        }
    }

    /// True when this scope can be rerun locally (imported-replay and nothing-yet cannot).
    pub const fn is_locally_rerunnable(self) -> bool {
        !matches!(self, Self::ImportedReplayOnly | Self::NothingConcreteYet)
    }
}

/// One bounded action a test-tree row offers, so a row never hides its reveal-identity /
/// rerun / debug / review-quarantine / export affordances, and a user can act on the row
/// without leaving the tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestTreeRowAction {
    /// Reveal the item's identity class, origin, freshness, and target/environment.
    RevealItemIdentity,
    /// Rerun exactly the resolved rerun scope.
    RerunItem,
    /// Debug this concrete item.
    DebugItem,
    /// Review the item's mute / quarantine and its release impact.
    ReviewQuarantine,
    /// Export the tree row as test evidence.
    ExportRow,
}

impl M5TestTreeRowAction {
    /// Every tree-row action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealItemIdentity,
        Self::RerunItem,
        Self::DebugItem,
        Self::ReviewQuarantine,
        Self::ExportRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealItemIdentity => "reveal_item_identity",
            Self::RerunItem => "rerun_item",
            Self::DebugItem => "debug_item",
            Self::ReviewQuarantine => "review_quarantine",
            Self::ExportRow => "export_row",
        }
    }
}

/// Controlled test-tree-row anatomy part the shared row surfaces. The parts in
/// [`M5TestTreeRowAnatomyPart::MANDATORY`] are required on every row so the item class /
/// identity, current state, imported/live origin, target/environment shorthand,
/// mute/quarantine state, and the rerun/debug action cue are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestTreeRowAnatomyPart {
    /// The item class + stable identity cue.
    IdentityClassCue,
    /// The current-state (verdict) cue.
    CurrentStateCue,
    /// The last-result freshness cue.
    FreshnessCue,
    /// The imported/live result origin cue.
    OriginCue,
    /// The target / environment shorthand cue.
    TargetEnvironmentCue,
    /// The parameterized-case-count cue.
    ParameterizedCountCue,
    /// The mute / quarantine + release-impact cue.
    MuteQuarantineCue,
    /// The rerun / debug action cue (with its keyboard route).
    RerunDebugActionCue,
}

impl M5TestTreeRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::IdentityClassCue,
        Self::CurrentStateCue,
        Self::FreshnessCue,
        Self::OriginCue,
        Self::TargetEnvironmentCue,
        Self::ParameterizedCountCue,
        Self::MuteQuarantineCue,
        Self::RerunDebugActionCue,
    ];

    /// The anatomy parts every row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::IdentityClassCue,
        Self::CurrentStateCue,
        Self::OriginCue,
        Self::MuteQuarantineCue,
        Self::RerunDebugActionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityClassCue => "identity_class_cue",
            Self::CurrentStateCue => "current_state_cue",
            Self::FreshnessCue => "freshness_cue",
            Self::OriginCue => "origin_cue",
            Self::TargetEnvironmentCue => "target_environment_cue",
            Self::ParameterizedCountCue => "parameterized_count_cue",
            Self::MuteQuarantineCue => "mute_quarantine_cue",
            Self::RerunDebugActionCue => "rerun_debug_action_cue",
        }
    }
}

/// A field the row export carries so test-tree-row truth is reconstructable. The fields in
/// [`M5TestTreeRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TestTreeRowExportField {
    /// The tree item class.
    ItemClass,
    /// The stable identity class.
    IdentityClass,
    /// The imported/live result origin.
    ResultOrigin,
    /// The last-result freshness.
    ResultFreshness,
    /// The derived row posture.
    RowPosture,
    /// The exact rerun scope.
    RerunScope,
    /// The mute / quarantine ownership and release impact.
    MuteQuarantineState,
    /// The bounded available actions.
    AvailableActions,
}

impl M5TestTreeRowExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ItemClass,
        Self::IdentityClass,
        Self::ResultOrigin,
        Self::ResultFreshness,
        Self::RowPosture,
        Self::RerunScope,
        Self::MuteQuarantineState,
        Self::AvailableActions,
    ];

    /// The export fields every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ItemClass,
        Self::IdentityClass,
        Self::ResultOrigin,
        Self::RowPosture,
        Self::RerunScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ItemClass => "item_class",
            Self::IdentityClass => "identity_class",
            Self::ResultOrigin => "result_origin",
            Self::ResultFreshness => "result_freshness",
            Self::RowPosture => "row_posture",
            Self::RerunScope => "rerun_scope",
            Self::MuteQuarantineState => "mute_quarantine_state",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// True when a result origin is imported from an external run rather than produced live
/// on this host.
pub const fn result_origin_is_imported(origin: M5TestResultOrigin) -> bool {
    matches!(
        origin,
        M5TestResultOrigin::ImportedCi
            | M5TestResultOrigin::ImportedTeammate
            | M5TestResultOrigin::ReplayedSnapshot
    )
}

/// True when a result origin cannot be attributed and must be treated as uncertain.
pub const fn result_origin_is_unattributed(origin: M5TestResultOrigin) -> bool {
    matches!(origin, M5TestResultOrigin::UnknownOrigin)
}

/// True when a freshness state means the result no longer matches its current source.
pub const fn freshness_is_stale(freshness: M5TestResultFreshness) -> bool {
    matches!(
        freshness,
        M5TestResultFreshness::Stale
            | M5TestResultFreshness::OutdatedSource
            | M5TestResultFreshness::Expired
    )
}

// ---- test-tree-row resolver ---------------------------------------------

/// The full input to the test-tree-row resolver for one item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestTreeRowResolutionInput {
    /// The tree item class.
    pub item_class: M5TestTreeItemClass,
    /// The stable identity class.
    pub identity_class: M5TestIdentityClass,
    /// The imported/live result origin.
    pub result_origin: M5TestResultOrigin,
    /// The last-result freshness.
    pub result_freshness: M5TestResultFreshness,
    /// The current-state verdict.
    pub current_verdict: M5InlineMarkerVerdict,
    /// The test target class (target chip).
    pub target_class: M5TestTargetClass,
    /// The test environment lane (environment chip).
    pub environment_lane: M5TestEnvironmentLane,
    /// The quarantine ownership behind the mute/quarantine state.
    pub quarantine_ownership: M5QuarantineOwnership,
    /// The release impact of the mute/quarantine.
    pub release_impact: M5TestReleaseImpact,
    /// The number of parameterized cases behind this item (0 when not parameterized).
    pub parameterized_case_count: u32,
    /// True when the item is muted / quarantined.
    pub item_muted: bool,
    /// The opaque user-facing item label (must be non-empty).
    pub item_label: String,
    /// The opaque stable item identity (must be non-empty).
    pub item_identity_ref: String,
}

/// The resolved test-tree-row truth for one item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTestTreeRow {
    /// The tree item class.
    pub item_class: M5TestTreeItemClass,
    /// The stable identity class.
    pub identity_class: M5TestIdentityClass,
    /// The imported/live result origin.
    pub result_origin: M5TestResultOrigin,
    /// The last-result freshness.
    pub result_freshness: M5TestResultFreshness,
    /// The current-state verdict.
    pub current_verdict: M5InlineMarkerVerdict,
    /// The test target class.
    pub target_class: M5TestTargetClass,
    /// The test environment lane.
    pub environment_lane: M5TestEnvironmentLane,
    /// The quarantine ownership.
    pub quarantine_ownership: M5QuarantineOwnership,
    /// The release impact.
    pub release_impact: M5TestReleaseImpact,
    /// The number of parameterized cases behind this item, preserved from the input.
    pub parameterized_case_count: u32,
    /// The opaque item label, preserved exactly from the input.
    pub item_label: String,
    /// The opaque stable item identity, preserved exactly from the input.
    pub item_identity_ref: String,
    /// The derived row posture.
    pub row_posture: M5TestTreeRowPosture,
    /// The exact rerun scope this row will run.
    pub rerun_scope: M5TestTreeRerunScope,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5TestTreeRowAction>,
    /// True when the row can be rerun locally.
    pub can_rerun: bool,
    /// True when the row can be debugged (a concrete, locally-runnable leaf).
    pub can_debug: bool,
    /// True when the item is muted / quarantined.
    pub is_muted: bool,
    /// True only for a concrete, live-local, fresh row — never for imported or partial
    /// rows, so imported / partial-discovery items never inherit live certainty.
    pub shows_live_certainty: bool,
    /// True when the row carries reduced (imported / partial-discovery) certainty.
    pub carries_reduced_certainty: bool,
    /// True when the row needs operator attention before it is trusted as green.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_test_tree_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5TestTreeRowResolutionError {
    /// The item label was empty.
    EmptyItemLabel,
    /// The item identity ref was empty.
    EmptyItemIdentity,
    /// A row descriptor carried forbidden material.
    ForbiddenTreeMaterial,
}

impl M5TestTreeRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyItemLabel => "empty_item_label",
            Self::EmptyItemIdentity => "empty_item_identity",
            Self::ForbiddenTreeMaterial => "forbidden_tree_material",
        }
    }
}

impl fmt::Display for M5TestTreeRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "test tree row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5TestTreeRowResolutionError {}

/// Resolves one test-tree row from its declared item state.
///
/// The derived row posture is computed in a fixed honesty-first order: a muted /
/// quarantined item wins first (its ownership and release impact head the row), then a
/// partial-discovery placeholder or ambiguous / unattributed identity (what will rerun is
/// not certain), then an imported result (reduced certainty, not a live-local result),
/// then a stale / outdated / expired live result, then a suite or parameterized template
/// that fans out, and otherwise a concrete, live-local, fresh row. The rerun scope is
/// derived directly from the item class so a rerun / debug never silently widens beyond
/// what the row names; the item class, identity class, result origin, freshness, target,
/// environment, and quarantine state are carried explicitly, never inferred away; the row
/// always offers reveal-identity and export, offers rerun only when the scope is locally
/// rerunnable, offers debug only for a concrete runnable leaf, and offers review-quarantine
/// only when the item is muted — so a user can tell exactly what will rerun or debug, and
/// with what certainty, from the row alone.
pub fn resolve_test_tree_row(
    input: &M5TestTreeRowResolutionInput,
) -> Result<M5ResolvedTestTreeRow, M5TestTreeRowResolutionError> {
    if input.item_label.trim().is_empty() {
        return Err(M5TestTreeRowResolutionError::EmptyItemLabel);
    }
    if input.item_identity_ref.trim().is_empty() {
        return Err(M5TestTreeRowResolutionError::EmptyItemIdentity);
    }
    if value_repr_is_forbidden(&input.item_label)
        || value_repr_is_forbidden(&input.item_identity_ref)
    {
        return Err(M5TestTreeRowResolutionError::ForbiddenTreeMaterial);
    }

    let row_posture = derive_row_posture(
        input.item_class,
        input.identity_class,
        input.result_origin,
        input.result_freshness,
        input.item_muted,
    );
    let rerun_scope = derive_rerun_scope(input.item_class);
    let can_rerun = rerun_scope.is_locally_rerunnable();
    let can_debug = can_rerun && input.item_class.is_concrete_leaf();
    let available_actions = derive_tree_actions(can_rerun, can_debug, input.item_muted);

    Ok(M5ResolvedTestTreeRow {
        item_class: input.item_class,
        identity_class: input.identity_class,
        result_origin: input.result_origin,
        result_freshness: input.result_freshness,
        current_verdict: input.current_verdict,
        target_class: input.target_class,
        environment_lane: input.environment_lane,
        quarantine_ownership: input.quarantine_ownership,
        release_impact: input.release_impact,
        parameterized_case_count: input.parameterized_case_count,
        item_label: input.item_label.clone(),
        item_identity_ref: input.item_identity_ref.clone(),
        row_posture,
        rerun_scope,
        available_actions,
        can_rerun,
        can_debug,
        is_muted: input.item_muted,
        shows_live_certainty: row_posture.shows_live_certainty(),
        carries_reduced_certainty: row_posture.carries_reduced_certainty(),
        needs_attention: row_posture.needs_attention(),
    })
}

/// The fixed honesty-first row-posture ladder.
fn derive_row_posture(
    item_class: M5TestTreeItemClass,
    identity_class: M5TestIdentityClass,
    result_origin: M5TestResultOrigin,
    result_freshness: M5TestResultFreshness,
    item_muted: bool,
) -> M5TestTreeRowPosture {
    use M5TestTreeItemClass as Item;
    use M5TestTreeRowPosture as Posture;
    if item_muted {
        Posture::QuarantinedRow
    } else if matches!(item_class, Item::PartialDiscoveryPlaceholder)
        || matches!(identity_class, M5TestIdentityClass::AmbiguousIdentity)
        || result_origin_is_unattributed(result_origin)
    {
        Posture::PartialDiscoveryRow
    } else if matches!(item_class, Item::ImportedResult) || result_origin_is_imported(result_origin)
    {
        Posture::ImportedEvidenceRow
    } else if freshness_is_stale(result_freshness) {
        Posture::StaleResultRow
    } else if matches!(item_class, Item::Suite | Item::Template) {
        Posture::SuiteAggregateRow
    } else {
        Posture::LiveConcreteRow
    }
}

/// Derives the exact rerun scope from the item class, so a rerun / debug never widens
/// beyond what the row names.
fn derive_rerun_scope(item_class: M5TestTreeItemClass) -> M5TestTreeRerunScope {
    use M5TestTreeItemClass as Item;
    use M5TestTreeRerunScope as Scope;
    match item_class {
        Item::Suite => Scope::WholeSuite,
        Item::Template => Scope::ParameterizedGroup,
        Item::ConcreteCase => Scope::SingleCase,
        Item::NotebookBackedItem => Scope::NotebookCells,
        Item::ImportedResult => Scope::ImportedReplayOnly,
        Item::PartialDiscoveryPlaceholder => Scope::NothingConcreteYet,
    }
}

/// Derives the bounded action set from the rerun / debug / mute signals.
///
/// Reveal-identity is always offered so the item class, origin, freshness, and
/// target/environment are always inspectable; rerun is offered only when the scope is
/// locally rerunnable; debug is offered only for a concrete runnable leaf; review-quarantine
/// is offered only when the item is muted; export-row is always offered.
fn derive_tree_actions(
    can_rerun: bool,
    can_debug: bool,
    item_muted: bool,
) -> Vec<M5TestTreeRowAction> {
    use M5TestTreeRowAction as Action;
    let mut actions = vec![Action::RevealItemIdentity];
    if can_rerun {
        actions.push(Action::RerunItem);
    }
    if can_debug {
        actions.push(Action::DebugItem);
    }
    if item_muted {
        actions.push(Action::ReviewQuarantine);
    }
    actions.push(Action::ExportRow);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked test-tree-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestTreeRowResolutionCase {
    /// The resolver input.
    pub input: M5TestTreeRowResolutionInput,
    /// The resolved truth. Must equal `resolve_test_tree_row(&input)`.
    pub resolved: M5ResolvedTestTreeRow,
}

impl M5TestTreeRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5TestTreeRowResolutionInput) -> Self {
        let resolved = resolve_test_tree_row(&input).expect("seed tree row case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_test_tree_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved item identity preserves the input identity exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.item_identity_ref == self.input.item_identity_ref
            && self.resolved.item_label == self.input.item_label
    }
}

/// One row in the primitive matrix: one test-surface consumer bound to the shared tree-row
/// anatomy, item classes, identity classes, result origins, freshness states, row
/// postures, rerun scopes, bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestTreeConsumerRow {
    /// Test-surface consumer family.
    pub consumer_surface: M5TestTreeConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5TestQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 test surface families that render / consume this row.
    pub surface_families: Vec<M5TestSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5TestDeploymentLine>,
    /// Anatomy parts this row renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5TestTreeRowAnatomyPart>,
    /// Tree item classes this consumer distinguishes.
    pub item_classes: Vec<M5TestTreeItemClass>,
    /// Identity classes this consumer distinguishes.
    pub identity_classes: Vec<M5TestIdentityClass>,
    /// Result origins this consumer distinguishes.
    pub result_origins: Vec<M5TestResultOrigin>,
    /// Row postures this consumer distinguishes.
    pub row_postures: Vec<M5TestTreeRowPosture>,
    /// Rerun scopes this consumer distinguishes.
    pub rerun_scopes: Vec<M5TestTreeRerunScope>,
    /// Bounded tree-row actions this consumer offers.
    pub row_actions: Vec<M5TestTreeRowAction>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5TestTreeRowExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5TestAccessibilityRoute>,
    /// Test / triage subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5TestConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5TestDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked tree-row resolutions proving the resolver on this consumer.
    pub tree_examples: Vec<M5TestTreeRowResolutionCase>,
    /// Hard invariant: this consumer never masks its item identity class or imported/live
    /// origin. MUST be `false`.
    pub masks_identity_or_origin: bool,
    /// Hard invariant: this consumer never hides a quarantine's release impact. MUST be
    /// `false`.
    pub hides_quarantine_release_impact: bool,
    /// Hard invariant: this consumer never renders imported or partial-discovery items with
    /// live certainty. MUST be `false`.
    pub overstates_imported_certainty: bool,
    /// Hard invariant: this consumer never silently widens the rerun scope. MUST be
    /// `false`.
    pub widens_rerun_scope_silently: bool,
}

impl M5TestTreeConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5TestTreeRowAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5TestTreeRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5TestTreeRowExportField> =
            self.export_fields.iter().copied().collect();
        M5TestTreeRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_identity_or_origin
            && !self.hides_quarantine_release_impact
            && !self.overstates_imported_certainty
            && !self.widens_rerun_scope_silently
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestTreeRowVocabularySet {
    /// Test-surface-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Row-posture tokens.
    pub row_postures: Vec<String>,
    /// Rerun-scope tokens.
    pub rerun_scopes: Vec<String>,
    /// Row-action tokens.
    pub row_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Tree-item-class tokens.
    pub item_classes: Vec<String>,
    /// Identity-class tokens (reused from the frozen matrix).
    pub identity_classes: Vec<String>,
    /// Result-origin tokens (reused from the frozen matrix).
    pub result_origins: Vec<String>,
    /// Result-freshness tokens (reused from the frozen matrix).
    pub result_freshness: Vec<String>,
    /// Marker-verdict tokens (reused from the frozen matrix).
    pub marker_verdicts: Vec<String>,
    /// Target-class tokens (reused from the frozen matrix).
    pub target_classes: Vec<String>,
    /// Environment-lane tokens (reused from the frozen matrix).
    pub environment_lanes: Vec<String>,
    /// Quarantine-ownership tokens (reused from the frozen matrix).
    pub quarantine_ownership_classes: Vec<String>,
    /// Release-impact tokens (reused from the frozen matrix).
    pub release_impacts: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5TestTreeRowVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5TestTreeConsumerSurface::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5TestTreeRowAnatomyPart::ALL, |v| v.as_str()),
            row_postures: tokens(&M5TestTreeRowPosture::ALL, |v| v.as_str()),
            rerun_scopes: tokens(&M5TestTreeRerunScope::ALL, |v| v.as_str()),
            row_actions: tokens(&M5TestTreeRowAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5TestTreeRowExportField::ALL, |v| v.as_str()),
            item_classes: tokens(&M5TestTreeItemClass::ALL, |v| v.as_str()),
            identity_classes: tokens(&M5TestIdentityClass::ALL, |v| v.as_str()),
            result_origins: tokens(&M5TestResultOrigin::ALL, |v| v.as_str()),
            result_freshness: tokens(&M5TestResultFreshness::ALL, |v| v.as_str()),
            marker_verdicts: tokens(&M5InlineMarkerVerdict::ALL, |v| v.as_str()),
            target_classes: tokens(&M5TestTargetClass::ALL, |v| v.as_str()),
            environment_lanes: tokens(&M5TestEnvironmentLane::ALL, |v| v.as_str()),
            quarantine_ownership_classes: tokens(&M5QuarantineOwnership::ALL, |v| v.as_str()),
            release_impacts: tokens(&M5TestReleaseImpact::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TestSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TestDeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TestAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestTreeRowGovernanceReview {
    /// The tree row shows its item class and stable identity.
    pub tree_row_shows_item_class_and_identity: bool,
    /// The tree row shows its current state and last-result freshness.
    pub tree_row_shows_state_and_freshness: bool,
    /// The tree row shows its imported/live result origin.
    pub tree_row_shows_result_origin: bool,
    /// The tree row shows its target / environment shorthand.
    pub tree_row_shows_target_and_environment: bool,
    /// The tree row shows its parameterized-case count.
    pub tree_row_shows_parameterized_count: bool,
    /// The tree row shows its mute / quarantine state and release impact.
    pub tree_row_shows_mute_and_release_impact: bool,
    /// The row names the exact rerun scope and never silently widens it.
    pub rerun_scope_explicit_and_never_widened: bool,
    /// Imported and partial-discovery items never inherit live certainty.
    pub imported_or_partial_never_reads_as_live: bool,
    /// Tree rows keep the same truth across every deployment line.
    pub tree_rows_stable_across_deployment_lines: bool,
    /// Tree rows keep the same truth across desktop, headless/export, and report consumers.
    pub tree_rows_stable_across_consumer_surfaces: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The support / export packet reconstructs identity, origin, and rerun-scope truth.
    pub support_export_reconstructs_tree_truth: bool,
    /// Later M5 rows cannot invent parallel tree-row vocabulary.
    pub later_rows_cannot_invent_parallel_tree_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestTreeRowConsumerProjection {
    /// Test-explorer and editor surfaces consume the shared tree-row vocabulary.
    pub test_and_editor_surfaces_consume_tree_vocabulary: bool,
    /// The row-posture resolver reads a single canonical source.
    pub row_posture_reads_single_source: bool,
    /// The rerun-scope derivation reads a single canonical source.
    pub rerun_scope_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop trees read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestTreeRowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the tree row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestTreeRowReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting test-evidence audit.
    pub test_evidence_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5TestTreeRowPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TestTreeRowPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Test-surface rows.
    pub rows: Vec<M5TestTreeConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TestTreeRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TestTreeRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TestTreeRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TestTreeRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TestTreeRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 test-tree-row primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TestTreeRowPacket {
    /// Record kind; must equal [`M5_TEST_TREE_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TEST_TREE_ROW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Test-surface rows.
    pub rows: Vec<M5TestTreeConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TestTreeRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TestTreeRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TestTreeRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TestTreeRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TestTreeRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TestTreeRowPacket {
    /// Builds an M5 tree-row-primitive packet from stable-lane input.
    pub fn new(input: M5TestTreeRowPacketInput) -> Self {
        Self {
            record_kind: M5_TEST_TREE_ROW_RECORD_KIND.to_owned(),
            schema_version: M5_TEST_TREE_ROW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 tree-row-primitive invariants.
    pub fn validate(&self) -> Vec<M5TestTreeRowViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TEST_TREE_ROW_RECORD_KIND {
            violations.push(M5TestTreeRowViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TEST_TREE_ROW_SCHEMA_VERSION {
            violations.push(M5TestTreeRowViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TestTreeRowViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_item_class_coverage(self, &mut violations);
        validate_certainty_coverage(self, &mut violations);
        validate_rerun_coverage(self, &mut violations);
        validate_quarantine_coverage(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 tree row primitive packet serializes"),
        ) {
            violations.push(M5TestTreeRowViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 tree row primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per test-surface consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy,item_classes,result_origins,row_postures,rerun_scopes,row_actions,tree_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.item_classes, |v| v.as_str()),
                join_tokens(&row.result_origins, |v| v.as_str()),
                join_tokens(&row.row_postures, |v| v.as_str()),
                join_tokens(&row.rerun_scopes, |v| v.as_str()),
                join_tokens(&row.row_actions, |v| v.as_str()),
                row.tree_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Test-Tree-Row Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Test-surface consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Row postures: {}\n",
            self.vocabulary_set.row_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Rerun scopes: {}\n",
            self.vocabulary_set.rerun_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Item classes: {}\n",
            self.vocabulary_set.item_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Test-surface consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!("  - Worked rows: {}\n", row.tree_examples.len()));
            for case in &row.tree_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` (rerun `{}`, live-certainty `{}`, muted `{}`)\n",
                    case.resolved.item_identity_ref,
                    case.resolved.item_class.as_str(),
                    case.resolved.result_origin.as_str(),
                    case.resolved.row_posture.as_str(),
                    case.resolved.rerun_scope.as_str(),
                    case.resolved.shows_live_certainty,
                    case.resolved.is_muted,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 tree-row-primitive export.
#[derive(Debug)]
pub enum M5TestTreeRowArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TestTreeRowViolation>),
}

impl fmt::Display for M5TestTreeRowArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 tree row primitive export parse failed: {error}"
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
                    "m5 tree row primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TestTreeRowArtifactError {}

/// Validation failures emitted by [`M5TestTreeRowPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TestTreeRowViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required test-surface consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A test-surface row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked tree resolutions.
    TreeExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every tree item class.
    ItemClassCoverageUnproven,
    /// The worked resolutions do not prove both a live-certainty and an imported/partial
    /// reduced-certainty row.
    CertaintyCoverageUnproven,
    /// The worked resolutions do not prove both a locally-rerunnable and a
    /// not-locally-rerunnable row.
    RerunCoverageUnproven,
    /// The worked resolutions do not prove both a muted and a non-muted row.
    QuarantineCoverageUnproven,
    /// A worked resolution does not preserve its exact item identity and label.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5TestTreeRowViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::TreeExampleMissing => "tree_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ItemClassCoverageUnproven => "item_class_coverage_unproven",
            Self::CertaintyCoverageUnproven => "certainty_coverage_unproven",
            Self::RerunCoverageUnproven => "rerun_coverage_unproven",
            Self::QuarantineCoverageUnproven => "quarantine_coverage_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 tree-row-primitive export.
pub fn current_stable_m5_test_tree_row_export(
) -> Result<M5TestTreeRowPacket, M5TestTreeRowArtifactError> {
    let packet: M5TestTreeRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-test-tree-row-primitive-proof/support_export.json"
    )))
    .map_err(M5TestTreeRowArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TestTreeRowArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5TestTreeRowPacket,
    violations: &mut Vec<M5TestTreeRowViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TEST_TREE_ROW_SCHEMA_REF,
        M5_TEST_TREE_ROW_DOC_REF,
        M5_TEST_TREE_ROW_COMPONENT_MATRIX_REF,
        M5_TEST_TREE_ROW_TEST_ITEM_IDENTITY_REF,
        M5_TEST_TREE_ROW_QUARANTINE_RECORD_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TestTreeRowViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5TestTreeRowPacket,
    violations: &mut Vec<M5TestTreeRowViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5TestTreeRowViolation::VocabularySetDrift);
    }
}

fn validate_rows(packet: &M5TestTreeRowPacket, violations: &mut Vec<M5TestTreeRowViolation>) {
    let present: BTreeSet<M5TestTreeConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5TestTreeConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5TestTreeRowViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.item_classes.is_empty()
            || row.identity_classes.is_empty()
            || row.result_origins.is_empty()
            || row.row_postures.is_empty()
            || row.rerun_scopes.is_empty()
            || row.row_actions.is_empty()
            || row.export_fields.is_empty()
        {
            violations.push(M5TestTreeRowViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5TestTreeRowViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5TestTreeRowViolation::MandatoryExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TestAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5TestTreeRowViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5TestTreeRowViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5TestTreeRowViolation::DowngradeTriggersMissing);
        }
        if row.tree_examples.is_empty() {
            violations.push(M5TestTreeRowViolation::TreeExampleMissing);
        }
        if row
            .tree_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5TestTreeRowViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5TestTreeRowViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5TestTreeRowViolation::RowInvariantViolated);
        }
    }
}

/// Every tree item class must be exercised by some worked resolution — the implementation
/// requirement that rows distinguish suite, template, concrete case, notebook-backed item,
/// imported result, and not-yet-discovered placeholder.
fn validate_item_class_coverage(
    packet: &M5TestTreeRowPacket,
    violations: &mut Vec<M5TestTreeRowViolation>,
) {
    let exercised: BTreeSet<M5TestTreeItemClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.tree_examples.iter())
        .map(|case| case.resolved.item_class)
        .collect();
    let covered = M5TestTreeItemClass::ALL
        .iter()
        .all(|class| exercised.contains(class));
    if !covered {
        violations.push(M5TestTreeRowViolation::ItemClassCoverageUnproven);
    }
}

/// At least one worked resolution must prove a live-certainty row and at least one must
/// prove an imported / partial-discovery reduced-certainty row — the acceptance-criterion
/// example that imported or partial-discovery items no longer inherit the same visual
/// certainty as current live results.
fn validate_certainty_coverage(
    packet: &M5TestTreeRowPacket,
    violations: &mut Vec<M5TestTreeRowViolation>,
) {
    let has_live = packet.rows.iter().any(|row| {
        row.tree_examples.iter().any(|case| {
            case.resolved.shows_live_certainty && !case.resolved.carries_reduced_certainty
        })
    });
    let has_reduced = packet.rows.iter().any(|row| {
        row.tree_examples.iter().any(|case| {
            case.resolved.carries_reduced_certainty && !case.resolved.shows_live_certainty
        })
    });
    if !(has_live && has_reduced) {
        violations.push(M5TestTreeRowViolation::CertaintyCoverageUnproven);
    }
}

/// At least one worked resolution must prove a locally-rerunnable row (offering the rerun
/// action) and at least one must prove a not-locally-rerunnable row (imported-replay or
/// nothing-yet, withholding the rerun action) — the acceptance-criterion example that a
/// user can tell what will actually rerun from the row alone.
fn validate_rerun_coverage(
    packet: &M5TestTreeRowPacket,
    violations: &mut Vec<M5TestTreeRowViolation>,
) {
    let has_rerunnable = packet.rows.iter().any(|row| {
        row.tree_examples.iter().any(|case| {
            case.resolved.can_rerun
                && case
                    .resolved
                    .available_actions
                    .contains(&M5TestTreeRowAction::RerunItem)
        })
    });
    let has_not_rerunnable = packet.rows.iter().any(|row| {
        row.tree_examples.iter().any(|case| {
            !case.resolved.can_rerun
                && !case
                    .resolved
                    .available_actions
                    .contains(&M5TestTreeRowAction::RerunItem)
        })
    });
    if !(has_rerunnable && has_not_rerunnable) {
        violations.push(M5TestTreeRowViolation::RerunCoverageUnproven);
    }
}

/// At least one worked resolution must prove a muted row (offering review-quarantine) and
/// at least one must prove a non-muted row — the implementation requirement that
/// mute/quarantine state and its release impact are never left implicit.
fn validate_quarantine_coverage(
    packet: &M5TestTreeRowPacket,
    violations: &mut Vec<M5TestTreeRowViolation>,
) {
    let has_muted = packet.rows.iter().any(|row| {
        row.tree_examples.iter().any(|case| {
            case.resolved.is_muted
                && case
                    .resolved
                    .available_actions
                    .contains(&M5TestTreeRowAction::ReviewQuarantine)
        })
    });
    let has_unmuted = packet
        .rows
        .iter()
        .any(|row| row.tree_examples.iter().any(|case| !case.resolved.is_muted));
    if !(has_muted && has_unmuted) {
        violations.push(M5TestTreeRowViolation::QuarantineCoverageUnproven);
    }
}

/// Every worked resolution must preserve its exact item identity and label — the invariant
/// that the tree row never rewrites the user's test identity.
fn validate_identity_preservation(
    packet: &M5TestTreeRowPacket,
    violations: &mut Vec<M5TestTreeRowViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.tree_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5TestTreeRowViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5TestTreeRowPacket,
    violations: &mut Vec<M5TestTreeRowViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.tree_row_shows_item_class_and_identity,
        review.tree_row_shows_state_and_freshness,
        review.tree_row_shows_result_origin,
        review.tree_row_shows_target_and_environment,
        review.tree_row_shows_parameterized_count,
        review.tree_row_shows_mute_and_release_impact,
        review.rerun_scope_explicit_and_never_widened,
        review.imported_or_partial_never_reads_as_live,
        review.tree_rows_stable_across_deployment_lines,
        review.tree_rows_stable_across_consumer_surfaces,
        review.every_row_declares_accessibility_route,
        review.support_export_reconstructs_tree_truth,
        review.later_rows_cannot_invent_parallel_tree_vocabulary,
    ] {
        if !ok {
            violations.push(M5TestTreeRowViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TestTreeRowPacket,
    violations: &mut Vec<M5TestTreeRowViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.test_and_editor_surfaces_consume_tree_vocabulary,
        projection.row_posture_reads_single_source,
        projection.rerun_scope_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5TestTreeRowViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TestTreeRowPacket,
    violations: &mut Vec<M5TestTreeRowViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TestTreeRowViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5TestTreeRowPacket,
    violations: &mut Vec<M5TestTreeRowViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.test_evidence_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5TestTreeRowViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
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
