//! Canonical seed builders for the M5 host-boundary-strip / execution-origin-receipt-row controls
//! packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code controls, the
//! artifact, and the fixtures never drift. Every resolved example is built by calling the real
//! resolvers so the packet can only carry projections the resolvers actually produce.

use super::*;

use aureline_execution::m5_host_boundary::{
    ConnectionState, HostKind, HostNarrowingReason, OriginReceiptState,
};

/// Stable packet id for the canonical controls packet.
pub const M5_HOST_ORIGIN_CONTROLS_PACKET_ID: &str =
    "m5-host-boundary-strip-execution-origin-receipt-row-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-11T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn strip(input: M5HostBoundaryStripResolutionInput) -> M5ResolvedHostBoundaryStrip {
    resolve_host_boundary_strip(input).expect("seed host-boundary strip input resolves")
}

fn receipt(
    input: M5ExecutionOriginReceiptRowResolutionInput,
) -> M5ResolvedExecutionOriginReceiptRow {
    resolve_execution_origin_receipt_row(input)
        .expect("seed execution-origin receipt input resolves")
}

// -- Canonical host-boundary strip examples -----------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn strip_input(
    strip_id: &str,
    host_kind: HostKind,
    is_devcontainer: bool,
    locality_disclosed: bool,
    target_label: &str,
    target_label_disclosed: bool,
    owning_runtime_lane: &str,
    owning_lane_disclosed: bool,
    connection_state: ConnectionState,
    reconnect_state_disclosed: bool,
) -> M5HostBoundaryStripResolutionInput {
    M5HostBoundaryStripResolutionInput {
        strip_id: strip_id.to_owned(),
        host_kind,
        is_devcontainer,
        locality_disclosed,
        target_label: target_label.to_owned(),
        target_label_disclosed,
        owning_runtime_lane: owning_runtime_lane.to_owned(),
        owning_lane_disclosed,
        connection_state,
        reconnect_state_disclosed,
        open_details_available: true,
        proof_fresh: true,
    }
}

/// Clean strip: work runs on the local desktop host.
fn strip_local() -> M5ResolvedHostBoundaryStrip {
    strip(strip_input(
        "host-strip:local",
        HostKind::Local,
        false,
        true,
        "web-frontend@local",
        true,
        "local desktop runtime",
        true,
        ConnectionState::Connected,
        true,
    ))
}

/// Clean strip: work runs on a remote SSH host.
fn strip_ssh() -> M5ResolvedHostBoundaryStrip {
    strip(strip_input(
        "host-strip:ssh",
        HostKind::Ssh,
        false,
        true,
        "api-server@build-host",
        true,
        "remote ssh runtime",
        true,
        ConnectionState::Connected,
        true,
    ))
}

/// Clean strip: work runs in a container runtime.
fn strip_container() -> M5ResolvedHostBoundaryStrip {
    strip(strip_input(
        "host-strip:container",
        HostKind::Container,
        false,
        true,
        "worker@container-a",
        true,
        "container runtime lane",
        true,
        ConnectionState::Connected,
        true,
    ))
}

/// Clean strip: work runs in a development container.
fn strip_devcontainer() -> M5ResolvedHostBoundaryStrip {
    strip(strip_input(
        "host-strip:devcontainer",
        HostKind::Container,
        true,
        true,
        "web-frontend@devcontainer",
        true,
        "devcontainer runtime lane",
        true,
        ConnectionState::Connected,
        true,
    ))
}

/// Clean strip: work runs on a managed cloud workspace.
fn strip_managed() -> M5ResolvedHostBoundaryStrip {
    strip(strip_input(
        "host-strip:managed",
        HostKind::ManagedWorkspace,
        false,
        true,
        "web-frontend@managed-ws",
        true,
        "managed workspace lane",
        true,
        ConnectionState::Connected,
        true,
    ))
}

