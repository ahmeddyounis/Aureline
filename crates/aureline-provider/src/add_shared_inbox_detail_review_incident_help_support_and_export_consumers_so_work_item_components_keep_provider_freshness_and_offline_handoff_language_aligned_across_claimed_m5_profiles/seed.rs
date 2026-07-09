//! Canonical seed builders for the M5 work-item component-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked bindings, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical work-item component-consumer packet.
pub const M5_WORK_ITEM_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-work-item-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5WorkItemComponentConsumer,
    component_family: M5WorkItemComponentFamily,
    parity_health: M5WorkItemConsumerParityHealth,
    export_caveats: &[M5WorkItemConsumerExportCaveat],
    note: &str,
) -> M5WorkItemComponentBindingCase {
    M5WorkItemComponentBindingCase::resolved(M5WorkItemComponentBindingInput {
        consumer,
        component_family,
        descriptor_families: M5WorkItemComponentDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5WorkItemComponentFamily,
    example_bindings: Vec<M5WorkItemComponentBindingCase>,
) -> M5WorkItemComponentBinding {
    M5WorkItemComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5WorkItemComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5WorkItemComponentBinding>,
) -> M5WorkItemComponentConsumerRow {
    M5WorkItemComponentConsumerRow {
        consumer,
        qualification: M5WorkItemQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5WorkItemSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5WorkItemDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5WorkItemConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5WorkItemComponentDescriptor::ALL.to_vec(),
        parity_health_modes: M5WorkItemConsumerParityHealth::ALL.to_vec(),
        export_caveats: M5WorkItemConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5WorkItemClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5WorkItemConsumerNarrowingReason::ALL.to_vec(),
        recovery_actions: M5WorkItemConsumerRecoveryAction::ALL.to_vec(),
        export_fields: M5WorkItemConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5WorkItemAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5WorkItemConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5WorkItemDowngradeTrigger::IdentityUnstated,
            M5WorkItemDowngradeTrigger::ProviderAuthorityUnstated,
            M5WorkItemDowngradeTrigger::LocalVersusProviderStateHidden,
            M5WorkItemDowngradeTrigger::LinkedContextUnstated,
            M5WorkItemDowngradeTrigger::PublishLaterContinuityHidden,
            M5WorkItemDowngradeTrigger::GenericTicketWordingUsed,
            M5WorkItemDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_WORK_ITEM_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_WORK_ITEM_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_work_item_grammar: false,
        drops_identity_authority_freshness_or_publish_later_when_narrowed: false,
        shows_queued_or_offline_state_as_committed: false,
        inherits_stronger_label_from_healthier_profile: false,
        uses_generic_ticket_wording: false,
    }
}

#[allow(clippy::vec_init_then_push)]
fn consumer_rows() -> Vec<M5WorkItemComponentConsumerRow> {
    use M5WorkItemComponentConsumer as Consumer;
    use M5WorkItemComponentFamily as Family;
    use M5WorkItemConsumerExportCaveat as Caveat;
    use M5WorkItemConsumerParityHealth as Health;

    let mut rows = Vec::new();

    // 1. Issue inbox — the work-item row and provider-chip group at full parity (canonical ID,
    //    owner, provider authority), plus the sync-pending pill auto-narrowed because the
    //    change is still queued locally and not published yet.
    rows.push(base_row(
        Consumer::Inbox,
        "Issue-inbox surface owner",
        "The issue inbox adopts the work-item row and provider-chip group at full parity, pointing at the canonical component schemas so canonical identity, provider authority, local-versus-provider state, linked context, the side-effect preview, and publish-later continuity match what detail, review, incident, Help / docs, the support / export desk, and the offline export packet read; the sync-pending pill auto-narrows while a change is still queued locally",
        "evidence:m5-work-item-consumer-inbox:001",
        vec![
            binding(
                Family::WorkItemRow,
                vec![case(
                    Consumer::Inbox,
                    Family::WorkItemRow,
                    Health::FullParity,
                    &[],
                    "inbox work-item row at full parity",
                )],
            ),
            binding(
                Family::ProviderChipGroup,
                vec![case(
                    Consumer::Inbox,
                    Family::ProviderChipGroup,
                    Health::FullParity,
                    &[],
                    "inbox provider-chip group at full parity",
                )],
            ),
            binding(
                Family::SyncPendingPill,
                vec![case(
                    Consumer::Inbox,
                    Family::SyncPendingPill,
                    Health::SyncPendingNarrowed,
                    &[Caveat::SyncPendingNotCommitted],
                    "inbox sync-pending pill narrowed by still-queued local change",
                )],
            ),
        ],
    ));

    // 2. Work-item detail — the detail header and work-item row at full parity, plus the
    //    relation strip auto-narrowed because a linked branch/review/test relation is stale.
    rows.push(base_row(
        Consumer::Detail,
        "Work-item detail surface owner",
        "The work-item detail adopts the detail header and work-item row at full parity, and the relation strip auto-narrowed because a linked branch/review/test relation is stale, keeping canonical identity, provider authority, local-versus-provider state, linked context, the side-effect preview, and publish-later continuity explicit so a stale relation never reads as current provider context",
        "evidence:m5-work-item-consumer-detail:001",
        vec![
            binding(
                Family::WorkItemDetailHeader,
                vec![case(
                    Consumer::Detail,
                    Family::WorkItemDetailHeader,
                    Health::FullParity,
                    &[],
                    "detail header at full parity",
                )],
            ),
            binding(
                Family::WorkItemRow,
                vec![case(
                    Consumer::Detail,
                    Family::WorkItemRow,
                    Health::FullParity,
                    &[],
                    "detail work-item row at full parity",
                )],
            ),
            binding(
                Family::RelationStrip,
                vec![case(
                    Consumer::Detail,
                    Family::RelationStrip,
                    Health::LinkedContextStaleNarrowed,
                    &[Caveat::LinkedContextStaleNotAuthoritative],
                    "detail relation strip narrowed by stale linked relation",
                )],
            ),
        ],
    ));

    // 3. Review workspace — the relation strip and related-evidence card at full parity, plus
    //    the status-transition sheet auto-narrowed because provider write scope is limited on
    //    this surface, so a transition stays read-or-limited-write.
    rows.push(base_row(
        Consumer::Review,
        "Review-workspace surface owner",
        "The review workspace adopts the relation strip and related-evidence card at full parity, and the status-transition sheet auto-narrowed because provider write scope is limited here, keeping canonical identity, provider authority, local-versus-provider state, linked context, the side-effect preview, and publish-later continuity disclosed so a transition never publishes beyond the scope Aureline actually holds",
        "evidence:m5-work-item-consumer-review:001",
        vec![
            binding(
                Family::RelationStrip,
                vec![case(
                    Consumer::Review,
                    Family::RelationStrip,
                    Health::FullParity,
                    &[],
                    "review relation strip at full parity",
                )],
            ),
            binding(
                Family::RelatedEvidenceCard,
                vec![case(
                    Consumer::Review,
                    Family::RelatedEvidenceCard,
                    Health::FullParity,
                    &[],
                    "review related-evidence card at full parity",
                )],
            ),
            binding(
                Family::StatusTransitionSheet,
                vec![case(
                    Consumer::Review,
                    Family::StatusTransitionSheet,
                    Health::ProviderScopeLimitedNarrowed,
                    &[Caveat::ScopeLimitedReadOnly],
                    "review status-transition sheet narrowed by limited provider scope",
                )],
            ),
        ],
    ));

    // 4. Incident workspace — the related-evidence card at full parity, plus the sync-pending
    //    pill auto-narrowed by a still-queued change and the offline-handoff-packet card
    //    auto-narrowed because the packet stays local-only until it is exported or published.
    rows.push(base_row(
        Consumer::Incident,
        "Incident-workspace surface owner",
        "The incident workspace adopts the related-evidence card at full parity, the sync-pending pill auto-narrowed by a still-queued change, and the offline-handoff-packet card auto-narrowed because the packet stays local-only until exported or published, keeping canonical identity, provider authority, local-versus-provider state, linked context, the side-effect preview, and publish-later continuity explicit so a locally-held incident update never masquerades as a provider-committed one",
        "evidence:m5-work-item-consumer-incident:001",
        vec![
            binding(
                Family::RelatedEvidenceCard,
                vec![case(
                    Consumer::Incident,
                    Family::RelatedEvidenceCard,
                    Health::FullParity,
                    &[],
                    "incident related-evidence card at full parity",
                )],
            ),
            binding(
                Family::SyncPendingPill,
                vec![case(
                    Consumer::Incident,
                    Family::SyncPendingPill,
                    Health::SyncPendingNarrowed,
                    &[Caveat::SyncPendingNotCommitted],
                    "incident sync-pending pill narrowed by still-queued change",
                )],
            ),
            binding(
                Family::OfflineHandoffPacketCard,
                vec![case(
                    Consumer::Incident,
                    Family::OfflineHandoffPacketCard,
                    Health::OfflineHandoffNarrowed,
                    &[Caveat::OfflineHandoffLocalOnly],
                    "incident offline-handoff-packet card narrowed by local-only packet",
                )],
            ),
        ],
    ));

    // 5. Help / docs — the work-item row, detail header, and status-transition sheet all at
    //    full parity: documentation describes the same identity, authority, and side-effect
    //    truth the product renders.
    rows.push(base_row(
        Consumer::Help,
        "Help / docs surface owner",
        "Help / docs adopt the work-item row, detail header, and status-transition sheet at full parity, referencing the canonical component schemas so canonical identity, provider authority, local-versus-provider state, linked context, the side-effect preview, and publish-later continuity stay one truth across every claimed work-item surface rather than being re-worded in prose",
        "evidence:m5-work-item-consumer-help:001",
        vec![
            binding(
                Family::WorkItemRow,
                vec![case(
                    Consumer::Help,
                    Family::WorkItemRow,
                    Health::FullParity,
                    &[],
                    "help work-item row at full parity",
                )],
            ),
            binding(
                Family::WorkItemDetailHeader,
                vec![case(
                    Consumer::Help,
                    Family::WorkItemDetailHeader,
                    Health::FullParity,
                    &[],
                    "help detail header at full parity",
                )],
            ),
            binding(
                Family::StatusTransitionSheet,
                vec![case(
                    Consumer::Help,
                    Family::StatusTransitionSheet,
                    Health::FullParity,
                    &[],
                    "help status-transition sheet at full parity",
                )],
            ),
        ],
    ));

    // 6. Support / export desk — all eight families, referencing the canonical schemas so its
    //    prose can never drift from the product truth. This is the authoritative rendering
    //    every other surface keeps parity with.
    rows.push(base_row(
        Consumer::Support,
        "Support / export desk surface owner",
        "The support / export desk adopts the work-item row, provider-chip group, relation strip, sync-pending pill, detail header, status-transition sheet, related-evidence card, and offline-handoff-packet card, referencing the canonical component schemas so its prose can never drift from the product truth and keeping canonical identity, provider authority, local-versus-provider state, linked context, the side-effect preview, and publish-later continuity exact in every exported case",
        "evidence:m5-work-item-consumer-support:001",
        vec![
            binding(
                Family::WorkItemRow,
                vec![case(
                    Consumer::Support,
                    Family::WorkItemRow,
                    Health::FullParity,
                    &[],
                    "support work-item row at full parity",
                )],
            ),
            binding(
                Family::ProviderChipGroup,
                vec![case(
                    Consumer::Support,
                    Family::ProviderChipGroup,
                    Health::FullParity,
                    &[],
                    "support provider-chip group at full parity",
                )],
            ),
            binding(
                Family::RelationStrip,
                vec![case(
                    Consumer::Support,
                    Family::RelationStrip,
                    Health::FullParity,
                    &[],
                    "support relation strip at full parity",
                )],
            ),
            binding(
                Family::SyncPendingPill,
                vec![case(
                    Consumer::Support,
                    Family::SyncPendingPill,
                    Health::FullParity,
                    &[],
                    "support sync-pending pill at full parity",
                )],
            ),
            binding(
                Family::WorkItemDetailHeader,
                vec![case(
                    Consumer::Support,
                    Family::WorkItemDetailHeader,
                    Health::FullParity,
                    &[],
                    "support detail header at full parity",
                )],
            ),
            binding(
                Family::StatusTransitionSheet,
                vec![case(
                    Consumer::Support,
                    Family::StatusTransitionSheet,
                    Health::FullParity,
                    &[],
                    "support status-transition sheet at full parity",
                )],
            ),
            binding(
                Family::RelatedEvidenceCard,
                vec![case(
                    Consumer::Support,
                    Family::RelatedEvidenceCard,
                    Health::FullParity,
                    &[],
                    "support related-evidence card at full parity",
                )],
            ),
            binding(
                Family::OfflineHandoffPacketCard,
                vec![case(
                    Consumer::Support,
                    Family::OfflineHandoffPacketCard,
                    Health::FullParity,
                    &[],
                    "support offline-handoff-packet card at full parity",
                )],
            ),
        ],
    ));

    // 7. Offline export packet — the provider-chip group and related-evidence card at full
    //    parity, plus the offline-handoff-packet card auto-narrowed because the exported packet
    //    stays local-only until it is published, so it never implies provider acceptance.
    rows.push(base_row(
        Consumer::Export,
        "Offline export packet surface owner",
        "The offline export packet adopts the provider-chip group and related-evidence card at full parity, and the offline-handoff-packet card auto-narrowed because the exported packet stays local-only until published, keeping canonical identity, provider authority, local-versus-provider state, linked context, the side-effect preview, and publish-later continuity explicit so an exported packet never implies provider acceptance",
        "evidence:m5-work-item-consumer-export:001",
        vec![
            binding(
                Family::ProviderChipGroup,
                vec![case(
                    Consumer::Export,
                    Family::ProviderChipGroup,
                    Health::FullParity,
                    &[],
                    "export provider-chip group at full parity",
                )],
            ),
            binding(
                Family::RelatedEvidenceCard,
                vec![case(
                    Consumer::Export,
                    Family::RelatedEvidenceCard,
                    Health::FullParity,
                    &[],
                    "export related-evidence card at full parity",
                )],
            ),
            binding(
                Family::OfflineHandoffPacketCard,
                vec![case(
                    Consumer::Export,
                    Family::OfflineHandoffPacketCard,
                    Health::OfflineHandoffNarrowed,
                    &[Caveat::OfflineHandoffLocalOnly],
                    "export offline-handoff-packet card narrowed by local-only packet",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5WorkItemComponentConsumerGovernanceReview {
    M5WorkItemComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        identity_authority_state_context_side_effect_publish_later_explicit_on_every_surface: true,
        degraded_state_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        queued_or_offline_state_never_shown_as_committed: true,
        no_generic_ticket_wording_conceals_provider_or_queued_state: true,
        help_support_export_present_same_work_item_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5WorkItemComponentConsumerProjection {
    M5WorkItemComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        canonical_identity_reads_single_source: true,
        provider_authority_reads_single_source: true,
        local_versus_provider_state_reads_single_source: true,
        linked_engineering_context_reads_single_source: true,
        publish_later_continuity_reads_single_source: true,
    }
}

fn proof_freshness() -> M5WorkItemComponentConsumerProofFreshness {
    M5WorkItemComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5WorkItemComponentConsumerReleasePosture {
    M5WorkItemComponentConsumerReleasePosture {
        release_packet_ref: M5_WORK_ITEM_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        work_item_consumer_audit_ref: M5_WORK_ITEM_COMPONENT_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_WORK_ITEM_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_CONSUMER_DOC_REF,
        M5_WORK_ITEM_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_WORK_ITEM_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        family_canonical_schema_ref(M5WorkItemComponentFamily::WorkItemRow),
        family_canonical_schema_ref(M5WorkItemComponentFamily::RelationStrip),
        family_canonical_schema_ref(M5WorkItemComponentFamily::WorkItemDetailHeader),
        family_canonical_schema_ref(M5WorkItemComponentFamily::RelatedEvidenceCard),
    ])
}

/// Builds the canonical M5 work-item component-consumer packet.
pub fn seeded_m5_work_item_component_consumer_packet() -> M5WorkItemComponentConsumerPacket {
    M5WorkItemComponentConsumerPacket::new(M5WorkItemComponentConsumerPacketInput {
        packet_id: M5_WORK_ITEM_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 work-item component consumers: the issue inbox, work-item detail, review workspace, incident workspace, Help / docs, the support / export desk, and the offline export packet keep provider, freshness, and offline-handoff parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5WorkItemComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the incident workspace is held at Beta because a slice of incident
/// renderings still resolve a local-only offline-handoff packet; every consumer stays visible.
pub fn seeded_m5_work_item_component_consumer_incident_beta_narrowed(
) -> M5WorkItemComponentConsumerPacket {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.packet_id = "m5-work-item-component-consumer:incident-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5WorkItemComponentConsumer::Incident)
        .expect("incident row present");
    row.qualification = M5WorkItemQualificationClass::Beta;
    packet
}

/// Narrowed variant: the review workspace is held at Preview because a slice of review
/// renderings still resolve a limited provider scope; every consumer stays visible.
pub fn seeded_m5_work_item_component_consumer_review_preview_narrowed(
) -> M5WorkItemComponentConsumerPacket {
    let mut packet = seeded_m5_work_item_component_consumer_packet();
    packet.packet_id = "m5-work-item-component-consumer:review-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5WorkItemComponentConsumer::Review)
        .expect("review row present");
    row.qualification = M5WorkItemQualificationClass::Preview;
    packet
}
