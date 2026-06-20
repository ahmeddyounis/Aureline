//! Inline tests for the shell source-language fallback inspector.

use aureline_i18n::{LocaleFallbackOriginClass, M5MessageSurface};

use aureline_i18n::{PackApplicationDecision, SkewDegradeReason};

use aureline_i18n::{LocalizationRenderState, RenderSeverityClass, TextDirection};

use super::contributed_support::{
    project_support_contributed_locale_support, project_user_contributed_locale_support,
    ContributedSupportAudience,
};
use super::fallback_inspector::{
    project_support_locale_fallback_inspector, project_user_locale_fallback_inspector,
    FallbackInspectorAudience,
};
use super::localized_surface::{
    project_support_localized_surface, project_user_localized_surface, LocalizedSurfaceAudience,
};
use super::pack_compatibility::{
    project_support_locale_pack_compatibility, project_user_locale_pack_compatibility,
    CompatibilityViewAudience,
};

use aureline_i18n::{ContributedDegradeReason, LocalizationIssueSourceClass};

#[test]
fn source_locale_is_fully_localized_with_no_missing_keys() {
    let view = project_user_locale_fallback_inspector("en-US");
    assert!(view.is_fully_localized());
    assert_eq!(view.total_missing_key_count, 0);
    assert!(!view.source_language_fallback_active);
    assert_eq!(view.effective_locale, "en-US");
    assert_eq!(
        view.fallback_origin,
        LocaleFallbackOriginClass::RequestedLocaleAuthoritative
    );
    for row in &view.surface_rows {
        assert_eq!(row.missing_key_count, 0);
        assert_eq!(row.localized_key_count, row.total_keys);
    }
}

#[test]
fn fully_translated_locale_reports_authoritative() {
    let view = project_user_locale_fallback_inspector("es-MX");
    assert!(view.is_fully_localized());
    assert_eq!(
        view.fallback_origin,
        LocaleFallbackOriginClass::RequestedLocaleAuthoritative
    );
    assert_eq!(view.requested_locale, "es-MX");
    assert_eq!(view.effective_locale, "es-MX");
}

#[test]
fn partial_locale_discloses_missing_keys_per_surface() {
    let view = project_user_locale_fallback_inspector("ja-JP");
    assert!(!view.is_fully_localized());
    assert!(view.source_language_fallback_active);
    assert_eq!(
        view.fallback_origin,
        LocaleFallbackOriginClass::RequestedLocalePartialWithBaseFill
    );
    assert!(view.source_language_route_active);

    // The total missing count is the sum of per-surface missing counts.
    let summed: usize = view.surface_rows.iter().map(|r| r.missing_key_count).sum();
    assert_eq!(summed, view.total_missing_key_count);
    assert!(view.total_missing_key_count > 0);

    // A surface with a newly added, not-yet-translated key shows the gap.
    let command = view
        .surface_row(M5MessageSurface::CommandPalette)
        .expect("command surface row present");
    assert!(command.missing_key_count >= 1);
}

#[test]
fn signature_failed_locale_falls_back_to_source_for_every_key() {
    let view = project_user_locale_fallback_inspector("de-DE");
    assert_eq!(view.effective_locale, "en-US");
    assert_eq!(view.total_missing_key_count, view.total_keys);
    assert_eq!(
        view.fallback_origin,
        LocaleFallbackOriginClass::PackSignatureFailedSourceLanguageOnly
    );
    for row in &view.surface_rows {
        assert_eq!(row.localized_key_count, 0);
        assert!(row.source_language_fallback_active || row.total_keys == 0);
    }
}

#[test]
fn unprofiled_locale_still_resolves_to_an_honest_view() {
    // fr-FR has no declared profile and no translated keys, so the inspector
    // derives a full source-language fallback rather than failing.
    let view = project_user_locale_fallback_inspector("fr-FR");
    assert_eq!(view.requested_locale, "fr-FR");
    assert_eq!(
        view.fallback_chain.first().map(String::as_str),
        Some("fr-FR")
    );
    assert_eq!(
        view.fallback_chain.last().map(String::as_str),
        Some("en-US")
    );
    assert_eq!(view.total_missing_key_count, view.total_keys);
    assert_eq!(
        view.fallback_origin,
        LocaleFallbackOriginClass::PackMissingSourceLanguageOnly
    );
}

