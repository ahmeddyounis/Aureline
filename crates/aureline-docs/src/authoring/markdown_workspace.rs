//! Governed Markdown authoring workspace truth packet.
//!
//! A [`MarkdownAuthoringWorkspace`] is the canonical, export-safe record for one
//! open Markdown authoring surface. It keeps source truth, preview safety, and
//! version/freshness disclosure visible while the document is edited in
//! [`DocsPreviewMode::Source`], [`DocsPreviewMode::Split`], or
//! [`DocsPreviewMode::Rendered`] modes.
//!
//! The packet binds:
//!
//! - the active mode, the remembered mode preference, the stable
//!   keyboard-reachable [`WorkspaceModeCommand`]s that switch modes, and a
//!   dedicated [`WorkspaceRecoveryCommand`] that always returns to raw source;
//! - the CommonMark baseline flag and note, the declared and active extension
//!   sets, the rendered-preview [`DocsPreviewSanitizationState`], and the
//!   diagram/math/custom-component [`WorkspaceRenderCapabilities`];
//! - the [`DocsSourceVersionBadge`] (source class, version, freshness, and
//!   version-match), the [`DocsMirrorOfflinePosture`] mirror/offline state, the
//!   [`DocsPublishBoundaryState`], and the [`BrowserHandoffAvailability`];
//! - the initiating code/doc [`WorkspaceAnchor`] context and the disclosure note
//!   that a rendered view is never canonical source.
//!
//! [`MarkdownAuthoringWorkspace::validate`] enforces the track invariant: docs
//! stay source-canonical, rendered views stay safe and labeled, rendered preview
//! and diagram/math/custom-component engines are never privileged execution
//! paths, source/version/freshness truth stays visible, and a rendered view
//! never masquerades as raw source.
//!
//! The boundary schema is
//! [`schemas/docs/markdown-authoring-workspace.schema.json`](../../../../schemas/docs/markdown-authoring-workspace.schema.json).
//! The contract/help doc is
//! [`docs/help/markdown-authoring-workspace.md`](../../../../docs/help/markdown-authoring-workspace.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/markdown-workspace-modes/`](../../../../fixtures/docs/m5/markdown-workspace-modes/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::citations::{DocsFreshnessClass, VersionMatchState};
use crate::evidence_model::DocsMirrorOfflinePosture;
use crate::maintenance::{
    DocsArtifactKind, DocsMaintenanceAction, DocsPreviewMode, DocsPreviewSanitizationState,
    DocsPublishBoundaryState, DocsSourceVersionBadge,
};

/// Stable record-kind tag carried by [`MarkdownAuthoringWorkspace`].
pub const MARKDOWN_WORKSPACE_RECORD_KIND: &str = "markdown_authoring_workspace_record";

/// Schema version for Markdown authoring workspace records.
pub const MARKDOWN_WORKSPACE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MARKDOWN_WORKSPACE_SCHEMA_REF: &str =
    "schemas/docs/markdown-authoring-workspace.schema.json";

/// Repo-relative path of the contract/help doc.
pub const MARKDOWN_WORKSPACE_DOC_REF: &str = "docs/help/markdown-authoring-workspace.md";

/// Repo-relative path of the protected fixture directory.
pub const MARKDOWN_WORKSPACE_FIXTURE_DIR: &str = "fixtures/docs/m5/markdown-workspace-modes";

/// Repo-relative path of the checked-in support-export artifact.
pub const MARKDOWN_WORKSPACE_ARTIFACT_REF: &str =
    "artifacts/docs/m5/markdown-workspace-proof/support_export.json";

/// Repo-relative path of the checked-in Markdown summary.
pub const MARKDOWN_WORKSPACE_SUMMARY_REF: &str = "artifacts/docs/m5/markdown-workspace-proof.md";

/// Stable command id that switches the workspace to [`DocsPreviewMode::Source`].
pub const MODE_SOURCE_COMMAND_ID: &str = "docs.authoring.mode.source";

/// Stable command id that switches the workspace to [`DocsPreviewMode::Split`].
pub const MODE_SPLIT_COMMAND_ID: &str = "docs.authoring.mode.split";

/// Stable command id that switches the workspace to [`DocsPreviewMode::Rendered`].
pub const MODE_RENDERED_COMMAND_ID: &str = "docs.authoring.mode.rendered";

