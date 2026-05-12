//! Suspicious-content detector for plain UTF-8 text.
//!
//! Detects three classes of risky codepoints/identifiers without normalizing
//! the input or rewriting bytes:
//!
//! - **Bidi controls** — explicit directional formatting and isolate
//!   codepoints that can reorder identifier display.
//! - **Invisible formatting** — zero-width joiners, soft hyphens, and other
//!   format codepoints that hide bytes between visible glyphs.
//! - **Mixed-script confusables** — identifiers that mix scripts (e.g. Latin
//!   and Cyrillic, Latin and Greek) so a "letter" is not what it appears.
//!
//! Detection is deterministic and byte-exact. Findings carry character/byte
//! offsets so a surface can attach an inline marker to the exact token
//! without re-parsing.

use serde::{Deserialize, Serialize};

use crate::records::{
    SurfaceFamily, SuspiciousContentCaseRecord, SuspiciousContentClass,
    SuspiciousContentFindingRecord, TRUST_CLASS_SCHEMA_VERSION,
};
use crate::transfer::{
    BodyPosture, RepresentationActionId, RepresentationClass, RepresentationTransferRecord,
    TEXT_REPRESENTATION_POLICY_SCHEMA_VERSION,
};
use crate::TrustClass;

/// Closed shared-detector outcome vocabulary.
///
/// Mirrors the table in `docs/security/suspicious_content_packet.md` so a
/// single detector outcome maps unambiguously to a trust-class projection
/// and a default transfer posture for any consuming surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectorOutcomeClass {
    Allow,
    Sanitize,
    Isolate,
    Block,
    RouteToSystemBrowser,
}

impl DetectorOutcomeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allow => "allow",
            Self::Sanitize => "sanitize",
            Self::Isolate => "isolate",
            Self::Block => "block",
            Self::RouteToSystemBrowser => "route_to_system_browser",
        }
    }
}

/// One suspicious-content finding with byte-exact location data.
///
/// `byte_offset`/`char_offset` point at the first codepoint of the matching
/// run so a surface can underline or annotate the exact token. `length_chars`
/// counts how many characters the finding spans (1 for an isolated zero-width
/// joiner, more for a confusable identifier).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspiciousFinding {
    pub finding_id: String,
    pub class: SuspiciousContentClass,
    pub byte_offset: usize,
    pub char_offset: usize,
    pub length_chars: usize,
    pub matched_codepoints: Vec<u32>,
    pub note: String,
}

/// Detector outcome for a piece of plain UTF-8 text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspiciousContentDetection {
    pub outcome: DetectorOutcomeClass,
    pub findings: Vec<SuspiciousFinding>,
}

impl SuspiciousContentDetection {
    /// True when the detector did not find anything worth annotating.
    pub fn is_clean(&self) -> bool {
        matches!(self.outcome, DetectorOutcomeClass::Allow) && self.findings.is_empty()
    }

    /// True when at least one finding is present.
    pub fn has_findings(&self) -> bool {
        !self.findings.is_empty()
    }

    /// Materialize a `suspicious_content_case_record` for a given case id and
    /// surface family. Returns `None` when there are no findings.
    pub fn materialize_case_record(
        &self,
        case_id: &str,
        surface_family: SurfaceFamily,
        annotation_mode: &str,
        location_kind: &str,
    ) -> Option<SuspiciousContentCaseRecord> {
        if self.findings.is_empty() {
            return None;
        }
        let findings = self
            .findings
            .iter()
            .map(|finding| SuspiciousContentFindingRecord {
                finding_id: finding.finding_id.clone(),
                content_class: finding.class.as_str().to_string(),
                location_kind: location_kind.to_string(),
                visibility_impact: visibility_impact_for(finding.class).to_string(),
                reveal_affordances: vec![
                    "inline_marker".to_string(),
                    "codepoint_inspector".to_string(),
                    "raw_toggle".to_string(),
                    "escaped_toggle".to_string(),
                    "copy_safe_representation".to_string(),
                ],
                suppression_scope: "workspace_or_admin_policy".to_string(),
                notes: Some(finding.note.clone()),
            })
            .collect();
        Some(SuspiciousContentCaseRecord {
            record_kind: "suspicious_content_case_record".to_string(),
            trust_class_schema_version: TRUST_CLASS_SCHEMA_VERSION,
            case_id: case_id.to_string(),
            surface_family: surface_family.as_str().to_string(),
            annotation_mode: annotation_mode.to_string(),
            findings,
            stricter_annotation_requirements: vec![
                "warning_inline_on_primary_surface".to_string(),
                "export_summary_preserves_warning".to_string(),
            ],
            notes: Some(
                "Raw and escaped inspection paths stay reachable; bytes are not rewritten."
                    .to_string(),
            ),
        })
    }

