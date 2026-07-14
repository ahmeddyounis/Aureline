# M5 channel-isolation, precedence-review, and rollback-target registries

This lane is the side-by-side coexistence-execution lane over the frozen
[M5 install-topology matrix](./m5_install_topology_contract.md) and its
[install-topology and state-root registries](./m5_install_topology_and_state_root_registries.md). It
makes *side-by-side channel isolation* a contract instead of a set of installer accidents: it resolves
every claimed side-by-side profile's stable, preview, beta, and LTS channel to one inspectable object,
isolates the channel root and mutable-state namespaces (channel root, state namespace, secrets namespace,
services namespace), keeps a preview or beta channel from ever reusing the stable durable-state namespace
without an explicit governed handoff, publishes the file-association / protocol-handler / deep-link /
default-open precedence rule — owner channel, precedence rank, conflict resolution, and inspectable
before/after — and binds every rollback target to the full artifact graph (primary executable, sidecars,
symbols, manifests, update metadata) rather than the primary executable alone. Installer, update,
diagnostics, admin, docs, and support surfaces resolve one canonical coexistence truth instead of a
per-surface, last-writer-wins assumption.

- **Canonical Rust module:**
  `crates/aureline-ui/src/m5_channel_isolation_precedence_review_and_rollback_targets` (the
  authoritative validator).
- **Schema:**
  `schemas/install/m5-channel-isolation-precedence-review-and-rollback-targets.schema.json`.
- **Upstream contracts:** rows point back at the frozen
  [`schemas/install/m5-install-topology-matrix.schema.json`](../../schemas/install/m5-install-topology-matrix.schema.json),
  the [`schemas/install/m5-install-topology.schema.json`](../../schemas/install/m5-install-topology.schema.json)
  side-by-side install-topology grammar, and the
  [`schemas/install/m5-install-topology-and-state-root-registries.schema.json`](../../schemas/install/m5-install-topology-and-state-root-registries.schema.json)
  implement lane as their canonical delivery-topology source.
- **Checked proof:**
  `artifacts/release/m5-channel-isolation-precedence-review-and-rollback-targets-proof/`
  (`support_export.json`, `matrix.csv`, `summary.md`).
- **Narrowed fixtures:**
  `fixtures/install/m5-channel-isolation-precedence-review-and-rollback-targets/`
  (`side_by_side_channel_beta_narrowed.json`, `offline_airgap_bundle_preview_narrowed.json`).

## Two registries

1. **Channel isolation** (`resolve_channel_isolation_entry`) — publishes one inspectable side-by-side channel
   per profile: a supported stable / preview / beta / lts channel, a channel root, a state-namespace root, a
   secrets-namespace root, the complete isolation inventory (channel root, state namespace, secrets namespace,
   services namespace), and a disclosed isolated-versus-governed-handoff containment. A clean entry names a
   canonical registry token, covers the canonical / accessible / audit presentation forms, stays isolated with
   no stable-namespace reuse, and inventories every field. Otherwise it degrades honestly — a preview / beta
   channel reusing the stable durable-state namespace degrades to `preview_corrupted_stable_durable_state`, and
   an ambiguous containment degrades so a coexisting channel can never corrupt this channel's durable state.
   `channel_state_is_isolated` is the guardrail that rejects an unsupported channel, an incomplete inventory, or
   any stable-namespace reuse.
2. **Association precedence and rollback** (`resolve_precedence_and_rollback_entry`) — keeps the
   file-association / protocol-handler / deep-link / default-open precedence rule inspectable and the rollback
   target bound to the full artifact graph. A clean entry names a classified precedence domain, discloses the
   owner channel, precedence rank, conflict resolution, rollback artifact graph, and inspectable-before-and-after
   field, and binds the full rollback artifact graph; a precedence rule that hides a disclosure field degrades to
   `handler_precedence_not_inspectable`, and a rollback target narrowed to the primary executable while its
   artifact-graph continuity is undocumented degrades to `rollback_artifact_graph_incomplete`.

## Channel-isolation reference

The channel entry carries its channel, channel root, state-namespace root, secrets-namespace root, and a
disclosed containment, so the registry — never a hand-copied per-profile assumption — is the single source of
truth. `render_channel_isolation_table()` renders exactly this, and only clean, isolated channels appear.

| profile_id | channel | channel_root | state_namespace_root | secrets_namespace_root | containment |
| --- | --- | --- | --- | --- | --- |
| `profile.side_by_side_stable` | stable | `%LOCALAPPDATA%\Aureline\Stable` | `%LOCALAPPDATA%\Aureline\Stable\state` | `%LOCALAPPDATA%\Aureline\Stable\secrets` | isolated |
| `profile.side_by_side_lts` | lts | `%LOCALAPPDATA%\Aureline\LTS` | `%LOCALAPPDATA%\Aureline\LTS\state` | `%LOCALAPPDATA%\Aureline\LTS\secrets` | isolated |

A preview / beta channel reusing the stable durable-state namespace degrades to
`preview_corrupted_stable_durable_state`, an incomplete isolation inventory degrades, and an ambiguous
containment degrades, so a namespace reuse, an incomplete inventory, or an ambiguous containment can never turn
release evidence green.

## Acceptance criteria (proven by resolved examples)

- **Preview and stable installs coexist without corrupting one another's durable state unless the user chooses a
  governed handoff.** Clean channel entries cover the stable, preview, beta, and lts channels across the
  installer / update / diagnostics / admin / support surfaces with a complete isolation inventory, an
  inventory-incomplete example degrades, and no clean channel published an incomplete inventory
  (`channel_isolation_contract_not_proven` otherwise).
- **Handler ownership and channel precedence are inspectable before and after update / import flows.** A
  containment-ambiguous example degrades, at least one clean contained channel entry is present, no clean
  channel entry is ambiguous, and clean precedence entries cover the canonical precedence domains
  (`handler_precedence_inspectability_not_proven` otherwise).
- **Rollback validation fails when a target lacks compatible sidecars, metadata, or association state.** A
  namespace-reuse example degrades, no clean channel reused the stable namespace, a precedence-not-inspectable
  example degrades, and a rollback-artifact-graph-incomplete example degrades
  (`rollback_artifact_graph_completeness_not_proven` otherwise).

## Regeneration

```text
cargo run -p aureline-ui --example dump_m5_channel_isolation_precedence_review_and_rollback_targets -- support-export
cargo run -p aureline-ui --example dump_m5_channel_isolation_precedence_review_and_rollback_targets -- csv
cargo run -p aureline-ui --example dump_m5_channel_isolation_precedence_review_and_rollback_targets -- report
cargo run -p aureline-ui --example dump_m5_channel_isolation_precedence_review_and_rollback_targets -- isolation-table
cargo run -p aureline-ui --example dump_m5_channel_isolation_precedence_review_and_rollback_targets -- fixture-side-by-side-channel-beta-narrowed
cargo run -p aureline-ui --example dump_m5_channel_isolation_precedence_review_and_rollback_targets -- fixture-offline-airgap-bundle-preview-narrowed
```
