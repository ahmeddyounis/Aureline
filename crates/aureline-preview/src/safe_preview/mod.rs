//! Representation-labeled safe preview record.
//!
//! See the crate-level docs for what this wedge owns and why. This module
//! defines the canonical [`SafePreviewRecord`] data shape, the closed
//! vocabularies the chrome quotes verbatim, the three named builders that
//! cover the M1 lanes ([`build_risky_text_preview`],
//! [`build_oversized_artifact_preview`], [`build_generated_content_preview`]),
//! and the [`SafePreviewRecord::validate`] check that surfaces every
//! representation-honesty rule the spec freezes.

use serde::{Deserialize, Serialize};

use aureline_content_safety::{
    BodyPosture, RepresentationActionId, RepresentationClass, SuspiciousContentDetection,
    TrustClass,
};

/// Stable record-kind tag carried in serialized safe-preview payloads.
pub const SAFE_PREVIEW_RECORD_KIND: &str = "safe_preview_record";

/// Schema version for the [`SafePreviewRecord`] payload shape.
pub const SAFE_PREVIEW_SCHEMA_VERSION: u32 = 1;

/// Prototype-label vocabulary carried on every preview. The chrome quotes the
/// token verbatim; surfaces MUST NOT drop the chip even when the preview is
/// nominally clean, because the wedge as a whole is a bounded prototype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrototypeLabel {
    /// Bounded M1 prototype of the representation-labeled safe-preview and
    /// copy/export wedge. Covers risky text, oversized artifacts, and
    /// generated content on one live shell row. Not a universal content
    /// viewer.
    M1PrototypeSafePreviewAndCopyExport,
}

impl PrototypeLabel {
    /// Stable string token recorded on the preview row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M1PrototypeSafePreviewAndCopyExport => {
                "m1_prototype_safe_preview_and_copy_export"
            }
        }
    }

    /// Human-readable chip label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::M1PrototypeSafePreviewAndCopyExport => {
                "Prototype — safe preview & copy/export (risky / oversized / generated)"
            }
        }
    }
}

/// Closed content-class vocabulary. Names which of the three M1 lanes a
/// preview belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentClass {
    /// Plain UTF-8 text that the shared detector flagged for bidi controls,
    /// invisible formatting, or mixed-script confusables.
    RiskyText,
    /// File / log / capture whose visible body is only a slice of the source.
    OversizedArtifact,
    /// Model-produced summary / diff / explanation.
    GeneratedContent,
}

impl ContentClass {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RiskyText => "risky_text",
            Self::OversizedArtifact => "oversized_artifact",
            Self::GeneratedContent => "generated_content",
        }
    }

    /// Human-readable label suitable for the chip body.
    pub const fn label(self) -> &'static str {
        match self {
            Self::RiskyText => "Risky text",
            Self::OversizedArtifact => "Oversized artifact",
            Self::GeneratedContent => "Generated content",
        }
    }
}

/// Closed origin / provenance class. Mirrors the schema vocabulary in
/// `/schemas/ux/representation_copy_export.schema.json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginClass {
    UserAuthoredOrImported,
    Generated,
    Unknown,
}

impl OriginClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAuthoredOrImported => "user_authored_or_imported",
            Self::Generated => "generated",
            Self::Unknown => "unknown",
        }
    }
}

/// Closed copy/export action-kind vocabulary. Mirrors
/// `representation_copy_export.schema.json` so the wedge cannot mint a
/// surface-local synonym.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyExportActionKind {
    Copy,
    Export,
}

impl CopyExportActionKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Export => "export",
        }
    }

    /// Action-kind associated with a transfer action id.
    pub const fn for_action(action: RepresentationActionId) -> Self {
        match action {
            RepresentationActionId::CopyRaw
            | RepresentationActionId::CopyRendered
            | RepresentationActionId::CopyEscaped => Self::Copy,
            RepresentationActionId::ExportSanitizedSnapshot
            | RepresentationActionId::ExportMetadataOnly => Self::Export,
        }
    }
}

/// On-screen representation the user is currently looking at. This is a
/// superset of the transfer-safe [`RepresentationClass`]: in addition to
/// `raw`, `rendered`, `escaped`, `sanitized`, and `blocked_metadata_only`,
/// the on-screen state can read `sandboxed` (for isolated remote content)
/// and `generated` (for model output). Transfers still use the
/// frozen [`RepresentationClass`] vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CurrentlyVisibleRepresentation {
    Raw,
    Rendered,
    Escaped,
    Sanitized,
    Sandboxed,
    Generated,
    BlockedMetadataOnly,
}

impl CurrentlyVisibleRepresentation {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Raw => "raw",
            Self::Rendered => "rendered",
            Self::Escaped => "escaped",
            Self::Sanitized => "sanitized",
            Self::Sandboxed => "sandboxed",
            Self::Generated => "generated",
            Self::BlockedMetadataOnly => "blocked_metadata_only",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Raw => "Raw source",
            Self::Rendered => "Rendered preview",
            Self::Escaped => "Escaped (safe inspection)",
            Self::Sanitized => "Sanitized snapshot",
            Self::Sandboxed => "Sandboxed render",
            Self::Generated => "Generated / derived",
            Self::BlockedMetadataOnly => "Body withheld — metadata only",
        }
    }
}

/// Closed scope-class vocabulary frozen by the parity schema. Names what
/// subset of the subject the copy/export payload covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClass {
    AnchoredSelection,
    VisibleRowsOrEvents,
    LoadedMaterializedSet,
    NamedSnapshotOnly,
    ProviderRawDownload,
    MetadataOnly,
}

impl ScopeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AnchoredSelection => "anchored_selection",
            Self::VisibleRowsOrEvents => "visible_rows_or_events",
            Self::LoadedMaterializedSet => "loaded_materialized_set",
            Self::NamedSnapshotOnly => "named_snapshot_only",
            Self::ProviderRawDownload => "provider_raw_download",
            Self::MetadataOnly => "metadata_only",
        }
    }
}

/// Closed transform-kind vocabulary frozen by the parity schema. Names how
/// the copy/export payload differs from the source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransformKind {
    None,
    SanitizedActiveContentRemoved,
    EscapedControlsOrMetacharacters,
    TruncatedOrWindowed,
    BufferedUnseenExcluded,
    DecodedWithReplacement,
    NormalizedNewlines,
    StrippedAnsiOrControlSequences,
    ExcerptedFromBinary,
    RedactedHighRiskSegments,
    SummarizedOrSampled,
}

