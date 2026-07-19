// Sequential pushes keep each contract scenario adjacent to its rationale.
#![allow(clippy::vec_init_then_push)]

//! Canonical seed builders for the M5 emergency-notice banner primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical emergency-notice-banner-primitive packet.
pub const M5_EMERGENCY_BANNER_PRIMITIVE_PACKET_ID: &str =
    "m5-emergency-notice-banner-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds one worked resolution case from a fully specified emergency input.
#[allow(clippy::too_many_arguments)]
fn emergency_case(
    reason_class: M5EmergencyReasonClass,
    notice_id: &str,
    severity: M5AdvisorySeverityClass,
    affected_capability_repr: &str,
    blast_radius_repr: &str,
    local_work_state: M5EmergencyLocalWorkState,
    deadline_repr: &str,
    recovery_repr: &str,
    signer_source_state_repr: &str,
    action_state: M5AdvisoryActionState,
    primary_action: M5AdvisoryRequiredAction,
    recovery_action: M5AdvisoryRequiredAction,
    continuity_claim: M5AdvisoryContinuityClaim,
    dismissal_policy: M5EmergencyDismissalPolicy,
) -> M5EmergencyBannerResolutionCase {
    M5EmergencyBannerResolutionCase::resolved(M5EmergencyBannerResolutionInput {
        reason_class,
        notice_id: notice_id.to_owned(),
        severity,
        affected_capability_repr: affected_capability_repr.to_owned(),
        blast_radius_repr: blast_radius_repr.to_owned(),
        local_work_state,
        deadline_repr: deadline_repr.to_owned(),
        recovery_repr: recovery_repr.to_owned(),
        signer_source_state_repr: signer_source_state_repr.to_owned(),
        action_state,
        primary_action,
        recovery_action,
        continuity_claim,
        dismissal_policy,
    })
}

/// A base row with the shared fields filled in and the full anatomy, severity,
/// channel, action, continuity, dismissal-policy, focus, export, and accessibility
/// parity every lane carries. Parity is the guarantee: every lane renders the same
/// emergency-banner model.
fn base_row(
    reason_class: M5EmergencyReasonClass,
    qualification: M5AdvisoryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_notices: Vec<M5EmergencyBannerResolutionCase>,
) -> M5EmergencyReasonRow {
    M5EmergencyReasonRow {
        reason_class,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        // Emergency banners live in the title / context bar: the top-of-window zone
        // where a kill switch, trust-root rotation, channel freeze, or forced-disable
        // changes what is safe to do next.
        shell_zone_slot: M5ShellZoneSlot::TitleContextBar,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        anatomy_parts: M5EmergencyBannerAnatomyPart::ALL.to_vec(),
        severity_classes: M5AdvisorySeverityClass::ALL.to_vec(),
        channels: M5EmergencyBannerChannel::ALL.to_vec(),
        action_states: M5AdvisoryActionState::ALL.to_vec(),
        required_actions: M5AdvisoryRequiredAction::ALL.to_vec(),
        continuity_claims: M5AdvisoryContinuityClaim::ALL.to_vec(),
        dismissal_policies: M5EmergencyDismissalPolicy::ALL.to_vec(),
        focus_behaviors: M5EmergencyBannerFocusBehavior::ALL.to_vec(),
        export_fields: M5AdvisoryExportField::ALL.to_vec(),
        accessibility_routes: M5AdvisoryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::Layout,
            M5ShellConsumerSurface::AttentionRouter,
            M5ShellConsumerSurface::NotificationEnvelope,
            M5ShellConsumerSurface::DocsHelp,
            M5ShellConsumerSurface::SupportExport,
            M5ShellConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5AdvisoryDowngradeTrigger::AffectedScopeHidden,
            M5AdvisoryDowngradeTrigger::LocalContinuityHidden,
            M5AdvisoryDowngradeTrigger::DismissalRuleViolated,
            M5AdvisoryDowngradeTrigger::ForcedDisableScopeHidden,
            M5AdvisoryDowngradeTrigger::StaleNoticeStateSilent,
            M5AdvisoryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_EMERGENCY_BANNER_SCHEMA_REF,
            M5_EMERGENCY_BANNER_EMERGENCY_ACTION_REF,
            M5_EMERGENCY_BANNER_DISABLE_BUNDLE_REF,
            M5_EMERGENCY_BANNER_LOCAL_CONTINUITY_REF,
        ]),
        example_notices,
        hides_field_behind_detail_drawer: false,
        implies_data_loss_without_proof: false,
        collapses_to_single_generic_dismiss: false,
        drops_copy_safe_id_or_export: false,
    }
}

