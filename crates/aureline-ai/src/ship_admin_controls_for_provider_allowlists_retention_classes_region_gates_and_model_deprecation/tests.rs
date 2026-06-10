use super::*;

const PACKET_ID: &str = "admin-controls:stable:0001";

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

/// A Stable provider-allowlist control that denies a provider org-wide.
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

/// A Stable retention-floor control that rejects routes below a disclosed floor.
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

/// A Stable region-gate control pinned to a named region set.
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

/// A Stable model-deprecation control with a scheduled sunset and replacement.
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

fn source_contracts() -> Vec<String> {
    vec![
        ADMIN_CONTROLS_SCHEMA_REF.to_owned(),
        ADMIN_CONTROLS_DOC_REF.to_owned(),
        PROVIDER_ROUTE_DISCLOSURE_SCHEMA_REF.to_owned(),
        PROVIDER_MODEL_REGISTRY_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
        TOOL_GATEWAY_DESCRIPTOR_SCHEMA_REF.to_owned(),
    ]
}

fn proof_freshness() -> AdminControlsProofFreshness {
    AdminControlsProofFreshness {
        proof_freshness_slo_hours: 24,
        last_proof_refresh: "2026-06-10T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> AdminControlPacket {
    AdminControlPacket::new(AdminControlPacketInput {
        packet_id: PACKET_ID.to_owned(),
        catalogue_label: "Admin controls catalogue".to_owned(),
        controls: vec![
            deny_provider_control(),
            retention_floor_control(),
            region_gate_control(),
            model_deprecation_control(),
        ],
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contracts(),
        redaction_class_token: "admin_controls_review_safe".to_owned(),
        minted_at: "2026-06-10T00:00:00Z".to_owned(),
    })
}

#[test]
fn baseline_packet_validates() {
    assert!(packet().validate().is_empty());
}

#[test]
fn families_resolve_from_directive() {
    let packet = packet();
    assert_eq!(
        packet.family_count(AdminControlFamilyClass::ProviderAllowlist),
        1
    );
    assert_eq!(
        packet.family_count(AdminControlFamilyClass::RetentionClassFloor),
        1
    );
    assert_eq!(packet.family_count(AdminControlFamilyClass::RegionGate), 1);
    assert_eq!(
        packet.family_count(AdminControlFamilyClass::ModelDeprecation),
        1
    );
    assert_eq!(packet.claimed_control_count(), 4);
    assert_eq!(packet.denial_control_count(), 3);
}

#[test]
fn round_trips_through_json() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: AdminControlPacket = serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(packet, parsed);
}

#[test]
fn provider_admin_block_projection() {
    let packet = packet();
    assert!(packet.is_provider_admin_blocked("provider:acme"));
    assert!(!packet.is_provider_admin_blocked("provider:managed"));
}

#[test]
fn live_controls_are_enforced_or_monitored() {
    assert_eq!(packet().live_controls().count(), 4);
}

#[test]
fn narrowed_qualification_follows_rules() {
    let control = deny_provider_control();
    assert_eq!(
        control.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Held
    );
    assert_eq!(
        control.narrowed_qualification(M5AiWorkflowDowngradeTrigger::TrustNarrowing),
        M5AiWorkflowQualificationClass::Stable
    );
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&AdminControlViolation::WrongRecordKind));
}

#[test]
fn wrong_schema_version_fails() {
    let mut packet = packet();
    packet.schema_version = 99;
    assert!(packet
        .validate()
        .contains(&AdminControlViolation::WrongSchemaVersion));
}

#[test]
fn missing_source_contract_fails() {
    let mut packet = packet();
    packet.source_contract_refs = vec![ADMIN_CONTROLS_SCHEMA_REF.to_owned()];
    assert!(packet
        .validate()
        .contains(&AdminControlViolation::MissingSourceContracts));
}

#[test]
fn duplicate_control_fails() {
    let mut packet = packet();
    packet.controls.push(deny_provider_control());
    assert!(packet
        .validate()
        .contains(&AdminControlViolation::DuplicateControl));
}

#[test]
fn no_controls_fails() {
    let mut packet = packet();
    packet.controls.clear();
    assert!(packet
        .validate()
        .contains(&AdminControlViolation::NoControls));
}

#[test]
fn model_deprecation_requires_model_target() {
    let mut control = model_deprecation_control();
    control.target_model_id = String::new();
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::ModelDeprecationMissingModelTarget));
}

#[test]
fn non_deprecation_rejects_model_target() {
    let mut control = retention_floor_control();
    control.target_model_id = "model:should-not-be-here".to_owned();
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::NonDeprecationCarriesModelTarget));
}

#[test]
fn retention_floor_must_be_disclosed() {
    let mut control = retention_floor_control();
    control.directive = AdminControlDirective::RetentionFloor(RetentionFloorDirective {
        required_floor: RouteRetentionClass::UnknownUnverified,
        denies_below_floor: true,
        floor_label: "floor".to_owned(),
    });
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::RetentionFloorNotDisclosed));
}

#[test]
fn region_gate_pinned_requires_tags() {
    let mut control = region_gate_control();
    control.directive = AdminControlDirective::RegionGate(RegionGateDirective {
        allowed_region_posture: RouteRegionClass::SingleRegionPinned,
        allowed_region_tags: Vec::new(),
        denies_outside_gate: true,
        gate_label: "gate".to_owned(),
    });
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::RegionGatePinnedWithoutTags));
}