#[test]
fn user_and_support_views_agree_on_the_numbers() {
    let user = project_user_locale_fallback_inspector("ar-SA");
    let support = project_support_locale_fallback_inspector("ar-SA");

    assert_eq!(user.audience, FallbackInspectorAudience::User);
    assert_eq!(support.audience, FallbackInspectorAudience::SupportExport);

    // Both surfaces quote the same fallback truth.
    assert_eq!(user.fallback_chain, support.fallback_chain);
    assert_eq!(user.fallback_origin, support.fallback_origin);
    assert_eq!(
        user.total_missing_key_count,
        support.total_missing_key_count
    );
    assert_eq!(user.surface_rows, support.surface_rows);

    // Neither surface leaks translated body text.
    assert!(user.raw_translated_body_omitted);
    assert!(support.raw_translated_body_omitted);
}

#[test]
fn view_serializes_to_export_safe_json() {
    let view = project_support_locale_fallback_inspector("ja-JP");
    let json = serde_json::to_string(&view).expect("view serializes");
    assert!(json.contains("shell_locale_fallback_inspector_view"));
    assert!(json.contains("missing_key_count"));
    // Stable ids are present; no translated prose body field exists.
    assert!(json.contains("registry_packet_id"));
}

#[test]
fn pack_compatibility_view_resolves_every_pack_without_ambiguity() {
    let view = project_user_locale_pack_compatibility();
    assert_eq!(view.audience, CompatibilityViewAudience::User);
    assert!(view.guardrail_clean);
    // No pack is left in an undefined half-localized state.
    assert!(view.all_states_resolved());
    assert_eq!(
        view.renderable_packs + view.degraded_source_language_packs,
        view.total_packs
    );
    assert!(view.raw_translated_body_omitted);
}

#[test]
fn pack_compatibility_view_surfaces_skew_reasons_and_versions() {
    let view = project_user_locale_pack_compatibility();

    let ja = view.row("locale-pack:core:ja-jp").expect("ja-jp row");
    assert!(ja.degraded_to_source_language());
    assert_eq!(ja.skew_degrade_reason, SkewDegradeReason::SignatureFailed);
    assert_eq!(ja.missing_key_count, ja.total_key_count);
    assert!(!ja.pack_version.is_empty());
    assert!(!ja.claimed_localized_profile);

    let fr = view.row("locale-pack:core:fr-fr").expect("fr-fr row");
    assert_eq!(
        fr.application_decision,
        PackApplicationDecision::ApplyLocalizedWithDisclosedMissingKeys
    );
    assert!(fr.missing_key_count > 0 && fr.missing_key_count < fr.total_key_count);
    assert!(fr.claimed_localized_profile);
}

#[test]
fn pack_compatibility_user_and_support_views_agree() {
    let user = project_user_locale_pack_compatibility();
    let support = project_support_locale_pack_compatibility();

    assert_eq!(user.audience, CompatibilityViewAudience::User);
    assert_eq!(support.audience, CompatibilityViewAudience::SupportExport);
    assert_eq!(user.rows, support.rows);
    assert_eq!(user.total_missing_keys, support.total_missing_keys);
    assert_eq!(user.guardrail_clean, support.guardrail_clean);
}

#[test]
fn localized_surface_keeps_command_ids_and_keyboard_paths_under_locale() {
    let user = project_user_localized_surface("es-MX");
    assert_eq!(user.audience, LocalizedSurfaceAudience::User);
    assert!(!user.raw_translated_body_omitted);

    let run_build = user.row("msg:command:run-build").expect("run-build row");
    // Label localized, but the command id and shortcut did not move.
    assert_eq!(
        run_build.command_id_ref.as_deref(),
        Some("workbench.action.tasks.runBuild")
    );
    assert_eq!(
        run_build.keyboard_path_hint.as_deref(),
        Some("Ctrl+Shift+B")
    );
    assert_eq!(
        run_build.localization_state,
        LocalizationRenderState::Localized
    );
    assert_eq!(
        run_build.display_label.as_deref(),
        Some("Ejecutar tarea de compilación")
    );
    assert!(run_build.is_command_discoverable());
    assert!(user
        .discoverable_command_ids
        .contains(&"workbench.action.tasks.runBuild".to_owned()));
}

