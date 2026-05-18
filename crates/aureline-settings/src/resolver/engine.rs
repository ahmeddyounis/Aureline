//! Effective-settings resolver: precedence engine + locked-write
//! flow.
//!
//! The resolver owns one schema registry and a stack of
//! per-scope overlays. Given a `setting_id`, it returns the
//! [`EffectiveValue`] including the shadow chain and any active
//! policy ceiling. Given a write attempt, it returns a typed
//! verdict (`Allowed` or `Denied`) with the denial reason and the
//! shadow chain that would have applied.
//!
//! The resolver is intentionally file-format-agnostic: callers
//! load JSON/JSONC artifacts and hand the parsed values to
//! [`ScopeOverlay::set_value`]. The resolver state is exportable
//! through [`EffectiveSettingsResolver::export_state`] so settings
//! state remains "file-based and exportable without requiring sync
//! or a managed account".

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::schema::{
    CapabilityDependency, LifecycleLabel, PreviewClass, RedactionClass, RestartPosture,
    SchemaRegistry, SettingDefinition, SettingScope, SettingValue, ValueValidationError,
};

use super::effective::{
    write_denial_token, write_intent_token, EffectiveCapabilityDependency, EffectiveControlStack,
    EffectiveLastWritten, EffectiveSettingRecord, EffectiveValue, ShadowChainEntry, ShadowRelation,
};
use super::lock::{LockReason, LockState, WriteDenialReason, WriteIntent};

const EFFECTIVE_SETTINGS_SCHEMA_VERSION: u32 = 1;
const SETTING_DEFINITION_SCHEMA_VERSION: &str = "settings_definition:v1";

/// Per-scope overlay carrying the values the resolver should layer
/// at that scope. Overlays own the human-readable `source_label`
/// surfaces render in the source pill.
///
/// An admin-policy overlay may also carry a `policy_constraint` per
/// `setting_id` that names the allowed value set the policy admits.
/// When `policy_constraint` is present, the resolver treats the
/// overlay as a ceiling and either pins the value (single-element
/// allow set) or constrains it (multi-element allow set).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeOverlay {
    pub scope: SettingScope,
    pub source_label: String,
    #[serde(default)]
    pub values: BTreeMap<String, SettingValue>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub policy_constraint: BTreeMap<String, PolicyConstraint>,
}

/// Allowed value set imposed by an admin-policy overlay. A
/// `single_value` constraint pins the setting to exactly one
/// value; a `narrowed_set` constraint admits only values in the
/// listed set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PolicyConstraint {
    SingleValue { value: SettingValue },
    NarrowedSet { allowed: Vec<SettingValue> },
}

/// Resolver-observed state for one declared capability dependency.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityState {
    /// True when the dependency is currently satisfied.
    pub satisfied: bool,
    /// Redaction-safe state label supplied by the capability owner.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_state: Option<String>,
}

impl CapabilityState {
    /// Builds a state row for a satisfied dependency.
    pub fn satisfied(observed_state: impl Into<String>) -> Self {
        Self {
            satisfied: true,
            observed_state: Some(observed_state.into()),
        }
    }

    /// Builds a state row for an unmet dependency.
    pub fn unmet(observed_state: impl Into<String>) -> Self {
        Self {
            satisfied: false,
            observed_state: Some(observed_state.into()),
        }
    }
}

impl PolicyConstraint {
    fn admits(&self, value: &SettingValue) -> bool {
        match self {
            Self::SingleValue { value: pinned } => pinned == value,
            Self::NarrowedSet { allowed } => allowed.iter().any(|a| a == value),
        }
    }

    fn first_allowed(&self) -> &SettingValue {
        match self {
            Self::SingleValue { value } => value,
            Self::NarrowedSet { allowed } => &allowed[0],
        }
    }

    fn pins_value(&self) -> bool {
        matches!(self, Self::SingleValue { .. })
    }
}

impl ScopeOverlay {
    pub fn new(scope: SettingScope, source_label: impl Into<String>) -> Self {
        Self {
            scope,
            source_label: source_label.into(),
            values: BTreeMap::new(),
            policy_constraint: BTreeMap::new(),
        }
    }

    /// Set a value for `setting_id` at this overlay.
    pub fn set_value(&mut self, setting_id: impl Into<String>, value: SettingValue) {
        self.values.insert(setting_id.into(), value);
    }

    /// Set a policy constraint on `setting_id`. Only meaningful on an
    /// `AdminPolicyNarrowing` overlay; the engine refuses to apply
    /// constraints from other scopes.
    pub fn set_policy_constraint(
        &mut self,
        setting_id: impl Into<String>,
        constraint: PolicyConstraint,
    ) {
        self.policy_constraint.insert(setting_id.into(), constraint);
    }
}

/// Result of [`EffectiveSettingsResolver::attempt_write`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteAttemptOutcome {
    pub setting_id: String,
    pub target_scope: SettingScope,
    pub proposed_value: SettingValue,
    pub verdict: WriteIntent,
    pub denial_reason: Option<WriteDenialReason>,
    pub effective_after: Option<EffectiveValue>,
    pub effective_before: Option<EffectiveValue>,
}

