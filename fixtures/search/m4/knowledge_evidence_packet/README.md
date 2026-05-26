# Knowledge-plane evidence packet fixtures

These fixtures exercise the stable knowledge-plane evidence packet at
`crates/aureline-graph/src/knowledge_evidence_packet/mod.rs`. Each
fixture declares its scenario, the constructor input the runtime
materializes, and the expected promotion state, finding kinds, and
consumer-projection preservation invariants.

| Fixture | Posture | Proves |
|---|---|---|
| `baseline_stable.json` | `stable` | Every closed ownership class (`curated_first_party`, `policy_derived`, `imported_provider_metadata`, `heuristic_inferred`) and every closed no-impact state binds to a citation-ready packet; nine consumer projections preserve the same packet verbatim. |
| `no_impact_collapsed_on_out_of_scope_blocks_stable.json` | `blocks_stable` | An impact card that declares `no_impact_in_workspace_or_slice` while carrying a non-zero `hidden_or_out_of_scope_count` blocks promotion with `no_impact_collapsed_on_out_of_scope_impact`. |
| `ownership_partiality_note_missing_blocks_stable.json` | `blocks_stable` | A heuristic-inferred ownership card that drops its partiality note blocks promotion with `ownership_partiality_note_missing`. |
| `generated_explainer_missing_citations_or_inference_blocks_stable.json` | `blocks_stable` | A generated explainer snapshot without citations or an explicit inference label blocks promotion with `generated_explainer_missing_citations_or_inference`. |
| `consumer_projection_drops_node_edge_identity_blocks_stable.json` | `blocks_stable` | A consumer projection that sets `preserves_node_edge_identity = false` blocks promotion with `node_edge_identity_dropped`, `consumer_projection_drift`, and `missing_consumer_projection`. |

Run the fixture suite with:

```
cargo test -p aureline-graph --test knowledge_evidence_packet
```
