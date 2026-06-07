//! Stable AI review-assist finding, scope, publish, and resolution truth.
//!
//! This module owns the export-safe packet used by AI review cards, hosted
//! review overlays, browser or companion follow-up, CLI/headless replay, and
//! support packets. It keeps AI review assist scoped to evidence-backed
//! suggestions: findings cite the analyzed scope, affected hunks, repo
//! instruction or check source, review-pack digest, and evidence-packet
//! lineage; publish sheets preview exactly where and what would be written; and
//! resolution rows preserve local-vs-hosted state without implying provider
//! write authority.
//!
//! The packet is metadata-oriented. It carries ids, refs, closed vocabulary
//! values, redaction-aware preview text, and short labels. Raw diffs, raw
//! provider payloads, provider URLs, credentials, prompt text, and secret values
//! stay outside this boundary.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AiReviewAssistTruthPacket`].
pub const AI_REVIEW_ASSIST_TRUTH_RECORD_KIND: &str = "ai_review_assist_truth";

/// Schema version for AI review-assist truth records.
pub const AI_REVIEW_ASSIST_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the AI review-assist truth boundary schema.
pub const AI_REVIEW_ASSIST_TRUTH_SCHEMA_REF: &str =
    "schemas/ai/ai-review-assist-and-publish-truth.schema.json";

/// Repo-relative path of the AI review-assist contract doc.
pub const AI_REVIEW_ASSIST_TRUTH_AI_DOC_REF: &str =
    "docs/ai/m4/ai-review-assist-and-publish-truth.md";

/// Repo-relative path of the stable review-pack evaluator contract this lane uses.
pub const AI_REVIEW_ASSIST_REVIEW_PACK_CONTRACT_REF: &str =
    "docs/review/m4/review-pack-evaluator-and-local-ci-parity.md";

/// Repo-relative path of the protected AI review-assist fixture directory.
pub const AI_REVIEW_ASSIST_TRUTH_FIXTURE_DIR: &str =
    "fixtures/ai/m4/ai-review-assist-and-publish-truth";

/// Repo-relative path of the checked AI review-assist support export.
pub const AI_REVIEW_ASSIST_TRUTH_ARTIFACT_REF: &str =
    "artifacts/ai/m4/ai-review-assist-and-publish-truth/support_export.json";

/// Repo-relative path of the checked AI review-assist Markdown summary.
pub const AI_REVIEW_ASSIST_TRUTH_SUMMARY_REF: &str =
    "artifacts/ai/m4/ai-review-assist-and-publish-truth/summary.md";

/// Scope class an AI review run analyzed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewScopeClass {
    /// User-selected diff ranges in the local review workspace.
    SelectedDiff,
    /// Local uncommitted worktree changes.
    UncommittedChanges,
    /// Hosted pull/merge request or equivalent review object.
    HostedReviewObject,
}

impl ReviewScopeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedDiff => "selected_diff",
            Self::UncommittedChanges => "uncommitted_changes",
            Self::HostedReviewObject => "hosted_review_object",
        }
    }
}

/// Freshness class for a scope-bound review run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewScopeFreshnessClass {
    /// The analyzed diff fingerprint still matches the active scope.
    Current,
    /// The diff changed materially after the AI review run.
    OutdatedDiffChanged,
    /// The current scope can be inspected, but a rerun is recommended.
    RerunRecommended,
}

impl ReviewScopeFreshnessClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::OutdatedDiffChanged => "outdated_diff_changed",
            Self::RerunRecommended => "rerun_recommended",
        }
    }

    const fn requires_non_fresh_resolution(self) -> bool {
        matches!(self, Self::OutdatedDiffChanged | Self::RerunRecommended)
    }
}

/// Action offered when a scope is stale or needs a new run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewScopeRerunActionClass {
    /// No rerun is needed.
    NotNeeded,
    /// Rerun the same review-check pack against the current scope.
    RerunSameReviewPack,
    /// Compare the current scope against the previously analyzed one.
    CompareChangedScope,
}

impl ReviewScopeRerunActionClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNeeded => "not_needed",
            Self::RerunSameReviewPack => "rerun_same_review_pack",
            Self::CompareChangedScope => "compare_changed_scope",
        }
    }
}

/// Source that shaped an AI review finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepoInstructionCheckSourceClass {
    /// Repository-authored AI instruction bundle.
    RepoInstructionBundle,
    /// Signed designated AI policy file.
    DesignatedPolicyFile,
    /// Review-pack required check.
    ReviewPackRequiredCheck,
    /// Review-pack advisory check.
    ReviewPackAdvisoryCheck,
    /// Local lint, diagnostic, or analyzer signal.
    LocalDiagnostic,
    /// Provider check mirror that remains overlay truth.
    ProviderCheckMirror,
}

impl RepoInstructionCheckSourceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepoInstructionBundle => "repo_instruction_bundle",
            Self::DesignatedPolicyFile => "designated_policy_file",
            Self::ReviewPackRequiredCheck => "review_pack_required_check",
            Self::ReviewPackAdvisoryCheck => "review_pack_advisory_check",
            Self::LocalDiagnostic => "local_diagnostic",
            Self::ProviderCheckMirror => "provider_check_mirror",
        }
    }
}

/// AI review finding category.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiReviewFindingClass {
    /// Potential correctness, safety, or reliability concern.
    RiskOrBugConcern,
    /// Missing or weakened test coverage.
    MissingTestCoverage,
    /// Security, dependency, or policy concern.
    DependencyOrSecurityConcern,
    /// Repository rule or check was triggered.
    RepoInstructionCheckFired,
    /// Performance or complexity concern.
    PerformanceOrComplexityConcern,
    /// Informational impact-area hint.
    ImpactAreaHint,
}

impl AiReviewFindingClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RiskOrBugConcern => "risk_or_bug_concern",
            Self::MissingTestCoverage => "missing_test_coverage",
            Self::DependencyOrSecurityConcern => "dependency_or_security_concern",
            Self::RepoInstructionCheckFired => "repo_instruction_check_fired",
            Self::PerformanceOrComplexityConcern => "performance_or_complexity_concern",
            Self::ImpactAreaHint => "impact_area_hint",
        }
    }
}

/// Advisory severity assigned to an AI review finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiReviewSeverityClass {
    /// High-severity advisory concern.
    HighAdvisory,
    /// Medium-severity advisory concern.
    MediumAdvisory,
    /// Low-severity advisory concern.
    LowAdvisory,
    /// Informational advisory.
    Informational,
}

impl AiReviewSeverityClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighAdvisory => "high_advisory",
            Self::MediumAdvisory => "medium_advisory",
            Self::LowAdvisory => "low_advisory",
            Self::Informational => "informational",
        }
    }
}

/// Confidence assigned to an AI review finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiReviewConfidenceClass {
    /// Evidence-backed and cited to the analyzed scope.
    EvidenceBacked,
    /// Inferred from surrounding evidence and needs human review.
    Inferred,
    /// Low confidence and blocked from hosted publication.
    LowConfidence,
}

impl AiReviewConfidenceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceBacked => "evidence_backed",
            Self::Inferred => "inferred",
            Self::LowConfidence => "low_confidence",
        }
    }

    const fn blocks_provider_publish(self) -> bool {
        matches!(self, Self::LowConfidence)
    }
}

/// Current resolution memory state for one AI review finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiReviewResolutionState {
    /// Finding is open and local.
    Open,
    /// User or policy dismissed the finding with recorded source.
    Dismissed,
    /// Finding was published to a local/export/provider destination.
    Published,
    /// Finding is outdated because analyzed scope drifted.
    Outdated,
    /// Finding was suppressed by a policy or repo instruction source.
    Suppressed,
}

impl AiReviewResolutionState {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Dismissed => "dismissed",
            Self::Published => "published",
            Self::Outdated => "outdated",
            Self::Suppressed => "suppressed",
        }
    }

    const fn admits_stale_scope(self) -> bool {
        matches!(self, Self::Outdated)
    }
}

/// Class of outbound review destination previewed by a publish sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishDestinationClass {
    /// No outbound destination; the finding remains local.
    LocalOnly,
    /// Hosted provider thread or inline comment.
    ProviderThreadComment,
    /// Hosted provider check annotation.
    ProviderCheckAnnotation,
    /// Portable review bundle export.
    ReviewBundleExport,
    /// Support export packet.
    SupportExport,
}

impl PublishDestinationClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ProviderThreadComment => "provider_thread_comment",
            Self::ProviderCheckAnnotation => "provider_check_annotation",
            Self::ReviewBundleExport => "review_bundle_export",
            Self::SupportExport => "support_export",
        }
    }

    const fn is_provider_destination(self) -> bool {
        matches!(
            self,
            Self::ProviderThreadComment | Self::ProviderCheckAnnotation
        )
    }
}

/// Provider write posture observed before a publish action is offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderWriteAccessClass {
    /// Provider write access is available for this destination.
    ProviderWriteAvailable,
    /// Provider write access is missing and local/copy/export fallback applies.
    MissingProviderWriteAccess,
    /// Provider write was blocked by policy.
    ProviderWriteBlockedByPolicy,
    /// Provider write does not apply to a local or export destination.
    NotApplicableLocalOrExport,
}

impl ProviderWriteAccessClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderWriteAvailable => "provider_write_available",
            Self::MissingProviderWriteAccess => "missing_provider_write_access",
            Self::ProviderWriteBlockedByPolicy => "provider_write_blocked_by_policy",
            Self::NotApplicableLocalOrExport => "not_applicable_local_or_export",
        }
    }
}

