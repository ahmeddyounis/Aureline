//! Voice → canonical-command bridge: transcript strips, candidate
//! disambiguation, and grouped-undo / audit parity with keyboard invocation.
//!
//! Voice is an explicit, privacy-bounded input mode, but a spoken phrase is only
//! ever *recognized* — it must still reach the command graph the keyboard and
//! command palette use. This module is the canonical bridge that turns a
//! recognized utterance into one of four bounded outcomes and proves, as an
//! inspectable truth packet, that none of them widens authority or commits
//! through a side path:
//!
//! - a [`VoiceCommandBridgeRow`] whose [`VoiceIntentResolutionClass`] resolves to
//!   exactly one canonical command, ambiguates into a candidate list, becomes
//!   dictation text, or is denied because the verb is not on the stable command
//!   graph;
//! - a [`TranscriptStripState`] that shows what Aureline heard — and whether a
//!   correction is available — *before* a surprising or high-impact command
//!   commits;
//! - a [`VoiceBridgeCandidate`] list that exposes the **same** stable command id,
//!   description (`primary_label_ref`), keyboard shortcut narration, and disabled
//!   reason the command palette projects, drawn from the same
//!   [`crate::descriptor::CommandDescriptorRecord`] vocabulary; and
//! - a [`GroupedUndoLineage`] that ties a committed voice invocation to the same
//!   grouped-undo group, shared history, and
//!   [`crate::authority::InvocationLineageRecord`] a keyboard invocation of the
//!   same command id would produce — never a parallel command path.
//!
//! [`VoiceCommandBridgePacket::validate`] refuses any row that lets an ambiguous
//! utterance execute silently, lets a high-impact command commit without a
//! confirmation gate and an available transcript correction, drops the palette
//! parity fields (id / description / shortcut / disabled reason) from a
//! candidate, invents a command that is not on the supplied stable command
//! graph, or commits without joining grouped undo and audit lineage. The packet
//! carries only typed class tokens, opaque ids, and redaction-aware label refs —
//! never raw audio bytes, raw transcript text, or raw provider payloads.
//!
//! The seed in [`seeded_voice_command_bridge_packet`] is the single
//! mint-from-truth source for the checked-in fixtures under
//! [`VOICE_COMMAND_BRIDGE_FIXTURES_DIR_REF`] and the published companion doc
//! [`VOICE_COMMAND_BRIDGE_DOC_REF`]; shell, diagnostics, and support-export
//! surfaces should ingest this packet rather than re-deriving voice command
//! parity by hand.

use std::collections::BTreeSet;
use std::path::Path;
use std::{fs, io};

use serde::{Deserialize, Serialize};

use crate::authority::InvocationLineageRecord;
use crate::descriptor::{CommandId, CommandRevisionRef, OpaqueId, ShortcutNarrationHint};
use crate::enablement::{DisabledReasonCode, EnablementDecisionClass};
use crate::invocation::NoBypassGuards;

/// Schema version exported with every voice-command-bridge record.
pub const VOICE_COMMAND_BRIDGE_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref quoted by every voice-command-bridge record.
pub const VOICE_COMMAND_BRIDGE_SHARED_CONTRACT_REF: &str = "commands:voice_command_bridge:v1";

/// Stable record kind for [`VoiceCommandBridgePacket`] payloads.
pub const VOICE_COMMAND_BRIDGE_PACKET_RECORD_KIND: &str =
    "commands_voice_command_bridge_packet_record";

/// Stable record kind for [`VoiceCommandBridgeRow`] payloads.
pub const VOICE_COMMAND_BRIDGE_ROW_RECORD_KIND: &str = "commands_voice_command_bridge_row_record";

/// Stable packet id quoted across surfaces.
pub const VOICE_COMMAND_BRIDGE_PACKET_ID: &str = "commands:voice_command_bridge:packet:v1";

/// Repo-relative path of the published companion doc.
pub const VOICE_COMMAND_BRIDGE_DOC_REF: &str = "docs/ux/voice-disambiguation-and-confirmation.md";

/// Repo-relative directory of the checked-in mint-from-truth fixtures.
pub const VOICE_COMMAND_BRIDGE_FIXTURES_DIR_REF: &str = "fixtures/voice/disambiguation";

/// Frozen command-descriptor contract whose id / label / shortcut / disabled
/// reason vocabulary the candidate list reuses verbatim.
pub const COMMAND_DESCRIPTOR_CONTRACT_REF: &str = "docs/commands/command_descriptor_contract.md";

/// Canonical command result-packet schema the committed lineage reuses.
pub const COMMAND_RESULT_PACKET_SCHEMA_REF: &str =
    "schemas/commands/command_result_packet.schema.json";

/// Cross-surface voice / dictation / speech-privacy contract this bridge rides.
pub const VOICE_AND_DICTATION_CONTRACT_REF: &str = "docs/ux/voice_and_dictation_contract.md";

/// Redaction class stamped on every record; the packet carries metadata only.
pub const REDACTION_CLASS: &str = "metadata_safe_default";

/// Recognition-confidence cue shown next to a transcript segment or candidate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceCue {
    /// High recognition confidence.
    High,
    /// Medium recognition confidence.
    Medium,
    /// Low recognition confidence; a correction is strongly suggested.
    Low,
}

impl ConfidenceCue {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
        }
    }
}