impl TransformKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SanitizedActiveContentRemoved => "sanitized_active_content_removed",
            Self::EscapedControlsOrMetacharacters => "escaped_controls_or_metacharacters",
            Self::TruncatedOrWindowed => "truncated_or_windowed",
            Self::BufferedUnseenExcluded => "buffered_unseen_excluded",
            Self::DecodedWithReplacement => "decoded_with_replacement",
            Self::NormalizedNewlines => "normalized_newlines",
            Self::StrippedAnsiOrControlSequences => "stripped_ansi_or_control_sequences",
            Self::ExcerptedFromBinary => "excerpted_from_binary",
            Self::RedactedHighRiskSegments => "redacted_high_risk_segments",
            Self::SummarizedOrSampled => "summarized_or_sampled",
        }
    }
}

/// Closed omission-reason vocabulary frozen by the parity schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OmissionReason {
    None,
    SizeBudget,
    StreamingWindow,
    BufferedUnrevealed,
    Policy,
    Redaction,
    SourceMissing,
    UnsupportedType,
    BinaryExcerptOnly,
}

impl OmissionReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SizeBudget => "size_budget",
            Self::StreamingWindow => "streaming_window",
            Self::BufferedUnrevealed => "buffered_unrevealed",
            Self::Policy => "policy",
            Self::Redaction => "redaction",
            Self::SourceMissing => "source_missing",
            Self::UnsupportedType => "unsupported_type",
            Self::BinaryExcerptOnly => "binary_excerpt_only",
        }
    }
}

/// Omission-summary record. Mirrors the parity schema's `omission_summary`
/// shape so support exports never lose track of what was excluded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmissionSummary {
    pub reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub omitted_bytes_estimate: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub omitted_line_count_estimate: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub omitted_member_count_estimate: Option<u64>,
}

impl OmissionSummary {
    /// Build a [`Self::reasons`]-only summary with no byte / line counts.
    pub fn from_reasons(reasons: &[OmissionReason]) -> Self {
        Self {
            reasons: reasons.iter().map(|r| r.as_str().to_owned()).collect(),
            omitted_bytes_estimate: None,
            omitted_line_count_estimate: None,
            omitted_member_count_estimate: None,
        }
    }

    /// True when the summary advertises a real omission (anything other than
    /// the `none` honesty sentinel).
    pub fn has_real_omission(&self) -> bool {
        self.reasons
            .iter()
            .any(|reason| reason != OmissionReason::None.as_str())
    }
}

/// Share-safety posture (`safe_for_issue_report`, `safe_for_support_bundle`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShareSafety {
    pub safe_for_issue_report: bool,
    pub safe_for_support_bundle: bool,
}

/// One claim-limit row. The chrome quotes these verbatim under the preview so
/// the wedge cannot overclaim universal-content-viewer fidelity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewClaimLimit {
    pub token: String,
    pub label: String,
}

impl SafePreviewClaimLimit {
    fn new(token: &str, label: &str) -> Self {
        Self {
            token: token.to_owned(),
            label: label.to_owned(),
        }
    }

    /// Canonical claim-limit set carried on every M1 preview. Order is
    /// stable; chrome MUST render the rows in this order.
    pub fn canonical_limits() -> Vec<Self> {
        vec![
            Self::new(
                "bounded_prototype_only",
                "Bounded M1 prototype — not a universal content viewer.",
            ),
            Self::new(
                "risky_oversized_generated_lanes_only",
                "Covers risky text, oversized artifacts, and generated content only.",
            ),
            Self::new(
                "transfer_actions_must_carry_representation",
                "Copy and export actions never imply raw source unless labeled raw.",
            ),
            Self::new(
                "no_remote_or_publish_boundary_moves",
                "Workspace-local only; no remote publish / share-bundle boundary moves.",
            ),
        ]
    }
}

/// One copy/export option on the preview row. The chrome quotes every field
/// verbatim; nothing here is a default the chrome may massage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportOption {
    /// Stable per-option id (e.g. `option:copy_raw:bidi`).
    pub option_id: String,
    /// Action kind (copy / export). Mirrors the parity schema.
    pub action_kind: String,
    /// Frozen transfer-action id (`copy_raw`, `copy_escaped`,
    /// `export_metadata_only`, etc).
    pub action_id: String,
    /// Frozen representation class for the transferred payload.
    pub representation_class: String,
    /// Frozen body-posture string for the transferred payload.
    pub body_posture: String,
    /// Human-readable label the chrome renders on the button row.
    pub label: String,
    /// Scope class for the transferred payload.
    pub scope_class: String,
    /// Transforms applied to the transferred payload. Never empty: a clean
    /// transfer uses `[none]`.
    pub transforms_applied: Vec<String>,
    /// Omission summary for the transferred payload.
    pub omission_summary: OmissionSummary,
    /// Disclosure fields the chrome MUST keep visible alongside the option
    /// (representation_label, trust_class_badge, scope_label, etc).
    pub required_disclosure_fields: Vec<String>,
    /// Paired action ids the chrome MUST keep reachable when this option is
    /// offered (e.g. `copy_raw` MUST keep `copy_escaped` reachable for risky
    /// text, and vice versa).
    pub must_offer_also: Vec<String>,
    /// Citation-anchor refs backing the payload. Required when the option's
    /// representation is `generated` or when the chrome claims
    /// `citation_anchors` disclosure.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub citation_anchor_refs: Vec<String>,
    /// Share-safety posture.
    pub share_safety: ShareSafety,
    /// Short reviewer-facing note that explains the option.
    pub notes: String,
}

