# Fixtures: M5 host-boundary matrix

This directory contains fixture metadata for the `m5_host_boundary_matrix` packet.

The canonical full corpus is checked in at:

`artifacts/execution/m5/m5-host-boundary.json`

## Coverage

- `notebook_run`, `preview_session`, `framework_action`, `profiler_capture`,
  `request_runtime_mutation`, `incident_resource_action`, `managed_workspace_run`, and
  `service_plane_action` are the only claimed execution lanes, and each carries exactly
  one row — no lane inherits a confirmed origin from an adjacent one.
- Each lane carries its own host-identity, origin-receipt, context-strip, execution,
  and support-export ref; every rebound lane also carries a previous-host and
  rebind-diff ref so the host change is reviewable instead of silently replacing the
  current host.
- Host kind covers `local`, `ssh`, `container`, `managed_workspace`, `browser_bridge`,
  and `service_plane`; origin locus covers `local`, `remote`, `managed`, `bridged`,
  and `service_plane`. The published locus is pinned to the host kind, so a remote,
  managed, bridged, or service-plane host can never imply local execution.
- Published attribution covers `confirmed`, `attributed`, `provisional`, `stale`, and
  `unattributed`, and the boundary decision covers `publish`, `narrow`,
  `flag_for_review`, and `withhold`.
- Origin receipt covers `signed`, `recorded`, `inferred`, and `missing`; connection
  covers `connected`, `bridged`, `reconnecting`, and `stale`; host binding covers
  `bound`, `rebound`, and `unbound`; export continuity covers `continuous`, `partial`,
  and `broken`.
- The six canonical narrowing reasons — `missing_origin_receipt`, `bridged_boundary`,
  `reconnecting_host`, `stale_context`, `unbound_host`, and `export_continuity_broken`
  — are each exercised by at least one lane.
- The attribution gate is exercised in every direction: the clean local `notebook_run`
  and the fully-attributed `managed_workspace_run` publish confirmed origins; the
  `preview_session`, `framework_action`, `profiler_capture`, and
  `incident_resource_action` lanes narrow to a lower attribution; the
  `request_runtime_mutation` lane flags its bridged boundary for review; and the
  receiptless, export-broken `service_plane_action` has its origin withheld. The
  `request_runtime_mutation` row is the guardrail case — a browser bridge whose locus
  stays `bridged` and is flagged so it cannot imply local execution — while the
  `managed_workspace_run` row proves a remote/managed host is not penalized for being
  remote: it publishes a confirmed origin with a `managed` locus. Each row's
  `published_attribution`, `published_locus`, `boundary_decision`, and
  `narrowing_reasons` equal the recomputed gate decision, so release and desktop/CLI
  tooling can prove underqualified lanes narrow before publication.
