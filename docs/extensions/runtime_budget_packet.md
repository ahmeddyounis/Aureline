# Extension-runtime budget, activation, and quarantine evidence packet seed

This document is the narrative companion to the machine-readable
budget register at
[`/artifacts/extensions/runtime_budget_rows.yaml`](../../artifacts/extensions/runtime_budget_rows.yaml),
the quarantine and recovery rule register at
[`/artifacts/extensions/quarantine_rules.yaml`](../../artifacts/extensions/quarantine_rules.yaml),
and the worked activation evidence fixtures under
[`/fixtures/extensions/activation_cases/`](../../fixtures/extensions/activation_cases/).
It names the reserved extension-runtime budget axes, the
activation-evidence packet shape, and the quarantine and recovery
rules in a reviewer-friendly form. The machine rows are authoritative
when narrative and rows disagree; this document MUST be updated in
the same change that lands any row bump.

The seed is deliberately narrow. It does **not** land the extension
runtime, the extension host process, a sandbox implementation, a
crash-loop tracker, or a marketplace. Its job is to freeze the
runtime-budget vocabulary early enough that ecosystem growth does
not become invisible startup, battery, and memory debt once real
extension runtime work begins.

## What this seed freezes

1. A **seven-axis runtime-budget vocabulary** — extension discovery,
   cold activation, warm activation, idle polling, memory, network
   egress, and crash-loop behaviour — with per-axis owner,
   measurement method, and typed quarantine threshold classes.
2. An **activation-evidence packet shape**: host startup, host
   shutdown, capability narrowing, activation reason, resource-
   governor state at activation, and recorded budget counters at
   each phase. The packet is the single structured record every
   support export, benchmark harness, and install-review surface
   reads rather than ad-hoc profiler screenshots.
3. A **quarantine and recovery rule set** for runaway, leaking, or
   repeatedly crashing extensions, including throttle, disable,
   quarantine, restart-posture, visibility, and ranking-fairness
   rules.
4. **Parity hooks** binding the runtime-budget vocabulary to the
   shared resource-governor (`docs/runtime/resource_governor_contract.md`),
   efficiency-state policy (`docs/perf/efficiency_state_policy.md`),
   maintainer-coverage policy (`docs/governance/maintainer_coverage_policy.md`),
   and registry / offline-bundle seed
   (`docs/extensions/registry_and_offline_bundle_seed.md`) so
   performance, governance, registry, and support lanes all read one
   runtime-budget model.

## Record kinds

| Record kind                              | Purpose                                                                                                | Home                                                                                                     |
|------------------------------------------|--------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------|
| `extension_runtime_budget_row`           | One row per budget axis. Carries owner, measurement method, nominal budget class, and quarantine thresholds. | `/artifacts/extensions/runtime_budget_rows.yaml`                                                          |
| `extension_activation_evidence_packet`   | One packet per host-startup / activation / shutdown observation. Carries activation reason, negotiated worlds, recorded counters per axis, and resource-governor state. | `/fixtures/extensions/activation_cases/*.json`                                                            |
| `extension_quarantine_rule_row`          | One rule per quarantine trigger. Names trip condition, response class, restart posture, visibility posture, and discovery-ranking posture. | `/artifacts/extensions/quarantine_rules.yaml`                                                              |
| `extension_recovery_rule_row`            | One rule per recovery condition. Names how a throttled / disabled / quarantined extension returns to service. | `/artifacts/extensions/quarantine_rules.yaml`                                                              |

## Lifecycle checkpoint linkage (shared ids across surfaces)

The runtime-budget packet provides the **evidence** (axes, counters, audit
events, trigger rules) for extension activation and quarantine decisions. The
shell, crash-loop surfaces, runtime status, and support exports additionally
need a stable “what happened” sequence that does not vary by surface.

The shared lifecycle state and checkpoint catalog lives in:

- [`/artifacts/extensions/extension_lifecycle_states.yaml`](../../artifacts/extensions/extension_lifecycle_states.yaml)
- [`/docs/extensions/extension_lifecycle_and_quarantine_sequence.md`](./extension_lifecycle_and_quarantine_sequence.md)

Mapping guidance (non-exhaustive):

- Any cold/warm activation attempt SHOULD record
  `checkpoint.extension.lifecycle.pending_activation`, then either
  `checkpoint.extension.lifecycle.active` or a deny/disable/quarantine checkpoint.
- Any applied response with `response_class = disable_until_next_session` or
  `disable_until_user_explicit_reenable` SHOULD record
  `checkpoint.extension.lifecycle.disabled` and cite the firing
  `trigger_rule_id`.
- Any applied response with `response_class = quarantine` SHOULD record
  `checkpoint.extension.lifecycle.quarantined` and cite the firing
  `trigger_rule_id` (and a forensic packet ref when crash-loop driven).
