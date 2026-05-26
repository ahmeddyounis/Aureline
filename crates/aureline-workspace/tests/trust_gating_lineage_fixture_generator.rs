//! Fixture generator helper for the trust-gating lineage replay gate.
//!
//! Only runs when `TRUST_GATING_LINEAGE_GEN_FIXTURES=1` is set in the
//! environment. Emits the canonical fixture JSON files into
//! `fixtures/workspace/m4/trust_gating_lineage/` so the replay gate has
//! a deterministic, checked-in corpus.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    default_trust_gating_inspection_hooks, project_trust_gating_lineage_with_hooks,
    GateDecisionClass, OverrideRouteClass, SilentExecutionPosture, TrustGatingInputs,
    TrustGatingLineageRecord, TrustInspectionHook, TrustInspectionHookClass,
    TrustSupportExportInputs, TrustSupportExportPosture, TrustSurfaceKind,
    TrustSurfaceObservation, WorkspaceTrustPosture,
};
use serde::Serialize;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/m4/trust_gating_lineage")
}

fn full_corpus(captured_at: &str) -> Vec<TrustSurfaceObservation> {
    vec![
        TrustSurfaceObservation {
            surface_id: "tasks.run".to_owned(),
            title: "Workspace task runner".to_owned(),
            surface_kind: TrustSurfaceKind::Tasks,
            declared_gate_decision: GateDecisionClass::AllowAfterExplicitGrant,
            silent_execution_posture: SilentExecutionPosture::ExplicitUserActionRequired,
            explicit_user_action_required: true,
            disclosure_id: "disclosure.tasks.run".to_owned(),
            override_route: OverrideRouteClass::DisclosedWithAudit,
            override_action_id: "tasks.override".to_owned(),
            override_disclosure_id: "disclosure.tasks.override".to_owned(),
            touches_credential_store: false,
            support_export: TrustSupportExportInputs::metadata_safe_baseline(
                TrustSupportExportPosture::MetadataSafeExport,
            ),
            captured_at: captured_at.to_owned(),
        },
        TrustSurfaceObservation {
            surface_id: "terminal.session".to_owned(),
            title: "Workspace terminal session".to_owned(),
            surface_kind: TrustSurfaceKind::Terminal,
            declared_gate_decision: GateDecisionClass::AllowAfterExplicitGrant,
            silent_execution_posture: SilentExecutionPosture::ExplicitUserActionRequired,
            explicit_user_action_required: true,
            disclosure_id: "disclosure.terminal.session".to_owned(),
            override_route: OverrideRouteClass::DisclosedSession,
            override_action_id: "terminal.override".to_owned(),
            override_disclosure_id: "disclosure.terminal.override".to_owned(),
            touches_credential_store: true,
            support_export: TrustSupportExportInputs::metadata_safe_baseline(
                TrustSupportExportPosture::MetadataSafeExport,
            ),
            captured_at: captured_at.to_owned(),
        },
        TrustSurfaceObservation {
            surface_id: "debug.launch".to_owned(),
            title: "Debug launch configuration".to_owned(),
            surface_kind: TrustSurfaceKind::Debug,
            declared_gate_decision: GateDecisionClass::AllowAfterExplicitGrant,
            silent_execution_posture: SilentExecutionPosture::ExplicitUserActionRequired,
            explicit_user_action_required: true,
            disclosure_id: "disclosure.debug.launch".to_owned(),
            override_route: OverrideRouteClass::None,
            override_action_id: "".to_owned(),
            override_disclosure_id: "".to_owned(),
            touches_credential_store: false,
            support_export: TrustSupportExportInputs::metadata_safe_baseline(
                TrustSupportExportPosture::MetadataSafeExport,
            ),
            captured_at: captured_at.to_owned(),
        },
        TrustSurfaceObservation {
            surface_id: "ai.apply".to_owned(),
            title: "AI apply pipeline".to_owned(),
            surface_kind: TrustSurfaceKind::AiApply,
            declared_gate_decision: GateDecisionClass::AllowAfterExplicitGrant,
            silent_execution_posture: SilentExecutionPosture::ExplicitUserActionRequired,
            explicit_user_action_required: true,
            disclosure_id: "disclosure.ai.apply".to_owned(),
            override_route: OverrideRouteClass::None,
            override_action_id: "".to_owned(),
            override_disclosure_id: "".to_owned(),
            touches_credential_store: false,
            support_export: TrustSupportExportInputs::metadata_safe_baseline(
                TrustSupportExportPosture::MetadataSafeExport,
            ),
            captured_at: captured_at.to_owned(),
        },
        TrustSurfaceObservation {
            surface_id: "extensions.privileged".to_owned(),
            title: "Privileged extension activation".to_owned(),
            surface_kind: TrustSurfaceKind::PrivilegedExtension,
            declared_gate_decision: GateDecisionClass::AllowAfterExplicitGrant,
            silent_execution_posture: SilentExecutionPosture::ExplicitUserActionRequired,
            explicit_user_action_required: true,
            disclosure_id: "disclosure.extensions.privileged".to_owned(),
            override_route: OverrideRouteClass::DisclosedOneTime,
            override_action_id: "extensions.override".to_owned(),
            override_disclosure_id: "disclosure.extensions.override".to_owned(),
            touches_credential_store: true,
            support_export: TrustSupportExportInputs::metadata_safe_baseline(
                TrustSupportExportPosture::MetadataSafeExport,
            ),
            captured_at: captured_at.to_owned(),
        },
    ]
}

