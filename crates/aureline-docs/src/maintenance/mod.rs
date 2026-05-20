//! Governed docs preview and maintenance records.
//!
//! The records in this module turn Markdown preview headers, evidence-backed
//! suggestion cards, stale-example/broken-link finding rows, and
//! README/changelog/onboarding maintenance rows into governed objects. They
//! carry source/version, validation, and publish-scope truth so docs work stays
//! review-first and source-first: a rendered preview is never mistaken for
//! canonical source or proof, suggestions stay diff-based and evidence-backed,
//! and local-only docs work is never silently promoted across a publish
//! boundary.
//!
//! The records intentionally do not carry raw document bodies, rendered HTML,
//! raw source files, or raw URLs. They reference that material through stable
//! opaque refs so that maintenance work can be reviewed and exported without
//! screenshots or copy/paste archaeology.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{CitationConfidenceClass, DocsFreshnessClass, VersionMatchState};

/// Schema version shared by docs preview and maintenance records.
pub const DOCS_MAINTENANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`DocsPreviewHeader`].
pub const DOCS_PREVIEW_HEADER_RECORD_KIND: &str = "docs_preview_header_record";

/// Stable record kind for [`DocsSuggestionCard`].
pub const DOCS_SUGGESTION_CARD_RECORD_KIND: &str = "docs_suggestion_card_record";

/// Stable record kind for [`DocsExampleFindingRow`].
pub const DOCS_EXAMPLE_FINDING_ROW_RECORD_KIND: &str = "docs_example_finding_row_record";

/// Stable record kind for [`DocsMaintenanceRow`].
pub const DOCS_MAINTENANCE_ROW_RECORD_KIND: &str = "docs_maintenance_row_record";

/// Stable record kind for [`DocsMaintenanceContract`].
pub const DOCS_MAINTENANCE_CONTRACT_RECORD_KIND: &str = "docs_maintenance_contract_record";

/// Stable record kind for [`DocsMaintenanceSurfaceProjection`].
pub const DOCS_MAINTENANCE_SURFACE_PROJECTION_RECORD_KIND: &str =
    "docs_maintenance_surface_projection_record";

/// Stable record kind for [`DocsMaintenanceReviewPacket`].
pub const DOCS_MAINTENANCE_REVIEW_PACKET_RECORD_KIND: &str =
    "docs_maintenance_review_packet_record";

/// Stable id for the seeded docs preview-and-maintenance contract.
pub const DOCS_PREVIEW_AND_MAINTENANCE_CONTRACT_ID: &str =
    "docs-maintenance:preview-and-maintenance:beta:v1";

/// Stable version ref for the seeded contract.
pub const DOCS_PREVIEW_AND_MAINTENANCE_VERSION_REF: &str =
    "docs-maintenance-rev:preview-and-maintenance:2026.05.20-01";

/// Repository-relative schema ref for suggestion-card records.
pub const DOCS_SUGGESTION_CARD_SCHEMA_REF: &str = "schemas/docs/docs_suggestion_card.schema.json";

/// Repository-relative schema ref for maintenance-row records.
pub const DOCS_MAINTENANCE_ROW_SCHEMA_REF: &str = "schemas/docs/docs_maintenance_row.schema.json";

/// Stable user-facing label for the source-toggle action.
pub const OPEN_SOURCE_ACTION_LABEL: &str = "Open source";

/// Stable user-facing label for the open-in-browser handoff action.
pub const OPEN_IN_BROWSER_ACTION_LABEL: &str = "Open in browser";

/// Stable user-facing label for the open-evidence action.
pub const OPEN_EVIDENCE_ACTION_LABEL: &str = "Open evidence";

/// Stable user-facing label for the review-diff action.
pub const OPEN_REVIEW_DIFF_ACTION_LABEL: &str = "Review diff";

/// Stable user-facing label for the open-failing-source action.
pub const OPEN_FAILING_SOURCE_ACTION_LABEL: &str = "Open failing source";

const GENERATED_AT: &str = "2026-05-20T15:00:00Z";

/// Markdown preview render mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPreviewMode {
    /// Raw Markdown source is shown; nothing is rendered.
    Source,
    /// Source and rendered preview are shown side by side.
    Split,
    /// Rendered preview is shown.
    Rendered,
}

impl DocsPreviewMode {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Source => "source",
            Self::Split => "split",
            Self::Rendered => "rendered",
        }
    }

    /// Returns true when this mode renders Markdown to a non-source view.
    pub const fn renders_preview(self) -> bool {
        matches!(self, Self::Split | Self::Rendered)
    }
}

/// HTML sanitization posture for a rendered Markdown preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPreviewSanitizationState {
    /// HTML was sanitized; scripts, iframes, and event handlers were stripped.
    SanitizedSafe,
    /// Raw embedded HTML was present and blocked from rendering.
    RawHtmlBlocked,
    /// Raw embedded HTML rendered under an explicit disclosure.
    RawHtmlAllowedDisclosed,
    /// Nothing is rendered, so sanitization does not apply.
    NotApplicable,
}

impl DocsPreviewSanitizationState {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SanitizedSafe => "sanitized_safe",
            Self::RawHtmlBlocked => "raw_html_blocked",
            Self::RawHtmlAllowedDisclosed => "raw_html_allowed_disclosed",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Returns true when the state requires a visible disclosure note.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::RawHtmlAllowedDisclosed)
    }
}

/// Documentation artifact family targeted by preview, suggestion, and rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsArtifactKind {
    /// Project README.
    Readme,
    /// Changelog or release-notes log.
    Changelog,
    /// In-product help article.
    HelpArticle,
    /// Onboarding or first-run note.
    OnboardingNote,
    /// Module-level or crate-level documentation.
    ModuleDoc,
    /// Migration notes.
    MigrationNotes,
    /// Release notes for a specific release.
    ReleaseNotes,
    /// Versioned docs pack.
    DocsPack,
    /// Support runbook.
    SupportRunbook,
    /// Benchmark-claim copy.
    BenchmarkCopy,
    /// Screenshot-bearing documentation.
    Screenshot,
    /// Symbol-linked reference page.
    SymbolReference,
    /// Generic Markdown document.
    MarkdownDoc,
}

impl DocsArtifactKind {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Readme => "readme",
            Self::Changelog => "changelog",
            Self::HelpArticle => "help_article",
            Self::OnboardingNote => "onboarding_note",
            Self::ModuleDoc => "module_doc",
            Self::MigrationNotes => "migration_notes",
            Self::ReleaseNotes => "release_notes",
            Self::DocsPack => "docs_pack",
            Self::SupportRunbook => "support_runbook",
            Self::BenchmarkCopy => "benchmark_copy",
            Self::Screenshot => "screenshot",
            Self::SymbolReference => "symbol_reference",
            Self::MarkdownDoc => "markdown_doc",
        }
    }

    /// Returns true when the artifact is release-facing public proof.
    pub const fn is_release_facing(self) -> bool {
        matches!(
            self,
            Self::Changelog | Self::ReleaseNotes | Self::BenchmarkCopy
        )
    }
}

/// Audience the maintained artifact is written for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsAudienceScope {
    /// Application or extension developer.
    Developer,
    /// Project or docs maintainer.
    Maintainer,
    /// End user of the product.
    EndUser,
    /// Release manager.
    ReleaseManager,
    /// Support operator.
    SupportOperator,
    /// Enterprise administrator.
    EnterpriseAdmin,
    /// Public reader of published material.
    PublicReader,
}

impl DocsAudienceScope {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Developer => "developer",
            Self::Maintainer => "maintainer",
            Self::EndUser => "end_user",
            Self::ReleaseManager => "release_manager",
            Self::SupportOperator => "support_operator",
            Self::EnterpriseAdmin => "enterprise_admin",
            Self::PublicReader => "public_reader",
        }
    }
}

/// Trigger source that produced a docs suggestion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSuggestionTrigger {
    /// A code diff changed referenced behavior, commands, or symbols.
    CodeDiff,
    /// A stale-example scan found drift.
    StaleExample,
    /// Release notes drifted from claim or compatibility truth.
    ReleaseNoteDrift,
    /// A documented snippet failed validation.
    FailingSnippet,
    /// A contract, API, or schema changed.
    ContractChange,
    /// A human reviewer recorded a maintenance note.
    HumanNote,
}

impl DocsSuggestionTrigger {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeDiff => "code_diff",
            Self::StaleExample => "stale_example",
            Self::ReleaseNoteDrift => "release_note_drift",
            Self::FailingSnippet => "failing_snippet",
            Self::ContractChange => "contract_change",
            Self::HumanNote => "human_note",
        }
    }
}

/// Apply posture for a docs suggestion card.
///
/// There is no auto-apply posture: suggestions are never silently applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSuggestionApplyPosture {
    /// Draft prose only; no change may be applied from this card.
    DraftOnly,
    /// A review diff is available; apply requires explicit human review.
    ReviewDiffOnly,
    /// Apply is allowed after explicit human review of the diff.
    ApplyAfterReview,
    /// Apply is blocked until evidence is supplied.
    BlockedPendingEvidence,
}

impl DocsSuggestionApplyPosture {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DraftOnly => "draft_only",
            Self::ReviewDiffOnly => "review_diff_only",
            Self::ApplyAfterReview => "apply_after_review",
            Self::BlockedPendingEvidence => "blocked_pending_evidence",
        }
    }

    /// Returns true when the posture requires a review diff ref.
    pub const fn requires_review_diff(self) -> bool {
        matches!(self, Self::ReviewDiffOnly | Self::ApplyAfterReview)
    }
}

/// Finding class for a stale-example or broken-link row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsFindingClass {
    /// A link target no longer resolves.
    BrokenLink,
    /// A documented example drifted from current behavior.
    StaleExample,
    /// A referenced command was renamed or removed.
    RenamedCommand,
    /// A referenced setting was renamed or removed.
    RenamedSetting,
    /// A referenced symbol was renamed or removed.
    RenamedSymbol,
    /// A screenshot no longer matches the current product.
    StaleScreenshot,
    /// Documented API behavior no longer matches the contract.
    ApiMismatch,
    /// Documented command output drifted from observed output.
    CommandOutputDrift,
    /// A documented import path drifted.
    ImportPathDrift,
    /// A documented version no longer matches the target build.
    VersionMismatch,
    /// A benchmark claim could not be verified against evidence.
    UnverifiableBenchmarkCopy,
    /// A migration note is missing for a breaking change.
    MissingMigrationNote,
}

impl DocsFindingClass {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrokenLink => "broken_link",
            Self::StaleExample => "stale_example",
            Self::RenamedCommand => "renamed_command",
            Self::RenamedSetting => "renamed_setting",
            Self::RenamedSymbol => "renamed_symbol",
            Self::StaleScreenshot => "stale_screenshot",
            Self::ApiMismatch => "api_mismatch",
            Self::CommandOutputDrift => "command_output_drift",
            Self::ImportPathDrift => "import_path_drift",
            Self::VersionMismatch => "version_mismatch",
            Self::UnverifiableBenchmarkCopy => "unverifiable_benchmark_copy",
            Self::MissingMigrationNote => "missing_migration_note",
        }
    }
}