/// Stable command id for the always-available recovery back to raw source.
pub const RECOVER_SOURCE_COMMAND_ID: &str = "docs.authoring.recover_source";

/// Posture of an optional rendered-preview capability beyond the CommonMark
/// baseline (diagrams, math, custom components).
///
/// There is deliberately no "privileged" or "executes" variant: rendered preview
/// capabilities are never a privileged execution path. The strongest active
/// posture is [`RenderCapability::SandboxedOptIn`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderCapability {
    /// Capability is disabled; content of this kind renders as inert source text.
    Disabled,
    /// Capability renders only under an explicit, sandboxed opt-in.
    SandboxedOptIn,
    /// Content of this kind was present and was blocked from rendering.
    Blocked,
    /// Capability does not apply in the current mode (for example source mode).
    NotApplicable,
}

impl RenderCapability {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::SandboxedOptIn => "sandboxed_opt_in",
            Self::Blocked => "blocked",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Returns true when the capability actively renders content (always sandboxed).
    pub const fn is_active(self) -> bool {
        matches!(self, Self::SandboxedOptIn)
    }
}

/// Diagram, math, and custom-component capability posture for a rendered preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRenderCapabilities {
    /// Diagram-engine posture (for example fenced diagram blocks).
    pub diagrams: RenderCapability,
    /// Math-rendering posture (for example inline or block math).
    pub math: RenderCapability,
    /// Custom-component / embed posture beyond the CommonMark + extension baseline.
    pub custom_components: RenderCapability,
}

impl WorkspaceRenderCapabilities {
    /// Every capability, for iteration in validation and rendering.
    pub const fn all(&self) -> [RenderCapability; 3] {
        [self.diagrams, self.math, self.custom_components]
    }

    /// All capabilities not applicable — the correct posture for source mode.
    pub const fn all_not_applicable() -> Self {
        Self {
            diagrams: RenderCapability::NotApplicable,
            math: RenderCapability::NotApplicable,
            custom_components: RenderCapability::NotApplicable,
        }
    }
}

/// Kind of initiating anchor a workspace was opened from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceAnchorKind {
    /// Opened from a code symbol (function, type, or module).
    CodeSymbol,
    /// Opened from a code file or path.
    CodeFile,
    /// Opened from a section of another document.
    DocSection,
    /// Opened from a review thread or comment.
    ReviewThread,
    /// Opened from a release note or changelog entry.
    ReleaseNote,
    /// Opened from a search or recall result.
    SearchResult,
}

impl WorkspaceAnchorKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeSymbol => "code_symbol",
            Self::CodeFile => "code_file",
            Self::DocSection => "doc_section",
            Self::ReviewThread => "review_thread",
            Self::ReleaseNote => "release_note",
            Self::SearchResult => "search_result",
        }
    }
}

/// Initiating code/doc anchor context preserved across mode switches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceAnchor {
    /// Kind of anchor the workspace was opened from.
    pub anchor_kind: WorkspaceAnchorKind,
    /// Stable anchor ref (symbol id, path, or anchor id — never a raw body).
    pub anchor_ref: String,
    /// User-facing anchor label.
    pub anchor_label: String,
    /// Whether the anchor context is preserved across source/split/rendered switches.
    pub preserved_across_modes: bool,
}

impl WorkspaceAnchor {
    fn is_well_formed(&self) -> bool {
        !self.anchor_ref.trim().is_empty()
            && !self.anchor_label.trim().is_empty()
            && self.preserved_across_modes
    }
}

/// Availability of a scoped handoff to the browser companion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserHandoffAvailability {
    /// A scoped, disclosed handoff to the browser companion is available.
    Available,
    /// Handoff is unavailable because the source is offline or mirror-pinned.
    UnavailableOffline,
    /// Handoff is blocked by policy.
    BlockedPolicy,
    /// No external handoff target applies to this workspace.
    NotApplicable,
}

impl BrowserHandoffAvailability {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::UnavailableOffline => "unavailable_offline",
            Self::BlockedPolicy => "blocked_policy",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Returns true only when a handoff action may be offered.
    pub const fn offers_handoff(self) -> bool {
        matches!(self, Self::Available)
    }
}

