//! Unit tests for the store-lock / external-root recovery builder and
//! validator.

use super::*;

fn clean_descriptor(state_id: &str, incident: IncidentClass) -> RecoveryState {
    let (resource_class, degraded_state_class) = match incident {
        IncidentClass::CredentialStoreLocked => (
            ResourceClass::CredentialStore,
            DegradedStateClass::StoreLocked,
        ),
        IncidentClass::CredentialStoreUnavailable => (
            ResourceClass::CredentialStore,
            DegradedStateClass::StoreUnavailable,
        ),
        IncidentClass::TrustStoreDrift => (
            ResourceClass::TrustStore,
            DegradedStateClass::TrustStoreDrifted,
        ),
        IncidentClass::RemovableVolumeMissing => (
            ResourceClass::RemovableVolume,
            DegradedStateClass::RootMissing,
        ),
        IncidentClass::NetworkShareMissing => {
            (ResourceClass::NetworkShare, DegradedStateClass::RootMissing)
        }
        IncidentClass::ExternalRootMissing => {
            (ResourceClass::ExternalRoot, DegradedStateClass::RootMissing)
        }
        IncidentClass::RootReturned => (
            ResourceClass::NetworkShare,
            DegradedStateClass::RootReturned,
        ),
    };
    let paused = if degraded_state_class.is_active_degradation() {
        vec![PausedCapability {
            capability_class: PausedCapabilityClass::ExternalRootAccess,
            capability_ref: format!("{state_id}:paused"),
        }]
    } else {
        vec![]
    };
    RecoveryState {
        state_id: state_id.to_owned(),
        incident_class: incident,
        resource_class,
        degraded_state_class,
        descriptor_revision_ref: format!("{state_id}:rev"),
        primary_label_ref: format!("{state_id}:label"),
        last_seen_identity_ref: format!("{state_id}:identity"),
        placeholder_ref: format!("{state_id}:placeholder"),
        paused_capabilities: paused,
        local_only_capabilities: vec![LocalOnlyCapability {
            capability_class: LocalOnlyCapabilityClass::LocalEditing,
            capability_ref: format!("{state_id}:local_only"),
        }],
        unsaved_local_state_posture: UnsavedLocalStatePosture::PreservedInPlace,
        local_continuity_preserved: true,
        recovery_actions: vec![RecoveryActionClass::LocateRoot],
        repair_guidance_ref: format!("{state_id}:repair"),
        implies_plaintext_fallback: false,
        resume_posture: ResumePostureClass::ExplicitResumeRequired,
        resumes_silently_on_recovery: false,
        protected_continuations: vec![],
        active_profile_owner_ref: format!("{state_id}:profile"),
        trust_checkpoint_ref: format!("{state_id}:trust"),
        canonical_command_ref: "cmd:workspace.root.recover".to_owned(),
        continuity_note: "local work preserved".to_owned(),
        degraded_state_vocabulary: vec!["The resource is unavailable".to_owned()],
        surface_parity: SurfaceClass::required().to_vec(),
        claimed_platforms: Platform::all().to_vec(),
        evidence_freshness: EvidenceFreshness::Fresh,
        evidence_captured_at: GENERATED_AT.to_owned(),
        downgrade_rule_ref: "downgrade:rule".to_owned(),
        marketed: true,
        registered_on_recovery_harness: true,
    }
}

#[test]
fn seeded_report_is_clean_and_validates() {
    let report = seeded_store_lock_recovery_report();
    assert!(report.report_clean, "seeded report must be clean");
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    validate_store_lock_recovery_report(&report).expect("seeded report must validate");
}

#[test]
fn seeded_report_covers_every_required_incident_kind() {
    let report = seeded_store_lock_recovery_report();
    assert!(report.every_kind_present());
    for kind in IncidentClass::required_kinds() {
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.descriptor.incident_class == kind),
            "no registered state for required incident kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn seeded_report_distinguishes_the_four_case_families() {
    let exports = seeded_store_lock_recovery_case_exports();
    assert_eq!(exports.len(), 4, "the four incident families must exist");
    let labels: Vec<&str> = exports
        .iter()
        .map(|export| export.case_label.as_str())
        .collect();
    assert!(labels.contains(&"credential_store_locked"));
    assert!(labels.contains(&"trust_store_drift"));
    assert!(labels.contains(&"missing_root"));
    assert!(labels.contains(&"root_returned"));
}

#[test]
fn plaintext_fallback_is_a_blocker() {
    let mut descriptor = clean_descriptor("state:plaintext", IncidentClass::CredentialStoreLocked);
    descriptor.implies_plaintext_fallback = true;
    let row = build_store_lock_recovery_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, RecoveryBlockingFinding::PlaintextFallbackImplied { .. })));
}

#[test]
fn local_work_must_be_preserved() {
    let mut descriptor = clean_descriptor("state:lostwork", IncidentClass::ExternalRootMissing);
    descriptor.local_continuity_preserved = false;
    let row = build_store_lock_recovery_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, RecoveryBlockingFinding::LocalWorkNotPreserved { .. })));
}

#[test]
fn missing_placeholder_is_silent_disappearance() {
    let mut descriptor =
        clean_descriptor("state:noplaceholder", IncidentClass::NetworkShareMissing);
    descriptor.placeholder_ref = String::new();
    let row = build_store_lock_recovery_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, RecoveryBlockingFinding::SilentDisappearance { .. })));
}

