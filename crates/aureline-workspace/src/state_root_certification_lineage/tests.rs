//! Unit tests for the state-root certification lineage projection.

use super::*;

fn make_support() -> StateRootSupportExportInputs {
    StateRootSupportExportInputs::metadata_safe_baseline(
        StateRootSupportExportPosture::MetadataSafeExport,
    )
}

#[allow(clippy::too_many_arguments)]
fn audit(
    resource_id: &str,
    class: StateRootResourceKind,
    storage_class_ref: &str,
    finding: AuditFindingClass,
    audit_disclosure: Option<&str>,
    mutates_state: bool,
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
        audit_transaction_id: format!("tx.{resource_id}.aud.0001"),
        finding_code: "WS-AUD-0001".to_owned(),
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
            format!("action.{resource_id}.audit.commit")
        } else {
            String::new()
        },
        commit_disclosure_id: if mutates_state {
            format!("disclosure.{resource_id}.audit.commit")
        } else {
            String::new()
        },
        cleanup_surface_refs: cleanup_refs,
        inspection_hook_refs: hook_refs,
        redaction_class: AuditRedactionClass::MetadataOnly,
        redaction_disclosure_ref: None,
        support_export: make_support(),
        captured_at: "mono:1700000800".to_owned(),
    }
}

fn surface(kind: AuditSurfaceKind) -> AuditSurfaceObservation {
    AuditSurfaceObservation {
        audit_surface_id: format!("surf.{}", kind.as_str()),
        label: kind.as_str().to_owned(),
        audit_surface_kind: kind,
        reachable: true,
        preserves_lineage_refs: true,
        preserves_trust_state: true,
        discloses_non_clean_findings: true,
        captured_at: "mono:1700000800".to_owned(),
    }
}

fn baseline_inputs() -> StateRootCertificationInputs {
    let resource_audits = vec![
        audit(
            "envelope-0",
            StateRootResourceKind::PersistentStateEnvelope,
            "cache:durable_workspace_state",
            AuditFindingClass::AuditClean,
            None,
            false,
        ),
        audit(
            "workspace-0",
            StateRootResourceKind::WorkspaceStateRoot,
            "cache:durable_workspace_state",
            AuditFindingClass::AuditClean,
            None,
            false,
        ),
        audit(
            "profile-0",
            StateRootResourceKind::ProfileRoot,
            "cache:durable_workspace_state",
            AuditFindingClass::AuditClean,
            None,
            false,
        ),
        audit(
            "recent-work-0",
            StateRootResourceKind::RecentWorkRoot,
            "cache:durable_workspace_state",
            AuditFindingClass::AuditClean,
            None,
            false,
        ),
        audit(
            "local-history-0",
            StateRootResourceKind::LocalHistoryRoot,
            "cache:local_history",
            AuditFindingClass::AuditClean,
            None,
            false,
        ),
        audit(
            "restore-checkpoint-0",
            StateRootResourceKind::RestoreCheckpointRoot,
            "cache:recovery_checkpoint",
            AuditFindingClass::AuditClean,
            None,
            false,
        ),
        audit(
            "cache-governance-0",
            StateRootResourceKind::CacheGovernanceRoot,
            "cache:local_disk_cache",
            AuditFindingClass::AuditClean,
            None,
            false,
        ),
    ];
    let audit_surfaces = vec![
        surface(AuditSurfaceKind::StorageDisciplineOverview),
        surface(AuditSurfaceKind::CacheGovernanceInspector),
        surface(AuditSurfaceKind::StateRootAuditPanel),
        surface(AuditSurfaceKind::CleanupInventoryAudit),
        surface(AuditSurfaceKind::EvictionRuleAudit),
        surface(AuditSurfaceKind::HeadlessAuditCli),
        surface(AuditSurfaceKind::SupportExportAuditSection),
    ];
    StateRootCertificationInputs {
        workspace_ref: "workspace-state-root-certification-0001".to_owned(),
        producer_ref: "producer-aureline-0001".to_owned(),
        corpus_ref: "state-root-certification-corpus-0001".to_owned(),
        captured_at: "mono:1700000800".to_owned(),
        claimed_profile: ClaimedStableProfile::StableDefault,
        resource_audits,
        audit_surfaces,
    }
}

