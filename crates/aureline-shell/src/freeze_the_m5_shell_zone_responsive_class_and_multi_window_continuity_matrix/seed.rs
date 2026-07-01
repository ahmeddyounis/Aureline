//! Canonical seed builders for the frozen M5 shell-zone / responsive-class /
//! multi-window continuity matrix.
//!
//! These builders are the single producer of the checked-in release-proof
//! support export and the narrowed fixtures. The headless emitter and the inline
//! tests both call them so the in-code matrix, the artifact, and the fixtures
//! never drift.

use super::*;

/// Stable packet id for the canonical shell-zone matrix.
pub const M5_SHELL_ZONE_MATRIX_PACKET_ID: &str = "m5-shell-zone-matrix:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Every claimed M5 shell family carries all six controlled shell vocabularies,
/// every owning-window routing expectation, every workspace-global continuity
/// truth, and survives every responsive class. Only the zone binding, window
/// classes, occupant transitions, collapse ladder, and placeholder behavior vary
/// per family.
#[allow(clippy::too_many_arguments)]
fn row(
    family: M5ShellSurfaceFamily,
    qualification: M5ShellQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    required_fields: &[&str],
    canonical_slot: M5ShellZoneSlot,
    fallback_slot: M5ShellZoneSlot,
    placeholder_behavior: M5PlaceholderBehavior,
    window_classes: Vec<M5WindowClass>,
    occupant_persistence: Vec<M5OccupantPersistence>,
    fallback_placements: Vec<M5FallbackPlacement>,
    required_proof_packet_refs: &[&str],
    downgrade_triggers: Vec<M5ShellDowngradeTrigger>,
    rollback_posture: M5ShellRollbackPosture,
    source_contract_refs: &[&str],
    consumer_surfaces: Vec<M5ShellConsumerSurface>,
) -> M5ShellSurfaceRow {
    M5ShellSurfaceRow {
        family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_fields: strings(required_fields),
        canonical_slot,
        fallback_slot,
        placeholder_behavior,
        state_vocabularies: family.required_state_vocabularies().to_vec(),
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes,
        occupant_persistence,
        fallback_placements,
        owning_window_routing: M5OwningWindowRouting::ALL.to_vec(),
        continuity_truths: M5ContinuityTruth::ALL.to_vec(),
        evidence_requirement: M5ShellEvidenceRequirement::Required,
        required_proof_packet_refs: strings(required_proof_packet_refs),
        downgrade_triggers,
        rollback_posture,
        source_contract_refs: strings(source_contract_refs),
        consumer_surfaces,
    }
}

