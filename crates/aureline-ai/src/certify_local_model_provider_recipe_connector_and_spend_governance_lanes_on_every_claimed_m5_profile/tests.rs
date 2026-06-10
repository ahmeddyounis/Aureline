use super::*;

const PACKET_ID: &str = "m5-lane-certification:stable:0001";

/// A fully passing disclosure scorecard covering every required dimension.
fn passing_scorecard() -> Vec<LaneDisclosureRow> {
    LaneDisclosureDimension::ALL
        .iter()
        .map(|&dimension| LaneDisclosureRow {
            dimension,
            score: 96,
            threshold: 90,
            status: DisclosureStatus::Pass,
        })
        .collect()
}

/// A disclosure scorecard with one borderline (warn) dimension, valid for Beta.
fn beta_scorecard() -> Vec<LaneDisclosureRow> {
    LaneDisclosureDimension::ALL
        .iter()
        .map(|&dimension| {
            if dimension == LaneDisclosureDimension::CostBudgetOwner {
                LaneDisclosureRow {
                    dimension,
                    score: 90,
                    threshold: 90,
                    status: DisclosureStatus::Warn,
                }
            } else {
                LaneDisclosureRow {
                    dimension,
                    score: 93,
                    threshold: 88,
                    status: DisclosureStatus::Pass,
                }
            }
        })
        .collect()
}

/// A claimed coverage row that discloses every invariant axis.
fn claimed_row(
    profile: M5LaneProfile,
    qualification: M5AiWorkflowQualificationClass,
    side_effect: ToolSideEffectClass,
    authority: AutomationAuthorityClass,
) -> ProfileCoverageRow {
    ProfileCoverageRow {
        profile,
        claimed_on_profile: true,
        profile_qualification: qualification,
        execution_mode: profile.expected_mode(),
        provider_disclosed: true,
        model_route_disclosed: true,
        region_disclosed: true,
        retention_disclosed: true,
        cost_owner_disclosed: true,
        side_effect_class: side_effect,
        automation_authority: authority,
        managed_claim_within_qualified_family: true,
    }
}

/// An unclaimed coverage row for a profile this lane does not ship on.
fn unclaimed_row(profile: M5LaneProfile) -> ProfileCoverageRow {
    ProfileCoverageRow {
        profile,
        claimed_on_profile: false,
        profile_qualification: M5AiWorkflowQualificationClass::Unavailable,
        execution_mode: profile.expected_mode(),
        provider_disclosed: false,
        model_route_disclosed: false,
        region_disclosed: false,
        retention_disclosed: false,
        cost_owner_disclosed: false,
        side_effect_class: ToolSideEffectClass::InspectOnly,
        automation_authority: AutomationAuthorityClass::InspectOnlyNoAuthority,
        managed_claim_within_qualified_family: false,
    }
}

/// Coverage that claims every profile at the lane's headline qualification.
fn full_coverage(
    qualification: M5AiWorkflowQualificationClass,
    side_effect: ToolSideEffectClass,
    authority: AutomationAuthorityClass,
) -> Vec<ProfileCoverageRow> {
    M5LaneProfile::ALL
        .iter()
        .map(|&profile| claimed_row(profile, qualification, side_effect, authority))
        .collect()
}

fn stable_downgrade_rules() -> Vec<LaneDowngradeRule> {
    vec![
        LaneDowngradeRule {
            trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
            narrowed_to: M5AiWorkflowQualificationClass::Beta,
            auto_enforced: true,
            rationale: "Stale proof narrows the public Stable claim to Beta".to_owned(),
        },
        LaneDowngradeRule {
            trigger: M5AiWorkflowDowngradeTrigger::PolicyBlocked,
            narrowed_to: M5AiWorkflowQualificationClass::Unavailable,
            auto_enforced: true,
            rationale: "A policy block makes the lane unavailable".to_owned(),
        },
    ]
}

fn beta_downgrade_rules() -> Vec<LaneDowngradeRule> {
    vec![
        LaneDowngradeRule {
            trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
            narrowed_to: M5AiWorkflowQualificationClass::Preview,
            auto_enforced: true,
            rationale: "Stale proof narrows the Beta claim to Preview".to_owned(),
        },
        LaneDowngradeRule {
            trigger: M5AiWorkflowDowngradeTrigger::UpstreamDependencyNarrowed,
            narrowed_to: M5AiWorkflowQualificationClass::Experimental,
            auto_enforced: true,
            rationale: "An upstream narrowing drops the lane to Experimental".to_owned(),
        },
    ]
}

