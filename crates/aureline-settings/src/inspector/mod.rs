//! Schema-backed effective-settings inspector records.
//!
//! The inspector is the shared projection layer above the resolver:
//! settings UI rows, CLI inspection, help deep links, write previews,
//! and support exports read these records instead of re-deriving a
//! private "settings" model from raw overlays.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub mod conflict;

use crate::resolver::{
    EffectiveSettingsResolver, EffectiveValue, LockReason, LockState, ResolveError,
    ShadowChainEntry, WriteAttemptOutcome, WriteDenialReason, WriteIntent,
};
use crate::schema::{
    CapabilityDependency, PreviewClass, RedactionClass, RestartPosture, SchemaRegistry,
    SensitivityClass, SettingDefinition, SettingScope, SettingValue,
};

/// Inspector packet schema version.
pub const SETTINGS_INSPECTOR_SCHEMA_VERSION: u32 = 1;

const SHARED_CONTRACT_REF: &str = "settings:effective_inspector_alpha:v1";

/// Context supplied by the caller while materializing inspector records.
#[derive(Debug, Clone, Default)]
pub struct SettingsInspectionContext {
    last_applied_revisions: BTreeMap<String, String>,
    capability_states: BTreeMap<String, CapabilityStateOverride>,
}

impl SettingsInspectionContext {
    /// Creates an empty inspection context.
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds the last-applied revision for `setting_id`.
    pub fn with_last_applied_revision(
        mut self,
        setting_id: impl Into<String>,
        revision: impl Into<String>,
    ) -> Self {
        self.last_applied_revisions
            .insert(setting_id.into(), revision.into());
        self
    }

    /// Adds an observed capability state for one dependency.
    pub fn with_capability_state(
        mut self,
        dependency: &CapabilityDependency,
        satisfied: bool,
        observed_state: impl Into<String>,
    ) -> Self {
        self.capability_states.insert(
            dependency.key(),
            CapabilityStateOverride {
                satisfied,
                observed_state: observed_state.into(),
            },
        );
        self
    }
}

/// Observed state for one capability dependency.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityStateOverride {
    /// True when the dependency is currently satisfied.
    pub satisfied: bool,
    /// Human-readable current state, redacted before export by the caller.
    pub observed_state: String,
}

/// Errors returned while materializing inspector records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsInspectError {
    /// The requested setting is not present in the schema registry.
    UnknownSetting { setting_id: String },
    /// The resolver could not resolve the effective value.
    ResolveFailed { setting_id: String, detail: String },
}

impl std::fmt::Display for SettingsInspectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownSetting { setting_id } => {
                write!(f, "setting_id {setting_id:?} is not registered")
            }
            Self::ResolveFailed { setting_id, detail } => {
                write!(f, "setting_id {setting_id:?} could not resolve: {detail}")
            }
        }
    }
}

impl std::error::Error for SettingsInspectError {}

/// Schema-backed setting definition fields exposed by the inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorSettingDefinition {
    /// Canonical setting id.
    pub setting_id: String,
    /// Declared value-type token.
    pub declared_type: String,
    /// Allowed write scopes.
    pub allowed_scopes: Vec<String>,
    /// Built-in default preview.
    pub default_value_preview: String,
    /// Migration aliases that redirect to this setting.
    pub migration_aliases: Vec<String>,
    /// Declared restart posture.
    pub restart_posture: String,
    /// Sensitivity class used by preview and export policy.
    pub sensitivity_class: String,
    /// Redaction class used by export surfaces.
    pub redaction_class: String,
    /// Preview, checkpoint, and approval posture.
    pub preview_class: String,
    /// Capability dependencies declared by the definition.
    pub capability_dependencies: Vec<InspectorCapabilityDependency>,
    /// Stable help document reference.
    pub help_doc_ref: Option<String>,
    /// Evidence references attached to the definition.
    pub evidence_refs: Vec<String>,
    /// One-sentence summary.
    pub summary: String,
}

impl InspectorSettingDefinition {
    fn from_definition(def: &SettingDefinition) -> Self {
        Self {
            setting_id: def.setting_id.clone(),
            declared_type: def.value_type.kind_token().to_owned(),
            allowed_scopes: def
                .allowed_scopes
                .iter()
                .map(|scope| scope.as_str().to_owned())
                .collect(),
            default_value_preview: redacted_value_summary(def, &def.default_value),
            migration_aliases: def
                .alias_set
                .iter()
                .map(|alias| alias.from_id.clone())
                .collect(),
            restart_posture: def.restart_posture.as_str().to_owned(),
            sensitivity_class: def.sensitivity_class.as_str().to_owned(),
            redaction_class: def.redaction_class.as_str().to_owned(),
            preview_class: def.preview_class.as_str().to_owned(),
            capability_dependencies: def
                .capability_dependencies
                .iter()
                .map(InspectorCapabilityDependency::from_dependency)
                .collect(),
            help_doc_ref: def.help_doc_ref.clone(),
            evidence_refs: def.evidence_refs.clone(),
            summary: def.summary.clone(),
        }
    }
}

/// Capability dependency projected from a setting definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorCapabilityDependency {
    /// Dependency kind token.
    pub kind: String,
    /// Stable required-state reference, if any.
    pub required_ref: Option<String>,
}

impl InspectorCapabilityDependency {
    fn from_dependency(dependency: &CapabilityDependency) -> Self {
        Self {
            kind: dependency.kind.as_str().to_owned(),
            required_ref: dependency.required_ref.clone(),
        }
    }
}

