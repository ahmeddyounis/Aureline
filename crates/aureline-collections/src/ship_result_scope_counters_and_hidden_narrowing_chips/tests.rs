use super::*;

const PACKET_ID: &str = "m5-result-scope-counter:test:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn all_scope_terms() -> Vec<ScopeCounterVocabularyTerm> {
    REQUIRED_SCOPE_VOCABULARY_TERMS.to_vec()
}

fn exact_counts(visible: u64, loaded: u64, matching: u64, total: u64) -> Vec<ResultScopeCount> {
    vec![
        ResultScopeCount::exact(ResultCountKind::Visible, visible),
        ResultScopeCount::exact(ResultCountKind::Loaded, loaded),
        ResultScopeCount::exact(ResultCountKind::Matching, matching),
        ResultScopeCount::exact(ResultCountKind::Total, total),
    ]
}

fn current_binding(
    binding_id: &str,
    surface: DenseCollectionSurface,
    view_kind: CollectionViewKind,
) -> ResultScopeCounterBinding {
    ResultScopeCounterBinding {
        binding_id: binding_id.to_owned(),
        surface,
        view_kind,
        label_summary: "Result-scope counters".to_owned(),
        counts: exact_counts(40, 200, 200, 200),
        narrowing_chips: Vec::new(),
        posture: ResultScopePosture::ClientComplete,
        virtualized: true,
        provider_backed: false,
        scope_vocabulary_terms: all_scope_terms(),
        counter_placement: CounterPlacement::NearActiveFilters,
        select_all_meaning: SelectAllMeaning::AllMatchingAfterExplicitExpansion,
        all_matching_requires_explicit_step: true,
        survives_reopen: true,
        evidence_refs: refs(&[&format!("evidence:{binding_id}")]),
    }
}

fn narrowed_binding(
    binding_id: &str,
    surface: DenseCollectionSurface,
    view_kind: CollectionViewKind,
) -> ResultScopeCounterBinding {
    let mut binding = current_binding(binding_id, surface, view_kind);
    // total=200, matching=170, hidden=30 split across policy + provider chips.
    binding.counts = vec![
        ResultScopeCount::exact(ResultCountKind::Visible, 40),
        ResultScopeCount::exact(ResultCountKind::Loaded, 170),
        ResultScopeCount::exact(ResultCountKind::Matching, 170),
        ResultScopeCount::exact(ResultCountKind::Total, 200),
        ResultScopeCount::exact(ResultCountKind::HiddenByScope, 30),
    ];
    binding.narrowing_chips = vec![
        HiddenNarrowingChip {
            cause: NarrowingCause::Policy,
            hidden_count: 18,
            chip_label: "18 hidden by access policy".to_owned(),
            near_active_filters: true,
        },
        HiddenNarrowingChip {
            cause: NarrowingCause::Provider,
            hidden_count: 12,
            chip_label: "12 capped by the provider page limit".to_owned(),
            near_active_filters: true,
        },
    ];
    binding
}

fn provider_paginated_binding(
    binding_id: &str,
    surface: DenseCollectionSurface,
    view_kind: CollectionViewKind,
) -> ResultScopeCounterBinding {
    let mut binding = current_binding(binding_id, surface, view_kind);
    binding.posture = ResultScopePosture::ProviderPaginated;
    binding.provider_backed = true;
    binding.counts = vec![
        ResultScopeCount::exact(ResultCountKind::Visible, 25),
        ResultScopeCount::exact(ResultCountKind::Loaded, 50),
        ResultScopeCount {
            kind: ResultCountKind::Matching,
            value: 1200,
            exactness: CountExactness::Approximate,
            freshness: CountFreshness::Fresh,
            basis_label: Some("about 1,200 matching, exact total is provider-side".to_owned()),
        },
        ResultScopeCount {
            kind: ResultCountKind::Total,
            value: 5000,
            exactness: CountExactness::Approximate,
            freshness: CountFreshness::Fresh,
            basis_label: Some("approximate catalog size reported by the provider".to_owned()),
        },
    ];
    binding
}

