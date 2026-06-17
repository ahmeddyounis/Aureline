use super::*;

fn packet() -> PackageStateDescriptors {
    current_package_state_descriptors().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        PACKAGE_STATE_DESCRIPTORS_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, PACKAGE_STATE_DESCRIPTORS_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_descriptors() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn path_is_stable() {
    assert_eq!(
        PACKAGE_STATE_DESCRIPTORS_PATH,
        "artifacts/deps/m5/package-state-descriptors.json"
    );
}

#[test]
fn every_descriptor_binds_to_the_frozen_matrix() {
    let packet = packet();
    let matrix = current_m5_package_state_matrix().expect("matrix loads");
    assert_eq!(packet.references_matrix_id, matrix.packet_id);
    assert!(packet.all_bind_matrix());
    for descriptor in &packet.descriptors {
        for label in descriptor.applicable_labels() {
            assert!(
                matrix.state(label).is_some(),
                "descriptor {} surfaces unbound label {}",
                descriptor.descriptor_id,
                label.as_str()
            );
        }
    }
}

#[test]
fn requested_and_resolved_identity_stay_separate() {
    let packet = packet();
    assert!(packet.requested_resolved_separate());
    for descriptor in &packet.descriptors {
        assert!(
            descriptor
                .requested_labels()
                .is_disjoint(&descriptor.resolved_labels()),
            "descriptor {} conflates requested and resolved identity",
            descriptor.descriptor_id
        );
    }
}

#[test]
fn direct_and_transitive_are_never_flattened() {
    let packet = packet();
    let direct = packet
        .descriptor("psd:cargo:serde:direct")
        .expect("direct descriptor");
    let transitive = packet
        .descriptor("psd:node:lodash:transitive-advisory")
        .expect("transitive descriptor");
    assert!(direct.is_direct() && !direct.is_transitive());
    assert!(transitive.is_transitive() && !transitive.is_direct());
    assert_ne!(direct.relation(), transitive.relation());
}

#[test]
fn registry_and_path_or_vcs_are_distinguished() {
    let packet = packet();
    let registry = packet
        .descriptor("psd:cargo:serde:direct")
        .expect("registry descriptor");
    let vcs = packet
        .descriptor("psd:pip:internal-tool:vcs-license")
        .expect("vcs descriptor");
    let path = packet
        .descriptor("psd:other:vendored-lib:path")
        .expect("path descriptor");
    assert!(registry.is_registry_sourced() && !registry.is_path_or_vcs());
    assert!(vcs.is_path_or_vcs() && !vcs.is_registry_sourced());
    assert!(path.is_path_or_vcs() && !path.is_registry_sourced());
}

#[test]
fn workspace_local_member_has_no_registry_source() {
    let packet = packet();
    let member = packet
        .descriptor("psd:cargo:aureline-core:workspace")
        .expect("workspace member");
    assert!(member.is_workspace_local());
    assert!(!member.is_registry_sourced());
    assert!(!member.is_path_or_vcs());
    assert!(member
        .resolved
        .as_ref()
        .expect("resolved")
        .registry_source
        .is_none());
}

#[test]
fn policy_pinned_is_a_requested_constraint_not_a_resolved_fact() {
    let packet = packet();
    let pinned = packet
        .descriptor("psd:node:react:mirror-suppressed")
        .expect("pinned descriptor");
    assert!(pinned.is_policy_pinned());
    assert!(pinned
        .requested_labels()
        .contains(&PackageStateLabel::PolicyPinned));
    assert!(!pinned
        .resolved_labels()
        .contains(&PackageStateLabel::PolicyPinned));
}

#[test]
fn auth_gated_state_does_not_overclaim_certainty() {
    let packet = packet();
    let gated = packet
        .descriptor("psd:node:enterprise-sdk:auth-gated")
        .expect("auth-gated descriptor");
    assert!(gated.is_auth_gated());
    assert!(gated.resolved.is_none());
    assert!(!gated.can_claim_resolved_exact());
    assert!(gated.must_disclose_environment());
    assert_eq!(
        gated.primary_message_class(),
        PackageStateMessageClass::AuthRequiredDisclosure
    );
    assert!(gated.primary_message_class().is_specific());
}

#[test]
fn offline_snapshot_has_a_pin_but_cannot_claim_exact() {
    let packet = packet();
    let offline = packet
        .descriptor("psd:pip:requests:offline")
        .expect("offline descriptor");
    assert!(offline.has_pinned_ref());
    assert!(!offline.can_claim_resolved_exact());
    assert!(offline.must_disclose_environment());
    assert_eq!(
        offline.primary_message_class(),
        PackageStateMessageClass::OfflineSnapshotDisclosure
    );
}

#[test]
fn stale_state_renders_specific_disclosure_not_a_generic_failure() {
    let packet = packet();
    let stale = packet
        .descriptor("psd:cargo:ghost-dep:stale")
        .expect("stale descriptor");
    assert!(stale.resolved.is_none());
    assert!(!stale.can_claim_resolved_exact());
    assert_eq!(
        stale.primary_message_class(),
        PackageStateMessageClass::UnknownOrStaleDisclosure
    );
    assert!(stale.primary_message_class().is_specific());
}

#[test]
fn mirror_resolution_discloses_its_source_while_staying_exact() {
    let packet = packet();
    let mirror = packet
        .descriptor("psd:node:react:mirror-suppressed")
        .expect("mirror descriptor");
    assert!(mirror.can_claim_resolved_exact());
    assert!(mirror.must_disclose_environment());
    assert_eq!(
        mirror.primary_message_class(),
        PackageStateMessageClass::MirrorBackedSource
    );
}

#[test]
fn no_descriptor_collapses_into_a_generic_message() {
    let packet = packet();
    assert!(packet.no_generic_collapse());
    for descriptor in &packet.descriptors {
        assert!(
            descriptor.primary_message_class().is_specific(),
            "descriptor {} renders a generic message",
            descriptor.descriptor_id
        );
    }
}

#[test]
fn open_and_suppressed_findings_are_distinguished() {
    let packet = packet();
    let advisory = packet
        .descriptor("psd:node:lodash:transitive-advisory")
        .expect("advisory descriptor");
    let suppressed = packet
        .descriptor("psd:node:react:mirror-suppressed")
        .expect("suppressed descriptor");
    assert_eq!(advisory.open_finding_count(), 1);
    assert_eq!(suppressed.open_finding_count(), 0);
    let suppressed_card = &suppressed.finding_cards()[0];
    assert!(suppressed_card.suppressed);
    assert!(suppressed_card.suppression_ref.is_some());
    assert!(suppressed_card.expiry_label.is_some());
}

#[test]
fn license_compliance_row_is_projected() {
    let packet = packet();
    let licensed = packet
        .descriptor("psd:pip:internal-tool:vcs-license")
        .expect("license descriptor");
    let row = licensed
        .license_compliance_row()
        .expect("license row present");
    assert_eq!(row.kind, FindingKind::LicenseReviewRequired.as_str());
    assert_eq!(
        row.state_label,
        PackageStateLabel::LicenseReviewRequired.as_str()
    );
    // A clean serde descriptor has no license row.
    assert!(packet
        .descriptor("psd:cargo:serde:direct")
        .expect("serde")
        .license_compliance_row()
        .is_none());
}

#[test]
fn update_proposal_blocks_when_auth_gated_offline_or_stale() {
    let packet = packet();
    for (id, expect_apply) in [
        ("psd:cargo:serde:direct", true),
        ("psd:node:enterprise-sdk:auth-gated", false),
        ("psd:pip:requests:offline", false),
        ("psd:cargo:ghost-dep:stale", false),
    ] {
        let descriptor = packet.descriptor(id).expect("descriptor");
        let proposal = descriptor.update_proposal();
        assert_eq!(
            proposal.can_apply, expect_apply,
            "descriptor {id} apply gate mismatch"
        );
        assert_eq!(proposal.blocked_reason.is_none(), expect_apply);
    }
}

#[test]
fn the_same_descriptor_object_feeds_every_surface() {
    let packet = packet();
    let descriptor = packet
        .descriptor("psd:node:react:mirror-suppressed")
        .expect("descriptor");
    // Detail view, export row, and CLI inspect row all derive from one object.
    let view = descriptor.view();
    let export = descriptor.export_row();
    assert_eq!(view.descriptor_id, export.descriptor_id);
    assert_eq!(view.primary_message_class, export.primary_message_class);
    assert_eq!(
        view.can_claim_resolved_exact,
        export.can_claim_resolved_exact
    );
    assert_eq!(view.applicable_labels, export.applicable_labels);
}

#[test]
fn surface_projections_pin_write_authority_from_the_matrix() {
    let packet = packet();
    let descriptor = packet
        .descriptor("psd:cargo:serde:direct")
        .expect("descriptor");
    let desktop = descriptor.surface_projection(PackageSurface::DesktopPackageWorkspace);
    assert!(desktop.can_mutate);
    assert!(!desktop.redacted);
    let ai = descriptor.surface_projection(PackageSurface::AiContext);
    assert!(!ai.can_mutate);
    let support = descriptor.surface_projection(PackageSurface::SupportExport);
    assert!(!support.can_mutate);
    assert!(support.redacted);
    // The view body is identical regardless of surface.
    assert_eq!(desktop.view, ai.view);
    assert_eq!(desktop.view, support.view);
}

#[test]
fn export_projection_is_redaction_safe() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.descriptors.len());
    assert!(projection.all_consistent);
    assert!(projection.requested_resolved_separate);
    assert!(projection.no_generic_collapse);
    assert!(projection.all_bind_matrix);
    for row in &projection.rows {
        assert!(!row.redacted_source_label.contains("://"));
    }
}

