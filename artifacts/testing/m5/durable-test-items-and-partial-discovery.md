# M5 Durable Test-Item Discovery

- Packet: `durable-test-discovery:stable:0001`
- Label: `M5 Durable Test-Item Discovery`
- Snapshots: 4 (3 partial)
- Consumers: 4 / 4
- Node kinds present: 5 / 5

## Snapshots

- **snapshot:framework-pack:checkout** (framework_pack): partiality `complete`, support `fully_mapped_local`
  - Framework pack test explorer with a complete local enumeration
  - nodes: 5 (3 runnable leaves)
    - `framework:suite:checkout` [suite] identity=`stable` source=`local_resolved`
    - `framework:case:add-item` [concrete_case] identity=`stable` source=`local_resolved`
    - `framework:template:totals` [parameterized_template] identity=`stable` source=`local_resolved`
    - `framework:invocation:totals:usd` [concrete_invocation] identity=`stable` source=`local_resolved` template=`framework:template:totals`
    - `framework:invocation:totals:eur` [concrete_invocation] identity=`stable` source=`local_resolved` template=`framework:template:totals`
- **snapshot:notebook:analysis** (notebook): partiality `streaming`, support `partially_mapped_visible`
  - Notebook test cells; later cells still streaming in
  - nodes: 2 (2 runnable leaves)
    - `notebook:test:analysis:cell-2` [notebook_linked_test] identity=`stable` source=`local_resolved`
    - `notebook:test:analysis:cell-5` [notebook_linked_test] identity=`stable` source=`local_resolved`
    - omitted `omitted:notebook:tail-cells` (not_yet_streamed): Cells after ordinal 5 have not yet streamed in; the tail of the notebook stays visibly pending
- **snapshot:test-tree:aggregate** (test_tree): partiality `heuristic`, support `needs_remap_preserved`
  - Aggregate test tree with one node whose source moved and now needs remap
  - nodes: 2 (2 runnable leaves)
    - `tree:case:login` [concrete_case] identity=`stable` source=`local_resolved`
    - `tree:case:logout` [concrete_case] identity=`remap_review_required` source=`source_moved_needs_remap` prior_chain=1
    - omitted `omitted:test-tree:adapter-down` (adapter_unavailable): One framework adapter was unavailable; its subtree is recorded as omitted rather than dropped
- **snapshot:imported-ci:smoke** (imported_ci): partiality `provider_imported`, support `imported_read_only_mapped`
  - Imported CI overlay; read-only and never shown as a live local rerun
  - nodes: 1 (1 runnable leaves)
    - `ci:case:smoke` [concrete_case] identity=`imported_read_only` source=`imported_read_only`
    - omitted `omitted:imported-ci:provider-scope` (provider_owned_scope): The full provider scope is completed CI-side; the local overlay shows only the imported subset
