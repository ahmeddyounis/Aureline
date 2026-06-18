//! Docs example/link validation reports: validation-mode states, last-checked
//! time, environment/version scope, producing-validator context, and
//! open-snippet/open-failing-source/compare-current-source/suppress/rerun parity.
//!
//! This module owns the runtime truth packet behind the docs validation report —
//! the surface that turns documented examples and links into typed, reviewable
//! validation rows instead of decorative pass/fail badges. Each
//! [`ValidationReportRow`] names a concrete subject (the doc file and, when
//! applicable, the snippet/link anchor), one [`ValidationMode`] (rendered,
//! syntax-checked, executed locally, executed remotely, skipped, stale,
//! unsupported, or broken-link), a [`ValidationOutcome`] that must agree with
//! that mode, an explicit `last_checked_at` time and a [`ValidationScope`]
//! (environment label, toolchain ref, target version ref, version match), the
//! [`ValidationProducer`] that produced the finding (the validator kind plus its
//! execution-context ref), a freshness/version/locality chip set, one
//! evidence-provenance disclosure, a source/evidence trace for actionable
//! findings, the full [`ValidationActionSet`] (Open snippet, Open failing source,
//! Compare current source, Suppress, Rerun), and a durable
//! [`ValidationSuppression`] so suppressing a finding stays attributable,
//! previewable, and reopenable.
//!
//! Six invariants make a validation row honest:
//!
//! - **Explicit, labeled mode.** Every row names exactly one validation mode, so
//!   a harmless rendered preview is never mistaken for a syntax-checked or an
//!   actually executed example.
//! - **Mode/outcome consistency.** A row's outcome must match its mode; a
//!   rendered preview (or a skipped/stale/unsupported row) may never claim an
//!   executed pass or fail ([`ValidationFindingKind::ExecutionClaimWithoutRun`]).
//! - **Explicit last-checked time and environment/version scope.** Every row
//!   carries a non-empty `last_checked_at` and a full environment/version scope,
//!   so a result is never presented without saying when and where it was checked.
//! - **Producer attribution.** Every row names the validator and execution
//!   context that produced it, the validator must be one the mode permits, and
//!   the action set preserves that producing context across suppress and rerun.
//! - **Actionable, traced findings.** A failing, broken-link, stale, or
//!   unsupported row must carry a source/evidence trace and the full action set;
//!   a finding with no trace is a decorative badge and is rejected.
//! - **Durable, honest suppression and visible cached truth.** A suppressed row
//!   carries an attribution ref and a durable history ref and stays reopenable;
//!   and an imported / mirrored / local-only / stale / derived source may never
//!   be presented as a high-freshness authoritative executed pass.
//!
//! The [`DocsValidationReportExport`] is the projection support, review, AI
//! evidence, release, and diagnostics surfaces ingest: one
//! [`ValidationReportExportRow`] per row preserving mode, outcome, last-checked
//! time, environment/version scope, freshness, provenance, the producing
//! validator, suppression state, action parity, and trace state — so the
//! environment/version scope and imported/cached state stay visible through
//! export and support flows.
//!
//! [`DocsValidationReportPacket::materialize`] computes the validation findings
//! and the promotion state (`stable`, `narrowed_below_stable`, or
//! `blocks_stable`) from the input — folding the packet-level degradation
//! severities into the promotion decision — so a clean report stays Stable, a
//! degraded-but-honest report narrows below Stable, and a report with an
//! unlabeled mode, a mode/outcome mismatch, a missing last-checked time or scope,
//! an untraced finding, a collapsed result/version truth, a misattributed
//! producer, or a non-reopenable suppression blocks before it reaches a consumer
//! surface. The packet is an inspectable, serde-serializable truth packet: it
//! carries no raw document bodies, no raw source files, no raw URLs, no rendered
//! HTML, no execution logs, no raw provider payloads, and no credentials — only
//! metadata, subject refs, mode/outcome tokens, scope and chip truth, provenance
//! disclosure, producer refs, trace refs, action parity, suppression history,
//! finding summaries, and contract refs.
//!
//! The boundary schema is
//! [`schemas/docs/docs-validation-report.schema.json`](../../../../schemas/docs/docs-validation-report.schema.json).
//! The contract doc is
//! [`docs/m5/docs-validation-report.md`](../../../../docs/m5/docs-validation-report.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/example-link-validation/`](../../../../fixtures/docs/m5/example-link-validation/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`DocsValidationReportPacket`].
pub const DOCS_VALIDATION_REPORT_RECORD_KIND: &str = "docs_validation_report";

/// Record-kind tag carried by the support-export wrapper.
pub const DOCS_VALIDATION_REPORT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "docs_validation_report_support_export";

/// Schema version for docs-validation-report records.
pub const DOCS_VALIDATION_REPORT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DOCS_VALIDATION_REPORT_SCHEMA_REF: &str =
    "schemas/docs/docs-validation-report.schema.json";

/// Repo-relative path of the docs-validation-report contract doc.
pub const DOCS_VALIDATION_REPORT_DOC_REF: &str = "docs/m5/docs-validation-report.md";

/// Repo-relative path of the protected fixture directory.
pub const DOCS_VALIDATION_REPORT_FIXTURE_DIR: &str = "fixtures/docs/m5/example-link-validation";

/// Repo-relative path of the checked support-export artifact.
pub const DOCS_VALIDATION_REPORT_ARTIFACT_REF: &str =
    "artifacts/docs/m5/docs-validation-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DOCS_VALIDATION_REPORT_SUMMARY_REF: &str = "artifacts/docs/m5/docs-validation-proof.md";

/// Kind of subject a validation row covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSubjectKind {
    /// A fenced code example.
    CodeExample,
    /// A shell/command example.
    ShellExample,
    /// A configuration example.
    ConfigExample,
    /// An external (or repo-relative) documentation link.
    Link,
    /// An in-document anchor link.
    AnchorLink,
}

impl ValidationSubjectKind {
    /// The subject kinds a report must cover to be a delivery-grade boundary.
    pub const REQUIRED: [Self; 2] = [Self::CodeExample, Self::Link];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeExample => "code_example",
            Self::ShellExample => "shell_example",
            Self::ConfigExample => "config_example",
            Self::Link => "link",
            Self::AnchorLink => "anchor_link",
        }
    }
}

/// The validation mode applied to a row — the central distinction between a
/// harmless rendered preview, a syntax check, and an actually executed example.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationMode {
    /// The example was rendered for preview only; nothing was executed.
    Rendered,
    /// The example was parsed/syntax-checked but not executed.
    SyntaxChecked,
    /// The example was actually executed in the local environment.
    ExecutedLocal,
    /// The example was actually executed in a remote/managed runner.
    ExecutedRemote,
    /// Validation was intentionally skipped.
    Skipped,
    /// A prior validation is stale; it must be re-checked.
    Stale,
    /// Validation is unsupported for this subject/environment.
    Unsupported,
    /// A link failed to resolve.
    BrokenLink,
}

impl ValidationMode {
    /// The modes a report must demonstrate to claim cross-surface coverage: a
    /// harmless rendered preview, a real local execution, a broken-link finding,
    /// and a stale-example finding.
    pub const REQUIRED: [Self; 4] = [
        Self::Rendered,
        Self::ExecutedLocal,
        Self::BrokenLink,
        Self::Stale,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rendered => "rendered",
            Self::SyntaxChecked => "syntax_checked",
            Self::ExecutedLocal => "executed_local",
            Self::ExecutedRemote => "executed_remote",
            Self::Skipped => "skipped",
            Self::Stale => "stale",
            Self::Unsupported => "unsupported",
            Self::BrokenLink => "broken_link",
        }
    }

    /// Whether this mode actually executed the example.
    pub const fn is_executed(self) -> bool {
        matches!(self, Self::ExecutedLocal | Self::ExecutedRemote)
    }

    /// Whether this mode is an actionable finding that demands a source/evidence
    /// trace (broken link, stale, or unsupported).
    pub const fn is_attention_finding(self) -> bool {
        matches!(self, Self::BrokenLink | Self::Stale | Self::Unsupported)
    }

    /// Whether the given outcome is consistent with this mode.
    pub const fn outcome_is_consistent(self, outcome: ValidationOutcome) -> bool {
        use ValidationOutcome::*;
        match self {
            Self::Rendered => matches!(outcome, RenderedPreviewOnly),
            Self::SyntaxChecked => matches!(outcome, SyntaxValid | SyntaxInvalid),
            Self::ExecutedLocal | Self::ExecutedRemote => {
                matches!(outcome, ExecutedPass | ExecutedFail | PassedWithWarnings)
            }
            Self::BrokenLink => matches!(outcome, LinkBroken),
            Self::Skipped | Self::Stale | Self::Unsupported => matches!(outcome, NotRun),
        }
    }

    /// Whether the given validator may legitimately produce this mode. The
    /// executed/rendered/syntax/broken modes name a single producing validator;
    /// the not-run/stale modes accept any validator whose prior run went stale or
    /// that declined to run.
    pub const fn permits_validator(self, validator: ValidatorKind) -> bool {
        use ValidatorKind::*;
        match self {
            Self::Rendered => matches!(validator, RenderedPreviewEngine),
            Self::SyntaxChecked => matches!(validator, SyntaxChecker),
            Self::ExecutedLocal => matches!(validator, LocalExampleHarness),
            Self::ExecutedRemote => matches!(validator, RemoteExampleRunner),
            Self::BrokenLink => matches!(validator, LinkChecker),
            Self::Skipped | Self::Stale | Self::Unsupported => true,
        }
    }
}

/// The outcome recorded for a validation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationOutcome {
    /// The example was executed and passed.
    ExecutedPass,
    /// The example was executed and failed.
    ExecutedFail,
    /// The example was executed and passed with warnings.
    PassedWithWarnings,
    /// The example was rendered for preview only; no execution was claimed.
    RenderedPreviewOnly,
    /// The example was syntax-checked and is valid; no execution was claimed.
    SyntaxValid,
    /// The example was syntax-checked and is invalid.
    SyntaxInvalid,
    /// A link failed to resolve.
    LinkBroken,
    /// The row was not run (skipped, stale, or unsupported).
    NotRun,
}

impl ValidationOutcome {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutedPass => "executed_pass",
            Self::ExecutedFail => "executed_fail",
            Self::PassedWithWarnings => "passed_with_warnings",
            Self::RenderedPreviewOnly => "rendered_preview_only",
            Self::SyntaxValid => "syntax_valid",
            Self::SyntaxInvalid => "syntax_invalid",
            Self::LinkBroken => "link_broken",
            Self::NotRun => "not_run",
        }
    }

    /// Whether this outcome claims that the example was actually executed.
    pub const fn claims_execution(self) -> bool {
        matches!(self, Self::ExecutedPass | Self::ExecutedFail)
    }

    /// Whether this outcome is the strong "executed and passed" claim.
    pub const fn claims_execution_pass(self) -> bool {
        matches!(self, Self::ExecutedPass)
    }

    /// Whether this outcome is a failing finding that demands a trace.
    pub const fn is_failing(self) -> bool {
        matches!(
            self,
            Self::ExecutedFail | Self::SyntaxInvalid | Self::LinkBroken
        )
    }
}

