# M5 Session-Summary-Bar / Watch-Mode-Banner Primitive

- Packet: `m5-session-summary-watch-banner-primitive:stable:0001`
- Label: `M5 session-summary-bar / watch-mode-banner primitive: session mode, exact selection, target/environment shorthand, running/backlog/retry counts, distinct discovering/executing/watch-backlog/imported-refresh/settled activity postures, current watch state, controlled live/reduced/polling/reconnecting/paused/unavailable watch postures, explained degradation, preserved last successful cycle, and bounded reveal/rerun/cancel/open-watch and reveal/recover/pause/export actions`
- Status consumers: 5 (5 stable)
- Session postures: discovering_session, executing_session, watch_backlog_session, imported_refresh_session, settled_session
- Watch postures: live_watch, reduced_watch, polling_watch, reconnecting_watch, paused_watch, unavailable_watch
- Watch fidelity states: live, reduced, polling, unavailable, paused, reconnecting
- Proof freshness SLO: 720 hours (last refresh: 2026-07-07T00:00:00Z)

## Status consumers

- **Test Explorer Status Bar**: `stable`
  - Owner: Test explorer status owner
  - Scope: The test-explorer status bar renders the shared session-summary bar so a watch session executing a selected subset with a backlog and retries reads as an executing session (never a generic spinner) whose exact selection, running/backlog counts, retry lineage, and live watch state are all explicit, and it renders the shared watch banner so a full-fidelity live watch reads as a live watch that can be paused
  - Worked sessions: 2 / watches: 1
    - session `session:explorer::watch-auth-pricing` (`executing_tests`) → `executing_session` (in-progress `true`, watch-degraded `false`)
    - session `session:explorer::run-once-whole` (`settled_complete`) → `settled_session` (in-progress `false`, watch-degraded `false`)
    - watch `watch:explorer::local-live` (`live`) → `live_watch` (degraded `false`, explains `true`)
- **Editor Status Bar**: `stable`
  - Owner: Editor status owner
  - Scope: The editor status bar renders the shared session-summary bar so a settled run-once session reads as a settled session that offers rerun of its exact whole-suite selection, and it renders the shared watch banner so a reduced-fidelity watch reads as a reduced watch that explains its resource-pressure degradation, preserves its last successful cycle, and exposes both recover and pause — never a green banner over a degraded watch
  - Worked sessions: 1 / watches: 2
    - session `session:editor::debug-single-case` (`discovering_tests`) → `discovering_session` (in-progress `true`, watch-degraded `true`)
    - watch `watch:editor::container-reduced` (`reduced`) → `reduced_watch` (degraded `true`, explains `true`)
    - watch `watch:editor::user-paused` (`paused`) → `paused_watch` (degraded `false`, explains `true`)
- **Run Panel Status**: `stable`
  - Owner: Run panel status owner
  - Scope: The run-panel status renders the shared session-summary bar so a coverage session draining a watch backlog reads as a distinct watch-backlog session and an imported replay refreshing imported status reads as a distinct imported-refresh session — proving discovery, execution, watch-backlog, and imported-status refresh never share one loading treatment — and it renders the shared watch banner so a polling watch explains its adapter limitation
  - Worked sessions: 2 / watches: 1
    - session `session:run-panel::coverage-changed` (`processing_watch_backlog`) → `watch_backlog_session` (in-progress `true`, watch-degraded `true`)
    - session `session:run-panel::replay-nightly` (`refreshing_imported_status`) → `imported_refresh_session` (in-progress `true`, watch-degraded `true`)
    - watch `watch:run-panel::ci-polling` (`polling`) → `polling_watch` (degraded `true`, explains `true`)
- **Headless / CLI Status**: `stable`
  - Owner: Headless CLI status owner
  - Scope: The headless / CLI status renders the shared session-summary bar so a settled scheduled session reads as a settled session with an explicit whole-suite selection and no backlog, and it renders the shared watch banner so a reconnecting watch explains its lost file-watch handle, preserves its last successful cycle, and exposes recover and pause — proving the same status grammar works without a desktop surface
  - Worked sessions: 1 / watches: 1
    - session `session:headless::scheduled-nightly` (`settled_complete`) → `settled_session` (in-progress `false`, watch-degraded `true`)
    - watch `watch:headless::reconnecting` (`reconnecting`) → `reconnecting_watch` (degraded `true`, explains `true`)
- **Session / Watch Report Export**: `stable`
  - Owner: Session watch report export owner
  - Scope: The session / watch report export renders the shared session-summary bar so an errored discovery session with retries reads as a distinct discovering session whose retry lineage stays explicit, and it renders the shared watch banner so an unavailable watch explains its offline host, preserves its last successful cycle, and offers recover while honestly withholding a pause it cannot perform — the same status a reviewer reads in the tree and triage consumers
  - Worked sessions: 1 / watches: 1
    - session `session:report::failed-only-error` (`discovering_tests`) → `discovering_session` (in-progress `true`, watch-degraded `true`)
    - watch `watch:report::offline-unavailable` (`unavailable`) → `unavailable_watch` (degraded `true`, explains `true`)