/// Clean strip: work runs across a browser / companion bridge; the bridged state is disclosed so the
/// strip stays clean while it crosses the boundary.
fn strip_browser_bridge() -> M5ResolvedHostBoundaryStrip {
    strip(strip_input(
        "host-strip:browser-bridge",
        HostKind::BrowserBridge,
        false,
        true,
        "notebook@companion-bridge",
        true,
        "browser bridge lane",
        true,
        ConnectionState::Bridged,
        true,
    ))
}

/// Clean strip: work runs on a connector-backed service plane.
fn strip_service_plane() -> M5ResolvedHostBoundaryStrip {
    strip(strip_input(
        "host-strip:service-plane",
        HostKind::ServicePlane,
        false,
        true,
        "incident-runner@service-plane",
        true,
        "service plane lane",
        true,
        ConnectionState::Connected,
        true,
    ))
}

/// Degraded strip: the locality class is undisclosed — proves AC1's locality half.
fn strip_locality_unstated() -> M5ResolvedHostBoundaryStrip {
    strip(strip_input(
        "host-strip:locality-hidden",
        HostKind::Local,
        false,
        false,
        "web-frontend@local",
        true,
        "local desktop runtime",
        true,
        ConnectionState::Connected,
        true,
    ))
}

/// Degraded strip: the owning runtime / service lane is undisclosed — proves the owning-lane half.
fn strip_owning_lane_unstated() -> M5ResolvedHostBoundaryStrip {
    strip(strip_input(
        "host-strip:owning-lane-hidden",
        HostKind::Container,
        false,
        true,
        "worker@container-b",
        true,
        "container runtime lane",
        false,
        ConnectionState::Connected,
        true,
    ))
}

/// Degraded strip: the connection is reconnecting but the reconnect / degraded state is undisclosed
/// — proves host ownership must not disappear on degrade.
fn strip_reconnect_degraded_unstated() -> M5ResolvedHostBoundaryStrip {
    strip(strip_input(
        "host-strip:reconnect-hidden",
        HostKind::ManagedWorkspace,
        false,
        true,
        "web-frontend@managed-ws",
        true,
        "managed workspace lane",
        true,
        ConnectionState::Reconnecting,
        false,
    ))
}

// -- Canonical execution-origin receipt row examples --------------------------------------------

#[allow(clippy::too_many_arguments)]
fn receipt_input(
    receipt_id: &str,
    action_class: &str,
    action_class_disclosed: bool,
    resolved_target_identity: &str,
    target_identity_disclosed: bool,
    host_kind: HostKind,
    receipt_state: OriginReceiptState,
    connection_state: ConnectionState,
    provenance_disclosed: bool,
    export_safe_lineage_present: bool,
    restored_or_handed_off: bool,
    ownership_retained: bool,
    host_narrowing_reason: Option<HostNarrowingReason>,
) -> M5ExecutionOriginReceiptRowResolutionInput {
    M5ExecutionOriginReceiptRowResolutionInput {
        receipt_id: receipt_id.to_owned(),
        action_class: action_class.to_owned(),
        action_class_disclosed,
        resolved_target_identity: resolved_target_identity.to_owned(),
        target_identity_disclosed,
        host_kind,
        receipt_state,
        connection_state,
        provenance_disclosed,
        export_safe_lineage_present,
        restored_or_handed_off,
        ownership_retained,
        host_narrowing_reason,
        proof_fresh: true,
    }
}

/// Clean receipt: a signed, local, receipt-backed run with an export-safe lineage.
fn receipt_local_signed() -> M5ResolvedExecutionOriginReceiptRow {
    receipt(receipt_input(
        "origin-receipt:local-signed",
        "run_tests",
        true,
        "web-frontend@local",
        true,
        HostKind::Local,
        OriginReceiptState::Signed,
        ConnectionState::Connected,
        true,
        true,
        false,
        true,
        None,
    ))
}

/// Clean receipt: a bridged run that attributes its bridged boundary and keeps an export-safe
/// lineage.
fn receipt_bridged_attributed() -> M5ResolvedExecutionOriginReceiptRow {
    receipt(receipt_input(
        "origin-receipt:bridged-attributed",
        "preview_render",
        true,
        "notebook@companion-bridge",
        true,
        HostKind::BrowserBridge,
        OriginReceiptState::Recorded,
        ConnectionState::Bridged,
        true,
        true,
        false,
        true,
        Some(HostNarrowingReason::BridgedBoundary),
    ))
}

