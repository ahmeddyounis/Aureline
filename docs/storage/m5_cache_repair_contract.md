# Cache-repair plan contract (M5 heavy artifact families)

The cache-repair plan is the operator-facing object the shell shows when a
derived cache or semantic index is detected corrupt or stale. It replaces vague
"clear everything" or factory-reset advice with a targeted, inspectable repair
that states:

- **which storage class is affected** and the **single-class scope** the repair
  is bounded to (one class in one workspace, or one class across the workspaces
  that share it) — never a global reset;
- **what the detected fault is** — a corrupt index, a checksum mismatch, a torn
  write, schema-version drift, staleness against source, a missing backing
  object, or orphaned entries;
- **the targeted repair action** — rebuild one index from source, refetch one
  pack by digest, revalidate against source, re-derive one cache on demand,
  quarantine-then-rebuild, quarantine-then-manual-review, or repair recovery
  state in place from its checkpoint;
- **the quarantine disposition** — whether the suspect copy is preserved because
  it still holds user-owned data or forensic value, and the quarantine ref that
  proves it was preserved *before* any clear;
- **the propagated stale / rebuild-needed / corrupt labels** every affected
  surface shows until the repair actually completes;
- **the no-reset-everything fallback** offered when the targeted repair fails —
  always a narrower-or-equal action, never a delete-all.

The canonical product object is `m5_cache_repair_plan`, owned by
`crates/aureline-support/src/m5_cache_repair` and bound to the boundary schema at
`schemas/storage/m5_cache_repair.schema.json`. It mints no new storage primitive:
the storage-class and storage-posture vocabularies re-export verbatim from
`artifacts/runtime/storage_classes.yaml`. The composer `compose_plan` folds the
canonical runtime storage-class profiles, so the plan and the storage-governance
matrix can never disagree about which classes are protected or require
export-before-delete.

## Invariants

A plan is admissible only when it holds every invariant below; the validator in
`m5_cache_repair` and the schema both enforce them, and the scenario corpus under
`fixtures/storage/m5_cache_repair_cases/` exercises them.

1. **The repair is targeted, never reset-everything.** `factory_reset_offered` is
   always `false`, `reset_everything_avoided` and `narrowest_sufficient_scope` are
   always `true`, the scope is one storage class, and `repair_action` is always a
   single-class action. There is no global / factory-reset value in any
   vocabulary.
2. **Scope names its workspace honestly.** A `single_class_single_workspace` plan
   names a `workspace_ref`; a `single_class_all_workspaces` plan names none.
3. **Suspect copies are quarantined before any clear.** A quarantine copy is
   preserved (`quarantine_disposition` is one of the quarantined dispositions and
   `quarantine_ref` is present) exactly when the suspect data still holds
   user-owned data or forensic value. Evidence and user-owned recovery classes
   are never `no_quarantine_disposable_only`; a repair action that clears suspect
   data never runs before a required quarantine copy exists.
4. **Protected classes preserve what they own.** A `user_owned_recovery_state`
   repair sets `preserves_user_owned_data = true` and repairs in place from a
   checkpoint; an `evidence_support_cache` repair sets
   `preserves_forensics_value = true` and quarantines for class-specific review.
   Neither is auto-rebuilt from a derived source, and neither is cleared.
5. **Stale labels propagate until repair completes.** Every affected surface
   carries the plan's `repair_label` and its derived `posture`, with
   `clears_on_repair_complete = true`. A surface label stays `label_active` until
   `repair_state` reaches `repair_complete_healthy`; on completion every label
   clears and `repair_label` is `healthy`. `repair_label` is `healthy` exactly
   when `repair_state` is complete.
6. **A failed repair offers a narrower fallback, not a reset.** A fallback is
   offered exactly when `repair_state` is `repair_failed_fallback_offered`, and
   it is always a narrower-or-equal action (retry, widen-to-workspace under
   review, open-without-cache, or class-specific review) — never reset-everything.
7. **The plan offers the inspector and the targeted repair.** Every plan carries
   `open_inspector_action_ref = action.storage.open_inspector` and
   `run_targeted_repair_action_ref = action.storage.run_targeted_repair`, so a
   corruption is never a dead end and never a factory-reset button.

## Support export

`CacheRepairPlanCorpus::support_export` projects the corpus into a metadata-safe
envelope (`m5_cache_repair_support_export`) the support-bundle pipeline quotes
without leaking raw payloads, paths, or credentials. It counts plans, repairs in
progress, failed repairs, quarantine-preserved repairs, and — always zero —
factory-reset offers. The checked-in golden lives at
`fixtures/storage/m5_cache_repair/support_export.golden.json` and is regenerated
with
`cargo run -p aureline-support --example dump_m5_cache_repair_support_export`.
