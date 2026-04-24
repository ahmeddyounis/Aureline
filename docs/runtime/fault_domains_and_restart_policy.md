# Fault domains, supervision tree, restart budgets, and forensic packets

This document freezes Aureline's failure-containment contract into a
concrete, named set of fault domains, restart policies, authority
revocation rules, visible escalation states, and forensic packet
requirements. It exists so restart and recovery posture is part of
the architecture freeze — not a later reliability patch bolted onto
an otherwise unsupervised process tree.

The packet has four parts:

- this narrative, which names the seven fault domains, their
  supervision lines, restart strike windows, authority/capability
  revocation rules, visible `Degraded`/`Disabled`/`Quarantined`
  escalation states, and forensic-packet requirements;
- [`/artifacts/runtime/restart_budgets.yaml`](../../artifacts/runtime/restart_budgets.yaml),
  the machine-readable per-domain strike windows, budgets,
  escalation ceilings, and ticket-revocation rules;
- [`/schemas/runtime/forensic_packet.schema.json`](../../schemas/runtime/forensic_packet.schema.json),
  the boundary schema for the forensic packet every supervised host
  emits on exit, stall, or degraded-state transition; and
- [`/fixtures/runtime/fault_domain_cases/`](../../fixtures/runtime/fault_domain_cases),
  worked case fixtures covering shell loss, knowledge worker
  quarantine, session helper crash-loop, extension host disable,
  AI broker transient failure, remote connector disconnect, and
  collector back-pressure.

## Companion artifacts

- [`/artifacts/architecture/runtime_host_classes.yaml`](../../artifacts/architecture/runtime_host_classes.yaml)
  — the twelve canonical host classes, their failure-isolation
  classes, restart classes, state-authority classes, allowed and
  forbidden capabilities, IPC families, and visibility surfaces.
  Fault-domain rows in this packet bind host-class ids from that
  file to a fault domain.
- [`/artifacts/architecture/service_ownership_matrix.yaml`](../../artifacts/architecture/service_ownership_matrix.yaml)
  — the host-class, service-sub-plane, and M0-artifact-family
  ownership map. Restart decisions route through the supervising
  host class declared there.
- [`/artifacts/architecture/process_placement_map.yaml`](../../artifacts/architecture/process_placement_map.yaml)
  — scheduling classes, failure policies per process role, and
  inline-work policy. Restart budgets in this packet do not widen
  inline-work policy.
- [`/docs/runtime/resource_governor_contract.md`](./resource_governor_contract.md)
  — shared resource-governor health-state vocabulary
  (`Nominal`, `Constrained`, `Degraded`, `ProtectCore`, `Recovery`)
  and the visible `warming`/`partial`/`degraded`/`offline`/
  `unsupported`/`overloaded` vocabulary. This packet reuses those
  tokens for escalation projection rather than minting parallel
  states.
- [`/artifacts/runtime/resource_governor_thresholds.yaml`](../../artifacts/runtime/resource_governor_thresholds.yaml)
  — machine-readable threshold families and shed order the
  governor reads in tandem with per-domain restart budgets.
- [`/docs/security/emergency_action_model.md`](../security/emergency_action_model.md)
  — channel-freeze, kill-switch, and revocation semantics.
  Emergency actions MAY trip or lock a restart budget; the record
  id is surfaced in the forensic packet and never replaced by an
  ad-hoc reason string.
- [`/docs/runtime/execution_context_vocabulary.md`](./execution_context_vocabulary.md)
  — authority envelope and scope vocabulary the forensic packet
  quotes when it captures a last-good execution context.

## Why this exists

The repo already freezes:

1. **Process topology.** Twelve host classes with per-class
   failure isolation, restart class, state authority, and allowed
   capabilities in `runtime_host_classes.yaml`.
2. **Ownership.** The supervision tree rooted at `local_supervisor`
   and the diagnostic/export packet owner pinned to
   `observability_collector` in `service_ownership_matrix.yaml`.
3. **Shared overload posture.** One resource-governor state
   machine and visible health-state vocabulary in
   `resource_governor_contract.md`.

What is still missing is one place that answers four operational
questions at the same time:

- **Which fault domain owns a given host class's restart decisions
  and strike accounting?**
- **What restart budget, escalation ceiling, and authority/capability
  revocation rule applies before a host is rehydrated?**
