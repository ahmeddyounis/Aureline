// Sequential pushes keep each contract scenario adjacent to its rationale.
#![allow(clippy::vec_init_then_push)]

//! Canonical seed builders for the M5 affected-install assessment panel primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical affected-install-panel-primitive packet.
pub const M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_PACKET_ID: &str =
    "m5-affected-install-panel-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds one worked resolution case from a fully specified affected-install input.
#[allow(clippy::too_many_arguments)]
fn assessment_case(
    install_profile: M5InstallProfileLane,
    advisory_id: &str,
    severity: M5AdvisorySeverityClass,
    affected_object_repr: &str,
    build_identity_repr: &str,
    impacted_components_repr: &str,
    install_state: M5AdvisoryInstallState,
    mirror_freshness: M5AdvisoryFreshnessState,
    delivery_profile: M5AdvisoryDeliveryProfile,
    fixed_build_or_mitigation_repr: &str,
    signer_source_state_repr: &str,
    action_state: M5AdvisoryActionState,
    primary_action: M5AdvisoryRequiredAction,
    help_action: M5AdvisoryRequiredAction,
    continuity_claim: M5AdvisoryContinuityClaim,
) -> M5AffectedInstallResolutionCase {
    M5AffectedInstallResolutionCase::resolved(M5AffectedInstallResolutionInput {
        install_profile,
        advisory_id: advisory_id.to_owned(),
        severity,
        affected_object_repr: affected_object_repr.to_owned(),
        build_identity_repr: build_identity_repr.to_owned(),
        impacted_components_repr: impacted_components_repr.to_owned(),
        install_state,
        mirror_freshness,
        delivery_profile,
        fixed_build_or_mitigation_repr: fixed_build_or_mitigation_repr.to_owned(),
        signer_source_state_repr: signer_source_state_repr.to_owned(),
        action_state,
        primary_action,
        help_action,
        continuity_claim,
    })
}

/// A base row with the shared fields filled in and the full anatomy, severity, channel,
/// action, continuity, delivery, freshness, focus, export, and accessibility parity
/// every lane carries. Parity is the guarantee: every lane renders the same
/// affected-install panel model.
fn base_row(
    install_profile: M5InstallProfileLane,
    qualification: M5AdvisoryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_assessments: Vec<M5AffectedInstallResolutionCase>,
) -> M5InstallProfileRow {
    M5InstallProfileRow {
        install_profile,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        // The affected-install panel lives in the right inspector: the detail zone
        // where "am I affected?" is answered against the local install graph.
        shell_zone_slot: M5ShellZoneSlot::RightInspector,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        anatomy_parts: M5AffectedInstallAnatomyPart::ALL.to_vec(),
        severity_classes: M5AdvisorySeverityClass::ALL.to_vec(),
        channels: M5AffectedInstallChannel::ALL.to_vec(),
        action_states: M5AdvisoryActionState::ALL.to_vec(),
        required_actions: M5AdvisoryRequiredAction::ALL.to_vec(),
        continuity_claims: M5AdvisoryContinuityClaim::ALL.to_vec(),
        delivery_profiles: M5AdvisoryDeliveryProfile::ALL.to_vec(),
        freshness_states: M5AdvisoryFreshnessState::ALL.to_vec(),
        focus_behaviors: M5AffectedInstallFocusBehavior::ALL.to_vec(),
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
            M5AdvisoryDowngradeTrigger::ExposureHiddenBehindGenericBanner,
            M5AdvisoryDowngradeTrigger::LocalContinuityHidden,
            M5AdvisoryDowngradeTrigger::MirrorLagUndisclosed,
            M5AdvisoryDowngradeTrigger::StaleNoticeStateSilent,
            M5AdvisoryDowngradeTrigger::ExternalDisclosureOnly,
            M5AdvisoryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_AFFECTED_INSTALL_PANEL_SCHEMA_REF,
            M5_AFFECTED_INSTALL_PANEL_ASSESSMENT_REF,
            M5_AFFECTED_INSTALL_PANEL_INSTALL_ROW_REF,
            M5_AFFECTED_INSTALL_PANEL_IDENTITY_DOC_REF,
        ]),
        example_assessments,
        hides_field_behind_detail_drawer: false,
        degrades_to_generic_update_prompt: false,
        requires_external_website_lookup: false,
        stale_mirror_stays_silently_green: false,
        drops_copy_safe_id_or_export: false,
    }
}

