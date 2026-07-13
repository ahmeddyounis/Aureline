//! Canonical seed builders for the M5 syntax-, diff-, and chart-token registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean syntax, diff, and chart entries are
//! built so the canonical syntax / diff / chart semantic roles, the diagnostics-precedence posture, the
//! moved-block-confidence and historical-vs-current-emphasis notes, the legend / pattern parity, and the
//! screenshot / PDF / support-packet / high-contrast export survival are proven across the editor,
//! review, notebook, data, docs, and support surfaces without any diagnostics collision, color-only
//! meaning, raw-color inlining, or export loss.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SYNTAX_DIFF_CHART_REGISTRIES_PACKET_ID: &str =
    "m5-syntax-diff-and-chart-token-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn syntax(input: M5SyntaxEntryResolutionInput) -> M5ResolvedSyntaxEntry {
    resolve_syntax_entry(input).expect("seed syntax entry resolves")
}

fn diff(input: M5DiffEntryResolutionInput) -> M5ResolvedDiffEntry {
    resolve_diff_entry(input).expect("seed diff entry resolves")
}

fn chart(input: M5ChartEntryResolutionInput) -> M5ResolvedChartEntry {
    resolve_chart_entry(input).expect("seed chart entry resolves")
}

fn all_channels() -> Vec<M5MeaningExportChannel> {
    vec![
        M5MeaningExportChannel::Screenshot,
        M5MeaningExportChannel::Pdf,
        M5MeaningExportChannel::SupportPacket,
        M5MeaningExportChannel::HighContrast,
        M5MeaningExportChannel::MonochromePrint,
    ]
}

fn partial_channels() -> Vec<M5MeaningExportChannel> {
    vec![
        M5MeaningExportChannel::Screenshot,
        M5MeaningExportChannel::Pdf,
    ]
}

// -- Clean syntax entries -----------------------------------------------------------------------

