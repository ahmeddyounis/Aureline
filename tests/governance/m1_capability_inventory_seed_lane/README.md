# M1 capability-inventory seed validation lane

Unattended proof lane that validates the M1 capability-inventory seed
at
[`artifacts/governance/capability_inventory_seed.yaml`](../../../artifacts/governance/capability_inventory_seed.yaml)
against:

- [`schemas/governance/capability_inventory.schema.json`](../../../schemas/governance/capability_inventory.schema.json) — envelope schema (vocabularies, required coverage, named consumers, required M1 surface coverage list);
- [`schemas/governance/capability_inventory_entry.schema.json`](../../../schemas/governance/capability_inventory_entry.schema.json) — row vocabulary; and
- the canonical landing page at
  [`docs/governance/capability_lifecycle_seed.md`](../../../docs/governance/capability_lifecycle_seed.md)
  so the seed cannot quietly point at a missing reviewer entry.

The lane is deliberately runnable on CI / nightly without a graphical
display: it only consumes the pure data joined out of the canonical
sources.

## What the lane proves

For every row in the seed the runner asserts:

- **Envelope is canonical** — `record_kind` is
  `capability_inventory_entry_record` and
  `capability_inventory_entry_schema_version` is `1`.
- **`capability_id` is unique** and matches the row schema's pattern
  (`capability_inventory.capability_id_pattern_invalid`,
  `capability_inventory.entries_duplicate_capability_id`).
- **Closed vocabularies match the row schema verbatim** for
  `capability_kind`, `surface_families`, `lifecycle_state`,
  `public_label_policy`, `public_claim_posture`, `export_visibility`,
  and `rollout_gate.gate_kind`.
- **Owner / lane / claim metadata is present** —
  `owner_dri` is a non-empty `@handle`
  (`capability_inventory.owner_dri_required`,
  `capability_inventory.owner_dri_pattern_invalid`),
  `owning_lane` is non-empty,
  `claim_lanes` and `dependency_marker_refs` are lists.
- **`lifecycle_state` is non-empty** and a member of the canonical
  Labs / Preview / Beta / Stable / Deprecated vocabulary
  (`capability_inventory.lifecycle_state_required`,
  `capability_inventory.lifecycle_state_unknown`).
- **`public_label` agrees with `public_label_policy`** — null when
  `public_label_forbidden`, non-empty when `public_label_required`
  (`capability_inventory.public_label_required_when_policy_required`,
  `capability_inventory.public_label_must_be_null_when_forbidden`).
- **`public_claim_posture = forbidden`** forces `claim_lanes` empty
  and `export_visibility` to `internal_redacted`
  (`capability_inventory.forbidden_claim_has_no_claim_lanes`,
  `capability_inventory.forbidden_claim_is_internal_redacted`).
- **Support-export widening is blocked** — `support_capability` rows
  must keep `export_visibility` in
  `{support_export_only, internal_redacted}`
  (`capability_inventory.support_export_visibility_widening_blocked`).
- **Pre-stable rows disclose their gate** — rows whose
  `lifecycle_state` is in `{labs, preview, beta}` and whose
  `rollout_gate` is non-null MUST set
  `public_disclosure_required = true`
  (`capability_inventory.rollout_gate_public_disclosure_required_for_pre_stable`).
- **Rollout-gated rows publish a kill-switch** — rows that declare a
  non-null `rollout_gate` MUST also publish a non-empty
  `kill_switch_path`
  (`capability_inventory.kill_switch_path_required_when_rollout_gate_present`).
- **Retiring rows publish a window note** — rows whose
  `lifecycle_state` is in `{deprecated, disabled_by_policy, retired}`
  MUST carry a non-null `retirement_metadata` block with a non-empty
  `retirement_target_window_note`
  (`capability_inventory.retirement_metadata_required_for_retiring_lifecycle_state`,
  `capability_inventory.retirement_target_window_note_required`).

For every M1-bearing surface row (`m1_surface_seed_membership: true`)
the runner additionally asserts:

- **The row's `capability_id` is in `required_m1_surface_coverage`**
  (`capability_inventory.m1_surface_seed_membership_not_in_required_coverage`).
- **The row carries a non-null `failure_drill`** whose `drill_id` is
  in `failure_drill_id_vocabulary`, whose `forced_input` declares at
  least one drift, and whose `expected_check_id` and
  `actionable_next_action` are non-empty
  (`capability_inventory.failure_drill_required_for_m1_surface_seed`,
  `capability_inventory.failure_drill_drill_id_unknown`,
  `capability_inventory.failure_drill_forced_input_empty`).

