//! Frozen M5 accessibility-bridge, live-announcement, focus-return, and
//! non-visual dynamic-surface matrix.
//!
//! This module locks the canonical M5 assistive-technology object model into one
//! export-safe packet. Each [`M5DynamicSurfaceA11yObjectRow`] names one governed
//! accessibility object — the accessibility-surface descriptor, the screen-reader
//! label model, the live-announcement class, the focus-return contract, the
//! dense-surface non-visual summary, and the bridge-diagnostics packet — and binds
//! it to its qualification class, required fields, the controlled state
//! vocabularies it carries, the concrete vocabulary tokens it admits, evidence
//! requirements, the proof packet that keeps it current, downgrade triggers,
//! rollback posture, source contracts, and consumer-surface parity.
//!
//! The matrix is the single M5 source of truth for whether claimed custom-rendered
//! dynamic surfaces may publish screen-reader-complete or keyboard-complete claims.
//! Shell, editor, terminal, notebook, data, review, help, and presentation surfaces
//! consume this packet rather than maintaining per-surface ad hoc assistive
//! behavior: custom-rendered surfaces expose semantic structure and durable
//! fallbacks; focus never teleports or vanishes on async updates; live regions
//! coalesce rather than spam; dynamic state changes announce meaning, not repaint
//! noise; and claimed screen-reader/keyboard-complete rows auto-narrow when bridge
//! or proof state goes stale.
//!
//! The controlled vocabularies mirror the canonical tokens already owned by the
//! screen-reader/live-region contract, the accessibility-tree contract, the
//! focus/zoom/pointer-independence contract, the collection-announcement contract,
//! and the operational-surface parity contract; the matrix freezes them in one
//! self-describing [`M5DynamicSurfaceA11yVocabularySet`] rather than minting
//! parallel tokens. It references those upstream accessibility contracts by path.
//! Raw provider payloads, credentials, secret material, screenshots, and
//! untranslated free-text prose stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/a11y/m5-dynamic-surface-a11y.schema.json`](../../../../schemas/a11y/m5-dynamic-surface-a11y.schema.json).
//! The contract doc is
//! [`docs/a11y/m5-dynamic-surface-a11y.md`](../../../../docs/a11y/m5-dynamic-surface-a11y.md).
//! The protected fixture directory is
//! [`fixtures/a11y/m5-dynamic-surfaces/`](../../../../fixtures/a11y/m5-dynamic-surfaces/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_dynamic_surface_a11y_matrix,
    seeded_m5_dynamic_surface_a11y_matrix_bridge_unavailable,
    seeded_m5_dynamic_surface_a11y_matrix_dense_summary_narrowed, M5_DYNAMIC_A11Y_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DynamicSurfaceA11yMatrixPacket`].
pub const M5_DYNAMIC_A11Y_MATRIX_RECORD_KIND: &str =
    "freeze_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix";

/// Schema version for M5 dynamic-surface accessibility matrix records.
pub const M5_DYNAMIC_A11Y_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_DYNAMIC_A11Y_MATRIX_SCHEMA_REF: &str =
    "schemas/a11y/m5-dynamic-surface-a11y.schema.json";

/// Repo-relative path of the M5 dynamic-surface accessibility contract doc.
pub const M5_DYNAMIC_A11Y_MATRIX_DOC_REF: &str = "docs/a11y/m5-dynamic-surface-a11y.md";

/// Repo-relative path of the frozen screen-reader announcement / live-region
/// contract.
pub const M5_DYNAMIC_A11Y_SCREEN_READER_CONTRACT_REF: &str =
    "docs/accessibility/screen_reader_and_live_region_contract.md";

/// Repo-relative path of the frozen accessibility-tree node taxonomy contract.
pub const M5_DYNAMIC_A11Y_TREE_CONTRACT_REF: &str =
    "docs/accessibility/accessibility_tree_contract.md";

/// Repo-relative path of the frozen focus / zoom / pointer-independence contract.
pub const M5_DYNAMIC_A11Y_FOCUS_CONTRACT_REF: &str =
    "docs/accessibility/focus_zoom_and_pointer_independence_contract.md";

/// Repo-relative path of the frozen dense-collection announcement contract.
pub const M5_DYNAMIC_A11Y_COLLECTION_CONTRACT_REF: &str =
    "docs/accessibility/collection_announcement_contract.md";

/// Repo-relative path of the frozen shell accessibility-bridge groundwork.
pub const M5_DYNAMIC_A11Y_SHELL_BRIDGE_CONTRACT_REF: &str = "docs/accessibility/m1_shell_bridge.md";

/// Repo-relative path of the frozen operational-surface parity contract.
pub const M5_DYNAMIC_A11Y_OPERATIONAL_PARITY_CONTRACT_REF: &str =
    "docs/accessibility/operational_surface_parity_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_DYNAMIC_A11Y_MATRIX_FIXTURE_DIR: &str = "fixtures/a11y/m5-dynamic-surfaces";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DYNAMIC_A11Y_MATRIX_ARTIFACT_REF: &str =
    "artifacts/a11y/m5-dynamic-surfaces/support_export.json";

/// Repo-relative path of the checked Markdown governance summary.
pub const M5_DYNAMIC_A11Y_MATRIX_SUMMARY_REF: &str = "artifacts/a11y/m5-dynamic-a11y-governance.md";

/// One of the six governed M5 dynamic-surface accessibility objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DynamicSurfaceA11yObjectKind {
    /// Semantic-structure descriptor for one custom-rendered dynamic surface.
    AccessibilitySurfaceDescriptor,
    /// Screen-reader name / role / value / state label model for a surface.
    ScreenReaderLabelModel,
    /// Live-announcement class governing politeness, coalescing, and fallback.
    LiveAnnouncementClass,
    /// Focus-return contract for asynchronous updates and overlay teardown.
    FocusReturnContract,
    /// Dense-surface non-visual summary for lists, trees, grids, and logs.
    DenseSurfaceNonVisualSummary,
    /// OS accessibility-bridge diagnostics packet.
    BridgeDiagnosticsPacket,
}

impl M5DynamicSurfaceA11yObjectKind {
    /// Every governed object, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AccessibilitySurfaceDescriptor,
        Self::ScreenReaderLabelModel,
        Self::LiveAnnouncementClass,
        Self::FocusReturnContract,
        Self::DenseSurfaceNonVisualSummary,
        Self::BridgeDiagnosticsPacket,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccessibilitySurfaceDescriptor => "accessibility_surface_descriptor",
            Self::ScreenReaderLabelModel => "screen_reader_label_model",
            Self::LiveAnnouncementClass => "live_announcement_class",
            Self::FocusReturnContract => "focus_return_contract",
            Self::DenseSurfaceNonVisualSummary => "dense_surface_non_visual_summary",
            Self::BridgeDiagnosticsPacket => "bridge_diagnostics_packet",
        }
    }

    /// Controlled state vocabularies this object kind MUST declare.
    pub fn required_state_vocabularies(self) -> &'static [M5DynamicSurfaceA11yStateVocabulary] {
        use M5DynamicSurfaceA11yStateVocabulary as V;
        match self {
            Self::AccessibilitySurfaceDescriptor => {
                &[V::SemanticRoleClass, V::NonVisualFidelity, V::BridgeState]
            }
            Self::ScreenReaderLabelModel => {
                &[V::SemanticRoleClass, V::FallbackDurability, V::BridgeState]
            }
            Self::LiveAnnouncementClass => &[
                V::AnnouncementPoliteness,
                V::CoalescingStrategy,
                V::FallbackDurability,
            ],
            Self::FocusReturnContract => &[V::FocusReturnDisposition, V::FallbackDurability],
            Self::DenseSurfaceNonVisualSummary => &[
                V::NonVisualFidelity,
                V::CoalescingStrategy,
                V::SemanticRoleClass,
            ],
            Self::BridgeDiagnosticsPacket => &[V::BridgeState, V::NonVisualFidelity],
        }
    }
}

/// Qualification class for an M5 dynamic-surface accessibility object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DynamicSurfaceA11yQualificationClass {
    /// Object qualifies for the Stable (screen-reader/keyboard-complete) claim.
    Stable,
    /// Object is narrowed to Beta.
    Beta,
    /// Object is narrowed to Preview.
    Preview,
    /// Object is experimental and not claimed.
    Experimental,
    /// Object is unavailable on this build.
    Unavailable,
    /// Object is held pending bridge or upstream resolution.
    Held,
}

impl M5DynamicSurfaceA11yQualificationClass {
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

    /// Whether the object may carry a public Stable assistive-tech claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Names one of the controlled state vocabularies an accessibility object carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DynamicSurfaceA11yStateVocabulary {
    /// Live-region politeness class (polite / assertive / silent).
    AnnouncementPoliteness,
    /// Live-region coalescing strategy.
    CoalescingStrategy,
    /// Announcement delivery / fallback durability.
    FallbackDurability,
    /// Non-visual fidelity of a surface or summary.
    NonVisualFidelity,
    /// OS accessibility-bridge connection state.
    BridgeState,
    /// Focus-return disposition after an async update or overlay teardown.
    FocusReturnDisposition,
    /// Semantic role class exposed by a surface descriptor or label model.
    SemanticRoleClass,
}

impl M5DynamicSurfaceA11yStateVocabulary {
    /// Every vocabulary, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::AnnouncementPoliteness,
        Self::CoalescingStrategy,
        Self::FallbackDurability,
        Self::NonVisualFidelity,
        Self::BridgeState,
        Self::FocusReturnDisposition,
        Self::SemanticRoleClass,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AnnouncementPoliteness => "announcement_politeness",
            Self::CoalescingStrategy => "coalescing_strategy",
            Self::FallbackDurability => "fallback_durability",
            Self::NonVisualFidelity => "non_visual_fidelity",
            Self::BridgeState => "bridge_state",
            Self::FocusReturnDisposition => "focus_return_disposition",
            Self::SemanticRoleClass => "semantic_role_class",
        }
    }
}

/// Controlled live-region politeness class for an announcement.
///
/// Mirrors the canonical `live_region_channel` tokens owned by the screen-reader /
/// live-region contract so a surface never mints a parallel urgency synonym.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum A11yAnnouncementPoliteness {
    /// Polite live region; queued behind the current utterance.
    Polite,
    /// Assertive live region; interrupts for safety-critical state.
    Assertive,
    /// No spoken announcement; the focused surface already conveys the state.
    Silent,
}

impl A11yAnnouncementPoliteness {
    /// Every politeness class, in declaration order.
    pub const ALL: [Self; 3] = [Self::Polite, Self::Assertive, Self::Silent];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Polite => "polite",
            Self::Assertive => "assertive",
            Self::Silent => "silent",
        }
    }
}

/// Controlled coalescing strategy for a live region so it never spams.
///
/// Mirrors the canonical `coalescing_strategy` tokens owned by the screen-reader /
/// live-region contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum A11yCoalescingStrategy {
    /// No coalescing; every event speaks once.
    None,
    /// Drop events whose meaning is unchanged.
    DedupeSameMeaning,
    /// Keep the last meaning and append a running count.
    LastMeaningWinsWithCount,
    /// Announce only the start and terminal state of a burst.
    StartAndTerminalOnly,
    /// Announce only on the focused surface.
    FocusedSurfaceOnly,
}

impl A11yCoalescingStrategy {
    /// Every coalescing strategy, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::None,
        Self::DedupeSameMeaning,
        Self::LastMeaningWinsWithCount,
        Self::StartAndTerminalOnly,
        Self::FocusedSurfaceOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DedupeSameMeaning => "dedupe_same_meaning",
            Self::LastMeaningWinsWithCount => "last_meaning_wins_with_count",
            Self::StartAndTerminalOnly => "start_and_terminal_only",
            Self::FocusedSurfaceOnly => "focused_surface_only",
        }
    }
}

/// Controlled delivery / fallback-durability class for an announcement.
///
/// Mirrors the canonical `delivery_timing` tokens owned by the screen-reader /
/// live-region contract. `durable_surface_only` is the durable fallback; a blocking
/// state is delivered `immediate`, and `not_delivered_silent` is a deliberate,
/// disclosed non-delivery — never a dropped meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum A11yFallbackDurability {
    /// Delivered immediately (blocking / safety-critical state).
    Immediate,
    /// Delivered after coalescing a burst.
    Coalesced,
    /// Delivered when the surface receives focus.
    OnFocus,
    /// Persisted to a durable fallback surface, not just a transient live region.
    DurableSurfaceOnly,
    /// Deliberately not spoken, with a disclosed silence reason.
    NotDeliveredSilent,
}

impl A11yFallbackDurability {
    /// Every durability class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Immediate,
        Self::Coalesced,
        Self::OnFocus,
        Self::DurableSurfaceOnly,
        Self::NotDeliveredSilent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Immediate => "immediate",
            Self::Coalesced => "coalesced",
            Self::OnFocus => "on_focus",
            Self::DurableSurfaceOnly => "durable_surface_only",
            Self::NotDeliveredSilent => "not_delivered_silent",
        }
    }
}

/// Controlled non-visual fidelity class for a surface or summary.
///
/// Mirrors the canonical `support_state` tokens owned by the accessibility-tree
/// contract so a surface never overstates its non-visual coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum A11yNonVisualFidelity {
    /// Full non-visual parity with the visual surface.
    FullAccessible,
    /// Non-visual access with a disclosed degradation.
    DegradedAccessible,
    /// Only a generated summary is exposed.
    SummaryOnly,
    /// Inspect-only; no non-visual mutation path.
    InspectOnly,
    /// No non-visual access; blocked and disclosed.
    UnsupportedBlocked,
    /// Not applicable for this object's current qualification.
    NotApplicable,
}

impl A11yNonVisualFidelity {
    /// Every fidelity class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullAccessible,
        Self::DegradedAccessible,
        Self::SummaryOnly,
        Self::InspectOnly,
        Self::UnsupportedBlocked,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullAccessible => "full_accessible",
            Self::DegradedAccessible => "degraded_accessible",
            Self::SummaryOnly => "summary_only",
            Self::InspectOnly => "inspect_only",
            Self::UnsupportedBlocked => "unsupported_blocked",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Controlled OS accessibility-bridge connection state.
///
/// `bridged_active` is the proven, connected bridge. `partial`, `stale`, and
/// `unavailable` are the disclosed narrowing states the spec requires; a claimed
/// surface auto-narrows when its bridge falls to one of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum A11yBridgeState {
    /// Bridge is connected and proven current.
    BridgedActive,
    /// Bridge is connected but only partially mapped.
    Partial,
    /// Bridge mapping is stale past its freshness floor.
    Stale,
    /// Bridge is unavailable on this platform / build.
    Unavailable,
}

impl A11yBridgeState {
    /// Every bridge state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::BridgedActive,
        Self::Partial,
        Self::Stale,
        Self::Unavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BridgedActive => "bridged_active",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Controlled focus-return disposition after an async update or overlay teardown.
///
/// Mirrors the canonical `focus_return_state` tokens owned by the focus / zoom /
/// pointer-independence contract. Every token returns focus to a real owner —
/// focus never teleports to an unrelated surface or vanishes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum A11yFocusReturnDisposition {
    /// Focus returned to the exact prior owner.
    ReturnedExact,
    /// Focus returned to the nearest safe ancestor.
    ReturnedNearestSafeAncestor,
    /// Focus returned to the current batch or detail owner.
    ReturnedCurrentBatchOrDetailOwner,
    /// Focus returned to an announced placeholder re-entry point.
    ReturnedPlaceholderAnnounced,
    /// A focus loss was denied; the prior owner was kept.
    FocusLossDenied,
    /// Focus return is not applicable for this non-interactive surface.
    FocusNotApplicableNonInteractive,
}

impl A11yFocusReturnDisposition {
    /// Every disposition, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReturnedExact,
        Self::ReturnedNearestSafeAncestor,
        Self::ReturnedCurrentBatchOrDetailOwner,
        Self::ReturnedPlaceholderAnnounced,
        Self::FocusLossDenied,
        Self::FocusNotApplicableNonInteractive,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReturnedExact => "returned_exact",
            Self::ReturnedNearestSafeAncestor => "returned_nearest_safe_ancestor",
            Self::ReturnedCurrentBatchOrDetailOwner => "returned_current_batch_or_detail_owner",
            Self::ReturnedPlaceholderAnnounced => "returned_placeholder_announced",
            Self::FocusLossDenied => "focus_loss_denied",
            Self::FocusNotApplicableNonInteractive => "focus_not_applicable_non_interactive",
        }
    }
}

/// Controlled semantic role class a surface descriptor or label model exposes.
///
/// Groups the canonical accessibility-tree `node_kind` / `generic_role` taxonomy
/// into the broad structural classes a dynamic surface must speak so its semantic
/// structure is never visual-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum A11ySemanticRoleClass {
    /// Landmark region / structural boundary.
    LandmarkRegion,
    /// Structure group / container.
    StructureGroup,
    /// Interactive control (button, tab, menu item, control).
    InteractiveControl,
    /// Text document / editor content.
    TextDocument,
    /// Status / notification region.
    StatusRegion,
    /// Live log / terminal region.
    LiveLogRegion,
    /// Data grid row / cell.
    DataGridCell,
    /// Notebook cell / output.
    NotebookCell,
}

impl A11ySemanticRoleClass {
    /// Every role class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::LandmarkRegion,
        Self::StructureGroup,
        Self::InteractiveControl,
        Self::TextDocument,
        Self::StatusRegion,
        Self::LiveLogRegion,
        Self::DataGridCell,
        Self::NotebookCell,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LandmarkRegion => "landmark_region",
            Self::StructureGroup => "structure_group",
            Self::InteractiveControl => "interactive_control",
            Self::TextDocument => "text_document",
            Self::StatusRegion => "status_region",
            Self::LiveLogRegion => "live_log_region",
            Self::DataGridCell => "data_grid_cell",
            Self::NotebookCell => "notebook_cell",
        }
    }
}

/// Evidence requirement level for an object row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DynamicSurfaceA11yEvidenceRequirement {
    /// At least one assistive-tech proof packet is required.
    Required,
    /// Proof is recommended but not blocking.
    Recommended,
    /// Proof is optional.
    Optional,
    /// Not applicable for this object's current qualification.
    NotApplicable,
}

impl M5DynamicSurfaceA11yEvidenceRequirement {
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

/// Downgrade trigger that can narrow an object below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DynamicSurfaceA11yDowngradeTrigger {
    /// Assistive-tech proof packet has gone stale.
    ProofStale,
    /// OS accessibility bridge is unavailable on the target.
    BridgeUnavailable,
    /// OS accessibility bridge is only partial or stale.
    BridgePartialOrStale,
    /// Focus teleported to an unrelated surface on an async update.
    FocusTeleported,
    /// Focus vanished (lost without a return target) on an async update.
    FocusLost,
    /// A live region spammed instead of coalescing.
    LiveRegionSpam,
    /// A dynamic state change announced repaint noise instead of meaning.
    AnnouncementMeaningLost,
    /// A surface or summary lost its non-visual fidelity.
    NonVisualFidelityLost,
    /// A screen-reader label or role drifted from its semantic source.
    LabelOrRoleDrift,
    /// A surface exposed state only via pointer hover or visual-only cues.
    PointerOrHoverDependence,
    /// Policy or trust restriction applies.
    PolicyBlocked,
    /// An upstream accessibility dependency narrowed.
    UpstreamDependencyNarrowed,
}

impl M5DynamicSurfaceA11yDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::BridgeUnavailable,
        Self::BridgePartialOrStale,
        Self::FocusTeleported,
        Self::FocusLost,
        Self::LiveRegionSpam,
        Self::AnnouncementMeaningLost,
        Self::NonVisualFidelityLost,
        Self::LabelOrRoleDrift,
        Self::PointerOrHoverDependence,
        Self::PolicyBlocked,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::BridgeUnavailable => "bridge_unavailable",
            Self::BridgePartialOrStale => "bridge_partial_or_stale",
            Self::FocusTeleported => "focus_teleported",
            Self::FocusLost => "focus_lost",
            Self::LiveRegionSpam => "live_region_spam",
            Self::AnnouncementMeaningLost => "announcement_meaning_lost",
            Self::NonVisualFidelityLost => "non_visual_fidelity_lost",
            Self::LabelOrRoleDrift => "label_or_role_drift",
            Self::PointerOrHoverDependence => "pointer_or_hover_dependence",
            Self::PolicyBlocked => "policy_blocked",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for an object row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DynamicSurfaceA11yRollbackPosture {
    /// Focus is returned to a real anchor; it never teleports or vanishes.
    FocusAnchorPreserved,
    /// Announcements coalesce; a live region never spams.
    AnnouncementCoalescedNotSpammed,
    /// Semantic structure is preserved; the surface is never visual-only.
    SemanticStructurePreserved,
    /// A durable fallback surface is kept for blocking states.
    DurableFallbackKept,
    /// Bridge degradation is disclosed, never hidden.
    BridgeDegradationDisclosed,
    /// Not applicable for the object's current qualification.
    NotApplicable,
}

impl M5DynamicSurfaceA11yRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FocusAnchorPreserved => "focus_anchor_preserved",
            Self::AnnouncementCoalescedNotSpammed => "announcement_coalesced_not_spammed",
            Self::SemanticStructurePreserved => "semantic_structure_preserved",
            Self::DurableFallbackKept => "durable_fallback_kept",
            Self::BridgeDegradationDisclosed => "bridge_degradation_disclosed",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project an accessibility object's qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DynamicSurfaceA11yConsumerSurface {
    /// Custom-rendered shell surface.
    Shell,
    /// Custom-rendered editor surface.
    Editor,
    /// Custom-rendered terminal surface.
    Terminal,
    /// Custom-rendered notebook surface.
    Notebook,
    /// Dense data grid / collection surface.
    DataGrid,
    /// Review / diff surface.
    Review,
    /// Help / About surface.
    Help,
    /// Presentation / walkthrough surface.
    Presentation,
    /// Support / export packet.
    SupportExport,
    /// AI explain / patch-review surfaces.
    AiSurfaces,
}

impl M5DynamicSurfaceA11yConsumerSurface {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Editor => "editor",
            Self::Terminal => "terminal",
            Self::Notebook => "notebook",
            Self::DataGrid => "data_grid",
            Self::Review => "review",
            Self::Help => "help",
            Self::Presentation => "presentation",
            Self::SupportExport => "support_export",
            Self::AiSurfaces => "ai_surfaces",
        }
    }
}

/// One row in the M5 dynamic-surface accessibility matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DynamicSurfaceA11yObjectRow {
    /// Governed accessibility object.
    pub object_kind: M5DynamicSurfaceA11yObjectKind,
    /// Qualification class earned by this object.
    pub qualification: M5DynamicSurfaceA11yQualificationClass,
    /// Owner role accountable for keeping this object's assistive-tech truth current.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required fields the object must carry.
    pub required_fields: Vec<String>,
    /// Controlled state vocabularies this object carries.
    pub state_vocabularies: Vec<M5DynamicSurfaceA11yStateVocabulary>,
    /// Announcement politeness classes admitted by this object.
    pub announcement_politeness: Vec<A11yAnnouncementPoliteness>,
    /// Coalescing strategies admitted by this object.
    pub coalescing_strategies: Vec<A11yCoalescingStrategy>,
    /// Fallback-durability classes admitted by this object.
    pub fallback_durabilities: Vec<A11yFallbackDurability>,
    /// Non-visual fidelity classes admitted by this object.
    pub non_visual_fidelities: Vec<A11yNonVisualFidelity>,
    /// Bridge states admitted by this object.
    pub bridge_states: Vec<A11yBridgeState>,
    /// Focus-return dispositions admitted by this object.
    pub focus_return_dispositions: Vec<A11yFocusReturnDisposition>,
    /// Semantic role classes admitted by this object.
    pub semantic_role_classes: Vec<A11ySemanticRoleClass>,
    /// Evidence requirement level.
    pub evidence_requirement: M5DynamicSurfaceA11yEvidenceRequirement,
    /// Assistive-tech proof packet refs that keep this object current.
    pub required_proof_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this object.
    pub downgrade_triggers: Vec<M5DynamicSurfaceA11yDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5DynamicSurfaceA11yRollbackPosture,
    /// Source contract refs consumed by this object.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this object's qualification.
    pub consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
}

impl M5DynamicSurfaceA11yObjectRow {
    /// Returns true when the row declares the given vocabulary.
    fn declares(&self, vocab: M5DynamicSurfaceA11yStateVocabulary) -> bool {
        self.state_vocabularies.contains(&vocab)
    }

    /// Returns true when the token vec for `vocab` is non-empty.
    fn vocab_tokens_present(&self, vocab: M5DynamicSurfaceA11yStateVocabulary) -> bool {
        use M5DynamicSurfaceA11yStateVocabulary as V;
        match vocab {
            V::AnnouncementPoliteness => !self.announcement_politeness.is_empty(),
            V::CoalescingStrategy => !self.coalescing_strategies.is_empty(),
            V::FallbackDurability => !self.fallback_durabilities.is_empty(),
            V::NonVisualFidelity => !self.non_visual_fidelities.is_empty(),
            V::BridgeState => !self.bridge_states.is_empty(),
            V::FocusReturnDisposition => !self.focus_return_dispositions.is_empty(),
            V::SemanticRoleClass => !self.semantic_role_classes.is_empty(),
        }
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
///
/// Each field lists every canonical token for one controlled vocabulary, in
/// declaration order. The matrix validates each list against the typed `ALL`
/// arrays so the frozen vocabulary cannot silently drift.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DynamicSurfaceA11yVocabularySet {
    /// Announcement-politeness tokens.
    pub announcement_politeness: Vec<String>,
    /// Coalescing-strategy tokens.
    pub coalescing_strategies: Vec<String>,
    /// Fallback-durability tokens.
    pub fallback_durabilities: Vec<String>,
    /// Non-visual-fidelity tokens.
    pub non_visual_fidelities: Vec<String>,
    /// Bridge-state tokens.
    pub bridge_states: Vec<String>,
    /// Focus-return-disposition tokens.
    pub focus_return_dispositions: Vec<String>,
    /// Semantic-role-class tokens.
    pub semantic_role_classes: Vec<String>,
}

impl M5DynamicSurfaceA11yVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            announcement_politeness: A11yAnnouncementPoliteness::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            coalescing_strategies: A11yCoalescingStrategy::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            fallback_durabilities: A11yFallbackDurability::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            non_visual_fidelities: A11yNonVisualFidelity::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            bridge_states: A11yBridgeState::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            focus_return_dispositions: A11yFocusReturnDisposition::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            semantic_role_classes: A11ySemanticRoleClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Assistive-technology conformance review block.
///
/// Every flag is a hard invariant; all must hold for the matrix to validate. The
/// flags encode the track invariant: non-visual truth stays first-class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DynamicSurfaceA11yConformanceReview {
    /// Custom-rendered surfaces expose semantic structure, not visual-only state.
    pub custom_surfaces_expose_semantic_structure: bool,
    /// Focus never teleports or vanishes on an async update.
    pub focus_never_teleports_or_vanishes_on_async_update: bool,
    /// Live regions coalesce rather than spam.
    pub live_regions_coalesce_rather_than_spam: bool,
    /// Dynamic state changes announce meaning, not repaint noise.
    pub dynamic_state_changes_announce_meaning_not_repaint_noise: bool,
    /// No surface depends on visual-only state or pointer hover.
    pub no_visual_only_state_or_pointer_hover_dependence: bool,
    /// Dense surfaces expose non-visual summaries.
    pub dense_surfaces_expose_non_visual_summaries: bool,
    /// Durable fallbacks are present for blocking states.
    pub durable_fallbacks_present_for_blocking_states: bool,
    /// Bridge degradation is disclosed, never hidden.
    pub bridge_degradation_disclosed_not_hidden: bool,
    /// Every surface resolves to one bridge-aware contract, not per-surface ad hoc
    /// behavior.
    pub one_bridge_aware_contract_not_per_surface_adhoc: bool,
    /// Claimed rows auto-narrow when bridge or proof state goes stale.
    pub claimed_rows_auto_narrow_when_bridge_or_proof_stale: bool,
    /// Downgrade narrows the claim rather than hiding the object.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified objects automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DynamicSurfaceA11yConsumerProjection {
    /// Shell consumes the shared accessibility object model.
    pub shell_consumes_object_model: bool,
    /// Editor exposes semantic structure from the object model.
    pub editor_exposes_semantic_structure: bool,
    /// Terminal announces via the shared live-region contract.
    pub terminal_announces_via_live_region: bool,
    /// Notebook returns focus on async updates.
    pub notebook_returns_focus_on_async_update: bool,
    /// Data grid exposes a non-visual summary.
    pub data_grid_exposes_non_visual_summary: bool,
    /// Review exposes semantic structure from the object model.
    pub review_exposes_semantic_structure: bool,
    /// Help documents the bridge-diagnostics packet.
    pub help_documents_bridge_diagnostics: bool,
    /// Presentation announces meaning, not repaint noise.
    pub presentation_announces_meaning_not_repaint: bool,
    /// Support export shows the shared object model.
    pub support_export_shows_object_model: bool,
    /// Unqualified surfaces are visibly labeled when not covered by this packet.
    pub unqualified_surfaces_labeled_when_uncovered: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DynamicSurfaceA11yProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the object.
    pub auto_narrow_on_stale: bool,
}

/// Release and mirror/offline parity posture for the dynamic-surface a11y lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DynamicSurfaceA11yReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting mirror/offline packet for the lane.
    pub mirror_offline_packet_ref: String,
    /// True when support/export parity is required for every object.
    pub support_export_parity_required: bool,
    /// True when mirror/offline parity is required for every object.
    pub mirror_offline_parity_required: bool,
    /// True when Stable promotion blocks while a claimed dynamic surface lacks a
    /// mapped assistive-tech proof row or current matrix entry.
    pub stable_promotion_blocks_without_mapped_proof: bool,
}

/// Constructor input for [`M5DynamicSurfaceA11yMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DynamicSurfaceA11yMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Object rows.
    pub object_rows: Vec<M5DynamicSurfaceA11yObjectRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5DynamicSurfaceA11yConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DynamicSurfaceA11yConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 dynamic-surface accessibility matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DynamicSurfaceA11yMatrixPacket {
    /// Record kind; must equal [`M5_DYNAMIC_A11Y_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DYNAMIC_A11Y_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Object rows.
    pub object_rows: Vec<M5DynamicSurfaceA11yObjectRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5DynamicSurfaceA11yConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DynamicSurfaceA11yConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DynamicSurfaceA11yMatrixPacket {
    /// Builds an M5 dynamic-surface accessibility matrix packet from stable-lane
    /// input.
    pub fn new(input: M5DynamicSurfaceA11yMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_DYNAMIC_A11Y_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_DYNAMIC_A11Y_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            object_rows: input.object_rows,
            vocabulary_set: input.vocabulary_set,
            conformance_review: input.conformance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 dynamic-surface accessibility matrix invariants.
    pub fn validate(&self) -> Vec<M5DynamicSurfaceA11yMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DYNAMIC_A11Y_MATRIX_RECORD_KIND {
            violations.push(M5DynamicSurfaceA11yMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DYNAMIC_A11Y_MATRIX_SCHEMA_VERSION {
            violations.push(M5DynamicSurfaceA11yMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DynamicSurfaceA11yMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_object_rows(self, &mut violations);
        validate_conformance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 dynamic-surface a11y matrix packet serializes"),
        ) {
            violations.push(M5DynamicSurfaceA11yMatrixViolation::RawBoundaryMaterialInExport);
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
            .expect("m5 dynamic-surface a11y matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .object_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Accessibility-Bridge, Live-Announcement, Focus-Return, and Non-Visual Dynamic-Surface Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Objects: {} ({} stable)\n",
            self.object_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Objects\n\n");
        for row in &self.object_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.object_kind.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Vocabularies: {}\n",
                row.state_vocabularies
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Rollback: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 dynamic-surface a11y matrix export.
#[derive(Debug)]
pub enum M5DynamicSurfaceA11yMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DynamicSurfaceA11yMatrixViolation>),
}

impl fmt::Display for M5DynamicSurfaceA11yMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 dynamic-surface a11y matrix export parse failed: {error}"
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
                    "m5 dynamic-surface a11y matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DynamicSurfaceA11yMatrixArtifactError {}

/// Validation failures emitted by [`M5DynamicSurfaceA11yMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DynamicSurfaceA11yMatrixViolation {
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
    /// A required governed object is missing from the matrix.
    RequiredObjectMissing,
    /// An object row is incomplete.
    ObjectRowIncomplete,
    /// An object row omits a vocabulary its kind requires.
    RequiredVocabularyMissing,
    /// A declared vocabulary has no concrete tokens.
    DeclaredVocabularyHasNoTokens,
    /// A token vec is populated for a vocabulary the row does not declare.
    UndeclaredVocabularyHasTokens,
    /// An object claiming Stable is missing required proof packet refs.
    StableObjectMissingProof,
    /// An object has no downgrade triggers.
    DowngradeTriggersMissing,
    /// An object has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Conformance review does not satisfy required invariants.
    ConformanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5DynamicSurfaceA11yMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::ObjectRowIncomplete => "object_row_incomplete",
            Self::RequiredVocabularyMissing => "required_vocabulary_missing",
            Self::DeclaredVocabularyHasNoTokens => "declared_vocabulary_has_no_tokens",
            Self::UndeclaredVocabularyHasTokens => "undeclared_vocabulary_has_tokens",
            Self::StableObjectMissingProof => "stable_object_missing_proof",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ConformanceReviewIncomplete => "conformance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 dynamic-surface a11y matrix export.
pub fn current_stable_m5_dynamic_surface_a11y_matrix_export(
) -> Result<M5DynamicSurfaceA11yMatrixPacket, M5DynamicSurfaceA11yMatrixArtifactError> {
    let packet: M5DynamicSurfaceA11yMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/a11y/m5-dynamic-surfaces/support_export.json"
    )))
    .map_err(M5DynamicSurfaceA11yMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DynamicSurfaceA11yMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5DynamicSurfaceA11yMatrixPacket,
    violations: &mut Vec<M5DynamicSurfaceA11yMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DYNAMIC_A11Y_MATRIX_SCHEMA_REF,
        M5_DYNAMIC_A11Y_MATRIX_DOC_REF,
        M5_DYNAMIC_A11Y_SCREEN_READER_CONTRACT_REF,
        M5_DYNAMIC_A11Y_TREE_CONTRACT_REF,
        M5_DYNAMIC_A11Y_FOCUS_CONTRACT_REF,
        M5_DYNAMIC_A11Y_COLLECTION_CONTRACT_REF,
        M5_DYNAMIC_A11Y_SHELL_BRIDGE_CONTRACT_REF,
        M5_DYNAMIC_A11Y_OPERATIONAL_PARITY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DynamicSurfaceA11yMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DynamicSurfaceA11yMatrixPacket,
    violations: &mut Vec<M5DynamicSurfaceA11yMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DynamicSurfaceA11yMatrixViolation::VocabularySetDrift);
    }
}

fn validate_object_rows(
    packet: &M5DynamicSurfaceA11yMatrixPacket,
    violations: &mut Vec<M5DynamicSurfaceA11yMatrixViolation>,
) {
    let present: BTreeSet<M5DynamicSurfaceA11yObjectKind> = packet
        .object_rows
        .iter()
        .map(|row| row.object_kind)
        .collect();
    for required in M5DynamicSurfaceA11yObjectKind::ALL {
        if !present.contains(&required) {
            violations.push(M5DynamicSurfaceA11yMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.object_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.required_fields.is_empty()
            || row.state_vocabularies.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5DynamicSurfaceA11yMatrixViolation::ObjectRowIncomplete);
        }

        for required_vocab in row.object_kind.required_state_vocabularies() {
            if !row.declares(*required_vocab) {
                violations.push(M5DynamicSurfaceA11yMatrixViolation::RequiredVocabularyMissing);
            }
        }

        for vocab in M5DynamicSurfaceA11yStateVocabulary::ALL {
            let declared = row.declares(vocab);
            let has_tokens = row.vocab_tokens_present(vocab);
            if declared && !has_tokens {
                violations.push(M5DynamicSurfaceA11yMatrixViolation::DeclaredVocabularyHasNoTokens);
            }
            if !declared && has_tokens {
                violations.push(M5DynamicSurfaceA11yMatrixViolation::UndeclaredVocabularyHasTokens);
            }
        }

        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DynamicSurfaceA11yMatrixViolation::StableObjectMissingProof);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DynamicSurfaceA11yMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DynamicSurfaceA11yMatrixViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_conformance_review(
    packet: &M5DynamicSurfaceA11yMatrixPacket,
    violations: &mut Vec<M5DynamicSurfaceA11yMatrixViolation>,
) {
    let review = &packet.conformance_review;
    for ok in [
        review.custom_surfaces_expose_semantic_structure,
        review.focus_never_teleports_or_vanishes_on_async_update,
        review.live_regions_coalesce_rather_than_spam,
        review.dynamic_state_changes_announce_meaning_not_repaint_noise,
        review.no_visual_only_state_or_pointer_hover_dependence,
        review.dense_surfaces_expose_non_visual_summaries,
        review.durable_fallbacks_present_for_blocking_states,
        review.bridge_degradation_disclosed_not_hidden,
        review.one_bridge_aware_contract_not_per_surface_adhoc,
        review.claimed_rows_auto_narrow_when_bridge_or_proof_stale,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5DynamicSurfaceA11yMatrixViolation::ConformanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DynamicSurfaceA11yMatrixPacket,
    violations: &mut Vec<M5DynamicSurfaceA11yMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_object_model,
        projection.editor_exposes_semantic_structure,
        projection.terminal_announces_via_live_region,
        projection.notebook_returns_focus_on_async_update,
        projection.data_grid_exposes_non_visual_summary,
        projection.review_exposes_semantic_structure,
        projection.help_documents_bridge_diagnostics,
        projection.presentation_announces_meaning_not_repaint,
        projection.support_export_shows_object_model,
        projection.unqualified_surfaces_labeled_when_uncovered,
    ] {
        if !ok {
            violations.push(M5DynamicSurfaceA11yMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DynamicSurfaceA11yMatrixPacket,
    violations: &mut Vec<M5DynamicSurfaceA11yMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DynamicSurfaceA11yMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DynamicSurfaceA11yMatrixPacket,
    violations: &mut Vec<M5DynamicSurfaceA11yMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
        || !posture.stable_promotion_blocks_without_mapped_proof
    {
        violations.push(M5DynamicSurfaceA11yMatrixViolation::ReleasePostureIncomplete);
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
