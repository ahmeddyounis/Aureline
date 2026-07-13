//! Canonical seed builders for the M5 color-system and semantic-theme-token registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean color and theme-token entries are
//! built so the canonical semantic-role families, the trust-sensitive restricted / remote /
//! collaboration / AI / debug states, the dark / light / high-contrast mode parity, and the
//! non-color-cue fallback are proven across the shell, editor, review, notebook, data, and support
//! surfaces without any color-only meaning, raw-color inlining, mode-parity gap, or theme-role drift.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_COLOR_THEME_REGISTRIES_PACKET_ID: &str =
    "m5-color-system-and-semantic-theme-token-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn color(input: M5ColorEntryResolutionInput) -> M5ResolvedColorEntry {
    resolve_color_entry(input).expect("seed color entry resolves")
}

fn theme(input: M5ThemeTokenEntryResolutionInput) -> M5ResolvedThemeTokenEntry {
    resolve_theme_token_entry(input).expect("seed theme-token entry resolves")
}

fn all_modes() -> Vec<M5ThemeMode> {
    M5ThemeMode::ALL.to_vec()
}

// -- Clean color entries (semantic-role + operational-state grammar across surfaces) ------------

#[allow(clippy::too_many_arguments)]
fn clean_color_base(
    entry_id: &str,
    token_name: &str,
    semantic_role: M5VisualSemanticRole,
    color_role: M5ColorRoleFamily,
    operational_state: M5OperationalStateFamily,
    non_color_cue: M5NonColorCue,
    surface_context: M5ColorRegistrySurfaceContext,
) -> M5ColorEntryResolutionInput {
    M5ColorEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        color_role,
        operational_state,
        non_color_cue,
        surface_context,
        defined_modes: all_modes(),
        meaning_stated_non_color_only: true,
        distinguishable_in_all_modes: true,
        references_canonical_token: true,
        proof_fresh: true,
    }
}

fn color_brand_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:shell:brand",
        "color.brand.primary",
        M5VisualSemanticRole::Brand,
        M5ColorRoleFamily::BrandPalette,
        M5OperationalStateFamily::Brand,
        M5NonColorCue::TextLabel,
        M5ColorRegistrySurfaceContext::Shell,
    ))
}

fn color_interactive_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:shell:interactive",
        "color.interactive.accent",
        M5VisualSemanticRole::Interactive,
        M5ColorRoleFamily::InteractivePalette,
        M5OperationalStateFamily::Interactive,
        M5NonColorCue::BorderTreatment,
        M5ColorRegistrySurfaceContext::Shell,
    ))
}

fn color_neutral_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:shell:neutral",
        "color.neutral.surface",
        M5VisualSemanticRole::Neutral,
        M5ColorRoleFamily::NeutralPalette,
        M5OperationalStateFamily::Neutral,
        M5NonColorCue::TextLabel,
        M5ColorRegistrySurfaceContext::Shell,
    ))
}

fn color_success_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:editor:success",
        "color.status.success",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Success,
        M5NonColorCue::IconGlyph,
        M5ColorRegistrySurfaceContext::Editor,
    ))
}

fn color_info_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:editor:info",
        "color.status.info",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Info,
        M5NonColorCue::IconGlyph,
        M5ColorRegistrySurfaceContext::Editor,
    ))
}

fn color_warning_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:review:warning",
        "color.status.warning",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Warning,
        M5NonColorCue::IconGlyph,
        M5ColorRegistrySurfaceContext::Review,
    ))
}

fn color_danger_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:review:danger",
        "color.status.danger",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Danger,
        M5NonColorCue::IconGlyph,
        M5ColorRegistrySurfaceContext::Review,
    ))
}

fn color_restricted_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:review:restricted",
        "color.state.restricted",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Restricted,
        M5NonColorCue::IconGlyph,
        M5ColorRegistrySurfaceContext::Review,
    ))
}

