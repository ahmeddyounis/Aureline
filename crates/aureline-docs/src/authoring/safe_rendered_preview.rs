//! Safe rendered-preview capability-boundary truth packet.
//!
//! A [`RenderedPreviewBoundary`] is the canonical, export-safe record for the
//! capability boundaries a single docs rendered preview enforces. Where the
//! [`crate::authoring::markdown_workspace`] packet records *which* mode a
//! workspace is in, this packet records *what a rendered preview is allowed to
//! do* in that mode: it makes the per-capability request state, render posture,
//! honest-degradation fallback, raw/source escape, external-open path, and
//! no-authority-expansion guarantee inspectable for diagrams, front matter,
//! math, callouts, remote assets, and extension/custom components.
//!
//! The packet binds:
//!
//! - the owning workspace ref, the active [`DocsPreviewMode`], and the
//!   [`PreviewSurfaceOwner`] so a rendered preview is never an unlabeled active
//!   surface and never impersonates the native shell or a browser-owned control
//!   plane;
//! - one [`PreviewCapabilityBoundary`] per [`PreviewCapabilityKind`], each
//!   carrying its [`CapabilityRequestState`], [`PreviewRenderPosture`], visible
//!   boundary cue, raw/source escape availability, [`CapabilityAuthority`]
//!   posture, and any external-open path;
//! - the rendered-preview sanitization posture, the always-available recovery
//!   back to raw source, the open-source action, and the
//!   [`AccessibilityParity`] posture that degrades honestly when theme, zoom,
//!   density, or motion parity cannot be preserved;
//! - the [`DocsSourceVersionBadge`], the [`DocsMirrorOfflinePosture`], the
//!   [`DocsPublishBoundaryState`], and the disclosures that the rendered view is
//!   never canonical source and never widens authority.
//!
//! [`RenderedPreviewBoundary::validate`] enforces the track invariant: richer
//! rendered preview is never an unlabeled active surface, every capability is a
//! requested, cued, escapable, non-privileged path, a raw/source escape is
//! always available, and capability, owner, and freshness truth survives
//! support and release evidence.
//!
//! The boundary schema is
//! [`schemas/docs/docs-rendered-preview-capabilities.schema.json`](../../../../schemas/docs/docs-rendered-preview-capabilities.schema.json).
//! The contract/help doc is
//! [`docs/help/rendered-preview-boundaries.md`](../../../../docs/help/rendered-preview-boundaries.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/rendered-preview-boundaries/`](../../../../fixtures/docs/m5/rendered-preview-boundaries/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::authoring::markdown_workspace::WorkspaceRecoveryCommand;
use crate::citations::{DocsFreshnessClass, VersionMatchState};
use crate::evidence_model::{DocsExternalOpenState, DocsMirrorOfflinePosture};
use crate::maintenance::{
    DocsArtifactKind, DocsMaintenanceAction, DocsPreviewMode, DocsPreviewSanitizationState,
    DocsPublishBoundaryState, DocsSourceVersionBadge,
};

/// Stable record-kind tag carried by [`RenderedPreviewBoundary`].
pub const RENDERED_PREVIEW_BOUNDARY_RECORD_KIND: &str = "rendered_preview_boundary_record";

/// Schema version for rendered-preview capability-boundary records.
pub const RENDERED_PREVIEW_BOUNDARY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const RENDERED_PREVIEW_BOUNDARY_SCHEMA_REF: &str =
    "schemas/docs/docs-rendered-preview-capabilities.schema.json";

/// Repo-relative path of the contract/help doc.
pub const RENDERED_PREVIEW_BOUNDARY_DOC_REF: &str = "docs/help/rendered-preview-boundaries.md";

/// Repo-relative path of the protected fixture directory.
pub const RENDERED_PREVIEW_BOUNDARY_FIXTURE_DIR: &str =
    "fixtures/docs/m5/rendered-preview-boundaries";

/// Repo-relative path of the checked-in support-export artifact.
pub const RENDERED_PREVIEW_BOUNDARY_ARTIFACT_REF: &str =
    "artifacts/docs/m5/rendered-preview-boundary-proof/support_export.json";

/// Repo-relative path of the checked-in Markdown summary.
pub const RENDERED_PREVIEW_BOUNDARY_SUMMARY_REF: &str =
    "artifacts/docs/m5/rendered-preview-boundary-proof.md";

/// Stable command id for the always-available recovery back to raw source.
pub const RECOVER_SOURCE_COMMAND_ID: &str = "docs.preview.recover_source";

/// Stable action ref for the open-source / switch-to-source escape.
pub const OPEN_SOURCE_ACTION_REF: &str = "docs.preview.open_source";

/// A rendered-preview capability that carries its own safety boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewCapabilityKind {
    /// Fenced diagram blocks rendered by a diagram engine.
    Diagrams,
    /// YAML/TOML front matter processed into a metadata view.
    FrontMatter,
    /// Inline or block math rendering.
    Math,
    /// Admonition / callout blocks beyond the CommonMark baseline.
    Callouts,
    /// Remote images, embeds, or other network-fetched assets.
    RemoteAssets,
    /// Extension or custom-component rendering beyond the baseline.
    CustomComponents,
}