/// A stable, keyboard-reachable command that switches the workspace mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceModeCommand {
    /// Stable command id used by the palette, keymap, and exports.
    pub command_id: String,
    /// Mode this command switches the workspace into.
    pub target_mode: DocsPreviewMode,
    /// Default key binding (a display token such as `Mod+Alt+1`).
    pub key_binding: String,
    /// Whether the command is reachable without a pointer.
    pub keyboard_reachable: bool,
    /// User-facing command label.
    pub label: String,
}

impl WorkspaceModeCommand {
    fn is_well_formed(&self) -> bool {
        !self.command_id.trim().is_empty()
            && !self.key_binding.trim().is_empty()
            && !self.label.trim().is_empty()
            && self.keyboard_reachable
    }
}

/// The dedicated recovery command that always returns the workspace to raw source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRecoveryCommand {
    /// Stable command id used by the palette, keymap, and exports.
    pub command_id: String,
    /// Default key binding (a display token such as `Escape`).
    pub key_binding: String,
    /// Whether the command is reachable without a pointer.
    pub keyboard_reachable: bool,
    /// User-facing command label.
    pub label: String,
    /// Mode this command reverts to; must be [`DocsPreviewMode::Source`].
    pub reverts_to_mode: DocsPreviewMode,
}

impl WorkspaceRecoveryCommand {
    fn is_well_formed(&self) -> bool {
        !self.command_id.trim().is_empty()
            && !self.key_binding.trim().is_empty()
            && !self.label.trim().is_empty()
            && self.keyboard_reachable
            && self.reverts_to_mode == DocsPreviewMode::Source
    }
}

