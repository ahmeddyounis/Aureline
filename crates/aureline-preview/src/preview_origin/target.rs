//! Preview-target descriptor.
//!
//! Names the *kind* of target the user sees. The spec freezes seven target
//! kinds (viewport preset, design renderer, simulator, physical device,
//! browser tab, embedded webview, remote preview target) and a closed
//! reduced-capability vocabulary so degraded targets cannot inherit the
//! desktop default's capability set by silence.
//!
//! The descriptor is intentionally narrower than the existing
//! `device_target_descriptor` (which is the device-picker row descriptor):
//! this one carries one row of "what is on screen right now" rather than
//! the whole picker.

use serde::{Deserialize, Serialize};

use super::PreviewOriginFinding;

/// Stable record-kind tag.
pub const PREVIEW_TARGET_DESCRIPTOR_RECORD_KIND: &str = "preview_target_descriptor_record";

/// Schema version mirrored by
/// `/schemas/preview/preview_target_descriptor.schema.json#preview_target_descriptor_record`.
pub const PREVIEW_TARGET_DESCRIPTOR_SCHEMA_VERSION: u32 = 1;

/// Closed preview-target vocabulary. Always renders as a labeled chip;
/// reduced-capability or unsupported targets degrade through
/// [`PreviewTargetReducedCapabilityReason`] rather than masquerading as
/// `desktop_default_viewport_preset`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewTargetClass {
    /// Viewport preset only — the view is a layout slice rendered by the
    /// design renderer, not bound to a real device or browser process.
    ViewportPresetOnly,
    /// Design renderer — Aureline's in-process design / token renderer
    /// running over a captured component or page snapshot.
    DesignRendererTarget,
    /// Simulator — an OS-vendor simulator (iOS Simulator, Android Emulator
    /// in simulator mode, etc.). Tethered through a workspace-bound
    /// adapter.
    SimulatorTarget,
    /// Physical device — a real phone / tablet / wearable / TV connected
    /// over a workspace-bound transport (USB, wireless adb, mDNS, etc.).
    PhysicalDeviceTarget,
    /// Browser tab — a tab inside an attached browser (local Chromium /
    /// Firefox / Safari) that Aureline drives via the browser-runtime
    /// session-origin descriptor.
    BrowserTabTarget,
    /// Embedded webview — a webview hosted inside Aureline's shell or an
    /// extension-host. Not a separate browser process.
    EmbeddedWebviewTarget,
    /// Remote preview target — a preview running on a remote / container
    /// / managed runtime, projected here as a view (e.g. via a remote
    /// devtools bridge, a remote frame stream, or a captured snapshot).
    RemotePreviewTarget,
}

impl PreviewTargetClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ViewportPresetOnly => "viewport_preset_only",
            Self::DesignRendererTarget => "design_renderer_target",
            Self::SimulatorTarget => "simulator_target",
            Self::PhysicalDeviceTarget => "physical_device_target",
            Self::BrowserTabTarget => "browser_tab_target",
            Self::EmbeddedWebviewTarget => "embedded_webview_target",
            Self::RemotePreviewTarget => "remote_preview_target",
        }
    }

    /// True for targets that represent a real device or running runtime
    /// (not a layout slice).
    pub const fn is_real_runtime_target(self) -> bool {
        !matches!(self, Self::ViewportPresetOnly | Self::DesignRendererTarget)
    }

    /// True for targets that participate in the browser-runtime
    /// inspection lane (and therefore can publish a
    /// [`crate::preview_origin::BrowserRuntimeSessionOrigin`]).
    pub const fn participates_in_browser_runtime(self) -> bool {
        matches!(
            self,
            Self::BrowserTabTarget | Self::EmbeddedWebviewTarget | Self::RemotePreviewTarget
        )
    }
}

/// Closed device-capability vocabulary. Names the capability profile a
/// target row can honestly claim. Reduced-capability rows MUST cite a
/// reason from [`PreviewTargetReducedCapabilityReason`] alongside the
/// `reduced_capability` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceCapabilityClass {
    DesktopDefault,
    Touch,
    Mobile,
    Tablet,
    Wearable,
    Television,
    HighDpi,
    LowResource,
    /// Reduced capability — at least one core capability is unavailable
    /// or restricted. MUST accompany a reason.
    ReducedCapability,
    /// Target kind does not support the capability concept at all (e.g. a
    /// pure static evidence projection).
    NotApplicable,
}

impl DeviceCapabilityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopDefault => "desktop_default",
            Self::Touch => "touch",
            Self::Mobile => "mobile",
            Self::Tablet => "tablet",
            Self::Wearable => "wearable",
            Self::Television => "television",
            Self::HighDpi => "high_dpi",
            Self::LowResource => "low_resource",
            Self::ReducedCapability => "reduced_capability",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed reduced-capability reason vocabulary. The chrome quotes the
/// reason verbatim so a degraded simulator never silently inherits the
/// desktop default's capability claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewTargetReducedCapabilityReason {
    None,
    NoInputDevice,
    NoCamera,
    NoGpuAcceleration,
    LowResourceMode,
    NetworkRestricted,
    ProtocolDowngraded,
    PolicyNarrowed,
    UnsupportedRuntimeFeature,
    EmulatorFallback,
}

impl PreviewTargetReducedCapabilityReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::NoInputDevice => "no_input_device",
            Self::NoCamera => "no_camera",
            Self::NoGpuAcceleration => "no_gpu_acceleration",
            Self::LowResourceMode => "low_resource_mode",
            Self::NetworkRestricted => "network_restricted",
            Self::ProtocolDowngraded => "protocol_downgraded",
            Self::PolicyNarrowed => "policy_narrowed",
            Self::UnsupportedRuntimeFeature => "unsupported_runtime_feature",
            Self::EmulatorFallback => "emulator_fallback",
        }
    }
}

/// Canonical preview-target descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewTargetDescriptor {
    pub record_kind: String,
    pub preview_target_descriptor_schema_version: u32,
    pub preview_target_descriptor_id: String,
    /// ISO 8601 UTC monotonic timestamp.
    pub observed_at: String,

    pub preview_target_class: PreviewTargetClass,
    pub device_capability_class: DeviceCapabilityClass,
    pub reduced_capability_reason: PreviewTargetReducedCapabilityReason,

    /// Opaque ref to the `preview_origin_descriptor_record` that owns the
    /// runtime behind this target.
    pub preview_origin_descriptor_ref: String,
    /// Opaque ref to the device-picker descriptor when the target row was
    /// chosen through the picker. Optional — purely synthetic targets
    /// (e.g. the design renderer) may not have a picker descriptor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_target_descriptor_ref: Option<String>,
    /// Opaque ref to the browser-runtime session-origin record when the
    /// target participates in the browser-runtime inspection lane.
    /// Required for browser_tab_target, embedded_webview_target, and any
    /// remote_preview_target that publishes a session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_runtime_session_origin_ref: Option<String>,

    /// Optional viewport width / height for layout slices. Real device
    /// targets typically leave these null.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewport_pixel_width: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub viewport_pixel_height: Option<u32>,

    /// Reviewer-facing redacted label safe to render on the chip.
    pub redacted_label: String,
    pub summary: String,
}

impl PreviewTargetDescriptor {
    pub fn validate(&self) -> Vec<PreviewOriginFinding> {
        let mut findings = Vec::new();
        let subject = self.preview_target_descriptor_id.as_str();

        if self.record_kind != PREVIEW_TARGET_DESCRIPTOR_RECORD_KIND {
            findings.push(PreviewOriginFinding::new(
                "preview_target_descriptor.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    PREVIEW_TARGET_DESCRIPTOR_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.preview_target_descriptor_schema_version != PREVIEW_TARGET_DESCRIPTOR_SCHEMA_VERSION
        {
            findings.push(PreviewOriginFinding::new(
                "preview_target_descriptor.schema_version",
                subject,
                format!(
                    "schema_version must be {}, found {}",
                    PREVIEW_TARGET_DESCRIPTOR_SCHEMA_VERSION,
                    self.preview_target_descriptor_schema_version
                ),
            ));
        }

        if matches!(
            self.device_capability_class,
            DeviceCapabilityClass::ReducedCapability
        ) && self.reduced_capability_reason == PreviewTargetReducedCapabilityReason::None
        {
            findings.push(PreviewOriginFinding::new(
                "preview_target_descriptor.reduced_capability_requires_reason",
                subject,
                "reduced_capability requires a reason other than none",
            ));
        }
        if !matches!(
            self.device_capability_class,
            DeviceCapabilityClass::ReducedCapability
        ) && self.reduced_capability_reason != PreviewTargetReducedCapabilityReason::None
        {
            findings.push(PreviewOriginFinding::new(
                "preview_target_descriptor.unreduced_capability_forbids_reason",
                subject,
                "non-reduced capability classes require reduced_capability_reason = none",
            ));
        }

        if matches!(
            self.preview_target_class,
            PreviewTargetClass::ViewportPresetOnly
        ) && matches!(
            self.device_capability_class,
            DeviceCapabilityClass::DesktopDefault
        ) && (self.viewport_pixel_width.is_none() || self.viewport_pixel_height.is_none())
        {
            findings.push(PreviewOriginFinding::new(
                "preview_target_descriptor.viewport_preset_requires_dimensions",
                subject,
                "viewport_preset_only with desktop_default capability must publish pixel width and height",
            ));
        }

        if self.preview_target_class.participates_in_browser_runtime()
            && self.browser_runtime_session_origin_ref.is_none()
        {
            findings.push(PreviewOriginFinding::new(
                "preview_target_descriptor.browser_runtime_requires_session_origin",
                subject,
                "browser-runtime targets must reference a browser_runtime_session_origin record",
            ));
        }
        if !self.preview_target_class.participates_in_browser_runtime()
            && self.browser_runtime_session_origin_ref.is_some()
        {
            findings.push(PreviewOriginFinding::new(
                "preview_target_descriptor.non_browser_runtime_must_not_publish_session_origin",
                subject,
                "only browser_tab / embedded_webview / remote_preview targets may reference a browser_runtime_session_origin",
            ));
        }

        findings
    }

    /// Render a deterministic plaintext summary safe to embed in support
    /// exports.
    pub fn render_plaintext(&self) -> String {
        format!(
            "preview_target {id} kind={kind} capability={cap} reduced={reason}: {summary}",
            id = self.preview_target_descriptor_id,
            kind = self.preview_target_class.as_str(),
            cap = self.device_capability_class.as_str(),
            reason = self.reduced_capability_reason.as_str(),
            summary = self.summary,
        )
    }
}
