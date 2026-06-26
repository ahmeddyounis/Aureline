//! Canonical seed builders for the M5 non-visual summary catalog.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code summaries, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical non-visual summary catalog.
pub const M5_NONVISUAL_SUMMARY_CATALOG_PACKET_ID: &str = "m5-nonvisual-summary:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

/// Proof packet ref every governed summary carries.
const SUMMARY_PROOF_REF: &str = "evidence:nonvisual-summary-conformance:m5";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn fallback(surface: M5DurableFallbackSurface, surface_ref: &str) -> M5DurableFallbackRef {
    M5DurableFallbackRef {
        surface,
        surface_ref: surface_ref.to_owned(),
        reopenable: true,
    }
}

fn dimension(name: &str, dimension_message_id: &str) -> M5SummaryDimension {
    M5SummaryDimension {
        name: name.to_owned(),
        dimension_message_id: dimension_message_id.to_owned(),
    }
}

fn structure(
    structure_message_id: &str,
    role_class: A11ySemanticRoleClass,
    dimensions: Vec<M5SummaryDimension>,
) -> M5SummaryStructure {
    M5SummaryStructure {
        structure_message_id: structure_message_id.to_owned(),
        role_class,
        dimensions,
    }
}

fn drilldown(
    drilldown_id: &str,
    kind: M5SummaryDrillDownKind,
    label: &str,
    route_message_id: &str,
    target_identity_ref: &str,
) -> M5SummaryDrillDown {
    M5SummaryDrillDown {
        drilldown_id: drilldown_id.to_owned(),
        kind,
        label: label.to_owned(),
        route_message_id: route_message_id.to_owned(),
        target_identity_ref: target_identity_ref.to_owned(),
        keyboard_reachable: true,
    }
}

/// A text alternative for a surface whose visual state materially affects decisions.
fn text_alternative(
    kind: M5SummaryTextAlternativeKind,
    alt_text_message_id: &str,
    export_metadata_fields: &[&str],
) -> M5SummaryTextAlternative {
    M5SummaryTextAlternative {
        kind,
        provided: true,
        alt_text_message_id: alt_text_message_id.to_owned(),
        export_metadata_fields: strings(export_metadata_fields),
    }
}

/// The "no alternative needed" declaration for a text-native surface.
fn no_text_alternative() -> M5SummaryTextAlternative {
    M5SummaryTextAlternative {
        kind: M5SummaryTextAlternativeKind::NotApplicable,
        provided: false,
        alt_text_message_id: String::new(),
        export_metadata_fields: Vec::new(),
    }
}

/// The downgrade triggers every governed summary carries: both bridge degradation
/// paths, stale proof, lost non-visual fidelity, and pointer/hover dependence.
fn standard_downgrade_triggers() -> Vec<M5DynamicSurfaceA11yDowngradeTrigger> {
    use M5DynamicSurfaceA11yDowngradeTrigger as D;
    vec![
        D::ProofStale,
        D::BridgeUnavailable,
        D::BridgePartialOrStale,
        D::NonVisualFidelityLost,
        D::PointerOrHoverDependence,
    ]
}

#[allow(clippy::too_many_arguments)]
fn summary(
    summary_id: &str,
    surface_kind: M5SummarySurfaceKind,
    label: &str,
    object_identity_ref: &str,
    presentation_state: M5SummaryPresentationState,
    producers: Vec<M5SummaryProducer>,
    structure: M5SummaryStructure,
    drilldowns: Vec<M5SummaryDrillDown>,
    text_alternative: M5SummaryTextAlternative,
    durable_fallback: M5DurableFallbackRef,
    consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
) -> M5SurfaceSummary {
    M5SurfaceSummary {
        summary_id: summary_id.to_owned(),
        surface_kind,
        label: label.to_owned(),
        owner_role: "Accessibility owner".to_owned(),
        object_identity_ref: object_identity_ref.to_owned(),
        qualification: M5DynamicSurfaceA11yQualificationClass::Stable,
        non_visual_fidelity: A11yNonVisualFidelity::FullAccessible,
        presentation_state,
        producers,
        structure,
        drilldowns,
        text_alternative,
        durable_fallback,
        downgrade_triggers: standard_downgrade_triggers(),
        required_proof_packet_refs: strings(&[SUMMARY_PROOF_REF]),
        source_contract_refs: strings(&[
            M5_NONVISUAL_SUMMARY_SURFACE_DESCRIPTOR_REF,
            M5_NONVISUAL_SUMMARY_SCREEN_READER_CONTRACT_REF,
        ]),
        consumer_surfaces,
    }
}

