//! Drift-and-recovery drills for hot-reload, source-map freshness, data
//! posture, and reconnect-safe dev-server / browser / device handling across
//! the claimed M5 preview lanes.
//!
//! Where [`crate::preview_session_descriptors`] materializes the *static*
//! per-session truth a preview surface presents right now, and
//! [`crate::preview_origin`] materializes the per-view origin / target /
//! mapping objects, this module materializes the **transition-time** truth: a
//! governed set of recovery drills, one per real M5 preview failure, that each
//! capture a before/after truth snapshot and prove the session preserves
//! source-sync, target-kind, and source-mapping-quality truth when a runtime
//! resets, a source map goes stale, a dev server disappears, a device
//! reconnects, a browser session expires, a runtime is replaced, or the data
//! posture flips between live / mock / captured.
//!
//! A [`PreviewDriftRecoveryDrillSet`] is the canonical answer to "when the
//! preview lane hits the failures users actually hit in M5 framework / runtime
//! work, does it fail honestly — keeping the target it was bound to, refusing to
//! claim a fresher source map than it has, and exporting a precise degraded
//! label and trigger — instead of going blank or silently jumping to the wrong
//! source?"
//!
//! Each [`PreviewDriftRecoveryDrill`] reuses the frozen
//! [`SourceSyncClass`](crate::freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix::SourceSyncClass),
//! the per-session
//! [`PreviewDataPostureClass`](crate::preview_session_descriptors::PreviewDataPostureClass),
//! [`PreviewFreshnessClass`](crate::preview_session_descriptors::PreviewFreshnessClass),
//! and [`PreviewConsumerSurface`](crate::preview_session_descriptors::PreviewConsumerSurface)
//! vocabularies, and the per-view
//! [`PreviewTargetClass`](crate::preview_origin::PreviewTargetClass),
//! [`DeviceCapabilityClass`](crate::preview_origin::DeviceCapabilityClass),
//! [`PreviewOriginClass`](crate::preview_origin::PreviewOriginClass), and
//! [`SourceMappingQualityClass`](crate::preview_origin::SourceMappingQualityClass)
//! vocabularies rather than minting synonyms. It adds the transition-level
//! dimensions this lane owns: [`DriftEventClass`], [`DriftRecoveryRoute`], and
//! [`DriftRecoveryTrigger`].
//!
//! The drill keeps source canonical. A drift never silently swaps the target
//! the session was bound to; a stale-source-map drift cannot keep claiming an
//! exact mapping; a lost dev server or expired browser session can no longer
//! claim a live runtime; a data-posture flip must actually change the governed
//! data chip; and every degraded post-drift state exports a precise, non-generic
//! degraded label and a recorded trigger that survives reopen / export so
//! downstream diagnostics, support, and release packets reuse the recovery
//! truth directly.
//!
//! Raw URLs, hostnames, cookies, raw provider payloads, credentials, and raw
//! runtime handles never cross this boundary; the packet carries only typed
//! class tokens, opaque revision / evidence refs, booleans, and redacted
//! labels.
//!
//! The boundary schema is
//! [`schemas/preview/preview_drift_recovery_drill_set.schema.json`](../../../../schemas/preview/preview_drift_recovery_drill_set.schema.json).
//! The contract doc is
//! [`docs/preview/m5/preview_drift_recovery_drills.md`](../../../../docs/preview/m5/preview_drift_recovery_drills.md).
//! The protected fixture directory is
//! [`fixtures/preview/m5/preview_drift_recovery_drills/`](../../../../fixtures/preview/m5/preview_drift_recovery_drills/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix::SourceSyncClass;
use crate::preview_origin::{
    DeviceCapabilityClass, PreviewOriginClass, PreviewTargetClass, SourceMappingQualityClass,
};
use crate::preview_session_descriptors::{
    PreviewConsumerSurface, PreviewDataPostureClass, PreviewFreshnessClass,
};

/// Stable record-kind tag carried by [`PreviewDriftRecoveryDrillSet`].
pub const PREVIEW_DRIFT_RECOVERY_DRILL_SET_RECORD_KIND: &str = "preview_drift_recovery_drill_set";