#[test]
fn command_ids_are_identical_across_locales() {
    let es = project_user_localized_surface("es-MX");
    let ja = project_user_localized_surface("ja-JP");
    let ar = project_user_localized_surface("ar-SA");

    assert_eq!(es.discoverable_command_ids, ja.discoverable_command_ids);
    assert_eq!(es.discoverable_command_ids, ar.discoverable_command_ids);

    // Keyboard hints are locale-neutral too.
    let key = |view: &super::localized_surface::LocalizedSurfaceView| {
        view.row("msg:command:open-settings")
            .and_then(|row| row.keyboard_path_hint.clone())
    };
    assert_eq!(key(&es), Some("Ctrl+,".to_owned()));
    assert_eq!(key(&es), key(&ja));
    assert_eq!(key(&es), key(&ar));
}

#[test]
fn disabled_reason_keeps_diagnostic_id_and_severity_while_localizing() {
    let user = project_user_localized_surface("ja-JP");
    let disabled = user
        .row("msg:error:command:disabled-reason")
        .expect("disabled-reason row");

    assert_eq!(
        disabled.diagnostic_id_ref.as_deref(),
        Some("command.disabled.reason")
    );
    assert_eq!(disabled.severity, RenderSeverityClass::Warning);
    // Placeholders carry through so the disabled reason stays renderable.
    assert!(disabled.placeholders_preserved);
    assert!(disabled
        .display_label
        .as_deref()
        .is_some_and(|label| label.contains("{command}") && label.contains("{reason}")));
    // Severity is never hidden by truncation.
    assert!(disabled.severity_preserved_under_truncation);
}

#[test]
fn rtl_locale_surface_is_right_to_left_and_marks_fallback() {
    let ar = project_user_localized_surface("ar-SA");
    assert_eq!(ar.text_direction, TextDirection::RightToLeft);
    assert!(ar.source_fallback_rows > 0);

    // A row ar-SA does not translate falls back, but stays command-discoverable.
    let open_notebook = ar.row("msg:command:open-notebook").expect("row");
    assert_eq!(
        open_notebook.localization_state,
        LocalizationRenderState::SourceLanguageFallback
    );
    assert_eq!(open_notebook.effective_locale, "en-US");
    assert_eq!(
        open_notebook.command_id_ref.as_deref(),
        Some("notebook.action.open")
    );
}

#[test]
fn support_export_omits_translated_body_but_keeps_ids_and_metrics() {
    let user = project_user_localized_surface("es-MX");
    let support = project_support_localized_surface("es-MX");

    assert_eq!(support.audience, LocalizedSurfaceAudience::SupportExport);
    assert!(support.raw_translated_body_omitted);
    assert!(support.all_severities_preserved());

    // Same stable ids and metrics on both audiences; only the body differs.
    assert_eq!(
        user.discoverable_command_ids,
        support.discoverable_command_ids
    );
    let support_row = support.row("msg:command:run-build").expect("row");
    assert!(support_row.display_label.is_none());
    assert!(support_row.truncated_label.is_none());
    assert_eq!(
        support_row.command_id_ref.as_deref(),
        Some("workbench.action.tasks.runBuild")
    );
    assert!(support_row.expansion_ratio_pct > 0);

    let json = serde_json::to_string(&support).expect("serializes");
    assert!(json.contains("shell_localized_surface_view"));
    // No translated Spanish body leaks into the support export.
    assert!(!json.contains("Ejecutar tarea de compilación"));
}