/// Detection posture distinguishing proven failure from suspected drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsFindingDetectionState {
    /// A resolver or validator observed a concrete failure.
    ProvenBroken,
    /// Drift signals exist but failure was not reproduced.
    SuspectedStale,
    /// Required validation is missing or expired even if content is unchanged.
    UnchangedUnverified,
}

impl DocsFindingDetectionState {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvenBroken => "proven_broken",
            Self::SuspectedStale => "suspected_stale",
            Self::UnchangedUnverified => "unchanged_unverified",
        }
    }
}

/// Validation mode applied to a documented example.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsExampleValidationMode {
    /// Example was rendered but not otherwise checked.
    Rendered,
    /// Example was syntax-checked.
    SyntaxChecked,
    /// Example was executed in the local environment.
    ExecutedLocally,
    /// Example was executed in a remote environment.
    ExecutedRemotely,
    /// Validation is unsupported in the current environment.
    Unsupported,
    /// Validation was deliberately skipped.
    Skipped,
    /// A prior validation result is stale and must be rerun.
    Stale,
    /// The example was not validated.
    NotValidated,
}

impl DocsExampleValidationMode {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rendered => "rendered",
            Self::SyntaxChecked => "syntax_checked",
            Self::ExecutedLocally => "executed_locally",
            Self::ExecutedRemotely => "executed_remotely",
            Self::Unsupported => "unsupported",
            Self::Skipped => "skipped",
            Self::Stale => "stale",
            Self::NotValidated => "not_validated",
        }
    }

    /// Returns true when this mode reflects a concrete validation run.
    pub const fn is_concrete_check(self) -> bool {
        matches!(
            self,
            Self::Rendered | Self::SyntaxChecked | Self::ExecutedLocally | Self::ExecutedRemotely
        )
    }
}

/// Suppression posture for a finding row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsFindingSuppressionState {
    /// Finding is active and visible.
    Active,
    /// Finding is suppressed until an owner reviews it.
    SuppressedUntilReviewed,
    /// Finding was acknowledged but kept visible.
    Acknowledged,
    /// Finding was resolved.
    Resolved,
}

impl DocsFindingSuppressionState {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::SuppressedUntilReviewed => "suppressed_until_reviewed",
            Self::Acknowledged => "acknowledged",
            Self::Resolved => "resolved",
        }
    }

    /// Returns true when the state requires a suppression ref.
    pub const fn requires_suppression_ref(self) -> bool {
        matches!(self, Self::SuppressedUntilReviewed)
    }
}

/// Local-only versus publish-boundary posture for docs work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPublishBoundaryState {
    /// Work stays local to the workspace and never crosses a publish boundary.
    LocalOnly,
    /// Work is staged for a scoped review handoff that stays inside review.
    ReviewHandoffScoped,
    /// Work is staged for a scoped, explicit publish handoff.
    PublishHandoffScoped,
    /// An external publish was attempted without scope and is blocked.
    BlockedUnscoped,
}

impl DocsPublishBoundaryState {
    /// Returns the stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ReviewHandoffScoped => "review_handoff_scoped",
            Self::PublishHandoffScoped => "publish_handoff_scoped",
            Self::BlockedUnscoped => "blocked_unscoped",
        }
    }

    /// Returns true when the state crosses a review or publish boundary.
    pub const fn crosses_boundary(self) -> bool {
        matches!(self, Self::ReviewHandoffScoped | Self::PublishHandoffScoped)
    }

    /// Returns true when the state must carry an explicit branch/release/channel scope.
    pub const fn requires_scope(self) -> bool {
        self.crosses_boundary()
    }
}

/// Branch, release, and channel scope bounding a publish boundary.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DocsPublishScope {
    /// Branch the docs work targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub branch_scope: Option<String>,
    /// Release the docs work targets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub release_scope: Option<String>,
    /// Channel the docs work targets (for example `beta`, `stable`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_scope: Option<String>,
}

impl DocsPublishScope {
    /// Returns true when at least one scope dimension is named.
    pub fn is_scoped(&self) -> bool {
        [&self.branch_scope, &self.release_scope, &self.channel_scope]
            .into_iter()
            .any(|value| {
                value
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty())
            })
    }
}

/// Keyboard-reachable action attached to a docs maintenance object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenanceAction {
    /// Stable action ref used by keyboard, command palette, and exports.
    pub action_ref: String,
    /// User-facing action label.
    pub action_label: String,
    /// Whether the action is reachable without a pointer.
    pub keyboard_reachable: bool,
}

impl DocsMaintenanceAction {
    /// Builds a keyboard-reachable action.
    pub fn new(action_ref: impl Into<String>, action_label: impl Into<String>) -> Self {
        Self {
            action_ref: action_ref.into(),
            action_label: action_label.into(),
            keyboard_reachable: true,
        }
    }

    fn is_well_formed(&self) -> bool {
        !self.action_ref.trim().is_empty()
            && !self.action_label.trim().is_empty()
            && self.keyboard_reachable
    }
}

/// Compact docs source/version badge rendered by a preview header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSourceVersionBadge {
    /// Source class token such as `project_docs`.
    pub source_class_token: String,
    /// Stable source pack or document ref.
    pub source_pack_ref: String,
    /// Source revision ref.
    pub source_revision_ref: String,
    /// Version or revision rendered.
    pub version_or_revision_ref: String,
    /// Source build date or deterministic build stamp.
    pub source_build_at: String,
    /// Running build identity the source was checked against.
    pub running_build_identity_ref: String,
    /// Freshness class for the source.
    pub freshness_class: DocsFreshnessClass,
    /// Version-match state against the active target.
    pub version_match_state: VersionMatchState,
}

impl DocsSourceVersionBadge {
    fn is_well_formed(&self) -> bool {
        ![
            &self.source_class_token,
            &self.source_pack_ref,
            &self.source_revision_ref,
            &self.version_or_revision_ref,
            &self.source_build_at,
            &self.running_build_identity_ref,
        ]
        .iter()
        .any(|value| value.trim().is_empty())
    }
}

/// Markdown preview header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPreviewHeader {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable header id.
    pub header_id: String,
    /// Artifact family being previewed.
    pub artifact_kind: DocsArtifactKind,
    /// Stable artifact ref (path or artifact id, never a raw body).
    pub artifact_ref: String,
    /// Current preview mode.
    pub preview_mode: DocsPreviewMode,
    /// Whether CommonMark is the parsing baseline (must be true).
    pub commonmark_baseline: bool,
    /// Enabled extension tokens beyond the CommonMark baseline.
    pub enabled_extensions: Vec<String>,
    /// HTML sanitization posture for rendered content.
    pub sanitization_state: DocsPreviewSanitizationState,
    /// Disclosure note for the sanitization posture when required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sanitization_note: Option<String>,
    /// Docs source/version badge.
    pub source_version_badge: DocsSourceVersionBadge,
    /// Local-only versus publish-boundary posture.
    pub publish_boundary_state: DocsPublishBoundaryState,
    /// Open-source (switch to source / open file) action.
    pub open_source_action: DocsMaintenanceAction,
    /// Open-in-browser handoff action when external opening is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_browser_action: Option<DocsMaintenanceAction>,
    /// Whether the source/split/rendered toggle is keyboard reachable.
    pub mode_toggle_keyboard_reachable: bool,
    /// Disclosure that the rendered preview is not canonical source or proof.
    pub rendered_is_not_canonical_note: String,
    /// Surface refs that render this header.
    pub surface_refs: Vec<String>,
}

impl DocsPreviewHeader {
    fn validate(&self, findings: &mut Vec<DocsMaintenanceFinding>) {
        if self.record_kind != DOCS_PREVIEW_HEADER_RECORD_KIND {
            findings.push(DocsMaintenanceFinding::new(
                &self.header_id,
                "preview_header.record_kind",
                "preview header record_kind is unsupported",
            ));
        }
        if self.schema_version != DOCS_MAINTENANCE_SCHEMA_VERSION {
            findings.push(DocsMaintenanceFinding::new(
                &self.header_id,
                "preview_header.schema_version",
                "preview header schema version is unsupported",
            ));
        }
        if self.header_id.trim().is_empty()
            || self.artifact_ref.trim().is_empty()
            || self.surface_refs.is_empty()
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.header_id,
                "preview_header.identity",
                "preview header id, artifact ref, and surface refs must be non-empty",
            ));
        }
        if !self.commonmark_baseline {
            findings.push(DocsMaintenanceFinding::new(
                &self.header_id,
                "preview_header.commonmark_baseline",
                "preview must declare a CommonMark baseline",
            ));
        }
        if !self.source_version_badge.is_well_formed() {
            findings.push(DocsMaintenanceFinding::new(
                &self.header_id,
                "preview_header.source_version_badge",
                "preview header must carry a complete source/version badge",
            ));
        }
        if !self.open_source_action.is_well_formed() {
            findings.push(DocsMaintenanceFinding::new(
                &self.header_id,
                "preview_header.open_source_action",
                "open-source action must be keyboard reachable and well formed",
            ));
        }
        if !self.mode_toggle_keyboard_reachable {
            findings.push(DocsMaintenanceFinding::new(
                &self.header_id,
                "preview_header.mode_toggle",
                "source/split/rendered toggle must be keyboard reachable",
            ));
        }
        if self.preview_mode.renders_preview()
            && self.rendered_is_not_canonical_note.trim().is_empty()
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.header_id,
                "preview_header.canonical_disclosure",
                "rendered or split preview must disclose that rendered output is not canonical source or proof",
            ));
        }
        if self.preview_mode == DocsPreviewMode::Source
            && self.sanitization_state != DocsPreviewSanitizationState::NotApplicable
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.header_id,
                "preview_header.source_sanitization",
                "source mode renders nothing, so sanitization must be not_applicable",
            ));
        }
        if self.sanitization_state.requires_disclosure()
            && self
                .sanitization_note
                .as_deref()
                .map_or(true, |note| note.trim().is_empty())
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.header_id,
                "preview_header.sanitization_note",
                "allowed raw HTML must carry a disclosure note",
            ));
        }
        if let Some(action) = &self.open_browser_action {
            if !action.is_well_formed() {
                findings.push(DocsMaintenanceFinding::new(
                    &self.header_id,
                    "preview_header.open_browser_action",
                    "open-in-browser action must be keyboard reachable and well formed",
                ));
            }
        }
    }
}