/// The validator that produced a validation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidatorKind {
    /// The rendered-preview engine (rendered, never executed).
    RenderedPreviewEngine,
    /// The syntax checker / parser.
    SyntaxChecker,
    /// The local example-execution harness.
    LocalExampleHarness,
    /// The remote/managed example runner.
    RemoteExampleRunner,
    /// The documentation link checker.
    LinkChecker,
    /// A manual reviewer.
    ManualReviewer,
}

impl ValidatorKind {
    /// Stable token recorded in the producer.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RenderedPreviewEngine => "rendered_preview_engine",
            Self::SyntaxChecker => "syntax_checker",
            Self::LocalExampleHarness => "local_example_harness",
            Self::RemoteExampleRunner => "remote_example_runner",
            Self::LinkChecker => "link_checker",
            Self::ManualReviewer => "manual_reviewer",
        }
    }
}

/// Evidence provenance for a row, kept visible so a cached or imported result is
/// never mistaken for authoritative live truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationEvidenceProvenance {
    /// First-party evidence verified against the in-repo build.
    FirstPartyVerified,
    /// Local-only evidence that has not been verified.
    LocalOnlyUnverified,
    /// Evidence imported from a signed pack or extension.
    Imported,
    /// Evidence served from a mirror.
    Mirrored,
    /// Evidence known to be stale.
    Stale,
    /// Derived / inferred evidence only.
    DerivedHeuristic,
}

impl ValidationEvidenceProvenance {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyVerified => "first_party_verified",
            Self::LocalOnlyUnverified => "local_only_unverified",
            Self::Imported => "imported",
            Self::Mirrored => "mirrored",
            Self::Stale => "stale",
            Self::DerivedHeuristic => "derived_heuristic",
        }
    }

    /// Whether this provenance may back an authoritative high-freshness executed
    /// pass. Only first-party verified evidence may.
    pub const fn is_authoritative(self) -> bool {
        matches!(self, Self::FirstPartyVerified)
    }

    /// Whether a row of this provenance must stay cited.
    pub const fn needs_citation(self) -> bool {
        !matches!(self, Self::FirstPartyVerified)
    }
}

/// Version-match state for a row, projected as the version chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationVersionMatch {
    /// Matches the active build/workspace revision exactly.
    ExactBuildMatch,
    /// Within an accepted compatible drift window.
    CompatibleMinorDrift,
    /// Drifted incompatibly from the active target.
    IncompatibleDriftDetected,
    /// Pre-release; verification has not completed.
    PreReleaseUnverified,
    /// The target build/workspace revision could not be verified.
    UnknownTargetBuild,
}

impl ValidationVersionMatch {
    /// Stable token recorded in the chip/scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuildMatch => "exact_build_match",
            Self::CompatibleMinorDrift => "compatible_minor_drift",
            Self::IncompatibleDriftDetected => "incompatible_drift_detected",
            Self::PreReleaseUnverified => "pre_release_unverified",
            Self::UnknownTargetBuild => "unknown_target_build",
        }
    }

    /// Whether this state may be presented as a confident current-version match.
    pub const fn is_confident_current(self) -> bool {
        matches!(self, Self::ExactBuildMatch)
    }
}

/// Freshness state for a row, projected as the freshness chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationFreshness {
    /// Live and authoritative at validation time.
    AuthoritativeLive,
    /// Cached within its freshness window.
    WarmCached,
    /// Cached and usable only with degraded disclosure.
    DegradedCached,
    /// Stale and must not claim current authority.
    Stale,
    /// Freshness could not be verified.
    Unverified,
    /// A refresh is pending; the source has not yet re-synced.
    RefreshPending,
}

impl ValidationFreshness {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
            Self::RefreshPending => "refresh_pending",
        }
    }

    /// Whether this state may claim live authoritative freshness.
    pub const fn is_authoritative_live(self) -> bool {
        matches!(self, Self::AuthoritativeLive)
    }
}

/// Locality / install posture for a row, projected as the locality chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationLocality {
    /// Resolved from local content or the in-repo index.
    Local,
    /// Resolved through an imported pack.
    ImportedPack,
    /// Resolved through a mirrored pack.
    MirroredPack,
    /// Resolved through a managed (org-hosted) service.
    Managed,
}

impl ValidationLocality {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::ImportedPack => "imported_pack",
            Self::MirroredPack => "mirrored_pack",
            Self::Managed => "managed",
        }
    }
}

/// Suppression state of a validation row in durable history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationSuppressionState {
    /// The row is active and surfaced in review.
    Active,
    /// The finding was suppressed (acknowledged, muted) by a reviewer.
    Suppressed,
    /// A previously-suppressed finding was reopened.
    Reopened,
}

impl ValidationSuppressionState {
    /// Stable token recorded in the suppression.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Suppressed => "suppressed",
            Self::Reopened => "reopened",
        }
    }

    /// Whether this state is a recorded suppression (rather than active/reopened)
    /// and so must be attributable, previewable, and reopenable.
    pub const fn is_suppressed(self) -> bool {
        matches!(self, Self::Suppressed)
    }
}

/// Severity of a degradation or validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationFindingSeverity {
    /// Blocks a Stable claim; the report must block.
    Blocking,
    /// Narrows below Stable but the report stays valid and attributable.
    Narrowing,
    /// Advisory only.
    Advisory,
}

impl ValidationFindingSeverity {
    /// Stable token recorded in the finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Narrowing => "narrowing",
            Self::Advisory => "advisory",
        }
    }
}

/// Consumer surface that must project the docs-validation-report packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationConsumerSurface {
    /// The docs validation report surface itself.
    DocsValidationReport,
    /// The docs authoring surface.
    DocsAuthoringSurface,
    /// The shared docs review panel.
    DocsReviewPanel,
    /// The docs browser shell.
    DocsBrowserShell,
    /// The release center (release-facing docs lane).
    ReleaseCenter,
    /// The AI context inspector.
    AiContextInspector,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl ValidationConsumerSurface {
    /// Stable token recorded in the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsValidationReport => "docs_validation_report",
            Self::DocsAuthoringSurface => "docs_authoring_surface",
            Self::DocsReviewPanel => "docs_review_panel",
            Self::DocsBrowserShell => "docs_browser_shell",
            Self::ReleaseCenter => "release_center",
            Self::AiContextInspector => "ai_context_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Class of a packet-level docs-validation degradation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationDegradationClass {
    /// A mirror is offline; rows are served from the last snapshot.
    MirrorOfflineSnapshot,
    /// The local example harness is unavailable, so example rows could not run.
    ExampleHarnessUnavailable,
    /// The remote runner is unavailable, so remote-execution rows could not run.
    RemoteRunnerUnavailable,
    /// The link checker is offline, so link rows could not be re-verified.
    LinkCheckerOffline,
    /// The rendered-preview engine is degraded.
    RenderEngineDegraded,
    /// The report was rerun at a narrowed scope.
    ScopeNarrowedRerun,
    /// The report claim was narrowed before publication.
    ReportNarrowed,
    /// The owning source is quarantined.
    QuarantinedSource,
}

impl ValidationDegradationClass {
    /// Stable token recorded in the degradation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorOfflineSnapshot => "mirror_offline_snapshot",
            Self::ExampleHarnessUnavailable => "example_harness_unavailable",
            Self::RemoteRunnerUnavailable => "remote_runner_unavailable",
            Self::LinkCheckerOffline => "link_checker_offline",
            Self::RenderEngineDegraded => "render_engine_degraded",
            Self::ScopeNarrowedRerun => "scope_narrowed_rerun",
            Self::ReportNarrowed => "report_narrowed",
            Self::QuarantinedSource => "quarantined_source",
        }
    }
}

/// Scope a docs-validation export covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationExportScope {
    /// Every row in the packet.
    AllRows,
    /// Failing / broken-link rows only.
    FailingOnly,
    /// Attention rows (failing, broken, stale, unsupported) only.
    AttentionOnly,
    /// Suppressed rows only.
    SuppressedOnly,
}

impl ValidationExportScope {
    /// Stable token recorded in the export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllRows => "all_rows",
            Self::FailingOnly => "failing_only",
            Self::AttentionOnly => "attention_only",
            Self::SuppressedOnly => "suppressed_only",
        }
    }
}

/// Promotion state computed for the docs-validation-report packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationPromotionState {
    /// Report qualifies for the Stable claim.
    Stable,
    /// Report narrowed below Stable but stays valid and attributable.
    NarrowedBelowStable,
    /// Report has a blocking finding and must not present as Stable.
    BlocksStable,
}

