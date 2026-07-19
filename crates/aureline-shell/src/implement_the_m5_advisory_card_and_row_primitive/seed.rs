// Sequential pushes keep each contract scenario adjacent to its rationale.
#![allow(clippy::vec_init_then_push)]

//! Canonical seed builders for the M5 security-advisory card / row primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never
//! drift.

use super::*;

/// Stable packet id for the canonical advisory-card-row-primitive packet.
pub const M5_ADVISORY_ROW_PRIMITIVE_PACKET_ID: &str = "m5-advisory-card-row-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds one worked resolution case from a fully specified advisory input.
#[allow(clippy::too_many_arguments)]
fn advisory_case(
    affected_surface: M5AffectedSurfaceLane,
    advisory_id: &str,
    severity: M5AdvisorySeverityClass,
    affected_object_repr: &str,
    install_state: M5AdvisoryInstallState,
    fixed_version_or_mitigation_repr: &str,
    signer_source_state_repr: &str,
    action_state: M5AdvisoryActionState,
    primary_action: M5AdvisoryRequiredAction,
    continuity_claim: M5AdvisoryContinuityClaim,
) -> M5AdvisoryRowResolutionCase {
    M5AdvisoryRowResolutionCase::resolved(M5AdvisoryRowResolutionInput {
        affected_surface,
        advisory_id: advisory_id.to_owned(),
        severity,
        affected_object_repr: affected_object_repr.to_owned(),
        install_state,
        fixed_version_or_mitigation_repr: fixed_version_or_mitigation_repr.to_owned(),
        signer_source_state_repr: signer_source_state_repr.to_owned(),
        action_state,
        primary_action,
        continuity_claim,
    })
}

/// A base row with the shared fields filled in and the full anatomy, severity,
/// channel, action, continuity, focus, export, and accessibility parity every lane
/// carries. Parity is the guarantee: every lane renders the same advisory-row model.
fn base_row(
    affected_surface: M5AffectedSurfaceLane,
    qualification: M5AdvisoryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    example_advisories: Vec<M5AdvisoryRowResolutionCase>,
) -> M5AdvisorySurfaceRow {
    M5AdvisorySurfaceRow {
        affected_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        // Advisory cards and rows live in the activity rail: the activity-center zone
        // where security advisories, revocations, and update history surface.
        shell_zone_slot: M5ShellZoneSlot::ActivityRail,
        responsive_classes: M5ResponsiveClass::ALL.to_vec(),
        window_classes: M5WindowClass::ALL.to_vec(),
        anatomy_parts: M5AdvisoryRowAnatomyPart::ALL.to_vec(),
        severity_classes: M5AdvisorySeverityClass::ALL.to_vec(),
        channels: M5AdvisoryRowChannel::ALL.to_vec(),
        action_states: M5AdvisoryActionState::ALL.to_vec(),
        required_actions: M5AdvisoryRequiredAction::ALL.to_vec(),
        continuity_claims: M5AdvisoryContinuityClaim::ALL.to_vec(),
        focus_behaviors: M5AdvisoryRowFocusBehavior::ALL.to_vec(),
        export_fields: M5AdvisoryExportField::ALL.to_vec(),
        accessibility_routes: M5AdvisoryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ShellConsumerSurface::ShellFrame,
            M5ShellConsumerSurface::Layout,
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
            M5AdvisoryDowngradeTrigger::UnsignedDistributionUndisclosed,
            M5AdvisoryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_ADVISORY_ROW_SCHEMA_REF,
            M5_ADVISORY_ROW_ADVISORY_CARD_REF,
            M5_ADVISORY_ROW_AFFECTED_INSTALL_REF,
        ]),
        example_advisories,
        hides_field_behind_detail_drawer: false,
        disappears_when_installed_but_affected: false,
        degrades_to_generic_update_prompt: false,
        drops_copy_safe_id_or_export: false,
    }
}

