//! M5 grouped-history continuity packet: named undo groups, exact-versus-
//! compensating recovery labels, back/forward navigation continuity, and
//! reopen-closed surface affordances across M5 mutation and navigation surfaces.
//!
//! Aureline's switching promise depends on keyboard-first, recoverable
//! interaction across every new M5 surface — editor, notebook, data/API,
//! preview, docs, review, runtime, and companion-adjacent panes. The frozen
//! keyboard-continuity matrix
//! [`crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix`]
//! pins those surfaces to their canonical interaction vocabulary and requires
//! that *history distinguishes exact undo, compensating undo, and checkpoint
//! restore* and that *reopen/recover paths degrade honestly*. This module
//! discharges the grouped-history half of that contract: it takes the frozen
//! [`UndoClass`], [`HistoryClass`], and [`ReopenRecoverClass`] vocabulary and
//! makes a mutation or navigation **named, classified, and recoverable** on the
//! new M5 surfaces instead of one opaque `Undo` or generic `Back`.
//!
//! * a [`HistoryContinuityRecord`] binds a claimed M5 surface (keyed by a
//!   [`KeyboardSurfaceKind`] and a non-display [`KeyboardSurfaceSubject`]) to one
//!   history entry: whether it is a mutation or a navigation
//!   ([`HistoryEntryKind`]), the [`AffectedObjectSummary`] it touches, the
//!   [`SourceAttributionClass`] that produced it, the canonical [`UndoClass`] /
//!   [`HistoryClass`] / [`ReopenRecoverClass`] it carries, the
//!   [`SurfaceLossCause`] that distinguishes an intentional close from a crash /
//!   disconnect loss, and the resolved [`RecoveryAffordanceClass`];
//! * a history entry is **never flattened into one opaque label when it is
//!   consequential**: a record whose mutation touches many objects / files, that
//!   crosses surfaces during navigation, that is not literally invertible, that
//!   was generated / automated, that is recoverable only from a checkpoint, that
//!   sits on a closed / lost surface, or whose history proof is stale / missing
//!   fires one or more [`HistoryContractTrigger`]s. Each trigger imposes a
//!   minimum-safety floor on the resolution, so a triggered record can never
//!   resolve to a flat [`RecoveryAffordanceClass::ExactStepUndo`]; it must offer a
//!   named exact group, preserve back/forward continuity, expose a labeled
//!   compensating action, regenerate from source, restore from a checkpoint, or
//!   reopen / recover the surface with the loss cause named;
//! * a reopen / recover **always distinguishes an intentional close from a crash
//!   or disconnect loss**, preserves a close timestamp and a source attribution
//!   where useful, and never lets a provider-linked surface read as a locally
//!   verified history.
//!
//! [`HistoryContinuityPacket::validate`] refuses a packet that flattens a
//! consequential entry into one opaque undo, that lowers a resolution below its
//! required safety floor, that conflates an intentional close with a loss, that
//! drops the exact-versus-compensating distinction, or that lets a
//! provider-linked surface read as a locally verified history.
//!
//! Raw provider payloads, file contents, and absolute private paths never cross
//! this boundary; the packet carries only typed class tokens, booleans, opaque /
//! relative ids, fingerprint digests, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/interaction/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont.schema.json`](../../../../schemas/interaction/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont.schema.json).
//! The contract doc is
//! [`docs/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont.md`](../../../../docs/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont.md).
//! The protected fixture directory is
//! [`fixtures/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont/`](../../../../fixtures/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Re-export the frozen taxonomy this consumer binds, so product, help, support,
// and migration surfaces can name those types through this module rather than
// reaching into the matrix module by hand.
pub use crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix::{
    AxisProofCurrency, AxisVerification, HistoryClass, KeyboardSurfaceKind, KeyboardSurfaceSubject,
    ReopenRecoverClass, SurfaceOriginClass, UndoClass, KEYBOARD_CONTINUITY_MATRIX_DOC_REF,
};

/// Stable record-kind tag carried by [`HistoryContinuityPacket`].
pub const HISTORY_CONTINUITY_RECORD_KIND: &str =
    "m5_named_undo_group_recovery_label_back_forward_continuity_reopen_packet";

/// Schema version for the grouped-history continuity packet.
pub const HISTORY_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const HISTORY_CONTINUITY_SCHEMA_REF: &str =
    "schemas/interaction/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont.schema.json";

/// Repo-relative path of the contract doc.
pub const HISTORY_CONTINUITY_DOC_REF: &str =
    "docs/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont.md";

/// Repo-relative path of the checked support-export artifact.
pub const HISTORY_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const HISTORY_CONTINUITY_SUMMARY_REF: &str =
    "artifacts/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont.md";

/// Repo-relative path of the protected fixture directory.
pub const HISTORY_CONTINUITY_FIXTURE_DIR: &str =
    "fixtures/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont";

/// Source contract ref of the clipboard / transfer / undo / history contract.
pub const HISTORY_RECOVERY_CONTRACT_REF: &str = "docs/ux/clipboard_history_contract.md";

/// Whether a history entry is a mutation (a change to be undone / recovered) or a
/// navigation (a move whose back/forward identity matters). Both are named and
/// classified rather than collapsed into one opaque history label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryEntryKind {
    /// A change to surface content / state that can be undone or recovered.
    Mutation,
    /// A back/forward navigation whose target identity must be preserved.
    Navigation,
}

impl HistoryEntryKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mutation => "mutation",
            Self::Navigation => "navigation",
        }
    }

    /// Whether this entry is a navigation rather than a mutation.
    pub const fn is_navigation(self) -> bool {
        matches!(self, Self::Navigation)
    }
}

/// Class of object a history entry affects. The class lets help, migration, and
/// support name the same affected objects the product exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryObjectClass {
    /// An editor text range (the editor-core baseline).
    EditorRange,
    /// A group of notebook cells.
    NotebookCellGroup,
    /// A set of data-grid / API result rows.
    ResultRowSet,
    /// A docs authoring section.
    DocsSection,
    /// A preview / preview-runtime navigation target.
    PreviewTarget,
    /// A review / pull-request item.
    ReviewItem,
    /// An embedded runtime session / state.
    RuntimeSession,
    /// A provider-linked companion thread.
    CompanionThread,
}

impl HistoryObjectClass {
    /// Every object class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::EditorRange,
        Self::NotebookCellGroup,
        Self::ResultRowSet,
        Self::DocsSection,
        Self::PreviewTarget,
        Self::ReviewItem,
        Self::RuntimeSession,
        Self::CompanionThread,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorRange => "editor_range",
            Self::NotebookCellGroup => "notebook_cell_group",
            Self::ResultRowSet => "result_row_set",
            Self::DocsSection => "docs_section",
            Self::PreviewTarget => "preview_target",
            Self::ReviewItem => "review_item",
            Self::RuntimeSession => "runtime_session",
            Self::CompanionThread => "companion_thread",
        }
    }
}

/// What produced a history entry. Preserved as a source attribution so a grouped
/// history can name *who or what* changed something rather than collapsing every
/// entry into an anonymous step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceAttributionClass {
    /// A direct user edit / navigation.
    UserDirect,
    /// An automation / agent-driven change.
    AutomationAgent,
    /// A provider-backed sync.
    ProviderSync,
    /// A change generated from a source the surface can regenerate from.
    GeneratedFromSource,
    /// A checkpoint / snapshot subsystem action.
    CheckpointSystem,
}

impl SourceAttributionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserDirect => "user_direct",
            Self::AutomationAgent => "automation_agent",
            Self::ProviderSync => "provider_sync",
            Self::GeneratedFromSource => "generated_from_source",
            Self::CheckpointSystem => "checkpoint_system",
        }
    }
}

/// Why a surface tied to a history entry is closed or lost. A reopen / recover
/// must distinguish an intentional close from a crash / disconnect loss rather
/// than presenting both as the same generic reopen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceLossCause {
    /// The surface is open / not closed; no reopen is pending.
    NotClosed,
    /// The user intentionally closed the surface.
    IntentionalClose,
    /// The surface was lost to a crash.
    CrashLoss,
    /// The surface was lost to a provider / runtime disconnect.
    DisconnectLoss,
}

impl SurfaceLossCause {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotClosed => "not_closed",
            Self::IntentionalClose => "intentional_close",
            Self::CrashLoss => "crash_loss",
            Self::DisconnectLoss => "disconnect_loss",
        }
    }

    /// Whether this cause classifies a real close / loss (rather than an open
    /// surface), so a reopen can name how the surface went away.
    pub const fn is_closed_or_lost(self) -> bool {
        !matches!(self, Self::NotClosed)
    }

    /// Whether this cause is an unintended loss (crash / disconnect) rather than a
    /// deliberate close.
    pub const fn is_unintended_loss(self) -> bool {
        matches!(self, Self::CrashLoss | Self::DisconnectLoss)
    }
}

