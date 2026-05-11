# Proof packet: M1 capability-inventory seed (Labs / Preview / Beta / Stable / Deprecated)

Purpose: anchor proof captures for the unattended M1 lane that
validates the canonical capability-inventory seed. The lane proves
the seed is consumable by docs, help/About, CLI/headless help,
support exports, compatibility reports, release notes, and the
existing CI contract-artifacts validator — without re-encoding the
lifecycle vocabulary or the M1 surface coverage on each surface.

Reviewer entry point:
[`/docs/governance/capability_lifecycle_seed.md`](../../../docs/governance/capability_lifecycle_seed.md).

## Canonical sources

- `artifacts/governance/capability_inventory_seed.yaml` — seed rows
  the runner consumes. Carries:
  - the M1 envelope (`schema_version`, `matrix_id`, `owner_dri`,
    `overview_page`, `entry_schema_ref`, `build_identity_ref`,
    `validation_lane_ref`),
  - closed envelope vocabularies
    (`lifecycle_state_vocabulary`, `failure_drill_id_vocabulary`,
    `required_lifecycle_state_coverage`,
    `required_m1_surface_coverage`),
  - the named runtime consumers the seed asserts are live, and
  - one inventory row per capability, each with a uniform
    `(capability_id, capability_kind, surface_families[],
    lifecycle_state, owner_dri, owning_lane, claim_lanes[],
    dependency_marker_refs[], rollout_gate, public_label,
    public_label_policy, public_claim_posture, export_visibility,
    kill_switch_path, retirement_metadata)` shape.
  - M1-bearing rows additionally carry
    `m1_surface_seed_membership: true` and a named `failure_drill`
    block.

- `schemas/governance/capability_inventory.schema.json` — envelope
  schema; freezes vocabularies, required coverage lists, named
  consumer shape, and matrix identity.

- `schemas/governance/capability_inventory_entry.schema.json` — row
  schema; freezes the closed lifecycle, kind, surface-family,
  rollout-gate, label-policy, claim-posture, and export-visibility
  vocabularies, plus the conditional invariants (forbidden -> no
  claim lanes + internal_redacted; retiring lifecycle -> non-null
  retirement_metadata with reviewable window note; M1 surface seed
  membership -> non-null failure_drill).

- `tests/governance/m1_capability_inventory_seed_lane/run_m1_capability_inventory_seed_lane.py`
  — unattended runner that replays the seed and emits the durable
  JSON capture.

## Named runtime consumers

- `docs/governance/capability_lifecycle_seed.md` — reviewer-facing
  landing page that quotes the seeded M1 surfaces verbatim so docs /
  help / release copy reads the same lifecycle vocabulary as the
  inventory.
- `docs/governance/capability_inventory_contract.md` — narrative
  contract that names the inventory as the canonical register for
  capability lifecycle and public-claim posture.
- `tools/ci/validate_contract_artifacts.py` — live CI consumer that
  already reads the inventory and fails closed on missing
  capability_ids, public-label / claim-posture mismatches, and
  projection coverage gaps.

## Live runtime consumers (read-only)

- `artifacts/build/build_identity.json` — exact-build identity that
  the capture embeds for cross-artifact traceability.

## Validation captures

- `artifacts/milestones/m1/captures/capability_inventory_seed_validation_capture.json`

## M1 surface coverage

The seed asserts the following M1 surfaces are present as
`m1_surface_seed_membership: true` rows:

| `capability_id` | Lifecycle | Drill |
| --- | --- | --- |
| `shell.frame` | `stable` | lifecycle_state dropped |
| `shell.start_center` | `stable` | kill_switch_path dropped while rollout_gate present |
| `workspace.entry` | `stable` | owner_dri dropped |
| `editor.basics` | `stable` | public_label dropped when policy is required |
| `command.quick_open` | `beta` | rollout-gate public-disclosure relaxed |
| `search.shell_search` | `beta` | lifecycle_state drifted to an unknown token |
| `terminal.embedded_seed` | `preview` | widened to retired without a window note |
| `support.export_seed` | `beta` | export_visibility widened from support-only to public |
| `help.about_pane` | `stable` | lifecycle_state dropped |

## Failure-drill coverage

Nine named drills, all reproducible under
`--force-drill <capability_id>:<drill_id>`:

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

## Refresh

Re-run the validation lane after a change to:

- the seed YAML,
- either schema (envelope or row),
- the reviewer-facing landing page,
- the named runtime consumers, or
- the build-identity record the capture embeds.

## Closure rule

The lane stays open until the latest capture lands under the
governed proof root and every row reports PASS for closed-vocabulary
membership (capability_kind, surface_families, lifecycle_state,
rollout_gate_kind, public_label_policy, public_claim_posture,
export_visibility), the conditional invariants (forbidden ->
empty claim_lanes + internal_redacted export, retiring lifecycle ->
non-null retirement_metadata with reviewable window note, M1 surface
seed membership -> non-null failure_drill from the named drill
vocabulary), the kill-switch rule (rollout_gate non-null -> non-empty
kill_switch_path), the pre-stable disclosure rule
(labs/preview/beta + rollout_gate -> public_disclosure_required =
true), the support-export widening rule (support_capability rows
must keep export_visibility in {support_export_only,
internal_redacted}), the lifecycle coverage rule (all of
labs/preview/beta/stable/deprecated seeded), the M1 surface
coverage rule, named-runtime-consumer existence, and its nine
named failure drills.
