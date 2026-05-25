//! Deterministic claimed-stable matrix for desktop handoff-conformance records.
//!
//! Every record here is a genuine projection of the live native desktop
//! contract packet in [`crate::platform_integration`] (which itself composes the
//! [`crate::deeplink::native_handoff`] handoff reviews, the desktop continuity
//! recovery rows, the platform drills, and the notification-privacy page) and
//! the auth-owned system-browser return-paths page surfaced through
//! [`crate::system_browser_return_paths`]. The corpus reads the seeded packet
//! and page for the contributing event / row ids and the live entry-surface,
//! handler-ownership, target-availability, and system-browser exception
//! vocabularies, then reconciles each handoff posture through the governed
//! [`DesktopHandoffConformanceRecord::build`] builder, so a record can never
//! drift from what ships.
//!
//! The matrix spans every required entry path and a span of Stable and narrowed
//! rows:
//!
//! - File-association, protocol-handler, system-open, system-browser auth
//!   return, reveal-in-shell, recent-item, jump-list, native open/save, and
//!   removable-volume / network-share recovery postures qualify **Stable**.
//! - A disclosed device-code auth exception qualifies **Stable** because the
//!   exception is surfaced explicitly with scope, return path, and recovery.
//! - A file-association posture whose Help/About binding surface is still in
//!   preview narrows to **Preview** by its lowest binding surface marker.
//! - A side-by-side last-writer-wins ownership drill and an embedded-browser
//!   auth drill narrow below Stable with a named reason instead of inheriting an
//!   adjacent green row.

use crate::notification_attention_stable::model::{
    AccessibilityDisclosure, AttentionRouteSurface, EntryRouteRecord, LayoutMode,
    LayoutModeDisclosure, LifecycleMarker, RecoveryRouteRecord, StableClaimClass,
};
use crate::platform_integration::{
    seeded_native_desktop_contract_packet_with_time, DesktopEntryEvent,
    DesktopInterruptionRecoveryRow, NativeDesktopContractPacket, PlatformDesktopDrillRow,
};
use crate::system_browser_return_paths::{
    seeded_system_browser_return_paths_beta_page, SystemBrowserPolicyExceptionClass,
    SystemBrowserReturnPathBetaRow, SystemBrowserReturnPathsBetaPage,
    SYSTEM_BROWSER_RETURN_PATHS_BETA_SHARED_CONTRACT_REF,
};

use super::model::{
    required_recovery_routes, AuthDefaultPosture, ChannelClass, DesktopHandoffConformanceInput,
    DesktopHandoffConformanceRecord, EntryPathClass, HandlerOwnershipDisclosure,
    HandoffClaimCeiling, HandoffRecoveryAction, HandoffSurfaceProjectionInput, HandoffTruthSurface,
    HandoffUpstream, PlatformConformanceRow, PlatformProfileClass, TargetRecoveryPosture,
    TrustReviewPosture, TypedTargetIntent,
};
use crate::deeplink::native_handoff::TargetAvailabilityClass;

/// Build identity pinned for every record in the corpus.
const BUILD_IDENTITY: &str = "build:aureline:stable:desktop-handoff:01";

/// Snapshot timestamp pinned for every record in the corpus.
pub const CORPUS_AS_OF: &str = "2026-05-25T12:00:00Z";

const DIAGNOSTICS_EXPORT_REF: &str = "aureline://diagnostics/desktop-handoff-conformance";
const SUPPORT_EXPORT_REF: &str = "aureline://support-export/desktop-handoff-conformance";
const EVIDENCE_ARTIFACT_REF: &str = "aureline://artifact/ux-m4-desktop-handoff-conformance";
const EVIDENCE_FIXTURE_REF: &str = "aureline://fixture/ux-m4-desktop-handoff-conformance";
const NARRATIVE_REF: &str = "aureline://doc/ux-m4-desktop-handoff-conformance";

/// One scenario in the claimed-stable handoff-conformance matrix.
#[derive(Debug, Clone)]
pub struct DesktopHandoffConformanceScenario {
    /// Stable scenario id (also the record id).
    pub scenario_id: &'static str,
    /// On-disk fixture filename.
    pub fixture_filename: String,
    /// Posture token pinned for the scenario.
    pub expected_posture: String,
    /// Expected entry path.
    pub expected_entry_path: EntryPathClass,
    /// Expected derived claim class.
    pub expected_claim_class: StableClaimClass,
    /// Expected stable-qualification verdict.
    pub expected_qualifies_stable: bool,
    /// Expected derived surface lifecycle marker (lowest binding surface).
    pub expected_surface_marker: LifecycleMarker,
    record: DesktopHandoffConformanceRecord,
}

impl DesktopHandoffConformanceScenario {
    /// Returns the governed record for this scenario.
    pub fn record(&self) -> DesktopHandoffConformanceRecord {
        self.record.clone()
    }
}

/// The claimed-stable handoff-conformance matrix, in canonical order.
pub fn desktop_handoff_conformance_corpus() -> Vec<DesktopHandoffConformanceScenario> {
    let packet = seeded_native_desktop_contract_packet_with_time(BUILD_IDENTITY, CORPUS_AS_OF);
    packet
        .validate()
        .expect("seeded native desktop contract packet must validate");
    let page = seeded_system_browser_return_paths_beta_page();

    vec![
        file_association_stable(&packet),
        protocol_handler_owned_stable(&packet),
        system_open_workspace_stable(&packet),
        system_browser_auth_return_stable(&packet, &page),
        reveal_in_shell_read_only_stable(&packet),
        recent_item_reopen_stable(&packet),
        jump_list_action_stable(&packet),
        native_save_boundary_stable(&packet),
        removable_volume_recovery_stable(&packet),
        network_share_recovery_stable(&packet),
        device_code_exception_disclosed_stable(&packet, &page),
        help_about_preview_surface(&packet),
        last_writer_wins_ownership_drill(&packet),
        embedded_browser_no_exception_drill(&packet, &page),
    ]
}

