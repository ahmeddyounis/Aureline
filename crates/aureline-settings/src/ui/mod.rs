//! Beta-grade settings UI projection.
//!
//! The UI module is the page-level surface above the inspector
//! records in [`crate::inspector`]. It adds the things a settings
//! reviewer expects from a beta-grade page without re-deriving
//! settings truth from raw overlays:
//!
//! - badges and labels for restart posture, sensitivity, lifecycle,
//!   and redaction;
//! - lock badges that name the owner (admin policy bundle, capability
//!   gate, surface restriction) and a remediation label callers can
//!   route to;
//! - scope-explicit write composers whose denial states still explain
//!   the owner and remediation path instead of failing silently;
//! - aggregate banners for restart-required and policy-locked rows;
//! - groupings by setting id prefix so the same page renders in
//!   user, workspace, profile, and policy scopes without re-querying
//!   the resolver per scope.
//!
//! The same projection feeds the CLI/headless renderer and the
//! support-export wrapper. UI rows, CLI rows, and support-export
//! rows always come from the same `source_record_ref` as the
//! inspector record they were built from, so UI, headless inspection,
//! and support reports report the same effective-value truth.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::inspector::{
    inspect_all_settings, inspect_setting, preview_write, EffectiveSettingInspectionRecord,
    InspectorCapabilityState, InspectorRestartState, InspectorShadowRow, PolicyLockExplanation,
    SettingWritePreviewRecord, SettingWritePreviewRequest, SettingsInspectError,
    SettingsInspectionContext, WriteDestinationPreview,
};
use crate::resolver::EffectiveSettingsResolver;

/// Settings UI beta projection schema version.
pub const SETTINGS_UI_BETA_SCHEMA_VERSION: u32 = 1;

const SHARED_CONTRACT_REF: &str = "settings:ui_beta:v1";

/// Beta UI projection over one inspected setting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsUiBetaRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta UI schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Ref back to the inspector record used to build this row.
    pub source_record_ref: String,
    /// Canonical setting id.
    pub setting_id: String,
    /// Group bucket derived from the setting id prefix.
    pub group_id: String,
    /// Short summary copied from the definition.
    pub summary: String,
    /// Redacted preview of the effective value.
    pub value_preview: String,
    /// Scope that supplied the effective value.
    pub winning_scope: String,
    /// Human-readable source pill label.
    pub winning_source_label: String,
    /// Scopes the setting may be written to.
    pub allowed_scopes: Vec<String>,
    /// Shadow chain copied from the inspector record.
    pub shadow_chain: Vec<InspectorShadowRow>,
    /// Lock badge for the row.
    pub lock_badge: LockBadge,
    /// Sensitivity badge for the row.
    pub sensitivity_badge: SensitivityBadge,
    /// Restart badge for the row.
    pub restart_badge: RestartBadge,
    /// Lifecycle badge derived from the definition's lifecycle label.
    pub lifecycle_badge: LifecycleBadge,
    /// Redaction badge for the row.
    pub redaction_badge: RedactionBadge,
    /// Capability availability summary.
    pub capability_availability: String,
    /// Detailed capability dependency rows.
    pub capability_dependencies: Vec<InspectorCapabilityState>,
    /// Help deep link the row routes to.
    pub help_deep_link_ref: String,
    /// Help doc ref from the definition.
    pub help_doc_ref: Option<String>,
    /// Validation state of the effective value.
    pub validation_status: String,
    /// Write affordance the row exposes to the user.
    pub write_affordance: WriteAffordance,
    /// Last applied revision known to the inspection context.
    pub last_applied_revision: Option<String>,
}

/// Lock badge attached to a beta UI row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockBadge {
    /// Lock-state token.
    pub state: String,
    /// Lock-reason token.
    pub reason: String,
    /// Short label for the row pill.
    pub label: String,
    /// Owner label that names who controls the lock.
    pub owner_label: String,
    /// Remediation label the user follows to unblock the change.
    pub remediation_label: String,
    /// Remediation action ref the surface routes to.
    pub remediation_action_ref: String,
    /// Policy source ref when an admin policy ceiling is active.
    pub policy_source_ref: Option<String>,
    /// Policy source scope when an admin policy ceiling is active.
    pub policy_source_scope: Option<String>,
}

/// Sensitivity badge attached to a beta UI row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SensitivityBadge {
    /// Sensitivity class token.
    pub class: String,
    /// Short label.
    pub label: String,
    /// Longer explanation for hover or detail surfaces.
    pub explanation: String,
}

/// Restart badge attached to a beta UI row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartBadge {
    /// Restart-posture token.
    pub posture: String,
    /// True when the value requires a restart or reload to fully apply.
    pub required: bool,
    /// Short label.
    pub label: String,
    /// Longer explanation for hover or detail surfaces.
    pub explanation: String,
}

/// Lifecycle badge attached to a beta UI row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleBadge {
    /// Lifecycle-label token.
    pub label_token: String,
    /// Short label.
    pub label: String,
}

