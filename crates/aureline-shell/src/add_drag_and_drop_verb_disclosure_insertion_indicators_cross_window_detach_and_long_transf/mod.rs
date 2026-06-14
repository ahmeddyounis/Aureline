//! M5 drag/drop transfer-safety packet: verb disclosure, insertion-point
//! indicators, cross-window detach/reattach continuity, and long-transfer
//! progress / cancellation / post-action summary across M5 drag-and-drop
//! surfaces.
//!
//! Aureline's switching promise depends on keyboard-first, recoverable
//! interaction across every new M5 surface — editor, notebook, data/API,
//! preview, docs, review, runtime, and companion-adjacent panes. The frozen
//! keyboard-continuity matrix
//! [`crate::freeze_the_m5_keyboard_mode_modal_sequence_clipboard_route_drag_drop_verb_and_grouped_history_matrix`]
//! pins those surfaces to their canonical interaction vocabulary and requires
//! that *drag/drop never hides verbs or scope*. This module discharges the
//! drag/drop half of that contract: it takes the frozen [`DragDropVerbClass`]
//! vocabulary and makes a drop **trustworthy and recoverable** on the new M5
//! transfer surfaces by advertising the resulting verb and insertion point
//! before a drop commits, preserving context and recovery when a transfer
//! detaches across windows, and tracking progress, cancellation, and a
//! post-action summary for large transfers instead of freezing into a generic
//! spinner.
//!
//! * a [`TransferSafetyRecord`] binds a claimed M5 surface (keyed by a
//!   [`KeyboardSurfaceKind`] and a non-display [`KeyboardSurfaceSubject`]) to one
//!   drop attempt: the [`TransferObjectRef`] it drags (a [`TransferObjectClass`]
//!   plus an opaque / workspace-relative object token), the resolved
//!   [`DragDropVerbClass`] the drop performs, the [`WindowScopeClass`] the
//!   transfer crosses, the [`TransferMagnitudeClass`] of the payload, and the
//!   resolved [`TransferDisclosureClass`];
//! * a drop is **never a silent commit when it is non-trivial**: a record whose
//!   verb is chosen on a modifier (multiple verbs possible), that detaches across
//!   windows, that carries a large payload, that imports / mutates the
//!   destination, that rests on a destructive default verb, or whose transfer
//!   proof is stale or missing fires one or more [`TransferContractTrigger`]s.
//!   Each trigger imposes a minimum-safety floor on the resolution, so a
//!   triggered record can never resolve to
//!   [`TransferDisclosureClass::DisclosedInlineCommit`]; it must disclose the
//!   explicit verb choice, preserve cross-window continuity, track progress /
//!   cancel / summary, confirm before mutation, or be rejected;
//! * the verb and insertion point are **always disclosed before commit**: every
//!   record records that the resulting verb was advertised before the drop
//!   committed, and that the insertion point was shown before commit (except a
//!   rejected drop that never reaches an insertion point), so drag/drop never
//!   hides a verb or scope and is never destructive or ambiguous by default.
//!
//! [`TransferSafetyPacket::validate`] refuses a packet that lets a non-trivial
//! drop commit silently, that lowers a resolution below its required safety
//! floor, that hides the resulting verb or insertion point before commit, that
//! orphans a detached tab / asset, that commits a destructive default verb, or
//! that lets a provider-linked surface read as a locally verified transfer.
//!
//! Raw drag payload byte buffers, raw provider payloads, file contents, and
//! absolute private paths never cross this boundary; the packet carries only
//! typed class tokens, booleans, opaque / relative ids, fingerprint digests, and
//! redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/interaction/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf.schema.json`](../../../../schemas/interaction/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf.schema.json).
//! The contract doc is
//! [`docs/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf.md`](../../../../docs/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf.md).
//! The protected fixture directory is
//! [`fixtures/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf/`](../../../../fixtures/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf/).

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
    AxisProofCurrency, AxisVerification, DragDropVerbClass, KeyboardSurfaceKind,
    KeyboardSurfaceSubject, SurfaceOriginClass, KEYBOARD_CONTINUITY_MATRIX_DOC_REF,
};

/// Stable record-kind tag carried by [`TransferSafetyPacket`].
pub const TRANSFER_SAFETY_RECORD_KIND: &str =
    "m5_drag_drop_transfer_safety_verb_insertion_cross_window_long_transfer_packet";

/// Schema version for the transfer-safety packet.
pub const TRANSFER_SAFETY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const TRANSFER_SAFETY_SCHEMA_REF: &str =
    "schemas/interaction/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf.schema.json";

/// Repo-relative path of the contract doc.
pub const TRANSFER_SAFETY_DOC_REF: &str =
    "docs/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf.md";

/// Repo-relative path of the checked support-export artifact.
pub const TRANSFER_SAFETY_ARTIFACT_REF: &str =
    "artifacts/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const TRANSFER_SAFETY_SUMMARY_REF: &str =
    "artifacts/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf.md";

/// Repo-relative path of the protected fixture directory.
pub const TRANSFER_SAFETY_FIXTURE_DIR: &str =
    "fixtures/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf";

/// Source contract ref of the clipboard / transfer / drag-drop history contract.
pub const DRAG_DROP_TRANSFER_CONTRACT_REF: &str = "docs/ux/clipboard_history_contract.md";

/// Class of object a drop drags onto a claimed M5 surface. The class lets help,
/// migration, and support name the same drag/drop targets the product exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferObjectClass {
    /// A notebook cell (code / markdown / output cell).
    NotebookCell,
    /// A data-grid / API result row.
    ResultGridRow,
    /// An artifact / evidence item.
    ArtifactItem,
    /// A work item (task / review item).
    WorkItem,
    /// A preview / preview-runtime asset.
    PreviewRuntimeAsset,
    /// An editor text fragment (the editor-core baseline).
    EditorTextFragment,
}

impl TransferObjectClass {
    /// Every object class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotebookCell,
        Self::ResultGridRow,
        Self::ArtifactItem,
        Self::WorkItem,
        Self::PreviewRuntimeAsset,
        Self::EditorTextFragment,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookCell => "notebook_cell",
            Self::ResultGridRow => "result_grid_row",
            Self::ArtifactItem => "artifact_item",
            Self::WorkItem => "work_item",
            Self::PreviewRuntimeAsset => "preview_runtime_asset",
            Self::EditorTextFragment => "editor_text_fragment",
        }
    }
}

/// Window scope a transfer crosses. A cross-window detach (or its reattach) must
/// preserve context and recovery rather than silently orphaning a tab or asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WindowScopeClass {
    /// A reorder / move within the same pane.
    SamePaneReorder,
    /// A transfer across panes inside the same window.
    CrossPaneSameWindow,
    /// A detach that tears the object out into another window.
    CrossWindowDetach,
    /// A reattach that folds a detached object back into a window.
    ReattachToWindow,
}

impl WindowScopeClass {
    /// Every window scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SamePaneReorder,
        Self::CrossPaneSameWindow,
        Self::CrossWindowDetach,
        Self::ReattachToWindow,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SamePaneReorder => "same_pane_reorder",
            Self::CrossPaneSameWindow => "cross_pane_same_window",
            Self::CrossWindowDetach => "cross_window_detach",
            Self::ReattachToWindow => "reattach_to_window",
        }
    }

    /// Whether this scope detaches or reattaches across windows, so it must
    /// preserve cross-window context and recovery.
    pub const fn is_cross_window(self) -> bool {
        matches!(self, Self::CrossWindowDetach | Self::ReattachToWindow)
    }
}

