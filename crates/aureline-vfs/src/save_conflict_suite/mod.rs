//! Difficult filesystem, external-change, and save-conflict regression suite.
//!
//! This module is the canonical loader, validator, and reporter for the
//! regression suite that exercises the difficult filesystem-identity,
//! external-change, and save-conflict scenarios on the three claimed
//! desktop platform rows (`linux_desktop`, `macos_desktop`,
//! `windows_desktop`). Each fixture under
//! [`/fixtures/recovery/m3/save_conflict_suite/`](../../../../fixtures/recovery/m3/save_conflict_suite/)
//! is one [`SaveConflictSuiteCase`] bound to:
//!
//! - one closed [`ScenarioClass`] (`external_change`, `save_conflict`,
//!   `permission_loss`, `alias_drift`, `difficult_save_path`),
//! - one closed [`PlatformRowClass`] (`linux_desktop`, `macos_desktop`,
//!   `windows_desktop`),
//! - one anchor `filesystem_identity_beta_case` fixture from the
//!   [`crate::identity_beta`] corpus that proves the underlying identity
//!   contract,
//! - one [`ExpectedBehavior`] describing the compare-before-write outcome,
//!   the silent-overwrite ban, the canonical save target redirect, the
//!   bounded resolution actions, and any required save-target blockers,
//! - one [`RegressionOutcome`] (`pass`, `downgrade_required`,
//!   `blocked_until_fix`), and
//! - one closed [`DowngradeLabel`] drawn from the suite vocabulary so a
//!   failing row can be downgraded without inventing new vocabulary.
//!
//! Bound to the boundary schema at
//! [`/schemas/recovery/save_conflict_suite_beta.schema.json`](../../../../schemas/recovery/save_conflict_suite_beta.schema.json),
//! the reviewer matrix doc at
//! [`/docs/state/m3/save_conflict_beta_matrix.md`](../../../../docs/state/m3/save_conflict_beta_matrix.md),
//! and the baseline report at
//! [`/artifacts/support/m3/save_conflict_report.md`](../../../../artifacts/support/m3/save_conflict_report.md).

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::identity_beta::{
    BetaCompareOutcome, BetaResolutionAction, BetaSaveTargetReviewBlocker, DifficultyClass,
};

/// Stable record-kind tag for a regression-suite case record.
pub const SAVE_CONFLICT_SUITE_CASE_RECORD_KIND: &str = "save_conflict_suite_case_record";

/// Stable record-kind tag for the regression-suite report record.
pub const SAVE_CONFLICT_SUITE_REPORT_RECORD_KIND: &str = "save_conflict_suite_report_record";

/// Frozen schema version for the regression-suite records.
pub const SAVE_CONFLICT_SUITE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema this module mirrors.
pub const SAVE_CONFLICT_SUITE_SCHEMA_REF: &str =
    "schemas/recovery/save_conflict_suite_beta.schema.json";

/// Repo-relative path of the reviewer matrix doc.
pub const SAVE_CONFLICT_SUITE_MATRIX_DOC_REF: &str = "docs/state/m3/save_conflict_beta_matrix.md";

/// Repo-relative path of the baseline report artifact.
pub const SAVE_CONFLICT_SUITE_REPORT_REF: &str = "artifacts/support/m3/save_conflict_report.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const SAVE_CONFLICT_SUITE_CORPUS_DIR: &str = "fixtures/recovery/m3/save_conflict_suite";

/// Repo-relative path of the protected corpus manifest.
pub const SAVE_CONFLICT_SUITE_CORPUS_MANIFEST_REF: &str =
    "fixtures/recovery/m3/save_conflict_suite/manifest.yaml";

/// Repo-relative path of the filesystem-identity beta reviewer doc the
/// suite anchors against.
pub const SAVE_CONFLICT_SUITE_FILESYSTEM_IDENTITY_BETA_DOC_REF: &str =
    "docs/state/m3/filesystem_identity_beta.md";

/// Closed regression-scenario vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScenarioClass {
    /// Concurrent external write detected via compare-before-write.
    ExternalChange,
    /// Conflict that requires the user to choose between compare, merge,
    /// save-as, or cancel.
    SaveConflict,
    /// Write permission lost between open and save.
    PermissionLoss,
    /// Alias drift exercised through a non-direct path-truth class
    /// (symlink, case-only, unicode-normalization, etc.).
    AliasDrift,
    /// Difficult save path (bind-mount, container overlay, etc.).
    DifficultSavePath,
}

