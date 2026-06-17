use super::*;

fn packet() -> PackageOperationHistory {
    current_package_operation_history().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, OPERATION_HISTORY_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, OPERATION_HISTORY_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn path_is_stable() {
    assert_eq!(
        OPERATION_HISTORY_PATH,
        "artifacts/deps/m5/operation-history.json"
    );
}

#[test]
fn corpus_covers_every_required_state() {
    let packet = packet();
    assert_eq!(packet.corpus_coverage_gaps(), Vec::new());
}

#[test]
fn every_operation_kind_is_represented() {
    let packet = packet();
    let kinds: BTreeSet<OperationKind> = packet.entries.iter().map(|e| e.operation_kind).collect();
    for kind in OperationKind::ALL {
        assert!(
            kinds.contains(&kind),
            "no entry exercises {}",
            kind.as_str()
        );
    }
}

#[test]
fn retention_subject_is_operation_history() {
    let packet = packet();
    assert_eq!(packet.retention_subject, RetentionSubject::OperationHistory);
    assert_eq!(
        packet.retention_subject.canonical_retention_class(),
        RetentionClass::BoundedLocalHistory
    );
}

#[test]
fn every_entry_binds_to_the_frozen_matrix() {
    let packet = packet();
    let matrix = current_m5_package_state_matrix().expect("matrix loads");
    assert_eq!(packet.references_matrix_id, matrix.packet_id);
    assert!(packet.all_bind_matrix());
    for entry in &packet.entries {
        for label in &entry.applicable_labels {
            assert!(
                matrix.state(*label).is_some(),
                "operation {} surfaces unbound label {}",
                entry.operation_id,
                label.as_str()
            );
        }
    }
}

#[test]
fn every_entry_discloses_what_support_needs() {
    let packet = packet();
    for entry in &packet.entries {
        assert!(
            entry.discloses_all_required(),
            "operation {} omits a required disclosure",
            entry.operation_id
        );
    }
}

#[test]
fn direct_versus_transitive_chain_is_recorded_for_changes() {
    let packet = packet();
    let install = packet.entry("oph:install:cargo:serde").expect("install");
    assert_eq!(install.direct_link_count(), 1);
    assert_eq!(install.transitive_link_count(), 1);
    // The transitive serde_derive names serde as its parent — a chain, not a list.
    let derive = install
        .impact_chain
        .iter()
        .find(|l| l.package_name == "serde_derive")
        .expect("transitive link");
    assert!(derive.is_transitive());
    assert_eq!(derive.parent_link_ids, vec!["lnk:serde".to_owned()]);
    assert!(install.has_visible_changed_chain());
}

#[test]
fn impact_chain_stays_visible_on_every_surface_including_support() {
    let packet = packet();
    let id = "oph:install:cargo:serde";
    for surface in PackageSurface::ALL {
        let projection = packet
            .surface_projection(id, surface)
            .expect("surface projection");
        assert!(
            projection.impact_chain_visible,
            "chain hidden on {}",
            surface.as_str()
        );
        assert_eq!(projection.direct_links, 1);
        assert_eq!(projection.transitive_links, 1);
    }
    assert!(packet.all_chains_visible());
}

#[test]
fn result_classes_stay_distinct() {
    let packet = packet();
    assert_eq!(
        packet
            .entry("oph:install:cargo:serde")
            .unwrap()
            .result_class,
        OperationResultClass::Applied
    );
    assert_eq!(
        packet
            .entry("oph:regenerate:cargo:noop")
            .unwrap()
            .result_class,
        OperationResultClass::NoChangeNeeded
    );
    assert_eq!(
        packet
            .entry("oph:regenerate:cargo:workspace")
            .unwrap()
            .result_class,
        OperationResultClass::PartiallyApplied
    );
    assert_eq!(
        packet
            .entry("oph:update:node:react-rollback")
            .unwrap()
            .result_class,
        OperationResultClass::RolledBack
    );
    assert_eq!(
        packet
            .entry("oph:install:cargo:failed")
            .unwrap()
            .result_class,
        OperationResultClass::FailedNoChange
    );
    assert_eq!(
        packet
            .entry("oph:regenerate:cargo:policy-blocked")
            .unwrap()
            .result_class,
        OperationResultClass::BlockedByPolicy
    );
    assert_eq!(
        packet
            .entry("oph:install:pip:private-auth")
            .unwrap()
            .result_class,
        OperationResultClass::BlockedByAuth
    );
}

#[test]
fn rolled_back_entry_keeps_chain_but_returns_identity() {
    let packet = packet();
    let entry = packet.entry("oph:update:node:react-rollback").unwrap();
    // The attempted impact is still recorded for the operator.
    assert!(entry.has_visible_changed_chain());
    // The net identity returned to its starting state after the revert.
    assert!(!entry.identity.identity_changed());
    assert!(entry.rollback.is_durable_recovery());
    assert!(!entry.rollback.revert_available);
}

#[test]
fn blocked_and_failed_entries_write_nothing() {
    let packet = packet();
    for id in [
        "oph:install:pip:private-auth",
        "oph:regenerate:cargo:policy-blocked",
        "oph:install:cargo:failed",
    ] {
        let entry = packet.entry(id).unwrap();
        assert!(entry.result_class.requires_no_write());
        assert!(!entry.identity.identity_changed(), "{id} moved identity");
        assert!(entry.impact_chain.is_empty(), "{id} claims a change");
        assert_eq!(entry.rollback.rollback_class, RollbackClass::NotApplicable);
        assert!(!entry.rollback.is_recoverable());
    }
}

#[test]
fn auth_block_is_distinct_from_a_failed_resolution() {
    let packet = packet();
    let auth = packet.entry("oph:install:pip:private-auth").unwrap();
    assert!(auth.registry_source.trust_blocked());
    assert_eq!(
        auth.applicable_labels,
        vec![PackageStateLabel::AuthRequired]
    );

    let failed = packet.entry("oph:install:cargo:failed").unwrap();
    assert_eq!(
        failed.applicable_labels,
        vec![PackageStateLabel::UnknownOrStale]
    );
    // The two outcomes never collapse into one generic message.
    assert_ne!(auth.result_class, failed.result_class);
}

#[test]
fn every_recoverable_entry_offers_full_recovery() {
    let packet = packet();
    for entry in &packet.entries {
        if entry.rollback.is_recoverable() {
            assert!(
                entry.rollback.offers_all_actions(),
                "operation {} lacks a recovery action",
                entry.operation_id
            );
            assert!(!entry.rollback.checkpoint_ref.trim().is_empty());
        }
    }
}

#[test]
fn history_is_redaction_default() {
    let packet = packet();
    assert!(packet.all_redaction_safe());
    for entry in &packet.entries {
        assert!(!entry.retention.full_manifest_body_retained);
        assert!(!entry.retention.raw_credentials_retained);
        assert!(entry.retention.is_consistent());
    }
}

#[test]
fn export_projection_is_redaction_safe() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.entries.len());
    assert!(projection.all_bind_matrix);
    assert!(projection.all_chains_visible);
    assert!(projection.all_redaction_safe);
    for row in &projection.rows {
        assert!(!row.summary.contains("://"));
        assert!(!row.label.contains("://"));
    }
}

