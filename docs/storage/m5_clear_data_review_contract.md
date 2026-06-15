# Clear-data review sheet contract (M5 heavy artifact families)

The clear-data review sheet is the operator-facing object the shell shows
**before** any cleanup commits. It replaces a generic "clear cache" or "reset"
button with a class-selective review that, per family and per workspace, names:

- **what is selected for cleanup** and how much disk it reclaims;
- **what is retained or protected** and therefore excluded;
- **what a rebuild would cost** (rebuild-cost class, offline risk, and a plain
  rebuild disclosure that is never hidden in logs);
- **whether an export-before-delete path exists first**;
- **which removals are irreversible**.

It covers three flows — **user-driven cleanup**, **admin-driven cleanup**, and
**offboarding/reset** — and three trigger families — manual requests, low-disk
pressure, and managed quota pressure.

The canonical product object is `m5_clear_data_review_sheet`, owned by
`crates/aureline-support/src/m5_clear_data_review` and bound to the boundary
schema at `schemas/storage/m5_clear_data_review.schema.json`. It mints no new
storage primitive: the storage-class, authority, rebuild-cost,
clear-protection, low-disk-ladder, pin-source, and clear-data-action
vocabularies re-export verbatim from the frozen artifact-family matrix at
`artifacts/storage/m5_artifact_family_storage_matrix.yaml`, and the
rebuild-safety/offline-risk vocabularies re-export from the storage inspector.

## Invariants

A sheet is admissible only when it holds every invariant below; the validator
in `m5_clear_data_review` and the schema both enforce them, and the scenario
corpus under `fixtures/storage/m5_clear_data_review_cases/` exercises them.

1. **No generic clear of a protected class.** Durable evidence
   (`evidence_support_cache`) and user-owned recovery state
   (`user_owned_recovery_state`) never offer `generic_clear_in_bulk`,
   `generic_clear_excluding_pins`, or `class_selective_clear`. Evidence requires
   `class_specific_review_required`; recovery state requires
   `explicit_per_item_review_required`.
2. **Protected-class exclusion by default.** Local history, rollback
   checkpoints, support evidence, policy bundles, offline entitlement bundles,
   and pinned review artifacts are excluded unless explicitly selected. A
   protected family can only reach the selected bucket with
   `explicit_selection = true`.
3. **Export-before-delete on protected classes.** Every protected row declares
   `export_required_before_delete` and links an export action, and the sheet
   carries a matching export-before-delete option.
4. **Rebuild-versus-loss disclosure.** Every row carries a non-empty rebuild
   disclosure, a rebuild-cost class, an offline-rebuild-risk class, and a
   reversibility class. Irreversible rows (authoritative recovery loss or
   evidence loss) spell out the consequence, and the sheet's
   `irreversible_consequences` list surfaces it.
5. **Low-disk ordering is never hidden.** A low-disk or managed-quota pressure
   sheet discloses the full low-disk eviction order across every artifact
   family.
6. **Quota pressure never silently deletes user-owned state.** Disk or quota
   pressure must never auto-select `user_owned_recovery_state`; when only
   protected classes remain over quota the sheet is `blocked_by_guardrail` and
   carries a guardrail notice instead of purging local state.
7. **Offboarding/reset accounts for every protected family.** An
   `offboarding_reset` sheet surfaces every protected family — profiler traces,
   replay bundles, support artifacts, review/incident evidence, and user-owned
   recovery state — as selected (explicitly, with export) or retained.
8. **Byte arithmetic and metadata safety.** Each row's `total_bytes` equals
   `freed_bytes + preserved_bytes`; the sheet totals equal the row sums; and the
   sheet carries no raw payload (`raw_content_exported = false`,
   `redaction_class = metadata_safe_default`).

## First consumer

`compose_review_sheet` folds the frozen artifact-family matrix plus a selection
request into a sheet that is correct by construction: it excludes protected
families unless explicitly selected, never auto-selects user-owned recovery
state under disk/quota pressure, never offers a generic clear of a protected
class, and discloses rebuild cost, export paths, and irreversible consequences
from the matrix row. The composed action for each family always stays within
the matrix's `allowed_clear_data_actions`.

## Support / export

`ClearDataReviewCorpus::support_export` projects the corpus into a
metadata-safe envelope — one summary row per sheet (flow, trigger, consent,
reclaim/preserved totals, export-option/irreversible/guardrail counts) — that
the support-bundle pipeline can quote without leaking raw payloads, paths, or
credentials. The golden projection is replay-gated at
`fixtures/storage/m5_clear_data_review/support_export.golden.json`.
