//! First-useful-work entry routes and setup-later posture for M5 depth lanes.
//!
//! The stable v1 shell established a safe first-run contract: every surface can
//! be entered, skipped, revisited, and repaired from the main shell without
//! hidden setup work or account-first dead ends. This module carries that same
//! contract forward into the new M5 depth lanes — notebooks, request and
//! database workspaces, profiler or trace captures, framework packs, docs and
//! local browser depth, preview routes, companion handoff, managed sync, and
//! offboarding — so retention features feel native instead of like sidecar
//! tools with their own onboarding rituals.
//!
//! Each lane is projected as one [`M5EntryRoute`] record that reuses the v1
//! entry vocabulary from [`aureline_workspace`] (the same
//! [`FirstUsefulEntrySource`], [`LandingSurface`], [`RouteReasonClass`],
//! [`ContinueWithoutClass`], and [`RouteSwitchOption`] tokens that Start Center
//! already routes through) rather than inventing a per-feature wizard. Every
//! route pins:
//!
//! - an explicit **local-core fallback** so the lane can be opened, inspected,
//!   and learned without an account, a provider attachment, a running kernel,
//!   or a managed-sync join;
//! - a **setup-later** action set so optional managed or provider-backed
//!   enrichments stay suggestions, never blockers;
//! - the **deferred actions** the route has *not* performed — no kernel
//!   started, no request sent, no preview route exposed, no sync joined, no
//!   offboarding action committed — so setup stays reviewable; and
//! - a **first-useful-work measurement** projection so partner studies and
//!   release packets can measure real switching friction instead of anecdotal
//!   impressions.
//!
//! The records are inspectable, serde-serializable truth packets that carry no
//! credential bodies, raw provider payloads, file paths, or project content.
//! They are consumed by the live shell, the headless inspector
//! (`aureline_shell_m5_entry_routes`), the support-export wrapper, the docs
//! page under `docs/m5/first_useful_work.md`, and the published packet artifact
//! under `artifacts/ux/m5/first-useful-work-packets/`. The seeded projection is
//! deterministic so the checked-in fixtures under
//! `fixtures/ux/m5/entry-and-resume/` are bit-for-bit equal to the output of
//! [`seeded_m5_entry_routes_packet`].

use serde::{Deserialize, Serialize};

use aureline_workspace::{
    ContinueWithoutClass, FirstUsefulEntrySource, LandingSurface, RouteReasonClass,
    RouteSwitchOption,
};

use crate::onboarding_metrics::TaskSuccessState;

/// Schema version exported with every record.
pub const M5_ENTRY_ROUTES_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by UI, CLI, docs, and support export.
pub const M5_ENTRY_ROUTES_SHARED_CONTRACT_REF: &str = "shell:m5_entry_routes:v1";

/// Stable record kind for [`M5EntryRoutesPacket`] payloads.
pub const M5_ENTRY_ROUTES_PACKET_RECORD_KIND: &str = "shell_m5_entry_routes_packet_record";

/// Stable record kind for [`M5EntryRoute`] payloads.
pub const M5_ENTRY_ROUTE_RECORD_KIND: &str = "shell_m5_entry_route_record";

/// Stable record kind for [`M5EntryRoutesSupportExport`] payloads.
pub const M5_ENTRY_ROUTES_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_entry_routes_support_export_record";

/// Stable packet id used to pivot across surfaces.
pub const M5_ENTRY_ROUTES_PACKET_ID: &str = "shell:m5_entry_routes:v1:default";

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-06-11T00:00:00Z";

/// One major M5 depth lane that must remain reachable through the safe
/// first-run and setup-later posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DepthLane {
    /// Notebook surfaces (cells, outputs) opened without a running kernel.
    Notebook,
    /// Request workspaces opened without sending a request.
    RequestWorkspace,
    /// Database workspaces opened without connecting to a database.
    DatabaseWorkspace,
    /// Profiler or trace captures inspected without running a new capture.
    ProfilerTraceCapture,
    /// Framework-pack surfaces browsed without installing a pack.
    FrameworkPack,
    /// Docs and local browser depth read without browser auth.
    DocsBrowser,
    /// Preview routes inspected without exposing a route.
    Preview,
    /// Companion handoff reviewed without joining a companion device.
    CompanionHandoff,
    /// Managed sync inspected without joining managed sync.
    ManagedSync,
    /// Offboarding reviewed without committing an irreversible action.
    Offboarding,
}

impl M5DepthLane {
    /// Returns the stable schema token for this lane.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::RequestWorkspace => "request_workspace",
            Self::DatabaseWorkspace => "database_workspace",
            Self::ProfilerTraceCapture => "profiler_trace_capture",
            Self::FrameworkPack => "framework_pack",
            Self::DocsBrowser => "docs_browser",
            Self::Preview => "preview",
            Self::CompanionHandoff => "companion_handoff",
            Self::ManagedSync => "managed_sync",
            Self::Offboarding => "offboarding",
        }
    }

    /// Returns the reviewer-facing label for this lane.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Notebook => "Notebook",
            Self::RequestWorkspace => "Request workspace",
            Self::DatabaseWorkspace => "Database workspace",
            Self::ProfilerTraceCapture => "Profiler / trace capture",
            Self::FrameworkPack => "Framework pack",
            Self::DocsBrowser => "Docs / local browser",
            Self::Preview => "Preview routes",
            Self::CompanionHandoff => "Companion handoff",
            Self::ManagedSync => "Managed sync",
            Self::Offboarding => "Offboarding",
        }
    }

    /// Returns every required M5 depth lane in canonical order.
    pub const fn required_lanes() -> [Self; 10] {
        [
            Self::Notebook,
            Self::RequestWorkspace,
            Self::DatabaseWorkspace,
            Self::ProfilerTraceCapture,
            Self::FrameworkPack,
            Self::DocsBrowser,
            Self::Preview,
            Self::CompanionHandoff,
            Self::ManagedSync,
            Self::Offboarding,
        ]
    }
}

