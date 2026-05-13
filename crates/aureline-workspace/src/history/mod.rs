//! Bookmark and navigation-history continuity records.
//!
//! This module owns the workspace-side consumer for remapped search and
//! navigation targets. It does not resolve deep links itself; instead it
//! preserves the user's durable bookmark or history artifact, points at the
//! remap packet that made the decision, and records the bounded recovery
//! actions a surface may offer when the target moved, left the active scope,
//! or could not be resolved.

use serde::{Deserialize, Serialize};

use crate::worksets::{ScopeClass, ScopeMode};

/// Schema version for navigation-continuity records.
pub const NAVIGATION_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Identifies the `navigation_continuity_record` record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationContinuityRecordKind {
    /// `navigation_continuity_record`
    NavigationContinuityRecord,
}

/// Durable navigation artifact whose continuity is being preserved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationArtifactKind {
    /// User-authored or imported bookmark.
    Bookmark,
    /// Back/forward or recent-location history entry.
    NavigationHistoryEntry,
}

impl NavigationArtifactKind {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bookmark => "bookmark",
            Self::NavigationHistoryEntry => "navigation_history_entry",
        }
    }
}

/// Continuity outcome for a bookmark or history entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationContinuityState {
    /// The artifact resolved to a new target through a remap packet.
    Remapped,
    /// The artifact remains visible as a placeholder with bounded actions.
    RecoverablePlaceholder,
    /// The artifact did not resolve and carries an explicit failure reason.
    FailedExplicitReason,
}

impl NavigationContinuityState {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Remapped => "remapped",
            Self::RecoverablePlaceholder => "recoverable_placeholder",
            Self::FailedExplicitReason => "failed_explicit_reason",
        }
    }
}

/// Surface or workflow that attempted to reopen the target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationOriginClass {
    /// Open action from a search result row.
    SearchResultOpen,
    /// Direct deep-link resolution.
    DeepLinkResolution,
    /// Bookmark restore or bookmark click.
    BookmarkRestore,
    /// Session restore replaying remembered navigation state.
    SessionRestore,
    /// Back/forward navigation.
    BackForward,
    /// Quick-open recent-place navigation.
    QuickOpenRecent,
    /// Symbol-jump navigation.
    SymbolJump,
}

impl NavigationOriginClass {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchResultOpen => "search_result_open",
            Self::DeepLinkResolution => "deep_link_resolution",
            Self::BookmarkRestore => "bookmark_restore",
            Self::SessionRestore => "session_restore",
            Self::BackForward => "back_forward",
            Self::QuickOpenRecent => "quick_open_recent",
            Self::SymbolJump => "symbol_jump",
        }
    }
}

/// Navigation surface that must keep destination repo or root identity visible.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationSurfaceClass {
    /// Inline peek view.
    Peek,
    /// Read-only preview view.
    Preview,
    /// Split editor or split tool surface.
    Split,
    /// Open target in a new pane.
    OpenInNewPane,
    /// Back/forward navigation chrome.
    BackNavigation,
    /// Primary open action.
    PrimaryOpen,
}

impl NavigationSurfaceClass {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Peek => "peek",
            Self::Preview => "preview",
            Self::Split => "split",
            Self::OpenInNewPane => "open_in_new_pane",
            Self::BackNavigation => "back_navigation",
            Self::PrimaryOpen => "primary_open",
        }
    }
}

/// Bounded action offered for a drifted or missing navigation target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationRecoveryAction {
    /// Open the remapped target.
    OpenRemappedTarget,
    /// Preview the remapped target without changing the active pane.
    PreviewRemappedTarget,
    /// Widen the active scope through a reviewable transition.
    WidenScope,
    /// Open the destination root explicitly.
    OpenTargetRoot,
    /// Keep the current scope and leave the artifact as a placeholder.
    KeepCurrentScope,
    /// Ask the user to locate or reattach the missing target.
    LocateMissingTarget,
    /// Rebuild or warm the index needed to resolve the target.
    RebuildIndex,
    /// Inspect the remap packet that made the continuity decision.
    InspectRemapPacket,
    /// Remove the stale bookmark or history row metadata.
    RemoveArtifact,
    /// Retry resolution after the current index or root finishes loading.
    RetryAfterIndexReady,
}

