use serde::{Deserialize, Serialize};

use crate::descriptor::RepairHookRef;
use crate::invocation::ArgumentProvenanceEntry;

/// Frozen enablement-decision vocabulary used across shell surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnablementDecisionClass {
    Enabled,
    DisabledWithReason,
    HiddenWithReason,
}

impl EnablementDecisionClass {
    /// Returns the canonical snake_case token for this decision class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Enabled => "enabled",
            Self::DisabledWithReason => "disabled_with_reason",
            Self::HiddenWithReason => "hidden_with_reason",
        }
    }
}

/// Frozen disabled-reason vocabulary cited by enablement decisions and denied
/// invocation outcomes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisabledReasonCode {
    WorkspaceTrustRestricted,
    CapabilityLifecycleRetired,
    CapabilityDisabledByPolicy,
    KillSwitchTripped,
    ClientScopeExcludesSurface,
    FreshnessFloorUnmet,
    RequiredProviderUnlinked,
    RequiredCredentialMissing,
    RequiredArgumentUnresolved,
    ExecutionContextUnavailable,
    ManagedOnlyChannelRequired,
    DependencyStateBelowCommandCeiling,
    CommandDeprecatedWithinWindow,
    CommandRetired,
    CommandVersionUnknown,
    PreviewDenialNoSafePreview,
    ApprovalDenialNoApprovalPath,
    PublisherNotPermitted,
    PolicyBlockedInContext,
    AuthorityClassUnresolved,
    IssuingSurfaceUnresolved,
    ScopeClassDriftedFromDescriptor,
    PreviewRequiredNotShown,
    BasisSnapshotDrifted,
}

impl DisabledReasonCode {
    /// Returns the canonical snake_case token for this disabled-reason code.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceTrustRestricted => "workspace_trust_restricted",
            Self::CapabilityLifecycleRetired => "capability_lifecycle_retired",
            Self::CapabilityDisabledByPolicy => "capability_disabled_by_policy",
            Self::KillSwitchTripped => "kill_switch_tripped",
            Self::ClientScopeExcludesSurface => "client_scope_excludes_surface",
            Self::FreshnessFloorUnmet => "freshness_floor_unmet",
            Self::RequiredProviderUnlinked => "required_provider_unlinked",
            Self::RequiredCredentialMissing => "required_credential_missing",
            Self::RequiredArgumentUnresolved => "required_argument_unresolved",
            Self::ExecutionContextUnavailable => "execution_context_unavailable",
            Self::ManagedOnlyChannelRequired => "managed_only_channel_required",
            Self::DependencyStateBelowCommandCeiling => "dependency_state_below_command_ceiling",
            Self::CommandDeprecatedWithinWindow => "command_deprecated_within_window",
            Self::CommandRetired => "command_retired",
            Self::CommandVersionUnknown => "command_version_unknown",
            Self::PreviewDenialNoSafePreview => "preview_denial_no_safe_preview",
            Self::ApprovalDenialNoApprovalPath => "approval_denial_no_approval_path",
            Self::PublisherNotPermitted => "publisher_not_permitted",
            Self::PolicyBlockedInContext => "policy_blocked_in_context",
            Self::AuthorityClassUnresolved => "authority_class_unresolved",
            Self::IssuingSurfaceUnresolved => "issuing_surface_unresolved",
            Self::ScopeClassDriftedFromDescriptor => "scope_class_drifted_from_descriptor",
            Self::PreviewRequiredNotShown => "preview_required_not_shown",
            Self::BasisSnapshotDrifted => "basis_snapshot_drifted",
        }
    }
}

impl std::fmt::Display for DisabledReasonCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// One disabled-reason record attached to a registry entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisabledReasonRecord {
    pub disabled_reason_code: DisabledReasonCode,
    pub owner_boundary_class: String,
    pub explanation_ref: String,
    pub repair_hook_ref: RepairHookRef,
    pub fallback_command_id: Option<String>,
}

/// The enablement snapshot embedded in registry entries and projected to
/// shell/palette/keybinding surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnablementSnapshot {
    pub decision_class: EnablementDecisionClass,
    pub disabled_reason_code: Option<DisabledReasonCode>,
    pub repair_hook_ref: Option<RepairHookRef>,
}

impl EnablementSnapshot {
    /// Returns an enablement snapshot representing an enabled command.
    pub fn enabled() -> Self {
        Self {
            decision_class: EnablementDecisionClass::Enabled,
            disabled_reason_code: None,
            repair_hook_ref: None,
        }
    }

    /// Returns an enablement snapshot representing a disabled command.
    pub fn disabled_with_reason(
        disabled_reason_code: DisabledReasonCode,
        repair_hook_ref: RepairHookRef,
    ) -> Self {
        Self {
            decision_class: EnablementDecisionClass::DisabledWithReason,
            disabled_reason_code: Some(disabled_reason_code),
            repair_hook_ref: Some(repair_hook_ref),
        }
    }
}

