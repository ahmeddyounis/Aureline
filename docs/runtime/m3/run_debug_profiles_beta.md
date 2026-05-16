# Beta Run / Debug Launch and Attach Profiles

This document is the reviewer-facing landing page for the beta launch and
attach profile model that backs the run, test, and debug surfaces. The
machine-readable boundary lives at
[`/schemas/runtime/launch_profile.schema.json`](../../../schemas/runtime/launch_profile.schema.json).
The runtime implementation lives at
[`/crates/aureline-runtime/src/launch_profiles/`](../../../crates/aureline-runtime/src/launch_profiles/).

The beta promise:

- one typed launch / attach profile object covering task, test, and debug lanes;
- every profile edit produces a new immutable revision so edits are durable, diffable, and reversible;
- a preview against the freshly resolved [`ExecutionContext`](../execution_context_seed.md) discloses target, environment, adapter, and side-effect truth before any dispatch;
- runtime, UI, and support exports project the same record so they cannot disagree on the selected profile or the resolved execution context;
- no silent toolchain fallback and no hidden remote parity claim: the profile snapshot, the disclosure rows, and the support export use the same closed vocabulary.

## Profile shape

Every profile is a [`launch_profile_record`](../../../schemas/runtime/launch_profile.schema.json#L141)
carrying:

- **identity** — `profile_id`, `workspace_id`, `display_name`;
- **posture** — `mode` (`launch` or `attach`) and `kind` (`task`, `test`, or `debug`);
- **target binding** — canonical target id, target-class token, label, optional working directory, and the expected workset-scope token;
- **adapter binding** — adapter id, transport class, requested DAP protocol version, and required capability tokens (required for `debug`-kind profiles);
- **environment binding** — capsule id, capsule hash, and declared overlay keys (no values, so support exports never leak secrets);
- **arguments** — program, args, working-directory override, attach process id;
- **declared side effects** — closed vocabulary covering target process spawn / attach, workspace filesystem writes, outbound and inbound network, process-env mutation, and remote-host handoff;
- **lineage** — `revision_id`, `parent_revision_id`, `last_edited_at`.

## Durable, diffable, reversible edits

Profiles are never mutated in place. Every edit creates a new
[`launch_profile_revision_record`](../../../schemas/runtime/launch_profile.schema.json#L218)
whose snapshot embeds the profile body at that point in time and whose
`parent_revision_id` points at the previous revision. The accompanying
[`launch_profile_edit_record`](../../../schemas/runtime/launch_profile.schema.json#L189)
carries a closed `edit_class`:

| Class | Meaning |
| --- | --- |
| `created` | Profile was first registered |
| `renamed_display_name` | Display name changed |
| `mode_changed` | Switched between launch and attach |
| `target_binding_changed` | Canonical target binding changed |
| `adapter_binding_changed` | Adapter binding added, replaced, or cleared |
| `environment_binding_changed` | Capsule binding changed |
| `arguments_changed` | Program / args / cwd / attach pid changed |
| `side_effects_changed` | Declared side-effect set changed |
| `rolled_back` | Profile was reverted to an earlier revision |

Rollback is itself an edit: the store creates a new revision whose snapshot
matches the target revision and whose
`rollback_target_revision_id` quotes the revision the rollback resolved to.
Prior working revisions are never overwritten, so a rollback after a bad
edit cannot corrupt the earlier state.

## Preview before execution

Surfaces call `LaunchProfileStore::preview` with a freshly resolved
[`ExecutionContext`](../execution_context_seed.md). The returned
[`launch_profile_preview_record`](../../../schemas/runtime/launch_profile.schema.json#L257)
projects three disclosure groups (target, environment, adapter), the
declared side-effect tokens, the resolved execution-context id, and the
preview state:

| State | Effect |
| --- | --- |
| `ready_to_dispatch` | Stored binding matches the resolved context |
| `drift_requires_review` | At least one disclosure row drifted; a visible review is required |
| `target_unreachable` | The resolved current target is unreachable |
| `unavailable_invalid_config` | Stored snapshot is missing required data (`missing_target_binding`, `missing_adapter_binding`, `attach_missing_process_id`, `launch_missing_program`) |

When the state is anything other than `ready_to_dispatch`, the preview
sets `requires_review_before_dispatch` and the `honesty_marker_present`
flag so shell, status, and support surfaces can render an honest cue.

## Runtime / UI / support agreement

The same preview record is consumed by:

- the **runtime** before it mints the next launch (the preview's
  `execution_context_ref` is the exact context the dispatch will use);
- the **shell** to render the row in the run-and-debug picker (see the
  shell consumer wired through
  [`crates/aureline-shell/src/run_debug_profiles_beta`](../../../crates/aureline-shell/src/run_debug_profiles_beta/));
- the **support export** through
  [`launch_profile_support_export_record`](../../../schemas/runtime/launch_profile.schema.json#L348)
  so reviewer evidence quotes the same revision, the same disclosure rows,
  and the same side-effect tokens that the user saw at dispatch time.

## Failure-drill fixtures

Reviewer fixtures live under
[`/fixtures/runtime/launch_profiles_beta/`](../../../fixtures/runtime/launch_profiles_beta/)
and exercise three named drills:

- `protected_walk_local.json` — create a debug-launch profile against the
  local desktop, preview against the matching execution context, and
  observe `ready_to_dispatch` with no drift and no honesty marker;
- `edit_and_rollback.json` — rename the profile, then roll back to the
  original revision; the lineage must retain both edits and the original
  snapshot;
- `current_context_target_drift.json` — preview the stored binding against
  a current context whose target class and working directory differ from
  the stored binding; the preview must move to
  `drift_requires_review` and disclose the changed rows.

The integration test that replays these fixtures lives at
[`crates/aureline-runtime/tests/launch_profiles_beta.rs`](../../../crates/aureline-runtime/tests/launch_profiles_beta.rs).

## Out of scope for this beta

- Full M5 notebook-kernel launch and attach depth.
- Collaboration / multi-user profile control productization.
- Launch-language breadth outside the claimed beta wedges.
- Cross-workspace profile import (revisions are workspace-scoped in this
  beta).
