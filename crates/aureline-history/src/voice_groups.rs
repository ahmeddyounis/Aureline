//! Grouped-history parity for dictated text captures.
//!
//! Dictation is an explicit, privacy-bounded input mode, but a dictated word, a
//! spoken punctuation mark, a formatting intent, or a correction gesture is
//! still ordinary text editing. This module proves that a dictation *capture*
//! lands in the **same** grouped editor history as typing and paste — never a
//! voice-only journal class and never a hidden speech buffer — by projecting the
//! capture onto the shared mutation-journal vocabulary ([`ReversalClass`],
//! [`RedactionClass`], [`DurableVsDisposable`], [`SourceClass`]) and refusing any
//! group whose members do not behave like the editor's frozen text-edit undo
//! classes.
//!
//! The record carries only typed class tokens, opaque ids, byte counts, and
//! redaction-aware label refs — never raw audio bytes, raw transcript text, or
//! raw provider payloads. The editor runtime bridge in `aureline-editor` builds
//! one [`VoiceHistoryGroupRecord`] per capture through
//! [`VoiceHistoryGroupRecord::from_input`]; shell, diagnostics, and
//! support-export surfaces should ingest the record rather than re-deriving
//! dictation history parity by hand.

use serde::{Deserialize, Serialize};

use crate::mutation_journal::{DurableVsDisposable, RedactionClass, ReversalClass, SourceClass};

/// Schema version stamped on every voice-history group record.
pub const VOICE_HISTORY_GROUP_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`VoiceHistoryGroupRecord`].
pub const VOICE_HISTORY_GROUP_RECORD_KIND: &str = "voice_history_group_record";

/// Canonical command id that opens a dictation capture. A capture always
/// dispatches through this single command so every dictated edit shares the
/// keyboard / palette command lineage rather than a side path.
pub const DICTATION_CAPTURE_COMMAND_ID: &str = "cmd:voice.dictation.capture";

/// Frozen editor undo-class ids a dictated edit is allowed to use.
///
/// A dictated edit is ordinary text editing, so it MUST land on one of these
/// classes — the same classes typing and paste use — never a bespoke voice-only
/// class. The list mirrors the text-edit subset of the frozen buffer undo-class
/// taxonomy.
pub const ORDINARY_TEXT_EDIT_UNDO_CLASS_IDS: [&str; 2] = ["text_edit", "multi_cursor_text_edit"];

/// Coarse class of one dictation intent, shared by the editor runtime bridge and
/// this history projection so both sides speak one vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DictationIntentClass {
    /// Plain dictated words inserted at the caret.
    Text,
    /// A spoken punctuation mark ("period", "comma", ...).
    Punctuation,
    /// A spoken formatting intent ("new line", "new paragraph", "tab").
    Formatting,
    /// A spoken correction gesture ("scratch that", "delete that", ...).
    Correction,
}

impl DictationIntentClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Punctuation => "punctuation",
            Self::Formatting => "formatting",
            Self::Correction => "correction",
        }
    }
}

/// Where the speech recognizer that produced a capture runs.
///
/// This is the privacy-relevant disclosure: a hosted recognizer means audio or
/// transcript left the device, so it can never be silently laundered into an
/// on-device lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DictationRecognitionLocality {
    /// Recognition ran on-device; no audio or transcript left the machine.
    OnDeviceLocal,
    /// Recognition ran on a hosted provider; disclosed as off-device.
    HostedProvider,
}

impl DictationRecognitionLocality {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnDeviceLocal => "on_device_local",
            Self::HostedProvider => "hosted_provider",
        }
    }
}

/// One member edit of a dictation history group.
///
/// Each member is a single committed text-edit transaction produced by routing a
/// dictation intent through the shared editor edit model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceHistoryGroupMember {
    /// Stable per-edit mutation id (shared with the editor edit record).
    pub mutation_id: String,
    /// Coarse intent class that produced the edit.
    pub intent_class: DictationIntentClass,
    /// Frozen editor undo-class id the committed transaction reported.
    pub undo_class_id: String,
    /// `true` when the committed transaction is reversible through normal undo.
    pub reversible: bool,
    /// Bytes inserted by the committed transaction.
    pub inserted_bytes: u64,
    /// Bytes removed by the committed transaction.
    pub removed_bytes: u64,
}

