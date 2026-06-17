//! Environment status strips and one-step "Why this execution context?" entrypoints for every
//! run-capable surface.
//!
//! Where the execution-context resolver ([`crate::execution_context::ExecutionContext`],
//! [`crate::execution_context::ExecutionContextResolver`], and the per-field
//! [`crate::execution_context::ExecutionContextExplanation`]) owns *what the active context is*, this
//! packet governs *where that truth is shown and how a blocked user reaches its inspectable answer*. It
//! is a registry of persistent status strips, one per run-capable M5 surface — the run, test, debug,
//! notebook, request, database, preview, pipeline, and incident surfaces — each carrying the active
//! interpreter / SDK / shell / container / remote-target facets, a one-step explainability entrypoint,
//! and the equivalent CLI / headless object id. It reuses the resolver truth by reference rather than
//! re-deriving any execution context of its own.
//!
//! The readiness analogue here is a fail-closed **status gate**. The guardrail the source set treats as
//! core supportability UX is that a run-capable surface must never present a generic "current target"
//! chip that hides a differing or blocked execution context. Each strip therefore publishes a
//! [`StripPresentation`] that is the weaker of two ceilings: its [`ContextStatusClass`] ceiling
//! (resolved, stale, remote-drift, and conflicting contexts flag the strip; a blocked environment caps
//! it at blocked) and the freshness ceiling of its shown facets (a stale or unknown facet flags the
//! strip). A strip can never claim a cleaner presentation than its inputs support: a stale target, a
//! blocked environment, a drifted remote, or a conflicting context narrows or blocks the strip
//! automatically, surfacing before the downstream run failure rather than after it. The recorded
//! presentation, downgrade reasons, and resolution path are all recomputed and validated against the
//! gate, so a clean chip can never be asserted by hand over a degraded context.
//!
//! Every strip always carries its one-step `explain_entrypoint_ref` and `cli_object_ref` — the
//! inspectable "Why this execution context?" answer and its CLI / headless equivalent — even when the
//! environment is blocked, so a blocked user can still ask where the run would happen and why. Every
//! required consumer surface — the desktop shell, the Support Center, the support export, the
//! issue-report packet, and the CLI / headless reference — binds to this one registry via a
//! [`StripConsumerBinding`] that must ingest it, preserve its status vocabulary and object ids, and
//! narrow with it, so the same status truth and object ids appear across desktop, Support Center,
//! support packets, and CLI without forking the wording.
//!
//! The packet is checked in at `artifacts/support/m5/m5-execution-context-explainability.json` and
//! embedded here. It is metadata-only: every field is a typed state, a count, or an opaque ref, and it
//! carries no credential bodies, raw provider payloads, live target handles, or workspace contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported environment-status-strip schema version.
pub const M5_ENVIRONMENT_STATUS_STRIP_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_ENVIRONMENT_STATUS_STRIP_RECORD_KIND: &str = "m5_environment_status_strips";

/// Repo-relative path to the checked-in packet.
pub const M5_ENVIRONMENT_STATUS_STRIP_PATH: &str =
    "artifacts/support/m5/m5-execution-context-explainability.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_ENVIRONMENT_STATUS_STRIP_SCHEMA_REF: &str =
    "schemas/runtime/m5-environment-status-strip.schema.json";

/// Repo-relative path to the companion document.
pub const M5_ENVIRONMENT_STATUS_STRIP_DOC_REF: &str =
    "docs/help/support/m5-why-this-execution-context.md";

/// Repo-relative path to the human-readable reviewer artifact.
pub const M5_ENVIRONMENT_STATUS_STRIP_ARTIFACT_DOC_REF: &str =
    "artifacts/support/m5/m5-execution-context-explainability.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_ENVIRONMENT_STATUS_STRIP_FIXTURE_DIR: &str =
    "fixtures/runtime/m5/m5-environment-status-strips";

/// Repo-relative path to the shiproom review packet that renders this registry.
pub const M5_ENVIRONMENT_STATUS_STRIP_REVIEW_PACKET_REF: &str =
    "artifacts/shiproom/m5-execution-context-explainability-review-packet/execution_context_explainability_review_packet.md";

/// Embedded checked-in packet JSON.
pub const M5_ENVIRONMENT_STATUS_STRIP_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/support/m5/m5-execution-context-explainability.json"
));

/// A run-capable M5 surface that must carry a persistent environment/target status strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunSurface {
    /// The run / launch surface.
    Run,
    /// The test explorer / inline-results surface.
    Test,
    /// The debug session surface.
    Debug,
    /// The notebook / kernel surface.
    Notebook,
    /// The API request / operation surface.
    Request,
    /// The database / statement surface.
    Database,
    /// The preview / runtime surface.
    Preview,
    /// The pipeline / build-and-test surface.
    Pipeline,
    /// The incident / recovery surface.
    Incident,
}

impl RunSurface {
    /// Every run-capable surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Run,
        Self::Test,
        Self::Debug,
        Self::Notebook,
        Self::Request,
        Self::Database,
        Self::Preview,
        Self::Pipeline,
        Self::Incident,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Test => "test",
            Self::Debug => "debug",
            Self::Notebook => "notebook",
            Self::Request => "request",
            Self::Database => "database",
            Self::Preview => "preview",
            Self::Pipeline => "pipeline",
            Self::Incident => "incident",
        }
    }
}