fn color_insight_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:data:insight",
        "color.state.insight",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Insight,
        M5NonColorCue::ShapePattern,
        M5ColorRegistrySurfaceContext::Data,
    ))
}

fn color_collaboration_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:data:collaboration",
        "color.state.collaboration",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Collaboration,
        M5NonColorCue::ShapePattern,
        M5ColorRegistrySurfaceContext::Data,
    ))
}

fn color_debug_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:data:debug",
        "color.state.debug",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Debug,
        M5NonColorCue::TextLabel,
        M5ColorRegistrySurfaceContext::Data,
    ))
}

fn color_remote_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:notebook:remote",
        "color.state.remote",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Remote,
        M5NonColorCue::BorderTreatment,
        M5ColorRegistrySurfaceContext::Notebook,
    ))
}

fn color_ai_clean() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:notebook:ai",
        "color.state.ai",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Ai,
        M5NonColorCue::ShapePattern,
        M5ColorRegistrySurfaceContext::Notebook,
    ))
}

// -- Degraded color entries ---------------------------------------------------------------------

/// Degraded color entry: the meaning is encoded by color alone.
fn color_color_only() -> M5ResolvedColorEntry {
    let mut input = clean_color_base(
        "color:shell:color-only",
        "color.status.warning",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Warning,
        M5NonColorCue::IconGlyph,
        M5ColorRegistrySurfaceContext::Shell,
    );
    input.meaning_stated_non_color_only = false;
    color(input)
}

/// Degraded color entry: no non-color cue is paired with the hue.
fn color_cue_missing() -> M5ResolvedColorEntry {
    let mut input = clean_color_base(
        "color:editor:cue-missing",
        "color.status.info",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Info,
        M5NonColorCue::IconGlyph,
        M5ColorRegistrySurfaceContext::Editor,
    );
    input.non_color_cue = M5NonColorCue::NoneDisallowed;
    color(input)
}

/// Degraded color entry: the dark / light / high-contrast mode parity is incomplete.
fn color_mode_incomplete() -> M5ResolvedColorEntry {
    let mut input = clean_color_base(
        "color:review:mode-incomplete",
        "color.state.remote",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Remote,
        M5NonColorCue::BorderTreatment,
        M5ColorRegistrySurfaceContext::Review,
    );
    input.defined_modes = vec![M5ThemeMode::Dark, M5ThemeMode::Light];
    color(input)
}

/// Degraded color entry: the state is indistinguishable from another state in at least one mode.
fn color_indistinct() -> M5ResolvedColorEntry {
    let mut input = clean_color_base(
        "color:data:indistinct",
        "color.state.ai",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Ai,
        M5NonColorCue::ShapePattern,
        M5ColorRegistrySurfaceContext::Data,
    );
    input.distinguishable_in_all_modes = false;
    color(input)
}

/// Degraded color entry: a raw color value is inlined instead of tracing to a canonical token.
fn color_raw_inlined() -> M5ResolvedColorEntry {
    let mut input = clean_color_base(
        "color:review:raw-inlined",
        "color.status.danger",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Danger,
        M5NonColorCue::IconGlyph,
        M5ColorRegistrySurfaceContext::Review,
    );
    input.references_canonical_token = false;
    color(input)
}

/// Degraded color entry: the operational state family is unclassified.
fn color_unclassified() -> M5ResolvedColorEntry {
    color(clean_color_base(
        "color:data:unclassified",
        "color.state.unknown",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::StateUnclassified,
        M5NonColorCue::IconGlyph,
        M5ColorRegistrySurfaceContext::Data,
    ))
}

/// Degraded color entry: the canonical token name is unstated.
fn color_token_unstated() -> M5ResolvedColorEntry {
    let mut input = clean_color_base(
        "color:support:token-unstated",
        "  ",
        M5VisualSemanticRole::Status,
        M5ColorRoleFamily::StatusPalette,
        M5OperationalStateFamily::Info,
        M5NonColorCue::IconGlyph,
        M5ColorRegistrySurfaceContext::Data,
    );
    input.token_name = "  ".to_owned();
    color(input)
}

