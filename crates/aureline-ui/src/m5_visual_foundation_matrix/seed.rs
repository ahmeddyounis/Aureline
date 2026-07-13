//! Canonical seed builders for the frozen M5 visual-foundation matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix, the
//! artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical visual-foundation matrix.
pub const M5_VISUAL_FOUNDATION_MATRIX_PACKET_ID: &str = "m5-visual-foundations:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every family must be able to show.
fn mandatory_labels() -> Vec<M5VisualFoundationRequiredLabel> {
    M5VisualFoundationRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a family carries.
fn labels_with(extra: &[M5VisualFoundationRequiredLabel]) -> Vec<M5VisualFoundationRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every family filled in and every family-specific vocabulary left
/// empty for the caller to populate.
fn base_row(
    foundation_family: M5VisualFoundationFamily,
    qualification: M5VisualFoundationQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5VisualFoundationRow {
    M5VisualFoundationRow {
        foundation_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5VisualFoundationSurfaceFamily::ALL.to_vec(),
        deployment_lines: M5VisualFoundationDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        color_roles: vec![],
        theme_token_roles: vec![],
        syntax_roles: vec![],
        diff_roles: vec![],
        chart_roles: vec![],
        typography_roles: vec![],
        geometry_roles: vec![],
        hit_target_rules: vec![],
        degraded_reasons: M5VisualFoundationDegradedReason::ALL.to_vec(),
        accessibility_routes: M5VisualFoundationAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5VisualFoundationConsumerSurface::SupportExport,
            M5VisualFoundationConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5VisualFoundationDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        collapses_status_or_trust_into_color_only: false,
        lets_syntax_or_diff_palette_collide_with_diagnostics: false,
        shrinks_hit_target_below_supported_minimum: false,
        lets_chart_meaning_depend_on_color_alone: false,
        forks_local_spacing_or_elevation_from_shared_geometry: false,
    }
}

fn foundation_rows() -> Vec<M5VisualFoundationRow> {
    use M5ChartTokenRole as CH;
    use M5ColorRoleFamily as CO;
    use M5DiffTokenRole as DI;
    use M5GeometryRole as GE;
    use M5HitTargetRule as HT;
    use M5SyntaxTokenRole as SY;
    use M5ThemeTokenRole as TH;
    use M5TypographyRole as TY;
    use M5VisualFoundationConsumerSurface as C;
    use M5VisualFoundationDowngradeTrigger as D;
    use M5VisualFoundationFamily as F;
    use M5VisualFoundationQualificationClass as Q;
    use M5VisualFoundationRequiredLabel as L;
    use M5VisualSemanticRole as R;

    let mut rows = Vec::new();

    // 1. Color system.
    let mut row = base_row(
        F::ColorSystem,
        Q::Stable,
        "Design-system foundations owner",
        "One color system with distinct brand, interactive, neutral, and status palettes, each always paired with a non-color cue so status and trust meaning never collapse into hue alone",
        "evidence:m5-color-system-parity:001",
        &[M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF, M5_COLOR_SYSTEM_SCHEMA_REF, M5_DESIGN_SYSTEM_FOUNDATIONS_SCHEMA_REF],
    );
    row.color_roles = CO::ALL.to_vec();
    row.semantic_roles = vec![R::Brand, R::Interactive, R::Neutral, R::Status];
    row.required_labels = labels_with(&[L::ContrastPairing]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::ReviewUi,
        C::DataUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::StatusOrTrustCollapsedToColorOnly,
        D::SemanticRoleUnstated,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Semantic theme tokens.
    let mut row = base_row(
        F::SemanticThemeToken,
        Q::Stable,
        "Design-system foundations owner",
        "One semantic theme-token set whose surface, text, border, and status roles stay stable across a complete dark / light / high-contrast pair, bound to the appearance-session and design-system foundations rather than raw hex",
        "evidence:m5-semantic-theme-token-parity:001",
        &[M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF, M5_COLOR_SYSTEM_SCHEMA_REF, M5_DESIGN_SYSTEM_FOUNDATION_PACKAGE_SCHEMA_REF],
    );
    row.theme_token_roles = TH::ALL.to_vec();
    row.semantic_roles = vec![R::Neutral, R::Interactive, R::Status];
    row.required_labels = labels_with(&[L::ThemeVariant]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::ReviewUi,
        C::SettingsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ThemePairIncomplete,
        D::SemanticRoleUnstated,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Syntax tokens.
    let mut row = base_row(
        F::SyntaxToken,
        Q::Stable,
        "Editor surface owner",
        "One syntax-highlighting token set naming keyword, string, comment, and identifier scopes that stay distinct from the diagnostics palette so a syntax color never reads as an error",
        "evidence:m5-syntax-token-parity:001",
        &[M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF, M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF],
    );
    row.syntax_roles = SY::ALL.to_vec();
    row.semantic_roles = vec![R::Syntax, R::Neutral];
    row.required_labels = labels_with(&[L::ContrastPairing]);
    row.consumer_surfaces = vec![
        C::EditorUi,
        C::ReviewUi,
        C::DocsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SyntaxOrDiffPaletteCollidedWithDiagnostics,
        D::SemanticRoleUnstated,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Diff tokens.
    let mut row = base_row(
        F::DiffToken,
        Q::Stable,
        "Review surface owner",
        "One diff token set naming addition, removal, context, and moved regions that stay distinct from the diagnostics palette and always pair color with a glyph so a diff never depends on hue alone",
        "evidence:m5-diff-token-parity:001",
        &[M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF, M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF],
    );
    row.diff_roles = DI::ALL.to_vec();
    row.semantic_roles = vec![R::Diff, R::Neutral];
    row.required_labels = labels_with(&[L::ContrastPairing]);
    row.consumer_surfaces = vec![
        C::ReviewUi,
        C::EditorUi,
        C::DataUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::SyntaxOrDiffPaletteCollidedWithDiagnostics,
        D::StatusOrTrustCollapsedToColorOnly,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Chart tokens.
    let mut row = base_row(
        F::ChartToken,
        Q::Stable,
        "Data surface owner",
        "One chart token set naming categorical, sequential, and diverging scales that always pair color with a shape or label and meet accessible contrast so chart meaning never depends on color alone",
        "evidence:m5-chart-token-parity:001",
        &[M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF, M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF],
    );
    row.chart_roles = CH::ALL.to_vec();
    row.semantic_roles = vec![R::Chart, R::Neutral];
    row.required_labels = labels_with(&[L::ContrastPairing]);
    row.consumer_surfaces = vec![
        C::DataUi,
        C::ReviewUi,
        C::DocsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::ChartMeaningDependedOnColorAlone,
        D::SemanticRoleUnstated,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Typography.
    let mut row = base_row(
        F::Typography,
        Q::Stable,
        "Design-system foundations owner",
        "One typography system naming display and body scales, code and UI font stacks, and tabular numerals so type scale, line-height, and font stacks stay stable across every surface",
        "evidence:m5-typography-parity:001",
        &[M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF, M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF],
    );
    row.typography_roles = TY::ALL.to_vec();
    row.semantic_roles = vec![R::Neutral, R::Brand];
    row.required_labels = labels_with(&[L::DensityContext]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::DocsUi,
        C::DataUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::TypographyScaleDrifted,
        D::FontStackUnstable,
        D::TabularNumeralsMissing,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. Spacing / sizing / radii / elevation.
    let mut row = base_row(
        F::SpacingSizingRadiiElevation,
        Q::Stable,
        "Design-system foundations owner",
        "One geometry system naming spacing, sizing, radius, and elevation steps that stay density-aware and machine-readable so no surface forks its own local spacing or elevation",
        "evidence:m5-geometry-parity:001",
        &[M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF, M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF],
    );
    row.geometry_roles = GE::ALL.to_vec();
    row.semantic_roles = vec![R::Neutral];
    row.required_labels = labels_with(&[L::DensityContext]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::ReviewUi,
        C::DataUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::LocalGeometryForkedFromFoundation,
        D::SemanticRoleUnstated,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 8. Hit target.
    let mut row = base_row(
        F::HitTarget,
        Q::Stable,
        "Accessibility foundations owner",
        "One hit-target baseline naming comfortable, compact, and coarse-pointer minima and inter-target spacing so an interactive target never shrinks below its supported minimum under compact density",
        "evidence:m5-hit-target-parity:001",
        &[M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF, M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF],
    );
    row.hit_target_rules = HT::ALL.to_vec();
    row.semantic_roles = vec![R::Neutral, R::Interactive];
    row.required_labels = labels_with(&[L::DensityContext]);
    row.consumer_surfaces = vec![
        C::ShellUi,
        C::EditorUi,
        C::ReviewUi,
        C::SettingsUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::HitTargetShrunkBelowMinimum,
        D::SemanticRoleUnstated,
        D::TokenReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5VisualFoundationGovernanceReview {
    M5VisualFoundationGovernanceReview {
        brand_interactive_neutral_status_stay_distinct: true,
        status_meaning_never_color_alone: true,
        syntax_diff_chart_never_collide_with_diagnostics: true,
        chart_meaning_never_color_alone: true,
        semantic_theme_roles_bind_to_appearance_session: true,
        theme_pairs_cover_dark_light_high_contrast: true,
        typography_scale_and_line_height_stable: true,
        tabular_numerals_available_for_numeric_data: true,
        code_and_ui_font_stacks_stable: true,
        spacing_sizing_radii_elevation_density_aware: true,
        geometry_rules_machine_readable: true,
        hit_targets_never_below_supported_minimum: true,
        no_surface_invents_local_geometry_or_color_meaning: true,
        every_family_declares_deployment_lines: true,
        every_family_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_visual_vocabulary: true,
    }
}

fn consumer_projection() -> M5VisualFoundationConsumerProjection {
    M5VisualFoundationConsumerProjection {
        shell_and_editor_consume_shared_visual_foundation: true,
        review_and_data_consume_shared_token_families: true,
        docs_consume_shared_typography_and_geometry: true,
        syntax_diff_chart_consumers_read_single_token_source: true,
        appearance_session_binds_to_shared_theme_tokens: true,
        support_export_reads_single_visual_foundation_source: true,
    }
}

fn proof_freshness() -> M5VisualFoundationProofFreshness {
    M5VisualFoundationProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5VisualFoundationReleasePosture {
    M5VisualFoundationReleasePosture {
        proof_packet_ref: M5_VISUAL_FOUNDATION_ARTIFACT_REF.to_owned(),
        foundation_audit_ref: M5_VISUAL_FOUNDATION_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
        M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
        M5_COLOR_SYSTEM_SCHEMA_REF,
        M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF,
        M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF,
        M5_DESIGN_SYSTEM_FOUNDATIONS_SCHEMA_REF,
        M5_DESIGN_SYSTEM_FOUNDATION_PACKAGE_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 visual-foundation matrix packet.
pub fn seeded_m5_visual_foundation_matrix() -> M5VisualFoundationMatrixPacket {
    M5VisualFoundationMatrixPacket::new(M5VisualFoundationMatrixPacketInput {
        packet_id: M5_VISUAL_FOUNDATION_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 color-system, semantic-theme-token, syntax / diff / chart-token, typography, and spacing / sizing / radii / elevation visual-foundation matrix"
                .to_owned(),
        foundation_rows: foundation_rows(),
        vocabulary_set: M5VisualFoundationVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: typography is held at Beta because font-stack and tabular-numeral parity is not yet
/// proven across every deployment line; every family stays visible.
pub fn seeded_m5_visual_foundation_matrix_typography_beta_narrowed(
) -> M5VisualFoundationMatrixPacket {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.packet_id = "m5-visual-foundations:typography-beta:0001".to_owned();
    let row = packet
        .foundation_rows
        .iter_mut()
        .find(|row| row.foundation_family == M5VisualFoundationFamily::Typography)
        .expect("typography row present");
    row.qualification = M5VisualFoundationQualificationClass::Beta;
    packet
}

/// Narrowed variant: chart tokens are narrowed to Preview pending accessible-contrast parity across every
/// deployment line; every family stays visible.
pub fn seeded_m5_visual_foundation_matrix_chart_token_preview_narrowed(
) -> M5VisualFoundationMatrixPacket {
    let mut packet = seeded_m5_visual_foundation_matrix();
    packet.packet_id = "m5-visual-foundations:chart-token-preview:0001".to_owned();
    let row = packet
        .foundation_rows
        .iter_mut()
        .find(|row| row.foundation_family == M5VisualFoundationFamily::ChartToken)
        .expect("chart-token row present");
    row.qualification = M5VisualFoundationQualificationClass::Preview;
    packet
}
