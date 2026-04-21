//! At-open file classification.
//!
//! Frozen evaluation order (matches the ADR 0003 large-file mode
//! switch conditions):
//!
//! 1. **Size threshold** — on-disk size exceeds the policy's
//!    `large_file_size_threshold`.
//! 2. **Resource pressure** — loading at the size threshold would
//!    push estimated buffer RSS above the soft budget.
//! 3. **Classification** — the file is detected as binary,
//!    minified beyond a complexity threshold, or matches a
//!    workspace-policy "large-file pack" rule (the prototype
//!    accepts an explicit set of suffixes).
//! 4. **Decode posture** — the file failed normal decoding and
//!    the user chose "open in large-file mode" from the
//!    decode-recovery surface.
//! 5. **Operator override** — the user explicitly opened the
//!    file in large-file mode.
//!
//! The first match wins; the trigger and a human-readable reason
//! ride with the resulting [`ClassificationDecision`] so support
//! bundles can replay why the mode applied.

use std::path::{Path, PathBuf};

/// Frozen mode vocabulary. Either the file went through the normal
/// piece-tree path or through the limited large-file path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileMode {
    Normal,
    LargeFile,
}

impl FileMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::LargeFile => "large_file",
        }
    }
}

/// Frozen large-file switch-trigger vocabulary. Variants are in
/// the ADR's evaluation order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LargeFileTrigger {
    SizeThreshold,
    ResourcePressure,
    Classification,
    DecodePosture,
    OperatorOverride,
}

impl LargeFileTrigger {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SizeThreshold => "size_threshold",
            Self::ResourcePressure => "resource_pressure",
            Self::Classification => "classification",
            Self::DecodePosture => "decode_posture",
            Self::OperatorOverride => "operator_override",
        }
    }
}

/// Detected byte-order-mark, if any.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BomKind {
    Utf8,
    Utf16Le,
    Utf16Be,
    Utf32Le,
    Utf32Be,
}

impl BomKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Utf8 => "utf_8",
            Self::Utf16Le => "utf_16_le",
            Self::Utf16Be => "utf_16_be",
            Self::Utf32Le => "utf_32_le",
            Self::Utf32Be => "utf_32_be",
        }
    }

    /// Detect a BOM at the start of `bytes`. Order matters: the
    /// UTF-32 BOMs share a prefix with the UTF-16 BOMs.
    pub fn detect(bytes: &[u8]) -> Option<Self> {
        if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
            return Some(Self::Utf8);
        }
        if bytes.starts_with(&[0x00, 0x00, 0xFE, 0xFF]) {
            return Some(Self::Utf32Be);
        }
        if bytes.starts_with(&[0xFF, 0xFE, 0x00, 0x00]) {
            return Some(Self::Utf32Le);
        }
        if bytes.starts_with(&[0xFE, 0xFF]) {
            return Some(Self::Utf16Be);
        }
        if bytes.starts_with(&[0xFF, 0xFE]) {
            return Some(Self::Utf16Le);
        }
        None
    }
}

/// Structural sniff summary computed once at open time on a
/// bounded prefix of the file. Counts and ratios only — no
/// rendered text, so the harness output stays byte-stable.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SniffSummary {
    /// How many bytes the sniff actually inspected. Capped at
    /// [`ClassificationPolicy::sniff_bytes`].
    pub sniff_bytes: u64,
    /// Whether any NUL byte was seen in the sniff window.
    pub has_null_bytes: bool,
    /// Longest run of bytes without LF or CR observed in the
    /// sniff window.
    pub max_line_length_in_sniff: u64,
    /// BOM detected at byte 0, if any.
    pub bom_kind: Option<BomKind>,
    /// Ratio of bytes that are neither printable ASCII nor a
    /// well-known whitespace byte, expressed as parts per
    /// thousand. `0` means the sniff was empty or fully
    /// printable.
    pub non_printable_per_mille: u16,
    /// Heuristic conclusions derived from the counts above.
    pub heuristics: SniffHeuristics,
}

/// Heuristic conclusions from a sniff. Recorded explicitly so the
/// classifier's reasoning is reviewable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SniffHeuristics {
    pub looks_binary: bool,
    pub looks_minified: bool,
    pub matches_pack_suffix: bool,
}

