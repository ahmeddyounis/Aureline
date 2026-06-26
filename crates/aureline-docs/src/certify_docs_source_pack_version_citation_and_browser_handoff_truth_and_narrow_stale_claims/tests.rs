use super::*;

const PACKET_ID: &str = "m5-docs-claim-certification:stable:0001";

fn packet() -> DocsClaimCertificationPacket {
    DocsClaimCertificationPacket::new(seeded_stable_docs_claim_certification_input())
}

#[test]
fn seeded_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn seeded_packet_certifies_every_profile() {
    let packet = packet();
    let present: BTreeSet<CertifiedDocsProfile> =
        packet.profile_rows.iter().map(|row| row.profile).collect();
    for profile in CertifiedDocsProfile::ALL {
        assert!(
            present.contains(&profile),
            "missing profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_every_evidence_class() {
    let packet = packet();
    let mut covered: BTreeSet<DocsEvidenceClass> = BTreeSet::new();
    for row in &packet.profile_rows {
        covered.extend(row.evidence_classes.iter().copied());
    }
    for class in DocsEvidenceClass::ALL {
        assert!(
            covered.contains(&class),
            "evidence class {} uncovered",
            class.as_str()
        );
    }
}

#[test]
fn seeded_packet_has_no_narrowed_or_blocked_profiles() {
    let packet = packet();
    assert!(packet.narrowed_profiles().is_empty());
    assert!(packet.retest_pending_profiles().is_empty());
    assert!(packet.publication_blockers().is_empty());
}

#[test]
fn every_row_evidence_refs_match_classes() {
    for row in packet().profile_rows {
        let expected_schema: BTreeSet<&str> = row
            .evidence_classes
            .iter()
            .flat_map(|class| class.evidence_schema_refs().iter().copied())
            .collect();
        let actual_schema: BTreeSet<&str> = row
            .evidence_schema_refs
            .iter()
            .map(String::as_str)
            .collect();
        assert_eq!(
            expected_schema,
            actual_schema,
            "profile {}",
            row.profile.as_str()
        );
    }
}

#[test]
fn citation_profiles_require_citation_basis() {
    for row in packet().profile_rows {
        if row
            .evidence_classes
            .contains(&DocsEvidenceClass::CitationSet)
        {
            assert!(
                row.citation_basis_required,
                "profile {} should require citation basis",
                row.profile.as_str()
            );
        }
    }
}

#[test]
fn browser_handoff_profiles_isolate_context() {
    for row in packet().profile_rows {
        if row
            .evidence_classes
            .contains(&DocsEvidenceClass::BrowserHandoff)
        {
            assert!(
                row.browser_handoff_context_isolated,
                "profile {} should isolate handoff context",
                row.profile.as_str()
            );
        }
    }
}

#[test]
fn missing_profile_fails() {
    let mut packet = packet();
    packet
        .profile_rows
        .retain(|row| row.profile != CertifiedDocsProfile::AiExplanation);
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::RequiredProfileMissing));
}

#[test]
fn evidence_ref_mismatch_fails() {
    let mut packet = packet();
    packet.profile_rows[0]
        .evidence_schema_refs
        .push("schemas/docs/wrong.schema.json".to_owned());
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::EvidenceRefMismatch));
}

#[test]
fn certified_promoted_profile_missing_evidence_fails() {
    let mut packet = packet();
    packet.profile_rows[0].evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::CertifiedProfileMissingEvidence));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.profile_rows[1].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::DowngradeTriggersMissing));
}

#[test]
fn profile_greener_than_matrix_fails() {
    let mut packet = packet();
    packet.profile_rows[2].not_greener_than_matrix = false;
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::ProfileGreenerThanMatrix));
}

#[test]
fn citation_basis_missing_fails() {
    let mut packet = packet();
    // AiExplanation depends on the citation-set class.
    for row in packet.profile_rows.iter_mut() {
        if row.profile == CertifiedDocsProfile::AiExplanation {
            row.citation_basis_required = false;
        }
    }
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::CitationBasisMissing));
}