impl ValidationPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation finding kind emitted by [`DocsValidationReportPacket::materialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationFindingKind {
    /// A required identity field is missing.
    MissingIdentity,
    /// The row set is empty.
    RowsEmpty,
    /// A row id is duplicated.
    DuplicateRowId,
    /// A required subject kind (code_example / link) is missing.
    RequiredSubjectKindMissing,
    /// A required mode (rendered / executed_local / broken_link / stale) is missing.
    RequiredModeCoverageMissing,
    /// A row does not name a concrete subject doc/anchor/label.
    SubjectIdentityMissing,
    /// A row is missing its detail.
    DetailMissing,
    /// A row is missing its last-checked time.
    LastCheckedMissing,
    /// A row is missing part of its environment/version scope.
    EnvironmentScopeMissing,
    /// A row is missing its producing-validator execution context.
    ProducerContextMissing,
    /// A row's producing validator is not one its mode permits.
    ProducerValidatorModeMismatch,
    /// A row's outcome claims execution but its mode did not run the example.
    ExecutionClaimWithoutRun,
    /// A row's outcome is otherwise inconsistent with its mode.
    ModeOutcomeInconsistent,
    /// A row is missing its provenance disclosure note.
    ProvenanceDisclosureMissing,
    /// An unverified source is presented as a high-freshness executed pass.
    ResultTruthCollapsed,
    /// A non-current version is presented as a confident live executed pass.
    VersionTruthCollapsed,
    /// An unverified evidence source is not cited.
    RowNotCited,
    /// A failing / broken / stale / unsupported finding carries no source trace.
    FindingNotTraced,
    /// The action parity (snippet / failing-source / compare / suppress / rerun) is incomplete.
    ActionParityIncomplete,
    /// The action set does not preserve the producing validator/context.
    ProducerNotPreserved,
    /// A suppressed row is missing its attribution or history ref.
    SuppressionNotAttributable,
    /// A suppressed row is not previewable or reopenable.
    SuppressionNotReopenable,
    /// The export drops a required preservation flag.
    ExportDropsPreservation,
    /// An export row references a row id absent from the rows.
    ExportRowOrphan,
    /// A row has no matching export row.
    ExportCoverageMissing,
    /// An export row's mode disagrees with the row.
    ExportModeMismatch,
    /// An export row's outcome disagrees with the row.
    ExportOutcomeMismatch,
    /// An export row's last-checked time disagrees with the row.
    ExportLastCheckedMismatch,
    /// An export row's environment/version scope disagrees with the row.
    ExportScopeMismatch,
    /// An export row's freshness disagrees with the row's chip.
    ExportFreshnessMismatch,
    /// An export row's provenance disagrees with the row.
    ExportProvenanceMismatch,
    /// An export row's producing validator disagrees with the row.
    ExportProducerMismatch,
    /// An export row's suppression state disagrees with the row.
    ExportSuppressionMismatch,
    /// An export row's action-parity flag disagrees with the row.
    ExportActionParityMismatch,
    /// An export row's cited flag disagrees with the row.
    ExportCitedMismatch,
    /// A degradation is incomplete (missing summary).
    DegradationIncomplete,
    /// A degradation references a row id absent from the rows.
    DegradationOrphan,
    /// A consumer projection drops a required preservation flag.
    ConsumerProjectionDrift,
    /// A consumer projection references the wrong packet id.
    ConsumerProjectionPacketIdMismatch,
    /// A required consumer surface is missing from the projections.
    RequiredSurfaceCoverageMissing,
    /// Raw bodies, raw URLs, execution logs, or secrets crossed the boundary.
    RawBoundaryMaterialPresent,
}

impl ValidationFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingIdentity => "missing_identity",
            Self::RowsEmpty => "rows_empty",
            Self::DuplicateRowId => "duplicate_row_id",
            Self::RequiredSubjectKindMissing => "required_subject_kind_missing",
            Self::RequiredModeCoverageMissing => "required_mode_coverage_missing",
            Self::SubjectIdentityMissing => "subject_identity_missing",
            Self::DetailMissing => "detail_missing",
            Self::LastCheckedMissing => "last_checked_missing",
            Self::EnvironmentScopeMissing => "environment_scope_missing",
            Self::ProducerContextMissing => "producer_context_missing",
            Self::ProducerValidatorModeMismatch => "producer_validator_mode_mismatch",
            Self::ExecutionClaimWithoutRun => "execution_claim_without_run",
            Self::ModeOutcomeInconsistent => "mode_outcome_inconsistent",
            Self::ProvenanceDisclosureMissing => "provenance_disclosure_missing",
            Self::ResultTruthCollapsed => "result_truth_collapsed",
            Self::VersionTruthCollapsed => "version_truth_collapsed",
            Self::RowNotCited => "row_not_cited",
            Self::FindingNotTraced => "finding_not_traced",
            Self::ActionParityIncomplete => "action_parity_incomplete",
            Self::ProducerNotPreserved => "producer_not_preserved",
            Self::SuppressionNotAttributable => "suppression_not_attributable",
            Self::SuppressionNotReopenable => "suppression_not_reopenable",
            Self::ExportDropsPreservation => "export_drops_preservation",
            Self::ExportRowOrphan => "export_row_orphan",
            Self::ExportCoverageMissing => "export_coverage_missing",
            Self::ExportModeMismatch => "export_mode_mismatch",
            Self::ExportOutcomeMismatch => "export_outcome_mismatch",
            Self::ExportLastCheckedMismatch => "export_last_checked_mismatch",
            Self::ExportScopeMismatch => "export_scope_mismatch",
            Self::ExportFreshnessMismatch => "export_freshness_mismatch",
            Self::ExportProvenanceMismatch => "export_provenance_mismatch",
            Self::ExportProducerMismatch => "export_producer_mismatch",
            Self::ExportSuppressionMismatch => "export_suppression_mismatch",
            Self::ExportActionParityMismatch => "export_action_parity_mismatch",
            Self::ExportCitedMismatch => "export_cited_mismatch",
            Self::DegradationIncomplete => "degradation_incomplete",
            Self::DegradationOrphan => "degradation_orphan",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ConsumerProjectionPacketIdMismatch => "consumer_projection_packet_id_mismatch",
            Self::RequiredSurfaceCoverageMissing => "required_surface_coverage_missing",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
        }
    }

    /// Default severity for this finding kind. Every validation finding blocks
    /// the Stable claim; narrowing comes only from data-carried degradation
    /// severities so a degraded-but-honest report narrows rather than blocks.
    pub const fn default_severity(self) -> ValidationFindingSeverity {
        ValidationFindingSeverity::Blocking
    }
}

/// The concrete subject a validation row covers — the doc file and (optionally)
/// the snippet/link anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationSubject {
    /// The kind of subject.
    pub subject_kind: ValidationSubjectKind,
    /// Repo-relative doc file ref the example/link lives in (no raw body).
    pub doc_ref: String,
    /// Snippet/link anchor within the doc when the row is scoped to one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snippet_anchor: Option<String>,
    /// Human-readable display path for the subject.
    pub display_path: String,
    /// Human-readable subject label (no raw bodies).
    pub label: String,
}

impl ValidationSubject {
    /// Whether the subject names a concrete doc, display path, and label (and a
    /// non-empty anchor when one is recorded).
    pub fn names_concrete_subject(&self) -> bool {
        if self.doc_ref.trim().is_empty()
            || self.display_path.trim().is_empty()
            || self.label.trim().is_empty()
        {
            return false;
        }
        match &self.snippet_anchor {
            Some(anchor) => !anchor.trim().is_empty(),
            None => true,
        }
    }
}

/// The environment/version scope a row was validated under.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationScope {
    /// Human-readable environment label (e.g. toolchain + target triple).
    pub environment_label: String,
    /// Toolchain ref the validation ran under (no raw payloads).
    pub toolchain_ref: String,
    /// Ref to the target build/source revision validated against.
    pub target_version_ref: String,
    /// Version-match state between the example and the active target.
    pub version_match: ValidationVersionMatch,
}

impl ValidationScope {
    /// Whether the scope names a concrete environment, toolchain, and target.
    pub fn is_complete(&self) -> bool {
        !self.environment_label.trim().is_empty()
            && !self.toolchain_ref.trim().is_empty()
            && !self.target_version_ref.trim().is_empty()
    }
}

/// The validator and execution context that produced a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationProducer {
    /// The validator that produced the finding.
    pub validator: ValidatorKind,
    /// Ref to the execution context (toolchain/runner digest) — no raw payloads.
    pub execution_context_ref: String,
    /// Human-readable producer detail (no raw logs).
    pub detail: String,
}

impl ValidationProducer {
    /// Whether the producer names a concrete execution context and detail.
    pub fn names_concrete_context(&self) -> bool {
        !self.execution_context_ref.trim().is_empty() && !self.detail.trim().is_empty()
    }
}

/// The action set a validation row exposes — Open snippet, Open failing source,
/// Compare current source, Suppress, and Rerun.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationActionSet {
    /// Open-snippet ref (open the example snippet / link occurrence).
    pub open_snippet_ref: String,
    /// Open-failing-source ref (open the underlying source the row points at).
    pub open_failing_source_ref: String,
    /// Compare-current-source ref (compare the doc snippet against current source).
    pub compare_current_source_ref: String,
    /// Whether Suppress is available.
    pub suppress_available: bool,
    /// Whether Rerun is available.
    pub rerun_available: bool,
    /// Whether the actions preserve the producing validator/execution context
    /// (so suppress and rerun keep the finding attributable).
    pub preserves_producer: bool,
}

impl ValidationActionSet {
    /// Whether every action is present (snippet, failing-source, compare,
    /// suppress, and rerun parity).
    pub fn parity_complete(&self) -> bool {
        !self.open_snippet_ref.trim().is_empty()
            && !self.open_failing_source_ref.trim().is_empty()
            && !self.compare_current_source_ref.trim().is_empty()
            && self.suppress_available
            && self.rerun_available
    }
}

/// The durable suppression record for a row — its history-backed state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationSuppression {
    /// The suppression state.
    pub state: ValidationSuppressionState,
    /// Ref to the actor who suppressed the finding (required when suppressed).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributed_to_ref: Option<String>,
    /// Durable history-entry ref (required when suppressed).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_ref: Option<String>,
    /// Whether the suppression is previewable.
    pub previewable: bool,
    /// Whether the suppression is reopenable.
    pub reopenable: bool,
    /// Human-readable suppression note (no raw bodies).
    pub note: String,
}

impl ValidationSuppression {
    /// Whether a suppressed row carries its attribution and history refs.
    pub fn is_attributable(&self) -> bool {
        if !self.state.is_suppressed() {
            return true;
        }
        let attributed = self
            .attributed_to_ref
            .as_ref()
            .is_some_and(|r| !r.trim().is_empty());
        let history = self
            .history_ref
            .as_ref()
            .is_some_and(|r| !r.trim().is_empty());
        attributed && history
    }

    /// Whether a suppressed row is previewable and reopenable.
    pub fn is_reopenable(&self) -> bool {
        if !self.state.is_suppressed() {
            return true;
        }
        self.previewable && self.reopenable
    }
}

/// The chip set rendered for one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationChipSet {
    /// Freshness chip.
    pub freshness: ValidationFreshness,
    /// Version-match chip (mirrors the scope version match).
    pub version_match: ValidationVersionMatch,
    /// Locality chip.
    pub locality: ValidationLocality,
}

/// One validation row — one bounded typed validation record for an example/link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReportRow {
    /// Stable row id within this packet.
    pub row_id: String,
    /// The concrete subject (doc/anchor/label).
    pub subject: ValidationSubject,
    /// The validation mode applied to this row.
    pub mode: ValidationMode,
    /// The outcome recorded (must agree with the mode).
    pub outcome: ValidationOutcome,
    /// When the row was last checked (RFC 3339).
    pub last_checked_at: String,
    /// The environment/version scope the row was validated under.
    pub scope: ValidationScope,
    /// The validator and execution context that produced the row.
    pub produced_by: ValidationProducer,
    /// Freshness/version/locality chips.
    pub chips: ValidationChipSet,
    /// The evidence-provenance disclosure for the row.
    pub provenance: ValidationEvidenceProvenance,
    /// Human-readable provenance disclosure note.
    pub provenance_disclosure_note: String,
    /// Ref to the source/evidence trace (the failing source, the link target, or
    /// the drifted source the example no longer matches) — required for failing,
    /// broken-link, stale, and unsupported rows; no raw body.
    pub source_trace_ref: String,
    /// The action set (snippet / failing-source / compare / suppress / rerun).
    pub actions: ValidationActionSet,
    /// The durable suppression record.
    pub suppression: ValidationSuppression,
    /// Human-readable detail / summary (no raw bodies).
    pub detail: String,
    /// Whether the row is cited back to its source.
    pub cited: bool,
    /// Citation ref when cited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_ref: Option<String>,
}

impl ValidationReportRow {
    /// Whether this row is an actionable finding that demands a source/evidence
    /// trace (a failing outcome or an attention mode).
    pub fn requires_source_trace(&self) -> bool {
        self.outcome.is_failing() || self.mode.is_attention_finding()
    }
}

/// One export row, mirroring a validation row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationReportExportRow {
    /// The row this export row mirrors.
    pub row_id_ref: String,
    /// Subject kind (must match the row).
    pub subject_kind: ValidationSubjectKind,
    /// Validation mode (must match the row).
    pub mode: ValidationMode,
    /// Outcome (must match the row).
    pub outcome: ValidationOutcome,
    /// Last-checked time (must match the row).
    pub last_checked_at: String,
    /// Environment label (must match the row's scope).
    pub environment_label: String,
    /// Version match (must match the row's scope).
    pub version_match: ValidationVersionMatch,
    /// Freshness (must match the row's chip).
    pub freshness: ValidationFreshness,
    /// Provenance (must match the row).
    pub provenance: ValidationEvidenceProvenance,
    /// Producing validator (must match the row).
    pub produced_by: ValidatorKind,
    /// Suppression state (must match the row).
    pub suppression_state: ValidationSuppressionState,
    /// Whether the row keeps full action parity.
    pub action_parity_complete: bool,
    /// Whether the row is cited.
    pub cited: bool,
}

/// The docs-validation-report export projection for the row set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsValidationReportExport {
    /// Scope this export covers.
    pub scope: ValidationExportScope,
    /// Whether the export preserves each row's validation mode.
    pub preserves_mode: bool,
    /// Whether the export preserves each row's outcome.
    pub preserves_outcome: bool,
    /// Whether the export preserves each row's last-checked time.
    pub preserves_last_checked: bool,
    /// Whether the export preserves each row's environment/version scope.
    pub preserves_scope: bool,
    /// Whether the export preserves each row's freshness label.
    pub preserves_freshness: bool,
    /// Whether the export preserves each row's provenance.
    pub preserves_provenance: bool,
    /// Whether the export preserves each row's producing validator.
    pub preserves_producer: bool,
    /// Whether the export preserves the full action parity.
    pub preserves_action_parity: bool,
    /// Whether the export preserves each row's suppression state.
    pub preserves_suppression: bool,
    /// Per-row export rows.
    pub rows: Vec<ValidationReportExportRow>,
}

impl DocsValidationReportExport {
    /// Whether the export preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_mode
            && self.preserves_outcome
            && self.preserves_last_checked
            && self.preserves_scope
            && self.preserves_freshness
            && self.preserves_provenance
            && self.preserves_producer
            && self.preserves_action_parity
            && self.preserves_suppression
    }
}

/// A packet-level docs-validation degradation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationDegradation {
    /// Degradation class.
    pub degradation_class: ValidationDegradationClass,
    /// Severity.
    pub severity: ValidationFindingSeverity,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// The row this degradation annotates, if scoped to one row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub row_id_ref: Option<String>,
    /// Optional supporting evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// How a consumer surface projects the docs-validation row set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationConsumerProjection {
    /// Surface that consumes the set.
    pub surface: ValidationConsumerSurface,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface preserves the validation modes.
    pub preserves_modes: bool,
    /// Whether the surface preserves the outcomes.
    pub preserves_outcomes: bool,
    /// Whether the surface preserves the last-checked times.
    pub preserves_last_checked: bool,
    /// Whether the surface preserves the environment/version scope.
    pub preserves_scope: bool,
    /// Whether the surface preserves the chip set.
    pub preserves_chips: bool,
    /// Whether the surface preserves the provenance disclosures.
    pub preserves_provenance: bool,
    /// Whether the surface preserves the producing validators.
    pub preserves_producer: bool,
    /// Whether the surface preserves the full action parity.
    pub preserves_action_parity: bool,
    /// Whether the surface preserves the suppression history.
    pub preserves_suppression: bool,
}

impl ValidationConsumerProjection {
    /// Whether the projection preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_modes
            && self.preserves_outcomes
            && self.preserves_last_checked
            && self.preserves_scope
            && self.preserves_chips
            && self.preserves_provenance
            && self.preserves_producer
            && self.preserves_action_parity
            && self.preserves_suppression
    }
}

/// A single validation finding on the docs-validation row set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Finding kind.
    pub finding_kind: ValidationFindingKind,
    /// Finding severity.
    pub severity: ValidationFindingSeverity,
    /// Human-readable summary.
    pub summary: String,
}

/// Constructor input for [`DocsValidationReportPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsValidationReportPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label (no raw URLs / no raw bodies).
    pub report_label: String,
    /// Opaque digest/ref for the report run.
    pub report_digest_ref: String,
    /// The validation rows.
    pub rows: Vec<ValidationReportRow>,
    /// The export projection.
    pub export: DocsValidationReportExport,
    /// Packet-level degradations.
    pub report_degradations: Vec<ValidationDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<ValidationConsumerProjection>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// Export-safe docs-validation-report packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsValidationReportPacket {
    /// Record kind; must equal [`DOCS_VALIDATION_REPORT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_VALIDATION_REPORT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable report label.
    pub report_label: String,
    /// Opaque digest/ref for the report run.
    pub report_digest_ref: String,
    /// The validation rows.
    pub rows: Vec<ValidationReportRow>,
    /// The export projection.
    pub export: DocsValidationReportExport,
    /// Packet-level degradations.
    pub report_degradations: Vec<ValidationDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<ValidationConsumerProjection>,
    /// Computed promotion state.
    pub promotion_state: ValidationPromotionState,
    /// Computed validation findings.
    pub validation_findings: Vec<ValidationFinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Required consumer surfaces that every docs-validation packet must project:
/// the report itself, the review lane, the release-facing docs lane, and the
/// support export.
const REQUIRED_SURFACES: [ValidationConsumerSurface; 4] = [
    ValidationConsumerSurface::DocsValidationReport,
    ValidationConsumerSurface::DocsReviewPanel,
    ValidationConsumerSurface::ReleaseCenter,
    ValidationConsumerSurface::SupportExport,
];

impl DocsValidationReportPacket {
    /// Materializes a docs-validation-report packet, computing validation
    /// findings and the promotion state from the input.
    pub fn materialize(input: DocsValidationReportPacketInput) -> Self {
        let mut findings = Vec::new();

        check_identity(&input, &mut findings);
        check_rows(&input, &mut findings);
        check_export(&input, &mut findings);
        check_degradations(&input, &mut findings);
        check_consumer_projections(&input, &mut findings);
        check_boundary(&input, &mut findings);

        let promotion_state = promotion_state(&findings, &input.report_degradations);

        Self {
            record_kind: DOCS_VALIDATION_REPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_VALIDATION_REPORT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            report_label: input.report_label,
            report_digest_ref: input.report_digest_ref,
            rows: input.rows,
            export: input.export,
            report_degradations: input.report_degradations,
            consumer_projections: input.consumer_projections,
            promotion_state,
            validation_findings: findings,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Whether the report qualifies for the Stable claim with no findings.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == ValidationPromotionState::Stable
            && self.validation_findings.is_empty()
    }

    /// Wraps the packet in a support-export envelope.
    pub fn support_export(
        &self,
        export_id: &str,
        exported_at: &str,
    ) -> DocsValidationReportSupportExport {
        DocsValidationReportSupportExport {
            record_kind: DOCS_VALIDATION_REPORT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_VALIDATION_REPORT_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            schema_ref: DOCS_VALIDATION_REPORT_SCHEMA_REF.to_owned(),
            doc_ref: DOCS_VALIDATION_REPORT_DOC_REF.to_owned(),
            packet: self.clone(),
        }
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("docs-validation-report packet serializes")
    }

    /// Deterministic Markdown summary for docs, support, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Docs Validation Report (example/link validation rows)\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Report: {}\n", self.report_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Rows: {} | Degradations: {}\n",
            self.rows.len(),
            self.report_degradations.len()
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            let anchor = row
                .subject
                .snippet_anchor
                .as_deref()
                .map(|anchor| format!("#{anchor}"))
                .unwrap_or_default();
            out.push_str(&format!(
                "- [{}] `{}` ({}) — subject `{}{}`\n",
                row.subject.subject_kind.as_str(),
                row.row_id,
                row.subject.label,
                row.subject.display_path,
                anchor,
            ));
            out.push_str(&format!(
                "  - Mode/outcome: `{}` / `{}` — last checked {}\n",
                row.mode.as_str(),
                row.outcome.as_str(),
                row.last_checked_at,
            ));
            out.push_str(&format!(
                "  - Scope: {} | toolchain `{}` | target `{}` | version `{}`\n",
                row.scope.environment_label,
                row.scope.toolchain_ref,
                row.scope.target_version_ref,
                row.scope.version_match.as_str(),
            ));
            out.push_str(&format!(
                "  - Produced by: `{}` ({}) | chips {} / {} / {}\n",
                row.produced_by.validator.as_str(),
                row.produced_by.execution_context_ref,
                row.chips.freshness.as_str(),
                row.chips.version_match.as_str(),
                row.chips.locality.as_str(),
            ));
            out.push_str(&format!(
                "  - Actions: snippet `{}` | failing-source `{}` | compare `{}` | suppress {} | rerun {}\n",
                row.actions.open_snippet_ref,
                row.actions.open_failing_source_ref,
                row.actions.compare_current_source_ref,
                row.actions.suppress_available,
                row.actions.rerun_available,
            ));
            out.push_str(&format!(
                "  - Provenance: `{}` | trace `{}` | suppression `{}` | cited {}\n",
                row.provenance.as_str(),
                row.source_trace_ref,
                row.suppression.state.as_str(),
                row.cited,
            ));
        }
        if !self.report_degradations.is_empty() {
            out.push_str("\n## Degradations\n\n");
            for degradation in &self.report_degradations {
                out.push_str(&format!(
                    "- [{}/{}]: {}\n",
                    degradation.degradation_class.as_str(),
                    degradation.severity.as_str(),
                    degradation.summary,
                ));
            }
        }
        out
    }
}

