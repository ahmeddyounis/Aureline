//! Settings-sync conflict packets and review-surface projection.
//!
//! This module is the settings-side bridge between the effective
//! resolver and sync review surfaces. It keeps conflict classes,
//! resolution paths, and policy-lock vocabulary aligned with the
//! published settings sync contract instead of letting UI consumers
//! mint local labels.

use serde::{Deserialize, Serialize};

use crate::resolver::{
    EffectiveSettingsResolver, LockReason, LockState, WriteDenialReason, WriteIntent,
};
use crate::schema::{
    PreviewClass, RedactionClass, SensitivityClass, SettingDefinition, SettingScope, SettingValue,
    SettingValueType,
};

use super::{SettingsInspectError, SETTINGS_INSPECTOR_SCHEMA_VERSION};

/// Shared contract ref consumed by settings UI, shell inspectors, CLI, and support export.
pub const SYNC_CONFLICT_SHARED_CONTRACT_REF: &str = "settings:sync_conflict_review_alpha:v1";

/// Opaque device identity used by sync conflict packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictDevice {
    /// Opaque sync device id.
    pub device_id: String,
    /// Optional user-facing label supplied by the caller after redaction.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_label: Option<String>,
}

impl SyncConflictDevice {
    /// Creates a device identity from an opaque device id.
    pub fn new(device_id: impl Into<String>) -> Self {
        Self {
            device_id: device_id.into(),
            device_label: None,
        }
    }

    /// Adds a redaction-safe display label for this device.
    pub fn with_label(mut self, device_label: impl Into<String>) -> Self {
        self.device_label = Some(device_label.into());
        self
    }
}

/// Request used to inspect an arriving settings-sync value for conflict review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictReviewRequest {
    /// Canonical setting id or registered alias.
    pub setting_id: String,
    /// Device currently resolving the local effective value.
    pub current_device: SyncConflictDevice,
    /// Device that produced the arriving synced value.
    pub conflicting_device: SyncConflictDevice,
    /// Scope targeted by the arriving synced value.
    pub conflicting_scope: SettingScope,
    /// Arriving synced value. Credential-bearing settings must pass broker handles only.
    pub conflicting_value: SettingValue,
}

/// Conflict classes from the settings sync packet vocabulary used by this inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncConflictClass {
    /// Local and synced values are equal; no write happens.
    ValueEqualNoOp,
    /// Scalar values differ.
    ScalarDivergence,
    /// Enum selections differ.
    EnumDivergence,
    /// Synced entry targeted a scope this setting or sync lane cannot accept.
    AllowedScopeMismatch,
    /// Synced entry would widen trust, egress, permissions, entitlement, or policy ceiling.
    ScopeBroadeningRefusal,
    /// Policy blocks optional sync for the entry.
    PolicyBlock,
}

impl SyncConflictClass {
    /// Returns the stable settings-sync conflict-class token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ValueEqualNoOp => "value_equal_no_op",
            Self::ScalarDivergence => "scalar_divergence",
            Self::EnumDivergence => "enum_divergence",
            Self::AllowedScopeMismatch => "allowed_scope_mismatch",
            Self::ScopeBroadeningRefusal => "scope_broadening_refusal",
            Self::PolicyBlock => "policy_block",
        }
    }
}

/// Resolution paths from the settings sync packet vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncConflictResolutionPath {
    /// Retain the local value and record the refusal.
    KeepLocal,
    /// Adopt the synced value through the settings write-intent path.
    KeepSynced,
    /// Open a field-aware merge preview before apply.
    MergePreview,
    /// Route through rollback-checkpoint or approval review before apply.
    RollbackFriendlyReview,
    /// Terminal refusal of the packet.
    Decline,
}

impl SyncConflictResolutionPath {
    /// Returns the stable resolution-path token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeepLocal => "keep_local",
            Self::KeepSynced => "keep_synced",
            Self::MergePreview => "merge_preview",
            Self::RollbackFriendlyReview => "rollback_friendly_review",
            Self::Decline => "decline",
        }
    }
}

