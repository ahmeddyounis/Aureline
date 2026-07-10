//! Rendered compare viewers, media-metadata rails, and redaction-or-trust badge
//! sets carrying scale/dimension metadata, alt-text/text fallback, sandbox/trust
//! state, format and size/duration/dimension measures, hidden-content posture,
//! sanitized/sandboxed/export-safe safety, local/team/support share guidance, and
//! explicit open-raw/export actions.
//!
//! This module narrows the `rendered_compare_viewer`, `media_metadata_rail`, and
//! `redaction_or_trust_badge_set` components frozen in
//! [`crate::freeze_the_m5_structured_artifact_review_component_matrix`] into
//! implemented, export-safe review controls. Every [`RenderedCompareViewer`]
//! answers, from the component alone, which media-like artifact is being compared,
//! its scale or dimension metadata, its accessibility text fallback, whether it is
//! sandboxed and whether its render is trusted, and how to open the raw artifact
//! or export it — so a rendered report or image-like artifact is never reviewed
//! without its accessibility or trust boundary. Every [`MediaMetadataRail`] names
//! the artifact format, its size / duration / dimensions, its hidden-content
//! state, its sanitized / sandboxed / export-safe posture, and its local / team /
//! support share guidance. Every [`RedactionOrTrustBadgeSet`] names the redaction
//! state, the trust level, and whether the redaction or trust posture is preserved
//! when the review is shared or exported, so share and export flows never flatten
//! rendered review into ambiguous attachments.
//!
//! The three controls are joined by artifact reference: every rendered compare
//! viewer and every media-metadata rail is accompanied by a redaction-or-trust
//! badge set for the same artifact, so the redaction and trust posture is always
//! visible wherever a media-like artifact is reviewed, shared, or exported.
//!
//! The fidelity-narrowing vocabulary ([`M5ArtifactFidelityState`]) and rollback
//! posture ([`M5ArtifactComponentRollbackPosture`]) are reused directly from the
//! frozen matrix so schema state and write-back safety read the same everywhere.
//! The packet references the upstream artifact-component-matrix, safe-preview,
//! design-snapshot, and redaction-row contracts by id rather than embedding their
//! content. Raw artifact bodies, raw render payloads, raw media bytes, credentials,
//! and live provider responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-rendered-compare-media-trust-controls.schema.json`](../../../../schemas/ui/m5-rendered-compare-media-trust-controls.schema.json).
//! The contract doc is
//! [`docs/review/m5/ship_rendered_compare_viewers_media_metadata_rails_and_redaction_or_trust_badge_sets.md`](../../../../docs/review/m5/ship_rendered_compare_viewers_media_metadata_rails_and_redaction_or_trust_badge_sets.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-rendered-compare-media-trust-controls/`](../../../../fixtures/ui/m5-rendered-compare-media-trust-controls/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_structured_artifact_review_component_matrix::{
    M5ArtifactComponent, M5ArtifactComponentRollbackPosture, M5ArtifactFidelityState,
    M5_ARTIFACT_COMPONENT_MATRIX_MEDIA_RAIL_CONTRACT_REF,
    M5_ARTIFACT_COMPONENT_MATRIX_REDACTION_BADGE_CONTRACT_REF,
    M5_ARTIFACT_COMPONENT_MATRIX_RENDERED_VIEWER_CONTRACT_REF,
    M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`MediaCompareControlsPacket`].
pub const MEDIA_COMPARE_CONTROLS_RECORD_KIND: &str = "rendered_compare_media_trust_controls";

/// Schema version for rendered-compare / media-rail / trust-badge control records.
pub const MEDIA_COMPARE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MEDIA_COMPARE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-rendered-compare-media-trust-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const MEDIA_COMPARE_CONTROLS_DOC_REF: &str =
    "docs/review/m5/ship_rendered_compare_viewers_media_metadata_rails_and_redaction_or_trust_badge_sets.md";

/// Repo-relative path of the protected fixture directory.
pub const MEDIA_COMPARE_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-rendered-compare-media-trust-controls";

/// Repo-relative path of the checked support-export artifact.
pub const MEDIA_COMPARE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-rendered-compare-media-trust-controls-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const MEDIA_COMPARE_CONTROLS_SUMMARY_REF: &str =
    "artifacts/release/m5-rendered-compare-media-trust-controls-proof/summary.md";

/// Sandbox / trust class of a rendered compare view.
///
/// This is the core honesty axis for the rendered compare viewer: a rendered
/// preview never hides whether it was sandboxed and whether its render is trusted,
/// so an untrusted render, a raw text fallback, or a redacted render is always
/// labeled as such rather than presented as a faithful, trusted rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderTrustClass {
    /// Rendered inside a sandbox and trusted to display faithfully.
    SandboxedTrusted,
    /// Rendered inside a sandbox but the render is not fully trusted.
    SandboxedUntrusted,
    /// No trusted render; falls back to an explicitly labeled raw/text view.
    RawTextFallback,
    /// The render is withheld under the redaction posture.
    RedactedWithheld,
}

impl RenderTrustClass {
    /// Every render trust class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SandboxedTrusted,
        Self::SandboxedUntrusted,
        Self::RawTextFallback,
        Self::RedactedWithheld,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SandboxedTrusted => "sandboxed_trusted",
            Self::SandboxedUntrusted => "sandboxed_untrusted",
            Self::RawTextFallback => "raw_text_fallback",
            Self::RedactedWithheld => "redacted_withheld",
        }
    }

    /// Whether the render happens inside a sandbox.
    pub const fn is_sandboxed(self) -> bool {
        matches!(self, Self::SandboxedTrusted | Self::SandboxedUntrusted)
    }

    /// Whether a rendered preview is shown at all (rather than withheld).
    pub const fn is_render_shown(self) -> bool {
        !matches!(self, Self::RedactedWithheld)
    }
}

