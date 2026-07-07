//! Canonical seed builders for the frozen M5 provider-account / offline-capture
//! component matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical provider-account / offline-capture component matrix.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-provider-account-offline-capture-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5ProviderRequiredLabel> {
    M5ProviderRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5ProviderRequiredLabel]) -> Vec<M5ProviderRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5ProviderAccountOfflineComponentFamily,
    qualification: M5ProviderQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5ProviderAccountOfflineComponentRow {
    M5ProviderAccountOfflineComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ProviderSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ProviderDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        provider_identity_classes: vec![],
        account_connection_states: vec![],
        tenant_scopes: vec![],
        mapping_origins: vec![],
        mapping_target_kinds: vec![],
        sync_modes: vec![],
        write_scopes: vec![],
        offline_capture_states: vec![],
        queued_draft_states: vec![],
        redaction_classes: vec![],
        export_boundaries: vec![],
        accessibility_routes: M5ProviderAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5ProviderConsumerSurface::AccountSettingsUi,
            M5ProviderConsumerSurface::SupportExport,
            M5ProviderConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5ProviderDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        masks_connection_or_scope: false,
        hides_export_or_redaction_boundary: false,
        invents_alternate_state_label: false,
        assumes_default_destination_silently: false,
    }
}

fn component_rows() -> Vec<M5ProviderAccountOfflineComponentRow> {
    use M5AccountConnectionState as CS;
    use M5ExportBoundaryClass as EB;
    use M5MappingOriginClass as MO;
    use M5MappingTargetKind as MT;
    use M5OfflineCaptureState as OC;
    use M5ProviderAccountOfflineComponentFamily as F;
    use M5ProviderConsumerSurface as C;
    use M5ProviderDowngradeTrigger as D;
    use M5ProviderIdentityClass as IC;
    use M5ProviderQualificationClass as Q;
    use M5ProviderRedactionClass as RC;
    use M5ProviderRequiredLabel as L;
    use M5ProviderSyncMode as SM;
    use M5ProviderWriteScope as WS;
    use M5QueuedDraftState as QD;
    use M5TenantScopeClass as TS;

    let mut rows = Vec::new();

    // 1. Provider-account row.
    let mut row = base_row(
        F::ProviderAccountRow,
        Q::Stable,
        "Provider-account row owner",
        "One provider-account-row model naming how the acting account is identified — a personal account, an organization member, a service account, a delegated credential, an installation grant, or an unlinked identity — its connection state (not configured, signed in, limited scope, stale session, offline cached read, or policy blocked), and the tenant scope it acts within, so a user never has to infer whether Aureline can read or write right now",
        "evidence:m5-provider-account-row-parity:001",
        &[
            M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SCHEMA_REF,
            M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_CONNECTED_ACCOUNT_REF,
        ],
    );
    row.provider_identity_classes = IC::ALL.to_vec();
    row.account_connection_states = CS::ALL.to_vec();
    row.tenant_scopes = TS::ALL.to_vec();
    row.required_labels = labels_with(&[L::ConnectionAndScope]);
    row.consumer_surfaces = vec![
        C::AccountSettingsUi,
        C::StatusBarUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ConnectionStateUnstated,
        D::TenantScopeUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Project-or-board mapping row.
    let mut row = base_row(
        F::ProjectOrBoardMappingRow,
        Q::Stable,
        "Project/board mapping row owner",
        "One project-or-board-mapping-row model naming where a publish will land — an issue-tracker project, a kanban board, a repository, a milestone, a label set, or an unmapped target — and how that default destination was derived (explicit user choice, inherited default, auto-matched, imported config, policy pinned, or unmapped origin), so a default publish destination is never assumed silently and never given an alternate label for its origin",
        "evidence:m5-project-board-mapping-row-parity:001",
        &[
            M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SCHEMA_REF,
            M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_TARGET_MAPPING_REF,
        ],
    );
    row.mapping_origins = MO::ALL.to_vec();
    row.mapping_target_kinds = MT::ALL.to_vec();
    row.required_labels = labels_with(&[L::MappingAndSyncMode]);
    row.consumer_surfaces = vec![
        C::MappingPickerUi,
        C::AccountSettingsUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::MappingOriginUnstated,
        D::DefaultDestinationAssumed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Sync-behavior row.
    let mut row = base_row(
        F::SyncBehaviorRow,
        Q::Stable,
        "Sync-behavior row owner",
        "One sync-behavior-row model naming how local and provider truth are kept in step — live bidirectional, read-only mirror, manual push, scheduled sync, paused sync, or offline only — the effective write scope Aureline has right now (full write, comment only, status only, read only, no write, or unknown), and the state of any locally queued draft, so a user never has to infer whether Aureline can write or what remains queued locally",
        "evidence:m5-sync-behavior-row-parity:001",
        &[
            M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SCHEMA_REF,
            M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SYNC_HEALTH_REF,
        ],
    );
    row.sync_modes = SM::ALL.to_vec();
    row.write_scopes = WS::ALL.to_vec();
    row.queued_draft_states = QD::ALL.to_vec();
    row.required_labels = labels_with(&[L::MappingAndSyncMode]);
    row.consumer_surfaces = vec![
        C::SyncStatusUi,
        C::StatusBarUi,
        C::OfflineQueueUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SyncModeUnstated,
        D::WriteScopeUnstated,
        D::QueuedDraftStateHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Offline-capture row.
    let mut row = base_row(
        F::OfflineCaptureRow,
        Q::Stable,
        "Offline-capture row owner",
        "One offline-capture-row model naming how a locally captured change is held — captured local, queued for publish, publish deferred, conflict held, discard pending, or synced and cleared — and the state of its queued draft, so a user always sees what remains queued locally and a pending publish is never silently dropped or shown as reconciled",
        "evidence:m5-offline-capture-row-parity:001",
        &[
            M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SCHEMA_REF,
            M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_OFFLINE_HANDOFF_REF,
        ],
    );
    row.offline_capture_states = OC::ALL.to_vec();
    row.queued_draft_states = QD::ALL.to_vec();
    row.required_labels = labels_with(&[L::ConnectionAndScope]);
    row.consumer_surfaces = vec![
        C::OfflineQueueUi,
        C::SyncStatusUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::OfflineCaptureStateUnstated,
        D::QueuedDraftStateHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Privacy-redaction row.
    let mut row = base_row(
        F::PrivacyRedactionRow,
        Q::Stable,
        "Privacy-redaction row owner",
        "One privacy-redaction-row model naming how much of a provider-linked object will be revealed — full body visible, metadata only, redacted share, policy restricted, raw bodies withheld, or no export — and the metadata-safe export boundary it keeps, so a user always sees what support and export flows will disclose and no surface invents an alternate label for a metadata-safe export",
        "evidence:m5-privacy-redaction-row-parity:001",
        &[
            M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SCHEMA_REF,
            M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_EXPORT_REDACTION_REF,
        ],
    );
    row.redaction_classes = RC::ALL.to_vec();
    row.export_boundaries = EB::ALL.to_vec();
    row.required_labels = labels_with(&[L::RedactionAndExportBoundary]);
    row.consumer_surfaces = vec![
        C::PrivacyReviewUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RedactionClassUnstated,
        D::ExportBoundaryHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5ProviderAccountOfflineComponentGovernanceReview {
    M5ProviderAccountOfflineComponentGovernanceReview {
        account_row_shows_connection_and_scope: true,
        mapping_row_shows_origin_and_target: true,
        sync_row_shows_mode_and_write_scope: true,
        offline_row_shows_capture_and_queued_state: true,
        privacy_row_shows_redaction_and_export_boundary: true,
        no_surface_invents_alternate_state_label: true,
        connection_state_vocabulary_named_once: true,
        mapping_sync_offline_export_named_once: true,
        default_destination_always_explicit: true,
        write_scope_always_explicit: true,
        queued_draft_state_always_explicit: true,
        export_boundary_always_explicit: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ProviderAccountOfflineComponentConsumerProjection {
    M5ProviderAccountOfflineComponentConsumerProjection {
        account_surfaces_consume_connection_vocabulary: true,
        mapping_surfaces_consume_origin_vocabulary: true,
        sync_surfaces_consume_mode_vocabulary: true,
        offline_surfaces_consume_capture_vocabulary: true,
        privacy_surfaces_consume_redaction_and_export_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ProviderAccountOfflineComponentProofFreshness {
    M5ProviderAccountOfflineComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ProviderAccountOfflineComponentReleasePosture {
    M5ProviderAccountOfflineComponentReleasePosture {
        proof_packet_ref: M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_ARTIFACT_REF.to_owned(),
        provider_account_audit_ref: M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SCHEMA_REF,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_DOC_REF,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_CONNECTED_ACCOUNT_REF,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_TARGET_MAPPING_REF,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SYNC_HEALTH_REF,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_OFFLINE_HANDOFF_REF,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_EXPORT_REDACTION_REF,
    ])
}

/// Builds the canonical frozen M5 provider-account / offline-capture component matrix
/// packet.
pub fn seeded_m5_provider_account_offline_capture_component_matrix(
) -> M5ProviderAccountOfflineComponentMatrixPacket {
    M5ProviderAccountOfflineComponentMatrixPacket::new(
        M5ProviderAccountOfflineComponentMatrixPacketInput {
            packet_id: M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_PACKET_ID.to_owned(),
            matrix_label:
                "M5 provider-account-row, project-or-board-mapping-row, sync-behavior-row, offline-capture-row, and privacy-redaction-row component matrix"
                    .to_owned(),
            component_rows: component_rows(),
            vocabulary_set: M5ProviderAccountOfflineComponentVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the offline-capture row is held at Beta because a slice of the
/// conflict-held state does not yet round-trip across every provider surface; every
/// component stays visible.
pub fn seeded_m5_provider_account_offline_capture_component_matrix_offline_capture_row_beta_narrowed(
) -> M5ProviderAccountOfflineComponentMatrixPacket {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.packet_id =
        "m5-provider-account-offline-capture-components:offline-capture-row-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5ProviderAccountOfflineComponentFamily::OfflineCaptureRow
        })
        .expect("offline-capture-row row present");
    row.qualification = M5ProviderQualificationClass::Beta;
    packet
}

/// Narrowed variant: the privacy-redaction row is narrowed to Preview pending
/// export-boundary parity proof across every surface; every component stays visible.
pub fn seeded_m5_provider_account_offline_capture_component_matrix_privacy_redaction_row_preview_narrowed(
) -> M5ProviderAccountOfflineComponentMatrixPacket {
    let mut packet = seeded_m5_provider_account_offline_capture_component_matrix();
    packet.packet_id =
        "m5-provider-account-offline-capture-components:privacy-redaction-row-preview:0001"
            .to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5ProviderAccountOfflineComponentFamily::PrivacyRedactionRow
        })
        .expect("privacy-redaction-row row present");
    row.qualification = M5ProviderQualificationClass::Preview;
    packet
}