impl NavigationRecoveryAction {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRemappedTarget => "open_remapped_target",
            Self::PreviewRemappedTarget => "preview_remapped_target",
            Self::WidenScope => "widen_scope",
            Self::OpenTargetRoot => "open_target_root",
            Self::KeepCurrentScope => "keep_current_scope",
            Self::LocateMissingTarget => "locate_missing_target",
            Self::RebuildIndex => "rebuild_index",
            Self::InspectRemapPacket => "inspect_remap_packet",
            Self::RemoveArtifact => "remove_artifact",
            Self::RetryAfterIndexReady => "retry_after_index_ready",
        }
    }

    /// True when the action is an explicit next step for a missing or
    /// outside-scope target.
    pub const fn is_bounded_missing_target_action(self) -> bool {
        matches!(
            self,
            Self::WidenScope
                | Self::OpenTargetRoot
                | Self::KeepCurrentScope
                | Self::LocateMissingTarget
                | Self::RebuildIndex
                | Self::RetryAfterIndexReady
        )
    }
}

/// Explicit reason a bookmark or history entry could not reopen exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationFailureReason {
    /// The target no longer exists or no resolver can find it.
    TargetMissing,
    /// The target exists but is outside the active workset or slice.
    TargetScopeExcluded,
    /// Policy or trust state blocks access to the target.
    TargetPolicyBlocked,
    /// The owning root is not currently reachable.
    RootUnavailable,
    /// More than one remap candidate matched the old target.
    AmbiguousRemap,
    /// The required index or graph lane is not ready.
    IndexNotReady,
    /// The target kind is not supported by the current surface.
    UnsupportedTargetKind,
    /// The target belongs to a different workspace than the active authority.
    WorkspaceMismatch,
    /// Remap evidence exists but does not meet the confidence floor.
    ConfidenceTooLow,
}

impl NavigationFailureReason {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetMissing => "target_missing",
            Self::TargetScopeExcluded => "target_scope_excluded",
            Self::TargetPolicyBlocked => "target_policy_blocked",
            Self::RootUnavailable => "root_unavailable",
            Self::AmbiguousRemap => "ambiguous_remap",
            Self::IndexNotReady => "index_not_ready",
            Self::UnsupportedTargetKind => "unsupported_target_kind",
            Self::WorkspaceMismatch => "workspace_mismatch",
            Self::ConfidenceTooLow => "confidence_too_low",
        }
    }
}

/// Scope identity preserved while reopening a bookmark or history entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationScopeIdentity {
    /// Workspace authority active when the continuity decision was made.
    pub workspace_id_ref: String,
    /// Stable scope/workset/slice identity active at reopen time.
    pub stable_scope_id_ref: String,
    /// Scope class active at reopen time.
    pub scope_class: ScopeClass,
    /// Full or sparse materialization mode for the active scope.
    pub scope_mode: ScopeMode,
    /// Workset id when the active scope is a named workset or slice.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workset_id_ref: Option<String>,
    /// Workset display name when it is safe to surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workset_name: Option<String>,
    /// Root refs included by the active scope.
    #[serde(default)]
    pub active_root_refs: Vec<String>,
    /// Workspace authority that owns the destination target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_workspace_id_ref: Option<String>,
    /// Root ref that owns the destination target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_root_id_ref: Option<String>,
    /// Short root or repo label safe for navigation chrome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_root_label: Option<String>,
}

impl NavigationScopeIdentity {
    /// Returns true when the destination crosses the active root, workset, or
    /// workspace boundary and therefore needs visible destination identity.
    pub fn crosses_visible_boundary(&self) -> bool {
        let workspace_crossed = self
            .target_workspace_id_ref
            .as_deref()
            .is_some_and(|target| target != self.workspace_id_ref);
        let root_crossed = self
            .target_root_id_ref
            .as_deref()
            .is_some_and(|target| !self.active_root_refs.iter().any(|root| root == target));
        workspace_crossed || root_crossed
    }
}

/// Destination identity visible on one navigation surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationDestinationVisibility {
    /// Surface where the destination identity appears.
    pub surface_class: NavigationSurfaceClass,
    /// Root ref shown on that surface.
    pub target_root_id_ref: String,
    /// Root or repo label shown on that surface.
    pub target_root_label: String,
    /// True when the surface visibly distinguishes the destination root.
    pub destination_repo_visible: bool,
}

