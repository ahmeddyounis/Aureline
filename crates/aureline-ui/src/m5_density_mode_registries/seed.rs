//! Canonical seed builders for the M5 density-mode registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean density-scale and persistence entries
//! are built so the canonical comfortable / standard / compact row and control heights, tab / chip spacing,
//! panel padding, gutter spacing, the list / tree / table / tab / panel / editor / inspector surface-element
//! coverage, and the profile-scope persistence with explained local overrides are proven across the shell,
//! editor, review, notebook, data, and support surfaces without any private scale, below-minimum hit target,
//! information-architecture change, or silent density switch.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_DENSITY_MODE_REGISTRIES_PACKET_ID: &str = "m5-density-mode-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn scale(input: M5DensityScaleEntryResolutionInput) -> M5ResolvedDensityScaleEntry {
    resolve_density_scale_entry(input).expect("seed density-scale entry resolves")
}

fn persistence(
    input: M5DensityPersistenceEntryResolutionInput,
) -> M5ResolvedDensityPersistenceEntry {
    resolve_density_persistence_entry(input).expect("seed density-persistence entry resolves")
}

fn all_elements() -> Vec<M5DensitySurfaceElement> {
    M5DensitySurfaceElement::ALL.to_vec()
}

// -- Clean density-scale entries (canonical tokens bound to the shared registry) -------------------

fn clean_scale_base(
    entry_id: &str,
    token_name: &str,
    mode: M5DensityMode,
    mode_role: M5DensityModeRole,
    surface_context: M5DensitySurfaceContext,
) -> M5DensityScaleEntryResolutionInput {
    let canonical = mode.canonical_scale();
    M5DensityScaleEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role: M5ShellGeometryRole::Density,
        density_mode_role: mode_role,
        density_mode: mode,
        surface_context,
        row_height_px: canonical.row_height_px,
        control_height_px: canonical.control_height_px,
        tab_chip_spacing_px: canonical.tab_chip_spacing_px,
        panel_padding_px: canonical.panel_padding_px,
        gutter_spacing_px: canonical.gutter_spacing_px,
        surface_elements: all_elements(),
        changes_information_architecture: false,
        preserves_command_focus_and_trust: true,
        proof_fresh: true,
    }
}

fn scale_comfortable_shell() -> M5ResolvedDensityScaleEntry {
    scale(clean_scale_base(
        "scale:shell:comfortable",
        "shell.density.comfortable.scale",
        M5DensityMode::Comfortable,
        M5DensityModeRole::ComfortableMode,
        M5DensitySurfaceContext::Shell,
    ))
}

fn scale_standard_editor() -> M5ResolvedDensityScaleEntry {
    scale(clean_scale_base(
        "scale:editor:standard",
        "shell.density.standard.scale",
        M5DensityMode::Standard,
        M5DensityModeRole::StandardMode,
        M5DensitySurfaceContext::Editor,
    ))
}

fn scale_compact_review() -> M5ResolvedDensityScaleEntry {
    scale(clean_scale_base(
        "scale:review:compact",
        "shell.density.compact.scale",
        M5DensityMode::Compact,
        M5DensityModeRole::CompactMode,
        M5DensitySurfaceContext::Review,
    ))
}

fn scale_standard_notebook() -> M5ResolvedDensityScaleEntry {
    scale(clean_scale_base(
        "scale:notebook:standard",
        "shell.density.standard.scale",
        M5DensityMode::Standard,
        M5DensityModeRole::PresentationOnlyChange,
        M5DensitySurfaceContext::Notebook,
    ))
}

fn scale_compact_data() -> M5ResolvedDensityScaleEntry {
    scale(clean_scale_base(
        "scale:data:compact",
        "shell.density.compact.scale",
        M5DensityMode::Compact,
        M5DensityModeRole::CompactMode,
        M5DensitySurfaceContext::Data,
    ))
}