/// User-facing action class admitted by a publish sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublishActionClass {
    /// Write the previewed content to the previewed destination.
    PublishToDestination,
    /// Copy the preview text without writing to a provider.
    CopyOnly,
    /// Export the finding into a portable packet.
    ExportLocalPacket,
    /// Keep the finding local.
    KeepLocal,
    /// Block provider publish and fall back to local/copy/export paths.
    BlockedProviderWriteMissing,
}

impl PublishActionClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublishToDestination => "publish_to_destination",
            Self::CopyOnly => "copy_only",
            Self::ExportLocalPacket => "export_local_packet",
            Self::KeepLocal => "keep_local",
            Self::BlockedProviderWriteMissing => "blocked_provider_write_missing",
        }
    }

    const fn is_provider_write(self) -> bool {
        matches!(self, Self::PublishToDestination)
    }

    const fn is_missing_write_downgrade(self) -> bool {
        matches!(
            self,
            Self::CopyOnly
                | Self::ExportLocalPacket
                | Self::KeepLocal
                | Self::BlockedProviderWriteMissing
        )
    }
}

/// Attribution state disclosed on a publish sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributionStateClass {
    /// Posted by the user with AI assist disclosed.
    PostedAsUserWithAiAssistDisclosed,
    /// Kept local with no hosted attribution.
    KeptLocalNoAttribution,
    /// Exported packet discloses AI assist.
    ExportedWithAiAssistDisclosed,
}

impl AttributionStateClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostedAsUserWithAiAssistDisclosed => "posted_as_user_with_ai_assist_disclosed",
            Self::KeptLocalNoAttribution => "kept_local_no_attribution",
            Self::ExportedWithAiAssistDisclosed => "exported_with_ai_assist_disclosed",
        }
    }
}

/// Redaction note attached to a publish preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionNoteClass {
    /// No redaction was required.
    NoRedactionRequired,
    /// Internal identifier was removed before preview.
    InternalIdentifierRedacted,
    /// Credential or secret handle was redacted.
    CredentialHandleRedacted,
    /// Redaction requires user review before publish.
    RedactionRequiresUserReview,
}

impl RedactionNoteClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRedactionRequired => "no_redaction_required",
            Self::InternalIdentifierRedacted => "internal_identifier_redacted",
            Self::CredentialHandleRedacted => "credential_handle_redacted",
            Self::RedactionRequiresUserReview => "redaction_requires_user_review",
        }
    }
}

/// Surface that consumes the same AI review-assist truth packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiReviewConsumerSurface {
    /// Desktop review workspace.
    DesktopReviewWorkspace,
    /// CLI/headless replay or JSON output.
    CliHeadless,
    /// Browser or companion review follow-up.
    BrowserCompanion,
    /// Support/export packet.
    SupportExport,
}

impl AiReviewConsumerSurface {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopReviewWorkspace => "desktop_review_workspace",
            Self::CliHeadless => "cli_headless",
            Self::BrowserCompanion => "browser_companion",
            Self::SupportExport => "support_export",
        }
    }

    /// Consumer surfaces required before the stable lane can be claimed.
    pub const fn required_surfaces() -> [Self; 4] {
        [
            Self::DesktopReviewWorkspace,
            Self::CliHeadless,
            Self::BrowserCompanion,
            Self::SupportExport,
        ]
    }
}

/// Export-safe lineage carried by each durable object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewObjectLineage {
    /// Stable object id.
    pub object_id: String,
    /// Evidence packet refs that back the object.
    pub evidence_packet_refs: Vec<String>,
    /// Export or support packet refs that preserve the object.
    pub export_lineage_refs: Vec<String>,
    /// True when this object is safe to include in support/export packets.
    pub export_safe: bool,
}

/// Affected file/hunk identity for one finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedReviewHunk {
    /// Opaque affected file ref.
    pub file_ref: String,
    /// Opaque hunk or structured range ref.
    pub hunk_ref: String,
    /// Diff fingerprint observed when the hunk was analyzed.
    pub diff_fingerprint_ref: String,
}

/// Review-scope selector bound to an AI review run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewScopeSelector {
    /// Stable selector id.
    pub selector_id: String,
    /// Scope class.
    pub scope_class: ReviewScopeClass,
    /// Base identity ref for selected, uncommitted, or hosted scopes.
    pub base_identity_ref: String,
    /// Head identity ref for selected, uncommitted, or hosted scopes.
    pub head_identity_ref: String,
    /// Review object ref when the scope is hosted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hosted_review_object_ref: Option<String>,
    /// Review pack digest used for the scope.
    pub review_pack_digest_ref: String,
    /// Scope freshness class.
    pub freshness_class: ReviewScopeFreshnessClass,
    /// Rerun action surfaced for stale or current scopes.
    pub rerun_action: ReviewScopeRerunActionClass,
    /// Material diff change ref when the scope became stale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub material_diff_change_ref: Option<String>,
    /// Scope lineage.
    pub lineage: AiReviewObjectLineage,
}

