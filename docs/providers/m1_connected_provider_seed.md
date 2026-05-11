# Connected-account registry, provider descriptor, browser-handoff packet seed, and publish-later object-model seed

Reviewer-facing landing page for the connected-account / provider
registry seed shipped on the protected M1 provider-linked entry path.
This page is the named runtime consumer wired on the validation lane;
keep it in lock-step with the schemas, the seed-row matrix, and the
proof packet listed below.

## Why this seed exists

Every protected M1 entry path that touches a hosted provider has to
answer the same five questions before it can render an affordance:

1. *Which provider is Aureline routing to, and under which descriptor
   (review/code host, issue tracker, CI/check provider, docs portal,
   identity provider, package registry, release publisher, AI provider,
   managed admin)?*
2. *Which actor class is the entry acting as, and is that actor class
   admitted by the descriptor?*
3. *What is the entry's disposition — `immediate_mutation_in_product`,
   `browser_handoff_required`, `publish_later_required`, or
   `inspect_only_no_mutation` — and is the disposition admitted by the
   descriptor's capabilities?*
4. *If the disposition is a deferred one, which publish-later object
   seed is the entry minting, and is its identity distinguishable from
   an immediate mutation?*
5. *Is the freshness of the underlying observation honest — fresh,
   stale within window, expired, or never observed — and does the
   registry status agree?*

Without one frozen seed: the review surface invents a "Connected"
state, the release surface invents another, CI invents a third, the
docs surface invents a fourth, and a deferred publish silently
collapses into the same "Open in browser" control as an immediate
mutation. This seed freezes one typed shape every protected entry path
reads.

## What this seed owns

This lane lands a **seed**, not a productized provider stack. It
freezes the shapes; it does not ship live account linking, live token
exchange, or live publish workflows.

Owned outputs:

- [`/schemas/providers/connected_account_registry.schema.json`](../../schemas/providers/connected_account_registry.schema.json)
  — the registry-entry, provider-descriptor, and publish-later
  object-seed vocabulary.
- [`/schemas/providers/browser_handoff_packet.schema.json`](../../schemas/providers/browser_handoff_packet.schema.json)
  — the provider-entry browser-handoff packet seed, return summary, and
  packet audit-event vocabulary the registry mints.
- [`/fixtures/providers/connected_provider_seed_rows/m1_connected_provider_seed_rows.yaml`](../../fixtures/providers/connected_provider_seed_rows/m1_connected_provider_seed_rows.yaml)
  — six seed rows covering every required actor class and every
  required mutation disposition class.
- [`/tests/providers/m1_connected_provider_seed_lane/`](../../tests/providers/m1_connected_provider_seed_lane/)
  — the unattended validation lane that replays every row and reproduces
  each named failure drill.
- [`/artifacts/milestones/m1/proof_packets/connected_provider_seed.md`](../../artifacts/milestones/m1/proof_packets/connected_provider_seed.md)
  — the owning proof packet under the governed proof root.

## What this seed composes with

- [`/schemas/integration/browser_handoff_packet.schema.json`](../../schemas/integration/browser_handoff_packet.schema.json)
  — the canonical browser-handoff packet record (frozen by ADR-0010).
  The provider-entry packet seed references this record through
  `integration_browser_handoff_packet_ref`; it does not redefine it.
- [`/schemas/providers/connected_account_record.schema.json`](../../schemas/providers/connected_account_record.schema.json)
  — the canonical connected-account sub-records (human account link,
  install grant, delegated credential, project-scoped grant,
  policy-injected service identity, acting-identity badge, invalidation
  event). The registry entry references one of these through
  `connected_provider_record_ref`; the registry does not duplicate them.
- [`/schemas/providers/publish_later_record.schema.json`](../../schemas/providers/publish_later_record.schema.json)
  — the canonical publish-later queue items, review records,
  consequence previews, account mappings, and provider-object
  relations. The publish-later object seed in this lane references
  queue items through `publish_later_queue_item_ref`; it does not
  duplicate the queue.
- [`/schemas/providers/browser_handoff_sheet.schema.json`](../../schemas/providers/browser_handoff_sheet.schema.json)
  — the user-facing typed disclosure that mediates one handoff. A
  packet seed minted under this lane is the upstream the sheet mediates
  for.

If this page disagrees with any of those schemas, those schemas win and
this page plus the seed-row matrix are updated in the same change.

## Frozen vocabularies

The registry schema freezes the following closed vocabularies; the
seed-row matrix mirrors them verbatim. Adding a new value is
additive-minor and bumps `connected_account_registry_schema_version`;
repurposing an existing value is breaking.

- `mutation_disposition_class` — `immediate_mutation_in_product`,
  `browser_handoff_required`, `publish_later_required`,
  `inspect_only_no_mutation`. Surfaces MUST render exactly one
  disposition at the point of intent; `publish_later_required` and
  `immediate_mutation_in_product` MAY NOT collapse into one control.