/// Errors returned by [`EffectiveSettingsResolver`] for non-write
/// operations. Write attempts return a typed
/// [`WriteAttemptOutcome`] instead of an error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    UnknownSetting {
        setting_id: String,
    },
    InvalidOverlayValue {
        setting_id: String,
        scope: SettingScope,
        detail: String,
    },
    PolicyConstraintFromNonPolicyScope {
        scope: SettingScope,
        setting_id: String,
    },
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownSetting { setting_id } => {
                write!(f, "setting_id {setting_id:?} is not registered")
            }
            Self::InvalidOverlayValue {
                setting_id,
                scope,
                detail,
            } => write!(
                f,
                "overlay value for {setting_id:?} at {} is invalid: {detail}",
                scope.as_str()
            ),
            Self::PolicyConstraintFromNonPolicyScope { scope, setting_id } => write!(
                f,
                "scope {} cannot carry a policy constraint for {setting_id:?}",
                scope.as_str()
            ),
        }
    }
}

impl std::error::Error for ResolveError {}

/// Effective-settings resolver. Owns the schema registry and a
/// stack of overlays.
#[derive(Debug, Clone)]
pub struct EffectiveSettingsResolver {
    registry: SchemaRegistry,
    overlays: BTreeMap<SettingScope, ScopeOverlay>,
    capability_states: BTreeMap<String, CapabilityState>,
}

impl EffectiveSettingsResolver {
    /// Build a resolver with `registry`. Overlays are added through
    /// [`Self::set_overlay`].
    pub fn new(registry: SchemaRegistry) -> Self {
        Self {
            registry,
            overlays: BTreeMap::new(),
            capability_states: BTreeMap::new(),
        }
    }

    /// Borrow the underlying schema registry.
    pub fn registry(&self) -> &SchemaRegistry {
        &self.registry
    }

    /// Replace (or insert) the overlay for `overlay.scope`. Returns
    /// the previous overlay, if any.
    ///
    /// Every value in the overlay is validated against the registered
    /// definition; an invalid value is rejected with
    /// [`ResolveError::InvalidOverlayValue`].
    pub fn set_overlay(
        &mut self,
        overlay: ScopeOverlay,
    ) -> Result<Option<ScopeOverlay>, ResolveError> {
        for (setting_id, value) in &overlay.values {
            let def = self.registry.definition(setting_id).ok_or_else(|| {
                ResolveError::UnknownSetting {
                    setting_id: setting_id.clone(),
                }
            })?;
            if let Err(err) = def.validate_value(value) {
                return Err(ResolveError::InvalidOverlayValue {
                    setting_id: setting_id.clone(),
                    scope: overlay.scope,
                    detail: err.to_string(),
                });
            }
        }
        if !overlay.policy_constraint.is_empty()
            && overlay.scope != SettingScope::AdminPolicyNarrowing
        {
            let setting_id = overlay
                .policy_constraint
                .keys()
                .next()
                .cloned()
                .unwrap_or_default();
            return Err(ResolveError::PolicyConstraintFromNonPolicyScope {
                scope: overlay.scope,
                setting_id,
            });
        }
        Ok(self.overlays.insert(overlay.scope, overlay))
    }

    /// Remove an overlay. Returns the removed overlay, if any.
    pub fn clear_overlay(&mut self, scope: SettingScope) -> Option<ScopeOverlay> {
        self.overlays.remove(&scope)
    }

    /// Records the current state for one capability dependency.
    ///
    /// Dependencies without an explicit state remain visible on exported
    /// records as satisfied by default; callers that know a dependency is
    /// unavailable must mark it unmet so the resolver can lock the setting.
    pub fn set_capability_state(
        &mut self,
        dependency: &CapabilityDependency,
        state: CapabilityState,
    ) -> Option<CapabilityState> {
        self.capability_states.insert(dependency.key(), state)
    }

    /// Removes the recorded state for one capability dependency.
    pub fn clear_capability_state(
        &mut self,
        dependency: &CapabilityDependency,
    ) -> Option<CapabilityState> {
        self.capability_states.remove(&dependency.key())
    }

    /// Iterate over explicit capability states in deterministic order.
    pub fn capability_states(&self) -> impl Iterator<Item = (&str, &CapabilityState)> {
        self.capability_states
            .iter()
            .map(|(key, state)| (key.as_str(), state))
    }

    /// Iterate over installed overlays in deterministic scope order.
    pub fn overlays(&self) -> impl Iterator<Item = &ScopeOverlay> {
        self.overlays.values()
    }

    /// Resolve the effective value for `setting_id`.
    pub fn resolve(&self, setting_id: &str) -> Result<EffectiveValue, ResolveError> {
        let def =
            self.registry
                .definition(setting_id)
                .ok_or_else(|| ResolveError::UnknownSetting {
                    setting_id: setting_id.to_owned(),
                })?;
        Ok(self.resolve_with_definition(def))
    }

    /// Resolve and project the canonical effective-setting record.
    pub fn resolve_record(&self, setting_id: &str) -> Result<EffectiveSettingRecord, ResolveError> {
        let def =
            self.registry
                .definition(setting_id)
                .ok_or_else(|| ResolveError::UnknownSetting {
                    setting_id: setting_id.to_owned(),
                })?;
        let effective = self.resolve_with_definition(def);
        Ok(self.effective_record_from(def, &effective, None))
    }