fn scale_comfortable_support() -> M5ResolvedDensityScaleEntry {
    scale(clean_scale_base(
        "scale:support:comfortable",
        "shell.density.comfortable.scale",
        M5DensityMode::Comfortable,
        M5DensityModeRole::PreservesInformationArchitecture,
        M5DensitySurfaceContext::Shell,
    ))
}

// -- Degraded density-scale entries -------------------------------------------------------------

/// Degraded density-scale entry: the declared scale drifts from the canonical density tokens (a private
/// scale an extension invented instead of resolving the shared tokens).
fn scale_outside_canonical() -> M5ResolvedDensityScaleEntry {
    let mut input = clean_scale_base(
        "scale:shell:private-scale",
        "shell.density.compact.scale",
        M5DensityMode::Compact,
        M5DensityModeRole::CompactMode,
        M5DensitySurfaceContext::Shell,
    );
    // A private scale that stays above the hit-target minimum but does not match the canonical tokens.
    input.row_height_px = 26;
    input.control_height_px = 30;
    input.tab_chip_spacing_px = 5;
    input.panel_padding_px = 10;
    input.gutter_spacing_px = 10;
    scale(input)
}

/// Degraded density-scale entry: a control height below the supported minimum shrinks the hit target.
fn scale_below_minimum() -> M5ResolvedDensityScaleEntry {
    let mut input = clean_scale_base(
        "scale:editor:below-minimum",
        "shell.density.compact.scale",
        M5DensityMode::Compact,
        M5DensityModeRole::CompactMode,
        M5DensitySurfaceContext::Editor,
    );
    // A control height of 20 px falls below the 28 px supported minimum.
    input.control_height_px = 20;
    scale(input)
}

/// Degraded density-scale entry: the density change rearranges information architecture.
fn scale_changes_information_architecture() -> M5ResolvedDensityScaleEntry {
    let mut input = clean_scale_base(
        "scale:review:changes-ia",
        "shell.density.standard.scale",
        M5DensityMode::Standard,
        M5DensityModeRole::StandardMode,
        M5DensitySurfaceContext::Review,
    );
    input.changes_information_architecture = true;
    scale(input)
}

/// Degraded density-scale entry: the density change alters command meaning, focus order, or trust.
fn scale_changes_command_focus() -> M5ResolvedDensityScaleEntry {
    let mut input = clean_scale_base(
        "scale:data:changes-command",
        "shell.density.compact.scale",
        M5DensityMode::Compact,
        M5DensityModeRole::CompactMode,
        M5DensitySurfaceContext::Data,
    );
    input.preserves_command_focus_and_trust = false;
    scale(input)
}

/// Degraded density-scale entry: the surface-element coverage is incomplete.
fn scale_element_incomplete() -> M5ResolvedDensityScaleEntry {
    let mut input = clean_scale_base(
        "scale:notebook:element-incomplete",
        "shell.density.standard.scale",
        M5DensityMode::Standard,
        M5DensityModeRole::StandardMode,
        M5DensitySurfaceContext::Notebook,
    );
    input.surface_elements = vec![
        M5DensitySurfaceElement::List,
        M5DensitySurfaceElement::Tree,
        M5DensitySurfaceElement::Table,
    ];
    scale(input)
}

/// Degraded density-scale entry: the canonical registry token name is unstated.
fn scale_token_unstated() -> M5ResolvedDensityScaleEntry {
    let mut input = clean_scale_base(
        "scale:support:token-unstated",
        "  ",
        M5DensityMode::Comfortable,
        M5DensityModeRole::ComfortableMode,
        M5DensitySurfaceContext::Shell,
    );
    input.token_name = "  ".to_owned();
    scale(input)
}

// -- Clean density-persistence entries ----------------------------------------------------------

