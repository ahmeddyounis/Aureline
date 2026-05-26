# Docs-browser truth packet fixtures

These fixtures exercise the stable docs-browser truth packet at
`crates/aureline-docs/src/docs_browser_truth_packet/mod.rs`. Each fixture
declares its scenario, the constructor input the runtime materializes, and the
expected promotion state plus expected finding kinds.

| Fixture | Posture | Proves |
|---|---|---|
| `baseline_stable.json` | `stable` | Every required source class (`project_docs`, `mirrored_official_docs`, `extension_docs_pack`, `live_external_docs`, `derived_explanation`) ships on the same build, every symbol flow preserves identity through peek / split / browser_handoff / support_export / ai_handoff, and the nine required consumer projections preserve the packet verbatim. |
| `missing_required_source_class_blocks_stable.json` | `blocks_stable` | A packet that drops the `extension_docs_pack` source descriptor blocks promotion with `required_source_class_coverage_missing`. |
| `symbol_flow_drops_split_step_blocks_stable.json` | `blocks_stable` | A symbol-linked flow that drops the `split` step from its preserved-identity list blocks promotion with `symbol_flow_identity_lost`. |
| `result_source_ref_unpinned_blocks_stable.json` | `blocks_stable` | A docs-result row whose `docs_source_ref` does not match any descriptor blocks promotion with `result_source_ref_unpinned`. |
| `consumer_projection_drops_source_class_blocks_stable.json` | `blocks_stable` | A consumer projection that sets `preserves_source_class = false` blocks promotion with `source_class_taxonomy_dropped` and `consumer_projection_drift`. |
| `live_external_handoff_missing_packet_blocks_stable.json` | `blocks_stable` | A live external docs descriptor with `available_explicit` browser handoff but no packet ref blocks promotion with `browser_handoff_packet_missing`. |

Run the fixture suite with:

```
cargo test -p aureline-docs --test docs_browser_truth_packet
```

Regenerate every fixture (and the canonical artifact) with the emitter:

```
cargo run -q -p aureline-docs --bin aureline_docs_browser_truth_packet -- packet \
  > artifacts/search/m4/docs_browser_truth_packet.json

for case in baseline_stable \
            missing_required_source_class_blocks_stable \
            symbol_flow_drops_split_step_blocks_stable \
            result_source_ref_unpinned_blocks_stable \
            consumer_projection_drops_source_class_blocks_stable \
            live_external_handoff_missing_packet_blocks_stable; do
  cargo run -q -p aureline-docs --bin aureline_docs_browser_truth_packet -- fixture "$case" \
    > "fixtures/search/m4/docs_browser_truth_packet/${case}.json"
done
```