    /// Build a representation-labeled transfer pair (`copy_raw`, `copy_escaped`)
    /// suitable for any surface where the detector found something.
    pub fn materialize_raw_and_escaped_transfers(
        &self,
        case_id: &str,
        surface_family: SurfaceFamily,
        trust_class: TrustClass,
    ) -> Vec<RepresentationTransferRecord> {
        if self.findings.is_empty() {
            return Vec::new();
        }
        vec![
            RepresentationTransferRecord {
                record_kind: "representation_transfer_record".to_string(),
                text_representation_policy_schema_version:
                    TEXT_REPRESENTATION_POLICY_SCHEMA_VERSION,
                case_id: case_id.to_string(),
                source_surface_family: surface_family.as_str().to_string(),
                source_trust_class: trust_class.as_str().to_string(),
                action_id: RepresentationActionId::CopyRaw.as_str().to_string(),
                representation_class: RepresentationClass::Raw.as_str().to_string(),
                label: "Copy raw".to_string(),
                body_posture: BodyPosture::ExactSourceBytes.as_str().to_string(),
                raw_source_required: true,
                active_content_removed: false,
                must_offer_also: vec![RepresentationActionId::CopyEscaped
                    .as_str()
                    .to_string()],
                required_disclosure_fields: vec![
                    "representation_label".to_string(),
                    "trust_class_badge".to_string(),
                    "warning_state".to_string(),
                ],
                notes: Some(
                    "Raw copy preserves the exact bytes so review and support can reconstruct the source."
                        .to_string(),
                ),
            },
            RepresentationTransferRecord {
                record_kind: "representation_transfer_record".to_string(),
                text_representation_policy_schema_version:
                    TEXT_REPRESENTATION_POLICY_SCHEMA_VERSION,
                case_id: case_id.to_string(),
                source_surface_family: surface_family.as_str().to_string(),
                source_trust_class: trust_class.as_str().to_string(),
                action_id: RepresentationActionId::CopyEscaped.as_str().to_string(),
                representation_class: RepresentationClass::Escaped.as_str().to_string(),
                label: "Copy escaped".to_string(),
                body_posture: BodyPosture::EscapedSourceText.as_str().to_string(),
                raw_source_required: true,
                active_content_removed: false,
                must_offer_also: vec![RepresentationActionId::CopyRaw.as_str().to_string()],
                required_disclosure_fields: vec![
                    "representation_label".to_string(),
                    "trust_class_badge".to_string(),
                    "warning_state".to_string(),
                ],
                notes: Some(
                    "Escaped copy is the safe-inspection path that does not replay control codepoints."
                        .to_string(),
                ),
            },
        ]
    }
}

/// Returns true when the supplied UTF-8 text contains any suspicious finding.
pub fn has_suspicious_content(text: &str) -> bool {
    detect_suspicious_content(text).has_findings()
}

