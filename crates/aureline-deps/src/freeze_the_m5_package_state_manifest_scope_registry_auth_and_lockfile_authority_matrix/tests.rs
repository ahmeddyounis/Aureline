use super::*;

fn packet() -> M5PackageStateMatrix {
    current_m5_package_state_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_PACKAGE_STATE_MATRIX_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_PACKAGE_STATE_MATRIX_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_label_has_exactly_one_state_row() {
    let packet = packet();
    assert_eq!(packet.state_rows.len(), packet.package_state_labels.len());
    for &label in &packet.package_state_labels {
        assert!(
            packet.state(label).is_some(),
            "missing row for label {}",
            label.as_str()
        );
    }
}

#[test]
fn every_registry_source_has_exactly_one_cell() {
    let packet = packet();
    assert_eq!(
        packet.registry_source_cells.len(),
        packet.registry_source_classes.len()
    );
    for &source in &packet.registry_source_classes {
        assert!(
            packet.registry_cell(source).is_some(),
            "missing cell for source {}",
            source.as_str()
        );
    }
}

#[test]
fn every_surface_binds_to_this_matrix() {
    let packet = packet();
    assert_eq!(packet.surface_bindings.len(), packet.package_surfaces.len());
    assert!(packet.all_surfaces_reference_matrix());
    for binding in &packet.surface_bindings {
        assert_eq!(
            binding.references_matrix_id,
            packet.packet_id,
            "surface {} references the wrong matrix",
            binding.surface.as_str()
        );
        assert_eq!(
            binding.write_authority,
            binding.surface.canonical_write_authority(),
            "surface {} carries the wrong write authority",
            binding.surface.as_str()
        );
    }
}

#[test]
fn every_retention_subject_has_exactly_one_rule() {
    let packet = packet();
    assert_eq!(
        packet.retention_rules.len(),
        packet.retention_subjects.len()
    );
    assert!(packet.all_retention_consistent());
    for &subject in &packet.retention_subjects {
        assert!(
            packet.retention_rule(subject).is_some(),
            "missing rule for subject {}",
            subject.as_str()
        );
    }
}

#[test]
fn every_part_is_consistent_with_the_contract() {
    let packet = packet();
    assert!(packet.all_consistent());
    for row in &packet.state_rows {
        assert_eq!(row.identity_side, row.label.identity_side());
        assert_eq!(row.message_class, row.label.canonical_message_class());
        assert_eq!(
            row.non_collapse_guarded,
            row.label.is_non_collapse_guarded()
        );
    }
    for cell in &packet.registry_source_cells {
        assert_eq!(cell.message_class, cell.source.canonical_message_class());
        assert_eq!(
            cell.requires_specific_disclosure,
            cell.source.requires_specific_disclosure()
        );
    }
}

#[test]
fn requested_and_resolved_truth_stay_separate() {
    let packet = packet();
    for row in &packet.state_rows {
        assert!(
            !(row.describes_requested() && row.describes_resolved()),
            "row {} conflates requested and resolved identity",
            row.row_id
        );
    }
    // The fixture exercises both sides separately.
    assert!(
        packet.state_rows.iter().any(|r| r.describes_requested()),
        "fixture needs a requested-side label"
    );
    assert!(
        packet.state_rows.iter().any(|r| r.describes_resolved()),
        "fixture needs a resolved-side label"
    );
}

#[test]
fn no_state_or_source_collapses_into_a_generic_message() {
    let packet = packet();
    assert!(packet.no_generic_collapse());
    for row in &packet.state_rows {
        assert!(
            row.message_class.is_specific(),
            "label {} renders a generic message",
            row.label.as_str()
        );
    }
    for cell in &packet.registry_source_cells {
        assert!(
            cell.message_class.is_specific(),
            "source {} renders a generic message",
            cell.source.as_str()
        );
    }
}

#[test]
fn offline_and_auth_required_states_are_guarded() {
    let packet = packet();
    for label in [
        PackageStateLabel::OfflineSnapshotOnly,
        PackageStateLabel::AuthRequired,
        PackageStateLabel::UnknownOrStale,
    ] {
        let row = packet.state(label).expect("guarded row present");
        assert!(
            row.non_collapse_guarded,
            "label {} is not guarded",
            label.as_str()
        );
        assert!(row.message_class.is_specific());
    }
}

#[test]
fn mirror_cache_and_offline_sources_disclose_specifically() {
    let packet = packet();
    for source in [
        RegistrySourceAuthority::EnterpriseMirror,
        RegistrySourceAuthority::LocalCache,
        RegistrySourceAuthority::OfflineSnapshot,
    ] {
        let cell = packet.registry_cell(source).expect("cell present");
        assert!(
            cell.requires_specific_disclosure,
            "source {} must disclose specifically",
            source.as_str()
        );
        assert!(cell.message_class.is_specific());
    }
}

#[test]
fn registry_cells_never_export_raw_secrets() {
    let packet = packet();
    for cell in &packet.registry_source_cells {
        assert!(!cell.redacted_source_label.trim().is_empty());
        assert!(
            !cell.redacted_source_label.contains("://"),
            "source {} label leaks a raw URL",
            cell.source.as_str()
        );
    }
}

#[test]
fn registry_credentials_are_never_persisted() {
    let packet = packet();
    let rule = packet
        .retention_rule(RetentionSubject::RegistryCredentials)
        .expect("credential rule present");
    assert!(!rule.stores_credential_body);
    assert!(rule.redaction_required);
    assert_eq!(
        rule.retention_class,
        RetentionClass::BrokerResolvedNeverPersisted
    );
}

#[test]
fn no_retention_rule_stores_a_credential_body() {
    let packet = packet();
    for rule in &packet.retention_rules {
        assert!(
            !rule.stores_credential_body,
            "subject {} stores a credential body",
            rule.subject.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_rows_and_contract() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.states.len(), packet.state_rows.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.all_consistent, packet.all_consistent());
    assert_eq!(
        projection.all_surfaces_reference_matrix,
        packet.all_surfaces_reference_matrix()
    );
    assert_eq!(projection.no_generic_collapse, packet.no_generic_collapse());
    assert_eq!(
        projection.non_collapse_guarded_count,
        packet
            .state_rows
            .iter()
            .filter(|r| r.non_collapse_guarded)
            .count()
    );
}

#[test]
fn identity_sides_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<IdentitySide> =
        packet.state_rows.iter().map(|r| r.identity_side).collect();
    for side in IdentitySide::ALL {
        assert!(
            present.contains(&side),
            "no state row exercises identity side {}",
            side.as_str()
        );
    }
}

#[test]
fn mutating_surfaces_are_present() {
    let packet = packet();
    assert!(
        packet
            .surface_bindings
            .iter()
            .any(|b| b.write_authority.can_mutate()),
        "fixture needs a mutating surface"
    );
    let desktop = packet
        .binding(PackageSurface::DesktopPackageWorkspace)
        .expect("desktop binding");
    assert!(desktop.write_authority.can_mutate());
    let ai = packet
        .binding(PackageSurface::AiContext)
        .expect("ai binding");
    assert_eq!(ai.write_authority, SurfaceWriteAuthority::InspectOnly);
}

#[test]
fn validate_flags_identity_side_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .state_rows
        .iter_mut()
        .find(|r| r.identity_side != IdentitySide::IndeterminateState)
    {
        row.identity_side = IdentitySide::IndeterminateState;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5PackageStateMatrixViolation::IdentitySideMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_generic_collapse_message() {
    let mut packet = packet();
    packet.state_rows[0].message_class = PackageStateMessageClass::GenericPackageNotFound;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5PackageStateMatrixViolation::GenericCollapseMessage { .. }
    )));
}

