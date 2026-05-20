//! Public/private handoff review and reproduction-packet preview.
//!
//! This module is the shell-side projection that the issue, security-
//! disclosure, docs-feedback, RFC/discussion, and community-support handoff
//! lanes read before a report leaves the product. It answers two questions the
//! user must be able to settle *before* a browser opens:
//!
//! 1. **Where is this going, and how visible will it be?** Every handoff target
//!    pins a typed [`TargetVisibilityClass`] (`Official public`,
//!    `Official private`, `Security disclosure`, `Community`,
//!    `Third-party / vendor`), a typed [`HandoffRouteClass`], the
//!    network/browser requirement, the data-exit boundary (reused from the
//!    About/help/community vocabulary), and at least one safe fallback route so
//!    a blocked handoff degrades to a labeled path instead of failing silently.
//!
//! 2. **What exactly will be shared?** A [`ReproPacketPreview`] summarizes the
//!    selected diagnostics, the redaction posture, the attachment list, and the
//!    exact anchor/object identity the report is about, and gates the share
//!    behind an explicit preview-before-share confirmation.
//!
//! The two records are bundled by [`HandoffReviewSheet`], which also carries a
//! [`DraftContinuity`] block: when the browser handoff is blocked, offline, or
//! policy-denied, the drafted text, selected attachments, target class, and
//! redaction selections are preserved with export / save / discard actions
//! rather than dropped. The sheet cross-validator proves the three pieces agree
//! — the redaction posture is safe for the target's visibility, the preserved
//! draft mirrors the chosen target class and redaction posture, the share is
//! gated on preview confirmation, and a blocked handoff never silently loses
//! work.
//!
//! Build-context export blocks and the data-exit vocabulary are reused verbatim
//! from [`crate::public_truth`] so the issue/report/disclosure lanes carry the
//! same versioned, redaction-safe export the About and community surfaces
//! already publish — the user never has to infer scope from a raw URL.
//!
//! Raw URLs, raw email addresses, and raw secret material MUST NOT appear; the
//! records carry opaque refs and bounded reviewable summaries only. The schema
//! boundaries are `schemas/public/handoff_target_review.schema.json` and
//! `schemas/public/repro_packet_preview.schema.json`; the contract narrative is
//! `docs/public/m3/handoff_and_repro_boundary.md`.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::public_truth::{BuildContextExport, BuildContextExportClass, DataExitBoundary};

/// Stable record-kind tag carried by [`HandoffTargetReview`].
pub const HANDOFF_TARGET_REVIEW_RECORD_KIND: &str = "handoff_target_review_record";

/// Stable record-kind tag carried by [`ReproPacketPreview`].
pub const REPRO_PACKET_PREVIEW_RECORD_KIND: &str = "repro_packet_preview_record";

/// Stable record-kind tag carried by [`HandoffReviewSheet`].
pub const HANDOFF_REVIEW_SHEET_RECORD_KIND: &str = "handoff_review_sheet_record";

/// Schema version for the [`HandoffTargetReview`] payload shape.
pub const HANDOFF_TARGET_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Schema version for the [`ReproPacketPreview`] payload shape.
pub const REPRO_PACKET_PREVIEW_SCHEMA_VERSION: u32 = 1;

/// Schema version for the [`HandoffReviewSheet`] payload shape.
pub const HANDOFF_REVIEW_SHEET_SCHEMA_VERSION: u32 = 1;

/// Frozen reference to the contract doc all three records point at.
pub const HANDOFF_AND_REPRO_CONTRACT_DOC_REF: &str =
    "docs/public/m3/handoff_and_repro_boundary.md";

/// Closed five-class target-visibility vocabulary stated on every pre-handoff
/// review sheet. Distinct from the four-class About destination vocabulary
/// because a security disclosure is its own visibility boundary, not a public
/// or private route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetVisibilityClass {
    OfficialPublic,
    OfficialPrivate,
    SecurityDisclosure,
    Community,
    ThirdPartyVendor,
}

impl TargetVisibilityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficialPublic => "official_public",
            Self::OfficialPrivate => "official_private",
            Self::SecurityDisclosure => "security_disclosure",
            Self::Community => "community",
            Self::ThirdPartyVendor => "third_party_vendor",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::OfficialPublic => "Official public",
            Self::OfficialPrivate => "Official private",
            Self::SecurityDisclosure => "Security disclosure",
            Self::Community => "Community",
            Self::ThirdPartyVendor => "Third-party / vendor",
        }
    }

    /// True when sharing to this target makes the report world-readable.
    pub const fn is_public(self) -> bool {
        matches!(self, Self::OfficialPublic | Self::Community)
    }
}

/// Closed handoff-route vocabulary covering the five lanes in scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffRouteClass {
    PublicIssue,
    SecurityDisclosure,
    DocsFeedback,
    RfcDiscussion,
    CommunitySupport,
}

impl HandoffRouteClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicIssue => "public_issue",
            Self::SecurityDisclosure => "security_disclosure",
            Self::DocsFeedback => "docs_feedback",
            Self::RfcDiscussion => "rfc_discussion",
            Self::CommunitySupport => "community_support",
        }
    }

    /// The closed set of visibilities a route may legitimately target. A
    /// visibility outside this set is denied so the lanes never blur together
    /// or coerce the user into a public target by accident.
    pub fn allows_visibility(self, visibility: TargetVisibilityClass) -> bool {
        use TargetVisibilityClass as V;
        match self {
            Self::PublicIssue => matches!(visibility, V::OfficialPublic | V::Community),
            Self::SecurityDisclosure => {
                matches!(visibility, V::SecurityDisclosure | V::OfficialPrivate)
            }
            Self::DocsFeedback => matches!(visibility, V::OfficialPublic | V::Community),
            Self::RfcDiscussion => matches!(visibility, V::Community | V::OfficialPublic),
            Self::CommunitySupport => matches!(visibility, V::Community | V::OfficialPrivate),
        }
    }
}