/// A facet of the active execution context a status strip surfaces.
///
/// These are the selectable parts of "where this runs" the goal names — the interpreter, SDK /
/// toolchain, shell, container, and remote target — each projected from the resolver truth rather than
/// re-derived here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextFacet {
    /// The active interpreter (e.g. the Python interpreter or Node runtime).
    Interpreter,
    /// The active SDK / toolchain.
    Sdk,
    /// The active shell.
    Shell,
    /// The active container / devcontainer.
    Container,
    /// The active remote / helper target.
    RemoteTarget,
}

impl ContextFacet {
    /// Every context facet, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Interpreter,
        Self::Sdk,
        Self::Shell,
        Self::Container,
        Self::RemoteTarget,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Interpreter => "interpreter",
            Self::Sdk => "sdk",
            Self::Shell => "shell",
            Self::Container => "container",
            Self::RemoteTarget => "remote_target",
        }
    }
}

/// How fresh a strip facet's projected value is relative to the live resolver truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextFreshness {
    /// The facet is current with the resolver truth.
    Fresh,
    /// The facet is known but stale; caps the strip at flagged.
    Stale,
    /// The facet's freshness cannot be determined; caps the strip at flagged.
    Unknown,
}

impl ContextFreshness {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Fresh, Self::Stale, Self::Unknown];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
        }
    }

    /// Confidence rank; higher is fresher. Used to pick the least-fresh facet.
    const fn rank(self) -> u8 {
        match self {
            Self::Fresh => 2,
            Self::Stale => 1,
            Self::Unknown => 0,
        }
    }

    /// Whether the facet is current with the resolver truth.
    pub const fn is_fresh(self) -> bool {
        matches!(self, Self::Fresh)
    }

    /// Highest presentation this freshness permits a strip that shows the facet.
    pub const fn presentation_ceiling(self) -> StripPresentation {
        match self {
            Self::Fresh => StripPresentation::Resolved,
            Self::Stale | Self::Unknown => StripPresentation::Flagged,
        }
    }
}

/// The overall state of a surface's execution context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextStatusClass {
    /// The context resolved cleanly and is current.
    Resolved,
    /// The context resolved but is stale; the strip flags a refresh.
    Stale,
    /// The environment is blocked; the run will fail until it is unblocked.
    Blocked,
    /// The remote / helper target has drifted from the recorded context.
    RemoteDrift,
    /// Two execution contexts conflict and must be reconciled.
    Conflicting,
}

impl ContextStatusClass {
    /// Every status class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Resolved,
        Self::Stale,
        Self::Blocked,
        Self::RemoteDrift,
        Self::Conflicting,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::Stale => "stale",
            Self::Blocked => "blocked",
            Self::RemoteDrift => "remote_drift",
            Self::Conflicting => "conflicting",
        }
    }

    /// Highest presentation this status permits.
    ///
    /// A blocked environment caps the strip at blocked; a stale, drifted, or conflicting context flags
    /// it; only a clean resolution permits a resolved strip.
    pub const fn presentation_ceiling(self) -> StripPresentation {
        match self {
            Self::Resolved => StripPresentation::Resolved,
            Self::Stale | Self::RemoteDrift | Self::Conflicting => StripPresentation::Flagged,
            Self::Blocked => StripPresentation::Blocked,
        }
    }

    /// The status-driven downgrade reason, if any.
    const fn downgrade_reason(self) -> Option<StripDowngradeReason> {
        match self {
            Self::Resolved => None,
            Self::Stale => Some(StripDowngradeReason::StaleContext),
            Self::Blocked => Some(StripDowngradeReason::BlockedEnvironment),
            Self::RemoteDrift => Some(StripDowngradeReason::RemoteDrift),
            Self::Conflicting => Some(StripDowngradeReason::ConflictingContext),
        }
    }
}

/// The presentation the status gate publishes for a strip, highest to lowest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StripPresentation {
    /// The strip shows a clean, current execution context.
    Resolved,
    /// The strip is shown but flagged; the context is stale, drifted, or conflicting.
    Flagged,
    /// The strip is shown with a blocked badge; the run will fail until the environment is unblocked.
    Blocked,
}

impl StripPresentation {
    /// Every presentation, highest to lowest.
    pub const ALL: [Self; 3] = [Self::Resolved, Self::Flagged, Self::Blocked];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::Flagged => "flagged",
            Self::Blocked => "blocked",
        }
    }

    /// Rank for the fail-closed gate; higher is more capable.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Resolved => 2,
            Self::Flagged => 1,
            Self::Blocked => 0,
        }
    }

    /// Whether the gate flagged or blocked the strip below a clean resolution.
    pub const fn requires_attention(self) -> bool {
        !matches!(self, Self::Resolved)
    }

    /// Whether the strip must warn before a downstream run failure.
    pub const fn warns_before_run(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// The weaker (lower-rank) of two presentations.
fn weaker(a: StripPresentation, b: StripPresentation) -> StripPresentation {
    if b.rank() < a.rank() {
        b
    } else {
        a
    }
}

/// A headline reason the status gate flags or blocks a strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StripDowngradeReason {
    /// The context is stale, or a shown facet is stale or unknown.
    StaleContext,
    /// The environment is blocked.
    BlockedEnvironment,
    /// The remote / helper target has drifted.
    RemoteDrift,
    /// Two execution contexts conflict.
    ConflictingContext,
}

impl StripDowngradeReason {
    /// Every downgrade reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StaleContext,
        Self::BlockedEnvironment,
        Self::RemoteDrift,
        Self::ConflictingContext,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleContext => "stale_context",
            Self::BlockedEnvironment => "blocked_environment",
            Self::RemoteDrift => "remote_drift",
            Self::ConflictingContext => "conflicting_context",
        }
    }
}