/// Support-export envelope for the docs-validation-report packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsValidationReportSupportExport {
    /// Record kind; must equal [`DOCS_VALIDATION_REPORT_SUPPORT_EXPORT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Contract doc ref.
    pub doc_ref: String,
    /// The wrapped docs-validation-report packet.
    pub packet: DocsValidationReportPacket,
}

/// Errors emitted when reading the checked-in docs-validation-report support export.
#[derive(Debug)]
pub enum DocsValidationReportArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Re-materialization disagreed with the checked-in promotion state.
    PromotionDrift {
        /// Promotion state recorded in the export.
        recorded: ValidationPromotionState,
        /// Promotion state computed by re-materialization.
        computed: ValidationPromotionState,
    },
    /// The checked-in packet should be clean Stable but is not.
    NotCleanStable(Vec<ValidationFinding>),
}

impl fmt::Display for DocsValidationReportArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "docs-validation-report export parse failed: {error}"
                )
            }
            Self::PromotionDrift { recorded, computed } => write!(
                formatter,
                "docs-validation-report promotion drift: recorded {} but computed {}",
                recorded.as_str(),
                computed.as_str()
            ),
            Self::NotCleanStable(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "docs-validation-report export is not clean stable: {tokens}"
                )
            }
        }
    }
}

