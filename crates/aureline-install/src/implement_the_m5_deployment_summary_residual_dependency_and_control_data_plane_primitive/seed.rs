// Canonical seed for the M5 deployment-summary primitive. Included from `mod.rs` so the
// seeded builder, its worked cases, the fixture generator, and the on-disk support
// export all stay byte-aligned.

/// Convenience constructor for one residual-dependency input.
fn residual(
    vendor_dependency_ref: &str,
    dependency_class: M5ResidualDependencyClass,
    required_for_operation: bool,
    failure_consequence: M5ResidualFailureConsequence,
    mitigation: M5ResidualMitigation,
) -> M5ResidualDependencyInput {
    M5ResidualDependencyInput {
        vendor_dependency_ref: vendor_dependency_ref.to_owned(),
        dependency_class,
        required_for_operation,
        failure_consequence,
        mitigation,
        disclosed: true,
    }
}

/// A shared, multi-tenant, vendor-managed deployment on the About deployment card.
fn shared_managed_about_input() -> M5DeploymentSummaryInput {
    M5DeploymentSummaryInput {
        deployment_id: "deployment:shared-managed:0001".to_owned(),
        surface_label: "About deployment summary card for a shared managed tenant".to_owned(),
        deployment_scope: M5DeploymentScopeClass::SharedManaged,
        operating_mode: M5DeploymentMode::Managed,
        tenant_org_ref: "tenant:acme-eng".to_owned(),
        region_ref: "region:us-east".to_owned(),
        mirror_offline_posture: M5DeploymentTruthMode::Live,
        last_control_plane_sync_ref: "sync:control-plane:2026-07-04T15:00Z".to_owned(),
        truth_mode: M5DeploymentTruthMode::Live,
        control_plane_state: M5PlaneState::Operational,
        data_plane_state: M5PlaneState::Operational,
        local_runtime_impaired: false,
        control_plane_impairment_flagged_as_local: false,
        local_safe_next_step: M5LocalSafeNextStep::ContinueLocalWork,
        residual_dependencies: vec![
            residual(
                "vendor:identity-provider",
                M5ResidualDependencyClass::IdentityProvider,
                true,
                M5ResidualFailureConsequence::BlocksSignIn,
                M5ResidualMitigation::AdminProvisioned,
            ),
            residual(
                "vendor:update-delivery",
                M5ResidualDependencyClass::UpdateDelivery,
                false,
                M5ResidualFailureConsequence::BlocksUpdates,
                M5ResidualMitigation::OfflineFallback,
            ),
        ],
        open_details_available: true,
        export_available: true,
        degraded: None,
    }
}

/// A self-hosted deployment whose vendor license-activation and model services remain
/// (strong boundary that discloses its required residual dependency, control-plane
/// degraded while local runtime keeps working).
fn self_hosted_admin_input() -> M5DeploymentSummaryInput {
    M5DeploymentSummaryInput {
        deployment_id: "deployment:self-hosted:0002".to_owned(),
        surface_label: "Admin deployment console for a self-hosted deployment".to_owned(),
        deployment_scope: M5DeploymentScopeClass::SelfHosted,
        operating_mode: M5DeploymentMode::SelfHosted,
        tenant_org_ref: "tenant:acme-self-hosted".to_owned(),
        region_ref: "region:on-prem-dc1".to_owned(),
        mirror_offline_posture: M5DeploymentTruthMode::Mirrored,
        last_control_plane_sync_ref: "sync:license:2026-07-03T09:12Z".to_owned(),
        truth_mode: M5DeploymentTruthMode::Mirrored,
        control_plane_state: M5PlaneState::Degraded,
        data_plane_state: M5PlaneState::Operational,
        local_runtime_impaired: false,
        control_plane_impairment_flagged_as_local: false,
        local_safe_next_step: M5LocalSafeNextStep::ContinueLocalWork,
        residual_dependencies: vec![
            residual(
                "vendor:license-activation",
                M5ResidualDependencyClass::LicenseActivation,
                true,
                M5ResidualFailureConsequence::BlocksActivation,
                M5ResidualMitigation::AdminProvisioned,
            ),
            residual(
                "vendor:model-inference",
                M5ResidualDependencyClass::ModelInferenceService,
                false,
                M5ResidualFailureConsequence::DegradesOptionalFeature,
                M5ResidualMitigation::SelfHostAlternative,
            ),
        ],
        open_details_available: true,
        export_available: true,
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::ResidualVendorDependency,
            degraded_label:
                "Vendor license activation is unreachable; the console names the residual dependency, its activation-blocking consequence, and the admin-provisioned alternative while local editing continues"
                    .to_owned(),
        }),
    }
}