fn evidence(refs: &[&str]) -> Vec<String> {
    refs.iter().map(|r| (*r).to_owned()).collect()
}

fn lane_certifications() -> Vec<LaneCertification> {
    vec![
        LaneCertification {
            lane: CertifiedLane::LocalModelPack,
            claimed_qualification: M5AiWorkflowQualificationClass::Stable,
            scope_summary: "Local model pack install, provenance, hardware fit, and offline/mirror channels".to_owned(),
            profile_coverage: vec![
                claimed_row(
                    M5LaneProfile::LocalOnly,
                    M5AiWorkflowQualificationClass::Stable,
                    ToolSideEffectClass::LocalReversibleEdit,
                    AutomationAuthorityClass::LocalReversibleOnly,
                ),
                claimed_row(
                    M5LaneProfile::OfflineMirror,
                    M5AiWorkflowQualificationClass::Stable,
                    ToolSideEffectClass::LocalReversibleEdit,
                    AutomationAuthorityClass::LocalReversibleOnly,
                ),
                unclaimed_row(M5LaneProfile::ByokDirect),
                unclaimed_row(M5LaneProfile::ManagedHosted),
                unclaimed_row(M5LaneProfile::HybridManaged),
            ],
            disclosure_scorecard: passing_scorecard(),
            downgrade_rules: stable_downgrade_rules(),
            evidence_packet_refs: evidence(&["evidence:local-model-pack-install:m5"]),
        },
        LaneCertification {
            lane: CertifiedLane::ProviderRouting,
            claimed_qualification: M5AiWorkflowQualificationClass::Stable,
            scope_summary: "Provider/model registry, route disclosure, graduation rings, and routing policy".to_owned(),
            profile_coverage: full_coverage(
                M5AiWorkflowQualificationClass::Stable,
                ToolSideEffectClass::InspectOnly,
                AutomationAuthorityClass::InspectOnlyNoAuthority,
            ),
            disclosure_scorecard: passing_scorecard(),
            downgrade_rules: stable_downgrade_rules(),
            evidence_packet_refs: evidence(&[
                "evidence:provider-route-disclosure:m5",
                "evidence:provider-model-graduation:m5",
            ]),
        },
        LaneCertification {
            lane: CertifiedLane::RecipeAutomation,
            claimed_qualification: M5AiWorkflowQualificationClass::Beta,
            scope_summary: "Signed/shared recipe packs and recorded-macro promotion under preview-first replay".to_owned(),
            profile_coverage: vec![
                claimed_row(
                    M5LaneProfile::LocalOnly,
                    M5AiWorkflowQualificationClass::Beta,
                    ToolSideEffectClass::LocalDestructiveEdit,
                    AutomationAuthorityClass::LocalWithApproval,
                ),
                claimed_row(
                    M5LaneProfile::ByokDirect,
                    M5AiWorkflowQualificationClass::Beta,
                    ToolSideEffectClass::ExternalReversibleComment,
                    AutomationAuthorityClass::ExternalReversibleWithApproval,
                ),
                claimed_row(
                    M5LaneProfile::ManagedHosted,
                    M5AiWorkflowQualificationClass::Beta,
                    ToolSideEffectClass::ExternalIrreversiblePublish,
                    AutomationAuthorityClass::ManagedOnlyTemplateAuthority,
                ),
                claimed_row(
                    M5LaneProfile::OfflineMirror,
                    M5AiWorkflowQualificationClass::Preview,
                    ToolSideEffectClass::LocalReversibleEdit,
                    AutomationAuthorityClass::LocalReversibleOnly,
                ),
                claimed_row(
                    M5LaneProfile::HybridManaged,
                    M5AiWorkflowQualificationClass::Beta,
                    ToolSideEffectClass::ExternalIrreversiblePublish,
                    AutomationAuthorityClass::ExternalIrreversibleAdminGated,
                ),
            ],
            disclosure_scorecard: beta_scorecard(),
            downgrade_rules: beta_downgrade_rules(),
            evidence_packet_refs: evidence(&[
                "evidence:recipe-pack-graduation:m5",
                "evidence:user-automation:m5",
            ]),
        },
        LaneCertification {
            lane: CertifiedLane::ExternalConnector,
            claimed_qualification: M5AiWorkflowQualificationClass::Beta,
            scope_summary: "External-tool gateway connectors with capability classes and side-effect disclosure".to_owned(),
            profile_coverage: vec![
                claimed_row(
                    M5LaneProfile::LocalOnly,
                    M5AiWorkflowQualificationClass::Beta,
                    ToolSideEffectClass::InspectOnly,
                    AutomationAuthorityClass::InspectOnlyNoAuthority,
                ),
                claimed_row(
                    M5LaneProfile::ByokDirect,
                    M5AiWorkflowQualificationClass::Beta,
                    ToolSideEffectClass::ExternalIrreversiblePublish,
                    AutomationAuthorityClass::ExternalIrreversibleAdminGated,
                ),
                claimed_row(
                    M5LaneProfile::ManagedHosted,
                    M5AiWorkflowQualificationClass::Beta,
                    ToolSideEffectClass::ExternalReversibleComment,
                    AutomationAuthorityClass::ExternalReversibleWithApproval,
                ),
                unclaimed_row(M5LaneProfile::OfflineMirror),
                claimed_row(
                    M5LaneProfile::HybridManaged,
                    M5AiWorkflowQualificationClass::Preview,
                    ToolSideEffectClass::ExternalReversibleComment,
                    AutomationAuthorityClass::ExternalReversibleWithApproval,
                ),
            ],
            disclosure_scorecard: beta_scorecard(),
            downgrade_rules: beta_downgrade_rules(),
            evidence_packet_refs: evidence(&["evidence:connector-manifest:m5"]),
        },
        LaneCertification {
            lane: CertifiedLane::SpendGovernance,
            claimed_qualification: M5AiWorkflowQualificationClass::Stable,
            scope_summary: "Spend ledgers, quota ceilings, budget attribution, and offline-safe usage reporting".to_owned(),
            profile_coverage: full_coverage(
                M5AiWorkflowQualificationClass::Stable,
                ToolSideEffectClass::InspectOnly,
                AutomationAuthorityClass::InspectOnlyNoAuthority,
            ),
            disclosure_scorecard: passing_scorecard(),
            downgrade_rules: stable_downgrade_rules(),
            evidence_packet_refs: evidence(&[
                "evidence:agent-budget:m5",
                "evidence:usage-reporting:m5",
            ]),
        },
    ]
}