#[test]
fn validate_flags_message_class_mismatch() {
    let mut packet = packet();
    let row = packet
        .state_rows
        .iter_mut()
        .find(|r| r.label == PackageStateLabel::Direct)
        .expect("direct row");
    row.message_class = PackageStateMessageClass::TransitiveDependency;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5PackageStateMatrixViolation::MessageClassMismatch { .. }
    )));
}

#[test]
fn validate_flags_surface_referencing_wrong_matrix() {
    let mut packet = packet();
    packet.surface_bindings[0].references_matrix_id = "some-other-matrix".to_owned();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5PackageStateMatrixViolation::SurfaceReferencesWrongMatrix { .. }
    )));
}

#[test]
fn validate_flags_credential_body_stored() {
    let mut packet = packet();
    let rule = packet
        .retention_rules
        .iter_mut()
        .find(|r| r.subject == RetentionSubject::RegistryCredentials)
        .expect("credential rule");
    rule.stores_credential_body = true;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5PackageStateMatrixViolation::CredentialBodyStored { .. }
    )));
}

#[test]
fn validate_flags_missing_state_row() {
    let mut packet = packet();
    let removed = packet.state_rows.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5PackageStateMatrixViolation::MissingStateRow { .. })));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_state_rows = packet.summary.total_state_rows.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5PackageStateMatrixViolation::SummaryMismatch));
}