- **What does repeated crash or stall look like as visible state
  (`Degraded`, `Disabled`, or `Quarantined`) rather than as an
  invisible respawn loop?**
- **What forensic data MUST every exit, stall, or escalation emit
  so support and recovery tasks can replay the incident without
  reaching into private host memory?**

This document and its three companion files are that join point.
They do not invent host classes, visible states, or export owners;
they freeze the restart and forensic story around the vocabulary
the rest of the repo already owns.

## Scope and non-goals

**In scope.**

- A closed taxonomy of fault domains that covers every restartable
  host class.
- A restart-policy contract for each domain with strike windows,
  budgets, escalation ceilings, and ticket revocation rules.
- A forensic-packet contract for every exit, stall, escalation, and
  recovery transition.
- Visible-state semantics for repeated failure (`Degraded`,
  `Disabled`, `Quarantined`, `Lost`) and recovery (`Warming`,
  `Ready`) that reuse the resource-governor vocabulary.

**Out of scope.**

- The full production process-supervisor implementation (timers,
  cgroup wiring, per-OS launcher glue, cryptographic signing of
  forensic packets, durable history retention policy). Those
  decisions remain for a later milestone. This packet is the
  contract they implement against.
- Registry and marketplace revocation transport, live signing
  infrastructure, or any networked supervisor coordination.
- New host classes, new state-authority classes, or new
  visibility surfaces. Any of those must be added by ADR in the
  host-class file first.

## Fault-domain taxonomy (frozen at seven)

A *fault domain* is the unit over which Aureline accounts for
restart budgets, applies an escalation ceiling, and owns the
"visible repeated failure" story. Every restartable host class
belongs to exactly one fault domain. Adding a fault domain is an
ADR-gated change that MUST update this document,
`restart_budgets.yaml`, `runtime_host_classes.yaml`, and the
`service_ownership_matrix.yaml`.

| Fault domain id | Title | Host classes | Primary supervisor | Summary |
| --- | --- | --- | --- | --- |
| `shell_interaction_core` | Shell interaction core | `desktop_shell` | self | One per desktop session; never silently restarted. Loss is visible app loss. |
| `workspace_knowledge_workers` | Workspace knowledge workers | `index_workers` | `local_supervisor` | Cancellable worker pool. Partial / stale answers are surfaced; crash-loop projects a visible `degraded` knowledge lane. |
| `session_scoped_execution_hosts` | Session-scoped execution hosts | `language_hosts`, `debug_hosts`, `git_worker`, `pty_host` | `local_supervisor` | One helper per user-visible session. Crash isolates to that session; repeated crashes quarantine the session kind. |
| `extension_and_tool_hosts` | Extension and tool hosts | `wasm_host_pool`, `external_extension_hosts` | `local_supervisor` | Sandboxed Wasm slots and per-process extension hosts. Crash-loop `disables` the component with typed reason. |
| `ai_and_external_tool_brokers` | AI and external-tool brokers | `ai_runtime` | `local_supervisor` | Narrows capability on failure; never blocks core editing. Projects `degraded` or `offline`. |
| `remote_connectors` | Remote connectors | `remote_connector` | `local_supervisor` | Disconnect degrades remote capability, not local stability. Projects `offline` past the disconnect threshold. |
| `policy_entitlement_and_verifier_helpers` | Policy / entitlement / verifier helpers | `local_supervisor`, `observability_collector` | self (`local_supervisor` is the root) | Lifecycle root, policy admission queries, typed RPC routing, audit ingest, support-bundle assembly. Collector is non-blocking; supervisor loss forces shell-visible degraded mode. |

Every host class in
[`runtime_host_classes.yaml`](../../artifacts/architecture/runtime_host_classes.yaml)
appears in exactly one row. The `observability_collector` is placed
in `policy_entitlement_and_verifier_helpers` because it is a
non-content-authoritative helper whose failure degrades verifier
and audit surfaces rather than any plane's authoritative truth.

### Domain attributes

Every fault-domain row declares the following axes. Closed
vocabularies live in `restart_budgets.yaml`.

- **Failure-isolation class** — re-exported from
  `runtime_host_classes.yaml`. Fault domains do not invent new
  isolation classes.
- **Restart class** — re-exported from
  `runtime_host_classes.yaml`. A domain may contain host classes
  with different restart classes (for example,
  `session_scoped_execution_hosts` contains
  `per_session_restart` and `quarantine_after_budget`).