/// Clean receipt: after a restore / handoff the execution origin is carried through — host ownership
/// is retained — so the receipt stays clean.
fn receipt_restored_retains_ownership() -> M5ResolvedExecutionOriginReceiptRow {
    receipt(receipt_input(
        "origin-receipt:restored-retained",
        "debug_attach",
        true,
        "api-server@build-host",
        true,
        HostKind::Ssh,
        OriginReceiptState::Signed,
        ConnectionState::Reconnecting,
        true,
        true,
        true,
        true,
        Some(HostNarrowingReason::ReconnectingHost),
    ))
}

/// Degraded receipt: the lineage is not export-safe / reusable — proves AC2.
fn receipt_lineage_not_export_safe() -> M5ResolvedExecutionOriginReceiptRow {
    receipt(receipt_input(
        "origin-receipt:lineage-unsafe",
        "run_tests",
        true,
        "worker@container-a",
        true,
        HostKind::Local,
        OriginReceiptState::Signed,
        ConnectionState::Connected,
        true,
        false,
        false,
        true,
        None,
    ))
}

/// Degraded receipt: a restore / handoff dropped the execution origin — proves the host-ownership
/// guardrail.
fn receipt_ownership_dropped_on_restore() -> M5ResolvedExecutionOriginReceiptRow {
    receipt(receipt_input(
        "origin-receipt:ownership-dropped",
        "resume_workspace",
        true,
        "web-frontend@managed-ws",
        true,
        HostKind::ManagedWorkspace,
        OriginReceiptState::Inferred,
        ConnectionState::Reconnecting,
        true,
        true,
        true,
        false,
        Some(HostNarrowingReason::ReconnectingHost),
    ))
}

/// Degraded receipt: the execution-context provenance is undisclosed.
fn receipt_provenance_unstated() -> M5ResolvedExecutionOriginReceiptRow {
    receipt(receipt_input(
        "origin-receipt:provenance-hidden",
        "run_tests",
        true,
        "worker@container-b",
        true,
        HostKind::Container,
        OriginReceiptState::Recorded,
        ConnectionState::Connected,
        false,
        true,
        false,
        true,
        None,
    ))
}

/// Degraded receipt: the action class is undisclosed.
fn receipt_action_class_unstated() -> M5ResolvedExecutionOriginReceiptRow {
    receipt(receipt_input(
        "origin-receipt:action-hidden",
        "run_tests",
        false,
        "web-frontend@local",
        true,
        HostKind::Local,
        OriginReceiptState::Missing,
        ConnectionState::Connected,
        true,
        true,
        false,
        true,
        Some(HostNarrowingReason::MissingOriginReceipt),
    ))
}

/// Degraded receipt: the resolved target identity is undisclosed.
fn receipt_target_identity_unstated() -> M5ResolvedExecutionOriginReceiptRow {
    receipt(receipt_input(
        "origin-receipt:identity-hidden",
        "run_tests",
        true,
        "incident-runner@service-plane",
        false,
        HostKind::ServicePlane,
        OriginReceiptState::Inferred,
        ConnectionState::Connected,
        true,
        true,
        false,
        true,
        None,
    ))
}