// -- Clean theme-token entries ------------------------------------------------------------------

fn clean_theme_base(
    entry_id: &str,
    token_name: &str,
    theme_token_role: M5ThemeTokenRole,
    semantic_role: M5VisualSemanticRole,
    surface_context: M5ColorRegistrySurfaceContext,
) -> M5ThemeTokenEntryResolutionInput {
    M5ThemeTokenEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        theme_token_role,
        semantic_role,
        surface_context,
        defined_modes: all_modes(),
        references_canonical_token: true,
        role_stable_across_surfaces: true,
        proof_fresh: true,
    }
}

fn theme_surface_clean() -> M5ResolvedThemeTokenEntry {
    theme(clean_theme_base(
        "theme:shell:surface",
        "theme.surface.base",
        M5ThemeTokenRole::SurfaceRole,
        M5VisualSemanticRole::Neutral,
        M5ColorRegistrySurfaceContext::Shell,
    ))
}

fn theme_text_clean() -> M5ResolvedThemeTokenEntry {
    theme(clean_theme_base(
        "theme:editor:text",
        "theme.text.primary",
        M5ThemeTokenRole::TextRole,
        M5VisualSemanticRole::Neutral,
        M5ColorRegistrySurfaceContext::Editor,
    ))
}

fn theme_border_clean() -> M5ResolvedThemeTokenEntry {
    theme(clean_theme_base(
        "theme:review:border",
        "theme.border.divider",
        M5ThemeTokenRole::BorderRole,
        M5VisualSemanticRole::Neutral,
        M5ColorRegistrySurfaceContext::Review,
    ))
}

fn theme_status_clean() -> M5ResolvedThemeTokenEntry {
    theme(clean_theme_base(
        "theme:data:status",
        "theme.status.accent",
        M5ThemeTokenRole::StatusRole,
        M5VisualSemanticRole::Status,
        M5ColorRegistrySurfaceContext::Data,
    ))
}

fn theme_pair_clean() -> M5ResolvedThemeTokenEntry {
    theme(clean_theme_base(
        "theme:notebook:pair",
        "theme.pair.dlhc",
        M5ThemeTokenRole::ThemePairDarkLightHighContrast,
        M5VisualSemanticRole::Neutral,
        M5ColorRegistrySurfaceContext::Notebook,
    ))
}

// -- Degraded theme-token entries ---------------------------------------------------------------

/// Degraded theme-token entry: a raw hex value is inlined on the surface instead of a token.
fn theme_raw_hex() -> M5ResolvedThemeTokenEntry {
    theme(clean_theme_base(
        "theme:shell:raw-hex",
        "theme.surface.raw",
        M5ThemeTokenRole::RawHexInSurfaceDisallowed,
        M5VisualSemanticRole::Neutral,
        M5ColorRegistrySurfaceContext::Shell,
    ))
}

/// Degraded theme-token entry: the dark / light / high-contrast theme pair is incomplete.
fn theme_pair_incomplete() -> M5ResolvedThemeTokenEntry {
    let mut input = clean_theme_base(
        "theme:review:pair-incomplete",
        "theme.surface.partial",
        M5ThemeTokenRole::SurfaceRole,
        M5VisualSemanticRole::Neutral,
        M5ColorRegistrySurfaceContext::Review,
    );
    input.defined_modes = vec![M5ThemeMode::Dark];
    theme(input)
}

