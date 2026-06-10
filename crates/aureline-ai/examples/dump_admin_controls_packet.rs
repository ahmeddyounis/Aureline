//! Conformance dump for the admin-controls packet (provider allowlists,
//! retention classes, region gates, and model deprecation).
//!
//! Prints the canonical support export so the checked-in artifact and fixtures
//! stay byte-aligned with the in-crate builder.

use aureline_ai::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass, M5AiWorkflowRollbackPosture,
    M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use aureline_ai::materialize_the_provider_and_model_registry_local_or_byok_or_managed_mode_disclosure_and_route_inspectors::{
    ExecutionModeClass, RouteRegionClass, RouteRetentionClass, PROVIDER_MODEL_REGISTRY_SCHEMA_REF,
    PROVIDER_ROUTE_DISCLOSURE_SCHEMA_REF,
};
use aureline_ai::ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation::*;
use aureline_ai::tool_gateway::{ToolApprovalPostureClass, TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF};

fn proof_stale(narrowed_to: M5AiWorkflowQualificationClass) -> AdminControlDowngradeRule {
    AdminControlDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to,
        auto_enforced: true,
        rationale: "Stale proof narrows the public claim".to_owned(),
    }
}

fn provider_unavailable(narrowed_to: M5AiWorkflowQualificationClass) -> AdminControlDowngradeRule {
    AdminControlDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProviderUnavailable,
        narrowed_to,
        auto_enforced: true,
        rationale: "Provider outage or quota exhaustion narrows the claim".to_owned(),
    }
}

fn deny_provider_control() -> AdminControlRow {
    AdminControlRow {
        control_id: "allowlist-deny-acme".to_owned(),
        control_label: "Deny Acme provider org-wide".to_owned(),
        control_family_label: "Provider allowlist".to_owned(),
        target_provider_id: "provider:acme".to_owned(),
        target_provider_label: "Acme Cloud Models".to_owned(),
        target_model_id: String::new(),
        governed_execution_mode: ExecutionModeClass::Byok,
        directive: AdminControlDirective::ProviderAllowlist(ProviderAllowlistDirective {
            decision: AdminAllowlistDecisionClass::ProviderDeniedByPolicy,
            decision_label: "Acme is denied by data-residency policy".to_owned(),
        }),
        enforcement_scope: AdminEnforcementScopeClass::OrganisationWide,
        enforcement_state: AdminControlStateClass::EnforcedActive,
        admin_authority: ToolApprovalPostureClass::RequiresAdminApproval,
        admin_identity_ref: "admin-identity:org-owner".to_owned(),
        audited: true,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Beta),
            provider_unavailable(M5AiWorkflowQualificationClass::Held),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:allowlist-deny-acme".to_owned()],
        explanation_label: "Blocks all BYOK dispatch to Acme org-wide".to_owned(),
    }
}

fn allow_managed_control() -> AdminControlRow {
    AdminControlRow {
        control_id: "allowlist-allow-managed".to_owned(),
        control_label: "Allow first-party managed under conditions".to_owned(),
        control_family_label: "Provider allowlist".to_owned(),
        target_provider_id: "provider:managed".to_owned(),
        target_provider_label: "First-party managed".to_owned(),
        target_model_id: String::new(),
        governed_execution_mode: ExecutionModeClass::Managed,
        directive: AdminControlDirective::ProviderAllowlist(ProviderAllowlistDirective {
            decision: AdminAllowlistDecisionClass::ProviderAllowedWithConditions,
            decision_label: "Managed allowed when the region gate and retention floor hold"
                .to_owned(),
        }),
        enforcement_scope: AdminEnforcementScopeClass::OrganisationWide,
        enforcement_state: AdminControlStateClass::EnforcedActive,
        admin_authority: ToolApprovalPostureClass::RequiresAdminApproval,
        admin_identity_ref: "admin-identity:org-owner".to_owned(),
        audited: true,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Beta),
            provider_unavailable(M5AiWorkflowQualificationClass::Held),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::FullyReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:allowlist-allow-managed".to_owned()],
        explanation_label: "Conditionally allows managed dispatch".to_owned(),
    }
}