/// Runtime context used by the enablement engine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandEnablementContext {
    pub client_scope: String,
    pub workspace_trust_state: String,
    pub execution_context_available: bool,
    #[serde(default)]
    pub provider_linked: Option<bool>,
    #[serde(default)]
    pub credential_available: Option<bool>,
    #[serde(default)]
    pub policy_disabled: bool,
    #[serde(default)]
    pub policy_blocked_in_context: bool,
    #[serde(default)]
    pub argument_provenance_map: Vec<ArgumentProvenanceEntry>,
}

/// Preflight decision classification for a would-be invocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreflightDecisionClass {
    Allowed,
    BlockedByPolicy,
    DisabledWithReason,
    PreviewRequired,
    ApprovalRequired,
}

/// Preflight decision returned by the enablement engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightDecision {
    pub decision_class: PreflightDecisionClass,
    pub enablement_snapshot: EnablementSnapshot,
}

fn required_arguments_unresolved<'a>(
    mut required_argument_names: impl Iterator<Item = &'a str>,
    argument_provenance_map: &[ArgumentProvenanceEntry],
) -> bool {
    required_argument_names.any(|argument_name| {
        argument_provenance_map
            .iter()
            .find(|row| row.argument_name == argument_name)
            .and_then(|row| row.resolved_value_ref.as_ref())
            .is_none()
    })
}

fn repair_hook_for(
    disabled_reason_records: &[DisabledReasonRecord],
    default_enablement_repair_hook_ref: Option<&RepairHookRef>,
    disabled_reason_code: DisabledReasonCode,
) -> Option<RepairHookRef> {
    disabled_reason_records
        .iter()
        .find(|record| record.disabled_reason_code == disabled_reason_code)
        .map(|record| record.repair_hook_ref.clone())
        .or_else(|| default_enablement_repair_hook_ref.cloned())
}

fn snapshot_for(
    disabled_reason_records: &[DisabledReasonRecord],
    default_enablement_repair_hook_ref: Option<&RepairHookRef>,
    disabled_reason_code: DisabledReasonCode,
) -> Option<EnablementSnapshot> {
    let repair_hook_ref = repair_hook_for(
        disabled_reason_records,
        default_enablement_repair_hook_ref,
        disabled_reason_code,
    )?;
    Some(EnablementSnapshot::disabled_with_reason(
        disabled_reason_code,
        repair_hook_ref,
    ))
}

/// Evaluates the command's enablement snapshot for the provided runtime context.
///
/// The snapshot is used verbatim by shell surfaces; callers must not mint their
/// own `(decision_class, disabled_reason_code, repair_hook_ref)` tuple.
pub fn evaluate_enablement(
    descriptor_client_scopes: &[String],
    descriptor_lifecycle_state: &str,
    descriptor_default_enablement_repair_hook_ref: Option<&RepairHookRef>,
    descriptor_typed_arguments: &[crate::descriptor::TypedArgument],
    seed_enablement_snapshot: &EnablementSnapshot,
    disabled_reason_records: &[DisabledReasonRecord],
    context: &CommandEnablementContext,
) -> EnablementSnapshot {
    if !descriptor_client_scopes
        .iter()
        .any(|scope| scope == &context.client_scope)
    {
        if let Some(snapshot) = snapshot_for(
            disabled_reason_records,
            descriptor_default_enablement_repair_hook_ref,
            DisabledReasonCode::ClientScopeExcludesSurface,
        ) {
            return snapshot;
        }
    }

    if descriptor_lifecycle_state == "retired" {
        if let Some(snapshot) = snapshot_for(
            disabled_reason_records,
            descriptor_default_enablement_repair_hook_ref,
            DisabledReasonCode::CommandRetired,
        ) {
            return snapshot;
        }
    }

    if descriptor_lifecycle_state == "disabled_by_policy" || context.policy_disabled {
        if let Some(snapshot) = snapshot_for(
            disabled_reason_records,
            descriptor_default_enablement_repair_hook_ref,
            DisabledReasonCode::CapabilityDisabledByPolicy,
        ) {
            return snapshot;
        }
    }

    if context.policy_blocked_in_context {
        if let Some(snapshot) = snapshot_for(
            disabled_reason_records,
            descriptor_default_enablement_repair_hook_ref,
            DisabledReasonCode::PolicyBlockedInContext,
        ) {
            return snapshot;
        }
    }

    if context.workspace_trust_state != "trusted" {
        if let Some(snapshot) = snapshot_for(
            disabled_reason_records,
            descriptor_default_enablement_repair_hook_ref,
            DisabledReasonCode::WorkspaceTrustRestricted,
        ) {
            return snapshot;
        }
    }

    if !context.execution_context_available {
        if let Some(snapshot) = snapshot_for(
            disabled_reason_records,
            descriptor_default_enablement_repair_hook_ref,
            DisabledReasonCode::ExecutionContextUnavailable,
        ) {
            return snapshot;
        }
    }

    if matches!(context.provider_linked, Some(false)) {
        if let Some(snapshot) = snapshot_for(
            disabled_reason_records,
            descriptor_default_enablement_repair_hook_ref,
            DisabledReasonCode::RequiredProviderUnlinked,
        ) {
            return snapshot;
        }
    }

    if matches!(context.credential_available, Some(false)) {
        if let Some(snapshot) = snapshot_for(
            disabled_reason_records,
            descriptor_default_enablement_repair_hook_ref,
            DisabledReasonCode::RequiredCredentialMissing,
        ) {
            return snapshot;
        }
    }

    let required_argument_names = descriptor_typed_arguments
        .iter()
        .filter(|slot| slot.is_required)
        .map(|slot| slot.argument_name.as_str());
    if required_arguments_unresolved(required_argument_names, &context.argument_provenance_map) {
        if let Some(snapshot) = snapshot_for(
            disabled_reason_records,
            descriptor_default_enablement_repair_hook_ref,
            DisabledReasonCode::RequiredArgumentUnresolved,
        ) {
            return snapshot;
        }
    }

    seed_enablement_snapshot.clone()
}