/// Schema version for the preview drift-recovery drill set.
pub const PREVIEW_DRIFT_RECOVERY_DRILL_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PREVIEW_DRIFT_RECOVERY_DRILL_SET_SCHEMA_REF: &str =
    "schemas/preview/preview_drift_recovery_drill_set.schema.json";

/// Repo-relative path of the contract doc.
pub const PREVIEW_DRIFT_RECOVERY_DRILL_SET_DOC_REF: &str =
    "docs/preview/m5/preview_drift_recovery_drills.md";

/// Repo-relative path of the protected fixture directory.
pub const PREVIEW_DRIFT_RECOVERY_DRILL_SET_FIXTURE_DIR: &str =
    "fixtures/preview/m5/preview_drift_recovery_drills";

/// Repo-relative path of the checked support-export artifact.
pub const PREVIEW_DRIFT_RECOVERY_DRILL_SET_ARTIFACT_REF: &str =
    "artifacts/preview/m5/preview_drift_recovery_drills/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const PREVIEW_DRIFT_RECOVERY_DRILL_SET_SUMMARY_REF: &str =
    "artifacts/preview/m5/preview_drift_recovery_drills.md";

/// Closed drift-event vocabulary: the real failures M5 preview lanes hit. The
/// chrome quotes the event token directly rather than collapsing every failure
/// into one generic "preview unavailable" state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftEventClass {
    /// A hot-reload reset swapped the module graph and discarded in-memory
    /// runtime state while the runtime stayed reachable.
    HotReloadReset,
    /// The source map that backed source jumps went stale relative to the
    /// canonical source.
    StaleSourceMap,
    /// The dev server backing a live preview disappeared.
    DevServerLost,
    /// A tethered device dropped and re-attached over the workspace transport.
    DeviceReconnect,
    /// The browser-runtime session expired and must be re-established.
    BrowserSessionExpired,
    /// The runtime behind the preview was replaced by a different runtime.
    RuntimeReplaced,
    /// The data posture flipped between live / mock / captured.
    DataPostureFlip,
}

impl DriftEventClass {
    /// The drift events this lane must drill, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::HotReloadReset,
        Self::StaleSourceMap,
        Self::DevServerLost,
        Self::DeviceReconnect,
        Self::BrowserSessionExpired,
        Self::RuntimeReplaced,
        Self::DataPostureFlip,
    ];

    /// Stable token recorded in the drill.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HotReloadReset => "hot_reload_reset",
            Self::StaleSourceMap => "stale_source_map",
            Self::DevServerLost => "dev_server_lost",
            Self::DeviceReconnect => "device_reconnect",
            Self::BrowserSessionExpired => "browser_session_expired",
            Self::RuntimeReplaced => "runtime_replaced",
            Self::DataPostureFlip => "data_posture_flip",
        }
    }
}

/// Closed recovery-route vocabulary the drill admits. Each event class admits a
/// fixed subset; [`PreviewDriftRecoveryDrill`] validation enforces the gate so a
/// drill cannot advertise a route that does not apply to its failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftRecoveryRoute {
    /// Re-establish a session against the same runtime (no source change).
    ReconnectSameRuntime,
    /// Restart the runtime process and reload.
    RestartRuntime,
    /// Rebuild from source, then reload.
    RebuildThenReload,
    /// Re-derive the source map, then reload.
    RemapSourceThenReload,
    /// Re-attach the tethered device session.
    ReattachDeviceSession,
    /// Re-import a capture snapshot to replace a lost live feed.
    ReimportCaptureSnapshot,
    /// Hold the view inspect-only until the source map is re-derived.
    HoldInspectOnlyUntilRemapped,
    /// Open the canonical source directly.
    OpenCanonicalSource,
    /// Export metadata only when no live view can be recovered.
    ExportMetadataOnly,
}