#[test]
fn no_field_leaks_a_raw_url() {
    let packet = packet();
    for entry in &packet.entries {
        assert!(!entry.scope.redacted_manifest_path.contains("://"));
        assert!(!entry.requested.requested_ref.contains("://"));
        assert!(!entry.registry_source.redacted_source_label.contains("://"));
        assert!(!entry.identity.lockfile_identity_before.contains("://"));
        assert!(!entry.identity.lockfile_identity_after.contains("://"));
        assert!(!entry.identity.manifest_digest_before.contains("://"));
        assert!(!entry.identity.manifest_digest_after.contains("://"));
        assert!(!entry.validation.redacted_evidence_ref.contains("://"));
        assert!(!entry.rollback.checkpoint_ref.contains("://"));
        for link in &entry.impact_chain {
            assert!(!link.package_name.contains("://"));
        }
        for action in &entry.rollback.actions {
            assert!(!action.redacted_target_ref.contains("://"));
        }
        for evidence in &entry.evidence_refs {
            assert!(!evidence.redacted_ref.contains("://"));
        }
    }
}

#[test]
fn ai_and_recipe_origins_pass_through_the_same_receipts() {
    let packet = packet();
    let ai = packet.entry("oph:update:cargo:tokio-downgrade").unwrap();
    assert!(ai.origin.is_automated());
    assert_eq!(ai.origin, OperationOrigin::AiProposal);
    // The AI-applied operation carries the same disclosures and recovery path.
    assert!(ai.discloses_all_required());
    assert!(ai.rollback.is_durable_recovery());

    let recipe = packet.entry("oph:regenerate:cargo:workspace").unwrap();
    assert_eq!(recipe.origin, OperationOrigin::RecipeProposal);
    assert!(recipe.discloses_all_required());
}

#[test]
fn validate_flags_full_manifest_body_retained() {
    let mut packet = packet();
    packet.entries[0].retention.full_manifest_body_retained = true;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageOperationHistoryViolation::FullManifestBodyRetained { .. }
    )));
}

#[test]
fn validate_flags_raw_credential_retained() {
    let mut packet = packet();
    packet.entries[0].retention.raw_credentials_retained = true;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageOperationHistoryViolation::RawCredentialRetained { .. }
    )));
}