/// The resolution path surfaced when a strip is flagged or blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextResolutionPath {
    /// Refresh the stale interpreter / toolchain / target selection.
    RefreshTarget,
    /// Reconnect or re-resolve the drifted remote / helper target.
    ReconnectRemote,
    /// Reconcile the conflicting execution contexts.
    ResolveConflict,
    /// Unblock the blocked environment.
    UnblockEnvironment,
    /// No resolution is needed; only valid when the strip resolves cleanly.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl ContextResolutionPath {
    /// Every resolution path, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RefreshTarget,
        Self::ReconnectRemote,
        Self::ResolveConflict,
        Self::UnblockEnvironment,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshTarget => "refresh_target",
            Self::ReconnectRemote => "reconnect_remote",
            Self::ResolveConflict => "resolve_conflict",
            Self::UnblockEnvironment => "unblock_environment",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real resolution path the user can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// A downstream surface that must ingest this registry and narrow with it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StripConsumerSurface {
    /// The desktop shell's status items on each run-capable surface.
    DesktopShell,
    /// The Support Center's environment / execution-context views.
    SupportCenter,
    /// The support export of the execution context.
    SupportExport,
    /// The issue-report / crash-intake packet.
    IssueReportPacket,
    /// The CLI / headless execution-context reference.
    CliHeadless,
}

impl StripConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::DesktopShell,
        Self::SupportCenter,
        Self::SupportExport,
        Self::IssueReportPacket,
        Self::CliHeadless,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopShell => "desktop_shell",
            Self::SupportCenter => "support_center",
            Self::SupportExport => "support_export",
            Self::IssueReportPacket => "issue_report_packet",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// One execution-context facet shown on a strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StripFacet {
    /// Facet this row surfaces.
    pub facet: ContextFacet,
    /// Human-readable value label (e.g. "Python 3.12 (.venv)"); carries no credential body.
    pub value_label: String,
    /// How fresh the value is relative to the resolver truth.
    pub freshness: ContextFreshness,
    /// Ref to the resolver truth this facet projects.
    pub descriptor_ref: String,
}

impl StripFacet {
    /// Whether the facet carries the non-empty value label and descriptor ref it requires.
    pub fn is_well_formed(&self) -> bool {
        !self.value_label.trim().is_empty() && !self.descriptor_ref.trim().is_empty()
    }

    /// Whether the facet is current with the resolver truth.
    pub fn is_current(&self) -> bool {
        self.freshness.is_fresh()
    }
}

/// One run-capable surface's persistent environment/target status strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentStatusStrip {
    /// Stable strip id.
    pub strip_id: String,
    /// Run-capable surface this strip lives on.
    pub surface: RunSurface,
    /// Human-readable status-item label shown in the strip.
    pub status_item_label: String,
    /// Ref to the persistent status-item placement on the surface.
    pub placement_ref: String,
    /// Ref to the resolved execution-context object this strip projects.
    pub context_ref: String,
    /// One-step "Why this execution context?" entrypoint; always present.
    pub explain_entrypoint_ref: String,
    /// The equivalent CLI / headless execution-context object id; always present.
    pub cli_object_ref: String,
    /// Execution-context facets shown on this strip; at least one.
    #[serde(default)]
    pub shown_facets: Vec<StripFacet>,
    /// Overall context status for this surface.
    pub status: ContextStatusClass,
    /// Presentation actually published after the gate; must equal the recomputed decision.
    pub presentation: StripPresentation,
    /// Headline downgrade reasons; must equal the recomputed set.
    #[serde(default)]
    pub downgrade_reasons: Vec<StripDowngradeReason>,
    /// Resolution path surfaced when the strip is flagged or blocked.
    pub resolution_path: ContextResolutionPath,
    /// True when the strip warns before a downstream run failure; required iff blocked.
    pub blocked_before_run: bool,
    /// Caveats attached to a flagged or blocked strip.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Facets or context fields that are stale, blocked, drifted, or conflicting.
    #[serde(default)]
    pub unmet_or_stale_fields: Vec<String>,
    /// Ref to the conformance suite backing the strip.
    pub conformance_ref: String,
    /// Ref to the strip's supporting evidence.
    pub evidence_ref: String,
    /// Active scope snapshot the strip answered, stamped for replay.
    pub scope_snapshot_ref: String,
    /// Ref to the machine-readable strip receipt.
    pub strip_receipt_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl EnvironmentStatusStrip {
    /// The least-fresh state across the shown facets, treating no facets as unknown.
    pub fn worst_facet_freshness(&self) -> ContextFreshness {
        self.shown_facets
            .iter()
            .map(|f| f.freshness)
            .min_by_key(|f| f.rank())
            .unwrap_or(ContextFreshness::Unknown)
    }

    /// Whether every shown facet is current with the resolver truth.
    pub fn all_facets_current(&self) -> bool {
        self.shown_facets.iter().all(StripFacet::is_current)
    }

    /// Highest presentation the shown facets' freshness permits.
    pub fn facet_freshness_ceiling(&self) -> StripPresentation {
        self.worst_facet_freshness().presentation_ceiling()
    }

    /// The presentation the gate permits this strip to publish.
    ///
    /// Lowers the clean baseline to the weaker of the status ceiling and the facet-freshness ceiling,
    /// so a blocked environment, a drifted remote, or a stale facet can never present a fuller claim
    /// than the inputs support.
    pub fn effective_presentation(&self) -> StripPresentation {
        weaker(
            self.status.presentation_ceiling(),
            self.facet_freshness_ceiling(),
        )
    }

    /// Whether the context is stale, or a shown facet is stale or unknown.
    pub fn has_stale_context(&self) -> bool {
        self.status == ContextStatusClass::Stale || !self.all_facets_current()
    }

    /// The headline downgrade reasons recomputed from the strip's observed states.
    pub fn computed_downgrade_reasons(&self) -> Vec<StripDowngradeReason> {
        StripDowngradeReason::ALL
            .into_iter()
            .filter(|reason| match reason {
                StripDowngradeReason::StaleContext => self.has_stale_context(),
                other => self.status.downgrade_reason() == Some(*other),
            })
            .collect()
    }

    /// The resolution path the gate must record, derived from the strip's observed states.
    ///
    /// A blocked environment is the hardest state, so it points at an unblock before a remote, conflict,
    /// or refresh path.
    pub fn computed_resolution_path(&self) -> ContextResolutionPath {
        match self.status {
            ContextStatusClass::Blocked => ContextResolutionPath::UnblockEnvironment,
            ContextStatusClass::Conflicting => ContextResolutionPath::ResolveConflict,
            ContextStatusClass::RemoteDrift => ContextResolutionPath::ReconnectRemote,
            ContextStatusClass::Stale => ContextResolutionPath::RefreshTarget,
            ContextStatusClass::Resolved => {
                if self.all_facets_current() {
                    ContextResolutionPath::NoneNeeded
                } else {
                    ContextResolutionPath::RefreshTarget
                }
            }
        }
    }

    /// Whether the strip resolves cleanly with nothing flagging it.
    pub fn is_resolved(&self) -> bool {
        self.effective_presentation() == StripPresentation::Resolved
    }

    /// Whether the strip carries its own non-empty one-step explain and CLI-equivalent refs.
    pub fn has_one_step_explainability(&self) -> bool {
        !self.explain_entrypoint_ref.trim().is_empty() && !self.cli_object_ref.trim().is_empty()
    }

    /// Whether the strip carries its own non-empty conformance, evidence, scope, and receipt refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.conformance_ref.trim().is_empty()
            && !self.evidence_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
            && !self.strip_receipt_ref.trim().is_empty()
    }

    /// Whether the recorded presentation, reasons, path, and blocked-before-run flag agree with the
    /// gate.
    pub fn gate_consistent(&self) -> bool {
        let effective = self.effective_presentation();
        self.presentation == effective
            && self.downgrade_reasons == self.computed_downgrade_reasons()
            && self.resolution_path == self.computed_resolution_path()
            && self.blocked_before_run == effective.warns_before_run()
    }
}