/// Constructor input for [`MarkdownAuthoringWorkspace::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarkdownAuthoringWorkspaceInput {
    /// Stable workspace id.
    pub workspace_id: String,
    /// Human-readable workspace label.
    pub workspace_label: String,
    /// Artifact family being authored.
    pub artifact_kind: DocsArtifactKind,
    /// Stable artifact ref (path or artifact id, never a raw body).
    pub artifact_ref: String,
    /// Active workspace mode.
    pub active_mode: DocsPreviewMode,
    /// Remembered mode preference restored on reopen.
    pub remembered_mode_preference: DocsPreviewMode,
    /// Mode-switch commands. Must cover source, split, and rendered.
    pub mode_commands: Vec<WorkspaceModeCommand>,
    /// Always-available recovery back to raw source.
    pub recover_to_source_command: WorkspaceRecoveryCommand,
    /// Whether CommonMark is the parsing baseline (must be true).
    pub commonmark_baseline: bool,
    /// Disclosure of the CommonMark baseline and enabled extensions.
    pub commonmark_baseline_note: String,
    /// Declared extensions enabled beyond the CommonMark baseline.
    pub enabled_extensions: Vec<String>,
    /// Extensions the renderer actually activated. Must be a subset of
    /// `enabled_extensions`; an undeclared active extension is a hidden extension.
    pub active_extensions: Vec<String>,
    /// HTML sanitization posture for rendered content.
    pub sanitization_state: DocsPreviewSanitizationState,
    /// Disclosure note for the sanitization posture when required.
    pub sanitization_note: Option<String>,
    /// Diagram/math/custom-component capability posture.
    pub render_capabilities: WorkspaceRenderCapabilities,
    /// Docs source/version/freshness badge.
    pub source_version_badge: DocsSourceVersionBadge,
    /// Mirror, cache, or offline posture for the source.
    pub mirror_offline_state: DocsMirrorOfflinePosture,
    /// Local-only versus publish-boundary posture.
    pub publish_boundary_state: DocsPublishBoundaryState,
    /// Browser-handoff availability.
    pub browser_handoff_availability: BrowserHandoffAvailability,
    /// Open-source / switch-to-source action (always keyboard reachable).
    pub open_source_action: DocsMaintenanceAction,
    /// Open-in-browser handoff action, present only when handoff is available.
    pub open_browser_action: Option<DocsMaintenanceAction>,
    /// Initiating code/doc anchor context, if the workspace was opened from one.
    pub anchor_context: Option<WorkspaceAnchor>,
    /// Disclosure that a rendered view is not canonical source or proof.
    pub rendered_is_not_canonical_note: String,
    /// Surface refs that render this workspace.
    pub surface_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe Markdown authoring workspace truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkdownAuthoringWorkspace {
    /// Record kind; must equal [`MARKDOWN_WORKSPACE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MARKDOWN_WORKSPACE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable workspace id.
    pub workspace_id: String,
    /// Human-readable workspace label.
    pub workspace_label: String,
    /// Artifact family being authored.
    pub artifact_kind: DocsArtifactKind,
    /// Stable artifact ref (path or artifact id, never a raw body).
    pub artifact_ref: String,
    /// Active workspace mode.
    pub active_mode: DocsPreviewMode,
    /// Remembered mode preference restored on reopen.
    pub remembered_mode_preference: DocsPreviewMode,
    /// Mode-switch commands.
    pub mode_commands: Vec<WorkspaceModeCommand>,
    /// Always-available recovery back to raw source.
    pub recover_to_source_command: WorkspaceRecoveryCommand,
    /// Whether CommonMark is the parsing baseline.
    pub commonmark_baseline: bool,
    /// Disclosure of the CommonMark baseline and enabled extensions.
    pub commonmark_baseline_note: String,
    /// Declared extensions enabled beyond the CommonMark baseline.
    pub enabled_extensions: Vec<String>,
    /// Extensions the renderer actually activated.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub active_extensions: Vec<String>,
    /// HTML sanitization posture for rendered content.
    pub sanitization_state: DocsPreviewSanitizationState,
    /// Disclosure note for the sanitization posture when required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sanitization_note: Option<String>,
    /// Diagram/math/custom-component capability posture.
    pub render_capabilities: WorkspaceRenderCapabilities,
    /// Docs source/version/freshness badge.
    pub source_version_badge: DocsSourceVersionBadge,
    /// Mirror, cache, or offline posture for the source.
    pub mirror_offline_state: DocsMirrorOfflinePosture,
    /// Local-only versus publish-boundary posture.
    pub publish_boundary_state: DocsPublishBoundaryState,
    /// Browser-handoff availability.
    pub browser_handoff_availability: BrowserHandoffAvailability,
    /// Open-source / switch-to-source action.
    pub open_source_action: DocsMaintenanceAction,
    /// Open-in-browser handoff action, present only when handoff is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_browser_action: Option<DocsMaintenanceAction>,
    /// Initiating code/doc anchor context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor_context: Option<WorkspaceAnchor>,
    /// Disclosure that a rendered view is not canonical source or proof.
    pub rendered_is_not_canonical_note: String,
    /// Surface refs that render this workspace.
    pub surface_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl MarkdownAuthoringWorkspace {
    /// Builds a Markdown authoring workspace packet from constructor input.
    pub fn new(input: MarkdownAuthoringWorkspaceInput) -> Self {
        Self {
            record_kind: MARKDOWN_WORKSPACE_RECORD_KIND.to_owned(),
            schema_version: MARKDOWN_WORKSPACE_SCHEMA_VERSION,
            workspace_id: input.workspace_id,
            workspace_label: input.workspace_label,
            artifact_kind: input.artifact_kind,
            artifact_ref: input.artifact_ref,
            active_mode: input.active_mode,
            remembered_mode_preference: input.remembered_mode_preference,
            mode_commands: input.mode_commands,
            recover_to_source_command: input.recover_to_source_command,
            commonmark_baseline: input.commonmark_baseline,
            commonmark_baseline_note: input.commonmark_baseline_note,
            enabled_extensions: input.enabled_extensions,
            active_extensions: input.active_extensions,
            sanitization_state: input.sanitization_state,
            sanitization_note: input.sanitization_note,
            render_capabilities: input.render_capabilities,
            source_version_badge: input.source_version_badge,
            mirror_offline_state: input.mirror_offline_state,
            publish_boundary_state: input.publish_boundary_state,
            browser_handoff_availability: input.browser_handoff_availability,
            open_source_action: input.open_source_action,
            open_browser_action: input.open_browser_action,
            anchor_context: input.anchor_context,
            rendered_is_not_canonical_note: input.rendered_is_not_canonical_note,
            surface_refs: input.surface_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Returns true when the active mode renders a preview (split or rendered).
    pub fn renders_preview(&self) -> bool {
        self.active_mode.renders_preview()
    }

    /// Validates the workspace truth invariants.
    pub fn validate(&self) -> Vec<MarkdownWorkspaceViolation> {
        let mut violations = Vec::new();

        if self.record_kind != MARKDOWN_WORKSPACE_RECORD_KIND {
            violations.push(MarkdownWorkspaceViolation::WrongRecordKind);
        }
        if self.schema_version != MARKDOWN_WORKSPACE_SCHEMA_VERSION {
            violations.push(MarkdownWorkspaceViolation::WrongSchemaVersion);
        }
        if self.workspace_id.trim().is_empty()
            || self.workspace_label.trim().is_empty()
            || self.artifact_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.surface_refs.is_empty()
        {
            violations.push(MarkdownWorkspaceViolation::MissingIdentity);
        }

        validate_commonmark_baseline(self, &mut violations);
        validate_mode_commands(self, &mut violations);
        validate_remembered_preference(self, &mut violations);
        validate_extensions(self, &mut violations);
        validate_preview_safety(self, &mut violations);
        validate_source_version_badge(self, &mut violations);
        validate_open_actions(self, &mut violations);
        validate_anchor(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("markdown authoring workspace serializes"),
        ) {
            violations.push(MarkdownWorkspaceViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Returns true when the workspace validates with no violations.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("markdown authoring workspace serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Markdown Authoring Workspace\n\n");
        out.push_str(&format!("- Workspace: `{}`\n", self.workspace_id));
        out.push_str(&format!("- Label: `{}`\n", self.workspace_label));
        out.push_str(&format!("- Artifact: `{}`\n", self.artifact_ref));
        out.push_str(&format!(
            "- Active mode: `{}` (remembered: `{}`)\n",
            self.active_mode.as_str(),
            self.remembered_mode_preference.as_str()
        ));
        out.push_str(&format!(
            "- Sanitization: `{}`\n",
            self.sanitization_state.as_str()
        ));
        out.push_str(&format!(
            "- Source: `{}` / freshness `{}` / version `{}`\n",
            self.source_version_badge.source_class_token,
            self.source_version_badge.freshness_class.as_str(),
            self.source_version_badge.version_match_state.as_str()
        ));
        out.push_str(&format!(
            "- Mirror/offline: `{}`\n",
            self.mirror_offline_state.as_str()
        ));
        out.push_str(&format!(
            "- Browser handoff: `{}`\n",
            self.browser_handoff_availability.as_str()
        ));
        out.push_str("\n## Render capabilities\n\n");
        out.push_str(&format!(
            "- Diagrams: `{}`\n",
            self.render_capabilities.diagrams.as_str()
        ));
        out.push_str(&format!(
            "- Math: `{}`\n",
            self.render_capabilities.math.as_str()
        ));
        out.push_str(&format!(
            "- Custom components: `{}`\n",
            self.render_capabilities.custom_components.as_str()
        ));
        out.push_str("\n## Mode commands\n\n");
        for command in &self.mode_commands {
            out.push_str(&format!(
                "- `{}` → `{}` ({})\n",
                command.command_id,
                command.target_mode.as_str(),
                command.key_binding
            ));
        }
        out.push_str(&format!(
            "- `{}` → recover to source ({})\n",
            self.recover_to_source_command.command_id, self.recover_to_source_command.key_binding
        ));
        out
    }
}

/// Errors emitted when reading the checked-in Markdown authoring workspace export.
#[derive(Debug)]
pub enum MarkdownWorkspaceArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<MarkdownWorkspaceViolation>),
}

impl fmt::Display for MarkdownWorkspaceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "markdown workspace export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "markdown workspace export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for MarkdownWorkspaceArtifactError {}

/// Validation failures emitted by [`MarkdownAuthoringWorkspace::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MarkdownWorkspaceViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// CommonMark baseline is not declared or its disclosure note is empty.
    CommonMarkBaselineMissing,
    /// Mode commands do not cover source/split/rendered, or one is malformed.
    ModeCommandsIncomplete,
    /// The recovery command does not return to raw source or is malformed.
    RecoveryCommandInvalid,
    /// The remembered preference or active mode has no mode command.
    RememberedModeUnsupported,
    /// An active extension was not declared in the enabled set.
    HiddenExtensionActive,
    /// Source mode renders content (non-`not_applicable` sanitization or capabilities).
    SourceModeRendersContent,
    /// A rendering mode declares a non-concrete preview-safety posture.
    UnsafePreviewDefault,
    /// A rendering mode does not disclose that rendered output is not canonical source.
    RenderedNotDisclosedAsNonCanonical,
    /// Allowed raw HTML is missing its disclosure note.
    SanitizationNoteMissing,
    /// The source/version/freshness badge is incomplete.
    SourceVersionBadgeIncomplete,
    /// The open-source action is missing or not keyboard reachable.
    OpenSourceActionInvalid,
    /// The browser-handoff action does not match the declared availability.
    BrowserHandoffActionMismatch,
    /// The anchor context is present but malformed.
    AnchorContextInvalid,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl MarkdownWorkspaceViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::CommonMarkBaselineMissing => "commonmark_baseline_missing",
            Self::ModeCommandsIncomplete => "mode_commands_incomplete",
            Self::RecoveryCommandInvalid => "recovery_command_invalid",
            Self::RememberedModeUnsupported => "remembered_mode_unsupported",
            Self::HiddenExtensionActive => "hidden_extension_active",
            Self::SourceModeRendersContent => "source_mode_renders_content",
            Self::UnsafePreviewDefault => "unsafe_preview_default",
            Self::RenderedNotDisclosedAsNonCanonical => "rendered_not_disclosed_as_non_canonical",
            Self::SanitizationNoteMissing => "sanitization_note_missing",
            Self::SourceVersionBadgeIncomplete => "source_version_badge_incomplete",
            Self::OpenSourceActionInvalid => "open_source_action_invalid",
            Self::BrowserHandoffActionMismatch => "browser_handoff_action_mismatch",
            Self::AnchorContextInvalid => "anchor_context_invalid",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable Markdown authoring workspace export.
pub fn current_stable_markdown_authoring_workspace_export(
) -> Result<MarkdownAuthoringWorkspace, MarkdownWorkspaceArtifactError> {
    let packet: MarkdownAuthoringWorkspace = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/markdown-workspace-proof/support_export.json"
    )))
    .map_err(MarkdownWorkspaceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(MarkdownWorkspaceArtifactError::Validation(violations))
    }
}

