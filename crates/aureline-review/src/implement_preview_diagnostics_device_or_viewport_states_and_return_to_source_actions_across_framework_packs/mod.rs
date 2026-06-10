//! Preview diagnostics, device / viewport states, and return-to-source actions across framework packs.
//!
//! This module implements the canonical M5 truth packet for the in-product
//! preview-diagnostics surface: the lane that surfaces a build, compile,
//! runtime, or hot-reload diagnostic raised while previewing a reviewed change
//! without ever hiding which framework pack produced it, which device /
//! viewport state the view was rendered in, or how fresh and how exact the
//! source mapping behind a return-to-source jump is. It binds four pillars into
//! one export-safe record:
//!
//! - **Framework packs** — each [`FrameworkPackRow`] names a convenience
//!   framework integration (React, Vue, Svelte, Angular, static HTML, a native
//!   runtime, a generic pack, or a provider-owned unknown) and discloses whether
//!   it supports source mapping, viewport emulation, and hot reload, so a
//!   diagnostic can never claim a capability the pack does not have.
//! - **Preview diagnostics** — each [`PreviewDiagnosticRow`] carries its durable
//!   review anchor, the framework pack it came from, a typed severity and kind, a
//!   redaction-aware message label, and an attention block, so a diagnostic is
//!   always anchored, attributable, and honest about its severity.
//! - **Device / viewport states** — each diagnostic carries a
//!   [`DeviceViewportState`] with a typed viewport class, a device label, a
//!   dimensions label, an emulation flag, and a disclosure flag, so a preview can
//!   never hide which device or viewport the diagnostic was captured in.
//! - **Return-to-source actions** — each diagnostic carries a
//!   [`SourceMappingDisclosure`] (mapping exactness and freshness) and a
//!   [`ReturnToSourceAction`] (a typed action kind, a read-only flag, and a
//!   handoff ref when the action leaves the product), so jumping from a preview
//!   back to source is read-only navigation unless an attributable handoff is
//!   cited, and a stale or missing source map narrows the action rather than
//!   jumping to the wrong place.
//!
//! The packet references upstream preview-target, device-target, hot-reload, and
//! trust-class contracts by id rather than embedding their content. Raw preview
//! URLs, raw host names, raw diagnostic stacks, raw source bodies, raw provider
//! payloads, raw absolute paths, raw author email addresses, credentials, and
//! live provider responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/review/implement-preview-diagnostics-device-or-viewport-states-and-return-to-source-actions-across-framework-packs.schema.json`](../../../../schemas/review/implement-preview-diagnostics-device-or-viewport-states-and-return-to-source-actions-across-framework-packs.schema.json).
//! The contract doc is
//! [`docs/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs.md`](../../../../docs/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs/`](../../../../fixtures/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`PreviewDiagnosticsPacket`].
pub const PREVIEW_DIAGNOSTICS_RECORD_KIND: &str =
    "implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs";

/// Schema version for preview diagnostics records.
pub const PREVIEW_DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PREVIEW_DIAGNOSTICS_SCHEMA_REF: &str =
    "schemas/review/implement-preview-diagnostics-device-or-viewport-states-and-return-to-source-actions-across-framework-packs.schema.json";

/// Repo-relative path of the preview diagnostics contract doc.
pub const PREVIEW_DIAGNOSTICS_DOC_REF: &str =
    "docs/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs.md";

/// Repo-relative path of the preview-target descriptor contract this lane builds on.
pub const PREVIEW_DIAGNOSTICS_PREVIEW_TARGET_CONTRACT_REF: &str =
    "schemas/preview/preview_target_descriptor.schema.json";

/// Repo-relative path of the device-target descriptor contract the viewport state binds to.
pub const PREVIEW_DIAGNOSTICS_DEVICE_TARGET_CONTRACT_REF: &str =
    "schemas/preview/device_target_descriptor.schema.json";

/// Repo-relative path of the hot-reload state contract diagnostics reference.
pub const PREVIEW_DIAGNOSTICS_HOT_RELOAD_CONTRACT_REF: &str =
    "schemas/preview/hot_reload_state.schema.json";

/// Repo-relative path of the trust-class vocabulary this lane reuses.
pub const PREVIEW_DIAGNOSTICS_TRUST_CLASS_CONTRACT_REF: &str =
    "schemas/security/trust_class.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const PREVIEW_DIAGNOSTICS_FIXTURE_DIR: &str =
    "fixtures/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs";

