//! Inline tests for the M5 runbook handoff register.

use super::*;

use crate::m5_runbook_governance::{seeded_operator_scenario_records, HandoffReasonClass};

fn canonical() -> M5RunbookHandoffRegister {
    seeded_m5_runbook_handoff_register()
}

#[test]
fn canonical_register_validates() {
    let register = canonical();
    assert!(register.validate().is_empty(), "{:?}", register.validate());
    assert_eq!(register.register_id, M5_RUNBOOK_HANDOFF_REGISTER_ID);
    assert_eq!(
        register.record_kind,
        M5_RUNBOOK_HANDOFF_REGISTER_RECORD_KIND
    );
    assert!(!register.handoffs.is_empty());
    assert_eq!(register.projections.len(), register.handoffs.len());
    assert!(register.conformance.all_hold());
    assert!(register.vocabulary.matches_canonical());
}

#[test]
fn every_handoff_is_an_explicit_attributable_transition() {
    let register = canonical();
    for p in &register.projections {
        // A pivot names where it goes, why, and whom it is attributed to.
        assert!(!p.destination_class.is_empty());
        assert!(!p.reason_class.is_empty());
        assert!(!p.destination_object_ref.is_empty());
        assert!(!p.attribution_ref.is_empty());
        assert!(p.attributable, "handoff {} unattributable", p.handoff_id);
        assert!(!p.creates_hidden_mutate_channel);
        // No pivot is ever in-product control.
        assert!(!p.executable_in_product);
    }
}

#[test]
fn return_anchors_preserve_target_and_evidence_identity() {
    let register = canonical();
    for p in &register.projections {
        assert!(
            !p.return_initiating_object_ref.is_empty(),
            "handoff {} has no return object",
            p.handoff_id
        );
        assert!(!p.return_target_continuity_ref.is_empty());
        assert!(!p.return_evidence_continuity_ref.is_empty());
        assert!(p.return_message_id.starts_with("runbooks_governance."));
    }
    // The live vendor-console pivot keeps the initiating execution's target/evidence.
    let vendor = register.projection("vendor-scale").expect("vendor handoff");
    assert_eq!(
        vendor.return_target_continuity_ref,
        "target:vendor-console/scaling-group"
    );
    assert_eq!(
        vendor.return_evidence_continuity_ref,
        "evidence:vendor:handoff"
    );
}

#[test]
fn a_reference_only_destination_is_read_only_not_control() {
    let register = canonical();
    let docs = register
        .projection("vendor-scaling-docs")
        .expect("reference-doc handoff");
    assert!(docs.is_reference_only);
    assert!(!docs.is_true_control_plane);
    assert!(!docs.executable_in_product);
    assert_eq!(
        docs.reason_class,
        HandoffReasonClass::ConsultReferenceDocumentation.as_str()
    );
}

#[test]
fn handoff_required_destination_is_the_true_control_plane() {
    let register = canonical();
    let vendor = register.projection("vendor-scale").expect("vendor handoff");
    assert!(vendor.is_true_control_plane);
    assert!(!vendor.is_reference_only);
    assert!(!vendor.executable_in_product);
}

#[test]
fn reference_plane_catalog_marks_every_destination_non_in_product() {
    let register = canonical();
    assert!(!register.reference_plane.is_empty());
    for entry in &register.reference_plane {
        assert!(entry.validate().is_empty(), "{:?}", entry.validate());
        assert!(
            !entry.executable_in_product,
            "{} is in-product",
            entry.entry_id
        );
        assert_eq!(
            entry.is_reference_only,
            entry.reference_plane_state.is_reference_only()
        );
    }
    // A browser reference doc is reference-only; a vendor console is handoff-required.
    let docs = register
        .reference_plane
        .iter()
        .find(|e| e.entry_id == "ref:vendor-scaling-docs")
        .expect("docs entry");
    assert!(docs.is_reference_only);
    let console = register
        .reference_plane
        .iter()
        .find(|e| e.entry_id == "ref:vendor-scaling-console")
        .expect("console entry");
    assert!(console.is_true_control_plane);
}

#[test]
fn a_reference_only_entry_claiming_in_product_control_is_rejected() {
    let mut register = canonical();
    register.reference_plane[1].executable_in_product = true;
    assert!(register
        .validate()
        .contains(&M5RunbookHandoffViolation::ReferenceOnlyClaimsInProductControl));
}

#[test]
fn operator_scenario_handoffs_are_all_represented() {
    let register = canonical();
    assert!(
        register
            .conformance
            .operator_scenario_handoffs_are_represented
    );
    for record in seeded_operator_scenario_records() {
        for step in &record.executed_steps {
            if let Some(handoff) = &step.handoff {
                assert!(
                    register.projection(&handoff.handoff_id).is_some(),
                    "scenario handoff {} missing from register",
                    handoff.handoff_id
                );
            }
        }
    }
}

#[test]
fn projection_is_identical_across_surfaces() {
    let register = canonical();
    let incident = register.projections_for_surface(RunbookHandoffSurface::IncidentWorkspace);
    let operator = register.projections_for_surface(RunbookHandoffSurface::OperatorHistory);
    let support = register.projections_for_surface(RunbookHandoffSurface::SupportExport);
    let docs = register.projections_for_surface(RunbookHandoffSurface::DocsHelp);
    assert_eq!(incident, operator);
    assert_eq!(incident, support);
    assert_eq!(incident, docs);
    assert_eq!(incident, register.projections);
}

#[test]
fn projection_drift_is_caught() {
    let mut register = canonical();
    register.projections[0].is_true_control_plane = !register.projections[0].is_true_control_plane;
    assert!(register
        .validate()
        .contains(&M5RunbookHandoffViolation::ProjectionDrift));
}

#[test]
fn duplicate_handoff_ids_are_rejected() {
    let mut register = canonical();
    let dup = register.handoffs[0].clone();
    register.handoffs.push(dup);
    register.projections = register
        .handoffs
        .iter()
        .map(RunbookHandoffProjection::derive)
        .collect();
    assert!(register
        .validate()
        .contains(&M5RunbookHandoffViolation::DuplicateHandoffId));
}

#[test]
fn every_destination_class_and_reference_plane_state_is_represented() {
    let register = canonical();
    let destinations: std::collections::BTreeSet<&str> = register
        .projections
        .iter()
        .map(|p| p.destination_class.as_str())
        .collect();
    for class in HandoffDestinationClass::ALL {
        assert!(
            destinations.contains(class.as_str()),
            "destination class {} not represented",
            class.as_str()
        );
    }
    assert!(register.projections.iter().any(|p| p.is_reference_only));
    assert!(register.projections.iter().any(|p| p.is_true_control_plane));
}

#[test]
fn round_trips_through_json() {
    let register = canonical();
    let json = register.export_safe_json();
    let parsed: M5RunbookHandoffRegister = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(parsed, register);
    assert!(parsed.validate().is_empty());
}

#[test]
fn markdown_summary_names_handoffs_and_reference_plane() {
    let summary = canonical().render_markdown_summary();
    assert!(summary.contains("Governed handoffs"));
    assert!(summary.contains("Reference-plane catalog"));
    assert!(summary.contains("never present as in-product control"));
    assert!(summary.contains("vendor-scale"));
}

#[test]
fn export_carries_no_forbidden_boundary_material() {
    let json = canonical().export_safe_json();
    for needle in ["credential", "secret", "password", "bearer_token"] {
        assert!(!json.contains(needle), "export leaked {needle}");
    }
}