#[test]
fn clean_inputs_project_stable_record() {
    let inputs = baseline_inputs();
    let record = project_state_root_certification_lineage("posture.clean", &inputs);
    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(record.is_support_export_safe());
    assert_eq!(
        record.record_kind,
        STATE_ROOT_CERTIFICATION_LINEAGE_RECORD_KIND
    );
    assert_eq!(
        record.schema_ref,
        STATE_ROOT_CERTIFICATION_LINEAGE_SCHEMA_REF
    );
    assert_eq!(record.claimed_profile, ClaimedStableProfile::StableDefault);
    assert!(
        record
            .resource_class_coverage
            .all_required_resource_classes_present
    );
    assert!(
        record
            .audit_surface_coverage
            .all_required_audit_surfaces_present
    );
    assert!(record.audit_honesty.all_rows_pin_storage_class_ref);
    assert!(record.audit_honesty.all_audit_disclosures_present);
    assert!(record.audit_honesty.all_redaction_disclosures_present);
    assert!(
        record
            .audit_honesty
            .all_dirty_rows_have_cleanup_precondition
    );
    assert!(record.preservation.all_rows_preserve_restore_provenance);
    assert!(record.preservation.all_rows_preserve_encoding_fidelity);
    assert!(record.preservation.all_rows_preserve_trust_state);
    assert!(record.preservation.all_rows_preserve_lineage_refs);
    assert!(record.no_silent_rerun.all_rows_safe_rerun_posture);
    assert!(
        record
            .no_silent_rerun
            .all_mutating_rows_have_commit_metadata
    );
    assert!(
        record
            .audit_transaction_pinning
            .all_rows_pin_audit_transaction_id
    );
    assert!(record.audit_transaction_pinning.all_rows_pin_finding_code);
    assert!(record.support_export_honesty.all_rows_preserve_fields);
    assert!(record.support_export_honesty.all_rows_exclude_raw_secrets);
    assert!(
        record
            .support_export_honesty
            .all_rows_exclude_raw_artifact_bytes
    );
    assert_eq!(record.inspection_hooks.len(), 8);
}

#[test]
fn missing_required_resource_class_narrows() {
    let mut inputs = baseline_inputs();
    inputs
        .resource_audits
        .retain(|a| a.resource_class != StateRootResourceKind::CacheGovernanceRoot);
    let record = project_state_root_certification_lineage("posture.missing_resource", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::RequiredResourceClassMissing));
}

#[test]
fn missing_required_audit_surface_narrows() {
    let mut inputs = baseline_inputs();
    inputs
        .audit_surfaces
        .retain(|s| s.audit_surface_kind != AuditSurfaceKind::EvictionRuleAudit);
    let record = project_state_root_certification_lineage("posture.missing_surface", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::RequiredAuditSurfaceMissing));
}

#[test]
fn missing_storage_class_ref_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(a) = inputs.resource_audits.first_mut() {
        a.storage_class_ref.clear();
    }
    let record = project_state_root_certification_lineage("posture.no_storage_ref", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::StorageClassRefMissing));
}

#[test]
fn silent_rerun_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(a) = inputs.resource_audits.first_mut() {
        a.rerun_posture = AuditRerunPosture::SilentRerunPermitted;
    }
    let record = project_state_root_certification_lineage("posture.silent_rerun", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::RerunSilentForbidden));
}

#[test]
fn missing_commit_metadata_narrows() {
    let mut inputs = baseline_inputs();
    inputs.resource_audits.push(audit(
        "mutating-0",
        StateRootResourceKind::CacheGovernanceRoot,
        "cache:local_disk_cache",
        AuditFindingClass::AuditDirtyWithDisclosure,
        Some("disclosure.mutating.0"),
        true,
    ));
    if let Some(a) = inputs.resource_audits.last_mut() {
        a.commit_action_id.clear();
    }
    let record = project_state_root_certification_lineage("posture.no_commit", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::CommitActionMetadataMissing));
}

#[test]
fn dirty_audit_without_disclosure_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(a) = inputs.resource_audits.first_mut() {
        a.audit_finding = AuditFindingClass::AuditDirtyWithDisclosure;
        a.audit_disclosure_ref = None;
        a.cleanup_surface_refs = vec!["cleanup:settings_panel".to_owned()];
        a.inspection_hook_refs = vec!["state_root_certification.compare_before_cleanup".to_owned()];
    }
    let record = project_state_root_certification_lineage("posture.dirty_no_disclosure", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::AuditDisclosureMissing));
}

#[test]
fn dirty_audit_without_cleanup_precondition_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(a) = inputs.resource_audits.first_mut() {
        a.audit_finding = AuditFindingClass::AuditDirtyWithDisclosure;
        a.audit_disclosure_ref = Some("disclosure.dirty.0".to_owned());
        a.cleanup_surface_refs.clear();
        a.inspection_hook_refs.clear();
    }
    let record = project_state_root_certification_lineage("posture.dirty_no_cleanup", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::CleanupPreconditionMissing));
}

#[test]
fn redacted_without_disclosure_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(a) = inputs.resource_audits.first_mut() {
        a.redaction_class = AuditRedactionClass::RedactedWithDisclosure;
        a.redaction_disclosure_ref = None;
    }
    let record = project_state_root_certification_lineage("posture.red_no_disclosure", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::RedactionDisclosureMissing));
}

#[test]
fn missing_audit_transaction_id_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(a) = inputs.resource_audits.first_mut() {
        a.audit_transaction_id.clear();
    }
    let record = project_state_root_certification_lineage("posture.no_tx", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::AuditTransactionIdNotPinned));
}

