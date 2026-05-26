# Finalize Portable-State Export/Import, Remembered-State Inspector, And Restore-Provenance Labeling — proof packet

Reviewer-facing proof packet for the finalized portable-state lane: portable-
state export/import packages, the remembered-state inspector, and the
restore-provenance card composed into one governed, export-safe record per
posture. This packet is the stable-line anchor for this lane: dashboards,
docs, Help/About surfaces, and support exports should ingest the typed
sources below rather than cloning this packet's text.

## Canonical machine sources

- Lineage projection and contract types:
  [`/crates/aureline-workspace/src/portable_state_lineage/`](../../../crates/aureline-workspace/src/portable_state_lineage/)
- Portable-state alpha package (the live input the projection ingests):
  [`/crates/aureline-workspace/src/state_packages/`](../../../crates/aureline-workspace/src/state_packages/)
- Schema:
  [`/schemas/workspace/portable_state_lineage.schema.json`](../../../schemas/workspace/portable_state_lineage.schema.json)
- Headless emitter / CLI:
  [`/crates/aureline-workspace/src/bin/aureline_portable_state_lineage.rs`](../../../crates/aureline-workspace/src/bin/aureline_portable_state_lineage.rs)
- Fixtures:
  [`/fixtures/workspace/m4/portable_state_lineage/`](../../../fixtures/workspace/m4/portable_state_lineage/)
- Replay gate:
  [`/crates/aureline-workspace/tests/portable_state_lineage_replay.rs`](../../../crates/aureline-workspace/tests/portable_state_lineage_replay.rs)
- Companion contract doc:
  [`/docs/workspace/m4/finalize-portable-state-export-import-and-restore-provenance.md`](../../../docs/workspace/m4/finalize-portable-state-export-import-and-restore-provenance.md)
- Typed consumer: `aureline_workspace::project_portable_state_lineage`

## What this packet proves

1. **State-class separation truth.** Each record carries explicit workspace
   authority, window topology, profile defaults, and machine-local hints
   rows with their persistence classification (`local_only`, `portable`,
   `shared`, `machine_local`). Machine-local hints are required to be
   classified machine-local and excluded from carried-body export. A
   missing class or misclassified machine-local hints narrows the record
   below Stable with `state_classes_incomplete` or
   `machine_local_hints_misclassified`. Worked examples:
   [`exact_restore_stable.json`](../../../fixtures/workspace/m4/portable_state_lineage/exact_restore_stable.json),
   [`compatible_restore_stable.json`](../../../fixtures/workspace/m4/portable_state_lineage/compatible_restore_stable.json).

2. **Restore-provenance truth with controlled fidelity classes.** The
   record names one of `exact`, `compatible`, `layout_only`,
   `recovered_drafts`, or `evidence_only` for the whole package, and
   surfaces topology adjustments alongside their visible-bounds proof and
   the preserved pane-id provenance. Display-topology adjustments that
   change displays without verifying visible bounds narrow with
   `topology_adjustment_unverified`; adjustments that lose pane-id
   provenance narrow with `topology_adjustment_lost_pane_ids`. Worked
   examples:
   [`layout_only_stable.json`](../../../fixtures/workspace/m4/portable_state_lineage/layout_only_stable.json),
   [`evidence_only_stable.json`](../../../fixtures/workspace/m4/portable_state_lineage/evidence_only_stable.json).

3. **Exclusions stay visible, not silent.** The redaction manifest is
   required to include `raw_secret_material_excluded`,
   `approval_ticket_excluded`, `delegated_credential_excluded`,
   `live_authority_handle_excluded`, `machine_unique_handle_excluded`, and
   `state_root_excluded`. Machine-local exclusions are required to name
   `contains_live_handle`, `display_hint_best_effort_only`,
   `state_root_only`, and `credential_store_only` so the package never
   silently drops secret-bearing or machine-unique handles. Missing rules
   or reasons narrow with `redaction_rule_missing`,
   `machine_local_exclusions_not_reviewed`, or
   `machine_local_exclusion_reason_missing`.

4. **No silent rerun on restore.** Non-live panes (context-only and
   placeholder-only) must carry `ExplicitUserActionRequired` and
   `PlaceholderPreserved` guardrails so terminals, debuggers, notebook
   kernels, preview servers, and remote sessions never resume silently.
   Missing guardrails narrow with `placeholder_missing_no_rerun_guardrail`.
   The remembered-state actions row is required to cover inspect, export,
   compare, and clear so support and crash-recovery flows can reach all
   four.

5. **Inspection precedes destructive cleanup.** A destructive action
   (apply, clear, export) is always reachable, so the record requires
   the `inspect_inspector`, `inspect_export_review`,
   `inspect_restore_provenance`, `compare_before_apply`,
   `rollback_checkpoint`, `export`, and `clear` hooks to be available
   before they fire. A missing hook narrows with
   `inspection_hook_unavailable`. Worked example:
   [`missing_compare_hook_narrowed.json`](../../../fixtures/workspace/m4/portable_state_lineage/missing_compare_hook_narrowed.json).

6. **Producer attribution is pinnable for replay.** Each record carries
   the producer ref, the schema version, the package creation timestamp,
   and an integrity hash derived from the package identity / class rows.
   Import / replay surfaces can compare the integrity hash against the
   source before applying. Incomplete attribution narrows with
   `producer_attribution_incomplete`.

7. **Lineage and export stay honest.** Every record sets
   `raw_payload_excluded = true` and carries only opaque refs to the
   source package, manifest, workspace, source snapshot, restore
   provenance, and linked profile artifacts. An empty package, manifest,
   or workspace ref narrows with `lineage_export_unsafe`.

8. **The record is replay-gated.** The replay gate re-projects each
   fixture and asserts it equals the checked-in `expected`, so the
   projection cannot drift without failing CI.

## Fixture corpus

| Fixture                              | Posture                                 | Fidelity         | Qualification           | Proves                                                |
| ------------------------------------ | --------------------------------------- | ---------------- | ----------------------- | ----------------------------------------------------- |
| `exact_restore_stable`               | All classes restore exactly             | `exact`          | `stable`                | Exact-restore fidelity, all pillars proven            |
| `compatible_restore_stable`          | DPI re-bucket / schema fallback         | `compatible`     | `stable`                | Compatible fidelity, downgrade disclosed              |
| `layout_only_stable`                 | Window topology only; live placeholders | `layout_only`    | `stable`                | Layout-only restore preserves placeholders            |
| `evidence_only_stable`               | Missing extension + remote + policy     | `evidence_only`  | `stable`                | Evidence-only restore preserves spatial context       |
| `missing_compare_hook_narrowed`      | `compare_before_apply` hook unavailable | `compatible`     | `narrowed_below_stable` | Destructive action with no compare hook narrows       |

## How to verify

```sh
# Unit + replay gate for the portable-state lineage projection.
cargo test -p aureline-workspace --lib portable_state_lineage
cargo test -p aureline-workspace --test portable_state_lineage_replay

# Truth source (portable-state alpha package).
cargo test -p aureline-workspace --test portable_state_alpha

# Headless emitter (JSON or --lines projection).
cargo run -p aureline-workspace --bin aureline_portable_state_lineage -- --lines \
  fixtures/workspace/m4/portable_state_lineage/compatible_restore_stable.json
```

## Stable-line registration

This lane's truth is the checked-in record, schema, fixtures, and replay
gate above. The lineage record self-describes its stable qualification:
surfaces that cannot prove the contract carry
`stable_qualification.level = narrowed_below_stable` with a named reason,
so they never inherit an adjacent green row. No public scope is widened
from this row.
