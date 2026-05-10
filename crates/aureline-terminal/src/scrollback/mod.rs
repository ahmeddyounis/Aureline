//! Bounded, redaction-aware scrollback store for one terminal session.
//!
//! The scrollback ring is the canonical durable record of what a terminal
//! session has emitted, scoped to one [`PtySessionId`]. The shell terminal
//! pane, support / export bundles, and the restore-as-transcript projection
//! all read from the same store; surfaces never carry private buffers.
//!
//! ## Why bounded and redaction-aware
//!
//! The terminal-truth contract under `/docs/execution/terminal_truth_contract.md`
//! and the seed schema under
//! `/schemas/terminal/session_restore_metadata.schema.json` forbid raw PTY
//! bytes, raw escape sequences, raw command lines, and raw clipboard bytes
//! from crossing any boundary. Every recorded line therefore carries:
//!
//! - a typed [`ScrollbackRedactionClass`] taken from the contract vocabulary,
//! - a `byte_length` so consumers can disclose the volume without re-rendering
//!   bytes,
//! - a stable digest that lets a support packet compare two captures of the
//!   same session without quoting plaintext, and
//! - an optional `text` body that is only populated for redaction classes
//!   broader than `metadata_and_hashes_only`.
//!
//! The ring drops oldest lines once it reaches its bound and exposes a
//! `dropped_line_count` so the chrome never lies about how much history is
//! visible.
//!
//! ## Failure-drill posture
//!
//! When a session ends, quarantines, or loses transport, the scrollback
//! remains addressable through the host so the restore-as-transcript projection
//! can reopen it without ever silently rerunning a command. The store cannot
//! re-execute, replay, or reissue a recorded line; it only quotes what was
//! observed.

use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

use crate::pty_host::PtySessionId;

/// Stable record-kind tag carried in serialized scrollback line records.
pub const SCROLLBACK_LINE_RECORD_KIND: &str = "terminal_scrollback_line_record";
/// Stable record-kind tag carried in serialized scrollback snapshots.
pub const SCROLLBACK_SNAPSHOT_RECORD_KIND: &str = "terminal_scrollback_snapshot_record";
/// Schema version for the scrollback record family.
pub const SCROLLBACK_SCHEMA_VERSION: u32 = 1;

/// Default maximum number of scrollback lines retained per session.
///
/// The bound is intentionally small for the seed surface: enough to carry
/// recent context after a restart, not enough to hide a runaway buffer. Live
/// callers may override with [`ScrollbackBound::custom`].
pub const DEFAULT_SCROLLBACK_LINE_BOUND: usize = 1024;

/// Frozen redaction-class vocabulary. Mirrors the
/// `redaction_class` enum in
/// `schemas/terminal/session_restore_metadata.schema.json` and the export-
/// review record family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScrollbackRedactionClass {
    /// Records carry only metadata (line index, byte length, digest). The
    /// optional text body MUST be `None` at this class.
    MetadataAndHashesOnly,
    /// Records carry text scoped to a support bundle the user explicitly
    /// approved. The text body is admitted in support-bundle-scoped form.
    SupportBundleScoped,
    /// Records carry a broadened capture body. Requires an explicit provenance
    /// record at the export boundary; the seed never uses this class
    /// implicitly.
    BroadenedCapture,
}

impl ScrollbackRedactionClass {
    /// Stable string token used in records, fixtures, and a11y exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataAndHashesOnly => "metadata_and_hashes_only",
            Self::SupportBundleScoped => "support_bundle_scoped",
            Self::BroadenedCapture => "broadened_capture",
        }
    }

    /// True when records of this class MUST omit the plaintext body.
    pub const fn requires_metadata_only(self) -> bool {
        matches!(self, Self::MetadataAndHashesOnly)
    }
}

/// Configurable upper bound for one scrollback ring.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScrollbackBound(usize);

impl ScrollbackBound {
    /// Default bound (`DEFAULT_SCROLLBACK_LINE_BOUND`).
    pub const fn default_bound() -> Self {
        Self(DEFAULT_SCROLLBACK_LINE_BOUND)
    }

    /// Custom bound. A bound of zero is normalized to one so the ring always
    /// retains the most recent observation.
    pub const fn custom(max_lines: usize) -> Self {
        Self(if max_lines == 0 { 1 } else { max_lines })
    }

    /// Maximum number of lines the ring will retain.
    pub const fn max_lines(self) -> usize {
        self.0
    }
}

impl Default for ScrollbackBound {
    fn default() -> Self {
        Self::default_bound()
    }
}