/// Capability-impact class of a candidate command. Mirrors the descriptor
/// `capability_scope_class` vocabulary so the bridge recognizes which commands
/// are surprising or high-impact and therefore must not commit silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandImpactClass {
    /// Inert metadata route (no state change).
    InertMetadataOnly,
    /// Reversible local read.
    ReversibleLocalRead,
    /// Reversible local mutation (undoable without a rollback handle).
    ReversibleLocalMutation,
    /// Recoverable durable mutation (requires a rollback handle).
    RecoverableDurableMutation,
    /// Destructive bulk mutation (multi-file, multi-record).
    DestructiveBulkMutation,
    /// Irreversible publish / network mutation.
    IrreversiblePublish,
}

impl CommandImpactClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InertMetadataOnly => "inert_metadata_only",
            Self::ReversibleLocalRead => "reversible_local_read",
            Self::ReversibleLocalMutation => "reversible_local_mutation",
            Self::RecoverableDurableMutation => "recoverable_durable_mutation",
            Self::DestructiveBulkMutation => "destructive_bulk_mutation",
            Self::IrreversiblePublish => "irreversible_publish",
        }
    }

    /// `true` for high-impact / surprising scopes that MUST route through a
    /// confirmation gate with an available transcript correction before commit.
    pub const fn is_high_impact(self) -> bool {
        matches!(
            self,
            Self::RecoverableDurableMutation
                | Self::DestructiveBulkMutation
                | Self::IrreversiblePublish
        )
    }
}

/// How a recognized spoken phrase resolves against the stable command graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VoiceIntentResolutionClass {
    /// Resolves to exactly one canonical command id.
    ResolvesToSingleCommand,
    /// Ambiguous: a candidate / disambiguation sheet is required before commit.
    AmbiguousRequiresDisambiguation,
    /// Resolves to dictation text routed through the shared edit model.
    ResolvesToDictationText,
    /// Denied: the verb is not on the stable command graph (never invented).
    DeniedNoCanonicalCommand,
}

impl VoiceIntentResolutionClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvesToSingleCommand => "resolves_to_single_command",
            Self::AmbiguousRequiresDisambiguation => "ambiguous_requires_disambiguation",
            Self::ResolvesToDictationText => "resolves_to_dictation_text",
            Self::DeniedNoCanonicalCommand => "denied_no_canonical_command",
        }
    }

    /// `true` when the class is expected to commit a selected canonical command
    /// or dictation edit.
    pub const fn binds_command(self) -> bool {
        matches!(
            self,
            Self::ResolvesToSingleCommand | Self::ResolvesToDictationText
        )
    }

    /// `true` when the class must route through a candidate list.
    pub const fn is_ambiguous(self) -> bool {
        matches!(self, Self::AmbiguousRequiresDisambiguation)
    }

    /// `true` when the class denies the utterance.
    pub const fn is_denied(self) -> bool {
        matches!(self, Self::DeniedNoCanonicalCommand)
    }
}

/// The gate a recognized phrase passes through before anything commits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationGateClass {
    /// Reversible, low-impact action commits directly (still joins grouped undo).
    DirectCommitLowImpact,
    /// Surprising / high-impact action waits on explicit confirmation, with the
    /// transcript strip and a correction shown first.
    ConfirmationRequiredBeforeCommit,
    /// Ambiguous utterance waits on a candidate selection; nothing commits yet.
    DisambiguationRequiredBeforeCommit,
    /// Denied utterance: nothing commits; a keyboard-first fallback is offered.
    BlockedNoCanonicalCommand,
}

impl ConfirmationGateClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectCommitLowImpact => "direct_commit_low_impact",
            Self::ConfirmationRequiredBeforeCommit => "confirmation_required_before_commit",
            Self::DisambiguationRequiredBeforeCommit => "disambiguation_required_before_commit",
            Self::BlockedNoCanonicalCommand => "blocked_no_canonical_command",
        }
    }

    /// `true` for gates that commit a command once cleared.
    pub const fn commits(self) -> bool {
        matches!(
            self,
            Self::DirectCommitLowImpact | Self::ConfirmationRequiredBeforeCommit
        )
    }
}

/// Whether a transcript correction is available before commit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptCorrectionAvailability {
    /// A correction window is required before the action can commit.
    RequiredBeforeCommit,
    /// A correction window is offered before commit.
    OfferedBeforeCommit,
    /// No correction is available (nothing to commit toward).
    Unavailable,
}

impl TranscriptCorrectionAvailability {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequiredBeforeCommit => "required_before_commit",
            Self::OfferedBeforeCommit => "offered_before_commit",
            Self::Unavailable => "unavailable",
        }
    }

    /// `true` when a correction is reachable before commit.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::RequiredBeforeCommit | Self::OfferedBeforeCommit)
    }
}

/// Transcript strip: what Aureline heard, plus correct / confirm / cancel
/// actions that all route through the canonical command path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TranscriptStripState {
    /// Representation label for the heard text (never raw spoken bytes).
    pub heard_text_label_ref: String,
    /// Recognition-confidence cue for the heard segment.
    pub confidence_cue: ConfidenceCue,
    /// Whether a correction is available before commit.
    pub correction_availability: TranscriptCorrectionAvailability,
    /// `true` when the strip is shown to the user before anything commits.
    pub shown_before_commit: bool,
    /// Canonical command id for the edit action.
    pub edit_command_id: CommandId,
    /// Canonical command id for the correct action.
    pub correct_command_id: CommandId,
    /// Canonical command id for the confirm action.
    pub confirm_command_id: CommandId,
    /// Canonical command id for the cancel action.
    pub cancel_command_id: CommandId,
    /// Accessibility label ref narrated by the screen reader.
    pub accessibility_label_ref: String,
}

