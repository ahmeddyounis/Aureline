use super::*;

fn packet() -> M5AuthorPublishMatrix {
    current_m5_author_publish_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_AUTHOR_PUBLISH_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_AUTHOR_PUBLISH_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_claimed_family_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.families.len(), packet.artifact_families.len());
    for &family in &packet.artifact_families {
        assert!(
            packet.family(family).is_some(),
            "missing row for family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_family_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_families_gate_consistent());
    for row in &packet.families {
        assert_eq!(
            row.published_trust_posture,
            row.effective_trust_posture(),
            "family {} publishes beyond the signing-state ceiling",
            row.family_id
        );
        assert_eq!(
            row.publish_readiness,
            row.computed_publish_readiness(),
            "family {} readiness diverges from the gate",
            row.family_id
        );
        assert_eq!(
            row.findings,
            row.computed_findings(),
            "family {} findings diverge from the gate",
            row.family_id
        );
    }
}

#[test]
fn every_family_carries_its_own_author_lane_evidence() {
    let packet = packet();
    for row in &packet.families {
        assert!(
            row.has_required_evidence(),
            "family {} is missing required author-lane refs",
            row.family_id
        );
    }
}

#[test]
fn export_projection_reflects_rows_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.families.len(), packet.families.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_families_gate_consistent,
        packet.all_families_gate_consistent()
    );
    assert_eq!(projection.ready_count, packet.ready_families().count());
    assert_eq!(
        projection.blocked_or_withheld_count,
        packet.blocked_or_withheld_families().count()
    );
    assert_eq!(
        projection.local_only_count,
        packet.local_only_families().count()
    );
    for (export, row) in projection.families.iter().zip(&packet.families) {
        assert_eq!(export.blocker_count, row.blocker_count());
        assert_eq!(export.warning_count, row.warning_count());
        assert_eq!(export.publish_ready, row.is_ready_to_publish());
    }
}

#[test]
fn finding_severity_and_domain_track_the_code() {
    let packet = packet();
    for row in &packet.families {
        for finding in &row.findings {
            assert_eq!(
                finding.severity,
                finding.code.severity(),
                "family {} finding {} has the wrong severity",
                row.family_id,
                finding.code.as_str()
            );
            assert_eq!(
                finding.domain,
                finding.code.domain(),
                "family {} finding {} has the wrong domain",
                row.family_id,
                finding.code.as_str()
            );
        }
    }
}

#[test]
fn findings_are_in_canonical_order() {
    let packet = packet();
    for row in &packet.families {
        let mut ranks: Vec<u8> = row.findings.iter().map(|f| f.code.rank()).collect();
        let sorted = {
            let mut s = ranks.clone();
            s.sort_unstable();
            s
        };
        assert_eq!(
            ranks, sorted,
            "family {} findings out of order",
            row.family_id
        );
        ranks.dedup();
        assert_eq!(
            ranks.len(),
            row.findings.len(),
            "family {} repeats a finding code",
            row.family_id
        );
    }
}

#[test]
fn runtime_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<RuntimeClass> = packet.families.iter().map(|f| f.runtime_class).collect();
    for class in RuntimeClass::ALL {
        assert!(
            present.contains(&class),
            "no family exercises runtime class {}",
            class.as_str()
        );
    }
}

#[test]
fn host_abi_classes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<HostAbiClass> = packet.families.iter().map(|f| f.host_abi).collect();
    for class in HostAbiClass::ALL {
        assert!(
            present.contains(&class),
            "no family exercises host/ABI class {}",
            class.as_str()
        );
    }
}

#[test]
fn workspace_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<WorkspaceState> =
        packet.families.iter().map(|f| f.workspace_state).collect();
    for state in WorkspaceState::ALL {
        assert!(
            present.contains(&state),
            "no family exercises workspace state {}",
            state.as_str()
        );
    }
}

#[test]
fn signature_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<SignatureState> =
        packet.families.iter().map(|f| f.signature_state).collect();
    for state in SignatureState::ALL {
        assert!(
            present.contains(&state),
            "no family exercises signing state {}",
            state.as_str()
        );
    }
}