/// Returns the canonical seeded Split-mode workspace used as the support export.
pub fn seeded_stable_markdown_authoring_workspace() -> MarkdownAuthoringWorkspace {
    MarkdownAuthoringWorkspace::new(seeded_stable_markdown_authoring_workspace_input())
}

/// Returns the constructor input for [`seeded_stable_markdown_authoring_workspace`].
pub fn seeded_stable_markdown_authoring_workspace_input() -> MarkdownAuthoringWorkspaceInput {
    MarkdownAuthoringWorkspaceInput {
        workspace_id: "docs-workspace:readme:split:0001".to_owned(),
        workspace_label: "README authoring workspace".to_owned(),
        artifact_kind: DocsArtifactKind::Readme,
        artifact_ref: "README.md".to_owned(),
        active_mode: DocsPreviewMode::Split,
        remembered_mode_preference: DocsPreviewMode::Split,
        mode_commands: seeded_mode_commands(),
        recover_to_source_command: seeded_recovery_command(),
        commonmark_baseline: true,
        commonmark_baseline_note:
            "CommonMark is the parsing baseline. GitHub-flavored tables, task lists, and fenced \
             code are the only enabled extensions; everything else renders as inert source text."
                .to_owned(),
        enabled_extensions: vec![
            "gfm_tables".to_owned(),
            "task_lists".to_owned(),
            "fenced_code".to_owned(),
        ],
        active_extensions: vec!["gfm_tables".to_owned(), "fenced_code".to_owned()],
        sanitization_state: DocsPreviewSanitizationState::SanitizedSafe,
        sanitization_note: None,
        render_capabilities: WorkspaceRenderCapabilities {
            diagrams: RenderCapability::SandboxedOptIn,
            math: RenderCapability::SandboxedOptIn,
            custom_components: RenderCapability::Disabled,
        },
        source_version_badge: DocsSourceVersionBadge {
            source_class_token: "project_docs".to_owned(),
            source_pack_ref: "project:readme".to_owned(),
            source_revision_ref: "rev:7131d437".to_owned(),
            version_or_revision_ref: "0.1.0-dev".to_owned(),
            source_build_at: "2026-06-18T00:00:00Z".to_owned(),
            running_build_identity_ref: "build:0.1.0-dev".to_owned(),
            freshness_class: DocsFreshnessClass::WarmCached,
            version_match_state: VersionMatchState::ExactBuildMatch,
        },
        mirror_offline_state: DocsMirrorOfflinePosture::LocalProjectPack,
        publish_boundary_state: DocsPublishBoundaryState::LocalOnly,
        browser_handoff_availability: BrowserHandoffAvailability::Available,
        open_source_action: DocsMaintenanceAction::new(RECOVER_SOURCE_COMMAND_ID, "Open source"),
        open_browser_action: Some(DocsMaintenanceAction::new(
            "docs.authoring.open_browser",
            "Open in browser",
        )),
        anchor_context: Some(WorkspaceAnchor {
            anchor_kind: WorkspaceAnchorKind::CodeSymbol,
            anchor_ref: "symbol:aureline_docs::current_stable_markdown_authoring_workspace_export"
                .to_owned(),
            anchor_label: "current_stable_markdown_authoring_workspace_export".to_owned(),
            preserved_across_modes: true,
        }),
        rendered_is_not_canonical_note:
            "Rendered preview is a safe, sanitized view of the source. It is not canonical source \
             or proof; the raw Markdown remains the source of truth."
                .to_owned(),
        surface_refs: vec!["authoring_workspace".to_owned(), "preview_pane".to_owned()],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-18T00:00:00Z".to_owned(),
    }
}