/// Repo-relative path of the checked support-export artifact.
pub const PREVIEW_DIAGNOSTICS_ARTIFACT_REF: &str =
    "artifacts/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PREVIEW_DIAGNOSTICS_SUMMARY_REF: &str =
    "artifacts/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs.md";

/// Framework pack a preview diagnostic originated from.
///
/// `unknown_pack_provider_owned` must never be flattened into a known pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkPackClass {
    /// A React web framework pack.
    WebReact,
    /// A Vue web framework pack.
    WebVue,
    /// A Svelte web framework pack.
    WebSvelte,
    /// An Angular web framework pack.
    WebAngular,
    /// A static-HTML web pack with no framework runtime.
    WebStaticHtml,
    /// A native application runtime pack.
    NativeRuntime,
    /// A generic framework pack not specialised further.
    GenericFrameworkPack,
    /// Provider returned a pack the contract does not recognise yet.
    UnknownPackProviderOwned,
}

impl FrameworkPackClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WebReact => "web_react",
            Self::WebVue => "web_vue",
            Self::WebSvelte => "web_svelte",
            Self::WebAngular => "web_angular",
            Self::WebStaticHtml => "web_static_html",
            Self::NativeRuntime => "native_runtime",
            Self::GenericFrameworkPack => "generic_framework_pack",
            Self::UnknownPackProviderOwned => "unknown_pack_provider_owned",
        }
    }

    /// Whether this pack class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(self, Self::UnknownPackProviderOwned)
    }
}

/// Typed severity of a preview diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    /// Informational diagnostic.
    Info,
    /// Warning diagnostic.
    Warning,
    /// Error diagnostic.
    Error,
    /// Fatal diagnostic that halts the preview.
    Fatal,
    /// Provider returned a severity the contract does not recognise yet.
    UnknownSeverityProviderOwned,
}

impl DiagnosticSeverity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Fatal => "fatal",
            Self::UnknownSeverityProviderOwned => "unknown_severity_provider_owned",
        }
    }

    /// Whether this severity needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::Error | Self::Fatal | Self::UnknownSeverityProviderOwned
        )
    }
}

/// Typed kind of a preview diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticKind {
    /// A build-time failure.
    BuildError,
    /// A compile-time failure.
    CompileError,
    /// A runtime failure in the preview.
    RuntimeError,
    /// A hot-reload failure.
    HotReloadFailure,
    /// A type-checking failure.
    TypeError,
    /// A lint warning.
    LintWarning,
    /// A console error raised by the preview runtime.
    ConsoleError,
    /// Provider returned a kind the contract does not recognise yet.
    UnknownDiagnosticProviderOwned,
}

impl DiagnosticKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildError => "build_error",
            Self::CompileError => "compile_error",
            Self::RuntimeError => "runtime_error",
            Self::HotReloadFailure => "hot_reload_failure",
            Self::TypeError => "type_error",
            Self::LintWarning => "lint_warning",
            Self::ConsoleError => "console_error",
            Self::UnknownDiagnosticProviderOwned => "unknown_diagnostic_provider_owned",
        }
    }

    /// Whether this kind needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(self, Self::UnknownDiagnosticProviderOwned)
    }
}

/// Device / viewport class the preview was rendered in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewportClass {
    /// A desktop viewport.
    Desktop,
    /// A tablet viewport.
    Tablet,
    /// A mobile viewport.
    Mobile,
    /// A fluid responsive viewport.
    ResponsiveFluid,
    /// A custom, named viewport preset.
    CustomViewport,
    /// An emulated physical device.
    DeviceEmulation,
    /// Provider returned a viewport the contract does not recognise yet.
    UnknownViewportProviderOwned,
}

impl ViewportClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Tablet => "tablet",
            Self::Mobile => "mobile",
            Self::ResponsiveFluid => "responsive_fluid",
            Self::CustomViewport => "custom_viewport",
            Self::DeviceEmulation => "device_emulation",
            Self::UnknownViewportProviderOwned => "unknown_viewport_provider_owned",
        }
    }

    /// Whether this viewport class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(self, Self::UnknownViewportProviderOwned)
    }
}

/// Exactness of the source mapping that powers a return-to-source jump.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceMappingClass {
    /// The mapping resolves to an exact line and column.
    ExactLineColumn,
    /// The mapping resolves to an approximate line.
    ApproximateLine,
    /// The mapping resolves to a file only.
    FileOnly,
    /// The content is generated and carries no source map.
    GeneratedNoSourceMap,
    /// Provider returned a mapping the contract does not recognise yet.
    UnknownMappingProviderOwned,
}

