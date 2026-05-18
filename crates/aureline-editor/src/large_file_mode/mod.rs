//! Limited-mode file disclosure records.
//!
//! This module projects the large-file classifier and constrained viewer into
//! a stable record that UI, CLI, review, repair, and support-export surfaces can
//! all quote. The record does not claim normal editor parity; it names the
//! reduced capabilities, safe preview posture, constrained write posture, and
//! explicit override route.

use serde::{Deserialize, Serialize};

use crate::large_file::{LargeFileDocument, LargeFileTrigger};

/// Schema version for limited-mode file records.
pub const LIMITED_MODE_FILE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for limited-mode file records.
pub const LIMITED_MODE_FILE_SCHEMA_REF: &str = "schemas/editor/limited_mode_file.schema.json";

/// Stable record-kind tag for limited-mode file records.
pub const LIMITED_MODE_FILE_RECORD_KIND: &str = "limited_mode_file_record";

/// Activation trigger for limited mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LimitedModeActivationTrigger {
    /// File size exceeded the configured threshold.
    SizeThreshold,
    /// Opening in the normal path would exceed the resource budget.
    ResourcePressure,
    /// Binary-like, minified, hostile, or pack-suffix classification.
    Classification,
    /// Decode recovery chose limited mode.
    DecodePosture,
    /// User explicitly opened limited mode.
    OperatorOverride,
}

impl LimitedModeActivationTrigger {
    /// Returns the stable string vocabulary for this trigger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SizeThreshold => "size_threshold",
            Self::ResourcePressure => "resource_pressure",
            Self::Classification => "classification",
            Self::DecodePosture => "decode_posture",
            Self::OperatorOverride => "operator_override",
        }
    }
}

impl From<LargeFileTrigger> for LimitedModeActivationTrigger {
    fn from(value: LargeFileTrigger) -> Self {
        match value {
            LargeFileTrigger::SizeThreshold => Self::SizeThreshold,
            LargeFileTrigger::ResourcePressure => Self::ResourcePressure,
            LargeFileTrigger::Classification => Self::Classification,
            LargeFileTrigger::DecodePosture => Self::DecodePosture,
            LargeFileTrigger::OperatorOverride => Self::OperatorOverride,
        }
    }
}

/// Capability state in limited mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LimitedModeCapabilityState {
    /// Capability remains available.
    Allowed,
    /// Capability remains available only through a narrower path.
    Downgraded,
    /// Capability is disabled in limited mode.
    Denied,
}

impl LimitedModeCapabilityState {
    /// Returns the stable string vocabulary for this capability state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Downgraded => "downgraded",
            Self::Denied => "denied",
        }
    }
}

/// One limited-mode capability row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LimitedModeCapabilityRecord {
    /// Stable capability id.
    pub capability_id: String,
    /// Limited-mode state.
    pub state: LimitedModeCapabilityState,
    /// User- and support-visible disclosure.
    pub disclosure: String,
}

/// Safe preview posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LimitedModeSafePreviewClass {
    /// Raw bytes are paged and rendered as a text preview where decodable.
    PagedRawTextPreview,
    /// Binary-like content is inspected through safe metadata and bounded ranges.
    BinarySafePreview,
}

impl LimitedModeSafePreviewClass {
    /// Returns the stable string vocabulary for this preview class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PagedRawTextPreview => "paged_raw_text_preview",
            Self::BinarySafePreview => "binary_safe_preview",
        }
    }
}

/// Edit posture in limited mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LimitedModeEditPolicyClass {
    /// Editing is read-only unless a narrower reviewed overlay is available.
    ReadOnlyByDefault,
    /// Only bounded range edits may be admitted.
    RangeConstrained,
}

impl LimitedModeEditPolicyClass {
    /// Returns the stable string vocabulary for this edit policy.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyByDefault => "read_only_by_default",
            Self::RangeConstrained => "range_constrained",
        }
    }
}

/// Write posture in limited mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LimitedModeWritePolicyClass {
    /// Whole-file participant writes are blocked.
    WholeFileParticipantsBlocked,
    /// Range-only reviewed writes may be admitted.
    RangeOnlyReviewedWrites,
}