impl ScenarioClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExternalChange => "external_change",
            Self::SaveConflict => "save_conflict",
            Self::PermissionLoss => "permission_loss",
            Self::AliasDrift => "alias_drift",
            Self::DifficultSavePath => "difficult_save_path",
        }
    }
}

/// Closed list of scenarios the suite must cover.
pub const REQUIRED_SCENARIO_CLASSES: [ScenarioClass; 5] = [
    ScenarioClass::ExternalChange,
    ScenarioClass::SaveConflict,
    ScenarioClass::PermissionLoss,
    ScenarioClass::AliasDrift,
    ScenarioClass::DifficultSavePath,
];

/// Closed claimed-desktop platform-row vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformRowClass {
    LinuxDesktop,
    MacosDesktop,
    WindowsDesktop,
}

impl PlatformRowClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LinuxDesktop => "linux_desktop",
            Self::MacosDesktop => "macos_desktop",
            Self::WindowsDesktop => "windows_desktop",
        }
    }
}

/// Closed list of platform rows the suite must cover.
pub const REQUIRED_PLATFORM_ROW_CLASSES: [PlatformRowClass; 3] = [
    PlatformRowClass::LinuxDesktop,
    PlatformRowClass::MacosDesktop,
    PlatformRowClass::WindowsDesktop,
];

/// Closed regression-outcome vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegressionOutcome {
    /// The row exercises the expected behavior fully.
    Pass,
    /// The row passes only after the closed downgrade label is applied.
    DowngradeRequired,
    /// The row is blocked until a fix lands; the downgrade label records
    /// the reason.
    BlockedUntilFix,
}

impl RegressionOutcome {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::DowngradeRequired => "downgrade_required",
            Self::BlockedUntilFix => "blocked_until_fix",
        }
    }
}

/// Closed downgrade-label vocabulary. A failing row downgrades using one
/// of these labels; nothing outside the suite is admitted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeLabel {
    /// No downgrade applied (the row passes outright).
    None,
    /// Red — the beta row is blocked until the regression is fixed.
    RedBlocksBetaRow,
    /// Yellow — the platform exercises the behavior through a different
    /// primitive, but the safety invariants still hold.
    YellowPlatformSkew,
    /// Yellow — partial coverage; the scenario is only observable on a
    /// subset of host configurations on this platform.
    YellowPartialCoverage,
    /// The row is restricted to the safe save-path until platform parity
    /// ships.
    DegradedToSafePathOnly,
    /// The protected corpus is stale; release candidate cannot promote
    /// until it is restored.
    StaleCorpusBlocksReleaseCandidate,
}

impl DowngradeLabel {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RedBlocksBetaRow => "red_blocks_beta_row",
            Self::YellowPlatformSkew => "yellow_platform_skew",
            Self::YellowPartialCoverage => "yellow_partial_coverage",
            Self::DegradedToSafePathOnly => "degraded_to_safe_path_only",
            Self::StaleCorpusBlocksReleaseCandidate => "stale_corpus_blocks_release_candidate",
        }
    }

    /// Returns true when this label admits the row as healthy (no
    /// downgrade applied).
    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::None)
    }
}

/// Closed open-gap class vocabulary; bounds why a row carries a residual
/// platform gap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenGapClass {
    /// No open gap (placeholder so the row can declare an empty list
    /// honestly).
    None,
    /// The platform implementation is pending; the row downgrades until
    /// it ships.
    PlatformImplementationPending,
    /// An external dependency (driver, mount, vendor extension) is
    /// unstable; the row degrades until the dependency stabilises.
    ExternalDependencyUnstable,
    /// Manual recovery is required; the row records the manual step so
    /// support reviewers can prove it.
    ManualRecoveryRequired,
    /// The watcher source is on a fallback; the row notes it so the
    /// scorecard can grade fall-through correctness instead of the
    /// optimistic path.
    WatcherFallbackOnly,
}

impl OpenGapClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::PlatformImplementationPending => "platform_implementation_pending",
            Self::ExternalDependencyUnstable => "external_dependency_unstable",
            Self::ManualRecoveryRequired => "manual_recovery_required",
            Self::WatcherFallbackOnly => "watcher_fallback_only",
        }
    }
}

/// One open-gap row attached to a regression case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenGapEntry {
    pub gap_class: OpenGapClass,
    pub summary: String,
}

/// Expected save / conflict-resolution behavior for a regression case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedBehavior {
    pub compare_outcome: BetaCompareOutcome,
    pub silent_overwrite_forbidden: bool,
    pub save_redirects_target: bool,
    pub resolution_actions: Vec<BetaResolutionAction>,
    #[serde(default)]
    pub required_blockers: Vec<BetaSaveTargetReviewBlocker>,
}

