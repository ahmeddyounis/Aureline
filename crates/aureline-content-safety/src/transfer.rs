//! Representation-labeled transfer record shapes.
//!
//! Mirrors `/schemas/security/text_representation_policy.schema.json`. Every
//! detector outcome resolves to one or more transfer records so a copy or
//! export action carries an explicit representation label instead of a
//! generic "Copy" / "Export" stem.

use serde::{Deserialize, Serialize};

/// Schema version constant for the text representation policy.
pub const TEXT_REPRESENTATION_POLICY_SCHEMA_VERSION: u32 = 1;

/// Closed transfer-action vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationActionId {
    CopyRaw,
    CopyRendered,
    CopyEscaped,
    ExportSanitizedSnapshot,
    ExportMetadataOnly,
}

impl RepresentationActionId {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CopyRaw => "copy_raw",
            Self::CopyRendered => "copy_rendered",
            Self::CopyEscaped => "copy_escaped",
            Self::ExportSanitizedSnapshot => "export_sanitized_snapshot",
            Self::ExportMetadataOnly => "export_metadata_only",
        }
    }
}

/// Transfer-safe representation class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationClass {
    Raw,
    Rendered,
    Escaped,
    Sanitized,
    BlockedMetadataOnly,
}

impl RepresentationClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Raw => "raw",
            Self::Rendered => "rendered",
            Self::Escaped => "escaped",
            Self::Sanitized => "sanitized",
            Self::BlockedMetadataOnly => "blocked_metadata_only",
        }
    }
}

/// Closed body-posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BodyPosture {
    ExactSourceBytes,
    RenderedView,
    EscapedSourceText,
    SanitizedStaticSnapshot,
    MetadataOnlyEnvelope,
}

impl BodyPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactSourceBytes => "exact_source_bytes",
            Self::RenderedView => "rendered_view",
            Self::EscapedSourceText => "escaped_source_text",
            Self::SanitizedStaticSnapshot => "sanitized_static_snapshot",
            Self::MetadataOnlyEnvelope => "metadata_only_envelope",
        }
    }
}

/// Representation transfer record (`representation_transfer_record`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationTransferRecord {
    pub record_kind: String,
    pub text_representation_policy_schema_version: u32,
    pub case_id: String,
    pub source_surface_family: String,
    pub source_trust_class: String,
    pub action_id: String,
    pub representation_class: String,
    pub label: String,
    pub body_posture: String,
    pub raw_source_required: bool,
    pub active_content_removed: bool,
    pub must_offer_also: Vec<String>,
    pub required_disclosure_fields: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
