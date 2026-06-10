# Docs and Code Semantic Recall

- Packet: `packet:m5:semantic_recall:retry_backoff_session`
- Session: semantic recall: request retry and backoff handling
- Promotion: `stable` (0 findings)
- Queries: 3 | Rows: 4 | Degradations: 1

## Query-session ledger

1. [initial/docs_and_code] retry and backoff handling
2. [narrowed/code_only] retry/backoff in the http client
3. [reformulated/docs_and_code] exponential backoff with jitter

## Results

1. `result:code:retry_with_backoff_fn` (code_symbol) — workspace_code / exact_build_match / authoritative_live / local / high
   - Reason: exact symbol match in workspace code; called directly by the http client send path
   - Provenance: verbatim_node / cited=true
2. `result:docs:retry_policy_guide` (docs_node) — project_docs / exact_build_match / authoritative_live / local / high
   - Reason: exact build match on the local retry-policy guide with strong lexical+semantic overlap
   - Provenance: verbatim_node / cited=true
3. `result:docs:mirrored_backoff_reference` (docs_node) — mirrored_official_docs / compatible_minor_drift / warm_cached / mirrored_pack / medium
   - Reason: pinned, signed mirror of the upstream backoff reference within the compatible drift window
   - Provenance: verbatim_node / cited=true
4. `result:code:http_client_send` (code_file) — workspace_code / exact_build_match / warm_cached / local / medium
   - Reason: workspace file that invokes the retry helper; explained via a cited summary, not a verbatim node
   - Provenance: derived_summary / cited=true

## Degradations

- [code_graph_stale/advisory]: the code graph was indexed before the last two commits; call edges may lag the working tree