    fn resolve_with_definition(&self, def: &SettingDefinition) -> EffectiveValue {
        // Step 1: collect ordinary candidates in increasing precedence
        // order so the last entry is the layered winner.
        let mut ordinary: Vec<(SettingScope, &SettingValue, String)> = Vec::new();
        // Built-in default is always present, even when no overlay is
        // registered.
        ordinary.push((
            SettingScope::BuiltInDefault,
            &def.default_value,
            self.overlays
                .get(&SettingScope::BuiltInDefault)
                .map(|o| o.source_label.clone())
                .unwrap_or_else(|| "Aureline default".to_owned()),
        ));
        for scope in SettingScope::ordinary_scopes() {
            if matches!(scope, SettingScope::BuiltInDefault) {
                continue;
            }
            if let Some(overlay) = self.overlays.get(scope) {
                if let Some(value) = overlay.values.get(&def.setting_id) {
                    ordinary.push((*scope, value, overlay.source_label.clone()));
                }
            }
        }

        // The built-in default may also be overridden through a
        // BuiltInDefault overlay (release channel test fixtures use
        // this). Replace the first row when an overlay value is
        // present.
        if let Some(overlay) = self.overlays.get(&SettingScope::BuiltInDefault) {
            if let Some(value) = overlay.values.get(&def.setting_id) {
                ordinary[0] = (
                    SettingScope::BuiltInDefault,
                    value,
                    overlay.source_label.clone(),
                );
            }
        }

        // Sort by precedence rank ascending; stable sort preserves
        // declared scope order at equal ranks (none of our ordinary
        // ranks tie, but defensive).
        ordinary.sort_by_key(|(scope, _, _)| scope.precedence_rank());

        let layered_winner_index = ordinary.len() - 1;

        // Step 2: check the policy ceiling.
        let policy_overlay = self.overlays.get(&SettingScope::AdminPolicyNarrowing);
        let policy_constraint = policy_overlay
            .filter(|_| def.is_policy_narrowable)
            .and_then(|o| o.policy_constraint.get(&def.setting_id));

        // Determine winning scope, value, source label, lock state.
        let mut winning_scope = ordinary[layered_winner_index].0;
        let mut winning_value: SettingValue = ordinary[layered_winner_index].1.clone();
        let mut winning_label = ordinary[layered_winner_index].2.clone();
        let mut lock_state = LockState::Inherited;
        let mut lock_reason = LockReason::Inherited;
        let mut policy_ceiling_active = false;

        if winning_scope.precedence_rank() > SettingScope::BuiltInDefault.precedence_rank() {
            // Layered winner is not the built-in default; not
            // inherited any more.
            lock_state = LockState::Open;
            lock_reason = LockReason::None;
        }

        if let Some(constraint) = policy_constraint {
            policy_ceiling_active = true;
            if !constraint.admits(&winning_value) {
                // Policy narrows the layered winner.
                winning_scope = SettingScope::AdminPolicyNarrowing;
                winning_value = constraint.first_allowed().clone();
                winning_label = policy_overlay
                    .map(|o| o.source_label.clone())
                    .unwrap_or_else(|| "Admin policy".to_owned());
                if constraint.pins_value() {
                    lock_state = LockState::PolicyLocked;
                    lock_reason = LockReason::PolicyLocked;
                } else {
                    lock_state = LockState::PolicyConstrained;
                    lock_reason = LockReason::PolicyConstrainsAllowedSet;
                }
            } else if constraint.pins_value() {
                // Policy pins the value but the layered winner
                // already matches; still surface the lock as policy
                // pinned so the surface does not pretend it is
                // editable.
                lock_state = LockState::PolicyLocked;
                lock_reason = LockReason::PolicyLocked;
            } else {
                // Constraint admits a set; layered winner is inside
                // the set. Surface the constraint without locking.
                lock_state = LockState::PolicyConstrained;
                lock_reason = LockReason::PolicyConstrainsAllowedSet;
            }
        }

        if self.has_unmet_capability(def) && !matches!(lock_state, LockState::PolicyLocked) {
            lock_state = LockState::CapabilityLocked;
            lock_reason = LockReason::CapabilityDependencyUnmet;
        }

        // Step 3: build the shadow chain.
        let mut shadow_chain: Vec<ShadowChainEntry> = Vec::new();
        for (scope, value, label) in ordinary.iter() {
            let relation = if *scope == winning_scope
                && **value == winning_value
                && !matches!(winning_scope, SettingScope::AdminPolicyNarrowing)
            {
                ShadowRelation::Winner
            } else if matches!(winning_scope, SettingScope::AdminPolicyNarrowing)
                && *scope == ordinary[layered_winner_index].0
            {
                ShadowRelation::Capped
            } else {
                ShadowRelation::Shadowed
            };
            shadow_chain.push(ShadowChainEntry {
                scope: *scope,
                source_label: label.clone(),
                value_preview: value.preview(),
                value_present: true,
                winner: matches!(relation, ShadowRelation::Winner),
                relation,
            });
        }
        if let Some(overlay) = policy_overlay {
            if let Some(constraint) = overlay.policy_constraint.get(&def.setting_id) {
                let value_preview = match constraint {
                    PolicyConstraint::SingleValue { value } => value.preview(),
                    PolicyConstraint::NarrowedSet { allowed } => {
                        let parts: Vec<String> =
                            allowed.iter().map(SettingValue::preview).collect();
                        format!("allowed=[{}]", parts.join(","))
                    }
                };
                let relation = if matches!(winning_scope, SettingScope::AdminPolicyNarrowing) {
                    ShadowRelation::Winner
                } else {
                    ShadowRelation::PolicyCeiling
                };
                shadow_chain.push(ShadowChainEntry {
                    scope: SettingScope::AdminPolicyNarrowing,
                    source_label: overlay.source_label.clone(),
                    value_preview,
                    value_present: true,
                    winner: matches!(relation, ShadowRelation::Winner),
                    relation,
                });
            }
        }

        EffectiveValue {
            setting_id: def.setting_id.clone(),
            value: winning_value,
            winning_scope,
            source_label: winning_label,
            shadow_chain,
            lock_state,
            lock_reason,
            restart_posture: def.restart_posture,
            policy_ceiling_active,
        }
    }