/// Resolved recovery affordance a surface exposes for one history entry. This is
/// the canonical recovery vocabulary help, migration, and support name.
///
/// Only [`Self::ExactStepUndo`] is the flat single-step baseline; every other
/// resolution names a more careful recovery — a named exact group, preserved
/// back/forward continuity, a labeled compensating action, a regenerate-from-
/// source, a checkpoint restore, or a reopen / recover with the loss cause named.
/// The [`Self::safety_rank`] orders the resolutions so a triggered record can be
/// held at or above a required floor and never flattened into the bare baseline.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryAffordanceClass {
    /// A single exact-step undo, honestly named by the entry label.
    ExactStepUndo,
    /// A named group of actions undone exactly as one unit.
    NamedGroupExactUndo,
    /// A navigation whose back/forward target identity is preserved.
    BackForwardContinuityPreserved,
    /// A labeled compensating action that is not a literal inverse.
    CompensatingActionLabeled,
    /// A generated change recovered by regenerating from its source.
    RegenerateFromSource,
    /// A checkpoint / snapshot restore rather than a step inverse.
    CheckpointRestoreLabeled,
    /// A closed / lost surface reopened or recovered with the loss cause named.
    ReopenOrRecoverLabeled,
}

impl RecoveryAffordanceClass {
    /// Every resolution class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ExactStepUndo,
        Self::NamedGroupExactUndo,
        Self::BackForwardContinuityPreserved,
        Self::CompensatingActionLabeled,
        Self::RegenerateFromSource,
        Self::CheckpointRestoreLabeled,
        Self::ReopenOrRecoverLabeled,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactStepUndo => "exact_step_undo",
            Self::NamedGroupExactUndo => "named_group_exact_undo",
            Self::BackForwardContinuityPreserved => "back_forward_continuity_preserved",
            Self::CompensatingActionLabeled => "compensating_action_labeled",
            Self::RegenerateFromSource => "regenerate_from_source",
            Self::CheckpointRestoreLabeled => "checkpoint_restore_labeled",
            Self::ReopenOrRecoverLabeled => "reopen_or_recover_labeled",
        }
    }

    /// Monotonic safety rank; higher is a more careful / restrictive recovery, so a
    /// triggered record must hold a resolution whose rank meets its floor.
    pub const fn safety_rank(self) -> u8 {
        match self {
            Self::ExactStepUndo => 0,
            Self::NamedGroupExactUndo => 1,
            Self::BackForwardContinuityPreserved => 2,
            Self::CompensatingActionLabeled => 3,
            Self::RegenerateFromSource => 4,
            Self::CheckpointRestoreLabeled => 5,
            Self::ReopenOrRecoverLabeled => 6,
        }
    }

    /// Whether this resolution is the flat single-step baseline that a
    /// consequential entry must never silently collapse into.
    pub const fn is_flat_baseline(self) -> bool {
        matches!(self, Self::ExactStepUndo)
    }

    /// The canonical [`UndoClass`] this resolution implies, where the recovery maps
    /// cleanly onto an undo class. Navigation continuity and reopen / recover are
    /// orthogonal to the undo ladder, so they imply no undo class.
    pub const fn canonical_undo_class(self) -> Option<UndoClass> {
        match self {
            Self::ExactStepUndo => Some(UndoClass::ExactUndo),
            Self::NamedGroupExactUndo => Some(UndoClass::GroupedExactUndo),
            Self::CompensatingActionLabeled | Self::RegenerateFromSource => {
                Some(UndoClass::CompensatingUndo)
            }
            Self::CheckpointRestoreLabeled => Some(UndoClass::CheckpointRestore),
            Self::BackForwardContinuityPreserved | Self::ReopenOrRecoverLabeled => None,
        }
    }

    /// Whether this resolution must cite a `group_label`.
    pub const fn requires_group_label(self) -> bool {
        matches!(self, Self::NamedGroupExactUndo)
    }

    /// Whether this resolution must cite a `continuity_note`.
    pub const fn requires_continuity_note(self) -> bool {
        matches!(self, Self::BackForwardContinuityPreserved)
    }

    /// Whether this resolution must cite a `compensating_label`.
    pub const fn requires_compensating_label(self) -> bool {
        matches!(self, Self::CompensatingActionLabeled)
    }

    /// Whether this resolution must cite a `regenerate_source_label`.
    pub const fn requires_regenerate_label(self) -> bool {
        matches!(self, Self::RegenerateFromSource)
    }

    /// Whether this resolution must cite a `checkpoint_label`.
    pub const fn requires_checkpoint_label(self) -> bool {
        matches!(self, Self::CheckpointRestoreLabeled)
    }

    /// Whether this resolution must cite a `reopen_label` and name a loss cause.
    pub const fn requires_reopen_label(self) -> bool {
        matches!(self, Self::ReopenOrRecoverLabeled)
    }
}

/// Why a record's history entry was held off the flat single-step lane. Each
/// trigger imposes a minimum-safety floor; the chrome quotes the trigger verbatim
/// instead of a generic label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryContractTrigger {
    /// The mutation touches many objects / files, so it must be a named group.
    BroadMultiObjectMutation,
    /// The navigation crosses surfaces, so back/forward identity must be preserved.
    CrossSurfaceNavigation,
    /// The mutation is not a literal inverse, so a compensating action is needed.
    NotLiterallyInvertible,
    /// The change was generated / automated, so regenerate-from-source applies.
    GeneratedOrAutomatedChange,
    /// Recovery is only possible from a checkpoint / snapshot.
    CheckpointOnlyRecovery,
    /// The surface tied to this entry is closed or lost and must be reopened.
    SurfaceClosedOrLost,
    /// The history proof backing this record is stale or missing.
    StaleOrMissingHistoryProof,
}

impl HistoryContractTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BroadMultiObjectMutation => "broad_multi_object_mutation",
            Self::CrossSurfaceNavigation => "cross_surface_navigation",
            Self::NotLiterallyInvertible => "not_literally_invertible",
            Self::GeneratedOrAutomatedChange => "generated_or_automated_change",
            Self::CheckpointOnlyRecovery => "checkpoint_only_recovery",
            Self::SurfaceClosedOrLost => "surface_closed_or_lost",
            Self::StaleOrMissingHistoryProof => "stale_or_missing_history_proof",
        }
    }

    /// Minimum resolution safety rank this trigger imposes.
    ///
    /// A broad mutation or stale proof requires at least a named exact group; a
    /// cross-surface navigation requires preserved back/forward continuity; a
    /// non-invertible change requires a labeled compensating action; a generated
    /// change requires a regenerate-from-source; a checkpoint-only recovery
    /// requires a checkpoint restore; a closed / lost surface requires a reopen /
    /// recover with the loss cause named.
    pub const fn minimum_resolution_rank(self) -> u8 {
        match self {
            Self::BroadMultiObjectMutation | Self::StaleOrMissingHistoryProof => {
                RecoveryAffordanceClass::NamedGroupExactUndo.safety_rank()
            }
            Self::CrossSurfaceNavigation => {
                RecoveryAffordanceClass::BackForwardContinuityPreserved.safety_rank()
            }
            Self::NotLiterallyInvertible => {
                RecoveryAffordanceClass::CompensatingActionLabeled.safety_rank()
            }
            Self::GeneratedOrAutomatedChange => {
                RecoveryAffordanceClass::RegenerateFromSource.safety_rank()
            }
            Self::CheckpointOnlyRecovery => {
                RecoveryAffordanceClass::CheckpointRestoreLabeled.safety_rank()
            }
            Self::SurfaceClosedOrLost => {
                RecoveryAffordanceClass::ReopenOrRecoverLabeled.safety_rank()
            }
        }
    }
}

/// Summary of the object(s) a history entry affects. Preserved so a grouped
/// history can name *what* changed rather than collapsing into opaque payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AffectedObjectSummary {
    /// Class of the affected object.
    pub object_class: HistoryObjectClass,
    /// Opaque / workspace-relative object token. Never an absolute private path.
    pub object_token: String,
    /// Count of distinct objects touched (at least one).
    pub object_count: u32,
    /// Count of distinct files touched (at least one).
    pub file_count: u32,
    /// Reviewable affected-object label.
    pub display_label: String,
}

impl AffectedObjectSummary {
    /// Whether the summary carries the identity a grouped history needs.
    pub fn is_valid(&self) -> bool {
        !self.object_token.trim().is_empty()
            && !self.display_label.trim().is_empty()
            && self.object_count >= 1
            && self.file_count >= 1
    }

    /// Whether the entry spans more than one object or file, so it must be a named
    /// group rather than one opaque step.
    pub fn is_broad(&self) -> bool {
        self.object_count > 1 || self.file_count > 1
    }
}