// ---------------------------------------------------------------------------
// Projection helpers
// ---------------------------------------------------------------------------

fn event<'a>(packet: &'a NativeDesktopContractPacket, token: &str) -> &'a DesktopEntryEvent {
    packet
        .desktop_entry_events
        .iter()
        .find(|event| event.source_surface_token == token)
        .unwrap_or_else(|| panic!("contract packet must carry a {token} entry event"))
}

fn recovery_row<'a>(
    packet: &'a NativeDesktopContractPacket,
    token: &str,
) -> &'a DesktopInterruptionRecoveryRow {
    packet
        .recovery_rows
        .iter()
        .find(|row| row.interruption_class_token == token)
        .unwrap_or_else(|| panic!("contract packet must carry a {token} recovery row"))
}

fn auth_row<'a>(
    page: &'a SystemBrowserReturnPathsBetaPage,
    exception_token: &str,
) -> &'a SystemBrowserReturnPathBetaRow {
    page.rows
        .iter()
        .find(|row| row.policy_exception_token == exception_token)
        .unwrap_or_else(|| panic!("auth page must carry a {exception_token} row"))
}

fn full_side_by_side() -> Vec<ChannelClass> {
    ChannelClass::REQUIRED.to_vec()
}

fn platform_rows(packet: &NativeDesktopContractPacket) -> Vec<PlatformConformanceRow> {
    let mapping = [
        (PlatformProfileClass::MacOs, "macos_15_plus_universal"),
        (PlatformProfileClass::Windows, "windows_11_23h2_plus_x86_64"),
        (
            PlatformProfileClass::Linux,
            "linux_ubuntu_24_04_gnome_wayland_x86_64",
        ),
    ];
    mapping
        .iter()
        .map(|(profile, profile_id)| {
            let drills: Vec<&PlatformDesktopDrillRow> = packet
                .platform_drills
                .iter()
                .filter(|drill| drill.platform_profile_id == *profile_id)
                .collect();
            let covered = !drills.is_empty() && drills.iter().all(|drill| drill.current_proof);
            let drill_class_tokens = drills
                .iter()
                .map(|drill| drill.drill_class_token.clone())
                .collect();
            let proof_ref = drills
                .first()
                .map(|drill| drill.row_id.clone())
                .unwrap_or_else(|| format!("platform-drill:{profile_id}"));
            PlatformConformanceRow {
                profile: *profile,
                profile_id: (*profile_id).to_string(),
                covered,
                proof_ref,
                drill_class_tokens,
            }
        })
        .collect()
}

fn surface_inputs(help_about_marker: LifecycleMarker) -> Vec<HandoffSurfaceProjectionInput> {
    vec![
        HandoffSurfaceProjectionInput {
            surface: HandoffTruthSurface::DesktopHandoffReview,
            surface_marker: LifecycleMarker::Stable,
            reads_shared_record: true,
        },
        HandoffSurfaceProjectionInput {
            surface: HandoffTruthSurface::CliInspect,
            surface_marker: LifecycleMarker::Stable,
            reads_shared_record: true,
        },
        HandoffSurfaceProjectionInput {
            surface: HandoffTruthSurface::HelpAbout,
            surface_marker: help_about_marker,
            reads_shared_record: true,
        },
        HandoffSurfaceProjectionInput {
            surface: HandoffTruthSurface::SupportExport,
            surface_marker: LifecycleMarker::Stable,
            reads_shared_record: true,
        },
    ]
}

fn entry_routes(posture_id: &str) -> Vec<EntryRouteRecord> {
    AttentionRouteSurface::REQUIRED
        .into_iter()
        .map(|surface| EntryRouteRecord {
            surface,
            route_ref: format!("aureline://handoff-review/{posture_id}#{}", surface.as_str()),
            keyboard_reachable: true,
            activates_same_item: true,
        })
        .collect()
}

fn accessibility(routes: &[RecoveryRouteRecord]) -> AccessibilityDisclosure {
    AccessibilityDisclosure {
        focus_order_index: 0,
        tab_stop_count: routes.len() as u32 + 1,
        row_narration: "Desktop handoff review, owned by the desktop shell; lists the resolved \
                        target, the owning channel, the trust posture, and the recovery actions."
            .to_string(),
        action_labels: routes.iter().map(|r| r.action_label.clone()).collect(),
        layout_modes: LayoutMode::REQUIRED
            .into_iter()
            .map(|mode| LayoutModeDisclosure {
                mode,
                row_narration_available: true,
                recovery_affordances_reachable: true,
            })
            .collect(),
    }
}

fn full_claim_ceiling() -> HandoffClaimCeiling {
    HandoffClaimCeiling {
        asserts_typed_intent_preserved: true,
        asserts_handler_ownership_explicit: true,
        asserts_system_browser_default: true,
        asserts_trust_review_enforced: true,
        asserts_recovery_truthful: true,
        asserts_platform_conformance_complete: true,
    }
}

fn ownership(owning: ChannelClass, handler_token: &str) -> HandlerOwnershipDisclosure {
    HandlerOwnershipDisclosure {
        owning_channel_ref: format!("aureline://install-channel/{}", owning.as_str()),
        owner_build_ref: BUILD_IDENTITY.to_string(),
        owning_channel_class: owning,
        handler_ownership_token: handler_token.to_string(),
        ownership_review_state_token: "no_change".to_string(),
        side_by_side_channels: full_side_by_side(),
        ownership_explicit: true,
        no_last_writer_wins: true,
        spoof_resistant: true,
    }
}