/// Safety baseline pinned on every regression case and on the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseSafety {
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub destructive_resets_present: bool,
    pub preserves_user_authored_files: bool,
}

impl CaseSafety {
    /// Returns the metadata-safe baseline every emitted record pins.
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
            preserves_user_authored_files: true,
        }
    }
}

/// Companion refs quoted on each regression case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseReferences {
    pub matrix_doc_ref: String,
    pub report_ref: String,
    pub schema_ref: String,
    pub filesystem_identity_beta_doc_ref: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub alpha_evidence_ref: Option<String>,
}

/// One regression-suite case record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveConflictSuiteCase {
    pub schema_version: u32,
    pub record_kind: String,
    pub case_id: String,
    pub title: String,
    pub scenario_class: ScenarioClass,
    pub platform_row_class: PlatformRowClass,
    pub anchor_fixture_ref: String,
    pub anchor_case_id: String,
    pub anchor_difficulty_class: DifficultyClass,
    pub expected_behavior: ExpectedBehavior,
    pub expected_outcome: RegressionOutcome,
    pub downgrade_label: DowngradeLabel,
    #[serde(default)]
    pub open_gaps: Vec<OpenGapEntry>,
    pub safety: CaseSafety,
    pub references: CaseReferences,
    pub captured_at: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reviewer_summary: Option<String>,
}

/// One fixture-bound entry in the regression suite corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveConflictSuiteCorpusEntry {
    pub fixture_ref: String,
    pub case: SaveConflictSuiteCase,
}

/// Regression suite corpus loaded from checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveConflictSuiteCorpus {
    pub entries: Vec<SaveConflictSuiteCorpusEntry>,
}

/// One row in the regression suite report matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportMatrixRow {
    pub case_id: String,
    pub scenario_class: ScenarioClass,
    pub platform_row_class: PlatformRowClass,
    pub anchor_difficulty_class: DifficultyClass,
    pub compare_outcome: BetaCompareOutcome,
    pub expected_outcome: RegressionOutcome,
    pub downgrade_label: DowngradeLabel,
    pub open_gap_classes: Vec<OpenGapClass>,
}

impl ReportMatrixRow {
    fn from_case(case: &SaveConflictSuiteCase) -> Self {
        let mut open_gap_classes: Vec<OpenGapClass> =
            case.open_gaps.iter().map(|gap| gap.gap_class).collect();
        if open_gap_classes.is_empty() {
            open_gap_classes.push(OpenGapClass::None);
        }
        Self {
            case_id: case.case_id.clone(),
            scenario_class: case.scenario_class,
            platform_row_class: case.platform_row_class,
            anchor_difficulty_class: case.anchor_difficulty_class,
            compare_outcome: case.expected_behavior.compare_outcome,
            expected_outcome: case.expected_outcome,
            downgrade_label: case.downgrade_label,
            open_gap_classes,
        }
    }
}

/// Per-platform summary row of the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformSummaryRow {
    pub platform_row_class: PlatformRowClass,
    pub case_count: u32,
    pub pass_count: u32,
    pub downgrade_required_count: u32,
    pub blocked_until_fix_count: u32,
    pub open_gap_count: u32,
}

/// Metadata-safe regression-suite report record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveConflictSuiteReport {
    pub schema_version: u32,
    pub record_kind: String,
    pub report_id: String,
    pub captured_at: String,
    pub matrix_doc_ref: String,
    pub schema_ref: String,
    pub corpus_manifest_ref: String,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub required_scenario_classes: Vec<ScenarioClass>,
    pub required_platform_row_classes: Vec<PlatformRowClass>,
    pub matrix_rows: Vec<ReportMatrixRow>,
    pub platform_summaries: Vec<PlatformSummaryRow>,
}

impl SaveConflictSuiteReport {
    /// Returns true when the report preserves the bounded suite contract.
    pub fn is_export_safe(&self) -> bool {
        if !self.raw_private_material_excluded || !self.ambient_authority_excluded {
            return false;
        }
        if self.matrix_rows.is_empty() {
            return false;
        }
        if self.platform_summaries.is_empty() {
            return false;
        }
        true
    }
}

/// One validation violation emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveConflictSuiteViolation {
    pub check_id: String,
    pub subject_ref: String,
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaveConflictSuiteValidationReport {
    pub violations: Vec<SaveConflictSuiteViolation>,
}

impl fmt::Display for SaveConflictSuiteValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} save-conflict-suite violation(s)",
            self.violations.len()
        )
    }
}

