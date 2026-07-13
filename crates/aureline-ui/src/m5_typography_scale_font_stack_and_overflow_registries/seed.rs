//! Canonical seed builders for the M5 typography-scale, font-stack, and text-overflow registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean type-scale and overflow entries are
//! built so the canonical title / body / label / code hierarchy, the UI-sans / code-mono font policy, the
//! line-height guards, the tabular-numeral rule for numeric data, and the meaning-preserving overflow
//! behavior are proven across the shell, editor, review, docs, data, and support surfaces without any
//! font-stack drift, silent clip, zoom / density regression, or raw-value inlining.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_PACKET_ID: &str =
    "m5-typography-scale-font-stack-and-overflow-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn type_scale(input: M5TypeScaleEntryResolutionInput) -> M5ResolvedTypeScaleEntry {
    resolve_type_scale_entry(input).expect("seed type-scale entry resolves")
}

fn overflow(input: M5OverflowEntryResolutionInput) -> M5ResolvedOverflowEntry {
    resolve_overflow_entry(input).expect("seed overflow entry resolves")
}

// -- Clean type-scale entries -------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn type_scale_base(
    entry_id: &str,
    token_name: &str,
    typography_role: M5TypographyRole,
    text_role: M5TextRole,
    font_stack: M5FontStackSelection,
    case_rule: M5TextCaseRule,
    surface_context: M5TextSurfaceContext,
    tabular_numerals_enabled: bool,
) -> M5TypeScaleEntryResolutionInput {
    M5TypeScaleEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5VisualSemanticRole::Neutral,
        typography_role,
        text_role,
        font_stack,
        case_rule,
        surface_context,
        line_height_guarded: true,
        tabular_numerals_enabled,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn type_title_clean() -> M5ResolvedTypeScaleEntry {
    type_scale(type_scale_base(
        "type:shell:title",
        "type.title",
        M5TypographyRole::DisplayScale,
        M5TextRole::Title,
        M5FontStackSelection::UiSansStack,
        M5TextCaseRule::SentenceCase,
        M5TextSurfaceContext::Shell,
        false,
    ))
}

fn type_body_clean() -> M5ResolvedTypeScaleEntry {
    type_scale(type_scale_base(
        "type:review:body",
        "type.body",
        M5TypographyRole::BodyScale,
        M5TextRole::Body,
        M5FontStackSelection::UiSansStack,
        M5TextCaseRule::SentenceCase,
        M5TextSurfaceContext::Review,
        false,
    ))
}

fn type_label_clean() -> M5ResolvedTypeScaleEntry {
    type_scale(type_scale_base(
        "type:docs:label",
        "type.label",
        M5TypographyRole::BodyScale,
        M5TextRole::Label,
        M5FontStackSelection::UiSansStack,
        M5TextCaseRule::SentenceCase,
        M5TextSurfaceContext::Docs,
        false,
    ))
}

fn type_code_clean() -> M5ResolvedTypeScaleEntry {
    type_scale(type_scale_base(
        "type:editor:code",
        "type.code",
        M5TypographyRole::CodeMonoStack,
        M5TextRole::Code,
        M5FontStackSelection::CodeMonoStack,
        M5TextCaseRule::DefaultText,
        M5TextSurfaceContext::Editor,
        false,
    ))
}

fn type_numeric_clean() -> M5ResolvedTypeScaleEntry {
    type_scale(type_scale_base(
        "type:data:numeric",
        "type.numeric_data",
        M5TypographyRole::TabularNumerals,
        M5TextRole::NumericData,
        M5FontStackSelection::UiSansStack,
        M5TextCaseRule::DefaultText,
        M5TextSurfaceContext::Data,
        true,
    ))
}

// -- Degraded type-scale entries ----------------------------------------------------------------

/// Degraded type-scale entry: a code role selects a UI stack, so the font stack does not match the role.
fn type_font_stack_unstable() -> M5ResolvedTypeScaleEntry {
    type_scale(type_scale_base(
        "type:editor:font-unstable",
        "type.code",
        M5TypographyRole::CodeMonoStack,
        M5TextRole::Code,
        M5FontStackSelection::UiSansStack,
        M5TextCaseRule::DefaultText,
        M5TextSurfaceContext::Editor,
        false,
    ))
}

/// Degraded type-scale entry: the line-height is not guarded and may drift.
fn type_line_height_drift() -> M5ResolvedTypeScaleEntry {
    let mut input = type_scale_base(
        "type:review:line-height-drift",
        "type.body",
        M5TypographyRole::BodyScale,
        M5TextRole::Body,
        M5FontStackSelection::UiSansStack,
        M5TextCaseRule::SentenceCase,
        M5TextSurfaceContext::Review,
        false,
    );
    input.line_height_guarded = false;
    type_scale(input)
}

/// Degraded type-scale entry: numeric data is missing tabular numerals.
fn type_tabular_missing() -> M5ResolvedTypeScaleEntry {
    type_scale(type_scale_base(
        "type:data:tabular-missing",
        "type.numeric_data",
        M5TypographyRole::TabularNumerals,
        M5TextRole::NumericData,
        M5FontStackSelection::UiSansStack,
        M5TextCaseRule::DefaultText,
        M5TextSurfaceContext::Data,
        false,
    ))
}

/// Degraded type-scale entry: the sentence-case / default-text rule is unstated.
fn type_case_unstated() -> M5ResolvedTypeScaleEntry {
    type_scale(type_scale_base(
        "type:shell:case-unstated",
        "type.title",
        M5TypographyRole::DisplayScale,
        M5TextRole::Title,
        M5FontStackSelection::UiSansStack,
        M5TextCaseRule::CaseRuleUnknown,
        M5TextSurfaceContext::Shell,
        false,
    ))
}

/// Degraded type-scale entry: the type-hierarchy role is unstated.
fn type_role_unstated() -> M5ResolvedTypeScaleEntry {
    type_scale(type_scale_base(
        "type:support:role-unstated",
        "type.body",
        M5TypographyRole::BodyScale,
        M5TextRole::RoleUnknown,
        M5FontStackSelection::UiSansStack,
        M5TextCaseRule::SentenceCase,
        M5TextSurfaceContext::Data,
        false,
    ))
}

/// Degraded type-scale entry: a raw type value is inlined instead of tracing to a canonical token.
fn type_raw_inlined() -> M5ResolvedTypeScaleEntry {
    let mut input = type_scale_base(
        "type:support:raw-inlined",
        "type.body",
        M5TypographyRole::BodyScale,
        M5TextRole::Body,
        M5FontStackSelection::UiSansStack,
        M5TextCaseRule::SentenceCase,
        M5TextSurfaceContext::Docs,
        false,
    );
    input.references_canonical_token = false;
    type_scale(input)
}

/// Degraded type-scale entry: the canonical token name is unstated.
fn type_token_unstated() -> M5ResolvedTypeScaleEntry {
    let mut input = type_scale_base(
        "type:support:token-unstated",
        "  ",
        M5TypographyRole::BodyScale,
        M5TextRole::Body,
        M5FontStackSelection::UiSansStack,
        M5TextCaseRule::SentenceCase,
        M5TextSurfaceContext::Docs,
        false,
    );
    input.token_name = "  ".to_owned();
    type_scale(input)
}

// -- Clean overflow entries ---------------------------------------------------------------------

fn overflow_base(
    entry_id: &str,
    token_name: &str,
    surface_element: M5TextSurfaceElement,
    overflow_treatment: M5OverflowTreatment,
    density_context: M5DensityContext,
    surface_context: M5TextSurfaceContext,
) -> M5OverflowEntryResolutionInput {
    M5OverflowEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5VisualSemanticRole::Neutral,
        surface_element,
        overflow_treatment,
        density_context,
        surface_context,
        full_meaning_reachable: true,
        survives_zoom: true,
        survives_density: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn overflow_tab_clean() -> M5ResolvedOverflowEntry {
    overflow(overflow_base(
        "overflow:shell:tab",
        "overflow.tab",
        M5TextSurfaceElement::Tab,
        M5OverflowTreatment::EllipsisWithReveal,
        M5DensityContext::Comfortable,
        M5TextSurfaceContext::Shell,
    ))
}

fn overflow_row_clean() -> M5ResolvedOverflowEntry {
    overflow(overflow_base(
        "overflow:data:row",
        "overflow.row",
        M5TextSurfaceElement::Row,
        M5OverflowTreatment::TruncateWithTooltip,
        M5DensityContext::Compact,
        M5TextSurfaceContext::Data,
    ))
}

fn overflow_inspector_clean() -> M5ResolvedOverflowEntry {
    overflow(overflow_base(
        "overflow:docs:inspector",
        "overflow.inspector",
        M5TextSurfaceElement::Inspector,
        M5OverflowTreatment::TruncateWithTooltip,
        M5DensityContext::Comfortable,
        M5TextSurfaceContext::Docs,
    ))
}

fn overflow_banner_clean() -> M5ResolvedOverflowEntry {
    overflow(overflow_base(
        "overflow:review:banner",
        "overflow.banner",
        M5TextSurfaceElement::Banner,
        M5OverflowTreatment::WrapToNextLine,
        M5DensityContext::Comfortable,
        M5TextSurfaceContext::Review,
    ))
}

fn overflow_code_metadata_clean() -> M5ResolvedOverflowEntry {
    overflow(overflow_base(
        "overflow:editor:code-metadata",
        "overflow.code_adjacent_metadata",
        M5TextSurfaceElement::CodeAdjacentMetadata,
        M5OverflowTreatment::HorizontalScroll,
        M5DensityContext::Compact,
        M5TextSurfaceContext::Editor,
    ))
}

// -- Degraded overflow entries ------------------------------------------------------------------

/// Degraded overflow entry: a silent clip destroys meaning.
fn overflow_meaning_destroyed() -> M5ResolvedOverflowEntry {
    overflow(overflow_base(
        "overflow:data:silent-clip",
        "overflow.row",
        M5TextSurfaceElement::Row,
        M5OverflowTreatment::SilentClipDisallowed,
        M5DensityContext::Compact,
        M5TextSurfaceContext::Data,
    ))
}

/// Degraded overflow entry: the full meaning is not reachable off the truncation.
fn overflow_full_meaning_unreachable() -> M5ResolvedOverflowEntry {
    let mut input = overflow_base(
        "overflow:editor:unreachable",
        "overflow.code_adjacent_metadata",
        M5TextSurfaceElement::CodeAdjacentMetadata,
        M5OverflowTreatment::TruncateWithTooltip,
        M5DensityContext::Compact,
        M5TextSurfaceContext::Editor,
    );
    input.full_meaning_reachable = false;
    overflow(input)
}

/// Degraded overflow entry: the behavior regresses under a zoom change.
fn overflow_zoom_regression() -> M5ResolvedOverflowEntry {
    let mut input = overflow_base(
        "overflow:shell:zoom-regression",
        "overflow.tab",
        M5TextSurfaceElement::Tab,
        M5OverflowTreatment::EllipsisWithReveal,
        M5DensityContext::Comfortable,
        M5TextSurfaceContext::Shell,
    );
    input.survives_zoom = false;
    overflow(input)
}

/// Degraded overflow entry: the behavior regresses under a density change.
fn overflow_density_regression() -> M5ResolvedOverflowEntry {
    let mut input = overflow_base(
        "overflow:review:density-regression",
        "overflow.banner",
        M5TextSurfaceElement::Banner,
        M5OverflowTreatment::WrapToNextLine,
        M5DensityContext::Compact,
        M5TextSurfaceContext::Review,
    );
    input.survives_density = false;
    overflow(input)
}

/// Degraded overflow entry: a raw layout value is inlined instead of tracing to a canonical token.
fn overflow_raw_inlined() -> M5ResolvedOverflowEntry {
    let mut input = overflow_base(
        "overflow:support:raw-inlined",
        "overflow.row",
        M5TextSurfaceElement::Row,
        M5OverflowTreatment::TruncateWithTooltip,
        M5DensityContext::Compact,
        M5TextSurfaceContext::Data,
    );
    input.references_canonical_token = false;
    overflow(input)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5TypographyConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5VisualFoundationDowngradeTrigger>,
    type_scale_entries: Vec<M5ResolvedTypeScaleEntry>,
    overflow_entries: Vec<M5ResolvedOverflowEntry>,
) -> M5TypographyOverflowRegistriesRow {
    M5TypographyOverflowRegistriesRow {
        consumer_surface,
        qualification: M5VisualFoundationQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5VisualFoundationDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5VisualFoundationRequiredLabel::Identity,
            M5VisualFoundationRequiredLabel::SemanticRole,
            M5VisualFoundationRequiredLabel::TokenReference,
            M5VisualFoundationRequiredLabel::DensityContext,
        ],
        accessibility_routes: M5VisualFoundationAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5TextAnatomyPart::ALL.to_vec(),
        export_fields: M5TextExportField::ALL.to_vec(),
        downgrade_triggers,
        type_scale_entries,
        overflow_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_SCHEMA_REF,
            M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF,
        ]),
        typography_scale_or_font_stack_drifted: false,
        overflow_silently_destroyed_meaning: false,
        zoom_or_density_regression_uncaught: false,
        raw_type_value_inlined_instead_of_token: false,
    }
}

