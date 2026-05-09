//! At-open large-file classification.
//!
//! Classification runs once per open request and returns a decision that is
//! intended to be surfaced to the user. The evaluation order is stable and
//! matches the large-file switch conditions described by the editor core ADR.
//!
//! The first matching trigger wins:
//!
//! 1. Size threshold
//! 2. Resource pressure (modeled as a soft RSS budget check)
//! 3. Content classification (binary/minified/pack suffix)
//! 4. Decode-recovery posture (user chose large-file mode after a decode failure)
//! 5. Operator override (explicit open in large-file mode)

use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Frozen mode vocabulary. Either the file went through the normal piece-tree
/// buffer path or through the constrained large-file path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileMode {
    Normal,
    LargeFile,
}

impl FileMode {
    /// Returns a stable string identifier for this mode.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::LargeFile => "large_file",
        }
    }
}

/// Frozen large-file activation-trigger vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LargeFileTrigger {
    SizeThreshold,
    ResourcePressure,
    Classification,
    DecodePosture,
    OperatorOverride,
}

impl LargeFileTrigger {
    /// Returns a stable string identifier for this trigger.
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

/// Detected byte-order mark, when present at byte 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BomKind {
    Utf8,
    Utf16Le,
    Utf16Be,
    Utf32Le,
    Utf32Be,
}

impl BomKind {
    /// Returns a stable string identifier for this BOM.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Utf8 => "utf_8",
            Self::Utf16Le => "utf_16_le",
            Self::Utf16Be => "utf_16_be",
            Self::Utf32Le => "utf_32_le",
            Self::Utf32Be => "utf_32_be",
        }
    }

    /// Detects a BOM at the start of `bytes`.
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

/// Structural sniff summary computed on a bounded file prefix.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SniffSummary {
    /// Bytes actually inspected, capped at [`ClassificationPolicy::sniff_bytes`].
    pub sniff_bytes: u64,
    /// Whether any NUL byte was observed.
    pub has_null_bytes: bool,
    /// Longest run of bytes without LF or CR observed.
    pub max_line_length_in_sniff: u64,
    /// BOM detected at byte 0, if any.
    pub bom_kind: Option<BomKind>,
    /// Ratio of bytes that are neither printable ASCII nor common whitespace,
    /// expressed as parts per thousand.
    pub non_printable_per_mille: u16,
    /// Derived heuristic flags.
    pub heuristics: SniffHeuristics,
}

/// Heuristic conclusions recorded explicitly so mode reasoning is reviewable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SniffHeuristics {
    pub looks_binary: bool,
    pub looks_minified: bool,
    pub matches_pack_suffix: bool,
}

/// Policy knobs used by the classifier.
#[derive(Debug, Clone)]
pub struct ClassificationPolicy {
    /// Size threshold above which the file enters large-file mode.
    pub large_file_size_threshold: u64,
    /// Soft RSS budget used by the resource-pressure trigger.
    pub soft_rss_budget: u64,
    /// Prefix byte count inspected for sniffing.
    pub sniff_bytes: u64,
    /// Treat NUL bytes in the sniff window as a binary signal.
    pub null_byte_marks_binary: bool,
    /// Per-mille threshold above which the sniff is treated as binary.
    pub non_printable_per_mille_marks_binary: u16,
    /// Lines longer than this within the sniff window mark the file as minified.
    pub minified_line_length: u64,
    /// Path suffixes that count as a large-file pack rule.
    pub large_file_pack_suffixes: Vec<String>,
    /// Whether the user explicitly requested large-file mode at open time.
    pub operator_override: bool,
    /// Whether decode recovery chose large-file mode after a decode failure.
    pub decode_recovery_chose_large_file: bool,
}

impl Default for ClassificationPolicy {
    fn default() -> Self {
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

/// Outcome of [`classify_file`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassificationDecision {
    pub path: PathBuf,
    pub bytes_on_disk: u64,
    pub mode: FileMode,
    /// `Some` iff `mode == LargeFile`.
    pub trigger: Option<LargeFileTrigger>,
    /// Human-readable reason suitable for in-product banners.
    pub reason: String,
    pub sniff: SniffSummary,
}

impl ClassificationDecision {
    /// Returns true when this decision activates large-file mode.
    pub fn is_large_file(&self) -> bool {
        self.mode == FileMode::LargeFile
    }
}

/// Errors surfaced by the classifier.
#[derive(Debug)]
pub enum ClassifyError {
    /// `std::fs` failed while reading file metadata or prefix bytes.
    Io(std::io::Error),
}

impl std::fmt::Display for ClassifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "io error during classification: {err}"),
        }
    }
}

impl std::error::Error for ClassifyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
        }
    }
}

impl From<std::io::Error> for ClassifyError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