#[test]
fn store_trust_and_root_unrecoverable_findings_stay_distinct() {
    let mut store = clean_descriptor("state:store", IncidentClass::CredentialStoreLocked);
    store.recovery_actions = vec![];
    let store_row = build_store_lock_recovery_row(store);
    assert!(store_row.blocking_findings.iter().any(|f| matches!(
        f,
        RecoveryBlockingFinding::CredentialStoreLockUnrecoverable { .. }
    )));

    let mut trust = clean_descriptor("state:trust", IncidentClass::TrustStoreDrift);
    trust.recovery_actions = vec![];
    let trust_row = build_store_lock_recovery_row(trust);
    assert!(trust_row.blocking_findings.iter().any(|f| matches!(
        f,
        RecoveryBlockingFinding::TrustStoreDriftUnrecoverable { .. }
    )));

    let mut root = clean_descriptor("state:root", IncidentClass::ExternalRootMissing);
    root.recovery_actions = vec![];
    let root_row = build_store_lock_recovery_row(root);
    assert!(root_row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, RecoveryBlockingFinding::MissingRootUnrecoverable { .. })));

    // The three classes never collapse: a store-lock row carries neither the
    // trust nor the root unrecoverable finding.
    assert!(!store_row.blocking_findings.iter().any(|f| matches!(
        f,
        RecoveryBlockingFinding::TrustStoreDriftUnrecoverable { .. }
            | RecoveryBlockingFinding::MissingRootUnrecoverable { .. }
    )));
}

#[test]
fn silent_resume_disposition_is_a_blocker() {
    let mut descriptor = clean_descriptor("state:silent", IncidentClass::RootReturned);
    descriptor.protected_continuations = vec![ProtectedContinuation {
        continuation_ref: "state:silent:cont".to_owned(),
        continuation_class: ContinuationClass::QueuedJob,
        resume_disposition: ResumeDispositionClass::SilentResume,
    }];
    let row = build_store_lock_recovery_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, RecoveryBlockingFinding::SilentResumeOnRecovery { .. })));
}

#[test]
fn returned_root_must_require_explicit_resume() {
    let mut descriptor = clean_descriptor("state:returned", IncidentClass::RootReturned);
    descriptor.resume_posture = ResumePostureClass::NotApplicable;
    let row = build_store_lock_recovery_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, RecoveryBlockingFinding::SilentResumeOnRecovery { .. })));
}

#[test]
fn silently_resuming_on_recovery_is_a_blocker() {
    let mut descriptor =
        clean_descriptor("state:autoresume", IncidentClass::RemovableVolumeMissing);
    descriptor.resumes_silently_on_recovery = true;
    let row = build_store_lock_recovery_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, RecoveryBlockingFinding::SilentResumeOnRecovery { .. })));
}

#[test]
fn active_degradation_without_paused_disclosure_is_a_blocker() {
    let mut descriptor =
        clean_descriptor("state:nopaused", IncidentClass::CredentialStoreUnavailable);
    descriptor.paused_capabilities = vec![];
    let row = build_store_lock_recovery_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, RecoveryBlockingFinding::MissingPausedDisclosure { .. })));
}

#[test]
fn missing_trust_checkpoint_bypasses_trust_evaluation() {
    let mut descriptor = clean_descriptor("state:notrust", IncidentClass::CredentialStoreLocked);
    descriptor.trust_checkpoint_ref = String::new();
    let row = build_store_lock_recovery_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, RecoveryBlockingFinding::TrustEvaluationBypassed { .. })));
}

#[test]
fn incomplete_surface_parity_is_a_blocker() {
    let mut descriptor = clean_descriptor("state:nosurface", IncidentClass::TrustStoreDrift);
    descriptor.surface_parity = vec![SurfaceClass::Desktop];
    let row = build_store_lock_recovery_row(descriptor);
    assert!(row
        .blocking_findings
        .iter()
        .any(|f| matches!(f, RecoveryBlockingFinding::SurfaceParityIncomplete { .. })));
}

#[test]
fn stale_evidence_on_marketed_state_is_a_blocker() {
    let mut descriptor = clean_descriptor("state:stale", IncidentClass::NetworkShareMissing);
    descriptor.evidence_freshness = EvidenceFreshness::Stale;
    let row = build_store_lock_recovery_row(descriptor);
    assert!(row.blocking_findings.iter().any(|f| matches!(
        f,
        RecoveryBlockingFinding::StaleEvidenceOnMarketedState { .. }
    )));
}

#[test]
fn returned_root_with_explicit_resume_is_clean() {
    let descriptor = clean_descriptor("state:returned_ok", IncidentClass::RootReturned);
    let row = build_store_lock_recovery_row(descriptor);
    assert!(
        row.blocking_findings.is_empty(),
        "a returned root with explicit resume must be clean: {:?}",
        row.blocking_findings
    );
}

#[test]
fn support_export_quotes_report_and_state_ids() {
    let report = seeded_store_lock_recovery_report();
    let export = StoreLockRecoverySupportExport::from_report(
        STORE_LOCK_RECOVERY_SUPPORT_EXPORT_ID,
        report.clone(),
    );
    assert!(export.case_ids.contains(&report.report_id));
    for entry in &report.entries {
        assert!(export.case_ids.contains(&entry.descriptor.state_id));
        assert!(export
            .case_ids
            .contains(&entry.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn validator_flags_a_blocking_finding() {
    let mut descriptor = clean_descriptor("state:bad", IncidentClass::CredentialStoreLocked);
    descriptor.implies_plaintext_fallback = true;
    let row = build_store_lock_recovery_row(descriptor);
    let report = build_store_lock_recovery_report(vec![row]);
    let errors = validate_store_lock_recovery_report(&report).expect_err("must fail");
    assert!(errors.iter().any(|e| matches!(
        e,
        StoreLockRecoveryValidationError::BlockingFindingPresent { .. }
    )));
}
