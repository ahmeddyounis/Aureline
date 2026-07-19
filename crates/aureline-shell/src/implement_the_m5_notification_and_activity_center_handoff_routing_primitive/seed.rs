// Sequential pushes keep each contract scenario adjacent to its rationale.
#![allow(clippy::vec_init_then_push)]

//! Canonical seed builders for the M5 notification / activity-center handoff routing
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical notification-activity-handoff-primitive packet.
pub const M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_PACKET_ID: &str =
    "m5-notification-activity-handoff-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds one worked resolution case from a fully specified notification-handoff input.
#[allow(clippy::too_many_arguments)]
fn handoff_case(
    delivery_lane: M5NotificationDeliveryLane,
    advisory_id: &str,
    severity: M5AdvisorySeverityClass,
    event_kind: M5NotificationEventKind,
    affected_scope_repr: &str,
    current_status_repr: &str,
    authoritative_surface: M5NotificationReopenSurface,
    reopen_target_repr: &str,
    signer_source_state_repr: &str,
    delivery_profile: M5AdvisoryDeliveryProfile,
    mirror_freshness: M5AdvisoryFreshnessState,
    action_state: M5AdvisoryActionState,
    primary_action: M5AdvisoryRequiredAction,
    continuity_claim: M5AdvisoryContinuityClaim,
) -> M5NotificationHandoffResolutionCase {
    M5NotificationHandoffResolutionCase::resolved(M5NotificationHandoffResolutionInput {
        delivery_lane,
        advisory_id: advisory_id.to_owned(),
        severity,
        event_kind,
        affected_scope_repr: affected_scope_repr.to_owned(),
        current_status_repr: current_status_repr.to_owned(),
        authoritative_surface,
        reopen_target_repr: reopen_target_repr.to_owned(),
        signer_source_state_repr: signer_source_state_repr.to_owned(),
        delivery_profile,
        mirror_freshness,
        action_state,
        primary_action,
        continuity_claim,
    })
}

/// A base row with the shared fields filled in and the full anatomy, severity, channel,
/// action, continuity, delivery, freshness, notification-behavior, event-kind, focus,
/// export, and accessibility parity every lane carries. Parity is the guarantee: every
/// lane renders the same notification / activity handoff model.
fn base_row(
    delivery_lane: M5NotificationDeliveryLane,
    qualification: M5AdvisoryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_handoffs: Vec<M5NotificationHandoffResolutionCase>,
) -> M5NotificationDeliveryRow {
    M5NotificationDeliveryRow {
        delivery_lane,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        // The notification / activity handoff is anchored in the status bar: the ambient
        // attention surface that expands into the durable activity center where the event
        // stays reopenable instead of collapsing to a badge.
        shell_zone_slot: M5ShellZoneSlot::StatusBar,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        anatomy_parts: M5NotificationHandoffAnatomyPart::ALL.to_vec(),
        severity_classes: M5AdvisorySeverityClass::ALL.to_vec(),
        channels: M5NotificationChannel::ALL.to_vec(),
        action_states: M5AdvisoryActionState::ALL.to_vec(),
        required_actions: M5AdvisoryRequiredAction::ALL.to_vec(),
        continuity_claims: M5AdvisoryContinuityClaim::ALL.to_vec(),
        delivery_profiles: M5AdvisoryDeliveryProfile::ALL.to_vec(),
        freshness_states: M5AdvisoryFreshnessState::ALL.to_vec(),
        notification_behaviors: M5AdvisoryNotificationBehavior::ALL.to_vec(),
        event_kinds: M5NotificationEventKind::ALL.to_vec(),
        focus_behaviors: M5NotificationFocusBehavior::ALL.to_vec(),
        export_fields: M5AdvisoryExportField::ALL.to_vec(),
        accessibility_routes: M5AdvisoryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::StatusBar,
            M5ShellConsumerSurface::AttentionRouter,
            M5ShellConsumerSurface::NotificationEnvelope,
            M5ShellConsumerSurface::DocsHelp,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ReleaseProof,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5AdvisoryDowngradeTrigger::AffectedScopeHidden,
            M5AdvisoryDowngradeTrigger::ExposureHiddenBehindGenericBanner,
            M5AdvisoryDowngradeTrigger::LocalContinuityHidden,
            M5AdvisoryDowngradeTrigger::DismissalRuleViolated,
            M5AdvisoryDowngradeTrigger::StaleNoticeStateSilent,
            M5AdvisoryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_NOTIFICATION_ACTIVITY_HANDOFF_SCHEMA_REF,
            M5_NOTIFICATION_ACTIVITY_HANDOFF_IDENTITY_REF,
            M5_NOTIFICATION_ACTIVITY_HANDOFF_OS_NOTIFICATION_DOC_REF,
            M5_NOTIFICATION_ACTIVITY_HANDOFF_ATTENTION_ROUTING_REF,
        ]),
        example_handoffs,
        collapses_to_badge_toast_or_website_only: false,
        hides_field_behind_detail_drawer: false,
        drops_event_from_durable_history: false,
        splits_notification_and_activity_vocabulary: false,
        drops_copy_safe_id_or_export: false,
    }
}

