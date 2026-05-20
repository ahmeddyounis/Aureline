//! Native extension manifest/schema authoring surface with inline validation,
//! permission explainability, and migration/deprecation guidance.
//!
//! This module turns an in-progress extension manifest into one inspectable
//! [`ManifestEditorSession`] record that the in-product manifest editor, CLI /
//! headless inspect, and support export all read instead of relaying opaque
//! CLI-only error text. A session carries:
//!
//! - **Inline validation** with field-level error anchors. The blocker checks
//!   are a faithful port of the beta extension validator
//!   ([`tools/extensions/m3/validator_cli/aureline_extension_validator.py`](../../../../tools/extensions/m3/validator_cli/aureline_extension_validator.py)):
//!   each finding carries the same stable `check_id`, `suite`, `status`, and
//!   `severity` the validator CLI and conformance kit emit, plus a JSON-pointer
//!   `anchor` so the editor can highlight the exact field.
//! - **Blockers vs advisories**: must-fix publish blockers (`severity = blocker`,
//!   the validator-parity checks) are separated from recommended UX/performance
//!   guidance (`advisory.*` checks, `severity = warning`/`info`), so authors can
//!   tell release blockers from advice.
//! - **Permission explanation chips**: per declared permission, the resolved
//!   capability class (shared with [`crate::permission_manifest`]), whether the
//!   scope is privileged, the trust-mode behavior, and prompt/review posture.
//! - **Migration / deprecation hints**: derived from the canonical lifecycle
//!   metadata packet ([`crate::lifecycle_metadata`]). Deprecated or replaced
//!   manifest fields produce actionable replacement guidance, removal horizons,
//!   and migration-doc links rather than generic invalid/unknown errors.
//! - **Version-range targeting** and required-shim posture (SDK line/window,
//!   host version range, platforms, support class, bridge state).
//! - **Open-schema / open-docs links** so authors can jump to the schema or
//!   migration docs without leaving Aureline.
//!
//! Validation is fully local: no extension code is executed and no network
//! round-trip is performed. The lifecycle packet is embedded at build time, so
//! permission reasoning and migration hints stay available offline or against a
//! mirror. The editor never writes unsupported fields and never suppresses a
//! schema error to feel friendly: every blocker the validator would raise is
//! surfaced with its stable id.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::lifecycle_metadata::{
    current_extension_lifecycle_metadata_packet, LifecycleDeprecationPostureClass,
    LifecycleMetadataRow, LifecycleStabilityLabel, LifecycleSurfaceKind,
};
use crate::manifest_baseline::RedactionClass;
use crate::permission_manifest::{capability_class_for_scope, CapabilityClassClass};

#[cfg(test)]
mod tests;

/// Record-kind tag carried on serialized [`ManifestEditorSession`] payloads.
pub const MANIFEST_EDITOR_SESSION_RECORD_KIND: &str = "extension_manifest_editor_session";

/// Schema version for manifest editor session payloads.
pub const MANIFEST_EDITOR_SESSION_SCHEMA_VERSION: u32 = 1;

/// Validator identity mirrored from the beta extension validator CLI so the
/// editor's exported findings are attributable to the same conformance lane.
pub const MANIFEST_EDITOR_VALIDATOR_ID: &str = "aureline.extension.validator.beta";

/// Validator version mirrored from the beta extension validator CLI.
pub const MANIFEST_EDITOR_VALIDATOR_VERSION: &str = "0.1.0";

/// Required `session_id` prefix.
pub const MANIFEST_EDITOR_SESSION_ID_PREFIX: &str = "extension_manifest_editor_session:";

const MANIFEST_SCHEMA_REF: &str = "schemas/extensions/beta_extension_manifest.schema.json";
const PERMISSION_SCHEMA_REF: &str = "schemas/extensions/permission_manifest.schema.json";
const LIFECYCLE_SCHEMA_REF: &str = "schemas/extensions/lifecycle_metadata.schema.json";
const CONFORMANCE_REPORT_SCHEMA_REF: &str = "schemas/extensions/conformance_kit_report.schema.json";
const VALIDATOR_CLI_REF: &str = "tools/extensions/m3/validator_cli/aureline_extension_validator.py";
const VERSIONING_POLICY_REF: &str = "docs/extensions/m3/sdk_versioning_and_deprecation.md";
const AUTHORING_DOCS_REF: &str = "docs/ecosystem/m3/manifest_editor_beta.md";
const LIFECYCLE_PACKET_REF: &str = "artifacts/extensions/m3/lifecycle_metadata_packet.json";

const MANIFEST_SCHEMA_VERSION: i64 = 1;
const MANIFEST_FIELD_SURFACE_PREFIX: &str = "manifest_field:extension_manifest.";

const PERMISSION_SCOPES: &[&str] = &[
    "filesystem_read",
    "filesystem_write",
    "shell_execute",
    "network_egress",
    "ai_provider_access",
    "connected_provider_access",
    "secret_handle_use",
    "workspace_settings_read",
    "workspace_settings_write",
    "execution_context_bind",
    "subscription_subscribe",
    "ui_command_contribute",
    "capability_inherit",
];
const PRIVILEGED_SCOPES: &[&str] = &[
    "filesystem_write",
    "shell_execute",
    "network_egress",
    "secret_handle_use",
    "workspace_settings_write",
    "execution_context_bind",
];
const TRUST_MODE_CLASSES: &[&str] = &[
    "allowed_in_trusted_workspace",
    "read_only_degrade",
    "disabled_in_restricted_mode",
    "explicit_approval_required",
];
const RUNTIME_ORIGINS: &[&str] = &[
    "wasm",
    "external_host",
    "helper_binary",
    "remote_side_component",
    "bridge",
];
const HOST_CONTRACT_FAMILIES: &[&str] = &[
    "wasm_component_model",
    "wasm_core_module",
    "external_host_process",
    "helper_binary",
    "remote_side_component",
    "compatibility_bridge",
];
const HOST_ABI_WINDOWS: &[&str] = &[
    "component_model_abi_window_beta_1",
    "core_module_abi_window_beta_1",
    "external_host_process_window_documented",
    "compatibility_bridge_window_documented",
];
const SUPPORT_CLASSES: &[&str] = &[
    "certified",
    "supported",
    "limited",
    "experimental",
    "community",
    "retest_pending",
    "evidence_stale",
    "unsupported",
];
const BRIDGE_STATES: &[&str] = &[
    "native",
    "bridge",
    "partial",
    "unsupported",
    "retest_pending",
];
const LIFECYCLE_STATES: &[&str] = &[
    "verified",
    "resolved",
    "activated",
    "degraded",
    "disabled",
    "removed",
];
const DEGRADED_BEHAVIORS: &[&str] = &[
    "read_only_degrade",
    "disable_background_work",
    "disable_until_review",
    "quarantine_pending_review",
];
const SCENARIO_CLASSES: &[&str] = &[
    "install",
    "activation",
    "permission_prompt",
    "degraded_path",
    "disable_rollback",
];
const NETWORK_ENDPOINT_CLASSES: &[&str] = &[
    "metadata_fetch",
    "package_registry",
    "vendor_api",
    "user_configured_url",
];
const SUPPORT_CLASSES_NEEDING_QUALIFICATION: &[&str] = &[
    "limited",
    "experimental",
    "community",
    "retest_pending",
    "evidence_stale",
];

/// Connectivity context the editor session was produced under.
///
/// Every variant produces identical local validation; the field exists so the
/// session can prove that permission reasoning and migration hints stay
/// available offline and against mirrors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestEditorConnectivityClass {
    /// Fully offline: no registry or mirror reachable.
    LocalOffline,
    /// An approved mirror or offline bundle is reachable.
    MirrorReachable,
    /// The primary registry is reachable.
    PrimaryRegistryReachable,
}

/// Conformance suite a finding belongs to (mirrors the validator vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestEditorFindingSuite {
    /// Manifest identity and shape.
    ManifestShape,
    /// Permission declaration completeness.
    PermissionDeclarations,
    /// Lifecycle activation, degraded, disable, and rollback metadata.
    LifecycleMetadata,
    /// SDK line, host ABI, and compatibility-target declarations.
    CompatibilityTargets,
    /// Replayable conformance fixture coverage.
    ConformanceFixtures,
    /// Editor-only recommended UX / performance advisories.
    EditorAdvisory,
}

/// Status of a single finding (mirrors the validator vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestEditorFindingStatus {
    /// The check passed.
    Pass,
    /// The check failed.
    Fail,
    /// The check raised a non-blocking warning.
    Warn,
    /// The check did not apply to this manifest.
    NotApplicable,
}

/// Severity of a single finding (mirrors the validator vocabulary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestEditorFindingSeverity {
    /// Must-fix publish blocker.
    Blocker,
    /// Recommended fix; does not block publication.
    Warning,
    /// Informational guidance.
    Info,
}