/// Resolution lifecycle states from the settings sync packet vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncConflictResolutionState {
    /// Initial unresolved state.
    Pending,
    /// A preview has been shown.
    Previewed,
    /// The user acknowledged the preview.
    Acknowledged,
    /// The conflict reached a resolved state.
    Resolved,
    /// The conflict was declined.
    Declined,
    /// The acknowledgement window expired.
    Expired,
    /// The producer withdrew the conflicting value before apply.
    Withdrawn,
}

impl SyncConflictResolutionState {
    /// Returns the stable resolution-state token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Previewed => "previewed",
            Self::Acknowledged => "acknowledged",
            Self::Resolved => "resolved",
            Self::Declined => "declined",
            Self::Expired => "expired",
            Self::Withdrawn => "withdrawn",
        }
    }
}

/// Redaction-aware value preview carried by a conflict packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictValuePreview {
    /// Preview kind from the settings sync value-preview vocabulary.
    pub value_preview_kind: String,
    /// Redacted or literal preview payload when the redaction class allows it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_preview: Option<serde_json::Value>,
    /// Class label used when the payload body is not exportable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub class_label: Option<String>,
    /// Broker alias or opaque handle for credential-bearing settings.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub credential_alias_ref: Option<String>,
}

/// One field-aware diff row inside a settings-sync conflict packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictFieldDiff {
    /// Field path for the changed value. Scalar settings use `$`.
    pub field_path: String,
    /// Change-kind token from the settings sync packet.
    pub change_kind: String,
    /// Local value preview.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_preview: Option<SyncConflictValuePreview>,
    /// Conflicting value preview.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conflicting_preview: Option<SyncConflictValuePreview>,
}

/// Field-aware delta between the current and conflicting setting values.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictDelta {
    /// Delta-kind token from the settings sync packet.
    pub delta_kind: String,
    /// Per-field diff rows.
    pub field_diff_rows: Vec<SyncConflictFieldDiff>,
    /// Redaction-safe summary.
    pub summary: String,
}

/// Canonical settings-sync conflict packet projected from the resolver.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Inspector schema version.
    pub schema_version: u32,
    /// Shared contract ref for every consumer of this packet.
    pub shared_contract_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Canonical setting id.
    pub setting_id: String,
    /// Device currently resolving the local effective value.
    pub current_device: SyncConflictDevice,
    /// Device that produced the arriving synced value.
    pub conflicting_device: SyncConflictDevice,
    /// Scope targeted by the arriving synced value.
    pub conflicting_scope: String,
    /// Lock-state token from the effective resolver.
    pub lock_state: LockState,
    /// Lock-reason token from the effective resolver.
    pub lock_reason: LockReason,
    /// Typed conflict class.
    pub conflict_class: SyncConflictClass,
    /// Current effective value preview.
    pub current_value: SyncConflictValuePreview,
    /// Arriving synced value preview.
    pub conflicting_value: SyncConflictValuePreview,
    /// Field-aware delta.
    pub conflict_delta: SyncConflictDelta,
    /// Resolution paths the packet offers.
    pub offered_resolution_paths: Vec<SyncConflictResolutionPath>,
    /// Recommended resolution path for the review surface.
    pub recommended_resolution_path: SyncConflictResolutionPath,
    /// True when the recommended path can be applied without human review.
    pub auto_resolvable: bool,
    /// Resolution lifecycle state.
    pub resolution_state: SyncConflictResolutionState,
    /// Setting redaction class applied to previews.
    pub redaction_class: String,
    /// Policy-lock source ref when a policy ceiling is active.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_lock_ref: Option<String>,
}

impl SyncConflictPacket {
    /// Returns true when this packet can be resolved without human review.
    pub const fn can_auto_resolve(&self) -> bool {
        self.auto_resolvable
    }
}

/// One action row rendered by a settings-sync conflict review surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictReviewAction {
    /// Resolution path represented by this action.
    pub resolution_path: SyncConflictResolutionPath,
    /// True when the action is currently available.
    pub available: bool,
    /// True when this action is the packet recommendation.
    pub recommended: bool,
    /// Canonical disabled reason token when unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
}