fn trust_enforced(
    event: &DesktopEntryEvent,
    requested_scope: &str,
    granted_scope: &str,
) -> TrustReviewPosture {
    TrustReviewPosture {
        trust_state_token: "trusted".to_string(),
        profile_or_tenant_scope_ref: event.trust_profile_context_ref.clone(),
        policy_epoch_ref: event.policy_epoch_ref.clone(),
        requested_authority_scope_token: requested_scope.to_string(),
        granted_authority_scope_token: granted_scope.to_string(),
        review_required_before_widening: true,
        trust_profile_tenant_checked: true,
        no_silent_authority_widening: true,
    }
}

fn auth_not_applicable() -> AuthDefaultPosture {
    AuthDefaultPosture {
        applies: false,
        default_to_system_browser: true,
        system_browser_default: true,
        exception_class: SystemBrowserPolicyExceptionClass::SystemBrowserDefaultNoException,
        exception_scope_ref: None,
        return_path_ref: None,
        return_mode_token: "not_applicable".to_string(),
        recovery_on_exception_ref: None,
        embedded_browser_used: false,
    }
}

fn exact_recovery() -> TargetRecoveryPosture {
    TargetRecoveryPosture {
        availability: TargetAvailabilityClass::ExactAvailable,
        placeholder_required: false,
        last_seen_identity_ref: None,
        unsaved_local_state_posture_token: "no_unsaved_local_state".to_string(),
        local_work_preserved: true,
        recovery_actions: vec![
            HandoffRecoveryAction::OpenBoundTarget,
            HandoffRecoveryAction::ReviewHandlerOwnership,
            HandoffRecoveryAction::ExportHandoffSupport,
        ],
        no_silent_replay_or_authority_reuse: true,
        canonical_target_path_label: None,
        write_posture_token: "local_default".to_string(),
        profile_remote_boundary_note: None,
    }
}

fn intent_from_event(
    event: &DesktopEntryEvent,
    posture_id: &str,
    availability: TargetAvailabilityClass,
) -> TypedTargetIntent {
    TypedTargetIntent {
        source_surface_token: event.source_surface_token.clone(),
        origin_class_token: event.origin_class_token.clone(),
        requested_action_class_token: event.requested_action_class_token.clone(),
        literal_target_label: event.literal_target_label.clone(),
        literal_target_ref: event.literal_target_ref.clone(),
        canonical_target_ref: format!("aureline://handoff-target/{posture_id}"),
        target_kind_token: event.target_kind_token.clone(),
        resulting_mode_token: event.resulting_mode_token.clone(),
        availability,
        freshness_class_token: event.freshness_class_token.clone(),
        deep_link_intent_ref: Some(event.canonical_target_ref.clone()),
        preserves_literal_target: true,
        preserves_typed_intent: true,
        preserves_resulting_mode: true,
        no_generic_shell_reopen: true,
    }
}

fn upstream(
    packet: &NativeDesktopContractPacket,
    contributing_event_refs: Vec<String>,
) -> HandoffUpstream {
    HandoffUpstream {
        native_contract_packet_ref: packet.packet_id.clone(),
        system_browser_page_ref: SYSTEM_BROWSER_RETURN_PATHS_BETA_SHARED_CONTRACT_REF.to_string(),
        ownership_audit_ref: "install:ownership_audit:v1".to_string(),
        contributing_event_refs,
    }
}

fn evidence_refs() -> Vec<String> {
    vec![
        EVIDENCE_ARTIFACT_REF.to_string(),
        EVIDENCE_FIXTURE_REF.to_string(),
    ]
}

#[allow(clippy::too_many_arguments)]
fn finish(
    scenario_id: &'static str,
    expected_claim_class: StableClaimClass,
    expected_qualifies_stable: bool,
    expected_surface_marker: LifecycleMarker,
    input: DesktopHandoffConformanceInput,
) -> DesktopHandoffConformanceScenario {
    let expected_entry_path = input.entry_path;
    let record = DesktopHandoffConformanceRecord::build(input)
        .unwrap_or_else(|err| panic!("{scenario_id} must build: {err}"));
    DesktopHandoffConformanceScenario {
        scenario_id,
        fixture_filename: format!("{scenario_id}.json"),
        expected_posture: record.posture_id.clone(),
        expected_entry_path,
        expected_claim_class,
        expected_qualifies_stable,
        expected_surface_marker,
        record,
    }
}

// ---------------------------------------------------------------------------
// Stable scenarios
// ---------------------------------------------------------------------------

fn file_association_stable(packet: &NativeDesktopContractPacket) -> DesktopHandoffConformanceScenario {
    let event = event(packet, "file_association");
    let posture_id = "file_association_stable";
    let routes = required_recovery_routes(false, false, false);
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "File association, Stable channel owns the handler".to_string(),
        title: "File association opens the literal file in the owning Stable install".to_string(),
        summary: "A registered file-type association opened a source file: the literal target, the \
                  source-locator intent, and the resulting editor mode were preserved and the Stable \
                  channel is the explicit handler owner across side-by-side installs."
            .to_string(),
        entry_path: EntryPathClass::FileAssociation,
        intent: intent_from_event(event, posture_id, TargetAvailabilityClass::ExactAvailable),
        handler_ownership: ownership(ChannelClass::Stable, &event.handler_ownership_token),
        auth_default: auth_not_applicable(),
        trust_review: trust_enforced(event, "read_write_scope", "read_write_scope"),
        recovery: exact_recovery(),
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![event.event_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        input,
    )
}