/// Returns the canonical source/split/rendered mode commands with keyboard parity.
pub fn seeded_mode_commands() -> Vec<WorkspaceModeCommand> {
    vec![
        WorkspaceModeCommand {
            command_id: MODE_SOURCE_COMMAND_ID.to_owned(),
            target_mode: DocsPreviewMode::Source,
            key_binding: "Mod+Alt+1".to_owned(),
            keyboard_reachable: true,
            label: "Source mode".to_owned(),
        },
        WorkspaceModeCommand {
            command_id: MODE_SPLIT_COMMAND_ID.to_owned(),
            target_mode: DocsPreviewMode::Split,
            key_binding: "Mod+Alt+2".to_owned(),
            keyboard_reachable: true,
            label: "Split mode".to_owned(),
        },
        WorkspaceModeCommand {
            command_id: MODE_RENDERED_COMMAND_ID.to_owned(),
            target_mode: DocsPreviewMode::Rendered,
            key_binding: "Mod+Alt+3".to_owned(),
            keyboard_reachable: true,
            label: "Rendered mode".to_owned(),
        },
    ]
}

/// Returns the canonical recovery-to-source command.
pub fn seeded_recovery_command() -> WorkspaceRecoveryCommand {
    WorkspaceRecoveryCommand {
        command_id: RECOVER_SOURCE_COMMAND_ID.to_owned(),
        key_binding: "Escape".to_owned(),
        keyboard_reachable: true,
        label: "Recover raw source".to_owned(),
        reverts_to_mode: DocsPreviewMode::Source,
    }
}