/// One candidate row on a disambiguation / confirmation sheet, carrying the
/// exact palette-parity fields a command-palette row would show.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceBridgeCandidate {
    /// Canonical command id this candidate would invoke.
    pub candidate_command_id: CommandId,
    /// Descriptor revision the candidate was projected against.
    pub command_revision_ref: CommandRevisionRef,
    /// Dotted canonical verb resolved from the descriptor.
    pub canonical_verb: String,
    /// Primary label ref (the same description the palette renders).
    pub primary_label_ref: String,
    /// Shortcut narration (the same chord cue the palette renders).
    pub shortcut: ShortcutNarrationHint,
    /// Enablement decision shared with keyboard / palette.
    pub enablement_decision_class: EnablementDecisionClass,
    /// Disabled reason code (shared vocabulary) when not enabled.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason_code: Option<DisabledReasonCode>,
    /// Capability-impact class of the candidate command.
    pub impact_class: CommandImpactClass,
    /// `true` when the candidate requires a preview before apply.
    pub preview_required: bool,
    /// `true` when the candidate requires approval before apply.
    pub approval_required: bool,
    /// Recognition-confidence cue for the candidate.
    pub confidence_cue: ConfidenceCue,
}

impl VoiceBridgeCandidate {
    /// `true` when the surface reports the candidate as enabled.
    pub fn is_enabled(&self) -> bool {
        self.enablement_decision_class == EnablementDecisionClass::Enabled
    }

    /// Returns the first missing palette-parity field, if any. A candidate must
    /// carry the same stable id, description, and shortcut narration the palette
    /// renders, so a voice sheet can never become a thinner side projection.
    pub fn missing_palette_field(&self) -> Option<&'static str> {
        if self.candidate_command_id.trim().is_empty() {
            return Some("candidate_command_id");
        }
        if self.primary_label_ref.trim().is_empty() {
            return Some("primary_label_ref");
        }
        if self.shortcut.when_bound_narration_ref.trim().is_empty()
            || self.shortcut.when_unbound_narration_ref.trim().is_empty()
        {
            return Some("shortcut_narration");
        }
        None
    }
}

/// Grouped-undo and audit lineage proving a committed voice command behaves like
/// its keyboard equivalent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroupedUndoLineage {
    /// Grouped-undo group this invocation joins (shared with keyboard/palette).
    pub undo_group_id: OpaqueId,
    /// `true` when the commit joins the shared history timeline, not a side log.
    pub joins_shared_undo_history: bool,
    /// History-entry ref recorded for the commit.
    pub history_entry_ref: OpaqueId,
    /// `true` when the commit dispatches through the canonical command session.
    pub commits_through_canonical_session: bool,
    /// The same command id a keyboard invocation would use.
    pub keyboard_equivalent_command_id: CommandId,
    /// Command → result → evidence → notification / activity → rollback lineage.
    pub lineage: InvocationLineageRecord,
    /// No-bypass guards asserted on the committed invocation.
    pub no_bypass_guards: NoBypassGuards,
}

/// One recognized spoken utterance routed through the bridge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceCommandBridgeRow {
    /// Record discriminator; equals [`VOICE_COMMAND_BRIDGE_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; equals [`VOICE_COMMAND_BRIDGE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Representation label for the spoken phrase (never raw bytes).
    pub spoken_phrase_label_ref: String,
    /// How the phrase resolved against the stable command graph.
    pub intent_class: VoiceIntentResolutionClass,
    /// The gate the phrase passes through before commit.
    pub confirmation_gate: ConfirmationGateClass,
    /// Transcript strip shown before commit.
    pub transcript_strip: TranscriptStripState,
    /// Candidate rows, in canonical order (≥2 for ambiguous intents).
    pub candidates: Vec<VoiceBridgeCandidate>,
    /// Selected canonical command id, once one is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_command_id: Option<CommandId>,
    /// Grouped-undo / audit lineage, present once the command commits.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouped_undo_lineage: Option<GroupedUndoLineage>,
    /// Canonical command id of the keyboard-first fallback (always present).
    pub keyboard_fallback_command_id: CommandId,
    /// Canonical docs/help anchor ref for the row.
    pub docs_help_anchor_ref: String,
    /// Redaction class.
    pub redaction_class: String,
}

impl VoiceCommandBridgeRow {
    /// Returns the candidate matching [`Self::selected_command_id`], if any.
    pub fn selected_candidate(&self) -> Option<&VoiceBridgeCandidate> {
        let selected = self.selected_command_id.as_deref()?;
        self.candidates
            .iter()
            .find(|candidate| candidate.candidate_command_id == selected)
    }

    /// Impact class of the selected command, if one is bound.
    pub fn selected_impact(&self) -> Option<CommandImpactClass> {
        self.selected_candidate()
            .map(|candidate| candidate.impact_class)
    }