impl LimitedModeWritePolicyClass {
    /// Returns the stable string vocabulary for this write policy.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WholeFileParticipantsBlocked => "whole_file_participants_blocked",
            Self::RangeOnlyReviewedWrites => "range_only_reviewed_writes",
        }
    }
}

/// Explicit override route for limited mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LimitedModeOverrideAction {
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Disclosure shown before escalation.
    pub disclosure: String,
}

/// File record explaining why limited mode is active and what is reduced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LimitedModeFileRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub limited_mode_file_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable record id.
    pub limited_mode_file_id: String,
    /// Workspace reference.
    pub workspace_ref: String,
    /// Logical document reference.
    pub document_ref: String,
    /// Canonical write target as resolved by VFS.
    pub canonical_uri: String,
    /// Bytes observed on disk at open.
    pub bytes_on_disk: u64,
    /// Activation trigger, when known.
    pub activation_trigger_class: Option<LimitedModeActivationTrigger>,
    /// Classifier reason shown to users and support.
    pub activation_reason: String,
    /// Safe preview posture.
    pub safe_preview_class: LimitedModeSafePreviewClass,
    /// Edit policy.
    pub edit_policy_class: LimitedModeEditPolicyClass,
    /// Write policy.
    pub write_policy_class: LimitedModeWritePolicyClass,
    /// Explicit override route.
    pub override_action: LimitedModeOverrideAction,
    /// Capability rows.
    pub capabilities: Vec<LimitedModeCapabilityRecord>,
    /// Whether the record excludes raw source bytes for support export.
    pub raw_payload_excluded: bool,
    /// Support-safe summary.
    pub support_summary: String,
}

impl LimitedModeFileRecord {
    /// Builds a limited-mode record from a constrained large-file document.
    pub fn from_large_file_document(
        limited_mode_file_id: impl Into<String>,
        workspace_ref: impl Into<String>,
        document_ref: impl Into<String>,
        document: &LargeFileDocument,
    ) -> Self {
        let decision = document.viewer.decision();
        let canonical_uri = document
            .identity
            .canonical_filesystem_object
            .canonical_uri
            .to_string();
        let activation_trigger_class = decision.trigger.map(Into::into);
        let safe_preview_class = if decision.sniff.heuristics.looks_binary {
            LimitedModeSafePreviewClass::BinarySafePreview
        } else {
            LimitedModeSafePreviewClass::PagedRawTextPreview
        };
        Self {
            record_kind: LIMITED_MODE_FILE_RECORD_KIND.to_owned(),
            limited_mode_file_schema_version: LIMITED_MODE_FILE_SCHEMA_VERSION,
            schema_ref: LIMITED_MODE_FILE_SCHEMA_REF.to_owned(),
            limited_mode_file_id: limited_mode_file_id.into(),
            workspace_ref: workspace_ref.into(),
            document_ref: document_ref.into(),
            canonical_uri,
            bytes_on_disk: decision.bytes_on_disk,
            activation_trigger_class,
            activation_reason: decision.reason.clone(),
            safe_preview_class,
            edit_policy_class: LimitedModeEditPolicyClass::ReadOnlyByDefault,
            write_policy_class: LimitedModeWritePolicyClass::WholeFileParticipantsBlocked,
            override_action: LimitedModeOverrideAction {
                action_id: "open_anyway".to_owned(),
                label: "Open anyway".to_owned(),
                disclosure:
                    "Opens the normal editor path and may be slow, memory intensive, or unsafe for broad writes."
                        .to_owned(),
            },
            capabilities: default_limited_mode_capabilities(),
            raw_payload_excluded: true,
            support_summary:
                "Limited mode uses safe preview, reduced analysis, constrained writes, and an explicit override route."
                    .to_owned(),
        }
    }

    /// Returns true when support export can include the record without raw bytes.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.record_kind == LIMITED_MODE_FILE_RECORD_KIND
            && self.schema_ref == LIMITED_MODE_FILE_SCHEMA_REF
            && !self.capabilities.is_empty()
    }

    /// Returns a capability row by id.
    pub fn capability(&self, capability_id: &str) -> Option<&LimitedModeCapabilityRecord> {
        self.capabilities
            .iter()
            .find(|capability| capability.capability_id == capability_id)
    }
}