/// Capability dependency state observed while inspecting a setting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorCapabilityState {
    /// Dependency kind token.
    pub kind: String,
    /// Stable required-state reference, if any.
    pub required_ref: Option<String>,
    /// Availability class for this dependency.
    pub availability: String,
    /// True when the dependency is known to be satisfied.
    pub satisfied: Option<bool>,
    /// Observed state label when the caller has one.
    pub observed_state: Option<String>,
}

/// One row in the inspectable source chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorShadowRow {
    /// Scope token for the candidate source.
    pub scope: String,
    /// Human-readable source label.
    pub source_label: String,
    /// Redacted value preview for this source.
    pub value_preview: String,
    /// Relation to the effective winner.
    pub relation: String,
    /// True when this row is the effective winner.
    pub winner: bool,
}

impl InspectorShadowRow {
    fn from_entry(entry: &ShadowChainEntry, def: &SettingDefinition) -> Self {
        Self {
            scope: entry.scope.as_str().to_owned(),
            source_label: entry.source_label.clone(),
            value_preview: redacted_preview_string(def, &entry.value_preview),
            relation: entry.relation.as_str().to_owned(),
            winner: entry.relation.as_str() == "winner",
        }
    }
}

/// Restart state exposed by inspectors and write previews.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InspectorRestartState {
    /// Declared restart posture token.
    pub restart_posture: String,
    /// True when the value needs a restart or reload to fully apply.
    pub restart_required: bool,
    /// Current state token for the inspected value.
    pub state: String,
    /// Short explanation suitable for CLI and support export.
    pub explanation: String,
}

/// Explanation attached to policy-locked or policy-constrained settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyLockExplanation {
    /// Lock reason token.
    pub reason: String,
    /// Source scope that supplied the policy ceiling.
    pub policy_source_scope: String,
    /// Human-readable source label from the policy overlay.
    pub policy_source_label: String,
    /// Stable source ref for support exports.
    pub policy_source_ref: String,
    /// Human-readable explanation.
    pub explanation: String,
}

/// Canonical effective-setting inspector record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveSettingInspectionRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Inspector schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, help, and support export.
    pub shared_contract_ref: String,
    /// Stable ref for consumers that need to point back to this record.
    pub source_record_ref: String,
    /// Schema-backed definition fields.
    pub definition: InspectorSettingDefinition,
    /// Canonical setting id.
    pub setting_id: String,
    /// Winning value when export posture permits literal value exposure.
    pub winning_value: Option<SettingValue>,
    /// Winning value preview or redacted summary.
    pub winning_value_summary: String,
    /// Scope that supplied the effective value.
    pub winning_scope: String,
    /// Human-readable source label.
    pub source_label: String,
    /// Ordered source chain with shadowed and policy-ceiling rows.
    pub shadow_chain: Vec<InspectorShadowRow>,
    /// Lock-state token.
    pub lock_state: String,
    /// Lock-reason token.
    pub lock_reason: String,
    /// Policy-lock explanation when a policy ceiling is active.
    pub policy_lock_explanation: Option<PolicyLockExplanation>,
    /// Validation state for the effective value.
    pub validation_status: String,
    /// Restart state for the effective value.
    pub restart_state: InspectorRestartState,
    /// Aggregate capability availability.
    pub capability_availability: String,
    /// Per-dependency capability states.
    pub capability_dependencies: Vec<InspectorCapabilityState>,
    /// Last applied revision known to the caller.
    pub last_applied_revision: Option<String>,
}

impl EffectiveSettingInspectionRecord {
    fn from_effective(
        def: &SettingDefinition,
        effective: &EffectiveValue,
        context: &SettingsInspectionContext,
    ) -> Self {
        let (winning_value, winning_value_summary) = export_value(def, &effective.value);
        let capability_dependencies = capability_states(def, context);
        let capability_availability = aggregate_capability_availability(&capability_dependencies);
        let validation_status = match def.validate_value(&effective.value) {
            Ok(()) => "valid".to_owned(),
            Err(err) => format!("invalid: {err}"),
        };
        Self {
            record_kind: "effective_setting_inspection_record".to_owned(),
            schema_version: SETTINGS_INSPECTOR_SCHEMA_VERSION,
            shared_contract_ref: SHARED_CONTRACT_REF.to_owned(),
            source_record_ref: source_record_ref(&def.setting_id),
            definition: InspectorSettingDefinition::from_definition(def),
            setting_id: def.setting_id.clone(),
            winning_value,
            winning_value_summary,
            winning_scope: effective.winning_scope.as_str().to_owned(),
            source_label: effective.source_label.clone(),
            shadow_chain: effective
                .shadow_chain
                .iter()
                .map(|entry| InspectorShadowRow::from_entry(entry, def))
                .collect(),
            lock_state: lock_state_token(effective.lock_state).to_owned(),
            lock_reason: lock_reason_token(effective.lock_reason).to_owned(),
            policy_lock_explanation: policy_lock_explanation(def, effective),
            validation_status,
            restart_state: restart_state(effective.restart_posture),
            capability_availability,
            capability_dependencies,
            last_applied_revision: context.last_applied_revisions.get(&def.setting_id).cloned(),
        }
    }
}

