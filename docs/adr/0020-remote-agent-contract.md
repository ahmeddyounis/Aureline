# ADR 0020 — Remote-agent contract, service-placement rules, reconnect semantics, and capability-negotiation ADR seed

- **Decision id:** D-0025 (see `artifacts/governance/decision_index.yaml#D-0025`)
- **Status:** Proposed — this is an ADR seed. The agent-hello / capability-negotiation packet, the session-lifecycle vocabulary, the heartbeat record, the target-identity binding, the authority-boundary rules, the version-skew handling, the service-placement row set, and the reconnect-decision vocabulary reserve shape and record fields so the shell command router, the supervisor, the remote-connector helper, the index worker, the task-execution host, the debug host, the notebook-kernel connector, the AI broker, the extension-host registry, the managed-workspace control plane, and the support-export lanes at a later milestone cannot invent them ad hoc. Full freeze lands in a successor ADR once the open questions in §Open questions are closed.
- **Decision date:** pending
- **Freeze deadline:** 2027-01-31
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** architecture_council (co-required with security_trust_review because the attach-ticket and credential-handle projection rules carry the broker invariants the trust-review remit already owns, and with compatibility_ecosystem_review because the host/guest negotiation rides the ADR-0019 capability-world identity scheme and the mixed-version envelope boundary family `desktop_cli_and_remote_agent`)
- **Related requirement ids:** none

## Context

Aureline's protected paths already assume that work may run in more
than one process and more than one host. ADR-0004 froze the typed
internal RPC envelope, ADR-0005 the subscription envelope's
authority classes, ADR-0006 the VFS path-identity contract,
ADR-0007 the credential-handle projection, ADR-0009 the
execution-context object model, ADR-0010 the connected-provider
grant-resolution and browser-handoff vocabulary, ADR-0011 the
five-axis capability-lifecycle markers (including the `client_scope`
axis whose frozen members contain `remote_agent`), ADR-0012 the
extension-manifest permission-publisher-policy vocabulary,
ADR-0015 the embedded-surface boundary, ADR-0016 the command-
dispatch boundary, ADR-0018 the workspace-trust packet, and ADR-0019
the Wasm / WIT extension-host and capability-world seed with its
`remote_side_component` host family. The origin/target/route
taxonomy, the host-boundary matrix at
`artifacts/remote/host_boundary_matrix.yaml`, the target-confidence
manifest at `fixtures/remote/target_confidence_manifest.yaml`, the
managed-workspace lifecycle at
`artifacts/runtime/managed_workspace_lifecycle.yaml`, the
mixed-version negotiation envelope at
`schemas/compat/mixed_version_envelope.schema.json` (whose
`boundary_family` enum already reserves `desktop_cli_and_remote_agent`),
and the version-skew register at
`artifacts/compat/version_skew_register.yaml` each reserve
surface-side fields that assume a remote agent will arrive. None
of them yet names **the contract a remote agent must speak to
identify itself, negotiate capability, hold a session, survive a
reconnect, and fail closed on version skew**.