/// An action a rendered compare viewer may offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenderedViewerAction {
    /// Open the raw artifact behind the render.
    OpenRaw,
    /// Export the artifact through an export-safe path.
    Export,
    /// Toggle the render scale or zoom.
    ToggleScale,
    /// View the accessibility text fallback.
    ViewTextFallback,
    /// Compare the two sides side by side.
    CompareSideBySide,
}

impl RenderedViewerAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRaw => "open_raw",
            Self::Export => "export",
            Self::ToggleScale => "toggle_scale",
            Self::ViewTextFallback => "view_text_fallback",
            Self::CompareSideBySide => "compare_side_by_side",
        }
    }
}

/// The kind of media-like artifact a metadata rail describes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaArtifactKind {
    /// A raster or vector image.
    Image,
    /// A video capture.
    Video,
    /// An audio capture.
    Audio,
    /// A design snapshot.
    DesignSnapshot,
    /// A rendered document or report.
    RenderedDocument,
    /// Some other media-like artifact.
    Other,
}

impl MediaArtifactKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::Video => "video",
            Self::Audio => "audio",
            Self::DesignSnapshot => "design_snapshot",
            Self::RenderedDocument => "rendered_document",
            Self::Other => "other",
        }
    }
}

/// The kind of measure a media-metadata rail carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaMeasureKind {
    /// Pixel or unit dimensions.
    Dimensions,
    /// Playback duration.
    Duration,
    /// Byte size.
    ByteSize,
    /// Page or frame count.
    PageCount,
}

impl MediaMeasureKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dimensions => "dimensions",
            Self::Duration => "duration",
            Self::ByteSize => "byte_size",
            Self::PageCount => "page_count",
        }
    }
}

/// The hidden-content state of a media-like artifact.
///
/// A media-like artifact can carry embedded sensitive content — stripped-away
/// layers, embedded metadata, or off-canvas material. This axis keeps that state
/// explicit so hidden content is never shared or exported without disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HiddenContentState {
    /// No embedded sensitive content was detected.
    NoEmbeddedSensitiveContent,
    /// Embedded sensitive content is present and must be disclosed.
    EmbeddedSensitiveContentPresent,
    /// The artifact was not scanned; the hidden-content state is unknown.
    EmbeddedContentScanUnknown,
}

impl HiddenContentState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::NoEmbeddedSensitiveContent,
        Self::EmbeddedSensitiveContentPresent,
        Self::EmbeddedContentScanUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoEmbeddedSensitiveContent => "no_embedded_sensitive_content",
            Self::EmbeddedSensitiveContentPresent => "embedded_sensitive_content_present",
            Self::EmbeddedContentScanUnknown => "embedded_content_scan_unknown",
        }
    }

    /// Whether embedded sensitive content is present.
    pub const fn has_hidden_content(self) -> bool {
        matches!(self, Self::EmbeddedSensitiveContentPresent)
    }

    /// Whether the hidden-content scan state is unknown.
    pub const fn is_scan_unknown(self) -> bool {
        matches!(self, Self::EmbeddedContentScanUnknown)
    }
}

/// The sanitized / sandboxed / export-safe posture of a media-like artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaSafetyPosture {
    /// Raw and unsanitized; safe only for local review.
    RawUnsanitized,
    /// Sanitized so embedded sensitive content is stripped.
    Sanitized,
    /// Rendered inside a sandbox that isolates the payload.
    Sandboxed,
    /// Export-safe and cleared to leave the local boundary.
    ExportSafe,
}

impl MediaSafetyPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawUnsanitized => "raw_unsanitized",
            Self::Sanitized => "sanitized",
            Self::Sandboxed => "sandboxed",
            Self::ExportSafe => "export_safe",
        }
    }

    /// Whether this posture is safe to share beyond the local boundary.
    pub const fn is_share_safe(self) -> bool {
        matches!(self, Self::Sanitized | Self::ExportSafe)
    }
}

/// The local / team / support share scope guidance for a media-like artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaShareScope {
    /// Local review only.
    LocalOnly,
    /// Shareable with the team.
    TeamShare,
    /// Exportable to a support packet.
    SupportExport,
}

impl MediaShareScope {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::TeamShare => "team_share",
            Self::SupportExport => "support_export",
        }
    }

    /// Whether this scope shares the artifact beyond the local boundary.
    pub const fn is_shareable_beyond_local(self) -> bool {
        matches!(self, Self::TeamShare | Self::SupportExport)
    }
}

/// The redaction state of a redaction-or-trust badge set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionState {
    /// Nothing is redacted.
    NotRedacted,
    /// Part of the artifact is redacted.
    PartiallyRedacted,
    /// The artifact is fully redacted.
    FullyRedacted,
    /// Redaction is pending and not yet applied.
    RedactionPending,
}

impl RedactionState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NotRedacted,
        Self::PartiallyRedacted,
        Self::FullyRedacted,
        Self::RedactionPending,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRedacted => "not_redacted",
            Self::PartiallyRedacted => "partially_redacted",
            Self::FullyRedacted => "fully_redacted",
            Self::RedactionPending => "redaction_pending",
        }
    }

    /// Whether content is redacted.
    pub const fn is_redacted(self) -> bool {
        matches!(self, Self::PartiallyRedacted | Self::FullyRedacted)
    }

    /// Whether a redaction note must accompany this state.
    pub const fn needs_redaction_note(self) -> bool {
        matches!(
            self,
            Self::PartiallyRedacted | Self::FullyRedacted | Self::RedactionPending
        )
    }
}

/// The trust level named by a redaction-or-trust badge set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustLevel {
    /// Fully trusted content.
    Trusted,
    /// Trusted only within a sandbox.
    SandboxedOnly,
    /// Untrusted content.
    Untrusted,
    /// Trust could not be verified.
    Unverified,
}

