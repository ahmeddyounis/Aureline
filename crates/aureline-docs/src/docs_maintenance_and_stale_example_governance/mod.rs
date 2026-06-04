//! Stable docs-maintenance and stale-example governance packet.
//!
//! This module turns docs render configuration, suggestions, validation
//! results, stale-example findings, suppression records, and maintenance
//! packets into one stable contract for Help/About, onboarding, migration,
//! release-note, docs-feedback, and support/community handoff surfaces. The
//! packet is metadata-only: rendered previews are never canonical source, raw
//! document bodies are omitted, and publish/browser handoff state is carried as
//! explicit fields instead of inferred from the surface that displays it.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::{
    CitationConfidenceClass, DocsAudienceScope, DocsExampleValidationMode, DocsFindingClass,
    DocsFindingDetectionState, DocsFindingSuppression, DocsFindingSuppressionState,
    DocsFreshnessClass, DocsPreviewMode, DocsPublishBoundaryState, DocsPublishScope,
    DocsSuggestionTrigger, VersionMatchState,
};

/// Stable record-kind tag for [`DocsMaintenanceGovernancePacket`].
pub const DOCS_MAINTENANCE_GOVERNANCE_RECORD_KIND: &str =
    "docs_maintenance_and_stale_example_governance_packet";

/// Stable record-kind tag for [`DocsMaintenanceGovernanceSupportExport`].
pub const DOCS_MAINTENANCE_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "docs_maintenance_and_stale_example_governance_support_export";

/// Integer schema version for the docs-maintenance governance packet.
pub const DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the contract doc.
pub const DOCS_MAINTENANCE_GOVERNANCE_DOC_REF: &str =
    "docs/m4/docs-maintenance-and-stale-example-governance.md";

/// Repo-relative path of the human-readable artifact narrative.
pub const DOCS_MAINTENANCE_GOVERNANCE_ARTIFACT_DOC_REF: &str =
    "artifacts/docs/m4/docs-maintenance-and-stale-example-governance.md";

/// Repo-relative path of the JSON schema.
pub const DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/docs/docs-maintenance-and-stale-example-governance.schema.json";

/// Repo-relative fixture corpus directory.
pub const DOCS_MAINTENANCE_GOVERNANCE_FIXTURE_DIR: &str =
    "fixtures/docs/m4/docs-maintenance-and-stale-example-governance";

/// Repo-relative path of the checked-in governance packet.
pub const DOCS_MAINTENANCE_GOVERNANCE_ARTIFACT_REF: &str =
    "artifacts/docs/m4/docs-maintenance-and-stale-example-governance.json";

/// Closed promotion state for the governance packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsMaintenanceGovernancePromotionState {
    /// Packet is complete enough for stable docs/help/onboarding claims.
    Stable,
    /// Packet can be reviewed but cannot back a widened stable claim.
    NeedsReview,
    /// Packet blocks stable promotion.
    BlocksStable,
}

impl DocsMaintenanceGovernancePromotionState {
    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NeedsReview => "needs_review",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Consumer surfaces that must reuse the same maintenance packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsMaintenanceGovernanceSurface {
    /// Help and About proof cards.
    HelpAbout,
    /// Onboarding cards and guided first-run surfaces.
    OnboardingCards,
    /// Migration guidance and compatibility notes.
    MigrationGuidance,
    /// Release notes and changelog publication rows.
    ReleaseNotes,
    /// Docs feedback export.
    DocsFeedbackExport,
    /// Support and community handoff packets.
    SupportCommunityHandoff,
    /// Docs pack detail sheets.
    DocsPack,
    /// CLI or headless validation export.
    CliHeadless,
}

impl DocsMaintenanceGovernanceSurface {
    /// Every surface required before stable docs/public-truth widening.
    pub const REQUIRED: [Self; 8] = [
        Self::HelpAbout,
        Self::OnboardingCards,
        Self::MigrationGuidance,
        Self::ReleaseNotes,
        Self::DocsFeedbackExport,
        Self::SupportCommunityHandoff,
        Self::DocsPack,
        Self::CliHeadless,
    ];

    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpAbout => "help_about",
            Self::OnboardingCards => "onboarding_cards",
            Self::MigrationGuidance => "migration_guidance",
            Self::ReleaseNotes => "release_notes",
            Self::DocsFeedbackExport => "docs_feedback_export",
            Self::SupportCommunityHandoff => "support_community_handoff",
            Self::DocsPack => "docs_pack",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// Docs artifact classes that require maintenance packets on stable rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsMaintenanceArtifactClass {
    /// Project README.
    Readme,
    /// Changelog or release log.
    Changelog,
    /// Onboarding document or card pack.
    Onboarding,
    /// Module-level or crate-level docs.
    ModuleDocs,
    /// Support article or runbook.
    SupportArticle,
}

impl DocsMaintenanceArtifactClass {
    /// Every artifact class required for stable docs-maintenance packets.
    pub const REQUIRED: [Self; 5] = [
        Self::Readme,
        Self::Changelog,
        Self::Onboarding,
        Self::ModuleDocs,
        Self::SupportArticle,
    ];

    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Readme => "readme",
            Self::Changelog => "changelog",
            Self::Onboarding => "onboarding",
            Self::ModuleDocs => "module_docs",
            Self::SupportArticle => "support_article",
        }
    }
}

/// Active-content posture for Markdown rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsActiveContentState {
    /// Scripts, iframes, event handlers, and active embeds were blocked.
    Blocked,
    /// Only sanitized inert content rendered.
    SanitizedInertOnly,
    /// Source mode rendered no active content.
    NotApplicable,
}

