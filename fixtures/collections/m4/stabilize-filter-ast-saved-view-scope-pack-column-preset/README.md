# Stable Dense-Collection Contract Fixtures

`baseline_stable.json` is generated from
`aureline_collections::seeded_dense_collection_contract_packet`.

Negative fixtures intentionally mutate the baseline:

- `missing_data_grid_blocks_stable.json`
- `provider_review_missing_batch_sheet_blocks_stable.json`
- `scope_vocabulary_dropped_blocks_stable.json`

Validate the canonical packet with:

```sh
cargo test -p aureline-collections
```