fn protocol_handler_owned_stable(
    packet: &NativeDesktopContractPacket,
) -> DesktopHandoffConformanceScenario {
    let event = event(packet, "protocol_handler");
    let posture_id = "protocol_handler_owned_stable";
    let routes = required_recovery_routes(false, false, false);
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "Protocol handler, explicit owner, trust review before widening".to_string(),
        title: "Protocol-handler deep link resolves its target with a trust review".to_string(),
        summary: "A custom-scheme protocol-handler invocation carried a deep-link intent that the \
                  shell resolved to the exact command target; the owning channel is explicit, the \
                  handler is spoof-resistant, and trust / profile / tenant review runs before any \
                  widened authority."
            .to_string(),
        entry_path: EntryPathClass::ProtocolHandler,
        intent: intent_from_event(event, posture_id, TargetAvailabilityClass::ExactAvailable),
        handler_ownership: ownership(ChannelClass::Stable, &event.handler_ownership_token),
        auth_default: auth_not_applicable(),
        trust_review: trust_enforced(event, "step_up_scope", "step_up_scope"),
        recovery: exact_recovery(),
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![event.event_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        input,
    )
}

fn system_open_workspace_stable(
    packet: &NativeDesktopContractPacket,
) -> DesktopHandoffConformanceScenario {
    let event = event(packet, "system_open");
    let posture_id = "system_open_workspace_stable";
    let routes = required_recovery_routes(false, false, false);
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "System open, exact workspace target".to_string(),
        title: "System open resolves the literal workspace, not a generic shell".to_string(),
        summary: "An OS-shell open request resolved to the exact workspace identity rather than \
                  reopening a generic home surface; the owning channel is explicit and the trust \
                  posture is preserved."
            .to_string(),
        entry_path: EntryPathClass::SystemOpen,
        intent: intent_from_event(event, posture_id, TargetAvailabilityClass::ExactAvailable),
        handler_ownership: ownership(ChannelClass::Stable, &event.handler_ownership_token),
        auth_default: auth_not_applicable(),
        trust_review: trust_enforced(event, "read_write_scope", "read_write_scope"),
        recovery: exact_recovery(),
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![event.event_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        input,
    )
}

fn system_browser_auth_return_stable(
    packet: &NativeDesktopContractPacket,
    page: &SystemBrowserReturnPathsBetaPage,
) -> DesktopHandoffConformanceScenario {
    let event = event(packet, "default_browser_callback");
    let row = auth_row(page, "system_browser_default_no_exception");
    let posture_id = "system_browser_auth_return_stable";
    let routes = required_recovery_routes(false, false, false);
    let auth = AuthDefaultPosture {
        applies: true,
        default_to_system_browser: true,
        system_browser_default: row.system_browser_default,
        exception_class: SystemBrowserPolicyExceptionClass::SystemBrowserDefaultNoException,
        exception_scope_ref: None,
        return_path_ref: Some(format!("aureline://auth-return/{posture_id}")),
        return_mode_token: row.return_path_label.return_mode_token.clone(),
        recovery_on_exception_ref: None,
        embedded_browser_used: false,
    };
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "Default-browser auth return, system-browser default".to_string(),
        title: "Auth callback returns through the system browser by default".to_string(),
        summary: "A claimed-identity sign-in handed off to the system default browser and returned \
                  through the typed return path; the row defaults to system-browser handoff with no \
                  exception, no embedded web view, and no silent authority widening."
            .to_string(),
        entry_path: EntryPathClass::DefaultBrowserAuthCallback,
        intent: intent_from_event(event, posture_id, TargetAvailabilityClass::ExactAvailable),
        handler_ownership: ownership(ChannelClass::Stable, &event.handler_ownership_token),
        auth_default: auth,
        trust_review: trust_enforced(
            event,
            &row.requested_authority_scope_token,
            &row.granted_authority_scope_token,
        ),
        recovery: exact_recovery(),
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![event.event_id.clone(), row.row_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        input,
    )
}

fn reveal_in_shell_read_only_stable(
    packet: &NativeDesktopContractPacket,
) -> DesktopHandoffConformanceScenario {
    let event = event(packet, "reveal_in_system_shell");
    let posture_id = "reveal_in_shell_read_only_stable";
    let routes = required_recovery_routes(false, false, false);
    let recovery = TargetRecoveryPosture {
        availability: TargetAvailabilityClass::AvailableReadOnly,
        placeholder_required: false,
        last_seen_identity_ref: None,
        unsaved_local_state_posture_token: "no_unsaved_local_state".to_string(),
        local_work_preserved: true,
        recovery_actions: vec![
            HandoffRecoveryAction::OpenBoundTarget,
            HandoffRecoveryAction::LocateTarget,
            HandoffRecoveryAction::ReviewHandlerOwnership,
            HandoffRecoveryAction::ExportHandoffSupport,
        ],
        no_silent_replay_or_authority_reuse: true,
        canonical_target_path_label: Some(event.literal_target_label.clone()),
        write_posture_token: "read_only".to_string(),
        profile_remote_boundary_note: Some(
            "Revealed in the OS file manager only; the file is shown read-only with its canonical \
             path."
                .to_string(),
        ),
    };
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "Reveal in shell, canonical path and read-only posture".to_string(),
        title: "Reveal-in-shell surfaces the canonical path and read-only posture".to_string(),
        summary: "A reveal-in-system-shell affordance surfaced the canonical target path and a \
                  read-only write posture instead of feeling like disconnected shell glue; the \
                  literal target is preserved and no mutating work replays."
            .to_string(),
        entry_path: EntryPathClass::RevealInSystemShell,
        intent: intent_from_event(event, posture_id, TargetAvailabilityClass::AvailableReadOnly),
        handler_ownership: ownership(ChannelClass::Stable, &event.handler_ownership_token),
        auth_default: auth_not_applicable(),
        trust_review: trust_enforced(event, "read_only_scope", "read_only_scope"),
        recovery,
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![event.event_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        input,
    )
}

