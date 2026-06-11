//! Component-state registry and design-token inheritance audit for the
//! M5 depth surfaces.
//!
//! The M5 depth lanes ship new panes — notebook cell chrome, result-grid
//! rows, profiler and trace panels, pipeline cards, preview-route badges,
//! docs/browser panes, companion surfaces, sync status, and offboarding —
//! each of which can quietly grow its own styling and state vocabulary.
//! This module carries the stable v1 shell promise forward into those
//! lanes: every M5 surface MUST inherit the same design-token system and
//! the same normalized component-state vocabulary as the v1 shell, and it
//! MUST never encode severity, trust, or lifecycle meaning through a
//! one-off color or a private badge outside the shared registry.
//!
//! The audit projects, for each registered M5 surface, the canonical
//! component-state descriptor against the binding that the surface
//! actually renders for each of the nine normalized component states the
//! M5 lanes introduce:
//!
//! - `loading`
//! - `cached`
//! - `stale`
//! - `partial`
//! - `policy_blocked`
//! - `degraded`
//! - `preview_only`
//! - `sync_pending`
//! - `boundary_handoff`
//!
//! The resulting [`M5ComponentStateAuditReport`] is the canonical truth
//! object for the M5 visual-system and state-semantics lane. It is
//! consumed by:
//!
//! - the live shell design-QA inspector (so the in-product audit quotes
//!   the same per-row findings the CLI prints);
//! - the headless inspector (`aureline_shell_m5_component_state`), which
//!   is the only mint-from-truth path for the JSON fixtures checked in
//!   under `fixtures/ux/m5/theme-token-consumers/`;
//! - the support-export wrapper that lets a reviewer pivot from a support
//!   case to the row that flagged drift;
//! - the markdown audit under
//!   `artifacts/ux/m5/component-state-audit/m5_component_state_audit.md`
//!   (rendered from the same seed); and
//! - the cross-surface hardening matrix, which can ingest the audit
//!   directly when qualifying or narrowing an M5 row.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every registered M5 surface must declare a binding for each of the
//!    nine normalized component states.
//! 2. Every surface must carry a canonical shared-registry anchor, a
//!    non-empty accessibility note, and a registration flag asserting it
//!    is part of the shared component-state registry; a missing anchor,
//!    missing note, or unregistered surface is a blocker.
//! 3. A high-salience surface (one that conveys lifecycle, trust, or
//!    severity meaning) with an `unknown_token_gap` on a required state,
//!    one that declares `color_allowed` instead of a required non-color
//!    cue policy, and any surface that renders a state through ad-hoc
//!    local semantics (`unregistered_local_state`) are blockers.
//! 4. Token group, token ref, style provenance, cue policy, non-color
//!    cue, registry anchor, and applied overrides MUST come from the same
//!    canonical descriptor and shared token system across every inherited
//!    binding; a drift is a blocker.
//! 5. A hard-coded theme value or an unresolved token fallback on any
//!    binding is a blocker, so a surface can never paint a state with a
//!    literal hex/rgb value or an unresolved fallback.
//! 6. At least one surface must inherit each of the nine normalized states
//!    so the audit cannot regress into a single-state view.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/ux/m5/theme-token-consumers/` are
//! bit-for-bit equal to the seeded report returned by
//! [`seeded_m5_component_state_audit`].

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every M5 component-state record.
pub const M5_COMPONENT_STATE_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every M5 component-state row.
pub const M5_COMPONENT_STATE_SHARED_CONTRACT_REF: &str = "shell:m5_component_state:v1";

/// Stable record kind for [`M5ComponentStateAuditReport`] payloads.
pub const M5_COMPONENT_STATE_REPORT_RECORD_KIND: &str =
    "shell_m5_component_state_audit_report_record";

/// Stable record kind for [`M5ComponentStateRow`] payloads.
pub const M5_COMPONENT_STATE_ROW_RECORD_KIND: &str = "shell_m5_component_state_row_record";

/// Stable record kind for [`M5ComponentStateSupportExport`] payloads.
pub const M5_COMPONENT_STATE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_component_state_support_export_record";

/// Stable report id quoted across surfaces.
pub const M5_COMPONENT_STATE_REPORT_ID: &str = "shell:m5_component_state:audit:v1";

/// Stable support-export id quoted in the published wrapper.
pub const M5_COMPONENT_STATE_SUPPORT_EXPORT_ID: &str = "support-export:m5-component-state:001";

/// Source schema ref for the canonical component-state contract.
pub const M5_COMPONENT_STATE_SOURCE_SCHEMA_REF: &str = "schemas/ux/m5-component-state.schema.json";

/// Path of the published markdown audit artifact.
pub const M5_COMPONENT_STATE_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/component-state-audit/m5_component_state_audit.md";

/// Path of the published companion doc.
pub const M5_COMPONENT_STATE_PUBLISHED_DOC_REF: &str = "docs/m5/component-state-parity.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-11T00:00:00Z";

/// One M5 depth surface whose component states the audit registers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceFamily {
    /// Notebook cell chrome (gutter, run state, output frame).
    NotebookCellChrome,
    /// Data / API result-grid rows.
    ResultGridRow,
    /// Profiler capture and flame panel.
    ProfilerPanel,
    /// Trace capture and replay panel.
    TracePanel,
    /// Review / pipeline status cards.
    PipelineCard,
    /// Live preview-route badges.
    PreviewRouteBadge,
    /// Embedded docs / browser panes.
    DocsBrowserPane,
    /// Companion / cross-device surfaces.
    CompanionSurface,
    /// Workspace sync status surfaces.
    SyncStatusSurface,
    /// Offboarding / export-and-wipe surfaces.
    OffboardingSurface,
}

impl M5SurfaceFamily {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookCellChrome => "notebook_cell_chrome",
            Self::ResultGridRow => "result_grid_row",
            Self::ProfilerPanel => "profiler_panel",
            Self::TracePanel => "trace_panel",
            Self::PipelineCard => "pipeline_card",
            Self::PreviewRouteBadge => "preview_route_badge",
            Self::DocsBrowserPane => "docs_browser_pane",
            Self::CompanionSurface => "companion_surface",
            Self::SyncStatusSurface => "sync_status_surface",
            Self::OffboardingSurface => "offboarding_surface",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::NotebookCellChrome => "Notebook cell chrome",
            Self::ResultGridRow => "Result-grid row",
            Self::ProfilerPanel => "Profiler panel",
            Self::TracePanel => "Trace panel",
            Self::PipelineCard => "Pipeline card",
            Self::PreviewRouteBadge => "Preview-route badge",
            Self::DocsBrowserPane => "Docs / browser pane",
            Self::CompanionSurface => "Companion surface",
            Self::SyncStatusSurface => "Sync status surface",
            Self::OffboardingSurface => "Offboarding surface",
        }
    }
}

/// One of the nine normalized component states the audit covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NormalizedState {
    /// Work in flight; the surface is rendering a determinate or
    /// indeterminate loading treatment.
    Loading,
    /// Content served from a local cache rather than a fresh fetch.
    Cached,
    /// Content known to be out of date relative to its source.
    Stale,
    /// A partial result set (truncated, sampled, or still streaming).
    Partial,
    /// The action or content is blocked by a policy or capability gate.
    PolicyBlocked,
    /// The surface is operating in a reduced-capability degraded mode.
    Degraded,
    /// The content is preview-only and not yet applied or committed.
    PreviewOnly,
    /// A durable sync or publish is pending for this surface.
    SyncPending,
    /// The surface is handing off across an embedded or device boundary.
    BoundaryHandoff,
}

impl M5NormalizedState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Loading => "loading",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Partial => "partial",
            Self::PolicyBlocked => "policy_blocked",
            Self::Degraded => "degraded",
            Self::PreviewOnly => "preview_only",
            Self::SyncPending => "sync_pending",
            Self::BoundaryHandoff => "boundary_handoff",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Loading => "Loading",
            Self::Cached => "Cached",
            Self::Stale => "Stale",
            Self::Partial => "Partial",
            Self::PolicyBlocked => "Policy-blocked",
            Self::Degraded => "Degraded",
            Self::PreviewOnly => "Preview-only",
            Self::SyncPending => "Sync-pending",
            Self::BoundaryHandoff => "Boundary-handoff",
        }
    }

    /// Returns the nine required normalized states in canonical order.
    pub const fn required_states() -> [Self; 9] {
        [
            Self::Loading,
            Self::Cached,
            Self::Stale,
            Self::Partial,
            Self::PolicyBlocked,
            Self::Degraded,
            Self::PreviewOnly,
            Self::SyncPending,
            Self::BoundaryHandoff,
        ]
    }
}

