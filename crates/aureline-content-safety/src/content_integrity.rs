//! Shared content-integrity warning records for first-party surfaces.
//!
//! The detector owns suspicious-source analysis. This module turns detector
//! findings into one record shape that editor, diff, search, preview, and
//! package surfaces can carry without re-parsing text or inventing local
//! warning vocabularies.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    detect_suspicious_content, BodyPosture, RepresentationActionId, RepresentationClass,
    SuspiciousContentClass, SuspiciousContentDetection, SuspiciousFinding,
};

/// Stable record-kind tag carried by every content-integrity warning.
pub const CONTENT_INTEGRITY_WARNING_RECORD_KIND: &str = "content_integrity_warning_record";

/// Schema version for [`ContentIntegrityWarningRecord`].
pub const CONTENT_INTEGRITY_WARNING_SCHEMA_VERSION: u32 = 1;

/// Product surfaces that consume shared content-integrity warnings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentIntegritySurfaceKind {
    /// Live editor buffer or editor save/open path.
    Editor,
    /// Diff, merge, save-review, or review-preview hunk.
    Diff,
    /// Search result row, quick-open row, or snippet.
    Search,
    /// Safe preview card or rich preview shell.
    Preview,
    /// Package, marketplace, or install-review surface.
    Package,
}

impl ContentIntegritySurfaceKind {
    /// Stable token used in warning records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Diff => "diff",
            Self::Search => "search",
            Self::Preview => "preview",
            Self::Package => "package",
        }
    }
}

/// Surfaces named by the shared content-integrity coverage contract.
pub const CONTENT_INTEGRITY_REQUIRED_SURFACES: [ContentIntegritySurfaceKind; 5] = [
    ContentIntegritySurfaceKind::Editor,
    ContentIntegritySurfaceKind::Diff,
    ContentIntegritySurfaceKind::Search,
    ContentIntegritySurfaceKind::Preview,
    ContentIntegritySurfaceKind::Package,
];

/// One shared warning projected from a suspicious-content detector finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentIntegrityWarningRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version for the warning shape.
    pub schema_version: u32,
    /// Case, packet, or session id that grouped this detector run.
    pub case_id: String,
    /// Surface enum that carries the warning.
    pub surface: ContentIntegritySurfaceKind,
    /// Stable surface token for compact consumers.
    pub surface_token: String,
    /// Opaque subject, row, preview, or package ref.
    pub subject_ref: String,
    /// Detector finding id projected by the shared detector.
    pub detector_finding_id: String,
    /// Suspicious-content class token from the shared vocabulary.
    pub content_class_token: String,
    /// Short label suitable for chips or review rows.
    pub warning_label: String,
    /// Continuity ref used to join the same warning across surfaces.
    pub continuity_ref: String,
    /// Byte offset of the finding in the inspected source text.
    pub byte_offset: usize,
    /// Character offset of the finding in the inspected source text.
    pub char_offset: usize,
    /// Character length of the finding.
    pub length_chars: usize,
    /// Codepoints matched by the detector.
    pub matched_codepoints: Vec<u32>,
    /// Exact source snippet when source text was available.
    pub raw_snippet: String,
    /// Safe-inspection snippet with deceptive codepoints escaped.
    pub escaped_snippet: String,
    /// Reveal actions that must remain reachable near the warning.
    pub reveal_action_ids: Vec<String>,
    /// Copy/export actions that preserve representation labels.
    pub transfer_action_ids: Vec<String>,
    /// Representation classes exposed by the transfer actions.
    pub transfer_representation_classes: Vec<String>,
    /// Body postures exposed by the transfer actions.
    pub transfer_body_postures: Vec<String>,
}

impl ContentIntegrityWarningRecord {
    /// Returns true when this warning uses the shared warning record kind.
    pub fn uses_shared_record_kind(&self) -> bool {
        self.record_kind == CONTENT_INTEGRITY_WARNING_RECORD_KIND
    }
}

/// Runs the shared detector and projects warnings for one product surface.
pub fn project_content_integrity_warnings(
    case_id: &str,
    surface: ContentIntegritySurfaceKind,
    subject_ref: &str,
    content: &str,
) -> Vec<ContentIntegrityWarningRecord> {
    let detection = detect_suspicious_content(content);
    project_content_integrity_warnings_from_detection(
        case_id,
        surface,
        subject_ref,
        &detection,
        Some(content),
    )
}

/// Projects warnings from an existing detector outcome.
///
/// Surfaces that already ran the detector upstream can call this to avoid a
/// second detector pass while still emitting the same warning record shape.
pub fn project_content_integrity_warnings_from_detection(
    case_id: &str,
    surface: ContentIntegritySurfaceKind,
    subject_ref: &str,
    detection: &SuspiciousContentDetection,
    source_text: Option<&str>,
) -> Vec<ContentIntegrityWarningRecord> {
    detection
        .findings
        .iter()
        .map(|finding| warning_from_finding(case_id, surface, subject_ref, finding, source_text))
        .collect()
}

/// Returns true when all required surfaces appear at least once.
pub fn warnings_cover_required_surfaces(warnings: &[ContentIntegrityWarningRecord]) -> bool {
    let present = warnings
        .iter()
        .map(|warning| warning.surface)
        .collect::<BTreeSet<_>>();
    CONTENT_INTEGRITY_REQUIRED_SURFACES
        .iter()
        .all(|surface| present.contains(surface))
}