/// Class of first useful action a lane exposes before any optional setup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstUsefulActionClass {
    /// Open the surface and inspect its structure without executing anything.
    OpenAndInspectLocally,
    /// Read local or bundled content without a network round-trip.
    ReadLocalContent,
    /// Review an imported or handoff packet locally.
    ReviewPacketLocally,
    /// Browse a local catalog of templates or packs.
    BrowseCatalogLocally,
    /// Inspect a previously captured artifact without re-running a capture.
    InspectCapturedArtifact,
    /// Review a plan or diff before committing an irreversible action.
    ReviewPlanBeforeCommit,
}

impl FirstUsefulActionClass {
    /// Returns the stable schema token for this action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenAndInspectLocally => "open_and_inspect_locally",
            Self::ReadLocalContent => "read_local_content",
            Self::ReviewPacketLocally => "review_packet_locally",
            Self::BrowseCatalogLocally => "browse_catalog_locally",
            Self::InspectCapturedArtifact => "inspect_captured_artifact",
            Self::ReviewPlanBeforeCommit => "review_plan_before_commit",
        }
    }
}

/// How quickly a lane reaches first useful work after entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeToFirstUsefulWorkClass {
    /// First useful work is reachable immediately on a local open.
    ImmediateLocalOpen,
    /// First useful work follows a bounded local index or parse step.
    AfterLocalIndex,
    /// First useful work follows an explicit, reversible user choice.
    AfterExplicitUserChoice,
    /// First useful work is intentionally deferred until the user opts into a
    /// managed or provider-backed step.
    DeferredUntilUserOptsIn,
}

impl TimeToFirstUsefulWorkClass {
    /// Returns the stable schema token for this timing class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImmediateLocalOpen => "immediate_local_open",
            Self::AfterLocalIndex => "after_local_index",
            Self::AfterExplicitUserChoice => "after_explicit_user_choice",
            Self::DeferredUntilUserOptsIn => "deferred_until_user_opts_in",
        }
    }
}

/// Typed class of work the route has *not* yet performed.
///
/// Every route enumerates the deferred actions so a first-run card can explain
/// what Aureline has not yet done and setup stays reviewable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeferredActionClass {
    /// No notebook kernel has been started.
    NoKernelStarted,
    /// No request has been sent over the network.
    NoRequestSent,
    /// No database connection has been opened.
    NoDatabaseConnected,
    /// No profiler or trace capture has been run.
    NoTraceCaptured,
    /// No framework pack has been installed.
    NoFrameworkPackInstalled,
    /// No browser authentication has been completed.
    NoBrowserAuthCompleted,
    /// No preview route has been exposed.
    NoPreviewRouteExposed,
    /// No companion device has been joined.
    NoCompanionJoined,
    /// No managed sync has been joined.
    NoSyncJoined,
    /// No offboarding action has been committed.
    NoOffboardingActionCommitted,
}

impl DeferredActionClass {
    /// Returns the stable schema token for this deferred-action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoKernelStarted => "no_kernel_started",
            Self::NoRequestSent => "no_request_sent",
            Self::NoDatabaseConnected => "no_database_connected",
            Self::NoTraceCaptured => "no_trace_captured",
            Self::NoFrameworkPackInstalled => "no_framework_pack_installed",
            Self::NoBrowserAuthCompleted => "no_browser_auth_completed",
            Self::NoPreviewRouteExposed => "no_preview_route_exposed",
            Self::NoCompanionJoined => "no_companion_joined",
            Self::NoSyncJoined => "no_sync_joined",
            Self::NoOffboardingActionCommitted => "no_offboarding_action_committed",
        }
    }
}

/// A reviewable statement of work the route has not performed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeferredActionStatement {
    /// Typed deferred-action class.
    pub action_class: DeferredActionClass,
    /// Reviewer-facing statement of what Aureline has not yet done.
    pub statement: String,
}

/// Class of optional managed or provider-backed enrichment a lane suggests.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnrichmentClass {
    /// Start a notebook kernel.
    StartKernel,
    /// Attach an AI or runtime provider.
    AttachProvider,
    /// Connect to a database.
    ConnectDatabase,
    /// Run a profiler or trace capture.
    RunTraceCapture,
    /// Install a framework pack.
    InstallFrameworkPack,
    /// Authenticate the embedded browser.
    BrowserAuth,
    /// Expose a preview route.
    ExposePreviewRoute,
    /// Join a companion device.
    JoinCompanion,
    /// Sign in for managed sync.
    SignInForManagedSync,
    /// Commit an offboarding export.
    CommitOffboardingExport,
}

impl EnrichmentClass {
    /// Returns the stable schema token for this enrichment class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StartKernel => "start_kernel",
            Self::AttachProvider => "attach_provider",
            Self::ConnectDatabase => "connect_database",
            Self::RunTraceCapture => "run_trace_capture",
            Self::InstallFrameworkPack => "install_framework_pack",
            Self::BrowserAuth => "browser_auth",
            Self::ExposePreviewRoute => "expose_preview_route",
            Self::JoinCompanion => "join_companion",
            Self::SignInForManagedSync => "sign_in_for_managed_sync",
            Self::CommitOffboardingExport => "commit_offboarding_export",
        }
    }
}

/// An optional enrichment the route may suggest but must never require.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OptionalEnrichment {
    /// Typed enrichment class.
    pub enrichment_class: EnrichmentClass,
    /// Always `false`: an enrichment is a suggestion, never a blocker for
    /// opening, inspecting, or learning the feature.
    pub mandatory: bool,
    /// Reviewer-facing summary of the suggested enrichment.
    pub summary: String,
}