/// One binding wiring a downstream surface to this registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StripConsumerBinding {
    /// Consumer surface this binding wires.
    pub consumer_surface: StripConsumerSurface,
    /// Stable binding ref.
    pub binding_ref: String,
    /// Packet id this surface ingests.
    pub packet_id_ref: String,
    /// Active scope snapshot stamped on the binding for replay.
    pub scope_snapshot_ref: String,
    /// True when the surface ingests this registry rather than a parallel list.
    pub ingests_registry: bool,
    /// True when the surface preserves the status vocabulary (status, presentation, reasons) verbatim.
    pub preserves_status_vocabulary: bool,
    /// True when the surface preserves the strip and CLI object ids rather than reminting them.
    pub preserves_object_ids: bool,
    /// True when the surface narrows automatically as strips are flagged or blocked.
    pub narrows_on_downgrade: bool,
    /// True when raw private material is excluded from the binding.
    pub raw_private_material_excluded: bool,
}

impl StripConsumerBinding {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && self.ingests_registry
            && self.preserves_status_vocabulary
            && self.preserves_object_ids
            && self.narrows_on_downgrade
            && self.raw_private_material_excluded
            && !self.binding_ref.trim().is_empty()
            && !self.scope_snapshot_ref.trim().is_empty()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5EnvironmentStatusStripSummary {
    /// Total strips.
    pub total_strips: usize,
    /// Strips that resolve cleanly.
    pub resolved_strips: usize,
    /// Strips the gate flagged.
    pub flagged_strips: usize,
    /// Strips the gate blocked.
    pub blocked_strips: usize,
    /// Strips carrying at least one downgrade reason.
    pub strips_with_downgrade_reasons: usize,
    /// Strips whose context is stale or whose shown facets are not all current.
    pub stale_strips: usize,
    /// Strips that warn before a downstream run failure.
    pub blocked_before_run_strips: usize,
    /// Distinct context facets shown.
    pub facets_shown: usize,
    /// Distinct run surfaces covered.
    pub surfaces_covered: usize,
}

/// A redaction-safe export row projected from a strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvironmentStatusStripExportRow {
    /// Strip id.
    pub strip_id: String,
    /// Surface token.
    pub surface: String,
    /// Status-item label.
    pub status_item_label: String,
    /// Context-ref this strip projects.
    pub context_ref: String,
    /// One-step explain entrypoint ref.
    pub explain_entrypoint_ref: String,
    /// CLI / headless equivalent object id.
    pub cli_object_ref: String,
    /// Shown-facet tokens.
    pub shown_facets: Vec<String>,
    /// Status token.
    pub status: String,
    /// Published-presentation token.
    pub presentation: String,
    /// Downgrade-reason tokens.
    pub downgrade_reasons: Vec<String>,
    /// Resolution-path token.
    pub resolution_path: String,
    /// Whether the strip warns before a downstream run failure.
    pub blocked_before_run: bool,
    /// Caveats attached to the strip.
    pub caveats: Vec<String>,
    /// Facets or context fields that are stale, blocked, drifted, or conflicting.
    pub unmet_or_stale_fields: Vec<String>,
    /// Scope snapshot the strip answered.
    pub scope_snapshot_ref: String,
    /// Strip-receipt ref.
    pub strip_receipt_ref: String,
    /// Whether the strip resolves cleanly.
    pub resolved: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the registry — the canonical execution-context index
/// downstream surfaces render instead of restating each surface's status by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvironmentStatusStripExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub rows: Vec<M5EnvironmentStatusStripExportRow>,
    /// Whether every strip's published presentation and decision agree with the gate.
    pub all_strips_gate_consistent: bool,
    /// Strips that resolve cleanly.
    pub resolved_count: usize,
    /// Strips the gate flagged.
    pub flagged_count: usize,
    /// Strips the gate blocked entirely.
    pub blocked_count: usize,
}