/// Shared token group an M5 surface inherits its state palette from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TokenGroup {
    /// Surface chrome tokens (notebook gutter, run frame).
    SurfaceChromeTokens,
    /// Dense data-row tokens.
    DataDensityTokens,
    /// Diagnostic-state tokens (profiler, trace).
    DiagnosticStateTokens,
    /// Pipeline status tokens.
    PipelineStatusTokens,
    /// Preview-badge tokens.
    PreviewBadgeTokens,
    /// Embedded-surface tokens (docs/browser).
    EmbeddedSurfaceTokens,
    /// Companion presence tokens.
    CompanionPresenceTokens,
    /// Sync status tokens.
    SyncStatusTokens,
    /// Lifecycle-state tokens.
    LifecycleStateTokens,
}

impl M5TokenGroup {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceChromeTokens => "surface_chrome_tokens",
            Self::DataDensityTokens => "data_density_tokens",
            Self::DiagnosticStateTokens => "diagnostic_state_tokens",
            Self::PipelineStatusTokens => "pipeline_status_tokens",
            Self::PreviewBadgeTokens => "preview_badge_tokens",
            Self::EmbeddedSurfaceTokens => "embedded_surface_tokens",
            Self::CompanionPresenceTokens => "companion_presence_tokens",
            Self::SyncStatusTokens => "sync_status_tokens",
            Self::LifecycleStateTokens => "lifecycle_state_tokens",
        }
    }
}

/// Where a surface's styling comes from, pinned on its descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StyleProvenance {
    /// The surface inherits the shared shell token system directly.
    ShellTokenInherited,
    /// The surface inherits the shell tokens but documents a scoped
    /// override.
    ShellTokenWithDeclaredOverride,
    /// The surface is extension-contributed and must declare any
    /// inheritance gaps.
    ExtensionContributed,
    /// The surface is provider-backed and must declare any inheritance
    /// gaps.
    ProviderBacked,
}

impl M5StyleProvenance {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellTokenInherited => "shell_token_inherited",
            Self::ShellTokenWithDeclaredOverride => "shell_token_with_declared_override",
            Self::ExtensionContributed => "extension_contributed",
            Self::ProviderBacked => "provider_backed",
        }
    }
}

/// How much trust, lifecycle, or severity meaning the surface conveys.
///
/// A surface that conveys lifecycle, trust, or severity meaning is
/// "high-salience": it MUST stay registered, token-driven, and carry a
/// non-color cue so the meaning is never encoded through color alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SemanticSalience {
    /// Purely decorative; carries no semantic meaning.
    DecorativeOnly,
    /// Informational only; no trust, lifecycle, or severity meaning.
    Informational,
    /// Conveys lifecycle state (preview, stale, pending).
    LifecycleBearing,
    /// Conveys trust or identity (companion presence, boundary).
    TrustBearing,
    /// Conveys severity or risk (blocked, destructive, failed).
    SeverityBearing,
}

impl M5SemanticSalience {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DecorativeOnly => "decorative_only",
            Self::Informational => "informational",
            Self::LifecycleBearing => "lifecycle_bearing",
            Self::TrustBearing => "trust_bearing",
            Self::SeverityBearing => "severity_bearing",
        }
    }

    /// `true` for salience classes that must never rely on color alone.
    pub const fn is_high_salience(self) -> bool {
        matches!(
            self,
            Self::LifecycleBearing | Self::TrustBearing | Self::SeverityBearing
        )
    }
}

/// Cue policy the descriptor pins for the surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CuePolicy {
    /// State meaning MUST carry a non-color cue (icon, text, or shape).
    NonColorCueRequired,
    /// Color-only treatment is acceptable for this surface.
    ColorAllowed,
}

impl M5CuePolicy {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonColorCueRequired => "non_color_cue_required",
            Self::ColorAllowed => "color_allowed",
        }
    }
}

/// The non-color cue a state binding carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AccessibilityCue {
    /// An icon paired with a text label.
    IconAndText,
    /// A text label alone.
    TextLabel,
    /// A shape or pattern treatment.
    ShapeOrPattern,
    /// Color alone, with no non-color cue. A blocker on high-salience
    /// surfaces and any surface that requires a non-color cue.
    ColorOnly,
}

impl M5AccessibilityCue {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IconAndText => "icon_and_text",
            Self::TextLabel => "text_label",
            Self::ShapeOrPattern => "shape_or_pattern",
            Self::ColorOnly => "color_only",
        }
    }
}

/// Lifecycle label retained on the canonical surface descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceLifecycle {
    /// Generally available.
    Stable,
    /// Beta lane; visibility and narrowing can change.
    Beta,
    /// Deprecated; surfaces must point at the replacement.
    Deprecated,
}

impl M5SurfaceLifecycle {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Binding status reported by a surface for one normalized state.
///
/// Only `Inherited` rows are subject to token/descriptor drift checks.
/// `ExplicitlyNarrowed`, `NotApplicable`, `DeclaredInheritanceGap`, and
/// `PlatformOmitted` rows are accepted as long as they carry a
/// `narrowing_reason`. `UnregisteredLocalState` (ad-hoc local styling)
/// and `UnknownTokenGap` are blocking findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BindingStatus {
    /// The surface inherits the shared registry token and state class.
    Inherited,
    /// The surface narrows this state; a `narrowing_reason` MUST be set.
    ExplicitlyNarrowed,
    /// The state does not apply to this surface; a reason MUST be set.
    NotApplicable,
    /// An extension/provider surface declares a known inheritance gap
    /// honestly; a reason MUST be set.
    DeclaredInheritanceGap,
    /// The state is not surfaced on this client/platform; a reason MUST be
    /// set.
    PlatformOmitted,
    /// The surface renders this state through ad-hoc local semantics
    /// outside the shared registry. Always a blocker.
    UnregisteredLocalState,
    /// The required state has a missing or unknown token binding. Always a
    /// blocker on high-salience surfaces.
    UnknownTokenGap,
}

impl M5BindingStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inherited => "inherited",
            Self::ExplicitlyNarrowed => "explicitly_narrowed",
            Self::NotApplicable => "not_applicable",
            Self::DeclaredInheritanceGap => "declared_inheritance_gap",
            Self::PlatformOmitted => "platform_omitted",
            Self::UnregisteredLocalState => "unregistered_local_state",
            Self::UnknownTokenGap => "unknown_token_gap",
        }
    }

    /// `true` for statuses that require a `narrowing_reason`.
    pub const fn requires_narrowing_reason(self) -> bool {
        matches!(
            self,
            Self::ExplicitlyNarrowed
                | Self::NotApplicable
                | Self::DeclaredInheritanceGap
                | Self::PlatformOmitted
        )
    }

    /// `true` for statuses projected from the descriptor and therefore
    /// subject to token/descriptor drift checks.
    pub const fn projects_descriptor(self) -> bool {
        matches!(self, Self::Inherited)
    }
}

/// Canonical descriptor for one M5 surface's component-state contract.
///
/// Every inherited binding in the audit quotes these fields verbatim; any
/// divergence is a blocking finding the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentStateDescriptor {
    /// Stable surface id (e.g. `surface:notebook.cell_chrome`).
    pub surface_id: String,
    /// Surface family the descriptor belongs to.
    pub surface_family: M5SurfaceFamily,
    /// Descriptor revision the audit was produced against.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Canonical shared-registry anchor ref the audit can reopen the
    /// surface's registry entry from.
    pub registry_anchor_ref: String,
    /// Accessibility note retained on the descriptor. MUST be non-empty.
    pub accessibility_note: String,
    /// Shared token group the surface inherits its state palette from.
    pub token_group: M5TokenGroup,
    /// Pinned style provenance.
    pub style_provenance: M5StyleProvenance,
    /// Pinned semantic salience.
    pub semantic_salience: M5SemanticSalience,
    /// Pinned cue policy.
    pub cue_policy: M5CuePolicy,
    /// Pinned surface lifecycle label.
    pub lifecycle_label: M5SurfaceLifecycle,
    /// Canonical documented-override set the surface owns. Bindings MUST
    /// NOT apply overrides outside this set.
    pub documented_overrides: Vec<String>,
    /// `true` once the surface is registered in the shared component-state
    /// registry (and not pane-local only). MUST be `true`.
    pub registered_in_shared_registry: bool,
}

impl M5ComponentStateDescriptor {
    /// `true` when this surface's salience makes it high-salience for the
    /// audit.
    pub const fn is_high_salience(&self) -> bool {
        self.semantic_salience.is_high_salience()
    }
}