/// Canonical safe-preview record. The chrome renders this struct directly;
/// the support / export pipeline quotes it verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafePreviewRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub prototype_label_token: String,
    pub prototype_label_display: String,
    /// Stable preview id (e.g. `preview:risky_text:src/lib.rs#config_loader`).
    pub preview_id: String,
    /// Opaque subject ref (file / capture / generated-summary handle). Raw
    /// bodies and secrets never appear here.
    pub source_subject_ref: String,
    pub content_class_token: String,
    pub content_class_display: String,
    pub trust_class_token: String,
    pub origin_class_token: String,
    pub source_surface_family: String,
    pub currently_visible_representation_token: String,
    pub currently_visible_representation_label: String,
    /// Total source byte count when known. Oversized previews populate this;
    /// risky text and generated previews leave it `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_byte_count: Option<u64>,
    /// Visible byte count (the slice the rendered preview actually shows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_byte_count: Option<u64>,
    /// Visible line count when applicable (oversized log/file previews).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_line_count: Option<u64>,
    /// Suspicious-content finding count for risky text previews (0 for the
    /// other content classes).
    pub suspicious_finding_count: u32,
    /// Stable summary line the chrome renders below the preview header. The
    /// support export quotes this verbatim.
    pub summary_line: String,
    /// Copy/export options the chrome surfaces. Order is stable; chrome MUST
    /// render the options in this order.
    pub copy_export_options: Vec<CopyExportOption>,
    pub claim_limits: Vec<SafePreviewClaimLimit>,
}

impl SafePreviewRecord {
    /// Render a deterministic plaintext block. Support exports and proof
    /// captures quote this verbatim — the format is stable across hosts and
    /// never bakes in wall-clock time.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] {}\n",
            self.prototype_label_token, self.prototype_label_display,
        ));
        out.push_str(&format!(
            "preview {} ({}) — content_class={} trust_class={} origin={}\n",
            self.preview_id,
            self.source_subject_ref,
            self.content_class_token,
            self.trust_class_token,
            self.origin_class_token,
        ));
        out.push_str(&format!(
            "source_surface={} currently_visible={}\n",
            self.source_surface_family, self.currently_visible_representation_token,
        ));
        if let Some(bytes) = self.total_byte_count {
            out.push_str(&format!(
                "total_bytes={} visible_bytes={} visible_lines={}\n",
                bytes,
                self.visible_byte_count.unwrap_or(0),
                self.visible_line_count.unwrap_or(0),
            ));
        }
        if self.suspicious_finding_count > 0 {
            out.push_str(&format!(
                "suspicious_findings={}\n",
                self.suspicious_finding_count
            ));
        }
        out.push_str(&format!("summary: {}\n", self.summary_line));
        out.push_str("copy_export_options:\n");
        for option in &self.copy_export_options {
            out.push_str(&format!(
                "  - {} [{} -> {} / scope={} body={}]: {}\n",
                option.option_id,
                option.action_kind,
                option.action_id,
                option.scope_class,
                option.body_posture,
                option.label,
            ));
            out.push_str(&format!(
                "      representation={} transforms={} share_safe_issue={} share_safe_support={}\n",
                option.representation_class,
                option.transforms_applied.join(","),
                option.share_safety.safe_for_issue_report,
                option.share_safety.safe_for_support_bundle,
            ));
            if !option.must_offer_also.is_empty() {
                out.push_str(&format!(
                    "      must_offer_also={}\n",
                    option.must_offer_also.join(","),
                ));
            }
            if !option.citation_anchor_refs.is_empty() {
                out.push_str(&format!(
                    "      citation_anchor_refs={}\n",
                    option.citation_anchor_refs.join(","),
                ));
            }
            if option.omission_summary.has_real_omission() {
                out.push_str(&format!(
                    "      omission_reasons={}\n",
                    option.omission_summary.reasons.join(","),
                ));
            }
            out.push_str(&format!("      notes={}\n", option.notes));
        }
        out.push_str("claim_limits:\n");
        for limit in &self.claim_limits {
            out.push_str(&format!("  - {}: {}\n", limit.token, limit.label));
        }
        out
    }

    /// Run the representation-honesty invariants over the record. Returns the
    /// list of violations the chrome MUST surface (and the wedge MUST fix)
    /// before letting copy/export proceed.
    ///
    /// Rules enforced:
    ///
    /// - Every preview MUST list at least one copy/export option. A surface
    ///   that cannot offer any representation collapses to
    ///   `export_metadata_only` rather than rendering an empty action set.
    /// - Every option's `action_kind` MUST match its `action_id` (the
    ///   `copy_*` ids pair with `copy`, the `export_*` ids pair with
    ///   `export`).
    /// - Every option MUST advertise the `representation_label` disclosure
    ///   field so the chrome cannot drop the label.
    /// - Risky-text previews MUST offer both `copy_raw` and `copy_escaped`
    ///   and each MUST keep the other in `must_offer_also` (the spec's
    ///   raw-versus-escaped parity rule).
    /// - Risky-text previews MUST also offer `export_metadata_only` so a
    ///   support export never silently includes suspicious bytes.
    /// - Generated previews MUST set `origin_class = generated`,
    ///   currently_visible_representation = `generated`, and MUST NOT
    ///   advertise `copy_raw` without supplying at least one
    ///   citation-anchor ref on that option.
    /// - Oversized previews that report `visible_byte_count <
    ///   total_byte_count` MUST attach a `truncated_or_windowed` transform
    ///   on at least one option and MUST name a scope other than
    ///   `loaded_materialized_set` on the option that surfaces the
    ///   windowed view; they MUST also publish a non-zero
    ///   `omitted_bytes_estimate` so the support export never silently
    ///   widens the slice.
    pub fn validate(&self) -> Vec<SafePreviewInvariantViolation> {
        let mut violations = Vec::new();

        if self.copy_export_options.is_empty() {
            violations.push(SafePreviewInvariantViolation::NoCopyExportOptions);
        }

        for option in &self.copy_export_options {
            let expected_kind = match option.action_id.as_str() {
                "copy_raw" | "copy_rendered" | "copy_escaped" => {
                    CopyExportActionKind::Copy.as_str()
                }
                "export_sanitized_snapshot" | "export_metadata_only" => {
                    CopyExportActionKind::Export.as_str()
                }
                _ => "",
            };
            if !expected_kind.is_empty() && option.action_kind != expected_kind {
                violations.push(SafePreviewInvariantViolation::ActionKindMismatch {
                    option_id: option.option_id.clone(),
                    expected_kind: expected_kind.to_owned(),
                    actual_kind: option.action_kind.clone(),
                });
            }
            if !option
                .required_disclosure_fields
                .iter()
                .any(|f| f == "representation_label")
            {
                violations.push(SafePreviewInvariantViolation::MissingRepresentationLabel {
                    option_id: option.option_id.clone(),
                });
            }
        }

        let has_action = |id: &str| {
            self.copy_export_options
                .iter()
                .any(|opt| opt.action_id == id)
        };

        if self.content_class_token == ContentClass::RiskyText.as_str() {
            if !has_action("copy_raw") {
                violations.push(SafePreviewInvariantViolation::MissingPairedAction {
                    content_class: ContentClass::RiskyText.as_str().to_owned(),
                    missing_action_id: "copy_raw".to_owned(),
                });
            }
            if !has_action("copy_escaped") {
                violations.push(SafePreviewInvariantViolation::MissingPairedAction {
                    content_class: ContentClass::RiskyText.as_str().to_owned(),
                    missing_action_id: "copy_escaped".to_owned(),
                });
            }
            if !has_action("export_metadata_only") {
                violations.push(SafePreviewInvariantViolation::MissingPairedAction {
                    content_class: ContentClass::RiskyText.as_str().to_owned(),
                    missing_action_id: "export_metadata_only".to_owned(),
                });
            }
            // Each of copy_raw and copy_escaped must reference the other.
            for option in &self.copy_export_options {
                if option.action_id == "copy_raw"
                    && !option.must_offer_also.iter().any(|s| s == "copy_escaped")
                {
                    violations.push(SafePreviewInvariantViolation::UnpairedRiskyTextAction {
                        option_id: option.option_id.clone(),
                        expected_peer: "copy_escaped".to_owned(),
                    });
                }
                if option.action_id == "copy_escaped"
                    && !option.must_offer_also.iter().any(|s| s == "copy_raw")
                {
                    violations.push(SafePreviewInvariantViolation::UnpairedRiskyTextAction {
                        option_id: option.option_id.clone(),
                        expected_peer: "copy_raw".to_owned(),
                    });
                }
                // The unlabeled-rendered failure: a risky-text preview
                // cannot silently offer copy_rendered as a fallback for the
                // raw or escaped row.
                if option.action_id == "copy_rendered"
                    && option.representation_class != RepresentationClass::Rendered.as_str()
                {
                    violations.push(SafePreviewInvariantViolation::UnlabeledRenderedCopy {
                        option_id: option.option_id.clone(),
                    });
                }
            }
        }

        if self.content_class_token == ContentClass::GeneratedContent.as_str() {
            if self.origin_class_token != OriginClass::Generated.as_str() {
                violations.push(SafePreviewInvariantViolation::GeneratedOriginMismatch {
                    actual_origin: self.origin_class_token.clone(),
                });
            }
            if self.currently_visible_representation_token
                != CurrentlyVisibleRepresentation::Generated.as_str()
            {
                violations.push(SafePreviewInvariantViolation::GeneratedVisibleMismatch {
                    actual_representation: self.currently_visible_representation_token.clone(),
                });
            }
            for option in &self.copy_export_options {
                if option.action_id == "copy_raw" && option.citation_anchor_refs.is_empty() {
                    violations.push(
                        SafePreviewInvariantViolation::GeneratedCopyRawWithoutCitation {
                            option_id: option.option_id.clone(),
                        },
                    );
                }
            }
        }

        if self.content_class_token == ContentClass::OversizedArtifact.as_str() {
            let visible = self.visible_byte_count.unwrap_or(0);
            let total = self.total_byte_count.unwrap_or(0);
            if visible < total {
                let mut found_windowed = false;
                let mut found_disclosed_scope = false;
                for option in &self.copy_export_options {
                    if option
                        .transforms_applied
                        .iter()
                        .any(|t| t == TransformKind::TruncatedOrWindowed.as_str())
                    {
                        found_windowed = true;
                    }
                    if option.action_kind == CopyExportActionKind::Copy.as_str()
                        && option.scope_class != ScopeClass::LoadedMaterializedSet.as_str()
                    {
                        found_disclosed_scope = true;
                    }
                }
                if !found_windowed {
                    violations.push(SafePreviewInvariantViolation::OversizedMissingWindowTransform);
                }
                if !found_disclosed_scope {
                    violations.push(SafePreviewInvariantViolation::OversizedScopeOverclaim);
                }
                let any_estimate = self
                    .copy_export_options
                    .iter()
                    .any(|opt| opt.omission_summary.omitted_bytes_estimate.unwrap_or(0) > 0);
                if !any_estimate {
                    violations.push(SafePreviewInvariantViolation::OversizedMissingOmittedBytes);
                }
            }
        }

        violations
    }
}