/// Evidence-backed docs suggestion card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSuggestionCard {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable card id.
    pub card_id: String,
    /// Ref to the underlying docs suggestion record.
    pub suggestion_ref: String,
    /// Artifact family targeted by the suggestion.
    pub artifact_kind: DocsArtifactKind,
    /// Stable target artifact ref.
    pub target_artifact_ref: String,
    /// Optional target section ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_section_ref: Option<String>,
    /// Trigger source that produced the suggestion.
    pub trigger: DocsSuggestionTrigger,
    /// Stable ref to the triggering diff, example, note, or contract change.
    pub trigger_source_ref: String,
    /// Confidence class for the suggestion.
    pub confidence_class: CitationConfidenceClass,
    /// Freshness class for the cited evidence.
    pub freshness_class: DocsFreshnessClass,
    /// Version-match state against the active target.
    pub version_match_state: VersionMatchState,
    /// Apply posture (never auto-apply).
    pub apply_posture: DocsSuggestionApplyPosture,
    /// Review-diff ref when a diff is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_diff_ref: Option<String>,
    /// Evidence refs backing the suggestion.
    pub evidence_refs: Vec<String>,
    /// Open-evidence action.
    pub open_evidence_action: DocsMaintenanceAction,
    /// Open-review-diff action when a diff is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_review_diff_action: Option<DocsMaintenanceAction>,
    /// Whether generated draft text is explicitly disclosed.
    pub generated_text_disclosed: bool,
    /// Whether silent rewriting is blocked (must be true).
    pub silent_rewrite_blocked: bool,
    /// Local-only versus publish-boundary posture.
    pub publish_boundary_state: DocsPublishBoundaryState,
    /// Surface refs that render this card.
    pub surface_refs: Vec<String>,
}

impl DocsSuggestionCard {
    fn validate(&self, findings: &mut Vec<DocsMaintenanceFinding>) {
        if self.record_kind != DOCS_SUGGESTION_CARD_RECORD_KIND {
            findings.push(DocsMaintenanceFinding::new(
                &self.card_id,
                "suggestion_card.record_kind",
                "suggestion card record_kind is unsupported",
            ));
        }
        if self.schema_version != DOCS_MAINTENANCE_SCHEMA_VERSION {
            findings.push(DocsMaintenanceFinding::new(
                &self.card_id,
                "suggestion_card.schema_version",
                "suggestion card schema version is unsupported",
            ));
        }
        if self.card_id.trim().is_empty()
            || self.suggestion_ref.trim().is_empty()
            || self.target_artifact_ref.trim().is_empty()
            || self.trigger_source_ref.trim().is_empty()
            || self.surface_refs.is_empty()
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.card_id,
                "suggestion_card.identity",
                "suggestion card id, suggestion ref, target ref, trigger ref, and surface refs must be non-empty",
            ));
        }
        if self.evidence_refs.is_empty() {
            findings.push(DocsMaintenanceFinding::new(
                &self.card_id,
                "suggestion_card.evidence",
                "suggestion card must be evidence-backed with at least one evidence ref",
            ));
        }
        if !self.silent_rewrite_blocked {
            findings.push(DocsMaintenanceFinding::new(
                &self.card_id,
                "suggestion_card.silent_rewrite",
                "suggestion card must block silent rewrites",
            ));
        }
        if self.apply_posture.requires_review_diff()
            && self
                .review_diff_ref
                .as_deref()
                .map_or(true, |value| value.trim().is_empty())
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.card_id,
                "suggestion_card.review_diff",
                "review-diff and apply-after-review cards must carry a review diff ref",
            ));
        }
        if !self.open_evidence_action.is_well_formed() {
            findings.push(DocsMaintenanceFinding::new(
                &self.card_id,
                "suggestion_card.open_evidence_action",
                "open-evidence action must be keyboard reachable and well formed",
            ));
        }
        if let Some(action) = &self.open_review_diff_action {
            if !action.is_well_formed() {
                findings.push(DocsMaintenanceFinding::new(
                    &self.card_id,
                    "suggestion_card.open_review_diff_action",
                    "open-review-diff action must be keyboard reachable and well formed",
                ));
            }
        }
        if self.publish_boundary_state == DocsPublishBoundaryState::BlockedUnscoped
            && self.apply_posture != DocsSuggestionApplyPosture::BlockedPendingEvidence
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.card_id,
                "suggestion_card.blocked_posture",
                "unscoped publish boundary must block the apply posture",
            ));
        }
    }
}

/// Stale-example or broken-link finding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsExampleFindingRow {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable finding id.
    pub finding_id: String,
    /// Artifact family the finding belongs to.
    pub artifact_kind: DocsArtifactKind,
    /// Stable target artifact ref.
    pub target_artifact_ref: String,
    /// Optional target section ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_section_ref: Option<String>,
    /// Finding class.
    pub finding_class: DocsFindingClass,
    /// Detection posture distinguishing proven from suspected drift.
    pub detection_state: DocsFindingDetectionState,
    /// Validation mode applied to the example.
    pub validation_mode: DocsExampleValidationMode,
    /// Last-checked time, when a check was run.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_checked_at: Option<String>,
    /// Environment scope ref the finding applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_scope_ref: Option<String>,
    /// Version scope ref the finding applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_scope_ref: Option<String>,
    /// Freshness class for the finding evidence.
    pub freshness_class: DocsFreshnessClass,
    /// Open-failing-source action.
    pub open_failing_source_action: DocsMaintenanceAction,
    /// Suppression posture.
    pub suppression_state: DocsFindingSuppressionState,
    /// Suppression ref when suppressed until reviewed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppression_ref: Option<String>,
    /// Evidence refs backing the finding.
    pub evidence_refs: Vec<String>,
    /// Surface refs that render this finding.
    pub surface_refs: Vec<String>,
}

impl DocsExampleFindingRow {
    fn validate(&self, findings: &mut Vec<DocsMaintenanceFinding>) {
        if self.record_kind != DOCS_EXAMPLE_FINDING_ROW_RECORD_KIND {
            findings.push(DocsMaintenanceFinding::new(
                &self.finding_id,
                "finding_row.record_kind",
                "finding row record_kind is unsupported",
            ));
        }
        if self.schema_version != DOCS_MAINTENANCE_SCHEMA_VERSION {
            findings.push(DocsMaintenanceFinding::new(
                &self.finding_id,
                "finding_row.schema_version",
                "finding row schema version is unsupported",
            ));
        }
        if self.finding_id.trim().is_empty()
            || self.target_artifact_ref.trim().is_empty()
            || self.surface_refs.is_empty()
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.finding_id,
                "finding_row.identity",
                "finding row id, target ref, and surface refs must be non-empty",
            ));
        }
        if self.detection_state == DocsFindingDetectionState::ProvenBroken
            && self.validation_mode == DocsExampleValidationMode::NotValidated
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.finding_id,
                "finding_row.proven_broken_validation",
                "proven-broken findings must cite a validation mode other than not_validated",
            ));
        }
        if self.validation_mode.is_concrete_check()
            && self
                .last_checked_at
                .as_deref()
                .map_or(true, |value| value.trim().is_empty())
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.finding_id,
                "finding_row.last_checked",
                "concrete validation modes must record a last-checked time",
            ));
        }
        if self.suppression_state.requires_suppression_ref()
            && self
                .suppression_ref
                .as_deref()
                .map_or(true, |value| value.trim().is_empty())
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.finding_id,
                "finding_row.suppression_ref",
                "suppress-until-reviewed findings must carry a suppression ref",
            ));
        }
        if self.evidence_refs.is_empty() {
            findings.push(DocsMaintenanceFinding::new(
                &self.finding_id,
                "finding_row.evidence",
                "finding row must carry at least one evidence ref",
            ));
        }
        if !self.open_failing_source_action.is_well_formed() {
            findings.push(DocsMaintenanceFinding::new(
                &self.finding_id,
                "finding_row.open_failing_source_action",
                "open-failing-source action must be keyboard reachable and well formed",
            ));
        }
    }
}

/// README/changelog/onboarding maintenance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenanceRow {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Artifact family the row maintains.
    pub artifact_kind: DocsArtifactKind,
    /// Stable artifact ref.
    pub artifact_ref: String,
    /// Audience the artifact targets.
    pub audience_scope: DocsAudienceScope,
    /// Branch/release/channel scope.
    pub publish_scope: DocsPublishScope,
    /// Local-only versus publish-boundary posture.
    pub publish_boundary_state: DocsPublishBoundaryState,
    /// Pending suggestion-card count.
    pub pending_suggestion_count: usize,
    /// Pending finding-row count.
    pub pending_finding_count: usize,
    /// Validation freshness class.
    pub validation_freshness: DocsFreshnessClass,
    /// Version-match state against the active target.
    pub version_match_state: VersionMatchState,
    /// Last-validated time, when validated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_validated_at: Option<String>,
    /// Publish-boundary notes shown before apply or export.
    pub publish_boundary_notes: Vec<String>,
    /// Suggestion card refs referenced by this row.
    pub suggestion_card_refs: Vec<String>,
    /// Finding row refs referenced by this row.
    pub finding_row_refs: Vec<String>,
    /// Apply or export action when allowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apply_export_action: Option<DocsMaintenanceAction>,
    /// Surface refs that render this row.
    pub surface_refs: Vec<String>,
}

impl DocsMaintenanceRow {
    fn validate(&self, findings: &mut Vec<DocsMaintenanceFinding>) {
        if self.record_kind != DOCS_MAINTENANCE_ROW_RECORD_KIND {
            findings.push(DocsMaintenanceFinding::new(
                &self.row_id,
                "maintenance_row.record_kind",
                "maintenance row record_kind is unsupported",
            ));
        }
        if self.schema_version != DOCS_MAINTENANCE_SCHEMA_VERSION {
            findings.push(DocsMaintenanceFinding::new(
                &self.row_id,
                "maintenance_row.schema_version",
                "maintenance row schema version is unsupported",
            ));
        }
        if self.row_id.trim().is_empty()
            || self.artifact_ref.trim().is_empty()
            || self.surface_refs.is_empty()
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.row_id,
                "maintenance_row.identity",
                "maintenance row id, artifact ref, and surface refs must be non-empty",
            ));
        }
        if self.pending_suggestion_count != self.suggestion_card_refs.len() {
            findings.push(DocsMaintenanceFinding::new(
                &self.row_id,
                "maintenance_row.suggestion_count",
                "pending suggestion count must match suggestion card refs",
            ));
        }
        if self.pending_finding_count != self.finding_row_refs.len() {
            findings.push(DocsMaintenanceFinding::new(
                &self.row_id,
                "maintenance_row.finding_count",
                "pending finding count must match finding row refs",
            ));
        }
        if self.publish_boundary_state.requires_scope() && !self.publish_scope.is_scoped() {
            findings.push(DocsMaintenanceFinding::new(
                &self.row_id,
                "maintenance_row.publish_scope",
                "review or publish handoff rows must carry branch/release/channel scope",
            ));
        }
        if self.publish_boundary_state != DocsPublishBoundaryState::LocalOnly
            && self.publish_boundary_notes.is_empty()
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.row_id,
                "maintenance_row.publish_notes",
                "non-local rows must carry publish-boundary notes before apply or export",
            ));
        }
        if self.publish_boundary_state == DocsPublishBoundaryState::BlockedUnscoped
            && self.apply_export_action.is_some()
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.row_id,
                "maintenance_row.blocked_action",
                "blocked unscoped rows must not expose an apply or export action",
            ));
        }
        if let Some(action) = &self.apply_export_action {
            if !action.is_well_formed() {
                findings.push(DocsMaintenanceFinding::new(
                    &self.row_id,
                    "maintenance_row.apply_export_action",
                    "apply or export action must be keyboard reachable and well formed",
                ));
            }
        }
    }
}

