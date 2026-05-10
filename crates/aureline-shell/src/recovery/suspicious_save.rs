//! Suspicious-content annotation for save-review surfaces.
//!
//! Save review is the protected pre-commit surface that lands the local
//! buffer. Running the shared suspicious-content detector over the local
//! bytes here gives the user a representation-labeled copy/export
//! affordance and a safe-mode hand-off point before the bytes are written.
//!
//! This module is intentionally narrow: it produces shared detector records
//! plus a small projection the save-review sheet can render. Cross-surface
//! safe-preview / copy / export depth lands in a later lane.

use serde::{Deserialize, Serialize};

use aureline_content_safety::{
    detect_suspicious_content, escape_for_safe_inspection, DetectorOutcomeClass,
    RepresentationTransferRecord, SurfaceFamily, SuspiciousContentCaseRecord, TrustClass,
};

use super::safe_mode::SAFE_MODE_ENTER_COMMAND_ID;

/// Schema version for [`SaveReviewSuspiciousContentAnnotation`].
pub const SAVE_REVIEW_SUSPICIOUS_CONTENT_SCHEMA_VERSION: u32 = 1;

/// Projection added to a save-review sheet when the local bytes contain
/// suspicious content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveReviewSuspiciousContentAnnotation {
    pub record_kind: String,
    pub schema_version: u32,
    pub case_id: String,
    pub detector_outcome: String,
    pub finding_count: usize,
    pub case_record: SuspiciousContentCaseRecord,
    pub representation_transfers: Vec<RepresentationTransferRecord>,
    pub escaped_preview_lines: Vec<String>,
    pub safe_mode_command_id: String,
    pub warning_lines: Vec<String>,
}

impl SaveReviewSuspiciousContentAnnotation {
    /// Concise status string suitable for status surfaces and command-runtime notes.
    pub fn status_line(&self) -> String {
        format!(
            "save_review_suspicious_content: case={case}; outcome={outcome}; findings={count}; \
             safe_mode_offered=true; safe_mode_command={cmd}",
            case = self.case_id,
            outcome = self.detector_outcome,
            count = self.finding_count,
            cmd = self.safe_mode_command_id,
        )
    }
}

/// Run the shared suspicious-content detector over the supplied save-review
/// payload (UTF-8 text). Returns `None` when the bytes are clean or the
/// content is not UTF-8 text.
///
/// The annotation reuses the schema-aligned record kinds emitted by
/// `aureline-content-safety` so any later preview surface can consume the
/// same evidence without re-parsing the raw content.
pub fn annotate_save_review_with_suspicious_content(
    case_id: &str,
    local_content: &[u8],
    annotation_mode: &str,
    location_kind: &str,
    max_preview_lines: usize,
) -> Option<SaveReviewSuspiciousContentAnnotation> {
    let text = std::str::from_utf8(local_content).ok()?;
    let detection = detect_suspicious_content(text);
    if detection.is_clean() {
        return None;
    }

    let case_record = detection.materialize_case_record(
        case_id,
        SurfaceFamily::ReviewSurface,
        annotation_mode,
        location_kind,
    )?;
    let representation_transfers = detection.materialize_raw_and_escaped_transfers(
        case_id,
        SurfaceFamily::ReviewSurface,
        TrustClass::RawText,
    );

    let escaped_text = escape_for_safe_inspection(text);
    let escaped_preview_lines = escaped_text
        .lines()
        .take(max_preview_lines.max(1))
        .map(|line| line.to_string())
        .collect();

    let warning_lines = vec![
        format!(
            "Suspicious content detected: {} finding(s). Bytes preserved exactly.",
            detection.findings.len()
        ),
        "Use Copy raw or Copy escaped to preserve representation labelling.".to_string(),
        format!(
            "If you are unsure, enter safe mode ({}) and review the finding before saving.",
            SAFE_MODE_ENTER_COMMAND_ID
        ),
    ];

    let detector_outcome = match detection.outcome {
        DetectorOutcomeClass::Allow => "allow",
        DetectorOutcomeClass::Sanitize => "sanitize",
        DetectorOutcomeClass::Isolate => "isolate",
        DetectorOutcomeClass::Block => "block",
        DetectorOutcomeClass::RouteToSystemBrowser => "route_to_system_browser",
    };

    Some(SaveReviewSuspiciousContentAnnotation {
        record_kind: "save_review_suspicious_content_annotation".to_string(),
        schema_version: SAVE_REVIEW_SUSPICIOUS_CONTENT_SCHEMA_VERSION,
        case_id: case_id.to_string(),
        detector_outcome: detector_outcome.to_string(),
        finding_count: detection.findings.len(),
        case_record,
        representation_transfers,
        escaped_preview_lines,
        safe_mode_command_id: SAFE_MODE_ENTER_COMMAND_ID.to_string(),
        warning_lines,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clean_text_yields_no_annotation() {
        let annotation = annotate_save_review_with_suspicious_content(
            "case:save:plain",
            b"plain ASCII content\n",
            "standard_inline",
            "review_hunk",
            8,
        );
        assert!(annotation.is_none());
    }

    #[test]
    fn bidi_in_local_content_emits_annotation_with_safe_mode_offer() {
        let local = "let \u{202E}admin = 1;".as_bytes();
        let annotation = annotate_save_review_with_suspicious_content(
            "case:save:bidi",
            local,
            "standard_inline",
            "review_hunk",
            8,
        )
        .expect("annotation must be present for bidi finding");

        assert_eq!(annotation.detector_outcome, "sanitize");
        assert!(annotation.finding_count >= 1);
        assert!(annotation
            .case_record
            .findings
            .iter()
            .any(|f| f.content_class == "bidi_control"));

        // Both representation transfers must be present and pair each other.
        let copy_raw = annotation
            .representation_transfers
            .iter()
            .find(|t| t.action_id == "copy_raw")
            .unwrap();
        let copy_escaped = annotation
            .representation_transfers
            .iter()
            .find(|t| t.action_id == "copy_escaped")
            .unwrap();
        assert!(copy_raw.must_offer_also.iter().any(|s| s == "copy_escaped"));
        assert!(copy_escaped.must_offer_also.iter().any(|s| s == "copy_raw"));

        // Escaped preview must escape the bidi codepoint.
        assert!(annotation
            .escaped_preview_lines
            .iter()
            .any(|line| line.contains("\\u{202E}")));

        // Status line and warnings include the safe-mode command id.
        let status = annotation.status_line();
        assert!(status.contains("safe_mode_offered=true"));
        assert!(annotation
            .warning_lines
            .iter()
            .any(|line| line.contains(SAFE_MODE_ENTER_COMMAND_ID)));
    }

    #[test]
    fn invalid_utf8_returns_no_annotation_without_panicking() {
        let bytes = [0xFFu8, 0xFE, 0xFD];
        assert!(annotate_save_review_with_suspicious_content(
            "case:save:binary",
            &bytes,
            "standard_inline",
            "review_hunk",
            8
        )
        .is_none());
    }
}