/// One durable AI review finding row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewFindingRow {
    /// Stable finding id.
    pub finding_id: String,
    /// Finding title safe for review/export.
    pub title: String,
    /// Finding class.
    pub finding_class: AiReviewFindingClass,
    /// Advisory severity.
    pub severity_class: AiReviewSeverityClass,
    /// Confidence class.
    pub confidence_class: AiReviewConfidenceClass,
    /// Selector id that was analyzed.
    pub scope_selector_id: String,
    /// Review pack digest used to mint this finding.
    pub review_pack_digest_ref: String,
    /// Source that shaped this finding.
    pub instruction_check_source: RepoInstructionCheckSourceClass,
    /// Opaque refs to source instructions, checks, or diagnostics.
    pub instruction_check_refs: Vec<String>,
    /// Affected files or hunks.
    pub affected_hunks: Vec<AffectedReviewHunk>,
    /// Evidence packet and export lineage.
    pub lineage: AiReviewObjectLineage,
    /// Current scope freshness copied from the selector at projection time.
    pub scope_freshness_class: ReviewScopeFreshnessClass,
    /// Current resolution state.
    pub resolution_state: AiReviewResolutionState,
    /// Publish sheet id when a publish/copy/export path exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_sheet_id: Option<String>,
}

/// Publish-to-review sheet previewing one outbound or local fallback action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishToReviewSheet {
    /// Stable publish sheet id.
    pub publish_sheet_id: String,
    /// Finding id this sheet publishes, copies, exports, or keeps local.
    pub finding_id: String,
    /// Provider ref or local/export target ref.
    pub provider_ref: String,
    /// Exact destination class.
    pub destination_class: PublishDestinationClass,
    /// Opaque provider thread/comment/check destination ref.
    pub destination_ref: String,
    /// Provider write access observed before the action was offered.
    pub provider_write_access: ProviderWriteAccessClass,
    /// Exact redaction-aware outbound text preview.
    pub outbound_text_preview: String,
    /// Attribution state shown before action.
    pub attribution_state: AttributionStateClass,
    /// Redaction note shown before action.
    pub redaction_note: RedactionNoteClass,
    /// Admitted action.
    pub action_class: PublishActionClass,
    /// Fallback actions available when provider write is missing.
    pub fallback_actions: Vec<PublishActionClass>,
    /// Publish sheet lineage.
    pub lineage: AiReviewObjectLineage,
}

/// Resolution memory row for one finding over time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolutionMemoryRow {
    /// Stable resolution id.
    pub resolution_id: String,
    /// Finding id this resolution row records.
    pub finding_id: String,
    /// Resolution state.
    pub state: AiReviewResolutionState,
    /// Actor ref that caused the state.
    pub actor_ref: String,
    /// Source ref or class that caused the state.
    pub source_ref: String,
    /// Timestamp for the resolution state.
    pub timestamp: String,
    /// Reopen lineage refs for dismissed, published, outdated, or suppressed rows.
    pub reopen_lineage_refs: Vec<String>,
    /// Predecessor resolution id when this row supersedes earlier memory.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predecessor_resolution_id: Option<String>,
    /// Publish sheet id when the state is published.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_sheet_id: Option<String>,
    /// Resolution lineage.
    pub lineage: AiReviewObjectLineage,
}

/// Consumer projection proving cross-surface parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewConsumerProjection {
    /// Consumer surface.
    pub surface: AiReviewConsumerSurface,
    /// True when analyzed scope and base/head identity are visible.
    pub preserves_scope_identity: bool,
    /// True when repo instruction/check source is visible.
    pub preserves_instruction_check_source: bool,
    /// True when publish destination and action state are visible.
    pub preserves_publish_destination_truth: bool,
    /// True when resolution memory is visible.
    pub preserves_resolution_memory: bool,
    /// True when local/copy/export fallback remains available.
    pub preserves_local_copy_export_fallback: bool,
    /// Export ref consumed by this surface.
    pub export_ref: String,
}

