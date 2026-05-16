# Workset switcher and portable workset artifact beta

Beta layer on top of the alpha `workset_artifact_record` and the alpha
workset switcher / scope banner contracts. The alpha layer froze the
durable scope artifact and the per-surface chip / banner / switcher row
projections. This beta layer adds the cross-surface truth the M3 spec calls
out:

- one closed [`WorksetPortabilityLabel`](#1-portability-label) on every
  switcher row;
- a typed [activation preview](#2-activation-preview) before applying a
  candidate row;
- a [reopen-parity packet](#3-reopen-parity-packet) so local, remote, and
  headless reopen paths preserve the same `stable_scope_id` with explicit
  downgrade reasons.

The machine-readable schema lives at:

- [`/schemas/workspace/workset_switcher_beta.schema.json`](../../../schemas/workspace/workset_switcher_beta.schema.json)

The canonical fixtures live under:

- [`/fixtures/workspace/m3/workset_switcher/`](../../../fixtures/workspace/m3/workset_switcher/)

The Rust types are exported from `aureline_workspace::workset_switcher`. The
integration test
[`crates/aureline-workspace/tests/workset_switcher_beta.rs`](../../../crates/aureline-workspace/tests/workset_switcher_beta.rs)
replays every fixture and proves the closed acceptance states.

## 1 Portability label

Every switcher row carries one closed `portability_label` derived from the
artifact's `portability_class`, `source_class`, and optional policy overlay:

| Label                        | Meaning                                                                |
| ---------------------------- | ---------------------------------------------------------------------- |
| `portable`                   | Identity + member refs round-trip cleanly across export/import.        |
| `portable_with_rebinding`    | Identity survives; member refs rebind on the new host.                 |
| `local_only`                 | Refs touch machine-local paths or session-only state.                  |
| `policy_limited`             | A policy overlay narrows the view; portability is conditional.        |
| `managed_provider_locked`    | Owned by a managed provider; cannot leave the provider boundary.      |

Frozen rules:

1. `managed_provider_locked` rows MUST NOT offer `export_workset_artifact`.
2. `local_only` rows whose source class is `ephemeral_session` MUST NOT
   offer `export_workset_artifact`.
3. `policy_limited` rows MUST carry a `policy_overlay` block. Admin-policy
   and license/export-control narrowing causes MUST keep
   `hidden_member_list_visible = false`.
4. Every inactive row MUST offer `preview_activation_diff` so opening a
   workset goes through a typed preview rather than a silent activation.

## 2 Activation preview

When a candidate row is selected, the chrome projects a typed
`workset_activation_preview` against the active artifact + candidate
artifact. The preview carries:

- `same_identity` — true when the candidate is the active artifact.
- `scope_drift` — one closed token (`same_identity`, `widens`, `narrows`,
  `mixed`, `portability_or_readiness_only`, `presentation_only`).
- `root_additions` / `root_removals` — typed root-taxonomy entries.
- `changes_portability` / `changes_readiness` — posture deltas.
- `base_portability_label` / `candidate_portability_label` and
  `base_readiness` / `candidate_readiness` for the header banner.
- an embedded `scope_widen_diff_record` (the alpha contract) when the
  candidate has a different identity, so the existing scope-diff review
  sheet reads one structured diff.

Frozen rules:

1. `same_identity = true` forbids `root_additions`, `root_removals`,
   `changes_portability`, and `changes_readiness`.
2. `scope_drift = widens` requires at least one `root_addition`.
3. `scope_drift = narrows` requires at least one `root_removal`.
4. `scope_drift = mixed` requires at least one of each.
5. `scope_drift = portability_or_readiness_only` is the only class that
   allows posture changes without root drift.

## 3 Reopen-parity packet

The reopen-parity packet bundles `workset_scope_consumer_binding` records
for one workset across the default consumer classes:

- `local_ui`
- `remote_ui`
- `headless`

Each binding carries `reopen_state` (`exact` or `degraded`) and, when
degraded, a typed `degraded_reason` from the alpha vocabulary. The packet
also exposes:

- `identity_preserved_across_consumers` — true when every binding quotes
  the same `workset_ref` and `stable_scope_id`;
- `exact_consumer_classes` — the consumers that reopened the saved scope
  exactly;
- `degraded` — one row per degraded consumer with a typed reason and a
  redaction-aware note.

Frozen rules:

1. Bindings MUST include `local_ui`, `remote_ui`, and `headless`. Surfaces
   may attach additional consumer bindings (`support_export`,
   `navigation`, `refactor_scope`); the packet still rejects duplicates.
2. Every binding MUST quote the same `workset_ref` and `stable_scope_id`.
3. A `managed_provider_locked` workset MUST NOT include a `support_export`
   binding in the packet — exporting is blocked at the portability layer.
4. Degraded entries MUST match the binding's `degraded_reason`. An exact
   binding MUST NOT appear in the degraded list and MUST NOT carry a
   `degraded_reason` on the binding.

## 4 Support-export bundle

`workset_switcher_beta_support_export` packages one switcher record, every
activation preview the chrome considered, and every reopen-parity packet
for the same workspace. Triage replays the bundle against the same set of
artifacts; the bundle itself never has to re-derive scope from a side
channel.

## 5 First consumer

The first shell consumer is
`aureline_shell::workset_switcher::beta::render_switcher_beta_lines` and
`render_reopen_parity_lines` — deterministic plaintext renderers used by
the support packet writer and the headless workset-switcher CLI. Both
quote the workspace types verbatim; neither renderer invents a parallel
portability vocabulary or downgrades a degraded binding to an unreasoned
state.
