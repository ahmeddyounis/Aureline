# M5 Test-Evidence Certification Fixtures

## framework_row_narrows_on_stale_coverage_proof.json

An auto-narrowing drill fixture for the test-evidence certification packet. Every
claimed M5 test row — framework-pack, notebook, AI-test-generation, review-panel,
and imported-CI — certifies its discovery, session, watch, and selector-portability
proof (the required core) plus the quality dimension it claims: coverage on the
framework row, snapshot on the notebook row, and flaky/quarantine on the review
row.

The second framework-pack row claims `certified`, but its coverage evidence has
aged outside its freshness window (`coverage_evidence` carries a `stale_expired`
proof currency). Because a claimed row may not outrun current proof, the row
auto-narrows to an effective grade of `uncertified`, records a
`stale_dimension_proof` narrow trigger, and carries a precise narrowed label
rather than a generic provider error. Every other row keeps current, reopenable
proof for each dimension it certifies, so its effective grade equals its claim.

The imported-CI row is held read-only: its `imported_row` flag agrees with an
`imported_read_only` subject identity, and its proof currency is `imported_current`,
which backs the imported row's claim but never a local one — an imported CI verdict
never reads as a live local rerun. The framework row's subject is a
`parameterized_template` and the review/CI rows are `concrete_invocation`s, so a
parameterized template never collapses into a concrete invocation. Each dimension
certification names a reopenable proof ref keyed by a non-display fingerprint
distinct from the ref.

The fixture validates against
`schemas/testing/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality.schema.json`
and is byte-identical to the checked support export at
`artifacts/testing/m5/certify-test-discovery-session-watch-coverage-flaky-snapshot-evidence-quality/support_export.json`.