/// Docs handoff banner preserving local-only versus publish-boundary state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsHandoffBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// Local-only versus publish-boundary posture.
    pub publish_boundary_state: DocsPublishBoundaryState,
    /// Branch/release/channel scope.
    pub publish_scope: DocsPublishScope,
    /// Local-only note shown while work stays local.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only_note: Option<String>,
    /// Publish-handoff note shown before a scoped handoff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_handoff_note: Option<String>,
    /// Whether review is possible without screenshots or copy/paste (must be true).
    pub screenshot_free_review: bool,
}

impl DocsHandoffBanner {
    fn validate(&self, findings: &mut Vec<DocsMaintenanceFinding>) {
        if self.banner_id.trim().is_empty() {
            findings.push(DocsMaintenanceFinding::new(
                &self.banner_id,
                "handoff_banner.identity",
                "handoff banner id must be non-empty",
            ));
        }
        if !self.screenshot_free_review {
            findings.push(DocsMaintenanceFinding::new(
                &self.banner_id,
                "handoff_banner.screenshot_free",
                "handoff banner must support screenshot-free review",
            ));
        }
        if self.publish_boundary_state.requires_scope() && !self.publish_scope.is_scoped() {
            findings.push(DocsMaintenanceFinding::new(
                &self.banner_id,
                "handoff_banner.publish_scope",
                "review or publish handoff banners must carry branch/release/channel scope",
            ));
        }
        match self.publish_boundary_state {
            DocsPublishBoundaryState::LocalOnly => {
                if self
                    .local_only_note
                    .as_deref()
                    .map_or(true, |note| note.trim().is_empty())
                {
                    findings.push(DocsMaintenanceFinding::new(
                        &self.banner_id,
                        "handoff_banner.local_only_note",
                        "local-only banners must explain that work stays local",
                    ));
                }
            }
            DocsPublishBoundaryState::ReviewHandoffScoped
            | DocsPublishBoundaryState::PublishHandoffScoped => {
                if self
                    .publish_handoff_note
                    .as_deref()
                    .map_or(true, |note| note.trim().is_empty())
                {
                    findings.push(DocsMaintenanceFinding::new(
                        &self.banner_id,
                        "handoff_banner.publish_handoff_note",
                        "scoped handoff banners must explain the publish boundary",
                    ));
                }
            }
            DocsPublishBoundaryState::BlockedUnscoped => {}
        }
    }
}

/// Coverage summary for docs preview-and-maintenance projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenanceCoverage {
    /// Number of preview headers.
    pub preview_header_count: usize,
    /// Number of suggestion cards.
    pub suggestion_card_count: usize,
    /// Number of finding rows.
    pub finding_row_count: usize,
    /// Number of maintenance rows.
    pub maintenance_row_count: usize,
    /// Count by preview-mode token.
    pub preview_mode_counts: BTreeMap<String, usize>,
    /// Count by suggestion-trigger token.
    pub trigger_counts: BTreeMap<String, usize>,
    /// Count by finding-class token.
    pub finding_class_counts: BTreeMap<String, usize>,
    /// Count by validation-mode token.
    pub validation_mode_counts: BTreeMap<String, usize>,
    /// Count by publish-boundary token across all rows.
    pub publish_boundary_counts: BTreeMap<String, usize>,
}

/// Governed contract holding docs preview-and-maintenance records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenanceContract {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable contract id.
    pub contract_id: String,
    /// Stable contract version ref.
    pub contract_version_ref: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Related schema, docs, and review artifacts.
    pub contract_refs: BTreeMap<String, String>,
    /// Handoff banner shared by the contract surfaces.
    pub handoff_banner: DocsHandoffBanner,
    /// Markdown preview headers.
    pub preview_headers: Vec<DocsPreviewHeader>,
    /// Evidence-backed suggestion cards.
    pub suggestion_cards: Vec<DocsSuggestionCard>,
    /// Stale-example/broken-link finding rows.
    pub finding_rows: Vec<DocsExampleFindingRow>,
    /// README/changelog/onboarding maintenance rows.
    pub maintenance_rows: Vec<DocsMaintenanceRow>,
    /// Material classes omitted from any export.
    pub omitted_material_classes: Vec<String>,
}

impl DocsMaintenanceContract {
    /// Returns the preview header with `header_id`.
    pub fn preview_header(&self, header_id: &str) -> Option<&DocsPreviewHeader> {
        self.preview_headers
            .iter()
            .find(|header| header.header_id == header_id)
    }

    /// Returns the suggestion card with `card_id`.
    pub fn suggestion_card(&self, card_id: &str) -> Option<&DocsSuggestionCard> {
        self.suggestion_cards
            .iter()
            .find(|card| card.card_id == card_id)
    }

    /// Returns the finding row with `finding_id`.
    pub fn finding_row(&self, finding_id: &str) -> Option<&DocsExampleFindingRow> {
        self.finding_rows
            .iter()
            .find(|row| row.finding_id == finding_id)
    }

    /// Projects render-ready rows and a coverage summary.
    pub fn surface_projection(&self) -> DocsMaintenanceSurfaceProjection {
        DocsMaintenanceSurfaceProjection {
            record_kind: DOCS_MAINTENANCE_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            projection_id: "docs-maintenance:preview-and-maintenance:surface-projection:v1"
                .to_owned(),
            generated_at: self.generated_at.clone(),
            contract_id: self.contract_id.clone(),
            contract_version_ref: self.contract_version_ref.clone(),
            handoff_banner: self.handoff_banner.clone(),
            preview_headers: self.preview_headers.clone(),
            suggestion_cards: self.suggestion_cards.clone(),
            finding_rows: self.finding_rows.clone(),
            maintenance_rows: self.maintenance_rows.clone(),
            coverage: self.coverage(),
        }
    }

    /// Projects a metadata-only, screenshot-free review packet.
    pub fn review_packet(
        &self,
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> DocsMaintenanceReviewPacket {
        DocsMaintenanceReviewPacket {
            record_kind: DOCS_MAINTENANCE_REVIEW_PACKET_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            generated_at: generated_at.into(),
            source_contract_id: self.contract_id.clone(),
            contract_version_ref: self.contract_version_ref.clone(),
            handoff_banner: self.handoff_banner.clone(),
            preview_headers: self.preview_headers.clone(),
            suggestion_cards: self.suggestion_cards.clone(),
            finding_rows: self.finding_rows.clone(),
            maintenance_rows: self.maintenance_rows.clone(),
            omitted_material_classes: self.omitted_material_classes.clone(),
            raw_document_bodies_exported: false,
        }
    }

    /// Validates the contract and every nested record.
    pub fn validate(&self) -> Vec<DocsMaintenanceFinding> {
        let mut findings = Vec::new();
        if self.record_kind != DOCS_MAINTENANCE_CONTRACT_RECORD_KIND {
            findings.push(DocsMaintenanceFinding::new(
                &self.contract_id,
                "contract.record_kind",
                "contract record_kind is unsupported",
            ));
        }
        if self.schema_version != DOCS_MAINTENANCE_SCHEMA_VERSION {
            findings.push(DocsMaintenanceFinding::new(
                &self.contract_id,
                "contract.schema_version",
                "contract schema version is unsupported",
            ));
        }
        if self.contract_id.trim().is_empty()
            || self.contract_version_ref.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.contract_id,
                "contract.identity",
                "contract id, version ref, and generated_at must be non-empty",
            ));
        }
        if self.omitted_material_classes.is_empty() {
            findings.push(DocsMaintenanceFinding::new(
                &self.contract_id,
                "contract.omitted_classes",
                "contract must disclose omitted material classes",
            ));
        }
        if self.preview_headers.is_empty()
            || self.suggestion_cards.is_empty()
            || self.finding_rows.is_empty()
            || self.maintenance_rows.is_empty()
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.contract_id,
                "contract.coverage",
                "contract must cover preview headers, suggestion cards, finding rows, and maintenance rows",
            ));
        }
        self.handoff_banner.validate(&mut findings);
        for header in &self.preview_headers {
            header.validate(&mut findings);
        }
        for card in &self.suggestion_cards {
            card.validate(&mut findings);
        }
        for row in &self.finding_rows {
            row.validate(&mut findings);
        }
        for row in &self.maintenance_rows {
            row.validate(&mut findings);
            for card_ref in &row.suggestion_card_refs {
                if self.suggestion_card(card_ref).is_none() {
                    findings.push(DocsMaintenanceFinding::new(
                        &row.row_id,
                        "maintenance_row.unknown_suggestion_ref",
                        "maintenance row references an unknown suggestion card",
                    ));
                }
            }
            for finding_ref in &row.finding_row_refs {
                if self.finding_row(finding_ref).is_none() {
                    findings.push(DocsMaintenanceFinding::new(
                        &row.row_id,
                        "maintenance_row.unknown_finding_ref",
                        "maintenance row references an unknown finding row",
                    ));
                }
            }
        }
        self.assert_required_coverage(&mut findings);
        findings
    }

    fn assert_required_coverage(&self, findings: &mut Vec<DocsMaintenanceFinding>) {
        for mode in [
            DocsPreviewMode::Source,
            DocsPreviewMode::Split,
            DocsPreviewMode::Rendered,
        ] {
            if !self
                .preview_headers
                .iter()
                .any(|header| header.preview_mode == mode)
            {
                findings.push(DocsMaintenanceFinding::new(
                    &self.contract_id,
                    "contract.preview_mode_coverage",
                    format!("contract must cover preview mode {}", mode.as_str()),
                ));
            }
        }
        // Every spec'd validation mode must appear across finding rows so the
        // distinction between rendered, syntax-checked, executed, unsupported,
        // skipped, stale, and not-validated stays exercised.
        for mode in [
            DocsExampleValidationMode::Rendered,
            DocsExampleValidationMode::SyntaxChecked,
            DocsExampleValidationMode::ExecutedLocally,
            DocsExampleValidationMode::ExecutedRemotely,
            DocsExampleValidationMode::Unsupported,
            DocsExampleValidationMode::Skipped,
            DocsExampleValidationMode::Stale,
            DocsExampleValidationMode::NotValidated,
        ] {
            if !self
                .finding_rows
                .iter()
                .any(|row| row.validation_mode == mode)
            {
                findings.push(DocsMaintenanceFinding::new(
                    &self.contract_id,
                    "contract.validation_mode_coverage",
                    format!("contract must exercise validation mode {}", mode.as_str()),
                ));
            }
        }
    }

    fn coverage(&self) -> DocsMaintenanceCoverage {
        let mut preview_mode_counts = BTreeMap::new();
        let mut trigger_counts = BTreeMap::new();
        let mut finding_class_counts = BTreeMap::new();
        let mut validation_mode_counts = BTreeMap::new();
        let mut publish_boundary_counts = BTreeMap::new();

        for header in &self.preview_headers {
            *preview_mode_counts
                .entry(header.preview_mode.as_str().to_owned())
                .or_insert(0) += 1;
            *publish_boundary_counts
                .entry(header.publish_boundary_state.as_str().to_owned())
                .or_insert(0) += 1;
        }
        for card in &self.suggestion_cards {
            *trigger_counts
                .entry(card.trigger.as_str().to_owned())
                .or_insert(0) += 1;
            *publish_boundary_counts
                .entry(card.publish_boundary_state.as_str().to_owned())
                .or_insert(0) += 1;
        }
        for row in &self.finding_rows {
            *finding_class_counts
                .entry(row.finding_class.as_str().to_owned())
                .or_insert(0) += 1;
            *validation_mode_counts
                .entry(row.validation_mode.as_str().to_owned())
                .or_insert(0) += 1;
        }
        for row in &self.maintenance_rows {
            *publish_boundary_counts
                .entry(row.publish_boundary_state.as_str().to_owned())
                .or_insert(0) += 1;
        }

        DocsMaintenanceCoverage {
            preview_header_count: self.preview_headers.len(),
            suggestion_card_count: self.suggestion_cards.len(),
            finding_row_count: self.finding_rows.len(),
            maintenance_row_count: self.maintenance_rows.len(),
            preview_mode_counts,
            trigger_counts,
            finding_class_counts,
            validation_mode_counts,
            publish_boundary_counts,
        }
    }
}