/// Per-state binding a surface reports for one normalized state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateBinding {
    /// Normalized state this binding covers.
    pub state: M5NormalizedState,
    /// Binding status the surface reports.
    pub binding_status: M5BindingStatus,
    /// Projected token group (`None` for non-inherited rows).
    pub projected_token_group: Option<M5TokenGroup>,
    /// Projected token ref (`None` for non-inherited rows).
    pub projected_token_ref: Option<String>,
    /// Projected style provenance (`None` for non-inherited rows).
    pub projected_style_provenance: Option<M5StyleProvenance>,
    /// Projected cue policy (`None` for non-inherited rows).
    pub projected_cue_policy: Option<M5CuePolicy>,
    /// Projected non-color cue (`None` for non-inherited rows).
    pub projected_non_color_cue: Option<M5AccessibilityCue>,
    /// Projected registry anchor ref (`None` for non-inherited rows).
    pub projected_registry_anchor_ref: Option<String>,
    /// Overrides the binding applies. MUST be a subset of the descriptor's
    /// documented-override set.
    pub applied_overrides: Vec<String>,
    /// A hard-coded theme value the binding paints with, when present.
    /// Always a blocker.
    pub hardcoded_value: Option<String>,
    /// An unresolved token fallback the binding falls back to, when
    /// present. Always a blocker.
    pub unresolved_token_fallback: Option<String>,
    /// Narrowing reason set when `binding_status` requires one.
    pub narrowing_reason: Option<String>,
    /// Reviewer-facing free-form note retained on the row.
    pub note: Option<String>,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum M5ParityBlockingFinding {
    /// A high-salience surface has an unknown token binding for a required
    /// state.
    UnknownTokenGap {
        /// Surface that exposes the gap.
        surface_id: String,
        /// State that exposes the gap.
        state: M5NormalizedState,
    },
    /// A surface renders a state through ad-hoc local semantics outside
    /// the shared registry.
    UnregisteredLocalState {
        surface_id: String,
        state: M5NormalizedState,
    },
    /// An inherited binding projects a token group that disagrees with the
    /// descriptor.
    TokenGroupDrift {
        surface_id: String,
        state: M5NormalizedState,
        /// Projected token group.
        projected_token_group: M5TokenGroup,
    },
    /// An inherited binding projects a token ref that disagrees with the
    /// canonical token ref.
    TokenRefDrift {
        surface_id: String,
        state: M5NormalizedState,
        /// Projected token ref.
        projected_token_ref: String,
    },
    /// An inherited binding projects a style provenance that disagrees
    /// with the descriptor.
    StyleProvenanceDrift {
        surface_id: String,
        state: M5NormalizedState,
        /// Projected style provenance.
        projected_style_provenance: M5StyleProvenance,
    },
    /// An inherited binding projects a cue policy that disagrees with the
    /// descriptor.
    CuePolicyDrift {
        surface_id: String,
        state: M5NormalizedState,
        /// Projected cue policy.
        projected_cue_policy: M5CuePolicy,
    },
    /// A binding encodes state meaning through color alone.
    ColorOnlyCue {
        surface_id: String,
        state: M5NormalizedState,
    },
    /// A binding paints a state with a hard-coded theme value.
    HardcodedThemeValue {
        surface_id: String,
        state: M5NormalizedState,
        /// The hard-coded value seen.
        hardcoded_value: String,
    },
    /// A binding falls back to an unresolved token fallback.
    UnresolvedTokenFallback {
        surface_id: String,
        state: M5NormalizedState,
        /// The unresolved fallback ref seen.
        unresolved_token_fallback: String,
    },
    /// An inherited binding cannot point back to the canonical registry
    /// anchor.
    MissingRegistryAnchor {
        surface_id: String,
        state: M5NormalizedState,
    },
    /// An inherited binding applies an override outside the documented
    /// set.
    OverrideDrift {
        surface_id: String,
        state: M5NormalizedState,
        /// First override seen that the descriptor does not own.
        offending_override: String,
    },
    /// A non-inherited row is missing the `narrowing_reason`.
    MissingNarrowingReason {
        surface_id: String,
        state: M5NormalizedState,
        binding_status: M5BindingStatus,
    },
    /// An inherited row is missing a projection field it requires.
    MissingProjection {
        surface_id: String,
        state: M5NormalizedState,
        /// Name of the missing projection field.
        field: String,
    },
    /// The descriptor carries no canonical registry anchor.
    DescriptorMissingRegistryAnchor { surface_id: String },
    /// The descriptor carries no accessibility note.
    MissingAccessibilityNote { surface_id: String },
    /// A high-salience surface declares `color_allowed` instead of a
    /// required non-color cue policy.
    MissingNonColorCuePolicy { surface_id: String },
    /// The surface is not registered in the shared component-state
    /// registry.
    SurfaceNotRegistered { surface_id: String },
}

impl M5ParityBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::UnknownTokenGap { .. } => "unknown_token_gap",
            Self::UnregisteredLocalState { .. } => "unregistered_local_state",
            Self::TokenGroupDrift { .. } => "token_group_drift",
            Self::TokenRefDrift { .. } => "token_ref_drift",
            Self::StyleProvenanceDrift { .. } => "style_provenance_drift",
            Self::CuePolicyDrift { .. } => "cue_policy_drift",
            Self::ColorOnlyCue { .. } => "color_only_cue",
            Self::HardcodedThemeValue { .. } => "hardcoded_theme_value",
            Self::UnresolvedTokenFallback { .. } => "unresolved_token_fallback",
            Self::MissingRegistryAnchor { .. } => "missing_registry_anchor",
            Self::OverrideDrift { .. } => "override_drift",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingProjection { .. } => "missing_projection",
            Self::DescriptorMissingRegistryAnchor { .. } => "descriptor_missing_registry_anchor",
            Self::MissingAccessibilityNote { .. } => "missing_accessibility_note",
            Self::MissingNonColorCuePolicy { .. } => "missing_non_color_cue_policy",
            Self::SurfaceNotRegistered { .. } => "surface_not_registered",
        }
    }

    /// Returns the surface id this finding is attached to.
    pub fn surface_id(&self) -> &str {
        match self {
            Self::UnknownTokenGap { surface_id, .. }
            | Self::UnregisteredLocalState { surface_id, .. }
            | Self::TokenGroupDrift { surface_id, .. }
            | Self::TokenRefDrift { surface_id, .. }
            | Self::StyleProvenanceDrift { surface_id, .. }
            | Self::CuePolicyDrift { surface_id, .. }
            | Self::ColorOnlyCue { surface_id, .. }
            | Self::HardcodedThemeValue { surface_id, .. }
            | Self::UnresolvedTokenFallback { surface_id, .. }
            | Self::MissingRegistryAnchor { surface_id, .. }
            | Self::OverrideDrift { surface_id, .. }
            | Self::MissingNarrowingReason { surface_id, .. }
            | Self::MissingProjection { surface_id, .. }
            | Self::DescriptorMissingRegistryAnchor { surface_id }
            | Self::MissingAccessibilityNote { surface_id }
            | Self::MissingNonColorCuePolicy { surface_id }
            | Self::SurfaceNotRegistered { surface_id } => surface_id,
        }
    }

    /// Returns the state this finding is attached to, when state-scoped.
    pub fn state(&self) -> Option<M5NormalizedState> {
        match self {
            Self::UnknownTokenGap { state, .. }
            | Self::UnregisteredLocalState { state, .. }
            | Self::TokenGroupDrift { state, .. }
            | Self::TokenRefDrift { state, .. }
            | Self::StyleProvenanceDrift { state, .. }
            | Self::CuePolicyDrift { state, .. }
            | Self::ColorOnlyCue { state, .. }
            | Self::HardcodedThemeValue { state, .. }
            | Self::UnresolvedTokenFallback { state, .. }
            | Self::MissingRegistryAnchor { state, .. }
            | Self::OverrideDrift { state, .. }
            | Self::MissingNarrowingReason { state, .. }
            | Self::MissingProjection { state, .. } => Some(*state),
            Self::DescriptorMissingRegistryAnchor { .. }
            | Self::MissingAccessibilityNote { .. }
            | Self::MissingNonColorCuePolicy { .. }
            | Self::SurfaceNotRegistered { .. } => None,
        }
    }
}

/// One per-surface parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentStateRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the surface.
    pub descriptor: M5ComponentStateDescriptor,
    /// State-by-state bindings, in canonical state order.
    pub bindings: Vec<M5StateBinding>,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<M5ParityBlockingFinding>,
    /// `true` when the surface's descriptor classifies it as high-salience.
    pub high_salience: bool,
}

/// Per-class blocking-finding count summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ParityFindingSummary {
    /// Total blocking findings across the audit.
    pub total_blocking_findings: usize,
    /// Number of `unknown_token_gap` findings.
    pub unknown_token_gap: usize,
    /// Number of `unregistered_local_state` findings.
    pub unregistered_local_state: usize,
    /// Number of `token_group_drift` findings.
    pub token_group_drift: usize,
    /// Number of `token_ref_drift` findings.
    pub token_ref_drift: usize,
    /// Number of `style_provenance_drift` findings.
    pub style_provenance_drift: usize,
    /// Number of `cue_policy_drift` findings.
    pub cue_policy_drift: usize,
    /// Number of `color_only_cue` findings.
    pub color_only_cue: usize,
    /// Number of `hardcoded_theme_value` findings.
    pub hardcoded_theme_value: usize,
    /// Number of `unresolved_token_fallback` findings.
    pub unresolved_token_fallback: usize,
    /// Number of `missing_registry_anchor` findings.
    pub missing_registry_anchor: usize,
    /// Number of `override_drift` findings.
    pub override_drift: usize,
    /// Number of `missing_narrowing_reason` findings.
    pub missing_narrowing_reason: usize,
    /// Number of `missing_projection` findings.
    pub missing_projection: usize,
    /// Number of `descriptor_missing_registry_anchor` findings.
    pub descriptor_missing_registry_anchor: usize,
    /// Number of `missing_accessibility_note` findings.
    pub missing_accessibility_note: usize,
    /// Number of `missing_non_color_cue_policy` findings.
    pub missing_non_color_cue_policy: usize,
    /// Number of `surface_not_registered` findings.
    pub surface_not_registered: usize,
}

