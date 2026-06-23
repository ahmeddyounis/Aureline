# Decision-history and audit-explorer contract

This document covers the *rendered* decision-history timelines and audit-event
explorers: the concrete, typed instances of the decision-history surface that
Aureline shows on its claimed managed-cloud, self-hosted, sovereign/air-gapped,
and mirrored/offline profiles.

Where the [admin-plane matrix](./m5-admin-plane.md) *names and freezes the
contract* — including the `decision_history_timeline` surface family, the states
it admits, the controlled vocabularies it binds, and the proof packet that keeps
it current — this lane *renders that surface*. It turns recent material allow /
deny / quota / force-disable / publish-scope decisions into a first-class local
product surface: a user or admin can read, on the machine in front of them, what
was decided, who or what decided it, the policy epoch and affected scope it
applied to, when it happened, and where to read the full explanation — without
scraping logs or opening a separate vendor console.

If this document, the companion schema, and the worked fixture disagree, the
normative sources in `.t2/docs/` win and this document plus its companions update
in the same change.

## Companion artifacts

- [`/schemas/admin/m5-decision-history.schema.json`](../../schemas/admin/m5-decision-history.schema.json)
  — boundary schema for `m5_decision_history_bundle`.
- [`/fixtures/admin/m5-decision-history/canonical_history.json`](../../fixtures/admin/m5-decision-history/canonical_history.json)
  — the published canonical decision-history bundle; the freeze gate asserts the
  in-code builder equals it byte-for-byte.
- [`/artifacts/admin/m5-decision-history.md`](../../artifacts/admin/m5-decision-history.md)
  — the human-readable companion (per-profile timeline tables).
- `crates/aureline-policy/src/m5_decision_history/` — the builder, invariants,
  validation, and human-readable projection.
- `cargo run -p aureline-policy --example dump_m5_decision_history` — the headless
  emitter (JSON, or `-- --lines` for the projection).

## Binds back to the matrix

The render layer is not free-form. Each timeline binds back to the frozen
[admin-plane matrix](./m5-admin-plane.md):

- **Every state it shows is one the matrix admits.** An event's `outcome_state`
  and the timeline's `coverage.coverage_state` must each be in the matrix's
  `applicable_states` for the decision-history surface — `active_enforced`,
  `unconfirmed_stale`, `mirror_offline_last_known`, `imported_snapshot_no_live`,
  or `unknown_requires_review` (`decision_history.surface_states_within_matrix`).
- **Every token it uses is a matrix term.** The `owner_escalation`,
  `data_residency_class`, and freshness tokens are exactly the terms the matrix's
  shared vocabulary defines.

So an edit that shows a state the matrix does not admit, or a token the matrix
does not define, flips an invariant and fails the freeze gate.

## Decision events

Each row is a material decision. It names:

- a **stable event id** and a **stable decision code** (`allow`, `deny`,
  `narrow`, `force_disable`, `quota_limit`, `defer_pending_refresh`,
  `defer_pending_admin`, `escalate`, `export_only`, `local_only_continue`,
  `mutation_recorded`, `request_recorded`, `rollback_recorded`,
  `unknown_offline`),
- a distinguished **actor class** — `user_action`, `admin_action`,
  `policy_evaluation`, `provider_limitation`, or `client_limitation` — so a
  provider's refusal or a client's offline limit is surfaced as itself rather
  than collapsed into a generic blocked/error row
  (`decision_history.actor_classes_distinguished`),
- the **policy epoch** (and the **entitlement epoch** where entitlement-bearing),
- the **affected target** and **scope** (`tenant_or_org`, `workspace`, `seat`,
  `device`, `capability_scope`, `session_or_command`, …),
- the **event time** and a monotonic **sequence**,
- the **outcome state** (bound to the matrix vocabulary) and **evidence
  freshness**, the **data residency**, and the **owner** of the next step,
- an **explanation link** for force-disable decisions (to a locked-state
  explanation or capability explanation),
- and both a **machine-readable summary** and a **plain-language handoff
  sentence** (`decision_history.export_parity`).

An event whose backing evidence is stale is never shown under a confirmed
`active_enforced` state; offline and imported rows use an explicit non-confirmed
state instead (`decision_history.no_silent_green`).

## Audit-event explorer filters

Every timeline offers a filter for each of the eight audit families the spec
requires, and every event resolves to exactly one of them
(`decision_history.explorer_filters_complete`):