impl Error for SaveConflictSuiteValidationReport {}

/// Save-conflict-suite evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct SaveConflictSuiteEvaluator;

impl SaveConflictSuiteEvaluator {
    /// Creates a new evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates one regression case record.
    pub fn validate_case(
        &self,
        case: &SaveConflictSuiteCase,
    ) -> Result<(), SaveConflictSuiteValidationReport> {
        let violations = validate_case(case);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(SaveConflictSuiteValidationReport { violations })
        }
    }

    /// Validates a corpus against the closed scenario × platform matrix
    /// coverage contract.
    pub fn validate_corpus(
        &self,
        corpus: &SaveConflictSuiteCorpus,
    ) -> Result<(), SaveConflictSuiteValidationReport> {
        let violations = validate_corpus(corpus);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(SaveConflictSuiteValidationReport { violations })
        }
    }

    /// Builds the metadata-safe report projection from a corpus.
    pub fn report(
        &self,
        report_id: impl Into<String>,
        captured_at: impl Into<String>,
        corpus: &SaveConflictSuiteCorpus,
    ) -> Result<SaveConflictSuiteReport, SaveConflictSuiteValidationReport> {
        self.validate_corpus(corpus)?;
        let mut matrix_rows: Vec<ReportMatrixRow> = corpus
            .entries
            .iter()
            .map(|entry| ReportMatrixRow::from_case(&entry.case))
            .collect();
        matrix_rows.sort_by(|a, b| {
            a.scenario_class
                .as_str()
                .cmp(b.scenario_class.as_str())
                .then_with(|| {
                    a.platform_row_class
                        .as_str()
                        .cmp(b.platform_row_class.as_str())
                })
        });

        let platform_summaries = REQUIRED_PLATFORM_ROW_CLASSES
            .iter()
            .map(|platform| summarize_platform(corpus, *platform))
            .collect();

        Ok(SaveConflictSuiteReport {
            schema_version: SAVE_CONFLICT_SUITE_SCHEMA_VERSION,
            record_kind: SAVE_CONFLICT_SUITE_REPORT_RECORD_KIND.to_owned(),
            report_id: report_id.into(),
            captured_at: captured_at.into(),
            matrix_doc_ref: SAVE_CONFLICT_SUITE_MATRIX_DOC_REF.to_owned(),
            schema_ref: SAVE_CONFLICT_SUITE_SCHEMA_REF.to_owned(),
            corpus_manifest_ref: SAVE_CONFLICT_SUITE_CORPUS_MANIFEST_REF.to_owned(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            required_scenario_classes: REQUIRED_SCENARIO_CLASSES.to_vec(),
            required_platform_row_classes: REQUIRED_PLATFORM_ROW_CLASSES.to_vec(),
            matrix_rows,
            platform_summaries,
        })
    }
}

fn summarize_platform(
    corpus: &SaveConflictSuiteCorpus,
    platform: PlatformRowClass,
) -> PlatformSummaryRow {
    let mut row = PlatformSummaryRow {
        platform_row_class: platform,
        case_count: 0,
        pass_count: 0,
        downgrade_required_count: 0,
        blocked_until_fix_count: 0,
        open_gap_count: 0,
    };
    for entry in &corpus.entries {
        if entry.case.platform_row_class != platform {
            continue;
        }
        row.case_count += 1;
        match entry.case.expected_outcome {
            RegressionOutcome::Pass => row.pass_count += 1,
            RegressionOutcome::DowngradeRequired => row.downgrade_required_count += 1,
            RegressionOutcome::BlockedUntilFix => row.blocked_until_fix_count += 1,
        }
        let counted_gaps: u32 = entry
            .case
            .open_gaps
            .iter()
            .filter(|gap| gap.gap_class != OpenGapClass::None)
            .count() as u32;
        row.open_gap_count += counted_gaps;
    }
    row
}