- Recovery-triggered re-enablement SHOULD pair
  `checkpoint.extension.lifecycle.recovered` with
  `activation_reason_class = queued_replay_after_recovery` so the next activation
  attempt is attributable as a recovery replay.

## Budget axes

Every `extension_runtime_budget_row` binds one axis to:

- `axis_id` — one of the seven frozen axis ids below.
- `owner_role` — named surface owner who ratifies threshold bumps.
- `measurement_method` — how the counter is observed: host-sampled,
  process-reported, kernel-sampled, self-attributed, etc. Implementations
  MUST reuse one of the measurement classes named in the row register.
- `reporting_surface_class` — where the counter lands: activation
  evidence packet, support export, benchmark harness, or all of the
  above.
- `nominal_budget_class` — the typed class the axis targets in
  `efficiency_state = nominal`. Quantitative ceilings per class
  land in the successor ADR.
- `quarantine_threshold_class` — the typed class at which the axis
  fires a quarantine rule.
- `resource_governor_linkage` — which resource-governor pressure
  threshold in `artifacts/runtime/resource_governor_thresholds.yaml`
  this axis projects onto (CPU, I/O, memory, queue lane, or
  optional-service impairment).
- `efficiency_state_linkage` — which efficiency-state worker-budget
  rule in `artifacts/perf/worker_budget_rules.yaml` this axis
  inherits throttle order from.
- `crash_loop_posture` — whether the axis is strictly a volumetric
  budget, a crash-loop detector, or both.

The seven frozen axes:

| Axis id                                 | Covers                                                                                                |
|-----------------------------------------|-------------------------------------------------------------------------------------------------------|
| `discovery`                             | Manifest indexing, capability-world negotiation-packet setup, and contribution-point registration at host startup. |
| `cold_activation`                       | First-touch activation of an extension in this session: module load, world bind, initial handshake. |
| `warm_activation`                       | Subsequent activations where warm caches, already-negotiated worlds, and compiled-module caches exist. |
| `idle_polling`                          | Background timers, pollers, and subscription loops an extension keeps alive when it has no user-visible work. |
| `memory`                                | Resident-memory footprint per extension host, with soft and hard caps for sustained-steady-state and peak. |
| `egress`                                | Outbound network bytes per session and per wall-clock hour, scoped by `aureline:network-egress` allow-list. |
| `crash_loop`                            | Abnormal-exit, panic, OOM-kill, and hang detections across a rolling window.                          |

Additional axes are additive-minor with a `runtime_budget_rows_schema_version`
bump. Repurposing an axis id is breaking and requires a new decision
row.

### Quarantine-threshold classes

The closed vocabulary the schema pins:

- `none_axis_does_not_quarantine` — axis is reported for visibility
  only; downstream rules do not trip on it.
- `sustained_soft_breach_trips_throttle` — axis trips a
  throttle-class response.
- `sustained_hard_breach_trips_disable` — axis trips a disable-class
  response (per-host disable-until-next-session or user-explicit
  re-enable).
- `immediate_hard_breach_trips_disable` — axis trips a disable-class
  response on first breach within the sampling window.
- `sustained_hard_breach_trips_quarantine` — axis trips a
  quarantine-class response (extension is removed from discovery
  ranking and install-review until an admin or publisher-continuity
  action clears it).
- `crash_loop_threshold_trips_quarantine` — axis trips quarantine on
  crash-loop-window breach.

## Activation-evidence packet

Every extension-host startup, activation, and shutdown observation
that participates in a benchmark, support export, or install-review
claim MUST publish one `extension_activation_evidence_packet`. The
packet is the single source of truth; generic profiler screenshots,
ad-hoc timelines, or flame graphs MAY be attached as refs, but MUST
NOT substitute for the typed packet.

### Packet shape (summary)

- `packet_id` — opaque stable id for this observation.
- `observed_at` — ISO 8601 UTC monotonic timestamp.
- `observation_kind` — one of:
  - `host_startup`
  - `cold_activation`
  - `warm_activation`
  - `host_shutdown_clean`
  - `host_shutdown_crash`
  - `host_shutdown_oom`
  - `host_shutdown_hang_cancelled`
  - `idle_poll_sample`
- `extension_identity_ref` — namespaced extension id (ADR-0012).
- `extension_version_ref` — declared extension version.
- `host_contract_family` — ADR-0012 host-contract family.
- `negotiation_packet_ref` — ref to the host-negotiation packet
  (`schemas/extensions/host_negotiation.schema.json`) that governs
  this activation. Capability narrowing applied to this activation
  is read from that packet; the evidence packet MUST NOT restate
  capability scope.