#[test]
fn published_trust_postures_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<TrustPosture> = packet
        .families
        .iter()
        .map(|f| f.published_trust_posture)
        .collect();
    for posture in TrustPosture::ALL {
        assert!(
            present.contains(&posture),
            "no family publishes trust posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn hot_reload_postures_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<HotReloadPosture> = packet
        .families
        .iter()
        .map(|f| f.hot_reload_posture)
        .collect();
    for posture in HotReloadPosture::ALL {
        assert!(
            present.contains(&posture),
            "no family exercises hot-reload posture {}",
            posture.as_str()
        );
    }
}

#[test]
fn publish_review_requirements_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<PublishReviewRequirement> = packet
        .families
        .iter()
        .map(|f| f.publish_review_requirement)
        .collect();
    for requirement in PublishReviewRequirement::ALL {
        assert!(
            present.contains(&requirement),
            "no family exercises publish-review requirement {}",
            requirement.as_str()
        );
    }
}

#[test]
fn conformance_outputs_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<ConformanceOutput> = packet
        .families
        .iter()
        .map(|f| f.conformance_output)
        .collect();
    for output in ConformanceOutput::ALL {
        assert!(
            present.contains(&output),
            "no family exercises conformance output {}",
            output.as_str()
        );
    }
}

#[test]
fn anti_abuse_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<AntiAbuseTransparency> = packet
        .families
        .iter()
        .map(|f| f.anti_abuse_transparency)
        .collect();
    for state in AntiAbuseTransparency::ALL {
        assert!(
            present.contains(&state),
            "no family exercises anti-abuse state {}",
            state.as_str()
        );
    }
}

#[test]
fn publish_readiness_states_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<PublishReadiness> = packet
        .families
        .iter()
        .map(|f| f.publish_readiness)
        .collect();
    for readiness in PublishReadiness::ALL {
        assert!(
            present.contains(&readiness),
            "no family exercises publish readiness {}",
            readiness.as_str()
        );
    }
}

#[test]
fn finding_codes_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<PublishFindingCode> = packet
        .families
        .iter()
        .flat_map(|f| f.findings.iter().map(|finding| finding.code))
        .collect();
    for code in PublishFindingCode::ALL {
        assert!(
            present.contains(&code),
            "no family exercises finding code {}",
            code.as_str()
        );
    }
}

#[test]
fn both_severities_are_exercised() {
    let packet = packet();
    let present: BTreeSet<FindingSeverity> = packet
        .families
        .iter()
        .flat_map(|f| f.findings.iter().map(|finding| finding.severity))
        .collect();
    for severity in FindingSeverity::ALL {
        assert!(
            present.contains(&severity),
            "no finding carries severity {}",
            severity.as_str()
        );
    }
}

#[test]
fn finding_domains_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<FindingDomain> = packet
        .families
        .iter()
        .flat_map(|f| f.findings.iter().map(|finding| finding.domain))
        .collect();
    for domain in FindingDomain::ALL {
        assert!(
            present.contains(&domain),
            "no finding carries domain {}",
            domain.as_str()
        );
    }
}

#[test]
fn ready_family_is_genuinely_clean() {
    let packet = packet();
    assert!(
        packet.ready_families().count() > 0,
        "fixture needs a ready-to-publish family"
    );
    for row in packet.ready_families() {
        assert_eq!(row.workspace_state, WorkspaceState::SourcePresentBuilt);
        assert_eq!(row.signature_state, SignatureState::SignedVerified);
        assert!(matches!(
            row.hot_reload_posture,
            HotReloadPosture::NoWidening | HotReloadPosture::RelaunchOnly
        ));
        assert_eq!(
            row.anti_abuse_transparency,
            AntiAbuseTransparency::DisclosedClean
        );
        assert!(row.findings.is_empty());
        assert_eq!(row.publish_readiness, PublishReadiness::ReadyToPublish);
    }
}

#[test]
fn local_or_untrusted_artifacts_never_inherit_trusted_badges() {
    let packet = packet();
    let mut proved = 0;
    for row in &packet.families {
        if row.signature_state.is_local_or_untrusted() {
            assert_eq!(
                row.published_trust_posture,
                TrustPosture::UnsignedLocalOnly,
                "family {} inherited a trusted badge from a local/untrusted signature",
                row.family_id
            );
            proved += 1;
        }
    }
    assert!(
        proved >= 1,
        "fixture must exercise the non-inheritance guardrail"
    );
}

#[test]
fn local_artifact_with_declared_high_trust_is_capped() {
    let packet = packet();
    let row = packet
        .family(ArtifactFamily::LocalModelPack)
        .expect("local-model row");
    assert_eq!(row.signature_state, SignatureState::UnsignedLocalDev);
    assert!(row.declared_trust_posture.is_trusted_badge());
    assert_eq!(row.published_trust_posture, TrustPosture::UnsignedLocalOnly);
}