fn registry_rows() -> Vec<M5TypographyOverflowRegistriesRow> {
    use M5VisualFoundationConsumerSurface as C;
    use M5VisualFoundationDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell renders titles on the canonical display scale and sans stack with a stated case rule, and ellipsizes tab labels with a reveal that survives zoom; an unstated case rule and a zoom regression degrade honestly instead of reading as a clean pass",
            "evidence:m5-typography-overflow-shell-ui:001",
            vec![
                D::TypographyScaleDrifted,
                D::SemanticRoleUnstated,
                D::ProofStale,
            ],
            vec![type_title_clean(), type_case_unstated()],
            vec![overflow_tab_clean(), overflow_zoom_regression()],
        ),
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor renders code on the monospace stack and scrolls code-adjacent metadata so the full path stays reachable; a code role that selects the UI stack and an unreachable truncation degrade honestly",
            "evidence:m5-typography-overflow-editor-ui:001",
            vec![
                D::FontStackUnstable,
                D::TypographyScaleDrifted,
                D::ProofStale,
            ],
            vec![type_code_clean(), type_font_stack_unstable()],
            vec![
                overflow_code_metadata_clean(),
                overflow_full_meaning_unreachable(),
            ],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review surface renders body text on the shared body scale and wraps banner notices so meaning survives; a line-height drift and a density regression degrade honestly",
            "evidence:m5-typography-overflow-review-ui:001",
            vec![
                D::TypographyScaleDrifted,
                D::LocalGeometryForkedFromFoundation,
                D::ProofStale,
            ],
            vec![type_body_clean(), type_line_height_drift()],
            vec![overflow_banner_clean(), overflow_density_regression()],
        ),
        base_row(
            C::DocsUi,
            "Docs surface owner",
            "The docs surface renders labels on the shared scale and truncates inspector fields with a tooltip that carries the full text, so type hierarchy and overflow behavior stay stable when the page is exported",
            "evidence:m5-typography-overflow-docs-ui:001",
            vec![
                D::TypographyScaleDrifted,
                D::SemanticRoleUnstated,
                D::ProofStale,
            ],
            vec![type_label_clean()],
            vec![overflow_inspector_clean()],
        ),
        base_row(
            C::DataUi,
            "Data surface owner",
            "The dense data surface renders counts / timings on tabular numerals and truncates rows with a tooltip at compact density; missing tabular numerals and a silent clip that destroys meaning degrade honestly",
            "evidence:m5-typography-overflow-data-ui:001",
            vec![
                D::TabularNumeralsMissing,
                D::TypographyScaleDrifted,
                D::ProofStale,
            ],
            vec![type_numeric_clean(), type_tabular_missing()],
            vec![overflow_row_clean(), overflow_meaning_destroyed()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved type-scale and overflow truth, so a raw-value regression, an unstated token or role, and a raw-layout overflow are visible in evidence rather than hidden behind rendering",
            "evidence:m5-typography-overflow-support-export:001",
            vec![
                D::TokenReferenceUnstated,
                D::SemanticRoleUnstated,
                D::ProofStale,
            ],
            vec![
                type_raw_inlined(),
                type_token_unstated(),
                type_role_unstated(),
            ],
            vec![overflow_raw_inlined()],
        ),
    ]
}

