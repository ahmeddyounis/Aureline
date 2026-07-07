use super::*;

fn proven_input(identity: &str) -> M5ProvenanceBundleInput {
    M5ProvenanceBundleInput {
        artifact_identity_repr: identity.to_owned(),
        digest_set: vec!["sha256:aa11".to_owned()],
        signature_status: M5SignatureStatus::SignedVerified,
        attestation_status: M5AttestationStatus::AttestedVerified,
        sbom_status: M5SbomStatus::SbomComplete,
        notice_bundle_status: M5SbomStatus::SbomComplete,
        digest_lineage_state: M5DigestLineageState::ImmutableDigestPinned,
        inventory_format: M5InventoryFormat::SpdxSbom,
        inventory_scope: M5InventoryScope::FullClosure,
        inventory_freshness: M5InventoryFreshness::InventoryFresh,
        generator_version_repr: "syft-1.18.0".to_owned(),
        inventory_export: M5InventoryExportAvailability::ExportAvailableOffline,
        mirror_refs: vec!["mirror:us-east/aureline".to_owned()],
        compare_available: true,
        export_available: true,
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_fully_proven_bundle_has_no_banner() {
    let resolved = resolve_provenance_bundle(&proven_input("artifact:core")).expect("resolves");
    assert_eq!(
        resolved.trust_posture,
        M5ProvenanceTrustPosture::TrustProvenExact
    );
    assert!(resolved.is_proven);
    assert!(!resolved.is_blocked);
    assert!(!resolved.is_narrowed);
    assert!(resolved.provenance_banner.is_none());
    assert!(resolved.compare_export_binding.binding_intact);
    assert_eq!(resolved.status_rows.len(), 3);
    assert!(resolved
        .status_rows
        .iter()
        .all(|r| r.presence_does_not_imply_security));
}

#[test]
fn resolver_signed_without_attestation_is_proven_but_honest() {
    let input = M5ProvenanceBundleInput {
        attestation_status: M5AttestationStatus::NoAttestation,
        digest_lineage_state: M5DigestLineageState::RebuildDigestMatched,
        ..proven_input("artifact:cli")
    };
    let resolved = resolve_provenance_bundle(&input).expect("resolves");
    assert_eq!(
        resolved.trust_posture,
        M5ProvenanceTrustPosture::TrustSignedNotAttested
    );
    assert!(resolved.is_proven);
    assert!(resolved.provenance_banner.is_none());
    // The attestation status row records "no_attestation" and a not-provided format.
    let att_row = resolved
        .status_rows
        .iter()
        .find(|r| r.kind == M5InventoryKind::Attestation)
        .expect("attestation row present");
    assert_eq!(att_row.status_token, "no_attestation");
    assert_eq!(att_row.format, M5InventoryFormat::NotProvidedFormat);
}

#[test]
fn resolver_inventory_presence_does_not_imply_security() {
    // Verified attestation + complete SBOM, but the signing key is unverified: the
    // bundle stays narrowed, never proven.
    let input = M5ProvenanceBundleInput {
        signature_status: M5SignatureStatus::SignedUnverifiedKey,
        ..proven_input("artifact:graph")
    };
    let resolved = resolve_provenance_bundle(&input).expect("resolves");
    assert_eq!(
        resolved.trust_posture,
        M5ProvenanceTrustPosture::NarrowedSignatureUnverified
    );
    assert!(!resolved.is_proven);
    assert!(resolved.is_narrowed);
    let banner = resolved.provenance_banner.expect("banner present");
    assert_eq!(banner.reason, M5ProvenanceBlockReason::SignatureUnverified);
    assert_eq!(banner.next_action, M5ProvenanceNextAction::VerifySigningKey);
    assert!(banner.headline.to_lowercase().contains("inventory"));
}

#[test]
fn resolver_unsigned_is_never_conflated_with_signed() {
    let input = M5ProvenanceBundleInput {
        signature_status: M5SignatureStatus::Unsigned,
        attestation_status: M5AttestationStatus::NoAttestation,
        ..proven_input("artifact:mirror")
    };
    let resolved = resolve_provenance_bundle(&input).expect("resolves");
    assert_eq!(
        resolved.trust_posture,
        M5ProvenanceTrustPosture::NarrowedSignatureUnverified
    );
    assert!(!resolved.is_proven);
}

#[test]
fn resolver_broken_signature_and_lineage_block_with_distinct_reasons() {
    let broken_sig = resolve_provenance_bundle(&M5ProvenanceBundleInput {
        signature_status: M5SignatureStatus::SignatureBroken,
        ..proven_input("artifact:shell")
    })
    .expect("resolves");
    assert_eq!(
        broken_sig.trust_posture,
        M5ProvenanceTrustPosture::BlockedSignatureBroken
    );
    assert_eq!(
        broken_sig.provenance_banner.unwrap().reason,
        M5ProvenanceBlockReason::SignatureBroken
    );

    let broken_lineage = resolve_provenance_bundle(&M5ProvenanceBundleInput {
        digest_lineage_state: M5DigestLineageState::DigestLineageBroken,
        ..proven_input("artifact:registry")
    })
    .expect("resolves");
    assert_eq!(
        broken_lineage.trust_posture,
        M5ProvenanceTrustPosture::BlockedDigestLineageBroken
    );
    assert_eq!(
        broken_lineage.provenance_banner.unwrap().next_action,
        M5ProvenanceNextAction::RebuildAndReconcileDigest
    );
}

#[test]
fn resolver_unknown_state_blocks_first() {
    let input = M5ProvenanceBundleInput {
        signature_status: M5SignatureStatus::SignaturePending,
        attestation_status: M5AttestationStatus::AttestationPending,
        sbom_status: M5SbomStatus::SbomGenerating,
        digest_lineage_state: M5DigestLineageState::DigestUnverified,
        ..proven_input("artifact:preview")
    };
    let resolved = resolve_provenance_bundle(&input).expect("resolves");
    assert_eq!(
        resolved.trust_posture,
        M5ProvenanceTrustPosture::BlockedProvenanceUnknown
    );
    assert!(resolved.is_blocked);
}

#[test]
fn resolver_attestation_and_inventory_narrow_distinctly() {
    let att = resolve_provenance_bundle(&M5ProvenanceBundleInput {
        attestation_status: M5AttestationStatus::AttestedUnverified,
        ..proven_input("artifact:update")
    })
    .expect("resolves");
    assert_eq!(
        att.trust_posture,
        M5ProvenanceTrustPosture::NarrowedAttestationUnverified
    );

    let sbom = resolve_provenance_bundle(&M5ProvenanceBundleInput {
        attestation_status: M5AttestationStatus::NoAttestation,
        sbom_status: M5SbomStatus::SbomPartial,
        inventory_scope: M5InventoryScope::PartialScope,
        ..proven_input("artifact:support")
    })
    .expect("resolves");
    assert_eq!(
        sbom.trust_posture,
        M5ProvenanceTrustPosture::NarrowedInventoryIncomplete
    );
    // The Partial scope is preserved on every status row.
    assert!(sbom
        .status_rows
        .iter()
        .all(|r| r.scope == M5InventoryScope::PartialScope));
}

#[test]
fn resolver_compare_export_binding_reflects_availability() {
    let no_mirror = resolve_provenance_bundle(&M5ProvenanceBundleInput {
        mirror_refs: vec![],
        ..proven_input("artifact:core")
    })
    .expect("resolves");
    assert!(!no_mirror.compare_export_binding.mirror_provenance_preserved);
    assert!(!no_mirror.compare_export_binding.binding_intact);
    assert!(no_mirror.compare_export_binding.digest_bound);

    let no_export = resolve_provenance_bundle(&M5ProvenanceBundleInput {
        export_available: false,
        ..proven_input("artifact:core")
    })
    .expect("resolves");
    assert!(!no_export.compare_export_binding.binding_intact);
}

#[test]
fn resolver_rejects_malformed_input() {
    let empty_identity = M5ProvenanceBundleInput {
        artifact_identity_repr: "  ".to_owned(),
        ..proven_input("artifact:core")
    };
    assert_eq!(
        resolve_provenance_bundle(&empty_identity),
        Err(M5ProvenanceBundleError::EmptyArtifactIdentity)
    );

    let empty_set = M5ProvenanceBundleInput {
        digest_set: vec![],
        ..proven_input("artifact:core")
    };
    assert_eq!(
        resolve_provenance_bundle(&empty_set),
        Err(M5ProvenanceBundleError::EmptyDigestSet)
    );

    let empty_digest = M5ProvenanceBundleInput {
        digest_set: vec!["".to_owned()],
        ..proven_input("artifact:core")
    };
    assert_eq!(
        resolve_provenance_bundle(&empty_digest),
        Err(M5ProvenanceBundleError::EmptyDigest)
    );

    let forbidden = M5ProvenanceBundleInput {
        mirror_refs: vec!["https://mirror.example/aureline".to_owned()],
        ..proven_input("artifact:core")
    };
    assert_eq!(
        resolve_provenance_bundle(&forbidden),
        Err(M5ProvenanceBundleError::ForbiddenProvenanceMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_provenance_bundle_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_PROVENANCE_BUNDLE_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_provenance_bundle_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .provenance_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5ProvenanceBundleConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.provenance_rows.len(),
        M5ProvenanceBundleConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_provenance_bundle_primitive_packet();
    for row in &packet.provenance_rows {
        for part in M5ProvenanceBundleAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5ProvenanceExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ReleaseCenterAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_resolutions.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_provenance_bundle_primitive_packet();
    let cases: Vec<&M5ProvenanceBundleResolutionCase> = packet
        .provenance_rows
        .iter()
        .flat_map(|row| row.example_resolutions.iter())
        .collect();

    for posture in M5ProvenanceTrustPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.trust_posture == posture),
            "no worked resolution exercises trust posture {}",
            posture.as_str()
        );
    }
    for signature in M5SignatureStatus::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.signature_status == signature),
            "no worked resolution exercises signature status {}",
            signature.as_str()
        );
    }
    for attestation in M5AttestationStatus::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.attestation_status == attestation),
            "no worked resolution exercises attestation status {}",
            attestation.as_str()
        );
    }
    for sbom in M5SbomStatus::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.sbom_status == sbom),
            "no worked resolution exercises sbom status {}",
            sbom.as_str()
        );
    }
    for lineage in M5DigestLineageState::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.digest_lineage_state == lineage),
            "no worked resolution exercises digest-lineage state {}",
            lineage.as_str()
        );
    }
    for scope in M5InventoryScope::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.status_rows.iter().any(|r| r.scope == scope)),
            "no worked resolution exercises inventory scope {}",
            scope.as_str()
        );
    }
    for format in M5InventoryFormat::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.status_rows.iter().any(|r| r.format == format)),
            "no worked resolution exercises inventory format {}",
            format.as_str()
        );
    }
    for reason in M5ProvenanceBlockReason::ALL {
        assert!(
            cases.iter().any(|c| c
                .resolved
                .provenance_banner
                .as_ref()
                .is_some_and(|b| b.reason == reason)),
            "no worked resolution exercises block reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent() {
    let packet = seeded_m5_provenance_bundle_primitive_packet();
    for row in &packet.provenance_rows {
        for case in &row.example_resolutions {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.provenance_rows.retain(|row| {
        row.consumer_surface != M5ProvenanceBundleConsumerSurface::CliProvenanceInspect
    });
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.vocabulary_set.trust_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.provenance_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ProvenanceBundleAnatomyPart::SignatureState);
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.provenance_rows[0]
        .export_fields
        .retain(|f| *f != M5ProvenanceExportField::MirrorRefs);
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.provenance_rows[0].example_resolutions[0]
        .resolved
        .is_proven = false;
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn example_resolution_missing_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.provenance_rows[2].example_resolutions.clear();
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::ExampleResolutionMissing));
}