/// Closed network/browser-requirement vocabulary disclosed before navigation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NetworkBrowserRequirement {
    OfflineCapturePreview,
    SystemBrowserPublicBrowse,
    SystemBrowserAuthenticatedPlane,
    EncryptedSecurityChannel,
    VendorOrThirdPartyCall,
}

impl NetworkBrowserRequirement {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfflineCapturePreview => "offline_capture_preview",
            Self::SystemBrowserPublicBrowse => "system_browser_public_browse",
            Self::SystemBrowserAuthenticatedPlane => "system_browser_authenticated_plane",
            Self::EncryptedSecurityChannel => "encrypted_security_channel",
            Self::VendorOrThirdPartyCall => "vendor_or_third_party_call",
        }
    }

    /// True when the requirement opens an external system browser.
    pub const fn needs_browser(self) -> bool {
        matches!(
            self,
            Self::SystemBrowserPublicBrowse | Self::SystemBrowserAuthenticatedPlane
        )
    }
}

/// Closed redaction-posture vocabulary for a reproduction packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionPostureClass {
    FullyRedactedPublicSafe,
    RedactedSupportScoped,
    SecurityChannelOnly,
    MetadataRefsOnly,
}

impl RedactionPostureClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyRedactedPublicSafe => "fully_redacted_public_safe",
            Self::RedactedSupportScoped => "redacted_support_scoped",
            Self::SecurityChannelOnly => "security_channel_only",
            Self::MetadataRefsOnly => "metadata_refs_only",
        }
    }

    /// True when the posture is safe to display on a world-readable target.
    pub const fn is_public_safe(self) -> bool {
        matches!(self, Self::FullyRedactedPublicSafe | Self::MetadataRefsOnly)
    }

    /// Whether this posture is compatible with the given target visibility.
    pub fn allowed_for_visibility(self, visibility: TargetVisibilityClass) -> bool {
        use TargetVisibilityClass as V;
        match visibility {
            V::OfficialPublic | V::Community | V::ThirdPartyVendor => self.is_public_safe(),
            V::OfficialPrivate => matches!(
                self,
                Self::RedactedSupportScoped
                    | Self::MetadataRefsOnly
                    | Self::FullyRedactedPublicSafe
            ),
            V::SecurityDisclosure => matches!(self, Self::SecurityChannelOnly),
        }
    }
}

/// Closed diagnostic-kind vocabulary for the selected diagnostics summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticKindClass {
    BuildIdentity,
    EnvironmentCapsule,
    RedactedLogTail,
    SanitizedConfigSnapshot,
    ReproStepsText,
    AnchorObjectRef,
    PerformanceTrace,
}

impl DiagnosticKindClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildIdentity => "build_identity",
            Self::EnvironmentCapsule => "environment_capsule",
            Self::RedactedLogTail => "redacted_log_tail",
            Self::SanitizedConfigSnapshot => "sanitized_config_snapshot",
            Self::ReproStepsText => "repro_steps_text",
            Self::AnchorObjectRef => "anchor_object_ref",
            Self::PerformanceTrace => "performance_trace",
        }
    }
}

/// Closed attachment-kind vocabulary for the previewed attachment list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentKindClass {
    BuildContextExportBlock,
    RedactedLogBundle,
    MinimalReproProject,
    SanitizedConfigBundle,
    AnchorObjectSnapshot,
}

impl AttachmentKindClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildContextExportBlock => "build_context_export_block",
            Self::RedactedLogBundle => "redacted_log_bundle",
            Self::MinimalReproProject => "minimal_repro_project",
            Self::SanitizedConfigBundle => "sanitized_config_bundle",
            Self::AnchorObjectSnapshot => "anchor_object_snapshot",
        }
    }
}

/// Closed handoff-outcome vocabulary tracked by the draft-continuity block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffOutcomeClass {
    PreviewPendingConfirmation,
    OpenedInSystemBrowser,
    BrowserBlocked,
    Offline,
    PolicyDenied,
    TargetPermissionDenied,
}

impl HandoffOutcomeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewPendingConfirmation => "preview_pending_confirmation",
            Self::OpenedInSystemBrowser => "opened_in_system_browser",
            Self::BrowserBlocked => "browser_blocked",
            Self::Offline => "offline",
            Self::PolicyDenied => "policy_denied",
            Self::TargetPermissionDenied => "target_permission_denied",
        }
    }

    /// True when the handoff did not leave the product and the draft must be
    /// preserved with export / save actions instead of lost.
    pub const fn is_blocked(self) -> bool {
        matches!(
            self,
            Self::BrowserBlocked
                | Self::Offline
                | Self::PolicyDenied
                | Self::TargetPermissionDenied
        )
    }
}

/// Closed preservation-action vocabulary offered when a handoff is blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservationActionClass {
    ExportPacket,
    SaveDraftLocal,
    RetryWhenOnline,
    CopyRefsToClipboard,
    Discard,
}

impl PreservationActionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportPacket => "export_packet",
            Self::SaveDraftLocal => "save_draft_local",
            Self::RetryWhenOnline => "retry_when_online",
            Self::CopyRefsToClipboard => "copy_refs_to_clipboard",
            Self::Discard => "discard",
        }
    }
}

