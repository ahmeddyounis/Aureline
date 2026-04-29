# Environment-Context, Infrastructure-Connector, and Action-Safety Contract

This document freezes the packet family Aureline uses when a surface
observes, previews, hands off, or mutates a live environment target.
It exists so Kubernetes, cloud, database, queue, preview, incident,
runbook, browser, desktop, and CLI flows all name the same target
truth and safety states instead of relying on an unlabeled "current
environment".

The contract is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or
Design System Style Guide, those source documents win and this
document plus the companion schemas must be updated in the same
change.

## Companion Artifacts

- [`/schemas/runtime/target_context.schema.json`](../../schemas/runtime/target_context.schema.json)
  — boundary schema for `target_context_record` and its audit events.
  This record names provider, account or subscription, project,
  region, cluster, namespace, environment class, runtime host, target
  scope, credential posture, freshness, stale-context state,
  mixed-authority state, and source-to-live relationships.
- [`/schemas/runtime/connector_session.schema.json`](../../schemas/runtime/connector_session.schema.json)
  — boundary schema for `connector_session_record` and connector
  audit events. It freezes the connector-session classes, capability
  posture, credential handles, bridge and handoff bindings, imported
  read-only evidence bindings, and downgrade or invalidation markers.
- [`/schemas/runtime/action_safety_review.schema.json`](../../schemas/runtime/action_safety_review.schema.json)
  — boundary schema for `action_safety_review_record` and action
  safety audit events. It binds inspect, dry-run, apply, restart,
  scale, delete, publish, and rollback verbs to explicit target
  confirmation, approval posture, preview hashes, audit refs, and
  no-hidden-target-switch checks.
- [`/docs/runtime/resource_drift_and_live_action_contract.md`](./resource_drift_and_live_action_contract.md)
  — resource-target, drift-summary, and live-action envelope companion
  contract. It keeps source refs, rendered refs, live handles, cached
  or imported state, provider-limited reads, and post-preview drift
  distinct before runtime surfaces broaden.
- [`/fixtures/runtime/environment_target_cases/`](../../fixtures/runtime/environment_target_cases/)
  — fixture cases covering ready mutation, expired credentials,
  changed local cluster context, mixed local-plus-managed authority,
  no-source-match live resources, browser handoff, and imported
  read-only evidence.
- [`/docs/runtime/environment_capsule_contract.md`](./environment_capsule_contract.md)
  — execution environment capsule contract. Environment capsules
  answer "what runtime did this execution run in"; target contexts
  answer "what live infrastructure target is this action observing or
  changing".
- [`/docs/runtime/origin_target_route_taxonomy.md`](./origin_target_route_taxonomy.md)
  — action origin / target / route truth. Action safety packets cite
  route and authority audit refs rather than redefining route classes.