fn retention_floor_control() -> AdminControlRow {
    AdminControlRow {
        control_id: "retention-floor-no-training".to_owned(),
        control_label: "Require no-training retention floor".to_owned(),
        control_family_label: "Retention class floor".to_owned(),
        target_provider_id: "provider:managed".to_owned(),
        target_provider_label: "First-party managed".to_owned(),
        target_model_id: String::new(),
        governed_execution_mode: ExecutionModeClass::Managed,
        directive: AdminControlDirective::RetentionFloor(RetentionFloorDirective {
            required_floor: RouteRetentionClass::NoRetentionPromised,
            denies_below_floor: true,
            floor_label: "Routes must promise request/response discard".to_owned(),
        }),
        enforcement_scope: AdminEnforcementScopeClass::TenantScoped,
        enforcement_state: AdminControlStateClass::EnforcedActive,
        admin_authority: ToolApprovalPostureClass::RequiresAdminApproval,
        admin_identity_ref: "admin-identity:tenant-admin".to_owned(),
        audited: true,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Beta),
            provider_unavailable(M5AiWorkflowQualificationClass::Held),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:retention-floor".to_owned()],
        explanation_label: "Denies any route below the no-training retention floor".to_owned(),
    }
}

fn region_gate_control() -> AdminControlRow {
    AdminControlRow {
        control_id: "region-gate-eu".to_owned(),
        control_label: "Pin managed routes to EU regions".to_owned(),
        control_family_label: "Region gate".to_owned(),
        target_provider_id: "provider:managed".to_owned(),
        target_provider_label: "First-party managed".to_owned(),
        target_model_id: String::new(),
        governed_execution_mode: ExecutionModeClass::Managed,
        directive: AdminControlDirective::RegionGate(RegionGateDirective {
            allowed_region_posture: RouteRegionClass::MultiRegionPinned,
            allowed_region_tags: vec!["eu-west".to_owned(), "eu-central".to_owned()],
            denies_outside_gate: true,
            gate_label: "Managed routes pinned to EU regions only".to_owned(),
        }),
        enforcement_scope: AdminEnforcementScopeClass::WorkspaceScoped,
        enforcement_state: AdminControlStateClass::EnforcedActive,
        admin_authority: ToolApprovalPostureClass::RequiresAdminApproval,
        admin_identity_ref: "admin-identity:workspace-owner".to_owned(),
        audited: true,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Beta),
            provider_unavailable(M5AiWorkflowQualificationClass::Held),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:region-gate-eu".to_owned()],
        explanation_label: "Denies any route outside the pinned EU region set".to_owned(),
    }
}

fn model_deprecation_control() -> AdminControlRow {
    AdminControlRow {
        control_id: "deprecate-legacy-model".to_owned(),
        control_label: "Deprecate legacy model with sunset".to_owned(),
        control_family_label: "Model deprecation".to_owned(),
        target_provider_id: "provider:managed".to_owned(),
        target_provider_label: "First-party managed".to_owned(),
        target_model_id: "model:legacy-large".to_owned(),
        governed_execution_mode: ExecutionModeClass::Managed,
        directive: AdminControlDirective::ModelDeprecation(ModelDeprecationDirective {
            lifecycle_stage: ModelLifecycleStageClass::DeprecatedSunsetScheduled,
            sunset_date: "2026-12-31T00:00:00Z".to_owned(),
            replacement_model_ref: "model:next-large".to_owned(),
            migration_path_ref: "docs/automation/m5/model_migration.md".to_owned(),
            stage_label: "Sunset scheduled; migrate to the next model".to_owned(),
        }),
        enforcement_scope: AdminEnforcementScopeClass::OrganisationWide,
        enforcement_state: AdminControlStateClass::EnforcedActive,
        admin_authority: ToolApprovalPostureClass::RequiresAdminApproval,
        admin_identity_ref: "admin-identity:org-owner".to_owned(),
        audited: true,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Beta),
            provider_unavailable(M5AiWorkflowQualificationClass::Held),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::CheckpointReversible,
        rollback_verified: true,
        evidence_packet_refs: vec!["evidence:deprecate-legacy".to_owned()],
        explanation_label: "Schedules sunset of the legacy model and names its replacement"
            .to_owned(),
    }
}

