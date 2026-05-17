//! Beta interaction-integrity packet for dense shell collection surfaces.
//!
//! The packet in this module is the shell-side join between the existing
//! collection contract, responsive shell layout, focus-return rules, identity
//! cues, and support-export truth. It does not render a widget. It provides one
//! reviewable record family that launch-critical dense surfaces can emit before
//! destructive, remote, provider-owned, or publish-capable actions continue.

use std::collections::{BTreeMap, BTreeSet};

use aureline_search::{
    BatchActionClass, BatchExecutionOriginClass, BatchMemberDisposition, BatchReviewMember,
    BatchReviewSheet, CollectionCountStatus, CollectionScopeCounters, CollectionSurfaceFamily,
    SelectionScopeClass, StableCollectionItemRef,
};
use serde::{Deserialize, Serialize};

use crate::app_frame::desktop_frame::DesktopFrame;

/// Schema version exported by interaction-integrity beta records.
pub const INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by shell, support, and replay fixtures.
pub const INTERACTION_INTEGRITY_SHARED_CONTRACT_REF: &str = "shell:interaction_integrity_beta:v1";

/// Stable record kind for [`InteractionIntegrityBetaPacket`] payloads.
pub const INTERACTION_INTEGRITY_PACKET_RECORD_KIND: &str =
    "shell_interaction_integrity_beta_packet_record";

/// Stable record kind for [`ObjectInteractionStateRecord`] payloads.
pub const OBJECT_INTERACTION_STATE_RECORD_KIND: &str = "object_interaction_state_record";

/// Stable record kind for [`BatchScopeTruthRecord`] payloads.
pub const BATCH_SCOPE_TRUTH_RECORD_KIND: &str = "batch_scope_truth_record";

/// Stable record kind for [`ShellIdentityCueRecord`] payloads.
pub const SHELL_IDENTITY_CUE_RECORD_KIND: &str = "shell_identity_cue_record";

/// Stable record kind for [`FocusReturnRuleRecord`] payloads.
pub const FOCUS_RETURN_RULE_RECORD_KIND: &str = "focus_return_rule_record";

/// Stable record kind for [`VocabularyParityRecord`] payloads.
pub const VOCABULARY_PARITY_RECORD_KIND: &str = "collection_vocabulary_parity_record";

/// Stable record kind for [`InteractionIntegritySupportExport`] payloads.
pub const INTERACTION_INTEGRITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "interaction_integrity_support_export_record";

/// Stable record kind for [`InteractionIntegrityReplayFixture`] payloads.
pub const INTERACTION_INTEGRITY_REPLAY_FIXTURE_RECORD_KIND: &str =
    "interaction_integrity_replay_fixture_record";

/// Launch-critical collection surface covered by the beta packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimedCollectionSurfaceClass {
    /// Search and quick-open result lists.
    SearchResultsList,
    /// Review queue or pull-request review grid.
    ReviewQueueGrid,
    /// Git change tree and mutation-review hierarchy.
    GitChangeTree,
    /// Extension, package, or marketplace inventory grid.
    MarketplaceInventoryGrid,
    /// Activity-center dense list or virtualized work history.
    ActivityCenterList,
}

impl ClaimedCollectionSurfaceClass {
    /// Stable token used by fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchResultsList => "search_results_list",
            Self::ReviewQueueGrid => "review_queue_grid",
            Self::GitChangeTree => "git_change_tree",
            Self::MarketplaceInventoryGrid => "marketplace_inventory_grid",
            Self::ActivityCenterList => "activity_center_list",
        }
    }
}

/// Geometry family for the covered collection surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionGeometryClass {
    /// Dense row list with virtual or paged backing.
    DenseList,
    /// Hierarchical tree with stable node identity.
    Tree,
    /// Columnar grid or table.
    Grid,
    /// Review surface whose rows are virtualized or provider-backed.
    VirtualizedReviewSurface,
}

impl CollectionGeometryClass {
    /// Stable token used by fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DenseList => "dense_list",
            Self::Tree => "tree",
            Self::Grid => "grid",
            Self::VirtualizedReviewSurface => "virtualized_review_surface",
        }
    }
}

/// Shared interaction token every covered dense surface must understand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectInteractionToken {
    /// Keyboard focus is on this object.
    Focused,
    /// The object drives preview, detail, or status context.
    Active,
    /// The object belongs to the durable batch selection.
    Selected,
    /// The object is awaiting refresh, provider answer, or prior work.
    Pending,
    /// The object is visible but unavailable for the proposed action.
    Disabled,
    /// The object is outside the current filter result.
    Filtered,
    /// The object is hidden by policy, workset, viewport, or compact fallback.
    Hidden,
    /// The object is blocked by policy, authority, state, or safety posture.
    Blocked,
}

impl ObjectInteractionToken {
    /// Stable token used in UI, keyboard help, accessibility, and support export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Focused => "focused",
            Self::Active => "active",
            Self::Selected => "selected",
            Self::Pending => "pending",
            Self::Disabled => "disabled",
            Self::Filtered => "filtered",
            Self::Hidden => "hidden",
            Self::Blocked => "blocked",
        }
    }
}

/// Role of a shell identity cue that must survive fallback and window changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityCueRole {
    /// Breadcrumb or route ancestry cue.
    Breadcrumb,
    /// Status-bar or status-strip cue.
    StatusItem,
    /// Editor or review tab identity cue.
    TabIdentity,
    /// Inspector header cue.
    InspectorHeader,
    /// Terminal header cue.
    TerminalHeader,
}

impl IdentityCueRole {
    /// Stable token used in replay fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Breadcrumb => "breadcrumb",
            Self::StatusItem => "status_item",
            Self::TabIdentity => "tab_identity",
            Self::InspectorHeader => "inspector_header",
            Self::TerminalHeader => "terminal_header",
        }
    }
}

/// Triggering surface that owns a focus-return rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FocusReturnTriggerClass {
    /// Modal dialog confirm, cancel, or close.
    ModalDialogDismiss,
    /// Sheet dismiss, cancel, or apply-without-navigation.
    SheetDismiss,
    /// Approval prompt complete, denial, or limited-mode choice.
    ApprovalPromptResolve,
    /// Activity-center or durable-row jump out and back.
    ActivityCenterJump,
    /// Pane close, split merge, or responsive fallback replacement.
    PaneCloseOrSplitMerge,
}

impl FocusReturnTriggerClass {
    /// Stable token used in replay fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModalDialogDismiss => "modal_dialog_dismiss",
            Self::SheetDismiss => "sheet_dismiss",
            Self::ApprovalPromptResolve => "approval_prompt_resolve",
            Self::ActivityCenterJump => "activity_center_jump",
            Self::PaneCloseOrSplitMerge => "pane_close_or_split_merge",
        }
    }
}

/// Replay drill family covered by the fixture corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayDrillClass {
    /// Virtualized selection and hidden-selected disclosure.
    VirtualizedSelection,
    /// Batch review and resulting target-id proof.
    BatchScopeReview,
    /// Resize, split, detach, and multi-window fallback.
    ResponsiveIdentityCue,
    /// Dialog, sheet, prompt, and activity-center focus return.
    FocusReturn,
    /// Redaction-safe support export.
    SupportExport,
}

impl ReplayDrillClass {
    /// Stable token used in fixture manifests.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VirtualizedSelection => "virtualized_selection",
            Self::BatchScopeReview => "batch_scope_review",
            Self::ResponsiveIdentityCue => "responsive_identity_cue",
            Self::FocusReturn => "focus_return",
            Self::SupportExport => "support_export",
        }
    }
}

/// Shared object state for one row, node, grid item, or virtualized review object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObjectInteractionStateRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable surface id that owns this object state.
    pub surface_id_ref: String,
    /// Surface class that owns this object state.
    pub surface_class: ClaimedCollectionSurfaceClass,
    /// Geometry class for the owning surface.
    pub geometry_class: CollectionGeometryClass,
    /// Stable object id used for focus, selection, review, and support export.
    pub object_id_ref: String,
    /// Redaction-safe label for local review surfaces.
    pub object_label: String,
    /// Stable identity cue that appears in compact UI and replay fixtures.
    pub identity_cue_label: String,
    /// True when keyboard focus is currently on this object.
    pub focused: bool,
    /// True when this object drives preview, detail, or status context.
    pub active: bool,
    /// True when this object is in the durable batch selection.
    pub selected: bool,
    /// True when the object awaits refresh, provider answer, or earlier work.
    pub pending: bool,
    /// True when the object is visible but unavailable for the proposed action.
    pub disabled: bool,
    /// True when the object is outside the active filter result.
    pub filtered: bool,
    /// True when the object is hidden by policy, workset, viewport, or fallback.
    pub hidden: bool,
    /// True when policy, authority, state, or safety blocks the object.
    pub blocked: bool,
    /// Reason label for disabled state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_label: Option<String>,
    /// Reason label for filtered state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filtered_reason_label: Option<String>,
    /// Reason label for hidden state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_reason_label: Option<String>,
    /// Reason label for blocked state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_reason_label: Option<String>,
    /// Computed state tokens for this object.
    pub state_tokens: Vec<ObjectInteractionToken>,
    /// Tokens exposed by UI copy and chips.
    pub ui_state_tokens: Vec<ObjectInteractionToken>,
    /// Tokens exposed by keyboard help.
    pub keyboard_help_state_tokens: Vec<ObjectInteractionToken>,
    /// Tokens exposed to accessibility narration.
    pub accessibility_state_tokens: Vec<ObjectInteractionToken>,
    /// Tokens exposed to support export.
    pub support_export_state_tokens: Vec<ObjectInteractionToken>,
    /// True when activation acts on the active/current object without changing selection.
    pub activation_preserves_selection: bool,
    /// True when virtualization, sort, and filtering preserve the stable object id.
    pub identity_survives_virtualization: bool,
}

