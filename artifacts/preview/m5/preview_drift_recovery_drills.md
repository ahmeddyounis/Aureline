# M5 Preview Drift-Recovery Drills

- Packet: `m5-preview-drift-recovery-drills:stable:0001`
- Label: `M5 Preview Drift-Recovery Drills`
- Drills: 7 (5 degraded, 2 clean recovery)
- Events: 7 / 7

## Drills

- **drift-recovery:hot-reload-reset:0001** (hot_reload_reset)
  - Hot-reload reset discarded in-memory component state while the dev server stayed reachable and source-bound
  - event=hot_reload_reset surface=framework_pack_preview before[sync=in_sync_from_source map=exact data=live] after[sync=in_sync_from_source map=exact data=live]
  - recovery: reconnect_same_runtime
- **drift-recovery:stale-source-map:0001** (stale_source_map)
  - Source map drifted from the canonical source; source jumps are held below exact until the map is re-derived
  - event=stale_source_map surface=framework_pack_preview before[sync=in_sync_from_source map=exact data=live] after[sync=drifted_from_source map=stale data=live]
  - recovery: remap_source_then_reload,hold_inspect_only_until_remapped
  - Degraded: Source map is stale relative to the canonical source; jumps land near, not on, the target until it is re-derived
- **drift-recovery:dev-server-lost:0001** (dev_server_lost)
  - Dev server disappeared; the preview fell back to its last capture and dropped its live-runtime claim
  - event=dev_server_lost surface=preview_route before[sync=in_sync_from_source map=heuristic data=live] after[sync=pending_rebuild map=heuristic data=captured]
  - recovery: restart_runtime,reimport_capture_snapshot,export_metadata_only
  - Degraded: Dev server is unreachable; showing the last capture, not a live view — restart the runtime to resume live preview
- **drift-recovery:device-reconnect:0001** (device_reconnect)
  - Tethered device dropped and is re-attaching; the same device target is preserved while reconnect completes
  - event=device_reconnect surface=preview_route before[sync=in_sync_from_source map=heuristic data=live] after[sync=in_sync_from_source map=heuristic data=live]
  - recovery: reattach_device_session,reconnect_same_runtime
  - Degraded: Device session dropped and must reconnect; the view is held until the same device re-attaches
- **drift-recovery:browser-session-expired:0001** (browser_session_expired)
  - Browser-runtime session expired; the last frame is retained as a capture and the live claim is dropped
  - event=browser_session_expired surface=preview_route before[sync=in_sync_from_source map=exact data=live] after[sync=in_sync_from_source map=exact data=captured]
  - recovery: reconnect_same_runtime,export_metadata_only
  - Degraded: Browser session expired; showing a captured last frame until the session is re-established
- **drift-recovery:runtime-replaced:0001** (runtime_replaced)
  - Runtime was replaced by a different runtime; the view is marked drifted from source until it is rebuilt
  - event=runtime_replaced surface=preview_route before[sync=in_sync_from_source map=exact data=live] after[sync=drifted_from_source map=heuristic data=live]
  - recovery: rebuild_then_reload,reconnect_same_runtime
  - Degraded: A different runtime took over; the view has drifted from the canonical source and must be rebuilt before it is trusted
- **drift-recovery:data-posture-flip:0001** (data_posture_flip)
  - Data posture flipped from live to mock; the governed data chip changes while source-sync and target hold
  - event=data_posture_flip surface=notebook_adjacent_preview before[sync=in_sync_from_source map=exact data=live] after[sync=in_sync_from_source map=exact data=mock]
  - recovery: reconnect_same_runtime
