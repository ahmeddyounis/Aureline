# M5 Historical-Reference, Archived-Snapshot, Imported/Offline-Evidence, and Live-Target-Handoff Matrix

- Packet: `m5-historical-reference:stable:0001`
- Label: `M5 historical-reference, archived-snapshot, imported/offline-evidence, and live-target-handoff matrix`
- Object classes: 5 (5 stable)
- Historical-reference roles: snapshot_labeling, capture_time_attribution, provenance_attribution, mutation_blocked_posture, live_target_handoff, imported_offline_disclosure, expiry_removal_handling
- Capture-lifecycle stages: captured, snapshot_labeled, provenance_attributed, live_target_resolved, retention_or_removal_marked
- Proof freshness SLO: 720 hours (last audit: 2026-07-14T00:00:00Z)

## Object classes

- **retirement_snapshot**: `stable` (evidence_state: `archived_snapshot`)
  - Owner: Retirement-snapshot evidence owner (backup: Release-governance backup owner)
  - Canonical schema: `schemas/program/m5-historical-snapshot-descriptor.schema.json`
  - Scope: One retirement / last-supported snapshot is shown as captured evidence, not a live object: it carries the archived-snapshot label, the capture time and last-supported provenance, an explicit open-current-object handoff when a successor still exists, and a mutation-blocked posture so the snapshot can be inspected without being mistaken for current truth
  - Live-target availability: live target available via validated open-current-object handoff
  - Required labels: identity, historical_role, canonical_reference, snapshot_label
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **support_export_evidence**: `stable` (evidence_state: `archived_snapshot`)
  - Owner: Support / export evidence owner (backup: Support-governance backup owner)
  - Canonical schema: `schemas/program/m5-historical-snapshot-descriptor.schema.json`
  - Scope: One captured support / export evidence bundle is labeled as a snapshot with its capture context, shows its retention / expiry / removal state, and offers a metadata-only inspection exit when no live object remains so support can reopen the record without pretending it is editable or current
  - Live-target availability: no live target: metadata-only inspection exit
  - Required labels: identity, historical_role, canonical_reference, capture_time
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **archived_runbook_packet**: `stable` (evidence_state: `archived_snapshot`)
  - Owner: Archived-runbook evidence owner (backup: Runbook-governance backup owner)
  - Canonical schema: `schemas/program/m5-live-target-handoff.schema.json`
  - Scope: One archived runbook execution packet is labeled as a historical run with its capture time and provenance, and any open-live-run action first validates target identity, trust, route, and authority so an archived run is never silently re-executed as if it were live
  - Live-target availability: live run target available via validated open-live-run handoff
  - Required labels: identity, historical_role, canonical_reference, live_target_availability
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **imported_offline_route_evidence**: `stable` (evidence_state: `imported_offline_evidence`)
  - Owner: Imported / offline route-evidence owner (backup: Continuity-governance backup owner)
  - Canonical schema: `schemas/program/m5-imported-offline-evidence-state.schema.json`
  - Scope: One imported / offline route-evidence record carries its imported / offline-only warning, its import context, a controlled restore-fidelity disclosure, and any current live-route mismatch so imported route data never masquerades as current live route, service, or workspace truth
  - Live-target availability: live route target unavailable offline: metadata-only inspection exit
  - Required labels: identity, historical_role, canonical_reference, live_target_availability
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
- **review_incident_snapshot**: `stable` (evidence_state: `archived_snapshot`)
  - Owner: Review / incident snapshot owner (backup: Incident-governance backup owner)
  - Canonical schema: `schemas/program/m5-live-target-handoff.schema.json`
  - Scope: One review / incident snapshot is labeled as captured evidence with its capture time and provenance, holds a mutation-blocked posture, and offers an open-current-object handoff validated for identity, trust, route, and authority so a review snapshot is never edited or reopened as if it were the live object
  - Live-target availability: current object available via validated open-current-object handoff
  - Required labels: identity, historical_role, canonical_reference, snapshot_label
  - Accessibility routes: keyboard_focusable, screen_reader_announced, high_zoom_reflow, high_contrast_safe, cli_exportable, support_packet_present
