use super::*;

fn packet() -> M5SerializationMatrix {
    current_m5_serialization_matrix().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_SERIALIZATION_MATRIX_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_SERIALIZATION_MATRIX_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_artifact_class_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.rows.len(), RememberedArtifactClass::ALL.len());
    for &class in &RememberedArtifactClass::ALL {
        assert!(
            packet.row(class).is_some(),
            "missing row for artifact class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_restorable_surface_has_exactly_one_row() {
    let packet = packet();
    assert_eq!(packet.surface_rows.len(), RestorableSurface::ALL.len());
    for &surface in &RestorableSurface::ALL {
        assert!(
            packet.surface_row(surface).is_some(),
            "missing row for surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_row_is_gate_consistent() {
    let packet = packet();
    assert!(packet.all_rows_gate_consistent());
    for row in &packet.rows {
        assert_eq!(
            row.published_fidelity,
            row.achieved_fidelity(),
            "{}",
            row.row_id
        );
        assert_eq!(
            row.downgrade_reasons,
            row.computed_downgrade_reasons(),
            "{}",
            row.row_id
        );
        assert_eq!(
            row.recovery_path,
            row.computed_recovery_path(),
            "{}",
            row.row_id
        );
    }
}

#[test]
fn published_fidelity_never_exceeds_declared_and_is_supported() {
    let packet = packet();
    for row in &packet.rows {
        assert!(
            row.published_fidelity.rank() <= row.declared_max_fidelity.rank(),
            "{} publishes above its declared maximum",
            row.row_id
        );
        assert!(
            row.supported_fidelity_classes
                .contains(&row.published_fidelity),
            "{} publishes an unsupported fidelity",
            row.row_id
        );
    }
}

#[test]
fn no_row_silently_deletes_layout() {
    let packet = packet();
    for row in &packet.rows {
        assert!(
            row.missing_dependency_behavior.preserves_slot(),
            "{} would silently delete layout",
            row.row_id
        );
    }
}

#[test]
fn exportable_rows_exclude_secrets_authority_and_machine_local() {
    let packet = packet();
    for row in &packet.rows {
        if row.exportable {
            assert!(
                row.ownership.exportable_into_portable_package(),
                "{} is exportable but not portable",
                row.row_id
            );
            for excl in RedactionExclusion::ALL {
                assert!(
                    row.redaction_exclusions.contains(&excl),
                    "{} exportable but missing {}",
                    row.row_id,
                    excl.as_str()
                );
            }
        }
    }
}

#[test]
fn machine_local_rows_are_never_exportable() {
    let packet = packet();
    for row in &packet.rows {
        if row.ownership == OwnershipClass::MachineLocal {
            assert!(
                !row.exportable,
                "{} is machine-local but exportable",
                row.row_id
            );
        }
    }
}

#[test]
fn surfaces_never_outclaim_the_classes_they_persist() {
    let packet = packet();
    for surface in &packet.surface_rows {
        let best_fidelity = surface
            .persisted_artifact_classes
            .iter()
            .filter_map(|c| packet.row(*c))
            .map(|r| r.declared_max_fidelity.rank())
            .max()
            .expect("surface persists at least one class");
        assert!(
            surface.max_supported_fidelity.rank() <= best_fidelity,
            "{} claims more fidelity than its classes support",
            surface.row_id
        );
        let best_portability = surface
            .persisted_artifact_classes
            .iter()
            .filter_map(|c| packet.row(*c))
            .map(|r| r.ownership.portability_rank())
            .max()
            .expect("surface persists at least one class");
        assert!(
            surface.portability.portability_rank() <= best_portability,
            "{} claims more portability than its classes support",
            surface.row_id
        );
    }
}

#[test]
fn every_continuity_surface_is_crosslinked() {
    let packet = packet();
    for surface in ContinuitySurface::ALL {
        assert!(
            packet.has_crosslink_for(surface),
            "missing cross-link for {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_binds_and_narrows() {
    let packet = packet();
    for surface in MatrixConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding for {}",
            surface.as_str()
        );
    }
}

#[test]
fn export_projection_is_consistent() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.rows.len(), packet.rows.len());
    assert!(projection.all_rows_gate_consistent);
    assert_eq!(
        projection.exact_count + projection.narrowed_count,
        packet.rows.len()
    );
    assert_eq!(projection.exact_count, packet.summary.exact_restore_rows);
    assert_eq!(
        projection.manual_review_count,
        packet.summary.manual_review_rows
    );
}

#[test]
fn support_export_round_trips_and_is_safe() {
    let packet = packet();
    let export = packet.support_export("export:serialization-matrix", "2026-06-16");
    assert!(export.is_export_safe());
    assert_eq!(export.matrix_packet_id_ref, packet.packet_id);
    let encoded = serde_json::to_string(&export).expect("serializes");
    let decoded: M5SerializationMatrixSupportExport =
        serde_json::from_str(&encoded).expect("round-trips");
    assert_eq!(decoded.matrix, packet);
}

#[test]
fn matrix_covers_every_fidelity_class_in_published_rows() {
    // The matrix is not a blanket "remembers everything" badge: every restore-fidelity class is
    // exercised by a real row, proving the gate narrows as well as certifies.
    let packet = packet();
    let published: BTreeSet<RestoreFidelityClass> =
        packet.rows.iter().map(|r| r.published_fidelity).collect();
    for class in RestoreFidelityClass::ALL {
        assert!(
            published.contains(&class),
            "no row publishes {}",
            class.as_str()
        );
    }
}

#[test]
fn matrix_covers_every_ownership_class() {
    let packet = packet();
    let owners: BTreeSet<OwnershipClass> = packet.rows.iter().map(|r| r.ownership).collect();
    for class in OwnershipClass::ALL {
        assert!(owners.contains(&class), "no row owns {}", class.as_str());
    }
}

// --- Synthetic gate drills: exercise condition variants the canonical rows do not, plus the
// fail-closed rejections, without touching the checked-in packet. ---

fn exact_template() -> ArtifactClassRow {
    ArtifactClassRow {
        row_id: "drill:row".to_owned(),
        artifact_class: RememberedArtifactClass::WorkspaceAuthorityCheckpoint,
        owner: "drill".to_owned(),
        persisted_scope: "drill".to_owned(),
        ownership: OwnershipClass::Local,
        exportable: false,
        redaction_exclusions: RedactionExclusion::BASELINE.to_vec(),
        supported_fidelity_classes: RestoreFidelityClass::ALL.to_vec(),
        declared_max_fidelity: RestoreFidelityClass::ExactRestore,
        schema_condition: SchemaCondition::SchemaMatch,
        dependency_condition: DependencyCondition::DependenciesPresent,
        topology_condition: TopologyCondition::TopologyIdentical,
        evidence_freshness: EvidenceFreshness::Current,
        missing_dependency_behavior: MissingDependencyBehavior::PlaceholderSlotPreserved,
        published_fidelity: RestoreFidelityClass::ExactRestore,
        downgrade_reasons: Vec::new(),
        recovery_path: RecoveryPath::NoneNeeded,
        continuity_surfaces: vec![ContinuitySurface::CrashRecovery],
        caveats: Vec::new(),
        stale_or_missing_fields: Vec::new(),
        schema_ref: "schemas/x".to_owned(),
        evidence_ref: "evidence:x".to_owned(),
        scope_snapshot_ref: "scope:x".to_owned(),
        note: "drill".to_owned(),
    }
}

#[test]
fn dependency_root_missing_forces_manual_review() {
    let mut row = exact_template();
    row.dependency_condition = DependencyCondition::DependencyRootMissing;
    assert_eq!(row.achieved_fidelity(), RestoreFidelityClass::ManualReview);
    assert_eq!(row.computed_recovery_path(), RecoveryPath::ManualReview);
    assert_eq!(
        row.computed_downgrade_reasons(),
        vec![DowngradeReason::DependencyMissing]
    );
}

#[test]
fn topology_incompatible_caps_layout_only() {
    let mut row = exact_template();
    row.topology_condition = TopologyCondition::TopologyIncompatible;
    assert_eq!(row.achieved_fidelity(), RestoreFidelityClass::LayoutOnly);
    assert_eq!(row.computed_recovery_path(), RecoveryPath::ReopenAsContext);
}

#[test]
fn missing_evidence_forces_manual_review() {
    let mut row = exact_template();
    row.evidence_freshness = EvidenceFreshness::Missing;
    assert_eq!(row.achieved_fidelity(), RestoreFidelityClass::ManualReview);
    assert_eq!(row.computed_recovery_path(), RecoveryPath::ManualReview);
}

#[test]
fn silent_delete_behavior_is_rejected() {
    let packet = packet();
    let mut broken = packet.clone();
    broken.rows[0].missing_dependency_behavior = MissingDependencyBehavior::SilentDelete;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5SerializationMatrixViolation::SilentLayoutDelete { .. })));
}