/// Redaction badge attached to a beta UI row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionBadge {
    /// Redaction-class token.
    pub class: String,
    /// Short label.
    pub label: String,
}

/// Write affordance state for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteAffordance {
    /// Affordance state token.
    pub state: String,
    /// Short label.
    pub label: String,
    /// Longer explanation for hover or detail surfaces.
    pub explanation: String,
}

impl SettingsUiBetaRow {
    fn from_record(record: &EffectiveSettingInspectionRecord) -> Self {
        let group_id = group_id_for(&record.setting_id);
        let lock_badge = lock_badge_from_record(record);
        let sensitivity_badge = sensitivity_badge_from_record(record);
        let restart_badge = restart_badge_from_state(&record.restart_state);
        let lifecycle_badge = lifecycle_badge_for(&record.setting_id);
        let redaction_badge = redaction_badge_for(&record.definition.redaction_class);
        let write_affordance = write_affordance_for(record);
        Self {
            record_kind: "settings_ui_beta_row".to_owned(),
            schema_version: SETTINGS_UI_BETA_SCHEMA_VERSION,
            shared_contract_ref: SHARED_CONTRACT_REF.to_owned(),
            source_record_ref: record.source_record_ref.clone(),
            setting_id: record.setting_id.clone(),
            group_id,
            summary: record.definition.summary.clone(),
            value_preview: record.winning_value_summary.clone(),
            winning_scope: record.winning_scope.clone(),
            winning_source_label: record.source_label.clone(),
            allowed_scopes: record.definition.allowed_scopes.clone(),
            shadow_chain: record.shadow_chain.clone(),
            lock_badge,
            sensitivity_badge,
            restart_badge,
            lifecycle_badge,
            redaction_badge,
            capability_availability: record.capability_availability.clone(),
            capability_dependencies: record.capability_dependencies.clone(),
            help_deep_link_ref: help_deep_link_ref(&record.setting_id),
            help_doc_ref: record.definition.help_doc_ref.clone(),
            validation_status: record.validation_status.clone(),
            write_affordance,
            last_applied_revision: record.last_applied_revision.clone(),
        }
    }
}

/// One grouped block of UI rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsUiBetaGroup {
    /// Stable group id derived from the setting id prefix.
    pub group_id: String,
    /// Display label.
    pub group_label: String,
    /// Rows in the group, in deterministic setting-id order.
    pub rows: Vec<SettingsUiBetaRow>,
}

/// Aggregate restart-posture summary for a page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestartPostureSummary {
    /// Number of rows that require a restart or reload.
    pub restart_required_count: usize,
    /// Distinct restart postures present on the page.
    pub postures_present: Vec<String>,
    /// True when a process restart is needed for at least one row.
    pub any_process_restart: bool,
    /// Short banner label.
    pub label: String,
}

/// Aggregate policy-lock summary for a page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyLockSummary {
    /// Number of rows currently under an admin-policy lock.
    pub policy_locked_count: usize,
    /// Number of rows under an admin-policy ceiling that admits
    /// multiple values.
    pub policy_constrained_count: usize,
    /// Distinct policy source labels present on the page.
    pub policy_sources_present: Vec<String>,
    /// Short banner label.
    pub label: String,
}

/// Aggregate redaction summary for a page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionSummary {
    /// Number of rows whose literal payload is redacted from export.
    pub redacted_value_count: usize,
    /// Distinct redaction classes present on the page.
    pub classes_present: Vec<String>,
}

/// Beta UI page projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsUiBetaPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta UI schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Page id selected by the caller.
    pub page_id: String,
    /// Page label.
    pub page_label: String,
    /// Grouped rows.
    pub groups: Vec<SettingsUiBetaGroup>,
    /// Aggregate restart summary banner.
    pub restart_summary: RestartPostureSummary,
    /// Aggregate policy-lock summary banner.
    pub policy_summary: PolicyLockSummary,
    /// Aggregate redaction summary.
    pub redaction_summary: RedactionSummary,
}

/// Materialize a beta UI page from the live resolver.
pub fn project_settings_ui_beta_page(
    resolver: &EffectiveSettingsResolver,
    context: &SettingsInspectionContext,
    page_id: impl Into<String>,
    page_label: impl Into<String>,
) -> Result<SettingsUiBetaPage, SettingsInspectError> {
    let records = inspect_all_settings(resolver, context)?;
    Ok(project_page_from_records(records, page_id, page_label))
}