/// Computes the preflight decision a surface should use before attempting
/// dispatch.
pub fn preflight(
    descriptor_client_scopes: &[String],
    descriptor_lifecycle_state: &str,
    descriptor_default_enablement_repair_hook_ref: Option<&RepairHookRef>,
    descriptor_typed_arguments: &[crate::descriptor::TypedArgument],
    descriptor_preview_class: &str,
    descriptor_approval_posture_class: &str,
    seed_enablement_snapshot: &EnablementSnapshot,
    disabled_reason_records: &[DisabledReasonRecord],
    context: &CommandEnablementContext,
) -> PreflightDecision {
    let enablement_snapshot = evaluate_enablement(
        descriptor_client_scopes,
        descriptor_lifecycle_state,
        descriptor_default_enablement_repair_hook_ref,
        descriptor_typed_arguments,
        seed_enablement_snapshot,
        disabled_reason_records,
        context,
    );

    if enablement_snapshot.decision_class != EnablementDecisionClass::Enabled {
        let is_policy = matches!(
            enablement_snapshot.disabled_reason_code,
            Some(
                DisabledReasonCode::CapabilityDisabledByPolicy
                    | DisabledReasonCode::PolicyBlockedInContext
            )
        );
        return PreflightDecision {
            decision_class: if is_policy {
                PreflightDecisionClass::BlockedByPolicy
            } else {
                PreflightDecisionClass::DisabledWithReason
            },
            enablement_snapshot,
        };
    }

    if descriptor_preview_class != "no_preview_required" {
        return PreflightDecision {
            decision_class: PreflightDecisionClass::PreviewRequired,
            enablement_snapshot,
        };
    }

    if descriptor_approval_posture_class != "no_approval_required" {
        return PreflightDecision {
            decision_class: PreflightDecisionClass::ApprovalRequired,
            enablement_snapshot,
        };
    }

    PreflightDecision {
        decision_class: PreflightDecisionClass::Allowed,
        enablement_snapshot,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::registry::seeded_registry;

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    struct EnablementCaseRecord {
        record_kind: String,
        schema_version: u32,
        case_id: String,
        #[serde(default)]
        case_summary: Option<String>,
        command_id: String,
        context: CommandEnablementContext,
        expected: EnablementExpectation,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    struct EnablementExpectation {
        decision_class: EnablementDecisionClass,
        disabled_reason_code: Option<DisabledReasonCode>,
    }

    fn read_fixture(path: &std::path::Path) -> String {
        std::fs::read_to_string(path).expect("fixture must read")
    }

    #[test]
    fn evaluates_disabled_reason_cases_from_fixtures() {
        let registry = seeded_registry();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/commands/disabled_reason_cases");

        for entry in std::fs::read_dir(&root).expect("fixture directory must exist") {
            let entry = entry.expect("fixture directory entry must read");
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }

            let payload = read_fixture(&path);
            let record: EnablementCaseRecord =
                serde_json::from_str(&payload).expect("enablement fixture must parse");
            assert_eq!(record.record_kind, "command_enablement_case_record");
            assert_eq!(record.schema_version, 1);
            assert!(
                !record.case_id.trim().is_empty(),
                "case_id must be non-empty"
            );

            let Some(command) = registry.get(&record.command_id) else {
                panic!(
                    "fixture references unknown command_id: {}",
                    record.command_id
                );
            };
            let snapshot = command.evaluate_enablement(&record.context);
            assert_eq!(
                snapshot.decision_class, record.expected.decision_class,
                "unexpected decision class for {}",
                record.command_id
            );
            assert_eq!(
                snapshot.disabled_reason_code, record.expected.disabled_reason_code,
                "unexpected disabled reason for {}",
                record.command_id
            );

            if snapshot.decision_class != EnablementDecisionClass::Enabled {
                assert!(
                    snapshot.disabled_reason_code.is_some(),
                    "disabled snapshots must carry a disabled_reason_code (case: {})",
                    record.case_id
                );
                assert!(
                    snapshot.repair_hook_ref.is_some(),
                    "disabled snapshots must carry a repair_hook_ref (case: {})",
                    record.case_id
                );
            }
        }
    }
}
