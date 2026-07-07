use super::*;

fn pinned_pack_input(name: &str) -> M5DocsPackRowResolutionInput {
    M5DocsPackRowResolutionInput {
        pack_name_repr: name.to_owned(),
        corpus_class: M5DocsCorpusClass::FirstPartyDocs,
        source_provider: M5DocsSourceProvider::FirstPartyHosted,
        version_scope: M5DocsVersionScope::PinnedRange,
        pack_state: M5DocsPackState::PinnedPack,
        freshness_state: M5DocsFreshnessState::LiveCurrent,
        verification_state: M5DocsPackVerificationState::SignatureVerified,
        item_count: 100,
        size_bytes: 1_048_576,
        signer_repr: "signer:aureline-release".to_owned(),
        refresh_time_repr: "refresh:2026-07-05T22:00Z".to_owned(),
        manage_action_target_repr: "manage:pack/example".to_owned(),
    }
}

fn drifted_finding_input(title: &str) -> M5DocsStaleExampleRowResolutionInput {
    M5DocsStaleExampleRowResolutionInput {
        finding_title_repr: title.to_owned(),
        affected_anchor_repr: "src/client.rs#Client::send".to_owned(),
        anchor_kind: M5DocsExampleAnchorKind::ApiSignature,
        corpus_class: M5DocsCorpusClass::ApiReference,
        source_provider: M5DocsSourceProvider::FirstPartyHosted,
        version_scope: M5DocsVersionScope::NearbyVersion,
        stale_example_status: M5DocsStaleExampleStatus::ApiSignatureDrifted,
        freshness_state: M5DocsFreshnessState::RecentlySynced,
        documented_version_repr: "api-1.2".to_owned(),
        current_version_repr: "api-1.4".to_owned(),
        open_current_source_target_repr: "open:source/client-send".to_owned(),
    }
}

// ---- pack resolver ------------------------------------------------------

#[test]
fn pack_resolver_pinned_verified_reads_as_pinned_current_and_live() {
    let resolved = resolve_docs_pack_row(&pinned_pack_input("core-docs")).expect("resolves");
    assert_eq!(
        resolved.trust_posture,
        M5DocsPackTrustPosture::PinnedCurrent
    );
    assert!(resolved.is_trusted_current);
    assert!(resolved.shows_as_live);
    assert!(!resolved.is_quarantined);
    assert!(resolved.is_signature_verified);
    assert!(resolved
        .available_actions
        .contains(&M5DocsPackAction::ExportPackManifest));
    assert!(resolved
        .available_actions
        .contains(&M5DocsPackAction::RemovePack));
    assert!(!resolved.disclosure_headline.trim().is_empty());
}

#[test]
fn pack_resolver_quarantine_and_verification_failure_never_read_as_trusted() {
    let quarantined = resolve_docs_pack_row(&M5DocsPackRowResolutionInput {
        pack_state: M5DocsPackState::QuarantinedPack,
        verification_state: M5DocsPackVerificationState::Unverified,
        freshness_state: M5DocsFreshnessState::UnknownFreshness,
        ..pinned_pack_input("q")
    })
    .expect("resolves");
    assert_eq!(
        quarantined.trust_posture,
        M5DocsPackTrustPosture::QuarantinedUntrusted
    );
    assert!(!quarantined.shows_as_live);
    assert!(quarantined.is_quarantined);
    assert!(quarantined
        .available_actions
        .contains(&M5DocsPackAction::ReviewQuarantine));

    let failed = resolve_docs_pack_row(&M5DocsPackRowResolutionInput {
        verification_state: M5DocsPackVerificationState::VerificationFailed,
        ..pinned_pack_input("f")
    })
    .expect("resolves");
    assert_eq!(
        failed.trust_posture,
        M5DocsPackTrustPosture::VerificationUnverified
    );
    assert!(!failed.shows_as_live);
    assert!(!failed.is_trusted_current);
}