/// Materializes an effective-setting inspector record from a resolver.
pub fn inspect_setting(
    resolver: &EffectiveSettingsResolver,
    setting_id_or_alias: &str,
    context: &SettingsInspectionContext,
) -> Result<EffectiveSettingInspectionRecord, SettingsInspectError> {
    let def = resolver
        .registry()
        .resolve_definition(setting_id_or_alias)
        .ok_or_else(|| SettingsInspectError::UnknownSetting {
            setting_id: setting_id_or_alias.to_owned(),
        })?;
    let effective = resolver
        .resolve(&def.setting_id)
        .map_err(|err| inspect_resolve_error(&def.setting_id, err))?;
    Ok(EffectiveSettingInspectionRecord::from_effective(
        def, &effective, context,
    ))
}

/// Materializes inspector records for every registered setting.
pub fn inspect_all_settings(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
) -> Result<Vec<EffectiveSettingInspectionRecord>, SettingsInspectError> {
    resolver
        .registry()
        .ids()
        .map(|setting_id| inspect_setting(resolver, setting_id, context))
        .collect()
}

/// UI projection that consumes an effective-setting inspector record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsUiInspectorProjection {
    /// Record discriminator.
    pub record_kind: String,
    /// Inspector schema version.
    pub schema_version: u32,
    /// Ref to the effective-setting inspection record.
    pub source_record_ref: String,
    /// Canonical setting id.
    pub setting_id: String,
    /// Value preview rendered by the row.
    pub value_preview: String,
    /// Winning source pill label.
    pub source_pill_label: String,
    /// Winning source scope token.
    pub winning_scope: String,
    /// Lock-state token.
    pub lock_state: String,
    /// Policy explanation ref, if policy is active.
    pub policy_explanation_ref: Option<String>,
    /// Restart posture token.
    pub restart_posture: String,
    /// Help deep-link ref generated from the same record.
    pub help_deep_link_ref: String,
}

/// Builds the UI projection for one inspected setting.
pub fn project_settings_ui(
    record: &EffectiveSettingInspectionRecord,
) -> SettingsUiInspectorProjection {
    SettingsUiInspectorProjection {
        record_kind: "settings_ui_inspector_projection".to_owned(),
        schema_version: SETTINGS_INSPECTOR_SCHEMA_VERSION,
        source_record_ref: record.source_record_ref.clone(),
        setting_id: record.setting_id.clone(),
        value_preview: record.winning_value_summary.clone(),
        source_pill_label: record.source_label.clone(),
        winning_scope: record.winning_scope.clone(),
        lock_state: record.lock_state.clone(),
        policy_explanation_ref: record
            .policy_lock_explanation
            .as_ref()
            .map(|explanation| explanation.policy_source_ref.clone()),
        restart_posture: record.restart_state.restart_posture.clone(),
        help_deep_link_ref: help_deep_link_ref(&record.setting_id),
    }
}

/// CLI projection that consumes an effective-setting inspector record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsCliInspectProjection {
    /// Record discriminator.
    pub record_kind: String,
    /// Inspector schema version.
    pub schema_version: u32,
    /// Ref to the effective-setting inspection record.
    pub source_record_ref: String,
    /// Canonical setting id.
    pub setting_id: String,
    /// Deterministic key-value fields for CLI output.
    pub fields: BTreeMap<String, String>,
}

/// Builds the CLI projection for one inspected setting.
pub fn project_cli_inspect(
    record: &EffectiveSettingInspectionRecord,
) -> SettingsCliInspectProjection {
    let mut fields = BTreeMap::new();
    fields.insert("setting_id".to_owned(), record.setting_id.clone());
    fields.insert("value".to_owned(), record.winning_value_summary.clone());
    fields.insert("winning_scope".to_owned(), record.winning_scope.clone());
    fields.insert("source_label".to_owned(), record.source_label.clone());
    fields.insert("lock_state".to_owned(), record.lock_state.clone());
    fields.insert("lock_reason".to_owned(), record.lock_reason.clone());
    fields.insert(
        "restart_posture".to_owned(),
        record.restart_state.restart_posture.clone(),
    );
    fields.insert(
        "capability_availability".to_owned(),
        record.capability_availability.clone(),
    );
    if let Some(revision) = &record.last_applied_revision {
        fields.insert("last_applied_revision".to_owned(), revision.clone());
    }
    SettingsCliInspectProjection {
        record_kind: "settings_cli_inspect_projection".to_owned(),
        schema_version: SETTINGS_INSPECTOR_SCHEMA_VERSION,
        source_record_ref: record.source_record_ref.clone(),
        setting_id: record.setting_id.clone(),
        fields,
    }
}

/// Help deep-link projection that consumes an effective-setting inspector record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsHelpDeepLinkProjection {
    /// Record discriminator.
    pub record_kind: String,
    /// Inspector schema version.
    pub schema_version: u32,
    /// Ref to the effective-setting inspection record.
    pub source_record_ref: String,
    /// Canonical setting id.
    pub setting_id: String,
    /// Stable route ref for opening the setting help target.
    pub route_ref: String,
    /// Stable help doc ref from the definition.
    pub help_doc_ref: Option<String>,
    /// Evidence refs from the definition.
    pub evidence_refs: Vec<String>,
}

