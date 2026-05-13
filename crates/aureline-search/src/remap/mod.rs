//! Deep-link remap packets for search and navigation targets.
//!
//! A remap packet is the canonical search-side record emitted when a bookmark,
//! recent location, search result, or deep link is reopened after files moved,
//! symbols renamed, or the active workset changed. The packet carries the old
//! target, the resolved target candidate when one exists, the active
//! workset/scope identity, confidence and evidence, destination-root visibility,
//! and bounded recovery actions. Surfaces consume this record instead of
//! guessing from a raw path.

use serde::{Deserialize, Serialize};

use aureline_workspace::{
    NavigationDestinationVisibility, NavigationRecoveryAction, NavigationSurfaceClass, ScopeClass,
    ScopeMode,
};

/// Schema version for deep-link remap packets.
pub const DEEP_LINK_REMAP_PACKET_SCHEMA_VERSION: u32 = 1;

/// Identifies the `deep_link_remap_packet_record` record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkRemapRecordKind {
    /// `deep_link_remap_packet_record`
    DeepLinkRemapPacketRecord,
}

/// Outcome class for a deep-link remap decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkRemapOutcome {
    /// The original target still resolves exactly.
    ResolvedExact,
    /// A safe remap target was found and may be opened.
    Remapped,
    /// A target candidate exists, but the surface must show a placeholder and
    /// bounded next actions before crossing scope or trust boundaries.
    RecoverablePlaceholder,
    /// No safe target resolved; the packet carries an explicit failure reason.
    FailedExplicitReason,
}

impl DeepLinkRemapOutcome {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvedExact => "resolved_exact",
            Self::Remapped => "remapped",
            Self::RecoverablePlaceholder => "recoverable_placeholder",
            Self::FailedExplicitReason => "failed_explicit_reason",
        }
    }
}

/// Drift state reported by deep-link resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkDriftState {
    /// The saved target resolved without remapping.
    ResolvedExact,
    /// The saved target resolved through a recorded move or rename chain.
    ResolvedRemapped,
    /// More than one candidate matched the saved target.
    ResolvedAmbiguous,
    /// The target no longer exists or cannot be found.
    TargetMissing,
    /// The target moved from its prior file/root location.
    TargetMoved,
    /// The target renamed but still has recognizable lineage.
    TargetRenamed,
    /// The target belongs to a different branch or revision.
    TargetBranchDrifted,
    /// Policy or trust state blocks the target.
    TargetPolicyBlocked,
    /// The target is outside the active workset or slice.
    TargetScopeExcluded,
    /// The target may exist, but the necessary index is not ready.
    IndexNotReadyForTarget,
    /// No resolver can safely classify the target.
    Unresolvable,
}

impl DeepLinkDriftState {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvedExact => "resolved_exact",
            Self::ResolvedRemapped => "resolved_remapped",
            Self::ResolvedAmbiguous => "resolved_ambiguous",
            Self::TargetMissing => "target_missing",
            Self::TargetMoved => "target_moved",
            Self::TargetRenamed => "target_renamed",
            Self::TargetBranchDrifted => "target_branch_drifted",
            Self::TargetPolicyBlocked => "target_policy_blocked",
            Self::TargetScopeExcluded => "target_scope_excluded",
            Self::IndexNotReadyForTarget => "index_not_ready_for_target",
            Self::Unresolvable => "unresolvable",
        }
    }

    const fn is_resolved(self) -> bool {
        matches!(
            self,
            Self::ResolvedExact | Self::ResolvedRemapped | Self::ResolvedAmbiguous
        )
    }
}

/// Target kind a deep link or remap packet can point at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemapTargetKind {
    /// Workspace file or path-like target.
    WorkspaceFile,
    /// Buffer-local anchor.
    BufferAnchor,
    /// Symbol known through the graph or structural index.
    GraphSymbol,
    /// Graph edge or relationship target.
    GraphEdge,
    /// Documentation page anchor.
    DocsPageAnchor,
    /// Bookmark target.
    Bookmark,
    /// Recent place or route target.
    RecentLocation,
}

impl RemapTargetKind {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceFile => "workspace_file",
            Self::BufferAnchor => "buffer_anchor",
            Self::GraphSymbol => "graph_symbol",
            Self::GraphEdge => "graph_edge",
            Self::DocsPageAnchor => "docs_page_anchor",
            Self::Bookmark => "bookmark",
            Self::RecentLocation => "recent_location",
        }
    }
}