/// Review-surface projection for shell, settings UI, CLI, and support exporters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyncConflictReviewSurface {
    /// Record discriminator.
    pub record_kind: String,
    /// Inspector schema version.
    pub schema_version: u32,
    /// Source conflict packet id.
    pub source_packet_ref: String,
    /// Shared contract ref for every consumer of this surface.
    pub shared_contract_ref: String,
    /// Canonical setting id.
    pub setting_id: String,
    /// Current device id.
    pub current_device_id: String,
    /// Conflicting device id.
    pub conflicting_device_id: String,
    /// Scope targeted by the arriving synced value.
    pub conflicting_scope: String,
    /// Lock-state token from the effective resolver.
    pub lock_state: String,
    /// Typed conflict class token.
    pub conflict_class: String,
    /// Recommended resolution-path token.
    pub recommended_resolution_path: String,
    /// True when the recommendation may apply without human review.
    pub auto_resolvable: bool,
    /// Action rows for the complete settings-sync resolution path set.
    pub action_rows: Vec<SyncConflictReviewAction>,
}

/// Builds a settings-sync conflict packet if the arriving value conflicts.
pub fn inspect_sync_conflict(
    resolver: &EffectiveSettingsResolver,
    request: SyncConflictReviewRequest,
) -> Result<Option<SyncConflictPacket>, SettingsInspectError> {
    let def = resolver
        .registry()
        .resolve_definition(&request.setting_id)
        .ok_or_else(|| SettingsInspectError::UnknownSetting {
            setting_id: request.setting_id.clone(),
        })?;
    let effective =
        resolver
            .resolve(&def.setting_id)
            .map_err(|err| SettingsInspectError::ResolveFailed {
                setting_id: def.setting_id.clone(),
                detail: err.to_string(),
            })?;

    if effective.value == request.conflicting_value
        && is_syncable_scope(request.conflicting_scope)
        && def.allows_scope(request.conflicting_scope)
    {
        return Ok(None);
    }

    let current_value = value_preview(def, &effective.value);
    let conflicting_value = value_preview(def, &request.conflicting_value);
    let target_scope_allowed =
        is_syncable_scope(request.conflicting_scope) && def.allows_scope(request.conflicting_scope);
    let mut probe = resolver.clone();
    let write_outcome = target_scope_allowed.then(|| {
        probe.attempt_write(
            &def.setting_id,
            request.conflicting_scope,
            request.conflicting_value.clone(),
        )
    });

    let policy_denied = write_outcome
        .as_ref()
        .and_then(|outcome| outcome.denial_reason.as_ref())
        .is_some_and(|reason| {
            matches!(
                reason,
                WriteDenialReason::PolicyLocked | WriteDenialReason::PolicyConstrainedValue
            )
        });
    let write_denied = write_outcome
        .as_ref()
        .is_some_and(|outcome| matches!(outcome.verdict, WriteIntent::Denied));

    let conflict_class = if !target_scope_allowed {
        SyncConflictClass::AllowedScopeMismatch
    } else if policy_denied {
        SyncConflictClass::ScopeBroadeningRefusal
    } else if write_denied {
        SyncConflictClass::PolicyBlock
    } else {
        divergence_class(def)
    };

    let (recommended_resolution_path, offered_resolution_paths, auto_resolvable) =
        recommendation_for(def.preview_class, conflict_class, policy_denied);
    let packet = SyncConflictPacket {
        record_kind: "sync_conflict_packet".to_owned(),
        schema_version: SETTINGS_INSPECTOR_SCHEMA_VERSION,
        shared_contract_ref: SYNC_CONFLICT_SHARED_CONTRACT_REF.to_owned(),
        packet_id: packet_id(
            &def.setting_id,
            request.conflicting_scope,
            &request.current_device.device_id,
            &request.conflicting_device.device_id,
        ),
        setting_id: def.setting_id.clone(),
        current_device: request.current_device,
        conflicting_device: request.conflicting_device,
        conflicting_scope: request.conflicting_scope.as_str().to_owned(),
        lock_state: effective.lock_state,
        lock_reason: effective.lock_reason,
        conflict_class,
        conflict_delta: conflict_delta(
            def,
            &current_value,
            &conflicting_value,
            conflict_class,
            policy_denied,
        ),
        current_value,
        conflicting_value,
        offered_resolution_paths,
        recommended_resolution_path,
        auto_resolvable,
        resolution_state: SyncConflictResolutionState::Pending,
        redaction_class: def.redaction_class.as_str().to_owned(),
        policy_lock_ref: policy_lock_ref(def, effective.policy_ceiling_active),
    };
    Ok(Some(packet))
}

