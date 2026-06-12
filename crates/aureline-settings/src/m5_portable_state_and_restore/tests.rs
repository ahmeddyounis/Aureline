//! Unit tests for the portable-state restore gate and its fail-closed guards.

use super::corpus::portable_state_restore_corpus;
use super::model::{
    BuildError, ExclusionReason, M5PortableStateRestoreCertification, M5PortableStateRestoreInput,
    MigrationLabel, PortabilityDisposition, PortableArtifactClass, PortableRestoreClaim,
};

fn baseline_input() -> M5PortableStateRestoreInput {
    let scenario = portable_state_restore_corpus()
        .into_iter()
        .find(|s| s.scenario_id == "exact_local_restore")
        .expect("baseline scenario exists");
    let record = scenario.record();
    M5PortableStateRestoreInput {
        record_id: record.record_id,
        as_of: record.as_of,
        summary: record.summary,
        package_classes: record.package_classes,
        restore_cards: record.restore_cards,
        surface_truth: record.surface_truth,
    }
}

#[test]
fn baseline_qualifies_for_exact_restore() {
    let record = M5PortableStateRestoreCertification::build(baseline_input()).expect("builds");
    assert_eq!(
        record.fidelity_qualification.claim_class,
        PortableRestoreClaim::ExactRestore
    );
    assert!(record.fidelity_qualification.qualifies_exact);
    assert_eq!(
        record.fidelity_qualification.effective_fidelity_ceiling,
        MigrationLabel::Exact
    );
    assert!(record.fidelity_qualification.narrowing_reasons.is_empty());
}

#[test]
fn every_required_artifact_class_must_be_classified() {
    let mut input = baseline_input();
    input
        .package_classes
        .retain(|row| row.artifact_class != PortableArtifactClass::DocsPacks);
    let err = M5PortableStateRestoreCertification::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::MissingArtifactClass {
            class: PortableArtifactClass::DocsPacks
        }
    );
}

#[test]
fn secret_class_cannot_be_carried_as_portable() {
    let mut input = baseline_input();
    let row = input
        .package_classes
        .iter_mut()
        .find(|row| row.artifact_class == PortableArtifactClass::SelectedSettings)
        .expect("settings row");
    row.disposition = PortabilityDisposition::Portable;
    row.exclusion_reason = Some(ExclusionReason::SecretMaterial);
    let err = M5PortableStateRestoreCertification::build(input).unwrap_err();
    assert_eq!(
        err,
        BuildError::SecretCarriedAsPortable {
            class: PortableArtifactClass::SelectedSettings,
            reason: ExclusionReason::SecretMaterial
        }
    );
}

#[test]
fn exact_claim_with_missing_dependency_is_rejected() {
    let mut input = baseline_input();
    let card = &mut input.restore_cards[0];
    card.migration_label = MigrationLabel::Exact;
    card.placeholders
        .push(super::model::MissingDependencyPlaceholder {
            kind: super::model::MissingDependencyKind::MissingExtension,
            affected_artifact_class: card.artifact_class,
            placeholder_ref: "aureline://placeholder/missing-extension".to_owned(),
            visible_in_layout: true,
            silently_dropped: false,
            recovery_hint: "install it".to_owned(),
        });
    let card_id = card.card_id.clone();
    let err = M5PortableStateRestoreCertification::build(input).unwrap_err();
    assert_eq!(err, BuildError::ExactClaimWithMissingDependency { card_id });
}

#[test]
fn exact_claim_across_schema_mismatch_is_rejected() {
    let mut input = baseline_input();
    let card = &mut input.restore_cards[0];
    card.migration_label = MigrationLabel::Exact;
    card.target_schema_version = "settings:v999".to_owned();
    let card_id = card.card_id.clone();
    let err = M5PortableStateRestoreCertification::build(input).unwrap_err();
    assert_eq!(err, BuildError::ExactClaimAcrossSchemaMismatch { card_id });
}

#[test]
fn silently_dropped_placeholder_narrows_below_exact() {
    let mut input = baseline_input();
    let card = &mut input.restore_cards[0];
    card.migration_label = MigrationLabel::LayoutOnly;
    card.placeholders
        .push(super::model::MissingDependencyPlaceholder {
            kind: super::model::MissingDependencyKind::MissingRemoteTarget,
            affected_artifact_class: card.artifact_class,
            placeholder_ref: "aureline://placeholder/dropped".to_owned(),
            visible_in_layout: false,
            silently_dropped: true,
            recovery_hint: "reconnect".to_owned(),
        });
    let record = M5PortableStateRestoreCertification::build(input).expect("builds");
    assert!(!record.pillars.placeholders_visible);
    assert_eq!(
        record.fidelity_qualification.claim_class,
        PortableRestoreClaim::Unsupported
    );
}

#[test]
fn downgrade_drills_stay_below_exact_but_remain_restorable() {
    for scenario in portable_state_restore_corpus() {
        if scenario.scenario_id == "exact_local_restore" {
            continue;
        }
        let record = scenario.record();
        assert_ne!(
            record.fidelity_qualification.effective_fidelity_ceiling,
            MigrationLabel::Exact,
            "{} should be below exact fidelity",
            scenario.scenario_id
        );
        assert_eq!(
            record.fidelity_qualification.claim_class,
            PortableRestoreClaim::DegradedRestore,
            "{} should remain a sound degraded restore",
            scenario.scenario_id
        );
    }
}

#[test]
fn corpus_covers_every_migration_label_and_dependency_kind() {
    use std::collections::BTreeSet;
    let mut labels = BTreeSet::new();
    let mut kinds = BTreeSet::new();
    for scenario in portable_state_restore_corpus() {
        let record = scenario.record();
        labels.extend(record.migration_label_coverage);
        kinds.extend(record.missing_dependency_coverage);
    }
    for label in MigrationLabel::ALL {
        assert!(labels.contains(&label), "missing label {label:?}");
    }
    for kind in [
        super::model::MissingDependencyKind::MissingExtension,
        super::model::MissingDependencyKind::MissingRemoteTarget,
        super::model::MissingDependencyKind::UnsupportedClient,
    ] {
        assert!(kinds.contains(&kind), "missing kind {kind:?}");
    }
}