impl DocsActiveContentState {
    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocked => "blocked",
            Self::SanitizedInertOnly => "sanitized_inert_only",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Security profile used by a docs render config.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsRenderSecurityProfile {
    /// CommonMark preview with source-first, sanitized rendering.
    SourceFirstSafeMarkdown,
    /// Mirrored/offline preview with active content blocked.
    MirrorOfflineSafe,
    /// Browser handoff is required before hosted publish inspection.
    BrowserHandoffRequired,
}

impl DocsRenderSecurityProfile {
    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceFirstSafeMarkdown => "source_first_safe_markdown",
            Self::MirrorOfflineSafe => "mirror_offline_safe",
            Self::BrowserHandoffRequired => "browser_handoff_required",
        }
    }
}

/// Mirror/offline and browser-handoff posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsMirrorBrowserHandoffPosture {
    /// Local source is available and no browser handoff applies.
    LocalOnly,
    /// Signed mirror or offline packet is used.
    MirroredOffline,
    /// External browser handoff is available through an explicit packet.
    BrowserHandoffAvailable,
    /// Browser handoff would apply but is blocked by policy.
    BrowserHandoffBlocked,
}

impl DocsMirrorBrowserHandoffPosture {
    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::MirroredOffline => "mirrored_offline",
            Self::BrowserHandoffAvailable => "browser_handoff_available",
            Self::BrowserHandoffBlocked => "browser_handoff_blocked",
        }
    }

    fn requires_packet_ref(self) -> bool {
        matches!(self, Self::BrowserHandoffAvailable)
    }
}

/// Share and export posture for governance records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsShareExportPosture {
    /// Export carries references only and omits raw docs bodies.
    ReferenceOnlyExportable,
    /// Export is scoped to a review packet.
    ReviewPacketExportable,
    /// Public export is blocked until evidence or scope is supplied.
    PublicPublishBlocked,
}

impl DocsShareExportPosture {
    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReferenceOnlyExportable => "reference_only_exportable",
            Self::ReviewPacketExportable => "review_packet_exportable",
            Self::PublicPublishBlocked => "public_publish_blocked",
        }
    }
}

/// Validation outcome for a documented artifact or snippet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsValidationOutcome {
    /// Validation passed in the declared mode.
    Passed,
    /// Validation failed and diagnostics are available.
    Failed,
    /// Prior validation exists but is stale.
    Stale,
    /// Validation mode is unsupported for this target.
    Unsupported,
    /// Validation is blocked by policy or missing evidence.
    Blocked,
}

impl DocsValidationOutcome {
    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Stale => "stale",
            Self::Unsupported => "unsupported",
            Self::Blocked => "blocked",
        }
    }
}

/// Finding kind emitted by packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsMaintenanceGovernanceFindingKind {
    /// Packet identity or version fields are incomplete.
    PacketIdentityIncomplete,
    /// Render config is missing source/version/security/boundary truth.
    RenderConfigIncomplete,
    /// Rendered preview is allowed to masquerade as canonical source.
    RenderedPreviewCanonicalized,
    /// Suggestion lacks evidence or a review diff where required.
    SuggestionNotEvidenceBacked,
    /// Validation result collapses rendered/syntax/executed states or lacks freshness.
    ValidationResultIncomplete,
    /// Stale-example finding or suppression lacks governed attribution.
    StaleFindingGovernanceIncomplete,
    /// Required artifact class lacks a maintenance packet.
    MaintenancePacketCoverageMissing,
    /// Maintenance packet lacks source/version/freshness/publish-boundary state.
    MaintenancePacketIncomplete,
    /// Consumer projection is missing or dropped shared vocabulary.
    ConsumerProjectionDroppedVocabulary,
    /// Raw document bodies, raw URLs, or rendered HTML were exported.
    RawBoundaryMaterialExported,
}

impl DocsMaintenanceGovernanceFindingKind {
    /// Returns the stable token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacketIdentityIncomplete => "packet_identity_incomplete",
            Self::RenderConfigIncomplete => "render_config_incomplete",
            Self::RenderedPreviewCanonicalized => "rendered_preview_canonicalized",
            Self::SuggestionNotEvidenceBacked => "suggestion_not_evidence_backed",
            Self::ValidationResultIncomplete => "validation_result_incomplete",
            Self::StaleFindingGovernanceIncomplete => "stale_finding_governance_incomplete",
            Self::MaintenancePacketCoverageMissing => "maintenance_packet_coverage_missing",
            Self::MaintenancePacketIncomplete => "maintenance_packet_incomplete",
            Self::ConsumerProjectionDroppedVocabulary => "consumer_projection_dropped_vocabulary",
            Self::RawBoundaryMaterialExported => "raw_boundary_material_exported",
        }
    }
}

/// Docs-render configuration shared by preview and browser handoff surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsRenderConfig {
    /// Stable render-config id.
    pub render_config_id: String,
    /// Preview mode.
    pub mode: DocsPreviewMode,
    /// CommonMark baseline ref, for example `commonmark:0.31`.
    pub commonmark_baseline_ref: String,
    /// Declared extension baseline.
    pub extension_baseline: Vec<String>,
    /// Active-content posture.
    pub active_content_state: DocsActiveContentState,
    /// Docs source ref.
    pub docs_source_ref: String,
    /// Docs version ref.
    pub docs_version_ref: String,
    /// Security profile.
    pub security_profile: DocsRenderSecurityProfile,
    /// Mirror/offline and browser-handoff posture.
    pub mirror_browser_handoff_posture: DocsMirrorBrowserHandoffPosture,
    /// Browser handoff packet when external opening is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// True when rendered output is explicitly labeled non-canonical.
    pub rendered_preview_not_canonical: bool,
    /// Evidence refs backing the render config.
    pub evidence_refs: Vec<String>,
}

impl DocsRenderConfig {
    fn is_well_formed(&self) -> bool {
        !self.render_config_id.trim().is_empty()
            && !self.commonmark_baseline_ref.trim().is_empty()
            && !self.docs_source_ref.trim().is_empty()
            && !self.docs_version_ref.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && (!self.mirror_browser_handoff_posture.requires_packet_ref()
                || self
                    .browser_handoff_packet_ref
                    .as_deref()
                    .is_some_and(|value| !value.trim().is_empty()))
    }
}