/// Build a beta UI page from already-materialized inspector records.
pub fn project_page_from_records(
    records: Vec<EffectiveSettingInspectionRecord>,
    page_id: impl Into<String>,
    page_label: impl Into<String>,
) -> SettingsUiBetaPage {
    let restart_summary = restart_summary_from_records(&records);
    let policy_summary = policy_summary_from_records(&records);
    let redaction_summary = redaction_summary_from_records(&records);

    let mut grouped: BTreeMap<String, Vec<SettingsUiBetaRow>> = BTreeMap::new();
    for record in &records {
        let row = SettingsUiBetaRow::from_record(record);
        grouped.entry(row.group_id.clone()).or_default().push(row);
    }
    let mut groups: Vec<SettingsUiBetaGroup> = grouped
        .into_iter()
        .map(|(group_id, rows)| SettingsUiBetaGroup {
            group_label: group_label_for(&group_id),
            group_id,
            rows,
        })
        .collect();
    for group in &mut groups {
        group.rows.sort_by(|a, b| a.setting_id.cmp(&b.setting_id));
    }

    SettingsUiBetaPage {
        record_kind: "settings_ui_beta_page".to_owned(),
        schema_version: SETTINGS_UI_BETA_SCHEMA_VERSION,
        shared_contract_ref: SHARED_CONTRACT_REF.to_owned(),
        page_id: page_id.into(),
        page_label: page_label.into(),
        groups,
        restart_summary,
        policy_summary,
        redaction_summary,
    }
}

/// Inspector pane (expanded row) projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsUiBetaInspectorPane {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta UI schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Ref back to the inspector record used to build this pane.
    pub source_record_ref: String,
    /// Canonical setting id.
    pub setting_id: String,
    /// Row projection rendered as the header of the pane.
    pub row: SettingsUiBetaRow,
    /// Definition summary block: declared type, allowed scopes, default,
    /// migration aliases, restart posture, and lifecycle.
    pub definition_summary: DefinitionSummary,
    /// Source chain table.
    pub source_chain: Vec<SourceChainRow>,
    /// Lock explanation block.
    pub lock_explanation: LockExplanation,
    /// Restart explanation block.
    pub restart_explanation: InspectorRestartState,
    /// Policy explanation copied from the inspector record.
    pub policy_lock_explanation: Option<PolicyLockExplanation>,
    /// Evidence refs from the definition.
    pub evidence_refs: Vec<String>,
}

/// Compact definition summary rendered above the source chain.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DefinitionSummary {
    /// Declared value type token.
    pub declared_type: String,
    /// Default value preview.
    pub default_value_preview: String,
    /// Allowed write scopes.
    pub allowed_scopes: Vec<String>,
    /// Migration aliases.
    pub migration_aliases: Vec<String>,
    /// Preview-class token.
    pub preview_class: String,
    /// Restart-posture token.
    pub restart_posture: String,
    /// Sensitivity-class token.
    pub sensitivity_class: String,
    /// Redaction-class token.
    pub redaction_class: String,
}

/// One row in the inspector pane source-chain table.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceChainRow {
    /// Scope token.
    pub scope: String,
    /// Human-readable source label.
    pub source_label: String,
    /// Redacted value preview.
    pub value_preview: String,
    /// Relation to the winner.
    pub relation: String,
    /// True when this is the winning row.
    pub winner: bool,
    /// Human-readable relation label for the table.
    pub relation_label: String,
}

/// Lock explanation block rendered by the inspector pane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockExplanation {
    /// Lock-state token.
    pub state: String,
    /// Lock-reason token.
    pub reason: String,
    /// Human-readable label.
    pub label: String,
    /// Owner label.
    pub owner_label: String,
    /// Remediation label.
    pub remediation_label: String,
    /// Remediation action ref.
    pub remediation_action_ref: String,
}

/// Build an inspector pane projection from an inspector record.
pub fn project_inspector_pane(
    record: &EffectiveSettingInspectionRecord,
) -> SettingsUiBetaInspectorPane {
    let row = SettingsUiBetaRow::from_record(record);
    let lock_explanation = lock_explanation_from_record(record);
    let source_chain = record
        .shadow_chain
        .iter()
        .map(SourceChainRow::from_inspector)
        .collect();
    SettingsUiBetaInspectorPane {
        record_kind: "settings_ui_beta_inspector_pane".to_owned(),
        schema_version: SETTINGS_UI_BETA_SCHEMA_VERSION,
        shared_contract_ref: SHARED_CONTRACT_REF.to_owned(),
        source_record_ref: record.source_record_ref.clone(),
        setting_id: record.setting_id.clone(),
        row,
        definition_summary: DefinitionSummary {
            declared_type: record.definition.declared_type.clone(),
            default_value_preview: record.definition.default_value_preview.clone(),
            allowed_scopes: record.definition.allowed_scopes.clone(),
            migration_aliases: record.definition.migration_aliases.clone(),
            preview_class: record.definition.preview_class.clone(),
            restart_posture: record.definition.restart_posture.clone(),
            sensitivity_class: record.definition.sensitivity_class.clone(),
            redaction_class: record.definition.redaction_class.clone(),
        },
        source_chain,
        lock_explanation,
        restart_explanation: record.restart_state.clone(),
        policy_lock_explanation: record.policy_lock_explanation.clone(),
        evidence_refs: record.definition.evidence_refs.clone(),
    }
}