    /// Attempt to write `value` at `target_scope` for `setting_id`.
    ///
    /// On `Allowed`, the resolver records the new overlay value and
    /// returns the effective value the next reader will observe.
    /// On `Denied`, the resolver records nothing; surfaces MUST
    /// surface the typed `denial_reason` and the existing shadow
    /// chain rather than retrying silently.
    pub fn attempt_write(
        &mut self,
        setting_id: &str,
        target_scope: SettingScope,
        value: SettingValue,
    ) -> WriteAttemptOutcome {
        let effective_before = self.resolve(setting_id).ok();

        let def = match self.registry.definition(setting_id) {
            Some(def) => def.clone(),
            None => {
                return WriteAttemptOutcome {
                    setting_id: setting_id.to_owned(),
                    target_scope,
                    proposed_value: value,
                    verdict: WriteIntent::Denied,
                    denial_reason: Some(WriteDenialReason::UnknownSetting {
                        setting_id: setting_id.to_owned(),
                    }),
                    effective_after: None,
                    effective_before: None,
                };
            }
        };

        if matches!(def.lifecycle_label, crate::schema::LifecycleLabel::Retired) {
            return self.deny(
                &def,
                target_scope,
                value,
                effective_before,
                WriteDenialReason::RetiredSetting,
            );
        }

        if !def.allows_scope(target_scope) {
            return self.deny(
                &def,
                target_scope,
                value,
                effective_before,
                WriteDenialReason::ScopeNotAllowed,
            );
        }

        if let Err(err) = def.validate_value(&value) {
            return self.deny(
                &def,
                target_scope,
                value,
                effective_before,
                WriteDenialReason::ValidationFailed {
                    detail: err.to_string(),
                },
            );
        }

        if self.has_unmet_capability(&def) {
            return self.deny(
                &def,
                target_scope,
                value,
                effective_before,
                WriteDenialReason::CapabilityDependencyUnmet,
            );
        }

        // Policy lock check: if policy pins the value and the proposed
        // value would not be admitted, deny.
        if let Some(overlay) = self.overlays.get(&SettingScope::AdminPolicyNarrowing) {
            if let Some(constraint) = overlay.policy_constraint.get(setting_id) {
                let _ = self::is_value_validation_failure(&def, &value);
                if constraint.pins_value() && !constraint.admits(&value) {
                    return self.deny(
                        &def,
                        target_scope,
                        value,
                        effective_before,
                        WriteDenialReason::PolicyLocked,
                    );
                }
                if !constraint.admits(&value) {
                    return self.deny(
                        &def,
                        target_scope,
                        value,
                        effective_before,
                        WriteDenialReason::PolicyConstrainedValue,
                    );
                }
            }
        }

        // Allowed: record the value.
        let overlay = self
            .overlays
            .entry(target_scope)
            .or_insert_with(|| ScopeOverlay::new(target_scope, default_label_for(target_scope)));
        overlay.values.insert(setting_id.to_owned(), value.clone());

        let effective_after = self.resolve_with_definition(&def);
        let verdict = allowed_intent_for(&def);

        WriteAttemptOutcome {
            setting_id: setting_id.to_owned(),
            target_scope,
            proposed_value: value,
            verdict,
            denial_reason: None,
            effective_after: Some(effective_after),
            effective_before,
        }
    }

    fn deny(
        &self,
        def: &SettingDefinition,
        target_scope: SettingScope,
        value: SettingValue,
        effective_before: Option<EffectiveValue>,
        reason: WriteDenialReason,
    ) -> WriteAttemptOutcome {
        let effective_after = effective_before.clone().or_else(|| {
            // Fall back to recomputing if the caller did not have an
            // earlier read (e.g. unknown setting case is special-cased
            // above).
            Some(self.resolve_with_definition(def))
        });
        WriteAttemptOutcome {
            setting_id: def.setting_id.clone(),
            target_scope,
            proposed_value: value,
            verdict: WriteIntent::Denied,
            denial_reason: Some(reason),
            effective_after,
            effective_before,
        }
    }

