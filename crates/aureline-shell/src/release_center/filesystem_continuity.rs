//! Release-center projection for M5 filesystem continuity certification.
//!
//! Release-center and shiproom truth should consume the same support-owned
//! certification packet so generated, imported, and deferred-write rows cannot
//! accidentally publish broader filesystem continuity claims.

use aureline_support::{
    seeded_m5_fs_mutation_lineage_certification_packet, M5FsMutationLineageReleaseCenterProjection,
};

/// Builds the seeded release-center projection for M5 filesystem continuity truth.
pub fn seeded_filesystem_continuity_release_center_projection(
) -> M5FsMutationLineageReleaseCenterProjection {
    seeded_m5_fs_mutation_lineage_certification_packet().release_center_projection()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_center_projection_reuses_canonical_packet_id() {
        let packet = seeded_m5_fs_mutation_lineage_certification_packet();
        let projection = seeded_filesystem_continuity_release_center_projection();
        assert_eq!(projection.packet_id, packet.packet_id);
        assert_eq!(
            projection.certification_row_count,
            packet.certification_rows.len()
        );
    }
}