#[test]
fn hot_reload_widening_blocks_publication() {
    let packet = packet();
    for row in &packet.families {
        let widens = matches!(
            row.hot_reload_posture,
            HotReloadPosture::RuntimeClassWidenedPendingReview
                | HotReloadPosture::PermissionsWidenedPendingReview
                | HotReloadPosture::ExternalExecutableAddedPendingReview
        );
        if widens {
            assert!(
                row.findings.iter().any(|f| f.is_blocker()
                    && f.domain == FindingDomain::HotReload),
                "family {} widens authority on hot reload but carries no blocking hot-reload finding",
                row.family_id
            );
            assert_ne!(
                row.publish_readiness,
                PublishReadiness::ReadyToPublish,
                "family {} widens authority on hot reload but is still ready to publish",
                row.family_id
            );
        }
    }
    // At least one family must exercise hot-reload widening.
    assert!(packet.families.iter().any(|row| matches!(
        row.hot_reload_posture,
        HotReloadPosture::RuntimeClassWidenedPendingReview
            | HotReloadPosture::PermissionsWidenedPendingReview
            | HotReloadPosture::ExternalExecutableAddedPendingReview
    )));
}

#[test]
fn quarantined_family_is_withheld() {
    let packet = packet();
    let quarantined: Vec<_> = packet
        .families
        .iter()
        .filter(|f| f.anti_abuse_transparency.is_quarantined())
        .collect();
    assert!(
        !quarantined.is_empty(),
        "fixture needs a quarantined family"
    );
    for row in quarantined {
        assert_eq!(row.publish_readiness, PublishReadiness::WithheldQuarantined);
        assert!(row
            .findings
            .iter()
            .any(|f| f.code == PublishFindingCode::AntiAbuseQuarantined));
    }
}

#[test]
fn blockers_versus_warnings_stay_explicit() {
    let packet = packet();
    // Publishable-with-warnings rows carry only warnings; blocked rows carry at
    // least one blocker.
    for row in &packet.families {
        match row.publish_readiness {
            PublishReadiness::PublishableWithWarnings => {
                assert!(row.warning_count() > 0, "{}", row.family_id);
                assert_eq!(row.blocker_count(), 0, "{}", row.family_id);
            }
            PublishReadiness::BlockedFromPublish => {
                assert!(row.blocker_count() > 0, "{}", row.family_id);
            }
            PublishReadiness::ReadyToPublish => {
                assert_eq!(row.blocker_count(), 0, "{}", row.family_id);
                assert_eq!(row.warning_count(), 0, "{}", row.family_id);
            }
            PublishReadiness::WithheldQuarantined => {}
        }
    }
}

#[test]
fn validate_flags_overstated_trust_posture() {
    let mut packet = packet();
    if let Some(row) = packet
        .families
        .iter_mut()
        .find(|f| f.effective_trust_posture() != TrustPosture::EnterpriseApproved)
    {
        row.published_trust_posture = TrustPosture::EnterpriseApproved;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5AuthorPublishViolation::OverstatedTrustPosture { .. })));
    }
}

#[test]
fn validate_flags_local_artifact_inherited_trust() {
    // Construct an inconsistency where an unsigned local-dev row publishes a
    // verified badge; both the cap and the non-inheritance check must fire.
    let mut packet = packet();
    let row = packet
        .family(ArtifactFamily::LocalModelPack)
        .cloned()
        .expect("local-model row");
    let idx = packet
        .families
        .iter()
        .position(|f| f.family_id == row.family_id)
        .unwrap();
    packet.families[idx].published_trust_posture = TrustPosture::VerifiedPublisher;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AuthorPublishViolation::LocalArtifactInheritedTrust { .. }
    )));
}

#[test]
fn validate_flags_readiness_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet
        .families
        .iter_mut()
        .find(|f| f.publish_readiness != PublishReadiness::WithheldQuarantined)
    {
        row.publish_readiness = PublishReadiness::WithheldQuarantined;
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5AuthorPublishViolation::ReadinessMismatch { .. })));
    }
}