/// Overall validator-equivalent result class for the blocker (conformance) lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestEditorResultClass {
    /// No blocker failed and no warning fired.
    Pass,
    /// No blocker failed but at least one warning fired.
    Warn,
    /// At least one blocker failed.
    Fail,
}

/// Compatibility badge derived from the conformance result (mirrors the kit).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestEditorCompatibilityBadgeClass {
    /// Manifest passed the blocker lane and is compatible on declared targets.
    CompatibleOnDeclaredTargets,
    /// Manifest must be re-qualified before publication.
    UnsupportedPendingQualification,
}

/// Publish-readiness decision derived from blockers and advisories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestEditorPublishReadinessClass {
    /// No blockers and no advisories.
    ReadyToPublish,
    /// No blockers, but advisories are present.
    ReadyWithAdvisories,
    /// One or more must-fix blockers must be resolved first.
    BlockedOnMustFix,
}

/// Typed reason paired with [`ManifestEditorPublishReadinessClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestEditorPublishReadinessReasonClass {
    /// The manifest is clean.
    NoBlockersNoAdvisories,
    /// The manifest is publishable but has recommended improvements.
    NoBlockersAdvisoriesPresent,
    /// Must-fix blockers remain.
    MustFixBlockersPresent,
}

/// One inline validation finding with a field-level anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEditorFinding {
    /// Stable check id (validator-parity for blockers, `advisory.*` for advice).
    pub check_id: String,
    /// Conformance suite the check belongs to.
    pub suite: ManifestEditorFindingSuite,
    /// Pass/fail/warn/not-applicable status.
    pub status: ManifestEditorFindingStatus,
    /// Blocker/warning/info severity.
    pub severity: ManifestEditorFindingSeverity,
    /// Human-readable validation message.
    pub message: String,
    /// Dotted manifest field path the finding applies to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// JSON-pointer anchor the editor can use to highlight the field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// Suggested fix or next step.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fix: Option<String>,
}

/// One permission explanation chip rendered next to a declared permission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionExplanationChip {
    /// JSON-pointer anchor for the permission entry.
    pub anchor: String,
    /// Declared scope token (verbatim, even when unknown).
    pub scope: String,
    /// Whether the scope is in the closed permission vocabulary.
    pub scope_known: bool,
    /// Resolved capability class when the scope is known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub capability_class: Option<CapabilityClassClass>,
    /// Whether the scope is privileged and widens trust.
    pub privileged: bool,
    /// Declared trust-mode behavior, when present and known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trust_mode: Option<String>,
    /// Number of declared targets for the permission.
    pub target_count: u32,
    /// Whether a non-empty purpose is declared.
    pub purpose_present: bool,
    /// Whether the permission is marked review-required.
    pub review_required: bool,
    /// Whether prompt copy is present.
    pub prompt_summary_present: bool,
    /// Declared network endpoint class for `network_egress`, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_endpoint_class: Option<String>,
    /// Whether the permission uses brokered secret handles.
    pub handle_only: bool,
    /// Human-readable impact summary safe for chips and support export.
    pub explanation: String,
}

/// One migration / deprecation hint derived from the lifecycle packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestMigrationHint {
    /// Lifecycle row id the hint is derived from.
    pub lifecycle_row_ref: String,
    /// Dotted manifest field path that holds the deprecated value.
    pub field_path: String,
    /// JSON-pointer anchor for the deprecated field.
    pub field_anchor: String,
    /// Deprecated value currently present in the manifest.
    pub deprecated_value: String,
    /// Deprecation posture for the row.
    pub posture: LifecycleDeprecationPostureClass,
    /// Replacement value authors should migrate to, when one exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_value: Option<String>,
    /// Reason no direct replacement exists, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_direct_replacement_reason: Option<String>,
    /// Target version where the deprecated value can be removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removal_target_version: Option<String>,
    /// Target date where the deprecated value can be removed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub removal_target_date: Option<String>,
    /// Documentation that explains the migration path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub migration_guide_ref: Option<String>,
    /// Minimal replacement example authors can paste.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replacement_example: Option<String>,
    /// Author-facing migration guidance.
    pub guidance: String,
}

/// Version-range targeting and required-shim posture summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VersionTargetingSummary {
    /// Declared SDK line id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sdk_line_id: Option<String>,
    /// Declared SDK line semver.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sdk_line_semver: Option<String>,
    /// Declared host ABI window.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_abi_window: Option<String>,
    /// Minimum supported Aureline version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aureline_version_min: Option<String>,
    /// Maximum supported Aureline version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aureline_version_max: Option<String>,
    /// Declared platform targets.
    pub platforms: Vec<String>,
    /// Declared compatibility support class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_class: Option<String>,
    /// Declared bridge state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bridge_state: Option<String>,
    /// Whether a compatibility shim/bridge is required on declared targets.
    pub required_shim: bool,
    /// Note explaining the shim requirement, when one applies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub shim_note: Option<String>,
    /// Cited lifecycle metadata rows.
    pub lifecycle_metadata_refs: Vec<String>,
}

/// Open-schema / open-docs links the editor exposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEditorLinks {
    /// Author-facing manifest schema.
    pub manifest_schema_ref: String,
    /// Permission vocabulary schema.
    pub permission_schema_ref: String,
    /// Lifecycle metadata schema.
    pub lifecycle_schema_ref: String,
    /// Conformance kit report schema.
    pub conformance_report_schema_ref: String,
    /// Validator CLI the editor's blocker findings mirror.
    pub validator_cli_ref: String,
    /// SDK versioning and deprecation policy.
    pub versioning_policy_ref: String,
    /// In-product manifest editor authoring guide.
    pub authoring_docs_ref: String,
    /// Canonical lifecycle metadata packet.
    pub lifecycle_packet_ref: String,
}

/// Validator-parity conformance export the CLI and conformance kit also emit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEditorConformanceExport {
    /// Validator identity shared with the CLI lane.
    pub validator_id: String,
    /// Validator version shared with the CLI lane.
    pub validator_version: String,
    /// Result class over the blocker (conformance) lane.
    pub result_class: ManifestEditorResultClass,
    /// Compatibility badge derived from the result class.
    pub compatibility_badge_class: ManifestEditorCompatibilityBadgeClass,
    /// Red-flag classes derived from failed blocker checks.
    pub red_flag_classes: Vec<String>,
    /// Number of passing blocker checks.
    pub passed: u32,
    /// Number of failing blocker checks.
    pub failed: u32,
    /// Number of warning-status blocker checks.
    pub warnings: u32,
    /// Number of failing checks with blocker severity.
    pub blockers: u32,
}

/// Input to evaluate a manifest editor session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEditorSessionInput {
    /// Stable session id.
    pub session_id: String,
    /// UTC timestamp for the session.
    pub generated_at: String,
    /// Repository-relative or workspace ref to the manifest under edit.
    pub subject_manifest_ref: String,
    /// Connectivity context the session was produced under.
    pub connectivity_class: ManifestEditorConnectivityClass,
    /// The (possibly incomplete) manifest payload under edit.
    pub manifest: Value,
}

/// Canonical manifest editor session record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestEditorSession {
    /// Record-kind tag for serialized sessions.
    pub record_kind: String,
    /// Schema version for the session shape.
    pub manifest_editor_session_schema_version: u32,
    /// Stable session id.
    pub session_id: String,
    /// UTC timestamp for the session.
    pub generated_at: String,
    /// Ref to the manifest under edit.
    pub subject_manifest_ref: String,
    /// Connectivity context the session was produced under.
    pub connectivity_class: ManifestEditorConnectivityClass,
    /// Always true: validation runs locally without executing extension code.
    pub local_validation_only: bool,
    /// Always false: no network round-trip is required to validate.
    pub network_round_trip_required: bool,
    /// Best-effort extracted package id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_id: Option<String>,
    /// Best-effort extracted publisher id.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publisher_id: Option<String>,
    /// Best-effort extracted extension version.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extension_version: Option<String>,
    /// Best-effort extracted runtime origin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runtime_origin: Option<String>,
    /// Best-effort extracted host contract family.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_contract_family: Option<String>,
    /// Inline validation findings (blocker lane plus advisories).
    pub findings: Vec<ManifestEditorFinding>,
    /// Number of failing checks with blocker severity.
    pub blocker_count: u32,
    /// Number of active advisory findings.
    pub advisory_count: u32,
    /// Number of passing blocker checks.
    pub passed_count: u32,
    /// Permission explanation chips.
    pub permission_chips: Vec<PermissionExplanationChip>,
    /// Migration / deprecation hints.
    pub migration_hints: Vec<ManifestMigrationHint>,
    /// Version-range targeting summary.
    pub version_targeting: VersionTargetingSummary,
    /// Validator-parity conformance export.
    pub conformance_export: ManifestEditorConformanceExport,
    /// Publish-readiness decision.
    pub publish_readiness: ManifestEditorPublishReadinessClass,
    /// Typed reason for the publish-readiness decision.
    pub publish_readiness_reason: ManifestEditorPublishReadinessReasonClass,
    /// Human-readable publish-readiness summary.
    pub publish_readiness_summary: String,
    /// Open-schema / open-docs links.
    pub links: ManifestEditorLinks,
    /// Redaction class proving the session is safe for support export.
    pub redaction_class: RedactionClass,
}