impl Error for DocsValidationReportArtifactError {}

/// Reads and re-validates the checked-in stable docs-validation-report support export.
pub fn current_stable_docs_validation_report_export(
) -> Result<DocsValidationReportSupportExport, DocsValidationReportArtifactError> {
    let export: DocsValidationReportSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/docs-validation-proof/support_export.json"
    )))
    .map_err(DocsValidationReportArtifactError::SupportExport)?;

    let recomputed = DocsValidationReportPacket::materialize(packet_to_input(&export.packet));
    if recomputed.promotion_state != export.packet.promotion_state {
        return Err(DocsValidationReportArtifactError::PromotionDrift {
            recorded: export.packet.promotion_state,
            computed: recomputed.promotion_state,
        });
    }
    if !export.packet.is_clean_stable() {
        return Err(DocsValidationReportArtifactError::NotCleanStable(
            export.packet.validation_findings.clone(),
        ));
    }
    Ok(export)
}

/// Rebuilds the materialization input from a packet (used for re-validation).
pub fn packet_to_input(packet: &DocsValidationReportPacket) -> DocsValidationReportPacketInput {
    DocsValidationReportPacketInput {
        packet_id: packet.packet_id.clone(),
        report_label: packet.report_label.clone(),
        report_digest_ref: packet.report_digest_ref.clone(),
        rows: packet.rows.clone(),
        export: packet.export.clone(),
        report_degradations: packet.report_degradations.clone(),
        consumer_projections: packet.consumer_projections.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    }
}

fn push_finding(
    findings: &mut Vec<ValidationFinding>,
    kind: ValidationFindingKind,
    summary: impl Into<String>,
) {
    findings.push(ValidationFinding {
        finding_kind: kind,
        severity: kind.default_severity(),
        summary: summary.into(),
    });
}

fn check_identity(input: &DocsValidationReportPacketInput, findings: &mut Vec<ValidationFinding>) {
    if input.packet_id.trim().is_empty()
        || input.report_label.trim().is_empty()
        || input.report_digest_ref.trim().is_empty()
        || input.redaction_class_token.trim().is_empty()
        || input.minted_at.trim().is_empty()
    {
        push_finding(
            findings,
            ValidationFindingKind::MissingIdentity,
            "packet identity fields must all be present",
        );
    }
}

fn check_rows(input: &DocsValidationReportPacketInput, findings: &mut Vec<ValidationFinding>) {
    if input.rows.is_empty() {
        push_finding(
            findings,
            ValidationFindingKind::RowsEmpty,
            "the docs-validation report must carry at least one row",
        );
        return;
    }

    let present_kinds: BTreeSet<ValidationSubjectKind> = input
        .rows
        .iter()
        .map(|row| row.subject.subject_kind)
        .collect();
    for required in ValidationSubjectKind::REQUIRED {
        if !present_kinds.contains(&required) {
            push_finding(
                findings,
                ValidationFindingKind::RequiredSubjectKindMissing,
                format!("required subject kind `{}` is missing", required.as_str()),
            );
        }
    }

    let present_modes: BTreeSet<ValidationMode> = input.rows.iter().map(|row| row.mode).collect();
    for required in ValidationMode::REQUIRED {
        if !present_modes.contains(&required) {
            push_finding(
                findings,
                ValidationFindingKind::RequiredModeCoverageMissing,
                format!(
                    "required validation mode `{}` is missing",
                    required.as_str()
                ),
            );
        }
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for row in &input.rows {
        if !seen_ids.insert(row.row_id.as_str()) {
            push_finding(
                findings,
                ValidationFindingKind::DuplicateRowId,
                format!("duplicate row id `{}`", row.row_id),
            );
        }
        check_one_row(row, findings);
    }
}

fn check_one_row(row: &ValidationReportRow, findings: &mut Vec<ValidationFinding>) {
    let id = &row.row_id;

    // Concrete subject.
    if !row.subject.names_concrete_subject() {
        push_finding(
            findings,
            ValidationFindingKind::SubjectIdentityMissing,
            format!("row `{id}` must name a concrete subject doc/anchor/label"),
        );
    }
    if row.detail.trim().is_empty() {
        push_finding(
            findings,
            ValidationFindingKind::DetailMissing,
            format!("row `{id}` is missing its detail"),
        );
    }

    // Explicit last-checked time and environment/version scope.
    if row.last_checked_at.trim().is_empty() {
        push_finding(
            findings,
            ValidationFindingKind::LastCheckedMissing,
            format!("row `{id}` is missing its last-checked time"),
        );
    }
    if !row.scope.is_complete() {
        push_finding(
            findings,
            ValidationFindingKind::EnvironmentScopeMissing,
            format!("row `{id}` is missing part of its environment/version scope"),
        );
    }

    // Producer attribution.
    if !row.produced_by.names_concrete_context() {
        push_finding(
            findings,
            ValidationFindingKind::ProducerContextMissing,
            format!("row `{id}` is missing its producing-validator execution context"),
        );
    }
    if !row.mode.permits_validator(row.produced_by.validator) {
        push_finding(
            findings,
            ValidationFindingKind::ProducerValidatorModeMismatch,
            format!(
                "row `{id}` mode `{}` was not produced by a permitted validator (`{}`)",
                row.mode.as_str(),
                row.produced_by.validator.as_str()
            ),
        );
    }
    if !row.actions.preserves_producer {
        push_finding(
            findings,
            ValidationFindingKind::ProducerNotPreserved,
            format!("row `{id}` actions must preserve the producing validator/context"),
        );
    }

    // Mode/outcome consistency — a rendered/skipped/stale row may never claim an
    // executed result.
    if row.outcome.claims_execution() && !row.mode.is_executed() {
        push_finding(
            findings,
            ValidationFindingKind::ExecutionClaimWithoutRun,
            format!(
                "row `{id}` outcome `{}` claims execution but mode `{}` did not run the example",
                row.outcome.as_str(),
                row.mode.as_str()
            ),
        );
    } else if !row.mode.outcome_is_consistent(row.outcome) {
        push_finding(
            findings,
            ValidationFindingKind::ModeOutcomeInconsistent,
            format!(
                "row `{id}` outcome `{}` is inconsistent with mode `{}`",
                row.outcome.as_str(),
                row.mode.as_str()
            ),
        );
    }

    // Provenance / cached-truth visibility.
    if row.provenance_disclosure_note.trim().is_empty() {
        push_finding(
            findings,
            ValidationFindingKind::ProvenanceDisclosureMissing,
            format!("row `{id}` is missing its provenance disclosure"),
        );
    }
    if !row.provenance.is_authoritative()
        && row.outcome.claims_execution_pass()
        && row.chips.freshness.is_authoritative_live()
    {
        push_finding(
            findings,
            ValidationFindingKind::ResultTruthCollapsed,
            format!(
                "row `{id}` is `{}` but presented as an authoritative live executed pass",
                row.provenance.as_str()
            ),
        );
    }
    if row.provenance.needs_citation() && !row.cited {
        push_finding(
            findings,
            ValidationFindingKind::RowNotCited,
            format!(
                "row `{id}` is `{}` but is not cited",
                row.provenance.as_str()
            ),
        );
    }
    if !row.chips.version_match.is_confident_current()
        && row.outcome.claims_execution_pass()
        && row.chips.freshness.is_authoritative_live()
    {
        push_finding(
            findings,
            ValidationFindingKind::VersionTruthCollapsed,
            format!(
                "row `{id}` presents version `{}` as a confident live executed pass",
                row.chips.version_match.as_str()
            ),
        );
    }

    // Actionable, traced findings.
    if row.requires_source_trace() && row.source_trace_ref.trim().is_empty() {
        push_finding(
            findings,
            ValidationFindingKind::FindingNotTraced,
            format!(
                "row `{id}` ({} / {}) is an actionable finding but carries no source trace",
                row.mode.as_str(),
                row.outcome.as_str()
            ),
        );
    }

    // Action parity.
    if !row.actions.parity_complete() {
        push_finding(
            findings,
            ValidationFindingKind::ActionParityIncomplete,
            format!(
                "row `{id}` must keep open-snippet, open-failing-source, compare-current-source, suppress, and rerun parity"
            ),
        );
    }

    // Durable, honest suppression.
    if !row.suppression.is_attributable() {
        push_finding(
            findings,
            ValidationFindingKind::SuppressionNotAttributable,
            format!(
                "row `{id}` suppression `{}` must carry attribution and a durable history ref",
                row.suppression.state.as_str()
            ),
        );
    }
    if !row.suppression.is_reopenable() {
        push_finding(
            findings,
            ValidationFindingKind::SuppressionNotReopenable,
            format!(
                "row `{id}` suppression `{}` must stay previewable and reopenable",
                row.suppression.state.as_str()
            ),
        );
    }
}

fn check_export(input: &DocsValidationReportPacketInput, findings: &mut Vec<ValidationFinding>) {
    let export = &input.export;
    if !export.preserves_all() {
        push_finding(
            findings,
            ValidationFindingKind::ExportDropsPreservation,
            "the export must preserve mode, outcome, last-checked, scope, freshness, provenance, producer, action parity, and suppression",
        );
    }

    let mut export_ids: BTreeSet<&str> = BTreeSet::new();
    for row in &export.rows {
        export_ids.insert(row.row_id_ref.as_str());
        let source = input.rows.iter().find(|r| r.row_id == row.row_id_ref);
        match source {
            None => push_finding(
                findings,
                ValidationFindingKind::ExportRowOrphan,
                format!("export row references unknown row `{}`", row.row_id_ref),
            ),
            Some(source) => check_export_row(source, row, findings),
        }
    }

    for row in &input.rows {
        if !export_ids.contains(row.row_id.as_str()) {
            push_finding(
                findings,
                ValidationFindingKind::ExportCoverageMissing,
                format!("row `{}` has no export row", row.row_id),
            );
        }
    }
}

fn check_export_row(
    source: &ValidationReportRow,
    row: &ValidationReportExportRow,
    findings: &mut Vec<ValidationFinding>,
) {
    let id = &row.row_id_ref;
    if source.subject.subject_kind != row.subject_kind {
        push_finding(
            findings,
            ValidationFindingKind::ExportModeMismatch,
            format!(
                "export for `{id}` records subject kind `{}` but the row is `{}`",
                row.subject_kind.as_str(),
                source.subject.subject_kind.as_str()
            ),
        );
    }
    if source.mode != row.mode {
        push_finding(
            findings,
            ValidationFindingKind::ExportModeMismatch,
            format!(
                "export for `{id}` records mode `{}` but the row is `{}`",
                row.mode.as_str(),
                source.mode.as_str()
            ),
        );
    }
    if source.outcome != row.outcome {
        push_finding(
            findings,
            ValidationFindingKind::ExportOutcomeMismatch,
            format!(
                "export for `{id}` records outcome `{}` but the row is `{}`",
                row.outcome.as_str(),
                source.outcome.as_str()
            ),
        );
    }
    if source.last_checked_at != row.last_checked_at {
        push_finding(
            findings,
            ValidationFindingKind::ExportLastCheckedMismatch,
            format!("export for `{id}` records a different last-checked time than the row"),
        );
    }
    if source.scope.environment_label != row.environment_label
        || source.scope.version_match != row.version_match
    {
        push_finding(
            findings,
            ValidationFindingKind::ExportScopeMismatch,
            format!("export for `{id}` records a different environment/version scope than the row"),
        );
    }
    if source.chips.freshness != row.freshness {
        push_finding(
            findings,
            ValidationFindingKind::ExportFreshnessMismatch,
            format!(
                "export for `{id}` records freshness `{}` but the chip is `{}`",
                row.freshness.as_str(),
                source.chips.freshness.as_str()
            ),
        );
    }
    if source.provenance != row.provenance {
        push_finding(
            findings,
            ValidationFindingKind::ExportProvenanceMismatch,
            format!(
                "export for `{id}` records provenance `{}` but the row is `{}`",
                row.provenance.as_str(),
                source.provenance.as_str()
            ),
        );
    }
    if source.produced_by.validator != row.produced_by {
        push_finding(
            findings,
            ValidationFindingKind::ExportProducerMismatch,
            format!(
                "export for `{id}` records producer `{}` but the row is `{}`",
                row.produced_by.as_str(),
                source.produced_by.validator.as_str()
            ),
        );
    }
    if source.suppression.state != row.suppression_state {
        push_finding(
            findings,
            ValidationFindingKind::ExportSuppressionMismatch,
            format!(
                "export for `{id}` records suppression `{}` but the row is `{}`",
                row.suppression_state.as_str(),
                source.suppression.state.as_str()
            ),
        );
    }
    if source.actions.parity_complete() != row.action_parity_complete {
        push_finding(
            findings,
            ValidationFindingKind::ExportActionParityMismatch,
            format!(
                "export for `{id}` records action parity `{}` but the row is `{}`",
                row.action_parity_complete,
                source.actions.parity_complete()
            ),
        );
    }
    if source.cited != row.cited {
        push_finding(
            findings,
            ValidationFindingKind::ExportCitedMismatch,
            format!(
                "export for `{id}` records cited `{}` but the row is `{}`",
                row.cited, source.cited
            ),
        );
    }
}

fn check_degradations(
    input: &DocsValidationReportPacketInput,
    findings: &mut Vec<ValidationFinding>,
) {
    let row_ids: BTreeSet<&str> = input.rows.iter().map(|row| row.row_id.as_str()).collect();

    for degradation in &input.report_degradations {
        if degradation.summary.trim().is_empty() {
            push_finding(
                findings,
                ValidationFindingKind::DegradationIncomplete,
                format!(
                    "degradation `{}` is missing a summary",
                    degradation.degradation_class.as_str()
                ),
            );
        }
        if let Some(row_id) = &degradation.row_id_ref {
            if !row_id.trim().is_empty() && !row_ids.contains(row_id.as_str()) {
                push_finding(
                    findings,
                    ValidationFindingKind::DegradationOrphan,
                    format!("degradation references unknown row `{row_id}`"),
                );
            }
        }
    }
}

fn check_consumer_projections(
    input: &DocsValidationReportPacketInput,
    findings: &mut Vec<ValidationFinding>,
) {
    let present: BTreeSet<ValidationConsumerSurface> = input
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    for required in REQUIRED_SURFACES {
        if !present.contains(&required) {
            push_finding(
                findings,
                ValidationFindingKind::RequiredSurfaceCoverageMissing,
                format!("required surface `{}` is missing", required.as_str()),
            );
        }
    }

    for projection in &input.consumer_projections {
        if projection.packet_id_ref != input.packet_id {
            push_finding(
                findings,
                ValidationFindingKind::ConsumerProjectionPacketIdMismatch,
                format!(
                    "surface `{}` references packet `{}`",
                    projection.surface.as_str(),
                    projection.packet_id_ref
                ),
            );
        }
        if !projection.preserves_all() {
            push_finding(
                findings,
                ValidationFindingKind::ConsumerProjectionDrift,
                format!(
                    "surface `{}` drops a required preservation flag",
                    projection.surface.as_str()
                ),
            );
        }
    }
}

fn check_boundary(input: &DocsValidationReportPacketInput, findings: &mut Vec<ValidationFinding>) {
    let value = serde_json::to_value(input).expect("docs-validation-report input serializes");
    if json_contains_forbidden_boundary_material(&value) {
        push_finding(
            findings,
            ValidationFindingKind::RawBoundaryMaterialPresent,
            "export must not carry raw bodies, raw URLs, execution logs, or secrets",
        );
    }
}

/// Computes the promotion state from the worst severity across the validation
/// findings and the attached degradations.
///
/// A blocking validation finding (unlabeled mode, mode/outcome mismatch, missing
/// last-checked time or scope, untraced finding, collapsed truth, misattributed
/// producer, non-reopenable suppression, or boundary violation) blocks the Stable
/// claim; an otherwise-clean report whose degradations carry a narrowing severity
/// narrows below Stable rather than hiding the rows.
fn promotion_state(
    findings: &[ValidationFinding],
    degradations: &[ValidationDegradation],
) -> ValidationPromotionState {
    let any_blocking = findings
        .iter()
        .any(|finding| finding.severity == ValidationFindingSeverity::Blocking)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == ValidationFindingSeverity::Blocking);
    if any_blocking {
        return ValidationPromotionState::BlocksStable;
    }

    let any_narrowing = findings
        .iter()
        .any(|finding| finding.severity == ValidationFindingSeverity::Narrowing)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == ValidationFindingSeverity::Narrowing);
    if any_narrowing {
        ValidationPromotionState::NarrowedBelowStable
    } else {
        ValidationPromotionState::Stable
    }
}