/// A sovereign / air-gapped deployment with no live vendor control plane (control plane
/// unavailable by design, local runtime unaffected, one disclosed optional residual).
fn sovereign_service_health_input() -> M5DeploymentSummaryInput {
    M5DeploymentSummaryInput {
        deployment_id: "deployment:sovereign:0003".to_owned(),
        surface_label: "Service-health panel for a sovereign air-gapped deployment".to_owned(),
        deployment_scope: M5DeploymentScopeClass::Sovereign,
        operating_mode: M5DeploymentMode::AirGapped,
        tenant_org_ref: "tenant:gov-sovereign".to_owned(),
        region_ref: "region:sovereign-enclave".to_owned(),
        mirror_offline_posture: M5DeploymentTruthMode::CachedOffline,
        last_control_plane_sync_ref: "sync:none:air-gapped".to_owned(),
        truth_mode: M5DeploymentTruthMode::CachedOffline,
        control_plane_state: M5PlaneState::Unavailable,
        data_plane_state: M5PlaneState::Operational,
        local_runtime_impaired: false,
        control_plane_impairment_flagged_as_local: false,
        local_safe_next_step: M5LocalSafeNextStep::WorkOfflineCached,
        residual_dependencies: vec![residual(
            "vendor:telemetry-channel",
            M5ResidualDependencyClass::TelemetryChannel,
            false,
            M5ResidualFailureConsequence::NoUserImpact,
            M5ResidualMitigation::DisableFeature,
        )],
        open_details_available: true,
        export_available: true,
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::OfflineCacheOnly,
            degraded_label:
                "This sovereign enclave has no live vendor control plane by design; the panel keeps the control plane distinct as unavailable while the local data plane and workspace stay operational offline"
                    .to_owned(),
        }),
    }
}

/// A dedicated managed deployment seen from diagnostics with both planes degraded.
fn dedicated_diagnostics_input() -> M5DeploymentSummaryInput {
    M5DeploymentSummaryInput {
        deployment_id: "deployment:dedicated-managed:0004".to_owned(),
        surface_label: "Diagnostics deployment pane for a dedicated managed tenant".to_owned(),
        deployment_scope: M5DeploymentScopeClass::DedicatedManaged,
        operating_mode: M5DeploymentMode::Managed,
        tenant_org_ref: "tenant:acme-dedicated".to_owned(),
        region_ref: "region:eu-west".to_owned(),
        mirror_offline_posture: M5DeploymentTruthMode::Live,
        last_control_plane_sync_ref: "sync:control-plane:2026-07-04T11:47Z".to_owned(),
        truth_mode: M5DeploymentTruthMode::Live,
        control_plane_state: M5PlaneState::Degraded,
        data_plane_state: M5PlaneState::Degraded,
        local_runtime_impaired: false,
        control_plane_impairment_flagged_as_local: false,
        local_safe_next_step: M5LocalSafeNextStep::RetryControlPlane,
        residual_dependencies: vec![residual(
            "vendor:identity-provider",
            M5ResidualDependencyClass::IdentityProvider,
            true,
            M5ResidualFailureConsequence::BlocksSignIn,
            M5ResidualMitigation::AdminProvisioned,
        )],
        open_details_available: true,
        export_available: true,
        degraded: Some(DegradedState {
            trigger: M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
            degraded_label:
                "The managed control plane is degraded; diagnostics keeps identity/policy health separate from the workspace data plane and names the retry-control-plane next step"
                    .to_owned(),
        }),
    }
}