fn summaries() -> Vec<M5SurfaceSummary> {
    use M5DurableFallbackSurface as Surface;
    use M5DynamicSurfaceA11yConsumerSurface as Consumer;
    use M5SummaryDrillDownKind as Kind;
    use M5SummaryPresentationState as State;
    use M5SummaryProducer as Producer;
    use M5SummaryTextAlternativeKind as Alt;

    vec![
        // Custom-rendered editor: authoritative buffer; structure is lines and regions.
        summary(
            "summary:custom-editor",
            M5SummarySurfaceKind::CustomEditor,
            "Custom-rendered editor",
            "editor:active-buffer",
            State::Authoritative,
            vec![Producer::Editor],
            structure(
                "summary.editor.structure",
                A11ySemanticRoleClass::TextDocument,
                vec![
                    dimension("lines", "summary.editor.dim.lines"),
                    dimension("regions", "summary.editor.dim.regions"),
                ],
            ),
            vec![
                drilldown(
                    "drilldown:editor.enumerate-structure",
                    Kind::EnumerateStructure,
                    "Enumerate editor regions",
                    "summary.editor.enumerate_structure",
                    "editor:regions",
                ),
                drilldown(
                    "drilldown:editor.open-line-detail",
                    Kind::OpenItemDetail,
                    "Open focused line detail",
                    "summary.editor.open_line_detail",
                    "editor:cursor-line",
                ),
                drilldown(
                    "drilldown:editor.jump-diagnostic",
                    Kind::JumpToRegion,
                    "Jump to next diagnostic region",
                    "summary.editor.jump_diagnostic",
                    "editor:diagnostic-region",
                ),
            ],
            no_text_alternative(),
            fallback(Surface::StatusDetail, "status-detail:editor-summary"),
            vec![Consumer::Editor, Consumer::SupportExport],
        ),
        // Terminal canvas: buffered scrollback; structure is command blocks and lines.
        summary(
            "summary:terminal-canvas",
            M5SummarySurfaceKind::TerminalCanvas,
            "Terminal canvas",
            "terminal:active-session",
            State::Buffered,
            vec![Producer::Terminal],
            structure(
                "summary.terminal.structure",
                A11ySemanticRoleClass::LiveLogRegion,
                vec![
                    dimension("command_blocks", "summary.terminal.dim.command_blocks"),
                    dimension("scrollback_lines", "summary.terminal.dim.scrollback_lines"),
                ],
            ),
            vec![
                drilldown(
                    "drilldown:terminal.enumerate-blocks",
                    Kind::EnumerateStructure,
                    "Enumerate command blocks",
                    "summary.terminal.enumerate_blocks",
                    "terminal:command-blocks",
                ),
                drilldown(
                    "drilldown:terminal.jump-last-output",
                    Kind::JumpToRegion,
                    "Jump to last command output",
                    "summary.terminal.jump_last_output",
                    "terminal:last-output",
                ),
                drilldown(
                    "drilldown:terminal.open-exit-detail",
                    Kind::OpenItemDetail,
                    "Open last command exit detail",
                    "summary.terminal.open_exit_detail",
                    "terminal:exit-detail",
                ),
            ],
            no_text_alternative(),
            fallback(Surface::ActivityRow, "activity-row:terminal-summary"),
            vec![Consumer::Terminal, Consumer::SupportExport],
        ),
        // Dense data grid: cached result set; structure is columns and rows.
        summary(
            "summary:data-grid",
            M5SummarySurfaceKind::DataGrid,
            "Dense data grid",
            "data-grid:active-view",
            State::Cached,
            vec![Producer::Data],
            structure(
                "summary.data_grid.structure",
                A11ySemanticRoleClass::DataGridCell,
                vec![
                    dimension("columns", "summary.data_grid.dim.columns"),
                    dimension("rows", "summary.data_grid.dim.rows"),
                ],
            ),
            vec![
                drilldown(
                    "drilldown:data-grid.enumerate-structure",
                    Kind::EnumerateStructure,
                    "Enumerate columns and row count",
                    "summary.data_grid.enumerate_structure",
                    "data-grid:structure",
                ),
                drilldown(
                    "drilldown:data-grid.open-cell-detail",
                    Kind::OpenItemDetail,
                    "Open focused cell detail",
                    "summary.data_grid.open_cell_detail",
                    "data-grid:cell",
                ),
                drilldown(
                    "drilldown:data-grid.describe-column",
                    Kind::DescribeSeries,
                    "Describe focused column",
                    "summary.data_grid.describe_column",
                    "data-grid:column",
                ),
            ],
            no_text_alternative(),
            fallback(Surface::SelectionSummary, "selection-summary:data-grid"),
            vec![Consumer::DataGrid, Consumer::SupportExport],
        ),
        // Tree / outline: cached lazily-expanded nodes; structure is nodes and depth.
        summary(
            "summary:tree-outline",
            M5SummarySurfaceKind::TreeOutline,
            "Tree outline",
            "tree:active-outline",
            State::Cached,
            vec![Producer::Data],
            structure(
                "summary.tree.structure",
                A11ySemanticRoleClass::StructureGroup,
                vec![
                    dimension("top_level_nodes", "summary.tree.dim.top_level_nodes"),
                    dimension("max_depth", "summary.tree.dim.max_depth"),
                ],
            ),
            vec![
                drilldown(
                    "drilldown:tree.enumerate-structure",
                    Kind::EnumerateStructure,
                    "Enumerate top-level nodes and depth",
                    "summary.tree.enumerate_structure",
                    "tree:structure",
                ),
                drilldown(
                    "drilldown:tree.open-node-detail",
                    Kind::OpenItemDetail,
                    "Open focused node detail",
                    "summary.tree.open_node_detail",
                    "tree:node",
                ),
            ],
            no_text_alternative(),
            fallback(Surface::SelectionSummary, "selection-summary:tree"),
            vec![Consumer::DataGrid, Consumer::SupportExport],
        ),
        // Streaming log: buffered tail; structure is entries and error count.
        summary(
            "summary:log-stream",
            M5SummarySurfaceKind::LogStream,
            "Streaming log",
            "log:active-stream",
            State::Buffered,
            vec![Producer::Observability],
            structure(
                "summary.log.structure",
                A11ySemanticRoleClass::LiveLogRegion,
                vec![
                    dimension("entries", "summary.log.dim.entries"),
                    dimension("error_count", "summary.log.dim.error_count"),
                ],
            ),
            vec![
                drilldown(
                    "drilldown:log.enumerate-severity",
                    Kind::EnumerateStructure,
                    "Enumerate log severity counts",
                    "summary.log.enumerate_severity",
                    "log:severity-summary",
                ),
                drilldown(
                    "drilldown:log.jump-error-block",
                    Kind::JumpToRegion,
                    "Jump to next error block",
                    "summary.log.jump_error_block",
                    "log:error-block",
                ),
            ],
            no_text_alternative(),
            fallback(Surface::ActivityRow, "activity-row:log-summary"),
            vec![Consumer::Terminal, Consumer::SupportExport],
        ),
        // Trace timeline: sampled spans; visual decision surface -> text alternative.
        summary(
            "summary:trace-timeline",
            M5SummarySurfaceKind::TraceTimeline,
            "Trace timeline",
            "trace:active-timeline",
            State::Sampled,
            vec![Producer::Observability],
            structure(
                "summary.trace.structure",
                A11ySemanticRoleClass::StructureGroup,
                vec![
                    dimension("spans", "summary.trace.dim.spans"),
                    dimension("critical_path_length", "summary.trace.dim.critical_path_length"),
                ],
            ),
            vec![
                drilldown(
                    "drilldown:trace.enumerate-structure",
                    Kind::EnumerateStructure,
                    "Enumerate spans and critical path",
                    "summary.trace.enumerate_structure",
                    "trace:structure",
                ),
                drilldown(
                    "drilldown:trace.describe-critical-path",
                    Kind::DescribeSeries,
                    "Describe critical-path spans",
                    "summary.trace.describe_critical_path",
                    "trace:critical-path",
                ),
                drilldown(
                    "drilldown:trace.read-alt-text",
                    Kind::ReadTextAlternative,
                    "Read trace timeline description",
                    "summary.trace.read_alt_text",
                    "trace:alt-text",
                ),
            ],
            text_alternative(
                Alt::ChartDescription,
                "summary.trace.alt_text",
                &["span_count", "critical_path_label_id", "total_duration_label_id"],
            ),
            fallback(Surface::StatusDetail, "status-detail:trace-summary"),
            vec![Consumer::Shell, Consumer::SupportExport],
        ),
        // Chart: approximate values; visual decision surface -> text alternative.
        summary(
            "summary:chart",
            M5SummarySurfaceKind::Chart,
            "Chart",
            "chart:active-chart",
            State::Approximate,
            vec![Producer::Observability],
            structure(
                "summary.chart.structure",
                A11ySemanticRoleClass::StructureGroup,
                vec![
                    dimension("series", "summary.chart.dim.series"),
                    dimension("data_points", "summary.chart.dim.data_points"),
                ],
            ),
            vec![
                drilldown(
                    "drilldown:chart.enumerate-structure",
                    Kind::EnumerateStructure,
                    "Enumerate axes and series",
                    "summary.chart.enumerate_structure",
                    "chart:structure",
                ),
                drilldown(
                    "drilldown:chart.describe-series",
                    Kind::DescribeSeries,
                    "Describe focused series",
                    "summary.chart.describe_series",
                    "chart:series",
                ),
                drilldown(
                    "drilldown:chart.read-alt-text",
                    Kind::ReadTextAlternative,
                    "Read chart description",
                    "summary.chart.read_alt_text",
                    "chart:alt-text",
                ),
                drilldown(
                    "drilldown:chart.open-metadata",
                    Kind::OpenMetadataView,
                    "Open chart metadata view",
                    "summary.chart.open_metadata",
                    "chart:metadata",
                ),
            ],
            text_alternative(
                Alt::ChartDescription,
                "summary.chart.alt_text",
                &[
                    "series_count",
                    "x_axis_label_id",
                    "y_axis_label_id",
                    "value_range_label_id",
                ],
            ),
            fallback(Surface::StatusDetail, "status-detail:chart-summary"),
            vec![Consumer::Shell, Consumer::SupportExport],
        ),
        // Rich review diff: AI-generated patch view; visual decision surface.
        summary(
            "summary:review-diff",
            M5SummarySurfaceKind::ReviewDiff,
            "Rich review diff",
            "review:active-diff",
            State::Generated,
            vec![Producer::Review],
            structure(
                "summary.review.structure",
                A11ySemanticRoleClass::StructureGroup,
                vec![
                    dimension("changed_files", "summary.review.dim.changed_files"),
                    dimension("hunks", "summary.review.dim.hunks"),
                ],
            ),
            vec![
                drilldown(
                    "drilldown:review.enumerate-structure",
                    Kind::EnumerateStructure,
                    "Enumerate changed files and hunks",
                    "summary.review.enumerate_structure",
                    "review:structure",
                ),
                drilldown(
                    "drilldown:review.open-hunk-detail",
                    Kind::OpenItemDetail,
                    "Open focused hunk detail",
                    "summary.review.open_hunk_detail",
                    "review:hunk",
                ),
                drilldown(
                    "drilldown:review.jump-conflict",
                    Kind::JumpToRegion,
                    "Jump to next conflict region",
                    "summary.review.jump_conflict",
                    "review:conflict",
                ),
            ],
            text_alternative(
                Alt::DiffSummary,
                "summary.review.alt_text",
                &["added_lines", "removed_lines", "changed_file_label_id"],
            ),
            fallback(Surface::PatchReviewHeader, "patch-review-header:review-summary"),
            vec![Consumer::Review, Consumer::SupportExport],
        ),
        // Image / design / rich artifact viewer: preview render; visual decision.
        summary(
            "summary:artifact-viewer",
            M5SummarySurfaceKind::ArtifactViewer,
            "Artifact viewer",
            "artifact:active-artifact",
            State::Preview,
            vec![Producer::Review, Producer::Docs],
            structure(
                "summary.artifact.structure",
                A11ySemanticRoleClass::StructureGroup,
                vec![
                    dimension("layers", "summary.artifact.dim.layers"),
                    dimension("dimensions_label", "summary.artifact.dim.dimensions_label"),
                ],
            ),
            vec![
                drilldown(
                    "drilldown:artifact.read-alt-text",
                    Kind::ReadTextAlternative,
                    "Read artifact alt text",
                    "summary.artifact.read_alt_text",
                    "artifact:alt-text",
                ),
                drilldown(
                    "drilldown:artifact.open-metadata",
                    Kind::OpenMetadataView,
                    "Open artifact metadata view",
                    "summary.artifact.open_metadata",
                    "artifact:metadata",
                ),
                drilldown(
                    "drilldown:artifact.enumerate-layers",
                    Kind::EnumerateStructure,
                    "Enumerate artifact layers",
                    "summary.artifact.enumerate_layers",
                    "artifact:structure",
                ),
            ],
            text_alternative(
                Alt::ImageAltText,
                "summary.artifact.alt_text",
                &["width_label_id", "height_label_id", "format_label_id", "layer_count"],
            ),
            fallback(Surface::BannerDetail, "banner-detail:artifact-summary"),
            vec![Consumer::Review, Consumer::SupportExport],
        ),
    ]
}

