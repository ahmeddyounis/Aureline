//! Source-fidelity detection and preservation helpers.
//!
//! The staged save coordinator commits durable bytes, but the editor must also
//! preserve representation details users and downstream tooling rely on:
//! encoding, BOM state, newline mode, final-newline posture, and executable
//! intent where the platform/root model supports it.
//!
//! This module provides:
//! - open-time detection helpers for representation metadata, and
//! - save-time encoding/BOM preservation for staged UTF-8 buffer bytes.

use aureline_vfs::PermissionSnapshot;
use serde::{Deserialize, Serialize};

/// Closed encoding vocabulary used by the source-fidelity record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectedEncoding {
    /// UTF-8 text without a BOM.
    Utf8,
    /// UTF-8 text that carried a BOM at open time.
    Utf8Bom,
    /// UTF-16LE text with a BOM at open time.
    Utf16LeBom,
    /// UTF-16BE text with a BOM at open time.
    Utf16BeBom,
    /// UTF-32LE text with a BOM at open time.
    Utf32LeBom,
    /// UTF-32BE text with a BOM at open time.
    Utf32BeBom,
    /// Bytes could not be decoded as text under the supported heuristics.
    UnknownBinaryLike,
}

impl DetectedEncoding {
    /// Returns the stable string vocabulary for this encoding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Utf8 => "utf8",
            Self::Utf8Bom => "utf8_bom",
            Self::Utf16LeBom => "utf16le_bom",
            Self::Utf16BeBom => "utf16be_bom",
            Self::Utf32LeBom => "utf32le_bom",
            Self::Utf32BeBom => "utf32be_bom",
            Self::UnknownBinaryLike => "unknown_binary_like",
        }
    }
}

/// Explanation for how Aureline chose an encoding at open time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DetectionSource {
    /// A byte-order mark determined the encoding.
    Bom,
    /// UTF-8 validation succeeded and no BOM was present.
    Utf8Heuristic,
    /// No supported decoder could be chosen confidently.
    DecodeFailedNoChoice,
}

impl DetectionSource {
    /// Returns the stable string vocabulary for this detection source.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bom => "bom",
            Self::Utf8Heuristic => "utf8_heuristic",
            Self::DecodeFailedNoChoice => "decode_failed_no_choice",
        }
    }
}

/// Whether a BOM was present at open time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BomStateDetected {
    Present,
    Absent,
    UnknownOrDegraded,
}

impl BomStateDetected {
    /// Returns the stable string vocabulary for this BOM state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Absent => "absent",
            Self::UnknownOrDegraded => "unknown_or_degraded",
        }
    }
}

/// Dominant newline mode detected at open time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NewlineModeDetected {
    Lf,
    Crlf,
    CrOnly,
    Mixed,
    UnknownOrDegraded,
}

impl NewlineModeDetected {
    /// Returns the stable string vocabulary for this newline mode.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lf => "lf",
            Self::Crlf => "crlf",
            Self::CrOnly => "cr_only",
            Self::Mixed => "mixed",
            Self::UnknownOrDegraded => "unknown_or_degraded",
        }
    }
}

/// Whether the file ended with a final newline terminator at open time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FinalNewlineDetected {
    Present,
    Absent,
    UnknownOrDegraded,
}

impl FinalNewlineDetected {
    /// Returns the stable string vocabulary for this final-newline posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Absent => "absent",
            Self::UnknownOrDegraded => "unknown_or_degraded",
        }
    }
}

/// Whether the open target was executable (or intended to be) at open time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutableIntent {
    Executable,
    NonExecutable,
    UnknownOrDegraded,
}

impl ExecutableIntent {
    /// Returns the stable string vocabulary for this executable intent.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Executable => "executable",
            Self::NonExecutable => "non_executable",
            Self::UnknownOrDegraded => "unknown_or_degraded",
        }
    }
}

/// Source-fidelity metadata detected at open time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFidelityRecord {
    /// Encoding chosen at open time.
    pub detected_encoding: DetectedEncoding,
    /// Why that encoding was selected.
    pub detection_source: DetectionSource,
    /// Whether a BOM was present at open.
    pub bom_state_detected: BomStateDetected,
    /// Dominant newline mode detected at open.
    pub newline_mode_detected: NewlineModeDetected,
    /// Whether the file ended with a newline terminator at open.
    pub final_newline_detected: FinalNewlineDetected,
    /// Executable-bit intent captured at open, when available.
    pub executable_intent: ExecutableIntent,
}

/// Result of detecting representation metadata and decoding a file into UTF-8 bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFidelityOpenOutcome {
    /// Open-time representation metadata.
    pub record: SourceFidelityRecord,
    /// UTF-8 bytes intended for the in-memory buffer (BOM stripped when present).
    pub buffer_utf8_bytes: Option<Vec<u8>>,
}