/// Closed invariant-violation vocabulary surfaced by
/// [`SafePreviewRecord::validate`]. Surfaces MUST quote the token verbatim
/// instead of inventing a generic warning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SafePreviewInvariantViolation {
    /// The preview has no copy/export options at all.
    NoCopyExportOptions,
    /// One option's `action_kind` disagrees with its `action_id`.
    ActionKindMismatch {
        option_id: String,
        expected_kind: String,
        actual_kind: String,
    },
    /// One option does not advertise the `representation_label` disclosure.
    MissingRepresentationLabel { option_id: String },
    /// A risky-text preview is missing one of the paired actions
    /// (`copy_raw`, `copy_escaped`, `export_metadata_only`).
    MissingPairedAction {
        content_class: String,
        missing_action_id: String,
    },
    /// A risky-text preview offers `copy_raw` or `copy_escaped` without
    /// listing the peer action in `must_offer_also`.
    UnpairedRiskyTextAction {
        option_id: String,
        expected_peer: String,
    },
    /// A risky-text preview offers `copy_rendered` with a representation
    /// class other than `rendered` — surfacing the named failure drill where
    /// rendered output could silently masquerade as raw or escaped.
    UnlabeledRenderedCopy { option_id: String },
    /// A generated-content preview reports an origin other than `generated`.
    GeneratedOriginMismatch { actual_origin: String },
    /// A generated-content preview's currently visible representation is not
    /// `generated`.
    GeneratedVisibleMismatch { actual_representation: String },
    /// A generated-content preview offers `copy_raw` without supplying a
    /// citation-anchor backing.
    GeneratedCopyRawWithoutCitation { option_id: String },
    /// An oversized preview reports a windowed body but no option carries
    /// the `truncated_or_windowed` transform.
    OversizedMissingWindowTransform,
    /// An oversized preview reports a windowed body but at least one copy
    /// option still claims `loaded_materialized_set` scope.
    OversizedScopeOverclaim,
    /// An oversized preview reports a windowed body but no option publishes
    /// a non-zero `omitted_bytes_estimate`.
    OversizedMissingOmittedBytes,
}