- `freshness_class` — `never_observed`, `fresh`, `stale_within_window`,
  `expired_beyond_window`. `expired_beyond_window` entries MUST route
  to a re-observe path rather than claim a current state.
- `registry_entry_status` — `seeded`, `live_observed`,
  `degraded_observed`, `unobserved_pending`, `revoked_or_disconnected`.
  `live_observed` entries MUST NOT claim `primary_actor_class`
  `unknown_actor_class`.
- `publish_later_object_class` — `local_draft_only`,
  `queued_publish_pending`, `browser_handoff_pending`,
  `deferred_publish_in_queue`, `imported_read_only_snapshot`.
  Surfaces MUST render exactly one class.
- `provider_actor_class` — `human_account`,
  `installation_or_app_grant`, `delegated_user_token`,
  `project_scoped_grant`, `policy_injected_service_identity`,
  `unknown_actor_class`. The vocabulary mirrors
  `/schemas/integration/browser_handoff_packet.schema.json`.
- `provider_capability_class` — `supports_immediate_mutation`,
  `supports_browser_handoff`, `supports_publish_later`,
  `supports_inspect_only`, `supports_account_switch`,
  `supports_step_up_authenticator`. Entries MUST NOT request a
  disposition the descriptor does not declare.

## Seed rows covered

The validation matrix seeds six protected entry paths so each frozen
mutation disposition and each actor class is exercised exactly once:

| Row | Actor class | Disposition | Notes |
| --- | --- | --- | --- |
| `provider_entry:human_account_review_publish_in_product`             | `human_account`                       | `immediate_mutation_in_product` | Fresh observation; no publish-later seed. |
| `provider_entry:install_grant_ci_check_publish_later`                | `installation_or_app_grant`           | `publish_later_required`        | `queued_publish_pending` with a `publish_later_queue_item_ref`. |
| `provider_entry:delegated_credential_release_publish_browser_only`   | `delegated_user_token`                | `browser_handoff_required`      | Mints a `provider_entry_browser_handoff_packet_record` with `publish_requires_browser_auth`. |
| `provider_entry:project_scoped_grant_inspect_only_snapshot`          | `project_scoped_grant`                | `inspect_only_no_mutation`      | No publish-later seed; `imported_read_only_snapshot` posture. |
| `provider_entry:policy_injected_service_publish_later_admin_approval`| `policy_injected_service_identity`    | `publish_later_required`        | `deferred_publish_in_queue` under admin approval. |
| `provider_entry:unknown_actor_class_repair_required`                 | `unknown_actor_class`                 | `inspect_only_no_mutation`      | Repair-only; `never_observed` freshness, `seeded` status. |

## Acceptance evidence

- The seed is machine-readable, versioned, and consumed by at least one
  live surface, export path, or CI/review check: the validation lane
  is registered in
  [`artifacts/milestones/m1/artifact_index.yaml`](../../artifacts/milestones/m1/artifact_index.yaml)
  under `proof_lanes.connected_provider_seed`.
- The connected-provider seed model captures **origin**, **target**,
  and **authority-relevant metadata** explicitly through
  `origin_disclosure`, `target_disclosure`, `actor_scope`, and the
  `expected_authority_on_destination` on the packet seed.
- At least one protected M1 provider-linked entry path
  (`provider_entry:delegated_credential_release_publish_browser_only`)
  is expressed using the shared handoff packet format.
- **Publish-later objects remain distinguishable from immediate
  mutation paths**: the matrix enforces that
  `publish_later_required` / `browser_handoff_required` entries cite at
  least one `publish_later_object_seed`; `inspect_only_no_mutation`
  entries cite none; and the failure drills reproduce these rules
  loudly when forced.
- Later browser handoff or publish-later work can consume the same
  registry shape without schema churn: the registry schema's record
  set is `oneOf {entry, descriptor, publish-later seed}` and the
  packet-seed schema's record set is `oneOf {packet, return, audit
  event}`; future lanes add fields additively under
  `connected_account_registry_schema_version` /
  `provider_entry_browser_handoff_schema_version`.

## How to inspect

```bash
python3 tests/providers/m1_connected_provider_seed_lane/run_m1_connected_provider_seed_lane.py --repo-root .
```

The runner writes the latest capture to
`artifacts/milestones/m1/captures/connected_provider_seed_validation_capture.json`
and prints `PASS` or `FAIL` to stdout with a per-row diagnostics map.

## Failure drills

The matrix names one drill per row with a typed `expected_check_id`.
Force any drill with `--force-drill <row_id>:<drill_id>` and the
runner exits `0` only when the precise expected check id reproduces.
See the lane's [README](../../tests/providers/m1_connected_provider_seed_lane/README.md)
for the full drill table.

## Owner & refresh

- Owner: `@ahmeddyounis`.
- Refresh: re-run the validation lane whenever the registry schema,
  the packet schema, the seed-row matrix, or this landing page changes.
- Out of scope: live account linking, live token exchange, hosted
  publish-queue execution, broad provider marketplace depth, and
  account-sync productization. Those land on later lanes that read
  this seed as their canonical contract.
