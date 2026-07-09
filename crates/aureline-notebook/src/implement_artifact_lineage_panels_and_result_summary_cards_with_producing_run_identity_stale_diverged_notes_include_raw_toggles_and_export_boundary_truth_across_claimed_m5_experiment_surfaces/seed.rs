//! Canonical seed builders for the artifact-lineage-panel / result-summary-card controls.
//!
//! These builders are the single producer of the checked-in support export and the scenario
//! fixtures. The headless emitter and the inline tests both call them so the in-code
//! components, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical artifact-lineage-panel / result-summary-card packet.
pub const ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_PACKET_ID: &str =
    "m5-artifact-lineage-panel-result-summary-card-controls:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-09T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn lineage_panel_source_refs() -> Vec<String> {
    strings(&[
        M5_ARTIFACT_LINEAGE_PANEL_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
    ])
}

fn summary_card_source_refs() -> Vec<String> {
    strings(&[
        M5_RESULT_SUMMARY_CARD_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
    ])
}

fn lineage_panel_downgrade_triggers() -> Vec<M5ExperimentDowngradeTrigger> {
    vec![
        M5ExperimentDowngradeTrigger::RunOriginUnstated,
        M5ExperimentDowngradeTrigger::DatasetProvenanceSevered,
        M5ExperimentDowngradeTrigger::CachedStateHidden,
        M5ExperimentDowngradeTrigger::AlternateStateLabelInvented,
        M5ExperimentDowngradeTrigger::ProofStale,
    ]
}

fn summary_card_downgrade_triggers() -> Vec<M5ExperimentDowngradeTrigger> {
    vec![
        M5ExperimentDowngradeTrigger::ExportScopeUnstated,
        M5ExperimentDowngradeTrigger::RawPayloadExposedByDefault,
        M5ExperimentDowngradeTrigger::CachedStateHidden,
        M5ExperimentDowngradeTrigger::AlternateStateLabelInvented,
        M5ExperimentDowngradeTrigger::ProofStale,
    ]
}

/// Builds an artifact lineage panel, deriving the traceability class, the fully-traced claim,
/// and the required notes from the honest inputs so the seed is always self-consistent with the
/// resolver.
#[allow(clippy::too_many_arguments)]
fn lineage_panel(
    panel_id: &str,
    artifact_label: &str,
    artifact_kind: M5ArtifactKindClass,
    lineage_state: M5LineageState,
    producing_run_id: &str,
    producing_run_label: &str,
    generator_step_note: &str,
    environment_fingerprint_ref: &str,
    saved_scope_note: &str,
    lineage_and_run_note: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    panel_actions: Vec<ArtifactLineageAction>,
    dispositions: Vec<M5ExperimentDisposition>,
) -> ArtifactLineagePanel {
    let disclosure = resolve_artifact_lineage(lineage_state);
    ArtifactLineagePanel {
        component: M5ExperimentComponentFamily::ArtifactLineagePanel,
        panel_id: panel_id.to_owned(),
        artifact_label: artifact_label.to_owned(),
        artifact_kind,
        lineage_state,
        traceability_class: disclosure.traceability_class,
        claims_fully_traced: disclosure.is_fully_traced,
        producing_run_id: producing_run_id.to_owned(),
        producing_run_label: producing_run_label.to_owned(),
        generator_step_note: generator_step_note.to_owned(),
        environment_fingerprint_ref: environment_fingerprint_ref.to_owned(),
        saved_scope_note: saved_scope_note.to_owned(),
        partial_lineage_note: if disclosure.needs_partial_lineage_note {
            "Only part of this artifact's lineage was captured; treat upstream as incomplete"
                .to_owned()
        } else {
            String::new()
        },
        stale_or_diverged_note: if disclosure.needs_stale_or_diverged_note {
            "Lineage is broken; this artifact has diverged from its recorded run and is stale"
                .to_owned()
        } else {
            String::new()
        },
        unknown_upstream_note: if disclosure.needs_unknown_upstream_note {
            "Derived from an unknown upstream; do not assume the producing recipe is reproducible"
                .to_owned()
        } else {
            String::new()
        },
        regenerated_note: if disclosure.needs_regenerated_note {
            "Regenerated from a known recipe; this is not the original artifact instance".to_owned()
        } else {
            String::new()
        },
        lineage_and_run_note: lineage_and_run_note.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        panel_actions,
        dispositions,
        downgrade_triggers: lineage_panel_downgrade_triggers(),
        required_labels: M5ExperimentRequiredLabel::ALL.to_vec(),
        surface_families: M5ExperimentSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ExperimentDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ExperimentAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "artifact_label",
            "artifact_kind",
            "lineage_state",
            "traceability_class",
            "producing_run_id",
            "generator_step_note",
            "environment_fingerprint_ref",
            "saved_scope_note",
            "deep_link_kind",
        ]),
        source_contract_refs: lineage_panel_source_refs(),
        masks_provenance_or_sensitivity_state: false,
        hides_producing_run_or_lineage_state: false,
        exposes_raw_payload_by_default: false,
        implies_apples_to_apples_without_parity: false,
        invents_alternate_state_label: false,
    }
}