    /// Collects every invariant this row violates against `canonical_ids` — the
    /// stable command graph the bridge is allowed to reach. An empty result
    /// means the row keeps voice command resolution inspectable,
    /// confirmation-gated, palette-faithful, and lineage-parity-clean.
    pub fn check(&self, canonical_ids: &BTreeSet<&str>) -> Vec<VoiceBridgeViolation> {
        let mut out = Vec::new();
        let id = || self.row_id.clone();

        // Every row keeps a keyboard-first fallback out of voice.
        if self.keyboard_fallback_command_id.trim().is_empty() {
            out.push(VoiceBridgeViolation::MissingKeyboardFallback { row_id: id() });
        } else if !canonical_ids.contains(self.keyboard_fallback_command_id.as_str()) {
            out.push(VoiceBridgeViolation::InventedNonCanonicalCommand {
                row_id: id(),
                command_id: self.keyboard_fallback_command_id.clone(),
            });
        }

        // A voice surface may never invent a command outside the stable graph.
        for candidate in &self.candidates {
            if !canonical_ids.contains(candidate.candidate_command_id.as_str()) {
                out.push(VoiceBridgeViolation::InventedNonCanonicalCommand {
                    row_id: id(),
                    command_id: candidate.candidate_command_id.clone(),
                });
            }
            if let Some(field) = candidate.missing_palette_field() {
                out.push(VoiceBridgeViolation::CandidateMissingPaletteParityField {
                    row_id: id(),
                    command_id: candidate.candidate_command_id.clone(),
                    field: field.to_owned(),
                });
            }
            if !candidate.is_enabled() && candidate.disabled_reason_code.is_none() {
                out.push(VoiceBridgeViolation::CandidateMissingDisabledReason {
                    row_id: id(),
                    command_id: candidate.candidate_command_id.clone(),
                });
            }
        }
        if let Some(selected) = self.selected_command_id.as_deref() {
            if !canonical_ids.contains(selected) {
                out.push(VoiceBridgeViolation::InventedNonCanonicalCommand {
                    row_id: id(),
                    command_id: selected.to_owned(),
                });
            }
        }

        match self.intent_class {
            VoiceIntentResolutionClass::DeniedNoCanonicalCommand => {
                // A denied utterance must bind nothing and commit nothing.
                if self.selected_command_id.is_some() || self.grouped_undo_lineage.is_some() {
                    out.push(VoiceBridgeViolation::DeniedIntentBoundCommand { row_id: id() });
                }
            }
            VoiceIntentResolutionClass::AmbiguousRequiresDisambiguation => {
                if self.candidates.len() < 2 {
                    out.push(VoiceBridgeViolation::AmbiguousWithoutCandidateList { row_id: id() });
                }
                // Ambiguity must never execute silently.
                if self.selected_command_id.is_some() || self.grouped_undo_lineage.is_some() {
                    out.push(VoiceBridgeViolation::AmbiguousIntentAutoExecuted { row_id: id() });
                }
            }
            VoiceIntentResolutionClass::ResolvesToSingleCommand
            | VoiceIntentResolutionClass::ResolvesToDictationText => {
                let high_impact = self
                    .selected_impact()
                    .map(CommandImpactClass::is_high_impact)
                    .unwrap_or(false);

                // High-impact / surprising commands need a confirmation gate and
                // an available, pre-commit transcript correction.
                if high_impact {
                    if self.confirmation_gate
                        != ConfirmationGateClass::ConfirmationRequiredBeforeCommit
                    {
                        out.push(VoiceBridgeViolation::SilentHighImpactWithoutConfirmation {
                            row_id: id(),
                        });
                    }
                    if !self.transcript_strip.correction_availability.is_available()
                        || !self.transcript_strip.shown_before_commit
                    {
                        out.push(
                            VoiceBridgeViolation::CorrectionUnavailableBeforeHighImpactCommit {
                                row_id: id(),
                            },
                        );
                    }
                }

                // A committing gate must produce grouped-undo / audit lineage.
                if self.confirmation_gate.commits() {
                    match &self.grouped_undo_lineage {
                        None => out.push(VoiceBridgeViolation::CommitWithoutGroupedUndoLineage {
                            row_id: id(),
                        }),
                        Some(undo) => {
                            out.extend(self.check_lineage(undo, canonical_ids, high_impact));
                        }
                    }
                }
            }
        }

        out
    }

    /// Validates grouped-undo / audit parity for a committed invocation.
    fn check_lineage(
        &self,
        undo: &GroupedUndoLineage,
        canonical_ids: &BTreeSet<&str>,
        high_impact: bool,
    ) -> Vec<VoiceBridgeViolation> {
        let mut out = Vec::new();
        let id = || self.row_id.clone();

        // The committed command id must agree across selection, keyboard
        // equivalent, and lineage — no second command graph.
        let selected = self.selected_command_id.as_deref();
        if Some(undo.keyboard_equivalent_command_id.as_str()) != selected
            || undo.lineage.command_id != undo.keyboard_equivalent_command_id
        {
            out.push(VoiceBridgeViolation::LineageCommandMismatch { row_id: id() });
        }
        if !canonical_ids.contains(undo.lineage.command_id.as_str()) {
            out.push(VoiceBridgeViolation::InventedNonCanonicalCommand {
                row_id: id(),
                command_id: undo.lineage.command_id.clone(),
            });
        }

        if !undo.commits_through_canonical_session {
            out.push(VoiceBridgeViolation::CommitsThroughSidePath { row_id: id() });
        }
        if !undo.joins_shared_undo_history
            || undo.undo_group_id.trim().is_empty()
            || undo.history_entry_ref.trim().is_empty()
        {
            out.push(VoiceBridgeViolation::UndoNotJoinedToSharedHistory { row_id: id() });
        }

        // Audit lineage must reconstruct end to end.
        if undo.lineage.invocation_session_id.trim().is_empty()
            || undo.lineage.result_packet_id.trim().is_empty()
            || undo.lineage.evidence_refs.is_empty()
            || (undo.lineage.notification_refs.is_empty() && undo.lineage.activity_refs.is_empty())
        {
            out.push(VoiceBridgeViolation::AuditLineageIncomplete { row_id: id() });
        }

        // High-impact commits keep every no-bypass guard strict.
        if high_impact {
            for (guard, value) in undo.no_bypass_guards.named_guards() {
                if !value {
                    out.push(VoiceBridgeViolation::HighImpactGuardsWeakened {
                        row_id: id(),
                        guard: guard.to_owned(),
                    });
                }
            }
        }

        out
    }

