//! Protected tests for stable runbook source, step envelope, deviation, and
//! browser/vendor-console handoff truth.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::stabilize_runbook_source_step_envelope_and_handoff_truth::{
    current_runbook_handoff_truth_corpus, load_runbook_handoff_truth_packet, AuthoritativePosture,
    DestinationClass, LocalCompletionState, ProviderObjectOwnershipClass, RunbookSourceClass,
    StepResultState, RUNBOOK_HANDOFF_TRUTH_ARTIFACT_REF, RUNBOOK_HANDOFF_TRUTH_DOC_REF,
    RUNBOOK_HANDOFF_TRUTH_FIXTURE_DIR, RUNBOOK_HANDOFF_TRUTH_FIXTURE_MANIFEST_REF,
    RUNBOOK_HANDOFF_TRUTH_PACKET_RECORD_KIND, RUNBOOK_HANDOFF_TRUTH_SCHEMA_VERSION,
    RUNBOOK_STEP_ENVELOPE_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn corpus_validates_and_covers_required_source_classes() {
    let corpus = current_runbook_handoff_truth_corpus().expect("corpus parses");
    assert!(!corpus.is_empty(), "corpus must not be empty");

    let mut source_classes = BTreeSet::new();
    for packet in &corpus {
        assert_eq!(packet.record_kind, RUNBOOK_HANDOFF_TRUTH_PACKET_RECORD_KIND);
        assert_eq!(packet.schema_version, RUNBOOK_HANDOFF_TRUTH_SCHEMA_VERSION);
        assert_eq!(packet.validate(), Vec::new(), "{packet:#?}");
        for source in &packet.source_descriptors {
            source_classes.insert(source.source_class);
        }
    }

    for required in RunbookSourceClass::REQUIRED {
        assert!(
            source_classes.contains(&required),
            "missing required source class: {required:?}"
        );
    }
}

#[test]
fn declared_schema_docs_artifact_and_fixture_paths_exist() {
    let root = repo_root();
    for path in [
        RUNBOOK_STEP_ENVELOPE_SCHEMA_REF,
        RUNBOOK_HANDOFF_TRUTH_DOC_REF,
        RUNBOOK_HANDOFF_TRUTH_ARTIFACT_REF,
        RUNBOOK_HANDOFF_TRUTH_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(path).is_file(), "{path} missing");
    }
    assert!(
        root.join(RUNBOOK_HANDOFF_TRUTH_FIXTURE_DIR).is_dir(),
        "{RUNBOOK_HANDOFF_TRUTH_FIXTURE_DIR} missing"
    );
}

#[test]
fn fixtures_round_trip_from_disk() {
    let root = repo_root();
    for file_name in [
        "managed_catalog_execution_with_deviation.yaml",
        "browser_vendor_console_handoff.yaml",
        "local_checklist_handoff_bundle.yaml",
    ] {
        let path = root.join(RUNBOOK_HANDOFF_TRUTH_FIXTURE_DIR).join(file_name);
        let yaml = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read {}: {err}", path.display()));
        let packet = load_runbook_handoff_truth_packet(&yaml)
            .unwrap_or_else(|err| panic!("parse {}: {err}", path.display()));
        assert_eq!(packet.validate(), Vec::new(), "{file_name} invalid");
    }
}

#[test]
fn result_states_stay_distinct_from_advisory_prose() {
    let corpus = current_runbook_handoff_truth_corpus().expect("corpus parses");
    let mut states = BTreeSet::new();
    for packet in &corpus {
        for execution in &packet.execution_records {
            states.insert(execution.result_state);
            assert!(
                !execution.incident_timeline_id.is_empty(),
                "{} missing incident timeline join",
                execution.execution_id
            );
            assert!(
                !execution.evidence_export_links.is_empty(),
                "{} missing evidence/export links",
                execution.execution_id
            );
        }
    }

    for required in StepResultState::REQUIRED {
        assert!(
            states.contains(&required),
            "missing result state: {required:?}"
        );
    }
}

#[test]
fn mutating_in_product_steps_reuse_shared_action_envelope() {
    let corpus = current_runbook_handoff_truth_corpus().expect("corpus parses");
    for packet in &corpus {
        for step in &packet.step_envelopes {
            if step.step_class.is_mutating()
                && step.destination_class == DestinationClass::InProduct
            {
                let envelope = step
                    .shared_action_envelope
                    .as_ref()
                    .unwrap_or_else(|| panic!("{} missing shared action envelope", step.step_id));
                assert!(!envelope.action_envelope_ref.is_empty());
                assert!(!envelope.preview_hash_ref.is_empty());
                assert!(!envelope.approval_ref.is_empty());
                assert!(!envelope.audit_ref.is_empty());
            }
        }
    }
}

#[test]
fn browser_only_vendor_docs_are_reference_only_with_handoff() {
    let corpus = current_runbook_handoff_truth_corpus().expect("corpus parses");
    let browser_packet = corpus
        .iter()
        .find(|packet| {
            packet.source_descriptors.iter().any(|source| {
                source.source_class == RunbookSourceClass::BrowserOnlyVendorDocumentation
            })
        })
        .expect("browser-only packet");

    for source in &browser_packet.source_descriptors {
        if source.source_class == RunbookSourceClass::BrowserOnlyVendorDocumentation {
            assert_eq!(
                source.authoritative_posture,
                AuthoritativePosture::ReferenceOnly
            );
        }
    }

    assert!(browser_packet
        .step_envelopes
        .iter()
        .any(|step| step.external_handoff.is_some()));
    assert!(browser_packet
        .handoff_bundles
        .iter()
        .any(
            |bundle| bundle.destination_class == DestinationClass::VendorConsole
                && bundle.raw_provider_payload_excluded
        ));
}

#[test]
fn local_checklist_completion_does_not_mutate_provider_owned_objects() {
    let corpus = current_runbook_handoff_truth_corpus().expect("corpus parses");
    let follow_up = corpus
        .iter()
        .flat_map(|packet| &packet.local_follow_ups)
        .find(|follow_up| {
            follow_up.provider_object_ownership == ProviderObjectOwnershipClass::ProviderOwned
        })
        .expect("provider-owned local follow-up");

    assert_eq!(
        follow_up.local_completion_state,
        LocalCompletionState::LocalChecklistOnly
    );
    assert!(!follow_up.provider_mutation_claimed);
    assert!(follow_up.reviewed_command_ref.is_none());
}

#[test]
fn validator_rejects_hidden_browser_or_provider_mutation_claims() {
    let mut corpus = current_runbook_handoff_truth_corpus().expect("corpus parses");
    let packet = corpus
        .iter_mut()
        .find(|packet| !packet.local_follow_ups.is_empty())
        .expect("local follow-up packet");
    packet.local_follow_ups[0].provider_mutation_claimed = true;

    let violations = packet.validate();
    assert!(violations.iter().any(|violation| {
        violation.check_id == "follow_up.local_completion_overclaims_provider_mutation"
    }));
    assert!(violations.iter().any(|violation| {
        violation.check_id == "follow_up.provider_mutation_without_reviewed_command"
    }));
}