/// A region gate overridden by a higher-tier policy that has narrowed its claim
/// out of the public lanes rather than presenting an unenforced qualification.
fn blocked_region_gate_control() -> AdminControlRow {
    AdminControlRow {
        control_id: "region-gate-apac-blocked".to_owned(),
        control_label: "APAC region gate overridden by org policy".to_owned(),
        control_family_label: "Region gate".to_owned(),
        target_provider_id: "provider:managed".to_owned(),
        target_provider_label: "First-party managed".to_owned(),
        target_model_id: String::new(),
        governed_execution_mode: ExecutionModeClass::Managed,
        directive: AdminControlDirective::RegionGate(RegionGateDirective {
            allowed_region_posture: RouteRegionClass::SingleRegionPinned,
            allowed_region_tags: vec!["apac-southeast".to_owned()],
            denies_outside_gate: true,
            gate_label: "APAC gate overridden by the organisation-wide EU pin".to_owned(),
        }),
        enforcement_scope: AdminEnforcementScopeClass::WorkspaceScoped,
        enforcement_state: AdminControlStateClass::BlockedByHigherPolicy,
        admin_authority: ToolApprovalPostureClass::RequiresAdminApproval,
        admin_identity_ref: "admin-identity:workspace-owner".to_owned(),
        audited: true,
        claimed_qualification: M5AiWorkflowQualificationClass::Held,
        downgrade_rules: vec![
            proof_stale(M5AiWorkflowQualificationClass::Unavailable),
            provider_unavailable(M5AiWorkflowQualificationClass::Unavailable),
        ],
        rollback_posture: M5AiWorkflowRollbackPosture::NotApplicable,
        rollback_verified: false,
        evidence_packet_refs: vec!["evidence:region-gate-apac".to_owned()],
        explanation_label: "Overridden by the org-wide EU pin; claims nothing public".to_owned(),
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        ADMIN_CONTROLS_SCHEMA_REF.to_owned(),
        ADMIN_CONTROLS_DOC_REF.to_owned(),
        PROVIDER_ROUTE_DISCLOSURE_SCHEMA_REF.to_owned(),
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
        TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF.to_owned(),
    ]
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = match which.as_str() {
        "fixture" => AdminControlPacket::new(AdminControlPacketInput {
            packet_id: "admin-controls:fixture:blocked:0001".to_owned(),
            catalogue_label: "Region Gate Overridden By Higher Policy".to_owned(),
            controls: vec![region_gate_control(), blocked_region_gate_control()],
            proof_freshness: AdminControlsProofFreshness {
                proof_freshness_slo_hours: 168,
                last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
                auto_narrow_on_stale: true,
            },
            source_contract_refs: source_contract_refs(),
            redaction_class_token: "metadata_safe_default".to_owned(),
            minted_at: "2026-06-09T00:00:00Z".to_owned(),
        }),
        _ => AdminControlPacket::new(AdminControlPacketInput {
            packet_id: "admin-controls:stable:0001".to_owned(),
            catalogue_label: "Admin Controls: Provider, Retention, Region, Deprecation".to_owned(),
            controls: vec![
                deny_provider_control(),
                allow_managed_control(),
                retention_floor_control(),
                region_gate_control(),
                model_deprecation_control(),
            ],
            proof_freshness: AdminControlsProofFreshness {
                proof_freshness_slo_hours: 168,
                last_proof_refresh: "2026-06-09T00:00:00Z".to_owned(),
                auto_narrow_on_stale: true,
            },
            source_contract_refs: source_contract_refs(),
            redaction_class_token: "metadata_safe_default".to_owned(),
            minted_at: "2026-06-09T00:00:00Z".to_owned(),
        }),
    };

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );
    println!("{}", packet.export_safe_json());
}