fn recent_item_reopen_stable(
    packet: &NativeDesktopContractPacket,
) -> DesktopHandoffConformanceScenario {
    let event = event(packet, "dock_taskbar_recent");
    let posture_id = "recent_item_reopen_stable";
    let routes = required_recovery_routes(false, false, false);
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "Recent item reopen, exact object".to_string(),
        title: "Recent-item activation reopens the exact object in the owning install".to_string(),
        summary: "A dock/taskbar recent-item activation reopened the exact prior object rather than \
                  a generic shell, the recent-item registration is owned by the explicit channel, \
                  and the OS surface cannot execute directly or replay mutating work."
            .to_string(),
        entry_path: EntryPathClass::RecentItemReopen,
        intent: intent_from_event(event, posture_id, TargetAvailabilityClass::ExactAvailable),
        handler_ownership: ownership(ChannelClass::Stable, &event.handler_ownership_token),
        auth_default: auth_not_applicable(),
        trust_review: trust_enforced(event, "read_write_scope", "read_write_scope"),
        recovery: exact_recovery(),
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![event.event_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        input,
    )
}

fn jump_list_action_stable(
    packet: &NativeDesktopContractPacket,
) -> DesktopHandoffConformanceScenario {
    let event = event(packet, "dock_taskbar_jump_action");
    let posture_id = "jump_list_action_stable";
    let routes = required_recovery_routes(false, false, false);
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "Jump-list action, exact target".to_string(),
        title: "Jump-list action reopens the bound workspace, not a generic shell".to_string(),
        summary: "A dock/taskbar jump-list action reopened the exact bound workspace; the action is \
                  owned by the explicit channel, summary-only, and forbidden from executing \
                  mutating work directly from the OS surface."
            .to_string(),
        entry_path: EntryPathClass::JumpListAction,
        intent: intent_from_event(event, posture_id, TargetAvailabilityClass::ExactAvailable),
        handler_ownership: ownership(ChannelClass::Stable, &event.handler_ownership_token),
        auth_default: auth_not_applicable(),
        trust_review: trust_enforced(event, "read_write_scope", "read_write_scope"),
        recovery: exact_recovery(),
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![event.event_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        input,
    )
}

fn native_save_boundary_stable(
    packet: &NativeDesktopContractPacket,
) -> DesktopHandoffConformanceScenario {
    let event = event(packet, "reveal_in_system_shell");
    let posture_id = "native_save_boundary_stable";
    let routes = required_recovery_routes(false, false, false);
    let mut intent =
        intent_from_event(event, posture_id, TargetAvailabilityClass::ExactAvailable);
    intent.source_surface_token = "native_save_dialog".to_string();
    intent.requested_action_class_token = "save_to_target".to_string();
    intent.resulting_mode_token = "native_save".to_string();
    let recovery = TargetRecoveryPosture {
        availability: TargetAvailabilityClass::ExactAvailable,
        placeholder_required: false,
        last_seen_identity_ref: None,
        unsaved_local_state_posture_token: "local_buffer_preserved".to_string(),
        local_work_preserved: true,
        recovery_actions: vec![
            HandoffRecoveryAction::OpenBoundTarget,
            HandoffRecoveryAction::ReviewHandlerOwnership,
            HandoffRecoveryAction::ExportHandoffSupport,
        ],
        no_silent_replay_or_authority_reuse: true,
        canonical_target_path_label: Some("//share/team/specs/handoff.md".to_string()),
        write_posture_token: "overwrite_with_review".to_string(),
        profile_remote_boundary_note: Some(
            "Save target is on a remote profile share; the canonical path and overwrite posture are \
             surfaced before the write."
                .to_string(),
        ),
    };
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "Native save, canonical path and overwrite posture".to_string(),
        title: "Native save surfaces canonical path, overwrite posture, and remote boundary"
            .to_string(),
        summary: "A native save dialog wrote to a remote profile share: the canonical target path, \
                  the overwrite-with-review posture, and the profile/remote boundary note were all \
                  surfaced before the write rather than feeling like disconnected shell glue."
            .to_string(),
        entry_path: EntryPathClass::NativeOpenSave,
        intent,
        handler_ownership: ownership(ChannelClass::Stable, &event.handler_ownership_token),
        auth_default: auth_not_applicable(),
        trust_review: trust_enforced(event, "read_write_scope", "read_write_scope"),
        recovery,
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![event.event_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        input,
    )
}