#[test]
fn pack_resolver_keeps_pin_stale_mirror_offline_update_distinct() {
    let update = resolve_docs_pack_row(&M5DocsPackRowResolutionInput {
        pack_state: M5DocsPackState::UpdateAvailable,
        ..pinned_pack_input("u")
    })
    .expect("resolves");
    assert_eq!(update.trust_posture, M5DocsPackTrustPosture::UpdateOverdue);
    assert!(update
        .available_actions
        .contains(&M5DocsPackAction::UpdatePack));

    let stale = resolve_docs_pack_row(&M5DocsPackRowResolutionInput {
        freshness_state: M5DocsFreshnessState::StaleExpired,
        ..pinned_pack_input("s")
    })
    .expect("resolves");
    assert_eq!(
        stale.trust_posture,
        M5DocsPackTrustPosture::StaleNeedsRefresh
    );
    assert!(!stale.shows_as_live);
    assert!(stale
        .available_actions
        .contains(&M5DocsPackAction::RefreshPack));

    let offline = resolve_docs_pack_row(&M5DocsPackRowResolutionInput {
        pack_state: M5DocsPackState::OfflinePack,
        freshness_state: M5DocsFreshnessState::CachedOffline,
        ..pinned_pack_input("o")
    })
    .expect("resolves");
    assert_eq!(offline.trust_posture, M5DocsPackTrustPosture::OfflineOnly);
    assert!(!offline.shows_as_live);
    assert!(!offline
        .available_actions
        .contains(&M5DocsPackAction::TakeOffline));

    let mirror = resolve_docs_pack_row(&M5DocsPackRowResolutionInput {
        pack_state: M5DocsPackState::MirroredPack,
        freshness_state: M5DocsFreshnessState::RecentlySynced,
        ..pinned_pack_input("m")
    })
    .expect("resolves");
    assert_eq!(
        mirror.trust_posture,
        M5DocsPackTrustPosture::MirrorServedNotLive
    );
    assert!(!mirror.shows_as_live);
}

#[test]
fn pack_resolver_rejects_malformed_input() {
    let empty_name = M5DocsPackRowResolutionInput {
        pack_name_repr: "  ".to_owned(),
        ..pinned_pack_input("x")
    };
    assert_eq!(
        resolve_docs_pack_row(&empty_name),
        Err(M5DocsPackFindingResolutionError::EmptyPackName)
    );

    let empty_target = M5DocsPackRowResolutionInput {
        manage_action_target_repr: "".to_owned(),
        ..pinned_pack_input("x")
    };
    assert_eq!(
        resolve_docs_pack_row(&empty_target),
        Err(M5DocsPackFindingResolutionError::EmptyActionTarget)
    );

    let forbidden = M5DocsPackRowResolutionInput {
        signer_repr: "https://evil.test/pack".to_owned(),
        ..pinned_pack_input("x")
    };
    assert_eq!(
        resolve_docs_pack_row(&forbidden),
        Err(M5DocsPackFindingResolutionError::ForbiddenFindingMaterial)
    );
}

// ---- stale-example resolver ---------------------------------------------

#[test]
fn finding_resolver_drift_is_actionable_with_version_context() {
    let resolved =
        resolve_stale_example_row(&drifted_finding_input("sig drift")).expect("resolves");
    assert_eq!(
        resolved.drift_posture,
        M5DocsExampleDriftPosture::SignatureDriftActionable
    );
    assert!(resolved.is_actionable_drift);
    assert!(!resolved.shows_as_current);
    assert!(resolved.has_version_drift);
    assert!(resolved
        .available_actions
        .contains(&M5DocsExampleAction::CompareDrift));
    assert!(resolved
        .available_actions
        .contains(&M5DocsExampleAction::OpenCurrentSource));
    assert!(!resolved.disclosure_headline.trim().is_empty());
}