#[test]
fn status_row_implies_security_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.provenance_rows[0].example_resolutions[0]
        .resolved
        .status_rows[0]
        .presence_does_not_imply_security = false;
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::StatusRowImpliesSecurity));
}

#[test]
fn provenance_coverage_unproven_fails_when_no_blocked_example_present() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    for row in &mut packet.provenance_rows {
        row.example_resolutions = vec![M5ProvenanceBundleResolutionCase::resolved(proven_input(
            "artifact:core",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::ProvenanceCoverageUnproven));
}

#[test]
fn inventory_does_not_imply_security_unproven_fails_without_example() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    // Every example either proven or blocked for a non-inventory reason.
    for row in &mut packet.provenance_rows {
        row.example_resolutions = vec![
            M5ProvenanceBundleResolutionCase::resolved(proven_input("artifact:core")),
            M5ProvenanceBundleResolutionCase::resolved(M5ProvenanceBundleInput {
                signature_status: M5SignatureStatus::SignatureBroken,
                attestation_status: M5AttestationStatus::NoAttestation,
                sbom_status: M5SbomStatus::SbomMissing,
                notice_bundle_status: M5SbomStatus::SbomMissing,
                ..proven_input("artifact:shell")
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::InventoryDoesNotImplySecurityUnproven));
}

#[test]
fn not_provided_and_partial_preserved_unproven_fails_without_example() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    // Every example a full-closure proven bundle, so neither not-provided nor partial
    // appears.
    for row in &mut packet.provenance_rows {
        row.example_resolutions = vec![
            M5ProvenanceBundleResolutionCase::resolved(proven_input("artifact:core")),
            M5ProvenanceBundleResolutionCase::resolved(M5ProvenanceBundleInput {
                signature_status: M5SignatureStatus::SignatureBroken,
                attestation_status: M5AttestationStatus::AttestedVerified,
                ..proven_input("artifact:shell")
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::NotProvidedAndPartialPreservedUnproven));
}

#[test]
fn compare_export_binding_intact_unproven_fails_without_example() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    for row in &mut packet.provenance_rows {
        row.example_resolutions = vec![
            M5ProvenanceBundleResolutionCase::resolved(M5ProvenanceBundleInput {
                mirror_refs: vec![],
                sbom_status: M5SbomStatus::SbomPartial,
                inventory_scope: M5InventoryScope::PartialScope,
                signature_status: M5SignatureStatus::SignedUnverifiedKey,
                ..proven_input("artifact:core")
            }),
            M5ProvenanceBundleResolutionCase::resolved(M5ProvenanceBundleInput {
                mirror_refs: vec![],
                signature_status: M5SignatureStatus::SignatureBroken,
                ..proven_input("artifact:shell")
            }),
        ];
    }
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::CompareExportBindingIntactUnproven));
}