/// One redaction-aware line record retained in the scrollback ring.
///
/// The record is the canonical truth a transcript packet, a support bundle,
/// or a restored-as-transcript projection quotes. It never carries raw PTY
/// bytes; the optional `text` body is gated by the line's redaction class and
/// is omitted entirely at `metadata_and_hashes_only`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScrollbackLineRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub line_index: u64,
    pub redaction_class: ScrollbackRedactionClass,
    pub redaction_class_token: String,
    pub byte_length: usize,
    pub digest: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub observed_at: String,
}

impl ScrollbackLineRecord {
    fn new(
        line_index: u64,
        text: &str,
        redaction_class: ScrollbackRedactionClass,
        observed_at: &str,
    ) -> Self {
        let digest = digest_line(line_index, text);
        let body = if redaction_class.requires_metadata_only() {
            None
        } else {
            Some(text.to_owned())
        };
        Self {
            record_kind: SCROLLBACK_LINE_RECORD_KIND.to_owned(),
            schema_version: SCROLLBACK_SCHEMA_VERSION,
            line_index,
            redaction_class,
            redaction_class_token: redaction_class.as_str().to_owned(),
            byte_length: text.len(),
            digest,
            text: body,
            observed_at: observed_at.to_owned(),
        }
    }
}

/// One inspectable bounded scrollback ring for a session.
///
/// The ring drops oldest lines once it reaches its bound and tracks the count
/// of dropped lines so the chrome never claims to show more history than is
/// retained.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalScrollback {
    session_id: PtySessionId,
    bound: ScrollbackBound,
    next_line_index: u64,
    dropped_line_count: u64,
    lines: VecDeque<ScrollbackLineRecord>,
}

impl TerminalScrollback {
    /// Construct an empty scrollback ring with the default bound.
    pub fn new(session_id: PtySessionId) -> Self {
        Self::with_bound(session_id, ScrollbackBound::default_bound())
    }

    /// Construct an empty scrollback ring with a custom bound.
    pub fn with_bound(session_id: PtySessionId, bound: ScrollbackBound) -> Self {
        Self {
            session_id,
            bound,
            next_line_index: 0,
            dropped_line_count: 0,
            lines: VecDeque::with_capacity(bound.max_lines().min(64)),
        }
    }

    /// Returns the session this ring belongs to.
    pub fn session_id(&self) -> &PtySessionId {
        &self.session_id
    }

    /// Returns the configured ring bound.
    pub fn bound(&self) -> ScrollbackBound {
        self.bound
    }

    /// Number of lines currently retained.
    pub fn len(&self) -> usize {
        self.lines.len()
    }

    /// True when no line has been recorded.
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }

    /// Total number of lines dropped because the ring reached its bound.
    pub fn dropped_line_count(&self) -> u64 {
        self.dropped_line_count
    }

    /// Total number of lines observed across the lifetime of the ring,
    /// including lines that have since been dropped.
    pub fn observed_line_count(&self) -> u64 {
        self.next_line_index
    }

    /// Iterate the retained lines in insertion order.
    pub fn lines(&self) -> impl Iterator<Item = &ScrollbackLineRecord> {
        self.lines.iter()
    }

    /// Append one observed line. The redaction class controls whether the
    /// plaintext body is retained; `metadata_and_hashes_only` keeps only the
    /// digest and byte length.
    pub fn record_line(
        &mut self,
        text: &str,
        redaction_class: ScrollbackRedactionClass,
        observed_at: &str,
    ) -> ScrollbackLineRecord {
        let record =
            ScrollbackLineRecord::new(self.next_line_index, text, redaction_class, observed_at);
        self.next_line_index = self.next_line_index.saturating_add(1);
        self.lines.push_back(record.clone());
        while self.lines.len() > self.bound.max_lines() {
            self.lines.pop_front();
            self.dropped_line_count = self.dropped_line_count.saturating_add(1);
        }
        record
    }

    /// Snapshot the retained ring into a serializable record. The snapshot is
    /// the canonical input the restore-as-transcript projection consumes after
    /// a restart.
    pub fn snapshot(&self, captured_at: &str) -> TerminalScrollbackSnapshot {
        TerminalScrollbackSnapshot {
            record_kind: SCROLLBACK_SNAPSHOT_RECORD_KIND.to_owned(),
            schema_version: SCROLLBACK_SCHEMA_VERSION,
            session_id: self.session_id.clone(),
            bound: self.bound,
            observed_line_count: self.next_line_index,
            dropped_line_count: self.dropped_line_count,
            captured_at: captured_at.to_owned(),
            lines: self.lines.iter().cloned().collect(),
        }
    }
}