/// Degraded theme-token entry: the theme-token role drifted across surfaces.
fn theme_role_drift() -> M5ResolvedThemeTokenEntry {
    let mut input = clean_theme_base(
        "theme:data:role-drift",
        "theme.text.secondary",
        M5ThemeTokenRole::TextRole,
        M5VisualSemanticRole::Neutral,
        M5ColorRegistrySurfaceContext::Data,
    );
    input.role_stable_across_surfaces = false;
    theme(input)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5ColorThemeConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5VisualFoundationDowngradeTrigger>,
    color_entries: Vec<M5ResolvedColorEntry>,
    theme_token_entries: Vec<M5ResolvedThemeTokenEntry>,
) -> M5ColorThemeRegistriesRow {
    M5ColorThemeRegistriesRow {
        consumer_surface,
        qualification: M5VisualFoundationQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5VisualFoundationDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5VisualFoundationRequiredLabel::Identity,
            M5VisualFoundationRequiredLabel::SemanticRole,
            M5VisualFoundationRequiredLabel::TokenReference,
            M5VisualFoundationRequiredLabel::ThemeVariant,
        ],
        accessibility_routes: M5VisualFoundationAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5ColorRegistryAnatomyPart::ALL.to_vec(),
        export_fields: M5ColorRegistryExportField::ALL.to_vec(),
        downgrade_triggers,
        color_entries,
        theme_token_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_COLOR_THEME_REGISTRIES_SCHEMA_REF,
            M5_COLOR_SYSTEM_SCHEMA_REF,
        ]),
        status_meaning_relies_on_color_alone: false,
        raw_color_value_inlined_instead_of_token: false,
        operational_state_indistinguishable_across_modes: false,
        theme_mode_parity_incomplete: false,
    }
}

fn registry_rows() -> Vec<M5ColorThemeRegistriesRow> {
    use M5VisualFoundationConsumerSurface as C;
    use M5VisualFoundationDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell consumes the canonical brand / interactive / neutral palettes and pairs every hue with a non-color cue; a color-only entry and a raw-hex theme token degrade honestly instead of reading as a clean pass",
            "evidence:m5-color-theme-shell-ui:001",
            vec![
                D::StatusOrTrustCollapsedToColorOnly,
                D::TokenReferenceUnstated,
                D::ProofStale,
            ],
            vec![
                color_brand_clean(),
                color_interactive_clean(),
                color_neutral_clean(),
                color_color_only(),
            ],
            vec![theme_surface_clean(), theme_raw_hex()],
        ),
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor consumes the canonical success / info status colors with an icon cue across dark, light, and high-contrast; an entry that drops its non-color cue degrades honestly",
            "evidence:m5-color-theme-editor-ui:001",
            vec![
                D::StatusOrTrustCollapsedToColorOnly,
                D::ThemePairIncomplete,
                D::ProofStale,
            ],
            vec![
                color_success_clean(),
                color_info_clean(),
                color_cue_missing(),
            ],
            vec![theme_text_clean()],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review surface keeps warning / danger / restricted colors distinct in every mode and traces every token to the canonical color system; a mode-parity gap and a raw-color inlining degrade honestly",
            "evidence:m5-color-theme-review-ui:001",
            vec![
                D::ThemePairIncomplete,
                D::TokenReferenceUnstated,
                D::ProofStale,
            ],
            vec![
                color_warning_clean(),
                color_danger_clean(),
                color_restricted_clean(),
                color_mode_incomplete(),
                color_raw_inlined(),
            ],
            vec![theme_border_clean(), theme_pair_incomplete()],
        ),
        base_row(
            C::DataUi,
            "Data surface owner",
            "The data surface keeps insight / collaboration / debug states distinguishable with shape and label cues; an indistinguishable-across-modes entry, an unclassified state, and a drifted theme role all degrade honestly",
            "evidence:m5-color-theme-data-ui:001",
            vec![
                D::StatusOrTrustCollapsedToColorOnly,
                D::SemanticRoleUnstated,
                D::ProofStale,
            ],
            vec![
                color_insight_clean(),
                color_collaboration_clean(),
                color_debug_clean(),
                color_indistinct(),
                color_unclassified(),
            ],
            vec![theme_status_clean(), theme_role_drift()],
        ),
        base_row(
            C::DocsUi,
            "Docs / notebook surface owner",
            "The docs and notebook surfaces keep the trust-sensitive remote and AI states distinct in dark, light, and high-contrast with border and shape cues, tracing each to the canonical theme pair",
            "evidence:m5-color-theme-docs-ui:001",
            vec![
                D::StatusOrTrustCollapsedToColorOnly,
                D::ThemePairIncomplete,
                D::ProofStale,
            ],
            vec![color_remote_clean(), color_ai_clean()],
            vec![theme_pair_clean()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved color and theme truth, so a raw-color regression or an unstated token is visible in evidence rather than hidden behind hue",
            "evidence:m5-color-theme-support-export:001",
            vec![
                D::TokenReferenceUnstated,
                D::StatusOrTrustCollapsedToColorOnly,
                D::ProofStale,
            ],
            vec![color_neutral_clean(), color_token_unstated()],
            vec![theme_surface_clean()],
        ),
    ]
}