/// Workspace-side continuity projection for one bookmark or history artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationContinuityRecord {
    /// Stable record-kind tag.
    pub record_kind: NavigationContinuityRecordKind,
    /// Integer schema version for this record.
    pub navigation_continuity_schema_version: u32,
    /// Stable continuity record id.
    pub continuity_id: String,
    /// Durable artifact whose continuity is being preserved.
    pub artifact_kind: NavigationArtifactKind,
    /// Opaque bookmark or history entry id.
    pub artifact_id_ref: String,
    /// Search deep-link id, when the artifact points through one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deep_link_id_ref: Option<String>,
    /// Remap packet that owns old/new target and confidence truth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remap_packet_id_ref: Option<String>,
    /// Continuity outcome for this artifact.
    pub continuity_state: NavigationContinuityState,
    /// Surface or workflow that attempted the reopen.
    pub origin_class: NavigationOriginClass,
    /// Scope and root identities preserved at reopen time.
    pub scope_identity: NavigationScopeIdentity,
    /// Destination identity projections for peek, preview, split, open, and back paths.
    #[serde(default)]
    pub destination_visibility: Vec<NavigationDestinationVisibility>,
    /// Bounded actions the surface may offer next.
    #[serde(default)]
    pub recovery_actions: Vec<NavigationRecoveryAction>,
    /// Explicit failure reason when continuity fails or degrades.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<NavigationFailureReason>,
    /// Placeholder row/card id when the target remains visible but unopened.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_ref: Option<String>,
    /// Monotonic or fixture timestamp for export parity.
    pub emitted_at: String,
}

impl NavigationContinuityRecord {
    /// Validates the continuity record against the workspace-history invariants.
    pub fn validate(&self) -> Result<(), NavigationContinuityError> {
        if self.navigation_continuity_schema_version != NAVIGATION_CONTINUITY_SCHEMA_VERSION {
            return Err(NavigationContinuityError::SchemaVersionMismatch(
                self.navigation_continuity_schema_version,
            ));
        }
        if self.continuity_id.trim().is_empty() {
            return Err(NavigationContinuityError::EmptyContinuityId);
        }
        if self.artifact_id_ref.trim().is_empty() {
            return Err(NavigationContinuityError::EmptyArtifactId);
        }
        if self.scope_identity.workspace_id_ref.trim().is_empty() {
            return Err(NavigationContinuityError::EmptyWorkspaceId);
        }
        if self.scope_identity.stable_scope_id_ref.trim().is_empty() {
            return Err(NavigationContinuityError::EmptyStableScopeId);
        }
        match self.continuity_state {
            NavigationContinuityState::Remapped => {
                self.require_remap_packet()?;
                if self.failure_reason.is_some() {
                    return Err(NavigationContinuityError::UnexpectedFailureReason);
                }
                self.require_action(NavigationRecoveryAction::OpenRemappedTarget)?;
            }
            NavigationContinuityState::RecoverablePlaceholder => {
                self.require_remap_packet()?;
                if self.placeholder_ref.as_deref().map_or(true, str::is_empty) {
                    return Err(NavigationContinuityError::MissingPlaceholderRef);
                }
                if !self
                    .recovery_actions
                    .iter()
                    .any(|action| action.is_bounded_missing_target_action())
                {
                    return Err(NavigationContinuityError::MissingBoundedRecoveryAction);
                }
            }
            NavigationContinuityState::FailedExplicitReason => {
                if self.failure_reason.is_none() {
                    return Err(NavigationContinuityError::MissingFailureReason);
                }
            }
        }
        if self.scope_identity.crosses_visible_boundary() {
            validate_destination_visibility(&self.destination_visibility)?;
        }
        Ok(())
    }

    fn require_remap_packet(&self) -> Result<(), NavigationContinuityError> {
        if self
            .remap_packet_id_ref
            .as_deref()
            .map_or(true, str::is_empty)
        {
            return Err(NavigationContinuityError::MissingRemapPacketRef);
        }
        Ok(())
    }

    fn require_action(
        &self,
        action: NavigationRecoveryAction,
    ) -> Result<(), NavigationContinuityError> {
        if self.recovery_actions.contains(&action) {
            Ok(())
        } else {
            Err(NavigationContinuityError::MissingRecoveryAction(action))
        }
    }
}

