# Collection-truth corpus fixtures

This directory holds the deterministic shell-side collection-truth
corpus packet plus the per-record projections consumed by the
`aureline-shell::collection_truth_corpus_fixtures` replay test, the
checked-in QE report and matrix artifacts under
`artifacts/qe/m3/`, and the QE drills doc under
`docs/qe/m3/collection_truth_drills.md`.

Layout:

```
packet.json                  -- full corpus packet
corpus_cases.json            -- expanded per-surface corpus cases
saved_view_migrations.json   -- saved-view migration cases
accessibility_drills.json    -- keyboard / screen-reader drills
support_export.json          -- redacted support export projection
matrix.json                  -- coverage matrix projection
```

`packet.json` is the full
`shell_collection_truth_corpus_packet_record` (validated by
`validate_collection_truth_corpus_packet`).

## Regenerating fixtures

```sh
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- packet \
  > fixtures/ux/m3/collection_truth_corpus/packet.json
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- cases \
  > fixtures/ux/m3/collection_truth_corpus/corpus_cases.json
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- migrations \
  > fixtures/ux/m3/collection_truth_corpus/saved_view_migrations.json
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- drills \
  > fixtures/ux/m3/collection_truth_corpus/accessibility_drills.json
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- support-export \
  > fixtures/ux/m3/collection_truth_corpus/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- matrix-json \
  > fixtures/ux/m3/collection_truth_corpus/matrix.json
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- matrix-json \
  > artifacts/qe/m3/collection_truth_matrix.json
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- report-md \
  > artifacts/qe/m3/collection_truth_report.md
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- drills-md \
  > docs/qe/m3/collection_truth_drills.md
```

## Validation

```sh
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth_corpus -- validate
cargo test -p aureline-shell --test collection_truth_corpus_fixtures
```

The seeded packet is deterministic; the fixture files are diffable
review state for design QA, support seeds, and the conformance suite.
