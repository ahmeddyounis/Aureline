//! Hot-reload state descriptor.
//!
//! The existing `/schemas/preview/hot_reload_state.schema.json` freezes a
//! six-class underlying vocabulary (applied / partial / restart_required /
//! rebuild_required / failed / unavailable) for the badge projection.
//!
//! This module adds the finer event-class distinction the spec requires —
//! hot reload, fast refresh, reconnect, full restart, and stale-output —
//! so claimed beta rows never collapse them into one generic "live
//! preview" label. The event class is layered *on top of* the underlying
//! six-class state through a projection map enforced by [`HotReloadStateDescriptor::validate`].

use serde::{Deserialize, Serialize};

use super::PreviewOriginFinding;

/// Stable record-kind tag.
pub const HOT_RELOAD_STATE_DESCRIPTOR_RECORD_KIND: &str = "hot_reload_state_descriptor_record";

/// Schema version mirrored by the descriptor block in
/// `/schemas/preview/hot_reload_state.schema.json`.
pub const HOT_RELOAD_STATE_DESCRIPTOR_SCHEMA_VERSION: u32 = 1;

/// Closed event-class vocabulary the descriptor uses. Each value is a
/// distinct state — the chrome MUST surface the token directly rather
/// than collapsing it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotReloadEventClass {
    /// Full hot reload — module graph swap that preserves the document
    /// but discards in-memory component state.
    HotReload,
    /// Fast refresh — finer-grained reload that preserves component
    /// state where the framework adapter declares it safe.
    FastRefresh,
    /// Reconnect — the previous runtime connection dropped and Aureline
    /// re-established a session against the same runtime (no source
    /// change implied).
    Reconnect,
    /// Full restart — the runtime process itself was restarted; all
    /// in-memory state (component, document, side-effect) is gone.
    FullRestart,
    /// Stale output — the runtime did not produce a new frame for the
    /// latest source change. The badge is honest that the view is older
    /// than the source.
    StaleOutput,
    /// Unavailable — the runtime adapter does not support live update at
    /// all. The view is whatever the static render or last capture
    /// produced.
    Unavailable,
}

impl HotReloadEventClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HotReload => "hot_reload",
            Self::FastRefresh => "fast_refresh",
            Self::Reconnect => "reconnect",
            Self::FullRestart => "full_restart",
            Self::StaleOutput => "stale_output",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Closed underlying-state vocabulary mirrored from the existing
/// hot-reload schema. The descriptor stores the underlying value so
/// surfaces that project to the six-class badge stay in sync with the
/// finer event-class label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotReloadUnderlyingStateClass {
    Applied,
    Partial,
    RestartRequired,
    RebuildRequired,
    Failed,
    Unavailable,
}

impl HotReloadUnderlyingStateClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Applied => "applied",
            Self::Partial => "partial",
            Self::RestartRequired => "restart_required",
            Self::RebuildRequired => "rebuild_required",
            Self::Failed => "failed",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Closed recovery-route vocabulary. A subset of the routes frozen by
/// `/schemas/preview/hot_reload_state.schema.json` so the descriptor and
/// the schema speak the same language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HotReloadStateRecoveryRoute {
    NoRecoveryRequiredApplied,
    WaitForPartialToSettle,
    RestartRuntimeRecovery,
    RebuildThenReloadRecovery,
    InspectOnlyWithDiffAgainstSourceRecovery,
    OpenCanonicalSourceRecovery,
    OpenRuntimeLogsRecovery,
    ExportMetadataOnlyRecovery,
    RequestManagedRuntimeRecovery,
    NoRecoveryRequiredStaticPreviewUnavailable,
}

