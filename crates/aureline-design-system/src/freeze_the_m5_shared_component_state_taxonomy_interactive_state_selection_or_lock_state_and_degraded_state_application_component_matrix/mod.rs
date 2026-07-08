//! Frozen M5 shared-component-state-taxonomy, interactive-state, selection-or-lock-state,
//! and degraded-state-application component matrix.
//!
//! This module locks Aureline's reusable component-state semantics into one export-safe
//! packet. Every launch-critical M5 surface — controls, dense collections, prompts and
//! dialogs, recovery surfaces, shell status/progress, and settings/capability sheets —
//! reads the same shared component-state taxonomy: `Default`, `Hover`, `Focus-visible`,
//! `Pressed/Active`, `Selected`, `Current`, `Disabled`, `Read-only`, `Loading`, `Pending`,
//! `Warning/Error`, `Locked`, and `Degraded`. Rather than leaving each feature family to
//! improvise those meanings, this matrix names the canonical state family once and freezes
//! the precedence and distinctness rules — locked-over-disabled, read-only-over-disabled,
//! current-vs-selected, and pending-vs-loading — that keep the states semantically distinct.
//!
//! What this matrix freezes is the stable vocabulary for the shared *state semantics*
//! themselves: the four component-state contract families (the shared taxonomy, the
//! interactive-state contract, the selection-or-lock-state contract, and the
//! degraded-state-application contract), the thirteen canonical state classes each family
//! governs, the precedence rules and disclosure triggers the taxonomy publishes, the
//! interaction input routes the interactive-state contract binds, the lock owners the
//! selection-or-lock contract discloses, the recovery-disclosure classes the
//! degraded-state contract names, the shared state-cause classes the selection-or-lock and
//! degraded contracts both bind, the deployment lines every contract must survive, the
//! non-visual accessibility routes, and the mandatory labels every contract must be able to
//! show. It does not re-implement the token registry, component contract, focus/selection,
//! or operational-state records that already own those details — it is the shared
//! state-semantic contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 control, collection,
//! prompt, or recovery surface may publish a component state. Every consumer reads this
//! packet so a disabled control never hides an explainable lock, a read-only control stays
//! inspectable, a current row and a selected row never collapse, a pending action never
//! masquerades as generic loading, and a degraded, warning, or error surface always names
//! its consequence and its recovery action. No M5 lane invents a private state name or a
//! parallel state grammar, and no state is encoded by color alone or reachable only by
//! hover.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5SharedComponentStateVocabularySet`] rather than minted per surface. Raw local paths,
//! credentials, and private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_shared_component_state_matrix,
    seeded_m5_shared_component_state_matrix_degraded_state_application_preview_narrowed,
    seeded_m5_shared_component_state_matrix_interactive_state_beta_narrowed,
    M5_SHARED_COMPONENT_STATE_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5SharedComponentStateMatrixPacket`].
pub const M5_SHARED_COMPONENT_STATE_MATRIX_RECORD_KIND: &str =
    "freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix";

/// Schema version for M5 shared-component-state-taxonomy component-matrix records.
pub const M5_SHARED_COMPONENT_STATE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the shared-component-state-taxonomy component boundary schema.
pub const M5_SHARED_COMPONENT_STATE_SCHEMA_REF: &str =
    "schemas/ui/m5-shared-component-state-taxonomy-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SHARED_COMPONENT_STATE_DOC_REF: &str =
    "docs/design-system/m5-shared-component-state-taxonomy-component-matrix.md";

/// Repo-relative path of the state-class contract the interactive-state and
/// selection-or-lock-state contracts bind against for stable state names.
pub const M5_SHARED_COMPONENT_STATE_STATE_CLASS_REF: &str = "schemas/state/state_class.schema.json";

/// Repo-relative path of the state-class-recovery contract the degraded-state-application
/// contract binds against.
pub const M5_SHARED_COMPONENT_STATE_RECOVERY_REF: &str =
    "schemas/state/state_class_recovery.schema.json";

/// Repo-relative path of the design-system component-contract the shared taxonomy binds
/// against.
pub const M5_SHARED_COMPONENT_STATE_COMPONENT_CONTRACT_REF: &str =
    "schemas/design-system/m5-component-contract.schema.json";

/// Repo-relative path of the focus/selection accessibility contract the interactive-state
/// and selection-or-lock-state contracts bind against.
pub const M5_SHARED_COMPONENT_STATE_FOCUS_SELECTION_REF: &str =
    "schemas/a11y/m5-focus-selection.schema.json";

/// Repo-relative path of the operational-surface-state contract the selection-or-lock and
/// degraded-state contracts bind against.
pub const M5_SHARED_COMPONENT_STATE_OPERATIONAL_STATE_REF: &str =
    "schemas/accessibility/operational_surface_state.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SHARED_COMPONENT_STATE_FIXTURE_DIR: &str = "fixtures/ui/m5-shared-state-taxonomy";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SHARED_COMPONENT_STATE_ARTIFACT_REF: &str =
    "artifacts/release/m5-shared-state-taxonomy-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SHARED_COMPONENT_STATE_CSV_REF: &str =
    "artifacts/release/m5-shared-state-taxonomy-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SHARED_COMPONENT_STATE_REPORT_REF: &str =
    "artifacts/design/m5-shared-state-taxonomy-component-matrix.md";

/// One of the four governed shared-component-state contract families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SharedComponentStateFamily {
    /// The shared component-state taxonomy: the full state family plus precedence and
    /// disclosure rules.
    SharedComponentStateTaxonomy,
    /// The interactive-state contract: default, hover, focus-visible, and pressed/active.
    InteractiveState,
    /// The selection-or-lock-state contract: selected, current, disabled, read-only, and
    /// locked.
    SelectionOrLockState,
    /// The degraded-state-application contract: loading, pending, warning/error, and
    /// degraded.
    DegradedStateApplication,
}

impl M5SharedComponentStateFamily {
    /// Every governed contract family, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SharedComponentStateTaxonomy,
        Self::InteractiveState,
        Self::SelectionOrLockState,
        Self::DegradedStateApplication,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SharedComponentStateTaxonomy => "shared_component_state_taxonomy",
            Self::InteractiveState => "interactive_state",
            Self::SelectionOrLockState => "selection_or_lock_state",
            Self::DegradedStateApplication => "degraded_state_application",
        }
    }

    /// `true` when this family is the shared taxonomy and must therefore declare the full
    /// state family, its precedence rules, and its disclosure triggers.
    pub const fn is_shared_component_state_taxonomy(self) -> bool {
        matches!(self, Self::SharedComponentStateTaxonomy)
    }

    /// `true` when this family is the interactive-state contract and must therefore declare
    /// its interaction input routes.
    pub const fn is_interactive_state(self) -> bool {
        matches!(self, Self::InteractiveState)
    }

    /// `true` when this family is the selection-or-lock-state contract and must therefore
    /// declare its lock owners.
    pub const fn is_selection_or_lock_state(self) -> bool {
        matches!(self, Self::SelectionOrLockState)
    }

    /// `true` when this family is the degraded-state-application contract and must therefore
    /// declare its recovery-disclosure classes.
    pub const fn is_degraded_state_application(self) -> bool {
        matches!(self, Self::DegradedStateApplication)
    }

    /// The exact subset of canonical state classes this contract family governs. The shared
    /// taxonomy governs all thirteen; the other three families partition the interactive,
    /// selection/lock, and degraded states between them.
    pub const fn governed_states(self) -> &'static [M5SharedComponentStateClass] {
        match self {
            Self::SharedComponentStateTaxonomy => &M5SharedComponentStateClass::ALL,
            Self::InteractiveState => &[
                M5SharedComponentStateClass::Default,
                M5SharedComponentStateClass::Hover,
                M5SharedComponentStateClass::FocusVisible,
                M5SharedComponentStateClass::PressedActive,
            ],
            Self::SelectionOrLockState => &[
                M5SharedComponentStateClass::Selected,
                M5SharedComponentStateClass::Current,
                M5SharedComponentStateClass::Disabled,
                M5SharedComponentStateClass::ReadOnly,
                M5SharedComponentStateClass::Locked,
            ],
            Self::DegradedStateApplication => &[
                M5SharedComponentStateClass::Loading,
                M5SharedComponentStateClass::Pending,
                M5SharedComponentStateClass::WarningError,
                M5SharedComponentStateClass::Degraded,
            ],
        }
    }
}

/// One of the thirteen canonical shared component-state classes. This is the single frozen
/// taxonomy every M5 surface maps its local state machine back to instead of minting a
/// private state name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SharedComponentStateClass {
    /// Default resting state.
    Default,
    /// Pointer hover.
    Hover,
    /// Focus rendered because the operator is likely using keyboard or assistive tech.
    FocusVisible,
    /// Pointer press or keyboard activation in flight.
    PressedActive,
    /// Durable selection across focus changes.
    Selected,
    /// Current route/location or live context owner.
    Current,
    /// Unavailable and non-actionable in the current context.
    Disabled,
    /// Inspectable but not editable or writable.
    ReadOnly,
    /// Background work in progress for this surface.
    Loading,
    /// User action submitted but not yet committed.
    Pending,
    /// Warning or error posture worth surfacing with its consequence.
    WarningError,
    /// Policy / trust / permission / ownership / source lock posture.
    Locked,
    /// Reduced capability remains; certainty or freshness is lowered.
    Degraded,
}

impl M5SharedComponentStateClass {
    /// Every canonical state class, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::Default,
        Self::Hover,
        Self::FocusVisible,
        Self::PressedActive,
        Self::Selected,
        Self::Current,
        Self::Disabled,
        Self::ReadOnly,
        Self::Loading,
        Self::Pending,
        Self::WarningError,
        Self::Locked,
        Self::Degraded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Hover => "hover",
            Self::FocusVisible => "focus_visible",
            Self::PressedActive => "pressed_active",
            Self::Selected => "selected",
            Self::Current => "current",
            Self::Disabled => "disabled",
            Self::ReadOnly => "read_only",
            Self::Loading => "loading",
            Self::Pending => "pending",
            Self::WarningError => "warning_error",
            Self::Locked => "locked",
            Self::Degraded => "degraded",
        }
    }
}

/// Controlled state precedence / distinctness rule — how the shared taxonomy resolves two
/// states that could apply at once, so states never silently collapse into one another.
/// These are the exact comparison rules the acceptance criteria call out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StatePrecedenceRule {
    /// A lock takes precedence over a plain disabled treatment so the lock stays
    /// explainable.
    LockedOverDisabled,
    /// A read-only posture takes precedence over a plain disabled treatment so
    /// inspectability is preserved.
    ReadOnlyOverDisabled,
    /// `current` and `selected` stay distinct and never collapse into one another.
    CurrentDistinctFromSelected,
    /// `pending` and `loading` stay distinct so a submitted action never reads as generic
    /// background work.
    PendingDistinctFromLoading,
}

impl M5StatePrecedenceRule {
    /// Every precedence rule, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LockedOverDisabled,
        Self::ReadOnlyOverDisabled,
        Self::CurrentDistinctFromSelected,
        Self::PendingDistinctFromLoading,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LockedOverDisabled => "locked_over_disabled",
            Self::ReadOnlyOverDisabled => "read_only_over_disabled",
            Self::CurrentDistinctFromSelected => "current_distinct_from_selected",
            Self::PendingDistinctFromLoading => "pending_distinct_from_loading",
        }
    }
}

/// Controlled disclosure trigger — the situations in which a state must publish its cause,
/// owner, block reason, or recovery action instead of applying a silent style-only change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateDisclosureTrigger {
    /// The state must name why it applies.
    StateCauseRequired,
    /// The state must name who owns or gates it.
    OwnerRequired,
    /// The state must name why an action is blocked.
    BlockReasonRequired,
    /// The state must name the recovery action.
    RecoveryActionRequired,
    /// A silent style-only state change is forbidden.
    SilentStyleOnlyForbidden,
}

impl M5StateDisclosureTrigger {
    /// Every disclosure trigger, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::StateCauseRequired,
        Self::OwnerRequired,
        Self::BlockReasonRequired,
        Self::RecoveryActionRequired,
        Self::SilentStyleOnlyForbidden,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateCauseRequired => "state_cause_required",
            Self::OwnerRequired => "owner_required",
            Self::BlockReasonRequired => "block_reason_required",
            Self::RecoveryActionRequired => "recovery_action_required",
            Self::SilentStyleOnlyForbidden => "silent_style_only_forbidden",
        }
    }
}

/// Controlled interaction input route — the non-visual routes an interactive state must be
/// reachable and announced through, so no interactive state is hover-only or pointer-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InteractionInputRoute {
    /// Pointer hover.
    PointerHover,
    /// Keyboard focus.
    KeyboardFocus,
    /// The focus-visible ring for keyboard / assistive-tech operators.
    FocusVisibleRing,
    /// Press or activation in flight.
    PressActivation,
    /// Announced to assistive technology.
    AssistiveTechAnnounced,
    /// Legible under reduced-motion.
    ReducedMotionSafe,
}

impl M5InteractionInputRoute {
    /// Every interaction input route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PointerHover,
        Self::KeyboardFocus,
        Self::FocusVisibleRing,
        Self::PressActivation,
        Self::AssistiveTechAnnounced,
        Self::ReducedMotionSafe,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PointerHover => "pointer_hover",
            Self::KeyboardFocus => "keyboard_focus",
            Self::FocusVisibleRing => "focus_visible_ring",
            Self::PressActivation => "press_activation",
            Self::AssistiveTechAnnounced => "assistive_tech_announced",
            Self::ReducedMotionSafe => "reduced_motion_safe",
        }
    }
}

/// Controlled lock owner class — who holds a lock behind a locked or disabled control, so a
/// disabled control never hides an explainable lock.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LockOwnerClass {
    /// A policy lock.
    PolicyLock,
    /// A trust-narrowing lock.
    TrustLock,
    /// A permission lock.
    PermissionLock,
    /// An ownership lock.
    OwnershipLock,
    /// A source-of-truth lock.
    SourceLock,
    /// No lock is in effect.
    NoLock,
}

impl M5LockOwnerClass {
    /// Every lock owner class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PolicyLock,
        Self::TrustLock,
        Self::PermissionLock,
        Self::OwnershipLock,
        Self::SourceLock,
        Self::NoLock,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyLock => "policy_lock",
            Self::TrustLock => "trust_lock",
            Self::PermissionLock => "permission_lock",
            Self::OwnershipLock => "ownership_lock",
            Self::SourceLock => "source_lock",
            Self::NoLock => "no_lock",
        }
    }
}

/// Controlled recovery-disclosure class — what a degraded, warning, or error state must
/// name, so such a state always names its consequence and its recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RecoveryDisclosureClass {
    /// Names the consequence of the degraded / warning / error state.
    NamesConsequence,
    /// Names the recovery action.
    NamesRecoveryAction,
    /// Names the freshness of what is shown.
    NamesFreshness,
    /// Names the retry path.
    NamesRetryPath,
    /// Names the fallback scope still available.
    NamesFallbackScope,
    /// No recovery action is available.
    NoRecoveryAvailable,
}

impl M5RecoveryDisclosureClass {
    /// Every recovery-disclosure class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NamesConsequence,
        Self::NamesRecoveryAction,
        Self::NamesFreshness,
        Self::NamesRetryPath,
        Self::NamesFallbackScope,
        Self::NoRecoveryAvailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamesConsequence => "names_consequence",
            Self::NamesRecoveryAction => "names_recovery_action",
            Self::NamesFreshness => "names_freshness",
            Self::NamesRetryPath => "names_retry_path",
            Self::NamesFallbackScope => "names_fallback_scope",
            Self::NoRecoveryAvailable => "no_recovery_available",
        }
    }
}

/// Controlled state-cause class — why a non-default state applies. Shared by the
/// selection-or-lock-state and degraded-state-application contracts, so a blocked, locked,
/// or degraded state always names a cause and never leaves it implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateCauseClass {
    /// A policy cause.
    PolicyCause,
    /// A permission cause.
    PermissionCause,
    /// An unmet-precondition cause.
    PreconditionCause,
    /// A connectivity cause.
    ConnectivityCause,
    /// A freshness cause.
    FreshnessCause,
    /// An unknown cause.
    UnknownCause,
}

impl M5StateCauseClass {
    /// Every state-cause class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PolicyCause,
        Self::PermissionCause,
        Self::PreconditionCause,
        Self::ConnectivityCause,
        Self::FreshnessCause,
        Self::UnknownCause,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyCause => "policy_cause",
            Self::PermissionCause => "permission_cause",
            Self::PreconditionCause => "precondition_cause",
            Self::ConnectivityCause => "connectivity_cause",
            Self::FreshnessCause => "freshness_cause",
            Self::UnknownCause => "unknown_cause",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a shared component state. No contract
/// may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComponentStateSurfaceFamily {
    /// Interactive controls.
    Controls,
    /// Dense collections (lists, trees, grids).
    DenseCollections,
    /// Prompts and dialogs.
    PromptsAndDialogs,
    /// Recovery surfaces.
    RecoverySurfaces,
    /// Shell status and progress surfaces.
    StatusAndProgress,
    /// Settings and capability sheets.
    SettingsAndCapability,
}

impl M5ComponentStateSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Controls,
        Self::DenseCollections,
        Self::PromptsAndDialogs,
        Self::RecoverySurfaces,
        Self::StatusAndProgress,
        Self::SettingsAndCapability,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Controls => "controls",
            Self::DenseCollections => "dense_collections",
            Self::PromptsAndDialogs => "prompts_and_dialogs",
            Self::RecoverySurfaces => "recovery_surfaces",
            Self::StatusAndProgress => "status_and_progress",
            Self::SettingsAndCapability => "settings_and_capability",
        }
    }
}

/// Deployment line a contract must survive with the same truth, so a state's meaning never
/// silently narrows or widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComponentStateDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5ComponentStateDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a contract's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComponentStateConsumerSurface {
    /// The design-system UI.
    DesignSystemUi,
    /// The shell UI.
    ShellUi,
    /// The command UI.
    CommandUi,
    /// The help UI.
    HelpUi,
    /// The settings UI.
    SettingsUi,
    /// The support export.
    SupportExport,
    /// The CLI / headless surface.
    CliHeadless,
    /// The general product UI.
    ProductUi,
}

impl M5ComponentStateConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DesignSystemUi,
        Self::ShellUi,
        Self::CommandUi,
        Self::HelpUi,
        Self::SettingsUi,
        Self::SupportExport,
        Self::CliHeadless,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesignSystemUi => "design_system_ui",
            Self::ShellUi => "shell_ui",
            Self::CommandUi => "command_ui",
            Self::HelpUi => "help_ui",
            Self::SettingsUi => "settings_ui",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every contract must offer so no state truth is
/// hover-only, pointer-only, or color-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComponentStateAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Encoded by more than color alone.
    NonColorEncoded,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5ComponentStateAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonColorEncoded,
        Self::NonHoverReachable,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonColorEncoded => "non_color_encoded",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed shared-component-state contract must be able to show. The first
/// three are hard requirements on every contract; the remaining three close the
/// acceptance-criteria ambiguity about state cause, owner / block reason, and recovery
/// action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComponentStateRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the state.
    KeyboardRoute,
    /// Why the state applies.
    StateCause,
    /// Who owns the state, or why an action is blocked.
    OwnerOrBlockReason,
    /// The recovery action out of the state.
    RecoveryAction,
}

impl M5ComponentStateRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::StateCause,
        Self::OwnerOrBlockReason,
        Self::RecoveryAction,
    ];

    /// The three labels every claimed contract must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::StateCause => "state_cause",
            Self::OwnerOrBlockReason => "owner_or_block_reason",
            Self::RecoveryAction => "recovery_action",
        }
    }
}

/// Qualification class for an M5 shared-component-state contract row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComponentStateQualificationClass {
    /// Contract qualifies for the Stable claim.
    Stable,
    /// Contract is narrowed to Beta.
    Beta,
    /// Contract is narrowed to Preview.
    Preview,
    /// Contract is experimental and not claimed.
    Experimental,
    /// Contract is unavailable on this build.
    Unavailable,
    /// Contract is held pending upstream resolution.
    Held,
}

impl M5ComponentStateQualificationClass {
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

    /// Whether the contract may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a shared-component-state contract below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComponentStateDowngradeTrigger {
    /// The taxonomy left a precedence rule unstated.
    PrecedenceRuleUnstated,
    /// A state left its cause unstated.
    StateCauseUnstated,
    /// A disabled control masked its lock owner.
    LockOwnerMasked,
    /// `current` and `selected` were collapsed into one another.
    CurrentSelectedCollapsed,
    /// A pending action was shown as generic loading.
    PendingShownAsLoading,
    /// A degraded / warning / error state omitted its consequence or recovery.
    ConsequenceOrRecoveryOmitted,
    /// A state was encoded by color alone.
    ColorOnlyTreatment,
    /// A state hid its non-visual keyboard route.
    KeyboardRouteMissing,
    /// A read-only control lost its inspectability.
    ReadOnlyInspectabilityLost,
    /// A required disclosure was not published.
    DisclosureRequirementUnmet,
    /// A surface invented an alternate / private label for a governed state.
    AlternateStateLabelInvented,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5ComponentStateDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::PrecedenceRuleUnstated,
        Self::StateCauseUnstated,
        Self::LockOwnerMasked,
        Self::CurrentSelectedCollapsed,
        Self::PendingShownAsLoading,
        Self::ConsequenceOrRecoveryOmitted,
        Self::ColorOnlyTreatment,
        Self::KeyboardRouteMissing,
        Self::ReadOnlyInspectabilityLost,
        Self::DisclosureRequirementUnmet,
        Self::AlternateStateLabelInvented,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrecedenceRuleUnstated => "precedence_rule_unstated",
            Self::StateCauseUnstated => "state_cause_unstated",
            Self::LockOwnerMasked => "lock_owner_masked",
            Self::CurrentSelectedCollapsed => "current_selected_collapsed",
            Self::PendingShownAsLoading => "pending_shown_as_loading",
            Self::ConsequenceOrRecoveryOmitted => "consequence_or_recovery_omitted",
            Self::ColorOnlyTreatment => "color_only_treatment",
            Self::KeyboardRouteMissing => "keyboard_route_missing",
            Self::ReadOnlyInspectabilityLost => "read_only_inspectability_lost",
            Self::DisclosureRequirementUnmet => "disclosure_requirement_unmet",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed shared-component-state contract family bound to the
/// state semantics it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SharedComponentStateRow {
    /// Governed contract family.
    pub component_family: M5SharedComponentStateFamily,
    /// Qualification class earned by this contract.
    pub qualification: M5ComponentStateQualificationClass,
    /// Owner role accountable for keeping this contract governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this contract.
    pub surface_families: Vec<M5ComponentStateSurfaceFamily>,
    /// Deployment lines this contract keeps the same truth across.
    pub deployment_lines: Vec<M5ComponentStateDeploymentLine>,
    /// Mandatory labels this contract must be able to show (must include the three
    /// [`M5ComponentStateRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5ComponentStateRequiredLabel>,
    /// Canonical state classes this contract governs (must equal the family's
    /// [`governed_states`](M5SharedComponentStateFamily::governed_states)).
    pub state_classes: Vec<M5SharedComponentStateClass>,
    /// Precedence rules this contract freezes (shared-component-state-taxonomy only).
    pub precedence_rules: Vec<M5StatePrecedenceRule>,
    /// Disclosure triggers this contract publishes (shared-component-state-taxonomy only).
    pub disclosure_triggers: Vec<M5StateDisclosureTrigger>,
    /// Interaction input routes this contract binds (interactive-state only).
    pub interaction_input_routes: Vec<M5InteractionInputRoute>,
    /// Lock owners this contract discloses (selection-or-lock-state only).
    pub lock_owner_classes: Vec<M5LockOwnerClass>,
    /// Recovery-disclosure classes this contract names (degraded-state-application only).
    pub recovery_disclosure_classes: Vec<M5RecoveryDisclosureClass>,
    /// State-cause classes this contract names (selection-or-lock-state and
    /// degraded-state-application).
    pub state_cause_classes: Vec<M5StateCauseClass>,
    /// Non-visual accessibility routes this contract offers.
    pub accessibility_routes: Vec<M5ComponentStateAccessibilityRoute>,
    /// Subsystems that consume this contract's projection.
    pub consumer_surfaces: Vec<M5ComponentStateConsumerSurface>,
    /// Downgrade triggers that apply to this contract.
    pub downgrade_triggers: Vec<M5ComponentStateDowngradeTrigger>,
    /// Proof packet refs that keep this contract current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this contract.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this contract never collapses `current` and `selected`. MUST be
    /// `false`.
    pub collapses_current_and_selected: bool,
    /// Hard invariant: this contract never masks an explainable lock behind a plain disabled
    /// treatment. MUST be `false`.
    pub masks_lock_behind_disabled: bool,
    /// Hard invariant: this contract never presents `pending` as generic `loading`. MUST be
    /// `false`.
    pub presents_pending_as_generic_loading: bool,
    /// Hard invariant: this contract never omits consequence or recovery on a degraded /
    /// warning / error state. MUST be `false`.
    pub omits_consequence_or_recovery_on_degraded: bool,
}