#[test]
fn finding_resolver_current_but_stale_is_held_for_reverify_not_shown_current() {
    let resolved = resolve_stale_example_row(&M5DocsStaleExampleRowResolutionInput {
        stale_example_status: M5DocsStaleExampleStatus::ExampleCurrent,
        freshness_state: M5DocsFreshnessState::StaleExpired,
        documented_version_repr: "cfg-1".to_owned(),
        current_version_repr: "cfg-1".to_owned(),
        ..drifted_finding_input("cfg")
    })
    .expect("resolves");
    assert_eq!(
        resolved.drift_posture,
        M5DocsExampleDriftPosture::ExampleCurrentPendingReverify
    );
    assert!(!resolved.shows_as_current);
    assert!(!resolved.has_version_drift);
}

#[test]
fn finding_resolver_verified_current_reads_as_current() {
    let resolved = resolve_stale_example_row(&M5DocsStaleExampleRowResolutionInput {
        stale_example_status: M5DocsStaleExampleStatus::ExampleCurrent,
        freshness_state: M5DocsFreshnessState::LiveCurrent,
        documented_version_repr: "".to_owned(),
        current_version_repr: "".to_owned(),
        ..drifted_finding_input("ok")
    })
    .expect("resolves");
    assert_eq!(
        resolved.drift_posture,
        M5DocsExampleDriftPosture::ExampleVerifiedCurrent
    );
    assert!(resolved.shows_as_current);
    assert!(!resolved.is_actionable_drift);
    assert!(!resolved
        .available_actions
        .contains(&M5DocsExampleAction::CompareDrift));
    assert!(resolved
        .available_actions
        .contains(&M5DocsExampleAction::OpenCurrentSource));
}

