//! Diagnostics-export projection for M5 filesystem continuity certification.
//!
//! Diagnostics export should preserve the certification row id, publication
//! state, connectivity posture, and narrow tokens from the canonical packet so
//! support and release reviewers can reconstruct the same continuity story.

use aureline_support::{
    seeded_m5_fs_mutation_lineage_certification_packet,
    M5FsMutationLineageDiagnosticsExportProjection,
};

/// Builds the seeded diagnostics-export projection for M5 filesystem continuity truth.
pub fn seeded_filesystem_continuity_diagnostics_projection(
) -> M5FsMutationLineageDiagnosticsExportProjection {
    seeded_m5_fs_mutation_lineage_certification_packet().diagnostics_export_projection()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostics_projection_reuses_canonical_packet_id() {
        let packet = seeded_m5_fs_mutation_lineage_certification_packet();
        let projection = seeded_filesystem_continuity_diagnostics_projection();
        assert_eq!(projection.packet_id, packet.packet_id);
        assert_eq!(
            projection.certification_row_count,
            packet.certification_rows.len()
        );
    }
}
