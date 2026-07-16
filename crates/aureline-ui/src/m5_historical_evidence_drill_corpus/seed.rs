//! Canonical seed for the historical-evidence drill corpus.
//!
//! The builders here are the only mint-from-truth path for the checked-in support export, matrix CSV, Markdown
//! summary, health dashboard, and narrowed fixtures. Every binding is derived from one per-fixture
//! [`HistoricalEvidenceGrammar`] so the same seeded fixture always carries the same non-live grammar across
//! surfaces, and every drill derives its state, handoff outcome, exact blocker, parity, content-available flag,
//! handoff-expectation refs, and action set from [`resolve_drill_disclosure`].

use super::*;

/// Packet mint timestamp (also the proof-refresh timestamp).
const SEED_TIMESTAMP: &str = "2026-07-16T00:00:00Z";

/// Export-safe redaction class carried by the packet.
const SEED_REDACTION_CLASS: &str = "support_safe_metadata_only";

/// The read-only, non-authoritative-for-mutation posture every seeded fixture holds.
const READ_ONLY_POSTURE: &str = "read_only_non_authoritative_for_mutation";

/// The full accessibility route set every drill offers so the non-live boundary, provenance, and handoff
/// expectation are discoverable without pointer-only chrome.
fn all_accessibility_routes() -> Vec<M5HistoricalReferenceAccessibilityRoute> {
    M5HistoricalReferenceAccessibilityRoute::ALL.to_vec()
}

fn grammar(
    historical_role: &str,
    snapshot_label: &str,
    capture_time: &str,
    provenance: &str,
) -> HistoricalEvidenceGrammar {
    HistoricalEvidenceGrammar {
        historical_role_word: historical_role.to_owned(),
        snapshot_label_word: snapshot_label.to_owned(),
        capture_time_word: capture_time.to_owned(),
        provenance_word: provenance.to_owned(),
        mutation_blocked_posture_word: READ_ONLY_POSTURE.to_owned(),
    }
}

fn provenance_join_for(object_class: M5HistoricalReferenceObject) -> ProvenanceJoin {
    ProvenanceJoin {
        source_snapshot_descriptor_ref: format!("snapshot-descriptor-{}", object_class.as_str()),
        capture_context_ref: format!("capture-context-{}", object_class.as_str()),
        producer_build_ref: format!("producer-build-{}", object_class.as_str()),
        provenance_lineage_ref: format!("provenance-lineage-{}", object_class.as_str()),
    }
}

fn blocker_note_for(drill: DrillScenario) -> String {
    match drill {
        DrillScenario::PreservedLiveTargetHandoff => {
            "Handoff cleared: identity, scope, route, trust, and authority validated; open the current live object through the validated live-target handoff."
        }
        DrillScenario::MissingLiveTarget => {
            "Blocked, missing live target: the current object was removed; falling back to a metadata-only exit rather than a dead end."
        }
        DrillScenario::RetiredLineReopen => {
            "Blocked, route unavailable: this is a retired line with no live counterpart; satisfy the migration prerequisite instead of an implicit reopen."
        }
        DrillScenario::StaleImportedEvidence => {
            "Blocked, trust posture insufficient: the imported evidence is stale; re-import fresh evidence to satisfy the prerequisite before reopening."
        }
        DrillScenario::ExpiredSnapshotMetadataOnlyFallback => {
            "Blocked, expired snapshot: the retention window closed and the content bytes are gone; metadata, capture time, and provenance render instead of a dead link."
        }
        DrillScenario::EvidenceOnlyReopenAfterVersionSchemaDrift => {
            "Blocked, imported or offline evidence only: version or schema drift means no live object can be reopened from this snapshot; the evidence stays non-live."
        }
    }
    .to_owned()
}

fn make_handoff_expectation(
    object_class: M5HistoricalReferenceObject,
    drill: DrillScenario,
    disclosure: DrillDisclosure,
) -> HandoffExpectation {
    let live_target_handoff_ref = if disclosure.requires_live_target_handoff_ref {
        Some(format!("live-target-handoff-{}", object_class.as_str()))
    } else {
        None
    };
    let metadata_only_exit_ref = if disclosure.requires_metadata_only_exit_ref {
        Some(format!("metadata-only-exit-{}", object_class.as_str()))
    } else {
        None
    };
    let satisfy_prerequisite_ref = if disclosure.requires_satisfy_prerequisite_ref {
        Some(format!("satisfy-prerequisite-{}", object_class.as_str()))
    } else {
        None
    };
    HandoffExpectation {
        expected_outcome: disclosure.expected_handoff_outcome,
        expected_blocker: disclosure.expected_blocker,
        live_target_handoff_ref,
        metadata_only_exit_ref,
        satisfy_prerequisite_ref,
        blocker_note: blocker_note_for(drill),
    }
}

