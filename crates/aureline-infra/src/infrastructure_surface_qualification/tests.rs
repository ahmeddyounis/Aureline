use super::{
    derive_displayed_posture, derive_narrow_reasons, EvidenceCurrency, InfrastructureNarrowReason,
    InfrastructureProofClass,
};
use crate::QualificationPosture;

#[test]
fn stable_posture_requires_all_current_proofs() {
    let proofs = vec![
        InfrastructureProofClass::RelationshipGraph,
        InfrastructureProofClass::TargetContext,
        InfrastructureProofClass::LiveCounterpart,
        InfrastructureProofClass::ExportParity,
    ];
    assert_eq!(
        derive_displayed_posture(&proofs, &proofs, EvidenceCurrency::Current),
        QualificationPosture::StableQualified
    );
    assert!(derive_narrow_reasons(&proofs, &proofs, EvidenceCurrency::Current).is_empty());
}

#[test]
fn missing_relationship_proof_narrows_to_file_only() {
    let required = vec![
        InfrastructureProofClass::RelationshipGraph,
        InfrastructureProofClass::ExportParity,
    ];
    let satisfied = vec![InfrastructureProofClass::ExportParity];
    assert_eq!(
        derive_displayed_posture(&required, &satisfied, EvidenceCurrency::Current),
        QualificationPosture::FileOnly
    );
    assert_eq!(
        derive_narrow_reasons(&required, &satisfied, EvidenceCurrency::Current),
        vec![InfrastructureNarrowReason::MissingRelationshipProof]
    );
}

#[test]
fn missing_handoff_boundary_narrows_to_handoff_only() {
    let required = vec![
        InfrastructureProofClass::TargetContext,
        InfrastructureProofClass::HandoffBoundary,
        InfrastructureProofClass::ExportParity,
    ];
    let satisfied = vec![
        InfrastructureProofClass::TargetContext,
        InfrastructureProofClass::ExportParity,
    ];
    assert_eq!(
        derive_displayed_posture(&required, &satisfied, EvidenceCurrency::Current),
        QualificationPosture::HandoffOnly
    );
}

#[test]
fn stale_proof_narrows_even_when_all_proofs_exist() {
    let proofs = vec![
        InfrastructureProofClass::TargetContext,
        InfrastructureProofClass::WrongTargetDrill,
        InfrastructureProofClass::StaleLiveOverlayDrill,
        InfrastructureProofClass::ExportParity,
    ];
    assert_eq!(
        derive_displayed_posture(&proofs, &proofs, EvidenceCurrency::Stale),
        QualificationPosture::InspectOnly
    );
    assert_eq!(
        derive_narrow_reasons(&proofs, &proofs, EvidenceCurrency::Stale),
        vec![InfrastructureNarrowReason::EvidenceStale]
    );
}

#[test]
fn missing_export_parity_blocks_the_surface() {
    let required = vec![
        InfrastructureProofClass::TargetContext,
        InfrastructureProofClass::ExportParity,
    ];
    let satisfied = vec![InfrastructureProofClass::TargetContext];
    assert_eq!(
        derive_displayed_posture(&required, &satisfied, EvidenceCurrency::Current),
        QualificationPosture::Downgraded
    );
}