/// The typed environment-status-strip registry packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5EnvironmentStatusStrips {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed run-surface vocabulary.
    pub surfaces: Vec<RunSurface>,
    /// Closed context-facet vocabulary.
    pub context_facets: Vec<ContextFacet>,
    /// Closed freshness vocabulary.
    pub freshness_states: Vec<ContextFreshness>,
    /// Closed status-class vocabulary.
    pub status_classes: Vec<ContextStatusClass>,
    /// Closed presentation vocabulary.
    pub presentations: Vec<StripPresentation>,
    /// Closed downgrade-reason vocabulary.
    pub downgrade_reasons: Vec<StripDowngradeReason>,
    /// Closed resolution-path vocabulary.
    pub resolution_paths: Vec<ContextResolutionPath>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<StripConsumerSurface>,
    /// Strips, one per run-capable surface.
    #[serde(default)]
    pub strips: Vec<EnvironmentStatusStrip>,
    /// Consumer bindings, one per required surface.
    #[serde(default)]
    pub consumer_bindings: Vec<StripConsumerBinding>,
    /// Summary counts.
    pub summary: M5EnvironmentStatusStripSummary,
}

impl M5EnvironmentStatusStrips {
    /// Returns the strip for the given surface.
    pub fn strip_for(&self, surface: RunSurface) -> Option<&EnvironmentStatusStrip> {
        self.strips.iter().find(|s| s.surface == surface)
    }

    /// Returns the strip with the given id.
    pub fn strip(&self, strip_id: &str) -> Option<&EnvironmentStatusStrip> {
        self.strips.iter().find(|s| s.strip_id == strip_id)
    }

    /// Strips that resolve cleanly.
    pub fn resolved_strips(&self) -> impl Iterator<Item = &EnvironmentStatusStrip> {
        self.strips
            .iter()
            .filter(|s| s.effective_presentation() == StripPresentation::Resolved)
    }

    /// Strips the gate flagged.
    pub fn flagged_strips(&self) -> impl Iterator<Item = &EnvironmentStatusStrip> {
        self.strips
            .iter()
            .filter(|s| s.effective_presentation() == StripPresentation::Flagged)
    }

    /// Strips the gate blocked entirely.
    pub fn blocked_strips(&self) -> impl Iterator<Item = &EnvironmentStatusStrip> {
        self.strips
            .iter()
            .filter(|s| s.effective_presentation() == StripPresentation::Blocked)
    }

    /// Whether a consumer binding preserves this registry for the given surface.
    pub fn has_binding_for(&self, surface: StripConsumerSurface) -> bool {
        self.consumer_bindings
            .iter()
            .any(|b| b.consumer_surface == surface && b.preserves_truth_for(&self.packet_id))
    }

    /// Whether every strip's recorded presentation, reasons, path, and blocked flag agree with the
    /// gate.
    pub fn all_strips_gate_consistent(&self) -> bool {
        self.strips
            .iter()
            .all(EnvironmentStatusStrip::gate_consistent)
    }

    /// Recomputes the summary block from the strips.
    pub fn computed_summary(&self) -> M5EnvironmentStatusStripSummary {
        let count_presentation = |decision: StripPresentation| {
            self.strips
                .iter()
                .filter(|s| s.effective_presentation() == decision)
                .count()
        };
        let mut facets = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        for strip in &self.strips {
            surfaces.insert(strip.surface);
            for facet in &strip.shown_facets {
                facets.insert(facet.facet);
            }
        }
        M5EnvironmentStatusStripSummary {
            total_strips: self.strips.len(),
            resolved_strips: count_presentation(StripPresentation::Resolved),
            flagged_strips: count_presentation(StripPresentation::Flagged),
            blocked_strips: count_presentation(StripPresentation::Blocked),
            strips_with_downgrade_reasons: self
                .strips
                .iter()
                .filter(|s| !s.downgrade_reasons.is_empty())
                .count(),
            stale_strips: self.strips.iter().filter(|s| s.has_stale_context()).count(),
            blocked_before_run_strips: self.strips.iter().filter(|s| s.blocked_before_run).count(),
            facets_shown: facets.len(),
            surfaces_covered: surfaces.len(),
        }
    }

