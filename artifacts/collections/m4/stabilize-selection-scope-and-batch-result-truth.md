# Selection scope and batch-result truth artifact

This artifact publishes the stable dense-collection selection-scope packet for
search, provider-backed review/admin queues, and package/test/data-grid
surfaces.

Canonical files:

- Schema:
  `schemas/collections/selection-scope.schema.json`
- Packet:
  `artifacts/collections/m4/stabilize-selection-scope-and-batch-result-truth/selection_scope_packet.json`
- Fixtures:
  `fixtures/collections/m4/stabilize-selection-scope-and-batch-result-truth/`
- Rust contract:
  `crates/aureline-collections/src/stabilize_selection_scope_and_batch_result_truth/mod.rs`

The packet proves that stable surfaces preserve visible-vs-loaded-vs-matching
scope, query snapshot refs, hidden-member counts, stale snapshot state,
provider/local execution origin, tree range semantics, batch review counts, and
per-item mixed outcomes without scraping localized UI text.

Validation is expected to block packets that:

- expand to all matching query results without an explicit second step;
- include collapsed tree descendants in a range selection by default;
- drop a required surface family or consumer projection;
- lose hidden-member counts in review, export, support, or accessibility lanes;
- collapse mixed batch results into a single success/failure summary.