/// Builds a result summary card, deriving the export disposition, the raw-payload /
/// metadata-only claims, the include-raw toggle, and the required notes from the honest inputs
/// so the seed is always self-consistent with the resolver.
#[allow(clippy::too_many_arguments)]
fn summary_card(
    card_id: &str,
    card_label: &str,
    summary_content_class: M5SummaryContentClass,
    export_scope_state: M5SummaryExportScope,
    headline_metric_note: &str,
    artifact_count_label: &str,
    freshness_note: &str,
    support_report_scope_note: &str,
    provenance_note: &str,
    summary_evidence_raw_handoff_note: &str,
    context_note: &str,
    deep_link_kind: DeepLinkKind,
    deep_link_ref: &str,
    card_actions: Vec<SummaryCardAction>,
    dispositions: Vec<M5ExperimentDisposition>,
) -> ResultSummaryCard {
    let disclosure = resolve_summary_export(export_scope_state);
    ResultSummaryCard {
        component: M5ExperimentComponentFamily::ResultSummaryCard,
        card_id: card_id.to_owned(),
        card_label: card_label.to_owned(),
        summary_content_class,
        export_scope_state,
        export_disposition: disclosure.export_disposition,
        claims_includes_raw_payload: disclosure.includes_raw_payload,
        claims_metadata_only: disclosure.is_metadata_only,
        include_raw_toggle_on: disclosure.includes_raw_payload,
        headline_metric_note: headline_metric_note.to_owned(),
        artifact_count_label: artifact_count_label.to_owned(),
        freshness_note: freshness_note.to_owned(),
        support_report_scope_note: support_report_scope_note.to_owned(),
        provenance_note: provenance_note.to_owned(),
        raw_inclusion_warning: if disclosure.needs_raw_inclusion_warning {
            "This export includes a raw payload only because include-raw was explicitly turned on"
                .to_owned()
        } else {
            String::new()
        },
        withheld_note: if disclosure.needs_withheld_note {
            "Export is withheld; only the on-screen summary is available, nothing leaves".to_owned()
        } else {
            String::new()
        },
        redaction_note: if disclosure.needs_redaction_note {
            "Export is redacted; redacted fields are removed before the summary leaves".to_owned()
        } else {
            String::new()
        },
        summary_evidence_raw_handoff_note: summary_evidence_raw_handoff_note.to_owned(),
        context_note: context_note.to_owned(),
        deep_link_kind,
        deep_link_ref: deep_link_ref.to_owned(),
        card_actions,
        dispositions,
        downgrade_triggers: summary_card_downgrade_triggers(),
        required_labels: M5ExperimentRequiredLabel::ALL.to_vec(),
        surface_families: M5ExperimentSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5ExperimentDeploymentLine::ALL.to_vec(),
        accessibility_routes: M5ExperimentAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
        fields_shown: strings(&[
            "card_label",
            "summary_content_class",
            "export_scope_state",
            "export_disposition",
            "headline_metric_note",
            "artifact_count_label",
            "freshness_note",
            "include_raw_toggle_on",
            "deep_link_kind",
        ]),
        source_contract_refs: summary_card_source_refs(),
        masks_provenance_or_sensitivity_state: false,
        hides_producing_run_or_lineage_state: false,
        exposes_raw_payload_by_default: false,
        implies_apples_to_apples_without_parity: false,
        invents_alternate_state_label: false,
    }
}