fn proof_freshness() -> M5LaneCertificationProofFreshness {
    M5LaneCertificationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_LANE_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_LANE_CERTIFICATION_DOC_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_SCHEMA_REF.to_owned(),
        M5_AI_WORKFLOW_MATRIX_ARTIFACT_REF.to_owned(),
    ];
    for lane in CertifiedLane::ALL {
        for schema_ref in lane.canonical_schema_refs() {
            refs.push((*schema_ref).to_owned());
        }
    }
    refs
}

fn packet() -> M5LaneCertificationPacket {
    M5LaneCertificationPacket::new(M5LaneCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        certification_label: "M5 Lane Certification".to_owned(),
        lane_certifications: lane_certifications(),
        proof_freshness: proof_freshness(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-07T00:00:00Z".to_owned(),
    })
}

#[test]
fn m5_lane_certification_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn all_governed_lanes_are_certified() {
    let packet = packet();
    let lanes: BTreeSet<CertifiedLane> = packet
        .lane_certifications
        .iter()
        .map(|cert| cert.lane)
        .collect();
    for lane in CertifiedLane::ALL {
        assert!(lanes.contains(&lane), "missing lane: {}", lane.as_str());
    }
}

#[test]
fn every_lane_covers_every_profile() {
    for cert in packet().lane_certifications {
        let profiles: BTreeSet<M5LaneProfile> = cert
            .profile_coverage
            .iter()
            .map(|row| row.profile)
            .collect();
        for profile in M5LaneProfile::ALL {
            assert!(
                profiles.contains(&profile),
                "lane {} missing profile {}",
                cert.lane.as_str(),
                profile.as_str()
            );
        }
    }
}

