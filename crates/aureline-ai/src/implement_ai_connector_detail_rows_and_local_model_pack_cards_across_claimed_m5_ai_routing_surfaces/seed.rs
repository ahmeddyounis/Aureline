//! Canonical seed builders for the M5 AI connector-detail-row / local-model-pack-card
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical connector/local-model-primitive packet.
pub const M5_AI_CONNECTOR_MODEL_PRIMITIVE_PACKET_ID: &str =
    "m5-ai-connector-detail-row-local-model-pack-card-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked connector resolution case from a full connector state.
#[allow(clippy::too_many_arguments)]
fn conn_case(
    canonical_id: &str,
    publisher_source: &str,
    execution_locus: M5AiExecutionLocus,
    declared_capabilities: &[M5AiConnectorCapability],
    auth_posture: M5AiAuthPosture,
    policy_blocked: bool,
    reachable: bool,
    session_warmed: bool,
    discloses_side_effects: bool,
) -> M5AiConnectorRowResolutionCase {
    M5AiConnectorRowResolutionCase::resolved(M5AiConnectorRowResolutionInput {
        canonical_id: canonical_id.to_owned(),
        publisher_source: publisher_source.to_owned(),
        execution_locus,
        declared_capabilities: declared_capabilities.to_vec(),
        auth_posture,
        policy_blocked,
        reachable,
        session_warmed,
        discloses_side_effects,
    })
}

/// Builds a worked local-model resolution case from a full pack state.
#[allow(clippy::too_many_arguments)]
fn model_case(
    model_identity: &str,
    digest: &str,
    size_on_disk_mb: u64,
    hardware_expectation_label: &str,
    required_memory_mb: u64,
    available_memory_mb: u64,
    requires_accelerator: bool,
    accelerator_present: bool,
    pack_state: M5AiModelPackState,
    provenance_verified: bool,
    requires_network_fetch: bool,
) -> M5AiModelPackResolutionCase {
    M5AiModelPackResolutionCase::resolved(M5AiModelPackResolutionInput {
        model_identity: model_identity.to_owned(),
        digest: digest.to_owned(),
        size_on_disk_mb,
        hardware_expectation_label: hardware_expectation_label.to_owned(),
        required_memory_mb,
        available_memory_mb,
        requires_accelerator,
        accelerator_present,
        pack_state,
        provenance_verified,
        requires_network_fetch,
    })
}

/// A base row with the shared fields filled in and the full connector / model anatomy,
/// locus, capability, auth, readiness, state, hardware, offline, action, export-field,
/// and accessibility parity every consumer carries.
fn base_row(
    consumer_surface: M5AiConnectorModelConsumerSurface,
    qualification: M5AiQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    connector_examples: Vec<M5AiConnectorRowResolutionCase>,
    model_examples: Vec<M5AiModelPackResolutionCase>,
) -> M5AiConnectorModelRow {
    M5AiConnectorModelRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5AiSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5AiDeploymentLine::ALL.to_vec(),
        connector_anatomy_parts: M5AiConnectorAnatomyPart::ALL.to_vec(),
        model_anatomy_parts: M5AiModelPackAnatomyPart::ALL.to_vec(),
        execution_loci: M5AiExecutionLocus::ALL.to_vec(),
        connector_capabilities: M5AiConnectorCapability::ALL.to_vec(),
        auth_postures: M5AiAuthPosture::ALL.to_vec(),
        connector_readinesses: M5AiConnectorReadiness::ALL.to_vec(),
        model_pack_states: M5AiModelPackState::ALL.to_vec(),
        model_pack_readinesses: M5AiModelPackReadiness::ALL.to_vec(),
        hardware_fits: M5AiModelHardwareFit::ALL.to_vec(),
        offline_postures: M5AiModelOfflinePosture::ALL.to_vec(),
        model_pack_actions: M5AiModelPackAction::ALL.to_vec(),
        connector_export_fields: M5AiConnectorExportField::ALL.to_vec(),
        model_export_fields: M5AiModelPackExportField::ALL.to_vec(),
        accessibility_routes: M5AiAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5AiConsumerSurface::ConnectorAdminConsole,
            M5AiConsumerSurface::ModelManagerUi,
            M5AiConsumerSurface::SupportExport,
            M5AiConsumerSurface::CliInspect,
            M5AiConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5AiExecutionDowngradeTrigger::RouteOrProviderMasked,
            M5AiExecutionDowngradeTrigger::AuthPostureMasked,
            M5AiExecutionDowngradeTrigger::ConnectorSideEffectUndisclosed,
            M5AiExecutionDowngradeTrigger::LocalModelProvenanceMasked,
            M5AiExecutionDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AI_CONNECTOR_MODEL_SCHEMA_REF,
            M5_AI_CONNECTOR_MODEL_GATEWAY_REF,
            M5_AI_CONNECTOR_MODEL_LOCAL_MODEL_REF,
        ]),
        connector_examples,
        model_examples,
        masks_execution_locus_or_authority: false,
        shows_blocked_connector_as_ready: false,
        hides_disk_hardware_or_offline_cost: false,
        invents_parallel_connector_or_model_grammar: false,
    }
}

