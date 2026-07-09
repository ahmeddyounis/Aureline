//! Canonical seed builders for the frozen M5 work-item component matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical work-item component matrix.
pub const M5_WORK_ITEM_COMPONENT_MATRIX_PACKET_ID: &str = "m5-work-item-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5WorkItemRequiredLabel> {
    M5WorkItemRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5WorkItemRequiredLabel]) -> Vec<M5WorkItemRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5WorkItemComponentFamily,
    qualification: M5WorkItemQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5WorkItemComponentRow {
    M5WorkItemComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5WorkItemSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5WorkItemDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        work_item_kinds: vec![],
        provider_authorities: vec![],
        local_states: vec![],
        relation_kinds: vec![],
        evidence_kinds: vec![],
        transition_effects: vec![],
        handoff_destinations: vec![],
        export_boundaries: vec![],
        accessibility_routes: M5WorkItemAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5WorkItemConsumerSurface::DetailUi,
            M5WorkItemConsumerSurface::SupportExport,
            M5WorkItemConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5WorkItemDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        masks_identity_or_authority: false,
        hides_local_or_publish_later_state: false,
        invents_alternate_state_label: false,
        uses_generic_ticket_wording: false,
    }
}

fn component_rows() -> Vec<M5WorkItemComponentRow> {
    use M5WorkItemComponentFamily as F;
    use M5WorkItemConsumerSurface as C;
    use M5WorkItemDowngradeTrigger as D;
    use M5WorkItemEvidenceKind as EK;
    use M5WorkItemExportBoundary as EB;
    use M5WorkItemHandoffDestination as HD;
    use M5WorkItemKind as K;
    use M5WorkItemLocalState as LS;
    use M5WorkItemProviderAuthority as PA;
    use M5WorkItemQualificationClass as Q;
    use M5WorkItemRelationKind as RK;
    use M5WorkItemRequiredLabel as L;
    use M5WorkItemTransitionEffect as TE;

    let mut rows = Vec::new();

    // 1. Work-item row.
    let mut row = base_row(
        F::WorkItemRow,
        Q::Stable,
        "Work-item row owner",
        "One work-item-row model naming the canonical work item — an issue, task, incident, change request, epic, or unknown kind — the provider authority behind it (provider owned, local draft, mirrored read only, imported snapshot, unlinked local, or policy pinned), and its local-versus-provider state (synced, local-only draft, queued for publish, publish deferred, publish failed, or conflict held), so generic ticket wording never conceals who owns the object or what is only local and not yet published",
        "evidence:m5-work-item-row-parity:001",
        &[
            M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
            M5_WORK_ITEM_ROW_SCHEMA_REF,
        ],
    );
    row.work_item_kinds = K::ALL.to_vec();
    row.provider_authorities = PA::ALL.to_vec();
    row.local_states = LS::ALL.to_vec();
    row.required_labels = labels_with(&[
        L::ProviderAuthority,
        L::LocalVersusProviderState,
        L::PublishLaterContinuity,
    ]);
    row.consumer_surfaces = vec![
        C::InboxUi,
        C::DetailUi,
        C::SyncStatusUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::IdentityUnstated,
        D::ProviderAuthorityUnstated,
        D::LocalVersusProviderStateHidden,
        D::GenericTicketWordingUsed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Provider-chip group.
    let mut row = base_row(
        F::ProviderChipGroup,
        Q::Stable,
        "Provider-chip group owner",
        "One provider-chip-group model naming who owns a work item and whether Aureline may write to it — provider owned, local draft, mirrored read only, imported snapshot, unlinked local, or policy pinned — so provider authority is always explicit and never left to generic ticket wording",
        "evidence:m5-provider-chip-group-parity:001",
        &[
            M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
            M5_PROVIDER_CHIP_GROUP_SCHEMA_REF,
        ],
    );
    row.provider_authorities = PA::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProviderAuthority]);
    row.consumer_surfaces = vec![
        C::InboxUi,
        C::DetailUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ProviderAuthorityUnstated,
        D::GenericTicketWordingUsed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Relation strip.
    let mut row = base_row(
        F::RelationStrip,
        Q::Stable,
        "Relation strip owner",
        "One relation-strip model naming the linked engineering context of a work item — a linked branch, pull request, review, test run, incident, or an unmapped relation — so the branch/review/test context is always explicit and never given an alternate label",
        "evidence:m5-relation-strip-parity:001",
        &[
            M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
            M5_RELATION_STRIP_SCHEMA_REF,
        ],
    );
    row.relation_kinds = RK::ALL.to_vec();
    row.required_labels = labels_with(&[L::LinkedEngineeringContext]);
    row.consumer_surfaces = vec![
        C::DetailUi,
        C::RelationPanelUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::LinkedContextUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Sync-pending pill.
    let mut row = base_row(
        F::SyncPendingPill,
        Q::Stable,
        "Sync-pending pill owner",
        "One sync-pending-pill model naming a work item's local-versus-provider state — synced, local-only draft, queued for publish, publish deferred, publish failed, or conflict held — so a pending publish is never silently dropped or shown as reconciled",
        "evidence:m5-sync-pending-pill-parity:001",
        &[
            M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
            M5_SYNC_PENDING_PILL_SCHEMA_REF,
        ],
    );
    row.local_states = LS::ALL.to_vec();
    row.required_labels = labels_with(&[L::LocalVersusProviderState, L::PublishLaterContinuity]);
    row.consumer_surfaces = vec![
        C::InboxUi,
        C::DetailUi,
        C::SyncStatusUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::LocalVersusProviderStateHidden,
        D::SyncPendingStateHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Work-item detail header.
    let mut row = base_row(
        F::WorkItemDetailHeader,
        Q::Stable,
        "Work-item detail header owner",
        "One work-item-detail-header model naming the canonical work item — issue, task, incident, change request, epic, or unknown kind — and the provider authority behind it, so the header always states identity and who owns the object before any transition or comment",
        "evidence:m5-work-item-detail-header-parity:001",
        &[
            M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
            M5_WORK_ITEM_DETAIL_HEADER_SCHEMA_REF,
        ],
    );
    row.work_item_kinds = K::ALL.to_vec();
    row.provider_authorities = PA::ALL.to_vec();
    row.required_labels = labels_with(&[L::ProviderAuthority]);
    row.consumer_surfaces = vec![C::DetailUi, C::SupportExport, C::CliInspect, C::ProductUi];
    row.downgrade_triggers = vec![
        D::IdentityUnstated,
        D::ProviderAuthorityUnstated,
        D::GenericTicketWordingUsed,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Status-transition sheet (narrowed to Beta by the narrowed builder).
    let mut row = base_row(
        F::StatusTransitionSheet,
        Q::Stable,
        "Status-transition sheet owner",
        "One status-transition-sheet model previewing the side effects of a transition before write — a local-only transition, a publish-now transition, open in provider, a comment side effect, a status side effect, or a blocked transition — so a user never has to infer whether a transition is only local or publishes to the provider",
        "evidence:m5-status-transition-sheet-parity:001",
        &[
            M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
            M5_STATUS_TRANSITION_SHEET_SCHEMA_REF,
        ],
    );
    row.transition_effects = TE::ALL.to_vec();
    row.required_labels = labels_with(&[L::SideEffectPreview, L::PublishLaterContinuity]);
    row.consumer_surfaces = vec![
        C::DetailUi,
        C::TransitionSheetUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SideEffectPreviewHidden,
        D::PublishLaterContinuityHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Related-evidence card.
    let mut row = base_row(
        F::RelatedEvidenceCard,
        Q::Stable,
        "Related-evidence card owner",
        "One related-evidence-card model naming the provenance of linked evidence — a test result, CI check, review thread, linked change, attached artifact, or external reference — so evidence never appears without disclosing what it is",
        "evidence:m5-related-evidence-card-parity:001",
        &[
            M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
            M5_RELATED_EVIDENCE_CARD_SCHEMA_REF,
        ],
    );
    row.evidence_kinds = EK::ALL.to_vec();
    row.required_labels = labels_with(&[L::LinkedEngineeringContext]);
    row.consumer_surfaces = vec![
        C::DetailUi,
        C::EvidencePanelUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::EvidenceProvenanceUnstated,
        D::LinkedContextUnstated,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Offline-handoff-packet card (narrowed to Preview by the narrowed builder).
    let mut row = base_row(
        F::OfflineHandoffPacketCard,
        Q::Stable,
        "Offline-handoff packet card owner",
        "One offline-handoff-packet-card model naming where a deferred change will land — a local queue, provider publish, exported packet, support bundle, another device, or discard after review — and the metadata-safe export boundary it keeps (metadata safe, body excluded, identifiers masked, credentials scrubbed, local only, or full disclosure blocked), so a handoff destination is never assumed silently and export never reveals more than disclosed",
        "evidence:m5-offline-handoff-packet-card-parity:001",
        &[
            M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
            M5_OFFLINE_HANDOFF_PACKET_CARD_SCHEMA_REF,
        ],
    );
    row.handoff_destinations = HD::ALL.to_vec();
    row.export_boundaries = EB::ALL.to_vec();
    row.required_labels = labels_with(&[L::PublishLaterContinuity]);
    row.consumer_surfaces = vec![
        C::DetailUi,
        C::SyncStatusUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::HandoffDestinationUnstated,
        D::ExportBoundaryHidden,
        D::PublishLaterContinuityHidden,
        D::AlternateStateLabelInvented,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5WorkItemComponentGovernanceReview {
    M5WorkItemComponentGovernanceReview {
        row_shows_identity_authority_and_state: true,
        chip_group_shows_authority: true,
        relation_strip_shows_linked_context: true,
        sync_pill_shows_local_versus_provider_state: true,
        detail_header_shows_identity_and_authority: true,
        transition_sheet_shows_side_effect_preview: true,
        evidence_card_shows_provenance: true,
        handoff_card_shows_destination_and_export_boundary: true,
        no_surface_invents_alternate_state_label: true,
        no_generic_ticket_wording_conceals_authority: true,
        publish_later_continuity_always_explicit: true,
        side_effect_preview_always_before_write: true,
        export_boundary_always_explicit: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5WorkItemComponentConsumerProjection {
    M5WorkItemComponentConsumerProjection {
        inbox_and_detail_surfaces_consume_identity_vocabulary: true,
        chip_surfaces_consume_authority_vocabulary: true,
        sync_surfaces_consume_local_state_vocabulary: true,
        relation_and_evidence_surfaces_consume_context_vocabulary: true,
        transition_and_handoff_surfaces_consume_publish_later_vocabulary: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5WorkItemComponentProofFreshness {
    M5WorkItemComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5WorkItemComponentReleasePosture {
    M5WorkItemComponentReleasePosture {
        proof_packet_ref: M5_WORK_ITEM_COMPONENT_ARTIFACT_REF.to_owned(),
        work_item_matrix_audit_ref: M5_WORK_ITEM_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_DOC_REF,
        M5_WORK_ITEM_ROW_SCHEMA_REF,
        M5_PROVIDER_CHIP_GROUP_SCHEMA_REF,
        M5_RELATION_STRIP_SCHEMA_REF,
        M5_SYNC_PENDING_PILL_SCHEMA_REF,
        M5_WORK_ITEM_DETAIL_HEADER_SCHEMA_REF,
        M5_STATUS_TRANSITION_SHEET_SCHEMA_REF,
        M5_RELATED_EVIDENCE_CARD_SCHEMA_REF,
        M5_OFFLINE_HANDOFF_PACKET_CARD_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 work-item component matrix packet.
pub fn seeded_m5_work_item_component_matrix() -> M5WorkItemComponentMatrixPacket {
    M5WorkItemComponentMatrixPacket::new(M5WorkItemComponentMatrixPacketInput {
        packet_id: M5_WORK_ITEM_COMPONENT_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 work-item-row, provider-chip-group, relation-strip, sync-pending-pill, work-item-detail-header, status-transition-sheet, related-evidence-card, and offline-handoff-packet-card component matrix"
                .to_owned(),
        component_rows: component_rows(),
        vocabulary_set: M5WorkItemComponentVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the status-transition sheet is held at Beta because a slice of the
/// publish-now side-effect preview does not yet round-trip across every provider surface;
/// every component stays visible.
pub fn seeded_m5_work_item_component_matrix_status_transition_sheet_beta_narrowed(
) -> M5WorkItemComponentMatrixPacket {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet.packet_id = "m5-work-item-components:status-transition-sheet-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5WorkItemComponentFamily::StatusTransitionSheet)
        .expect("status-transition-sheet row present");
    row.qualification = M5WorkItemQualificationClass::Beta;
    packet
}

/// Narrowed variant: the offline-handoff-packet card is narrowed to Preview pending
/// export-boundary parity proof across every surface; every component stays visible.
pub fn seeded_m5_work_item_component_matrix_offline_handoff_packet_card_preview_narrowed(
) -> M5WorkItemComponentMatrixPacket {
    let mut packet = seeded_m5_work_item_component_matrix();
    packet.packet_id =
        "m5-work-item-components:offline-handoff-packet-card-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| row.component_family == M5WorkItemComponentFamily::OfflineHandoffPacketCard)
        .expect("offline-handoff-packet-card row present");
    row.qualification = M5WorkItemQualificationClass::Preview;
    packet
}
