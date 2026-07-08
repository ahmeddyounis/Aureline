//! Canonical seed builders for the M5 shared-state-taxonomy component-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked bindings, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical state-component-consumer packet.
pub const M5_STATE_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-shared-state-taxonomy-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5StateComponentConsumer,
    component_family: M5SharedComponentStateFamily,
    parity_health: M5StateConsumerParityHealth,
    export_caveats: &[M5StateConsumerExportCaveat],
    note: &str,
) -> M5StateComponentBindingCase {
    M5StateComponentBindingCase::resolved(M5StateComponentBindingInput {
        consumer,
        component_family,
        descriptor_families: M5StateComponentDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5SharedComponentStateFamily,
    example_bindings: Vec<M5StateComponentBindingCase>,
) -> M5StateComponentBinding {
    M5StateComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5StateComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5StateComponentBinding>,
) -> M5StateComponentConsumerRow {
    M5StateComponentConsumerRow {
        consumer,
        qualification: M5ComponentStateQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ComponentStateSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ComponentStateDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5StateConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5StateComponentDescriptor::ALL.to_vec(),
        parity_health_modes: M5StateConsumerParityHealth::ALL.to_vec(),
        export_caveats: M5StateConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5StateClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5StateConsumerNarrowingReason::ALL.to_vec(),
        recovery_actions: M5StateConsumerRecoveryAction::ALL.to_vec(),
        export_fields: M5StateConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5ComponentStateAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ComponentStateConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ComponentStateDowngradeTrigger::StateCauseUnstated,
            M5ComponentStateDowngradeTrigger::LockOwnerMasked,
            M5ComponentStateDowngradeTrigger::CurrentSelectedCollapsed,
            M5ComponentStateDowngradeTrigger::PendingShownAsLoading,
            M5ComponentStateDowngradeTrigger::ConsequenceOrRecoveryOmitted,
            M5ComponentStateDowngradeTrigger::ColorOnlyTreatment,
            M5ComponentStateDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_STATE_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_STATE_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_state_semantics_per_surface: false,
        invents_private_state_names: false,
        drops_cause_or_recovery_when_narrowed: false,
        shows_partial_state_as_exact: false,
        collapses_distinct_states_or_uses_color_only: false,
    }
}

fn consumer_rows() -> Vec<M5StateComponentConsumerRow> {
    use M5SharedComponentStateFamily as Family;
    use M5StateComponentConsumer as Consumer;
    use M5StateConsumerExportCaveat as Caveat;
    use M5StateConsumerParityHealth as Health;

    vec![
        // 1. Shell chrome — the interactive-state contract and the degraded-state-application contract
        //    at full parity, pointing at the canonical contract schemas so default/hover/focus and
        //    loading/pending/warning/degraded truth match what every other surface reads.
        base_row(
        Consumer::ShellChrome,
        "Shell chrome surface owner",
        "Shell chrome adopts the interactive-state contract and the degraded-state-application contract at full parity, referencing the canonical contract schemas so the same state semantics, state cause, consequence/recovery, and accessibility label appear here as in command/help, search, review, settings, provider, test, and support surfaces",
        "evidence:m5-state-consumer-shell-chrome:001",
        vec![
            binding(
                Family::InteractiveState,
                vec![case(
                    Consumer::ShellChrome,
                    Family::InteractiveState,
                    Health::FullParity,
                    &[],
                    "shell chrome interactive-state contract at full parity",
                )],
            ),
            binding(
                Family::DegradedStateApplication,
                vec![case(
                    Consumer::ShellChrome,
                    Family::DegradedStateApplication,
                    Health::FullParity,
                    &[],
                    "shell chrome degraded-state-application contract at full parity",
                )],
            ),
        ],
        ),

    // 2. Command / help — the shared taxonomy itself and the interactive-state contract at full
    //    parity: the command palette and Help pane describe the same canonical state names and
    //    interactive treatments the product renders.
        base_row(
        Consumer::CommandHelp,
        "Command / help surface owner",
        "Command and help surfaces adopt the shared component-state taxonomy and the interactive-state contract at full parity, referencing the canonical schemas so state semantics, state cause, consequence/recovery, and accessibility label stay one truth rather than being re-worded in prose",
        "evidence:m5-state-consumer-command-help:001",
        vec![
            binding(
                Family::SharedComponentStateTaxonomy,
                vec![case(
                    Consumer::CommandHelp,
                    Family::SharedComponentStateTaxonomy,
                    Health::FullParity,
                    &[],
                    "command/help shared taxonomy at full parity",
                )],
            ),
            binding(
                Family::InteractiveState,
                vec![case(
                    Consumer::CommandHelp,
                    Family::InteractiveState,
                    Health::FullParity,
                    &[],
                    "command/help interactive-state contract at full parity",
                )],
            ),
        ],
        ),

    // 3. Search / dense collection — the selection-or-lock-state contract at full parity, plus the
    //    interactive-state contract auto-narrowed because a non-visual accessibility route is
    //    reduced here and falls back to the full accessible state description.
        base_row(
        Consumer::SearchDenseCollection,
        "Search / dense-collection surface owner",
        "Search and dense collections adopt the selection-or-lock-state contract at full parity and the interactive-state contract auto-narrowed because a non-visual accessibility route is reduced here, keeping state semantics, state cause, consequence/recovery, and accessibility label explicit so a reduced route never becomes a color-only cue",
        "evidence:m5-state-consumer-search-dense:001",
        vec![
            binding(
                Family::SelectionOrLockState,
                vec![case(
                    Consumer::SearchDenseCollection,
                    Family::SelectionOrLockState,
                    Health::FullParity,
                    &[],
                    "search dense-collection selection-or-lock-state contract at full parity",
                )],
            ),
            binding(
                Family::InteractiveState,
                vec![case(
                    Consumer::SearchDenseCollection,
                    Family::InteractiveState,
                    Health::AccessibilityRouteReducedNarrowed,
                    &[Caveat::AccessibilityRouteReducedFallback],
                    "search dense-collection interactive-state narrowed by reduced accessibility route",
                )],
            ),
        ],
        ),

    // 4. Review / work-item — the selection-or-lock-state contract at full parity, plus the
    //    degraded-state-application contract auto-narrowed because a state cause is not yet
    //    resolved and is disclosed as unexplained rather than asserted as a settled, exact state.
        base_row(
        Consumer::ReviewWorkItem,
        "Review / work-item surface owner",
        "Review and work-item flows adopt the selection-or-lock-state contract at full parity and the degraded-state-application contract auto-narrowed because a state cause is not yet resolved, keeping state semantics, state cause, consequence/recovery, and accessibility label explicit so an unexplained state never reads as a settled, exact state",
        "evidence:m5-state-consumer-review-work-item:001",
        vec![
            binding(
                Family::SelectionOrLockState,
                vec![case(
                    Consumer::ReviewWorkItem,
                    Family::SelectionOrLockState,
                    Health::FullParity,
                    &[],
                    "review work-item selection-or-lock-state contract at full parity",
                )],
            ),
            binding(
                Family::DegradedStateApplication,
                vec![case(
                    Consumer::ReviewWorkItem,
                    Family::DegradedStateApplication,
                    Health::StateCauseUnresolvedNarrowed,
                    &[Caveat::StateCauseUnresolvedNotExplained],
                    "review work-item degraded-state narrowed by unresolved state cause",
                )],
            ),
        ],
        ),

    // 5. Settings / capability — the selection-or-lock-state contract auto-narrowed because a
    //    lock / block owner is re-resolved (a locked capability names its owner rather than masking
    //    as a plain disabled control), plus the degraded-state-application contract at full parity.
        base_row(
        Consumer::SettingsCapability,
        "Settings / capability surface owner",
        "Settings and capability prompts adopt the selection-or-lock-state contract auto-narrowed because a lock / block owner is re-resolved, and the degraded-state-application contract at full parity, keeping state semantics, state cause, consequence/recovery, and accessibility label explicit so a locked capability names its owner rather than masking as a plain disabled control",
        "evidence:m5-state-consumer-settings-capability:001",
        vec![
            binding(
                Family::SelectionOrLockState,
                vec![case(
                    Consumer::SettingsCapability,
                    Family::SelectionOrLockState,
                    Health::LockOwnerUnresolvedNarrowed,
                    &[Caveat::LockOwnerReassigned],
                    "settings capability selection-or-lock-state narrowed by re-resolved lock owner",
                )],
            ),
            binding(
                Family::DegradedStateApplication,
                vec![case(
                    Consumer::SettingsCapability,
                    Family::DegradedStateApplication,
                    Health::FullParity,
                    &[],
                    "settings capability degraded-state-application contract at full parity",
                )],
            ),
        ],
        ),

    // 6. Provider / offline-capture — the selection-or-lock-state contract at full parity, plus the
    //    degraded-state-application contract auto-narrowed because no recovery path is available and
    //    the state renders as degraded with reduced capability instead of a healthy exact state.
        base_row(
        Consumer::ProviderOfflineCapture,
        "Provider / offline-capture surface owner",
        "Provider and offline-capture rows adopt the selection-or-lock-state contract at full parity and the degraded-state-application contract auto-narrowed because no recovery path is available, keeping state semantics, state cause, consequence/recovery, and accessibility label explicit so a degraded row names its reduced capability rather than a healthy exact state",
        "evidence:m5-state-consumer-provider-offline-capture:001",
        vec![
            binding(
                Family::SelectionOrLockState,
                vec![case(
                    Consumer::ProviderOfflineCapture,
                    Family::SelectionOrLockState,
                    Health::FullParity,
                    &[],
                    "provider offline-capture selection-or-lock-state contract at full parity",
                )],
            ),
            binding(
                Family::DegradedStateApplication,
                vec![case(
                    Consumer::ProviderOfflineCapture,
                    Family::DegradedStateApplication,
                    Health::RecoveryUnavailableNarrowed,
                    &[Caveat::RecoveryUnavailableDegraded],
                    "provider offline-capture degraded-state narrowed by unavailable recovery",
                )],
            ),
        ],
        ),

    // 7. Test / watch — the degraded-state-application contract and the interactive-state contract
    //    at full parity: a test run's loading/pending/warning/degraded posture and its interactive
    //    controls read the same canonical truth the product renders.
        base_row(
        Consumer::TestWatch,
        "Test / watch surface owner",
        "Test and watch surfaces adopt the degraded-state-application contract and the interactive-state contract at full parity, referencing the canonical schemas so state semantics, state cause, consequence/recovery, and accessibility label stay one truth across every claimed surface",
        "evidence:m5-state-consumer-test-watch:001",
        vec![
            binding(
                Family::DegradedStateApplication,
                vec![case(
                    Consumer::TestWatch,
                    Family::DegradedStateApplication,
                    Health::FullParity,
                    &[],
                    "test watch degraded-state-application contract at full parity",
                )],
            ),
            binding(
                Family::InteractiveState,
                vec![case(
                    Consumer::TestWatch,
                    Family::InteractiveState,
                    Health::FullParity,
                    &[],
                    "test watch interactive-state contract at full parity",
                )],
            ),
        ],
        ),

    // 8. Support / recovery — all four families, referencing the canonical schemas so its exported
    //    prose can never drift from the product truth. This is the authoritative rendering every
    //    other surface keeps parity with.
        base_row(
        Consumer::SupportRecovery,
        "Support / recovery surface owner",
        "The support / recovery lane adopts the shared component-state taxonomy, the interactive-state contract, the selection-or-lock-state contract, and the degraded-state-application contract, referencing the canonical schemas so its exported prose can never drift from the product truth and keeping state semantics, state cause, consequence/recovery, and accessibility label exact in every exported case",
        "evidence:m5-state-consumer-support-recovery:001",
        vec![
            binding(
                Family::SharedComponentStateTaxonomy,
                vec![case(
                    Consumer::SupportRecovery,
                    Family::SharedComponentStateTaxonomy,
                    Health::FullParity,
                    &[],
                    "support recovery shared taxonomy at full parity",
                )],
            ),
            binding(
                Family::InteractiveState,
                vec![case(
                    Consumer::SupportRecovery,
                    Family::InteractiveState,
                    Health::FullParity,
                    &[],
                    "support recovery interactive-state contract at full parity",
                )],
            ),
            binding(
                Family::SelectionOrLockState,
                vec![case(
                    Consumer::SupportRecovery,
                    Family::SelectionOrLockState,
                    Health::FullParity,
                    &[],
                    "support recovery selection-or-lock-state contract at full parity",
                )],
            ),
            binding(
                Family::DegradedStateApplication,
                vec![case(
                    Consumer::SupportRecovery,
                    Family::DegradedStateApplication,
                    Health::FullParity,
                    &[],
                    "support recovery degraded-state-application contract at full parity",
                )],
            ),
        ],
    ),
    ]
}

fn governance_review() -> M5StateComponentConsumerGovernanceReview {
    M5StateComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_private_state_names: true,
        state_cause_recovery_accessibility_explicit_on_every_surface: true,
        degraded_state_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        partial_state_never_shown_as_exact: true,
        support_docs_present_same_state_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5StateComponentConsumerProjection {
    M5StateComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        state_semantics_reads_single_source: true,
        state_cause_reads_single_source: true,
        consequence_and_recovery_reads_single_source: true,
        accessibility_label_reads_single_source: true,
    }
}

fn proof_freshness() -> M5StateComponentConsumerProofFreshness {
    M5StateComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5StateComponentConsumerReleasePosture {
    M5StateComponentConsumerReleasePosture {
        release_packet_ref: M5_STATE_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        state_component_consumer_audit_ref: M5_STATE_COMPONENT_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_STATE_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_STATE_COMPONENT_CONSUMER_DOC_REF,
        M5_STATE_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_STATE_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        M5_STATE_COMPONENT_CONSUMER_STATE_CLASS_REF,
        M5_STATE_COMPONENT_CONSUMER_STATE_RECOVERY_REF,
        family_canonical_schema_ref(M5SharedComponentStateFamily::InteractiveState),
        family_canonical_schema_ref(M5SharedComponentStateFamily::SelectionOrLockState),
        family_canonical_schema_ref(M5SharedComponentStateFamily::DegradedStateApplication),
    ])
}

/// Builds the canonical M5 shared-state-taxonomy component-consumer packet.
pub fn seeded_m5_state_component_consumer_packet() -> M5StateComponentConsumerPacket {
    M5StateComponentConsumerPacket::new(M5StateComponentConsumerPacketInput {
        packet_id: M5_STATE_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 shared-state-taxonomy component consumers: shell chrome, command/help, search/dense collections, review/work-item, settings/capability, provider/offline-capture, test/watch, and support/recovery keep state-semantics, cause, consequence/recovery, and accessibility parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5StateComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the provider / offline-capture consumer is held at Beta because a slice of
/// its degraded rows still has no recovery path; every consumer stays visible.
pub fn seeded_m5_state_component_consumer_provider_offline_capture_beta_narrowed(
) -> M5StateComponentConsumerPacket {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.packet_id =
        "m5-shared-state-taxonomy-component-consumer:provider-offline-capture-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5StateComponentConsumer::ProviderOfflineCapture)
        .expect("provider-offline-capture row present");
    row.qualification = M5ComponentStateQualificationClass::Beta;
    packet
}

/// Narrowed variant: the test / watch consumer is held at Preview because a slice of its watch
/// surfaces still leaves a state cause unresolved; every consumer stays visible.
pub fn seeded_m5_state_component_consumer_test_watch_preview_narrowed(
) -> M5StateComponentConsumerPacket {
    let mut packet = seeded_m5_state_component_consumer_packet();
    packet.packet_id =
        "m5-shared-state-taxonomy-component-consumer:test-watch-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5StateComponentConsumer::TestWatch)
        .expect("test-watch row present");
    row.qualification = M5ComponentStateQualificationClass::Preview;
    packet
}
