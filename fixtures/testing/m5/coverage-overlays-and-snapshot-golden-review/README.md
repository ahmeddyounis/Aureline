# Coverage / snapshot-review fixtures

Proof fixtures for the coverage / snapshot-review packet
(`test_coverage_review_packet`). Each fixture is an export-safe packet that
`CoverageReviewPacket::validate` accepts and that exercises a specific truth the
contract guarantees.

- `binary_snapshot_requires_raw_inspection.json` — a packet whose overlays span a
  verified current run (line and branch), a changed-line emphasis, an imported CI
  artifact held read-only, a cached local result, and a stale prior result, whose
  merge sheet discloses an excluded duplicate / imported run plus an omitted shard
  and platform, and whose snapshot cards include a binary `image_snapshot` with no
  text or raw fallback. That binary card routes through `needs_raw_inspection`
  rather than a blind accept, proving a binary-only change cannot be accepted
  without raw inspection, alongside an imported card held read-only.

The boundary schema is
`schemas/testing/coverage-overlays-and-snapshot-golden-review.schema.json`; the
contract doc is
`docs/testing/m5/coverage-overlays-and-snapshot-golden-review.md`. Regenerate the
canonical export and this fixture with:

```bash
cargo run -p aureline-runtime --example dump_coverage_overlay_snapshot_review
cargo run -p aureline-runtime --example dump_coverage_overlay_snapshot_review fixture
```
