//! Canonical seed builders for the frozen M5 terminal-tab, remote-target-pill,
//! environment-status-strip, toolchain-pin-row, presence-avatar-stack, and
//! repair-action-card component matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical runtime-boundary-component matrix.
pub const M5_RUNTIME_BOUNDARY_MATRIX_PACKET_ID: &str = "m5-runtime-boundary-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-06T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5RuntimeBoundaryRequiredLabel> {
    M5RuntimeBoundaryRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5RuntimeBoundaryRequiredLabel]) -> Vec<M5RuntimeBoundaryRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5RuntimeBoundaryComponentFamily,
    qualification: M5RuntimeBoundaryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    shell_zone_slot: M5ShellZoneSlot,
    proof_ref: &str,
) -> M5RuntimeBoundaryComponentRow {
    M5RuntimeBoundaryComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        shell_zone_slot,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        surface_families: M5ShellSurfaceFamily::ALL.to_vec(),
        required_labels: mandatory_labels(),
        shell_integration_qualities: vec![],
        session_liveness_states: vec![],
        host_boundary_classes: vec![],
        connection_states: vec![],
        runtime_source_classes: vec![],
        toolchain_source_classes: vec![],
        toolchain_pin_states: vec![],
        collaboration_roles: vec![],
        follow_states: vec![],
        repair_blast_radii: vec![],
        reversibility_classes: vec![],
        accessibility_routes: M5RuntimeBoundaryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5RuntimeBoundaryDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RUNTIME_BOUNDARY_SCHEMA_REF,
            M5_RUNTIME_BOUNDARY_SHELL_ZONE_REF,
        ]),
        masks_host_or_runtime_boundary: false,
        conflates_live_and_restored_session: false,
        invents_private_status_grammar: false,
        overstates_reversibility_or_drops_audit_truth: false,
    }
}