The `.t2` source documents commit the product to a remote-helper
plane that speaks the same framed protocol as the local supervisor
(`.t2/docs/Aureline_Technical_Architecture_Document.md` — "AD-008
extension runtime | Wasm capability sandbox + isolated external
hosts"; "remote connector … re-exposes the same framed protocol on
the shell side"). They also commit the product to managed-workspace
and provider-side variants whose control-plane tokens are
distinguishable from a remote-agent attach ticket. Without a
typed-interface seed, every lane that touches a remote — the
command router's `remote_agent_attach_route`, the supervisor's
`remote_connector` fault domain, the VFS watcher plane's remote
mount surface, the index worker's remote-crawl adapter, the
task-execution host's remote-launched process, the debug host's
remote attach, the notebook-kernel connector's remote kernel, the
AI broker's remote tool-call plane, the extension-host registry's
`remote_side_component` surface, the mirror adapter's remote
continuity row, the install-review sheet's "runs on a remote agent"
chip, and the support-export path's remote-scoped evidence — would
have to invent its own hello handshake, its own heartbeat cadence,
its own reconnect rules, and its own version-skew posture. That
is exactly the fragmentation this ADR seed forbids.

This ADR rides alongside ADR-0001 (the `managed_admin_surface`
client-scope gate applies to any remote agent whose session reaches
an admin surface), ADR-0004 (every remote-agent record crosses RPC
as a typed payload; raw transport frames, raw process-launch bodies,
raw agent-binary bytes, and raw bridge-shim payloads never do),
ADR-0005 (capability-world views projected through a remote agent
ride the shared subscription envelope with authority class
`derived_knowledge` and a declared freshness hint), ADR-0006 (a
filesystem-touching remote agent binds to the VFS path-identity
contract; remote file identity projects through the canonical
identity record and never through a raw absolute path), ADR-0007
(credential handles issued to a remote agent project under broker
handle classes only; raw secret bytes never cross the agent
boundary), ADR-0008 (admin-policy narrowing is an orthogonal
ceiling over any negotiated capability set), ADR-0009 (every
session with a shell / task-touching capability resolves the
execution-context id before any capability fires), ADR-0010 (a
remote agent that drives a connected-provider surface quotes the
grant-resolution vocabulary and MAY NOT mint a new approval),
ADR-0011 (every session carries the five-axis lifecycle projection;
a remote agent's `client_scope` is `remote_agent` and its
degraded-by-trust / degraded-by-policy axes project through the
same markers), ADR-0012 (effective permission sets and publisher
continuity remain host-owned; a remote agent may declare a
permission set but never widen it), ADR-0015 (embedded surfaces
projected through a remote agent obey the native-reserved boundary),
ADR-0016 (terminal and command surfaces route through the command-
dispatch boundary regardless of which host ultimately owns the
action), ADR-0018 (every session resolves the trust-decision packet
before admitting a capability invocation; a handle issued under
`trusted` MUST NOT survive downgrade to `restricted`), and
ADR-0019 (worlds admitted through a remote agent are declared under
the capability-world identity scheme and narrowed through the host-
negotiation packet).

The remote-agent runtime itself — the concrete transport frame,
the concrete attestation envelope, the concrete deployment bundle,
and the concrete provider-side adapter set — does not land at this
milestone. What this seed reserves is the **remote-agent hello
record**, the **session-lifecycle vocabulary**, the **heartbeat
record**, the **target-identity binding**, the **authority-
boundary invariants**, the **service-placement row set**, the
**reconnect-decision vocabulary**, and the **version-skew posture
the negotiation must project** so the successor ADR has concrete
fields and records to compose against rather than prose.

## Decision

Aureline reserves seven record families — **remote-agent hello
record**, **remote-agent hello response record** (the host's
admission reply), **remote-agent heartbeat record**, **remote-agent
session-lifecycle transition record**, **remote-agent reconnect
decision record**, **service-placement row**, and **target-
identity witness record** — plus a frozen vocabulary for session-
lifecycle states, heartbeat postures, reconnect-reason classes,
in-flight-action cancellation postures, authority-boundary
invariants, placement classes, and denial reasons. Every
vocabulary named below is opened as an enumerable set whose
initial members are frozen by this seed and whose additions are
additive-minor with a `remote_agent_schema_version` bump. Where
this seed names a boundary-family binding, the binding is a seed —
additive fields, record kinds, or enumerated classes are minor
with a schema-version bump; repurposing any named item is breaking
and requires a new decision row.

The intent is deliberately narrower than the successor ADR. This
seed freezes **shape, names, and invariants**, not the transport
frame, not the attestation envelope, not the full adapter set, and
not the deployment topology for any particular remote-agent
flavour.

### Remote-agent role binding

A remote agent is not one thing. The seed reserves one row per
role class so later lanes resolve exactly one contract per role
rather than collapsing every remote helper into a single generic
"remote" surface.

| Role class                           | Binding kind                                              | Carries                                                                                                                                                                                |
|--------------------------------------|-----------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `remote_agent_primary`               | Long-lived, typed-RPC remote agent attached to the shell  | `attach_ticket_ref`, `target_identity_witness`, `session_ticket_ref`, declared `service_placement_row_refs`, declared `capability_world_refs`, declared permission-scope projection. |
| `remote_agent_helper_bridge`         | Short-lived helper launched by the primary agent          | `helper_binary_contract_id` (ADR-0019), bounded `capability_world_refs`, declared `service_placement_row_refs` narrowed to helper-eligible rows, parent-session linkage.               |
| `compatibility_bridge_remote`        | Foreign-ecosystem bridge running on the remote side       | `bridge_profile_id` (ADR-0019), translation rules, declared `service_placement_row_refs` narrowed to bridge-eligible rows.                                                             |
| `managed_workspace_agent`            | Agent whose session binds to a managed-workspace instance | `managed_workspace_instance_ref`, `managed_workspace_lifecycle_state`, `activation_budget_summary_ref`, control-plane token linkage.                                                   |
| `provider_side_remote_agent`         | Agent inside a connected-provider's execution surface     | `connected_provider_ref`, `approval_ticket_ref` (ADR-0010), bounded `service_placement_row_refs`, provider-scope projection.                                                           |
| `browser_return_callback_agent`      | Short-lived agent serving a browser-handoff return path   | `browser_handoff_packet_ref` (ADR-0010), single-invocation scope, read-only or cancel-only placement rows.                                                                             |

Role rows cross the RPC boundary as typed payloads. Raw transport
frames, raw agent-binary bytes, raw helper-launch bodies, raw
bridge-shim payloads, and raw provider-side launch bodies never do.

### Remote-agent hello record

Every remote-agent session — the initial attach, a re-attach, a
helper-launch from an existing session, a compatibility-bridge
handshake, a managed-workspace activation — opens with one
**remote-agent hello record**. The record is recorded; the
negotiation is typed; widening is denied.

Reserved fields:

| Field                                   | Notes                                                                                                                                                                                                   |
|-----------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `hello_id`                              | Opaque, stable id for this hello (safe to log).                                                                                                                                                         |
| `remote_agent_role`                     | One of the role classes above.                                                                                                                                                                          |
| `remote_agent_identity_ref`             | Opaque agent identity (stable across reconnects only when `target_identity_witness` is stable).                                                                                                         |
| `remote_agent_build_identity_ref`       | Build-identity fingerprint of the remote agent binary (ADR-0017 exact-build identity inheritance when available; otherwise an attestation-digest placeholder).                                          |
| `advertised_protocol_min`               | Minimum RPC protocol version the agent speaks (ADR-0004 `contract_version`).                                                                                                                            |
| `advertised_protocol_max`               | Maximum RPC protocol version the agent speaks.                                                                                                                                                          |
| `advertised_capability_worlds`          | Ordered list of ADR-0019 `capability_world_ref` entries the agent declares.                                                                                                                             |
| `advertised_service_placement_rows`     | Ordered list of `service_placement_row_id` entries from `artifacts/runtime/service_placement_rows.yaml` the agent declares it can carry.                                                                |
| `advertised_permission_scope_projection`| Projection, in ADR-0012 permission-scope vocabulary, of the scopes the agent requires per declared placement row.                                                                                       |
| `target_identity_witness`               | A `target_identity_witness_record` (see below) pinning the logical target this agent claims to be.                                                                                                      |
| `identity_mode_ref`                     | Ref to the ADR-0001 identity-mode envelope inherited by the session.                                                                                                                                    |
| `trust_state_ref`                       | Ref to the ADR-0018 trust-decision record governing the session.                                                                                                                                        |
| `attach_ticket_ref`                     | Ref to the ticket that admitted this attach (remote-agent attach ticket, managed-control-plane token, approval ticket, or browser-handoff packet — exactly one).                                        |
| `execution_context_root_ref`            | Ref to the ADR-0009 execution-context root (null for sessions that do not reach shell / task surfaces).                                                                                                 |
| `declared_egress_class`                 | Egress class (`artifacts/network/egress_classes.yaml`) the agent will attempt to use.                                                                                                                   |
| `declared_egress_host_allowlist`        | Host allow-list the agent will honour (per `artifacts/network/egress_classes.yaml` + ADR-0012 `egress_host_narrowing`).                                                                                 |
| `heartbeat_interval_seconds_declared`   | Heartbeat cadence the agent offers (host narrows).                                                                                                                                                      |
| `reconnect_window_seconds_declared`     | Reconnect-window cap the agent offers (host narrows; see §Reconnect).                                                                                                                                   |
| `mixed_version_envelope_ref`            | Ref to a mixed-version negotiation envelope row (`schemas/compat/mixed_version_envelope.schema.json`) with `boundary_family = desktop_cli_and_remote_agent`.                                            |
| `audit_event_refs`                      | Ordered list of hello-audit events emitted for this record.                                                                                                                                             |
| `redaction_class`                       | Redaction posture (`metadata_and_hashes_only` default; `broadened_capture_opt_in` requires explicit provenance).                                                                                        |
| `captured_at`                           | Monotonic timestamp.                                                                                                                                                                                    |
| `schema_version`                        | Integer; additive-minor on additions.                                                                                                                                                                   |

### Remote-agent hello response record

The host replies with a **remote-agent hello response record**.
Every field that narrows widens-never. Denial is typed, visible,
and repairable.

Reserved fields:

| Field                                   | Notes                                                                                                                                                          |
|-----------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `hello_id`                              | Echoes the hello id.                                                                                                                                           |
| `session_id`                            | Opaque session id issued by the host on admission.                                                                                                             |
| `admission_class`                       | One of `admitted`, `admitted_read_only`, `admitted_degraded`, `denied`, `deferred_pending_reapproval`.                                                         |
| `negotiated_protocol`                   | Single contract version chosen as the intersection of `advertised_protocol_min..max` and the host's supported range.                                           |
| `negotiated_capability_worlds`          | Admitted capability-world refs (always a subset of advertised and of the host's offered set under ADR-0019 narrowing).                                         |
| `negotiated_service_placement_rows`     | Admitted placement-row refs (always a subset of advertised and of host policy).                                                                                |
| `narrowing_reasons`                     | Ordered list of typed reasons for every capability world or placement row that was advertised but not admitted (see §Narrowing reasons).                      |
| `denial_reasons`                        | Ordered list of typed denial reasons when `admission_class = denied` (see §Denial posture).                                                                    |
| `heartbeat_interval_seconds_assigned`   | Heartbeat cadence the host requires (never wider than advertised).                                                                                             |
| `heartbeat_missed_budget`               | Number of consecutive missed heartbeats before the supervisor projects `degraded -> offline` per ADR-0009 host-class policy.                                   |
| `reconnect_window_seconds_assigned`     | Reconnect-window cap (never wider than advertised).                                                                                                            |
| `in_flight_action_handling`             | One of `cancel_all_on_disconnect`, `cancel_mutations_preserve_read_only`, `preserve_read_only_resubscribe_on_reconnect` (default), `reserved_for_provider_side`. |
| `session_ticket_ref`                    | Ref to the session ticket issued by the host for this session.                                                                                                 |
| `audit_event_refs`                      | Ordered list of admission-audit events.                                                                                                                        |
| `captured_at`                           | Monotonic timestamp.                                                                                                                                           |
| `schema_version`                        | Integer; additive-minor on additions.                                                                                                                          |

### Session-lifecycle vocabulary

Every session moves through a state machine the supervisor owns.
Additional states are additive-minor. Widening transitions are
breaking and require a new decision row.

Reserved states:

- `session_opening` — hello record accepted; admission in progress.
- `session_active` — admitted, heartbeats within budget.
- `session_degraded` — heartbeats missed inside budget, or the
  supervisor observed a reason class in the `degraded_reason`
  vocabulary below.
- `session_offline_reconnect_window_open` — transport lost; the
  reconnect window is still open.
- `session_reconnected_same_identity` — reconnected inside the
  window with `target_identity_witness_match = matched`.
- `session_reconnected_identity_changed` — reconnected inside the
  window with `target_identity_witness_match = changed` or
  `unverifiable`; every mutation authority is revoked pending
  reapproval.
- `session_downgraded_read_only` — admission narrowed after
  reconnect or after a trust / policy event; every capability
  world whose narrowing reason is `workspace_trust_restricted`,
  `admin_policy_permission_floor`, `admin_policy_deny_list`,
  `admin_policy_egress_host_narrowing`, or
  `capability_lifecycle_degraded` is projected read-only only.
- `session_cancelled_in_flight` — in-flight mutations cancelled
  per `in_flight_action_handling`; read-only work may continue if
  admitted.
- `session_closing` — orderly shutdown initiated by either side.
- `session_closed` — fully torn down; tickets revoked.
- `session_quarantined` — supervisor projected quarantine per
  ADR-0009 `knowledge_worker_quarantine` / `language_host_session_quarantine`
  pattern; re-admission requires a fresh hello.

Reserved `degraded_reason` classes:

- `heartbeat_missed_within_budget`
- `transport_jitter_observed`
- `reachability_state_degraded` (per the origin/target/route taxonomy)
- `capability_world_degraded_by_trust`
- `capability_world_degraded_by_policy`
- `version_skew_approaching_window_edge`
- `managed_workspace_recovering`

### Heartbeat record

Every active session emits heartbeats at the negotiated cadence.
Heartbeats are typed, bounded, and carry only the inspection
surface necessary for the supervisor to project `active ->
degraded -> offline` without rewriting session state.

Reserved fields:

| Field                              | Notes                                                                                                                                                       |
|------------------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `session_id`                       | Session the heartbeat belongs to.                                                                                                                           |
| `heartbeat_seq`                    | Monotonic per-session sequence number.                                                                                                                      |
| `sent_at`                          | Monotonic timestamp at the agent.                                                                                                                           |
| `observed_at`                      | Monotonic timestamp at the host when the heartbeat was received.                                                                                            |
| `rtt_class`                        | One of `within_budget`, `warning_band`, `budget_exceeded_once`, `budget_exceeded_repeat`, `unknown_clock_skew`.                                              |
| `missed_heartbeats_in_window`      | Integer; used for escalation.                                                                                                                               |
| `target_identity_witness_digest`   | Short digest of the current `target_identity_witness`; the supervisor checks this for drift.                                                                |
| `trust_state_epoch`                | Ref / epoch of the trust-decision record the agent currently sees; mismatch forces `session_downgraded_read_only`.                                          |
| `capability_world_epoch`           | Ref / epoch of the negotiated capability-world set the agent currently sees; mismatch forces renegotiation.                                                 |
| `advertised_pressure`              | Optional structured pressure signal (queue depths, in-flight counts), metadata only; never includes raw bodies.                                             |
| `schema_version`                   | Integer; additive-minor on additions.                                                                                                                       |

Raw session payloads, raw transport frames, raw credential bytes,
and raw extension artefacts never appear in a heartbeat record.

### Target-identity witness record

A remote agent claims to be a specific host; a long-lived session
must survive instance replacement, identity rotation, and
control-plane failover without silently widening authority. Every
hello, every reconnect, and every session-lifecycle transition
carries a **target-identity witness record**.

Reserved fields:

| Field                              | Notes                                                                                                                                                                                                        |
|------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `witness_id`                       | Opaque id of the witness record.                                                                                                                                                                             |
| `logical_target_ref`               | The logical target the session claims (e.g. a remote-agent alias; a managed-workspace instance id; a provider-linked ref). Stable across instance replacement.                                               |
| `canonical_instance_identity`      | Canonical instance identity (e.g. SSH host-key fingerprint, managed-instance id, provider node id). May rotate, and rotation forces a witness-change event.                                                  |
| `instance_identity_rotation_ref`   | Nullable. Ref to a rotation event when a managed control plane, a host-key rotation, or an instance replacement is the authoritative source of the new `canonical_instance_identity`.                       |
| `attestation_ref`                  | Nullable. Ref to an attestation record (build identity, signature, publisher fingerprint, code-signing chain). Required for agents that request mutating capability worlds or approvals-bearing surfaces.    |
| `prior_canonical_instance_identity`| Nullable. Preserved when a reconnect observes a change.                                                                                                                                                      |
| `witness_match_class`              | One of `matched`, `changed_with_admitted_rotation`, `changed_unadmitted`, `unverifiable`.                                                                                                                    |
| `captured_at`                      | Monotonic timestamp.                                                                                                                                                                                         |
| `schema_version`                   | Integer; additive-minor on additions.                                                                                                                                                                        |

A `witness_match_class` of `changed_unadmitted` or `unverifiable`
MUST force the session into `session_reconnected_identity_changed`
or `session_downgraded_read_only` and MUST NOT silently replay
any in-flight mutation.

### Reconnect-decision record

Reconnect is not one thing. The seed reserves one record kind with
a frozen `reconnect_reason_class` vocabulary and a frozen
`in_flight_action_handling` vocabulary so every remote lane reads
one contract rather than minting its own reconnect posture.

Reserved fields:

| Field                                      | Notes                                                                                                                                                      |
|--------------------------------------------|------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `reconnect_id`                             | Opaque id.                                                                                                                                                 |
| `prior_session_id`                         | The session the reconnect refers to.                                                                                                                       |
| `new_session_id`                           | Nullable. Populated when admission issues a new session id.                                                                                                |
| `reconnect_reason_class`                   | One of the reconnect reasons below.                                                                                                                        |
| `target_identity_witness`                  | A witness record observed on the reconnect attempt (always populated).                                                                                     |
| `witness_match_class`                      | Copied from the witness record for ease of indexing.                                                                                                       |
| `trust_state_ref_at_reconnect`             | The trust-decision record observed at reconnect (may differ from the session open).                                                                        |
| `policy_pack_epoch_at_reconnect`           | Ref / epoch for admin-policy narrowing.                                                                                                                    |
| `capability_world_renegotiation_required`  | Boolean; true when any admitted world must be renegotiated (trust downgrade, policy narrowing, lifecycle change, witness change, version skew).            |
| `in_flight_action_handling_applied`        | One of the `in_flight_action_handling` values above; always recorded (never nullable).                                                                     |
| `cancelled_action_refs`                    | Refs to mutations cancelled per `in_flight_action_handling_applied`.                                                                                       |
| `preserved_read_only_subscription_refs`    | Refs to read-only subscriptions resumed after reconnect.                                                                                                   |
| `reapproval_requirement_class`             | Copied from the host-boundary matrix vocabulary (`no_reapproval_required`, `session_ticket_refresh_required`, `approval_ticket_reissue_required`, `admin_confirmation_required`, `policy_narrowing_required`, `trust_reevaluation_required`). |
| `route_change_reason_code`                 | Copied from the origin/target/route taxonomy (`route_changed_authority_escalation`, `route_changed_target_unreachable`, `route_changed_target_replaced`, `route_changed_trust_narrowed`, `route_changed_identity_rotated`). |
| `audit_event_refs`                         | Ordered list of reconnect audit events emitted.                                                                                                            |
| `captured_at`                              | Monotonic timestamp.                                                                                                                                       |
| `schema_version`                           | Integer; additive-minor on additions.                                                                                                                      |

Reserved `reconnect_reason_class` values (additive-minor):

- `transient_disconnect_same_identity` — the witness matches, the
  trust state is unchanged, no policy epoch roll has occurred,
  and the reconnect happens inside the assigned reconnect window.
  Non-mutating subscriptions may be resumed without renegotiation;
  in-flight mutations are cancelled unless
  `in_flight_action_handling = preserve_read_only_resubscribe_on_reconnect`
  (the default), in which case only read-only state resumes.
- `changed_target_identity` — the witness changed and the change
  is not covered by `instance_identity_rotation_ref`. Every
  capability world is renegotiated; every handle derived from the
  prior session is revoked; the session enters
  `session_reconnected_identity_changed`; any in-flight mutation
  is cancelled and never replayed.
- `admitted_identity_rotation` — the witness changed but an
  admitted rotation ref accompanies the change (SSH host-key
  rotation admitted by the trust store, managed-instance
  replacement admitted by the control plane, provider node
  failover admitted by the provider). Read-only subscriptions may
  resume; mutating capability worlds require renegotiation and
  may require `approval_ticket_reissue_required`.
- `resumed_read_only_after_trust_downgrade` — trust state moved
  from `trusted` to `restricted` (ADR-0018). Every capability
  world whose floor requires `trusted` projects read-only only;
  in-flight mutations are cancelled.
- `resumed_read_only_after_policy_narrowing` — an admin-policy
  deny-list, permission-floor, or egress-host-narrowing epoch
  roll narrowed the negotiated set. In-flight mutations against
  narrowed worlds are cancelled.
- `resumed_read_only_after_credential_expiry` — a credential
  alias used by the session expired (ADR-0007). The session
  resumes read-only only; mutations requiring the expired
  credential are cancelled and surface a repair affordance.
- `cancellation_or_non_replay_of_in_flight` — an authority event
  (emergency action, trust revoke, kill-switch, extension
  quarantine) required that in-flight mutations be cancelled and
  not replayed. The reconnect MUST record the authority source.
- `reconnect_window_exceeded` — the reconnect happens after the
  assigned reconnect window. Re-admission requires a fresh hello
  and a fresh session id; no state is preserved implicitly.
- `version_skew_outside_window` — the reconnect attempt declared
  a protocol version or a world vocabulary version outside the
  negotiated skew window. Admission denies with a typed reason
  and the mixed-version envelope row is cited.

Reserved `in_flight_action_handling` values:

- `cancel_all_on_disconnect` — every in-flight action is cancelled
  on disconnect; nothing is replayable. The default for a provider-
  side remote agent and for any session whose trust state is not
  `trusted`.
- `cancel_mutations_preserve_read_only` — mutations cancelled;
  non-mutating reads and subscriptions may be preserved and
  re-attached.
- `preserve_read_only_resubscribe_on_reconnect` — default for
  `remote_agent_primary` at `trusted` state: subscriptions are
  resumed on transient disconnect same-identity; mutations are
  cancelled.
- `reserved_for_provider_side` — reserved. Provider-side semantics
  land in the successor ADR; this seed forbids a free-form
  "provider replays in-flight mutations" posture.

Silent replay of any in-flight mutation on reconnect — under any
`reconnect_reason_class`, under any `in_flight_action_handling`
value — is forbidden. A remote agent or host that replays a
mutation without a fresh authority record is non-conforming.

### Authority-boundary invariants

Remote agents cross explicit trust boundaries. The seed reserves
the following invariants; later lanes read them rather than invent
per-surface exceptions.

1. **Credential authority stays desktop.** Raw secret bytes never
   cross the agent boundary. A remote agent receives
   `credential_alias` handles only (ADR-0007); the broker remains
   desktop-owned. A remote agent that issues a credential is
   non-conforming.
2. **Trust-decision authority stays desktop.** The ADR-0018 trust-
   decision packet is minted by the host, not by the remote agent.
   A remote agent that grants, widens, or narrows trust is
   non-conforming. A remote agent MAY observe the trust state
   ref and MAY refuse a request on the basis of it.
3. **Admin-policy narrowing stays host-side.** The admin-policy
   ceiling (ADR-0008) is evaluated host-side on admission. A
   remote agent MAY NOT widen past the host-evaluated ceiling.
4. **Approval tickets are minted host-side.** An approval ticket
   (ADR-0010) is minted by the host and projected to the agent.
   A remote agent that mints an approval ticket is non-conforming.
5. **Execution-context resolution is host-first.** The execution-
   context root (ADR-0009) is resolved host-side before any
   task / shell capability fires through the agent.
6. **VFS path identity is host-first.** Filesystem-touching
   capability worlds project paths through ADR-0006 canonical
   identity; raw absolute paths never cross the agent boundary.
7. **Subscription authority is derived.** Capability-world views
   projected through a remote agent ride the shared subscription
   envelope with authority class `derived_knowledge` and a
   declared freshness hint (ADR-0005).
8. **Raw bodies never cross.** Raw transport frames, raw process
   launch bodies, raw agent binary bytes, raw helper-binary
   invocation bodies, raw bridge-shim payloads, raw credential
   bytes, raw signing-key material, and raw policy-bundle bytes
   MUST NOT appear on any remote-agent record.

### Service-placement row set

Every responsibility that could plausibly live either on the
desktop or on a remote agent carries a **service-placement row**
in `artifacts/runtime/service_placement_rows.yaml`. A row states
where the responsibility MAY run, where it MUST NOT run, and what
happens when the remote side is unavailable. Adding a row is
additive-minor.

Reserved `placement_class` values:

- `desktop_only_never_remote` — the responsibility is shell- or
  supervisor-owned and MAY NOT be carried by any remote agent
  (e.g. credential broker, trust-decision packet mint, admin-
  policy evaluation ceiling, shell input dispatch, render submit).
- `local_only_preferred_degrades_remote_eligible` — the
  responsibility normally runs on the desktop but may be routed
  through a remote agent with an explicit capability-world
  declaration and a narrowed trust state.
- `remote_agent_near_code_eligible` — the responsibility is
  eligible to run on a remote agent near the code (workspace
  index refresh, file-system watcher, build/test launch) and
  normally prefers that location when one is attached.
- `remote_agent_near_code_required_when_attached` — when a
  remote agent is attached, the responsibility MUST run on the
  remote side (e.g. remote-workspace file reads); a desktop
  fallback is reduced-scope and surfaces a typed
  `local_only_continuation` record.
- `provider_side_only` — the responsibility lives only inside a
  connected-provider surface (ADR-0010); the desktop and any
  remote-agent-primary MAY call it but MAY NOT host it.
- `managed_workspace_agent_only` — the responsibility lives only
  inside a managed-workspace agent instance.
- `compatibility_bridge_only` — the responsibility is translated
  through an ADR-0019 compatibility bridge; a native remote agent
  MAY NOT carry it without a bridge binding.
- `spectrum_owner_routing` — the responsibility is owned by a
  spectrum router (e.g. the AI broker) whose placement decision
  is itself typed and auditable; the row names the routing
  vocabulary.

Reserved `downgrade_class` values (per row):

- `fail_closed_no_local_fallback` — no local-only continuation;
  denying the action is the correct behaviour (e.g. remote-only
  workloads in a provider-side agent).
- `local_only_continuation_read_only` — local-only continuation
  is admissible but read-only; mutations require remote
  reachability.
- `local_only_continuation_full_local_scope` — the responsibility
  is fully recoverable locally; the remote side was a preference,
  not a requirement.
- `remote_preferred_degrades_to_bridge` — a compatibility bridge
  may carry the responsibility on downgrade.
- `provider_side_degrades_read_only` — provider outage forces a
  read-only projection; mutations surface
  `approval_ticket_reissue_required`.

Reserved `reconnect_class` values (per row):

- `resumes_read_only_on_transient_disconnect`
- `requires_renegotiation_on_identity_change`
- `requires_admin_confirmation_on_control_plane_failover`
- `cancels_in_flight_on_trust_downgrade`
- `cancels_in_flight_on_policy_narrowing`
- `cancels_in_flight_on_credential_expiry`
- `fresh_session_required_on_window_exceeded`

A placement row whose `downgrade_class` or `reconnect_class` is
unset is non-conforming at admission.

### Narrowing reasons

Every advertised capability world or service-placement row that
was **not** admitted carries one typed narrowing reason. Reserved
values extend the ADR-0019 narrowing vocabulary with remote-
agent-specific reasons and are additive-minor:

- `service_placement_row_not_admitted_by_host_policy`
- `service_placement_row_desktop_only_never_remote`
- `service_placement_row_requires_bridge_binding`
- `target_identity_witness_unverifiable`
- `target_identity_witness_changed_unadmitted`
- `attestation_ref_required_not_present`
- `managed_workspace_lifecycle_state_not_ready`
- `connected_provider_approval_ticket_not_present`
- `remote_egress_class_denied_by_policy`
- `version_skew_outside_supported_window`
- `reconnect_window_exceeded_no_preserved_state`

Plus every reason reserved by ADR-0019 §Narrowing reasons applies
per-world (`workspace_trust_restricted`, `admin_policy_deny_list`,
`admin_policy_permission_floor`, `admin_policy_egress_host_narrowing`,
`capability_lifecycle_degraded`, `world_vocabulary_version_unknown`,
`host_abi_range_mismatch`, `guest_abi_range_mismatch`,
`compatibility_bridge_profile_unbound`,
`budget_declaration_unacceptable`).

### Audit events reserved

The eventual remote-agent crate emits a typed audit stream on
`remote_agent_session`. The seed reserves at minimum the following
audit-event ids; additional events are additive-minor.

- `remote_agent_hello_opened`
- `remote_agent_hello_admitted`
- `remote_agent_hello_admitted_read_only`
- `remote_agent_hello_denied`
- `remote_agent_session_heartbeat_missed`
- `remote_agent_session_degraded`
- `remote_agent_session_offline_reconnect_window_open`
- `remote_agent_session_reconnected_same_identity`
- `remote_agent_session_reconnected_identity_changed`
- `remote_agent_session_downgraded_read_only`
- `remote_agent_session_cancelled_in_flight`
- `remote_agent_session_version_skew_denied`
- `remote_agent_session_attestation_missing_denied`
- `remote_agent_session_target_identity_rotation_admitted`
- `remote_agent_session_target_identity_rotation_rejected`
- `remote_agent_session_policy_pack_denied`
- `remote_agent_session_trust_state_denied`
- `remote_agent_session_closed`
- `remote_agent_session_quarantined`

Raw transport frames, raw process launch bodies, raw agent binary
bytes, raw helper-binary invocation bodies, raw bridge-shim
payloads, raw credential bytes, raw signing-key material, and raw
policy-bundle bytes never appear on any of these events.

### Denial posture

Failures in hello and reconnect fail closed. Denial is typed,
visible, auditable, and repairable. The following denial reasons
are reserved by this seed; additional reasons are additive-minor.

- `remote_agent_role_class_unknown`
- `remote_agent_attestation_required_missing`
- `remote_agent_attestation_signature_invalid`
- `remote_agent_attach_ticket_missing`
- `remote_agent_attach_ticket_expired`
- `remote_agent_attach_ticket_revoked`
- `remote_agent_target_identity_witness_unverifiable`
- `remote_agent_target_identity_witness_changed_unadmitted`
- `remote_agent_protocol_range_mismatch`
- `remote_agent_capability_world_unsupported_on_host`
- `remote_agent_service_placement_row_desktop_only`
- `remote_agent_trust_state_denies_session`
- `remote_agent_policy_pack_denies_session`
- `remote_agent_egress_class_denied`
- `remote_agent_managed_workspace_not_ready`
- `remote_agent_mixed_version_envelope_outside_window`
- `remote_agent_heartbeat_deadline_exceeded`
- `remote_agent_reconnect_window_exceeded_no_state_preserved`
- `remote_agent_identity_rotation_not_admitted`

Silent downgrade to a generic "not available" chip is forbidden;
every denial emits the corresponding audit event with its typed
reason and a repair affordance.

### Version-skew handling

Every admitted session cites a mixed-version negotiation envelope
with `boundary_family = desktop_cli_and_remote_agent`
(`schemas/compat/mixed_version_envelope.schema.json`). The
envelope's `reserved_boundary_fields.desktop_cli_and_remote_agent`
block names `client_version`, `agent_version`, `min_protocol`,
`max_protocol`, and `toolchain_manifest_epoch`. The session's
`negotiated_protocol` MUST be the intersection of the envelope's
producer / consumer protocol ranges; outside-window postures are
read from `artifacts/compat/version_skew_register.yaml`
(`fail_closed`, `read_only`, `degraded`, `explicitly_unsupported`).
A session whose advertised protocol range is outside the register
MUST deny with `remote_agent_mixed_version_envelope_outside_window`
and MUST NOT degrade silently to the nearest compatible version.
Upgrade-order and rollback-order violations surface as denial
reasons, not silent coercion.

### Process-boundary constraints

1. Remote-agent hello records, hello response records, heartbeat
   records, session-lifecycle transition records, reconnect-
   decision records, service-placement rows, and target-identity
   witness records cross the RPC boundary as typed payloads
   (ADR-0004). Raw transport frames, raw agent binary bytes, raw
   helper-binary invocation bodies, raw bridge-shim payloads, raw
   credential bytes, raw signing-key material, and raw policy-
   bundle bytes never cross.
2. A remote-agent session reads capability-world rows only through
   the shared subscription envelope (ADR-0005) with authority class
   `derived_knowledge` and a declared freshness hint.
3. A credential handle a remote agent requests projects under
   ADR-0007 handle classes only; raw secret bytes never cross the
   agent boundary.
4. A session bound to the shell / task surface resolves the
   ADR-0009 execution-context id before any capability fires; raw
   command lines and env bodies never cross.
5. A session touching a connected provider quotes the ADR-0010
   grant-resolution vocabulary and MAY NOT mint a new approval
   ticket.
6. A session carrying an ADR-0019 capability world declares the
   world identity and the permission-scope projection; widening is
   denied with the ADR-0012
   `effective_permission_widening_attempted` reason.
7. Every session resolves the ADR-0018 `trust_state` packet before
   admitting any capability invocation; a handle issued under
   `trusted` MUST NOT survive downgrade to `restricted`.
8. Crash dumps and core files MUST NOT inherit unresolved
   remote-agent hello or reconnect packets; a crash discards the
   packet rather than persisting a partial set.
9. Mutation-journal entries, save manifests, claim manifests, and
   support bundles carry session ids, hello ids, reconnect ids,
   target-identity witness digests, placement-row refs,
   capability-world refs, narrowing-reason labels, and
   reconnect-reason-class labels only; they MUST NOT embed raw
   transport frames, raw agent binary bytes, raw bridge-shim
   payloads, raw credential bytes, raw signing-key material, or
   raw policy-bundle bytes.

### Schema-of-record posture

The cross-tool boundary schema at
`schemas/runtime/remote_agent_hello.schema.json` exports the
remote-agent hello, hello response, heartbeat, session-lifecycle
transition, reconnect-decision, and target-identity witness
records. The placement-row registry at
`artifacts/runtime/service_placement_rows.yaml` binds each
responsibility to its placement class, downgrade class, and
reconnect class. Worked reconnect fixtures live under
`fixtures/runtime/reconnect_cases/` and pin the four reconnect
scenarios the acceptance criteria call out. Rust types in an
eventual remote-agent crate will carry the same record shape once
the successor ADR lands.

Adding a record kind, a session-lifecycle state, a reconnect-reason
class, a narrowing reason, an audit-event id, a denial reason, or
an additive field is additive-minor with a schema-version bump.
Repurposing any member is breaking and requires a new decision
row. No external IDL or code-generator toolchain at this
milestone; this mirrors ADR 0004 through ADR 0019.

## Consequences

- **Reserved:** the seven record families (remote-agent hello,
  hello response, heartbeat, session-lifecycle transition,
  reconnect decision, service-placement row, target-identity
  witness), the role class set, the session-lifecycle state set,
  the reconnect-reason class set, the in-flight-action handling
  set, the placement-class / downgrade-class / reconnect-class
  vocabularies, the narrowing-reason set, the audit-event id set,
  and the denial-reason set. Every later lane reads these rather
  than inventing its own.
- **Reserved:** the authority-boundary invariants. Credential
  authority, trust-decision authority, admin-policy narrowing,
  approval-ticket minting, execution-context resolution, and VFS
  path identity stay desktop-first. A remote agent observes and
  projects these; it MUST NOT mint them.
- **Reserved:** the process-boundary constraints. Raw transport
  frames, raw agent binary bytes, raw helper-binary invocation
  bodies, raw bridge-shim payloads, raw credential bytes, raw
  signing-key material, and raw policy-bundle bytes never cross
  RPC. Remote-agent records cross as typed payloads.
- **Reserved:** the schema-of-record posture. The JSON Schema at
  `schemas/runtime/remote_agent_hello.schema.json` is the cross-
  tool boundary; the placement registry at
  `artifacts/runtime/service_placement_rows.yaml` binds rows; the
  worked fixtures at `fixtures/runtime/reconnect_cases/` pin the
  reconnect scenarios. No external IDL at this milestone.
- **Permitted:** later additive-minor additions to any enumerated
  set (new role classes, new reconnect reasons, new placement
  rows, new narrowing reasons, new audit events, new denial
  reasons) with a schema / vocabulary bump.
- **Permitted:** admin policy packs, trust-state narrowing,
  capability-lifecycle markers, compatibility-bridge translation,
  and version-skew register cases MAY each narrow a declared
  session further. None MAY widen.
- **Follow-up:** the successor ADR closes the open questions
  below (transport frame, attestation envelope, provider-side
  semantics, managed-workspace continuity, helper-launch
  contract, bridge-profile set for remote-side components, SDK
  binding per role, heartbeat-cadence ceilings per profile, and
  reconnect-window ceilings per profile) and promotes this
  seed's `Proposed` status to `Accepted`.
- **Follow-up:** the service-topology, authority-ticket, mixed-
  version, support-bundle, mutation-journal, install-review, and
  permission-inspector lanes each cite this ADR as the governing
  remote-lane contract. A lane that hides the remote-agent
  session id, reconnect id, target-identity witness, or version-
  skew envelope ref on a remote-touching action denies with the
  appropriate denial reason.
- **Ratifies:** ADR-0001 identity-mode envelope inherited,
  ADR-0004 typed RPC payload rules, ADR-0005 subscription
  authority `derived_knowledge`, ADR-0006 VFS path identity,
  ADR-0007 credential-handle projection, ADR-0008 admin-policy
  narrowing ceiling, ADR-0009 execution-context resolution,
  ADR-0010 connected-provider grant-resolution and approval-
  ticket vocabulary, ADR-0011 capability-lifecycle client-scope
  projection (`remote_agent` client scope), ADR-0012 effective-
  permission projection rules, ADR-0015 embedded-surface
  boundary, ADR-0016 command-dispatch boundary, ADR-0018 trust-
  decision packet, ADR-0019 capability-world identity scheme and
  host-negotiation packet.

## Alternatives considered

- **Defer remote-agent vocabulary until the runtime lands.**
  Rejected: the origin/target/route taxonomy, the host-boundary
  matrix, the managed-workspace lifecycle, and the mixed-version
  envelope already reserve remote-agent-shaped fields that would
  either stay free-form or be minted per-surface. The install-
  review sheet cannot render `runs_on_remote_agent` without a
  typed hello; support export cannot reconstruct a reconnect
  without a typed reconnect record; the mixed-version envelope's
  `desktop_cli_and_remote_agent` block is unreachable without a
  session reference.
- **Bind remote-agent vocabulary onto ADR-0019 host-negotiation.**
  Rejected: ADR-0019 owns the extension-host capability-world
  identity and host-family binding; remote agents span extension
  hosts, VFS watchers, task executors, debug sessions, notebook
  kernels, and AI tool-call planes. Collapsing the remote-agent
  contract into the extension-host one forces every non-extension
  lane to masquerade as an extension.
- **Treat every remote as one generic "remote helper" surface.**
  Rejected: provider-side agents, managed-workspace agents, and
  compatibility-bridge remotes have different trust, approval-
  ticket, and lifecycle postures. A single surface would force
  the supervisor to guess; role-class binding makes the guess
  explicit.
- **Let each lane emit its own heartbeat / reconnect packet.**
  Rejected: the supervisor, the forensic-packet exporter, the
  mutation journal, the support bundle, and the install-review
  sheet would have to speak N different reconnect vocabularies.
  A single reconnect-decision record is the minimum viable
  boundary.
- **Allow silent replay of idempotent mutations on reconnect.**
  Rejected: idempotency is a property of an action's body, not
  its authority. Trust downgrades, policy epoch rolls, and
  credential expiries between disconnect and reconnect can
  invalidate the original authority. Replay without a fresh
  authority record blurs host-boundary truth and hides authority
  escalation.
- **Let the remote agent mint approval tickets or trust states.**
  Rejected: this collapses the authority boundary ADR-0010 and
  ADR-0018 freeze. Mint authority stays desktop-side; the agent
  observes, projects, and refuses.
- **External IDL + codegen (Protobuf, Cap'n Proto, Smithy).**
  Rejected: same reasoning as ADR 0004 through ADR 0019 — no
  second-language consumer yet beyond the JSON Schema boundary
  itself. The schema export reserves a clean integration point
  for the hello, heartbeat, and reconnect records.

The `D-0025` `freeze_lane` default-if-unresolved posture would
block the remote-helper, managed-workspace, mixed-version, and
install-review lanes from closing the remote-agent contract at
the first-beta milestone until a successor ADR lands. Accepting
the seed's `Proposed` status now — with its reserved vocabulary,
records, placement registry, and worked reconnect fixtures —
avoids that freeze by giving the successor ADR concrete records
to compose against.

## Open questions

These questions MUST be answered by the successor ADR before this
seed is promoted to `Accepted`. They are listed so no later lane
assumes a resolution silently.

1. **Transport frame.** What is the concrete transport frame the
   remote-agent session runs over (framed RPC over QUIC, over a
   single TCP session, over SSH-channel multiplexing, over a
   helper-bridge process pipe), and how does it compose with the
   ADR-0004 envelope's `trace` / `deadline` / `cancellation_channel`
   fields across a network?
2. **Attestation envelope.** What is the typed attestation envelope
   a mutating remote agent MUST carry (build-identity fingerprint,
   signature chain, publisher continuity ref, SBOM ref), and how
   does the host verify it before admission?
3. **Provider-side semantics.** What are the concrete session-
   lifecycle and reconnect semantics for `provider_side_remote_agent`
   (per-invocation session, provider-side persistence semantics,
   idempotency tokens the host owns), and which
   `in_flight_action_handling` value applies?
4. **Managed-workspace continuity.** How does `managed_workspace_agent`
   compose with the managed-workspace lifecycle (warming, recovering,
   idle-suspended, hibernated, retired) and with the local-only
   continuation row when the control plane is unreachable?
5. **Helper-launch contract.** What is the typed launch contract for
   `remote_agent_helper_bridge` (kill-switch cadence, stdio
   isolation, short-lived window semantics, credential-handle
   projection, signature re-verification) and how does it inherit
   the parent session's identity-mode envelope?
6. **Bridge-profile set for remote-side components.** Which
   compatibility-bridge profiles ship at the first beta for remote-
   side components, and how does each honour vs translate vs refuse
   the placement rows?
7. **SDK binding per role.** How does the stable-surface SDK bind
   to each role (client scope per ADR-0011), and what stability-
   window labels apply to the hello / heartbeat / reconnect record
   families?
8. **Heartbeat-cadence ceilings per profile.** What are the
   concrete heartbeat interval and missed-heartbeat budget
   ceilings per role class, per identity mode, and per trust state?
9. **Reconnect-window ceilings per profile.** What are the
   concrete reconnect-window ceilings per role class, per
   identity mode, and per trust state, and how does a session-
   ticket-expiry event compose with them?
10. **Cross-agent subscription fanout.** When multiple remote
    agents serve one workspace, do their capability-world
    subscriptions share a fanout budget (ADR-0019
    `subscription_fanout_budget_class`) or is fanout declared
    per-session? The nesting shape is open.

Each question blocks the `Proposed` -> `Accepted` transition and
is tracked in the `decision_history` of `D-0025`.

## Source anchors

- `.t2/docs/Aureline_PRD.md` — "remote development parity …
  identity, credentials, policy, trust, and transport posture are
  shared with the desktop client."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — "AD-008
  | Extension runtime | Wasm capability sandbox + isolated external
  hosts."
- `.t2/docs/Aureline_Technical_Architecture_Document.md` — "remote
  connector … re-exposes the same framed protocol on the shell
  side."
- `.t2/docs/Aureline_Technical_Design_Document.md` — "remote attach
  follows the same approval-ticket path as the connected-provider
  handoff; in-flight mutations do not silently replay on
  reconnect."
- `.t2/docs/Aureline_Milestones_Document.md` — "service topology,
  authority-ticket, and mixed-version tasks cite the remote-lane
  contract for every remote-touching row."

## Linked artifacts

- Decision register row:
  `artifacts/governance/decision_index.yaml#D-0025`
- RFC: none (the open-question option space runs down in the
  successor ADR).
- Remote-agent hello boundary schema:
  `schemas/runtime/remote_agent_hello.schema.json`
- Service-placement registry:
  `artifacts/runtime/service_placement_rows.yaml`
- Worked reconnect fixtures:
  - `fixtures/runtime/reconnect_cases/transient_disconnect_same_identity.json`
  - `fixtures/runtime/reconnect_cases/changed_target_identity_unadmitted.json`
  - `fixtures/runtime/reconnect_cases/resumed_read_only_after_trust_downgrade.json`
  - `fixtures/runtime/reconnect_cases/cancellation_non_replay_in_flight.json`
  - `fixtures/runtime/reconnect_cases/admitted_identity_rotation_managed_workspace.json`
  - `fixtures/runtime/reconnect_cases/version_skew_outside_window_denied.json`
- RPC envelope this contract rides:
  `docs/adr/0004-rpc-transport-and-schema-toolchain.md`
- Subscription envelope remote views ride:
  `docs/adr/0005-subscription-envelope-and-invalidation-semantics.md`
- VFS path identity remote filesystem-touching rows bind to:
  `docs/adr/0006-vfs-save-cache-identity.md`
- Secret-broker handle classes remote credential handles cite:
  `docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`
- Admin-policy narrowing ceiling this contract honours:
  `docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`
- Execution-context model shell / task-touching rows bind to:
  `docs/adr/0009-execution-context-and-scope.md`
- Connected-provider vocabulary provider-side sessions quote:
  `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`
- Capability-lifecycle vocabulary per-row projection cites:
  `docs/adr/0011-capability-lifecycle-and-dependency-markers.md`
- Extension-manifest permission-scope vocabulary worlds project:
  `docs/adr/0012-extension-manifest-permission-publisher-policy.md`
- Embedded-surface boundary remote surfaces honour:
  `docs/adr/0015-embedded-surface-boundary-and-auth-handoff.md`
- Command-dispatch boundary terminal / task rows route through:
  `docs/adr/0016-shell-windowing-input-accessibility-boundary.md`
- Workspace-trust packet every session resolves:
  `docs/adr/0018-workspace-trust-and-restricted-mode.md`
- Capability-world identity scheme and host-negotiation packet
  this contract rides:
  `docs/adr/0019-wasm-wit-extension-host-and-capability-worlds.md`
- Origin/target/route taxonomy reconnect-reason and route-change
  codes inherit:
  `docs/runtime/origin_target_route_taxonomy.md`
- Host-boundary matrix remote rows compose with:
  `artifacts/remote/host_boundary_matrix.yaml`
- Managed-workspace lifecycle managed-agent rows bind to:
  `artifacts/runtime/managed_workspace_lifecycle.yaml`
- Mixed-version negotiation envelope the session cites:
  `schemas/compat/mixed_version_envelope.schema.json`
- Version-skew register outside-window postures cite:
  `artifacts/compat/version_skew_register.yaml`
- Service-topology / process-placement map the placement rows
  compose with:
  `docs/architecture/service_topology_and_process_placement.md`
- Affected lanes:
  `governance_lane:architecture_council`,
  `governance_lane:security_trust_review`,
  `governance_lane:compatibility_ecosystem_review`,
  `governance_lane:docs_public_truth`,
  `governance_lane:support_export`,
  `governance_lane:governance_packets`.

## Supersession history

First acceptance (as a seed at `Status: Proposed`). A successor
ADR promotes this seed to `Accepted` once the open questions are
closed, and records the supersession in this section without
rewriting the body above.
