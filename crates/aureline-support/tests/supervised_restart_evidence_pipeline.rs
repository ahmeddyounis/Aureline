//! Protected tests for the supervised-restart evidence pipeline.

use std::path::{Path, PathBuf};

use aureline_support::{
    seeded_fault_domain_view_packet, seeded_supervised_restart_evidence_packet, RestartDomainClass,
    SupervisedRestartEvidencePacket, SUPERVISED_RESTART_EVIDENCE_PACKET_RECORD_KIND,
    SUPERVISED_RESTART_EVIDENCE_PIPELINE_DOC_REF, SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF,
    SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn seeded_packet_covers_all_five_fault_domains() {
    let packet = seeded_supervised_restart_evidence_packet();
    assert_eq!(
        packet.record_kind,
        SUPERVISED_RESTART_EVIDENCE_PACKET_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_VERSION
    );
    assert!(!packet.build_id.is_empty());
    assert_eq!(packet.domain_summaries.len(), 5);

    let domain_tokens: Vec<String> = packet
        .domain_summaries
        .iter()
        .map(|s| s.domain_token.clone())
        .collect();
    for expected in ["local", "remote", "extension", "debug", "notebook"] {
        assert!(
            domain_tokens.contains(&expected.to_owned()),
            "missing domain {expected}"
        );
    }
}

#[test]
fn packet_derives_from_fault_domain_view_packet() {
    let fault_packet = seeded_fault_domain_view_packet();
    let packet = SupervisedRestartEvidencePacket::from_fault_domain_view_packet(
        "supervised-restart:derived",
        "2026-06-02T23:25:40Z",
        "aureline-build:test:m4",
        &fault_packet,
    );

    assert_eq!(packet.workspace_id, fault_packet.workspace_id);
    assert_eq!(packet.host_lane_count as usize, fault_packet.rows.len());
    assert!(packet.is_export_safe());
    assert!(packet.validate().is_empty());
}

#[test]
fn non_mutating_lanes_may_rehydrate_safely_while_mutating_lanes_require_review() {
    let packet = seeded_supervised_restart_evidence_packet();

    for lane in &packet.host_lane_identities {
        let policy = packet
            .no_rerun_policies
            .iter()
            .find(|p| p.host_lane_ref == lane.host_lane_ref)
            .expect("every lane must have a no-rerun policy");

        if lane.can_mutate || lane.externally_routed {
            assert!(
                policy.forbids_silent_rerun,
                "mutating or externally routed lane {} must forbid silent rerun (policy={})",
                lane.host_lane_ref, policy.policy_token
            );
        }
    }
}

#[test]
fn reattached_sessions_preserve_local_continuity_and_label_lost_authority() {
    let packet = seeded_supervised_restart_evidence_packet();

    for decision in &packet.review_decisions {
        if decision.explicit_review_required {
            assert!(
                !decision.current_lane_accepted,
                "{}: lanes requiring explicit review cannot claim current status",
                decision.decision_id
            );
        }

        if !decision.lost_state_refs.is_empty() {
            assert!(
                !decision.current_lane_accepted,
                "{}: lanes with lost state must not claim current status",
                decision.decision_id
            );
        }
    }
}

#[test]
fn support_bundle_and_shiproom_packet_includes_restart_lineage_and_host_lane_identity() {
    let packet = seeded_supervised_restart_evidence_packet();

    assert!(!packet.lineage_entries.is_empty());
    assert!(!packet.host_lane_identities.is_empty());
    assert!(!packet.review_decisions.is_empty());
    assert!(!packet.no_rerun_policies.is_empty());
    assert!(!packet.domain_summaries.is_empty());

    // Every lineage entry carries exact-build correlation.
    for entry in &packet.lineage_entries {
        assert!(
            !entry.build_id.is_empty(),
            "{}: lineage entry must carry build_id",
            entry.entry_id
        );
    }

    // The packet quotes its own doc and schema refs.
    assert_eq!(packet.doc_ref, SUPERVISED_RESTART_EVIDENCE_PIPELINE_DOC_REF);
    assert_eq!(
        packet.schema_ref,
        SUPERVISED_RESTART_EVIDENCE_PIPELINE_SCHEMA_REF
    );
}

#[test]
fn stable_rows_show_restart_quarantine_and_reattach_posture_consistently() {
    let packet = seeded_supervised_restart_evidence_packet();

    let plaintext = packet.render_plaintext();
    assert!(plaintext.contains("Supervised-restart evidence packet"));
    assert!(plaintext.contains(&packet.build_id));

    // All five domains appear in plaintext.
    for domain in ["local", "remote", "extension", "debug", "notebook"] {
        assert!(
            plaintext.contains(domain),
            "plaintext must mention domain '{domain}'"
        );
    }

    // No raw paths or secrets leak.
    assert!(!plaintext.contains("/Users/"));
    assert!(!plaintext.contains("BEARER"));
    assert!(!plaintext.contains("SSH_PRIVATE_KEY"));
}

#[test]
fn validation_rejects_missing_build_id() {
    let mut packet = seeded_supervised_restart_evidence_packet();
    packet.build_id.clear();
    let violations = packet.validate();
    assert!(
        violations.iter().any(|v| v.path == "build_id"),
        "missing build_id must be rejected"
    );
}

#[test]
fn validation_rejects_mutating_lane_without_no_rerun_policy() {
    let mut packet = seeded_supervised_restart_evidence_packet();
    // Remove the no-rerun policy for a mutating lane (e.g., notebook kernel).
    packet.no_rerun_policies.retain(|p| {
        !packet
            .host_lane_identities
            .iter()
            .any(|lane| lane.host_lane_ref == p.host_lane_ref && lane.can_mutate)
    });
    let violations = packet.validate();
    assert!(
        violations.iter().any(|v| v.path == "no_rerun_policies"),
        "mutating lane without no-rerun policy must be rejected: {:?}",
        violations
    );
}

#[test]
fn validation_rejects_review_required_decision_that_claims_current() {
    let mut packet = seeded_supervised_restart_evidence_packet();
    for decision in &mut packet.review_decisions {
        if decision.explicit_review_required {
            decision.current_lane_accepted = true;
            break;
        }
    }
    let violations = packet.validate();
    assert!(
        violations.iter().any(|v| v.path == "review_decisions"),
        "review-required decision claiming current must be rejected: {:?}",
        violations
    );
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_supervised_restart_evidence_packet();
    let json = serde_json::to_string(&packet).expect("serialize");
    let round: SupervisedRestartEvidencePacket = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(round, packet);
}

#[test]
fn checked_in_support_docs_schemas_and_artifacts_exist() {
    let root = repo_root();
    for rel in [
        "schemas/support/supervised-restart-evidence-pipeline.schema.json",
        "artifacts/support/m4/supervised-restart-evidence-pipeline.md",
        "docs/help/support/supervised-restart-evidence-pipeline.md",
        "artifacts/support/fault_domain_packets/host_lane_fault_domain_packet.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn domain_summary_counts_are_consistent_with_lane_state() {
    let packet = seeded_supervised_restart_evidence_packet();

    for summary in &packet.domain_summaries {
        let lanes_in_domain: Vec<_> = packet
            .host_lane_identities
            .iter()
            .filter(|lane| {
                packet.lineage_entries.iter().any(|entry| {
                    entry.host_lane_ref == lane.host_lane_ref
                        && entry.domain_token == summary.domain_token
                })
            })
            .collect();

        assert_eq!(
            summary.lane_count as usize,
            lanes_in_domain.len(),
            "{}: lane count must match",
            summary.domain_token
        );
    }
}

#[test]
fn mutating_and_externally_routed_domains_are_correctly_classified() {
    for domain in RestartDomainClass::ALL {
        match domain {
            RestartDomainClass::Local | RestartDomainClass::Extension => {
                assert!(!domain.can_mutate());
                assert!(!domain.is_externally_routed());
            }
            RestartDomainClass::Debug | RestartDomainClass::Notebook => {
                assert!(domain.can_mutate());
                assert!(!domain.is_externally_routed());
            }
            RestartDomainClass::Remote => {
                assert!(domain.can_mutate());
                assert!(domain.is_externally_routed());
            }
        }
    }
}
