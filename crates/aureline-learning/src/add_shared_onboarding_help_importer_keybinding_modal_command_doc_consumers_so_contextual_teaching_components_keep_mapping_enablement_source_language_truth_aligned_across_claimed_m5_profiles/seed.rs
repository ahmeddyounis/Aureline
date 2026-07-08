//! Canonical seed builders for the M5 contextual-teaching component-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked bindings, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical contextual-teaching component-consumer packet.
pub const M5_TEACHING_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-contextual-teaching-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5TeachingComponentConsumer,
    component_family: M5ContextualTeachingComponentFamily,
    parity_health: M5TeachingConsumerParityHealth,
    export_caveats: &[M5TeachingConsumerExportCaveat],
    note: &str,
) -> M5TeachingComponentBindingCase {
    M5TeachingComponentBindingCase::resolved(M5TeachingComponentBindingInput {
        consumer,
        component_family,
        descriptor_families: M5TeachingComponentDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5ContextualTeachingComponentFamily,
    example_bindings: Vec<M5TeachingComponentBindingCase>,
) -> M5TeachingComponentBinding {
    M5TeachingComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5TeachingComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5TeachingComponentBinding>,
) -> M5TeachingComponentConsumerRow {
    M5TeachingComponentConsumerRow {
        consumer,
        qualification: M5TeachingQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5TeachingSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5TeachingDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5TeachingConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5TeachingComponentDescriptor::ALL.to_vec(),
        parity_health_modes: M5TeachingConsumerParityHealth::ALL.to_vec(),
        export_caveats: M5TeachingConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5TeachingClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5TeachingConsumerNarrowingReason::ALL.to_vec(),
        recovery_actions: M5TeachingConsumerRecoveryAction::ALL.to_vec(),
        export_fields: M5TeachingConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5TeachingAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5TeachingConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5TeachingDowngradeTrigger::TipCommandBindingUnstated,
            M5TeachingDowngradeTrigger::MigrationMappingUnstated,
            M5TeachingDowngradeTrigger::SequenceHelpStateUnstated,
            M5TeachingDowngradeTrigger::BlockedActionOwnerUnstated,
            M5TeachingDowngradeTrigger::SourceLanguageFallbackUnstated,
            M5TeachingDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_TEACHING_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_TEACHING_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_teaching_grammar: false,
        drops_command_mapping_owner_or_citation_when_narrowed: false,
        shows_partial_or_unsupported_state_as_exact: false,
        inherits_stronger_label_from_healthier_profile: false,
    }
}

fn consumer_rows() -> Vec<M5TeachingComponentConsumerRow> {
    use M5ContextualTeachingComponentFamily as Family;
    use M5TeachingComponentConsumer as Consumer;
    use M5TeachingConsumerExportCaveat as Caveat;
    use M5TeachingConsumerParityHealth as Health;

    let mut rows = Vec::new();

    // 1. First-run onboarding — the contextual-tip card and sequence-help strip at full parity,
    //    pointing at the canonical component schemas so command-binding, migration-mapping,
    //    blocked-action-explanation, and source-language-citation truth match what the importer,
    //    keybinding help, command docs, the Help pane, and the localized support packet read.
    rows.push(base_row(
        Consumer::OnboardingFlow,
        "First-run onboarding surface owner",
        "First-run onboarding adopts the contextual-tip card and sequence-help strip at full parity, referencing the canonical component schemas so the same command binding, migration mapping, blocked-action explanation, and source-language citation appear here as in the importer, keybinding help, command docs, the Help pane, and the localized support packet",
        "evidence:m5-teaching-consumer-onboarding:001",
        vec![
            binding(
                Family::ContextualTipCard,
                vec![case(
                    Consumer::OnboardingFlow,
                    Family::ContextualTipCard,
                    Health::FullParity,
                    &[],
                    "onboarding contextual-tip card at full parity",
                )],
            ),
            binding(
                Family::SequenceHelpStrip,
                vec![case(
                    Consumer::OnboardingFlow,
                    Family::SequenceHelpStrip,
                    Health::FullParity,
                    &[],
                    "onboarding sequence-help strip at full parity",
                )],
            ),
        ],
    ));

    // 2. Migration importer — the contextual-tip card at full parity, plus the migration-bridge
    //    card auto-narrowed because imported behavior is only partially mapped (not an exact
    //    native equivalent) and the why-unavailable explanation row auto-narrowed because the
    //    blocked-action owner changed after the import.
    rows.push(base_row(
        Consumer::MigrationImporter,
        "Migration importer surface owner",
        "The migration importer adopts the contextual-tip card at full parity, the migration-bridge card auto-narrowed because imported behavior is only partially mapped, and the why-unavailable explanation row auto-narrowed because the blocked-action owner changed, keeping command binding, migration mapping, blocked-action explanation, and source-language citation explicit so a partial import never reads as an exact native equivalent",
        "evidence:m5-teaching-consumer-migration-importer:001",
        vec![
            binding(
                Family::ContextualTipCard,
                vec![case(
                    Consumer::MigrationImporter,
                    Family::ContextualTipCard,
                    Health::FullParity,
                    &[],
                    "migration importer contextual-tip card at full parity",
                )],
            ),
            binding(
                Family::MigrationBridgeCard,
                vec![case(
                    Consumer::MigrationImporter,
                    Family::MigrationBridgeCard,
                    Health::ImportedBehaviorPartialNarrowed,
                    &[Caveat::ImportedBehaviorPartialNotExact],
                    "migration importer migration-bridge card narrowed by partial import",
                )],
            ),
            binding(
                Family::WhyUnavailableExplanationRow,
                vec![case(
                    Consumer::MigrationImporter,
                    Family::WhyUnavailableExplanationRow,
                    Health::BlockedOwnerChangedNarrowed,
                    &[Caveat::BlockedActionOwnerReassigned],
                    "migration importer why-unavailable row narrowed by changed owner",
                )],
            ),
        ],
    ));

    // 3. Keybinding / leader help — the migration-bridge card at full parity (old keybinding →
    //    new command), plus the sequence-help strip auto-narrowed because the command-language
    //    sequence is unsupported here and stays a disclosed dead-end.
    rows.push(base_row(
        Consumer::KeybindingLeaderHelp,
        "Keybinding / leader-help surface owner",
        "Keybinding / leader help adopts the migration-bridge card at full parity for old-keybinding-to-new-command mapping, and the sequence-help strip auto-narrowed because the command-language sequence is unsupported here, keeping command binding, migration mapping, blocked-action explanation, and source-language citation explicit so an unsupported sequence never claims a backing command it lacks",
        "evidence:m5-teaching-consumer-keybinding-help:001",
        vec![
            binding(
                Family::MigrationBridgeCard,
                vec![case(
                    Consumer::KeybindingLeaderHelp,
                    Family::MigrationBridgeCard,
                    Health::FullParity,
                    &[],
                    "keybinding help migration-bridge card at full parity",
                )],
            ),
            binding(
                Family::SequenceHelpStrip,
                vec![case(
                    Consumer::KeybindingLeaderHelp,
                    Family::SequenceHelpStrip,
                    Health::SequenceUnsupportedNarrowed,
                    &[Caveat::SequenceUnsupportedNoBackingCommand],
                    "keybinding help sequence-help strip narrowed by unsupported sequence",
                )],
            ),
        ],
    ));

    // 4. Command docs — the contextual-tip card, why-unavailable explanation row, and
    //    source-language fallback surface all at full parity: documentation describes the same
    //    command, blocked-action, and localized-citation truth the product renders.
    rows.push(base_row(
        Consumer::CommandDocs,
        "Command-docs surface owner",
        "Command docs adopt the contextual-tip card, why-unavailable explanation row, and source-language fallback surface at full parity, referencing the canonical component schemas so command binding, migration mapping, blocked-action explanation, and source-language citation stay one truth across every claimed teaching surface rather than being re-worded in prose",
        "evidence:m5-teaching-consumer-command-docs:001",
        vec![
            binding(
                Family::ContextualTipCard,
                vec![case(
                    Consumer::CommandDocs,
                    Family::ContextualTipCard,
                    Health::FullParity,
                    &[],
                    "command docs contextual-tip card at full parity",
                )],
            ),
            binding(
                Family::WhyUnavailableExplanationRow,
                vec![case(
                    Consumer::CommandDocs,
                    Family::WhyUnavailableExplanationRow,
                    Health::FullParity,
                    &[],
                    "command docs why-unavailable row at full parity",
                )],
            ),
            binding(
                Family::SourceLanguageFallback,
                vec![case(
                    Consumer::CommandDocs,
                    Family::SourceLanguageFallback,
                    Health::FullParity,
                    &[],
                    "command docs source-language fallback at full parity",
                )],
            ),
        ],
    ));

    // 5. Help pane — the sequence-help strip at full parity, plus the why-unavailable
    //    explanation row auto-narrowed by a changed blocked-action owner and the source-language
    //    fallback surface auto-narrowed because the localized content is stale / policy-limited
    //    and falls back to the source language with its citation preserved.
    rows.push(base_row(
        Consumer::HelpPane,
        "Help-pane surface owner",
        "The Help pane adopts the sequence-help strip at full parity, the why-unavailable explanation row auto-narrowed by a changed blocked-action owner, and the source-language fallback surface auto-narrowed because the localized content is stale or policy-limited, keeping command binding, migration mapping, blocked-action explanation, and source-language citation explicit so a stale translation never severs its canonical citation",
        "evidence:m5-teaching-consumer-help-pane:001",
        vec![
            binding(
                Family::SequenceHelpStrip,
                vec![case(
                    Consumer::HelpPane,
                    Family::SequenceHelpStrip,
                    Health::FullParity,
                    &[],
                    "help pane sequence-help strip at full parity",
                )],
            ),
            binding(
                Family::WhyUnavailableExplanationRow,
                vec![case(
                    Consumer::HelpPane,
                    Family::WhyUnavailableExplanationRow,
                    Health::BlockedOwnerChangedNarrowed,
                    &[Caveat::BlockedActionOwnerReassigned],
                    "help pane why-unavailable row narrowed by changed owner",
                )],
            ),
            binding(
                Family::SourceLanguageFallback,
                vec![case(
                    Consumer::HelpPane,
                    Family::SourceLanguageFallback,
                    Health::LocalizedFallbackStaleNarrowed,
                    &[Caveat::LocalizedFallbackStaleOrPolicyLimited],
                    "help pane source-language fallback narrowed by stale localization",
                )],
            ),
        ],
    ));

    // 6. Localized support packet — all five families, referencing the canonical schemas so its
    //    prose can never drift from the product truth. This is the authoritative rendering every
    //    other surface keeps parity with.
    rows.push(base_row(
        Consumer::LocalizedSupportPacket,
        "Localized support-packet surface owner",
        "The localized support packet adopts the contextual-tip card, migration-bridge card, sequence-help strip, why-unavailable explanation row, and source-language fallback surface, referencing the canonical component schemas so its prose can never drift from the product truth and keeping command binding, migration mapping, blocked-action explanation, and source-language citation exact in every exported case",
        "evidence:m5-teaching-consumer-localized-support:001",
        vec![
            binding(
                Family::ContextualTipCard,
                vec![case(
                    Consumer::LocalizedSupportPacket,
                    Family::ContextualTipCard,
                    Health::FullParity,
                    &[],
                    "localized support contextual-tip card at full parity",
                )],
            ),
            binding(
                Family::MigrationBridgeCard,
                vec![case(
                    Consumer::LocalizedSupportPacket,
                    Family::MigrationBridgeCard,
                    Health::FullParity,
                    &[],
                    "localized support migration-bridge card at full parity",
                )],
            ),
            binding(
                Family::SequenceHelpStrip,
                vec![case(
                    Consumer::LocalizedSupportPacket,
                    Family::SequenceHelpStrip,
                    Health::FullParity,
                    &[],
                    "localized support sequence-help strip at full parity",
                )],
            ),
            binding(
                Family::WhyUnavailableExplanationRow,
                vec![case(
                    Consumer::LocalizedSupportPacket,
                    Family::WhyUnavailableExplanationRow,
                    Health::FullParity,
                    &[],
                    "localized support why-unavailable row at full parity",
                )],
            ),
            binding(
                Family::SourceLanguageFallback,
                vec![case(
                    Consumer::LocalizedSupportPacket,
                    Family::SourceLanguageFallback,
                    Health::FullParity,
                    &[],
                    "localized support source-language fallback at full parity",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5TeachingComponentConsumerGovernanceReview {
    M5TeachingComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        command_mapping_owner_citation_explicit_on_every_surface: true,
        degraded_state_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        partial_or_unsupported_state_never_shown_as_exact: true,
        localized_support_presents_same_teaching_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5TeachingComponentConsumerProjection {
    M5TeachingComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        command_binding_reads_single_source: true,
        migration_mapping_reads_single_source: true,
        blocked_action_explanation_reads_single_source: true,
        source_language_citation_reads_single_source: true,
    }
}

fn proof_freshness() -> M5TeachingComponentConsumerProofFreshness {
    M5TeachingComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5TeachingComponentConsumerReleasePosture {
    M5TeachingComponentConsumerReleasePosture {
        release_packet_ref: M5_TEACHING_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        teaching_component_consumer_audit_ref: M5_TEACHING_COMPONENT_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TEACHING_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_TEACHING_COMPONENT_CONSUMER_DOC_REF,
        M5_TEACHING_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_TEACHING_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        family_canonical_schema_ref(M5ContextualTeachingComponentFamily::ContextualTipCard),
        family_canonical_schema_ref(M5ContextualTeachingComponentFamily::MigrationBridgeCard),
        family_canonical_schema_ref(M5ContextualTeachingComponentFamily::SequenceHelpStrip),
        family_canonical_schema_ref(
            M5ContextualTeachingComponentFamily::WhyUnavailableExplanationRow,
        ),
    ])
}

/// Builds the canonical M5 contextual-teaching component-consumer packet.
pub fn seeded_m5_teaching_component_consumer_packet() -> M5TeachingComponentConsumerPacket {
    M5TeachingComponentConsumerPacket::new(M5TeachingComponentConsumerPacketInput {
        packet_id: M5_TEACHING_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 contextual-teaching component consumers: first-run onboarding, the migration importer, keybinding / leader help, command docs, the Help pane, and the localized support packet keep command, mapping, blocked-action, and source-language parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5TeachingComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the migration importer is held at Beta because a slice of imported
/// behavior is still only partially mapped; every consumer stays visible.
pub fn seeded_m5_teaching_component_consumer_migration_importer_beta_narrowed(
) -> M5TeachingComponentConsumerPacket {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.packet_id =
        "m5-contextual-teaching-component-consumer:migration-importer-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5TeachingComponentConsumer::MigrationImporter)
        .expect("migration-importer row present");
    row.qualification = M5TeachingQualificationClass::Beta;
    packet
}

/// Narrowed variant: the Help pane is held at Preview because a slice of localized fallback
/// content is still stale or policy-limited; every consumer stays visible.
pub fn seeded_m5_teaching_component_consumer_help_pane_preview_narrowed(
) -> M5TeachingComponentConsumerPacket {
    let mut packet = seeded_m5_teaching_component_consumer_packet();
    packet.packet_id =
        "m5-contextual-teaching-component-consumer:help-pane-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5TeachingComponentConsumer::HelpPane)
        .expect("help-pane row present");
    row.qualification = M5TeachingQualificationClass::Preview;
    packet
}
