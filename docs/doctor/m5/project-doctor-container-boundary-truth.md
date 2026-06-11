# Project Doctor container boundary truth

This document describes the canonical packet that makes the **container and
devcontainer boundary** legible in M5 remote-preview and incident workflows. It
is the user-facing companion to the governed artifact at
`artifacts/doctor/m5/project-doctor-container-boundary-truth.json`, the boundary
schema at
`schemas/doctor/project-doctor-container-boundary-truth.schema.json`, and the
typed model in the `aureline-doctor` crate
(`container_engine_summaries_open_in_container_preflights_and_route_time_bound_truth`).

It answers a single question for every container-dependent action: **before a
user reopens, rebuilds, or attaches, can they tell which engine and container
class is active, what target is being rebuilt or attached, which hooks, mounts,
and ports are involved, and what happens to the boundary and the logs — without
being stranded in a dead-end modal when the engine is unreachable or
policy-blocked?**

## What each scenario pins

Every scenario seeds one container-dependent situation in one M5 workflow
surface (`remote_preview` or `incident_workflow`) and records, in a single
rerunnable record:

- **Which engine, and is it usable?** An `engine_summary` naming the
  `engine_class` (`docker`, `podman`, `devcontainers_cli`, `managed_cloud`), its
  `reachability` (`reachable`, `unreachable`, `policy_blocked`), its
  `support_class` (`certified`, `supported`, `experimental`, `unsupported`), a
  human `certification_note`, and the `diagnostics_actions` offered. An engine
  that is not reachable must still offer at least one diagnostics action, so the
  user is never stranded.
- **Which boundary and target?** A `workspace_mode` (`attached_container`,
  `devcontainer`, `remote_managed`), a `boundary_label` (`local`, `remote`,
  `managed`) consistent with that mode, and a non-empty `target_ref` — so a
  window is never treated as merely "in a container" without naming the engine,
  the boundary, and the target identity being rebuilt or attached.
- **What will the reopen do?** A `rebuild_review` preflight that discloses the
  `definition_source`, the `rebuild_decision` (`rebuild` vs `reuse_existing`),
  the `lifecycle_hooks` (and which are `trust_gated`), `extension_installs`,
  `published_ports`, `writable_mounts`, `affected_services`, `affected_images`,
  and a non-empty `stay_local_alternative` so no flow is a dead end.
- **What is the log truth?** A `log_truth` block with an `availability` (`live`,
  `buffered`, `snapshot`, `unavailable`), an export-safe `export_time_range_ref`
  (present whenever logs are available), a metadata-safe `redaction_posture`, and
  a human `truth_note` — so logs reach incident and preview flows as labeled,
  time-bound, export-safe truth rather than context-free streams.

## The non-inheriting preflight gate

Each scenario publishes a `published_preflight_decision` (`proceed_full`,
`proceed_with_disclosure`, or `blocked_offer_alternative`) and a
`published_preflight_reason`. Those values are **not asserted by hand** — they are
validated against the decision the gate recomputes from the scenario's own state,
in this precedence:

1. engine `policy_blocked` → `blocked_offer_alternative` / `policy_blocked`
2. engine `unreachable` → `blocked_offer_alternative` / `engine_unreachable`
3. engine `unsupported` → `proceed_with_disclosure` / `unsupported_engine`
4. any trust-gated lifecycle hook → `proceed_with_disclosure` / `trust_gated_hooks`
5. any rebuild/port/writable-mount/extension side effect →
   `proceed_with_disclosure` / `side_effects_require_review`
6. otherwise → `proceed_full` / `none`

Because the decision is recomputed per scenario, no scenario inherits "proceed"
from an adjacent one. A `blocked_offer_alternative` scenario always carries a
non-empty stay-local alternative (never a dead end), and a trust-gated hook can
never coexist with a `proceed_full` decision — trust-gated hooks never run
silently. A `proceed_full` scenario is independently guarded to require a
reachable, supported engine with no trust-gated hooks and no side effects.

## Cross-surface parity

Every scenario renders on all six `parity_surfaces` — `desktop_sheet`,
`cli_inspect`, `headless_json`, `browser_handoff`, `support_export`, and
`incident_packet` — and carries the locale-invariant `machine_meaning_keys`
(`scenario_id`, `surface`, `engine_class`, `workspace_mode`,
`preflight_decision`) so localized prose can never change what a surface means.
The `support_linkage` block preserves stable identity (`bundle_manifest_ref`,
`escalation_packet_ref`, `preserved_finding_ids`, `preserved_scope_refs` covering
mount/port/tunnel/target) and is metadata-safe by construction: `redaction_class`
is always `metadata_safe_default`, and raw private material and content
overcapture are always excluded.

## Reading the packet from code

```rust
use aureline_doctor::container_engine_summaries_open_in_container_preflights_and_route_time_bound_truth::{
    current_project_doctor_container_boundary_truth, PreflightDecision,
};

let packet = current_project_doctor_container_boundary_truth()?;
assert!(packet.validate().is_empty());

for scenario in &packet.scenarios {
    // The published gate decision always equals the recomputed one.
    assert_eq!(
        (scenario.published_preflight_decision, scenario.published_preflight_reason),
        scenario.recompute_gate(),
    );
}
# Ok::<(), serde_json::Error>(())
```

The corpus is checked in and rerunnable: the typed model embeds the artifact via
`include_str!`, so the crate's tests and any CI gate validate the same rows
without re-deriving them.
