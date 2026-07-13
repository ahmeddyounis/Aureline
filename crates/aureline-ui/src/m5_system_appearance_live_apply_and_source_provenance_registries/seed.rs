//! Canonical seed builders for the M5 system-appearance live-apply and appearance-source-provenance packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and
//! the fixtures never drift. Every resolved example is built by calling the real resolvers so the packet
//! can only carry projections the resolvers actually produce. Clean response and provenance entries are built
//! so the live-apply / restart-required / unsupported postures, the canonical posture labels (`applies live`
//! / `restart required` / `not supported on this host`), the applied / canonical / accessible response forms,
//! and the stable-ID / record-surface / source-signal provenance triple are proven across the shell, settings,
//! docs, onboarding, CLI, and support surfaces without any hand-copied per-platform behavior, mislabeled
//! posture, lost active-context continuity, response-form gap, or unrecorded appearance source.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SYSTEM_APPEARANCE_REGISTRIES_PACKET_ID: &str =
    "m5-system-appearance-live-apply-and-source-provenance-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-13T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn response(
    input: M5AppearanceLiveApplyEntryResolutionInput,
) -> M5ResolvedAppearanceLiveApplyEntry {
    resolve_appearance_live_apply_entry(input).expect("seed appearance-response entry resolves")
}

fn provenance(
    input: M5AppearanceSourceProvenanceEntryResolutionInput,
) -> M5ResolvedAppearanceSourceProvenanceEntry {
    resolve_appearance_source_provenance_entry(input).expect("seed provenance entry resolves")
}

fn all_forms() -> Vec<M5AppearanceResponseForm> {
    M5AppearanceResponseForm::ALL.to_vec()
}

// -- Clean appearance-response entries (registry-bound live-apply or explained fallback) ---------

#[allow(clippy::too_many_arguments)]
fn clean_response_base(
    entry_id: &str,
    command_id: &str,
    token_name: &str,
    semantic_role: M5PlatformFitRole,
    appearance_role: M5ThemeContrastLiveChangeRole,
    posture: M5AppearancePosture,
    surface_context: M5AppearanceSurfaceContext,
    applied_appearance_summary: &str,
    posture_label: &str,
) -> M5AppearanceLiveApplyEntryResolutionInput {
    M5AppearanceLiveApplyEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        command_id: command_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        appearance_role,
        posture,
        surface_context,
        response_form_coverage: all_forms(),
        applied_appearance_summary: applied_appearance_summary.to_owned(),
        posture_label: posture_label.to_owned(),
        bound_to_registry: true,
        preserves_active_context_continuity: true,
        live_reapplied: posture.applies_live(),
        fallback_explained: true,
        proof_fresh: true,
    }
}

fn response_shell_live() -> M5ResolvedAppearanceLiveApplyEntry {
    response(clean_response_base(
        "response:shell:theme:live",
        "command.appearance.apply",
        "appearance.theme.live",
        M5PlatformFitRole::Appearance,
        M5ThemeContrastLiveChangeRole::LiveThemeResponse,
        M5AppearancePosture::LiveApply,
        M5AppearanceSurfaceContext::ShellChrome,
        "dark theme, accent blue",
        "applies live",
    ))
}

fn response_editor_live() -> M5ResolvedAppearanceLiveApplyEntry {
    response(clean_response_base(
        "response:editor:contrast:live",
        "command.appearance.apply",
        "appearance.contrast.live",
        M5PlatformFitRole::Appearance,
        M5ThemeContrastLiveChangeRole::LiveContrastResponse,
        M5AppearancePosture::LiveApply,
        M5AppearanceSurfaceContext::ActiveEditor,
        "high-contrast on",
        "applies live",
    ))
}

fn response_dialog_restart() -> M5ResolvedAppearanceLiveApplyEntry {
    response(clean_response_base(
        "response:dialog:textscale:restart",
        "command.appearance.apply",
        "appearance.text_scale.restart",
        M5PlatformFitRole::Appearance,
        M5ThemeContrastLiveChangeRole::AccentAndTextScaleResponse,
        M5AppearancePosture::RestartRequired,
        M5AppearanceSurfaceContext::OpenDialog,
        "text scale 125%",
        "restart required",
    ))
}

