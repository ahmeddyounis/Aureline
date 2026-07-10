//! Canonical seed builders for the M5 experiment component-consumer lane.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, the worked bindings, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical experiment component-consumer packet.
pub const M5_EXPERIMENT_COMPONENT_CONSUMER_PACKET_ID: &str =
    "m5-experiment-component-consumer:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked binding case for one consumer/family adoption.
fn case(
    consumer: M5ExperimentComponentConsumer,
    component_family: M5ExperimentComponentFamily,
    parity_health: M5ExperimentConsumerParityHealth,
    export_caveats: &[M5ExperimentConsumerExportCaveat],
    note: &str,
) -> M5ExperimentComponentBindingCase {
    M5ExperimentComponentBindingCase::resolved(M5ExperimentComponentBindingInput {
        consumer,
        component_family,
        descriptor_families: M5ExperimentComponentDescriptor::ALL.to_vec(),
        parity_health,
        export_caveats: export_caveats.to_vec(),
        note_repr: Some(note.to_owned()),
    })
}

/// Builds a component binding that points at its canonical family refs.
fn binding(
    component_family: M5ExperimentComponentFamily,
    example_bindings: Vec<M5ExperimentComponentBindingCase>,
) -> M5ExperimentComponentBinding {
    M5ExperimentComponentBinding {
        component_family,
        canonical_schema_ref: family_canonical_schema_ref(component_family).to_owned(),
        canonical_artifact_ref: family_canonical_artifact_ref(component_family).to_owned(),
        references_canonical_not_local_prose: true,
        example_bindings,
    }
}

/// A base row with the shared parity vocabulary filled in.
fn base_row(
    consumer: M5ExperimentComponentConsumer,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    component_bindings: Vec<M5ExperimentComponentBinding>,
) -> M5ExperimentComponentConsumerRow {
    M5ExperimentComponentConsumerRow {
        consumer,
        qualification: M5ExperimentQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5ExperimentSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ExperimentDeploymentLine::ALL.to_vec(),
        anatomy_parts: M5ExperimentConsumerAnatomyPart::ALL.to_vec(),
        descriptor_families: M5ExperimentComponentDescriptor::ALL.to_vec(),
        parity_health_modes: M5ExperimentConsumerParityHealth::ALL.to_vec(),
        export_caveats: M5ExperimentConsumerExportCaveat::ALL.to_vec(),
        claim_parity_states: M5ExperimentClaimParityState::ALL.to_vec(),
        narrowing_reasons: M5ExperimentConsumerNarrowingReason::ALL.to_vec(),
        recovery_actions: M5ExperimentConsumerRecoveryAction::ALL.to_vec(),
        export_fields: M5ExperimentConsumerExportField::ALL.to_vec(),
        accessibility_routes: M5ExperimentAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
        downgrade_triggers: vec![
            M5ExperimentDowngradeTrigger::DatasetProvenanceSevered,
            M5ExperimentDowngradeTrigger::SensitivityClassUnstated,
            M5ExperimentDowngradeTrigger::ComparabilityOverstated,
            M5ExperimentDowngradeTrigger::ExportScopeUnstated,
            M5ExperimentDowngradeTrigger::RawPayloadExposedByDefault,
            M5ExperimentDowngradeTrigger::ProofStale,
        ],
        component_bindings,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_EXPERIMENT_COMPONENT_CONSUMER_SCHEMA_REF,
            M5_EXPERIMENT_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        ]),
        rewords_claims_per_surface: false,
        invents_new_experiment_grammar: false,
        drops_lineage_sensitivity_or_comparability_when_narrowed: false,
        implies_apples_to_apples_without_parity: false,
        exposes_raw_payload_by_default: false,
    }
}

