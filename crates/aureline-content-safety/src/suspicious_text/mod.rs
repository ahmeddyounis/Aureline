//! Cross-surface suspicious-text projections for source-facing surfaces.
//!
//! The detector in [`crate::detector`] owns the byte-level suspicious-content
//! findings. This module projects those findings onto the first core source
//! surfaces that need identical warning behavior: editor, diff, search, and
//! review. It keeps the warning class, location, raw snippet, escaped snippet,
//! copy/export actions, and continuity reference together so a surface can
//! reopen, copy, or export the same finding without dropping the warning.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    detect_suspicious_content, BodyPosture, DetectorOutcomeClass, RepresentationActionId,
    RepresentationClass, SuspiciousContentClass, SuspiciousFinding,
};

/// Schema version for suspicious-text source-surface projection packets.
pub const SUSPICIOUS_TEXT_SURFACE_PACKET_SCHEMA_VERSION: u32 = 1;

/// Closed source-facing surface set covered by the suspicious-text alpha path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuspiciousTextSurfaceKind {
    /// Live editor buffer or source editor pane.
    Editor,
    /// Diff or merge hunk projection.
    Diff,
    /// Search result row or snippet projection.
    Search,
    /// Review hunk, thread, or review packet anchor.
    Review,
}

impl SuspiciousTextSurfaceKind {
    /// Stable token used in records, fixtures, and CLI output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Diff => "diff",
            Self::Search => "search",
            Self::Review => "review",
        }
    }

    /// Human-readable noun used in action labels.
    pub const fn label_noun(self) -> &'static str {
        match self {
            Self::Editor => "selection",
            Self::Diff => "hunk",
            Self::Search => "snippet",
            Self::Review => "review anchor",
        }
    }
}

/// Core surfaces that must preserve the shared suspicious-text warning classes.
pub const CORE_SOURCE_SURFACES: [SuspiciousTextSurfaceKind; 4] = [
    SuspiciousTextSurfaceKind::Editor,
    SuspiciousTextSurfaceKind::Diff,
    SuspiciousTextSurfaceKind::Search,
    SuspiciousTextSurfaceKind::Review,
];

/// Location family for a warning anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuspiciousTextLocationKind {
    /// Byte and character range in a live editor buffer.
    EditorRange,
    /// Byte and character range inside a diff hunk.
    DiffHunk,
    /// Byte and character range inside a search result row snippet.
    SearchRow,
    /// Byte and character range carried by a review anchor.
    ReviewAnchor,
}

impl SuspiciousTextLocationKind {
    /// Stable token used in records, fixtures, and CLI output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorRange => "editor_range",
            Self::DiffHunk => "diff_hunk",
            Self::SearchRow => "search_row",
            Self::ReviewAnchor => "review_anchor",
        }
    }
}

/// Reveal, copy, and export actions that must stay attached to a warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuspiciousTextWarningAction {
    /// Reveal exact source text on the invoking surface.
    RevealRawSource,
    /// Reveal escaped source text with risky codepoints expanded.
    RevealEscapedSource,
    /// Open a codepoint inspector for the suspicious range.
    InspectCodepoints,
    /// Copy the exact source representation.
    CopyRaw,
    /// Copy the escaped safe-inspection representation.
    CopyEscaped,
    /// Export a safe representation with warning refs preserved.
    ExportSafeRepresentation,
}

impl SuspiciousTextWarningAction {
    /// Stable action id used in records, fixtures, and CLI output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealRawSource => "reveal_raw_source",
            Self::RevealEscapedSource => "reveal_escaped_source",
            Self::InspectCodepoints => "inspect_codepoints",
            Self::CopyRaw => "copy_raw",
            Self::CopyEscaped => "copy_escaped",
            Self::ExportSafeRepresentation => "export_safe_representation",
        }
    }
}