#[test]
fn finding_resolver_rejects_malformed_input() {
    let empty_anchor = M5DocsStaleExampleRowResolutionInput {
        affected_anchor_repr: "   ".to_owned(),
        ..drifted_finding_input("x")
    };
    assert_eq!(
        resolve_stale_example_row(&empty_anchor),
        Err(M5DocsPackFindingResolutionError::EmptyExampleAnchor)
    );

    let empty_title = M5DocsStaleExampleRowResolutionInput {
        finding_title_repr: "".to_owned(),
        ..drifted_finding_input("x")
    };
    assert_eq!(
        resolve_stale_example_row(&empty_title),
        Err(M5DocsPackFindingResolutionError::EmptyFindingTitle)
    );

    let forbidden = M5DocsStaleExampleRowResolutionInput {
        open_current_source_target_repr: "https://evil.test/src".to_owned(),
        ..drifted_finding_input("x")
    };
    assert_eq!(
        resolve_stale_example_row(&forbidden),
        Err(M5DocsPackFindingResolutionError::ForbiddenFindingMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_pack_finding_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_DOCS_PACK_FINDING_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_pack_finding_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .pack_finding_rows
        .iter()
        .map(|r| r.consumer_surface)
        .collect();
    for surface in M5DocsPackConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.pack_finding_rows.len(),
        M5DocsPackConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_pack_finding_primitive_packet();
    for row in &packet.pack_finding_rows {
        for part in M5DocsPackRowAnatomyPart::MANDATORY {
            assert!(row.pack_anatomy_parts.contains(&part));
        }
        for part in M5DocsStaleExampleRowAnatomyPart::MANDATORY {
            assert!(row.example_anatomy_parts.contains(&part));
        }
        for field in M5DocsPackFindingExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5DocsAccessibilityRoute::KeyboardFocusable));
        assert!(!row.pack_examples.is_empty());
        assert!(!row.stale_example_findings.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_pack_finding_primitive_packet();
    let packs: Vec<&M5DocsPackRowResolutionCase> = packet
        .pack_finding_rows
        .iter()
        .flat_map(|row| row.pack_examples.iter())
        .collect();
    let findings: Vec<&M5DocsStaleExampleRowResolutionCase> = packet
        .pack_finding_rows
        .iter()
        .flat_map(|row| row.stale_example_findings.iter())
        .collect();

    for state in M5DocsPackState::ALL {
        assert!(
            packs.iter().any(|c| c.resolved.pack_state == state),
            "no worked pack exercises pack state {}",
            state.as_str()
        );
    }
    for posture in M5DocsPackTrustPosture::ALL {
        assert!(
            packs.iter().any(|c| c.resolved.trust_posture == posture),
            "no worked pack exercises trust posture {}",
            posture.as_str()
        );
    }
    for verify in M5DocsPackVerificationState::ALL {
        assert!(
            packs
                .iter()
                .any(|c| c.resolved.verification_state == verify),
            "no worked pack exercises verification state {}",
            verify.as_str()
        );
    }
    for status in M5DocsStaleExampleStatus::ALL {
        assert!(
            findings
                .iter()
                .any(|c| c.resolved.stale_example_status == status),
            "no worked finding exercises stale-example status {}",
            status.as_str()
        );
    }
    for posture in M5DocsExampleDriftPosture::ALL {
        assert!(
            findings.iter().any(|c| c.resolved.drift_posture == posture),
            "no worked finding exercises drift posture {}",
            posture.as_str()
        );
    }
    for kind in M5DocsExampleAnchorKind::ALL {
        assert!(
            findings.iter().any(|c| c.resolved.anchor_kind == kind),
            "no worked finding exercises anchor kind {}",
            kind.as_str()
        );
    }
    for corpus in M5DocsCorpusClass::ALL {
        assert!(
            packs.iter().any(|c| c.resolved.corpus_class == corpus)
                || findings.iter().any(|c| c.resolved.corpus_class == corpus),
            "no worked case exercises corpus class {}",
            corpus.as_str()
        );
    }
    for provider in M5DocsSourceProvider::ALL {
        assert!(
            packs.iter().any(|c| c.resolved.source_provider == provider)
                || findings
                    .iter()
                    .any(|c| c.resolved.source_provider == provider),
            "no worked case exercises source provider {}",
            provider.as_str()
        );
    }
    for scope in M5DocsVersionScope::ALL {
        assert!(
            packs.iter().any(|c| c.resolved.version_scope == scope)
                || findings.iter().any(|c| c.resolved.version_scope == scope),
            "no worked case exercises version scope {}",
            scope.as_str()
        );
    }
    for freshness in M5DocsFreshnessState::ALL {
        assert!(
            packs
                .iter()
                .any(|c| c.resolved.freshness_state == freshness)
                || findings
                    .iter()
                    .any(|c| c.resolved.freshness_state == freshness),
            "no worked case exercises freshness state {}",
            freshness.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_anchored() {
    let packet = seeded_m5_pack_finding_primitive_packet();
    for row in &packet.pack_finding_rows {
        for case in &row.pack_examples {
            assert!(
                case.is_self_consistent(),
                "pack case for {} drifted from resolver output",
                row.consumer_surface.as_str()
            );
        }
        for case in &row.stale_example_findings {
            assert!(
                case.is_self_consistent(),
                "finding case for {} drifted from resolver output",
                row.consumer_surface.as_str()
            );
            assert!(
                !case.resolved.affected_anchor_repr.trim().is_empty(),
                "finding case for {} dropped its affected anchor",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet
        .pack_finding_rows
        .retain(|row| row.consumer_surface != M5DocsPackConsumerSurface::SupportPackEvidence);
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.vocabulary_set.trust_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn mandatory_pack_anatomy_missing_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.pack_finding_rows[0]
        .pack_anatomy_parts
        .retain(|p| *p != M5DocsPackRowAnatomyPart::PackStateBadge);
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::MandatoryPackAnatomyMissing));
}

#[test]
fn mandatory_example_anatomy_missing_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.pack_finding_rows[0]
        .example_anatomy_parts
        .retain(|p| *p != M5DocsStaleExampleRowAnatomyPart::AffectedAnchorRef);
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::MandatoryExampleAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.pack_finding_rows[0]
        .export_fields
        .retain(|f| *f != M5DocsPackFindingExportField::TrustPosture);
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::MandatoryExportFieldMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.pack_finding_rows[0].pack_examples[0]
        .resolved
        .shows_as_live = false;
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::ExampleResolutionDrift));
}

#[test]
fn example_resolution_missing_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.pack_finding_rows[2].stale_example_findings.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::ExampleResolutionMissing));
}