#[test]
fn provenance_invariant_violation_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.provenance_rows[0].conflates_signed_and_unsigned_provenance = true;
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::ProvenanceInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.provenance_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet
        .governance_review
        .trust_never_derived_from_inventory_presence = false;
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.consumer_projection.inventory_rows_read_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_provenance_bundle_primitive_packet();
    packet.release_posture.support_export_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ProvenanceBundlePrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_provenance_bundle_primitive_packet().render_markdown_summary();
    for surface in M5ProvenanceBundleConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_provenance_bundle_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(
        lines.len(),
        1 + M5ProvenanceBundleConsumerSurface::ALL.len()
    );
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5ProvenanceBundleConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_provenance_bundle_primitive_export()
        .expect("checked M5 provenance-bundle primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_PROVENANCE_BUNDLE_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_provenance_bundle_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_provenance_bundle_primitive_evaluation_provenance_sheet_beta_narrowed(),
        seeded_m5_provenance_bundle_primitive_cli_provenance_inspect_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.provenance_rows.len(),
            M5ProvenanceBundleConsumerSurface::ALL.len()
        );
    }

    let evaluation =
        seeded_m5_provenance_bundle_primitive_evaluation_provenance_sheet_beta_narrowed();
    let row = evaluation
        .provenance_rows
        .iter()
        .find(|r| {
            r.consumer_surface == M5ProvenanceBundleConsumerSurface::EvaluationProvenanceSheet
        })
        .expect("evaluation row present");
    assert_eq!(row.qualification, M5ReleaseCenterQualificationClass::Beta);

    let cli = seeded_m5_provenance_bundle_primitive_cli_provenance_inspect_preview_narrowed();
    let row = cli
        .provenance_rows
        .iter()
        .find(|r| r.consumer_surface == M5ProvenanceBundleConsumerSurface::CliProvenanceInspect)
        .expect("cli row present");
    assert_eq!(
        row.qualification,
        M5ReleaseCenterQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let evaluation: M5ProvenanceBundlePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-artifact-provenance-bundle-card-primitive/evaluation_provenance_sheet_beta_narrowed.json"
    )))
    .expect("evaluation fixture parses");
    assert!(evaluation.validate().is_empty());
    assert_eq!(
        evaluation,
        seeded_m5_provenance_bundle_primitive_evaluation_provenance_sheet_beta_narrowed()
    );

    let cli: M5ProvenanceBundlePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-artifact-provenance-bundle-card-primitive/cli_provenance_inspect_preview_narrowed.json"
    )))
    .expect("cli fixture parses");
    assert!(cli.validate().is_empty());
    assert_eq!(
        cli,
        seeded_m5_provenance_bundle_primitive_cli_provenance_inspect_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_provenance_bundle_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