/// Surface-specific location for one suspicious-text warning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspiciousTextAnchor {
    /// Surface where this anchor is rendered.
    pub surface: SuspiciousTextSurfaceKind,
    /// Surface-specific location family.
    pub location_kind: SuspiciousTextLocationKind,
    /// Opaque subject, hunk, row, or review-anchor ref.
    pub subject_ref: String,
    /// Shared continuity ref used to join the same finding across surfaces.
    pub continuity_ref: String,
    /// Zero-based line index in the source text supplied to the detector.
    pub line_index: usize,
    /// Zero-based character column index on `line_index`.
    pub column_index: usize,
    /// Byte offset of the first suspicious codepoint in the source text.
    pub byte_offset: usize,
    /// Character offset of the first suspicious codepoint in the source text.
    pub char_offset: usize,
    /// Number of Unicode scalar values covered by the warning.
    pub length_chars: usize,
}

/// One warning projected onto a concrete source-facing surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspiciousTextWarning {
    /// Surface-stable warning id.
    pub warning_id: String,
    /// Detector finding id this warning projects from.
    pub detector_finding_id: String,
    /// Shared suspicious-content class from the detector.
    pub content_class: SuspiciousContentClass,
    /// Exact surface location for this warning.
    pub anchor: SuspiciousTextAnchor,
    /// Codepoints observed by the detector for this warning.
    pub matched_codepoints: Vec<u32>,
    /// Exact source snippet for this finding.
    pub raw_snippet: String,
    /// Safe-inspection snippet with risky codepoints escaped.
    pub escaped_snippet: String,
    /// Short warning label suitable for chips or review summaries.
    pub warning_label: String,
    /// Actions that must remain reachable from this warning.
    pub available_actions: Vec<SuspiciousTextWarningAction>,
}

impl SuspiciousTextWarning {
    /// Returns the stable suspicious-content class token.
    pub const fn class_token(&self) -> &'static str {
        self.content_class.as_str()
    }

    /// True when the warning exposes both raw and escaped reveal actions.
    pub fn offers_raw_and_escaped_reveal(&self) -> bool {
        self.available_actions
            .contains(&SuspiciousTextWarningAction::RevealRawSource)
            && self
                .available_actions
                .contains(&SuspiciousTextWarningAction::RevealEscapedSource)
    }
}

/// Representation transfer choice offered by one surface projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspiciousTextTransferChoice {
    /// Transfer action id drawn from the representation policy vocabulary.
    pub action_id: String,
    /// Representation class drawn from the representation policy vocabulary.
    pub representation_class: String,
    /// Body posture drawn from the representation policy vocabulary.
    pub body_posture: String,
    /// Surface label that names the representation explicitly.
    pub label: String,
    /// True when this action is the safe path for sharing suspicious text.
    pub safe_representation_path: bool,
    /// Warning ids that must travel with the transferred representation.
    pub attached_warning_ids: Vec<String>,
}

/// Safe export projection for one source-facing surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspiciousTextSafeExport {
    /// Export action id drawn from the representation policy vocabulary.
    pub action_id: String,
    /// Representation class drawn from the representation policy vocabulary.
    pub representation_class: String,
    /// Body posture drawn from the representation policy vocabulary.
    pub body_posture: String,
    /// Escaped source text to include in the safe export body.
    pub escaped_body: String,
    /// Warning ids preserved in the export summary.
    pub attached_warning_ids: Vec<String>,
    /// Whether the export normalized or stripped source bytes.
    pub normalization_applied: bool,
}

/// Source-facing projection for one surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspiciousTextSurfaceProjection {
    /// Surface this projection describes.
    pub surface: SuspiciousTextSurfaceKind,
    /// Surface subject ref used by anchors and reopen paths.
    pub subject_ref: String,
    /// Warnings attached to exact source locations on this surface.
    pub warnings: Vec<SuspiciousTextWarning>,
    /// Copy choices surfaced with explicit raw/escaped representation labels.
    pub copy_choices: Vec<SuspiciousTextTransferChoice>,
    /// Safe export path surfaced when suspicious content is present.
    pub safe_export: Option<SuspiciousTextSafeExport>,
    /// Whether reopen paths must preserve warning anchors for this projection.
    pub review_reopen_preserves_warning_refs: bool,
}