fn removable_volume_recovery_stable(
    packet: &NativeDesktopContractPacket,
) -> DesktopHandoffConformanceScenario {
    let row = recovery_row(packet, "removable_volume_loss");
    let posture_id = "removable_volume_recovery_stable";
    let routes = required_recovery_routes(true, false, false);
    let intent = TypedTargetIntent {
        source_surface_token: "dock_taskbar_recent".to_string(),
        origin_class_token: "os_shell".to_string(),
        requested_action_class_token: "open_existing_context".to_string(),
        literal_target_label: "client-drive: payments workspace".to_string(),
        literal_target_ref: format!("literal:{}", row.row_id),
        canonical_target_ref: format!("aureline://handoff-target/{posture_id}"),
        target_kind_token: "workspace_root".to_string(),
        resulting_mode_token: row.resulting_mode_token.clone(),
        availability: TargetAvailabilityClass::MissingOrUnmounted,
        freshness_class_token: "stale".to_string(),
        deep_link_intent_ref: Some(row.affected_target_ref.clone()),
        preserves_literal_target: true,
        preserves_typed_intent: true,
        preserves_resulting_mode: true,
        no_generic_shell_reopen: true,
    };
    let recovery = TargetRecoveryPosture {
        availability: TargetAvailabilityClass::MissingOrUnmounted,
        placeholder_required: true,
        last_seen_identity_ref: Some(row.affected_target_ref.clone()),
        unsaved_local_state_posture_token: "unsaved_local_state_preserved".to_string(),
        local_work_preserved: row.local_work_preserved,
        recovery_actions: vec![
            HandoffRecoveryAction::OpenBoundTarget,
            HandoffRecoveryAction::LocateTarget,
            HandoffRecoveryAction::OpenCachedContext,
            HandoffRecoveryAction::ClosePlaceholder,
            HandoffRecoveryAction::ReviewHandlerOwnership,
            HandoffRecoveryAction::ExportHandoffSupport,
        ],
        no_silent_replay_or_authority_reuse: row.no_silent_replay_or_authority_reuse,
        canonical_target_path_label: Some("/Volumes/client-drive/payments".to_string()),
        write_posture_token: "blocked_until_located".to_string(),
        profile_remote_boundary_note: Some(
            "Removable volume is unmounted; the placeholder retains the last-seen identity and \
             unsaved local state."
                .to_string(),
        ),
    };
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "Removable volume lost, recoverable placeholder".to_string(),
        title: "Removable-volume reopen renders a recoverable placeholder".to_string(),
        summary: "A recent-item reopen pointed at a workspace on an unmounted removable volume: \
                  instead of disappearing, the shell rendered a recoverable placeholder with the \
                  last-seen identity, an unsaved-local-state posture, and explicit Locate / Open \
                  cached context / Close placeholder actions."
            .to_string(),
        entry_path: EntryPathClass::RemovableVolumeReopen,
        intent,
        handler_ownership: ownership(ChannelClass::Stable, "current_user_registered"),
        auth_default: auth_not_applicable(),
        trust_review: TrustReviewPosture {
            trust_state_token: "trusted".to_string(),
            profile_or_tenant_scope_ref: "trusted:scope:local-removable".to_string(),
            policy_epoch_ref: "pe:platform:removable-recovery:01".to_string(),
            requested_authority_scope_token: "read_write_scope".to_string(),
            granted_authority_scope_token: "no_scope_granted".to_string(),
            review_required_before_widening: true,
            trust_profile_tenant_checked: true,
            no_silent_authority_widening: true,
        },
        recovery,
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![row.row_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        input,
    )
}

fn network_share_recovery_stable(
    packet: &NativeDesktopContractPacket,
) -> DesktopHandoffConformanceScenario {
    let row = recovery_row(packet, "network_share_unavailable");
    let posture_id = "network_share_recovery_stable";
    let routes = required_recovery_routes(true, false, false);
    let intent = TypedTargetIntent {
        source_surface_token: "system_open".to_string(),
        origin_class_token: "os_shell".to_string(),
        requested_action_class_token: "open_existing_context".to_string(),
        literal_target_label: "\\\\server-share\\app-ts".to_string(),
        literal_target_ref: format!("literal:{}", row.row_id),
        canonical_target_ref: format!("aureline://handoff-target/{posture_id}"),
        target_kind_token: "workspace_root".to_string(),
        resulting_mode_token: row.resulting_mode_token.clone(),
        availability: TargetAvailabilityClass::RemoteUnreachable,
        freshness_class_token: "stale".to_string(),
        deep_link_intent_ref: Some(row.affected_target_ref.clone()),
        preserves_literal_target: true,
        preserves_typed_intent: true,
        preserves_resulting_mode: true,
        no_generic_shell_reopen: true,
    };
    let recovery = TargetRecoveryPosture {
        availability: TargetAvailabilityClass::RemoteUnreachable,
        placeholder_required: true,
        last_seen_identity_ref: Some(row.affected_target_ref.clone()),
        unsaved_local_state_posture_token: "no_unsaved_local_state".to_string(),
        local_work_preserved: row.local_work_preserved,
        recovery_actions: vec![
            HandoffRecoveryAction::OpenBoundTarget,
            HandoffRecoveryAction::LocateTarget,
            HandoffRecoveryAction::OpenCachedContext,
            HandoffRecoveryAction::ClosePlaceholder,
            HandoffRecoveryAction::ReviewHandlerOwnership,
            HandoffRecoveryAction::ExportHandoffSupport,
        ],
        no_silent_replay_or_authority_reuse: row.no_silent_replay_or_authority_reuse,
        canonical_target_path_label: Some("//server-share/app-ts".to_string()),
        write_posture_token: "read_only_cached".to_string(),
        profile_remote_boundary_note: Some(
            "Network share is unreachable; a read-only cached context is offered and live writes \
             are blocked until reconnect."
                .to_string(),
        ),
    };
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "Network share unreachable, cached-context recovery".to_string(),
        title: "Network-share reopen renders a recoverable placeholder".to_string(),
        summary: "An open request pointed at an unreachable network share: the shell rendered a \
                  recoverable placeholder with the last-seen identity and offered Locate / Open \
                  cached context / Close placeholder rather than discarding the original intent."
            .to_string(),
        entry_path: EntryPathClass::NetworkShareReopen,
        intent,
        handler_ownership: ownership(ChannelClass::Stable, "current_user_registered"),
        auth_default: auth_not_applicable(),
        trust_review: TrustReviewPosture {
            trust_state_token: "trusted".to_string(),
            profile_or_tenant_scope_ref: "trusted:scope:network-share".to_string(),
            policy_epoch_ref: "pe:platform:network-share-recovery:01".to_string(),
            requested_authority_scope_token: "read_write_scope".to_string(),
            granted_authority_scope_token: "read_only_scope".to_string(),
            review_required_before_widening: true,
            trust_profile_tenant_checked: true,
            no_silent_authority_widening: true,
        },
        recovery,
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![row.row_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        input,
    )
}

