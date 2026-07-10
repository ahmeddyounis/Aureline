# M5 snapshot-review-card / coverage-import-merge-sheet primitive

This document is the contract reference for the reusable M5 **snapshot / golden review card** and
**coverage-import / merge sheet** — two governed test-intelligence components implemented as one
twin primitive in the `aureline-runtime` crate
(`implement_snapshot_or_golden_review_cards_and_coverage_import_merge_sheets_with_artifact_baseline_identity_raw_or_text_fallback_shard_inclusion_truth_and_stale_or_incompatible_warnings_across_claimed_m5_review_surfaces`).

It narrows two of the seven families frozen by the
[test-intelligence component matrix](m5_test_intelligence_component_matrix.md) —
`snapshot_review_card` and `coverage_import_merge_sheet` — into two resolvers plus a parity
matrix, so a reviewer sees the baseline and the merge scope **before** trusting a derived quality
signal.

## Why this exists

A user should never accept a snapshot / golden change with a blind `Accept all` that hides the
artifact count, the scope the snapshot was captured under, or whether a raw / text fallback is
even available. And a merged coverage number should never be treated as exact current truth while
a shard omission, an incompatible artifact, or a stale report is still unresolved. This primitive
makes each of those states explicit and identical across every claimed review consumer.

## Snapshot / golden review card

`resolve_snapshot_review_card` takes one card's artifact kind, baseline identity, diff state,
render / raw fallback mode, environment / viewport / theme / serializer / locale scope
dimensions, diff count, and provenance class, and derives a **review posture** that is
one-to-one with the snapshot diff state:

| Snapshot diff state | Review posture | Acceptance decision? |
| --- | --- | --- |
| `matches_baseline` | `matches_baseline_card` | no |
| `diff_detected` | `diff_detected_card` | yes |
| `new_snapshot` | `new_snapshot_card` | yes |
| `obsolete_snapshot` | `obsolete_snapshot_card` | no |
| `render_unavailable` | `render_unavailable_card` | no |
| `raw_text_fallback` | `raw_text_fallback_card` | no |

Because the map is one-to-one, a new snapshot never reads as a matched baseline. An **acceptance
decision (a detected diff or a new snapshot) is only accepted when it discloses at least one scope
dimension**; otherwise resolution fails with `BlindAcceptanceWithoutScope`. This is the
acceptance-criterion guarantee: a snapshot acceptance can never collapse to a blind `Accept all`
without its artifact count, scope, and fallback visibility. An **opaque binary artifact or a
render-unavailable card is only accepted when a raw / text fallback path exists**; otherwise
resolution fails with `RawFallbackMissingForOpaqueArtifact`, so a binary-only change always keeps
a raw / text fallback. The artifact kind, baseline identity, diff count, fallback mode, and scope
are always carried.

Actions: `reveal_snapshot_details`, `open_raw_fallback`, and `export_snapshot_review` are always
offered; `accept_baseline` and `reject_change` whenever the card is an acceptance decision.

## Coverage-import / merge sheet

`resolve_coverage_import_merge_sheet` takes one sheet's coverage import source, merge-resolution
state, line-versus-branch metric kinds, included and excluded run labels, commit / build
identity, stale and incompatible flags, and an exact-current-truth claim, and derives a **merge
posture** that is one-to-one with the merge-resolution state:

| Merge-resolution state | Merge posture | Needs attention? |
| --- | --- | --- |
| `merged_clean` | `merged_clean_sheet` | no |
| `shard_omission_detected` | `shard_omission_sheet` | yes |
| `conflicting_overlap` | `conflicting_overlap_sheet` | yes |
| `partial_merge` | `partial_merge_sheet` | yes |
| `superseded_by_newer` | `superseded_sheet` | yes |
| `merge_unavailable` | `merge_unavailable_sheet` | yes |

Because the map is one-to-one, a shard omission never reads as a clean merge and the sheet never
invents an alternate label. A **shard-omission or partial-merge sheet must name at least one
excluded run**; otherwise resolution fails with `OmittedShardsWithoutDisclosure`. A **merged
number is only treated as exact current truth when the merge is clean and no omission,
incompatible artifact, or stale report is unresolved**; otherwise resolution fails with
`ExactTruthWithUnresolvedWarnings`. This is the acceptance-criterion guarantee: a coverage merge
exposes omitted shards / platforms and incompatible artifacts before any merged result is treated
as exact current truth. The included and excluded runs, the commit / build identity, the
stale-or-incompatible warnings, and the line-versus-branch support are always carried.

Actions: `reveal_merge_details`, `review_run_scope`, and `export_merge_sheet` are always offered;
`open_incompatible_report` whenever a warning is unresolved.

## Parity matrix

`M5SnapshotMergeComponentsPacket` binds one row per claimed review consumer — the snapshot review
panel, the editor snapshot diff, the coverage-import / merge panel, the headless/CLI review
surface, and the review export — to the shared card and sheet anatomy, vocabulary, postures,
actions, export fields, and non-visual accessibility routes, so the same snapshot / merge grammar
holds across the panel, the editor, the merge panel, CI/headless, and support consumers with
identical vocabulary. Each row carries four hard invariants (all `false`):

- `collapses_snapshot_accept_without_scope_or_fallback`
- `hides_baseline_identity_or_artifact_count`
- `hides_shard_omission_or_incompatible_warning`
- `invents_alternate_snapshot_or_merge_state_label`

## Boundary

Raw snapshot payloads, pasted paths, credentials, and private endpoints stay outside the export
boundary; every card identity, baseline ref, sheet identity, commit / build identity, and run
label is carried only as an opaque, export-safe representation.

## Artifacts

- Canonical packet schema: `schemas/ui/m5-snapshot-review-card.schema.json`
- Coverage-import-merge-sheet companion schema: `schemas/ui/m5-coverage-import-merge-sheet.schema.json`
- Support export: `artifacts/release/m5-snapshot-coverage-import-primitive-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-snapshot-coverage-import-primitive-proof/matrix.csv`
- Design report: `artifacts/design/m5-snapshot-coverage-import-primitive.md`
- Narrowed fixtures: `fixtures/ui/m5-snapshot-coverage-import-primitive/`

All are minted from the seed builders by the `aureline_runtime_snapshot_coverage_import_primitive`
headless emitter; the checked-in support export is asserted equal to the seed builder in tests.