/// Typed structural finding emitted by [`validate_manifest_editor_session`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestEditorSessionFinding {
    /// Stable check id for the failed invariant.
    pub check_id: &'static str,
    /// Human-readable validation message.
    pub message: String,
}

impl ManifestEditorSessionFinding {
    fn new(check_id: &'static str, message: impl Into<String>) -> Self {
        Self {
            check_id,
            message: message.into(),
        }
    }
}

/// Evaluate one [`ManifestEditorSessionInput`] into a canonical
/// [`ManifestEditorSession`].
///
/// Validation is local and offline-safe: the lifecycle metadata packet is
/// embedded at build time, no extension code is executed, and no network call
/// is made regardless of [`ManifestEditorConnectivityClass`].
pub fn evaluate_manifest_editor_session(
    input: ManifestEditorSessionInput,
) -> ManifestEditorSession {
    let ManifestEditorSessionInput {
        session_id,
        generated_at,
        subject_manifest_ref,
        connectivity_class,
        manifest,
    } = input;

    let mut findings: Vec<ManifestEditorFinding> = Vec::new();
    run_conformance_checks(&manifest, &mut findings);

    let migration_hints = collect_migration_hints(&manifest);
    run_advisory_checks(&manifest, &migration_hints, &mut findings);

    let permission_chips = collect_permission_chips(&manifest);
    let version_targeting = summarize_version_targeting(&manifest);

    let blocker_count = findings
        .iter()
        .filter(|f| {
            f.status == ManifestEditorFindingStatus::Fail
                && f.severity == ManifestEditorFindingSeverity::Blocker
        })
        .count() as u32;
    let advisory_count = findings
        .iter()
        .filter(|f| f.suite == ManifestEditorFindingSuite::EditorAdvisory)
        .count() as u32;
    let passed_count = findings
        .iter()
        .filter(|f| {
            f.suite != ManifestEditorFindingSuite::EditorAdvisory
                && f.status == ManifestEditorFindingStatus::Pass
        })
        .count() as u32;

    let conformance_export = project_conformance_export(&findings);

    let (publish_readiness, publish_readiness_reason, publish_readiness_summary) =
        decide_publish_readiness(blocker_count, advisory_count);

    ManifestEditorSession {
        record_kind: MANIFEST_EDITOR_SESSION_RECORD_KIND.to_string(),
        manifest_editor_session_schema_version: MANIFEST_EDITOR_SESSION_SCHEMA_VERSION,
        session_id,
        generated_at,
        subject_manifest_ref,
        connectivity_class,
        local_validation_only: true,
        network_round_trip_required: false,
        package_id: string_field(&manifest, "package_id"),
        publisher_id: string_field(&manifest, "publisher_id"),
        extension_version: string_field(&manifest, "version"),
        runtime_origin: string_field(&manifest, "runtime_origin"),
        host_contract_family: string_field(&manifest, "host_contract_family"),
        findings,
        blocker_count,
        advisory_count,
        passed_count,
        permission_chips,
        migration_hints,
        version_targeting,
        conformance_export,
        publish_readiness,
        publish_readiness_reason,
        publish_readiness_summary,
        links: ManifestEditorLinks {
            manifest_schema_ref: MANIFEST_SCHEMA_REF.to_string(),
            permission_schema_ref: PERMISSION_SCHEMA_REF.to_string(),
            lifecycle_schema_ref: LIFECYCLE_SCHEMA_REF.to_string(),
            conformance_report_schema_ref: CONFORMANCE_REPORT_SCHEMA_REF.to_string(),
            validator_cli_ref: VALIDATOR_CLI_REF.to_string(),
            versioning_policy_ref: VERSIONING_POLICY_REF.to_string(),
            authoring_docs_ref: AUTHORING_DOCS_REF.to_string(),
            lifecycle_packet_ref: LIFECYCLE_PACKET_REF.to_string(),
        },
        redaction_class: RedactionClass::MetadataSafeDefault,
    }
}

/// Validate structural invariants for a manifest editor session.
pub fn validate_manifest_editor_session(
    record: &ManifestEditorSession,
) -> Vec<ManifestEditorSessionFinding> {
    let mut findings = Vec::new();
    if record.record_kind != MANIFEST_EDITOR_SESSION_RECORD_KIND {
        findings.push(ManifestEditorSessionFinding::new(
            "manifest_editor_session.record_kind_wrong",
            format!(
                "record_kind must be '{MANIFEST_EDITOR_SESSION_RECORD_KIND}'; got {:?}",
                record.record_kind
            ),
        ));
    }
    if record.manifest_editor_session_schema_version != MANIFEST_EDITOR_SESSION_SCHEMA_VERSION {
        findings.push(ManifestEditorSessionFinding::new(
            "manifest_editor_session.schema_version_wrong",
            "manifest_editor_session_schema_version must match the current schema version",
        ));
    }
    if !record
        .session_id
        .starts_with(MANIFEST_EDITOR_SESSION_ID_PREFIX)
    {
        findings.push(ManifestEditorSessionFinding::new(
            "manifest_editor_session.session_id_unprefixed",
            format!("session_id must start with '{MANIFEST_EDITOR_SESSION_ID_PREFIX}'"),
        ));
    }
    if record.subject_manifest_ref.trim().is_empty() {
        findings.push(ManifestEditorSessionFinding::new(
            "manifest_editor_session.subject_manifest_ref_missing",
            "subject_manifest_ref must be non-empty",
        ));
    }
    if !record.local_validation_only || record.network_round_trip_required {
        findings.push(ManifestEditorSessionFinding::new(
            "manifest_editor_session.must_validate_locally",
            "sessions must validate locally without a network round-trip",
        ));
    }
    if record.redaction_class != RedactionClass::MetadataSafeDefault {
        findings.push(ManifestEditorSessionFinding::new(
            "manifest_editor_session.redaction_class_must_be_metadata_safe",
            "session redaction_class must be MetadataSafeDefault",
        ));
    }

    let expected_blockers = record
        .findings
        .iter()
        .filter(|f| {
            f.status == ManifestEditorFindingStatus::Fail
                && f.severity == ManifestEditorFindingSeverity::Blocker
        })
        .count() as u32;
    if record.blocker_count != expected_blockers {
        findings.push(ManifestEditorSessionFinding::new(
            "manifest_editor_session.blocker_count_inconsistent",
            "blocker_count must equal failing blocker-severity findings",
        ));
    }
    let expected_advisories = record
        .findings
        .iter()
        .filter(|f| f.suite == ManifestEditorFindingSuite::EditorAdvisory)
        .count() as u32;
    if record.advisory_count != expected_advisories {
        findings.push(ManifestEditorSessionFinding::new(
            "manifest_editor_session.advisory_count_inconsistent",
            "advisory_count must equal editor advisory findings",
        ));
    }

    for finding in &record.findings {
        if finding.check_id.trim().is_empty() {
            findings.push(ManifestEditorSessionFinding::new(
                "manifest_editor_session.finding_check_id_missing",
                "every finding must carry a non-empty check_id",
            ));
        }
        let is_advisory = finding.suite == ManifestEditorFindingSuite::EditorAdvisory;
        if is_advisory == (finding.severity == ManifestEditorFindingSeverity::Blocker) {
            findings.push(ManifestEditorSessionFinding::new(
                "manifest_editor_session.finding_severity_lane_mismatch",
                "advisory findings must not be blocker severity and blocker findings must be",
            ));
        }
        if is_advisory != finding.check_id.starts_with("advisory.") {
            findings.push(ManifestEditorSessionFinding::new(
                "manifest_editor_session.advisory_check_id_namespace",
                "advisory findings must use the 'advisory.' check-id namespace",
            ));
        }
        if finding.field.is_some() && finding.anchor.is_none() {
            findings.push(ManifestEditorSessionFinding::new(
                "manifest_editor_session.finding_anchor_missing",
                "findings with a field must carry a JSON-pointer anchor",
            ));
        }
    }

    if record.publish_readiness == ManifestEditorPublishReadinessClass::BlockedOnMustFix
        && record.blocker_count == 0
    {
        findings.push(ManifestEditorSessionFinding::new(
            "manifest_editor_session.publish_readiness_inconsistent",
            "blocked publish readiness requires at least one blocker",
        ));
    }
    if record.publish_readiness == ManifestEditorPublishReadinessClass::ReadyToPublish
        && (record.blocker_count != 0 || record.advisory_count != 0)
    {
        findings.push(ManifestEditorSessionFinding::new(
            "manifest_editor_session.publish_readiness_inconsistent",
            "ready-to-publish requires zero blockers and zero advisories",
        ));
    }

    findings
}