    /// Produces the execution-context index downstream surfaces render instead of restating each
    /// surface's status by hand.
    pub fn export_projection(&self) -> M5EnvironmentStatusStripExportProjection {
        let rows = self
            .strips
            .iter()
            .map(|s| M5EnvironmentStatusStripExportRow {
                strip_id: s.strip_id.clone(),
                surface: s.surface.as_str().to_owned(),
                status_item_label: s.status_item_label.clone(),
                context_ref: s.context_ref.clone(),
                explain_entrypoint_ref: s.explain_entrypoint_ref.clone(),
                cli_object_ref: s.cli_object_ref.clone(),
                shown_facets: s
                    .shown_facets
                    .iter()
                    .map(|f| f.facet.as_str().to_owned())
                    .collect(),
                status: s.status.as_str().to_owned(),
                presentation: s.presentation.as_str().to_owned(),
                downgrade_reasons: s
                    .downgrade_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                resolution_path: s.resolution_path.as_str().to_owned(),
                blocked_before_run: s.blocked_before_run,
                caveats: s.caveats.clone(),
                unmet_or_stale_fields: s.unmet_or_stale_fields.clone(),
                scope_snapshot_ref: s.scope_snapshot_ref.clone(),
                strip_receipt_ref: s.strip_receipt_ref.clone(),
                resolved: s.is_resolved(),
                summary: format!(
                    "{}: status {}, presentation {}, resolution {}",
                    s.surface.as_str(),
                    s.status.as_str(),
                    s.presentation.as_str(),
                    s.resolution_path.as_str()
                ),
            })
            .collect();
        M5EnvironmentStatusStripExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            rows,
            all_strips_gate_consistent: self.all_strips_gate_consistent(),
            resolved_count: self.resolved_strips().count(),
            flagged_count: self.flagged_strips().count(),
            blocked_count: self.blocked_strips().count(),
        }
    }

    /// Builds an export-safe support packet preserving the exact strip registry.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> M5EnvironmentStatusStripSupportExport {
        M5EnvironmentStatusStripSupportExport {
            record_kind: M5_ENVIRONMENT_STATUS_STRIP_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_ENVIRONMENT_STATUS_STRIP_SCHEMA_VERSION,
            export_id: export_id.into(),
            packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            registry: self.clone(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5EnvironmentStatusStripViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        for strip in &self.strips {
            if !seen_ids.insert(strip.strip_id.clone()) {
                violations.push(M5EnvironmentStatusStripViolation::DuplicateStrip {
                    strip_id: strip.strip_id.clone(),
                });
            }
            if !seen_surfaces.insert(strip.surface) {
                violations.push(M5EnvironmentStatusStripViolation::DuplicateSurface {
                    surface: strip.surface.as_str(),
                });
            }
            self.validate_strip(strip, &mut violations);
        }

        // Every run-capable surface must carry exactly one strip, so the registry the desktop shell,
        // Support Center, CLI/headless, support export, and issue-report packets all read is the same
        // one and complete.
        for surface in RunSurface::ALL {
            if !seen_surfaces.contains(&surface) {
                violations.push(M5EnvironmentStatusStripViolation::MissingSurface {
                    surface: surface.as_str(),
                });
            }
        }

        for surface in StripConsumerSurface::REQUIRED {
            if !self.has_binding_for(surface) {
                violations.push(M5EnvironmentStatusStripViolation::MissingConsumerBinding {
                    surface: surface.as_str(),
                });
            }
        }
        for binding in &self.consumer_bindings {
            if !binding.preserves_truth_for(&self.packet_id) {
                violations.push(M5EnvironmentStatusStripViolation::ConsumerBindingDrift {
                    binding_ref: binding.binding_ref.clone(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5EnvironmentStatusStripViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5EnvironmentStatusStripViolation>) {
        if self.schema_version != M5_ENVIRONMENT_STATUS_STRIP_SCHEMA_VERSION {
            violations.push(
                M5EnvironmentStatusStripViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_ENVIRONMENT_STATUS_STRIP_RECORD_KIND {
            violations.push(M5EnvironmentStatusStripViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5EnvironmentStatusStripViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            ("surfaces", self.surfaces == RunSurface::ALL.to_vec()),
            (
                "context_facets",
                self.context_facets == ContextFacet::ALL.to_vec(),
            ),
            (
                "freshness_states",
                self.freshness_states == ContextFreshness::ALL.to_vec(),
            ),
            (
                "status_classes",
                self.status_classes == ContextStatusClass::ALL.to_vec(),
            ),
            (
                "presentations",
                self.presentations == StripPresentation::ALL.to_vec(),
            ),
            (
                "downgrade_reasons",
                self.downgrade_reasons == StripDowngradeReason::ALL.to_vec(),
            ),
            (
                "resolution_paths",
                self.resolution_paths == ContextResolutionPath::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == StripConsumerSurface::REQUIRED.to_vec(),
            ),
        ] {
            if !ok {
                violations
                    .push(M5EnvironmentStatusStripViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_strip(
        &self,
        strip: &EnvironmentStatusStrip,
        violations: &mut Vec<M5EnvironmentStatusStripViolation>,
    ) {
        for (field, value) in [
            ("strip_id", &strip.strip_id),
            ("status_item_label", &strip.status_item_label),
            ("placement_ref", &strip.placement_ref),
            ("context_ref", &strip.context_ref),
            ("explain_entrypoint_ref", &strip.explain_entrypoint_ref),
            ("cli_object_ref", &strip.cli_object_ref),
            ("conformance_ref", &strip.conformance_ref),
            ("evidence_ref", &strip.evidence_ref),
            ("scope_snapshot_ref", &strip.scope_snapshot_ref),
            ("strip_receipt_ref", &strip.strip_receipt_ref),
            ("note", &strip.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5EnvironmentStatusStripViolation::EmptyField {
                    id: strip.strip_id.clone(),
                    field_name: field,
                });
            }
        }

        // Every strip must carry its one-step explainability entry and its CLI/headless equivalent, so
        // a blocked user can always ask where the run happens and why — even when the environment is
        // blocked.
        if !strip.has_one_step_explainability() {
            violations.push(
                M5EnvironmentStatusStripViolation::MissingOneStepExplainability {
                    strip_id: strip.strip_id.clone(),
                },
            );
        }

        // Every strip must show at least one execution-context facet, so it can never collapse into a
        // generic "current target" chip that hides which interpreter / SDK / shell / container / remote
        // target won.
        if strip.shown_facets.is_empty() {
            violations.push(M5EnvironmentStatusStripViolation::NoShownFacets {
                strip_id: strip.strip_id.clone(),
            });
        }
        let mut seen_facets = BTreeSet::new();
        for facet in &strip.shown_facets {
            if !seen_facets.insert(facet.facet) {
                violations.push(M5EnvironmentStatusStripViolation::DuplicateShownFacet {
                    strip_id: strip.strip_id.clone(),
                    facet: facet.facet.as_str(),
                });
            }
            if !facet.is_well_formed() {
                violations.push(M5EnvironmentStatusStripViolation::FacetIncomplete {
                    strip_id: strip.strip_id.clone(),
                    facet: facet.facet.as_str(),
                });
            }
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &strip.downgrade_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(
                    M5EnvironmentStatusStripViolation::DuplicateDowngradeReason {
                        strip_id: strip.strip_id.clone(),
                        reason: reason.as_str(),
                    },
                );
            }
        }

        // The published presentation must equal the gate's recomputed decision, so a stale, blocked,
        // drifted, or conflicting context can never read as a clean current-target chip.
        let effective = strip.effective_presentation();
        if strip.presentation != effective {
            violations.push(M5EnvironmentStatusStripViolation::OverstatedPresentation {
                strip_id: strip.strip_id.clone(),
                published: strip.presentation.as_str(),
                computed: effective.as_str(),
            });
        }

        let computed_reasons = strip.computed_downgrade_reasons();
        if strip.downgrade_reasons != computed_reasons {
            violations.push(
                M5EnvironmentStatusStripViolation::DowngradeReasonsMismatch {
                    strip_id: strip.strip_id.clone(),
                },
            );
        }

        let computed_path = strip.computed_resolution_path();
        if strip.resolution_path != computed_path {
            violations.push(M5EnvironmentStatusStripViolation::ResolutionPathMismatch {
                strip_id: strip.strip_id.clone(),
                declared: strip.resolution_path.as_str(),
                required: computed_path.as_str(),
            });
        }

        // A blocked strip must warn before the downstream run failure, and a non-blocked strip must
        // not claim it does, so the blocked state becomes visible at the locus of work rather than at
        // launch time.
        if strip.blocked_before_run != effective.warns_before_run() {
            violations.push(
                M5EnvironmentStatusStripViolation::BlockedBeforeRunMismatch {
                    strip_id: strip.strip_id.clone(),
                },
            );
        }

        // A flagged or blocked strip must name a real resolution path, a caveat, and the stale-or-blocked
        // field driving the downgrade, so a narrowing never drops its remediation or hides its cause.
        if effective.requires_attention() {
            if !strip.resolution_path.is_offered() {
                violations.push(M5EnvironmentStatusStripViolation::MissingResolutionPath {
                    strip_id: strip.strip_id.clone(),
                });
            }
            if strip.caveats.is_empty() {
                violations.push(M5EnvironmentStatusStripViolation::EmptyField {
                    id: strip.strip_id.clone(),
                    field_name: "caveats",
                });
            }
            if strip.unmet_or_stale_fields.is_empty() {
                violations.push(M5EnvironmentStatusStripViolation::EmptyField {
                    id: strip.strip_id.clone(),
                    field_name: "unmet_or_stale_fields",
                });
            }
        }

        // A cleanly resolved strip must be genuinely whole: every shown facet current, status resolved,
        // and nothing flagging it.
        if effective == StripPresentation::Resolved
            && (!strip.all_facets_current()
                || strip.status != ContextStatusClass::Resolved
                || !strip.downgrade_reasons.is_empty()
                || !strip.caveats.is_empty()
                || !strip.unmet_or_stale_fields.is_empty()
                || strip.resolution_path.is_offered()
                || strip.blocked_before_run)
        {
            violations.push(M5EnvironmentStatusStripViolation::ResolvedStripNotWhole {
                strip_id: strip.strip_id.clone(),
            });
        }
    }
}

/// A validation violation for the environment-status-strip registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5EnvironmentStatusStripViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Strip or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A strip id appears more than once.
    DuplicateStrip {
        /// Duplicate strip id.
        strip_id: String,
    },
    /// A surface appears in more than one strip.
    DuplicateSurface {
        /// Surface token.
        surface: &'static str,
    },
    /// A run-capable surface has no strip.
    MissingSurface {
        /// Surface token.
        surface: &'static str,
    },
    /// A strip is missing its one-step explain entry or CLI-equivalent object id.
    MissingOneStepExplainability {
        /// Strip id.
        strip_id: String,
    },
    /// A strip shows no execution-context facet.
    NoShownFacets {
        /// Strip id.
        strip_id: String,
    },
    /// A strip shows the same facet more than once.
    DuplicateShownFacet {
        /// Strip id.
        strip_id: String,
        /// Facet token.
        facet: &'static str,
    },
    /// A shown facet is missing its value label or descriptor ref.
    FacetIncomplete {
        /// Strip id.
        strip_id: String,
        /// Facet token.
        facet: &'static str,
    },
    /// A strip lists a downgrade reason more than once.
    DuplicateDowngradeReason {
        /// Strip id.
        strip_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A strip publishes a presentation cleaner than the gate computes.
    OverstatedPresentation {
        /// Strip id.
        strip_id: String,
        /// Published presentation token.
        published: &'static str,
        /// Computed effective presentation token.
        computed: &'static str,
    },
    /// A strip's downgrade reasons disagree with the recomputed reasons.
    DowngradeReasonsMismatch {
        /// Strip id.
        strip_id: String,
    },
    /// A strip's resolution path disagrees with the recomputed path.
    ResolutionPathMismatch {
        /// Strip id.
        strip_id: String,
        /// Declared path token.
        declared: &'static str,
        /// Required path token.
        required: &'static str,
    },
    /// A strip's blocked-before-run flag disagrees with the gate.
    BlockedBeforeRunMismatch {
        /// Strip id.
        strip_id: String,
    },
    /// A flagged or blocked strip offers no resolution path.
    MissingResolutionPath {
        /// Strip id.
        strip_id: String,
    },
    /// A strip resolves cleanly but flags a state or carries a reason.
    ResolvedStripNotWhole {
        /// Strip id.
        strip_id: String,
    },
    /// A required consumer surface has no binding.
    MissingConsumerBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A consumer binding drops or remints registry truth.
    ConsumerBindingDrift {
        /// Binding ref.
        binding_ref: String,
    },
    /// The summary counts disagree with the strips.
    SummaryMismatch,
}

impl fmt::Display for M5EnvironmentStatusStripViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateStrip { strip_id } => write!(f, "duplicate strip id {strip_id}"),
            Self::DuplicateSurface { surface } => {
                write!(f, "surface {surface} has more than one strip")
            }
            Self::MissingSurface { surface } => write!(f, "missing strip for surface {surface}"),
            Self::MissingOneStepExplainability { strip_id } => write!(
                f,
                "strip {strip_id} is missing its one-step explain entry or CLI-equivalent object id"
            ),
            Self::NoShownFacets { strip_id } => {
                write!(f, "strip {strip_id} shows no execution-context facet")
            }
            Self::DuplicateShownFacet { strip_id, facet } => {
                write!(f, "strip {strip_id} shows facet {facet} more than once")
            }
            Self::FacetIncomplete { strip_id, facet } => {
                write!(f, "strip {strip_id} facet {facet} is missing its value label or descriptor ref")
            }
            Self::DuplicateDowngradeReason { strip_id, reason } => {
                write!(f, "strip {strip_id} repeats downgrade reason {reason}")
            }
            Self::OverstatedPresentation {
                strip_id,
                published,
                computed,
            } => write!(
                f,
                "strip {strip_id} publishes presentation {published} but the gate computes {computed}"
            ),
            Self::DowngradeReasonsMismatch { strip_id } => {
                write!(f, "strip {strip_id} downgrade reasons disagree with the gate")
            }
            Self::ResolutionPathMismatch {
                strip_id,
                declared,
                required,
            } => write!(
                f,
                "strip {strip_id} records resolution {declared} but the gate requires {required}"
            ),
            Self::BlockedBeforeRunMismatch { strip_id } => write!(
                f,
                "strip {strip_id} blocked-before-run flag disagrees with the gate"
            ),
            Self::MissingResolutionPath { strip_id } => {
                write!(f, "strip {strip_id} is flagged or blocked but offers no resolution path")
            }
            Self::ResolvedStripNotWhole { strip_id } => {
                write!(f, "strip {strip_id} resolves cleanly but flags a state or carries a reason")
            }
            Self::MissingConsumerBinding { surface } => {
                write!(f, "missing consumer binding for surface {surface}")
            }
            Self::ConsumerBindingDrift { binding_ref } => {
                write!(f, "binding {binding_ref} does not preserve registry truth")
            }
            Self::SummaryMismatch => write!(f, "packet summary counts disagree with the strips"),
        }
    }
}

impl Error for M5EnvironmentStatusStripViolation {}

/// Stable record-kind tag for [`M5EnvironmentStatusStripSupportExport`].
pub const M5_ENVIRONMENT_STATUS_STRIP_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_environment_status_strips_support_export";

/// Support-export wrapper preserving the registry verbatim for support and evidence packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EnvironmentStatusStripSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// Exact registry preserved by the export.
    pub registry: M5EnvironmentStatusStrips,
}

impl M5EnvironmentStatusStripSupportExport {
    /// Whether the export preserves the same packet id and a clean registry.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == M5_ENVIRONMENT_STATUS_STRIP_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == M5_ENVIRONMENT_STATUS_STRIP_SCHEMA_VERSION
            && self.packet_id_ref == self.registry.packet_id
            && self.raw_private_material_excluded
            && self.registry.validate().is_empty()
    }
}

/// Loads the embedded environment-status-strip registry packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5EnvironmentStatusStrips`].
pub fn current_m5_environment_status_strips() -> Result<M5EnvironmentStatusStrips, serde_json::Error>
{
    serde_json::from_str(M5_ENVIRONMENT_STATUS_STRIP_JSON)
}

#[cfg(test)]
mod tests;