fn validate_corpus(corpus: &SaveConflictSuiteCorpus) -> Vec<SaveConflictSuiteViolation> {
    let mut violations = Vec::new();

    if corpus.entries.is_empty() {
        push_violation(
            &mut violations,
            "corpus.empty",
            SAVE_CONFLICT_SUITE_CORPUS_DIR,
            "corpus must contain at least one regression case",
        );
        return violations;
    }

    let mut case_ids = BTreeSet::new();
    let mut fixture_refs = BTreeSet::new();
    let mut seen_tuples: BTreeSet<(ScenarioClass, PlatformRowClass)> = BTreeSet::new();

    for entry in &corpus.entries {
        if !fixture_refs.insert(entry.fixture_ref.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_fixture_ref",
                &entry.fixture_ref,
                "fixture_ref must be unique within the corpus",
            );
        }
        let case = &entry.case;
        if !case_ids.insert(case.case_id.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_case_id",
                &case.case_id,
                "case_id must be unique within the corpus",
            );
        }
        let tuple = (case.scenario_class, case.platform_row_class);
        if !seen_tuples.insert(tuple) {
            push_violation(
                &mut violations,
                "corpus.duplicate_scenario_platform_tuple",
                &case.case_id,
                format!(
                    "duplicate (scenario_class={}, platform_row_class={}) tuple",
                    case.scenario_class.as_str(),
                    case.platform_row_class.as_str()
                ),
            );
        }
        violations.extend(validate_case(case));
    }

    for scenario in REQUIRED_SCENARIO_CLASSES {
        for platform in REQUIRED_PLATFORM_ROW_CLASSES {
            if !seen_tuples.contains(&(scenario, platform)) {
                push_violation(
                    &mut violations,
                    "corpus.required_scenario_platform_missing",
                    format!("{}/{}", scenario.as_str(), platform.as_str()),
                    format!(
                        "required (scenario_class={}, platform_row_class={}) tuple has no seeded case",
                        scenario.as_str(),
                        platform.as_str()
                    ),
                );
            }
        }
    }

    violations
}

fn validate_case(case: &SaveConflictSuiteCase) -> Vec<SaveConflictSuiteViolation> {
    let mut violations = Vec::new();
    let target = case.case_id.as_str();

    if case.schema_version != SAVE_CONFLICT_SUITE_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "case.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if case.record_kind != SAVE_CONFLICT_SUITE_CASE_RECORD_KIND {
        push_violation(
            &mut violations,
            "case.record_kind",
            target,
            "record_kind must be save_conflict_suite_case_record",
        );
    }
    if case.case_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.case_id",
            target,
            "case_id must be non-empty",
        );
    }
    if case.title.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.title",
            target,
            "title must be non-empty",
        );
    }
    if case.anchor_fixture_ref.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.anchor_fixture_ref",
            target,
            "anchor_fixture_ref must point at a filesystem_identity_beta fixture",
        );
    }
    if case.anchor_case_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.anchor_case_id",
            target,
            "anchor_case_id must reference a filesystem_identity_beta case_id",
        );
    }

    validate_expected_behavior(&mut violations, target, case);
    validate_outcome_and_downgrade(&mut violations, target, case);
    validate_open_gaps(&mut violations, target, &case.open_gaps);
    validate_safety(&mut violations, target, &case.safety);
    validate_references(&mut violations, target, &case.references);

    violations
}

fn validate_expected_behavior(
    violations: &mut Vec<SaveConflictSuiteViolation>,
    target: &str,
    case: &SaveConflictSuiteCase,
) {
    let behavior = &case.expected_behavior;
    if behavior.resolution_actions.is_empty() {
        push_violation(
            violations,
            "case.expected_behavior.resolution_actions",
            target,
            "expected_behavior.resolution_actions must declare at least one action",
        );
    }
    let unique: BTreeSet<BetaResolutionAction> =
        behavior.resolution_actions.iter().copied().collect();
    if unique.len() != behavior.resolution_actions.len() {
        push_violation(
            violations,
            "case.expected_behavior.resolution_actions_duplicate",
            target,
            "expected_behavior.resolution_actions must not contain duplicates",
        );
    }
    let direct_write_allowed = behavior.compare_outcome == BetaCompareOutcome::Unchanged
        && !behavior.silent_overwrite_forbidden;
    if !direct_write_allowed && unique.contains(&BetaResolutionAction::Write) {
        push_violation(
            violations,
            "case.expected_behavior.write_only_when_unchanged",
            target,
            "expected_behavior.resolution_actions may only include write when compare_outcome is unchanged and silent_overwrite_forbidden is false",
        );
    }
    if behavior.compare_outcome != BetaCompareOutcome::Unchanged
        && !behavior.silent_overwrite_forbidden
    {
        push_violation(
            violations,
            "case.expected_behavior.non_unchanged_must_forbid_silent_overwrite",
            target,
            "non-unchanged compare_outcome must set silent_overwrite_forbidden = true",
        );
    }
    let blocker_set: BTreeSet<BetaSaveTargetReviewBlocker> =
        behavior.required_blockers.iter().copied().collect();
    if blocker_set.len() != behavior.required_blockers.len() {
        push_violation(
            violations,
            "case.expected_behavior.required_blockers_duplicate",
            target,
            "expected_behavior.required_blockers must not contain duplicates",
        );
    }

    if matches!(case.scenario_class, ScenarioClass::PermissionLoss)
        && !blocker_set.contains(&BetaSaveTargetReviewBlocker::ReadOnly)
        && !blocker_set.contains(&BetaSaveTargetReviewBlocker::NotWritablePerSnapshot)
        && !blocker_set.contains(&BetaSaveTargetReviewBlocker::PolicyConstrained)
    {
        push_violation(
            violations,
            "case.expected_behavior.permission_loss_blocker_missing",
            target,
            "permission_loss rows must require at least one of read_only / not_writable_per_snapshot / policy_constrained blockers",
        );
    }

    if matches!(case.scenario_class, ScenarioClass::ExternalChange)
        && behavior.compare_outcome != BetaCompareOutcome::ExternalChangeDetected
    {
        push_violation(
            violations,
            "case.expected_behavior.external_change_outcome",
            target,
            "external_change rows must declare compare_outcome = external_change_detected",
        );
    }

    if matches!(case.scenario_class, ScenarioClass::SaveConflict)
        && behavior.compare_outcome != BetaCompareOutcome::SaveConflict
        && behavior.compare_outcome != BetaCompareOutcome::ExternalChangeDetected
    {
        push_violation(
            violations,
            "case.expected_behavior.save_conflict_outcome",
            target,
            "save_conflict rows must declare compare_outcome = save_conflict or external_change_detected",
        );
    }
}