    fn has_unmet_capability(&self, def: &SettingDefinition) -> bool {
        def.capability_dependencies.iter().any(|dependency| {
            self.capability_states
                .get(&dependency.key())
                .is_some_and(|state| !state.satisfied)
        })
    }

    fn effective_record_from(
        &self,
        def: &SettingDefinition,
        effective: &EffectiveValue,
        denial_reason: Option<&WriteDenialReason>,
    ) -> EffectiveSettingRecord {
        let write_intent = if denial_reason.is_some() {
            WriteIntent::Denied
        } else {
            allowed_intent_for(def)
        };
        EffectiveSettingRecord {
            record_kind: "effective_setting_record".to_owned(),
            settings_schema_version: EFFECTIVE_SETTINGS_SCHEMA_VERSION,
            setting_id: def.setting_id.clone(),
            value: redacted_json_value(def, &effective.value),
            resolved_scope: effective.winning_scope.as_str().to_owned(),
            source_label: effective.source_label.clone(),
            lifecycle_label: lifecycle_token(def.lifecycle_label).to_owned(),
            shadow_chain: effective.shadow_chain.clone(),
            lock_state: exported_lock_state(effective.lock_state).to_owned(),
            lock_reason: exported_lock_reason(effective.lock_reason).to_owned(),
            write_intent: write_intent_token(write_intent).to_owned(),
            write_denial_reason: write_denial_token(denial_reason).to_owned(),
            restart_posture: def.restart_posture.as_str().to_owned(),
            preview_class: def.preview_class.as_str().to_owned(),
            capability_dependencies: self.effective_capability_dependencies(def),
            control_stack: EffectiveControlStack {
                source_label: effective.source_label.clone(),
                lifecycle_label: lifecycle_token(def.lifecycle_label).to_owned(),
                last_refresh_at: None,
                expires_at: None,
                offline_fallback: Some("authoritative_local".to_owned()),
                explain_why_ref: Some(format!("settings://explain/{}", def.setting_id)),
                control_authority: control_authority_for(effective).to_owned(),
                narrowing_ceiling_active: effective.policy_ceiling_active,
            },
            last_written: EffectiveLastWritten {
                at: "1970-01-01T00:00:00Z".to_owned(),
                actor_class: "user_command".to_owned(),
                mutation_journal_ref: None,
                rollback_checkpoint_ref: None,
            },
            schema_version: SETTING_DEFINITION_SCHEMA_VERSION.to_owned(),
            redaction_class: def.redaction_class.as_str().to_owned(),
        }
    }

    fn effective_capability_dependencies(
        &self,
        def: &SettingDefinition,
    ) -> Vec<EffectiveCapabilityDependency> {
        def.capability_dependencies
            .iter()
            .map(|dependency| {
                let state = self.capability_states.get(&dependency.key());
                EffectiveCapabilityDependency {
                    kind: dependency.kind.as_str().to_owned(),
                    required_ref: dependency.required_ref.clone(),
                    satisfied: state.map(|state| state.satisfied).unwrap_or(true),
                    observed_state: state.and_then(|state| state.observed_state.clone()),
                }
            })
            .collect()
    }

    /// Export the current overlay state as a JSON value. Round-trips
    /// losslessly through [`Self::import_state`]. The export is
    /// deterministic (BTreeMap-backed); fixtures can diff cleanly
    /// across runs.
    pub fn export_state(&self) -> serde_json::Value {
        let overlays: Vec<&ScopeOverlay> = self.overlays.values().collect();
        serde_json::json!({
            "record_kind": "effective_settings_resolver_state",
            "schema_version": 1,
            "overlays": overlays,
            "capability_states": &self.capability_states,
        })
    }

    /// Restore overlay state from a JSON value previously produced by
    /// [`Self::export_state`]. Re-validates every value; rejects
    /// states that disagree with the active registry.
    pub fn import_state(&mut self, value: &serde_json::Value) -> Result<(), ResolveError> {
        #[derive(Deserialize)]
        struct Envelope {
            overlays: Vec<ScopeOverlay>,
            #[serde(default)]
            capability_states: BTreeMap<String, CapabilityState>,
        }
        let envelope: Envelope = serde_json::from_value(value.clone()).map_err(|err| {
            ResolveError::InvalidOverlayValue {
                setting_id: String::new(),
                scope: SettingScope::BuiltInDefault,
                detail: err.to_string(),
            }
        })?;
        self.overlays.clear();
        self.capability_states = envelope.capability_states;
        for overlay in envelope.overlays {
            self.set_overlay(overlay)?;
        }
        Ok(())
    }
}