/// Surface projection for docs preview-and-maintenance records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenanceSurfaceProjection {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Deterministic generation timestamp.
    pub generated_at: String,
    /// Source contract id.
    pub contract_id: String,
    /// Source contract version ref.
    pub contract_version_ref: String,
    /// Handoff banner rendered with the surfaces.
    pub handoff_banner: DocsHandoffBanner,
    /// Markdown preview headers.
    pub preview_headers: Vec<DocsPreviewHeader>,
    /// Evidence-backed suggestion cards.
    pub suggestion_cards: Vec<DocsSuggestionCard>,
    /// Stale-example/broken-link finding rows.
    pub finding_rows: Vec<DocsExampleFindingRow>,
    /// README/changelog/onboarding maintenance rows.
    pub maintenance_rows: Vec<DocsMaintenanceRow>,
    /// Coverage summary for review and release packets.
    pub coverage: DocsMaintenanceCoverage,
}

/// Metadata-only, screenshot-free review packet for docs maintenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenanceReviewPacket {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Export generation timestamp.
    pub generated_at: String,
    /// Source contract id.
    pub source_contract_id: String,
    /// Source contract version ref.
    pub contract_version_ref: String,
    /// Handoff banner preserving local-only versus publish-boundary state.
    pub handoff_banner: DocsHandoffBanner,
    /// Markdown preview headers.
    pub preview_headers: Vec<DocsPreviewHeader>,
    /// Evidence-backed suggestion cards.
    pub suggestion_cards: Vec<DocsSuggestionCard>,
    /// Stale-example/broken-link finding rows.
    pub finding_rows: Vec<DocsExampleFindingRow>,
    /// README/changelog/onboarding maintenance rows.
    pub maintenance_rows: Vec<DocsMaintenanceRow>,
    /// Material classes omitted from the export.
    pub omitted_material_classes: Vec<String>,
    /// Whether raw document bodies were exported (must be false).
    pub raw_document_bodies_exported: bool,
}

impl DocsMaintenanceReviewPacket {
    /// Validates packet reconstruction against `contract`.
    pub fn validate_against_contract(
        &self,
        contract: &DocsMaintenanceContract,
    ) -> Result<(), Vec<DocsMaintenanceFinding>> {
        let mut findings = Vec::new();
        if self.record_kind != DOCS_MAINTENANCE_REVIEW_PACKET_RECORD_KIND {
            findings.push(DocsMaintenanceFinding::new(
                &self.packet_id,
                "review_packet.record_kind",
                "review packet record_kind is unsupported",
            ));
        }
        if self.schema_version != DOCS_MAINTENANCE_SCHEMA_VERSION {
            findings.push(DocsMaintenanceFinding::new(
                &self.packet_id,
                "review_packet.schema_version",
                "review packet schema version is unsupported",
            ));
        }
        if self.packet_id.trim().is_empty() || self.generated_at.trim().is_empty() {
            findings.push(DocsMaintenanceFinding::new(
                &self.packet_id,
                "review_packet.identity",
                "review packet id and generated_at must be non-empty",
            ));
        }
        if self.source_contract_id != contract.contract_id
            || self.contract_version_ref != contract.contract_version_ref
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.packet_id,
                "review_packet.contract_ref",
                "review packet contract refs drifted",
            ));
        }
        if self.raw_document_bodies_exported || self.omitted_material_classes.is_empty() {
            findings.push(DocsMaintenanceFinding::new(
                &self.packet_id,
                "review_packet.raw_bodies",
                "review packet must omit raw document bodies and disclose omitted classes",
            ));
        }
        if self.preview_headers != contract.preview_headers
            || self.suggestion_cards != contract.suggestion_cards
            || self.finding_rows != contract.finding_rows
            || self.maintenance_rows != contract.maintenance_rows
            || self.handoff_banner != contract.handoff_banner
        {
            findings.push(DocsMaintenanceFinding::new(
                &self.packet_id,
                "review_packet.row_drift",
                "review packet drifted from contract records",
            ));
        }
        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }

    /// Deterministic JSON serialization for support/export fixtures.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("docs maintenance review packet serializes")
    }
}

/// Validation finding for docs preview-and-maintenance records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenanceFinding {
    /// Row or object that failed validation.
    pub row_ref: String,
    /// Stable validation check id.
    pub check_id: String,
    /// Reviewable validation message.
    pub message: String,
}

impl DocsMaintenanceFinding {
    fn new(
        row_ref: impl Into<String>,
        check_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            row_ref: row_ref.into(),
            check_id: check_id.into(),
            message: message.into(),
        }
    }
}

fn action(action_ref: &str, action_label: &str) -> DocsMaintenanceAction {
    DocsMaintenanceAction::new(action_ref, action_label)
}

fn current_badge(source_class_token: &str, source_pack_ref: &str) -> DocsSourceVersionBadge {
    DocsSourceVersionBadge {
        source_class_token: source_class_token.to_owned(),
        source_pack_ref: source_pack_ref.to_owned(),
        source_revision_ref: format!("docs-source-rev:{source_pack_ref}:2026.05.20-01"),
        version_or_revision_ref: "rev:aureline-docs:2026.05.20-beta".to_owned(),
        source_build_at: "2026-05-20T14:30:00Z".to_owned(),
        running_build_identity_ref: "build:aureline:2026.05.20-beta".to_owned(),
        freshness_class: DocsFreshnessClass::AuthoritativeLive,
        version_match_state: VersionMatchState::ExactBuildMatch,
    }
}

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