fn warning_from_finding(
    case_id: &str,
    surface: ContentIntegritySurfaceKind,
    subject_ref: &str,
    finding: &SuspiciousFinding,
    source_text: Option<&str>,
) -> ContentIntegrityWarningRecord {
    let raw_snippet = raw_snippet_for_finding(source_text, finding);
    let escaped_snippet = escaped_snippet_for_finding(&raw_snippet, finding);
    let surface_token = surface.as_str();

    ContentIntegrityWarningRecord {
        record_kind: CONTENT_INTEGRITY_WARNING_RECORD_KIND.to_string(),
        schema_version: CONTENT_INTEGRITY_WARNING_SCHEMA_VERSION,
        case_id: case_id.to_string(),
        surface,
        surface_token: surface_token.to_string(),
        subject_ref: subject_ref.to_string(),
        detector_finding_id: finding.finding_id.clone(),
        content_class_token: finding.class.as_str().to_string(),
        warning_label: warning_label_for(finding.class).to_string(),
        continuity_ref: format!(
            "content-integrity:{case_id}:{surface_token}:{}",
            finding.finding_id
        ),
        byte_offset: finding.byte_offset,
        char_offset: finding.char_offset,
        length_chars: finding.length_chars,
        matched_codepoints: finding.matched_codepoints.clone(),
        raw_snippet,
        escaped_snippet,
        reveal_action_ids: vec![
            "reveal_raw_source".to_string(),
            "reveal_escaped_source".to_string(),
            "inspect_codepoints".to_string(),
        ],
        transfer_action_ids: vec![
            RepresentationActionId::CopyRaw.as_str().to_string(),
            RepresentationActionId::CopyEscaped.as_str().to_string(),
            RepresentationActionId::ExportSanitizedSnapshot
                .as_str()
                .to_string(),
        ],
        transfer_representation_classes: vec![
            RepresentationClass::Raw.as_str().to_string(),
            RepresentationClass::Escaped.as_str().to_string(),
            RepresentationClass::Sanitized.as_str().to_string(),
        ],
        transfer_body_postures: vec![
            BodyPosture::ExactSourceBytes.as_str().to_string(),
            BodyPosture::EscapedSourceText.as_str().to_string(),
            BodyPosture::SanitizedStaticSnapshot.as_str().to_string(),
        ],
    }
}

fn raw_snippet_for_finding(source_text: Option<&str>, finding: &SuspiciousFinding) -> String {
    if let Some(text) = source_text {
        return text
            .chars()
            .skip(finding.char_offset)
            .take(finding.length_chars)
            .collect();
    }

    finding
        .matched_codepoints
        .iter()
        .filter_map(|codepoint| char::from_u32(*codepoint))
        .collect()
}

fn escaped_snippet_for_finding(raw_snippet: &str, finding: &SuspiciousFinding) -> String {
    match finding.class {
        SuspiciousContentClass::BidiControl
        | SuspiciousContentClass::InvisibleFormatting
        | SuspiciousContentClass::RawRenderedDivergence => raw_snippet
            .chars()
            .map(escape_char)
            .collect::<Vec<_>>()
            .join(""),
        SuspiciousContentClass::MixedScriptConfusable
        | SuspiciousContentClass::WholeScriptConfusable => raw_snippet
            .chars()
            .map(|ch| {
                if ch.is_ascii() {
                    ch.to_string()
                } else {
                    escape_char(ch)
                }
            })
            .collect::<Vec<_>>()
            .join(""),
    }
}

fn escape_char(ch: char) -> String {
    if ch.is_control() || !ch.is_ascii() {
        format!("\\u{{{:04X}}}", ch as u32)
    } else {
        ch.to_string()
    }
}

fn warning_label_for(class: SuspiciousContentClass) -> &'static str {
    match class {
        SuspiciousContentClass::BidiControl => "Bidi control present",
        SuspiciousContentClass::InvisibleFormatting => "Invisible formatting present",
        SuspiciousContentClass::MixedScriptConfusable => "Mixed-script confusable present",
        SuspiciousContentClass::WholeScriptConfusable => "Whole-script confusable present",
        SuspiciousContentClass::RawRenderedDivergence => "Raw/rendered divergence present",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projects_shared_warning_records_for_suspicious_text() {
        let warnings = project_content_integrity_warnings(
            "case:content-integrity:test",
            ContentIntegritySurfaceKind::Editor,
            "buffer:src/lib.rs",
            "let p\u{0430}yload = user\u{202E}name\u{200D};",
        );

        assert_eq!(warnings.len(), 3);
        assert!(warnings
            .iter()
            .all(|warning| warning.uses_shared_record_kind()));
        assert!(warnings
            .iter()
            .all(|warning| warning.surface_token == "editor"));
        assert!(warnings.iter().all(|warning| {
            warning
                .transfer_action_ids
                .iter()
                .any(|action| action == "copy_escaped")
        }));
        assert!(warnings
            .iter()
            .any(|warning| warning.escaped_snippet.contains("\\u{202E}")));
    }
}