fn read_only_corpus(captured_at: &str) -> Vec<TrustSurfaceObservation> {
    let mut surfaces = full_corpus(captured_at);
    for surface in &mut surfaces {
        surface.declared_gate_decision = GateDecisionClass::AllowReadOnly;
        surface.silent_execution_posture = SilentExecutionPosture::ReadOnlyNoMutation;
        surface.explicit_user_action_required = false;
        surface.disclosure_id = "".to_owned();
        surface.override_route = OverrideRouteClass::None;
        surface.override_action_id = "".to_owned();
        surface.override_disclosure_id = "".to_owned();
    }
    surfaces
}

fn restricted_corpus(captured_at: &str) -> Vec<TrustSurfaceObservation> {
    let mut surfaces = full_corpus(captured_at);
    for surface in &mut surfaces {
        surface.declared_gate_decision = GateDecisionClass::BlockUntilRepair;
        surface.silent_execution_posture = SilentExecutionPosture::CannotFireSilently;
        surface.explicit_user_action_required = false;
        surface.disclosure_id = "".to_owned();
        surface.override_route = OverrideRouteClass::None;
        surface.override_action_id = "".to_owned();
        surface.override_disclosure_id = "".to_owned();
    }
    surfaces
}

fn pending_corpus(captured_at: &str) -> Vec<TrustSurfaceObservation> {
    let mut surfaces = full_corpus(captured_at);
    for surface in &mut surfaces {
        surface.declared_gate_decision = GateDecisionClass::BlockPendingTrustDecision;
        surface.silent_execution_posture = SilentExecutionPosture::CannotFireSilently;
        surface.explicit_user_action_required = false;
        surface.disclosure_id = "".to_owned();
        surface.override_route = OverrideRouteClass::None;
        surface.override_action_id = "".to_owned();
        surface.override_disclosure_id = "".to_owned();
    }
    surfaces
}

fn base_inputs(
    workspace_ref: &str,
    corpus_ref: &str,
    captured_at: &str,
    posture: WorkspaceTrustPosture,
    surfaces: Vec<TrustSurfaceObservation>,
) -> TrustGatingInputs {
    TrustGatingInputs {
        workspace_ref: workspace_ref.to_owned(),
        producer_ref: "producer-aureline-fixtures-0001".to_owned(),
        corpus_ref: corpus_ref.to_owned(),
        captured_at: captured_at.to_owned(),
        workspace_trust_posture: posture,
        surfaces,
    }
}