// Keep the numbered contract cases beside their explanatory comments.
#[allow(clippy::vec_init_then_push)]
fn rows() -> Vec<M5AiConnectorModelRow> {
    use M5AiAuthPosture as Auth;
    use M5AiConnectorCapability as Cap;
    use M5AiExecutionLocus as Locus;
    use M5AiModelPackState as PackState;

    let mut rows = Vec::new();

    // 1. AI settings — a warm in-process read-only connector (no authority needed) and a
    //    cold local-subprocess shell/file connector that needs authority; a freely
    //    selectable local-cached model and a mirror-served model.
    rows.push(base_row(
        M5AiConnectorModelConsumerSurface::AiSettings,
        M5AiQualificationClass::Stable,
        "AI settings owner",
        "The AI settings surface renders the shared connector row and model card so a warm in-process read-only connector reads as invocable without an authority grant, a local-subprocess shell/file connector reads as cold and dependent on a managed credential before invocation, and a local model pack shows its digest, disk cost, hardware fit, and offline posture rather than a bare installed state",
        "evidence:m5-ai-connector-model-settings:001",
        vec![
            conn_case(
                "connector.local-fs-read",
                "aureline first-party",
                Locus::InProcessLocal,
                &[Cap::ReadOnlyQuery],
                Auth::Unauthenticated,
                false,
                true,
                true,
                false,
            ),
            conn_case(
                "connector.shell-runner",
                "aureline first-party",
                Locus::LocalSubprocess,
                &[Cap::ShellExecution, Cap::FileMutation],
                Auth::ManagedCredential,
                false,
                true,
                false,
                true,
            ),
        ],
        vec![
            model_case(
                "model.small-instruct",
                "sha256-aaa111",
                4200,
                "8 GB RAM, AVX2",
                4000,
                16000,
                false,
                false,
                PackState::Installed,
                true,
                false,
            ),
            model_case(
                "model.mirror-pack",
                "sha256-bbb222",
                8100,
                "12 GB RAM",
                8000,
                16000,
                false,
                false,
                PackState::Mirrored,
                true,
                false,
            ),
        ],
    ));

    // 2. Model picker — a warm remote managed connector and a policy-blocked third-party
    //    cloud connector; an offline-only model that fits with swap and an
    //    update-pending model that needs a network fetch.
    rows.push(base_row(
        M5AiConnectorModelConsumerSurface::ModelPicker,
        M5AiQualificationClass::Stable,
        "Model picker owner",
        "The model picker renders the shared connector row and model card so a warm remote managed connector reads as invocable behind an OAuth-delegated grant, a policy-blocked third-party cloud connector reads as blocked rather than ready, and each model pack shows its offline posture and hardware fit — an offline-only pack that fits only with swap and an update-pending pack that still requires a network fetch",
        "evidence:m5-ai-connector-model-picker:001",
        vec![
            conn_case(
                "connector.code-search",
                "aureline managed",
                Locus::RemoteManagedService,
                &[Cap::ReadOnlyQuery, Cap::NetworkEgress],
                Auth::OauthDelegated,
                false,
                true,
                true,
                true,
            ),
            conn_case(
                "connector.cloud-deploy",
                "third-party marketplace",
                Locus::ThirdPartyCloud,
                &[Cap::ExternalServiceCall],
                Auth::ByokScoped,
                true,
                true,
                false,
                true,
            ),
        ],
        vec![
            model_case(
                "model.offline-only",
                "sha256-ccc333",
                15000,
                "16 GB RAM (tight)",
                13000,
                16000,
                false,
                false,
                PackState::OfflineOnly,
                true,
                false,
            ),
            model_case(
                "model.update-ready",
                "sha256-ddd444",
                9000,
                "8 GB RAM",
                6000,
                16000,
                false,
                false,
                PackState::UpdateAvailable,
                true,
                true,
            ),
        ],
    ));

    // 3. Route inspector — an unavailable local-container connector and a warm on-prem
    //    bridge connector; two hardware-blocked models (one over memory, one needing a
    //    missing accelerator).
    rows.push(base_row(
        M5AiConnectorModelConsumerSurface::RouteInspector,
        M5AiQualificationClass::Stable,
        "Route inspector owner",
        "The route inspector renders the shared connector row and model card so an unreachable local-container connector reads as unavailable, a warm on-prem bridge connector reads as invocable behind a token-scoped credential, and a model that exceeds memory or needs a missing accelerator reads as hardware-blocked with run-fit-check and remove actions rather than a bare installed state",
        "evidence:m5-ai-connector-model-route:001",
        vec![
            conn_case(
                "connector.container-run",
                "aureline first-party",
                Locus::LocalContainer,
                &[Cap::ShellExecution],
                Auth::ServiceAccount,
                false,
                false,
                false,
                true,
            ),
            conn_case(
                "connector.onprem-db",
                "customer on-prem",
                Locus::OnPremBridge,
                &[Cap::CredentialScoped, Cap::ReadOnlyQuery],
                Auth::TokenScoped,
                false,
                true,
                true,
                true,
            ),
        ],
        vec![
            model_case(
                "model.too-big",
                "sha256-eee555",
                40000,
                "32 GB RAM",
                32000,
                16000,
                false,
                false,
                PackState::Installed,
                true,
                false,
            ),
            model_case(
                "model.needs-gpu",
                "sha256-fff666",
                22000,
                "GPU with 24 GB VRAM",
                8000,
                16000,
                true,
                false,
                PackState::Installed,
                true,
                false,
            ),
        ],
    ));

    // 4. Evidence view — a cold in-process read-only connector and a warm third-party
    //    egress connector; two verification-held models (one quarantined, one with
    //    unverified provenance).
    rows.push(base_row(
        M5AiConnectorModelConsumerSurface::EvidenceView,
        M5AiQualificationClass::Stable,
        "Evidence view owner",
        "The evidence view renders the shared connector row and model card so a cold in-process read-only connector reads as invocable without authority, a warm third-party egress connector reads as invocable behind a token-scoped credential, and a quarantined or provenance-unverified model reads as verification-held with verify and remove actions rather than as installed",
        "evidence:m5-ai-connector-model-evidence:001",
        vec![
            conn_case(
                "connector.readonly-metrics",
                "aureline first-party",
                Locus::InProcessLocal,
                &[Cap::ReadOnlyQuery],
                Auth::Unauthenticated,
                false,
                true,
                false,
                false,
            ),
            conn_case(
                "connector.egress-fetch",
                "third-party marketplace",
                Locus::ThirdPartyCloud,
                &[Cap::NetworkEgress],
                Auth::TokenScoped,
                false,
                true,
                true,
                true,
            ),
        ],
        vec![
            model_case(
                "model.quarantined",
                "sha256-ggg777",
                7000,
                "8 GB RAM",
                4000,
                16000,
                false,
                false,
                PackState::Quarantined,
                true,
                false,
            ),
            model_case(
                "model.unverified-prov",
                "sha256-hhh888",
                6500,
                "8 GB RAM",
                4000,
                16000,
                false,
                false,
                PackState::ProvenanceUnverified,
                false,
                false,
            ),
        ],
    ));

    // 5. CLI / support export — an unavailable local-subprocess file connector and a
    //    policy-blocked remote external-service connector; a hardware-unfit-state model
    //    and a freely selectable installed model that requires a network fetch — the
    //    same connector/model vocabulary a support or CLI reviewer reads elsewhere.
    rows.push(base_row(
        M5AiConnectorModelConsumerSurface::CliSupportExport,
        M5AiQualificationClass::Stable,
        "CLI / support export owner",
        "The CLI / support export renders the shared connector row and model card so an unreachable local-subprocess file connector reads as unavailable, a policy-blocked remote external-service connector reads as blocked, and each model pack's digest, disk cost, hardware fit, and offline posture are reconstructable from the support export alone",
        "evidence:m5-ai-connector-model-cli:001",
        vec![
            conn_case(
                "connector.file-writer",
                "aureline first-party",
                Locus::LocalSubprocess,
                &[Cap::FileMutation],
                Auth::ServiceAccount,
                false,
                false,
                false,
                true,
            ),
            conn_case(
                "connector.external-api",
                "aureline managed",
                Locus::RemoteManagedService,
                &[Cap::ExternalServiceCall, Cap::NetworkEgress],
                Auth::OauthDelegated,
                true,
                true,
                false,
                true,
            ),
        ],
        vec![
            model_case(
                "model.hw-unfit-state",
                "sha256-iii999",
                30000,
                "24 GB RAM",
                8000,
                16000,
                false,
                false,
                PackState::HardwareUnfit,
                true,
                false,
            ),
            model_case(
                "model.installed-cached",
                "sha256-jjj000",
                5000,
                "8 GB RAM",
                5000,
                16000,
                false,
                false,
                PackState::Installed,
                true,
                true,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5AiConnectorModelGovernanceReview {
    M5AiConnectorModelGovernanceReview {
        one_primitive_carries_connector_and_model_truth: true,
        execution_locus_and_authority_always_shown: true,
        connector_readiness_never_masks_blocked: true,
        side_effecting_capability_always_disclosed: true,
        disk_hardware_and_offline_always_shown: true,
        model_state_never_generic_installed: true,
        bounded_actions_reflect_readiness: true,
        support_export_reconstructs_row_and_card_truth: true,
        no_surface_invents_parallel_vocabulary: true,
        every_row_declares_accessibility_route: true,
        descriptors_stable_across_ui_export_support: true,
    }
}

fn consumer_projection() -> M5AiConnectorModelConsumerProjection {
    M5AiConnectorModelConsumerProjection {
        routing_surfaces_consume_shared_primitive: true,
        connector_readiness_reads_single_source: true,
        model_readiness_reads_single_source: true,
        offline_posture_reads_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5AiConnectorModelProofFreshness {
    M5AiConnectorModelProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AiConnectorModelReleasePosture {
    M5AiConnectorModelReleasePosture {
        release_packet_ref: M5_AI_CONNECTOR_MODEL_ARTIFACT_REF.to_owned(),
        ai_audit_ref: M5_AI_CONNECTOR_MODEL_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AI_CONNECTOR_MODEL_SCHEMA_REF,
        M5_AI_CONNECTOR_MODEL_DOC_REF,
        M5_AI_CONNECTOR_MODEL_COMPONENT_MATRIX_REF,
        M5_AI_CONNECTOR_MODEL_GATEWAY_REF,
        M5_AI_CONNECTOR_MODEL_LOCAL_MODEL_REF,
    ])
}

/// Builds the canonical M5 AI connector/local-model-primitive packet.
pub fn seeded_m5_ai_connector_model_primitive_packet() -> M5AiConnectorModelPrimitivePacket {
    M5AiConnectorModelPrimitivePacket::new(M5AiConnectorModelPrimitivePacketInput {
        packet_id: M5_AI_CONNECTOR_MODEL_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 AI connector detail row and local model pack card primitive: canonical id, publisher, execution locus, capabilities, auth posture, warm/cold/unavailable/policy-blocked readiness, model identity, digest, disk cost, hardware fit, offline posture, and bounded select/verify/remove actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5AiConnectorModelVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the route inspector is narrowed to Preview pending connector
/// readiness parity proof across every headless export path; every consumer stays
/// visible.
pub fn seeded_m5_ai_connector_model_primitive_route_inspector_preview_narrowed(
) -> M5AiConnectorModelPrimitivePacket {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet.packet_id =
        "m5-ai-connector-detail-row-local-model-pack-card-primitive:route-inspector-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5AiConnectorModelConsumerSurface::RouteInspector)
        .expect("route-inspector row present");
    row.qualification = M5AiQualificationClass::Preview;
    packet
}

/// Narrowed variant: the evidence view is held at Beta because a slice of evidence-view
/// cards do not yet render the offline-posture cue on every profile; every consumer
/// stays visible.
pub fn seeded_m5_ai_connector_model_primitive_evidence_view_beta_narrowed(
) -> M5AiConnectorModelPrimitivePacket {
    let mut packet = seeded_m5_ai_connector_model_primitive_packet();
    packet.packet_id =
        "m5-ai-connector-detail-row-local-model-pack-card-primitive:evidence-view-beta:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5AiConnectorModelConsumerSurface::EvidenceView)
        .expect("evidence-view row present");
    row.qualification = M5AiQualificationClass::Beta;
    packet
}