/// First-useful-work measurement projection for one lane.
///
/// The projection carries the success-state coverage the lane's task-success
/// instrumentation can record plus refs into the telemetry capture and
/// measurement surface that back it. It carries no raw user content.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstUsefulWorkMeasurement {
    /// Class of first useful action measured for the lane.
    pub action_class: FirstUsefulActionClass,
    /// Timing class for reaching first useful work.
    pub time_to_first_useful_work_class: TimeToFirstUsefulWorkClass,
    /// True when first useful work is reachable before any optional setup.
    pub reached_before_optional_setup: bool,
    /// Task-success states the lane's instrumentation can record.
    pub success_states_covered: Vec<TaskSuccessState>,
    /// Telemetry capture ref that backs the measurement.
    pub telemetry_capture_ref: String,
    /// Measurement surface ref the lane binds to.
    pub measurement_surface_ref: String,
    /// True when no raw sensitive user content is captured for the lane.
    pub no_raw_sensitive_user_content: bool,
}

/// One M5 depth lane projected as a first-useful-work entry route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EntryRoute {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the route.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable route id quoted across surfaces.
    pub route_id: String,
    /// Depth lane that owns the route.
    pub lane: M5DepthLane,
    /// Reviewer-facing heading for the route.
    pub title: String,
    /// Entry source that admits the route, reusing the v1 vocabulary.
    pub entry_source: FirstUsefulEntrySource,
    /// Landing surface selected by first-useful-work routing.
    pub landing_surface: LandingSurface,
    /// Reason first-useful-work routing selected the landing surface.
    pub route_reason: RouteReasonClass,
    /// True when the lane claims local-first continuity.
    pub local_first_claim: bool,
    /// True when an explicit local-core fallback is exposed.
    pub local_core_fallback: bool,
    /// Reviewer-facing summary of the local-core fallback.
    pub local_core_summary: String,
    /// Setup-later actions exposed instead of forced setup.
    pub setup_later_actions: Vec<ContinueWithoutClass>,
    /// Reversible switch options near the first landing surface.
    pub switch_options: Vec<RouteSwitchOption>,
    /// True when browser auth is a hidden prerequisite (must be `false` on
    /// local-first lanes).
    pub requires_browser_auth: bool,
    /// True when provider attachment is a hidden prerequisite.
    pub requires_provider_attachment: bool,
    /// True when kernel execution is a hidden prerequisite.
    pub requires_kernel_execution: bool,
    /// True when managed sync is a hidden prerequisite.
    pub requires_managed_sync: bool,
    /// Deferred actions the route has not yet performed.
    pub deferred_actions: Vec<DeferredActionStatement>,
    /// Optional enrichments the route may suggest but never require.
    pub optional_enrichments: Vec<OptionalEnrichment>,
    /// First-useful-work measurement projection for the lane.
    pub first_useful_work: FirstUsefulWorkMeasurement,
    /// Docs/help refs that publish the route.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs that retain the route in support evidence.
    pub support_export_refs: Vec<String>,
    /// Partner scorecard refs that consume the route.
    pub partner_scorecard_refs: Vec<String>,
    /// Reviewer-facing narrative summary.
    pub narrative: String,
}

impl M5EntryRoute {
    /// Returns `true` when no hidden prerequisite gates basic open, inspect, or
    /// learnability flows for the lane.
    pub const fn no_hidden_prerequisite(&self) -> bool {
        !(self.requires_browser_auth
            || self.requires_provider_attachment
            || self.requires_kernel_execution
            || self.requires_managed_sync)
    }

    /// Returns deterministic compact rows for text review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!("{} [{}]", self.title, self.lane.as_str()),
            format!(
                "  entry={} landing={} reason={}",
                self.entry_source.as_str(),
                self.landing_surface.as_str(),
                self.route_reason.as_str()
            ),
            format!(
                "  local_core_fallback={} no_hidden_prerequisite={}",
                self.local_core_fallback,
                self.no_hidden_prerequisite()
            ),
            format!(
                "  setup_later={}",
                self.setup_later_actions
                    .iter()
                    .map(|action| action.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            format!(
                "  first_useful_work={} timing={} before_setup={}",
                self.first_useful_work.action_class.as_str(),
                self.first_useful_work
                    .time_to_first_useful_work_class
                    .as_str(),
                self.first_useful_work.reached_before_optional_setup
            ),
        ];
        for deferred in &self.deferred_actions {
            lines.push(format!(
                "  not_yet_done: {}",
                deferred.action_class.as_str()
            ));
        }
        lines
    }
}

/// Lane coverage summary across the packet's routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaneCoverageSummary {
    /// Lanes covered by the packet, in canonical order.
    pub covered_lanes: Vec<M5DepthLane>,
    /// Total number of required lanes.
    pub total_required_lanes: usize,
    /// Number of covered lanes claiming local-first continuity.
    pub local_first_lanes: usize,
}

impl LaneCoverageSummary {
    fn from_routes(routes: &[M5EntryRoute]) -> Self {
        let mut covered_lanes = Vec::new();
        let mut local_first_lanes = 0;
        for lane in M5DepthLane::required_lanes() {
            if let Some(route) = routes.iter().find(|route| route.lane == lane) {
                covered_lanes.push(lane);
                if route.local_first_claim {
                    local_first_lanes += 1;
                }
            }
        }
        Self {
            covered_lanes,
            total_required_lanes: M5DepthLane::required_lanes().len(),
            local_first_lanes,
        }
    }

    /// Returns `true` when every required lane is covered.
    pub fn covers_every_lane(&self) -> bool {
        self.covered_lanes.len() == self.total_required_lanes
    }
}

