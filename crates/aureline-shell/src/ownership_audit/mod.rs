//! Shell-side consumer for the desktop-entry ownership audit packet.
//!
//! The shell module reads the protected ownership-audit fixture
//! exported by [`aureline_install::ownership_audit`] and produces the
//! same surface and support-export projections the install crate
//! validates. It is the first consuming surface for the audit lane:
//! the live shell, the headless inspector binary
//! (`aureline_shell_ownership_audit`), the support-export wrapper, and
//! the integration test all read this single fixture so About,
//! install review, diagnostics, CLI, and support-export rows project
//! the same ownership truth.

use std::path::{Path, PathBuf};

use aureline_install::ownership_audit::{
    OwnershipAuditPacket, OwnershipAuditSupportExport, OwnershipAuditSurfaceProjection,
    OwnershipAuditValidationReport,
};

/// Fixture path relative to the workspace root.
pub const OWNERSHIP_AUDIT_FIXTURE_PATH: &str =
    "fixtures/install/ownership_audit/ownership_audit_packet.json";

/// Reads the bundled ownership-audit packet from the workspace fixture.
///
/// Returns the parsed packet or a typed error string for the headless
/// inspector binary to report verbatim.
pub fn load_seeded_ownership_audit_packet() -> Result<OwnershipAuditPacket, String> {
    let path = workspace_fixture_path();
    let bytes = std::fs::read(&path).map_err(|err| {
        format!(
            "read ownership audit fixture from {} failed: {err}",
            path.display()
        )
    })?;
    serde_json::from_slice::<OwnershipAuditPacket>(&bytes)
        .map_err(|err| format!("parse ownership audit fixture failed: {err}"))
}

/// Returns the seeded packet's surface projection.
pub fn seeded_ownership_audit_surface_projection(
) -> Result<OwnershipAuditSurfaceProjection, String> {
    Ok(load_seeded_ownership_audit_packet()?.surface_projection())
}

/// Returns the seeded packet's metadata-safe support export wrapper.
pub fn seeded_ownership_audit_support_export() -> Result<OwnershipAuditSupportExport, String> {
    Ok(load_seeded_ownership_audit_packet()?.support_export_projection())
}

/// Validates the seeded packet and returns the report.
pub fn validate_seeded_ownership_audit_packet() -> Result<OwnershipAuditValidationReport, String> {
    Ok(load_seeded_ownership_audit_packet()?.validate())
}

fn workspace_fixture_path() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .join("../..")
        .join(OWNERSHIP_AUDIT_FIXTURE_PATH)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates() {
        let report = validate_seeded_ownership_audit_packet().expect("packet loads");
        assert!(
            report.passed,
            "seeded ownership audit packet must validate: {:#?}",
            report.findings
        );
    }

    #[test]
    fn seeded_surface_and_support_projections_round_trip() {
        let surface = seeded_ownership_audit_surface_projection().expect("surface");
        let export = seeded_ownership_audit_support_export().expect("export");
        assert_eq!(surface.rows.len(), export.projection.rows.len());
        assert_eq!(surface.packet_id, export.packet_id);
        assert_eq!(export.redaction_class, "metadata_only_no_paths_or_secrets");
    }
}