impl VoiceHistoryGroupMember {
    /// Creates a member edit descriptor.
    pub fn new(
        mutation_id: impl Into<String>,
        intent_class: DictationIntentClass,
        undo_class_id: impl Into<String>,
        reversible: bool,
        inserted_bytes: u64,
        removed_bytes: u64,
    ) -> Self {
        Self {
            mutation_id: mutation_id.into(),
            intent_class,
            undo_class_id: undo_class_id.into(),
            reversible,
            inserted_bytes,
            removed_bytes,
        }
    }

    /// `true` when the member rode one of the ordinary text-edit undo classes.
    pub fn uses_ordinary_text_edit_class(&self) -> bool {
        ORDINARY_TEXT_EDIT_UNDO_CLASS_IDS.contains(&self.undo_class_id.as_str())
    }
}

/// Inputs for building a [`VoiceHistoryGroupRecord`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VoiceHistoryGroupInput {
    /// Opaque capture / group id.
    pub group_id: String,
    /// Stable id of the text-entry surface the capture targeted.
    pub surface_id: String,
    /// Canonical command id that opened the capture.
    pub command_id: String,
    /// Recognition locality disclosed for the capture.
    pub recognition_locality: DictationRecognitionLocality,
    /// Member edits, in apply order.
    pub members: Vec<VoiceHistoryGroupMember>,
}

/// Grouped-history parity record for one dictation capture.
///
/// Built through [`VoiceHistoryGroupRecord::from_input`], the record always
/// stamps the safe shared-vocabulary values (exact-undo, user-authored,
/// metadata-only, joins shared history). [`VoiceHistoryGroupRecord::check`]
/// re-validates those invariants so a hand-authored or deserialized record that
/// drifted is caught.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VoiceHistoryGroupRecord {
    /// Record discriminator; equals [`VOICE_HISTORY_GROUP_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; equals [`VOICE_HISTORY_GROUP_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Opaque capture / group id.
    pub group_id: String,
    /// Stable id of the text-entry surface the capture targeted.
    pub surface_id: String,
    /// Canonical command id that opened the capture.
    pub command_id: String,
    /// Recognition locality disclosed for the capture.
    pub recognition_locality: DictationRecognitionLocality,
    /// Journal source class — the local human author, mediated by a recognizer.
    pub source_class: SourceClass,
    /// Journal reversal class — dictated edits reverse by exact undo.
    pub reversal_class: ReversalClass,
    /// Journal redaction class — metadata only; no raw bodies cross the boundary.
    pub redaction_class: RedactionClass,
    /// Journal durability class — dictated text is durable user-authored content.
    pub durable_vs_disposable: DurableVsDisposable,
    /// `true` when the capture joins the shared undo history, not a side log.
    pub joins_shared_undo_history: bool,
    /// Member edits, in apply order.
    pub members: Vec<VoiceHistoryGroupMember>,
}

impl VoiceHistoryGroupRecord {
    /// Projects a dictation capture onto the shared history vocabulary.
    ///
    /// The author is the local human; the dictated edits are exact-undo
    /// reversible, durable user-authored, and metadata-only for export, and they
    /// join the shared undo history — the same posture typing produces.
    pub fn from_input(input: VoiceHistoryGroupInput) -> Self {
        Self {
            record_kind: VOICE_HISTORY_GROUP_RECORD_KIND.to_owned(),
            schema_version: VOICE_HISTORY_GROUP_SCHEMA_VERSION,
            group_id: input.group_id,
            surface_id: input.surface_id,
            command_id: input.command_id,
            recognition_locality: input.recognition_locality,
            source_class: SourceClass::HumanLocal,
            reversal_class: ReversalClass::ExactUndo,
            redaction_class: RedactionClass::MetadataOnly,
            durable_vs_disposable: DurableVsDisposable::DurableUserAuthored,
            joins_shared_undo_history: true,
            members: input.members,
        }
    }

