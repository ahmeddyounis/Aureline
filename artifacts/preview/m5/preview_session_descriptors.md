# M5 Preview-Session Descriptors

- Packet: `m5-preview-session-descriptors:stable:0001`
- Label: `M5 Preview-Session Descriptors`
- Sessions: 6 (2 downgraded)
- Surfaces: 4 / 4

## Sessions

- **preview-session:framework-pack:0001** (framework_pack_preview)
  - Framework-pack live preview rendered from the canonical source with an exact round-trip
  - session=source_bound_live_preview source_sync=in_sync_from_source data=live freshness=fresh origin=local_dev_server target=browser_tab_target
  - source_revision=`source_revision:preview-session:framework-pack:0001` runtime_backed=true write_capable=true
- **preview-session:preview-route:0001** (preview_route)
  - Preview-route session showing mock data over a source-bound live preview
  - session=source_bound_live_preview source_sync=in_sync_from_source data=mock freshness=fresh origin=local_dev_server target=browser_tab_target
  - source_revision=`source_revision:preview-session:preview-route:0001` runtime_backed=true write_capable=false
- **preview-session:notebook-adjacent:0001** (notebook_adjacent_preview)
  - Notebook-adjacent captured cell output replayed from a pinned source revision
  - session=snapshot_projection source_sync=in_sync_from_source data=captured freshness=fresh origin=imported_or_static_evidence target=design_renderer_target
  - source_revision=`source_revision:preview-session:notebook-adjacent:0001` runtime_backed=false write_capable=false
- **preview-session:preview-route:0002** (preview_route)
  - Preview-route session whose captured view drifted from source and is past its freshness SLO
  - session=snapshot_projection source_sync=drifted_from_source data=captured freshness=stale origin=imported_or_static_evidence target=design_renderer_target
  - source_revision=`source_revision:preview-session:preview-route:0002` runtime_backed=false write_capable=false
  - Downgraded: Captured preview is past its freshness SLO and has drifted from the canonical source; rebuild before relying on it
- **preview-session:preview-route:0003** (preview_route)
  - Preview-route runtime-only inspection of a remote runtime with no saved-source backing
  - session=runtime_backed_inspection source_sync=runtime_only_no_source data=live freshness=fresh origin=remote_or_container_runtime target=browser_tab_target
  - source_revision=`none` runtime_backed=true write_capable=false
- **preview-session:support-export:0001** (support_export_projection)
  - Support/export projection of a session whose data posture is not yet identified
  - session=snapshot_projection source_sync=in_sync_from_source data=unidentified freshness=fresh origin=imported_or_static_evidence target=viewport_preset_only
  - source_revision=`source_revision:preview-session:support-export:0001` runtime_backed=false write_capable=false
  - Downgraded: Data posture not yet identified for this projected session; held below a live/mock/captured chip until it is classified