/// Heuristic that rejects obviously forbidden material in the export.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("raw_body:")
                || lower.contains("raw_url:")
                || lower.contains("exec_log:")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Seeded stable docs-validation-report input used by the producer, tests, and
/// fixtures.
pub fn seeded_stable_docs_validation_report_input() -> DocsValidationReportPacketInput {
    let packet_id = "packet:m5:docs_validation_report:retry_backoff_release".to_owned();
    DocsValidationReportPacketInput {
        packet_id: packet_id.clone(),
        report_label: "docs validation report: the retry/backoff release docs sweep".to_owned(),
        report_digest_ref: "reportdigest:sha256:retry-backoff-release-validation".to_owned(),
        rows: seeded_rows(),
        export: seeded_export(),
        report_degradations: vec![ValidationDegradation {
            degradation_class: ValidationDegradationClass::LinkCheckerOffline,
            severity: ValidationFindingSeverity::Advisory,
            summary: "the live link checker was offline for one external host; the broken-link row is served from the last snapshot".to_owned(),
            row_id_ref: Some("row:tutorial:runbook_broken_link".to_owned()),
            evidence_ref: Some("evidence:docs-validation-report:link-checker-state".to_owned()),
        }],
        consumer_projections: required_projections(&packet_id),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-12T00:00:00Z".to_owned(),
    }
}

fn active_suppression(note: &str) -> ValidationSuppression {
    ValidationSuppression {
        state: ValidationSuppressionState::Active,
        attributed_to_ref: None,
        history_ref: None,
        previewable: true,
        reopenable: true,
        note: note.to_owned(),
    }
}

fn seeded_rows() -> Vec<ValidationReportRow> {
    vec![
        readme_executed_local_row(),
        tutorial_rendered_preview_row(),
        help_syntax_checked_row(),
        guide_executed_remote_row(),
        tutorial_broken_link_row(),
        readme_stale_example_row(),
        help_skipped_row(),
        guide_unsupported_row(),
    ]
}

fn readme_executed_local_row() -> ValidationReportRow {
    ValidationReportRow {
        row_id: "row:readme:config_example_executed_local".to_owned(),
        subject: ValidationSubject {
            subject_kind: ValidationSubjectKind::CodeExample,
            doc_ref: "docs/guides/retry_with_backoff/README.md".to_owned(),
            snippet_anchor: Some("configuration".to_owned()),
            display_path: "README → Configuration → max_elapsed example".to_owned(),
            label: "the retry_with_backoff configuration example".to_owned(),
        },
        mode: ValidationMode::ExecutedLocal,
        outcome: ValidationOutcome::ExecutedPass,
        last_checked_at: "2026-06-11T22:14:00Z".to_owned(),
        scope: ValidationScope {
            environment_label: "rustc 1.84.0 on x86_64-unknown-linux-gnu".to_owned(),
            toolchain_ref: "toolchain:rust-1.84.0:x86_64-unknown-linux-gnu".to_owned(),
            target_version_ref: "source:crates/aureline-net/src/retry.rs@workspace-rev".to_owned(),
            version_match: ValidationVersionMatch::ExactBuildMatch,
        },
        produced_by: ValidationProducer {
            validator: ValidatorKind::LocalExampleHarness,
            execution_context_ref: "exec-context:local-harness:rust-1.84.0".to_owned(),
            detail: "compiled and ran the configuration example in the local example harness".to_owned(),
        },
        chips: ValidationChipSet {
            freshness: ValidationFreshness::AuthoritativeLive,
            version_match: ValidationVersionMatch::ExactBuildMatch,
            locality: ValidationLocality::Local,
        },
        provenance: ValidationEvidenceProvenance::FirstPartyVerified,
        provenance_disclosure_note: "first-party evidence: the example was compiled and executed locally against the in-repo source at the active revision".to_owned(),
        source_trace_ref: "open-source:repo:crates/aureline-net/src/retry.rs#with_backoff".to_owned(),
        actions: ValidationActionSet {
            open_snippet_ref: "open-snippet:docs/guides/retry_with_backoff/README.md#configuration".to_owned(),
            open_failing_source_ref: "open-source:repo:crates/aureline-net/src/retry.rs#with_backoff".to_owned(),
            compare_current_source_ref: "compare:readme-config-example:current-source".to_owned(),
            suppress_available: true,
            rerun_available: true,
            preserves_producer: true,
        },
        suppression: active_suppression("active; the executed example passes against the current source"),
        detail: "the README configuration example compiles and runs against the current retry source".to_owned(),
        cited: true,
        citation_ref: Some("cite:executed-local:readme-config-example".to_owned()),
    }
}