/// Constructor input for [`HistoryContinuityRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryContinuityRecordInput {
    /// Stable record id.
    pub record_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the record covers.
    pub subject: KeyboardSurfaceSubject,
    /// Reviewable record label.
    pub label_summary: String,
    /// Whether this entry is a mutation or a navigation.
    pub entry_kind: HistoryEntryKind,
    /// The object(s) the entry affects.
    pub affected_object: AffectedObjectSummary,
    /// What produced the entry.
    pub source_attribution: SourceAttributionClass,
    /// Canonical undo class carried by the entry.
    pub undo_class: UndoClass,
    /// Canonical grouped-history class.
    pub history_class: HistoryClass,
    /// Canonical reopen/recover class.
    pub reopen_recover: ReopenRecoverClass,
    /// Why the tied surface is closed or lost (or not closed).
    pub loss_cause: SurfaceLossCause,
    /// RFC 3339 close timestamp, preserved when the surface was closed / lost.
    pub closed_at: Option<String>,
    /// Whether the mutation has a literal inverse.
    pub literally_invertible: bool,
    /// Whether the entry was generated / automated.
    pub generated_or_automated: bool,
    /// Whether a navigation crosses surfaces (identity matters).
    pub cross_surface_navigation: bool,
    /// Whether recovery is only possible from a checkpoint.
    pub checkpoint_only: bool,
    /// Reopenable verification proof backing the resolution.
    pub verification: AxisVerification,
    /// The resolved recovery affordance.
    pub resolution: RecoveryAffordanceClass,
    /// Triggers recorded as firing for this record.
    pub fired_triggers: Vec<HistoryContractTrigger>,
    /// Required when `resolution` is `named_group_exact_undo`.
    pub group_label: Option<String>,
    /// Required when `resolution` is `back_forward_continuity_preserved`.
    pub continuity_note: Option<String>,
    /// Required when `resolution` is `compensating_action_labeled`.
    pub compensating_label: Option<String>,
    /// Required when `resolution` is `regenerate_from_source`.
    pub regenerate_source_label: Option<String>,
    /// Required when `resolution` is `checkpoint_restore_labeled`.
    pub checkpoint_label: Option<String>,
    /// Required when `resolution` is `reopen_or_recover_labeled`.
    pub reopen_label: Option<String>,
    /// Evidence packet refs backing this record.
    pub evidence_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

/// One grouped-history record binding a claimed M5 surface to one history entry
/// with a named undo class, an honest recovery label, and a reopen / recover
/// posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryContinuityRecord {
    /// Stable record id.
    pub record_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the record covers.
    pub subject: KeyboardSurfaceSubject,
    /// Reviewable record label.
    pub label_summary: String,
    /// Whether this entry is a mutation or a navigation.
    pub entry_kind: HistoryEntryKind,
    /// The object(s) the entry affects.
    pub affected_object: AffectedObjectSummary,
    /// What produced the entry.
    pub source_attribution: SourceAttributionClass,
    /// Canonical undo class carried by the entry.
    pub undo_class: UndoClass,
    /// Canonical grouped-history class.
    pub history_class: HistoryClass,
    /// Canonical reopen/recover class.
    pub reopen_recover: ReopenRecoverClass,
    /// Why the tied surface is closed or lost (or not closed).
    pub loss_cause: SurfaceLossCause,
    /// RFC 3339 close timestamp, preserved when the surface was closed / lost.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub closed_at: Option<String>,
    /// Whether the mutation has a literal inverse.
    pub literally_invertible: bool,
    /// Whether the entry was generated / automated.
    pub generated_or_automated: bool,
    /// Whether a navigation crosses surfaces (identity matters).
    pub cross_surface_navigation: bool,
    /// Whether recovery is only possible from a checkpoint.
    pub checkpoint_only: bool,
    /// Reopenable verification proof backing the resolution.
    pub verification: AxisVerification,
    /// The resolved recovery affordance.
    pub resolution: RecoveryAffordanceClass,
    /// Triggers recorded as firing for this record. Must equal the computed set.
    pub fired_triggers: Vec<HistoryContractTrigger>,
    /// Required when `resolution` is `named_group_exact_undo`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_label: Option<String>,
    /// Required when `resolution` is `back_forward_continuity_preserved`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuity_note: Option<String>,
    /// Required when `resolution` is `compensating_action_labeled`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compensating_label: Option<String>,
    /// Required when `resolution` is `regenerate_from_source`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub regenerate_source_label: Option<String>,
    /// Required when `resolution` is `checkpoint_restore_labeled`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_label: Option<String>,
    /// Required when `resolution` is `reopen_or_recover_labeled`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reopen_label: Option<String>,
    /// Guardrail: record does not carry raw provider payloads.
    pub raw_provider_payload_present: bool,
    /// Guardrail: record does not carry an absolute private path.
    pub absolute_private_path_present: bool,
    /// Guardrail: distinct undo / recovery classes were not flattened into one.
    pub distinct_classes_flattened: bool,
    /// Guardrail: an intentional close was not conflated with a crash / loss.
    pub reopen_loss_cause_conflated: bool,
    /// Evidence packet refs backing this record.
    pub evidence_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

impl HistoryContinuityRecord {
    /// Builds a record from its input, defaulting the guardrail flags to their
    /// safe values.
    pub fn new(input: HistoryContinuityRecordInput) -> Self {
        Self {
            record_id: input.record_id,
            surface_kind: input.surface_kind,
            subject: input.subject,
            label_summary: input.label_summary,
            entry_kind: input.entry_kind,
            affected_object: input.affected_object,
            source_attribution: input.source_attribution,
            undo_class: input.undo_class,
            history_class: input.history_class,
            reopen_recover: input.reopen_recover,
            loss_cause: input.loss_cause,
            closed_at: input.closed_at,
            literally_invertible: input.literally_invertible,
            generated_or_automated: input.generated_or_automated,
            cross_surface_navigation: input.cross_surface_navigation,
            checkpoint_only: input.checkpoint_only,
            verification: input.verification,
            resolution: input.resolution,
            fired_triggers: input.fired_triggers,
            group_label: input.group_label,
            continuity_note: input.continuity_note,
            compensating_label: input.compensating_label,
            regenerate_source_label: input.regenerate_source_label,
            checkpoint_label: input.checkpoint_label,
            reopen_label: input.reopen_label,
            raw_provider_payload_present: false,
            absolute_private_path_present: false,
            distinct_classes_flattened: false,
            reopen_loss_cause_conflated: false,
            evidence_refs: input.evidence_refs,
            minted_at: input.minted_at,
        }
    }

    /// Whether history for this record is provider-backed / imported.
    pub fn provider_or_imported(&self) -> bool {
        self.subject.origin_class.is_provider_or_imported()
    }

    /// Whether the verification proof backs a current history claim for this
    /// record's origin posture.
    pub fn history_proof_current(&self) -> bool {
        self.verification.backs_claim(self.provider_or_imported())
    }

    /// Whether the surface tied to this entry is closed or lost.
    pub fn surface_closed_or_lost(&self) -> bool {
        self.loss_cause.is_closed_or_lost()
    }

    /// The set of triggers that actually fire for this record, computed from its
    /// entry kind, breadth, invertibility, source, checkpoint posture, loss cause,
    /// and proof.
    pub fn computed_triggers(&self) -> BTreeSet<HistoryContractTrigger> {
        let mut triggers = BTreeSet::new();
        if self.entry_kind == HistoryEntryKind::Mutation && self.affected_object.is_broad() {
            triggers.insert(HistoryContractTrigger::BroadMultiObjectMutation);
        }
        if self.entry_kind.is_navigation() && self.cross_surface_navigation {
            triggers.insert(HistoryContractTrigger::CrossSurfaceNavigation);
        }
        if self.entry_kind == HistoryEntryKind::Mutation && !self.literally_invertible {
            triggers.insert(HistoryContractTrigger::NotLiterallyInvertible);
        }
        if self.generated_or_automated {
            triggers.insert(HistoryContractTrigger::GeneratedOrAutomatedChange);
        }
        if self.checkpoint_only {
            triggers.insert(HistoryContractTrigger::CheckpointOnlyRecovery);
        }
        if self.surface_closed_or_lost() {
            triggers.insert(HistoryContractTrigger::SurfaceClosedOrLost);
        }
        if !self.history_proof_current() {
            triggers.insert(HistoryContractTrigger::StaleOrMissingHistoryProof);
        }
        triggers
    }

    /// The recorded triggers as a set.
    pub fn recorded_triggers(&self) -> BTreeSet<HistoryContractTrigger> {
        self.fired_triggers.iter().copied().collect()
    }

    /// The minimum resolution safety rank this record must meet, given its
    /// triggers.
    pub fn required_floor_rank(&self) -> u8 {
        self.computed_triggers()
            .iter()
            .map(|trigger| trigger.minimum_resolution_rank())
            .max()
            .unwrap_or(0)
    }

    /// Whether the entry must be held off the flat single-step lane.
    pub fn must_not_flatten(&self) -> bool {
        self.required_floor_rank() > 0
    }

    /// Whether the recorded resolution meets the required safety floor.
    pub fn resolution_meets_floor(&self) -> bool {
        self.resolution.safety_rank() >= self.required_floor_rank()
    }