/// Confidence tier for a remap decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemapConfidenceClass {
    /// Stable identity token matched exactly.
    ExactIdentity,
    /// Semantic or graph evidence strongly confirms the remap.
    HighSemantic,
    /// Structural symbol evidence confirms the remap within the current file/root.
    Structural,
    /// Heuristic evidence exists but needs review before opening.
    Heuristic,
    /// Evidence exists but is below the opening confidence floor.
    Insufficient,
    /// No usable evidence was available.
    Unavailable,
}

impl RemapConfidenceClass {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactIdentity => "exact_identity",
            Self::HighSemantic => "high_semantic",
            Self::Structural => "structural",
            Self::Heuristic => "heuristic",
            Self::Insufficient => "insufficient",
            Self::Unavailable => "unavailable",
        }
    }

    /// True when confidence is high enough to open a remapped target without
    /// first degrading to a placeholder.
    pub const fn permits_direct_remap(self) -> bool {
        matches!(
            self,
            Self::ExactIdentity | Self::HighSemantic | Self::Structural
        )
    }
}

/// Evidence family that contributed to remap confidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemapEvidenceClass {
    /// Canonical filesystem identity matched across a move.
    FilesystemIdentity,
    /// Rename/move history joined the old and new target.
    RenameMoveHistory,
    /// Stable symbol identity matched across a rename.
    SymbolStableId,
    /// Graph relation or remap edge linked old and new targets.
    GraphRemapEdge,
    /// Search result identity linked the old and current row.
    PlannerResultIdentity,
    /// User-curated alias linked old and new targets.
    UserCuratedAlias,
    /// Path/name similarity contributed as heuristic evidence.
    PathSimilarity,
    /// Indexed candidate exists but needs additional confirmation.
    IndexedCandidate,
}

impl RemapEvidenceClass {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FilesystemIdentity => "filesystem_identity",
            Self::RenameMoveHistory => "rename_move_history",
            Self::SymbolStableId => "symbol_stable_id",
            Self::GraphRemapEdge => "graph_remap_edge",
            Self::PlannerResultIdentity => "planner_result_identity",
            Self::UserCuratedAlias => "user_curated_alias",
            Self::PathSimilarity => "path_similarity",
            Self::IndexedCandidate => "indexed_candidate",
        }
    }
}

/// Explicit failure reason for a remap packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RemapFailureReason {
    /// The target no longer exists or no resolver can find it.
    TargetMissing,
    /// The target exists but is outside the active workset or slice.
    TargetScopeExcluded,
    /// Policy or trust state blocks access to the target.
    TargetPolicyBlocked,
    /// The owning root is not currently reachable.
    RootUnavailable,
    /// More than one candidate matched the old target.
    AmbiguousCandidates,
    /// The required index or graph lane is not ready.
    IndexNotReady,
    /// The target belongs to a different workspace than the active authority.
    WorkspaceMismatch,
    /// Remap evidence exists but does not meet the confidence floor.
    ConfidenceTooLow,
    /// The target kind is not supported by this resolver.
    UnsupportedTargetKind,
}

impl RemapFailureReason {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetMissing => "target_missing",
            Self::TargetScopeExcluded => "target_scope_excluded",
            Self::TargetPolicyBlocked => "target_policy_blocked",
            Self::RootUnavailable => "root_unavailable",
            Self::AmbiguousCandidates => "ambiguous_candidates",
            Self::IndexNotReady => "index_not_ready",
            Self::WorkspaceMismatch => "workspace_mismatch",
            Self::ConfidenceTooLow => "confidence_too_low",
            Self::UnsupportedTargetKind => "unsupported_target_kind",
        }
    }
}

/// Opaque target identity carried on a remap packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemapTarget {
    /// Target kind being remapped.
    pub target_kind: RemapTargetKind,
    /// Opaque canonical target ref.
    pub target_ref: String,
    /// Workspace authority that owns the target.
    pub workspace_id_ref: String,
    /// Root ref that owns the target, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_id_ref: Option<String>,
    /// Short root or repo label safe for navigation chrome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_label: Option<String>,
    /// Stable result key or identity key when this target came from search.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stable_result_key: Option<String>,
    /// Opaque path identity, when the target points into a file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path_identity_ref: Option<String>,
    /// Opaque symbol anchor, when the target points at a symbol.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub symbol_anchor_ref: Option<String>,
    /// Opaque graph node, when graph or structural evidence exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub graph_node_ref: Option<String>,
    /// Snapshot, commit, or graph epoch reference used by the resolver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_ref: Option<String>,
}