fn clean_persistence_base(
    entry_id: &str,
    token_name: &str,
    persistence_scope: M5DensityPersistenceScope,
    override_reason: M5DensityOverrideReason,
    surface_context: M5DensitySurfaceContext,
) -> M5DensityPersistenceEntryResolutionInput {
    M5DensityPersistenceEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        token_name: token_name.to_owned(),
        density_mode_role: M5DensityModeRole::PreservesInformationArchitecture,
        semantic_role: M5ShellGeometryRole::Density,
        persistence_scope,
        override_reason,
        surface_context,
        switched_silently_by_provider_theme_or_workflow: false,
        proof_fresh: true,
    }
}

fn persistence_profile_shell() -> M5ResolvedDensityPersistenceEntry {
    persistence(clean_persistence_base(
        "persistence:shell:profile",
        "shell.density.persistence.profile",
        M5DensityPersistenceScope::ProfileScoped,
        M5DensityOverrideReason::NotOverridden,
        M5DensitySurfaceContext::Shell,
    ))
}

fn persistence_explained_presentation() -> M5ResolvedDensityPersistenceEntry {
    persistence(clean_persistence_base(
        "persistence:editor:presentation-viewer",
        "shell.density.persistence.presentation_override",
        M5DensityPersistenceScope::ExplainedLocalOverride,
        M5DensityOverrideReason::PresentationViewer,
        M5DensitySurfaceContext::Editor,
    ))
}

fn persistence_explained_accessibility() -> M5ResolvedDensityPersistenceEntry {
    persistence(clean_persistence_base(
        "persistence:review:accessibility-viewer",
        "shell.density.persistence.accessibility_override",
        M5DensityPersistenceScope::ExplainedLocalOverride,
        M5DensityOverrideReason::AccessibilityViewer,
        M5DensitySurfaceContext::Review,
    ))
}

fn persistence_profile_settings() -> M5ResolvedDensityPersistenceEntry {
    persistence(clean_persistence_base(
        "persistence:settings:profile",
        "shell.density.persistence.profile",
        M5DensityPersistenceScope::ProfileScoped,
        M5DensityOverrideReason::NotOverridden,
        M5DensitySurfaceContext::Notebook,
    ))
}

// -- Degraded density-persistence entries -------------------------------------------------------

/// Degraded persistence entry: the density switched silently because a provider, theme, or workflow changed.
fn persistence_silent_switch() -> M5ResolvedDensityPersistenceEntry {
    let mut input = clean_persistence_base(
        "persistence:shell:silent-switch",
        "shell.density.persistence.profile",
        M5DensityPersistenceScope::ProfileScoped,
        M5DensityOverrideReason::NotOverridden,
        M5DensitySurfaceContext::Shell,
    );
    input.switched_silently_by_provider_theme_or_workflow = true;
    persistence(input)
}

/// Degraded persistence entry: a local override that is not explicitly explained.
fn persistence_unexplained_override() -> M5ResolvedDensityPersistenceEntry {
    persistence(clean_persistence_base(
        "persistence:review:unexplained-override",
        "shell.density.persistence.local_override",
        M5DensityPersistenceScope::ExplainedLocalOverride,
        M5DensityOverrideReason::UnexplainedDisallowed,
        M5DensitySurfaceContext::Review,
    ))
}

/// Degraded persistence entry: the persistence scope is unclassified.
fn persistence_scope_unclassified() -> M5ResolvedDensityPersistenceEntry {
    persistence(clean_persistence_base(
        "persistence:data:scope-unclassified",
        "shell.density.persistence.unknown",
        M5DensityPersistenceScope::ScopeUnclassified,
        M5DensityOverrideReason::NotOverridden,
        M5DensitySurfaceContext::Data,
    ))
}

