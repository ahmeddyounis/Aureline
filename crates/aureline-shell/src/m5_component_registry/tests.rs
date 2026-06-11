//! Inline unit tests for the M5 component-state audit.

use super::*;

#[test]
fn seeded_audit_passes_validation() {
    let report = seeded_m5_component_state_audit();
    validate_m5_component_state_audit(&report).expect("seeded audit must validate");
}

#[test]
fn seeded_audit_inherits_every_required_state() {
    let report = seeded_m5_component_state_audit();
    assert!(report.every_required_state_inherited());
}

#[test]
fn seeded_audit_has_no_blocking_findings() {
    let report = seeded_m5_component_state_audit();
    assert!(report.report_clean);
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    for row in &report.rows {
        assert!(
            row.blocking_findings.is_empty(),
            "row {} carried blocking findings: {:?}",
            row.descriptor.surface_id,
            row.blocking_findings
        );
    }
}

#[test]
fn seeded_audit_covers_every_surface_family() {
    let report = seeded_m5_component_state_audit();
    let families: BTreeSet<M5SurfaceFamily> = report
        .rows
        .iter()
        .map(|row| row.descriptor.surface_family)
        .collect();
    for expected in [
        M5SurfaceFamily::NotebookCellChrome,
        M5SurfaceFamily::ResultGridRow,
        M5SurfaceFamily::ProfilerPanel,
        M5SurfaceFamily::TracePanel,
        M5SurfaceFamily::PipelineCard,
        M5SurfaceFamily::PreviewRouteBadge,
        M5SurfaceFamily::DocsBrowserPane,
        M5SurfaceFamily::CompanionSurface,
        M5SurfaceFamily::SyncStatusSurface,
        M5SurfaceFamily::OffboardingSurface,
    ] {
        assert!(
            families.contains(&expected),
            "surface family {} is not registered",
            expected.as_str()
        );
    }
}

#[test]
fn high_salience_surfaces_require_a_non_color_cue_policy() {
    let report = seeded_m5_component_state_audit();
    assert!(report.high_salience_surface_count > 0);
    for row in &report.rows {
        if row.high_salience {
            assert_eq!(
                row.descriptor.cue_policy,
                M5CuePolicy::NonColorCueRequired,
                "high-salience surface {} must require a non-color cue",
                row.descriptor.surface_id
            );
        }
    }
}

#[test]
fn registry_anchor_index_covers_every_surface() {
    let report = seeded_m5_component_state_audit();
    assert_eq!(report.registry_anchor_index.len(), report.rows.len());
    for row in &report.rows {
        let entry = report
            .registry_anchor_index
            .iter()
            .find(|entry| entry.surface_id == row.descriptor.surface_id)
            .expect("every surface must have a registry-anchor entry");
        assert_eq!(
            entry.registry_anchor_ref,
            row.descriptor.registry_anchor_ref
        );
        assert!(!entry.registry_anchor_ref.is_empty());
    }
}

/// A minimal high-salience descriptor used to exercise binding findings.
fn high_salience_descriptor() -> M5ComponentStateDescriptor {
    M5ComponentStateDescriptor {
        surface_id: "surface:sync.status_surface".to_owned(),
        surface_family: M5SurfaceFamily::SyncStatusSurface,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        registry_anchor_ref: "registry:anchor:sync:status_surface".to_owned(),
        accessibility_note: "note".to_owned(),
        token_group: M5TokenGroup::SyncStatusTokens,
        style_provenance: M5StyleProvenance::ShellTokenInherited,
        semantic_salience: M5SemanticSalience::SeverityBearing,
        cue_policy: M5CuePolicy::NonColorCueRequired,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        documented_overrides: vec![],
        registered_in_shared_registry: true,
    }
}

fn bare_binding(state: M5NormalizedState, status: M5BindingStatus) -> M5StateBinding {
    M5StateBinding {
        state,
        binding_status: status,
        projected_token_group: None,
        projected_token_ref: None,
        projected_style_provenance: None,
        projected_cue_policy: None,
        projected_non_color_cue: None,
        projected_registry_anchor_ref: None,
        applied_overrides: vec![],
        hardcoded_value: None,
        unresolved_token_fallback: None,
        narrowing_reason: None,
        note: None,
    }
}