impl SourceChainRow {
    fn from_inspector(row: &InspectorShadowRow) -> Self {
        let relation_label = match row.relation.as_str() {
            "winner" => "Winning source",
            "shadowed" => "Shadowed by a higher scope",
            "capped" => "Capped by policy ceiling",
            "policy_ceiling" => "Active policy ceiling",
            other => other,
        }
        .to_owned();
        Self {
            scope: row.scope.clone(),
            source_label: row.source_label.clone(),
            value_preview: row.value_preview.clone(),
            relation: row.relation.clone(),
            winner: row.winner,
            relation_label,
        }
    }
}

/// Scope-explicit write composer projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsUiBetaWriteComposer {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta UI schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Ref back to the underlying write-preview record.
    pub source_record_ref: String,
    /// Canonical setting id.
    pub setting_id: String,
    /// Scope the caller selected.
    pub target_scope: String,
    /// Effective destination preview.
    pub destination_preview: WriteDestinationPreview,
    /// Verdict token.
    pub verdict: String,
    /// Apply-state token.
    pub apply_state: String,
    /// Restart posture inherited from the definition.
    pub restart_posture: Option<String>,
    /// Preview class inherited from the definition.
    pub preview_class: Option<String>,
    /// True when a preview must be shown before apply.
    pub preview_required: bool,
    /// True when a rollback checkpoint is required before apply.
    pub checkpoint_required: bool,
    /// True when an approval ticket is required before apply.
    pub approval_required: bool,
    /// Denial explanation when the verdict is denied.
    pub denial_explanation: Option<DenialExplanation>,
    /// Underlying write preview record.
    pub write_preview: SettingWritePreviewRecord,
}

/// Denial explanation rendered when a write composer is blocked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DenialExplanation {
    /// Denial reason token.
    pub reason: String,
    /// Owner label that names who controls the lock.
    pub owner_label: String,
    /// Remediation label the user follows to unblock the change.
    pub remediation_label: String,
    /// Remediation action ref the surface routes to.
    pub remediation_action_ref: String,
    /// Policy source ref when an admin policy ceiling is active.
    pub policy_source_ref: Option<String>,
}

/// Build a write composer projection by routing through the inspector's
/// preview-write flow. The resolver is never mutated.
pub fn project_write_composer(
    resolver: &EffectiveSettingsResolver,
    request: SettingWritePreviewRequest,
    context: &SettingsInspectionContext,
) -> SettingsUiBetaWriteComposer {
    let preview = preview_write(resolver, request, context);
    write_composer_from_preview(preview)
}

/// Build a write composer projection from an existing write-preview record.
pub fn write_composer_from_preview(
    preview: SettingWritePreviewRecord,
) -> SettingsUiBetaWriteComposer {
    let denial_explanation = if preview.verdict == "denied" {
        Some(denial_explanation_from_preview(&preview))
    } else {
        None
    };
    SettingsUiBetaWriteComposer {
        record_kind: "settings_ui_beta_write_composer".to_owned(),
        schema_version: SETTINGS_UI_BETA_SCHEMA_VERSION,
        shared_contract_ref: SHARED_CONTRACT_REF.to_owned(),
        source_record_ref: format!("write-preview:{}", preview.setting_id),
        setting_id: preview.setting_id.clone(),
        target_scope: preview.target_scope.clone(),
        destination_preview: preview.destination_preview.clone(),
        verdict: preview.verdict.clone(),
        apply_state: preview.apply_state.clone(),
        restart_posture: preview.restart_posture.clone(),
        preview_class: preview.preview_class.clone(),
        preview_required: preview.preview_required,
        checkpoint_required: preview.checkpoint_required,
        approval_required: preview.approval_required,
        denial_explanation,
        write_preview: preview,
    }
}

/// Support-export projection that carries the beta UI rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SettingsUiBetaSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta UI schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by support tooling.
    pub shared_contract_ref: String,
    /// Export id supplied by the caller.
    pub export_id: String,
    /// Beta UI page included in the export.
    pub page: SettingsUiBetaPage,
    /// Canonical inspector records that back every row.
    pub effective_settings: Vec<EffectiveSettingInspectionRecord>,
}

/// Build a support export that carries both the beta UI page and the
/// inspector records the page was built from. The shared
/// `source_record_ref` on every row matches the inspector record at
/// the same setting id, so UI, CLI/headless, and support exports
/// report the same effective-value truth.
pub fn project_support_export(
    export_id: impl Into<String>,
    page: SettingsUiBetaPage,
    records: Vec<EffectiveSettingInspectionRecord>,
) -> SettingsUiBetaSupportExport {
    SettingsUiBetaSupportExport {
        record_kind: "settings_ui_beta_support_export".to_owned(),
        schema_version: SETTINGS_UI_BETA_SCHEMA_VERSION,
        shared_contract_ref: SHARED_CONTRACT_REF.to_owned(),
        export_id: export_id.into(),
        page,
        effective_settings: records,
    }
}