impl PreviewCapabilityKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Diagrams => "diagrams",
            Self::FrontMatter => "front_matter",
            Self::Math => "math",
            Self::Callouts => "callouts",
            Self::RemoteAssets => "remote_assets",
            Self::CustomComponents => "custom_components",
        }
    }

    /// The complete set of capabilities a boundary packet must cover.
    pub const fn all() -> [PreviewCapabilityKind; 6] {
        [
            Self::Diagrams,
            Self::FrontMatter,
            Self::Math,
            Self::Callouts,
            Self::RemoteAssets,
            Self::CustomComponents,
        ]
    }
}

/// Explicit request state for one rendered-preview capability.
///
/// A capability never renders without an explicit grant: the strongest
/// request state that permits active rendering is [`CapabilityRequestState::GrantedSandboxed`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityRequestState {
    /// No content of this kind is present, or the capability is simply off.
    NotRequested,
    /// Content of this kind is present and an explicit request awaits consent.
    RequestedAwaitingConsent,
    /// The capability was explicitly granted and renders sandboxed.
    GrantedSandboxed,
    /// The request was denied by policy.
    DeniedByPolicy,
    /// The capability does not apply in the current mode (for example source mode).
    NotApplicable,
}

impl CapabilityRequestState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::RequestedAwaitingConsent => "requested_awaiting_consent",
            Self::GrantedSandboxed => "granted_sandboxed",
            Self::DeniedByPolicy => "denied_by_policy",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Returns true when the request explicitly grants a sandboxed render.
    pub const fn is_granted(self) -> bool {
        matches!(self, Self::GrantedSandboxed)
    }
}

/// What a rendered preview is actually doing for one capability.
///
/// There is deliberately no "privileged" or "executes" posture: rendered
/// preview capabilities are never a privileged execution path. The strongest
/// active posture is [`PreviewRenderPosture::SandboxedActive`], and unsafe
/// content degrades honestly to [`PreviewRenderPosture::StaticOnly`],
/// [`PreviewRenderPosture::RawFallback`], or [`PreviewRenderPosture::Blocked`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewRenderPosture {
    /// Capability is disabled; content of this kind renders as inert source text.
    Disabled,
    /// Content renders only under an explicit, sandboxed grant (never privileged).
    SandboxedActive,
    /// Content degrades honestly to a static, non-interactive rendering.
    StaticOnly,
    /// Content degrades honestly to raw source text.
    RawFallback,
    /// Content of this kind was present and was blocked from rendering, with a cue.
    Blocked,
    /// Capability does not apply in the current mode.
    NotApplicable,
}

impl PreviewRenderPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::SandboxedActive => "sandboxed_active",
            Self::StaticOnly => "static_only",
            Self::RawFallback => "raw_fallback",
            Self::Blocked => "blocked",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Returns true when the posture actively runs a render engine (always sandboxed).
    pub const fn is_active(self) -> bool {
        matches!(self, Self::SandboxedActive)
    }

    /// Returns true when content was present but could not be fully rendered safely.
    pub const fn is_degraded(self) -> bool {
        matches!(self, Self::StaticOnly | Self::RawFallback | Self::Blocked)
    }
}

/// Authority posture for one rendered-preview capability.
///
/// Only [`CapabilityAuthority::NoAuthorityExpansion`] is valid; the impersonation
/// variants are modeled so [`RenderedPreviewBoundary::validate`] can reject any
/// capability that would impersonate a native approval or claim a browser-owned
/// control plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityAuthority {
    /// The capability never expands authority beyond a sandboxed render.
    NoAuthorityExpansion,
    /// INVALID: the capability impersonates a native approval surface.
    ImpersonatesNativeApproval,
    /// INVALID: the capability claims a browser-owned control plane.
    ClaimsBrowserControlPlane,
}

impl CapabilityAuthority {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAuthorityExpansion => "no_authority_expansion",
            Self::ImpersonatesNativeApproval => "impersonates_native_approval",
            Self::ClaimsBrowserControlPlane => "claims_browser_control_plane",
        }
    }

    /// Returns true only for the one posture that does not widen authority.
    pub const fn is_safe(self) -> bool {
        matches!(self, Self::NoAuthorityExpansion)
    }
}

/// Owner / origin of the rendered-preview surface.
///
/// A rendered preview must always disclose a legitimate owner. The impersonation
/// variants are modeled so validation can reject a preview that masquerades as
/// the native shell or a browser-owned control plane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewSurfaceOwner {
    /// Aureline's own sandboxed docs preview renderer.
    DocsPreviewSandbox,
    /// A scoped, disclosed handoff to the browser companion renders the preview.
    DisclosedBrowserCompanion,
    /// INVALID: the preview impersonates the native shell or approval surface.
    ImpersonatedNativeShell,
    /// INVALID: the preview claims to be a browser-owned control plane.
    ImpersonatedBrowserControlPlane,
}

impl PreviewSurfaceOwner {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsPreviewSandbox => "docs_preview_sandbox",
            Self::DisclosedBrowserCompanion => "disclosed_browser_companion",
            Self::ImpersonatedNativeShell => "impersonated_native_shell",
            Self::ImpersonatedBrowserControlPlane => "impersonated_browser_control_plane",
        }
    }

    /// Returns true for a disclosed, non-impersonating owner.
    pub const fn is_legitimate(self) -> bool {
        matches!(
            self,
            Self::DocsPreviewSandbox | Self::DisclosedBrowserCompanion
        )
    }
}