fn lineage_panels() -> Vec<ArtifactLineagePanel> {
    use ArtifactLineageAction as Action;
    use DeepLinkKind as Link;
    use M5ArtifactKindClass as Kind;
    use M5ExperimentDisposition as Disp;
    use M5LineageState as State;

    vec![
        // 1. Model checkpoint, lineage complete → fully traced.
        lineage_panel(
            "al-model-001",
            "Ranker checkpoint v7",
            Kind::ModelCheckpoint,
            State::LineageComplete,
            "run-notebook-1042",
            "Training run (notebook cell 12)",
            "Produced by the train step in cell 12 after the fit loop",
            "fingerprint:env-cuda-12-torch-2-3",
            "Saved to the local run artifact store",
            "Checkpoint traces to run-notebook-1042 with complete lineage; open or compare directly",
            "Confirm the run and environment fingerprint before promoting this checkpoint",
            Link::RunObject,
            "run:notebook-1042/artifacts/ranker-v7",
            vec![
                Action::OpenArtifact,
                Action::TraceToRun,
                Action::ExportLineage,
                Action::OpenDeepLink,
                Action::CompareLineage,
            ],
            vec![Disp::LocalRun, Disp::Reproducible],
        ),
        // 2. Metrics table, regenerated → regenerated (fully traced, regenerated note).
        lineage_panel(
            "al-metrics-002",
            "Eval metrics table",
            Kind::MetricsTable,
            State::Regenerated,
            "run-notebook-1042",
            "Metrics re-run (notebook cell 18)",
            "Regenerated by re-running the evaluation cell against the same recipe",
            "fingerprint:env-cuda-12-torch-2-3",
            "Saved to the local run artifact store",
            "Metrics table was regenerated from run-notebook-1042's recipe; trace to run to compare",
            "Check that the regenerated metrics match the original run before trusting a compare",
            Link::RunObject,
            "run:notebook-1042/artifacts/eval-metrics",
            vec![
                Action::OpenArtifact,
                Action::TraceToRun,
                Action::ExportLineage,
                Action::OpenDeepLink,
            ],
            vec![Disp::LocalRun, Disp::Reproducible],
        ),
        // 3. Plot figure, lineage partial → partially traced (partial note).
        lineage_panel(
            "al-plot-003",
            "Loss curve figure",
            Kind::PlotFigure,
            State::LineagePartial,
            "run-notebook-1042",
            "Plotting run (notebook cell 20)",
            "Rendered by the plotting cell from a metrics frame whose full inputs are not captured",
            "fingerprint:env-cuda-12-torch-2-3",
            "Saved alongside the notebook output",
            "Figure traces to run-notebook-1042 but its upstream metrics lineage is only partial",
            "Verify the missing upstream inputs before citing this figure as evidence",
            Link::NotebookLocation,
            "notebook:experiment.ipynb#cell-20",
            vec![
                Action::OpenArtifact,
                Action::TraceToRun,
                Action::ExportLineage,
                Action::OpenDeepLink,
            ],
            vec![Disp::LocalRun, Disp::LikelyReproducible],
        ),
        // 4. Exported report, derived upstream known → partially traced.
        lineage_panel(
            "al-report-004",
            "Weekly results report",
            Kind::ExportedReport,
            State::DerivedUpstreamKnown,
            "run-managed-2207",
            "Report export (managed run)",
            "Assembled by the report exporter from a known upstream managed run",
            "fingerprint:env-managed-runner-2207",
            "Exported to the local reports folder",
            "Report is derived from the known upstream run-managed-2207; open it to inspect lineage",
            "Inspect the upstream managed run before circulating this report",
            Link::RunObject,
            "run:managed-2207/artifacts/weekly-report",
            vec![
                Action::OpenArtifact,
                Action::TraceToRun,
                Action::ExportLineage,
                Action::OpenDeepLink,
                Action::CompareLineage,
            ],
            vec![Disp::ManagedRun, Disp::LikelyReproducible],
        ),
        // 5. Log bundle, lineage broken → untraced (stale / diverged note).
        lineage_panel(
            "al-log-005",
            "Training log bundle",
            Kind::LogBundle,
            State::LineageBroken,
            "run-imported-0031",
            "Imported run (external tracker)",
            "Log bundle was imported; the link back to its producing step is broken",
            "fingerprint:unavailable-imported",
            "Attached to the imported run record",
            "Log bundle's lineage to run-imported-0031 is broken; treat it as stale, not verified",
            "Re-run or re-import to restore lineage before relying on this log bundle",
            Link::DocsAnchor,
            "docs:notebooks/artifact-lineage-broken",
            vec![
                Action::OpenArtifact,
                Action::TraceToRun,
                Action::ExportLineage,
                Action::OpenDeepLink,
            ],
            vec![Disp::ImportedRun, Disp::NeedsRerun],
        ),
        // 6. Unknown artifact, derived upstream unknown → untraced (unknown-upstream note).
        lineage_panel(
            "al-unknown-006",
            "Unlabeled attachment",
            Kind::UnknownArtifact,
            State::DerivedUpstreamUnknown,
            "run-manual-attach-0009",
            "Manually attached run",
            "Attached manually; the producing step and upstream are not recorded",
            "fingerprint:unavailable-manual-attach",
            "Attached to the manual-attach run record",
            "Attachment routes to run-manual-attach-0009 but its upstream is unknown and untraced",
            "Classify the upstream before treating this attachment as verified experiment output",
            Link::NoDeepLink,
            "",
            vec![
                Action::OpenArtifact,
                Action::TraceToRun,
                Action::ExportLineage,
            ],
            vec![Disp::ManualAttach, Disp::ContextIncomplete],
        ),
    ]
}