#[test]
fn truncation_shortens_label_without_hiding_scope_or_severity() {
    let user = project_user_localized_surface("es-MX");
    let error = user
        .row("msg:error:locale-pack:signature-failed")
        .expect("error row");

    // The long localized error truncates to the budget yet keeps its severity,
    // scope, and stable diagnostic id.
    assert!(error.was_truncated);
    assert_eq!(error.severity, RenderSeverityClass::Warning);
    assert_eq!(error.surface_key, "error");
    assert_eq!(
        error.diagnostic_id_ref.as_deref(),
        Some("i18n.locale_pack.signature_failed")
    );
    assert!(error.severity_preserved_under_truncation);
    let truncated = error.truncated_label.as_deref().expect("truncated label");
    assert!(truncated.chars().count() <= user.truncation_budget_graphemes);
}

#[test]
fn contributed_support_view_attributes_every_source_class() {
    let view = project_user_contributed_locale_support();
    assert_eq!(view.audience, ContributedSupportAudience::User);

    // Issue attribution covers all three source classes the spec calls out.
    assert!(view.issue_counts_by_source.first_party_pack > 0);
    assert!(view.issue_counts_by_source.extension_pack > 0);
    // A deliberately narrower companion is not a support defect, so the
    // companion issue count stays zero even though a companion row degrades.
    assert_eq!(view.issue_counts_by_source.companion_overlay, 0);

    let extension_rows: Vec<_> = view
        .rows_for_source(LocalizationIssueSourceClass::ExtensionPack)
        .collect();
    let companion_rows: Vec<_> = view
        .rows_for_source(LocalizationIssueSourceClass::CompanionOverlay)
        .collect();
    assert!(!extension_rows.is_empty());
    assert!(!companion_rows.is_empty());
}

#[test]
fn contributed_support_view_keeps_host_labels_canonical() {
    let view = project_user_contributed_locale_support();
    assert!(view.host_stable_labels_canonical);
    assert!(view.host_labels_all_canonical());
    assert_eq!(view.host_stable_label_classes_protected, 4);
    assert!(view.guardrail_clean);
    // Even degraded contributed rows keep host-stable labels canonical.
    for row in &view.rows {
        assert!(row.host_stable_labels_canonical);
    }
}

#[test]
fn contributed_support_view_discloses_narrower_companion_truthfully() {
    let view = project_user_contributed_locale_support();
    let row = view
        .row("contributed-support:companion:browser-handoff:ja-jp")
        .expect("companion ja-jp row");
    assert!(row.degraded_to_source_language);
    assert_eq!(
        row.degrade_reason,
        ContributedDegradeReason::CompanionScopeNarrowerThanDesktop
    );
    assert!(!row.missing_support_on_claimed_profile);
    assert!(!row.claimed_localized_profile);
}

#[test]
fn contributed_support_export_omits_translated_bodies() {
    let support = project_support_contributed_locale_support();
    assert_eq!(support.audience, ContributedSupportAudience::SupportExport);
    assert!(support.raw_translated_body_omitted);
    let json = serde_json::to_string(&support).expect("serializes");
    assert!(json.contains("shell_contributed_locale_support_view"));
}

use super::attention_vocabulary::{
    project_support_attention_vocabulary, project_user_attention_vocabulary,
    AttentionSurfaceFamily, AttentionVocabularyAudience,
};

#[test]
fn every_durable_attention_surface_binds_to_one_governed_vocabulary() {
    let view = project_user_attention_vocabulary("es-MX");
    // Every bound state resolves to a governed glossary term.
    let glossary = aureline_i18n::seeded_attention_vocabulary_glossary();
    for row in &view.rows {
        assert!(
            glossary.governs(&row.governed_term_key),
            "{} -> {} is not governed",
            row.canonical_token,
            row.governed_term_key
        );
    }
    // All seven canonical surface families are represented.
    let families: std::collections::BTreeSet<_> =
        view.rows.iter().map(|r| r.surface_family).collect();
    for family in [
        AttentionSurfaceFamily::ActivityRowState,
        AttentionSurfaceFamily::ActivityPartition,
        AttentionSurfaceFamily::JobRowBadge,
        AttentionSurfaceFamily::DurableLifecycleState,
        AttentionSurfaceFamily::QuietHoursMode,
        AttentionSurfaceFamily::SuppressionReason,
        AttentionSurfaceFamily::QuietHoursDecision,
    ] {
        assert!(families.contains(&family), "missing {family:?}");
    }
}

