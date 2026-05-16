# Target-Discovery Beta Fixtures

These fixtures back the integration test for the beta target-discovery layer
at
[`/crates/aureline-runtime/src/target_discovery/`](../../../../crates/aureline-runtime/src/target_discovery/).
Every fixture is exercised by
[`/crates/aureline-runtime/tests/target_discovery_beta.rs`](../../../../crates/aureline-runtime/tests/target_discovery_beta.rs).

The fixtures pin the closed vocabulary the beta layer ships with:

- `native_local_and_helper_managed.json` — one local-host row discovered via
  native protocol (all protected actions allowed) plus one managed-workspace
  row discovered via a structured adapter that resolved to stale-imported
  freshness because the helper version was not negotiated (run / test / build
  blocked on stale freshness; debug launch / attach blocked on unsupported
  capability; export artifact always permitted).