fn validate_commonmark_baseline(
    packet: &MarkdownAuthoringWorkspace,
    violations: &mut Vec<MarkdownWorkspaceViolation>,
) {
    if !packet.commonmark_baseline || packet.commonmark_baseline_note.trim().is_empty() {
        violations.push(MarkdownWorkspaceViolation::CommonMarkBaselineMissing);
    }
}

fn validate_mode_commands(
    packet: &MarkdownAuthoringWorkspace,
    violations: &mut Vec<MarkdownWorkspaceViolation>,
) {
    let mut command_ids = BTreeSet::new();
    let mut covered_modes = BTreeSet::new();
    let mut all_well_formed = true;
    for command in &packet.mode_commands {
        if !command.is_well_formed() {
            all_well_formed = false;
        }
        command_ids.insert(command.command_id.clone());
        covered_modes.insert(command.target_mode);
    }
    let covers_all_modes = [
        DocsPreviewMode::Source,
        DocsPreviewMode::Split,
        DocsPreviewMode::Rendered,
    ]
    .iter()
    .all(|mode| covered_modes.contains(mode));
    let ids_unique = command_ids.len() == packet.mode_commands.len();
    if !covers_all_modes || !all_well_formed || !ids_unique {
        violations.push(MarkdownWorkspaceViolation::ModeCommandsIncomplete);
    }

    if !packet.recover_to_source_command.is_well_formed() {
        violations.push(MarkdownWorkspaceViolation::RecoveryCommandInvalid);
    }
}