fn validate_outcome_and_downgrade(
    violations: &mut Vec<SaveConflictSuiteViolation>,
    target: &str,
    case: &SaveConflictSuiteCase,
) {
    match case.expected_outcome {
        RegressionOutcome::Pass => {
            if !case.downgrade_label.is_healthy() {
                push_violation(
                    violations,
                    "case.outcome.pass_must_not_carry_downgrade",
                    target,
                    "pass rows must declare downgrade_label = none",
                );
            }
        }
        RegressionOutcome::DowngradeRequired | RegressionOutcome::BlockedUntilFix => {
            if case.downgrade_label.is_healthy() {
                push_violation(
                    violations,
                    "case.outcome.non_pass_must_declare_downgrade",
                    target,
                    "downgrade_required and blocked_until_fix rows must declare a non-none downgrade_label",
                );
            }
            if case.open_gaps.is_empty()
                || case
                    .open_gaps
                    .iter()
                    .all(|gap| gap.gap_class == OpenGapClass::None)
            {
                push_violation(
                    violations,
                    "case.outcome.non_pass_must_record_open_gap",
                    target,
                    "downgrade_required and blocked_until_fix rows must record at least one open_gap with a non-none gap_class",
                );
            }
        }
    }
    if case.expected_outcome == RegressionOutcome::BlockedUntilFix
        && !matches!(
            case.downgrade_label,
            DowngradeLabel::RedBlocksBetaRow | DowngradeLabel::StaleCorpusBlocksReleaseCandidate
        )
    {
        push_violation(
            violations,
            "case.outcome.blocked_until_fix_label_class",
            target,
            "blocked_until_fix rows must downgrade with red_blocks_beta_row or stale_corpus_blocks_release_candidate",
        );
    }
}

fn validate_open_gaps(
    violations: &mut Vec<SaveConflictSuiteViolation>,
    target: &str,
    gaps: &[OpenGapEntry],
) {
    let mut seen = BTreeSet::new();
    for gap in gaps {
        if gap.summary.trim().is_empty() {
            push_violation(
                violations,
                "case.open_gaps.summary",
                target,
                "open_gaps.summary must be non-empty",
            );
        }
        if !seen.insert(gap.gap_class) {
            push_violation(
                violations,
                "case.open_gaps.duplicate_gap_class",
                target,
                format!("duplicate open_gap_class {}", gap.gap_class.as_str()),
            );
        }
    }
}

fn validate_safety(
    violations: &mut Vec<SaveConflictSuiteViolation>,
    target: &str,
    safety: &CaseSafety,
) {
    if !safety.raw_private_material_excluded {
        push_violation(
            violations,
            "case.safety.raw_private_material_excluded",
            target,
            "raw_private_material_excluded must be true",
        );
    }
    if !safety.ambient_authority_excluded {
        push_violation(
            violations,
            "case.safety.ambient_authority_excluded",
            target,
            "ambient_authority_excluded must be true",
        );
    }
    if safety.destructive_resets_present {
        push_violation(
            violations,
            "case.safety.destructive_resets_present",
            target,
            "destructive_resets_present must be false",
        );
    }
    if !safety.preserves_user_authored_files {
        push_violation(
            violations,
            "case.safety.preserves_user_authored_files",
            target,
            "preserves_user_authored_files must be true",
        );
    }
}

