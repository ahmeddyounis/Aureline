//! Boundary-record shapes for the safe-preview trust-class vocabulary.
//!
//! These mirror `/schemas/security/trust_class.schema.json` so detector
//! outputs serialize directly into the schema-defined record kinds. Record
//! shapes only carry the fields the detector currently produces; surface
//! consumers may extend the projection with additional `effective_*` fields
//! through their own builders.

use serde::{Deserialize, Serialize};

/// Schema version constant for the safe-preview trust-class vocabulary.
pub const TRUST_CLASS_SCHEMA_VERSION: u32 = 1;

/// Closed safe-preview trust-class vocabulary (architecture spellings verbatim).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TrustClass {
    RawText,
    SanitizedRich,
    TrustedLocalActive,
    IsolatedRemoteActive,
}

impl TrustClass {
    /// Stable string used in records and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawText => "RawText",
            Self::SanitizedRich => "SanitizedRich",
            Self::TrustedLocalActive => "TrustedLocalActive",
            Self::IsolatedRemoteActive => "IsolatedRemoteActive",
        }
    }
}

/// Closed surface-family vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceFamily {
    EditorContent,
    DocsHelpPage,
    ReviewSurface,
    RichPreview,
    InstallReview,
    PublishReview,
    RemoteAttachReview,
    ApprovalSurface,
    DeleteReviewSurface,
    SupportExportSurface,
}

impl SurfaceFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorContent => "editor_content",
            Self::DocsHelpPage => "docs_help_page",
            Self::ReviewSurface => "review_surface",
            Self::RichPreview => "rich_preview",
            Self::InstallReview => "install_review",
            Self::PublishReview => "publish_review",
            Self::RemoteAttachReview => "remote_attach_review",
            Self::ApprovalSurface => "approval_surface",
            Self::DeleteReviewSurface => "delete_review_surface",
            Self::SupportExportSurface => "support_export_surface",
        }
    }
}

/// Closed suspicious-content class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuspiciousContentClass {
    BidiControl,
    InvisibleFormatting,
    MixedScriptConfusable,
    WholeScriptConfusable,
    RawRenderedDivergence,
}

impl SuspiciousContentClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BidiControl => "bidi_control",
            Self::InvisibleFormatting => "invisible_formatting",
            Self::MixedScriptConfusable => "mixed_script_confusable",
            Self::WholeScriptConfusable => "whole_script_confusable",
            Self::RawRenderedDivergence => "raw_rendered_divergence",
        }
    }
}

/// One suspicious-content finding (matches `suspicious_content_finding_record`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspiciousContentFindingRecord {
    pub finding_id: String,
    pub content_class: String,
    pub location_kind: String,
    pub visibility_impact: String,
    pub reveal_affordances: Vec<String>,
    pub suppression_scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Suspicious-content case record (matches `suspicious_content_case_record`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuspiciousContentCaseRecord {
    pub record_kind: String,
    pub trust_class_schema_version: u32,
    pub case_id: String,
    pub surface_family: String,
    pub annotation_mode: String,
    pub findings: Vec<SuspiciousContentFindingRecord>,
    pub stricter_annotation_requirements: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Representative label set carried by a surface trust resolution record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabelExamples {
    pub badge_label: String,
    pub primary_copy_label: String,
    pub primary_export_label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning_label: Option<String>,
}

/// Surface trust resolution record (matches `surface_trust_resolution_record`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceTrustResolutionRecord {
    pub record_kind: String,
    pub trust_class_schema_version: u32,
    pub case_id: String,
    pub surface_family: String,
    pub content_subject_ref: String,
    pub owner_identity_ref: String,
    pub origin_identity_ref: String,
    pub origin_kind: String,
    pub current_trust_class: String,
    pub connectivity_state: String,
    pub effective_allowed_behaviors: Vec<String>,
    pub default_transfer_actions: Vec<String>,
    pub upgrade_trigger_requirements: Vec<String>,
    pub downgrade_trigger_observations: Vec<String>,
    pub effective_downgrade_posture: String,
    pub required_owner_origin_chrome: Vec<String>,
    pub label_examples: LabelExamples,
    pub related_finding_refs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