fn device_code_exception_disclosed_stable(
    packet: &NativeDesktopContractPacket,
    page: &SystemBrowserReturnPathsBetaPage,
) -> DesktopHandoffConformanceScenario {
    let event = event(packet, "default_browser_callback");
    let row = auth_row(page, "admin_policy_device_code_required");
    let posture_id = "device_code_exception_disclosed_stable";
    let routes = required_recovery_routes(false, true, false);
    let auth = AuthDefaultPosture {
        applies: true,
        default_to_system_browser: false,
        system_browser_default: false,
        exception_class: SystemBrowserPolicyExceptionClass::AdminPolicyDeviceCodeRequired,
        exception_scope_ref: Some(format!("aureline://auth-scope/{posture_id}")),
        return_path_ref: Some(format!("aureline://auth-return/{posture_id}")),
        return_mode_token: row.return_path_label.return_mode_token.clone(),
        recovery_on_exception_ref: Some(format!("aureline://auth-recovery/{posture_id}")),
        embedded_browser_used: false,
    };
    let recovery = TargetRecoveryPosture {
        availability: TargetAvailabilityClass::AuthRequired,
        placeholder_required: false,
        last_seen_identity_ref: None,
        unsaved_local_state_posture_token: "no_unsaved_local_state".to_string(),
        local_work_preserved: true,
        recovery_actions: vec![
            HandoffRecoveryAction::OpenBoundTarget,
            HandoffRecoveryAction::Reauthenticate,
            HandoffRecoveryAction::ReviewHandlerOwnership,
            HandoffRecoveryAction::ExportHandoffSupport,
        ],
        no_silent_replay_or_authority_reuse: true,
        canonical_target_path_label: None,
        write_posture_token: "local_default".to_string(),
        profile_remote_boundary_note: Some(
            "Admin policy requires device-code on this row; the system-browser default is replaced \
             by an explicitly disclosed exception."
                .to_string(),
        ),
    };
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "Auth exception disclosed: admin device-code required".to_string(),
        title: "Non-system-browser auth path surfaces an explicit exception".to_string(),
        summary: "An admin policy requires device-code auth on this row: the row does not default \
                  to system-browser handoff but surfaces the exception explicitly with its target \
                  scope, return path, and recovery behaviour, so the deviation is visible rather \
                  than silent."
            .to_string(),
        entry_path: EntryPathClass::DefaultBrowserAuthCallback,
        intent: intent_from_event(event, posture_id, TargetAvailabilityClass::AuthRequired),
        handler_ownership: ownership(ChannelClass::AdminManaged, &event.handler_ownership_token),
        auth_default: auth,
        trust_review: trust_enforced(
            event,
            &row.requested_authority_scope_token,
            &row.granted_authority_scope_token,
        ),
        recovery,
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![event.event_id.clone(), row.row_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Stable,
        true,
        LifecycleMarker::Stable,
        input,
    )
}

// ---------------------------------------------------------------------------
// Narrowed scenarios
// ---------------------------------------------------------------------------

fn help_about_preview_surface(
    packet: &NativeDesktopContractPacket,
) -> DesktopHandoffConformanceScenario {
    let event = event(packet, "file_association");
    let posture_id = "help_about_preview_surface";
    let routes = required_recovery_routes(false, false, false);
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "Exact handoff, Help/About surface in preview".to_string(),
        title: "Exact file-association handoff narrowed by a preview Help/About surface".to_string(),
        summary: "The file-association handoff itself is exact and proves every pillar, but the \
                  Help/About binding surface is still in preview, so the posture narrows below \
                  Stable by its lowest binding surface marker rather than inheriting an adjacent \
                  green row."
            .to_string(),
        entry_path: EntryPathClass::FileAssociation,
        intent: intent_from_event(event, posture_id, TargetAvailabilityClass::ExactAvailable),
        handler_ownership: ownership(ChannelClass::Stable, &event.handler_ownership_token),
        auth_default: auth_not_applicable(),
        trust_review: trust_enforced(event, "read_write_scope", "read_write_scope"),
        recovery: exact_recovery(),
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Preview),
        claim_ceiling: full_claim_ceiling(),
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![event.event_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Preview,
        false,
        LifecycleMarker::Preview,
        input,
    )
}

