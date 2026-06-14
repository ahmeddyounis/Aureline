# Durable test-item discovery fixtures

Proof fixtures for the durable test-item discovery packet
(`durable_test_item_discovery_packet`). Each fixture is an export-safe packet
that `DurableTestDiscoveryPacket::validate` accepts and that exercises a specific
truth the contract guarantees.

- `discovery_snapshot_preserves_identity_chain_on_source_move.json` — a packet
  whose framework-pack snapshot keeps a parameterized template distinct from its
  concrete invocations (separate `node_id`s), whose notebook snapshot streams in
  with a visible omitted-scope tail, whose aggregate test-tree snapshot degrades
  one node to `remap_review_required` after a source move **without dropping its
  prior identity chain**, and whose imported CI overlay stays read-only and never
  reads as a live local rerun.

The boundary schema is
`schemas/testing/durable-test-items-and-partial-discovery.schema.json`; the
contract doc is `docs/testing/m5/durable-test-items-and-partial-discovery.md`.
Regenerate the canonical export with:

```bash
cargo run -p aureline-runtime --example dump_durable_test_discovery
```