fn surface_rows() -> Vec<M5AdvisorySurfaceRow> {
    use M5AdvisoryActionState as A;
    use M5AdvisoryContinuityClaim as C;
    use M5AdvisoryInstallState as I;
    use M5AdvisoryRequiredAction as R;
    use M5AdvisorySeverityClass as S;

    let mut rows = Vec::with_capacity(6);

    // 1. Desktop app — a critical runtime vulnerability, installed and exposed right
    //    now. The row names the fix and the primary action inline; exposure reads as
    //    `exposed`.
    rows.push(base_row(
        M5AffectedSurfaceLane::DesktopApp,
        M5AdvisoryQualificationClass::Stable,
        "Desktop app security owner",
        "The desktop-app lane renders the shared advisory row so a critical, installed-and-exposed runtime vulnerability shows severity, affected object, `exposed` state, the fixed version, the signer state, and `update_to_fixed_version` inline — no detail drawer, no generic update banner",
        "evidence:m5-advisory-row-desktop:001",
        vec![advisory_case(
            M5AffectedSurfaceLane::DesktopApp,
            "AURELINE-ADV-2026-0101",
            S::Critical,
            "desktop-app:core-runtime",
            I::InstalledActive,
            "fixed-in-2.4.1",
            "signer_source_state:signed_current",
            A::ImmediateRemediation,
            R::UpdateToFixedVersion,
            C::RequiresDisablingAffectedProfile,
        )],
    ));

    // 2. Extension — a high-severity issue whose affected extension is blocked. The
    //    row stays visible and reads `contained_by_block`; it does not vanish or
    //    degrade to a generic update prompt just because the extension is blocked.
    rows.push(base_row(
        M5AffectedSurfaceLane::Extension,
        M5AdvisoryQualificationClass::Stable,
        "Marketplace / extension trust owner",
        "The extension lane renders the shared advisory row so a high-severity, installed-but-blocked extension keeps its row, reads `contained_by_block`, discloses the unsigned distribution, and offers `disable_or_remove` — the installed-but-affected item never disappears",
        "evidence:m5-advisory-row-extension:001",
        vec![advisory_case(
            M5AffectedSurfaceLane::Extension,
            "AURELINE-ADV-2026-0102",
            S::High,
            "extension:code-lens",
            I::InstalledBlocked,
            "mitigation-disable-affected-extension",
            "signer_source_state:unsigned_distribution_disclosed",
            A::Blocking,
            R::DisableOrRemove,
            C::DegradedLocalMode,
        )],
    ));

    // 3. Remote helper — a moderate issue whose helper is awaiting rollback. The row
    //    stays visible and reads `awaiting_rollback` with `rollback_or_repin`.
    rows.push(base_row(
        M5AffectedSurfaceLane::RemoteHelper,
        M5AdvisoryQualificationClass::Stable,
        "Remote-connector trust owner",
        "The remote-helper lane renders the shared advisory row so a moderate, installed-and-awaiting-rollback helper keeps its row, reads `awaiting_rollback`, discloses the mirror lag, and offers `rollback_or_repin` while local continuity is pending the fix",
        "evidence:m5-advisory-row-remote:001",
        vec![advisory_case(
            M5AffectedSurfaceLane::RemoteHelper,
            "AURELINE-ADV-2026-0103",
            S::Moderate,
            "remote-helper:build-agent",
            I::InstalledAwaitingRollback,
            "rollback-to-1.9.4",
            "signer_source_state:mirror_behind_disclosed",
            A::ActionRequired,
            R::RollbackOrRepin,
            C::ContinuityPendingFix,
        )],
    ));

    // 4. Managed service — an operational emergency whose service is disabled. The row
    //    stays visible and reads `contained_by_disable`; the primary action routes to
    //    an administrator, and there is no safe local continuity.
    rows.push(base_row(
        M5AffectedSurfaceLane::ManagedService,
        M5AdvisoryQualificationClass::Stable,
        "Managed-service governance owner",
        "The managed-service lane renders the shared advisory row so an operational-emergency, installed-but-disabled service keeps its row, reads `contained_by_disable`, states `no_safe_local_continuity`, and routes to `contact_admin` instead of a generic update prompt",
        "evidence:m5-advisory-row-managed:001",
        vec![advisory_case(
            M5AffectedSurfaceLane::ManagedService,
            "AURELINE-ADV-2026-0104",
            S::OperationalEmergency,
            "managed-service:sync-relay",
            I::InstalledDisabled,
            "mitigation-await-managed-rollout",
            "signer_source_state:managed_signed_current",
            A::ImmediateRemediation,
            R::ContactAdmin,
            C::NoSafeLocalContinuity,
        )],
    ));

    // 5. Docs artifact — a low-severity issue already superseded by a fixed handbook.
    //    The row reads `resolved`, mitigation is complete, and local use is
    //    unaffected. Kept as history, never hidden.
    rows.push(base_row(
        M5AffectedSurfaceLane::DocsArtifact,
        M5AdvisoryQualificationClass::Stable,
        "Docs / knowledge integrity owner",
        "The docs-artifact lane renders the shared advisory row so a low-severity, superseded advisory reads `resolved` with `mitigation_complete`, keeps a signed-snapshot signer state, and states local use is unaffected — resolved advisories stay visible as history",
        "evidence:m5-advisory-row-docs:001",
        vec![advisory_case(
            M5AffectedSurfaceLane::DocsArtifact,
            "AURELINE-ADV-2026-0105",
            S::Low,
            "docs-artifact:signed-handbook",
            I::Superseded,
            "fixed-in-handbook-2026.06",
            "signer_source_state:signed_snapshot_imported",
            A::MitigationComplete,
            R::None,
            C::LocalUseUnaffected,
        )],
    ));

    // 6. Signing / update path — two worked advisories. The first is informational and
    //    not installed on this device (`not_affected`); the second is a moderate issue
    //    mitigated in place, disclosing offline-mirror lag. Together they exercise the
    //    not-affected and mitigated-in-place exposure states.
    rows.push(base_row(
        M5AffectedSurfaceLane::SigningUpdatePath,
        M5AdvisoryQualificationClass::Stable,
        "Signing / update path owner",
        "The signing-update-path lane renders the shared advisory row so an informational, not-installed advisory reads `not_affected` with a review action, and a moderate, mitigated-in-place advisory reads `mitigated_in_place` while disclosing offline-mirror lag and offering a support-packet export",
        "evidence:m5-advisory-row-signing:001",
        vec![
            advisory_case(
                M5AffectedSurfaceLane::SigningUpdatePath,
                "AURELINE-ADV-2026-0106",
                S::Informational,
                "signing-update-path:release-channel",
                I::NotInstalled,
                "fixed-in-2.4.1",
                "signer_source_state:signed_current",
                A::Informational,
                R::ReviewNotice,
                C::LocalUseUnaffected,
            ),
            advisory_case(
                M5AffectedSurfaceLane::SigningUpdatePath,
                "AURELINE-ADV-2026-0107",
                S::Moderate,
                "signing-update-path:mirror-index",
                I::InstalledMitigated,
                "mitigation-pin-signed-index",
                "signer_source_state:signed_current",
                A::ReviewRecommended,
                R::ExportSupportPacket,
                C::OfflineMirrorLagDisclosed,
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5AdvisoryRowGovernanceReview {
    M5AdvisoryRowGovernanceReview {
        one_row_model_across_channels: true,
        severity_scope_exposure_visible_without_drawer: true,
        installed_but_affected_stays_visible: true,
        never_degrades_to_generic_update_prompt: true,
        copy_safe_advisory_id_preserved: true,
        export_summary_reconstructs_advisory_truth: true,
        primary_action_parity_across_channels: true,
        every_row_bound_to_shell_zone: true,
        every_row_declares_accessibility_route: true,
        later_lanes_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5AdvisoryRowConsumerProjection {
    M5AdvisoryRowConsumerProjection {
        update_center_renders_shared_row: true,
        marketplace_renders_shared_row: true,
        help_about_renders_shared_row: true,
        support_export_reads_single_source: true,
        resolver_reads_single_advisory_vocabulary: true,
    }
}

fn proof_freshness() -> M5AdvisoryRowProofFreshness {
    M5AdvisoryRowProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5AdvisoryRowReleasePosture {
    M5AdvisoryRowReleasePosture {
        release_packet_ref: M5_ADVISORY_ROW_ARTIFACT_REF.to_owned(),
        advisory_row_audit_ref: M5_ADVISORY_ROW_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_ADVISORY_ROW_SCHEMA_REF,
        M5_ADVISORY_ROW_DOC_REF,
        M5_ADVISORY_ROW_SHELL_ZONE_REF,
        M5_ADVISORY_ROW_COMPONENT_MATRIX_REF,
        M5_ADVISORY_ROW_ADVISORY_CARD_REF,
        M5_ADVISORY_ROW_AFFECTED_INSTALL_REF,
        M5_ADVISORY_ROW_SEVERITY_MATRIX_REF,
    ])
}

/// Builds the canonical M5 advisory-card-row-primitive packet.
pub fn seeded_m5_advisory_card_row_primitive_packet() -> M5AdvisoryRowPrimitivePacket {
    M5AdvisoryRowPrimitivePacket::new(M5AdvisoryRowPrimitivePacketInput {
        packet_id: M5_ADVISORY_ROW_PRIMITIVE_PACKET_ID.to_owned(),
        matrix_label:
            "M5 security-advisory card / row primitive: severity, affected surface, exposure state, fixed version or mitigation, signer / source truth, and primary-action parity across channels"
                .to_owned(),
        surface_rows: surface_rows(),
        vocabulary_set: M5AdvisoryRowVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the extension lane is held at Beta because a slice of the
/// blocked-extension continuity note does not yet render on every profile; every lane
/// stays visible.
pub fn seeded_m5_advisory_card_row_primitive_extension_beta_narrowed(
) -> M5AdvisoryRowPrimitivePacket {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.packet_id = "m5-advisory-card-row-primitive:extension-beta:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.affected_surface == M5AffectedSurfaceLane::Extension)
        .expect("extension row present");
    row.qualification = M5AdvisoryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the signing / update path lane is narrowed to Preview pending
/// mirror-freshness parity across every offline profile; every lane stays visible.
pub fn seeded_m5_advisory_card_row_primitive_signing_update_path_preview_narrowed(
) -> M5AdvisoryRowPrimitivePacket {
    let mut packet = seeded_m5_advisory_card_row_primitive_packet();
    packet.packet_id = "m5-advisory-card-row-primitive:signing-update-path-preview:0001".to_owned();
    let row = packet
        .surface_rows
        .iter_mut()
        .find(|row| row.affected_surface == M5AffectedSurfaceLane::SigningUpdatePath)
        .expect("signing-update-path row present");
    row.qualification = M5AdvisoryQualificationClass::Preview;
    packet
}
