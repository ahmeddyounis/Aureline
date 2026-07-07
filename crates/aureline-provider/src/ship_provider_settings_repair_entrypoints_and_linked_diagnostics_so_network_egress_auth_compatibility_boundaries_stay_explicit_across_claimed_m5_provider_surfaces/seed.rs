//! Canonical seed builders for the M5 provider-settings repair-entrypoint row primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical repair-entrypoint-row primitive packet.
pub const M5_PROVIDER_REPAIR_ENTRYPOINT_PACKET_ID: &str =
    "m5-provider-settings-repair-entrypoint-row:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked repair-entrypoint resolution case from a full boundary state.
#[allow(clippy::too_many_arguments)]
fn repair_case(
    boundary_class: M5ProviderBoundaryClass,
    connection_state: M5AccountConnectionState,
    has_queued_drafts: bool,
    has_cached_read: bool,
    policy_escalation_available: bool,
    boundary_label: &str,
    repair_target_label: &str,
    repair_ref: &str,
) -> M5ProviderRepairEntrypointResolutionCase {
    M5ProviderRepairEntrypointResolutionCase::resolved(M5ProviderRepairEntrypointResolutionInput {
        boundary_class,
        connection_state,
        has_queued_drafts,
        has_cached_read,
        policy_escalation_available,
        boundary_label: boundary_label.to_owned(),
        repair_target_label: repair_target_label.to_owned(),
        repair_ref: repair_ref.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full repair-row anatomy, boundary, state,
/// posture, entrypoint, diagnostic, guarantee, action, export-field, and accessibility parity
/// every consumer carries.
fn base_row(
    consumer_surface: M5ProviderRepairConsumerSurface,
    qualification: M5ProviderQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    examples: Vec<M5ProviderRepairEntrypointResolutionCase>,
) -> M5ProviderRepairConsumerRow {
    M5ProviderRepairConsumerRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ProviderSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ProviderDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5ProviderRepairRowAnatomyPart::ALL.to_vec(),
        boundary_classes: M5ProviderBoundaryClass::ALL.to_vec(),
        connection_states: M5AccountConnectionState::ALL.to_vec(),
        repair_postures: M5ProviderRepairPosture::ALL.to_vec(),
        repair_entrypoints: M5RepairEntrypointClass::ALL.to_vec(),
        linked_diagnostics: M5LinkedDiagnosticClass::ALL.to_vec(),
        continuity_guarantees: M5RepairContinuityGuarantee::ALL.to_vec(),
        row_actions: M5ProviderRepairRowAction::ALL.to_vec(),
        export_fields: M5ProviderRepairRowExportField::ALL.to_vec(),
        accessibility_routes: M5ProviderAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ProviderConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ProviderDowngradeTrigger::ConnectionStateUnstated,
            M5ProviderDowngradeTrigger::MappingOriginUnstated,
            M5ProviderDowngradeTrigger::SyncModeUnstated,
            M5ProviderDowngradeTrigger::ExportBoundaryHidden,
            M5ProviderDowngradeTrigger::AlternateStateLabelInvented,
            M5ProviderDowngradeTrigger::DefaultDestinationAssumed,
            M5ProviderDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PROVIDER_REPAIR_ENTRYPOINT_SCHEMA_REF,
            M5_PROVIDER_REPAIR_ENTRYPOINT_NETWORK_REMEDIATION_REF,
            M5_PROVIDER_REPAIR_ENTRYPOINT_REAUTH_REQUIREMENT_REF,
            M5_PROVIDER_REPAIR_ENTRYPOINT_PROVIDER_COMPAT_REF,
            M5_PROVIDER_REPAIR_ENTRYPOINT_SUPPORT_BUNDLE_REF,
            M5_PROVIDER_REPAIR_ENTRYPOINT_EXPORT_REDACTION_REF,
            M5_PROVIDER_REPAIR_ENTRYPOINT_OFFLINE_HANDOFF_REF,
        ]),
        examples,
        isolates_settings_from_diagnostics: false,
        loses_queued_work: false,
        requires_blind_credential_reentry: false,
        breaks_cached_read_continuity: false,
        breaks_reviewed_export_path: false,
    }
}

