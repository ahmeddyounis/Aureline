# M1 connected-provider seed validation lane

Unattended proof lane that validates
[`fixtures/providers/connected_provider_seed_rows/m1_connected_provider_seed_rows.yaml`](../../../fixtures/providers/connected_provider_seed_rows/m1_connected_provider_seed_rows.yaml)
against
[`schemas/providers/connected_account_registry.schema.json`](../../../schemas/providers/connected_account_registry.schema.json)
and
[`schemas/providers/browser_handoff_packet.schema.json`](../../../schemas/providers/browser_handoff_packet.schema.json),
and cross-checks each row against the canonical provider record set
in [`schemas/providers/connected_account_record.schema.json`](../../../schemas/providers/connected_account_record.schema.json)
and [`schemas/providers/publish_later_record.schema.json`](../../../schemas/providers/publish_later_record.schema.json).

The lane is deliberately runnable on CI / nightly without a graphical
display: it only consumes the pure data joined out of the canonical
sources.

## What the lane proves

For every row in the matrix the runner asserts:

- **`row_id` is namespaced** under `provider_entry:` and is unique.
- **Discriminators are honored** — descriptor `entry_kind` is
  `provider_descriptor_record`, entry `entry_kind` is
  `connected_account_registry_entry_record`, publish-later seeds carry
  `publish_later_object_seed_record`, and packet seeds carry
  `provider_entry_browser_handoff_packet_record`.
- **Closed vocabularies match the schemas verbatim** —
  `mutation_disposition_class`, `freshness_class`,
  `registry_entry_status`, `publish_later_object_class`,
  `provider_actor_class`, and `provider_capability_class` all match the
  registry schema's `$defs`.
- **Descriptor admits the entry** — the entry's
  `actor_scope.primary_actor_class` is in the descriptor's
  `supported_actor_classes`, and the entry's
  `mutation_disposition_class` resolves against the descriptor's
  `supported_capabilities`.
- **Publish-later object coverage** — `publish_later_required` and
  `browser_handoff_required` entries cite at least one
  `publish_later_object_seed_ref`; `inspect_only_no_mutation` entries
  cite none; `browser_handoff_required` entries cite a non-empty
  `browser_handoff_routing_summary_ref`.
- **Publish-later seed rules** — `local_draft_only` and
  `imported_read_only_snapshot` seeds carry the allowed dispositions
  only; `queued_publish_pending` and `deferred_publish_in_queue` seeds
  cite a non-empty `publish_later_queue_item_ref`; and
  `browser_handoff_pending` seeds carry `browser_handoff_required`.
- **Freshness & status agree** — `never_observed` entries carry
  `registry_entry_status` `seeded` or `unobserved_pending`;
  `live_observed` entries never carry `primary_actor_class`
  `unknown_actor_class`.
- **Packet seed disposition agrees with the entry** — when a row
  defines a packet seed, its `mutation_disposition_class` must match
  the entry's.
- **Required coverage is met** — at least one row exists for each
  member of `required_mutation_disposition_coverage` (every
  `mutation_disposition_class`) and `required_actor_coverage` (every
  `provider_actor_class`).
- **Failure drills are reproducible** — every row names one drill in
  `failure_drill_id_vocabulary` plus the precise `expected_check_id`
  the runner reproduces when the drill is forced with `--force-drill`.

## Run

```bash
python3 tests/providers/m1_connected_provider_seed_lane/run_m1_connected_provider_seed_lane.py --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/milestones/m1/captures/connected_provider_seed_validation_capture.json`
and exits non-zero on any check failure.

### Force a named failure drill

```bash
python3 tests/providers/m1_connected_provider_seed_lane/run_m1_connected_provider_seed_lane.py \
    --repo-root . \
    --force-drill <row_id>:<drill_id>
```

In `--force-drill` mode the runner exits `0` only when the row's
declared `expected_check_id` is reproduced by the forced input.

| Row | Drill | Expected check id |
|---|---|---|
| `provider_entry:human_account_review_publish_in_product`         | `human_account_drill.disposition_not_in_descriptor_capabilities` | `connected_provider_seed.mutation_disposition_not_supported_by_descriptor` |
| `provider_entry:install_grant_ci_check_publish_later`            | `install_grant_drill.publish_later_seed_missing_for_publish_later` | `connected_provider_seed.publish_later_seed_required` |
| `provider_entry:delegated_credential_release_publish_browser_only` | `delegated_credential_drill.browser_handoff_summary_missing`     | `connected_provider_seed.browser_handoff_routing_summary_required` |
| `provider_entry:project_scoped_grant_inspect_only_snapshot`      | `project_scoped_drill.inspect_only_seed_present`                 | `connected_provider_seed.inspect_only_must_have_no_publish_later_seed` |
| `provider_entry:policy_injected_service_publish_later_admin_approval` | `policy_injected_drill.publish_later_queue_item_missing`     | `connected_provider_seed.publish_later_queue_item_required` |
| `provider_entry:unknown_actor_class_repair_required`             | `unknown_actor_drill.live_observed_under_unknown_actor`          | `connected_provider_seed.live_observed_must_not_carry_unknown_actor` |

Optional flags:

- `--matrix <path>` — point at an alternate matrix file.
- `--registry-schema <path>` — alternate registry schema.
- `--packet-schema <path>` — alternate packet schema.
- `--report <path>` — change the capture output path.
- `--build-identity <path>` — change which build identity record is
  embedded in the capture.

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Reviewer landing page | `docs/providers/m1_connected_provider_seed.md` |
| Seed-row matrix | `fixtures/providers/connected_provider_seed_rows/m1_connected_provider_seed_rows.yaml` |
| Registry schema | `schemas/providers/connected_account_registry.schema.json` |
| Packet schema | `schemas/providers/browser_handoff_packet.schema.json` |
| Latest capture | `artifacts/milestones/m1/captures/connected_provider_seed_validation_capture.json` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/connected_provider_seed.md` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
under `proof_lanes.connected_provider_seed` so reviewers can find the
latest capture, owner, and validation-lane reference without searching
ad hoc folders.

## Refresh policy

Re-run the lane (and refresh the capture) when any of the following
change:

- `fixtures/providers/connected_provider_seed_rows/m1_connected_provider_seed_rows.yaml`
- `schemas/providers/connected_account_registry.schema.json`
- `schemas/providers/browser_handoff_packet.schema.json`
- `docs/providers/m1_connected_provider_seed.md`
- the connected-account record or publish-later record vocabularies it
  composes with.