impl TrustLevel {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::SandboxedOnly => "sandboxed_only",
            Self::Untrusted => "untrusted",
            Self::Unverified => "unverified",
        }
    }

    /// Whether this level requires an untrusted note.
    pub const fn is_untrusted(self) -> bool {
        matches!(self, Self::Untrusted | Self::Unverified)
    }
}

/// A badge a redaction-or-trust badge set may carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustBadge {
    /// Content is redacted.
    Redacted,
    /// Content is sanitized.
    Sanitized,
    /// Content is sandboxed.
    Sandboxed,
    /// Content is export-safe.
    ExportSafe,
    /// Content is trusted.
    Trusted,
    /// Content is untrusted.
    Untrusted,
    /// Content is local-only.
    LocalOnly,
}

impl TrustBadge {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Redacted => "redacted",
            Self::Sanitized => "sanitized",
            Self::Sandboxed => "sandboxed",
            Self::ExportSafe => "export_safe",
            Self::Trusted => "trusted",
            Self::Untrusted => "untrusted",
            Self::LocalOnly => "local_only",
        }
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaCompareControlsDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// The rendered preview is not trusted.
    RenderUntrusted,
    /// Media metadata could not be extracted.
    MediaMetadataUnavailable,
    /// Embedded sensitive content was detected.
    HiddenContentDetected,
    /// Redaction was applied and narrows visible content.
    RedactionApplied,
    /// Sandbox rendering is enforced; the raw render is unavailable.
    SandboxEnforced,
    /// Control trust narrowed.
    TrustNarrowing,
    /// An upstream dependency component narrowed.
    UpstreamDependencyNarrowed,
}

impl MediaCompareControlsDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::RenderUntrusted,
        Self::MediaMetadataUnavailable,
        Self::HiddenContentDetected,
        Self::RedactionApplied,
        Self::SandboxEnforced,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::RenderUntrusted => "render_untrusted",
            Self::MediaMetadataUnavailable => "media_metadata_unavailable",
            Self::HiddenContentDetected => "hidden_content_detected",
            Self::RedactionApplied => "redaction_applied",
            Self::SandboxEnforced => "sandbox_enforced",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must reuse these controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaCompareControlsConsumerSurface {
    /// Diff / compare view.
    DiffCompareView,
    /// Notebook review surface.
    NotebookReview,
    /// Artifact browser (coverage, profile, crash, SBOM, media adjuncts).
    ArtifactBrowser,
    /// Design-snapshot review surface.
    DesignSnapshotReview,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Help / About surface.
    HelpAbout,
}

impl MediaCompareControlsConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DiffCompareView,
        Self::NotebookReview,
        Self::ArtifactBrowser,
        Self::DesignSnapshotReview,
        Self::CliHeadless,
        Self::SupportExport,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiffCompareView => "diff_compare_view",
            Self::NotebookReview => "notebook_review",
            Self::ArtifactBrowser => "artifact_browser",
            Self::DesignSnapshotReview => "design_snapshot_review",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Disclosures a rendered compare viewer must carry, derived from its trust class.
///
/// A sandboxed render must carry a sandbox note; an untrusted render must carry an
/// untrusted-render note; a raw text fallback must label the raw fallback
/// explicitly rather than presenting it as a faithful render; a withheld render
/// must carry a redaction note. Only a sandboxed-trusted render is directly
/// trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderedViewerDisclosure {
    /// Whether the viewer must carry a sandbox note.
    pub needs_sandbox_note: bool,
    /// Whether the viewer must carry an untrusted-render note.
    pub needs_untrusted_render_note: bool,
    /// Whether the viewer must explicitly label a raw/text fallback.
    pub needs_raw_fallback_label: bool,
    /// Whether the viewer must carry a redaction note for a withheld render.
    pub needs_redaction_note: bool,
    /// Whether the render is directly trusted (sandboxed and trusted).
    pub render_directly_trusted: bool,
}

/// Resolves the disclosures a rendered compare viewer must carry from its trust class.
pub fn resolve_rendered_viewer_disclosure(
    trust_class: RenderTrustClass,
) -> RenderedViewerDisclosure {
    RenderedViewerDisclosure {
        needs_sandbox_note: trust_class.is_sandboxed(),
        needs_untrusted_render_note: matches!(trust_class, RenderTrustClass::SandboxedUntrusted),
        needs_raw_fallback_label: matches!(trust_class, RenderTrustClass::RawTextFallback),
        needs_redaction_note: matches!(trust_class, RenderTrustClass::RedactedWithheld),
        render_directly_trusted: matches!(trust_class, RenderTrustClass::SandboxedTrusted),
    }
}

/// Disclosures a media-metadata rail must carry, derived from its hidden-content state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MediaRailDisclosure {
    /// Whether the rail must carry a hidden-content note.
    pub needs_hidden_content_note: bool,
    /// Whether the rail must carry an unknown-scan note.
    pub needs_unknown_scan_note: bool,
}

/// Resolves the disclosures a media-metadata rail must carry from its hidden-content state.
pub fn resolve_media_rail_disclosure(state: HiddenContentState) -> MediaRailDisclosure {
    MediaRailDisclosure {
        needs_hidden_content_note: state.has_hidden_content() || state.is_scan_unknown(),
        needs_unknown_scan_note: state.is_scan_unknown(),
    }
}

/// Disclosures a redaction-or-trust badge set must carry, derived from its state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BadgeSetDisclosure {
    /// Whether the badge set must carry a redaction note.
    pub needs_redaction_note: bool,
    /// Whether the badge set must carry an untrusted note.
    pub needs_untrusted_note: bool,
}

/// Resolves the disclosures a redaction-or-trust badge set must carry.
pub fn resolve_badge_set_disclosure(
    redaction_state: RedactionState,
    trust_level: TrustLevel,
) -> BadgeSetDisclosure {
    BadgeSetDisclosure {
        needs_redaction_note: redaction_state.needs_redaction_note(),
        needs_untrusted_note: trust_level.is_untrusted(),
    }
}

