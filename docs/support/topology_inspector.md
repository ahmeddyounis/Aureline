# Support Topology Inspector Contract

This contract defines the metadata-only support/export packet that mirrors the
runtime topology inspector for M5 host families.

## Scope

The packet preserves:

- visible result-to-host mappings for notebook, preview, profiler/replay,
  data/API, AI tool, provider-run, pipeline, and remote-backed surfaces
- host descriptors: family, role, locality, inline boundary badges, health, and
  fault-domain ownership
- restart-budget truth: strike count, strike window, budget state, preserved
  checkpoints, stale visible artifacts, and next quarantine trigger
- reattach review state, crash-loop or quarantine banners, and lane-filtered
  event provenance
- explicit visible-truth labels instead of implicit stale state

## Required labels

Support/export rows must preserve the same user-facing labels as the shell:

- `stale`
- `rebuilding`
- `provider unavailable`
- `reconnecting`
- `local fallback`
- `captured snapshot`

## Boundaries

The packet is metadata-only:

- raw command lines, payload bodies, environment values, prompts, responses,
  paths, and secrets do not cross this boundary
- crash-loop and restart evidence remains linked by stable ids and refs
- current-vs-stale truth must stay explicit; export packets may not collapse a
  degraded lane back to `current`

## Primary records

- `fault_domain_view_packet`
- `fault_domain_topology_result_row`
- `fault_domain_view_row`
- `fault_domain_restart_card_record`
- `reattach_review_sheet_record`
- `crash_loop_quarantine_banner_record`
- `lane_filtered_event_viewer_record`