/// Magnitude of the payload a transfer moves. A large transfer must show
/// progress, cancellation, and a post-action summary rather than freezing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferMagnitudeClass {
    /// A small / instantaneous transfer with no progress ceremony needed.
    TrivialInline,
    /// A large paste / import / attach / drop that needs progress tracking.
    LargeNeedsProgress,
}

impl TransferMagnitudeClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrivialInline => "trivial_inline",
            Self::LargeNeedsProgress => "large_needs_progress",
        }
    }

    /// Whether this magnitude needs a progress / cancel / summary affordance.
    pub const fn needs_progress(self) -> bool {
        matches!(self, Self::LargeNeedsProgress)
    }
}

/// Resolved transfer-safety posture a surface exposes for one drop attempt. This
/// is the canonical transfer-disclosure vocabulary help, migration, and support
/// name.
///
/// Only [`Self::DisclosedInlineCommit`] commits inline without extra ceremony;
/// every other resolution discloses the explicit verb choice, preserves
/// cross-window continuity, tracks progress / cancel / summary, confirms before a
/// destructive / import mutation, or rejects an ambiguous / unsafe transfer. The
/// [`Self::safety_rank`] orders the resolutions so a triggered record can be held
/// at or above a required floor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferDisclosureClass {
    /// The verb and insertion point are shown before an inline, instantaneous
    /// commit in the same context.
    DisclosedInlineCommit,
    /// The resulting verb (move / copy / import / open / split) is explicitly
    /// chosen and advertised before commit.
    ExplicitVerbChoiceDisclosed,
    /// A cross-window detach / reattach preserves context and recovery rather than
    /// orphaning the tab or asset.
    CrossWindowContinuityPreserved,
    /// A large transfer shows progress, a cancel control, and a post-action
    /// summary.
    ProgressCancelSummaryTracked,
    /// A destructive / import semantic is confirmed (verb and scope) before the
    /// destination state mutates.
    ConfirmedBeforeMutation,
    /// An ambiguous, destructive-default, or otherwise unsafe transfer is
    /// rejected.
    RejectedAmbiguousOrUnsafe,
}

impl TransferDisclosureClass {
    /// Every resolution class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DisclosedInlineCommit,
        Self::ExplicitVerbChoiceDisclosed,
        Self::CrossWindowContinuityPreserved,
        Self::ProgressCancelSummaryTracked,
        Self::ConfirmedBeforeMutation,
        Self::RejectedAmbiguousOrUnsafe,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisclosedInlineCommit => "disclosed_inline_commit",
            Self::ExplicitVerbChoiceDisclosed => "explicit_verb_choice_disclosed",
            Self::CrossWindowContinuityPreserved => "cross_window_continuity_preserved",
            Self::ProgressCancelSummaryTracked => "progress_cancel_summary_tracked",
            Self::ConfirmedBeforeMutation => "confirmed_before_mutation",
            Self::RejectedAmbiguousOrUnsafe => "rejected_ambiguous_or_unsafe",
        }
    }

    /// Monotonic safety rank; higher is a stronger / more restrictive resolution,
    /// so a triggered record must hold a resolution whose rank meets its floor.
    pub const fn safety_rank(self) -> u8 {
        match self {
            Self::DisclosedInlineCommit => 0,
            Self::ExplicitVerbChoiceDisclosed => 1,
            Self::CrossWindowContinuityPreserved => 2,
            Self::ProgressCancelSummaryTracked => 3,
            Self::ConfirmedBeforeMutation => 4,
            Self::RejectedAmbiguousOrUnsafe => 5,
        }
    }

    /// Whether this resolution commits inline without escalation.
    pub const fn is_inline_commit(self) -> bool {
        matches!(self, Self::DisclosedInlineCommit)
    }

    /// Whether this resolution must cite a `verb_choice_label`.
    pub const fn requires_verb_choice_label(self) -> bool {
        matches!(self, Self::ExplicitVerbChoiceDisclosed)
    }

    /// Whether this resolution must cite a `continuity_note`.
    pub const fn requires_continuity_note(self) -> bool {
        matches!(self, Self::CrossWindowContinuityPreserved)
    }

    /// Whether this resolution must cite a `progress_note`.
    pub const fn requires_progress_note(self) -> bool {
        matches!(self, Self::ProgressCancelSummaryTracked)
    }

    /// Whether this resolution must cite a `confirmation_label`.
    pub const fn requires_confirmation_label(self) -> bool {
        matches!(self, Self::ConfirmedBeforeMutation)
    }

    /// Whether this resolution must cite a `rejection_reason_label`.
    pub const fn requires_rejection_reason(self) -> bool {
        matches!(self, Self::RejectedAmbiguousOrUnsafe)
    }
}

/// Why a record's drop was held off the inline-commit lane. Each trigger imposes
/// a minimum-safety floor; the chrome quotes the trigger verbatim instead of a
/// generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransferContractTrigger {
    /// The verb is chosen on a modifier (multiple verbs possible), so the choice
    /// must be advertised before commit.
    AmbiguousOrMultiVerb,
    /// The transfer detaches / reattaches across windows.
    CrossWindowDetach,
    /// The transfer payload is large and needs progress tracking.
    LargeTransferMagnitude,
    /// The drop imports / mutates the destination and must be confirmed.
    DestructiveOrImportSemantics,
    /// The drop rests on a destructive / ambiguous default verb.
    DestructiveDefaultVerb,
    /// The transfer proof backing this record is stale or missing.
    StaleOrMissingTransferProof,
}

impl TransferContractTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AmbiguousOrMultiVerb => "ambiguous_or_multi_verb",
            Self::CrossWindowDetach => "cross_window_detach",
            Self::LargeTransferMagnitude => "large_transfer_magnitude",
            Self::DestructiveOrImportSemantics => "destructive_or_import_semantics",
            Self::DestructiveDefaultVerb => "destructive_default_verb",
            Self::StaleOrMissingTransferProof => "stale_or_missing_transfer_proof",
        }
    }

    /// Minimum resolution safety rank this trigger imposes.
    ///
    /// A multi-verb drop or stale proof requires at least an explicit verb-choice
    /// disclosure; a cross-window detach requires preserved cross-window
    /// continuity; a large transfer requires progress / cancel / summary; an
    /// import / destructive mutation requires a confirmation before mutating; a
    /// destructive default verb is rejected outright.
    pub const fn minimum_resolution_rank(self) -> u8 {
        match self {
            Self::AmbiguousOrMultiVerb | Self::StaleOrMissingTransferProof => {
                TransferDisclosureClass::ExplicitVerbChoiceDisclosed.safety_rank()
            }
            Self::CrossWindowDetach => {
                TransferDisclosureClass::CrossWindowContinuityPreserved.safety_rank()
            }
            Self::LargeTransferMagnitude => {
                TransferDisclosureClass::ProgressCancelSummaryTracked.safety_rank()
            }
            Self::DestructiveOrImportSemantics => {
                TransferDisclosureClass::ConfirmedBeforeMutation.safety_rank()
            }
            Self::DestructiveDefaultVerb => {
                TransferDisclosureClass::RejectedAmbiguousOrUnsafe.safety_rank()
            }
        }
    }
}

/// The object a drop drags onto a surface. Preserved so a transfer can be named
/// and recovered rather than collapsed into opaque payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferObjectRef {
    /// Class of the dragged object.
    pub object_class: TransferObjectClass,
    /// Opaque / workspace-relative object token. Never an absolute private path.
    pub object_token: String,
    /// Reviewable label.
    pub display_label: String,
}

