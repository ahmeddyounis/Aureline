//! Help-surface projection for M5 filesystem continuity certification.
//!
//! This module is intentionally thin: help surfaces should quote the canonical
//! support-owned certification packet rather than restating filesystem, save,
//! lineage, or deferred-intent posture in a parallel vocabulary.

use aureline_support::{
    seeded_m5_fs_mutation_lineage_certification_packet, M5FsMutationLineageHelpSurfaceProjection,
};

/// Builds the seeded help-surface projection for M5 filesystem continuity truth.
pub fn seeded_filesystem_continuity_help_projection() -> M5FsMutationLineageHelpSurfaceProjection {
    seeded_m5_fs_mutation_lineage_certification_packet().help_surface_projection()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_projection_reuses_canonical_packet_id() {
        let packet = seeded_m5_fs_mutation_lineage_certification_packet();
        let projection = seeded_filesystem_continuity_help_projection();
        assert_eq!(projection.packet_id, packet.packet_id);
        assert_eq!(
            projection.certification_row_count,
            packet.certification_rows.len()
        );
    }
}
