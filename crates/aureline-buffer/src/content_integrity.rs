//! Editor-buffer content-integrity projections.
//!
//! The buffer crate is the editor entry point for bytes opened into, or saved
//! from, source buffers. This module consumes the shared content-safety
//! warning projection so editor surfaces carry the same warning records as
//! diff, search, preview, and package surfaces.

use aureline_content_safety::{
    project_content_integrity_warnings, ContentIntegritySurfaceKind, ContentIntegrityWarningRecord,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`EditorContentIntegrityProjection`].
pub const EDITOR_CONTENT_INTEGRITY_PROJECTION_RECORD_KIND: &str =
    "editor_content_integrity_projection";

/// Schema version for [`EditorContentIntegrityProjection`].
pub const EDITOR_CONTENT_INTEGRITY_PROJECTION_SCHEMA_VERSION: u32 = 1;

/// Editor lifecycle event that requested content-integrity warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorContentIntegrityEvent {
    /// Bytes were opened into an editor buffer.
    Open,
    /// Bytes are being reviewed before or during save.
    Save,
}

impl EditorContentIntegrityEvent {
    /// Stable token used in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Save => "save",
        }
    }
}

/// Editor projection carrying shared content-integrity warnings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorContentIntegrityProjection {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Case, open, save, or session id for this detector run.
    pub case_id: String,
    /// Opaque editor buffer or save-target subject ref.
    pub subject_ref: String,
    /// Editor lifecycle event that requested the projection.
    pub event: EditorContentIntegrityEvent,
    /// Stable token for [`Self::event`].
    pub event_token: String,
    /// Shared warning records emitted by `aureline-content-safety`.
    pub warnings: Vec<ContentIntegrityWarningRecord>,
}

impl EditorContentIntegrityProjection {
    /// Returns true when at least one shared warning is present.
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Projects editor open/save content-integrity warnings from UTF-8 bytes.
///
/// Non-UTF-8 bytes return an empty warning set; decode-recovery surfaces carry
/// their own source-fidelity records and should not be treated as clean text.
pub fn project_editor_content_integrity(
    case_id: &str,
    subject_ref: &str,
    event: EditorContentIntegrityEvent,
    content: &[u8],
) -> EditorContentIntegrityProjection {
    let warnings = std::str::from_utf8(content)
        .map(|text| {
            project_content_integrity_warnings(
                case_id,
                ContentIntegritySurfaceKind::Editor,
                subject_ref,
                text,
            )
        })
        .unwrap_or_default();

    EditorContentIntegrityProjection {
        record_kind: EDITOR_CONTENT_INTEGRITY_PROJECTION_RECORD_KIND.to_string(),
        schema_version: EDITOR_CONTENT_INTEGRITY_PROJECTION_SCHEMA_VERSION,
        case_id: case_id.to_string(),
        subject_ref: subject_ref.to_string(),
        event,
        event_token: event.as_str().to_string(),
        warnings,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_content_safety::CONTENT_INTEGRITY_WARNING_RECORD_KIND;

    #[test]
    fn editor_projection_emits_shared_warning_records() {
        let projection = project_editor_content_integrity(
            "case:editor:content-integrity",
            "buffer:src/lib.rs",
            EditorContentIntegrityEvent::Open,
            "let admin\u{202E} = user\u{200D}name;".as_bytes(),
        );

        assert!(projection.has_warnings());
        assert!(projection.warnings.iter().all(|warning| {
            warning.record_kind == CONTENT_INTEGRITY_WARNING_RECORD_KIND
                && warning.surface_token == "editor"
        }));
    }
}