    /// Total bytes inserted across the group's members.
    pub fn inserted_bytes(&self) -> u64 {
        self.members.iter().map(|m| m.inserted_bytes).sum()
    }

    /// Total bytes removed across the group's members.
    pub fn removed_bytes(&self) -> u64 {
        self.members.iter().map(|m| m.removed_bytes).sum()
    }

    /// Collects every invariant this record violates. An empty result means the
    /// dictation capture lands as an ordinary, reversible, user-authored,
    /// metadata-only group joined to shared history.
    pub fn check(&self) -> Vec<VoiceGroupViolation> {
        let mut out = Vec::new();
        let id = || self.group_id.clone();

        if self.group_id.trim().is_empty()
            || self.surface_id.trim().is_empty()
            || self.command_id.trim().is_empty()
        {
            out.push(VoiceGroupViolation::MissingLineageField { group_id: id() });
        }
        if self.members.is_empty() {
            out.push(VoiceGroupViolation::EmptyGroup { group_id: id() });
        }
        if !self.joins_shared_undo_history {
            out.push(VoiceGroupViolation::NotJoinedToSharedHistory { group_id: id() });
        }
        if self.durable_vs_disposable != DurableVsDisposable::DurableUserAuthored {
            out.push(VoiceGroupViolation::NotUserAuthored { group_id: id() });
        }
        if self.redaction_class != RedactionClass::MetadataOnly {
            out.push(VoiceGroupViolation::RawBodyRetentionRisk { group_id: id() });
        }
        if self.reversal_class != ReversalClass::ExactUndo {
            out.push(VoiceGroupViolation::NotExactUndo { group_id: id() });
        }
        for member in &self.members {
            if !member.uses_ordinary_text_edit_class() {
                out.push(VoiceGroupViolation::MemberUsesNonEditUndoClass {
                    group_id: id(),
                    mutation_id: member.mutation_id.clone(),
                    undo_class_id: member.undo_class_id.clone(),
                });
            }
            if !member.reversible {
                out.push(VoiceGroupViolation::MemberNotReversible {
                    group_id: id(),
                    mutation_id: member.mutation_id.clone(),
                });
            }
        }

        out
    }

    /// `true` when no invariant is violated.
    pub fn is_well_formed(&self) -> bool {
        self.check().is_empty()
    }

    /// One compact, support-safe summary line.
    pub fn compact_line(&self) -> String {
        format!(
            "{} | surface={} | locality={} | members={} | +{}/-{}B | well_formed={}",
            self.group_id,
            self.surface_id,
            self.recognition_locality.as_str(),
            self.members.len(),
            self.inserted_bytes(),
            self.removed_bytes(),
            self.is_well_formed(),
        )
    }
}

/// One way a [`VoiceHistoryGroupRecord`] can break dictation history parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "violation_kind", rename_all = "snake_case")]
pub enum VoiceGroupViolation {
    /// A required lineage field (group / surface / command id) is empty.
    MissingLineageField {
        /// Offending group id.
        group_id: String,
    },
    /// A capture committed nothing yet claims a history group.
    EmptyGroup {
        /// Offending group id.
        group_id: String,
    },
    /// A capture did not join the shared undo history.
    NotJoinedToSharedHistory {
        /// Offending group id.
        group_id: String,
    },
    /// A dictated capture is not classified as durable user-authored content.
    NotUserAuthored {
        /// Offending group id.
        group_id: String,
    },
    /// A dictated capture risks raw-body retention (not metadata-only).
    RawBodyRetentionRisk {
        /// Offending group id.
        group_id: String,
    },
    /// A dictated capture does not reverse by exact undo.
    NotExactUndo {
        /// Offending group id.
        group_id: String,
    },
    /// A member edit used an undo class outside the ordinary text-edit set.
    MemberUsesNonEditUndoClass {
        /// Offending group id.
        group_id: String,
        /// Offending member mutation id.
        mutation_id: String,
        /// The non-ordinary undo class id the member carried.
        undo_class_id: String,
    },
    /// A member edit is not reversible through normal undo.
    MemberNotReversible {
        /// Offending group id.
        group_id: String,
        /// Offending member mutation id.
        mutation_id: String,
    },
}