fn group_id_for(setting_id: &str) -> String {
    setting_id
        .split('.')
        .next()
        .unwrap_or(setting_id)
        .to_owned()
}

fn group_label_for(group_id: &str) -> String {
    match group_id {
        "editor" => "Editor",
        "shell" => "Shell",
        "ui" => "Appearance",
        "security" => "Security and trust",
        "vfs" => "Files and watchers",
        "ai" => "AI",
        other => other,
    }
    .to_owned()
}

fn lock_badge_from_record(record: &EffectiveSettingInspectionRecord) -> LockBadge {
    let label = lock_label_for(&record.lock_state);
    let (owner_label, remediation_label, remediation_action_ref) = lock_owner_remediation(record);
    let (policy_source_ref, policy_source_scope) = match &record.policy_lock_explanation {
        Some(explanation) => (
            Some(explanation.policy_source_ref.clone()),
            Some(explanation.policy_source_scope.clone()),
        ),
        None => (None, None),
    };
    LockBadge {
        state: record.lock_state.clone(),
        reason: record.lock_reason.clone(),
        label,
        owner_label,
        remediation_label,
        remediation_action_ref,
        policy_source_ref,
        policy_source_scope,
    }
}

fn lock_label_for(state: &str) -> String {
    match state {
        "unlocked" => "Unlocked",
        "policy_locked" => "Policy locked",
        "policy_constrained" => "Policy constrained",
        "read_only_surface" => "Read only here",
        other => other,
    }
    .to_owned()
}

fn lock_owner_remediation(record: &EffectiveSettingInspectionRecord) -> (String, String, String) {
    match record.lock_state.as_str() {
        "policy_locked" | "policy_constrained" => {
            let owner = record
                .policy_lock_explanation
                .as_ref()
                .map(|explanation| explanation.policy_source_label.clone())
                .unwrap_or_else(|| "Admin policy".to_owned());
            let action_ref = record
                .policy_lock_explanation
                .as_ref()
                .map(|explanation| explanation.policy_source_ref.clone())
                .unwrap_or_else(|| format!("policy:{}:unspecified", record.setting_id));
            (
                owner,
                "Contact your administrator to change the policy.".to_owned(),
                action_ref,
            )
        }
        "read_only_surface" => (
            "Surface restriction".to_owned(),
            "Open the matching scope to edit this setting.".to_owned(),
            format!("settings://scope/edit/{}", record.setting_id),
        ),
        _ => (
            "Open for editing".to_owned(),
            "Edit the value at the desired scope.".to_owned(),
            format!("settings://scope/edit/{}", record.setting_id),
        ),
    }
}

fn sensitivity_badge_from_record(record: &EffectiveSettingInspectionRecord) -> SensitivityBadge {
    let class = record.definition.sensitivity_class.clone();
    let (label, explanation) = match class.as_str() {
        "general_preference" => (
            "General preference",
            "Non-sensitive preference; safe to display and export.",
        ),
        "machine_local" => (
            "Machine local",
            "Local to this machine; not carried across machines by sync.",
        ),
        "high_risk_control" => (
            "High-risk control",
            "Trust, AI, network, or automation control. Preview and rollback before apply.",
        ),
        "credential_reference" => (
            "Credential reference",
            "Brokered credential handle; raw secret material never appears here.",
        ),
        _ => ("Sensitivity", "See definition row for sensitivity class."),
    };
    SensitivityBadge {
        class,
        label: label.to_owned(),
        explanation: explanation.to_owned(),
    }
}

fn restart_badge_from_state(state: &InspectorRestartState) -> RestartBadge {
    let (label, explanation) = match state.restart_posture.as_str() {
        "no_restart" => (
            "Applies live",
            "Live apply with no restart required.",
        ),
        "reload_workspace" => (
            "Reload workspace",
            "Reload the active workspace for the new value to fully apply.",
        ),
        "restart_extensions" => (
            "Restart extensions",
            "Restart extension or runtime hosts for the new value to fully apply.",
        ),
        "restart_shell" => (
            "Restart shell",
            "Restart the desktop shell process for the new value to fully apply.",
        ),
        _ => ("Restart posture", state.explanation.as_str()),
    };
    RestartBadge {
        posture: state.restart_posture.clone(),
        required: state.restart_required,
        label: label.to_owned(),
        explanation: explanation.to_owned(),
    }
}

fn lifecycle_badge_for(_setting_id: &str) -> LifecycleBadge {
    // The inspector record does not yet expose the definition's
    // lifecycle label; surfaces that need a richer badge can be
    // extended once the inspector forwards the lifecycle token.
    LifecycleBadge {
        label_token: "stable".to_owned(),
        label: "Stable".to_owned(),
    }
}

