# Docs-pack truth packet fixtures

These fixtures exercise the stable docs-pack truth packet at
`crates/aureline-docs/src/docs_pack_truth_packet/mod.rs`. Each fixture declares
its scenario, the constructor input the runtime materializes, and the expected
promotion state plus expected finding kinds.

| Fixture | Posture | Proves |
|---|---|---|
| `baseline_stable.json` | `stable` | Every required source class (`project_docs`, `generated_reference`, `mirrored_official_docs`, `curated_knowledge_pack`, `support_runbook`, `extension_docs_pack`), every required local-availability posture (`available_local`, `mirror_offline_pinned`, `not_installed`, `unavailable_disclosed`, `quarantined`), and every required render mode (`rendered`, `syntax_checked`, `executed_locally`, `executed_remotely`, `mirrored_only`, `browser_handoff_only`, `not_rendered`) is exercised; nearby-version, stale-example, and quarantined-pack states stay distinct; citation sets export without raw pack bodies; and the ten required consumer projections preserve the packet verbatim. |
| `offline_pack_loses_signer_identity_blocks_stable.json` | `blocks_stable` | A pack with local content unavailable drops its signing-authority ref and the validator blocks promotion with `pack_manifest_incomplete`. Identity must stay attributable even when content is unavailable locally. |
| `nearby_version_dropped_collapses_stale_state_blocks_stable.json` | `blocks_stable` | A nearby-version finding drops its `nearby_version_ref` and the validator blocks promotion with `stale_state_collapsed`. |
| `citation_set_bundles_raw_pack_blocks_stable.json` | `blocks_stable` | A citation-set export bundles raw pack bodies by default and the validator blocks promotion with `citation_set_bundles_raw_pack`. |
| `stale_suppression_loses_attribution_blocks_stable.json` | `blocks_stable` | A stale-example suppression drops its actor attribution and the validator blocks promotion with `suppression_attribution_lost`. |
| `consumer_projection_drops_render_mode_blocks_stable.json` | `blocks_stable` | A consumer projection sets `preserves_render_mode = false` and the validator blocks promotion with `render_mode_vocabulary_dropped` and `consumer_projection_drift`. |
| `quarantined_finding_collapsed_blocks_stable.json` | `blocks_stable` | A quarantined-pack finding now references a publishable pack and the validator blocks promotion with `stale_state_collapsed`. |

Run the fixture suite with:

```
cargo test -p aureline-docs --test docs_pack_truth_packet
```

Regenerate every fixture (and the canonical artifact) with the emitter:

```
cargo run -q -p aureline-docs --bin aureline_docs_pack_truth_packet -- packet \
  > artifacts/search/m4/docs_pack_truth_packet.json

for case in baseline_stable \
            offline_pack_loses_signer_identity_blocks_stable \
            nearby_version_dropped_collapses_stale_state_blocks_stable \
            citation_set_bundles_raw_pack_blocks_stable \
            stale_suppression_loses_attribution_blocks_stable \
            consumer_projection_drops_render_mode_blocks_stable \
            quarantined_finding_collapsed_blocks_stable; do
  cargo run -q -p aureline-docs --bin aureline_docs_pack_truth_packet -- fixture "$case" \
    > "fixtures/search/m4/docs_pack_truth_packet/${case}.json"
done
```