/// Returns the open-time source-fidelity record and decoded UTF-8 bytes when available.
///
/// When decoding fails under the supported heuristics, the record reports
/// `unknown_binary_like` and `buffer_utf8_bytes` is `None`.
pub fn detect_and_decode_for_buffer(
    bytes_on_disk: &[u8],
    permission_snapshot: &PermissionSnapshot,
) -> SourceFidelityOpenOutcome {
    let executable_intent = executable_intent_from_snapshot(permission_snapshot);

    let (encoding, source, bom_len, bom_state) = detect_encoding(bytes_on_disk);
    let body = bytes_on_disk.get(bom_len..).unwrap_or(bytes_on_disk);

    let decoded = match decode_to_string(encoding, body) {
        Ok(text) => Some(text),
        Err(_) => None,
    };

    let (newline_mode, final_newline) = match decoded.as_deref() {
        Some(text) => (detect_newline_mode(text), detect_final_newline(text)),
        None => (
            NewlineModeDetected::UnknownOrDegraded,
            FinalNewlineDetected::UnknownOrDegraded,
        ),
    };

    let record = SourceFidelityRecord {
        detected_encoding: encoding,
        detection_source: source,
        bom_state_detected: bom_state,
        newline_mode_detected: newline_mode,
        final_newline_detected: final_newline,
        executable_intent,
    };

    let buffer_utf8_bytes = decoded.map(|s| s.into_bytes());
    SourceFidelityOpenOutcome {
        record,
        buffer_utf8_bytes,
    }
}

/// Encodes `buffer_utf8_bytes` back into the on-disk representation described by `record`.
///
/// The caller is responsible for gating save when `record.detected_encoding` is not a supported
/// text encoding for round-trip.
pub fn encode_for_save(
    record: &SourceFidelityRecord,
    buffer_utf8_bytes: &[u8],
) -> Result<Vec<u8>, String> {
    let text = std::str::from_utf8(buffer_utf8_bytes)
        .map_err(|err| format!("buffer bytes must be UTF-8: {err}"))?;

    match record.detected_encoding {
        DetectedEncoding::Utf8 => Ok(text.as_bytes().to_vec()),
        DetectedEncoding::Utf8Bom => {
            let mut out = Vec::with_capacity(3 + buffer_utf8_bytes.len());
            out.extend_from_slice(&[0xEF, 0xBB, 0xBF]);
            out.extend_from_slice(buffer_utf8_bytes);
            Ok(out)
        }
        DetectedEncoding::Utf16LeBom => encode_utf16(text, Endianness::Little, true),
        DetectedEncoding::Utf16BeBom => encode_utf16(text, Endianness::Big, true),
        DetectedEncoding::Utf32LeBom => encode_utf32(text, Endianness::Little, true),
        DetectedEncoding::Utf32BeBom => encode_utf32(text, Endianness::Big, true),
        DetectedEncoding::UnknownBinaryLike => Err(
            "cannot encode: open-time encoding is unknown_binary_like; decode recovery must resolve before save"
                .to_owned(),
        ),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Endianness {
    Little,
    Big,
}

fn detect_encoding(bytes: &[u8]) -> (DetectedEncoding, DetectionSource, usize, BomStateDetected) {
    if bytes.starts_with(&[0x00, 0x00, 0xFE, 0xFF]) {
        return (
            DetectedEncoding::Utf32BeBom,
            DetectionSource::Bom,
            4,
            BomStateDetected::Present,
        );
    }
    if bytes.starts_with(&[0xFF, 0xFE, 0x00, 0x00]) {
        return (
            DetectedEncoding::Utf32LeBom,
            DetectionSource::Bom,
            4,
            BomStateDetected::Present,
        );
    }
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        return (
            DetectedEncoding::Utf8Bom,
            DetectionSource::Bom,
            3,
            BomStateDetected::Present,
        );
    }
    if bytes.starts_with(&[0xFE, 0xFF]) {
        return (
            DetectedEncoding::Utf16BeBom,
            DetectionSource::Bom,
            2,
            BomStateDetected::Present,
        );
    }
    if bytes.starts_with(&[0xFF, 0xFE]) {
        return (
            DetectedEncoding::Utf16LeBom,
            DetectionSource::Bom,
            2,
            BomStateDetected::Present,
        );
    }

    if std::str::from_utf8(bytes).is_ok() {
        return (
            DetectedEncoding::Utf8,
            DetectionSource::Utf8Heuristic,
            0,
            BomStateDetected::Absent,
        );
    }

    (
        DetectedEncoding::UnknownBinaryLike,
        DetectionSource::DecodeFailedNoChoice,
        0,
        BomStateDetected::UnknownOrDegraded,
    )
}

fn decode_to_string(encoding: DetectedEncoding, body: &[u8]) -> Result<String, String> {
    match encoding {
        DetectedEncoding::Utf8 | DetectedEncoding::Utf8Bom => std::str::from_utf8(body)
            .map(|s| s.to_owned())
            .map_err(|err| err.to_string()),
        DetectedEncoding::Utf16LeBom => decode_utf16(body, Endianness::Little),
        DetectedEncoding::Utf16BeBom => decode_utf16(body, Endianness::Big),
        DetectedEncoding::Utf32LeBom => decode_utf32(body, Endianness::Little),
        DetectedEncoding::Utf32BeBom => decode_utf32(body, Endianness::Big),
        DetectedEncoding::UnknownBinaryLike => {
            Err("unknown_binary_like: no decoder chosen".to_owned())
        }
    }
}

fn decode_utf16(body: &[u8], endian: Endianness) -> Result<String, String> {
    if body.len() % 2 != 0 {
        return Err("utf16 body length is not even".to_owned());
    }
    let mut code_units: Vec<u16> = Vec::with_capacity(body.len() / 2);
    for pair in body.chunks_exact(2) {
        let value = match endian {
            Endianness::Little => u16::from_le_bytes([pair[0], pair[1]]),
            Endianness::Big => u16::from_be_bytes([pair[0], pair[1]]),
        };
        code_units.push(value);
    }
    String::from_utf16(&code_units).map_err(|err| err.to_string())
}

fn decode_utf32(body: &[u8], endian: Endianness) -> Result<String, String> {
    if body.len() % 4 != 0 {
        return Err("utf32 body length is not divisible by 4".to_owned());
    }
    let mut out = String::new();
    for quad in body.chunks_exact(4) {
        let value = match endian {
            Endianness::Little => u32::from_le_bytes([quad[0], quad[1], quad[2], quad[3]]),
            Endianness::Big => u32::from_be_bytes([quad[0], quad[1], quad[2], quad[3]]),
        };
        let ch = char::from_u32(value)
            .ok_or_else(|| format!("utf32 contains invalid code point: 0x{value:08X}"))?;
        out.push(ch);
    }
    Ok(out)
}

fn encode_utf16(text: &str, endian: Endianness, include_bom: bool) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    if include_bom {
        match endian {
            Endianness::Little => out.extend_from_slice(&[0xFF, 0xFE]),
            Endianness::Big => out.extend_from_slice(&[0xFE, 0xFF]),
        }
    }
    for unit in text.encode_utf16() {
        let bytes = match endian {
            Endianness::Little => unit.to_le_bytes(),
            Endianness::Big => unit.to_be_bytes(),
        };
        out.extend_from_slice(&bytes);
    }
    Ok(out)
}