/// Classifier policy. Workspaces tune these knobs; the prototype
/// exposes them so tests can drive every trigger deterministically.
#[derive(Debug, Clone)]
pub struct ClassificationPolicy {
    /// On-disk size threshold above which a file enters
    /// large-file mode by trigger 1.
    pub large_file_size_threshold: u64,
    /// Soft RSS budget, in bytes, that the resource-pressure
    /// trigger compares against. The prototype models pressure as
    /// `2 * file_size > soft_rss_budget`; production swaps this
    /// for a real resource-manager probe behind the same trigger
    /// id.
    pub soft_rss_budget: u64,
    /// Number of bytes to read for the sniff window. Bounded so a
    /// pathological file does not gate open latency.
    pub sniff_bytes: u64,
    /// Treat any sniffed file with a NUL byte in the window as
    /// binary.
    pub null_byte_marks_binary: bool,
    /// Per-mille threshold above which the sniff is considered
    /// binary.
    pub non_printable_per_mille_marks_binary: u16,
    /// Lines longer than this in the sniff window mark the file
    /// as minified.
    pub minified_line_length: u64,
    /// Path-suffix matches that count as a workspace-policy
    /// "large-file pack". Compared case-insensitively after the
    /// last `.`.
    pub large_file_pack_suffixes: Vec<String>,
    /// Whether the user passed an operator override at open time.
    pub operator_override: bool,
    /// Whether the prior decode attempt failed and the user
    /// chose "open in large-file mode".
    pub decode_recovery_chose_large_file: bool,
}

impl Default for ClassificationPolicy {
    fn default() -> Self {
        // ADR-aligned defaults: 100 MiB size threshold; a generous
        // soft RSS budget that production replaces with a real
        // probe; sniff window large enough to catch single-line
        // minified bundles without paging in megabytes; common
        // pack-suffix examples drawn from the ADR's "minified
        // pack" wording.
        Self {
            large_file_size_threshold: 100 * 1024 * 1024,
            soft_rss_budget: 8 * 1024 * 1024 * 1024,
            sniff_bytes: 64 * 1024,
            null_byte_marks_binary: true,
            non_printable_per_mille_marks_binary: 200,
            minified_line_length: 8 * 1024,
            large_file_pack_suffixes: vec![
                "min.js".to_owned(),
                "min.css".to_owned(),
                "min.map".to_owned(),
                "bundle.js".to_owned(),
                "wasm".to_owned(),
            ],
            operator_override: false,
            decode_recovery_chose_large_file: false,
        }
    }
}

/// Outcome of [`classify_bytes`] / [`classify_file`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassificationDecision {
    pub path: PathBuf,
    pub bytes_on_disk: u64,
    pub mode: FileMode,
    /// `Some` iff `mode == LargeFile`.
    pub trigger: Option<LargeFileTrigger>,
    pub reason: String,
    pub sniff: SniffSummary,
}

impl ClassificationDecision {
    pub fn is_large_file(&self) -> bool {
        self.mode == FileMode::LargeFile
    }
}

/// Errors the classifier surfaces back to callers.
#[derive(Debug)]
pub enum ClassifyError {
    /// `std::fs` reported an I/O error while sizing or sniffing.
    Io(std::io::Error),
}

impl std::fmt::Display for ClassifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "io error during classification: {e}"),
        }
    }
}

impl std::error::Error for ClassifyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
        }
    }
}

impl From<std::io::Error> for ClassifyError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e)
    }
}

