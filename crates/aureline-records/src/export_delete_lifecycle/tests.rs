use super::*;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_records_export_delete_lifecycle_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn seeded_packet_covers_required_families() {
    let packet = seeded_records_export_delete_lifecycle_packet();
    let families = packet
        .family_links
        .iter()
        .map(|link| link.artifact_family)
        .collect::<BTreeSet<_>>();

    assert!(families.contains(&GovernedArtifactFamily::AiEvidencePacket));
    assert!(families.contains(&GovernedArtifactFamily::ProviderLinkedWorkItem));
    assert!(families.contains(&GovernedArtifactFamily::SyncMirrorLedger));
    assert!(families.contains(&GovernedArtifactFamily::IncidentSupportPacket));
    assert!(families.contains(&GovernedArtifactFamily::OffboardingRecord));
}

#[test]
fn every_export_job_has_manifest() {
    let packet = seeded_records_export_delete_lifecycle_packet();
    assert!(packet
        .export_jobs
        .iter()
        .all(|job| !job.manifest.bundle_id.trim().is_empty()));
}

#[test]
fn terminal_delete_results_require_receipt_or_blocker() {
    let packet = seeded_records_export_delete_lifecycle_packet();
    for case in &packet.delete_cases {
        match case.outcome {
            RecordOperationOutcome::Completed | RecordOperationOutcome::Partial => {
                assert!(case.destruction_receipt.is_some(), "{case:?}");
            }
            RecordOperationOutcome::BlockedByHold
            | RecordOperationOutcome::PolicyRetained
            | RecordOperationOutcome::OutsidePlatformScope
            | RecordOperationOutcome::ManualLocalCaptureRequired
            | RecordOperationOutcome::NotFound
            | RecordOperationOutcome::OmittedByRedaction => {
                assert!(case.typed_blocker_state.is_some(), "{case:?}");
            }
            RecordOperationOutcome::Requested | RecordOperationOutcome::Queued => {}
        }
    }
}

#[test]
fn projections_cover_every_family_link() {
    let packet = seeded_records_export_delete_lifecycle_packet();
    assert_eq!(
        packet.cli_headless_projection().len(),
        packet.family_links.len()
    );
    assert_eq!(
        packet.help_docs_projection().len(),
        packet.family_links.len()
    );
    assert_eq!(
        packet.support_export_projection().len(),
        packet.family_links.len()
    );
}

#[test]
fn checked_in_canonical_fixture_matches_seeded_packet() {
    let fixture = repo_root()
        .join("fixtures/governance/records_export_delete_lifecycle/canonical_packet.yaml");
    let raw = std::fs::read_to_string(&fixture).expect("canonical fixture is readable");
    let parsed: RecordsExportDeleteLifecyclePacket =
        serde_yaml::from_str(&raw).expect("canonical fixture parses");

    assert!(
        parsed.validate().is_empty(),
        "canonical fixture must validate cleanly: {:?}",
        parsed.validate()
    );
    assert_eq!(
        parsed,
        seeded_records_export_delete_lifecycle_packet(),
        "canonical fixture drifted from the seeded packet; regenerate it"
    );
}

#[test]
fn missing_receipt_is_rejected() {
    let mut packet = seeded_records_export_delete_lifecycle_packet();
    let case = packet
        .delete_cases
        .iter_mut()
        .find(|case| case.outcome == RecordOperationOutcome::Completed)
        .expect("completed case");
    case.destruction_receipt = None;

    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        RecordsExportDeleteLifecycleViolation::DeleteCaseMissingReceipt { .. }
    )));
}