impl HotReloadStateRecoveryRoute {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRecoveryRequiredApplied => "no_recovery_required_applied",
            Self::WaitForPartialToSettle => "wait_for_partial_to_settle",
            Self::RestartRuntimeRecovery => "restart_runtime_recovery",
            Self::RebuildThenReloadRecovery => "rebuild_then_reload_recovery",
            Self::InspectOnlyWithDiffAgainstSourceRecovery => {
                "inspect_only_with_diff_against_source_recovery"
            }
            Self::OpenCanonicalSourceRecovery => "open_canonical_source_recovery",
            Self::OpenRuntimeLogsRecovery => "open_runtime_logs_recovery",
            Self::ExportMetadataOnlyRecovery => "export_metadata_only_recovery",
            Self::RequestManagedRuntimeRecovery => "request_managed_runtime_recovery",
            Self::NoRecoveryRequiredStaticPreviewUnavailable => {
                "no_recovery_required_static_preview_unavailable"
            }
        }
    }
}

/// Canonical hot-reload state descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotReloadStateDescriptor {
    pub record_kind: String,
    pub hot_reload_state_descriptor_schema_version: u32,
    pub hot_reload_state_descriptor_id: String,

    /// Opaque ref to the preview-origin descriptor.
    pub preview_origin_descriptor_ref: String,
    /// Opaque ref to the bound preview_snapshot_record.
    pub preview_snapshot_record_ref: String,

    /// ISO 8601 UTC monotonic timestamp.
    pub observed_at: String,

    pub event_class: HotReloadEventClass,
    pub underlying_state_class: HotReloadUnderlyingStateClass,
    pub recovery_routes: Vec<HotReloadStateRecoveryRoute>,

    /// Whether the most recent event preserved in-memory component state.
    /// FastRefresh defaults to true; HotReload, FullRestart, StaleOutput
    /// default to false; Reconnect is best-effort and may be either.
    pub component_state_preserved: bool,

    /// Reviewer-facing one-sentence summary. Never contains raw stack
    /// frames or stderr.
    pub summary: String,
}

