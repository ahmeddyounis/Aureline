//! One reusable M5 toolchain-pin-row / precedence-inspector / switch-review-card
//! primitive: the target kind, the current selection, the winning scope and source,
//! the pin / conflict / override state, the ordered winning-versus-shadowed
//! precedence stack, and the predicted blast radius of a context switch, projected
//! the same way across every claimed M5 environment selector.
//!
//! Aureline's frozen runtime-boundary component matrix
//! ([`crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix`])
//! names the toolchain pin row as one governed component family and freezes its
//! controlled vocabulary — the toolchain source classes and the toolchain pin
//! states. This module *implements* that contract, plus the precedence inspector and
//! the switch-review card it needs, as one reusable primitive so a user can review or
//! change one interpreter, SDK, shell, kernel, or runtime choice without guessing
//! which scope currently wins, why it won, what it shadows, or what would change
//! after switching.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_toolchain_selection`] — that takes one toolchain
//!    target's ordered candidate layers (each with its scope, source, and opaque
//!    selection), its selection health, and an optional switch request, and produces
//!    one [`M5ResolvedToolchainSelection`] carrying the derived winning scope and
//!    source, the derived pin state (pinned-resolved versus pinned-missing-fallback
//!    versus unpinned versus pin-conflict versus pin-overridden), the ordered
//!    winning-versus-shadowed precedence stack with an explicit shadow reason on
//!    every overshadowed layer, the available clear / revert / repair actions, and —
//!    when a switch is requested — the predicted blast radius, reversibility, restart
//!    / reconnect requirement, newly blocked actions, and safe local-only fallback.
//!    The resolver never lets a workspace or policy override silently shadow a lower
//!    durable pin, never shows a degraded or mismatched selection as cleanly resolved
//!    without a repair action, and never presents a switch without its blast radius.
//! 2. A parity matrix — [`M5ToolchainPinSwitchReviewPrimitivePacket`] — that binds
//!    one row per claimed M5 environment selector (the status-bar selector, the
//!    command-palette switcher, the settings toolchain row, the interpreter picker,
//!    the SDK selector, the shell-profile picker, the kernel picker, the runtime-
//!    target switcher, and the repair-panel selector) to the shared pin-row anatomy,
//!    precedence-inspector anatomy, and switch-review-card anatomy, the same pin
//!    states, scopes, health states, actions, blast radii, and reversibility classes,
//!    the same export fields, and the same non-visual accessibility routes, so the
//!    winning-scope / shadowed-layer / revert-or-repair truth stays identical on every
//!    surface and the support / export packet reconstructs toolchain resolution from
//!    one shared model.
//!
//! The toolchain source class ([`M5ToolchainSourceClass`]), toolchain pin state
//! ([`M5ToolchainPinState`]), repair blast radius ([`M5RepairBlastRadius`]),
//! reversibility class ([`M5ReversibilityClass`]), non-visual accessibility routes
//! ([`M5RuntimeBoundaryAccessibilityRoute`]), qualification classes
//! ([`M5RuntimeBoundaryQualificationClass`]), and downgrade triggers
//! ([`M5RuntimeBoundaryDowngradeTrigger`]) are reused verbatim from the frozen
//! runtime-boundary matrix; the shell topology — zones, responsive classes, window
//! classes, and consumer surfaces — is reused from the frozen shell-zone matrix. This
//! module mints new vocabulary only for what the frozen matrix left implicit about the
//! pin row, the precedence inspector, and the switch-review card themselves: their
//! environment selectors, their anatomy parts, their target kinds, their pin scopes,
//! their selection-health states, their pin actions, and their export fields. No M5
//! surface invents a second toolchain-selection grammar.
//!
//! Raw pin-file paths, raw version strings, raw usernames, tokens, credentials, and
//! user text bodies stay outside the support boundary; every target title, selection,
//! and switch target is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-toolchain-pin-row.schema.json`](../../../../schemas/ui/m5-toolchain-pin-row.schema.json)
//! and the contract doc is
//! [`docs/components/m5_toolchain_pin_switch_review_primitive_contract.md`](../../../../docs/components/m5_toolchain_pin_switch_review_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-toolchain-pin-switch-review-primitive/`](../../../../fixtures/ui/m5-toolchain-pin-switch-review-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_toolchain_pin_switch_review_primitive_packet,
    seeded_m5_toolchain_pin_switch_review_primitive_repair_panel_beta_narrowed,
    seeded_m5_toolchain_pin_switch_review_primitive_runtime_target_preview_narrowed,
    M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_PACKET_ID,
};

// The toolchain source class, toolchain pin state, repair blast radius, reversibility
// class, accessibility routes, qualification classes, and downgrade triggers are
// frozen once, in the runtime-boundary component matrix. This primitive reuses them
// verbatim so it never invents a parallel toolchain-selection vocabulary.
pub use crate::freeze_the_m5_terminal_tab_remote_target_pill_environment_status_strip_toolchain_pin_row_presence_avatar_stack_and_repair_action_card_component_matrix::{
    M5RepairBlastRadius, M5ReversibilityClass, M5RuntimeBoundaryAccessibilityRoute,
    M5RuntimeBoundaryDowngradeTrigger, M5RuntimeBoundaryQualificationClass, M5ToolchainPinState,
    M5ToolchainSourceClass,
};

// The canonical shell topology — zones, responsive classes, window classes, and
// consumer surfaces — is frozen once, in the shell-zone matrix.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ToolchainPinSwitchReviewPrimitivePacket`].
pub const M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_toolchain_pin_row_precedence_inspector_and_switch_review_card_winning_scope_shadowed_layer_and_revert_or_repair_primitive";

/// Schema version for M5 toolchain-pin / switch-review primitive records.
pub const M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the toolchain-pin-row boundary schema (the packet schema).
pub const M5_TOOLCHAIN_PIN_ROW_SCHEMA_REF: &str = "schemas/ui/m5-toolchain-pin-row.schema.json";

/// Repo-relative path of the companion context-precedence-inspector component schema.
pub const M5_PRECEDENCE_INSPECTOR_SCHEMA_REF: &str =
    "schemas/ui/m5-context-precedence-inspector.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_TOOLCHAIN_PIN_SWITCH_REVIEW_DOC_REF: &str =
    "docs/components/m5_toolchain_pin_switch_review_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds against.
pub const M5_TOOLCHAIN_PIN_SWITCH_REVIEW_SHELL_ZONE_REF: &str =
    "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen runtime-boundary component matrix this primitive
/// narrows from.
pub const M5_TOOLCHAIN_PIN_SWITCH_REVIEW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-runtime-boundary-components.schema.json";

/// Repo-relative path of the toolchain-manager parity contract this primitive projects
/// source / scope / resolution truth from.
pub const M5_TOOLCHAIN_PIN_SWITCH_REVIEW_TOOLCHAIN_MANAGER_REF: &str =
    "schemas/runtime/finalize_environment_and_toolchain_manager_parity_across_ui_truth.schema.json";

