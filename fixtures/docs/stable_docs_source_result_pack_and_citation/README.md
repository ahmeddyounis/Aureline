# Stable Docs Contract Fixtures

These fixtures exercise the stable docs-source, docs-result, docs-pack manifest, pack detail sheet, derived-citation, and citation-drawer contract.

Regenerate with:

```sh
for f in baseline_stable source_result_freshness_drift_blocks_stable citation_set_bundles_raw_pack_blocks_stable pack_detail_sheet_hides_actions_blocks_stable citation_drawer_drops_inference_marker_blocks_stable consumer_projection_drops_precedence_blocks_stable; do
  cargo run -q -p aureline-docs --bin aureline_stable_docs_contract -- fixture "$f" > "fixtures/docs/stable_docs_source_result_pack_and_citation/$f.json"
done
```