/// First-useful-work entry-routes packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EntryRoutesPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Reviewer-facing summary line printed above the routes.
    pub headline: String,
    /// Entry routes in canonical lane order.
    pub routes: Vec<M5EntryRoute>,
    /// Lane coverage summary across the routes.
    pub lane_coverage: LaneCoverageSummary,
    /// True when no route requires a hidden prerequisite for basic open,
    /// inspect, or learnability flows.
    pub no_hidden_prerequisites: bool,
    /// True when no route captures raw sensitive user content.
    pub no_raw_sensitive_user_content: bool,
    /// Partner scorecard refs that consume the packet.
    pub partner_scorecard_refs: Vec<String>,
    /// Readiness review refs that consume the packet.
    pub readiness_review_refs: Vec<String>,
    /// Markdown packet artifact that publishes the routes.
    pub published_packet_ref: String,
    /// Docs/help refs the packet reopens from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the packet reopens from.
    pub support_export_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl M5EntryRoutesPacket {
    /// Returns the route count for the packet.
    pub fn route_count(&self) -> usize {
        self.routes.len()
    }

    /// Returns `true` when every required lane is covered.
    pub fn covers_every_lane(&self) -> bool {
        self.lane_coverage.covers_every_lane()
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "packet: id={}, routes={}, lanes={}/{}",
            self.packet_id,
            self.routes.len(),
            self.lane_coverage.covered_lanes.len(),
            self.lane_coverage.total_required_lanes,
        ));
        lines.push(format!(
            "no_hidden_prerequisites={} local_first_lanes={}",
            self.no_hidden_prerequisites, self.lane_coverage.local_first_lanes
        ));
        for route in &self.routes {
            lines.extend(route.compact_lines());
        }
        lines
    }

    /// Renders the markdown artifact for the packet.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# First-useful-work entry routes for M5 depth lanes\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_entry_routes`](../../../crates/aureline-shell/src/m5_entry_routes/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_entry_routes -- markdown > \\\n  artifacts/ux/m5/first-useful-work-packets/m5_entry_routes_packet.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!("- Routes: {}\n", self.routes.len()));
        out.push_str(&format!(
            "- Lanes covered: {}/{}\n",
            self.lane_coverage.covered_lanes.len(),
            self.lane_coverage.total_required_lanes
        ));
        out.push_str(&format!(
            "- No hidden prerequisites: {}\n",
            self.no_hidden_prerequisites
        ));
        out.push_str(&format!(
            "- No raw sensitive user content: {}\n",
            self.no_raw_sensitive_user_content
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Lane coverage\n\n");
        out.push_str(
            "| Lane | Entry | Landing | Local-core fallback | Hidden prereq | First useful work |\n",
        );
        out.push_str("|---|---|---|:---:|:---:|---|\n");
        for route in &self.routes {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | {} | {} | `{}` ({}) |\n",
                route.lane.display_label(),
                route.entry_source.as_str(),
                route.landing_surface.as_str(),
                if route.local_core_fallback {
                    "yes"
                } else {
                    "no"
                },
                if route.no_hidden_prerequisite() {
                    "none"
                } else {
                    "REQUIRED"
                },
                route.first_useful_work.action_class.as_str(),
                route
                    .first_useful_work
                    .time_to_first_useful_work_class
                    .as_str(),
            ));
        }
        out.push('\n');

        for route in &self.routes {
            out.push_str(&format!(
                "## {} (`{}`)\n\n",
                route.title,
                route.lane.as_str()
            ));
            out.push_str(&format!("{}\n\n", route.narrative));
            out.push_str(&format!(
                "- Local-core fallback: {}\n",
                route.local_core_summary
            ));
            out.push_str(&format!(
                "- Setup-later actions: {}\n",
                route
                    .setup_later_actions
                    .iter()
                    .map(|action| format!("`{}`", action.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str("- Not yet done:\n");
            for deferred in &route.deferred_actions {
                out.push_str(&format!(
                    "  - `{}` — {}\n",
                    deferred.action_class.as_str(),
                    deferred.statement
                ));
            }
            out.push_str("- Optional enrichments (never required):\n");
            for enrichment in &route.optional_enrichments {
                out.push_str(&format!(
                    "  - `{}` — {}\n",
                    enrichment.enrichment_class.as_str(),
                    enrichment.summary
                ));
            }
            out.push('\n');
        }

        out
    }
}

/// Support-export wrapper that quotes the packet plus every stable id reviewers
/// need to pivot across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EntryRoutesSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the wrapper.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: M5EntryRoutesPacket,
    /// Stable packet id, route ids, and telemetry capture refs in
    /// deterministic order.
    pub case_ids: Vec<String>,
}

impl M5EntryRoutesSupportExport {
    /// Builds the support-export wrapper for a packet.
    pub fn from_packet(support_export_id: impl Into<String>, packet: M5EntryRoutesPacket) -> Self {
        let mut case_ids = Vec::new();
        case_ids.push(packet.packet_id.clone());
        for route in &packet.routes {
            case_ids.push(route.route_id.clone());
        }
        for route in &packet.routes {
            case_ids.push(route.first_useful_work.telemetry_capture_ref.clone());
        }
        Self {
            record_kind: M5_ENTRY_ROUTES_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_ENTRY_ROUTES_SCHEMA_VERSION,
            shared_contract_ref: M5_ENTRY_ROUTES_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            case_ids,
        }
    }
}

/// Validation error produced by [`validate_m5_entry_routes_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum M5EntryRoutesValidationError {
    /// A required lane has no route in the packet.
    MissingLane {
        /// Lane with the missing route.
        lane: String,
    },
    /// A local-first lane does not expose a local-core fallback.
    LocalFirstWithoutFallback {
        /// Route that violated the invariant.
        route_id: String,
    },
    /// A route does not expose the `set_up_later` action.
    SetupLaterActionMissing {
        /// Route that violated the invariant.
        route_id: String,
    },
    /// A local-first lane declares a hidden prerequisite.
    HiddenPrerequisiteRequired {
        /// Route that violated the invariant.
        route_id: String,
        /// The prerequisite that was required.
        prerequisite: String,
    },
    /// An enrichment is marked mandatory rather than suggested.
    MandatoryEnrichment {
        /// Route that violated the invariant.
        route_id: String,
        /// The enrichment that was marked mandatory.
        enrichment: String,
    },
    /// A route does not declare any deferred-action statement.
    DeferredStatementMissing {
        /// Route that violated the invariant.
        route_id: String,
    },
    /// A local-first lane does not reach first useful work before optional
    /// setup.
    FirstUsefulWorkBehindSetup {
        /// Route that violated the invariant.
        route_id: String,
    },
    /// A route's first-useful-work measurement does not cover completion.
    FirstUsefulWorkCompletionUnmeasured {
        /// Route that violated the invariant.
        route_id: String,
    },
    /// A route declares it captures raw sensitive user content.
    RawSensitiveContentDeclared {
        /// Route that violated the invariant.
        route_id: String,
    },
    /// The lane coverage summary does not match the routes.
    LaneCoverageStale,
    /// The packet does not declare a partner scorecard ref.
    PartnerScorecardMissing,
    /// The packet does not declare a readiness review ref.
    ReadinessReviewMissing,
}

