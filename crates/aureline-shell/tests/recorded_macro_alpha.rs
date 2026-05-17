//! Fixture-driven coverage for the recorded-macro alpha page.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_shell::macros::{
    AttributionSurfaceClass, AuditEventClass, RecordedMacroAlphaPage, ReplayDispositionClass,
    StepCommandLineageClass, TrustGateClass, WriteClass,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/commands/recorded_macro_alpha/page.json")
}

fn load_page() -> RecordedMacroAlphaPage {
    let text = fs::read_to_string(fixture_path()).expect("read recorded-macro alpha fixture");
    serde_json::from_str(&text).expect("parse recorded-macro alpha fixture")
}

#[test]
fn alpha_fixture_validates() {
    let page = load_page();
    let report = page.validate();
    assert!(
        report.passed,
        "recorded-macro alpha fixture failed validation: {:#?}",
        report.findings
    );
}

#[test]
fn fixture_covers_all_five_replay_dispositions() {
    let page = load_page();
    let report = page.validate();
    for class in [
        ReplayDispositionClass::ProceedLocalEditorOnly,
        ReplayDispositionClass::PreviewRequiredBeforeApply,
        ReplayDispositionClass::DowngradedToObserverNoMutation,
        ReplayDispositionClass::PromotedToDeclarativeRecipe,
        ReplayDispositionClass::DeniedUnsafeReplay,
    ] {
        assert!(
            report.coverage.replay_disposition_classes.contains(&class),
            "missing replay-disposition coverage: {class:?}"
        );
    }
}

#[test]
fn fixture_covers_required_audit_events() {
    let page = load_page();
    let report = page.validate();
    for class in [
        AuditEventClass::MacroRecorded,
        AuditEventClass::MacroReplayRequested,
        AuditEventClass::MacroReplayAdmitted,
        AuditEventClass::MacroReplayDenied,
        AuditEventClass::MacroReplayPreviewRequired,
        AuditEventClass::MacroReplayDowngraded,
        AuditEventClass::MacroReplayPromotedToRecipe,
        AuditEventClass::MacroAttributionMinted,
    ] {
        assert!(
            report.coverage.audit_event_classes.contains(&class),
            "missing audit-event coverage: {class:?}"
        );
    }
}

#[test]
fn fixture_covers_attribution_surfaces() {
    let page = load_page();
    let report = page.validate();
    for class in [
        AttributionSurfaceClass::SupportExport,
        AttributionSurfaceClass::ActivityHistory,
        AttributionSurfaceClass::AdminAuditExport,
    ] {
        assert!(
            report.coverage.attribution_surface_classes.contains(&class),
            "missing attribution-surface coverage: {class:?}"
        );
    }
}

#[test]
fn no_definition_observes_managed_only_denied_trust_gate() {
    let page = load_page();
    for def in &page.definitions {
        assert_ne!(
            def.trust_gate,
            TrustGateClass::ManagedOnlyDenied,
            "definition {} must not record a managed_only_denied trust gate",
            def.definition_id
        );
    }
}

#[test]
fn every_definition_mints_support_and_activity_attribution() {
    let page = load_page();
    for def in &page.definitions {
        assert!(
            def.support_attribution_minted,
            "definition {} must mint a support-export attribution",
            def.definition_id
        );
        assert!(
            def.activity_attribution_minted,
            "definition {} must mint an activity-history attribution",
            def.definition_id
        );
    }
}

#[test]
fn every_disposition_resolves_to_a_definition() {
    let page = load_page();
    let def_ids: std::collections::BTreeSet<&str> = page
        .definitions
        .iter()
        .map(|def| def.definition_id.as_str())
        .collect();
    for disp in &page.replay_dispositions {
        assert!(
            def_ids.contains(disp.definition_ref.as_str()),
            "disposition {} must resolve to a definition",
            disp.disposition_id
        );
    }
}

