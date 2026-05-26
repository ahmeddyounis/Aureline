//! Unit tests for the trust-gating lineage projection.

use super::*;

#[allow(clippy::too_many_arguments)]
fn stable_surface(
    surface_id: &str,
    title: &str,
    kind: TrustSurfaceKind,
    declared: GateDecisionClass,
    silent: SilentExecutionPosture,
    explicit_action: bool,
    disclosure_id: &str,
    override_route: OverrideRouteClass,
    override_action_id: &str,
    override_disclosure_id: &str,
    touches_credential_store: bool,
    posture: TrustSupportExportPosture,
) -> TrustSurfaceObservation {
    TrustSurfaceObservation {
        surface_id: surface_id.to_owned(),
        title: title.to_owned(),
        surface_kind: kind,
        declared_gate_decision: declared,
        silent_execution_posture: silent,
        explicit_user_action_required: explicit_action,
        disclosure_id: disclosure_id.to_owned(),
        override_route,
        override_action_id: override_action_id.to_owned(),
        override_disclosure_id: override_disclosure_id.to_owned(),
        touches_credential_store,
        support_export: TrustSupportExportInputs::metadata_safe_baseline(posture),
        captured_at: "mono:1700000200".to_owned(),
    }
}

fn trusted_corpus_inputs() -> TrustGatingInputs {
    TrustGatingInputs {
        workspace_ref: "workspace-trusted-0001".to_owned(),
        producer_ref: "producer-aureline-0001".to_owned(),
        corpus_ref: "trust-gating-corpus-trusted-0001".to_owned(),
        captured_at: "mono:1700000200".to_owned(),
        workspace_trust_posture: WorkspaceTrustPosture::Trusted,
        surfaces: vec![
            stable_surface(
                "tasks.run",
                "Workspace task runner",
                TrustSurfaceKind::Tasks,
                GateDecisionClass::AllowAfterExplicitGrant,
                SilentExecutionPosture::ExplicitUserActionRequired,
                true,
                "disclosure.tasks.run",
                OverrideRouteClass::DisclosedWithAudit,
                "tasks.override",
                "disclosure.tasks.override",
                false,
                TrustSupportExportPosture::MetadataSafeExport,
            ),
            stable_surface(
                "terminal.session",
                "Workspace terminal session",
                TrustSurfaceKind::Terminal,
                GateDecisionClass::AllowAfterExplicitGrant,
                SilentExecutionPosture::ExplicitUserActionRequired,
                true,
                "disclosure.terminal.session",
                OverrideRouteClass::DisclosedSession,
                "terminal.override",
                "disclosure.terminal.override",
                true,
                TrustSupportExportPosture::MetadataSafeExport,
            ),
            stable_surface(
                "debug.launch",
                "Debug launch configuration",
                TrustSurfaceKind::Debug,
                GateDecisionClass::AllowAfterExplicitGrant,
                SilentExecutionPosture::ExplicitUserActionRequired,
                true,
                "disclosure.debug.launch",
                OverrideRouteClass::None,
                "",
                "",
                false,
                TrustSupportExportPosture::MetadataSafeExport,
            ),
            stable_surface(
                "ai.apply",
                "AI apply",
                TrustSurfaceKind::AiApply,
                GateDecisionClass::AllowAfterExplicitGrant,
                SilentExecutionPosture::ExplicitUserActionRequired,
                true,
                "disclosure.ai.apply",
                OverrideRouteClass::None,
                "",
                "",
                false,
                TrustSupportExportPosture::MetadataSafeExport,
            ),
            stable_surface(
                "extensions.privileged",
                "Privileged extension activation",
                TrustSurfaceKind::PrivilegedExtension,
                GateDecisionClass::AllowAfterExplicitGrant,
                SilentExecutionPosture::ExplicitUserActionRequired,
                true,
                "disclosure.extensions.privileged",
                OverrideRouteClass::DisclosedOneTime,
                "extensions.override",
                "disclosure.extensions.override",
                true,
                TrustSupportExportPosture::MetadataSafeExport,
            ),
        ],
    }
}

#[test]
fn clean_inputs_project_stable_record() {
    let inputs = trusted_corpus_inputs();
    let record = project_trust_gating_lineage("posture.clean", &inputs);

    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(record.is_support_export_safe());
    assert_eq!(record.record_kind, TRUST_GATING_LINEAGE_RECORD_KIND);
    assert_eq!(record.schema_ref, TRUST_GATING_LINEAGE_SCHEMA_REF);
    assert!(record.surface_coverage.all_required_surfaces_present);
    assert_eq!(record.surface_coverage.surface_rows.len(), 5);
    assert!(
        record
            .gate_decision_truth
            .no_unconditional_allow_outside_trusted
    );
    assert!(
        record
            .silent_execution_honesty
            .all_grant_surfaces_require_explicit_user_action
    );
    assert!(record.override_route_honesty.all_override_routes_disclosed);
    assert!(
        record
            .support_export_honesty
            .all_credential_surfaces_have_safe_posture
    );
    assert_eq!(record.inspection_hooks.len(), 6);
    assert!(record.producer_attribution.integrity_hash.starts_with("tgl:"));
}

#[test]
fn missing_required_surface_narrows_record() {
    let mut inputs = trusted_corpus_inputs();
    inputs
        .surfaces
        .retain(|surface| surface.surface_kind != TrustSurfaceKind::Debug);

    let record = project_trust_gating_lineage("posture.missing_debug", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::RequiredTrustSurfaceMissing));
}

#[test]
fn restricted_workspace_with_unconditional_narrows_record() {
    let mut inputs = trusted_corpus_inputs();
    inputs.workspace_trust_posture = WorkspaceTrustPosture::Restricted;
    inputs.surfaces[0].declared_gate_decision = GateDecisionClass::AllowUnconditional;

    let record = project_trust_gating_lineage("posture.restricted_unconditional", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::RestrictedWorkspaceAllowsUnconditional));
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::UnconditionalAllowWithoutTrustedPosture));
}