/// A rendered compare viewer carrying scale metadata, accessibility fallback, and trust state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderedCompareViewer {
    /// Frozen component this viewer implements; must be `rendered_compare_viewer`.
    pub component: M5ArtifactComponent,
    /// Stable viewer id.
    pub viewer_id: String,
    /// Stable artifact reference; shared with a redaction-or-trust badge set.
    pub artifact_ref: String,
    /// Human-readable artifact-class label; required and non-empty.
    pub artifact_class_label: String,
    /// Sandbox / trust class of the render.
    pub trust_class: RenderTrustClass,
    /// Scale or dimension metadata; required and non-empty.
    pub scale_or_dimension_metadata: String,
    /// Accessibility text / alt-text fallback; required and non-empty.
    pub alt_text_fallback: String,
    /// Sandbox note; required and non-empty when the render is sandboxed.
    pub sandbox_note: String,
    /// Untrusted-render note; required and non-empty when the render is untrusted.
    pub untrusted_render_note: String,
    /// Raw/text fallback label; required and non-empty when falling back to raw text.
    pub raw_fallback_label: String,
    /// Redaction note; required and non-empty when the render is withheld.
    pub redaction_note: String,
    /// Actions offered on this viewer; must include open-raw and export.
    pub available_actions: Vec<RenderedViewerAction>,
    /// Schema fidelity state, reused from the frozen component matrix.
    pub schema_fidelity: M5ArtifactFidelityState,
    /// Raw-context jump action; required and non-empty.
    pub raw_context_action: String,
    /// Rollback / write-back posture, reused from the frozen component matrix.
    pub rollback_posture: M5ArtifactComponentRollbackPosture,
    /// Viewer fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this viewer.
    pub source_contract_refs: Vec<String>,
}

impl RenderedCompareViewer {
    /// Disclosures this viewer must carry, derived from its trust class.
    pub fn disclosure(&self) -> RenderedViewerDisclosure {
        resolve_rendered_viewer_disclosure(self.trust_class)
    }

    /// Whether this viewer offers the given action.
    pub fn offers(&self, action: RenderedViewerAction) -> bool {
        self.available_actions.contains(&action)
    }
}

/// A media-metadata rail carrying format, measure, hidden-content state, and share guidance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaMetadataRail {
    /// Frozen component this rail implements; must be `media_metadata_rail`.
    pub component: M5ArtifactComponent,
    /// Stable rail id.
    pub rail_id: String,
    /// Stable artifact reference; shared with a redaction-or-trust badge set.
    pub artifact_ref: String,
    /// The kind of media-like artifact.
    pub artifact_kind: MediaArtifactKind,
    /// Artifact format label; required and non-empty.
    pub format_label: String,
    /// The kind of measure this rail carries.
    pub measure_kind: MediaMeasureKind,
    /// The measure value (size / duration / dimensions); required and non-empty.
    pub measure_value: String,
    /// Hidden-content state.
    pub hidden_content_state: HiddenContentState,
    /// Hidden-content note; required and non-empty when content is present or unknown.
    pub hidden_content_note: String,
    /// Sanitized / sandboxed / export-safe posture.
    pub safety_posture: MediaSafetyPosture,
    /// Local / team / support share scope.
    pub share_scope: MediaShareScope,
    /// Local / team / support share guidance; required and non-empty.
    pub share_guidance: String,
    /// Schema fidelity state, reused from the frozen component matrix.
    pub schema_fidelity: M5ArtifactFidelityState,
    /// Raw-context jump action; required and non-empty.
    pub raw_context_action: String,
    /// Rollback / write-back posture, reused from the frozen component matrix.
    pub rollback_posture: M5ArtifactComponentRollbackPosture,
    /// Rail fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this rail.
    pub source_contract_refs: Vec<String>,
}

impl MediaMetadataRail {
    /// Disclosures this rail must carry, derived from its hidden-content state.
    pub fn disclosure(&self) -> MediaRailDisclosure {
        resolve_media_rail_disclosure(self.hidden_content_state)
    }

    /// Whether this rail would share embedded sensitive content beyond the local
    /// boundary without a sanitized or export-safe posture.
    pub fn shares_unsanitized_hidden_content(&self) -> bool {
        self.hidden_content_state.has_hidden_content()
            && self.share_scope.is_shareable_beyond_local()
            && !self.safety_posture.is_share_safe()
    }
}

/// A redaction-or-trust badge set naming redaction state, trust level, and export posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionOrTrustBadgeSet {
    /// Frozen component this set implements; must be `redaction_or_trust_badge_set`.
    pub component: M5ArtifactComponent,
    /// Stable badge-set id.
    pub badge_set_id: String,
    /// Stable artifact reference; shared with a viewer or rail.
    pub artifact_ref: String,
    /// Redaction state.
    pub redaction_state: RedactionState,
    /// Trust level.
    pub trust_level: TrustLevel,
    /// Badges carried by this set; must be non-empty.
    pub available_badges: Vec<TrustBadge>,
    /// Redaction note; required and non-empty when the redaction state requires it.
    pub redaction_note: String,
    /// Untrusted note; required and non-empty when the trust level is untrusted.
    pub untrusted_note: String,
    /// Local / team / support share guidance; required and non-empty.
    pub share_guidance: String,
    /// Whether the redaction / trust posture is preserved when shared or exported.
    pub export_posture_preserved: bool,
    /// Schema fidelity state, reused from the frozen component matrix.
    pub schema_fidelity: M5ArtifactFidelityState,
    /// Raw-context jump action; required and non-empty.
    pub raw_context_action: String,
    /// Rollback / write-back posture, reused from the frozen component matrix.
    pub rollback_posture: M5ArtifactComponentRollbackPosture,
    /// Badge-set fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this set.
    pub source_contract_refs: Vec<String>,
}