/// Returns the default limited-mode capability table.
pub fn default_limited_mode_capabilities() -> Vec<LimitedModeCapabilityRecord> {
    vec![
        capability(
            "view",
            LimitedModeCapabilityState::Allowed,
            "Read-only viewport rendering remains available.",
        ),
        capability(
            "search_viewport",
            LimitedModeCapabilityState::Allowed,
            "Viewport-bounded search remains available.",
        ),
        capability(
            "search_whole_file",
            LimitedModeCapabilityState::Downgraded,
            "Whole-file search uses streaming page reads.",
        ),
        capability(
            "copy",
            LimitedModeCapabilityState::Allowed,
            "Selection copy remains available.",
        ),
        capability(
            "diagnostics_viewport",
            LimitedModeCapabilityState::Allowed,
            "Viewport-bounded diagnostics may run.",
        ),
        capability(
            "diagnostics_whole_file",
            LimitedModeCapabilityState::Denied,
            "Full-file diagnostics are disabled.",
        ),
        capability(
            "multi_cursor_viewport",
            LimitedModeCapabilityState::Allowed,
            "Viewport-bounded multi-cursor remains available.",
        ),
        capability(
            "multi_cursor_whole_file",
            LimitedModeCapabilityState::Denied,
            "Whole-file multi-cursor edits are disabled.",
        ),
        capability(
            "full_file_format_on_save",
            LimitedModeCapabilityState::Denied,
            "Full-file format-on-save is disabled.",
        ),
        capability(
            "range_format_on_save",
            LimitedModeCapabilityState::Allowed,
            "Range-only format-on-save may run when reviewed by policy.",
        ),
        capability(
            "whole_file_syntax_parse",
            LimitedModeCapabilityState::Denied,
            "Full-file syntax parsing is disabled.",
        ),
        capability(
            "viewport_syntax_parse",
            LimitedModeCapabilityState::Allowed,
            "Viewport-bounded syntax parsing may run.",
        ),
        capability(
            "indexing",
            LimitedModeCapabilityState::Denied,
            "Background indexing is disabled.",
        ),
        capability(
            "background_analysis",
            LimitedModeCapabilityState::Denied,
            "Background analysis is disabled.",
        ),
        capability(
            "cursor_local_lookup",
            LimitedModeCapabilityState::Allowed,
            "On-demand cursor-local lookup remains available.",
        ),
        capability(
            "whole_file_load_into_ram",
            LimitedModeCapabilityState::Denied,
            "Whole-file loading into the normal buffer is disabled.",
        ),
        capability(
            "rich_refactor_single_file",
            LimitedModeCapabilityState::Denied,
            "Rich single-file refactor is disabled.",
        ),
        capability(
            "rich_refactor_multi_file",
            LimitedModeCapabilityState::Denied,
            "Rich multi-file refactor is disabled.",
        ),
        capability(
            "ai_apply_range",
            LimitedModeCapabilityState::Downgraded,
            "AI apply is constrained to reviewed ranges.",
        ),
        capability(
            "ai_apply_whole_file",
            LimitedModeCapabilityState::Denied,
            "Whole-file AI apply is disabled.",
        ),
        capability(
            "save_participant_range_only",
            LimitedModeCapabilityState::Allowed,
            "Range-only save participants may run when the staged-save policy allows.",
        ),
        capability(
            "save_participant_whole_file",
            LimitedModeCapabilityState::Denied,
            "Whole-file save participants are disabled.",
        ),
        capability(
            "undo_redo_history_full",
            LimitedModeCapabilityState::Downgraded,
            "Undo storage uses tighter bounded history.",
        ),
        capability(
            "accessibility_tree_viewport",
            LimitedModeCapabilityState::Allowed,
            "Viewport accessibility remains available.",
        ),
    ]
}

fn capability(
    capability_id: &str,
    state: LimitedModeCapabilityState,
    disclosure: &str,
) -> LimitedModeCapabilityRecord {
    LimitedModeCapabilityRecord {
        capability_id: capability_id.to_owned(),
        state,
        disclosure: disclosure.to_owned(),
    }
}