fn redaction_badge_for(class: &str) -> RedactionBadge {
    let label = match class {
        "none" => "Value shown",
        "ui_string_only" => "Value shown in UI only",
        "redact_value_preserve_shape" => "Value redacted (shape kept)",
        "redact_to_class_label" => "Value redacted to class label",
        "exclude_from_export" => "Excluded from export",
        _ => "Redaction",
    };
    RedactionBadge {
        class: class.to_owned(),
        label: label.to_owned(),
    }
}

fn write_affordance_for(record: &EffectiveSettingInspectionRecord) -> WriteAffordance {
    match record.lock_state.as_str() {
        "policy_locked" => WriteAffordance {
            state: "locked".to_owned(),
            label: "Locked by policy".to_owned(),
            explanation: "Admin policy pins this value. Contact your administrator.".to_owned(),
        },
        "policy_constrained" => WriteAffordance {
            state: "constrained".to_owned(),
            label: "Limited by policy".to_owned(),
            explanation: "Admin policy admits only a narrower value set.".to_owned(),
        },
        "read_only_surface" => WriteAffordance {
            state: "read_only".to_owned(),
            label: "Read only".to_owned(),
            explanation: "This surface cannot edit the setting in the current scope.".to_owned(),
        },
        _ => match record.definition.preview_class.as_str() {
            "rollback_checkpoint_and_approval_required" => WriteAffordance {
                state: "gated_with_rollback_checkpoint_and_approval".to_owned(),
                label: "Preview, rollback, and approval required".to_owned(),
                explanation:
                    "Editing requires a rollback checkpoint and approval before the value applies."
                        .to_owned(),
            },
            "rollback_checkpoint_required" => WriteAffordance {
                state: "gated_with_rollback_checkpoint".to_owned(),
                label: "Preview and rollback required".to_owned(),
                explanation: "Editing requires a rollback checkpoint before the value applies."
                    .to_owned(),
            },
            "managed_action_only" => WriteAffordance {
                state: "managed_action_only".to_owned(),
                label: "Managed action".to_owned(),
                explanation: "Only a managed authority can change this setting.".to_owned(),
            },
            "preview_required" => WriteAffordance {
                state: "gated_with_preview".to_owned(),
                label: "Preview required".to_owned(),
                explanation: "A change preview is shown before the value applies.".to_owned(),
            },
            _ => WriteAffordance {
                state: "editable".to_owned(),
                label: "Edit".to_owned(),
                explanation: "Editing applies the value at the selected scope.".to_owned(),
            },
        },
    }
}

fn lock_explanation_from_record(record: &EffectiveSettingInspectionRecord) -> LockExplanation {
    let badge = lock_badge_from_record(record);
    LockExplanation {
        state: badge.state,
        reason: badge.reason,
        label: badge.label,
        owner_label: badge.owner_label,
        remediation_label: badge.remediation_label,
        remediation_action_ref: badge.remediation_action_ref,
    }
}

fn denial_explanation_from_preview(preview: &SettingWritePreviewRecord) -> DenialExplanation {
    let reason = preview
        .denial_reason
        .clone()
        .unwrap_or_else(|| "unknown_denial".to_owned());
    let effective_after = preview.effective_after.as_ref();
    let policy_source_ref = effective_after
        .and_then(|record| record.policy_lock_explanation.as_ref())
        .map(|explanation| explanation.policy_source_ref.clone());

    let (owner_label, remediation_label, remediation_action_ref) = match reason.as_str() {
        "policy_locked_value" | "policy_constrained_value" => {
            let owner = effective_after
                .and_then(|record| record.policy_lock_explanation.as_ref())
                .map(|explanation| explanation.policy_source_label.clone())
                .unwrap_or_else(|| "Admin policy".to_owned());
            let action_ref = policy_source_ref
                .clone()
                .unwrap_or_else(|| format!("policy:{}:unspecified", preview.setting_id));
            (
                owner,
                "Contact your administrator to change the policy.".to_owned(),
                action_ref,
            )
        }
        "scope_not_allowed_for_setting" => (
            "Definition row".to_owned(),
            "Pick a scope from the definition's allowed-scopes set.".to_owned(),
            format!("settings://scope/edit/{}", preview.setting_id),
        ),
        "validation_failed" => (
            "Value validation".to_owned(),
            "Adjust the value to satisfy the declared type and bounds.".to_owned(),
            format!("settings://scope/edit/{}", preview.setting_id),
        ),
        "setting_unknown_at_registry" => (
            "Schema registry".to_owned(),
            "The setting id is not registered. Check the registry.".to_owned(),
            format!("docs:settings:{}", preview.setting_id),
        ),
        "setting_retired" => (
            "Schema registry".to_owned(),
            "This setting is retired. Migrate to its successor if one exists.".to_owned(),
            format!("docs:settings:{}", preview.setting_id),
        ),
        _ => (
            "Settings authority".to_owned(),
            "Review the denial reason and adjust the request.".to_owned(),
            format!("settings://scope/edit/{}", preview.setting_id),
        ),
    };

    DenialExplanation {
        reason,
        owner_label,
        remediation_label,
        remediation_action_ref,
        policy_source_ref,
    }
}