/// Builds the review-surface projection for a settings-sync conflict packet.
pub fn project_sync_conflict_review_surface(
    packet: &SyncConflictPacket,
) -> SyncConflictReviewSurface {
    let offered = &packet.offered_resolution_paths;
    let disabled_reason = disabled_reason_token(packet);
    let action_rows = [
        SyncConflictResolutionPath::KeepLocal,
        SyncConflictResolutionPath::KeepSynced,
        SyncConflictResolutionPath::MergePreview,
        SyncConflictResolutionPath::RollbackFriendlyReview,
        SyncConflictResolutionPath::Decline,
    ]
    .into_iter()
    .map(|resolution_path| {
        let available = offered.iter().any(|offered| *offered == resolution_path);
        SyncConflictReviewAction {
            resolution_path,
            available,
            recommended: resolution_path == packet.recommended_resolution_path,
            disabled_reason: (!available).then(|| disabled_reason.clone()),
        }
    })
    .collect();

    SyncConflictReviewSurface {
        record_kind: "sync_conflict_review_surface".to_owned(),
        schema_version: SETTINGS_INSPECTOR_SCHEMA_VERSION,
        source_packet_ref: packet.packet_id.clone(),
        shared_contract_ref: packet.shared_contract_ref.clone(),
        setting_id: packet.setting_id.clone(),
        current_device_id: packet.current_device.device_id.clone(),
        conflicting_device_id: packet.conflicting_device.device_id.clone(),
        conflicting_scope: packet.conflicting_scope.clone(),
        lock_state: packet.lock_state.as_str().to_owned(),
        conflict_class: packet.conflict_class.as_str().to_owned(),
        recommended_resolution_path: packet.recommended_resolution_path.as_str().to_owned(),
        auto_resolvable: packet.auto_resolvable,
        action_rows,
    }
}

fn is_syncable_scope(scope: SettingScope) -> bool {
    matches!(
        scope,
        SettingScope::UserGlobal | SettingScope::LanguageOverride
    )
}

fn value_preview(def: &SettingDefinition, value: &SettingValue) -> SyncConflictValuePreview {
    if matches!(def.sensitivity_class, SensitivityClass::CredentialReference) {
        return SyncConflictValuePreview {
            value_preview_kind: "credential_alias_only".to_owned(),
            value_preview: None,
            class_label: Some(def.sensitivity_class.as_str().to_owned()),
            credential_alias_ref: credential_alias_ref(value),
        };
    }

    match def.redaction_class {
        RedactionClass::None | RedactionClass::UiStringOnly => SyncConflictValuePreview {
            value_preview_kind: "literal_value".to_owned(),
            value_preview: Some(value.to_json()),
            class_label: None,
            credential_alias_ref: None,
        },
        RedactionClass::RedactValuePreserveShape => SyncConflictValuePreview {
            value_preview_kind: "redacted_shape_preserved".to_owned(),
            value_preview: Some(serde_json::json!({
                "kind": def.value_type.kind_token(),
            })),
            class_label: None,
            credential_alias_ref: None,
        },
        RedactionClass::RedactToClassLabel => SyncConflictValuePreview {
            value_preview_kind: "class_label_only".to_owned(),
            value_preview: None,
            class_label: Some(def.sensitivity_class.as_str().to_owned()),
            credential_alias_ref: None,
        },
        RedactionClass::ExcludeFromExport => SyncConflictValuePreview {
            value_preview_kind: "excluded_from_export".to_owned(),
            value_preview: None,
            class_label: Some(def.sensitivity_class.as_str().to_owned()),
            credential_alias_ref: None,
        },
    }
}

fn credential_alias_ref(value: &SettingValue) -> Option<String> {
    match value {
        SettingValue::String(value) if looks_like_handle(value) => Some(value.clone()),
        SettingValue::String(_) => Some("credential_alias_ref_redacted".to_owned()),
        _ => None,
    }
}

fn looks_like_handle(value: &str) -> bool {
    [
        "cred:",
        "credential:",
        "credential_alias:",
        "handle:",
        "secret-handle:",
        "vault:",
    ]
    .iter()
    .any(|prefix| value.starts_with(prefix))
}