- **Strike window** — the rolling time window over which crashes
  and stalls are counted toward the restart budget.
- **Restart budget** — the maximum automatic restarts permitted
  inside the strike window before the domain escalates.
- **Backoff profile** — the bounded-backoff schedule between
  automatic restarts (never below the per-domain floor, never
  above the per-domain ceiling).
- **Escalation ceiling** — the visible state the domain MUST
  project when the budget is exhausted. Drawn from the closed
  escalation-state vocabulary below. Every fault domain has a
  default escalation ceiling; there is no unowned restart class.
- **Authority-revocation rule** — which outstanding authority or
  capability tickets MUST be revoked before the host is
  rehydrated. Silent ticket re-use after restart is forbidden.
- **Forensic-packet trigger set** — the events that MUST emit a
  forensic packet (exit, stall, escalation, recovery).
- **Recovery owner** — the actor who clears the escalation state
  (user, local admin, supervisor auto-clear on metric window,
  support operator).

## Supervision tree

The supervision tree is rooted at the desktop session and mirrors
the ownership already frozen in
[`service_ownership_matrix.yaml`](../../artifacts/architecture/service_ownership_matrix.yaml).

```
desktop_session_root
├── desktop_shell                       (shell_interaction_core)
└── local_supervisor                    (policy_entitlement_and_verifier_helpers)
    ├── index_workers                   (workspace_knowledge_workers)
    ├── language_hosts                  (session_scoped_execution_hosts)
    ├── debug_hosts                     (session_scoped_execution_hosts)
    ├── git_worker                      (session_scoped_execution_hosts)
    ├── pty_host                        (session_scoped_execution_hosts)
    ├── wasm_host_pool                  (extension_and_tool_hosts)
    ├── external_extension_hosts        (extension_and_tool_hosts)
    ├── ai_runtime                      (ai_and_external_tool_brokers)
    ├── remote_connector                (remote_connectors)
    └── observability_collector         (policy_entitlement_and_verifier_helpers)
```

Supervision rules:

1. **Desktop shell is a peer of the supervisor.** The shell is
   never restarted behind the user. Supervisor loss forces the
   shell into a visible degraded mode; shell loss is visible app
   loss.
2. **Supervisor owns child restart decisions.** Every child host
   is either restarted by the supervisor under its declared
   restart class or marked `Quarantined` / `Disabled` per the
   escalation ceiling.
3. **Session helpers fail in isolation.** One failing language,
   debug, Git, or PTY session MUST NOT degrade another session or
   the shell.
4. **Sandbox and external-extension hosts carry no ambient trust.**
   Restart does not re-grant a previously revoked capability
   manifest; the permission manifest is re-admitted from scratch.
5. **AI and remote failures never block editing.** Restart
   backoff narrows capability and surfaces disclosure; it does
   not serialize behind typing, save, or caret / selection.
6. **Collector back-pressure is non-blocking.** A saturated
   observability collector drops or buffers bounded data; it
   never stalls protected-path emitters.

## Restart strike windows and budgets

Each fault domain declares one primary strike window and one
primary budget. The windows are defined by the shared resource
governor's threshold windows (`short_window_seconds = 5`,
`medium_window_seconds = 30`, `long_window_seconds = 120` per
[`resource_governor_thresholds.yaml`](../../artifacts/runtime/resource_governor_thresholds.yaml))
so restart policy and governor pressure policy measure the same
time buckets.

- **Short window (5 s).** Counts tight crash-loops. A host that
  exits within 5 s of a prior exit increments the short-window
  strike count.
- **Medium window (30 s).** Counts persistent instability. Used
  for heartbeats, stall detection, and RPC-contract mismatches.
- **Long window (120 s).** Counts drifting instability. Applies
  to remote connectors, extension hosts, and AI brokers where
  intermittent failure is the dominant mode.

Strike accounting rules:

1. Every automatic restart increments the short and medium-window
   counters.
2. A recorded *clean exit* (see forensic packet `exit_reason_class
   = clean_shutdown`) does not increment any counter.
3. A stall detected by missed heartbeat increments the
   medium-window counter and emits a `stall_detected` forensic
   packet before the supervisor kills the stalled host.
4. A recovery transition (see forensic packet `recovery_transition`)
   does not reset the window counters; the counters reset only when
   the full strike window elapses without another incident.