#[test]
fn validate_flags_closed_vocabulary_mismatch() {
    let mut packet = packet();
    packet
        .package_state_labels
        .retain(|l| *l != PackageStateLabel::UnknownOrStale);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5PackageStateMatrixViolation::ClosedVocabularyMismatch {
            field: "package_state_labels"
        }
    )));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(PackageStateLabel::Direct.as_str(), "direct");
    assert_eq!(
        PackageStateLabel::OfflineSnapshotOnly.as_str(),
        "offline_snapshot_only"
    );
    assert_eq!(PackageStateLabel::AuthRequired.as_str(), "auth_required");
    assert_eq!(IdentitySide::ResolvedIdentity.as_str(), "resolved_identity");
    assert_eq!(
        PackageStateMessageClass::GenericPackageNotFound.as_str(),
        "generic_package_not_found"
    );
    assert_eq!(
        RegistrySourceAuthority::EnterpriseMirror.as_str(),
        "enterprise_mirror"
    );
    assert_eq!(
        AuthMode::AuthRequiredUnsatisfied.as_str(),
        "auth_required_unsatisfied"
    );
    assert_eq!(
        LockfileAuthority::LockfileDivergent.as_str(),
        "lockfile_divergent"
    );
    assert_eq!(
        ResolverIdentityClass::MirrorBackedResolver.as_str(),
        "mirror_backed_resolver"
    );
    assert_eq!(RollbackClass::Irreversible.as_str(), "irreversible");
    assert_eq!(
        RetentionSubject::RegistryCredentials.as_str(),
        "registry_credentials"
    );
    assert_eq!(SurfaceWriteAuthority::Mutates.as_str(), "mutates");
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
    round_trip(&PackageStateLabel::ALL);
    round_trip(&IdentitySide::ALL);
    round_trip(&PackageStateMessageClass::ALL);
    round_trip(&ManifestScopeClass::ALL);
    round_trip(&RegistrySourceAuthority::ALL);
    round_trip(&AuthMode::ALL);
    round_trip(&LockfileAuthority::ALL);
    round_trip(&ResolverIdentityClass::ALL);
    round_trip(&RollbackClass::ALL);
    round_trip(&RetentionSubject::ALL);
    round_trip(&RetentionClass::ALL);
    round_trip(&PackageSurface::ALL);
    round_trip(&SurfaceWriteAuthority::ALL);
}

#[test]
fn whole_workspace_scope_requires_explicit_confirmation() {
    assert!(ManifestScopeClass::WholeWorkspace.requires_explicit_confirmation());
    assert!(!ManifestScopeClass::SelectedManifest.requires_explicit_confirmation());
}

#[test]
fn path_is_stable() {
    assert_eq!(
        M5_PACKAGE_STATE_MATRIX_PATH,
        "artifacts/deps/m5/freeze-the-m5-package-state-manifest-scope-registry-auth-and-lockfile-authority-matrix.json"
    );
}