// ---------------------------------------------------------------------------
// Conformance (blocker) checks: faithful port of the validator CLI.
// ---------------------------------------------------------------------------

fn run_conformance_checks(manifest: &Value, findings: &mut Vec<ManifestEditorFinding>) {
    let root_is_object = require_object(Some(manifest), "(root)", findings);
    let root = if root_is_object { Some(manifest) } else { None };

    check_manifest_shape(root, findings);
    check_compatibility_targets(root, findings);
    check_permission_declarations(root, findings);
    check_lifecycle_metadata(root, findings);
    check_conformance_fixtures(root, findings);
}

fn check_manifest_shape(root: Option<&Value>, findings: &mut Vec<ManifestEditorFinding>) {
    let required_top = [
        "manifest_version",
        "package_id",
        "publisher_id",
        "version",
        "runtime_origin",
        "host_contract_family",
        "sdk",
        "compatibility",
        "permissions",
        "lifecycle",
        "conformance",
    ];
    let mut missing_top: Vec<&str> = required_top
        .iter()
        .copied()
        .filter(|key| vget(root, key).is_none())
        .collect();
    missing_top.sort_unstable();
    push_check(
        findings,
        "manifest_shape.required_fields",
        ManifestEditorFindingSuite::ManifestShape,
        missing_top.is_empty(),
        if missing_top.is_empty() {
            "manifest carries every required top-level section".to_string()
        } else {
            format!(
                "manifest is missing required top-level fields: {}",
                missing_top.join(", ")
            )
        },
        None,
        Some("Add the missing section before publishing."),
    );

    push_check(
        findings,
        "manifest_shape.schema_version",
        ManifestEditorFindingSuite::ManifestShape,
        vget(root, "manifest_version").and_then(Value::as_i64) == Some(MANIFEST_SCHEMA_VERSION),
        "manifest_version pins the beta manifest schema",
        Some("manifest_version"),
        Some("Set manifest_version to 1."),
    );

    let package_id = vget(root, "package_id").and_then(Value::as_str);
    push_check(
        findings,
        "manifest_shape.package_id",
        ManifestEditorFindingSuite::ManifestShape,
        package_id.map(is_package_id).unwrap_or(false),
        "package_id is a stable reverse-DNS style package id",
        Some("package_id"),
        Some("Use a stable package id such as com.example.tooling."),
    );

    let publisher_id = vget(root, "publisher_id").and_then(Value::as_str);
    push_check(
        findings,
        "manifest_shape.publisher_identity",
        ManifestEditorFindingSuite::ManifestShape,
        publisher_id
            .map(|p| !p.trim().is_empty() && p != "anonymous" && p != "unknown")
            .unwrap_or(false),
        "publisher_id is explicit and not anonymous",
        Some("publisher_id"),
        Some("Bind the package to a publisher identity before validation."),
    );

    push_check(
        findings,
        "manifest_shape.version_semver",
        ManifestEditorFindingSuite::ManifestShape,
        vget(root, "version")
            .and_then(Value::as_str)
            .map(is_semver)
            .unwrap_or(false),
        "version follows semver",
        Some("version"),
        Some("Use MAJOR.MINOR.PATCH with an optional prerelease suffix."),
    );

    push_check(
        findings,
        "manifest_shape.runtime_origin_known",
        ManifestEditorFindingSuite::ManifestShape,
        str_in(vget(root, "runtime_origin"), RUNTIME_ORIGINS),
        "runtime_origin uses the closed beta vocabulary",
        Some("runtime_origin"),
        Some("Choose one of: bridge, external_host, helper_binary, remote_side_component, wasm"),
    );

    push_check(
        findings,
        "manifest_shape.host_contract_family_known",
        ManifestEditorFindingSuite::ManifestShape,
        str_in(vget(root, "host_contract_family"), HOST_CONTRACT_FAMILIES),
        "host_contract_family uses the closed beta vocabulary",
        Some("host_contract_family"),
        Some("Choose a host contract family compatible with the runtime origin."),
    );
}

fn check_compatibility_targets(root: Option<&Value>, findings: &mut Vec<ManifestEditorFinding>) {
    let sdk = vget(root, "sdk");
    require_object(sdk, "sdk", findings);
    let compatibility = vget(root, "compatibility");
    require_object(compatibility, "compatibility", findings);
    let aureline_versions = vget(compatibility, "aureline_versions");
    require_object(
        aureline_versions,
        "compatibility.aureline_versions",
        findings,
    );

    let runtime_origin = vget(root, "runtime_origin").and_then(Value::as_str);
    let host_family = vget(root, "host_contract_family").and_then(Value::as_str);

    push_check(
        findings,
        "compatibility_targets.sdk_line_id",
        ManifestEditorFindingSuite::CompatibilityTargets,
        vget(sdk, "line_id").and_then(Value::as_str) == Some("aureline.sdk.beta"),
        "sdk.line_id targets the published beta SDK line",
        Some("sdk.line_id"),
        Some("Set sdk.line_id to aureline.sdk.beta for this validator lane."),
    );
    push_check(
        findings,
        "compatibility_targets.sdk_semver",
        ManifestEditorFindingSuite::CompatibilityTargets,
        vget(sdk, "line_semver")
            .and_then(Value::as_str)
            .map(is_semver)
            .unwrap_or(false),
        "sdk.line_semver follows semver",
        Some("sdk.line_semver"),
        Some("Use the SDK line semver published with the beta host."),
    );
    let lifecycle_refs = array_of(vget(sdk, "lifecycle_metadata_refs"));
    push_check(
        findings,
        "compatibility_targets.lifecycle_metadata_refs",
        ManifestEditorFindingSuite::CompatibilityTargets,
        !lifecycle_refs.is_empty()
            && lifecycle_refs
                .iter()
                .all(|v| v.as_str().map(is_lifecycle_ref).unwrap_or(false)),
        "sdk.lifecycle_metadata_refs cite canonical lifecycle rows",
        Some("sdk.lifecycle_metadata_refs"),
        Some("Cite lifecycle_row:* entries from the lifecycle metadata packet."),
    );
    push_check(
        findings,
        "compatibility_targets.host_abi_window",
        ManifestEditorFindingSuite::CompatibilityTargets,
        str_in(vget(sdk, "host_abi_window"), HOST_ABI_WINDOWS),
        "sdk.host_abi_window uses a supported beta ABI window",
        Some("sdk.host_abi_window"),
        Some("Use a documented beta ABI window for the selected runtime origin."),
    );
    push_check(
        findings,
        "compatibility_targets.host_family_matches_runtime_origin",
        ManifestEditorFindingSuite::CompatibilityTargets,
        match (runtime_origin, host_family) {
            (Some(origin), Some(family)) => host_family_allowed(origin, family),
            _ => false,
        },
        "host contract family is compatible with runtime origin",
        Some("host_contract_family"),
        Some("Align runtime_origin and host_contract_family before validation."),
    );
    let wit_refs = array_of(vget(sdk, "wit_world_refs"));
    push_check(
        findings,
        "compatibility_targets.wit_worlds_declared",
        ManifestEditorFindingSuite::CompatibilityTargets,
        runtime_origin != Some("wasm")
            || (!wit_refs.is_empty()
                && wit_refs
                    .iter()
                    .all(|v| v.as_str().map(is_wit_world).unwrap_or(false))),
        "wasm manifests declare WIT world refs",
        Some("sdk.wit_world_refs"),
        Some("List each claimed WIT world as aureline:<world>@MAJOR.MINOR.PATCH."),
    );
    push_check(
        findings,
        "compatibility_targets.external_host_contract_declared",
        ManifestEditorFindingSuite::CompatibilityTargets,
        runtime_origin != Some("external_host")
            || non_empty(vget(sdk, "external_host_contract_ref")),
        "external-host manifests declare an external host contract ref",
        Some("sdk.external_host_contract_ref"),
        Some("Add the supervised external-host contract ref."),
    );
    push_check(
        findings,
        "compatibility_targets.aureline_version_range",
        ManifestEditorFindingSuite::CompatibilityTargets,
        non_empty(vget(aureline_versions, "min")) && non_empty(vget(aureline_versions, "max")),
        "compatibility target declares min and max Aureline versions",
        Some("compatibility.aureline_versions"),
        Some("Declare the supported host version range."),
    );
    let platforms = array_of(vget(compatibility, "platforms"));
    push_check(
        findings,
        "compatibility_targets.platforms_declared",
        ManifestEditorFindingSuite::CompatibilityTargets,
        !platforms.is_empty()
            && platforms
                .iter()
                .all(|v| v.as_str().map(|s| !s.trim().is_empty()).unwrap_or(false)),
        "compatibility target declares at least one platform",
        Some("compatibility.platforms"),
        Some("List the supported OS/architecture target rows."),
    );
    push_check(
        findings,
        "compatibility_targets.support_class",
        ManifestEditorFindingSuite::CompatibilityTargets,
        str_in(vget(compatibility, "support_class"), SUPPORT_CLASSES),
        "support_class uses the shared compatibility vocabulary",
        Some("compatibility.support_class"),
        Some("Use a support class from the compatibility report vocabulary."),
    );
    push_check(
        findings,
        "compatibility_targets.bridge_state",
        ManifestEditorFindingSuite::CompatibilityTargets,
        str_in(vget(compatibility, "bridge_state"), BRIDGE_STATES),
        "bridge_state uses the shared bridge compatibility vocabulary",
        Some("compatibility.bridge_state"),
        Some("Use native, bridge, partial, unsupported, or retest_pending."),
    );
}