impl SourceMappingClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactLineColumn => "exact_line_column",
            Self::ApproximateLine => "approximate_line",
            Self::FileOnly => "file_only",
            Self::GeneratedNoSourceMap => "generated_no_source_map",
            Self::UnknownMappingProviderOwned => "unknown_mapping_provider_owned",
        }
    }

    /// Whether the mapping can resolve a return-to-source target at all.
    pub const fn can_return_to_source(self) -> bool {
        matches!(
            self,
            Self::ExactLineColumn | Self::ApproximateLine | Self::FileOnly
        )
    }

    /// Whether this mapping class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::GeneratedNoSourceMap | Self::UnknownMappingProviderOwned
        )
    }
}

/// Freshness of the source mapping relative to the current preview build.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceMappingFreshness {
    /// The mapping was produced by the current build.
    FreshCurrentBuild,
    /// The mapping was produced by a prior build and may be stale.
    StalePriorBuild,
    /// Provider returned a freshness the contract does not recognise yet.
    UnknownFreshnessProviderOwned,
}

impl SourceMappingFreshness {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshCurrentBuild => "fresh_current_build",
            Self::StalePriorBuild => "stale_prior_build",
            Self::UnknownFreshnessProviderOwned => "unknown_freshness_provider_owned",
        }
    }

    /// Whether the mapping is fresh against the current build.
    pub const fn is_fresh(self) -> bool {
        matches!(self, Self::FreshCurrentBuild)
    }

    /// Whether this freshness needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        matches!(
            self,
            Self::StalePriorBuild | Self::UnknownFreshnessProviderOwned
        )
    }
}

/// Typed return-to-source action kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnToSourceActionKind {
    /// Jump to the source location inside the product; read-only navigation.
    JumpToSourceLocal,
    /// Reveal the source location in the editor; read-only navigation.
    RevealInEditor,
    /// Copy the source location to the clipboard; read-only navigation.
    CopySourceLocation,
    /// Hand off to the browser to open the source; leaves the product and must be attributed.
    OpenInBrowserHandoff,
    /// No source map is available, so no return-to-source action is offered.
    UnsupportedNoSourceMap,
}

impl ReturnToSourceActionKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JumpToSourceLocal => "jump_to_source_local",
            Self::RevealInEditor => "reveal_in_editor",
            Self::CopySourceLocation => "copy_source_location",
            Self::OpenInBrowserHandoff => "open_in_browser_handoff",
            Self::UnsupportedNoSourceMap => "unsupported_no_source_map",
        }
    }

    /// Whether this action is read-only navigation that mutates no state.
    pub const fn is_read_only(self) -> bool {
        !matches!(self, Self::OpenInBrowserHandoff)
    }

    /// Whether this action must cite a browser-handoff packet ref.
    pub const fn requires_handoff_ref(self) -> bool {
        matches!(self, Self::OpenInBrowserHandoff)
    }

    /// Whether this action is the unsupported (no-source-map) action.
    pub const fn is_unsupported(self) -> bool {
        matches!(self, Self::UnsupportedNoSourceMap)
    }
}

/// Why a return-to-source action is blocked, if it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnToSourceBlockedClass {
    /// The action is admissible.
    NotBlocked,
    /// No source map is available to resolve a target.
    BlockedNoSourceMap,
    /// The source map is stale and a review is required first.
    BlockedSourceMapStaleReviewRequired,
    /// The content is generated and carries no resolvable origin.
    BlockedGeneratedContentNoOrigin,
    /// Policy forbids the browser handoff this action depends on.
    BlockedPolicyForbidsHandoff,
    /// The surface is offline or disconnected.
    BlockedOfflineOrDisconnected,
    /// Provider returned a block reason the contract does not recognise yet.
    BlockedUnknownReasonProviderOwned,
}

impl ReturnToSourceBlockedClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotBlocked => "not_blocked",
            Self::BlockedNoSourceMap => "blocked_no_source_map",
            Self::BlockedSourceMapStaleReviewRequired => "blocked_source_map_stale_review_required",
            Self::BlockedGeneratedContentNoOrigin => "blocked_generated_content_no_origin",
            Self::BlockedPolicyForbidsHandoff => "blocked_policy_forbids_handoff",
            Self::BlockedOfflineOrDisconnected => "blocked_offline_or_disconnected",
            Self::BlockedUnknownReasonProviderOwned => "blocked_unknown_reason_provider_owned",
        }
    }

    /// Whether the action is blocked.
    pub const fn is_blocked(self) -> bool {
        !matches!(self, Self::NotBlocked)
    }

    /// Whether this block class needs at least one explicit attention reason.
    pub const fn requires_attention_reason(self) -> bool {
        self.is_blocked()
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewDiagnosticsDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A diagnostic was surfaced without attribution.
    DiagnosticAttributionMissing,
    /// A return-to-source jump relied on a stale source map.
    SourceMapStale,
    /// A source mapping was not disclosed.
    SourceMapUndisclosed,
    /// A device / viewport state was not disclosed.
    ViewportStateUndisclosed,
    /// A diagnostic came from a provider-owned unknown framework pack.
    FrameworkPackUnknown,
    /// A return-to-source action was unsupported.
    ReturnToSourceUnsupported,
    /// Workspace trust narrowed.
    TrustNarrowing,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl PreviewDiagnosticsDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::DiagnosticAttributionMissing,
        Self::SourceMapStale,
        Self::SourceMapUndisclosed,
        Self::ViewportStateUndisclosed,
        Self::FrameworkPackUnknown,
        Self::ReturnToSourceUnsupported,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::DiagnosticAttributionMissing => "diagnostic_attribution_missing",
            Self::SourceMapStale => "source_map_stale",
            Self::SourceMapUndisclosed => "source_map_undisclosed",
            Self::ViewportStateUndisclosed => "viewport_state_undisclosed",
            Self::FrameworkPackUnknown => "framework_pack_unknown",
            Self::ReturnToSourceUnsupported => "return_to_source_unsupported",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project this lane's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewDiagnosticsConsumerSurface {
    /// Preview diagnostics panel.
    PreviewDiagnosticsPanel,
    /// Preview panel.
    PreviewPanel,
    /// Diagnostic card.
    DiagnosticCard,
    /// Return-to-source action affordance.
    ReturnToSourceAction,
    /// Review workspace header.
    ReviewWorkspaceHeader,
    /// Command palette.
    CommandPalette,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl PreviewDiagnosticsConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::PreviewDiagnosticsPanel,
        Self::PreviewPanel,
        Self::DiagnosticCard,
        Self::ReturnToSourceAction,
        Self::ReviewWorkspaceHeader,
        Self::CommandPalette,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewDiagnosticsPanel => "preview_diagnostics_panel",
            Self::PreviewPanel => "preview_panel",
            Self::DiagnosticCard => "diagnostic_card",
            Self::ReturnToSourceAction => "return_to_source_action",
            Self::ReviewWorkspaceHeader => "review_workspace_header",
            Self::CommandPalette => "command_palette",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Device / viewport state a preview diagnostic was captured in.
///
/// The state must be disclosed: a non-empty device label, a non-empty dimensions
/// label, and `state_disclosed` set true, so a preview can never hide which
/// device or viewport produced the view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceViewportState {
    /// Typed viewport class.
    pub viewport_class: ViewportClass,
    /// Redaction-aware device label (no raw device serial or hostname).
    pub device_label: String,
    /// Redaction-aware dimensions label (e.g. "390×844").
    pub dimensions_label: String,
    /// Whether the viewport is an emulated device.
    pub emulated: bool,
    /// Whether the device / viewport state is disclosed; required true.
    pub state_disclosed: bool,
}

impl DeviceViewportState {
    /// Whether the device / viewport state is fully disclosed.
    pub fn is_disclosed(&self) -> bool {
        self.state_disclosed
            && !self.device_label.trim().is_empty()
            && !self.dimensions_label.trim().is_empty()
    }

    /// Whether this state needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.viewport_class.requires_attention_reason()
    }
}

/// Source mapping disclosure for a return-to-source jump.
///
/// The disclosure must carry a non-empty mapping label and `mapping_disclosed`
/// set true, so the exactness and freshness of a return-to-source jump is never
/// hidden.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceMappingDisclosure {
    /// Typed mapping exactness.
    pub mapping_class: SourceMappingClass,
    /// Typed mapping freshness.
    pub freshness_class: SourceMappingFreshness,
    /// Whether the source mapping is disclosed; required true.
    pub mapping_disclosed: bool,
    /// Redaction-aware mapping label (no raw absolute path or source body).
    pub mapping_label: String,
}

impl SourceMappingDisclosure {
    /// Whether the source mapping is fully disclosed.
    pub fn is_disclosed(&self) -> bool {
        self.mapping_disclosed && !self.mapping_label.trim().is_empty()
    }

    /// Whether this disclosure needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.mapping_class.requires_attention_reason()
            || self.freshness_class.requires_attention_reason()
    }
}