impl DriftRecoveryRoute {
    /// Stable token recorded in the drill.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconnectSameRuntime => "reconnect_same_runtime",
            Self::RestartRuntime => "restart_runtime",
            Self::RebuildThenReload => "rebuild_then_reload",
            Self::RemapSourceThenReload => "remap_source_then_reload",
            Self::ReattachDeviceSession => "reattach_device_session",
            Self::ReimportCaptureSnapshot => "reimport_capture_snapshot",
            Self::HoldInspectOnlyUntilRemapped => "hold_inspect_only_until_remapped",
            Self::OpenCanonicalSource => "open_canonical_source",
            Self::ExportMetadataOnly => "export_metadata_only",
        }
    }
}

/// Closed drift-recovery trigger vocabulary. Names why a post-drift snapshot is
/// held below a current, in-sync, fresh posture; the chrome quotes the trigger
/// verbatim instead of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftRecoveryTrigger {
    /// The session must be reconnected before it is trustworthy again.
    ReconnectRequired,
    /// The backing runtime is no longer reachable.
    RuntimeUnavailable,
    /// The source map went stale relative to the canonical source.
    SourceMapStale,
    /// No source map is available for jumps after the drift.
    SourceMapUnavailable,
    /// The view drifted from the canonical source.
    DriftedFromSource,
    /// The data posture was reclassified and must be re-confirmed.
    DataPostureReclassified,
    /// The view is awaiting a rebuild from source.
    AwaitingRebuild,
}

impl DriftRecoveryTrigger {
    /// Stable token recorded in the drill.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconnectRequired => "reconnect_required",
            Self::RuntimeUnavailable => "runtime_unavailable",
            Self::SourceMapStale => "source_map_stale",
            Self::SourceMapUnavailable => "source_map_unavailable",
            Self::DriftedFromSource => "drifted_from_source",
            Self::DataPostureReclassified => "data_posture_reclassified",
            Self::AwaitingRebuild => "awaiting_rebuild",
        }
    }
}

/// A point-in-time snapshot of the user-visible preview truth, captured either
/// before a drift event or in the post-drift, pre-full-recovery state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftTruthSnapshot {
    /// Source-sync state of the derivative preview against canonical source.
    pub source_sync_class: SourceSyncClass,
    /// Data posture chip.
    pub data_posture: PreviewDataPostureClass,
    /// Freshness chip.
    pub freshness_class: PreviewFreshnessClass,
    /// Device / viewport target kind on screen.
    pub target_kind: PreviewTargetClass,
    /// Device capability profile of the target.
    pub device_capability_class: DeviceCapabilityClass,
    /// Source-mapping quality the surface advertises for jumps.
    pub source_mapping_quality: SourceMappingQualityClass,
    /// Runtime identity backing the view.
    pub runtime_origin_class: PreviewOriginClass,
    /// True when a live runtime backs the view.
    pub runtime_backed: bool,
    /// True when the session must be reconnected before it is trustworthy.
    pub reconnect_required: bool,
}

impl DriftTruthSnapshot {
    /// Whether this snapshot must be held below a current / in-sync / fresh
    /// posture because of stale, drifted, unidentified, or disconnected state.
    pub fn is_degraded(&self) -> bool {
        self.source_sync_class.is_unidentified()
            || matches!(self.source_sync_class, SourceSyncClass::DriftedFromSource)
            || self.freshness_class.forces_downgrade()
            || self.data_posture.is_unidentified()
            || matches!(
                self.source_mapping_quality,
                SourceMappingQualityClass::Stale | SourceMappingQualityClass::Unavailable
            )
            || self.reconnect_required
    }