impl HotReloadStateDescriptor {
    pub fn validate(&self) -> Vec<PreviewOriginFinding> {
        let mut findings = Vec::new();
        let subject = self.hot_reload_state_descriptor_id.as_str();

        if self.record_kind != HOT_RELOAD_STATE_DESCRIPTOR_RECORD_KIND {
            findings.push(PreviewOriginFinding::new(
                "hot_reload_state_descriptor.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    HOT_RELOAD_STATE_DESCRIPTOR_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.hot_reload_state_descriptor_schema_version
            != HOT_RELOAD_STATE_DESCRIPTOR_SCHEMA_VERSION
        {
            findings.push(PreviewOriginFinding::new(
                "hot_reload_state_descriptor.schema_version",
                subject,
                format!(
                    "schema_version must be {}, found {}",
                    HOT_RELOAD_STATE_DESCRIPTOR_SCHEMA_VERSION,
                    self.hot_reload_state_descriptor_schema_version
                ),
            ));
        }

        if self.recovery_routes.is_empty() {
            findings.push(PreviewOriginFinding::new(
                "hot_reload_state_descriptor.recovery_routes_not_empty",
                subject,
                "every descriptor must declare at least one recovery route",
            ));
        }

        // Event-class to underlying-state projection map.
        use HotReloadEventClass as E;
        use HotReloadUnderlyingStateClass as U;
        let projection_ok = matches!(
            (self.event_class, self.underlying_state_class),
            (E::HotReload, U::Applied)
                | (E::HotReload, U::Partial)
                | (E::FastRefresh, U::Applied)
                | (E::Reconnect, U::Applied)
                | (E::Reconnect, U::Partial)
                | (E::FullRestart, U::RestartRequired)
                | (E::FullRestart, U::Applied)
                | (E::StaleOutput, U::Partial)
                | (E::StaleOutput, U::Failed)
                | (E::StaleOutput, U::RebuildRequired)
                | (E::Unavailable, U::Unavailable)
        );
        if !projection_ok {
            findings.push(PreviewOriginFinding::new(
                "hot_reload_state_descriptor.projection_map",
                subject,
                format!(
                    "event_class={} cannot project from underlying_state_class={}",
                    self.event_class.as_str(),
                    self.underlying_state_class.as_str(),
                ),
            ));
        }

        // FastRefresh implies preserved component state by definition; if
        // a caller claims FastRefresh but `component_state_preserved =
        // false`, that is dishonest — the event should be HotReload.
        if self.event_class == HotReloadEventClass::FastRefresh && !self.component_state_preserved {
            findings.push(PreviewOriginFinding::new(
                "hot_reload_state_descriptor.fast_refresh_implies_preserved_state",
                subject,
                "fast_refresh implies component_state_preserved = true; emit hot_reload instead",
            ));
        }
        // FullRestart never preserves component state.
        if self.event_class == HotReloadEventClass::FullRestart && self.component_state_preserved {
            findings.push(PreviewOriginFinding::new(
                "hot_reload_state_descriptor.full_restart_forbids_preserved_state",
                subject,
                "full_restart never preserves component state",
            ));
        }
        // Unavailable cannot claim preserved state.
        if self.event_class == HotReloadEventClass::Unavailable && self.component_state_preserved {
            findings.push(PreviewOriginFinding::new(
                "hot_reload_state_descriptor.unavailable_forbids_preserved_state",
                subject,
                "unavailable runtime cannot preserve component state",
            ));
        }

        // Recovery-route gate per event class — mirrors the schema's
        // per-state allowed set.
        let allowed = allowed_recovery_routes_for(self.event_class);
        for route in &self.recovery_routes {
            if !allowed.iter().any(|allowed_route| allowed_route == route) {
                findings.push(PreviewOriginFinding::new(
                    "hot_reload_state_descriptor.recovery_route_for_event",
                    subject,
                    format!(
                        "recovery_route {} not admissible for event_class {}",
                        route.as_str(),
                        self.event_class.as_str(),
                    ),
                ));
            }
        }

        findings
    }

    /// Render a deterministic plaintext summary safe to embed in support
    /// exports.
    pub fn render_plaintext(&self) -> String {
        let routes: Vec<&str> = self.recovery_routes.iter().map(|r| r.as_str()).collect();
        format!(
            "hot_reload_state {id} event={event} underlying={under} preserved={preserved} recovery={routes}: {summary}",
            id = self.hot_reload_state_descriptor_id,
            event = self.event_class.as_str(),
            under = self.underlying_state_class.as_str(),
            preserved = self.component_state_preserved,
            routes = routes.join(","),
            summary = self.summary,
        )
    }
}

fn allowed_recovery_routes_for(
    event: HotReloadEventClass,
) -> &'static [HotReloadStateRecoveryRoute] {
    use HotReloadEventClass as E;
    use HotReloadStateRecoveryRoute as R;
    match event {
        E::HotReload => &[
            R::NoRecoveryRequiredApplied,
            R::WaitForPartialToSettle,
            R::OpenCanonicalSourceRecovery,
            R::InspectOnlyWithDiffAgainstSourceRecovery,
        ],
        E::FastRefresh => &[
            R::NoRecoveryRequiredApplied,
            R::OpenCanonicalSourceRecovery,
            R::InspectOnlyWithDiffAgainstSourceRecovery,
        ],
        E::Reconnect => &[
            R::NoRecoveryRequiredApplied,
            R::WaitForPartialToSettle,
            R::OpenRuntimeLogsRecovery,
            R::OpenCanonicalSourceRecovery,
            R::InspectOnlyWithDiffAgainstSourceRecovery,
            R::RequestManagedRuntimeRecovery,
        ],
        E::FullRestart => &[
            R::RestartRuntimeRecovery,
            R::RebuildThenReloadRecovery,
            R::OpenRuntimeLogsRecovery,
            R::OpenCanonicalSourceRecovery,
            R::InspectOnlyWithDiffAgainstSourceRecovery,
            R::RequestManagedRuntimeRecovery,
        ],
        E::StaleOutput => &[
            R::RestartRuntimeRecovery,
            R::RebuildThenReloadRecovery,
            R::InspectOnlyWithDiffAgainstSourceRecovery,
            R::ExportMetadataOnlyRecovery,
            R::OpenCanonicalSourceRecovery,
            R::OpenRuntimeLogsRecovery,
        ],
        E::Unavailable => &[
            R::NoRecoveryRequiredStaticPreviewUnavailable,
            R::ExportMetadataOnlyRecovery,
            R::OpenCanonicalSourceRecovery,
            R::RequestManagedRuntimeRecovery,
        ],
    }
}
