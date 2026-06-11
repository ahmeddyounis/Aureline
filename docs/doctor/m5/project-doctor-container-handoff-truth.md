# Project Doctor container handoff truth

This document describes the canonical packet that keeps a container route's
**share truth** first-class once it leaves the open-in-container preflight. It is
the user-facing companion to the governed artifact at
`artifacts/doctor/m5/project-doctor-container-handoff-truth.json`, the schema at
`schemas/doctor/project-doctor-container-handoff-truth.schema.json`, and the
typed model in the `aureline-doctor` crate
(`ship_published_port_or_tunnel_revocation_writable_mount_and_lifecycle_script_disclosure_and_browser_or_companion_handoff`).

It answers a single question for every published port, tunnel, or handoff:
**after a container route is published, can a user see and revoke exactly what is
exposed (to whom, until when, under which policy), can they tell what environment
mutation the reopen actually carries, and does the browser or companion handoff
say who and what it points at — instead of behaving like a durable opaque
share?**

## What each scenario pins

Every scenario seeds one container-route share situation in one workflow surface
(`remote_preview`, `incident_workflow`, or `companion_follow`) and records, in a
single rerunnable record:

- **Which route, to whom, until when, revocable how?** A `route` naming the
  `route_kind` (`published_port` or `tunnel`), the `target_ref`/
  `target_service_ref` identity and the `container_port`/`host_port`, the
  `audience_scope` (`local_only`, `lan`, `authenticated_team`, `org`, `public`),
  the `policy_posture` (`policy_allowed`, `policy_restricted`, `policy_blocked`),
  a `time_bound` whose `class` (`session_bound`, `time_boxed`, `deadline`) and
  non-empty `expires_at_ref` mean the route is **never unbounded**, and a
  `revocation` block whose non-empty `revocation_action_ref` keeps the revocation
  path **first-class** and whose `state` (`active`, `expired`, `revoked`) carries
  `revoked_evidence_ref` once the route is dead.
- **What does the reopen mutate?** An `environment_disclosure` listing the
  `writable_mounts` and `lifecycle_scripts` (lifecycle hooks and install
  scripts, each flagged `trust_gated` and `writes_outside_container`) plus the
  `disclosure_persists_in` flows — `reopen`, `attach`, `rebuild`, `export`, and
  `support_bundle` — so writable mounts and lifecycle/install scripts are never
  reduced to undocumented side effects.
- **Who does the handoff point at?** A `handoff` attributing the browser or
  companion share to its `owner_ref`/`origin_ref`, `engine_class`, `target_ref`,
  `route_id`, and `target_service_ref`, plus its `liveness` (`live` vs
  `snapshot`), `captured_at_ref`, `revocation_visible` flag, `mutation_scope`
  (`read_only` vs `bounded_write`), and `approval_gated` flag — so a handoff is
  attributable truth, never a flattened opaque URL.

## The non-inheriting handoff gate

Each scenario publishes a `published_handoff_posture` (`share_live`,
`share_with_disclosure`, `share_snapshot_only`, or `blocked_offer_alternative`)
and a `published_handoff_reason`. Those values are **not asserted by hand** — they
are validated against the posture the gate recomputes from the scenario's own
state, in this precedence:

1. route `policy_blocked` → `blocked_offer_alternative` / `policy_blocked`
2. route `revoked` → `share_snapshot_only` / `route_revoked`
3. route `expired` → `share_snapshot_only` / `route_expired`
4. handoff `bounded_write` → `share_with_disclosure` /
   `bounded_write_requires_approval`
5. audience `public` → `share_with_disclosure` / `audience_public`
6. `policy_restricted` → `share_with_disclosure` / `policy_restricted`
7. any writable mount or trust-gated / external-write script →
   `share_with_disclosure` / `environment_mutation_disclosed`
8. otherwise → `share_live` / `none`

Because the posture is recomputed per scenario, no scenario inherits "share live"
from an adjacent one. A `blocked_offer_alternative` scenario always carries a
non-empty stay-local alternative (never a dead end); a `revoked` or `expired`
route always collapses to a snapshot handoff with visible revocation (never a
live opaque share); and a `bounded_write` handoff is always `approval_gated`
(there is no unrestricted mutate channel). A `share_live` scenario is
independently guarded to require an active, policy-allowed, non-public,
read-only, live share with no disclosed environment mutation.

## Cross-surface parity

Every scenario renders on all seven `parity_surfaces` — `desktop_sheet`,
`cli_inspect`, `headless_json`, `browser_handoff`, `companion_handoff`,
`support_export`, and `incident_packet` — and carries the locale-invariant
`machine_meaning_keys` (`scenario_id`, `surface`, `engine_class`, `route_kind`,
`handoff_posture`) so localized prose can never change what a surface means. The
`support_linkage` block preserves stable identity (`bundle_manifest_ref`,
`escalation_packet_ref`, `preserved_finding_ids`, `preserved_scope_refs` covering
route/port/tunnel/mount/target) and is metadata-safe by construction:
`redaction_class` is always `metadata_safe_default`, and raw private material and
content overcapture are always excluded.

## Reading the packet from code

```rust
use aureline_doctor::ship_published_port_or_tunnel_revocation_writable_mount_and_lifecycle_script_disclosure_and_browser_or_companion_handoff::{
    current_project_doctor_container_handoff_truth, HandoffPosture,
};

let packet = current_project_doctor_container_handoff_truth()?;
assert!(packet.validate().is_empty());

for scenario in &packet.scenarios {
    // The published gate posture always equals the recomputed one.
    assert_eq!(
        (scenario.published_handoff_posture, scenario.published_handoff_reason),
        scenario.recompute_gate(),
    );
}
# Ok::<(), serde_json::Error>(())
```

The corpus is checked in and rerunnable: the typed model embeds the artifact via
`include_str!`, so the crate's tests and any CI gate validate the same rows
without re-deriving them.