#[test]
fn no_field_leaks_a_raw_url_or_token() {
    let packet = packet();
    for descriptor in &packet.descriptors {
        assert!(!descriptor.redacted_source_label.contains("://"));
        assert!(!descriptor.requested.requested_ref.contains("://"));
        if let Some(resolved) = &descriptor.resolved {
            assert!(!resolved.resolved_ref.contains("://"));
        }
    }
}

#[test]
fn ecosystems_are_exhaustively_represented() {
    let packet = packet();
    let present: BTreeSet<EcosystemKind> =
        packet.descriptors.iter().map(|d| d.ecosystem()).collect();
    for ecosystem in EcosystemKind::ALL {
        assert!(
            present.contains(&ecosystem),
            "no descriptor exercises ecosystem {}",
            ecosystem.as_str()
        );
    }
}

#[test]
fn validate_flags_resolved_confidence_mismatch() {
    let mut packet = packet();
    // Force an unresolved-confidence descriptor to carry a resolved identity.
    let descriptor = packet
        .descriptors
        .iter_mut()
        .find(|d| d.descriptor_id == "psd:cargo:ghost-dep:stale")
        .expect("stale descriptor");
    descriptor.resolved = Some(ResolvedIdentity {
        relation: DependencyRelation::Transitive,
        resolved_ref: "9.9.9".to_owned(),
        registry_source: Some(RegistrySourceAuthority::PublicRegistry),
        resolver_identity: ResolverIdentityClass::FirstPartyResolver,
        lockfile_authority: LockfileAuthority::ExactLockfilePinned,
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageStateDescriptorsViolation::ResolvedConfidenceMismatch { .. }
    )));
}

