//! Frozen M5 Markdown-authoring, safe-preview, docs-maintenance, and
//! docs-evidence-handoff matrix.
//!
//! This module locks the canonical M5 depth qualification for the five
//! docs-authoring surfaces — the Markdown authoring workspace, the CommonMark
//! safe-preview baseline, docs-maintenance suggestions, docs validation, and
//! docs evidence handoff — into one export-safe packet. Each
//! [`M5AuthoringMatrixLaneRow`] binds a surface to its qualification class, the
//! workspace modes it supports, its rendered-preview safety class, its
//! validation states, its docs-suggestion triggers, its evidence-handoff scope,
//! required evidence packet refs, downgrade triggers, rollback posture, source
//! contracts, and consumer-surface parity.
//!
//! The matrix is the single source of truth for whether these surfaces may ship
//! as Stable, Beta, Preview, or must narrow further. It maps every claimed M5
//! docs/browser authoring surface onto the canonical vocabulary already owned by
//! the docs-maintenance runtime — source/split/rendered modes, sanitized/raw-HTML
//! preview safety classes, validation states, suggestion triggers, and
//! local/review/publish handoff scopes — instead of letting README, changelog,
//! help, or tutorial editing drift into feature-local conventions. It references
//! the upstream maintenance, suggestion, browser-handoff, and docs-pack contracts
//! by id rather than embedding their content. Raw document bodies, raw source
//! files, rendered HTML, raw provider payloads, credentials, and live vendor-doc
//! snapshots stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/docs/freeze-the-m5-markdown-authoring-safe-preview-docs-maintenance-and-docs-evidence-handoff-matrix.schema.json`](../../../../schemas/docs/freeze-the-m5-markdown-authoring-safe-preview-docs-maintenance-and-docs-evidence-handoff-matrix.schema.json).
//! The contract doc is
//! [`docs/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix.md`](../../../../docs/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/`](../../../../fixtures/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AuthoringMatrixPacket`].
pub const M5_AUTHORING_MATRIX_RECORD_KIND: &str =
    "freeze_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix";

/// Schema version for M5 docs-authoring matrix records.
pub const M5_AUTHORING_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_AUTHORING_MATRIX_SCHEMA_REF: &str =
    "schemas/docs/freeze-the-m5-markdown-authoring-safe-preview-docs-maintenance-and-docs-evidence-handoff-matrix.schema.json";

/// Repo-relative path of the M5 docs-authoring matrix contract doc.
pub const M5_AUTHORING_MATRIX_DOC_REF: &str =
    "docs/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix.md";

/// Repo-relative path of the frozen docs-maintenance row contract.
pub const M5_AUTHORING_MATRIX_MAINTENANCE_CONTRACT_REF: &str =
    "schemas/docs/docs_maintenance_row.schema.json";

/// Repo-relative path of the frozen docs-suggestion card contract.
pub const M5_AUTHORING_MATRIX_SUGGESTION_CONTRACT_REF: &str =
    "schemas/docs/docs_suggestion_card.schema.json";

/// Repo-relative path of the frozen docs-browser handoff truth contract.
pub const M5_AUTHORING_MATRIX_BROWSER_HANDOFF_CONTRACT_REF: &str =
    "schemas/docs/docs_browser_truth_packet.schema.json";

/// Repo-relative path of the frozen docs-pack truth contract (mirror/offline parity).
pub const M5_AUTHORING_MATRIX_DOCS_PACK_CONTRACT_REF: &str =
    "schemas/docs/docs_pack_truth_packet.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_AUTHORING_MATRIX_FIXTURE_DIR: &str =
    "fixtures/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AUTHORING_MATRIX_ARTIFACT_REF: &str =
    "artifacts/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_AUTHORING_MATRIX_SUMMARY_REF: &str =
    "artifacts/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix.md";