/// Repo-relative path of the precedence-resolution contract this primitive projects
/// winning-scope / shadowed-layer truth from.
pub const M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRECEDENCE_REF: &str =
    "schemas/settings/precedence_resolution.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_TOOLCHAIN_PIN_SWITCH_REVIEW_FIXTURE_DIR: &str =
    "fixtures/ui/m5-toolchain-pin-switch-review-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TOOLCHAIN_PIN_SWITCH_REVIEW_ARTIFACT_REF: &str =
    "artifacts/release/m5-toolchain-pin-switch-review-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_TOOLCHAIN_PIN_SWITCH_REVIEW_CSV_REF: &str =
    "artifacts/release/m5-toolchain-pin-switch-review-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_TOOLCHAIN_PIN_SWITCH_REVIEW_REPORT_REF: &str =
    "artifacts/components/m5-toolchain-pin-switch-review-primitive.md";

/// One claimed M5 environment selector that renders the shared toolchain pin row,
/// precedence inspector, and switch-review card. These are the surfaces where a user
/// reviews or changes one interpreter, SDK, shell, kernel, or runtime choice — the
/// status-bar selector, the command-palette switcher, the settings toolchain row, the
/// interpreter picker, the SDK selector, the shell-profile picker, the kernel picker,
/// the runtime-target switcher, and the repair-panel selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EnvironmentSelectorSurface {
    /// The status-bar toolchain selector.
    StatusBarSelector,
    /// The command-palette "select environment" switcher.
    CommandPaletteSwitcher,
    /// The settings toolchain pin row.
    SettingsToolchainRow,
    /// The interpreter picker.
    InterpreterPicker,
    /// The SDK selector.
    SdkSelector,
    /// The shell-profile picker.
    ShellProfilePicker,
    /// The notebook kernel picker.
    KernelPicker,
    /// The runtime-target switcher.
    RuntimeTargetSwitcher,
    /// The Project Doctor repair-panel selector.
    RepairPanelSelector,
}

impl M5EnvironmentSelectorSurface {
    /// Every claimed environment selector, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::StatusBarSelector,
        Self::CommandPaletteSwitcher,
        Self::SettingsToolchainRow,
        Self::InterpreterPicker,
        Self::SdkSelector,
        Self::ShellProfilePicker,
        Self::KernelPicker,
        Self::RuntimeTargetSwitcher,
        Self::RepairPanelSelector,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatusBarSelector => "status_bar_selector",
            Self::CommandPaletteSwitcher => "command_palette_switcher",
            Self::SettingsToolchainRow => "settings_toolchain_row",
            Self::InterpreterPicker => "interpreter_picker",
            Self::SdkSelector => "sdk_selector",
            Self::ShellProfilePicker => "shell_profile_picker",
            Self::KernelPicker => "kernel_picker",
            Self::RuntimeTargetSwitcher => "runtime_target_switcher",
            Self::RepairPanelSelector => "repair_panel_selector",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::StatusBarSelector => "Status-Bar Selector",
            Self::CommandPaletteSwitcher => "Command-Palette Switcher",
            Self::SettingsToolchainRow => "Settings Toolchain Row",
            Self::InterpreterPicker => "Interpreter Picker",
            Self::SdkSelector => "SDK Selector",
            Self::ShellProfilePicker => "Shell-Profile Picker",
            Self::KernelPicker => "Kernel Picker",
            Self::RuntimeTargetSwitcher => "Runtime-Target Switcher",
            Self::RepairPanelSelector => "Repair-Panel Selector",
        }
    }
}

/// The kind of toolchain target a pin row governs, so the same primitive covers one
/// interpreter, SDK, shell, kernel, or runtime choice without a per-kind grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToolchainTargetKind {
    /// A language interpreter (e.g. Python, Ruby, Node).
    Interpreter,
    /// A software development kit (e.g. .NET, JDK).
    Sdk,
    /// A shell / terminal profile.
    Shell,
    /// A notebook kernel.
    Kernel,
    /// A runtime / container target.
    Runtime,
}

impl M5ToolchainTargetKind {
    /// Every toolchain target kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Interpreter,
        Self::Sdk,
        Self::Shell,
        Self::Kernel,
        Self::Runtime,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Interpreter => "interpreter",
            Self::Sdk => "sdk",
            Self::Shell => "shell",
            Self::Kernel => "kernel",
            Self::Runtime => "runtime",
        }
    }
}

/// The scope at which a toolchain layer expresses a selection, ordered from the
/// highest-precedence override down to the lowest-precedence default so a winning
/// layer never hides which scope shadowed a lower one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PinScope {
    /// A managed / policy override (highest precedence).
    PolicyScope,
    /// A session override, effective for this session only.
    SessionScope,
    /// A checked-in project pin.
    ProjectScope,
    /// A workspace setting.
    WorkspaceScope,
    /// A per-user setting.
    UserScope,
    /// A host / system-installed selection.
    HostScope,
    /// A built-in global default (lowest precedence).
    GlobalDefaultScope,
}

impl M5PinScope {
    /// Every pin scope, in precedence order (highest first).
    pub const ALL: [Self; 7] = [
        Self::PolicyScope,
        Self::SessionScope,
        Self::ProjectScope,
        Self::WorkspaceScope,
        Self::UserScope,
        Self::HostScope,
        Self::GlobalDefaultScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyScope => "policy_scope",
            Self::SessionScope => "session_scope",
            Self::ProjectScope => "project_scope",
            Self::WorkspaceScope => "workspace_scope",
            Self::UserScope => "user_scope",
            Self::HostScope => "host_scope",
            Self::GlobalDefaultScope => "global_default_scope",
        }
    }

    /// The precedence rank of this scope; a lower rank wins over a higher one.
    pub const fn precedence_rank(self) -> u8 {
        match self {
            Self::PolicyScope => 0,
            Self::SessionScope => 1,
            Self::ProjectScope => 2,
            Self::WorkspaceScope => 3,
            Self::UserScope => 4,
            Self::HostScope => 5,
            Self::GlobalDefaultScope => 6,
        }
    }

    /// True when this scope is an override that can supersede a durable pin.
    pub const fn is_override(self) -> bool {
        matches!(self, Self::PolicyScope | Self::SessionScope)
    }

    /// True when this scope is a durable, user-authored pin (project / workspace /
    /// user), i.e. one whose silent shadowing must be explained.
    pub const fn is_durable_pin(self) -> bool {
        matches!(
            self,
            Self::ProjectScope | Self::WorkspaceScope | Self::UserScope
        )
    }
}

/// The health of the resolved winning selection, so a stale, mismatched, or missing
/// selection is never shown as cleanly resolved and always keeps an explicit repair
/// action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SelectionHealth {
    /// The resolved selection is present and healthy.
    Healthy,
    /// The resolved selection is stale / degraded and should be refreshed.
    DegradedStale,
    /// The resolved selection version mismatches what the pin requested.
    MismatchedVersion,
    /// The resolved selection is missing / unavailable and a fallback is in use.
    MissingUnavailable,
}