| Filter | Family token | Selects |
| --- | --- | --- |
| Trust | `trust_change` | Trust-root, signer, and verification posture changes |
| Policy | `policy_change` | Policy-bundle / effective-policy changes |
| Auth | `auth_session` | Authentication and session lifecycle events |
| Remote mutation | `remote_mutation` | Writes against a remote / managed target |
| Provider routing | `provider_routing` | AI/provider routing and network egress decisions |
| Collaboration control | `collaboration_control` | Collaboration-control grants and revocations |
| Publish state | `publish_state` | Publish-scope / marketplace publication changes |
| Managed identity scope | `managed_identity_scope` | Org switch, seat, and directory-scope changes |

## Export parity

Every row is exportable two ways and a timeline offers both forms:

- a **machine-readable summary** (`machine_readable_json`) — stable codes, actor
  class, policy epoch, scope, and time as JSON summary objects, for tooling, and
- a **plain-language handoff packet** (`plain_language_handoff`) — the same rows
  as reviewable sentences for a support or admin handoff, with no raw payloads.

## Coverage and offline retention

Each timeline labels its coverage window with a completeness class (`complete`,
`partial_offline`, `partial_imported`, `partial_redacted`) and a coverage note,
so a partial history is never presented as complete
(`decision_history.coverage_labeled`). Every profile — including self-hosted,
sovereign/air-gapped, and mirrored/offline — keeps a locally inspectable history
that does not require a vendor console or control plane
(`decision_history.locally_inspectable_offline`).

## Profiles covered

The bundle renders one packet per claimed managed-bearing profile:
`managed_cloud`, `self_hosted`, `sovereign_air_gapped`, and `mirrored_offline`.
Each maps to a matrix admin path and a deployment profile.

## Cross-surface parity

There is exactly **one typed packet per profile**, and each packet declares the
consumers the matrix maps to this surface: shell admin center, CLI/headless
inspect, support export, procurement, and managed-service. Because every consumer
serializes the same packet, the decision history is identical across UI, CLI,
support export, procurement, and managed-service surfaces by construction
(`decision_history.consumer_parity`).

## Invariants

The builder computes each invariant's `holds` flag from the rendered data, so an
inconsistent edit flips an invariant and fails the freeze gate.

- `decision_history.surface_states_within_matrix` — every rendered state is one
  the frozen matrix admits for the decision-history surface.
- `decision_history.decision_truth` — every event names a stable id, decision
  code, policy epoch, affected target and scope, and time; ids are unique.
- `decision_history.actor_classes_distinguished` — every event names a specific
  actor class and each timeline uses at least two distinct classes.
- `decision_history.actor_classes_all_present` — every actor class appears across
  the bundle, so provider and client limitations are surfaced as themselves.
- `decision_history.explorer_filters_complete` — every timeline offers all eight
  family filters and every event resolves to exactly one.
- `decision_history.export_parity` — every row carries both export
  representations and every timeline offers both export forms.
- `decision_history.no_silent_green` — stale evidence never sits under a
  confirmed active/enforced state.
- `decision_history.locally_inspectable_offline` — every profile keeps a locally
  inspectable, vendor-console-independent history.
- `decision_history.coverage_labeled` — a partial history is labeled, never
  implied complete.
- `decision_history.ownership_visible` — every event names an owner and every
  force-disable links to an explanation.
- `decision_history.consumer_parity` — one typed packet serves every consumer the
  matrix declares for this surface identically.
- `decision_history.profiles_covered` — the managed-cloud, self-hosted,
  sovereign/air-gapped, and mirrored/offline profiles are all rendered.
- `decision_history.export_safe` — every stable id is an opaque token with no URL
  scheme or absolute path.

## Export safety

The record carries no endpoint URLs, hostnames, credentials, raw provider
payloads, raw policy bodies, or absolute paths — only opaque object refs, stable
tokens, rendered metadata-safe summaries, and short reviewable sentences.
`is_support_export_safe()` enforces that `raw_payload_excluded` is true, every
file ref is repo-relative, and every stable token id is opaque, so the bundle is
safe to embed in a support export verbatim.

## Composes with

This contract renders the decision-history surface the
[admin-plane matrix](./m5-admin-plane.md) freezes, alongside the
[admin-plane render](./m5-admin-render.md) layer (its locked-state explanations
are the targets of this surface's force-disable explanation links), and composes
with the [audit-event explorer contract](./audit_event_explorer_contract.md) the
matrix binds for the durable audit rows.

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_decision_history > \
  fixtures/admin/m5-decision-history/canonical_history.json

# Freeze gate: in-code bundle must equal the checked-in fixture
cargo test -p aureline-policy --test m5_decision_history

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_decision_history -- --lines
```