/// Run the suspicious-content detector on plain UTF-8 text.
///
/// The detector inspects three independent dimensions in one pass:
///
/// 1. Bidi-control codepoints (LRE/RLE/PDF/LRO/RLO/LRI/RLI/FSI/PDI/ALM/etc.)
/// 2. Invisible-formatting codepoints (zero-width joiner, zero-width
///    non-joiner, zero-width space, byte-order mark, soft hyphen, word
///    joiner, invisible separators, language tags).
/// 3. Mixed-script identifiers (identifier-like runs containing letters from
///    more than one script, where Common/Inherited do not count as a script).
pub fn detect_suspicious_content(text: &str) -> SuspiciousContentDetection {
    let mut findings = Vec::new();

    // Pass 1: bidi controls and invisible formatting (per-codepoint).
    for (char_offset, (byte_offset, ch)) in text.char_indices().enumerate() {
        if is_bidi_control(ch) {
            findings.push(SuspiciousFinding {
                finding_id: format!("finding:bidi:{char_offset}"),
                class: SuspiciousContentClass::BidiControl,
                byte_offset,
                char_offset,
                length_chars: 1,
                matched_codepoints: vec![ch as u32],
                note: format!(
                    "Bidi control U+{cp:04X} can reorder text without changing source bytes.",
                    cp = ch as u32
                ),
            });
        } else if is_invisible_formatting(ch) {
            findings.push(SuspiciousFinding {
                finding_id: format!("finding:invisible:{char_offset}"),
                class: SuspiciousContentClass::InvisibleFormatting,
                byte_offset,
                char_offset,
                length_chars: 1,
                matched_codepoints: vec![ch as u32],
                note: format!(
                    "Invisible formatting U+{cp:04X} hides codepoints between glyphs.",
                    cp = ch as u32
                ),
            });
        }
    }

    // Pass 2: mixed-script confusable identifiers.
    findings.extend(scan_mixed_script_identifiers(text));

    let outcome = if findings.is_empty() {
        DetectorOutcomeClass::Allow
    } else {
        // For raw-text editor surfaces the outcome is "annotate and do not
        // normalize"; the schema spelling is `sanitize` on the annotation
        // layer with the body content remaining `RawText`.
        DetectorOutcomeClass::Sanitize
    };

    SuspiciousContentDetection { outcome, findings }
}

/// Render text with suspicious codepoints replaced by `\u{XXXX}` escapes.
///
/// Visible non-suspicious characters round-trip unchanged. The escaped form
/// is the safe-inspection path for pasting into chat, logs, or review notes.
pub fn escape_for_safe_inspection(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        if is_bidi_control(ch) || is_invisible_formatting(ch) {
            out.push_str(&format!("\\u{{{:04X}}}", ch as u32));
        } else {
            out.push(ch);
        }
    }
    out
}

const fn is_bidi_control(ch: char) -> bool {
    matches!(
        ch as u32,
        // Explicit directional embedding / override / pop
        0x202A..=0x202E
        // Isolates
        | 0x2066..=0x2069
        // Arabic letter mark / Hebrew mark
        | 0x061C
        | 0x200E
        | 0x200F
    )
}

const fn is_invisible_formatting(ch: char) -> bool {
    matches!(
        ch as u32,
        // Zero-width space, ZWNJ, ZWJ
        0x200B..=0x200D
        // Soft hyphen
        | 0x00AD
        // Word joiner, invisible math operators, function application
        | 0x2060..=0x2064
        // Invisible separator / plus
        | 0xFEFF
        // Mongolian vowel separator
        | 0x180E
        // Tag characters
        | 0xE0000..=0xE007F
        // Variation selectors (often weaponized to mask identifiers)
        | 0xFE00..=0xFE0F
    )
}

fn visibility_impact_for(class: SuspiciousContentClass) -> &'static str {
    match class {
        SuspiciousContentClass::BidiControl => "reorders_text",
        SuspiciousContentClass::InvisibleFormatting => "hides_codepoints",
        SuspiciousContentClass::MixedScriptConfusable
        | SuspiciousContentClass::WholeScriptConfusable => "looks_like_other_identifier",
        SuspiciousContentClass::RawRenderedDivergence => "rendered_differs_from_source",
    }
}