impl M5SelectionHealth {
    /// Every selection-health state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Healthy,
        Self::DegradedStale,
        Self::MismatchedVersion,
        Self::MissingUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::DegradedStale => "degraded_stale",
            Self::MismatchedVersion => "mismatched_version",
            Self::MissingUnavailable => "missing_unavailable",
        }
    }

    /// True when this health state is degraded / mismatched / missing and therefore
    /// requires an explicit repair action.
    pub const fn is_degraded(self) -> bool {
        !matches!(self, Self::Healthy)
    }
}

/// One action a toolchain pin row can offer, so a review or change is never a
/// dead-end and a degraded selection always keeps a repair path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PinAction {
    /// Open the precedence inspector to review the winning and shadowed layers.
    ReviewPrecedence,
    /// Clear the winning override so a lower durable pin resumes.
    ClearOverride,
    /// Revert to a shadowed durable pin.
    RevertToShadowedPin,
    /// Repair the degraded / mismatched / missing selection.
    RepairSelection,
}

impl M5PinAction {
    /// Every pin action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReviewPrecedence,
        Self::ClearOverride,
        Self::RevertToShadowedPin,
        Self::RepairSelection,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewPrecedence => "review_precedence",
            Self::ClearOverride => "clear_override",
            Self::RevertToShadowedPin => "revert_to_shadowed_pin",
            Self::RepairSelection => "repair_selection",
        }
    }
}

/// One anatomy part the shared toolchain pin row surfaces. The parts in
/// [`M5ToolchainPinRowPart::MANDATORY`] are required on every row so a user can tell
/// what target it governs, what won, at which scope, and from which source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToolchainPinRowPart {
    /// The toolchain target kind.
    TargetKind,
    /// The current resolved selection.
    CurrentSelection,
    /// The winning scope cue.
    WinningScope,
    /// The winning source / provenance cue.
    SourceProvenance,
    /// The conflict / override note.
    ConflictOrOverrideNote,
    /// The clear / revert action affordance.
    ClearOrRevertAction,
}

impl M5ToolchainPinRowPart {
    /// Every pin-row part, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TargetKind,
        Self::CurrentSelection,
        Self::WinningScope,
        Self::SourceProvenance,
        Self::ConflictOrOverrideNote,
        Self::ClearOrRevertAction,
    ];

    /// The pin-row parts every toolchain pin row must render.
    pub const MANDATORY: [Self; 4] = [
        Self::TargetKind,
        Self::CurrentSelection,
        Self::WinningScope,
        Self::SourceProvenance,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetKind => "target_kind",
            Self::CurrentSelection => "current_selection",
            Self::WinningScope => "winning_scope",
            Self::SourceProvenance => "source_provenance",
            Self::ConflictOrOverrideNote => "conflict_or_override_note",
            Self::ClearOrRevertAction => "clear_or_revert_action",
        }
    }
}

/// One anatomy part the shared precedence inspector surfaces. The parts in
/// [`M5PrecedenceInspectorPart::MANDATORY`] are required so a user can read the
/// ordered winning stack, the overshadowed candidates, and why each was shadowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrecedenceInspectorPart {
    /// The ordered winning stack.
    OrderedWinningStack,
    /// The overshadowed candidate layers.
    OvershadowedCandidates,
    /// The explanation of why each candidate was shadowed.
    ShadowExplanation,
    /// The surfaces the winning selection affects.
    AffectedSurfaces,
}

impl M5PrecedenceInspectorPart {
    /// Every precedence-inspector part, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OrderedWinningStack,
        Self::OvershadowedCandidates,
        Self::ShadowExplanation,
        Self::AffectedSurfaces,
    ];

    /// The inspector parts every precedence inspector must render.
    pub const MANDATORY: [Self; 3] = [
        Self::OrderedWinningStack,
        Self::OvershadowedCandidates,
        Self::ShadowExplanation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrderedWinningStack => "ordered_winning_stack",
            Self::OvershadowedCandidates => "overshadowed_candidates",
            Self::ShadowExplanation => "shadow_explanation",
            Self::AffectedSurfaces => "affected_surfaces",
        }
    }
}

/// One anatomy part the shared switch-review card surfaces. The parts in
/// [`M5SwitchReviewCardPart::MANDATORY`] are required so a user can see the immediate
/// changes, the restart / reconnect requirement, and the safe local-only fallback
/// before switching context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SwitchReviewCardPart {
    /// The immediate-changes summary.
    ImmediateChanges,
    /// The restart / reconnect requirement.
    RestartOrReconnectRequirement,
    /// The newly blocked actions.
    NewlyBlockedActions,
    /// The safe local-only fallback.
    SafeLocalOnlyFallback,
    /// The blast radius and reversibility.
    BlastRadiusAndReversibility,
}

impl M5SwitchReviewCardPart {
    /// Every switch-review-card part, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ImmediateChanges,
        Self::RestartOrReconnectRequirement,
        Self::NewlyBlockedActions,
        Self::SafeLocalOnlyFallback,
        Self::BlastRadiusAndReversibility,
    ];

    /// The card parts every switch-review card must render.
    pub const MANDATORY: [Self; 3] = [
        Self::ImmediateChanges,
        Self::RestartOrReconnectRequirement,
        Self::SafeLocalOnlyFallback,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImmediateChanges => "immediate_changes",
            Self::RestartOrReconnectRequirement => "restart_or_reconnect_requirement",
            Self::NewlyBlockedActions => "newly_blocked_actions",
            Self::SafeLocalOnlyFallback => "safe_local_only_fallback",
            Self::BlastRadiusAndReversibility => "blast_radius_and_reversibility",
        }
    }
}

/// A field the support / export packet carries so toolchain resolution is
/// reconstructable from the shared model. The fields in
/// [`M5ToolchainSelectionExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ToolchainSelectionExportField {
    /// The toolchain target kind.
    TargetKind,
    /// The opaque current-selection representation.
    CurrentSelection,
    /// The winning scope.
    WinningScope,
    /// The winning source.
    WinningSource,
    /// The derived pin state.
    PinState,
    /// The ordered winning-versus-shadowed layers.
    ShadowedLayers,
    /// The conflict / override disclosure.
    ConflictOrOverride,
    /// The selection health.
    SelectionHealth,
    /// The predicted switch blast radius.
    SwitchBlastRadius,
    /// The predicted switch reversibility.
    SwitchReversibility,
    /// The available actions.
    AvailableActions,
}

impl M5ToolchainSelectionExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::TargetKind,
        Self::CurrentSelection,
        Self::WinningScope,
        Self::WinningSource,
        Self::PinState,
        Self::ShadowedLayers,
        Self::ConflictOrOverride,
        Self::SelectionHealth,
        Self::SwitchBlastRadius,
        Self::SwitchReversibility,
        Self::AvailableActions,
    ];

    /// The export fields every toolchain-selection export must carry.
    pub const MANDATORY: [Self; 6] = [
        Self::TargetKind,
        Self::WinningScope,
        Self::WinningSource,
        Self::PinState,
        Self::SelectionHealth,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetKind => "target_kind",
            Self::CurrentSelection => "current_selection",
            Self::WinningScope => "winning_scope",
            Self::WinningSource => "winning_source",
            Self::PinState => "pin_state",
            Self::ShadowedLayers => "shadowed_layers",
            Self::ConflictOrOverride => "conflict_or_override",
            Self::SelectionHealth => "selection_health",
            Self::SwitchBlastRadius => "switch_blast_radius",
            Self::SwitchReversibility => "switch_reversibility",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// One candidate layer expressing a toolchain selection at one scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PinCandidateLayer {
    /// The scope this layer expresses a selection at.
    pub scope: M5PinScope,
    /// The source class that would supply the selection at this scope.
    pub source: M5ToolchainSourceClass,
    /// The opaque, export-safe selection this layer would resolve to.
    pub selection_repr: String,
    /// Whether this layer actually contributes a selection right now.
    pub present: bool,
}