fn governance_review() -> M5TypographyOverflowGovernanceReview {
    M5TypographyOverflowGovernanceReview {
        one_readable_type_hierarchy_across_surfaces: true,
        code_and_ui_font_policy_is_stable: true,
        line_height_guards_hold: true,
        tabular_numerals_present_for_numeric_data: true,
        overflow_never_silently_destroys_meaning: true,
        full_meaning_reachable_off_truncation: true,
        zoom_and_density_regressions_caught_before_release: true,
        raw_type_value_drift_caught_before_release: true,
        first_consumers_use_canonical_type_scale: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5TypographyOverflowConsumerProjection {
    M5TypographyOverflowConsumerProjection {
        shell_and_editor_consume_shared_type_scale: true,
        review_consumes_shared_type_scale: true,
        data_consumes_tabular_numeral_policy: true,
        docs_consumes_shared_type_scale: true,
        type_and_layout_meaning_traces_to_single_domain_contract: true,
        support_export_reads_single_typography_source: true,
    }
}

fn proof_freshness() -> M5TypographyOverflowProofFreshness {
    M5TypographyOverflowProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5TypographyOverflowReleasePosture {
    M5TypographyOverflowReleasePosture {
        proof_packet_ref: M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_ARTIFACT_REF.to_owned(),
        foundation_audit_ref: M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_SCHEMA_REF,
        M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_DOC_REF,
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
        M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
        M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 typography-scale, font-stack, and text-overflow registries packet.
pub fn seeded_m5_typography_overflow_registries() -> M5TypographyOverflowRegistriesPacket {
    M5TypographyOverflowRegistriesPacket::new(M5TypographyOverflowRegistriesPacketInput {
        packet_id: M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 typography-scale, font-stack, and text-overflow registries with a canonical title / body / label / code hierarchy, stable UI-sans and code-mono stack selection, line-height guards, tabular numerals for counts / timings / diagnostics, and meaning-preserving overflow / truncation / wrap behavior across the shell, editor, review, docs, data, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5TypographyOverflowVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the editor-UI row is held at Beta pending code-mono stack proof on every deployment
/// line; every row stays visible and every example stays honest.
pub fn seeded_m5_typography_overflow_registries_editor_ui_beta_narrowed(
) -> M5TypographyOverflowRegistriesPacket {
    let mut packet = seeded_m5_typography_overflow_registries();
    packet.packet_id =
        "m5-typography-scale-font-stack-and-overflow-registries:editor-ui-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualFoundationConsumerSurface::EditorUi)
        .expect("editor-ui row present");
    row.qualification = M5VisualFoundationQualificationClass::Beta;
    packet
}

/// Narrowed variant: the data-UI row is narrowed to Preview pending tabular-numeral parity on every dense
/// table; every row stays visible and every example stays honest.
pub fn seeded_m5_typography_overflow_registries_data_ui_preview_narrowed(
) -> M5TypographyOverflowRegistriesPacket {
    let mut packet = seeded_m5_typography_overflow_registries();
    packet.packet_id =
        "m5-typography-scale-font-stack-and-overflow-registries:data-ui-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualFoundationConsumerSurface::DataUi)
        .expect("data-ui row present");
    row.qualification = M5VisualFoundationQualificationClass::Preview;
    packet
}