fn delivery_rows() -> Vec<M5NotificationDeliveryRow> {
    use M5AdvisoryActionState as A;
    use M5AdvisoryContinuityClaim as C;
    use M5AdvisoryDeliveryProfile as D;
    use M5AdvisoryFreshnessState as F;
    use M5AdvisoryRequiredAction as R;
    use M5AdvisorySeverityClass as S;
    use M5NotificationEventKind as E;
    use M5NotificationReopenSurface as U;

    let mut rows = Vec::with_capacity(6);

    // 1. Foreground focused — critical, newly published: the app is focused, so the event
    //    delivers a native notification plus a durable activity row and reopens onto the
    //    affected-install panel.
    rows.push(base_row(
        M5NotificationDeliveryLane::ForegroundFocused,
        M5AdvisoryQualificationClass::Stable,
        "Activity-center / notification owner",
        "The foreground-focused lane routes a published critical advisory to a native notification plus a durable activity row that reopens onto the affected-install panel, sharing one advisory id and severity across both",
        "evidence:m5-notification-foreground-focused:001",
        vec![handoff_case(
            M5NotificationDeliveryLane::ForegroundFocused,
            "AURELINE-ADV-2026-0501",
            S::Critical,
            E::AdvisoryPublished,
            "affected_scope:desktop_app_2026.6.0",
            "current_status:published_action_required",
            U::AffectedInstallPanel,
            "reopen_target:affected_install_panel_deeplink",
            "signer_source_state:first_party_signed_current",
            D::LocalOnly,
            F::UpToDate,
            A::ActionRequired,
            R::UpdateToFixedVersion,
            C::DegradedLocalMode,
        )],
    ));

    // 2. Background unfocused — high, revocation: the app is backgrounded; a revocation
    //    event still delivers a native notification plus a durable activity row and reopens
    //    onto the disclosure block.
    rows.push(base_row(
        M5NotificationDeliveryLane::BackgroundUnfocused,
        M5AdvisoryQualificationClass::Stable,
        "Revocation / attention-routing owner",
        "The background-unfocused lane routes a high-severity revocation to a native notification plus a durable activity row that reopens onto the disclosure block, never collapsing to a bare badge",
        "evidence:m5-notification-background-unfocused:001",
        vec![handoff_case(
            M5NotificationDeliveryLane::BackgroundUnfocused,
            "AURELINE-ADV-2026-0502",
            S::High,
            E::AdvisoryRevoked,
            "affected_scope:extension_signing_key_2026.5",
            "current_status:revoked_disable_required",
            U::DisclosureBlock,
            "reopen_target:disclosure_block_deeplink",
            "signer_source_state:revocation_signed_current",
            D::Managed,
            F::UpToDate,
            A::Blocking,
            R::DisableOrRemove,
            C::RequiresDisablingAffectedProfile,
        )],
    ));

    // 3. Quiet hours active — moderate, mitigation available: quiet hours suppress the OS
    //    notification for a non-emergency event, but it still lands durably in the activity
    //    center — never badge-only.
    rows.push(base_row(
        M5NotificationDeliveryLane::QuietHoursActive,
        M5AdvisoryQualificationClass::Stable,
        "Quiet-hours / durability owner",
        "The quiet-hours lane suppresses the OS notification for a moderate mitigation-available event but keeps it durable in the activity center, reopening onto the advisory card instead of collapsing to a badge",
        "evidence:m5-notification-quiet-hours:001",
        vec![handoff_case(
            M5NotificationDeliveryLane::QuietHoursActive,
            "AURELINE-ADV-2026-0503",
            S::Moderate,
            E::MitigationAvailable,
            "affected_scope:desktop_app_2026.4.0",
            "current_status:mitigation_available_update_recommended",
            U::AdvisoryCard,
            "reopen_target:advisory_card_deeplink",
            "signer_source_state:first_party_signed_current",
            D::LocalOnly,
            F::UpToDate,
            A::ReviewRecommended,
            R::UpdateToFixedVersion,
            C::LocalUseUnaffected,
        )],
    ));

    // 4. Do-not-disturb enforced — operational emergency: an emergency-grade event bypasses
    //    do-not-disturb and is delivered, reopening onto the emergency notice.
    rows.push(base_row(
        M5NotificationDeliveryLane::DoNotDisturbEnforced,
        M5AdvisoryQualificationClass::Stable,
        "Emergency-notice routing owner",
        "The do-not-disturb lane still delivers an operational-emergency event by bypassing the suppression and reopens onto the emergency notice, keeping the blast radius and continuity visible",
        "evidence:m5-notification-do-not-disturb:001",
        vec![handoff_case(
            M5NotificationDeliveryLane::DoNotDisturbEnforced,
            "AURELINE-ADV-2026-0504",
            S::OperationalEmergency,
            E::EmergencyNotice,
            "affected_scope:managed_service_all_channels",
            "current_status:emergency_immediate_remediation",
            U::EmergencyNotice,
            "reopen_target:emergency_notice_deeplink",
            "signer_source_state:emergency_bundle_signed_current",
            D::Managed,
            F::UpToDate,
            A::ImmediateRemediation,
            R::RollbackOrRepin,
            C::NoSafeLocalContinuity,
        )],
    ));

    // 5. Offline / mirror deferred — low, resolved: delivery is deferred behind an offline
    //    / mirror lag, then lands durably, reopening onto the disclosure block.
    rows.push(base_row(
        M5NotificationDeliveryLane::OfflineOrMirrorDeferred,
        M5AdvisoryQualificationClass::Stable,
        "Mirror / offline-continuity owner",
        "The offline / mirror-deferred lane defers a low-severity resolved event behind a mirror lag then lands it durably in the activity center, disclosing the freshness and reopening onto the disclosure block",
        "evidence:m5-notification-offline-deferred:001",
        vec![handoff_case(
            M5NotificationDeliveryLane::OfflineOrMirrorDeferred,
            "AURELINE-ADV-2026-0505",
            S::Low,
            E::AdvisoryResolved,
            "affected_scope:offline_bundle_2026.3.0",
            "current_status:resolved_fixed_build_promoted",
            U::DisclosureBlock,
            "reopen_target:disclosure_block_deeplink",
            "signer_source_state:mirror_signed_within_grace",
            D::OfflineMirror,
            F::StaleWithinGrace,
            A::MitigationComplete,
            R::WaitForSupersedingAction,
            C::OfflineMirrorLagDisclosed,
        )],
    ));

    // 6. Managed policy restricted — informational, updated: a managed policy restricts the
    //    OS notification for a non-emergency event, but it still lands durably in the
    //    activity center and reopens onto the affected-install panel.
    rows.push(base_row(
        M5NotificationDeliveryLane::ManagedPolicyRestricted,
        M5AdvisoryQualificationClass::Stable,
        "Managed-deployment / admin owner",
        "The managed-policy-restricted lane keeps an informational advisory-updated event durable in the activity center when OS notifications are policy-restricted, reopening onto the affected-install panel",
        "evidence:m5-notification-managed-policy:001",
        vec![handoff_case(
            M5NotificationDeliveryLane::ManagedPolicyRestricted,
            "AURELINE-ADV-2026-0506",
            S::Informational,
            E::AdvisoryUpdated,
            "affected_scope:managed_deployed_2026.2.0",
            "current_status:updated_review_recommended",
            U::AffectedInstallPanel,
            "reopen_target:affected_install_panel_deeplink",
            "signer_source_state:managed_policy_signed_current",
            D::Managed,
            F::Unknown,
            A::Informational,
            R::ReviewNotice,
            C::ContinuityPendingFix,
        )],
    ));

    rows
}