/// Builds the help deep-link projection for one inspected setting.
pub fn project_help_deep_link(
    record: &EffectiveSettingInspectionRecord,
) -> SettingsHelpDeepLinkProjection {
    SettingsHelpDeepLinkProjection {
        record_kind: "settings_help_deep_link_projection".to_owned(),
        schema_version: SETTINGS_INSPECTOR_SCHEMA_VERSION,
        source_record_ref: record.source_record_ref.clone(),
        setting_id: record.setting_id.clone(),
        route_ref: help_deep_link_ref(&record.setting_id),
        help_doc_ref: record.definition.help_doc_ref.clone(),
        evidence_refs: record.definition.evidence_refs.clone(),
    }
}

/// Support-export projection that carries canonical inspection records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsSupportExportProjection {
    /// Record discriminator.
    pub record_kind: String,
    /// Inspector schema version.
    pub schema_version: u32,
    /// Export id supplied by the caller.
    pub export_id: String,
    /// Shared contract ref consumed by support tooling.
    pub shared_contract_ref: String,
    /// Canonical effective-setting records included in the export.
    pub effective_settings: Vec<EffectiveSettingInspectionRecord>,
    /// Number of settings with active policy lock or constraint state.
    pub policy_locked_count: usize,
    /// Number of values whose literal payload was redacted from export.
    pub redacted_value_count: usize,
}

/// Builds a support-export projection from inspected settings.
pub fn project_support_export(
    export_id: impl Into<String>,
    records: Vec<EffectiveSettingInspectionRecord>,
) -> SettingsSupportExportProjection {
    let policy_locked_count = records
        .iter()
        .filter(|record| record.policy_lock_explanation.is_some())
        .count();
    let redacted_value_count = records
        .iter()
        .filter(|record| record.winning_value.is_none())
        .count();
    SettingsSupportExportProjection {
        record_kind: "settings_support_export_projection".to_owned(),
        schema_version: SETTINGS_INSPECTOR_SCHEMA_VERSION,
        export_id: export_id.into(),
        shared_contract_ref: SHARED_CONTRACT_REF.to_owned(),
        effective_settings: records,
        policy_locked_count,
        redacted_value_count,
    }
}

/// Reason class for a settings write intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteReasonClass {
    /// Interactive user edit.
    UserEdit,
    /// Profile apply or profile switch.
    ProfileApply,
    /// Imported settings artifact.
    Import,
    /// Optional settings sync.
    Sync,
    /// Admin-policy writer.
    Policy,
    /// Automation or extension API writer.
    Automation,
}

impl WriteReasonClass {
    /// Returns the stable token used in write-preview records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserEdit => "user_edit",
            Self::ProfileApply => "profile_apply",
            Self::Import => "import",
            Self::Sync => "sync",
            Self::Policy => "policy",
            Self::Automation => "automation",
        }
    }
}

/// Actor class for a settings write intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteActorClass {
    /// User keystroke in a settings surface.
    UserKeystroke,
    /// User command or command-palette action.
    UserCommand,
    /// Imported profile actor.
    ImportedProfile,
    /// Workspace migration actor.
    WorkspaceMigration,
    /// Admin-policy injector actor.
    AdminPolicyInjector,
    /// Experiment rollout actor.
    ExperimentRollout,
    /// Session override actor.
    SessionOverride,
    /// Extension API actor.
    ExtensionApi,
    /// AI apply action acknowledged by the user.
    AiApply,
}

impl WriteActorClass {
    /// Returns the stable token used in write-preview records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserKeystroke => "user_keystroke",
            Self::UserCommand => "user_command",
            Self::ImportedProfile => "imported_profile",
            Self::WorkspaceMigration => "workspace_migration",
            Self::AdminPolicyInjector => "admin_policy_injector",
            Self::ExperimentRollout => "experiment_rollout",
            Self::SessionOverride => "session_override",
            Self::ExtensionApi => "extension_api",
            Self::AiApply => "ai_apply",
        }
    }
}

/// Request used to preview a scope-explicit settings write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingWritePreviewRequest {
    /// Canonical setting id or migration alias.
    pub setting_id: String,
    /// Scope the caller selected.
    pub target_scope: SettingScope,
    /// Proposed value.
    pub proposed_value: SettingValue,
    /// Actor class for the mutation attempt.
    pub actor_class: WriteActorClass,
    /// Reason class for the mutation attempt.
    pub reason_class: WriteReasonClass,
    /// Rollback checkpoint ref, if already created.
    pub checkpoint_ref: Option<String>,
    /// Approval ticket ref, if already granted.
    pub approval_ticket_ref: Option<String>,
}

/// Destination preview for a scope-explicit settings write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteDestinationPreview {
    /// Requested target scope.
    pub requested_scope: String,
    /// Effective write scope selected by the resolver.
    pub effective_write_scope: String,
    /// Stable artifact or authority ref that would receive the write.
    pub target_artifact_ref: String,
    /// True when no broader scope is used.
    pub scope_explicit: bool,
    /// Scope-broadening verdict token.
    pub scope_broadening_verdict: String,
}

/// Structured change summary produced before a settings write applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteChangeSummary {
    /// Preview of the current effective value.
    pub before_value_preview: Option<String>,
    /// Preview of the value after apply.
    pub after_value_preview: Option<String>,
    /// Risk class for the proposed change.
    pub risk_class: String,
    /// True when the write has a rollback checkpoint or does not require one.
    pub rollback_ready: bool,
    /// Human-readable summary for preview surfaces.
    pub summary: String,
}