impl SafePreviewInvariantViolation {
    /// Stable token suitable for a row badge.
    pub fn token(&self) -> &'static str {
        match self {
            Self::NoCopyExportOptions => "no_copy_export_options",
            Self::ActionKindMismatch { .. } => "action_kind_mismatch",
            Self::MissingRepresentationLabel { .. } => "missing_representation_label",
            Self::MissingPairedAction { .. } => "missing_paired_action",
            Self::UnpairedRiskyTextAction { .. } => "unpaired_risky_text_action",
            Self::UnlabeledRenderedCopy { .. } => "unlabeled_rendered_copy",
            Self::GeneratedOriginMismatch { .. } => "generated_origin_mismatch",
            Self::GeneratedVisibleMismatch { .. } => "generated_visible_mismatch",
            Self::GeneratedCopyRawWithoutCitation { .. } => {
                "generated_copy_raw_without_citation"
            }
            Self::OversizedMissingWindowTransform => "oversized_missing_window_transform",
            Self::OversizedScopeOverclaim => "oversized_scope_overclaim",
            Self::OversizedMissingOmittedBytes => "oversized_missing_omitted_bytes",
        }
    }
}

/// Input for [`build_risky_text_preview`].
#[derive(Debug, Clone)]
pub struct RiskyTextInput {
    pub preview_id: String,
    pub source_subject_ref: String,
    pub source_surface_family: String,
    pub trust_class: TrustClass,
    pub detection: SuspiciousContentDetection,
}

/// Input for [`build_oversized_artifact_preview`].
#[derive(Debug, Clone)]
pub struct OversizedArtifactInput {
    pub preview_id: String,
    pub source_subject_ref: String,
    pub source_surface_family: String,
    pub trust_class: TrustClass,
    pub total_byte_count: u64,
    pub visible_byte_count: u64,
    pub visible_line_count: u64,
    pub omitted_bytes_estimate: u64,
    pub omitted_line_count_estimate: u64,
}

/// Input for [`build_generated_content_preview`].
#[derive(Debug, Clone)]
pub struct GeneratedContentInput {
    pub preview_id: String,
    pub source_subject_ref: String,
    pub source_surface_family: String,
    pub generator_id: String,
    pub citation_anchor_refs: Vec<String>,
    pub canonical_source_subject_ref: Option<String>,
}

/// Build a [`SafePreviewRecord`] for the risky-text lane. Caller supplies the
/// detector outcome from [`aureline_content_safety`]; this function does not
/// re-derive what is suspicious. When the detection has no findings, the
/// builder still returns a record so the chrome can keep the wedge mounted on
/// clean text; the rendered representation is `raw` and the failure-drill
/// pairs are not required.
pub fn build_risky_text_preview(input: RiskyTextInput) -> SafePreviewRecord {
    let prototype = PrototypeLabel::M1PrototypeSafePreviewAndCopyExport;
    let finding_count = input.detection.findings.len() as u32;
    let has_findings = finding_count > 0;
    let currently_visible = if has_findings {
        CurrentlyVisibleRepresentation::Escaped
    } else {
        CurrentlyVisibleRepresentation::Raw
    };

    let summary = if has_findings {
        format!(
            "{} suspicious finding(s) — rendered as escaped; raw and escaped copies stay paired.",
            finding_count
        )
    } else {
        "Detector found no risky codepoints; rendered as raw source.".to_owned()
    };

    let options = vec![
        copy_raw_option_for_risky_text(&input, has_findings),
        copy_escaped_option_for_risky_text(&input, has_findings),
        export_metadata_only_option_for_risky_text(&input),
    ];

    SafePreviewRecord {
        record_kind: SAFE_PREVIEW_RECORD_KIND.to_owned(),
        schema_version: SAFE_PREVIEW_SCHEMA_VERSION,
        prototype_label_token: prototype.as_str().to_owned(),
        prototype_label_display: prototype.label().to_owned(),
        preview_id: input.preview_id,
        source_subject_ref: input.source_subject_ref,
        content_class_token: ContentClass::RiskyText.as_str().to_owned(),
        content_class_display: ContentClass::RiskyText.label().to_owned(),
        trust_class_token: input.trust_class.as_str().to_owned(),
        origin_class_token: OriginClass::UserAuthoredOrImported.as_str().to_owned(),
        source_surface_family: input.source_surface_family,
        currently_visible_representation_token: currently_visible.as_str().to_owned(),
        currently_visible_representation_label: currently_visible.label().to_owned(),
        total_byte_count: None,
        visible_byte_count: None,
        visible_line_count: None,
        suspicious_finding_count: finding_count,
        summary_line: summary,
        copy_export_options: options,
        claim_limits: SafePreviewClaimLimit::canonical_limits(),
    }
}

/// Build a [`SafePreviewRecord`] for the oversized-artifact lane.
pub fn build_oversized_artifact_preview(input: OversizedArtifactInput) -> SafePreviewRecord {
    let prototype = PrototypeLabel::M1PrototypeSafePreviewAndCopyExport;
    let currently_visible = CurrentlyVisibleRepresentation::Rendered;
    let windowed = input.visible_byte_count < input.total_byte_count;
    let summary = if windowed {
        format!(
            "Rendered as a windowed slice: visible {} of {} bytes ({} lines). Copy/export options name scope and omission verbatim.",
            input.visible_byte_count, input.total_byte_count, input.visible_line_count,
        )
    } else {
        format!(
            "Full body loaded ({} bytes, {} lines).",
            input.total_byte_count, input.visible_line_count,
        )
    };

    let options = vec![
        copy_visible_window_option_for_oversized(&input, windowed),
        export_sanitized_snapshot_option_for_oversized(&input),
        export_metadata_only_option_for_oversized(&input, windowed),
    ];

    SafePreviewRecord {
        record_kind: SAFE_PREVIEW_RECORD_KIND.to_owned(),
        schema_version: SAFE_PREVIEW_SCHEMA_VERSION,
        prototype_label_token: prototype.as_str().to_owned(),
        prototype_label_display: prototype.label().to_owned(),
        preview_id: input.preview_id,
        source_subject_ref: input.source_subject_ref,
        content_class_token: ContentClass::OversizedArtifact.as_str().to_owned(),
        content_class_display: ContentClass::OversizedArtifact.label().to_owned(),
        trust_class_token: input.trust_class.as_str().to_owned(),
        origin_class_token: OriginClass::UserAuthoredOrImported.as_str().to_owned(),
        source_surface_family: input.source_surface_family,
        currently_visible_representation_token: currently_visible.as_str().to_owned(),
        currently_visible_representation_label: currently_visible.label().to_owned(),
        total_byte_count: Some(input.total_byte_count),
        visible_byte_count: Some(input.visible_byte_count),
        visible_line_count: Some(input.visible_line_count),
        suspicious_finding_count: 0,
        summary_line: summary,
        copy_export_options: options,
        claim_limits: SafePreviewClaimLimit::canonical_limits(),
    }
}