/// One of the five M5 docs-authoring surfaces governed by this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthoringSurface {
    /// Governed Markdown authoring workspace for README/changelog/help/tutorial/module docs.
    MarkdownAuthoringWorkspace,
    /// CommonMark safe-preview baseline rendering Markdown to a sanitized, labeled view.
    #[serde(rename = "commonmark_preview")]
    CommonMarkPreview,
    /// Diff-first docs-maintenance and stale-example suggestions.
    DocsMaintenanceSuggestions,
    /// Validation states for documented examples and links.
    DocsValidation,
    /// Evidence handoff tying a prose change back to code/schema/release truth.
    DocsEvidenceHandoff,
}

impl M5AuthoringSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MarkdownAuthoringWorkspace,
        Self::CommonMarkPreview,
        Self::DocsMaintenanceSuggestions,
        Self::DocsValidation,
        Self::DocsEvidenceHandoff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarkdownAuthoringWorkspace => "markdown_authoring_workspace",
            Self::CommonMarkPreview => "commonmark_preview",
            Self::DocsMaintenanceSuggestions => "docs_maintenance_suggestions",
            Self::DocsValidation => "docs_validation",
            Self::DocsEvidenceHandoff => "docs_evidence_handoff",
        }
    }
}

/// Qualification class for an M5 docs-authoring surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthoringQualificationClass {
    /// Surface qualifies for the Stable claim.
    Stable,
    /// Surface is narrowed to Beta.
    Beta,
    /// Surface is narrowed to Preview.
    Preview,
    /// Surface is experimental and not claimed.
    Experimental,
    /// Surface is unavailable on this build.
    Unavailable,
    /// Surface is held pending upstream resolution.
    Held,
}

impl M5AuthoringQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the surface may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Authoring workspace mode a surface supports.
///
/// Mirrors the canonical `DocsPreviewMode` vocabulary owned by the
/// docs-maintenance runtime so authoring, preview, and maintenance share one set
/// of mode tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthoringWorkspaceMode {
    /// Raw Markdown source is shown; nothing is rendered.
    Source,
    /// Source and rendered preview are shown side by side.
    Split,
    /// Rendered preview is shown.
    Rendered,
}

impl M5AuthoringWorkspaceMode {
    /// Stable token recorded in the matrix.
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

/// Rendered-preview safety class for an authoring surface.
///
/// Mirrors the canonical `DocsPreviewSanitizationState` vocabulary: rendered
/// previews are sanitized by default and raw embedded HTML is blocked unless it
/// renders under an explicit disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthoringPreviewSafetyClass {
    /// HTML was sanitized; scripts, iframes, and event handlers were stripped.
    SanitizedSafe,
    /// Raw embedded HTML was present and blocked from rendering.
    RawHtmlBlocked,
    /// Raw embedded HTML rendered under an explicit disclosure.
    RawHtmlAllowedDisclosed,
    /// Nothing is rendered, so sanitization does not apply.
    NotApplicable,
}

impl M5AuthoringPreviewSafetyClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SanitizedSafe => "sanitized_safe",
            Self::RawHtmlBlocked => "raw_html_blocked",
            Self::RawHtmlAllowedDisclosed => "raw_html_allowed_disclosed",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Returns true when the class reflects a concrete rendered-preview posture.
    pub const fn is_concrete_render_posture(self) -> bool {
        !matches!(self, Self::NotApplicable)
    }
}

/// Validation state a docs-authoring surface may report.
///
/// Validation truth never silently upgrades to verified; unverified, stale, and
/// unsupported states stay visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthoringValidationState {
    /// A validator observed a concrete pass against current behavior.
    Validated,
    /// Drift signals exist but failure was not reproduced.
    SuspectedStale,
    /// Content is unchanged but required validation is missing or expired.
    UnchangedUnverified,
    /// Validation is unsupported in the current environment.
    Unsupported,
    /// Validation was deliberately skipped.
    Skipped,
    /// A prior validation result is stale and must be rerun.
    StaleRerunRequired,
    /// The example or link was not validated.
    NotValidated,
}