/// One rendered-preview capability boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewCapabilityBoundary {
    /// Capability this boundary governs.
    pub capability_kind: PreviewCapabilityKind,
    /// Explicit request state for the capability.
    pub request_state: CapabilityRequestState,
    /// What the rendered preview is actually doing for the capability.
    pub render_posture: PreviewRenderPosture,
    /// Visible boundary cue shown beside the capability.
    pub boundary_cue: String,
    /// Whether a raw/source escape is available for this capability.
    pub escape_to_source_available: bool,
    /// No-authority-expansion posture for the capability.
    pub authority_posture: CapabilityAuthority,
    /// External-open path posture (for example for remote assets).
    pub external_open_state: DocsExternalOpenState,
    /// Open-externally action, present only when external opening is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_externally_action: Option<DocsMaintenanceAction>,
    /// Stable consent ref, present only when the capability was granted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_ref: Option<String>,
    /// Disclosure note explaining a blocked, denied, or degraded posture.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl PreviewCapabilityBoundary {
    /// Returns true when the boundary is fully inert (source-mode posture).
    fn is_not_applicable(&self) -> bool {
        self.request_state == CapabilityRequestState::NotApplicable
            && self.render_posture == PreviewRenderPosture::NotApplicable
            && self.external_open_state == DocsExternalOpenState::NotRequired
            && self.open_externally_action.is_none()
            && self.consent_ref.is_none()
    }

    /// Per-boundary validation, pushing violations onto the shared list.
    fn validate(&self, violations: &mut Vec<PreviewBoundaryViolation>) {
        if self.boundary_cue.trim().is_empty() {
            violations.push(PreviewBoundaryViolation::BoundaryCueMissing);
        }
        if !self.escape_to_source_available {
            violations.push(PreviewBoundaryViolation::EscapeRouteMissing);
        }
        if !self.authority_posture.is_safe() {
            violations.push(PreviewBoundaryViolation::CapabilityExpandsAuthority);
        }

        // A sandboxed-active render requires an explicit grant and a consent ref;
        // a grant requires an active render. The two states stay in lock-step so a
        // capability can never render without a recorded, explicit grant.
        let active = self.render_posture.is_active();
        let granted = self.request_state.is_granted();
        let consent_present = self
            .consent_ref
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty());
        if active != granted || (active && !consent_present) {
            violations.push(PreviewBoundaryViolation::ActiveCapabilityWithoutGrant);
        }

        // A blocked or denied capability must disclose why it degraded.
        let degraded_needs_note = self.render_posture == PreviewRenderPosture::Blocked
            || self.request_state == CapabilityRequestState::DeniedByPolicy;
        if degraded_needs_note
            && self
                .note
                .as_deref()
                .map_or(true, |note| note.trim().is_empty())
        {
            violations.push(PreviewBoundaryViolation::DegradationNotDisclosed);
        }

        // The external-open action must match the disclosed external-open state.
        match (
            self.external_open_state == DocsExternalOpenState::Available,
            &self.open_externally_action,
        ) {
            (true, Some(action)) if action_is_well_formed(action) => {}
            (false, None) => {}
            _ => violations.push(PreviewBoundaryViolation::ExternalOpenActionMismatch),
        }
    }
}

/// Theme/zoom/density/motion/keyboard parity posture for the rendered preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityParity {
    /// Whether the rendered preview honors the active theme.
    pub theme_parity: bool,
    /// Whether the rendered preview honors the active zoom level.
    pub zoom_parity: bool,
    /// Whether the rendered preview honors the active density.
    pub density_parity: bool,
    /// Whether the rendered preview honors the reduced-motion preference.
    pub reduced_motion_parity: bool,
    /// Whether the rendered preview is fully keyboard reachable (always required).
    pub keyboard_parity: bool,
    /// Disclosure note required when any non-keyboard parity dimension is degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parity_note: Option<String>,
}

impl AccessibilityParity {
    /// Returns true when every theme/zoom/density/motion dimension is preserved.
    fn is_fully_preserved(&self) -> bool {
        self.theme_parity && self.zoom_parity && self.density_parity && self.reduced_motion_parity
    }
}

