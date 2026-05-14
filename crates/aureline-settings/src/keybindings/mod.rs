//! Keybinding inspection rows for settings and support surfaces.
//!
//! The settings crate does not resolve key sequences itself. It owns the
//! exported row shape settings, help, and support surfaces use after the
//! keybinding resolver has produced winning-source and conflict truth.

use serde::{Deserialize, Serialize};

/// Schema version for keybinding settings inspection records.
pub const KEYBINDING_SETTINGS_SCHEMA_VERSION: u32 = 1;

/// Source layer vocabulary mirrored from the keybinding resolver contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KeybindingSettingSourceLayer {
    /// The host platform reserved or captured the key before dispatch.
    PlatformReserved,
    /// A security response or emergency interlock blocked dispatch.
    EmergencySecurityHardBlock,
    /// Managed policy pinned, remapped, or denied the binding.
    AdminPolicyLock,
    /// A temporary mode, leader state, or scoped overlay changed meaning.
    TemporaryModeOverlay,
    /// User or profile-owned shortcut truth won.
    UserProfileBinding,
    /// Workspace-scoped shortcut recommendation won or was visible.
    WorkspaceRecommendation,
    /// Extension-contributed shortcut row was visible.
    ExtensionBinding,
    /// Shipped Aureline default binding was visible.
    CoreDefault,
    /// No source supplied an active binding.
    NotBound,
}

impl KeybindingSettingSourceLayer {
    /// Returns the stable token used by schemas and exported fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlatformReserved => "platform_reserved",
            Self::EmergencySecurityHardBlock => "emergency_security_hard_block",
            Self::AdminPolicyLock => "admin_policy_lock",
            Self::TemporaryModeOverlay => "temporary_mode_overlay",
            Self::UserProfileBinding => "user_profile_binding",
            Self::WorkspaceRecommendation => "workspace_recommendation",
            Self::ExtensionBinding => "extension_binding",
            Self::CoreDefault => "core_default",
            Self::NotBound => "not_bound",
        }
    }
}

/// One source that contributed to an inspected keybinding outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeybindingSettingSourceRecord {
    /// Resolver layer that supplied or blocked the candidate.
    pub source_layer: KeybindingSettingSourceLayer,
    /// Stable source ref for support exports.
    pub source_ref: String,
    /// Human-facing source label.
    pub source_label: String,
    /// True when this source supplied the current winner.
    pub winner: bool,
    /// Explanation for losing, shadowed, blocked, or winning state.
    pub outcome_reason_code: String,
}

/// Policy or platform narrowing attached to a shortcut outcome.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeybindingNarrowingRecord {
    /// Narrowing kind such as platform reservation or policy lock.
    pub narrowing_class: String,
    /// Stable owner or policy ref.
    pub owner_ref: String,
    /// Short export-safe explanation.
    pub explanation: String,
}

/// Conflict pointer surfaced by settings, help, migration, and support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeybindingSettingsConflictRecord {
    /// Resolver conflict-review packet ref.
    pub conflict_review_ref: String,
    /// Literal sequence under review.
    pub literal_sequence: String,
    /// Winning command id when one is resolved.
    pub winning_command_id: Option<String>,
    /// Losing command ids that remain inspectable.
    pub losing_command_ids: Vec<String>,
    /// Retained migration report or shortcut digest that introduced the conflict.
    pub migration_report_ref: Option<String>,
}

/// Settings-row projection for one command's active keybinding truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KeybindingSettingInspectionRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version for the settings projection.
    pub schema_version: u32,
    /// Stable row id used by support and help surfaces.
    pub row_id: String,
    /// Canonical command id.
    pub command_id: String,
    /// Human-facing title from the command registry.
    pub command_title: String,
    /// Current literal key sequence, or `unassigned`.
    pub current_sequence: String,
    /// Source chain considered by the resolver for this sequence.
    pub source_chain: Vec<KeybindingSettingSourceRecord>,
    /// Preview class inherited from the command descriptor.
    pub preview_class: String,
    /// Approval posture inherited from the command descriptor.
    pub approval_posture_class: String,
    /// Capability or authority class inherited from the command descriptor.
    pub authority_class: String,
    /// Resolver packet ref that backs the row.
    pub resolver_packet_ref: Option<String>,
    /// Conflict record when the row is contested or shadowed.
    pub conflict: Option<KeybindingSettingsConflictRecord>,
    /// Policy or platform narrowing that affects the row.
    pub narrowing: Option<KeybindingNarrowingRecord>,
    /// Retained migration or import report that can reopen the source delta.
    pub retained_report_ref: Option<String>,
}

impl KeybindingSettingInspectionRecord {
    /// Creates a settings inspection row with the stable record discriminator.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        row_id: impl Into<String>,
        command_id: impl Into<String>,
        command_title: impl Into<String>,
        current_sequence: impl Into<String>,
        source_chain: Vec<KeybindingSettingSourceRecord>,
        preview_class: impl Into<String>,
        approval_posture_class: impl Into<String>,
        authority_class: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: "keybinding_setting_inspection_record".to_string(),
            schema_version: KEYBINDING_SETTINGS_SCHEMA_VERSION,
            row_id: row_id.into(),
            command_id: command_id.into(),
            command_title: command_title.into(),
            current_sequence: current_sequence.into(),
            source_chain,
            preview_class: preview_class.into(),
            approval_posture_class: approval_posture_class.into(),
            authority_class: authority_class.into(),
            resolver_packet_ref: None,
            conflict: None,
            narrowing: None,
            retained_report_ref: None,
        }
    }
}