impl M5SharedComponentStateRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ComponentStateRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ComponentStateRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row governs exactly the state classes its family owns.
    fn governs_expected_states(&self) -> bool {
        let present: BTreeSet<M5SharedComponentStateClass> =
            self.state_classes.iter().copied().collect();
        let expected: BTreeSet<M5SharedComponentStateClass> = self
            .component_family
            .governed_states()
            .iter()
            .copied()
            .collect();
        present == expected
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_current_and_selected
            && !self.masks_lock_behind_disabled
            && !self.presents_pending_as_generic_loading
            && !self.omits_consequence_or_recovery_on_degraded
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SharedComponentStateVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Canonical state-class tokens.
    pub state_classes: Vec<String>,
    /// Precedence-rule tokens.
    pub precedence_rules: Vec<String>,
    /// Disclosure-trigger tokens.
    pub disclosure_triggers: Vec<String>,
    /// Interaction-input-route tokens.
    pub interaction_input_routes: Vec<String>,
    /// Lock-owner-class tokens.
    pub lock_owner_classes: Vec<String>,
    /// Recovery-disclosure-class tokens.
    pub recovery_disclosure_classes: Vec<String>,
    /// State-cause-class tokens.
    pub state_cause_classes: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5SharedComponentStateVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5SharedComponentStateFamily::ALL, |v| v.as_str()),
            state_classes: tokens(&M5SharedComponentStateClass::ALL, |v| v.as_str()),
            precedence_rules: tokens(&M5StatePrecedenceRule::ALL, |v| v.as_str()),
            disclosure_triggers: tokens(&M5StateDisclosureTrigger::ALL, |v| v.as_str()),
            interaction_input_routes: tokens(&M5InteractionInputRoute::ALL, |v| v.as_str()),
            lock_owner_classes: tokens(&M5LockOwnerClass::ALL, |v| v.as_str()),
            recovery_disclosure_classes: tokens(&M5RecoveryDisclosureClass::ALL, |v| v.as_str()),
            state_cause_classes: tokens(&M5StateCauseClass::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ComponentStateSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ComponentStateDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ComponentStateConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ComponentStateAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ComponentStateRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5SharedComponentStateGovernanceReview {
    /// The shared taxonomy names all thirteen canonical states once.
    pub taxonomy_names_all_thirteen_states: bool,
    /// The precedence rules (locked-over-disabled, read-only-over-disabled,
    /// current-vs-selected, pending-vs-loading) are named once.
    pub precedence_rules_named_once: bool,
    /// A disabled control never hides an explainable lock.
    pub disabled_never_hides_explainable_lock: bool,
    /// A read-only control preserves inspectability.
    pub read_only_preserves_inspectability: bool,
    /// `current` and `selected` never collapse.
    pub current_and_selected_never_collapse: bool,
    /// `pending` never masquerades as generic `loading`.
    pub pending_never_shown_as_generic_loading: bool,
    /// Degraded / warning / error states always name consequence and recovery.
    pub degraded_warning_error_names_consequence_and_recovery: bool,
    /// A state's cause, owner, or block reason is always disclosed when required.
    pub state_cause_owner_or_block_reason_always_disclosed: bool,
    /// No state is encoded by color alone.
    pub no_state_is_color_only: bool,
    /// Every state is keyboard-visible.
    pub every_state_is_keyboard_visible: bool,
    /// Every state is screen-reader explainable.
    pub every_state_is_screen_reader_explainable: bool,
    /// Every contract keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every contract declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// No surface invents a private state name.
    pub no_surface_invents_private_state_name: bool,
    /// Later M5 rows cannot invent parallel state vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SharedComponentStateConsumerProjection {
    /// Controls consume the interactive-state vocabulary.
    pub controls_consume_interactive_state_vocabulary: bool,
    /// Collections consume the selection-or-lock vocabulary.
    pub collections_consume_selection_lock_vocabulary: bool,
    /// Prompts consume the state-cause vocabulary.
    pub prompts_consume_state_cause_vocabulary: bool,
    /// Recovery surfaces consume the degraded-state vocabulary.
    pub recovery_surfaces_consume_degraded_vocabulary: bool,
    /// Shell status / progress surfaces consume the shared taxonomy.
    pub shell_status_progress_consume_shared_taxonomy: bool,
    /// Support / export reads a single canonical state source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SharedComponentStateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the contract.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the shared-component-state lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SharedComponentStateReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting state-taxonomy audit for the lane.
    pub state_taxonomy_audit_ref: String,
    /// True when support/export parity is required for every contract.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every contract.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SharedComponentStateMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SharedComponentStateMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Contract rows.
    pub component_rows: Vec<M5SharedComponentStateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SharedComponentStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SharedComponentStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SharedComponentStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SharedComponentStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SharedComponentStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 shared-component-state matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SharedComponentStateMatrixPacket {
    /// Record kind; must equal [`M5_SHARED_COMPONENT_STATE_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SHARED_COMPONENT_STATE_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Contract rows.
    pub component_rows: Vec<M5SharedComponentStateRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SharedComponentStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SharedComponentStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SharedComponentStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SharedComponentStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SharedComponentStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SharedComponentStateMatrixPacket {
    /// Builds an M5 shared-component-state matrix packet from stable-lane input.
    pub fn new(input: M5SharedComponentStateMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_SHARED_COMPONENT_STATE_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_SHARED_COMPONENT_STATE_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 shared-component-state matrix invariants.
    pub fn validate(&self) -> Vec<M5SharedComponentStateMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SHARED_COMPONENT_STATE_MATRIX_RECORD_KIND {
            violations.push(M5SharedComponentStateMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SHARED_COMPONENT_STATE_MATRIX_SCHEMA_VERSION {
            violations.push(M5SharedComponentStateMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SharedComponentStateMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 shared-component-state matrix packet serializes"),
        ) {
            violations.push(M5SharedComponentStateMatrixViolation::RawMaterialInExport);
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
            .expect("m5 shared-component-state matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed contract.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,state_classes,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.state_classes, |v| v.as_str()),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Shared-Component-State-Taxonomy, Interactive-State, Selection-or-Lock-State, and Degraded-State-Application Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Contract families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Canonical state classes: {}\n",
            self.vocabulary_set.state_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Precedence rules: {}\n",
            self.vocabulary_set.precedence_rules.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Contract families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - State classes: {}\n",
                row.state_classes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 shared-component-state matrix export.
#[derive(Debug)]
pub enum M5SharedComponentStateMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SharedComponentStateMatrixViolation>),
}

impl fmt::Display for M5SharedComponentStateMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 shared-component-state matrix export parse failed: {error}"
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
                    "m5 shared-component-state matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SharedComponentStateMatrixArtifactError {}

/// Validation failures emitted by [`M5SharedComponentStateMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SharedComponentStateMatrixViolation {
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
    /// A required governed contract family is missing from the matrix.
    RequiredComponentMissing,
    /// A contract row is incomplete.
    ComponentRowIncomplete,
    /// A contract row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A contract row does not govern exactly the state classes its family owns.
    StateSubsetMismatch,
    /// The shared-taxonomy contract declares no precedence rules.
    PrecedenceRuleMissing,
    /// The shared-taxonomy contract declares no disclosure triggers.
    DisclosureTriggerMissing,
    /// The interactive-state contract declares no interaction input routes.
    InteractionInputRouteMissing,
    /// The selection-or-lock-state contract declares no lock owners.
    LockOwnerClassMissing,
    /// The degraded-state-application contract declares no recovery-disclosure classes.
    RecoveryDisclosureClassMissing,
    /// A selection-or-lock or degraded contract declares no state-cause classes.
    StateCauseClassMissing,
    /// A contract declares no surface families.
    SurfaceFamilyMissing,
    /// A contract declares no deployment lines.
    DeploymentLineMissing,
    /// A contract declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A contract declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A contract declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A contract claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A contract violates a hard invariant (collapsed current/selected, masked lock behind
    /// disabled, pending shown as loading, or omitted consequence/recovery on degraded).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SharedComponentStateMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::StateSubsetMismatch => "state_subset_mismatch",
            Self::PrecedenceRuleMissing => "precedence_rule_missing",
            Self::DisclosureTriggerMissing => "disclosure_trigger_missing",
            Self::InteractionInputRouteMissing => "interaction_input_route_missing",
            Self::LockOwnerClassMissing => "lock_owner_class_missing",
            Self::RecoveryDisclosureClassMissing => "recovery_disclosure_class_missing",
            Self::StateCauseClassMissing => "state_cause_class_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 shared-component-state matrix export.
pub fn current_stable_m5_shared_component_state_matrix_export(
) -> Result<M5SharedComponentStateMatrixPacket, M5SharedComponentStateMatrixArtifactError> {
    let packet: M5SharedComponentStateMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shared-state-taxonomy-proof/support_export.json"
    )))
    .map_err(M5SharedComponentStateMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SharedComponentStateMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5SharedComponentStateMatrixPacket,
    violations: &mut Vec<M5SharedComponentStateMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SHARED_COMPONENT_STATE_SCHEMA_REF,
        M5_SHARED_COMPONENT_STATE_DOC_REF,
        M5_SHARED_COMPONENT_STATE_STATE_CLASS_REF,
        M5_SHARED_COMPONENT_STATE_RECOVERY_REF,
        M5_SHARED_COMPONENT_STATE_COMPONENT_CONTRACT_REF,
        M5_SHARED_COMPONENT_STATE_FOCUS_SELECTION_REF,
        M5_SHARED_COMPONENT_STATE_OPERATIONAL_STATE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SharedComponentStateMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SharedComponentStateMatrixPacket,
    violations: &mut Vec<M5SharedComponentStateMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5SharedComponentStateMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5SharedComponentStateMatrixPacket,
    violations: &mut Vec<M5SharedComponentStateMatrixViolation>,
) {
    let present: BTreeSet<M5SharedComponentStateFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5SharedComponentStateFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5SharedComponentStateMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5SharedComponentStateMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5SharedComponentStateMatrixViolation::MandatoryLabelMissing);
        }
        if !row.governs_expected_states() {
            violations.push(M5SharedComponentStateMatrixViolation::StateSubsetMismatch);
        }
        if family.is_shared_component_state_taxonomy() && row.precedence_rules.is_empty() {
            violations.push(M5SharedComponentStateMatrixViolation::PrecedenceRuleMissing);
        }
        if family.is_shared_component_state_taxonomy() && row.disclosure_triggers.is_empty() {
            violations.push(M5SharedComponentStateMatrixViolation::DisclosureTriggerMissing);
        }
        if family.is_interactive_state() && row.interaction_input_routes.is_empty() {
            violations.push(M5SharedComponentStateMatrixViolation::InteractionInputRouteMissing);
        }
        if family.is_selection_or_lock_state() && row.lock_owner_classes.is_empty() {
            violations.push(M5SharedComponentStateMatrixViolation::LockOwnerClassMissing);
        }
        if family.is_degraded_state_application() && row.recovery_disclosure_classes.is_empty() {
            violations.push(M5SharedComponentStateMatrixViolation::RecoveryDisclosureClassMissing);
        }
        // State-cause is shared by the selection-or-lock-state and degraded-state-application
        // contracts.
        if (family.is_selection_or_lock_state() || family.is_degraded_state_application())
            && row.state_cause_classes.is_empty()
        {
            violations.push(M5SharedComponentStateMatrixViolation::StateCauseClassMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5SharedComponentStateMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5SharedComponentStateMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5SharedComponentStateMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5SharedComponentStateMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5SharedComponentStateMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5SharedComponentStateMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5SharedComponentStateMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5SharedComponentStateMatrixPacket,
    violations: &mut Vec<M5SharedComponentStateMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.taxonomy_names_all_thirteen_states,
        review.precedence_rules_named_once,
        review.disabled_never_hides_explainable_lock,
        review.read_only_preserves_inspectability,
        review.current_and_selected_never_collapse,
        review.pending_never_shown_as_generic_loading,
        review.degraded_warning_error_names_consequence_and_recovery,
        review.state_cause_owner_or_block_reason_always_disclosed,
        review.no_state_is_color_only,
        review.every_state_is_keyboard_visible,
        review.every_state_is_screen_reader_explainable,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.no_surface_invents_private_state_name,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5SharedComponentStateMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SharedComponentStateMatrixPacket,
    violations: &mut Vec<M5SharedComponentStateMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.controls_consume_interactive_state_vocabulary,
        projection.collections_consume_selection_lock_vocabulary,
        projection.prompts_consume_state_cause_vocabulary,
        projection.recovery_surfaces_consume_degraded_vocabulary,
        projection.shell_status_progress_consume_shared_taxonomy,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5SharedComponentStateMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SharedComponentStateMatrixPacket,
    violations: &mut Vec<M5SharedComponentStateMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SharedComponentStateMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SharedComponentStateMatrixPacket,
    violations: &mut Vec<M5SharedComponentStateMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.state_taxonomy_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SharedComponentStateMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