/// Computes a sniff summary for `bytes`, treating the input as a bounded prefix
/// of the on-disk file.
pub fn sniff_bytes_summary(bytes: &[u8], policy: &ClassificationPolicy) -> SniffSummary {
    let bom_kind = BomKind::detect(bytes);
    let body_start = match bom_kind {
        Some(BomKind::Utf8) => 3,
        Some(BomKind::Utf16Le | BomKind::Utf16Be) => 2,
        Some(BomKind::Utf32Le | BomKind::Utf32Be) => 4,
        None => 0,
    };

    let mut sniff_bytes = 0u64;
    let mut has_null_bytes = false;
    let mut non_printable = 0u64;
    let mut max_run = 0u64;
    let mut current_run = 0u64;

    for &b in bytes.get(body_start..).unwrap_or(bytes) {
        sniff_bytes += 1;
        if b == 0 {
            has_null_bytes = true;
        }
        if b == b'\n' || b == b'\r' {
            if current_run > max_run {
                max_run = current_run;
            }
            current_run = 0;
        } else {
            current_run += 1;
        }

        if !is_printable_or_whitespace(b) {
            non_printable += 1;
        }
    }
    if current_run > max_run {
        max_run = current_run;
    }

    let per_mille = if sniff_bytes == 0 {
        0
    } else {
        ((non_printable.saturating_mul(1000)) / sniff_bytes) as u16
    };

    let heuristics = SniffHeuristics {
        looks_binary: policy.null_byte_marks_binary && has_null_bytes
            || per_mille >= policy.non_printable_per_mille_marks_binary,
        looks_minified: max_run >= policy.minified_line_length,
        matches_pack_suffix: false,
    };

    SniffSummary {
        sniff_bytes,
        has_null_bytes,
        max_line_length_in_sniff: max_run,
        bom_kind,
        non_printable_per_mille: per_mille,
        heuristics,
    }
}

fn is_printable_or_whitespace(byte: u8) -> bool {
    matches!(byte, b'\t' | b'\n' | b'\r' | 0x0C | b' ')
        || (byte.is_ascii_graphic() && byte != 0x7F)
}

fn matches_pack_suffix(path: &Path, suffixes: &[String]) -> bool {
    let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
        return false;
    };
    let lower = name.to_ascii_lowercase();
    suffixes
        .iter()
        .any(|suffix| lower.ends_with(&suffix.to_ascii_lowercase()))
}

fn bounded_prefix(path: &Path, sniff_bytes: u64) -> Result<Vec<u8>, ClassifyError> {
    if sniff_bytes == 0 {
        return Ok(Vec::new());
    }
    let mut file = fs::File::open(path)?;
    let mut buf = vec![0u8; sniff_bytes as usize];
    let mut read = 0usize;
    while read < buf.len() {
        match file.read(&mut buf[read..]) {
            Ok(0) => break,
            Ok(n) => read += n,
            Err(e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(ClassifyError::Io(e)),
        }
    }
    buf.truncate(read);
    Ok(buf)
}

/// Classifies the on-disk file at `path` under `policy`.
pub fn classify_file(
    path: &Path,
    policy: &ClassificationPolicy,
) -> Result<ClassificationDecision, ClassifyError> {
    let metadata = fs::metadata(path)?;
    let bytes_on_disk = metadata.len();
    let sniff_bytes = policy.sniff_bytes.min(bytes_on_disk);
    let sniff = bounded_prefix(path, sniff_bytes)?;
    let mut summary = sniff_bytes_summary(&sniff, policy);
    summary.heuristics.matches_pack_suffix = matches_pack_suffix(path, &policy.large_file_pack_suffixes);

    // 1) Size threshold.
    if bytes_on_disk > policy.large_file_size_threshold {
        return Ok(ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::SizeThreshold),
            reason: format!(
                "File size {} bytes exceeds large-file threshold {} bytes.",
                bytes_on_disk, policy.large_file_size_threshold
            ),
            sniff: summary,
        });
    }

    // 2) Resource pressure (conservative model).
    if bytes_on_disk.saturating_mul(2) > policy.soft_rss_budget {
        return Ok(ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::ResourcePressure),
            reason: format!(
                "Opening {} bytes would exceed the soft memory budget {} bytes.",
                bytes_on_disk, policy.soft_rss_budget
            ),
            sniff: summary,
        });
    }

    // 3) Content classification.
    if summary.heuristics.looks_binary {
        return Ok(ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::Classification),
            reason: "File appears to be binary or otherwise unsafe for full editor processing."
                .to_owned(),
            sniff: summary,
        });
    }
    if summary.heuristics.looks_minified {
        return Ok(ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::Classification),
            reason: "File appears to be minified or unusually dense; opening in a constrained viewer."
                .to_owned(),
            sniff: summary,
        });
    }
    if summary.heuristics.matches_pack_suffix {
        return Ok(ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::Classification),
            reason: "File matches a large-file pack rule; opening in a constrained viewer.".to_owned(),
            sniff: summary,
        });
    }

    // 4) Decode posture.
    if policy.decode_recovery_chose_large_file {
        return Ok(ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::DecodePosture),
            reason: "Opened in large-file mode after decode recovery.".to_owned(),
            sniff: summary,
        });
    }

    // 5) Operator override.
    if policy.operator_override {
        return Ok(ClassificationDecision {
            path: path.to_path_buf(),
            bytes_on_disk,
            mode: FileMode::LargeFile,
            trigger: Some(LargeFileTrigger::OperatorOverride),
            reason: "Opened in large-file mode by explicit user choice.".to_owned(),
            sniff: summary,
        });
    }

    Ok(ClassificationDecision {
        path: path.to_path_buf(),
        bytes_on_disk,
        mode: FileMode::Normal,
        trigger: None,
        reason: "File is eligible for the normal editor buffer path.".to_owned(),
        sniff: summary,
    })
}