/// One pre-handoff review row stating where a report is going and how visible
/// it will be once it leaves the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffTargetReview {
    pub handoff_target_review_schema_version: u32,
    pub record_kind: String,
    pub target_id: String,
    pub route_class: HandoffRouteClass,
    pub visibility_class: TargetVisibilityClass,
    pub destination_identity_ref: String,
    pub destination_label: String,
    pub network_browser_requirement_class: NetworkBrowserRequirement,
    pub data_exit_boundary_class: DataExitBoundary,
    pub safe_fallback_refs: Vec<String>,
    #[serde(default)]
    pub build_context_exports: Vec<BuildContextExport>,
    pub headline_label: String,
    pub target_summary: String,
    pub contract_doc_ref: String,
    pub notes: Option<String>,
}

impl HandoffTargetReview {
    /// Validate the row against the handoff-boundary contract.
    pub fn validate(&self) -> Result<(), HandoffReviewValidationError> {
        if self.handoff_target_review_schema_version != HANDOFF_TARGET_REVIEW_SCHEMA_VERSION {
            return Err(HandoffReviewValidationError::WrongTargetSchemaVersion {
                target_id: self.target_id.clone(),
                actual: self.handoff_target_review_schema_version,
            });
        }
        if self.record_kind != HANDOFF_TARGET_REVIEW_RECORD_KIND {
            return Err(HandoffReviewValidationError::WrongTargetRecordKind {
                target_id: self.target_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if !self.target_id.starts_with("handoff_target:") {
            return Err(HandoffReviewValidationError::MalformedTargetId {
                target_id: self.target_id.clone(),
            });
        }
        if self.contract_doc_ref != HANDOFF_AND_REPRO_CONTRACT_DOC_REF {
            return Err(HandoffReviewValidationError::WrongContractDocRef {
                record_id: self.target_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }
        for (field, value) in [
            ("headline_label", &self.headline_label),
            ("target_summary", &self.target_summary),
            ("destination_label", &self.destination_label),
        ] {
            if non_empty(value).is_none() {
                return Err(HandoffReviewValidationError::EmptyRequiredField {
                    record_id: self.target_id.clone(),
                    field,
                });
            }
        }
        if !ref_is_opaque(&self.destination_identity_ref) {
            return Err(HandoffReviewValidationError::RawRefLeak {
                record_id: self.target_id.clone(),
                field: "destination_identity_ref",
            });
        }

        // Every handoff target offers at least one safe fallback so a blocked
        // route degrades to a labeled path instead of dead-ending.
        if self.safe_fallback_refs.is_empty() {
            return Err(HandoffReviewValidationError::MissingSafeFallback {
                target_id: self.target_id.clone(),
            });
        }
        for fallback in &self.safe_fallback_refs {
            if !ref_is_opaque(fallback) {
                return Err(HandoffReviewValidationError::RawRefLeak {
                    record_id: self.target_id.clone(),
                    field: "safe_fallback_refs",
                });
            }
        }

        // The route may only target a visibility from its allowed set — no
        // accidental coercion into a public target.
        if !self.route_class.allows_visibility(self.visibility_class) {
            return Err(HandoffReviewValidationError::RouteVisibilityMismatch {
                target_id: self.target_id.clone(),
                route: self.route_class,
                visibility: self.visibility_class,
            });
        }

        // Visibility pins the data-exit boundary and the network requirement.
        if !visibility_allows_data_exit(self.visibility_class, self.data_exit_boundary_class) {
            return Err(HandoffReviewValidationError::VisibilityDataExitMismatch {
                target_id: self.target_id.clone(),
                visibility: self.visibility_class,
                data_exit: self.data_exit_boundary_class,
            });
        }
        if !visibility_allows_network(self.visibility_class, self.network_browser_requirement_class)
        {
            return Err(HandoffReviewValidationError::VisibilityNetworkMismatch {
                target_id: self.target_id.clone(),
                visibility: self.visibility_class,
                network: self.network_browser_requirement_class,
            });
        }

        // Every handoff lane attaches a versioned, redaction-safe build-context
        // export reused from the About/help/community surfaces.
        if self.build_context_exports.is_empty() {
            return Err(HandoffReviewValidationError::MissingBuildContextExport {
                target_id: self.target_id.clone(),
            });
        }
        for export in &self.build_context_exports {
            if export.export_block_schema_version < 1 {
                return Err(HandoffReviewValidationError::BuildContextExportSchemaVersionInvalid {
                    target_id: self.target_id.clone(),
                    actual: export.export_block_schema_version,
                });
            }
            if !export.raw_screenshots_excluded || !export.raw_secrets_excluded {
                return Err(HandoffReviewValidationError::BuildContextExportNotRedactionSafe {
                    target_id: self.target_id.clone(),
                });
            }
            if non_empty(&export.export_block_ref).is_none()
                || non_empty(&export.export_summary).is_none()
            {
                return Err(HandoffReviewValidationError::BuildContextExportFieldEmpty {
                    target_id: self.target_id.clone(),
                });
            }
        }

        Ok(())
    }
}

/// The exact anchor / object identity a reproduction packet is about, so the
/// report names a precise object rather than a fuzzy description.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExactAnchorIdentity {
    pub anchor_ref: String,
    pub object_ref: String,
    pub anchor_label: String,
}

/// One selected diagnostic summarized in the reproduction-packet preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSelection {
    pub kind_class: DiagnosticKindClass,
    pub included: bool,
    pub summary: String,
}

/// One attachment listed in the reproduction-packet preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReproAttachment {
    pub kind_class: AttachmentKindClass,
    pub attachment_ref: String,
    #[serde(default = "always_true")]
    pub redaction_applied: bool,
    pub summary: String,
}

fn always_true() -> bool {
    true
}

/// The reproduction-packet preview the user reviews before share: which
/// diagnostics are selected, how they are redacted, what is attached, the exact
/// anchor identity, and whether the preview has been confirmed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReproPacketPreview {
    pub repro_packet_preview_schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub redaction_posture_class: RedactionPostureClass,
    pub selected_diagnostics: Vec<DiagnosticSelection>,
    pub attachments: Vec<ReproAttachment>,
    pub anchor_identity: ExactAnchorIdentity,
    pub preview_confirmed_before_share: bool,
    #[serde(default = "always_true")]
    pub raw_secrets_excluded: bool,
    #[serde(default = "always_true")]
    pub raw_screenshots_excluded: bool,
    pub headline_label: String,
    pub packet_summary: String,
    pub contract_doc_ref: String,
    pub notes: Option<String>,
}

impl ReproPacketPreview {
    /// Validate the preview against the reproduction-packet contract.
    pub fn validate(&self) -> Result<(), HandoffReviewValidationError> {
        if self.repro_packet_preview_schema_version != REPRO_PACKET_PREVIEW_SCHEMA_VERSION {
            return Err(HandoffReviewValidationError::WrongPacketSchemaVersion {
                packet_id: self.packet_id.clone(),
                actual: self.repro_packet_preview_schema_version,
            });
        }
        if self.record_kind != REPRO_PACKET_PREVIEW_RECORD_KIND {
            return Err(HandoffReviewValidationError::WrongPacketRecordKind {
                packet_id: self.packet_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if !self.packet_id.starts_with("repro_packet_preview:") {
            return Err(HandoffReviewValidationError::MalformedPacketId {
                packet_id: self.packet_id.clone(),
            });
        }
        if self.contract_doc_ref != HANDOFF_AND_REPRO_CONTRACT_DOC_REF {
            return Err(HandoffReviewValidationError::WrongContractDocRef {
                record_id: self.packet_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }
        for (field, value) in [
            ("headline_label", &self.headline_label),
            ("packet_summary", &self.packet_summary),
        ] {
            if non_empty(value).is_none() {
                return Err(HandoffReviewValidationError::EmptyRequiredField {
                    record_id: self.packet_id.clone(),
                    field,
                });
            }
        }

        // Raw payloads never ride a preview that can be shared.
        if !self.raw_secrets_excluded || !self.raw_screenshots_excluded {
            return Err(HandoffReviewValidationError::RawPayloadNotExcluded {
                packet_id: self.packet_id.clone(),
            });
        }

        if self.selected_diagnostics.is_empty() {
            return Err(HandoffReviewValidationError::NoDiagnosticsSelected {
                packet_id: self.packet_id.clone(),
            });
        }
        if !self.selected_diagnostics.iter().any(|d| d.included) {
            return Err(HandoffReviewValidationError::NoDiagnosticsSelected {
                packet_id: self.packet_id.clone(),
            });
        }
        for diagnostic in &self.selected_diagnostics {
            if non_empty(&diagnostic.summary).is_none() {
                return Err(HandoffReviewValidationError::EmptyRequiredField {
                    record_id: self.packet_id.clone(),
                    field: "selected_diagnostics.summary",
                });
            }
        }

        for attachment in &self.attachments {
            if !ref_is_opaque(&attachment.attachment_ref) {
                return Err(HandoffReviewValidationError::RawRefLeak {
                    record_id: self.packet_id.clone(),
                    field: "attachments.attachment_ref",
                });
            }
            if non_empty(&attachment.summary).is_none() {
                return Err(HandoffReviewValidationError::EmptyRequiredField {
                    record_id: self.packet_id.clone(),
                    field: "attachments.summary",
                });
            }
            if !attachment.redaction_applied {
                return Err(HandoffReviewValidationError::AttachmentNotRedacted {
                    packet_id: self.packet_id.clone(),
                    attachment: attachment.kind_class,
                });
            }
        }

        if !ref_is_opaque(&self.anchor_identity.anchor_ref)
            || !ref_is_opaque(&self.anchor_identity.object_ref)
        {
            return Err(HandoffReviewValidationError::RawRefLeak {
                record_id: self.packet_id.clone(),
                field: "anchor_identity",
            });
        }
        if non_empty(&self.anchor_identity.anchor_label).is_none() {
            return Err(HandoffReviewValidationError::EmptyRequiredField {
                record_id: self.packet_id.clone(),
                field: "anchor_identity.anchor_label",
            });
        }

        Ok(())
    }
}

/// The draft-continuity block: what survives when a handoff is blocked,
/// offline, or policy-denied. Drafted text, selected attachments, target class,
/// and redaction selections are preserved with export / save / discard actions
/// rather than silently lost.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DraftContinuity {
    pub handoff_outcome_class: HandoffOutcomeClass,
    pub intent_preserved: bool,
    #[serde(default)]
    pub silent_loss: bool,
    pub preserved_draft_text_ref: Option<String>,
    #[serde(default)]
    pub preserved_attachment_refs: Vec<String>,
    pub preserved_visibility_class: TargetVisibilityClass,
    pub preserved_redaction_posture_class: RedactionPostureClass,
    pub available_actions: Vec<PreservationActionClass>,
    pub selected_fallback_ref: Option<String>,
    pub continuity_summary: String,
}

impl DraftContinuity {
    fn validate(&self, sheet_id: &str) -> Result<(), HandoffReviewValidationError> {
        if non_empty(&self.continuity_summary).is_none() {
            return Err(HandoffReviewValidationError::EmptyRequiredField {
                record_id: sheet_id.to_owned(),
                field: "draft_continuity.continuity_summary",
            });
        }
        // Silent loss is never allowed, regardless of outcome.
        if self.silent_loss {
            return Err(HandoffReviewValidationError::SilentLossNotAllowed {
                sheet_id: sheet_id.to_owned(),
            });
        }
        if self.available_actions.is_empty() {
            return Err(HandoffReviewValidationError::NoPreservationActions {
                sheet_id: sheet_id.to_owned(),
            });
        }
        for fallback in self.preserved_attachment_refs.iter() {
            if !ref_is_opaque(fallback) {
                return Err(HandoffReviewValidationError::RawRefLeak {
                    record_id: sheet_id.to_owned(),
                    field: "draft_continuity.preserved_attachment_refs",
                });
            }
        }
        if let Some(text_ref) = self.preserved_draft_text_ref.as_deref() {
            if !ref_is_opaque(text_ref) {
                return Err(HandoffReviewValidationError::RawRefLeak {
                    record_id: sheet_id.to_owned(),
                    field: "draft_continuity.preserved_draft_text_ref",
                });
            }
        }
        if let Some(fallback) = self.selected_fallback_ref.as_deref() {
            if !ref_is_opaque(fallback) {
                return Err(HandoffReviewValidationError::RawRefLeak {
                    record_id: sheet_id.to_owned(),
                    field: "draft_continuity.selected_fallback_ref",
                });
            }
        }

        // A blocked handoff must preserve intent, keep the draft text, and offer
        // both export and local save — never just discard.
        if self.handoff_outcome_class.is_blocked() {
            if !self.intent_preserved {
                return Err(HandoffReviewValidationError::BlockedHandoffDroppedIntent {
                    sheet_id: sheet_id.to_owned(),
                    outcome: self.handoff_outcome_class,
                });
            }
            if self.preserved_draft_text_ref.is_none() {
                return Err(HandoffReviewValidationError::BlockedHandoffMissingDraftText {
                    sheet_id: sheet_id.to_owned(),
                    outcome: self.handoff_outcome_class,
                });
            }
            let has_export = self
                .available_actions
                .contains(&PreservationActionClass::ExportPacket);
            let has_save = self
                .available_actions
                .contains(&PreservationActionClass::SaveDraftLocal);
            if !has_export || !has_save {
                return Err(
                    HandoffReviewValidationError::BlockedHandoffMissingPreservationActions {
                        sheet_id: sheet_id.to_owned(),
                        outcome: self.handoff_outcome_class,
                    },
                );
            }
        }

        Ok(())
    }
}

/// One pre-handoff review sheet bundling the target review, the reproduction-
/// packet preview, and the draft-continuity block rendered together before a
/// report leaves the shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffReviewSheet {
    pub handoff_review_sheet_schema_version: u32,
    pub record_kind: String,
    pub sheet_id: String,
    pub sheet_summary: String,
    pub target_review: HandoffTargetReview,
    pub repro_packet_preview: ReproPacketPreview,
    pub draft_continuity: DraftContinuity,
    pub contract_doc_ref: String,
    #[serde(default)]
    pub notes: Option<String>,
}

impl HandoffReviewSheet {
    /// Cross-validate the sheet: each constituent record validates, the
    /// redaction posture is safe for the target visibility, the preserved draft
    /// mirrors the chosen target class and redaction posture, the share is
    /// gated on preview confirmation, and a selected fallback is one the target
    /// actually offered.
    pub fn validate(&self) -> Result<(), HandoffReviewValidationError> {
        if self.handoff_review_sheet_schema_version != HANDOFF_REVIEW_SHEET_SCHEMA_VERSION {
            return Err(HandoffReviewValidationError::WrongSheetSchemaVersion {
                sheet_id: self.sheet_id.clone(),
                actual: self.handoff_review_sheet_schema_version,
            });
        }
        if self.record_kind != HANDOFF_REVIEW_SHEET_RECORD_KIND {
            return Err(HandoffReviewValidationError::WrongSheetRecordKind {
                sheet_id: self.sheet_id.clone(),
                actual: self.record_kind.clone(),
            });
        }
        if !self.sheet_id.starts_with("handoff_review_sheet:") {
            return Err(HandoffReviewValidationError::MalformedSheetId {
                sheet_id: self.sheet_id.clone(),
            });
        }
        if self.contract_doc_ref != HANDOFF_AND_REPRO_CONTRACT_DOC_REF {
            return Err(HandoffReviewValidationError::WrongContractDocRef {
                record_id: self.sheet_id.clone(),
                actual: self.contract_doc_ref.clone(),
            });
        }
        if non_empty(&self.sheet_summary).is_none() {
            return Err(HandoffReviewValidationError::EmptyRequiredField {
                record_id: self.sheet_id.clone(),
                field: "sheet_summary",
            });
        }

        self.target_review.validate()?;
        self.repro_packet_preview.validate()?;
        self.draft_continuity.validate(&self.sheet_id)?;

        // The redaction posture must be safe for the chosen visibility — a
        // public target may not carry a support- or security-scoped payload.
        if !self
            .repro_packet_preview
            .redaction_posture_class
            .allowed_for_visibility(self.target_review.visibility_class)
        {
            return Err(HandoffReviewValidationError::RedactionPostureUnsafeForVisibility {
                sheet_id: self.sheet_id.clone(),
                posture: self.repro_packet_preview.redaction_posture_class,
                visibility: self.target_review.visibility_class,
            });
        }

        // The preserved draft mirrors the chosen target class and redaction
        // posture, proving a blocked handoff keeps the user's actual selections.
        if self.draft_continuity.preserved_visibility_class != self.target_review.visibility_class {
            return Err(HandoffReviewValidationError::PreservedVisibilityMismatch {
                sheet_id: self.sheet_id.clone(),
                preserved: self.draft_continuity.preserved_visibility_class,
                target: self.target_review.visibility_class,
            });
        }
        if self.draft_continuity.preserved_redaction_posture_class
            != self.repro_packet_preview.redaction_posture_class
        {
            return Err(HandoffReviewValidationError::PreservedRedactionMismatch {
                sheet_id: self.sheet_id.clone(),
                preserved: self.draft_continuity.preserved_redaction_posture_class,
                packet: self.repro_packet_preview.redaction_posture_class,
            });
        }

        // A handoff only opens the browser after the preview is confirmed.
        if matches!(
            self.draft_continuity.handoff_outcome_class,
            HandoffOutcomeClass::OpenedInSystemBrowser
        ) && !self.repro_packet_preview.preview_confirmed_before_share
        {
            return Err(HandoffReviewValidationError::SharedWithoutPreviewConfirmation {
                sheet_id: self.sheet_id.clone(),
            });
        }

        // A selected fallback must be one the target review actually offered.
        if let Some(selected) = self.draft_continuity.selected_fallback_ref.as_deref() {
            if !self
                .target_review
                .safe_fallback_refs
                .iter()
                .any(|f| f == selected)
            {
                return Err(HandoffReviewValidationError::SelectedFallbackNotOffered {
                    sheet_id: self.sheet_id.clone(),
                    fallback_ref: selected.to_owned(),
                });
            }
        }

        Ok(())
    }

    /// Render a deterministic plaintext block for support exports and
    /// reviewer-facing previews. Stable for the same input snapshot.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str("Pre-handoff review and reproduction-packet preview\n");
        out.push_str(&format!("Sheet: {}\n", self.sheet_id));
        out.push_str(&format!("Summary: {}\n\n", self.sheet_summary));

        let t = &self.target_review;
        out.push_str("Handoff target:\n");
        out.push_str(&format!(
            "- [{}] {} — visibility={} ({}) route={} network={} data_exit={}\n",
            t.target_id,
            t.headline_label,
            t.visibility_class.as_str(),
            t.visibility_class.label(),
            t.route_class.as_str(),
            t.network_browser_requirement_class.as_str(),
            t.data_exit_boundary_class.as_str(),
        ));
        out.push_str(&format!("    destination: {}\n", t.destination_identity_ref));
        for fallback in &t.safe_fallback_refs {
            out.push_str(&format!("    safe fallback: {fallback}\n"));
        }
        for export in &t.build_context_exports {
            out.push_str(&format!(
                "    build-context export: {} (v{}, ref={})\n",
                export.export_class.as_str(),
                export.export_block_schema_version,
                export.export_block_ref,
            ));
        }
        out.push('\n');

        let p = &self.repro_packet_preview;
        out.push_str("Reproduction packet preview:\n");
        out.push_str(&format!(
            "- [{}] {} — redaction={} preview_confirmed={}\n",
            p.packet_id,
            p.headline_label,
            p.redaction_posture_class.as_str(),
            p.preview_confirmed_before_share,
        ));
        out.push_str(&format!(
            "    anchor: {} (object={})\n",
            p.anchor_identity.anchor_ref, p.anchor_identity.object_ref,
        ));
        for diagnostic in &p.selected_diagnostics {
            out.push_str(&format!(
                "    diagnostic: {} included={}\n",
                diagnostic.kind_class.as_str(),
                diagnostic.included,
            ));
        }
        for attachment in &p.attachments {
            out.push_str(&format!(
                "    attachment: {} (ref={}, redacted={})\n",
                attachment.kind_class.as_str(),
                attachment.attachment_ref,
                attachment.redaction_applied,
            ));
        }
        out.push('\n');

        let d = &self.draft_continuity;
        out.push_str("Draft continuity:\n");
        out.push_str(&format!(
            "- outcome={} intent_preserved={} silent_loss={}\n",
            d.handoff_outcome_class.as_str(),
            d.intent_preserved,
            d.silent_loss,
        ));
        out.push_str(&format!(
            "    preserved visibility={} redaction={}\n",
            d.preserved_visibility_class.as_str(),
            d.preserved_redaction_posture_class.as_str(),
        ));
        if let Some(text_ref) = &d.preserved_draft_text_ref {
            out.push_str(&format!("    preserved draft text: {text_ref}\n"));
        }
        let actions: Vec<&str> = d.available_actions.iter().map(|a| a.as_str()).collect();
        out.push_str(&format!("    actions: {}\n", actions.join(", ")));
        if let Some(fallback) = &d.selected_fallback_ref {
            out.push_str(&format!("    selected fallback: {fallback}\n"));
        }
        out
    }
}

/// Whether a target visibility permits the given data-exit boundary.
fn visibility_allows_data_exit(
    visibility: TargetVisibilityClass,
    data_exit: DataExitBoundary,
) -> bool {
    use DataExitBoundary as D;
    use TargetVisibilityClass as V;
    match visibility {
        V::OfficialPublic | V::Community => matches!(
            data_exit,
            D::NoPayloadLeavesProduct
                | D::MetadataSafeObjectRefs
                | D::ProposalRefsOnly
                | D::ExternalPublicBrowse
        ),
        V::OfficialPrivate => matches!(
            data_exit,
            D::RedactedSupportPacket | D::MetadataSafeObjectRefs | D::NoPayloadLeavesProduct
        ),
        V::SecurityDisclosure => matches!(data_exit, D::SecurityPayloadsOnly),
        V::ThirdPartyVendor => matches!(
            data_exit,
            D::ExternalPublicBrowse | D::VendorOrThirdPartyOutbound
        ),
    }
}

/// Whether a target visibility permits the given network/browser requirement.
fn visibility_allows_network(
    visibility: TargetVisibilityClass,
    network: NetworkBrowserRequirement,
) -> bool {
    use NetworkBrowserRequirement as N;
    use TargetVisibilityClass as V;
    match visibility {
        V::OfficialPublic | V::Community => matches!(
            network,
            N::OfflineCapturePreview | N::SystemBrowserPublicBrowse
        ),
        V::OfficialPrivate => matches!(
            network,
            N::OfflineCapturePreview | N::SystemBrowserAuthenticatedPlane
        ),
        V::SecurityDisclosure => {
            matches!(network, N::OfflineCapturePreview | N::EncryptedSecurityChannel)
        }
        V::ThirdPartyVendor => {
            matches!(network, N::OfflineCapturePreview | N::VendorOrThirdPartyCall)
        }
    }
}

/// True when a ref is an opaque token rather than a raw URL, email, or blank.
fn ref_is_opaque(value: &str) -> bool {
    let trimmed = value.trim();
    !trimmed.is_empty()
        && trimmed == value
        && !trimmed.contains("://")
        && !trimmed.contains('@')
        && !trimmed.contains(char::is_whitespace)
}

fn non_empty(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Closed validation-error vocabulary for the handoff-review contracts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HandoffReviewValidationError {
    WrongTargetSchemaVersion {
        target_id: String,
        actual: u32,
    },
    WrongTargetRecordKind {
        target_id: String,
        actual: String,
    },
    MalformedTargetId {
        target_id: String,
    },
    MissingSafeFallback {
        target_id: String,
    },
    RouteVisibilityMismatch {
        target_id: String,
        route: HandoffRouteClass,
        visibility: TargetVisibilityClass,
    },
    VisibilityDataExitMismatch {
        target_id: String,
        visibility: TargetVisibilityClass,
        data_exit: DataExitBoundary,
    },
    VisibilityNetworkMismatch {
        target_id: String,
        visibility: TargetVisibilityClass,
        network: NetworkBrowserRequirement,
    },
    MissingBuildContextExport {
        target_id: String,
    },
    BuildContextExportSchemaVersionInvalid {
        target_id: String,
        actual: u32,
    },
    BuildContextExportNotRedactionSafe {
        target_id: String,
    },
    BuildContextExportFieldEmpty {
        target_id: String,
    },

    WrongPacketSchemaVersion {
        packet_id: String,
        actual: u32,
    },
    WrongPacketRecordKind {
        packet_id: String,
        actual: String,
    },
    MalformedPacketId {
        packet_id: String,
    },
    RawPayloadNotExcluded {
        packet_id: String,
    },
    NoDiagnosticsSelected {
        packet_id: String,
    },
    AttachmentNotRedacted {
        packet_id: String,
        attachment: AttachmentKindClass,
    },

    WrongSheetSchemaVersion {
        sheet_id: String,
        actual: u32,
    },
    WrongSheetRecordKind {
        sheet_id: String,
        actual: String,
    },
    MalformedSheetId {
        sheet_id: String,
    },
    RedactionPostureUnsafeForVisibility {
        sheet_id: String,
        posture: RedactionPostureClass,
        visibility: TargetVisibilityClass,
    },
    PreservedVisibilityMismatch {
        sheet_id: String,
        preserved: TargetVisibilityClass,
        target: TargetVisibilityClass,
    },
    PreservedRedactionMismatch {
        sheet_id: String,
        preserved: RedactionPostureClass,
        packet: RedactionPostureClass,
    },
    SharedWithoutPreviewConfirmation {
        sheet_id: String,
    },
    SelectedFallbackNotOffered {
        sheet_id: String,
        fallback_ref: String,
    },
    SilentLossNotAllowed {
        sheet_id: String,
    },
    NoPreservationActions {
        sheet_id: String,
    },
    BlockedHandoffDroppedIntent {
        sheet_id: String,
        outcome: HandoffOutcomeClass,
    },
    BlockedHandoffMissingDraftText {
        sheet_id: String,
        outcome: HandoffOutcomeClass,
    },
    BlockedHandoffMissingPreservationActions {
        sheet_id: String,
        outcome: HandoffOutcomeClass,
    },

    WrongContractDocRef {
        record_id: String,
        actual: String,
    },
    EmptyRequiredField {
        record_id: String,
        field: &'static str,
    },
    RawRefLeak {
        record_id: String,
        field: &'static str,
    },
}

impl fmt::Display for HandoffReviewValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongTargetSchemaVersion { target_id, actual } => write!(
                f,
                "handoff target {target_id} has unsupported handoff_target_review_schema_version {actual}"
            ),
            Self::WrongTargetRecordKind { target_id, actual } => write!(
                f,
                "handoff target {target_id} has unsupported record kind {actual}"
            ),
            Self::MalformedTargetId { target_id } => {
                write!(f, "handoff target id {target_id} must start with handoff_target:")
            }
            Self::MissingSafeFallback { target_id } => write!(
                f,
                "handoff target {target_id} must offer at least one safe fallback route"
            ),
            Self::RouteVisibilityMismatch {
                target_id,
                route,
                visibility,
            } => write!(
                f,
                "handoff target {target_id} route {} cannot target visibility {}",
                route.as_str(),
                visibility.as_str()
            ),
            Self::VisibilityDataExitMismatch {
                target_id,
                visibility,
                data_exit,
            } => write!(
                f,
                "handoff target {target_id} visibility {} cannot use data exit {}",
                visibility.as_str(),
                data_exit.as_str()
            ),
            Self::VisibilityNetworkMismatch {
                target_id,
                visibility,
                network,
            } => write!(
                f,
                "handoff target {target_id} visibility {} cannot use network requirement {}",
                visibility.as_str(),
                network.as_str()
            ),
            Self::MissingBuildContextExport { target_id } => write!(
                f,
                "handoff target {target_id} must attach a build-context export block"
            ),
            Self::BuildContextExportSchemaVersionInvalid { target_id, actual } => write!(
                f,
                "handoff target {target_id} has invalid build-context export schema version {actual}"
            ),
            Self::BuildContextExportNotRedactionSafe { target_id } => write!(
                f,
                "handoff target {target_id} build-context export is not redaction safe"
            ),
            Self::BuildContextExportFieldEmpty { target_id } => write!(
                f,
                "handoff target {target_id} has an empty build-context export field"
            ),
            Self::WrongPacketSchemaVersion { packet_id, actual } => write!(
                f,
                "repro packet {packet_id} has unsupported repro_packet_preview_schema_version {actual}"
            ),
            Self::WrongPacketRecordKind { packet_id, actual } => write!(
                f,
                "repro packet {packet_id} has unsupported record kind {actual}"
            ),
            Self::MalformedPacketId { packet_id } => write!(
                f,
                "repro packet id {packet_id} must start with repro_packet_preview:"
            ),
            Self::RawPayloadNotExcluded { packet_id } => write!(
                f,
                "repro packet {packet_id} must exclude raw secrets and raw screenshots"
            ),
            Self::NoDiagnosticsSelected { packet_id } => write!(
                f,
                "repro packet {packet_id} must include at least one selected diagnostic"
            ),
            Self::AttachmentNotRedacted {
                packet_id,
                attachment,
            } => write!(
                f,
                "repro packet {packet_id} attachment {} must be redacted before share",
                attachment.as_str()
            ),
            Self::WrongSheetSchemaVersion { sheet_id, actual } => write!(
                f,
                "handoff sheet {sheet_id} has unsupported handoff_review_sheet_schema_version {actual}"
            ),
            Self::WrongSheetRecordKind { sheet_id, actual } => write!(
                f,
                "handoff sheet {sheet_id} has unsupported record kind {actual}"
            ),
            Self::MalformedSheetId { sheet_id } => write!(
                f,
                "handoff sheet id {sheet_id} must start with handoff_review_sheet:"
            ),
            Self::RedactionPostureUnsafeForVisibility {
                sheet_id,
                posture,
                visibility,
            } => write!(
                f,
                "handoff sheet {sheet_id} redaction posture {} is unsafe for visibility {}",
                posture.as_str(),
                visibility.as_str()
            ),
            Self::PreservedVisibilityMismatch {
                sheet_id,
                preserved,
                target,
            } => write!(
                f,
                "handoff sheet {sheet_id} preserved visibility {} does not match target visibility {}",
                preserved.as_str(),
                target.as_str()
            ),
            Self::PreservedRedactionMismatch {
                sheet_id,
                preserved,
                packet,
            } => write!(
                f,
                "handoff sheet {sheet_id} preserved redaction {} does not match packet redaction {}",
                preserved.as_str(),
                packet.as_str()
            ),
            Self::SharedWithoutPreviewConfirmation { sheet_id } => write!(
                f,
                "handoff sheet {sheet_id} opened the browser without a confirmed preview"
            ),
            Self::SelectedFallbackNotOffered {
                sheet_id,
                fallback_ref,
            } => write!(
                f,
                "handoff sheet {sheet_id} selected fallback {fallback_ref} is not an offered safe fallback"
            ),
            Self::SilentLossNotAllowed { sheet_id } => write!(
                f,
                "handoff sheet {sheet_id} declares silent_loss; drafts must be preserved"
            ),
            Self::NoPreservationActions { sheet_id } => write!(
                f,
                "handoff sheet {sheet_id} draft continuity must offer at least one action"
            ),
            Self::BlockedHandoffDroppedIntent { sheet_id, outcome } => write!(
                f,
                "handoff sheet {sheet_id} blocked outcome {} must preserve intent",
                outcome.as_str()
            ),
            Self::BlockedHandoffMissingDraftText { sheet_id, outcome } => write!(
                f,
                "handoff sheet {sheet_id} blocked outcome {} must preserve the draft text",
                outcome.as_str()
            ),
            Self::BlockedHandoffMissingPreservationActions { sheet_id, outcome } => write!(
                f,
                "handoff sheet {sheet_id} blocked outcome {} must offer export and local save",
                outcome.as_str()
            ),
            Self::WrongContractDocRef { record_id, actual } => write!(
                f,
                "record {record_id} cites wrong contract doc {actual}"
            ),
            Self::EmptyRequiredField { record_id, field } => {
                write!(f, "record {record_id} is missing required field {field}")
            }
            Self::RawRefLeak { record_id, field } => write!(
                f,
                "record {record_id} field {field} contains a raw URL, email, or whitespace; opaque refs only"
            ),
        }
    }
}

impl std::error::Error for HandoffReviewValidationError {}

/// Convenience: validate a slice of sheets and reject duplicate sheet ids.
pub fn validate_sheets(sheets: &[HandoffReviewSheet]) -> Result<(), HandoffReviewValidationError> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for sheet in sheets {
        sheet.validate()?;
        if !seen.insert(sheet.sheet_id.as_str()) {
            return Err(HandoffReviewValidationError::MalformedSheetId {
                sheet_id: sheet.sheet_id.clone(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests;