impl M5ParityFindingSummary {
    fn empty() -> Self {
        Self {
            total_blocking_findings: 0,
            unknown_token_gap: 0,
            unregistered_local_state: 0,
            token_group_drift: 0,
            token_ref_drift: 0,
            style_provenance_drift: 0,
            cue_policy_drift: 0,
            color_only_cue: 0,
            hardcoded_theme_value: 0,
            unresolved_token_fallback: 0,
            missing_registry_anchor: 0,
            override_drift: 0,
            missing_narrowing_reason: 0,
            missing_projection: 0,
            descriptor_missing_registry_anchor: 0,
            missing_accessibility_note: 0,
            missing_non_color_cue_policy: 0,
            surface_not_registered: 0,
        }
    }

    fn record(&mut self, finding: &M5ParityBlockingFinding) {
        self.total_blocking_findings += 1;
        match finding {
            M5ParityBlockingFinding::UnknownTokenGap { .. } => self.unknown_token_gap += 1,
            M5ParityBlockingFinding::UnregisteredLocalState { .. } => {
                self.unregistered_local_state += 1
            }
            M5ParityBlockingFinding::TokenGroupDrift { .. } => self.token_group_drift += 1,
            M5ParityBlockingFinding::TokenRefDrift { .. } => self.token_ref_drift += 1,
            M5ParityBlockingFinding::StyleProvenanceDrift { .. } => {
                self.style_provenance_drift += 1
            }
            M5ParityBlockingFinding::CuePolicyDrift { .. } => self.cue_policy_drift += 1,
            M5ParityBlockingFinding::ColorOnlyCue { .. } => self.color_only_cue += 1,
            M5ParityBlockingFinding::HardcodedThemeValue { .. } => self.hardcoded_theme_value += 1,
            M5ParityBlockingFinding::UnresolvedTokenFallback { .. } => {
                self.unresolved_token_fallback += 1
            }
            M5ParityBlockingFinding::MissingRegistryAnchor { .. } => {
                self.missing_registry_anchor += 1
            }
            M5ParityBlockingFinding::OverrideDrift { .. } => self.override_drift += 1,
            M5ParityBlockingFinding::MissingNarrowingReason { .. } => {
                self.missing_narrowing_reason += 1
            }
            M5ParityBlockingFinding::MissingProjection { .. } => self.missing_projection += 1,
            M5ParityBlockingFinding::DescriptorMissingRegistryAnchor { .. } => {
                self.descriptor_missing_registry_anchor += 1
            }
            M5ParityBlockingFinding::MissingAccessibilityNote { .. } => {
                self.missing_accessibility_note += 1
            }
            M5ParityBlockingFinding::MissingNonColorCuePolicy { .. } => {
                self.missing_non_color_cue_policy += 1
            }
            M5ParityBlockingFinding::SurfaceNotRegistered { .. } => {
                self.surface_not_registered += 1
            }
        }
    }
}

/// Per-state coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateCoverageSummary {
    /// State this summary covers.
    pub state: M5NormalizedState,
    /// Number of `inherited` rows on this state.
    pub inherited_rows: usize,
    /// Number of `explicitly_narrowed` rows on this state.
    pub explicitly_narrowed_rows: usize,
    /// Number of `not_applicable` rows on this state.
    pub not_applicable_rows: usize,
    /// Number of `declared_inheritance_gap` rows on this state.
    pub declared_inheritance_gap_rows: usize,
    /// Number of `platform_omitted` rows on this state.
    pub platform_omitted_rows: usize,
    /// Number of `unregistered_local_state` rows on this state.
    pub unregistered_local_state_rows: usize,
    /// Number of `unknown_token_gap` rows on this state.
    pub unknown_token_gap_rows: usize,
}

impl M5StateCoverageSummary {
    fn narrowed_rows(&self) -> usize {
        self.explicitly_narrowed_rows
            + self.not_applicable_rows
            + self.declared_inheritance_gap_rows
            + self.platform_omitted_rows
    }
}

/// A single registry-anchor index entry the audit publishes so design QA
/// and docs surfaces can reopen each M5 surface by its registry anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RegistryAnchorEntry {
    /// Surface family the anchor belongs to.
    pub surface_family: M5SurfaceFamily,
    /// Surface id the anchor reopens.
    pub surface_id: String,
    /// Canonical registry anchor ref.
    pub registry_anchor_ref: String,
}

/// M5 component-state and token-inheritance audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentStateAuditReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Required normalized states, in canonical order.
    pub required_states: Vec<M5NormalizedState>,
    /// Per-surface parity rows, sorted by `descriptor.surface_id`.
    pub rows: Vec<M5ComponentStateRow>,
    /// Per-state coverage summary, in canonical state order.
    pub state_coverage: Vec<M5StateCoverageSummary>,
    /// Per-class blocking-finding summary.
    pub findings_summary: M5ParityFindingSummary,
    /// Canonical registry-anchor index, sorted by surface id.
    pub registry_anchor_index: Vec<M5RegistryAnchorEntry>,
    /// Number of registered M5 surfaces present.
    pub registered_surface_count: usize,
    /// Number of high-salience surfaces present.
    pub high_salience_surface_count: usize,
    /// Total state bindings checked.
    pub state_bindings_checked: usize,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this audit is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the audit can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the audit can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the audit was generated.
    pub generated_at: String,
}

