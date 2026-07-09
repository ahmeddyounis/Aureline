//! Canonical seed builders for the run-comparison-table / compare-guard-banner controls.
//!
//! These builders are the single producer of the checked-in support export and the scenario
//! fixtures. The headless emitter and the inline tests both call them so the in-code components,
//! the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical run-comparison-table / compare-guard-banner packet.
pub const RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_PACKET_ID: &str =
    "m5-run-comparison-table-compare-guard-banner-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn comparison_table_source_refs() -> Vec<String> {
    strings(&[
        M5_RUN_COMPARISON_TABLE_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
    ])
}

fn guard_banner_source_refs() -> Vec<String> {
    strings(&[
        M5_COMPARE_GUARD_BANNER_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
    ])
}

fn comparison_table_downgrade_triggers() -> Vec<M5ExperimentDowngradeTrigger> {
    vec![
        M5ExperimentDowngradeTrigger::ComparabilityOverstated,
        M5ExperimentDowngradeTrigger::CodeRevisionUnstated,
        M5ExperimentDowngradeTrigger::CachedStateHidden,
        M5ExperimentDowngradeTrigger::AlternateStateLabelInvented,
        M5ExperimentDowngradeTrigger::ProofStale,
    ]
}

fn guard_banner_downgrade_triggers() -> Vec<M5ExperimentDowngradeTrigger> {
    vec![
        M5ExperimentDowngradeTrigger::ComparabilityOverstated,
        M5ExperimentDowngradeTrigger::EnvironmentFingerprintUnstated,
        M5ExperimentDowngradeTrigger::CachedStateHidden,
        M5ExperimentDowngradeTrigger::AlternateStateLabelInvented,
        M5ExperimentDowngradeTrigger::ProofStale,
    ]
}

/// Named-field seed for a run comparison table. Keeps the honest inputs explicit so the derived
/// fairness class, the fair-baseline claim, and the state-conditional notes never drift.
struct ComparisonTableSeed<'a> {
    table_id: &'a str,
    comparison_label: &'a str,
    comparison_axis: M5ComparisonAxisClass,
    comparability_state: M5ComparabilityState,
    baseline_run_id: &'a str,
    baseline_run_label: &'a str,
    candidate_run_id: &'a str,
    candidate_run_label: &'a str,
    metric_value_note: &'a str,
    delta_note: &'a str,
    threshold_state_note: &'a str,
    confidence_note: &'a str,
    comparator_type_note: &'a str,
    code_difference_note: &'a str,
    data_difference_note: &'a str,
    environment_difference_note: &'a str,
    hardware_difference_note: &'a str,
    comparability_and_parity_note: &'a str,
    context_note: &'a str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &'a str,
    table_actions: Vec<RunComparisonAction>,
    dispositions: Vec<M5ExperimentDisposition>,
}