fn allowed_intent_for(def: &SettingDefinition) -> WriteIntent {
    match def.preview_class {
        PreviewClass::RollbackCheckpointAndApprovalRequired => {
            WriteIntent::AllowedWithRollbackCheckpointAndApproval
        }
        PreviewClass::RollbackCheckpointRequired => WriteIntent::AllowedWithRollbackCheckpoint,
        PreviewClass::ManagedActionOnly => WriteIntent::AllowedRequiresApprovalTicket,
        PreviewClass::PreviewRequired => WriteIntent::AllowedWithPreview,
        PreviewClass::SafeApply
            if !matches!(
                def.restart_posture,
                RestartPosture::NoRestart | RestartPosture::ReloadView
            ) =>
        {
            WriteIntent::AllowedWithRestart
        }
        PreviewClass::SafeApply => WriteIntent::Allowed,
    }
}

fn lifecycle_token(lifecycle: LifecycleLabel) -> &'static str {
    match lifecycle {
        LifecycleLabel::Stable => "stable",
        LifecycleLabel::Preview => "beta",
        LifecycleLabel::Experimental => "experimental",
        LifecycleLabel::Deprecated => "deprecated",
        LifecycleLabel::Retired => "retired",
    }
}

fn exported_lock_state(lock_state: LockState) -> &'static str {
    match lock_state {
        LockState::Open | LockState::Inherited => "unlocked",
        LockState::PolicyLocked => "policy_locked",
        LockState::PolicyConstrained => "policy_constrained",
        LockState::CapabilityLocked => "capability_locked",
        LockState::UnsupportedScope | LockState::DegradedReadOnly => "read_only_surface",
    }
}

fn exported_lock_reason(lock_reason: LockReason) -> &'static str {
    match lock_reason {
        LockReason::None | LockReason::Inherited => "none",
        LockReason::PolicyLocked => "policy_pins_value",
        LockReason::PolicyConstrainsAllowedSet => "policy_constrains_allowed_set",
        LockReason::CapabilityDependencyUnmet => "capability_dependency_unmet",
        LockReason::UnsupportedScope => "surface_cannot_write_this_scope",
        LockReason::DegradedReadOnly => "degraded_read_only",
        LockReason::SettingRetired => "setting_retired",
        LockReason::ManagedModeOnly => "managed_mode_only",
    }
}

fn redacted_json_value(def: &SettingDefinition, value: &SettingValue) -> serde_json::Value {
    match def.redaction_class {
        RedactionClass::None | RedactionClass::UiStringOnly => value.to_json(),
        RedactionClass::RedactValuePreserveShape => serde_json::json!({
            "redacted": true,
            "kind": def.value_type.kind_token(),
        }),
        RedactionClass::RedactToClassLabel => serde_json::json!({
            "class_label": def.sensitivity_class.as_str(),
        }),
        RedactionClass::ExcludeFromExport => serde_json::json!({
            "excluded_from_export": true,
        }),
    }
}

fn control_authority_for(effective: &EffectiveValue) -> &'static str {
    if effective.policy_ceiling_active {
        "signed_admin_bundle"
    } else if effective.winning_scope == SettingScope::ChannelOrExperimentDefault {
        "experiment_rollout"
    } else if effective.winning_scope == SettingScope::BuiltInDefault {
        "embedded_default"
    } else {
        "user_profile_workspace"
    }
}

fn default_label_for(scope: SettingScope) -> &'static str {
    match scope {
        SettingScope::BuiltInDefault => "Aureline default",
        SettingScope::ChannelOrExperimentDefault => "Release channel",
        SettingScope::ImportedProfileDefault => "Imported profile",
        SettingScope::UserGlobal => "User settings",
        SettingScope::MachineSpecific => "This machine",
        SettingScope::Workspace => "Workspace settings",
        SettingScope::FolderOrModuleOverride => "Folder override",
        SettingScope::LanguageOverride => "Language override",
        SettingScope::SessionOverride => "Session override",
        SettingScope::AdminPolicyNarrowing => "Admin policy",
    }
}

