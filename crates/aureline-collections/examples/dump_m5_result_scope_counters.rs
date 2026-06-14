//! Conformance dump for the M5 result-scope counter and hidden-narrowing chip
//! packet.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_collections::ship_result_scope_counters_and_hidden_narrowing_chips::*;
use aureline_collections::stabilize_filter_ast_saved_view_scope_pack_column_preset::{
    ScopeCounterVocabularyTerm, SelectAllMeaning,
};
use aureline_collections::DenseCollectionSurface;

const PACKET_ID: &str = "m5-result-scope-counter:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

const SCOPE_VOCABULARY_TERMS: [ScopeCounterVocabularyTerm; 8] = [
    ScopeCounterVocabularyTerm::Visible,
    ScopeCounterVocabularyTerm::Loaded,
    ScopeCounterVocabularyTerm::Matching,
    ScopeCounterVocabularyTerm::Selected,
    ScopeCounterVocabularyTerm::Approx,
    ScopeCounterVocabularyTerm::Exact,
    ScopeCounterVocabularyTerm::HiddenByPolicy,
    ScopeCounterVocabularyTerm::OutsideCurrentFilter,
];

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn qualified(
    kind: ResultCountKind,
    value: u64,
    exactness: CountExactness,
    freshness: CountFreshness,
    basis_label: &str,
) -> ResultScopeCount {
    ResultScopeCount {
        kind,
        value,
        exactness,
        freshness,
        basis_label: Some(basis_label.to_owned()),
    }
}

