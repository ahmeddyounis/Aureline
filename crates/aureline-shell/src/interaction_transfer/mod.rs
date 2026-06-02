//! Beta interaction-transfer packet for editor, diff, review, result-grid,
//! and provider-linked surfaces.
//!
//! The packet in this module is the shell-side join between clipboard payload
//! truth, drag/drop intent advertising, named undo-group attribution, workspace
//! back/forward history, and reopen-source distinction. The companion
//! [`crate::transfer`] module already carries the alpha vocabulary for
//! per-action records minted by editor, terminal, search, review, and
//! workspace-admission surfaces; this module composes the alpha vocabulary into
//! one reviewable beta record family that surfaces on dense editor/diff/review,
//! result-grid (search/problems/log/work-item), and provider-linked
//! (extension/marketplace/identity) panes can emit before consequential copy,
//! drop, undo, back, forward, or reopen actions.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Schema version exported by interaction-transfer beta records.
pub const INTERACTION_TRANSFER_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by shell, support, docs, and replay fixtures.
pub const INTERACTION_TRANSFER_SHARED_CONTRACT_REF: &str = "shell:interaction_transfer_beta:v1";

/// Stable record kind for [`InteractionTransferBetaPacket`].
pub const INTERACTION_TRANSFER_PACKET_RECORD_KIND: &str =
    "shell_interaction_transfer_beta_packet_record";

/// Stable record kind for [`ClipboardPayloadClassRecord`].
pub const CLIPBOARD_PAYLOAD_CLASS_RECORD_KIND: &str = "clipboard_payload_class_record";

/// Stable record kind for [`DropIntentRecord`].
pub const DROP_INTENT_RECORD_KIND: &str = "drop_intent_record";

/// Stable record kind for [`ReopenHistoryEntryRecord`].
pub const REOPEN_HISTORY_ENTRY_RECORD_KIND: &str = "reopen_history_entry_record";

/// Stable record kind for [`UndoGroupAttributionRecord`].
pub const UNDO_GROUP_ATTRIBUTION_RECORD_KIND: &str = "undo_group_attribution_record";

/// Stable record kind for [`BackForwardEntryRecord`].
pub const BACK_FORWARD_ENTRY_RECORD_KIND: &str = "back_forward_entry_record";

/// Stable record kind for [`InteractionTransferSupportExport`].
pub const INTERACTION_TRANSFER_SUPPORT_EXPORT_RECORD_KIND: &str =
    "interaction_transfer_support_export_record";

/// Beta-claimed surface kinds covered by the interaction-transfer packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionSurfaceClass {
    /// Editor canvas, buffer, group, or tab.
    Editor,
    /// Diff or compare surface.
    Diff,
    /// Review queue, pull-request, or change-list surface.
    Review,
    /// Result grid: search results, problems, log views, work-item boards,
    /// admin grids.
    ResultGrid,
    /// Provider-linked surface: extension/marketplace, identity, or remote
    /// provider listing.
    ProviderLinked,
}

impl InteractionSurfaceClass {
    /// Stable token used in UI, keyboard help, accessibility, and support
    /// export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Diff => "diff",
            Self::Review => "review",
            Self::ResultGrid => "result_grid",
            Self::ProviderLinked => "provider_linked",
        }
    }
}

/// Representation class declared on the default copy and any non-default
/// rich/context variants. Matches the alpha
/// [`crate::transfer::TransferRepresentationClass`] vocabulary so support
/// export and headless drills stay aligned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PayloadRepresentationClass {
    /// Plain text suitable for default clipboard writes.
    PlainText,
    /// Source or raw identifier that has passed safety review.
    RawSafe,
    /// Rendered representation rather than source identity.
    Rendered,
    /// Source plus target, hunk, query, or provenance context.
    WithContext,
    /// Source representation with controls or metacharacters escaped.
    Escaped,
    /// Sanitized inert snapshot.
    Sanitized,
    /// Redacted snapshot intended for support export.
    Redacted,
    /// Metadata envelope with the raw body withheld.
    MetadataOnly,
}

impl PayloadRepresentationClass {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlainText => "plain_text",
            Self::RawSafe => "raw_safe",
            Self::Rendered => "rendered",
            Self::WithContext => "with_context",
            Self::Escaped => "escaped",
            Self::Sanitized => "sanitized",
            Self::Redacted => "redacted",
            Self::MetadataOnly => "metadata_only",
        }
    }
}

/// Clipboard route posture disclosed when it materially changes behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClipboardRoutePosture {
    /// Local desktop clipboard.
    LocalSystem,
    /// Remote host clipboard bridge.
    RemoteBridge,
    /// Editor-owned named register or search register.
    NamedRegister,
    /// Clipboard route blocked by policy or trust posture.
    PolicyBlocked,
}

impl ClipboardRoutePosture {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSystem => "local_system",
            Self::RemoteBridge => "remote_bridge",
            Self::NamedRegister => "named_register",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Sensitive-payload review posture for tokens, private paths, support links,
/// and similarly risky bodies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SensitiveCopyPosture {
    /// Payload is not sensitive; default copy proceeds.
    NotSensitive,
    /// Label-first preview is required before clipboard write.
    LabelFirstPreview,
    /// Clipboard write is blocked; only metadata or sanitized form may transfer.
    WriteBlocked,
}

impl SensitiveCopyPosture {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotSensitive => "not_sensitive",
            Self::LabelFirstPreview => "label_first_preview",
            Self::WriteBlocked => "write_blocked",
        }
    }
}

/// Drop verb advertised inline on every drop target before commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DropVerb {
    /// Move the source into the target scope.
    Move,
    /// Copy the source into the target scope.
    Copy,
    /// Attach the source as evidence on the target surface.
    Attach,
    /// Open the source in place.
    Open,
    /// Import or extract the source.
    Import,
    /// Split into another editor group or window.
    Split,
    /// Drop is denied; the verb advertised is the refusal explanation.
    Blocked,
}

impl DropVerb {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Move => "move",
            Self::Copy => "copy",
            Self::Attach => "attach",
            Self::Open => "open",
            Self::Import => "import",
            Self::Split => "split",
            Self::Blocked => "blocked",
        }
    }
}

/// Modifier-key cue displayed inline with the drop verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModifierCue {
    /// No modifier active.
    None,
    /// Holding the platform copy modifier flips move to copy.
    HoldCopyModifier,
    /// Holding the platform move modifier flips copy to move.
    HoldMoveModifier,
    /// Holding the platform split modifier targets a new group or window.
    HoldSplitModifier,
    /// Holding the platform link/import modifier requests import semantics.
    HoldImportModifier,
}

impl ModifierCue {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::HoldCopyModifier => "hold_copy_modifier",
            Self::HoldMoveModifier => "hold_move_modifier",
            Self::HoldSplitModifier => "hold_split_modifier",
            Self::HoldImportModifier => "hold_import_modifier",
        }
    }
}

/// Scope of a named undo or recovery group registered against a broad
/// mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndoGroupScope {
    /// Multi-file replace (search-and-replace, refactor, or rename).
    MultiFileReplace,
    /// Settings import.
    SettingsImport,
    /// AI apply suggestion mutation.
    AiApply,
    /// Extension-supplied refactor or codemod.
    ExtensionRefactor,
    /// Other broad mutation that registers a single named group.
    OtherBroadMutation,
    /// Surface cannot register undo; the action must declare preview or
    /// checkpoint posture.
    NoUndoAvailable,
}

impl UndoGroupScope {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MultiFileReplace => "multi_file_replace",
            Self::SettingsImport => "settings_import",
            Self::AiApply => "ai_apply",
            Self::ExtensionRefactor => "extension_refactor",
            Self::OtherBroadMutation => "other_broad_mutation",
            Self::NoUndoAvailable => "no_undo_available",
        }
    }
}

/// Posture declared by a surface that cannot register undo for an action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoUndoPosture {
    /// A preview review must complete before the action commits.
    PreviewBeforeCommit,
    /// A restore checkpoint must be created or verified before the action
    /// commits.
    CheckpointBeforeCommit,
    /// The action is refused until a preview or checkpoint posture exists.
    Refused,
}

impl NoUndoPosture {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewBeforeCommit => "preview_before_commit",
            Self::CheckpointBeforeCommit => "checkpoint_before_commit",
            Self::Refused => "refused",
        }
    }
}

/// Source class of a reopen or recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenSourceClass {
    /// The user closed the surface intentionally and later reopened it.
    ClosedIntentionally,
    /// The surface returned via workspace-scoped back/forward navigation.
    BackForwardNavigation,
    /// The surface was recovered after an abnormal termination.
    CrashRecovery,
    /// The surface was recovered after transport or runtime disconnect.
    DisconnectRecovery,
    /// The surface returned as a placeholder while live authority was missing.
    PlaceholderReopen,
}

impl ReopenSourceClass {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClosedIntentionally => "closed_intentionally",
            Self::BackForwardNavigation => "back_forward_navigation",
            Self::CrashRecovery => "crash_recovery",
            Self::DisconnectRecovery => "disconnect_recovery",
            Self::PlaceholderReopen => "placeholder_reopen",
        }
    }
}

/// Direction of a workspace-scoped back/forward entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackForwardDirection {
    /// Moves to a prior entry in the workspace history.
    Back,
    /// Moves to a later entry in the workspace history.
    Forward,
}

impl BackForwardDirection {
    /// Stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Back => "back",
            Self::Forward => "forward",
        }
    }
}

/// Non-default copy variant declared by a surface alongside its default
/// plain-text copy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyVariantDisclosure {
    /// Opaque variant id.
    pub variant_id_ref: String,
    /// Short human-readable label shown in the copy menu.
    pub variant_label: String,
    /// Representation class declared on the variant.
    pub representation_class: PayloadRepresentationClass,
    /// Stable token for [`Self::representation_class`].
    pub representation_class_token: String,
    /// True when the variant copies rendered or redacted form instead of
    /// raw/source truth.
    pub diverges_from_source_truth: bool,
}

