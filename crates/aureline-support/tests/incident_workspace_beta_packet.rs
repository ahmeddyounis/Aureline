//! Protected drill for the M3 beta incident-workspace handoff packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::incident_workspace_beta::{
    current_incident_workspace_beta_packet_corpus, load_incident_workspace_beta_packet,
    validate_packet_record, ClaimDowngradeToken, EvidenceCustodyClass, HandoffConsumerClass,
    RecoveryOptionClass, INCIDENT_WORKSPACE_ALPHA_DOC_REF,
    INCIDENT_WORKSPACE_BETA_ARTIFACT_REF, INCIDENT_WORKSPACE_BETA_FIXTURE_DIR,
    INCIDENT_WORKSPACE_BETA_FIXTURE_MANIFEST_REF, INCIDENT_WORKSPACE_BETA_PACKET_DOC_REF,
    INCIDENT_WORKSPACE_BETA_PACKET_RECORD_KIND, INCIDENT_WORKSPACE_BETA_PACKET_SCHEMA_REF,
    INCIDENT_WORKSPACE_BETA_SCENARIO_CORPUS_DOC_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn corpus_validates_against_the_beta_packet_contract() {
    let corpus = current_incident_workspace_beta_packet_corpus().expect("corpus parses");
    assert_eq!(corpus.entries.len(), 3);
    let violations = corpus.validate();
    assert_eq!(violations, Vec::new(), "{violations:#?}");
    for packet in corpus.packets() {
        assert_eq!(
            packet.record_kind,
            INCIDENT_WORKSPACE_BETA_PACKET_RECORD_KIND
        );
        assert!(packet.workspace_identity.preserves_user_authored_files);
        assert!(packet.privacy_baseline.raw_private_material_excluded);
        assert!(packet.privacy_baseline.ambient_authority_excluded);
        assert_eq!(
            packet.references.doc_ref,
            INCIDENT_WORKSPACE_BETA_PACKET_DOC_REF
        );
        assert_eq!(
            packet.references.schema_ref,
            INCIDENT_WORKSPACE_BETA_PACKET_SCHEMA_REF
        );
        assert_eq!(
            packet.references.scenario_corpus_doc_ref,
            INCIDENT_WORKSPACE_BETA_SCENARIO_CORPUS_DOC_REF
        );
    }
}

#[test]
fn corpus_covers_local_only_managed_copy_and_held_record_custody_lanes() {
    let corpus = current_incident_workspace_beta_packet_corpus().expect("corpus parses");
    let mut classes: BTreeSet<EvidenceCustodyClass> = BTreeSet::new();
    for packet in corpus.packets() {
        for artifact in &packet.evidence_artifacts {
            classes.insert(artifact.custody_class);
        }
    }
    assert!(classes.contains(&EvidenceCustodyClass::LocalOnlyArtifact));
    assert!(classes.contains(&EvidenceCustodyClass::ManagedCopyAvailable));
    assert!(
        classes.contains(&EvidenceCustodyClass::HeldRecordUnderSecurityHold)
            || classes.contains(&EvidenceCustodyClass::HeldRecordUnderLegalHold)
    );
}

#[test]
fn joint_lane_routes_security_triage_and_keeps_security_recovery_option() {
    let corpus = current_incident_workspace_beta_packet_corpus().expect("corpus parses");
    let joint = corpus
        .packets()
        .find(|packet| {
            packet
                .handoff_consumer_classes
                .contains(&HandoffConsumerClass::SupportIntakeAndSecurityTriage)
        })
        .expect("joint lane fixture present");
    assert!(joint
        .recovery_options
        .iter()
        .any(|opt| opt.recovery_option_class == RecoveryOptionClass::OpenSecurityPrivateTriage));
    assert!(joint
        .claim_state
        .downgrade_tokens
        .contains(&ClaimDowngradeToken::HeldRecordBlocksExport));
}

#[test]
fn managed_copy_lane_carries_managed_copy_pending_admin_review_token() {
    let corpus = current_incident_workspace_beta_packet_corpus().expect("corpus parses");
    let managed = corpus
        .packets()
        .find(|packet| {
            packet
                .evidence_artifacts
                .iter()
                .any(|a| a.custody_class == EvidenceCustodyClass::ManagedCopyAvailable)
        })
        .expect("managed copy fixture present");
    assert!(managed
        .claim_state
        .downgrade_tokens
        .contains(&ClaimDowngradeToken::ManagedCopyPendingAdminReview));
}

#[test]
fn yaml_round_trip_load_matches_corpus_entry() {
    let corpus = current_incident_workspace_beta_packet_corpus().expect("corpus parses");
    let root = repo_root();
    for entry in &corpus.entries {
        let yaml = std::fs::read_to_string(root.join(&entry.fixture_ref))
            .unwrap_or_else(|err| panic!("read {}: {err}", entry.fixture_ref));
        let packet = load_incident_workspace_beta_packet(&yaml)
            .unwrap_or_else(|err| panic!("parse {}: {err}", entry.fixture_ref));
        assert_eq!(packet, entry.packet);
    }
}