/// Constructor input for [`RenderedPreviewBoundary::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedPreviewBoundaryInput {
    /// Stable boundary id.
    pub boundary_id: String,
    /// Human-readable boundary label.
    pub boundary_label: String,
    /// Ref of the owning Markdown authoring workspace.
    pub workspace_ref: String,
    /// Artifact family being previewed.
    pub artifact_kind: DocsArtifactKind,
    /// Stable artifact ref (path or artifact id, never a raw body).
    pub artifact_ref: String,
    /// Active workspace mode driving the preview.
    pub active_mode: DocsPreviewMode,
    /// Disclosed owner / origin of the rendered-preview surface.
    pub surface_owner: PreviewSurfaceOwner,
    /// Disclosure of who owns and renders the preview surface.
    pub origin_disclosure: String,
    /// HTML sanitization posture for rendered content.
    pub sanitization_state: DocsPreviewSanitizationState,
    /// Disclosure note for the sanitization posture when required.
    pub sanitization_note: Option<String>,
    /// Per-capability boundaries. Must cover every [`PreviewCapabilityKind`].
    pub capability_boundaries: Vec<PreviewCapabilityBoundary>,
    /// Always-available recovery back to raw source.
    pub recover_to_source_command: WorkspaceRecoveryCommand,
    /// Open-source / switch-to-source action (always keyboard reachable).
    pub open_source_action: DocsMaintenanceAction,
    /// Theme/zoom/density/motion/keyboard parity posture.
    pub accessibility_parity: AccessibilityParity,
    /// Disclosure that the preview never widens authority.
    pub no_authority_expansion_note: String,
    /// Disclosure that a rendered view is not canonical source or proof.
    pub rendered_is_not_canonical_note: String,
    /// Docs source/version/freshness badge.
    pub source_version_badge: DocsSourceVersionBadge,
    /// Mirror, cache, or offline posture for the source.
    pub mirror_offline_state: DocsMirrorOfflinePosture,
    /// Local-only versus publish-boundary posture.
    pub publish_boundary_state: DocsPublishBoundaryState,
    /// Surface refs that render this boundary.
    pub surface_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe rendered-preview capability-boundary truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderedPreviewBoundary {
    /// Record kind; must equal [`RENDERED_PREVIEW_BOUNDARY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RENDERED_PREVIEW_BOUNDARY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable boundary id.
    pub boundary_id: String,
    /// Human-readable boundary label.
    pub boundary_label: String,
    /// Ref of the owning Markdown authoring workspace.
    pub workspace_ref: String,
    /// Artifact family being previewed.
    pub artifact_kind: DocsArtifactKind,
    /// Stable artifact ref (path or artifact id, never a raw body).
    pub artifact_ref: String,
    /// Active workspace mode driving the preview.
    pub active_mode: DocsPreviewMode,
    /// Disclosed owner / origin of the rendered-preview surface.
    pub surface_owner: PreviewSurfaceOwner,
    /// Disclosure of who owns and renders the preview surface.
    pub origin_disclosure: String,
    /// HTML sanitization posture for rendered content.
    pub sanitization_state: DocsPreviewSanitizationState,
    /// Disclosure note for the sanitization posture when required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sanitization_note: Option<String>,
    /// Per-capability boundaries.
    pub capability_boundaries: Vec<PreviewCapabilityBoundary>,
    /// Always-available recovery back to raw source.
    pub recover_to_source_command: WorkspaceRecoveryCommand,
    /// Open-source / switch-to-source action.
    pub open_source_action: DocsMaintenanceAction,
    /// Theme/zoom/density/motion/keyboard parity posture.
    pub accessibility_parity: AccessibilityParity,
    /// Disclosure that the preview never widens authority.
    pub no_authority_expansion_note: String,
    /// Disclosure that a rendered view is not canonical source or proof.
    pub rendered_is_not_canonical_note: String,
    /// Docs source/version/freshness badge.
    pub source_version_badge: DocsSourceVersionBadge,
    /// Mirror, cache, or offline posture for the source.
    pub mirror_offline_state: DocsMirrorOfflinePosture,
    /// Local-only versus publish-boundary posture.
    pub publish_boundary_state: DocsPublishBoundaryState,
    /// Surface refs that render this boundary.
    pub surface_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RenderedPreviewBoundary {
    /// Builds a rendered-preview boundary packet from constructor input.
    pub fn new(input: RenderedPreviewBoundaryInput) -> Self {
        Self {
            record_kind: RENDERED_PREVIEW_BOUNDARY_RECORD_KIND.to_owned(),
            schema_version: RENDERED_PREVIEW_BOUNDARY_SCHEMA_VERSION,
            boundary_id: input.boundary_id,
            boundary_label: input.boundary_label,
            workspace_ref: input.workspace_ref,
            artifact_kind: input.artifact_kind,
            artifact_ref: input.artifact_ref,
            active_mode: input.active_mode,
            surface_owner: input.surface_owner,
            origin_disclosure: input.origin_disclosure,
            sanitization_state: input.sanitization_state,
            sanitization_note: input.sanitization_note,
            capability_boundaries: input.capability_boundaries,
            recover_to_source_command: input.recover_to_source_command,
            open_source_action: input.open_source_action,
            accessibility_parity: input.accessibility_parity,
            no_authority_expansion_note: input.no_authority_expansion_note,
            rendered_is_not_canonical_note: input.rendered_is_not_canonical_note,
            source_version_badge: input.source_version_badge,
            mirror_offline_state: input.mirror_offline_state,
            publish_boundary_state: input.publish_boundary_state,
            surface_refs: input.surface_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Returns true when the active mode renders a preview (split or rendered).
    pub fn renders_preview(&self) -> bool {
        self.active_mode.renders_preview()
    }

    /// Returns the boundary for a capability kind, if present.
    pub fn boundary(&self, kind: PreviewCapabilityKind) -> Option<&PreviewCapabilityBoundary> {
        self.capability_boundaries
            .iter()
            .find(|boundary| boundary.capability_kind == kind)
    }

    /// Validates the rendered-preview boundary truth invariants.
    pub fn validate(&self) -> Vec<PreviewBoundaryViolation> {
        let mut violations = Vec::new();

        if self.record_kind != RENDERED_PREVIEW_BOUNDARY_RECORD_KIND {
            violations.push(PreviewBoundaryViolation::WrongRecordKind);
        }
        if self.schema_version != RENDERED_PREVIEW_BOUNDARY_SCHEMA_VERSION {
            violations.push(PreviewBoundaryViolation::WrongSchemaVersion);
        }
        if self.boundary_id.trim().is_empty()
            || self.boundary_label.trim().is_empty()
            || self.workspace_ref.trim().is_empty()
            || self.artifact_ref.trim().is_empty()
            || self.origin_disclosure.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.surface_refs.is_empty()
        {
            violations.push(PreviewBoundaryViolation::MissingIdentity);
        }

        validate_capability_coverage(self, &mut violations);
        for boundary in &self.capability_boundaries {
            boundary.validate(&mut violations);
        }
        validate_authority(self, &mut violations);
        validate_escapes(self, &mut violations);
        validate_mode_posture(self, &mut violations);
        validate_accessibility(self, &mut violations);
        validate_source_version_badge(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("rendered preview boundary serializes"),
        ) {
            violations.push(PreviewBoundaryViolation::RawBoundaryMaterialInExport);
        }

        violations.sort();
        violations.dedup();
        violations
    }

    /// Returns true when the boundary validates with no violations.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("rendered preview boundary serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Rendered Preview Boundary\n\n");
        out.push_str(&format!("- Boundary: `{}`\n", self.boundary_id));
        out.push_str(&format!("- Label: `{}`\n", self.boundary_label));
        out.push_str(&format!("- Workspace: `{}`\n", self.workspace_ref));
        out.push_str(&format!("- Artifact: `{}`\n", self.artifact_ref));
        out.push_str(&format!("- Active mode: `{}`\n", self.active_mode.as_str()));
        out.push_str(&format!(
            "- Surface owner: `{}`\n",
            self.surface_owner.as_str()
        ));
        out.push_str(&format!(
            "- Sanitization: `{}`\n",
            self.sanitization_state.as_str()
        ));
        out.push_str(&format!(
            "- Source: `{}` / freshness `{}` / version `{}`\n",
            self.source_version_badge.source_class_token,
            self.source_version_badge.freshness_class.as_str(),
            self.source_version_badge.version_match_state.as_str()
        ));
        out.push_str(&format!(
            "- Mirror/offline: `{}`\n",
            self.mirror_offline_state.as_str()
        ));
        out.push_str("\n## Capability boundaries\n\n");
        out.push_str("| Capability | Request | Render | Authority | External |\n");
        out.push_str("| --- | --- | --- | --- | --- |\n");
        for boundary in &self.capability_boundaries {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                boundary.capability_kind.as_str(),
                boundary.request_state.as_str(),
                boundary.render_posture.as_str(),
                boundary.authority_posture.as_str(),
                boundary.external_open_state.as_str(),
            ));
        }
        out.push_str("\n## Escapes\n\n");
        out.push_str(&format!(
            "- Recover to source: `{}` ({})\n",
            self.recover_to_source_command.command_id, self.recover_to_source_command.key_binding
        ));
        out.push_str(&format!(
            "- Open source: `{}`\n",
            self.open_source_action.action_ref
        ));
        out
    }
}