fn clean_syntax_base(
    entry_id: &str,
    token_name: &str,
    syntax_role: M5SyntaxTokenRole,
    posture: M5SyntaxDiagnosticsPosture,
    surface_context: M5CodeDataSurfaceContext,
) -> M5SyntaxEntryResolutionInput {
    M5SyntaxEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5VisualSemanticRole::Syntax,
        syntax_role,
        diagnostics_posture: posture,
        surface_context,
        export_channels: all_channels(),
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn syntax_keyword_clean() -> M5ResolvedSyntaxEntry {
    syntax(clean_syntax_base(
        "syntax:editor:keyword",
        "syntax.keyword",
        M5SyntaxTokenRole::Keyword,
        M5SyntaxDiagnosticsPosture::DiagnosticsOutrankSyntax,
        M5CodeDataSurfaceContext::Editor,
    ))
}

fn syntax_string_clean() -> M5ResolvedSyntaxEntry {
    syntax(clean_syntax_base(
        "syntax:editor:string",
        "syntax.string_literal",
        M5SyntaxTokenRole::StringLiteral,
        M5SyntaxDiagnosticsPosture::DistinctNoOverlap,
        M5CodeDataSurfaceContext::Editor,
    ))
}

fn syntax_comment_clean() -> M5ResolvedSyntaxEntry {
    syntax(clean_syntax_base(
        "syntax:notebook:comment",
        "syntax.comment",
        M5SyntaxTokenRole::Comment,
        M5SyntaxDiagnosticsPosture::DiagnosticsOutrankSyntax,
        M5CodeDataSurfaceContext::Notebook,
    ))
}

fn syntax_identifier_clean() -> M5ResolvedSyntaxEntry {
    syntax(clean_syntax_base(
        "syntax:docs:identifier",
        "syntax.identifier",
        M5SyntaxTokenRole::Identifier,
        M5SyntaxDiagnosticsPosture::DistinctNoOverlap,
        M5CodeDataSurfaceContext::Docs,
    ))
}

fn syntax_distinct_clean() -> M5ResolvedSyntaxEntry {
    syntax(clean_syntax_base(
        "syntax:editor:distinct",
        "syntax.distinct_from_diagnostic",
        M5SyntaxTokenRole::DistinctFromDiagnostic,
        M5SyntaxDiagnosticsPosture::DiagnosticsOutrankSyntax,
        M5CodeDataSurfaceContext::Editor,
    ))
}

// -- Degraded syntax entries --------------------------------------------------------------------

/// Degraded syntax entry: the scope collides with a diagnostics color.
fn syntax_collision() -> M5ResolvedSyntaxEntry {
    syntax(clean_syntax_base(
        "syntax:editor:collision",
        "syntax.keyword",
        M5SyntaxTokenRole::SyntaxDiagnosticCollisionDisallowed,
        M5SyntaxDiagnosticsPosture::DiagnosticsOutrankSyntax,
        M5CodeDataSurfaceContext::Editor,
    ))
}

/// Degraded syntax entry: diagnostics do not outrank syntax color where they overlap.
fn syntax_precedence_missing() -> M5ResolvedSyntaxEntry {
    syntax(clean_syntax_base(
        "syntax:editor:precedence-missing",
        "syntax.string_literal",
        M5SyntaxTokenRole::StringLiteral,
        M5SyntaxDiagnosticsPosture::SyntaxOutranksDiagnosticsDisallowed,
        M5CodeDataSurfaceContext::Editor,
    ))
}

/// Degraded syntax entry: a raw color value is inlined instead of tracing to a canonical token.
fn syntax_raw_inlined() -> M5ResolvedSyntaxEntry {
    let mut input = clean_syntax_base(
        "syntax:editor:raw-inlined",
        "syntax.identifier",
        M5SyntaxTokenRole::Identifier,
        M5SyntaxDiagnosticsPosture::DiagnosticsOutrankSyntax,
        M5CodeDataSurfaceContext::Editor,
    );
    input.references_canonical_token = false;
    syntax(input)
}

/// Degraded syntax entry: the meaning is lost under a required export / high-contrast channel.
fn syntax_export_loss() -> M5ResolvedSyntaxEntry {
    let mut input = clean_syntax_base(
        "syntax:editor:export-loss",
        "syntax.comment",
        M5SyntaxTokenRole::Comment,
        M5SyntaxDiagnosticsPosture::DiagnosticsOutrankSyntax,
        M5CodeDataSurfaceContext::Editor,
    );
    input.export_channels = partial_channels();
    syntax(input)
}

/// Degraded syntax entry: the canonical token name is unstated.
fn syntax_token_unstated() -> M5ResolvedSyntaxEntry {
    let mut input = clean_syntax_base(
        "syntax:support:token-unstated",
        "  ",
        M5SyntaxTokenRole::Keyword,
        M5SyntaxDiagnosticsPosture::DiagnosticsOutrankSyntax,
        M5CodeDataSurfaceContext::Editor,
    );
    input.token_name = "  ".to_owned();
    syntax(input)
}

// -- Clean diff entries -------------------------------------------------------------------------

fn clean_diff_base(
    entry_id: &str,
    token_name: &str,
    diff_role: M5DiffTokenRole,
    moved_confidence: M5DiffMovedConfidence,
    emphasis: M5DiffEmphasis,
    non_color_cue: M5CodeDataNonColorCue,
    surface_context: M5CodeDataSurfaceContext,
) -> M5DiffEntryResolutionInput {
    M5DiffEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5VisualSemanticRole::Diff,
        diff_role,
        moved_confidence,
        historical_emphasis: emphasis,
        non_color_cue,
        surface_context,
        export_channels: all_channels(),
        distinct_from_diagnostics: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn diff_addition_clean() -> M5ResolvedDiffEntry {
    diff(clean_diff_base(
        "diff:review:addition",
        "diff.addition",
        M5DiffTokenRole::Addition,
        M5DiffMovedConfidence::NotAMove,
        M5DiffEmphasis::CurrentEmphasis,
        M5CodeDataNonColorCue::FillPattern,
        M5CodeDataSurfaceContext::Review,
    ))
}

fn diff_removal_clean() -> M5ResolvedDiffEntry {
    diff(clean_diff_base(
        "diff:review:removal",
        "diff.removal",
        M5DiffTokenRole::Removal,
        M5DiffMovedConfidence::NotAMove,
        M5DiffEmphasis::HistoricalEmphasis,
        M5CodeDataNonColorCue::TextLabel,
        M5CodeDataSurfaceContext::Review,
    ))
}

fn diff_context_clean() -> M5ResolvedDiffEntry {
    diff(clean_diff_base(
        "diff:review:context",
        "diff.context",
        M5DiffTokenRole::Context,
        M5DiffMovedConfidence::NotAMove,
        M5DiffEmphasis::BalancedEmphasis,
        M5CodeDataNonColorCue::TextLabel,
        M5CodeDataSurfaceContext::Review,
    ))
}

fn diff_moved_clean() -> M5ResolvedDiffEntry {
    diff(clean_diff_base(
        "diff:notebook:moved",
        "diff.moved",
        M5DiffTokenRole::Moved,
        M5DiffMovedConfidence::HighConfidenceMove,
        M5DiffEmphasis::CurrentEmphasis,
        M5CodeDataNonColorCue::SeriesMarker,
        M5CodeDataSurfaceContext::Notebook,
    ))
}

fn diff_moved_likely_clean() -> M5ResolvedDiffEntry {
    diff(clean_diff_base(
        "diff:notebook:moved-likely",
        "diff.moved",
        M5DiffTokenRole::Moved,
        M5DiffMovedConfidence::LikelyMove,
        M5DiffEmphasis::HistoricalEmphasis,
        M5CodeDataNonColorCue::SeriesMarker,
        M5CodeDataSurfaceContext::Notebook,
    ))
}

// -- Degraded diff entries ----------------------------------------------------------------------

/// Degraded diff entry: the diff region collides with a diagnostics color.
fn diff_collision() -> M5ResolvedDiffEntry {
    diff(clean_diff_base(
        "diff:review:collision",
        "diff.addition",
        M5DiffTokenRole::DiffDiagnosticCollisionDisallowed,
        M5DiffMovedConfidence::NotAMove,
        M5DiffEmphasis::CurrentEmphasis,
        M5CodeDataNonColorCue::FillPattern,
        M5CodeDataSurfaceContext::Review,
    ))
}

/// Degraded diff entry: no non-color cue is paired with the hue.
fn diff_cue_missing() -> M5ResolvedDiffEntry {
    diff(clean_diff_base(
        "diff:review:cue-missing",
        "diff.removal",
        M5DiffTokenRole::Removal,
        M5DiffMovedConfidence::NotAMove,
        M5DiffEmphasis::HistoricalEmphasis,
        M5CodeDataNonColorCue::NoneDisallowed,
        M5CodeDataSurfaceContext::Review,
    ))
}

/// Degraded diff entry: the moved-block confidence is unstated.
fn diff_moved_unstated() -> M5ResolvedDiffEntry {
    diff(clean_diff_base(
        "diff:notebook:moved-unstated",
        "diff.moved",
        M5DiffTokenRole::Moved,
        M5DiffMovedConfidence::ConfidenceUnknown,
        M5DiffEmphasis::CurrentEmphasis,
        M5CodeDataNonColorCue::SeriesMarker,
        M5CodeDataSurfaceContext::Notebook,
    ))
}

/// Degraded diff entry: the historical-vs-current emphasis is unstated.
fn diff_emphasis_unstated() -> M5ResolvedDiffEntry {
    diff(clean_diff_base(
        "diff:notebook:emphasis-unstated",
        "diff.context",
        M5DiffTokenRole::Context,
        M5DiffMovedConfidence::NotAMove,
        M5DiffEmphasis::EmphasisUnknown,
        M5CodeDataNonColorCue::TextLabel,
        M5CodeDataSurfaceContext::Notebook,
    ))
}

/// Degraded diff entry: the meaning is lost under a required export / high-contrast channel.
fn diff_export_loss() -> M5ResolvedDiffEntry {
    let mut input = clean_diff_base(
        "diff:review:export-loss",
        "diff.addition",
        M5DiffTokenRole::Addition,
        M5DiffMovedConfidence::NotAMove,
        M5DiffEmphasis::CurrentEmphasis,
        M5CodeDataNonColorCue::FillPattern,
        M5CodeDataSurfaceContext::Review,
    );
    input.export_channels = partial_channels();
    diff(input)
}

/// Degraded diff entry: a raw color value is inlined instead of tracing to a canonical token.
fn diff_raw_inlined() -> M5ResolvedDiffEntry {
    let mut input = clean_diff_base(
        "diff:review:raw-inlined",
        "diff.removal",
        M5DiffTokenRole::Removal,
        M5DiffMovedConfidence::NotAMove,
        M5DiffEmphasis::HistoricalEmphasis,
        M5CodeDataNonColorCue::TextLabel,
        M5CodeDataSurfaceContext::Review,
    );
    input.references_canonical_token = false;
    diff(input)
}

// -- Clean chart entries ------------------------------------------------------------------------

fn clean_chart_base(
    entry_id: &str,
    token_name: &str,
    chart_role: M5ChartTokenRole,
    non_color_cue: M5CodeDataNonColorCue,
    surface_context: M5CodeDataSurfaceContext,
) -> M5ChartEntryResolutionInput {
    M5ChartEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5VisualSemanticRole::Chart,
        chart_role,
        non_color_cue,
        surface_context,
        export_channels: all_channels(),
        legend_or_pattern_present: true,
        accessible_contrast: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn chart_categorical_clean() -> M5ResolvedChartEntry {
    chart(clean_chart_base(
        "chart:data:categorical",
        "chart.categorical_series",
        M5ChartTokenRole::CategoricalSeries,
        M5CodeDataNonColorCue::Legend,
        M5CodeDataSurfaceContext::Data,
    ))
}

fn chart_sequential_clean() -> M5ResolvedChartEntry {
    chart(clean_chart_base(
        "chart:data:sequential",
        "chart.sequential_scale",
        M5ChartTokenRole::SequentialScale,
        M5CodeDataNonColorCue::FillPattern,
        M5CodeDataSurfaceContext::Data,
    ))
}

fn chart_diverging_clean() -> M5ResolvedChartEntry {
    chart(clean_chart_base(
        "chart:docs:diverging",
        "chart.diverging_scale",
        M5ChartTokenRole::DivergingScale,
        M5CodeDataNonColorCue::SeriesMarker,
        M5CodeDataSurfaceContext::Docs,
    ))
}

fn chart_paired_clean() -> M5ResolvedChartEntry {
    chart(clean_chart_base(
        "chart:data:paired",
        "chart.paired_with_shape_or_label",
        M5ChartTokenRole::PairedWithShapeOrLabel,
        M5CodeDataNonColorCue::Legend,
        M5CodeDataSurfaceContext::Data,
    ))
}

// -- Degraded chart entries ---------------------------------------------------------------------

/// Degraded chart entry: the meaning is encoded by color alone.
fn chart_color_alone() -> M5ResolvedChartEntry {
    chart(clean_chart_base(
        "chart:data:color-alone",
        "chart.categorical_series",
        M5ChartTokenRole::ChartColorAloneDisallowed,
        M5CodeDataNonColorCue::Legend,
        M5CodeDataSurfaceContext::Data,
    ))
}

/// Degraded chart entry: no legend or pattern is present where color alone would be insufficient.
fn chart_legend_missing() -> M5ResolvedChartEntry {
    let mut input = clean_chart_base(
        "chart:data:legend-missing",
        "chart.sequential_scale",
        M5ChartTokenRole::SequentialScale,
        M5CodeDataNonColorCue::FillPattern,
        M5CodeDataSurfaceContext::Data,
    );
    input.legend_or_pattern_present = false;
    chart(input)
}

/// Degraded chart entry: the series does not meet accessible contrast.
fn chart_contrast_insufficient() -> M5ResolvedChartEntry {
    let mut input = clean_chart_base(
        "chart:data:contrast",
        "chart.diverging_scale",
        M5ChartTokenRole::DivergingScale,
        M5CodeDataNonColorCue::SeriesMarker,
        M5CodeDataSurfaceContext::Data,
    );
    input.accessible_contrast = false;
    chart(input)
}

/// Degraded chart entry: the meaning is lost under a required export / high-contrast channel.
fn chart_export_loss() -> M5ResolvedChartEntry {
    let mut input = clean_chart_base(
        "chart:data:export-loss",
        "chart.categorical_series",
        M5ChartTokenRole::CategoricalSeries,
        M5CodeDataNonColorCue::Legend,
        M5CodeDataSurfaceContext::Data,
    );
    input.export_channels = partial_channels();
    chart(input)
}

/// Degraded chart entry: a raw color value is inlined instead of tracing to a canonical token.
fn chart_raw_inlined() -> M5ResolvedChartEntry {
    let mut input = clean_chart_base(
        "chart:data:raw-inlined",
        "chart.sequential_scale",
        M5ChartTokenRole::SequentialScale,
        M5CodeDataNonColorCue::FillPattern,
        M5CodeDataSurfaceContext::Data,
    );
    input.references_canonical_token = false;
    chart(input)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5CodeDataConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5VisualFoundationDowngradeTrigger>,
    syntax_entries: Vec<M5ResolvedSyntaxEntry>,
    diff_entries: Vec<M5ResolvedDiffEntry>,
    chart_entries: Vec<M5ResolvedChartEntry>,
) -> M5SyntaxDiffChartRegistriesRow {
    M5SyntaxDiffChartRegistriesRow {
        consumer_surface,
        qualification: M5VisualFoundationQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5VisualFoundationDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5VisualFoundationRequiredLabel::Identity,
            M5VisualFoundationRequiredLabel::SemanticRole,
            M5VisualFoundationRequiredLabel::TokenReference,
            M5VisualFoundationRequiredLabel::ContrastPairing,
        ],
        accessibility_routes: M5VisualFoundationAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5CodeDataAnatomyPart::ALL.to_vec(),
        export_fields: M5CodeDataExportField::ALL.to_vec(),
        downgrade_triggers,
        syntax_entries,
        diff_entries,
        chart_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SYNTAX_DIFF_CHART_REGISTRIES_SCHEMA_REF,
            M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF,
        ]),
        syntax_or_diff_palette_collides_with_diagnostics: false,
        chart_meaning_relies_on_color_alone: false,
        meaning_lost_under_high_contrast_or_export: false,
        raw_color_value_inlined_instead_of_token: false,
    }
}