fn check_permission_declarations(root: Option<&Value>, findings: &mut Vec<ManifestEditorFinding>) {
    let permissions = array_of(vget(root, "permissions"));
    push_check(
        findings,
        "permission_declarations.present",
        ManifestEditorFindingSuite::PermissionDeclarations,
        !permissions.is_empty(),
        "manifest declares at least one permission entry",
        Some("permissions"),
        Some("Declare every permission scope the extension may request."),
    );

    let mut declared_scope_targets: std::collections::BTreeSet<(String, String)> =
        std::collections::BTreeSet::new();
    let mut total_target_rows: usize = 0;
    for (idx, entry) in permissions.iter().enumerate() {
        let scope = entry.get("scope").and_then(Value::as_str);
        let targets = array_of(entry.get("targets"));
        let field = format!("permissions[{idx}]");
        push_check(
            findings,
            "permission_declarations.scope_known",
            ManifestEditorFindingSuite::PermissionDeclarations,
            scope
                .map(|s| PERMISSION_SCOPES.contains(&s))
                .unwrap_or(false),
            format!("{field}.scope uses the closed permission vocabulary"),
            Some(&format!("{field}.scope")),
            Some("Use a scope from schemas/extensions/permission_manifest.schema.json."),
        );
        push_check(
            findings,
            "permission_declarations.targets_declared",
            ManifestEditorFindingSuite::PermissionDeclarations,
            !targets.is_empty()
                && targets
                    .iter()
                    .all(|v| v.as_str().map(|s| !s.trim().is_empty()).unwrap_or(false)),
            format!("{field}.targets declares at least one target"),
            Some(&format!("{field}.targets")),
            Some("Declare the target scope the permission applies to."),
        );
        push_check(
            findings,
            "permission_declarations.purpose_text",
            ManifestEditorFindingSuite::PermissionDeclarations,
            non_empty(entry.get("purpose")),
            format!("{field}.purpose is non-empty"),
            Some(&format!("{field}.purpose")),
            Some("Explain why the extension needs this permission."),
        );
        push_check(
            findings,
            "permission_declarations.trust_mode",
            ManifestEditorFindingSuite::PermissionDeclarations,
            str_in(entry.get("trust_mode"), TRUST_MODE_CLASSES),
            format!("{field}.trust_mode uses the trust-mode vocabulary"),
            Some(&format!("{field}.trust_mode")),
            Some("Declare how this permission behaves in restricted mode."),
        );
        let privileged = scope
            .map(|s| PRIVILEGED_SCOPES.contains(&s))
            .unwrap_or(false);
        let prompt_summary_present = non_empty(entry.get("prompt").and_then(|p| p.get("summary")));
        push_check(
            findings,
            "permission_declarations.prompt_copy",
            ManifestEditorFindingSuite::PermissionDeclarations,
            !privileged || prompt_summary_present,
            format!("{field} carries prompt copy when the scope is privileged"),
            Some(&format!("{field}.prompt.summary")),
            Some("Add prompt.summary for privileged scopes."),
        );
        push_check(
            findings,
            "permission_declarations.review_required_for_privileged_scope",
            ManifestEditorFindingSuite::PermissionDeclarations,
            !privileged || entry.get("review_required") == Some(&Value::Bool(true)),
            format!("{field} marks privileged scopes review-required"),
            Some(&format!("{field}.review_required")),
            Some("Set review_required to true for privileged scopes."),
        );
        if scope == Some("network_egress") {
            push_check(
                findings,
                "permission_declarations.network_endpoint_class",
                ManifestEditorFindingSuite::PermissionDeclarations,
                str_in(
                    entry.get("network").and_then(|n| n.get("endpoint_class")),
                    NETWORK_ENDPOINT_CLASSES,
                ),
                format!("{field}.network.endpoint_class is declared"),
                Some(&format!("{field}.network.endpoint_class")),
                Some("Declare the network endpoint class."),
            );
        }
        if scope == Some("secret_handle_use") {
            push_check(
                findings,
                "permission_declarations.secret_handle_only",
                ManifestEditorFindingSuite::PermissionDeclarations,
                entry.get("handle_only") == Some(&Value::Bool(true)),
                format!("{field} uses brokered secret handles instead of raw secrets"),
                Some(&format!("{field}.handle_only")),
                Some("Set handle_only to true for secret access."),
            );
        }
        total_target_rows += targets.len();
        if let Some(scope) = scope {
            for target in targets {
                if let Some(target) = target.as_str() {
                    declared_scope_targets.insert((scope.to_string(), target.to_string()));
                }
            }
        }
    }
    push_check(
        findings,
        "permission_declarations.no_duplicate_scope_target",
        ManifestEditorFindingSuite::PermissionDeclarations,
        declared_scope_targets.len() == total_target_rows,
        "permission scope-target pairs are not duplicated",
        Some("permissions"),
        Some("Collapse duplicate permission target rows into one entry."),
    );
}

fn check_lifecycle_metadata(root: Option<&Value>, findings: &mut Vec<ManifestEditorFinding>) {
    let lifecycle = vget(root, "lifecycle");
    require_object(lifecycle, "lifecycle", findings);
    let activation = vget(lifecycle, "activation");
    let degraded = vget(lifecycle, "degraded_path");
    let disable = vget(lifecycle, "disable");
    let rollback = vget(lifecycle, "rollback");

    push_check(
        findings,
        "lifecycle_metadata.state_known",
        ManifestEditorFindingSuite::LifecycleMetadata,
        str_in(vget(lifecycle, "state"), LIFECYCLE_STATES),
        "lifecycle.state uses the shared extension lifecycle vocabulary",
        Some("lifecycle.state"),
        Some("Use verified, resolved, activated, degraded, disabled, or removed."),
    );
    let triggers = array_of(vget(activation, "triggers"));
    push_check(
        findings,
        "lifecycle_metadata.activation_triggers",
        ManifestEditorFindingSuite::LifecycleMetadata,
        !triggers.is_empty()
            && triggers
                .iter()
                .all(|v| v.as_str().map(|s| !s.trim().is_empty()).unwrap_or(false)),
        "activation triggers are declared",
        Some("lifecycle.activation.triggers"),
        Some("Declare the events that can activate the extension."),
    );
    push_check(
        findings,
        "lifecycle_metadata.activation_budget",
        ManifestEditorFindingSuite::LifecycleMetadata,
        vget(activation, "budget_ms")
            .and_then(Value::as_i64)
            .map(|b| b > 0)
            .unwrap_or(false),
        "activation budget is declared in milliseconds",
        Some("lifecycle.activation.budget_ms"),
        Some("Set a positive activation budget in milliseconds."),
    );
    push_check(
        findings,
        "lifecycle_metadata.degraded_path",
        ManifestEditorFindingSuite::LifecycleMetadata,
        str_in(vget(degraded, "behavior_class"), DEGRADED_BEHAVIORS)
            && vget(degraded, "preserves_core_editing") == Some(&Value::Bool(true)),
        "degraded path preserves core editing and has a typed behavior",
        Some("lifecycle.degraded_path"),
        Some("Declare a typed degraded behavior that preserves local editing."),
    );
    push_check(
        findings,
        "lifecycle_metadata.disable_support",
        ManifestEditorFindingSuite::LifecycleMetadata,
        vget(disable, "supported") == Some(&Value::Bool(true))
            && vget(disable, "preserves_user_state") == Some(&Value::Bool(true)),
        "disable behavior is supported and preserves user state",
        Some("lifecycle.disable"),
        Some("Declare disable.supported and disable.preserves_user_state as true."),
    );
    push_check(
        findings,
        "lifecycle_metadata.rollback_support",
        ManifestEditorFindingSuite::LifecycleMetadata,
        vget(rollback, "supported") == Some(&Value::Bool(true))
            && non_empty(vget(rollback, "last_known_good_ref")),
        "rollback behavior names a last-known-good target",
        Some("lifecycle.rollback"),
        Some("Declare rollback.supported and rollback.last_known_good_ref."),
    );
}