fn governance_review() -> M5NotificationHandoffGovernanceReview {
    M5NotificationHandoffGovernanceReview {
        one_handoff_model_across_delivery_lanes: true,
        event_identity_severity_scope_visible_without_drawer: true,
        native_notification_and_activity_row_share_vocabulary: true,
        never_collapses_to_badge_toast_or_website_only: true,
        suppressed_os_notification_still_lands_durably: true,
        emergency_severity_bypasses_quiet_hours: true,
        reopen_lands_on_authoritative_surface: true,
        no_dead_end_reopen: true,
        privacy_safe_no_sensitive_body_in_payload: true,
        export_summary_reconstructs_advisory_truth: true,
        every_row_bound_to_shell_zone: true,
        every_row_declares_accessibility_route: true,
        later_lanes_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5NotificationHandoffConsumerProjection {
    M5NotificationHandoffConsumerProjection {
        activity_center_renders_shared_handoff: true,
        native_notification_renders_shared_handoff: true,
        help_about_renders_shared_handoff: true,
        support_bundle_renders_shared_handoff: true,
        resolver_reads_single_notification_vocabulary: true,
    }
}

fn proof_freshness() -> M5NotificationHandoffProofFreshness {
    M5NotificationHandoffProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5NotificationHandoffReleasePosture {
    M5NotificationHandoffReleasePosture {
        release_packet_ref: M5_NOTIFICATION_ACTIVITY_HANDOFF_ARTIFACT_REF.to_owned(),
        notification_audit_ref: M5_NOTIFICATION_ACTIVITY_HANDOFF_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_NOTIFICATION_ACTIVITY_HANDOFF_SCHEMA_REF,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_DOC_REF,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_SHELL_ZONE_REF,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_COMPONENT_MATRIX_REF,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_IDENTITY_REF,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_OS_NOTIFICATION_DOC_REF,
        M5_NOTIFICATION_ACTIVITY_HANDOFF_ATTENTION_ROUTING_REF,
    ])
}

/// Builds the canonical M5 notification-activity-handoff-primitive packet.
pub fn seeded_m5_notification_activity_handoff_primitive_packet() -> M5NotificationHandoffPacket {
    M5NotificationHandoffPacket::new(M5NotificationHandoffPacketInput {
        packet_id: M5_NOTIFICATION_ACTIVITY_HANDOFF_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 notification / activity-center handoff routing primitive: durable, reopenable advisory and revocation routing that never collapses to badge-only, toast-only, or website-only across activity-center, native-notification, Help/About, and support-bundle channels"
                .to_owned(),
        delivery_rows: delivery_rows(),
        vocabulary_set: M5NotificationHandoffVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the quiet-hours lane is held at Beta because a slice of the
/// suppressed-durable projection does not yet render on every quiet-hours profile; every
/// lane stays visible.
pub fn seeded_m5_notification_activity_handoff_primitive_quiet_hours_beta_narrowed(
) -> M5NotificationHandoffPacket {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.packet_id =
        "m5-notification-activity-handoff-primitive:quiet-hours-beta:0001".to_owned();
    let row = packet
        .delivery_rows
        .iter_mut()
        .find(|row| row.delivery_lane == M5NotificationDeliveryLane::QuietHoursActive)
        .expect("quiet-hours row present");
    row.qualification = M5AdvisoryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the offline / mirror-deferred lane is narrowed to Preview pending
/// deferred-then-durable parity across every offline profile; every lane stays visible.
pub fn seeded_m5_notification_activity_handoff_primitive_offline_deferred_preview_narrowed(
) -> M5NotificationHandoffPacket {
    let mut packet = seeded_m5_notification_activity_handoff_primitive_packet();
    packet.packet_id =
        "m5-notification-activity-handoff-primitive:offline-deferred-preview:0001".to_owned();
    let row = packet
        .delivery_rows
        .iter_mut()
        .find(|row| row.delivery_lane == M5NotificationDeliveryLane::OfflineOrMirrorDeferred)
        .expect("offline-deferred row present");
    row.qualification = M5AdvisoryQualificationClass::Preview;
    packet
}
