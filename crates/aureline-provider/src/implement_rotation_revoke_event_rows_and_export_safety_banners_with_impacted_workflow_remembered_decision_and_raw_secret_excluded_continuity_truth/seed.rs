//! Canonical seed builders for the rotation/revoke / export-safety controls.
//!
//! These builders are the single producer of the checked-in support export and the
//! scenario fixtures. The headless emitter and the inline tests both call them so the
//! in-code controls, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical rotation/revoke / export-safety packet.
pub const ROTATION_REVOKE_EXPORT_SAFETY_PACKET_ID: &str =
    "m5-rotation-revoke-export-safety-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn event_row_source_refs() -> Vec<String> {
    strings(&[
        M5_ROTATION_REVOKE_EVENT_ROW_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_SECRET_HANDLE_REF,
    ])
}

fn banner_source_refs() -> Vec<String> {
    strings(&[
        M5_EXPORT_SAFETY_BANNER_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_FOUNDATION_EXPORT_REDACTION_REF,
    ])
}

/// Builds a rotation/revoke-event row, deriving the continuity class, the still-usable
/// claim, and the required notes from the honest inputs so the seed is always
/// self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn event_row(
    row_id: &str,
    credential_class: M5CredentialClass,
    credential_id_label: &str,
    prior_state: M5CredentialLifecycleState,
    new_state: M5CredentialLifecycleState,
    impacted: ImpactedWorkflowClass,
) -> RotationRevokeEventRow {
    let disclosure = resolve_credential_continuity(new_state);
    RotationRevokeEventRow {
        component: M5CredentialComponentFamily::RotationRevokeEventRow,
        row_id: row_id.to_owned(),
        credential_class,
        credential_id_label: credential_id_label.to_owned(),
        prior_state,
        new_state,
        prior_state_note: format!("Prior state before this event: {}", prior_state.as_str()),
        new_state_note: format!("New state after this event: {}", new_state.as_str()),
        continuity_class: disclosure.continuity_class,
        claims_still_usable: disclosure.is_still_usable,
        still_active_note: if disclosure.needs_still_active_note {
            "Still active: this credential remains usable after the event".to_owned()
        } else {
            String::new()
        },
        action_required_note: if disclosure.needs_action_required_note {
            "Action required: this credential still works but needs a refresh or rotation soon"
                .to_owned()
        } else {
            String::new()
        },
        no_longer_usable_note: if disclosure.needs_no_longer_usable_note {
            "No longer usable: this credential is revoked or expired and can no longer be used"
                .to_owned()
        } else {
            String::new()
        },
        superseded_note: if disclosure.needs_superseded_note {
            "Superseded: a newer credential has replaced this one; stop relying on it".to_owned()
        } else {
            String::new()
        },
        impacted_workflows: vec![impacted],
        impacted_workflows_note: format!(
            "Impacted workflows: affects {} (running sessions, queued jobs, and remembered decisions are listed here)",
            impacted.as_str()
        ),
        recovery_next_step_note:
            "Recovery next step: re-authenticate or rotate, then resume the affected workflows"
                .to_owned(),
        audit_note: "Audit: this rotation / revoke event is recorded in the credential audit trail"
            .to_owned(),
        default_actions: RotationRevokeEventRowAction::ALL.to_vec(),
        degraded_states: M5CredentialDegradedState::ALL.to_vec(),
        required_labels: M5CredentialRequiredLabel::ALL.to_vec(),
        surface_families: M5CredentialSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CredentialDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CredentialAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CredentialConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "credential_class",
            "prior_state",
            "new_state",
            "continuity_class",
            "impacted_workflows",
            "recovery_next_step",
            "audit",
        ]),
        source_contract_refs: event_row_source_refs(),
        masks_impacted_workflows: false,
        implies_raw_secret_exportable: false,
        uses_friendly_connected_wording: false,
    }
}