impl RedactionOrTrustBadgeSet {
    /// Disclosures this badge set must carry, derived from its state.
    pub fn disclosure(&self) -> BadgeSetDisclosure {
        resolve_badge_set_disclosure(self.redaction_state, self.trust_level)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaCompareControlsTrustReview {
    /// Render trust and sandbox state stay explicit on every viewer.
    pub render_trust_always_explicit: bool,
    /// An accessibility text fallback is always present on every viewer.
    pub accessibility_fallback_always_present: bool,
    /// Scale or dimension metadata is always present on every viewer.
    pub scale_or_dimension_metadata_present: bool,
    /// Hidden-content state is always disclosed on media rails.
    pub hidden_content_state_disclosed: bool,
    /// Metadata visibility stays explicit for media-like artifacts.
    pub metadata_visibility_explicit: bool,
    /// Sanitized / sandboxed / export-safe posture stays explicit.
    pub sanitized_or_export_safe_posture_explicit: bool,
    /// Local / team / support share guidance stays explicit.
    pub share_guidance_explicit: bool,
    /// Redaction / trust posture is preserved when shared or exported.
    pub redaction_posture_preserved_on_export: bool,
    /// Raw / export-safe fallbacks stay explicit when render fidelity narrows.
    pub raw_export_safe_fallback_explicit: bool,
    /// A raw-context jump action is always reachable from every control.
    pub raw_context_always_reachable: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified controls automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl MediaCompareControlsTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.render_trust_always_explicit
            && self.accessibility_fallback_always_present
            && self.scale_or_dimension_metadata_present
            && self.hidden_content_state_disclosed
            && self.metadata_visibility_explicit
            && self.sanitized_or_export_safe_posture_explicit
            && self.share_guidance_explicit
            && self.redaction_posture_preserved_on_export
            && self.raw_export_safe_fallback_explicit
            && self.raw_context_always_reachable
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaCompareControlsConsumerProjection {
    /// Rendered viewer shows its render-trust class and scale metadata.
    pub rendered_viewer_shows_render_trust_and_scale: bool,
    /// Rendered viewer shows its accessibility text fallback.
    pub rendered_viewer_shows_alt_text_fallback: bool,
    /// Media rail shows format and measure.
    pub media_rail_shows_format_and_measure: bool,
    /// Media rail shows hidden-content state and posture.
    pub media_rail_shows_hidden_content_and_posture: bool,
    /// Badge set shows redaction state and trust level.
    pub badge_set_shows_redaction_and_trust: bool,
    /// Share / export preserves redaction / trust posture.
    pub share_export_preserves_posture: bool,
    /// Raw context is reachable from every control.
    pub raw_context_reachable_from_all: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_truth: bool,
}

impl MediaCompareControlsConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.rendered_viewer_shows_render_trust_and_scale
            && self.rendered_viewer_shows_alt_text_fallback
            && self.media_rail_shows_format_and_measure
            && self.media_rail_shows_hidden_content_and_posture
            && self.badge_set_shows_redaction_and_trust
            && self.share_export_preserves_posture
            && self.raw_context_reachable_from_all
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.help_about_shows_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaCompareControlsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`MediaCompareControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaCompareControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Rendered compare viewers.
    pub rendered_compare_viewers: Vec<RenderedCompareViewer>,
    /// Media-metadata rails.
    pub media_metadata_rails: Vec<MediaMetadataRail>,
    /// Redaction-or-trust badge sets.
    pub redaction_trust_badge_sets: Vec<RedactionOrTrustBadgeSet>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<MediaCompareControlsDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<MediaCompareControlsConsumerSurface>,
    /// Trust review block.
    pub trust_review: MediaCompareControlsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: MediaCompareControlsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: MediaCompareControlsProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe rendered-compare / media-rail / trust-badge controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MediaCompareControlsPacket {
    /// Record kind; must equal [`MEDIA_COMPARE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MEDIA_COMPARE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Rendered compare viewers.
    pub rendered_compare_viewers: Vec<RenderedCompareViewer>,
    /// Media-metadata rails.
    pub media_metadata_rails: Vec<MediaMetadataRail>,
    /// Redaction-or-trust badge sets.
    pub redaction_trust_badge_sets: Vec<RedactionOrTrustBadgeSet>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<MediaCompareControlsDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<MediaCompareControlsConsumerSurface>,
    /// Trust review block.
    pub trust_review: MediaCompareControlsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: MediaCompareControlsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: MediaCompareControlsProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl MediaCompareControlsPacket {
    /// Builds a rendered-compare / media-rail / trust-badge controls packet.
    pub fn new(input: MediaCompareControlsPacketInput) -> Self {
        Self {
            record_kind: MEDIA_COMPARE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: MEDIA_COMPARE_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            rendered_compare_viewers: input.rendered_compare_viewers,
            media_metadata_rails: input.media_metadata_rails,
            redaction_trust_badge_sets: input.redaction_trust_badge_sets,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the rendered-compare / media-rail / trust-badge controls invariants.
    pub fn validate(&self) -> Vec<MediaCompareControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != MEDIA_COMPARE_CONTROLS_RECORD_KIND {
            violations.push(MediaCompareControlsViolation::WrongRecordKind);
        }
        if self.schema_version != MEDIA_COMPARE_CONTROLS_SCHEMA_VERSION {
            violations.push(MediaCompareControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(MediaCompareControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(MediaCompareControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(MediaCompareControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rendered_compare_viewers(self, &mut violations);
        validate_media_metadata_rails(self, &mut violations);
        validate_redaction_trust_badge_sets(self, &mut violations);
        validate_pairing(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(MediaCompareControlsViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(MediaCompareControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(MediaCompareControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("media compare controls packet serializes"),
        ) {
            violations.push(MediaCompareControlsViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("media compare controls packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let untrusted_renders = self
            .rendered_compare_viewers
            .iter()
            .filter(|viewer| !viewer.disclosure().render_directly_trusted)
            .count();
        let rails_with_hidden_content = self
            .media_metadata_rails
            .iter()
            .filter(|rail| rail.disclosure().needs_hidden_content_note)
            .count();
        let redacted_sets = self
            .redaction_trust_badge_sets
            .iter()
            .filter(|set| set.redaction_state.is_redacted())
            .count();

        let mut out = String::new();
        out.push_str("# Rendered Compare Viewers, Media Rails & Trust Badge Sets\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Rendered compare viewers: {} ({} not directly trusted)\n",
            self.rendered_compare_viewers.len(),
            untrusted_renders
        ));
        out.push_str(&format!(
            "- Media-metadata rails: {} ({} carry hidden-content notes)\n",
            self.media_metadata_rails.len(),
            rails_with_hidden_content
        ));
        out.push_str(&format!(
            "- Redaction / trust badge sets: {} ({} redacted)\n",
            self.redaction_trust_badge_sets.len(),
            redacted_sets
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Rendered compare viewers\n\n");
        for viewer in &self.rendered_compare_viewers {
            out.push_str(&format!(
                "- **{}** [`{}`]: {} — scale `{}`\n",
                viewer.artifact_class_label,
                viewer.artifact_ref,
                viewer.trust_class.as_str(),
                viewer.scale_or_dimension_metadata
            ));
        }

        out.push_str("\n## Media-metadata rails\n\n");
        for rail in &self.media_metadata_rails {
            out.push_str(&format!(
                "- **{}** [`{}`]: {} {} — {} ({}), share {}\n",
                rail.format_label,
                rail.artifact_ref,
                rail.measure_kind.as_str(),
                rail.measure_value,
                rail.hidden_content_state.as_str(),
                rail.safety_posture.as_str(),
                rail.share_scope.as_str()
            ));
        }

        out.push_str("\n## Redaction / trust badge sets\n\n");
        for set in &self.redaction_trust_badge_sets {
            out.push_str(&format!(
                "- [`{}`]: {} / {} — export posture preserved: {}\n",
                set.artifact_ref,
                set.redaction_state.as_str(),
                set.trust_level.as_str(),
                set.export_posture_preserved
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in media/compare-controls export.
#[derive(Debug)]
pub enum MediaCompareControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<MediaCompareControlsViolation>),
}

impl fmt::Display for MediaCompareControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "media compare controls export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "media compare controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for MediaCompareControlsArtifactError {}

/// Validation failures emitted by [`MediaCompareControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MediaCompareControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No rendered compare viewers are present.
    RenderedCompareViewersMissing,
    /// A rendered compare viewer is incomplete.
    RenderedCompareViewerIncomplete,
    /// A rendered compare viewer carries the wrong frozen component class.
    RenderedCompareViewerWrongComponentClass,
    /// A rendered compare viewer does not name its artifact class.
    RenderedArtifactClassMissing,
    /// A rendered compare viewer does not carry scale or dimension metadata.
    ScaleOrDimensionMetadataMissing,
    /// A rendered compare viewer does not carry an accessibility text fallback.
    AltTextFallbackMissing,
    /// A sandboxed render does not carry a sandbox note.
    SandboxNoteMissing,
    /// An untrusted render does not carry an untrusted-render note.
    UntrustedRenderNoteMissing,
    /// A raw text fallback is not explicitly labeled.
    RawFallbackLabelMissing,
    /// A withheld render does not carry a redaction note.
    RenderRedactionNoteMissing,
    /// A rendered compare viewer offers no actions.
    ViewerActionsMissing,
    /// A rendered compare viewer does not offer an open-raw action.
    OpenRawActionMissing,
    /// A rendered compare viewer does not offer an export action.
    ExportActionMissing,
    /// The viewers do not cover the trusted, untrusted, and raw-fallback classes.
    RenderTrustClassCoverageMissing,
    /// No media-metadata rails are present.
    MediaMetadataRailsMissing,
    /// A media-metadata rail is incomplete.
    MediaMetadataRailIncomplete,
    /// A media-metadata rail carries the wrong frozen component class.
    MediaMetadataRailWrongComponentClass,
    /// A media-metadata rail does not name its format.
    MediaFormatMissing,
    /// A media-metadata rail does not carry a size / duration / dimension measure.
    MediaMeasureMissing,
    /// A media-metadata rail does not carry a hidden-content note when required.
    HiddenContentNoteMissing,
    /// A media-metadata rail does not carry local / team / support share guidance.
    MediaShareGuidanceMissing,
    /// A media-metadata rail would share embedded sensitive content unsanitized.
    UnsanitizedHiddenContentShareable,
    /// The rails do not cover the present, absent, and unknown hidden-content states.
    HiddenContentStateCoverageMissing,
    /// No redaction-or-trust badge sets are present.
    RedactionOrTrustBadgeSetsMissing,
    /// A redaction-or-trust badge set is incomplete.
    RedactionOrTrustBadgeSetIncomplete,
    /// A redaction-or-trust badge set carries the wrong frozen component class.
    RedactionOrTrustBadgeSetWrongComponentClass,
    /// A redaction-or-trust badge set carries no badges.
    TrustBadgesMissing,
    /// A redacted badge set does not carry a redaction note.
    BadgeRedactionNoteMissing,
    /// An untrusted badge set does not carry an untrusted note.
    UntrustedBadgeNoteMissing,
    /// A redaction-or-trust badge set does not carry share guidance.
    BadgeShareGuidanceMissing,
    /// A redaction-or-trust badge set does not preserve posture on share/export.
    ExportPostureNotPreserved,
    /// The badge sets do not cover the not-redacted, partial, and full states.
    RedactionStateCoverageMissing,
    /// A viewer or rail has no accompanying redaction-or-trust badge set.
    TrustBadgeSetMissing,
    /// A viewer, rail, or badge set does not carry a raw-context jump action.
    RawContextActionMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl MediaCompareControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RenderedCompareViewersMissing => "rendered_compare_viewers_missing",
            Self::RenderedCompareViewerIncomplete => "rendered_compare_viewer_incomplete",
            Self::RenderedCompareViewerWrongComponentClass => {
                "rendered_compare_viewer_wrong_component_class"
            }
            Self::RenderedArtifactClassMissing => "rendered_artifact_class_missing",
            Self::ScaleOrDimensionMetadataMissing => "scale_or_dimension_metadata_missing",
            Self::AltTextFallbackMissing => "alt_text_fallback_missing",
            Self::SandboxNoteMissing => "sandbox_note_missing",
            Self::UntrustedRenderNoteMissing => "untrusted_render_note_missing",
            Self::RawFallbackLabelMissing => "raw_fallback_label_missing",
            Self::RenderRedactionNoteMissing => "render_redaction_note_missing",
            Self::ViewerActionsMissing => "viewer_actions_missing",
            Self::OpenRawActionMissing => "open_raw_action_missing",
            Self::ExportActionMissing => "export_action_missing",
            Self::RenderTrustClassCoverageMissing => "render_trust_class_coverage_missing",
            Self::MediaMetadataRailsMissing => "media_metadata_rails_missing",
            Self::MediaMetadataRailIncomplete => "media_metadata_rail_incomplete",
            Self::MediaMetadataRailWrongComponentClass => {
                "media_metadata_rail_wrong_component_class"
            }
            Self::MediaFormatMissing => "media_format_missing",
            Self::MediaMeasureMissing => "media_measure_missing",
            Self::HiddenContentNoteMissing => "hidden_content_note_missing",
            Self::MediaShareGuidanceMissing => "media_share_guidance_missing",
            Self::UnsanitizedHiddenContentShareable => "unsanitized_hidden_content_shareable",
            Self::HiddenContentStateCoverageMissing => "hidden_content_state_coverage_missing",
            Self::RedactionOrTrustBadgeSetsMissing => "redaction_or_trust_badge_sets_missing",
            Self::RedactionOrTrustBadgeSetIncomplete => "redaction_or_trust_badge_set_incomplete",
            Self::RedactionOrTrustBadgeSetWrongComponentClass => {
                "redaction_or_trust_badge_set_wrong_component_class"
            }
            Self::TrustBadgesMissing => "trust_badges_missing",
            Self::BadgeRedactionNoteMissing => "badge_redaction_note_missing",
            Self::UntrustedBadgeNoteMissing => "untrusted_badge_note_missing",
            Self::BadgeShareGuidanceMissing => "badge_share_guidance_missing",
            Self::ExportPostureNotPreserved => "export_posture_not_preserved",
            Self::RedactionStateCoverageMissing => "redaction_state_coverage_missing",
            Self::TrustBadgeSetMissing => "trust_badge_set_missing",
            Self::RawContextActionMissing => "raw_context_action_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable media/compare-controls export.
pub fn current_media_compare_controls_export(
) -> Result<MediaCompareControlsPacket, MediaCompareControlsArtifactError> {
    let packet: MediaCompareControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-rendered-compare-media-trust-controls-proof/support_export.json"
    )))
    .map_err(MediaCompareControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(MediaCompareControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &MediaCompareControlsPacket,
    violations: &mut Vec<MediaCompareControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        MEDIA_COMPARE_CONTROLS_SCHEMA_REF,
        MEDIA_COMPARE_CONTROLS_DOC_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_RENDERED_VIEWER_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_MEDIA_RAIL_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_REDACTION_BADGE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(MediaCompareControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rendered_compare_viewers(
    packet: &MediaCompareControlsPacket,
    violations: &mut Vec<MediaCompareControlsViolation>,
) {
    if packet.rendered_compare_viewers.is_empty() {
        violations.push(MediaCompareControlsViolation::RenderedCompareViewersMissing);
        return;
    }

    let mut classes: BTreeSet<RenderTrustClass> = BTreeSet::new();

    for viewer in &packet.rendered_compare_viewers {
        classes.insert(viewer.trust_class);

        if viewer.viewer_id.trim().is_empty()
            || viewer.artifact_ref.trim().is_empty()
            || viewer.fields_shown.is_empty()
            || viewer.source_contract_refs.is_empty()
        {
            violations.push(MediaCompareControlsViolation::RenderedCompareViewerIncomplete);
        }
        if viewer.component != M5ArtifactComponent::RenderedCompareViewer {
            violations
                .push(MediaCompareControlsViolation::RenderedCompareViewerWrongComponentClass);
        }
        if viewer.artifact_class_label.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::RenderedArtifactClassMissing);
        }
        if viewer.scale_or_dimension_metadata.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::ScaleOrDimensionMetadataMissing);
        }
        if viewer.alt_text_fallback.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::AltTextFallbackMissing);
        }
        if viewer.raw_context_action.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::RawContextActionMissing);
        }
        if viewer.available_actions.is_empty() {
            violations.push(MediaCompareControlsViolation::ViewerActionsMissing);
        } else {
            if !viewer.offers(RenderedViewerAction::OpenRaw) {
                violations.push(MediaCompareControlsViolation::OpenRawActionMissing);
            }
            if !viewer.offers(RenderedViewerAction::Export) {
                violations.push(MediaCompareControlsViolation::ExportActionMissing);
            }
        }

        let disclosure = viewer.disclosure();
        if disclosure.needs_sandbox_note && viewer.sandbox_note.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::SandboxNoteMissing);
        }
        if disclosure.needs_untrusted_render_note && viewer.untrusted_render_note.trim().is_empty()
        {
            violations.push(MediaCompareControlsViolation::UntrustedRenderNoteMissing);
        }
        if disclosure.needs_raw_fallback_label && viewer.raw_fallback_label.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::RawFallbackLabelMissing);
        }
        if disclosure.needs_redaction_note && viewer.redaction_note.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::RenderRedactionNoteMissing);
        }
    }

    for required in [
        RenderTrustClass::SandboxedTrusted,
        RenderTrustClass::SandboxedUntrusted,
        RenderTrustClass::RawTextFallback,
    ] {
        if !classes.contains(&required) {
            violations.push(MediaCompareControlsViolation::RenderTrustClassCoverageMissing);
            break;
        }
    }
}

fn validate_media_metadata_rails(
    packet: &MediaCompareControlsPacket,
    violations: &mut Vec<MediaCompareControlsViolation>,
) {
    if packet.media_metadata_rails.is_empty() {
        violations.push(MediaCompareControlsViolation::MediaMetadataRailsMissing);
        return;
    }

    let mut states: BTreeSet<HiddenContentState> = BTreeSet::new();

    for rail in &packet.media_metadata_rails {
        states.insert(rail.hidden_content_state);

        if rail.rail_id.trim().is_empty()
            || rail.artifact_ref.trim().is_empty()
            || rail.fields_shown.is_empty()
            || rail.source_contract_refs.is_empty()
        {
            violations.push(MediaCompareControlsViolation::MediaMetadataRailIncomplete);
        }
        if rail.component != M5ArtifactComponent::MediaMetadataRail {
            violations.push(MediaCompareControlsViolation::MediaMetadataRailWrongComponentClass);
        }
        if rail.format_label.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::MediaFormatMissing);
        }
        if rail.measure_value.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::MediaMeasureMissing);
        }
        if rail.share_guidance.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::MediaShareGuidanceMissing);
        }
        if rail.raw_context_action.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::RawContextActionMissing);
        }

        let disclosure = rail.disclosure();
        if disclosure.needs_hidden_content_note && rail.hidden_content_note.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::HiddenContentNoteMissing);
        }
        if rail.shares_unsanitized_hidden_content() {
            violations.push(MediaCompareControlsViolation::UnsanitizedHiddenContentShareable);
        }
    }

    for required in [
        HiddenContentState::NoEmbeddedSensitiveContent,
        HiddenContentState::EmbeddedSensitiveContentPresent,
        HiddenContentState::EmbeddedContentScanUnknown,
    ] {
        if !states.contains(&required) {
            violations.push(MediaCompareControlsViolation::HiddenContentStateCoverageMissing);
            break;
        }
    }
}