    /// Whether the snapshot's chips are internally honest: a live posture is
    /// backed by a live runtime, a captured posture has no live runtime, a
    /// runtime-only view is runtime-backed, and the source-mapping quality does
    /// not over-claim against the source-sync state.
    pub fn is_consistent(&self) -> bool {
        // Live data must be backed by a live runtime that can emit live events.
        if self.data_posture.is_live()
            && !(self.runtime_backed && self.runtime_origin_class.admits_live_runtime_events())
        {
            return false;
        }
        // A captured snapshot has no live runtime feed.
        if self.data_posture.is_captured() && self.runtime_backed {
            return false;
        }
        // A runtime-only view must actually be runtime-backed.
        if self.source_sync_class.is_runtime_only() && !self.runtime_backed {
            return false;
        }
        // A drifted view cannot claim an exact source map.
        if matches!(self.source_sync_class, SourceSyncClass::DriftedFromSource)
            && matches!(
                self.source_mapping_quality,
                SourceMappingQualityClass::Exact
            )
        {
            return false;
        }
        // An in-sync view cannot carry a stale source map.
        if matches!(self.source_sync_class, SourceSyncClass::InSyncFromSource)
            && matches!(
                self.source_mapping_quality,
                SourceMappingQualityClass::Stale
            )
        {
            return false;
        }
        true
    }
}

/// One drift-and-recovery drill: a before/after transition triggered by a real
/// M5 preview failure, proving the session fails honestly and preserves the
/// truth it was bound to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewDriftRecoveryDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// The drift event this drill exercises.
    pub drift_event_class: DriftEventClass,
    /// Consumer surface the drill runs against (reused vocabulary).
    pub consumer_surface: PreviewConsumerSurface,
    /// Human-readable label summary safe to render on the chip strip.
    pub label_summary: String,
    /// ISO 8601 UTC timestamp the drill was observed.
    pub observed_at: String,

    /// User-visible truth before the drift.
    pub before: DriftTruthSnapshot,
    /// User-visible truth in the post-drift, pre-full-recovery state.
    pub after: DriftTruthSnapshot,

    /// Admissible recovery routes for this drift.
    pub recovery_routes: Vec<DriftRecoveryRoute>,

    /// True when the post-drift degraded label and trigger survive reopen and
    /// export rather than resetting to a blank or "current" state.
    pub survives_reopen_export: bool,

    /// Trigger that holds the post-drift snapshot below current; required when
    /// the `after` snapshot is degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<DriftRecoveryTrigger>,
    /// Precise degraded label; required when the `after` snapshot is degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,

    /// Evidence packet refs backing this drill.
    pub evidence_refs: Vec<String>,
}

impl PreviewDriftRecoveryDrill {
    /// Whether the target the session was bound to is preserved across the
    /// drift. A drift must never silently swap to a different target.
    pub fn target_kind_preserved(&self) -> bool {
        self.before.target_kind == self.after.target_kind
    }

    /// Whether the post-drift snapshot is degraded and therefore must carry a
    /// trigger and a precise label.
    pub fn after_is_degraded(&self) -> bool {
        self.after.is_degraded()
    }

    /// Whether the downgrade evidence is consistent: a degraded post-drift
    /// snapshot carries both a recorded trigger and a precise, non-generic
    /// degraded label, and a cleanly recovered snapshot carries neither.
    pub fn downgrade_consistent(&self) -> bool {
        if self.after_is_degraded() {
            self.downgrade_trigger.is_some()
                && self
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.downgrade_trigger.is_none() && self.degraded_label.is_none()
        }
    }

    /// Whether the post-drift snapshot is honest for the specific drift event:
    /// e.g. a stale-source-map drift cannot keep claiming a fresh map, a lost
    /// dev server or expired session cannot keep claiming a live runtime, and a
    /// data-posture flip must actually change the governed data chip.
    pub fn event_after_consistent(&self) -> bool {
        match self.drift_event_class {
            DriftEventClass::HotReloadReset => {
                // The runtime stayed reachable, so its identity is preserved.
                self.after.runtime_origin_class == self.before.runtime_origin_class
            }
            DriftEventClass::StaleSourceMap => matches!(
                self.after.source_mapping_quality,
                SourceMappingQualityClass::Stale | SourceMappingQualityClass::Unavailable
            ),
            DriftEventClass::DevServerLost | DriftEventClass::BrowserSessionExpired => {
                // The live feed is gone; the view can no longer claim a runtime.
                !self.after.runtime_backed
            }
            DriftEventClass::DeviceReconnect => {
                // The same device re-attaches, so its runtime identity holds.
                self.after.runtime_origin_class == self.before.runtime_origin_class
            }
            DriftEventClass::RuntimeReplaced => {
                // A replaced runtime must re-derive sync state rather than carry
                // the previous runtime's in-sync claim forward unchanged.
                !(matches!(
                    self.before.source_sync_class,
                    SourceSyncClass::InSyncFromSource
                ) && matches!(
                    self.after.source_sync_class,
                    SourceSyncClass::InSyncFromSource
                ) && self.after.runtime_origin_class == self.before.runtime_origin_class)
            }
            DriftEventClass::DataPostureFlip => self.after.data_posture != self.before.data_posture,
        }
    }