fn governance_review() -> M5ColorThemeGovernanceReview {
    M5ColorThemeGovernanceReview {
        color_registry_names_token_role_and_state: true,
        brand_interactive_neutral_status_stay_distinct: true,
        status_meaning_never_relies_on_color_alone: true,
        every_color_entry_covers_all_theme_modes: true,
        trust_sensitive_states_distinguishable_in_every_mode: true,
        theme_tokens_name_stable_role_not_raw_hex: true,
        theme_tokens_cover_dark_light_high_contrast_pair: true,
        raw_color_drift_caught_before_release: true,
        first_consumers_use_canonical_families: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5ColorThemeConsumerProjection {
    M5ColorThemeConsumerProjection {
        shell_consumes_shared_registries: true,
        editor_consumes_shared_registries: true,
        review_consumes_shared_registries: true,
        notebook_and_data_consume_shared_registries: true,
        color_meaning_traces_to_single_domain_contract: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5ColorThemeProofFreshness {
    M5ColorThemeProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5ColorThemeReleasePosture {
    M5ColorThemeReleasePosture {
        proof_packet_ref: M5_COLOR_THEME_REGISTRIES_ARTIFACT_REF.to_owned(),
        foundation_audit_ref: M5_COLOR_THEME_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_COLOR_THEME_REGISTRIES_SCHEMA_REF,
        M5_COLOR_THEME_REGISTRIES_DOC_REF,
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
        M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
        M5_COLOR_SYSTEM_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 color-system and semantic-theme-token registries packet.
pub fn seeded_m5_color_theme_registries() -> M5ColorThemeRegistriesPacket {
    M5ColorThemeRegistriesPacket::new(M5ColorThemeRegistriesPacketInput {
        packet_id: M5_COLOR_THEME_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 color-system and semantic-theme-token registries with dark / light / high-contrast parity, non-color-only meaning, explicit operational-state mappings for brand/interactive/neutral/success/warning/danger/info/insight and the trust-sensitive restricted/remote/collaboration/ai/debug states, and canonical-token tracing across shell, editor, review, notebook, data, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5ColorThemeVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the shell-UI row is held at Beta pending color-parity proof on every deployment
/// line; every row stays visible and every example stays honest.
pub fn seeded_m5_color_theme_registries_shell_ui_beta_narrowed() -> M5ColorThemeRegistriesPacket {
    let mut packet = seeded_m5_color_theme_registries();
    packet.packet_id =
        "m5-color-system-and-semantic-theme-token-registries:shell-ui-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualFoundationConsumerSurface::ShellUi)
        .expect("shell-ui row present");
    row.qualification = M5VisualFoundationQualificationClass::Beta;
    packet
}

/// Narrowed variant: the data-UI row is narrowed to Preview pending state-distinguishability parity on
/// every surface; every row stays visible and every example stays honest.
pub fn seeded_m5_color_theme_registries_data_ui_preview_narrowed() -> M5ColorThemeRegistriesPacket {
    let mut packet = seeded_m5_color_theme_registries();
    packet.packet_id =
        "m5-color-system-and-semantic-theme-token-registries:data-ui-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5VisualFoundationConsumerSurface::DataUi)
        .expect("data-ui row present");
    row.qualification = M5VisualFoundationQualificationClass::Preview;
    packet
}