/// Evidence-backed docs suggestion object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSuggestionObject {
    /// Stable suggestion id.
    pub suggestion_id: String,
    /// Target artifact ref.
    pub target_artifact_ref: String,
    /// Target section ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_section_ref: Option<String>,
    /// Trigger class.
    pub trigger: DocsSuggestionTrigger,
    /// Evidence refs backing the suggestion.
    pub evidence_refs: Vec<String>,
    /// Review diff ref.
    pub diff_ref: String,
    /// Confidence class.
    pub confidence_class: CitationConfidenceClass,
    /// Freshness class.
    pub freshness_class: DocsFreshnessClass,
    /// Audience scope.
    pub audience_scope: DocsAudienceScope,
    /// Share/export posture.
    pub share_export_posture: DocsShareExportPosture,
    /// Publish-boundary state.
    pub publish_boundary_state: DocsPublishBoundaryState,
    /// Publish-boundary note.
    pub publish_boundary_note: String,
}

impl DocsSuggestionObject {
    fn is_well_formed(&self) -> bool {
        !self.suggestion_id.trim().is_empty()
            && !self.target_artifact_ref.trim().is_empty()
            && !self.diff_ref.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && !self.publish_boundary_note.trim().is_empty()
    }
}

/// Docs-validation result for an artifact or snippet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsValidationResult {
    /// Stable validation result id.
    pub validation_result_id: String,
    /// Artifact or snippet id.
    pub artifact_or_snippet_ref: String,
    /// Validation mode.
    pub validation_mode: DocsExampleValidationMode,
    /// Validation outcome.
    pub outcome: DocsValidationOutcome,
    /// Toolchain or context identity.
    pub toolchain_context_ref: String,
    /// Output snapshot ref.
    pub output_snapshot_ref: String,
    /// Diagnostics refs.
    pub diagnostics_refs: Vec<String>,
    /// Freshness class.
    pub freshness_class: DocsFreshnessClass,
    /// Last checked time.
    pub last_checked_at: String,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
}

impl DocsValidationResult {
    fn is_well_formed(&self) -> bool {
        !self.validation_result_id.trim().is_empty()
            && !self.artifact_or_snippet_ref.trim().is_empty()
            && !self.toolchain_context_ref.trim().is_empty()
            && !self.output_snapshot_ref.trim().is_empty()
            && !self.last_checked_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
    }
}

/// Stale-example finding with governed suppression support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleExampleGovernanceFinding {
    /// Stable finding id.
    pub finding_id: String,
    /// Finding class.
    pub finding_class: DocsFindingClass,
    /// Detection state.
    pub detection_state: DocsFindingDetectionState,
    /// Validation mode used to classify the finding.
    pub validation_mode: DocsExampleValidationMode,
    /// Environment scope ref.
    pub environment_scope_ref: String,
    /// Version scope ref.
    pub version_scope_ref: String,
    /// Last checked time.
    pub last_checked_at: String,
    /// Freshness class.
    pub freshness_class: DocsFreshnessClass,
    /// Suppression state.
    pub suppression_state: DocsFindingSuppressionState,
    /// Suppression detail when suppressed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppression: Option<DocsFindingSuppression>,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
    /// Publish-boundary notes.
    pub publish_boundary_notes: Vec<String>,
}

impl StaleExampleGovernanceFinding {
    fn is_well_formed(&self) -> bool {
        !self.finding_id.trim().is_empty()
            && !self.environment_scope_ref.trim().is_empty()
            && !self.version_scope_ref.trim().is_empty()
            && !self.last_checked_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && !self.publish_boundary_notes.is_empty()
            && (self.suppression_state != DocsFindingSuppressionState::SuppressedUntilReviewed
                || self.suppression.as_ref().is_some_and(|suppression| {
                    !suppression.actor_ref.trim().is_empty()
                        && !suppression.reason.trim().is_empty()
                        && !suppression.expiry_at.trim().is_empty()
                        && !suppression.evidence_refs.is_empty()
                }))
    }
}

/// Maintenance packet required for a stable docs artifact class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenancePacket {
    /// Stable maintenance packet id.
    pub maintenance_packet_id: String,
    /// Artifact class.
    pub artifact_class: DocsMaintenanceArtifactClass,
    /// Artifact ref.
    pub artifact_ref: String,
    /// Source ref.
    pub docs_source_ref: String,
    /// Version ref.
    pub docs_version_ref: String,
    /// Audience scope.
    pub audience_scope: DocsAudienceScope,
    /// Branch/release/channel scope.
    pub publish_scope: DocsPublishScope,
    /// Validation result refs.
    pub validation_result_refs: Vec<String>,
    /// Pending suggestion refs.
    pub pending_suggestion_refs: Vec<String>,
    /// Stale finding refs.
    pub stale_finding_refs: Vec<String>,
    /// Validation freshness.
    pub validation_freshness: DocsFreshnessClass,
    /// Version-match state.
    pub version_match_state: VersionMatchState,
    /// Publish-boundary state.
    pub publish_boundary_state: DocsPublishBoundaryState,
    /// Publish-boundary notes.
    pub publish_boundary_notes: Vec<String>,
    /// Share/export posture.
    pub share_export_posture: DocsShareExportPosture,
    /// Surface refs consuming this packet.
    pub surface_refs: Vec<String>,
}

impl DocsMaintenancePacket {
    fn is_well_formed(&self) -> bool {
        !self.maintenance_packet_id.trim().is_empty()
            && !self.artifact_ref.trim().is_empty()
            && !self.docs_source_ref.trim().is_empty()
            && !self.docs_version_ref.trim().is_empty()
            && !self.validation_result_refs.is_empty()
            && !self.publish_boundary_notes.is_empty()
            && !self.surface_refs.is_empty()
            && (!self.publish_boundary_state.requires_scope() || self.publish_scope.is_scoped())
    }
}