fn restart_summary_from_records(records: &[EffectiveSettingInspectionRecord]) -> RestartPostureSummary {
    let mut postures: BTreeMap<String, ()> = BTreeMap::new();
    let mut required = 0usize;
    let mut any_process_restart = false;
    for record in records {
        let posture = &record.restart_state.restart_posture;
        postures.insert(posture.clone(), ());
        if record.restart_state.restart_required {
            required += 1;
        }
        if posture == "restart_shell" || posture == "restart_extensions" {
            any_process_restart = true;
        }
    }
    let postures_present: Vec<String> = postures.into_keys().collect();
    let label = if required == 0 {
        "All values apply live".to_owned()
    } else if any_process_restart {
        format!("{required} value(s) require a restart")
    } else {
        format!("{required} value(s) require a reload")
    };
    RestartPostureSummary {
        restart_required_count: required,
        postures_present,
        any_process_restart,
        label,
    }
}

fn policy_summary_from_records(records: &[EffectiveSettingInspectionRecord]) -> PolicyLockSummary {
    let mut locked = 0usize;
    let mut constrained = 0usize;
    let mut sources: BTreeMap<String, ()> = BTreeMap::new();
    for record in records {
        match record.lock_state.as_str() {
            "policy_locked" => locked += 1,
            "policy_constrained" => constrained += 1,
            _ => {}
        }
        if let Some(explanation) = &record.policy_lock_explanation {
            sources.insert(explanation.policy_source_label.clone(), ());
        }
    }
    let sources_present: Vec<String> = sources.into_keys().collect();
    let label = if locked == 0 && constrained == 0 {
        "No active policy locks".to_owned()
    } else {
        format!("{locked} locked, {constrained} constrained by policy")
    };
    PolicyLockSummary {
        policy_locked_count: locked,
        policy_constrained_count: constrained,
        policy_sources_present: sources_present,
        label,
    }
}

fn redaction_summary_from_records(records: &[EffectiveSettingInspectionRecord]) -> RedactionSummary {
    let mut redacted = 0usize;
    let mut classes: BTreeMap<String, ()> = BTreeMap::new();
    for record in records {
        if record.winning_value.is_none() {
            redacted += 1;
        }
        classes.insert(record.definition.redaction_class.clone(), ());
    }
    RedactionSummary {
        redacted_value_count: redacted,
        classes_present: classes.into_keys().collect(),
    }
}

fn help_deep_link_ref(setting_id: &str) -> String {
    format!("settings://ui/beta/{setting_id}")
}