impl RemapTarget {
    fn validate(&self, field: RemapTargetField) -> Result<(), DeepLinkRemapPacketError> {
        if self.target_ref.trim().is_empty() {
            return Err(DeepLinkRemapPacketError::EmptyTargetRef(field));
        }
        if self.workspace_id_ref.trim().is_empty() {
            return Err(DeepLinkRemapPacketError::EmptyTargetWorkspace(field));
        }
        Ok(())
    }
}

/// Scope/workset identity captured by a remap packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemapScopePacket {
    /// Workspace that minted the original deep link.
    pub originating_workspace_id_ref: String,
    /// Workspace authority active when the packet was emitted.
    pub active_workspace_id_ref: String,
    /// Stable scope/workset/slice identity active at resolution time.
    pub stable_scope_id_ref: String,
    /// Scope class active at resolution time.
    pub scope_class: ScopeClass,
    /// Full or sparse materialization mode for the active scope.
    pub scope_mode: ScopeMode,
    /// Workset id when a workset or sparse slice is active.
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
    pub destination_workspace_id_ref: Option<String>,
    /// Root ref that owns the destination target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_root_id_ref: Option<String>,
    /// Short destination root or repo label safe for navigation chrome.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_root_label: Option<String>,
    /// True when the destination belongs to the active scope.
    pub destination_in_active_scope: bool,
}

impl RemapScopePacket {
    /// Returns true when the resolved target crosses root, workset, sparse, or
    /// workspace boundaries visible to the user.
    pub fn crosses_visible_boundary(&self) -> bool {
        let has_destination_identity =
            self.destination_workspace_id_ref.is_some() || self.destination_root_id_ref.is_some();
        if !has_destination_identity {
            return false;
        }
        if !self.destination_in_active_scope {
            return true;
        }
        let workspace_crossed = self
            .destination_workspace_id_ref
            .as_deref()
            .is_some_and(|target| target != self.active_workspace_id_ref);
        let root_crossed = self
            .destination_root_id_ref
            .as_deref()
            .is_some_and(|target| !self.active_root_refs.iter().any(|root| root == target));
        workspace_crossed || root_crossed
    }

    fn validate(&self) -> Result<(), DeepLinkRemapPacketError> {
        if self.originating_workspace_id_ref.trim().is_empty() {
            return Err(DeepLinkRemapPacketError::EmptyOriginatingWorkspace);
        }
        if self.active_workspace_id_ref.trim().is_empty() {
            return Err(DeepLinkRemapPacketError::EmptyActiveWorkspace);
        }
        if self.stable_scope_id_ref.trim().is_empty() {
            return Err(DeepLinkRemapPacketError::EmptyStableScopeId);
        }
        if self.crosses_visible_boundary()
            && (self
                .destination_root_id_ref
                .as_deref()
                .map_or(true, str::is_empty)
                || self
                    .destination_root_label
                    .as_deref()
                    .map_or(true, str::is_empty))
        {
            return Err(DeepLinkRemapPacketError::MissingDestinationRootIdentity);
        }
        Ok(())
    }
}

/// Confidence and evidence attached to a remap packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RemapConfidence {
    /// Confidence tier assigned by the resolver.
    pub confidence_class: RemapConfidenceClass,
    /// Optional score bucket from 0 to 100.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence_score: Option<u8>,
    /// Evidence families that contributed to the decision.
    #[serde(default)]
    pub evidence_classes: Vec<RemapEvidenceClass>,
}

impl RemapConfidence {
    fn validate(&self) -> Result<(), DeepLinkRemapPacketError> {
        if self.confidence_score.is_some_and(|score| score > 100) {
            return Err(DeepLinkRemapPacketError::ConfidenceScoreOutOfRange);
        }
        Ok(())
    }
}

