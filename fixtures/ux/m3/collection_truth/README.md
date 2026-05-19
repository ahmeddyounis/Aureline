# Collection-truth beta fixtures

This directory holds the deterministic shell-side collection-truth
beta packet plus per-surface worked records consumed by both the
`aureline-shell::collection_truth` fixture replay test and downstream
support/design QA.

Layout:

```
packet.json
<surface_family>/
  filter_bar.json
  saved_view.json
  scope_counter.json
  batch_review.json
```

`packet.json` is the full
`shell_collection_truth_beta_packet_record` (validated by
`validate_collection_truth_beta_packet`). The per-family files are
serializations of the same records with their own boundary schema
files under `/schemas/ux/`:

- `filter_bar.json` ↔ `schemas/ux/filter_bar_state.schema.json`
- `saved_view.json` ↔ `schemas/ux/saved_collection_view.schema.json`
- `scope_counter.json` ↔ `schemas/ux/collection_scope_counter.schema.json`
- `batch_review.json` ↔ `schemas/ux/batch_review_sheet.schema.json`

## Regenerating fixtures

```sh
cargo run -q -p aureline-shell --bin aureline_shell_collection_truth -- packet \
  > fixtures/ux/m3/collection_truth/packet.json
for fam in search_or_result_grid review_inbox log_or_event_collection \
           package_or_inventory_grid work_item_board admin_or_settings_grid; do
  mkdir -p "fixtures/ux/m3/collection_truth/$fam"
  cargo run -q -p aureline-shell --bin aureline_shell_collection_truth -- \
    filter-bar "$fam"   > "fixtures/ux/m3/collection_truth/$fam/filter_bar.json"
  cargo run -q -p aureline-shell --bin aureline_shell_collection_truth -- \
    saved-view "$fam"   > "fixtures/ux/m3/collection_truth/$fam/saved_view.json"
  cargo run -q -p aureline-shell --bin aureline_shell_collection_truth -- \
    scope-counter "$fam"> "fixtures/ux/m3/collection_truth/$fam/scope_counter.json"
  cargo run -q -p aureline-shell --bin aureline_shell_collection_truth -- \
    batch-review "$fam" > "fixtures/ux/m3/collection_truth/$fam/batch_review.json"
done
```

The seeded packet is deterministic; the fixture files are diffable
review state for design QA, support seeds, and conformance suites.