    /// Whether the recorded resolution silently flattens a consequential entry
    /// into the bare exact-step baseline.
    pub fn silently_flattens(&self) -> bool {
        self.resolution.is_flat_baseline() && self.must_not_flatten()
    }

    /// Whether the recorded trigger set matches the computed set.
    pub fn triggers_consistent(&self) -> bool {
        self.recorded_triggers() == self.computed_triggers()
    }

    /// Whether the recorded undo class matches the one the resolution implies,
    /// where the recovery maps cleanly onto an undo class. Navigation continuity
    /// and reopen / recover impose no undo class.
    pub fn undo_class_consistent(&self) -> bool {
        self.resolution
            .canonical_undo_class()
            .is_none_or(|expected| expected == self.undo_class)
    }

    /// Whether the loss cause, close timestamp, and reopen resolution agree: a
    /// closed / lost surface names a real cause and a close timestamp, an open
    /// surface names neither, and a reopen / recover resolution requires the
    /// surface to be closed or lost.
    pub fn loss_cause_consistent(&self) -> bool {
        let timestamp_present = self
            .closed_at
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty());
        if self.surface_closed_or_lost() {
            if !timestamp_present {
                return false;
            }
        } else {
            if timestamp_present {
                return false;
            }
            if self.resolution.requires_reopen_label() {
                return false;
            }
        }
        true
    }

    /// Whether the reopen/recover class is consistent with the resolution: a
    /// reopen / recover resolution must not claim reopen is unavailable.
    pub fn reopen_recover_consistent(&self) -> bool {
        if self.resolution.requires_reopen_label() {
            self.reopen_recover != ReopenRecoverClass::ReopenUnavailableHonest
        } else {
            true
        }
    }

    /// Whether the resolution carries exactly the detail field it requires.
    pub fn resolution_detail_consistent(&self) -> bool {
        let present = |opt: &Option<String>| {
            opt.as_deref()
                .is_some_and(|value| !value.trim().is_empty() && !label_is_generic(value))
        };
        let check = |required: bool, value: &Option<String>| {
            if required {
                present(value)
            } else {
                value.is_none()
            }
        };
        check(self.resolution.requires_group_label(), &self.group_label)
            && check(
                self.resolution.requires_continuity_note(),
                &self.continuity_note,
            )
            && check(
                self.resolution.requires_compensating_label(),
                &self.compensating_label,
            )
            && check(
                self.resolution.requires_regenerate_label(),
                &self.regenerate_source_label,
            )
            && check(
                self.resolution.requires_checkpoint_label(),
                &self.checkpoint_label,
            )
            && check(self.resolution.requires_reopen_label(), &self.reopen_label)
    }

    /// Whether the imported posture is consistent: a provider/imported surface
    /// never reads as a locally verified history, and a local surface never leans
    /// on imported proof.
    pub fn imported_posture_consistent(&self) -> bool {
        if self.provider_or_imported() {
            !self.verification.proof_currency.is_current_local()
        } else {
            !self.verification.proof_currency.is_imported_current()
        }
    }

    /// Whether no raw boundary material or flattening / conflation side effect is
    /// flagged present.
    pub fn no_raw_boundary_material(&self) -> bool {
        !self.raw_provider_payload_present
            && !self.absolute_private_path_present
            && !self.distinct_classes_flattened
            && !self.reopen_loss_cause_conflated
    }

    /// Whether every field required to record this record is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.record_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.minted_at.trim().is_empty()
            && self.subject.is_valid()
            && self.affected_object.is_valid()
            && self.verification.is_well_formed()
            && self.triggers_consistent()
            && !self.silently_flattens()
            && self.resolution_meets_floor()
            && self.resolution_detail_consistent()
            && self.undo_class_consistent()
            && self.loss_cause_consistent()
            && self.reopen_recover_consistent()
            && self.imported_posture_consistent()
            && self.no_raw_boundary_material()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryContinuityGuardrails {
    /// Exact undo, compensating action, and checkpoint restore stay distinct.
    pub distinct_classes_never_flattened: bool,
    /// The exact-versus-compensating distinction is reconstructable for support.
    pub exact_versus_compensating_distinguished: bool,
    /// Back/forward continuity is preserved where navigation identity matters.
    pub back_forward_continuity_preserved: bool,
    /// Reopen / recover distinguishes an intentional close from a crash / loss.
    pub reopen_distinguishes_intentional_from_loss: bool,
    /// Close timestamps or source labels are preserved where useful.
    pub timestamps_or_source_preserved: bool,
    /// Provider-linked history never reads as a locally verified history.
    pub provider_history_never_reads_as_local: bool,
    /// No new general macro language or editor core is introduced here.
    pub no_new_macro_language_introduced: bool,
}

impl HistoryContinuityGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.distinct_classes_never_flattened
            && self.exact_versus_compensating_distinguished
            && self.back_forward_continuity_preserved
            && self.reopen_distinguishes_intentional_from_loss
            && self.timestamps_or_source_preserved
            && self.provider_history_never_reads_as_local
            && self.no_new_macro_language_introduced
    }
}

/// Consumer projection block: the surfaces that read this packet without cloning
/// undo / recovery language by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryContinuityConsumerProjection {
    /// Product surfaces ingest this packet.
    pub product_ingests_packet: bool,
    /// Help / migration guidance ingests the same packet.
    pub help_migration_ingests_packet: bool,
    /// Support / export tooling ingests the same packet.
    pub support_export_ingests_packet: bool,
    /// Release-control surfaces ingest the same packet.
    pub release_control_ingests_packet: bool,
    /// Help / migration / support can name the same undo classes and recovery
    /// labels the product exposes from this packet.
    pub history_classes_and_recovery_labels_nameable: bool,
}

impl HistoryContinuityConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_packet
            && self.help_migration_ingests_packet
            && self.support_export_ingests_packet
            && self.release_control_ingests_packet
            && self.history_classes_and_recovery_labels_nameable
    }
}

/// Verification freshness block for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryContinuityFreshness {
    /// Verification-freshness SLO in hours.
    pub verification_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last verification refresh.
    pub last_verification_refresh: String,
    /// True when stale verification automatically forces records off the flat lane.
    pub auto_escalate_on_stale: bool,
}

impl HistoryContinuityFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.verification_freshness_slo_hours > 0
            && !self.last_verification_refresh.trim().is_empty()
    }
}