    /// Whether every recovery route is admissible for the drift event.
    pub fn recovery_routes_admissible(&self) -> bool {
        let allowed = allowed_recovery_routes_for(self.drift_event_class);
        !self.recovery_routes.is_empty()
            && self
                .recovery_routes
                .iter()
                .all(|route| allowed.contains(route))
    }

    /// Deterministic governed chip line summarizing the transition.
    pub fn chip_tokens(&self) -> String {
        format!(
            "event={event} surface={surface} before[sync={bsync} map={bmap} data={bdata}] after[sync={async_} map={amap} data={adata}]",
            event = self.drift_event_class.as_str(),
            surface = self.consumer_surface.as_str(),
            bsync = self.before.source_sync_class.as_str(),
            bmap = self.before.source_mapping_quality.as_str(),
            bdata = self.before.data_posture.as_str(),
            async_ = self.after.source_sync_class.as_str(),
            amap = self.after.source_mapping_quality.as_str(),
            adata = self.after.data_posture.as_str(),
        )
    }

    /// Whether every dimension required to record this drill is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.drill_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.observed_at.trim().is_empty()
            && self.before.is_consistent()
            && self.after.is_consistent()
            && self.target_kind_preserved()
            && self.downgrade_consistent()
            && self.event_after_consistent()
            && self.recovery_routes_admissible()
            && self.survives_reopen_export
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Admissible recovery routes per drift event. Mirrors the schema's per-event
/// allowed set so the Rust and schema sides speak the same language.
fn allowed_recovery_routes_for(event: DriftEventClass) -> &'static [DriftRecoveryRoute] {
    use DriftEventClass as E;
    use DriftRecoveryRoute as R;
    match event {
        E::HotReloadReset => &[
            R::ReconnectSameRuntime,
            R::RebuildThenReload,
            R::OpenCanonicalSource,
        ],
        E::StaleSourceMap => &[
            R::RemapSourceThenReload,
            R::HoldInspectOnlyUntilRemapped,
            R::RebuildThenReload,
            R::OpenCanonicalSource,
        ],
        E::DevServerLost => &[
            R::RestartRuntime,
            R::ReconnectSameRuntime,
            R::ReimportCaptureSnapshot,
            R::OpenCanonicalSource,
            R::ExportMetadataOnly,
        ],
        E::DeviceReconnect => &[
            R::ReattachDeviceSession,
            R::ReconnectSameRuntime,
            R::OpenCanonicalSource,
        ],
        E::BrowserSessionExpired => &[
            R::ReconnectSameRuntime,
            R::RestartRuntime,
            R::OpenCanonicalSource,
            R::ExportMetadataOnly,
        ],
        E::RuntimeReplaced => &[
            R::RebuildThenReload,
            R::ReconnectSameRuntime,
            R::OpenCanonicalSource,
        ],
        E::DataPostureFlip => &[
            R::ReconnectSameRuntime,
            R::ReimportCaptureSnapshot,
            R::OpenCanonicalSource,
        ],
    }
}