5. Budgets are per *host instance identity* for per-session helpers
   (one language-server session, one DAP session) and per *host
   class* for pool-backed hosts (index workers, Wasm pool).

## Authority and capability ticket revocation

Before any supervised host is rehydrated, the supervisor MUST
revoke every outstanding authority or capability ticket the prior
instance held. A rehydrated host starts from a clean ticket table.
This rule closes the "restart reuse" hole where a crashed host's
subscription, capability grant, or remote handle silently
resurrects after restart.

Ticket classes that MUST be revoked before rehydrate:

- **Authority envelopes.** Subscription authority envelopes the
  prior instance held (see ADR-0005 / subscription envelope) are
  marked stale; consumers receive a `resync_required` frame with
  a `host_restart` reason.
- **Capability envelopes.** Target capability envelopes (see
  `execution_context.schema.json` `capability_envelope_ref`) the
  prior instance held — PTY allocation, GPU access, network
  egress tickets — are revoked and re-issued only after policy
  admission re-evaluates.
- **Extension permission manifests.** Wasm and external extension
  permission manifests are re-admitted; silent re-grant is
  forbidden.
- **Remote transport sessions.** Remote connector SSH /
  container / agent sessions are torn down; reconnect starts
  from the `warming` visible state.
- **Credential handles.** Credential aliases held by Git, remote
  connectors, or AI brokers are invalidated; the credential
  broker re-issues per ADR-0007.
- **AI conversation handles.** Provider conversation handles the
  AI runtime held are considered terminated; new conversations
  start under re-evaluated policy.
- **Debug and task session handles.** Session-scoped helpers
  (language, debug, Git, PTY) MUST NOT silently resume a prior
  session id after restart; the session is marked terminated and
  the user re-initiates.

Silent ticket reuse after restart is non-conforming. Every
revoked ticket id MUST appear in the forensic packet's
`revoked_tickets` array.

## Visible escalation states

Repeated crash or stall projects into the visible health-state
vocabulary shared with the resource governor. Fault-domain
escalation adds three scoped labels (`Disabled`, `Quarantined`,
`Lost`) on top of the governor's `degraded`, `offline`, and
`unsupported` tokens. Surfaces MUST use one of these labels
rather than an internal enum or an invisible respawn loop.

| Escalation state | Meaning | Required visible copy | Who can clear it |
| --- | --- | --- | --- |
| `Warming` | Host is restarting after a prior failure and has not yet reached a heartbeat. | what is not yet usable; expected next heartbeat | supervisor auto-clears on heartbeat |
| `Degraded` | Host is running with narrowed capability after one or more restarts inside the short window. | what still works; last failure reason; repair path | supervisor auto-clears when short-window counter drains |
| `Disabled` | Host is stopped by declared policy (kill switch, policy narrowing, user pause). Not a failure state per se; restart is policy-gated. | why disabled; policy / emergency action id; resume path | local admin, user, or superseding emergency action |
| `Quarantined` | Host exhausted its restart budget inside the medium window; automatic restart is paused pending review. | what failed; number of failures inside the window; how to resume | user or local admin explicitly resumes |
| `Lost` | Shell interaction core exited (shell_interaction_core only). The application is no longer present; no automatic recovery. | loss notice; recovery path (relaunch) | user via relaunch |
| `Offline` | Remote or managed dependency host is unreachable past the offline threshold; local-safe paths remain. | local-safe alternative; reconnect owner | supervisor auto-clears on reconnect |
| `Ready` | Host is running nominally; no active escalation. | ordinary readiness only | — |

Projection rules:

1. The escalation state precedence is
   `Lost > Disabled > Quarantined > Offline > Degraded > Warming > Ready`.
2. A crash-loop that reaches the budget projects `Quarantined`
   and MUST NOT continue auto-restarting until cleared.
3. An emergency-action record (see
   [emergency_action_model.md](../security/emergency_action_model.md))
   that freezes or kill-switches a host projects `Disabled`. The
   emergency action id is the repair path.
4. A restart inside the strike window projects `Degraded` until
   the window drains or the host is quarantined.
5. `Warming` is not indefinite. A host that cannot reach its
   first heartbeat within the long window MUST escalate to
   `Quarantined` with a stall reason.
6. `Lost` applies only to `shell_interaction_core`. Every other
   domain has a non-lossy escalation path.

