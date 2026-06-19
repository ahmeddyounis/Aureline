//! Inline unit tests for the typed M5 interchange-conformance register.

use super::*;

fn register() -> M5InterchangeConformanceRegister {
    current_m5_interchange_conformance_register().expect("checked-in register parses")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, M5_INTERCHANGE_CONFORMANCE_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_INTERCHANGE_CONFORMANCE_RECORD_KIND);
    assert_eq!(r.register_id, M5_INTERCHANGE_CONFORMANCE_REGISTER_ID);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn summary_recomputes_from_rows() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert!(r.summary.total_families > 0);
    assert_eq!(
        r.summary.conformant_families + r.summary.narrowed_families + r.summary.failed_families,
        r.rows.len(),
        "every family is conformant, narrowed, or failed"
    );
}

#[test]
fn every_named_interchange_family_is_present() {
    // The acceptance anchor: each named M5 interchange family has a validator and a
    // cross-surface conformance runner exercising a real emitted artifact.
    let r = register();
    for family in [
        "request_api_collections",
        "notebook_parity_exports",
        "docs_packets",
        "trace_profile_replay_exports",
        "support_bundles",
        "portable_state_packages",
    ] {
        let row = r
            .row(family)
            .unwrap_or_else(|| panic!("{family} is registered"));
        assert!(
            !row.validator.descriptor_ref.is_empty(),
            "{family} has an import/export validator"
        );
        assert!(
            !row.runner.artifact_ref.is_empty(),
            "{family} has a conformance runner exercising a real emitted artifact"
        );
        assert_eq!(
            row.runner.surfaces_exercised,
            ConsumerSurface::ALL.to_vec(),
            "{family} runner exercises every consumer surface"
        );
        assert!(
            !row.validator.reason_codes_emitted.is_empty(),
            "{family} enumerates the reason codes its validator reports"
        );
    }
}

#[test]
fn every_row_has_one_dimension_per_kind() {
    let r = register();
    for row in &r.rows {
        let kinds: Vec<DimensionKind> = row.dimensions.iter().map(|d| d.dimension_kind).collect();
        assert_eq!(
            kinds,
            DimensionKind::ALL.to_vec(),
            "{} must carry one dimension per kind in order",
            row.family_id
        );
    }
}

#[test]
fn conformance_state_and_decision_follow_the_dimensions() {
    let r = register();
    for row in &r.rows {
        assert_eq!(
            row.conformance_state,
            row.computed_conformance_state(),
            "{} conformance state must follow its dimensions",
            row.family_id
        );
        assert_eq!(
            row.decision,
            row.computed_decision(),
            "{} decision must follow its conformance state",
            row.family_id
        );
    }
}

#[test]
fn consumers_agree_on_version_label_and_degraded_vocabulary() {
    // The acceptance anchor: desktop, CLI/headless, and support/export consumers agree on
    // contract version, lifecycle label, and degraded-state vocabulary for each family.
    let r = register();
    for row in &r.rows {
        let a = &row.consumer_agreement;
        assert!(a.agrees, "{}: consumers must agree", row.family_id);
        assert_eq!(a.surfaces, ConsumerSurface::ALL.to_vec());
        assert_eq!(a.agreed_contract_version, row.contract_version);
        assert_eq!(a.agreed_lifecycle_label, row.lifecycle_label);
        assert_eq!(a.agreed_degraded_states, row.degraded_states_supported);
    }
}

#[test]
fn compare_only_is_a_valid_class_not_a_downgrade() {
    // The guardrail: a family scoped to compare-only behavior is conformant in that class,
    // not forced to support write-back.
    let r = register();
    let notebook = r
        .row("notebook_parity_exports")
        .expect("notebook_parity_exports present");
    assert_eq!(notebook.conformance_class, ConformanceClass::CompareOnly);
    assert!(!notebook.conformance_class.requires_write_back());
    assert_eq!(notebook.conformance_state, ConformanceState::Conformant);
    assert_eq!(notebook.decision, DecisionState::Clear);
}

#[test]
fn catalog_linked_family_inherits_its_published_label() {
    let r = register();
    let support = r.row("support_bundles").expect("support_bundles present");
    assert_eq!(support.catalog_family_id, "support_bundles_and_handoff");
    assert_eq!(support.lifecycle_label, LifecycleLabel::Stable);
    assert!(support.catalog_entry_ref.is_some());

    let trace = r
        .row("trace_profile_replay_exports")
        .expect("trace_profile_replay_exports present");
    assert_eq!(trace.catalog_family_id, "replay_and_trace_evidence");
    assert_eq!(trace.lifecycle_label, LifecycleLabel::Beta);
}

#[test]
fn unlinked_family_has_no_catalog_refs() {
    let r = register();
    let req = r.row("request_api_collections").unwrap();
    assert!(req.catalog_family_id.is_empty());
    assert!(req.catalog_entry_ref.is_none());
    assert!(req.matrix_row_ref.is_none());
}

#[test]
fn clear_register_resolves_and_projects() {
    let r = register();
    let (label, state, decision) = r
        .resolve_conformance("portable_state_packages")
        .expect("portable_state_packages resolves");
    assert_eq!(label, LifecycleLabel::Beta);
    assert_eq!(state, ConformanceState::Conformant);
    assert_eq!(decision, DecisionState::Clear);
    assert!(!r.holds_promotion());
    assert!(r.failed_rows().is_empty());
    assert!(r.resolve_conformance("not_a_family").is_none());

    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.rows.len());
    for prow in &projection.rows {
        let src = r
            .row(&prow.family_id)
            .expect("projection family is in the model");
        assert_eq!(prow.lifecycle_label, src.lifecycle_label);
        assert_eq!(prow.conformance_state, src.conformance_state);
    }
}

#[test]
fn duplicate_family_id_is_rejected() {
    let mut r = register();
    let dup = r.rows[0].clone();
    r.rows.push(dup);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "rows.duplicate_family_id"));
}

#[test]
fn missing_dimension_is_rejected() {
    let mut r = register();
    r.rows[0].dimensions.pop();
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "rows.dimension_coverage"));
}

#[test]
fn silently_widened_trust_is_rejected() {
    // A release-blocking family whose required trust dimension fails but is still labeled
    // conformant must be rejected: import must not silently widen trust.
    let mut r = register();
    let idx = r
        .rows
        .iter()
        .position(|row| row.release_blocking)
        .expect("a release-blocking row");
    for dim in &mut r.rows[idx].dimensions {
        if dim.dimension_kind == DimensionKind::TrustNotWidened {
            dim.outcome = DimensionOutcome::Fail;
        }
    }
    // conformance_state still says Conformant -> recompute disagrees.
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "rows.conformance_state"));
}

#[test]
fn summary_drift_is_rejected() {
    let mut r = register();
    r.summary.total_families += 1;
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "summary.count_mismatch"));
}