#[test]
fn pack_state_distinctness_unproven_fails_when_a_state_is_missing() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    // Replace every pack example with a pinned-current pack so mirror/offline/stale/
    // quarantined/update states are no longer proven.
    for row in &mut packet.pack_finding_rows {
        row.pack_examples = vec![M5DocsPackRowResolutionCase::resolved(pinned_pack_input(
            "core-docs",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::PackStateDistinctnessUnproven));
}

#[test]
fn example_drift_actionable_unproven_fails_when_no_drift_present() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    // Every finding becomes verified-current, so no actionable drift is proven.
    let current =
        M5DocsStaleExampleRowResolutionCase::resolved(M5DocsStaleExampleRowResolutionInput {
            stale_example_status: M5DocsStaleExampleStatus::ExampleCurrent,
            freshness_state: M5DocsFreshnessState::LiveCurrent,
            documented_version_repr: "".to_owned(),
            current_version_repr: "".to_owned(),
            ..drifted_finding_input("ok")
        });
    for row in &mut packet.pack_finding_rows {
        row.stale_example_findings = vec![current.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::ExampleDriftActionableUnproven));
}

#[test]
fn trust_honesty_unproven_fails_when_only_trusted_packs_present() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    for row in &mut packet.pack_finding_rows {
        row.pack_examples = vec![M5DocsPackRowResolutionCase::resolved(pinned_pack_input(
            "core-docs",
        ))];
    }
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::TrustHonestyUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.pack_finding_rows[0].hides_version_drift = true;
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.pack_finding_rows[0]
        .required_proof_packet_refs
        .clear();
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.governance_review.pack_states_stay_distinct = false;
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.consumer_projection.trust_posture_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_pack_finding_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5DocsPackFindingPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_pack_finding_primitive_packet().render_markdown_summary();
    for surface in M5DocsPackConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_pack_finding_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5DocsPackConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5DocsPackConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_pack_finding_primitive_export()
        .expect("checked M5 pack/finding primitive export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_DOCS_PACK_FINDING_PRIMITIVE_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_pack_finding_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_pack_finding_primitive_onboarding_pack_beta_narrowed(),
        seeded_m5_pack_finding_primitive_ai_pack_context_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.pack_finding_rows.len(),
            M5DocsPackConsumerSurface::ALL.len()
        );
    }

    let onboarding = seeded_m5_pack_finding_primitive_onboarding_pack_beta_narrowed();
    let row = onboarding
        .pack_finding_rows
        .iter()
        .find(|r| r.consumer_surface == M5DocsPackConsumerSurface::OnboardingPackStep)
        .expect("onboarding row present");
    assert_eq!(row.qualification, M5DocsQualificationClass::Beta);

    let ai = seeded_m5_pack_finding_primitive_ai_pack_context_preview_narrowed();
    let row = ai
        .pack_finding_rows
        .iter()
        .find(|r| r.consumer_surface == M5DocsPackConsumerSurface::AiPackContext)
        .expect("ai row present");
    assert_eq!(row.qualification, M5DocsQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let onboarding: M5DocsPackFindingPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/onboarding_pack_beta_narrowed.json"
    )))
    .expect("onboarding fixture parses");
    assert!(onboarding.validate().is_empty());
    assert_eq!(
        onboarding,
        seeded_m5_pack_finding_primitive_onboarding_pack_beta_narrowed()
    );

    let ai: M5DocsPackFindingPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/ai_pack_context_preview_narrowed.json"
    )))
    .expect("ai fixture parses");
    assert!(ai.validate().is_empty());
    assert_eq!(
        ai,
        seeded_m5_pack_finding_primitive_ai_pack_context_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_pack_finding_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