#[test]
fn validate_flags_findings_mismatch() {
    let mut packet = packet();
    if let Some(row) = packet.families.iter_mut().find(|f| {
        !f.findings
            .iter()
            .any(|x| x.code == PublishFindingCode::BuildFailed)
    }) {
        row.findings
            .insert(0, PublishFinding::of(PublishFindingCode::BuildFailed));
        let violations = packet.validate();
        assert!(violations
            .iter()
            .any(|v| matches!(v, M5AuthorPublishViolation::FindingsMismatch { .. })));
    }
}

#[test]
fn validate_flags_finding_severity_mismatch() {
    let mut packet = packet();
    let row = packet
        .families
        .iter_mut()
        .find(|f| !f.findings.is_empty())
        .expect("a row with findings");
    row.findings[0].severity = match row.findings[0].severity {
        FindingSeverity::Blocker => FindingSeverity::Warning,
        FindingSeverity::Warning => FindingSeverity::Blocker,
    };
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5AuthorPublishViolation::FindingSeverityMismatch { .. })));
}

#[test]
fn validate_flags_missing_family_row() {
    let mut packet = packet();
    let removed = packet.families.pop();
    assert!(removed.is_some());
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5AuthorPublishViolation::MissingFamilyRow { .. })));
}

#[test]
fn validate_flags_unclaimed_family_row() {
    let mut packet = packet();
    packet
        .artifact_families
        .retain(|f| *f != ArtifactFamily::MirroredRegistryVariant);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5AuthorPublishViolation::UnclaimedFamilyRow { .. })));
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AuthorPublishViolation::ClosedVocabularyMismatch {
            field: "artifact_families"
        }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_families = packet.summary.total_families.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&M5AuthorPublishViolation::SummaryMismatch));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        M5_AUTHOR_PUBLISH_RECORD_KIND,
        "m5_author_and_publish_preview_matrix"
    );
    assert_eq!(
        RuntimeClass::WasmCapabilitySandbox.as_str(),
        "wasm_capability_sandbox"
    );
    assert_eq!(HostAbiClass::BrowserRuntime.as_str(), "browser_runtime");
    assert_eq!(WorkspaceState::BuildFailed.as_str(), "build_failed");
    assert_eq!(
        SignatureState::UnsignedLocalDev.as_str(),
        "unsigned_local_dev"
    );
    assert_eq!(
        TrustPosture::UnsignedLocalOnly.as_str(),
        "unsigned_local_only"
    );
    assert_eq!(
        HotReloadPosture::ExternalExecutableAddedPendingReview.as_str(),
        "external_executable_added_pending_review"
    );
    assert_eq!(
        PublishReviewRequirement::NotPublishableFromLocal.as_str(),
        "not_publishable_from_local"
    );
    assert_eq!(ConformanceOutput::RetestPending.as_str(), "retest_pending");
    assert_eq!(
        AntiAbuseTransparency::PublisherLossHistoryDisclosed.as_str(),
        "publisher_loss_history_disclosed"
    );
    assert_eq!(
        PublishFindingCode::AntiAbuseQuarantined.as_str(),
        "anti_abuse_quarantined"
    );
    assert_eq!(
        PublishReadiness::WithheldQuarantined.as_str(),
        "withheld_quarantined"
    );
}

#[test]
fn trust_rank_orders_low_to_high() {
    assert!(TrustPosture::UnsignedLocalOnly.rank() < TrustPosture::RegistryBound.rank());
    assert!(TrustPosture::RegistryBound.rank() < TrustPosture::VerifiedPublisher.rank());
    assert!(TrustPosture::VerifiedPublisher.rank() < TrustPosture::EnterpriseApproved.rank());
    assert_eq!(
        TrustPosture::EnterpriseApproved.min(TrustPosture::RegistryBound),
        TrustPosture::RegistryBound
    );
}

#[test]
fn signature_trust_ceilings_hold() {
    assert_eq!(
        SignatureState::SignedVerified.trust_ceiling(),
        TrustPosture::EnterpriseApproved
    );
    assert_eq!(
        SignatureState::SignedUnverified.trust_ceiling(),
        TrustPosture::RegistryBound
    );
    assert_eq!(
        SignatureState::UnsignedLocalDev.trust_ceiling(),
        TrustPosture::UnsignedLocalOnly
    );
    assert_eq!(
        SignatureState::UnsignedSideload.trust_ceiling(),
        TrustPosture::UnsignedLocalOnly
    );
    assert_eq!(
        SignatureState::RevokedSignature.trust_ceiling(),
        TrustPosture::UnsignedLocalOnly
    );
}
