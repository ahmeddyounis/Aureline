use super::*;

fn packet() -> M5AdmissionAndRoutingPacket {
    current_m5_admission_and_routing_packet().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_ADMISSION_AND_ROUTING_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_ADMISSION_AND_ROUTING_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_matches_recomputed() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_wedge_has_a_checkpoint() {
    // Every claimed M5 wedge carries a truthful admission checkpoint, not an empty state.
    let packet = packet();
    assert!(packet.covers_every_wedge());
    for wedge in M5Wedge::ALL {
        assert!(
            packet.checkpoints_for_wedge(wedge).next().is_some(),
            "wedge {} has no checkpoint",
            wedge.as_str()
        );
    }
}

#[test]
fn every_admission_class_is_exercised_and_distinct() {
    // Certified, probable, mixed, unknown, restricted, and missing-prerequisite stay distinct.
    let packet = packet();
    for class in AdmissionClass::ALL {
        assert!(
            packet.checkpoints_with_class(class).next().is_some(),
            "admission class {} is not exercised",
            class.as_str()
        );
    }
}

#[test]
fn every_first_useful_work_route_is_exercised() {
    let packet = packet();
    for route in FirstUsefulWorkRoute::ALL {
        let exercised = packet
            .checkpoints
            .iter()
            .any(|c| c.first_useful_work_route == route);
        assert!(exercised, "route {} is not exercised", route.as_str());
    }
}

#[test]
fn every_setup_timing_is_exercised() {
    let packet = packet();
    for timing in SetupTiming::ALL {
        let exercised = packet
            .checkpoints
            .iter()
            .flat_map(|c| c.setup_items.iter())
            .any(|i| i.timing == timing);
        assert!(
            exercised,
            "setup timing {} is not exercised",
            timing.as_str()
        );
    }
}

#[test]
fn no_setup_item_ever_auto_runs() {
    let packet = packet();
    for cp in &packet.checkpoints {
        assert!(
            !cp.has_auto_run_setup(),
            "checkpoint {} has a setup item that auto-runs",
            cp.checkpoint_id
        );
        for item in &cp.setup_items {
            assert!(!item.auto_runs, "setup item must not auto-run");
        }
    }
}

#[test]
fn all_trust_and_layout_guards_stay_closed() {
    // Probable or mixed detection never auto-installs packs, rewrites layout, or widens trust.
    let packet = packet();
    for cp in &packet.checkpoints {
        assert!(
            cp.guards_closed(),
            "checkpoint {} holds a guardrail flag open",
            cp.checkpoint_id
        );
        assert!(!cp.forces_wizard, "no checkpoint forces a wizard");
        assert!(!cp.auto_installs_packs, "no checkpoint auto-installs packs");
        assert!(!cp.rewrites_layout_without_review);
        assert!(!cp.widens_trust_without_review);
    }
}

#[test]
fn only_certified_presents_as_certified_support() {
    // Probable or mixed detection is never presented as certified support.
    let packet = packet();
    for cp in &packet.checkpoints {
        assert_eq!(
            cp.presented_as_certified_support,
            cp.admission_class == AdmissionClass::Certified,
            "checkpoint {} certified-presentation diverges from its class",
            cp.checkpoint_id
        );
    }
}

#[test]
fn admission_class_never_outranks_archetype_confidence() {
    let packet = packet();
    for cp in &packet.checkpoints {
        assert!(
            cp.class_within_confidence(),
            "checkpoint {} admission class out-ranks archetype confidence",
            cp.checkpoint_id
        );
        assert!(
            cp.bundle_within_confidence(),
            "checkpoint {} bundle recommendation out-ranks archetype confidence",
            cp.checkpoint_id
        );
        assert!(
            cp.detection_source_canonical(),
            "checkpoint {} detection source is not canonical for its class",
            cp.checkpoint_id
        );
    }
}

#[test]
fn non_blocking_setup_is_deferrable_without_losing_local_safe_work() {
    // Users can defer non-blocking setup without losing minimal local-safe work; only an explicit
    // policy restriction removes it.
    let packet = packet();
    for cp in &packet.checkpoints {
        assert_eq!(
            cp.local_safe_work_available,
            cp.admission_class != AdmissionClass::Restricted,
            "checkpoint {} local-safe availability diverges from the gate",
            cp.checkpoint_id
        );
        if !cp.has_blocking_setup() && cp.admission_class != AdmissionClass::Restricted {
            assert!(
                cp.local_safe_work_available,
                "checkpoint {} should keep local-safe work when nothing blocks",
                cp.checkpoint_id
            );
        }
    }
    // The missing-prerequisite profiler keeps local-safe work even with a blocking gate.
    let profiler = packet
        .checkpoint("m5-adm:profiler-missing-prereq")
        .expect("profiler checkpoint present");
    assert!(profiler.has_blocking_setup());
    assert!(profiler.local_safe_work_available);
    // Only the restricted folder removes local-safe work.
    let restricted = packet
        .checkpoint("m5-adm:local-folder-restricted")
        .expect("restricted checkpoint present");
    assert_eq!(restricted.admission_class, AdmissionClass::Restricted);
    assert!(!restricted.local_safe_work_available);
}

#[test]
fn no_route_is_a_forced_wizard() {
    let packet = packet();
    for cp in &packet.checkpoints {
        assert!(
            cp.route_is_consistent(),
            "checkpoint {} route is inconsistent with its class and blocking",
            cp.checkpoint_id
        );
        assert!(
            !cp.forces_wizard,
            "checkpoint {} forces a wizard",
            cp.checkpoint_id
        );
    }
}

#[test]
fn every_checkpoint_carries_provenance() {
    let packet = packet();
    for cp in &packet.checkpoints {
        assert!(
            cp.provenance_complete(),
            "checkpoint {} lacks routing, archetype, or bundle provenance",
            cp.checkpoint_id
        );
    }
}

#[test]
fn export_projection_reflects_checkpoints() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.checkpoints.len(), packet.checkpoints.len());
    assert!(projection.all_checkpoints_consistent);
    assert_eq!(
        projection.checkpoints_with_local_safe_work,
        packet.summary.checkpoints_with_local_safe_work
    );
    for (row, cp) in projection.checkpoints.iter().zip(&packet.checkpoints) {
        assert_eq!(row.checkpoint_id, cp.checkpoint_id);
        assert_eq!(row.admission_class, cp.admission_class);
        assert_eq!(row.first_useful_work_route, cp.first_useful_work_route);
    }
}