/// Returns the seeded docs preview-and-maintenance contract.
pub fn seeded_docs_preview_and_maintenance_contract() -> DocsMaintenanceContract {
    let handoff_banner = DocsHandoffBanner {
        banner_id: "docs-handoff-banner:preview-and-maintenance:beta:v1".to_owned(),
        publish_boundary_state: DocsPublishBoundaryState::ReviewHandoffScoped,
        publish_scope: DocsPublishScope {
            branch_scope: Some("branch:docs/maintenance-beta".to_owned()),
            release_scope: Some("release:beta-2".to_owned()),
            channel_scope: Some("beta".to_owned()),
        },
        local_only_note: None,
        publish_handoff_note: Some(
            "Docs maintenance review packet — beta scope; not published to stable docs.".to_owned(),
        ),
        screenshot_free_review: true,
    };

    let preview_headers = vec![
        DocsPreviewHeader {
            record_kind: DOCS_PREVIEW_HEADER_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            header_id: "docs-preview-header:readme:source".to_owned(),
            artifact_kind: DocsArtifactKind::Readme,
            artifact_ref: "docs-artifact:readme:root".to_owned(),
            preview_mode: DocsPreviewMode::Source,
            commonmark_baseline: true,
            enabled_extensions: Vec::new(),
            sanitization_state: DocsPreviewSanitizationState::NotApplicable,
            sanitization_note: None,
            source_version_badge: current_badge("project_docs", "docs-pack:project-readme"),
            publish_boundary_state: DocsPublishBoundaryState::LocalOnly,
            open_source_action: action(
                "action:docs-preview.open-source:readme",
                OPEN_SOURCE_ACTION_LABEL,
            ),
            open_browser_action: None,
            mode_toggle_keyboard_reachable: true,
            rendered_is_not_canonical_note: String::new(),
            surface_refs: refs(&["surface:docs_preview:readme"]),
        },
        DocsPreviewHeader {
            record_kind: DOCS_PREVIEW_HEADER_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            header_id: "docs-preview-header:changelog:split".to_owned(),
            artifact_kind: DocsArtifactKind::Changelog,
            artifact_ref: "docs-artifact:changelog:beta".to_owned(),
            preview_mode: DocsPreviewMode::Split,
            commonmark_baseline: true,
            enabled_extensions: refs(&["tables", "autolink"]),
            sanitization_state: DocsPreviewSanitizationState::SanitizedSafe,
            sanitization_note: None,
            source_version_badge: current_badge("project_docs", "docs-pack:changelog"),
            publish_boundary_state: DocsPublishBoundaryState::ReviewHandoffScoped,
            open_source_action: action(
                "action:docs-preview.open-source:changelog",
                OPEN_SOURCE_ACTION_LABEL,
            ),
            open_browser_action: Some(action(
                "action:docs-preview.open-browser:changelog",
                OPEN_IN_BROWSER_ACTION_LABEL,
            )),
            mode_toggle_keyboard_reachable: true,
            rendered_is_not_canonical_note:
                "Rendered preview — the source Markdown is canonical; this view is not proof."
                    .to_owned(),
            surface_refs: refs(&["surface:docs_preview:changelog"]),
        },
        DocsPreviewHeader {
            record_kind: DOCS_PREVIEW_HEADER_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            header_id: "docs-preview-header:help-article:rendered".to_owned(),
            artifact_kind: DocsArtifactKind::HelpArticle,
            artifact_ref: "docs-artifact:help:docs-maintenance".to_owned(),
            preview_mode: DocsPreviewMode::Rendered,
            commonmark_baseline: true,
            enabled_extensions: refs(&["tables", "footnotes", "strikethrough"]),
            sanitization_state: DocsPreviewSanitizationState::RawHtmlAllowedDisclosed,
            sanitization_note: Some(
                "Embedded HTML rendered; scripts, iframes, and event handlers were stripped."
                    .to_owned(),
            ),
            source_version_badge: current_badge(
                "curated_knowledge_pack",
                "docs-pack:help-maintenance",
            ),
            publish_boundary_state: DocsPublishBoundaryState::PublishHandoffScoped,
            open_source_action: action(
                "action:docs-preview.open-source:help-article",
                OPEN_SOURCE_ACTION_LABEL,
            ),
            open_browser_action: Some(action(
                "action:docs-preview.open-browser:help-article",
                OPEN_IN_BROWSER_ACTION_LABEL,
            )),
            mode_toggle_keyboard_reachable: true,
            rendered_is_not_canonical_note:
                "Rendered preview — not canonical source and not release proof.".to_owned(),
            surface_refs: refs(&["surface:docs_preview:help_article"]),
        },
    ];

    let suggestion_cards = vec![
        DocsSuggestionCard {
            record_kind: DOCS_SUGGESTION_CARD_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            card_id: "docs-suggestion-card:readme:code-diff".to_owned(),
            suggestion_ref: "docs-suggestion:readme.update.command-rename".to_owned(),
            artifact_kind: DocsArtifactKind::Readme,
            target_artifact_ref: "docs-artifact:readme:root".to_owned(),
            target_section_ref: Some("section:readme#commands".to_owned()),
            trigger: DocsSuggestionTrigger::CodeDiff,
            trigger_source_ref: "diff:commands.rename-open-folder".to_owned(),
            confidence_class: CitationConfidenceClass::EvidenceBacked,
            freshness_class: DocsFreshnessClass::AuthoritativeLive,
            version_match_state: VersionMatchState::ExactBuildMatch,
            apply_posture: DocsSuggestionApplyPosture::ApplyAfterReview,
            review_diff_ref: Some("diff:docs-suggestion:readme.update.command-rename".to_owned()),
            evidence_refs: refs(&[
                "evidence:command-descriptor:open-folder",
                "evidence:diff:commands.rename-open-folder",
            ]),
            open_evidence_action: action(
                "action:docs-suggestion.open-evidence:readme.code-diff",
                OPEN_EVIDENCE_ACTION_LABEL,
            ),
            open_review_diff_action: Some(action(
                "action:docs-suggestion.review-diff:readme.code-diff",
                OPEN_REVIEW_DIFF_ACTION_LABEL,
            )),
            generated_text_disclosed: true,
            silent_rewrite_blocked: true,
            publish_boundary_state: DocsPublishBoundaryState::ReviewHandoffScoped,
            surface_refs: refs(&["surface:docs_maintenance:suggestion_cards"]),
        },
        DocsSuggestionCard {
            record_kind: DOCS_SUGGESTION_CARD_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            card_id: "docs-suggestion-card:help-article:stale-example".to_owned(),
            suggestion_ref: "docs-suggestion:help.update.stale-snippet".to_owned(),
            artifact_kind: DocsArtifactKind::HelpArticle,
            target_artifact_ref: "docs-artifact:help:docs-maintenance".to_owned(),
            target_section_ref: Some("section:help#examples".to_owned()),
            trigger: DocsSuggestionTrigger::StaleExample,
            trigger_source_ref: "stale-example-scan:help.snippet-01".to_owned(),
            confidence_class: CitationConfidenceClass::Inferred,
            freshness_class: DocsFreshnessClass::WarmCached,
            version_match_state: VersionMatchState::CompatibleMinorDrift,
            apply_posture: DocsSuggestionApplyPosture::ReviewDiffOnly,
            review_diff_ref: Some("diff:docs-suggestion:help.update.stale-snippet".to_owned()),
            evidence_refs: refs(&["evidence:stale-example-scan:help.snippet-01"]),
            open_evidence_action: action(
                "action:docs-suggestion.open-evidence:help.stale-example",
                OPEN_EVIDENCE_ACTION_LABEL,
            ),
            open_review_diff_action: Some(action(
                "action:docs-suggestion.review-diff:help.stale-example",
                OPEN_REVIEW_DIFF_ACTION_LABEL,
            )),
            generated_text_disclosed: true,
            silent_rewrite_blocked: true,
            publish_boundary_state: DocsPublishBoundaryState::LocalOnly,
            surface_refs: refs(&["surface:docs_maintenance:suggestion_cards"]),
        },
        DocsSuggestionCard {
            record_kind: DOCS_SUGGESTION_CARD_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            card_id: "docs-suggestion-card:release-notes:drift".to_owned(),
            suggestion_ref: "docs-suggestion:release-notes.align.support-window".to_owned(),
            artifact_kind: DocsArtifactKind::ReleaseNotes,
            target_artifact_ref: "docs-artifact:release-notes:beta-2".to_owned(),
            target_section_ref: Some("section:release-notes#support".to_owned()),
            trigger: DocsSuggestionTrigger::ReleaseNoteDrift,
            trigger_source_ref: "claim-row:beta-support-window".to_owned(),
            confidence_class: CitationConfidenceClass::EvidenceBacked,
            freshness_class: DocsFreshnessClass::WarmCached,
            version_match_state: VersionMatchState::CompatibleMinorDrift,
            apply_posture: DocsSuggestionApplyPosture::ApplyAfterReview,
            review_diff_ref: Some(
                "diff:docs-suggestion:release-notes.align.support-window".to_owned(),
            ),
            evidence_refs: refs(&[
                "evidence:claim-row:beta-support-window",
                "evidence:compatibility-row:tsjs-launch",
            ]),
            open_evidence_action: action(
                "action:docs-suggestion.open-evidence:release-notes.drift",
                OPEN_EVIDENCE_ACTION_LABEL,
            ),
            open_review_diff_action: Some(action(
                "action:docs-suggestion.review-diff:release-notes.drift",
                OPEN_REVIEW_DIFF_ACTION_LABEL,
            )),
            generated_text_disclosed: true,
            silent_rewrite_blocked: true,
            publish_boundary_state: DocsPublishBoundaryState::PublishHandoffScoped,
            surface_refs: refs(&["surface:docs_maintenance:suggestion_cards"]),
        },
        DocsSuggestionCard {
            record_kind: DOCS_SUGGESTION_CARD_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            card_id: "docs-suggestion-card:module-doc:failing-snippet".to_owned(),
            suggestion_ref: "docs-suggestion:module-doc.fix.failing-example".to_owned(),
            artifact_kind: DocsArtifactKind::ModuleDoc,
            target_artifact_ref: "docs-artifact:module-doc:aureline-docs".to_owned(),
            target_section_ref: Some("section:module-doc#example-03".to_owned()),
            trigger: DocsSuggestionTrigger::FailingSnippet,
            trigger_source_ref: "snippet-validation:module-doc.example-03".to_owned(),
            confidence_class: CitationConfidenceClass::LowConfidence,
            freshness_class: DocsFreshnessClass::Stale,
            version_match_state: VersionMatchState::IncompatibleDriftDetected,
            apply_posture: DocsSuggestionApplyPosture::BlockedPendingEvidence,
            review_diff_ref: None,
            evidence_refs: refs(&["evidence:snippet-validation:module-doc.example-03"]),
            open_evidence_action: action(
                "action:docs-suggestion.open-evidence:module-doc.failing-snippet",
                OPEN_EVIDENCE_ACTION_LABEL,
            ),
            open_review_diff_action: None,
            generated_text_disclosed: false,
            silent_rewrite_blocked: true,
            publish_boundary_state: DocsPublishBoundaryState::LocalOnly,
            surface_refs: refs(&["surface:docs_maintenance:suggestion_cards"]),
        },
        DocsSuggestionCard {
            record_kind: DOCS_SUGGESTION_CARD_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            card_id: "docs-suggestion-card:migration-notes:contract-change".to_owned(),
            suggestion_ref: "docs-suggestion:migration-notes.add.contract-change".to_owned(),
            artifact_kind: DocsArtifactKind::MigrationNotes,
            target_artifact_ref: "docs-artifact:migration-notes:beta-2".to_owned(),
            target_section_ref: Some("section:migration-notes#docs-node".to_owned()),
            trigger: DocsSuggestionTrigger::ContractChange,
            trigger_source_ref: "contract-change:api.docs-node".to_owned(),
            confidence_class: CitationConfidenceClass::EvidenceBacked,
            freshness_class: DocsFreshnessClass::AuthoritativeLive,
            version_match_state: VersionMatchState::ExactBuildMatch,
            apply_posture: DocsSuggestionApplyPosture::ApplyAfterReview,
            review_diff_ref: Some(
                "diff:docs-suggestion:migration-notes.add.contract-change".to_owned(),
            ),
            evidence_refs: refs(&["evidence:contract-change:api.docs-node"]),
            open_evidence_action: action(
                "action:docs-suggestion.open-evidence:migration-notes.contract-change",
                OPEN_EVIDENCE_ACTION_LABEL,
            ),
            open_review_diff_action: Some(action(
                "action:docs-suggestion.review-diff:migration-notes.contract-change",
                OPEN_REVIEW_DIFF_ACTION_LABEL,
            )),
            generated_text_disclosed: true,
            silent_rewrite_blocked: true,
            publish_boundary_state: DocsPublishBoundaryState::ReviewHandoffScoped,
            surface_refs: refs(&["surface:docs_maintenance:suggestion_cards"]),
        },
        DocsSuggestionCard {
            record_kind: DOCS_SUGGESTION_CARD_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            card_id: "docs-suggestion-card:onboarding-note:human-note".to_owned(),
            suggestion_ref: "docs-suggestion:onboarding-note.review.human-note".to_owned(),
            artifact_kind: DocsArtifactKind::OnboardingNote,
            target_artifact_ref: "docs-artifact:onboarding-note:first-run".to_owned(),
            target_section_ref: None,
            trigger: DocsSuggestionTrigger::HumanNote,
            trigger_source_ref: "human-note:onboarding.reviewer-01".to_owned(),
            confidence_class: CitationConfidenceClass::UnknownUnverified,
            freshness_class: DocsFreshnessClass::Unverified,
            version_match_state: VersionMatchState::UnknownTargetBuild,
            apply_posture: DocsSuggestionApplyPosture::DraftOnly,
            review_diff_ref: None,
            evidence_refs: refs(&["evidence:human-note:onboarding.reviewer-01"]),
            open_evidence_action: action(
                "action:docs-suggestion.open-evidence:onboarding-note.human-note",
                OPEN_EVIDENCE_ACTION_LABEL,
            ),
            open_review_diff_action: None,
            generated_text_disclosed: false,
            silent_rewrite_blocked: true,
            publish_boundary_state: DocsPublishBoundaryState::LocalOnly,
            surface_refs: refs(&["surface:docs_maintenance:suggestion_cards"]),
        },
    ];

    let finding_rows = vec![
        finding_row(FindingSeed {
            finding_id: "docs-finding:readme:broken-link",
            artifact_kind: DocsArtifactKind::Readme,
            target_artifact_ref: "docs-artifact:readme:root",
            target_section_ref: Some("section:readme#links"),
            finding_class: DocsFindingClass::BrokenLink,
            detection_state: DocsFindingDetectionState::ProvenBroken,
            validation_mode: DocsExampleValidationMode::Rendered,
            last_checked_at: Some("2026-05-20T14:40:00Z"),
            environment_scope_ref: Some("env:local-link-check"),
            version_scope_ref: Some("rev:aureline-docs:2026.05.20-beta"),
            freshness_class: DocsFreshnessClass::AuthoritativeLive,
            suppression_state: DocsFindingSuppressionState::Active,
            suppression_ref: None,
            evidence_refs: &["evidence:link-check:readme.contributing"],
        }),
        finding_row(FindingSeed {
            finding_id: "docs-finding:help-article:stale-example",
            artifact_kind: DocsArtifactKind::HelpArticle,
            target_artifact_ref: "docs-artifact:help:docs-maintenance",
            target_section_ref: Some("section:help#examples"),
            finding_class: DocsFindingClass::StaleExample,
            detection_state: DocsFindingDetectionState::SuspectedStale,
            validation_mode: DocsExampleValidationMode::SyntaxChecked,
            last_checked_at: Some("2026-05-20T14:41:00Z"),
            environment_scope_ref: Some("env:syntax-check"),
            version_scope_ref: Some("rev:aureline-docs:2026.05.20-beta"),
            freshness_class: DocsFreshnessClass::WarmCached,
            suppression_state: DocsFindingSuppressionState::Active,
            suppression_ref: None,
            evidence_refs: &["evidence:stale-example-scan:help.snippet-01"],
        }),
        finding_row(FindingSeed {
            finding_id: "docs-finding:module-doc:command-output-drift",
            artifact_kind: DocsArtifactKind::ModuleDoc,
            target_artifact_ref: "docs-artifact:module-doc:aureline-docs",
            target_section_ref: Some("section:module-doc#example-03"),
            finding_class: DocsFindingClass::CommandOutputDrift,
            detection_state: DocsFindingDetectionState::ProvenBroken,
            validation_mode: DocsExampleValidationMode::ExecutedLocally,
            last_checked_at: Some("2026-05-20T14:42:00Z"),
            environment_scope_ref: Some("env:local-runner"),
            version_scope_ref: Some("rev:aureline-docs:2026.05.20-beta"),
            freshness_class: DocsFreshnessClass::AuthoritativeLive,
            suppression_state: DocsFindingSuppressionState::Active,
            suppression_ref: None,
            evidence_refs: &["evidence:snippet-validation:module-doc.example-03"],
        }),
        finding_row(FindingSeed {
            finding_id: "docs-finding:reference:api-mismatch",
            artifact_kind: DocsArtifactKind::SymbolReference,
            target_artifact_ref: "docs-artifact:reference:docs-node",
            target_section_ref: Some("section:reference#docs-node"),
            finding_class: DocsFindingClass::ApiMismatch,
            detection_state: DocsFindingDetectionState::ProvenBroken,
            validation_mode: DocsExampleValidationMode::ExecutedRemotely,
            last_checked_at: Some("2026-05-20T14:43:00Z"),
            environment_scope_ref: Some("env:remote-runner"),
            version_scope_ref: Some("rev:aureline-docs:2026.05.20-beta"),
            freshness_class: DocsFreshnessClass::AuthoritativeLive,
            suppression_state: DocsFindingSuppressionState::Active,
            suppression_ref: None,
            evidence_refs: &["evidence:contract-change:api.docs-node"],
        }),
        finding_row(FindingSeed {
            finding_id: "docs-finding:benchmark-copy:unverifiable",
            artifact_kind: DocsArtifactKind::BenchmarkCopy,
            target_artifact_ref: "docs-artifact:benchmark-copy:beta-2",
            target_section_ref: Some("section:benchmark#throughput"),
            finding_class: DocsFindingClass::UnverifiableBenchmarkCopy,
            detection_state: DocsFindingDetectionState::UnchangedUnverified,
            validation_mode: DocsExampleValidationMode::Unsupported,
            last_checked_at: None,
            environment_scope_ref: Some("env:no-benchmark-harness"),
            version_scope_ref: Some("release:beta-2"),
            freshness_class: DocsFreshnessClass::Unverified,
            suppression_state: DocsFindingSuppressionState::Active,
            suppression_ref: None,
            evidence_refs: &["evidence:benchmark-publication-pack:missing"],
        }),
        finding_row(FindingSeed {
            finding_id: "docs-finding:settings:renamed-setting",
            artifact_kind: DocsArtifactKind::HelpArticle,
            target_artifact_ref: "docs-artifact:help:settings",
            target_section_ref: Some("section:help#settings"),
            finding_class: DocsFindingClass::RenamedSetting,
            detection_state: DocsFindingDetectionState::UnchangedUnverified,
            validation_mode: DocsExampleValidationMode::Skipped,
            last_checked_at: None,
            environment_scope_ref: None,
            version_scope_ref: Some("rev:aureline-docs:2026.05.20-beta"),
            freshness_class: DocsFreshnessClass::WarmCached,
            suppression_state: DocsFindingSuppressionState::SuppressedUntilReviewed,
            suppression_ref: Some("suppression:docs-finding:settings.renamed-setting:owner-review"),
            evidence_refs: &["evidence:settings-registry:renamed-setting"],
        }),
        finding_row(FindingSeed {
            finding_id: "docs-finding:reference:renamed-symbol",
            artifact_kind: DocsArtifactKind::SymbolReference,
            target_artifact_ref: "docs-artifact:reference:symbol-index",
            target_section_ref: Some("section:reference#symbol"),
            finding_class: DocsFindingClass::RenamedSymbol,
            detection_state: DocsFindingDetectionState::SuspectedStale,
            validation_mode: DocsExampleValidationMode::Stale,
            last_checked_at: Some("2026-05-12T09:00:00Z"),
            environment_scope_ref: Some("env:symbol-index"),
            version_scope_ref: Some("rev:aureline-docs:2026.05.12-beta"),
            freshness_class: DocsFreshnessClass::Stale,
            suppression_state: DocsFindingSuppressionState::Acknowledged,
            suppression_ref: None,
            evidence_refs: &["evidence:symbol-linked-reference:renamed"],
        }),
        finding_row(FindingSeed {
            finding_id: "docs-finding:migration-notes:missing-note",
            artifact_kind: DocsArtifactKind::MigrationNotes,
            target_artifact_ref: "docs-artifact:migration-notes:beta-2",
            target_section_ref: None,
            finding_class: DocsFindingClass::MissingMigrationNote,
            detection_state: DocsFindingDetectionState::UnchangedUnverified,
            validation_mode: DocsExampleValidationMode::NotValidated,
            last_checked_at: None,
            environment_scope_ref: None,
            version_scope_ref: Some("release:beta-2"),
            freshness_class: DocsFreshnessClass::Unverified,
            suppression_state: DocsFindingSuppressionState::Active,
            suppression_ref: None,
            evidence_refs: &["evidence:contract-change:api.docs-node"],
        }),
    ];

    let maintenance_rows = vec![
        DocsMaintenanceRow {
            record_kind: DOCS_MAINTENANCE_ROW_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            row_id: "docs-maintenance-row:readme".to_owned(),
            artifact_kind: DocsArtifactKind::Readme,
            artifact_ref: "docs-artifact:readme:root".to_owned(),
            audience_scope: DocsAudienceScope::PublicReader,
            publish_scope: DocsPublishScope::default(),
            publish_boundary_state: DocsPublishBoundaryState::LocalOnly,
            pending_suggestion_count: 1,
            pending_finding_count: 1,
            validation_freshness: DocsFreshnessClass::AuthoritativeLive,
            version_match_state: VersionMatchState::ExactBuildMatch,
            last_validated_at: Some("2026-05-20T14:40:00Z".to_owned()),
            publish_boundary_notes: Vec::new(),
            suggestion_card_refs: refs(&["docs-suggestion-card:readme:code-diff"]),
            finding_row_refs: refs(&["docs-finding:readme:broken-link"]),
            apply_export_action: Some(action(
                "action:docs-maintenance.apply-local:readme",
                "Apply locally",
            )),
            surface_refs: refs(&["surface:docs_maintenance:rows"]),
        },
        DocsMaintenanceRow {
            record_kind: DOCS_MAINTENANCE_ROW_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            row_id: "docs-maintenance-row:changelog".to_owned(),
            artifact_kind: DocsArtifactKind::Changelog,
            artifact_ref: "docs-artifact:changelog:beta".to_owned(),
            audience_scope: DocsAudienceScope::ReleaseManager,
            publish_scope: DocsPublishScope {
                branch_scope: Some("branch:release/beta-2".to_owned()),
                release_scope: Some("release:beta-2".to_owned()),
                channel_scope: Some("beta".to_owned()),
            },
            publish_boundary_state: DocsPublishBoundaryState::PublishHandoffScoped,
            pending_suggestion_count: 1,
            pending_finding_count: 2,
            validation_freshness: DocsFreshnessClass::WarmCached,
            version_match_state: VersionMatchState::CompatibleMinorDrift,
            last_validated_at: Some("2026-05-20T14:43:00Z".to_owned()),
            publish_boundary_notes: refs(&[
                "Beta changelog — scoped to the beta channel; not stable docs.",
                "Publish handoff required before this leaves review.",
            ]),
            suggestion_card_refs: refs(&["docs-suggestion-card:release-notes:drift"]),
            finding_row_refs: refs(&[
                "docs-finding:reference:api-mismatch",
                "docs-finding:benchmark-copy:unverifiable",
            ]),
            apply_export_action: Some(action(
                "action:docs-maintenance.export-handoff:changelog",
                "Export publish handoff",
            )),
            surface_refs: refs(&["surface:docs_maintenance:rows"]),
        },
        DocsMaintenanceRow {
            record_kind: DOCS_MAINTENANCE_ROW_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            row_id: "docs-maintenance-row:onboarding".to_owned(),
            artifact_kind: DocsArtifactKind::OnboardingNote,
            artifact_ref: "docs-artifact:onboarding-note:first-run".to_owned(),
            audience_scope: DocsAudienceScope::EndUser,
            publish_scope: DocsPublishScope {
                branch_scope: Some("branch:docs/onboarding-beta".to_owned()),
                release_scope: None,
                channel_scope: Some("beta".to_owned()),
            },
            publish_boundary_state: DocsPublishBoundaryState::ReviewHandoffScoped,
            pending_suggestion_count: 2,
            pending_finding_count: 2,
            validation_freshness: DocsFreshnessClass::WarmCached,
            version_match_state: VersionMatchState::CompatibleMinorDrift,
            last_validated_at: Some("2026-05-20T14:41:00Z".to_owned()),
            publish_boundary_notes: refs(&[
                "Onboarding note staged for review; stays inside review scope.",
            ]),
            suggestion_card_refs: refs(&[
                "docs-suggestion-card:onboarding-note:human-note",
                "docs-suggestion-card:help-article:stale-example",
            ]),
            finding_row_refs: refs(&[
                "docs-finding:settings:renamed-setting",
                "docs-finding:help-article:stale-example",
            ]),
            apply_export_action: Some(action(
                "action:docs-maintenance.export-review:onboarding",
                "Export review packet",
            )),
            surface_refs: refs(&["surface:docs_maintenance:rows"]),
        },
        DocsMaintenanceRow {
            record_kind: DOCS_MAINTENANCE_ROW_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
            row_id: "docs-maintenance-row:release-notes".to_owned(),
            artifact_kind: DocsArtifactKind::ReleaseNotes,
            artifact_ref: "docs-artifact:release-notes:beta-2".to_owned(),
            audience_scope: DocsAudienceScope::PublicReader,
            publish_scope: DocsPublishScope::default(),
            publish_boundary_state: DocsPublishBoundaryState::BlockedUnscoped,
            pending_suggestion_count: 2,
            pending_finding_count: 3,
            validation_freshness: DocsFreshnessClass::Unverified,
            version_match_state: VersionMatchState::UnknownTargetBuild,
            last_validated_at: None,
            publish_boundary_notes: refs(&[
                "Publish blocked: no branch/release/channel scope was provided.",
                "Add a scope to enable a publish handoff.",
            ]),
            suggestion_card_refs: refs(&[
                "docs-suggestion-card:module-doc:failing-snippet",
                "docs-suggestion-card:migration-notes:contract-change",
            ]),
            finding_row_refs: refs(&[
                "docs-finding:module-doc:command-output-drift",
                "docs-finding:reference:renamed-symbol",
                "docs-finding:migration-notes:missing-note",
            ]),
            apply_export_action: None,
            surface_refs: refs(&["surface:docs_maintenance:rows"]),
        },
    ];

    DocsMaintenanceContract {
        record_kind: DOCS_MAINTENANCE_CONTRACT_RECORD_KIND.to_owned(),
        schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
        contract_id: DOCS_PREVIEW_AND_MAINTENANCE_CONTRACT_ID.to_owned(),
        contract_version_ref: DOCS_PREVIEW_AND_MAINTENANCE_VERSION_REF.to_owned(),
        generated_at: GENERATED_AT.to_owned(),
        contract_refs: BTreeMap::from([
            (
                "suggestion_card_schema".to_owned(),
                DOCS_SUGGESTION_CARD_SCHEMA_REF.to_owned(),
            ),
            (
                "maintenance_row_schema".to_owned(),
                DOCS_MAINTENANCE_ROW_SCHEMA_REF.to_owned(),
            ),
            (
                "help_doc".to_owned(),
                "docs/help/m3/docs_preview_and_maintenance_beta.md".to_owned(),
            ),
            (
                "fixtures".to_owned(),
                "fixtures/docs/m3/docs_preview_and_maintenance/".to_owned(),
            ),
        ]),
        handoff_banner,
        preview_headers,
        suggestion_cards,
        finding_rows,
        maintenance_rows,
        omitted_material_classes: refs(&[
            "raw_document_body",
            "rendered_html",
            "raw_source_file",
            "raw_docs_url",
            "private_workspace_path",
            "account_identifier",
        ]),
    }
}