fn summary_cards() -> Vec<ResultSummaryCard> {
    use DeepLinkKind as Link;
    use M5ExperimentDisposition as Disp;
    use M5SummaryContentClass as Content;
    use M5SummaryExportScope as Scope;
    use SummaryCardAction as Action;

    vec![
        // 1. Headline metric, summary scope → metadata-safe (include-raw off).
        summary_card(
            "rs-headline-001",
            "Ranker headline result",
            Content::HeadlineMetric,
            Scope::SummaryScope,
            "Headline: NDCG@10 improved to 0.412 (+0.9%)",
            "Backed by 3 artifacts (checkpoint, metrics, figure)",
            "Fresh: reflects the latest run-notebook-1042",
            "Report scope: summary line only",
            "Produced by run-notebook-1042 with complete lineage",
            "Summary only by default; switch to evidence or raw only by explicit choice",
            "Summary-only headline; no evidence or raw payload leaves this card",
            Link::RunObject,
            "run:notebook-1042/summary/headline",
            vec![
                Action::ReviewExportScope,
                Action::ShareSummaryOnly,
                Action::ExportEvidence,
                Action::OpenDeepLink,
            ],
            vec![Disp::LocalRun, Disp::Reproducible],
        ),
        // 2. Metric table, metadata scope → metadata-safe (include-raw off).
        summary_card(
            "rs-metrics-002",
            "Eval metric table summary",
            Content::MetricTable,
            Scope::MetadataScope,
            "Metrics: NDCG@10 0.412, MRR 0.331, coverage 0.98",
            "Backed by 1 artifact (metrics table)",
            "Fresh: regenerated with the latest run",
            "Report scope: metric table plus metadata",
            "Produced by run-notebook-1042; metrics table regenerated from the recipe",
            "Metadata-scope table by default; include raw only by explicit choice",
            "Metric table and metadata only; the underlying rows stay local",
            Link::RunObject,
            "run:notebook-1042/summary/metrics",
            vec![
                Action::ReviewExportScope,
                Action::ShareSummaryOnly,
                Action::ExportEvidence,
                Action::OpenDeepLink,
            ],
            vec![Disp::LocalRun, Disp::Reproducible],
        ),
        // 3. Narrative summary, evidence scope → evidence-scoped (include-raw off).
        summary_card(
            "rs-narrative-003",
            "Experiment narrative",
            Content::NarrativeSummary,
            Scope::EvidenceScope,
            "Narrative: the ranker change improved NDCG without hurting latency",
            "Backed by 3 artifacts with evidence links",
            "Fresh: reflects the latest run-notebook-1042",
            "Report scope: narrative plus evidence links",
            "Produced by run-notebook-1042; evidence links resolve to run artifacts",
            "Evidence links are included; switch to summary-only to omit them, raw only on request",
            "Narrative with evidence links; no raw payload is attached to this share",
            Link::NotebookLocation,
            "notebook:experiment.ipynb#summary",
            vec![
                Action::ReviewExportScope,
                Action::ShareSummaryOnly,
                Action::ExportEvidence,
                Action::OpenDeepLink,
            ],
            vec![Disp::LocalRun, Disp::LikelyReproducible],
        ),
        // 4. Evidence link, redacted scope → redacted (include-raw off, redaction note).
        summary_card(
            "rs-evidence-004",
            "Redacted evidence bundle",
            Content::EvidenceLink,
            Scope::RedactedScope,
            "Headline held out; evidence links point to redacted artifacts",
            "Backed by 2 artifacts (redacted)",
            "Fresh: redacted from the latest run",
            "Report scope: redacted evidence links only",
            "Produced by run-managed-2207; sensitive fields redacted before export",
            "Redacted evidence by default; raw payload never leaves this card",
            "Evidence links are redacted before they leave; raw fields never cross the boundary",
            Link::DocsAnchor,
            "docs:notebooks/result-summary-redacted",
            vec![
                Action::ReviewExportScope,
                Action::ShareSummaryOnly,
                Action::ExportEvidence,
                Action::OpenDeepLink,
            ],
            vec![Disp::ManagedRun, Disp::LikelyReproducible],
        ),
        // 5. Raw payload ref, raw scope → raw-included (include-raw ON, warned).
        summary_card(
            "rs-raw-005",
            "Raw payload export",
            Content::RawPayloadRef,
            Scope::RawScope,
            "Headline plus a reference to the raw prediction payload",
            "Backed by 3 artifacts including a raw payload",
            "Fresh: reflects the latest run-notebook-1042",
            "Report scope: summary plus raw payload reference",
            "Produced by run-notebook-1042; raw payload lives in the local run store",
            "Raw payload is included only because include-raw was explicitly turned on for this export",
            "Raw payload is included by explicit toggle, never by default; turn it off to keep it local",
            Link::DocsAnchor,
            "docs:notebooks/result-summary-include-raw",
            vec![
                Action::ReviewExportScope,
                Action::ShareSummaryOnly,
                Action::IncludeRawPayload,
                Action::OpenDeepLink,
            ],
            vec![Disp::LocalRun, Disp::ContextIncomplete],
        ),
        // 6. No result, export withheld → withheld (include-raw off, withheld note).
        summary_card(
            "rs-noresult-006",
            "No-result summary",
            Content::NoResult,
            Scope::ExportWithheld,
            "No headline result: the run did not produce a comparable metric",
            "Backed by 0 result artifacts",
            "Stale: no fresh result is available",
            "Report scope: export withheld until a result exists",
            "Attached to run-manual-attach-0009; no producing result recorded",
            "Export is withheld; there is no summary, evidence, or raw payload to hand off yet",
            "Nothing is exported while there is no result; only the on-screen note is shown",
            Link::NoDeepLink,
            "",
            vec![Action::ReviewExportScope, Action::ShareSummaryOnly],
            vec![Disp::ManualAttach, Disp::ContextIncomplete],
        ),
    ]
}