impl CopyVariantDisclosure {
    /// Builds a copy variant with stable tokens included.
    pub fn new(
        variant_id_ref: impl Into<String>,
        variant_label: impl Into<String>,
        representation_class: PayloadRepresentationClass,
        diverges_from_source_truth: bool,
    ) -> Self {
        Self {
            variant_id_ref: variant_id_ref.into(),
            variant_label: variant_label.into(),
            representation_class,
            representation_class_token: representation_class.as_str().to_owned(),
            diverges_from_source_truth,
        }
    }
}

/// Clipboard payload class declared on a copy or export-adjacent action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClipboardPayloadClassRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque payload class id.
    pub payload_class_id: String,
    /// Surface that emits the payload.
    pub surface_class: InteractionSurfaceClass,
    /// Stable token for [`Self::surface_class`].
    pub surface_class_token: String,
    /// Source ref of the object being copied.
    pub source_ref: String,
    /// Redaction-aware label that names the source object.
    pub source_label: String,
    /// Representation class declared on the default copy.
    pub default_representation_class: PayloadRepresentationClass,
    /// Stable token for [`Self::default_representation_class`].
    pub default_representation_class_token: String,
    /// Short label that names what the default copy will produce.
    pub default_representation_label: String,
    /// Non-default copy variants advertised on this surface.
    pub variant_disclosures: Vec<CopyVariantDisclosure>,
    /// Clipboard route posture disclosed for this payload.
    pub clipboard_route: ClipboardRoutePosture,
    /// Stable token for [`Self::clipboard_route`].
    pub clipboard_route_token: String,
    /// True when the clipboard route reveal materially changes user behavior.
    pub route_disclosure_material: bool,
    /// Sensitive-payload posture for this class.
    pub sensitive_posture: SensitiveCopyPosture,
    /// Stable token for [`Self::sensitive_posture`].
    pub sensitive_posture_token: String,
    /// Sensitive value-class tokens detected on this payload.
    pub sensitive_value_class_tokens: Vec<String>,
    /// Command id that opens the sensitive-copy review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sensitive_preview_action_id: Option<String>,
    /// True when the preview must render labels before any bytes leave the
    /// product.
    pub label_first_path: bool,
    /// True when the clipboard write is deferred until the review continues.
    pub clipboard_write_deferred_until_review: bool,
    /// True when the surface can paste safely into terminals, reviews, issue
    /// trackers, and support flows without hidden formatting traps.
    pub paste_targets_neutral: bool,
}

/// Drop intent declared by a target slot before drop commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DropIntentRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque intent id.
    pub intent_id: String,
    /// Surface that hosts the drop target.
    pub surface_class: InteractionSurfaceClass,
    /// Stable token for [`Self::surface_class`].
    pub surface_class_token: String,
    /// Stable drop-target slot ref.
    pub target_slot_ref: String,
    /// Redaction-aware label for the destination scope.
    pub destination_scope_label: String,
    /// Verb advertised inline before drop completes.
    pub advertised_verb: DropVerb,
    /// Stable token for [`Self::advertised_verb`].
    pub advertised_verb_token: String,
    /// Modifier-key cue shown alongside the verb.
    pub modifier_cue: ModifierCue,
    /// Stable token for [`Self::modifier_cue`].
    pub modifier_cue_token: String,
    /// Short label that names what the modifier currently means.
    pub modifier_meaning_label: String,
    /// True when the drop will mutate broad workspace state.
    pub broad_workspace_mutation: bool,
    /// True when a checkpoint is created or verified before commit.
    pub checkpoint_before_commit: bool,
    /// True when a collision-or-overwrite review is part of the drop.
    pub collision_or_overwrite_review: bool,
    /// True when policy or trust blocks the drop before commit.
    pub blocked_before_commit: bool,
    /// Short refusal label shown when the drop is blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blocked_reason_label: Option<String>,
    /// True when the verb advertisement is visible to assistive tech.
    pub verb_announced_to_a11y: bool,
}

/// Named undo group bound to a broad mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UndoGroupAttributionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque undo-group id.
    pub undo_group_id: String,
    /// Surface that owns the undo group.
    pub surface_class: InteractionSurfaceClass,
    /// Stable token for [`Self::surface_class`].
    pub surface_class_token: String,
    /// Group scope class.
    pub group_scope: UndoGroupScope,
    /// Stable token for [`Self::group_scope`].
    pub group_scope_token: String,
    /// Source attribution label shown on the undo entry (e.g. command id,
    /// extension id, AI session id).
    pub source_attribution_label: String,
    /// Stable command id that registered the group.
    pub command_id_ref: String,
    /// True when the group covers more than one file or step.
    pub multi_file_or_multi_step: bool,
    /// True when the group exposes a single reviewable undo entry rather than
    /// many opaque one-step undos.
    pub single_reviewable_undo_entry: bool,
    /// No-undo posture when the surface declared that undo is unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_undo_posture: Option<NoUndoPosture>,
    /// Stable token for [`Self::no_undo_posture`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_undo_posture_token: Option<String>,
    /// Short label that names the no-undo posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_undo_posture_label: Option<String>,
}

/// Workspace-scoped back/forward entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackForwardEntryRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque entry id.
    pub entry_id: String,
    /// Surface that recorded the entry.
    pub surface_class: InteractionSurfaceClass,
    /// Stable token for [`Self::surface_class`].
    pub surface_class_token: String,
    /// Direction declared for this entry relative to the cursor.
    pub direction: BackForwardDirection,
    /// Stable token for [`Self::direction`].
    pub direction_token: String,
    /// Target identity restored when this entry is followed.
    pub target_identity_ref: String,
    /// Short label that names the restored target.
    pub target_identity_label: String,
    /// Workspace scope label visible alongside the entry.
    pub workspace_scope_label: String,
    /// Recorded timestamp (UTC, monotonic-source).
    pub recorded_at: String,
    /// Short source label shown on the entry (command id, navigation route).
    pub source_label: String,
    /// True when the entry preserves cursor or selection identity.
    pub preserves_selection_or_cursor: bool,
}

/// Reopen-history entry persisted in workspace state and visible in the
/// reopen-history surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenHistoryEntryRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque history-entry id.
    pub history_entry_id: String,
    /// Surface that owns the closed/recovered object.
    pub surface_class: InteractionSurfaceClass,
    /// Stable token for [`Self::surface_class`].
    pub surface_class_token: String,
    /// Source class for this reopen entry.
    pub source_class: ReopenSourceClass,
    /// Stable token for [`Self::source_class`].
    pub source_class_token: String,
    /// Opaque closed-object ref.
    pub closed_object_ref: String,
    /// Short closed-object label.
    pub closed_object_label: String,
    /// Reopen command id.
    pub reopen_command_id: String,
    /// Target identity that will be restored.
    pub restored_target_identity_ref: String,
    /// Selection or hunk ref restored when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored_selection_ref: Option<String>,
    /// Scroll-anchor ref restored when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored_scroll_anchor_ref: Option<String>,
    /// Closed-at timestamp.
    pub closed_at: String,
    /// Last-known activity timestamp when distinct from closed-at.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_activity_at: Option<String>,
    /// Short label disclosing whether live authority truly survived.
    pub continuity_label: String,
    /// True when live authority truly survived; false when a placeholder is
    /// shown.
    pub restored_live_authority: bool,
    /// True when rerun, replay, or duplicate-effect commit is forbidden.
    pub auto_rerun_forbidden: bool,
    /// True when the entry is workspace-scoped (visible in this workspace
    /// only) rather than global.
    pub workspace_scoped: bool,
    /// Short label that names the source of the reopen entry on the surface.
    pub source_label: String,
}

/// Metadata-only support export for the interaction-transfer beta packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionTransferSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Packet id referenced by this export.
    pub packet_id_ref: String,
    /// Clipboard payload class ids included.
    pub clipboard_payload_class_ids: Vec<String>,
    /// Drop intent ids included.
    pub drop_intent_ids: Vec<String>,
    /// Undo group ids included.
    pub undo_group_ids: Vec<String>,
    /// Back/forward entry ids included.
    pub back_forward_entry_ids: Vec<String>,
    /// Reopen-history entry ids included.
    pub reopen_history_entry_ids: Vec<String>,
    /// True only when a separate high-friction raw-body path was used. Always
    /// false in the metadata-only export.
    pub raw_payload_bodies_included: bool,
    /// Omitted payload classes named explicitly for support.
    pub omitted_payload_classes: Vec<String>,
    /// Schema refs support tooling can use to decode the records.
    pub schema_refs: Vec<String>,
}

impl InteractionTransferSupportExport {
    /// Builds the metadata-only support export.
    pub fn metadata_only(
        packet_id_ref: impl Into<String>,
        clipboard_payload_class_ids: Vec<String>,
        drop_intent_ids: Vec<String>,
        undo_group_ids: Vec<String>,
        back_forward_entry_ids: Vec<String>,
        reopen_history_entry_ids: Vec<String>,
    ) -> Self {
        Self {
            record_kind: INTERACTION_TRANSFER_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: INTERACTION_TRANSFER_BETA_SCHEMA_VERSION,
            shared_contract_ref: INTERACTION_TRANSFER_SHARED_CONTRACT_REF.to_owned(),
            packet_id_ref: packet_id_ref.into(),
            clipboard_payload_class_ids,
            drop_intent_ids,
            undo_group_ids,
            back_forward_entry_ids,
            reopen_history_entry_ids,
            raw_payload_bodies_included: false,
            omitted_payload_classes: vec![
                "raw_clipboard_body".to_owned(),
                "raw_file_body".to_owned(),
                "raw_drop_payload".to_owned(),
                "raw_private_path".to_owned(),
                "raw_provider_token".to_owned(),
            ],
            schema_refs: vec![
                "schemas/ux/clipboard_payload_class.schema.json".to_owned(),
                "schemas/ux/drop_intent.schema.json".to_owned(),
                "schemas/ux/reopen_history_entry.schema.json".to_owned(),
                "schemas/events/transfer_action.schema.json".to_owned(),
            ],
        }
    }
}

