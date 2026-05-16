//! Extension settings and permission inspector projections.
//!
//! This module is the shell-owned consumer for extension inspector truth:
//! permission rows come from the extension manifest and effective-permission
//! contracts, runtime placement comes from the runtime admission contract,
//! and extension-local settings stay row-based with source attribution and
//! diff rows instead of opaque blobs. The support-export projection is built
//! from the same rows the inspector renders, so support tooling can reproduce
//! the same truth without raw secret values.

use serde::{Deserialize, Serialize};

use aureline_extensions::{
    capability_class_for_scope, compute_effective_permission_baseline, decide_manifest_install,
    evaluate_runtime_v1_beta_contract, project_permission_manifest,
    project_permission_manifest_support_export, project_runtime_v1_beta_support_export,
    CapabilityClassClass, DeclaredVsEffectiveDiffEntry, EffectivePermissionBaselineRecord,
    EffectivePermissionDiffClass, ExtensionLifecycleStateClass, ExtensionManifestBaselineRecord,
    HostContractFamilyClass, HostPlacementClass, HostSupervisionClass, ManifestOriginSourceClass,
    ManifestScopeCompletenessClass, PermissionManifestRecord, PermissionScopeClass,
    PermissionScopeEntry, PolicyPackNarrowing, PublisherLifecycleStateClass,
    PublisherTrustTierClass, RedactionClass, RestartPostureClass, RuntimeAdmissionDecisionClass,
    RuntimeAdmissionReasonClass, RuntimeLifecycleStateClass, RuntimeV1BetaContractInput,
    SdkAlignmentClass, SummaryFreshnessClass, EXTENSION_MANIFEST_BASELINE_RECORD_KIND,
    EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
};

#[cfg(test)]
mod tests;

/// Record kind for [`ExtensionInspectorPage`] payloads.
pub const EXTENSION_INSPECTOR_PAGE_RECORD_KIND: &str = "extension_inspector_page";

/// Record kind for [`ExtensionPermissionInspector`] payloads.
pub const EXTENSION_PERMISSION_INSPECTOR_RECORD_KIND: &str = "extension_permission_inspector";

/// Record kind for [`ExtensionSettingsInspector`] payloads.
pub const EXTENSION_SETTINGS_INSPECTOR_RECORD_KIND: &str = "extension_settings_inspector";

/// Record kind for [`ExtensionInspectorSupportExport`] payloads.
pub const EXTENSION_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND: &str =
    "extension_inspector_support_export";

/// Schema version for extension inspector payloads.
pub const EXTENSION_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by shell, CLI/headless, docs, and support export.
pub const EXTENSION_INSPECTOR_SHARED_CONTRACT_REF: &str = "shell:extension_inspectors_beta:v1";

const SEEDED_EXTENSION_IDENTITY: &str = "dev.aureline.samples/wasm-notes";
const SEEDED_EXTENSION_VERSION: &str = "1.0.0-beta.1";
const SEEDED_GENERATED_AT: &str = "2026-05-16T18:30:00Z";

/// User-visible disposition for a permission row after effective policy is applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionCapabilityDispositionClass {
    /// The capability is declared and effectively available.
    Granted,
    /// The capability is blocked and absent from the effective permission set.
    Denied,
    /// The capability is present only under a policy lock or policy narrowing.
    PolicyLocked,
}

/// Lock reason attached to policy-shaped permission rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionCapabilityLockReasonClass {
    /// No policy lock affected this row.
    None,
    /// Policy narrowed the row without denying it.
    NarrowedByPolicy,
    /// Policy requires a step-up approval before use.
    StepUpRequired,
    /// Policy denied the row.
    DeniedByPolicy,
    /// A requested permission was not declared and was blocked.
    WideningBlocked,
}

/// Source class for one extension-local settings value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionSettingSourceScopeClass {
    /// Extension package default.
    ExtensionDefault,
    /// User-global extension settings.
    UserGlobal,
    /// Machine-local extension settings.
    MachineSpecific,
    /// Workspace extension settings.
    Workspace,
    /// Admin policy ceiling for extension settings.
    AdminPolicy,
    /// Current-session override.
    Session,
}

/// Relation between a source row and the effective setting value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionSettingSourceRelationClass {
    /// This source supplies the effective value.
    Winner,
    /// This source is shadowed by a higher-precedence source.
    Shadowed,
    /// This source is the active policy ceiling.
    PolicyCeiling,
}

/// Lock state for an extension-local setting row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionSettingLockStateClass {
    /// The setting can be changed at an allowed scope.
    Unlocked,
    /// Admin policy pins or constrains the value.
    PolicyLocked,
}

/// Field-level diff class for extension-local settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionSettingDiffClass {
    /// Value was introduced at the effective source.
    Added,
    /// Value changed between sources.
    Changed,
    /// Value is locked by policy.
    PolicyLocked,
    /// A secret-bearing value is represented by a brokered handle only.
    RedactedSecretHandle,
}

/// Redaction class for extension-local setting rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionSettingRedactionClass {
    /// Literal metadata-safe value is exportable.
    MetadataSafe,
    /// Value body is redacted but its shape is preserved.
    ValueRedacted,
    /// Secret-bearing value is replaced by a brokered handle summary.
    SecretValueRedacted,
}

/// Summary counts for a permission inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionPermissionInspectorSummary {
    /// Total permission rows.
    pub row_count: usize,
    /// Rows effectively granted.
    pub granted_count: usize,
    /// Rows denied by policy or widening guardrails.
    pub denied_count: usize,
    /// Rows locked or narrowed by policy.
    pub policy_locked_count: usize,
    /// Rows that require a step-up approval.
    pub step_up_required_count: usize,
}

/// Source refs consumed by the permission inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionPermissionInspectorSourceRefs {
    /// Manifest baseline record ref.
    pub manifest_baseline_ref: String,
    /// Permission manifest record ref.
    pub permission_manifest_ref: String,
    /// Effective-permission summary ref.
    pub effective_permission_summary_ref: String,
    /// Runtime admission contract ref.
    pub runtime_contract_ref: String,
    /// Permission support-export ref derived from the same permission manifest.
    pub permission_support_export_ref: String,
    /// Runtime support-export ref derived from the same runtime contract.
    pub runtime_support_export_ref: String,
}

/// One permission row shown by the inspector and replayed by support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionPermissionInspectorRow {
    /// Stable row id.
    pub permission_row_id: String,
    /// Capability class for the permission scope.
    pub capability_class_class: CapabilityClassClass,
    /// Permission scope class.
    pub scope_class: PermissionScopeClass,
    /// Redaction-safe permission target.
    pub scope_target: String,
    /// Declared scope constraint, when any.
    pub declared_scope_constraint: Option<String>,
    /// Effective scope constraint after policy, when any.
    pub effective_scope_constraint: Option<String>,
    /// Human-readable purpose label from the manifest.
    pub rationale_label: String,
    /// Effective disposition class.
    pub disposition_class: ExtensionCapabilityDispositionClass,
    /// Declared-vs-effective diff class.
    pub diff_class: EffectivePermissionDiffClass,
    /// Lock reason class.
    pub lock_reason_class: ExtensionCapabilityLockReasonClass,
    /// Policy-pack refs that shaped this row.
    pub policy_narrowing_refs: Vec<String>,
    /// Export-safe explanation for UI, CLI/headless, and support export.
    pub export_safe_explanation: String,
    /// True when the support export must include the row.
    pub support_export_required: bool,
    /// Redaction class for this row.
    pub redaction_class: RedactionClass,
}

