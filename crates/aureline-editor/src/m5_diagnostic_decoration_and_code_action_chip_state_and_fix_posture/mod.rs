//! Implemented M5 diagnostic-decoration and code-action-chip primitives.
//!
//! The frozen [editor-inline component matrix][matrix] names the reusable editor / review / AI inline
//! UI components and locks their controlled vocabulary. This module is the second implement lane over
//! that matrix (after the [editor-tab / gutter lane][tabgutter]): it turns the two inline
//! *problem-and-action* components — the **diagnostic decoration** and the **code-action chip** — into
//! resolvers that produce export-safe, honest projections, so a user can read what a problem underline
//! means (severity, source/provider, freshness, and where it links) and what a quick action will do
//! (exact-versus-inferred fix posture, preview-required apply scope, blocked-action reasons, and
//! side-effect class) *before invoking it*, without any of that truth buried in a tooltip, encoded by
//! color alone, or optimistically overstated.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render diagnostic decorations with severity, source/provider, freshness or stale state, and
//!   stable linkage to Problems / output / support packets.** [`resolve_diagnostic_decoration`]
//!   refuses to read as a clean decoration when the problem identity is unstated, the severity is
//!   unresolved or encoded by color alone, the source/provider is unstated, the freshness is
//!   unresolved, a stale diagnostic is shown as current, the anchor durability is unresolved or has
//!   silently drifted, the linkage target is unresolved or its linkage to Problems / output / support
//!   is broken, an imported diagnostic overstates its certainty, or no command-backed detail path is
//!   reachable; it degrades instead.
//! * **Render code-action chips with exact-versus-inferred fix posture, preview-required apply scope,
//!   blocked-action reasons, and side-effect class where a fix touches multiple files or external
//!   state.** [`resolve_code_action_chip`] degrades when the chip identity is unstated, the fix posture
//!   is unresolved or encoded by color alone, an inferred fix is presented as exact, the apply scope is
//!   unresolved, a preview-required action bypasses its preview, a blocked action hides its reason, the
//!   side-effect class is unresolved or hidden for a multi-file / external-state fix, or no
//!   command-backed detail path is reachable.
//! * **Prevent notebook, editor, and imported-diagnostic surfaces from implying identical certainty
//!   when the underlying evidence class differs.** The packet proves, by resolved examples, that one
//!   severity / source / freshness vocabulary correlates underlines, markers, chips, and panel entries,
//!   that a user can tell whether a fix is exact, inferred, blocked, or review-required, and that no
//!   claimed inline action path bypasses the broader preview / apply truth.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5EditorInlineDisposition`] inline-disposition vocabulary, the [`M5DiagnosticSeverity`]
//! diagnostic-severity vocabulary, the [`M5FixPosture`] fix-posture vocabulary, and the
//! [`M5AnchorDurability`] anchor-durability vocabulary — so editor, notebook, AI, diagnostics, support,
//! and export surfaces can never fork their own severity, fix, or anchor wording. Raw secret values and
//! private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_editor_inline_component_matrix
//! [tabgutter]: crate::m5_editor_tab_and_gutter_state_and_marker_layering

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_diagnostic_chip_controls, seeded_m5_diagnostic_chip_controls_ai_ui_preview_narrowed,
    seeded_m5_diagnostic_chip_controls_diagnostics_ui_beta_narrowed,
    M5_DIAGNOSTIC_CHIP_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_editor_inline_component_matrix::{
    M5AnchorDurability, M5DiagnosticSeverity, M5EditorInlineAccessibilityRoute,
    M5EditorInlineComponentFamily, M5EditorInlineConsumerSurface, M5EditorInlineDeploymentLine,
    M5EditorInlineDisposition, M5EditorInlineDowngradeTrigger, M5EditorInlineQualificationClass,
    M5EditorInlineRequiredLabel, M5FixPosture, M5_CODE_ACTION_CHIP_SCHEMA_REF,
    M5_DIAGNOSTIC_DECORATION_SCHEMA_REF, M5_EDITOR_INLINE_COMPONENT_DOC_REF,
    M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5DiagnosticChipControlsPacket`].
pub const M5_DIAGNOSTIC_CHIP_CONTROLS_RECORD_KIND: &str =
    "implement_m5_diagnostic_decoration_and_code_action_chip_controls";

/// Schema version for M5 diagnostic-decoration / code-action-chip controls records.
pub const M5_DIAGNOSTIC_CHIP_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_DIAGNOSTIC_CHIP_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-diagnostic-decoration-code-action-chip-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_DIAGNOSTIC_CHIP_CONTROLS_DOC_REF: &str =
    "docs/editor/m5_diagnostic_decoration_and_code_action_chip_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DIAGNOSTIC_CHIP_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-diagnostic-decoration-code-action-chip-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_DIAGNOSTIC_CHIP_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-diagnostic-decoration-code-action-chip-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_DIAGNOSTIC_CHIP_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-diagnostic-decoration-code-action-chip-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_DIAGNOSTIC_CHIP_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-diagnostic-decoration-code-action-chip-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy
/// so no lane invents a parallel surface set.
pub type M5DiagnosticChipConsumerSurface = M5EditorInlineConsumerSurface;

/// Controlled source / provider class a diagnostic decoration names, so a diagnostic's evidence class
/// is legible and an imported / external diagnostic is never presented with the same certainty as a
/// native one. Minted by this lane because the frozen matrix carries diagnostic *severity* but not the
/// source / provider the diagnostic-decoration acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticSourceClass {
    /// A language server / analyzer.
    LanguageServer,
    /// The compiler / build.
    Compiler,
    /// A linter.
    Linter,
    /// A test runner.
    TestRunner,
    /// An imported / external diagnostic (its evidence class differs from a native run).
    ImportedExternal,
    /// The source / provider cannot currently be resolved.
    SourceUnknown,
}

impl M5DiagnosticSourceClass {
    /// Every source class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LanguageServer,
        Self::Compiler,
        Self::Linter,
        Self::TestRunner,
        Self::ImportedExternal,
        Self::SourceUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LanguageServer => "language_server",
            Self::Compiler => "compiler",
            Self::Linter => "linter",
            Self::TestRunner => "test_runner",
            Self::ImportedExternal => "imported_external",
            Self::SourceUnknown => "source_unknown",
        }
    }

    /// Whether this is an imported / external diagnostic whose certainty must be distinguished.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedExternal)
    }

    /// Whether the source class is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::SourceUnknown)
    }
}

/// Controlled freshness a diagnostic decoration names, so a stale diagnostic is never presented as a
/// current one. Minted by this lane as an explicit freshness axis alongside the reused severity vocab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticFreshness {
    /// The diagnostic is current for the buffer as shown.
    Current,
    /// The diagnostic is stale relative to the current buffer.
    Stale,
    /// The diagnostic is being recomputed.
    Recomputing,
    /// The diagnostic was superseded by a newer run.
    Superseded,
    /// The diagnostic was never computed on this build.
    NeverComputed,
    /// The freshness cannot currently be resolved.
    FreshnessUnknown,
}

impl M5DiagnosticFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Current,
        Self::Stale,
        Self::Recomputing,
        Self::Superseded,
        Self::NeverComputed,
        Self::FreshnessUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Recomputing => "recomputing",
            Self::Superseded => "superseded",
            Self::NeverComputed => "never_computed",
            Self::FreshnessUnknown => "freshness_unknown",
        }
    }

    /// Whether this freshness names a stale / superseded diagnostic that must never read as current.
    pub const fn is_stale(self) -> bool {
        matches!(self, Self::Stale | Self::Superseded)
    }

    /// Whether the freshness is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::FreshnessUnknown)
    }
}

/// Controlled linkage target a diagnostic decoration points at, so a decoration keeps a stable link to
/// Problems / output / support rather than floating as a bare underline. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticLinkageTarget {
    /// The Problems panel entry.
    ProblemsPanel,
    /// An output / build channel.
    OutputChannel,
    /// The support packet.
    SupportPacket,
    /// The diagnostics export.
    DiagnosticsExport,
    /// The inline editor decoration itself.
    EditorInline,
    /// The linkage target cannot currently be resolved.
    LinkageUnresolved,
}

impl M5DiagnosticLinkageTarget {
    /// Every linkage target, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProblemsPanel,
        Self::OutputChannel,
        Self::SupportPacket,
        Self::DiagnosticsExport,
        Self::EditorInline,
        Self::LinkageUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProblemsPanel => "problems_panel",
            Self::OutputChannel => "output_channel",
            Self::SupportPacket => "support_packet",
            Self::DiagnosticsExport => "diagnostics_export",
            Self::EditorInline => "editor_inline",
            Self::LinkageUnresolved => "linkage_unresolved",
        }
    }

    /// Whether the linkage target is resolved (not the unresolved sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::LinkageUnresolved)
    }
}

/// Controlled apply scope a code-action chip names, so a user knows whether invoking it previews,
/// reviews, applies directly, is blocked, or does not apply — never bypassing the preview / apply truth
/// established elsewhere in the sheet. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CodeActionApplyScope {
    /// The action shows a preview before it applies.
    PreviewRequired,
    /// The action requires review before it takes effect.
    ReviewRequired,
    /// The action applies directly.
    DirectApply,
    /// The action is blocked and cannot be invoked.
    Blocked,
    /// No action applies.
    NotApplicable,
    /// The apply scope cannot currently be resolved.
    ScopeUnresolved,
}

impl M5CodeActionApplyScope {
    /// Every apply scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PreviewRequired,
        Self::ReviewRequired,
        Self::DirectApply,
        Self::Blocked,
        Self::NotApplicable,
        Self::ScopeUnresolved,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewRequired => "preview_required",
            Self::ReviewRequired => "review_required",
            Self::DirectApply => "direct_apply",
            Self::Blocked => "blocked",
            Self::NotApplicable => "not_applicable",
            Self::ScopeUnresolved => "scope_unresolved",
        }
    }

    /// Whether this scope must route through a preview / review before it applies.
    pub const fn requires_preview(self) -> bool {
        matches!(self, Self::PreviewRequired | Self::ReviewRequired)
    }

    /// Whether this scope names a blocked action that must carry a reason.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::Blocked)
    }

    /// Whether the apply scope is resolved (not the unresolved sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ScopeUnresolved)
    }
}

/// Controlled side-effect class a code-action chip names, so a fix that touches multiple files or
/// external state never reads as a single-file edit. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CodeActionSideEffectClass {
    /// The fix touches a single file only.
    SingleFile,
    /// The fix touches multiple files.
    MultiFile,
    /// The fix touches the whole workspace.
    WorkspaceWide,
    /// The fix touches external state (network / filesystem / process).
    ExternalState,
    /// The fix is irreversible.
    Irreversible,
    /// The side-effect class cannot currently be resolved.
    SideEffectUnknown,
}

impl M5CodeActionSideEffectClass {
    /// Every side-effect class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleFile,
        Self::MultiFile,
        Self::WorkspaceWide,
        Self::ExternalState,
        Self::Irreversible,
        Self::SideEffectUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleFile => "single_file",
            Self::MultiFile => "multi_file",
            Self::WorkspaceWide => "workspace_wide",
            Self::ExternalState => "external_state",
            Self::Irreversible => "irreversible",
            Self::SideEffectUnknown => "side_effect_unknown",
        }
    }

    /// Whether this class touches multiple files or external state and so must be disclosed.
    pub const fn touches_multiple_or_external(self) -> bool {
        matches!(
            self,
            Self::MultiFile | Self::WorkspaceWide | Self::ExternalState | Self::Irreversible
        )
    }

    /// Whether the side-effect class is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::SideEffectUnknown)
    }
}

/// Controlled reason a code-action chip is blocked, so a blocked action never hides why. Minted by this
/// lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CodeActionBlockReason {
    /// The action is not blocked.
    NotBlocked,
    /// The action is denied by policy.
    PolicyDenied,
    /// A precondition for the action is unmet.
    PreconditionUnmet,
    /// The action conflicts with a concurrent change.
    ConflictingChange,
    /// The caller lacks the capability to invoke the action.
    InsufficientCapability,
    /// The block reason cannot currently be resolved.
    BlockReasonUnknown,
}

impl M5CodeActionBlockReason {
    /// Every block reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotBlocked,
        Self::PolicyDenied,
        Self::PreconditionUnmet,
        Self::ConflictingChange,
        Self::InsufficientCapability,
        Self::BlockReasonUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotBlocked => "not_blocked",
            Self::PolicyDenied => "policy_denied",
            Self::PreconditionUnmet => "precondition_unmet",
            Self::ConflictingChange => "conflicting_change",
            Self::InsufficientCapability => "insufficient_capability",
            Self::BlockReasonUnknown => "block_reason_unknown",
        }
    }

    /// Whether this names a concrete block reason (a blocked action must carry one).
    pub const fn is_stated(self) -> bool {
        !matches!(self, Self::NotBlocked | Self::BlockReasonUnknown)
    }
}

/// One mandatory rendered part a diagnostic decoration or code-action chip must be able to show, so no
/// severity, source, freshness, anchor, fix-posture, apply-scope, or side-effect fact is left implicit
/// behind compact chrome, a tooltip, or a secondary panel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticChipAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed inline disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The diagnostic severity (decoration).
    DiagnosticSeverity,
    /// The diagnostic source / provider (decoration).
    DiagnosticSource,
    /// The diagnostic freshness / stale state (decoration).
    DiagnosticFreshness,
    /// The anchor durability behind the decoration (decoration).
    AnchorDurability,
    /// The linkage target to Problems / output / support (decoration).
    LinkageTarget,
    /// The exact-versus-inferred fix posture (chip).
    FixPosture,
    /// The preview-required apply scope (chip).
    ApplyScope,
    /// The side-effect class where a fix touches multiple files or external state (chip).
    SideEffectClass,
    /// The blocked-action reason (chip).
    BlockReason,
    /// The command-backed path to trace the diagnostic or fix (both components).
    StateCommand,
}

impl M5DiagnosticChipAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::DiagnosticSeverity,
        Self::DiagnosticSource,
        Self::DiagnosticFreshness,
        Self::AnchorDurability,
        Self::LinkageTarget,
        Self::FixPosture,
        Self::ApplyScope,
        Self::SideEffectClass,
        Self::BlockReason,
        Self::StateCommand,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::DiagnosticSeverity => "diagnostic_severity",
            Self::DiagnosticSource => "diagnostic_source",
            Self::DiagnosticFreshness => "diagnostic_freshness",
            Self::AnchorDurability => "anchor_durability",
            Self::LinkageTarget => "linkage_target",
            Self::FixPosture => "fix_posture",
            Self::ApplyScope => "apply_scope",
            Self::SideEffectClass => "side_effect_class",
            Self::BlockReason => "block_reason",
            Self::StateCommand => "state_command",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route to trace a diagnostic
/// or understand a fix behind a degraded component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticChipNextAction {
    /// Open the command-backed diagnostic detail.
    OpenDiagnosticDetail,
    /// Inspect the exact-versus-inferred fix posture behind the chip.
    InspectFixPosture,
    /// Preview the fix before applying it.
    PreviewFixBeforeApply,
    /// Review a blocked action or its reason.
    ReviewBlockedAction,
    /// Review a stale or imported diagnostic whose certainty differs.
    ReviewStaleOrImported,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5DiagnosticChipNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenDiagnosticDetail,
        Self::InspectFixPosture,
        Self::PreviewFixBeforeApply,
        Self::ReviewBlockedAction,
        Self::ReviewStaleOrImported,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDiagnosticDetail => "open_diagnostic_detail",
            Self::InspectFixPosture => "inspect_fix_posture",
            Self::PreviewFixBeforeApply => "preview_fix_before_apply",
            Self::ReviewBlockedAction => "review_blocked_action",
            Self::ReviewStaleOrImported => "review_stale_or_imported",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticChipExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The inline dispositions carried.
    Dispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The diagnostic severity named by the decoration.
    DiagnosticSeverity,
    /// The diagnostic source / provider named by the decoration.
    DiagnosticSource,
    /// The diagnostic freshness named by the decoration.
    DiagnosticFreshness,
    /// The fix posture named by the chip.
    FixPosture,
    /// The apply scope named by the chip.
    ApplyScope,
    /// The side-effect class named by the chip.
    SideEffectClass,
    /// The accountable owner role.
    OwnerRole,
}

impl M5DiagnosticChipExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::DiagnosticSeverity,
        Self::DiagnosticSource,
        Self::DiagnosticFreshness,
        Self::FixPosture,
        Self::ApplyScope,
        Self::SideEffectClass,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::Dispositions => "dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::DiagnosticSeverity => "diagnostic_severity",
            Self::DiagnosticSource => "diagnostic_source",
            Self::DiagnosticFreshness => "diagnostic_freshness",
            Self::FixPosture => "fix_posture",
            Self::ApplyScope => "apply_scope",
            Self::SideEffectClass => "side_effect_class",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a diagnostic decoration degraded below a clean, legible state. The degrade-first ladder
/// returns one of these instead of ever letting an ambiguous decoration read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiagnosticDecorationDegradeReason {
    /// The problem identity / message is unstated; a user cannot tell what the underline means.
    DiagnosticIdentityUnstated,
    /// The severity cannot currently be resolved.
    SeverityUnresolved,
    /// The severity is encoded by color alone rather than named.
    SeverityEncodedByColorAlone,
    /// The source / provider of the diagnostic is unstated.
    SourceProviderUnstated,
    /// The freshness cannot currently be resolved.
    FreshnessUnresolved,
    /// A stale diagnostic is shown as current.
    StaleShownAsCurrent,
    /// The anchor durability cannot currently be resolved.
    AnchorDurabilityUnresolved,
    /// The anchor drifted, went outdated, or was orphaned without being disclosed.
    AnchorDriftHidden,
    /// The linkage target to Problems / output / support cannot currently be resolved.
    LinkageTargetUnresolved,
    /// The stable linkage to Problems / output / support is broken.
    ProblemsLinkageBroken,
    /// An imported / external diagnostic overstates its certainty relative to a native run.
    ImportedCertaintyOverstated,
    /// No command-backed path to trace the diagnostic is reachable.
    DiagnosticDetailPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5DiagnosticDecorationDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::DiagnosticIdentityUnstated,
        Self::SeverityUnresolved,
        Self::SeverityEncodedByColorAlone,
        Self::SourceProviderUnstated,
        Self::FreshnessUnresolved,
        Self::StaleShownAsCurrent,
        Self::AnchorDurabilityUnresolved,
        Self::AnchorDriftHidden,
        Self::LinkageTargetUnresolved,
        Self::ProblemsLinkageBroken,
        Self::ImportedCertaintyOverstated,
        Self::DiagnosticDetailPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosticIdentityUnstated => "diagnostic_identity_unstated",
            Self::SeverityUnresolved => "severity_unresolved",
            Self::SeverityEncodedByColorAlone => "severity_encoded_by_color_alone",
            Self::SourceProviderUnstated => "source_provider_unstated",
            Self::FreshnessUnresolved => "freshness_unresolved",
            Self::StaleShownAsCurrent => "stale_shown_as_current",
            Self::AnchorDurabilityUnresolved => "anchor_durability_unresolved",
            Self::AnchorDriftHidden => "anchor_drift_hidden",
            Self::LinkageTargetUnresolved => "linkage_target_unresolved",
            Self::ProblemsLinkageBroken => "problems_linkage_broken",
            Self::ImportedCertaintyOverstated => "imported_certainty_overstated",
            Self::DiagnosticDetailPathMissing => "diagnostic_detail_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5DiagnosticChipNextAction {
        match self {
            Self::DiagnosticIdentityUnstated
            | Self::SeverityUnresolved
            | Self::SeverityEncodedByColorAlone
            | Self::SourceProviderUnstated
            | Self::FreshnessUnresolved
            | Self::AnchorDurabilityUnresolved
            | Self::AnchorDriftHidden
            | Self::LinkageTargetUnresolved
            | Self::ProblemsLinkageBroken
            | Self::DiagnosticDetailPathMissing => M5DiagnosticChipNextAction::OpenDiagnosticDetail,
            Self::StaleShownAsCurrent | Self::ImportedCertaintyOverstated | Self::ProofStale => {
                M5DiagnosticChipNextAction::ReviewStaleOrImported
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EditorInlineDowngradeTrigger {
        match self {
            Self::SeverityEncodedByColorAlone => {
                M5EditorInlineDowngradeTrigger::TabMarkerDiagnosticColorOnly
            }
            Self::FreshnessUnresolved | Self::StaleShownAsCurrent => {
                M5EditorInlineDowngradeTrigger::DiagnosticFreshnessUnstated
            }
            Self::AnchorDurabilityUnresolved => M5EditorInlineDowngradeTrigger::AnchorStateUnstated,
            Self::AnchorDriftHidden => M5EditorInlineDowngradeTrigger::CommentAnchorDriftedSilently,
            Self::ProofStale => M5EditorInlineDowngradeTrigger::ProofStale,
            _ => M5EditorInlineDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// Reason a code-action chip degraded below a clean, invocable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CodeActionChipDegradeReason {
    /// The chip identity / action label is unstated.
    ChipIdentityUnstated,
    /// The fix posture cannot currently be resolved.
    FixPostureUnresolved,
    /// The fix posture is encoded by color alone rather than named.
    FixPostureEncodedByColorAlone,
    /// An inferred / heuristic fix is presented as an exact one.
    InferredFixShownAsExact,
    /// The apply scope cannot currently be resolved.
    ApplyScopeUnresolved,
    /// A preview-required action bypasses its preview / apply truth.
    PreviewRequiredButBypassed,
    /// A blocked action hides its reason.
    BlockedReasonHidden,
    /// The side-effect class cannot currently be resolved.
    SideEffectClassUnresolved,
    /// A multi-file / external-state fix hides its side-effect class.
    SideEffectClassHidden,
    /// No command-backed path to trace the fix is reachable.
    ChipDetailPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CodeActionChipDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ChipIdentityUnstated,
        Self::FixPostureUnresolved,
        Self::FixPostureEncodedByColorAlone,
        Self::InferredFixShownAsExact,
        Self::ApplyScopeUnresolved,
        Self::PreviewRequiredButBypassed,
        Self::BlockedReasonHidden,
        Self::SideEffectClassUnresolved,
        Self::SideEffectClassHidden,
        Self::ChipDetailPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ChipIdentityUnstated => "chip_identity_unstated",
            Self::FixPostureUnresolved => "fix_posture_unresolved",
            Self::FixPostureEncodedByColorAlone => "fix_posture_encoded_by_color_alone",
            Self::InferredFixShownAsExact => "inferred_fix_shown_as_exact",
            Self::ApplyScopeUnresolved => "apply_scope_unresolved",
            Self::PreviewRequiredButBypassed => "preview_required_but_bypassed",
            Self::BlockedReasonHidden => "blocked_reason_hidden",
            Self::SideEffectClassUnresolved => "side_effect_class_unresolved",
            Self::SideEffectClassHidden => "side_effect_class_hidden",
            Self::ChipDetailPathMissing => "chip_detail_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5DiagnosticChipNextAction {
        match self {
            Self::ChipIdentityUnstated
            | Self::FixPostureUnresolved
            | Self::FixPostureEncodedByColorAlone
            | Self::InferredFixShownAsExact
            | Self::ChipDetailPathMissing => M5DiagnosticChipNextAction::InspectFixPosture,
            Self::ApplyScopeUnresolved
            | Self::PreviewRequiredButBypassed
            | Self::SideEffectClassUnresolved
            | Self::SideEffectClassHidden => M5DiagnosticChipNextAction::PreviewFixBeforeApply,
            Self::BlockedReasonHidden => M5DiagnosticChipNextAction::ReviewBlockedAction,
            Self::ProofStale => M5DiagnosticChipNextAction::ReviewStaleOrImported,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5EditorInlineDowngradeTrigger {
        match self {
            Self::FixPostureEncodedByColorAlone => {
                M5EditorInlineDowngradeTrigger::TabMarkerDiagnosticColorOnly
            }
            Self::InferredFixShownAsExact => {
                M5EditorInlineDowngradeTrigger::InferredFixShownAsExact
            }
            Self::ProofStale => M5EditorInlineDowngradeTrigger::ProofStale,
            _ => M5EditorInlineDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// True when a diagnostic severity cannot be resolved.
fn severity_is_unresolved(severity: M5DiagnosticSeverity) -> bool {
    matches!(severity, M5DiagnosticSeverity::SeverityUnknown)
}

/// True when a severity itself names a stale diagnostic.
fn severity_is_stale(severity: M5DiagnosticSeverity) -> bool {
    matches!(severity, M5DiagnosticSeverity::StaleDiagnostic)
}

/// True when an anchor still points at a durable range (exact or cleanly re-anchored).
fn anchor_is_durable(anchor: M5AnchorDurability) -> bool {
    matches!(
        anchor,
        M5AnchorDurability::AnchoredExact | M5AnchorDurability::ReAnchored
    )
}

/// True when an anchor has drifted, gone outdated, or been orphaned.
fn anchor_is_drifted(anchor: M5AnchorDurability) -> bool {
    matches!(
        anchor,
        M5AnchorDurability::DriftedApproximate
            | M5AnchorDurability::OutdatedAnchor
            | M5AnchorDurability::OrphanedAnchor
    )
}

/// True when a fix posture is inferred / heuristic and must never read as exact.
fn fix_is_inferred(posture: M5FixPosture) -> bool {
    matches!(
        posture,
        M5FixPosture::InferredFix | M5FixPosture::HeuristicSuggestion
    )
}

/// Input to [`resolve_diagnostic_decoration`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DiagnosticDecorationResolutionInput {
    /// Stable identity of the decoration instance.
    pub decoration_id: String,
    /// The problem message / identity shown; empty means unstated.
    pub message_label: String,
    /// The diagnostic severity.
    pub severity: M5DiagnosticSeverity,
    /// True when the severity is stated non-color-only (name / icon-with-label, never color alone).
    pub severity_stated: bool,
    /// The source / provider class.
    pub source_class: M5DiagnosticSourceClass,
    /// The freshness / stale state.
    pub freshness: M5DiagnosticFreshness,
    /// True when a stale / superseded diagnostic is disclosed as such, never shown as current.
    pub stale_disclosed: bool,
    /// The anchor durability behind the decoration.
    pub anchor_durability: M5AnchorDurability,
    /// True when a drifted / outdated / orphaned anchor is disclosed, never silently drifted.
    pub anchor_drift_disclosed: bool,
    /// The linkage target to Problems / output / support.
    pub linkage_target: M5DiagnosticLinkageTarget,
    /// True when the linkage to Problems / output / support is stable.
    pub linkage_stable: bool,
    /// True when an imported / external diagnostic's certainty is distinguished from a native run.
    pub imported_certainty_distinguished: bool,
    /// True when a command-backed entrypoint to trace the diagnostic is reachable, never menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe diagnostic-decoration projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDiagnosticDecoration {
    /// Stable identity of the decoration instance.
    pub decoration_id: String,
    /// The problem message / identity named by the decoration.
    pub message_label: String,
    /// The severity token named by the decoration.
    pub severity: String,
    /// Whether the severity is stated non-color-only.
    pub severity_stated: bool,
    /// The source / provider token named by the decoration.
    pub source_class: String,
    /// Whether the diagnostic is imported / external.
    pub source_is_imported: bool,
    /// The freshness token named by the decoration.
    pub freshness: String,
    /// Whether the diagnostic is stale (by freshness or by severity).
    pub freshness_is_stale: bool,
    /// Whether a stale diagnostic is disclosed as such.
    pub stale_disclosed: bool,
    /// The anchor-durability token named by the decoration.
    pub anchor_durability: String,
    /// Whether the anchor still points at a durable range.
    pub anchor_is_durable: bool,
    /// Whether the anchor has drifted / gone outdated / been orphaned.
    pub anchor_is_drifted: bool,
    /// Whether a drifted anchor is disclosed.
    pub anchor_drift_disclosed: bool,
    /// The linkage-target token named by the decoration.
    pub linkage_target: String,
    /// Whether the linkage to Problems / output / support is stable.
    pub linkage_stable: bool,
    /// Whether an imported diagnostic's certainty is distinguished from a native run.
    pub imported_certainty_distinguished: bool,
    /// Whether a command-backed entrypoint to trace the diagnostic is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the decoration could not read as a clean, legible state.
    pub degrade_reason: Option<M5DiagnosticDecorationDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5DiagnosticChipNextAction,
    /// Whether the diagnostic is legible at a glance (clean decoration naming every fact).
    pub decoration_legible_at_a_glance: bool,
}

impl M5ResolvedDiagnosticDecoration {
    /// Whether this decoration reads as a clean, legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_code_action_chip`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CodeActionChipResolutionInput {
    /// Stable identity of the chip instance.
    pub chip_id: String,
    /// The action label shown; empty means unstated.
    pub action_label: String,
    /// The exact-versus-inferred fix posture.
    pub fix_posture: M5FixPosture,
    /// True when the fix posture is stated non-color-only.
    pub posture_stated: bool,
    /// True when the chip claims / reads as an exact fix.
    pub shown_as_exact: bool,
    /// The preview-required apply scope.
    pub apply_scope: M5CodeActionApplyScope,
    /// True when a preview / review path is available for a preview-required action.
    pub preview_available: bool,
    /// The side-effect class where a fix touches multiple files or external state.
    pub side_effect_class: M5CodeActionSideEffectClass,
    /// True when a multi-file / external-state side effect is disclosed.
    pub side_effect_disclosed: bool,
    /// The blocked-action reason (must be a concrete reason when the action is blocked).
    pub block_reason: M5CodeActionBlockReason,
    /// True when a command-backed entrypoint to trace the fix is reachable, never menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe code-action-chip projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCodeActionChip {
    /// Stable identity of the chip instance.
    pub chip_id: String,
    /// The action label named by the chip.
    pub action_label: String,
    /// The fix-posture token named by the chip.
    pub fix_posture: String,
    /// Whether the fix posture is an exact, verified fix.
    pub fix_is_exact: bool,
    /// Whether the fix posture is inferred / heuristic.
    pub fix_is_inferred: bool,
    /// Whether the chip claims / reads as an exact fix.
    pub claims_exact: bool,
    /// Whether the fix posture is stated non-color-only.
    pub posture_stated: bool,
    /// The apply-scope token named by the chip.
    pub apply_scope: String,
    /// Whether this action routes through a preview / review before it applies.
    pub requires_preview: bool,
    /// Whether a preview / review path is available.
    pub preview_available: bool,
    /// Whether this action is blocked.
    pub is_blocked: bool,
    /// The block-reason token named by the chip.
    pub block_reason: String,
    /// Whether a concrete block reason is stated.
    pub block_reason_stated: bool,
    /// The side-effect-class token named by the chip.
    pub side_effect_class: String,
    /// Whether this fix touches multiple files or external state.
    pub touches_multiple_or_external: bool,
    /// Whether a multi-file / external-state side effect is disclosed.
    pub side_effect_disclosed: bool,
    /// Whether a command-backed entrypoint to trace the fix is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the chip could not read as a clean, invocable state.
    pub degrade_reason: Option<M5CodeActionChipDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5DiagnosticChipNextAction,
    /// Whether the fix posture is legible at a glance (clean chip naming every fact).
    pub fix_posture_legible_at_a_glance: bool,
}

impl M5ResolvedCodeActionChip {
    /// Whether this chip reads as a clean, invocable state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5DiagnosticChipResolutionError {
    /// The decoration id was empty.
    EmptyDecorationId,
    /// The chip id was empty.
    EmptyChipId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5DiagnosticChipResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyDecorationId => "empty_decoration_id",
            Self::EmptyChipId => "empty_chip_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5DiagnosticChipResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 diagnostic-decoration / code-action-chip resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DiagnosticChipResolutionError {}

/// Resolves a diagnostic decoration so a problem underline is legible at a glance: the decoration names
/// its severity (non-color-only), source / provider, freshness (never showing a stale diagnostic as
/// current), anchor durability (never silently drifting), and stable linkage to Problems / output /
/// support, never overstates the certainty of an imported diagnostic, and always offers a
/// command-backed detail entrypoint.
pub fn resolve_diagnostic_decoration(
    input: M5DiagnosticDecorationResolutionInput,
) -> Result<M5ResolvedDiagnosticDecoration, M5DiagnosticChipResolutionError> {
    if input.decoration_id.trim().is_empty() {
        return Err(M5DiagnosticChipResolutionError::EmptyDecorationId);
    }
    if string_is_forbidden(&input.decoration_id) || string_is_forbidden(&input.message_label) {
        return Err(M5DiagnosticChipResolutionError::ForbiddenMaterial);
    }

    let source_is_imported = input.source_class.is_imported();
    let freshness_is_stale = input.freshness.is_stale() || severity_is_stale(input.severity);
    let durable = anchor_is_durable(input.anchor_durability);
    let drifted = anchor_is_drifted(input.anchor_durability);

    let degrade_reason = if input.message_label.trim().is_empty() {
        Some(M5DiagnosticDecorationDegradeReason::DiagnosticIdentityUnstated)
    } else if severity_is_unresolved(input.severity) {
        Some(M5DiagnosticDecorationDegradeReason::SeverityUnresolved)
    } else if !input.severity_stated {
        Some(M5DiagnosticDecorationDegradeReason::SeverityEncodedByColorAlone)
    } else if !input.source_class.is_resolved() {
        Some(M5DiagnosticDecorationDegradeReason::SourceProviderUnstated)
    } else if !input.freshness.is_resolved() {
        Some(M5DiagnosticDecorationDegradeReason::FreshnessUnresolved)
    } else if freshness_is_stale && !input.stale_disclosed {
        Some(M5DiagnosticDecorationDegradeReason::StaleShownAsCurrent)
    } else if matches!(
        input.anchor_durability,
        M5AnchorDurability::AnchorUnresolved
    ) {
        Some(M5DiagnosticDecorationDegradeReason::AnchorDurabilityUnresolved)
    } else if drifted && !input.anchor_drift_disclosed {
        Some(M5DiagnosticDecorationDegradeReason::AnchorDriftHidden)
    } else if !input.linkage_target.is_resolved() {
        Some(M5DiagnosticDecorationDegradeReason::LinkageTargetUnresolved)
    } else if !input.linkage_stable {
        Some(M5DiagnosticDecorationDegradeReason::ProblemsLinkageBroken)
    } else if source_is_imported && !input.imported_certainty_distinguished {
        Some(M5DiagnosticDecorationDegradeReason::ImportedCertaintyOverstated)
    } else if !input.detail_command_available {
        Some(M5DiagnosticDecorationDegradeReason::DiagnosticDetailPathMissing)
    } else if !input.proof_fresh {
        Some(M5DiagnosticDecorationDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5DiagnosticChipNextAction::OpenDiagnosticDetail,
    };

    Ok(M5ResolvedDiagnosticDecoration {
        decoration_id: input.decoration_id,
        message_label: input.message_label,
        severity: input.severity.as_str().to_owned(),
        severity_stated: input.severity_stated,
        source_class: input.source_class.as_str().to_owned(),
        source_is_imported,
        freshness: input.freshness.as_str().to_owned(),
        freshness_is_stale,
        stale_disclosed: input.stale_disclosed,
        anchor_durability: input.anchor_durability.as_str().to_owned(),
        anchor_is_durable: durable,
        anchor_is_drifted: drifted,
        anchor_drift_disclosed: input.anchor_drift_disclosed,
        linkage_target: input.linkage_target.as_str().to_owned(),
        linkage_stable: input.linkage_stable,
        imported_certainty_distinguished: input.imported_certainty_distinguished,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        decoration_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves a code-action chip so a user can tell — before invoking it — whether a fix is exact,
/// inferred, blocked, or review-required: the chip names its fix posture (never showing an inferred fix
/// as exact), its preview-required apply scope (never bypassing the preview / apply truth), its
/// side-effect class where a fix touches multiple files or external state, and its blocked-action
/// reason, and always offers a command-backed detail entrypoint.
pub fn resolve_code_action_chip(
    input: M5CodeActionChipResolutionInput,
) -> Result<M5ResolvedCodeActionChip, M5DiagnosticChipResolutionError> {
    if input.chip_id.trim().is_empty() {
        return Err(M5DiagnosticChipResolutionError::EmptyChipId);
    }
    if string_is_forbidden(&input.chip_id) || string_is_forbidden(&input.action_label) {
        return Err(M5DiagnosticChipResolutionError::ForbiddenMaterial);
    }

    let inferred = fix_is_inferred(input.fix_posture);
    let requires_preview = input.apply_scope.requires_preview();
    let is_blocked = input.apply_scope.is_blocked();
    let touches = input.side_effect_class.touches_multiple_or_external();
    let block_reason_stated = input.block_reason.is_stated();

    let degrade_reason = if input.action_label.trim().is_empty() {
        Some(M5CodeActionChipDegradeReason::ChipIdentityUnstated)
    } else if matches!(input.fix_posture, M5FixPosture::PostureUnknown) {
        Some(M5CodeActionChipDegradeReason::FixPostureUnresolved)
    } else if !input.posture_stated {
        Some(M5CodeActionChipDegradeReason::FixPostureEncodedByColorAlone)
    } else if inferred && input.shown_as_exact {
        Some(M5CodeActionChipDegradeReason::InferredFixShownAsExact)
    } else if !input.apply_scope.is_resolved() {
        Some(M5CodeActionChipDegradeReason::ApplyScopeUnresolved)
    } else if requires_preview && !input.preview_available {
        Some(M5CodeActionChipDegradeReason::PreviewRequiredButBypassed)
    } else if is_blocked && !block_reason_stated {
        Some(M5CodeActionChipDegradeReason::BlockedReasonHidden)
    } else if !input.side_effect_class.is_resolved() {
        Some(M5CodeActionChipDegradeReason::SideEffectClassUnresolved)
    } else if touches && !input.side_effect_disclosed {
        Some(M5CodeActionChipDegradeReason::SideEffectClassHidden)
    } else if !input.detail_command_available {
        Some(M5CodeActionChipDegradeReason::ChipDetailPathMissing)
    } else if !input.proof_fresh {
        Some(M5CodeActionChipDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5DiagnosticChipNextAction::PreviewFixBeforeApply,
    };

    Ok(M5ResolvedCodeActionChip {
        chip_id: input.chip_id,
        action_label: input.action_label,
        fix_posture: input.fix_posture.as_str().to_owned(),
        fix_is_exact: matches!(input.fix_posture, M5FixPosture::ExactFix),
        fix_is_inferred: inferred,
        claims_exact: input.shown_as_exact,
        posture_stated: input.posture_stated,
        apply_scope: input.apply_scope.as_str().to_owned(),
        requires_preview,
        preview_available: input.preview_available,
        is_blocked,
        block_reason: input.block_reason.as_str().to_owned(),
        block_reason_stated,
        side_effect_class: input.side_effect_class.as_str().to_owned(),
        touches_multiple_or_external: touches,
        side_effect_disclosed: input.side_effect_disclosed,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        fix_posture_legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved diagnostic-decoration and
/// code-action-chip examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiagnosticChipControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5DiagnosticChipConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5EditorInlineQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5EditorInlineDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5EditorInlineRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5EditorInlineAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5DiagnosticChipAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5DiagnosticChipExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5EditorInlineDowngradeTrigger>,
    /// Resolved diagnostic-decoration examples.
    pub diagnostic_examples: Vec<M5ResolvedDiagnosticDecoration>,
    /// Resolved code-action-chip examples.
    pub chip_examples: Vec<M5ResolvedCodeActionChip>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a diagnostic never encodes severity / source by color alone.
    pub diagnostic_severity_or_source_encoded_by_color_alone: bool,
    /// Hard invariant: a diagnostic anchor or freshness never silently drifts.
    pub diagnostic_anchor_or_freshness_silently_drifts: bool,
    /// Hard invariant: an inferred or blocked fix is never presented as exact or ready.
    pub inferred_or_blocked_fix_presented_as_exact_or_ready: bool,
    /// Hard invariant: a code action never bypasses the preview / apply truth.
    pub code_action_bypasses_preview_or_apply_truth: bool,
}

impl M5DiagnosticChipControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DiagnosticChipAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DiagnosticChipAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DiagnosticChipExportField> =
            self.export_fields.iter().copied().collect();
        M5DiagnosticChipExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.diagnostic_severity_or_source_encoded_by_color_alone
            && !self.diagnostic_anchor_or_freshness_silently_drifts
            && !self.inferred_or_blocked_fix_presented_as_exact_or_ready
            && !self.code_action_bypasses_preview_or_apply_truth
    }

    /// True when every resolved example on this row is honest: no clean decoration encodes severity by
    /// color alone, shows a stale diagnostic as current, drifts an anchor silently, overstates an
    /// imported diagnostic's certainty, breaks its linkage, or lacks a trace path; and no clean chip
    /// shows an inferred fix as exact, bypasses a required preview, hides a multi-file / external side
    /// effect, hides a blocked-action reason, or lacks a trace path.
    fn examples_are_honest(&self) -> bool {
        self.diagnostic_examples
            .iter()
            .all(|ex| !ex.is_clean() || decoration_is_honest(ex))
            && self
                .chip_examples
                .iter()
                .all(|ex| !ex.is_clean() || chip_is_honest(ex))
    }
}

/// True when a clean diagnostic decoration keeps every guardrail: severity stated, no stale-as-current,
/// no silent anchor drift, no overstated imported certainty, stable linkage, and a reachable trace.
fn decoration_is_honest(ex: &M5ResolvedDiagnosticDecoration) -> bool {
    ex.severity_stated
        && (ex.stale_disclosed || !ex.freshness_is_stale)
        && (ex.anchor_drift_disclosed || !ex.anchor_is_drifted)
        && (ex.imported_certainty_distinguished || !ex.source_is_imported)
        && ex.linkage_stable
        && ex.detail_command_available
}

/// True when a clean code-action chip keeps every guardrail: posture stated, no inferred-as-exact, no
/// bypassed preview, no hidden multi-file / external side effect, no hidden block reason, and a
/// reachable trace.
fn chip_is_honest(ex: &M5ResolvedCodeActionChip) -> bool {
    ex.posture_stated
        && !(ex.fix_is_inferred && ex.claims_exact)
        && (ex.preview_available || !ex.requires_preview)
        && (ex.side_effect_disclosed || !ex.touches_multiple_or_external)
        && (ex.block_reason_stated || !ex.is_blocked)
        && ex.detail_command_available
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiagnosticChipVocabularySet {
    /// Inline-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Diagnostic-severity tokens (bound from the frozen matrix).
    pub diagnostic_severities: Vec<String>,
    /// Fix-posture tokens (bound from the frozen matrix).
    pub fix_postures: Vec<String>,
    /// Anchor-durability tokens (bound from the frozen matrix).
    pub anchor_durabilities: Vec<String>,
    /// Diagnostic source-class tokens (minted by this lane).
    pub diagnostic_source_classes: Vec<String>,
    /// Diagnostic freshness tokens (minted by this lane).
    pub diagnostic_freshnesses: Vec<String>,
    /// Diagnostic linkage-target tokens (minted by this lane).
    pub diagnostic_linkage_targets: Vec<String>,
    /// Apply-scope tokens (minted by this lane).
    pub apply_scopes: Vec<String>,
    /// Side-effect-class tokens (minted by this lane).
    pub side_effect_classes: Vec<String>,
    /// Block-reason tokens (minted by this lane).
    pub block_reasons: Vec<String>,
    /// Diagnostic-decoration degrade-reason tokens.
    pub diagnostic_decoration_degrade_reasons: Vec<String>,
    /// Code-action-chip degrade-reason tokens.
    pub code_action_chip_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5DiagnosticChipVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5EditorInlineDisposition::ALL, |v| v.as_str()),
            diagnostic_severities: tokens(&M5DiagnosticSeverity::ALL, |v| v.as_str()),
            fix_postures: tokens(&M5FixPosture::ALL, |v| v.as_str()),
            anchor_durabilities: tokens(&M5AnchorDurability::ALL, |v| v.as_str()),
            diagnostic_source_classes: tokens(&M5DiagnosticSourceClass::ALL, |v| v.as_str()),
            diagnostic_freshnesses: tokens(&M5DiagnosticFreshness::ALL, |v| v.as_str()),
            diagnostic_linkage_targets: tokens(&M5DiagnosticLinkageTarget::ALL, |v| v.as_str()),
            apply_scopes: tokens(&M5CodeActionApplyScope::ALL, |v| v.as_str()),
            side_effect_classes: tokens(&M5CodeActionSideEffectClass::ALL, |v| v.as_str()),
            block_reasons: tokens(&M5CodeActionBlockReason::ALL, |v| v.as_str()),
            diagnostic_decoration_degrade_reasons: tokens(
                &M5DiagnosticDecorationDegradeReason::ALL,
                |v| v.as_str(),
            ),
            code_action_chip_degrade_reasons: tokens(&M5CodeActionChipDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5DiagnosticChipAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5DiagnosticChipNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DiagnosticChipExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5EditorInlineConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5DiagnosticChipGovernanceReview {
    /// The decoration names its severity, source, and freshness with one shared vocabulary.
    pub decoration_names_severity_source_and_freshness: bool,
    /// The decoration states severity with no-color-only semantics.
    pub decoration_severity_no_color_only: bool,
    /// The decoration keeps a stable linkage to Problems / output / support.
    pub decoration_linkage_stable_to_problems_output_support: bool,
    /// Stale diagnostics are never shown as current.
    pub stale_diagnostics_never_shown_as_current: bool,
    /// Anchors never silently drift.
    pub anchors_never_silently_drift: bool,
    /// Imported diagnostics never overstate certainty relative to native runs.
    pub imported_diagnostics_never_overstate_certainty: bool,
    /// The chip names its exact-versus-inferred fix posture.
    pub chip_names_exact_versus_inferred_posture: bool,
    /// Inferred fixes are never presented as exact.
    pub inferred_fixes_never_presented_as_exact: bool,
    /// The chip names its preview-required apply scope and never bypasses preview / apply truth.
    pub chip_apply_scope_never_bypasses_preview: bool,
    /// Blocked actions always carry a reason.
    pub blocked_actions_always_carry_reason: bool,
    /// Multi-file / external-state side effects are always disclosed.
    pub multi_file_or_external_side_effects_disclosed: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiagnosticChipConsumerProjection {
    /// Editor surfaces consume the shared diagnostic and chip vocabulary.
    pub editor_surfaces_consume_diagnostic_and_chip_vocabulary: bool,
    /// The notebook consumes the shared diagnostic and chip vocabulary for code cells.
    pub notebook_consumes_diagnostic_and_chip_vocabulary: bool,
    /// AI surfaces consume the shared fix-posture and apply-scope vocabulary.
    pub ai_surfaces_consume_fix_posture_and_apply_scope_vocabulary: bool,
    /// Diagnostics consume the shared severity / source / freshness vocabulary.
    pub diagnostics_consume_severity_source_and_freshness_vocabulary: bool,
    /// Problem and fix facts trace back to one canonical component contract.
    pub facts_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical editor-inline source.
    pub support_export_reads_single_editor_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiagnosticChipProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiagnosticChipReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DiagnosticChipControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DiagnosticChipControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5DiagnosticChipControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DiagnosticChipVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DiagnosticChipGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DiagnosticChipConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DiagnosticChipProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DiagnosticChipReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 diagnostic-decoration / code-action-chip controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiagnosticChipControlsPacket {
    /// Record kind; must equal [`M5_DIAGNOSTIC_CHIP_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DIAGNOSTIC_CHIP_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5DiagnosticChipControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DiagnosticChipVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DiagnosticChipGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DiagnosticChipConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DiagnosticChipProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DiagnosticChipReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DiagnosticChipControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5DiagnosticChipControlsPacketInput) -> Self {
        Self {
            record_kind: M5_DIAGNOSTIC_CHIP_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_DIAGNOSTIC_CHIP_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            controls_label: input.controls_label,
            controls_rows: input.controls_rows,
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

    /// Validates the controls-packet invariants.
    pub fn validate(&self) -> Vec<M5DiagnosticChipControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DIAGNOSTIC_CHIP_CONTROLS_RECORD_KIND {
            violations.push(M5DiagnosticChipControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DIAGNOSTIC_CHIP_CONTROLS_SCHEMA_VERSION {
            violations.push(M5DiagnosticChipControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DiagnosticChipControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5DiagnosticChipControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 diagnostic-decoration / code-action-chip controls packet serializes"),
        ) {
            violations.push(M5DiagnosticChipControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 diagnostic-decoration / code-action-chip controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,diagnostic_examples,chip_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .diagnostic_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.chip_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.diagnostic_examples.len(),
                row.chip_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Diagnostic-Decoration and Code-Action-Chip Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Diagnostic source classes: {}\n",
            self.vocabulary_set.diagnostic_source_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Code-action apply scopes: {}\n",
            self.vocabulary_set.apply_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Diagnostic examples: {} / chip examples: {}\n",
                row.diagnostic_examples.len(),
                row.chip_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5DiagnosticChipControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DiagnosticChipControlsViolation>),
}

impl fmt::Display for M5DiagnosticChipControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 diagnostic-decoration / code-action-chip controls export parse failed: {error}"
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
                    "m5 diagnostic-decoration / code-action-chip controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DiagnosticChipControlsArtifactError {}

/// Validation failures emitted by [`M5DiagnosticChipControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DiagnosticChipControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The controls packet declares no rows.
    NoControlsRows,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row does not point at both component schemas.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (color-only, drift, overstated certainty,
    /// inferred-as-exact, preview bypass, hidden side effect, hidden block reason, or missing trace).
    DishonestExample,
    /// A controls row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Severity / source / freshness vocabulary is not proven: clean decorations do not cover the
    /// shared severity and source grammar across surfaces, or no color-only / stale example degrades.
    SeveritySourceFreshnessVocabularyNotProven,
    /// Fix-posture legibility is not proven: clean chips do not cover distinct postures and scopes, or
    /// no inferred-as-exact example degrades.
    FixPostureLegibilityNotProven,
    /// Preview / apply truth is not proven: no clean chip requires and offers a preview, no preview
    /// bypass degrades, or a clean decoration and chip do not both offer a command-backed detail path.
    PreviewApplyTruthNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DiagnosticChipControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoControlsRows => "no_controls_rows",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::SeveritySourceFreshnessVocabularyNotProven => {
                "severity_source_freshness_vocabulary_not_proven"
            }
            Self::FixPostureLegibilityNotProven => "fix_posture_legibility_not_proven",
            Self::PreviewApplyTruthNotProven => "preview_apply_truth_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_diagnostic_chip_controls_export(
) -> Result<M5DiagnosticChipControlsPacket, M5DiagnosticChipControlsArtifactError> {
    let packet: M5DiagnosticChipControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-diagnostic-decoration-code-action-chip-controls-proof/support_export.json"
    )))
    .map_err(M5DiagnosticChipControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DiagnosticChipControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5DiagnosticChipControlsPacket,
    violations: &mut Vec<M5DiagnosticChipControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DIAGNOSTIC_CHIP_CONTROLS_SCHEMA_REF,
        M5_DIAGNOSTIC_CHIP_CONTROLS_DOC_REF,
        M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
        M5_EDITOR_INLINE_COMPONENT_DOC_REF,
        M5_DIAGNOSTIC_DECORATION_SCHEMA_REF,
        M5_CODE_ACTION_CHIP_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DiagnosticChipControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5DiagnosticChipControlsPacket,
    violations: &mut Vec<M5DiagnosticChipControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5DiagnosticChipControlsViolation::NoControlsRows);
        return;
    }
    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5DiagnosticChipControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DiagnosticChipControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DiagnosticChipControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_DIAGNOSTIC_DECORATION_SCHEMA_REF)
            || !refs.contains(M5_CODE_ACTION_CHIP_SCHEMA_REF)
        {
            violations.push(M5DiagnosticChipControlsViolation::ComponentSchemaRefMissing);
        }
        if row.diagnostic_examples.is_empty() || row.chip_examples.is_empty() {
            violations.push(M5DiagnosticChipControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5DiagnosticChipControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5DiagnosticChipControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5DiagnosticChipControlsPacket,
    violations: &mut Vec<M5DiagnosticChipControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.decoration_names_severity_source_and_freshness,
        review.decoration_severity_no_color_only,
        review.decoration_linkage_stable_to_problems_output_support,
        review.stale_diagnostics_never_shown_as_current,
        review.anchors_never_silently_drift,
        review.imported_diagnostics_never_overstate_certainty,
        review.chip_names_exact_versus_inferred_posture,
        review.inferred_fixes_never_presented_as_exact,
        review.chip_apply_scope_never_bypasses_preview,
        review.blocked_actions_always_carry_reason,
        review.multi_file_or_external_side_effects_disclosed,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5DiagnosticChipControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DiagnosticChipControlsPacket,
    violations: &mut Vec<M5DiagnosticChipControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.editor_surfaces_consume_diagnostic_and_chip_vocabulary,
        projection.notebook_consumes_diagnostic_and_chip_vocabulary,
        projection.ai_surfaces_consume_fix_posture_and_apply_scope_vocabulary,
        projection.diagnostics_consume_severity_source_and_freshness_vocabulary,
        projection.facts_trace_to_single_component_contract,
        projection.support_export_reads_single_editor_source,
    ] {
        if !ok {
            violations.push(M5DiagnosticChipControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DiagnosticChipControlsPacket,
    violations: &mut Vec<M5DiagnosticChipControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DiagnosticChipControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DiagnosticChipControlsPacket,
    violations: &mut Vec<M5DiagnosticChipControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DiagnosticChipControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5DiagnosticChipControlsPacket,
    violations: &mut Vec<M5DiagnosticChipControlsViolation>,
) {
    let decorations = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.diagnostic_examples.iter())
    };
    let chips = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.chip_examples.iter())
    };

    // AC1: problem underlines, markers, chips, and panel entries correlate through one
    // severity/source/freshness vocabulary. Clean decorations cover at least two distinct severities
    // and two distinct sources, a color-only-severity example degrades, a stale-shown-as-current
    // example degrades, and no clean decoration is color-only.
    let clean_severities: BTreeSet<String> = decorations()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.severity.clone())
        .collect();
    let clean_sources: BTreeSet<String> = decorations()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.source_class.clone())
        .collect();
    let color_only_severity_degrades = decorations().any(|ex| {
        ex.degrade_reason == Some(M5DiagnosticDecorationDegradeReason::SeverityEncodedByColorAlone)
    });
    let stale_shown_degrades = decorations().any(|ex| {
        ex.degrade_reason == Some(M5DiagnosticDecorationDegradeReason::StaleShownAsCurrent)
    });
    let no_clean_color_only = decorations().all(|ex| !ex.is_clean() || ex.severity_stated);
    if !(clean_severities.len() >= 2
        && clean_sources.len() >= 2
        && color_only_severity_degrades
        && stale_shown_degrades
        && no_clean_color_only)
    {
        violations
            .push(M5DiagnosticChipControlsViolation::SeveritySourceFreshnessVocabularyNotProven);
    }

    // AC2: users can tell whether a fix is exact, inferred, blocked, or review-required before invoking
    // it. Clean chips cover at least two distinct fix postures and two distinct apply scopes, an
    // inferred-shown-as-exact example degrades, and no clean chip shows an inferred fix as exact.
    let clean_postures: BTreeSet<String> = chips()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.fix_posture.clone())
        .collect();
    let clean_scopes: BTreeSet<String> = chips()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.apply_scope.clone())
        .collect();
    let inferred_as_exact_degrades = chips().any(|ex| {
        ex.degrade_reason == Some(M5CodeActionChipDegradeReason::InferredFixShownAsExact)
    });
    let no_clean_inferred_as_exact =
        chips().all(|ex| !(ex.is_clean() && ex.fix_is_inferred && ex.claims_exact));
    if !(clean_postures.len() >= 2
        && clean_scopes.len() >= 2
        && inferred_as_exact_degrades
        && no_clean_inferred_as_exact)
    {
        violations.push(M5DiagnosticChipControlsViolation::FixPostureLegibilityNotProven);
    }

    // AC3: no claimed M5 inline action path bypasses the broader preview/apply truth. At least one
    // clean chip requires a preview and offers one, a preview-bypass example degrades, no clean chip
    // bypasses a required preview, and a clean decoration and clean chip both offer a command-backed
    // detail entrypoint.
    let preview_clean =
        chips().any(|ex| ex.is_clean() && ex.requires_preview && ex.preview_available);
    let preview_bypass_degrades = chips().any(|ex| {
        ex.degrade_reason == Some(M5CodeActionChipDegradeReason::PreviewRequiredButBypassed)
    });
    let no_clean_bypass =
        chips().all(|ex| !ex.is_clean() || !ex.requires_preview || ex.preview_available);
    let traceable_decoration = decorations().any(|ex| ex.is_clean() && ex.detail_command_available);
    let traceable_chip = chips().any(|ex| ex.is_clean() && ex.detail_command_available);
    if !(preview_clean
        && preview_bypass_degrades
        && no_clean_bypass
        && traceable_decoration
        && traceable_chip)
    {
        violations.push(M5DiagnosticChipControlsViolation::PreviewApplyTruthNotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
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

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The two component families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5EditorInlineComponentFamily; 2] = [
    M5EditorInlineComponentFamily::DiagnosticDecoration,
    M5EditorInlineComponentFamily::CodeActionChip,
];