impl TransferObjectRef {
    /// Whether the object carries the identity a transfer needs.
    pub fn is_valid(&self) -> bool {
        !self.object_token.trim().is_empty() && !self.display_label.trim().is_empty()
    }
}

/// Constructor input for [`TransferSafetyRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferSafetyRecordInput {
    /// Stable record id.
    pub record_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the record covers.
    pub subject: KeyboardSurfaceSubject,
    /// Reviewable record label.
    pub label_summary: String,
    /// The dragged object.
    pub object: TransferObjectRef,
    /// The resolved drop verb.
    pub drop_verb: DragDropVerbClass,
    /// The window scope the transfer crosses.
    pub window_scope: WindowScopeClass,
    /// The magnitude of the payload.
    pub transfer_magnitude: TransferMagnitudeClass,
    /// Whether the drop imports / mutates the destination and must be confirmed.
    pub import_or_destructive: bool,
    /// Whether the resulting verb was advertised before the drop committed.
    pub verb_disclosed_before_commit: bool,
    /// Whether the insertion point was shown before the drop committed.
    pub insertion_point_disclosed_before_commit: bool,
    /// Reopenable verification proof backing the resolution.
    pub verification: AxisVerification,
    /// The resolved transfer-safety posture.
    pub resolution: TransferDisclosureClass,
    /// Triggers recorded as firing for this record.
    pub fired_triggers: Vec<TransferContractTrigger>,
    /// Required when `resolution` is `explicit_verb_choice_disclosed`.
    pub verb_choice_label: Option<String>,
    /// Required when `resolution` is `cross_window_continuity_preserved`.
    pub continuity_note: Option<String>,
    /// Required when `resolution` is `progress_cancel_summary_tracked`.
    pub progress_note: Option<String>,
    /// Required when `resolution` is `confirmed_before_mutation`.
    pub confirmation_label: Option<String>,
    /// Required when `resolution` is `rejected_ambiguous_or_unsafe`.
    pub rejection_reason_label: Option<String>,
    /// Evidence packet refs backing this record.
    pub evidence_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

/// One transfer-safety record binding a claimed M5 surface to one resolved drop
/// attempt with verb / insertion disclosure and a recovery posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSafetyRecord {
    /// Stable record id.
    pub record_id: String,
    /// Kind of claimed M5 surface.
    pub surface_kind: KeyboardSurfaceKind,
    /// Durable subject the record covers.
    pub subject: KeyboardSurfaceSubject,
    /// Reviewable record label.
    pub label_summary: String,
    /// The dragged object.
    pub object: TransferObjectRef,
    /// The resolved drop verb.
    pub drop_verb: DragDropVerbClass,
    /// The window scope the transfer crosses.
    pub window_scope: WindowScopeClass,
    /// The magnitude of the payload.
    pub transfer_magnitude: TransferMagnitudeClass,
    /// Whether the drop imports / mutates the destination and must be confirmed.
    pub import_or_destructive: bool,
    /// Whether the resulting verb was advertised before the drop committed.
    pub verb_disclosed_before_commit: bool,
    /// Whether the insertion point was shown before the drop committed.
    pub insertion_point_disclosed_before_commit: bool,
    /// Reopenable verification proof backing the resolution.
    pub verification: AxisVerification,
    /// The resolved transfer-safety posture.
    pub resolution: TransferDisclosureClass,
    /// Triggers recorded as firing for this record. Must equal the computed set.
    pub fired_triggers: Vec<TransferContractTrigger>,
    /// Required when `resolution` is `explicit_verb_choice_disclosed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verb_choice_label: Option<String>,
    /// Required when `resolution` is `cross_window_continuity_preserved`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub continuity_note: Option<String>,
    /// Required when `resolution` is `progress_cancel_summary_tracked`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_note: Option<String>,
    /// Required when `resolution` is `confirmed_before_mutation`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirmation_label: Option<String>,
    /// Required when `resolution` is `rejected_ambiguous_or_unsafe`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rejection_reason_label: Option<String>,
    /// Guardrail: record does not carry raw drag payload byte buffers.
    pub raw_payload_bytes_present: bool,
    /// Guardrail: record does not carry raw provider payloads.
    pub raw_provider_payload_present: bool,
    /// Guardrail: record does not carry an absolute private path.
    pub absolute_private_path_present: bool,
    /// Guardrail: no destructive default verb was committed.
    pub destructive_default_commit_taken: bool,
    /// Guardrail: no detached tab / asset was orphaned.
    pub orphaned_on_detach: bool,
    /// Evidence packet refs backing this record.
    pub evidence_refs: Vec<String>,
    /// Mint timestamp.
    pub minted_at: String,
}

impl TransferSafetyRecord {
    /// Builds a record from its input, defaulting the guardrail flags to their
    /// safe values.
    pub fn new(input: TransferSafetyRecordInput) -> Self {
        Self {
            record_id: input.record_id,
            surface_kind: input.surface_kind,
            subject: input.subject,
            label_summary: input.label_summary,
            object: input.object,
            drop_verb: input.drop_verb,
            window_scope: input.window_scope,
            transfer_magnitude: input.transfer_magnitude,
            import_or_destructive: input.import_or_destructive,
            verb_disclosed_before_commit: input.verb_disclosed_before_commit,
            insertion_point_disclosed_before_commit: input.insertion_point_disclosed_before_commit,
            verification: input.verification,
            resolution: input.resolution,
            fired_triggers: input.fired_triggers,
            verb_choice_label: input.verb_choice_label,
            continuity_note: input.continuity_note,
            progress_note: input.progress_note,
            confirmation_label: input.confirmation_label,
            rejection_reason_label: input.rejection_reason_label,
            raw_payload_bytes_present: false,
            raw_provider_payload_present: false,
            absolute_private_path_present: false,
            destructive_default_commit_taken: false,
            orphaned_on_detach: false,
            evidence_refs: input.evidence_refs,
            minted_at: input.minted_at,
        }
    }

    /// Whether the transfer for this record is provider-backed / imported.
    pub fn provider_or_imported(&self) -> bool {
        self.subject.origin_class.is_provider_or_imported()
    }

    /// Whether the verification proof backs a current transfer claim for this
    /// record's origin posture.
    pub fn transfer_proof_current(&self) -> bool {
        self.verification.backs_claim(self.provider_or_imported())
    }

    /// The set of triggers that actually fire for this record, computed from its
    /// verb, window scope, magnitude, semantics, and proof.
    pub fn computed_triggers(&self) -> BTreeSet<TransferContractTrigger> {
        let mut triggers = BTreeSet::new();
        if matches!(self.drop_verb, DragDropVerbClass::VerbChoiceOnModifier) {
            triggers.insert(TransferContractTrigger::AmbiguousOrMultiVerb);
        }
        if self.window_scope.is_cross_window() {
            triggers.insert(TransferContractTrigger::CrossWindowDetach);
        }
        if self.transfer_magnitude.needs_progress() {
            triggers.insert(TransferContractTrigger::LargeTransferMagnitude);
        }
        if self.import_or_destructive {
            triggers.insert(TransferContractTrigger::DestructiveOrImportSemantics);
        }
        if self.drop_verb.is_denied() {
            triggers.insert(TransferContractTrigger::DestructiveDefaultVerb);
        }
        if !self.transfer_proof_current() {
            triggers.insert(TransferContractTrigger::StaleOrMissingTransferProof);
        }
        triggers
    }