/// Constructor input for [`HistoryContinuityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HistoryContinuityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-surface grouped-history records.
    pub records: Vec<HistoryContinuityRecord>,
    /// Guardrail invariants block.
    pub guardrails: HistoryContinuityGuardrails,
    /// Consumer projection block.
    pub consumer_projection: HistoryContinuityConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: HistoryContinuityFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe grouped-history continuity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryContinuityPacket {
    /// Record kind; must equal [`HISTORY_CONTINUITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`HISTORY_CONTINUITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-surface grouped-history records.
    pub records: Vec<HistoryContinuityRecord>,
    /// Guardrail invariants block.
    pub guardrails: HistoryContinuityGuardrails,
    /// Consumer projection block.
    pub consumer_projection: HistoryContinuityConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: HistoryContinuityFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl HistoryContinuityPacket {
    /// Builds a grouped-history continuity packet.
    pub fn new(input: HistoryContinuityPacketInput) -> Self {
        Self {
            record_kind: HISTORY_CONTINUITY_RECORD_KIND.to_owned(),
            schema_version: HISTORY_CONTINUITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            records: input.records,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            verification_freshness: input.verification_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surface kinds represented by some record in this packet.
    pub fn represented_surface_kinds(&self) -> BTreeSet<KeyboardSurfaceKind> {
        self.records
            .iter()
            .map(|record| record.surface_kind)
            .collect()
    }

    /// Object classes represented across records.
    pub fn represented_object_classes(&self) -> BTreeSet<HistoryObjectClass> {
        self.records
            .iter()
            .map(|record| record.affected_object.object_class)
            .collect()
    }

    /// Resolution classes represented across records.
    pub fn represented_resolutions(&self) -> BTreeSet<RecoveryAffordanceClass> {
        self.records
            .iter()
            .map(|record| record.resolution)
            .collect()
    }

    /// Undo classes represented across records.
    pub fn represented_undo_classes(&self) -> BTreeSet<UndoClass> {
        self.records
            .iter()
            .map(|record| record.undo_class)
            .collect()
    }

    /// Count of records held off the flat single-step lane.
    pub fn forced_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.must_not_flatten())
            .count()
    }

    /// Count of records resolved to the flat exact-step baseline.
    pub fn flat_baseline_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.resolution.is_flat_baseline())
            .count()
    }

    /// Count of navigation records.
    pub fn navigation_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.entry_kind.is_navigation())
            .count()
    }

    /// Count of mutation records.
    pub fn mutation_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.entry_kind == HistoryEntryKind::Mutation)
            .count()
    }

    /// Count of reopen / recover records.
    pub fn reopen_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.resolution == RecoveryAffordanceClass::ReopenOrRecoverLabeled)
            .count()
    }

    /// Count of provider-linked / imported records.
    pub fn provider_or_imported_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.provider_or_imported())
            .count()
    }

    /// Resolves a record by its id.
    pub fn record(&self, record_id: &str) -> Option<&HistoryContinuityRecord> {
        self.records
            .iter()
            .find(|record| record.record_id == record_id)
    }

    /// Validates the grouped-history continuity invariants.
    pub fn validate(&self) -> Vec<HistoryContinuityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != HISTORY_CONTINUITY_RECORD_KIND {
            violations.push(HistoryContinuityViolation::WrongRecordKind);
        }
        if self.schema_version != HISTORY_CONTINUITY_SCHEMA_VERSION {
            violations.push(HistoryContinuityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(HistoryContinuityViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_records(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(HistoryContinuityViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(HistoryContinuityViolation::ConsumerProjectionIncomplete);
        }
        if !self.verification_freshness.is_valid() {
            violations.push(HistoryContinuityViolation::VerificationFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("history continuity packet serializes"),
        ) {
            violations.push(HistoryContinuityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("history continuity packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Grouped-History Continuity: Named Undo Groups, Exact-versus-Compensating Recovery Labels, Back/Forward Continuity, and Reopen-Closed Affordances\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Records: {} ({} mutation, {} navigation, {} flat baseline, {} forced off flat, {} reopen/recover, {} provider/imported)\n",
            self.records.len(),
            self.mutation_record_count(),
            self.navigation_record_count(),
            self.flat_baseline_record_count(),
            self.forced_record_count(),
            self.reopen_record_count(),
            self.provider_or_imported_record_count()
        ));
        out.push_str(&format!(
            "- Surface kinds: {} / {}\n",
            self.represented_surface_kinds().len(),
            KeyboardSurfaceKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Recovery classes: {} / {}\n",
            self.represented_resolutions().len(),
            RecoveryAffordanceClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Undo classes: {} / {}\n",
            self.represented_undo_classes().len(),
            UndoClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Verification freshness SLO: {} hours (last refresh: {})\n",
            self.verification_freshness.verification_freshness_slo_hours,
            self.verification_freshness.last_verification_refresh
        ));
        out.push_str("\n## Records\n\n");
        for record in &self.records {
            out.push_str(&format!(
                "- **{}** ({}): recovery `{}`, undo `{}`\n",
                record.record_id,
                record.surface_kind.as_str(),
                record.resolution.as_str(),
                record.undo_class.as_str()
            ));
            out.push_str(&format!("  - {}\n", record.label_summary));
            out.push_str(&format!(
                "  - {} of `{}` ({}), source `{}`, history `{}`, reopen `{}`\n",
                record.entry_kind.as_str(),
                record.affected_object.object_token,
                record.affected_object.object_class.as_str(),
                record.source_attribution.as_str(),
                record.history_class.as_str(),
                record.reopen_recover.as_str()
            ));
            out.push_str(&format!(
                "  - objects={}, files={}, loss `{}`\n",
                record.affected_object.object_count,
                record.affected_object.file_count,
                record.loss_cause.as_str()
            ));
            let triggers = record
                .fired_triggers
                .iter()
                .map(|trigger| trigger.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "  - triggers: [{}]\n",
                if triggers.is_empty() {
                    "none"
                } else {
                    &triggers
                }
            ));
            if let Some(label) = &record.group_label {
                out.push_str(&format!("  - Named group: {label}\n"));
            }
            if let Some(note) = &record.continuity_note {
                out.push_str(&format!("  - Back/forward continuity: {note}\n"));
            }
            if let Some(label) = &record.compensating_label {
                out.push_str(&format!("  - Compensating action: {label}\n"));
            }
            if let Some(label) = &record.regenerate_source_label {
                out.push_str(&format!("  - Regenerate from source: {label}\n"));
            }
            if let Some(label) = &record.checkpoint_label {
                out.push_str(&format!("  - Checkpoint restore: {label}\n"));
            }
            if let Some(label) = &record.reopen_label {
                out.push_str(&format!("  - Reopen/recover: {label}\n"));
            }
            if let Some(closed_at) = &record.closed_at {
                out.push_str(&format!("  - Closed at: {closed_at}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum HistoryContinuityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<HistoryContinuityViolation>),
}

impl fmt::Display for HistoryContinuityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "history continuity export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "history continuity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for HistoryContinuityArtifactError {}

/// Validation failures emitted by [`HistoryContinuityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HistoryContinuityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed surface kind is represented by no record.
    RequiredSurfaceKindMissing,
    /// A required affected-object class is represented by no record.
    RequiredObjectClassMissing,
    /// The required recovery-affordance classes are not all represented.
    ResolutionCoverageMissing,
    /// No record demonstrates an entry held off the flat single-step lane.
    ForcedRecordCaseMissing,
    /// No clean flat-baseline exact-step record is present.
    FlatBaselineMissing,
    /// No navigation record preserving back/forward continuity is present.
    NavigationContinuityCaseMissing,
    /// No mutation record is present.
    MutationCaseMissing,
    /// No reopen record for an intentional close is present.
    IntentionalReopenCaseMissing,
    /// No reopen record for a crash / disconnect loss is present.
    LossRecoverCaseMissing,
    /// No provider-linked / imported record is present.
    ProviderOrImportedCaseMissing,
    /// A record is incomplete.
    RecordIncomplete,
    /// A consequential entry was flattened into the bare exact-step baseline.
    SilentFlatteningOfHistory,
    /// A record's resolution ranks below its required safety floor.
    ResolutionBelowRequiredFloor,
    /// A record's recorded triggers do not match the computed set.
    TriggerSetInconsistent,
    /// A record's resolution detail field is missing, generic, or unexpected.
    ResolutionDetailInconsistent,
    /// A record's undo class does not match the one its resolution implies.
    UndoClassInconsistent,
    /// A record's loss cause, close timestamp, or reopen resolution disagree.
    LossCauseInconsistent,
    /// A reopen / recover record claims reopen is unavailable.
    ReopenRecoverInconsistent,
    /// A record dropped its affected-object summary.
    AffectedObjectMissing,
    /// A provider/imported record reads as a locally verified history.
    ImportedReadsAsLocal,
    /// A record's verification proof is not reopenable.
    VerificationProofNotReopenable,
    /// A record lacks evidence refs.
    RecordEvidenceMissing,
    /// A record's subject fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A record flags raw boundary material / flattening side effect present.
    RawBoundaryMaterialPresent,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Verification freshness block is incomplete.
    VerificationFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl HistoryContinuityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceKindMissing => "required_surface_kind_missing",
            Self::RequiredObjectClassMissing => "required_object_class_missing",
            Self::ResolutionCoverageMissing => "resolution_coverage_missing",
            Self::ForcedRecordCaseMissing => "forced_record_case_missing",
            Self::FlatBaselineMissing => "flat_baseline_missing",
            Self::NavigationContinuityCaseMissing => "navigation_continuity_case_missing",
            Self::MutationCaseMissing => "mutation_case_missing",
            Self::IntentionalReopenCaseMissing => "intentional_reopen_case_missing",
            Self::LossRecoverCaseMissing => "loss_recover_case_missing",
            Self::ProviderOrImportedCaseMissing => "provider_or_imported_case_missing",
            Self::RecordIncomplete => "record_incomplete",
            Self::SilentFlatteningOfHistory => "silent_flattening_of_history",
            Self::ResolutionBelowRequiredFloor => "resolution_below_required_floor",
            Self::TriggerSetInconsistent => "trigger_set_inconsistent",
            Self::ResolutionDetailInconsistent => "resolution_detail_inconsistent",
            Self::UndoClassInconsistent => "undo_class_inconsistent",
            Self::LossCauseInconsistent => "loss_cause_inconsistent",
            Self::ReopenRecoverInconsistent => "reopen_recover_inconsistent",
            Self::AffectedObjectMissing => "affected_object_missing",
            Self::ImportedReadsAsLocal => "imported_reads_as_local",
            Self::VerificationProofNotReopenable => "verification_proof_not_reopenable",
            Self::RecordEvidenceMissing => "record_evidence_missing",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::VerificationFreshnessIncomplete => "verification_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_history_continuity_export(
) -> Result<HistoryContinuityPacket, HistoryContinuityArtifactError> {
    let packet: HistoryContinuityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/interaction/m5/ship-named-undo-groups-exact-versus-compensating-recovery-labels-back-forward-history-cont/support_export.json"
    )))
    .map_err(HistoryContinuityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(HistoryContinuityArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &HistoryContinuityPacket,
    violations: &mut Vec<HistoryContinuityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        HISTORY_CONTINUITY_SCHEMA_REF,
        HISTORY_CONTINUITY_DOC_REF,
        HISTORY_CONTINUITY_ARTIFACT_REF,
        KEYBOARD_CONTINUITY_MATRIX_DOC_REF,
        HISTORY_RECOVERY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(HistoryContinuityViolation::MissingSourceContracts);
            break;
        }
    }
}

/// Surface kinds that must appear so the packet proves grouped-history parity
/// across the new M5 mutation and navigation surfaces, plus the editor-core
/// baseline.
const REQUIRED_SURFACE_KINDS: [KeyboardSurfaceKind; 7] = [
    KeyboardSurfaceKind::EditorCore,
    KeyboardSurfaceKind::NotebookSurface,
    KeyboardSurfaceKind::DataApiSurface,
    KeyboardSurfaceKind::PreviewSurface,
    KeyboardSurfaceKind::DocsSurface,
    KeyboardSurfaceKind::ReviewSurface,
    KeyboardSurfaceKind::RuntimeSurface,
];

/// Affected-object classes whose grouped-history parity this packet must
/// demonstrate.
const REQUIRED_OBJECT_CLASSES: [HistoryObjectClass; 7] = [
    HistoryObjectClass::EditorRange,
    HistoryObjectClass::NotebookCellGroup,
    HistoryObjectClass::ResultRowSet,
    HistoryObjectClass::DocsSection,
    HistoryObjectClass::PreviewTarget,
    HistoryObjectClass::ReviewItem,
    HistoryObjectClass::RuntimeSession,
];

fn validate_coverage(
    packet: &HistoryContinuityPacket,
    violations: &mut Vec<HistoryContinuityViolation>,
) {
    let surface_kinds = packet.represented_surface_kinds();
    for required in REQUIRED_SURFACE_KINDS {
        if !surface_kinds.contains(&required) {
            violations.push(HistoryContinuityViolation::RequiredSurfaceKindMissing);
            break;
        }
    }

    let object_classes = packet.represented_object_classes();
    for required in REQUIRED_OBJECT_CLASSES {
        if !object_classes.contains(&required) {
            violations.push(HistoryContinuityViolation::RequiredObjectClassMissing);
            break;
        }
    }

    let resolutions = packet.represented_resolutions();
    for required in RecoveryAffordanceClass::ALL {
        if !resolutions.contains(&required) {
            violations.push(HistoryContinuityViolation::ResolutionCoverageMissing);
            break;
        }
    }

    if !packet
        .records
        .iter()
        .any(|record| record.must_not_flatten() && record.is_complete())
    {
        violations.push(HistoryContinuityViolation::ForcedRecordCaseMissing);
    }

    if !packet.records.iter().any(|record| {
        record.resolution.is_flat_baseline() && !record.must_not_flatten() && record.is_complete()
    }) {
        violations.push(HistoryContinuityViolation::FlatBaselineMissing);
    }

    if !packet.records.iter().any(|record| {
        record.resolution == RecoveryAffordanceClass::BackForwardContinuityPreserved
            && record.entry_kind.is_navigation()
            && record.is_complete()
    }) {
        violations.push(HistoryContinuityViolation::NavigationContinuityCaseMissing);
    }

    if packet.mutation_record_count() == 0 {
        violations.push(HistoryContinuityViolation::MutationCaseMissing);
    }

    if !packet.records.iter().any(|record| {
        record.resolution == RecoveryAffordanceClass::ReopenOrRecoverLabeled
            && record.loss_cause == SurfaceLossCause::IntentionalClose
            && record.is_complete()
    }) {
        violations.push(HistoryContinuityViolation::IntentionalReopenCaseMissing);
    }

    if !packet.records.iter().any(|record| {
        record.resolution == RecoveryAffordanceClass::ReopenOrRecoverLabeled
            && record.loss_cause.is_unintended_loss()
            && record.is_complete()
    }) {
        violations.push(HistoryContinuityViolation::LossRecoverCaseMissing);
    }

    if packet.provider_or_imported_record_count() == 0 {
        violations.push(HistoryContinuityViolation::ProviderOrImportedCaseMissing);
    }
}

fn validate_records(
    packet: &HistoryContinuityPacket,
    violations: &mut Vec<HistoryContinuityViolation>,
) {
    for record in &packet.records {
        if !record.is_complete() {
            violations.push(HistoryContinuityViolation::RecordIncomplete);
        }
        if record.silently_flattens() {
            violations.push(HistoryContinuityViolation::SilentFlatteningOfHistory);
        }
        if !record.resolution_meets_floor() {
            violations.push(HistoryContinuityViolation::ResolutionBelowRequiredFloor);
        }
        if !record.triggers_consistent() {
            violations.push(HistoryContinuityViolation::TriggerSetInconsistent);
        }
        if !record.resolution_detail_consistent() {
            violations.push(HistoryContinuityViolation::ResolutionDetailInconsistent);
        }
        if !record.undo_class_consistent() {
            violations.push(HistoryContinuityViolation::UndoClassInconsistent);
        }
        if !record.loss_cause_consistent() {
            violations.push(HistoryContinuityViolation::LossCauseInconsistent);
        }
        if !record.reopen_recover_consistent() {
            violations.push(HistoryContinuityViolation::ReopenRecoverInconsistent);
        }
        if !record.affected_object.is_valid() {
            violations.push(HistoryContinuityViolation::AffectedObjectMissing);
        }
        if !record.imported_posture_consistent() {
            violations.push(HistoryContinuityViolation::ImportedReadsAsLocal);
        }
        if !record.verification.is_well_formed() {
            violations.push(HistoryContinuityViolation::VerificationProofNotReopenable);
        }
        if record.evidence_refs.is_empty()
            || record.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(HistoryContinuityViolation::RecordEvidenceMissing);
        }
        if !record.subject.fingerprint_independent_of_id() {
            violations.push(HistoryContinuityViolation::FingerprintSubstitutesIdentity);
        }
        if !record.no_raw_boundary_material() {
            violations.push(HistoryContinuityViolation::RawBoundaryMaterialPresent);
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "undo"
            | "redo"
            | "back"
            | "forward"
            | "group"
            | "changed"
            | "recovered"
            | "restored"
            | "reopened"
            | "regenerated"
            | "compensated"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key") || lower.contains("password") || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Stable packet id minted by [`seeded_history_continuity_packet`].
pub const SEED_HISTORY_CONTINUITY_PACKET_ID: &str = "m5-history-continuity:stable:0001";

/// Mint timestamp used by [`seeded_history_continuity_packet`].
pub const SEED_HISTORY_CONTINUITY_MINTED_AT: &str = "2026-06-14T00:00:00Z";

/// Builds the canonical, validating grouped-history continuity packet that the
/// checked-in support export, the Markdown summary, and the conformance tests all
/// share, so the in-crate builder stays byte-aligned with the artifact.
///
/// The seed anchors a clean flat exact-step baseline (an editor-core range edit),
/// then exercises each non-default recovery on a distinct M5 surface: a broad
/// notebook reformat undone as one named exact group, a preview navigation whose
/// cross-surface back/forward identity is preserved, a docs autoformat exposed as
/// a labeled compensating action, a data/API result regenerated from its source
/// query, a review state restored from a checkpoint, a runtime session reopened
/// after a disconnect loss, and a provider-linked companion thread reopened after
/// an intentional close whose imported proof never reads as a local history.
pub fn seeded_history_continuity_packet() -> HistoryContinuityPacket {
    HistoryContinuityPacket::new(HistoryContinuityPacketInput {
        packet_id: SEED_HISTORY_CONTINUITY_PACKET_ID.to_owned(),
        label: "M5 Grouped-History Continuity: Named Undo Groups, Exact-versus-Compensating Recovery Labels, Back/Forward Continuity, and Reopen-Closed Affordances"
            .to_owned(),
        records: seeded_records(),
        guardrails: HistoryContinuityGuardrails {
            distinct_classes_never_flattened: true,
            exact_versus_compensating_distinguished: true,
            back_forward_continuity_preserved: true,
            reopen_distinguishes_intentional_from_loss: true,
            timestamps_or_source_preserved: true,
            provider_history_never_reads_as_local: true,
            no_new_macro_language_introduced: true,
        },
        consumer_projection: HistoryContinuityConsumerProjection {
            product_ingests_packet: true,
            help_migration_ingests_packet: true,
            support_export_ingests_packet: true,
            release_control_ingests_packet: true,
            history_classes_and_recovery_labels_nameable: true,
        },
        verification_freshness: HistoryContinuityFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_HISTORY_CONTINUITY_MINTED_AT.to_owned(),
            auto_escalate_on_stale: true,
        },
        source_contract_refs: vec![
            HISTORY_CONTINUITY_SCHEMA_REF.to_owned(),
            HISTORY_CONTINUITY_DOC_REF.to_owned(),
            HISTORY_CONTINUITY_ARTIFACT_REF.to_owned(),
            KEYBOARD_CONTINUITY_MATRIX_DOC_REF.to_owned(),
            HISTORY_RECOVERY_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_HISTORY_CONTINUITY_MINTED_AT.to_owned(),
    })
}

fn seeded_records() -> Vec<HistoryContinuityRecord> {
    vec![
        editor_core_exact_step_record(),
        notebook_named_group_record(),
        preview_back_forward_record(),
        docs_compensating_record(),
        data_api_regenerate_record(),
        review_checkpoint_record(),
        runtime_loss_reopen_record(),
        companion_intentional_reopen_record(),
    ]
}

/// Builds a verification proof keyed by a non-display fingerprint distinct from
/// the record id.
fn proof_for(record_id: &str, currency: AxisProofCurrency, summary: &str) -> AxisVerification {
    let (proof_ref, proof_fingerprint_token) = if currency.is_absent() {
        (None, None)
    } else {
        (
            Some(format!("evidence:{record_id}")),
            Some(format!("fp:proof:{record_id}")),
        )
    };
    AxisVerification {
        proof_currency: currency,
        proof_ref,
        proof_fingerprint_token,
        summary: summary.to_owned(),
    }
}

/// Builds a subject whose fingerprint is independent of its surface id.
fn subject_for(record_id: &str, origin_class: SurfaceOriginClass) -> KeyboardSurfaceSubject {
    KeyboardSurfaceSubject {
        surface_id: format!("surface:{record_id}"),
        origin_class,
        surface_fingerprint_token: format!("fp:surface:{record_id}"),
    }
}

#[allow(clippy::too_many_arguments)]
fn affected(
    object_class: HistoryObjectClass,
    object_token: &str,
    object_count: u32,
    file_count: u32,
    display_label: &str,
) -> AffectedObjectSummary {
    AffectedObjectSummary {
        object_class,
        object_token: object_token.to_owned(),
        object_count,
        file_count,
        display_label: display_label.to_owned(),
    }
}

fn editor_core_exact_step_record() -> HistoryContinuityRecord {
    let record_id = "history:editor-core:0001";
    HistoryContinuityRecord::new(HistoryContinuityRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::EditorCore,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Editor-core range edit recorded as a single exact-step undo, honestly named".to_owned(),
        entry_kind: HistoryEntryKind::Mutation,
        affected_object: affected(
            HistoryObjectClass::EditorRange,
            "range:src/lib.rs#L40-L52",
            1,
            1,
            "Editor range in src/lib.rs",
        ),
        source_attribution: SourceAttributionClass::UserDirect,
        undo_class: UndoClass::ExactUndo,
        history_class: HistoryClass::LinearStepHistory,
        reopen_recover: ReopenRecoverClass::ExactReopen,
        loss_cause: SurfaceLossCause::NotClosed,
        closed_at: None,
        literally_invertible: true,
        generated_or_automated: false,
        cross_surface_navigation: false,
        checkpoint_only: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Editor-core range edit verified to undo as one exact inverse step",
        ),
        resolution: RecoveryAffordanceClass::ExactStepUndo,
        fired_triggers: vec![],
        group_label: None,
        continuity_note: None,
        compensating_label: None,
        regenerate_source_label: None,
        checkpoint_label: None,
        reopen_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_HISTORY_CONTINUITY_MINTED_AT.to_owned(),
    })
}

fn notebook_named_group_record() -> HistoryContinuityRecord {
    let record_id = "history:notebook:0001";
    HistoryContinuityRecord::new(HistoryContinuityRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::NotebookSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Notebook reformat across four cells undone as one named exact group, not four opaque steps"
                .to_owned(),
        entry_kind: HistoryEntryKind::Mutation,
        affected_object: affected(
            HistoryObjectClass::NotebookCellGroup,
            "cells:notebook/analysis#cell-2..cell-5",
            4,
            1,
            "Notebook cells 2–5",
        ),
        source_attribution: SourceAttributionClass::UserDirect,
        undo_class: UndoClass::GroupedExactUndo,
        history_class: HistoryClass::GroupedStepHistory,
        reopen_recover: ReopenRecoverClass::ExactReopen,
        loss_cause: SurfaceLossCause::NotClosed,
        closed_at: None,
        literally_invertible: true,
        generated_or_automated: false,
        cross_surface_navigation: false,
        checkpoint_only: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Notebook reformat verified to undo the four-cell group exactly as one named unit",
        ),
        resolution: RecoveryAffordanceClass::NamedGroupExactUndo,
        fired_triggers: vec![HistoryContractTrigger::BroadMultiObjectMutation],
        group_label: Some(
            "Reformat cells 2–5 — one named group that undoes all four cell edits exactly as a unit"
                .to_owned(),
        ),
        continuity_note: None,
        compensating_label: None,
        regenerate_source_label: None,
        checkpoint_label: None,
        reopen_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_HISTORY_CONTINUITY_MINTED_AT.to_owned(),
    })
}

fn preview_back_forward_record() -> HistoryContinuityRecord {
    let record_id = "history:preview:0001";
    HistoryContinuityRecord::new(HistoryContinuityRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::PreviewSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Preview view-source navigation keeps back/forward identity across the preview and editor surfaces"
                .to_owned(),
        entry_kind: HistoryEntryKind::Navigation,
        affected_object: affected(
            HistoryObjectClass::PreviewTarget,
            "target:preview/home#section-pricing",
            1,
            1,
            "Preview pricing section ↔ source",
        ),
        source_attribution: SourceAttributionClass::UserDirect,
        undo_class: UndoClass::NoUndoHonest,
        history_class: HistoryClass::CrossSurfaceContinuity,
        reopen_recover: ReopenRecoverClass::ExactReopen,
        loss_cause: SurfaceLossCause::NotClosed,
        closed_at: None,
        literally_invertible: true,
        generated_or_automated: false,
        cross_surface_navigation: true,
        checkpoint_only: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Preview→source jump verified to keep a back target that returns to the exact preview scroll position",
        ),
        resolution: RecoveryAffordanceClass::BackForwardContinuityPreserved,
        fired_triggers: vec![HistoryContractTrigger::CrossSurfaceNavigation],
        group_label: None,
        continuity_note: Some(
            "Back returns to the preview pricing section at its prior scroll position; forward re-opens the source range — identity preserved across surfaces"
                .to_owned(),
        ),
        compensating_label: None,
        regenerate_source_label: None,
        checkpoint_label: None,
        reopen_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_HISTORY_CONTINUITY_MINTED_AT.to_owned(),
    })
}