fn comparison_table(seed: ComparisonTableSeed<'_>) -> RunComparisonTable {
    let disclosure = resolve_run_comparison(seed.comparability_state);
    RunComparisonTable {
        component: M5ExperimentComponentFamily::RunComparisonTable,
        table_id: seed.table_id.to_owned(),
        comparison_label: seed.comparison_label.to_owned(),
        comparison_axis: seed.comparison_axis,
        comparability_state: seed.comparability_state,
        fairness_class: disclosure.fairness_class,
        claims_fair_baseline: disclosure.is_fair_baseline,
        baseline_run_id: seed.baseline_run_id.to_owned(),
        baseline_run_label: seed.baseline_run_label.to_owned(),
        candidate_run_id: seed.candidate_run_id.to_owned(),
        candidate_run_label: seed.candidate_run_label.to_owned(),
        metric_value_note: seed.metric_value_note.to_owned(),
        delta_note: seed.delta_note.to_owned(),
        threshold_state_note: seed.threshold_state_note.to_owned(),
        confidence_note: seed.confidence_note.to_owned(),
        comparator_type_note: seed.comparator_type_note.to_owned(),
        code_difference_note: seed.code_difference_note.to_owned(),
        data_difference_note: seed.data_difference_note.to_owned(),
        environment_difference_note: seed.environment_difference_note.to_owned(),
        hardware_difference_note: seed.hardware_difference_note.to_owned(),
        caveat_note: if disclosure.needs_caveat_note {
            "Comparable only with caveats; read the caveats before trusting this delta".to_owned()
        } else {
            String::new()
        },
        not_comparable_note: if disclosure.needs_not_comparable_note {
            "Not comparable; the two runs do not share a fair baseline, so the delta is not a fair result"
                .to_owned()
        } else {
            String::new()
        },
        confounder_note: if disclosure.needs_confounder_note {
            "Confounded; a changed factor moves with the metric, so the delta is not attributable"
                .to_owned()
        } else {
            String::new()
        },
        insufficient_overlap_note: if disclosure.needs_insufficient_overlap_note {
            "Insufficient overlap; the runs share too little to establish a fair comparison"
                .to_owned()
        } else {
            String::new()
        },
        unknown_comparability_note: if disclosure.needs_unknown_comparability_note {
            "Unknown comparability; parity cannot be established from the recorded lineage"
                .to_owned()
        } else {
            String::new()
        },
        comparability_and_parity_note: seed.comparability_and_parity_note.to_owned(),
        context_note: seed.context_note.to_owned(),
        deep_link_kind: seed.deep_link_kind,
        deep_link_ref: seed.deep_link_ref.to_owned(),
        table_actions: seed.table_actions,
        dispositions: seed.dispositions,
        downgrade_triggers: comparison_table_downgrade_triggers(),
        required_labels: M5ExperimentRequiredLabel::ALL.to_vec(),
        surface_families: M5ExperimentSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ExperimentDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ExperimentAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "comparison_label",
            "comparison_axis",
            "comparability_state",
            "fairness_class",
            "baseline_run_id",
            "candidate_run_id",
            "delta_note",
            "code_difference_note",
            "data_difference_note",
            "environment_difference_note",
            "hardware_difference_note",
            "deep_link_kind",
        ]),
        source_contract_refs: comparison_table_source_refs(),
        masks_provenance_or_sensitivity_state: false,
        hides_baseline_or_candidate_identity: false,
        hides_difference_factors_beside_delta: false,
        implies_apples_to_apples_without_parity: false,
        invents_alternate_state_label: false,
    }
}

/// Named-field seed for a compare guard banner. Keeps the honest inputs explicit so the derived
/// guard comparability class, the permits-fair-comparison claim, and the state-conditional notes
/// never drift.
struct GuardBannerSeed<'a> {
    banner_id: &'a str,
    banner_label: &'a str,
    guard_reason: M5CompareGuardReason,
    guard_state: M5CompareGuardState,
    comparability_disclosure_note: &'a str,
    missing_lineage_fields_note: &'a str,
    changed_factors_note: &'a str,
    redaction_note: &'a str,
    guard_reason_note: &'a str,
    comparability_and_parity_note: &'a str,
    context_note: &'a str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &'a str,
    banner_actions: Vec<CompareGuardAction>,
    dispositions: Vec<M5ExperimentDisposition>,
}