fn divergence_class(def: &SettingDefinition) -> SyncConflictClass {
    match def.value_type {
        SettingValueType::Enum { .. } => SyncConflictClass::EnumDivergence,
        _ => SyncConflictClass::ScalarDivergence,
    }
}

fn recommendation_for(
    preview_class: PreviewClass,
    conflict_class: SyncConflictClass,
    policy_denied: bool,
) -> (
    SyncConflictResolutionPath,
    Vec<SyncConflictResolutionPath>,
    bool,
) {
    if conflict_class == SyncConflictClass::AllowedScopeMismatch {
        return (
            SyncConflictResolutionPath::KeepLocal,
            vec![
                SyncConflictResolutionPath::KeepLocal,
                SyncConflictResolutionPath::Decline,
            ],
            false,
        );
    }

    if policy_denied {
        return (
            SyncConflictResolutionPath::MergePreview,
            vec![
                SyncConflictResolutionPath::KeepLocal,
                SyncConflictResolutionPath::MergePreview,
                SyncConflictResolutionPath::Decline,
            ],
            false,
        );
    }

    if preview_class.requires_checkpoint() || preview_class.requires_approval() {
        return (
            SyncConflictResolutionPath::RollbackFriendlyReview,
            vec![
                SyncConflictResolutionPath::KeepLocal,
                SyncConflictResolutionPath::RollbackFriendlyReview,
                SyncConflictResolutionPath::Decline,
            ],
            false,
        );
    }

    (
        SyncConflictResolutionPath::KeepSynced,
        vec![
            SyncConflictResolutionPath::KeepLocal,
            SyncConflictResolutionPath::KeepSynced,
            SyncConflictResolutionPath::Decline,
        ],
        !preview_class.requires_preview(),
    )
}

fn conflict_delta(
    def: &SettingDefinition,
    current_value: &SyncConflictValuePreview,
    conflicting_value: &SyncConflictValuePreview,
    conflict_class: SyncConflictClass,
    policy_denied: bool,
) -> SyncConflictDelta {
    let delta_kind = match conflict_class {
        SyncConflictClass::EnumDivergence => "enum_reselect",
        SyncConflictClass::ValueEqualNoOp => "no_change",
        _ => "scalar_replace",
    };
    let change_kind = if policy_denied { "blocked" } else { "replaced" };
    let summary = if policy_denied {
        "Arriving synced value conflicts with the active policy lock."
    } else if conflict_class == SyncConflictClass::AllowedScopeMismatch {
        "Arriving synced value targets a scope this setting cannot sync."
    } else {
        "Arriving synced value differs from the current effective value."
    };
    let field_diff_rows = if conflict_class == SyncConflictClass::ValueEqualNoOp {
        Vec::new()
    } else {
        vec![SyncConflictFieldDiff {
            field_path: "$".to_owned(),
            change_kind: change_kind.to_owned(),
            current_preview: Some(current_value.clone()),
            conflicting_preview: Some(conflicting_value.clone()),
        }]
    };

    SyncConflictDelta {
        delta_kind: if matches!(
            def.redaction_class,
            RedactionClass::RedactValuePreserveShape
                | RedactionClass::RedactToClassLabel
                | RedactionClass::ExcludeFromExport
        ) {
            "redacted_structural".to_owned()
        } else {
            delta_kind.to_owned()
        },
        field_diff_rows,
        summary: summary.to_owned(),
    }
}

fn policy_lock_ref(def: &SettingDefinition, policy_ceiling_active: bool) -> Option<String> {
    policy_ceiling_active.then(|| format!("policy:{}", def.setting_id))
}

fn disabled_reason_token(packet: &SyncConflictPacket) -> String {
    if matches!(
        packet.lock_state,
        LockState::PolicyLocked | LockState::PolicyConstrained
    ) {
        return packet.lock_state.as_str().to_owned();
    }
    if packet.conflict_class == SyncConflictClass::AllowedScopeMismatch {
        return SyncConflictClass::AllowedScopeMismatch.as_str().to_owned();
    }
    "not_offered_for_conflict".to_owned()
}

fn packet_id(
    setting_id: &str,
    scope: SettingScope,
    current_device_id: &str,
    conflicting_device_id: &str,
) -> String {
    format!(
        "settings-sync-conflict:{setting_id}:{}:{current_device_id}:{conflicting_device_id}",
        scope.as_str()
    )
}