#[test]
fn validate_flags_missing_registry_source() {
    let mut packet = packet();
    let descriptor = packet
        .descriptors
        .iter_mut()
        .find(|d| d.descriptor_id == "psd:cargo:serde:direct")
        .expect("serde descriptor");
    descriptor
        .resolved
        .as_mut()
        .expect("resolved")
        .registry_source = None;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageStateDescriptorsViolation::MissingRegistrySource { .. }
    )));
}

#[test]
fn validate_flags_invalid_finding_linkage() {
    let mut packet = packet();
    let descriptor = packet
        .descriptors
        .iter_mut()
        .find(|d| d.descriptor_id == "psd:node:react:mirror-suppressed")
        .expect("suppressed descriptor");
    // Drop the suppression linkage from a suppressed finding.
    descriptor.findings[0].suppression_ref = None;
    descriptor.findings[0].expiry_label = None;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageStateDescriptorsViolation::FindingLinkageInvalid { .. }
    )));
}

#[test]
fn validate_flags_raw_url_leak() {
    let mut packet = packet();
    packet.descriptors[0].redacted_source_label = "https://secret.example.com/registry".to_owned();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, PackageStateDescriptorsViolation::RawUrlLeak { .. })));
}

#[test]
fn validate_flags_matrix_binding_mismatch() {
    let mut packet = packet();
    packet.references_matrix_id = "some-other-matrix:v9".to_owned();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageStateDescriptorsViolation::MatrixBindingMismatch { .. }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_descriptors = packet.summary.total_descriptors.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&PackageStateDescriptorsViolation::SummaryMismatch));
}