/// Errors emitted when reading the checked-in rendered-preview boundary export.
#[derive(Debug)]
pub enum RenderedPreviewBoundaryArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PreviewBoundaryViolation>),
}

impl fmt::Display for RenderedPreviewBoundaryArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "rendered preview boundary export parse failed: {error}"
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
                    "rendered preview boundary export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RenderedPreviewBoundaryArtifactError {}

/// Validation failures emitted by [`RenderedPreviewBoundary::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PreviewBoundaryViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Capability boundaries do not cover every kind exactly once.
    CapabilityCoverageIncomplete,
    /// A capability's boundary cue is missing.
    BoundaryCueMissing,
    /// A capability does not offer a raw/source escape.
    EscapeRouteMissing,
    /// A capability would expand authority beyond a sandboxed render.
    CapabilityExpandsAuthority,
    /// The no-authority-expansion disclosure note is missing.
    NoAuthorityExpansionNoteMissing,
    /// The rendered-preview surface owner impersonates an authority surface.
    SurfaceOwnerImpersonatesAuthority,
    /// A capability renders without an explicit grant and consent.
    ActiveCapabilityWithoutGrant,
    /// A blocked or denied capability does not disclose why it degraded.
    DegradationNotDisclosed,
    /// The external-open action does not match the declared external-open state.
    ExternalOpenActionMismatch,
    /// The recovery command does not return to raw source or is malformed.
    RecoveryCommandInvalid,
    /// The open-source action is missing or not keyboard reachable.
    OpenSourceActionInvalid,
    /// Source mode renders content (a non-inert capability or sanitization posture).
    SourceModeRendersContent,
    /// A rendering mode declares a non-concrete preview-safety posture.
    UnsafePreviewDefault,
    /// A rendering mode does not disclose that rendered output is not canonical source.
    RenderedNotDisclosedAsNonCanonical,
    /// Allowed raw HTML is missing its disclosure note.
    SanitizationNoteMissing,
    /// The rendered preview is not keyboard reachable.
    AccessibilityKeyboardParityMissing,
    /// A degraded parity dimension is not disclosed.
    AccessibilityParityDegradedWithoutNote,
    /// The source/version/freshness badge is incomplete.
    SourceVersionBadgeIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PreviewBoundaryViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::CapabilityCoverageIncomplete => "capability_coverage_incomplete",
            Self::BoundaryCueMissing => "boundary_cue_missing",
            Self::EscapeRouteMissing => "escape_route_missing",
            Self::CapabilityExpandsAuthority => "capability_expands_authority",
            Self::NoAuthorityExpansionNoteMissing => "no_authority_expansion_note_missing",
            Self::SurfaceOwnerImpersonatesAuthority => "surface_owner_impersonates_authority",
            Self::ActiveCapabilityWithoutGrant => "active_capability_without_grant",
            Self::DegradationNotDisclosed => "degradation_not_disclosed",
            Self::ExternalOpenActionMismatch => "external_open_action_mismatch",
            Self::RecoveryCommandInvalid => "recovery_command_invalid",
            Self::OpenSourceActionInvalid => "open_source_action_invalid",
            Self::SourceModeRendersContent => "source_mode_renders_content",
            Self::UnsafePreviewDefault => "unsafe_preview_default",
            Self::RenderedNotDisclosedAsNonCanonical => "rendered_not_disclosed_as_non_canonical",
            Self::SanitizationNoteMissing => "sanitization_note_missing",
            Self::AccessibilityKeyboardParityMissing => "accessibility_keyboard_parity_missing",
            Self::AccessibilityParityDegradedWithoutNote => {
                "accessibility_parity_degraded_without_note"
            }
            Self::SourceVersionBadgeIncomplete => "source_version_badge_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable rendered-preview boundary export.