/// Materialize a beta UI inspector pane for one setting.
pub fn inspect_setting_pane(
    resolver: &EffectiveSettingsResolver,
    setting_id_or_alias: &str,
    context: &SettingsInspectionContext,
) -> Result<SettingsUiBetaInspectorPane, SettingsInspectError> {
    let record = inspect_setting(resolver, setting_id_or_alias, context)?;
    Ok(project_inspector_pane(&record))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inspector::{WriteActorClass, WriteReasonClass};
    use crate::resolver::{PolicyConstraint, ScopeOverlay};
    use crate::schema::{SchemaRegistry, SettingScope, SettingValue};

    fn seeded_resolver() -> EffectiveSettingsResolver {
        let mut resolver = EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog());
        let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "User settings");
        user.set_value("editor.tab_size", SettingValue::Integer(8));
        user.set_value(
            "security.ai.egress_policy",
            SettingValue::String("any_hosted_provider".into()),
        );
        resolver.set_overlay(user).unwrap();
        let mut workspace = ScopeOverlay::new(SettingScope::Workspace, "Workspace settings");
        workspace.set_value("editor.tab_size", SettingValue::Integer(2));
        resolver.set_overlay(workspace).unwrap();
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

    fn seeded_context(resolver: &EffectiveSettingsResolver) -> SettingsInspectionContext {
        let mut context = SettingsInspectionContext::new()
            .with_last_applied_revision("editor.tab_size", "settings-rev:00041")
            .with_last_applied_revision("security.ai.egress_policy", "settings-rev:00042");
        for def in resolver.registry().definitions() {
            for dependency in &def.capability_dependencies {
                context = context.with_capability_state(dependency, true, "available");
            }
        }
        context
    }

    #[test]
    fn beta_row_carries_lock_owner_and_remediation_for_policy_locked_setting() {
        let resolver = seeded_resolver();
        let context = seeded_context(&resolver);
        let record =
            inspect_setting(&resolver, "security.ai.egress_policy", &context).unwrap();
        let row = SettingsUiBetaRow::from_record(&record);

        assert_eq!(row.setting_id, "security.ai.egress_policy");
        assert_eq!(row.group_id, "security");
        assert_eq!(row.winning_scope, "admin_policy_narrowing");
        assert_eq!(row.lock_badge.state, "policy_locked");
        assert_eq!(row.lock_badge.owner_label, "Admin policy bundle v3");
        assert!(row
            .lock_badge
            .remediation_label
            .to_lowercase()
            .contains("administrator"));
        assert!(row.lock_badge.policy_source_ref.is_some());
        assert_eq!(row.write_affordance.state, "locked");
        assert!(row.restart_badge.required);
        assert_eq!(row.restart_badge.posture, "restart_extensions");
        assert_eq!(row.sensitivity_badge.class, "high_risk_control");
        assert_eq!(row.redaction_badge.class, "ui_string_only");
    }

    #[test]
    fn beta_page_groups_rows_and_aggregates_restart_and_policy_banners() {
        let resolver = seeded_resolver();
        let context = seeded_context(&resolver);
        let page = project_settings_ui_beta_page(
            &resolver,
            &context,
            "all",
            "All settings",
        )
        .unwrap();

        assert_eq!(page.page_id, "all");
        assert!(page.groups.iter().any(|group| group.group_id == "editor"));
        assert!(page.groups.iter().any(|group| group.group_id == "security"));
        assert!(page.policy_summary.policy_locked_count >= 1);
        assert!(page
            .policy_summary
            .policy_sources_present
            .iter()
            .any(|label| label.contains("Admin policy")));
        assert!(page.restart_summary.restart_required_count >= 1);
        assert!(page
            .restart_summary
            .postures_present
            .contains(&"restart_extensions".to_owned()));
    }

    #[test]
    fn inspector_pane_renders_source_chain_with_labelled_relations() {
        let resolver = seeded_resolver();
        let context = seeded_context(&resolver);
        let pane = inspect_setting_pane(
            &resolver,
            "security.ai.egress_policy",
            &context,
        )
        .unwrap();

        assert_eq!(pane.setting_id, "security.ai.egress_policy");
        assert!(pane
            .source_chain
            .iter()
            .any(|row| row.relation == "winner" && row.relation_label == "Winning source"));
        assert!(pane.source_chain.iter().any(|row| row.relation == "capped"));
        assert_eq!(pane.lock_explanation.state, "policy_locked");
        assert!(pane.policy_lock_explanation.is_some());
        assert_eq!(pane.restart_explanation.restart_posture, "restart_extensions");
    }

    #[test]
    fn write_composer_denial_carries_owner_and_remediation() {
        let resolver = seeded_resolver();
        let context = seeded_context(&resolver);
        let composer = project_write_composer(
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

        assert_eq!(composer.verdict, "denied");
        let denial = composer.denial_explanation.expect("denied composer");
        assert_eq!(denial.reason, "policy_locked_value");
        assert!(denial.owner_label.contains("Admin policy"));
        assert!(denial.policy_source_ref.is_some());
        assert!(denial.remediation_label.to_lowercase().contains("administrator"));
    }

    #[test]
    fn write_composer_scope_explicit_high_risk_path_carries_preview_and_rollback() {
        let resolver = seeded_resolver();
        let context = seeded_context(&resolver);
        let composer = project_write_composer(
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

        assert_eq!(composer.target_scope, "workspace");
        assert!(composer.destination_preview.scope_explicit);
        assert_eq!(composer.destination_preview.scope_broadening_verdict, "none");
        assert_eq!(composer.apply_state, "awaiting_preview");
        assert!(composer.preview_required);
        assert!(composer.checkpoint_required);
        assert!(composer.approval_required);
        assert!(composer.denial_explanation.is_none());
    }

    #[test]
    fn support_export_carries_page_and_inspector_records_with_shared_refs() {
        let resolver = seeded_resolver();
        let context = seeded_context(&resolver);
        let records = inspect_all_settings(&resolver, &context).unwrap();
        let page = project_page_from_records(records.clone(), "all", "All settings");
        let export = project_support_export(
            "support-export:settings-ui-beta:001",
            page.clone(),
            records.clone(),
        );

        for row in page.groups.iter().flat_map(|group| group.rows.iter()) {
            let matching = export
                .effective_settings
                .iter()
                .find(|record| record.source_record_ref == row.source_record_ref);
            assert!(
                matching.is_some(),
                "row {} should match an inspector record by source_record_ref",
                row.setting_id
            );
        }
        assert_eq!(export.shared_contract_ref, SHARED_CONTRACT_REF);
        assert_eq!(
            export.page.shared_contract_ref,
            SHARED_CONTRACT_REF.to_owned()
        );
    }

    #[test]
    fn beta_row_round_trips_through_serde() {
        let resolver = seeded_resolver();
        let context = seeded_context(&resolver);
        let record = inspect_setting(&resolver, "editor.tab_size", &context).unwrap();
        let row = SettingsUiBetaRow::from_record(&record);
        let payload = serde_json::to_string(&row).unwrap();
        let restored: SettingsUiBetaRow = serde_json::from_str(&payload).unwrap();
        assert_eq!(row, restored);
    }
}