/// A requested context switch whose blast radius the switch-review card predicts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SwitchRequest {
    /// The scope the switch would write to.
    pub to_scope: M5PinScope,
    /// The source the switch would select from.
    pub to_source: M5ToolchainSourceClass,
    /// The opaque, export-safe selection the switch would move to.
    pub to_selection_repr: String,
    /// Whether the switch requires a runtime restart.
    pub requires_restart: bool,
    /// Whether the switch requires a remote reconnect.
    pub requires_reconnect: bool,
    /// Opaque tokens for actions that would be newly blocked after the switch.
    pub newly_blocked_actions: Vec<String>,
    /// Whether a safe local-only fallback is available if the switch is declined.
    pub safe_local_only_fallback: bool,
}

/// One ranked layer in the resolved precedence stack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RankedLayer {
    /// The layer's scope.
    pub scope: M5PinScope,
    /// The layer's source.
    pub source: M5ToolchainSourceClass,
    /// The opaque selection this layer resolves to.
    pub selection_repr: String,
    /// The layer's precedence rank (lower wins).
    pub precedence_rank: u8,
    /// True when this layer is the winning layer.
    pub is_winner: bool,
    /// True when this layer is a durable, user-authored pin.
    pub is_durable_pin: bool,
    /// The reason this layer was shadowed, present for every non-winning layer.
    pub shadow_reason: Option<String>,
}

/// The predicted review of a requested context switch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SwitchReview {
    /// The scope the switch would write to.
    pub to_scope: M5PinScope,
    /// The source the switch would select from.
    pub to_source: M5ToolchainSourceClass,
    /// The opaque selection the switch would move to.
    pub to_selection_repr: String,
    /// True when the switch requires a runtime restart.
    pub restart_required: bool,
    /// True when the switch requires a remote reconnect.
    pub reconnect_required: bool,
    /// Opaque tokens for actions that would be newly blocked after the switch.
    pub newly_blocked_actions: Vec<String>,
    /// True when the switch would newly block some actions.
    pub blocks_actions_after_switch: bool,
    /// True when a safe local-only fallback is available if the switch is declined.
    pub safe_local_only_fallback: bool,
    /// The derived blast radius of the switch.
    pub blast_radius: M5RepairBlastRadius,
    /// The derived reversibility of the switch.
    pub reversibility: M5ReversibilityClass,
}

/// The full input to the toolchain-selection resolver for one environment selector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToolchainSelectionResolutionInput {
    /// The opaque, export-safe target-title representation.
    pub target_title: String,
    /// The kind of toolchain target this row governs.
    pub target_kind: M5ToolchainTargetKind,
    /// The candidate layers, in any order.
    pub candidate_layers: Vec<M5PinCandidateLayer>,
    /// The health of the resolved winning selection.
    pub selection_health: M5SelectionHealth,
    /// An optional requested switch to predict.
    pub switch_request: Option<M5SwitchRequest>,
}

/// The resolved toolchain selection for one environment selector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedToolchainSelection {
    /// The opaque target-title representation.
    pub target_title: String,
    /// The kind of toolchain target this row governs.
    pub target_kind: M5ToolchainTargetKind,
    /// The winning scope.
    pub winning_scope: M5PinScope,
    /// The winning source.
    pub winning_source: M5ToolchainSourceClass,
    /// The opaque winning-selection representation.
    pub winning_selection_repr: String,
    /// The derived pin state.
    pub pin_state: M5ToolchainPinState,
    /// The health of the winning selection.
    pub selection_health: M5SelectionHealth,
    /// True when the winning selection is degraded / mismatched / missing.
    pub selection_is_degraded: bool,
    /// The ordered precedence stack (winner first), one entry per present layer.
    pub ordered_layers: Vec<M5RankedLayer>,
    /// True when a shadowed durable pin is present and disclosed. Always `true` when
    /// [`Self::shadows_durable_pin`] is `true`.
    pub discloses_shadowed_pins: bool,
    /// True when the winner shadows a lower durable pin that resolves differently.
    pub shadows_durable_pin: bool,
    /// The actions this row exposes.
    pub available_actions: Vec<M5PinAction>,
    /// True when a degraded selection exposes a repair action (or the selection is
    /// healthy). Always `true`.
    pub exposes_repair_when_degraded: bool,
    /// The predicted switch review, when a switch was requested.
    pub switch_review: Option<M5SwitchReview>,
}

/// Errors returned by [`resolve_toolchain_selection`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ToolchainSelectionResolutionError {
    /// The target title was empty.
    EmptyTargetTitle,
    /// No candidate layers were supplied.
    EmptyCandidateLayers,
    /// No candidate layer was present, so nothing could win.
    NoPresentLayer,
    /// Two candidate layers shared the same scope.
    DuplicateScope,
    /// A present candidate layer had an empty selection.
    EmptySelection,
    /// A requested switch had an empty target selection.
    EmptySwitchSelection,
    /// A representation carried forbidden material.
    ForbiddenToolchainMaterial,
}

impl M5ToolchainSelectionResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyTargetTitle => "empty_target_title",
            Self::EmptyCandidateLayers => "empty_candidate_layers",
            Self::NoPresentLayer => "no_present_layer",
            Self::DuplicateScope => "duplicate_scope",
            Self::EmptySelection => "empty_selection",
            Self::EmptySwitchSelection => "empty_switch_selection",
            Self::ForbiddenToolchainMaterial => "forbidden_toolchain_material",
        }
    }
}

impl fmt::Display for M5ToolchainSelectionResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "toolchain-selection resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ToolchainSelectionResolutionError {}

