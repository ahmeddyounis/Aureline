# First-useful-work entry routes for M5 depth lanes

Generated from the seeded packet in
[`crate::m5_entry_routes`](../../../crates/aureline-shell/src/m5_entry_routes/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_entry_routes -- markdown > \
  artifacts/ux/m5/first-useful-work-packets/m5_entry_routes_packet.md
```

- Packet id: `shell:m5_entry_routes:v1:default`
- Routes: 10
- Lanes covered: 10/10
- No hidden prerequisites: true
- No raw sensitive user content: true
- Generated at: `2026-06-11T00:00:00Z`

## Lane coverage

| Lane | Entry | Landing | Local-core fallback | Hidden prereq | First useful work |
|---|---|---|:---:|:---:|---|
| Notebook | `single_file_open` | `file_editor_with_root_cues` | yes | none | `open_and_inspect_locally` (immediate_local_open) |
| Request workspace | `folder_or_repo_open` | `explorer_plus_readme_or_changed_files` | yes | none | `open_and_inspect_locally` (immediate_local_open) |
| Database workspace | `folder_or_repo_open` | `explorer_plus_readme_or_changed_files` | yes | none | `open_and_inspect_locally` (immediate_local_open) |
| Profiler / trace capture | `single_file_open` | `file_editor_with_root_cues` | yes | none | `inspect_captured_artifact` (after_local_index) |
| Framework pack | `folder_or_repo_open` | `generic_shell_with_diagnostics` | yes | none | `browse_catalog_locally` (immediate_local_open) |
| Docs / local browser | `review_or_incident_deep_link` | `linked_review_incident_or_work_item` | yes | none | `read_local_content` (immediate_local_open) |
| Preview routes | `folder_or_repo_open` | `explorer_plus_readme_or_changed_files` | yes | none | `open_and_inspect_locally` (immediate_local_open) |
| Companion handoff | `imported_state_or_handoff_packet` | `import_compare_or_restore_sheet` | yes | none | `review_packet_locally` (after_explicit_user_choice) |
| Managed sync | `restore_last_session` | `restored_layout_with_placeholders` | yes | none | `open_and_inspect_locally` (immediate_local_open) |
| Offboarding | `restore_last_session` | `import_compare_or_restore_sheet` | yes | none | `review_plan_before_commit` (after_explicit_user_choice) |

## Open a notebook and inspect cells without a kernel (`notebook`)

Opening a notebook lands directly in a read-only inspection of cells and captured outputs, so the user reaches first useful work before deciding whether to start a kernel.

- Local-core fallback: The notebook opens read-only with cells, outputs, and structure visible; no kernel is started.
- Setup-later actions: `set_up_later`, `inspect_only`
- Not yet done:
  - `no_kernel_started` — No notebook kernel has been started; cached outputs are shown as captured.
- Optional enrichments (never required):
  - `start_kernel` — Start a kernel to execute cells when you are ready.

## Open a request workspace and inspect requests without sending (`request_workspace`)

A request workspace opens with definitions and saved responses visible, so the user can read and learn the surface before sending anything over the network.

- Local-core fallback: Request definitions, environments, and history open for inspection; no request is sent.
- Setup-later actions: `set_up_later`, `inspect_only`
- Not yet done:
  - `no_request_sent` — No request has been sent; saved responses are shown as previously captured.
- Optional enrichments (never required):
  - `attach_provider` — Attach an environment or secret store to send live requests.

## Open a database workspace and inspect schema without connecting (`database_workspace`)

A database workspace opens to saved connection definitions and cached schema, so the user can inspect and learn the surface before opening a live connection.

- Local-core fallback: Saved connection definitions and cached schema open for inspection; no database connection is opened.
- Setup-later actions: `set_up_later`, `inspect_only`
- Not yet done:
  - `no_database_connected` — No database connection has been opened; cached schema is shown as last captured.
- Optional enrichments (never required):
  - `connect_database` — Connect to a database to run live queries when you are ready.

## Inspect a captured profile or trace without running a capture (`profiler_trace_capture`)

Opening a captured profile or trace lands in an inspection view of the recorded data, so the user reaches first useful work without running a new, side-effecting capture.

- Local-core fallback: A previously captured profile or trace opens for inspection; no new capture is run.
- Setup-later actions: `set_up_later`, `inspect_only`
- Not yet done:
  - `no_trace_captured` — No profiler or trace capture has been run; the existing capture is shown as recorded.
- Optional enrichments (never required):
  - `run_trace_capture` — Run a new profiler or trace capture when you are ready.

## Browse the framework-pack catalog without installing (`framework_pack`)

The framework-pack surface opens to a browsable local catalog of capabilities and scope, so the user can learn what each pack does before installing one.

- Local-core fallback: Framework-pack descriptions, capabilities, and scope open for browsing; no pack is installed.
- Setup-later actions: `set_up_later`, `dismiss_recommendation`
- Not yet done:
  - `no_framework_pack_installed` — No framework pack has been installed; capabilities are described from the local catalog.
- Optional enrichments (never required):
  - `install_framework_pack` — Install a framework pack to enable its surfaces when you are ready.

## Read local docs without browser authentication (`docs_browser`)

Docs and the local browser open bundled and cached content for reading, so the user reaches first useful work without any browser sign-in.

- Local-core fallback: Bundled and cached docs open for reading; the embedded browser is not authenticated.
- Setup-later actions: `set_up_later`, `open_minimal`
- Not yet done:
  - `no_browser_auth_completed` — No browser authentication has been completed; only local and cached docs are shown.
- Optional enrichments (never required):
  - `browser_auth` — Authenticate the embedded browser to reach gated documentation when you are ready.

## Inspect preview route definitions without exposing a route (`preview`)

Preview opens to a read-only view of route definitions and their scope, so the user can learn the surface before exposing any route.

- Local-core fallback: Preview route definitions and scope open for inspection; no preview route is exposed.
- Setup-later actions: `set_up_later`, `inspect_only`
- Not yet done:
  - `no_preview_route_exposed` — No preview route has been exposed; route definitions are shown without serving.
- Optional enrichments (never required):
  - `expose_preview_route` — Expose a preview route to serve it when you are ready.

## Review a companion handoff packet without joining (`companion_handoff`)

A companion handoff packet opens for local review and comparison, so the user can inspect what would transfer before joining a companion device.

- Local-core fallback: The handoff packet opens for local review and comparison; no companion device is joined.
- Setup-later actions: `set_up_later`, `compare_before_restore`
- Not yet done:
  - `no_companion_joined` — No companion device has been joined; the handoff packet is reviewed locally only.
- Optional enrichments (never required):
  - `join_companion` — Join the companion device to continue the handoff when you are ready.

## Inspect managed sync state without joining sync (`managed_sync`)

Managed sync opens to a local inspection of sync state and what would be shared, so the user can learn the surface and continue locally without signing in.

- Local-core fallback: Local sync state and what would be shared open for inspection; managed sync is not joined.
- Setup-later actions: `set_up_later`, `continue_in_restricted_mode`
- Not yet done:
  - `no_sync_joined` — No managed sync has been joined; only local state is shown and nothing is uploaded.
- Optional enrichments (never required):
  - `sign_in_for_managed_sync` — Sign in for managed sync to share state across devices when you are ready.

## Review an offboarding plan without committing an action (`offboarding`)

Offboarding opens to a review of the plan and an export preview, so the user can understand the full effect before committing any irreversible action.

- Local-core fallback: The offboarding plan and export preview open for review; no irreversible offboarding action is committed.
- Setup-later actions: `set_up_later`, `compare_before_restore`
- Not yet done:
  - `no_offboarding_action_committed` — No offboarding action has been committed; the plan and export preview are review-only.
- Optional enrichments (never required):
  - `commit_offboarding_export` — Commit the offboarding export after reviewing the plan when you are ready.