    /// One compact, support-safe summary line for the row.
    pub fn compact_line(&self) -> String {
        let selected = self.selected_command_id.as_deref().unwrap_or("-");
        let impact = self
            .selected_impact()
            .map(CommandImpactClass::as_str)
            .unwrap_or("-");
        let undo_group = self
            .grouped_undo_lineage
            .as_ref()
            .map(|undo| undo.undo_group_id.as_str())
            .unwrap_or("-");
        format!(
            "{} | intent={} | gate={} | selected={} | impact={} | correction={} | undo_group={}",
            self.row_id,
            self.intent_class.as_str(),
            self.confirmation_gate.as_str(),
            selected,
            impact,
            self.transcript_strip.correction_availability.as_str(),
            undo_group,
        )
    }
}

/// One way a [`VoiceCommandBridgeRow`] can break the voice-command-bridge
/// contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "violation_kind", rename_all = "snake_case")]
pub enum VoiceBridgeViolation {
    /// A row offers no keyboard-first fallback out of voice.
    MissingKeyboardFallback {
        /// Offending row id.
        row_id: String,
    },
    /// A denied utterance bound or committed a command.
    DeniedIntentBoundCommand {
        /// Offending row id.
        row_id: String,
    },
    /// An ambiguous utterance exposes fewer than two candidates.
    AmbiguousWithoutCandidateList {
        /// Offending row id.
        row_id: String,
    },
    /// An ambiguous utterance executed without a candidate selection.
    AmbiguousIntentAutoExecuted {
        /// Offending row id.
        row_id: String,
    },
    /// A high-impact command commits without a confirmation gate.
    SilentHighImpactWithoutConfirmation {
        /// Offending row id.
        row_id: String,
    },
    /// A high-impact command commits without an available transcript correction.
    CorrectionUnavailableBeforeHighImpactCommit {
        /// Offending row id.
        row_id: String,
    },
    /// A candidate drops a palette-parity field (id / description / shortcut).
    CandidateMissingPaletteParityField {
        /// Offending row id.
        row_id: String,
        /// Offending candidate command id.
        command_id: String,
        /// Missing field name.
        field: String,
    },
    /// A disabled candidate drops the typed disabled reason.
    CandidateMissingDisabledReason {
        /// Offending row id.
        row_id: String,
        /// Offending candidate command id.
        command_id: String,
    },
    /// A command id is not on the supplied stable command graph (invented).
    InventedNonCanonicalCommand {
        /// Offending row id.
        row_id: String,
        /// Offending command id.
        command_id: String,
    },
    /// A committing gate produced no grouped-undo / audit lineage.
    CommitWithoutGroupedUndoLineage {
        /// Offending row id.
        row_id: String,
    },
    /// The committed lineage command id drifts from the selection / keyboard
    /// equivalent.
    LineageCommandMismatch {
        /// Offending row id.
        row_id: String,
    },
    /// A commit bypassed the canonical command session (side path).
    CommitsThroughSidePath {
        /// Offending row id.
        row_id: String,
    },
    /// A commit did not join the shared grouped-undo history.
    UndoNotJoinedToSharedHistory {
        /// Offending row id.
        row_id: String,
    },
    /// A committed lineage cannot reconstruct end to end.
    AuditLineageIncomplete {
        /// Offending row id.
        row_id: String,
    },
    /// A high-impact commit weakened a strict no-bypass guard.
    HighImpactGuardsWeakened {
        /// Offending row id.
        row_id: String,
        /// Weakened guard name.
        guard: String,
    },
}