/// Permission inspector that joins permission, policy, and runtime truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionPermissionInspector {
    /// Record discriminator.
    pub record_kind: String,
    /// Inspector schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by all surfaces.
    pub shared_contract_ref: String,
    /// Stable inspector id.
    pub inspector_id: String,
    /// Extension identity ref.
    pub extension_identity_ref: String,
    /// Extension version.
    pub extension_version: String,
    /// Source records consumed by the inspector.
    pub source_refs: ExtensionPermissionInspectorSourceRefs,
    /// Runtime host placement class.
    pub host_placement_class: HostPlacementClass,
    /// Runtime host supervision class.
    pub host_supervision_class: HostSupervisionClass,
    /// Runtime lifecycle state.
    pub lifecycle_state_class: RuntimeLifecycleStateClass,
    /// Runtime admission decision.
    pub admission_decision_class: RuntimeAdmissionDecisionClass,
    /// Runtime admission reason.
    pub admission_reason_class: RuntimeAdmissionReasonClass,
    /// Permission rows.
    pub rows: Vec<ExtensionPermissionInspectorRow>,
    /// Summary counts.
    pub summary: ExtensionPermissionInspectorSummary,
}

/// One source row for an extension-local setting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionSettingSourceRow {
    /// Stable source row ref.
    pub source_ref: String,
    /// Source scope class.
    pub source_scope_class: ExtensionSettingSourceScopeClass,
    /// Human-readable source label.
    pub source_label: String,
    /// Redacted preview of the value from this source.
    pub value_preview: String,
    /// Relation to the effective value.
    pub relation_class: ExtensionSettingSourceRelationClass,
    /// Stable attribution ref for support/export correlation.
    pub source_attribution_ref: String,
    /// True when raw secret bytes were exported by this row.
    pub raw_secret_value_exported: bool,
}

/// One field-level diff row for an extension-local setting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionSettingDiffRow {
    /// Stable diff row id.
    pub diff_row_id: String,
    /// Field path inside the setting value.
    pub field_path: String,
    /// Diff class.
    pub diff_class: ExtensionSettingDiffClass,
    /// Prior source ref, when present.
    pub prior_source_ref: Option<String>,
    /// Effective source ref.
    pub effective_source_ref: String,
    /// Prior value preview.
    pub prior_value_preview: Option<String>,
    /// Effective value preview.
    pub effective_value_preview: String,
    /// Stable attribution ref for support/export correlation.
    pub source_attribution_ref: String,
    /// True when raw secret bytes were exported by this diff row.
    pub raw_secret_value_exported: bool,
}

/// One extension-local setting row shown by the inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionSettingInspectorRow {
    /// Stable setting row id.
    pub setting_row_id: String,
    /// Extension-local setting id.
    pub setting_id: String,
    /// Declared value-type token.
    pub value_type: String,
    /// Redacted effective value preview.
    pub effective_value_preview: String,
    /// Winning source scope.
    pub winning_scope_class: ExtensionSettingSourceScopeClass,
    /// Winning source ref.
    pub winning_source_ref: String,
    /// Source rows, ordered by precedence.
    pub source_chain: Vec<ExtensionSettingSourceRow>,
    /// Field-level diff rows.
    pub diff_rows: Vec<ExtensionSettingDiffRow>,
    /// Lock state for this row.
    pub lock_state_class: ExtensionSettingLockStateClass,
    /// Policy source ref when policy is active.
    pub policy_source_ref: Option<String>,
    /// Redaction class.
    pub redaction_class: ExtensionSettingRedactionClass,
    /// True when raw secret bytes were exported by this setting row.
    pub raw_secret_value_exported: bool,
    /// Export-safe row summary.
    pub export_safe_summary: String,
}

/// Summary counts for extension-local settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionSettingsInspectorSummary {
    /// Total settings rows.
    pub row_count: usize,
    /// Settings with policy locks.
    pub policy_locked_count: usize,
    /// Settings whose value bodies are redacted.
    pub redacted_value_count: usize,
    /// Settings that include at least one field-level diff.
    pub diffable_setting_count: usize,
}

/// Extension-local settings inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionSettingsInspector {
    /// Record discriminator.
    pub record_kind: String,
    /// Inspector schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by all surfaces.
    pub shared_contract_ref: String,
    /// Stable inspector id.
    pub inspector_id: String,
    /// Extension identity ref.
    pub extension_identity_ref: String,
    /// Extension version.
    pub extension_version: String,
    /// Stable source ref for the extension-local settings store.
    pub settings_store_ref: String,
    /// Setting rows.
    pub rows: Vec<ExtensionSettingInspectorRow>,
    /// Summary counts.
    pub summary: ExtensionSettingsInspectorSummary,
}

/// Page-level summary for the bounded inspector set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionInspectorPageSummary {
    /// Permission row count.
    pub permission_row_count: usize,
    /// Setting row count.
    pub setting_row_count: usize,
    /// Denied permission count.
    pub denied_permission_count: usize,
    /// Policy-locked permission count.
    pub policy_locked_permission_count: usize,
    /// Policy-locked setting count.
    pub policy_locked_setting_count: usize,
    /// Redacted setting count.
    pub redacted_setting_count: usize,
}

/// Shell page that contains the permission and settings inspectors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionInspectorPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Inspector schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by shell, CLI/headless, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Page label for headless/support consumers.
    pub page_label: String,
    /// Extension identity ref.
    pub extension_identity_ref: String,
    /// Extension version.
    pub extension_version: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Permission inspector.
    pub permission_inspector: ExtensionPermissionInspector,
    /// Settings inspector.
    pub settings_inspector: ExtensionSettingsInspector,
    /// Page summary counts.
    pub summary: ExtensionInspectorPageSummary,
}

/// Permission row shape carried by support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionPermissionSupportExportRow {
    /// Stable permission row id.
    pub permission_row_id: String,
    /// Capability class.
    pub capability_class_class: CapabilityClassClass,
    /// Permission scope class.
    pub scope_class: PermissionScopeClass,
    /// Redaction-safe permission target.
    pub scope_target: String,
    /// Effective disposition.
    pub disposition_class: ExtensionCapabilityDispositionClass,
    /// Declared-vs-effective diff class.
    pub diff_class: EffectivePermissionDiffClass,
    /// Policy refs that shaped this row.
    pub policy_narrowing_refs: Vec<String>,
    /// Export-safe explanation.
    pub export_safe_explanation: String,
    /// Redaction class.
    pub redaction_class: RedactionClass,
}

/// Settings row shape carried by support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionSettingSupportExportRow {
    /// Stable setting row id.
    pub setting_row_id: String,
    /// Extension-local setting id.
    pub setting_id: String,
    /// Redacted effective value preview.
    pub effective_value_preview: String,
    /// Winning source scope.
    pub winning_scope_class: ExtensionSettingSourceScopeClass,
    /// Winning source ref.
    pub winning_source_ref: String,
    /// Source rows needed to reproduce attribution.
    pub source_chain: Vec<ExtensionSettingSourceRow>,
    /// Diff rows needed to reproduce field-level changes.
    pub diff_rows: Vec<ExtensionSettingDiffRow>,
    /// Lock state.
    pub lock_state_class: ExtensionSettingLockStateClass,
    /// Policy source ref when policy is active.
    pub policy_source_ref: Option<String>,
    /// Redaction class.
    pub redaction_class: ExtensionSettingRedactionClass,
    /// True when raw secret bytes were exported by this row.
    pub raw_secret_value_exported: bool,
}