/// Builds an export-safety banner, deriving the export-safety posture and the required
/// notes from the honest inputs so the seed is always self-consistent with the resolver.
fn export_banner(
    banner_id: &str,
    export_surface_class: ExportSurfaceClass,
    export_safety_class: M5CredentialExportSafetyClass,
    reveal_posture: M5CredentialRevealPosture,
) -> ExportSafetyBanner {
    let disclosure = resolve_export_safety_posture(export_safety_class);
    ExportSafetyBanner {
        component: M5CredentialComponentFamily::ExportSafetyBanner,
        banner_id: banner_id.to_owned(),
        export_surface_class,
        export_surface_note: format!(
            "Export surface: this banner governs the {} export",
            export_surface_class.as_str()
        ),
        export_safety_class,
        reveal_posture,
        export_safety_posture: disclosure.export_safety_posture,
        claims_preserves_handle_labels: disclosure.preserves_handle_class_labels,
        raw_secret_excluded_note:
            "Raw credentials are excluded by default from profiles, support bundles, handoff packets, recipes, and portable workspace exports"
                .to_owned(),
        handle_label_note: if disclosure.needs_handle_label_note {
            "Preserved: handle-class and source labels are kept so the reference stays readable"
                .to_owned()
        } else {
            String::new()
        },
        redaction_note: if disclosure.needs_redaction_note {
            "Redacted: a redacted or endpoint-masked share is exported, never the raw value"
                .to_owned()
        } else {
            String::new()
        },
        blocked_note: if disclosure.needs_blocked_note {
            "Blocked: export is blocked entirely for this class; not even a handle is exported"
                .to_owned()
        } else {
            String::new()
        },
        preserved_labels_note: format!(
            "Preserved labels: class {} keeps {} labels where allowed",
            export_safety_class.as_str(),
            if disclosure.preserves_handle_class_labels {
                "handle-class and source"
            } else {
                "no"
            }
        ),
        reveal_posture_note: format!("Reveal posture behind this export: {}", reveal_posture.as_str()),
        default_actions: ExportSafetyBannerAction::ALL.to_vec(),
        degraded_states: M5CredentialDegradedState::ALL.to_vec(),
        required_labels: M5CredentialRequiredLabel::ALL.to_vec(),
        surface_families: M5CredentialSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5CredentialDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5CredentialAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5CredentialConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "export_surface",
            "export_safety_class",
            "export_safety_posture",
            "raw_secret_excluded",
            "preserved_labels",
            "reveal_posture",
        ]),
        source_contract_refs: banner_source_refs(),
        implies_raw_secret_exportable: false,
        leaves_exclusion_to_implication: false,
        uses_friendly_connected_wording: false,
    }
}

fn event_rows() -> Vec<RotationRevokeEventRow> {
    use ImpactedWorkflowClass as Impact;
    use M5CredentialClass as Class;
    use M5CredentialLifecycleState as Lifecycle;

    vec![
        // 1. Rotation completed to a fresh, active credential; sessions pick up the new one.
        event_row(
            "event-rotated-active",
            Class::OauthToken,
            "GitHub OAuth token (acme-org)",
            Lifecycle::RotationDue,
            Lifecycle::ActiveCurrent,
            Impact::RunningSession,
        ),
        // 2. A refresh is now needed; a queued job depends on it.
        event_row(
            "event-refresh-needed",
            Class::ApiKey,
            "npm registry API key (mirror)",
            Lifecycle::ActiveCurrent,
            Lifecycle::RefreshNeeded,
            Impact::QueuedJob,
        ),
        // 3. A rotation is now due; a scheduled automation depends on it.
        event_row(
            "event-rotation-due",
            Class::PersonalAccessToken,
            "GitLab PAT (internal)",
            Lifecycle::ActiveCurrent,
            Lifecycle::RotationDue,
            Impact::ScheduledAutomation,
        ),
        // 4. Credential revoked; a remembered decision referenced it.
        event_row(
            "event-revoked",
            Class::SshOrSigningKey,
            "Release signing key (platform-team)",
            Lifecycle::ActiveCurrent,
            Lifecycle::Revoked,
            Impact::RememberedDecision,
        ),
        // 5. Credential expired; a delegated forward relied on it.
        event_row(
            "event-expired",
            Class::ClientCertificate,
            "Client certificate (contoso)",
            Lifecycle::RefreshNeeded,
            Lifecycle::Expired,
            Impact::DelegatedForward,
        ),
        // 6. Credential superseded by a newer one; nothing active is affected.
        event_row(
            "event-superseded",
            Class::DeviceCodeGrant,
            "Device-code grant (self)",
            Lifecycle::ActiveCurrent,
            Lifecycle::Superseded,
            Impact::NoActiveImpact,
        ),
    ]
}