#[test]
fn all_checkpoints_consistent_holds() {
    assert!(packet().all_checkpoints_consistent());
}

#[test]
fn validate_flags_probable_presented_as_certified() {
    let mut packet = packet();
    let cp = packet
        .checkpoints
        .iter_mut()
        .find(|c| c.admission_class == AdmissionClass::Probable)
        .expect("a probable checkpoint exists");
    cp.presented_as_certified_support = true;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AdmissionAndRoutingViolation::CertifiedPresentationMismatch { .. }
    )));
}

#[test]
fn validate_flags_an_auto_run_setup_item() {
    let mut packet = packet();
    let cp = packet
        .checkpoints
        .iter_mut()
        .find(|c| !c.setup_items.is_empty())
        .expect("a checkpoint with setup items exists");
    cp.setup_items[0].auto_runs = true;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5AdmissionAndRoutingViolation::AutoRunSetupItem { .. })));
}

#[test]
fn validate_flags_admission_outranking_confidence() {
    let mut packet = packet();
    // Force a probable checkpoint to claim certified admission without certified confidence.
    let cp = packet
        .checkpoints
        .iter_mut()
        .find(|c| c.archetype_confidence == ArchetypeConfidence::Probable)
        .expect("a probable-confidence checkpoint exists");
    cp.admission_class = AdmissionClass::Certified;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AdmissionAndRoutingViolation::AdmissionExceedsConfidence { .. }
    )));
}

#[test]
fn validate_flags_a_dropped_local_safe_path() {
    let mut packet = packet();
    // Drop local-safe work on a non-restricted checkpoint — the spec keeps minimal work alive.
    let cp = packet
        .checkpoints
        .iter_mut()
        .find(|c| c.admission_class != AdmissionClass::Restricted)
        .expect("a non-restricted checkpoint exists");
    cp.local_safe_work_available = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5AdmissionAndRoutingViolation::LocalSafeMismatch { .. })));
}

#[test]
fn validate_flags_missing_wedge_coverage() {
    let mut packet = packet();
    packet
        .checkpoints
        .retain(|c| c.wedge != M5Wedge::NotebookWorkspace);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5AdmissionAndRoutingViolation::MissingWedgeCoverage {
            wedge: M5Wedge::NotebookWorkspace
        }
    )));
}

#[test]
fn constants_point_at_checked_in_paths() {
    assert_eq!(
        M5_ADMISSION_AND_ROUTING_PATH,
        "artifacts/workspace/m5/m5-admission-and-routing.json"
    );
    assert_eq!(
        M5_ADMISSION_AND_ROUTING_SCHEMA_REF,
        "schemas/workspace/m5-admission-and-routing.schema.json"
    );
    assert_eq!(
        M5_ADMISSION_AND_ROUTING_DOC_REF,
        "docs/workspace/m5/m5-admission-and-routing.md"
    );
}