fn install_rows() -> Vec<M5InstallProfileRow> {
    use M5AdvisoryActionState as A;
    use M5AdvisoryContinuityClaim as C;
    use M5AdvisoryDeliveryProfile as D;
    use M5AdvisoryFreshnessState as F;
    use M5AdvisoryInstallState as I;
    use M5AdvisoryRequiredAction as R;
    use M5AdvisorySeverityClass as S;

    let mut rows = Vec::with_capacity(6);

    // 1. Per-user installed — critical: the running per-user build is installed and
    //    active against a critical advisory. The verdict is `affected`, resolved
    //    against the local install graph with no website lookup; the rollback / repin
    //    action and a support-export action stay attached to the panel.
    rows.push(base_row(
        M5InstallProfileLane::PerUserInstalled,
        M5AdvisoryQualificationClass::Stable,
        "Update / install-topology owner",
        "The per-user-installed lane answers `am I affected?` against the local install graph so a critical advisory on the running build shows `affected`, the exact build/channel identity, the impacted components, `up_to_date` mirror freshness, and an attached rollback/repin plus support-export action — no external website lookup",
        "evidence:m5-affected-install-per-user:001",
        vec![assessment_case(
            M5InstallProfileLane::PerUserInstalled,
            "AURELINE-ADV-2026-0301",
            S::Critical,
            "artifact:aureline-desktop",
            "build_identity:2026.6.0+stable.peruser",
            "impacted_components:renderer-core|extension-host",
            I::InstalledActive,
            F::UpToDate,
            D::LocalOnly,
            "fixed_build:2026.6.1+stable",
            "signer_source_state:signed_current",
            A::ImmediateRemediation,
            R::RollbackOrRepin,
            R::ExportSupportPacket,
            C::NoSafeLocalContinuity,
        )],
    ));

    // 2. Per-machine installed — high: the per-machine build is installed and awaiting
    //    a rollback / repin. The verdict is `awaiting_rollback_or_repin`; the primary
    //    action is the rollback/repin itself, proving the rollback action is attached.
    rows.push(base_row(
        M5InstallProfileLane::PerMachineInstalled,
        M5AdvisoryQualificationClass::Stable,
        "Managed install / rollback owner",
        "The per-machine-installed lane shows `awaiting_rollback_or_repin` when the machine-wide build is pinned to a fixed rollback target, keeping the rollback/repin and support-export actions attached to the same panel instead of scattering them across separate surfaces",
        "evidence:m5-affected-install-per-machine:001",
        vec![assessment_case(
            M5InstallProfileLane::PerMachineInstalled,
            "AURELINE-ADV-2026-0302",
            S::High,
            "artifact:aureline-desktop",
            "build_identity:2026.6.0+stable.permachine",
            "impacted_components:updater-service",
            I::InstalledAwaitingRollback,
            F::UpToDate,
            D::Managed,
            "fixed_build:2026.6.1+stable",
            "signer_source_state:signed_current",
            A::ActionRequired,
            R::RollbackOrRepin,
            R::ExportSupportPacket,
            C::ContinuityPendingFix,
        )],
    ));

    // 3. Portable — moderate: the portable build has a mitigation applied in place. The
    //    verdict is `mitigated_no_action_needed`; local work continues in a degraded
    //    but safe mode.
    rows.push(base_row(
        M5InstallProfileLane::Portable,
        M5AdvisoryQualificationClass::Stable,
        "Portable-distribution owner",
        "The portable lane shows `mitigated_no_action_needed` when a compensating control is applied in place, keeping the install identity, impacted components, and mirror freshness visible in the same assessment surface",
        "evidence:m5-affected-install-portable:001",
        vec![assessment_case(
            M5InstallProfileLane::Portable,
            "AURELINE-ADV-2026-0303",
            S::Moderate,
            "artifact:aureline-portable",
            "build_identity:2026.6.0+portable_stable",
            "impacted_components:renderer-core",
            I::InstalledMitigated,
            F::UpToDate,
            D::ManualImport,
            "mitigation:disable_affected_renderer_flag",
            "signer_source_state:signed_current",
            A::MitigationComplete,
            R::WaitForSupersedingAction,
            R::ExportSupportPacket,
            C::DegradedLocalMode,
        )],
    ));

    // 4. Managed deployed — operational emergency: the admin-deployed build is blocked
    //    by revocation. The verdict is `contained_action_advised`; the help action is a
    //    contact-admin action, proving a help action stays attached to the panel.
    rows.push(base_row(
        M5InstallProfileLane::ManagedDeployed,
        M5AdvisoryQualificationClass::Stable,
        "Managed-deployment owner",
        "The managed-deployed lane shows `contained_action_advised` when the admin-deployed build is blocked by revocation, keeping a disable action and an attached contact-admin help action on the panel while local continuity requires disabling the affected profile",
        "evidence:m5-affected-install-managed:001",
        vec![assessment_case(
            M5InstallProfileLane::ManagedDeployed,
            "AURELINE-ADV-2026-0304",
            S::OperationalEmergency,
            "artifact:aureline-managed",
            "build_identity:2026.6.0+stable.managed",
            "impacted_components:extension-host|policy-engine",
            I::InstalledBlocked,
            F::UpToDate,
            D::Managed,
            "fixed_build:2026.6.1+stable",
            "signer_source_state:signed_current",
            A::Blocking,
            R::DisableOrRemove,
            R::ContactAdmin,
            C::RequiresDisablingAffectedProfile,
        )],
    ));

    // 5. Offline bundle — two worked assessments. The first is an offline-bundle lane
    //    whose mirror snapshot is expired: a would-be-clean verdict auto-narrows to
    //    `clean_pending_mirror_refresh` so mirror lag never stays silently green. The
    //    second is a resolved lane superseded by a fixed build over a fresh mirror.
    rows.push(base_row(
        M5InstallProfileLane::OfflineBundle,
        M5AdvisoryQualificationClass::Stable,
        "Offline / mirror-continuity owner",
        "The offline-bundle lane auto-narrows a clean verdict to `clean_pending_mirror_refresh` when the offline snapshot is expired — mirror lag is disclosed instead of staying silently green — and shows `resolved` once a fixed build supersedes the advisory over a fresh mirror",
        "evidence:m5-affected-install-offline-bundle:001",
        vec![
            assessment_case(
                M5InstallProfileLane::OfflineBundle,
                "AURELINE-ADV-2026-0305",
                S::Informational,
                "artifact:aureline-offline",
                "build_identity:2026.5.0+offline_bundle",
                "impacted_components:none_confirmed_offline",
                I::NotInstalled,
                F::OfflineExpired,
                D::OfflineMirror,
                "mitigation:refresh_offline_mirror_snapshot",
                "signer_source_state:signed_snapshot_expired",
                A::ReviewRecommended,
                R::ReviewNotice,
                R::ExportSupportPacket,
                C::OfflineMirrorLagDisclosed,
            ),
            assessment_case(
                M5InstallProfileLane::OfflineBundle,
                "AURELINE-ADV-2026-0306",
                S::Low,
                "artifact:aureline-offline",
                "build_identity:2026.6.1+offline_bundle",
                "impacted_components:superseded_by_fixed_build",
                I::Superseded,
                F::UpToDate,
                D::OfflineMirror,
                "fixed_build:2026.6.1+offline_bundle",
                "signer_source_state:signed_snapshot_imported",
                A::MitigationComplete,
                R::None,
                R::ExportSupportPacket,
                C::LocalUseUnaffected,
            ),
        ],
    ));

    // 6. Side-by-side preview — low: the preview build is not installed on this device,
    //    so the verdict is `not_affected` over a fresh mirror; local use is unaffected.
    rows.push(base_row(
        M5InstallProfileLane::SideBySidePreview,
        M5AdvisoryQualificationClass::Stable,
        "Preview / side-by-side owner",
        "The side-by-side-preview lane shows `not_affected` when the preview build is not installed on this device over a fresh mirror, so the assessment states local use is unaffected without a generic update prompt",
        "evidence:m5-affected-install-side-by-side:001",
        vec![assessment_case(
            M5InstallProfileLane::SideBySidePreview,
            "AURELINE-ADV-2026-0307",
            S::Low,
            "artifact:aureline-preview",
            "build_identity:2026.7.0+preview.sxs",
            "impacted_components:not_installed_on_device",
            I::NotInstalled,
            F::UpToDate,
            D::ManualImport,
            "mitigation:not_applicable_not_installed",
            "signer_source_state:signed_current",
            A::Informational,
            R::ReviewNotice,
            R::ExportSupportPacket,
            C::LocalUseUnaffected,
        )],
    ));

    rows
}