/// Degraded persistence entry: the canonical registry token name is unstated.
fn persistence_token_unstated() -> M5ResolvedDensityPersistenceEntry {
    let mut input = clean_persistence_base(
        "persistence:support:token-unstated",
        "  ",
        M5DensityPersistenceScope::ProfileScoped,
        M5DensityOverrideReason::NotOverridden,
        M5DensitySurfaceContext::Shell,
    );
    input.token_name = "  ".to_owned();
    persistence(input)
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5DensityModeRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5ShellGeometryDowngradeTrigger>,
    density_scale_entries: Vec<M5ResolvedDensityScaleEntry>,
    density_persistence_entries: Vec<M5ResolvedDensityPersistenceEntry>,
) -> M5DensityModeRegistriesRow {
    M5DensityModeRegistriesRow {
        consumer_surface,
        qualification: M5ShellGeometryQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5ShellGeometryDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5ShellGeometryRequiredLabel::Identity,
            M5ShellGeometryRequiredLabel::SemanticRole,
            M5ShellGeometryRequiredLabel::RegistryReference,
            M5ShellGeometryRequiredLabel::DensityMode,
        ],
        accessibility_routes: M5ShellGeometryAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5DensityRegistryAnatomyPart::ALL.to_vec(),
        export_fields: M5DensityRegistryExportField::ALL.to_vec(),
        downgrade_triggers,
        density_scale_entries,
        density_persistence_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DENSITY_MODE_REGISTRIES_SCHEMA_REF,
            M5_DENSITY_MODE_SCHEMA_REF,
        ]),
        density_change_alters_information_architecture: false,
        density_change_alters_command_focus_or_trust: false,
        shrinks_hit_target_below_supported_minimum: false,
        silently_switches_density_outside_profile_scope: false,
    }
}

fn registry_rows() -> Vec<M5DensityModeRegistriesRow> {
    use M5ShellGeometryConsumerSurface as C;
    use M5ShellGeometryDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves the comfortable density scale from the shared registry and persists the choice at profile scope; a private per-widget scale and a silent provider-driven density switch degrade honestly instead of reading as a clean pass",
            "evidence:m5-density-mode-shell-ui:001",
            vec![
                D::MetricCopiedByHandAcrossPackages,
                D::DensityChangedCommandOrFocusOrTrust,
                D::ProofStale,
            ],
            vec![scale_comfortable_shell(), scale_outside_canonical()],
            vec![persistence_profile_shell(), persistence_silent_switch()],
        ),
        base_row(
            C::EditorUi,
            "Editor surface owner",
            "The editor resolves the standard density scale and keeps control hit targets above their 28 px minimum at high zoom; a control height below the supported minimum degrades honestly, and an explained presentation-viewer override is allowed",
            "evidence:m5-density-mode-editor-ui:001",
            vec![
                D::HitTargetShrankBelowMinimum,
                D::DensityModeUnstated,
                D::ProofStale,
            ],
            vec![scale_standard_editor(), scale_below_minimum()],
            vec![persistence_explained_presentation()],
        ),
        base_row(
            C::ReviewUi,
            "Review surface owner",
            "The review surface resolves the compact density scale and keeps command meaning and focus order unchanged; a density change that would rearrange information architecture and an unexplained local override both degrade honestly, while an accessibility-viewer override is allowed",
            "evidence:m5-density-mode-review-ui:001",
            vec![
                D::DensityChangedCommandOrFocusOrTrust,
                D::HitTargetShrankBelowMinimum,
                D::ProofStale,
            ],
            vec![scale_compact_review(), scale_changes_information_architecture()],
            vec![
                persistence_explained_accessibility(),
                persistence_unexplained_override(),
            ],
        ),
        base_row(
            C::DataUi,
            "Data surface owner",
            "The data surface resolves the compact density scale and keeps the density change presentation-only; a density change that would alter command / focus / trust and an unclassified persistence scope both degrade honestly instead of fracturing the layout",
            "evidence:m5-density-mode-data-ui:001",
            vec![
                D::DensityChangedCommandOrFocusOrTrust,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![scale_compact_data(), scale_changes_command_focus()],
            vec![persistence_scope_unclassified()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings surface resolves the standard density scale across every surface element and persists the choice at profile scope; a density scale that omits the inspector element degrades honestly instead of claiming full coverage",
            "evidence:m5-density-mode-settings-ui:001",
            vec![
                D::DensityModeUnstated,
                D::DensityChangedCommandOrFocusOrTrust,
                D::ProofStale,
            ],
            vec![scale_standard_notebook(), scale_element_incomplete()],
            vec![persistence_profile_settings()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved density-scale and persistence truth, so a private scale or an unstated registry token is visible in evidence rather than hidden behind a screenshot",
            "evidence:m5-density-mode-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::MetricCopiedByHandAcrossPackages,
                D::ProofStale,
            ],
            vec![scale_comfortable_support(), scale_token_unstated()],
            vec![persistence_token_unstated()],
        ),
    ]
}