impl VoiceBridgeViolation {
    /// Stable class token for the violation kind.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::MissingKeyboardFallback { .. } => "missing_keyboard_fallback",
            Self::DeniedIntentBoundCommand { .. } => "denied_intent_bound_command",
            Self::AmbiguousWithoutCandidateList { .. } => "ambiguous_without_candidate_list",
            Self::AmbiguousIntentAutoExecuted { .. } => "ambiguous_intent_auto_executed",
            Self::SilentHighImpactWithoutConfirmation { .. } => {
                "silent_high_impact_without_confirmation"
            }
            Self::CorrectionUnavailableBeforeHighImpactCommit { .. } => {
                "correction_unavailable_before_high_impact_commit"
            }
            Self::CandidateMissingPaletteParityField { .. } => {
                "candidate_missing_palette_parity_field"
            }
            Self::CandidateMissingDisabledReason { .. } => "candidate_missing_disabled_reason",
            Self::InventedNonCanonicalCommand { .. } => "invented_non_canonical_command",
            Self::CommitWithoutGroupedUndoLineage { .. } => "commit_without_grouped_undo_lineage",
            Self::LineageCommandMismatch { .. } => "lineage_command_mismatch",
            Self::CommitsThroughSidePath { .. } => "commits_through_side_path",
            Self::UndoNotJoinedToSharedHistory { .. } => "undo_not_joined_to_shared_history",
            Self::AuditLineageIncomplete { .. } => "audit_lineage_incomplete",
            Self::HighImpactGuardsWeakened { .. } => "high_impact_guards_weakened",
        }
    }

    /// Offending row id.
    pub fn row_id(&self) -> &str {
        match self {
            Self::MissingKeyboardFallback { row_id }
            | Self::DeniedIntentBoundCommand { row_id }
            | Self::AmbiguousWithoutCandidateList { row_id }
            | Self::AmbiguousIntentAutoExecuted { row_id }
            | Self::SilentHighImpactWithoutConfirmation { row_id }
            | Self::CorrectionUnavailableBeforeHighImpactCommit { row_id }
            | Self::CandidateMissingPaletteParityField { row_id, .. }
            | Self::CandidateMissingDisabledReason { row_id, .. }
            | Self::InventedNonCanonicalCommand { row_id, .. }
            | Self::CommitWithoutGroupedUndoLineage { row_id }
            | Self::LineageCommandMismatch { row_id }
            | Self::CommitsThroughSidePath { row_id }
            | Self::UndoNotJoinedToSharedHistory { row_id }
            | Self::AuditLineageIncomplete { row_id }
            | Self::HighImpactGuardsWeakened { row_id, .. } => row_id,
        }
    }
}

/// Cross-row invariant manifest. Every field is `true` exactly when the packet
/// validates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceBridgeInvariantManifest {
    /// Ambiguous and denied utterances never execute silently.
    pub ambiguous_intents_never_auto_execute: bool,
    /// High-impact commands require a confirmation gate and a correction.
    pub high_impact_requires_confirmation_and_correction: bool,
    /// Candidates carry the same id / description / shortcut the palette shows.
    pub candidates_carry_palette_parity_fields: bool,
    /// Disabled candidates carry the typed disabled reason.
    pub disabled_candidates_carry_reason: bool,
    /// No voice path invents a command off the stable command graph.
    pub voice_commands_use_canonical_command_graph: bool,
    /// Voice commits join grouped undo and reconstructable audit lineage.
    pub voice_commits_join_grouped_undo_and_audit: bool,
    /// Every row offers a keyboard-first fallback.
    pub every_row_offers_keyboard_fallback: bool,
}

impl VoiceBridgeInvariantManifest {
    /// The all-satisfied manifest.
    pub const fn all_true() -> Self {
        Self {
            ambiguous_intents_never_auto_execute: true,
            high_impact_requires_confirmation_and_correction: true,
            candidates_carry_palette_parity_fields: true,
            disabled_candidates_carry_reason: true,
            voice_commands_use_canonical_command_graph: true,
            voice_commits_join_grouped_undo_and_audit: true,
            every_row_offers_keyboard_fallback: true,
        }
    }

    /// Recomputes the manifest from a row set by lowering each row's violations
    /// onto the matching invariant.
    pub fn from_rows(rows: &[VoiceCommandBridgeRow], canonical_ids: &BTreeSet<&str>) -> Self {
        let mut manifest = Self::all_true();
        for row in rows {
            for violation in row.check(canonical_ids) {
                match violation {
                    VoiceBridgeViolation::AmbiguousWithoutCandidateList { .. }
                    | VoiceBridgeViolation::AmbiguousIntentAutoExecuted { .. }
                    | VoiceBridgeViolation::DeniedIntentBoundCommand { .. } => {
                        manifest.ambiguous_intents_never_auto_execute = false;
                    }
                    VoiceBridgeViolation::SilentHighImpactWithoutConfirmation { .. }
                    | VoiceBridgeViolation::CorrectionUnavailableBeforeHighImpactCommit {
                        ..
                    } => {
                        manifest.high_impact_requires_confirmation_and_correction = false;
                    }
                    VoiceBridgeViolation::CandidateMissingPaletteParityField { .. } => {
                        manifest.candidates_carry_palette_parity_fields = false;
                    }
                    VoiceBridgeViolation::CandidateMissingDisabledReason { .. } => {
                        manifest.disabled_candidates_carry_reason = false;
                    }
                    VoiceBridgeViolation::InventedNonCanonicalCommand { .. } => {
                        manifest.voice_commands_use_canonical_command_graph = false;
                    }
                    VoiceBridgeViolation::CommitWithoutGroupedUndoLineage { .. }
                    | VoiceBridgeViolation::LineageCommandMismatch { .. }
                    | VoiceBridgeViolation::CommitsThroughSidePath { .. }
                    | VoiceBridgeViolation::UndoNotJoinedToSharedHistory { .. }
                    | VoiceBridgeViolation::AuditLineageIncomplete { .. }
                    | VoiceBridgeViolation::HighImpactGuardsWeakened { .. } => {
                        manifest.voice_commits_join_grouped_undo_and_audit = false;
                    }
                    VoiceBridgeViolation::MissingKeyboardFallback { .. } => {
                        manifest.every_row_offers_keyboard_fallback = false;
                    }
                }
            }
        }
        manifest
    }
}