fn response_settings_live() -> M5ResolvedAppearanceLiveApplyEntry {
    response(clean_response_base(
        "response:settings:theme:live",
        "command.appearance.apply",
        "appearance.theme.settings.live",
        M5PlatformFitRole::Appearance,
        M5ThemeContrastLiveChangeRole::LiveThemeResponse,
        M5AppearancePosture::LiveApply,
        M5AppearanceSurfaceContext::SettingsPreview,
        "light theme, accent teal",
        "applies live",
    ))
}

fn response_docs_command_stability() -> M5ResolvedAppearanceLiveApplyEntry {
    response(clean_response_base(
        "response:docs:help:live",
        "command.appearance.apply",
        "appearance.help.live",
        M5PlatformFitRole::CommandStability,
        M5ThemeContrastLiveChangeRole::BoundToAppearanceRegistry,
        M5AppearancePosture::LiveApply,
        M5AppearanceSurfaceContext::DocsHelp,
        "dark theme, accent blue",
        "applies live",
    ))
}

// -- Degraded appearance-response entries --------------------------------------------------------

/// Degraded response entry: the behavior is a hand-copied per-platform response instead of tracing to the
/// registry.
fn response_hand_copied() -> M5ResolvedAppearanceLiveApplyEntry {
    let mut input = clean_response_base(
        "response:shell:hand-copied",
        "command.appearance.apply",
        "appearance.theme.live",
        M5PlatformFitRole::Appearance,
        M5ThemeContrastLiveChangeRole::SilentThemeDriftDisallowed,
        M5AppearancePosture::LiveApply,
        M5AppearanceSurfaceContext::ShellChrome,
        "dark theme, accent blue",
        "applies live",
    );
    input.bound_to_registry = false;
    response(input)
}

/// Degraded response entry: a live-apply entry that did not reapply live is mislabeled for its posture.
fn response_mislabeled() -> M5ResolvedAppearanceLiveApplyEntry {
    let mut input = clean_response_base(
        "response:settings:mislabeled:live",
        "command.appearance.apply",
        "appearance.theme.settings.live",
        M5PlatformFitRole::Appearance,
        M5ThemeContrastLiveChangeRole::LiveThemeResponse,
        M5AppearancePosture::LiveApply,
        M5AppearanceSurfaceContext::SettingsPreview,
        "light theme, accent teal",
        "applies live",
    );
    // A live-apply entry that claims to apply live but did not reapply is mislabeled for its posture.
    input.live_reapplied = false;
    response(input)
}

/// Degraded response entry: the change resets local context instead of preserving active-editor continuity.
fn response_continuity_lost() -> M5ResolvedAppearanceLiveApplyEntry {
    let mut input = clean_response_base(
        "response:editor:continuity-lost:live",
        "command.appearance.apply",
        "appearance.contrast.live",
        M5PlatformFitRole::Appearance,
        M5ThemeContrastLiveChangeRole::LiveContrastResponse,
        M5AppearancePosture::LiveApply,
        M5AppearanceSurfaceContext::ActiveEditor,
        "high-contrast on",
        "applies live",
    );
    input.preserves_active_context_continuity = false;
    response(input)
}

/// Degraded response entry: the applied / canonical / accessible response-form coverage is incomplete.
fn response_form_incomplete() -> M5ResolvedAppearanceLiveApplyEntry {
    let mut input = clean_response_base(
        "response:docs:form-incomplete:live",
        "command.appearance.apply",
        "appearance.help.live",
        M5PlatformFitRole::Appearance,
        M5ThemeContrastLiveChangeRole::BoundToAppearanceRegistry,
        M5AppearancePosture::LiveApply,
        M5AppearanceSurfaceContext::DocsHelp,
        "dark theme, accent blue",
        "applies live",
    );
    input.response_form_coverage = vec![M5AppearanceResponseForm::AppliedVisualReapply];
    response(input)
}

/// Degraded response entry: a restart-required change with no explained fallback narrows behavior silently.
fn response_narrower_unexplained() -> M5ResolvedAppearanceLiveApplyEntry {
    let mut input = clean_response_base(
        "response:cli:narrower-unexplained:restart",
        "command.appearance.apply",
        "appearance.text_scale.restart",
        M5PlatformFitRole::Appearance,
        M5ThemeContrastLiveChangeRole::AccentAndTextScaleResponse,
        M5AppearancePosture::RestartRequired,
        M5AppearanceSurfaceContext::OpenDialog,
        "text scale 125%",
        "restart required",
    );
    input.fallback_explained = false;
    response(input)
}