/// Constructor input for [`AiReviewAssistTruthPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiReviewAssistTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Review workspace or workflow id.
    pub review_workspace_ref: String,
    /// Review-pack digest shared by all rows.
    pub review_pack_digest_ref: String,
    /// Packet display label.
    pub display_label: String,
    /// Scope selectors used by this packet.
    pub scope_selectors: Vec<ReviewScopeSelector>,
    /// AI review finding rows.
    pub findings: Vec<AiReviewFindingRow>,
    /// Publish-to-review sheets.
    pub publish_sheets: Vec<PublishToReviewSheet>,
    /// Resolution memory rows.
    pub resolution_memory: Vec<ResolutionMemoryRow>,
    /// Consumer projections.
    pub consumer_projections: Vec<AiReviewConsumerProjection>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI review-assist truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewAssistTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Review workspace or workflow id.
    pub review_workspace_ref: String,
    /// Review-pack digest shared by all rows.
    pub review_pack_digest_ref: String,
    /// Packet display label.
    pub display_label: String,
    /// Scope selectors used by this packet.
    pub scope_selectors: Vec<ReviewScopeSelector>,
    /// AI review finding rows.
    pub findings: Vec<AiReviewFindingRow>,
    /// Publish-to-review sheets.
    pub publish_sheets: Vec<PublishToReviewSheet>,
    /// Resolution memory rows.
    pub resolution_memory: Vec<ResolutionMemoryRow>,
    /// Consumer projections.
    pub consumer_projections: Vec<AiReviewConsumerProjection>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AiReviewAssistTruthPacket {
    /// Builds an AI review-assist truth packet from stable-lane input.
    pub fn new(input: AiReviewAssistTruthPacketInput) -> Self {
        Self {
            record_kind: AI_REVIEW_ASSIST_TRUTH_RECORD_KIND.to_owned(),
            schema_version: AI_REVIEW_ASSIST_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            review_workspace_ref: input.review_workspace_ref,
            review_pack_digest_ref: input.review_pack_digest_ref,
            display_label: input.display_label,
            scope_selectors: input.scope_selectors,
            findings: input.findings,
            publish_sheets: input.publish_sheets,
            resolution_memory: input.resolution_memory,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates stable AI review-assist truth invariants.
    pub fn validate(&self) -> Vec<AiReviewAssistTruthViolation> {
        let mut violations = Vec::new();
        if self.record_kind != AI_REVIEW_ASSIST_TRUTH_RECORD_KIND {
            violations.push(AiReviewAssistTruthViolation::WrongRecordKind);
        }
        if self.schema_version != AI_REVIEW_ASSIST_TRUTH_SCHEMA_VERSION {
            violations.push(AiReviewAssistTruthViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.review_workspace_ref.trim().is_empty()
            || self.review_pack_digest_ref.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiReviewAssistTruthViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_scopes(self, &mut violations);
        validate_findings(self, &mut violations);
        validate_publish_sheets(self, &mut violations);
        validate_resolution_memory(self, &mut violations);
        validate_consumers(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("ai review-assist packet serializes"),
        ) {
            violations.push(AiReviewAssistTruthViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("ai review-assist packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stale_findings = self
            .findings
            .iter()
            .filter(|finding| finding.resolution_state == AiReviewResolutionState::Outdated)
            .count();
        let provider_publish_sheets = self
            .publish_sheets
            .iter()
            .filter(|sheet| sheet.destination_class.is_provider_destination())
            .count();
        let fallback_sheets = self
            .publish_sheets
            .iter()
            .filter(|sheet| {
                matches!(
                    sheet.provider_write_access,
                    ProviderWriteAccessClass::MissingProviderWriteAccess
                        | ProviderWriteAccessClass::ProviderWriteBlockedByPolicy
                )
            })
            .count();

        let mut out = String::new();
        out.push_str("# AI Review Assist And Publish Truth\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Review workspace: `{}`\n",
            self.review_workspace_ref
        ));
        out.push_str(&format!(
            "- Review-pack digest: `{}`\n",
            self.review_pack_digest_ref
        ));
        out.push_str(&format!(
            "- Scope selectors: {}\n",
            self.scope_selectors.len()
        ));
        out.push_str(&format!(
            "- Findings: {} ({} outdated)\n",
            self.findings.len(),
            stale_findings
        ));
        out.push_str(&format!(
            "- Publish sheets: {} ({} provider, {} fallback)\n",
            self.publish_sheets.len(),
            provider_publish_sheets,
            fallback_sheets
        ));
        out.push_str(&format!(
            "- Resolution rows: {}\n",
            self.resolution_memory.len()
        ));
        out.push_str(&format!(
            "- Consumer projections: {}\n",
            self.consumer_projections.len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in AI review-assist export.
#[derive(Debug)]
pub enum AiReviewAssistTruthArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiReviewAssistTruthViolation>),
}

impl fmt::Display for AiReviewAssistTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "ai review-assist export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "ai review-assist export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiReviewAssistTruthArtifactError {}

/// Validation failures emitted by [`AiReviewAssistTruthPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiReviewAssistTruthViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// Scope selector set is incomplete or malformed.
    ScopeSelectorIncomplete,
    /// Scope selector did not distinguish the required analysis targets.
    RequiredScopeClassMissing,
    /// Scope freshness and rerun action disagree.
    ScopeFreshnessRerunMismatch,
    /// Finding row is incomplete.
    FindingIncomplete,
    /// Finding is not bound to a known scope selector.
    FindingScopeMissing,
    /// Finding does not use the packet review-pack digest.
    ReviewPackDigestMismatch,
    /// Finding freshness does not match the selector freshness.
    FindingFreshnessMismatch,
    /// Material diff changes did not downgrade stale findings.
    MaterialDiffChangeNotDowngraded,
    /// Publish sheet row is incomplete.
    PublishSheetIncomplete,
    /// Publish sheet does not preview destination and outbound text.
    PublishPreviewIncomplete,
    /// Publish sheet illegally writes to a provider without provider write access.
    ProviderWriteMissingNotDowngraded,
    /// Low-confidence or outdated finding is allowed to publish to provider.
    UnsafeFindingPublishAllowed,
    /// Resolution memory is incomplete or not linked.
    ResolutionMemoryIncomplete,
    /// Published resolution is missing a publish sheet.
    PublishedResolutionMissingSheet,
    /// Resolution state disagrees with finding state.
    ResolutionStateMismatch,
    /// Consumer surface parity is incomplete.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiReviewAssistTruthViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ScopeSelectorIncomplete => "scope_selector_incomplete",
            Self::RequiredScopeClassMissing => "required_scope_class_missing",
            Self::ScopeFreshnessRerunMismatch => "scope_freshness_rerun_mismatch",
            Self::FindingIncomplete => "finding_incomplete",
            Self::FindingScopeMissing => "finding_scope_missing",
            Self::ReviewPackDigestMismatch => "review_pack_digest_mismatch",
            Self::FindingFreshnessMismatch => "finding_freshness_mismatch",
            Self::MaterialDiffChangeNotDowngraded => "material_diff_change_not_downgraded",
            Self::PublishSheetIncomplete => "publish_sheet_incomplete",
            Self::PublishPreviewIncomplete => "publish_preview_incomplete",
            Self::ProviderWriteMissingNotDowngraded => "provider_write_missing_not_downgraded",
            Self::UnsafeFindingPublishAllowed => "unsafe_finding_publish_allowed",
            Self::ResolutionMemoryIncomplete => "resolution_memory_incomplete",
            Self::PublishedResolutionMissingSheet => "published_resolution_missing_sheet",
            Self::ResolutionStateMismatch => "resolution_state_mismatch",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable AI review-assist support export.
pub fn current_stable_ai_review_assist_truth_export(
) -> Result<AiReviewAssistTruthPacket, AiReviewAssistTruthArtifactError> {
    let packet: AiReviewAssistTruthPacket = serde_json::from_str(include_str!(concat!(
        "../../../../",
        "artifacts/ai/m4/ai-review-assist-and-publish-truth/support_export.json"
    )))
    .map_err(AiReviewAssistTruthArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiReviewAssistTruthArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &AiReviewAssistTruthPacket,
    violations: &mut Vec<AiReviewAssistTruthViolation>,
) {
    let refs = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for required in [
        AI_REVIEW_ASSIST_TRUTH_AI_DOC_REF,
        AI_REVIEW_ASSIST_REVIEW_PACK_CONTRACT_REF,
        AI_REVIEW_ASSIST_TRUTH_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(AiReviewAssistTruthViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_scopes(
    packet: &AiReviewAssistTruthPacket,
    violations: &mut Vec<AiReviewAssistTruthViolation>,
) {
    if packet.scope_selectors.is_empty() {
        violations.push(AiReviewAssistTruthViolation::ScopeSelectorIncomplete);
        return;
    }
    let mut classes = BTreeSet::new();
    for scope in &packet.scope_selectors {
        classes.insert(scope.scope_class);
        if scope.selector_id.trim().is_empty()
            || scope.base_identity_ref.trim().is_empty()
            || scope.head_identity_ref.trim().is_empty()
            || scope.review_pack_digest_ref.trim().is_empty()
            || scope.review_pack_digest_ref != packet.review_pack_digest_ref
            || !lineage_complete(&scope.lineage)
        {
            violations.push(AiReviewAssistTruthViolation::ScopeSelectorIncomplete);
        }
        if scope.scope_class == ReviewScopeClass::HostedReviewObject
            && scope
                .hosted_review_object_ref
                .as_deref()
                .unwrap_or_default()
                .trim()
                .is_empty()
        {
            violations.push(AiReviewAssistTruthViolation::ScopeSelectorIncomplete);
        }
        if scope.freshness_class == ReviewScopeFreshnessClass::Current
            && scope.rerun_action != ReviewScopeRerunActionClass::NotNeeded
        {
            violations.push(AiReviewAssistTruthViolation::ScopeFreshnessRerunMismatch);
        }
        if scope.freshness_class.requires_non_fresh_resolution()
            && (scope.rerun_action == ReviewScopeRerunActionClass::NotNeeded
                || scope.material_diff_change_ref.is_none())
        {
            violations.push(AiReviewAssistTruthViolation::ScopeFreshnessRerunMismatch);
        }
    }
    for required in [
        ReviewScopeClass::SelectedDiff,
        ReviewScopeClass::UncommittedChanges,
        ReviewScopeClass::HostedReviewObject,
    ] {
        if !classes.contains(&required) {
            violations.push(AiReviewAssistTruthViolation::RequiredScopeClassMissing);
        }
    }
}

fn validate_findings(
    packet: &AiReviewAssistTruthPacket,
    violations: &mut Vec<AiReviewAssistTruthViolation>,
) {
    let scopes = packet
        .scope_selectors
        .iter()
        .map(|scope| (scope.selector_id.as_str(), scope))
        .collect::<BTreeMap<_, _>>();
    if packet.findings.is_empty() {
        violations.push(AiReviewAssistTruthViolation::FindingIncomplete);
        return;
    }
    for finding in &packet.findings {
        if finding.finding_id.trim().is_empty()
            || finding.title.trim().is_empty()
            || finding.instruction_check_refs.is_empty()
            || finding.affected_hunks.is_empty()
            || !lineage_complete(&finding.lineage)
        {
            violations.push(AiReviewAssistTruthViolation::FindingIncomplete);
        }
        if finding.review_pack_digest_ref != packet.review_pack_digest_ref {
            violations.push(AiReviewAssistTruthViolation::ReviewPackDigestMismatch);
        }
        let Some(scope) = scopes.get(finding.scope_selector_id.as_str()) else {
            violations.push(AiReviewAssistTruthViolation::FindingScopeMissing);
            continue;
        };
        if finding.scope_freshness_class != scope.freshness_class {
            violations.push(AiReviewAssistTruthViolation::FindingFreshnessMismatch);
        }
        if scope.freshness_class.requires_non_fresh_resolution()
            && !finding.resolution_state.admits_stale_scope()
        {
            violations.push(AiReviewAssistTruthViolation::MaterialDiffChangeNotDowngraded);
        }
        if finding.confidence_class.blocks_provider_publish() && finding.publish_sheet_id.is_some()
        {
            violations.push(AiReviewAssistTruthViolation::UnsafeFindingPublishAllowed);
        }
        if finding.affected_hunks.iter().any(|hunk| {
            hunk.file_ref.trim().is_empty()
                || hunk.hunk_ref.trim().is_empty()
                || hunk.diff_fingerprint_ref.trim().is_empty()
        }) {
            violations.push(AiReviewAssistTruthViolation::FindingIncomplete);
        }
    }
}

fn validate_publish_sheets(
    packet: &AiReviewAssistTruthPacket,
    violations: &mut Vec<AiReviewAssistTruthViolation>,
) {
    let findings = packet
        .findings
        .iter()
        .map(|finding| (finding.finding_id.as_str(), finding))
        .collect::<BTreeMap<_, _>>();
    for sheet in &packet.publish_sheets {
        if sheet.publish_sheet_id.trim().is_empty()
            || sheet.finding_id.trim().is_empty()
            || sheet.provider_ref.trim().is_empty()
            || !lineage_complete(&sheet.lineage)
        {
            violations.push(AiReviewAssistTruthViolation::PublishSheetIncomplete);
        }
        if sheet.destination_ref.trim().is_empty() || sheet.outbound_text_preview.trim().is_empty()
        {
            violations.push(AiReviewAssistTruthViolation::PublishPreviewIncomplete);
        }
        let Some(finding) = findings.get(sheet.finding_id.as_str()) else {
            violations.push(AiReviewAssistTruthViolation::PublishSheetIncomplete);
            continue;
        };
        if finding.resolution_state == AiReviewResolutionState::Outdated
            && sheet.action_class.is_provider_write()
        {
            violations.push(AiReviewAssistTruthViolation::UnsafeFindingPublishAllowed);
        }
        if finding.confidence_class.blocks_provider_publish()
            && sheet.action_class.is_provider_write()
        {
            violations.push(AiReviewAssistTruthViolation::UnsafeFindingPublishAllowed);
        }
        if sheet.destination_class.is_provider_destination()
            && sheet.action_class.is_provider_write()
            && sheet.provider_write_access != ProviderWriteAccessClass::ProviderWriteAvailable
        {
            violations.push(AiReviewAssistTruthViolation::ProviderWriteMissingNotDowngraded);
        }
        if matches!(
            sheet.provider_write_access,
            ProviderWriteAccessClass::MissingProviderWriteAccess
                | ProviderWriteAccessClass::ProviderWriteBlockedByPolicy
        ) && (!sheet.action_class.is_missing_write_downgrade()
            || !sheet
                .fallback_actions
                .iter()
                .any(|action| matches!(action, PublishActionClass::CopyOnly))
            || !sheet
                .fallback_actions
                .iter()
                .any(|action| matches!(action, PublishActionClass::ExportLocalPacket))
            || !sheet
                .fallback_actions
                .iter()
                .any(|action| matches!(action, PublishActionClass::KeepLocal)))
        {
            violations.push(AiReviewAssistTruthViolation::ProviderWriteMissingNotDowngraded);
        }
    }
}

fn validate_resolution_memory(
    packet: &AiReviewAssistTruthPacket,
    violations: &mut Vec<AiReviewAssistTruthViolation>,
) {
    let findings = packet
        .findings
        .iter()
        .map(|finding| (finding.finding_id.as_str(), finding))
        .collect::<BTreeMap<_, _>>();
    let publish_sheets = packet
        .publish_sheets
        .iter()
        .map(|sheet| sheet.publish_sheet_id.as_str())
        .collect::<BTreeSet<_>>();
    if packet.resolution_memory.len() != packet.findings.len() {
        violations.push(AiReviewAssistTruthViolation::ResolutionMemoryIncomplete);
    }
    let mut seen_findings = BTreeSet::new();
    for row in &packet.resolution_memory {
        if row.resolution_id.trim().is_empty()
            || row.finding_id.trim().is_empty()
            || row.actor_ref.trim().is_empty()
            || row.source_ref.trim().is_empty()
            || row.timestamp.trim().is_empty()
            || !lineage_complete(&row.lineage)
        {
            violations.push(AiReviewAssistTruthViolation::ResolutionMemoryIncomplete);
        }
        if !row
            .reopen_lineage_refs
            .iter()
            .all(|entry| !entry.trim().is_empty())
        {
            violations.push(AiReviewAssistTruthViolation::ResolutionMemoryIncomplete);
        }
        let Some(finding) = findings.get(row.finding_id.as_str()) else {
            violations.push(AiReviewAssistTruthViolation::ResolutionMemoryIncomplete);
            continue;
        };
        seen_findings.insert(row.finding_id.as_str());
        if finding.resolution_state != row.state {
            violations.push(AiReviewAssistTruthViolation::ResolutionStateMismatch);
        }
        if row.state == AiReviewResolutionState::Published {
            match row.publish_sheet_id.as_deref() {
                Some(sheet_id) if publish_sheets.contains(sheet_id) => {}
                _ => violations.push(AiReviewAssistTruthViolation::PublishedResolutionMissingSheet),
            }
        }
    }
    if seen_findings.len() != packet.findings.len() {
        violations.push(AiReviewAssistTruthViolation::ResolutionMemoryIncomplete);
    }
}

fn validate_consumers(
    packet: &AiReviewAssistTruthPacket,
    violations: &mut Vec<AiReviewAssistTruthViolation>,
) {
    let mut present = BTreeSet::new();
    for projection in &packet.consumer_projections {
        present.insert(projection.surface);
        if !projection.preserves_scope_identity
            || !projection.preserves_instruction_check_source
            || !projection.preserves_publish_destination_truth
            || !projection.preserves_resolution_memory
            || !projection.preserves_local_copy_export_fallback
            || projection.export_ref.trim().is_empty()
        {
            violations.push(AiReviewAssistTruthViolation::ConsumerProjectionIncomplete);
        }
    }
    for surface in AiReviewConsumerSurface::required_surfaces() {
        if !present.contains(&surface) {
            violations.push(AiReviewAssistTruthViolation::ConsumerProjectionIncomplete);
        }
    }
}

fn lineage_complete(lineage: &AiReviewObjectLineage) -> bool {
    !lineage.object_id.trim().is_empty()
        && !lineage.evidence_packet_refs.is_empty()
        && !lineage.export_lineage_refs.is_empty()
        && lineage.export_safe
        && lineage
            .evidence_packet_refs
            .iter()
            .all(|entry| !entry.trim().is_empty())
        && lineage
            .export_lineage_refs
            .iter()
            .all(|entry| !entry.trim().is_empty())
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_boundary_material(text),
        serde_json::Value::Array(values) => {
            values.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(values) => values
            .values()
            .any(json_contains_forbidden_boundary_material),
        _ => false,
    }
}

fn contains_forbidden_boundary_material(text: &str) -> bool {
    let lowered = text.to_ascii_lowercase();
    lowered.contains("-----begin ")
        || lowered.contains("oauth_token")
        || lowered.contains("api_key=")
        || lowered.contains("bearer ")
        || lowered.contains("provider_payload:")
        || lowered.contains("raw_diff:")
        || lowered.contains("raw_prompt:")
}

#[cfg(test)]
mod tests;