#[test]
fn unknown_token_gap_blocks_when_high_salience() {
    let descriptor = high_salience_descriptor();
    let bindings = vec![bare_binding(
        M5NormalizedState::PolicyBlocked,
        M5BindingStatus::UnknownTokenGap,
    )];
    let row = build_m5_component_state_row(descriptor, bindings);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ParityBlockingFinding::UnknownTokenGap { .. })));
}

#[test]
fn unregistered_local_state_blocks() {
    let descriptor = high_salience_descriptor();
    let bindings = vec![bare_binding(
        M5NormalizedState::Stale,
        M5BindingStatus::UnregisteredLocalState,
    )];
    let row = build_m5_component_state_row(descriptor, bindings);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ParityBlockingFinding::UnregisteredLocalState { .. })));
}

#[test]
fn hardcoded_value_and_unresolved_fallback_block() {
    let descriptor = high_salience_descriptor();
    let mut binding = bare_binding(M5NormalizedState::Degraded, M5BindingStatus::Inherited);
    binding.projected_token_group = Some(M5TokenGroup::SyncStatusTokens);
    binding.projected_token_ref = Some(canonical_token_ref(
        M5TokenGroup::SyncStatusTokens,
        M5NormalizedState::Degraded,
    ));
    binding.projected_style_provenance = Some(M5StyleProvenance::ShellTokenInherited);
    binding.projected_cue_policy = Some(M5CuePolicy::NonColorCueRequired);
    binding.projected_non_color_cue = Some(M5AccessibilityCue::IconAndText);
    binding.projected_registry_anchor_ref = Some("registry:anchor:sync:status_surface".to_owned());
    binding.hardcoded_value = Some("#ff0000".to_owned());
    binding.unresolved_token_fallback = Some("token:missing:fallback".to_owned());
    let row = build_m5_component_state_row(descriptor, vec![binding]);
    let tokens: BTreeSet<&str> = row
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("hardcoded_theme_value"));
    assert!(tokens.contains("unresolved_token_fallback"));
}

#[test]
fn color_only_cue_blocks_on_high_salience() {
    let descriptor = high_salience_descriptor();
    let mut binding = bare_binding(M5NormalizedState::PolicyBlocked, M5BindingStatus::Inherited);
    binding.projected_token_group = Some(M5TokenGroup::SyncStatusTokens);
    binding.projected_token_ref = Some(canonical_token_ref(
        M5TokenGroup::SyncStatusTokens,
        M5NormalizedState::PolicyBlocked,
    ));
    binding.projected_style_provenance = Some(M5StyleProvenance::ShellTokenInherited);
    binding.projected_cue_policy = Some(M5CuePolicy::NonColorCueRequired);
    binding.projected_non_color_cue = Some(M5AccessibilityCue::ColorOnly);
    binding.projected_registry_anchor_ref = Some("registry:anchor:sync:status_surface".to_owned());
    let row = build_m5_component_state_row(descriptor, vec![binding]);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ParityBlockingFinding::ColorOnlyCue { .. })));
}

#[test]
fn descriptor_level_findings_fire() {
    let descriptor = M5ComponentStateDescriptor {
        surface_id: "surface:sync.status_surface".to_owned(),
        surface_family: M5SurfaceFamily::SyncStatusSurface,
        descriptor_revision_ref: "rev".to_owned(),
        primary_label_ref: "label".to_owned(),
        registry_anchor_ref: "  ".to_owned(),
        accessibility_note: "  ".to_owned(),
        token_group: M5TokenGroup::SyncStatusTokens,
        style_provenance: M5StyleProvenance::ShellTokenInherited,
        semantic_salience: M5SemanticSalience::SeverityBearing,
        cue_policy: M5CuePolicy::ColorAllowed,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        documented_overrides: vec![],
        registered_in_shared_registry: false,
    };
    let row = build_m5_component_state_row(descriptor, vec![]);
    let tokens: BTreeSet<&str> = row
        .blocking_findings
        .iter()
        .map(|f| f.class_token())
        .collect();
    assert!(tokens.contains("descriptor_missing_registry_anchor"));
    assert!(tokens.contains("missing_accessibility_note"));
    assert!(tokens.contains("missing_non_color_cue_policy"));
    assert!(tokens.contains("surface_not_registered"));
}