fn validate_references(
    violations: &mut Vec<SaveConflictSuiteViolation>,
    target: &str,
    refs: &CaseReferences,
) {
    if refs.matrix_doc_ref != SAVE_CONFLICT_SUITE_MATRIX_DOC_REF {
        push_violation(
            violations,
            "case.references.matrix_doc_ref",
            target,
            format!("references.matrix_doc_ref must pin {SAVE_CONFLICT_SUITE_MATRIX_DOC_REF}"),
        );
    }
    if refs.report_ref != SAVE_CONFLICT_SUITE_REPORT_REF {
        push_violation(
            violations,
            "case.references.report_ref",
            target,
            format!("references.report_ref must pin {SAVE_CONFLICT_SUITE_REPORT_REF}"),
        );
    }
    if refs.schema_ref != SAVE_CONFLICT_SUITE_SCHEMA_REF {
        push_violation(
            violations,
            "case.references.schema_ref",
            target,
            format!("references.schema_ref must pin {SAVE_CONFLICT_SUITE_SCHEMA_REF}"),
        );
    }
    if refs.filesystem_identity_beta_doc_ref != SAVE_CONFLICT_SUITE_FILESYSTEM_IDENTITY_BETA_DOC_REF
    {
        push_violation(
            violations,
            "case.references.filesystem_identity_beta_doc_ref",
            target,
            format!(
                "references.filesystem_identity_beta_doc_ref must pin {SAVE_CONFLICT_SUITE_FILESYSTEM_IDENTITY_BETA_DOC_REF}"
            ),
        );
    }
}