/// Degraded response entry: the canonical registry token name is unstated.
fn response_token_unstated() -> M5ResolvedAppearanceLiveApplyEntry {
    let mut input = clean_response_base(
        "response:support:token-unstated:live",
        "command.appearance.apply",
        "  ",
        M5PlatformFitRole::Appearance,
        M5ThemeContrastLiveChangeRole::LiveThemeResponse,
        M5AppearancePosture::LiveApply,
        M5AppearanceSurfaceContext::ShellChrome,
        "dark theme, accent blue",
        "applies live",
    );
    input.token_name = "  ".to_owned();
    response(input)
}

// -- Clean appearance-source-provenance entries -------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_provenance_base(
    entry_id: &str,
    command_id: &str,
    token_name: &str,
    provenance_role: M5ThemeContrastLiveChangeRole,
    record_surface: M5AppearanceRecordSurface,
    surface_context: M5AppearanceSurfaceContext,
    source_signal_label: &str,
    record_route: &str,
) -> M5AppearanceSourceProvenanceEntryResolutionInput {
    M5AppearanceSourceProvenanceEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        command_id: command_id.to_owned(),
        token_name: token_name.to_owned(),
        provenance_role,
        semantic_role: M5PlatformFitRole::Appearance,
        record_surface,
        surface_context,
        response_form_coverage: all_forms(),
        source_signal_label: source_signal_label.to_owned(),
        record_route: record_route.to_owned(),
        posture_recorded: true,
        proof_fresh: true,
    }
}

fn provenance_settings() -> M5ResolvedAppearanceSourceProvenanceEntry {
    provenance(clean_provenance_base(
        "provenance:settings:source",
        "command.appearance.source",
        "provenance.settings.source",
        M5ThemeContrastLiveChangeRole::BoundToAppearanceRegistry,
        M5AppearanceRecordSurface::Settings,
        M5AppearanceSurfaceContext::SettingsPreview,
        "system appearance",
        "settings.appearance.source",
    ))
}

fn provenance_diagnostics() -> M5ResolvedAppearanceSourceProvenanceEntry {
    provenance(clean_provenance_base(
        "provenance:diagnostics:source",
        "command.appearance.source",
        "provenance.diagnostics.source",
        M5ThemeContrastLiveChangeRole::LiveThemeResponse,
        M5AppearanceRecordSurface::Diagnostics,
        M5AppearanceSurfaceContext::ShellChrome,
        "system contrast",
        "diagnostics.appearance.source",
    ))
}

fn provenance_support_export() -> M5ResolvedAppearanceSourceProvenanceEntry {
    provenance(clean_provenance_base(
        "provenance:support:source",
        "command.appearance.source",
        "provenance.support.source",
        M5ThemeContrastLiveChangeRole::AccentAndTextScaleResponse,
        M5AppearanceRecordSurface::SupportExport,
        M5AppearanceSurfaceContext::DocsHelp,
        "system accent and text scale",
        "support.appearance.source",
    ))
}

// -- Degraded appearance-source-provenance entries ----------------------------------------------

/// Degraded provenance entry: the active source or posture is not recorded — the recording triple is broken.
fn provenance_not_recorded() -> M5ResolvedAppearanceSourceProvenanceEntry {
    let mut input = clean_provenance_base(
        "provenance:shell:not-recorded",
        "command.appearance.source",
        "provenance.settings.source",
        M5ThemeContrastLiveChangeRole::BoundToAppearanceRegistry,
        M5AppearanceRecordSurface::Settings,
        M5AppearanceSurfaceContext::SettingsPreview,
        "system appearance",
        "settings.appearance.source",
    );
    input.posture_recorded = false;
    provenance(input)
}

/// Degraded provenance entry: the applied / canonical / accessible response-form coverage is incomplete.
fn provenance_phrasing_incomplete() -> M5ResolvedAppearanceSourceProvenanceEntry {
    let mut input = clean_provenance_base(
        "provenance:docs:phrasing-incomplete",
        "command.appearance.source",
        "provenance.support.source",
        M5ThemeContrastLiveChangeRole::AccentAndTextScaleResponse,
        M5AppearanceRecordSurface::SupportExport,
        M5AppearanceSurfaceContext::DocsHelp,
        "system accent and text scale",
        "support.appearance.source",
    );
    input.response_form_coverage = vec![M5AppearanceResponseForm::AppliedVisualReapply];
    provenance(input)
}