pub fn current_stable_rendered_preview_boundary_export(
) -> Result<RenderedPreviewBoundary, RenderedPreviewBoundaryArtifactError> {
    let packet: RenderedPreviewBoundary = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/rendered-preview-boundary-proof/support_export.json"
    )))
    .map_err(RenderedPreviewBoundaryArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RenderedPreviewBoundaryArtifactError::Validation(violations))
    }
}

/// Returns the canonical seeded Split-mode boundary used as the support export.
pub fn seeded_stable_rendered_preview_boundary() -> RenderedPreviewBoundary {
    RenderedPreviewBoundary::new(seeded_stable_rendered_preview_boundary_input())
}

/// Returns the constructor input for [`seeded_stable_rendered_preview_boundary`].
pub fn seeded_stable_rendered_preview_boundary_input() -> RenderedPreviewBoundaryInput {
    RenderedPreviewBoundaryInput {
        boundary_id: "docs-preview-boundary:readme:split:0001".to_owned(),
        boundary_label: "README rendered-preview boundary".to_owned(),
        workspace_ref: "docs-workspace:readme:split:0001".to_owned(),
        artifact_kind: DocsArtifactKind::Readme,
        artifact_ref: "README.md".to_owned(),
        active_mode: DocsPreviewMode::Split,
        surface_owner: PreviewSurfaceOwner::DocsPreviewSandbox,
        origin_disclosure:
            "Rendered by Aureline's sandboxed docs preview. It is not the native shell and not a \
             browser-owned control plane."
                .to_owned(),
        sanitization_state: DocsPreviewSanitizationState::SanitizedSafe,
        sanitization_note: None,
        capability_boundaries: seeded_capability_boundaries(),
        recover_to_source_command: seeded_recovery_command(),
        open_source_action: DocsMaintenanceAction::new(OPEN_SOURCE_ACTION_REF, "Open source"),
        accessibility_parity: AccessibilityParity {
            theme_parity: true,
            zoom_parity: true,
            density_parity: true,
            reduced_motion_parity: true,
            keyboard_parity: true,
            parity_note: None,
        },
        no_authority_expansion_note:
            "The rendered preview never approves actions, never grants permissions, and never \
             impersonates the native shell or browser control plane. Capabilities render \
             sandboxed and inert."
                .to_owned(),
        rendered_is_not_canonical_note:
            "Rendered preview is a safe, sanitized view of the source. It is not canonical source \
             or proof; the raw Markdown remains the source of truth."
                .to_owned(),
        source_version_badge: DocsSourceVersionBadge {
            source_class_token: "project_docs".to_owned(),
            source_pack_ref: "project:readme".to_owned(),
            source_revision_ref: "rev:7131d437".to_owned(),
            version_or_revision_ref: "0.1.0-dev".to_owned(),
            source_build_at: "2026-06-18T00:00:00Z".to_owned(),
            running_build_identity_ref: "build:0.1.0-dev".to_owned(),
            freshness_class: DocsFreshnessClass::WarmCached,
            version_match_state: VersionMatchState::ExactBuildMatch,
        },
        mirror_offline_state: DocsMirrorOfflinePosture::LocalProjectPack,
        publish_boundary_state: DocsPublishBoundaryState::LocalOnly,
        surface_refs: vec!["authoring_workspace".to_owned(), "preview_pane".to_owned()],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-18T00:00:00Z".to_owned(),
    }
}