/// Scope-explicit write preview record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingWritePreviewRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Inspector schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, help, and support export.
    pub shared_contract_ref: String,
    /// Canonical setting id.
    pub setting_id: String,
    /// Scope selected by the caller.
    pub target_scope: String,
    /// Actor class token.
    pub actor_class: String,
    /// Reason class token.
    pub reason_class: String,
    /// Proposed value preview or redacted summary.
    pub proposed_value_preview: String,
    /// Write verdict token.
    pub verdict: String,
    /// Denial reason token, if denied.
    pub denial_reason: Option<String>,
    /// Preview class copied from the definition.
    pub preview_class: Option<String>,
    /// Restart posture copied from the definition.
    pub restart_posture: Option<String>,
    /// True when a preview must be shown before apply.
    pub preview_required: bool,
    /// True when a rollback checkpoint is required before apply.
    pub checkpoint_required: bool,
    /// Rollback checkpoint ref, if present.
    pub checkpoint_ref: Option<String>,
    /// True when approval is required before apply.
    pub approval_required: bool,
    /// Approval ticket ref, if present.
    pub approval_ticket_ref: Option<String>,
    /// Apply state token.
    pub apply_state: String,
    /// Destination preview for the write.
    pub destination_preview: WriteDestinationPreview,
    /// Structured change summary.
    pub change_summary: WriteChangeSummary,
    /// Effective-setting record before the proposed write.
    pub effective_before: Option<EffectiveSettingInspectionRecord>,
    /// Effective-setting record after the proposed write on the preview clone.
    pub effective_after: Option<EffectiveSettingInspectionRecord>,
}

/// Previews a settings write without mutating the live resolver.
pub fn preview_write(
    resolver: &EffectiveSettingsResolver,
    request: SettingWritePreviewRequest,
    context: &SettingsInspectionContext,
) -> SettingWritePreviewRecord {
    let def = resolver.registry().resolve_definition(&request.setting_id);
    let canonical_setting_id = def
        .map(|definition| definition.setting_id.clone())
        .unwrap_or_else(|| request.setting_id.clone());
    let mut probe = resolver.clone();
    let outcome = probe.attempt_write(
        &canonical_setting_id,
        request.target_scope,
        request.proposed_value.clone(),
    );
    write_preview_from_outcome(resolver.registry(), def, outcome, request, context)
}

fn write_preview_from_outcome(
    registry: &SchemaRegistry,
    def: Option<&SettingDefinition>,
    outcome: WriteAttemptOutcome,
    request: SettingWritePreviewRequest,
    context: &SettingsInspectionContext,
) -> SettingWritePreviewRecord {
    let preview_class = def.map(|definition| definition.preview_class);
    let restart_posture = def.map(|definition| definition.restart_posture);
    let high_risk = def.map(is_high_risk_setting).unwrap_or(false);
    let preview_required = preview_class
        .map(PreviewClass::requires_preview)
        .unwrap_or(false)
        || high_risk;
    let checkpoint_required = preview_class
        .map(PreviewClass::requires_checkpoint)
        .unwrap_or(false);
    let approval_required = preview_class
        .map(PreviewClass::requires_approval)
        .unwrap_or(false);
    let verdict = write_preview_verdict(&outcome, preview_class, restart_posture, preview_required);
    let apply_state = apply_state(
        &outcome.verdict,
        preview_required,
        checkpoint_required,
        approval_required,
        request.checkpoint_ref.as_ref(),
        request.approval_ticket_ref.as_ref(),
    );
    let effective_before =
        materialize_outcome_effective(registry, def, outcome.effective_before.as_ref(), context);
    let effective_after =
        materialize_outcome_effective(registry, def, outcome.effective_after.as_ref(), context);
    let before_value_preview = effective_before
        .as_ref()
        .map(|record| record.winning_value_summary.clone());
    let after_value_preview = effective_after
        .as_ref()
        .map(|record| record.winning_value_summary.clone());
    let proposed_value_preview = def
        .map(|definition| redacted_value_summary(definition, &request.proposed_value))
        .unwrap_or_else(|| request.proposed_value.preview());
    let risk_class = def
        .map(|definition| definition.sensitivity_class.as_str().to_owned())
        .unwrap_or_else(|| "unknown_setting".to_owned());
    let rollback_ready = !checkpoint_required || request.checkpoint_ref.is_some();
    let setting_id = outcome.setting_id.clone();
    SettingWritePreviewRecord {
        record_kind: "setting_write_preview_record".to_owned(),
        schema_version: SETTINGS_INSPECTOR_SCHEMA_VERSION,
        shared_contract_ref: SHARED_CONTRACT_REF.to_owned(),
        setting_id: setting_id.clone(),
        target_scope: outcome.target_scope.as_str().to_owned(),
        actor_class: request.actor_class.as_str().to_owned(),
        reason_class: request.reason_class.as_str().to_owned(),
        proposed_value_preview,
        verdict,
        denial_reason: outcome
            .denial_reason
            .as_ref()
            .map(write_denial_reason_token)
            .map(str::to_owned),
        preview_class: preview_class.map(|class| class.as_str().to_owned()),
        restart_posture: restart_posture.map(|posture| posture.as_str().to_owned()),
        preview_required,
        checkpoint_required,
        checkpoint_ref: request.checkpoint_ref,
        approval_required,
        approval_ticket_ref: request.approval_ticket_ref,
        apply_state,
        destination_preview: WriteDestinationPreview {
            requested_scope: outcome.target_scope.as_str().to_owned(),
            effective_write_scope: outcome.target_scope.as_str().to_owned(),
            target_artifact_ref: target_artifact_ref(outcome.target_scope, &setting_id),
            scope_explicit: true,
            scope_broadening_verdict: "none".to_owned(),
        },
        change_summary: WriteChangeSummary {
            before_value_preview,
            after_value_preview,
            risk_class,
            rollback_ready,
            summary: format!(
                "Write {setting_id} at {} with no broader-scope fan-out.",
                outcome.target_scope.as_str()
            ),
        },
        effective_before,
        effective_after,
    }
}

