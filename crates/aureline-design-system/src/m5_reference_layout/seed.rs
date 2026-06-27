//! Canonical seed builder for the M5 design-system reference-layout package.
//!
//! This builder is the single producer of the checked-in reference-layout fixtures (the package
//! file and one file per workspace), the release-packet proof, and the shell-slot conformance
//! packet. The headless emitter and the inline tests both call it so the in-code layouts, the
//! schema fixtures, the proof, and the conformance packet never drift. The governed zone, slot,
//! fallback-placement, and placeholder-class tokens match the canonical shell vocabulary, so the
//! descriptors name the same layout identities shell code consumes.

use super::*;

/// Stable id of the canonical reference-layout package.
pub const M5_REFERENCE_LAYOUT_PACKAGE_ID: &str = "design-system:reference-layout-package:core";

/// Version of the canonical reference-layout package.
pub const M5_REFERENCE_LAYOUT_PACKAGE_VERSION: &str = "1.0.0";

/// Mint timestamp pinned by the seed builder.
const SEED_TIMESTAMP: &str = "2026-06-26T00:00:00Z";

const PACKAGE_OWNER_ROLE: &str = "Design system owner";
const WORKSPACE_OWNER_ROLE: &str = "Workspace owner";

fn source_contract_refs() -> Vec<String> {
    vec![
        M5_REFERENCE_LAYOUT_SCHEMA_REF.to_owned(),
        M5_REFERENCE_LAYOUT_DOC_REF.to_owned(),
        M5_REFERENCE_LAYOUT_PROOF_REF.to_owned(),
    ]
}

fn workspace_id(kind: M5WorkspaceKind) -> String {
    format!("design-system:reference-layout:{}", kind.as_str())
}

fn lifecycle() -> M5WorkspaceLifecycle {
    M5WorkspaceLifecycle {
        owner_role: WORKSPACE_OWNER_ROLE.to_owned(),
        lifecycle_state: M5WorkspaceLifecycleState::Stable,
        layout_version: 1,
        introduced_in_package_version: M5_REFERENCE_LAYOUT_PACKAGE_VERSION.to_owned(),
    }
}

/// Builds one zone occupancy, using the zone's canonical slot id.
fn occ(
    zone: M5ShellZone,
    role: &str,
    surface_kind: M5SurfaceKind,
    required: bool,
    placeholder: &str,
) -> M5ZoneOccupancy {
    M5ZoneOccupancy {
        zone,
        slot_id: zone.canonical_slot_id().to_owned(),
        surface_role: role.to_owned(),
        surface_kind,
        required,
        placeholder_behavior: placeholder.to_owned(),
    }
}

/// Builds one zone occupancy with an explicit slot id (used where a zone has more than one slot).
fn occ_slot(
    zone: M5ShellZone,
    slot_id: &str,
    role: &str,
    surface_kind: M5SurfaceKind,
    required: bool,
    placeholder: &str,
) -> M5ZoneOccupancy {
    M5ZoneOccupancy {
        zone,
        slot_id: slot_id.to_owned(),
        surface_role: role.to_owned(),
        surface_kind,
        required,
        placeholder_behavior: placeholder.to_owned(),
    }
}

/// Builds one responsive collapse rule (collapses preserve zone identity).
fn collapse(
    adaptive_class: M5AdaptiveClass,
    collapsed_zones: &[M5ShellZone],
    placement: M5FallbackPlacement,
    behavior: &str,
) -> M5ResponsiveCollapseRule {
    M5ResponsiveCollapseRule {
        adaptive_class,
        collapsed_zones: collapsed_zones.to_vec(),
        placement,
        preserves_zone_identity: true,
        behavior: behavior.to_owned(),
    }
}

/// Builds one missing-dependency rule, deriving the governed placeholder message id.
fn missing(
    ws_id: &str,
    dependency_id: &str,
    slug: &str,
    affected_zone: M5ShellZone,
    placeholder_class: M5PlaceholderClass,
    degraded_behavior: &str,
) -> M5MissingDependencyRule {
    M5MissingDependencyRule {
        dependency_id: dependency_id.to_owned(),
        affected_zone,
        placeholder_class,
        placeholder_message_id: format!(
            "{}{}.placeholder.{}",
            M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX, ws_id, slug
        ),
        degraded_behavior: degraded_behavior.to_owned(),
    }
}