fn push_violation(
    violations: &mut Vec<SaveConflictSuiteViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(SaveConflictSuiteViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

/// Loads a YAML-encoded [`SaveConflictSuiteCase`].
pub fn load_save_conflict_suite_case(
    yaml: &str,
) -> Result<SaveConflictSuiteCase, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Returns the checked-in M3 save-conflict regression suite corpus.
pub fn current_save_conflict_suite_corpus() -> Result<SaveConflictSuiteCorpus, serde_yaml::Error> {
    let entries = CASE_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<SaveConflictSuiteCase>(yaml).map(|case| {
                SaveConflictSuiteCorpusEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    case,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(SaveConflictSuiteCorpus { entries })
}

/// Returns the set of fixture refs the corpus loads, in declaration order.
pub fn current_save_conflict_suite_fixture_refs() -> impl Iterator<Item = &'static str> {
    CASE_FIXTURES.iter().map(|(fixture_ref, _)| *fixture_ref)
}

const CASE_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/recovery/m3/save_conflict_suite/external_change_linux_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/external_change_linux_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/external_change_macos_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/external_change_macos_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/external_change_windows_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/external_change_windows_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/save_conflict_linux_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/save_conflict_linux_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/save_conflict_macos_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/save_conflict_macos_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/save_conflict_windows_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/save_conflict_windows_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/permission_loss_linux_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/permission_loss_linux_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/permission_loss_macos_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/permission_loss_macos_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/permission_loss_windows_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/permission_loss_windows_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/alias_drift_linux_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/alias_drift_linux_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/alias_drift_macos_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/alias_drift_macos_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/alias_drift_windows_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/alias_drift_windows_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/difficult_save_path_linux_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/difficult_save_path_linux_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/difficult_save_path_macos_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/difficult_save_path_macos_case.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/save_conflict_suite/difficult_save_path_windows_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/save_conflict_suite/difficult_save_path_windows_case.yaml"
        )),
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn passing_case() -> SaveConflictSuiteCase {
        SaveConflictSuiteCase {
            schema_version: SAVE_CONFLICT_SUITE_SCHEMA_VERSION,
            record_kind: SAVE_CONFLICT_SUITE_CASE_RECORD_KIND.to_owned(),
            case_id: "case:test:passing".to_owned(),
            title: "Passing test case".to_owned(),
            scenario_class: ScenarioClass::SaveConflict,
            platform_row_class: PlatformRowClass::LinuxDesktop,
            anchor_fixture_ref: "fixtures/recovery/m3/filesystem_identity/symlink_alias_case.yaml"
                .to_owned(),
            anchor_case_id: "case:filesystem_identity_beta:symlink_alias".to_owned(),
            anchor_difficulty_class: DifficultyClass::SymlinkAlias,
            expected_behavior: ExpectedBehavior {
                compare_outcome: BetaCompareOutcome::SaveConflict,
                silent_overwrite_forbidden: true,
                save_redirects_target: true,
                resolution_actions: vec![
                    BetaResolutionAction::Compare,
                    BetaResolutionAction::SaveAs,
                    BetaResolutionAction::Cancel,
                ],
                required_blockers: vec![],
            },
            expected_outcome: RegressionOutcome::Pass,
            downgrade_label: DowngradeLabel::None,
            open_gaps: vec![],
            safety: CaseSafety::metadata_safe_baseline(),
            references: CaseReferences {
                matrix_doc_ref: SAVE_CONFLICT_SUITE_MATRIX_DOC_REF.to_owned(),
                report_ref: SAVE_CONFLICT_SUITE_REPORT_REF.to_owned(),
                schema_ref: SAVE_CONFLICT_SUITE_SCHEMA_REF.to_owned(),
                filesystem_identity_beta_doc_ref:
                    SAVE_CONFLICT_SUITE_FILESYSTEM_IDENTITY_BETA_DOC_REF.to_owned(),
                alpha_evidence_ref: None,
            },
            captured_at: "2026-05-16T00:00:00Z".to_owned(),
            reviewer_summary: None,
        }
    }

    #[test]
    fn passing_case_validates() {
        SaveConflictSuiteEvaluator::new()
            .validate_case(&passing_case())
            .expect("passing case must validate");
    }

    #[test]
    fn refuses_pass_with_downgrade_label() {
        let mut case = passing_case();
        case.downgrade_label = DowngradeLabel::YellowPlatformSkew;
        let err = SaveConflictSuiteEvaluator::new()
            .validate_case(&case)
            .expect_err("pass with downgrade must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.outcome.pass_must_not_carry_downgrade"));
    }

    #[test]
    fn refuses_downgrade_without_open_gap() {
        let mut case = passing_case();
        case.expected_outcome = RegressionOutcome::DowngradeRequired;
        case.downgrade_label = DowngradeLabel::YellowPlatformSkew;
        let err = SaveConflictSuiteEvaluator::new()
            .validate_case(&case)
            .expect_err("downgrade without open gap must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.outcome.non_pass_must_record_open_gap"));
    }

    #[test]
    fn refuses_destructive_reset() {
        let mut case = passing_case();
        case.safety.destructive_resets_present = true;
        let err = SaveConflictSuiteEvaluator::new()
            .validate_case(&case)
            .expect_err("destructive reset must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.safety.destructive_resets_present"));
    }

    #[test]
    fn refuses_external_change_with_wrong_compare_outcome() {
        let mut case = passing_case();
        case.scenario_class = ScenarioClass::ExternalChange;
        case.expected_behavior.compare_outcome = BetaCompareOutcome::Unchanged;
        case.expected_behavior.silent_overwrite_forbidden = false;
        case.expected_behavior.resolution_actions = vec![BetaResolutionAction::Write];
        let err = SaveConflictSuiteEvaluator::new()
            .validate_case(&case)
            .expect_err("external change with wrong outcome must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.expected_behavior.external_change_outcome"));
    }

    #[test]
    fn checked_in_corpus_loads_and_validates() {
        let corpus = current_save_conflict_suite_corpus().expect("checked-in corpus must parse");
        SaveConflictSuiteEvaluator::new()
            .validate_corpus(&corpus)
            .expect("checked-in corpus must validate");
        for scenario in REQUIRED_SCENARIO_CLASSES {
            for platform in REQUIRED_PLATFORM_ROW_CLASSES {
                assert!(
                    corpus.entries.iter().any(|entry| {
                        entry.case.scenario_class == scenario
                            && entry.case.platform_row_class == platform
                    }),
                    "missing (scenario_class={}, platform_row_class={}) tuple",
                    scenario.as_str(),
                    platform.as_str()
                );
            }
        }
    }

    #[test]
    fn report_is_export_safe() {
        let corpus = current_save_conflict_suite_corpus().unwrap();
        let report = SaveConflictSuiteEvaluator::new()
            .report("report:test", "2026-05-16T00:00:00Z", &corpus)
            .expect("report must build");
        assert!(report.is_export_safe());
        assert_eq!(report.matrix_rows.len(), corpus.entries.len());
        assert_eq!(
            report.platform_summaries.len(),
            REQUIRED_PLATFORM_ROW_CLASSES.len()
        );
        let total_cases: u32 = report.platform_summaries.iter().map(|p| p.case_count).sum();
        assert_eq!(total_cases as usize, corpus.entries.len());
    }
}