fn docs_compensating_record() -> HistoryContinuityRecord {
    let record_id = "history:docs:0001";
    HistoryContinuityRecord::new(HistoryContinuityRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DocsSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Docs publish exposed as a labeled compensating action because it cannot be exactly inverted"
                .to_owned(),
        entry_kind: HistoryEntryKind::Mutation,
        affected_object: affected(
            HistoryObjectClass::DocsSection,
            "section:docs/guide#getting-started",
            1,
            1,
            "Docs getting-started section",
        ),
        source_attribution: SourceAttributionClass::UserDirect,
        undo_class: UndoClass::CompensatingUndo,
        history_class: HistoryClass::LinearStepHistory,
        reopen_recover: ReopenRecoverClass::RecoveredApproximate,
        loss_cause: SurfaceLossCause::NotClosed,
        closed_at: None,
        literally_invertible: false,
        generated_or_automated: false,
        cross_surface_navigation: false,
        checkpoint_only: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Docs publish verified to expose an unpublish compensating action rather than a false exact undo",
        ),
        resolution: RecoveryAffordanceClass::CompensatingActionLabeled,
        fired_triggers: vec![HistoryContractTrigger::NotLiterallyInvertible],
        group_label: None,
        continuity_note: None,
        compensating_label: Some(
            "Publish has no literal inverse; the history offers an explicit 'unpublish and restore prior draft' compensating action"
                .to_owned(),
        ),
        regenerate_source_label: None,
        checkpoint_label: None,
        reopen_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_HISTORY_CONTINUITY_MINTED_AT.to_owned(),
    })
}