fn corpus_evidence_for(binding_id: &str) -> CorpusEvidenceBindings {
    CorpusEvidenceBindings {
        screenshot_ref: format!(
            "artifacts/support/m5-historical-evidence-drills/screenshots/{binding_id}.png"
        ),
        accessibility_check_ref: format!(
            "artifacts/support/m5-historical-evidence-drills/accessibility/{binding_id}.json"
        ),
        cli_support_export_ref: M5_HISTORICAL_EVIDENCE_DRILL_ARTIFACT_REF.to_owned(),
        health_dashboard_ref: M5_HISTORICAL_EVIDENCE_DRILL_DASHBOARD_REF.to_owned(),
    }
}

fn allowed_actions_for(disclosure: DrillDisclosure) -> Vec<DrillCorpusAction> {
    let mut actions = DrillCorpusAction::BASE.to_vec();
    if disclosure.offers_open_live_target {
        actions.push(DrillCorpusAction::OpenCurrentLiveObject);
    }
    actions
}

fn binding_refs(object_class: M5HistoricalReferenceObject) -> Vec<String> {
    vec![
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF.to_owned(),
        object_class.canonical_domain_schema_ref().to_owned(),
    ]
}

#[allow(clippy::too_many_arguments)]
fn make_binding(
    binding_id: &str,
    fixture_id: &str,
    fixture_label: &str,
    object_class: M5HistoricalReferenceObject,
    consumer: M5HistoricalReferenceConsumerSurface,
    drill: DrillScenario,
    non_live_grammar: HistoricalEvidenceGrammar,
) -> DrillCorpusBinding {
    let disclosure = resolve_drill_disclosure(drill);
    DrillCorpusBinding {
        binding_id: binding_id.to_owned(),
        fixture_id: fixture_id.to_owned(),
        fixture_label: fixture_label.to_owned(),
        object_class,
        consumer,
        drill,
        drill_label: drill.stable_label().to_owned(),
        evidence_state: disclosure.expected_state,
        expected_handoff_outcome: disclosure.expected_handoff_outcome,
        expected_blocker: disclosure.expected_blocker,
        non_live_grammar,
        content_available: disclosure.expects_content_available,
        parity_state: disclosure.parity,
        allowed_actions: allowed_actions_for(disclosure),
        accessibility_routes: all_accessibility_routes(),
        provenance_join: provenance_join_for(object_class),
        handoff_expectation: make_handoff_expectation(object_class, drill, disclosure),
        corpus_evidence: corpus_evidence_for(binding_id),
        non_live_boundary_explicitly_called_out: true,
        looks_live_by_omission: false,
        reopens_live_target_without_validating_identity_trust_route_and_authority: false,
        dead_links_expired_or_removed_artifact: false,
        non_live_evidence_unjoined_to_capture_context: false,
        presents_as_current_or_reopens_through_ambiguous_route: false,
        source_contract_refs: binding_refs(object_class),
    }
}

/// One consumer-surface adoption of a seeded fixture, before any drill override.
struct BindingSpec {
    binding_id: &'static str,
    consumer: M5HistoricalReferenceConsumerSurface,
    drill: DrillScenario,
}

/// One seeded historical-reference fixture exercised across several consumer surfaces at one non-live grammar.
struct FixtureSpec {
    fixture_id: &'static str,
    fixture_label: &'static str,
    object_class: M5HistoricalReferenceObject,
    grammar: HistoricalEvidenceGrammar,
    bindings: Vec<BindingSpec>,
}

fn spec(
    fixture_id: &'static str,
    fixture_label: &'static str,
    object_class: M5HistoricalReferenceObject,
    grammar: HistoricalEvidenceGrammar,
    bindings: Vec<BindingSpec>,
) -> FixtureSpec {
    FixtureSpec {
        fixture_id,
        fixture_label,
        object_class,
        grammar,
        bindings,
    }
}

fn bs(
    binding_id: &'static str,
    consumer: M5HistoricalReferenceConsumerSurface,
    drill: DrillScenario,
) -> BindingSpec {
    BindingSpec {
        binding_id,
        consumer,
        drill,
    }
}