/// Inspectable truth packet for the voice-command-bridge lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceCommandBridgePacket {
    /// Record discriminator; equals [`VOICE_COMMAND_BRIDGE_PACKET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; equals [`VOICE_COMMAND_BRIDGE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Ref to the published companion doc.
    pub doc_ref: String,
    /// Ref to the checked-in fixtures directory.
    pub fixtures_dir_ref: String,
    /// Ref to the command-descriptor contract the candidate fields come from.
    pub command_descriptor_contract_ref: String,
    /// Ref to the command result-packet schema the committed lineage reuses.
    pub command_result_packet_schema_ref: String,
    /// Ref to the cross-surface voice / dictation contract this bridge rides.
    pub voice_and_dictation_contract_ref: String,
    /// The stable command graph the bridge is allowed to reach, sorted.
    pub canonical_command_ids: Vec<CommandId>,
    /// Bridge rows, in canonical order.
    pub rows: Vec<VoiceCommandBridgeRow>,
    /// Cross-row invariant manifest.
    pub invariants: VoiceBridgeInvariantManifest,
    /// `true` — no raw audio / transcript bytes ever cross this boundary.
    pub raw_audio_or_transcript_bytes_excluded: bool,
}

impl VoiceCommandBridgePacket {
    /// Builds a packet from `rows` and the stable command graph `canonical_ids`,
    /// stamping the canonical envelope and recomputing the invariant manifest.
    pub fn new(rows: Vec<VoiceCommandBridgeRow>, canonical_ids: Vec<CommandId>) -> Self {
        let mut canonical_command_ids = canonical_ids;
        canonical_command_ids.sort();
        canonical_command_ids.dedup();
        let id_set: BTreeSet<&str> = canonical_command_ids.iter().map(String::as_str).collect();
        let invariants = VoiceBridgeInvariantManifest::from_rows(&rows, &id_set);
        Self {
            record_kind: VOICE_COMMAND_BRIDGE_PACKET_RECORD_KIND.to_owned(),
            schema_version: VOICE_COMMAND_BRIDGE_SCHEMA_VERSION,
            shared_contract_ref: VOICE_COMMAND_BRIDGE_SHARED_CONTRACT_REF.to_owned(),
            packet_id: VOICE_COMMAND_BRIDGE_PACKET_ID.to_owned(),
            doc_ref: VOICE_COMMAND_BRIDGE_DOC_REF.to_owned(),
            fixtures_dir_ref: VOICE_COMMAND_BRIDGE_FIXTURES_DIR_REF.to_owned(),
            command_descriptor_contract_ref: COMMAND_DESCRIPTOR_CONTRACT_REF.to_owned(),
            command_result_packet_schema_ref: COMMAND_RESULT_PACKET_SCHEMA_REF.to_owned(),
            voice_and_dictation_contract_ref: VOICE_AND_DICTATION_CONTRACT_REF.to_owned(),
            canonical_command_ids,
            rows,
            invariants,
            raw_audio_or_transcript_bytes_excluded: true,
        }
    }

    /// The stable command graph as a borrow-friendly set.
    pub fn canonical_id_set(&self) -> BTreeSet<&str> {
        self.canonical_command_ids
            .iter()
            .map(String::as_str)
            .collect()
    }

    /// Returns the row with `row_id`, if present.
    pub fn row(&self, row_id: &str) -> Option<&VoiceCommandBridgeRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// Collects every invariant violation across all rows. An empty result means
    /// every voice command resolution stays inspectable, confirmation-gated,
    /// palette-faithful, and lineage-parity-clean.
    pub fn validate(&self) -> Vec<VoiceBridgeViolation> {
        let id_set = self.canonical_id_set();
        self.rows
            .iter()
            .flat_map(|row| row.check(&id_set))
            .collect()
    }

    /// `true` when no row violates an invariant.
    pub fn is_well_formed(&self) -> bool {
        self.validate().is_empty()
    }