/// Return-to-source action bound to a preview diagnostic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnToSourceAction {
    /// Typed action kind.
    pub action_kind: ReturnToSourceActionKind,
    /// Whether the action is disclosed; required true.
    pub action_disclosed: bool,
    /// Whether the action is read-only navigation; must match the action kind.
    pub read_only: bool,
    /// Human-readable, redaction-aware action label.
    pub action_label: String,
    /// Browser-handoff packet ref; required when the action kind is `open_in_browser_handoff`.
    pub handoff_ref: Option<String>,
}

impl ReturnToSourceAction {
    /// Whether the action is fully disclosed.
    pub fn is_disclosed(&self) -> bool {
        self.action_disclosed && !self.action_label.trim().is_empty()
    }

    /// Whether the read-only flag matches the action kind.
    pub fn read_only_flag_consistent(&self) -> bool {
        self.read_only == self.action_kind.is_read_only()
    }
}

/// One framework pack row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkPackRow {
    /// Stable pack id diagnostics reference.
    pub pack_id: String,
    /// Typed framework pack class.
    pub pack_class: FrameworkPackClass,
    /// Human-readable pack label.
    pub pack_label: String,
    /// Whether the pack supports source mapping for return-to-source.
    pub supports_source_mapping: bool,
    /// Whether the pack supports device / viewport emulation.
    pub supports_viewport_emulation: bool,
    /// Whether the pack supports hot reload.
    pub supports_hot_reload: bool,
    /// Human-readable coverage label.
    pub coverage_label: String,
    /// Human-readable disclosure label.
    pub disclosure_label: String,
}