/// Guardrail invariants block for the drift-recovery drill set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftRecoveryGuardrails {
    /// Source remains canonical; the drill is derivative, never a second
    /// writable truth model.
    pub source_canonical_no_second_writable_model: bool,
    /// A drift never silently swaps the target the session was bound to.
    pub drift_never_silently_swaps_target: bool,
    /// Runtime state never hides source-mapping uncertainty across a drift.
    pub runtime_state_never_hides_source_mapping_uncertainty: bool,
    /// Inspect-only holds are never auto-upgraded into write-capable flows.
    pub inspect_only_never_auto_upgraded_to_write: bool,
    /// Embedded preview/browser boundaries are not blurred into product authority.
    pub embedded_boundaries_not_blurred_into_product: bool,
    /// Degraded post-drift state exports precise truth that survives reopen.
    pub degraded_state_exports_truth_surviving_reopen: bool,
}

impl DriftRecoveryGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.source_canonical_no_second_writable_model
            && self.drift_never_silently_swaps_target
            && self.runtime_state_never_hides_source_mapping_uncertainty
            && self.inspect_only_never_auto_upgraded_to_write
            && self.embedded_boundaries_not_blurred_into_product
            && self.degraded_state_exports_truth_surviving_reopen
    }
}

/// Consumer-projection block for the drift-recovery drill set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DriftRecoveryConsumerProjection {
    /// Product surfaces ingest these drills instead of cloning recovery copy.
    pub product_ingests_drills: bool,
    /// Docs/help ingests the same drills.
    pub docs_help_ingests_drills: bool,
    /// Diagnostics ingests the same drills.
    pub diagnostics_ingests_drills: bool,
    /// Support export ingests the same drills.
    pub support_export_ingests_drills: bool,
    /// Release-control surfaces ingest the same drills.
    pub release_control_ingests_drills: bool,
    /// Degraded post-drift state is visibly labeled below current everywhere.
    pub degraded_state_labeled_below_current: bool,
}

impl DriftRecoveryConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_drills
            && self.docs_help_ingests_drills
            && self.diagnostics_ingests_drills
            && self.support_export_ingests_drills
            && self.release_control_ingests_drills
            && self.degraded_state_labeled_below_current
    }
}