/// Build a [`SafePreviewRecord`] for the generated-content lane.
pub fn build_generated_content_preview(input: GeneratedContentInput) -> SafePreviewRecord {
    let prototype = PrototypeLabel::M1PrototypeSafePreviewAndCopyExport;
    let currently_visible = CurrentlyVisibleRepresentation::Generated;
    let has_citations = !input.citation_anchor_refs.is_empty();
    let has_canonical_source = input.canonical_source_subject_ref.is_some();
    let summary = format!(
        "Generated / derived content from {} — rendered as generated; copy_rendered preserves the generated label.",
        input.generator_id,
    );

    let mut options = Vec::new();
    options.push(copy_rendered_option_for_generated(&input));
    if has_canonical_source && has_citations {
        options.push(copy_raw_option_for_generated(&input));
    }
    options.push(export_sanitized_snapshot_option_for_generated(&input));
    options.push(export_metadata_only_option_for_generated(&input));

    SafePreviewRecord {
        record_kind: SAFE_PREVIEW_RECORD_KIND.to_owned(),
        schema_version: SAFE_PREVIEW_SCHEMA_VERSION,
        prototype_label_token: prototype.as_str().to_owned(),
        prototype_label_display: prototype.label().to_owned(),
        preview_id: input.preview_id,
        source_subject_ref: input.source_subject_ref,
        content_class_token: ContentClass::GeneratedContent.as_str().to_owned(),
        content_class_display: ContentClass::GeneratedContent.label().to_owned(),
        trust_class_token: TrustClass::SanitizedRich.as_str().to_owned(),
        origin_class_token: OriginClass::Generated.as_str().to_owned(),
        source_surface_family: input.source_surface_family,
        currently_visible_representation_token: currently_visible.as_str().to_owned(),
        currently_visible_representation_label: currently_visible.label().to_owned(),
        total_byte_count: None,
        visible_byte_count: None,
        visible_line_count: None,
        suspicious_finding_count: 0,
        summary_line: summary,
        copy_export_options: options,
        claim_limits: SafePreviewClaimLimit::canonical_limits(),
    }
}

// -- internal option builders ------------------------------------------------

fn standard_disclosure_fields() -> Vec<String> {
    vec![
        "representation_label".to_owned(),
        "trust_class_badge".to_owned(),
        "source_surface".to_owned(),
        "scope_label".to_owned(),
        "transform_summary".to_owned(),
        "omission_summary".to_owned(),
    ]
}

fn copy_raw_option_for_risky_text(
    input: &RiskyTextInput,
    has_findings: bool,
) -> CopyExportOption {
    let notes = if has_findings {
        "Raw copy preserves exact source bytes for reconstruction; not share-safe when controls present."
    } else {
        "Raw copy preserves exact source bytes."
    };
    CopyExportOption {
        option_id: format!("option:{}:copy_raw", input.preview_id),
        action_kind: CopyExportActionKind::Copy.as_str().to_owned(),
        action_id: RepresentationActionId::CopyRaw.as_str().to_owned(),
        representation_class: RepresentationClass::Raw.as_str().to_owned(),
        body_posture: BodyPosture::ExactSourceBytes.as_str().to_owned(),
        label: "Copy raw source".to_owned(),
        scope_class: ScopeClass::AnchoredSelection.as_str().to_owned(),
        transforms_applied: vec![TransformKind::None.as_str().to_owned()],
        omission_summary: OmissionSummary::from_reasons(&[OmissionReason::None]),
        required_disclosure_fields: standard_disclosure_fields(),
        must_offer_also: vec![RepresentationActionId::CopyEscaped.as_str().to_owned()],
        citation_anchor_refs: Vec::new(),
        share_safety: ShareSafety {
            safe_for_issue_report: !has_findings,
            safe_for_support_bundle: true,
        },
        notes: notes.to_owned(),
    }
}

fn copy_escaped_option_for_risky_text(
    input: &RiskyTextInput,
    has_findings: bool,
) -> CopyExportOption {
    let mut transforms = vec![
        TransformKind::EscapedControlsOrMetacharacters
            .as_str()
            .to_owned(),
    ];
    if !has_findings {
        transforms = vec![TransformKind::None.as_str().to_owned()];
    }
    CopyExportOption {
        option_id: format!("option:{}:copy_escaped", input.preview_id),
        action_kind: CopyExportActionKind::Copy.as_str().to_owned(),
        action_id: RepresentationActionId::CopyEscaped.as_str().to_owned(),
        representation_class: RepresentationClass::Escaped.as_str().to_owned(),
        body_posture: BodyPosture::EscapedSourceText.as_str().to_owned(),
        label: "Copy escaped (safe inspection)".to_owned(),
        scope_class: ScopeClass::AnchoredSelection.as_str().to_owned(),
        transforms_applied: transforms,
        omission_summary: OmissionSummary::from_reasons(&[OmissionReason::None]),
        required_disclosure_fields: standard_disclosure_fields(),
        must_offer_also: vec![RepresentationActionId::CopyRaw.as_str().to_owned()],
        citation_anchor_refs: Vec::new(),
        share_safety: ShareSafety {
            safe_for_issue_report: true,
            safe_for_support_bundle: true,
        },
        notes:
            "Escaped copy renders bidi / invisible / mixed-script codepoints as stable \\u escapes."
                .to_owned(),
    }
}