/// The four seeded fixture families — a last-supported retirement snapshot, a captured support / export evidence
/// bundle, a runbook / incident archived packet (split across the archived-runbook and review-incident object
/// classes), and an imported / offline route packet — and the drills that exercise each across the shell /
/// archive-viewer, help / docs, support, review / incident, runbook-archive, release-center, companion / export,
/// program-governance, and CLI / export consumers.
fn fixture_specs() -> Vec<FixtureSpec> {
    use DrillScenario::*;
    use M5HistoricalReferenceConsumerSurface::*;
    use M5HistoricalReferenceObject::*;

    vec![
        spec(
            "last-supported-retirement-snapshot",
            "Last-supported retirement snapshot fixture",
            RetirementSnapshot,
            grammar(
                "live_target_handoff",
                "retirement_last_supported_snapshot",
                "retirement_capture_time",
                "last_supported_build_provenance",
            ),
            vec![
                bs(
                    "hed-retirement-release",
                    ReleaseCenter,
                    PreservedLiveTargetHandoff,
                ),
                bs("hed-retirement-shell", Shell, RetiredLineReopen),
                bs("hed-retirement-cli", CliExport, MissingLiveTarget),
            ],
        ),
        spec(
            "support-export-evidence-bundle",
            "Captured support / export evidence bundle fixture",
            SupportExportEvidence,
            grammar(
                "provenance_attribution",
                "captured_support_export_evidence",
                "evidence_capture_time",
                "support_bundle_capture_context",
            ),
            vec![
                bs("hed-support-support", Support, PreservedLiveTargetHandoff),
                bs(
                    "hed-support-help",
                    HelpDocs,
                    ExpiredSnapshotMetadataOnlyFallback,
                ),
                bs(
                    "hed-support-companion",
                    CompanionExport,
                    EvidenceOnlyReopenAfterVersionSchemaDrift,
                ),
            ],
        ),
        spec(
            "runbook-incident-archived-packet",
            "Runbook / incident archived packet fixture",
            ArchivedRunbookPacket,
            grammar(
                "snapshot_labeling",
                "archived_runbook_execution_packet",
                "run_capture_time",
                "runbook_run_provenance",
            ),
            vec![
                bs(
                    "hed-runbook-runbook",
                    RunbookArchive,
                    PreservedLiveTargetHandoff,
                ),
                bs("hed-runbook-review", ReviewIncident, StaleImportedEvidence),
                bs("hed-runbook-program", ProgramGovernance, MissingLiveTarget),
            ],
        ),
        spec(
            "imported-offline-route-packet",
            "Imported / offline route packet fixture",
            ImportedOfflineRouteEvidence,
            grammar(
                "imported_offline_disclosure",
                "imported_offline_route_evidence",
                "import_capture_time",
                "import_offline_source_provenance",
            ),
            vec![
                bs(
                    "hed-imported-shell",
                    Shell,
                    EvidenceOnlyReopenAfterVersionSchemaDrift,
                ),
                bs(
                    "hed-imported-runbook",
                    RunbookArchive,
                    StaleImportedEvidence,
                ),
                bs(
                    "hed-imported-cli",
                    CliExport,
                    ExpiredSnapshotMetadataOnlyFallback,
                ),
            ],
        ),
        spec(
            "review-incident-archived-snapshot",
            "Review / incident archived snapshot fixture",
            ReviewIncidentSnapshot,
            grammar(
                "mutation_blocked_posture",
                "review_incident_snapshot",
                "incident_capture_time",
                "incident_evidence_provenance",
            ),
            vec![
                bs(
                    "hed-review-review",
                    ReviewIncident,
                    PreservedLiveTargetHandoff,
                ),
                bs("hed-review-shell", Shell, RetiredLineReopen),
                bs(
                    "hed-review-companion",
                    CompanionExport,
                    EvidenceOnlyReopenAfterVersionSchemaDrift,
                ),
            ],
        ),
    ]
}

/// Builds all drill bindings, applying `drill_override` to override a binding's drill.
fn build_bindings<F>(drill_override: F) -> Vec<DrillCorpusBinding>
where
    F: Fn(&str, DrillScenario) -> DrillScenario,
{
    let mut bindings = Vec::new();
    for fixture in fixture_specs() {
        for spec in &fixture.bindings {
            let drill = drill_override(spec.binding_id, spec.drill);
            bindings.push(make_binding(
                spec.binding_id,
                fixture.fixture_id,
                fixture.fixture_label,
                fixture.object_class,
                spec.consumer,
                drill,
                fixture.grammar.clone(),
            ));
        }
    }
    bindings
}