fn registry_rows() -> Vec<M5SyntaxDiffChartRegistriesRow> {
    use M5VisualFoundationConsumerSurface as C;
    use M5VisualFoundationDowngradeTrigger as D;

    vec![
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor consumes the canonical keyword / string / identifier syntax scopes with diagnostics outranking syntax color; a diagnostics collision and a raw-color inlining degrade honestly instead of reading as a clean pass",
            "evidence:m5-syntax-diff-chart-editor-ui:001",
            vec![
                D::SyntaxOrDiffPaletteCollidedWithDiagnostics,
                D::TokenReferenceUnstated,
                D::ProofStale,
            ],
            vec![
                syntax_keyword_clean(),
                syntax_string_clean(),
                syntax_distinct_clean(),
                syntax_collision(),
                syntax_raw_inlined(),
            ],
            vec![],
            vec![],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review surface keeps addition / removal / context diff regions distinct from diagnostics, pairs each with a label or pattern, and states historical-vs-current emphasis; a diagnostics collision and a cue-missing region degrade honestly",
            "evidence:m5-syntax-diff-chart-review-ui:001",
            vec![
                D::SyntaxOrDiffPaletteCollidedWithDiagnostics,
                D::StatusOrTrustCollapsedToColorOnly,
                D::ProofStale,
            ],
            vec![],
            vec![
                diff_addition_clean(),
                diff_removal_clean(),
                diff_context_clean(),
                diff_collision(),
                diff_cue_missing(),
            ],
            vec![],
        ),
        base_row(
            C::DataUi,
            "Data surface owner",
            "The data surface keeps categorical / sequential / diverging chart series distinguishable with legends, patterns, and markers at accessible contrast; a color-alone series, a legend-missing series, and a low-contrast series degrade honestly",
            "evidence:m5-syntax-diff-chart-data-ui:001",
            vec![
                D::ChartMeaningDependedOnColorAlone,
                D::TokenReferenceUnstated,
                D::ProofStale,
            ],
            vec![],
            vec![],
            vec![
                chart_categorical_clean(),
                chart_sequential_clean(),
                chart_paired_clean(),
                chart_color_alone(),
                chart_legend_missing(),
                chart_contrast_insufficient(),
            ],
        ),
        base_row(
            C::DocsUi,
            "Docs surface owner",
            "The docs surface renders code, diffs, and charts with the same syntax identifier scope, diverging chart scale, and non-color cues so meaning survives when the page is exported",
            "evidence:m5-syntax-diff-chart-docs-ui:001",
            vec![
                D::SyntaxOrDiffPaletteCollidedWithDiagnostics,
                D::ChartMeaningDependedOnColorAlone,
                D::ProofStale,
            ],
            vec![syntax_identifier_clean()],
            vec![],
            vec![chart_diverging_clean()],
        ),
        base_row(
            C::ShellUi,
            "Shell / notebook surface owner",
            "The shell and notebook surfaces render inline code comments and moved-block diffs with stated moved-block confidence and high-contrast survival; a precedence-losing scope, an export-losing view, and unstated moved / emphasis notes degrade honestly",
            "evidence:m5-syntax-diff-chart-shell-ui:001",
            vec![
                D::SyntaxOrDiffPaletteCollidedWithDiagnostics,
                D::SemanticRoleUnstated,
                D::ProofStale,
            ],
            vec![
                syntax_comment_clean(),
                syntax_precedence_missing(),
                syntax_export_loss(),
            ],
            vec![
                diff_moved_clean(),
                diff_moved_likely_clean(),
                diff_moved_unstated(),
                diff_emphasis_unstated(),
                diff_export_loss(),
            ],
            vec![],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved syntax, diff, and chart truth, so a raw-color regression, an unstated token, or an export-losing chart is visible in evidence rather than hidden behind hue",
            "evidence:m5-syntax-diff-chart-support-export:001",
            vec![
                D::TokenReferenceUnstated,
                D::ChartMeaningDependedOnColorAlone,
                D::ProofStale,
            ],
            vec![syntax_keyword_clean(), syntax_token_unstated()],
            vec![diff_addition_clean(), diff_raw_inlined()],
            vec![
                chart_categorical_clean(),
                chart_raw_inlined(),
                chart_export_loss(),
            ],
        ),
    ]
}