/// Surface projection proving consumers reuse the same packet vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenanceGovernanceProjection {
    /// Consumer surface.
    pub surface: DocsMaintenanceGovernanceSurface,
    /// Source governance packet id.
    pub packet_id_ref: String,
    /// Maintenance packet refs shown by the surface.
    pub maintenance_packet_refs: Vec<String>,
    /// True when source/version refs are preserved verbatim.
    pub preserves_source_version_refs: bool,
    /// True when validation freshness is preserved verbatim.
    pub preserves_validation_freshness: bool,
    /// True when stale findings and suppressions remain visible/exportable.
    pub preserves_stale_findings_and_suppressions: bool,
    /// True when publish-boundary and browser handoff posture is preserved.
    pub preserves_publish_boundary_and_handoff: bool,
    /// True when destination classes do not collapse to generic success.
    pub preserves_destination_vocabulary: bool,
}

impl DocsMaintenanceGovernanceProjection {
    fn is_well_formed(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && !self.maintenance_packet_refs.is_empty()
            && self.preserves_source_version_refs
            && self.preserves_validation_freshness
            && self.preserves_stale_findings_and_suppressions
            && self.preserves_publish_boundary_and_handoff
            && self.preserves_destination_vocabulary
    }
}

/// Validation finding emitted by a docs-maintenance governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenanceGovernanceFinding {
    /// Finding kind.
    pub finding_kind: DocsMaintenanceGovernanceFindingKind,
    /// Affected object ref.
    pub object_ref: String,
    /// Reviewable summary.
    pub summary: String,
}

impl DocsMaintenanceGovernanceFinding {
    fn new(
        finding_kind: DocsMaintenanceGovernanceFindingKind,
        object_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            object_ref: object_ref.into(),
            summary: summary.into(),
        }
    }
}

/// Input used to materialize a governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenanceGovernancePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Deterministic generation time.
    pub generated_at: String,
    /// Source branch ref.
    pub source_branch_ref: String,
    /// Release ref.
    pub release_ref: String,
    /// Channel ref.
    pub channel_ref: String,
    /// Render configs.
    pub render_configs: Vec<DocsRenderConfig>,
    /// Suggestion objects.
    pub suggestions: Vec<DocsSuggestionObject>,
    /// Validation results.
    pub validation_results: Vec<DocsValidationResult>,
    /// Stale-example findings.
    pub stale_example_findings: Vec<StaleExampleGovernanceFinding>,
    /// Maintenance packets.
    pub maintenance_packets: Vec<DocsMaintenancePacket>,
    /// Consumer projections.
    pub consumer_projections: Vec<DocsMaintenanceGovernanceProjection>,
    /// Omitted raw material classes.
    pub omitted_material_classes: Vec<String>,
    /// Whether raw document bodies, raw URLs, or rendered HTML were exported.
    pub raw_boundary_material_exported: bool,
}

/// Stable governance packet for docs maintenance and stale examples.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenanceGovernancePacket {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Deterministic generation time.
    pub generated_at: String,
    /// Source branch ref.
    pub source_branch_ref: String,
    /// Release ref.
    pub release_ref: String,
    /// Channel ref.
    pub channel_ref: String,
    /// Render configs.
    pub render_configs: Vec<DocsRenderConfig>,
    /// Suggestion objects.
    pub suggestions: Vec<DocsSuggestionObject>,
    /// Validation results.
    pub validation_results: Vec<DocsValidationResult>,
    /// Stale-example findings.
    pub stale_example_findings: Vec<StaleExampleGovernanceFinding>,
    /// Maintenance packets.
    pub maintenance_packets: Vec<DocsMaintenancePacket>,
    /// Consumer projections.
    pub consumer_projections: Vec<DocsMaintenanceGovernanceProjection>,
    /// Omitted raw material classes.
    pub omitted_material_classes: Vec<String>,
    /// Whether raw document bodies, raw URLs, or rendered HTML were exported.
    pub raw_boundary_material_exported: bool,
    /// Promotion state derived from validation findings.
    pub promotion_state: DocsMaintenanceGovernancePromotionState,
    /// Validation findings.
    pub validation_findings: Vec<DocsMaintenanceGovernanceFinding>,
}

impl DocsMaintenanceGovernancePacket {
    /// Materializes and validates a governance packet from input.
    pub fn materialize(input: DocsMaintenanceGovernancePacketInput) -> Self {
        let mut packet = Self {
            record_kind: DOCS_MAINTENANCE_GOVERNANCE_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            source_branch_ref: input.source_branch_ref,
            release_ref: input.release_ref,
            channel_ref: input.channel_ref,
            render_configs: input.render_configs,
            suggestions: input.suggestions,
            validation_results: input.validation_results,
            stale_example_findings: input.stale_example_findings,
            maintenance_packets: input.maintenance_packets,
            consumer_projections: input.consumer_projections,
            omitted_material_classes: input.omitted_material_classes,
            raw_boundary_material_exported: input.raw_boundary_material_exported,
            promotion_state: DocsMaintenanceGovernancePromotionState::Stable,
            validation_findings: Vec::new(),
        };
        packet.validation_findings = packet.validate();
        packet.promotion_state = if packet.validation_findings.is_empty() {
            DocsMaintenanceGovernancePromotionState::Stable
        } else if packet.validation_findings.iter().any(|finding| {
            matches!(
                finding.finding_kind,
                DocsMaintenanceGovernanceFindingKind::RawBoundaryMaterialExported
                    | DocsMaintenanceGovernanceFindingKind::RenderedPreviewCanonicalized
                    | DocsMaintenanceGovernanceFindingKind::MaintenancePacketCoverageMissing
                    | DocsMaintenanceGovernanceFindingKind::ConsumerProjectionDroppedVocabulary
            )
        }) {
            DocsMaintenanceGovernancePromotionState::BlocksStable
        } else {
            DocsMaintenanceGovernancePromotionState::NeedsReview
        };
        packet
    }