fn downgrade_triggers() -> Vec<M5ExperimentDowngradeTrigger> {
    vec![
        M5ExperimentDowngradeTrigger::RunOriginUnstated,
        M5ExperimentDowngradeTrigger::DatasetProvenanceSevered,
        M5ExperimentDowngradeTrigger::ExportScopeUnstated,
        M5ExperimentDowngradeTrigger::RawPayloadExposedByDefault,
        M5ExperimentDowngradeTrigger::CachedStateHidden,
        M5ExperimentDowngradeTrigger::AlternateStateLabelInvented,
        M5ExperimentDowngradeTrigger::ProofStale,
    ]
}

fn artifact_review() -> ArtifactSummaryReview {
    ArtifactSummaryReview {
        lineage_panel_shows_artifact_and_producing_run: true,
        lineage_panel_shows_lineage_state_and_generator_step: true,
        lineage_panel_offers_open_trace_export: true,
        summary_card_shows_headline_and_export_scope: true,
        summary_card_offers_review_and_summary_only: true,
        traceability_and_export_derived_never_asserted: true,
        untraced_or_broken_never_shown_as_traced: true,
        every_artifact_routes_to_producing_run: true,
        raw_payload_never_included_by_default: true,
        include_raw_is_explicit_choice_never_default: true,
        summary_evidence_raw_scope_visible_before_share: true,
        every_next_step_names_stable_deep_link: true,
        reuses_existing_provenance_export_vocabulary: true,
        provenance_and_sensitivity_state_visible: true,
        cached_offline_local_only_state_visible: true,
        no_surface_invents_alternate_state_label: true,
        components_stable_across_deployment_lines: true,
        copy_and_export_safe: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> ArtifactSummaryConsumerProjection {
    ArtifactSummaryConsumerProjection {
        lineage_ui_reads_single_source: true,
        result_summary_surface_reads_single_source: true,
        producing_run_and_lineage_visible_before_trust: true,
        export_scope_visible_before_share: true,
        support_export_shows_component_truth: true,
        help_docs_shows_component_truth: true,
    }
}

fn proof_freshness() -> ArtifactSummaryProofFreshness {
    ArtifactSummaryProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_SCHEMA_REF,
        ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_DOC_REF,
        M5_EXPERIMENT_COMPONENT_SCHEMA_REF,
        M5_EXPERIMENT_COMPONENT_DOC_REF,
        M5_ARTIFACT_LINEAGE_PANEL_SCHEMA_REF,
        M5_RESULT_SUMMARY_CARD_SCHEMA_REF,
    ])
}