#[test]
fn running_state_shares_one_term_across_surfaces() {
    // The activity-center row and the durable job row must use the same governed
    // term for "running" — no per-surface wording drift.
    let view = project_user_attention_vocabulary("ja-JP");
    let rows = view.rows_for_term("attention.lifecycle.running");
    let surfaces: std::collections::BTreeSet<_> = rows.iter().map(|r| r.surface_family).collect();
    assert!(surfaces.contains(&AttentionSurfaceFamily::ActivityRowState));
    assert!(surfaces.contains(&AttentionSurfaceFamily::DurableLifecycleState));
    // Every binding paints the same localized label.
    let labels: std::collections::BTreeSet<_> =
        rows.iter().filter_map(|r| r.display_term.clone()).collect();
    assert_eq!(labels.len(), 1, "running term drifts across surfaces");
}

#[test]
fn quiet_hours_held_shares_one_term_across_surfaces() {
    let view = project_user_attention_vocabulary("ar-SA");
    let rows = view.rows_for_term("attention.suppression.held_quiet_hours");
    let surfaces: std::collections::BTreeSet<_> = rows.iter().map(|r| r.surface_family).collect();
    // An activity row state, a suppression reason, and a fanout decision agree.
    assert!(surfaces.contains(&AttentionSurfaceFamily::ActivityRowState));
    assert!(surfaces.contains(&AttentionSurfaceFamily::SuppressionReason));
    assert!(surfaces.contains(&AttentionSurfaceFamily::QuietHoursDecision));
}

#[test]
fn severity_rank_is_inherited_from_the_glossary() {
    let view = project_user_attention_vocabulary("es-MX");
    let glossary = aureline_i18n::seeded_attention_vocabulary_glossary();
    for row in &view.rows {
        let term = glossary.term(&row.governed_term_key).unwrap();
        assert_eq!(
            row.severity_rank, term.severity_rank,
            "{}",
            row.canonical_token
        );
        assert_eq!(row.action_order_index, term.action_order_index);
    }
    assert!(view.all_severities_preserved());
}

#[test]
fn arabic_view_renders_right_to_left() {
    let view = project_user_attention_vocabulary("ar-SA");
    assert_eq!(view.text_direction, TextDirection::RightToLeft);
    for row in &view.rows {
        assert_eq!(row.text_direction, TextDirection::RightToLeft);
    }
}

#[test]
fn support_export_omits_translated_body_but_keeps_anchors() {
    let support = project_support_attention_vocabulary("ja-JP");
    assert_eq!(support.audience, AttentionVocabularyAudience::SupportExport);
    assert!(support.raw_translated_body_omitted);
    for row in &support.rows {
        // No translated prose crosses the support boundary...
        assert!(row.display_term.is_none());
        assert!(row.truncated_term.is_none());
        // ...but support can still map the localized state back to the glossary.
        assert!(!row.governed_term_key.is_empty());
        assert!(!row.canonical_token.is_empty());
    }
    let json = serde_json::to_string(&support).expect("serializes");
    assert!(json.contains("shell_attention_vocabulary_view"));
    // The translated labels must not leak into the support payload.
    assert!(!json.contains("実行中"));
}

#[test]
fn user_view_carries_localized_labels() {
    let view = project_user_attention_vocabulary("es-MX");
    assert_eq!(view.audience, AttentionVocabularyAudience::User);
    assert!(!view.raw_translated_body_omitted);
    let running = view
        .rows
        .iter()
        .find(|r| r.governed_term_key == "attention.lifecycle.running")
        .unwrap();
    assert_eq!(running.display_term.as_deref(), Some("En ejecución"));
}