fn materialize_outcome_effective(
    registry: &SchemaRegistry,
    def: Option<&SettingDefinition>,
    effective: Option<&EffectiveValue>,
    context: &SettingsInspectionContext,
) -> Option<EffectiveSettingInspectionRecord> {
    let effective = effective?;
    let definition = def.or_else(|| registry.definition(&effective.setting_id))?;
    Some(EffectiveSettingInspectionRecord::from_effective(
        definition, effective, context,
    ))
}

fn capability_states(
    def: &SettingDefinition,
    context: &SettingsInspectionContext,
) -> Vec<InspectorCapabilityState> {
    def.capability_dependencies
        .iter()
        .map(|dependency| {
            let state = context.capability_states.get(&dependency.key());
            InspectorCapabilityState {
                kind: dependency.kind.as_str().to_owned(),
                required_ref: dependency.required_ref.clone(),
                availability: match state {
                    Some(state) if state.satisfied => "available",
                    Some(_) => "unavailable",
                    None => "unknown",
                }
                .to_owned(),
                satisfied: state.map(|state| state.satisfied),
                observed_state: state.map(|state| state.observed_state.clone()),
            }
        })
        .collect()
}

fn aggregate_capability_availability(states: &[InspectorCapabilityState]) -> String {
    if states.is_empty() {
        return "not_required".to_owned();
    }
    if states
        .iter()
        .any(|state| state.availability.as_str() == "unavailable")
    {
        return "unavailable".to_owned();
    }
    if states
        .iter()
        .any(|state| state.availability.as_str() == "unknown")
    {
        return "unknown".to_owned();
    }
    "available".to_owned()
}

fn export_value(def: &SettingDefinition, value: &SettingValue) -> (Option<SettingValue>, String) {
    let summary = redacted_value_summary(def, value);
    let value = match def.redaction_class {
        RedactionClass::None | RedactionClass::UiStringOnly => Some(value.clone()),
        RedactionClass::RedactValuePreserveShape
        | RedactionClass::RedactToClassLabel
        | RedactionClass::ExcludeFromExport => None,
    };
    (value, summary)
}

fn redacted_value_summary(def: &SettingDefinition, value: &SettingValue) -> String {
    match def.redaction_class {
        RedactionClass::None | RedactionClass::UiStringOnly => value.preview(),
        RedactionClass::RedactValuePreserveShape => {
            format!("redacted {} value", def.value_type.kind_token())
        }
        RedactionClass::RedactToClassLabel => {
            format!("{} value present", def.sensitivity_class.as_str())
        }
        RedactionClass::ExcludeFromExport => "excluded_from_export".to_owned(),
    }
}

fn redacted_preview_string(def: &SettingDefinition, value_preview: &str) -> String {
    match def.redaction_class {
        RedactionClass::None | RedactionClass::UiStringOnly => value_preview.to_owned(),
        RedactionClass::RedactValuePreserveShape => {
            format!("redacted {} value", def.value_type.kind_token())
        }
        RedactionClass::RedactToClassLabel => {
            format!("{} value present", def.sensitivity_class.as_str())
        }
        RedactionClass::ExcludeFromExport => "excluded_from_export".to_owned(),
    }
}

fn lock_state_token(lock_state: LockState) -> &'static str {
    match lock_state {
        LockState::Open | LockState::Inherited => "unlocked",
        LockState::PolicyLocked => "policy_locked",
        LockState::PolicyConstrained => "policy_constrained",
        LockState::CapabilityLocked => "capability_locked",
        LockState::UnsupportedScope | LockState::DegradedReadOnly => "read_only_surface",
    }
}

fn lock_reason_token(lock_reason: LockReason) -> &'static str {
    match lock_reason {
        LockReason::None => "none",
        LockReason::Inherited => "inherited",
        LockReason::PolicyLocked => "policy_pins_value",
        LockReason::PolicyConstrainsAllowedSet => "policy_constrains_allowed_set",
        LockReason::CapabilityDependencyUnmet => "capability_dependency_unmet",
        LockReason::UnsupportedScope => "surface_cannot_write_this_scope",
        LockReason::DegradedReadOnly => "degraded_read_only",
        LockReason::SettingRetired => "setting_retired",
        LockReason::ManagedModeOnly => "managed_mode_only",
    }
}

fn restart_state(posture: RestartPosture) -> InspectorRestartState {
    let restart_required = !matches!(posture, RestartPosture::NoRestart);
    InspectorRestartState {
        restart_posture: posture.as_str().to_owned(),
        restart_required,
        state: if restart_required {
            "restart_or_reload_required"
        } else {
            "applied_live"
        }
        .to_owned(),
        explanation: if restart_required {
            "The definition declares a restart or reload before the new value fully applies."
        } else {
            "The definition declares live apply with no restart required."
        }
        .to_owned(),
    }
}