fn reason_rows() -> Vec<M5EmergencyReasonRow> {
    use M5AdvisoryActionState as A;
    use M5AdvisoryContinuityClaim as C;
    use M5AdvisoryRequiredAction as R;
    use M5AdvisorySeverityClass as S;
    use M5EmergencyDismissalPolicy as P;
    use M5EmergencyLocalWorkState as L;

    let mut rows = Vec::with_capacity(5);

    // 1. Capability kill switch — an operational emergency: a compromised extension
    //    capability is killed. The affected capability is suspended, but editing,
    //    review, and export continue safely. The banner is blocked until remediated —
    //    acknowledge only, no snooze, no dismiss.
    rows.push(base_row(
        M5EmergencyReasonClass::CapabilityKillSwitch,
        M5AdvisoryQualificationClass::Stable,
        "Extension trust owner",
        "The kill-switch lane renders the shared emergency banner so a compromised extension capability shows `capability_kill_switch`, the affected capability, the single-capability blast radius, `affected_capability_suspended_local_safe` continuity, and a must-acknowledge (blocked-until-remediated) dismissal rule — editing, review, and export stay safe, and there is no generic close button",
        "evidence:m5-emergency-banner-kill-switch:001",
        vec![emergency_case(
            M5EmergencyReasonClass::CapabilityKillSwitch,
            "AURELINE-EMG-2026-0201",
            S::OperationalEmergency,
            "extension:code-lens:network-capability",
            "blast_radius:single_extension_capability",
            L::AffectedCapabilitySuspended,
            "deadline:acknowledge_within_24h",
            "recovery:await_signed_replacement",
            "signer_source_state:signed_current",
            A::ImmediateRemediation,
            R::DisableOrRemove,
            R::WaitForSupersedingAction,
            C::RequiresDisablingAffectedProfile,
            P::NotDismissableBlocked,
        )],
    ));

    // 2. Trust-root rotation — critical: the trust root rotated with a continuity
    //    statement. Updates are blocked pending acknowledgement of the new root, but
    //    local files stay safe. Acknowledgement is required.
    rows.push(base_row(
        M5EmergencyReasonClass::TrustRootRotation,
        M5AdvisoryQualificationClass::Stable,
        "Signing / trust-root owner",
        "The trust-root-rotation lane renders the shared emergency banner so a rotated trust root shows `trust_root_rotation`, the new-and-previous signer continuity, the all-signed-updates blast radius, `blocked_pending_acknowledgement` continuity, and an acknowledge-required dismissal rule while local files stay safe",
        "evidence:m5-emergency-banner-trust-root:001",
        vec![emergency_case(
            M5EmergencyReasonClass::TrustRootRotation,
            "AURELINE-EMG-2026-0202",
            S::Critical,
            "signing:trust-root",
            "blast_radius:all_signed_updates_blocked",
            L::BlockedPendingAcknowledgement,
            "deadline:acknowledge_before_next_update",
            "recovery:import_signed_snapshot",
            "signer_source_state:rotated_with_continuity_statement",
            A::ActionRequired,
            R::RotateTrustRoot,
            R::ImportSignedSnapshot,
            C::ContinuityPendingFix,
            P::AcknowledgeRequired,
        )],
    ));

    // 3. Channel freeze — high: the stable update channel is frozen. Local work
    //    continues in a degraded but safe mode. The banner may be acknowledged or
    //    snoozed until a scheduled review.
    rows.push(base_row(
        M5EmergencyReasonClass::ChannelFreeze,
        M5AdvisoryQualificationClass::Stable,
        "Update / release channel owner",
        "The channel-freeze lane renders the shared emergency banner so a frozen stable channel shows `channel_freeze`, the paused-updates blast radius, `local_work_continues_degraded` continuity, and an acknowledge-or-snooze dismissal rule — no update, but everything local still works",
        "evidence:m5-emergency-banner-channel-freeze:001",
        vec![emergency_case(
            M5EmergencyReasonClass::ChannelFreeze,
            "AURELINE-EMG-2026-0203",
            S::High,
            "update:stable-channel",
            "blast_radius:release_channel_updates_paused",
            L::DegradedButSafe,
            "deadline:no_hard_deadline",
            "recovery:wait_for_superseding_release",
            "signer_source_state:signed_current",
            A::ReviewRecommended,
            R::WaitForSupersedingAction,
            R::ExportSupportPacket,
            C::DegradedLocalMode,
            P::AcknowledgeOrSnooze,
        )],
    ));

    // 4. Forced disable — low: a deprecated extension is forcibly disabled. Editing,
    //    review, and export all continue safely; local use is unaffected. This is the
    //    clean local-safe proof — the banner never implies data loss. Acknowledge is
    //    required so the user knows the capability changed.
    rows.push(base_row(
        M5EmergencyReasonClass::ForcedDisable,
        M5AdvisoryQualificationClass::Stable,
        "Extension governance owner",
        "The forced-disable lane renders the shared emergency banner so a forcibly disabled deprecated extension shows `forced_disable`, the single-extension blast radius, `local_work_continues_safely` continuity, and an acknowledge-required dismissal rule — editing, review, and export continue safely and the banner never implies data loss",
        "evidence:m5-emergency-banner-forced-disable:001",
        vec![emergency_case(
            M5EmergencyReasonClass::ForcedDisable,
            "AURELINE-EMG-2026-0204",
            S::Low,
            "extension:legacy-formatter",
            "blast_radius:single_deprecated_extension",
            L::EditingReviewExportSafe,
            "deadline:no_hard_deadline",
            "recovery:update_to_supported_extension",
            "signer_source_state:signed_current",
            A::ReviewRecommended,
            R::DisableOrRemove,
            R::UpdateToFixedVersion,
            C::LocalUseUnaffected,
            P::AcknowledgeRequired,
        )],
    ));

    // 5. Signed emergency bundle — two worked emergencies. The first is an
    //    informational managed-service notice whose continuity assessment is still
    //    pending and which is freely dismissible. The second is the only worked
    //    emergency where the signed bundle confirms a specific, localized data-loss
    //    event — and only there does the banner state data loss.
    rows.push(base_row(
        M5EmergencyReasonClass::SignedEmergencyBundle,
        M5AdvisoryQualificationClass::Stable,
        "Managed emergency-distribution owner",
        "The signed-emergency-bundle lane renders the shared emergency banner so an informational managed notice reads `continuity_assessment_pending` and is dismissible, while a signed bundle that confirms a localized cache-corruption event reads `data_loss_proven` — data loss is stated only when the event actually proves it",
        "evidence:m5-emergency-banner-signed-bundle:001",
        vec![
            emergency_case(
                M5EmergencyReasonClass::SignedEmergencyBundle,
                "AURELINE-EMG-2026-0205",
                S::Informational,
                "managed-service:sync-relay",
                "blast_radius:managed_service_notice_only",
                L::ContinuityNotYetDetermined,
                "deadline:review_at_convenience",
                "recovery:review_signed_bundle",
                "signer_source_state:managed_signed_current",
                A::Informational,
                R::ReviewNotice,
                R::None,
                C::OfflineMirrorLagDisclosed,
                P::InformationalDismissible,
            ),
            emergency_case(
                M5EmergencyReasonClass::SignedEmergencyBundle,
                "AURELINE-EMG-2026-0206",
                S::Moderate,
                "remote-helper:build-agent-cache",
                "blast_radius:localized_cache_corruption",
                L::DataLossConfirmed,
                "deadline:import_snapshot_now",
                "recovery:import_signed_snapshot",
                "signer_source_state:signed_snapshot_imported",
                A::ImmediateRemediation,
                R::ImportSignedSnapshot,
                R::RollbackOrRepin,
                C::NoSafeLocalContinuity,
                P::FullyDismissible,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5EmergencyBannerGovernanceReview {
    M5EmergencyBannerGovernanceReview {
        one_banner_model_across_reason_classes: true,
        reason_scope_continuity_deadline_visible_without_drawer: true,
        never_implies_data_loss_without_proof: true,
        local_safe_continuity_preserved: true,
        dismissal_rules_match_event_class: true,
        copy_safe_notice_id_preserved: true,
        export_summary_reconstructs_emergency_truth: true,
        every_row_bound_to_shell_zone: true,
        every_row_declares_accessibility_route: true,
        later_lanes_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5EmergencyBannerConsumerProjection {
    M5EmergencyBannerConsumerProjection {
        update_center_renders_shared_banner: true,
        extension_host_renders_shared_banner: true,
        native_notification_renders_shared_banner: true,
        support_export_reads_single_source: true,
        resolver_reads_single_emergency_vocabulary: true,
    }
}

fn proof_freshness() -> M5EmergencyBannerProofFreshness {
    M5EmergencyBannerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5EmergencyBannerReleasePosture {
    M5EmergencyBannerReleasePosture {
        release_packet_ref: M5_EMERGENCY_BANNER_ARTIFACT_REF.to_owned(),
        emergency_banner_audit_ref: M5_EMERGENCY_BANNER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_EMERGENCY_BANNER_SCHEMA_REF,
        M5_EMERGENCY_BANNER_DOC_REF,
        M5_EMERGENCY_BANNER_SHELL_ZONE_REF,
        M5_EMERGENCY_BANNER_COMPONENT_MATRIX_REF,
        M5_EMERGENCY_BANNER_EMERGENCY_ACTION_REF,
        M5_EMERGENCY_BANNER_DISABLE_BUNDLE_REF,
        M5_EMERGENCY_BANNER_LOCAL_CONTINUITY_REF,
    ])
}

/// Builds the canonical M5 emergency-notice-banner-primitive packet.
pub fn seeded_m5_emergency_notice_banner_primitive_packet() -> M5EmergencyBannerPrimitivePacket {
    M5EmergencyBannerPrimitivePacket::new(M5EmergencyBannerPrimitivePacketInput {
        packet_id: M5_EMERGENCY_BANNER_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 emergency-notice banner primitive: reason class, affected capability, blast radius, local-work continuity, deadline / urgency, primary / recovery actions, and dismissal-rule parity across channels"
                .to_owned(),
        reason_rows: reason_rows(),
        vocabulary_set: M5EmergencyBannerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the forced-disable lane is held at Beta because a slice of the
/// forced-disable continuity note does not yet render on every profile; every lane
/// stays visible.
pub fn seeded_m5_emergency_notice_banner_primitive_forced_disable_beta_narrowed(
) -> M5EmergencyBannerPrimitivePacket {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.packet_id = "m5-emergency-notice-banner-primitive:forced-disable-beta:0001".to_owned();
    let row = packet
        .reason_rows
        .iter_mut()
        .find(|row| row.reason_class == M5EmergencyReasonClass::ForcedDisable)
        .expect("forced-disable row present");
    row.qualification = M5AdvisoryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the signed-emergency-bundle lane is narrowed to Preview pending
/// mirror-freshness parity across every offline profile; every lane stays visible.
pub fn seeded_m5_emergency_notice_banner_primitive_signed_emergency_bundle_preview_narrowed(
) -> M5EmergencyBannerPrimitivePacket {
    let mut packet = seeded_m5_emergency_notice_banner_primitive_packet();
    packet.packet_id =
        "m5-emergency-notice-banner-primitive:signed-emergency-bundle-preview:0001".to_owned();
    let row = packet
        .reason_rows
        .iter_mut()
        .find(|row| row.reason_class == M5EmergencyReasonClass::SignedEmergencyBundle)
        .expect("signed-emergency-bundle row present");
    row.qualification = M5AdvisoryQualificationClass::Preview;
    packet
}