fn stale_binding(
    binding_id: &str,
    surface: DenseCollectionSurface,
    view_kind: CollectionViewKind,
) -> ResultScopeCounterBinding {
    let mut binding = current_binding(binding_id, surface, view_kind);
    binding.posture = ResultScopePosture::PartialDataPending;
    binding.counts = vec![
        ResultScopeCount::exact(ResultCountKind::Visible, 30),
        ResultScopeCount {
            kind: ResultCountKind::Loaded,
            value: 120,
            exactness: CountExactness::Exact,
            freshness: CountFreshness::Partial,
            basis_label: Some("120 loaded so far; more rows still streaming in".to_owned()),
        },
        ResultScopeCount {
            kind: ResultCountKind::Matching,
            value: 300,
            exactness: CountExactness::Exact,
            freshness: CountFreshness::Stale,
            basis_label: Some("300 matching as of the last refresh 40s ago".to_owned()),
        },
        ResultScopeCount {
            kind: ResultCountKind::Total,
            value: 300,
            exactness: CountExactness::Exact,
            freshness: CountFreshness::Stale,
            basis_label: Some("300 total as of the last refresh".to_owned()),
        },
    ];
    binding
}

fn baseline_bindings() -> Vec<ResultScopeCounterBinding> {
    vec![
        current_binding(
            "counter:pipeline",
            DenseCollectionSurface::PipelineRunList,
            CollectionViewKind::List,
        ),
        narrowed_binding(
            "counter:review",
            DenseCollectionSurface::ReviewQueue,
            CollectionViewKind::Queue,
        ),
        stale_binding(
            "counter:incident",
            DenseCollectionSurface::IncidentList,
            CollectionViewKind::List,
        ),
        current_binding(
            "counter:graph",
            DenseCollectionSurface::GraphList,
            CollectionViewKind::Tree,
        ),
        provider_paginated_binding(
            "counter:marketplace",
            DenseCollectionSurface::MarketplaceResults,
            CollectionViewKind::Table,
        ),
        current_binding(
            "counter:admin",
            DenseCollectionSurface::ProviderAdminTable,
            CollectionViewKind::Table,
        ),
    ]
}

fn guardrails() -> ResultScopeGuardrails {
    ResultScopeGuardrails {
        hidden_narrowing_always_visible: true,
        exact_versus_approximate_labeled: true,
        visible_loaded_matching_never_blurred: true,
        counters_survive_virtualization_and_reopen: true,
        provider_policy_narrowing_not_generic: true,
    }
}

fn consumer_projection() -> ResultScopeConsumerProjection {
    ResultScopeConsumerProjection {
        product_renders_counters_and_chips: true,
        diagnostics_reconstructs_scope_truth: true,
        support_export_reuses_records: true,
        docs_help_reuses_vocabulary: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        RESULT_SCOPE_COUNTER_SCHEMA_REF,
        RESULT_SCOPE_COUNTER_DOC_REF,
        RESULT_SCOPE_COUNTER_ARTIFACT_REF,
    ])
}