/// Resolves one toolchain target's pin row, precedence inspector, and switch-review
/// card from its candidate layers, selection health, and optional switch request.
///
/// The winning layer is the present candidate with the lowest precedence rank; every
/// other present layer is ranked below it and carries an explicit shadow reason, so a
/// workspace or policy override can never silently shadow a lower durable pin. The pin
/// state is derived: an unpinned target (only defaults present) reads unpinned, a
/// missing selection reads pinned-missing-fallback, an override shadowing a durable
/// pin reads pin-overridden, two durable pins disagreeing read pin-conflict, and a
/// clean resolution reads pinned-resolved. A degraded / mismatched / missing selection
/// always keeps an explicit repair action, and — when a switch is requested — the
/// predicted blast radius and reversibility are always resolved.
pub fn resolve_toolchain_selection(
    input: &M5ToolchainSelectionResolutionInput,
) -> Result<M5ResolvedToolchainSelection, M5ToolchainSelectionResolutionError> {
    if input.target_title.trim().is_empty() {
        return Err(M5ToolchainSelectionResolutionError::EmptyTargetTitle);
    }
    if value_repr_is_forbidden(&input.target_title) {
        return Err(M5ToolchainSelectionResolutionError::ForbiddenToolchainMaterial);
    }
    if input.candidate_layers.is_empty() {
        return Err(M5ToolchainSelectionResolutionError::EmptyCandidateLayers);
    }

    let mut seen_scopes: BTreeSet<M5PinScope> = BTreeSet::new();
    for layer in &input.candidate_layers {
        if !seen_scopes.insert(layer.scope) {
            return Err(M5ToolchainSelectionResolutionError::DuplicateScope);
        }
        if layer.present && layer.selection_repr.trim().is_empty() {
            return Err(M5ToolchainSelectionResolutionError::EmptySelection);
        }
        if value_repr_is_forbidden(&layer.selection_repr) {
            return Err(M5ToolchainSelectionResolutionError::ForbiddenToolchainMaterial);
        }
    }

    let present: Vec<&M5PinCandidateLayer> = input
        .candidate_layers
        .iter()
        .filter(|layer| layer.present)
        .collect();
    if present.is_empty() {
        return Err(M5ToolchainSelectionResolutionError::NoPresentLayer);
    }

    // The winning layer is the present candidate with the lowest precedence rank.
    let winner = present
        .iter()
        .copied()
        .min_by_key(|layer| layer.scope.precedence_rank())
        .expect("at least one present layer");

    // The ordered stack: every present layer sorted by precedence rank, winner first,
    // with an explicit shadow reason on every non-winning layer.
    let mut ordered_present = present.clone();
    ordered_present.sort_by_key(|layer| layer.scope.precedence_rank());
    let ordered_layers: Vec<M5RankedLayer> = ordered_present
        .iter()
        .map(|layer| {
            let is_winner = layer.scope == winner.scope;
            M5RankedLayer {
                scope: layer.scope,
                source: layer.source,
                selection_repr: layer.selection_repr.clone(),
                precedence_rank: layer.scope.precedence_rank(),
                is_winner,
                is_durable_pin: layer.scope.is_durable_pin(),
                shadow_reason: if is_winner {
                    None
                } else {
                    Some(format!(
                        "shadowed by higher-precedence {}",
                        winner.scope.as_str()
                    ))
                },
            }
        })
        .collect();

    let shadows_durable_pin = ordered_layers.iter().any(|layer| {
        !layer.is_winner && layer.is_durable_pin && layer.selection_repr != winner.selection_repr
    });

    let has_explicit_pin = present
        .iter()
        .any(|layer| layer.scope.is_override() || layer.scope.is_durable_pin());
    let durable_conflict = present.iter().any(|layer| {
        layer.scope != winner.scope
            && layer.scope.is_durable_pin()
            && layer.selection_repr != winner.selection_repr
    });

    let pin_state = if !has_explicit_pin {
        M5ToolchainPinState::Unpinned
    } else if input.selection_health == M5SelectionHealth::MissingUnavailable {
        M5ToolchainPinState::PinnedMissingFallback
    } else if winner.scope.is_override() && durable_conflict {
        M5ToolchainPinState::PinOverridden
    } else if winner.scope.is_durable_pin() && durable_conflict {
        M5ToolchainPinState::PinConflict
    } else {
        M5ToolchainPinState::PinnedResolved
    };

    let mut available_actions = vec![M5PinAction::ReviewPrecedence];
    if winner.scope.is_override() {
        available_actions.push(M5PinAction::ClearOverride);
    }
    if ordered_layers
        .iter()
        .any(|layer| !layer.is_winner && layer.is_durable_pin)
    {
        available_actions.push(M5PinAction::RevertToShadowedPin);
    }
    if input.selection_health.is_degraded() {
        available_actions.push(M5PinAction::RepairSelection);
    }

    let switch_review = input.switch_request.as_ref().map(|request| {
        let blast_radius = if request.requires_reconnect {
            M5RepairBlastRadius::MultiTargetScoped
        } else if matches!(
            request.to_scope,
            M5PinScope::PolicyScope | M5PinScope::HostScope | M5PinScope::GlobalDefaultScope
        ) {
            M5RepairBlastRadius::HostEnvironmentScoped
        } else if request.requires_restart {
            M5RepairBlastRadius::ToolchainScoped
        } else {
            M5RepairBlastRadius::WorkspaceScoped
        };
        let reversibility = if request.safe_local_only_fallback && !request.requires_reconnect {
            M5ReversibilityClass::FullyReversibleCheckpoint
        } else if request.safe_local_only_fallback {
            M5ReversibilityClass::ReversibleWithBackup
        } else if request.requires_reconnect {
            M5ReversibilityClass::ReversalRequiresManualSteps
        } else if request.requires_restart {
            M5ReversibilityClass::PartiallyReversible
        } else {
            M5ReversibilityClass::ReversibleWithBackup
        };
        M5SwitchReview {
            to_scope: request.to_scope,
            to_source: request.to_source,
            to_selection_repr: request.to_selection_repr.clone(),
            restart_required: request.requires_restart,
            reconnect_required: request.requires_reconnect,
            newly_blocked_actions: request.newly_blocked_actions.clone(),
            blocks_actions_after_switch: !request.newly_blocked_actions.is_empty(),
            safe_local_only_fallback: request.safe_local_only_fallback,
            blast_radius,
            reversibility,
        }
    });

    if let Some(request) = &input.switch_request {
        if request.to_selection_repr.trim().is_empty() {
            return Err(M5ToolchainSelectionResolutionError::EmptySwitchSelection);
        }
        if value_repr_is_forbidden(&request.to_selection_repr) {
            return Err(M5ToolchainSelectionResolutionError::ForbiddenToolchainMaterial);
        }
    }

    Ok(M5ResolvedToolchainSelection {
        target_title: input.target_title.clone(),
        target_kind: input.target_kind,
        winning_scope: winner.scope,
        winning_source: winner.source,
        winning_selection_repr: winner.selection_repr.clone(),
        pin_state,
        selection_health: input.selection_health,
        selection_is_degraded: input.selection_health.is_degraded(),
        ordered_layers,
        discloses_shadowed_pins: true,
        shadows_durable_pin,
        available_actions,
        exposes_repair_when_degraded: true,
        switch_review,
    })
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs toolchain resolution from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToolchainSelectionResolutionCase {
    /// The resolver input.
    pub input: M5ToolchainSelectionResolutionInput,
    /// The resolved truth. Must equal `resolve_toolchain_selection(&input)`.
    pub resolved: M5ResolvedToolchainSelection,
}

impl M5ToolchainSelectionResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ToolchainSelectionResolutionInput) -> Self {
        let resolved = resolve_toolchain_selection(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_toolchain_selection(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one environment selector bound to the shared pin
/// row, precedence inspector, and switch-review card anatomy, pin states, scopes,
/// health states, actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvironmentSelectorRow {
    /// Environment selector family.
    pub selector_surface: M5EnvironmentSelectorSurface,
    /// Qualification class earned by this surface.
    pub qualification: M5RuntimeBoundaryQualificationClass,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this pin row / inspector / card attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this component must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this component keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Pin-row parts this surface renders (must include the mandatory parts).
    pub pin_row_parts: Vec<M5ToolchainPinRowPart>,
    /// Precedence-inspector parts this surface renders (must include the mandatory
    /// parts).
    pub inspector_parts: Vec<M5PrecedenceInspectorPart>,
    /// Switch-review-card parts this surface renders (must include the mandatory
    /// parts).
    pub switch_card_parts: Vec<M5SwitchReviewCardPart>,
    /// Target kinds this surface governs.
    pub target_kinds: Vec<M5ToolchainTargetKind>,
    /// Pin states this surface distinguishes.
    pub pin_states: Vec<M5ToolchainPinState>,
    /// Pin scopes this surface distinguishes.
    pub pin_scopes: Vec<M5PinScope>,
    /// Selection-health states this surface distinguishes.
    pub selection_health_states: Vec<M5SelectionHealth>,
    /// Pin actions this surface offers.
    pub pin_actions: Vec<M5PinAction>,
    /// Export fields this surface carries (must include the mandatory fields).
    pub export_fields: Vec<M5ToolchainSelectionExportField>,
    /// Non-visual accessibility routes this surface offers.
    pub accessibility_routes: Vec<M5RuntimeBoundaryAccessibilityRoute>,
    /// Shell subsystems that consume this surface's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5RuntimeBoundaryDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface.
    pub example_resolutions: Vec<M5ToolchainSelectionResolutionCase>,
    /// Hard invariant: this surface never lets an override silently shadow a durable
    /// pin. MUST be `false`.
    pub silently_shadows_durable_pin: bool,
    /// Hard invariant: this surface never shows a degraded selection as cleanly
    /// resolved. MUST be `false`.
    pub shows_degraded_as_resolved: bool,
    /// Hard invariant: this surface never invents a private selection grammar. MUST be
    /// `false`.
    pub invents_private_selection_grammar: bool,
    /// Hard invariant: this surface never presents a switch without its blast radius.
    /// MUST be `false`.
    pub hides_switch_blast_radius: bool,
}

impl M5EnvironmentSelectorRow {
    /// True when the row declares every mandatory pin-row part.
    fn declares_mandatory_pin_row_parts(&self) -> bool {
        let present: BTreeSet<M5ToolchainPinRowPart> = self.pin_row_parts.iter().copied().collect();
        M5ToolchainPinRowPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory inspector part.
    fn declares_mandatory_inspector_parts(&self) -> bool {
        let present: BTreeSet<M5PrecedenceInspectorPart> =
            self.inspector_parts.iter().copied().collect();
        M5PrecedenceInspectorPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory switch-card part.
    fn declares_mandatory_switch_card_parts(&self) -> bool {
        let present: BTreeSet<M5SwitchReviewCardPart> =
            self.switch_card_parts.iter().copied().collect();
        M5SwitchReviewCardPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ToolchainSelectionExportField> =
            self.export_fields.iter().copied().collect();
        M5ToolchainSelectionExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.silently_shadows_durable_pin
            && !self.shows_degraded_as_resolved
            && !self.invents_private_selection_grammar
            && !self.hides_switch_blast_radius
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToolchainPinSwitchReviewVocabularySet {
    /// Environment-selector tokens.
    pub selector_surfaces: Vec<String>,
    /// Pin-row-part tokens.
    pub pin_row_parts: Vec<String>,
    /// Precedence-inspector-part tokens.
    pub inspector_parts: Vec<String>,
    /// Switch-review-card-part tokens.
    pub switch_card_parts: Vec<String>,
    /// Target-kind tokens.
    pub target_kinds: Vec<String>,
    /// Pin-scope tokens.
    pub pin_scopes: Vec<String>,
    /// Selection-health tokens.
    pub selection_health_states: Vec<String>,
    /// Pin-action tokens.
    pub pin_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Pin-state tokens (reused from the frozen matrix).
    pub pin_states: Vec<String>,
    /// Toolchain-source-class tokens (reused from the frozen matrix).
    pub toolchain_source_classes: Vec<String>,
    /// Repair-blast-radius tokens (reused from the frozen matrix).
    pub blast_radii: Vec<String>,
    /// Reversibility-class tokens (reused from the frozen matrix).
    pub reversibility_classes: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5ToolchainPinSwitchReviewVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            selector_surfaces: tokens(&M5EnvironmentSelectorSurface::ALL, |v| v.as_str()),
            pin_row_parts: tokens(&M5ToolchainPinRowPart::ALL, |v| v.as_str()),
            inspector_parts: tokens(&M5PrecedenceInspectorPart::ALL, |v| v.as_str()),
            switch_card_parts: tokens(&M5SwitchReviewCardPart::ALL, |v| v.as_str()),
            target_kinds: tokens(&M5ToolchainTargetKind::ALL, |v| v.as_str()),
            pin_scopes: tokens(&M5PinScope::ALL, |v| v.as_str()),
            selection_health_states: tokens(&M5SelectionHealth::ALL, |v| v.as_str()),
            pin_actions: tokens(&M5PinAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ToolchainSelectionExportField::ALL, |v| v.as_str()),
            pin_states: tokens(&M5ToolchainPinState::ALL, |v| v.as_str()),
            toolchain_source_classes: tokens(&M5ToolchainSourceClass::ALL, |v| v.as_str()),
            blast_radii: tokens(&M5RepairBlastRadius::ALL, |v| v.as_str()),
            reversibility_classes: tokens(&M5ReversibilityClass::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5RuntimeBoundaryAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5ToolchainPinSwitchReviewGovernanceReview {
    /// One primitive carries pin, precedence, and switch truth on every surface.
    pub one_primitive_carries_pin_precedence_and_switch: bool,
    /// The target kind, current selection, winning scope, and source are always shown.
    pub target_kind_selection_scope_and_source_always_shown: bool,
    /// A workspace or policy override never silently shadows a durable pin.
    pub override_never_silently_shadows_durable_pin: bool,
    /// The winning and shadowed layers are always inspectable before switching.
    pub winning_and_shadowed_layers_always_inspectable: bool,
    /// The predicted blast radius is always shown before a switch.
    pub predicted_blast_radius_always_shown_before_switch: bool,
    /// A degraded or mismatched selection always keeps an explicit repair action.
    pub degraded_selection_always_keeps_repair_action: bool,
    /// The support / export packet reconstructs pin / precedence / switch truth.
    pub support_export_reconstructs_pin_precedence_switch: bool,
    /// No surface invents a second toolchain-selection grammar.
    pub no_surface_invents_second_selection_grammar: bool,
    /// Every row is bound to a canonical shell zone.
    pub every_row_bound_to_shell_zone: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel toolchain-selection vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToolchainPinSwitchReviewConsumerProjection {
    /// Status-bar, palette, settings, picker, switcher, and repair surfaces all consume
    /// the shared primitive.
    pub environment_selectors_consume_shared_primitive: bool,
    /// The pin resolver reads a single canonical precedence source.
    pub pin_resolver_reads_single_precedence_source: bool,
    /// The precedence inspector reads a single canonical layer source.
    pub precedence_inspector_reads_single_layer_source: bool,
    /// The switch-review card reads a single canonical switch source.
    pub switch_review_reads_single_switch_source: bool,
    /// Support / export reads a single canonical selection source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToolchainPinSwitchReviewProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToolchainPinSwitchReviewReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting toolchain-selection audit.
    pub selection_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ToolchainPinSwitchReviewPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ToolchainPinSwitchReviewPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Selector rows.
    pub selector_rows: Vec<M5EnvironmentSelectorRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ToolchainPinSwitchReviewVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ToolchainPinSwitchReviewGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ToolchainPinSwitchReviewConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ToolchainPinSwitchReviewProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ToolchainPinSwitchReviewReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 toolchain-pin / switch-review primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ToolchainPinSwitchReviewPrimitivePacket {
    /// Record kind; must equal
    /// [`M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Selector rows.
    pub selector_rows: Vec<M5EnvironmentSelectorRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ToolchainPinSwitchReviewVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ToolchainPinSwitchReviewGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ToolchainPinSwitchReviewConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ToolchainPinSwitchReviewProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ToolchainPinSwitchReviewReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ToolchainPinSwitchReviewPrimitivePacket {
    /// Builds an M5 toolchain-pin / switch-review primitive packet from stable-lane
    /// input.
    pub fn new(input: M5ToolchainPinSwitchReviewPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            selector_rows: input.selector_rows,
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

    /// Validates the M5 toolchain-pin / switch-review primitive invariants.
    pub fn validate(&self) -> Vec<M5ToolchainPinSwitchReviewPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_RECORD_KIND {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_selector_rows(self, &mut violations);
        validate_shadow_disclosure_covered(self, &mut violations);
        validate_switch_blast_radius_covered(self, &mut violations);
        validate_degraded_repair_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 toolchain-pin / switch-review primitive packet serializes"),
        ) {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::RawMaterialInExport);
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
            .expect("m5 toolchain-pin / switch-review primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per environment selector.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "selector_surface,qualification,owner,shell_zone_slot,pin_row_parts,inspector_parts,switch_card_parts,target_kinds,pin_states,pin_scopes,selection_health_states,pin_actions,export_fields,example_count\n",
        );
        for row in &self.selector_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.selector_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.pin_row_parts, |v| v.as_str()),
                join_tokens(&row.inspector_parts, |v| v.as_str()),
                join_tokens(&row.switch_card_parts, |v| v.as_str()),
                join_tokens(&row.target_kinds, |v| v.as_str()),
                join_tokens(&row.pin_states, |v| v.as_str()),
                join_tokens(&row.pin_scopes, |v| v.as_str()),
                join_tokens(&row.selection_health_states, |v| v.as_str()),
                join_tokens(&row.pin_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .selector_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Toolchain-Pin Row, Precedence Inspector, and Switch-Review Card Primitive: Winning Scope, Shadowed Layers, and Revert-or-Repair\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Environment selectors: {} ({} stable)\n",
            self.selector_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Target kinds: {}\n",
            self.vocabulary_set.target_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Pin states: {}\n",
            self.vocabulary_set.pin_states.join(", ")
        ));
        out.push_str(&format!(
            "- Pin scopes: {}\n",
            self.vocabulary_set.pin_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Selection-health states: {}\n",
            self.vocabulary_set.selection_health_states.join(", ")
        ));
        out.push_str(&format!(
            "- Pin actions: {}\n",
            self.vocabulary_set.pin_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Environment selectors\n\n");
        for row in &self.selector_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.selector_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                let switch = match &case.resolved.switch_review {
                    Some(review) => format!(
                        "switch → {} ({})",
                        review.blast_radius.as_str(),
                        review.reversibility.as_str()
                    ),
                    None => "no switch".to_owned(),
                };
                out.push_str(&format!(
                    "    - `{}` ({}) → won at `{}`, `{}` (source `{}`, health `{}`, {})\n",
                    case.resolved.target_title,
                    case.resolved.target_kind.as_str(),
                    case.resolved.winning_scope.as_str(),
                    case.resolved.pin_state.as_str(),
                    case.resolved.winning_source.as_str(),
                    case.resolved.selection_health.as_str(),
                    switch,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 toolchain-pin / switch-review export.
#[derive(Debug)]
pub enum M5ToolchainPinSwitchReviewPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ToolchainPinSwitchReviewPrimitiveViolation>),
}

impl fmt::Display for M5ToolchainPinSwitchReviewPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 toolchain-pin / switch-review primitive export parse failed: {error}"
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
                    "m5 toolchain-pin / switch-review primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ToolchainPinSwitchReviewPrimitiveArtifactError {}

/// Validation failures emitted by
/// [`M5ToolchainPinSwitchReviewPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ToolchainPinSwitchReviewPrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required environment-selector family is missing from the matrix.
    RequiredSelectorMissing,
    /// A selector row is incomplete.
    SelectorRowIncomplete,
    /// A selector row omits one of the mandatory pin-row parts.
    MandatoryPinRowPartMissing,
    /// A selector row omits one of the mandatory precedence-inspector parts.
    MandatoryInspectorPartMissing,
    /// A selector row omits one of the mandatory switch-review-card parts.
    MandatorySwitchCardPartMissing,
    /// A selector row declares no target kinds.
    TargetKindMissing,
    /// A selector row declares no pin states.
    PinStateMissing,
    /// A selector row declares no pin scopes.
    PinScopeMissing,
    /// A selector row declares no selection-health states.
    SelectionHealthMissing,
    /// A selector row declares no pin actions.
    PinActionMissing,
    /// A selector row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A selector row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A selector row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A selector row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A selector row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A surface claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// No worked resolution proves an override shadowing a durable pin with the shadow
    /// disclosed.
    ShadowDisclosureUnproven,
    /// No worked resolution proves a requested switch resolving to a blast radius and
    /// reversibility.
    SwitchBlastRadiusUnproven,
    /// No worked resolution proves a degraded selection keeping an explicit repair
    /// action.
    DegradedRepairUnproven,
    /// A selector row violates a hard invariant.
    SelectorInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ToolchainPinSwitchReviewPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSelectorMissing => "required_selector_missing",
            Self::SelectorRowIncomplete => "selector_row_incomplete",
            Self::MandatoryPinRowPartMissing => "mandatory_pin_row_part_missing",
            Self::MandatoryInspectorPartMissing => "mandatory_inspector_part_missing",
            Self::MandatorySwitchCardPartMissing => "mandatory_switch_card_part_missing",
            Self::TargetKindMissing => "target_kind_missing",
            Self::PinStateMissing => "pin_state_missing",
            Self::PinScopeMissing => "pin_scope_missing",
            Self::SelectionHealthMissing => "selection_health_missing",
            Self::PinActionMissing => "pin_action_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::ShadowDisclosureUnproven => "shadow_disclosure_unproven",
            Self::SwitchBlastRadiusUnproven => "switch_blast_radius_unproven",
            Self::DegradedRepairUnproven => "degraded_repair_unproven",
            Self::SelectorInvariantViolated => "selector_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 toolchain-pin / switch-review export.
pub fn current_stable_m5_toolchain_pin_switch_review_primitive_export() -> Result<
    M5ToolchainPinSwitchReviewPrimitivePacket,
    M5ToolchainPinSwitchReviewPrimitiveArtifactError,
> {
    let packet: M5ToolchainPinSwitchReviewPrimitivePacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-toolchain-pin-switch-review-proof/support_export.json"
        )))
        .map_err(M5ToolchainPinSwitchReviewPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ToolchainPinSwitchReviewPrimitiveArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ToolchainPinSwitchReviewPrimitivePacket,
    violations: &mut Vec<M5ToolchainPinSwitchReviewPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TOOLCHAIN_PIN_ROW_SCHEMA_REF,
        M5_PRECEDENCE_INSPECTOR_SCHEMA_REF,
        M5_TOOLCHAIN_PIN_SWITCH_REVIEW_DOC_REF,
        M5_TOOLCHAIN_PIN_SWITCH_REVIEW_SHELL_ZONE_REF,
        M5_TOOLCHAIN_PIN_SWITCH_REVIEW_COMPONENT_MATRIX_REF,
        M5_TOOLCHAIN_PIN_SWITCH_REVIEW_TOOLCHAIN_MANAGER_REF,
        M5_TOOLCHAIN_PIN_SWITCH_REVIEW_PRECEDENCE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ToolchainPinSwitchReviewPrimitivePacket,
    violations: &mut Vec<M5ToolchainPinSwitchReviewPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_selector_rows(
    packet: &M5ToolchainPinSwitchReviewPrimitivePacket,
    violations: &mut Vec<M5ToolchainPinSwitchReviewPrimitiveViolation>,
) {
    let present: BTreeSet<M5EnvironmentSelectorSurface> = packet
        .selector_rows
        .iter()
        .map(|row| row.selector_surface)
        .collect();
    for required in M5EnvironmentSelectorSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::RequiredSelectorMissing);
            return;
        }
    }

    for row in &packet.selector_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.pin_row_parts.is_empty()
            || row.inspector_parts.is_empty()
            || row.switch_card_parts.is_empty()
        {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::SelectorRowIncomplete);
        }
        if !row.declares_mandatory_pin_row_parts() {
            violations
                .push(M5ToolchainPinSwitchReviewPrimitiveViolation::MandatoryPinRowPartMissing);
        }
        if !row.declares_mandatory_inspector_parts() {
            violations
                .push(M5ToolchainPinSwitchReviewPrimitiveViolation::MandatoryInspectorPartMissing);
        }
        if !row.declares_mandatory_switch_card_parts() {
            violations
                .push(M5ToolchainPinSwitchReviewPrimitiveViolation::MandatorySwitchCardPartMissing);
        }
        if row.target_kinds.is_empty() {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::TargetKindMissing);
        }
        if row.pin_states.is_empty() {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::PinStateMissing);
        }
        if row.pin_scopes.is_empty() {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::PinScopeMissing);
        }
        if row.selection_health_states.is_empty() {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::SelectionHealthMissing);
        }
        if row.pin_actions.is_empty() {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::PinActionMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5ToolchainPinSwitchReviewPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5RuntimeBoundaryAccessibilityRoute::KeyboardFocusable)
        {
            violations
                .push(M5ToolchainPinSwitchReviewPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations
                .push(M5ToolchainPinSwitchReviewPrimitiveViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations
                .push(M5ToolchainPinSwitchReviewPrimitiveViolation::SelectorInvariantViolated);
        }
    }
}

/// At least one worked resolution across the matrix must prove an override (or higher
/// layer) shadowing a durable pin with the shadow disclosed — the acceptance-criterion
/// example that a workspace or policy override never silently shadows a user / global
/// pin.
fn validate_shadow_disclosure_covered(
    packet: &M5ToolchainPinSwitchReviewPrimitivePacket,
    violations: &mut Vec<M5ToolchainPinSwitchReviewPrimitiveViolation>,
) {
    let proven = packet.selector_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.shadows_durable_pin
                && case.resolved.discloses_shadowed_pins
                && case.resolved.ordered_layers.iter().any(|layer| {
                    !layer.is_winner && layer.is_durable_pin && layer.shadow_reason.is_some()
                })
        })
    });
    if !proven {
        violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::ShadowDisclosureUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a requested switch
/// resolving to an explicit blast radius and reversibility — the acceptance-criterion
/// example that a user can review the predicted blast radius before switching.
fn validate_switch_blast_radius_covered(
    packet: &M5ToolchainPinSwitchReviewPrimitivePacket,
    violations: &mut Vec<M5ToolchainPinSwitchReviewPrimitiveViolation>,
) {
    let proven = packet.selector_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.switch_review.is_some())
    });
    if !proven {
        violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::SwitchBlastRadiusUnproven);
    }
}

