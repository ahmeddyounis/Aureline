# M5 coverage overlays and snapshot / golden review

This document is the contract for the **coverage overlays**, **coverage merge /
import review**, and **snapshot / golden review** the M5 test-intelligence lane
uses to keep the evidence drawn over the editor and review surfaces trustworthy.
Where the stability-verdict / quarantine contract governs whether a *test* is
trustworthy, this contract governs whether the *coverage and snapshot evidence*
shown next to the code is trustworthy.

Coverage is no longer a single green percentage and a snapshot change is no longer
a blind accept-all. A coverage overlay distinguishes a verified current run from an
imported CI artifact, a cached local result, and a stale prior result; it tells
line coverage apart from branch coverage; and it emphasizes changed lines
distinctly from whole-scope coverage. A merge sheet discloses which runs it
included, which it excluded and why, and which shards or platforms it omitted. A
snapshot / golden review card preserves the artifact kind, counts, baseline scope,
and a raw fallback before any accept or reject.

## Source of truth

- Packet type: `CoverageReviewPacket`
  (`crates/aureline-runtime/src/coverage_overlays_and_snapshot_golden_review/`).
- Boundary schema:
  `schemas/testing/coverage-overlays-and-snapshot-golden-review.schema.json`.
- Checked support export:
  `artifacts/testing/m5/coverage-overlays-and-snapshot-golden-review/support_export.json`.
- Markdown summary:
  `artifacts/testing/m5/coverage-overlays-and-snapshot-golden-review.md`.
- Protected fixtures:
  `fixtures/testing/m5/coverage-overlays-and-snapshot-golden-review/`.

Regenerate the canonical export, summary, and fixture after any shape change:

```bash
cargo run -p aureline-runtime --example dump_coverage_overlay_snapshot_review
cargo run -p aureline-runtime --example dump_coverage_overlay_snapshot_review summary
cargo run -p aureline-runtime --example dump_coverage_overlay_snapshot_review fixture
```

## Coverage overlays

A `CoverageOverlayRecord` ties a stable `overlay_id` and a durable `CoverageScope`
to:

- a controlled `CoverageEvidenceProvenance` — `verified_current_run`,
  `imported_ci_artifact`, `cached_local_result`, `stale_prior_result`, or
  `unknown_requires_review` — the provenance vocabulary, so a verified run never
  reads the same as imported, cached, or stale evidence;
- a `CoverageMetricMode` (`line_coverage` or `branch_coverage`), an always-present
  `line_measure`, and an optional `branch_measure` present iff `branch_supported`;
- an optional `ChangedLineEmphasis` recording coverage of the lines changed since a
  diff base, kept distinct from whole-scope coverage;
- a gutter `legend` of `CoverageLegendEntry` rows over the `CoverageCellClass`
  vocabulary (`covered`, `uncovered`, `partially_covered_branch`, `not_executable`,
  `changed_covered`, `changed_uncovered`, `imported_unverified`,
  `stale_not_comparable`).

The packet validation requires `verified_current_run`, `imported_ci_artifact`,
`cached_local_result`, and `stale_prior_result` to each be represented, and both
`line_coverage` and `branch_coverage` modes to appear, so the vocabulary is
exercised, not merely declared (`provenance_coverage_missing`,
`metric_mode_coverage_missing`). At least one overlay must carry changed-line
emphasis (`changed_line_case_missing`).

Identity and truth rules every overlay obeys (`CoverageOverlayRecord::is_valid`):

- **A scope's fingerprint is never its bare id.** Each `CoverageScope` carries a
  `scope_fingerprint_token` distinct from its `scope_id`
  (`fingerprint_substitutes_identity`).
- **No green over stale or imported.** Only a `verified_current_run` overlay may set
  `presents_as_authoritative`; a stale overlay always discloses a
  `stale_not_comparable` legend cell (`green_over_stale_or_imported`).
- **Branch-versus-line truth.** `branch_supported` agrees with the presence of
  `branch_measure`, and a `branch_coverage`-mode overlay must actually measure
  branches (`branch_without_measure`).
- **Imported never reads as local** (`imported_reads_as_local`): an
  `imported_ci_artifact` overlay carries an `origin_provider_ref` and an
  `imported_unverified` legend cell, and a non-imported overlay carries neither.

## Coverage merge / import review

A `CoverageMergeReview` ties a stable `merge_id` and a durable `CoverageScope` to a
`merged_measure` and an explicit list of `MergedRunEntry` rows. Every contributing
run carries a `CoverageRunDisposition` — `included` or one of the excluded reasons
(`excluded_scope_mismatch`, `excluded_stale`, `excluded_imported_incomparable`,
`excluded_duplicate`, `excluded_conflict`). Omitted shards and platforms are
disclosed as `OmittedScopeEntry` rows over the `OmittedScopeKind` vocabulary
(`shard`, `platform`).

A merge sheet stays honest rather than implying complete certainty:

- a duplicate or conflict exclusion must carry a disclosed note rather than being
  silently dropped (`merge_invalid`);
- a merge that excludes any run or omits any shard / platform may not set
  `implies_complete_certainty` (`merge_implies_false_certainty`).

The packet validation requires at least one merge that exercises both an included
and an excluded run (`merge_distinction_missing`) and at least one merge that
discloses an omitted shard or platform (`omitted_scope_case_missing`).

## Snapshot / golden review

A `SnapshotReviewCard` ties a stable `card_id` and a durable `SnapshotSubject` to:

- a `SnapshotArtifactKind` (`image_snapshot`, `text_snapshot`,
  `serialized_snapshot`, `golden_file`, `binary_golden`);
- the `changed_artifact_count` and `total_artifact_count`;
- a `SnapshotBaselineScope` (`per_test`, `per_parameter_case`, `shared_fixture`,
  `platform_specific`);
- a `RawFallbackAvailability` (`text_diff_available`, `raw_artifact_referenced`,
  `unavailable_binary_only`);
- a `SnapshotReviewDecision` (`pending_review`, `accepted`, `rejected`,
  `needs_raw_inspection`).

Changes are preview-first; there is no blind accept-all:

- **No blind accept.** A change may only be `accepted` when a raw / text fallback
  supports a reviewed accept; a binary-only artifact with no fallback can never be
  accepted and routes through `needs_raw_inspection` (`snapshot_blind_accept`).
- **Preview-first.** Every card sets `preview_first`.
- **Templates stay distinct from invocations.** Each `SnapshotSubject` carries a
  `DurableTestNodeKind`; the packet requires both `parameterized_template` and
  `concrete_invocation` to appear (`template_collapsed_with_invocation`).
- **Imported never reads as local** (`imported_reads_as_local`): an imported card
  carries an `origin_provider_ref` and an `imported_read_only` subject identity, and
  is never `accepted` as a local baseline.

The packet validation requires at least one binary-only card routed through
`needs_raw_inspection` (`raw_fallback_case_missing`) and at least one imported card
held read-only (`imported_snapshot_case_missing`).

## Consumer projection and boundary discipline

The `CoverageReviewConsumerProjection` block records that the editor coverage
gutter, the coverage legend, the merge / import review sheet, the snapshot / golden
review UI, and release / support exports all normalize onto these records instead of
re-deriving coverage or snapshot truth.

The packet carries only typed class tokens, booleans, counts, opaque ids,
fingerprint digests, and redaction-aware reviewable labels. Raw coverage payloads,
snapshot bytes, golden-file bodies, baseline diffs, raw provider payloads, provider
cursors, credentials, and host names never cross this boundary.