- [`/docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md)
  — browser handoff and approval ticket precedent consumed by
  `browser_handoff` connector sessions.

Normative sources projected here:

- `.t2/docs/Aureline_Technical_Architecture_Document.md`
  sections on cluster context, infrastructure connectors, source /
  rendered / planned / observed resource relationships, and the
  cluster context action-safety matrix.
- `.t2/docs/Aureline_Technical_Design_Document.md`
  appendices for environment context, resource target drift,
  live-action envelopes, runbook step execution, approval, and
  rollback.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md`
  sections on infrastructure config, cluster-resource UX, target
  context chips, apply preview, live-action review, and console
  handoff banners.
- `.t2/docs/Aureline_PRD.md`
  requirements for remote connectors, least-privilege authority,
  DevOps / SRE workflows, transport governance, and explicit
  approval gates.

## Scope

Frozen at this revision:

- one `target_context_record` shape for live infrastructure targets,
  including clusters, namespaces, projects, regions, accounts,
  subscriptions, environment class, runtime host, target scope,
  credential identity, freshness, stale-context state, mixed-authority
  state, and resource-relationship truth;
- one `connector_session_record` shape for `local_config`,
  `delegated_identity`, `ssh_or_tunnel_bridge`, `managed_connector`,
  `browser_handoff`, and `imported_read_only_evidence` sessions;
- one `action_safety_review_record` shape for `inspect`, `dry_run`,
  `apply`, `restart`, `scale`, `delete`, `publish`, and `rollback`
  verbs;
- typed downgrade and invalidation triggers for expired credentials,
  changed cluster context, connector drift, mixed local-plus-managed
  authority, target ambiguity after load, freshness-floor misses,
  policy epoch changes, credential-scope narrowing, browser handoff
  return mismatch, and imported evidence with no live authority;
- a shared source-to-rendered-to-planned-to-live relationship record
  with explicit `no_source_match` cases.

Out of scope at this revision:

- shipping a Kubernetes, cloud, database, queue, or provider
  connector;
- provider-specific API request bodies, kubeconfig loading, OAuth,
  device-code, SSH-agent, vault, or browser-cookie implementation;
- final UI microcopy for target-context chips or action sheets;
- executing runbooks. Runbook packets consume this review packet when
  a step inspects or mutates a protected target.

## Invariants

Every protected live surface follows these rules:

1. **No unlabeled current environment.** A surface may not inspect or
   mutate by inheriting ambient `kubectl`, cloud profile, shell,
   database, queue, or browser state. It must show or emit a
   `target_context_record` with `ambient_context_prohibited = true`.
2. **One target truth across surfaces.** Desktop, CLI JSON, support
   export, runbook, incident workspace, browser companion, automation,
   and AI action sheets use the same `target_context_ref`,
   `connector_session_ref`, and `action_review_id` when they refer to
   the same action.
3. **Mutation is fail-closed.** A stale, ambiguous, mixed-authority,
   invalidated, unavailable, or imported-read-only context may inspect
   or explain where policy permits, but it cannot silently mutate.
4. **Source and live state stay linked.** Authored, rendered, planned,
   observed, provider overlay, and imported evidence identities stay
   distinguishable. When a live object cannot be traced back to
   source or plan, the relationship state is `no_source_match`; the
   product must not imply a source match exists.
5. **No hidden target switch.** The target context loaded into the
   review packet and the context resolved immediately before
   execution must match. Any cluster, namespace, account,
   subscription, project, region, connector, or browser-handoff
   target switch blocks the action.
6. **Credential handles only.** Packets carry credential handle class,
   issuance source, expiry, scope, step-up state, and revocation refs.
   They do not carry raw secrets, raw kubeconfig, SSH private keys,
   cloud tokens, browser cookies, or raw provider responses.

## Target Context

`target_context_record` is the explicit target identity that every
live-action surface must name. Its required spine is:

- `target_context_id`, `display_label`, `context_state`, and
  `write_posture`;
- `target_identity.provider_class`, `environment_class`,
  `tenant_ref`, `account_ref`, `subscription_ref`, `project_ref`,
  `region`, `zone`, `cluster_ref`, `namespace_ref`, `runtime_host`,
  and `target_scope`;
- `credential_identity` with handle class, handle ref, issuance
  source, issued and expiry timestamps, write scope, and step-up
  state;
- `freshness_provenance` with freshness class, first-loaded time,
  observed time, source connector session, raw-response ref,
  completeness label, and policy epoch;
- `stale_context_state` and `mixed_authority_state`;
- `resource_relationships[]` and `audit_refs[]`.

### Context States

`context_state = current` means the target identity, credential
posture, freshness floor, connector session, and policy epoch are
valid for the surface's claimed action.

`context_state = stale` means the target was once resolved but can no
longer authorize mutation without re-resolution. Examples include
expired credentials, freshness-floor miss, changed cluster context,
connector drift, target moved or deleted, policy epoch change, source
revision change, imported-evidence-only state, or target ambiguity
discovered after load.

`context_state = ambiguous` means the target identity is not unique.
For example, the same service name may resolve in multiple
namespaces, or a provider redirect may map to multiple projects.

`context_state = mixed_authority` means incompatible authorities
contributed target truth. The common high-risk case is a local config
selecting one cluster or namespace while a managed connector supplies
credentials for another.

`context_state = invalidated` means the context must be discarded
before any live action. `inspect_only` means the context may explain
or compare but not mutate. `unavailable` means no live target
connection exists.

### Environment Identity

The target identity always carries explicit fields for:

- provider class;
- tenant, account, subscription, and project refs where applicable;
- region and zone where applicable;
- cluster and namespace where applicable;
- runtime host class and host ref;
- target scope class, stable scope id, and display label.

Fields that do not apply are `null`, not omitted. This makes the
absence of a namespace, subscription, or region reviewable rather
than implicit.

### Freshness And Write Posture

Freshness is recorded as `live`, `current_snapshot`,
`cached_within_floor`, `stale`, `partial`, `offline`,
`imported_read_only`, or `unknown`. A cached or imported context may
remain useful for inspect and support export, but it cannot claim live
mutation authority.

Write posture is recorded as `read_only`, `inspect_only`,
`dry_run_only`, `write_capable_pending_approval`,
`write_capable_approved`, or `write_blocked`. Production and
disaster-recovery contexts default to read-only or approval-pending
posture until an explicit review packet attaches a valid approval.

## Connector Sessions

`connector_session_record` binds a target context to the authority or
evidence source that produced it. Its `connector_session_class`
vocabulary is closed:

- `local_config` — local CLI config, local kubeconfig, local cloud
  profile, local database connection, or local queue config. This
  class may inspect or dry-run only after the resolved target is shown
  explicitly.
- `delegated_identity` — short-lived or brokered identity delegated
  by an identity provider, vault, enterprise policy service, or
  managed control plane.
- `ssh_or_tunnel_bridge` — SSH, port-forward, private-network bridge,
  or managed tunnel through which a target is observed or controlled.
- `managed_connector` — remote agent, managed workspace helper, or
  product-managed connector with scoped credentials.
- `browser_handoff` — attributable system-browser or provider-console
  handoff. The handoff records destination class, target context,
  return anchor, and evidence refs.
- `imported_read_only_evidence` — support bundle, incident export,
  captured provider snapshot, or other imported evidence. It can
  explain and compare; it never grants live mutation authority.

Each session declares `backing_connector_class` as `static_file_only`,
`cli_mediated`, `agent_mediated_live`, or
`provider_console_overlay`. This crosswalk preserves the architecture
connector classes while allowing session-specific states.

### Capability Posture

Capabilities are per verb family:

- `inspect`;
- `dry_run`;
- `apply`;
- `restart`;
- `scale`;
- `delete`;
- `publish`;
- `rollback`;
- `attach`;
- `forward`;
- `exec`.

Each capability is `allowed`, `approval_required`, `inspect_only`,
`denied`, or `not_claimed`. A connector that can inspect may not infer
apply, restart, scale, delete, publish, or rollback capability from
that fact.

### Invalidation And Downgrade

The connector session records `invalidation_markers[]`. Each marker
has a typed trigger and resulting posture:

| Trigger | Required posture |
|---|---|
| `expired_credentials` | `downgrade_to_inspect_only`, `require_reapproval`, or `block_mutation` |
| `changed_cluster_context` | `require_target_reconfirm` or `block_mutation` |
| `connector_drift` | `downgrade_to_inspect_only` or `invalidate_session` |
| `mixed_local_plus_managed_authority` | `block_mutation` |
| `target_ambiguity_after_load` | `require_target_reconfirm` or `block_mutation` |
| `freshness_floor_unmet` | `downgrade_to_inspect_only` or `block_mutation` |
| `policy_epoch_changed` | `require_reapproval` or `block_mutation` |
| `credential_scope_narrowed` | `require_reapproval` or `block_mutation` |
| `browser_handoff_return_mismatch` | `block_mutation` |
| `imported_evidence_no_live_auth` | `downgrade_to_inspect_only` or `block_mutation` |

The resulting posture is data, not prose. Surfaces render the typed
state and may add explanatory copy, but the safety decision comes from
the packet.

## Action-Safety Reviews

`action_safety_review_record` is emitted before any protected action
executes and remains exportable after execution or block. The schema
defines eight verbs:

- `inspect`;
- `dry_run`;
- `apply`;
- `restart`;
- `scale`;
- `delete`;
- `publish`;
- `rollback`.

`inspect` uses `action_class = inspect`. `dry_run` uses
`action_class = compare_or_simulate`. All other verbs use
`action_class = mutate`.

### Required Review Fields

Every review packet carries:

- `target_context_ref` and `connector_session_ref`;
- `target_context_state_at_review` and
  `connector_session_state_at_review`;
- `target_confirmation` with loaded target ref, pre-execute target
  ref, confirmation state, actor, timestamp, and hidden-switch flags;
- `approval_posture` with approval state, ticket ref, policy ref,
  approver, timestamps, and summary;
- `action_envelope` with verb, target context ref, connector session
  ref, change summary, command preview ref, request preview ref,
  preview hash, dry-run result ref, rollback plan ref, and affected
  resource links;
- `safety_blockers[]`, `surface_bindings[]`, and `audit_refs[]`.

### Admission Decisions

`admission_decision = allow_inspect` is valid for read-only
inspection when the connector and policy permit it. Stale or imported
evidence may still inspect if clearly labeled.

`allow_dry_run` is valid for simulation or preview when inputs,
target context, tool identity, and partial-truth labels are present.
If simulation cannot resolve scope, the packet remains partial and
cannot be silently promoted to mutation.

`allow_mutation` is valid only when all of these are true:

- `target_context_state_at_review = current`;
- `connector_session_state_at_review = active`;
- `target_confirmation.confirmation_state = confirmed_exact`;
- `target_confirmation.no_hidden_target_switch = true`;
- `target_confirmation.hidden_switch_detected = false`;
- `approval_posture.approval_state = approved`;
- `action_envelope.preview_hash` is non-null.

`downgrade_to_inspect_only` means the surface may show current or
cached evidence but must disable mutation. `block` means the action
cannot proceed until the target, connector, approval, preview, or
source relationship is repaired and a new packet is created.

### Safety Blockers

The `safety_blockers[]` vocabulary is closed:

- `no_blocker`;
- `stale_context`;
- `ambiguous_target`;
- `mixed_authority`;
- `expired_credentials`;
- `connector_drift`;
- `hidden_target_switch`;
- `missing_preview`;
- `approval_missing_or_expired`;
- `policy_blocked`;
- `source_relationship_unresolved`;
- `live_target_unavailable`.

`no_blocker` is used only when the decision is allow-inspect,
allow-dry-run, or allow-mutation. Mutating actions with any other
blocker must downgrade or block.

## Source-To-Live Relationships

Target contexts and action envelopes share the same
`resource_relationship` shape. It distinguishes:

- authored source identity;
- rendered identity;
- planned or validated identity;
- observed live identity;
- provider overlays and imported evidence;
- explicit `no_source_match`.

Required relationship states are:

- `authored_to_rendered`;
- `rendered_to_planned`;
- `planned_to_live`;
- `authored_to_live`;
- `observed_to_authored`;
- `no_source_match`;
- `unknown_pending_review`.

When `relationship_state = no_source_match`, the packet must state why:

- `live_object_imported_out_of_band`;
- `source_deleted`;
- `provider_generated_resource`;
- `permission_limited`;
- `rendered_overlay_unavailable`;
- `ambiguous_multiple_sources`;
- `unknown_pending_review`.

A live action can therefore be traced to authored, rendered, planned,
or observed resource identity, or it is explicitly labeled
`no_source_match`. There is no unlabeled gap.

## Surface Reuse

The packet family is shared by:

- desktop action sheets and target-context chips;
- CLI JSON and dry-run output;
- support bundles and evidence exports;
- incident workspaces and runbook step records;
- browser companion and provider-console handoff packets;
- automation and AI action review surfaces.

Surface-specific UI may choose condensed labels, but exported and
programmatic forms keep the schema field names. A support engineer, a
CLI user, and a desktop reviewer should be able to join the same
`target_context_ref`, `connector_session_ref`, and `action_review_id`
without translating formats.

## Conformance

Conforming implementations must prove:

1. Live-action surfaces always name the exact target context they are
   acting on and never fall back to an unlabeled current environment.
2. Stale, ambiguous, mixed-authority, invalidated, unavailable, or
   imported-read-only contexts downgrade or block mutation through
   typed states.
3. Expired credentials, changed cluster context, connector drift,
   mixed local-plus-managed authority, and target ambiguity after load
   produce `invalidation_markers[]`.
4. Mutating verbs cannot reach `allow_mutation` without exact target
   confirmation, active connector session, approved posture, no hidden
   target switch, and a preview hash.
5. Browser handoffs are attributable transitions with destination,
   target context, evidence refs, and return anchors.
6. Imported evidence remains read-only and cannot grant live authority.
7. Source-to-rendered-to-live relationships survive through target
   context, connector session, action review, support export, and
   runbook packets, including explicit `no_source_match` cases.