Envelope coverage:

- **Required lifecycle coverage** — every member of
  `required_lifecycle_state_coverage`
  (`labs, preview, beta, stable, deprecated`) appears at least once
  across the entries
  (`capability_inventory.coverage_missing_required_lifecycle_states`).
- **Required M1 surface coverage** — every member of
  `required_m1_surface_coverage` is present as an
  `m1_surface_seed_membership: true` row
  (`capability_inventory.coverage_missing_required_m1_surfaces`).
- **Named runtime consumers exist** — every
  `named_runtime_consumers[].consumer_ref` resolves on disk and
  `consumed_fields` is non-empty
  (`capability_inventory.named_runtime_consumer_ref_missing`,
  `capability_inventory.named_runtime_consumer_consumed_fields_empty`).

## Run

```bash
python3 tests/governance/m1_capability_inventory_seed_lane/run_m1_capability_inventory_seed_lane.py --repo-root .
```

The runner emits a deterministic JSON capture to
`artifacts/milestones/m1/captures/capability_inventory_seed_validation_capture.json`
and exits non-zero on any check failure.

### Force a named failure drill

```bash
python3 tests/governance/m1_capability_inventory_seed_lane/run_m1_capability_inventory_seed_lane.py \
    --repo-root . \
    --force-drill <capability_id>:<drill_id>
```

In `--force-drill` mode the runner exits `0` only when the row's
declared `expected_check_id` is reproduced by the forced input.

| Row (`capability_id`) | Drill | Expected check id |
| --- | --- | --- |
| `shell.frame` | `capability_inventory_drill.shell_frame_lifecycle_state_dropped` | `capability_inventory.lifecycle_state_required` |
| `shell.start_center` | `capability_inventory_drill.start_center_kill_switch_path_dropped` | `capability_inventory.kill_switch_path_required_when_rollout_gate_present` |
| `workspace.entry` | `capability_inventory_drill.workspace_entry_owner_dri_dropped` | `capability_inventory.owner_dri_required` |
| `editor.basics` | `capability_inventory_drill.editor_basics_public_label_dropped` | `capability_inventory.public_label_required_when_policy_required` |
| `command.quick_open` | `capability_inventory_drill.quick_open_rollout_gate_disclosure_dropped` | `capability_inventory.rollout_gate_public_disclosure_required_for_pre_stable` |
| `search.shell_search` | `capability_inventory_drill.search_shell_lifecycle_state_drifted_to_unknown_token` | `capability_inventory.lifecycle_state_unknown` |
| `terminal.embedded_seed` | `capability_inventory_drill.terminal_seed_widened_to_retired_without_window_note` | `capability_inventory.retirement_metadata_required_for_retiring_lifecycle_state` |
| `support.export_seed` | `capability_inventory_drill.support_export_export_visibility_widened_to_public` | `capability_inventory.support_export_visibility_widening_blocked` |
| `help.about_pane` | `capability_inventory_drill.help_about_lifecycle_state_dropped` | `capability_inventory.lifecycle_state_required` |

Optional flags:

- `--matrix <path>` — point at an alternate seed file.
- `--envelope-schema <path>` — alternate envelope schema.
- `--row-schema <path>` — alternate row schema.
- `--build-identity <path>` — alternate build-identity record.
- `--report <path>` — change the capture output path.

## Where the evidence lives

| Artifact | Path |
| --- | --- |
| Reviewer landing page | `docs/governance/capability_lifecycle_seed.md` |
| Seed (canonical) | `artifacts/governance/capability_inventory_seed.yaml` |
| Envelope schema | `schemas/governance/capability_inventory.schema.json` |
| Row schema | `schemas/governance/capability_inventory_entry.schema.json` |
| Latest capture | `artifacts/milestones/m1/captures/capability_inventory_seed_validation_capture.json` |
| Owning proof packet | `artifacts/milestones/m1/proof_packets/capability_lifecycle_seed.md` |

The lane is registered in
[`artifacts/milestones/m1/artifact_index.yaml`](../../../artifacts/milestones/m1/artifact_index.yaml)
under `proof_lanes.capability_inventory_seed` so reviewers can find the
latest capture, owner, and validation-lane reference without searching
ad hoc folders.

## Refresh policy

Re-run the lane (and refresh the capture) when any of the following
change:

- `artifacts/governance/capability_inventory_seed.yaml`
- `schemas/governance/capability_inventory.schema.json`
- `schemas/governance/capability_inventory_entry.schema.json`
- `docs/governance/capability_lifecycle_seed.md`
- any named runtime consumer's path or read fields