/// Returns the seeded docs preview-and-maintenance surface projection.
pub fn seeded_docs_preview_and_maintenance_surface_projection() -> DocsMaintenanceSurfaceProjection
{
    seeded_docs_preview_and_maintenance_contract().surface_projection()
}

/// Returns the seeded docs preview-and-maintenance review packet.
pub fn seeded_docs_preview_and_maintenance_review_packet() -> DocsMaintenanceReviewPacket {
    seeded_docs_preview_and_maintenance_contract().review_packet(
        "docs-maintenance-review-packet:preview-and-maintenance:001",
        GENERATED_AT,
    )
}

/// Validates all seeded docs preview-and-maintenance records.
pub fn validate_seeded_docs_preview_and_maintenance() -> Result<(), Vec<DocsMaintenanceFinding>> {
    let contract = seeded_docs_preview_and_maintenance_contract();
    let packet = contract.review_packet(
        "docs-maintenance-review-packet:preview-and-maintenance:001",
        GENERATED_AT,
    );
    let mut findings = contract.validate();
    if let Err(mut packet_findings) = packet.validate_against_contract(&contract) {
        findings.append(&mut packet_findings);
    }
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

struct FindingSeed {
    finding_id: &'static str,
    artifact_kind: DocsArtifactKind,
    target_artifact_ref: &'static str,
    target_section_ref: Option<&'static str>,
    finding_class: DocsFindingClass,
    detection_state: DocsFindingDetectionState,
    validation_mode: DocsExampleValidationMode,
    last_checked_at: Option<&'static str>,
    environment_scope_ref: Option<&'static str>,
    version_scope_ref: Option<&'static str>,
    freshness_class: DocsFreshnessClass,
    suppression_state: DocsFindingSuppressionState,
    suppression_ref: Option<&'static str>,
    evidence_refs: &'static [&'static str],
}