impl SuspiciousTextSurfaceProjection {
    /// Returns the suspicious-content class tokens visible on this surface.
    pub fn warning_class_tokens(&self) -> Vec<&'static str> {
        self.warnings
            .iter()
            .map(SuspiciousTextWarning::class_token)
            .collect()
    }

    /// Returns true when copy and export choices retain every warning id.
    pub fn transfer_choices_preserve_warning_refs(&self) -> bool {
        let warning_ids: BTreeSet<_> = self
            .warnings
            .iter()
            .map(|warning| warning.warning_id.as_str())
            .collect();
        if warning_ids.is_empty() {
            return true;
        }

        let copy_preserves = self.copy_choices.iter().all(|choice| {
            warning_ids.iter().all(|warning_id| {
                choice
                    .attached_warning_ids
                    .iter()
                    .any(|id| id == warning_id)
            })
        });
        let export_preserves = self.safe_export.as_ref().is_some_and(|safe_export| {
            warning_ids.iter().all(|warning_id| {
                safe_export
                    .attached_warning_ids
                    .iter()
                    .any(|id| id == warning_id)
            })
        });
        copy_preserves && export_preserves
    }

    /// Returns true when copy/export expose an escaped safe representation.
    pub fn offers_safe_representation_path(&self) -> bool {
        self.copy_choices.iter().any(|choice| {
            choice.action_id == RepresentationActionId::CopyEscaped.as_str()
                && choice.safe_representation_path
        }) && self.safe_export.is_some()
    }
}

/// Inputs needed to project one detector run across the core source surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SuspiciousTextProjectionSeed<'a> {
    /// Stable case id shared by all surface projections.
    pub case_id: &'a str,
    /// Source text inspected by the shared detector.
    pub content: &'a str,
    /// Opaque editor subject ref.
    pub editor_subject_ref: &'a str,
    /// Opaque diff hunk ref.
    pub diff_hunk_ref: &'a str,
    /// Opaque search row ref.
    pub search_row_ref: &'a str,
    /// Opaque review anchor ref.
    pub review_anchor_ref: &'a str,
}

/// Cross-surface packet emitted by the suspicious-text projection path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspiciousTextSurfacePacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this packet.
    pub schema_version: u32,
    /// Case id shared by all projections.
    pub case_id: String,
    /// Detector outcome before surface projection.
    pub detector_outcome: DetectorOutcomeClass,
    /// Distinct suspicious-content class tokens found in the source text.
    pub finding_classes: Vec<String>,
    /// Number of bytes inspected by the detector.
    pub source_len_bytes: usize,
    /// Whether projection normalized or stripped the source.
    pub normalization_applied: bool,
    /// Surface projections for editor, diff, search, and review.
    pub surfaces: Vec<SuspiciousTextSurfaceProjection>,
}

impl SuspiciousTextSurfacePacket {
    /// Stable record-kind value for serialized packets.
    pub const RECORD_KIND: &'static str = "suspicious_text_surface_packet";

    /// Returns true when every core source surface is present exactly once.
    pub fn covers_core_source_surfaces(&self) -> bool {
        let present: BTreeSet<_> = self
            .surfaces
            .iter()
            .map(|surface| surface.surface)
            .collect();
        CORE_SOURCE_SURFACES
            .iter()
            .all(|surface| present.contains(surface))
            && present.len() == CORE_SOURCE_SURFACES.len()
    }

    /// Returns true when every surface exposes the same warning-class set.
    pub fn all_surfaces_share_warning_classes(&self) -> bool {
        let expected: BTreeSet<_> = self.finding_classes.iter().map(String::as_str).collect();
        self.surfaces.iter().all(|surface| {
            let actual: BTreeSet<_> = surface.warning_class_tokens().into_iter().collect();
            actual == expected
        })
    }

    /// Returns true when every warning exposes raw and escaped reveal actions.
    pub fn all_warnings_offer_raw_and_escaped_reveal(&self) -> bool {
        self.surfaces.iter().all(|surface| {
            surface
                .warnings
                .iter()
                .all(SuspiciousTextWarning::offers_raw_and_escaped_reveal)
        })
    }

    /// Returns true when every surface keeps warning refs on copy/export.
    pub fn all_transfers_preserve_warning_refs(&self) -> bool {
        self.surfaces
            .iter()
            .all(SuspiciousTextSurfaceProjection::transfer_choices_preserve_warning_refs)
    }

    /// Returns true when every surface exposes a safe representation path.
    pub fn all_surfaces_offer_safe_representation_path(&self) -> bool {
        self.surfaces
            .iter()
            .all(|surface| surface.warnings.is_empty() || surface.offers_safe_representation_path())
    }
}