    /// The recorded triggers as a set.
    pub fn recorded_triggers(&self) -> BTreeSet<TransferContractTrigger> {
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

    /// Whether the drop must be held off the inline-commit lane.
    pub fn must_not_commit_silently(&self) -> bool {
        self.required_floor_rank() > 0
    }

    /// Whether the recorded resolution meets the required safety floor.
    pub fn resolution_meets_floor(&self) -> bool {
        self.resolution.safety_rank() >= self.required_floor_rank()
    }

    /// Whether the recorded resolution silently commits a drop that must not.
    pub fn silently_commits_unsafe(&self) -> bool {
        self.resolution.is_inline_commit() && self.must_not_commit_silently()
    }

    /// Whether the recorded trigger set matches the computed set.
    pub fn triggers_consistent(&self) -> bool {
        self.recorded_triggers() == self.computed_triggers()
    }

    /// Whether the resulting verb is always disclosed before commit.
    pub fn verb_disclosure_holds(&self) -> bool {
        self.verb_disclosed_before_commit
    }

    /// Whether the insertion point is disclosed before commit, unless the drop is
    /// rejected and never reaches an insertion point.
    pub fn insertion_disclosure_holds(&self) -> bool {
        self.insertion_point_disclosed_before_commit
            || self.resolution == TransferDisclosureClass::RejectedAmbiguousOrUnsafe
    }

    /// Whether the resolution carries exactly the detail field it requires.
    pub fn resolution_detail_consistent(&self) -> bool {
        let present = |opt: &Option<String>| {
            opt.as_deref()
                .is_some_and(|value| !value.trim().is_empty() && !label_is_generic(value))
        };
        let verb_ok = if self.resolution.requires_verb_choice_label() {
            present(&self.verb_choice_label)
        } else {
            self.verb_choice_label.is_none()
        };
        let continuity_ok = if self.resolution.requires_continuity_note() {
            present(&self.continuity_note)
        } else {
            self.continuity_note.is_none()
        };
        let progress_ok = if self.resolution.requires_progress_note() {
            present(&self.progress_note)
        } else {
            self.progress_note.is_none()
        };
        let confirm_ok = if self.resolution.requires_confirmation_label() {
            present(&self.confirmation_label)
        } else {
            self.confirmation_label.is_none()
        };
        let reject_ok = if self.resolution.requires_rejection_reason() {
            present(&self.rejection_reason_label)
        } else {
            self.rejection_reason_label.is_none()
        };
        verb_ok && continuity_ok && progress_ok && confirm_ok && reject_ok
    }

    /// Whether the imported posture is consistent: a provider/imported surface
    /// never reads as a locally verified transfer, and a local surface never leans
    /// on imported proof.
    pub fn imported_posture_consistent(&self) -> bool {
        if self.provider_or_imported() {
            !self.verification.proof_currency.is_current_local()
        } else {
            !self.verification.proof_currency.is_imported_current()
        }
    }

    /// Whether no raw boundary material or destructive-default / orphaning side
    /// effect is flagged present.
    pub fn no_raw_boundary_material(&self) -> bool {
        !self.raw_payload_bytes_present
            && !self.raw_provider_payload_present
            && !self.absolute_private_path_present
            && !self.destructive_default_commit_taken
            && !self.orphaned_on_detach
    }

    /// Whether every field required to record this record is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.record_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.minted_at.trim().is_empty()
            && self.subject.is_valid()
            && self.object.is_valid()
            && self.verb_disclosure_holds()
            && self.insertion_disclosure_holds()
            && self.verification.is_well_formed()
            && self.triggers_consistent()
            && !self.silently_commits_unsafe()
            && self.resolution_meets_floor()
            && self.resolution_detail_consistent()
            && self.imported_posture_consistent()
            && self.no_raw_boundary_material()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSafetyGuardrails {
    /// Every drop advertises its resulting verb before commit.
    pub verb_disclosed_before_commit: bool,
    /// Every committing drop advertises its insertion point before commit.
    pub insertion_point_disclosed_before_commit: bool,
    /// Drag/drop is never destructive or ambiguous by default.
    pub never_destructive_or_ambiguous_by_default: bool,
    /// Cross-window detach / reattach preserves context and recovery.
    pub cross_window_continuity_preserved: bool,
    /// Large transfers show progress, cancellation, and a post-action summary.
    pub long_transfer_progress_cancel_summary: bool,
    /// Provider-linked transfers never read as a locally verified transfer.
    pub provider_transfers_never_read_as_local: bool,
    /// No new general macro language or editor core is introduced here.
    pub no_new_macro_language_introduced: bool,
}

impl TransferSafetyGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.verb_disclosed_before_commit
            && self.insertion_point_disclosed_before_commit
            && self.never_destructive_or_ambiguous_by_default
            && self.cross_window_continuity_preserved
            && self.long_transfer_progress_cancel_summary
            && self.provider_transfers_never_read_as_local
            && self.no_new_macro_language_introduced
    }
}

/// Consumer projection block: the surfaces that read this packet without cloning
/// transfer-verb language by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSafetyConsumerProjection {
    /// Product surfaces ingest this packet.
    pub product_ingests_packet: bool,
    /// Help / migration guidance ingests the same packet.
    pub help_migration_ingests_packet: bool,
    /// Support / export tooling ingests the same packet.
    pub support_export_ingests_packet: bool,
    /// Release-control surfaces ingest the same packet.
    pub release_control_ingests_packet: bool,
    /// Help / migration / support can name the same transfer verbs and disclosure
    /// classes the product exposes from this packet.
    pub transfer_verbs_and_disclosures_nameable: bool,
}

impl TransferSafetyConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_packet
            && self.help_migration_ingests_packet
            && self.support_export_ingests_packet
            && self.release_control_ingests_packet
            && self.transfer_verbs_and_disclosures_nameable
    }
}

/// Verification freshness block for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSafetyFreshness {
    /// Verification-freshness SLO in hours.
    pub verification_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last verification refresh.
    pub last_verification_refresh: String,
    /// True when stale verification automatically forces records off inline commit.
    pub auto_escalate_on_stale: bool,
}

impl TransferSafetyFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.verification_freshness_slo_hours > 0
            && !self.last_verification_refresh.trim().is_empty()
    }
}

