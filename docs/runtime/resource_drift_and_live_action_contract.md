# Resource-Target, Drift-Summary, and Live-Action Envelope Contract

This document freezes the runtime packet family Aureline uses when a
surface links source configuration to rendered artifacts, plans, live
resources, cached observations, imported snapshots, provider overlays,
and operational actions.

It exists so manifest editors, resource explorers, previews, incident
workspaces, runbooks, CLI output, support exports, and provider handoffs
can keep source, rendered, and live state separate without inventing
one-off labels or treating the most recent runtime snapshot as the
canonical source of truth.

The contract is normative. If it disagrees with the PRD, Technical
Architecture Document, Technical Design Document, UI / UX Spec, or
Design System Style Guide, those source documents win and this document
plus the companion schemas must be updated in the same change.

## Companion Artifacts

- [`/schemas/runtime/resource_target.schema.json`](../../schemas/runtime/resource_target.schema.json)
  - boundary schema for `resource_target_record` and its audit events.
  A resource target is a deployable unit, service, job, config, queue,
  database, endpoint, storage object, provider overlay, or other live
  resource with source refs, rendered refs, live handles, owner binding,
  target context, and route or connector binding.
- [`/schemas/runtime/drift_summary.schema.json`](../../schemas/runtime/drift_summary.schema.json)
  - boundary schema for `drift_summary_record` and its audit events.
  It distinguishes source-versus-rendered, rendered-versus-live,
  planned-versus-live, stale read, partial provider authority,
  imported snapshot, no-source-match, unknown drift, and mismatch
  discovered after an action preview was generated.
- [`/schemas/runtime/live_action_envelope.schema.json`](../../schemas/runtime/live_action_envelope.schema.json)
  - boundary schema for `live_action_envelope_record` and its audit
  events. It covers inspect, log tail, open dashboard, restart, scale,
  rollout, rollback, and edit actions with target identity, approval
  posture, actor, expiry, evidence, result refs, and rollback guidance.
- [`/fixtures/runtime/resource_drift_cases/`](../../fixtures/runtime/resource_drift_cases/)
  - worked cases for linked desired/rendered/live targets, drifted live
  state, stale imported snapshots, partial provider authority, and live
  actions blocked by post-preview drift.
- [`/docs/runtime/environment_connector_action_safety_contract.md`](./environment_connector_action_safety_contract.md)
  - target context, connector session, and action-safety review packet
  family consumed by these records.
- [`/docs/runtime/origin_target_route_taxonomy.md`](./origin_target_route_taxonomy.md)
  - route truth consumed by resource-target bindings and live-action
  envelopes.