/// A shared managed deployment reconstructed from an imported support snapshot.
fn support_replay_input() -> M5DeploymentSummaryInput {
    M5DeploymentSummaryInput {
        deployment_id: "deployment:support-replay:0005".to_owned(),
        surface_label: "Support / export replay reconstructing a deployment snapshot".to_owned(),
        deployment_scope: M5DeploymentScopeClass::SharedManaged,
        operating_mode: M5DeploymentMode::Managed,
        tenant_org_ref: "tenant:acme-eng".to_owned(),
        region_ref: "region:us-east".to_owned(),
        mirror_offline_posture: M5DeploymentTruthMode::Imported,
        last_control_plane_sync_ref: "sync:snapshot:2026-07-02T18:30Z".to_owned(),
        truth_mode: M5DeploymentTruthMode::Imported,
        control_plane_state: M5PlaneState::Unknown,
        data_plane_state: M5PlaneState::Operational,
        local_runtime_impaired: false,
        control_plane_impairment_flagged_as_local: false,
        local_safe_next_step: M5LocalSafeNextStep::ContinueLocalWork,
        residual_dependencies: vec![residual(
            "vendor:telemetry-channel",
            M5ResidualDependencyClass::TelemetryChannel,
            false,
            M5ResidualFailureConsequence::NoUserImpact,
            M5ResidualMitigation::DisableFeature,
        )],
        open_details_available: true,
        export_available: true,
        degraded: None,
    }
}

/// A local-only desktop deployment on the docs reference surface (no control plane, no
/// residual vendor dependency).
fn local_only_docs_input() -> M5DeploymentSummaryInput {
    M5DeploymentSummaryInput {
        deployment_id: "deployment:local-only:0006".to_owned(),
        surface_label: "Docs deployment reference for a local-only desktop install".to_owned(),
        deployment_scope: M5DeploymentScopeClass::LocalOnly,
        operating_mode: M5DeploymentMode::Desktop,
        tenant_org_ref: "tenant:local".to_owned(),
        region_ref: "region:local-device".to_owned(),
        mirror_offline_posture: M5DeploymentTruthMode::Live,
        last_control_plane_sync_ref: "sync:none:local-only".to_owned(),
        truth_mode: M5DeploymentTruthMode::Live,
        control_plane_state: M5PlaneState::Unknown,
        data_plane_state: M5PlaneState::Operational,
        local_runtime_impaired: false,
        control_plane_impairment_flagged_as_local: false,
        local_safe_next_step: M5LocalSafeNextStep::ContinueLocalWork,
        residual_dependencies: Vec::new(),
        open_details_available: true,
        export_available: true,
        degraded: None,
    }
}

fn case(input: M5DeploymentSummaryInput) -> M5DeploymentSummaryCase {
    M5DeploymentSummaryCase::resolved(input)
}

