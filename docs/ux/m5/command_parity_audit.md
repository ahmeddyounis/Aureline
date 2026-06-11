# M5 command-parity and discoverability audit (companion doc)

This page is the companion to the M5 command-parity and discoverability
audit. It carries the stable v1 shell promise forward into the M5 depth
lanes: every meaningful action in the notebook, data/API, profiler,
trace/replay, docs/browser, template/scaffold, review/pipeline, preview,
companion, incident, sync, and offboarding surfaces must be
discoverable, explainable, and automation-honest through the same
command graph, help anchors, and disabled-state reasoning model as the
v1 core — never only through a pane-local icon or a browser deep link.

The audit data, the per-row blocking findings, the per-channel coverage
numbers, and the help-anchor index all come from one mint-from-truth
path — the seeded audit in
[`crate::m5_command_registry`](../../../crates/aureline-shell/src/m5_command_registry/mod.rs)
— so the live shell discoverability inspector, the CLI/headless
inspector, the support-export wrapper, the XT-12 learnability matrix,
and the CI gate never disagree on what each M5 surface promises.

Authoritative artifacts:

- [`/artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md`](../../../artifacts/ux/m5/discoverability-audits/m5_command_parity_audit.md)
  — the rendered audit generated from the seeded projection.
- [`/fixtures/ux/m5/command-parity/report.json`](../../../fixtures/ux/m5/command-parity/report.json)
  — the JSON snapshot of the same record consumed by every surface.
- [`/fixtures/ux/m5/command-parity/support_export.json`](../../../fixtures/ux/m5/command-parity/support_export.json)
  — the support-export wrapper a reviewer pivots on.
- [`/schemas/commands/m5-command-descriptor-diff.schema.json`](../../../schemas/commands/m5-command-descriptor-diff.schema.json)
  — the boundary schema the fixtures conform to.

## The six required discoverability channels

The audit covers exactly the six channels every M5 command must remain
discoverable through:

| Channel | Description |
| ------- | ----------- |
| `command_palette` | Palette result rows. |
| `keybinding_help` | Keybinding help, shortcut help, and conflict resolution. |
| `help_search` | Help search and the help-anchor index. |
| `onboarding_tour` | Onboarding and guided-tour references. |
| `cli_headless` | CLI / headless help and dispatch rows. |
| `ai_automation` | AI automation surfaces invoked by stable command identity. |

For every registered M5 command, each channel carries a projection. The
projection is one of:

- `claimed` — channel owns a first-class projection of the command and
  quotes the canonical descriptor verbatim.
- `explicitly_narrowed` — channel narrows away from the descriptor but
  names a `narrowing_reason` (e.g. `approval_required_network_publish`,
  `keybinding_unassigned_at_beta`, `draft_only_human_runs_cells`).
- `discoverable_only` — channel only lists the command for
  discoverability; dispatch happens elsewhere.
- `browser_handoff_only` — channel routes to browser handoff only.
- `voice_addressable` — channel is voice-addressable only.
- `not_surfaced_on_this_client` — the client scope excludes this channel.
- `custom_pane_only` — the command is reachable only through its own
  pane. **Always blocking** as a pointer-only island.
- `unknown_high_risk_gap` — channel is missing or unknown. **Always
  blocking** for any high-risk command.

A command is "high-risk" when its descriptor pins a non-trivial preview
class (anything other than `no_preview_required`) or a non-trivial
capability scope (`recoverable_durable_mutation`,
`destructive_bulk_mutation`, or `irreversible_publish`).

## Explainability and automation honesty

Every descriptor pins a `disabled_reason_mode` and an
`automation_suitability` so the palette, help, and automation surfaces
can explain *why* a command is unavailable, preview-only, or gated
rather than silently disappearing it:

- `disabled_reason_mode = typed_reason_required_when_unavailable` means
  the surface MUST show a typed disabled reason when the command cannot
  run. A high-risk command that declares `always_invokable` is a
  `missing_disabled_reason_mode` blocker.
- `automation_suitability` is one of `fully_automatable`,
  `preview_then_confirm`, `draft_only`, or `human_only`. The AI
  automation channel MUST project the same value; a wider projection is
  an `automation_suitability_drift` blocker, so a new M5 action can
  never widen authority compared with the stable v1 command model.

## What the validator rejects

The audit fails the gate when any blocking finding remains:

- `unknown_high_risk_gap`, `pointer_only_affordance` — hidden or
  pane-only islands.
- `command_id_drift`, `label_drift`, `lifecycle_label_drift`,
  `preview_class_drift`, `disabled_reason_drift`,
  `automation_suitability_drift`, `alias_drift` — a claimed channel that
  disagrees with the canonical descriptor.
- `missing_help_anchor`, `descriptor_missing_help_anchor` — a claimed
  channel or the descriptor that cannot point back to the help anchor.
- `missing_search_metadata` — a descriptor with no searchable
  discoverability keywords.
- `command_not_promoted` — a command id not yet promoted into the stable
  command graph.
- `missing_narrowing_reason`, `missing_projection` — a non-claimed row
  with no reason, or a claimed row missing a projected field.

## Consuming the audit

The XT-12 learnability matrix and later release-center, docs/help, and
support-export surfaces ingest the checked-in `report.json` directly
when qualifying or narrowing an M5 row instead of cloning status text.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_command_parity -- validate
cargo test -p aureline-shell --test m5_command_parity_fixtures
python3 tools/ci/m5/command_parity_check.py
```