impl ObjectInteractionStateRecord {
    fn from_seed(seed: SeedObjectState) -> Self {
        let state_tokens = object_state_tokens(
            seed.focused,
            seed.active,
            seed.selected,
            seed.pending,
            seed.disabled,
            seed.filtered,
            seed.hidden,
            seed.blocked,
        );
        Self {
            record_kind: OBJECT_INTERACTION_STATE_RECORD_KIND.to_owned(),
            schema_version: INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION,
            shared_contract_ref: INTERACTION_INTEGRITY_SHARED_CONTRACT_REF.to_owned(),
            surface_id_ref: seed.surface_id_ref.to_owned(),
            surface_class: seed.surface_class,
            geometry_class: seed.geometry_class,
            object_id_ref: seed.object_id_ref.to_owned(),
            object_label: seed.object_label.to_owned(),
            identity_cue_label: seed.identity_cue_label.to_owned(),
            focused: seed.focused,
            active: seed.active,
            selected: seed.selected,
            pending: seed.pending,
            disabled: seed.disabled,
            filtered: seed.filtered,
            hidden: seed.hidden,
            blocked: seed.blocked,
            disabled_reason_label: seed.disabled_reason_label.map(str::to_owned),
            filtered_reason_label: seed.filtered_reason_label.map(str::to_owned),
            hidden_reason_label: seed.hidden_reason_label.map(str::to_owned),
            blocked_reason_label: seed.blocked_reason_label.map(str::to_owned),
            ui_state_tokens: state_tokens.clone(),
            keyboard_help_state_tokens: state_tokens.clone(),
            accessibility_state_tokens: state_tokens.clone(),
            support_export_state_tokens: state_tokens.clone(),
            state_tokens,
            activation_preserves_selection: true,
            identity_survives_virtualization: true,
        }
    }

    /// True when this selected object can be included in the next batch target set.
    pub const fn eligible_for_batch_target(&self) -> bool {
        self.selected
            && !self.pending
            && !self.disabled
            && !self.filtered
            && !self.hidden
            && !self.blocked
    }
}

/// Batch-scope truth wrapped around the shared collection batch-review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchScopeTruthRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable batch-scope id.
    pub batch_scope_id: String,
    /// Surface that owns this batch review.
    pub surface_id_ref: String,
    /// Human-readable action label.
    pub action_label: String,
    /// True when the action is destructive, remote, provider-owned, publish-capable, or export-bearing.
    pub action_is_destructive_or_publish_capable: bool,
    /// Shared collection batch-review sheet.
    pub batch_review_sheet: BatchReviewSheet,
    /// Count of selected objects that will be acted on.
    pub included_count: u64,
    /// Count of selected or visible objects explicitly excluded.
    pub excluded_count: u64,
    /// Count of selected objects blocked before commit.
    pub blocked_count: u64,
    /// Count of selected objects filtered out of scope.
    pub filtered_count: u64,
    /// Count of selected objects hidden by policy, workset, viewport, or fallback.
    pub hidden_count: u64,
    /// Count of selected objects disabled for this action.
    pub disabled_count: u64,
    /// Count of selected objects pending refresh or prior work.
    pub pending_count: u64,
    /// Stable ids that will actually be targeted if the action continues.
    pub resulting_target_id_refs: Vec<String>,
    /// Stable ids excluded from the action.
    pub excluded_item_id_refs: Vec<String>,
    /// Stable ids blocked from the action.
    pub blocked_item_id_refs: Vec<String>,
    /// Stable ids filtered out of the action.
    pub filtered_item_id_refs: Vec<String>,
    /// Stable ids hidden from the visible action population.
    pub hidden_item_id_refs: Vec<String>,
    /// Stable ids disabled for this action.
    pub disabled_item_id_refs: Vec<String>,
    /// Stable ids pending refresh or prior work.
    pub pending_item_id_refs: Vec<String>,
    /// Stable snapshot or query basis reviewed by the sheet.
    pub basis_snapshot_ref: String,
    /// True when material drift invalidates the commit path.
    pub material_scope_drift_invalidates_commit: bool,
    /// True when the continue path is enabled for included safe targets.
    pub commit_path_enabled: bool,
    /// Compact count label rendered by sheets and accessibility summaries.
    pub batch_review_summary_label: String,
    /// True when hidden or filtered objects are mechanically excluded from targets.
    pub hidden_or_filtered_silently_included_denied: bool,
    /// True when every target is represented by a stable object identity.
    pub stable_object_identity_visible: bool,
}

/// Shell identity cue preserved across responsive fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellIdentityCueRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable cue id.
    pub cue_id: String,
    /// Surface that owns this cue.
    pub surface_id_ref: String,
    /// Cue role.
    pub cue_role: IdentityCueRole,
    /// Stable canonical object carried by the cue.
    pub canonical_object_id_ref: String,
    /// Window id before fallback or detach.
    pub window_id_ref: String,
    /// Shell zone id before fallback.
    pub source_zone_id_ref: String,
    /// Slot id before fallback.
    pub source_slot_id_ref: String,
    /// Slot id after resize fallback.
    pub resize_fallback_slot_id_ref: String,
    /// Slot id after split fallback.
    pub split_fallback_slot_id_ref: String,
    /// Slot id after detach fallback.
    pub detach_fallback_slot_id_ref: String,
    /// Slot id after multi-window fallback.
    pub multi_window_fallback_slot_id_ref: String,
    /// Stable display label for the cue.
    pub display_label: String,
    /// Responsive fallback mode tokens observed by the shell.
    pub responsive_fallback_modes: Vec<String>,
    /// True when the cue keeps the same canonical identity after resize.
    pub stable_under_resize: bool,
    /// True when the cue keeps the same canonical identity after split.
    pub stable_under_split: bool,
    /// True when the cue keeps the same canonical identity after detach.
    pub stable_under_detach: bool,
    /// True when the cue keeps the same canonical identity in multi-window fallback.
    pub stable_under_multi_window_fallback: bool,
}

/// Focus-return rule for dialogs, sheets, prompts, jumps, and split fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FocusReturnRuleRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable rule id.
    pub rule_id: String,
    /// Triggering surface class.
    pub trigger_class: FocusReturnTriggerClass,
    /// Surface that opened the dialog, sheet, prompt, or jump.
    pub origin_surface_id_ref: String,
    /// Original object or owner that should receive focus again.
    pub origin_object_id_ref: String,
    /// Invoking control that opened the transient surface.
    pub invoking_control_id_ref: String,
    /// Preferred post-close focus target.
    pub expected_return_target_id_ref: String,
    /// Fallback focus target when the original object vanished.
    pub fallback_return_target_id_ref: String,
    /// Screen-reader announcement emitted on return.
    pub screen_reader_announcement: String,
    /// True when the rule is visible in UI or support/replay records.
    pub focus_return_explicit: bool,
    /// True when focus never returns to a hidden or offscreen pane.
    pub never_returns_to_hidden_or_offscreen: bool,
    /// True when selection or cursor state is preserved.
    pub preserves_selection_or_cursor_state: bool,
}

/// Vocabulary parity record for a launch-critical collection surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VocabularyParityRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Surface covered by this parity row.
    pub surface_id_ref: String,
    /// Surface class covered by this parity row.
    pub surface_class: ClaimedCollectionSurfaceClass,
    /// Tokens available in UI copy, chips, and sheets.
    pub ui_tokens: Vec<ObjectInteractionToken>,
    /// Tokens available in keyboard help.
    pub keyboard_help_tokens: Vec<ObjectInteractionToken>,
    /// Tokens available in accessibility narration.
    pub accessibility_tokens: Vec<ObjectInteractionToken>,
    /// Tokens available in support export.
    pub support_export_tokens: Vec<ObjectInteractionToken>,
    /// True when every projection uses the same token set.
    pub token_sets_match: bool,
}

/// Redaction-safe support export for selection, filter, and target truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionIntegritySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable support export id.
    pub support_export_id: String,
    /// Stable packet id this export summarizes.
    pub packet_id_ref: String,
    /// Selected object ids across covered surfaces.
    pub selected_scope_id_refs: Vec<String>,
    /// Selected object ids filtered out of scope.
    pub filtered_scope_id_refs: Vec<String>,
    /// Selected object ids hidden from visible scope.
    pub hidden_scope_id_refs: Vec<String>,
    /// Selected object ids blocked from the reviewed action.
    pub blocked_scope_id_refs: Vec<String>,
    /// Stable object ids that would be targeted by reviewed actions.
    pub resulting_target_id_refs: Vec<String>,
    /// Reviewed batch scope ids included in the export.
    pub reviewed_batch_scope_refs: Vec<String>,
    /// State vocabulary available to support export consumers.
    pub support_export_state_tokens: Vec<ObjectInteractionToken>,
    /// Redaction class for this export.
    pub redaction_class: String,
    /// True when raw filter literals are absent.
    pub raw_filter_literals_included: bool,
    /// True when raw object payloads are absent.
    pub raw_object_payloads_included: bool,
    /// True when raw private labels are absent.
    pub raw_private_labels_included: bool,
}

/// Replay fixture row that binds a drill to concrete fixture files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionIntegrityReplayFixture {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Drill class.
    pub drill_class: ReplayDrillClass,
    /// Surfaces exercised by the drill.
    pub covered_surface_refs: Vec<String>,
    /// Fixture paths used by replay and support review.
    pub fixture_refs: Vec<String>,
    /// Expected support export id after replay.
    pub expected_support_export_ref: String,
    /// Command that reproduces the packet without a live UI.
    pub replay_command: String,
}

/// Aggregate summary for the interaction-integrity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionIntegrityBetaSummary {
    /// Number of object-state records.
    pub object_state_count: usize,
    /// Number of batch-scope truth records.
    pub batch_scope_count: usize,
    /// Number of shell identity cue records.
    pub identity_cue_count: usize,
    /// Number of focus-return rules.
    pub focus_return_rule_count: usize,
    /// Number of vocabulary parity rows.
    pub vocabulary_parity_count: usize,
    /// Number of replay fixtures.
    pub replay_fixture_count: usize,
    /// Surface classes present in the packet.
    pub surface_classes_present: Vec<ClaimedCollectionSurfaceClass>,
    /// Geometry classes present in the packet.
    pub geometry_classes_present: Vec<CollectionGeometryClass>,
    /// Object state tokens present in the packet.
    pub state_tokens_present: Vec<ObjectInteractionToken>,
    /// Number of reviewed batch targets.
    pub resulting_target_count: usize,
    /// Number of selected hidden or filtered ids excluded from targets.
    pub hidden_or_filtered_excluded_count: usize,
    /// Number of identity cues proving responsive fallback.
    pub responsive_identity_cue_count: usize,
}