#[derive(Debug, Serialize)]
struct FixtureEnvelope<'a> {
    posture_id: &'a str,
    inputs: &'a TrustGatingInputs,
    inspection_hooks: &'a Vec<TrustInspectionHook>,
    expected: &'a TrustGatingLineageRecord,
}

fn write_fixture(
    name: &str,
    posture_id: &str,
    inputs: TrustGatingInputs,
    inspection_hooks: Vec<TrustInspectionHook>,
) {
    let record =
        project_trust_gating_lineage_with_hooks(posture_id, &inputs, inspection_hooks.clone());
    let envelope = FixtureEnvelope {
        posture_id,
        inputs: &inputs,
        inspection_hooks: &inspection_hooks,
        expected: &record,
    };
    let path = fixtures_dir().join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(&envelope).expect("envelope serializes");
    std::fs::write(&path, json + "\n").expect("fixture write");
    eprintln!("wrote {}", path.display());
}

#[test]
fn generate_fixtures() {
    if std::env::var("TRUST_GATING_LINEAGE_GEN_FIXTURES")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    std::fs::create_dir_all(fixtures_dir()).expect("ensure fixture dir");

    // Trusted workspace: every privileged surface is gated by an
    // allow-after-explicit-grant decision.
    let trusted_inputs = base_inputs(
        "workspace-rust-service-0001",
        "trust-gating-corpus-trusted-0001",
        "mono:1700000200",
        WorkspaceTrustPosture::Trusted,
        full_corpus("mono:1700000200"),
    );
    write_fixture(
        "trusted_grant_stable",
        "posture:trusted_grant",
        trusted_inputs,
        default_trust_gating_inspection_hooks(),
    );

    // Restricted workspace: every privileged surface is blocked until
    // trust is repaired.
    let restricted_inputs = base_inputs(
        "workspace-rust-service-0001",
        "trust-gating-corpus-restricted-0001",
        "mono:1700000210",
        WorkspaceTrustPosture::Restricted,
        restricted_corpus("mono:1700000210"),
    );
    write_fixture(
        "restricted_blocked_stable",
        "posture:restricted_blocked",
        restricted_inputs,
        default_trust_gating_inspection_hooks(),
    );

    // Pending evaluation: every privileged surface is blocked until
    // the trust decision lands.
    let pending_inputs = base_inputs(
        "workspace-rust-service-0001",
        "trust-gating-corpus-pending-0001",
        "mono:1700000220",
        WorkspaceTrustPosture::PendingEvaluation,
        pending_corpus("mono:1700000220"),
    );
    write_fixture(
        "pending_blocked_stable",
        "posture:pending_blocked",
        pending_inputs,
        default_trust_gating_inspection_hooks(),
    );

    // Restricted workspace, read-only allowance: every privileged
    // surface stays in inspect-only mode while the workspace is
    // restricted.
    let read_only_inputs = base_inputs(
        "workspace-rust-service-0001",
        "trust-gating-corpus-read-only-0001",
        "mono:1700000230",
        WorkspaceTrustPosture::Restricted,
        read_only_corpus("mono:1700000230"),
    );
    write_fixture(
        "restricted_read_only_stable",
        "posture:restricted_read_only",
        read_only_inputs,
        default_trust_gating_inspection_hooks(),
    );

    // Narrowed: same trusted corpus but the review-grant-scope hook
    // is unavailable.
    let narrowed_inputs = base_inputs(
        "workspace-rust-service-0001",
        "trust-gating-corpus-narrowed-0001",
        "mono:1700000240",
        WorkspaceTrustPosture::Trusted,
        full_corpus("mono:1700000240"),
    );
    let mut narrowed_hooks = default_trust_gating_inspection_hooks();
    for hook in &mut narrowed_hooks {
        if hook.hook_class == TrustInspectionHookClass::ReviewGrantScope {
            hook.available = false;
            hook.disclosure = "Review-grant-scope unavailable on this posture.".to_owned();
        }
    }
    write_fixture(
        "missing_review_grant_hook_narrowed",
        "posture:missing_review_grant_hook",
        narrowed_inputs,
        narrowed_hooks,
    );
}