fn export_banners() -> Vec<ExportSafetyBanner> {
    use ExportSurfaceClass as Surface;
    use M5CredentialExportSafetyClass as Safety;
    use M5CredentialRevealPosture as Reveal;

    vec![
        // 1. Profile export: raw secret excluded, handle-class / source labels preserved.
        export_banner(
            "banner-profile",
            Surface::Profile,
            Safety::RawSecretExcluded,
            Reveal::HandleOnly,
        ),
        // 2. Support bundle: metadata only, labels preserved.
        export_banner(
            "banner-support-bundle",
            Surface::SupportBundle,
            Safety::MetadataOnly,
            Reveal::MaskedLastFour,
        ),
        // 3. Handoff packet: only a handle reference is exported.
        export_banner(
            "banner-handoff-packet",
            Surface::HandoffPacket,
            Safety::HandleReferenceOnly,
            Reveal::NeverRevealed,
        ),
        // 4. Recipe: a redacted share is exported.
        export_banner(
            "banner-recipe",
            Surface::Recipe,
            Safety::RedactedShare,
            Reveal::ClipboardScoped,
        ),
        // 5. Portable workspace: endpoints masked in export.
        export_banner(
            "banner-portable-workspace",
            Surface::PortableWorkspace,
            Safety::EndpointsMasked,
            Reveal::PolicyBlockedReveal,
        ),
        // 6. Audit log: export blocked entirely.
        export_banner(
            "banner-audit-log",
            Surface::AuditLog,
            Safety::ExportBlocked,
            Reveal::RevealOnDemand,
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5CredentialDowngradeTrigger> {
    vec![
        M5CredentialDowngradeTrigger::LifecycleStateHidden,
        M5CredentialDowngradeTrigger::ExportSafetyBoundaryHidden,
        M5CredentialDowngradeTrigger::RevealPostureUnstated,
        M5CredentialDowngradeTrigger::CredentialClassUnstated,
        M5CredentialDowngradeTrigger::AlternateStateLabelInvented,
        M5CredentialDowngradeTrigger::FriendlyConnectedWordingUsed,
        M5CredentialDowngradeTrigger::SessionOnlyFallbackHidden,
        M5CredentialDowngradeTrigger::ProofStale,
    ]
}

fn trust_review() -> RotationRevokeExportSafetyTrustReview {
    RotationRevokeExportSafetyTrustReview {
        row_shows_credential_class_and_prior_new_state: true,
        rotation_revoke_impacted_workflows_always_shown: true,
        running_sessions_queued_jobs_remembered_decisions_stay_distinct: true,
        recovery_next_step_always_shown: true,
        revoked_expired_never_reads_as_still_usable: true,
        audit_and_export_actions_present: true,
        banner_states_raw_secret_excluded_by_default: true,
        export_exclusion_never_left_to_implication: true,
        handle_class_and_source_labels_preserved_where_allowed: true,
        export_surface_named_across_all_surfaces: true,
        reveal_posture_always_shown: true,
        raw_secret_handling_never_normalized: true,
        no_friendly_connected_wording: true,
        controls_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> RotationRevokeExportSafetyConsumerProjection {
    RotationRevokeExportSafetyConsumerProjection {
        row_shows_impacted_workflows_and_recovery_without_docs: true,
        revoked_expired_state_visible_before_reuse: true,
        banner_shows_exclusion_posture_inline: true,
        cli_headless_shows_control_truth: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> RotationRevokeExportSafetyProofFreshness {
    RotationRevokeExportSafetyProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_REF,
        ROTATION_REVOKE_EXPORT_SAFETY_DOC_REF,
        M5_CREDENTIAL_COMPONENT_SCHEMA_REF,
        M5_CREDENTIAL_COMPONENT_DOC_REF,
        M5_ROTATION_REVOKE_EVENT_ROW_SCHEMA_REF,
        M5_EXPORT_SAFETY_BANNER_SCHEMA_REF,
    ])
}

/// Builds the canonical rotation/revoke / export-safety controls packet.
pub fn seeded_rotation_revoke_export_safety_controls() -> RotationRevokeExportSafetyControlsPacket {
    RotationRevokeExportSafetyControlsPacket::new(RotationRevokeExportSafetyControlsPacketInput {
        packet_id: ROTATION_REVOKE_EXPORT_SAFETY_PACKET_ID.to_owned(),
        surface_label:
            "M5 rotation/revoke-event rows and export-safety banners: credential class, prior/new lifecycle state, derived continuity, impacted running sessions/queued jobs/remembered decisions, recovery next step, audit/export actions, export surface, export-safety class, reveal posture, derived redaction posture, preserved handle-class/source labels, and the raw-secret-excluded default"
                .to_owned(),
        event_rows: event_rows(),
        export_banners: export_banners(),
        downgrade_triggers: downgrade_triggers(),
        consumer_surfaces: M5CredentialConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Scenario fixture: spotlights a revoke event whose impacted running sessions, queued
/// jobs, and remembered decisions must stay explicit and never read as still usable. Every
/// lifecycle state, continuity class, and impacted-workflow class stays covered so the
/// fixture validates on its own.
pub fn seeded_rotation_revoke_export_safety_controls_revoke_event_impacted_workflows(
) -> RotationRevokeExportSafetyControlsPacket {
    let mut packet = seeded_rotation_revoke_export_safety_controls();
    packet.packet_id =
        "m5-rotation-revoke-export-safety-controls:fixture:revoke-event-impacted-workflows"
            .to_owned();
    packet.surface_label =
        "M5 rotation/revoke-event rows: a revoke names its impacted workflows and never reads as still usable"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights an export-safety banner that must state raw credentials are
/// excluded by default and never leave exclusion to implication. Every export-safety class,
/// posture, and export surface stays covered so the fixture validates on its own.
pub fn seeded_rotation_revoke_export_safety_controls_export_banner_raw_excluded(
) -> RotationRevokeExportSafetyControlsPacket {
    let mut packet = seeded_rotation_revoke_export_safety_controls();
    packet.packet_id =
        "m5-rotation-revoke-export-safety-controls:fixture:export-banner-raw-excluded".to_owned();
    packet.surface_label =
        "M5 export-safety banners: raw credentials are excluded by default and never left to implication"
            .to_owned();
    packet
}