fn check_conformance_fixtures(root: Option<&Value>, findings: &mut Vec<ManifestEditorFinding>) {
    let conformance = vget(root, "conformance");
    require_object(conformance, "conformance", findings);
    let fixture_rows = array_of(vget(conformance, "fixtures"));
    let observed: std::collections::BTreeSet<&str> = fixture_rows
        .iter()
        .filter_map(|row| row.get("scenario_class").and_then(Value::as_str))
        .collect();
    push_check(
        findings,
        "conformance_fixtures.required_scenario_coverage",
        ManifestEditorFindingSuite::ConformanceFixtures,
        SCENARIO_CLASSES.iter().all(|s| observed.contains(s)),
        "conformance fixtures cover install, activation, permission prompts, degraded paths, and disable or rollback",
        Some("conformance.fixtures"),
        Some("Add fixture rows for every required scenario class."),
    );
    for (idx, row) in fixture_rows.iter().enumerate() {
        let field = format!("conformance.fixtures[{idx}]");
        push_check(
            findings,
            "conformance_fixtures.scenario_class_known",
            ManifestEditorFindingSuite::ConformanceFixtures,
            str_in(row.get("scenario_class"), SCENARIO_CLASSES),
            format!("{field}.scenario_class uses the conformance scenario vocabulary"),
            Some(&format!("{field}.scenario_class")),
            Some("Use a known conformance scenario class."),
        );
        push_check(
            findings,
            "conformance_fixtures.fixture_ref",
            ManifestEditorFindingSuite::ConformanceFixtures,
            non_empty(row.get("fixture_ref")),
            format!("{field}.fixture_ref is non-empty"),
            Some(&format!("{field}.fixture_ref")),
            Some("Point the row at a replayable fixture or evidence ref."),
        );
    }
}

// ---------------------------------------------------------------------------
// Advisory (non-blocking) checks: editor-only UX / performance guidance.
// ---------------------------------------------------------------------------