#[test]
fn token_ref_drift_blocks() {
    let mut report = seeded_m5_component_state_audit();
    let row = report
        .rows
        .iter_mut()
        .find(|row| row.descriptor.surface_id == "surface:review.pipeline_card")
        .expect("pipeline card row present");
    let binding = row
        .bindings
        .iter_mut()
        .find(|binding| binding.state == M5NormalizedState::PolicyBlocked)
        .expect("policy_blocked binding present");
    binding.projected_token_ref = Some("token:private:policy_blocked".to_owned());
    let rebuilt = build_m5_component_state_row(row.descriptor.clone(), row.bindings.clone());
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ParityBlockingFinding::TokenRefDrift { .. })));
}

#[test]
fn override_drift_blocks() {
    let descriptor = high_salience_descriptor();
    let mut binding = bare_binding(M5NormalizedState::Stale, M5BindingStatus::Inherited);
    binding.projected_token_group = Some(M5TokenGroup::SyncStatusTokens);
    binding.projected_token_ref = Some(canonical_token_ref(
        M5TokenGroup::SyncStatusTokens,
        M5NormalizedState::Stale,
    ));
    binding.projected_style_provenance = Some(M5StyleProvenance::ShellTokenInherited);
    binding.projected_cue_policy = Some(M5CuePolicy::NonColorCueRequired);
    binding.projected_non_color_cue = Some(M5AccessibilityCue::IconAndText);
    binding.projected_registry_anchor_ref = Some("registry:anchor:sync:status_surface".to_owned());
    binding.applied_overrides = vec!["override:undeclared".to_owned()];
    let row = build_m5_component_state_row(descriptor, vec![binding]);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ParityBlockingFinding::OverrideDrift { .. })));
}

#[test]
fn missing_narrowing_reason_blocks() {
    let mut report = seeded_m5_component_state_audit();
    let row = report
        .rows
        .iter_mut()
        .find(|row| row.descriptor.surface_id == "surface:docs_browser.pane")
        .expect("docs browser row present");
    let binding = row
        .bindings
        .iter_mut()
        .find(|binding| binding.binding_status == M5BindingStatus::DeclaredInheritanceGap)
        .expect("declared gap binding present");
    binding.narrowing_reason = None;
    let rebuilt = build_m5_component_state_row(row.descriptor.clone(), row.bindings.clone());
    assert!(rebuilt
        .blocking_findings
        .iter()
        .any(|f| matches!(f, M5ParityBlockingFinding::MissingNarrowingReason { .. })));
}

#[test]
fn missing_required_state_blocks_validation() {
    let mut report = seeded_m5_component_state_audit();
    report.rows[0]
        .bindings
        .retain(|binding| binding.state != M5NormalizedState::Loading);
    let result = validate_m5_component_state_audit(&report);
    assert!(result.is_err());
}

#[test]
fn support_export_quotes_every_surface_id() {
    let report = seeded_m5_component_state_audit();
    let export = M5ComponentStateSupportExport::from_report(
        M5_COMPONENT_STATE_SUPPORT_EXPORT_ID,
        report.clone(),
    );
    assert!(export.case_ids.contains(&report.report_id));
    for row in &report.rows {
        assert!(export.case_ids.contains(&row.descriptor.surface_id));
        assert!(export
            .case_ids
            .contains(&row.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn render_markdown_is_deterministic() {
    let report = seeded_m5_component_state_audit();
    assert_eq!(report.render_markdown(), report.render_markdown());
}