#[test]
fn browser_handoff_context_not_isolated_fails() {
    let mut packet = packet();
    for row in packet.profile_rows.iter_mut() {
        if row.profile == CertifiedDocsProfile::DocsBrowser {
            row.browser_handoff_context_isolated = false;
        }
    }
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::BrowserHandoffContextNotIsolated));
}

#[test]
fn verdict_qualification_mismatch_fails() {
    let mut packet = packet();
    // Held qualification with a Certified (publication-permitting) verdict.
    packet.profile_rows[0].qualification = DocsClaimQualificationClass::Held;
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::VerdictQualificationMismatch));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::MissingSourceContracts));
}

#[test]
fn missing_evidence_corpus_fails() {
    let mut packet = packet();
    packet.evidence_corpus_refs.clear();
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::MissingEvidenceCorpus));
}

#[test]
fn compatibility_report_incomplete_fails() {
    let mut packet = packet();
    packet.compatibility_report.no_profile_greener_than_matrix = false;
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::CompatibilityReportIncomplete));
}

#[test]
fn downgrade_rules_incomplete_fails() {
    let mut packet = packet();
    packet.downgrade_rules[0].auto_enforced = false;
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::DowngradeRulesIncomplete));
}

#[test]
fn missing_evidence_class_rule_fails() {
    let mut packet = packet();
    // Drop every rule covering the browser-handoff staleness trigger.
    packet
        .downgrade_rules
        .retain(|rule| rule.trigger != DocsClaimDowngradeTrigger::BrowserHandoffEvidenceStale);
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::DowngradeRulesIncomplete));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_profile_greener_than_packet = false;
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .narrowed_profiles_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&DocsClaimCertificationViolation::ProofFreshnessIncomplete));
}

#[test]
fn auto_narrow_for_stale_source_class_marks_retest_pending() {
    let narrowed = packet().narrowed_for_stale_evidence(&[DocsEvidenceClass::SourceClass]);
    // Every profile depends on source class, so all become retest-pending.
    assert!(narrowed.validate().is_empty(), "{:?}", narrowed.validate());
    let pending = narrowed.retest_pending_profiles();
    for profile in CertifiedDocsProfile::ALL {
        assert!(
            pending.contains(&profile),
            "profile {} should be retest-pending",
            profile.as_str()
        );
    }
    // Retest-pending is a narrow, not a hard block.
    assert!(narrowed.publication_blockers().is_empty());
}

#[test]
fn auto_narrow_for_stale_citation_set_only_touches_citation_profiles() {
    let narrowed = packet().narrowed_for_stale_evidence(&[DocsEvidenceClass::CitationSet]);
    assert!(narrowed.validate().is_empty(), "{:?}", narrowed.validate());
    let pending = narrowed.retest_pending_profiles();
    // HelpAbout does not depend on the citation set, so it stays certified.
    assert!(!pending.contains(&CertifiedDocsProfile::HelpAbout));
    assert!(pending.contains(&CertifiedDocsProfile::AiExplanation));
    assert!(pending.contains(&CertifiedDocsProfile::OnboardingLearning));
}

#[test]
fn markdown_summary_lists_every_profile() {
    let summary = packet().render_markdown_summary();
    for profile in CertifiedDocsProfile::ALL {
        assert!(
            summary.contains(profile.as_str()),
            "summary missing profile {}",
            profile.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_docs_claim_certification_export()
        .expect("checked certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(packet.profile_rows.len(), CertifiedDocsProfile::ALL.len());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    let retest: DocsClaimCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims/source_class_evidence_stale_retest_pending.json"
    )))
    .expect("retest-pending fixture parses");
    assert!(retest.validate().is_empty(), "{:?}", retest.validate());
    assert!(retest
        .retest_pending_profiles()
        .contains(&CertifiedDocsProfile::DocsBrowser));
    assert!(retest.publication_blockers().is_empty());

    let blocked: DocsClaimCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims/browser_handoff_evidence_stale_blocks_publication.json"
    )))
    .expect("browser-handoff-blocked fixture parses");
    assert!(blocked.validate().is_empty(), "{:?}", blocked.validate());
    assert!(blocked
        .publication_blockers()
        .contains(&CertifiedDocsProfile::AiExplanation));
}