fn encode_utf32(text: &str, endian: Endianness, include_bom: bool) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    if include_bom {
        match endian {
            Endianness::Little => out.extend_from_slice(&[0xFF, 0xFE, 0x00, 0x00]),
            Endianness::Big => out.extend_from_slice(&[0x00, 0x00, 0xFE, 0xFF]),
        }
    }
    for ch in text.chars() {
        let value = ch as u32;
        let bytes = match endian {
            Endianness::Little => value.to_le_bytes(),
            Endianness::Big => value.to_be_bytes(),
        };
        out.extend_from_slice(&bytes);
    }
    Ok(out)
}

fn detect_newline_mode(text: &str) -> NewlineModeDetected {
    let bytes = text.as_bytes();
    let mut i = 0usize;
    let mut lf = 0u64;
    let mut crlf = 0u64;
    let mut cr = 0u64;

    while i < bytes.len() {
        match bytes[i] {
            b'\n' => {
                lf += 1;
                i += 1;
            }
            b'\r' => {
                if i + 1 < bytes.len() && bytes[i + 1] == b'\n' {
                    crlf += 1;
                    i += 2;
                } else {
                    cr += 1;
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }

    let present = (lf > 0) as u8 + (crlf > 0) as u8 + (cr > 0) as u8;
    if present == 0 {
        return NewlineModeDetected::Lf;
    }
    if present == 1 {
        if crlf > 0 {
            return NewlineModeDetected::Crlf;
        }
        if cr > 0 {
            return NewlineModeDetected::CrOnly;
        }
        return NewlineModeDetected::Lf;
    }
    NewlineModeDetected::Mixed
}

fn detect_final_newline(text: &str) -> FinalNewlineDetected {
    if text.ends_with("\r\n") || text.ends_with('\n') || text.ends_with('\r') {
        return FinalNewlineDetected::Present;
    }
    FinalNewlineDetected::Absent
}

fn executable_intent_from_snapshot(snapshot: &PermissionSnapshot) -> ExecutableIntent {
    let mode_str = snapshot.mode.trim();
    let Ok(mode) = u32::from_str_radix(mode_str, 8) else {
        return ExecutableIntent::UnknownOrDegraded;
    };
    if mode & 0o111 != 0 {
        ExecutableIntent::Executable
    } else {
        ExecutableIntent::NonExecutable
    }
}