fn governance_review() -> M5DensityModeRegistriesGovernanceReview {
    M5DensityModeRegistriesGovernanceReview {
        density_registry_names_token_role_and_mode: true,
        density_scale_encoded_as_logical_pixel_tokens: true,
        every_surface_resolves_from_shared_registry: true,
        density_changes_presentation_only: true,
        hit_targets_never_shrink_below_supported_minimum: true,
        every_mode_covers_all_surface_elements: true,
        density_persists_at_profile_scope_by_default: true,
        density_never_switches_silently: true,
        first_consumers_use_canonical_density_grammar: true,
        every_row_declares_mandatory_anatomy: true,
        every_row_declares_accessibility_route: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5DensityModeRegistriesConsumerProjection {
    M5DensityModeRegistriesConsumerProjection {
        shell_consumes_shared_registries: true,
        editor_consumes_shared_registries: true,
        review_consumes_shared_registries: true,
        notebook_and_data_consume_shared_registries: true,
        density_traces_to_single_domain_contract: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5DensityModeRegistriesProofFreshness {
    M5DensityModeRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DensityModeRegistriesReleasePosture {
    M5DensityModeRegistriesReleasePosture {
        proof_packet_ref: M5_DENSITY_MODE_REGISTRIES_ARTIFACT_REF.to_owned(),
        geometry_audit_ref: M5_DENSITY_MODE_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DENSITY_MODE_REGISTRIES_SCHEMA_REF,
        M5_DENSITY_MODE_REGISTRIES_DOC_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
        M5_DENSITY_MODE_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 density-mode registries packet.
pub fn seeded_m5_density_mode_registries() -> M5DensityModeRegistriesPacket {
    M5DensityModeRegistriesPacket::new(M5DensityModeRegistriesPacketInput {
        packet_id: M5_DENSITY_MODE_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 density-mode registries with canonical comfortable / standard / compact row and control heights, tab / chip spacing, panel padding, and gutter spacing tokens, list / tree / table / tab / panel / editor / inspector surface-element coverage, profile-scope persistence with explained local overrides, and registry-bound tracing across shell, editor, review, notebook, data, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5DensityModeRegistriesVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the editor-UI row is held at Beta pending hit-target-minimum proof at 400% zoom on
/// every deployment line; every row stays visible and every example stays honest.
pub fn seeded_m5_density_mode_registries_editor_ui_beta_narrowed() -> M5DensityModeRegistriesPacket
{
    let mut packet = seeded_m5_density_mode_registries();
    packet.packet_id = "m5-density-mode-registries:editor-ui-beta:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ShellGeometryConsumerSurface::EditorUi)
        .expect("editor-ui row present");
    row.qualification = M5ShellGeometryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the settings-UI row is narrowed to Preview pending surface-element parity on every
/// surface; every row stays visible and every example stays honest.
pub fn seeded_m5_density_mode_registries_settings_ui_preview_narrowed(
) -> M5DensityModeRegistriesPacket {
    let mut packet = seeded_m5_density_mode_registries();
    packet.packet_id = "m5-density-mode-registries:settings-ui-preview:0001".to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5ShellGeometryConsumerSurface::SettingsUi)
        .expect("settings-ui row present");
    row.qualification = M5ShellGeometryQualificationClass::Preview;
    packet
}