Normative source projections:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` section on
  infrastructure-as-code and resource relationship truth layers.
- `.t2/docs/Aureline_Technical_Design_Document.md` section on
  resource target, drift snapshot, and live-action envelope contracts,
  plus its resource-target appendix.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` section on
  infrastructure and cluster-resource UX.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` sections on
  manifest rows, resource links, rendered/live compare, and apply
  previews.

## Scope

Frozen at this revision:

- one `resource_target_record` shape for deployable units, services,
  jobs, configs, queues, databases, endpoints, storage, topics,
  streams, runtime hosts, provider overlays, and other live resources;
- one `drift_summary_record` shape that names the comparison basis,
  coverage, provider authority, affected resource targets, evidence,
  partiality, stale reads, imported snapshots, and preview invalidation;
- one `live_action_envelope_record` shape for inspect, log tail, open
  dashboard, restart, scale, rollout, rollback, and edit actions;
- audit-event rows for resource-target, drift-summary, and live-action
  schema families;
- fixture cases proving support, incident, and export packets can
  reconstruct which resource target and drift state were in play when a
  live action occurred.

Out of scope at this revision:

- a live resource explorer or control plane;
- provider-specific API request bodies, raw manifests, raw plan blobs,
  raw log lines, raw console URLs, cookies, credentials, or tokens;
- final UI microcopy for drift banners or action sheets;
- shipping provider adapters for Kubernetes, cloud, database, queues,
  dashboards, or runbooks.

## Invariants

Every surface that emits or consumes this packet family follows these
rules:

1. **Truth layers are separate.** Desired source, rendered output,
   planned or validated output, live observation, provider overlay,
   cached live state, and imported snapshots remain separately named.
   A surface may summarize them together, but the packet keeps their
   refs independently addressable.
2. **Live is not canonical source.** A live handle or imported provider
   snapshot may explain current or historical runtime state, but it
   never becomes the authoritative source configuration unless an
   explicit source ref and reviewable transform say so.
3. **Target identity is mandatory.** Live handles and live-action
   envelopes carry `target_context_ref`; when a connector or route was
   involved they also carry `connector_session_ref` and
   `route_truth_ref` where available.
4. **Unknown is explicit.** Unknown drift, unknown target mapping,
   unknown provider authority, or unknown rollback guidance is a typed
   state that blocks or downgrades mutation. Silence is not a valid
   unknown state.
5. **Partial authority is data.** Provider-limited, permission-limited,
   timeout-limited, policy-redacted, stale, and imported-only reads are
   recorded as coverage and authority fields, not prose-only warnings.
6. **Preview drift invalidates mutation.** If desired, rendered, live,
   target, or provider authority state changes after a preview is
   generated, the drift summary names the stale preview and affected
   live-action envelopes. Mutation requires a renewed preview or
   explicit reapproval packet.
7. **Actions are reconstructable.** Any live action packet includes an
   explicit envelope, target binding, actor, approval posture, expiry,
   evidence refs, result refs, and rollback guidance. Support, incident,
   and export packets can join those refs without scraping UI text.

## Resource Targets

`resource_target_record` is the stable identity for a resource that can
appear as desired source, rendered artifact, planned result, live object,
provider overlay, cached live observation, or imported snapshot.

Required spine:

- `resource_target_id`, `display_label`, `resource_kind`,
  `stable_identity`, `state_class`, and `target_context_ref`;
- `source_refs[]` for authored, generated, template, policy, imported,
  or repository source identity;
- `rendered_refs[]` for rendered artifacts with renderer identity,
  renderer version, input set, digest, and freshness;
- `live_handles[]` for live, cached-live, provider-overlay, imported,
  or unknown object handles with provider class, target context,
  connector session, native kind, stable live UID, generation,
  observed time, freshness, and raw-response ref;
- `owner_binding` for team, service, repository, component, runbook,
  incident, provider-owner, manual, or unknown ownership;
- `route_or_connector_bindings[]` for target context, connector
  session, route truth, action review, browser handoff, authority
  linkage, and binding posture;
- `freshness_class`, `redaction_class`, timestamps, and `audit_refs[]`.

### Resource Target States

`desired_only` means source exists but no rendered, planned, or live
counterpart has been attached.

`rendered_only` means a renderer produced an artifact, but live
observation has not confirmed it.

`planned_only` means a plan or validation result exists but no live
counterpart is known.

`live_only` means a live object exists without a known source or
rendered artifact. Surfaces must not imply source ownership.

`linked_desired_rendered_live` means the record carries at least one
source ref, one rendered ref, and one live handle.

`drifted` means a drift summary has identified material divergence
between truth layers.

`cached`, `permission_limited`, `unavailable`, `imported_snapshot`, and
`unknown` are downgrade states. They can support inspection, comparison,
or export where policy permits, but they do not grant mutation
authority.

## Drift Summaries

`drift_summary_record` is the comparison packet for one or more resource
targets. It names the compared truth layers and whether the comparison
is complete, partial, stale, permission-limited, imported-only,
unknown, or invalidated after preview.

Required spine:

- `drift_summary_id`, `target_context_ref`, `compare_mode`,
  `overall_drift_posture`, `coverage_class`, and
  `provider_authority_class`;
- `tool_identity` with tool ref, name, version, and input set;
- `resource_target_refs[]`;
- `conditions[]` naming each source-versus-rendered,
  rendered-versus-live, source-versus-live, planned-versus-live,
  stale-read, partial-authority, provider-overlay-only,
  imported-snapshot, no-source-match, unknown, or post-preview mismatch
  condition;
- `preview_mismatch` naming stale preview refs and affected live-action
  envelopes when applicable;
- capture, expiry, support-export, and audit refs.

### Drift Conditions

`source_versus_rendered` records renderer or template output that no
longer matches source inputs.

`rendered_versus_live` records material divergence between rendered
intent and observed live state.

`source_versus_live` records divergence when no rendered layer is
available or the source maps directly to a live object.

`planned_versus_live` records a plan or dry-run result that no longer
matches the live target.

`stale_read` records cached or old observations. It must name freshness
and partial reason; it cannot claim current live truth.

`partial_provider_authority` records permission, provider API,
timeout, or policy limits that prevent complete live comparison.

`provider_overlay_only` records provider-owned context such as console
metadata or rollout panels that may inform the user but is not local
source truth.

`imported_snapshot` records support bundles, incident exports, browser
handoffs, or captured provider snapshots with no live mutation
authority.

`desired_live_mismatch_after_preview` records the case where a preview
or action envelope became stale before execution.

`no_source_match` records a live object with no known source, rendered,
or plan link.

`unknown_drift` records unresolved comparison state and blocks mutation
until a new comparison or review explains it.

## Live-Action Envelopes

`live_action_envelope_record` is emitted for any operational action
over a resource target. It is broader than the action-safety review
packet because it also covers observe, stream, and provider-dashboard
handoff actions, while still linking to action-safety review packets
for protected mutations.

Required spine:

- `live_action_envelope_id`, `action_kind`, `action_risk_class`,
  `result_state`, and `mutation_blockers[]`;
- `target_binding` with `resource_target_ref`, `target_context_ref`,
  `connector_session_ref`, `route_truth_ref`, `drift_summary_ref`, and
  `action_safety_review_ref`;
- `approval_posture` with approval state, ticket ref, policy ref,
  approver, timestamps, expiry, and summary;
- `actor_binding` with actor class, actor ref, originating surface,
  delegation ref, and session ref;
- `expiry_binding` with creation time, expiry, TTL, single-use posture,
  and revocation ref;
- `action_payload` with human review summary, command or request
  preview refs, preview hash, dry-run result ref, planned effect ref,
  and redaction class;
- `rollback_guidance` naming exact rollback, compensating action,
  handoff, read-only no-rollback, or unknown posture;
- `result_binding` with result, provider result, evidence, rollback,
  support export, and incident refs;
- `audit_refs[]`.

### Action Kinds

`inspect` is read-only observe. It still needs target identity and
freshness but may use `approval_state = not_required`.

`log_tail` opens a live stream and is boundary-raising. It must carry
expiry and evidence refs even when it does not mutate resource state.

`open_dashboard` is a handoff. It must preserve target context,
destination evidence, route truth, and no-live-mutation posture when
the dashboard is inspect-only.

`restart`, `scale`, `rollout`, `rollback`, and `edit` are live
mutations. They require explicit approval posture, preview or request
refs, target binding, drift summary where available, evidence refs, and
rollback guidance. An approved-ready or executing mutation requires an
approved posture and preview hash.

### Mutation Blockers

The blocker vocabulary is closed:

- `no_blocker`;
- `approval_missing_or_expired`;
- `preview_missing`;
- `drift_mismatch_after_preview`;
- `stale_or_imported_live_state`;
- `partial_provider_authority`;
- `target_context_not_current`;
- `connector_not_active`;
- `policy_blocked`;
- `rollback_guidance_missing`;
- `unknown_requires_review`.

Any blocker other than `no_blocker` downgrades, blocks, supersedes, or
requires reapproval before mutation. Read-only or handoff actions may
remain available if target and policy permit them.

## Honesty Rules

### Unknown Drift

Unknown drift is not "probably in sync". A surface with
`overall_drift_posture = unknown` or a condition
`unknown_drift` may inspect source, rendered output, or cached evidence
where policy permits, but it cannot offer mutation until the comparison
basis is renewed or explicitly reviewed.

### Cached Or Imported Live State

Cached live state and imported snapshots must carry freshness,
captured-at, source-system, and no-live-authority posture. They may
support compare, support export, incident review, and provider handoff,
but not direct live mutation.

### Provider-Limited Reads

Provider-limited reads must name `coverage_class`,
`provider_authority_class`, and `partial_reason`. A resource explorer
may show partial rows, but a drift summary with partial authority cannot
claim complete rendered-versus-live agreement.

### Desired-Versus-Live Mismatch After Preview

If source, rendered, planned, live, target-context, or provider-authority
state materially changes after a preview was generated, the drift
summary records `desired_live_mismatch_after_preview` or the closest
specific condition, and the live-action envelope records
`drift_mismatch_after_preview`. The old preview remains evidence but is
not executable authority.

## Support, Incident, And Export Reuse

Support bundles, incident workspaces, runbook packets, browser
companions, CLI output, and desktop surfaces reuse the same join keys:

- `resource_target_id`;
- `target_context_ref`;
- `connector_session_ref`;
- `route_truth_ref`;
- `drift_summary_id`;
- `live_action_envelope_id`;
- approval, evidence, result, rollback, incident, and support-export
  refs.

This means an export can reconstruct exactly which resource target,
target context, drift posture, preview, approval, actor, result, and
rollback path were in play without embedding raw provider payloads or
private credentials.

## Conformance

Conforming implementations must prove:

1. Desired, rendered, planned, live, cached, provider-overlay, and
   imported-snapshot truth layers remain separately addressable in
   resource-target and drift-summary packets.
2. Every live handle and live-action envelope names an explicit target
   context; mutation packets also name connector, route, approval,
   evidence, result, and rollback refs where applicable.
3. Unknown, stale, cached, imported, provider-limited, permission-
   limited, unavailable, and no-source-match states degrade honestly and
   do not imply complete live authority.
4. Post-preview drift invalidates the old live-action envelope or moves
   it to reapproval before mutation.
5. Support, incident, runbook, CLI, desktop, browser companion, and
   export packets can join resource target, drift summary, and live
   action records by id without translating per-surface labels.