fn baseline_packet() -> ResultScopeCounterPacket {
    ResultScopeCounterPacket::new(ResultScopeCounterPacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "Test result-scope counter packet".to_owned(),
        bindings: baseline_bindings(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

#[test]
fn baseline_packet_validates() {
    assert!(baseline_packet().validate().is_empty());
}

#[test]
fn missing_required_count_is_rejected() {
    let mut binding = current_binding(
        "b",
        DenseCollectionSurface::PipelineRunList,
        CollectionViewKind::List,
    );
    binding
        .counts
        .retain(|count| count.kind != ResultCountKind::Total);
    assert!(!binding.has_required_counts());

    let mut packet = baseline_packet();
    packet.bindings[0] = binding;
    let violations = packet.validate();
    assert!(violations.contains(&ResultScopeCounterViolation::RequiredCountMissing));
}

#[test]
fn non_monotonic_counts_are_rejected() {
    let mut binding = current_binding(
        "b",
        DenseCollectionSurface::PipelineRunList,
        CollectionViewKind::List,
    );
    // visible 500 > loaded 200 breaks nesting.
    binding.counts = exact_counts(500, 200, 200, 200);
    assert!(!binding.counts_monotonic());
}

#[test]
fn approximate_count_without_basis_label_is_rejected() {
    let mut binding = current_binding(
        "b",
        DenseCollectionSurface::PipelineRunList,
        CollectionViewKind::List,
    );
    binding.posture = ResultScopePosture::ProviderPaginated;
    binding.counts = vec![
        ResultScopeCount::exact(ResultCountKind::Visible, 25),
        ResultScopeCount::exact(ResultCountKind::Loaded, 50),
        ResultScopeCount::exact(ResultCountKind::Matching, 100),
        ResultScopeCount {
            kind: ResultCountKind::Total,
            value: 5000,
            exactness: CountExactness::Approximate,
            freshness: CountFreshness::Fresh,
            basis_label: None,
        },
    ];
    assert!(!binding.counts.iter().all(ResultScopeCount::is_valid));
}

#[test]
fn generic_basis_label_is_rejected() {
    let count = ResultScopeCount {
        kind: ResultCountKind::Total,
        value: 5000,
        exactness: CountExactness::Approximate,
        freshness: CountFreshness::Fresh,
        basis_label: Some("approximate".to_owned()),
    };
    assert!(!count.is_valid());
}

#[test]
fn hidden_count_must_reconcile_with_chips() {
    let mut binding = narrowed_binding(
        "b",
        DenseCollectionSurface::ReviewQueue,
        CollectionViewKind::Queue,
    );
    // Break the chip sum so it no longer matches the hidden-by-scope value.
    binding.narrowing_chips[0].hidden_count = 5;
    assert!(!binding.hidden_reconciles());
    assert!(!binding.narrowing_consistent());
}

#[test]
fn hidden_count_must_equal_total_minus_matching() {
    let mut binding = narrowed_binding(
        "b",
        DenseCollectionSurface::ReviewQueue,
        CollectionViewKind::Queue,
    );
    // total - matching = 30, but make hidden + chips claim 25.
    binding.counts = vec![
        ResultScopeCount::exact(ResultCountKind::Visible, 40),
        ResultScopeCount::exact(ResultCountKind::Loaded, 170),
        ResultScopeCount::exact(ResultCountKind::Matching, 170),
        ResultScopeCount::exact(ResultCountKind::Total, 200),
        ResultScopeCount::exact(ResultCountKind::HiddenByScope, 25),
    ];
    binding.narrowing_chips = vec![HiddenNarrowingChip {
        cause: NarrowingCause::Policy,
        hidden_count: 25,
        chip_label: "25 hidden by policy".to_owned(),
        near_active_filters: true,
    }];
    assert!(!binding.hidden_reconciles());
}

#[test]
fn chip_off_filters_is_rejected() {
    let mut binding = narrowed_binding(
        "b",
        DenseCollectionSurface::ReviewQueue,
        CollectionViewKind::Queue,
    );
    binding.narrowing_chips[1].near_active_filters = false;
    assert!(!binding.narrowing_consistent());
}

#[test]
fn duplicate_narrowing_cause_is_rejected() {
    let mut binding = narrowed_binding(
        "b",
        DenseCollectionSurface::ReviewQueue,
        CollectionViewKind::Queue,
    );
    binding.narrowing_chips[1].cause = NarrowingCause::Policy;
    assert!(!binding.narrowing_consistent());
}

#[test]
fn client_complete_posture_rejects_approximate_counts() {
    let mut binding = current_binding(
        "b",
        DenseCollectionSurface::PipelineRunList,
        CollectionViewKind::List,
    );
    binding.counts[3].exactness = CountExactness::Approximate;
    binding.counts[3].basis_label = Some("about two hundred runs".to_owned());
    assert!(!binding.posture_consistent());
}

#[test]
fn provider_paginated_posture_requires_approximate_total() {
    let mut binding = provider_paginated_binding(
        "b",
        DenseCollectionSurface::MarketplaceResults,
        CollectionViewKind::Table,
    );
    binding.counts[3].exactness = CountExactness::Exact;
    binding.counts[3].basis_label = None;
    assert!(!binding.posture_consistent());
}

#[test]
fn footer_only_placement_is_rejected() {
    let mut binding = current_binding(
        "b",
        DenseCollectionSurface::PipelineRunList,
        CollectionViewKind::List,
    );
    binding.counter_placement = CounterPlacement::FooterOnly;
    assert!(!binding.is_complete());

    let mut packet = baseline_packet();
    packet.bindings[0] = binding;
    assert!(packet
        .validate()
        .contains(&ResultScopeCounterViolation::CounterPlacementBuried));
}

#[test]
fn visible_as_all_matching_without_step_is_rejected() {
    let mut binding = current_binding(
        "b",
        DenseCollectionSurface::PipelineRunList,
        CollectionViewKind::List,
    );
    binding.all_matching_requires_explicit_step = false;
    assert!(!binding.is_complete());

    let mut packet = baseline_packet();
    packet.bindings[0] = binding;
    assert!(packet
        .validate()
        .contains(&ResultScopeCounterViolation::AllMatchingWithoutExplicitStep));
}

#[test]
fn incomplete_scope_vocabulary_is_rejected() {
    let mut binding = current_binding(
        "b",
        DenseCollectionSurface::PipelineRunList,
        CollectionViewKind::List,
    );
    binding.scope_vocabulary_terms = vec![ScopeCounterVocabularyTerm::Visible];
    assert!(!binding.scope_vocabulary_ok());
}

#[test]
fn missing_required_surface_is_rejected() {
    let mut packet = baseline_packet();
    packet
        .bindings
        .retain(|binding| binding.surface != DenseCollectionSurface::ProviderAdminTable);
    assert!(packet
        .validate()
        .contains(&ResultScopeCounterViolation::RequiredSurfaceMissing));
}

#[test]
fn missing_view_kind_is_rejected() {
    let mut packet = baseline_packet();
    // Drop every tree binding so the tree view kind is unrepresented.
    for binding in &mut packet.bindings {
        if binding.view_kind == CollectionViewKind::Tree {
            binding.view_kind = CollectionViewKind::List;
        }
    }
    assert!(packet
        .validate()
        .contains(&ResultScopeCounterViolation::RequiredViewKindMissing));
}

#[test]
fn missing_narrowing_case_is_rejected() {
    let mut packet = baseline_packet();
    for binding in &mut packet.bindings {
        binding.narrowing_chips.clear();
        binding
            .counts
            .retain(|count| count.kind != ResultCountKind::HiddenByScope);
    }
    assert!(packet
        .validate()
        .contains(&ResultScopeCounterViolation::NarrowingCaseMissing));
}

#[test]
fn reconstruction_recovers_scope_truth() {
    let packet = baseline_packet();
    let reconstructions = packet.reconstructions();
    assert_eq!(reconstructions.len(), packet.bindings.len());

    let review = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.binding_id == "counter:review")
        .expect("review reconstruction present");
    assert_eq!(review.matching_value, Some(170));
    assert_eq!(review.total_value, Some(200));
    assert_eq!(review.hidden_by_scope_value, Some(30));
    assert!(review
        .narrowing_cause_tokens
        .contains(&"provider".to_owned()));
    assert!(review.all_matching_requires_explicit_step);

    let marketplace = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.binding_id == "counter:marketplace")
        .expect("marketplace reconstruction present");
    assert!(marketplace.has_approximate_count);
    assert!(marketplace.provider_backed);

    let incident = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.binding_id == "counter:incident")
        .expect("incident reconstruction present");
    assert!(incident.has_qualified_freshness);
}