fn is_value_validation_failure(
    def: &SettingDefinition,
    value: &SettingValue,
) -> Option<ValueValidationError> {
    def.validate_value(value).err()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema::SchemaRegistry;

    fn registry() -> SchemaRegistry {
        SchemaRegistry::with_seed_catalog()
    }

    #[test]
    fn resolves_to_built_in_default_when_no_overlays() {
        let resolver = EffectiveSettingsResolver::new(registry());
        let effective = resolver.resolve("editor.tab_size").unwrap();
        assert_eq!(effective.value, SettingValue::Integer(4));
        assert_eq!(effective.winning_scope, SettingScope::BuiltInDefault);
        assert_eq!(effective.lock_state, LockState::Inherited);
        assert_eq!(effective.lock_reason, LockReason::Inherited);
        assert!(!effective.policy_ceiling_active);
        assert_eq!(effective.shadow_chain.len(), 1);
        assert_eq!(effective.shadow_chain[0].relation, ShadowRelation::Winner);
    }

    #[test]
    fn workspace_overlay_shadows_user_overlay() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
        user.set_value("editor.tab_size", SettingValue::Integer(8));
        resolver.set_overlay(user).unwrap();
        let mut workspace = ScopeOverlay::new(SettingScope::Workspace, "Workspace");
        workspace.set_value("editor.tab_size", SettingValue::Integer(2));
        resolver.set_overlay(workspace).unwrap();

        let effective = resolver.resolve("editor.tab_size").unwrap();
        assert_eq!(effective.value, SettingValue::Integer(2));
        assert_eq!(effective.winning_scope, SettingScope::Workspace);
        assert_eq!(effective.lock_state, LockState::Open);
        assert_eq!(effective.lock_reason, LockReason::None);
        // chain rows: built-in, user, workspace
        assert_eq!(effective.shadow_chain.len(), 3);
        assert_eq!(
            effective
                .shadow_chain
                .iter()
                .find(|e| e.scope == SettingScope::UserGlobal)
                .unwrap()
                .relation,
            ShadowRelation::Shadowed
        );
        assert_eq!(
            effective
                .shadow_chain
                .iter()
                .find(|e| e.scope == SettingScope::Workspace)
                .unwrap()
                .relation,
            ShadowRelation::Winner
        );
    }

    #[test]
    fn invalid_overlay_value_is_rejected() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
        user.set_value("editor.tab_size", SettingValue::Integer(99));
        let err = resolver.set_overlay(user).unwrap_err();
        assert!(matches!(err, ResolveError::InvalidOverlayValue { .. }));
    }

    #[test]
    fn admin_policy_narrows_layered_winner_to_locked_value() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
        user.set_value(
            "security.ai.egress_policy",
            SettingValue::String("any_hosted_provider".into()),
        );
        resolver.set_overlay(user).unwrap();

        let mut policy =
            ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle v3");
        policy.set_policy_constraint(
            "security.ai.egress_policy",
            PolicyConstraint::SingleValue {
                value: SettingValue::String("approved_hosted_providers_only".into()),
            },
        );
        resolver.set_overlay(policy).unwrap();

        let effective = resolver.resolve("security.ai.egress_policy").unwrap();
        assert_eq!(
            effective.value,
            SettingValue::String("approved_hosted_providers_only".into())
        );
        assert_eq!(effective.winning_scope, SettingScope::AdminPolicyNarrowing);
        assert_eq!(effective.lock_state, LockState::PolicyLocked);
        assert_eq!(effective.lock_reason, LockReason::PolicyLocked);
        assert!(effective.policy_ceiling_active);
        assert!(effective.pinned_by_policy());
        // capped row for the layered winner stays visible
        let user_row = effective
            .shadow_chain
            .iter()
            .find(|e| e.scope == SettingScope::UserGlobal)
            .unwrap();
        assert_eq!(user_row.relation, ShadowRelation::Capped);
    }

    #[test]
    fn write_to_unsupported_scope_is_denied() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let outcome = resolver.attempt_write(
            "vfs.watcher.fallback_polling_ms",
            SettingScope::UserGlobal,
            SettingValue::Integer(500),
        );
        assert_eq!(outcome.verdict, WriteIntent::Denied);
        assert!(matches!(
            outcome.denial_reason,
            Some(WriteDenialReason::ScopeNotAllowed)
        ));
        // No overlay was recorded.
        assert!(!resolver.overlays.contains_key(&SettingScope::UserGlobal));
    }

    #[test]
    fn write_violating_validation_is_denied_without_recording() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let outcome = resolver.attempt_write(
            "editor.tab_size",
            SettingScope::UserGlobal,
            SettingValue::Integer(99),
        );
        assert_eq!(outcome.verdict, WriteIntent::Denied);
        assert!(matches!(
            outcome.denial_reason,
            Some(WriteDenialReason::ValidationFailed { .. })
        ));
        let after = resolver.resolve("editor.tab_size").unwrap();
        assert_eq!(after.value, SettingValue::Integer(4));
    }

    #[test]
    fn write_under_policy_lock_is_denied_with_typed_reason() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let mut policy =
            ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle");
        policy.set_policy_constraint(
            "security.ai.egress_policy",
            PolicyConstraint::SingleValue {
                value: SettingValue::String("approved_hosted_providers_only".into()),
            },
        );
        resolver.set_overlay(policy).unwrap();

        let outcome = resolver.attempt_write(
            "security.ai.egress_policy",
            SettingScope::UserGlobal,
            SettingValue::String("any_hosted_provider".into()),
        );
        assert_eq!(outcome.verdict, WriteIntent::Denied);
        assert!(matches!(
            outcome.denial_reason,
            Some(WriteDenialReason::PolicyLocked)
        ));
        // The shadow chain on the failed-write outcome must surface
        // the policy ceiling.
        let after = outcome.effective_after.unwrap();
        assert!(after.policy_ceiling_active);
        assert!(after
            .shadow_chain
            .iter()
            .any(|e| e.scope == SettingScope::AdminPolicyNarrowing));
    }

    #[test]
    fn write_under_policy_constraint_admitting_value_is_allowed() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let mut policy =
            ScopeOverlay::new(SettingScope::AdminPolicyNarrowing, "Admin policy bundle");
        policy.set_policy_constraint(
            "security.ai.egress_policy",
            PolicyConstraint::NarrowedSet {
                allowed: vec![
                    SettingValue::String("approved_hosted_providers_only".into()),
                    SettingValue::String("disabled".into()),
                ],
            },
        );
        resolver.set_overlay(policy).unwrap();

        let outcome = resolver.attempt_write(
            "security.ai.egress_policy",
            SettingScope::UserGlobal,
            SettingValue::String("disabled".into()),
        );
        assert!(outcome.verdict.is_allowed());
        let after = outcome.effective_after.unwrap();
        // Layered winner is now the user value, but policy
        // constraint stays visible.
        assert_eq!(after.value, SettingValue::String("disabled".into()));
        assert_eq!(after.winning_scope, SettingScope::UserGlobal);
        assert_eq!(after.lock_state, LockState::PolicyConstrained);
    }

    #[test]
    fn capability_dependency_state_locks_reads_and_writes() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let dependency = resolver
            .registry()
            .definition("security.ai.egress_policy")
            .unwrap()
            .capability_dependencies[0]
            .clone();
        resolver.set_capability_state(&dependency, CapabilityState::unmet("identity_mode=local"));

        let effective = resolver.resolve("security.ai.egress_policy").unwrap();
        assert_eq!(effective.lock_state, LockState::CapabilityLocked);
        assert_eq!(effective.lock_reason, LockReason::CapabilityDependencyUnmet);

        let outcome = resolver.attempt_write(
            "security.ai.egress_policy",
            SettingScope::UserGlobal,
            SettingValue::String("approved_hosted_providers_only".into()),
        );
        assert_eq!(outcome.verdict, WriteIntent::Denied);
        assert!(matches!(
            outcome.denial_reason,
            Some(WriteDenialReason::CapabilityDependencyUnmet)
        ));

        let record = resolver
            .resolve_record("security.ai.egress_policy")
            .expect("record resolves");
        assert_eq!(record.lock_state, "capability_locked");
        assert_eq!(record.lock_reason, "capability_dependency_unmet");
        assert_eq!(record.capability_dependencies[0].satisfied, false);
    }

    #[test]
    fn effective_setting_record_carries_schema_shadow_and_control_stack() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
        user.set_value("editor.tab_size", SettingValue::Integer(8));
        resolver.set_overlay(user).unwrap();

        let record = resolver.resolve_record("editor.tab_size").unwrap();

        assert_eq!(record.record_kind, "effective_setting_record");
        assert_eq!(record.setting_id, "editor.tab_size");
        assert_eq!(record.resolved_scope, "user_global");
        assert_eq!(record.write_intent, "allowed");
        assert_eq!(record.write_denial_reason, "none");
        assert_eq!(record.preview_class, "safe_apply");
        assert_eq!(
            record.control_stack.control_authority,
            "user_profile_workspace"
        );
        assert!(record
            .shadow_chain
            .iter()
            .any(|row| row.scope == SettingScope::UserGlobal && row.winner));
    }

    #[test]
    fn write_intent_names_restart_and_preview_requirements() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let restart_outcome = resolver.attempt_write(
            "vfs.watcher.fallback_polling_ms",
            SettingScope::Workspace,
            SettingValue::Integer(500),
        );
        assert_eq!(restart_outcome.verdict, WriteIntent::AllowedWithRestart);

        let mut high_risk = EffectiveSettingsResolver::new(registry());
        let high_risk_outcome = high_risk.attempt_write(
            "security.ai.egress_policy",
            SettingScope::Workspace,
            SettingValue::String("approved_hosted_providers_only".into()),
        );
        assert_eq!(
            high_risk_outcome.verdict,
            WriteIntent::AllowedWithRollbackCheckpointAndApproval
        );
    }

    #[test]
    fn export_and_import_state_round_trips() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
        user.set_value("editor.tab_size", SettingValue::Integer(8));
        user.set_value("editor.format_on_save", SettingValue::Boolean(true));
        resolver.set_overlay(user).unwrap();

        let exported = resolver.export_state();
        let mut other = EffectiveSettingsResolver::new(registry());
        other.import_state(&exported).unwrap();

        let lhs = resolver.resolve("editor.tab_size").unwrap();
        let rhs = other.resolve("editor.tab_size").unwrap();
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn unknown_setting_returns_typed_error() {
        let resolver = EffectiveSettingsResolver::new(registry());
        let err = resolver.resolve("does.not.exist").unwrap_err();
        assert!(matches!(err, ResolveError::UnknownSetting { .. }));
    }

    #[test]
    fn unknown_setting_write_returns_typed_outcome() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let outcome = resolver.attempt_write(
            "does.not.exist",
            SettingScope::UserGlobal,
            SettingValue::Boolean(true),
        );
        assert_eq!(outcome.verdict, WriteIntent::Denied);
        assert!(matches!(
            outcome.denial_reason,
            Some(WriteDenialReason::UnknownSetting { .. })
        ));
    }

    #[test]
    fn policy_constraint_from_non_policy_scope_is_rejected() {
        let mut resolver = EffectiveSettingsResolver::new(registry());
        let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
        user.set_policy_constraint(
            "security.ai.egress_policy",
            PolicyConstraint::SingleValue {
                value: SettingValue::String("disabled".into()),
            },
        );
        let err = resolver.set_overlay(user).unwrap_err();
        assert!(matches!(
            err,
            ResolveError::PolicyConstraintFromNonPolicyScope { .. }
        ));
    }
}
