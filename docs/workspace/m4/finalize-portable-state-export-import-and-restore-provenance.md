# Portable-state lineage — contract

This document describes the portable-state lineage record: the workspace's
governed, export-safe projection that finalizes portable-state
export/import, the remembered-state inspector, and the restore-provenance
card into one record per posture.

The record is the single artifact every consuming surface (workspace
status, export review sheet, restore-provenance card, support export,
Help/About, headless CLI) ingests instead of cloning status text.

## Input

The projection ingests a live
[`PortableStateAlphaPackage`](../../../crates/aureline-workspace/src/state_packages/mod.rs)
verbatim. The state-packages module owns the heavy lifting (state-class
separation, redaction manifest, machine-local exclusions, topology
adjustments, placeholder cards). This module promotes that validated
package into a stable-line lineage artifact that proves the contract on
the captured posture.

For determinism and replay, the projection accepts the same package shape
fixtures and the headless emitter consume.

## What the record proves

The four claims the stable line is anchored on, specialized to portable
state:

- **State-class separation truth.** Workspace authority, window topology,
  profile defaults, and machine-local hints are all present, each
  classified honestly (`local_only`, `portable`, `shared`,
  `machine_local`), and the machine-local class is not exported as
  authority.
- **Restore-provenance truth.** A controlled restore-fidelity class
  (`exact`, `compatible`, `layout_only`, `recovered_drafts`, or
  `evidence_only`) is named for the package, topology adjustments verify
  visible bounds and preserve pane-id provenance, and the placeholder
  summary covers live / context-only / placeholder-only postures.
- **Exclusion / redaction honesty.** The redaction manifest names every
  required rule (raw secret material, approval ticket, delegated
  credential, live authority handle, machine-unique handle, state root),
  the machine-local exclusion catalogue names the required reasons
  (contains live handle, display-hint best-effort only, state-root only,
  credential-store only), and `machine_local_exclusions_reviewed` is true
  before any export commits.
- **No-rerun honesty.** Context-only and placeholder-only panes carry
  `ExplicitUserActionRequired` and `PlaceholderPreserved` guardrails so
  restore never silently reruns a terminal command, task, debugger,
  notebook kernel, preview server, or remote session. Remembered-state
  actions cover inspect / export / compare / clear so support and
  crash-recovery flows can reach all four.

In addition the record carries the package producer ref, schema version,
and an integrity hash so import / replay surfaces can pin the source
producer before applying.

## Restore-fidelity classes

The controlled vocabulary the lineage record uses for the whole package:

| Class                | When it fires                                                                                    |
| -------------------- | ------------------------------------------------------------------------------------------------ |
| `exact`              | Every required portable class restores exactly with no placeholders forced by the package state. |
| `compatible`         | Restore is honest but downgrades classes through a named translation or fallback (DPI re-bucket, schema migration). |
| `layout_only`        | Restore reopens window topology only; live surfaces stay placeholders or context-only.           |
| `recovered_drafts`   | Restore recovers a drafted buffer or local-session context the previous session had not durably saved. |
| `evidence_only`      | Restore can only show evidence; live capability is not available (missing extension, missing remote, policy-blocked surface). |

## Narrow reasons

When a claim cannot be proven on the captured posture the record auto-
narrows below Stable with a named reason. Protective postures (a
package with read-only roots that block save, a partial restore where
the user opted into layout-only on purpose) stay Stable — the contract
working as designed is a pass, not a gap.

| Narrow reason                                | Fires when                                                                          |
| -------------------------------------------- | ----------------------------------------------------------------------------------- |
| `package_validator_failed`                   | The underlying portable-state package failed its own validator                      |
| `state_classes_incomplete`                   | Workspace authority / window topology / profile defaults / machine-local hints missing |
| `machine_local_hints_misclassified`          | Machine-local hints exported as authority or not classified `machine_local`         |
| `redaction_rule_missing`                     | A required redaction rule is missing from the manifest                              |
| `machine_local_exclusions_not_reviewed`      | The exporter did not review machine-local exclusions before export                  |
| `machine_local_exclusion_reason_missing`     | A required exclusion reason (live handle / display hint / state root / credential store) is absent |
| `topology_adjustment_unverified`             | A display-topology adjustment changed displays without verifying visible bounds     |
| `topology_adjustment_lost_pane_ids`          | A display-topology adjustment lost pane-id provenance                               |
| `placeholder_missing_no_rerun_guardrail`     | A non-live pane is missing the explicit-user-action / placeholder-preserved guardrails |
| `inspection_hook_unavailable`                | A required pre-destructive inspection hook is unavailable                           |
| `producer_attribution_incomplete`            | The producer ref / schema version / created-at are not pinned                       |
| `lineage_export_unsafe`                      | Package, manifest, or workspace ref is empty                                        |

## Inspection hooks

A portable-state posture must always let the user inspect remembered
state, inspect the export review, inspect the restore-provenance card,
compare current state with the package before applying, capture a
rollback checkpoint before applying, export the record, and clear
remembered state selectively. The default hook set has all seven hooks
available; fixtures may model a degraded subset to prove the corresponding
narrow reason.

| Hook class                     | Action id                                       | Purpose                                                 |
| ------------------------------ | ----------------------------------------------- | ------------------------------------------------------- |
| `inspect_inspector`            | `portable_state.inspect_inspector`              | Open the remembered-state inspector                     |
| `inspect_export_review`        | `portable_state.inspect_export_review`          | Open the export review sheet (portable/local-only/shared) |
| `inspect_restore_provenance`   | `portable_state.inspect_restore_provenance`     | Open the restore-provenance card with fidelity class    |
| `compare_before_apply`         | `portable_state.compare_before_apply`           | Produce a structured diff before applying               |
| `rollback_checkpoint`          | `portable_state.rollback_checkpoint`            | Capture a one-step rollback checkpoint before applying  |
| `export`                       | `portable_state.export`                         | Export the record for support without raw payload bytes |
| `clear`                        | `portable_state.clear`                          | Clear remembered state selectively                      |

## Consumer surfaces

The same projection is consumed by:

- The workspace portable-state status surface (remembered-state inspector
  and restore-provenance card).
- The headless CLI emitter
  (`crates/aureline-workspace/src/bin/aureline_portable_state_lineage.rs`).
- Help/About and support export (via `portable_state_lineage_lines`).
- The replay gate
  (`crates/aureline-workspace/tests/portable_state_lineage_replay.rs`),
  which re-projects every fixture and asserts equality.

The shared human-readable projection (`portable_state_lineage_lines`) is
the canonical text every surface quotes — none of them re-render their
own status text from the underlying fields.

## Schema and stability

The boundary schema is
[`schemas/workspace/portable_state_lineage.schema.json`](../../../schemas/workspace/portable_state_lineage.schema.json).
The record's `portable_state_lineage_schema_version` is currently `1`
and is owned by the workspace crate; any change to the projection must
update the schema, fixtures, and replay gate together.