/// Detect identifier-like runs (alphanumeric + `_`) with letters from more
/// than one script. Common/Inherited script characters (digits, `_`) do not
/// count toward "more than one script" because they appear in any identifier.
fn scan_mixed_script_identifiers(text: &str) -> Vec<SuspiciousFinding> {
    let mut findings = Vec::new();
    let mut current_start_byte: Option<usize> = None;
    let mut current_start_char: usize = 0;
    let mut current_end_char: usize = 0;
    let mut current_scripts: u8 = 0;
    let mut current_codepoints: Vec<u32> = Vec::new();

    let mut byte_index = 0usize;

    for (char_index, ch) in text.chars().enumerate() {
        let char_len = ch.len_utf8();
        if is_identifier_char(ch) {
            if current_start_byte.is_none() {
                current_start_byte = Some(byte_index);
                current_start_char = char_index;
                current_scripts = 0;
                current_codepoints.clear();
            }
            current_scripts |= script_bit(ch);
            current_codepoints.push(ch as u32);
            current_end_char = char_index + 1;
        } else if let Some(start_byte) = current_start_byte.take() {
            if current_scripts.count_ones() >= 2 {
                findings.push(SuspiciousFinding {
                    finding_id: format!("finding:mixed_script:{}", current_start_char),
                    class: SuspiciousContentClass::MixedScriptConfusable,
                    byte_offset: start_byte,
                    char_offset: current_start_char,
                    length_chars: current_end_char - current_start_char,
                    matched_codepoints: std::mem::take(&mut current_codepoints),
                    note: "Identifier mixes letters from more than one script.".to_string(),
                });
            }
        }
        byte_index += char_len;
    }
    if let Some(start_byte) = current_start_byte.take() {
        if current_scripts.count_ones() >= 2 {
            findings.push(SuspiciousFinding {
                finding_id: format!("finding:mixed_script:{}", current_start_char),
                class: SuspiciousContentClass::MixedScriptConfusable,
                byte_offset: start_byte,
                char_offset: current_start_char,
                length_chars: current_end_char - current_start_char,
                matched_codepoints: current_codepoints,
                note: "Identifier mixes letters from more than one script.".to_string(),
            });
        }
    }

    findings
}

fn is_identifier_char(ch: char) -> bool {
    ch == '_' || ch.is_alphanumeric()
}