    /// Validates the packet and returns all findings.
    pub fn validate(&self) -> Vec<DocsMaintenanceGovernanceFinding> {
        let mut findings = Vec::new();
        if self.record_kind != DOCS_MAINTENANCE_GOVERNANCE_RECORD_KIND
            || self.schema_version != DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_VERSION
            || self.packet_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.source_branch_ref.trim().is_empty()
            || self.release_ref.trim().is_empty()
            || self.channel_ref.trim().is_empty()
        {
            findings.push(DocsMaintenanceGovernanceFinding::new(
                DocsMaintenanceGovernanceFindingKind::PacketIdentityIncomplete,
                &self.packet_id,
                "packet identity, refs, record kind, or schema version are incomplete",
            ));
        }
        if self.raw_boundary_material_exported || self.omitted_material_classes.is_empty() {
            findings.push(DocsMaintenanceGovernanceFinding::new(
                DocsMaintenanceGovernanceFindingKind::RawBoundaryMaterialExported,
                &self.packet_id,
                "governance packet must omit raw document bodies, rendered HTML, and raw URLs",
            ));
        }
        for config in &self.render_configs {
            if !config.is_well_formed() {
                findings.push(DocsMaintenanceGovernanceFinding::new(
                    DocsMaintenanceGovernanceFindingKind::RenderConfigIncomplete,
                    &config.render_config_id,
                    "render config must carry mode, CommonMark baseline, extensions, source/version, security, handoff posture, and evidence",
                ));
            }
            if config.mode.renders_preview() && !config.rendered_preview_not_canonical {
                findings.push(DocsMaintenanceGovernanceFinding::new(
                    DocsMaintenanceGovernanceFindingKind::RenderedPreviewCanonicalized,
                    &config.render_config_id,
                    "rendered previews must be explicitly labeled as non-canonical",
                ));
            }
        }
        for suggestion in &self.suggestions {
            if !suggestion.is_well_formed() {
                findings.push(DocsMaintenanceGovernanceFinding::new(
                    DocsMaintenanceGovernanceFindingKind::SuggestionNotEvidenceBacked,
                    &suggestion.suggestion_id,
                    "suggestions must be evidence-backed, diff-based, audience-scoped, and publish-boundary labeled",
                ));
            }
        }
        for result in &self.validation_results {
            if !result.is_well_formed() {
                findings.push(DocsMaintenanceGovernanceFinding::new(
                    DocsMaintenanceGovernanceFindingKind::ValidationResultIncomplete,
                    &result.validation_result_id,
                    "validation results must preserve artifact/snippet, mode, toolchain, snapshot, diagnostics, freshness, and evidence",
                ));
            }
        }
        for finding in &self.stale_example_findings {
            if !finding.is_well_formed() {
                findings.push(DocsMaintenanceGovernanceFinding::new(
                    DocsMaintenanceGovernanceFindingKind::StaleFindingGovernanceIncomplete,
                    &finding.finding_id,
                    "stale-example findings and suppressions must carry environment/version, freshness, evidence, actor, reason, and expiry",
                ));
            }
        }
        for packet in &self.maintenance_packets {
            if !packet.is_well_formed() {
                findings.push(DocsMaintenanceGovernanceFinding::new(
                    DocsMaintenanceGovernanceFindingKind::MaintenancePacketIncomplete,
                    &packet.maintenance_packet_id,
                    "maintenance packets must carry source/version, validation freshness, pending refs, publish-boundary state, and export posture",
                ));
            }
        }
        self.validate_cross_references(&mut findings);
        findings
    }

    fn validate_cross_references(&self, findings: &mut Vec<DocsMaintenanceGovernanceFinding>) {
        let validation_ids: BTreeSet<&str> = self
            .validation_results
            .iter()
            .map(|result| result.validation_result_id.as_str())
            .collect();
        let suggestion_ids: BTreeSet<&str> = self
            .suggestions
            .iter()
            .map(|suggestion| suggestion.suggestion_id.as_str())
            .collect();
        let finding_ids: BTreeSet<&str> = self
            .stale_example_findings
            .iter()
            .map(|finding| finding.finding_id.as_str())
            .collect();
        let maintenance_ids: BTreeSet<&str> = self
            .maintenance_packets
            .iter()
            .map(|packet| packet.maintenance_packet_id.as_str())
            .collect();

        for class in DocsMaintenanceArtifactClass::REQUIRED {
            if !self
                .maintenance_packets
                .iter()
                .any(|packet| packet.artifact_class == class)
            {
                findings.push(DocsMaintenanceGovernanceFinding::new(
                    DocsMaintenanceGovernanceFindingKind::MaintenancePacketCoverageMissing,
                    self.packet_id.as_str(),
                    format!(
                        "stable docs rows require a maintenance packet for {}",
                        class.as_str()
                    ),
                ));
            }
        }

        for packet in &self.maintenance_packets {
            if packet
                .validation_result_refs
                .iter()
                .any(|id| !validation_ids.contains(id.as_str()))
                || packet
                    .pending_suggestion_refs
                    .iter()
                    .any(|id| !suggestion_ids.contains(id.as_str()))
                || packet
                    .stale_finding_refs
                    .iter()
                    .any(|id| !finding_ids.contains(id.as_str()))
            {
                findings.push(DocsMaintenanceGovernanceFinding::new(
                    DocsMaintenanceGovernanceFindingKind::MaintenancePacketIncomplete,
                    &packet.maintenance_packet_id,
                    "maintenance packet references unknown validation, suggestion, or stale-finding objects",
                ));
            }
        }

        for surface in DocsMaintenanceGovernanceSurface::REQUIRED {
            match self
                .consumer_projections
                .iter()
                .find(|projection| projection.surface == surface)
            {
                Some(projection)
                    if projection.is_well_formed(&self.packet_id)
                        && projection
                            .maintenance_packet_refs
                            .iter()
                            .all(|id| maintenance_ids.contains(id.as_str())) => {}
                _ => findings.push(DocsMaintenanceGovernanceFinding::new(
                    DocsMaintenanceGovernanceFindingKind::ConsumerProjectionDroppedVocabulary,
                    self.packet_id.as_str(),
                    format!(
                        "consumer projection {} must preserve shared source/version, validation, stale-example, handoff, and destination vocabulary",
                        surface.as_str()
                    ),
                )),
            }
        }
    }