/// Fingerprint proving support export parity with the page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionInspectorParityFingerprint {
    /// Permission row ids in page order.
    pub permission_row_ids: Vec<String>,
    /// Setting row ids in page order.
    pub setting_row_ids: Vec<String>,
    /// Permission summary copied from the page.
    pub permission_summary: ExtensionPermissionInspectorSummary,
    /// Settings summary copied from the page.
    pub settings_summary: ExtensionSettingsInspectorSummary,
    /// Runtime host placement copied from the page.
    pub host_placement_class: HostPlacementClass,
    /// Runtime lifecycle copied from the page.
    pub lifecycle_state_class: RuntimeLifecycleStateClass,
}

/// Support-export packet for the bounded extension inspector set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionInspectorSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Inspector schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by support tooling.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Page ref whose rows produced this export.
    pub page_ref: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Extension identity ref.
    pub extension_identity_ref: String,
    /// Extension version.
    pub extension_version: String,
    /// Permission inspector ref.
    pub permission_inspector_ref: String,
    /// Settings inspector ref.
    pub settings_inspector_ref: String,
    /// Runtime host placement copied from the page.
    pub host_placement_class: HostPlacementClass,
    /// Runtime lifecycle copied from the page.
    pub lifecycle_state_class: RuntimeLifecycleStateClass,
    /// Permission rows copied from the page in export-safe form.
    pub permission_rows: Vec<ExtensionPermissionSupportExportRow>,
    /// Setting rows copied from the page in export-safe form.
    pub setting_rows: Vec<ExtensionSettingSupportExportRow>,
    /// True when any raw secret value was exported.
    pub raw_secret_values_exported: bool,
    /// Parity fingerprint copied from the page.
    pub parity_fingerprint: ExtensionInspectorParityFingerprint,
}

impl ExtensionInspectorSupportExport {
    /// Builds a support export from the same rows the inspector page renders.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: &ExtensionInspectorPage,
    ) -> Self {
        let permission_rows = page
            .permission_inspector
            .rows
            .iter()
            .map(ExtensionPermissionSupportExportRow::from_permission_row)
            .collect();
        let setting_rows: Vec<ExtensionSettingSupportExportRow> = page
            .settings_inspector
            .rows
            .iter()
            .map(ExtensionSettingSupportExportRow::from_setting_row)
            .collect();
        let raw_secret_values_exported =
            setting_rows.iter().any(|row| row.raw_secret_value_exported)
                || setting_rows.iter().any(|row| {
                    row.source_chain
                        .iter()
                        .any(|source| source.raw_secret_value_exported)
                        || row
                            .diff_rows
                            .iter()
                            .any(|diff| diff.raw_secret_value_exported)
                });
        Self {
            record_kind: EXTENSION_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: EXTENSION_INSPECTOR_SCHEMA_VERSION,
            shared_contract_ref: EXTENSION_INSPECTOR_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            page_ref: page.page_id.clone(),
            generated_at: generated_at.into(),
            extension_identity_ref: page.extension_identity_ref.clone(),
            extension_version: page.extension_version.clone(),
            permission_inspector_ref: page.permission_inspector.inspector_id.clone(),
            settings_inspector_ref: page.settings_inspector.inspector_id.clone(),
            host_placement_class: page.permission_inspector.host_placement_class,
            lifecycle_state_class: page.permission_inspector.lifecycle_state_class,
            permission_rows,
            setting_rows,
            raw_secret_values_exported,
            parity_fingerprint: ExtensionInspectorParityFingerprint {
                permission_row_ids: page
                    .permission_inspector
                    .rows
                    .iter()
                    .map(|row| row.permission_row_id.clone())
                    .collect(),
                setting_row_ids: page
                    .settings_inspector
                    .rows
                    .iter()
                    .map(|row| row.setting_row_id.clone())
                    .collect(),
                permission_summary: page.permission_inspector.summary.clone(),
                settings_summary: page.settings_inspector.summary.clone(),
                host_placement_class: page.permission_inspector.host_placement_class,
                lifecycle_state_class: page.permission_inspector.lifecycle_state_class,
            },
        }
    }
}

impl ExtensionPermissionSupportExportRow {
    fn from_permission_row(row: &ExtensionPermissionInspectorRow) -> Self {
        Self {
            permission_row_id: row.permission_row_id.clone(),
            capability_class_class: row.capability_class_class,
            scope_class: row.scope_class,
            scope_target: row.scope_target.clone(),
            disposition_class: row.disposition_class,
            diff_class: row.diff_class,
            policy_narrowing_refs: row.policy_narrowing_refs.clone(),
            export_safe_explanation: row.export_safe_explanation.clone(),
            redaction_class: row.redaction_class,
        }
    }
}

impl ExtensionSettingSupportExportRow {
    fn from_setting_row(row: &ExtensionSettingInspectorRow) -> Self {
        Self {
            setting_row_id: row.setting_row_id.clone(),
            setting_id: row.setting_id.clone(),
            effective_value_preview: row.effective_value_preview.clone(),
            winning_scope_class: row.winning_scope_class,
            winning_source_ref: row.winning_source_ref.clone(),
            source_chain: row.source_chain.clone(),
            diff_rows: row.diff_rows.clone(),
            lock_state_class: row.lock_state_class,
            policy_source_ref: row.policy_source_ref.clone(),
            redaction_class: row.redaction_class,
            raw_secret_value_exported: row.raw_secret_value_exported,
        }
    }
}

/// Validation error for extension inspector packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExtensionInspectorValidationError {
    /// Page record kind is wrong.
    PageRecordKindWrong,
    /// Support export record kind is wrong.
    SupportExportRecordKindWrong,
    /// Schema version is wrong.
    SchemaVersionWrong,
    /// Shared contract ref is wrong.
    SharedContractWrong,
    /// Runtime placement or lifecycle is missing or unsupported for the inspector.
    RuntimeTruthMissing,
    /// A required permission disposition is absent.
    MissingPermissionDisposition {
        /// Missing disposition.
        disposition_class: ExtensionCapabilityDispositionClass,
    },
    /// A setting row is not diffable or source-attributed.
    SettingRowOpaque {
        /// Setting row id.
        setting_row_id: String,
    },
    /// A raw secret value was exported.
    RawSecretValueExported {
        /// Setting row id.
        setting_row_id: String,
    },
    /// Support export drifted from the page.
    SupportExportParityDrift {
        /// Field that drifted.
        field: String,
    },
}

impl std::fmt::Display for ExtensionInspectorValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PageRecordKindWrong => write!(f, "extension inspector page record kind is wrong"),
            Self::SupportExportRecordKindWrong => {
                write!(f, "extension inspector support export record kind is wrong")
            }
            Self::SchemaVersionWrong => write!(f, "extension inspector schema version is wrong"),
            Self::SharedContractWrong => write!(f, "extension inspector shared contract is wrong"),
            Self::RuntimeTruthMissing => {
                write!(
                    f,
                    "extension inspector runtime host or lifecycle truth is missing"
                )
            }
            Self::MissingPermissionDisposition { disposition_class } => write!(
                f,
                "extension inspector missing required permission disposition {disposition_class:?}"
            ),
            Self::SettingRowOpaque { setting_row_id } => write!(
                f,
                "extension setting row {setting_row_id} is missing source attribution or diffs"
            ),
            Self::RawSecretValueExported { setting_row_id } => {
                write!(
                    f,
                    "extension setting row {setting_row_id} exported a raw secret value"
                )
            }
            Self::SupportExportParityDrift { field } => {
                write!(f, "extension inspector support export drifted on {field}")
            }
        }
    }
}