// -- Row builders ------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5HostOriginConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5BuildRemoteDowngradeTrigger>,
    host_boundary_strip_examples: Vec<M5ResolvedHostBoundaryStrip>,
    execution_origin_receipt_row_examples: Vec<M5ResolvedExecutionOriginReceiptRow>,
) -> M5HostOriginControlsRow {
    M5HostOriginControlsRow {
        consumer_surface,
        qualification: M5BuildRemoteQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5BuildRemoteDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5BuildRemoteRequiredLabel::Identity,
            M5BuildRemoteRequiredLabel::State,
            M5BuildRemoteRequiredLabel::KeyboardRoute,
            M5BuildRemoteRequiredLabel::HostAndExecutionOrigin,
        ],
        accessibility_routes: M5BuildRemoteAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5HostOriginAnatomyPart::ALL.to_vec(),
        export_fields: M5HostOriginExportField::ALL.to_vec(),
        downgrade_triggers,
        host_boundary_strip_examples,
        execution_origin_receipt_row_examples,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_HOST_ORIGIN_CONTROLS_SCHEMA_REF,
            M5_HOST_BOUNDARY_STRIP_SCHEMA_REF,
            M5_EXECUTION_ORIGIN_RECEIPT_ROW_SCHEMA_REF,
        ]),
        hides_host_locality_or_owning_lane: false,
        drops_execution_origin_when_restored_or_degraded: false,
        receipt_lineage_not_stable_for_reuse: false,
        conceals_boundary_or_origin_in_generic_status_wording: false,
    }
}