fn guard_banner(seed: GuardBannerSeed<'_>) -> CompareGuardBanner {
    let disclosure = resolve_compare_guard(seed.guard_state);
    CompareGuardBanner {
        component: M5ExperimentComponentFamily::CompareGuardBanner,
        banner_id: seed.banner_id.to_owned(),
        banner_label: seed.banner_label.to_owned(),
        guard_reason: seed.guard_reason,
        guard_state: seed.guard_state,
        guard_class: disclosure.guard_class,
        claims_permits_fair_comparison: disclosure.permits_fair_comparison,
        comparability_disclosure_note: seed.comparability_disclosure_note.to_owned(),
        missing_lineage_fields_note: seed.missing_lineage_fields_note.to_owned(),
        changed_factors_note: seed.changed_factors_note.to_owned(),
        redaction_note: seed.redaction_note.to_owned(),
        guard_reason_note: seed.guard_reason_note.to_owned(),
        partial_comparability_note: if disclosure.needs_partial_comparability_note {
            "Only partially comparable; the guard permits the compare but the caveats stand"
                .to_owned()
        } else {
            String::new()
        },
        override_warning: if disclosure.needs_override_warning {
            "Guard overridden by explicit choice; this comparison is not certified fair".to_owned()
        } else {
            String::new()
        },
        blocked_note: if disclosure.needs_blocked_note {
            "Comparison blocked; the parity evidence is incomplete, so no fair delta is shown"
                .to_owned()
        } else {
            String::new()
        },
        guard_unavailable_note: if disclosure.needs_guard_unavailable_note {
            "Guard unavailable; comparability cannot be established from the recorded lineage"
                .to_owned()
        } else {
            String::new()
        },
        comparability_and_parity_note: seed.comparability_and_parity_note.to_owned(),
        context_note: seed.context_note.to_owned(),
        deep_link_kind: seed.deep_link_kind,
        deep_link_ref: seed.deep_link_ref.to_owned(),
        banner_actions: seed.banner_actions,
        dispositions: seed.dispositions,
        downgrade_triggers: guard_banner_downgrade_triggers(),
        required_labels: M5ExperimentRequiredLabel::ALL.to_vec(),
        surface_families: M5ExperimentSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ExperimentDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ExperimentAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "banner_label",
            "guard_reason",
            "guard_state",
            "guard_class",
            "comparability_disclosure_note",
            "missing_lineage_fields_note",
            "changed_factors_note",
            "redaction_note",
            "deep_link_kind",
        ]),
        source_contract_refs: guard_banner_source_refs(),
        masks_provenance_or_sensitivity_state: false,
        hides_baseline_or_candidate_identity: false,
        hides_difference_factors_beside_delta: false,
        implies_apples_to_apples_without_parity: false,
        invents_alternate_state_label: false,
    }
}