/// One preview diagnostic row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewDiagnosticRow {
    /// Stable diagnostic id.
    pub diagnostic_id: String,
    /// Durable review anchor id bound to this diagnostic.
    pub durable_anchor_id: String,
    /// Framework pack id this diagnostic came from.
    pub pack_id: String,
    /// Human-readable preview target label (what was being previewed).
    pub preview_target_label: String,
    /// Device / viewport state the diagnostic was captured in.
    pub viewport_state: DeviceViewportState,
    /// Typed severity.
    pub severity: DiagnosticSeverity,
    /// Typed diagnostic kind.
    pub diagnostic_kind: DiagnosticKind,
    /// Redaction-aware diagnostic message label (no raw stack or source body).
    pub message_label: String,
    /// Source mapping disclosure for the return-to-source jump.
    pub source_mapping: SourceMappingDisclosure,
    /// Return-to-source action bound to the diagnostic.
    pub return_to_source: ReturnToSourceAction,
    /// Why the return-to-source action is blocked, if it is.
    pub blocked_class: ReturnToSourceBlockedClass,
    /// Human-readable actor attribution (under whose authority the diagnostic surfaced).
    pub actor_attribution_label: String,
    /// Opaque ref to the audit row that lands when the diagnostic action fires.
    pub audit_row_ref: String,
    /// Attention reasons; required and non-empty when the diagnostic needs attention.
    pub attention_reasons: Vec<String>,
    /// Human-readable review summary.
    pub review_summary: String,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl PreviewDiagnosticRow {
    /// Whether this diagnostic needs at least one explicit attention reason.
    pub const fn requires_attention_reason(&self) -> bool {
        self.severity.requires_attention_reason()
            || self.diagnostic_kind.requires_attention_reason()
            || self.viewport_state.requires_attention_reason()
            || self.source_mapping.requires_attention_reason()
            || self.blocked_class.requires_attention_reason()
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewDiagnosticsTrustReview {
    /// The diagnostic severity is explicit, never implied.
    pub diagnostic_severity_explicit: bool,
    /// The device / viewport state is disclosed, never hidden.
    pub device_viewport_state_disclosed: bool,
    /// The source mapping is disclosed, never hidden.
    pub source_mapping_disclosed: bool,
    /// The source mapping freshness is disclosed.
    pub source_mapping_freshness_disclosed: bool,
    /// The return-to-source action is disclosed, never hidden.
    pub return_to_source_action_disclosed: bool,
    /// A return-to-source action is read-only unless an attributable handoff is cited.
    pub return_to_source_read_only_unless_attributed: bool,
    /// The framework pack identity is explicit, never assumed.
    pub framework_pack_identity_explicit: bool,
    /// Every diagnostic is bound to a durable review anchor.
    pub every_diagnostic_anchored: bool,
    /// Every diagnostic action is attributable to an actor.
    pub every_action_attributable: bool,
    /// No return-to-source action creates hidden write scope.
    pub no_hidden_write_scope: bool,
    /// A stale source map narrows the return-to-source action rather than jumping blind.
    pub stale_source_map_narrows_action: bool,
    /// Downgrade narrows the claim rather than hiding the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl PreviewDiagnosticsTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.diagnostic_severity_explicit
            && self.device_viewport_state_disclosed
            && self.source_mapping_disclosed
            && self.source_mapping_freshness_disclosed
            && self.return_to_source_action_disclosed
            && self.return_to_source_read_only_unless_attributed
            && self.framework_pack_identity_explicit
            && self.every_diagnostic_anchored
            && self.every_action_attributable
            && self.no_hidden_write_scope
            && self.stale_source_map_narrows_action
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewDiagnosticsConsumerProjection {
    /// Preview diagnostics panel shows the severity.
    pub preview_diagnostics_panel_shows_severity: bool,
    /// Preview panel shows the device / viewport state.
    pub preview_panel_shows_viewport_state: bool,
    /// Diagnostic card shows the framework pack.
    pub diagnostic_card_shows_framework_pack: bool,
    /// Diagnostic card shows the source mapping.
    pub diagnostic_card_shows_source_mapping: bool,
    /// Return-to-source action shows the source-map freshness.
    pub return_to_source_action_shows_freshness: bool,
    /// Review workspace header shows the actor attribution.
    pub review_workspace_header_shows_attribution: bool,
    /// Command palette shows the diagnostic state.
    pub command_palette_shows_diagnostic_state: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_truth: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_truth: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_truth: bool,
    /// Preview / Labs lanes are labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified: bool,
}

impl PreviewDiagnosticsConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.preview_diagnostics_panel_shows_severity
            && self.preview_panel_shows_viewport_state
            && self.diagnostic_card_shows_framework_pack
            && self.diagnostic_card_shows_source_mapping
            && self.return_to_source_action_shows_freshness
            && self.review_workspace_header_shows_attribution
            && self.command_palette_shows_diagnostic_state
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.diagnostics_shows_truth
            && self.help_about_shows_truth
            && self.preview_labs_label_for_unqualified
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewDiagnosticsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`PreviewDiagnosticsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewDiagnosticsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Framework pack rows.
    pub framework_pack_rows: Vec<FrameworkPackRow>,
    /// Preview diagnostic rows.
    pub diagnostic_rows: Vec<PreviewDiagnosticRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<PreviewDiagnosticsDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<PreviewDiagnosticsConsumerSurface>,
    /// Trust review block.
    pub trust_review: PreviewDiagnosticsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PreviewDiagnosticsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PreviewDiagnosticsProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe preview diagnostics, device / viewport, and return-to-source packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewDiagnosticsPacket {
    /// Record kind; must equal [`PREVIEW_DIAGNOSTICS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PREVIEW_DIAGNOSTICS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Framework pack rows.
    pub framework_pack_rows: Vec<FrameworkPackRow>,
    /// Preview diagnostic rows.
    pub diagnostic_rows: Vec<PreviewDiagnosticRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<PreviewDiagnosticsDowngradeTrigger>,
    /// Consumer surfaces that must project this lane's truth.
    pub consumer_surfaces: Vec<PreviewDiagnosticsConsumerSurface>,
    /// Trust review block.
    pub trust_review: PreviewDiagnosticsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: PreviewDiagnosticsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: PreviewDiagnosticsProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PreviewDiagnosticsPacket {
    /// Builds a preview diagnostics packet from stable-lane input.
    pub fn new(input: PreviewDiagnosticsPacketInput) -> Self {
        Self {
            record_kind: PREVIEW_DIAGNOSTICS_RECORD_KIND.to_owned(),
            schema_version: PREVIEW_DIAGNOSTICS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            framework_pack_rows: input.framework_pack_rows,
            diagnostic_rows: input.diagnostic_rows,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the preview diagnostics review invariants.
    pub fn validate(&self) -> Vec<PreviewDiagnosticsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PREVIEW_DIAGNOSTICS_RECORD_KIND {
            violations.push(PreviewDiagnosticsViolation::WrongRecordKind);
        }
        if self.schema_version != PREVIEW_DIAGNOSTICS_SCHEMA_VERSION {
            violations.push(PreviewDiagnosticsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PreviewDiagnosticsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(PreviewDiagnosticsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(PreviewDiagnosticsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_framework_pack_rows(self, &mut violations);
        validate_diagnostic_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(PreviewDiagnosticsViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(PreviewDiagnosticsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(PreviewDiagnosticsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("preview diagnostics packet serializes"),
        ) {
            violations.push(PreviewDiagnosticsViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("preview diagnostics packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let error_diagnostics = self
            .diagnostic_rows
            .iter()
            .filter(|row| {
                matches!(
                    row.severity,
                    DiagnosticSeverity::Error | DiagnosticSeverity::Fatal
                )
            })
            .count();
        let blocked_actions = self
            .diagnostic_rows
            .iter()
            .filter(|row| row.blocked_class.is_blocked())
            .count();
        let stale_source_maps = self
            .diagnostic_rows
            .iter()
            .filter(|row| !row.source_mapping.freshness_class.is_fresh())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Preview Diagnostics, Device/Viewport States, and Return-to-Source Actions Across Framework Packs\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Framework packs: {}\n",
            self.framework_pack_rows.len()
        ));
        out.push_str(&format!(
            "- Diagnostics: {} ({} error/fatal, {} blocked actions, {} stale source maps)\n",
            self.diagnostic_rows.len(),
            error_diagnostics,
            blocked_actions,
            stale_source_maps
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Framework packs\n\n");
        for row in &self.framework_pack_rows {
            out.push_str(&format!(
                "- **{}** (`{}`): source-map {}, viewport {}, hot-reload {}\n",
                row.pack_label,
                row.pack_class.as_str(),
                row.supports_source_mapping,
                row.supports_viewport_emulation,
                row.supports_hot_reload
            ));
        }

        out.push_str("\n## Diagnostics\n\n");
        for row in &self.diagnostic_rows {
            out.push_str(&format!(
                "- **{}** ({}/{}) on pack `{}` → anchor `{}`: viewport `{}`, mapping `{}`/`{}`, action `{}`, blocked `{}`, authority `{}`\n",
                row.preview_target_label,
                row.severity.as_str(),
                row.diagnostic_kind.as_str(),
                row.pack_id,
                row.durable_anchor_id,
                row.viewport_state.viewport_class.as_str(),
                row.source_mapping.mapping_class.as_str(),
                row.source_mapping.freshness_class.as_str(),
                row.return_to_source.action_kind.as_str(),
                row.blocked_class.as_str(),
                row.actor_attribution_label
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in preview diagnostics export.
#[derive(Debug)]
pub enum PreviewDiagnosticsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PreviewDiagnosticsViolation>),
}

impl fmt::Display for PreviewDiagnosticsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "preview diagnostics export parse failed: {error}"
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
                    "preview diagnostics export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PreviewDiagnosticsArtifactError {}

/// Validation failures emitted by [`PreviewDiagnosticsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreviewDiagnosticsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No framework pack rows are present.
    FrameworkPackRowsMissing,
    /// A framework pack row is incomplete.
    FrameworkPackRowIncomplete,
    /// No diagnostic rows are present.
    DiagnosticRowsMissing,
    /// A diagnostic row is incomplete.
    DiagnosticRowIncomplete,
    /// A diagnostic references a pack id with no framework pack row.
    OrphanPackReference,
    /// A diagnostic's device / viewport state is undisclosed.
    ViewportStateUndisclosed,
    /// A diagnostic's source mapping is undisclosed.
    SourceMappingUndisclosed,
    /// A diagnostic's return-to-source action is undisclosed.
    ReturnToSourceUndisclosed,
    /// A return-to-source action's read-only flag does not match its kind.
    ReturnToSourceReadOnlyMismatch,
    /// A return-to-source handoff action is missing its handoff ref.
    ReturnToSourceHandoffRefMissing,
    /// An unsupported return-to-source action is not blocked for it.
    ReturnToSourceUnsupportedNotBlocked,
    /// A diagnostic is missing its actor attribution or audit row.
    AttributionMissing,
    /// A diagnostic needing attention is missing its attention reasons.
    AttentionReasonMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PreviewDiagnosticsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::FrameworkPackRowsMissing => "framework_pack_rows_missing",
            Self::FrameworkPackRowIncomplete => "framework_pack_row_incomplete",
            Self::DiagnosticRowsMissing => "diagnostic_rows_missing",
            Self::DiagnosticRowIncomplete => "diagnostic_row_incomplete",
            Self::OrphanPackReference => "orphan_pack_reference",
            Self::ViewportStateUndisclosed => "viewport_state_undisclosed",
            Self::SourceMappingUndisclosed => "source_mapping_undisclosed",
            Self::ReturnToSourceUndisclosed => "return_to_source_undisclosed",
            Self::ReturnToSourceReadOnlyMismatch => "return_to_source_read_only_mismatch",
            Self::ReturnToSourceHandoffRefMissing => "return_to_source_handoff_ref_missing",
            Self::ReturnToSourceUnsupportedNotBlocked => "return_to_source_unsupported_not_blocked",
            Self::AttributionMissing => "attribution_missing",
            Self::AttentionReasonMissing => "attention_reason_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable preview diagnostics export.
pub fn current_preview_diagnostics_export(
) -> Result<PreviewDiagnosticsPacket, PreviewDiagnosticsArtifactError> {
    let packet: PreviewDiagnosticsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/implement_preview_diagnostics_device_or_viewport_states_and_return_to_source_actions_across_framework_packs/support_export.json"
    )))
    .map_err(PreviewDiagnosticsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PreviewDiagnosticsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &PreviewDiagnosticsPacket,
    violations: &mut Vec<PreviewDiagnosticsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PREVIEW_DIAGNOSTICS_SCHEMA_REF,
        PREVIEW_DIAGNOSTICS_DOC_REF,
        PREVIEW_DIAGNOSTICS_PREVIEW_TARGET_CONTRACT_REF,
        PREVIEW_DIAGNOSTICS_DEVICE_TARGET_CONTRACT_REF,
        PREVIEW_DIAGNOSTICS_HOT_RELOAD_CONTRACT_REF,
        PREVIEW_DIAGNOSTICS_TRUST_CLASS_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PreviewDiagnosticsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_framework_pack_rows(
    packet: &PreviewDiagnosticsPacket,
    violations: &mut Vec<PreviewDiagnosticsViolation>,
) {
    if packet.framework_pack_rows.is_empty() {
        violations.push(PreviewDiagnosticsViolation::FrameworkPackRowsMissing);
        return;
    }

    for row in &packet.framework_pack_rows {
        if row.pack_id.trim().is_empty()
            || row.pack_label.trim().is_empty()
            || row.coverage_label.trim().is_empty()
            || row.disclosure_label.trim().is_empty()
        {
            violations.push(PreviewDiagnosticsViolation::FrameworkPackRowIncomplete);
        }
    }
}

fn validate_diagnostic_rows(
    packet: &PreviewDiagnosticsPacket,
    violations: &mut Vec<PreviewDiagnosticsViolation>,
) {
    if packet.diagnostic_rows.is_empty() {
        violations.push(PreviewDiagnosticsViolation::DiagnosticRowsMissing);
        return;
    }

    let pack_ids: BTreeSet<&str> = packet
        .framework_pack_rows
        .iter()
        .map(|row| row.pack_id.as_str())
        .collect();

    for row in &packet.diagnostic_rows {
        if row.diagnostic_id.trim().is_empty()
            || row.durable_anchor_id.trim().is_empty()
            || row.preview_target_label.trim().is_empty()
            || row.message_label.trim().is_empty()
            || row.review_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(PreviewDiagnosticsViolation::DiagnosticRowIncomplete);
        }
        if !row.pack_id.trim().is_empty() && !pack_ids.contains(row.pack_id.as_str()) {
            violations.push(PreviewDiagnosticsViolation::OrphanPackReference);
        }
        if !row.viewport_state.is_disclosed() {
            violations.push(PreviewDiagnosticsViolation::ViewportStateUndisclosed);
        }
        if !row.source_mapping.is_disclosed() {
            violations.push(PreviewDiagnosticsViolation::SourceMappingUndisclosed);
        }
        validate_return_to_source(row, violations);
        if row.actor_attribution_label.trim().is_empty() || row.audit_row_ref.trim().is_empty() {
            violations.push(PreviewDiagnosticsViolation::AttributionMissing);
        }
        if row.requires_attention_reason() && row.attention_reasons.is_empty() {
            violations.push(PreviewDiagnosticsViolation::AttentionReasonMissing);
        }
    }
}

fn validate_return_to_source(
    row: &PreviewDiagnosticRow,
    violations: &mut Vec<PreviewDiagnosticsViolation>,
) {
    let action = &row.return_to_source;
    if !action.is_disclosed() {
        violations.push(PreviewDiagnosticsViolation::ReturnToSourceUndisclosed);
    }
    if !action.read_only_flag_consistent() {
        violations.push(PreviewDiagnosticsViolation::ReturnToSourceReadOnlyMismatch);
    }
    if action.action_kind.requires_handoff_ref()
        && !action
            .handoff_ref
            .as_deref()
            .is_some_and(|reference| !reference.trim().is_empty())
    {
        violations.push(PreviewDiagnosticsViolation::ReturnToSourceHandoffRefMissing);
    }
    if action.action_kind.is_unsupported() && !row.blocked_class.is_blocked() {
        violations.push(PreviewDiagnosticsViolation::ReturnToSourceUnsupportedNotBlocked);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret ")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