fn controls_rows() -> Vec<M5HostOriginControlsRow> {
    use M5BuildRemoteConsumerSurface as C;
    use M5BuildRemoteDowngradeTrigger as D;

    vec![
        base_row(
            C::RunTestDebugUi,
            "Run/test/debug surface owner",
            "Every run, test, and debug target renders a host-boundary strip naming its locality class, target label, and owning runtime/service lane before the user trusts logs or actions; an execution-origin receipt row names the action class, resolved target identity, execution-context provenance, and export-safe lineage",
            "evidence:m5-host-origin-run-test-debug:001",
            vec![
                D::HostBoundaryUnstated,
                D::ExecutionOriginUnstated,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![strip_local(), strip_ssh(), strip_locality_unstated()],
            vec![receipt_local_signed(), receipt_lineage_not_export_safe()],
        ),
        base_row(
            C::PreviewUi,
            "Preview surface owner",
            "Preview targets reuse the same host-boundary strip vocabulary, distinguishing container and devcontainer execution and degrading honestly when the owning lane is unstated; the execution-origin receipt keeps host ownership through a restore instead of dropping it",
            "evidence:m5-host-origin-preview:001",
            vec![
                D::HostBoundaryUnstated,
                D::ExecutionOriginUnstated,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![
                strip_container(),
                strip_devcontainer(),
                strip_owning_lane_unstated(),
            ],
            vec![
                receipt_bridged_attributed(),
                receipt_ownership_dropped_on_restore(),
            ],
        ),
        base_row(
            C::CompanionUi,
            "AI tool-routing owner",
            "AI tool routing reads the same host-boundary strip so a managed or browser-bridge target is distinguishable before the model runs, debugs, or hands off work; the execution-origin receipt carries host ownership through a reconnecting restore so ownership never disappears",
            "evidence:m5-host-origin-ai-tool-routing:001",
            vec![D::HostBoundaryUnstated, D::ExecutionOriginUnstated, D::ProofStale],
            vec![strip_managed(), strip_browser_bridge()],
            vec![receipt_restored_retains_ownership()],
        ),
        base_row(
            C::IncidentUi,
            "Incident/ops owner",
            "Incident and ops surfaces keep the same host-boundary language, distinguishing service-plane execution and degrading honestly when a reconnecting host hides its degraded state; the execution-origin receipt degrades rather than publish an unstated target identity",
            "evidence:m5-host-origin-incident:001",
            vec![
                D::HostBoundaryUnstated,
                D::ExecutionOriginUnstated,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![strip_service_plane(), strip_reconnect_degraded_unstated()],
            vec![receipt_target_identity_unstated(), receipt_local_signed()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved strip and receipt truth, so a stale provenance, an unattributed action class, or a dropped execution origin is visible in evidence rather than hidden behind feature-local prose, and the lineage stays reusable across diagnostics and release evidence",
            "evidence:m5-host-origin-support-export:001",
            vec![
                D::HostBoundaryUnstated,
                D::ExecutionOriginUnstated,
                D::GenericStatusWordingUsed,
                D::ProofStale,
            ],
            vec![strip_local()],
            vec![
                receipt_provenance_unstated(),
                receipt_action_class_unstated(),
                receipt_local_signed(),
            ],
        ),
    ]
}

fn governance_review() -> M5HostOriginGovernanceReview {
    M5HostOriginGovernanceReview {
        strip_names_locality_and_target_label: true,
        strip_names_owning_lane_and_reconnect_state: true,
        host_ownership_always_explicit: true,
        receipt_names_action_class_and_target_identity: true,
        receipt_names_provenance_and_export_safe_lineage: true,
        host_ownership_never_disappears_on_restore: true,
        receipt_lineage_always_reusable: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5HostOriginConsumerProjection {
    M5HostOriginConsumerProjection {
        run_test_debug_surfaces_consume_host_boundary_vocabulary: true,
        preview_surfaces_consume_host_boundary_vocabulary: true,
        ai_tool_routing_consumes_host_boundary_vocabulary: true,
        incident_ops_consumes_host_boundary_vocabulary: true,
        support_export_reads_single_origin_source: true,
        host_boundary_language_consistent_across_surfaces: true,
    }
}

fn proof_freshness() -> M5HostOriginProofFreshness {
    M5HostOriginProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5HostOriginReleasePosture {
    M5HostOriginReleasePosture {
        proof_packet_ref: M5_HOST_ORIGIN_CONTROLS_ARTIFACT_REF.to_owned(),
        boundary_audit_ref: M5_HOST_ORIGIN_CONTROLS_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_HOST_ORIGIN_CONTROLS_SCHEMA_REF,
        M5_HOST_ORIGIN_CONTROLS_DOC_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
        M5_BUILD_REMOTE_BOUNDARY_COMPONENT_DOC_REF,
        M5_HOST_BOUNDARY_STRIP_SCHEMA_REF,
        M5_EXECUTION_ORIGIN_RECEIPT_ROW_SCHEMA_REF,
        M5_HOST_ORIGIN_HOST_BOUNDARY_PATH,
    ])
}

/// Builds the canonical M5 host-boundary-strip / execution-origin-receipt-row controls packet.
pub fn seeded_m5_host_origin_controls() -> M5HostOriginControlsPacket {
    M5HostOriginControlsPacket::new(M5HostOriginControlsPacketInput {
        packet_id: M5_HOST_ORIGIN_CONTROLS_PACKET_ID.to_owned(),
        controls_label:
            "M5 host-boundary-strip and execution-origin-receipt-row controls with locality class, target label, owning runtime/service lane, reconnect/degraded state, action class, resolved target identity, execution-context provenance, and export-safe lineage truth"
                .to_owned(),
        controls_rows: controls_rows(),
        vocabulary_set: M5HostOriginVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the run/test/debug row is held at Beta pending host-boundary-strip parity on
/// every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_host_origin_controls_host_boundary_strip_beta_narrowed(
) -> M5HostOriginControlsPacket {
    let mut packet = seeded_m5_host_origin_controls();
    packet.packet_id =
        "m5-host-boundary-strip-execution-origin-receipt-row-controls:host-boundary-strip-beta:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BuildRemoteConsumerSurface::RunTestDebugUi)
        .expect("run/test/debug row present");
    row.qualification = M5BuildRemoteQualificationClass::Beta;
    packet
}

/// Narrowed variant: the preview row is narrowed to Preview pending execution-origin-receipt-row
/// parity on every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_host_origin_controls_execution_origin_receipt_row_preview_narrowed(
) -> M5HostOriginControlsPacket {
    let mut packet = seeded_m5_host_origin_controls();
    packet.packet_id =
        "m5-host-boundary-strip-execution-origin-receipt-row-controls:execution-origin-receipt-row-preview:0001"
            .to_owned();
    let row = packet
        .controls_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5BuildRemoteConsumerSurface::PreviewUi)
        .expect("preview row present");
    row.qualification = M5BuildRemoteQualificationClass::Preview;
    packet
}
