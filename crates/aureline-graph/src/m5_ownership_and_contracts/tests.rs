use super::*;

fn packet() -> M5OwnershipContractPacket {
    current_m5_ownership_and_contracts_packet().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_OWNERSHIP_CONTRACTS_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_OWNERSHIP_CONTRACTS_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_body() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_source_class_is_exercised() {
    // Source kind stays visible: the corpus distinguishes curated, policy-derived, imported, and
    // heuristic answers so a derived hint never collapses into curated truth.
    let packet = packet();
    let classes: BTreeSet<OwnershipSourceClass> =
        packet.descriptors.iter().map(|d| d.source_class).collect();
    for class in OwnershipSourceClass::ALL {
        assert!(
            classes.contains(&class),
            "no descriptor exercises source class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_role_is_exercised_distinctly() {
    // Owner, reviewer, maintainer, support contact, and change-control links stay distinct rather
    // than collapsing into one generic owner field.
    let packet = packet();
    let roles: BTreeSet<OwnershipRole> = packet.descriptors.iter().map(|d| d.role).collect();
    for role in OwnershipRole::ALL {
        assert!(
            roles.contains(&role),
            "no descriptor exercises role {}",
            role.as_str()
        );
    }
}

#[test]
fn every_visibility_is_exercised() {
    let packet = packet();
    let visibilities: BTreeSet<OwnershipVisibility> =
        packet.descriptors.iter().map(|d| d.visibility).collect();
    for visibility in OwnershipVisibility::ALL {
        assert!(
            visibilities.contains(&visibility),
            "no descriptor exercises visibility {}",
            visibility.as_str()
        );
    }
}

#[test]
fn every_inferred_descriptor_carries_a_source_reason() {
    let packet = packet();
    assert!(packet.all_inferred_descriptors_labeled());
    for descriptor in &packet.descriptors {
        if descriptor.source_class.requires_source_reason() {
            assert!(
                descriptor
                    .source_reason
                    .as_ref()
                    .is_some_and(|reason| !reason.trim().is_empty()),
                "descriptor {} ({}) carries no source_reason",
                descriptor.descriptor_id,
                descriptor.source_class.as_str()
            );
        }
    }
}

#[test]
fn inference_never_overwrites_curated_truth() {
    let packet = packet();
    assert!(packet.inference_never_overwrites_curated());
    for descriptor in &packet.descriptors {
        if descriptor.source_class.is_inferred_or_imported() {
            for superseded_id in &descriptor.supersedes {
                let target = packet.descriptor(superseded_id).expect("declared target");
                assert!(
                    !target.source_class.is_authoritative(),
                    "{} ({}) overwrites authoritative {} ({})",
                    descriptor.descriptor_id,
                    descriptor.source_class.as_str(),
                    superseded_id,
                    target.source_class.as_str()
                );
            }
        }
    }
}

#[test]
fn authoritative_descriptor_follows_precedence() {
    // For a subject with both curated and heuristic answers in the same role, curated wins.
    let packet = packet();
    let authoritative = packet
        .authoritative_descriptor("node:dir:crates/auth", OwnershipRole::Owner)
        .expect("auth dir owner exists");
    assert_eq!(authoritative.source_class, OwnershipSourceClass::Curated);
}

#[test]
fn change_control_links_are_kept_distinct() {
    let packet = packet();
    for descriptor in &packet.descriptors {
        assert!(
            descriptor.change_control_link_is_well_formed(),
            "descriptor {} has a malformed change-control link",
            descriptor.descriptor_id
        );
        if descriptor.role.is_change_control() {
            assert!(descriptor.change_control_url.is_some());
        } else {
            assert!(descriptor.change_control_url.is_none());
        }
    }
}

#[test]
fn every_descriptor_carries_an_export_safe_permalink() {
    let packet = packet();
    for descriptor in &packet.descriptors {
        assert!(
            descriptor.permalink_is_export_safe(),
            "descriptor {} has an unsafe permalink",
            descriptor.descriptor_id
        );
        assert_eq!(
            packet.permalink_for_descriptor(&descriptor.descriptor_id),
            Some(descriptor.export_permalink.as_str())
        );
    }
}

#[test]
fn every_surface_has_exactly_one_binding_that_preserves_labels() {
    let packet = packet();
    assert_eq!(
        packet.consumer_bindings.len(),
        OwnershipConsumerSurface::ALL.len(),
        "one binding per surface"
    );
    for surface in OwnershipConsumerSurface::ALL {
        let binding = packet
            .consumer_binding(surface)
            .unwrap_or_else(|| panic!("missing binding for surface {}", surface.as_str()));
        assert!(
            binding.preserves_source_labels,
            "binding for {} flattens source labels",
            surface.as_str()
        );
    }
}

#[test]
fn every_binding_is_stamped_with_the_active_snapshot() {
    let packet = packet();
    for binding in &packet.consumer_bindings {
        assert_eq!(binding.snapshot_id, packet.active_scope.snapshot_id);
        assert_eq!(binding.scope_id, packet.active_scope.scope_id);
    }
}

#[test]
fn no_binding_carries_a_descriptor_beyond_its_visibility_ceiling() {
    let packet = packet();
    for binding in &packet.consumer_bindings {
        for descriptor_id in &binding.carries_descriptor_ids {
            let descriptor = packet
                .descriptor(descriptor_id)
                .expect("declared descriptor");
            assert!(
                descriptor.visibility.fits_within(binding.max_visibility),
                "binding {} carries {} descriptor {} beyond its {} ceiling",
                binding.binding_id,
                descriptor.visibility.as_str(),
                descriptor_id,
                binding.max_visibility.as_str()
            );
        }
    }
}

#[test]
fn support_export_carries_every_export_safe_descriptor_and_no_private() {
    // Support and enterprise review can cite ownership without a private dashboard lookup, and the
    // restricted descriptor never widens into the export.
    let packet = packet();
    assert!(packet.every_export_safe_descriptor_in_support_export());
    assert!(packet.no_private_in_support_export());
    let binding = packet
        .consumer_binding(OwnershipConsumerSurface::SupportExport)
        .expect("support export binding");
    for descriptor_id in &binding.carries_descriptor_ids {
        let descriptor = packet
            .descriptor(descriptor_id)
            .expect("declared descriptor");
        assert!(
            descriptor.visibility.is_export_safe(),
            "support export carries non-export-safe descriptor {descriptor_id}"
        );
    }
}

#[test]
fn packet_binds_to_canonical_upstream_packets() {
    let packet = packet();
    assert_eq!(
        packet.governance_matrix_ref,
        M5_OWNERSHIP_CONTRACTS_GOVERNANCE_MATRIX_REF
    );
    assert_eq!(
        packet.scope_packet_ref,
        M5_OWNERSHIP_CONTRACTS_SCOPE_PACKET_REF
    );
    assert_eq!(
        packet.topology_packet_ref,
        M5_OWNERSHIP_CONTRACTS_TOPOLOGY_PACKET_REF
    );
}

#[test]
fn export_projection_redacts_private_and_reflects_guardrails() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.snapshot_id, packet.active_scope.snapshot_id);
    assert_eq!(projection.scope_id, packet.active_scope.scope_id);
    // Private descriptors are withheld from the export entirely.
    let private_count = packet
        .descriptors
        .iter()
        .filter(|d| !d.is_export_safe())
        .count();
    assert_eq!(projection.redacted_private_count, private_count);
    assert_eq!(
        projection.descriptors.len(),
        packet.descriptors.len() - private_count
    );
    for row in &projection.descriptors {
        assert_ne!(row.visibility, OwnershipVisibility::Private.as_str());
        assert!(!row.permalink.trim().is_empty());
        assert!(row.permalink.contains(&row.descriptor_id));
    }
    assert!(projection.curated_truth_preserved);
    assert!(projection.all_inferred_descriptors_labeled);
    assert!(projection.source_labels_preserved_everywhere);
    assert!(projection.every_export_safe_descriptor_in_support_export);
    assert!(projection.no_private_in_support_export);
}

#[test]
fn validate_flags_inference_overwriting_curated() {
    let mut packet = packet();
    // Point a heuristic descriptor at a curated descriptor it must not overwrite.
    let curated_id = packet
        .descriptors
        .iter()
        .find(|d| d.source_class == OwnershipSourceClass::Curated)
        .map(|d| d.descriptor_id.clone())
        .expect("a curated descriptor exists");
    if let Some(descriptor) = packet
        .descriptors
        .iter_mut()
        .find(|d| d.source_class.is_inferred_or_imported())
    {
        descriptor.supersedes = vec![curated_id];
        packet.summary = packet.computed_summary();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5OwnershipContractViolation::InferenceOverwritesCurated { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_source_reason() {
    let mut packet = packet();
    if let Some(descriptor) = packet
        .descriptors
        .iter_mut()
        .find(|d| d.source_class.requires_source_reason())
    {
        descriptor.source_reason = None;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5OwnershipContractViolation::MissingSourceReason { .. })));
    }
}

#[test]
fn validate_flags_change_control_without_link() {
    let mut packet = packet();
    if let Some(descriptor) = packet
        .descriptors
        .iter_mut()
        .find(|d| d.role.is_change_control())
    {
        descriptor.change_control_url = None;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5OwnershipContractViolation::ChangeControlWithoutLink { .. }
        )));
    }
}