fn rows() -> Vec<M5ProviderRepairConsumerRow> {
    use M5AccountConnectionState as Conn;
    use M5ProviderBoundaryClass as Boundary;

    vec![
        // 1. Provider-account row — a stale session repairs through a reviewed reauth handoff
        //    (queued drafts and cached read preserved, never a blind credential prompt) and a
        //    limited-scope session repairs through a scope review — neither collapses into
        //    "retry login".
        base_row(
            M5ProviderRepairConsumerSurface::ProviderAccountRow,
            M5ProviderQualificationClass::Stable,
            "Provider-account row owner",
            "The provider-account row names the real boundary so a stale session reads as reauth-session and links to the reviewed reauth handoff — queued drafts and cached read intact, never a blind credential prompt — and a limited-scope session reads as widen-scope and links to the scope review, each with its support-bundle and export/redaction diagnostics one click away",
            "evidence:m5-provider-repair-account:001",
            vec![
                repair_case(
                    Boundary::AuthStaleSession,
                    Conn::StaleSession,
                    true,
                    true,
                    false,
                    "acme-eng session stale",
                    "acme-eng reauth handoff",
                    "repair:acme-eng:reauth:1",
                ),
                repair_case(
                    Boundary::AuthScopeLimited,
                    Conn::LimitedScope,
                    false,
                    true,
                    false,
                    "acme-eng scope too narrow",
                    "acme-eng scope review",
                    "repair:acme-eng:scope:1",
                ),
            ],
        ),
        // 2. Project / board mapping row — a broken mapping repairs through the mapping repair
        //    (queued drafts preserved) and an incompatible provider repairs through the
        //    compatibility report, both linking to the provider-compatibility diagnostic.
        base_row(
            M5ProviderRepairConsumerSurface::ProjectBoardMappingRow,
            M5ProviderQualificationClass::Stable,
            "Project/board mapping row owner",
            "The mapping row names the real boundary so a broken mapping reads as remap-target and links to the mapping repair with queued drafts preserved, and an incompatible provider reads as compatibility-review and links to the compatibility report — both wired to the provider-compatibility diagnostic and the reviewed export path rather than a bare error",
            "evidence:m5-provider-repair-mapping:001",
            vec![
                repair_case(
                    Boundary::MappingBroken,
                    Conn::SignedIn,
                    true,
                    false,
                    false,
                    "acme-eng board mapping broken",
                    "acme-eng mapping repair",
                    "repair:acme-eng:mapping:1",
                ),
                repair_case(
                    Boundary::ProviderIncompatible,
                    Conn::SignedIn,
                    false,
                    true,
                    false,
                    "acme-eng provider version skew",
                    "acme-eng compatibility report",
                    "repair:acme-eng:compat:1",
                ),
            ],
        ),
        // 3. Sync-behavior row — a network/egress block on an offline cached-read session repairs
        //    through the network diagnostics (cached read preserved) and an incompatible provider
        //    repairs through the compatibility report.
        base_row(
            M5ProviderRepairConsumerSurface::SyncBehaviorRow,
            M5ProviderQualificationClass::Stable,
            "Sync-behavior row owner",
            "The sync-behavior row names the real boundary so a network/egress block reads as network-egress-repair and links to the network diagnostics while the offline cached read keeps working and queued drafts stay put, and an incompatible provider reads as compatibility-review — the row is never an isolated sidebar divorced from the network and compatibility diagnostics",
            "evidence:m5-provider-repair-sync:001",
            vec![
                repair_case(
                    Boundary::NetworkEgressBlocked,
                    Conn::OfflineCachedRead,
                    true,
                    true,
                    false,
                    "acme-eng egress blocked",
                    "acme-eng network diagnostics",
                    "repair:acme-eng:network:1",
                ),
                repair_case(
                    Boundary::ProviderIncompatible,
                    Conn::SignedIn,
                    false,
                    false,
                    false,
                    "acme-eng sync api skew",
                    "acme-eng compatibility report",
                    "repair:acme-eng:compat:2",
                ),
            ],
        ),
        // 4. Privacy / redaction row — a policy-blocked boundary repairs only through a reviewed
        //    escalation (no self-serve entrypoint, cached read preserved) and a stale session
        //    repairs through the reviewed reauth handoff with queued drafts intact.
        base_row(
            M5ProviderRepairConsumerSurface::PrivacyRedactionRow,
            M5ProviderQualificationClass::Stable,
            "Privacy/redaction row owner",
            "The privacy/redaction row names the real boundary so a policy-blocked boundary reads as policy-blocked and offers only a reviewed escalation — never a self-serve bypass — while a stale session reads as reauth-session and links to the reviewed reauth handoff with queued drafts intact, both keeping the reviewed export path and support-bundle diagnostics one click away",
            "evidence:m5-provider-repair-privacy:001",
            vec![
                repair_case(
                    Boundary::PolicyBlocked,
                    Conn::PolicyBlocked,
                    false,
                    true,
                    true,
                    "acme-eng policy blocks export widen",
                    "acme-eng policy review",
                    "repair:acme-eng:policy:1",
                ),
                repair_case(
                    Boundary::AuthStaleSession,
                    Conn::StaleSession,
                    true,
                    true,
                    false,
                    "acme-eng privacy session stale",
                    "acme-eng reauth handoff",
                    "repair:acme-eng:reauth:2",
                ),
            ],
        ),
        // 5. Provider status bar — a network/egress block repairs through the network diagnostics
        //    (queued drafts and cached read preserved) and a policy-blocked boundary offers a
        //    reviewed escalation — the same rows a user reads elsewhere, from the bar alone.
        base_row(
            M5ProviderRepairConsumerSurface::ProviderStatusBar,
            M5ProviderQualificationClass::Stable,
            "Provider status bar owner",
            "The provider status bar names the real boundary so a network/egress block reads as network-egress-repair with queued drafts and cached read preserved and links to the network diagnostics, and a policy-blocked boundary offers only a reviewed escalation — a user can tell the boundary, entrypoint, and preserved work from the bar alone, never retry-login folklore",
            "evidence:m5-provider-repair-status-bar:001",
            vec![
                repair_case(
                    Boundary::NetworkEgressBlocked,
                    Conn::OfflineCachedRead,
                    true,
                    true,
                    false,
                    "acme-eng bar egress blocked",
                    "acme-eng network diagnostics",
                    "repair:acme-eng:network:2",
                ),
                repair_case(
                    Boundary::PolicyBlocked,
                    Conn::PolicyBlocked,
                    false,
                    true,
                    true,
                    "acme-eng bar policy blocked",
                    "acme-eng policy review",
                    "repair:acme-eng:policy:2",
                ),
            ],
        ),
    ]
}