/// Coverage summary computed by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionTransferCoverageSummary {
    /// Surfaces represented in the packet.
    pub surfaces_present: Vec<InteractionSurfaceClass>,
    /// True when every covered surface has a default plain-text copy.
    pub every_surface_default_plain_text: bool,
    /// True when at least one drop intent advertises move/copy distinction.
    pub move_copy_distinction_advertised: bool,
    /// True when at least one drop intent advertises attach/open distinction.
    pub attach_open_distinction_advertised: bool,
    /// True when at least one undo group covers a multi-file or multi-step
    /// broad mutation.
    pub broad_mutation_undo_group_covered: bool,
    /// True when at least one surface declares a no-undo preview/checkpoint
    /// posture.
    pub no_undo_preview_or_checkpoint_covered: bool,
    /// True when back/forward entries exist in both directions.
    pub back_forward_both_directions_covered: bool,
    /// True when reopen history distinguishes intentional close from
    /// crash/disconnect recovery.
    pub intentional_versus_recovery_distinguished: bool,
    /// True when reopen history includes a placeholder-reopen entry.
    pub placeholder_reopen_covered: bool,
}

/// Beta packet that joins clipboard, drop, undo, back/forward, and reopen
/// truth for editor/diff/review/result-grid/provider-linked surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InteractionTransferBetaPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque packet id.
    pub packet_id: String,
    /// Fixture or monotonic timestamp.
    pub minted_at: String,
    /// Source contracts consumed to build this packet.
    pub source_contract_refs: Vec<String>,
    /// Clipboard payload-class rows.
    pub clipboard_payload_classes: Vec<ClipboardPayloadClassRecord>,
    /// Drop intent rows.
    pub drop_intents: Vec<DropIntentRecord>,
    /// Named undo-group attribution rows.
    pub undo_groups: Vec<UndoGroupAttributionRecord>,
    /// Back/forward entries.
    pub back_forward_entries: Vec<BackForwardEntryRecord>,
    /// Reopen-history entries.
    pub reopen_history_entries: Vec<ReopenHistoryEntryRecord>,
    /// Metadata-only support export.
    pub support_export: InteractionTransferSupportExport,
    /// Coverage summary computed by the validator.
    pub summary: InteractionTransferCoverageSummary,
}

/// Validation error returned by
/// [`validate_interaction_transfer_beta_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InteractionTransferValidationError {
    /// Packet metadata does not match shared constants.
    PacketMetadataMismatch { reason: String },
    /// Surface coverage missing for the named scope.
    CoverageMissing { missing: String },
    /// A clipboard payload-class row failed validation.
    ClipboardPayloadInvalid {
        payload_class_id: String,
        reason: String,
    },
    /// A drop intent row failed validation.
    DropIntentInvalid { intent_id: String, reason: String },
    /// A named undo-group row failed validation.
    UndoGroupInvalid {
        undo_group_id: String,
        reason: String,
    },
    /// A back/forward entry failed validation.
    BackForwardInvalid { entry_id: String, reason: String },
    /// A reopen-history entry failed validation.
    ReopenHistoryInvalid {
        history_entry_id: String,
        reason: String,
    },
    /// Support export drifted from the packet contents.
    SupportExportInvalid { reason: String },
}

impl std::fmt::Display for InteractionTransferValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PacketMetadataMismatch { reason } => {
                write!(f, "packet metadata mismatch: {reason}")
            }
            Self::CoverageMissing { missing } => write!(f, "coverage missing: {missing}"),
            Self::ClipboardPayloadInvalid {
                payload_class_id,
                reason,
            } => write!(f, "clipboard payload {payload_class_id} invalid: {reason}"),
            Self::DropIntentInvalid { intent_id, reason } => {
                write!(f, "drop intent {intent_id} invalid: {reason}")
            }
            Self::UndoGroupInvalid {
                undo_group_id,
                reason,
            } => write!(f, "undo group {undo_group_id} invalid: {reason}"),
            Self::BackForwardInvalid { entry_id, reason } => {
                write!(f, "back/forward entry {entry_id} invalid: {reason}")
            }
            Self::ReopenHistoryInvalid {
                history_entry_id,
                reason,
            } => write!(
                f,
                "reopen history entry {history_entry_id} invalid: {reason}"
            ),
            Self::SupportExportInvalid { reason } => {
                write!(f, "support export invalid: {reason}")
            }
        }
    }
}

impl std::error::Error for InteractionTransferValidationError {}

/// Returns the full set of surface classes the beta packet must cover.
pub fn required_surfaces() -> [InteractionSurfaceClass; 5] {
    [
        InteractionSurfaceClass::Editor,
        InteractionSurfaceClass::Diff,
        InteractionSurfaceClass::Review,
        InteractionSurfaceClass::ResultGrid,
        InteractionSurfaceClass::ProviderLinked,
    ]
}

/// Returns the set of broad-mutation undo-group scopes the packet must cover.
pub fn required_broad_mutation_scopes() -> [UndoGroupScope; 4] {
    [
        UndoGroupScope::MultiFileReplace,
        UndoGroupScope::SettingsImport,
        UndoGroupScope::AiApply,
        UndoGroupScope::ExtensionRefactor,
    ]
}