#[test]
fn missing_lane_fails_validation() {
    let mut packet = packet();
    packet
        .lane_certifications
        .retain(|cert| cert.lane != CertifiedLane::SpendGovernance);
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::RequiredLaneMissing));
}

#[test]
fn duplicate_lane_fails_validation() {
    let mut packet = packet();
    let first = packet.lane_certifications[0].clone();
    packet.lane_certifications.push(first);
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::DuplicateLane));
}

#[test]
fn missing_profile_fails_validation() {
    let mut packet = packet();
    packet.lane_certifications[0].profile_coverage.pop();
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::RequiredProfileMissing));
}

#[test]
fn duplicate_profile_fails_validation() {
    let mut packet = packet();
    let row = packet.lane_certifications[0].profile_coverage[0].clone();
    packet.lane_certifications[0].profile_coverage.push(row);
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::DuplicateProfile));
}

#[test]
fn disclosure_dimension_missing_fails() {
    let mut packet = packet();
    packet.lane_certifications[0].disclosure_scorecard.pop();
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::DisclosureDimensionMissing));
}

#[test]
fn disclosure_status_inconsistent_fails() {
    let mut packet = packet();
    // Claim a pass while the score is below threshold.
    packet.lane_certifications[0].disclosure_scorecard[0].score = 10;
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::DisclosureStatusInconsistent));
}

#[test]
fn stable_lane_with_warn_dimension_fails() {
    let mut packet = packet();
    packet.lane_certifications[0].disclosure_scorecard[0].status = DisclosureStatus::Warn;
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::StableLaneDisclosureNotPassing));
}

#[test]
fn beta_lane_with_failing_dimension_fails() {
    let mut packet = packet();
    // RecipeAutomation lane is index 2 (Beta).
    let row = &mut packet.lane_certifications[2].disclosure_scorecard[0];
    row.score = 10;
    row.threshold = 90;
    row.status = DisclosureStatus::Fail;
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::BetaLaneDisclosureFailing));
}

#[test]
fn claimed_profile_missing_disclosure_fails() {
    let mut packet = packet();
    // Hide cost on a claimed local-only row.
    packet.lane_certifications[0].profile_coverage[0].cost_owner_disclosed = false;
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::ClaimedProfileMissingDisclosure));
}

#[test]
fn locality_profile_mismatch_fails() {
    let mut packet = packet();
    // A managed-hosted row must resolve to the managed execution mode.
    let row = packet.lane_certifications[1]
        .profile_coverage
        .iter_mut()
        .find(|row| row.profile == M5LaneProfile::ManagedHosted)
        .expect("managed-hosted coverage row");
    row.execution_mode = RoutePolicyModeClass::Local;
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::LocalityProfileMismatch));
}

#[test]
fn managed_claim_widened_fails() {
    let mut packet = packet();
    let row = packet.lane_certifications[1]
        .profile_coverage
        .iter_mut()
        .find(|row| row.profile == M5LaneProfile::ManagedHosted)
        .expect("managed-hosted coverage row");
    row.managed_claim_within_qualified_family = false;
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::ManagedClaimWidened));
}

#[test]
fn automation_authority_insufficient_fails() {
    let mut packet = packet();
    // Recipe automation on managed-hosted does an irreversible publish; drop its
    // authority below the admin-gated floor.
    let row = packet.lane_certifications[2]
        .profile_coverage
        .iter_mut()
        .find(|row| row.profile == M5LaneProfile::ManagedHosted)
        .expect("managed-hosted coverage row");
    row.automation_authority = AutomationAuthorityClass::LocalReversibleOnly;
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::AutomationAuthorityInsufficient));
}

#[test]
fn profile_claim_flag_inconsistent_fails() {
    let mut packet = packet();
    // Flip an unclaimed row's flag without giving it a claimed qualification.
    let row = packet.lane_certifications[0]
        .profile_coverage
        .iter_mut()
        .find(|row| row.profile == M5LaneProfile::ByokDirect)
        .expect("byok-direct coverage row");
    row.claimed_on_profile = true;
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::ProfileClaimFlagInconsistent));
}