fn export_metadata_only_option_for_risky_text(input: &RiskyTextInput) -> CopyExportOption {
    CopyExportOption {
        option_id: format!("option:{}:export_metadata_only", input.preview_id),
        action_kind: CopyExportActionKind::Export.as_str().to_owned(),
        action_id: RepresentationActionId::ExportMetadataOnly
            .as_str()
            .to_owned(),
        representation_class: RepresentationClass::BlockedMetadataOnly.as_str().to_owned(),
        body_posture: BodyPosture::MetadataOnlyEnvelope.as_str().to_owned(),
        label: "Export metadata only (support-safe)".to_owned(),
        scope_class: ScopeClass::MetadataOnly.as_str().to_owned(),
        transforms_applied: vec![TransformKind::RedactedHighRiskSegments.as_str().to_owned()],
        omission_summary: OmissionSummary::from_reasons(&[OmissionReason::Policy]),
        required_disclosure_fields: vec![
            "representation_label".to_owned(),
            "trust_class_badge".to_owned(),
            "metadata_only_reason".to_owned(),
            "omission_summary".to_owned(),
        ],
        must_offer_also: Vec::new(),
        citation_anchor_refs: Vec::new(),
        share_safety: ShareSafety {
            safe_for_issue_report: true,
            safe_for_support_bundle: true,
        },
        notes: "Support export defaults to metadata-only; raw bodies stay withheld by policy."
            .to_owned(),
    }
}

fn copy_visible_window_option_for_oversized(
    input: &OversizedArtifactInput,
    windowed: bool,
) -> CopyExportOption {
    let scope = if windowed {
        ScopeClass::VisibleRowsOrEvents
    } else {
        ScopeClass::LoadedMaterializedSet
    };
    let mut transforms = Vec::new();
    if windowed {
        transforms.push(TransformKind::TruncatedOrWindowed.as_str().to_owned());
        transforms.push(TransformKind::BufferedUnseenExcluded.as_str().to_owned());
    } else {
        transforms.push(TransformKind::None.as_str().to_owned());
    }
    let omission = if windowed {
        OmissionSummary {
            reasons: vec![
                OmissionReason::StreamingWindow.as_str().to_owned(),
                OmissionReason::BufferedUnrevealed.as_str().to_owned(),
            ],
            omitted_bytes_estimate: Some(input.omitted_bytes_estimate),
            omitted_line_count_estimate: Some(input.omitted_line_count_estimate),
            omitted_member_count_estimate: None,
        }
    } else {
        OmissionSummary::from_reasons(&[OmissionReason::None])
    };
    CopyExportOption {
        option_id: format!("option:{}:copy_visible_window", input.preview_id),
        action_kind: CopyExportActionKind::Copy.as_str().to_owned(),
        action_id: RepresentationActionId::CopyRendered.as_str().to_owned(),
        representation_class: RepresentationClass::Rendered.as_str().to_owned(),
        body_posture: BodyPosture::RenderedView.as_str().to_owned(),
        label: if windowed {
            "Copy visible window".to_owned()
        } else {
            "Copy rendered".to_owned()
        },
        scope_class: scope.as_str().to_owned(),
        transforms_applied: transforms,
        omission_summary: omission,
        required_disclosure_fields: standard_disclosure_fields(),
        must_offer_also: vec![RepresentationActionId::ExportMetadataOnly
            .as_str()
            .to_owned()],
        citation_anchor_refs: Vec::new(),
        share_safety: ShareSafety {
            safe_for_issue_report: false,
            safe_for_support_bundle: true,
        },
        notes: "Rendered view is a windowed slice; unseen buffered bytes are excluded verbatim."
            .to_owned(),
    }
}

fn export_sanitized_snapshot_option_for_oversized(
    input: &OversizedArtifactInput,
) -> CopyExportOption {
    CopyExportOption {
        option_id: format!("option:{}:export_sanitized_snapshot", input.preview_id),
        action_kind: CopyExportActionKind::Export.as_str().to_owned(),
        action_id: RepresentationActionId::ExportSanitizedSnapshot
            .as_str()
            .to_owned(),
        representation_class: RepresentationClass::Sanitized.as_str().to_owned(),
        body_posture: BodyPosture::SanitizedStaticSnapshot.as_str().to_owned(),
        label: "Export sanitized snapshot".to_owned(),
        scope_class: ScopeClass::NamedSnapshotOnly.as_str().to_owned(),
        transforms_applied: vec![
            TransformKind::SanitizedActiveContentRemoved.as_str().to_owned(),
            TransformKind::StrippedAnsiOrControlSequences
                .as_str()
                .to_owned(),
        ],
        omission_summary: OmissionSummary::from_reasons(&[OmissionReason::Policy]),
        required_disclosure_fields: standard_disclosure_fields(),
        must_offer_also: Vec::new(),
        citation_anchor_refs: Vec::new(),
        share_safety: ShareSafety {
            safe_for_issue_report: true,
            safe_for_support_bundle: true,
        },
        notes: "Sanitized snapshot strips active content and ANSI / control sequences.".to_owned(),
    }
}

fn export_metadata_only_option_for_oversized(
    input: &OversizedArtifactInput,
    windowed: bool,
) -> CopyExportOption {
    CopyExportOption {
        option_id: format!("option:{}:export_metadata_only", input.preview_id),
        action_kind: CopyExportActionKind::Export.as_str().to_owned(),
        action_id: RepresentationActionId::ExportMetadataOnly
            .as_str()
            .to_owned(),
        representation_class: RepresentationClass::BlockedMetadataOnly.as_str().to_owned(),
        body_posture: BodyPosture::MetadataOnlyEnvelope.as_str().to_owned(),
        label: "Export metadata only".to_owned(),
        scope_class: ScopeClass::MetadataOnly.as_str().to_owned(),
        transforms_applied: vec![TransformKind::RedactedHighRiskSegments.as_str().to_owned()],
        omission_summary: OmissionSummary {
            reasons: vec![OmissionReason::Policy.as_str().to_owned()],
            omitted_bytes_estimate: if windowed {
                Some(input.omitted_bytes_estimate)
            } else {
                None
            },
            omitted_line_count_estimate: if windowed {
                Some(input.omitted_line_count_estimate)
            } else {
                None
            },
            omitted_member_count_estimate: None,
        },
        required_disclosure_fields: vec![
            "representation_label".to_owned(),
            "trust_class_badge".to_owned(),
            "metadata_only_reason".to_owned(),
            "omission_summary".to_owned(),
        ],
        must_offer_also: Vec::new(),
        citation_anchor_refs: Vec::new(),
        share_safety: ShareSafety {
            safe_for_issue_report: true,
            safe_for_support_bundle: true,
        },
        notes: "Metadata-only export preserves identity handle and omission summary; bytes withheld."
            .to_owned(),
    }
}