/// Returns the canonical capability boundaries for the seeded Split-mode preview.
pub fn seeded_capability_boundaries() -> Vec<PreviewCapabilityBoundary> {
    vec![
        // Diagrams render under an explicit sandboxed grant.
        PreviewCapabilityBoundary {
            capability_kind: PreviewCapabilityKind::Diagrams,
            request_state: CapabilityRequestState::GrantedSandboxed,
            render_posture: PreviewRenderPosture::SandboxedActive,
            boundary_cue: "Diagrams render sandboxed".to_owned(),
            escape_to_source_available: true,
            authority_posture: CapabilityAuthority::NoAuthorityExpansion,
            external_open_state: DocsExternalOpenState::NotRequired,
            open_externally_action: None,
            consent_ref: Some("consent:diagrams:sandboxed".to_owned()),
            note: None,
        },
        // Front matter is parsed and shown as a static, non-interactive table.
        PreviewCapabilityBoundary {
            capability_kind: PreviewCapabilityKind::FrontMatter,
            request_state: CapabilityRequestState::RequestedAwaitingConsent,
            render_posture: PreviewRenderPosture::StaticOnly,
            boundary_cue: "Front matter shown as static metadata".to_owned(),
            escape_to_source_available: true,
            authority_posture: CapabilityAuthority::NoAuthorityExpansion,
            external_open_state: DocsExternalOpenState::NotRequired,
            open_externally_action: None,
            consent_ref: None,
            note: None,
        },
        // Math renders under an explicit sandboxed grant.
        PreviewCapabilityBoundary {
            capability_kind: PreviewCapabilityKind::Math,
            request_state: CapabilityRequestState::GrantedSandboxed,
            render_posture: PreviewRenderPosture::SandboxedActive,
            boundary_cue: "Math renders sandboxed".to_owned(),
            escape_to_source_available: true,
            authority_posture: CapabilityAuthority::NoAuthorityExpansion,
            external_open_state: DocsExternalOpenState::NotRequired,
            open_externally_action: None,
            consent_ref: Some("consent:math:sandboxed".to_owned()),
            note: None,
        },
        // Callouts render as static admonition blocks.
        PreviewCapabilityBoundary {
            capability_kind: PreviewCapabilityKind::Callouts,
            request_state: CapabilityRequestState::RequestedAwaitingConsent,
            render_posture: PreviewRenderPosture::StaticOnly,
            boundary_cue: "Callouts render as static admonitions".to_owned(),
            escape_to_source_available: true,
            authority_posture: CapabilityAuthority::NoAuthorityExpansion,
            external_open_state: DocsExternalOpenState::NotRequired,
            open_externally_action: None,
            consent_ref: None,
            note: None,
        },
        // Remote assets stay blocked; an open-externally path is offered instead.
        PreviewCapabilityBoundary {
            capability_kind: PreviewCapabilityKind::RemoteAssets,
            request_state: CapabilityRequestState::RequestedAwaitingConsent,
            render_posture: PreviewRenderPosture::Blocked,
            boundary_cue: "Remote assets blocked — open externally".to_owned(),
            escape_to_source_available: true,
            authority_posture: CapabilityAuthority::NoAuthorityExpansion,
            external_open_state: DocsExternalOpenState::Available,
            open_externally_action: Some(DocsMaintenanceAction::new(
                "docs.preview.open_remote_asset_externally",
                "Open externally",
            )),
            consent_ref: None,
            note: Some(
                "Remote assets are not fetched inside the preview. Open them externally to keep \
                 the preview offline-safe."
                    .to_owned(),
            ),
        },
        // Custom components are disabled and render inert as source text.
        PreviewCapabilityBoundary {
            capability_kind: PreviewCapabilityKind::CustomComponents,
            request_state: CapabilityRequestState::NotRequested,
            render_posture: PreviewRenderPosture::Disabled,
            boundary_cue: "Custom components disabled".to_owned(),
            escape_to_source_available: true,
            authority_posture: CapabilityAuthority::NoAuthorityExpansion,
            external_open_state: DocsExternalOpenState::NotRequired,
            open_externally_action: None,
            consent_ref: None,
            note: None,
        },
    ]
}

/// Returns the canonical recovery-to-source command.
pub fn seeded_recovery_command() -> WorkspaceRecoveryCommand {
    WorkspaceRecoveryCommand {
        command_id: RECOVER_SOURCE_COMMAND_ID.to_owned(),
        key_binding: "Escape".to_owned(),
        keyboard_reachable: true,
        label: "Recover raw source".to_owned(),
        reverts_to_mode: DocsPreviewMode::Source,
    }
}

/// Returns the fully-inert capability boundaries used in source mode.
pub fn not_applicable_capability_boundaries() -> Vec<PreviewCapabilityBoundary> {
    PreviewCapabilityKind::all()
        .into_iter()
        .map(|capability_kind| PreviewCapabilityBoundary {
            capability_kind,
            request_state: CapabilityRequestState::NotApplicable,
            render_posture: PreviewRenderPosture::NotApplicable,
            boundary_cue: "Source mode — nothing renders".to_owned(),
            escape_to_source_available: true,
            authority_posture: CapabilityAuthority::NoAuthorityExpansion,
            external_open_state: DocsExternalOpenState::NotRequired,
            open_externally_action: None,
            consent_ref: None,
            note: None,
        })
        .collect()
}