#[test]
fn missing_finding_code_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(a) = inputs.resource_audits.first_mut() {
        a.finding_code.clear();
    }
    let record = project_state_root_certification_lineage("posture.no_finding", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::FindingCodeMissing));
}

#[test]
fn preservation_loss_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(a) = inputs.resource_audits.first_mut() {
        a.preserves_restore_provenance = false;
    }
    let record = project_state_root_certification_lineage("posture.provenance_lost", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::RestoreProvenanceNotPreserved));
}

#[test]
fn trust_state_loss_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(a) = inputs.resource_audits.first_mut() {
        a.preserves_trust_state = false;
    }
    let record = project_state_root_certification_lineage("posture.trust_lost", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::TrustStateNotPreserved));
}

#[test]
fn lineage_refs_loss_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(a) = inputs.resource_audits.first_mut() {
        a.preserves_lineage_refs = false;
    }
    let record = project_state_root_certification_lineage("posture.lineage_lost", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::LineageRefsNotPreserved));
}

#[test]
fn audit_surface_unreachable_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(s) = inputs.audit_surfaces.first_mut() {
        s.reachable = false;
    }
    let record = project_state_root_certification_lineage("posture.surf_unreachable", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::AuditSurfaceUnreachable));
}

#[test]
fn audit_surface_disclosure_gap_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(s) = inputs.audit_surfaces.first_mut() {
        s.discloses_non_clean_findings = false;
    }
    let record = project_state_root_certification_lineage("posture.surf_gap", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::AuditSurfaceDisclosureGap));
}

#[test]
fn unavailable_hook_narrows() {
    let inputs = baseline_inputs();
    let mut hooks = default_state_root_inspection_hooks();
    for hook in &mut hooks {
        if hook.hook_class == StateRootInspectionHookClass::CompareBeforeCleanup {
            hook.available = false;
        }
    }
    let record = project_state_root_certification_lineage_with_hooks(
        "posture.no_compare_hook",
        &inputs,
        hooks,
    );
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::InspectionHookUnavailable));
}

#[test]
fn support_export_redaction_unsafe_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(a) = inputs.resource_audits.first_mut() {
        a.support_export.raw_secrets_excluded = false;
    }
    let record = project_state_root_certification_lineage("posture.unsafe_export", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::SupportExportRedactionUnsafe));
}

#[test]
fn support_export_field_dropped_narrows() {
    let mut inputs = baseline_inputs();
    if let Some(a) = inputs.resource_audits.first_mut() {
        a.support_export.includes_finding_code = false;
    }
    let record = project_state_root_certification_lineage("posture.dropped_field", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::SupportExportFieldsDropped));
}

#[test]
fn claimed_profile_not_stable_narrows() {
    let mut inputs = baseline_inputs();
    inputs.claimed_profile = ClaimedStableProfile::NarrowedBelowStable;
    let record = project_state_root_certification_lineage("posture.profile_narrowed", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::ClaimedProfileNotStable));
}

#[test]
fn producer_attribution_incomplete_narrows() {
    let mut inputs = baseline_inputs();
    inputs.producer_ref.clear();
    let record = project_state_root_certification_lineage("posture.no_producer", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::ProducerAttributionIncomplete));
}

#[test]
fn lineage_export_unsafe_narrows() {
    let mut inputs = baseline_inputs();
    inputs.workspace_ref.clear();
    let record = project_state_root_certification_lineage("posture.no_workspace", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::LineageExportUnsafe));
}

#[test]
fn empty_corpus_narrows() {
    let mut inputs = baseline_inputs();
    inputs.resource_audits.clear();
    let record = project_state_root_certification_lineage("posture.empty", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&StateRootCertificationLineageNarrowReason::CorpusEmpty));
}

#[test]
fn lines_render_each_section() {
    let inputs = baseline_inputs();
    let record = project_state_root_certification_lineage("posture.lines", &inputs);
    let lines = state_root_certification_lineage_lines(&record);
    assert!(lines
        .iter()
        .any(|l| l.contains("State-root certification lineage")));
    assert!(lines.iter().any(|l| l == "Resource audits:"));
    assert!(lines.iter().any(|l| l == "Audit surfaces:"));
    assert!(lines.iter().any(|l| l.contains("Audit honesty")));
    assert!(lines.iter().any(|l| l.contains("Preservation")));
    assert!(lines.iter().any(|l| l.contains("No-silent-rerun")));
    assert!(lines
        .iter()
        .any(|l| l.contains("Audit-transaction pinning")));
    assert!(lines
        .iter()
        .any(|l| l.contains("Audit-surface reachability")));
    assert!(lines.iter().any(|l| l.contains("Support-export honesty")));
    assert!(lines.iter().any(|l| l == "Inspection hooks:"));
}