#[test]
fn validate_flags_duplicate_descriptor_id() {
    let mut packet = packet();
    let clone = packet.descriptors[0].clone();
    packet.descriptors.push(clone);
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        PackageStateDescriptorsViolation::DuplicateDescriptorId { .. }
    )));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(EcosystemKind::NodePnpm.as_str(), "node_pnpm");
    assert_eq!(DependencyRelation::PathOrVcs.as_str(), "path_or_vcs");
    assert_eq!(
        RequestedSourceKind::VersionControlRef.as_str(),
        "version_control_ref"
    );
    assert_eq!(
        ResolutionConfidence::OfflineSnapshotOnly.as_str(),
        "offline_snapshot_only"
    );
    assert_eq!(FindingKind::SuppressedUntil.as_str(), "suppressed_until");
}

#[test]
fn relation_and_finding_labels_map_to_the_frozen_vocabulary() {
    assert_eq!(
        DependencyRelation::Direct.state_label(),
        PackageStateLabel::Direct
    );
    assert_eq!(
        DependencyRelation::PathOrVcs.state_label(),
        PackageStateLabel::PathOrVcsSource
    );
    assert_eq!(
        FindingKind::AdvisoryOpen.state_label(),
        PackageStateLabel::AdvisoryOpen
    );
    // Resolved-identity labels and finding-overlay labels sit on opposite
    // identity sides, proving they are not conflated.
    assert!(DependencyRelation::Direct
        .state_label()
        .describes_resolved());
    assert_eq!(
        FindingKind::AdvisoryOpen.state_label().identity_side(),
        crate::IdentitySide::FindingOverlay
    );
}

/// Cross-ecosystem fixtures, embedded so they validate without a runtime walk.
const FIXTURE_PRIVATE_REGISTRY: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/package-state/private_registry_advisory.json"
));
const FIXTURE_WORKSPACE_LOCAL: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/package-state/workspace_local_member.json"
));
const FIXTURE_VCS_PINNED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/package-state/vcs_pinned_license_review.json"
));

#[test]
fn cross_ecosystem_fixtures_parse_and_validate() {
    for (name, json) in [
        ("private_registry_advisory", FIXTURE_PRIVATE_REGISTRY),
        ("workspace_local_member", FIXTURE_WORKSPACE_LOCAL),
        ("vcs_pinned_license_review", FIXTURE_VCS_PINNED),
    ] {
        let packet: PackageStateDescriptors =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("{name} parses: {e}"));
        assert_eq!(packet.validate(), Vec::new(), "{name} validates");
        assert!(packet.all_bind_matrix(), "{name} binds the matrix");
    }
}

#[test]
fn fixtures_cover_private_registry_workspace_local_and_vcs() {
    let private: PackageStateDescriptors =
        serde_json::from_str(FIXTURE_PRIVATE_REGISTRY).expect("private fixture");
    let private_descriptor = &private.descriptors[0];
    assert!(private_descriptor.is_transitive());
    assert_eq!(
        private_descriptor
            .resolved
            .as_ref()
            .and_then(|r| r.registry_source),
        Some(RegistrySourceAuthority::PrivateRegistry)
    );
    assert_eq!(private_descriptor.open_finding_count(), 1);

    let workspace: PackageStateDescriptors =
        serde_json::from_str(FIXTURE_WORKSPACE_LOCAL).expect("workspace fixture");
    let workspace_descriptor = &workspace.descriptors[0];
    assert!(workspace_descriptor.is_workspace_local());
    assert!(!workspace_descriptor.is_registry_sourced());

    let vcs: PackageStateDescriptors =
        serde_json::from_str(FIXTURE_VCS_PINNED).expect("vcs fixture");
    let vcs_descriptor = &vcs.descriptors[0];
    assert!(vcs_descriptor.is_path_or_vcs());
    assert!(vcs_descriptor.license_compliance_row().is_some());
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
    round_trip(&EcosystemKind::ALL);
    round_trip(&DependencyRelation::ALL);
    round_trip(&RequestedSourceKind::ALL);
    round_trip(&ResolutionConfidence::ALL);
    round_trip(&FindingKind::ALL);
}