/// Degraded provenance entry: the record surface is unclassified.
fn provenance_surface_unclassified() -> M5ResolvedAppearanceSourceProvenanceEntry {
    provenance(clean_provenance_base(
        "provenance:onboarding:surface-unclassified",
        "command.appearance.source",
        "provenance.unknown.source",
        M5ThemeContrastLiveChangeRole::LiveThemeResponse,
        M5AppearanceRecordSurface::RecordSurfaceUnclassified,
        M5AppearanceSurfaceContext::SettingsPreview,
        "system appearance",
        "settings.appearance.source",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5SystemAppearanceRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5PlatformFitDowngradeTrigger>,
    appearance_live_apply_entries: Vec<M5ResolvedAppearanceLiveApplyEntry>,
    appearance_source_provenance_entries: Vec<M5ResolvedAppearanceSourceProvenanceEntry>,
) -> M5SystemAppearanceRegistriesRow {
    M5SystemAppearanceRegistriesRow {
        consumer_surface,
        qualification: M5PlatformFitQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5PlatformFitDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5PlatformFitRequiredLabel::Identity,
            M5PlatformFitRequiredLabel::SemanticRole,
            M5PlatformFitRequiredLabel::RegistryReference,
            M5PlatformFitRequiredLabel::HostPlatform,
            M5PlatformFitRequiredLabel::PathVerb,
        ],
        accessibility_routes: M5PlatformFitAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5AppearanceRegistryAnatomyPart::ALL.to_vec(),
        export_fields: M5AppearanceRegistryExportField::ALL.to_vec(),
        downgrade_triggers,
        appearance_live_apply_entries,
        appearance_source_provenance_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SYSTEM_APPEARANCE_REGISTRIES_SCHEMA_REF,
            M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
        ]),
        appearance_change_corrupts_focus_layout_or_meaning_on_protected_path: false,
        live_change_forces_mystery_repaint_or_resets_context: false,
        appearance_response_hardcoded_instead_of_registry: false,
        diagnostics_or_export_cannot_distinguish_live_from_restart: false,
    }
}

fn registry_rows() -> Vec<M5SystemAppearanceRegistriesRow> {
    use M5PlatformFitConsumerSurface as C;
    use M5PlatformFitDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell reapplies the system theme live to the shell chrome from the shared appearance registry and records the active appearance source in settings; a hand-copied per-platform response and a source that is not recorded degrade honestly instead of reading as a clean pass",
            "evidence:m5-appearance-shell-ui:001",
            vec![
                D::ShortcutNotationDriftedByPlatform,
                D::ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback,
                D::ProofStale,
            ],
            vec![response_shell_live(), response_hand_copied()],
            vec![provenance_settings(), provenance_not_recorded()],
        ),
        base_row(
            C::SettingsUi,
            "Settings surface owner",
            "The settings preview reapplies the system theme live from the registry and records the source and posture in diagnostics; a live-apply entry that did not reapply live is caught as mislabeled for its posture",
            "evidence:m5-appearance-settings-ui:001",
            vec![
                D::ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback,
                D::PathVerbUnstated,
                D::ProofStale,
            ],
            vec![response_settings_live(), response_mislabeled()],
            vec![provenance_diagnostics()],
        ),
        base_row(
            C::DocsHelp,
            "Docs/help surface owner",
            "Docs and help render the live theme response across the applied, canonical, and accessible response forms and record the source in the support export; a response and a provenance record that omit a response form degrade honestly so a diagnostics panel cannot reintroduce an incorrect posture",
            "evidence:m5-appearance-docs-help:001",
            vec![
                D::PathVerbUnstated,
                D::ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback,
                D::ProofStale,
            ],
            vec![response_docs_command_stability(), response_form_incomplete()],
            vec![provenance_support_export(), provenance_phrasing_incomplete()],
        ),
        base_row(
            C::Onboarding,
            "Onboarding surface owner",
            "Onboarding reapplies the live contrast response to the active editor from the registry while preserving active-context continuity; a change that resets local context and a record with an unclassified record surface degrade honestly",
            "evidence:m5-appearance-onboarding:001",
            vec![
                D::PlatformWordingChangedCommandOrPermissionMeaning,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![response_editor_live(), response_continuity_lost()],
            vec![provenance_surface_unclassified()],
        ),
        base_row(
            C::CliExport,
            "CLI/export owner",
            "The CLI export records the restart-required text-scale posture from the appearance registry and explains the narrower behavior; a restart-required change with no explained fallback degrades honestly instead of silently narrowing",
            "evidence:m5-appearance-cli-export:001",
            vec![
                D::ThemeOrContrastChangeDidNotApplyLiveOrExplainFallback,
                D::PathVerbUnstated,
                D::ProofStale,
            ],
            vec![response_dialog_restart(), response_narrower_unexplained()],
            vec![provenance_settings()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved appearance-response and provenance truth, so a hand-copied response or an unstated registry token is visible in evidence rather than hidden behind a diagnostics panel",
            "evidence:m5-appearance-support-export:001",
            vec![
                D::PathVerbUnstated,
                D::HostPlatformUnstated,
                D::ProofStale,
            ],
            vec![response_shell_live(), response_token_unstated()],
            vec![provenance_diagnostics()],
        ),
    ]
}