fn policy_lock_explanation(
    def: &SettingDefinition,
    effective: &EffectiveValue,
) -> Option<PolicyLockExplanation> {
    if !effective.policy_ceiling_active {
        return None;
    }
    let policy_row = effective
        .shadow_chain
        .iter()
        .find(|row| row.scope == SettingScope::AdminPolicyNarrowing)?;
    Some(PolicyLockExplanation {
        reason: lock_reason_token(effective.lock_reason).to_owned(),
        policy_source_scope: SettingScope::AdminPolicyNarrowing.as_str().to_owned(),
        policy_source_label: policy_row.source_label.clone(),
        policy_source_ref: format!("policy:{}:{}", def.setting_id, policy_row.source_label),
        explanation: format!(
            "{} controls {} from {}.",
            policy_row.source_label,
            def.setting_id,
            SettingScope::AdminPolicyNarrowing.as_str()
        ),
    })
}

fn inspect_resolve_error(setting_id: &str, err: ResolveError) -> SettingsInspectError {
    SettingsInspectError::ResolveFailed {
        setting_id: setting_id.to_owned(),
        detail: err.to_string(),
    }
}

fn source_record_ref(setting_id: &str) -> String {
    format!("effective-setting:{setting_id}")
}

fn help_deep_link_ref(setting_id: &str) -> String {
    format!("settings://inspect/{setting_id}")
}

fn target_artifact_ref(scope: SettingScope, setting_id: &str) -> String {
    format!("settings://scope/{}/{setting_id}", scope.as_str())
}

fn is_high_risk_setting(def: &SettingDefinition) -> bool {
    matches!(def.sensitivity_class, SensitivityClass::HighRiskControl)
        || def.preview_class.requires_preview()
        || def.setting_id.starts_with("security.")
        || def.setting_id.starts_with("ai.")
        || def.setting_id.contains(".egress")
}

fn write_preview_verdict(
    outcome: &WriteAttemptOutcome,
    preview_class: Option<PreviewClass>,
    restart_posture: Option<RestartPosture>,
    preview_required: bool,
) -> String {
    if matches!(outcome.verdict, WriteIntent::Denied) {
        return "denied".to_owned();
    }
    if !matches!(outcome.verdict, WriteIntent::Allowed) {
        return outcome.verdict.as_str().to_owned();
    }
    match preview_class {
        Some(PreviewClass::RollbackCheckpointAndApprovalRequired) => {
            "allowed_with_rollback_checkpoint_and_approval".to_owned()
        }
        Some(PreviewClass::RollbackCheckpointRequired) => {
            "allowed_with_rollback_checkpoint".to_owned()
        }
        Some(PreviewClass::ManagedActionOnly) => "allowed_requires_approval_ticket".to_owned(),
        Some(PreviewClass::PreviewRequired) => "allowed_with_preview".to_owned(),
        _ if preview_required => "allowed_with_preview".to_owned(),
        _ if restart_posture
            .map(|posture| !matches!(posture, RestartPosture::NoRestart))
            .unwrap_or(false) =>
        {
            "allowed_with_restart".to_owned()
        }
        _ => "allowed".to_owned(),
    }
}

fn apply_state(
    verdict: &WriteIntent,
    preview_required: bool,
    checkpoint_required: bool,
    approval_required: bool,
    checkpoint_ref: Option<&String>,
    approval_ticket_ref: Option<&String>,
) -> String {
    if matches!(verdict, WriteIntent::Denied) {
        return "denied".to_owned();
    }
    if checkpoint_required && checkpoint_ref.is_none() {
        return "awaiting_checkpoint".to_owned();
    }
    if approval_required && approval_ticket_ref.is_none() {
        return "awaiting_approval".to_owned();
    }
    if preview_required {
        return "awaiting_preview".to_owned();
    }
    "ready_to_apply".to_owned()
}