/// At least one worked resolution across the matrix must prove a degraded / mismatched
/// / missing selection keeping an explicit repair action — the acceptance-criterion
/// example that repair and revert stay explicit when the selection is degraded.
fn validate_degraded_repair_covered(
    packet: &M5ToolchainPinSwitchReviewPrimitivePacket,
    violations: &mut Vec<M5ToolchainPinSwitchReviewPrimitiveViolation>,
) {
    let proven = packet.selector_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.selection_is_degraded
                && case
                    .resolved
                    .available_actions
                    .contains(&M5PinAction::RepairSelection)
        })
    });
    if !proven {
        violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::DegradedRepairUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ToolchainPinSwitchReviewPrimitivePacket,
    violations: &mut Vec<M5ToolchainPinSwitchReviewPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_pin_precedence_and_switch,
        review.target_kind_selection_scope_and_source_always_shown,
        review.override_never_silently_shadows_durable_pin,
        review.winning_and_shadowed_layers_always_inspectable,
        review.predicted_blast_radius_always_shown_before_switch,
        review.degraded_selection_always_keeps_repair_action,
        review.support_export_reconstructs_pin_precedence_switch,
        review.no_surface_invents_second_selection_grammar,
        review.every_row_bound_to_shell_zone,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5ToolchainPinSwitchReviewPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ToolchainPinSwitchReviewPrimitivePacket,
    violations: &mut Vec<M5ToolchainPinSwitchReviewPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.environment_selectors_consume_shared_primitive,
        projection.pin_resolver_reads_single_precedence_source,
        projection.precedence_inspector_reads_single_layer_source,
        projection.switch_review_reads_single_switch_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations
                .push(M5ToolchainPinSwitchReviewPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ToolchainPinSwitchReviewPrimitivePacket,
    violations: &mut Vec<M5ToolchainPinSwitchReviewPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ToolchainPinSwitchReviewPrimitivePacket,
    violations: &mut Vec<M5ToolchainPinSwitchReviewPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.selection_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ToolchainPinSwitchReviewPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