fn trust_review() -> DrillCorpusTrustReview {
    DrillCorpusTrustReview {
        fixtures_seed_four_or_more_historical_reference_states: true,
        fixtures_seed_two_or_more_live_target_handoff_outcomes: true,
        exact_blockers_are_distinguishable: true,
        every_object_class_seeded_by_two_or_more_consumers: true,
        non_live_grammar_identical_for_same_fixture: true,
        historical_role_words_stay_in_frozen_vocabulary: true,
        capture_context_present_on_every_binding: true,
        metadata_provenance_and_boundary_render_instead_of_dead_link: true,
        retired_line_reopen_is_refused_not_silently_reopened: true,
        missing_target_falls_back_to_metadata_only_exit: true,
        stale_import_blocks_on_trust_prerequisite: true,
        expired_snapshot_shows_metadata_not_dead_link: true,
        evidence_only_reopen_after_drift_stays_non_live: true,
        open_live_offered_only_when_handoff_clears: true,
        corpus_bound_to_screenshots_accessibility_cli_and_dashboards: true,
        corpus_referenced_by_release_and_support_not_ad_hoc: true,
        accessibility_routes_present_for_boundary_provenance_and_join: true,
        support_export_point_canonical_contracts: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

fn consumer_projection() -> DrillCorpusProjection {
    DrillCorpusProjection {
        shell_consumes_corpus: true,
        help_docs_consumes_corpus: true,
        support_consumes_corpus: true,
        review_incident_consumes_corpus: true,
        runbook_archive_consumes_corpus: true,
        release_center_consumes_corpus: true,
        companion_export_consumes_corpus: true,
        program_governance_consumes_corpus: true,
        cli_export_consumes_corpus: true,
        every_object_class_stated_by_two_or_more_consumers: true,
        non_live_grammar_identical_for_same_fixture: true,
        non_live_boundary_disclosed_not_hidden: true,
        drill_maps_back_to_one_historical_reference_object: true,
    }
}

fn proof_freshness() -> DrillCorpusProofFreshness {
    DrillCorpusProofFreshness {
        proof_freshness_slo_hours: M5_HISTORICAL_EVIDENCE_DRILL_PROOF_SLO_HOURS,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_HISTORICAL_EVIDENCE_DRILL_SCHEMA_REF.to_owned(),
        M5_HISTORICAL_EVIDENCE_DRILL_DOC_REF.to_owned(),
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF.to_owned(),
        M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF.to_owned(),
    ];
    // The five object classes map to three canonical domain schemas; include each distinct one once.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5HistoricalReferenceObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    for domain in domains {
        refs.push(domain.to_owned());
    }
    refs
}

fn packet_from_bindings(
    packet_id: &str,
    surface_label: &str,
    drill_bindings: Vec<DrillCorpusBinding>,
) -> M5HistoricalEvidenceDrillCorpusPacket {
    M5HistoricalEvidenceDrillCorpusPacket::new(M5HistoricalEvidenceDrillCorpusPacketInput {
        packet_id: packet_id.to_owned(),
        surface_label: surface_label.to_owned(),
        drill_bindings,
        downgrade_triggers: HistoricalEvidenceDrillDowngradeTrigger::ALL.to_vec(),
        consumer_surfaces: M5HistoricalReferenceConsumerSurface::ALL.to_vec(),
        trust_review: trust_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: SEED_REDACTION_CLASS.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// The canonical, checked-in historical-evidence drill corpus.
pub fn seeded_m5_historical_evidence_drill_corpus() -> M5HistoricalEvidenceDrillCorpusPacket {
    packet_from_bindings(
        M5_HISTORICAL_EVIDENCE_DRILL_PACKET_ID,
        "M5 historical-evidence drill corpus (fixtures and regression drills)",
        build_bindings(|_, default| default),
    )
}

/// Fixture: two preserved live-target-joinable drills narrowed to missing-live-target, proving the corpus
/// surfaces the missing-target blocker, drops the open-live affordance, and falls back to a metadata-only exit.
pub fn seeded_m5_historical_evidence_drill_corpus_missing_target_narrowed(
) -> M5HistoricalEvidenceDrillCorpusPacket {
    packet_from_bindings(
        "m5-historical-evidence-drill:missing-target:0001",
        "M5 historical-evidence drill corpus (missing-target narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "hed-support-support" => DrillScenario::MissingLiveTarget,
            "hed-runbook-runbook" => DrillScenario::MissingLiveTarget,
            _ => default,
        }),
    )
}

/// Fixture: two preserved live-target-joinable drills narrowed to expired-snapshot-metadata-only, proving the
/// metadata, capture time, and provenance render instead of a dead link once the content is gone.
pub fn seeded_m5_historical_evidence_drill_corpus_expired_snapshot_narrowed(
) -> M5HistoricalEvidenceDrillCorpusPacket {
    packet_from_bindings(
        "m5-historical-evidence-drill:expired-snapshot:0001",
        "M5 historical-evidence drill corpus (expired-snapshot narrowed)",
        build_bindings(|binding_id, default| match binding_id {
            "hed-retirement-release" => DrillScenario::ExpiredSnapshotMetadataOnlyFallback,
            "hed-review-review" => DrillScenario::ExpiredSnapshotMetadataOnlyFallback,
            _ => default,
        }),
    )
}