impl InteractionIntegrityBetaSummary {
    fn from_parts(
        object_states: &[ObjectInteractionStateRecord],
        batch_scope_reviews: &[BatchScopeTruthRecord],
        identity_cues: &[ShellIdentityCueRecord],
        focus_return_rules: &[FocusReturnRuleRecord],
        vocabulary_parity: &[VocabularyParityRecord],
        replay_fixtures: &[InteractionIntegrityReplayFixture],
    ) -> Self {
        let mut surface_classes = BTreeSet::new();
        let mut geometry_classes = BTreeSet::new();
        let mut state_tokens = BTreeSet::new();
        for row in object_states {
            surface_classes.insert(row.surface_class);
            geometry_classes.insert(row.geometry_class);
            for token in &row.state_tokens {
                state_tokens.insert(*token);
            }
        }
        let resulting_target_count = batch_scope_reviews
            .iter()
            .map(|review| review.resulting_target_id_refs.len())
            .sum();
        let hidden_or_filtered_excluded_count = batch_scope_reviews
            .iter()
            .map(|review| review.hidden_item_id_refs.len() + review.filtered_item_id_refs.len())
            .sum();
        let responsive_identity_cue_count = identity_cues
            .iter()
            .filter(|cue| {
                cue.stable_under_resize
                    && cue.stable_under_split
                    && cue.stable_under_detach
                    && cue.stable_under_multi_window_fallback
            })
            .count();
        Self {
            object_state_count: object_states.len(),
            batch_scope_count: batch_scope_reviews.len(),
            identity_cue_count: identity_cues.len(),
            focus_return_rule_count: focus_return_rules.len(),
            vocabulary_parity_count: vocabulary_parity.len(),
            replay_fixture_count: replay_fixtures.len(),
            surface_classes_present: surface_classes.into_iter().collect(),
            geometry_classes_present: geometry_classes.into_iter().collect(),
            state_tokens_present: state_tokens.into_iter().collect(),
            resulting_target_count,
            hidden_or_filtered_excluded_count,
            responsive_identity_cue_count,
        }
    }
}

/// Top-level interaction-integrity beta packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionIntegrityBetaPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Generated-at timestamp or deterministic fixture clock.
    pub generated_at: String,
    /// Aggregate summary.
    pub summary: InteractionIntegrityBetaSummary,
    /// Shared object state records.
    pub object_states: Vec<ObjectInteractionStateRecord>,
    /// Batch-scope review records.
    pub batch_scope_reviews: Vec<BatchScopeTruthRecord>,
    /// Responsive identity cue records.
    pub identity_cues: Vec<ShellIdentityCueRecord>,
    /// Focus-return rule records.
    pub focus_return_rules: Vec<FocusReturnRuleRecord>,
    /// Vocabulary parity rows.
    pub vocabulary_parity: Vec<VocabularyParityRecord>,
    /// Redaction-safe support export.
    pub support_export: InteractionIntegritySupportExport,
    /// Replay fixture rows.
    pub replay_fixtures: Vec<InteractionIntegrityReplayFixture>,
}

/// Validation error for interaction-integrity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InteractionIntegrityValidationError {
    /// Packet metadata did not match the beta record family.
    PacketMetadataWrong {
        /// Reason label.
        reason: String,
    },
    /// Required surface, geometry, or state-token coverage is missing.
    CoverageMissing {
        /// Missing coverage item.
        missing: String,
    },
    /// One object-state row collapsed or omitted required state truth.
    ObjectStateInvalid {
        /// Object id.
        object_id_ref: String,
        /// Reason label.
        reason: String,
    },
    /// Batch review included a hidden, filtered, blocked, disabled, or pending object.
    UnsafeBatchTargetIncluded {
        /// Batch scope id.
        batch_scope_id: String,
        /// Object id.
        object_id_ref: String,
        /// Reason label.
        reason: String,
    },
    /// Batch review counts drifted from object state.
    BatchScopeDrift {
        /// Batch scope id.
        batch_scope_id: String,
        /// Reason label.
        reason: String,
    },
    /// Shell identity cue failed stable fallback requirements.
    IdentityCueUnstable {
        /// Cue id.
        cue_id: String,
        /// Reason label.
        reason: String,
    },
    /// Focus-return rule is incomplete or unsafe.
    FocusReturnInvalid {
        /// Rule id.
        rule_id: String,
        /// Reason label.
        reason: String,
    },
    /// Vocabulary parity row diverged across UI, keyboard, accessibility, or support export.
    VocabularyParityDrift {
        /// Surface id.
        surface_id_ref: String,
        /// Reason label.
        reason: String,
    },
    /// Support export leaked raw material or drifted from reviewed targets.
    SupportExportInvalid {
        /// Reason label.
        reason: String,
    },
    /// Replay fixture coverage is incomplete.
    ReplayFixtureInvalid {
        /// Fixture id.
        fixture_id: String,
        /// Reason label.
        reason: String,
    },
}

impl std::fmt::Display for InteractionIntegrityValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PacketMetadataWrong { reason } => {
                write!(f, "interaction-integrity packet metadata wrong: {reason}")
            }
            Self::CoverageMissing { missing } => {
                write!(f, "interaction-integrity coverage missing {missing}")
            }
            Self::ObjectStateInvalid {
                object_id_ref,
                reason,
            } => write!(f, "object state {object_id_ref} invalid: {reason}"),
            Self::UnsafeBatchTargetIncluded {
                batch_scope_id,
                object_id_ref,
                reason,
            } => write!(
                f,
                "batch {batch_scope_id} included unsafe target {object_id_ref}: {reason}"
            ),
            Self::BatchScopeDrift {
                batch_scope_id,
                reason,
            } => write!(f, "batch {batch_scope_id} scope drifted: {reason}"),
            Self::IdentityCueUnstable { cue_id, reason } => {
                write!(f, "identity cue {cue_id} unstable: {reason}")
            }
            Self::FocusReturnInvalid { rule_id, reason } => {
                write!(f, "focus-return rule {rule_id} invalid: {reason}")
            }
            Self::VocabularyParityDrift {
                surface_id_ref,
                reason,
            } => write!(
                f,
                "vocabulary parity for {surface_id_ref} drifted: {reason}"
            ),
            Self::SupportExportInvalid { reason } => {
                write!(f, "support export invalid: {reason}")
            }
            Self::ReplayFixtureInvalid { fixture_id, reason } => {
                write!(f, "replay fixture {fixture_id} invalid: {reason}")
            }
        }
    }
}

impl std::error::Error for InteractionIntegrityValidationError {}