/// Validates a packet against the M5 first-useful-work acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_m5_entry_routes_packet(
    packet: &M5EntryRoutesPacket,
) -> Result<(), Vec<M5EntryRoutesValidationError>> {
    let mut errors = Vec::new();

    let coverage = LaneCoverageSummary::from_routes(&packet.routes);
    if coverage != packet.lane_coverage {
        errors.push(M5EntryRoutesValidationError::LaneCoverageStale);
    }

    for lane in M5DepthLane::required_lanes() {
        if !packet.routes.iter().any(|route| route.lane == lane) {
            errors.push(M5EntryRoutesValidationError::MissingLane {
                lane: lane.as_str().to_owned(),
            });
        }
    }

    for route in &packet.routes {
        if !route
            .setup_later_actions
            .contains(&ContinueWithoutClass::SetUpLater)
        {
            errors.push(M5EntryRoutesValidationError::SetupLaterActionMissing {
                route_id: route.route_id.clone(),
            });
        }

        if route.deferred_actions.is_empty() {
            errors.push(M5EntryRoutesValidationError::DeferredStatementMissing {
                route_id: route.route_id.clone(),
            });
        }

        for enrichment in &route.optional_enrichments {
            if enrichment.mandatory {
                errors.push(M5EntryRoutesValidationError::MandatoryEnrichment {
                    route_id: route.route_id.clone(),
                    enrichment: enrichment.enrichment_class.as_str().to_owned(),
                });
            }
        }

        if !route.first_useful_work.no_raw_sensitive_user_content {
            errors.push(M5EntryRoutesValidationError::RawSensitiveContentDeclared {
                route_id: route.route_id.clone(),
            });
        }

        if !route
            .first_useful_work
            .success_states_covered
            .contains(&TaskSuccessState::Completion)
        {
            errors.push(
                M5EntryRoutesValidationError::FirstUsefulWorkCompletionUnmeasured {
                    route_id: route.route_id.clone(),
                },
            );
        }

        if route.local_first_claim {
            if !route.local_core_fallback {
                errors.push(M5EntryRoutesValidationError::LocalFirstWithoutFallback {
                    route_id: route.route_id.clone(),
                });
            }
            for (required, token) in [
                (route.requires_browser_auth, "browser_auth"),
                (route.requires_provider_attachment, "provider_attachment"),
                (route.requires_kernel_execution, "kernel_execution"),
                (route.requires_managed_sync, "managed_sync"),
            ] {
                if required {
                    errors.push(M5EntryRoutesValidationError::HiddenPrerequisiteRequired {
                        route_id: route.route_id.clone(),
                        prerequisite: token.to_owned(),
                    });
                }
            }
            if !route.first_useful_work.reached_before_optional_setup {
                errors.push(M5EntryRoutesValidationError::FirstUsefulWorkBehindSetup {
                    route_id: route.route_id.clone(),
                });
            }
        }
    }

    if packet.partner_scorecard_refs.is_empty() {
        errors.push(M5EntryRoutesValidationError::PartnerScorecardMissing);
    }
    if packet.readiness_review_refs.is_empty() {
        errors.push(M5EntryRoutesValidationError::ReadinessReviewMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Builds the seeded M5 entry-routes packet.
pub fn seeded_m5_entry_routes_packet() -> M5EntryRoutesPacket {
    let routes = seeded_routes();
    let lane_coverage = LaneCoverageSummary::from_routes(&routes);
    let no_hidden_prerequisites = routes.iter().all(|route| route.no_hidden_prerequisite());
    let no_raw_sensitive_user_content = routes
        .iter()
        .all(|route| route.first_useful_work.no_raw_sensitive_user_content);

    M5EntryRoutesPacket {
        record_kind: M5_ENTRY_ROUTES_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_ENTRY_ROUTES_SCHEMA_VERSION,
        shared_contract_ref: M5_ENTRY_ROUTES_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_ENTRY_ROUTES_PACKET_ID.to_owned(),
        headline: "First-useful-work entry routes and setup-later posture for the M5 depth lanes."
            .to_owned(),
        routes,
        lane_coverage,
        no_hidden_prerequisites,
        no_raw_sensitive_user_content,
        partner_scorecard_refs: vec![
            "partner-scorecard:m5:first_useful_work".to_owned(),
            "partner-scorecard:m5:switching_friction".to_owned(),
        ],
        readiness_review_refs: vec![
            "readiness-review:m5:depth_lane_entry".to_owned(),
            "readiness-review:m5:setup_later_posture".to_owned(),
        ],
        published_packet_ref: "artifacts/ux/m5/first-useful-work-packets/m5_entry_routes_packet.md"
            .to_owned(),
        docs_help_refs: vec![
            "docs/m5/first_useful_work.md".to_owned(),
            "docs/help/start_center_open_folder.md".to_owned(),
        ],
        support_export_refs: vec!["support:export.include_m5_entry_routes_packet".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

struct RouteSeed {
    route_id: &'static str,
    lane: M5DepthLane,
    title: &'static str,
    entry_source: FirstUsefulEntrySource,
    landing_surface: LandingSurface,
    route_reason: RouteReasonClass,
    setup_later_actions: &'static [ContinueWithoutClass],
    switch_options: &'static [RouteSwitchOption],
    local_core_summary: &'static str,
    deferred: &'static [(DeferredActionClass, &'static str)],
    enrichments: &'static [(EnrichmentClass, &'static str)],
    action_class: FirstUsefulActionClass,
    timing: TimeToFirstUsefulWorkClass,
    telemetry_capture_ref: &'static str,
    measurement_surface_ref: &'static str,
    docs_help_refs: &'static [&'static str],
    narrative: &'static str,
}

fn build_route(seed: &RouteSeed) -> M5EntryRoute {
    M5EntryRoute {
        record_kind: M5_ENTRY_ROUTE_RECORD_KIND.to_owned(),
        schema_version: M5_ENTRY_ROUTES_SCHEMA_VERSION,
        shared_contract_ref: M5_ENTRY_ROUTES_SHARED_CONTRACT_REF.to_owned(),
        route_id: seed.route_id.to_owned(),
        lane: seed.lane,
        title: seed.title.to_owned(),
        entry_source: seed.entry_source,
        landing_surface: seed.landing_surface,
        route_reason: seed.route_reason,
        local_first_claim: true,
        local_core_fallback: true,
        local_core_summary: seed.local_core_summary.to_owned(),
        setup_later_actions: seed.setup_later_actions.to_vec(),
        switch_options: seed.switch_options.to_vec(),
        requires_browser_auth: false,
        requires_provider_attachment: false,
        requires_kernel_execution: false,
        requires_managed_sync: false,
        deferred_actions: seed
            .deferred
            .iter()
            .map(|(class, statement)| DeferredActionStatement {
                action_class: *class,
                statement: (*statement).to_owned(),
            })
            .collect(),
        optional_enrichments: seed
            .enrichments
            .iter()
            .map(|(class, summary)| OptionalEnrichment {
                enrichment_class: *class,
                mandatory: false,
                summary: (*summary).to_owned(),
            })
            .collect(),
        first_useful_work: FirstUsefulWorkMeasurement {
            action_class: seed.action_class,
            time_to_first_useful_work_class: seed.timing,
            reached_before_optional_setup: true,
            success_states_covered: vec![
                TaskSuccessState::Completion,
                TaskSuccessState::Fallback,
                TaskSuccessState::Abandonment,
                TaskSuccessState::RepairRequired,
            ],
            telemetry_capture_ref: seed.telemetry_capture_ref.to_owned(),
            measurement_surface_ref: seed.measurement_surface_ref.to_owned(),
            no_raw_sensitive_user_content: true,
        },
        docs_help_refs: seed
            .docs_help_refs
            .iter()
            .map(|r| (*r).to_owned())
            .collect(),
        support_export_refs: vec!["support:export.include_m5_entry_routes_packet".to_owned()],
        partner_scorecard_refs: vec!["partner-scorecard:m5:first_useful_work".to_owned()],
        narrative: seed.narrative.to_owned(),
    }
}

fn seeded_routes() -> Vec<M5EntryRoute> {
    route_seeds().iter().map(build_route).collect()
}

fn route_seeds() -> Vec<RouteSeed> {
    vec![
        RouteSeed {
            route_id: "route:m5.notebook.open_inspect",
            lane: M5DepthLane::Notebook,
            title: "Open a notebook and inspect cells without a kernel",
            entry_source: FirstUsefulEntrySource::SingleFileOpen,
            landing_surface: LandingSurface::FileEditorWithRootCues,
            route_reason: RouteReasonClass::StandaloneFile,
            setup_later_actions: &[
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::InspectOnly,
            ],
            switch_options: &[RouteSwitchOption::OpenLastFile, RouteSwitchOption::SetUpLater],
            local_core_summary:
                "The notebook opens read-only with cells, outputs, and structure visible; no kernel is started.",
            deferred: &[(
                DeferredActionClass::NoKernelStarted,
                "No notebook kernel has been started; cached outputs are shown as captured.",
            )],
            enrichments: &[(
                EnrichmentClass::StartKernel,
                "Start a kernel to execute cells when you are ready.",
            )],
            action_class: FirstUsefulActionClass::OpenAndInspectLocally,
            timing: TimeToFirstUsefulWorkClass::ImmediateLocalOpen,
            telemetry_capture_ref: "capture:m5.notebook.first_useful_work",
            measurement_surface_ref: "surface:m5.notebook.entry",
            docs_help_refs: &["docs/m5/first_useful_work.md", "docs/help/notebook_open.md"],
            narrative:
                "Opening a notebook lands directly in a read-only inspection of cells and captured outputs, so the user reaches first useful work before deciding whether to start a kernel.",
        },
        RouteSeed {
            route_id: "route:m5.request_workspace.open_inspect",
            lane: M5DepthLane::RequestWorkspace,
            title: "Open a request workspace and inspect requests without sending",
            entry_source: FirstUsefulEntrySource::FolderOrRepoOpen,
            landing_surface: LandingSurface::ExplorerPlusReadmeOrChangedFiles,
            route_reason: RouteReasonClass::RepoEvidence,
            setup_later_actions: &[
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::InspectOnly,
            ],
            switch_options: &[RouteSwitchOption::OpenChangedFiles, RouteSwitchOption::SetUpLater],
            local_core_summary:
                "Request definitions, environments, and history open for inspection; no request is sent.",
            deferred: &[(
                DeferredActionClass::NoRequestSent,
                "No request has been sent; saved responses are shown as previously captured.",
            )],
            enrichments: &[(
                EnrichmentClass::AttachProvider,
                "Attach an environment or secret store to send live requests.",
            )],
            action_class: FirstUsefulActionClass::OpenAndInspectLocally,
            timing: TimeToFirstUsefulWorkClass::ImmediateLocalOpen,
            telemetry_capture_ref: "capture:m5.request_workspace.first_useful_work",
            measurement_surface_ref: "surface:m5.request_workspace.entry",
            docs_help_refs: &["docs/m5/first_useful_work.md", "docs/help/request_workspace.md"],
            narrative:
                "A request workspace opens with definitions and saved responses visible, so the user can read and learn the surface before sending anything over the network.",
        },
        RouteSeed {
            route_id: "route:m5.database_workspace.open_inspect",
            lane: M5DepthLane::DatabaseWorkspace,
            title: "Open a database workspace and inspect schema without connecting",
            entry_source: FirstUsefulEntrySource::FolderOrRepoOpen,
            landing_surface: LandingSurface::ExplorerPlusReadmeOrChangedFiles,
            route_reason: RouteReasonClass::RepoEvidence,
            setup_later_actions: &[
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::InspectOnly,
            ],
            switch_options: &[RouteSwitchOption::OpenChangedFiles, RouteSwitchOption::SetUpLater],
            local_core_summary:
                "Saved connection definitions and cached schema open for inspection; no database connection is opened.",
            deferred: &[(
                DeferredActionClass::NoDatabaseConnected,
                "No database connection has been opened; cached schema is shown as last captured.",
            )],
            enrichments: &[(
                EnrichmentClass::ConnectDatabase,
                "Connect to a database to run live queries when you are ready.",
            )],
            action_class: FirstUsefulActionClass::OpenAndInspectLocally,
            timing: TimeToFirstUsefulWorkClass::ImmediateLocalOpen,
            telemetry_capture_ref: "capture:m5.database_workspace.first_useful_work",
            measurement_surface_ref: "surface:m5.database_workspace.entry",
            docs_help_refs: &["docs/m5/first_useful_work.md", "docs/help/database_workspace.md"],
            narrative:
                "A database workspace opens to saved connection definitions and cached schema, so the user can inspect and learn the surface before opening a live connection.",
        },
        RouteSeed {
            route_id: "route:m5.profiler.inspect_capture",
            lane: M5DepthLane::ProfilerTraceCapture,
            title: "Inspect a captured profile or trace without running a capture",
            entry_source: FirstUsefulEntrySource::SingleFileOpen,
            landing_surface: LandingSurface::FileEditorWithRootCues,
            route_reason: RouteReasonClass::StandaloneFile,
            setup_later_actions: &[
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::InspectOnly,
            ],
            switch_options: &[RouteSwitchOption::OpenLastFile, RouteSwitchOption::SetUpLater],
            local_core_summary:
                "A previously captured profile or trace opens for inspection; no new capture is run.",
            deferred: &[(
                DeferredActionClass::NoTraceCaptured,
                "No profiler or trace capture has been run; the existing capture is shown as recorded.",
            )],
            enrichments: &[(
                EnrichmentClass::RunTraceCapture,
                "Run a new profiler or trace capture when you are ready.",
            )],
            action_class: FirstUsefulActionClass::InspectCapturedArtifact,
            timing: TimeToFirstUsefulWorkClass::AfterLocalIndex,
            telemetry_capture_ref: "capture:m5.profiler.first_useful_work",
            measurement_surface_ref: "surface:m5.profiler.entry",
            docs_help_refs: &["docs/m5/first_useful_work.md", "docs/help/profiler_inspect.md"],
            narrative:
                "Opening a captured profile or trace lands in an inspection view of the recorded data, so the user reaches first useful work without running a new, side-effecting capture.",
        },
        RouteSeed {
            route_id: "route:m5.framework_pack.browse_catalog",
            lane: M5DepthLane::FrameworkPack,
            title: "Browse the framework-pack catalog without installing",
            entry_source: FirstUsefulEntrySource::FolderOrRepoOpen,
            landing_surface: LandingSurface::GenericShellWithDiagnostics,
            route_reason: RouteReasonClass::UnknownGenericSafeDefault,
            setup_later_actions: &[
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::DismissRecommendation,
            ],
            switch_options: &[RouteSwitchOption::OpenPlainExplorer, RouteSwitchOption::SetUpLater],
            local_core_summary:
                "Framework-pack descriptions, capabilities, and scope open for browsing; no pack is installed.",
            deferred: &[(
                DeferredActionClass::NoFrameworkPackInstalled,
                "No framework pack has been installed; capabilities are described from the local catalog.",
            )],
            enrichments: &[(
                EnrichmentClass::InstallFrameworkPack,
                "Install a framework pack to enable its surfaces when you are ready.",
            )],
            action_class: FirstUsefulActionClass::BrowseCatalogLocally,
            timing: TimeToFirstUsefulWorkClass::ImmediateLocalOpen,
            telemetry_capture_ref: "capture:m5.framework_pack.first_useful_work",
            measurement_surface_ref: "surface:m5.framework_pack.entry",
            docs_help_refs: &["docs/m5/first_useful_work.md", "docs/help/framework_packs.md"],
            narrative:
                "The framework-pack surface opens to a browsable local catalog of capabilities and scope, so the user can learn what each pack does before installing one.",
        },
        RouteSeed {
            route_id: "route:m5.docs_browser.read_local",
            lane: M5DepthLane::DocsBrowser,
            title: "Read local docs without browser authentication",
            entry_source: FirstUsefulEntrySource::ReviewOrIncidentDeepLink,
            landing_surface: LandingSurface::LinkedReviewIncidentOrWorkItem,
            route_reason: RouteReasonClass::LinkedObjectArrival,
            setup_later_actions: &[
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::OpenMinimal,
            ],
            switch_options: &[RouteSwitchOption::OpenReadme, RouteSwitchOption::SetUpLater],
            local_core_summary:
                "Bundled and cached docs open for reading; the embedded browser is not authenticated.",
            deferred: &[(
                DeferredActionClass::NoBrowserAuthCompleted,
                "No browser authentication has been completed; only local and cached docs are shown.",
            )],
            enrichments: &[(
                EnrichmentClass::BrowserAuth,
                "Authenticate the embedded browser to reach gated documentation when you are ready.",
            )],
            action_class: FirstUsefulActionClass::ReadLocalContent,
            timing: TimeToFirstUsefulWorkClass::ImmediateLocalOpen,
            telemetry_capture_ref: "capture:m5.docs_browser.first_useful_work",
            measurement_surface_ref: "surface:m5.docs_browser.entry",
            docs_help_refs: &["docs/m5/first_useful_work.md", "docs/help/docs_browser.md"],
            narrative:
                "Docs and the local browser open bundled and cached content for reading, so the user reaches first useful work without any browser sign-in.",
        },
        RouteSeed {
            route_id: "route:m5.preview.inspect_routes",
            lane: M5DepthLane::Preview,
            title: "Inspect preview route definitions without exposing a route",
            entry_source: FirstUsefulEntrySource::FolderOrRepoOpen,
            landing_surface: LandingSurface::ExplorerPlusReadmeOrChangedFiles,
            route_reason: RouteReasonClass::RepoEvidence,
            setup_later_actions: &[
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::InspectOnly,
            ],
            switch_options: &[RouteSwitchOption::OpenChangedFiles, RouteSwitchOption::SetUpLater],
            local_core_summary:
                "Preview route definitions and scope open for inspection; no preview route is exposed.",
            deferred: &[(
                DeferredActionClass::NoPreviewRouteExposed,
                "No preview route has been exposed; route definitions are shown without serving.",
            )],
            enrichments: &[(
                EnrichmentClass::ExposePreviewRoute,
                "Expose a preview route to serve it when you are ready.",
            )],
            action_class: FirstUsefulActionClass::OpenAndInspectLocally,
            timing: TimeToFirstUsefulWorkClass::ImmediateLocalOpen,
            telemetry_capture_ref: "capture:m5.preview.first_useful_work",
            measurement_surface_ref: "surface:m5.preview.entry",
            docs_help_refs: &["docs/m5/first_useful_work.md", "docs/help/preview_routes.md"],
            narrative:
                "Preview opens to a read-only view of route definitions and their scope, so the user can learn the surface before exposing any route.",
        },
        RouteSeed {
            route_id: "route:m5.companion_handoff.review_packet",
            lane: M5DepthLane::CompanionHandoff,
            title: "Review a companion handoff packet without joining",
            entry_source: FirstUsefulEntrySource::ImportedStateOrHandoffPacket,
            landing_surface: LandingSurface::ImportCompareOrRestoreSheet,
            route_reason: RouteReasonClass::ImportedPacketReview,
            setup_later_actions: &[
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::CompareBeforeRestore,
            ],
            switch_options: &[RouteSwitchOption::CompareImport, RouteSwitchOption::SetUpLater],
            local_core_summary:
                "The handoff packet opens for local review and comparison; no companion device is joined.",
            deferred: &[(
                DeferredActionClass::NoCompanionJoined,
                "No companion device has been joined; the handoff packet is reviewed locally only.",
            )],
            enrichments: &[(
                EnrichmentClass::JoinCompanion,
                "Join the companion device to continue the handoff when you are ready.",
            )],
            action_class: FirstUsefulActionClass::ReviewPacketLocally,
            timing: TimeToFirstUsefulWorkClass::AfterExplicitUserChoice,
            telemetry_capture_ref: "capture:m5.companion_handoff.first_useful_work",
            measurement_surface_ref: "surface:m5.companion_handoff.entry",
            docs_help_refs: &["docs/m5/first_useful_work.md", "docs/help/companion_handoff.md"],
            narrative:
                "A companion handoff packet opens for local review and comparison, so the user can inspect what would transfer before joining a companion device.",
        },
        RouteSeed {
            route_id: "route:m5.managed_sync.inspect_state",
            lane: M5DepthLane::ManagedSync,
            title: "Inspect managed sync state without joining sync",
            entry_source: FirstUsefulEntrySource::RestoreLastSession,
            landing_surface: LandingSurface::RestoredLayoutWithPlaceholders,
            route_reason: RouteReasonClass::RestoreProvenance,
            setup_later_actions: &[
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::ContinueInRestrictedMode,
            ],
            switch_options: &[RouteSwitchOption::ReviewTrust, RouteSwitchOption::SetUpLater],
            local_core_summary:
                "Local sync state and what would be shared open for inspection; managed sync is not joined.",
            deferred: &[(
                DeferredActionClass::NoSyncJoined,
                "No managed sync has been joined; only local state is shown and nothing is uploaded.",
            )],
            enrichments: &[(
                EnrichmentClass::SignInForManagedSync,
                "Sign in for managed sync to share state across devices when you are ready.",
            )],
            action_class: FirstUsefulActionClass::OpenAndInspectLocally,
            timing: TimeToFirstUsefulWorkClass::ImmediateLocalOpen,
            telemetry_capture_ref: "capture:m5.managed_sync.first_useful_work",
            measurement_surface_ref: "surface:m5.managed_sync.entry",
            docs_help_refs: &["docs/m5/first_useful_work.md", "docs/help/managed_sync.md"],
            narrative:
                "Managed sync opens to a local inspection of sync state and what would be shared, so the user can learn the surface and continue locally without signing in.",
        },
        RouteSeed {
            route_id: "route:m5.offboarding.review_plan",
            lane: M5DepthLane::Offboarding,
            title: "Review an offboarding plan without committing an action",
            entry_source: FirstUsefulEntrySource::RestoreLastSession,
            landing_surface: LandingSurface::ImportCompareOrRestoreSheet,
            route_reason: RouteReasonClass::RestoreProvenance,
            setup_later_actions: &[
                ContinueWithoutClass::SetUpLater,
                ContinueWithoutClass::CompareBeforeRestore,
            ],
            switch_options: &[RouteSwitchOption::CompareImport, RouteSwitchOption::SetUpLater],
            local_core_summary:
                "The offboarding plan and export preview open for review; no irreversible offboarding action is committed.",
            deferred: &[(
                DeferredActionClass::NoOffboardingActionCommitted,
                "No offboarding action has been committed; the plan and export preview are review-only.",
            )],
            enrichments: &[(
                EnrichmentClass::CommitOffboardingExport,
                "Commit the offboarding export after reviewing the plan when you are ready.",
            )],
            action_class: FirstUsefulActionClass::ReviewPlanBeforeCommit,
            timing: TimeToFirstUsefulWorkClass::AfterExplicitUserChoice,
            telemetry_capture_ref: "capture:m5.offboarding.first_useful_work",
            measurement_surface_ref: "surface:m5.offboarding.entry",
            docs_help_refs: &["docs/m5/first_useful_work.md", "docs/help/offboarding.md"],
            narrative:
                "Offboarding opens to a review of the plan and an export preview, so the user can understand the full effect before committing any irreversible action.",
        },
    ]
}

#[cfg(test)]
mod tests;