fn write_denial_reason_token(reason: &WriteDenialReason) -> &'static str {
    match reason {
        WriteDenialReason::UnknownSetting { .. } => "setting_unknown_at_registry",
        WriteDenialReason::ScopeNotAllowed => "scope_not_allowed_for_setting",
        WriteDenialReason::ScopeBroadeningWouldWidenTrust => "scope_broadening_would_widen_trust",
        WriteDenialReason::PolicyLocked => "policy_locked_value",
        WriteDenialReason::PolicyConstrainedValue => "policy_constrained_value",
        WriteDenialReason::CapabilityDependencyUnmet => "capability_dependency_unmet",
        WriteDenialReason::PreviewRequiredNotAcknowledged => "preview_required_not_acknowledged",
        WriteDenialReason::RollbackCheckpointNotCreated => "rollback_checkpoint_not_created",
        WriteDenialReason::ApprovalTicketMissing => "approval_ticket_missing",
        WriteDenialReason::RestartRequiredNotAcknowledged => "restart_required_not_acknowledged",
        WriteDenialReason::ValidationFailed { .. } => "validation_failed",
        WriteDenialReason::RetiredSetting => "setting_retired",
        WriteDenialReason::ManagedModeOnly => "managed_mode_only",
        WriteDenialReason::ReadOnlySurface => "read_only_surface",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resolver::{PolicyConstraint, ScopeOverlay};
    use crate::schema::SchemaRegistry;

    fn resolver_with_policy_lock() -> EffectiveSettingsResolver {
        let mut resolver = EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog());
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
        resolver
    }

    fn context_for_egress(resolver: &EffectiveSettingsResolver) -> SettingsInspectionContext {
        let def = resolver
            .registry()
            .definition("security.ai.egress_policy")
            .unwrap();
        SettingsInspectionContext::new()
            .with_last_applied_revision("security.ai.egress_policy", "settings-rev:00042")
            .with_capability_state(
                &def.capability_dependencies[0],
                true,
                "identity_mode=managed_convenience",
            )
    }

    #[test]
    fn inspection_record_exposes_schema_effective_policy_and_revision() {
        let resolver = resolver_with_policy_lock();
        let context = context_for_egress(&resolver);
        let record = inspect_setting(&resolver, "security.ai.egress_policy", &context).unwrap();

        assert_eq!(record.definition.declared_type, "enum");
        assert_eq!(
            record.definition.migration_aliases,
            vec!["ai.network.egress_policy"]
        );
        assert_eq!(
            record.definition.preview_class,
            "rollback_checkpoint_and_approval_required"
        );
        assert_eq!(record.winning_scope, "admin_policy_narrowing");
        assert_eq!(
            record.winning_value_summary,
            "approved_hosted_providers_only"
        );
        assert_eq!(record.lock_state, "policy_locked");
        assert_eq!(record.lock_reason, "policy_pins_value");
        assert_eq!(record.validation_status, "valid");
        assert_eq!(record.restart_state.restart_posture, "restart_extensions");
        assert_eq!(record.capability_availability, "available");
        assert_eq!(
            record.last_applied_revision.as_deref(),
            Some("settings-rev:00042")
        );
        assert!(record.policy_lock_explanation.is_some());
        assert!(record
            .shadow_chain
            .iter()
            .any(|row| row.scope == "user_global" && row.relation == "capped"));
    }

    #[test]
    fn alias_inspection_lands_on_canonical_setting() {
        let resolver = resolver_with_policy_lock();
        let context = context_for_egress(&resolver);
        let record = inspect_setting(&resolver, "ai.network.egress_policy", &context).unwrap();
        assert_eq!(record.setting_id, "security.ai.egress_policy");
    }

    #[test]
    fn ui_cli_help_and_support_share_source_record() {
        let resolver = resolver_with_policy_lock();
        let context = context_for_egress(&resolver);
        let record = inspect_setting(&resolver, "security.ai.egress_policy", &context).unwrap();
        let ui = project_settings_ui(&record);
        let cli = project_cli_inspect(&record);
        let help = project_help_deep_link(&record);
        let export = project_support_export("support-export:settings:alpha", vec![record.clone()]);

        assert_eq!(ui.source_record_ref, record.source_record_ref);
        assert_eq!(cli.source_record_ref, record.source_record_ref);
        assert_eq!(help.source_record_ref, record.source_record_ref);
        assert_eq!(
            export.effective_settings[0].source_record_ref,
            record.source_record_ref
        );
        assert_eq!(export.policy_locked_count, 1);
    }

    #[test]
    fn high_risk_write_preview_is_scope_explicit_and_rollback_ready() {
        let resolver = resolver_with_policy_lock();
        let context = context_for_egress(&resolver);
        let preview = preview_write(
            &resolver,
            SettingWritePreviewRequest {
                setting_id: "security.ai.egress_policy".to_owned(),
                target_scope: SettingScope::Workspace,
                proposed_value: SettingValue::String("approved_hosted_providers_only".into()),
                actor_class: WriteActorClass::UserCommand,
                reason_class: WriteReasonClass::UserEdit,
                checkpoint_ref: Some("checkpoint:settings:egress:workspace:001".to_owned()),
                approval_ticket_ref: Some("approval:settings:egress:001".to_owned()),
            },
            &context,
        );

        assert_eq!(
            preview.verdict,
            "allowed_with_rollback_checkpoint_and_approval"
        );
        assert_eq!(preview.target_scope, "workspace");
        assert!(preview.destination_preview.scope_explicit);
        assert_eq!(preview.destination_preview.scope_broadening_verdict, "none");
        assert_eq!(preview.apply_state, "awaiting_preview");
        assert!(preview.checkpoint_required);
        assert!(preview.approval_required);
        assert!(preview.change_summary.rollback_ready);
        assert_eq!(
            preview.effective_before.as_ref().unwrap().setting_id,
            preview.setting_id
        );
        assert_eq!(
            preview.effective_after.as_ref().unwrap().setting_id,
            preview.setting_id
        );
    }

    #[test]
    fn policy_denied_write_preview_explains_lock_source() {
        let resolver = resolver_with_policy_lock();
        let context = context_for_egress(&resolver);
        let preview = preview_write(
            &resolver,
            SettingWritePreviewRequest {
                setting_id: "security.ai.egress_policy".to_owned(),
                target_scope: SettingScope::UserGlobal,
                proposed_value: SettingValue::String("any_hosted_provider".into()),
                actor_class: WriteActorClass::UserCommand,
                reason_class: WriteReasonClass::UserEdit,
                checkpoint_ref: None,
                approval_ticket_ref: None,
            },
            &context,
        );

        assert_eq!(preview.verdict, "denied");
        assert_eq!(
            preview.denial_reason.as_deref(),
            Some("policy_locked_value")
        );
        assert!(preview
            .effective_after
            .as_ref()
            .unwrap()
            .policy_lock_explanation
            .is_some());
    }
}