    /// Returns true when a required consumer surface projection exists.
    pub fn has_projection_for(&self, surface: DocsMaintenanceGovernanceSurface) -> bool {
        self.consumer_projections
            .iter()
            .any(|projection| projection.surface == surface)
    }

    /// Builds an export-safe support/community handoff projection.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
    ) -> DocsMaintenanceGovernanceSupportExport {
        DocsMaintenanceGovernanceSupportExport {
            record_kind: DOCS_MAINTENANCE_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_VERSION,
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            export_packet_id_ref: self.packet_id.clone(),
            export_packet: self.clone(),
            raw_boundary_material_exported: false,
        }
    }
}

/// Export-safe support/community handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsMaintenanceGovernanceSupportExport {
    /// Stable record discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export generation time.
    pub generated_at: String,
    /// Source governance packet id.
    pub export_packet_id_ref: String,
    /// Full metadata-only governance packet.
    pub export_packet: DocsMaintenanceGovernancePacket,
    /// Whether raw document bodies, raw URLs, or rendered HTML were exported.
    pub raw_boundary_material_exported: bool,
}

impl DocsMaintenanceGovernanceSupportExport {
    /// Returns true when the export preserves the packet and omits raw bodies.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DOCS_MAINTENANCE_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DOCS_MAINTENANCE_GOVERNANCE_SCHEMA_VERSION
            && !self.export_id.trim().is_empty()
            && !self.generated_at.trim().is_empty()
            && self.export_packet_id_ref == self.export_packet.packet_id
            && !self.raw_boundary_material_exported
            && !self.export_packet.raw_boundary_material_exported
    }
}

/// Error returned when loading the checked-in governance artifact fails.
#[derive(Debug)]
pub enum DocsMaintenanceGovernanceArtifactError {
    /// Artifact file could not be read.
    Read {
        /// Repo-relative artifact path.
        path: &'static str,
        /// Source IO error.
        source: std::io::Error,
    },
    /// Artifact JSON could not be parsed.
    Parse {
        /// Repo-relative artifact path.
        path: &'static str,
        /// Source JSON error.
        source: serde_json::Error,
    },
    /// Artifact validates with findings.
    Validate {
        /// Validation findings.
        findings: Vec<DocsMaintenanceGovernanceFinding>,
    },
}

impl fmt::Display for DocsMaintenanceGovernanceArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => write!(f, "failed to read {path}: {source}"),
            Self::Parse { path, source } => write!(f, "failed to parse {path}: {source}"),
            Self::Validate { findings } => {
                write!(f, "docs-maintenance governance artifact failed validation")?;
                for finding in findings {
                    write!(
                        f,
                        "; {} {}: {}",
                        finding.finding_kind.as_str(),
                        finding.object_ref,
                        finding.summary
                    )?;
                }
                Ok(())
            }
        }
    }
}

impl Error for DocsMaintenanceGovernanceArtifactError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } => Some(source),
            Self::Parse { source, .. } => Some(source),
            Self::Validate { .. } => None,
        }
    }
}

/// Loads and validates the checked-in stable governance packet.
pub fn current_docs_maintenance_and_stale_example_governance_packet(
) -> Result<DocsMaintenanceGovernancePacket, DocsMaintenanceGovernanceArtifactError> {
    let artifact_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(DOCS_MAINTENANCE_GOVERNANCE_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&artifact_path).map_err(|source| {
        DocsMaintenanceGovernanceArtifactError::Read {
            path: DOCS_MAINTENANCE_GOVERNANCE_ARTIFACT_REF,
            source,
        }
    })?;
    let packet: DocsMaintenanceGovernancePacket =
        serde_json::from_str(&payload).map_err(|source| {
            DocsMaintenanceGovernanceArtifactError::Parse {
                path: DOCS_MAINTENANCE_GOVERNANCE_ARTIFACT_REF,
                source,
            }
        })?;
    let findings = packet.validate();
    if findings.is_empty()
        && packet.promotion_state == DocsMaintenanceGovernancePromotionState::Stable
    {
        Ok(packet)
    } else {
        Err(DocsMaintenanceGovernanceArtifactError::Validate { findings })
    }
}

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn publish_scope() -> DocsPublishScope {
    DocsPublishScope {
        branch_scope: Some("branch:release/stable-docs".to_owned()),
        release_scope: Some("release:stable-2026.06".to_owned()),
        channel_scope: Some("stable".to_owned()),
    }
}

fn validation(id: &str, target: &str, mode: DocsExampleValidationMode) -> DocsValidationResult {
    DocsValidationResult {
        validation_result_id: id.to_owned(),
        artifact_or_snippet_ref: target.to_owned(),
        validation_mode: mode,
        outcome: DocsValidationOutcome::Passed,
        toolchain_context_ref: "toolchain:docs-validation:stable-2026.06".to_owned(),
        output_snapshot_ref: format!("snapshot:{id}:output"),
        diagnostics_refs: refs(&["diagnostics:docs-validation:clean"]),
        freshness_class: DocsFreshnessClass::AuthoritativeLive,
        last_checked_at: "2026-06-04T17:00:00Z".to_owned(),
        evidence_refs: refs(&["evidence:docs-validation:stable-2026.06"]),
    }
}

