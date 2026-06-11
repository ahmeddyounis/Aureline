//! Unit tests for the M5 truth-surface evidence-ingestion register.

use super::*;

fn register() -> M5TruthSurfaceIngestion {
    current_m5_truth_surface_ingestion().expect("embedded register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let reg = register();
    assert_eq!(
        reg.schema_version,
        M5_TRUTH_SURFACE_INGESTION_SCHEMA_VERSION
    );
    assert_eq!(reg.record_kind, M5_TRUTH_SURFACE_INGESTION_RECORD_KIND);
    let violations = reg.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn covers_every_family_on_every_surface() {
    let reg = register();
    for kind in FamilyKind::ALL {
        let surfaces: std::collections::BTreeSet<_> =
            reg.rows_for_family(kind).map(|r| r.truth_surface).collect();
        for surface in TruthSurface::ALL {
            assert!(
                surfaces.contains(&surface),
                "family {kind:?} missing surface {surface:?}"
            );
        }
    }
    assert_eq!(
        reg.rows.len(),
        FamilyKind::ALL.len() * TruthSurface::ALL.len()
    );
}

#[test]
fn every_ingest_state_is_represented() {
    let reg = register();
    for state in [
        IngestState::Current,
        IngestState::Stale,
        IngestState::Narrowed,
        IngestState::PolicyBlocked,
        IngestState::PreviewOnly,
        IngestState::Underqualified,
    ] {
        assert!(
            reg.rows.iter().any(|r| r.ingest_state == state),
            "no row in state {state:?}"
        );
    }
}

#[test]
fn every_posture_is_represented_and_always_explicit() {
    let reg = register();
    for posture in [
        PostureClass::LocalOnly,
        PostureClass::Mirrored,
        PostureClass::Managed,
        PostureClass::BrowserHandoff,
    ] {
        assert!(
            reg.rows.iter().any(|r| r.posture == posture),
            "no row with posture {posture:?}"
        );
    }
}

#[test]
fn no_surface_widens_beyond_the_canonical_ceiling() {
    let reg = register();
    for row in &reg.rows {
        assert!(
            row.published_label.rank() <= row.canonical_claim_label.rank(),
            "{} widens past its ceiling",
            row.entry_id
        );
        assert_eq!(
            row.published_label, row.canonical_published_label,
            "{} does not reflect the canonical effective label",
            row.entry_id
        );
    }
}

#[test]
fn service_health_rows_carry_a_contract_state_others_do_not() {
    let reg = register();
    for row in &reg.rows {
        match row.truth_surface {
            TruthSurface::ServiceHealth => assert!(
                row.service_contract_state.is_some(),
                "{} missing contract state",
                row.entry_id
            ),
            _ => assert!(
                row.service_contract_state.is_none(),
                "{} should not carry a contract state",
                row.entry_id
            ),
        }
    }
}

#[test]
fn release_center_shows_a_narrowed_family() {
    let reg = register();
    let narrowed = reg.rows_for_surface(TruthSurface::ReleaseCenter).any(|r| {
        !r.published_label.is_at_or_above_cutline() && !r.active_ingest_reasons.is_empty()
    });
    assert!(
        narrowed,
        "release center should surface at least one narrowed family"
    );
}

#[test]
fn summary_matches_computed() {
    let reg = register();
    assert_eq!(reg.summary, reg.computed_summary());
}

#[test]
fn publication_proceeds_when_nothing_widens() {
    let reg = register();
    assert_eq!(reg.publication.decision, "proceed");
    assert!(reg.publication.blocking_rule_ids.is_empty());
    assert!(reg.publication.blocking_entry_ids.is_empty());
}

#[test]
fn widening_a_published_label_fails_validation() {
    let mut reg = register();
    // Force a surface to advertise stable past a preview ceiling.
    let row = reg
        .rows
        .iter_mut()
        .find(|r| {
            r.canonical_claim_label == StableClaimLevel::Stable
                && r.published_label != StableClaimLevel::Lts
        })
        .expect("a stable-ceiling row exists");
    row.published_label = StableClaimLevel::Lts;
    let violations = reg.validate();
    assert!(
        violations
            .iter()
            .any(|v| matches!(v, IngestionViolation::PublishedExceedsCeiling { .. })),
        "expected a ceiling violation, got {violations:?}"
    );
}

#[test]
fn dropping_a_service_contract_state_fails_validation() {
    let mut reg = register();
    let row = reg
        .rows
        .iter_mut()
        .find(|r| r.truth_surface == TruthSurface::ServiceHealth)
        .expect("a service-health row exists");
    row.service_contract_state = None;
    let violations = reg.validate();
    assert!(
        violations.iter().any(|v| matches!(
            v,
            IngestionViolation::ServiceHealthMissingContractState { .. }
        )),
        "expected a missing-contract-state violation, got {violations:?}"
    );
}