fn comparison_tables() -> Vec<RunComparisonTable> {
    use DeepLinkKind as Link;
    use M5ComparabilityState as State;
    use M5ComparisonAxisClass as Axis;
    use M5ExperimentDisposition as Disp;
    use RunComparisonAction as Action;

    vec![
        // 1. Metric delta, comparable → fair baseline.
        comparison_table(ComparisonTableSeed {
            table_id: "rc-metric-001",
            comparison_label: "Ranker NDCG@10: v7 vs v6",
            comparison_axis: Axis::MetricDelta,
            comparability_state: State::Comparable,
            baseline_run_id: "run-notebook-1041",
            baseline_run_label: "Baseline run v6 (notebook cell 12)",
            candidate_run_id: "run-notebook-1042",
            candidate_run_label: "Candidate run v7 (notebook cell 12)",
            metric_value_note: "NDCG@10: baseline 0.408, candidate 0.412",
            delta_note: "Delta +0.004 (+0.9%) in favor of the candidate",
            threshold_state_note: "Threshold: above the +0.005 promotion gate is not met",
            confidence_note: "Confidence: 200 held-out queries, bootstrap CI excludes zero",
            comparator_type_note: "Comparator: paired held-out evaluation on the same query set",
            code_difference_note: "Code: identical revision; only the ranker weights changed",
            data_difference_note: "Data: identical eval snapshot pinned to the same catalog anchor",
            environment_difference_note: "Environment: identical fingerprint env-cuda-12-torch-2-3",
            hardware_difference_note: "Hardware: identical single-A100 profile class",
            comparability_and_parity_note:
                "Fair baseline: code, data, environment, and hardware all match, so the delta is attributable",
            context_note: "Confirm the promotion gate before shipping the candidate",
            deep_link_kind: Link::RunObject,
            deep_link_ref: "run:notebook-1042/compare/ndcg-v7-v6",
            table_actions: vec![
                Action::OpenBaselineRun,
                Action::OpenCurrentRun,
                Action::ExportComparison,
                Action::OpenFullLineage,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::LocalRun, Disp::Reproducible],
        }),
        // 2. Param diff, comparable with caveats → caveated baseline.
        comparison_table(ComparisonTableSeed {
            table_id: "rc-param-002",
            comparison_label: "Learning-rate sweep: lr=3e-4 vs lr=1e-4",
            comparison_axis: Axis::ParamDiff,
            comparability_state: State::ComparableWithCaveats,
            baseline_run_id: "run-notebook-1039",
            baseline_run_label: "Baseline lr=1e-4 (notebook cell 9)",
            candidate_run_id: "run-notebook-1043",
            candidate_run_label: "Candidate lr=3e-4 (notebook cell 9)",
            metric_value_note: "Val loss: baseline 0.284, candidate 0.271",
            delta_note: "Delta -0.013 val loss in favor of the candidate",
            threshold_state_note: "Threshold: below the 0.275 target is met by the candidate",
            confidence_note: "Confidence: single seed each; variance not yet estimated",
            comparator_type_note: "Comparator: final-epoch validation loss",
            code_difference_note: "Code: identical revision; only the learning-rate parameter changed",
            data_difference_note: "Data: identical training and validation snapshot",
            environment_difference_note: "Environment: identical fingerprint env-cuda-12-torch-2-3",
            hardware_difference_note: "Hardware: identical single-A100 profile class",
            comparability_and_parity_note:
                "Caveated: only the swept parameter differs, but a single seed limits confidence",
            context_note: "Re-run with multiple seeds before treating the parameter win as settled",
            deep_link_kind: Link::NotebookLocation,
            deep_link_ref: "notebook:sweep.ipynb#cell-9",
            table_actions: vec![
                Action::OpenBaselineRun,
                Action::OpenCurrentRun,
                Action::ExportComparison,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::LocalRun, Disp::LikelyReproducible],
        }),
        // 3. Dataset diff, not comparable → unfair baseline (not-comparable note).
        comparison_table(ComparisonTableSeed {
            table_id: "rc-dataset-003",
            comparison_label: "Accuracy: managed run vs local run",
            comparison_axis: Axis::DatasetDiff,
            comparability_state: State::NotComparable,
            baseline_run_id: "run-notebook-1042",
            baseline_run_label: "Local baseline (notebook cell 12)",
            candidate_run_id: "run-managed-2207",
            candidate_run_label: "Managed candidate run",
            metric_value_note: "Accuracy: baseline 0.86, candidate 0.91",
            delta_note: "Delta +0.05 accuracy in favor of the candidate",
            threshold_state_note: "Threshold: cannot be assessed across mismatched datasets",
            confidence_note: "Confidence: not meaningful; the eval sets differ",
            comparator_type_note: "Comparator: top-1 accuracy on each run's own eval set",
            code_difference_note: "Code: revisions differ by fourteen commits",
            data_difference_note: "Data: DIFFERENT eval snapshots; the managed run used a newer catalog",
            environment_difference_note: "Environment: managed-runner fingerprint differs from local",
            hardware_difference_note: "Hardware: managed multi-GPU profile vs local single-A100",
            comparability_and_parity_note:
                "Not a fair baseline: the eval datasets differ, so the accuracy delta is not attributable",
            context_note: "Re-run both on the same eval snapshot before comparing accuracy",
            deep_link_kind: Link::DatasetCatalogAnchor,
            deep_link_ref: "dataset:catalog/eval-snapshot-mismatch",
            table_actions: vec![
                Action::OpenBaselineRun,
                Action::OpenCurrentRun,
                Action::ExportComparison,
                Action::OpenFullLineage,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::ManagedRun, Disp::NeedsRerun],
        }),
        // 4. Env diff, confounded → unfair baseline (confounder note).
        comparison_table(ComparisonTableSeed {
            table_id: "rc-env-004",
            comparison_label: "Throughput: torch 2.3 vs torch 2.1",
            comparison_axis: Axis::EnvDiff,
            comparability_state: State::Confounded,
            baseline_run_id: "run-notebook-1042",
            baseline_run_label: "Baseline on torch 2.3",
            candidate_run_id: "run-notebook-1038",
            candidate_run_label: "Candidate on torch 2.1",
            metric_value_note: "Throughput: baseline 1.4k tok/s, candidate 1.1k tok/s",
            delta_note: "Delta -0.3k tok/s; the candidate appears slower",
            threshold_state_note: "Threshold: not meaningful while the environment is confounded",
            confidence_note: "Confidence: the environment change moves with the metric",
            comparator_type_note: "Comparator: median tokens per second over the eval loop",
            code_difference_note: "Code: identical revision",
            data_difference_note: "Data: identical eval snapshot",
            environment_difference_note: "Environment: DIFFERENT torch build (2.3 vs 2.1) changes kernels",
            hardware_difference_note: "Hardware: identical single-A100 profile class",
            comparability_and_parity_note:
                "Confounded: the torch version confounds throughput, so the delta is not attributable to code",
            context_note: "Pin the same torch build before reading a throughput delta",
            deep_link_kind: Link::RunObject,
            deep_link_ref: "run:notebook-1042/compare/throughput-env-confounded",
            table_actions: vec![
                Action::OpenBaselineRun,
                Action::OpenCurrentRun,
                Action::ExportComparison,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::LocalRun, Disp::NeedsRerun],
        }),
        // 5. Code revision diff, insufficient overlap → unproven baseline (insufficient note).
        comparison_table(ComparisonTableSeed {
            table_id: "rc-code-005",
            comparison_label: "F1: imported run vs local run",
            comparison_axis: Axis::CodeRevisionDiff,
            comparability_state: State::InsufficientOverlap,
            baseline_run_id: "run-notebook-1042",
            baseline_run_label: "Local baseline (notebook cell 12)",
            candidate_run_id: "run-imported-0031",
            candidate_run_label: "Imported candidate (external tracker)",
            metric_value_note: "F1: baseline 0.72, candidate 0.75 (partial labels)",
            delta_note: "Delta +0.03 F1, but computed over a small shared slice",
            threshold_state_note: "Threshold: cannot be assessed with insufficient overlap",
            confidence_note: "Confidence: only 40 shared examples overlap between the runs",
            comparator_type_note: "Comparator: F1 over the intersecting label set",
            code_difference_note: "Code: revisions differ; the imported run's revision is unrecorded",
            data_difference_note: "Data: only a small slice overlaps between the two eval sets",
            environment_difference_note: "Environment: imported fingerprint is unavailable",
            hardware_difference_note: "Hardware: imported profile class is unknown",
            comparability_and_parity_note:
                "Unproven: too little overlaps to establish a fair baseline, so the F1 delta is not settled",
            context_note: "Expand the shared eval slice before trusting the F1 delta",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:notebooks/compare-insufficient-overlap",
            table_actions: vec![
                Action::OpenBaselineRun,
                Action::OpenCurrentRun,
                Action::ExportComparison,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::ImportedRun, Disp::ContextIncomplete],
        }),
        // 6. Artifact diff, unknown comparability → unproven baseline (unknown note).
        comparison_table(ComparisonTableSeed {
            table_id: "rc-artifact-006",
            comparison_label: "Checkpoint size: attached vs local",
            comparison_axis: Axis::ArtifactDiff,
            comparability_state: State::UnknownComparability,
            baseline_run_id: "run-notebook-1042",
            baseline_run_label: "Local baseline checkpoint",
            candidate_run_id: "run-manual-attach-0009",
            candidate_run_label: "Manually attached checkpoint",
            metric_value_note: "Checkpoint size: baseline 1.2 GB, candidate 1.4 GB",
            delta_note: "Delta +0.2 GB, but the artifacts may not be the same kind",
            threshold_state_note: "Threshold: not applicable to an unverified artifact pair",
            confidence_note: "Confidence: the attached artifact's lineage is unknown",
            comparator_type_note: "Comparator: on-disk checkpoint byte size",
            code_difference_note: "Code: the attached artifact's producing revision is unknown",
            data_difference_note: "Data: the attached artifact's training data is unknown",
            environment_difference_note: "Environment: the attached artifact's fingerprint is unknown",
            hardware_difference_note: "Hardware: the attached artifact's profile class is unknown",
            comparability_and_parity_note:
                "Unproven: comparability is unknown, so this size delta is not a meaningful result",
            context_note: "Classify the attached artifact's lineage before comparing it",
            deep_link_kind: Link::NoDeepLink,
            deep_link_ref: "",
            table_actions: vec![
                Action::OpenBaselineRun,
                Action::OpenCurrentRun,
                Action::ExportComparison,
            ],
            dispositions: vec![Disp::ManualAttach, Disp::ContextIncomplete],
        }),
    ]
}

