//! Canonical seed builders for the M5 remote-target / environment primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so
//! the in-code matrix, the artifact, the worked resolutions, and the fixtures never
//! drift.

use super::*;

/// Stable packet id for the canonical remote-target / environment primitive packet.
pub const M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_PACKET_ID: &str =
    "m5-remote-target-environment-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked resolution case from a full run-context state.
#[allow(clippy::too_many_arguments)]
fn case(
    context_title: &str,
    host_boundary: M5HostBoundaryClass,
    connection_state: Option<M5RemoteConnectionState>,
    runtime_kind_repr: &str,
    resolved_runtime_repr: &str,
    runtime_source: M5RuntimeSourceClass,
    scope: M5ResolvedScope,
    effective_value_provenance: M5EffectiveValueProvenance,
) -> M5RunContextResolutionCase {
    M5RunContextResolutionCase::resolved(M5RunContextResolutionInput {
        context_title: context_title.to_owned(),
        host_boundary,
        connection_state,
        runtime_kind_repr: runtime_kind_repr.to_owned(),
        resolved_runtime_repr: resolved_runtime_repr.to_owned(),
        runtime_source,
        scope,
        effective_value_provenance,
    })
}

/// A base row with the shared fields filled in and the full pill-part, strip-part,
/// target-posture, readiness, provenance, scope, export-field, and accessibility
/// parity every surface carries.
fn base_row(
    run_surface: M5RunCapableSurface,
    qualification: M5RuntimeBoundaryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    shell_zone_slot: M5ShellZoneSlot,
    proof_ref: &str,
    example_resolutions: Vec<M5RunContextResolutionCase>,
) -> M5RunCapableSurfaceRow {
    M5RunCapableSurfaceRow {
        run_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        shell_zone_slot,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        pill_parts: M5RemoteTargetPillPart::ALL.to_vec(),
        strip_parts: M5EnvironmentStripPart::ALL.to_vec(),
        target_postures: M5RemoteTargetPosture::ALL.to_vec(),
        readiness_states: M5EnvironmentReadiness::ALL.to_vec(),
        provenance_states: M5EffectiveValueProvenance::ALL.to_vec(),
        resolved_scopes: M5ResolvedScope::ALL.to_vec(),
        export_fields: M5RunContextExportField::ALL.to_vec(),
        accessibility_routes: M5RuntimeBoundaryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::Layout,
            M5ShellConsumerSurface::DocsHelp,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5RuntimeBoundaryDowngradeTrigger::HostBoundaryMasked,
            M5RuntimeBoundaryDowngradeTrigger::ConnectionStateStale,
            M5RuntimeBoundaryDowngradeTrigger::RuntimeSourceUnexplained,
            M5RuntimeBoundaryDowngradeTrigger::AuditTruthLostOffPrimarySurface,
            M5RuntimeBoundaryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_REMOTE_TARGET_SCHEMA_REF,
            M5_ENVIRONMENT_STRIP_SCHEMA_REF,
            M5_REMOTE_TARGET_ENVIRONMENT_TARGET_CONTEXT_REF,
        ]),
        example_resolutions,
        masks_host_or_environment_boundary: false,
        conflates_ready_and_degraded_or_blocked: false,
        invents_private_status_grammar: false,
        hides_why_this_context_entrypoint: false,
    }
}