fn last_writer_wins_ownership_drill(
    packet: &NativeDesktopContractPacket,
) -> DesktopHandoffConformanceScenario {
    let event = event(packet, "protocol_handler");
    let posture_id = "last_writer_wins_ownership_drill";
    let routes = required_recovery_routes(false, false, false);
    // A side-by-side Preview install silently re-registered the handler.
    let handler_ownership = HandlerOwnershipDisclosure {
        owning_channel_ref: "aureline://install-channel/preview".to_string(),
        owner_build_ref: "build:aureline:preview:unknown".to_string(),
        owning_channel_class: ChannelClass::Preview,
        handler_ownership_token: "conflict_unknown_owner".to_string(),
        ownership_review_state_token: "denied_conflict".to_string(),
        side_by_side_channels: full_side_by_side(),
        ownership_explicit: false,
        no_last_writer_wins: false,
        spoof_resistant: false,
    };
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "Last-writer-wins handler theft drill".to_string(),
        title: "Side-by-side install silently stole the protocol handler".to_string(),
        summary: "An adversarial drill where a side-by-side Preview install silently re-registered \
                  the protocol handler with an unknown owner. The lane detects the last-writer-wins \
                  ownership and narrows the posture below Stable with a named reason instead of \
                  papering over the conflict."
            .to_string(),
        entry_path: EntryPathClass::ProtocolHandler,
        intent: intent_from_event(event, posture_id, TargetAvailabilityClass::ExactAvailable),
        handler_ownership,
        auth_default: auth_not_applicable(),
        trust_review: trust_enforced(event, "step_up_scope", "step_up_scope"),
        recovery: exact_recovery(),
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        // The claim ceiling must not assert the ownership it cannot prove.
        claim_ceiling: HandoffClaimCeiling {
            asserts_handler_ownership_explicit: false,
            ..full_claim_ceiling()
        },
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![event.event_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Beta,
        false,
        LifecycleMarker::Stable,
        input,
    )
}

fn embedded_browser_no_exception_drill(
    packet: &NativeDesktopContractPacket,
    page: &SystemBrowserReturnPathsBetaPage,
) -> DesktopHandoffConformanceScenario {
    let event = event(packet, "default_browser_callback");
    let row = auth_row(page, "system_browser_default_no_exception");
    let posture_id = "embedded_browser_no_exception_drill";
    let routes = required_recovery_routes(false, true, false);
    // An embedded web view captured the auth return with no disclosed exception.
    let auth = AuthDefaultPosture {
        applies: true,
        default_to_system_browser: false,
        system_browser_default: false,
        exception_class: SystemBrowserPolicyExceptionClass::SystemBrowserDefaultNoException,
        exception_scope_ref: None,
        return_path_ref: None,
        return_mode_token: row.return_path_label.return_mode_token.clone(),
        recovery_on_exception_ref: None,
        embedded_browser_used: true,
    };
    let input = DesktopHandoffConformanceInput {
        record_id: posture_id.to_string(),
        as_of: CORPUS_AS_OF.to_string(),
        posture_id: posture_id.to_string(),
        posture_label: "Embedded-browser auth without a disclosed exception".to_string(),
        title: "Embedded web view captured the auth return without disclosure".to_string(),
        summary: "An adversarial drill where an embedded web view swallowed the auth callback \
                  without defaulting to the system browser and without surfacing an explicit \
                  exception. The lane narrows the posture below Stable with a named reason."
            .to_string(),
        entry_path: EntryPathClass::DefaultBrowserAuthCallback,
        intent: intent_from_event(event, posture_id, TargetAvailabilityClass::ExactAvailable),
        handler_ownership: ownership(ChannelClass::Stable, &event.handler_ownership_token),
        auth_default: auth,
        trust_review: trust_enforced(
            event,
            &row.requested_authority_scope_token,
            &row.granted_authority_scope_token,
        ),
        recovery: exact_recovery(),
        platform_conformance: platform_rows(packet),
        surface_projections: surface_inputs(LifecycleMarker::Stable),
        // The claim ceiling must not assert system-browser-default conformance.
        claim_ceiling: HandoffClaimCeiling {
            asserts_system_browser_default: false,
            ..full_claim_ceiling()
        },
        recovery_routes: routes.clone(),
        routes: entry_routes(posture_id),
        accessibility: accessibility(&routes),
        available_without_account: true,
        available_without_managed_services: true,
        upstream: upstream(packet, vec![event.event_id.clone(), row.row_id.clone()]),
        diagnostics_export_ref: DIAGNOSTICS_EXPORT_REF.to_string(),
        support_export_ref: SUPPORT_EXPORT_REF.to_string(),
        evidence_refs: evidence_refs(),
        narrative_refs: vec![NARRATIVE_REF.to_string()],
    };
    finish(
        posture_id,
        StableClaimClass::Beta,
        false,
        LifecycleMarker::Stable,
        input,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corpus_builds_and_spans_stable_and_narrowed() {
        let corpus = desktop_handoff_conformance_corpus();
        assert_eq!(corpus.len(), 14);
        let stable = corpus
            .iter()
            .filter(|s| s.expected_claim_class == StableClaimClass::Stable)
            .count();
        assert!(stable >= 8, "matrix must include several Stable rows");
        assert!(stable < corpus.len(), "matrix must include narrowed rows");
    }

    #[test]
    fn corpus_covers_every_entry_path() {
        let corpus = desktop_handoff_conformance_corpus();
        for required in EntryPathClass::REQUIRED {
            assert!(
                corpus.iter().any(|s| s.expected_entry_path == required),
                "matrix must cover entry path {}",
                required.as_str()
            );
        }
    }

    #[test]
    fn removable_and_network_recovery_render_placeholders() {
        for scenario_id in ["removable_volume_recovery_stable", "network_share_recovery_stable"] {
            let scenario = desktop_handoff_conformance_corpus()
                .into_iter()
                .find(|s| s.scenario_id == scenario_id)
                .expect("scenario present");
            let record = scenario.record();
            assert!(record.recovery.placeholder_required);
            assert!(record.recovery.last_seen_identity_ref.is_some());
            assert!(record.pillars.recovery_truthful);
        }
    }
}