fn data_api_regenerate_record() -> HistoryContinuityRecord {
    let record_id = "history:data-api:0001";
    HistoryContinuityRecord::new(HistoryContinuityRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DataApiSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Data/API result set recovered by regenerating from its source query, attributed to automation"
                .to_owned(),
        entry_kind: HistoryEntryKind::Mutation,
        affected_object: affected(
            HistoryObjectClass::ResultRowSet,
            "result:query/orders-by-region#run-12",
            1,
            1,
            "Orders-by-region result set",
        ),
        source_attribution: SourceAttributionClass::GeneratedFromSource,
        undo_class: UndoClass::CompensatingUndo,
        history_class: HistoryClass::LinearStepHistory,
        reopen_recover: ReopenRecoverClass::RecoveredApproximate,
        loss_cause: SurfaceLossCause::NotClosed,
        closed_at: None,
        literally_invertible: true,
        generated_or_automated: true,
        cross_surface_navigation: false,
        checkpoint_only: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Data/API result verified to regenerate deterministically from its source query rather than diffing rows",
        ),
        resolution: RecoveryAffordanceClass::RegenerateFromSource,
        fired_triggers: vec![HistoryContractTrigger::GeneratedOrAutomatedChange],
        group_label: None,
        continuity_note: None,
        compensating_label: None,
        regenerate_source_label: Some(
            "Result was generated from the saved query; recovery re-runs that source query rather than restoring a row snapshot"
                .to_owned(),
        ),
        checkpoint_label: None,
        reopen_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_HISTORY_CONTINUITY_MINTED_AT.to_owned(),
    })
}