/// Constructor input for [`PreviewDriftRecoveryDrillSet::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreviewDriftRecoveryDrillSetInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-event recovery drills.
    pub drills: Vec<PreviewDriftRecoveryDrill>,
    /// Guardrail invariants block.
    pub guardrails: DriftRecoveryGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DriftRecoveryConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe preview drift-recovery drill set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewDriftRecoveryDrillSet {
    /// Record kind; must equal [`PREVIEW_DRIFT_RECOVERY_DRILL_SET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PREVIEW_DRIFT_RECOVERY_DRILL_SET_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Per-event recovery drills.
    pub drills: Vec<PreviewDriftRecoveryDrill>,
    /// Guardrail invariants block.
    pub guardrails: DriftRecoveryGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DriftRecoveryConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl PreviewDriftRecoveryDrillSet {
    /// Builds a preview drift-recovery drill set packet.
    pub fn new(input: PreviewDriftRecoveryDrillSetInput) -> Self {
        Self {
            record_kind: PREVIEW_DRIFT_RECOVERY_DRILL_SET_RECORD_KIND.to_owned(),
            schema_version: PREVIEW_DRIFT_RECOVERY_DRILL_SET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            drills: input.drills,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Drift events represented by some drill in this set.
    pub fn represented_events(&self) -> BTreeSet<DriftEventClass> {
        self.drills.iter().map(|d| d.drift_event_class).collect()
    }

    /// Count of drills whose post-drift snapshot is degraded.
    pub fn degraded_drill_count(&self) -> usize {
        self.drills.iter().filter(|d| d.after_is_degraded()).count()
    }

    /// Count of drills that recover cleanly (post-drift snapshot not degraded).
    pub fn clean_recovery_count(&self) -> usize {
        self.drills
            .iter()
            .filter(|d| !d.after_is_degraded())
            .count()
    }

    /// Validates the drift-recovery drill set invariants.
    pub fn validate(&self) -> Vec<PreviewDriftRecoveryDrillSetViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PREVIEW_DRIFT_RECOVERY_DRILL_SET_RECORD_KIND {
            violations.push(PreviewDriftRecoveryDrillSetViolation::WrongRecordKind);
        }
        if self.schema_version != PREVIEW_DRIFT_RECOVERY_DRILL_SET_SCHEMA_VERSION {
            violations.push(PreviewDriftRecoveryDrillSetViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(PreviewDriftRecoveryDrillSetViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_drills(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("preview drift recovery drill set serializes"),
        ) {
            violations.push(PreviewDriftRecoveryDrillSetViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("preview drift recovery drill set serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Preview Drift-Recovery Drills\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!(
            "- Drills: {} ({} degraded, {} clean recovery)\n",
            self.drills.len(),
            self.degraded_drill_count(),
            self.clean_recovery_count(),
        ));
        out.push_str(&format!(
            "- Events: {} / {}\n",
            self.represented_events().len(),
            DriftEventClass::ALL.len(),
        ));
        out.push_str("\n## Drills\n\n");
        for drill in &self.drills {
            out.push_str(&format!(
                "- **{}** ({})\n",
                drill.drill_id,
                drill.drift_event_class.as_str()
            ));
            out.push_str(&format!("  - {}\n", drill.label_summary));
            out.push_str(&format!("  - {}\n", drill.chip_tokens()));
            let routes: Vec<&str> = drill.recovery_routes.iter().map(|r| r.as_str()).collect();
            out.push_str(&format!("  - recovery: {}\n", routes.join(",")));
            if let Some(label) = &drill.degraded_label {
                out.push_str(&format!("  - Degraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in drift-recovery drill set export.
#[derive(Debug)]
pub enum PreviewDriftRecoveryDrillSetArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<PreviewDriftRecoveryDrillSetViolation>),
}

impl fmt::Display for PreviewDriftRecoveryDrillSetArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "preview drift recovery drill set export parse failed: {error}"
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
                    "preview drift recovery drill set export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for PreviewDriftRecoveryDrillSetArtifactError {}

/// Validation failures emitted by [`PreviewDriftRecoveryDrillSet::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PreviewDriftRecoveryDrillSetViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required drift event is exercised by no drill.
    RequiredDriftEventMissing,
    /// The set demonstrates no degraded post-drift drill.
    DegradedDrillCaseMissing,
    /// The set demonstrates no clean-recovery drill.
    CleanRecoveryCaseMissing,
    /// A drill is incomplete.
    DrillIncomplete,
    /// A drift silently swapped the bound target.
    TargetKindSilentlySwapped,
    /// A before/after snapshot is internally inconsistent.
    SnapshotInconsistent,
    /// The post-drift snapshot is dishonest for the drift event.
    EventAfterInconsistent,
    /// A degraded drill lacks a precise degraded label or trigger.
    DegradedDrillMissingLabelOrTrigger,
    /// A cleanly recovered drill carries a downgrade trigger or label.
    CleanDrillCarriesDowngrade,
    /// A recovery route is not admissible for the drift event.
    RecoveryRouteNotAdmissible,
    /// A drill's degraded state does not survive reopen/export.
    DriftStateDoesNotSurviveReopen,
    /// A drill lacks evidence refs.
    DrillEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl PreviewDriftRecoveryDrillSetViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredDriftEventMissing => "required_drift_event_missing",
            Self::DegradedDrillCaseMissing => "degraded_drill_case_missing",
            Self::CleanRecoveryCaseMissing => "clean_recovery_case_missing",
            Self::DrillIncomplete => "drill_incomplete",
            Self::TargetKindSilentlySwapped => "target_kind_silently_swapped",
            Self::SnapshotInconsistent => "snapshot_inconsistent",
            Self::EventAfterInconsistent => "event_after_inconsistent",
            Self::DegradedDrillMissingLabelOrTrigger => "degraded_drill_missing_label_or_trigger",
            Self::CleanDrillCarriesDowngrade => "clean_drill_carries_downgrade",
            Self::RecoveryRouteNotAdmissible => "recovery_route_not_admissible",
            Self::DriftStateDoesNotSurviveReopen => "drift_state_does_not_survive_reopen",
            Self::DrillEvidenceMissing => "drill_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in drift-recovery drill set export.
pub fn current_m5_preview_drift_recovery_drill_set_export(
) -> Result<PreviewDriftRecoveryDrillSet, PreviewDriftRecoveryDrillSetArtifactError> {
    let packet: PreviewDriftRecoveryDrillSet = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/preview/m5/preview_drift_recovery_drills/support_export.json"
    )))
    .map_err(PreviewDriftRecoveryDrillSetArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(PreviewDriftRecoveryDrillSetArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &PreviewDriftRecoveryDrillSet,
    violations: &mut Vec<PreviewDriftRecoveryDrillSetViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PREVIEW_DRIFT_RECOVERY_DRILL_SET_SCHEMA_REF,
        PREVIEW_DRIFT_RECOVERY_DRILL_SET_DOC_REF,
        PREVIEW_DRIFT_RECOVERY_DRILL_SET_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(PreviewDriftRecoveryDrillSetViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &PreviewDriftRecoveryDrillSet,
    violations: &mut Vec<PreviewDriftRecoveryDrillSetViolation>,
) {
    let events = packet.represented_events();
    for required in DriftEventClass::ALL {
        if !events.contains(&required) {
            violations.push(PreviewDriftRecoveryDrillSetViolation::RequiredDriftEventMissing);
            break;
        }
    }

    if !packet
        .drills
        .iter()
        .any(|d| d.after_is_degraded() && d.downgrade_consistent())
    {
        violations.push(PreviewDriftRecoveryDrillSetViolation::DegradedDrillCaseMissing);
    }

    if packet.clean_recovery_count() == 0 {
        violations.push(PreviewDriftRecoveryDrillSetViolation::CleanRecoveryCaseMissing);
    }
}

fn validate_drills(
    packet: &PreviewDriftRecoveryDrillSet,
    violations: &mut Vec<PreviewDriftRecoveryDrillSetViolation>,
) {
    for drill in &packet.drills {
        if !drill.is_complete() {
            violations.push(PreviewDriftRecoveryDrillSetViolation::DrillIncomplete);
        }
        if !drill.target_kind_preserved() {
            violations.push(PreviewDriftRecoveryDrillSetViolation::TargetKindSilentlySwapped);
        }
        if !drill.before.is_consistent() || !drill.after.is_consistent() {
            violations.push(PreviewDriftRecoveryDrillSetViolation::SnapshotInconsistent);
        }
        if !drill.event_after_consistent() {
            violations.push(PreviewDriftRecoveryDrillSetViolation::EventAfterInconsistent);
        }
        if drill.after_is_degraded()
            && (drill.downgrade_trigger.is_none()
                || !drill
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations
                .push(PreviewDriftRecoveryDrillSetViolation::DegradedDrillMissingLabelOrTrigger);
        }
        if !drill.after_is_degraded()
            && (drill.downgrade_trigger.is_some() || drill.degraded_label.is_some())
        {
            violations.push(PreviewDriftRecoveryDrillSetViolation::CleanDrillCarriesDowngrade);
        }
        if !drill.recovery_routes_admissible() {
            violations.push(PreviewDriftRecoveryDrillSetViolation::RecoveryRouteNotAdmissible);
        }
        if !drill.survives_reopen_export {
            violations.push(PreviewDriftRecoveryDrillSetViolation::DriftStateDoesNotSurviveReopen);
        }
        if drill.evidence_refs.is_empty() || drill.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(PreviewDriftRecoveryDrillSetViolation::DrillEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &PreviewDriftRecoveryDrillSet,
    violations: &mut Vec<PreviewDriftRecoveryDrillSetViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(PreviewDriftRecoveryDrillSetViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &PreviewDriftRecoveryDrillSet,
    violations: &mut Vec<PreviewDriftRecoveryDrillSetViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(PreviewDriftRecoveryDrillSetViolation::ConsumerProjectionIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "stale"
            | "downgraded"
            | "disconnected"
            | "reconnecting"
    )
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