fn seeded_surface_rows() -> Vec<M5DeploymentSummarySurfaceRow> {
    let base_source_refs = vec![
        M5_DEPLOYMENT_SUMMARY_SCHEMA_REF.to_owned(),
        M5_DEPLOYMENT_SUMMARY_COMPONENT_MATRIX_REF.to_owned(),
    ];
    let all_export_fields = M5DeploymentSummaryExportField::ALL.to_vec();

    vec![
        M5DeploymentSummarySurfaceRow {
            surface_family: M5DeploymentSummarySurfaceFamily::AboutDeploymentCard,
            owner_role: "Deployment-summary guild".to_owned(),
            scope_summary: "About-page deployment summary card naming scope, tenant/region, mirror posture, and last control-plane sync"
                .to_owned(),
            deployment_scopes: vec![
                M5DeploymentScopeClass::SharedManaged,
                M5DeploymentScopeClass::DedicatedManaged,
            ],
            truth_modes: vec![M5DeploymentTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
                M5DeploymentDowngradeTrigger::ResidualVendorDependency,
            ],
            consumer_surfaces: vec!["about_page".to_owned(), "docs_onboarding".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_summaries: vec![case(shared_managed_about_input())],
            overclaims_boundary: false,
            masks_control_plane_as_local: false,
            hides_residual_dependency: false,
            drops_local_safe_step: false,
        },
        M5DeploymentSummarySurfaceRow {
            surface_family: M5DeploymentSummarySurfaceFamily::AdminDeploymentConsole,
            owner_role: "Deployment-admin guild".to_owned(),
            scope_summary: "Admin deployment console keeping a self-hosted boundary honest about its residual vendor dependency"
                .to_owned(),
            deployment_scopes: vec![
                M5DeploymentScopeClass::SelfHosted,
                M5DeploymentScopeClass::DedicatedManaged,
            ],
            truth_modes: vec![M5DeploymentTruthMode::Mirrored],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::ResidualVendorDependency,
                M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
            ],
            consumer_surfaces: vec!["admin_console".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_summaries: vec![case(self_hosted_admin_input())],
            overclaims_boundary: false,
            masks_control_plane_as_local: false,
            hides_residual_dependency: false,
            drops_local_safe_step: false,
        },
        M5DeploymentSummarySurfaceRow {
            surface_family: M5DeploymentSummarySurfaceFamily::ServiceHealthPanel,
            owner_role: "Service-health guild".to_owned(),
            scope_summary: "Service-health panel keeping control-plane and data-plane health distinct with a local-safe next step"
                .to_owned(),
            deployment_scopes: vec![
                M5DeploymentScopeClass::Sovereign,
                M5DeploymentScopeClass::SelfHosted,
            ],
            truth_modes: vec![M5DeploymentTruthMode::CachedOffline],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
                M5DeploymentDowngradeTrigger::OfflineCacheOnly,
            ],
            consumer_surfaces: vec!["service_health".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_summaries: vec![case(sovereign_service_health_input())],
            overclaims_boundary: false,
            masks_control_plane_as_local: false,
            hides_residual_dependency: false,
            drops_local_safe_step: false,
        },
        M5DeploymentSummarySurfaceRow {
            surface_family: M5DeploymentSummarySurfaceFamily::DiagnosticsDeployment,
            owner_role: "Diagnostics guild".to_owned(),
            scope_summary: "Diagnostics deployment pane separating a degraded control plane from a degraded data plane"
                .to_owned(),
            deployment_scopes: vec![M5DeploymentScopeClass::DedicatedManaged],
            truth_modes: vec![M5DeploymentTruthMode::Live],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
                M5DeploymentDowngradeTrigger::ProvenanceIncomplete,
            ],
            consumer_surfaces: vec!["diagnostics".to_owned(), "support_export".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_summaries: vec![case(dedicated_diagnostics_input())],
            overclaims_boundary: false,
            masks_control_plane_as_local: false,
            hides_residual_dependency: false,
            drops_local_safe_step: false,
        },
        M5DeploymentSummarySurfaceRow {
            surface_family: M5DeploymentSummarySurfaceFamily::SupportExportReplay,
            owner_role: "Support / export guild".to_owned(),
            scope_summary: "Offline replay reconstructing deployment scope, planes, and residual dependency from an imported snapshot"
                .to_owned(),
            deployment_scopes: vec![M5DeploymentScopeClass::SharedManaged],
            truth_modes: vec![M5DeploymentTruthMode::Imported],
            export_fields: all_export_fields.clone(),
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::ProvenanceIncomplete,
                M5DeploymentDowngradeTrigger::OfflineCacheOnly,
            ],
            consumer_surfaces: vec!["support_export".to_owned(), "diagnostics".to_owned()],
            source_contract_refs: base_source_refs.clone(),
            example_summaries: vec![case(support_replay_input())],
            overclaims_boundary: false,
            masks_control_plane_as_local: false,
            hides_residual_dependency: false,
            drops_local_safe_step: false,
        },
        M5DeploymentSummarySurfaceRow {
            surface_family: M5DeploymentSummarySurfaceFamily::DocsDeploymentReference,
            owner_role: "Docs / help guild".to_owned(),
            scope_summary: "Docs deployment reference framing a local-only desktop install with no control plane or residual dependency"
                .to_owned(),
            deployment_scopes: vec![M5DeploymentScopeClass::LocalOnly],
            truth_modes: vec![M5DeploymentTruthMode::Live],
            export_fields: all_export_fields,
            downgrade_triggers: vec![
                M5DeploymentDowngradeTrigger::OfflineCacheOnly,
                M5DeploymentDowngradeTrigger::ProvenanceIncomplete,
            ],
            consumer_surfaces: vec!["docs_reference".to_owned(), "about_page".to_owned()],
            source_contract_refs: base_source_refs,
            example_summaries: vec![case(local_only_docs_input())],
            overclaims_boundary: false,
            masks_control_plane_as_local: false,
            hides_residual_dependency: false,
            drops_local_safe_step: false,
        },
    ]
}