impl std::error::Error for ExtensionInspectorValidationError {}

/// Builds the seeded extension inspector page used by shell and headless tests.
pub fn seeded_extension_inspector_page() -> ExtensionInspectorPage {
    let permission_inspector = seeded_permission_inspector();
    let settings_inspector = seeded_settings_inspector();
    let summary = ExtensionInspectorPageSummary {
        permission_row_count: permission_inspector.summary.row_count,
        setting_row_count: settings_inspector.summary.row_count,
        denied_permission_count: permission_inspector.summary.denied_count,
        policy_locked_permission_count: permission_inspector.summary.policy_locked_count,
        policy_locked_setting_count: settings_inspector.summary.policy_locked_count,
        redacted_setting_count: settings_inspector.summary.redacted_value_count,
    };
    ExtensionInspectorPage {
        record_kind: EXTENSION_INSPECTOR_PAGE_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_INSPECTOR_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_INSPECTOR_SHARED_CONTRACT_REF.to_owned(),
        page_id: format!(
            "extension_inspector_page:{}:{}",
            SEEDED_EXTENSION_IDENTITY, SEEDED_EXTENSION_VERSION
        ),
        page_label: "Extension inspectors: settings, permissions, runtime placement".to_owned(),
        extension_identity_ref: SEEDED_EXTENSION_IDENTITY.to_owned(),
        extension_version: SEEDED_EXTENSION_VERSION.to_owned(),
        generated_at: SEEDED_GENERATED_AT.to_owned(),
        permission_inspector,
        settings_inspector,
        summary,
    }
}

/// Builds the seeded support export for the extension inspector page.
pub fn seeded_extension_inspector_support_export() -> ExtensionInspectorSupportExport {
    let page = seeded_extension_inspector_page();
    ExtensionInspectorSupportExport::from_page(
        "extension_inspector_support_export:dev.aureline.samples/wasm-notes:1.0.0-beta.1",
        SEEDED_GENERATED_AT,
        &page,
    )
}