fn guard_banners() -> Vec<CompareGuardBanner> {
    use CompareGuardAction as Action;
    use DeepLinkKind as Link;
    use M5CompareGuardReason as Reason;
    use M5CompareGuardState as State;
    use M5ExperimentDisposition as Disp;

    vec![
        // 1. Dataset mismatch, comparison permitted → comparable permitted.
        guard_banner(GuardBannerSeed {
            banner_id: "cg-permitted-001",
            banner_label: "Comparison permitted after dataset check",
            guard_reason: Reason::DatasetMismatch,
            guard_state: State::ComparisonPermitted,
            comparability_disclosure_note:
                "Comparable: the dataset mismatch was reconciled to the same eval snapshot",
            missing_lineage_fields_note: "Missing lineage fields: none; both runs record full lineage",
            changed_factors_note: "Changed factors: none beyond the ranker weights under test",
            redaction_note: "Redaction: no fields were redacted from this comparison",
            guard_reason_note: "Guarded because a dataset mismatch was detected, then reconciled",
            comparability_and_parity_note:
                "Permitted: parity evidence is complete, so this comparison is a fair baseline",
            context_note: "Proceed with the compare; the guard confirms parity",
            deep_link_kind: Link::RunObject,
            deep_link_ref: "run:notebook-1042/guard/dataset-permitted",
            banner_actions: vec![
                Action::OpenFullLineage,
                Action::ReviewComparability,
                Action::ViewChangedFactors,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::LocalRun, Disp::Reproducible],
        }),
        // 2. Environment drift, comparison caveated → partially comparable (partial note).
        guard_banner(GuardBannerSeed {
            banner_id: "cg-caveated-002",
            banner_label: "Comparison caveated: environment drift",
            guard_reason: Reason::EnvironmentDrift,
            guard_state: State::ComparisonCaveated,
            comparability_disclosure_note:
                "Partially comparable: the runs share data and code but the environment drifted",
            missing_lineage_fields_note: "Missing lineage fields: the candidate's exact package set is partial",
            changed_factors_note: "Changed factors: minor CUDA driver drift between the two runs",
            redaction_note: "Redaction: no fields were redacted from this comparison",
            guard_reason_note: "Guarded because the environment fingerprints drifted apart",
            comparability_and_parity_note:
                "Caveated: parity holds for code and data but not fully for environment",
            context_note: "Read the drift caveat before trusting a small delta",
            deep_link_kind: Link::NotebookLocation,
            deep_link_ref: "notebook:experiment.ipynb#guard-env-drift",
            banner_actions: vec![
                Action::OpenFullLineage,
                Action::ReviewComparability,
                Action::ViewChangedFactors,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::LocalRun, Disp::LikelyReproducible],
        }),
        // 3. Code revision gap, guard acknowledged → partially comparable (partial note).
        guard_banner(GuardBannerSeed {
            banner_id: "cg-acknowledged-003",
            banner_label: "Guard acknowledged: code revision gap",
            guard_reason: Reason::CodeRevisionGap,
            guard_state: State::GuardAcknowledged,
            comparability_disclosure_note:
                "Partially comparable: a small code revision gap was reviewed and acknowledged",
            missing_lineage_fields_note: "Missing lineage fields: none; both revisions are recorded",
            changed_factors_note: "Changed factors: three commits touch logging, not the model path",
            redaction_note: "Redaction: no fields were redacted from this comparison",
            guard_reason_note: "Guarded because the runs are a few commits apart",
            comparability_and_parity_note:
                "Acknowledged: the reviewer judged the code gap immaterial to the metric",
            context_note: "The acknowledgement is recorded; re-open lineage to re-check the gap",
            deep_link_kind: Link::RunObject,
            deep_link_ref: "run:managed-2207/guard/code-gap-acknowledged",
            banner_actions: vec![
                Action::OpenFullLineage,
                Action::ReviewComparability,
                Action::AcknowledgeGuard,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::ManagedRun, Disp::LikelyReproducible],
        }),
        // 4. Metric definition change, overridden by choice → overridden (override warning).
        guard_banner(GuardBannerSeed {
            banner_id: "cg-overridden-004",
            banner_label: "Guard overridden: metric definition changed",
            guard_reason: Reason::MetricDefinitionChange,
            guard_state: State::GuardOverriddenByChoice,
            comparability_disclosure_note:
                "Not certified comparable: the metric definition changed but the guard was overridden",
            missing_lineage_fields_note: "Missing lineage fields: the old metric's definition ref is not linked",
            changed_factors_note: "Changed factors: the F1 averaging changed from macro to micro",
            redaction_note: "Redaction: no fields were redacted from this comparison",
            guard_reason_note: "Guarded because the metric definition changed between the runs",
            comparability_and_parity_note:
                "Overridden: a user chose to compare across a metric-definition change; not a fair baseline",
            context_note: "Recompute both runs under one metric definition before trusting the delta",
            deep_link_kind: Link::DocsAnchor,
            deep_link_ref: "docs:notebooks/compare-guard-overridden",
            banner_actions: vec![
                Action::OpenFullLineage,
                Action::ReviewComparability,
                Action::ViewChangedFactors,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::LocalRun, Disp::NeedsRerun],
        }),
        // 5. Sample size imbalance, comparison blocked → blocked (blocked note).
        guard_banner(GuardBannerSeed {
            banner_id: "cg-blocked-005",
            banner_label: "Comparison blocked: sample size imbalance",
            guard_reason: Reason::SampleSizeImbalance,
            guard_state: State::ComparisonBlocked,
            comparability_disclosure_note:
                "Not comparable: the candidate evaluated 40 examples against the baseline's 2,000",
            missing_lineage_fields_note: "Missing lineage fields: the imported run's eval size was inferred",
            changed_factors_note: "Changed factors: a 50x sample-size imbalance between the runs",
            redaction_note: "Redaction: no fields were redacted from this comparison",
            guard_reason_note: "Guarded because the two runs evaluated wildly different sample sizes",
            comparability_and_parity_note:
                "Blocked: parity evidence is incomplete, so no fair delta is shown for this pair",
            context_note: "Evaluate the candidate on the full eval set before comparing",
            deep_link_kind: Link::DatasetCatalogAnchor,
            deep_link_ref: "dataset:catalog/compare-blocked-sample-size",
            banner_actions: vec![
                Action::OpenFullLineage,
                Action::ReviewComparability,
                Action::ViewChangedFactors,
                Action::OpenDeepLink,
            ],
            dispositions: vec![Disp::ImportedRun, Disp::NeedsRerun],
        }),
        // 6. Confounder present, guard unavailable → guard unavailable (unavailable note).
        guard_banner(GuardBannerSeed {
            banner_id: "cg-unavailable-006",
            banner_label: "Guard unavailable: confounder present",
            guard_reason: Reason::ConfounderPresent,
            guard_state: State::GuardUnavailable,
            comparability_disclosure_note:
                "Comparability unknown: a confounder is present and the guard cannot establish parity",
            missing_lineage_fields_note:
                "Missing lineage fields: the attached run has no recorded environment or data lineage",
            changed_factors_note: "Changed factors: an unrecorded confounder co-varies with the metric",
            redaction_note: "Redaction: no fields were redacted; there is nothing to redact",
            guard_reason_note: "Guarded because a confounder is present and lineage is missing",
            comparability_and_parity_note:
                "Unavailable: comparability cannot be established, so no fair delta is claimed",
            context_note: "Attach full lineage before the guard can assess this comparison",
            deep_link_kind: Link::NoDeepLink,
            deep_link_ref: "",
            banner_actions: vec![Action::OpenFullLineage, Action::ReviewComparability],
            dispositions: vec![Disp::ManualAttach, Disp::ContextIncomplete],
        }),
    ]
}