#[test]
fn validate_flags_non_change_control_with_link() {
    let mut packet = packet();
    if let Some(descriptor) = packet
        .descriptors
        .iter_mut()
        .find(|d| !d.role.is_change_control())
    {
        descriptor.change_control_url = Some("https://example.com/policy".to_owned());
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5OwnershipContractViolation::NonChangeControlWithLink { .. }
        )));
    }
}

#[test]
fn validate_flags_private_descriptor_in_support_export() {
    let mut packet = packet();
    let private_id = packet
        .descriptors
        .iter()
        .find(|d| !d.is_export_safe())
        .map(|d| d.descriptor_id.clone())
        .expect("a private descriptor exists");
    if let Some(binding) = packet
        .consumer_bindings
        .iter_mut()
        .find(|b| b.surface == OwnershipConsumerSurface::SupportExport)
    {
        binding.carries_descriptor_ids.push(private_id);
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5OwnershipContractViolation::PrivateDescriptorInSupportExport { .. }
        )));
    }
}

#[test]
fn validate_flags_visibility_exceeding_binding() {
    let mut packet = packet();
    // Drop the support-export ceiling to public so its internal descriptors exceed it.
    if let Some(binding) = packet
        .consumer_bindings
        .iter_mut()
        .find(|b| b.surface == OwnershipConsumerSurface::SupportExport)
    {
        binding.max_visibility = OwnershipVisibility::Public;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5OwnershipContractViolation::VisibilityExceedsBinding { .. }
        )));
    }
}