#[test]
fn support_export_omits_raw_fields_and_preserves_attribution_lineage() {
    let page = load_page();
    let projection = page.support_export_projection();
    let json = serde_json::to_string(&projection).expect("projection serializes");
    assert_eq!(
        projection.record_kind,
        "recorded_macro_alpha_support_export"
    );
    assert!(!json.contains("raw_keystroke_bytes"));
    assert!(!json.contains("raw_buffer_bytes"));
    assert!(!json.contains("raw_shell_fragment"));
    assert!(!json.contains("raw_credential_present"));
    assert!(!json.contains("raw_payload_exported"));
    assert_eq!(
        projection.definition_summaries.len(),
        page.definitions.len()
    );
    assert_eq!(
        projection.disposition_summaries.len(),
        page.replay_dispositions.len()
    );
    assert_eq!(projection.audit_summaries.len(), page.audit_events.len());
    assert_eq!(
        projection.attribution_summaries.len(),
        page.attributions.len()
    );
    for summary in &projection.definition_summaries {
        assert!(
            !summary.command_ids.is_empty(),
            "support export must preserve at least one command id per definition"
        );
    }
}

#[test]
fn unmapped_keystroke_step_forces_a_non_proceed_disposition() {
    let page = load_page();
    let def_with_unmapped = page
        .definitions
        .iter()
        .find(|def| {
            def.steps.iter().any(|step| {
                step.step_command_lineage == StepCommandLineageClass::UnmappedKeystrokeDenied
            })
        })
        .expect("fixture must include a definition with an unmapped-keystroke step");
    let disposition = page
        .replay_dispositions
        .iter()
        .find(|disp| disp.definition_ref == def_with_unmapped.definition_id)
        .expect("unmapped-keystroke definition must have a disposition");
    assert!(
        !disposition.disposition.is_silent_proceed(),
        "unmapped-keystroke definition must not dispatch on the silent proceed lane"
    );
}

#[test]
fn dropping_proceed_disposition_breaks_required_coverage_after_edit() {
    let mut page = load_page();
    page.replay_dispositions
        .retain(|disp| disp.disposition != ReplayDispositionClass::ProceedLocalEditorOnly);
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id
            == "recorded_macro_alpha.coverage_replay_disposition_missing"));
}

#[test]
fn dropping_support_attribution_breaks_definition_invariant_after_edit() {
    let mut page = load_page();
    if let Some(def) = page.definitions.first_mut() {
        def.support_attribution_minted = false;
    }
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id
            == "recorded_macro_alpha.definition_support_attribution_missing"));
}

#[test]
fn editing_a_single_buffer_safe_definition_to_include_denied_write_class_is_refused_after_edit() {
    let mut page = load_page();
    let proceed_def = page
        .definitions
        .iter_mut()
        .find(|def| def.definition_id == "recorded_macro_alpha.def.normalize_imports.01")
        .expect("proceed-lane definition present");
    proceed_def
        .steps
        .first_mut()
        .expect("definition has steps")
        .write_classes
        .push(WriteClass::NetworkMutationDenied);
    let report = page.validate();
    assert!(!report.passed);
    assert!(
        report
            .findings
            .iter()
            .any(|finding| finding.check_id
                == "recorded_macro_alpha.definition_silent_proceed_denied")
    );
}

#[test]
fn editing_an_audit_event_to_drop_disposition_ref_is_refused_after_edit() {
    let mut page = load_page();
    let event = page
        .audit_events
        .iter_mut()
        .find(|event| event.event_class == AuditEventClass::MacroReplayPreviewRequired)
        .expect("preview-required audit event present");
    event.replay_disposition_ref = None;
    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.check_id
            == "recorded_macro_alpha.audit_event_disposition_ref_unknown"));
}

#[test]
fn editing_a_disposition_to_drop_its_required_reason_is_refused_after_edit() {
    let mut page = load_page();
    let disp = page
        .replay_dispositions
        .iter_mut()
        .find(|disp| disp.disposition == ReplayDispositionClass::DeniedUnsafeReplay)
        .expect("denied disposition present");
    disp.denial_reason_label = None;
    let report = page.validate();
    assert!(!report.passed);
    assert!(report.findings.iter().any(
        |finding| finding.check_id == "recorded_macro_alpha.disposition_denial_reason_missing"
    ));
}