fn downgrade_triggers() -> Vec<M5ExperimentDowngradeTrigger> {
    vec![
        M5ExperimentDowngradeTrigger::ComparabilityOverstated,
        M5ExperimentDowngradeTrigger::CodeRevisionUnstated,
        M5ExperimentDowngradeTrigger::EnvironmentFingerprintUnstated,
        M5ExperimentDowngradeTrigger::CachedStateHidden,
        M5ExperimentDowngradeTrigger::AlternateStateLabelInvented,
        M5ExperimentDowngradeTrigger::ProofStale,
    ]
}

fn compare_review() -> CompareReview {
    CompareReview {
        comparison_table_shows_baseline_and_candidate: true,
        comparison_table_shows_delta_and_difference_factors: true,
        comparison_table_offers_open_runs_and_export: true,
        compare_guard_banner_shows_comparability_and_reason: true,
        compare_guard_banner_offers_full_lineage_and_review: true,
        fairness_and_comparability_derived_never_asserted: true,
        unfair_or_unproven_never_shown_as_fair: true,
        differences_shown_beside_delta: true,
        apples_to_apples_never_implied_without_parity: true,
        reproducibility_trust_labels_used_consistently: true,
        every_next_step_names_stable_deep_link: true,
        reuses_existing_comparability_vocabulary: true,
        provenance_and_sensitivity_state_visible: true,
        changed_factors_and_missing_lineage_visible: true,
        redaction_state_visible_before_share: true,
        no_surface_invents_alternate_state_label: true,
        components_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> CompareConsumerProjection {
    CompareConsumerProjection {
        comparison_ui_reads_single_source: true,
        compare_guard_surface_reads_single_source: true,
        baseline_and_candidate_visible_before_trust: true,
        difference_factors_visible_before_trust: true,
        support_export_shows_component_truth: true,
        help_docs_shows_component_truth: true,
    }
}

fn proof_freshness() -> CompareProofFreshness {
    CompareProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_SCHEMA_REF,
        RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_DOC_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_DOC_REF,
        M5_RUN_COMPARISON_TABLE_SCHEMA_REF,
        M5_COMPARE_GUARD_BANNER_SCHEMA_REF,
    ])
}

