//! Fixture generator helper for the state-root certification lineage
//! replay gate.
//!
//! Only runs when `STATE_ROOT_CERTIFICATION_LINEAGE_GEN_FIXTURES=1`
//! is set in the environment. Emits the canonical fixture JSON files
//! into `fixtures/workspace/m4/state_root_certification_lineage/` so
//! the replay gate has a deterministic, checked-in corpus.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    default_state_root_inspection_hooks, project_state_root_certification_lineage_with_hooks,
    AuditFindingClass, AuditRedactionClass, AuditRerunPosture, AuditSurfaceKind,
    AuditSurfaceObservation, ClaimedStableProfile, ResourceAuditObservation,
    StateRootCertificationInputs, StateRootCertificationLineageRecord, StateRootInspectionHook,
    StateRootInspectionHookClass, StateRootResourceKind, StateRootSupportExportInputs,
    StateRootSupportExportPosture,
};
use serde::Serialize;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/m4/state_root_certification_lineage")
}

fn support() -> StateRootSupportExportInputs {
    StateRootSupportExportInputs::metadata_safe_baseline(
        StateRootSupportExportPosture::MetadataSafeExport,
    )
}

#[allow(clippy::too_many_arguments)]
fn make_audit(
    resource_id: &str,
    class: StateRootResourceKind,
    storage_class_ref: &str,
    finding: AuditFindingClass,
    audit_disclosure: Option<&str>,
    redaction: AuditRedactionClass,
    redaction_disclosure: Option<&str>,
    mutates_state: bool,
    finding_code: &str,
    captured_at: &str,
) -> ResourceAuditObservation {
    let cleanup_refs = if finding.requires_cleanup_precondition() {
        vec!["cleanup:settings_panel".to_owned()]
    } else {
        Vec::new()
    };
    let hook_refs = if finding.requires_cleanup_precondition() {
        vec!["state_root_certification.compare_before_cleanup".to_owned()]
    } else {
        Vec::new()
    };
    ResourceAuditObservation {
        resource_id: resource_id.to_owned(),
        resource_class: class,
        resource_ref: format!("state-root:{resource_id}"),
        storage_class_ref: storage_class_ref.to_owned(),
        audit_finding: finding,
        audit_disclosure_ref: audit_disclosure.map(str::to_owned),
        audit_transaction_id: format!("tx.{resource_id}.aud"),
        finding_code: finding_code.to_owned(),
        preserves_restore_provenance: true,
        preserves_encoding_fidelity: true,
        preserves_trust_state: true,
        preserves_lineage_refs: true,
        rerun_posture: if mutates_state {
            AuditRerunPosture::ExplicitUserActionRequired
        } else {
            AuditRerunPosture::TerminalNoFurtherRun
        },
        mutates_state,
        commit_action_id: if mutates_state {
            format!("action.{resource_id}.aud.commit")
        } else {
            String::new()
        },
        commit_disclosure_id: if mutates_state {
            format!("disclosure.{resource_id}.aud.commit")
        } else {
            String::new()
        },
        cleanup_surface_refs: cleanup_refs,
        inspection_hook_refs: hook_refs,
        redaction_class: redaction,
        redaction_disclosure_ref: redaction_disclosure.map(str::to_owned),
        support_export: support(),
        captured_at: captured_at.to_owned(),
    }
}

fn make_surface(kind: AuditSurfaceKind, captured_at: &str) -> AuditSurfaceObservation {
    AuditSurfaceObservation {
        audit_surface_id: format!("surf.{}", kind.as_str()),
        label: kind.as_str().to_owned(),
        audit_surface_kind: kind,
        reachable: true,
        preserves_lineage_refs: true,
        preserves_trust_state: true,
        discloses_non_clean_findings: true,
        captured_at: captured_at.to_owned(),
    }
}

