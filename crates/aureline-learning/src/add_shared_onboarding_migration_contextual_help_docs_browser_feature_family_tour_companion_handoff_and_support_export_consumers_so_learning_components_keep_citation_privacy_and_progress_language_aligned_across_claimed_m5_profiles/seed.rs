//! Canonical seed builders for the M5 learning component-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked bindings, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical learning component-consumer packet.
pub const M5_LEARNING_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-learning-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5LearningComponentConsumer,
    component_family: M5LearningComponentFamily,
    parity_health: M5LearningConsumerParityHealth,
    export_caveats: &[M5LearningConsumerExportCaveat],
    note: &str,
) -> M5LearningComponentBindingCase {
    M5LearningComponentBindingCase::resolved(M5LearningComponentBindingInput {
        consumer,
        component_family,
        descriptor_families: M5LearningComponentDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5LearningComponentFamily,
    example_bindings: Vec<M5LearningComponentBindingCase>,
) -> M5LearningComponentBinding {
    M5LearningComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5LearningComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5LearningComponentBinding>,
) -> M5LearningComponentConsumerRow {
    M5LearningComponentConsumerRow {
        consumer,
        qualification: M5LearningQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5LearningSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5LearningDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5LearningConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5LearningComponentDescriptor::ALL.to_vec(),
        parity_health_modes: M5LearningConsumerParityHealth::ALL.to_vec(),
        export_caveats: M5LearningConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5LearningClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5LearningConsumerNarrowingReason::ALL.to_vec(),
        recovery_actions: M5LearningConsumerRecoveryAction::ALL.to_vec(),
        export_fields: M5LearningConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5LearningAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5LearningConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5LearningDowngradeTrigger::GlossaryCitationSevered,
            M5LearningDowngradeTrigger::OfflineOrLocalOnlyStateHidden,
            M5LearningDowngradeTrigger::CachedStateHidden,
            M5LearningDowngradeTrigger::NotInstalledStateHidden,
            M5LearningDowngradeTrigger::ProgressOwnershipUnstated,
            M5LearningDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_LEARNING_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_LEARNING_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_learning_grammar: false,
        drops_citation_progress_or_explain_do_when_narrowed: false,
        shows_uncited_or_unavailable_source_as_live_cited: false,
        widens_trust_or_mutating_authority: false,
    }
}

fn consumer_rows() -> Vec<M5LearningComponentConsumerRow> {
    use M5LearningComponentConsumer as Consumer;
    use M5LearningComponentFamily as Family;
    use M5LearningConsumerExportCaveat as Caveat;
    use M5LearningConsumerParityHealth as Health;

    let mut rows = Vec::new();

    // 1. First-run onboarding — the learning-mode toggle and tip card at full parity, pointing at
    //    the canonical component schemas so the same citation, source-class, progress / privacy,
    //    and explain-versus-do truth appears here as in migration, contextual help, docs / browser,
    //    the feature-family tour, the companion handoff, and the support / export packet.
    rows.push(base_row(
        Consumer::Onboarding,
        "First-run onboarding surface owner",
        "First-run onboarding adopts the learning-mode toggle and tip card at full parity, referencing the canonical component schemas so the same citation, source-class, progress / privacy, and explain-versus-do language appears here as in migration, contextual help, docs / browser, the feature-family tour, the companion handoff, and the support / export packet",
        "evidence:m5-learning-consumer-onboarding:001",
        vec![
            binding(
                Family::LearningModeToggle,
                vec![case(
                    Consumer::Onboarding,
                    Family::LearningModeToggle,
                    Health::FullParity,
                    &[],
                    "onboarding learning-mode toggle at full parity",
                )],
            ),
            binding(
                Family::TipCard,
                vec![case(
                    Consumer::Onboarding,
                    Family::TipCard,
                    Health::FullParity,
                    &[],
                    "onboarding tip card at full parity",
                )],
            ),
        ],
    ));

    // 2. Migration onboarding — the tip card at full parity, plus the glossary chip / card
    //    auto-narrowed because its pack is served from a cached copy while the migration finishes,
    //    so cached content never reads as the live pack.
    rows.push(base_row(
        Consumer::Migration,
        "Migration-onboarding surface owner",
        "Migration onboarding adopts the tip card at full parity and the glossary chip / card auto-narrowed because its pack is served from a cached copy, keeping citation, source-class, progress / privacy, and explain-versus-do explicit so cached content never reads as the live pack",
        "evidence:m5-learning-consumer-migration:001",
        vec![
            binding(
                Family::TipCard,
                vec![case(
                    Consumer::Migration,
                    Family::TipCard,
                    Health::FullParity,
                    &[],
                    "migration tip card at full parity",
                )],
            ),
            binding(
                Family::GlossaryChipOrCard,
                vec![case(
                    Consumer::Migration,
                    Family::GlossaryChipOrCard,
                    Health::CachedPackNarrowed,
                    &[Caveat::ContentServedFromCachedPack],
                    "migration glossary chip / card narrowed by cached pack",
                )],
            ),
        ],
    ));

    // 3. Contextual help — the tip card, guided-exercise step, and safe-explanation banner all at
    //    full parity: inline help teaches the same citation, progress, and explain-versus-do truth
    //    the product renders.
    rows.push(base_row(
        Consumer::ContextualHelp,
        "Contextual-help surface owner",
        "Contextual help adopts the tip card, guided-exercise step, and safe-explanation banner at full parity, referencing the canonical component schemas so citation, source-class, progress / privacy, and explain-versus-do stay one truth across every claimed learning surface rather than being re-worded in prose",
        "evidence:m5-learning-consumer-contextual-help:001",
        vec![
            binding(
                Family::TipCard,
                vec![case(
                    Consumer::ContextualHelp,
                    Family::TipCard,
                    Health::FullParity,
                    &[],
                    "contextual help tip card at full parity",
                )],
            ),
            binding(
                Family::GuidedExerciseStep,
                vec![case(
                    Consumer::ContextualHelp,
                    Family::GuidedExerciseStep,
                    Health::FullParity,
                    &[],
                    "contextual help guided-exercise step at full parity",
                )],
            ),
            binding(
                Family::SafeExplanationBanner,
                vec![case(
                    Consumer::ContextualHelp,
                    Family::SafeExplanationBanner,
                    Health::FullParity,
                    &[],
                    "contextual help safe-explanation banner at full parity",
                )],
            ),
        ],
    ));

    // 4. Docs / browser — the glossary chip / card auto-narrowed because its cited source content
    //    is stale, plus the safe-explanation banner at full parity, so a stale citation discloses
    //    its freshness rather than reading as live.
    rows.push(base_row(
        Consumer::DocsBrowser,
        "Docs / browser surface owner",
        "The docs / browser surface adopts the glossary chip / card auto-narrowed because its cited source content is stale, and the safe-explanation banner at full parity, keeping citation, source-class, progress / privacy, and explain-versus-do explicit so stale cited content discloses its freshness rather than reading as live",
        "evidence:m5-learning-consumer-docs-browser:001",
        vec![
            binding(
                Family::GlossaryChipOrCard,
                vec![case(
                    Consumer::DocsBrowser,
                    Family::GlossaryChipOrCard,
                    Health::StaleSourceNarrowed,
                    &[Caveat::SourceContentStale],
                    "docs / browser glossary chip / card narrowed by stale source",
                )],
            ),
            binding(
                Family::SafeExplanationBanner,
                vec![case(
                    Consumer::DocsBrowser,
                    Family::SafeExplanationBanner,
                    Health::FullParity,
                    &[],
                    "docs / browser safe-explanation banner at full parity",
                )],
            ),
        ],
    ));

    // 5. Feature-family tour — the learning-mode toggle and guided-exercise step at full parity,
    //    plus the progress marker auto-narrowed because progress is local-only (no supported sync /
    //    export path was chosen), so progress stays user-owned and default-local.
    rows.push(base_row(
        Consumer::FeatureFamilyTour,
        "Feature-family tour surface owner",
        "The feature-family tour adopts the learning-mode toggle and guided-exercise step at full parity, and the progress marker auto-narrowed because progress is local-only, keeping citation, source-class, progress / privacy, and explain-versus-do explicit so progress stays user-owned and default-local unless a supported sync / export path is chosen",
        "evidence:m5-learning-consumer-feature-family-tour:001",
        vec![
            binding(
                Family::LearningModeToggle,
                vec![case(
                    Consumer::FeatureFamilyTour,
                    Family::LearningModeToggle,
                    Health::FullParity,
                    &[],
                    "feature-family tour learning-mode toggle at full parity",
                )],
            ),
            binding(
                Family::GuidedExerciseStep,
                vec![case(
                    Consumer::FeatureFamilyTour,
                    Family::GuidedExerciseStep,
                    Health::FullParity,
                    &[],
                    "feature-family tour guided-exercise step at full parity",
                )],
            ),
            binding(
                Family::ProgressMarker,
                vec![case(
                    Consumer::FeatureFamilyTour,
                    Family::ProgressMarker,
                    Health::ProgressLocalOnlyNarrowed,
                    &[Caveat::ProgressLocalOnlyNotSynced],
                    "feature-family tour progress marker narrowed by local-only progress",
                )],
            ),
        ],
    ));

    // 6. Companion handoff — the safe-explanation banner auto-narrowed because a cited source is
    //    unavailable or not installed on the companion, plus the progress marker at full parity, so
    //    an unavailable citation never reads as a live, cited source.
    rows.push(base_row(
        Consumer::CompanionHandoff,
        "Companion-handoff surface owner",
        "The companion handoff adopts the safe-explanation banner auto-narrowed because a cited source is unavailable or not installed on the companion, and the progress marker at full parity, keeping citation, source-class, progress / privacy, and explain-versus-do explicit so an unavailable citation never reads as a live, cited source",
        "evidence:m5-learning-consumer-companion-handoff:001",
        vec![
            binding(
                Family::SafeExplanationBanner,
                vec![case(
                    Consumer::CompanionHandoff,
                    Family::SafeExplanationBanner,
                    Health::CitationUnavailableNarrowed,
                    &[Caveat::CitedSourceUnavailableOrNotInstalled],
                    "companion handoff safe-explanation banner narrowed by unavailable citation",
                )],
            ),
            binding(
                Family::ProgressMarker,
                vec![case(
                    Consumer::CompanionHandoff,
                    Family::ProgressMarker,
                    Health::FullParity,
                    &[],
                    "companion handoff progress marker at full parity",
                )],
            ),
        ],
    ));

    // 7. Support / export packet — all six families, referencing the canonical schemas so its
    //    prose can never drift from the product truth. This is the authoritative rendering every
    //    other surface keeps parity with.
    rows.push(base_row(
        Consumer::SupportExport,
        "Support / export-packet surface owner",
        "The support / export packet adopts the learning-mode toggle, tip card, guided-exercise step, glossary chip / card, safe-explanation banner, and progress marker, referencing the canonical component schemas so its prose can never drift from the product truth and keeping citation, source-class, progress / privacy, and explain-versus-do exact in every exported case",
        "evidence:m5-learning-consumer-support-export:001",
        vec![
            binding(
                Family::LearningModeToggle,
                vec![case(
                    Consumer::SupportExport,
                    Family::LearningModeToggle,
                    Health::FullParity,
                    &[],
                    "support / export learning-mode toggle at full parity",
                )],
            ),
            binding(
                Family::TipCard,
                vec![case(
                    Consumer::SupportExport,
                    Family::TipCard,
                    Health::FullParity,
                    &[],
                    "support / export tip card at full parity",
                )],
            ),
            binding(
                Family::GuidedExerciseStep,
                vec![case(
                    Consumer::SupportExport,
                    Family::GuidedExerciseStep,
                    Health::FullParity,
                    &[],
                    "support / export guided-exercise step at full parity",
                )],
            ),
            binding(
                Family::GlossaryChipOrCard,
                vec![case(
                    Consumer::SupportExport,
                    Family::GlossaryChipOrCard,
                    Health::FullParity,
                    &[],
                    "support / export glossary chip / card at full parity",
                )],
            ),
            binding(
                Family::SafeExplanationBanner,
                vec![case(
                    Consumer::SupportExport,
                    Family::SafeExplanationBanner,
                    Health::FullParity,
                    &[],
                    "support / export safe-explanation banner at full parity",
                )],
            ),
            binding(
                Family::ProgressMarker,
                vec![case(
                    Consumer::SupportExport,
                    Family::ProgressMarker,
                    Health::FullParity,
                    &[],
                    "support / export progress marker at full parity",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5LearningComponentConsumerGovernanceReview {
    M5LearningComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        citation_progress_and_explain_do_explicit_on_every_surface: true,
        degraded_state_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        uncited_or_unavailable_source_never_shown_as_live_cited: true,
        support_export_presents_same_learning_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5LearningComponentConsumerProjection {
    M5LearningComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        citation_source_reads_single_source: true,
        source_class_freshness_reads_single_source: true,
        progress_ownership_privacy_reads_single_source: true,
        explain_versus_do_reads_single_source: true,
    }
}

fn proof_freshness() -> M5LearningComponentConsumerProofFreshness {
    M5LearningComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5LearningComponentConsumerReleasePosture {
    M5LearningComponentConsumerReleasePosture {
        release_packet_ref: M5_LEARNING_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        learning_component_consumer_audit_ref: M5_LEARNING_COMPONENT_CONSUMER_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_LEARNING_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_LEARNING_COMPONENT_CONSUMER_DOC_REF,
        M5_LEARNING_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_LEARNING_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        family_canonical_schema_ref(M5LearningComponentFamily::LearningModeToggle),
        family_canonical_schema_ref(M5LearningComponentFamily::GuidedExerciseStep),
        family_canonical_schema_ref(M5LearningComponentFamily::GlossaryChipOrCard),
    ])
}

/// Builds the canonical M5 learning component-consumer packet.
pub fn seeded_m5_learning_component_consumer_packet() -> M5LearningComponentConsumerPacket {
    M5LearningComponentConsumerPacket::new(M5LearningComponentConsumerPacketInput {
        packet_id: M5_LEARNING_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 learning component consumers: onboarding, migration, contextual help, docs / browser, the feature-family tour, the companion handoff, and the support / export packet keep citation, source-class, progress / privacy, and explain-versus-do parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5LearningComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the docs / browser surface is held at Beta because a slice of cited source
/// content is still stale; every consumer stays visible.
pub fn seeded_m5_learning_component_consumer_docs_browser_beta_narrowed(
) -> M5LearningComponentConsumerPacket {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.packet_id = "m5-learning-component-consumer:docs-browser-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5LearningComponentConsumer::DocsBrowser)
        .expect("docs-browser row present");
    row.qualification = M5LearningQualificationClass::Beta;
    packet
}

/// Narrowed variant: the companion handoff is held at Preview because a cited source is still
/// unavailable or not installed on the companion; every consumer stays visible.
pub fn seeded_m5_learning_component_consumer_companion_handoff_preview_narrowed(
) -> M5LearningComponentConsumerPacket {
    let mut packet = seeded_m5_learning_component_consumer_packet();
    packet.packet_id = "m5-learning-component-consumer:companion-handoff-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5LearningComponentConsumer::CompanionHandoff)
        .expect("companion-handoff row present");
    row.qualification = M5LearningQualificationClass::Preview;
    packet
}