- `activation_reason_class` — closed vocabulary:
  - `user_invoked_command`
  - `user_opened_workspace_containing_activation_event_binding`
  - `ide_focus_or_selection_event_binding`
  - `contributed_ui_surface_became_visible`
  - `queued_replay_after_recovery`
  - `scheduled_background_timer`
  - `dependency_extension_activation`
  - `explicit_admin_or_automation_trigger`
- `recorded_budget_counters` — one entry per budget axis with the
  observed counter value (or the opaque ref to the counter row when
  raw bytes/times would otherwise cross the boundary) and the
  typed pressure-class projection (`nominal`, `soft_breach`,
  `hard_breach`, `crash_loop_window_breach`, `not_applicable`).
- `resource_governor_state_ref` — governor state at activation
  (`nominal`, `constrained`, `degraded`, `protect_core`, `recovery`)
  drawn from `artifacts/runtime/resource_governor_thresholds.yaml`.
- `efficiency_state_ref` — efficiency state at activation from
  `artifacts/perf/worker_budget_rules.yaml`.
- `visibility_state_ref` — render-visibility state of the surface
  that triggered activation, when applicable.
- `audit_event_refs` — ordered list of audit events emitted during
  this observation. Reserved ids are listed below.
- `redaction_class` — one of `metadata_safe_default`,
  `support_bundle`, `benchmark_archive`.

Raw logs, raw core dumps, raw wasm-module bytes, raw stack traces,
raw network-payload bytes, and raw memory snapshots MUST NOT cross
this boundary. Every such field is an opaque ref.

### Per-phase counter contract

- `host_startup` packets carry `discovery` counters and
  `cold_activation` = 0 for every extension (no activation yet).
- `cold_activation` packets carry `cold_activation` counters and a
  negotiation-packet ref.
- `warm_activation` packets carry `warm_activation` counters and
  MUST cite the prior `cold_activation` packet id.
- `idle_poll_sample` packets carry `idle_polling`, `memory`, and
  `egress` counters over a frozen sampling window
  (default 60 s, declared in the row register).
- `host_shutdown_*` packets carry final `memory` and cumulative
  `egress` counters plus the `crash_loop` outcome class.

### Reserved audit events

Emitted on the ADR-0012 `extension_runtime` stream. Raw artifact
bytes, raw key material, and raw payload bodies MUST NOT appear on
any event.

- `extension_host_started`
- `extension_host_discovery_budget_breached_soft`
- `extension_host_discovery_budget_breached_hard`
- `extension_activated_cold`
- `extension_activated_warm`
- `extension_activation_denied_by_governor`
- `extension_activation_deferred_efficiency_state`
- `extension_idle_polling_budget_throttled`
- `extension_idle_polling_budget_paused`
- `extension_memory_soft_cap_breached`
- `extension_memory_hard_cap_breached`
- `extension_egress_budget_breached`
- `extension_crash_detected`
- `extension_hang_cancelled`
- `extension_oom_killed`
- `extension_quarantine_tripped`
- `extension_quarantined_recovery_attempted`
- `extension_quarantine_cleared`
- `extension_host_shutdown_clean`
- `extension_host_shutdown_forced`

## Quarantine and recovery

Every quarantine rule is a typed row in
`/artifacts/extensions/quarantine_rules.yaml`. Silent fallback to
"extension stopped working" chips, silent removal from discovery,
and silent disable across sessions are non-conforming; every
response class MUST emit the paired audit event and a repair
affordance.

### Response classes

- `throttle_background_work` — extension stays installed and
  user-invocable; background polling is budgeted to a lower cadence
  per `artifacts/perf/worker_budget_rules.yaml` extension-polling
  rules. User-invoked commands remain attributable.
- `disable_until_next_session` — extension is deactivated for the
  remainder of this host session. A warm restart of the host (at
  next session start) re-admits it under a fresh budget window.
- `disable_until_user_explicit_reenable` — extension is
  deactivated until the user or admin explicitly re-enables it.
  Cold restart of the host does not re-admit the extension.
- `quarantine` — extension is removed from discovery ranking and
  install-review admission. A quarantined row is surfaced on
  support export, install-review, permission-inspector, and
  publisher-continuity claim manifests verbatim.

### Restart-posture classes

- `no_restart_attempted` — quarantine or user-explicit-reenable
  class responses; the host does not auto-restart.
- `one_warm_restart_under_budget` — host attempts one warm restart
  within the crash-loop window; on failure promotes to
  `disable_until_next_session`.
- `exponential_backoff_bounded` — successive restarts use bounded
  exponential backoff (ceiling declared in the row register);
  exceeding the ceiling promotes to `disable_until_user_explicit_reenable`.

### Visibility and ranking-fairness postures