fn baseline_resource_audits(captured_at: &str) -> Vec<ResourceAuditObservation> {
    vec![
        make_audit(
            "envelope.0",
            StateRootResourceKind::PersistentStateEnvelope,
            "cache:durable_workspace_state",
            AuditFindingClass::AuditClean,
            None,
            AuditRedactionClass::MetadataOnly,
            None,
            false,
            "WS-AUD-0001",
            captured_at,
        ),
        make_audit(
            "workspace-state.0",
            StateRootResourceKind::WorkspaceStateRoot,
            "cache:durable_workspace_state",
            AuditFindingClass::AuditClean,
            None,
            AuditRedactionClass::MetadataOnly,
            None,
            false,
            "WS-AUD-0002",
            captured_at,
        ),
        make_audit(
            "profile.0",
            StateRootResourceKind::ProfileRoot,
            "cache:durable_workspace_state",
            AuditFindingClass::AuditClean,
            None,
            AuditRedactionClass::MetadataOnly,
            None,
            false,
            "WS-AUD-0003",
            captured_at,
        ),
        make_audit(
            "recent-work.0",
            StateRootResourceKind::RecentWorkRoot,
            "cache:durable_workspace_state",
            AuditFindingClass::AuditClean,
            None,
            AuditRedactionClass::MetadataOnly,
            None,
            false,
            "WS-AUD-0004",
            captured_at,
        ),
        make_audit(
            "local-history.0",
            StateRootResourceKind::LocalHistoryRoot,
            "cache:local_history",
            AuditFindingClass::AuditClean,
            None,
            AuditRedactionClass::MetadataOnly,
            None,
            false,
            "WS-AUD-0005",
            captured_at,
        ),
        make_audit(
            "restore-checkpoint.0",
            StateRootResourceKind::RestoreCheckpointRoot,
            "cache:recovery_checkpoint",
            AuditFindingClass::AuditClean,
            None,
            AuditRedactionClass::MetadataOnly,
            None,
            false,
            "WS-AUD-0006",
            captured_at,
        ),
        make_audit(
            "cache-governance.0",
            StateRootResourceKind::CacheGovernanceRoot,
            "cache:local_disk_cache",
            AuditFindingClass::AuditClean,
            None,
            AuditRedactionClass::MetadataOnly,
            None,
            false,
            "WS-AUD-0007",
            captured_at,
        ),
    ]
}

fn baseline_audit_surfaces(captured_at: &str) -> Vec<AuditSurfaceObservation> {
    vec![
        make_surface(AuditSurfaceKind::StorageDisciplineOverview, captured_at),
        make_surface(AuditSurfaceKind::CacheGovernanceInspector, captured_at),
        make_surface(AuditSurfaceKind::StateRootAuditPanel, captured_at),
        make_surface(AuditSurfaceKind::CleanupInventoryAudit, captured_at),
        make_surface(AuditSurfaceKind::EvictionRuleAudit, captured_at),
        make_surface(AuditSurfaceKind::HeadlessAuditCli, captured_at),
        make_surface(AuditSurfaceKind::SupportExportAuditSection, captured_at),
    ]
}

fn base_inputs(
    workspace_ref: &str,
    corpus_ref: &str,
    captured_at: &str,
    profile: ClaimedStableProfile,
    resource_audits: Vec<ResourceAuditObservation>,
    audit_surfaces: Vec<AuditSurfaceObservation>,
) -> StateRootCertificationInputs {
    StateRootCertificationInputs {
        workspace_ref: workspace_ref.to_owned(),
        producer_ref: "producer-aureline-fixtures-0001".to_owned(),
        corpus_ref: corpus_ref.to_owned(),
        captured_at: captured_at.to_owned(),
        claimed_profile: profile,
        resource_audits,
        audit_surfaces,
    }
}

#[derive(Debug, Serialize)]
struct FixtureEnvelope<'a> {
    posture_id: &'a str,
    inputs: &'a StateRootCertificationInputs,
    inspection_hooks: &'a Vec<StateRootInspectionHook>,
    expected: &'a StateRootCertificationLineageRecord,
}