#[allow(clippy::vec_init_then_push)]
fn consumer_rows() -> Vec<M5ExperimentComponentConsumerRow> {
    use M5ExperimentComponentConsumer as Consumer;
    use M5ExperimentComponentFamily as Family;
    use M5ExperimentConsumerExportCaveat as Caveat;
    use M5ExperimentConsumerParityHealth as Health;

    let mut rows = Vec::new();

    // 1. Notebook run history — the experiment run row at full parity, and the environment
    //    fingerprint card auto-narrowed because the run's producing lineage / provenance is
    //    incomplete, so origin, revision, and fingerprint stay explicit here as in every other
    //    claimed experiment consumer.
    rows.push(base_row(
        Consumer::NotebookRunHistory,
        "Notebook run-history surface owner",
        "The notebook run history adopts the experiment run row at full parity and the environment fingerprint card auto-narrowed because the run's producing lineage / provenance is incomplete, referencing the canonical component schemas so lineage / provenance, sensitivity, comparability, and export-scope language appears here as in tasks / tests / evals, review evidence, the compare view, the companion summary, the CLI / headless export, and the support / export packet",
        "evidence:m5-experiment-consumer-notebook-run-history:001",
        vec![
            binding(
                Family::ExperimentRunRow,
                vec![case(
                    Consumer::NotebookRunHistory,
                    Family::ExperimentRunRow,
                    Health::FullParity,
                    &[],
                    "notebook run-history experiment run row at full parity",
                )],
            ),
            binding(
                Family::EnvironmentFingerprintCard,
                vec![case(
                    Consumer::NotebookRunHistory,
                    Family::EnvironmentFingerprintCard,
                    Health::ProvenanceIncompleteNarrowed,
                    &[Caveat::LineageProvenanceIncomplete],
                    "notebook run-history environment fingerprint card narrowed by incomplete provenance",
                )],
            ),
        ],
    ));

    // 2. Tasks / tests / evals — the experiment run row, dataset provenance card, and run comparison
    //    table at full parity: task, test, and eval runs read the same run-origin, provenance, and
    //    comparability truth the product renders.
    rows.push(base_row(
        Consumer::TaskTestEvalRuns,
        "Tasks / tests / evals surface owner",
        "Tasks / tests / evals adopt the experiment run row, dataset provenance card, and run comparison table at full parity, referencing the canonical component schemas so lineage / provenance, sensitivity, comparability, and export-scope stay one truth across every claimed experiment surface rather than being re-worded in prose",
        "evidence:m5-experiment-consumer-task-test-eval:001",
        vec![
            binding(
                Family::ExperimentRunRow,
                vec![case(
                    Consumer::TaskTestEvalRuns,
                    Family::ExperimentRunRow,
                    Health::FullParity,
                    &[],
                    "tasks / tests / evals experiment run row at full parity",
                )],
            ),
            binding(
                Family::DatasetProvenanceCard,
                vec![case(
                    Consumer::TaskTestEvalRuns,
                    Family::DatasetProvenanceCard,
                    Health::FullParity,
                    &[],
                    "tasks / tests / evals dataset provenance card at full parity",
                )],
            ),
            binding(
                Family::RunComparisonTable,
                vec![case(
                    Consumer::TaskTestEvalRuns,
                    Family::RunComparisonTable,
                    Health::FullParity,
                    &[],
                    "tasks / tests / evals run comparison table at full parity",
                )],
            ),
        ],
    ));

    // 3. Review evidence — the dataset provenance card, artifact lineage panel, and compare guard
    //    banner at full parity, plus the sensitivity / sharing banner auto-narrowed because the data
    //    is sensitive and redacted rather than raw, so raw production-like data is never exposed by
    //    default in a review flow.
    rows.push(base_row(
        Consumer::ReviewEvidence,
        "Review-evidence surface owner",
        "Review evidence adopts the dataset provenance card, artifact lineage panel, and compare guard banner at full parity, and the sensitivity / sharing banner auto-narrowed because the data is sensitive and redacted rather than raw, keeping lineage / provenance, sensitivity, comparability, and export-scope explicit so raw production-like data is never exposed by default",
        "evidence:m5-experiment-consumer-review-evidence:001",
        vec![
            binding(
                Family::DatasetProvenanceCard,
                vec![case(
                    Consumer::ReviewEvidence,
                    Family::DatasetProvenanceCard,
                    Health::FullParity,
                    &[],
                    "review evidence dataset provenance card at full parity",
                )],
            ),
            binding(
                Family::SensitivitySharingBanner,
                vec![case(
                    Consumer::ReviewEvidence,
                    Family::SensitivitySharingBanner,
                    Health::SensitivityRestrictedNarrowed,
                    &[Caveat::SensitiveDataRedactedNotRaw],
                    "review evidence sensitivity / sharing banner narrowed by restricted sensitive data",
                )],
            ),
            binding(
                Family::ArtifactLineagePanel,
                vec![case(
                    Consumer::ReviewEvidence,
                    Family::ArtifactLineagePanel,
                    Health::FullParity,
                    &[],
                    "review evidence artifact lineage panel at full parity",
                )],
            ),
            binding(
                Family::CompareGuardBanner,
                vec![case(
                    Consumer::ReviewEvidence,
                    Family::CompareGuardBanner,
                    Health::FullParity,
                    &[],
                    "review evidence compare guard banner at full parity",
                )],
            ),
        ],
    ));

    // 4. Compare view — the compare guard banner at full parity, the result summary card at full
    //    parity, and the run comparison table auto-narrowed because the comparison lacks parity
    //    evidence, so a metric delta never implies an apples-to-apples fair baseline.
    rows.push(base_row(
        Consumer::CompareView,
        "Compare-view surface owner",
        "The compare view adopts the compare guard banner and result summary card at full parity, and the run comparison table auto-narrowed because the comparison lacks parity evidence, keeping lineage / provenance, sensitivity, comparability, and export-scope explicit so a metric delta never implies an apples-to-apples fair baseline",
        "evidence:m5-experiment-consumer-compare-view:001",
        vec![
            binding(
                Family::ResultSummaryCard,
                vec![case(
                    Consumer::CompareView,
                    Family::ResultSummaryCard,
                    Health::FullParity,
                    &[],
                    "compare view result summary card at full parity",
                )],
            ),
            binding(
                Family::RunComparisonTable,
                vec![case(
                    Consumer::CompareView,
                    Family::RunComparisonTable,
                    Health::NotComparableNarrowed,
                    &[Caveat::ComparisonNotApplesToApples],
                    "compare view run comparison table narrowed by unproven comparability",
                )],
            ),
            binding(
                Family::CompareGuardBanner,
                vec![case(
                    Consumer::CompareView,
                    Family::CompareGuardBanner,
                    Health::FullParity,
                    &[],
                    "compare view compare guard banner at full parity",
                )],
            ),
        ],
    ));

    // 5. Companion summary — the sensitivity / sharing banner at full parity, plus the result
    //    summary card auto-narrowed because the companion-safe export carries metadata only, so a
    //    shared summary never implies it includes the raw payload.
    rows.push(base_row(
        Consumer::CompanionSummary,
        "Companion-summary surface owner",
        "The companion summary adopts the sensitivity / sharing banner at full parity and the result summary card auto-narrowed because the companion-safe export carries metadata only, keeping lineage / provenance, sensitivity, comparability, and export-scope explicit so a shared summary never implies it includes the raw payload",
        "evidence:m5-experiment-consumer-companion-summary:001",
        vec![
            binding(
                Family::SensitivitySharingBanner,
                vec![case(
                    Consumer::CompanionSummary,
                    Family::SensitivitySharingBanner,
                    Health::FullParity,
                    &[],
                    "companion summary sensitivity / sharing banner at full parity",
                )],
            ),
            binding(
                Family::ResultSummaryCard,
                vec![case(
                    Consumer::CompanionSummary,
                    Family::ResultSummaryCard,
                    Health::MetadataOnlyExportNarrowed,
                    &[Caveat::ExportMetadataOnlyNotRaw],
                    "companion summary result summary card narrowed by metadata-only export",
                )],
            ),
        ],
    ));

    // 6. CLI / headless export — the environment fingerprint card and artifact lineage panel at full
    //    parity: a headless export carries the same fingerprint and lineage truth the desktop UIs
    //    render.
    rows.push(base_row(
        Consumer::CliHeadlessExport,
        "CLI / headless-export surface owner",
        "The CLI / headless export adopts the environment fingerprint card and artifact lineage panel at full parity, referencing the canonical component schemas so lineage / provenance, sensitivity, comparability, and export-scope stay one truth across desktop, companion-safe summaries, and headless exports rather than being re-worded per surface",
        "evidence:m5-experiment-consumer-cli-headless-export:001",
        vec![
            binding(
                Family::EnvironmentFingerprintCard,
                vec![case(
                    Consumer::CliHeadlessExport,
                    Family::EnvironmentFingerprintCard,
                    Health::FullParity,
                    &[],
                    "CLI / headless export environment fingerprint card at full parity",
                )],
            ),
            binding(
                Family::ArtifactLineagePanel,
                vec![case(
                    Consumer::CliHeadlessExport,
                    Family::ArtifactLineagePanel,
                    Health::FullParity,
                    &[],
                    "CLI / headless export artifact lineage panel at full parity",
                )],
            ),
        ],
    ));

    // 7. Support / export packet — all eight families, referencing the canonical schemas so its
    //    prose can never drift from the product truth. This is the authoritative rendering every
    //    other surface keeps parity with.
    rows.push(base_row(
        Consumer::SupportExport,
        "Support / export-packet surface owner",
        "The support / export packet adopts the experiment run row, dataset provenance card, artifact lineage panel, run comparison table, environment fingerprint card, compare guard banner, sensitivity / sharing banner, and result summary card, referencing the canonical component schemas so its prose can never drift from the product truth and keeping lineage / provenance, sensitivity, comparability, and export-scope exact in every exported case",
        "evidence:m5-experiment-consumer-support-export:001",
        vec![
            binding(
                Family::ExperimentRunRow,
                vec![case(
                    Consumer::SupportExport,
                    Family::ExperimentRunRow,
                    Health::FullParity,
                    &[],
                    "support / export experiment run row at full parity",
                )],
            ),
            binding(
                Family::DatasetProvenanceCard,
                vec![case(
                    Consumer::SupportExport,
                    Family::DatasetProvenanceCard,
                    Health::FullParity,
                    &[],
                    "support / export dataset provenance card at full parity",
                )],
            ),
            binding(
                Family::ArtifactLineagePanel,
                vec![case(
                    Consumer::SupportExport,
                    Family::ArtifactLineagePanel,
                    Health::FullParity,
                    &[],
                    "support / export artifact lineage panel at full parity",
                )],
            ),
            binding(
                Family::RunComparisonTable,
                vec![case(
                    Consumer::SupportExport,
                    Family::RunComparisonTable,
                    Health::FullParity,
                    &[],
                    "support / export run comparison table at full parity",
                )],
            ),
            binding(
                Family::EnvironmentFingerprintCard,
                vec![case(
                    Consumer::SupportExport,
                    Family::EnvironmentFingerprintCard,
                    Health::FullParity,
                    &[],
                    "support / export environment fingerprint card at full parity",
                )],
            ),
            binding(
                Family::CompareGuardBanner,
                vec![case(
                    Consumer::SupportExport,
                    Family::CompareGuardBanner,
                    Health::FullParity,
                    &[],
                    "support / export compare guard banner at full parity",
                )],
            ),
            binding(
                Family::SensitivitySharingBanner,
                vec![case(
                    Consumer::SupportExport,
                    Family::SensitivitySharingBanner,
                    Health::FullParity,
                    &[],
                    "support / export sensitivity / sharing banner at full parity",
                )],
            ),
            binding(
                Family::ResultSummaryCard,
                vec![case(
                    Consumer::SupportExport,
                    Family::ResultSummaryCard,
                    Health::FullParity,
                    &[],
                    "support / export result summary card at full parity",
                )],
            ),
        ],
    ));

    rows
}