#[test]
fn validate_flags_rollback_inconsistent_with_result() {
    let mut packet = packet();
    // A no-change operation may not carry a recoverable rollback.
    let entry = packet
        .entries
        .iter_mut()
        .find(|e| e.operation_id == "oph:regenerate:cargo:noop")
        .unwrap();
    entry.rollback.rollback_class = RollbackClass::ReversibleCheckpointed;
    entry.rollback.revert_available = true;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageOperationHistoryViolation::RollbackInconsistent { .. }
    )));
}

#[test]
fn validate_flags_identity_mismatch_for_applied() {
    let mut packet = packet();
    let entry = packet
        .entries
        .iter_mut()
        .find(|e| e.operation_id == "oph:install:cargo:serde")
        .unwrap();
    // An applied operation that did not move its identity is contradictory.
    entry.identity.lockfile_identity_after = entry.identity.lockfile_identity_before.clone();
    entry.identity.manifest_digest_after = entry.identity.manifest_digest_before.clone();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageOperationHistoryViolation::IdentityResultMismatch { .. }
    )));
}

#[test]
fn validate_flags_validation_contradicting_result() {
    let mut packet = packet();
    let entry = packet
        .entries
        .iter_mut()
        .find(|e| e.operation_id == "oph:install:cargo:serde")
        .unwrap();
    // An applied operation cannot carry a failed validation.
    entry.validation.result = ValidationResult::Failed;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageOperationHistoryViolation::ValidationContradictsResult { .. }
    )));
}

#[test]
fn validate_flags_unexpected_impact_on_no_write() {
    let mut packet = packet();
    let entry = packet
        .entries
        .iter_mut()
        .find(|e| e.operation_id == "oph:install:cargo:failed")
        .unwrap();
    entry.impact_chain.push(ImpactChainLink {
        link_id: "lnk:ghost".to_owned(),
        package_name: "ghost-crate".to_owned(),
        relation: DependencyRelation::Direct,
        change_kind: ImpactChangeKind::Added,
        depth: 0,
        parent_link_ids: Vec::new(),
        version_before: None,
        version_after: Some("9.9.0".to_owned()),
        registry_source: RegistrySourceAuthority::PublicRegistry,
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageOperationHistoryViolation::UnexpectedImpactOnNoWrite { .. }
    )));
}

#[test]
fn validate_flags_dangling_parent_link() {
    let mut packet = packet();
    let entry = packet
        .entries
        .iter_mut()
        .find(|e| e.operation_id == "oph:install:cargo:serde")
        .unwrap();
    let derive = entry
        .impact_chain
        .iter_mut()
        .find(|l| l.package_name == "serde_derive")
        .unwrap();
    derive.parent_link_ids = vec!["lnk:does-not-exist".to_owned()];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageOperationHistoryViolation::DanglingParentLink { .. }
    )));
}

#[test]
fn validate_flags_inconsistent_impact_link() {
    let mut packet = packet();
    let entry = packet
        .entries
        .iter_mut()
        .find(|e| e.operation_id == "oph:install:cargo:serde")
        .unwrap();
    // A direct link cannot sit at a transitive depth.
    entry.impact_chain[0].depth = 3;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageOperationHistoryViolation::ImpactChainLinkInconsistent { .. }
    )));
}

#[test]
fn validate_flags_auth_block_without_blocked_source() {
    let mut packet = packet();
    let entry = packet
        .entries
        .iter_mut()
        .find(|e| e.operation_id == "oph:install:pip:private-auth")
        .unwrap();
    entry.registry_source.auth_mode = AuthMode::Anonymous;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageOperationHistoryViolation::AuthBlockSourceMismatch { .. }
    )));
}

#[test]
fn validate_flags_raw_url_leak() {
    let mut packet = packet();
    packet.entries[0].registry_source.redacted_source_label =
        "https://secret.example.com/registry".to_owned();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, PackageOperationHistoryViolation::RawUrlLeak { .. })));
}

#[test]
fn validate_flags_unbound_label() {
    let mut packet = packet();
    // SuppressedUntil is a valid frozen label but the matrix resolves it; use a
    // label and then point the matrix binding elsewhere to exercise unbinding via
    // matrix mismatch instead.
    packet.references_matrix_id = "some-other-matrix:v9".to_owned();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageOperationHistoryViolation::MatrixBindingMismatch { .. }
    )));
}

#[test]
fn validate_flags_retention_subject_mismatch() {
    let mut packet = packet();
    packet.retention_subject = RetentionSubject::RegistryCredentials;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageOperationHistoryViolation::RetentionSubjectMismatch { .. }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_operations = packet.summary.total_operations.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&PackageOperationHistoryViolation::SummaryMismatch));
}