## Forensic-packet requirements

Every exit, stall, escalation, and recovery transition MUST emit
exactly one forensic packet. The packet is the canonical record
supervision, support, and recovery tasks reuse; private in-host
logs are not a substitute.

The forensic packet carries:

- **Identity.** `fault_domain_id`, `host_class_id`, and a stable
  `host_instance_id` allocated at host spawn.
- **Trigger.** `trigger_class` (`exit`, `stall_detected`,
  `escalation`, `recovery_transition`), plus
  `exit_reason_class` or `stall_reason_class` drawn from the
  closed vocabulary in
  [`forensic_packet.schema.json`](../../schemas/runtime/forensic_packet.schema.json).
- **Timing.** `spawn_at`, `last_heartbeat_at`, `observed_at`, and
  the short / medium / long-window strike counts the supervisor
  observed at trigger time.
- **Heartbeat snapshot.** Declared heartbeat interval, missed
  heartbeat count, and the last RPC-contract version the host
  advertised on the supervisor's typed RPC routing stream.
- **Resource snapshot.** Bounded CPU, memory, I/O, queue-depth,
  and open-handle counters sampled at trigger time. Raw process
  memory bodies never cross the forensic boundary; the snapshot
  carries typed counters only.
- **Capability envelopes.** The authority and capability
  envelopes the host held at trigger time, quoted by opaque ref
  into `execution_context.schema.json` and the subscription
  envelope.
- **Contract versions.** The RPC, subscription, scheme-version,
  and manifest versions the host was running against.
- **Last-good checkpoint.** An opaque ref to the last
  supervisor-acknowledged checkpoint (for pool workers and
  session helpers that declare a resume boundary). Hosts without
  a checkpoint (shell, PTY scrollback-only) MUST emit `null`.
- **Degraded-state lineage.** The ordered history of escalation
  transitions this host traversed in the current supervisor
  session (for example `Ready -> Degraded -> Quarantined`), so
  support can replay the full lineage without reaching into a
  private trace.
- **Revoked tickets.** Every ticket id the supervisor revoked
  before the next rehydrate.
- **Emergency-action linkage.** Opaque refs to any emergency-
  action or advisory records that gated this escalation, copied
  from
  [emergency_action_model.md](../security/emergency_action_model.md).
- **Redaction class.** Identifies the redaction posture the
  support collector applies when the packet is folded into a
  support bundle.

The forensic packet's owner is `observability_collector`. Every
other host class is an emitter. Emitters MUST use one of the
allowed IPC families
(`audit_event_stream`, `event_stream`, `typed_binary_rpc`) declared
in
[`service_ownership_matrix.yaml`](../../artifacts/architecture/service_ownership_matrix.yaml).
Side-channel assembly (shared memory, globals, file drop) is
non-conforming.

## Acceptance mapping

This packet, together with the three companion files, establishes:

1. **No restartable service class remains unowned or without a
   default escalation ceiling.** Every host class in
   `runtime_host_classes.yaml` appears in exactly one
   fault-domain row; every row declares a default escalation
   ceiling and recovery owner in `restart_budgets.yaml`.
2. **Repeated crash or stall becomes visible state, not an
   invisible respawn loop.** The escalation-state vocabulary
   (`Warming`, `Degraded`, `Disabled`, `Quarantined`, `Lost`,
   `Offline`, `Ready`) and the precedence / transition rules
   force every repeated failure into a visible surface rather
   than a silent counter.
3. **Support and recovery tasks can reuse the same fault-domain
   ids and forensic-packet structure.** The forensic packet is a
   cross-tool schema; its opaque refs compose with advisories,
   emergency actions, execution-context records, and support
   bundles without inventing surface-local fields.

## Non-goals clarified

- **Wall-clock values.** `restart_budgets.yaml` carries seeded
  default budgets. Production tuning may vary per OS, per target
  class, or per policy bundle. This packet freezes the shape; it
  does not claim the numbers are final.
- **Per-OS supervisor glue.** cgroups, launchd, Windows service
  control, containerised supervisors, and per-OS crash reporters
  are implementation concerns. The forensic packet is designed to
  be emitted uniformly across those platforms.
- **Cryptographic signing of forensic packets.** Signing is
  deferred; the schema includes a redaction class and an opaque
  export-identity ref so a signing layer can be added without
  changing the payload shape.