fn governance_review() -> M5SyntaxDiffChartGovernanceReview {
    M5SyntaxDiffChartGovernanceReview {
        syntax_diff_chart_share_one_semantic_mapping: true,
        diagnostics_outrank_syntax_where_they_overlap: true,
        syntax_and_diff_never_collide_with_diagnostics: true,
        diff_meaning_survives_high_contrast_and_export: true,
        chart_meaning_never_relies_on_color_alone: true,
        legend_or_pattern_parity_present_where_color_insufficient: true,
        moved_block_confidence_and_historical_emphasis_stated: true,
        raw_color_drift_caught_before_release: true,
        first_consumers_use_canonical_families: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SyntaxDiffChartConsumerProjection {
    M5SyntaxDiffChartConsumerProjection {
        editor_consumes_shared_syntax_tokens: true,
        review_consumes_shared_diff_tokens: true,
        data_consumes_shared_chart_tokens: true,
        docs_and_notebook_consume_shared_registries: true,
        code_and_data_meaning_traces_to_single_domain_contract: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SyntaxDiffChartProofFreshness {
    M5SyntaxDiffChartProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SyntaxDiffChartReleasePosture {
    M5SyntaxDiffChartReleasePosture {
        proof_packet_ref: M5_SYNTAX_DIFF_CHART_REGISTRIES_ARTIFACT_REF.to_owned(),
        foundation_audit_ref: M5_SYNTAX_DIFF_CHART_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SYNTAX_DIFF_CHART_REGISTRIES_SCHEMA_REF,
        M5_SYNTAX_DIFF_CHART_REGISTRIES_DOC_REF,
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
        M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
        M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 syntax-, diff-, and chart-token registries packet.
pub fn seeded_m5_syntax_diff_chart_registries() -> M5SyntaxDiffChartRegistriesPacket {
    M5SyntaxDiffChartRegistriesPacket::new(M5SyntaxDiffChartRegistriesPacketInput {
        packet_id: M5_SYNTAX_DIFF_CHART_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 syntax-, diff-, and chart-token registries with diagnostics precedence over syntax color, moved-block-confidence and historical-vs-current-emphasis notes, legend / pattern / marker parity, and screenshot / PDF / support-packet / high-contrast export survival across the editor, review, notebook, data, docs, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5SyntaxDiffChartVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the editor-UI row is held at Beta pending diagnostics-precedence proof on every
/// deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_syntax_diff_chart_registries_editor_ui_beta_narrowed(
) -> M5SyntaxDiffChartRegistriesPacket {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    packet.packet_id = "m5-syntax-diff-and-chart-token-registries:editor-ui-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualFoundationConsumerSurface::EditorUi)
        .expect("editor-ui row present");
    row.qualification = M5VisualFoundationQualificationClass::Beta;
    packet
}

/// Narrowed variant: the data-UI row is narrowed to Preview pending legend / pattern parity on every
/// chart series; every row stays visible and every example stays honest.
pub fn seeded_m5_syntax_diff_chart_registries_data_ui_preview_narrowed(
) -> M5SyntaxDiffChartRegistriesPacket {
    let mut packet = seeded_m5_syntax_diff_chart_registries();
    packet.packet_id = "m5-syntax-diff-and-chart-token-registries:data-ui-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualFoundationConsumerSurface::DataUi)
        .expect("data-ui row present");
    row.qualification = M5VisualFoundationQualificationClass::Preview;
    packet
}