fn maintenance_packet(
    id: &str,
    class: DocsMaintenanceArtifactClass,
    artifact_ref: &str,
    validation_ref: &str,
    suggestion_refs: &[&str],
    finding_refs: &[&str],
    surface_refs: &[&str],
) -> DocsMaintenancePacket {
    DocsMaintenancePacket {
        maintenance_packet_id: id.to_owned(),
        artifact_class: class,
        artifact_ref: artifact_ref.to_owned(),
        docs_source_ref: format!("source:{artifact_ref}"),
        docs_version_ref: "docs-version:stable-2026.06".to_owned(),
        audience_scope: DocsAudienceScope::PublicReader,
        publish_scope: publish_scope(),
        validation_result_refs: refs(&[validation_ref]),
        pending_suggestion_refs: refs(suggestion_refs),
        stale_finding_refs: refs(finding_refs),
        validation_freshness: DocsFreshnessClass::AuthoritativeLive,
        version_match_state: VersionMatchState::ExactBuildMatch,
        publish_boundary_state: DocsPublishBoundaryState::PublishHandoffScoped,
        publish_boundary_notes: refs(&[
            "Stable docs publication is scoped to the release branch and channel.",
        ]),
        share_export_posture: DocsShareExportPosture::ReferenceOnlyExportable,
        surface_refs: refs(surface_refs),
    }
}