impl M5AuthoringValidationState {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Validated => "validated",
            Self::SuspectedStale => "suspected_stale",
            Self::UnchangedUnverified => "unchanged_unverified",
            Self::Unsupported => "unsupported",
            Self::Skipped => "skipped",
            Self::StaleRerunRequired => "stale_rerun_required",
            Self::NotValidated => "not_validated",
        }
    }
}

/// Trigger source that produces a docs-maintenance suggestion.
///
/// Mirrors the canonical `DocsSuggestionTrigger` vocabulary owned by the
/// docs-maintenance runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthoringSuggestionTrigger {
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

impl M5AuthoringSuggestionTrigger {
    /// Stable token recorded in the matrix.
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

/// Evidence-handoff scope label for a docs-authoring surface.
///
/// Mirrors the canonical `DocsPublishBoundaryState` vocabulary: docs work stays
/// local unless it crosses an explicit, scoped review or publish boundary, and an
/// unscoped external publish attempt is blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthoringEvidenceHandoffScope {
    /// Work stays local to the workspace and never crosses a publish boundary.
    LocalOnly,
    /// Work is staged for a scoped review handoff that stays inside review.
    ReviewHandoffScoped,
    /// Work is staged for a scoped, explicit publish handoff.
    PublishHandoffScoped,
    /// An external publish was attempted without scope and is blocked.
    BlockedUnscoped,
}

impl M5AuthoringEvidenceHandoffScope {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ReviewHandoffScoped => "review_handoff_scoped",
            Self::PublishHandoffScoped => "publish_handoff_scoped",
            Self::BlockedUnscoped => "blocked_unscoped",
        }
    }

    /// Returns true when the scope crosses a review or publish boundary.
    pub const fn crosses_boundary(self) -> bool {
        matches!(self, Self::ReviewHandoffScoped | Self::PublishHandoffScoped)
    }
}

/// Evidence requirement level for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthoringEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Not applicable for this surface's current qualification.
    NotApplicable,
}

impl M5AuthoringEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade trigger that can narrow a surface below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthoringDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Pinned, signed mirror is offline or unavailable.
    MirrorOffline,
    /// Source version no longer matches the indexed/pinned version.
    SourceVersionMismatch,
    /// Freshness window for the docs source expired.
    FreshnessExpired,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified authoring/handoff boundary.
    ScopeExpansionUnqualified,
    /// A rendered preview encountered unsafe content and was blocked.
    UnsafePreviewBlocked,
    /// An upstream dependency surface narrowed.
    UpstreamDependencyNarrowed,
}

impl M5AuthoringDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::MirrorOffline,
        Self::SourceVersionMismatch,
        Self::FreshnessExpired,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UnsafePreviewBlocked,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::MirrorOffline => "mirror_offline",
            Self::SourceVersionMismatch => "source_version_mismatch",
            Self::FreshnessExpired => "freshness_expired",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UnsafePreviewBlocked => "unsafe_preview_blocked",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthoringRollbackPosture {
    /// Source stays canonical; the surface never mutates source from a rendered view.
    SourceCanonicalNoMutation,
    /// Every change ships as a reviewable diff; nothing is applied without review.
    DiffFirstReviewRequired,
    /// Handoff always preserves a safe return path to the IDE.
    ReturnPathPreserved,
    /// Evidence is preserved but no automatic revert exists.
    EvidencePreservedNoRevert,
    /// Not applicable for the surface's current qualification.
    NotApplicable,
}