/// Project one shared detector run across editor, diff, search, and review.
pub fn project_suspicious_text_core_surfaces(
    seed: &SuspiciousTextProjectionSeed<'_>,
) -> SuspiciousTextSurfacePacket {
    let detection = detect_suspicious_content(seed.content);
    let finding_classes = distinct_finding_classes(&detection.findings);
    let surfaces = CORE_SOURCE_SURFACES
        .into_iter()
        .map(|surface| project_surface(seed, surface, &detection.findings))
        .collect();

    SuspiciousTextSurfacePacket {
        record_kind: SuspiciousTextSurfacePacket::RECORD_KIND.to_string(),
        schema_version: SUSPICIOUS_TEXT_SURFACE_PACKET_SCHEMA_VERSION,
        case_id: seed.case_id.to_string(),
        detector_outcome: detection.outcome,
        finding_classes,
        source_len_bytes: seed.content.len(),
        normalization_applied: false,
        surfaces,
    }
}

fn distinct_finding_classes(findings: &[SuspiciousFinding]) -> Vec<String> {
    findings
        .iter()
        .map(|finding| finding.class.as_str().to_string())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn project_surface(
    seed: &SuspiciousTextProjectionSeed<'_>,
    surface: SuspiciousTextSurfaceKind,
    findings: &[SuspiciousFinding],
) -> SuspiciousTextSurfaceProjection {
    let subject_ref = subject_ref_for(seed, surface).to_string();
    let warnings: Vec<_> = findings
        .iter()
        .map(|finding| project_warning(seed, surface, &subject_ref, finding))
        .collect();
    let warning_ids = warnings
        .iter()
        .map(|warning| warning.warning_id.clone())
        .collect::<Vec<_>>();
    let copy_choices = if warnings.is_empty() {
        vec![raw_copy_choice(surface, Vec::new())]
    } else {
        vec![
            raw_copy_choice(surface, warning_ids.clone()),
            escaped_copy_choice(surface, warning_ids.clone()),
        ]
    };
    let safe_export = if warnings.is_empty() {
        None
    } else {
        Some(SuspiciousTextSafeExport {
            action_id: RepresentationActionId::ExportSanitizedSnapshot
                .as_str()
                .to_string(),
            representation_class: RepresentationClass::Sanitized.as_str().to_string(),
            body_posture: BodyPosture::SanitizedStaticSnapshot.as_str().to_string(),
            escaped_body: escape_for_surface_export(seed.content, findings),
            attached_warning_ids: warning_ids,
            normalization_applied: false,
        })
    };

    SuspiciousTextSurfaceProjection {
        surface,
        subject_ref,
        warnings,
        copy_choices,
        safe_export,
        review_reopen_preserves_warning_refs: true,
    }
}

fn project_warning(
    seed: &SuspiciousTextProjectionSeed<'_>,
    surface: SuspiciousTextSurfaceKind,
    subject_ref: &str,
    finding: &SuspiciousFinding,
) -> SuspiciousTextWarning {
    let (line_index, column_index) = line_column_for_char_offset(seed.content, finding.char_offset);
    let raw_snippet = snippet_for_finding(seed.content, finding);
    let escaped_snippet = escape_for_warning_snippet(&raw_snippet, finding);
    let detector_id = finding.finding_id.replace(':', "_");
    let surface_token = surface.as_str();

    SuspiciousTextWarning {
        warning_id: format!("warning:{surface_token}:{detector_id}"),
        detector_finding_id: finding.finding_id.clone(),
        content_class: finding.class,
        anchor: SuspiciousTextAnchor {
            surface,
            location_kind: location_kind_for(surface),
            subject_ref: subject_ref.to_string(),
            continuity_ref: format!("suspicious-text:{}:{}", seed.case_id, finding.finding_id),
            line_index,
            column_index,
            byte_offset: finding.byte_offset,
            char_offset: finding.char_offset,
            length_chars: finding.length_chars,
        },
        matched_codepoints: finding.matched_codepoints.clone(),
        raw_snippet,
        escaped_snippet,
        warning_label: warning_label_for(finding.class).to_string(),
        available_actions: vec![
            SuspiciousTextWarningAction::RevealRawSource,
            SuspiciousTextWarningAction::RevealEscapedSource,
            SuspiciousTextWarningAction::InspectCodepoints,
            SuspiciousTextWarningAction::CopyRaw,
            SuspiciousTextWarningAction::CopyEscaped,
            SuspiciousTextWarningAction::ExportSafeRepresentation,
        ],
    }
}

fn raw_copy_choice(
    surface: SuspiciousTextSurfaceKind,
    attached_warning_ids: Vec<String>,
) -> SuspiciousTextTransferChoice {
    SuspiciousTextTransferChoice {
        action_id: RepresentationActionId::CopyRaw.as_str().to_string(),
        representation_class: RepresentationClass::Raw.as_str().to_string(),
        body_posture: BodyPosture::ExactSourceBytes.as_str().to_string(),
        label: format!("Copy raw {}", surface.label_noun()),
        safe_representation_path: false,
        attached_warning_ids,
    }
}

fn escaped_copy_choice(
    surface: SuspiciousTextSurfaceKind,
    attached_warning_ids: Vec<String>,
) -> SuspiciousTextTransferChoice {
    SuspiciousTextTransferChoice {
        action_id: RepresentationActionId::CopyEscaped.as_str().to_string(),
        representation_class: RepresentationClass::Escaped.as_str().to_string(),
        body_posture: BodyPosture::EscapedSourceText.as_str().to_string(),
        label: format!("Copy escaped {}", surface.label_noun()),
        safe_representation_path: true,
        attached_warning_ids,
    }
}

fn subject_ref_for<'a>(
    seed: &'a SuspiciousTextProjectionSeed<'_>,
    surface: SuspiciousTextSurfaceKind,
) -> &'a str {
    match surface {
        SuspiciousTextSurfaceKind::Editor => seed.editor_subject_ref,
        SuspiciousTextSurfaceKind::Diff => seed.diff_hunk_ref,
        SuspiciousTextSurfaceKind::Search => seed.search_row_ref,
        SuspiciousTextSurfaceKind::Review => seed.review_anchor_ref,
    }
}