impl VoiceGroupViolation {
    /// Stable class token for the violation kind.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::MissingLineageField { .. } => "missing_lineage_field",
            Self::EmptyGroup { .. } => "empty_group",
            Self::NotJoinedToSharedHistory { .. } => "not_joined_to_shared_history",
            Self::NotUserAuthored { .. } => "not_user_authored",
            Self::RawBodyRetentionRisk { .. } => "raw_body_retention_risk",
            Self::NotExactUndo { .. } => "not_exact_undo",
            Self::MemberUsesNonEditUndoClass { .. } => "member_uses_non_edit_undo_class",
            Self::MemberNotReversible { .. } => "member_not_reversible",
        }
    }

    /// Offending group id.
    pub fn group_id(&self) -> &str {
        match self {
            Self::MissingLineageField { group_id }
            | Self::EmptyGroup { group_id }
            | Self::NotJoinedToSharedHistory { group_id }
            | Self::NotUserAuthored { group_id }
            | Self::RawBodyRetentionRisk { group_id }
            | Self::NotExactUndo { group_id }
            | Self::MemberUsesNonEditUndoClass { group_id, .. }
            | Self::MemberNotReversible { group_id, .. } => group_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn member(id: &str, class: DictationIntentClass, undo: &str) -> VoiceHistoryGroupMember {
        VoiceHistoryGroupMember::new(id, class, undo, true, 3, 0)
    }

    fn well_formed_input() -> VoiceHistoryGroupInput {
        VoiceHistoryGroupInput {
            group_id: "capture:1".to_owned(),
            surface_id: "editor.main".to_owned(),
            command_id: DICTATION_CAPTURE_COMMAND_ID.to_owned(),
            recognition_locality: DictationRecognitionLocality::OnDeviceLocal,
            members: vec![
                member("capture:1:edit:00", DictationIntentClass::Text, "text_edit"),
                member(
                    "capture:1:edit:01",
                    DictationIntentClass::Punctuation,
                    "text_edit",
                ),
            ],
        }
    }

    #[test]
    fn from_input_is_parity_clean() {
        let record = VoiceHistoryGroupRecord::from_input(well_formed_input());
        assert!(record.is_well_formed(), "{:?}", record.check());
        assert_eq!(record.source_class, SourceClass::HumanLocal);
        assert_eq!(record.reversal_class, ReversalClass::ExactUndo);
        assert_eq!(record.redaction_class, RedactionClass::MetadataOnly);
        assert_eq!(
            record.durable_vs_disposable,
            DurableVsDisposable::DurableUserAuthored
        );
        assert_eq!(record.inserted_bytes(), 6);
    }

    #[test]
    fn non_edit_undo_class_is_flagged() {
        let mut record = VoiceHistoryGroupRecord::from_input(well_formed_input());
        record.members[0].undo_class_id = "machine_generated_change".to_owned();
        let violations = record.check();
        assert!(violations
            .iter()
            .any(|v| matches!(v, VoiceGroupViolation::MemberUsesNonEditUndoClass { .. })));
    }

    #[test]
    fn empty_group_is_flagged() {
        let mut input = well_formed_input();
        input.members.clear();
        let record = VoiceHistoryGroupRecord::from_input(input);
        assert!(record
            .check()
            .iter()
            .any(|v| matches!(v, VoiceGroupViolation::EmptyGroup { .. })));
    }

    #[test]
    fn retention_drift_is_flagged() {
        let mut record = VoiceHistoryGroupRecord::from_input(well_formed_input());
        record.redaction_class = RedactionClass::HighRisk;
        assert!(record
            .check()
            .iter()
            .any(|v| matches!(v, VoiceGroupViolation::RawBodyRetentionRisk { .. })));
    }
}