impl M5ComponentStateAuditReport {
    /// Returns `true` when every required state is inherited by at least
    /// one surface.
    pub fn every_required_state_inherited(&self) -> bool {
        for state in M5NormalizedState::required_states() {
            let any_inherited = self.rows.iter().any(|row| {
                row.bindings.iter().any(|binding| {
                    binding.state == state && binding.binding_status == M5BindingStatus::Inherited
                })
            });
            if !any_inherited {
                return false;
            }
        }
        true
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "audit: surfaces={}, high_salience={}, bindings={}, blocking={}, clean={}",
            self.registered_surface_count,
            self.high_salience_surface_count,
            self.state_bindings_checked,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for state in &self.state_coverage {
            lines.push(format!(
                "{}: inherited={}, narrowed={}, unregistered={}, unknown_token={}",
                state.state.display_label(),
                state.inherited_rows,
                state.narrowed_rows(),
                state.unregistered_local_state_rows,
                state.unknown_token_gap_rows,
            ));
        }
        for row in &self.rows {
            for finding in &row.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {} -- {}",
                    finding.class_token(),
                    finding.surface_id(),
                    finding
                        .state()
                        .map(M5NormalizedState::as_str)
                        .unwrap_or("surface"),
                ));
            }
        }
        lines
    }

    /// Renders the markdown audit artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 component-state and design-token inheritance audit\n");
        out.push('\n');
        out.push_str(
            "Generated from the seeded audit in\n\
             [`crate::m5_component_registry`](../../../../crates/aureline-shell/src/m5_component_registry/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_component_state -- report-md > \\\n  artifacts/ux/m5/component-state-audit/m5_component_state_audit.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Registered M5 surfaces: `{}`\n",
            self.registered_surface_count
        ));
        out.push_str(&format!(
            "- High-salience surfaces: `{}`\n",
            self.high_salience_surface_count
        ));
        out.push_str(&format!(
            "- State bindings checked: `{}`\n",
            self.state_bindings_checked
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Per-state coverage\n\n");
        out.push_str(
            "| State | Inherited | Narrowed | Unregistered | Unknown token |\n\
             | ----- | --------: | -------: | -----------: | ------------: |\n",
        );
        for state in &self.state_coverage {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                state.state.display_label(),
                state.inherited_rows,
                state.narrowed_rows(),
                state.unregistered_local_state_rows,
                state.unknown_token_gap_rows,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `unknown_token_gap` | {} |\n",
            self.findings_summary.unknown_token_gap
        ));
        out.push_str(&format!(
            "| `unregistered_local_state` | {} |\n",
            self.findings_summary.unregistered_local_state
        ));
        out.push_str(&format!(
            "| `token_group_drift` | {} |\n",
            self.findings_summary.token_group_drift
        ));
        out.push_str(&format!(
            "| `token_ref_drift` | {} |\n",
            self.findings_summary.token_ref_drift
        ));
        out.push_str(&format!(
            "| `style_provenance_drift` | {} |\n",
            self.findings_summary.style_provenance_drift
        ));
        out.push_str(&format!(
            "| `cue_policy_drift` | {} |\n",
            self.findings_summary.cue_policy_drift
        ));
        out.push_str(&format!(
            "| `color_only_cue` | {} |\n",
            self.findings_summary.color_only_cue
        ));
        out.push_str(&format!(
            "| `hardcoded_theme_value` | {} |\n",
            self.findings_summary.hardcoded_theme_value
        ));
        out.push_str(&format!(
            "| `unresolved_token_fallback` | {} |\n",
            self.findings_summary.unresolved_token_fallback
        ));
        out.push_str(&format!(
            "| `missing_registry_anchor` | {} |\n",
            self.findings_summary.missing_registry_anchor
        ));
        out.push_str(&format!(
            "| `override_drift` | {} |\n",
            self.findings_summary.override_drift
        ));
        out.push_str(&format!(
            "| `missing_narrowing_reason` | {} |\n",
            self.findings_summary.missing_narrowing_reason
        ));
        out.push_str(&format!(
            "| `missing_projection` | {} |\n",
            self.findings_summary.missing_projection
        ));
        out.push_str(&format!(
            "| `descriptor_missing_registry_anchor` | {} |\n",
            self.findings_summary.descriptor_missing_registry_anchor
        ));
        out.push_str(&format!(
            "| `missing_accessibility_note` | {} |\n",
            self.findings_summary.missing_accessibility_note
        ));
        out.push_str(&format!(
            "| `missing_non_color_cue_policy` | {} |\n",
            self.findings_summary.missing_non_color_cue_policy
        ));
        out.push_str(&format!(
            "| `surface_not_registered` | {} |\n\n",
            self.findings_summary.surface_not_registered
        ));

        out.push_str("## Registry anchor index\n\n");
        out.push_str(
            "| Surface family | Surface | Registry anchor |\n| -------------- | ------- | --------------- |\n",
        );
        for entry in &self.registry_anchor_index {
            out.push_str(&format!(
                "| {} | `{}` | `{}` |\n",
                entry.surface_family.display_label(),
                entry.surface_id,
                entry.registry_anchor_ref,
            ));
        }
        out.push('\n');

        out.push_str("## Per-surface rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "### `{}` ({}, {})\n\n",
                row.descriptor.surface_id,
                row.descriptor.surface_family.as_str(),
                row.descriptor.lifecycle_label.as_str()
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                row.descriptor.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Token group: `{}`\n",
                row.descriptor.token_group.as_str()
            ));
            out.push_str(&format!(
                "- Style provenance: `{}`\n",
                row.descriptor.style_provenance.as_str()
            ));
            out.push_str(&format!(
                "- Semantic salience: `{}`\n",
                row.descriptor.semantic_salience.as_str()
            ));
            out.push_str(&format!(
                "- Cue policy: `{}`\n",
                row.descriptor.cue_policy.as_str()
            ));
            out.push_str(&format!(
                "- Registry anchor: `{}`\n",
                row.descriptor.registry_anchor_ref
            ));
            out.push_str(&format!(
                "- High-salience: `{}`\n\n",
                if row.high_salience { "yes" } else { "no" }
            ));

            out.push_str(
                "| State | Status | Token ref | Cue | Provenance | Narrowing reason |\n\
                 | ----- | ------ | --------- | --- | ---------- | ---------------- |\n",
            );
            for binding in &row.bindings {
                let token = binding.projected_token_ref.as_deref().unwrap_or("-");
                let cue = binding
                    .projected_non_color_cue
                    .map(|cue| cue.as_str())
                    .unwrap_or("-");
                let provenance = binding
                    .projected_style_provenance
                    .map(|prov| prov.as_str())
                    .unwrap_or("-");
                let narrowing = binding.narrowing_reason.as_deref().unwrap_or("-");
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` | `{}` | {} |\n",
                    binding.state.display_label(),
                    binding.binding_status.as_str(),
                    token,
                    cue,
                    provenance,
                    narrowing,
                ));
            }
            out.push('\n');

            if row.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &row.blocking_findings {
                    out.push_str(&format!(
                        "- `{}` on `{}`\n",
                        finding.class_token(),
                        finding
                            .state()
                            .map(M5NormalizedState::as_str)
                            .unwrap_or("surface"),
                    ));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_component_state -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_component_state_fixtures\n");
        out.push_str("python3 tools/ci/m5/component_state_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the M5 component-state audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComponentStateSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Audit report quoted in full.
    pub report: M5ComponentStateAuditReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl M5ComponentStateSupportExport {
    /// Builds the support-export wrapper for an audit report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: M5ComponentStateAuditReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for row in &report.rows {
            case_ids.push(row.descriptor.surface_id.clone());
            case_ids.push(row.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: M5_COMPONENT_STATE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_COMPONENT_STATE_SCHEMA_VERSION,
            shared_contract_ref: M5_COMPONENT_STATE_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Canonical token ref for a token group and normalized state.
///
/// Every inherited binding MUST project exactly this ref so a surface
/// cannot paint a state from a private token or a literal value.
pub fn canonical_token_ref(group: M5TokenGroup, state: M5NormalizedState) -> String {
    format!("token:{}:{}", group.as_str(), state.as_str())
}

/// Computes the per-row blocking findings from a descriptor and its state
/// bindings.
fn compute_row_findings(
    descriptor: &M5ComponentStateDescriptor,
    bindings: &[M5StateBinding],
    high_salience: bool,
) -> Vec<M5ParityBlockingFinding> {
    let mut findings = Vec::new();

    // Descriptor-level (surface-scoped) findings.
    if descriptor.registry_anchor_ref.trim().is_empty() {
        findings.push(M5ParityBlockingFinding::DescriptorMissingRegistryAnchor {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if descriptor.accessibility_note.trim().is_empty() {
        findings.push(M5ParityBlockingFinding::MissingAccessibilityNote {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if high_salience && descriptor.cue_policy == M5CuePolicy::ColorAllowed {
        findings.push(M5ParityBlockingFinding::MissingNonColorCuePolicy {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if !descriptor.registered_in_shared_registry {
        findings.push(M5ParityBlockingFinding::SurfaceNotRegistered {
            surface_id: descriptor.surface_id.clone(),
        });
    }

    let documented_overrides: BTreeSet<&str> = descriptor
        .documented_overrides
        .iter()
        .map(String::as_str)
        .collect();

    for binding in bindings {
        let state = binding.state;

        // A hard-coded value or unresolved fallback is a blocker on any
        // binding regardless of status.
        if let Some(value) = &binding.hardcoded_value {
            findings.push(M5ParityBlockingFinding::HardcodedThemeValue {
                surface_id: descriptor.surface_id.clone(),
                state,
                hardcoded_value: value.clone(),
            });
        }
        if let Some(fallback) = &binding.unresolved_token_fallback {
            findings.push(M5ParityBlockingFinding::UnresolvedTokenFallback {
                surface_id: descriptor.surface_id.clone(),
                state,
                unresolved_token_fallback: fallback.clone(),
            });
        }

        match binding.binding_status {
            M5BindingStatus::UnknownTokenGap => {
                if high_salience {
                    findings.push(M5ParityBlockingFinding::UnknownTokenGap {
                        surface_id: descriptor.surface_id.clone(),
                        state,
                    });
                }
            }
            M5BindingStatus::UnregisteredLocalState => {
                findings.push(M5ParityBlockingFinding::UnregisteredLocalState {
                    surface_id: descriptor.surface_id.clone(),
                    state,
                });
            }
            M5BindingStatus::Inherited => {
                match binding.projected_token_group {
                    Some(group) if group == descriptor.token_group => {}
                    Some(group) => findings.push(M5ParityBlockingFinding::TokenGroupDrift {
                        surface_id: descriptor.surface_id.clone(),
                        state,
                        projected_token_group: group,
                    }),
                    None => findings.push(M5ParityBlockingFinding::MissingProjection {
                        surface_id: descriptor.surface_id.clone(),
                        state,
                        field: "projected_token_group".to_owned(),
                    }),
                }

                let expected_token_ref = canonical_token_ref(descriptor.token_group, state);
                match &binding.projected_token_ref {
                    Some(token) if token == &expected_token_ref => {}
                    Some(token) => findings.push(M5ParityBlockingFinding::TokenRefDrift {
                        surface_id: descriptor.surface_id.clone(),
                        state,
                        projected_token_ref: token.clone(),
                    }),
                    None => findings.push(M5ParityBlockingFinding::MissingProjection {
                        surface_id: descriptor.surface_id.clone(),
                        state,
                        field: "projected_token_ref".to_owned(),
                    }),
                }

                match binding.projected_style_provenance {
                    Some(prov) if prov == descriptor.style_provenance => {}
                    Some(prov) => findings.push(M5ParityBlockingFinding::StyleProvenanceDrift {
                        surface_id: descriptor.surface_id.clone(),
                        state,
                        projected_style_provenance: prov,
                    }),
                    None => findings.push(M5ParityBlockingFinding::MissingProjection {
                        surface_id: descriptor.surface_id.clone(),
                        state,
                        field: "projected_style_provenance".to_owned(),
                    }),
                }

                match binding.projected_cue_policy {
                    Some(policy) if policy == descriptor.cue_policy => {}
                    Some(policy) => findings.push(M5ParityBlockingFinding::CuePolicyDrift {
                        surface_id: descriptor.surface_id.clone(),
                        state,
                        projected_cue_policy: policy,
                    }),
                    None => findings.push(M5ParityBlockingFinding::MissingProjection {
                        surface_id: descriptor.surface_id.clone(),
                        state,
                        field: "projected_cue_policy".to_owned(),
                    }),
                }

                match binding.projected_non_color_cue {
                    Some(M5AccessibilityCue::ColorOnly) => {
                        if high_salience
                            || descriptor.cue_policy == M5CuePolicy::NonColorCueRequired
                        {
                            findings.push(M5ParityBlockingFinding::ColorOnlyCue {
                                surface_id: descriptor.surface_id.clone(),
                                state,
                            });
                        }
                    }
                    Some(_) => {}
                    None => findings.push(M5ParityBlockingFinding::MissingProjection {
                        surface_id: descriptor.surface_id.clone(),
                        state,
                        field: "projected_non_color_cue".to_owned(),
                    }),
                }

                match &binding.projected_registry_anchor_ref {
                    Some(anchor) if anchor == &descriptor.registry_anchor_ref => {}
                    Some(_) | None => {
                        findings.push(M5ParityBlockingFinding::MissingRegistryAnchor {
                            surface_id: descriptor.surface_id.clone(),
                            state,
                        });
                    }
                }

                for applied in &binding.applied_overrides {
                    if !documented_overrides.contains(applied.as_str()) {
                        findings.push(M5ParityBlockingFinding::OverrideDrift {
                            surface_id: descriptor.surface_id.clone(),
                            state,
                            offending_override: applied.clone(),
                        });
                    }
                }
            }
            status if status.requires_narrowing_reason() => {
                let reason_ok = binding
                    .narrowing_reason
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    == Some(false);
                if !reason_ok {
                    findings.push(M5ParityBlockingFinding::MissingNarrowingReason {
                        surface_id: descriptor.surface_id.clone(),
                        state,
                        binding_status: status,
                    });
                }
            }
            _ => {}
        }
    }
    findings
}

/// Computes the per-state and per-class summaries from finished rows.
fn summarize_report(
    rows: &[M5ComponentStateRow],
) -> (Vec<M5StateCoverageSummary>, M5ParityFindingSummary) {
    let mut summary = M5ParityFindingSummary::empty();
    let mut coverage: Vec<M5StateCoverageSummary> = M5NormalizedState::required_states()
        .into_iter()
        .map(|state| M5StateCoverageSummary {
            state,
            inherited_rows: 0,
            explicitly_narrowed_rows: 0,
            not_applicable_rows: 0,
            declared_inheritance_gap_rows: 0,
            platform_omitted_rows: 0,
            unregistered_local_state_rows: 0,
            unknown_token_gap_rows: 0,
        })
        .collect();

    for row in rows {
        for binding in &row.bindings {
            if let Some(coverage_row) = coverage
                .iter_mut()
                .find(|coverage| coverage.state == binding.state)
            {
                match binding.binding_status {
                    M5BindingStatus::Inherited => coverage_row.inherited_rows += 1,
                    M5BindingStatus::ExplicitlyNarrowed => {
                        coverage_row.explicitly_narrowed_rows += 1
                    }
                    M5BindingStatus::NotApplicable => coverage_row.not_applicable_rows += 1,
                    M5BindingStatus::DeclaredInheritanceGap => {
                        coverage_row.declared_inheritance_gap_rows += 1
                    }
                    M5BindingStatus::PlatformOmitted => coverage_row.platform_omitted_rows += 1,
                    M5BindingStatus::UnregisteredLocalState => {
                        coverage_row.unregistered_local_state_rows += 1
                    }
                    M5BindingStatus::UnknownTokenGap => coverage_row.unknown_token_gap_rows += 1,
                }
            }
        }
        for finding in &row.blocking_findings {
            summary.record(finding);
        }
    }

    (coverage, summary)
}

/// Builds an [`M5ComponentStateRow`] from a descriptor and its state
/// bindings, computing the per-row blocking findings.
pub fn build_m5_component_state_row(
    descriptor: M5ComponentStateDescriptor,
    bindings: Vec<M5StateBinding>,
) -> M5ComponentStateRow {
    let high_salience = descriptor.is_high_salience();
    let blocking_findings = compute_row_findings(&descriptor, &bindings, high_salience);

    M5ComponentStateRow {
        record_kind: M5_COMPONENT_STATE_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_COMPONENT_STATE_SCHEMA_VERSION,
        shared_contract_ref: M5_COMPONENT_STATE_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        bindings,
        blocking_findings,
        high_salience,
    }
}

/// Builds a full [`M5ComponentStateAuditReport`] from per-surface rows.
pub fn build_m5_component_state_audit(
    rows: Vec<M5ComponentStateRow>,
) -> M5ComponentStateAuditReport {
    let mut rows = rows;
    rows.sort_by(|left, right| left.descriptor.surface_id.cmp(&right.descriptor.surface_id));

    let registered_surface_count = rows.len();
    let high_salience_surface_count = rows.iter().filter(|row| row.high_salience).count();
    let state_bindings_checked = rows.iter().map(|row| row.bindings.len()).sum::<usize>();

    let (state_coverage, findings_summary) = summarize_report(&rows);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut registry_anchor_index: Vec<M5RegistryAnchorEntry> = rows
        .iter()
        .map(|row| M5RegistryAnchorEntry {
            surface_family: row.descriptor.surface_family,
            surface_id: row.descriptor.surface_id.clone(),
            registry_anchor_ref: row.descriptor.registry_anchor_ref.clone(),
        })
        .collect();
    registry_anchor_index.sort_by(|left, right| left.surface_id.cmp(&right.surface_id));

    M5ComponentStateAuditReport {
        record_kind: M5_COMPONENT_STATE_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_COMPONENT_STATE_SCHEMA_VERSION,
        shared_contract_ref: M5_COMPONENT_STATE_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_COMPONENT_STATE_REPORT_ID.to_owned(),
        source_schema_ref: M5_COMPONENT_STATE_SOURCE_SCHEMA_REF.to_owned(),
        required_states: M5NormalizedState::required_states().to_vec(),
        rows,
        state_coverage,
        findings_summary,
        registry_anchor_index,
        registered_surface_count,
        high_salience_surface_count,
        state_bindings_checked,
        report_clean,
        published_report_ref: M5_COMPONENT_STATE_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: M5_COMPONENT_STATE_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            M5_COMPONENT_STATE_PUBLISHED_DOC_REF.to_owned(),
            "docs/ux/m5/command_parity_audit.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-component-state".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_m5_component_state_audit`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum M5ComponentStateValidationError {
    /// The audit has no registered surfaces.
    NoRegisteredSurfaces,
    /// A required state has no inherited row.
    RequiredStateNotInherited { state: String },
    /// A row is missing a required state from its binding set.
    MissingRequiredState { surface_id: String, state: String },
    /// A blocking finding remains on the row.
    BlockingFindingPresent {
        surface_id: String,
        state: String,
        class: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// A row's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { surface_id: String },
}

/// Validates an audit report against the M5 acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_m5_component_state_audit(
    report: &M5ComponentStateAuditReport,
) -> Result<(), Vec<M5ComponentStateValidationError>> {
    let mut errors = Vec::new();

    if report.rows.is_empty() {
        errors.push(M5ComponentStateValidationError::NoRegisteredSurfaces);
    }

    for required in M5NormalizedState::required_states() {
        let any_inherited = report.rows.iter().any(|row| {
            row.bindings.iter().any(|binding| {
                binding.state == required && binding.binding_status == M5BindingStatus::Inherited
            })
        });
        if !any_inherited {
            errors.push(M5ComponentStateValidationError::RequiredStateNotInherited {
                state: required.as_str().to_owned(),
            });
        }
    }

    for row in &report.rows {
        for required in M5NormalizedState::required_states() {
            if !row.bindings.iter().any(|binding| binding.state == required) {
                errors.push(M5ComponentStateValidationError::MissingRequiredState {
                    surface_id: row.descriptor.surface_id.clone(),
                    state: required.as_str().to_owned(),
                });
            }
        }
        if row.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(
                M5ComponentStateValidationError::MissingDescriptorRevisionRef {
                    surface_id: row.descriptor.surface_id.clone(),
                },
            );
        }
        for finding in &row.blocking_findings {
            errors.push(M5ComponentStateValidationError::BlockingFindingPresent {
                surface_id: finding.surface_id().to_owned(),
                state: finding
                    .state()
                    .map(|state| state.as_str().to_owned())
                    .unwrap_or_else(|| "surface".to_owned()),
                class: finding.class_token().to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(M5ComponentStateValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(M5ComponentStateValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_m5_component_state_audit`].
struct SurfaceSeed {
    surface_id: &'static str,
    surface_family: M5SurfaceFamily,
    descriptor_revision_ref: &'static str,
    primary_label_ref: &'static str,
    registry_anchor_ref: &'static str,
    accessibility_note: &'static str,
    token_group: M5TokenGroup,
    style_provenance: M5StyleProvenance,
    semantic_salience: M5SemanticSalience,
    cue_policy: M5CuePolicy,
    lifecycle_label: M5SurfaceLifecycle,
    documented_overrides: &'static [&'static str],
    bindings: &'static [BindingSeed],
}

struct BindingSeed {
    state: M5NormalizedState,
    binding_status: M5BindingStatus,
    non_color_cue: M5AccessibilityCue,
    narrowing_reason: Option<&'static str>,
    note: Option<&'static str>,
    applied_overrides: &'static [&'static str],
}

/// Helper: an inherited binding with the given non-color cue.
const fn inherited(state: M5NormalizedState, cue: M5AccessibilityCue) -> BindingSeed {
    BindingSeed {
        state,
        binding_status: M5BindingStatus::Inherited,
        non_color_cue: cue,
        narrowing_reason: None,
        note: None,
        applied_overrides: &[],
    }
}

/// Helper: a not-applicable binding with a documented reason.
const fn not_applicable(state: M5NormalizedState, reason: &'static str) -> BindingSeed {
    BindingSeed {
        state,
        binding_status: M5BindingStatus::NotApplicable,
        non_color_cue: M5AccessibilityCue::TextLabel,
        narrowing_reason: Some(reason),
        note: None,
        applied_overrides: &[],
    }
}

/// Helper: an honestly-declared inheritance gap with a documented reason.
const fn declared_gap(state: M5NormalizedState, reason: &'static str) -> BindingSeed {
    BindingSeed {
        state,
        binding_status: M5BindingStatus::DeclaredInheritanceGap,
        non_color_cue: M5AccessibilityCue::TextLabel,
        narrowing_reason: Some(reason),
        note: None,
        applied_overrides: &[],
    }
}

const SURFACE_SEEDS: &[SurfaceSeed] = &[
    // Notebook cell chrome. Lifecycle-bearing; inherits chrome tokens.
    SurfaceSeed {
        surface_id: "surface:notebook.cell_chrome",
        surface_family: M5SurfaceFamily::NotebookCellChrome,
        descriptor_revision_ref: "surface-rev:notebook.cell_chrome:2026.06.01-01",
        primary_label_ref: "label:notebook.cell_chrome:primary",
        registry_anchor_ref: "registry:anchor:notebook:cell_chrome",
        accessibility_note: "Cell run, error, and stale states carry an icon and text, never color alone.",
        token_group: M5TokenGroup::SurfaceChromeTokens,
        style_provenance: M5StyleProvenance::ShellTokenInherited,
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        cue_policy: M5CuePolicy::NonColorCueRequired,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        documented_overrides: &[],
        bindings: &[
            inherited(M5NormalizedState::Loading, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Cached, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::Stale, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Partial, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::PolicyBlocked, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Degraded, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::PreviewOnly, M5AccessibilityCue::ShapeOrPattern),
            not_applicable(
                M5NormalizedState::SyncPending,
                "notebook_cells_sync_through_document_not_cell_chrome",
            ),
            inherited(
                M5NormalizedState::BoundaryHandoff,
                M5AccessibilityCue::IconAndText,
            ),
        ],
    },
    // Result-grid rows. Lifecycle-bearing; dense data tokens.
    SurfaceSeed {
        surface_id: "surface:data_api.result_grid_row",
        surface_family: M5SurfaceFamily::ResultGridRow,
        descriptor_revision_ref: "surface-rev:data_api.result_grid_row:2026.06.01-01",
        primary_label_ref: "label:data_api.result_grid_row:primary",
        registry_anchor_ref: "registry:anchor:data_api:result_grid_row",
        accessibility_note: "Cached, stale, and partial rows carry a row-level badge with text, never tint alone.",
        token_group: M5TokenGroup::DataDensityTokens,
        style_provenance: M5StyleProvenance::ShellTokenInherited,
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        cue_policy: M5CuePolicy::NonColorCueRequired,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        documented_overrides: &["override:data_api.result_grid_row:compact_density"],
        bindings: &[
            inherited(M5NormalizedState::Loading, M5AccessibilityCue::ShapeOrPattern),
            inherited(M5NormalizedState::Cached, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Stale, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Partial, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::PolicyBlocked, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Degraded, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::PreviewOnly, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::SyncPending, M5AccessibilityCue::IconAndText),
            not_applicable(
                M5NormalizedState::BoundaryHandoff,
                "result_rows_do_not_cross_an_embedded_boundary",
            ),
        ],
    },
    // Profiler panel. Informational; diagnostic tokens; color allowed.
    SurfaceSeed {
        surface_id: "surface:profiler.capture_panel",
        surface_family: M5SurfaceFamily::ProfilerPanel,
        descriptor_revision_ref: "surface-rev:profiler.capture_panel:2026.06.01-01",
        primary_label_ref: "label:profiler.capture_panel:primary",
        registry_anchor_ref: "registry:anchor:profiler:capture_panel",
        accessibility_note: "Diagnostic intensities are token-driven; severity is also labelled in the legend.",
        token_group: M5TokenGroup::DiagnosticStateTokens,
        style_provenance: M5StyleProvenance::ShellTokenInherited,
        semantic_salience: M5SemanticSalience::Informational,
        cue_policy: M5CuePolicy::ColorAllowed,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        documented_overrides: &[],
        bindings: &[
            inherited(M5NormalizedState::Loading, M5AccessibilityCue::ShapeOrPattern),
            inherited(M5NormalizedState::Cached, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::Stale, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::Partial, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::PolicyBlocked, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Degraded, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::PreviewOnly, M5AccessibilityCue::TextLabel),
            not_applicable(
                M5NormalizedState::SyncPending,
                "captures_are_local_until_explicitly_exported",
            ),
            not_applicable(
                M5NormalizedState::BoundaryHandoff,
                "profiler_capture_does_not_hand_off_across_a_boundary",
            ),
        ],
    },
    // Trace panel. Informational; diagnostic tokens; color allowed.
    SurfaceSeed {
        surface_id: "surface:trace.replay_panel",
        surface_family: M5SurfaceFamily::TracePanel,
        descriptor_revision_ref: "surface-rev:trace.replay_panel:2026.06.01-01",
        primary_label_ref: "label:trace.replay_panel:primary",
        registry_anchor_ref: "registry:anchor:trace:replay_panel",
        accessibility_note: "Replay timeline states carry a text label in addition to the token tint.",
        token_group: M5TokenGroup::DiagnosticStateTokens,
        style_provenance: M5StyleProvenance::ShellTokenInherited,
        semantic_salience: M5SemanticSalience::Informational,
        cue_policy: M5CuePolicy::ColorAllowed,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        documented_overrides: &[],
        bindings: &[
            inherited(M5NormalizedState::Loading, M5AccessibilityCue::ShapeOrPattern),
            inherited(M5NormalizedState::Cached, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::Stale, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::Partial, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::PolicyBlocked, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Degraded, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::PreviewOnly, M5AccessibilityCue::TextLabel),
            not_applicable(
                M5NormalizedState::SyncPending,
                "trace_sessions_are_local_until_exported",
            ),
            inherited(
                M5NormalizedState::BoundaryHandoff,
                M5AccessibilityCue::IconAndText,
            ),
        ],
    },
    // Pipeline card. Severity-bearing; pipeline status tokens.
    SurfaceSeed {
        surface_id: "surface:review.pipeline_card",
        surface_family: M5SurfaceFamily::PipelineCard,
        descriptor_revision_ref: "surface-rev:review.pipeline_card:2026.06.01-01",
        primary_label_ref: "label:review.pipeline_card:primary",
        registry_anchor_ref: "registry:anchor:review:pipeline_card",
        accessibility_note: "Pass, fail, and blocked states carry a status icon and text, never color alone.",
        token_group: M5TokenGroup::PipelineStatusTokens,
        style_provenance: M5StyleProvenance::ShellTokenInherited,
        semantic_salience: M5SemanticSalience::SeverityBearing,
        cue_policy: M5CuePolicy::NonColorCueRequired,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        documented_overrides: &[],
        bindings: &[
            inherited(M5NormalizedState::Loading, M5AccessibilityCue::ShapeOrPattern),
            inherited(M5NormalizedState::Cached, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::Stale, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Partial, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::PolicyBlocked, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Degraded, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::PreviewOnly, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::SyncPending, M5AccessibilityCue::IconAndText),
            inherited(
                M5NormalizedState::BoundaryHandoff,
                M5AccessibilityCue::IconAndText,
            ),
        ],
    },
    // Preview-route badge. Lifecycle-bearing; preview badge tokens.
    SurfaceSeed {
        surface_id: "surface:preview.route_badge",
        surface_family: M5SurfaceFamily::PreviewRouteBadge,
        descriptor_revision_ref: "surface-rev:preview.route_badge:2026.06.01-01",
        primary_label_ref: "label:preview.route_badge:primary",
        registry_anchor_ref: "registry:anchor:preview:route_badge",
        accessibility_note: "Preview-only and stale badges spell out the state in text beside the token tint.",
        token_group: M5TokenGroup::PreviewBadgeTokens,
        style_provenance: M5StyleProvenance::ShellTokenInherited,
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        cue_policy: M5CuePolicy::NonColorCueRequired,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        documented_overrides: &[],
        bindings: &[
            inherited(M5NormalizedState::Loading, M5AccessibilityCue::ShapeOrPattern),
            inherited(M5NormalizedState::Cached, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::Stale, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Partial, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::PolicyBlocked, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Degraded, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::PreviewOnly, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::SyncPending, M5AccessibilityCue::IconAndText),
            inherited(
                M5NormalizedState::BoundaryHandoff,
                M5AccessibilityCue::IconAndText,
            ),
        ],
    },
    // Docs / browser pane. Informational; extension-contributed; declares gaps.
    SurfaceSeed {
        surface_id: "surface:docs_browser.pane",
        surface_family: M5SurfaceFamily::DocsBrowserPane,
        descriptor_revision_ref: "surface-rev:docs_browser.pane:2026.06.01-01",
        primary_label_ref: "label:docs_browser.pane:primary",
        registry_anchor_ref: "registry:anchor:docs_browser:pane",
        accessibility_note: "Embedded-content states inherit shell tokens; provider-only gaps are declared, not hidden.",
        token_group: M5TokenGroup::EmbeddedSurfaceTokens,
        style_provenance: M5StyleProvenance::ExtensionContributed,
        semantic_salience: M5SemanticSalience::Informational,
        cue_policy: M5CuePolicy::ColorAllowed,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        documented_overrides: &[],
        bindings: &[
            inherited(M5NormalizedState::Loading, M5AccessibilityCue::ShapeOrPattern),
            inherited(M5NormalizedState::Cached, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::Stale, M5AccessibilityCue::TextLabel),
            declared_gap(
                M5NormalizedState::Partial,
                "embedded_provider_renders_partial_content_without_a_shell_token_hook",
            ),
            inherited(M5NormalizedState::PolicyBlocked, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Degraded, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::PreviewOnly, M5AccessibilityCue::TextLabel),
            not_applicable(
                M5NormalizedState::SyncPending,
                "browsed_content_is_not_synced_by_the_workspace",
            ),
            inherited(
                M5NormalizedState::BoundaryHandoff,
                M5AccessibilityCue::IconAndText,
            ),
        ],
    },
    // Companion surface. Trust-bearing; provider-backed; declares gaps.
    SurfaceSeed {
        surface_id: "surface:companion.surface",
        surface_family: M5SurfaceFamily::CompanionSurface,
        descriptor_revision_ref: "surface-rev:companion.surface:2026.06.01-01",
        primary_label_ref: "label:companion.surface:primary",
        registry_anchor_ref: "registry:anchor:companion:surface",
        accessibility_note: "Presence and handoff states carry an icon and text; provider-only gaps are declared honestly.",
        token_group: M5TokenGroup::CompanionPresenceTokens,
        style_provenance: M5StyleProvenance::ProviderBacked,
        semantic_salience: M5SemanticSalience::TrustBearing,
        cue_policy: M5CuePolicy::NonColorCueRequired,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        documented_overrides: &[],
        bindings: &[
            inherited(M5NormalizedState::Loading, M5AccessibilityCue::ShapeOrPattern),
            inherited(M5NormalizedState::Cached, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::Stale, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Partial, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::PolicyBlocked, M5AccessibilityCue::IconAndText),
            declared_gap(
                M5NormalizedState::Degraded,
                "companion_provider_reports_degraded_link_without_a_shell_token_hook",
            ),
            inherited(M5NormalizedState::PreviewOnly, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::SyncPending, M5AccessibilityCue::IconAndText),
            inherited(
                M5NormalizedState::BoundaryHandoff,
                M5AccessibilityCue::IconAndText,
            ),
        ],
    },
    // Sync status surface. Severity-bearing; sync status tokens.
    SurfaceSeed {
        surface_id: "surface:sync.status_surface",
        surface_family: M5SurfaceFamily::SyncStatusSurface,
        descriptor_revision_ref: "surface-rev:sync.status_surface:2026.06.01-01",
        primary_label_ref: "label:sync.status_surface:primary",
        registry_anchor_ref: "registry:anchor:sync:status_surface",
        accessibility_note: "Sync-pending and conflict states carry an icon and text, never color alone.",
        token_group: M5TokenGroup::SyncStatusTokens,
        style_provenance: M5StyleProvenance::ShellTokenInherited,
        semantic_salience: M5SemanticSalience::SeverityBearing,
        cue_policy: M5CuePolicy::NonColorCueRequired,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        documented_overrides: &[],
        bindings: &[
            inherited(M5NormalizedState::Loading, M5AccessibilityCue::ShapeOrPattern),
            inherited(M5NormalizedState::Cached, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::Stale, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Partial, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::PolicyBlocked, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Degraded, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::PreviewOnly, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::SyncPending, M5AccessibilityCue::IconAndText),
            inherited(
                M5NormalizedState::BoundaryHandoff,
                M5AccessibilityCue::IconAndText,
            ),
        ],
    },
    // Offboarding surface. Severity-bearing; lifecycle-state tokens.
    SurfaceSeed {
        surface_id: "surface:offboarding.surface",
        surface_family: M5SurfaceFamily::OffboardingSurface,
        descriptor_revision_ref: "surface-rev:offboarding.surface:2026.06.01-01",
        primary_label_ref: "label:offboarding.surface:primary",
        registry_anchor_ref: "registry:anchor:offboarding:surface",
        accessibility_note: "Destructive and blocked states carry an icon and text, never color alone.",
        token_group: M5TokenGroup::LifecycleStateTokens,
        style_provenance: M5StyleProvenance::ShellTokenInherited,
        semantic_salience: M5SemanticSalience::SeverityBearing,
        cue_policy: M5CuePolicy::NonColorCueRequired,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        documented_overrides: &[],
        bindings: &[
            inherited(M5NormalizedState::Loading, M5AccessibilityCue::ShapeOrPattern),
            inherited(M5NormalizedState::Cached, M5AccessibilityCue::TextLabel),
            inherited(M5NormalizedState::Stale, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Partial, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::PolicyBlocked, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::Degraded, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::PreviewOnly, M5AccessibilityCue::IconAndText),
            inherited(M5NormalizedState::SyncPending, M5AccessibilityCue::IconAndText),
            inherited(
                M5NormalizedState::BoundaryHandoff,
                M5AccessibilityCue::IconAndText,
            ),
        ],
    },
];

fn build_binding_from_seed(
    descriptor: &M5ComponentStateDescriptor,
    seed: &BindingSeed,
) -> M5StateBinding {
    let projects_descriptor = seed.binding_status.projects_descriptor();
    M5StateBinding {
        state: seed.state,
        binding_status: seed.binding_status,
        projected_token_group: projects_descriptor.then_some(descriptor.token_group),
        projected_token_ref: projects_descriptor
            .then(|| canonical_token_ref(descriptor.token_group, seed.state)),
        projected_style_provenance: projects_descriptor.then_some(descriptor.style_provenance),
        projected_cue_policy: projects_descriptor.then_some(descriptor.cue_policy),
        projected_non_color_cue: projects_descriptor.then_some(seed.non_color_cue),
        projected_registry_anchor_ref: projects_descriptor
            .then(|| descriptor.registry_anchor_ref.clone()),
        applied_overrides: seed
            .applied_overrides
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        hardcoded_value: None,
        unresolved_token_fallback: None,
        narrowing_reason: seed.narrowing_reason.map(str::to_owned),
        note: seed.note.map(str::to_owned),
    }
}

fn build_row_from_seed(seed: &SurfaceSeed) -> M5ComponentStateRow {
    let descriptor = M5ComponentStateDescriptor {
        surface_id: seed.surface_id.to_owned(),
        surface_family: seed.surface_family,
        descriptor_revision_ref: seed.descriptor_revision_ref.to_owned(),
        primary_label_ref: seed.primary_label_ref.to_owned(),
        registry_anchor_ref: seed.registry_anchor_ref.to_owned(),
        accessibility_note: seed.accessibility_note.to_owned(),
        token_group: seed.token_group,
        style_provenance: seed.style_provenance,
        semantic_salience: seed.semantic_salience,
        cue_policy: seed.cue_policy,
        lifecycle_label: seed.lifecycle_label,
        documented_overrides: seed
            .documented_overrides
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        registered_in_shared_registry: true,
    };
    let bindings: Vec<M5StateBinding> = seed
        .bindings
        .iter()
        .map(|binding_seed| build_binding_from_seed(&descriptor, binding_seed))
        .collect();
    build_m5_component_state_row(descriptor, bindings)
}

/// Seeded audit builder used by the headless inspector and the
/// integration test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/ux/m5/theme-token-consumers/`.
pub fn seeded_m5_component_state_audit() -> M5ComponentStateAuditReport {
    let rows = SURFACE_SEEDS.iter().map(build_row_from_seed).collect();
    build_m5_component_state_audit(rows)
}