    /// Support-safe compact lines, one per row, plus a header.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity(self.rows.len() + 1);
        lines.push(format!(
            "{} | rows={} | invariants_ok={}",
            self.packet_id,
            self.rows.len(),
            self.is_well_formed(),
        ));
        lines.extend(self.rows.iter().map(VoiceCommandBridgeRow::compact_line));
        lines
    }

    /// Renders the published Markdown companion summary.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Voice disambiguation and confirmation\n\n");
        out.push_str(
            "Generated from the `voice_bridge` seed. Do not edit by hand; regenerate with \
             `cargo run -p aureline-commands --example dump_voice_command_bridge -- write`.\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Descriptor contract: `{}`\n",
            self.command_descriptor_contract_ref
        ));
        out.push_str(&format!(
            "- Result-packet schema: `{}`\n",
            self.command_result_packet_schema_ref
        ));
        out.push_str(&format!(
            "- Voice/dictation contract: `{}`\n",
            self.voice_and_dictation_contract_ref
        ));
        out.push_str(&format!("- Fixtures: `{}`\n\n", self.fixtures_dir_ref));

        out.push_str("## Recognized utterances\n\n");
        out.push_str("| Row | Intent | Gate | Selected | Impact | Correction | Undo group |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.rows {
            let selected = row.selected_command_id.as_deref().unwrap_or("-");
            let impact = row
                .selected_impact()
                .map(CommandImpactClass::as_str)
                .unwrap_or("-");
            let undo_group = row
                .grouped_undo_lineage
                .as_ref()
                .map(|undo| undo.undo_group_id.as_str())
                .unwrap_or("-");
            out.push_str(&format!(
                "| `{}` | {} | {} | `{}` | {} | {} | `{}` |\n",
                row.row_id,
                row.intent_class.as_str(),
                row.confirmation_gate.as_str(),
                selected,
                impact,
                row.transcript_strip.correction_availability.as_str(),
                undo_group,
            ));
        }
        out.push('\n');

        out.push_str("## Candidate parity\n\n");
        out.push_str(
            "Each candidate carries the same stable command id, description, keyboard \
             shortcut narration, and disabled reason the command palette projects.\n\n",
        );
        out.push_str(
            "| Row | Candidate | Enablement | Disabled reason | Impact | Preview | Approval |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.rows {
            for candidate in &row.candidates {
                let disabled = candidate
                    .disabled_reason_code
                    .map(|code| code.as_str())
                    .unwrap_or("-");
                out.push_str(&format!(
                    "| `{}` | `{}` | {} | `{}` | {} | {} | {} |\n",
                    row.row_id,
                    candidate.candidate_command_id,
                    candidate.enablement_decision_class.as_str(),
                    disabled,
                    candidate.impact_class.as_str(),
                    candidate.preview_required,
                    candidate.approval_required,
                ));
            }
        }
        out.push('\n');

        out.push_str("## Invariants\n\n");
        let inv = &self.invariants;
        for (label, value) in [
            (
                "Ambiguous / denied utterances never execute silently",
                inv.ambiguous_intents_never_auto_execute,
            ),
            (
                "High-impact commands require confirmation and a correction",
                inv.high_impact_requires_confirmation_and_correction,
            ),
            (
                "Candidates carry palette parity fields",
                inv.candidates_carry_palette_parity_fields,
            ),
            (
                "Disabled candidates carry a reason",
                inv.disabled_candidates_carry_reason,
            ),
            (
                "Voice commands use the canonical command graph",
                inv.voice_commands_use_canonical_command_graph,
            ),
            (
                "Voice commits join grouped undo and audit",
                inv.voice_commits_join_grouped_undo_and_audit,
            ),
            (
                "Every row offers a keyboard-first fallback",
                inv.every_row_offers_keyboard_fallback,
            ),
        ] {
            out.push_str(&format!(
                "- [{}] {}\n",
                if value { "x" } else { " " },
                label
            ));
        }
        out
    }

    /// Serializes the packet as the canonical export-safe pretty JSON (no
    /// trailing newline).
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("voice command-bridge packet serializes")
    }
}

impl NoBypassGuards {
    /// Named (guard, value) pairs, in a stable order, for per-guard reporting.
    pub fn named_guards(&self) -> [(&'static str, bool); 10] {
        [
            (
                "trust_revalidation_required",
                self.trust_revalidation_required,
            ),
            (
                "policy_revalidation_required",
                self.policy_revalidation_required,
            ),
            (
                "permission_prompt_revalidation_required",
                self.permission_prompt_revalidation_required,
            ),
            ("preview_path_preserved", self.preview_path_preserved),
            ("approval_path_preserved", self.approval_path_preserved),
            (
                "credential_broker_revalidation_required",
                self.credential_broker_revalidation_required,
            ),
            (
                "execution_context_revalidation_required",
                self.execution_context_revalidation_required,
            ),
            (
                "freshness_floor_revalidation_required",
                self.freshness_floor_revalidation_required,
            ),
            (
                "capability_class_may_not_widen",
                self.capability_class_may_not_widen,
            ),
            (
                "result_schema_may_not_be_replaced",
                self.result_schema_may_not_be_replaced,
            ),
        ]
    }
}

/// Serializes a value as pretty JSON with a trailing newline (the on-disk
/// fixture form).
pub fn fixture_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    let mut json = serde_json::to_string_pretty(value)?;
    json.push('\n');
    Ok(json)
}

/// Stable per-row fixture file name (the slug after the last `:` in the row id).
pub fn row_fixture_file_name(row: &VoiceCommandBridgeRow) -> String {
    let slug = row.row_id.rsplit(':').next().unwrap_or(&row.row_id);
    format!("{slug}.json")
}

/// Writes the seeded packet, the per-row fixtures, and the compact summary to
/// `dir`. This is the single mint path the example dump and the equality test
/// share, so the checked-in fixtures can never drift silently.
pub fn write_fixtures(dir: &Path, packet: &VoiceCommandBridgePacket) -> io::Result<()> {
    fs::create_dir_all(dir)?;

    let packet_json =
        fixture_json(packet).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    fs::write(dir.join("packet.json"), packet_json)?;

    for row in &packet.rows {
        let json = fixture_json(row).map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        fs::write(dir.join(row_fixture_file_name(row)), json)?;
    }

    let mut compact = packet.compact_lines().join("\n");
    compact.push('\n');
    fs::write(dir.join("compact.txt"), compact)?;

    Ok(())
}

mod seed;
pub use seed::seeded_voice_command_bridge_packet;

#[cfg(test)]
mod tests;