#[test]
fn profile_qualification_exceeds_claim_fails() {
    let mut packet = packet();
    // RecipeAutomation headline is Beta; claim Stable on a profile.
    let row = &mut packet.lane_certifications[2].profile_coverage[0];
    row.profile_qualification = M5AiWorkflowQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::ProfileQualificationExceedsClaim));
}

#[test]
fn claimed_lane_missing_evidence_fails() {
    let mut packet = packet();
    packet.lane_certifications[0].evidence_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::ClaimedLaneMissingEvidence));
}

#[test]
fn downgrade_rules_missing_fails() {
    let mut packet = packet();
    packet.lane_certifications[0].downgrade_rules.clear();
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::DowngradeRulesMissing));
}

#[test]
fn downgrade_rule_missing_proof_stale_fails() {
    let mut packet = packet();
    packet.lane_certifications[0]
        .downgrade_rules
        .retain(|rule| rule.trigger != M5AiWorkflowDowngradeTrigger::ProofStale);
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::DowngradeRuleMissingProofStale));
}

#[test]
fn downgrade_rule_not_narrowing_fails() {
    let mut packet = packet();
    // Narrowing "to" Stable from a Stable claim does not narrow.
    packet.lane_certifications[0].downgrade_rules[0].narrowed_to =
        M5AiWorkflowQualificationClass::Stable;
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::DowngradeRuleNotNarrowing));
}

#[test]
fn missing_base_source_contracts_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|r| r != M5_AI_WORKFLOW_MATRIX_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::MissingSourceContracts));
}

#[test]
fn lane_canonical_schema_unbound_fails() {
    let mut packet = packet();
    let local_ref = CertifiedLane::LocalModelPack.primary_schema_ref();
    packet.source_contract_refs.retain(|r| r != local_ref);
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::LaneCanonicalSchemaUnbound));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5LaneCertificationViolation::ProofFreshnessIncomplete));
}

#[test]
fn narrowed_qualification_applies_matching_rule() {
    let cert = &packet().lane_certifications[0];
    assert_eq!(
        cert.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Beta
    );
    // A trigger with no rule leaves the claim unchanged.
    assert_eq!(
        cert.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProviderUnavailable),
        M5AiWorkflowQualificationClass::Stable
    );
}

#[test]
fn stable_lane_count_matches_fixture() {
    assert_eq!(packet().stable_lane_count(), 3);
}

#[test]
fn claimed_profile_count_matches_fixture() {
    // LocalModelPack 2 + ProviderRouting 5 + RecipeAutomation 5 +
    // ExternalConnector 4 + SpendGovernance 5 = 21 claimed pairs.
    assert_eq!(packet().claimed_profile_count(), 21);
}

#[test]
fn markdown_summary_lists_every_lane_and_profile() {
    let summary = packet().render_markdown_summary();
    for lane in CertifiedLane::ALL {
        assert!(summary.contains(lane.as_str()), "missing {}", lane.as_str());
    }
    for profile in M5LaneProfile::ALL {
        assert!(
            summary.contains(profile.as_str()),
            "missing {}",
            profile.as_str()
        );
    }
}

#[test]
fn round_trips_through_json() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5LaneCertificationPacket =
        serde_json::from_str(&json).expect("packet round-trips through JSON");
    assert_eq!(parsed, packet);
}

#[test]
fn checked_support_export_validates() {
    let packet = current_m5_lane_certification_export()
        .expect("checked m5 lane certification export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(packet.lane_certifications.len(), CertifiedLane::ALL.len());
}

#[test]
fn checked_held_fixture_validates_and_narrows() {
    let packet: M5LaneCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ai/m5/certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile/held_connector_lane_certification.json"
    )))
    .expect("held fixture parses");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let connector = packet
        .lane_certifications
        .iter()
        .find(|cert| cert.lane == CertifiedLane::ExternalConnector)
        .expect("held fixture carries the external-connector lane");
    // A held lane is not a claimed lane and narrows to unavailable on stale proof.
    assert!(!connector.is_claimed());
    assert!(connector.evidence_packet_refs.is_empty());
    assert_eq!(
        connector.narrowed_qualification(M5AiWorkflowDowngradeTrigger::ProofStale),
        M5AiWorkflowQualificationClass::Unavailable
    );
    assert!(connector
        .profile_coverage
        .iter()
        .all(|row| !row.claimed_on_profile));
}