fn surface_rows() -> Vec<M5RunCapableSurfaceRow> {
    use M5EffectiveValueProvenance as Prov;
    use M5HostBoundaryClass as Host;
    use M5RemoteConnectionState as Conn;
    use M5ResolvedScope as Scope;
    use M5RuntimeSourceClass as Source;

    let mut rows = Vec::new();

    // 1. Run console — a live local resolved runtime (LocalInline / Ready), and a
    //    connected remote whose value is offline-cached (DegradedCached).
    rows.push(base_row(
        M5RunCapableSurface::RunConsole,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Run console owner",
        "The run console renders the shared remote-target pill and environment status strip so a project-pinned local runtime reads as local-inline and ready with its winning source, while a connected remote serving an offline-cached value reads as degraded-cached rather than cleanly ready",
        M5ShellZoneSlot::StatusBar,
        "evidence:m5-remote-target-run:001",
        vec![
            case(
                "run-local",
                Host::LocalHost,
                None,
                "node",
                "node 20.11.0",
                Source::ProjectPinned,
                Scope::ProjectScope,
                Prov::Resolved,
            ),
            case(
                "run-remote",
                Host::RemoteSshHost,
                Some(Conn::Connected),
                "node",
                "node 18.19.0",
                Source::WorkspaceConfigured,
                Scope::WorkspaceScope,
                Prov::CachedOffline,
            ),
        ],
    ));

    // 2. Test runner — a container establishing its connection (Establishing / Ready),
    //    and a reconnecting remote whose target is unreachable (DegradedUnreachable).
    rows.push(base_row(
        M5RunCapableSurface::TestRunner,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Test runner owner",
        "The test runner renders the shared pill and strip so a container-provided runtime whose connection is establishing reads as establishing and ready once resolved, while a reconnecting remote reads as degraded-unreachable-target rather than confirming readiness against a target that is not reachable",
        M5ShellZoneSlot::StatusBar,
        "evidence:m5-remote-target-test:001",
        vec![
            case(
                "test-container",
                Host::ContainerHost,
                Some(Conn::Connecting),
                "python",
                "python 3.12.2",
                Source::ContainerProvided,
                Scope::HostScope,
                Prov::Resolved,
            ),
            case(
                "test-remote",
                Host::RemoteSshHost,
                Some(Conn::Reconnecting),
                "python",
                "python 3.11.7",
                Source::ToolManagerResolved,
                Scope::SessionScope,
                Prov::Resolved,
            ),
        ],
    ));

    // 3. Debug session — an offline-cached managed host with a narrowed value
    //    (DegradedNarrowed), and a disconnected VM blocked by policy (BlockedByPolicy).
    rows.push(base_row(
        M5RunCapableSurface::DebugSession,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Debug session owner",
        "The debug session renders the shared pill and strip so a managed workspace host serving a narrowed/approximate value over an offline cache reads as offline-cached and degraded-narrowed, while a disconnected VM whose runtime is blocked by policy reads as disconnected and blocked-by-policy before work starts",
        M5ShellZoneSlot::StatusBar,
        "evidence:m5-remote-target-debug:001",
        vec![
            case(
                "debug-managed",
                Host::ManagedWorkspaceHost,
                Some(Conn::OfflineCached),
                "node",
                "node 20.x (approx)",
                Source::SessionOverride,
                Scope::GlobalDefaultScope,
                Prov::NarrowedApproximate,
            ),
            case(
                "debug-vm",
                Host::VirtualMachineHost,
                Some(Conn::Disconnected),
                "dotnet",
                "dotnet 8.0.3",
                Source::SystemDefault,
                Scope::WorkspaceScope,
                Prov::PolicyBlocked,
            ),
        ],
    ));

    // 4. Notebook runtime — a wasm-sandbox whose value could not resolve
    //    (BlockedUnresolved), and a local resolved default (LocalInline / Ready).
    rows.push(base_row(
        M5RunCapableSurface::NotebookRuntime,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Notebook runtime owner",
        "The notebook runtime renders the shared pill and strip so a wasm-sandbox kernel whose runtime value could not be resolved reads as blocked-unresolved rather than a stale value, while a local system-default kernel reads as local-inline and ready with its winning source and scope explicit",
        M5ShellZoneSlot::StatusBar,
        "evidence:m5-remote-target-notebook:001",
        vec![
            case(
                "kernel-wasm",
                Host::WasmSandboxHost,
                Some(Conn::Connected),
                "python",
                "python (kernelspec)",
                Source::ToolManagerResolved,
                Scope::SessionScope,
                Prov::Unresolved,
            ),
            case(
                "kernel-local",
                Host::LocalHost,
                None,
                "python",
                "python 3.12.2",
                Source::SystemDefault,
                Scope::GlobalDefaultScope,
                Prov::Resolved,
            ),
        ],
    ));

    // 5. Request runner — a connected remote resolved (ConnectedHealthy / Ready), and
    //    an offline-cached managed host (OfflineCached / DegradedCached).
    rows.push(base_row(
        M5RunCapableSurface::RequestRunner,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Request runner owner",
        "The request runner renders the shared pill and strip so a connected remote environment reads as connected-healthy and ready, while a managed workspace host serving an offline-cached value reads as offline-cached and degraded-cached with the same source/scope/readiness vocabulary as every other surface",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-remote-target-request:001",
        vec![
            case(
                "request-remote",
                Host::RemoteSshHost,
                Some(Conn::Connected),
                "node",
                "node 20.11.0",
                Source::WorkspaceConfigured,
                Scope::WorkspaceScope,
                Prov::Resolved,
            ),
            case(
                "request-managed",
                Host::ManagedWorkspaceHost,
                Some(Conn::OfflineCached),
                "node",
                "node 18.19.0",
                Source::ContainerProvided,
                Scope::HostScope,
                Prov::CachedOffline,
            ),
        ],
    ));

    // 6. Database session — a connected remote blocked by policy (BlockedByPolicy),
    //    and a local resolved project pin (LocalInline / Ready).
    rows.push(base_row(
        M5RunCapableSurface::DatabaseSession,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Database session owner",
        "The database session renders the shared pill and strip so a connected remote database whose runtime access is blocked by policy reads as connected-healthy on the pill but blocked-by-policy on the strip, while a local project-pinned client reads as local-inline and ready",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-remote-target-database:001",
        vec![
            case(
                "db-remote",
                Host::RemoteSshHost,
                Some(Conn::Connected),
                "postgres-client",
                "psql 16.2",
                Source::SystemDefault,
                Scope::HostScope,
                Prov::PolicyBlocked,
            ),
            case(
                "db-local",
                Host::LocalHost,
                None,
                "postgres-client",
                "psql 16.2",
                Source::ProjectPinned,
                Scope::ProjectScope,
                Prov::Resolved,
            ),
        ],
    ));

    // 7. Preview server — a connected container resolved (ConnectedHealthy / Ready),
    //    and a reconnecting VM (Reconnecting / DegradedUnreachableTarget).
    rows.push(base_row(
        M5RunCapableSurface::PreviewServer,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Preview server owner",
        "The preview server renders the shared pill and strip so a connected container-provided runtime reads as connected-healthy and ready, while a reconnecting VM reads as reconnecting on the pill and degraded-unreachable-target on the strip until the target is reachable again",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-remote-target-preview:001",
        vec![
            case(
                "preview-container",
                Host::ContainerHost,
                Some(Conn::Connected),
                "node",
                "node 20.11.0",
                Source::ContainerProvided,
                Scope::WorkspaceScope,
                Prov::Resolved,
            ),
            case(
                "preview-vm",
                Host::VirtualMachineHost,
                Some(Conn::Reconnecting),
                "node",
                "node 20.11.0",
                Source::WorkspaceConfigured,
                Scope::WorkspaceScope,
                Prov::Resolved,
            ),
        ],
    ));

    // 8. Pipeline run — a connected managed host with a narrowed value
    //    (DegradedNarrowed), and a disconnected remote unresolved (BlockedUnresolved).
    rows.push(base_row(
        M5RunCapableSurface::PipelineRun,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Pipeline run owner",
        "The pipeline run renders the shared pill and strip so a connected managed host resolving a narrowed value reads as connected-healthy and degraded-narrowed, while a disconnected remote whose value could not be resolved reads as disconnected and blocked-unresolved rather than presenting an unproven value as ready",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-remote-target-pipeline:001",
        vec![
            case(
                "pipeline-managed",
                Host::ManagedWorkspaceHost,
                Some(Conn::Connected),
                "rust",
                "rustc 1.79.0 (approx)",
                Source::ToolManagerResolved,
                Scope::ProjectScope,
                Prov::NarrowedApproximate,
            ),
            case(
                "pipeline-remote",
                Host::RemoteSshHost,
                Some(Conn::Disconnected),
                "rust",
                "rustc (unresolved)",
                Source::SystemDefault,
                Scope::HostScope,
                Prov::Unresolved,
            ),
        ],
    ));

    // 9. Incident surface — a presenter-driven connected remote (ConnectedHealthy /
    //    Ready), and a container serving an offline cache (OfflineCached / Cached).
    rows.push(base_row(
        M5RunCapableSurface::IncidentSurface,
        M5RuntimeBoundaryQualificationClass::Stable,
        "Incident surface owner",
        "The incident/break-glass surface renders the shared pill and strip so a connected remote session override reads as connected-healthy and ready with the winning source and scope explicit before any keystroke, while a container serving an offline cache reads as offline-cached and degraded-cached",
        M5ShellZoneSlot::MainWorkspace,
        "evidence:m5-remote-target-incident:001",
        vec![
            case(
                "incident-remote",
                Host::RemoteSshHost,
                Some(Conn::Connected),
                "bash",
                "bash 5.2.21",
                Source::SessionOverride,
                Scope::SessionScope,
                Prov::Resolved,
            ),
            case(
                "incident-container",
                Host::ContainerHost,
                Some(Conn::OfflineCached),
                "bash",
                "bash 5.2.15",
                Source::ContainerProvided,
                Scope::HostScope,
                Prov::CachedOffline,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5RemoteTargetEnvironmentGovernanceReview {
    M5RemoteTargetEnvironmentGovernanceReview {
        one_primitive_carries_target_and_environment: true,
        target_identity_and_host_boundary_always_shown: true,
        winning_source_and_scope_always_explicit: true,
        readiness_state_always_resolved: true,
        cached_narrowed_or_blocked_never_shown_as_ready: true,
        why_this_context_entrypoint_always_present: true,
        support_export_reconstructs_source_scope_readiness: true,
        no_surface_invents_second_status_grammar: true,
        every_row_bound_to_shell_zone: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5RemoteTargetEnvironmentConsumerProjection {
    M5RemoteTargetEnvironmentConsumerProjection {
        run_capable_surfaces_consume_shared_primitive: true,
        readiness_resolver_reads_single_source: true,
        winning_source_reads_single_resolution_source: true,
        target_pill_reads_single_connection_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5RemoteTargetEnvironmentProofFreshness {
    M5RemoteTargetEnvironmentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RemoteTargetEnvironmentReleasePosture {
    M5RemoteTargetEnvironmentReleasePosture {
        release_packet_ref: M5_REMOTE_TARGET_ENVIRONMENT_ARTIFACT_REF.to_owned(),
        environment_audit_ref: M5_REMOTE_TARGET_ENVIRONMENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_REMOTE_TARGET_SCHEMA_REF,
        M5_ENVIRONMENT_STRIP_SCHEMA_REF,
        M5_REMOTE_TARGET_ENVIRONMENT_DOC_REF,
        M5_REMOTE_TARGET_ENVIRONMENT_SHELL_ZONE_REF,
        M5_REMOTE_TARGET_ENVIRONMENT_COMPONENT_MATRIX_REF,
        M5_REMOTE_TARGET_ENVIRONMENT_EXECUTION_CONTEXT_REF,
        M5_REMOTE_TARGET_ENVIRONMENT_TARGET_CONTEXT_REF,
    ])
}

/// Builds the canonical M5 remote-target / environment primitive packet.
pub fn seeded_m5_remote_target_environment_primitive_packet(
) -> M5RemoteTargetEnvironmentPrimitivePacket {
    M5RemoteTargetEnvironmentPrimitivePacket::new(M5RemoteTargetEnvironmentPrimitivePacketInput {
        packet_id: M5_REMOTE_TARGET_ENVIRONMENT_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 remote-target pill and environment status strip primitive: target identity, host boundary, degraded/reconnect state, resolved runtime source, scope, readiness, and 'Why this context?' entrypoint"
                .to_owned(),
        surface_rows: surface_rows(),
        vocabulary_set: M5RemoteTargetEnvironmentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the incident surface is held at Beta because a slice of
/// break-glass sessions do not yet render the degraded-cache cue on every profile;
/// every surface stays visible.
pub fn seeded_m5_remote_target_environment_primitive_incident_surface_beta_narrowed(
) -> M5RemoteTargetEnvironmentPrimitivePacket {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.packet_id =
        "m5-remote-target-environment-primitive:incident-surface-beta:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.run_surface == M5RunCapableSurface::IncidentSurface)
        .expect("incident surface row present");
    row.qualification = M5RuntimeBoundaryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the pipeline run is narrowed to Preview pending unresolved-value
/// readiness parity proof across every export path; every surface stays visible.
pub fn seeded_m5_remote_target_environment_primitive_pipeline_run_preview_narrowed(
) -> M5RemoteTargetEnvironmentPrimitivePacket {
    let mut packet = seeded_m5_remote_target_environment_primitive_packet();
    packet.packet_id =
        "m5-remote-target-environment-primitive:pipeline-run-preview:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.run_surface == M5RunCapableSurface::PipelineRun)
        .expect("pipeline run row present");
    row.qualification = M5RuntimeBoundaryQualificationClass::Preview;
    packet
}