/// Builds one reopen / reset route, deriving the governed command message id.
fn route(
    ws_id: &str,
    route_id: &str,
    route_kind: M5LayoutRouteKind,
    keys: &str,
    description: &str,
) -> M5LayoutRoute {
    M5LayoutRoute {
        route_id: route_id.to_owned(),
        route_kind,
        command_message_id: format!(
            "{}{}.route.{}",
            M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX, ws_id, route_id
        ),
        keys: keys.to_owned(),
        description: description.to_owned(),
    }
}

/// The reset-to-reference-layout route every workspace offers.
fn reset_route(ws_id: &str) -> M5LayoutRoute {
    route(
        ws_id,
        "reset_layout",
        M5LayoutRouteKind::Reset,
        "Ctrl+Alt+0",
        "Reset the workspace to its reference zone layout.",
    )
}

/// Assembles one workspace reference layout, deriving the summary message id.
fn layout(
    kind: M5WorkspaceKind,
    display_name: &str,
    zone_occupancy: Vec<M5ZoneOccupancy>,
    responsive_collapse: Vec<M5ResponsiveCollapseRule>,
    missing_dependency_rules: Vec<M5MissingDependencyRule>,
    reopen_routes: Vec<M5LayoutRoute>,
) -> M5WorkspaceReferenceLayout {
    let ws_id = workspace_id(kind);
    M5WorkspaceReferenceLayout {
        workspace_kind: kind,
        workspace_id: ws_id.clone(),
        display_name: display_name.to_owned(),
        lifecycle: lifecycle(),
        zone_occupancy,
        responsive_collapse,
        missing_dependency_rules,
        reopen_routes,
        summary_message_id: format!("{}{}.summary", M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX, ws_id),
    }
}

const NO_COLLAPSE: &[M5ShellZone] = &[];

// --- Notebook ---------------------------------------------------------------