/// Serializable snapshot of one scrollback ring.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalScrollbackSnapshot {
    pub record_kind: String,
    pub schema_version: u32,
    pub session_id: PtySessionId,
    pub bound: ScrollbackBound,
    pub observed_line_count: u64,
    pub dropped_line_count: u64,
    pub captured_at: String,
    pub lines: Vec<ScrollbackLineRecord>,
}

impl TerminalScrollbackSnapshot {
    /// True when the ring dropped at least one line because the bound was
    /// reached. Restored surfaces use this to disclose truncation honestly.
    pub fn was_truncated(&self) -> bool {
        self.dropped_line_count > 0
    }

    /// Number of lines retained in the snapshot.
    pub fn retained_line_count(&self) -> usize {
        self.lines.len()
    }
}

/// Stable digest for one scrollback line. Lets two captures of the same
/// session compare line bodies without quoting plaintext on the support
/// boundary.
fn digest_line(line_index: u64, text: &str) -> String {
    // FNV-1a 64-bit; deterministic across builds and platforms, sufficient
    // for the seed's compare/equality use cases. Not a cryptographic hash.
    const OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
    const PRIME: u64 = 0x0000_0100_0000_01b3;
    let mut hash: u64 = OFFSET;
    for byte in line_index.to_le_bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(PRIME);
    }
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(PRIME);
    }
    format!("fnv1a64:{hash:016x}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pty_host::HostClass;

    fn session_id() -> PtySessionId {
        PtySessionId::from_parts("ws-test", HostClass::HostDesktop, 0)
    }

    #[test]
    fn metadata_only_lines_omit_plaintext_body() {
        let mut ring = TerminalScrollback::new(session_id());
        let record =
            ring.record_line("$ git status", ScrollbackRedactionClass::MetadataAndHashesOnly, "mono:0");
        assert_eq!(record.line_index, 0);
        assert_eq!(record.byte_length, "$ git status".len());
        assert_eq!(record.redaction_class_token, "metadata_and_hashes_only");
        assert!(record.text.is_none(), "metadata-only forbids plaintext body");
        assert!(record.digest.starts_with("fnv1a64:"));
        assert_eq!(ring.len(), 1);
        assert_eq!(ring.observed_line_count(), 1);
        assert_eq!(ring.dropped_line_count(), 0);
    }

    #[test]
    fn support_bundle_scoped_lines_retain_body() {
        let mut ring = TerminalScrollback::new(session_id());
        let record = ring.record_line(
            "build succeeded in 3.4s",
            ScrollbackRedactionClass::SupportBundleScoped,
            "mono:1",
        );
        assert_eq!(record.text.as_deref(), Some("build succeeded in 3.4s"));
        assert_eq!(record.redaction_class_token, "support_bundle_scoped");
    }

    #[test]
    fn ring_is_bounded_and_reports_dropped_lines() {
        let mut ring = TerminalScrollback::with_bound(session_id(), ScrollbackBound::custom(3));
        for i in 0..5u64 {
            ring.record_line(
                &format!("line {i}"),
                ScrollbackRedactionClass::SupportBundleScoped,
                "mono:0",
            );
        }
        assert_eq!(ring.len(), 3);
        assert_eq!(ring.dropped_line_count(), 2);
        assert_eq!(ring.observed_line_count(), 5);
        let retained: Vec<u64> = ring.lines().map(|line| line.line_index).collect();
        assert_eq!(retained, vec![2, 3, 4]);
    }

    #[test]
    fn snapshot_round_trips_via_serde() {
        let mut ring = TerminalScrollback::with_bound(session_id(), ScrollbackBound::custom(2));
        ring.record_line(
            "hello",
            ScrollbackRedactionClass::SupportBundleScoped,
            "mono:0",
        );
        ring.record_line(
            "world",
            ScrollbackRedactionClass::SupportBundleScoped,
            "mono:1",
        );
        ring.record_line(
            "again",
            ScrollbackRedactionClass::SupportBundleScoped,
            "mono:2",
        );
        let snapshot = ring.snapshot("mono:3");
        assert!(snapshot.was_truncated());
        assert_eq!(snapshot.retained_line_count(), 2);
        let json = serde_json::to_string(&snapshot).expect("serialize snapshot");
        let round: TerminalScrollbackSnapshot =
            serde_json::from_str(&json).expect("deserialize snapshot");
        assert_eq!(round, snapshot);
    }

    #[test]
    fn zero_bound_is_normalized_to_one_line_min() {
        let bound = ScrollbackBound::custom(0);
        assert_eq!(bound.max_lines(), 1);
    }

    #[test]
    fn digest_is_stable_for_same_input_and_index() {
        let a = digest_line(7, "foo");
        let b = digest_line(7, "foo");
        let c = digest_line(8, "foo");
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