fn governance_review() -> M5SystemAppearanceRegistriesGovernanceReview {
    M5SystemAppearanceRegistriesGovernanceReview {
        appearance_registry_names_token_role_and_posture: true,
        live_changes_applied_from_shared_registry: true,
        live_versus_fallback_posture_truth_kept_explicit: true,
        narrower_behavior_explained_on_every_profile: true,
        active_context_continuity_preserved_through_live_change: true,
        appearance_source_and_posture_recorded_in_settings_diagnostics_and_export: true,
        every_entry_covers_all_response_forms: true,
        appearance_response_bound_to_single_registry_not_hand_copied: true,
        diagnostics_and_export_generated_from_registry: true,
        posture_or_provenance_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SystemAppearanceRegistriesConsumerProjection {
    M5SystemAppearanceRegistriesConsumerProjection {
        shell_consumes_shared_registries: true,
        settings_consumes_shared_registries: true,
        docs_help_consumes_shared_registries: true,
        onboarding_and_cli_consume_shared_registries: true,
        appearance_traces_to_single_domain_contract: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SystemAppearanceRegistriesProofFreshness {
    M5SystemAppearanceRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SystemAppearanceRegistriesReleasePosture {
    M5SystemAppearanceRegistriesReleasePosture {
        proof_packet_ref: M5_SYSTEM_APPEARANCE_REGISTRIES_ARTIFACT_REF.to_owned(),
        platform_fit_audit_ref: M5_SYSTEM_APPEARANCE_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SYSTEM_APPEARANCE_REGISTRIES_SCHEMA_REF,
        M5_SYSTEM_APPEARANCE_REGISTRIES_DOC_REF,
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
        M5_PLATFORM_FIT_MATRIX_DOC_REF,
        M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 system-appearance live-apply and appearance-source-provenance registries packet.
pub fn seeded_m5_system_appearance_live_apply_and_source_provenance_registries(
) -> M5SystemAppearanceRegistriesPacket {
    M5SystemAppearanceRegistriesPacket::new(M5SystemAppearanceRegistriesPacketInput {
        packet_id: M5_SYSTEM_APPEARANCE_REGISTRIES_PACKET_ID.to_owned(),
        registries_label:
            "M5 system-appearance live-apply and appearance-source-provenance registries with live-apply / restart-required / unsupported postures, canonical posture labels (applies live / restart required / not supported on this host), applied / canonical / accessible response-form coverage, preserved active-context continuity, and stable-ID / record-surface / source-signal provenance across shell, settings, docs, onboarding, CLI, and support surfaces"
                .to_owned(),
        registry_rows: registry_rows(),
        vocabulary_set: M5SystemAppearanceRegistriesVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the docs/help row is held at Beta pending diagnostics-generation parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_system_appearance_live_apply_and_source_provenance_registries_docs_help_beta_narrowed(
) -> M5SystemAppearanceRegistriesPacket {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet.packet_id =
        "m5-system-appearance-live-apply-and-source-provenance-registries:docs-help-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5PlatformFitConsumerSurface::DocsHelp)
        .expect("docs-help row present");
    row.qualification = M5PlatformFitQualificationClass::Beta;
    packet
}

/// Narrowed variant: the CLI/export restart-posture row is narrowed to Preview pending restart-fallback parity
/// on every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_system_appearance_live_apply_and_source_provenance_registries_restart_posture_preview_narrowed(
) -> M5SystemAppearanceRegistriesPacket {
    let mut packet = seeded_m5_system_appearance_live_apply_and_source_provenance_registries();
    packet.packet_id =
        "m5-system-appearance-live-apply-and-source-provenance-registries:restart-posture-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5PlatformFitConsumerSurface::CliExport)
        .expect("cli-export row present");
    row.qualification = M5PlatformFitQualificationClass::Preview;
    packet
}