fn validate_destination_visibility(
    visibility: &[NavigationDestinationVisibility],
) -> Result<(), NavigationContinuityError> {
    const REQUIRED: [NavigationSurfaceClass; 5] = [
        NavigationSurfaceClass::Peek,
        NavigationSurfaceClass::Preview,
        NavigationSurfaceClass::Split,
        NavigationSurfaceClass::OpenInNewPane,
        NavigationSurfaceClass::BackNavigation,
    ];

    for surface in REQUIRED {
        let Some(row) = visibility.iter().find(|row| row.surface_class == surface) else {
            return Err(NavigationContinuityError::MissingDestinationVisibility(
                surface,
            ));
        };
        if row.target_root_id_ref.trim().is_empty() || row.target_root_label.trim().is_empty() {
            return Err(NavigationContinuityError::IncompleteDestinationVisibility(
                surface,
            ));
        }
        if !row.destination_repo_visible {
            return Err(NavigationContinuityError::DestinationRepoNotVisible(
                surface,
            ));
        }
    }
    Ok(())
}

/// Validation errors for [`NavigationContinuityRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationContinuityError {
    /// The record uses an unsupported schema version.
    SchemaVersionMismatch(u32),
    /// `continuity_id` is empty.
    EmptyContinuityId,
    /// `artifact_id_ref` is empty.
    EmptyArtifactId,
    /// The active workspace identity is empty.
    EmptyWorkspaceId,
    /// The active stable scope identity is empty.
    EmptyStableScopeId,
    /// Remapped or placeholder continuity lacks a remap packet ref.
    MissingRemapPacketRef,
    /// A remapped continuity row also carried a failure reason.
    UnexpectedFailureReason,
    /// A required recovery action is absent.
    MissingRecoveryAction(NavigationRecoveryAction),
    /// Recoverable placeholder continuity lacks a placeholder ref.
    MissingPlaceholderRef,
    /// Recoverable placeholder continuity lacks a bounded next action.
    MissingBoundedRecoveryAction,
    /// Failed continuity lacks an explicit failure reason.
    MissingFailureReason,
    /// Cross-root or cross-workspace navigation lacks a required surface row.
    MissingDestinationVisibility(NavigationSurfaceClass),
    /// A required destination-visibility row lacks root identity or label.
    IncompleteDestinationVisibility(NavigationSurfaceClass),
    /// A required destination surface does not visibly show the destination root.
    DestinationRepoNotVisible(NavigationSurfaceClass),
}

impl std::fmt::Display for NavigationContinuityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch(version) => write!(
                f,
                "unsupported navigation_continuity_schema_version {version}; expected {NAVIGATION_CONTINUITY_SCHEMA_VERSION}"
            ),
            Self::EmptyContinuityId => write!(f, "continuity_id must not be empty"),
            Self::EmptyArtifactId => write!(f, "artifact_id_ref must not be empty"),
            Self::EmptyWorkspaceId => write!(f, "workspace_id_ref must not be empty"),
            Self::EmptyStableScopeId => write!(f, "stable_scope_id_ref must not be empty"),
            Self::MissingRemapPacketRef => {
                write!(f, "remapped continuity must cite a remap packet")
            }
            Self::UnexpectedFailureReason => {
                write!(f, "remapped continuity must not carry a failure reason")
            }
            Self::MissingRecoveryAction(action) => {
                write!(f, "missing required recovery action {}", action.as_str())
            }
            Self::MissingPlaceholderRef => {
                write!(f, "recoverable placeholder continuity requires placeholder_ref")
            }
            Self::MissingBoundedRecoveryAction => write!(
                f,
                "recoverable placeholder continuity requires a bounded recovery action"
            ),
            Self::MissingFailureReason => {
                write!(f, "failed continuity requires an explicit failure reason")
            }
            Self::MissingDestinationVisibility(surface) => write!(
                f,
                "cross-boundary navigation requires destination visibility on {}",
                surface.as_str()
            ),
            Self::IncompleteDestinationVisibility(surface) => write!(
                f,
                "destination visibility on {} must include root id and label",
                surface.as_str()
            ),
            Self::DestinationRepoNotVisible(surface) => write!(
                f,
                "destination repo/root must be visible on {}",
                surface.as_str()
            ),
        }
    }
}

impl std::error::Error for NavigationContinuityError {}