fn validate_remembered_preference(
    packet: &MarkdownAuthoringWorkspace,
    violations: &mut Vec<MarkdownWorkspaceViolation>,
) {
    let covered: BTreeSet<DocsPreviewMode> = packet
        .mode_commands
        .iter()
        .map(|command| command.target_mode)
        .collect();
    if !covered.contains(&packet.remembered_mode_preference)
        || !covered.contains(&packet.active_mode)
    {
        violations.push(MarkdownWorkspaceViolation::RememberedModeUnsupported);
    }
}

fn validate_extensions(
    packet: &MarkdownAuthoringWorkspace,
    violations: &mut Vec<MarkdownWorkspaceViolation>,
) {
    let declared: BTreeSet<&str> = packet
        .enabled_extensions
        .iter()
        .map(String::as_str)
        .collect();
    if packet
        .active_extensions
        .iter()
        .any(|active| !declared.contains(active.as_str()))
    {
        violations.push(MarkdownWorkspaceViolation::HiddenExtensionActive);
    }
}

fn validate_preview_safety(
    packet: &MarkdownAuthoringWorkspace,
    violations: &mut Vec<MarkdownWorkspaceViolation>,
) {
    let renders = packet.renders_preview();
    let capabilities_all_not_applicable = packet
        .render_capabilities
        .all()
        .iter()
        .all(|capability| matches!(capability, RenderCapability::NotApplicable));

    if !renders {
        // Source mode renders nothing: sanitization and capabilities must be inert.
        if packet.sanitization_state != DocsPreviewSanitizationState::NotApplicable
            || !capabilities_all_not_applicable
        {
            violations.push(MarkdownWorkspaceViolation::SourceModeRendersContent);
        }
        return;
    }

    // A rendering mode must carry a concrete, safe-by-default sanitization posture.
    if packet.sanitization_state == DocsPreviewSanitizationState::NotApplicable {
        violations.push(MarkdownWorkspaceViolation::UnsafePreviewDefault);
    }
    // A rendering mode must never let render capabilities read as not-applicable.
    if capabilities_all_not_applicable {
        violations.push(MarkdownWorkspaceViolation::UnsafePreviewDefault);
    }
    // A rendering mode must disclose that rendered output is not canonical source.
    if packet.rendered_is_not_canonical_note.trim().is_empty() {
        violations.push(MarkdownWorkspaceViolation::RenderedNotDisclosedAsNonCanonical);
    }
    // Allowed raw HTML must carry a disclosure note.
    if packet.sanitization_state.requires_disclosure()
        && packet
            .sanitization_note
            .as_deref()
            .map_or(true, |note| note.trim().is_empty())
    {
        violations.push(MarkdownWorkspaceViolation::SanitizationNoteMissing);
    }
}

fn validate_source_version_badge(
    packet: &MarkdownAuthoringWorkspace,
    violations: &mut Vec<MarkdownWorkspaceViolation>,
) {
    let badge = &packet.source_version_badge;
    let well_formed = ![
        &badge.source_class_token,
        &badge.source_pack_ref,
        &badge.source_revision_ref,
        &badge.version_or_revision_ref,
        &badge.source_build_at,
        &badge.running_build_identity_ref,
    ]
    .iter()
    .any(|value| value.trim().is_empty());
    if !well_formed {
        violations.push(MarkdownWorkspaceViolation::SourceVersionBadgeIncomplete);
    }
}

fn validate_open_actions(
    packet: &MarkdownAuthoringWorkspace,
    violations: &mut Vec<MarkdownWorkspaceViolation>,
) {
    if !action_is_well_formed(&packet.open_source_action) {
        violations.push(MarkdownWorkspaceViolation::OpenSourceActionInvalid);
    }

    match (
        packet.browser_handoff_availability.offers_handoff(),
        &packet.open_browser_action,
    ) {
        (true, Some(action)) if action_is_well_formed(action) => {}
        (false, None) => {}
        _ => violations.push(MarkdownWorkspaceViolation::BrowserHandoffActionMismatch),
    }
}

fn validate_anchor(
    packet: &MarkdownAuthoringWorkspace,
    violations: &mut Vec<MarkdownWorkspaceViolation>,
) {
    if let Some(anchor) = &packet.anchor_context {
        if !anchor.is_well_formed() {
            violations.push(MarkdownWorkspaceViolation::AnchorContextInvalid);
        }
    }
}

fn action_is_well_formed(action: &DocsMaintenanceAction) -> bool {
    !action.action_ref.trim().is_empty()
        && !action.action_label.trim().is_empty()
        && action.keyboard_reachable
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