fn run_advisory_checks(
    manifest: &Value,
    migration_hints: &[ManifestMigrationHint],
    findings: &mut Vec<ManifestEditorFinding>,
) {
    for hint in migration_hints {
        let mut message = format!(
            "manifest field `{}` value `{}` is deprecated",
            hint.field_path, hint.deprecated_value
        );
        if let Some(replacement) = &hint.replacement_value {
            message.push_str(&format!("; migrate to `{replacement}`"));
        }
        push_advisory(
            findings,
            "advisory.deprecated_manifest_field",
            ManifestEditorFindingSeverity::Warning,
            message,
            Some(&hint.field_path),
            hint.migration_guide_ref.clone(),
        );
    }

    let lifecycle = manifest.get("lifecycle");
    let activation = vget(lifecycle, "activation");
    if vget(activation, "startup") == Some(&Value::Bool(true)) {
        push_advisory(
            findings,
            "advisory.activation_eager_startup",
            ManifestEditorFindingSeverity::Warning,
            "activation participates in shell startup; prefer lazy activation unless startup work is required".to_string(),
            Some("lifecycle.activation.startup"),
            Some("Set lifecycle.activation.startup to false and rely on lazy triggers where possible.".to_string()),
        );
    }
    if let Some(budget) = vget(activation, "budget_ms").and_then(Value::as_i64) {
        if budget > 250 {
            push_advisory(
                findings,
                "advisory.activation_budget_generous",
                ManifestEditorFindingSeverity::Info,
                format!(
                    "activation budget is {budget} ms; a tighter budget keeps activation snappy"
                ),
                Some("lifecycle.activation.budget_ms"),
                Some(
                    "Aim for the smallest activation budget that fits the extension's work."
                        .to_string(),
                ),
            );
        }
    }

    let compatibility = manifest.get("compatibility");
    if let Some(support_class) = vget(compatibility, "support_class").and_then(Value::as_str) {
        if SUPPORT_CLASSES_NEEDING_QUALIFICATION.contains(&support_class) {
            push_advisory(
                findings,
                "advisory.compatibility_support_class_reduced",
                ManifestEditorFindingSeverity::Info,
                format!(
                    "support_class `{support_class}` renders a reduced-support badge in marketplace and install review"
                ),
                Some("compatibility.support_class"),
                Some("Qualify the extension to raise the support class when ready.".to_string()),
            );
        }
    }
    if let Some(bridge_state) = vget(compatibility, "bridge_state").and_then(Value::as_str) {
        if bridge_state == "bridge" || bridge_state == "partial" {
            push_advisory(
                findings,
                "advisory.bridge_state_requires_shim",
                ManifestEditorFindingSeverity::Info,
                format!(
                    "bridge_state `{bridge_state}` requires a compatibility shim and shows a translated/partial badge to users"
                ),
                Some("compatibility.bridge_state"),
                Some("Document the shim and verify the bridged capability window.".to_string()),
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Migration hints from the lifecycle metadata packet.
// ---------------------------------------------------------------------------

fn collect_migration_hints(manifest: &Value) -> Vec<ManifestMigrationHint> {
    let packet = match current_extension_lifecycle_metadata_packet() {
        Ok(packet) => packet,
        Err(_) => return Vec::new(),
    };
    let mut hints = Vec::new();
    for row in &packet.rows {
        if row.surface_kind != LifecycleSurfaceKind::ManifestSchema {
            continue;
        }
        if !row_is_deprecated(row) {
            continue;
        }
        let Some((field_path, deprecated_value)) = parse_manifest_field_surface(&row.surface_ref)
        else {
            continue;
        };
        let actual = navigate(manifest, &field_path).and_then(Value::as_str);
        if actual != Some(deprecated_value.as_str()) {
            continue;
        }
        let replacement_value = row
            .deprecation
            .replacement_surface_ref
            .as_deref()
            .and_then(parse_manifest_field_surface)
            .map(|(_, value)| value);
        let field_anchor = field_path_to_anchor(&field_path);
        let leaf = field_path.rsplit('.').next().unwrap_or(field_path.as_str());
        let replacement_example = replacement_value
            .as_ref()
            .map(|value| format!("\"{leaf}\": \"{value}\""));
        let guidance = build_migration_guidance(row, &field_path, &deprecated_value);
        hints.push(ManifestMigrationHint {
            lifecycle_row_ref: row.row_id.clone(),
            field_path,
            field_anchor,
            deprecated_value,
            posture: row.deprecation.deprecation_posture_class,
            replacement_value,
            no_direct_replacement_reason: row.deprecation.no_direct_replacement_reason.clone(),
            removal_target_version: row.deprecation.removal_target_version.clone(),
            removal_target_date: row.deprecation.removal_target_date.clone(),
            migration_guide_ref: row.deprecation.migration_guide_ref.clone(),
            replacement_example,
            guidance,
        });
    }
    hints
}

fn build_migration_guidance(
    row: &LifecycleMetadataRow,
    field_path: &str,
    deprecated_value: &str,
) -> String {
    let mut guidance = format!("`{field_path}` value `{deprecated_value}` is deprecated");
    if let Some(replacement) = row
        .deprecation
        .replacement_surface_ref
        .as_deref()
        .and_then(parse_manifest_field_surface)
        .map(|(_, value)| value)
    {
        guidance.push_str(&format!("; replace it with `{replacement}`"));
    } else if let Some(reason) = &row.deprecation.no_direct_replacement_reason {
        guidance.push_str(&format!("; no direct replacement: {reason}"));
    }
    match (
        &row.deprecation.removal_target_version,
        &row.deprecation.removal_target_date,
    ) {
        (Some(version), Some(date)) => {
            guidance.push_str(&format!(". Removal target: {version} ({date})"))
        }
        (Some(version), None) => guidance.push_str(&format!(". Removal target: {version}")),
        (None, Some(date)) => guidance.push_str(&format!(". Removal target: {date}")),
        (None, None) => {}
    }
    if let Some(migration) = &row.deprecation.migration_guide_ref {
        guidance.push_str(&format!(". See {migration}"));
    }
    guidance.push('.');
    guidance
}

fn parse_manifest_field_surface(surface_ref: &str) -> Option<(String, String)> {
    let rest = surface_ref.strip_prefix(MANIFEST_FIELD_SURFACE_PREFIX)?;
    let (path, value) = rest.rsplit_once('.')?;
    if path.is_empty() || value.is_empty() {
        return None;
    }
    Some((path.to_string(), value.to_string()))
}

fn row_is_deprecated(row: &LifecycleMetadataRow) -> bool {
    matches!(
        row.stability_label,
        LifecycleStabilityLabel::Deprecated | LifecycleStabilityLabel::Retired
    ) || row.deprecation.deprecation_posture_class
        != LifecycleDeprecationPostureClass::NotDeprecated
}

// ---------------------------------------------------------------------------
// Permission explanation chips.
// ---------------------------------------------------------------------------

fn collect_permission_chips(manifest: &Value) -> Vec<PermissionExplanationChip> {
    let permissions = array_of(manifest.get("permissions"));
    let mut chips = Vec::with_capacity(permissions.len());
    for (idx, entry) in permissions.iter().enumerate() {
        let scope = entry.get("scope").and_then(Value::as_str).unwrap_or("");
        let scope_class = parse_permission_scope(scope);
        let capability_class = scope_class.map(capability_class_for_scope);
        let privileged = PRIVILEGED_SCOPES.contains(&scope);
        let trust_mode = entry
            .get("trust_mode")
            .and_then(Value::as_str)
            .filter(|s| TRUST_MODE_CLASSES.contains(s))
            .map(str::to_string);
        let target_count = array_of(entry.get("targets")).len() as u32;
        let purpose_present = non_empty(entry.get("purpose"));
        let review_required = entry.get("review_required") == Some(&Value::Bool(true));
        let prompt_summary_present = non_empty(entry.get("prompt").and_then(|p| p.get("summary")));
        let network_endpoint_class = entry
            .get("network")
            .and_then(|n| n.get("endpoint_class"))
            .and_then(Value::as_str)
            .map(str::to_string);
        let handle_only = entry.get("handle_only") == Some(&Value::Bool(true));

        let explanation = build_permission_explanation(
            scope,
            scope_class.is_some(),
            capability_class,
            privileged,
            trust_mode.as_deref(),
            review_required,
        );

        chips.push(PermissionExplanationChip {
            anchor: format!("/permissions/{idx}"),
            scope: scope.to_string(),
            scope_known: scope_class.is_some(),
            capability_class,
            privileged,
            trust_mode,
            target_count,
            purpose_present,
            review_required,
            prompt_summary_present,
            network_endpoint_class,
            handle_only,
            explanation,
        });
    }
    chips
}

fn build_permission_explanation(
    scope: &str,
    scope_known: bool,
    capability_class: Option<CapabilityClassClass>,
    privileged: bool,
    trust_mode: Option<&str>,
    review_required: bool,
) -> String {
    if !scope_known {
        return format!(
            "`{scope}` is not in the closed permission vocabulary; the host cannot reason about its impact"
        );
    }
    let capability = capability_class
        .map(capability_class_label)
        .unwrap_or("unknown");
    let mut explanation = format!("Grants {capability} capability");
    if privileged {
        explanation.push_str(" (privileged: widens trust and needs review)");
    } else {
        explanation.push_str(" (non-privileged)");
    }
    if let Some(trust_mode) = trust_mode {
        explanation.push_str(&format!("; restricted-mode behavior: {trust_mode}"));
    }
    if review_required {
        explanation.push_str("; flagged review-required");
    }
    explanation.push('.');
    explanation
}

fn capability_class_label(class: CapabilityClassClass) -> &'static str {
    match class {
        CapabilityClassClass::Network => "network",
        CapabilityClassClass::Filesystem => "filesystem",
        CapabilityClassClass::Process => "process",
        CapabilityClassClass::Data => "data",
        CapabilityClassClass::Ui => "ui",
        CapabilityClassClass::Credential => "credential",
    }
}

fn parse_permission_scope(scope: &str) -> Option<crate::manifest_baseline::PermissionScopeClass> {
    use crate::manifest_baseline::PermissionScopeClass as P;
    Some(match scope {
        "filesystem_read" => P::FilesystemRead,
        "filesystem_write" => P::FilesystemWrite,
        "shell_execute" => P::ShellExecute,
        "network_egress" => P::NetworkEgress,
        "ai_provider_access" => P::AiProviderAccess,
        "connected_provider_access" => P::ConnectedProviderAccess,
        "secret_handle_use" => P::SecretHandleUse,
        "workspace_settings_read" => P::WorkspaceSettingsRead,
        "workspace_settings_write" => P::WorkspaceSettingsWrite,
        "execution_context_bind" => P::ExecutionContextBind,
        "subscription_subscribe" => P::SubscriptionSubscribe,
        "ui_command_contribute" => P::UiCommandContribute,
        "capability_inherit" => P::CapabilityInherit,
        _ => return None,
    })
}

// ---------------------------------------------------------------------------
// Version targeting summary.
// ---------------------------------------------------------------------------

fn summarize_version_targeting(manifest: &Value) -> VersionTargetingSummary {
    let sdk = manifest.get("sdk");
    let compatibility = manifest.get("compatibility");
    let aureline_versions = vget(compatibility, "aureline_versions");
    let bridge_state = vget(compatibility, "bridge_state")
        .and_then(Value::as_str)
        .map(str::to_string);
    let required_shim = matches!(bridge_state.as_deref(), Some("bridge") | Some("partial"));
    let shim_note = match bridge_state.as_deref() {
        Some("bridge") => {
            Some("A compatibility bridge translates the declared capability set.".to_string())
        }
        Some("partial") => {
            Some("Only part of the declared capability set is supported on target.".to_string())
        }
        _ => None,
    };
    VersionTargetingSummary {
        sdk_line_id: vget(sdk, "line_id")
            .and_then(Value::as_str)
            .map(str::to_string),
        sdk_line_semver: vget(sdk, "line_semver")
            .and_then(Value::as_str)
            .map(str::to_string),
        host_abi_window: vget(sdk, "host_abi_window")
            .and_then(Value::as_str)
            .map(str::to_string),
        aureline_version_min: vget(aureline_versions, "min")
            .and_then(Value::as_str)
            .map(str::to_string),
        aureline_version_max: vget(aureline_versions, "max")
            .and_then(Value::as_str)
            .map(str::to_string),
        platforms: array_of(vget(compatibility, "platforms"))
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect(),
        support_class: vget(compatibility, "support_class")
            .and_then(Value::as_str)
            .map(str::to_string),
        bridge_state,
        required_shim,
        shim_note,
        lifecycle_metadata_refs: array_of(vget(sdk, "lifecycle_metadata_refs"))
            .iter()
            .filter_map(|v| v.as_str().map(str::to_string))
            .collect(),
    }
}

// ---------------------------------------------------------------------------
// Conformance export and publish readiness.
// ---------------------------------------------------------------------------

fn project_conformance_export(
    findings: &[ManifestEditorFinding],
) -> ManifestEditorConformanceExport {
    let conformance: Vec<&ManifestEditorFinding> = findings
        .iter()
        .filter(|f| f.suite != ManifestEditorFindingSuite::EditorAdvisory)
        .collect();
    let passed = conformance
        .iter()
        .filter(|f| f.status == ManifestEditorFindingStatus::Pass)
        .count() as u32;
    let warnings = conformance
        .iter()
        .filter(|f| f.status == ManifestEditorFindingStatus::Warn)
        .count() as u32;
    let failed = conformance
        .iter()
        .filter(|f| f.status == ManifestEditorFindingStatus::Fail)
        .count() as u32;
    let blockers = conformance
        .iter()
        .filter(|f| {
            f.status == ManifestEditorFindingStatus::Fail
                && f.severity == ManifestEditorFindingSeverity::Blocker
        })
        .count() as u32;

    let result_class = if blockers > 0 {
        ManifestEditorResultClass::Fail
    } else if warnings > 0 {
        ManifestEditorResultClass::Warn
    } else {
        ManifestEditorResultClass::Pass
    };
    let compatibility_badge_class = if result_class == ManifestEditorResultClass::Fail {
        ManifestEditorCompatibilityBadgeClass::UnsupportedPendingQualification
    } else {
        ManifestEditorCompatibilityBadgeClass::CompatibleOnDeclaredTargets
    };

    let mut red_flags: std::collections::BTreeSet<&'static str> = std::collections::BTreeSet::new();
    for finding in conformance.iter().filter(|f| {
        f.status == ManifestEditorFindingStatus::Fail
            && f.severity == ManifestEditorFindingSeverity::Blocker
    }) {
        for flag in classify_red_flags(&finding.check_id) {
            red_flags.insert(flag);
        }
    }

    ManifestEditorConformanceExport {
        validator_id: MANIFEST_EDITOR_VALIDATOR_ID.to_string(),
        validator_version: MANIFEST_EDITOR_VALIDATOR_VERSION.to_string(),
        result_class,
        compatibility_badge_class,
        red_flag_classes: red_flags.into_iter().map(str::to_string).collect(),
        passed,
        failed,
        warnings,
        blockers,
    }
}

fn classify_red_flags(check_id: &str) -> Vec<&'static str> {
    let mut flags = Vec::new();
    if check_id.starts_with("manifest_shape.required")
        || check_id.starts_with("manifest_shape.schema")
    {
        flags.push("missing_manifest_shape");
    }
    if check_id == "manifest_shape.publisher_identity" {
        flags.push("opaque_publisher_identity");
    }
    if check_id.starts_with("permission_declarations.") {
        flags.push("undeclared_privileged_permission");
    }
    if check_id.starts_with("lifecycle_metadata.") {
        flags.push("missing_lifecycle_metadata");
    }
    if check_id.starts_with("compatibility_targets.") {
        flags.push("incompatible_sdk_target");
    }
    if check_id == "lifecycle_metadata.disable_support"
        || check_id == "lifecycle_metadata.rollback_support"
    {
        flags.push("missing_disable_or_rollback");
    }
    flags
}

fn decide_publish_readiness(
    blocker_count: u32,
    advisory_count: u32,
) -> (
    ManifestEditorPublishReadinessClass,
    ManifestEditorPublishReadinessReasonClass,
    String,
) {
    if blocker_count > 0 {
        return (
            ManifestEditorPublishReadinessClass::BlockedOnMustFix,
            ManifestEditorPublishReadinessReasonClass::MustFixBlockersPresent,
            format!(
                "{blocker_count} must-fix blocker(s) must be resolved before publishing; {advisory_count} advisory(ies) noted"
            ),
        );
    }
    if advisory_count > 0 {
        return (
            ManifestEditorPublishReadinessClass::ReadyWithAdvisories,
            ManifestEditorPublishReadinessReasonClass::NoBlockersAdvisoriesPresent,
            format!("no blockers; {advisory_count} recommended improvement(s) noted"),
        );
    }
    (
        ManifestEditorPublishReadinessClass::ReadyToPublish,
        ManifestEditorPublishReadinessReasonClass::NoBlockersNoAdvisories,
        "no blockers and no advisories; manifest is ready to publish".to_string(),
    )
}

// ---------------------------------------------------------------------------
// JSON helpers.
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn push_check(
    findings: &mut Vec<ManifestEditorFinding>,
    check_id: &str,
    suite: ManifestEditorFindingSuite,
    ok: bool,
    message: impl Into<String>,
    field: Option<&str>,
    fix: Option<&str>,
) {
    findings.push(ManifestEditorFinding {
        check_id: check_id.to_string(),
        suite,
        status: if ok {
            ManifestEditorFindingStatus::Pass
        } else {
            ManifestEditorFindingStatus::Fail
        },
        severity: ManifestEditorFindingSeverity::Blocker,
        message: message.into(),
        field: field.map(str::to_string),
        anchor: field.map(field_path_to_anchor),
        fix: fix.map(str::to_string),
    });
}

fn push_advisory(
    findings: &mut Vec<ManifestEditorFinding>,
    check_id: &str,
    severity: ManifestEditorFindingSeverity,
    message: String,
    field: Option<&str>,
    fix: Option<String>,
) {
    findings.push(ManifestEditorFinding {
        check_id: check_id.to_string(),
        suite: ManifestEditorFindingSuite::EditorAdvisory,
        status: ManifestEditorFindingStatus::Warn,
        severity,
        message,
        field: field.map(str::to_string),
        anchor: field.map(field_path_to_anchor),
        fix,
    });
}

fn require_object(
    value: Option<&Value>,
    label: &str,
    findings: &mut Vec<ManifestEditorFinding>,
) -> bool {
    if matches!(value, Some(Value::Object(_))) {
        return true;
    }
    findings.push(ManifestEditorFinding {
        check_id: "manifest_shape.object_required".to_string(),
        suite: ManifestEditorFindingSuite::ManifestShape,
        status: ManifestEditorFindingStatus::Fail,
        severity: ManifestEditorFindingSeverity::Blocker,
        message: format!("{label} must be a JSON object"),
        field: Some(label.to_string()),
        anchor: Some(field_path_to_anchor(label)),
        fix: Some("Use an object for this manifest section.".to_string()),
    });
    false
}

fn vget<'a>(value: Option<&'a Value>, key: &str) -> Option<&'a Value> {
    value.and_then(|v| v.get(key))
}