#[test]
fn region_gate_unpinned_rejects_tags() {
    let mut control = region_gate_control();
    control.directive = AdminControlDirective::RegionGate(RegionGateDirective {
        allowed_region_posture: RouteRegionClass::OnDeviceOnly,
        allowed_region_tags: vec!["eu".to_owned()],
        denies_outside_gate: false,
        gate_label: "gate".to_owned(),
    });
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::RegionGateUnpinnedWithTags));
}

#[test]
fn scheduled_sunset_requires_date() {
    let mut control = model_deprecation_control();
    control.directive = AdminControlDirective::ModelDeprecation(ModelDeprecationDirective {
        lifecycle_stage: ModelLifecycleStageClass::DeprecatedSunsetScheduled,
        sunset_date: String::new(),
        replacement_model_ref: "model:next".to_owned(),
        migration_path_ref: "docs/x.md".to_owned(),
        stage_label: "stage".to_owned(),
    });
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::DeprecationMissingSunsetDate));
}

#[test]
fn announced_deprecation_requires_migration_path() {
    let mut control = model_deprecation_control();
    control.directive = AdminControlDirective::ModelDeprecation(ModelDeprecationDirective {
        lifecycle_stage: ModelLifecycleStageClass::DeprecationAnnounced,
        sunset_date: String::new(),
        replacement_model_ref: String::new(),
        migration_path_ref: String::new(),
        stage_label: "stage".to_owned(),
    });
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::DeprecationMissingMigrationPath));
}

#[test]
fn denial_control_requires_admin_gate() {
    let mut control = deny_provider_control();
    control.admin_authority = ToolApprovalPostureClass::AllowedWithoutPrompt;
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::DenialControlWithoutAdminGate));
}

#[test]
fn denial_control_requires_audit() {
    let mut control = deny_provider_control();
    control.audited = false;
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::DenialControlNotAudited));
}

#[test]
fn claimed_denial_must_be_live() {
    let mut control = deny_provider_control();
    control.enforcement_state = AdminControlStateClass::DraftNotEnforced;
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::ClaimedDenialNotEnforced));
}

#[test]
fn blocked_control_cannot_claim_qualification() {
    let mut control = deny_provider_control();
    control.enforcement_state = AdminControlStateClass::BlockedByHigherPolicy;
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::BlockedControlClaimsQualification));
}

#[test]
fn pending_allowlist_cannot_claim_stable() {
    let mut control = deny_provider_control();
    control.directive = AdminControlDirective::ProviderAllowlist(ProviderAllowlistDirective {
        decision: AdminAllowlistDecisionClass::ProviderPendingReview,
        decision_label: "Awaiting admin review".to_owned(),
    });
    // Pending is not a denial, so the live-state denial rule does not apply.
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::PendingAllowlistClaimsStable));
}

#[test]
fn claimed_control_requires_evidence() {
    let mut control = region_gate_control();
    control.evidence_packet_refs.clear();
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::ClaimedControlMissingEvidence));
}

#[test]
fn claimed_reversible_rollback_must_be_verified() {
    let mut control = region_gate_control();
    control.rollback_verified = false;
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::ClaimedRollbackUnverified));
}

#[test]
fn downgrade_rules_required() {
    let mut control = region_gate_control();
    control.downgrade_rules.clear();
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::DowngradeRulesMissing));
}

#[test]
fn downgrade_rule_must_narrow() {
    let mut control = region_gate_control();
    control.downgrade_rules = vec![
        proof_stale(M5AiWorkflowQualificationClass::Stable),
        provider_unavailable(M5AiWorkflowQualificationClass::Held),
    ];
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::DowngradeRuleNotNarrowing));
}

#[test]
fn missing_provider_unavailable_trigger_fails() {
    let mut control = region_gate_control();
    control.downgrade_rules = vec![proof_stale(M5AiWorkflowQualificationClass::Beta)];
    let mut violations = Vec::new();
    validate_control(&control, &mut violations);
    assert!(violations.contains(&AdminControlViolation::DowngradeRuleMissingProviderUnavailable));
}

#[test]
fn raw_provider_material_is_rejected() {
    let mut packet = packet();
    packet.controls[0].explanation_label = "see https://acme.example/keys".to_owned();
    assert!(packet
        .validate()
        .contains(&AdminControlViolation::RawProviderMaterialInExport));
}

#[test]
fn markdown_summary_is_deterministic() {
    let summary = packet().render_markdown_summary();
    assert_eq!(summary, packet().render_markdown_summary());
    assert!(summary.contains("Admin Controls"));
    assert!(summary.contains("provider `provider:acme`"));
}

#[test]
fn checked_export_validates() {
    let packet = current_admin_controls_export().expect("checked admin control export validates");
    assert_eq!(packet.record_kind, ADMIN_CONTROLS_RECORD_KIND);
    assert!(packet.claimed_control_count() >= 1);
}

#[test]
fn blocked_control_fixture_validates() {
    let packet: AdminControlPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/ship_admin_controls_for_provider_allowlists_retention_classes_region_gates_and_model_deprecation/blocked_control_narrows.json"
    )))
    .expect("blocked control fixture parses");
    assert!(packet.validate().is_empty());
    // The fixture demonstrates a control overridden by higher policy that has
    // narrowed its claim to a non-public lane.
    assert!(packet.controls.iter().any(|c| c.enforcement_state
        == AdminControlStateClass::BlockedByHigherPolicy
        && !c.is_claimed()));
}
