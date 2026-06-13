use super::*;

#[test]
fn canonical_packet_validates_and_is_export_safe() {
    let packet = seeded_m5_fs_mutation_lineage_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_export_safe());
}

#[test]
fn canonical_packet_covers_every_matrix_row() {
    let packet = seeded_m5_fs_mutation_lineage_certification_packet();
    let matrix = seeded_filesystem_mutation_lineage_matrix_packet();
    for row in &matrix.rows {
        assert!(
            packet
                .certification_rows
                .iter()
                .any(|candidate| candidate.surface_row_id == row.row_id),
            "missing certification row for {}",
            row.row_id
        );
    }
}

#[test]
fn state_distribution_stays_explicit() {
    let packet = seeded_m5_fs_mutation_lineage_certification_packet();
    assert_eq!(count_rows(&packet, CertificationStateClass::Qualified), 2);
    assert_eq!(count_rows(&packet, CertificationStateClass::Limited), 2);
    assert_eq!(count_rows(&packet, CertificationStateClass::Stale), 4);
    assert_eq!(
        count_rows(&packet, CertificationStateClass::ReconcileRequired),
        3
    );
}

#[test]
fn support_bundle_projection_reuses_the_same_packet() {
    let packet = seeded_m5_fs_mutation_lineage_certification_packet();
    let projection = packet
        .support_bundle_projection()
        .expect("support projection");
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(projection.rows.len(), packet.certification_rows.len());
    assert!(projection.raw_payload_excluded);
    assert!(projection.ambient_authority_excluded);
}

#[test]
fn missing_recovery_variant_narrows_previously_qualified_rows() {
    let packet = seeded_missing_recovery_linkage_m5_fs_mutation_lineage_certification_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    for surface_row_id in ["notebook_document", "request_workspace_document"] {
        let row = packet
            .certification_rows
            .iter()
            .find(|row| row.surface_row_id == surface_row_id)
            .expect("row exists");
        assert_eq!(row.published_state, CertificationStateClass::Limited);
        assert!(row
            .stale_or_narrow_tokens
            .iter()
            .any(|token| token == "recovery_mapping_missing"));
    }
}