fn copy_rendered_option_for_generated(input: &GeneratedContentInput) -> CopyExportOption {
    let mut required = standard_disclosure_fields();
    required.push("citation_anchors".to_owned());
    CopyExportOption {
        option_id: format!("option:{}:copy_rendered", input.preview_id),
        action_kind: CopyExportActionKind::Copy.as_str().to_owned(),
        action_id: RepresentationActionId::CopyRendered.as_str().to_owned(),
        representation_class: RepresentationClass::Rendered.as_str().to_owned(),
        body_posture: BodyPosture::RenderedView.as_str().to_owned(),
        label: "Copy generated text (labeled)".to_owned(),
        scope_class: ScopeClass::AnchoredSelection.as_str().to_owned(),
        transforms_applied: vec![TransformKind::SummarizedOrSampled.as_str().to_owned()],
        omission_summary: OmissionSummary::from_reasons(&[OmissionReason::None]),
        required_disclosure_fields: required,
        must_offer_also: vec![RepresentationActionId::ExportSanitizedSnapshot
            .as_str()
            .to_owned()],
        citation_anchor_refs: input.citation_anchor_refs.clone(),
        share_safety: ShareSafety {
            safe_for_issue_report: false,
            safe_for_support_bundle: true,
        },
        notes:
            "Generated copy preserves the generated label; recipients cannot mistake it for raw source."
                .to_owned(),
    }
}

fn copy_raw_option_for_generated(input: &GeneratedContentInput) -> CopyExportOption {
    let canonical = input.canonical_source_subject_ref.clone().unwrap_or_default();
    let mut required = standard_disclosure_fields();
    required.push("citation_anchors".to_owned());
    required.push("origin_or_provenance".to_owned());
    CopyExportOption {
        option_id: format!("option:{}:copy_raw_canonical", input.preview_id),
        action_kind: CopyExportActionKind::Copy.as_str().to_owned(),
        action_id: RepresentationActionId::CopyRaw.as_str().to_owned(),
        representation_class: RepresentationClass::Raw.as_str().to_owned(),
        body_posture: BodyPosture::ExactSourceBytes.as_str().to_owned(),
        label: format!("Copy raw source for {canonical}"),
        scope_class: ScopeClass::AnchoredSelection.as_str().to_owned(),
        transforms_applied: vec![TransformKind::None.as_str().to_owned()],
        omission_summary: OmissionSummary::from_reasons(&[OmissionReason::None]),
        required_disclosure_fields: required,
        must_offer_also: vec![RepresentationActionId::CopyRendered.as_str().to_owned()],
        citation_anchor_refs: input.citation_anchor_refs.clone(),
        share_safety: ShareSafety {
            safe_for_issue_report: false,
            safe_for_support_bundle: true,
        },
        notes:
            "Raw copy is offered only for canonical source bytes backed by citation anchors; never the generated text."
                .to_owned(),
    }
}

fn export_sanitized_snapshot_option_for_generated(
    input: &GeneratedContentInput,
) -> CopyExportOption {
    let mut required = standard_disclosure_fields();
    required.push("citation_anchors".to_owned());
    required.push("origin_or_provenance".to_owned());
    CopyExportOption {
        option_id: format!("option:{}:export_sanitized_snapshot", input.preview_id),
        action_kind: CopyExportActionKind::Export.as_str().to_owned(),
        action_id: RepresentationActionId::ExportSanitizedSnapshot
            .as_str()
            .to_owned(),
        representation_class: RepresentationClass::Sanitized.as_str().to_owned(),
        body_posture: BodyPosture::SanitizedStaticSnapshot.as_str().to_owned(),
        label: "Export sanitized snapshot (with citations)".to_owned(),
        scope_class: ScopeClass::NamedSnapshotOnly.as_str().to_owned(),
        transforms_applied: vec![
            TransformKind::SanitizedActiveContentRemoved.as_str().to_owned(),
            TransformKind::SummarizedOrSampled.as_str().to_owned(),
        ],
        omission_summary: OmissionSummary::from_reasons(&[OmissionReason::None]),
        required_disclosure_fields: required,
        must_offer_also: Vec::new(),
        citation_anchor_refs: input.citation_anchor_refs.clone(),
        share_safety: ShareSafety {
            safe_for_issue_report: true,
            safe_for_support_bundle: true,
        },
        notes:
            "Sanitized snapshot keeps the generated label and embeds the citation anchor list verbatim."
                .to_owned(),
    }
}

fn export_metadata_only_option_for_generated(input: &GeneratedContentInput) -> CopyExportOption {
    let _ = input;
    CopyExportOption {
        option_id: format!("option:{}:export_metadata_only", input.preview_id),
        action_kind: CopyExportActionKind::Export.as_str().to_owned(),
        action_id: RepresentationActionId::ExportMetadataOnly
            .as_str()
            .to_owned(),
        representation_class: RepresentationClass::BlockedMetadataOnly.as_str().to_owned(),
        body_posture: BodyPosture::MetadataOnlyEnvelope.as_str().to_owned(),
        label: "Export metadata only".to_owned(),
        scope_class: ScopeClass::MetadataOnly.as_str().to_owned(),
        transforms_applied: vec![TransformKind::RedactedHighRiskSegments.as_str().to_owned()],
        omission_summary: OmissionSummary::from_reasons(&[OmissionReason::Policy]),
        required_disclosure_fields: vec![
            "representation_label".to_owned(),
            "trust_class_badge".to_owned(),
            "metadata_only_reason".to_owned(),
            "omission_summary".to_owned(),
            "origin_or_provenance".to_owned(),
        ],
        must_offer_also: Vec::new(),
        citation_anchor_refs: Vec::new(),
        share_safety: ShareSafety {
            safe_for_issue_report: true,
            safe_for_support_bundle: true,
        },
        notes: "Metadata-only export records the generator id and identity handle without the bytes."
            .to_owned(),
    }
}

#[cfg(test)]
mod tests;
