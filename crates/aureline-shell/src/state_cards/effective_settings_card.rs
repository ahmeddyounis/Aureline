//! Effective-settings inspector card: shell consumer of the
//! [`aureline_settings::EffectiveSettingsResolver`].
//!
//! The card is the M1 minimal slice that gives shell surfaces a
//! single, projectable record for "what is the current value of
//! setting X, where did it come from, what is shadowed, and what
//! happens if I write Y at scope Z?". It is the dual of the
//! workspace-readiness chip: same projection idea, but for settings
//! truth instead of workspace truth.
//!
//! Why a card on top of the resolver:
//!
//! - settings UI, settings-readiness chips, the support/export
//!   surface, and the "Explain why" affordance all need the same
//!   projection of `effective_value -> shadow_chain -> lock_state`;
//!   doing the projection in one place keeps them honest;
//! - the locked-write attempt projection is the failure-drill proof
//!   surface: a denied write returns the typed denial reason and
//!   the shadow chain that would have applied, never a generic
//!   "could not save" string;
//! - the card is serializable so support exports and proof artifacts
//!   can quote the same record the shell renders.

use aureline_settings::{
    EffectiveSettingsResolver, EffectiveValue, SettingScope, SettingValue, ShadowChainEntry,
    WriteAttemptOutcome, WriteDenialReason,
};
use serde::{Deserialize, Serialize};

const EFFECTIVE_SETTINGS_CARD_SCHEMA_VERSION: u32 = 1;

/// One shadow-chain row projected into a shell-renderable shape.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveSettingsShadowRow {
    pub scope: String,
    pub source_label: String,
    pub value_preview: String,
    pub relation: String,
}

impl EffectiveSettingsShadowRow {
    fn from_entry(entry: &ShadowChainEntry) -> Self {
        Self {
            scope: entry.scope.as_str().to_owned(),
            source_label: entry.source_label.clone(),
            value_preview: entry.value_preview.clone(),
            relation: entry.relation.as_str().to_owned(),
        }
    }
}

/// Effective-settings inspector card. One per `setting_id` view.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveSettingsCardRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub setting_id: String,
    pub value_preview: String,
    pub winning_scope: String,
    pub source_label: String,
    pub lock_state: String,
    pub lock_reason: String,
    pub restart_posture: String,
    pub policy_ceiling_active: bool,
    pub shadow_chain: Vec<EffectiveSettingsShadowRow>,
    pub summary_line: String,
}

impl EffectiveSettingsCardRecord {
    /// Materialize a card from an [`EffectiveValue`].
    pub fn from_effective(effective: &EffectiveValue) -> Self {
        let shadow_chain: Vec<EffectiveSettingsShadowRow> = effective
            .shadow_chain
            .iter()
            .map(EffectiveSettingsShadowRow::from_entry)
            .collect();
        let summary_line = format_summary_line(effective);
        Self {
            record_kind: "effective_settings_card_record".to_owned(),
            schema_version: EFFECTIVE_SETTINGS_CARD_SCHEMA_VERSION,
            setting_id: effective.setting_id.clone(),
            value_preview: effective.value.preview(),
            winning_scope: effective.winning_scope.as_str().to_owned(),
            source_label: effective.source_label.clone(),
            lock_state: effective.lock_state.as_str().to_owned(),
            lock_reason: effective.lock_reason.as_str().to_owned(),
            restart_posture: effective.restart_posture.as_str().to_owned(),
            policy_ceiling_active: effective.policy_ceiling_active,
            shadow_chain,
            summary_line,
        }
    }
}

/// Materialize a card directly from a resolver. Returns `None` when
/// the resolver does not know the setting; surfaces MUST handle that
/// case by routing the user to the schema registry rather than
/// rendering an empty card.
pub fn materialize_effective_settings_card(
    resolver: &EffectiveSettingsResolver,
    setting_id: &str,
) -> Option<EffectiveSettingsCardRecord> {
    resolver
        .resolve(setting_id)
        .ok()
        .map(|effective| EffectiveSettingsCardRecord::from_effective(&effective))
}

/// One-line summary of a setting's effective value, suitable for the
/// title bar / status surfaces. Surfaces MAY add their own chrome
/// but MUST NOT change the lock or restart vocabulary.
fn format_summary_line(effective: &EffectiveValue) -> String {
    let mut out = format!(
        "{setting_id} = {value} ({scope})",
        setting_id = effective.setting_id,
        value = effective.value.preview(),
        scope = effective.winning_scope.as_str(),
    );
    if !matches!(effective.lock_state, aureline_settings::LockState::Open) {
        out.push_str(&format!(" [{}]", effective.lock_state.as_str()));
    }
    if effective.policy_ceiling_active {
        out.push_str(" — policy ceiling active");
    }
    out
}