fn tutorial_rendered_preview_row() -> ValidationReportRow {
    ValidationReportRow {
        row_id: "row:tutorial:overview_rendered_preview".to_owned(),
        subject: ValidationSubject {
            subject_kind: ValidationSubjectKind::CodeExample,
            doc_ref: "docs/tutorials/resilient-networking.md".to_owned(),
            snippet_anchor: Some("overview-diagram".to_owned()),
            display_path: "Tutorial → Resilient networking → Overview".to_owned(),
            label: "the overview pseudo-code block".to_owned(),
        },
        mode: ValidationMode::Rendered,
        outcome: ValidationOutcome::RenderedPreviewOnly,
        last_checked_at: "2026-06-11T22:15:00Z".to_owned(),
        scope: ValidationScope {
            environment_label: "rendered-preview engine (no execution)".to_owned(),
            toolchain_ref: "render-engine:commonmark-safe".to_owned(),
            target_version_ref: "doc:docs/tutorials/resilient-networking.md@workspace-rev".to_owned(),
            version_match: ValidationVersionMatch::ExactBuildMatch,
        },
        produced_by: ValidationProducer {
            validator: ValidatorKind::RenderedPreviewEngine,
            execution_context_ref: "exec-context:render-preview:commonmark-safe".to_owned(),
            detail: "rendered the overview block for preview only; it is illustrative pseudo-code and is not executed".to_owned(),
        },
        chips: ValidationChipSet {
            freshness: ValidationFreshness::AuthoritativeLive,
            version_match: ValidationVersionMatch::ExactBuildMatch,
            locality: ValidationLocality::Local,
        },
        provenance: ValidationEvidenceProvenance::FirstPartyVerified,
        provenance_disclosure_note: "rendered preview only — this block is illustrative pseudo-code and is explicitly not an executed or syntax-checked example".to_owned(),
        source_trace_ref: "open-snippet:docs/tutorials/resilient-networking.md#overview-diagram".to_owned(),
        actions: ValidationActionSet {
            open_snippet_ref: "open-snippet:docs/tutorials/resilient-networking.md#overview-diagram".to_owned(),
            open_failing_source_ref: "open-source:doc:docs/tutorials/resilient-networking.md#overview-diagram".to_owned(),
            compare_current_source_ref: "compare:tutorial-overview:rendered-only".to_owned(),
            suppress_available: true,
            rerun_available: true,
            preserves_producer: true,
        },
        suppression: active_suppression("active; the block is rendered for preview only and makes no execution claim"),
        detail: "the tutorial overview block is rendered for preview only and is not presented as an executed example".to_owned(),
        cited: true,
        citation_ref: Some("cite:rendered:tutorial-overview".to_owned()),
    }
}

fn help_syntax_checked_row() -> ValidationReportRow {
    ValidationReportRow {
        row_id: "row:help:builder_syntax_checked".to_owned(),
        subject: ValidationSubject {
            subject_kind: ValidationSubjectKind::CodeExample,
            doc_ref: "docs/help/retry-and-backoff.md".to_owned(),
            snippet_anchor: Some("builder-api".to_owned()),
            display_path: "Help → Retry and backoff → Builder API".to_owned(),
            label: "the builder-API example".to_owned(),
        },
        mode: ValidationMode::SyntaxChecked,
        outcome: ValidationOutcome::SyntaxValid,
        last_checked_at: "2026-06-11T22:16:00Z".to_owned(),
        scope: ValidationScope {
            environment_label: "rustc 1.84.0 parse-only (no execution)".to_owned(),
            toolchain_ref: "toolchain:rust-1.84.0:parse-only".to_owned(),
            target_version_ref: "source:crates/aureline-net/src/retry.rs@workspace-rev".to_owned(),
            version_match: ValidationVersionMatch::ExactBuildMatch,
        },
        produced_by: ValidationProducer {
            validator: ValidatorKind::SyntaxChecker,
            execution_context_ref: "exec-context:syntax-checker:rust-1.84.0".to_owned(),
            detail: "parsed and type-checked the builder example without executing it".to_owned(),
        },
        chips: ValidationChipSet {
            freshness: ValidationFreshness::AuthoritativeLive,
            version_match: ValidationVersionMatch::ExactBuildMatch,
            locality: ValidationLocality::Local,
        },
        provenance: ValidationEvidenceProvenance::FirstPartyVerified,
        provenance_disclosure_note: "syntax-checked only — the example parses against the current API but was not executed".to_owned(),
        source_trace_ref: "open-source:repo:crates/aureline-net/src/retry.rs#builder".to_owned(),
        actions: ValidationActionSet {
            open_snippet_ref: "open-snippet:docs/help/retry-and-backoff.md#builder-api".to_owned(),
            open_failing_source_ref: "open-source:repo:crates/aureline-net/src/retry.rs#builder".to_owned(),
            compare_current_source_ref: "compare:help-builder:current-source".to_owned(),
            suppress_available: true,
            rerun_available: true,
            preserves_producer: true,
        },
        suppression: active_suppression("active; the example parses against the current builder API"),
        detail: "the help builder-API example is syntax-checked against the current API but not executed".to_owned(),
        cited: true,
        citation_ref: Some("cite:syntax-checked:help-builder".to_owned()),
    }
}

fn guide_executed_remote_row() -> ValidationReportRow {
    ValidationReportRow {
        row_id: "row:guide:cli_executed_remote".to_owned(),
        subject: ValidationSubject {
            subject_kind: ValidationSubjectKind::ShellExample,
            doc_ref: "docs/guides/retry_with_backoff/operations.md".to_owned(),
            snippet_anchor: Some("smoke-test".to_owned()),
            display_path: "Guide → Operations → Smoke test".to_owned(),
            label: "the operations smoke-test command".to_owned(),
        },
        mode: ValidationMode::ExecutedRemote,
        outcome: ValidationOutcome::PassedWithWarnings,
        last_checked_at: "2026-06-11T22:18:00Z".to_owned(),
        scope: ValidationScope {
            environment_label: "managed runner: ubuntu-24.04, aarch64".to_owned(),
            toolchain_ref: "toolchain:managed-runner:ubuntu-24.04-aarch64".to_owned(),
            target_version_ref: "release:next-channel@retry_backoff".to_owned(),
            version_match: ValidationVersionMatch::CompatibleMinorDrift,
        },
        produced_by: ValidationProducer {
            validator: ValidatorKind::RemoteExampleRunner,
            execution_context_ref: "exec-context:remote-runner:ubuntu-24.04-aarch64".to_owned(),
            detail: "ran the smoke-test command on the managed runner; it passed with a deprecation warning".to_owned(),
        },
        chips: ValidationChipSet {
            freshness: ValidationFreshness::WarmCached,
            version_match: ValidationVersionMatch::CompatibleMinorDrift,
            locality: ValidationLocality::Managed,
        },
        provenance: ValidationEvidenceProvenance::FirstPartyVerified,
        provenance_disclosure_note: "executed on the managed runner; passed with a deprecation warning that is surfaced rather than hidden".to_owned(),
        source_trace_ref: "open-source:repo:crates/aureline-net/examples/smoke.rs".to_owned(),
        actions: ValidationActionSet {
            open_snippet_ref: "open-snippet:docs/guides/retry_with_backoff/operations.md#smoke-test".to_owned(),
            open_failing_source_ref: "open-source:repo:crates/aureline-net/examples/smoke.rs".to_owned(),
            compare_current_source_ref: "compare:guide-smoke:current-source".to_owned(),
            suppress_available: true,
            rerun_available: true,
            preserves_producer: true,
        },
        suppression: active_suppression("active; the remote run passes with a disclosed deprecation warning"),
        detail: "the operations smoke-test ran on the managed runner and passed with a disclosed deprecation warning".to_owned(),
        cited: true,
        citation_ref: Some("cite:executed-remote:guide-smoke".to_owned()),
    }
}

fn tutorial_broken_link_row() -> ValidationReportRow {
    ValidationReportRow {
        row_id: "row:tutorial:runbook_broken_link".to_owned(),
        subject: ValidationSubject {
            subject_kind: ValidationSubjectKind::Link,
            doc_ref: "docs/tutorials/resilient-networking.md".to_owned(),
            snippet_anchor: Some("operations-runbook".to_owned()),
            display_path: "Tutorial → Resilient networking → Operations runbook link".to_owned(),
            label: "the operations runbook link".to_owned(),
        },
        mode: ValidationMode::BrokenLink,
        outcome: ValidationOutcome::LinkBroken,
        last_checked_at: "2026-06-11T22:20:00Z".to_owned(),
        scope: ValidationScope {
            environment_label: "link checker against the imported ops pack mirror".to_owned(),
            toolchain_ref: "link-checker:imported-ops-pack-mirror".to_owned(),
            target_version_ref: "pack:ops/runbooks@imported-rev".to_owned(),
            version_match: ValidationVersionMatch::CompatibleMinorDrift,
        },
        produced_by: ValidationProducer {
            validator: ValidatorKind::LinkChecker,
            execution_context_ref: "exec-context:link-checker:imported-ops-pack".to_owned(),
            detail: "the runbook link returned a 404 after the imported ops page was renamed".to_owned(),
        },
        chips: ValidationChipSet {
            freshness: ValidationFreshness::WarmCached,
            version_match: ValidationVersionMatch::CompatibleMinorDrift,
            locality: ValidationLocality::ImportedPack,
        },
        provenance: ValidationEvidenceProvenance::Imported,
        provenance_disclosure_note: "imported from the signed ops docs pack and served from the mirror; the broken-link result is held to cached freshness pending a live re-check".to_owned(),
        source_trace_ref: "open-failing-source:pack:ops/runbooks/retry_backoff_runbook.md".to_owned(),
        actions: ValidationActionSet {
            open_snippet_ref: "open-snippet:docs/tutorials/resilient-networking.md#operations-runbook".to_owned(),
            open_failing_source_ref: "open-failing-source:pack:ops/runbooks/retry_backoff_runbook.md".to_owned(),
            compare_current_source_ref: "compare:tutorial-runbook-link:current-target".to_owned(),
            suppress_available: true,
            rerun_available: true,
            preserves_producer: true,
        },
        suppression: active_suppression("active; the broken-link finding is open and traced to the renamed ops page"),
        detail: "the tutorial operations-runbook link is broken; the finding is traced to the renamed imported ops page".to_owned(),
        cited: true,
        citation_ref: Some("cite:broken-link:tutorial-runbook".to_owned()),
    }
}