#[test]
fn corpus_doc_artifact_and_schema_exist_at_declared_paths() {
    let root = repo_root();
    assert!(
        root.join(INCIDENT_WORKSPACE_BETA_FIXTURE_DIR).is_dir(),
        "fixture dir {INCIDENT_WORKSPACE_BETA_FIXTURE_DIR} missing"
    );
    assert!(
        root.join(INCIDENT_WORKSPACE_BETA_FIXTURE_MANIFEST_REF)
            .is_file(),
        "manifest {INCIDENT_WORKSPACE_BETA_FIXTURE_MANIFEST_REF} missing"
    );
    assert!(
        root.join(INCIDENT_WORKSPACE_BETA_PACKET_DOC_REF).is_file(),
        "doc {INCIDENT_WORKSPACE_BETA_PACKET_DOC_REF} missing"
    );
    assert!(
        root.join(INCIDENT_WORKSPACE_BETA_PACKET_SCHEMA_REF).is_file(),
        "schema {INCIDENT_WORKSPACE_BETA_PACKET_SCHEMA_REF} missing"
    );
    assert!(
        root.join(INCIDENT_WORKSPACE_BETA_SCENARIO_CORPUS_DOC_REF)
            .is_file(),
        "scenario corpus doc {INCIDENT_WORKSPACE_BETA_SCENARIO_CORPUS_DOC_REF} missing"
    );
    assert!(
        root.join(INCIDENT_WORKSPACE_BETA_ARTIFACT_REF).is_file(),
        "artifact {INCIDENT_WORKSPACE_BETA_ARTIFACT_REF} missing"
    );
    assert!(
        root.join(INCIDENT_WORKSPACE_ALPHA_DOC_REF).is_file(),
        "alpha contract {INCIDENT_WORKSPACE_ALPHA_DOC_REF} missing"
    );
}

#[test]
fn validator_refuses_a_packet_that_drops_user_authored_files() {
    let corpus = current_incident_workspace_beta_packet_corpus().expect("corpus parses");
    let mut packet = corpus
        .packets()
        .next()
        .expect("at least one fixture")
        .clone();
    packet.workspace_identity.preserves_user_authored_files = false;
    let violations = validate_packet_record(&packet);
    assert!(violations
        .iter()
        .any(|v| v.check_id == "packet.workspace_identity.preserves_user_authored_files"));
}

#[test]
fn validator_refuses_a_packet_that_admits_raw_private_material() {
    let corpus = current_incident_workspace_beta_packet_corpus().expect("corpus parses");
    let mut packet = corpus
        .packets()
        .next()
        .expect("at least one fixture")
        .clone();
    packet.privacy_baseline.raw_private_material_excluded = false;
    let violations = validate_packet_record(&packet);
    assert!(violations
        .iter()
        .any(|v| v.check_id == "packet.privacy_baseline.raw_private_material_excluded"));
}

#[test]
fn validator_refuses_a_held_record_without_the_held_record_blocks_export_token() {
    let corpus = current_incident_workspace_beta_packet_corpus().expect("corpus parses");
    let mut packet = corpus
        .packets()
        .find(|p| {
            p.handoff_consumer_classes
                .contains(&HandoffConsumerClass::SupportIntakeAndSecurityTriage)
        })
        .expect("joint lane fixture present")
        .clone();
    packet
        .claim_state
        .downgrade_tokens
        .retain(|t| t != &ClaimDowngradeToken::HeldRecordBlocksExport);
    let violations = validate_packet_record(&packet);
    assert!(violations
        .iter()
        .any(|v| v.check_id == "packet.claim_state.held_record_token_required"));
}

#[test]
fn validator_refuses_a_managed_copy_without_the_managed_copy_pending_admin_review_token() {
    let corpus = current_incident_workspace_beta_packet_corpus().expect("corpus parses");
    let mut packet = corpus
        .packets()
        .find(|p| {
            p.evidence_artifacts
                .iter()
                .any(|a| a.custody_class == EvidenceCustodyClass::ManagedCopyAvailable)
        })
        .expect("managed copy fixture present")
        .clone();
    packet
        .claim_state
        .downgrade_tokens
        .retain(|t| t != &ClaimDowngradeToken::ManagedCopyPendingAdminReview);
    let violations = validate_packet_record(&packet);
    assert!(violations
        .iter()
        .any(|v| v.check_id == "packet.claim_state.managed_copy_token_required"));
}

#[test]
fn validator_refuses_a_security_route_without_the_security_recovery_option() {
    let corpus = current_incident_workspace_beta_packet_corpus().expect("corpus parses");
    let mut packet = corpus
        .packets()
        .find(|p| {
            p.handoff_consumer_classes
                .contains(&HandoffConsumerClass::SupportIntakeAndSecurityTriage)
        })
        .expect("joint lane fixture present")
        .clone();
    packet
        .recovery_options
        .retain(|opt| opt.recovery_option_class != RecoveryOptionClass::OpenSecurityPrivateTriage);
    let violations = validate_packet_record(&packet);
    assert!(violations
        .iter()
        .any(|v| v.check_id == "packet.recovery_options.security_route_missing"));
}

#[test]
fn validator_refuses_a_packet_with_wrong_doc_ref() {
    let corpus = current_incident_workspace_beta_packet_corpus().expect("corpus parses");
    let mut packet = corpus
        .packets()
        .next()
        .expect("at least one fixture")
        .clone();
    packet.references.doc_ref = "docs/support/wrong.md".to_owned();
    let violations = validate_packet_record(&packet);
    assert!(violations
        .iter()
        .any(|v| v.check_id == "packet.references.doc_ref"));
}