fn finding_row(seed: FindingSeed) -> DocsExampleFindingRow {
    DocsExampleFindingRow {
        record_kind: DOCS_EXAMPLE_FINDING_ROW_RECORD_KIND.to_owned(),
        schema_version: DOCS_MAINTENANCE_SCHEMA_VERSION,
        finding_id: seed.finding_id.to_owned(),
        artifact_kind: seed.artifact_kind,
        target_artifact_ref: seed.target_artifact_ref.to_owned(),
        target_section_ref: seed.target_section_ref.map(str::to_owned),
        finding_class: seed.finding_class,
        detection_state: seed.detection_state,
        validation_mode: seed.validation_mode,
        last_checked_at: seed.last_checked_at.map(str::to_owned),
        environment_scope_ref: seed.environment_scope_ref.map(str::to_owned),
        version_scope_ref: seed.version_scope_ref.map(str::to_owned),
        freshness_class: seed.freshness_class,
        open_failing_source_action: action(
            &format!(
                "action:docs-finding.open-failing-source:{}",
                seed.finding_id
            ),
            OPEN_FAILING_SOURCE_ACTION_LABEL,
        ),
        suppression_state: seed.suppression_state,
        suppression_ref: seed.suppression_ref.map(str::to_owned),
        evidence_refs: refs(seed.evidence_refs),
        surface_refs: refs(&["surface:docs_maintenance:finding_rows"]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_contract_validates() {
        validate_seeded_docs_preview_and_maintenance()
            .expect("seeded docs preview-and-maintenance records validate");
    }

    #[test]
    fn preview_modes_and_canonical_disclosure_are_covered() {
        let contract = seeded_docs_preview_and_maintenance_contract();
        for mode in [
            DocsPreviewMode::Source,
            DocsPreviewMode::Split,
            DocsPreviewMode::Rendered,
        ] {
            let header = contract
                .preview_headers
                .iter()
                .find(|header| header.preview_mode == mode)
                .expect("preview mode is covered");
            assert!(header.commonmark_baseline);
            assert!(header.mode_toggle_keyboard_reachable);
            assert!(header.open_source_action.keyboard_reachable);
            if mode.renders_preview() {
                assert!(!header.rendered_is_not_canonical_note.trim().is_empty());
            }
        }
    }

    #[test]
    fn suggestion_cards_are_diff_based_and_evidence_backed() {
        let contract = seeded_docs_preview_and_maintenance_contract();
        for card in &contract.suggestion_cards {
            assert!(card.silent_rewrite_blocked);
            assert!(!card.evidence_refs.is_empty());
            if card.apply_posture.requires_review_diff() {
                assert!(card
                    .review_diff_ref
                    .as_deref()
                    .is_some_and(|value| !value.is_empty()));
            }
        }
        // Every trigger source is represented.
        for trigger in [
            DocsSuggestionTrigger::CodeDiff,
            DocsSuggestionTrigger::StaleExample,
            DocsSuggestionTrigger::ReleaseNoteDrift,
            DocsSuggestionTrigger::FailingSnippet,
            DocsSuggestionTrigger::ContractChange,
            DocsSuggestionTrigger::HumanNote,
        ] {
            assert!(contract
                .suggestion_cards
                .iter()
                .any(|card| card.trigger == trigger));
        }
    }

    #[test]
    fn finding_rows_distinguish_all_validation_modes() {
        let contract = seeded_docs_preview_and_maintenance_contract();
        for mode in [
            DocsExampleValidationMode::Rendered,
            DocsExampleValidationMode::SyntaxChecked,
            DocsExampleValidationMode::ExecutedLocally,
            DocsExampleValidationMode::ExecutedRemotely,
            DocsExampleValidationMode::Unsupported,
            DocsExampleValidationMode::Skipped,
            DocsExampleValidationMode::Stale,
            DocsExampleValidationMode::NotValidated,
        ] {
            assert!(
                contract
                    .finding_rows
                    .iter()
                    .any(|row| row.validation_mode == mode),
                "validation mode {} is exercised",
                mode.as_str()
            );
        }
        assert!(contract.finding_rows.iter().any(|row| row.suppression_state
            == DocsFindingSuppressionState::SuppressedUntilReviewed
            && row.suppression_ref.is_some()));
    }

    #[test]
    fn maintenance_rows_preserve_publish_scope() {
        let contract = seeded_docs_preview_and_maintenance_contract();
        let scoped = contract
            .maintenance_rows
            .iter()
            .find(|row| row.publish_boundary_state.requires_scope())
            .expect("a scoped maintenance row exists");
        assert!(scoped.publish_scope.is_scoped());
        assert!(!scoped.publish_boundary_notes.is_empty());

        assert!(contract
            .maintenance_rows
            .iter()
            .any(|row| row.publish_boundary_state == DocsPublishBoundaryState::LocalOnly));
        assert!(contract
            .maintenance_rows
            .iter()
            .any(
                |row| row.publish_boundary_state == DocsPublishBoundaryState::BlockedUnscoped
                    && row.apply_export_action.is_none()
            ));
    }

    #[test]
    fn review_packet_is_metadata_only() {
        let contract = seeded_docs_preview_and_maintenance_contract();
        let packet = seeded_docs_preview_and_maintenance_review_packet();
        packet
            .validate_against_contract(&contract)
            .expect("review packet validates");
        assert!(!packet.raw_document_bodies_exported);
        assert!(packet.handoff_banner.screenshot_free_review);
        let json = packet.export_safe_json();
        assert!(!json.contains("://"));
    }
}