/// Shell-renderable projection of a write attempt. Returned by the
/// failure-drill flow when the user tries to write a locked value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockedWriteReviewRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub setting_id: String,
    pub target_scope: String,
    pub proposed_value_preview: String,
    pub verdict: String,
    pub denial_reason_code: Option<String>,
    pub denial_reason_message: Option<String>,
    pub effective_after: Option<EffectiveSettingsCardRecord>,
    pub effective_before: Option<EffectiveSettingsCardRecord>,
}

impl LockedWriteReviewRecord {
    pub fn from_outcome(outcome: &WriteAttemptOutcome) -> Self {
        Self {
            record_kind: "locked_write_review_record".to_owned(),
            schema_version: EFFECTIVE_SETTINGS_CARD_SCHEMA_VERSION,
            setting_id: outcome.setting_id.clone(),
            target_scope: outcome.target_scope.as_str().to_owned(),
            proposed_value_preview: outcome.proposed_value.preview(),
            verdict: outcome.verdict.as_str().to_owned(),
            denial_reason_code: outcome
                .denial_reason
                .as_ref()
                .map(|r| r.code_token().to_owned()),
            denial_reason_message: outcome.denial_reason.as_ref().map(format_denial_message),
            effective_after: outcome
                .effective_after
                .as_ref()
                .map(EffectiveSettingsCardRecord::from_effective),
            effective_before: outcome
                .effective_before
                .as_ref()
                .map(EffectiveSettingsCardRecord::from_effective),
        }
    }
}

fn format_denial_message(reason: &WriteDenialReason) -> String {
    match reason {
        WriteDenialReason::UnknownSetting { setting_id } => {
            format!("Setting {setting_id:?} is not registered.")
        }
        WriteDenialReason::ScopeNotAllowed => {
            "This setting cannot be written at the requested scope.".to_owned()
        }
        WriteDenialReason::ScopeBroadeningWouldWidenTrust => {
            "The proposed write would broaden trust, egress, or authority beyond the selected scope."
                .to_owned()
        }
        WriteDenialReason::PolicyLocked => {
            "Admin policy pins this value; the proposed write was refused.".to_owned()
        }
        WriteDenialReason::PolicyConstrainedValue => {
            "Admin policy constrains the allowed values; the proposed value is outside the set."
                .to_owned()
        }
        WriteDenialReason::CapabilityDependencyUnmet => {
            "A declared capability dependency is not currently satisfied.".to_owned()
        }
        WriteDenialReason::PreviewRequiredNotAcknowledged => {
            "A required settings preview has not been acknowledged.".to_owned()
        }
        WriteDenialReason::RollbackCheckpointNotCreated => {
            "A required rollback checkpoint has not been created.".to_owned()
        }
        WriteDenialReason::ApprovalTicketMissing => {
            "A required approval ticket is missing.".to_owned()
        }
        WriteDenialReason::RestartRequiredNotAcknowledged => {
            "The declared restart posture has not been acknowledged.".to_owned()
        }
        WriteDenialReason::ValidationFailed { detail } => {
            format!("Proposed value failed validation: {detail}")
        }
        WriteDenialReason::RetiredSetting => {
            "This setting is retired and no longer accepts writes.".to_owned()
        }
        WriteDenialReason::ManagedModeOnly => {
            "This setting can only be changed by a managed authority.".to_owned()
        }
        WriteDenialReason::ReadOnlySurface => {
            "This surface can inspect the setting but cannot mutate it.".to_owned()
        }
    }
}

