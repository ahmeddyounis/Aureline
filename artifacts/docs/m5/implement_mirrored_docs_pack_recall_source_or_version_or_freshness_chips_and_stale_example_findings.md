# Mirrored Docs-Pack Recall

- Packet: `packet:m5:docs_pack_recall:async_runtime_setup`
- Recall: docs recall: async runtime setup
- Promotion: `stable` (0 findings)
- Rows: 3 (2 stale findings)

## Results

1. `result:project_docs:runtime_overview` — project_docs / exact_build_match / authoritative_live / local / high
   - Reason: exact build match on local project docs with strong lexical+semantic overlap
2. `result:mirrored:tokio_runtime_guide` — mirrored_official_docs / compatible_minor_drift / warm_cached / mirrored_pack / high
   - Reason: pinned, signed mirror of official docs within the compatible drift window
3. `result:curated:async_patterns` — curated_knowledge_pack / unknown_target_build / degraded_cached / mirrored_pack / medium
   - Reason: curated knowledge pack match; target build unknown so version is disclosed, not assumed

## Stale-example findings

- `result:mirrored:tokio_runtime_guide` [nearby_version/advisory]: a nearer-version example exists for the active build
- `result:curated:async_patterns` [stale_example/advisory]: the example predates the current API and is flagged for review
