//! Conformance dump for the M5 lane certification packet.
//!
//! Prints the canonical support export (default) or the narrowed fixture
//! (`fixture` argument) so the checked-in artifact and fixtures stay byte-aligned
//! with the in-crate builder.

use aureline_ai::certify_local_model_provider_recipe_connector_and_spend_governance_lanes_on_every_claimed_m5_profile::*;
use aureline_ai::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents::{
    M5AiWorkflowDowngradeTrigger, M5AiWorkflowQualificationClass,
    M5_AI_WORKFLOW_MATRIX_ARTIFACT_REF, M5_AI_WORKFLOW_MATRIX_SCHEMA_REF,
};
use aureline_ai::implement_signed_and_shared_recipe_packs_safe_automation_graduation_and_preview_first_replay::AutomationAuthorityClass;
use aureline_ai::tool_gateway::ToolSideEffectClass;

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

fn held_scorecard() -> Vec<LaneDisclosureRow> {
    LaneDisclosureDimension::ALL
        .iter()
        .map(|&dimension| LaneDisclosureRow {
            dimension,
            score: 70,
            threshold: 90,
            status: DisclosureStatus::Fail,
        })
        .collect()
}

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

fn held_coverage() -> Vec<ProfileCoverageRow> {
    M5LaneProfile::ALL
        .iter()
        .map(|&p| unclaimed_row(p))
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

fn held_downgrade_rules() -> Vec<LaneDowngradeRule> {
    vec![LaneDowngradeRule {
        trigger: M5AiWorkflowDowngradeTrigger::ProofStale,
        narrowed_to: M5AiWorkflowQualificationClass::Unavailable,
        auto_enforced: true,
        rationale: "A held lane goes unavailable on stale proof rather than claiming".to_owned(),
    }]
}

fn evidence(refs: &[&str]) -> Vec<String> {
    refs.iter().map(|r| (*r).to_owned()).collect()
}

fn local_model_pack_lane() -> LaneCertification {
    LaneCertification {
        lane: CertifiedLane::LocalModelPack,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        scope_summary:
            "Local model pack install, provenance, hardware fit, and offline/mirror channels"
                .to_owned(),
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
    }
}

fn provider_routing_lane() -> LaneCertification {
    LaneCertification {
        lane: CertifiedLane::ProviderRouting,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        scope_summary:
            "Provider/model registry, route disclosure, graduation rings, and routing policy"
                .to_owned(),
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
    }
}

fn recipe_automation_lane() -> LaneCertification {
    LaneCertification {
        lane: CertifiedLane::RecipeAutomation,
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        scope_summary:
            "Signed/shared recipe packs and recorded-macro promotion under preview-first replay"
                .to_owned(),
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
    }
}

fn external_connector_lane() -> LaneCertification {
    LaneCertification {
        lane: CertifiedLane::ExternalConnector,
        claimed_qualification: M5AiWorkflowQualificationClass::Beta,
        scope_summary:
            "External-tool gateway connectors with capability classes and side-effect disclosure"
                .to_owned(),
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
    }
}

fn spend_governance_lane() -> LaneCertification {
    LaneCertification {
        lane: CertifiedLane::SpendGovernance,
        claimed_qualification: M5AiWorkflowQualificationClass::Stable,
        scope_summary:
            "Spend ledgers, quota ceilings, budget attribution, and offline-safe usage reporting"
                .to_owned(),
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
    }
}

/// External connectors held pending upstream provider graduation; the lane is
/// not a claimed lane (no profile claim, no evidence) and narrows to unavailable
/// on stale proof.
fn held_external_connector_lane() -> LaneCertification {
    LaneCertification {
        lane: CertifiedLane::ExternalConnector,
        claimed_qualification: M5AiWorkflowQualificationClass::Held,
        scope_summary:
            "External-tool connectors held pending upstream provider graduation; no public claim"
                .to_owned(),
        profile_coverage: held_coverage(),
        disclosure_scorecard: held_scorecard(),
        downgrade_rules: held_downgrade_rules(),
        evidence_packet_refs: Vec::new(),
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

fn proof_freshness() -> M5LaneCertificationProofFreshness {
    M5LaneCertificationProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let render_summary = which == "summary";
    let packet = match which.as_str() {
        "fixture" => M5LaneCertificationPacket::new(M5LaneCertificationPacketInput {
            packet_id: "m5-lane-certification:fixture:held:0001".to_owned(),
            certification_label: "Held Connector Lane Narrows Its Claim".to_owned(),
            lane_certifications: vec![
                local_model_pack_lane(),
                provider_routing_lane(),
                recipe_automation_lane(),
                held_external_connector_lane(),
                spend_governance_lane(),
            ],
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: "metadata_safe_default".to_owned(),
            minted_at: "2026-06-07T00:00:00Z".to_owned(),
        }),
        _ => M5LaneCertificationPacket::new(M5LaneCertificationPacketInput {
            packet_id: "m5-lane-certification:stable:0001".to_owned(),
            certification_label: "M5 Lane Certification".to_owned(),
            lane_certifications: vec![
                local_model_pack_lane(),
                provider_routing_lane(),
                recipe_automation_lane(),
                external_connector_lane(),
                spend_governance_lane(),
            ],
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: "metadata_safe_default".to_owned(),
            minted_at: "2026-06-07T00:00:00Z".to_owned(),
        }),
    };

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );
    if render_summary {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