fn governance_review() -> M5ProviderRepairGovernanceReview {
    M5ProviderRepairGovernanceReview {
        rows_link_to_network_egress_diagnostics: true,
        rows_link_to_auth_diagnostics: true,
        rows_link_to_support_bundle_diagnostics: true,
        rows_link_to_provider_compatibility_diagnostics: true,
        repair_preserves_queued_drafts: true,
        repair_preserves_cached_read_continuity: true,
        repair_preserves_reviewed_export_path: true,
        repair_never_requires_blind_credential_reentry: true,
        settings_never_isolated_from_diagnostics: true,
        every_boundary_names_a_repair_entrypoint: true,
        rows_stable_across_deployment_lines: true,
        rows_stable_across_consumer_surfaces: true,
        every_row_declares_accessibility_route: true,
        support_export_reconstructs_repair_truth: true,
        later_rows_cannot_invent_parallel_repair_vocabulary: true,
    }
}

fn consumer_projection() -> M5ProviderRepairConsumerProjection {
    M5ProviderRepairConsumerProjection {
        provider_surfaces_consume_repair_vocabulary: true,
        repair_posture_reads_single_source: true,
        linked_diagnostics_reads_single_source: true,
        support_export_reads_single_source: true,
        headless_and_desktop_read_single_source: true,
    }
}

fn proof_freshness() -> M5ProviderRepairProofFreshness {
    M5ProviderRepairProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ProviderRepairReleasePosture {
    M5ProviderRepairReleasePosture {
        release_packet_ref: M5_PROVIDER_REPAIR_ENTRYPOINT_ARTIFACT_REF.to_owned(),
        provider_repair_diagnostics_audit_ref: M5_PROVIDER_REPAIR_ENTRYPOINT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PROVIDER_REPAIR_ENTRYPOINT_SCHEMA_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_DOC_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_COMPONENT_MATRIX_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_NETWORK_REMEDIATION_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_REAUTH_REQUIREMENT_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_PROVIDER_COMPAT_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_SUPPORT_BUNDLE_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_EXPORT_REDACTION_REF,
        M5_PROVIDER_REPAIR_ENTRYPOINT_OFFLINE_HANDOFF_REF,
    ])
}

/// Builds the canonical M5 provider repair-entrypoint-row packet.
pub fn seeded_m5_provider_repair_entrypoint_packet() -> M5ProviderRepairEntrypointPacket {
    M5ProviderRepairEntrypointPacket::new(M5ProviderRepairEntrypointPacketInput {
        packet_id: M5_PROVIDER_REPAIR_ENTRYPOINT_PACKET_ID.to_owned(),
        matrix_label:
            "M5 provider-settings repair-entrypoint row primitive: boundary class (network-egress/auth-stale/auth-scope/mapping-broken/provider-incompatible/policy-blocked), account connection state, repair posture and concrete entrypoint, linked diagnostics (network-egress/auth-session/support-bundle/provider-compatibility/export-redaction), continuity guarantees (queued drafts/cached read/reviewed export/no blind credential re-entry), and bounded reveal/open-entrypoint/open-diagnostics/export-evidence/request-escalation actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5ProviderRepairVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the sync-behavior-row consumer is held at Beta because a slice of sync rows
/// do not yet render the keyboard route cue on every profile; every consumer stays visible.
pub fn seeded_m5_provider_repair_entrypoint_sync_behavior_beta_narrowed(
) -> M5ProviderRepairEntrypointPacket {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet.packet_id =
        "m5-provider-settings-repair-entrypoint-row:sync-behavior-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ProviderRepairConsumerSurface::SyncBehaviorRow)
        .expect("sync-behavior row present");
    row.qualification = M5ProviderQualificationClass::Beta;
    packet
}

/// Narrowed variant: the privacy/redaction-row consumer is narrowed to Preview pending
/// reviewed-escalation parity proof across every deployment; every consumer stays visible.
pub fn seeded_m5_provider_repair_entrypoint_privacy_redaction_preview_narrowed(
) -> M5ProviderRepairEntrypointPacket {
    let mut packet = seeded_m5_provider_repair_entrypoint_packet();
    packet.packet_id =
        "m5-provider-settings-repair-entrypoint-row:privacy-redaction-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ProviderRepairConsumerSurface::PrivacyRedactionRow)
        .expect("privacy-redaction row present");
    row.qualification = M5ProviderQualificationClass::Preview;
    packet
}