/// Returns the seeded stable docs-maintenance governance input.
pub fn seeded_docs_maintenance_and_stale_example_governance_input(
) -> DocsMaintenanceGovernancePacketInput {
    let packet_id = "packet:docs-maintenance-governance:stable-2026.06".to_owned();
    let maintenance_ids = [
        "maintenance-packet:readme:stable",
        "maintenance-packet:changelog:stable",
        "maintenance-packet:onboarding:stable",
        "maintenance-packet:module-docs:stable",
        "maintenance-packet:support-article:stable",
    ];

    DocsMaintenanceGovernancePacketInput {
        packet_id: packet_id.clone(),
        generated_at: "2026-06-04T17:05:00Z".to_owned(),
        source_branch_ref: "branch:release/stable-docs".to_owned(),
        release_ref: "release:stable-2026.06".to_owned(),
        channel_ref: "stable".to_owned(),
        render_configs: vec![
            DocsRenderConfig {
                render_config_id: "render-config:readme:source".to_owned(),
                mode: DocsPreviewMode::Source,
                commonmark_baseline_ref: "commonmark:0.31".to_owned(),
                extension_baseline: Vec::new(),
                active_content_state: DocsActiveContentState::NotApplicable,
                docs_source_ref: "source:docs/readme".to_owned(),
                docs_version_ref: "docs-version:stable-2026.06".to_owned(),
                security_profile: DocsRenderSecurityProfile::SourceFirstSafeMarkdown,
                mirror_browser_handoff_posture: DocsMirrorBrowserHandoffPosture::LocalOnly,
                browser_handoff_packet_ref: None,
                rendered_preview_not_canonical: true,
                evidence_refs: refs(&["evidence:render-config:readme"]),
            },
            DocsRenderConfig {
                render_config_id: "render-config:release-notes:rendered".to_owned(),
                mode: DocsPreviewMode::Rendered,
                commonmark_baseline_ref: "commonmark:0.31".to_owned(),
                extension_baseline: refs(&["tables", "autolink"]),
                active_content_state: DocsActiveContentState::Blocked,
                docs_source_ref: "source:docs/release-notes".to_owned(),
                docs_version_ref: "docs-version:stable-2026.06".to_owned(),
                security_profile: DocsRenderSecurityProfile::BrowserHandoffRequired,
                mirror_browser_handoff_posture:
                    DocsMirrorBrowserHandoffPosture::BrowserHandoffAvailable,
                browser_handoff_packet_ref: Some(
                    "browser-handoff:docs-publish:stable-2026.06".to_owned(),
                ),
                rendered_preview_not_canonical: true,
                evidence_refs: refs(&["evidence:render-config:release-notes"]),
            },
            DocsRenderConfig {
                render_config_id: "render-config:support-article:mirror".to_owned(),
                mode: DocsPreviewMode::Split,
                commonmark_baseline_ref: "commonmark:0.31".to_owned(),
                extension_baseline: refs(&["tables"]),
                active_content_state: DocsActiveContentState::SanitizedInertOnly,
                docs_source_ref: "source:docs/support-article".to_owned(),
                docs_version_ref: "docs-version:stable-2026.06".to_owned(),
                security_profile: DocsRenderSecurityProfile::MirrorOfflineSafe,
                mirror_browser_handoff_posture: DocsMirrorBrowserHandoffPosture::MirroredOffline,
                browser_handoff_packet_ref: None,
                rendered_preview_not_canonical: true,
                evidence_refs: refs(&["evidence:render-config:support"]),
            },
        ],
        suggestions: vec![
            DocsSuggestionObject {
                suggestion_id: "suggestion:readme:refresh-command-example".to_owned(),
                target_artifact_ref: "docs/readme".to_owned(),
                target_section_ref: Some("section:getting-started".to_owned()),
                trigger: DocsSuggestionTrigger::StaleExample,
                evidence_refs: refs(&["evidence:stale-example:readme-command"]),
                diff_ref: "diff:docs/readme:refresh-command-example".to_owned(),
                confidence_class: CitationConfidenceClass::EvidenceBacked,
                freshness_class: DocsFreshnessClass::AuthoritativeLive,
                audience_scope: DocsAudienceScope::Developer,
                share_export_posture: DocsShareExportPosture::ReviewPacketExportable,
                publish_boundary_state: DocsPublishBoundaryState::ReviewHandoffScoped,
                publish_boundary_note: "Suggestion is review-scoped before stable publication."
                    .to_owned(),
            },
            DocsSuggestionObject {
                suggestion_id: "suggestion:release-notes:publish-boundary".to_owned(),
                target_artifact_ref: "docs/release/notes".to_owned(),
                target_section_ref: Some("section:known-limits".to_owned()),
                trigger: DocsSuggestionTrigger::ReleaseNoteDrift,
                evidence_refs: refs(&["evidence:release-notes:claim-boundary"]),
                diff_ref: "diff:docs/release/notes:known-limits".to_owned(),
                confidence_class: CitationConfidenceClass::EvidenceBacked,
                freshness_class: DocsFreshnessClass::AuthoritativeLive,
                audience_scope: DocsAudienceScope::ReleaseManager,
                share_export_posture: DocsShareExportPosture::ReviewPacketExportable,
                publish_boundary_state: DocsPublishBoundaryState::PublishHandoffScoped,
                publish_boundary_note: "Publish handoff remains scoped to stable channel."
                    .to_owned(),
            },
        ],
        validation_results: vec![
            validation(
                "validation:readme:rendered",
                "docs/readme",
                DocsExampleValidationMode::Rendered,
            ),
            validation(
                "validation:changelog:syntax",
                "docs/changelog",
                DocsExampleValidationMode::SyntaxChecked,
            ),
            validation(
                "validation:onboarding:executed-local",
                "docs/onboarding",
                DocsExampleValidationMode::ExecutedLocally,
            ),
            validation(
                "validation:module-docs:executed-remote",
                "docs/module-docs",
                DocsExampleValidationMode::ExecutedRemotely,
            ),
            validation(
                "validation:support-article:rendered",
                "docs/support/article",
                DocsExampleValidationMode::Rendered,
            ),
        ],
        stale_example_findings: vec![
            StaleExampleGovernanceFinding {
                finding_id: "stale-finding:readme:command-example".to_owned(),
                finding_class: DocsFindingClass::StaleExample,
                detection_state: DocsFindingDetectionState::ProvenBroken,
                validation_mode: DocsExampleValidationMode::ExecutedLocally,
                environment_scope_ref: "environment:local:stable-docs".to_owned(),
                version_scope_ref: "version:aureline:stable-2026.06".to_owned(),
                last_checked_at: "2026-06-04T17:00:00Z".to_owned(),
                freshness_class: DocsFreshnessClass::AuthoritativeLive,
                suppression_state: DocsFindingSuppressionState::Active,
                suppression: None,
                evidence_refs: refs(&["evidence:stale-example:readme-command"]),
                publish_boundary_notes: refs(&[
                    "Finding remains visible in Help/About, onboarding, release notes, and support handoff.",
                ]),
            },
            StaleExampleGovernanceFinding {
                finding_id: "stale-finding:support:mirrored-example".to_owned(),
                finding_class: DocsFindingClass::CommandOutputDrift,
                detection_state: DocsFindingDetectionState::SuspectedStale,
                validation_mode: DocsExampleValidationMode::Stale,
                environment_scope_ref: "environment:mirror-offline:stable-docs".to_owned(),
                version_scope_ref: "version:aureline:stable-2026.06".to_owned(),
                last_checked_at: "2026-06-04T17:00:00Z".to_owned(),
                freshness_class: DocsFreshnessClass::WarmCached,
                suppression_state: DocsFindingSuppressionState::SuppressedUntilReviewed,
                suppression: Some(DocsFindingSuppression {
                    actor_ref: "actor:docs-maintainer".to_owned(),
                    reason: "Mirror is current but remote execution is unavailable until the next support-lab window.".to_owned(),
                    expiry_at: "2026-06-18T00:00:00Z".to_owned(),
                    evidence_refs: refs(&["evidence:suppression:support-mirror"]),
                }),
                evidence_refs: refs(&["evidence:stale-example:support-mirror"]),
                publish_boundary_notes: refs(&[
                    "Suppression is time-bounded and remains exportable with actor, reason, expiry, and evidence.",
                ]),
            },
        ],
        maintenance_packets: vec![
            maintenance_packet(
                maintenance_ids[0],
                DocsMaintenanceArtifactClass::Readme,
                "docs/readme",
                "validation:readme:rendered",
                &["suggestion:readme:refresh-command-example"],
                &["stale-finding:readme:command-example"],
                &["surface:help-about", "surface:docs-feedback-export"],
            ),
            maintenance_packet(
                maintenance_ids[1],
                DocsMaintenanceArtifactClass::Changelog,
                "docs/changelog",
                "validation:changelog:syntax",
                &["suggestion:release-notes:publish-boundary"],
                &[],
                &["surface:release-notes", "surface:docs-pack"],
            ),
            maintenance_packet(
                maintenance_ids[2],
                DocsMaintenanceArtifactClass::Onboarding,
                "docs/onboarding",
                "validation:onboarding:executed-local",
                &[],
                &["stale-finding:readme:command-example"],
                &["surface:onboarding-cards"],
            ),
            maintenance_packet(
                maintenance_ids[3],
                DocsMaintenanceArtifactClass::ModuleDocs,
                "docs/module-docs",
                "validation:module-docs:executed-remote",
                &[],
                &[],
                &["surface:cli-headless", "surface:migration-guidance"],
            ),
            maintenance_packet(
                maintenance_ids[4],
                DocsMaintenanceArtifactClass::SupportArticle,
                "docs/support/article",
                "validation:support-article:rendered",
                &[],
                &["stale-finding:support:mirrored-example"],
                &["surface:support-community-handoff"],
            ),
        ],
        consumer_projections: DocsMaintenanceGovernanceSurface::REQUIRED
            .iter()
            .map(|surface| DocsMaintenanceGovernanceProjection {
                surface: *surface,
                packet_id_ref: packet_id.clone(),
                maintenance_packet_refs: maintenance_ids.iter().map(|id| (*id).to_owned()).collect(),
                preserves_source_version_refs: true,
                preserves_validation_freshness: true,
                preserves_stale_findings_and_suppressions: true,
                preserves_publish_boundary_and_handoff: true,
                preserves_destination_vocabulary: true,
            })
            .collect(),
        omitted_material_classes: refs(&[
            "raw_document_bodies",
            "rendered_html",
            "raw_urls",
            "provider_payloads",
            "ambient_credentials",
        ]),
        raw_boundary_material_exported: false,
    }
}