fn governance_review() -> M5ExperimentComponentConsumerGovernanceReview {
    M5ExperimentComponentConsumerGovernanceReview {
        consumers_adopt_shared_primitives: true,
        consumers_reference_canonical_schema: true,
        descriptor_vocabulary_shared_not_reworded: true,
        no_consumer_invents_new_grammar: true,
        lineage_sensitivity_comparability_and_export_scope_explicit_on_every_surface: true,
        degraded_state_auto_narrows_claim: true,
        narrowed_rendering_always_shows_self_contained_banner: true,
        banner_names_exact_reason_and_recovery_action: true,
        comparison_never_implies_apples_to_apples_without_parity: true,
        support_export_presents_same_experiment_truth: true,
        every_row_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5ExperimentComponentConsumerProjection {
    M5ExperimentComponentConsumerProjection {
        all_consumers_adopt_shared_components: true,
        lineage_provenance_reads_single_source: true,
        sensitivity_state_reads_single_source: true,
        comparability_reads_single_source: true,
        export_scope_reads_single_source: true,
    }
}

fn proof_freshness() -> M5ExperimentComponentConsumerProofFreshness {
    M5ExperimentComponentConsumerProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ExperimentComponentConsumerReleasePosture {
    M5ExperimentComponentConsumerReleasePosture {
        release_packet_ref: M5_EXPERIMENT_COMPONENT_CONSUMER_ARTIFACT_REF.to_owned(),
        experiment_component_consumer_audit_ref: M5_EXPERIMENT_COMPONENT_CONSUMER_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_EXPERIMENT_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_CONSUMER_DOC_REF,
        M5_EXPERIMENT_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_EXPERIMENT_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        family_canonical_schema_ref(M5ExperimentComponentFamily::ExperimentRunRow),
        family_canonical_schema_ref(M5ExperimentComponentFamily::DatasetProvenanceCard),
        family_canonical_schema_ref(M5ExperimentComponentFamily::ArtifactLineagePanel),
        family_canonical_schema_ref(M5ExperimentComponentFamily::RunComparisonTable),
    ])
}

/// Builds the canonical M5 experiment component-consumer packet.
pub fn seeded_m5_experiment_component_consumer_packet() -> M5ExperimentComponentConsumerPacket {
    M5ExperimentComponentConsumerPacket::new(M5ExperimentComponentConsumerPacketInput {
        packet_id: M5_EXPERIMENT_COMPONENT_CONSUMER_PACKET_ID.to_owned(),
        matrix_label:
            "M5 experiment component consumers: the notebook run history, tasks / tests / evals, review evidence, the compare view, the companion summary, the CLI / headless export, and the support / export packet keep lineage / provenance, sensitivity, comparability, and export-scope parity"
                .to_owned(),
        consumer_rows: consumer_rows(),
        vocabulary_set: M5ExperimentComponentConsumerVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the compare view is held at Beta because a slice of comparison parity evidence
/// is still incomplete; every consumer stays visible.
pub fn seeded_m5_experiment_component_consumer_compare_view_beta_narrowed(
) -> M5ExperimentComponentConsumerPacket {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.packet_id = "m5-experiment-component-consumer:compare-view-beta:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5ExperimentComponentConsumer::CompareView)
        .expect("compare-view row present");
    row.qualification = M5ExperimentQualificationClass::Beta;
    packet
}

/// Narrowed variant: review evidence is held at Preview because a slice of sensitive data is still
/// redaction-pending; every consumer stays visible.
pub fn seeded_m5_experiment_component_consumer_review_evidence_preview_narrowed(
) -> M5ExperimentComponentConsumerPacket {
    let mut packet = seeded_m5_experiment_component_consumer_packet();
    packet.packet_id = "m5-experiment-component-consumer:review-evidence-preview:0001".to_owned();
    let row = packet
        .consumer_rows
        .iter_mut()
        .find(|row| row.consumer == M5ExperimentComponentConsumer::ReviewEvidence)
        .expect("review-evidence row present");
    row.qualification = M5ExperimentQualificationClass::Preview;
    packet
}