#[test]
fn export_is_metadata_safe() {
    let packet = baseline_packet();
    let json = packet.export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("bearer "));
    assert!(packet.validate().is_empty());
}

#[test]
fn record_kind_and_schema_version_are_pinned() {
    let packet = baseline_packet();
    assert_eq!(packet.record_kind, RESULT_SCOPE_COUNTER_RECORD_KIND);
    assert_eq!(packet.schema_version, RESULT_SCOPE_COUNTER_SCHEMA_VERSION);
}

#[test]
fn checked_in_export_validates() {
    let packet = current_m5_result_scope_counter_export()
        .expect("checked-in result-scope counter export parses and validates");
    assert_eq!(packet.packet_id, "m5-result-scope-counter:stable:0001");
    assert!(packet.validate().is_empty());
    for required in REQUIRED_COUNTER_SURFACES {
        assert!(packet.represented_surfaces().contains(&required));
    }
    for required in CollectionViewKind::ALL {
        assert!(packet.represented_view_kinds().contains(&required));
    }
    assert!(packet.narrowed_binding_count() >= 1);
}

#[test]
fn round_trips_through_json() {
    let packet = baseline_packet();
    let json = packet.export_safe_json();
    let parsed: ResultScopeCounterPacket = serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(parsed, packet);
}