/// Constructor input for [`TransferSafetyPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferSafetyPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-surface transfer-safety records.
    pub records: Vec<TransferSafetyRecord>,
    /// Guardrail invariants block.
    pub guardrails: TransferSafetyGuardrails,
    /// Consumer projection block.
    pub consumer_projection: TransferSafetyConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: TransferSafetyFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe transfer-safety packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferSafetyPacket {
    /// Record kind; must equal [`TRANSFER_SAFETY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`TRANSFER_SAFETY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Per-surface transfer-safety records.
    pub records: Vec<TransferSafetyRecord>,
    /// Guardrail invariants block.
    pub guardrails: TransferSafetyGuardrails,
    /// Consumer projection block.
    pub consumer_projection: TransferSafetyConsumerProjection,
    /// Verification freshness block.
    pub verification_freshness: TransferSafetyFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl TransferSafetyPacket {
    /// Builds a transfer-safety packet.
    pub fn new(input: TransferSafetyPacketInput) -> Self {
        Self {
            record_kind: TRANSFER_SAFETY_RECORD_KIND.to_owned(),
            schema_version: TRANSFER_SAFETY_SCHEMA_VERSION,
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
    pub fn represented_object_classes(&self) -> BTreeSet<TransferObjectClass> {
        self.records
            .iter()
            .map(|record| record.object.object_class)
            .collect()
    }

    /// Resolution classes represented across records.
    pub fn represented_resolutions(&self) -> BTreeSet<TransferDisclosureClass> {
        self.records
            .iter()
            .map(|record| record.resolution)
            .collect()
    }

    /// Count of records held off the inline-commit lane.
    pub fn forced_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.must_not_commit_silently())
            .count()
    }

    /// Count of records resolved to an inline commit.
    pub fn inline_commit_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.resolution.is_inline_commit())
            .count()
    }

    /// Count of records resolved to cross-window continuity.
    pub fn cross_window_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| {
                record.resolution == TransferDisclosureClass::CrossWindowContinuityPreserved
            })
            .count()
    }

    /// Count of records resolved to progress / cancel / summary tracking.
    pub fn long_transfer_record_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| {
                record.resolution == TransferDisclosureClass::ProgressCancelSummaryTracked
            })
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
    pub fn record(&self, record_id: &str) -> Option<&TransferSafetyRecord> {
        self.records
            .iter()
            .find(|record| record.record_id == record_id)
    }

    /// Validates the transfer-safety invariants.
    pub fn validate(&self) -> Vec<TransferSafetyViolation> {
        let mut violations = Vec::new();

        if self.record_kind != TRANSFER_SAFETY_RECORD_KIND {
            violations.push(TransferSafetyViolation::WrongRecordKind);
        }
        if self.schema_version != TRANSFER_SAFETY_SCHEMA_VERSION {
            violations.push(TransferSafetyViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(TransferSafetyViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_records(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(TransferSafetyViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(TransferSafetyViolation::ConsumerProjectionIncomplete);
        }
        if !self.verification_freshness.is_valid() {
            violations.push(TransferSafetyViolation::VerificationFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("transfer safety packet serializes"),
        ) {
            violations.push(TransferSafetyViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("transfer safety packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Drag/Drop Transfer Safety: Verb Disclosure, Insertion Indicators, Cross-Window Detach, and Long-Transfer Progress\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Records: {} ({} inline commit, {} forced off inline, {} cross-window, {} long-transfer, {} provider/imported)\n",
            self.records.len(),
            self.inline_commit_record_count(),
            self.forced_record_count(),
            self.cross_window_record_count(),
            self.long_transfer_record_count(),
            self.provider_or_imported_record_count()
        ));
        out.push_str(&format!(
            "- Surface kinds: {} / {}\n",
            self.represented_surface_kinds().len(),
            KeyboardSurfaceKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Object classes: {} / {}\n",
            self.represented_object_classes().len(),
            TransferObjectClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Resolution classes: {} / {}\n",
            self.represented_resolutions().len(),
            TransferDisclosureClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Verification freshness SLO: {} hours (last refresh: {})\n",
            self.verification_freshness.verification_freshness_slo_hours,
            self.verification_freshness.last_verification_refresh
        ));
        out.push_str("\n## Records\n\n");
        for record in &self.records {
            out.push_str(&format!(
                "- **{}** ({}): resolution `{}`\n",
                record.record_id,
                record.surface_kind.as_str(),
                record.resolution.as_str()
            ));
            out.push_str(&format!("  - {}\n", record.label_summary));
            out.push_str(&format!(
                "  - object `{}` ({}), verb `{}`, scope `{}`, magnitude `{}`\n",
                record.object.object_token,
                record.object.object_class.as_str(),
                record.drop_verb.as_str(),
                record.window_scope.as_str(),
                record.transfer_magnitude.as_str()
            ));
            out.push_str(&format!(
                "  - verb disclosed={}, insertion disclosed={}\n",
                record.verb_disclosed_before_commit, record.insertion_point_disclosed_before_commit
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
            if let Some(label) = &record.verb_choice_label {
                out.push_str(&format!("  - Verb-choice: {label}\n"));
            }
            if let Some(note) = &record.continuity_note {
                out.push_str(&format!("  - Cross-window continuity: {note}\n"));
            }
            if let Some(note) = &record.progress_note {
                out.push_str(&format!("  - Progress/cancel/summary: {note}\n"));
            }
            if let Some(label) = &record.confirmation_label {
                out.push_str(&format!("  - Confirmed before mutation: {label}\n"));
            }
            if let Some(label) = &record.rejection_reason_label {
                out.push_str(&format!("  - Rejected: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum TransferSafetyArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TransferSafetyViolation>),
}

impl fmt::Display for TransferSafetyArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "transfer safety export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "transfer safety export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for TransferSafetyArtifactError {}

/// Validation failures emitted by [`TransferSafetyPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TransferSafetyViolation {
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
    /// A required transfer object class is represented by no record.
    RequiredObjectClassMissing,
    /// The required transfer-disclosure classes are not all represented.
    ResolutionCoverageMissing,
    /// No record demonstrates a drop held off the inline-commit lane.
    ForcedRecordCaseMissing,
    /// No clean inline-commit baseline record is present.
    InlineBaselineMissing,
    /// No cross-window continuity record is present.
    CrossWindowCaseMissing,
    /// No long-transfer progress record is present.
    LongTransferCaseMissing,
    /// No provider-linked / imported record is present.
    ProviderOrImportedCaseMissing,
    /// A record is incomplete.
    RecordIncomplete,
    /// A non-trivial drop was allowed to commit silently on the inline lane.
    SilentCommitOfUnsafeTransfer,
    /// A record's resolution ranks below its required safety floor.
    ResolutionBelowRequiredFloor,
    /// A record's recorded triggers do not match the computed set.
    TriggerSetInconsistent,
    /// A record's resolution detail field is missing, generic, or unexpected.
    ResolutionDetailInconsistent,
    /// A record did not disclose its resulting verb before commit.
    VerbDisclosureMissing,
    /// A committing record did not disclose its insertion point before commit.
    InsertionDisclosureMissing,
    /// A record dropped its dragged object.
    TransferObjectMissing,
    /// A provider/imported record reads as a locally verified transfer.
    ImportedReadsAsLocal,
    /// A record's verification proof is not reopenable.
    VerificationProofNotReopenable,
    /// A record lacks evidence refs.
    RecordEvidenceMissing,
    /// A record's subject fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A record flags raw boundary material / destructive side effect present.
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

impl TransferSafetyViolation {
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
            Self::InlineBaselineMissing => "inline_baseline_missing",
            Self::CrossWindowCaseMissing => "cross_window_case_missing",
            Self::LongTransferCaseMissing => "long_transfer_case_missing",
            Self::ProviderOrImportedCaseMissing => "provider_or_imported_case_missing",
            Self::RecordIncomplete => "record_incomplete",
            Self::SilentCommitOfUnsafeTransfer => "silent_commit_of_unsafe_transfer",
            Self::ResolutionBelowRequiredFloor => "resolution_below_required_floor",
            Self::TriggerSetInconsistent => "trigger_set_inconsistent",
            Self::ResolutionDetailInconsistent => "resolution_detail_inconsistent",
            Self::VerbDisclosureMissing => "verb_disclosure_missing",
            Self::InsertionDisclosureMissing => "insertion_disclosure_missing",
            Self::TransferObjectMissing => "transfer_object_missing",
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
pub fn current_transfer_safety_export() -> Result<TransferSafetyPacket, TransferSafetyArtifactError>
{
    let packet: TransferSafetyPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/interaction/m5/add-drag-and-drop-verb-disclosure-insertion-indicators-cross-window-detach-and-long-transf/support_export.json"
    )))
    .map_err(TransferSafetyArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TransferSafetyArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &TransferSafetyPacket,
    violations: &mut Vec<TransferSafetyViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        TRANSFER_SAFETY_SCHEMA_REF,
        TRANSFER_SAFETY_DOC_REF,
        TRANSFER_SAFETY_ARTIFACT_REF,
        KEYBOARD_CONTINUITY_MATRIX_DOC_REF,
        DRAG_DROP_TRANSFER_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(TransferSafetyViolation::MissingSourceContracts);
            break;
        }
    }
}

/// Surface kinds that must appear so the packet proves transfer-safety parity
/// across the new M5 drag/drop surfaces, plus the editor-core baseline.
const REQUIRED_SURFACE_KINDS: [KeyboardSurfaceKind; 6] = [
    KeyboardSurfaceKind::EditorCore,
    KeyboardSurfaceKind::NotebookSurface,
    KeyboardSurfaceKind::DataApiSurface,
    KeyboardSurfaceKind::PreviewSurface,
    KeyboardSurfaceKind::ReviewSurface,
    KeyboardSurfaceKind::RuntimeSurface,
];

/// Object classes whose transfer parity this packet must demonstrate.
const REQUIRED_OBJECT_CLASSES: [TransferObjectClass; 5] = [
    TransferObjectClass::NotebookCell,
    TransferObjectClass::ResultGridRow,
    TransferObjectClass::ArtifactItem,
    TransferObjectClass::WorkItem,
    TransferObjectClass::PreviewRuntimeAsset,
];

fn validate_coverage(packet: &TransferSafetyPacket, violations: &mut Vec<TransferSafetyViolation>) {
    let surface_kinds = packet.represented_surface_kinds();
    for required in REQUIRED_SURFACE_KINDS {
        if !surface_kinds.contains(&required) {
            violations.push(TransferSafetyViolation::RequiredSurfaceKindMissing);
            break;
        }
    }

    let object_classes = packet.represented_object_classes();
    for required in REQUIRED_OBJECT_CLASSES {
        if !object_classes.contains(&required) {
            violations.push(TransferSafetyViolation::RequiredObjectClassMissing);
            break;
        }
    }

    let resolutions = packet.represented_resolutions();
    for required in TransferDisclosureClass::ALL {
        if !resolutions.contains(&required) {
            violations.push(TransferSafetyViolation::ResolutionCoverageMissing);
            break;
        }
    }

    if !packet
        .records
        .iter()
        .any(|record| record.must_not_commit_silently() && record.is_complete())
    {
        violations.push(TransferSafetyViolation::ForcedRecordCaseMissing);
    }

    if !packet.records.iter().any(|record| {
        record.resolution.is_inline_commit()
            && !record.must_not_commit_silently()
            && record.is_complete()
    }) {
        violations.push(TransferSafetyViolation::InlineBaselineMissing);
    }

    if !packet.records.iter().any(|record| {
        record.resolution == TransferDisclosureClass::CrossWindowContinuityPreserved
            && record.window_scope.is_cross_window()
            && record.is_complete()
    }) {
        violations.push(TransferSafetyViolation::CrossWindowCaseMissing);
    }

    if !packet.records.iter().any(|record| {
        record.resolution == TransferDisclosureClass::ProgressCancelSummaryTracked
            && record.transfer_magnitude.needs_progress()
            && record.is_complete()
    }) {
        violations.push(TransferSafetyViolation::LongTransferCaseMissing);
    }

    if packet.provider_or_imported_record_count() == 0 {
        violations.push(TransferSafetyViolation::ProviderOrImportedCaseMissing);
    }
}

fn validate_records(packet: &TransferSafetyPacket, violations: &mut Vec<TransferSafetyViolation>) {
    for record in &packet.records {
        if !record.is_complete() {
            violations.push(TransferSafetyViolation::RecordIncomplete);
        }
        if record.silently_commits_unsafe() {
            violations.push(TransferSafetyViolation::SilentCommitOfUnsafeTransfer);
        }
        if !record.resolution_meets_floor() {
            violations.push(TransferSafetyViolation::ResolutionBelowRequiredFloor);
        }
        if !record.triggers_consistent() {
            violations.push(TransferSafetyViolation::TriggerSetInconsistent);
        }
        if !record.resolution_detail_consistent() {
            violations.push(TransferSafetyViolation::ResolutionDetailInconsistent);
        }
        if !record.verb_disclosure_holds() {
            violations.push(TransferSafetyViolation::VerbDisclosureMissing);
        }
        if !record.insertion_disclosure_holds() {
            violations.push(TransferSafetyViolation::InsertionDisclosureMissing);
        }
        if !record.object.is_valid() {
            violations.push(TransferSafetyViolation::TransferObjectMissing);
        }
        if !record.imported_posture_consistent() {
            violations.push(TransferSafetyViolation::ImportedReadsAsLocal);
        }
        if !record.verification.is_well_formed() {
            violations.push(TransferSafetyViolation::VerificationProofNotReopenable);
        }
        if record.evidence_refs.is_empty()
            || record.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(TransferSafetyViolation::RecordEvidenceMissing);
        }
        if !record.subject.fingerprint_independent_of_id() {
            violations.push(TransferSafetyViolation::FingerprintSubstitutesIdentity);
        }
        if !record.no_raw_boundary_material() {
            violations.push(TransferSafetyViolation::RawBoundaryMaterialPresent);
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
            | "provider error"
            | "request failed"
            | "failed"
            | "moved"
            | "copied"
            | "imported"
            | "rejected"
            | "unverified"
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

/// Stable packet id minted by [`seeded_transfer_safety_packet`].
pub const SEED_TRANSFER_SAFETY_PACKET_ID: &str = "m5-transfer-safety:stable:0001";

/// Mint timestamp used by [`seeded_transfer_safety_packet`].
pub const SEED_TRANSFER_SAFETY_MINTED_AT: &str = "2026-06-14T00:00:00Z";

/// Builds the canonical, validating transfer-safety packet that the checked-in
/// support export, the Markdown summary, and the conformance tests all share, so
/// the in-crate builder stays byte-aligned with the artifact.
///
/// The seed anchors clean inline-commit baselines (editor fragment and notebook
/// cell reorder), then exercises each non-default resolution on a distinct M5
/// surface: a data/API result-row drop whose verb is chosen on a modifier
/// disclosed explicitly, a review work-item detached across windows with
/// continuity preserved, a preview-runtime asset large drop tracked with
/// progress / cancel / summary, an artifact imported into a notebook confirmed
/// before mutation, a runtime asset whose destructive default verb is rejected,
/// and a provider-linked companion artifact detached across windows whose
/// imported proof never reads as a local transfer.
pub fn seeded_transfer_safety_packet() -> TransferSafetyPacket {
    TransferSafetyPacket::new(TransferSafetyPacketInput {
        packet_id: SEED_TRANSFER_SAFETY_PACKET_ID.to_owned(),
        label: "M5 Drag/Drop Transfer Safety: Verb Disclosure, Insertion Indicators, Cross-Window Detach, and Long-Transfer Progress"
            .to_owned(),
        records: seeded_records(),
        guardrails: TransferSafetyGuardrails {
            verb_disclosed_before_commit: true,
            insertion_point_disclosed_before_commit: true,
            never_destructive_or_ambiguous_by_default: true,
            cross_window_continuity_preserved: true,
            long_transfer_progress_cancel_summary: true,
            provider_transfers_never_read_as_local: true,
            no_new_macro_language_introduced: true,
        },
        consumer_projection: TransferSafetyConsumerProjection {
            product_ingests_packet: true,
            help_migration_ingests_packet: true,
            support_export_ingests_packet: true,
            release_control_ingests_packet: true,
            transfer_verbs_and_disclosures_nameable: true,
        },
        verification_freshness: TransferSafetyFreshness {
            verification_freshness_slo_hours: 168,
            last_verification_refresh: SEED_TRANSFER_SAFETY_MINTED_AT.to_owned(),
            auto_escalate_on_stale: true,
        },
        source_contract_refs: vec![
            TRANSFER_SAFETY_SCHEMA_REF.to_owned(),
            TRANSFER_SAFETY_DOC_REF.to_owned(),
            TRANSFER_SAFETY_ARTIFACT_REF.to_owned(),
            KEYBOARD_CONTINUITY_MATRIX_DOC_REF.to_owned(),
            DRAG_DROP_TRANSFER_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TRANSFER_SAFETY_MINTED_AT.to_owned(),
    })
}

fn seeded_records() -> Vec<TransferSafetyRecord> {
    vec![
        editor_core_inline_record(),
        notebook_inline_record(),
        data_api_verb_choice_record(),
        review_cross_window_record(),
        preview_long_transfer_record(),
        notebook_import_confirm_record(),
        runtime_destructive_reject_record(),
        companion_provider_record(),
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

fn object(
    object_class: TransferObjectClass,
    object_token: &str,
    display_label: &str,
) -> TransferObjectRef {
    TransferObjectRef {
        object_class,
        object_token: object_token.to_owned(),
        display_label: display_label.to_owned(),
    }
}

fn editor_core_inline_record() -> TransferSafetyRecord {
    let record_id = "transfer:editor-core:0001";
    TransferSafetyRecord::new(TransferSafetyRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::EditorCore,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Editor-core text fragment reordered inline with verb and insertion shown"
            .to_owned(),
        object: object(
            TransferObjectClass::EditorTextFragment,
            "fragment:src/lib.rs#L10-L18",
            "Editor text fragment in src/lib.rs",
        ),
        drop_verb: DragDropVerbClass::MoveVerbExplicit,
        window_scope: WindowScopeClass::SamePaneReorder,
        transfer_magnitude: TransferMagnitudeClass::TrivialInline,
        import_or_destructive: false,
        verb_disclosed_before_commit: true,
        insertion_point_disclosed_before_commit: true,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Editor-core reorder verified to show the move verb and insertion caret before commit",
        ),
        resolution: TransferDisclosureClass::DisclosedInlineCommit,
        fired_triggers: vec![],
        verb_choice_label: None,
        continuity_note: None,
        progress_note: None,
        confirmation_label: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_TRANSFER_SAFETY_MINTED_AT.to_owned(),
    })
}

fn notebook_inline_record() -> TransferSafetyRecord {
    let record_id = "transfer:notebook:0001";
    TransferSafetyRecord::new(TransferSafetyRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::NotebookSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary: "Notebook cell reordered inline with the move verb and insertion line shown"
            .to_owned(),
        object: object(
            TransferObjectClass::NotebookCell,
            "cell:notebook/analysis#cell-3",
            "Notebook cell 3",
        ),
        drop_verb: DragDropVerbClass::MoveVerbExplicit,
        window_scope: WindowScopeClass::SamePaneReorder,
        transfer_magnitude: TransferMagnitudeClass::TrivialInline,
        import_or_destructive: false,
        verb_disclosed_before_commit: true,
        insertion_point_disclosed_before_commit: true,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Notebook cell reorder verified to show the move verb and insertion line before commit",
        ),
        resolution: TransferDisclosureClass::DisclosedInlineCommit,
        fired_triggers: vec![],
        verb_choice_label: None,
        continuity_note: None,
        progress_note: None,
        confirmation_label: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_TRANSFER_SAFETY_MINTED_AT.to_owned(),
    })
}

fn data_api_verb_choice_record() -> TransferSafetyRecord {
    let record_id = "transfer:data-api:0001";
    TransferSafetyRecord::new(TransferSafetyRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::DataApiSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Data/API result-row drop whose move-or-copy verb is chosen on a modifier and disclosed"
                .to_owned(),
        object: object(
            TransferObjectClass::ResultGridRow,
            "row:data/api/users#L42",
            "Data/API result row 42",
        ),
        drop_verb: DragDropVerbClass::VerbChoiceOnModifier,
        window_scope: WindowScopeClass::CrossPaneSameWindow,
        transfer_magnitude: TransferMagnitudeClass::TrivialInline,
        import_or_destructive: false,
        verb_disclosed_before_commit: true,
        insertion_point_disclosed_before_commit: true,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Data/API row drop verified to advertise the chosen move/copy verb before commit",
        ),
        resolution: TransferDisclosureClass::ExplicitVerbChoiceDisclosed,
        fired_triggers: vec![TransferContractTrigger::AmbiguousOrMultiVerb],
        verb_choice_label: Some(
            "Hold the modifier to copy the row; release to move it — the chosen verb is shown before drop"
                .to_owned(),
        ),
        continuity_note: None,
        progress_note: None,
        confirmation_label: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_TRANSFER_SAFETY_MINTED_AT.to_owned(),
    })
}

fn review_cross_window_record() -> TransferSafetyRecord {
    let record_id = "transfer:review:0001";
    TransferSafetyRecord::new(TransferSafetyRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::ReviewSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Review work item detached into another window with context and recovery preserved"
                .to_owned(),
        object: object(
            TransferObjectClass::WorkItem,
            "work-item:review/pr-204#item-7",
            "Review work item 7",
        ),
        drop_verb: DragDropVerbClass::MoveVerbExplicit,
        window_scope: WindowScopeClass::CrossWindowDetach,
        transfer_magnitude: TransferMagnitudeClass::TrivialInline,
        import_or_destructive: false,
        verb_disclosed_before_commit: true,
        insertion_point_disclosed_before_commit: true,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Review work-item detach verified to preserve context and a reattach recovery path",
        ),
        resolution: TransferDisclosureClass::CrossWindowContinuityPreserved,
        fired_triggers: vec![TransferContractTrigger::CrossWindowDetach],
        verb_choice_label: None,
        continuity_note: Some(
            "Detaching opens a new window that keeps the work item's context and offers a reattach to recover its prior pane"
                .to_owned(),
        ),
        progress_note: None,
        confirmation_label: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_TRANSFER_SAFETY_MINTED_AT.to_owned(),
    })
}

fn preview_long_transfer_record() -> TransferSafetyRecord {
    let record_id = "transfer:preview:0001";
    TransferSafetyRecord::new(TransferSafetyRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::PreviewSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Large preview-runtime asset dropped with progress, a cancel control, and a post-action summary"
                .to_owned(),
        object: object(
            TransferObjectClass::PreviewRuntimeAsset,
            "asset:preview/runtime/bundle-9921",
            "Preview-runtime asset bundle",
        ),
        drop_verb: DragDropVerbClass::CopyVerbExplicit,
        window_scope: WindowScopeClass::CrossPaneSameWindow,
        transfer_magnitude: TransferMagnitudeClass::LargeNeedsProgress,
        import_or_destructive: false,
        verb_disclosed_before_commit: true,
        insertion_point_disclosed_before_commit: true,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Large preview asset drop verified to show progress, a cancel control, and a post-action summary",
        ),
        resolution: TransferDisclosureClass::ProgressCancelSummaryTracked,
        fired_triggers: vec![TransferContractTrigger::LargeTransferMagnitude],
        verb_choice_label: None,
        continuity_note: None,
        progress_note: Some(
            "The drop streams with a progress bar and a cancel control, then reports a post-action summary of what landed"
                .to_owned(),
        ),
        confirmation_label: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_TRANSFER_SAFETY_MINTED_AT.to_owned(),
    })
}

fn notebook_import_confirm_record() -> TransferSafetyRecord {
    let record_id = "transfer:notebook:import:0001";
    TransferSafetyRecord::new(TransferSafetyRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::NotebookSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Artifact imported into a notebook confirmed for verb and scope before the destination mutates"
                .to_owned(),
        object: object(
            TransferObjectClass::ArtifactItem,
            "artifact:evidence/run-7741",
            "Artifact evidence item",
        ),
        drop_verb: DragDropVerbClass::CopyVerbExplicit,
        window_scope: WindowScopeClass::CrossPaneSameWindow,
        transfer_magnitude: TransferMagnitudeClass::TrivialInline,
        import_or_destructive: true,
        verb_disclosed_before_commit: true,
        insertion_point_disclosed_before_commit: true,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Artifact import verified to confirm the import verb and target scope before mutating the notebook",
        ),
        resolution: TransferDisclosureClass::ConfirmedBeforeMutation,
        fired_triggers: vec![TransferContractTrigger::DestructiveOrImportSemantics],
        verb_choice_label: None,
        continuity_note: None,
        progress_note: None,
        confirmation_label: Some(
            "Importing inserts a new cell that references the artifact; confirm the verb and target cell before it lands"
                .to_owned(),
        ),
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_TRANSFER_SAFETY_MINTED_AT.to_owned(),
    })
}

fn runtime_destructive_reject_record() -> TransferSafetyRecord {
    let record_id = "transfer:runtime:reject:0001";
    TransferSafetyRecord::new(TransferSafetyRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::RuntimeSurface,
        subject: subject_for(record_id, SurfaceOriginClass::EmbeddedRuntimeSurface),
        label_summary:
            "Runtime asset drop with a destructive default verb is rejected with the verb disclosed"
                .to_owned(),
        object: object(
            TransferObjectClass::PreviewRuntimeAsset,
            "asset:runtime/live-handle-13",
            "Embedded runtime live asset",
        ),
        drop_verb: DragDropVerbClass::DestructiveDefaultDenied,
        window_scope: WindowScopeClass::CrossPaneSameWindow,
        transfer_magnitude: TransferMagnitudeClass::TrivialInline,
        import_or_destructive: false,
        verb_disclosed_before_commit: true,
        insertion_point_disclosed_before_commit: false,
        verification: proof_for(
            record_id,
            AxisProofCurrency::VerifiedCurrent,
            "Runtime asset drop verified to reject a destructive default verb rather than committing it",
        ),
        resolution: TransferDisclosureClass::RejectedAmbiguousOrUnsafe,
        fired_triggers: vec![TransferContractTrigger::DestructiveDefaultVerb],
        verb_choice_label: None,
        continuity_note: None,
        progress_note: None,
        confirmation_label: None,
        rejection_reason_label: Some(
            "Dropping onto a live runtime asset would overwrite it by default, so the drop is rejected rather than committed"
                .to_owned(),
        ),
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_TRANSFER_SAFETY_MINTED_AT.to_owned(),
    })
}

fn companion_provider_record() -> TransferSafetyRecord {
    let record_id = "transfer:companion:0001";
    TransferSafetyRecord::new(TransferSafetyRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::CompanionSurface,
        subject: subject_for(record_id, SurfaceOriginClass::ProviderLinkedSurface),
        label_summary:
            "Provider-linked companion artifact detached across windows; imported proof never reads as local"
                .to_owned(),
        object: object(
            TransferObjectClass::ArtifactItem,
            "artifact:companion/thread-204#attachment-7",
            "Provider-backed companion artifact",
        ),
        drop_verb: DragDropVerbClass::LinkVerbExplicit,
        window_scope: WindowScopeClass::CrossWindowDetach,
        transfer_magnitude: TransferMagnitudeClass::TrivialInline,
        import_or_destructive: false,
        verb_disclosed_before_commit: true,
        insertion_point_disclosed_before_commit: true,
        verification: proof_for(
            record_id,
            AxisProofCurrency::ImportedCurrent,
            "Provider-backed companion detach verified with imported proof, never a local transfer",
        ),
        resolution: TransferDisclosureClass::CrossWindowContinuityPreserved,
        fired_triggers: vec![TransferContractTrigger::CrossWindowDetach],
        verb_choice_label: None,
        continuity_note: Some(
            "Detaching opens a provider-linked window that keeps the artifact's thread context and a reattach recovery path"
                .to_owned(),
        ),
        progress_note: None,
        confirmation_label: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_TRANSFER_SAFETY_MINTED_AT.to_owned(),
    })
}