fn chip(cause: NarrowingCause, hidden_count: u64, chip_label: &str) -> HiddenNarrowingChip {
    HiddenNarrowingChip {
        cause,
        hidden_count,
        chip_label: chip_label.to_owned(),
        near_active_filters: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn binding(
    binding_id: &str,
    surface: DenseCollectionSurface,
    view_kind: CollectionViewKind,
    label: &str,
    counts: Vec<ResultScopeCount>,
    narrowing_chips: Vec<HiddenNarrowingChip>,
    posture: ResultScopePosture,
    virtualized: bool,
    provider_backed: bool,
    select_all_meaning: SelectAllMeaning,
) -> ResultScopeCounterBinding {
    ResultScopeCounterBinding {
        binding_id: binding_id.to_owned(),
        surface,
        view_kind,
        label_summary: label.to_owned(),
        counts,
        narrowing_chips,
        posture,
        virtualized,
        provider_backed,
        scope_vocabulary_terms: SCOPE_VOCABULARY_TERMS.to_vec(),
        counter_placement: CounterPlacement::NearActiveFilters,
        select_all_meaning,
        all_matching_requires_explicit_step: true,
        survives_reopen: true,
        evidence_refs: refs(&[&format!("evidence:binding:{binding_id}")]),
    }
}

fn bindings() -> Vec<ResultScopeCounterBinding> {
    vec![
        // Complete client list: every count exact and fresh.
        binding(
            "counter:pipeline-run-list:0001",
            DenseCollectionSurface::PipelineRunList,
            CollectionViewKind::List,
            "Pipeline run list with a complete, exact, current dataset",
            vec![
                ResultScopeCount::exact(ResultCountKind::Visible, 40),
                ResultScopeCount::exact(ResultCountKind::Loaded, 184),
                ResultScopeCount::exact(ResultCountKind::Matching, 184),
                ResultScopeCount::exact(ResultCountKind::Total, 184),
            ],
            Vec::new(),
            ResultScopePosture::ClientComplete,
            true,
            false,
            SelectAllMeaning::AllMatchingAfterExplicitExpansion,
        ),
        // Review queue with policy + provider narrowing disclosed as chips.
        binding(
            "counter:review-queue:0001",
            DenseCollectionSurface::ReviewQueue,
            CollectionViewKind::Queue,
            "Review queue with policy and provider narrowing disclosed near the filters",
            vec![
                ResultScopeCount::exact(ResultCountKind::Visible, 30),
                ResultScopeCount::exact(ResultCountKind::Loaded, 152),
                ResultScopeCount::exact(ResultCountKind::Matching, 152),
                ResultScopeCount::exact(ResultCountKind::Total, 200),
                ResultScopeCount::exact(ResultCountKind::HiddenByScope, 48),
            ],
            vec![
                chip(
                    NarrowingCause::Policy,
                    30,
                    "30 items hidden by your review-access policy",
                ),
                chip(
                    NarrowingCause::Workset,
                    18,
                    "18 items outside the active workset",
                ),
            ],
            ResultScopePosture::ClientComplete,
            true,
            false,
            SelectAllMeaning::AllMatchingAfterExplicitExpansion,
        ),
        // Incident list: stale + partial counts from a pending refresh.
        binding(
            "counter:incident-list:0001",
            DenseCollectionSurface::IncidentList,
            CollectionViewKind::List,
            "Incident list with stale matching counts and partial loaded rows pending refresh",
            vec![
                ResultScopeCount::exact(ResultCountKind::Visible, 25),
                qualified(
                    ResultCountKind::Loaded,
                    96,
                    CountExactness::Exact,
                    CountFreshness::Partial,
                    "96 incidents loaded so far; more are still streaming in",
                ),
                qualified(
                    ResultCountKind::Matching,
                    140,
                    CountExactness::Exact,
                    CountFreshness::Stale,
                    "140 matching as of the last refresh 45 seconds ago",
                ),
                qualified(
                    ResultCountKind::Total,
                    140,
                    CountExactness::Exact,
                    CountFreshness::Stale,
                    "140 total as of the last refresh",
                ),
            ],
            Vec::new(),
            ResultScopePosture::PartialDataPending,
            true,
            false,
            SelectAllMeaning::LoadedRows,
        ),
        // Graph tree: complete, exact, current hierarchical surface.
        binding(
            "counter:graph-list:0001",
            DenseCollectionSurface::GraphList,
            CollectionViewKind::Tree,
            "Reference graph tree with a complete, exact dataset",
            vec![
                ResultScopeCount::exact(ResultCountKind::Visible, 22),
                ResultScopeCount::exact(ResultCountKind::Loaded, 310),
                ResultScopeCount::exact(ResultCountKind::Matching, 310),
                ResultScopeCount::exact(ResultCountKind::Total, 310),
            ],
            Vec::new(),
            ResultScopePosture::ClientComplete,
            true,
            false,
            SelectAllMeaning::AllMatchingAfterExplicitExpansion,
        ),
        // Marketplace table: provider-paginated with approximate matching/total.
        binding(
            "counter:marketplace-results:0001",
            DenseCollectionSurface::MarketplaceResults,
            CollectionViewKind::Table,
            "Marketplace results table paged behind the provider with approximate totals",
            vec![
                ResultScopeCount::exact(ResultCountKind::Visible, 24),
                ResultScopeCount::exact(ResultCountKind::Loaded, 50),
                qualified(
                    ResultCountKind::Matching,
                    1200,
                    CountExactness::Approximate,
                    CountFreshness::Fresh,
                    "about 1,200 matching; the exact count stays provider-side",
                ),
                qualified(
                    ResultCountKind::Total,
                    5000,
                    CountExactness::Approximate,
                    CountFreshness::Fresh,
                    "approximate catalog size reported by the marketplace provider",
                ),
            ],
            Vec::new(),
            ResultScopePosture::ProviderPaginated,
            true,
            true,
            SelectAllMeaning::ProviderSideQueryAfterReview,
        ),
        // Provider/admin table: narrow client window over a known larger set.
        binding(
            "counter:provider-admin-table:0001",
            DenseCollectionSurface::ProviderAdminTable,
            CollectionViewKind::Table,
            "Provider/admin table rendering a narrow client window with client-limited rows hidden",
            vec![
                ResultScopeCount::exact(ResultCountKind::Visible, 20),
                ResultScopeCount::exact(ResultCountKind::Loaded, 80),
                ResultScopeCount::exact(ResultCountKind::Matching, 240),
                ResultScopeCount::exact(ResultCountKind::Total, 250),
                ResultScopeCount::exact(ResultCountKind::HiddenByScope, 10),
            ],
            vec![chip(
                NarrowingCause::Client,
                10,
                "10 rows not loaded on this narrow client window",
            )],
            ResultScopePosture::NarrowClientWindowed,
            true,
            true,
            SelectAllMeaning::AllMatchingAfterExplicitExpansion,
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
        "schemas/collections/filter_ast.schema.json",
        "schemas/collections/saved_view.schema.json",
        "schemas/collections/selection-scope.schema.json",
    ])
}

fn packet() -> ResultScopeCounterPacket {
    ResultScopeCounterPacket::new(ResultScopeCounterPacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "M5 Result-Scope Counters And Hidden-Narrowing Chips".to_owned(),
        bindings: bindings(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