/// Search-owned remap packet for a drifted deep link or navigation target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeepLinkRemapPacket {
    /// Stable record-kind tag.
    pub record_kind: DeepLinkRemapRecordKind,
    /// Integer schema version for this record.
    pub deep_link_remap_packet_schema_version: u32,
    /// Stable remap packet id.
    pub remap_packet_id: String,
    /// Search deep-link id being resolved.
    pub deep_link_id_ref: String,
    /// Deep-link drift state produced by resolution.
    pub deep_link_drift_state: DeepLinkDriftState,
    /// Remap outcome for this packet.
    pub outcome_class: DeepLinkRemapOutcome,
    /// Original target captured by the bookmark, history entry, or deep link.
    pub old_target: RemapTarget,
    /// Current target candidate when resolution produced one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_target: Option<RemapTarget>,
    /// Active scope/workset identity and destination boundary state.
    pub scope: RemapScopePacket,
    /// Confidence and evidence for the remap decision.
    pub confidence: RemapConfidence,
    /// Predecessor deep-link or packet refs joined into this remap.
    #[serde(default)]
    pub remap_chain_refs: Vec<String>,
    /// Destination identity projections for peek, preview, split, open, and back paths.
    #[serde(default)]
    pub destination_visibility: Vec<NavigationDestinationVisibility>,
    /// Bounded actions the surface may offer next.
    #[serde(default)]
    pub recovery_actions: Vec<NavigationRecoveryAction>,
    /// Explicit failure reason when the packet does not directly open a target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub failure_reason: Option<RemapFailureReason>,
    /// Monotonic or fixture timestamp for export parity.
    pub emitted_at: String,
}

impl DeepLinkRemapPacket {
    /// Validates the remap packet against search/navigation truth invariants.
    pub fn validate(&self) -> Result<(), DeepLinkRemapPacketError> {
        if self.deep_link_remap_packet_schema_version != DEEP_LINK_REMAP_PACKET_SCHEMA_VERSION {
            return Err(DeepLinkRemapPacketError::SchemaVersionMismatch(
                self.deep_link_remap_packet_schema_version,
            ));
        }
        if self.remap_packet_id.trim().is_empty() {
            return Err(DeepLinkRemapPacketError::EmptyRemapPacketId);
        }
        if self.deep_link_id_ref.trim().is_empty() {
            return Err(DeepLinkRemapPacketError::EmptyDeepLinkId);
        }
        self.old_target.validate(RemapTargetField::OldTarget)?;
        if let Some(new_target) = self.new_target.as_ref() {
            new_target.validate(RemapTargetField::NewTarget)?;
            self.validate_scope_matches_target(new_target)?;
        }
        self.scope.validate()?;
        self.confidence.validate()?;

        match self.outcome_class {
            DeepLinkRemapOutcome::ResolvedExact => {
                self.require_new_target()?;
                if self.deep_link_drift_state != DeepLinkDriftState::ResolvedExact {
                    return Err(DeepLinkRemapPacketError::OutcomeDriftMismatch);
                }
                if self.failure_reason.is_some() {
                    return Err(DeepLinkRemapPacketError::UnexpectedFailureReason);
                }
            }
            DeepLinkRemapOutcome::Remapped => {
                self.require_new_target()?;
                if self.deep_link_drift_state != DeepLinkDriftState::ResolvedRemapped {
                    return Err(DeepLinkRemapPacketError::OutcomeDriftMismatch);
                }
                if self.remap_chain_refs.is_empty() {
                    return Err(DeepLinkRemapPacketError::MissingRemapChain);
                }
                if !self.confidence.confidence_class.permits_direct_remap() {
                    return Err(DeepLinkRemapPacketError::ConfidenceTooLowForDirectRemap);
                }
                self.require_action(NavigationRecoveryAction::OpenRemappedTarget)?;
                if self.failure_reason.is_some() {
                    return Err(DeepLinkRemapPacketError::UnexpectedFailureReason);
                }
            }
            DeepLinkRemapOutcome::RecoverablePlaceholder => {
                self.require_new_target()?;
                if self.deep_link_drift_state.is_resolved() {
                    return Err(DeepLinkRemapPacketError::OutcomeDriftMismatch);
                }
                if !self
                    .recovery_actions
                    .iter()
                    .any(|action| action.is_bounded_missing_target_action())
                {
                    return Err(DeepLinkRemapPacketError::MissingBoundedRecoveryAction);
                }
                if self.failure_reason.is_none() {
                    return Err(DeepLinkRemapPacketError::MissingFailureReason);
                }
            }
            DeepLinkRemapOutcome::FailedExplicitReason => {
                if self.new_target.is_some() {
                    return Err(DeepLinkRemapPacketError::UnexpectedNewTarget);
                }
                if self.failure_reason.is_none() {
                    return Err(DeepLinkRemapPacketError::MissingFailureReason);
                }
            }
        }

        if self.scope.crosses_visible_boundary() {
            validate_destination_visibility(&self.destination_visibility)?;
        }
        Ok(())
    }