/// Validates an interaction-integrity beta packet.
pub fn validate_interaction_integrity_beta_packet(
    packet: &InteractionIntegrityBetaPacket,
) -> Result<(), Vec<InteractionIntegrityValidationError>> {
    let mut errors = Vec::new();

    if packet.record_kind != INTERACTION_INTEGRITY_PACKET_RECORD_KIND {
        errors.push(InteractionIntegrityValidationError::PacketMetadataWrong {
            reason: "record kind mismatch".to_owned(),
        });
    }
    if packet.schema_version != INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION {
        errors.push(InteractionIntegrityValidationError::PacketMetadataWrong {
            reason: "schema version mismatch".to_owned(),
        });
    }
    if packet.shared_contract_ref != INTERACTION_INTEGRITY_SHARED_CONTRACT_REF {
        errors.push(InteractionIntegrityValidationError::PacketMetadataWrong {
            reason: "shared contract mismatch".to_owned(),
        });
    }

    let object_by_id: BTreeMap<&str, &ObjectInteractionStateRecord> = packet
        .object_states
        .iter()
        .map(|row| (row.object_id_ref.as_str(), row))
        .collect();
    let mut surface_classes = BTreeSet::new();
    let mut geometry_classes = BTreeSet::new();
    let mut state_tokens = BTreeSet::new();
    for row in &packet.object_states {
        surface_classes.insert(row.surface_class);
        geometry_classes.insert(row.geometry_class);
        for token in &row.state_tokens {
            state_tokens.insert(*token);
        }
        validate_object_state(row, &mut errors);
    }
    for required in required_surface_classes() {
        if !surface_classes.contains(&required) {
            errors.push(InteractionIntegrityValidationError::CoverageMissing {
                missing: format!("surface:{}", required.as_str()),
            });
        }
    }
    for required in required_geometry_classes() {
        if !geometry_classes.contains(&required) {
            errors.push(InteractionIntegrityValidationError::CoverageMissing {
                missing: format!("geometry:{}", required.as_str()),
            });
        }
    }
    for required in required_state_tokens() {
        if !state_tokens.contains(&required) {
            errors.push(InteractionIntegrityValidationError::CoverageMissing {
                missing: format!("state:{}", required.as_str()),
            });
        }
    }

    for review in &packet.batch_scope_reviews {
        validate_batch_scope(review, &object_by_id, &mut errors);
    }
    validate_identity_cues(&packet.identity_cues, &mut errors);
    validate_focus_return_rules(&packet.focus_return_rules, &mut errors);
    validate_vocabulary_parity(&packet.vocabulary_parity, &mut errors);
    validate_support_export(packet, &mut errors);
    validate_replay_fixtures(&packet.replay_fixtures, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Builds the seeded interaction-integrity beta packet used by fixtures and the inspector.
pub fn seeded_interaction_integrity_beta_packet() -> InteractionIntegrityBetaPacket {
    let object_states = seeded_object_states();
    let batch_scope_reviews = vec![
        build_batch_scope_review(
            "batch_scope:search:delete-selected-results",
            "surface:search:results",
            "collection:search:results",
            "search.delete_selected_results",
            "Delete selected results",
            BatchActionClass::DestructiveMutation,
            SelectionScopeClass::ExplicitCustomSet,
            BatchExecutionOriginClass::ClientLocalExecution,
            "query_snapshot:search:results:2026-05-17T00:00:00Z",
            &object_states,
        ),
        build_batch_scope_review(
            "batch_scope:review:publish-selected-updates",
            "surface:review:queue",
            "collection:review:queue",
            "review.publish_selected_updates",
            "Publish selected review updates",
            BatchActionClass::ProviderOwnedMutation,
            SelectionScopeClass::ExplicitCustomSet,
            BatchExecutionOriginClass::ProviderAuthoritativeExecution,
            "provider_snapshot:review:queue:2026-05-17T00:00:00Z",
            &object_states,
        ),
        build_batch_scope_review(
            "batch_scope:git:discard-selected-changes",
            "surface:git:change-tree",
            "collection:git:change-tree",
            "git.discard_selected_changes",
            "Discard selected changes",
            BatchActionClass::DestructiveMutation,
            SelectionScopeClass::ExplicitCustomSet,
            BatchExecutionOriginClass::ClientLocalExecution,
            "git_status_snapshot:working-tree:2026-05-17T00:00:00Z",
            &object_states,
        ),
        build_batch_scope_review(
            "batch_scope:extensions:enable-selected",
            "surface:extensions:inventory",
            "collection:extensions:inventory",
            "extensions.enable_selected",
            "Enable selected extensions",
            BatchActionClass::ProviderOwnedMutation,
            SelectionScopeClass::ExplicitCustomSet,
            BatchExecutionOriginClass::MixedClientThenProvider,
            "package_review_snapshot:extensions:2026-05-17T00:00:00Z",
            &object_states,
        ),
        build_batch_scope_review(
            "batch_scope:activity:acknowledge-selected",
            "surface:activity:center",
            "collection:activity:center",
            "activity.acknowledge_selected",
            "Acknowledge selected activity rows",
            BatchActionClass::LocalReversible,
            SelectionScopeClass::ExplicitCustomSet,
            BatchExecutionOriginClass::ClientLocalExecution,
            "activity_snapshot:durable-attention:2026-05-17T00:00:00Z",
            &object_states,
        ),
    ];
    let identity_cues = seeded_identity_cues();
    let focus_return_rules = seeded_focus_return_rules();
    let vocabulary_parity = seeded_vocabulary_parity();
    let packet_id = "shell:interaction-integrity:beta:packet:default".to_owned();
    let support_export = build_support_export(&packet_id, &object_states, &batch_scope_reviews);
    let replay_fixtures = seeded_replay_fixtures(&support_export.support_export_id);
    let summary = InteractionIntegrityBetaSummary::from_parts(
        &object_states,
        &batch_scope_reviews,
        &identity_cues,
        &focus_return_rules,
        &vocabulary_parity,
        &replay_fixtures,
    );

    InteractionIntegrityBetaPacket {
        record_kind: INTERACTION_INTEGRITY_PACKET_RECORD_KIND.to_owned(),
        schema_version: INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION,
        shared_contract_ref: INTERACTION_INTEGRITY_SHARED_CONTRACT_REF.to_owned(),
        packet_id,
        generated_at: "2026-05-17T00:00:00Z".to_owned(),
        summary,
        object_states,
        batch_scope_reviews,
        identity_cues,
        focus_return_rules,
        vocabulary_parity,
        support_export,
        replay_fixtures,
    }
}

#[derive(Debug, Clone, Copy)]
struct SeedObjectState {
    surface_id_ref: &'static str,
    surface_class: ClaimedCollectionSurfaceClass,
    geometry_class: CollectionGeometryClass,
    object_id_ref: &'static str,
    object_label: &'static str,
    identity_cue_label: &'static str,
    focused: bool,
    active: bool,
    selected: bool,
    pending: bool,
    disabled: bool,
    filtered: bool,
    hidden: bool,
    blocked: bool,
    disabled_reason_label: Option<&'static str>,
    filtered_reason_label: Option<&'static str>,
    hidden_reason_label: Option<&'static str>,
    blocked_reason_label: Option<&'static str>,
}

#[allow(clippy::too_many_arguments)]
fn seed_object(
    surface_id_ref: &'static str,
    surface_class: ClaimedCollectionSurfaceClass,
    geometry_class: CollectionGeometryClass,
    object_id_ref: &'static str,
    object_label: &'static str,
    identity_cue_label: &'static str,
    focused: bool,
    active: bool,
    selected: bool,
    pending: bool,
    disabled: bool,
    filtered: bool,
    hidden: bool,
    blocked: bool,
    disabled_reason_label: Option<&'static str>,
    filtered_reason_label: Option<&'static str>,
    hidden_reason_label: Option<&'static str>,
    blocked_reason_label: Option<&'static str>,
) -> ObjectInteractionStateRecord {
    ObjectInteractionStateRecord::from_seed(SeedObjectState {
        surface_id_ref,
        surface_class,
        geometry_class,
        object_id_ref,
        object_label,
        identity_cue_label,
        focused,
        active,
        selected,
        pending,
        disabled,
        filtered,
        hidden,
        blocked,
        disabled_reason_label,
        filtered_reason_label,
        hidden_reason_label,
        blocked_reason_label,
    })
}

fn seeded_object_states() -> Vec<ObjectInteractionStateRecord> {
    use ClaimedCollectionSurfaceClass::{
        ActivityCenterList, GitChangeTree, MarketplaceInventoryGrid, ReviewQueueGrid,
        SearchResultsList,
    };
    use CollectionGeometryClass::{DenseList, Grid, Tree, VirtualizedReviewSurface};

    vec![
        seed_object(
            "surface:search:results",
            SearchResultsList,
            DenseList,
            "search:result:payments-timeout",
            "payments timeout result",
            "payments timeout",
            true,
            true,
            true,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
        ),
        seed_object(
            "surface:search:results",
            SearchResultsList,
            DenseList,
            "search:result:prod-secret",
            "redacted production secret result",
            "redacted prod result",
            false,
            false,
            true,
            false,
            false,
            true,
            true,
            false,
            None,
            Some("outside current workset filter"),
            Some("policy hides production path"),
            None,
        ),
        seed_object(
            "surface:search:results",
            SearchResultsList,
            DenseList,
            "search:result:generated-lockfile",
            "generated lockfile result",
            "generated lockfile",
            false,
            false,
            true,
            false,
            true,
            false,
            false,
            true,
            Some("generated artifact is read-only"),
            None,
            None,
            Some("generated artifact cannot be deleted from search"),
        ),
        seed_object(
            "surface:search:results",
            SearchResultsList,
            DenseList,
            "search:result:index-refresh",
            "index-refresh pending result",
            "index refresh pending",
            false,
            false,
            true,
            true,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
        ),
        seed_object(
            "surface:search:results",
            SearchResultsList,
            DenseList,
            "search:result:docs-link",
            "docs link result",
            "docs link",
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
        ),
        seed_object(
            "surface:review:queue",
            ReviewQueueGrid,
            VirtualizedReviewSurface,
            "review:change:pr-181",
            "review update PR 181",
            "PR 181",
            true,
            true,
            true,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
        ),
        seed_object(
            "surface:review:queue",
            ReviewQueueGrid,
            VirtualizedReviewSurface,
            "review:change:pr-182",
            "review update PR 182",
            "PR 182",
            false,
            false,
            true,
            false,
            false,
            true,
            false,
            false,
            None,
            Some("filtered out by owner:me saved view"),
            None,
            None,
        ),
        seed_object(
            "surface:review:queue",
            ReviewQueueGrid,
            VirtualizedReviewSurface,
            "review:change:pr-183",
            "review update PR 183",
            "PR 183",
            false,
            false,
            true,
            false,
            true,
            false,
            false,
            true,
            Some("provider requires fresh approval"),
            None,
            None,
            Some("publish blocked by provider policy"),
        ),
        seed_object(
            "surface:review:queue",
            ReviewQueueGrid,
            VirtualizedReviewSurface,
            "review:change:pr-184",
            "review update PR 184",
            "PR 184",
            false,
            false,
            true,
            true,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
        ),
        seed_object(
            "surface:review:queue",
            ReviewQueueGrid,
            VirtualizedReviewSurface,
            "review:change:pr-185",
            "review update PR 185",
            "PR 185",
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
        ),
        seed_object(
            "surface:git:change-tree",
            GitChangeTree,
            Tree,
            "git:change:src-lib",
            "src/lib.rs",
            "src/lib.rs",
            true,
            true,
            true,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
        ),
        seed_object(
            "surface:git:change-tree",
            GitChangeTree,
            Tree,
            "git:change:generated-code",
            "generated API file",
            "generated API",
            false,
            false,
            true,
            false,
            true,
            false,
            false,
            true,
            Some("generated output requires source regeneration"),
            None,
            None,
            Some("discard blocked for generated output"),
        ),
        seed_object(
            "surface:git:change-tree",
            GitChangeTree,
            Tree,
            "git:change:hidden-vendor",
            "vendor subtree change",
            "vendor subtree",
            false,
            false,
            true,
            false,
            false,
            true,
            true,
            false,
            None,
            Some("outside current sparse checkout"),
            Some("vendor subtree hidden by workset"),
            None,
        ),
        seed_object(
            "surface:git:change-tree",
            GitChangeTree,
            Tree,
            "git:change:readme",
            "README.md",
            "README.md",
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
        ),
        seed_object(
            "surface:extensions:inventory",
            MarketplaceInventoryGrid,
            Grid,
            "extension:devtools:stable",
            "Aureline DevTools stable",
            "DevTools stable",
            true,
            true,
            true,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
        ),
        seed_object(
            "surface:extensions:inventory",
            MarketplaceInventoryGrid,
            Grid,
            "extension:prod-publisher:revoked",
            "revoked publisher extension",
            "revoked publisher",
            false,
            false,
            true,
            false,
            true,
            false,
            false,
            true,
            Some("publisher transfer requires review"),
            None,
            None,
            Some("extension is revoked"),
        ),
        seed_object(
            "surface:extensions:inventory",
            MarketplaceInventoryGrid,
            Grid,
            "extension:mirror:filtered",
            "mirror-only extension",
            "mirror-only",
            false,
            false,
            true,
            false,
            false,
            true,
            true,
            false,
            None,
            Some("filtered to online catalog"),
            Some("mirror-only row hidden in online view"),
            None,
        ),
        seed_object(
            "surface:extensions:inventory",
            MarketplaceInventoryGrid,
            Grid,
            "extension:preview:pending",
            "preview extension pending moderation",
            "preview pending",
            false,
            false,
            true,
            true,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
        ),
        seed_object(
            "surface:activity:center",
            ActivityCenterList,
            DenseList,
            "activity:run-tests",
            "test run activity row",
            "test run",
            true,
            true,
            true,
            false,
            false,
            false,
            false,
            false,
            None,
            None,
            None,
            None,
        ),
        seed_object(
            "surface:activity:center",
            ActivityCenterList,
            DenseList,
            "activity:admin-hold",
            "admin-held activity row",
            "admin hold",
            false,
            false,
            true,
            false,
            true,
            false,
            false,
            true,
            Some("admin hold requires review"),
            None,
            None,
            Some("acknowledge blocked by admin hold"),
        ),
        seed_object(
            "surface:activity:center",
            ActivityCenterList,
            DenseList,
            "activity:quiet-hidden",
            "quiet-hours held row",
            "quiet-hours held",
            false,
            false,
            true,
            false,
            false,
            true,
            true,
            false,
            None,
            Some("filtered to active work only"),
            Some("quiet-hours held row hidden from compact list"),
            None,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn build_batch_scope_review(
    batch_scope_id: &str,
    surface_id_ref: &str,
    collection_view_id_ref: &str,
    action_id: &str,
    action_label: &str,
    action_class: BatchActionClass,
    selection_scope_class: SelectionScopeClass,
    execution_origin_class: BatchExecutionOriginClass,
    basis_snapshot_ref: &str,
    object_states: &[ObjectInteractionStateRecord],
) -> BatchScopeTruthRecord {
    let surface_objects = object_states
        .iter()
        .filter(|row| row.surface_id_ref == surface_id_ref)
        .collect::<Vec<_>>();
    let selected = surface_objects
        .iter()
        .filter(|row| row.selected)
        .collect::<Vec<_>>();
    let resulting_target_id_refs = selected
        .iter()
        .filter(|row| row.eligible_for_batch_target())
        .map(|row| row.object_id_ref.clone())
        .collect::<Vec<_>>();
    let filtered_item_id_refs = selected
        .iter()
        .filter(|row| row.filtered)
        .map(|row| row.object_id_ref.clone())
        .collect::<Vec<_>>();
    let hidden_item_id_refs = selected
        .iter()
        .filter(|row| row.hidden)
        .map(|row| row.object_id_ref.clone())
        .collect::<Vec<_>>();
    let disabled_item_id_refs = selected
        .iter()
        .filter(|row| row.disabled)
        .map(|row| row.object_id_ref.clone())
        .collect::<Vec<_>>();
    let pending_item_id_refs = selected
        .iter()
        .filter(|row| row.pending)
        .map(|row| row.object_id_ref.clone())
        .collect::<Vec<_>>();
    let blocked_item_id_refs = selected
        .iter()
        .filter(|row| row.blocked || row.disabled)
        .map(|row| row.object_id_ref.clone())
        .collect::<Vec<_>>();

    let members = surface_objects
        .iter()
        .map(|row| {
            let disposition = member_disposition(row);
            BatchReviewMember {
                item: StableCollectionItemRef::new(
                    row.object_id_ref.clone(),
                    collection_surface_family(row.surface_class),
                    row.surface_id_ref.clone(),
                    row.object_label.clone(),
                )
                .with_blocked(row.blocked || row.disabled)
                .with_hidden(row.hidden || row.filtered)
                .with_stale(row.pending),
                disposition,
                reason_label: member_reason(row, disposition),
            }
        })
        .collect::<Vec<_>>();

    let hidden_or_filtered_count = sorted_unique(
        filtered_item_id_refs
            .iter()
            .chain(hidden_item_id_refs.iter())
            .cloned(),
    )
    .len() as u64;
    let counters = CollectionScopeCounters::from_known_values(
        surface_objects
            .iter()
            .filter(|row| !row.hidden && !row.filtered)
            .count() as u64,
        surface_objects.len() as u64,
        surface_objects.len() as u64,
        selected.len() as u64,
        blocked_item_id_refs.len() as u64,
        hidden_or_filtered_count,
        selected
            .iter()
            .filter(|row| row.hidden && row.blocked)
            .count() as u64,
        filtered_item_id_refs.len() as u64,
        CollectionCountStatus::Exact,
    );
    let batch_review_sheet = BatchReviewSheet::from_members(
        batch_scope_id,
        collection_view_id_ref,
        action_id,
        action_label,
        action_class,
        selection_scope_class,
        execution_origin_class,
        counters,
        members,
        "Continue only with included stable identities; excluded, blocked, filtered, and hidden objects remain untouched.",
    );
    let excluded_item_id_refs = batch_review_sheet.excluded_item_id_refs.clone();
    let included_count = resulting_target_id_refs.len() as u64;
    let excluded_count = excluded_item_id_refs.len() as u64;
    let blocked_count = blocked_item_id_refs.len() as u64;
    let filtered_count = filtered_item_id_refs.len() as u64;
    let hidden_count = hidden_item_id_refs.len() as u64;
    let disabled_count = disabled_item_id_refs.len() as u64;
    let pending_count = pending_item_id_refs.len() as u64;

    BatchScopeTruthRecord {
        record_kind: BATCH_SCOPE_TRUTH_RECORD_KIND.to_owned(),
        schema_version: INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION,
        shared_contract_ref: INTERACTION_INTEGRITY_SHARED_CONTRACT_REF.to_owned(),
        batch_scope_id: batch_scope_id.to_owned(),
        surface_id_ref: surface_id_ref.to_owned(),
        action_label: action_label.to_owned(),
        action_is_destructive_or_publish_capable: action_is_protected(action_class),
        batch_review_sheet,
        included_count,
        excluded_count,
        blocked_count,
        filtered_count,
        hidden_count,
        disabled_count,
        pending_count,
        resulting_target_id_refs,
        excluded_item_id_refs,
        blocked_item_id_refs,
        filtered_item_id_refs,
        hidden_item_id_refs,
        disabled_item_id_refs,
        pending_item_id_refs,
        basis_snapshot_ref: basis_snapshot_ref.to_owned(),
        material_scope_drift_invalidates_commit: true,
        commit_path_enabled: included_count > 0,
        batch_review_summary_label: format!(
            "{included_count} included · {excluded_count} excluded · {blocked_count} blocked · {filtered_count} filtered · {hidden_count} hidden"
        ),
        hidden_or_filtered_silently_included_denied: true,
        stable_object_identity_visible: true,
    }
}

fn seeded_identity_cues() -> Vec<ShellIdentityCueRecord> {
    let mut expanded_frame = DesktopFrame::new(1920, 1080);
    expanded_frame.open_placeholder_tab();
    let _ = expanded_frame.request_split_focused_editor_group();
    let expanded_modes = expanded_frame
        .responsive_fallback_modes()
        .into_iter()
        .map(|mode| mode.name().to_owned())
        .collect::<Vec<_>>();
    let mut compact_frame = expanded_frame.clone();
    compact_frame.relayout(1120, 820);
    let compact_modes = compact_frame
        .responsive_fallback_modes()
        .into_iter()
        .map(|mode| mode.name().to_owned())
        .collect::<Vec<_>>();
    let mut modes = expanded_modes;
    for mode in compact_modes {
        if !modes.contains(&mode) {
            modes.push(mode);
        }
    }

    vec![
        identity_cue(
            "identity_cue:breadcrumb:git-src-lib",
            "surface:git:change-tree",
            IdentityCueRole::Breadcrumb,
            "git:change:src-lib",
            "window:primary",
            "title_context_bar",
            "slot.title_context_bar.identity",
            "slot.title_context_bar.identity",
            "slot.title_context_bar.identity",
            "slot.title_context_bar.identity",
            "slot.title_context_bar.identity",
            "src/lib.rs breadcrumb",
            modes.clone(),
        ),
        identity_cue(
            "identity_cue:status:search-results",
            "surface:search:results",
            IdentityCueRole::StatusItem,
            "search:result:payments-timeout",
            "window:primary",
            "status_bar",
            "status.slot.work.summary",
            "status.slot.work.summary",
            "status.slot.work.summary",
            "status.slot.work.summary",
            "status.slot.work.summary",
            "search result status",
            modes.clone(),
        ),
        identity_cue(
            "identity_cue:tab:review-pr-181",
            "surface:review:queue",
            IdentityCueRole::TabIdentity,
            "review:change:pr-181",
            "window:primary",
            "main_workspace",
            "slot.main_workspace.review_surface",
            "slot.main_workspace.review_surface",
            "slot.main_workspace.review_surface",
            "slot.main_workspace.review_surface",
            "slot.main_workspace.review_surface",
            "PR 181 review tab",
            modes.clone(),
        ),
        identity_cue(
            "identity_cue:inspector:extension-devtools",
            "surface:extensions:inventory",
            IdentityCueRole::InspectorHeader,
            "extension:devtools:stable",
            "window:primary",
            "right_inspector",
            "slot.right_inspector.contextual_detail",
            "slot.overlay.dialog_or_sheet",
            "slot.overlay.dialog_or_sheet",
            "slot.overlay.dialog_or_sheet",
            "slot.overlay.dialog_or_sheet",
            "DevTools inspector",
            modes.clone(),
        ),
        identity_cue(
            "identity_cue:terminal:build-linux-3",
            "surface:terminal:bottom-panel",
            IdentityCueRole::TerminalHeader,
            "terminal:session:build-linux-3",
            "window:primary",
            "bottom_panel",
            "slot.bottom_panel.tool_panels",
            "slot.bottom_panel.tool_panels",
            "slot.bottom_panel.tool_panels",
            "slot.bottom_panel.tool_panels",
            "slot.bottom_panel.tool_panels",
            "build-linux-3 terminal",
            modes,
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn identity_cue(
    cue_id: &str,
    surface_id_ref: &str,
    cue_role: IdentityCueRole,
    canonical_object_id_ref: &str,
    window_id_ref: &str,
    source_zone_id_ref: &str,
    source_slot_id_ref: &str,
    resize_fallback_slot_id_ref: &str,
    split_fallback_slot_id_ref: &str,
    detach_fallback_slot_id_ref: &str,
    multi_window_fallback_slot_id_ref: &str,
    display_label: &str,
    responsive_fallback_modes: Vec<String>,
) -> ShellIdentityCueRecord {
    ShellIdentityCueRecord {
        record_kind: SHELL_IDENTITY_CUE_RECORD_KIND.to_owned(),
        schema_version: INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION,
        shared_contract_ref: INTERACTION_INTEGRITY_SHARED_CONTRACT_REF.to_owned(),
        cue_id: cue_id.to_owned(),
        surface_id_ref: surface_id_ref.to_owned(),
        cue_role,
        canonical_object_id_ref: canonical_object_id_ref.to_owned(),
        window_id_ref: window_id_ref.to_owned(),
        source_zone_id_ref: source_zone_id_ref.to_owned(),
        source_slot_id_ref: source_slot_id_ref.to_owned(),
        resize_fallback_slot_id_ref: resize_fallback_slot_id_ref.to_owned(),
        split_fallback_slot_id_ref: split_fallback_slot_id_ref.to_owned(),
        detach_fallback_slot_id_ref: detach_fallback_slot_id_ref.to_owned(),
        multi_window_fallback_slot_id_ref: multi_window_fallback_slot_id_ref.to_owned(),
        display_label: display_label.to_owned(),
        responsive_fallback_modes,
        stable_under_resize: true,
        stable_under_split: true,
        stable_under_detach: true,
        stable_under_multi_window_fallback: true,
    }
}

fn seeded_focus_return_rules() -> Vec<FocusReturnRuleRecord> {
    vec![
        focus_return_rule(
            "focus_return:dialog:git-discard",
            FocusReturnTriggerClass::ModalDialogDismiss,
            "surface:git:change-tree",
            "git:change:src-lib",
            "command.git.discard_selected_changes",
            "git:change:src-lib",
            "surface:git:change-tree",
            "Returned to src/lib.rs in the change tree.",
        ),
        focus_return_rule(
            "focus_return:sheet:review-publish",
            FocusReturnTriggerClass::SheetDismiss,
            "surface:review:queue",
            "review:change:pr-181",
            "command.review.publish_selected_updates",
            "review:change:pr-181",
            "surface:review:queue",
            "Returned to the selected review batch.",
        ),
        focus_return_rule(
            "focus_return:prompt:extension-enable",
            FocusReturnTriggerClass::ApprovalPromptResolve,
            "surface:extensions:inventory",
            "extension:devtools:stable",
            "command.extensions.enable_selected",
            "extension:devtools:stable",
            "surface:extensions:inventory",
            "Returned to DevTools stable after approval review.",
        ),
        focus_return_rule(
            "focus_return:activity:open-details",
            FocusReturnTriggerClass::ActivityCenterJump,
            "surface:activity:center",
            "activity:run-tests",
            "command.activity.open_details",
            "activity:run-tests",
            "surface:activity:center",
            "Returned to the test run activity row.",
        ),
        focus_return_rule(
            "focus_return:pane:inspector-sheet",
            FocusReturnTriggerClass::PaneCloseOrSplitMerge,
            "surface:extensions:inventory",
            "extension:devtools:stable",
            "command.shell.toggle_inspector",
            "extension:devtools:stable",
            "slot.main_workspace.working_set",
            "Inspector fallback closed; focus returned to the extension row.",
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn focus_return_rule(
    rule_id: &str,
    trigger_class: FocusReturnTriggerClass,
    origin_surface_id_ref: &str,
    origin_object_id_ref: &str,
    invoking_control_id_ref: &str,
    expected_return_target_id_ref: &str,
    fallback_return_target_id_ref: &str,
    screen_reader_announcement: &str,
) -> FocusReturnRuleRecord {
    FocusReturnRuleRecord {
        record_kind: FOCUS_RETURN_RULE_RECORD_KIND.to_owned(),
        schema_version: INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION,
        shared_contract_ref: INTERACTION_INTEGRITY_SHARED_CONTRACT_REF.to_owned(),
        rule_id: rule_id.to_owned(),
        trigger_class,
        origin_surface_id_ref: origin_surface_id_ref.to_owned(),
        origin_object_id_ref: origin_object_id_ref.to_owned(),
        invoking_control_id_ref: invoking_control_id_ref.to_owned(),
        expected_return_target_id_ref: expected_return_target_id_ref.to_owned(),
        fallback_return_target_id_ref: fallback_return_target_id_ref.to_owned(),
        screen_reader_announcement: screen_reader_announcement.to_owned(),
        focus_return_explicit: true,
        never_returns_to_hidden_or_offscreen: true,
        preserves_selection_or_cursor_state: true,
    }
}

fn seeded_vocabulary_parity() -> Vec<VocabularyParityRecord> {
    [
        (
            "surface:search:results",
            ClaimedCollectionSurfaceClass::SearchResultsList,
        ),
        (
            "surface:review:queue",
            ClaimedCollectionSurfaceClass::ReviewQueueGrid,
        ),
        (
            "surface:git:change-tree",
            ClaimedCollectionSurfaceClass::GitChangeTree,
        ),
        (
            "surface:extensions:inventory",
            ClaimedCollectionSurfaceClass::MarketplaceInventoryGrid,
        ),
        (
            "surface:activity:center",
            ClaimedCollectionSurfaceClass::ActivityCenterList,
        ),
    ]
    .into_iter()
    .map(|(surface_id_ref, surface_class)| {
        let tokens = required_state_tokens().to_vec();
        VocabularyParityRecord {
            record_kind: VOCABULARY_PARITY_RECORD_KIND.to_owned(),
            schema_version: INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION,
            shared_contract_ref: INTERACTION_INTEGRITY_SHARED_CONTRACT_REF.to_owned(),
            surface_id_ref: surface_id_ref.to_owned(),
            surface_class,
            ui_tokens: tokens.clone(),
            keyboard_help_tokens: tokens.clone(),
            accessibility_tokens: tokens.clone(),
            support_export_tokens: tokens,
            token_sets_match: true,
        }
    })
    .collect()
}

fn build_support_export(
    packet_id_ref: &str,
    object_states: &[ObjectInteractionStateRecord],
    batch_scope_reviews: &[BatchScopeTruthRecord],
) -> InteractionIntegritySupportExport {
    let selected_scope_id_refs = sorted_unique(
        object_states
            .iter()
            .filter(|row| row.selected)
            .map(|row| row.object_id_ref.clone()),
    );
    let filtered_scope_id_refs = sorted_unique(
        object_states
            .iter()
            .filter(|row| row.selected && row.filtered)
            .map(|row| row.object_id_ref.clone()),
    );
    let hidden_scope_id_refs = sorted_unique(
        object_states
            .iter()
            .filter(|row| row.selected && row.hidden)
            .map(|row| row.object_id_ref.clone()),
    );
    let blocked_scope_id_refs = sorted_unique(
        object_states
            .iter()
            .filter(|row| row.selected && (row.blocked || row.disabled))
            .map(|row| row.object_id_ref.clone()),
    );
    let resulting_target_id_refs = sorted_unique(
        batch_scope_reviews
            .iter()
            .flat_map(|review| review.resulting_target_id_refs.iter().cloned()),
    );
    let reviewed_batch_scope_refs = batch_scope_reviews
        .iter()
        .map(|review| review.batch_scope_id.clone())
        .collect();

    InteractionIntegritySupportExport {
        record_kind: INTERACTION_INTEGRITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION,
        shared_contract_ref: INTERACTION_INTEGRITY_SHARED_CONTRACT_REF.to_owned(),
        support_export_id: "support_export:interaction-integrity:beta:default".to_owned(),
        packet_id_ref: packet_id_ref.to_owned(),
        selected_scope_id_refs,
        filtered_scope_id_refs,
        hidden_scope_id_refs,
        blocked_scope_id_refs,
        resulting_target_id_refs,
        reviewed_batch_scope_refs,
        support_export_state_tokens: required_state_tokens().to_vec(),
        redaction_class: "metadata_only_redacted_ids".to_owned(),
        raw_filter_literals_included: false,
        raw_object_payloads_included: false,
        raw_private_labels_included: false,
    }
}

fn seeded_replay_fixtures(support_export_id: &str) -> Vec<InteractionIntegrityReplayFixture> {
    vec![
        replay_fixture(
            "replay:virtualized-selection",
            ReplayDrillClass::VirtualizedSelection,
            vec![
                "surface:search:results".to_owned(),
                "surface:review:queue".to_owned(),
            ],
            vec![
                "fixtures/shell/m3/interaction_integrity/state_model.json".to_owned(),
                "fixtures/shell/m3/interaction_integrity/batch_reviews.json".to_owned(),
            ],
            support_export_id,
        ),
        replay_fixture(
            "replay:batch-scope-review",
            ReplayDrillClass::BatchScopeReview,
            vec![
                "surface:git:change-tree".to_owned(),
                "surface:extensions:inventory".to_owned(),
            ],
            vec![
                "fixtures/shell/m3/interaction_integrity/batch_reviews.json".to_owned(),
                "fixtures/shell/m3/interaction_integrity/support_export.json".to_owned(),
            ],
            support_export_id,
        ),
        replay_fixture(
            "replay:responsive-identity-cues",
            ReplayDrillClass::ResponsiveIdentityCue,
            vec![
                "surface:git:change-tree".to_owned(),
                "surface:review:queue".to_owned(),
                "surface:terminal:bottom-panel".to_owned(),
            ],
            vec!["fixtures/shell/m3/interaction_integrity/identity_cues.json".to_owned()],
            support_export_id,
        ),
        replay_fixture(
            "replay:focus-return",
            ReplayDrillClass::FocusReturn,
            vec![
                "surface:git:change-tree".to_owned(),
                "surface:review:queue".to_owned(),
                "surface:activity:center".to_owned(),
            ],
            vec!["fixtures/shell/m3/interaction_integrity/focus_return.json".to_owned()],
            support_export_id,
        ),
        replay_fixture(
            "replay:support-export",
            ReplayDrillClass::SupportExport,
            vec![
                "surface:search:results".to_owned(),
                "surface:review:queue".to_owned(),
                "surface:git:change-tree".to_owned(),
                "surface:extensions:inventory".to_owned(),
                "surface:activity:center".to_owned(),
            ],
            vec!["fixtures/shell/m3/interaction_integrity/support_export.json".to_owned()],
            support_export_id,
        ),
    ]
}

fn replay_fixture(
    fixture_id: &str,
    drill_class: ReplayDrillClass,
    covered_surface_refs: Vec<String>,
    fixture_refs: Vec<String>,
    expected_support_export_ref: &str,
) -> InteractionIntegrityReplayFixture {
    InteractionIntegrityReplayFixture {
        record_kind: INTERACTION_INTEGRITY_REPLAY_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION,
        shared_contract_ref: INTERACTION_INTEGRITY_SHARED_CONTRACT_REF.to_owned(),
        fixture_id: fixture_id.to_owned(),
        drill_class,
        covered_surface_refs,
        fixture_refs,
        expected_support_export_ref: expected_support_export_ref.to_owned(),
        replay_command:
            "cargo run -q -p aureline-shell --bin aureline_shell_interaction_integrity -- validate"
                .to_owned(),
    }
}

fn validate_object_state(
    row: &ObjectInteractionStateRecord,
    errors: &mut Vec<InteractionIntegrityValidationError>,
) {
    if row.record_kind != OBJECT_INTERACTION_STATE_RECORD_KIND
        || row.schema_version != INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION
        || row.shared_contract_ref != INTERACTION_INTEGRITY_SHARED_CONTRACT_REF
    {
        errors.push(InteractionIntegrityValidationError::ObjectStateInvalid {
            object_id_ref: row.object_id_ref.clone(),
            reason: "record metadata mismatch".to_owned(),
        });
    }
    if row.object_id_ref.trim().is_empty() || row.surface_id_ref.trim().is_empty() {
        errors.push(InteractionIntegrityValidationError::ObjectStateInvalid {
            object_id_ref: row.object_id_ref.clone(),
            reason: "stable object and surface ids are required".to_owned(),
        });
    }
    let expected = object_state_tokens(
        row.focused,
        row.active,
        row.selected,
        row.pending,
        row.disabled,
        row.filtered,
        row.hidden,
        row.blocked,
    );
    if row.state_tokens != expected
        || row.ui_state_tokens != expected
        || row.keyboard_help_state_tokens != expected
        || row.accessibility_state_tokens != expected
        || row.support_export_state_tokens != expected
    {
        errors.push(InteractionIntegrityValidationError::ObjectStateInvalid {
            object_id_ref: row.object_id_ref.clone(),
            reason: "state tokens must match UI, keyboard, accessibility, and support export"
                .to_owned(),
        });
    }
    if row.filtered
        && row
            .filtered_reason_label
            .as_deref()
            .map_or(true, str::is_empty)
    {
        errors.push(InteractionIntegrityValidationError::ObjectStateInvalid {
            object_id_ref: row.object_id_ref.clone(),
            reason: "filtered objects need a reason label".to_owned(),
        });
    }
    if row.hidden
        && row
            .hidden_reason_label
            .as_deref()
            .map_or(true, str::is_empty)
    {
        errors.push(InteractionIntegrityValidationError::ObjectStateInvalid {
            object_id_ref: row.object_id_ref.clone(),
            reason: "hidden objects need a reason label".to_owned(),
        });
    }
    if row.disabled
        && row
            .disabled_reason_label
            .as_deref()
            .map_or(true, str::is_empty)
    {
        errors.push(InteractionIntegrityValidationError::ObjectStateInvalid {
            object_id_ref: row.object_id_ref.clone(),
            reason: "disabled objects need a reason label".to_owned(),
        });
    }
    if row.blocked
        && row
            .blocked_reason_label
            .as_deref()
            .map_or(true, str::is_empty)
    {
        errors.push(InteractionIntegrityValidationError::ObjectStateInvalid {
            object_id_ref: row.object_id_ref.clone(),
            reason: "blocked objects need a reason label".to_owned(),
        });
    }
    if !row.activation_preserves_selection || !row.identity_survives_virtualization {
        errors.push(InteractionIntegrityValidationError::ObjectStateInvalid {
            object_id_ref: row.object_id_ref.clone(),
            reason: "activation and virtualization invariants must be explicit".to_owned(),
        });
    }
}

fn validate_batch_scope(
    review: &BatchScopeTruthRecord,
    object_by_id: &BTreeMap<&str, &ObjectInteractionStateRecord>,
    errors: &mut Vec<InteractionIntegrityValidationError>,
) {
    if review.record_kind != BATCH_SCOPE_TRUTH_RECORD_KIND
        || review.schema_version != INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION
        || review.shared_contract_ref != INTERACTION_INTEGRITY_SHARED_CONTRACT_REF
    {
        errors.push(InteractionIntegrityValidationError::BatchScopeDrift {
            batch_scope_id: review.batch_scope_id.clone(),
            reason: "record metadata mismatch".to_owned(),
        });
    }
    if review.action_is_destructive_or_publish_capable && !review.batch_review_sheet.review_required
    {
        errors.push(InteractionIntegrityValidationError::BatchScopeDrift {
            batch_scope_id: review.batch_scope_id.clone(),
            reason: "protected batch action requires a review sheet".to_owned(),
        });
    }
    if !review.hidden_or_filtered_silently_included_denied
        || !review.stable_object_identity_visible
        || !review.material_scope_drift_invalidates_commit
    {
        errors.push(InteractionIntegrityValidationError::BatchScopeDrift {
            batch_scope_id: review.batch_scope_id.clone(),
            reason: "batch invariants are not explicit".to_owned(),
        });
    }
    if review.commit_path_enabled && review.resulting_target_id_refs.is_empty() {
        errors.push(InteractionIntegrityValidationError::BatchScopeDrift {
            batch_scope_id: review.batch_scope_id.clone(),
            reason: "enabled commit path needs resulting target ids".to_owned(),
        });
    }
    let included = sorted_unique(review.resulting_target_id_refs.iter().cloned());
    if sorted_unique(
        review
            .batch_review_sheet
            .included_item_id_refs
            .iter()
            .cloned(),
    ) != included
    {
        errors.push(InteractionIntegrityValidationError::BatchScopeDrift {
            batch_scope_id: review.batch_scope_id.clone(),
            reason: "shared batch sheet included ids must match resulting target ids".to_owned(),
        });
    }
    for target in &review.resulting_target_id_refs {
        match object_by_id.get(target.as_str()) {
            Some(row) => {
                if !row.eligible_for_batch_target() {
                    errors.push(InteractionIntegrityValidationError::UnsafeBatchTargetIncluded {
                        batch_scope_id: review.batch_scope_id.clone(),
                        object_id_ref: target.clone(),
                        reason: "target is hidden, filtered, blocked, disabled, pending, or unselected".to_owned(),
                    });
                }
            }
            None => errors.push(
                InteractionIntegrityValidationError::UnsafeBatchTargetIncluded {
                    batch_scope_id: review.batch_scope_id.clone(),
                    object_id_ref: target.clone(),
                    reason: "target id has no object state row".to_owned(),
                },
            ),
        }
    }
    for target in review
        .filtered_item_id_refs
        .iter()
        .chain(review.hidden_item_id_refs.iter())
    {
        if review.resulting_target_id_refs.contains(target) {
            errors.push(
                InteractionIntegrityValidationError::UnsafeBatchTargetIncluded {
                    batch_scope_id: review.batch_scope_id.clone(),
                    object_id_ref: target.clone(),
                    reason: "hidden or filtered object was included".to_owned(),
                },
            );
        }
    }
    if review.included_count != review.resulting_target_id_refs.len() as u64
        || review.filtered_count != review.filtered_item_id_refs.len() as u64
        || review.hidden_count != review.hidden_item_id_refs.len() as u64
        || review.blocked_count != review.blocked_item_id_refs.len() as u64
        || review.disabled_count != review.disabled_item_id_refs.len() as u64
        || review.pending_count != review.pending_item_id_refs.len() as u64
    {
        errors.push(InteractionIntegrityValidationError::BatchScopeDrift {
            batch_scope_id: review.batch_scope_id.clone(),
            reason: "review counts do not match id lists".to_owned(),
        });
    }
    for word in ["included", "excluded", "blocked", "filtered", "hidden"] {
        if !review.batch_review_summary_label.contains(word) {
            errors.push(InteractionIntegrityValidationError::BatchScopeDrift {
                batch_scope_id: review.batch_scope_id.clone(),
                reason: format!("summary label missing {word} count"),
            });
        }
    }
    for finding in review.batch_review_sheet.validate() {
        errors.push(InteractionIntegrityValidationError::BatchScopeDrift {
            batch_scope_id: review.batch_scope_id.clone(),
            reason: finding.summary,
        });
    }
}

fn validate_identity_cues(
    cues: &[ShellIdentityCueRecord],
    errors: &mut Vec<InteractionIntegrityValidationError>,
) {
    let present = cues.iter().map(|cue| cue.cue_role).collect::<BTreeSet<_>>();
    for required in [
        IdentityCueRole::Breadcrumb,
        IdentityCueRole::StatusItem,
        IdentityCueRole::TabIdentity,
        IdentityCueRole::InspectorHeader,
        IdentityCueRole::TerminalHeader,
    ] {
        if !present.contains(&required) {
            errors.push(InteractionIntegrityValidationError::CoverageMissing {
                missing: format!("identity_cue:{}", required.as_str()),
            });
        }
    }
    for cue in cues {
        if cue.record_kind != SHELL_IDENTITY_CUE_RECORD_KIND
            || cue.schema_version != INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION
            || cue.shared_contract_ref != INTERACTION_INTEGRITY_SHARED_CONTRACT_REF
        {
            errors.push(InteractionIntegrityValidationError::IdentityCueUnstable {
                cue_id: cue.cue_id.clone(),
                reason: "record metadata mismatch".to_owned(),
            });
        }
        if cue.display_label.trim().is_empty()
            || cue.canonical_object_id_ref.trim().is_empty()
            || cue.responsive_fallback_modes.is_empty()
        {
            errors.push(InteractionIntegrityValidationError::IdentityCueUnstable {
                cue_id: cue.cue_id.clone(),
                reason: "cue needs display, canonical identity, and fallback modes".to_owned(),
            });
        }
        if !cue.stable_under_resize
            || !cue.stable_under_split
            || !cue.stable_under_detach
            || !cue.stable_under_multi_window_fallback
        {
            errors.push(InteractionIntegrityValidationError::IdentityCueUnstable {
                cue_id: cue.cue_id.clone(),
                reason:
                    "cue must stay stable under resize, split, detach, and multi-window fallback"
                        .to_owned(),
            });
        }
    }
}

fn validate_focus_return_rules(
    rules: &[FocusReturnRuleRecord],
    errors: &mut Vec<InteractionIntegrityValidationError>,
) {
    let present = rules
        .iter()
        .map(|rule| rule.trigger_class)
        .collect::<BTreeSet<_>>();
    for required in [
        FocusReturnTriggerClass::ModalDialogDismiss,
        FocusReturnTriggerClass::SheetDismiss,
        FocusReturnTriggerClass::ApprovalPromptResolve,
        FocusReturnTriggerClass::ActivityCenterJump,
        FocusReturnTriggerClass::PaneCloseOrSplitMerge,
    ] {
        if !present.contains(&required) {
            errors.push(InteractionIntegrityValidationError::CoverageMissing {
                missing: format!("focus_return:{}", required.as_str()),
            });
        }
    }
    for rule in rules {
        if rule.record_kind != FOCUS_RETURN_RULE_RECORD_KIND
            || rule.schema_version != INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION
            || rule.shared_contract_ref != INTERACTION_INTEGRITY_SHARED_CONTRACT_REF
        {
            errors.push(InteractionIntegrityValidationError::FocusReturnInvalid {
                rule_id: rule.rule_id.clone(),
                reason: "record metadata mismatch".to_owned(),
            });
        }
        if !rule.focus_return_explicit
            || !rule.never_returns_to_hidden_or_offscreen
            || !rule.preserves_selection_or_cursor_state
            || rule.screen_reader_announcement.trim().is_empty()
            || rule.expected_return_target_id_ref.trim().is_empty()
            || rule.fallback_return_target_id_ref.trim().is_empty()
        {
            errors.push(InteractionIntegrityValidationError::FocusReturnInvalid {
                rule_id: rule.rule_id.clone(),
                reason: "focus return must be explicit, announced, safe, and state-preserving"
                    .to_owned(),
            });
        }
    }
}

fn validate_vocabulary_parity(
    rows: &[VocabularyParityRecord],
    errors: &mut Vec<InteractionIntegrityValidationError>,
) {
    let required_tokens = required_state_tokens().to_vec();
    let present = rows
        .iter()
        .map(|row| row.surface_class)
        .collect::<BTreeSet<_>>();
    for required in required_surface_classes() {
        if !present.contains(&required) {
            errors.push(InteractionIntegrityValidationError::CoverageMissing {
                missing: format!("vocabulary:{}", required.as_str()),
            });
        }
    }
    for row in rows {
        if row.record_kind != VOCABULARY_PARITY_RECORD_KIND
            || row.schema_version != INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION
            || row.shared_contract_ref != INTERACTION_INTEGRITY_SHARED_CONTRACT_REF
        {
            errors.push(InteractionIntegrityValidationError::VocabularyParityDrift {
                surface_id_ref: row.surface_id_ref.clone(),
                reason: "record metadata mismatch".to_owned(),
            });
        }
        if !row.token_sets_match
            || row.ui_tokens != required_tokens
            || row.keyboard_help_tokens != required_tokens
            || row.accessibility_tokens != required_tokens
            || row.support_export_tokens != required_tokens
        {
            errors.push(InteractionIntegrityValidationError::VocabularyParityDrift {
                surface_id_ref: row.surface_id_ref.clone(),
                reason: "UI, keyboard, accessibility, and support token sets must match".to_owned(),
            });
        }
    }
}

fn validate_support_export(
    packet: &InteractionIntegrityBetaPacket,
    errors: &mut Vec<InteractionIntegrityValidationError>,
) {
    let export = &packet.support_export;
    if export.record_kind != INTERACTION_INTEGRITY_SUPPORT_EXPORT_RECORD_KIND
        || export.schema_version != INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION
        || export.shared_contract_ref != INTERACTION_INTEGRITY_SHARED_CONTRACT_REF
        || export.packet_id_ref != packet.packet_id
    {
        errors.push(InteractionIntegrityValidationError::SupportExportInvalid {
            reason: "record metadata mismatch".to_owned(),
        });
    }
    if export.raw_filter_literals_included
        || export.raw_object_payloads_included
        || export.raw_private_labels_included
    {
        errors.push(InteractionIntegrityValidationError::SupportExportInvalid {
            reason: "support export must be metadata-only".to_owned(),
        });
    }
    if export.support_export_state_tokens != required_state_tokens().to_vec() {
        errors.push(InteractionIntegrityValidationError::SupportExportInvalid {
            reason: "support export state vocabulary is incomplete".to_owned(),
        });
    }
    let expected_targets = sorted_unique(
        packet
            .batch_scope_reviews
            .iter()
            .flat_map(|review| review.resulting_target_id_refs.iter().cloned()),
    );
    if export.resulting_target_id_refs != expected_targets {
        errors.push(InteractionIntegrityValidationError::SupportExportInvalid {
            reason: "resulting target ids drifted from batch reviews".to_owned(),
        });
    }
    for filtered in &export.filtered_scope_id_refs {
        if export.resulting_target_id_refs.contains(filtered) {
            errors.push(InteractionIntegrityValidationError::SupportExportInvalid {
                reason: format!("filtered id {filtered} appears in resulting targets"),
            });
        }
    }
    let batch_scope_ids = packet
        .batch_scope_reviews
        .iter()
        .map(|review| review.batch_scope_id.clone())
        .collect::<Vec<_>>();
    if export.reviewed_batch_scope_refs != batch_scope_ids {
        errors.push(InteractionIntegrityValidationError::SupportExportInvalid {
            reason: "reviewed batch scope refs drifted".to_owned(),
        });
    }
}

fn validate_replay_fixtures(
    fixtures: &[InteractionIntegrityReplayFixture],
    errors: &mut Vec<InteractionIntegrityValidationError>,
) {
    let present = fixtures
        .iter()
        .map(|fixture| fixture.drill_class)
        .collect::<BTreeSet<_>>();
    for required in [
        ReplayDrillClass::VirtualizedSelection,
        ReplayDrillClass::BatchScopeReview,
        ReplayDrillClass::ResponsiveIdentityCue,
        ReplayDrillClass::FocusReturn,
        ReplayDrillClass::SupportExport,
    ] {
        if !present.contains(&required) {
            errors.push(InteractionIntegrityValidationError::CoverageMissing {
                missing: format!("replay:{}", required.as_str()),
            });
        }
    }
    for fixture in fixtures {
        if fixture.record_kind != INTERACTION_INTEGRITY_REPLAY_FIXTURE_RECORD_KIND
            || fixture.schema_version != INTERACTION_INTEGRITY_BETA_SCHEMA_VERSION
            || fixture.shared_contract_ref != INTERACTION_INTEGRITY_SHARED_CONTRACT_REF
        {
            errors.push(InteractionIntegrityValidationError::ReplayFixtureInvalid {
                fixture_id: fixture.fixture_id.clone(),
                reason: "record metadata mismatch".to_owned(),
            });
        }
        if fixture.covered_surface_refs.is_empty()
            || fixture.fixture_refs.is_empty()
            || fixture.expected_support_export_ref.trim().is_empty()
            || fixture.replay_command.trim().is_empty()
        {
            errors.push(InteractionIntegrityValidationError::ReplayFixtureInvalid {
                fixture_id: fixture.fixture_id.clone(),
                reason: "fixture needs surfaces, fixture refs, support export, and replay command"
                    .to_owned(),
            });
        }
    }
}

fn object_state_tokens(
    focused: bool,
    active: bool,
    selected: bool,
    pending: bool,
    disabled: bool,
    filtered: bool,
    hidden: bool,
    blocked: bool,
) -> Vec<ObjectInteractionToken> {
    let mut tokens = Vec::new();
    if focused {
        tokens.push(ObjectInteractionToken::Focused);
    }
    if active {
        tokens.push(ObjectInteractionToken::Active);
    }
    if selected {
        tokens.push(ObjectInteractionToken::Selected);
    }
    if pending {
        tokens.push(ObjectInteractionToken::Pending);
    }
    if disabled {
        tokens.push(ObjectInteractionToken::Disabled);
    }
    if filtered {
        tokens.push(ObjectInteractionToken::Filtered);
    }
    if hidden {
        tokens.push(ObjectInteractionToken::Hidden);
    }
    if blocked {
        tokens.push(ObjectInteractionToken::Blocked);
    }
    tokens
}

fn member_disposition(row: &ObjectInteractionStateRecord) -> BatchMemberDisposition {
    if row.blocked || row.disabled {
        BatchMemberDisposition::Blocked
    } else if row.hidden || row.filtered {
        BatchMemberDisposition::Hidden
    } else if row.pending {
        BatchMemberDisposition::Excluded
    } else if row.selected {
        BatchMemberDisposition::Included
    } else {
        BatchMemberDisposition::Excluded
    }
}

fn member_reason(
    row: &ObjectInteractionStateRecord,
    disposition: BatchMemberDisposition,
) -> String {
    match disposition {
        BatchMemberDisposition::Included => "Included in the reviewed action.".to_owned(),
        BatchMemberDisposition::Excluded if row.pending => {
            "Excluded while pending refresh or prior work.".to_owned()
        }
        BatchMemberDisposition::Excluded => "Excluded from the selected batch scope.".to_owned(),
        BatchMemberDisposition::Blocked => row
            .blocked_reason_label
            .clone()
            .or_else(|| row.disabled_reason_label.clone())
            .unwrap_or_else(|| "Blocked before the action can continue.".to_owned()),
        BatchMemberDisposition::Hidden => row
            .filtered_reason_label
            .clone()
            .or_else(|| row.hidden_reason_label.clone())
            .unwrap_or_else(|| "Hidden or filtered outside the reviewed scope.".to_owned()),
        BatchMemberDisposition::Stale => "Stale relative to the reviewed basis.".to_owned(),
    }
}

fn collection_surface_family(
    surface_class: ClaimedCollectionSurfaceClass,
) -> CollectionSurfaceFamily {
    match surface_class {
        ClaimedCollectionSurfaceClass::SearchResultsList => {
            CollectionSurfaceFamily::SearchCollection
        }
        ClaimedCollectionSurfaceClass::ReviewQueueGrid => CollectionSurfaceFamily::ReviewCollection,
        ClaimedCollectionSurfaceClass::GitChangeTree => CollectionSurfaceFamily::ReviewCollection,
        ClaimedCollectionSurfaceClass::MarketplaceInventoryGrid => {
            CollectionSurfaceFamily::PackageOrInventoryGrid
        }
        ClaimedCollectionSurfaceClass::ActivityCenterList => {
            CollectionSurfaceFamily::WorkItemCollection
        }
    }
}

const fn action_is_protected(action_class: BatchActionClass) -> bool {
    matches!(
        action_class,
        BatchActionClass::RemoteMutation
            | BatchActionClass::DestructiveMutation
            | BatchActionClass::ExportOrShare
            | BatchActionClass::ProviderOwnedMutation
    )
}

fn required_surface_classes() -> [ClaimedCollectionSurfaceClass; 5] {
    [
        ClaimedCollectionSurfaceClass::SearchResultsList,
        ClaimedCollectionSurfaceClass::ReviewQueueGrid,
        ClaimedCollectionSurfaceClass::GitChangeTree,
        ClaimedCollectionSurfaceClass::MarketplaceInventoryGrid,
        ClaimedCollectionSurfaceClass::ActivityCenterList,
    ]
}

fn required_geometry_classes() -> [CollectionGeometryClass; 4] {
    [
        CollectionGeometryClass::DenseList,
        CollectionGeometryClass::Tree,
        CollectionGeometryClass::Grid,
        CollectionGeometryClass::VirtualizedReviewSurface,
    ]
}

fn required_state_tokens() -> [ObjectInteractionToken; 8] {
    [
        ObjectInteractionToken::Focused,
        ObjectInteractionToken::Active,
        ObjectInteractionToken::Selected,
        ObjectInteractionToken::Pending,
        ObjectInteractionToken::Disabled,
        ObjectInteractionToken::Filtered,
        ObjectInteractionToken::Hidden,
        ObjectInteractionToken::Blocked,
    ]
}

fn sorted_unique<I>(values: I) -> Vec<String>
where
    I: IntoIterator<Item = String>,
{
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates() {
        let packet = seeded_interaction_integrity_beta_packet();
        validate_interaction_integrity_beta_packet(&packet).expect("seed packet must validate");
        assert_eq!(packet.summary.surface_classes_present.len(), 5);
        assert_eq!(packet.summary.state_tokens_present.len(), 8);
    }

    #[test]
    fn hidden_and_filtered_selected_objects_are_not_targets() {
        let packet = seeded_interaction_integrity_beta_packet();
        let hidden_or_filtered = packet
            .object_states
            .iter()
            .filter(|row| row.selected && (row.hidden || row.filtered))
            .map(|row| row.object_id_ref.clone())
            .collect::<BTreeSet<_>>();
        let targets = packet
            .batch_scope_reviews
            .iter()
            .flat_map(|review| review.resulting_target_id_refs.iter().cloned())
            .collect::<BTreeSet<_>>();

        assert!(!hidden_or_filtered.is_empty());
        assert!(hidden_or_filtered.is_disjoint(&targets));
    }

    #[test]
    fn identity_cues_cover_shell_roles() {
        let packet = seeded_interaction_integrity_beta_packet();
        let roles = packet
            .identity_cues
            .iter()
            .map(|cue| cue.cue_role)
            .collect::<BTreeSet<_>>();
        assert!(roles.contains(&IdentityCueRole::Breadcrumb));
        assert!(roles.contains(&IdentityCueRole::StatusItem));
        assert!(roles.contains(&IdentityCueRole::TabIdentity));
        assert!(roles.contains(&IdentityCueRole::InspectorHeader));
        assert!(roles.contains(&IdentityCueRole::TerminalHeader));
    }
}