/// Compute the sniff summary for `bytes`, treating the input as
/// the bounded prefix of a real file.
pub fn sniff_bytes_summary(bytes: &[u8], policy: &ClassificationPolicy) -> SniffSummary {
    let bom_kind = BomKind::detect(bytes);
    let body_start = match bom_kind {
        Some(BomKind::Utf8) => 3,
        Some(BomKind::Utf16Le | BomKind::Utf16Be) => 2,
        Some(BomKind::Utf32Le | BomKind::Utf32Be) => 4,
        None => 0,
    }
    .min(bytes.len());
    let body = &bytes[body_start..];

    let mut has_null = false;
    let mut non_printable: u64 = 0;
    let mut current_line: u64 = 0;
    let mut max_line: u64 = 0;
    for &byte in body {
        if byte == 0x00 {
            has_null = true;
        }
        let printable = matches!(byte, 0x09 | 0x0A | 0x0D | 0x20..=0x7E);
        if !printable && byte != 0x0A && byte != 0x0D {
            // The whole printable-ASCII test already excludes
            // 0x0A and 0x0D; this `if` is a defensive belt-and-
            // braces so the "non-printable" count below tracks
            // bytes outside the printable + whitespace set.
            non_printable += 1;
        }
        if byte == b'\n' || byte == b'\r' {
            if current_line > max_line {
                max_line = current_line;
            }
            current_line = 0;
        } else {
            current_line += 1;
        }
    }
    if current_line > max_line {
        max_line = current_line;
    }

    let denom = body.len() as u64;
    let per_mille: u16 = if denom == 0 {
        0
    } else {
        // Cap at 1000 — non_printable can never exceed denom but
        // be defensive in case the loop above grows.
        ((non_printable.saturating_mul(1000)) / denom).min(1000) as u16
    };

    let looks_binary = (policy.null_byte_marks_binary && has_null)
        || per_mille >= policy.non_printable_per_mille_marks_binary;
    let looks_minified = max_line >= policy.minified_line_length;

    SniffSummary {
        sniff_bytes: bytes.len() as u64,
        has_null_bytes: has_null,
        max_line_length_in_sniff: max_line,
        bom_kind,
        non_printable_per_mille: per_mille,
        heuristics: SniffHeuristics {
            looks_binary,
            looks_minified,
            matches_pack_suffix: false,
        },
    }
}

/// Classify a file already loaded into memory. Used by tests and
/// the deterministic harness so behaviour does not depend on real
/// disk layout.
pub fn classify_bytes(
    path: &Path,
    bytes: &[u8],
    policy: &ClassificationPolicy,
) -> ClassificationDecision {
    let bytes_on_disk = bytes.len() as u64;
    let sniff_window = bytes
        .get(..policy.sniff_bytes.min(bytes_on_disk) as usize)
        .unwrap_or(bytes);
    let mut sniff = sniff_bytes_summary(sniff_window, policy);
    sniff.heuristics.matches_pack_suffix = matches_pack_suffix(path, &policy.large_file_pack_suffixes);
    decide(path, bytes_on_disk, sniff, policy)
}

/// Classify a file by reading the bounded sniff prefix from disk.
pub fn classify_file(
    path: &Path,
    policy: &ClassificationPolicy,
) -> Result<ClassificationDecision, ClassifyError> {
    let metadata = std::fs::metadata(path)?;
    let bytes_on_disk = metadata.len();
    let sniff_window = read_sniff_prefix(path, bytes_on_disk, policy.sniff_bytes)?;
    let mut sniff = sniff_bytes_summary(&sniff_window, policy);
    sniff.heuristics.matches_pack_suffix =
        matches_pack_suffix(path, &policy.large_file_pack_suffixes);
    Ok(decide(path, bytes_on_disk, sniff, policy))
}