    /// Returns stable recovery-action tokens.
    pub fn recovery_action_tokens(&self) -> Vec<&'static str> {
        self.recovery_actions
            .iter()
            .map(|action| action.as_str())
            .collect()
    }

    fn require_new_target(&self) -> Result<(), DeepLinkRemapPacketError> {
        if self.new_target.is_none() {
            return Err(DeepLinkRemapPacketError::MissingNewTarget);
        }
        Ok(())
    }

    fn require_action(
        &self,
        action: NavigationRecoveryAction,
    ) -> Result<(), DeepLinkRemapPacketError> {
        if self.recovery_actions.contains(&action) {
            Ok(())
        } else {
            Err(DeepLinkRemapPacketError::MissingRecoveryAction(action))
        }
    }

    fn validate_scope_matches_target(
        &self,
        target: &RemapTarget,
    ) -> Result<(), DeepLinkRemapPacketError> {
        if self
            .scope
            .destination_workspace_id_ref
            .as_deref()
            .is_some_and(|workspace| workspace != target.workspace_id_ref)
        {
            return Err(DeepLinkRemapPacketError::DestinationWorkspaceMismatch);
        }
        if let (Some(scope_root), Some(target_root)) = (
            self.scope.destination_root_id_ref.as_deref(),
            target.root_id_ref.as_deref(),
        ) {
            if scope_root != target_root {
                return Err(DeepLinkRemapPacketError::DestinationRootMismatch);
            }
        }
        Ok(())
    }
}

fn validate_destination_visibility(
    visibility: &[NavigationDestinationVisibility],
) -> Result<(), DeepLinkRemapPacketError> {
    const REQUIRED: [NavigationSurfaceClass; 5] = [
        NavigationSurfaceClass::Peek,
        NavigationSurfaceClass::Preview,
        NavigationSurfaceClass::Split,
        NavigationSurfaceClass::OpenInNewPane,
        NavigationSurfaceClass::BackNavigation,
    ];

    for surface in REQUIRED {
        let Some(row) = visibility.iter().find(|row| row.surface_class == surface) else {
            return Err(DeepLinkRemapPacketError::MissingDestinationVisibility(
                surface,
            ));
        };
        if row.target_root_id_ref.trim().is_empty() || row.target_root_label.trim().is_empty() {
            return Err(DeepLinkRemapPacketError::IncompleteDestinationVisibility(
                surface,
            ));
        }
        if !row.destination_repo_visible {
            return Err(DeepLinkRemapPacketError::DestinationRepoNotVisible(surface));
        }
    }
    Ok(())
}

/// Target field referenced by a remap packet validation error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemapTargetField {
    /// The original target captured by the link.
    OldTarget,
    /// The resolved target candidate.
    NewTarget,
}