fn readme_stale_example_row() -> ValidationReportRow {
    ValidationReportRow {
        row_id: "row:readme:jitter_stale_example".to_owned(),
        subject: ValidationSubject {
            subject_kind: ValidationSubjectKind::CodeExample,
            doc_ref: "docs/guides/retry_with_backoff/README.md".to_owned(),
            snippet_anchor: Some("jitter".to_owned()),
            display_path: "README → Jitter → with_jitter example".to_owned(),
            label: "the with_jitter example".to_owned(),
        },
        mode: ValidationMode::Stale,
        outcome: ValidationOutcome::NotRun,
        last_checked_at: "2026-05-30T09:00:00Z".to_owned(),
        scope: ValidationScope {
            environment_label: "rustc 1.84.0 on x86_64-unknown-linux-gnu (prior run)".to_owned(),
            toolchain_ref: "toolchain:rust-1.84.0:x86_64-unknown-linux-gnu".to_owned(),
            target_version_ref: "source:crates/aureline-net/src/retry.rs@prior-rev".to_owned(),
            version_match: ValidationVersionMatch::IncompatibleDriftDetected,
        },
        produced_by: ValidationProducer {
            validator: ValidatorKind::LocalExampleHarness,
            execution_context_ref: "exec-context:local-harness:prior-rev".to_owned(),
            detail: "the example last passed against a prior revision; the with_jitter symbol has since been renamed, so the result is stale".to_owned(),
        },
        chips: ValidationChipSet {
            freshness: ValidationFreshness::Stale,
            version_match: ValidationVersionMatch::IncompatibleDriftDetected,
            locality: ValidationLocality::Local,
        },
        provenance: ValidationEvidenceProvenance::Stale,
        provenance_disclosure_note: "the prior pass is stale: the source the example referenced has drifted, so the row is held to not-run pending a re-run".to_owned(),
        source_trace_ref: "open-failing-source:repo:crates/aureline-net/src/retry.rs#with_full_jitter".to_owned(),
        actions: ValidationActionSet {
            open_snippet_ref: "open-snippet:docs/guides/retry_with_backoff/README.md#jitter".to_owned(),
            open_failing_source_ref: "open-failing-source:repo:crates/aureline-net/src/retry.rs#with_full_jitter".to_owned(),
            compare_current_source_ref: "compare:readme-jitter:prior-vs-current-source".to_owned(),
            suppress_available: true,
            rerun_available: true,
            preserves_producer: true,
        },
        suppression: active_suppression("active; the stale-example finding is open and points at the renamed source"),
        detail: "the README with_jitter example is stale after the symbol rename; the finding is traced to the current source".to_owned(),
        cited: true,
        citation_ref: Some("cite:stale-example:readme-jitter".to_owned()),
    }
}

fn help_skipped_row() -> ValidationReportRow {
    ValidationReportRow {
        row_id: "row:help:network_skipped".to_owned(),
        subject: ValidationSubject {
            subject_kind: ValidationSubjectKind::ShellExample,
            doc_ref: "docs/help/retry-and-backoff.md".to_owned(),
            snippet_anchor: Some("live-endpoint".to_owned()),
            display_path: "Help → Retry and backoff → Live endpoint".to_owned(),
            label: "the live-endpoint smoke command".to_owned(),
        },
        mode: ValidationMode::Skipped,
        outcome: ValidationOutcome::NotRun,
        last_checked_at: "2026-06-11T22:22:00Z".to_owned(),
        scope: ValidationScope {
            environment_label: "local harness without network access".to_owned(),
            toolchain_ref: "toolchain:rust-1.84.0:offline".to_owned(),
            target_version_ref: "doc:docs/help/retry-and-backoff.md@workspace-rev".to_owned(),
            version_match: ValidationVersionMatch::ExactBuildMatch,
        },
        produced_by: ValidationProducer {
            validator: ValidatorKind::LocalExampleHarness,
            execution_context_ref: "exec-context:local-harness:offline".to_owned(),
            detail: "the command needs a live endpoint, so it was intentionally skipped offline".to_owned(),
        },
        chips: ValidationChipSet {
            freshness: ValidationFreshness::WarmCached,
            version_match: ValidationVersionMatch::ExactBuildMatch,
            locality: ValidationLocality::Local,
        },
        provenance: ValidationEvidenceProvenance::FirstPartyVerified,
        provenance_disclosure_note: "intentionally skipped: the example needs a live endpoint that is not available in the offline harness".to_owned(),
        source_trace_ref: "open-snippet:docs/help/retry-and-backoff.md#live-endpoint".to_owned(),
        actions: ValidationActionSet {
            open_snippet_ref: "open-snippet:docs/help/retry-and-backoff.md#live-endpoint".to_owned(),
            open_failing_source_ref: "open-source:doc:docs/help/retry-and-backoff.md#live-endpoint".to_owned(),
            compare_current_source_ref: "compare:help-live-endpoint:current-source".to_owned(),
            suppress_available: true,
            rerun_available: true,
            preserves_producer: true,
        },
        suppression: ValidationSuppression {
            state: ValidationSuppressionState::Suppressed,
            attributed_to_ref: Some("actor:maintainer:docs-owner".to_owned()),
            history_ref: Some("history:docs-validation-report:help_network_skipped#suppressed".to_owned()),
            previewable: true,
            reopenable: true,
            note: "suppressed by the docs owner: the live-endpoint example is intentionally not run in CI; recorded in durable history and reopenable".to_owned(),
        },
        detail: "the help live-endpoint command is intentionally skipped offline; the suppression is attributed and reopenable".to_owned(),
        cited: true,
        citation_ref: Some("cite:skipped:help-live-endpoint".to_owned()),
    }
}

fn guide_unsupported_row() -> ValidationReportRow {
    ValidationReportRow {
        row_id: "row:guide:windows_unsupported".to_owned(),
        subject: ValidationSubject {
            subject_kind: ValidationSubjectKind::ShellExample,
            doc_ref: "docs/guides/retry_with_backoff/operations.md".to_owned(),
            snippet_anchor: Some("windows-service".to_owned()),
            display_path: "Guide → Operations → Windows service".to_owned(),
            label: "the Windows service registration command".to_owned(),
        },
        mode: ValidationMode::Unsupported,
        outcome: ValidationOutcome::NotRun,
        last_checked_at: "2026-06-11T22:24:00Z".to_owned(),
        scope: ValidationScope {
            environment_label: "mirrored runner pool (linux only)".to_owned(),
            toolchain_ref: "toolchain:mirrored-runner-pool:linux".to_owned(),
            target_version_ref: "pack:ops/runbooks@mirrored-rev".to_owned(),
            version_match: ValidationVersionMatch::UnknownTargetBuild,
        },
        produced_by: ValidationProducer {
            validator: ValidatorKind::RemoteExampleRunner,
            execution_context_ref: "exec-context:mirrored-runner-pool:linux".to_owned(),
            detail: "the command targets Windows; the mirrored runner pool has no Windows host, so validation is unsupported here".to_owned(),
        },
        chips: ValidationChipSet {
            freshness: ValidationFreshness::WarmCached,
            version_match: ValidationVersionMatch::UnknownTargetBuild,
            locality: ValidationLocality::MirroredPack,
        },
        provenance: ValidationEvidenceProvenance::Mirrored,
        provenance_disclosure_note: "served from the mirror: validation is unsupported on the mirrored linux-only runner pool, so the Windows command is held to not-run rather than claimed as passing".to_owned(),
        source_trace_ref: "open-failing-source:pack:ops/runbooks/windows_service.md".to_owned(),
        actions: ValidationActionSet {
            open_snippet_ref: "open-snippet:docs/guides/retry_with_backoff/operations.md#windows-service".to_owned(),
            open_failing_source_ref: "open-failing-source:pack:ops/runbooks/windows_service.md".to_owned(),
            compare_current_source_ref: "compare:guide-windows-service:current-source".to_owned(),
            suppress_available: true,
            rerun_available: true,
            preserves_producer: true,
        },
        suppression: active_suppression("active; the unsupported finding is open and discloses the missing Windows runner"),
        detail: "the Windows service command cannot be validated on the mirrored linux-only runner pool; the gap is disclosed, not hidden".to_owned(),
        cited: true,
        citation_ref: Some("cite:unsupported:guide-windows-service".to_owned()),
    }
}

fn export_row(row: &ValidationReportRow) -> ValidationReportExportRow {
    ValidationReportExportRow {
        row_id_ref: row.row_id.clone(),
        subject_kind: row.subject.subject_kind,
        mode: row.mode,
        outcome: row.outcome,
        last_checked_at: row.last_checked_at.clone(),
        environment_label: row.scope.environment_label.clone(),
        version_match: row.scope.version_match,
        freshness: row.chips.freshness,
        provenance: row.provenance,
        produced_by: row.produced_by.validator,
        suppression_state: row.suppression.state,
        action_parity_complete: row.actions.parity_complete(),
        cited: row.cited,
    }
}

fn seeded_export() -> DocsValidationReportExport {
    let rows = seeded_rows().iter().map(export_row).collect();
    DocsValidationReportExport {
        scope: ValidationExportScope::AllRows,
        preserves_mode: true,
        preserves_outcome: true,
        preserves_last_checked: true,
        preserves_scope: true,
        preserves_freshness: true,
        preserves_provenance: true,
        preserves_producer: true,
        preserves_action_parity: true,
        preserves_suppression: true,
        rows,
    }
}

fn required_projections(packet_id: &str) -> Vec<ValidationConsumerProjection> {
    [
        ValidationConsumerSurface::DocsValidationReport,
        ValidationConsumerSurface::DocsAuthoringSurface,
        ValidationConsumerSurface::DocsReviewPanel,
        ValidationConsumerSurface::DocsBrowserShell,
        ValidationConsumerSurface::ReleaseCenter,
        ValidationConsumerSurface::AiContextInspector,
        ValidationConsumerSurface::CliHeadless,
        ValidationConsumerSurface::SupportExport,
        ValidationConsumerSurface::Diagnostics,
        ValidationConsumerSurface::HelpAbout,
    ]
    .into_iter()
    .map(|surface| ValidationConsumerProjection {
        surface,
        packet_id_ref: packet_id.to_owned(),
        preserves_modes: true,
        preserves_outcomes: true,
        preserves_last_checked: true,
        preserves_scope: true,
        preserves_chips: true,
        preserves_provenance: true,
        preserves_producer: true,
        preserves_action_parity: true,
        preserves_suppression: true,
    })
    .collect()
}