fn review_checkpoint_record() -> HistoryContinuityRecord {
    let record_id = "history:review:0001";
    HistoryContinuityRecord::new(HistoryContinuityRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::ReviewSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Review thread state restored from a checkpoint rather than a step inverse"
                .to_owned(),
        entry_kind: HistoryEntryKind::Mutation,
        affected_object: affected(
            HistoryObjectClass::ReviewItem,
            "thread:review/pr-204#thread-9",
            1,
            1,
            "Review thread 9 on pull request 204",
        ),
        source_attribution: SourceAttributionClass::CheckpointSystem,
        undo_class: UndoClass::CheckpointRestore,
        history_class: HistoryClass::CheckpointLineage,
        reopen_recover: ReopenRecoverClass::CheckpointRecover,
        loss_cause: SurfaceLossCause::NotClosed,
        closed_at: None,
        literally_invertible: true,
        generated_or_automated: false,
        cross_surface_navigation: false,
        checkpoint_only: true,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Review thread verified to restore from its labeled checkpoint, with the checkpoint lineage preserved",
        ),
        resolution: RecoveryAffordanceClass::CheckpointRestoreLabeled,
        fired_triggers: vec![HistoryContractTrigger::CheckpointOnlyRecovery],
        group_label: None,
        continuity_note: None,
        compensating_label: None,
        regenerate_source_label: None,
        checkpoint_label: Some(
            "Restore review thread 9 to checkpoint taken before the bulk resolve — a snapshot restore, not a step inverse"
                .to_owned(),
        ),
        reopen_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_HISTORY_CONTINUITY_MINTED_AT.to_owned(),
    })
}

fn runtime_loss_reopen_record() -> HistoryContinuityRecord {
    let record_id = "history:runtime:0001";
    HistoryContinuityRecord::new(HistoryContinuityRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::RuntimeSurface,
        subject: subject_for(record_id, SurfaceOriginClass::EmbeddedRuntimeSurface),
        label_summary:
            "Runtime session recovered after a disconnect loss, labeled as a loss recovery rather than an intentional reopen"
                .to_owned(),
        entry_kind: HistoryEntryKind::Mutation,
        affected_object: affected(
            HistoryObjectClass::RuntimeSession,
            "session:runtime/dev-server#pid-8841",
            1,
            1,
            "Embedded runtime dev-server session",
        ),
        source_attribution: SourceAttributionClass::AutomationAgent,
        undo_class: UndoClass::NoUndoHonest,
        history_class: HistoryClass::LinearStepHistory,
        reopen_recover: ReopenRecoverClass::RecoveredApproximate,
        loss_cause: SurfaceLossCause::DisconnectLoss,
        closed_at: Some("2026-06-13T22:14:05Z".to_owned()),
        literally_invertible: true,
        generated_or_automated: false,
        cross_surface_navigation: false,
        checkpoint_only: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Runtime session verified to recover after a disconnect, labeled as a loss recovery with its disconnect timestamp preserved",
        ),
        resolution: RecoveryAffordanceClass::ReopenOrRecoverLabeled,
        fired_triggers: vec![HistoryContractTrigger::SurfaceClosedOrLost],
        group_label: None,
        continuity_note: None,
        compensating_label: None,
        regenerate_source_label: None,
        checkpoint_label: None,
        reopen_label: Some(
            "Dev-server session was lost to a runtime disconnect at 22:14:05Z; recovery restores it as an approximate session and says so — not presented as a clean reopen"
                .to_owned(),
        ),
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_HISTORY_CONTINUITY_MINTED_AT.to_owned(),
    })
}

fn companion_intentional_reopen_record() -> HistoryContinuityRecord {
    let record_id = "history:companion:0001";
    HistoryContinuityRecord::new(HistoryContinuityRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::CompanionSurface,
        subject: subject_for(record_id, SurfaceOriginClass::ProviderLinkedSurface),
        label_summary:
            "Provider-linked companion thread reopened after an intentional close; imported proof never reads as local"
                .to_owned(),
        entry_kind: HistoryEntryKind::Mutation,
        affected_object: affected(
            HistoryObjectClass::CompanionThread,
            "thread:companion/assistant#thread-77",
            1,
            1,
            "Provider-backed companion thread 77",
        ),
        source_attribution: SourceAttributionClass::ProviderSync,
        undo_class: UndoClass::NoUndoHonest,
        history_class: HistoryClass::CrossSurfaceContinuity,
        reopen_recover: ReopenRecoverClass::ExactReopen,
        loss_cause: SurfaceLossCause::IntentionalClose,
        closed_at: Some("2026-06-13T20:02:00Z".to_owned()),
        literally_invertible: true,
        generated_or_automated: false,
        cross_surface_navigation: false,
        checkpoint_only: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::ImportedCurrent,
            "Provider-backed companion reopen verified with imported proof, labeled an intentional close with its close timestamp preserved",
        ),
        resolution: RecoveryAffordanceClass::ReopenOrRecoverLabeled,
        fired_triggers: vec![HistoryContractTrigger::SurfaceClosedOrLost],
        group_label: None,
        continuity_note: None,
        compensating_label: None,
        regenerate_source_label: None,
        checkpoint_label: None,
        reopen_label: Some(
            "Companion thread 77 was intentionally closed at 20:02:00Z; reopen restores its provider-backed thread context exactly, distinct from a crash recovery"
                .to_owned(),
        ),
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_HISTORY_CONTINUITY_MINTED_AT.to_owned(),
    })
}

/// Packet id minted by [`fixture_history_continuity_packet`].
pub const FIXTURE_HISTORY_CONTINUITY_PACKET_ID: &str =
    "m5-history-continuity:fixture:stale-proof-forces-named-group:0001";

/// Builds the protected fixture variant: it keeps the full seeded record set —
/// including the clean flat exact-step baseline — and adds one drill record for an
/// editor-core range edit that would otherwise be a flat exact-step undo but is
/// forced off the flat lane because its history proof aged outside the freshness
/// window.
///
/// The fixture is a *valid* packet: the drill record correctly records the
/// [`HistoryContractTrigger::StaleOrMissingHistoryProof`] trigger and resolves to
/// [`RecoveryAffordanceClass::NamedGroupExactUndo`] with a precise group label, so
/// it validates while demonstrating that stale evidence — not just a broad,
/// cross-surface, non-invertible, generated, checkpoint-only, or closed-surface
/// entry — forces a history entry off the flat single-step lane.
pub fn fixture_history_continuity_packet() -> HistoryContinuityPacket {
    let mut packet = seeded_history_continuity_packet();
    packet.packet_id = FIXTURE_HISTORY_CONTINUITY_PACKET_ID.to_owned();
    packet.label =
        "M5 Grouped-History Continuity fixture: stale history proof forces a flat exact-step undo into a named re-verified group"
            .to_owned();
    packet.records.push(stale_proof_drill_record());
    packet
}

/// An editor-core range edit that would undo as one flat exact step, but whose
/// history proof has aged outside its freshness window, so it is forced into a
/// named re-verified group recovery.
fn stale_proof_drill_record() -> HistoryContinuityRecord {
    let record_id = "history:editor-core:stale-proof:0001";
    HistoryContinuityRecord::new(HistoryContinuityRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::EditorCore,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Editor-core range edit whose stale history proof forces it into a named re-verified group"
                .to_owned(),
        entry_kind: HistoryEntryKind::Mutation,
        affected_object: affected(
            HistoryObjectClass::EditorRange,
            "range:src/lib.rs#L88-L96",
            1,
            1,
            "Editor range in src/lib.rs with stale proof",
        ),
        source_attribution: SourceAttributionClass::UserDirect,
        undo_class: UndoClass::GroupedExactUndo,
        history_class: HistoryClass::GroupedStepHistory,
        reopen_recover: ReopenRecoverClass::ExactReopen,
        loss_cause: SurfaceLossCause::NotClosed,
        closed_at: None,
        literally_invertible: true,
        generated_or_automated: false,
        cross_surface_navigation: false,
        checkpoint_only: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::StaleExpired,
            "Editor-core range edit proof aged outside its freshness window",
        ),
        resolution: RecoveryAffordanceClass::NamedGroupExactUndo,
        fired_triggers: vec![HistoryContractTrigger::StaleOrMissingHistoryProof],
        group_label: Some(
            "History proof aged outside its freshness window; the edit is offered as a named, re-verified exact-undo group rather than a bare step"
                .to_owned(),
        ),
        continuity_note: None,
        compensating_label: None,
        regenerate_source_label: None,
        checkpoint_label: None,
        reopen_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_HISTORY_CONTINUITY_MINTED_AT.to_owned(),
    })
}