#[test]
fn validate_flags_duplicate_operation_id() {
    let mut packet = packet();
    let clone = packet.entries[0].clone();
    packet.entries.push(clone);
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, PackageOperationHistoryViolation::DuplicateRowId { .. })));
}

#[test]
fn surface_write_authority_gates_revert() {
    let packet = packet();
    let id = "oph:install:cargo:serde";
    let desktop = packet
        .surface_projection(id, PackageSurface::DesktopPackageWorkspace)
        .unwrap();
    assert!(desktop.can_revert_here);
    assert!(!desktop.redacted);

    let support = packet
        .surface_projection(id, PackageSurface::SupportExport)
        .unwrap();
    assert!(!support.can_revert_here);
    assert!(support.redacted);

    let ai = packet
        .surface_projection(id, PackageSurface::AiContext)
        .unwrap();
    assert!(!ai.can_revert_here);

    // An already-reverted operation cannot be reverted again, even from desktop.
    let reverted = packet
        .surface_projection(
            "oph:update:node:react-rollback",
            PackageSurface::DesktopPackageWorkspace,
        )
        .unwrap();
    assert!(!reverted.can_revert_here);
}

#[test]
fn tokens_are_stable() {
    assert_eq!(OperationKind::Regenerate.as_str(), "regenerate");
    assert_eq!(
        OperationResultClass::PartiallyApplied.as_str(),
        "partially_applied"
    );
    assert_eq!(
        OperationResultClass::BlockedByAuth.as_str(),
        "blocked_by_auth"
    );
    assert_eq!(OperationOrigin::CliHeadless.as_str(), "cli_headless");
    assert_eq!(
        ValidationResult::PassedWithWarnings.as_str(),
        "passed_with_warnings"
    );
    assert_eq!(ImpactChangeKind::Repinned.as_str(), "repinned");
    assert_eq!(
        EvidenceKind::RollbackCheckpoint.as_str(),
        "rollback_checkpoint"
    );
    assert_eq!(RevertActionKind::ExportPatch.as_str(), "export_patch");
}

#[test]
fn every_vocabulary_round_trips_through_serde() {
    fn round_trip<T>(all: &[T])
    where
        T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        for value in all {
            let json = serde_json::to_string(value).expect("serialize");
            let back: T = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, value);
        }
    }
    round_trip(&OperationKind::ALL);
    round_trip(&OperationOrigin::ALL);
    round_trip(&OperationResultClass::ALL);
    round_trip(&ValidationResult::ALL);
    round_trip(&ImpactChangeKind::ALL);
    round_trip(&EvidenceKind::ALL);
    round_trip(&RevertActionKind::ALL);
}

/// Scenario fixtures, embedded so they validate without a runtime walk.
const FIXTURE_INSTALL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/operation-history/install_applied.json"
));
const FIXTURE_PARTIAL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/operation-history/regenerate_partial_recovery.json"
));
const FIXTURE_ROLLBACK: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/operation-history/update_rolled_back.json"
));
const FIXTURE_AUTH: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/operation-history/auth_blocked.json"
));

#[test]
fn fixtures_parse_and_validate() {
    for (name, json) in [
        ("install_applied", FIXTURE_INSTALL),
        ("regenerate_partial_recovery", FIXTURE_PARTIAL),
        ("update_rolled_back", FIXTURE_ROLLBACK),
        ("auth_blocked", FIXTURE_AUTH),
    ] {
        let packet: PackageOperationHistory =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("{name} parses: {e}"));
        assert_eq!(packet.validate(), Vec::new(), "{name} validates");
        assert!(packet.all_bind_matrix(), "{name} binds the matrix");
    }
}

#[test]
fn fixtures_cover_distinct_outcomes() {
    let partial: PackageOperationHistory =
        serde_json::from_str(FIXTURE_PARTIAL).expect("partial fixture");
    assert_eq!(
        partial.entries[0].result_class,
        OperationResultClass::PartiallyApplied
    );
    assert!(partial.entries[0].rollback.is_durable_recovery());

    let rollback: PackageOperationHistory =
        serde_json::from_str(FIXTURE_ROLLBACK).expect("rollback fixture");
    assert_eq!(
        rollback.entries[0].result_class,
        OperationResultClass::RolledBack
    );
    assert!(!rollback.entries[0].identity.identity_changed());

    let auth: PackageOperationHistory = serde_json::from_str(FIXTURE_AUTH).expect("auth fixture");
    assert_eq!(
        auth.entries[0].result_class,
        OperationResultClass::BlockedByAuth
    );
    assert!(auth.entries[0].impact_chain.is_empty());
}