#[test]
fn pending_workspace_with_grant_narrows_record() {
    let mut inputs = trusted_corpus_inputs();
    inputs.workspace_trust_posture = WorkspaceTrustPosture::PendingEvaluation;
    // The grant decision is illegal on a pending workspace.
    for surface in &mut inputs.surfaces {
        surface.declared_gate_decision = GateDecisionClass::BlockPendingTrustDecision;
        surface.silent_execution_posture = SilentExecutionPosture::CannotFireSilently;
        surface.disclosure_id = "".to_owned();
        surface.explicit_user_action_required = false;
        surface.override_route = OverrideRouteClass::None;
        surface.override_action_id = "".to_owned();
        surface.override_disclosure_id = "".to_owned();
    }
    inputs.surfaces[0].declared_gate_decision = GateDecisionClass::AllowAfterExplicitGrant;
    inputs.surfaces[0].silent_execution_posture = SilentExecutionPosture::ExplicitUserActionRequired;
    inputs.surfaces[0].disclosure_id = "disclosure.tasks.run".to_owned();
    inputs.surfaces[0].explicit_user_action_required = true;

    let record = project_trust_gating_lineage("posture.pending_grant", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::PendingWorkspaceAllowsExecution));
}

#[test]
fn grant_without_disclosure_narrows_record() {
    let mut inputs = trusted_corpus_inputs();
    inputs.surfaces[0].disclosure_id = "".to_owned();

    let record = project_trust_gating_lineage("posture.no_disclosure", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::SilentGrantWithoutDisclosure));
}

#[test]
fn grant_without_explicit_user_action_narrows_record() {
    let mut inputs = trusted_corpus_inputs();
    inputs.surfaces[0].explicit_user_action_required = false;

    let record = project_trust_gating_lineage("posture.silent_grant", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::SilentGrantWithoutDisclosure));
}

#[test]
fn override_route_without_disclosure_narrows_record() {
    let mut inputs = trusted_corpus_inputs();
    inputs.surfaces[0].override_disclosure_id = "".to_owned();

    let record = project_trust_gating_lineage("posture.override_undisclosed", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::OverrideRouteUndisclosed));
}

#[test]
fn read_only_surface_missing_posture_narrows_record() {
    let mut inputs = trusted_corpus_inputs();
    inputs.surfaces[0].declared_gate_decision = GateDecisionClass::AllowReadOnly;
    // Posture stays ExplicitUserActionRequired, which mismatches.
    let record = project_trust_gating_lineage("posture.read_only_mismatch", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::ReadOnlyMissingPosture));
}

#[test]
fn support_export_dropping_fields_narrows_record() {
    let mut inputs = trusted_corpus_inputs();
    inputs.surfaces[0].support_export.includes_gate_decision = false;

    let record = project_trust_gating_lineage("posture.support_dropped", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::SupportExportFieldsDropped));
}

#[test]
fn credential_surface_local_only_narrows_record() {
    let mut inputs = trusted_corpus_inputs();
    let credential = inputs
        .surfaces
        .iter_mut()
        .find(|surface| surface.touches_credential_store)
        .expect("credential-touching surface seeded");
    credential.support_export.posture = TrustSupportExportPosture::LocalOnly;

    let record = project_trust_gating_lineage("posture.credential_local_only", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::SupportExportPostureUnsafe));
}

#[test]
fn support_export_redaction_unsafe_narrows_record() {
    let mut inputs = trusted_corpus_inputs();
    inputs.surfaces[0].support_export.raw_secrets_excluded = false;

    let record = project_trust_gating_lineage("posture.raw_secret", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::SupportExportRedactionUnsafe));
}

#[test]
fn missing_inspection_hook_narrows_record() {
    let inputs = trusted_corpus_inputs();
    let mut hooks = default_trust_gating_inspection_hooks();
    for hook in &mut hooks {
        if hook.hook_class == TrustInspectionHookClass::ReviewGrantScope {
            hook.available = false;
        }
    }

    let record = project_trust_gating_lineage_with_hooks("posture.no_review", &inputs, hooks);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::InspectionHookUnavailable));
}

#[test]
fn empty_workspace_ref_narrows_record() {
    let mut inputs = trusted_corpus_inputs();
    inputs.workspace_ref = "".to_owned();

    let record = project_trust_gating_lineage("posture.empty_workspace", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&TrustGatingLineageNarrowReason::LineageExportUnsafe));
}

#[test]
fn lines_projection_renders_required_sections() {
    let inputs = trusted_corpus_inputs();
    let record = project_trust_gating_lineage("posture.lines", &inputs);
    let lines = trust_gating_lineage_lines(&record);

    assert!(lines.iter().any(|line| line.contains("Trust-gating lineage")));
    assert!(lines.iter().any(|line| line.contains("surface_coverage")));
    assert!(lines.iter().any(|line| line == "Surface rows:"));
    assert!(lines.iter().any(|line| line.contains("Gate decision truth")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Silent execution honesty")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Override route honesty")));
    assert!(lines.iter().any(|line| line.contains("Support-export honesty")));
    assert!(lines.iter().any(|line| line == "Inspection hooks:"));
}

#[test]
fn record_round_trips_through_json() {
    let inputs = trusted_corpus_inputs();
    let record = project_trust_gating_lineage("posture.round_trip", &inputs);
    let serialized = serde_json::to_string(&record).expect("record must serialize");
    let parsed: TrustGatingLineageRecord =
        serde_json::from_str(&serialized).expect("record must deserialize");
    assert_eq!(record, parsed);
}