fn validate_capability_coverage(
    packet: &RenderedPreviewBoundary,
    violations: &mut Vec<PreviewBoundaryViolation>,
) {
    let covered: BTreeSet<PreviewCapabilityKind> = packet
        .capability_boundaries
        .iter()
        .map(|boundary| boundary.capability_kind)
        .collect();
    let covers_all = PreviewCapabilityKind::all()
        .iter()
        .all(|kind| covered.contains(kind));
    let no_duplicates = covered.len() == packet.capability_boundaries.len();
    if !covers_all || !no_duplicates {
        violations.push(PreviewBoundaryViolation::CapabilityCoverageIncomplete);
    }
}

fn validate_authority(
    packet: &RenderedPreviewBoundary,
    violations: &mut Vec<PreviewBoundaryViolation>,
) {
    if !packet.surface_owner.is_legitimate() {
        violations.push(PreviewBoundaryViolation::SurfaceOwnerImpersonatesAuthority);
    }
    if packet.no_authority_expansion_note.trim().is_empty() {
        violations.push(PreviewBoundaryViolation::NoAuthorityExpansionNoteMissing);
    }
}

fn validate_escapes(
    packet: &RenderedPreviewBoundary,
    violations: &mut Vec<PreviewBoundaryViolation>,
) {
    let recovery = &packet.recover_to_source_command;
    let recovery_ok = !recovery.command_id.trim().is_empty()
        && !recovery.key_binding.trim().is_empty()
        && !recovery.label.trim().is_empty()
        && recovery.keyboard_reachable
        && recovery.reverts_to_mode == DocsPreviewMode::Source;
    if !recovery_ok {
        violations.push(PreviewBoundaryViolation::RecoveryCommandInvalid);
    }

    if !action_is_well_formed(&packet.open_source_action) {
        violations.push(PreviewBoundaryViolation::OpenSourceActionInvalid);
    }
}

fn validate_mode_posture(
    packet: &RenderedPreviewBoundary,
    violations: &mut Vec<PreviewBoundaryViolation>,
) {
    if !packet.renders_preview() {
        // Source mode renders nothing: sanitization and every capability stay inert.
        let all_inert = packet
            .capability_boundaries
            .iter()
            .all(PreviewCapabilityBoundary::is_not_applicable);
        if packet.sanitization_state != DocsPreviewSanitizationState::NotApplicable || !all_inert {
            violations.push(PreviewBoundaryViolation::SourceModeRendersContent);
        }
        return;
    }

    // A rendering mode must carry a concrete, safe-by-default sanitization posture.
    if packet.sanitization_state == DocsPreviewSanitizationState::NotApplicable {
        violations.push(PreviewBoundaryViolation::UnsafePreviewDefault);
    }
    // A rendering mode must never leave every capability not-applicable.
    let all_not_applicable = packet
        .capability_boundaries
        .iter()
        .all(|boundary| boundary.render_posture == PreviewRenderPosture::NotApplicable);
    if all_not_applicable {
        violations.push(PreviewBoundaryViolation::UnsafePreviewDefault);
    }
    // A rendering mode must disclose that rendered output is not canonical source.
    if packet.rendered_is_not_canonical_note.trim().is_empty() {
        violations.push(PreviewBoundaryViolation::RenderedNotDisclosedAsNonCanonical);
    }
    // Allowed raw HTML must carry a disclosure note.
    if packet.sanitization_state.requires_disclosure()
        && packet
            .sanitization_note
            .as_deref()
            .map_or(true, |note| note.trim().is_empty())
    {
        violations.push(PreviewBoundaryViolation::SanitizationNoteMissing);
    }
}

fn validate_accessibility(
    packet: &RenderedPreviewBoundary,
    violations: &mut Vec<PreviewBoundaryViolation>,
) {
    let parity = &packet.accessibility_parity;
    if !parity.keyboard_parity {
        violations.push(PreviewBoundaryViolation::AccessibilityKeyboardParityMissing);
    }
    if !parity.is_fully_preserved()
        && parity
            .parity_note
            .as_deref()
            .map_or(true, |note| note.trim().is_empty())
    {
        violations.push(PreviewBoundaryViolation::AccessibilityParityDegradedWithoutNote);
    }
}

fn validate_source_version_badge(
    packet: &RenderedPreviewBoundary,
    violations: &mut Vec<PreviewBoundaryViolation>,
) {
    let badge = &packet.source_version_badge;
    let well_formed = ![
        &badge.source_class_token,
        &badge.source_pack_ref,
        &badge.source_revision_ref,
        &badge.version_or_revision_ref,
        &badge.source_build_at,
        &badge.running_build_identity_ref,
    ]
    .iter()
    .any(|value| value.trim().is_empty());
    if !well_formed {
        violations.push(PreviewBoundaryViolation::SourceVersionBadgeIncomplete);
    }
}

fn action_is_well_formed(action: &DocsMaintenanceAction) -> bool {
    !action.action_ref.trim().is_empty()
        && !action.action_label.trim().is_empty()
        && action.keyboard_reachable
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