fn surface_rows() -> Vec<M5ShellSurfaceRow> {
    use M5ShellConsumerSurface as C;
    use M5ShellDowngradeTrigger as D;
    use M5FallbackPlacement as F;
    use M5OccupantPersistence as O;
    use M5PlaceholderBehavior as P;
    use M5ShellZoneSlot as Z;
    use M5WindowClass as W;

    // Every family binds to a slot contract and the reference-layout contract;
    // remote/provider-backed families additionally cite session-restore, and
    // routed families cite attention-routing and the notification envelope.
    let base_contracts = &[
        M5_SHELL_ZONING_CONTRACT_REF,
        M5_SHELL_REFERENCE_LAYOUT_CONTRACT_REF,
    ];
    let routed_contracts = &[
        M5_SHELL_ZONING_CONTRACT_REF,
        M5_SHELL_ATTENTION_ROUTING_CONTRACT_REF,
        M5_SHELL_NOTIFICATION_ENVELOPE_CONTRACT_REF,
    ];
    let remote_contracts = &[
        M5_SHELL_ZONING_CONTRACT_REF,
        M5_SHELL_WINDOW_TOPOLOGY_CONTRACT_REF,
        M5_SHELL_SESSION_RESTORE_CONTRACT_REF,
    ];

    vec![
        row(
            M5ShellSurfaceFamily::Notebook,
            M5ShellQualificationClass::Stable,
            "Notebook surface owner",
            "Notebook editor / cell surface docked in the main workspace; it may split side-by-side or tab with peer editors, detach to a secondary window carrying full workspace-global truth, and collapse to an in-slot placeholder that preserves the notebook identity and reopen path when a kernel or provider is missing",
            &[
                "surface_id",
                "canonical_slot",
                "fallback_slot",
                "placeholder_behavior",
                "owning_window_route",
            ],
            Z::MainWorkspace,
            Z::MainWorkspace,
            P::InSlotIdentityPreserved,
            vec![W::PrimaryWorkspaceWindow, W::SecondaryDetachedWindow],
            vec![O::SideBySide, O::Tabbed],
            vec![F::Docked, F::Overflow, F::Placeholder],
            &["evidence:m5-shell-continuity:notebook"],
            vec![
                D::CollapseChangedTaskIdentity,
                D::WorkspaceTruthDivergedAcrossWindows,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ShellRollbackPosture::CollapsePreservesTaskIdentity,
            remote_contracts,
            vec![
                C::ShellFrame,
                C::Windowing,
                C::Layout,
                C::DocsHelp,
                C::ReleaseProof,
                C::ProductUi,
            ],
        ),
        row(
            M5ShellSurfaceFamily::DataGrid,
            M5ShellQualificationClass::Stable,
            "Data surface owner",
            "Tabular data grid docked in the main workspace; it may split side-by-side or tab with peer surfaces, detach to a secondary window, and collapse to a placeholder that prompts to reconnect the remote or reauthorize the provider when the data source is unavailable",
            &[
                "surface_id",
                "canonical_slot",
                "fallback_slot",
                "placeholder_behavior",
                "owning_window_route",
            ],
            Z::MainWorkspace,
            Z::MainWorkspace,
            P::ReconnectRemoteOrProvider,
            vec![W::PrimaryWorkspaceWindow, W::SecondaryDetachedWindow],
            vec![O::SideBySide, O::Tabbed],
            vec![F::Docked, F::Overflow, F::Placeholder],
            &["evidence:m5-shell-continuity:data-grid"],
            vec![
                D::WorkspaceTruthDivergedAcrossWindows,
                D::PlaceholderLostIdentityOrReopen,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ShellRollbackPosture::WindowPreservesWorkspaceGlobalTruth,
            remote_contracts,
            vec![
                C::ShellFrame,
                C::Windowing,
                C::Layout,
                C::DocsHelp,
                C::ReleaseProof,
                C::ProductUi,
            ],
        ),
        row(
            M5ShellSurfaceFamily::Profiler,
            M5ShellQualificationClass::Stable,
            "Profiler surface owner",
            "Profiler / performance surface docked in the bottom panel; it tabs, sheets, or overflows under compact widths, may float as a utility window scoped to one capture, and collapses to a placeholder that prompts to reconnect the profiling provider when a capture session is lost",
            &[
                "surface_id",
                "canonical_slot",
                "fallback_slot",
                "placeholder_behavior",
                "owning_window_route",
            ],
            Z::BottomPanel,
            Z::TransientOverlay,
            P::ReconnectRemoteOrProvider,
            vec![
                W::PrimaryWorkspaceWindow,
                W::SecondaryDetachedWindow,
                W::FloatingUtilityWindow,
            ],
            vec![O::Tabbed, O::Sheeted, O::Overflowed],
            vec![F::Docked, F::Sheet, F::Overflow, F::Placeholder],
            &["evidence:m5-shell-continuity:profiler"],
            vec![
                D::CriticalStateHiddenOnCollapse,
                D::PlaceholderLostIdentityOrReopen,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ShellRollbackPosture::CriticalStateStaysVisibleOrOverflowed,
            remote_contracts,
            vec![
                C::ShellFrame,
                C::Windowing,
                C::Layout,
                C::DocsHelp,
                C::ReleaseProof,
                C::ProductUi,
            ],
        ),
        row(
            M5ShellSurfaceFamily::Pipeline,
            M5ShellQualificationClass::Stable,
            "Pipeline surface owner",
            "Pipeline / workflow graph docked in the main workspace; it may split side-by-side or tab with peer surfaces, drop to the bottom panel as a fallback slot under compact widths, and collapse to a placeholder that prompts to reconnect the run provider when live status is unavailable",
            &[
                "surface_id",
                "canonical_slot",
                "fallback_slot",
                "placeholder_behavior",
                "owning_window_route",
            ],
            Z::MainWorkspace,
            Z::BottomPanel,
            P::ReconnectRemoteOrProvider,
            vec![W::PrimaryWorkspaceWindow, W::SecondaryDetachedWindow],
            vec![O::SideBySide, O::Tabbed],
            vec![F::Docked, F::Overflow, F::Placeholder],
            &["evidence:m5-shell-continuity:pipeline"],
            vec![
                D::CollapseChangedTaskIdentity,
                D::WorkspaceTruthDivergedAcrossWindows,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ShellRollbackPosture::AttachesOnlyToDeclaredSlot,
            remote_contracts,
            vec![
                C::ShellFrame,
                C::Windowing,
                C::Layout,
                C::DocsHelp,
                C::ReleaseProof,
                C::ProductUi,
            ],
        ),
        row(
            M5ShellSurfaceFamily::Docs,
            M5ShellQualificationClass::Stable,
            "Docs surface owner",
            "Documentation reader docked in the main workspace; it may split side-by-side, tab, or sheet, float as a utility window for reference-while-working, and collapse to an in-slot placeholder that preserves the document anchor and reopen path when content is not yet loaded",
            &[
                "surface_id",
                "canonical_slot",
                "fallback_slot",
                "placeholder_behavior",
                "owning_window_route",
            ],
            Z::MainWorkspace,
            Z::TransientOverlay,
            P::InSlotIdentityPreserved,
            vec![
                W::PrimaryWorkspaceWindow,
                W::SecondaryDetachedWindow,
                W::FloatingUtilityWindow,
            ],
            vec![O::SideBySide, O::Tabbed, O::Sheeted],
            vec![F::Docked, F::Sheet, F::Overflow, F::Placeholder],
            &["evidence:m5-shell-continuity:docs"],
            vec![
                D::SlotUndeclared,
                D::CollapseChangedTaskIdentity,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ShellRollbackPosture::CollapsePreservesTaskIdentity,
            base_contracts,
            vec![
                C::ShellFrame,
                C::Layout,
                C::DocsHelp,
                C::ReleaseProof,
                C::SupportExport,
                C::ProductUi,
            ],
        ),
        row(
            M5ShellSurfaceFamily::Preview,
            M5ShellQualificationClass::Stable,
            "Preview surface owner",
            "Preview surface (render, diff, media) docked in the right inspector; it may split side-by-side, sheet, or overflow under compact widths, float as a utility window or companion overlay attached to its owning editor, and collapse to an in-slot placeholder preserving the previewed object identity",
            &[
                "surface_id",
                "canonical_slot",
                "fallback_slot",
                "placeholder_behavior",
                "owning_window_route",
            ],
            Z::RightInspector,
            Z::TransientOverlay,
            P::InSlotIdentityPreserved,
            vec![
                W::PrimaryWorkspaceWindow,
                W::FloatingUtilityWindow,
                W::CompanionOverlayWindow,
            ],
            vec![O::SideBySide, O::Sheeted, O::Overflowed],
            vec![F::Docked, F::Sheet, F::Overflow, F::Placeholder],
            &["evidence:m5-shell-continuity:preview"],
            vec![
                D::CriticalStateHiddenOnCollapse,
                D::OwningWindowRoutingLost,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ShellRollbackPosture::RoutesToOwningWindowObject,
            routed_contracts,
            vec![
                C::ShellFrame,
                C::Layout,
                C::AttentionRouter,
                C::DocsHelp,
                C::ReleaseProof,
                C::ProductUi,
            ],
        ),
        row(
            M5ShellSurfaceFamily::Review,
            M5ShellQualificationClass::Stable,
            "Review surface owner",
            "Review / change-request surface docked in the main workspace; it may split side-by-side or tab with the diff it reviews, fall back to the right inspector under compact widths, detach to a secondary window, and route approval dialogs back to the owning window and object without focus theft",
            &[
                "surface_id",
                "canonical_slot",
                "fallback_slot",
                "placeholder_behavior",
                "owning_window_route",
            ],
            Z::MainWorkspace,
            Z::RightInspector,
            P::ReconnectRemoteOrProvider,
            vec![W::PrimaryWorkspaceWindow, W::SecondaryDetachedWindow],
            vec![O::SideBySide, O::Tabbed],
            vec![F::Docked, F::Overflow, F::Placeholder],
            &["evidence:m5-shell-continuity:review"],
            vec![
                D::OwningWindowRoutingLost,
                D::WorkspaceTruthDivergedAcrossWindows,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ShellRollbackPosture::RoutesToOwningWindowObject,
            routed_contracts,
            vec![
                C::ShellFrame,
                C::Windowing,
                C::AttentionRouter,
                C::NotificationEnvelope,
                C::DocsHelp,
                C::ReleaseProof,
                C::ProductUi,
            ],
        ),
        row(
            M5ShellSurfaceFamily::Incident,
            M5ShellQualificationClass::Beta,
            "Incident surface owner",
            "Incident / operations-response surface docked in the main workspace; it may split side-by-side or tab, fall back to the right inspector, detach to a secondary window for a war-room display, and route paging and approval actions back to the owning incident object without orphaning",
            &[
                "surface_id",
                "canonical_slot",
                "fallback_slot",
                "placeholder_behavior",
                "owning_window_route",
            ],
            Z::MainWorkspace,
            Z::RightInspector,
            P::ReconnectRemoteOrProvider,
            vec![W::PrimaryWorkspaceWindow, W::SecondaryDetachedWindow],
            vec![O::SideBySide, O::Tabbed],
            vec![F::Docked, F::Overflow, F::Placeholder],
            &["evidence:m5-shell-continuity:incident"],
            vec![
                D::OwningWindowRoutingLost,
                D::SecondaryDisplayTopologyDrift,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ShellRollbackPosture::RoutesToOwningWindowObject,
            routed_contracts,
            vec![
                C::ShellFrame,
                C::Windowing,
                C::AttentionRouter,
                C::NotificationEnvelope,
                C::DocsHelp,
                C::ReleaseProof,
                C::ProductUi,
            ],
        ),
        row(
            M5ShellSurfaceFamily::Companion,
            M5ShellQualificationClass::Beta,
            "Companion surface owner",
            "Companion assistant surface that sheets or overflows against the right inspector, may run as a companion overlay window or floating utility attached to its owning window, and collapses to a placeholder that prompts to install or enable the companion when its dependency is absent — it never claims a solo top-level chrome",
            &[
                "surface_id",
                "canonical_slot",
                "fallback_slot",
                "placeholder_behavior",
                "owning_window_route",
            ],
            Z::RightInspector,
            Z::TransientOverlay,
            P::InstallOrEnableDependency,
            vec![
                W::PrimaryWorkspaceWindow,
                W::CompanionOverlayWindow,
                W::FloatingUtilityWindow,
            ],
            vec![O::Sheeted, O::Overflowed, O::SoloDocked],
            vec![F::Sheet, F::Overflow, F::Placeholder],
            &["evidence:m5-shell-continuity:companion"],
            vec![
                D::SlotUndeclared,
                D::OwningWindowRoutingLost,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ShellRollbackPosture::AttachesOnlyToDeclaredSlot,
            routed_contracts,
            vec![
                C::ShellFrame,
                C::Layout,
                C::AttentionRouter,
                C::NotificationEnvelope,
                C::DocsHelp,
                C::ReleaseProof,
                C::ProductUi,
            ],
        ),
        row(
            M5ShellSurfaceFamily::Operator,
            M5ShellQualificationClass::Beta,
            "Operator surface owner",
            "Operator / control-plane surface docked in the bottom panel; it tabs, sheets, or overflows under compact widths, may detach to a secondary window or float as a utility scoped to one control, and collapses to a placeholder that prompts to reconnect the control-plane provider when it recenters after a display topology drift",
            &[
                "surface_id",
                "canonical_slot",
                "fallback_slot",
                "placeholder_behavior",
                "owning_window_route",
            ],
            Z::BottomPanel,
            Z::TransientOverlay,
            P::RecenteredOnTopologyDrift,
            vec![
                W::PrimaryWorkspaceWindow,
                W::SecondaryDetachedWindow,
                W::FloatingUtilityWindow,
            ],
            vec![O::Tabbed, O::Sheeted, O::Overflowed],
            vec![F::Docked, F::Sheet, F::Overflow, F::Placeholder],
            &["evidence:m5-shell-continuity:operator"],
            vec![
                D::SecondaryDisplayTopologyDrift,
                D::WorkspaceTruthDivergedAcrossWindows,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            M5ShellRollbackPosture::WindowPreservesWorkspaceGlobalTruth,
            remote_contracts,
            vec![
                C::ShellFrame,
                C::Windowing,
                C::Layout,
                C::StatusBar,
                C::DocsHelp,
                C::ReleaseProof,
                C::ProductUi,
            ],
        ),
    ]
}

fn continuity_review() -> M5ShellContinuityReview {
    M5ShellContinuityReview {
        new_surfaces_attach_only_to_declared_slots: true,
        responsive_collapse_never_changes_task_identity: true,
        responsive_collapse_never_hides_critical_state: true,
        every_window_preserves_workspace_global_trust_remote_profile_recovery: true,
        layout_stays_local_while_truth_stays_global: true,
        dialogs_notifications_approvals_route_to_owning_window_object: true,
        no_focus_theft_or_orphaning: true,
        secondary_display_and_zoom_preserve_identity: true,
        one_shell_zone_matrix_not_local_layout_prose: true,
        no_surface_invents_its_own_slot_or_collapse: true,
        downgrade_narrows_instead_of_hides: true,
        unmapped_surface_blocks_shell_maturity_claim: true,
    }
}

fn consumer_projection() -> M5ShellConsumerProjection {
    M5ShellConsumerProjection {
        shell_frame_consumes_slot_matrix: true,
        windowing_consumes_window_classes: true,
        layout_consumes_responsive_classes: true,
        status_bar_consumes_status_slot: true,
        attention_router_routes_to_owning_window: true,
        notification_envelope_uses_owning_window_routing: true,
        docs_help_consume_slot_metadata: true,
        release_proof_consumes_slot_metadata: true,
        support_export_shows_shell_zone_matrix: true,
        preview_labs_label_for_unmapped_surfaces: true,
    }
}

fn proof_freshness() -> M5ShellProofFreshness {
    M5ShellProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ShellReleasePosture {
    M5ShellReleasePosture {
        release_packet_ref: "evidence:m5-shell-continuity-release-packet".to_owned(),
        multi_window_proof_packet_ref: "evidence:m5-shell-continuity-multi-window-packet"
            .to_owned(),
        support_export_parity_required: true,
        multi_window_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SHELL_ZONE_MATRIX_SCHEMA_REF,
        M5_SHELL_RESPONSIVE_CLASS_SCHEMA_REF,
        M5_SHELL_ZONE_MATRIX_DOC_REF,
        M5_SHELL_ZONING_CONTRACT_REF,
        M5_SHELL_WINDOW_TOPOLOGY_CONTRACT_REF,
        M5_SHELL_ATTENTION_ROUTING_CONTRACT_REF,
        M5_SHELL_NOTIFICATION_ENVELOPE_CONTRACT_REF,
        M5_SHELL_SESSION_RESTORE_CONTRACT_REF,
        M5_SHELL_REFERENCE_LAYOUT_CONTRACT_REF,
    ])
}

fn base_input() -> M5ShellZoneMatrixPacketInput {
    M5ShellZoneMatrixPacketInput {
        packet_id: M5_SHELL_ZONE_MATRIX_PACKET_ID.to_owned(),
        matrix_label: "M5 Shell-Zone, Responsive-Class, and Multi-Window Continuity Matrix"
            .to_owned(),
        surface_rows: surface_rows(),
        vocabulary_set: M5ShellVocabularySet::canonical(),
        continuity_review: continuity_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical stable M5 shell-zone matrix packet.
///
/// This is the single producer of the checked-in release-proof support export.
pub fn seeded_m5_shell_zone_matrix() -> M5ShellZoneMatrixPacket {
    M5ShellZoneMatrixPacket::new(base_input())
}

/// Builds a narrowed variant where the profiler is held after a lost-provider
/// finding, proving downgrade narrows the claim rather than hiding the surface.
pub fn seeded_m5_shell_zone_matrix_profiler_remote_held() -> M5ShellZoneMatrixPacket {
    let mut input = base_input();
    input.packet_id = "m5-shell-zone-matrix:profiler-remote-held:0001".to_owned();
    for row in &mut input.surface_rows {
        if row.family == M5ShellSurfaceFamily::Profiler {
            row.qualification = M5ShellQualificationClass::Held;
            // A held family no longer carries a public claim, so proof becomes
            // recommended rather than required; the surface stays mapped.
            row.evidence_requirement = M5ShellEvidenceRequirement::Recommended;
        }
    }
    M5ShellZoneMatrixPacket::new(input)
}

/// Builds a narrowed variant where the companion surface is pulled to preview
/// after an owning-window-routing finding, proving auto-narrowing keeps the
/// surface mapped into the matrix.
pub fn seeded_m5_shell_zone_matrix_companion_overlay_narrowed() -> M5ShellZoneMatrixPacket {
    let mut input = base_input();
    input.packet_id = "m5-shell-zone-matrix:companion-overlay-narrowed:0001".to_owned();
    for row in &mut input.surface_rows {
        if row.family == M5ShellSurfaceFamily::Companion {
            row.qualification = M5ShellQualificationClass::Preview;
        }
    }
    M5ShellZoneMatrixPacket::new(input)
}