fn notebook() -> M5WorkspaceReferenceLayout {
    let id = workspace_id(M5WorkspaceKind::Notebook);
    layout(
        M5WorkspaceKind::Notebook,
        "Notebook workspace",
        vec![
            occ(
                M5ShellZone::TitleContextBar,
                "Notebook identity, kernel target, and trust",
                M5SurfaceKind::HostChrome,
                true,
                "Reserve the bar and show the notebook name with an unresolved-kernel cue.",
            ),
            occ(
                M5ShellZone::ActivityRail,
                "Top-level route rail",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the rail with a quiet skeleton until routes resolve.",
            ),
            occ(
                M5ShellZone::LeftSidebar,
                "Notebook outline and cell navigator",
                M5SurfaceKind::FirstParty,
                false,
                "Show an outline skeleton; never collapse silently.",
            ),
            occ_slot(
                M5ShellZone::MainWorkspace,
                "slot.main_workspace.working_set",
                "Cell working set (code, markdown, output)",
                M5SurfaceKind::FirstParty,
                true,
                "Show the empty-notebook guidance with an add-cell route, never a blank canvas.",
            ),
            occ(
                M5ShellZone::RightInspector,
                "Variable, output, and kernel inspector",
                M5SurfaceKind::ProviderBacked,
                false,
                "Show an inspector skeleton until the kernel reports.",
            ),
            occ(
                M5ShellZone::BottomPanel,
                "Kernel log and execution output",
                M5SurfaceKind::ProviderBacked,
                false,
                "Reserve the panel and disclose connection progress.",
            ),
            occ(
                M5ShellZone::StatusBar,
                "Kernel state, run progress, and trust",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the strip present and announce kernel/run transitions.",
            ),
        ],
        vec![
            collapse(
                M5AdaptiveClass::CompactDesktop,
                &[M5ShellZone::LeftSidebar, M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "Outline and inspector move to attached sheets; the cell working set stays \
                 full-width.",
            ),
            collapse(
                M5AdaptiveClass::StandardDesktop,
                &[M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "The inspector moves to an attached sheet; the outline stays docked.",
            ),
            collapse(
                M5AdaptiveClass::ExpandedDesktop,
                NO_COLLAPSE,
                M5FallbackPlacement::Docked,
                "All zones stay docked.",
            ),
        ],
        vec![
            missing(
                &id,
                "notebook.kernel_runtime",
                "kernel",
                M5ShellZone::BottomPanel,
                M5PlaceholderClass::MissingProvider,
                "Cells stay editable and the last run's outputs stay visible; execution resumes \
                 when a kernel reconnects.",
            ),
            missing(
                &id,
                "notebook.remote_session",
                "remote",
                M5ShellZone::MainWorkspace,
                M5PlaceholderClass::MissingRemote,
                "Stored cells and outputs stay readable read-only; execution is disabled until the \
                 session returns.",
            ),
        ],
        vec![
            route(
                &id,
                "reopen_inspector",
                M5LayoutRouteKind::Reopen,
                "Ctrl+Alt+I",
                "Reopen the variable and output inspector in the right inspector zone.",
            ),
            reset_route(&id),
        ],
    )
}

// --- Data grid --------------------------------------------------------------

fn data_grid() -> M5WorkspaceReferenceLayout {
    let id = workspace_id(M5WorkspaceKind::DataGrid);
    layout(
        M5WorkspaceKind::DataGrid,
        "Data-grid workspace",
        vec![
            occ(
                M5ShellZone::TitleContextBar,
                "Result-grid identity, source, and row count",
                M5SurfaceKind::HostChrome,
                true,
                "Reserve the bar and show the source with an unresolved-query cue.",
            ),
            occ(
                M5ShellZone::ActivityRail,
                "Top-level route rail",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the rail with a quiet skeleton until routes resolve.",
            ),
            occ(
                M5ShellZone::LeftSidebar,
                "Saved queries and schema browser",
                M5SurfaceKind::FirstParty,
                false,
                "Show a schema skeleton; never collapse silently.",
            ),
            occ_slot(
                M5ShellZone::MainWorkspace,
                "slot.main_workspace.working_set",
                "Dense, virtualizable result grid",
                M5SurfaceKind::FirstParty,
                true,
                "Show the empty-result guidance with a run-query route, never a blank grid.",
            ),
            occ(
                M5ShellZone::RightInspector,
                "Row detail and cell inspector",
                M5SurfaceKind::ProviderBacked,
                false,
                "Show an inspector skeleton until a row is selected.",
            ),
            occ(
                M5ShellZone::BottomPanel,
                "Query editor and execution console",
                M5SurfaceKind::FirstParty,
                false,
                "Reserve the panel and keep the last query editable.",
            ),
            occ(
                M5ShellZone::StatusBar,
                "Query state, row count, and source trust",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the strip present and announce query/row transitions.",
            ),
        ],
        vec![
            collapse(
                M5AdaptiveClass::CompactDesktop,
                &[M5ShellZone::LeftSidebar, M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "The schema browser and row detail move to attached sheets; the grid stays \
                 full-width.",
            ),
            collapse(
                M5AdaptiveClass::StandardDesktop,
                &[M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "Row detail moves to an attached sheet; the schema browser stays docked.",
            ),
            collapse(
                M5AdaptiveClass::ExpandedDesktop,
                NO_COLLAPSE,
                M5FallbackPlacement::Docked,
                "All zones stay docked.",
            ),
        ],
        vec![
            missing(
                &id,
                "data_grid.query_provider",
                "query",
                M5ShellZone::MainWorkspace,
                M5PlaceholderClass::MissingProvider,
                "The last result set stays readable and exportable; new queries are disabled until \
                 the provider reconnects.",
            ),
            missing(
                &id,
                "data_grid.row_detail_service",
                "detail",
                M5ShellZone::RightInspector,
                M5PlaceholderClass::MissingProvider,
                "Grid rows stay visible; per-row detail loads when the service returns.",
            ),
        ],
        vec![
            route(
                &id,
                "reopen_query_console",
                M5LayoutRouteKind::Reopen,
                "Ctrl+Alt+Q",
                "Reopen the query editor and execution console in the bottom panel zone.",
            ),
            reset_route(&id),
        ],
    )
}

// --- Profiler ---------------------------------------------------------------

fn profiler() -> M5WorkspaceReferenceLayout {
    let id = workspace_id(M5WorkspaceKind::Profiler);
    layout(
        M5WorkspaceKind::Profiler,
        "Profiler workspace",
        vec![
            occ(
                M5ShellZone::TitleContextBar,
                "Profiler capture identity and target",
                M5SurfaceKind::HostChrome,
                true,
                "Reserve the bar and show the target with an unresolved-capture cue.",
            ),
            occ(
                M5ShellZone::ActivityRail,
                "Top-level route rail",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the rail with a quiet skeleton until routes resolve.",
            ),
            occ(
                M5ShellZone::LeftSidebar,
                "Capture list and call-tree navigator",
                M5SurfaceKind::FirstParty,
                false,
                "Show a capture-list skeleton; never collapse silently.",
            ),
            occ_slot(
                M5ShellZone::MainWorkspace,
                "slot.main_workspace.working_set",
                "Flame and capture working set",
                M5SurfaceKind::FirstParty,
                true,
                "Show the empty-capture guidance with a start-capture route, never a blank flame.",
            ),
            occ(
                M5ShellZone::RightInspector,
                "Frame and allocation detail",
                M5SurfaceKind::ProviderBacked,
                false,
                "Show an inspector skeleton until a frame is selected.",
            ),
            occ(
                M5ShellZone::BottomPanel,
                "Capture log and sampling output",
                M5SurfaceKind::ProviderBacked,
                false,
                "Reserve the panel and disclose sampling progress.",
            ),
            occ(
                M5ShellZone::StatusBar,
                "Capture state and sampling progress",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the strip present and announce capture/sampling transitions.",
            ),
        ],
        vec![
            collapse(
                M5AdaptiveClass::CompactDesktop,
                &[
                    M5ShellZone::LeftSidebar,
                    M5ShellZone::RightInspector,
                    M5ShellZone::BottomPanel,
                ],
                M5FallbackPlacement::Sheet,
                "The capture list, frame detail, and sampling log move to attached sheets; the \
                 flame surface stays full-width.",
            ),
            collapse(
                M5AdaptiveClass::StandardDesktop,
                &[M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "Frame detail moves to an attached sheet; the capture list and log stay docked.",
            ),
            collapse(
                M5AdaptiveClass::ExpandedDesktop,
                NO_COLLAPSE,
                M5FallbackPlacement::Docked,
                "All zones stay docked.",
            ),
        ],
        vec![
            missing(
                &id,
                "profiler.capture_provider",
                "capture",
                M5ShellZone::MainWorkspace,
                M5PlaceholderClass::MissingProvider,
                "Saved captures stay viewable; new captures are disabled until the profiler \
                 backend reconnects.",
            ),
            missing(
                &id,
                "profiler.symbol_service",
                "symbols",
                M5ShellZone::RightInspector,
                M5PlaceholderClass::MissingProvider,
                "Frames stay visible with raw addresses; symbol names resolve when the service \
                 returns.",
            ),
        ],
        vec![
            route(
                &id,
                "reopen_inspector",
                M5LayoutRouteKind::Reopen,
                "Ctrl+Alt+I",
                "Reopen the frame and allocation inspector in the right inspector zone.",
            ),
            reset_route(&id),
        ],
    )
}

// --- Pipeline ---------------------------------------------------------------

fn pipeline() -> M5WorkspaceReferenceLayout {
    let id = workspace_id(M5WorkspaceKind::Pipeline);
    layout(
        M5WorkspaceKind::Pipeline,
        "Pipeline workspace",
        vec![
            occ(
                M5ShellZone::TitleContextBar,
                "Pipeline identity, run, and trust",
                M5SurfaceKind::HostChrome,
                true,
                "Reserve the bar and show the pipeline with an unresolved-run cue.",
            ),
            occ(
                M5ShellZone::ActivityRail,
                "Top-level route rail",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the rail with a quiet skeleton until routes resolve.",
            ),
            occ(
                M5ShellZone::LeftSidebar,
                "Stage list and run history",
                M5SurfaceKind::FirstParty,
                false,
                "Show a run-history skeleton; never collapse silently.",
            ),
            occ_slot(
                M5ShellZone::MainWorkspace,
                "slot.main_workspace.review_surface",
                "Stage cards and run review surface",
                M5SurfaceKind::FirstParty,
                true,
                "Show the no-run guidance with a trigger-run route, never a blank board.",
            ),
            occ(
                M5ShellZone::RightInspector,
                "Job log and artifact detail",
                M5SurfaceKind::ProviderBacked,
                false,
                "Show an inspector skeleton until a job is selected.",
            ),
            occ(
                M5ShellZone::BottomPanel,
                "Live run output and step log",
                M5SurfaceKind::ProviderBacked,
                false,
                "Reserve the panel and disclose stream progress.",
            ),
            occ(
                M5ShellZone::StatusBar,
                "Run state, stage progress, and trust",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the strip present and announce run/stage transitions.",
            ),
        ],
        vec![
            collapse(
                M5AdaptiveClass::CompactDesktop,
                &[M5ShellZone::LeftSidebar, M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "Run history and job detail move to attached sheets; the stage board stays \
                 full-width.",
            ),
            collapse(
                M5AdaptiveClass::StandardDesktop,
                &[M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "Job detail moves to an attached sheet; run history stays docked.",
            ),
            collapse(
                M5AdaptiveClass::ExpandedDesktop,
                NO_COLLAPSE,
                M5FallbackPlacement::Docked,
                "All zones stay docked.",
            ),
        ],
        vec![
            missing(
                &id,
                "pipeline.run_provider",
                "run",
                M5ShellZone::MainWorkspace,
                M5PlaceholderClass::MissingProvider,
                "The last run's stage cards stay readable; new runs are disabled until the \
                 pipeline backend reconnects.",
            ),
            missing(
                &id,
                "pipeline.log_stream",
                "log",
                M5ShellZone::BottomPanel,
                M5PlaceholderClass::MissingProvider,
                "Stage cards stay visible; the live log resumes when the stream reconnects.",
            ),
        ],
        vec![
            route(
                &id,
                "reopen_job_detail",
                M5LayoutRouteKind::Reopen,
                "Ctrl+Alt+J",
                "Reopen the job log and artifact detail in the right inspector zone.",
            ),
            reset_route(&id),
        ],
    )
}

// --- Docs -------------------------------------------------------------------

fn docs() -> M5WorkspaceReferenceLayout {
    let id = workspace_id(M5WorkspaceKind::Docs);
    layout(
        M5WorkspaceKind::Docs,
        "Docs workspace",
        vec![
            occ(
                M5ShellZone::TitleContextBar,
                "Docs source identity and trust",
                M5SurfaceKind::HostChrome,
                true,
                "Reserve the bar and show the source with an unresolved-page cue.",
            ),
            occ(
                M5ShellZone::ActivityRail,
                "Top-level route rail",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the rail with a quiet skeleton until routes resolve.",
            ),
            occ(
                M5ShellZone::LeftSidebar,
                "Table of contents and search",
                M5SurfaceKind::FirstParty,
                false,
                "Show a table-of-contents skeleton; never collapse silently.",
            ),
            occ_slot(
                M5ShellZone::MainWorkspace,
                "slot.main_workspace.working_set",
                "Embedded docs/browser reading pane",
                M5SurfaceKind::FirstParty,
                true,
                "Show the no-page guidance with a search route, never a blank pane.",
            ),
            occ(
                M5ShellZone::RightInspector,
                "On-this-page outline and related links",
                M5SurfaceKind::FirstParty,
                false,
                "Show an outline skeleton until the page renders.",
            ),
            occ(
                M5ShellZone::StatusBar,
                "Reading position, source, and offline state",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the strip present and announce page/offline transitions.",
            ),
        ],
        vec![
            collapse(
                M5AdaptiveClass::CompactDesktop,
                &[M5ShellZone::LeftSidebar, M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "The table of contents and page outline move to attached sheets; the reading pane \
                 stays full-width.",
            ),
            collapse(
                M5AdaptiveClass::StandardDesktop,
                &[M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "The page outline moves to an attached sheet; the table of contents stays docked.",
            ),
            collapse(
                M5AdaptiveClass::ExpandedDesktop,
                NO_COLLAPSE,
                M5FallbackPlacement::Docked,
                "All zones stay docked.",
            ),
        ],
        vec![
            missing(
                &id,
                "docs.content_provider",
                "content",
                M5ShellZone::MainWorkspace,
                M5PlaceholderClass::MissingProvider,
                "Cached pages stay readable offline; uncached pages load when the content provider \
                 returns.",
            ),
            missing(
                &id,
                "docs.search_index",
                "search",
                M5ShellZone::LeftSidebar,
                M5PlaceholderClass::MissingProvider,
                "The table of contents stays browsable; full-text search resumes when the index \
                 loads.",
            ),
        ],
        vec![
            route(
                &id,
                "reopen_outline",
                M5LayoutRouteKind::Reopen,
                "Ctrl+Alt+O",
                "Reopen the on-this-page outline in the right inspector zone.",
            ),
            reset_route(&id),
        ],
    )
}

// --- Preview ----------------------------------------------------------------

fn preview() -> M5WorkspaceReferenceLayout {
    let id = workspace_id(M5WorkspaceKind::Preview);
    layout(
        M5WorkspaceKind::Preview,
        "Preview workspace",
        vec![
            occ(
                M5ShellZone::TitleContextBar,
                "Preview route, target, and trust",
                M5SurfaceKind::HostChrome,
                true,
                "Reserve the bar and show the route with an unresolved-server cue.",
            ),
            occ(
                M5ShellZone::ActivityRail,
                "Top-level route rail",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the rail with a quiet skeleton until routes resolve.",
            ),
            occ_slot(
                M5ShellZone::MainWorkspace,
                "slot.main_workspace.working_set",
                "Live preview canvas",
                M5SurfaceKind::FirstParty,
                true,
                "Show the no-preview guidance with a start-server route, never a blank canvas.",
            ),
            occ(
                M5ShellZone::RightInspector,
                "Route, device, and trust inspector",
                M5SurfaceKind::ProviderBacked,
                false,
                "Show an inspector skeleton until the preview connects.",
            ),
            occ(
                M5ShellZone::BottomPanel,
                "Preview console and request log",
                M5SurfaceKind::ProviderBacked,
                false,
                "Reserve the panel and disclose connection progress.",
            ),
            occ(
                M5ShellZone::StatusBar,
                "Preview route, reload state, and trust",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the strip present and announce route/reload transitions.",
            ),
        ],
        vec![
            collapse(
                M5AdaptiveClass::CompactDesktop,
                &[M5ShellZone::RightInspector, M5ShellZone::BottomPanel],
                M5FallbackPlacement::Sheet,
                "The inspector and console move to attached sheets; the preview canvas stays \
                 full-width.",
            ),
            collapse(
                M5AdaptiveClass::StandardDesktop,
                &[M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "The inspector moves to an attached sheet; the console stays docked.",
            ),
            collapse(
                M5AdaptiveClass::ExpandedDesktop,
                NO_COLLAPSE,
                M5FallbackPlacement::Docked,
                "All zones stay docked.",
            ),
        ],
        vec![
            missing(
                &id,
                "preview.dev_server",
                "server",
                M5ShellZone::MainWorkspace,
                M5PlaceholderClass::MissingRemote,
                "The last rendered frame stays visible with a stale badge; live reload resumes \
                 when the dev server returns.",
            ),
            missing(
                &id,
                "preview.route_trust",
                "trust",
                M5ShellZone::RightInspector,
                M5PlaceholderClass::MissingProvider,
                "The canvas stays visible; route trust detail resolves when the trust service \
                 responds.",
            ),
        ],
        vec![
            route(
                &id,
                "reopen_console",
                M5LayoutRouteKind::Reopen,
                "Ctrl+Alt+K",
                "Reopen the preview console and request log in the bottom panel zone.",
            ),
            reset_route(&id),
        ],
    )
}

// --- Incident ---------------------------------------------------------------

fn incident() -> M5WorkspaceReferenceLayout {
    let id = workspace_id(M5WorkspaceKind::Incident);
    layout(
        M5WorkspaceKind::Incident,
        "Incident workspace",
        vec![
            occ(
                M5ShellZone::TitleContextBar,
                "Incident identity, severity, and trust",
                M5SurfaceKind::HostChrome,
                true,
                "Reserve the bar and show the incident with an unresolved-signal cue.",
            ),
            occ(
                M5ShellZone::ActivityRail,
                "Top-level route rail",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the rail with a quiet skeleton until routes resolve.",
            ),
            occ(
                M5ShellZone::LeftSidebar,
                "Incident timeline and linked signals",
                M5SurfaceKind::FirstParty,
                false,
                "Show a timeline skeleton; never collapse silently.",
            ),
            occ_slot(
                M5ShellZone::MainWorkspace,
                "slot.main_workspace.review_surface",
                "Incident timeline and evidence review surface",
                M5SurfaceKind::FirstParty,
                true,
                "Show the no-incident guidance with a recent-incidents route, never a blank \
                 surface.",
            ),
            occ(
                M5ShellZone::RightInspector,
                "Signal, owner, and action detail",
                M5SurfaceKind::ProviderBacked,
                false,
                "Show an inspector skeleton until a signal is selected.",
            ),
            occ(
                M5ShellZone::BottomPanel,
                "Linked logs and live signal stream",
                M5SurfaceKind::ProviderBacked,
                false,
                "Reserve the panel and disclose stream progress.",
            ),
            occ(
                M5ShellZone::StatusBar,
                "Incident state, severity, and acknowledgement",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the strip present and announce severity/acknowledgement transitions.",
            ),
        ],
        vec![
            collapse(
                M5AdaptiveClass::CompactDesktop,
                &[M5ShellZone::LeftSidebar, M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "The timeline list and signal detail move to attached sheets; the review surface \
                 stays full-width.",
            ),
            collapse(
                M5AdaptiveClass::StandardDesktop,
                &[M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "Signal detail moves to an attached sheet; the timeline list stays docked.",
            ),
            collapse(
                M5AdaptiveClass::ExpandedDesktop,
                NO_COLLAPSE,
                M5FallbackPlacement::Docked,
                "All zones stay docked.",
            ),
        ],
        vec![
            missing(
                &id,
                "incident.signal_provider",
                "signals",
                M5ShellZone::BottomPanel,
                M5PlaceholderClass::MissingProvider,
                "The recorded timeline stays readable; live signals resume when the provider \
                 reconnects.",
            ),
            missing(
                &id,
                "incident.remote_session",
                "remote",
                M5ShellZone::MainWorkspace,
                M5PlaceholderClass::MissingRemote,
                "The recorded incident timeline stays readable; new actions are disabled until the \
                 session returns.",
            ),
        ],
        vec![
            route(
                &id,
                "reopen_signal_detail",
                M5LayoutRouteKind::Reopen,
                "Ctrl+Alt+S",
                "Reopen the signal, owner, and action detail in the right inspector zone.",
            ),
            reset_route(&id),
        ],
    )
}

// --- Companion --------------------------------------------------------------

fn companion() -> M5WorkspaceReferenceLayout {
    let id = workspace_id(M5WorkspaceKind::Companion);
    layout(
        M5WorkspaceKind::Companion,
        "Companion workspace",
        vec![
            occ(
                M5ShellZone::TitleContextBar,
                "Companion identity and paired session",
                M5SurfaceKind::HostChrome,
                true,
                "Reserve the bar and show the pairing target with an unresolved-session cue.",
            ),
            occ(
                M5ShellZone::ActivityRail,
                "Top-level route rail",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the rail with a quiet skeleton until routes resolve.",
            ),
            occ_slot(
                M5ShellZone::MainWorkspace,
                "slot.main_workspace.working_set",
                "Mirrored primary-session working set",
                M5SurfaceKind::FirstParty,
                true,
                "Show the not-paired guidance with a pair-device route, never a blank mirror.",
            ),
            occ(
                M5ShellZone::RightInspector,
                "Companion actions and device handoff",
                M5SurfaceKind::FirstParty,
                false,
                "Show an action skeleton until the paired session reports.",
            ),
            occ(
                M5ShellZone::TransientOverlay,
                "Pairing and handoff sheets",
                M5SurfaceKind::HostChrome,
                false,
                "Keep the overlay dismissible and reserved for pairing/handoff sheets.",
            ),
            occ(
                M5ShellZone::StatusBar,
                "Pairing state, sync freshness, and trust",
                M5SurfaceKind::HostChrome,
                true,
                "Keep the strip present and announce pairing/sync transitions.",
            ),
        ],
        vec![
            collapse(
                M5AdaptiveClass::CompactDesktop,
                &[M5ShellZone::RightInspector],
                M5FallbackPlacement::Sheet,
                "Companion actions move to an attached sheet; the mirrored working set stays \
                 full-width.",
            ),
            collapse(
                M5AdaptiveClass::StandardDesktop,
                NO_COLLAPSE,
                M5FallbackPlacement::Docked,
                "All zones stay docked.",
            ),
            collapse(
                M5AdaptiveClass::ExpandedDesktop,
                NO_COLLAPSE,
                M5FallbackPlacement::Docked,
                "All zones stay docked.",
            ),
        ],
        vec![
            missing(
                &id,
                "companion.paired_session",
                "session",
                M5ShellZone::MainWorkspace,
                M5PlaceholderClass::MissingRemote,
                "The last mirrored snapshot stays readable; live mirroring resumes when the paired \
                 session returns.",
            ),
            missing(
                &id,
                "companion.handoff_service",
                "handoff",
                M5ShellZone::TransientOverlay,
                M5PlaceholderClass::MissingProvider,
                "The mirrored view stays visible; device handoff resumes when the service returns.",
            ),
        ],
        vec![
            route(
                &id,
                "reopen_actions",
                M5LayoutRouteKind::Reopen,
                "Ctrl+Alt+A",
                "Reopen the companion actions and device handoff in the right inspector zone.",
            ),
            reset_route(&id),
        ],
    )
}

/// Builds the canonical reference-layout package (version 1.0.0).
///
/// Publishes one [`M5WorkspaceReferenceLayout`] per [`M5WorkspaceKind`] — notebooks, data grids,
/// the profiler, pipelines, docs, preview, incident, and companion surfaces — each describing how
/// the workspace occupies the governed shell zones, collapses responsively, degrades when a
/// dependency is missing, and reopens or resets.
pub fn seeded_m5_reference_layout_package() -> M5ReferenceLayoutPackage {
    M5ReferenceLayoutPackage {
        record_kind: M5_REFERENCE_LAYOUT_PACKAGE_RECORD_KIND.to_owned(),
        schema_version: M5_REFERENCE_LAYOUT_SCHEMA_VERSION,
        package_id: M5_REFERENCE_LAYOUT_PACKAGE_ID.to_owned(),
        package_version: M5_REFERENCE_LAYOUT_PACKAGE_VERSION.to_owned(),
        owner_role: PACKAGE_OWNER_ROLE.to_owned(),
        layouts: vec![
            notebook(),
            data_grid(),
            profiler(),
            pipeline(),
            docs(),
            preview(),
            incident(),
            companion(),
        ],
        proof_lane_ref: M5_REFERENCE_LAYOUT_PROOF_REF.to_owned(),
        release_packet_ref: M5_REFERENCE_LAYOUT_RELEASE_PACKET_REF.to_owned(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        summary_message_id: format!(
            "{}{}.summary",
            M5_REFERENCE_LAYOUT_MESSAGE_ID_PREFIX, M5_REFERENCE_LAYOUT_PACKAGE_ID
        ),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}