/// Return a bit-mask script tag for `ch`. Common/Inherited characters
/// (ASCII digits, `_`) return 0 so they do not contribute to the mixed-script
/// count.
fn script_bit(ch: char) -> u8 {
    let cp = ch as u32;
    if !ch.is_alphabetic() {
        return 0;
    }
    match cp {
        // Latin (basic + supplements + extended A/B + IPA + Latin Extended Additional)
        0x0041..=0x005A | 0x0061..=0x007A | 0x00C0..=0x00FF | 0x0100..=0x024F | 0x1E00..=0x1EFF => {
            1 << 0
        }
        // Cyrillic
        0x0400..=0x04FF | 0x0500..=0x052F | 0x2DE0..=0x2DFF | 0xA640..=0xA69F => 1 << 1,
        // Greek
        0x0370..=0x03FF | 0x1F00..=0x1FFF => 1 << 2,
        // Armenian
        0x0530..=0x058F => 1 << 3,
        // Hebrew
        0x0590..=0x05FF => 1 << 4,
        // Arabic
        0x0600..=0x06FF | 0x0750..=0x077F | 0xFB50..=0xFDFF | 0xFE70..=0xFEFF => 1 << 5,
        // CJK Unified Ideographs (basic)
        0x4E00..=0x9FFF => 1 << 6,
        // Hiragana / Katakana
        0x3040..=0x30FF => 1 << 7,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_clean_ascii_identifier() {
        let detection = detect_suspicious_content("fn config_loader() { let user_id = 1; }");
        assert!(detection.is_clean());
        assert_eq!(detection.outcome, DetectorOutcomeClass::Allow);
    }

    #[test]
    fn detects_bidi_control_in_identifier() {
        // U+202E RIGHT-TO-LEFT OVERRIDE
        let text = "let \u{202E}admin = 1;";
        let detection = detect_suspicious_content(text);
        assert!(detection.has_findings());
        assert_eq!(detection.outcome, DetectorOutcomeClass::Sanitize);
        assert!(detection
            .findings
            .iter()
            .any(|f| f.class == SuspiciousContentClass::BidiControl));
    }

    #[test]
    fn detects_zero_width_joiner_inside_identifier() {
        // U+200D ZERO WIDTH JOINER between letters
        let text = "let admin\u{200D}user = 1;";
        let detection = detect_suspicious_content(text);
        assert!(detection.has_findings());
        assert!(detection
            .findings
            .iter()
            .any(|f| f.class == SuspiciousContentClass::InvisibleFormatting));
    }

    #[test]
    fn detects_mixed_script_identifier_latin_cyrillic() {
        // 'а' here is U+0430 CYRILLIC SMALL LETTER A; the rest are Latin.
        let text = "let p\u{0430}yload = 1;";
        let detection = detect_suspicious_content(text);
        assert!(detection.has_findings());
        assert!(detection
            .findings
            .iter()
            .any(|f| f.class == SuspiciousContentClass::MixedScriptConfusable));
    }

    #[test]
    fn escape_for_safe_inspection_replaces_bidi_and_invisible() {
        let text = "abc\u{202E}def\u{200D}ghi";
        let escaped = escape_for_safe_inspection(text);
        assert!(escaped.contains("\\u{202E}"));
        assert!(escaped.contains("\\u{200D}"));
        assert!(!escaped.contains('\u{202E}'));
    }

    #[test]
    fn case_record_round_trips_via_serde() {
        let text = "let \u{202E}x = 1;";
        let detection = detect_suspicious_content(text);
        let case = detection
            .materialize_case_record(
                "case:test",
                SurfaceFamily::EditorContent,
                "standard_inline",
                "identifier",
            )
            .expect("case record present");
        let json = serde_json::to_string(&case).expect("serialize");
        let back: SuspiciousContentCaseRecord = serde_json::from_str(&json).expect("parse");
        assert_eq!(back.record_kind, "suspicious_content_case_record");
        assert_eq!(back.surface_family, "editor_content");
        assert_eq!(back.findings.len(), detection.findings.len());
    }

    #[test]
    fn raw_and_escaped_transfers_pair_each_other() {
        let text = "abc\u{202E}xyz";
        let detection = detect_suspicious_content(text);
        let transfers = detection.materialize_raw_and_escaped_transfers(
            "case:test",
            SurfaceFamily::EditorContent,
            TrustClass::RawText,
        );
        assert_eq!(transfers.len(), 2);
        let raw = transfers
            .iter()
            .find(|r| r.action_id == "copy_raw")
            .unwrap();
        let escaped = transfers
            .iter()
            .find(|r| r.action_id == "copy_escaped")
            .unwrap();
        assert!(raw.must_offer_also.iter().any(|s| s == "copy_escaped"));
        assert!(escaped.must_offer_also.iter().any(|s| s == "copy_raw"));
    }

    #[test]
    fn no_findings_yield_no_case_record_or_transfers() {
        let detection = detect_suspicious_content("plain text");
        assert!(detection
            .materialize_case_record(
                "case:none",
                SurfaceFamily::EditorContent,
                "standard_inline",
                "identifier",
            )
            .is_none());
        assert!(detection
            .materialize_raw_and_escaped_transfers(
                "case:none",
                SurfaceFamily::EditorContent,
                TrustClass::RawText,
            )
            .is_empty());
    }
}