/// Validates the beta packet and returns the full list of violations.
pub fn validate_interaction_transfer_beta_packet(
    packet: &InteractionTransferBetaPacket,
) -> Result<(), Vec<InteractionTransferValidationError>> {
    let mut errors = Vec::new();
    validate_packet_metadata(packet, &mut errors);
    validate_clipboard_payloads(packet, &mut errors);
    validate_drop_intents(packet, &mut errors);
    validate_undo_groups(packet, &mut errors);
    validate_back_forward(packet, &mut errors);
    validate_reopen_history(packet, &mut errors);
    validate_support_export(packet, &mut errors);
    validate_summary(packet, &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn validate_packet_metadata(
    packet: &InteractionTransferBetaPacket,
    errors: &mut Vec<InteractionTransferValidationError>,
) {
    if packet.record_kind != INTERACTION_TRANSFER_PACKET_RECORD_KIND
        || packet.schema_version != INTERACTION_TRANSFER_BETA_SCHEMA_VERSION
        || packet.shared_contract_ref != INTERACTION_TRANSFER_SHARED_CONTRACT_REF
    {
        errors.push(InteractionTransferValidationError::PacketMetadataMismatch {
            reason: "packet record metadata does not match shared constants".to_owned(),
        });
    }
    if packet.packet_id.trim().is_empty() || packet.minted_at.trim().is_empty() {
        errors.push(InteractionTransferValidationError::PacketMetadataMismatch {
            reason: "packet id and minted_at must be set".to_owned(),
        });
    }
    if packet.source_contract_refs.is_empty() {
        errors.push(InteractionTransferValidationError::PacketMetadataMismatch {
            reason: "packet must declare at least one source_contract_ref".to_owned(),
        });
    }
}

fn validate_clipboard_payloads(
    packet: &InteractionTransferBetaPacket,
    errors: &mut Vec<InteractionTransferValidationError>,
) {
    let mut default_surfaces = BTreeSet::new();
    for row in &packet.clipboard_payload_classes {
        if row.record_kind != CLIPBOARD_PAYLOAD_CLASS_RECORD_KIND
            || row.schema_version != INTERACTION_TRANSFER_BETA_SCHEMA_VERSION
            || row.shared_contract_ref != INTERACTION_TRANSFER_SHARED_CONTRACT_REF
        {
            errors.push(
                InteractionTransferValidationError::ClipboardPayloadInvalid {
                    payload_class_id: row.payload_class_id.clone(),
                    reason: "record metadata mismatch".to_owned(),
                },
            );
        }
        if row.default_representation_class != PayloadRepresentationClass::PlainText {
            errors.push(
                InteractionTransferValidationError::ClipboardPayloadInvalid {
                    payload_class_id: row.payload_class_id.clone(),
                    reason: "default copy must be plain text".to_owned(),
                },
            );
        }
        if row.surface_class_token != row.surface_class.as_str()
            || row.default_representation_class_token != row.default_representation_class.as_str()
            || row.clipboard_route_token != row.clipboard_route.as_str()
            || row.sensitive_posture_token != row.sensitive_posture.as_str()
        {
            errors.push(
                InteractionTransferValidationError::ClipboardPayloadInvalid {
                    payload_class_id: row.payload_class_id.clone(),
                    reason: "stable tokens do not match enum variants".to_owned(),
                },
            );
        }
        match row.sensitive_posture {
            SensitiveCopyPosture::NotSensitive => {
                if !row.sensitive_value_class_tokens.is_empty() {
                    errors.push(
                        InteractionTransferValidationError::ClipboardPayloadInvalid {
                            payload_class_id: row.payload_class_id.clone(),
                            reason:
                                "non-sensitive payload must not declare sensitive value classes"
                                    .to_owned(),
                        },
                    );
                }
            }
            SensitiveCopyPosture::LabelFirstPreview => {
                if row.sensitive_value_class_tokens.is_empty()
                    || !row.label_first_path
                    || !row.clipboard_write_deferred_until_review
                    || row.sensitive_preview_action_id.is_none()
                {
                    errors.push(
                        InteractionTransferValidationError::ClipboardPayloadInvalid {
                            payload_class_id: row.payload_class_id.clone(),
                            reason: "label-first preview posture is incomplete".to_owned(),
                        },
                    );
                }
            }
            SensitiveCopyPosture::WriteBlocked => {
                if row.sensitive_value_class_tokens.is_empty()
                    || row.clipboard_write_deferred_until_review
                {
                    errors.push(
                        InteractionTransferValidationError::ClipboardPayloadInvalid {
                            payload_class_id: row.payload_class_id.clone(),
                            reason: "write-blocked posture must declare sensitive value classes \
                                 and must not defer to a label-first review"
                                .to_owned(),
                        },
                    );
                }
            }
        }
        if !row.paste_targets_neutral {
            errors.push(
                InteractionTransferValidationError::ClipboardPayloadInvalid {
                    payload_class_id: row.payload_class_id.clone(),
                    reason: "default copy must paste safely into terminals, reviews, issue \
                         trackers, and support flows"
                        .to_owned(),
                },
            );
        }
        if matches!(row.clipboard_route, ClipboardRoutePosture::RemoteBridge)
            && !row.route_disclosure_material
        {
            errors.push(
                InteractionTransferValidationError::ClipboardPayloadInvalid {
                    payload_class_id: row.payload_class_id.clone(),
                    reason: "remote clipboard bridge must declare route disclosure as material"
                        .to_owned(),
                },
            );
        }
        default_surfaces.insert(row.surface_class);
    }
    for required in required_surfaces() {
        if !default_surfaces.contains(&required) {
            errors.push(InteractionTransferValidationError::CoverageMissing {
                missing: format!("clipboard_payload:{}", required.as_str()),
            });
        }
    }
}

fn validate_drop_intents(
    packet: &InteractionTransferBetaPacket,
    errors: &mut Vec<InteractionTransferValidationError>,
) {
    let mut surfaces = BTreeSet::new();
    let mut verbs = BTreeSet::new();
    for row in &packet.drop_intents {
        if row.record_kind != DROP_INTENT_RECORD_KIND
            || row.schema_version != INTERACTION_TRANSFER_BETA_SCHEMA_VERSION
            || row.shared_contract_ref != INTERACTION_TRANSFER_SHARED_CONTRACT_REF
        {
            errors.push(InteractionTransferValidationError::DropIntentInvalid {
                intent_id: row.intent_id.clone(),
                reason: "record metadata mismatch".to_owned(),
            });
        }
        if row.target_slot_ref.trim().is_empty()
            || row.destination_scope_label.trim().is_empty()
            || row.modifier_meaning_label.trim().is_empty()
        {
            errors.push(InteractionTransferValidationError::DropIntentInvalid {
                intent_id: row.intent_id.clone(),
                reason: "drop intent must name target slot, destination scope, and modifier \
                         meaning"
                    .to_owned(),
            });
        }
        if row.surface_class_token != row.surface_class.as_str()
            || row.advertised_verb_token != row.advertised_verb.as_str()
            || row.modifier_cue_token != row.modifier_cue.as_str()
        {
            errors.push(InteractionTransferValidationError::DropIntentInvalid {
                intent_id: row.intent_id.clone(),
                reason: "stable tokens do not match enum variants".to_owned(),
            });
        }
        if !row.verb_announced_to_a11y {
            errors.push(InteractionTransferValidationError::DropIntentInvalid {
                intent_id: row.intent_id.clone(),
                reason: "drop verb must be announced to accessibility tech".to_owned(),
            });
        }
        if matches!(row.advertised_verb, DropVerb::Blocked) {
            if !row.blocked_before_commit || row.blocked_reason_label.is_none() {
                errors.push(InteractionTransferValidationError::DropIntentInvalid {
                    intent_id: row.intent_id.clone(),
                    reason: "blocked drop must declare blocked_before_commit and a reason label"
                        .to_owned(),
                });
            }
        } else if row.blocked_before_commit {
            errors.push(InteractionTransferValidationError::DropIntentInvalid {
                intent_id: row.intent_id.clone(),
                reason: "non-blocked verb must not declare blocked_before_commit".to_owned(),
            });
        }
        if row.broad_workspace_mutation && !row.checkpoint_before_commit {
            errors.push(InteractionTransferValidationError::DropIntentInvalid {
                intent_id: row.intent_id.clone(),
                reason: "broad workspace mutation must create or verify a checkpoint before \
                         commit"
                    .to_owned(),
            });
        }
        surfaces.insert(row.surface_class);
        if !matches!(row.advertised_verb, DropVerb::Blocked) {
            verbs.insert(row.advertised_verb);
        }
    }
    for required in required_surfaces() {
        if !surfaces.contains(&required) {
            errors.push(InteractionTransferValidationError::CoverageMissing {
                missing: format!("drop_intent:{}", required.as_str()),
            });
        }
    }
    for required in [
        DropVerb::Move,
        DropVerb::Copy,
        DropVerb::Attach,
        DropVerb::Open,
        DropVerb::Import,
        DropVerb::Split,
    ] {
        if !verbs.contains(&required) {
            errors.push(InteractionTransferValidationError::CoverageMissing {
                missing: format!("drop_intent_verb:{}", required.as_str()),
            });
        }
    }
}

fn validate_undo_groups(
    packet: &InteractionTransferBetaPacket,
    errors: &mut Vec<InteractionTransferValidationError>,
) {
    let mut scopes = BTreeSet::new();
    let mut no_undo_declared = false;
    for row in &packet.undo_groups {
        if row.record_kind != UNDO_GROUP_ATTRIBUTION_RECORD_KIND
            || row.schema_version != INTERACTION_TRANSFER_BETA_SCHEMA_VERSION
            || row.shared_contract_ref != INTERACTION_TRANSFER_SHARED_CONTRACT_REF
        {
            errors.push(InteractionTransferValidationError::UndoGroupInvalid {
                undo_group_id: row.undo_group_id.clone(),
                reason: "record metadata mismatch".to_owned(),
            });
        }
        if row.source_attribution_label.trim().is_empty() || row.command_id_ref.trim().is_empty() {
            errors.push(InteractionTransferValidationError::UndoGroupInvalid {
                undo_group_id: row.undo_group_id.clone(),
                reason: "undo group must name source attribution and command id".to_owned(),
            });
        }
        if row.surface_class_token != row.surface_class.as_str()
            || row.group_scope_token != row.group_scope.as_str()
        {
            errors.push(InteractionTransferValidationError::UndoGroupInvalid {
                undo_group_id: row.undo_group_id.clone(),
                reason: "stable tokens do not match enum variants".to_owned(),
            });
        }
        match row.group_scope {
            UndoGroupScope::NoUndoAvailable => {
                no_undo_declared = true;
                match (row.no_undo_posture, row.no_undo_posture_token.as_deref()) {
                    (Some(posture), Some(token))
                        if token == posture.as_str()
                            && !matches!(posture, NoUndoPosture::Refused) => {}
                    (Some(NoUndoPosture::Refused), Some(token))
                        if token == NoUndoPosture::Refused.as_str() => {}
                    _ => {
                        errors.push(InteractionTransferValidationError::UndoGroupInvalid {
                            undo_group_id: row.undo_group_id.clone(),
                            reason: "no-undo scope must declare a posture and matching token"
                                .to_owned(),
                        });
                    }
                }
                if row
                    .no_undo_posture_label
                    .as_deref()
                    .map(|label| label.trim().is_empty())
                    .unwrap_or(true)
                {
                    errors.push(InteractionTransferValidationError::UndoGroupInvalid {
                        undo_group_id: row.undo_group_id.clone(),
                        reason: "no-undo scope must declare a posture label".to_owned(),
                    });
                }
                if row.single_reviewable_undo_entry {
                    errors.push(InteractionTransferValidationError::UndoGroupInvalid {
                        undo_group_id: row.undo_group_id.clone(),
                        reason: "no-undo scope cannot also be a single reviewable undo entry"
                            .to_owned(),
                    });
                }
            }
            _ => {
                if row.no_undo_posture.is_some()
                    || row.no_undo_posture_token.is_some()
                    || row.no_undo_posture_label.is_some()
                {
                    errors.push(InteractionTransferValidationError::UndoGroupInvalid {
                        undo_group_id: row.undo_group_id.clone(),
                        reason: "non-no-undo scope must not declare a no-undo posture".to_owned(),
                    });
                }
                if !row.single_reviewable_undo_entry {
                    errors.push(InteractionTransferValidationError::UndoGroupInvalid {
                        undo_group_id: row.undo_group_id.clone(),
                        reason: "broad mutation must register a single reviewable undo entry"
                            .to_owned(),
                    });
                }
            }
        }
        scopes.insert(row.group_scope);
    }
    for required in required_broad_mutation_scopes() {
        if !scopes.contains(&required) {
            errors.push(InteractionTransferValidationError::CoverageMissing {
                missing: format!("undo_group_scope:{}", required.as_str()),
            });
        }
    }
    if !no_undo_declared {
        errors.push(InteractionTransferValidationError::CoverageMissing {
            missing: "undo_group_scope:no_undo_available".to_owned(),
        });
    }
}

fn validate_back_forward(
    packet: &InteractionTransferBetaPacket,
    errors: &mut Vec<InteractionTransferValidationError>,
) {
    let mut directions = BTreeSet::new();
    for row in &packet.back_forward_entries {
        if row.record_kind != BACK_FORWARD_ENTRY_RECORD_KIND
            || row.schema_version != INTERACTION_TRANSFER_BETA_SCHEMA_VERSION
            || row.shared_contract_ref != INTERACTION_TRANSFER_SHARED_CONTRACT_REF
        {
            errors.push(InteractionTransferValidationError::BackForwardInvalid {
                entry_id: row.entry_id.clone(),
                reason: "record metadata mismatch".to_owned(),
            });
        }
        if row.target_identity_ref.trim().is_empty()
            || row.target_identity_label.trim().is_empty()
            || row.workspace_scope_label.trim().is_empty()
            || row.recorded_at.trim().is_empty()
            || row.source_label.trim().is_empty()
        {
            errors.push(InteractionTransferValidationError::BackForwardInvalid {
                entry_id: row.entry_id.clone(),
                reason: "back/forward entry must name target identity, workspace scope, \
                         timestamp, and source label"
                    .to_owned(),
            });
        }
        if row.surface_class_token != row.surface_class.as_str()
            || row.direction_token != row.direction.as_str()
        {
            errors.push(InteractionTransferValidationError::BackForwardInvalid {
                entry_id: row.entry_id.clone(),
                reason: "stable tokens do not match enum variants".to_owned(),
            });
        }
        directions.insert(row.direction);
    }
    for required in [BackForwardDirection::Back, BackForwardDirection::Forward] {
        if !directions.contains(&required) {
            errors.push(InteractionTransferValidationError::CoverageMissing {
                missing: format!("back_forward_direction:{}", required.as_str()),
            });
        }
    }
}

fn validate_reopen_history(
    packet: &InteractionTransferBetaPacket,
    errors: &mut Vec<InteractionTransferValidationError>,
) {
    let mut sources = BTreeSet::new();
    let mut surfaces = BTreeSet::new();
    for row in &packet.reopen_history_entries {
        if row.record_kind != REOPEN_HISTORY_ENTRY_RECORD_KIND
            || row.schema_version != INTERACTION_TRANSFER_BETA_SCHEMA_VERSION
            || row.shared_contract_ref != INTERACTION_TRANSFER_SHARED_CONTRACT_REF
        {
            errors.push(InteractionTransferValidationError::ReopenHistoryInvalid {
                history_entry_id: row.history_entry_id.clone(),
                reason: "record metadata mismatch".to_owned(),
            });
        }
        if row.closed_object_ref.trim().is_empty()
            || row.restored_target_identity_ref.trim().is_empty()
            || row.reopen_command_id.trim().is_empty()
            || row.continuity_label.trim().is_empty()
            || row.source_label.trim().is_empty()
            || row.closed_at.trim().is_empty()
        {
            errors.push(InteractionTransferValidationError::ReopenHistoryInvalid {
                history_entry_id: row.history_entry_id.clone(),
                reason: "reopen entry must name closed object, restored identity, command, \
                         continuity, timestamps, and source label"
                    .to_owned(),
            });
        }
        if row.surface_class_token != row.surface_class.as_str()
            || row.source_class_token != row.source_class.as_str()
        {
            errors.push(InteractionTransferValidationError::ReopenHistoryInvalid {
                history_entry_id: row.history_entry_id.clone(),
                reason: "stable tokens do not match enum variants".to_owned(),
            });
        }
        match row.source_class {
            ReopenSourceClass::CrashRecovery | ReopenSourceClass::DisconnectRecovery => {
                if !row.auto_rerun_forbidden {
                    errors.push(InteractionTransferValidationError::ReopenHistoryInvalid {
                        history_entry_id: row.history_entry_id.clone(),
                        reason: "crash/disconnect recovery must forbid auto rerun".to_owned(),
                    });
                }
                if row.restored_live_authority {
                    errors.push(InteractionTransferValidationError::ReopenHistoryInvalid {
                        history_entry_id: row.history_entry_id.clone(),
                        reason: "crash/disconnect recovery cannot claim live authority".to_owned(),
                    });
                }
            }
            ReopenSourceClass::PlaceholderReopen => {
                if row.restored_live_authority {
                    errors.push(InteractionTransferValidationError::ReopenHistoryInvalid {
                        history_entry_id: row.history_entry_id.clone(),
                        reason: "placeholder reopen cannot claim live authority".to_owned(),
                    });
                }
            }
            _ => {}
        }
        sources.insert(row.source_class);
        surfaces.insert(row.surface_class);
    }
    if !sources.contains(&ReopenSourceClass::ClosedIntentionally) {
        errors.push(InteractionTransferValidationError::CoverageMissing {
            missing: "reopen_source:closed_intentionally".to_owned(),
        });
    }
    let has_recovery = sources.contains(&ReopenSourceClass::CrashRecovery)
        || sources.contains(&ReopenSourceClass::DisconnectRecovery);
    if !has_recovery {
        errors.push(InteractionTransferValidationError::CoverageMissing {
            missing: "reopen_source:crash_or_disconnect_recovery".to_owned(),
        });
    }
    if !sources.contains(&ReopenSourceClass::PlaceholderReopen) {
        errors.push(InteractionTransferValidationError::CoverageMissing {
            missing: "reopen_source:placeholder_reopen".to_owned(),
        });
    }
    if !sources.contains(&ReopenSourceClass::BackForwardNavigation) {
        errors.push(InteractionTransferValidationError::CoverageMissing {
            missing: "reopen_source:back_forward_navigation".to_owned(),
        });
    }
    for required in required_surfaces() {
        if !surfaces.contains(&required) {
            errors.push(InteractionTransferValidationError::CoverageMissing {
                missing: format!("reopen_history_surface:{}", required.as_str()),
            });
        }
    }
}

fn validate_support_export(
    packet: &InteractionTransferBetaPacket,
    errors: &mut Vec<InteractionTransferValidationError>,
) {
    let export = &packet.support_export;
    if export.record_kind != INTERACTION_TRANSFER_SUPPORT_EXPORT_RECORD_KIND
        || export.schema_version != INTERACTION_TRANSFER_BETA_SCHEMA_VERSION
        || export.shared_contract_ref != INTERACTION_TRANSFER_SHARED_CONTRACT_REF
        || export.packet_id_ref != packet.packet_id
    {
        errors.push(InteractionTransferValidationError::SupportExportInvalid {
            reason: "record metadata mismatch".to_owned(),
        });
    }
    if export.raw_payload_bodies_included {
        errors.push(InteractionTransferValidationError::SupportExportInvalid {
            reason: "support export must remain metadata-only".to_owned(),
        });
    }
    let expected_clipboard = sorted_unique(
        packet
            .clipboard_payload_classes
            .iter()
            .map(|row| row.payload_class_id.clone()),
    );
    let expected_drops = sorted_unique(packet.drop_intents.iter().map(|row| row.intent_id.clone()));
    let expected_undo = sorted_unique(
        packet
            .undo_groups
            .iter()
            .map(|row| row.undo_group_id.clone()),
    );
    let expected_back_forward = sorted_unique(
        packet
            .back_forward_entries
            .iter()
            .map(|row| row.entry_id.clone()),
    );
    let expected_reopen = sorted_unique(
        packet
            .reopen_history_entries
            .iter()
            .map(|row| row.history_entry_id.clone()),
    );
    if sorted_unique(export.clipboard_payload_class_ids.iter().cloned()) != expected_clipboard {
        errors.push(InteractionTransferValidationError::SupportExportInvalid {
            reason: "clipboard payload ids drifted from packet contents".to_owned(),
        });
    }
    if sorted_unique(export.drop_intent_ids.iter().cloned()) != expected_drops {
        errors.push(InteractionTransferValidationError::SupportExportInvalid {
            reason: "drop intent ids drifted from packet contents".to_owned(),
        });
    }
    if sorted_unique(export.undo_group_ids.iter().cloned()) != expected_undo {
        errors.push(InteractionTransferValidationError::SupportExportInvalid {
            reason: "undo group ids drifted from packet contents".to_owned(),
        });
    }
    if sorted_unique(export.back_forward_entry_ids.iter().cloned()) != expected_back_forward {
        errors.push(InteractionTransferValidationError::SupportExportInvalid {
            reason: "back/forward entry ids drifted from packet contents".to_owned(),
        });
    }
    if sorted_unique(export.reopen_history_entry_ids.iter().cloned()) != expected_reopen {
        errors.push(InteractionTransferValidationError::SupportExportInvalid {
            reason: "reopen history entry ids drifted from packet contents".to_owned(),
        });
    }
}

fn validate_summary(
    packet: &InteractionTransferBetaPacket,
    errors: &mut Vec<InteractionTransferValidationError>,
) {
    let computed = compute_summary(packet);
    if packet.summary != computed {
        errors.push(InteractionTransferValidationError::PacketMetadataMismatch {
            reason: "coverage summary drifted from computed truth".to_owned(),
        });
    }
}

fn compute_summary(packet: &InteractionTransferBetaPacket) -> InteractionTransferCoverageSummary {
    let surfaces_present: Vec<InteractionSurfaceClass> = packet
        .clipboard_payload_classes
        .iter()
        .map(|row| row.surface_class)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let every_surface_default_plain_text = required_surfaces().iter().all(|surface| {
        packet.clipboard_payload_classes.iter().any(|row| {
            row.surface_class == *surface
                && row.default_representation_class == PayloadRepresentationClass::PlainText
        })
    });
    let drop_verbs: BTreeSet<DropVerb> = packet
        .drop_intents
        .iter()
        .map(|row| row.advertised_verb)
        .collect();
    let move_copy_distinction_advertised =
        drop_verbs.contains(&DropVerb::Move) && drop_verbs.contains(&DropVerb::Copy);
    let attach_open_distinction_advertised =
        drop_verbs.contains(&DropVerb::Attach) && drop_verbs.contains(&DropVerb::Open);
    let broad_mutation_undo_group_covered = packet
        .undo_groups
        .iter()
        .any(|row| row.multi_file_or_multi_step && row.single_reviewable_undo_entry);
    let no_undo_preview_or_checkpoint_covered = packet.undo_groups.iter().any(|row| {
        matches!(row.group_scope, UndoGroupScope::NoUndoAvailable)
            && matches!(
                row.no_undo_posture,
                Some(NoUndoPosture::PreviewBeforeCommit)
                    | Some(NoUndoPosture::CheckpointBeforeCommit)
            )
    });
    let back_forward_directions: BTreeSet<BackForwardDirection> = packet
        .back_forward_entries
        .iter()
        .map(|row| row.direction)
        .collect();
    let back_forward_both_directions_covered = back_forward_directions
        .contains(&BackForwardDirection::Back)
        && back_forward_directions.contains(&BackForwardDirection::Forward);
    let sources: BTreeSet<ReopenSourceClass> = packet
        .reopen_history_entries
        .iter()
        .map(|row| row.source_class)
        .collect();
    let intentional_versus_recovery_distinguished = sources
        .contains(&ReopenSourceClass::ClosedIntentionally)
        && (sources.contains(&ReopenSourceClass::CrashRecovery)
            || sources.contains(&ReopenSourceClass::DisconnectRecovery));
    let placeholder_reopen_covered = sources.contains(&ReopenSourceClass::PlaceholderReopen);

    InteractionTransferCoverageSummary {
        surfaces_present,
        every_surface_default_plain_text,
        move_copy_distinction_advertised,
        attach_open_distinction_advertised,
        broad_mutation_undo_group_covered,
        no_undo_preview_or_checkpoint_covered,
        back_forward_both_directions_covered,
        intentional_versus_recovery_distinguished,
        placeholder_reopen_covered,
    }
}

fn sorted_unique<I: IntoIterator<Item = String>>(values: I) -> Vec<String> {
    values
        .into_iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Returns the seeded beta packet used by tests and the headless inspector.
pub fn seeded_interaction_transfer_beta_packet() -> InteractionTransferBetaPacket {
    let clipboard_payload_classes = seeded_clipboard_payload_classes();
    let drop_intents = seeded_drop_intents();
    let undo_groups = seeded_undo_groups();
    let back_forward_entries = seeded_back_forward_entries();
    let reopen_history_entries = seeded_reopen_history_entries();

    let packet_id = "packet:interaction-transfer-beta".to_owned();
    let clipboard_ids = clipboard_payload_classes
        .iter()
        .map(|row| row.payload_class_id.clone())
        .collect();
    let drop_ids = drop_intents
        .iter()
        .map(|row| row.intent_id.clone())
        .collect();
    let undo_ids = undo_groups
        .iter()
        .map(|row| row.undo_group_id.clone())
        .collect();
    let back_forward_ids = back_forward_entries
        .iter()
        .map(|row| row.entry_id.clone())
        .collect();
    let reopen_ids = reopen_history_entries
        .iter()
        .map(|row| row.history_entry_id.clone())
        .collect();
    let support_export = InteractionTransferSupportExport::metadata_only(
        packet_id.clone(),
        clipboard_ids,
        drop_ids,
        undo_ids,
        back_forward_ids,
        reopen_ids,
    );

    let mut packet = InteractionTransferBetaPacket {
        record_kind: INTERACTION_TRANSFER_PACKET_RECORD_KIND.to_owned(),
        schema_version: INTERACTION_TRANSFER_BETA_SCHEMA_VERSION,
        shared_contract_ref: INTERACTION_TRANSFER_SHARED_CONTRACT_REF.to_owned(),
        packet_id,
        minted_at: "2026-05-19T09:56:36Z".to_owned(),
        source_contract_refs: vec![
            "docs/ux/m3/interaction_transfer_beta.md".to_owned(),
            "docs/ux/clipboard_history_contract.md".to_owned(),
            "docs/ux/copy_export_representation_parity.md".to_owned(),
            "docs/ux/cross_window_transfer_contract.md".to_owned(),
            "docs/ux/shell_close_reopen_contract.md".to_owned(),
            "schemas/ux/clipboard_payload_class.schema.json".to_owned(),
            "schemas/ux/drop_intent.schema.json".to_owned(),
            "schemas/ux/reopen_history_entry.schema.json".to_owned(),
        ],
        clipboard_payload_classes,
        drop_intents,
        undo_groups,
        back_forward_entries,
        reopen_history_entries,
        support_export,
        summary: InteractionTransferCoverageSummary {
            surfaces_present: Vec::new(),
            every_surface_default_plain_text: false,
            move_copy_distinction_advertised: false,
            attach_open_distinction_advertised: false,
            broad_mutation_undo_group_covered: false,
            no_undo_preview_or_checkpoint_covered: false,
            back_forward_both_directions_covered: false,
            intentional_versus_recovery_distinguished: false,
            placeholder_reopen_covered: false,
        },
    };
    packet.summary = compute_summary(&packet);
    packet
}

fn clipboard_row(
    payload_class_id: &str,
    surface_class: InteractionSurfaceClass,
    source_ref: &str,
    source_label: &str,
    default_representation_label: &str,
    variant_disclosures: Vec<CopyVariantDisclosure>,
    clipboard_route: ClipboardRoutePosture,
    route_disclosure_material: bool,
    sensitive_posture: SensitiveCopyPosture,
    sensitive_value_class_tokens: Vec<String>,
    sensitive_preview_action_id: Option<&str>,
    label_first_path: bool,
    clipboard_write_deferred_until_review: bool,
) -> ClipboardPayloadClassRecord {
    ClipboardPayloadClassRecord {
        record_kind: CLIPBOARD_PAYLOAD_CLASS_RECORD_KIND.to_owned(),
        schema_version: INTERACTION_TRANSFER_BETA_SCHEMA_VERSION,
        shared_contract_ref: INTERACTION_TRANSFER_SHARED_CONTRACT_REF.to_owned(),
        payload_class_id: payload_class_id.to_owned(),
        surface_class,
        surface_class_token: surface_class.as_str().to_owned(),
        source_ref: source_ref.to_owned(),
        source_label: source_label.to_owned(),
        default_representation_class: PayloadRepresentationClass::PlainText,
        default_representation_class_token: PayloadRepresentationClass::PlainText
            .as_str()
            .to_owned(),
        default_representation_label: default_representation_label.to_owned(),
        variant_disclosures,
        clipboard_route,
        clipboard_route_token: clipboard_route.as_str().to_owned(),
        route_disclosure_material,
        sensitive_posture,
        sensitive_posture_token: sensitive_posture.as_str().to_owned(),
        sensitive_value_class_tokens,
        sensitive_preview_action_id: sensitive_preview_action_id.map(str::to_owned),
        label_first_path,
        clipboard_write_deferred_until_review,
        paste_targets_neutral: true,
    }
}

fn seeded_clipboard_payload_classes() -> Vec<ClipboardPayloadClassRecord> {
    vec![
        clipboard_row(
            "clipboard:editor:selection",
            InteractionSurfaceClass::Editor,
            "editor:buffer:main.rs",
            "main.rs selection",
            "Copies the raw selected text exactly as written.",
            vec![
                CopyVariantDisclosure::new(
                    "variant:editor:with_path",
                    "Copy with file path",
                    PayloadRepresentationClass::WithContext,
                    false,
                ),
                CopyVariantDisclosure::new(
                    "variant:editor:rendered",
                    "Copy rendered Markdown",
                    PayloadRepresentationClass::Rendered,
                    true,
                ),
            ],
            ClipboardRoutePosture::LocalSystem,
            false,
            SensitiveCopyPosture::NotSensitive,
            Vec::new(),
            None,
            true,
            false,
        ),
        clipboard_row(
            "clipboard:diff:hunk",
            InteractionSurfaceClass::Diff,
            "diff:compare:hunk-42",
            "diff hunk 42",
            "Copies the plain-text hunk lines without diff markers.",
            vec![CopyVariantDisclosure::new(
                "variant:diff:patch",
                "Copy as patch",
                PayloadRepresentationClass::WithContext,
                false,
            )],
            ClipboardRoutePosture::LocalSystem,
            false,
            SensitiveCopyPosture::NotSensitive,
            Vec::new(),
            None,
            true,
            false,
        ),
        clipboard_row(
            "clipboard:review:diagnostic",
            InteractionSurfaceClass::Review,
            "review:diagnostic:row-7",
            "review diagnostic row 7",
            "Copies the plain-text diagnostic message and file:line ref.",
            vec![CopyVariantDisclosure::new(
                "variant:review:with_context",
                "Copy with rule id and code excerpt",
                PayloadRepresentationClass::WithContext,
                false,
            )],
            ClipboardRoutePosture::LocalSystem,
            false,
            SensitiveCopyPosture::NotSensitive,
            Vec::new(),
            None,
            true,
            false,
        ),
        clipboard_row(
            "clipboard:result_grid:search_row",
            InteractionSurfaceClass::ResultGrid,
            "result_grid:search:row-12",
            "search result row 12",
            "Copies one plain-text line per selected row in the grid.",
            vec![
                CopyVariantDisclosure::new(
                    "variant:result_grid:tsv",
                    "Copy as TSV",
                    PayloadRepresentationClass::RawSafe,
                    false,
                ),
                CopyVariantDisclosure::new(
                    "variant:result_grid:rendered",
                    "Copy rendered cells",
                    PayloadRepresentationClass::Rendered,
                    true,
                ),
            ],
            ClipboardRoutePosture::LocalSystem,
            false,
            SensitiveCopyPosture::NotSensitive,
            Vec::new(),
            None,
            true,
            false,
        ),
        clipboard_row(
            "clipboard:provider_linked:support_link",
            InteractionSurfaceClass::ProviderLinked,
            "provider:identity:support-link",
            "provider support link",
            "Copies the privacy-safe support label only.",
            vec![CopyVariantDisclosure::new(
                "variant:provider_linked:redacted",
                "Copy redacted reference",
                PayloadRepresentationClass::Redacted,
                true,
            )],
            ClipboardRoutePosture::RemoteBridge,
            true,
            SensitiveCopyPosture::LabelFirstPreview,
            vec![
                "provider_support_link".to_owned(),
                "private_path".to_owned(),
            ],
            Some("cmd:clipboard.preview_sensitive:provider_linked"),
            true,
            true,
        ),
    ]
}

fn drop_row(
    intent_id: &str,
    surface_class: InteractionSurfaceClass,
    target_slot_ref: &str,
    destination_scope_label: &str,
    advertised_verb: DropVerb,
    modifier_cue: ModifierCue,
    modifier_meaning_label: &str,
    broad_workspace_mutation: bool,
    checkpoint_before_commit: bool,
    collision_or_overwrite_review: bool,
    blocked_before_commit: bool,
    blocked_reason_label: Option<&str>,
) -> DropIntentRecord {
    DropIntentRecord {
        record_kind: DROP_INTENT_RECORD_KIND.to_owned(),
        schema_version: INTERACTION_TRANSFER_BETA_SCHEMA_VERSION,
        shared_contract_ref: INTERACTION_TRANSFER_SHARED_CONTRACT_REF.to_owned(),
        intent_id: intent_id.to_owned(),
        surface_class,
        surface_class_token: surface_class.as_str().to_owned(),
        target_slot_ref: target_slot_ref.to_owned(),
        destination_scope_label: destination_scope_label.to_owned(),
        advertised_verb,
        advertised_verb_token: advertised_verb.as_str().to_owned(),
        modifier_cue,
        modifier_cue_token: modifier_cue.as_str().to_owned(),
        modifier_meaning_label: modifier_meaning_label.to_owned(),
        broad_workspace_mutation,
        checkpoint_before_commit,
        collision_or_overwrite_review,
        blocked_before_commit,
        blocked_reason_label: blocked_reason_label.map(str::to_owned),
        verb_announced_to_a11y: true,
    }
}

fn seeded_drop_intents() -> Vec<DropIntentRecord> {
    vec![
        drop_row(
            "drop_intent:editor:tab_group_split",
            InteractionSurfaceClass::Editor,
            "editor:tab_group:right",
            "right editor group",
            DropVerb::Split,
            ModifierCue::HoldSplitModifier,
            "Hold the split modifier to open the buffer in the right group.",
            false,
            false,
            false,
            false,
            None,
        ),
        drop_row(
            "drop_intent:editor:move_within_workspace",
            InteractionSurfaceClass::Editor,
            "editor:tree:src/lib.rs",
            "current workspace",
            DropVerb::Move,
            ModifierCue::None,
            "Release without a modifier to move the file inside the workspace.",
            false,
            true,
            true,
            false,
            None,
        ),
        drop_row(
            "drop_intent:editor:copy_with_modifier",
            InteractionSurfaceClass::Editor,
            "editor:tree:src/lib.rs",
            "current workspace",
            DropVerb::Copy,
            ModifierCue::HoldCopyModifier,
            "Hold the copy modifier to duplicate the file inside the workspace.",
            false,
            false,
            true,
            false,
            None,
        ),
        drop_row(
            "drop_intent:diff:open_compare_here",
            InteractionSurfaceClass::Diff,
            "diff:compare:slot",
            "diff compare slot",
            DropVerb::Open,
            ModifierCue::None,
            "Release without a modifier to open the source against the compare target.",
            false,
            false,
            false,
            false,
            None,
        ),
        drop_row(
            "drop_intent:review:attach_evidence",
            InteractionSurfaceClass::Review,
            "review:evidence:slot",
            "review evidence bucket",
            DropVerb::Attach,
            ModifierCue::None,
            "Release without a modifier to attach evidence to the review.",
            false,
            false,
            false,
            false,
            None,
        ),
        drop_row(
            "drop_intent:result_grid:add_to_work_item",
            InteractionSurfaceClass::ResultGrid,
            "result_grid:work_item:slot",
            "work item board",
            DropVerb::Import,
            ModifierCue::HoldImportModifier,
            "Hold the import modifier to import the row into the work-item board.",
            true,
            true,
            true,
            false,
            None,
        ),
        drop_row(
            "drop_intent:provider_linked:blocked",
            InteractionSurfaceClass::ProviderLinked,
            "provider:extension:install_slot",
            "extension install slot",
            DropVerb::Blocked,
            ModifierCue::None,
            "Drop is blocked by policy; release does nothing.",
            false,
            false,
            false,
            true,
            Some("Workspace policy refuses unsigned extension drops here."),
        ),
    ]
}

fn undo_row(
    undo_group_id: &str,
    surface_class: InteractionSurfaceClass,
    group_scope: UndoGroupScope,
    source_attribution_label: &str,
    command_id_ref: &str,
    multi_file_or_multi_step: bool,
    single_reviewable_undo_entry: bool,
    no_undo_posture: Option<NoUndoPosture>,
    no_undo_posture_label: Option<&str>,
) -> UndoGroupAttributionRecord {
    UndoGroupAttributionRecord {
        record_kind: UNDO_GROUP_ATTRIBUTION_RECORD_KIND.to_owned(),
        schema_version: INTERACTION_TRANSFER_BETA_SCHEMA_VERSION,
        shared_contract_ref: INTERACTION_TRANSFER_SHARED_CONTRACT_REF.to_owned(),
        undo_group_id: undo_group_id.to_owned(),
        surface_class,
        surface_class_token: surface_class.as_str().to_owned(),
        group_scope,
        group_scope_token: group_scope.as_str().to_owned(),
        source_attribution_label: source_attribution_label.to_owned(),
        command_id_ref: command_id_ref.to_owned(),
        multi_file_or_multi_step,
        single_reviewable_undo_entry,
        no_undo_posture,
        no_undo_posture_token: no_undo_posture.map(|posture| posture.as_str().to_owned()),
        no_undo_posture_label: no_undo_posture_label.map(str::to_owned),
    }
}

fn seeded_undo_groups() -> Vec<UndoGroupAttributionRecord> {
    vec![
        undo_row(
            "undo:editor:multi_file_replace",
            InteractionSurfaceClass::Editor,
            UndoGroupScope::MultiFileReplace,
            "Search-and-replace across 12 files (cmd:editor.replace_all)",
            "cmd:editor.replace_all",
            true,
            true,
            None,
            None,
        ),
        undo_row(
            "undo:editor:settings_import",
            InteractionSurfaceClass::Editor,
            UndoGroupScope::SettingsImport,
            "Settings import from settings_bundle.json",
            "cmd:settings.import",
            true,
            true,
            None,
            None,
        ),
        undo_row(
            "undo:editor:ai_apply",
            InteractionSurfaceClass::Editor,
            UndoGroupScope::AiApply,
            "AI apply (session ai-2026-05-19-001)",
            "cmd:ai.apply_suggestion",
            true,
            true,
            None,
            None,
        ),
        undo_row(
            "undo:editor:extension_refactor",
            InteractionSurfaceClass::Editor,
            UndoGroupScope::ExtensionRefactor,
            "Extension refactor from ext.rust-analyzer (rename symbol)",
            "cmd:extensions.rust-analyzer.rename",
            true,
            true,
            None,
            None,
        ),
        undo_row(
            "undo:provider_linked:no_undo_preview",
            InteractionSurfaceClass::ProviderLinked,
            UndoGroupScope::NoUndoAvailable,
            "Provider-linked publish (cmd:provider.publish)",
            "cmd:provider.publish",
            false,
            false,
            Some(NoUndoPosture::PreviewBeforeCommit),
            Some("This action cannot be undone; review the publish preview before commit."),
        ),
    ]
}

fn back_forward_row(
    entry_id: &str,
    surface_class: InteractionSurfaceClass,
    direction: BackForwardDirection,
    target_identity_ref: &str,
    target_identity_label: &str,
    workspace_scope_label: &str,
    recorded_at: &str,
    source_label: &str,
    preserves_selection_or_cursor: bool,
) -> BackForwardEntryRecord {
    BackForwardEntryRecord {
        record_kind: BACK_FORWARD_ENTRY_RECORD_KIND.to_owned(),
        schema_version: INTERACTION_TRANSFER_BETA_SCHEMA_VERSION,
        shared_contract_ref: INTERACTION_TRANSFER_SHARED_CONTRACT_REF.to_owned(),
        entry_id: entry_id.to_owned(),
        surface_class,
        surface_class_token: surface_class.as_str().to_owned(),
        direction,
        direction_token: direction.as_str().to_owned(),
        target_identity_ref: target_identity_ref.to_owned(),
        target_identity_label: target_identity_label.to_owned(),
        workspace_scope_label: workspace_scope_label.to_owned(),
        recorded_at: recorded_at.to_owned(),
        source_label: source_label.to_owned(),
        preserves_selection_or_cursor,
    }
}

fn seeded_back_forward_entries() -> Vec<BackForwardEntryRecord> {
    vec![
        back_forward_row(
            "back_forward:editor:back:main_rs",
            InteractionSurfaceClass::Editor,
            BackForwardDirection::Back,
            "editor:buffer:main.rs#fn_run",
            "main.rs · fn run",
            "current workspace",
            "2026-05-19T09:50:11Z",
            "cmd:navigation.go_back",
            true,
        ),
        back_forward_row(
            "back_forward:editor:forward:lib_rs",
            InteractionSurfaceClass::Editor,
            BackForwardDirection::Forward,
            "editor:buffer:lib.rs#mod_interaction_transfer",
            "lib.rs · mod interaction_transfer",
            "current workspace",
            "2026-05-19T09:51:02Z",
            "cmd:navigation.go_forward",
            true,
        ),
        back_forward_row(
            "back_forward:diff:back:hunk_42",
            InteractionSurfaceClass::Diff,
            BackForwardDirection::Back,
            "diff:compare:hunk-42",
            "diff hunk 42",
            "current workspace",
            "2026-05-19T09:52:18Z",
            "cmd:diff.go_back",
            true,
        ),
        back_forward_row(
            "back_forward:result_grid:forward:row_12",
            InteractionSurfaceClass::ResultGrid,
            BackForwardDirection::Forward,
            "result_grid:search:row-12",
            "search row 12",
            "current workspace",
            "2026-05-19T09:53:44Z",
            "cmd:result_grid.go_forward",
            true,
        ),
    ]
}

fn reopen_row(
    history_entry_id: &str,
    surface_class: InteractionSurfaceClass,
    source_class: ReopenSourceClass,
    closed_object_ref: &str,
    closed_object_label: &str,
    reopen_command_id: &str,
    restored_target_identity_ref: &str,
    closed_at: &str,
    last_activity_at: Option<&str>,
    continuity_label: &str,
    restored_live_authority: bool,
    auto_rerun_forbidden: bool,
    workspace_scoped: bool,
    source_label: &str,
) -> ReopenHistoryEntryRecord {
    ReopenHistoryEntryRecord {
        record_kind: REOPEN_HISTORY_ENTRY_RECORD_KIND.to_owned(),
        schema_version: INTERACTION_TRANSFER_BETA_SCHEMA_VERSION,
        shared_contract_ref: INTERACTION_TRANSFER_SHARED_CONTRACT_REF.to_owned(),
        history_entry_id: history_entry_id.to_owned(),
        surface_class,
        surface_class_token: surface_class.as_str().to_owned(),
        source_class,
        source_class_token: source_class.as_str().to_owned(),
        closed_object_ref: closed_object_ref.to_owned(),
        closed_object_label: closed_object_label.to_owned(),
        reopen_command_id: reopen_command_id.to_owned(),
        restored_target_identity_ref: restored_target_identity_ref.to_owned(),
        restored_selection_ref: None,
        restored_scroll_anchor_ref: None,
        closed_at: closed_at.to_owned(),
        last_activity_at: last_activity_at.map(str::to_owned),
        continuity_label: continuity_label.to_owned(),
        restored_live_authority,
        auto_rerun_forbidden,
        workspace_scoped,
        source_label: source_label.to_owned(),
    }
}

fn seeded_reopen_history_entries() -> Vec<ReopenHistoryEntryRecord> {
    vec![
        reopen_row(
            "reopen:editor:closed_intentionally",
            InteractionSurfaceClass::Editor,
            ReopenSourceClass::ClosedIntentionally,
            "editor:closed_buffer:notes.md",
            "notes.md",
            "cmd:editor.reopen_closed",
            "editor:buffer:notes.md",
            "2026-05-19T08:21:09Z",
            Some("2026-05-19T08:19:55Z"),
            "Closed intentionally; reopen restores the same buffer identity.",
            true,
            false,
            true,
            "Closed earlier today",
        ),
        reopen_row(
            "reopen:diff:back_forward",
            InteractionSurfaceClass::Diff,
            ReopenSourceClass::BackForwardNavigation,
            "diff:closed:hunk-42",
            "diff hunk 42",
            "cmd:diff.go_back",
            "diff:compare:hunk-42",
            "2026-05-19T09:52:18Z",
            None,
            "Returned via workspace back/forward navigation; identity preserved.",
            true,
            false,
            true,
            "Back/forward navigation",
        ),
        reopen_row(
            "reopen:review:crash_recovery",
            InteractionSurfaceClass::Review,
            ReopenSourceClass::CrashRecovery,
            "review:closed:pr-2018",
            "PR #2018 review",
            "cmd:review.reopen_after_crash",
            "review:pr:2018",
            "2026-05-19T06:14:02Z",
            Some("2026-05-19T06:13:50Z"),
            "Restored after crash; live authority did not survive — rerun is not automatic.",
            false,
            true,
            true,
            "Recovered after crash",
        ),
        reopen_row(
            "reopen:result_grid:disconnect_recovery",
            InteractionSurfaceClass::ResultGrid,
            ReopenSourceClass::DisconnectRecovery,
            "result_grid:closed:work-items",
            "work-item board",
            "cmd:result_grid.reopen_after_disconnect",
            "result_grid:work_items:default",
            "2026-05-19T07:02:33Z",
            Some("2026-05-19T07:02:18Z"),
            "Restored after transport disconnect; provider scope must be re-verified.",
            false,
            true,
            true,
            "Recovered after disconnect",
        ),
        reopen_row(
            "reopen:provider_linked:placeholder",
            InteractionSurfaceClass::ProviderLinked,
            ReopenSourceClass::PlaceholderReopen,
            "provider:closed:extension-listing",
            "provider extension listing",
            "cmd:provider.reopen_placeholder",
            "provider:extension:listing",
            "2026-05-19T07:30:00Z",
            None,
            "Placeholder reopened while provider authority is missing; live data not available.",
            false,
            true,
            true,
            "Placeholder reopen",
        ),
    ]
}

/// Returns action counts grouped by record-kind for the support summary.
pub fn record_counts_by_kind(packet: &InteractionTransferBetaPacket) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    counts.insert(
        CLIPBOARD_PAYLOAD_CLASS_RECORD_KIND.to_owned(),
        packet.clipboard_payload_classes.len(),
    );
    counts.insert(
        DROP_INTENT_RECORD_KIND.to_owned(),
        packet.drop_intents.len(),
    );
    counts.insert(
        UNDO_GROUP_ATTRIBUTION_RECORD_KIND.to_owned(),
        packet.undo_groups.len(),
    );
    counts.insert(
        BACK_FORWARD_ENTRY_RECORD_KIND.to_owned(),
        packet.back_forward_entries.len(),
    );
    counts.insert(
        REOPEN_HISTORY_ENTRY_RECORD_KIND.to_owned(),
        packet.reopen_history_entries.len(),
    );
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates() {
        let packet = seeded_interaction_transfer_beta_packet();
        validate_interaction_transfer_beta_packet(&packet).expect("seed packet must validate");
        assert_eq!(packet.summary.surfaces_present.len(), 5);
        assert!(packet.summary.every_surface_default_plain_text);
        assert!(packet.summary.move_copy_distinction_advertised);
        assert!(packet.summary.attach_open_distinction_advertised);
        assert!(packet.summary.broad_mutation_undo_group_covered);
        assert!(packet.summary.no_undo_preview_or_checkpoint_covered);
        assert!(packet.summary.back_forward_both_directions_covered);
        assert!(packet.summary.intentional_versus_recovery_distinguished);
        assert!(packet.summary.placeholder_reopen_covered);
    }

    #[test]
    fn default_copy_must_be_plain_text() {
        let mut packet = seeded_interaction_transfer_beta_packet();
        packet.clipboard_payload_classes[0].default_representation_class =
            PayloadRepresentationClass::Rendered;
        packet.clipboard_payload_classes[0].default_representation_class_token =
            PayloadRepresentationClass::Rendered.as_str().to_owned();
        packet.summary = compute_summary(&packet);
        let errors = validate_interaction_transfer_beta_packet(&packet)
            .expect_err("non-plain-text default must fail");
        assert!(errors.iter().any(|e| matches!(
            e,
            InteractionTransferValidationError::ClipboardPayloadInvalid { .. }
        )));
    }

    #[test]
    fn crash_recovery_must_forbid_rerun() {
        let mut packet = seeded_interaction_transfer_beta_packet();
        let row = packet
            .reopen_history_entries
            .iter_mut()
            .find(|row| matches!(row.source_class, ReopenSourceClass::CrashRecovery))
            .expect("seed must contain crash recovery");
        row.auto_rerun_forbidden = false;
        let errors = validate_interaction_transfer_beta_packet(&packet)
            .expect_err("crash recovery without rerun lock must fail");
        assert!(errors.iter().any(|e| matches!(
            e,
            InteractionTransferValidationError::ReopenHistoryInvalid { .. }
        )));
    }

    #[test]
    fn blocked_drop_must_declare_reason() {
        let mut packet = seeded_interaction_transfer_beta_packet();
        let row = packet
            .drop_intents
            .iter_mut()
            .find(|row| matches!(row.advertised_verb, DropVerb::Blocked))
            .expect("seed must contain a blocked drop");
        row.blocked_reason_label = None;
        let errors = validate_interaction_transfer_beta_packet(&packet)
            .expect_err("blocked drop without reason must fail");
        assert!(errors.iter().any(|e| matches!(
            e,
            InteractionTransferValidationError::DropIntentInvalid { .. }
        )));
    }

    #[test]
    fn back_forward_must_cover_both_directions() {
        let mut packet = seeded_interaction_transfer_beta_packet();
        packet
            .back_forward_entries
            .retain(|row| matches!(row.direction, BackForwardDirection::Back));
        packet.summary = compute_summary(&packet);
        let errors = validate_interaction_transfer_beta_packet(&packet)
            .expect_err("missing forward direction must fail");
        assert!(errors.iter().any(|e| matches!(
            e,
            InteractionTransferValidationError::CoverageMissing { .. }
        )));
    }

    #[test]
    fn support_export_must_track_packet_ids() {
        let mut packet = seeded_interaction_transfer_beta_packet();
        packet.support_export.drop_intent_ids.pop();
        let errors = validate_interaction_transfer_beta_packet(&packet)
            .expect_err("support export drift must fail");
        assert!(errors.iter().any(|e| matches!(
            e,
            InteractionTransferValidationError::SupportExportInvalid { .. }
        )));
    }

    #[test]
    fn record_counts_match_seed() {
        let packet = seeded_interaction_transfer_beta_packet();
        let counts = record_counts_by_kind(&packet);
        assert_eq!(counts[CLIPBOARD_PAYLOAD_CLASS_RECORD_KIND], 5);
        assert_eq!(counts[DROP_INTENT_RECORD_KIND], 7);
        assert_eq!(counts[UNDO_GROUP_ATTRIBUTION_RECORD_KIND], 5);
        assert_eq!(counts[BACK_FORWARD_ENTRY_RECORD_KIND], 4);
        assert_eq!(counts[REOPEN_HISTORY_ENTRY_RECORD_KIND], 5);
    }
}