fn location_kind_for(surface: SuspiciousTextSurfaceKind) -> SuspiciousTextLocationKind {
    match surface {
        SuspiciousTextSurfaceKind::Editor => SuspiciousTextLocationKind::EditorRange,
        SuspiciousTextSurfaceKind::Diff => SuspiciousTextLocationKind::DiffHunk,
        SuspiciousTextSurfaceKind::Search => SuspiciousTextLocationKind::SearchRow,
        SuspiciousTextSurfaceKind::Review => SuspiciousTextLocationKind::ReviewAnchor,
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

fn snippet_for_finding(text: &str, finding: &SuspiciousFinding) -> String {
    text.chars()
        .skip(finding.char_offset)
        .take(finding.length_chars)
        .collect()
}

fn escape_for_warning_snippet(snippet: &str, finding: &SuspiciousFinding) -> String {
    let start = finding.char_offset;
    let end = start + finding.length_chars;
    escape_chars_by_policy(
        snippet
            .chars()
            .enumerate()
            .map(|(idx, ch)| (start + idx, ch)),
        |char_offset, ch| should_escape_for_finding(finding, char_offset, ch, start, end),
    )
}

fn escape_for_surface_export(text: &str, findings: &[SuspiciousFinding]) -> String {
    escape_chars_by_policy(text.chars().enumerate(), |char_offset, ch| {
        findings.iter().any(|finding| {
            let start = finding.char_offset;
            let end = start + finding.length_chars;
            should_escape_for_finding(finding, char_offset, ch, start, end)
        })
    })
}

fn escape_chars_by_policy<I, F>(chars: I, should_escape: F) -> String
where
    I: IntoIterator<Item = (usize, char)>,
    F: Fn(usize, char) -> bool,
{
    let mut out = String::new();
    for (char_offset, ch) in chars {
        if should_escape(char_offset, ch) {
            out.push_str(&format!("\\u{{{:04X}}}", ch as u32));
        } else {
            out.push(ch);
        }
    }
    out
}

fn should_escape_for_finding(
    finding: &SuspiciousFinding,
    char_offset: usize,
    ch: char,
    start: usize,
    end: usize,
) -> bool {
    if !(start..end).contains(&char_offset) {
        return false;
    }
    match finding.class {
        SuspiciousContentClass::BidiControl
        | SuspiciousContentClass::InvisibleFormatting
        | SuspiciousContentClass::RawRenderedDivergence => true,
        SuspiciousContentClass::MixedScriptConfusable
        | SuspiciousContentClass::WholeScriptConfusable => !ch.is_ascii(),
    }
}

fn line_column_for_char_offset(text: &str, target_char_offset: usize) -> (usize, usize) {
    let mut line_index = 0usize;
    let mut column_index = 0usize;

    for (char_offset, ch) in text.chars().enumerate() {
        if char_offset == target_char_offset {
            return (line_index, column_index);
        }
        if ch == '\n' {
            line_index += 1;
            column_index = 0;
        } else {
            column_index += 1;
        }
    }

    (line_index, column_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn seed() -> SuspiciousTextProjectionSeed<'static> {
        SuspiciousTextProjectionSeed {
            case_id: "case:suspicious-text:test",
            content: "let p\u{0430}yload = \"ok\";\nlet admin\u{202E} = user\u{200D}name;\n",
            editor_subject_ref: "buffer:src/app.ts",
            diff_hunk_ref: "diff:src/app.ts:hunk:1",
            search_row_ref: "search:row:src/app.ts:1",
            review_anchor_ref: "review:anchor:src/app.ts:hunk:1",
        }
    }

    #[test]
    fn projects_same_warning_classes_across_core_surfaces() {
        let packet = project_suspicious_text_core_surfaces(&seed());

        assert!(packet.covers_core_source_surfaces());
        assert!(packet.all_surfaces_share_warning_classes());
        assert_eq!(
            packet.finding_classes,
            vec![
                "bidi_control".to_string(),
                "invisible_formatting".to_string(),
                "mixed_script_confusable".to_string(),
            ]
        );
        assert!(packet
            .surfaces
            .iter()
            .all(|surface| surface.warnings.len() == 3));
    }

    #[test]
    fn warnings_keep_exact_anchors_and_reveal_actions() {
        let packet = project_suspicious_text_core_surfaces(&seed());

        assert!(packet.all_warnings_offer_raw_and_escaped_reveal());
        for surface in &packet.surfaces {
            for warning in &surface.warnings {
                assert_eq!(warning.anchor.subject_ref, surface.subject_ref);
                assert!(!warning.anchor.continuity_ref.is_empty());
                assert!(!warning.raw_snippet.is_empty());
                assert!(!warning.escaped_snippet.is_empty());
            }
        }
    }

    #[test]
    fn copy_and_export_choices_preserve_warning_refs() {
        let packet = project_suspicious_text_core_surfaces(&seed());

        assert!(packet.all_surfaces_offer_safe_representation_path());
        assert!(packet.all_transfers_preserve_warning_refs());
        for surface in &packet.surfaces {
            assert!(surface
                .copy_choices
                .iter()
                .any(|choice| choice.action_id == "copy_raw"));
            assert!(surface.copy_choices.iter().any(|choice| {
                choice.action_id == "copy_escaped" && choice.safe_representation_path
            }));
            let safe_export = surface.safe_export.as_ref().expect("safe export");
            assert_eq!(safe_export.action_id, "export_sanitized_snapshot");
            assert!(!safe_export.normalization_applied);
            assert!(safe_export.escaped_body.contains("\\u{0430}"));
            assert!(safe_export.escaped_body.contains("\\u{202E}"));
            assert!(safe_export.escaped_body.contains("\\u{200D}"));
        }
    }

    #[test]
    fn packet_round_trips_via_serde() {
        let packet = project_suspicious_text_core_surfaces(&seed());
        let json = serde_json::to_string(&packet).expect("serialize");
        let back: SuspiciousTextSurfacePacket = serde_json::from_str(&json).expect("parse");

        assert_eq!(back.record_kind, SuspiciousTextSurfacePacket::RECORD_KIND);
        assert_eq!(back.surfaces.len(), 4);
        assert!(back.all_surfaces_share_warning_classes());
    }
}