#[test]
fn machine_local_export_is_rejected() {
    let packet = packet();
    let mut broken = packet.clone();
    // window_topology_snapshot is machine-local; marking it exportable must be rejected.
    let idx = broken
        .rows
        .iter()
        .position(|r| r.ownership == OwnershipClass::MachineLocal)
        .expect("a machine-local row exists");
    broken.rows[idx].exportable = true;
    broken.rows[idx].redaction_exclusions = RedactionExclusion::ALL.to_vec();
    let violations = broken.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5SerializationMatrixViolation::NonPortableExport { .. })));
}

#[test]
fn overstated_fidelity_is_rejected() {
    let packet = packet();
    let mut broken = packet.clone();
    // Claim an exact restore on a row whose dependency is missing.
    let idx = broken
        .rows
        .iter()
        .position(|r| r.artifact_class == RememberedArtifactClass::PlaceholderCard)
        .expect("placeholder row exists");
    broken.rows[idx].published_fidelity = RestoreFidelityClass::ExactRestore;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5SerializationMatrixViolation::OverstatedFidelity { .. })));
}

#[test]
fn exportable_row_missing_exclusion_is_rejected() {
    let packet = packet();
    let mut broken = packet.clone();
    let idx = broken
        .rows
        .iter()
        .position(|r| r.exportable)
        .expect("an exportable row exists");
    broken.rows[idx]
        .redaction_exclusions
        .retain(|e| *e != RedactionExclusion::ExcludesMachineLocalAnchors);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5SerializationMatrixViolation::MissingRedactionExclusion { .. }
    )));
}
