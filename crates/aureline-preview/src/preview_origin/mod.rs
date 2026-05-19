//! Preview-origin, preview-target, hot-reload, browser-runtime
//! session-origin, and source-mapping truth model for claimed beta preview
//! rows.
//!
//! Where `safe_preview` owns the representation-label wedge over risky text,
//! oversized artifacts, and generated content, this module owns the
//! preview / runtime / target / session-origin / source-mapping truth model
//! that every claimed beta preview row consults to answer four questions:
//!
//! 1. Which runtime or renderer produced the view the user is looking at
//!    right now? (PreviewOriginDescriptor)
//! 2. Which target kind does this view represent — a viewport preset, a
//!    design renderer, a simulator, a physical device, a browser tab, an
//!    embedded webview, or a remote preview target? (PreviewTargetDescriptor)
//! 3. How fresh and how exact is the source mapping that powers a source
//!    jump from this view, and what cross-origin / protocol limits apply
//!    when the runtime is a browser session? (SourceMappingDescriptor +
//!    BrowserRuntimeSessionOrigin)
//! 4. Whether the next action mutates local runtime state, browser state, or
//!    a remote target — and what side-effect review / support-export
//!    summary the surface must keep on screen. (RuntimeMutationActionPlan)
//!
//! The vocabularies below are the closed enum sets the cross-surface
//! schemas at
//! [`/schemas/preview/preview_target_descriptor.schema.json`](../../../../schemas/preview/preview_target_descriptor.schema.json),
//! [`/schemas/preview/hot_reload_state.schema.json`](../../../../schemas/preview/hot_reload_state.schema.json),
//! and
//! [`/schemas/browser_runtime/session_origin.schema.json`](../../../../schemas/browser_runtime/session_origin.schema.json)
//! freeze. Adding a new variant is additive-minor; repurposing an existing
//! value is breaking.

use serde::{Deserialize, Serialize};

mod browser_session;
mod hot_reload;
mod mutation_plan;
mod origin;
mod source_mapping;
mod target;

pub use browser_session::{
    BrowserRuntimeSessionOrigin, BrowserSessionOriginClass, BrowserSessionScopeClass,
    CrossOriginPostureClass, ProtocolPostureClass, BROWSER_SESSION_ORIGIN_RECORD_KIND,
    BROWSER_SESSION_ORIGIN_SCHEMA_VERSION,
};
pub use hot_reload::{
    HotReloadEventClass, HotReloadStateDescriptor, HotReloadStateRecoveryRoute,
    HotReloadUnderlyingStateClass, HOT_RELOAD_STATE_DESCRIPTOR_RECORD_KIND,
    HOT_RELOAD_STATE_DESCRIPTOR_SCHEMA_VERSION,
};
pub use mutation_plan::{
    MutationActionKind, MutationBlastClass, MutationReviewRequirement, RuntimeMutationActionPlan,
    RUNTIME_MUTATION_ACTION_PLAN_RECORD_KIND, RUNTIME_MUTATION_ACTION_PLAN_SCHEMA_VERSION,
};
pub use origin::{
    PreviewOriginClass, PreviewOriginDescriptor, PreviewOriginLifecyclePhase,
    PreviewOriginSharingPosture, PREVIEW_ORIGIN_DESCRIPTOR_RECORD_KIND,
    PREVIEW_ORIGIN_DESCRIPTOR_SCHEMA_VERSION,
};
pub use source_mapping::{SourceMappingDescriptor, SourceMappingQualityClass};
pub use target::{
    DeviceCapabilityClass, PreviewTargetClass, PreviewTargetDescriptor,
    PreviewTargetReducedCapabilityReason, PREVIEW_TARGET_DESCRIPTOR_RECORD_KIND,
    PREVIEW_TARGET_DESCRIPTOR_SCHEMA_VERSION,
};

/// Closed lane vocabulary mirrored from the existing
/// `preview_runtime_strip.schema.json`. Re-exported here so consumers of the
/// preview-origin truth model do not have to import the strip module just to
/// reference a lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewLaneClass {
    BrowserPreviewLane,
    NativePreviewLane,
    EmbeddedPreviewLane,
}

impl PreviewLaneClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrowserPreviewLane => "browser_preview_lane",
            Self::NativePreviewLane => "native_preview_lane",
            Self::EmbeddedPreviewLane => "embedded_preview_lane",
        }
    }
}

/// Closed validation-finding shape shared by the preview-origin records.
/// Mirrors the framework-status-strip `Finding` type so support exports and
/// audit packets can quote violations verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewOriginFinding {
    pub check_id: String,
    pub subject_ref: String,
    pub message: String,
}

impl PreviewOriginFinding {
    pub(crate) fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

#[cfg(test)]
mod tests;
