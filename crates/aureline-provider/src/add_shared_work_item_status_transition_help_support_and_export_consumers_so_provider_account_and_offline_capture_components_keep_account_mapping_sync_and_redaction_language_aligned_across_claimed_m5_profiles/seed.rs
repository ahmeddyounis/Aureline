//! Canonical seed builders for the M5 provider-account / offline-capture component-consumer
//! lane.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked bindings, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical provider-account / offline-capture component-consumer
/// packet.
pub const M5_PROVIDER_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-provider-account-offline-capture-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5ProviderComponentConsumer,
    component_family: M5ProviderAccountOfflineComponentFamily,
    parity_health: M5ProviderConsumerParityHealth,
    export_caveats: &[M5ProviderConsumerExportCaveat],
    note: &str,
) -> M5ProviderComponentBindingCase {
    M5ProviderComponentBindingCase::resolved(M5ProviderComponentBindingInput {
        consumer,
        component_family,
        descriptor_families: M5ProviderComponentDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5ProviderAccountOfflineComponentFamily,
    example_bindings: Vec<M5ProviderComponentBindingCase>,
) -> M5ProviderComponentBinding {
    M5ProviderComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5ProviderComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5ProviderComponentBinding>,
) -> M5ProviderComponentConsumerRow {
    M5ProviderComponentConsumerRow {
        consumer,
        qualification: M5ProviderQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ProviderSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ProviderDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5ProviderConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5ProviderComponentDescriptor::ALL.to_vec(),
        parity_health_modes: M5ProviderConsumerParityHealth::ALL.to_vec(),
        export_caveats: M5ProviderConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5ProviderClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5ProviderConsumerNarrowingReason::ALL.to_vec(),
        recovery_actions: M5ProviderConsumerRecoveryAction::ALL.to_vec(),
        export_fields: M5ProviderConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5ProviderAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ProviderConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ProviderDowngradeTrigger::ConnectionStateUnstated,
            M5ProviderDowngradeTrigger::MappingOriginUnstated,
            M5ProviderDowngradeTrigger::SyncModeUnstated,
            M5ProviderDowngradeTrigger::QueuedDraftStateHidden,
            M5ProviderDowngradeTrigger::RedactionClassUnstated,
            M5ProviderDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_PROVIDER_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_PROVIDER_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_provider_grammar: false,
        drops_account_mapping_queue_or_redaction_when_narrowed: false,
        shows_cached_or_offline_state_as_committed: false,
        inherits_stronger_label_from_healthier_profile: false,
    }
}

// Sequential pushes preserve the numbered consumer-matrix narrative below.
#[allow(clippy::vec_init_then_push)]
fn consumer_rows() -> Vec<M5ProviderComponentConsumerRow> {
    use M5ProviderAccountOfflineComponentFamily as Family;
    use M5ProviderComponentConsumer as Consumer;
    use M5ProviderConsumerExportCaveat as Caveat;
    use M5ProviderConsumerParityHealth as Health;

    let mut rows = Vec::new();

    // 1. Work-item detail — the provider-account row and project/board mapping row at full
    //    parity (which account, where a publish lands), plus the sync-behavior row
    //    auto-narrowed because provider scope is limited on this surface, so writes are
    //    read-or-limited-write rather than full.
    rows.push(base_row(
        Consumer::WorkItemDetail,
        "Work-item detail surface owner",
        "Work-item detail adopts the provider-account row and project/board mapping row at full parity, pointing at the canonical component schemas so account state, destination mapping, queued-draft state, and redaction posture match what status-transition review, issue intake, Help / docs, the export desk, and browser handoff read; the sync-behavior row auto-narrows while provider scope is limited",
        "evidence:m5-provider-consumer-work-item-detail:001",
        vec![
            binding(
                Family::ProviderAccountRow,
                vec![case(
                    Consumer::WorkItemDetail,
                    Family::ProviderAccountRow,
                    Health::FullParity,
                    &[],
                    "work-item detail provider-account row at full parity",
                )],
            ),
            binding(
                Family::ProjectOrBoardMappingRow,
                vec![case(
                    Consumer::WorkItemDetail,
                    Family::ProjectOrBoardMappingRow,
                    Health::FullParity,
                    &[],
                    "work-item detail mapping row at full parity",
                )],
            ),
            binding(
                Family::SyncBehaviorRow,
                vec![case(
                    Consumer::WorkItemDetail,
                    Family::SyncBehaviorRow,
                    Health::ScopeLimitedNarrowed,
                    &[Caveat::ScopeLimitedReadOnly],
                    "work-item detail sync-behavior row narrowed by limited provider scope",
                )],
            ),
        ],
    ));

    // 2. Status-transition review — the sync-behavior row at full parity, plus the
    //    provider-account row auto-narrowed by a stale session (a cached read, not committed
    //    state) and the project/board mapping row auto-narrowed because the mapping is
    //    policy-locked here.
    rows.push(base_row(
        Consumer::StatusTransitionReview,
        "Status-transition review surface owner",
        "Status-transition review adopts the sync-behavior row at full parity, the provider-account row auto-narrowed by a stale session so the read is not shown as provider-committed, and the project/board mapping row auto-narrowed because the mapping is policy-locked, keeping account state, destination mapping, queued-draft state, and redaction posture explicit so a transition never publishes into an unstated destination",
        "evidence:m5-provider-consumer-status-transition-review:001",
        vec![
            binding(
                Family::ProviderAccountRow,
                vec![case(
                    Consumer::StatusTransitionReview,
                    Family::ProviderAccountRow,
                    Health::SessionStaleNarrowed,
                    &[Caveat::SessionStaleCachedRead],
                    "status-transition review provider-account row narrowed by stale session",
                )],
            ),
            binding(
                Family::SyncBehaviorRow,
                vec![case(
                    Consumer::StatusTransitionReview,
                    Family::SyncBehaviorRow,
                    Health::FullParity,
                    &[],
                    "status-transition review sync-behavior row at full parity",
                )],
            ),
            binding(
                Family::ProjectOrBoardMappingRow,
                vec![case(
                    Consumer::StatusTransitionReview,
                    Family::ProjectOrBoardMappingRow,
                    Health::MappingPolicyLockedNarrowed,
                    &[Caveat::MappingPolicyLockedNoPublish],
                    "status-transition review mapping row narrowed by policy-locked mapping",
                )],
            ),
        ],
    ));

    // 3. Issue intake — the provider-account row and project/board mapping row at full
    //    parity, plus the offline-capture row auto-narrowed because the captured packet
    //    remains local-only and is not provider-committed yet.
    rows.push(base_row(
        Consumer::IssueIntake,
        "Issue-intake surface owner",
        "Issue intake adopts the provider-account row and project/board mapping row at full parity, and the offline-capture row auto-narrowed because the captured packet remains local-only, keeping account state, destination mapping, queued-draft state, and redaction posture disclosed so a locally-captured issue never masquerades as a provider-committed one",
        "evidence:m5-provider-consumer-issue-intake:001",
        vec![
            binding(
                Family::ProviderAccountRow,
                vec![case(
                    Consumer::IssueIntake,
                    Family::ProviderAccountRow,
                    Health::FullParity,
                    &[],
                    "issue intake provider-account row at full parity",
                )],
            ),
            binding(
                Family::ProjectOrBoardMappingRow,
                vec![case(
                    Consumer::IssueIntake,
                    Family::ProjectOrBoardMappingRow,
                    Health::FullParity,
                    &[],
                    "issue intake mapping row at full parity",
                )],
            ),
            binding(
                Family::OfflineCaptureRow,
                vec![case(
                    Consumer::IssueIntake,
                    Family::OfflineCaptureRow,
                    Health::PacketLocalOnlyNarrowed,
                    &[Caveat::PacketLocalOnlyNotCommitted],
                    "issue intake offline-capture row narrowed by local-only packet",
                )],
            ),
        ],
    ));

    // 4. Help / docs — the provider-account row, privacy/redaction row, and sync-behavior row
    //    all at full parity: documentation describes the same account, redaction, and sync
    //    truth the product renders.
    rows.push(base_row(
        Consumer::DocsHelp,
        "Help / docs surface owner",
        "Help / docs adopt the provider-account row, privacy/redaction row, and sync-behavior row at full parity, referencing the canonical component schemas so account state, destination mapping, queued-draft state, and redaction posture stay one truth across every claimed provider surface rather than being re-worded in prose",
        "evidence:m5-provider-consumer-docs-help:001",
        vec![
            binding(
                Family::ProviderAccountRow,
                vec![case(
                    Consumer::DocsHelp,
                    Family::ProviderAccountRow,
                    Health::FullParity,
                    &[],
                    "docs / help provider-account row at full parity",
                )],
            ),
            binding(
                Family::PrivacyRedactionRow,
                vec![case(
                    Consumer::DocsHelp,
                    Family::PrivacyRedactionRow,
                    Health::FullParity,
                    &[],
                    "docs / help privacy/redaction row at full parity",
                )],
            ),
            binding(
                Family::SyncBehaviorRow,
                vec![case(
                    Consumer::DocsHelp,
                    Family::SyncBehaviorRow,
                    Health::FullParity,
                    &[],
                    "docs / help sync-behavior row at full parity",
                )],
            ),
        ],
    ));

    // 5. Support / export desk — all five families, referencing the canonical schemas so its
    //    prose can never drift from the product truth. This is the authoritative rendering
    //    every other surface keeps parity with.
    rows.push(base_row(
        Consumer::SupportExport,
        "Support / export desk surface owner",
        "The support / export desk adopts the provider-account row, project/board mapping row, sync-behavior row, offline-capture row, and privacy/redaction row, referencing the canonical component schemas so its prose can never drift from the product truth and keeping account state, destination mapping, queued-draft state, and redaction posture exact in every exported case",
        "evidence:m5-provider-consumer-support-export:001",
        vec![
            binding(
                Family::ProviderAccountRow,
                vec![case(
                    Consumer::SupportExport,
                    Family::ProviderAccountRow,
                    Health::FullParity,
                    &[],
                    "support / export provider-account row at full parity",
                )],
            ),
            binding(
                Family::ProjectOrBoardMappingRow,
                vec![case(
                    Consumer::SupportExport,
                    Family::ProjectOrBoardMappingRow,
                    Health::FullParity,
                    &[],
                    "support / export mapping row at full parity",
                )],
            ),
            binding(
                Family::SyncBehaviorRow,
                vec![case(
                    Consumer::SupportExport,
                    Family::SyncBehaviorRow,
                    Health::FullParity,
                    &[],
                    "support / export sync-behavior row at full parity",
                )],
            ),
            binding(
                Family::OfflineCaptureRow,
                vec![case(
                    Consumer::SupportExport,
                    Family::OfflineCaptureRow,
                    Health::FullParity,
                    &[],
                    "support / export offline-capture row at full parity",
                )],
            ),
            binding(
                Family::PrivacyRedactionRow,
                vec![case(
                    Consumer::SupportExport,
                    Family::PrivacyRedactionRow,
                    Health::FullParity,
                    &[],
                    "support / export privacy/redaction row at full parity",
                )],
            ),
        ],
    ));

    // 6. Browser handoff — the privacy/redaction row at full parity, plus the provider-account
    //    row auto-narrowed by a stale session mid-handoff and the offline-capture row
    //    auto-narrowed because captured drafts stay local-only until the handoff completes.
    rows.push(base_row(
        Consumer::BrowserHandoff,
        "Browser-handoff surface owner",
        "Browser handoff adopts the privacy/redaction row at full parity, the provider-account row auto-narrowed by a stale session mid-handoff, and the offline-capture row auto-narrowed because captured drafts stay local-only until the handoff completes, keeping account state, destination mapping, queued-draft state, and redaction posture explicit so a mid-handoff cached read never masquerades as provider-committed state",
        "evidence:m5-provider-consumer-browser-handoff:001",
        vec![
            binding(
                Family::ProviderAccountRow,
                vec![case(
                    Consumer::BrowserHandoff,
                    Family::ProviderAccountRow,
                    Health::SessionStaleNarrowed,
                    &[Caveat::SessionStaleCachedRead],
                    "browser handoff provider-account row narrowed by stale session",
                )],
            ),
            binding(
                Family::OfflineCaptureRow,
                vec![case(
                    Consumer::BrowserHandoff,
                    Family::OfflineCaptureRow,
                    Health::PacketLocalOnlyNarrowed,
                    &[Caveat::PacketLocalOnlyNotCommitted],
                    "browser handoff offline-capture row narrowed by local-only packet",
                )],
            ),
            binding(
                Family::PrivacyRedactionRow,
                vec![case(
                    Consumer::BrowserHandoff,
                    Family::PrivacyRedactionRow,
                    Health::FullParity,
                    &[],
                    "browser handoff privacy/redaction row at full parity",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5ProviderComponentConsumerGovernanceReview {
    M5ProviderComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        account_mapping_queue_redaction_explicit_on_every_surface: true,
        degraded_state_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        cached_or_offline_state_never_shown_as_committed: true,
        support_export_presents_same_account_and_redaction_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ProviderComponentConsumerProjection {
    M5ProviderComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        account_state_reads_single_source: true,
        destination_mapping_reads_single_source: true,
        queued_draft_state_reads_single_source: true,
        redaction_posture_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ProviderComponentConsumerProofFreshness {
    M5ProviderComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ProviderComponentConsumerReleasePosture {
    M5ProviderComponentConsumerReleasePosture {
        release_packet_ref: M5_PROVIDER_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        provider_account_consumer_audit_ref: M5_PROVIDER_COMPONENT_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_PROVIDER_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_PROVIDER_COMPONENT_CONSUMER_DOC_REF,
        M5_PROVIDER_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_PROVIDER_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        family_canonical_schema_ref(M5ProviderAccountOfflineComponentFamily::ProviderAccountRow),
        family_canonical_schema_ref(
            M5ProviderAccountOfflineComponentFamily::ProjectOrBoardMappingRow,
        ),
        family_canonical_schema_ref(M5ProviderAccountOfflineComponentFamily::OfflineCaptureRow),
    ])
}

/// Builds the canonical M5 provider-account / offline-capture component-consumer packet.
pub fn seeded_m5_provider_component_consumer_packet() -> M5ProviderComponentConsumerPacket {
    M5ProviderComponentConsumerPacket::new(M5ProviderComponentConsumerPacketInput {
        packet_id: M5_PROVIDER_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 provider-account / offline-capture component consumers: work-item detail, status-transition review, issue intake, Help / docs, the export desk, and browser handoff keep account, mapping, sync, queue, and redaction parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5ProviderComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the browser-handoff flow is held at Beta because a slice of mid-handoff
/// renderings still resolve a stale session; every consumer stays visible.
pub fn seeded_m5_provider_component_consumer_browser_handoff_beta_narrowed(
) -> M5ProviderComponentConsumerPacket {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.packet_id =
        "m5-provider-account-offline-capture-component-consumer:browser-handoff-beta:0001"
            .to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5ProviderComponentConsumer::BrowserHandoff)
        .expect("browser-handoff row present");
    row.qualification = M5ProviderQualificationClass::Beta;
    packet
}

/// Narrowed variant: the issue-intake surface is held at Preview because a slice of intake
/// renderings still capture packets local-only; every consumer stays visible.
pub fn seeded_m5_provider_component_consumer_issue_intake_preview_narrowed(
) -> M5ProviderComponentConsumerPacket {
    let mut packet = seeded_m5_provider_component_consumer_packet();
    packet.packet_id =
        "m5-provider-account-offline-capture-component-consumer:issue-intake-preview:0001"
            .to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5ProviderComponentConsumer::IssueIntake)
        .expect("issue-intake row present");
    row.qualification = M5ProviderQualificationClass::Preview;
    packet
}