fn validate_redaction_trust_badge_sets(
    packet: &MediaCompareControlsPacket,
    violations: &mut Vec<MediaCompareControlsViolation>,
) {
    if packet.redaction_trust_badge_sets.is_empty() {
        violations.push(MediaCompareControlsViolation::RedactionOrTrustBadgeSetsMissing);
        return;
    }

    let mut states: BTreeSet<RedactionState> = BTreeSet::new();

    for set in &packet.redaction_trust_badge_sets {
        states.insert(set.redaction_state);

        if set.badge_set_id.trim().is_empty()
            || set.artifact_ref.trim().is_empty()
            || set.fields_shown.is_empty()
            || set.source_contract_refs.is_empty()
        {
            violations.push(MediaCompareControlsViolation::RedactionOrTrustBadgeSetIncomplete);
        }
        if set.component != M5ArtifactComponent::RedactionOrTrustBadgeSet {
            violations
                .push(MediaCompareControlsViolation::RedactionOrTrustBadgeSetWrongComponentClass);
        }
        if set.available_badges.is_empty() {
            violations.push(MediaCompareControlsViolation::TrustBadgesMissing);
        }
        if set.share_guidance.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::BadgeShareGuidanceMissing);
        }
        if set.raw_context_action.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::RawContextActionMissing);
        }
        // Share and export flows must preserve the redaction / trust posture rather
        // than flattening rendered review into an ambiguous attachment.
        if !set.export_posture_preserved {
            violations.push(MediaCompareControlsViolation::ExportPostureNotPreserved);
        }

        let disclosure = set.disclosure();
        if disclosure.needs_redaction_note && set.redaction_note.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::BadgeRedactionNoteMissing);
        }
        if disclosure.needs_untrusted_note && set.untrusted_note.trim().is_empty() {
            violations.push(MediaCompareControlsViolation::UntrustedBadgeNoteMissing);
        }
    }

    for required in [
        RedactionState::NotRedacted,
        RedactionState::PartiallyRedacted,
        RedactionState::FullyRedacted,
    ] {
        if !states.contains(&required) {
            violations.push(MediaCompareControlsViolation::RedactionStateCoverageMissing);
            break;
        }
    }
}

fn validate_pairing(
    packet: &MediaCompareControlsPacket,
    violations: &mut Vec<MediaCompareControlsViolation>,
) {
    let badge_refs: BTreeSet<&str> = packet
        .redaction_trust_badge_sets
        .iter()
        .map(|set| set.artifact_ref.as_str())
        .collect();
    // Every rendered compare viewer and every media-metadata rail must be
    // accompanied by a redaction-or-trust badge set for the same artifact, so the
    // redaction and trust posture is always visible where media is reviewed.
    let mut missing = false;
    for viewer in &packet.rendered_compare_viewers {
        if !badge_refs.contains(viewer.artifact_ref.as_str()) {
            missing = true;
            break;
        }
    }
    if !missing {
        for rail in &packet.media_metadata_rails {
            if !badge_refs.contains(rail.artifact_ref.as_str()) {
                missing = true;
                break;
            }
        }
    }
    if missing {
        violations.push(MediaCompareControlsViolation::TrustBadgeSetMissing);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