fn seeded_governance_review() -> M5DeploymentSummaryGovernanceReview {
    M5DeploymentSummaryGovernanceReview {
        one_primitive_carries_all_surfaces: true,
        deployment_identity_preserved_across_surfaces: true,
        boundary_never_overclaimed: true,
        control_plane_distinct_from_local_runtime: true,
        residual_dependency_always_explicit_and_exportable: true,
        support_export_reconstructs_deployment: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn seeded_consumer_projection() -> M5DeploymentSummaryConsumerProjection {
    M5DeploymentSummaryConsumerProjection {
        deployment_surfaces_consume_shared_primitive: true,
        resolver_reads_single_model: true,
        status_strip_reads_single_plane_source: true,
        support_export_reads_single_source: true,
    }
}

fn seeded_release_posture() -> M5DeploymentSummaryReleasePosture {
    M5DeploymentSummaryReleasePosture {
        release_packet_ref: M5_DEPLOYMENT_SUMMARY_ARTIFACT_REF.to_owned(),
        deployment_audit_ref: M5_DEPLOYMENT_SUMMARY_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

/// Builds the canonical, checked-in M5 deployment-summary primitive packet. This is the
/// one source of truth shared by the tests, the fixture generator, and the on-disk
/// support export so all three stay byte-aligned.
pub fn seeded_m5_deployment_summary_packet() -> M5DeploymentSummaryPrimitivePacket {
    M5DeploymentSummaryPrimitivePacket::new(M5DeploymentSummaryPrimitivePacketInput {
        packet_id: "m5-deployment-summary-primitive:stable:0001".to_owned(),
        matrix_label:
            "M5 Deployment-Summary Primitive: Deployment Summary Card, Residual-Dependency Rows, and Control/Data-Plane Status Strip"
                .to_owned(),
        surface_rows: seeded_surface_rows(),
        vocabulary_set: M5DeploymentSummaryVocabularySet::canonical(),
        governance_review: seeded_governance_review(),
        consumer_projection: seeded_consumer_projection(),
        release_posture: seeded_release_posture(),
        source_contract_refs: vec![
            M5_DEPLOYMENT_SUMMARY_SCHEMA_REF.to_owned(),
            M5_DEPLOYMENT_SUMMARY_DOC_REF.to_owned(),
            M5_DEPLOYMENT_SUMMARY_COMPONENT_MATRIX_REF.to_owned(),
            M5_DEPLOYMENT_SUMMARY_ARTIFACT_REF.to_owned(),
        ],
        redaction_class_token: "install_component_boundary_v1".to_owned(),
        minted_at: "2026-07-04T00:00:00Z".to_owned(),
    })
}