/// Validation errors for [`DeepLinkRemapPacket`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeepLinkRemapPacketError {
    /// The packet uses an unsupported schema version.
    SchemaVersionMismatch(u32),
    /// `remap_packet_id` is empty.
    EmptyRemapPacketId,
    /// `deep_link_id_ref` is empty.
    EmptyDeepLinkId,
    /// Target ref is empty.
    EmptyTargetRef(RemapTargetField),
    /// Target workspace is empty.
    EmptyTargetWorkspace(RemapTargetField),
    /// Originating workspace id is empty.
    EmptyOriginatingWorkspace,
    /// Active workspace id is empty.
    EmptyActiveWorkspace,
    /// Stable scope id is empty.
    EmptyStableScopeId,
    /// Cross-boundary packet lacks destination root identity.
    MissingDestinationRootIdentity,
    /// Confidence score is outside 0..=100.
    ConfidenceScoreOutOfRange,
    /// Outcome and drift state are incompatible.
    OutcomeDriftMismatch,
    /// A target-bearing outcome lacks `new_target`.
    MissingNewTarget,
    /// A failed outcome unexpectedly carries `new_target`.
    UnexpectedNewTarget,
    /// A remapped packet lacks predecessor refs.
    MissingRemapChain,
    /// The remap confidence floor does not permit direct opening.
    ConfidenceTooLowForDirectRemap,
    /// A required recovery action is absent.
    MissingRecoveryAction(NavigationRecoveryAction),
    /// A recoverable or failed packet lacks an explicit failure reason.
    MissingFailureReason,
    /// Directly openable remap carries a failure reason.
    UnexpectedFailureReason,
    /// Recoverable placeholder lacks bounded next actions.
    MissingBoundedRecoveryAction,
    /// Destination workspace does not match the new target.
    DestinationWorkspaceMismatch,
    /// Destination root does not match the new target.
    DestinationRootMismatch,
    /// Cross-root or cross-workspace navigation lacks a required surface row.
    MissingDestinationVisibility(NavigationSurfaceClass),
    /// A required destination-visibility row lacks root identity or label.
    IncompleteDestinationVisibility(NavigationSurfaceClass),
    /// A required destination surface does not visibly show the destination root.
    DestinationRepoNotVisible(NavigationSurfaceClass),
}

impl std::fmt::Display for DeepLinkRemapPacketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SchemaVersionMismatch(version) => write!(
                f,
                "unsupported deep_link_remap_packet_schema_version {version}; expected {DEEP_LINK_REMAP_PACKET_SCHEMA_VERSION}"
            ),
            Self::EmptyRemapPacketId => write!(f, "remap_packet_id must not be empty"),
            Self::EmptyDeepLinkId => write!(f, "deep_link_id_ref must not be empty"),
            Self::EmptyTargetRef(field) => {
                write!(f, "{} target_ref must not be empty", field.label())
            }
            Self::EmptyTargetWorkspace(field) => {
                write!(f, "{} workspace_id_ref must not be empty", field.label())
            }
            Self::EmptyOriginatingWorkspace => {
                write!(f, "originating_workspace_id_ref must not be empty")
            }
            Self::EmptyActiveWorkspace => write!(f, "active_workspace_id_ref must not be empty"),
            Self::EmptyStableScopeId => write!(f, "stable_scope_id_ref must not be empty"),
            Self::MissingDestinationRootIdentity => write!(
                f,
                "cross-boundary remap requires destination root id and label"
            ),
            Self::ConfidenceScoreOutOfRange => {
                write!(f, "confidence_score must be between 0 and 100")
            }
            Self::OutcomeDriftMismatch => {
                write!(f, "outcome_class does not match deep_link_drift_state")
            }
            Self::MissingNewTarget => write!(f, "outcome requires a new_target"),
            Self::UnexpectedNewTarget => write!(f, "failed outcome must not carry new_target"),
            Self::MissingRemapChain => write!(f, "remapped outcome requires remap_chain_refs"),
            Self::ConfidenceTooLowForDirectRemap => write!(
                f,
                "confidence_class does not permit direct remapped opening"
            ),
            Self::MissingRecoveryAction(action) => {
                write!(f, "missing required recovery action {}", action.as_str())
            }
            Self::MissingFailureReason => {
                write!(f, "degraded or failed outcome requires failure_reason")
            }
            Self::UnexpectedFailureReason => {
                write!(f, "directly openable outcome must not carry failure_reason")
            }
            Self::MissingBoundedRecoveryAction => write!(
                f,
                "recoverable placeholder requires a bounded recovery action"
            ),
            Self::DestinationWorkspaceMismatch => {
                write!(f, "scope destination workspace does not match new target")
            }
            Self::DestinationRootMismatch => {
                write!(f, "scope destination root does not match new target")
            }
            Self::MissingDestinationVisibility(surface) => write!(
                f,
                "cross-boundary remap requires destination visibility on {}",
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

impl std::error::Error for DeepLinkRemapPacketError {}

impl RemapTargetField {
    const fn label(self) -> &'static str {
        match self {
            Self::OldTarget => "old_target",
            Self::NewTarget => "new_target",
        }
    }
}