/// Packet id minted by [`fixture_transfer_safety_packet`].
pub const FIXTURE_TRANSFER_SAFETY_PACKET_ID: &str =
    "m5-transfer-safety:fixture:stale-proof-forces-disclosure:0001";

/// Builds the protected fixture variant: it keeps the full seeded record set —
/// including the clean inline-commit baselines — and adds one drill record for a
/// notebook cell reorder that would otherwise commit inline but is forced off the
/// inline lane because its transfer proof aged outside the freshness window.
///
/// The fixture is a *valid* packet: the drill record correctly records the
/// [`TransferContractTrigger::StaleOrMissingTransferProof`] trigger and resolves
/// to [`TransferDisclosureClass::ExplicitVerbChoiceDisclosed`] with a precise
/// verb-choice label, so it validates while demonstrating that stale evidence —
/// not just a multi-verb, cross-window, large, or import drop — forces a transfer
/// off the inline-commit lane.
pub fn fixture_transfer_safety_packet() -> TransferSafetyPacket {
    let mut packet = seeded_transfer_safety_packet();
    packet.packet_id = FIXTURE_TRANSFER_SAFETY_PACKET_ID.to_owned();
    packet.label =
        "M5 Drag/Drop Transfer Safety fixture: stale transfer proof forces an inline commit into an explicit disclosure"
            .to_owned();
    packet.records.push(stale_proof_drill_record());
    packet
}