/// Project a write attempt and return the renderable review record
/// without mutating the resolver. Surfaces use this to preview a
/// locked-write outcome before they call `attempt_write` for real.
pub fn preview_locked_write(
    resolver: &EffectiveSettingsResolver,
    setting_id: &str,
    target_scope: SettingScope,
    value: SettingValue,
) -> LockedWriteReviewRecord {
    // Clone the resolver so the preview path never mutates the live
    // overlay stack. The preview is a read-only inspection; callers
    // commit by invoking the resolver's `attempt_write` directly.
    let mut probe = resolver.clone();
    let outcome = probe.attempt_write(setting_id, target_scope, value);
    LockedWriteReviewRecord::from_outcome(&outcome)
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_settings::{
        EffectiveSettingsResolver, PolicyConstraint, SchemaRegistry, ScopeOverlay, SettingScope,
        SettingValue,
    };

    fn resolver_with_user_overlay() -> EffectiveSettingsResolver {
        let mut resolver = EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog());
        let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
        user.set_value("editor.tab_size", SettingValue::Integer(8));
        resolver.set_overlay(user).unwrap();
        resolver
    }

    fn resolver_with_policy_lock() -> EffectiveSettingsResolver {
        let mut resolver = EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog());
        let mut policy =
            ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle v3");
        policy.set_policy_constraint(
            "security.ai.egress_policy",
            PolicyConstraint::SingleValue {
                value: SettingValue::String("approved_hosted_providers_only".into()),
            },
        );
        resolver.set_overlay(policy).unwrap();
        resolver
    }

    #[test]
    fn card_quotes_winning_scope_and_shadow_chain() {
        let resolver = resolver_with_user_overlay();
        let card = materialize_effective_settings_card(&resolver, "editor.tab_size").unwrap();
        assert_eq!(card.setting_id, "editor.tab_size");
        assert_eq!(card.value_preview, "8");
        assert_eq!(card.winning_scope, "user_global");
        assert_eq!(card.lock_state, "open");
        assert_eq!(card.lock_reason, "none");
        assert_eq!(card.restart_posture, "no_restart");
        assert!(!card.policy_ceiling_active);
        assert!(card
            .shadow_chain
            .iter()
            .any(|row| row.scope == "built_in_default" && row.relation == "shadowed"));
        assert!(card
            .shadow_chain
            .iter()
            .any(|row| row.scope == "user_global" && row.relation == "winner"));
    }

    #[test]
    fn unknown_setting_returns_no_card() {
        let resolver = resolver_with_user_overlay();
        assert!(materialize_effective_settings_card(&resolver, "does.not.exist").is_none());
    }

    #[test]
    fn policy_lock_card_surfaces_lock_state_and_ceiling_flag() {
        let resolver = resolver_with_policy_lock();
        let card =
            materialize_effective_settings_card(&resolver, "security.ai.egress_policy").unwrap();
        assert_eq!(card.winning_scope, "admin_policy_narrowing");
        assert_eq!(card.lock_state, "policy_locked");
        assert_eq!(card.lock_reason, "policy_locked");
        assert!(card.policy_ceiling_active);
        assert!(card.summary_line.contains("policy ceiling active"));
        assert!(card
            .shadow_chain
            .iter()
            .any(|row| row.relation == "winner" && row.scope == "admin_policy_narrowing"));
    }

    #[test]
    fn preview_locked_write_quotes_typed_denial_reason() {
        let resolver = resolver_with_policy_lock();
        let review = preview_locked_write(
            &resolver,
            "security.ai.egress_policy",
            SettingScope::UserGlobal,
            SettingValue::String("any_hosted_provider".into()),
        );
        assert_eq!(review.verdict, "denied");
        assert_eq!(review.denial_reason_code.as_deref(), Some("policy_locked"));
        assert!(review
            .denial_reason_message
            .as_deref()
            .unwrap()
            .contains("Admin policy"));
        let after = review.effective_after.unwrap();
        assert!(after.policy_ceiling_active);
    }

    #[test]
    fn preview_does_not_mutate_resolver() {
        let resolver = resolver_with_policy_lock();
        let original = resolver.resolve("security.ai.egress_policy").unwrap();
        let _ = preview_locked_write(
            &resolver,
            "security.ai.egress_policy",
            SettingScope::UserGlobal,
            SettingValue::String("disabled".into()),
        );
        let after_preview = resolver.resolve("security.ai.egress_policy").unwrap();
        assert_eq!(original, after_preview);
    }

    #[test]
    fn preview_for_validation_failure_returns_typed_message() {
        let resolver = EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog());
        let review = preview_locked_write(
            &resolver,
            "editor.tab_size",
            SettingScope::UserGlobal,
            SettingValue::Integer(99),
        );
        assert_eq!(review.verdict, "denied");
        assert_eq!(
            review.denial_reason_code.as_deref(),
            Some("validation_failed")
        );
    }

    #[test]
    fn card_record_round_trips_through_serde_json() {
        let resolver = resolver_with_user_overlay();
        let card = materialize_effective_settings_card(&resolver, "editor.tab_size").unwrap();
        let payload = serde_json::to_string(&card).unwrap();
        let restored: EffectiveSettingsCardRecord = serde_json::from_str(&payload).unwrap();
        assert_eq!(card, restored);
    }
}