fn array_of(value: Option<&Value>) -> &[Value] {
    value
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[])
}

fn non_empty(value: Option<&Value>) -> bool {
    value
        .and_then(Value::as_str)
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false)
}

fn str_in(value: Option<&Value>, allowed: &[&str]) -> bool {
    value
        .and_then(Value::as_str)
        .map(|s| allowed.contains(&s))
        .unwrap_or(false)
}

fn string_field(manifest: &Value, key: &str) -> Option<String> {
    manifest
        .get(key)
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn navigate<'a>(root: &'a Value, dotted: &str) -> Option<&'a Value> {
    let mut current = root;
    for segment in dotted.split('.') {
        current = current.get(segment)?;
    }
    Some(current)
}

/// Convert a validator-style field path (`permissions[0].scope`,
/// `compatibility.aureline_versions`, `(root)`) into a JSON pointer.
fn field_path_to_anchor(field: &str) -> String {
    if field == "(root)" {
        return String::new();
    }
    let mut anchor = String::new();
    for segment in field.split('.') {
        let mut name = segment;
        let mut indices: Vec<&str> = Vec::new();
        if let Some(open) = segment.find('[') {
            name = &segment[..open];
            let mut rest = &segment[open..];
            while let Some(close) = rest.find(']') {
                let idx = &rest[1..close];
                indices.push(idx);
                rest = &rest[close + 1..];
                if !rest.starts_with('[') {
                    break;
                }
            }
        }
        if !name.is_empty() {
            anchor.push('/');
            anchor.push_str(&escape_pointer(name));
        }
        for idx in indices {
            anchor.push('/');
            anchor.push_str(&escape_pointer(idx));
        }
    }
    anchor
}

fn escape_pointer(segment: &str) -> String {
    segment.replace('~', "~0").replace('/', "~1")
}

fn host_family_allowed(runtime_origin: &str, host_family: &str) -> bool {
    match runtime_origin {
        "wasm" => host_family == "wasm_component_model" || host_family == "wasm_core_module",
        "external_host" => host_family == "external_host_process",
        "helper_binary" => host_family == "helper_binary",
        "remote_side_component" => host_family == "remote_side_component",
        "bridge" => host_family == "compatibility_bridge",
        _ => false,
    }
}

fn is_semver(s: &str) -> bool {
    let (core, pre) = match s.split_once('-') {
        Some((c, p)) => (c, Some(p)),
        None => (s, None),
    };
    let parts: Vec<&str> = core.split('.').collect();
    if parts.len() != 3 {
        return false;
    }
    if !parts
        .iter()
        .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
    {
        return false;
    }
    if let Some(pre) = pre {
        if pre.is_empty() {
            return false;
        }
        for ident in pre.split('.') {
            if ident.is_empty()
                || !ident
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-')
            {
                return false;
            }
        }
    }
    true
}

fn is_package_id(s: &str) -> bool {
    let segments: Vec<&str> = s.split('.').collect();
    if segments.len() < 2 {
        return false;
    }
    let first = segments[0];
    let mut chars = first.chars();
    match chars.next() {
        Some(c) if c.is_ascii_lowercase() => {}
        _ => return false,
    }
    if !chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return false;
    }
    segments[1..].iter().all(|seg| {
        !seg.is_empty()
            && seg
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    })
}

fn is_wit_world(s: &str) -> bool {
    let Some(rest) = s.strip_prefix("aureline:") else {
        return false;
    };
    let Some((name, ver)) = rest.split_once('@') else {
        return false;
    };
    let bytes = name.as_bytes();
    if bytes.len() < 2 {
        return false;
    }
    if !(bytes[0] as char).is_ascii_lowercase() {
        return false;
    }
    let last = bytes[bytes.len() - 1] as char;
    if !(last.is_ascii_lowercase() || last.is_ascii_digit()) {
        return false;
    }
    for &b in &bytes[1..bytes.len() - 1] {
        let c = b as char;
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            return false;
        }
    }
    let vparts: Vec<&str> = ver.split('.').collect();
    vparts.len() == 3
        && vparts
            .iter()
            .all(|p| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit()))
}

fn is_lifecycle_ref(s: &str) -> bool {
    let Some(rest) = s.strip_prefix("lifecycle_row:") else {
        return false;
    };
    !rest.is_empty()
        && rest
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '.' | ':' | '-'))
}