/// Validates an extension inspector page.
pub fn validate_extension_inspector_page(
    page: &ExtensionInspectorPage,
) -> Result<(), Vec<ExtensionInspectorValidationError>> {
    let mut errors = Vec::new();

    if page.record_kind != EXTENSION_INSPECTOR_PAGE_RECORD_KIND {
        errors.push(ExtensionInspectorValidationError::PageRecordKindWrong);
    }
    if page.schema_version != EXTENSION_INSPECTOR_SCHEMA_VERSION
        || page.permission_inspector.schema_version != EXTENSION_INSPECTOR_SCHEMA_VERSION
        || page.settings_inspector.schema_version != EXTENSION_INSPECTOR_SCHEMA_VERSION
    {
        errors.push(ExtensionInspectorValidationError::SchemaVersionWrong);
    }
    if page.shared_contract_ref != EXTENSION_INSPECTOR_SHARED_CONTRACT_REF
        || page.permission_inspector.shared_contract_ref != EXTENSION_INSPECTOR_SHARED_CONTRACT_REF
        || page.settings_inspector.shared_contract_ref != EXTENSION_INSPECTOR_SHARED_CONTRACT_REF
    {
        errors.push(ExtensionInspectorValidationError::SharedContractWrong);
    }
    if matches!(
        page.permission_inspector.host_placement_class,
        HostPlacementClass::UnknownPlacementClass
    ) || matches!(
        page.permission_inspector.lifecycle_state_class,
        RuntimeLifecycleStateClass::Discovered | RuntimeLifecycleStateClass::Removed
    ) {
        errors.push(ExtensionInspectorValidationError::RuntimeTruthMissing);
    }

    for disposition_class in [
        ExtensionCapabilityDispositionClass::Granted,
        ExtensionCapabilityDispositionClass::Denied,
        ExtensionCapabilityDispositionClass::PolicyLocked,
    ] {
        if !page
            .permission_inspector
            .rows
            .iter()
            .any(|row| row.disposition_class == disposition_class)
        {
            errors.push(
                ExtensionInspectorValidationError::MissingPermissionDisposition {
                    disposition_class,
                },
            );
        }
    }

    for row in &page.settings_inspector.rows {
        if row.source_chain.is_empty() || row.diff_rows.is_empty() {
            errors.push(ExtensionInspectorValidationError::SettingRowOpaque {
                setting_row_id: row.setting_row_id.clone(),
            });
        }
        if row.raw_secret_value_exported
            || row
                .source_chain
                .iter()
                .any(|source| source.raw_secret_value_exported)
            || row
                .diff_rows
                .iter()
                .any(|diff| diff.raw_secret_value_exported)
        {
            errors.push(ExtensionInspectorValidationError::RawSecretValueExported {
                setting_row_id: row.setting_row_id.clone(),
            });
        }
    }

    let recomputed_permission_summary = permission_summary(&page.permission_inspector.rows);
    if page.permission_inspector.summary != recomputed_permission_summary {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "permission_inspector.summary".to_owned(),
            },
        );
    }

    let recomputed_settings_summary = settings_summary(&page.settings_inspector.rows);
    if page.settings_inspector.summary != recomputed_settings_summary {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "settings_inspector.summary".to_owned(),
            },
        );
    }

    let recomputed_summary = ExtensionInspectorPageSummary {
        permission_row_count: page.permission_inspector.rows.len(),
        setting_row_count: page.settings_inspector.rows.len(),
        denied_permission_count: page
            .permission_inspector
            .rows
            .iter()
            .filter(|row| row.disposition_class == ExtensionCapabilityDispositionClass::Denied)
            .count(),
        policy_locked_permission_count: page
            .permission_inspector
            .rows
            .iter()
            .filter(|row| {
                row.disposition_class == ExtensionCapabilityDispositionClass::PolicyLocked
            })
            .count(),
        policy_locked_setting_count: page
            .settings_inspector
            .rows
            .iter()
            .filter(|row| row.lock_state_class == ExtensionSettingLockStateClass::PolicyLocked)
            .count(),
        redacted_setting_count: page
            .settings_inspector
            .rows
            .iter()
            .filter(|row| row.redaction_class != ExtensionSettingRedactionClass::MetadataSafe)
            .count(),
    };
    if page.summary != recomputed_summary {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "page.summary".to_owned(),
            },
        );
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates a support export against its source page.
pub fn validate_extension_inspector_support_export(
    export: &ExtensionInspectorSupportExport,
    page: &ExtensionInspectorPage,
) -> Result<(), Vec<ExtensionInspectorValidationError>> {
    let mut errors = Vec::new();

    if export.record_kind != EXTENSION_INSPECTOR_SUPPORT_EXPORT_RECORD_KIND {
        errors.push(ExtensionInspectorValidationError::SupportExportRecordKindWrong);
    }
    if export.schema_version != EXTENSION_INSPECTOR_SCHEMA_VERSION {
        errors.push(ExtensionInspectorValidationError::SchemaVersionWrong);
    }
    if export.shared_contract_ref != EXTENSION_INSPECTOR_SHARED_CONTRACT_REF {
        errors.push(ExtensionInspectorValidationError::SharedContractWrong);
    }
    if export.page_ref != page.page_id {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "page_ref".to_owned(),
            },
        );
    }
    if export.host_placement_class != page.permission_inspector.host_placement_class {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "host_placement_class".to_owned(),
            },
        );
    }
    if export.lifecycle_state_class != page.permission_inspector.lifecycle_state_class {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "lifecycle_state_class".to_owned(),
            },
        );
    }
    let permission_row_ids: Vec<String> = page
        .permission_inspector
        .rows
        .iter()
        .map(|row| row.permission_row_id.clone())
        .collect();
    let export_permission_row_ids: Vec<String> = export
        .permission_rows
        .iter()
        .map(|row| row.permission_row_id.clone())
        .collect();
    if permission_row_ids != export_permission_row_ids {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "permission_rows".to_owned(),
            },
        );
    }
    let expected_permission_rows: Vec<ExtensionPermissionSupportExportRow> = page
        .permission_inspector
        .rows
        .iter()
        .map(ExtensionPermissionSupportExportRow::from_permission_row)
        .collect();
    if export.permission_rows != expected_permission_rows {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "permission_rows.content".to_owned(),
            },
        );
    }
    let setting_row_ids: Vec<String> = page
        .settings_inspector
        .rows
        .iter()
        .map(|row| row.setting_row_id.clone())
        .collect();
    let export_setting_row_ids: Vec<String> = export
        .setting_rows
        .iter()
        .map(|row| row.setting_row_id.clone())
        .collect();
    if setting_row_ids != export_setting_row_ids {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "setting_rows".to_owned(),
            },
        );
    }
    let expected_setting_rows: Vec<ExtensionSettingSupportExportRow> = page
        .settings_inspector
        .rows
        .iter()
        .map(ExtensionSettingSupportExportRow::from_setting_row)
        .collect();
    if export.setting_rows != expected_setting_rows {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "setting_rows.content".to_owned(),
            },
        );
    }
    if export.raw_secret_values_exported {
        errors.push(ExtensionInspectorValidationError::RawSecretValueExported {
            setting_row_id: "support_export".to_owned(),
        });
    }
    if export.parity_fingerprint.permission_row_ids != permission_row_ids {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "parity_fingerprint.permission_row_ids".to_owned(),
            },
        );
    }
    if export.parity_fingerprint.setting_row_ids != setting_row_ids {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "parity_fingerprint.setting_row_ids".to_owned(),
            },
        );
    }
    if export.parity_fingerprint.permission_summary != page.permission_inspector.summary {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "parity_fingerprint.permission_summary".to_owned(),
            },
        );
    }
    if export.parity_fingerprint.settings_summary != page.settings_inspector.summary {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "parity_fingerprint.settings_summary".to_owned(),
            },
        );
    }
    if export.parity_fingerprint.host_placement_class
        != page.permission_inspector.host_placement_class
    {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "parity_fingerprint.host_placement_class".to_owned(),
            },
        );
    }
    if export.parity_fingerprint.lifecycle_state_class
        != page.permission_inspector.lifecycle_state_class
    {
        errors.push(
            ExtensionInspectorValidationError::SupportExportParityDrift {
                field: "parity_fingerprint.lifecycle_state_class".to_owned(),
            },
        );
    }

    for support_row in &export.setting_rows {
        if support_row.raw_secret_value_exported
            || support_row
                .source_chain
                .iter()
                .any(|source| source.raw_secret_value_exported)
            || support_row
                .diff_rows
                .iter()
                .any(|diff| diff.raw_secret_value_exported)
        {
            errors.push(ExtensionInspectorValidationError::RawSecretValueExported {
                setting_row_id: support_row.setting_row_id.clone(),
            });
        }
        if support_row.source_chain.is_empty() || support_row.diff_rows.is_empty() {
            errors.push(ExtensionInspectorValidationError::SettingRowOpaque {
                setting_row_id: support_row.setting_row_id.clone(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn seeded_permission_inspector() -> ExtensionPermissionInspector {
    let manifest = seeded_manifest_baseline();
    let narrowings = seeded_policy_narrowings();
    let requested_effective: Vec<(PermissionScopeClass, String)> = manifest
        .declared_permissions
        .iter()
        .map(|entry| (entry.scope_class, entry.scope_target.clone()))
        .collect();
    let effective = compute_effective_permission_baseline(
        &manifest,
        &requested_effective,
        &narrowings,
        SEEDED_GENERATED_AT,
        SummaryFreshnessClass::AuthoritativeLive,
    );
    let permission_manifest = project_permission_manifest(
        &manifest,
        "permission_manifest:dev.aureline.samples/wasm-notes:1.0.0-beta.1",
    );
    let install_decision = decide_manifest_install(&manifest, &effective, SEEDED_GENERATED_AT);
    let runtime = evaluate_runtime_v1_beta_contract(RuntimeV1BetaContractInput {
        contract_id: "runtime_v1_beta:dev.aureline.samples/wasm-notes:1.0.0-beta.1:inspectors"
            .to_owned(),
        extension_identity_ref: SEEDED_EXTENSION_IDENTITY.to_owned(),
        extension_version: SEEDED_EXTENSION_VERSION.to_owned(),
        manifest_baseline_ref: manifest.manifest_baseline_id.clone(),
        manifest_install_decision_class: install_decision.install_decision_class,
        manifest_install_decision_reason_class: install_decision.install_decision_reason_class,
        host_contract_family_class: manifest.host_contract_family_class,
        host_placement_class: HostPlacementClass::WasmIsolatedSubprocess,
        host_supervision_class: HostSupervisionClass::SeparateSubprocessSupervised,
        host_negotiation_packet_ref:
            "host_negotiation:dev.aureline.samples/wasm-notes:1.0.0-beta.1:inspectors".to_owned(),
        declared_capability_world_refs: vec![
            "aureline:workspace-read@1.0.0".to_owned(),
            "aureline:workspace-settings@1.0.0".to_owned(),
            "aureline:network-egress@1.0.0".to_owned(),
            "aureline:shell-execute@1.0.0".to_owned(),
        ],
        negotiated_capability_world_refs: vec![
            "aureline:workspace-read@1.0.0".to_owned(),
            "aureline:workspace-settings@1.0.0".to_owned(),
            "aureline:network-egress@1.0.0".to_owned(),
        ],
        narrowed_capability_world_refs: vec!["aureline:shell-execute@1.0.0".to_owned()],
        narrowing_reasons_recorded: true,
        effective_permission_summary_ref:
            "effective_permission_summary:dev.aureline.samples/wasm-notes:1.0.0-beta.1".to_owned(),
        effective_permission_diff_present: true,
        effective_permission_widening_attempted_blocked_count: effective
            .widening_attempted_blocked_count,
        lifecycle_state_class: RuntimeLifecycleStateClass::Active,
        restart_posture_class: RestartPostureClass::OneWarmRestartUnderBudget,
        restart_attempt_count: 1,
        degraded_state_class: aureline_extensions::DegradedStateClass::NoneNominal,
        runtime_budget_summary_ref:
            "runtime_budget_summary:dev.aureline.samples/wasm-notes:1.0.0-beta.1".to_owned(),
        runtime_budget_quarantine_active: false,
        runtime_budget_crash_loop_active: false,
        sdk_release_bundle_ref: "sdk_release_bundle:aureline-extensions-sdk:1.0.0-beta.1"
            .to_owned(),
        marketplace_metadata_ref:
            "marketplace_metadata:dev.aureline.samples/wasm-notes:1.0.0-beta.1".to_owned(),
        sdk_alignment_class: SdkAlignmentClass::Aligned,
        audit_event_refs: vec![
            "audit:extension_inspector_runtime_contract_opened".to_owned(),
            "audit:extension_inspector_worlds_narrowed".to_owned(),
        ],
        decided_at: SEEDED_GENERATED_AT.to_owned(),
    });
    let permission_support = project_permission_manifest_support_export(
        &permission_manifest,
        None,
        "permission_manifest_support_export:dev.aureline.samples/wasm-notes:1.0.0-beta.1",
    );
    let runtime_support = project_runtime_v1_beta_support_export(&runtime);
    let rows = build_permission_rows(&permission_manifest, &effective, &narrowings);
    let summary = permission_summary(&rows);
    ExtensionPermissionInspector {
        record_kind: EXTENSION_PERMISSION_INSPECTOR_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_INSPECTOR_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_INSPECTOR_SHARED_CONTRACT_REF.to_owned(),
        inspector_id: "extension_permission_inspector:dev.aureline.samples/wasm-notes:1.0.0-beta.1"
            .to_owned(),
        extension_identity_ref: SEEDED_EXTENSION_IDENTITY.to_owned(),
        extension_version: SEEDED_EXTENSION_VERSION.to_owned(),
        source_refs: ExtensionPermissionInspectorSourceRefs {
            manifest_baseline_ref: manifest.manifest_baseline_id,
            permission_manifest_ref: permission_manifest.permission_manifest_id,
            effective_permission_summary_ref:
                "effective_permission_summary:dev.aureline.samples/wasm-notes:1.0.0-beta.1"
                    .to_owned(),
            runtime_contract_ref: runtime.contract_id,
            permission_support_export_ref: permission_support.export_id,
            runtime_support_export_ref: runtime_support.export_id,
        },
        host_placement_class: runtime_support.host_placement_class,
        host_supervision_class: runtime_support.host_supervision_class,
        lifecycle_state_class: runtime_support.lifecycle_state_class,
        admission_decision_class: runtime_support.admission_decision_class,
        admission_reason_class: runtime_support.admission_reason_class,
        rows,
        summary,
    }
}

fn seeded_manifest_baseline() -> ExtensionManifestBaselineRecord {
    ExtensionManifestBaselineRecord {
        record_kind: EXTENSION_MANIFEST_BASELINE_RECORD_KIND.to_owned(),
        extension_manifest_baseline_schema_version: EXTENSION_MANIFEST_BASELINE_SCHEMA_VERSION,
        manifest_baseline_id: "manifest_baseline:dev.aureline.samples/wasm-notes:1.0.0-beta.1"
            .to_owned(),
        extension_identity: SEEDED_EXTENSION_IDENTITY.to_owned(),
        extension_version: SEEDED_EXTENSION_VERSION.to_owned(),
        extension_lifecycle_state_class: ExtensionLifecycleStateClass::Published,
        host_contract_family_class: HostContractFamilyClass::WasmComponentModel,
        manifest_origin_source_class: ManifestOriginSourceClass::PublicRegistry,
        origin_source_label: "public registry: registry.aureline.dev".to_owned(),
        publisher_identity_ref: "publisher:dev.aureline.samples".to_owned(),
        publisher_display_label: "Aureline Samples".to_owned(),
        publisher_trust_tier_class: PublisherTrustTierClass::VerifiedPublisher,
        publisher_lifecycle_state_class: PublisherLifecycleStateClass::Active,
        publisher_signing_key_ref: "publisher_signer:dev.aureline.samples:beta".to_owned(),
        declared_permissions: vec![
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::FilesystemRead,
                scope_target: "workspace:/notes/**".to_owned(),
                scope_constraint: Some("read-only under notes workspace prefix".to_owned()),
                rationale_label: "Read notes documents for the extension panel.".to_owned(),
            },
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::WorkspaceSettingsRead,
                scope_target: "workspace:settings:aureline.notes.*".to_owned(),
                scope_constraint: Some("read extension-owned settings only".to_owned()),
                rationale_label: "Render extension-local settings in the notes panel.".to_owned(),
            },
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::WorkspaceSettingsWrite,
                scope_target: "workspace:settings:aureline.notes.sync_mode".to_owned(),
                scope_constraint: Some("write extension-owned sync mode only".to_owned()),
                rationale_label: "Persist user-approved sync mode changes.".to_owned(),
            },
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::NetworkEgress,
                scope_target: "api.notes.example".to_owned(),
                scope_constraint: Some("egress to declared host only".to_owned()),
                rationale_label: "Fetch optional note metadata from the declared endpoint."
                    .to_owned(),
            },
            PermissionScopeEntry {
                scope_class: PermissionScopeClass::ShellExecute,
                scope_target: "command:git".to_owned(),
                scope_constraint: Some("read-only git status probe".to_owned()),
                rationale_label: "Check repository status before note export.".to_owned(),
            },
        ],
        manifest_scope_completeness_class: ManifestScopeCompletenessClass::Complete,
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

fn seeded_policy_narrowings() -> Vec<PolicyPackNarrowing> {
    vec![
        PolicyPackNarrowing {
            policy_pack_ref: "policy_pack:extension-inspector-default#settings-write".to_owned(),
            scope_class: PermissionScopeClass::WorkspaceSettingsWrite,
            scope_target: "workspace:settings:aureline.notes.sync_mode".to_owned(),
            diff_class: EffectivePermissionDiffClass::StepUpRequired,
            narrowing_reason_label:
                "admin policy requires step-up before extension settings writes".to_owned(),
        },
        PolicyPackNarrowing {
            policy_pack_ref: "policy_pack:extension-inspector-default#network-egress".to_owned(),
            scope_class: PermissionScopeClass::NetworkEgress,
            scope_target: "api.notes.example".to_owned(),
            diff_class: EffectivePermissionDiffClass::Narrowed,
            narrowing_reason_label:
                "admin policy limits extension egress to the approved host allow-list".to_owned(),
        },
        PolicyPackNarrowing {
            policy_pack_ref: "policy_pack:extension-inspector-default#shell-execute".to_owned(),
            scope_class: PermissionScopeClass::ShellExecute,
            scope_target: "command:git".to_owned(),
            diff_class: EffectivePermissionDiffClass::Denied,
            narrowing_reason_label: "admin policy denies shell execution from this extension"
                .to_owned(),
        },
    ]
}

fn build_permission_rows(
    permission_manifest: &PermissionManifestRecord,
    effective: &EffectivePermissionBaselineRecord,
    narrowings: &[PolicyPackNarrowing],
) -> Vec<ExtensionPermissionInspectorRow> {
    effective
        .declared_vs_effective_diff
        .iter()
        .map(|diff| {
            let declared = permission_manifest
                .declared_permissions
                .iter()
                .find(|entry| {
                    entry.scope_class == diff.scope_class && entry.scope_target == diff.scope_target
                });
            let effective_entry = effective.effective_permissions.iter().find(|entry| {
                entry.scope_class == diff.scope_class && entry.scope_target == diff.scope_target
            });
            let policy_narrowing_refs: Vec<String> = narrowings
                .iter()
                .filter(|narrowing| {
                    narrowing.scope_class == diff.scope_class
                        && narrowing.scope_target == diff.scope_target
                })
                .map(|narrowing| narrowing.policy_pack_ref.clone())
                .collect();
            ExtensionPermissionInspectorRow {
                permission_row_id: permission_row_id(diff),
                capability_class_class: declared
                    .map(|entry| entry.capability_class_class)
                    .unwrap_or_else(|| capability_class_for_scope(diff.scope_class)),
                scope_class: diff.scope_class,
                scope_target: diff.scope_target.clone(),
                declared_scope_constraint: declared
                    .and_then(|entry| entry.scope_constraint.clone()),
                effective_scope_constraint: effective_entry
                    .and_then(|entry| entry.scope_constraint.clone()),
                rationale_label: declared
                    .map(|entry| entry.rationale_label.clone())
                    .unwrap_or_else(|| "Blocked undeclared capability request.".to_owned()),
                disposition_class: disposition_for(diff.diff_class),
                diff_class: diff.diff_class,
                lock_reason_class: lock_reason_for(diff.diff_class),
                policy_narrowing_refs,
                export_safe_explanation: explanation_for(diff),
                support_export_required: true,
                redaction_class: RedactionClass::MetadataSafeDefault,
            }
        })
        .collect()
}

fn permission_summary(
    rows: &[ExtensionPermissionInspectorRow],
) -> ExtensionPermissionInspectorSummary {
    ExtensionPermissionInspectorSummary {
        row_count: rows.len(),
        granted_count: rows
            .iter()
            .filter(|row| row.disposition_class == ExtensionCapabilityDispositionClass::Granted)
            .count(),
        denied_count: rows
            .iter()
            .filter(|row| row.disposition_class == ExtensionCapabilityDispositionClass::Denied)
            .count(),
        policy_locked_count: rows
            .iter()
            .filter(|row| {
                row.disposition_class == ExtensionCapabilityDispositionClass::PolicyLocked
            })
            .count(),
        step_up_required_count: rows
            .iter()
            .filter(|row| {
                row.lock_reason_class == ExtensionCapabilityLockReasonClass::StepUpRequired
            })
            .count(),
    }
}

fn disposition_for(
    diff_class: EffectivePermissionDiffClass,
) -> ExtensionCapabilityDispositionClass {
    match diff_class {
        EffectivePermissionDiffClass::Unchanged => ExtensionCapabilityDispositionClass::Granted,
        EffectivePermissionDiffClass::Narrowed | EffectivePermissionDiffClass::StepUpRequired => {
            ExtensionCapabilityDispositionClass::PolicyLocked
        }
        EffectivePermissionDiffClass::Denied
        | EffectivePermissionDiffClass::WideningAttemptedBlocked => {
            ExtensionCapabilityDispositionClass::Denied
        }
    }
}

fn lock_reason_for(diff_class: EffectivePermissionDiffClass) -> ExtensionCapabilityLockReasonClass {
    match diff_class {
        EffectivePermissionDiffClass::Unchanged => ExtensionCapabilityLockReasonClass::None,
        EffectivePermissionDiffClass::Narrowed => {
            ExtensionCapabilityLockReasonClass::NarrowedByPolicy
        }
        EffectivePermissionDiffClass::StepUpRequired => {
            ExtensionCapabilityLockReasonClass::StepUpRequired
        }
        EffectivePermissionDiffClass::Denied => ExtensionCapabilityLockReasonClass::DeniedByPolicy,
        EffectivePermissionDiffClass::WideningAttemptedBlocked => {
            ExtensionCapabilityLockReasonClass::WideningBlocked
        }
    }
}

fn explanation_for(diff: &DeclaredVsEffectiveDiffEntry) -> String {
    match diff.diff_class {
        EffectivePermissionDiffClass::Unchanged => {
            "Granted as declared by the permission manifest.".to_owned()
        }
        EffectivePermissionDiffClass::Narrowed => {
            format!(
                "Policy narrowed this capability: {}",
                diff.narrowing_reason_label
            )
        }
        EffectivePermissionDiffClass::StepUpRequired => {
            format!(
                "Policy locks this capability until step-up review: {}",
                diff.narrowing_reason_label
            )
        }
        EffectivePermissionDiffClass::Denied => {
            format!("Denied by policy: {}", diff.narrowing_reason_label)
        }
        EffectivePermissionDiffClass::WideningAttemptedBlocked => {
            "Denied because the requested capability was not declared.".to_owned()
        }
    }
}

fn permission_row_id(diff: &DeclaredVsEffectiveDiffEntry) -> String {
    format!(
        "extension_permission_row:{}:{}:{}",
        SEEDED_EXTENSION_IDENTITY,
        token_for_permission_scope(diff.scope_class),
        sanitize_ref(&diff.scope_target)
    )
}

fn token_for_permission_scope(scope: PermissionScopeClass) -> &'static str {
    match scope {
        PermissionScopeClass::FilesystemRead => "filesystem_read",
        PermissionScopeClass::FilesystemWrite => "filesystem_write",
        PermissionScopeClass::ShellExecute => "shell_execute",
        PermissionScopeClass::NetworkEgress => "network_egress",
        PermissionScopeClass::AiProviderAccess => "ai_provider_access",
        PermissionScopeClass::ConnectedProviderAccess => "connected_provider_access",
        PermissionScopeClass::SecretHandleUse => "secret_handle_use",
        PermissionScopeClass::WorkspaceSettingsRead => "workspace_settings_read",
        PermissionScopeClass::WorkspaceSettingsWrite => "workspace_settings_write",
        PermissionScopeClass::ExecutionContextBind => "execution_context_bind",
        PermissionScopeClass::SubscriptionSubscribe => "subscription_subscribe",
        PermissionScopeClass::UiCommandContribute => "ui_command_contribute",
        PermissionScopeClass::CapabilityInherit => "capability_inherit",
    }
}

fn seeded_settings_inspector() -> ExtensionSettingsInspector {
    let rows = vec![
        sync_mode_setting_row(),
        project_root_setting_row(),
        registry_token_setting_row(),
    ];
    let summary = settings_summary(&rows);
    ExtensionSettingsInspector {
        record_kind: EXTENSION_SETTINGS_INSPECTOR_RECORD_KIND.to_owned(),
        schema_version: EXTENSION_INSPECTOR_SCHEMA_VERSION,
        shared_contract_ref: EXTENSION_INSPECTOR_SHARED_CONTRACT_REF.to_owned(),
        inspector_id: "extension_settings_inspector:dev.aureline.samples/wasm-notes:1.0.0-beta.1"
            .to_owned(),
        extension_identity_ref: SEEDED_EXTENSION_IDENTITY.to_owned(),
        extension_version: SEEDED_EXTENSION_VERSION.to_owned(),
        settings_store_ref: "extension_settings_store:dev.aureline.samples/wasm-notes:workspace"
            .to_owned(),
        rows,
        summary,
    }
}

fn settings_summary(rows: &[ExtensionSettingInspectorRow]) -> ExtensionSettingsInspectorSummary {
    ExtensionSettingsInspectorSummary {
        row_count: rows.len(),
        policy_locked_count: rows
            .iter()
            .filter(|row| row.lock_state_class == ExtensionSettingLockStateClass::PolicyLocked)
            .count(),
        redacted_value_count: rows
            .iter()
            .filter(|row| row.redaction_class != ExtensionSettingRedactionClass::MetadataSafe)
            .count(),
        diffable_setting_count: rows.iter().filter(|row| !row.diff_rows.is_empty()).count(),
    }
}

fn sync_mode_setting_row() -> ExtensionSettingInspectorRow {
    let setting_id = "aureline.notes.sync_mode";
    ExtensionSettingInspectorRow {
        setting_row_id: format!("extension_setting_row:{SEEDED_EXTENSION_IDENTITY}:sync_mode"),
        setting_id: setting_id.to_owned(),
        value_type: "enum".to_owned(),
        effective_value_preview: "manual".to_owned(),
        winning_scope_class: ExtensionSettingSourceScopeClass::AdminPolicy,
        winning_source_ref: source_ref(setting_id, "admin_policy"),
        source_chain: vec![
            source_row(
                setting_id,
                "extension_default",
                ExtensionSettingSourceScopeClass::ExtensionDefault,
                "Extension default",
                "off",
                ExtensionSettingSourceRelationClass::Shadowed,
            ),
            source_row(
                setting_id,
                "user_global",
                ExtensionSettingSourceScopeClass::UserGlobal,
                "User extension settings",
                "cloud",
                ExtensionSettingSourceRelationClass::Shadowed,
            ),
            source_row(
                setting_id,
                "workspace",
                ExtensionSettingSourceScopeClass::Workspace,
                "Workspace extension settings",
                "manual",
                ExtensionSettingSourceRelationClass::Shadowed,
            ),
            source_row(
                setting_id,
                "admin_policy",
                ExtensionSettingSourceScopeClass::AdminPolicy,
                "Admin policy bundle",
                "manual",
                ExtensionSettingSourceRelationClass::PolicyCeiling,
            ),
        ],
        diff_rows: vec![
            diff_row(
                setting_id,
                "sync_mode_policy_lock",
                "$",
                ExtensionSettingDiffClass::PolicyLocked,
                Some(source_ref(setting_id, "user_global")),
                source_ref(setting_id, "admin_policy"),
                Some("cloud"),
                "manual",
            ),
            diff_row(
                setting_id,
                "sync_mode_workspace_winner",
                "$",
                ExtensionSettingDiffClass::Changed,
                Some(source_ref(setting_id, "extension_default")),
                source_ref(setting_id, "workspace"),
                Some("off"),
                "manual",
            ),
        ],
        lock_state_class: ExtensionSettingLockStateClass::PolicyLocked,
        policy_source_ref: Some(
            "policy_pack:extension-inspector-default#settings-write".to_owned(),
        ),
        redaction_class: ExtensionSettingRedactionClass::MetadataSafe,
        raw_secret_value_exported: false,
        export_safe_summary:
            "aureline.notes.sync_mode effective=manual source=admin_policy policy_locked=true"
                .to_owned(),
    }
}

fn project_root_setting_row() -> ExtensionSettingInspectorRow {
    let setting_id = "aureline.notes.project_root";
    ExtensionSettingInspectorRow {
        setting_row_id: format!("extension_setting_row:{SEEDED_EXTENSION_IDENTITY}:project_root"),
        setting_id: setting_id.to_owned(),
        value_type: "string".to_owned(),
        effective_value_preview: "workspace-relative:notes".to_owned(),
        winning_scope_class: ExtensionSettingSourceScopeClass::Workspace,
        winning_source_ref: source_ref(setting_id, "workspace"),
        source_chain: vec![
            source_row(
                setting_id,
                "extension_default",
                ExtensionSettingSourceScopeClass::ExtensionDefault,
                "Extension default",
                "workspace-relative:.",
                ExtensionSettingSourceRelationClass::Shadowed,
            ),
            source_row(
                setting_id,
                "workspace",
                ExtensionSettingSourceScopeClass::Workspace,
                "Workspace extension settings",
                "workspace-relative:notes",
                ExtensionSettingSourceRelationClass::Winner,
            ),
        ],
        diff_rows: vec![diff_row(
            setting_id,
            "project_root_workspace_override",
            "$",
            ExtensionSettingDiffClass::Changed,
            Some(source_ref(setting_id, "extension_default")),
            source_ref(setting_id, "workspace"),
            Some("workspace-relative:."),
            "workspace-relative:notes",
        )],
        lock_state_class: ExtensionSettingLockStateClass::Unlocked,
        policy_source_ref: None,
        redaction_class: ExtensionSettingRedactionClass::MetadataSafe,
        raw_secret_value_exported: false,
        export_safe_summary:
            "aureline.notes.project_root effective=workspace-relative:notes source=workspace"
                .to_owned(),
    }
}

fn registry_token_setting_row() -> ExtensionSettingInspectorRow {
    let setting_id = "aureline.notes.registry_token";
    ExtensionSettingInspectorRow {
        setting_row_id: format!("extension_setting_row:{SEEDED_EXTENSION_IDENTITY}:registry_token"),
        setting_id: setting_id.to_owned(),
        value_type: "credential_handle".to_owned(),
        effective_value_preview: "credential handle present: code-host-registry".to_owned(),
        winning_scope_class: ExtensionSettingSourceScopeClass::MachineSpecific,
        winning_source_ref: source_ref(setting_id, "machine_specific"),
        source_chain: vec![
            source_row(
                setting_id,
                "extension_default",
                ExtensionSettingSourceScopeClass::ExtensionDefault,
                "Extension default",
                "not configured",
                ExtensionSettingSourceRelationClass::Shadowed,
            ),
            source_row(
                setting_id,
                "machine_specific",
                ExtensionSettingSourceScopeClass::MachineSpecific,
                "OS credential store handle",
                "credential handle present: code-host-registry",
                ExtensionSettingSourceRelationClass::Winner,
            ),
        ],
        diff_rows: vec![diff_row(
            setting_id,
            "registry_token_handle_present",
            "$",
            ExtensionSettingDiffClass::RedactedSecretHandle,
            Some(source_ref(setting_id, "extension_default")),
            source_ref(setting_id, "machine_specific"),
            Some("not configured"),
            "credential handle present: code-host-registry",
        )],
        lock_state_class: ExtensionSettingLockStateClass::Unlocked,
        policy_source_ref: None,
        redaction_class: ExtensionSettingRedactionClass::SecretValueRedacted,
        raw_secret_value_exported: false,
        export_safe_summary:
            "aureline.notes.registry_token effective=credential_handle_present source=machine_specific raw_secret=false"
                .to_owned(),
    }
}

fn source_row(
    setting_id: &str,
    source_suffix: &str,
    source_scope_class: ExtensionSettingSourceScopeClass,
    source_label: &str,
    value_preview: &str,
    relation_class: ExtensionSettingSourceRelationClass,
) -> ExtensionSettingSourceRow {
    ExtensionSettingSourceRow {
        source_ref: source_ref(setting_id, source_suffix),
        source_scope_class,
        source_label: source_label.to_owned(),
        value_preview: value_preview.to_owned(),
        relation_class,
        source_attribution_ref: format!(
            "extension_setting_attribution:{}:{}:{}",
            SEEDED_EXTENSION_IDENTITY,
            sanitize_ref(setting_id),
            source_suffix
        ),
        raw_secret_value_exported: false,
    }
}

fn diff_row(
    setting_id: &str,
    diff_suffix: &str,
    field_path: &str,
    diff_class: ExtensionSettingDiffClass,
    prior_source_ref: Option<String>,
    effective_source_ref: String,
    prior_value_preview: Option<&str>,
    effective_value_preview: &str,
) -> ExtensionSettingDiffRow {
    ExtensionSettingDiffRow {
        diff_row_id: format!(
            "extension_setting_diff:{}:{}:{}",
            SEEDED_EXTENSION_IDENTITY,
            sanitize_ref(setting_id),
            diff_suffix
        ),
        field_path: field_path.to_owned(),
        diff_class,
        prior_source_ref,
        effective_source_ref,
        prior_value_preview: prior_value_preview.map(str::to_owned),
        effective_value_preview: effective_value_preview.to_owned(),
        source_attribution_ref: format!(
            "extension_setting_attribution:{}:{}:{}",
            SEEDED_EXTENSION_IDENTITY,
            sanitize_ref(setting_id),
            diff_suffix
        ),
        raw_secret_value_exported: false,
    }
}

fn source_ref(setting_id: &str, source_suffix: &str) -> String {
    format!(
        "extension_setting_source:{}:{}:{}",
        SEEDED_EXTENSION_IDENTITY,
        sanitize_ref(setting_id),
        source_suffix
    )
}

fn sanitize_ref(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut last_was_sep = false;
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_was_sep = false;
        } else if !last_was_sep {
            out.push('_');
            last_was_sep = true;
        }
    }
    out.trim_matches('_').to_owned()
}