fn conformance_review() -> M5NonVisualSummaryConformanceReview {
    M5NonVisualSummaryConformanceReview {
        surfaces_explain_structure_without_vision_or_hover: true,
        drilldowns_remain_actionable_not_vague_one_liners: true,
        summaries_linked_to_same_object_identity_as_visual: true,
        provisional_states_visible_in_non_visual_representation: true,
        visual_artifacts_provide_text_alternative_and_metadata_view: true,
        chart_and_artifact_viewers_no_longer_require_visual_interpretation_alone: true,
        no_pixel_only_or_pointer_only_truth_source: true,
        claimed_surfaces_auto_narrow_when_bridge_or_proof_stale: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> M5NonVisualSummaryConsumerProjection {
    M5NonVisualSummaryConsumerProjection {
        editor_consumes_summaries: true,
        terminal_consumes_summaries: true,
        data_grid_and_tree_consume_summaries: true,
        observability_logs_traces_charts_consume_summaries: true,
        review_and_artifact_viewers_consume_summaries: true,
        docs_help_reuse_summaries: true,
        support_export_reuses_summaries: true,
        at_conformance_packets_reuse_summaries: true,
    }
}

fn proof_freshness() -> M5DynamicSurfaceA11yProofFreshness {
    M5DynamicSurfaceA11yProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DynamicSurfaceA11yReleasePosture {
    M5DynamicSurfaceA11yReleasePosture {
        release_packet_ref: "evidence:nonvisual-summary-release-packet:m5".to_owned(),
        mirror_offline_packet_ref: "evidence:nonvisual-summary-mirror-offline-packet:m5".to_owned(),
        support_export_parity_required: true,
        mirror_offline_parity_required: true,
        stable_promotion_blocks_without_mapped_proof: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_NONVISUAL_SUMMARY_SCHEMA_REF,
        M5_NONVISUAL_SUMMARY_DOC_REF,
        M5_NONVISUAL_SUMMARY_MATRIX_REF,
        M5_NONVISUAL_SUMMARY_ANNOUNCEMENT_GRAMMAR_REF,
        M5_NONVISUAL_SUMMARY_SURFACE_DESCRIPTOR_REF,
        M5_NONVISUAL_SUMMARY_SCREEN_READER_CONTRACT_REF,
    ])
}

fn base_input() -> M5NonVisualSummaryCatalogPacketInput {
    M5NonVisualSummaryCatalogPacketInput {
        packet_id: M5_NONVISUAL_SUMMARY_CATALOG_PACKET_ID.to_owned(),
        catalog_label: "M5 Non-Visual Custom-Surface Summaries".to_owned(),
        summaries: summaries(),
        shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet::canonical(),
        summary_vocabulary_set: M5NonVisualSummaryVocabularySet::canonical(),
        conformance_review: conformance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical stable non-visual summary catalog packet.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_m5_nonvisual_summary_catalog() -> M5NonVisualSummaryCatalogPacket {
    M5NonVisualSummaryCatalogPacket::new(base_input())
}

/// Builds a narrowed variant where the chart summary's assistive-tech proof has gone
/// stale, proving the surface narrows from Stable to Beta while keeping its structure,
/// drill-downs, text alternative, object identity, and `proof_stale` trigger intact.
pub fn seeded_m5_nonvisual_summary_catalog_proof_stale_narrowed() -> M5NonVisualSummaryCatalogPacket
{
    let mut input = base_input();
    input.packet_id = "m5-nonvisual-summary:proof-stale-narrowed:0001".to_owned();
    for summary in &mut input.summaries {
        if summary.surface_kind == M5SummarySurfaceKind::Chart {
            summary.qualification = M5DynamicSurfaceA11yQualificationClass::Beta;
        }
    }
    M5NonVisualSummaryCatalogPacket::new(input)
}

/// Builds a narrowed variant where the artifact-viewer summary's OS accessibility
/// bridge is unavailable, proving the surface narrows from Stable to Preview and drops
/// its non-visual fidelity to `degraded_accessible` while keeping its text alternative,
/// metadata view, drill-downs, and `bridge_unavailable` trigger — the artifact still
/// exposes its non-visual alternative rather than disappearing behind pixels.
pub fn seeded_m5_nonvisual_summary_catalog_bridge_unavailable_narrowed(
) -> M5NonVisualSummaryCatalogPacket {
    let mut input = base_input();
    input.packet_id = "m5-nonvisual-summary:bridge-unavailable-narrowed:0001".to_owned();
    for summary in &mut input.summaries {
        if summary.surface_kind == M5SummarySurfaceKind::ArtifactViewer {
            summary.qualification = M5DynamicSurfaceA11yQualificationClass::Preview;
            summary.non_visual_fidelity = A11yNonVisualFidelity::DegradedAccessible;
        }
    }
    M5NonVisualSummaryCatalogPacket::new(input)
}