A quarantined or disabled extension MUST surface through the typed
visibility postures in `quarantine_rules.yaml`. Specifically:

- Install-review MUST surface the quarantine class and the trip
  reason class verbatim; hiding either denies with
  `review_disclosure_incomplete` (mirrors the registry seed).
- Discovery ranking MUST NOT bury a quarantined extension as a
  silently lower-ranked result; the extension is removed from
  ranking under `quarantine` and rendered as "unavailable in your
  organization / workspace" with the typed reason.
- Ranking fairness MUST NOT credit an extension's "installed-in-
  many-workspaces" signal when the extension is in an active
  quarantine or disable window; quarantine suppresses the signal
  rather than inverting the rank.
- Support export MUST carry the quarantine / disable rows and
  their audit-event refs so a maintainer or publisher can diagnose
  without reading raw logs.

### Recovery rules

Recovery rules are paired to response classes. Each rule names:

- `quarantine_response_class` being recovered from.
- `recovery_precondition_class` — one of `resource_governor_returned_to_nominal`,
  `efficiency_state_returned_to_nominal_or_recovery`,
  `admin_policy_cleared_quarantine`, `publisher_continuity_row_cleared`,
  `user_explicit_reenable`, `next_session_cold_start`,
  `crash_loop_window_cleared_without_breach`.
- `restart_posture_on_recovery` — from the restart-posture set
  above.
- `visible_projection_on_recovery` — one of `warming`, `partial`,
  `ready` (projected via the shared visible-health-state
  vocabulary in `artifacts/runtime/resource_governor_thresholds.yaml`).

A recovery attempt MUST NOT replay a queued user action that
predates the quarantine without an explicit user or admin re-run;
this mirrors the automation-contract rule against stale-authority
replay.

## Parity hooks

The runtime-budget packet is designed to share vocabulary rather
than mint new ones:

- **Resource governor.** Every budget axis declares a
  `resource_governor_linkage` so degraded / protect-core transitions
  already described in the governor contract apply without new
  enums. The governor is authoritative for visible-health-state
  projection; the budget packet does not mint a second vocabulary.
- **Efficiency state.** Every budget axis declares an
  `efficiency_state_linkage` so battery and thermal throttle order
  is inherited from the efficiency-state policy rather than restated.
- **Registry / offline-bundle seed.** Quarantine rows MAY cite a
  `publisher_continuity_ref` and `registry_source_class` from the
  registry seed; a quarantined-local-copy side-load continues to
  read the registry seed's `quarantined_local_copy_install_denied`
  denial reason.
- **Governance — maintainer coverage.** Rule rows whose
  `response_class` is `disable_until_user_explicit_reenable` or
  `quarantine` MUST cite a maintainer-coverage posture reference
  so a single-owner or unreviewed quarantine decision cannot ship
  silently.
- **Record-class registry.** The activation-evidence packet and the
  quarantine / recovery rows are recorded on the
  `extension_runtime_evidence` class. Retention, export, and
  redaction defaults are inherited from the record-class registry.

## Consumer expectations

The downstream surfaces below MUST read this seed rather than invent
runtime-budget shaped fields:

- **Extension host (when implemented).** Emit one
  `extension_activation_evidence_packet` per observation. Trip
  quarantine rules by row id rather than hard-coded thresholds.
- **Benchmark harness.** Read the packet directly; freeze no
  per-benchmark counter names. Add new counters through the row
  register.
- **Install / update review sheet.** Project the quarantine class,
  trip reason, and recovery pre-condition on every affected row.
- **Permission inspector.** Project the negotiated-world set from
  the cited `negotiation_packet_ref`; do not restate capability
  scope in the evidence packet.
- **Support export, mutation-journal entry, claim manifest.** Carry
  the packet id, extension identity, observation kind, and audit-
  event refs. Raw logs, raw core dumps, raw wasm-module bytes
  forbidden.
- **Discovery ranking.** Consume the quarantine posture as a binary
  admit / remove decision; never silently rerank.
- **Resource governor / efficiency state.** Read budget breaches
  via the axis's declared linkages; do not mint new thresholds.

## Out of scope

- Running an extension host, shipping a sandbox, a crash reporter,
  or a marketplace. This seed freezes the runtime-budget vocabulary,
  not the implementation.
- Quantitative ceilings per budget class. The classes are frozen;
  the per-class numeric ceilings land in the successor ADR and in
  the reference-hardware manifest.
- Crash-loop forensics beyond the packet and audit-event shape.
  Deep crash analysis is a separate lane.
- Publisher trust decisions, ranking algorithms, or marketplace
  policy. Quarantine is volumetric / behavioural, not reputational;
  reputational tiers remain a registry / governance concern.