/// A notebook cell reorder that would commit inline, but whose transfer proof has
/// aged outside its freshness window, so it is forced into an explicit
/// verb-choice disclosure.
fn stale_proof_drill_record() -> TransferSafetyRecord {
    let record_id = "transfer:notebook:stale-proof:0001";
    TransferSafetyRecord::new(TransferSafetyRecordInput {
        record_id: record_id.to_owned(),
        surface_kind: KeyboardSurfaceKind::NotebookSurface,
        subject: subject_for(record_id, SurfaceOriginClass::FirstPartySurface),
        label_summary:
            "Notebook cell reorder whose stale transfer proof forces it into an explicit disclosure"
                .to_owned(),
        object: object(
            TransferObjectClass::NotebookCell,
            "cell:notebook/analysis#cell-9",
            "Notebook cell 9 with stale proof",
        ),
        drop_verb: DragDropVerbClass::MoveVerbExplicit,
        window_scope: WindowScopeClass::SamePaneReorder,
        transfer_magnitude: TransferMagnitudeClass::TrivialInline,
        import_or_destructive: false,
        verb_disclosed_before_commit: true,
        insertion_point_disclosed_before_commit: true,
        verification: proof_for(
            record_id,
            AxisProofCurrency::StaleExpired,
            "Notebook cell reorder proof aged outside its freshness window",
        ),
        resolution: TransferDisclosureClass::ExplicitVerbChoiceDisclosed,
        fired_triggers: vec![TransferContractTrigger::StaleOrMissingTransferProof],
        verb_choice_label: Some(
            "Transfer proof aged outside its freshness window; the move is offered as an explicit, re-verified disclosure"
                .to_owned(),
        ),
        continuity_note: None,
        progress_note: None,
        confirmation_label: None,
        rejection_reason_label: None,
        evidence_refs: vec![format!("evidence:record:{record_id}")],
        minted_at: SEED_TRANSFER_SAFETY_MINTED_AT.to_owned(),
    })
}