fn component_rows() -> Vec<M5RuntimeBoundaryComponentRow> {
    use M5CollaborationRole as CR;
    use M5FollowState as FS;
    use M5HostBoundaryClass as HB;
    use M5RemoteConnectionState as CN;
    use M5RepairBlastRadius as BR;
    use M5ReversibilityClass as RV;
    use M5RuntimeBoundaryComponentFamily as F;
    use M5RuntimeBoundaryDowngradeTrigger as D;
    use M5RuntimeBoundaryQualificationClass as Q;
    use M5RuntimeBoundaryRequiredLabel as L;
    use M5RuntimeSourceClass as Rc;
    use M5ShellConsumerSurface as C;
    use M5ShellIntegrationQuality as SI;
    use M5ShellZoneSlot as Z;
    use M5TerminalSessionLiveness as SL;
    use M5ToolchainPinState as TP;
    use M5ToolchainSourceClass as TS;

    let mut rows = Vec::new();

    // 1. Terminal tab / header strip.
    let mut row = base_row(
        F::TerminalTab,
        Q::Stable,
        "Terminal/session component owner",
        "One terminal-tab model carrying the session title, the host boundary it runs against, and the true shell-integration quality; it never implies richer integration than the live session provides and never conflates a live session with a restored transcript",
        Z::BottomPanel,
        "evidence:m5-terminal-tab-parity:001",
    );
    row.shell_integration_qualities = SI::ALL.to_vec();
    row.session_liveness_states = SL::ALL.to_vec();
    row.required_labels = labels_with(&[L::Boundary, L::ResolvedSource]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::Layout,
        C::StatusBar,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ShellIntegrationQualityHidden,
        D::SessionLivenessAmbiguous,
        D::HostBoundaryMasked,
        D::AuditTruthLostOffPrimarySurface,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Remote target pill.
    let mut row = base_row(
        F::RemoteTargetPill,
        Q::Stable,
        "Remote/transport component owner",
        "One remote-target-pill model naming the host boundary — local, remote, container, managed workspace, virtual machine, or sandbox — and the live connection state, so a remote or offline target is never masked as a healthy local one",
        Z::TitleContextBar,
        "evidence:m5-remote-target-pill-parity:001",
    );
    row.host_boundary_classes = HB::ALL.to_vec();
    row.connection_states = CN::ALL.to_vec();
    row.required_labels = labels_with(&[L::Boundary]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::StatusBar,
        C::AttentionRouter,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::HostBoundaryMasked,
        D::ConnectionStateStale,
        D::AuditTruthLostOffPrimarySurface,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Environment status strip.
    let mut row = base_row(
        F::EnvironmentStatusStrip,
        Q::Stable,
        "Environment/runtime component owner",
        "One environment-status-strip model naming the winning runtime source — project pin, workspace, tool manager, system default, container, or session override — so a user never has to infer which runtime is active or why it won",
        Z::StatusBar,
        "evidence:m5-environment-status-strip-parity:001",
    );
    row.runtime_source_classes = Rc::ALL.to_vec();
    row.required_labels = labels_with(&[L::ResolvedSource, L::Boundary]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::StatusBar,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RuntimeSourceUnexplained,
        D::HostBoundaryMasked,
        D::AuditTruthLostOffPrimarySurface,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Toolchain pin row.
    let mut row = base_row(
        F::ToolchainPinRow,
        Q::Stable,
        "Toolchain/tooling component owner",
        "One toolchain-pin-row model explaining why a toolchain won — the source that selected it and its pin state — so a missing, conflicting, or overridden pin is disclosed rather than shown as a clean resolution",
        Z::RightInspector,
        "evidence:m5-toolchain-pin-row-parity:001",
    );
    row.toolchain_source_classes = TS::ALL.to_vec();
    row.toolchain_pin_states = TP::ALL.to_vec();
    row.required_labels = labels_with(&[L::ResolvedSource]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::Layout,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ToolchainPinConflictHidden,
        D::RuntimeSourceUnexplained,
        D::AuditTruthLostOffPrimarySurface,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Presence avatar stack.
    let mut row = base_row(
        F::PresenceAvatarStack,
        Q::Stable,
        "Collaboration/presence component owner",
        "One presence-avatar-stack model showing each participant's collaboration role and follow state, so an observer is never conflated with a controller and who-follows-whom is always explicit",
        Z::TitleContextBar,
        "evidence:m5-presence-avatar-stack-parity:001",
    );
    row.collaboration_roles = CR::ALL.to_vec();
    row.follow_states = FS::ALL.to_vec();
    row.required_labels = labels_with(&[L::Boundary]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::AttentionRouter,
        C::NotificationEnvelope,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::CollaborationRoleMasked,
        D::FollowStateAmbiguous,
        D::AuditTruthLostOffPrimarySurface,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Repair action card.
    let mut row = base_row(
        F::RepairActionCard,
        Q::Stable,
        "Repair/diagnostics component owner",
        "One repair-action-card model showing a repair's blast radius and reversibility class before approval, so a user always knows what a repair will change and whether it can be undone; it never understates blast radius or overstates reversibility",
        Z::TransientOverlay,
        "evidence:m5-repair-action-card-parity:001",
    );
    row.repair_blast_radii = BR::ALL.to_vec();
    row.reversibility_classes = RV::ALL.to_vec();
    row.required_labels = labels_with(&[L::Reversibility, L::Boundary]);
    row.consumer_surfaces = vec![
        C::ShellFrame,
        C::ReleaseProof,
        C::DocsHelp,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RepairBlastRadiusUnderstated,
        D::ReversibilityOverstated,
        D::AuditTruthLostOffPrimarySurface,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5RuntimeBoundaryGovernanceReview {
    M5RuntimeBoundaryGovernanceReview {
        terminal_tab_shows_boundary_and_shell_integration: true,
        remote_pill_shows_host_boundary_and_connection: true,
        environment_strip_names_winning_runtime_source: true,
        toolchain_row_explains_why_toolchain_won: true,
        presence_stack_shows_role_and_follow_state: true,
        repair_card_shows_blast_radius_and_reversibility: true,
        live_versus_restored_never_conflated: true,
        no_component_invents_second_status_grammar: true,
        every_component_bound_to_shell_zone: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5RuntimeBoundaryConsumerProjection {
    M5RuntimeBoundaryConsumerProjection {
        terminal_and_session_surfaces_consume_matrix: true,
        remote_and_environment_surfaces_consume_boundary_vocabulary: true,
        collaboration_surfaces_consume_role_follow_vocabulary: true,
        repair_surfaces_consume_reversibility_vocabulary: true,
        support_export_reads_single_source: true,
        accessibility_bridge_reads_single_source: true,
    }
}

fn proof_freshness() -> M5RuntimeBoundaryProofFreshness {
    M5RuntimeBoundaryProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RuntimeBoundaryReleasePosture {
    M5RuntimeBoundaryReleasePosture {
        release_packet_ref: "artifacts/release/m5-runtime-boundary-proof/support_export.json"
            .to_owned(),
        runtime_boundary_audit_ref: "artifacts/components/m5-runtime-boundary-components.md"
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RUNTIME_BOUNDARY_SCHEMA_REF,
        M5_RUNTIME_BOUNDARY_DOC_REF,
        M5_RUNTIME_BOUNDARY_SHELL_ZONE_REF,
        M5_RUNTIME_BOUNDARY_TERMINAL_CONTRACT_REF,
        M5_RUNTIME_BOUNDARY_REPAIR_CONTRACT_REF,
    ])
}

/// Builds the canonical frozen M5 runtime-boundary-component matrix packet.
pub fn seeded_m5_runtime_boundary_component_matrix() -> M5RuntimeBoundaryMatrixPacket {
    M5RuntimeBoundaryMatrixPacket::new(M5RuntimeBoundaryMatrixPacketInput {
        packet_id: M5_RUNTIME_BOUNDARY_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 terminal-tab, remote-target-pill, environment-status-strip, toolchain-pin-row, presence-avatar-stack, and repair-action-card component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5RuntimeBoundaryVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the presence avatar stack is held at Beta because a slice of
/// follow-state transitions do not yet round-trip across every export path; every
/// component stays visible.
pub fn seeded_m5_runtime_boundary_component_matrix_presence_avatar_stack_beta_narrowed(
) -> M5RuntimeBoundaryMatrixPacket {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet.packet_id = "m5-runtime-boundary-components:presence-avatar-stack-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::PresenceAvatarStack)
        .expect("presence-avatar-stack row present");
    row.qualification = M5RuntimeBoundaryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the repair action card is narrowed to Preview pending
/// reversibility-class parity proof across every repair transaction; every
/// component stays visible.
pub fn seeded_m5_runtime_boundary_component_matrix_repair_action_card_preview_narrowed(
) -> M5RuntimeBoundaryMatrixPacket {
    let mut packet = seeded_m5_runtime_boundary_component_matrix();
    packet.packet_id = "m5-runtime-boundary-components:repair-action-card-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5RuntimeBoundaryComponentFamily::RepairActionCard)
        .expect("repair-action-card row present");
    row.qualification = M5RuntimeBoundaryQualificationClass::Preview;
    packet
}