fn read_sniff_prefix(
    path: &Path,
    bytes_on_disk: u64,
    sniff_cap: u64,
) -> std::io::Result<Vec<u8>> {
    use std::io::Read;
    let cap = sniff_cap.min(bytes_on_disk) as usize;
    if cap == 0 {
        return Ok(Vec::new());
    }
    let mut file = std::fs::File::open(path)?;
    let mut buf = vec![0u8; cap];
    let mut total = 0usize;
    while total < cap {
        match file.read(&mut buf[total..]) {
            Ok(0) => break,
            Ok(n) => total += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    buf.truncate(total);
    Ok(buf)
}

fn matches_pack_suffix(path: &Path, suffixes: &[String]) -> bool {
    let name = match path.file_name().and_then(|s| s.to_str()) {
        Some(n) => n.to_ascii_lowercase(),
        None => return false,
    };
    suffixes
        .iter()
        .any(|suffix| name.ends_with(&suffix.to_ascii_lowercase()))
}

fn decide(
    path: &Path,
    bytes_on_disk: u64,
    sniff: SniffSummary,
    policy: &ClassificationPolicy,
) -> ClassificationDecision {
    // Trigger order is frozen: size → resource → classification →
    // decode posture → operator override. First match wins; later
    // triggers do not promote past an earlier hit.
    if bytes_on_disk > policy.large_file_size_threshold {
        return ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::SizeThreshold),
            reason: format!(
                "on-disk size {bytes_on_disk} bytes exceeds policy threshold {} bytes",
                policy.large_file_size_threshold
            ),
            sniff,
        };
    }
    // Crude resource-pressure model: a buffer that mirrors the
    // file plus an equal-sized journal would push past the soft
    // budget. Production replaces this with a real resource-
    // manager probe behind the same trigger id.
    if bytes_on_disk.saturating_mul(2) > policy.soft_rss_budget && bytes_on_disk > 0 {
        return ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::ResourcePressure),
            reason: format!(
                "estimated buffer footprint (2 * {bytes_on_disk} bytes) exceeds soft rss budget {} bytes",
                policy.soft_rss_budget
            ),
            sniff,
        };
    }
    if sniff.heuristics.looks_binary {
        return ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::Classification),
            reason: format!(
                "sniff classified file as binary (null_bytes={}, non_printable_per_mille={})",
                sniff.has_null_bytes, sniff.non_printable_per_mille
            ),
            sniff,
        };
    }
    if sniff.heuristics.looks_minified {
        return ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::Classification),
            reason: format!(
                "sniff classified file as minified (max_line_length_in_sniff={} >= threshold {})",
                sniff.max_line_length_in_sniff, policy.minified_line_length
            ),
            sniff,
        };
    }
    if sniff.heuristics.matches_pack_suffix {
        return ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::Classification),
            reason: "path matches a workspace 'large-file pack' suffix".to_owned(),
            sniff,
        };
    }
    if policy.decode_recovery_chose_large_file {
        return ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::DecodePosture),
            reason:
                "decode recovery resolved with 'open in large-file mode' from the user surface"
                    .to_owned(),
            sniff,
        };
    }
    if policy.operator_override {
        return ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::OperatorOverride),
            reason: "user opened the file in large-file mode explicitly".to_owned(),
            sniff,
        };
    }
    ClassificationDecision {
        path: path.to_path_buf(),
        bytes_on_disk,
        mode: FileMode::Normal,
        trigger: None,
        reason: format!(
            "no large-file trigger fired (size {bytes_on_disk} <= threshold {}, sniff clean)",
            policy.large_file_size_threshold
        ),
        sniff,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn pol() -> ClassificationPolicy {
        ClassificationPolicy::default()
    }

    #[test]
    fn empty_file_classifies_normal() {
        let d = classify_bytes(&PathBuf::from("empty.txt"), b"", &pol());
        assert_eq!(d.mode, FileMode::Normal);
        assert!(d.trigger.is_none());
    }

    #[test]
    fn small_text_classifies_normal() {
        let d = classify_bytes(
            &PathBuf::from("hello.rs"),
            b"fn main() { println!(\"hello\"); }\n",
            &pol(),
        );
        assert_eq!(d.mode, FileMode::Normal);
        assert!(d.trigger.is_none());
        assert_eq!(d.sniff.bom_kind, None);
        assert!(!d.sniff.heuristics.looks_binary);
        assert!(!d.sniff.heuristics.looks_minified);
    }

    #[test]
    fn size_threshold_fires_first_even_when_other_signals_apply() {
        let mut p = pol();
        p.large_file_size_threshold = 8;
        // Bytes are also "binary" (NUL inside) and would trip
        // classification, but size_threshold MUST win because it
        // is evaluated first.
        let d = classify_bytes(
            &PathBuf::from("blob.bin"),
            b"abcdefghij\0klmnop",
            &p,
        );
        assert_eq!(d.mode, FileMode::LargeFile);
        assert_eq!(d.trigger, Some(LargeFileTrigger::SizeThreshold));
    }

    #[test]
    fn null_byte_marks_binary_and_classification_fires() {
        let bytes = b"hello\0world\n";
        let d = classify_bytes(&PathBuf::from("blob.dat"), bytes, &pol());
        assert_eq!(d.mode, FileMode::LargeFile);
        assert_eq!(d.trigger, Some(LargeFileTrigger::Classification));
        assert!(d.sniff.has_null_bytes);
        assert!(d.sniff.heuristics.looks_binary);
    }

    #[test]
    fn long_single_line_is_minified() {
        let mut p = pol();
        p.minified_line_length = 64;
        let bytes: Vec<u8> = std::iter::repeat(b'a').take(200).collect();
        let d = classify_bytes(&PathBuf::from("bundle.js"), &bytes, &p);
        assert_eq!(d.mode, FileMode::LargeFile);
        assert_eq!(d.trigger, Some(LargeFileTrigger::Classification));
        assert!(d.sniff.heuristics.looks_minified);
    }

    #[test]
    fn pack_suffix_routes_to_large_file_even_when_clean() {
        let bytes = b"// small but flagged as a pack\nlet a = 1;\n";
        let d = classify_bytes(&PathBuf::from("vendor.min.js"), bytes, &pol());
        assert_eq!(d.mode, FileMode::LargeFile);
        assert_eq!(d.trigger, Some(LargeFileTrigger::Classification));
        assert!(d.sniff.heuristics.matches_pack_suffix);
    }

    #[test]
    fn decode_recovery_then_operator_override_evaluates_in_order() {
        let mut p = pol();
        p.decode_recovery_chose_large_file = true;
        p.operator_override = true;
        let d = classify_bytes(&PathBuf::from("clean.txt"), b"plain text", &p);
        assert_eq!(d.mode, FileMode::LargeFile);
        // Decode posture must fire before operator override.
        assert_eq!(d.trigger, Some(LargeFileTrigger::DecodePosture));
    }

    #[test]
    fn operator_override_alone_routes_to_large_file() {
        let mut p = pol();
        p.operator_override = true;
        let d = classify_bytes(&PathBuf::from("clean.txt"), b"plain text", &p);
        assert_eq!(d.mode, FileMode::LargeFile);
        assert_eq!(d.trigger, Some(LargeFileTrigger::OperatorOverride));
    }

    #[test]
    fn bom_detection_reports_each_kind() {
        assert_eq!(
            BomKind::detect(&[0xEF, 0xBB, 0xBF, b'a']),
            Some(BomKind::Utf8)
        );
        assert_eq!(BomKind::detect(&[0xFE, 0xFF, 0, 0]), Some(BomKind::Utf16Be));
        assert_eq!(BomKind::detect(&[0xFF, 0xFE, 0, 0]), Some(BomKind::Utf32Le));
        assert_eq!(BomKind::detect(&[0xFF, 0xFE, b'a']), Some(BomKind::Utf16Le));
        assert_eq!(
            BomKind::detect(&[0x00, 0x00, 0xFE, 0xFF]),
            Some(BomKind::Utf32Be)
        );
        assert_eq!(BomKind::detect(b"hello"), None);
    }

    #[test]
    fn classification_decision_explains_itself() {
        let mut p = pol();
        p.large_file_size_threshold = 4;
        let d = classify_bytes(&PathBuf::from("big.txt"), b"hello world", &p);
        assert!(d.reason.contains("exceeds policy threshold"));
        assert!(d.is_large_file());
    }

    #[test]
    fn classify_file_round_trips_through_disk() {
        let dir = tempdir();
        let path = dir.join("clean.txt");
        std::fs::write(&path, b"clean small text\n").unwrap();
        let d = classify_file(&path, &pol()).unwrap();
        assert_eq!(d.mode, FileMode::Normal);
        assert_eq!(d.bytes_on_disk, "clean small text\n".len() as u64);
        cleanup(dir);
    }

    fn tempdir() -> PathBuf {
        let mut p = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        p.push(format!("aureline-largefile-proto-{nanos}-{}", std::process::id()));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn cleanup(p: PathBuf) {
        let _ = std::fs::remove_dir_all(p);
    }
}