fn write_fixture(
    name: &str,
    posture_id: &str,
    inputs: StateRootCertificationInputs,
    inspection_hooks: Vec<StateRootInspectionHook>,
) {
    let record = project_state_root_certification_lineage_with_hooks(
        posture_id,
        &inputs,
        inspection_hooks.clone(),
    );
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
    if std::env::var("STATE_ROOT_CERTIFICATION_LINEAGE_GEN_FIXTURES")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    std::fs::create_dir_all(fixtures_dir()).expect("ensure fixture dir");

    // Baseline Stable: every required resource class + every required
    // audit surface, all audits clean, no cleanup pressure.
    write_fixture(
        "baseline_state_root_certification_stable",
        "posture:baseline_state_root_certification",
        base_inputs(
            "workspace-rust-service-0001",
            "state-root-certification-corpus-baseline-0001",
            "mono:1700000800",
            ClaimedStableProfile::StableDefault,
            baseline_resource_audits("mono:1700000800"),
            baseline_audit_surfaces("mono:1700000800"),
        ),
        default_state_root_inspection_hooks(),
    );

    // Extended Stable: adds a dirty-with-disclosure prebuild cache audit
    // carrying a cleanup-surface ref + inspection-hook ref + audit
    // disclosure + redaction disclosure, plus an optional
    // mutation-journal audit row. Profile switches to support-export
    // lane to prove the Stable claim is honored.
    let mut extended_audits = baseline_resource_audits("mono:1700000810");
    extended_audits.push(make_audit(
        "prebuild-cache.0",
        StateRootResourceKind::PrebuildCacheRoot,
        "cache:prebuild_artifact_cache",
        AuditFindingClass::AuditDirtyWithDisclosure,
        Some("disclosure.prebuild-cache.0"),
        AuditRedactionClass::RedactedWithDisclosure,
        Some("disclosure.redaction.prebuild-cache.0"),
        true,
        "WS-AUD-0010",
        "mono:1700000810",
    ));
    if let Some(row) = extended_audits.last_mut() {
        row.cleanup_surface_refs = vec![
            "cleanup:settings_panel".to_owned(),
            "cleanup:support_cleanup_tool".to_owned(),
        ];
        row.inspection_hook_refs = vec![
            "state_root_certification.compare_before_cleanup".to_owned(),
            "state_root_certification.export_before_cleanup".to_owned(),
        ];
    }
    extended_audits.push(make_audit(
        "mutation-journal.0",
        StateRootResourceKind::MutationJournalRoot,
        "cache:durable_workspace_state",
        AuditFindingClass::AuditClean,
        None,
        AuditRedactionClass::MetadataOnly,
        None,
        false,
        "WS-AUD-0011",
        "mono:1700000810",
    ));
    write_fixture(
        "extended_with_dirty_disclosed_stable",
        "posture:extended_with_dirty_disclosed",
        base_inputs(
            "workspace-rust-service-0001",
            "state-root-certification-corpus-extended-0001",
            "mono:1700000810",
            ClaimedStableProfile::StableSupportExport,
            extended_audits,
            baseline_audit_surfaces("mono:1700000810"),
        ),
        default_state_root_inspection_hooks(),
    );

    // Narrowed: a dirty audit ships without its audit_disclosure_ref —
    // must narrow with `audit_disclosure_missing`.
    let mut silent_audits = baseline_resource_audits("mono:1700000820");
    if let Some(a) = silent_audits.first_mut() {
        a.audit_finding = AuditFindingClass::AuditDirtyWithDisclosure;
        a.audit_disclosure_ref = None;
        a.cleanup_surface_refs = vec!["cleanup:settings_panel".to_owned()];
        a.inspection_hook_refs = vec!["state_root_certification.compare_before_cleanup".to_owned()];
    }
    write_fixture(
        "dirty_audit_silent_narrowed",
        "posture:dirty_audit_silent",
        base_inputs(
            "workspace-rust-service-0001",
            "state-root-certification-corpus-silent-0001",
            "mono:1700000820",
            ClaimedStableProfile::StableDefault,
            silent_audits,
            baseline_audit_surfaces("mono:1700000820"),
        ),
        default_state_root_inspection_hooks(),
    );

    // Narrowed: an audit row declares `silent_rerun_permitted`
    // (forbidden on Stable rows).
    let mut silent_rerun_audits = baseline_resource_audits("mono:1700000830");
    if let Some(a) = silent_rerun_audits
        .iter_mut()
        .find(|a| a.resource_class == StateRootResourceKind::CacheGovernanceRoot)
    {
        a.rerun_posture = AuditRerunPosture::SilentRerunPermitted;
    }
    write_fixture(
        "audit_silent_rerun_narrowed",
        "posture:audit_silent_rerun",
        base_inputs(
            "workspace-rust-service-0001",
            "state-root-certification-corpus-silent-rerun-0001",
            "mono:1700000830",
            ClaimedStableProfile::StableDefault,
            silent_rerun_audits,
            baseline_audit_surfaces("mono:1700000830"),
        ),
        default_state_root_inspection_hooks(),
    );

    // Narrowed: required `compare_before_cleanup` inspection hook is
    // unavailable on this posture.
    let narrowed_inputs = base_inputs(
        "workspace-rust-service-0001",
        "state-root-certification-corpus-narrowed-hook-0001",
        "mono:1700000840",
        ClaimedStableProfile::StableDefault,
        baseline_resource_audits("mono:1700000840"),
        baseline_audit_surfaces("mono:1700000840"),
    );
    let mut narrowed_hooks = default_state_root_inspection_hooks();
    for hook in &mut narrowed_hooks {
        if hook.hook_class == StateRootInspectionHookClass::CompareBeforeCleanup {
            hook.available = false;
            hook.disclosure = "Compare-before-cleanup unavailable on this posture.".to_owned();
        }
    }
    write_fixture(
        "missing_compare_before_cleanup_hook_narrowed",
        "posture:missing_compare_before_cleanup_hook",
        narrowed_inputs,
        narrowed_hooks,
    );
}