impl M5AuthoringRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceCanonicalNoMutation => "source_canonical_no_mutation",
            Self::DiffFirstReviewRequired => "diff_first_review_required",
            Self::ReturnPathPreserved => "return_path_preserved",
            Self::EvidencePreservedNoRevert => "evidence_preserved_no_revert",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project a docs-authoring surface's qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthoringConsumerSurface {
    /// Markdown authoring workspace.
    AuthoringWorkspace,
    /// Rendered-preview pane.
    PreviewPane,
    /// Docs-maintenance panel.
    DocsMaintenancePanel,
    /// Docs browser / reader surface.
    DocsBrowser,
    /// Browser companion / handoff follow-up.
    BrowserCompanion,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Release center / publish review.
    ReleaseCenter,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl M5AuthoringConsumerSurface {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoringWorkspace => "authoring_workspace",
            Self::PreviewPane => "preview_pane",
            Self::DocsMaintenancePanel => "docs_maintenance_panel",
            Self::DocsBrowser => "docs_browser",
            Self::BrowserCompanion => "browser_companion",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseCenter => "release_center",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One row in the M5 docs-authoring matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthoringMatrixLaneRow {
    /// Docs-authoring surface.
    pub surface: M5AuthoringSurface,
    /// Qualification class earned by this surface.
    pub qualification: M5AuthoringQualificationClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Workspace modes the surface supports.
    pub supported_workspace_modes: Vec<M5AuthoringWorkspaceMode>,
    /// Rendered-preview safety class.
    pub preview_safety_class: M5AuthoringPreviewSafetyClass,
    /// Validation states the surface may report.
    pub validation_states: Vec<M5AuthoringValidationState>,
    /// Docs-suggestion triggers handled by the surface.
    pub suggestion_triggers: Vec<M5AuthoringSuggestionTrigger>,
    /// Evidence-handoff scope for the surface.
    pub evidence_handoff_scope: M5AuthoringEvidenceHandoffScope,
    /// Evidence requirement level.
    pub evidence_requirement: M5AuthoringEvidenceRequirement,
    /// Required evidence packet refs for this qualification.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5AuthoringDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5AuthoringRollbackPosture,
    /// Source contract refs consumed by this surface.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this surface's qualification.
    pub consumer_surfaces: Vec<M5AuthoringConsumerSurface>,
}

impl M5AuthoringMatrixLaneRow {
    /// Returns true when the surface renders a preview in any supported mode.
    pub fn renders_preview(&self) -> bool {
        self.supported_workspace_modes
            .iter()
            .any(|mode| mode.renders_preview())
    }
}

/// Trust and provenance review block.
///
/// Every flag is a hard invariant; all must hold for the matrix to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthoringMatrixTrustReview {
    /// Docs stay source-canonical; rendered views remain safe and labeled.
    pub source_canonical_rendered_safe_and_labeled: bool,
    /// Rendered preview is sanitized and safe by default.
    pub rendered_preview_safe_by_default: bool,
    /// Rendered preview, diagram engines, and suggestions are never privileged execution paths.
    pub preview_not_privileged_execution_path: bool,
    /// Docs suggestions stay diff-first and are never silently auto-applied.
    pub suggestions_diff_first_never_auto_applied: bool,
    /// Source, version, and freshness truth stays visible.
    pub source_version_freshness_truth_visible: bool,
    /// Validation state is never silently upgraded to verified.
    pub validation_state_never_silently_upgraded: bool,
    /// Evidence handoff stays source-linked to code, schema, or release truth.
    pub evidence_handoff_source_linked: bool,
    /// Browser handoff never hides owner, origin, or boundary changes.
    pub handoff_never_hides_owner_origin_or_boundary: bool,
    /// Browser handoff never silently widens authority.
    pub handoff_never_silently_widens_authority: bool,
    /// No full browser product, collaborative rich-text editor, or remote CMS is in scope.
    pub no_full_browser_collab_editor_or_remote_cms: bool,
    /// Downgrade narrows the claim rather than hiding the surface.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthoringMatrixConsumerProjection {
    /// Authoring workspace shows the active mode and source truth.
    pub authoring_workspace_shows_mode_and_source_truth: bool,
    /// Preview pane shows the rendered-preview safety class.
    pub preview_shows_safety_class: bool,
    /// Maintenance panel shows suggestion triggers and the review diff.
    pub maintenance_panel_shows_suggestion_triggers_and_diff: bool,
    /// Validation surface shows the validation state.
    pub validation_shows_state: bool,
    /// Evidence handoff shows the handoff scope and source link.
    pub evidence_handoff_shows_scope_and_source_link: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_qualification: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_qualification: bool,
    /// Release center shows qualification truth.
    pub release_center_shows_qualification: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_qualification: bool,
    /// Preview / Labs surfaces are visibly labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified_surfaces: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthoringMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the surface.
    pub auto_narrow_on_stale: bool,
}