fn governance_review() -> M5AffectedInstallGovernanceReview {
    M5AffectedInstallGovernanceReview {
        one_panel_model_across_install_profiles: true,
        identity_components_exposure_visible_without_drawer: true,
        resolves_against_local_install_graph: true,
        mirror_freshness_and_install_mode_visible: true,
        stale_mirror_auto_narrows_clean_verdict: true,
        rollback_repin_help_actions_attached: true,
        copy_safe_advisory_id_preserved: true,
        export_summary_reconstructs_assessment_truth: true,
        every_row_bound_to_shell_zone: true,
        every_row_declares_accessibility_route: true,
        later_lanes_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5AffectedInstallConsumerProjection {
    M5AffectedInstallConsumerProjection {
        update_center_renders_shared_panel: true,
        help_about_renders_shared_panel: true,
        support_bundle_renders_shared_panel: true,
        admin_report_reads_single_source: true,
        resolver_reads_single_install_vocabulary: true,
    }
}

fn proof_freshness() -> M5AffectedInstallProofFreshness {
    M5AffectedInstallProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AffectedInstallReleasePosture {
    M5AffectedInstallReleasePosture {
        release_packet_ref: M5_AFFECTED_INSTALL_PANEL_ARTIFACT_REF.to_owned(),
        affected_install_audit_ref: M5_AFFECTED_INSTALL_PANEL_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_AFFECTED_INSTALL_PANEL_SCHEMA_REF,
        M5_AFFECTED_INSTALL_PANEL_DOC_REF,
        M5_AFFECTED_INSTALL_PANEL_SHELL_ZONE_REF,
        M5_AFFECTED_INSTALL_PANEL_COMPONENT_MATRIX_REF,
        M5_AFFECTED_INSTALL_PANEL_ASSESSMENT_REF,
        M5_AFFECTED_INSTALL_PANEL_INSTALL_ROW_REF,
        M5_AFFECTED_INSTALL_PANEL_IDENTITY_DOC_REF,
    ])
}

/// Builds the canonical M5 affected-install-panel-primitive packet.
pub fn seeded_m5_affected_install_panel_primitive_packet() -> M5AffectedInstallPanelPacket {
    M5AffectedInstallPanelPacket::new(M5AffectedInstallPanelPacketInput {
        packet_id: M5_AFFECTED_INSTALL_PANEL_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 affected-install assessment panel primitive: build / channel / install-mode identity, impacted components, current exposure, mitigation status, mirror freshness, and rollback / repin / help-action parity across channels"
                .to_owned(),
        install_rows: install_rows(),
        vocabulary_set: M5AffectedInstallVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the managed-deployed lane is held at Beta because a slice of the
/// managed-deployment continuity note does not yet render on every profile; every lane
/// stays visible.
pub fn seeded_m5_affected_install_panel_primitive_managed_deployed_beta_narrowed(
) -> M5AffectedInstallPanelPacket {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.packet_id = "m5-affected-install-panel-primitive:managed-deployed-beta:0001".to_owned();
    let row = packet
        .install_rows
        .iter_mut()
        .find(|row| row.install_profile == M5InstallProfileLane::ManagedDeployed)
        .expect("managed-deployed row present");
    row.qualification = M5AdvisoryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the offline-bundle lane is narrowed to Preview pending
/// mirror-freshness parity across every offline profile; every lane stays visible.
pub fn seeded_m5_affected_install_panel_primitive_offline_bundle_preview_narrowed(
) -> M5AffectedInstallPanelPacket {
    let mut packet = seeded_m5_affected_install_panel_primitive_packet();
    packet.packet_id = "m5-affected-install-panel-primitive:offline-bundle-preview:0001".to_owned();
    let row = packet
        .install_rows
        .iter_mut()
        .find(|row| row.install_profile == M5InstallProfileLane::OfflineBundle)
        .expect("offline-bundle row present");
    row.qualification = M5AdvisoryQualificationClass::Preview;
    packet
}