#[test]
fn validate_flags_source_labels_not_preserved() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.preserves_source_labels = false;
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5OwnershipContractViolation::SourceLabelsNotPreserved { .. }
        )));
    }
}

#[test]
fn validate_flags_export_safe_descriptor_missing_from_support_export() {
    let mut packet = packet();
    if let Some(binding) = packet
        .consumer_bindings
        .iter_mut()
        .find(|b| b.surface == OwnershipConsumerSurface::SupportExport)
    {
        binding.carries_descriptor_ids.clear();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5OwnershipContractViolation::ExportSafeDescriptorMissingFromSupportExport { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_surface_binding() {
    let mut packet = packet();
    packet
        .consumer_bindings
        .retain(|b| b.surface != OwnershipConsumerSurface::OnboardingContext);
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5OwnershipContractViolation::MissingSurfaceBinding { .. }
    )));
}

#[test]
fn validate_flags_snapshot_binding_mismatch() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.snapshot_id = "workset-scope:snapshot:stale".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5OwnershipContractViolation::SnapshotBindingMismatch { .. }
        )));
    }
}

#[test]
fn validate_flags_unsafe_descriptor_permalink() {
    let mut packet = packet();
    if let Some(descriptor) = packet.descriptors.first_mut() {
        descriptor.export_permalink =
            "aureline://workspace:aureline/ownership/descriptor/mismatch".to_owned();
        let violations = packet.validate();
        assert!(violations.iter().any(|v| matches!(
            v,
            M5OwnershipContractViolation::UnsafeDescriptorPermalink { .. }
        )));
    }
}

#[test]
fn validate_flags_governance_ref_mismatch() {
    let mut packet = packet();
    packet.governance_matrix_ref = "artifacts/graph/m5/not-the-matrix.json".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&M5OwnershipContractViolation::GovernanceMatrixRefMismatch));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.descriptor_count = packet.summary.descriptor_count.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5OwnershipContractViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(OwnershipSourceClass::Curated.as_str(), "curated");
    assert_eq!(
        OwnershipSourceClass::PolicyDerived.as_str(),
        "policy_derived"
    );
    assert_eq!(OwnershipSourceClass::Imported.as_str(), "imported");
    assert_eq!(OwnershipSourceClass::Heuristic.as_str(), "heuristic");
    assert_eq!(OwnershipRole::Owner.as_str(), "owner");
    assert_eq!(OwnershipRole::SupportContact.as_str(), "support_contact");
    assert_eq!(OwnershipRole::ChangeControl.as_str(), "change_control");
    assert_eq!(OwnershipVisibility::Public.as_str(), "public");
    assert_eq!(OwnershipVisibility::Private.as_str(), "private");
    assert_eq!(
        OwnershipConsumerSurface::AiOwnershipSuggestion.as_str(),
        "ai_ownership_suggestion"
    );
    assert_eq!(
        OwnershipConsumerSurface::SupportExport.as_str(),
        "support_export"
    );
    assert!(OwnershipSourceClass::Curated.is_authoritative());
    assert!(OwnershipSourceClass::PolicyDerived.is_authoritative());
    assert!(OwnershipSourceClass::Heuristic.is_inferred_or_imported());
    assert!(OwnershipSourceClass::Curated.outranks(OwnershipSourceClass::Heuristic));
    assert!(!OwnershipSourceClass::Heuristic.outranks(OwnershipSourceClass::Curated));
    assert!(OwnershipRole::ChangeControl.is_change_control());
    assert!(!OwnershipRole::Owner.is_change_control());
    assert!(OwnershipVisibility::Internal.is_export_safe());
    assert!(!OwnershipVisibility::Private.is_export_safe());
    assert!(OwnershipVisibility::Public.is_public_safe());
    assert!(OwnershipVisibility::Public.fits_within(OwnershipVisibility::Internal));
    assert!(!OwnershipVisibility::Private.fits_within(OwnershipVisibility::Internal));
    assert!(OwnershipConsumerSurface::SupportExport.is_support_export());
}