/// Release and mirror/offline parity posture for the authoring lane.
///
/// Captures the supporting release packet, mirror/offline packet, and the
/// support/export parity expectations the lane must satisfy before authoring
/// depth widens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthoringMatrixReleasePosture {
    /// Ref of the supporting release packet for the authoring lane.
    pub release_packet_ref: String,
    /// Ref of the supporting mirror/offline packet for the authoring lane.
    pub mirror_offline_packet_ref: String,
    /// True when support/export parity is required for every authoring surface.
    pub support_export_parity_required: bool,
    /// True when mirror/offline parity is required for every authoring surface.
    pub mirror_offline_parity_required: bool,
}

/// Constructor input for [`M5AuthoringMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AuthoringMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Lane rows.
    pub lane_rows: Vec<M5AuthoringMatrixLaneRow>,
    /// Trust review block.
    pub trust_review: M5AuthoringMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5AuthoringMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AuthoringMatrixProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5AuthoringMatrixReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 docs-authoring matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthoringMatrixPacket {
    /// Record kind; must equal [`M5_AUTHORING_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AUTHORING_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Lane rows.
    pub lane_rows: Vec<M5AuthoringMatrixLaneRow>,
    /// Trust review block.
    pub trust_review: M5AuthoringMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5AuthoringMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AuthoringMatrixProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5AuthoringMatrixReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AuthoringMatrixPacket {
    /// Builds an M5 docs-authoring matrix packet from stable-lane input.
    pub fn new(input: M5AuthoringMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_AUTHORING_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_AUTHORING_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            lane_rows: input.lane_rows,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 docs-authoring matrix invariants.
    pub fn validate(&self) -> Vec<M5AuthoringMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_AUTHORING_MATRIX_RECORD_KIND {
            violations.push(M5AuthoringMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AUTHORING_MATRIX_SCHEMA_VERSION {
            violations.push(M5AuthoringMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AuthoringMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_lane_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 authoring matrix packet serializes"),
        ) {
            violations.push(M5AuthoringMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 authoring matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_surfaces = self
            .lane_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Docs Authoring, Preview, Maintenance, and Evidence-Handoff Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Surfaces: {} ({} stable)\n",
            self.lane_rows.len(),
            stable_surfaces
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Surfaces\n\n");
        for row in &self.lane_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Preview safety: {}\n",
                row.preview_safety_class.as_str()
            ));
            out.push_str(&format!(
                "  - Handoff scope: {}\n",
                row.evidence_handoff_scope.as_str()
            ));
            out.push_str(&format!(
                "  - Rollback: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 docs-authoring matrix export.
#[derive(Debug)]
pub enum M5AuthoringMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AuthoringMatrixViolation>),
}

impl fmt::Display for M5AuthoringMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 authoring matrix export parse failed: {error}"
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
                    "m5 authoring matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AuthoringMatrixArtifactError {}

/// Validation failures emitted by [`M5AuthoringMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AuthoringMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required surface is missing from the matrix.
    RequiredSurfaceMissing,
    /// A lane row is incomplete.
    LaneRowIncomplete,
    /// A surface claiming Stable is missing required evidence packet refs.
    StableSurfaceMissingEvidence,
    /// A rendering surface declares a non-concrete preview safety class.
    UnsafePreviewDefault,
    /// The docs-maintenance suggestions surface declares no suggestion triggers.
    SuggestionTriggersMissing,
    /// A surface has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5AuthoringMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::LaneRowIncomplete => "lane_row_incomplete",
            Self::StableSurfaceMissingEvidence => "stable_surface_missing_evidence",
            Self::UnsafePreviewDefault => "unsafe_preview_default",
            Self::SuggestionTriggersMissing => "suggestion_triggers_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 docs-authoring matrix export.
pub fn current_stable_m5_markdown_authoring_matrix_export(
) -> Result<M5AuthoringMatrixPacket, M5AuthoringMatrixArtifactError> {
    let packet: M5AuthoringMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/freeze_the_m5_markdown_authoring_safe_preview_docs_maintenance_and_docs_evidence_handoff_matrix/support_export.json"
    )))
    .map_err(M5AuthoringMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AuthoringMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5AuthoringMatrixPacket,
    violations: &mut Vec<M5AuthoringMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AUTHORING_MATRIX_SCHEMA_REF,
        M5_AUTHORING_MATRIX_DOC_REF,
        M5_AUTHORING_MATRIX_MAINTENANCE_CONTRACT_REF,
        M5_AUTHORING_MATRIX_SUGGESTION_CONTRACT_REF,
        M5_AUTHORING_MATRIX_BROWSER_HANDOFF_CONTRACT_REF,
        M5_AUTHORING_MATRIX_DOCS_PACK_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AuthoringMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_lane_rows(
    packet: &M5AuthoringMatrixPacket,
    violations: &mut Vec<M5AuthoringMatrixViolation>,
) {
    let present: BTreeSet<M5AuthoringSurface> =
        packet.lane_rows.iter().map(|row| row.surface).collect();
    for required in M5AuthoringSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5AuthoringMatrixViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.lane_rows {
        if row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.supported_workspace_modes.is_empty()
            || row.validation_states.is_empty()
        {
            violations.push(M5AuthoringMatrixViolation::LaneRowIncomplete);
        }
        if row.qualification.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5AuthoringMatrixViolation::StableSurfaceMissingEvidence);
        }
        if row.renders_preview() && !row.preview_safety_class.is_concrete_render_posture() {
            violations.push(M5AuthoringMatrixViolation::UnsafePreviewDefault);
        }
        if row.surface == M5AuthoringSurface::DocsMaintenanceSuggestions
            && row.suggestion_triggers.is_empty()
        {
            violations.push(M5AuthoringMatrixViolation::SuggestionTriggersMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AuthoringMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AuthoringMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_trust_review(
    packet: &M5AuthoringMatrixPacket,
    violations: &mut Vec<M5AuthoringMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.source_canonical_rendered_safe_and_labeled,
        review.rendered_preview_safe_by_default,
        review.preview_not_privileged_execution_path,
        review.suggestions_diff_first_never_auto_applied,
        review.source_version_freshness_truth_visible,
        review.validation_state_never_silently_upgraded,
        review.evidence_handoff_source_linked,
        review.handoff_never_hides_owner_origin_or_boundary,
        review.handoff_never_silently_widens_authority,
        review.no_full_browser_collab_editor_or_remote_cms,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5AuthoringMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AuthoringMatrixPacket,
    violations: &mut Vec<M5AuthoringMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.authoring_workspace_shows_mode_and_source_truth,
        projection.preview_shows_safety_class,
        projection.maintenance_panel_shows_suggestion_triggers_and_diff,
        projection.validation_shows_state,
        projection.evidence_handoff_shows_scope_and_source_link,
        projection.cli_headless_shows_qualification,
        projection.support_export_shows_qualification,
        projection.release_center_shows_qualification,
        projection.help_about_shows_qualification,
        projection.preview_labs_label_for_unqualified_surfaces,
    ] {
        if !ok {
            violations.push(M5AuthoringMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AuthoringMatrixPacket,
    violations: &mut Vec<M5AuthoringMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AuthoringMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AuthoringMatrixPacket,
    violations: &mut Vec<M5AuthoringMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
    {
        violations.push(M5AuthoringMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
