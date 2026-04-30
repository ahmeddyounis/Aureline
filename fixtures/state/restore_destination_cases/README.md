# Restore-destination review fixtures

These fixtures are short, reviewable scenarios that anchor the
restore-destination review, retained-vs-overwritten classes, and
checkpoint-before-overwrite vocabulary frozen in
[`/docs/state/restore_destination_review_contract.md`](../../../docs/state/restore_destination_review_contract.md)
and validated by:

- [`/schemas/state/restore_destination_review.schema.json`](../../../schemas/state/restore_destination_review.schema.json)
- [`/schemas/state/retained_vs_overwritten_row.schema.json`](../../../schemas/state/retained_vs_overwritten_row.schema.json)

Each fixture is one `state_restore_destination_review_record` body.
The destination-review record composes over a cross-artifact
`state_restore_provenance_and_placeholder_record` (referenced by
`restore_provenance_record_ref`) for every outcome above
`exact_restore`, so a reviewer can read the destination claim and the
fidelity claim side by side without flattening retained-vs-overwritten
treatment and missing-dependency placeholder cards into one opaque
payload.

## Scope rules

- Fixtures validate against the destination-review schema and the
  retained-vs-overwritten row schema; they do not encode raw secrets,
  raw absolute paths, raw command lines, raw logs, or raw source
  content.
- A new fixture MUST exercise at least one
  `destination_review_outcome` value (`exact_restore`,
  `partial_restore`, `incompatible_schema_downgrade`,
  `inspect_only_imported_package`, `placeholder_only_overlay`).
- Every reviewable class in §3 of the contract MUST appear in exactly
  one of `retained_rows[]`, `overwritten_rows[]`,
  `placeholder_only_rows[]`, `downgraded_rows[]`, or `omitted_rows[]`.
  A class that is silently absent from every row set is non-conforming.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.

## Index

| Fixture | Outcome | Key coverage |
|---|---|---|
| [`exact_restore_same_machine.yaml`](./exact_restore_same_machine.yaml) | `exact_restore` | every reviewable class round-trips; checkpoint gate is `not_applicable`; confirm_overwrite admitted with no checkpoint required |
| [`partial_restore_compatible_with_checkpoint.yaml`](./partial_restore_compatible_with_checkpoint.yaml) | `partial_restore` | workspace state overwritten and profile defaults merged through equivalence map under a mandatory mixed-scope checkpoint; layout topology overwritten with a recommended layout-snapshot; two live surfaces stand in as placeholder rows |
| [`incompatible_schema_downgrade.yaml`](./incompatible_schema_downgrade.yaml) | `incompatible_schema_downgrade` | producer refused schema downgrade; every class is `downgraded_to_compare_only` or `omitted`; checkpoint gate `not_applicable`; confirm_overwrite disabled with `compatibility_range_outside` |
| [`inspect_only_imported_package.yaml`](./inspect_only_imported_package.yaml) | `inspect_only_imported_package` | imported competitor profile bundle; every class is `inspect_only_overlay` or `omitted`; confirm_overwrite disabled with `inspect_only_outcome` |

## Coverage contract

The shared fixture set MUST keep:

- at least one case for each `destination_review_outcome` listed in the
  closed five-class set;
- at least one case where the checkpoint-before-overwrite gate is in
  `created` because at least one row's treatment overwrites or merges
  durable user state;
- at least one case where the gate is in `not_applicable` because no
  apply runs (exact restore, incompatible schema downgrade,
  inspect-only imported package);
- at least one case that exercises `recommended_before_overwrite` on a
  layout-topology row alongside `mandatory_before_overwrite` on a
  workspace-state or profile-defaults row;
- at least one case that surfaces `placeholder_only` rows alongside
  overwritten rows so reviewers see how the inventory composes; and
- at least one case where every reviewable class lands in exactly one
  row set and no class is silently absent from the inventory.