/// Builds the canonical artifact-lineage-panel / result-summary-card controls packet.
pub fn seeded_artifact_lineage_panel_result_summary_card_controls(
) -> ArtifactLineagePanelResultSummaryCardControlsPacket {
    ArtifactLineagePanelResultSummaryCardControlsPacket::new(
        ArtifactLineagePanelResultSummaryCardControlsPacketInput {
            packet_id: ARTIFACT_LINEAGE_PANEL_RESULT_SUMMARY_CARD_PACKET_ID.to_owned(),
            surface_label:
                "M5 artifact lineage panels and result summary cards: producing-run identity, stale/diverged notes, include-raw toggles, and export-boundary truth across claimed experiment surfaces"
                    .to_owned(),
            lineage_panels: lineage_panels(),
            summary_cards: summary_cards(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: M5ExperimentConsumerSurface::ALL.to_vec(),
            artifact_review: artifact_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Scenario fixture: spotlights a broken-lineage artifact panel that must never read as a
/// fully-traced artifact. Every traceability class, artifact kind, and lineage state stays
/// covered so the fixture validates on its own.
pub fn seeded_artifact_lineage_panel_result_summary_card_controls_lineage_panel_broken(
) -> ArtifactLineagePanelResultSummaryCardControlsPacket {
    let mut packet = seeded_artifact_lineage_panel_result_summary_card_controls();
    packet.packet_id =
        "m5-artifact-lineage-panel-result-summary-card-controls:fixture:lineage-panel-broken"
            .to_owned();
    packet.surface_label =
        "M5 artifact lineage panels: a broken-lineage artifact never reads as fully traced"
            .to_owned();
    packet
}

/// Scenario fixture: spotlights a raw-payload result summary card that must flag its raw scope
/// and never read as a metadata-only default. Every export disposition, summary content class,
/// and export scope stays covered so the fixture validates on its own.
pub fn seeded_artifact_lineage_panel_result_summary_card_controls_summary_card_raw_payload(
) -> ArtifactLineagePanelResultSummaryCardControlsPacket {
    let mut packet = seeded_artifact_lineage_panel_result_summary_card_controls();
    packet.packet_id =
        "m5-artifact-lineage-panel-result-summary-card-controls:fixture:summary-card-raw-payload"
            .to_owned();
    packet.surface_label =
        "M5 result summary cards: a raw-payload export is never the metadata-only default"
            .to_owned();
    packet
}