/// Builds the canonical run-comparison-table / compare-guard-banner controls packet.
pub fn seeded_run_comparison_table_compare_guard_banner_controls(
) -> RunComparisonTableCompareGuardBannerControlsPacket {
    RunComparisonTableCompareGuardBannerControlsPacket::new(
        RunComparisonTableCompareGuardBannerControlsPacketInput {
            packet_id: RUN_COMPARISON_TABLE_COMPARE_GUARD_BANNER_PACKET_ID.to_owned(),
            surface_label:
                "M5 run comparison tables and compare guard banners: baseline/candidate identity, confounder disclosure, and no-fair-delta claims when parity evidence is incomplete across claimed compare flows"
                    .to_owned(),
            comparison_tables: comparison_tables(),
            guard_banners: guard_banners(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
            compare_review: compare_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a not-comparable run comparison table that must never read as a
/// fair baseline. Every fairness class, comparison axis, and comparability state stays covered so
/// the fixture validates on its own.
pub fn seeded_run_comparison_table_compare_guard_banner_controls_comparison_table_not_comparable(
) -> RunComparisonTableCompareGuardBannerControlsPacket {
    let mut packet = seeded_run_comparison_table_compare_guard_banner_controls();
    packet.packet_id =
        "m5-run-comparison-table-compare-guard-banner-controls:fixture:comparison-table-not-comparable"
            .to_owned();
    packet.surface_label =
        "M5 run comparison tables: a not-comparable comparison never reads as a fair baseline"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a blocked compare guard banner that must never permit a fair
/// comparison when parity evidence is incomplete. Every guard comparability class, guard reason,
/// and guard state stays covered so the fixture validates on its own.
pub fn seeded_run_comparison_table_compare_guard_banner_controls_compare_guard_banner_blocked(
) -> RunComparisonTableCompareGuardBannerControlsPacket {
    let mut packet = seeded_run_comparison_table_compare_guard_banner_controls();
    packet.packet_id =
        "m5-run-comparison-table-compare-guard-banner-controls:fixture:compare-guard-banner-blocked"
            .to_owned();
    packet.surface_label =
        "M5 compare guard banners: a blocked guard never permits a fair comparison".to_owned();
    packet
}
