# Capability lifecycle seed

This page is the reviewer-facing entry point for the capability
inventory's M1 surface coverage. It quotes the seeded surfaces verbatim
so docs, help, support exports, and release copy all read the same
lifecycle vocabulary as the inventory artifact.

The inventory exists so capability state is first-class across in-
product UI, docs / help / About, CLI / headless help, support and
release exports, compatibility reports, and release notes — never as
ambiguous availability prose. Every row carries an explicit
`lifecycle_state`, owner DRI, customer-visible label policy, kill-switch
path, rollout-state disclosure, and (when retired) retirement
metadata. A row is the truth source for the surface; copy that
disagrees with the row is wrong.

Reviewers should read this page top-to-bottom and then open the seed at
[`/artifacts/governance/capability_inventory_seed.yaml`](../../artifacts/governance/capability_inventory_seed.yaml)
for the full row vocabulary.

## Canonical sources

- [`/artifacts/governance/capability_inventory_seed.yaml`](../../artifacts/governance/capability_inventory_seed.yaml)
  — canonical inventory rows (entries + envelope).
- [`/schemas/governance/capability_inventory.schema.json`](../../schemas/governance/capability_inventory.schema.json)
  — envelope schema (vocabularies, required coverage, named consumers,
  required M1 surface coverage list).
- [`/schemas/governance/capability_inventory_entry.schema.json`](../../schemas/governance/capability_inventory_entry.schema.json)
  — row schema (lifecycle state, owner, kill-switch path, retirement
  metadata, rollout gate, public-label policy, public-claim posture,
  export visibility, named failure drill).
- [`/docs/governance/capability_inventory_contract.md`](capability_inventory_contract.md)
  — narrative contract that names the inventory as the canonical
  register.

## Lifecycle vocabulary

The seed pins the canonical lifecycle vocabulary and the M1-required
coverage subset (Labs / Preview / Beta / Stable / Deprecated must each
appear at least once across the entries).

| State | When to use |
| --- | --- |
| `labs` | Internal-only experimentation; never publicly claimable. |
| `preview` | User-visible but explicitly preview; surfaces MUST disclose the gate. |
| `beta` | Feature-complete behind a beta gate; surfaces MUST disclose the gate. |
| `stable` | Default-on, contract-stable; widely claimable. |
| `lts_facing` | Long-lived stable surface that LTS exports can claim. |
| `deprecated` | Announced for retirement; carries `retirement_metadata` with a reviewable target window. |
| `disabled_by_policy` | Disabled in this build / channel; carries `retirement_metadata` so operators see the exit plan. |
| `retired` | Removed; row stays for traceability with `retirement_metadata` recording the cutover. |

## M1-bearing surfaces

The seed marks the M1-bearing surfaces with
`m1_surface_seed_membership: true`. Each row carries an opaque kill-
switch path (when the row also declares a non-null `rollout_gate`),
retirement metadata (non-null when the row is in a retiring lifecycle
state), and a named failure drill that the validation lane reproduces.

| `capability_id` | Customer-visible label | Lifecycle | Owner | Kill-switch path | Failure drill |
| --- | --- | --- | --- | --- | --- |
| `shell.frame` | Shell frame | `stable` | `@ahmeddyounis` | _structural — non-killable_ | `capability_inventory_drill.shell_frame_lifecycle_state_dropped` |
| `shell.start_center` | Start Center | `stable` | `@ahmeddyounis` | `policy:shell.start_center.kill_switch` | `capability_inventory_drill.start_center_kill_switch_path_dropped` |
| `workspace.entry` | Workspace entry | `stable` | `@ahmeddyounis` | _structural — non-killable_ | `capability_inventory_drill.workspace_entry_owner_dri_dropped` |
| `editor.basics` | Editor basics | `stable` | `@ahmeddyounis` | _structural — non-killable_ | `capability_inventory_drill.editor_basics_public_label_dropped` |
| `command.quick_open` | Quick open | `beta` | `@ahmeddyounis` | `policy:command.quick_open.kill_switch` | `capability_inventory_drill.quick_open_rollout_gate_disclosure_dropped` |
| `search.shell_search` | Search (shell) | `beta` | `@ahmeddyounis` | `policy:search.shell_search.kill_switch` | `capability_inventory_drill.search_shell_lifecycle_state_drifted_to_unknown_token` |
| `terminal.embedded_seed` | Embedded terminal (preview) | `preview` | `@ahmeddyounis` | `policy:terminal.embedded_seed.kill_switch` | `capability_inventory_drill.terminal_seed_widened_to_retired_without_window_note` |
| `support.export_seed` | Support export (seed) | `beta` | `@ahmeddyounis` | `policy:support.export_seed.kill_switch` | `capability_inventory_drill.support_export_export_visibility_widened_to_public` |
| `help.about_pane` | Help / About | `stable` | `@ahmeddyounis` | _structural — non-killable_ | `capability_inventory_drill.help_about_lifecycle_state_dropped` |

The lane requires every row in
`required_m1_surface_coverage` to be present with
`m1_surface_seed_membership: true`. Adding or retiring a surface
requires editing the seed and the runner together so the inventory
cannot quietly drop a protected surface.

## How lifecycle state changes are reviewed and propagated

1. **Propose the change in the inventory**, not in copy. Any change to
   `lifecycle_state`, `kill_switch_path`, `retirement_metadata`,
   `rollout_gate`, `public_label`, `public_label_policy`,
   `public_claim_posture`, or `export_visibility` lands in the seed row
   first.
2. **Re-run the validation lane.** The lane fails closed on missing
   lifecycle metadata, lifecycle widening without disclosure,
   support-export widening, retirement-without-window-note, and
   forbidden / claim-posture mismatches.
3. **Propagate to copy after the lane passes.** Docs, help, About,
   support exports, and release notes consume the row's
   customer-visible label, lifecycle posture, and kill-switch path
   verbatim. Surfaces that disagree with the row are non-conforming.
4. **Retire by editing, not deleting.** Move the row's
   `lifecycle_state` to `deprecated` (or `retired`) and fill
   `retirement_metadata.retirement_target_window_note`; do not delete
   the row.

## Validation lane

- Runner: [`tests/governance/m1_capability_inventory_seed_lane/run_m1_capability_inventory_seed_lane.py`](../../tests/governance/m1_capability_inventory_seed_lane/run_m1_capability_inventory_seed_lane.py)
- Latest capture:
  [`artifacts/milestones/m1/captures/capability_inventory_seed_validation_capture.json`](../../artifacts/milestones/m1/captures/capability_inventory_seed_validation_capture.json)
- Owning packet:
  [`artifacts/milestones/m1/proof_packets/capability_lifecycle_seed.md`](../../artifacts/milestones/m1/proof_packets/capability_lifecycle_seed.md)

Run the lane:

```bash
python3 tests/governance/m1_capability_inventory_seed_lane/run_m1_capability_inventory_seed_lane.py --repo-root .
```

Force a named failure drill:

```bash
python3 tests/governance/m1_capability_inventory_seed_lane/run_m1_capability_inventory_seed_lane.py \
    --repo-root . \
    --force-drill <capability_id>:<drill_id>
```

Under `--force-drill` the runner exits 0 only when the row's declared
`expected_check_id` is reproduced from the forced input. Drift on the
unforced rows still fails the lane.
