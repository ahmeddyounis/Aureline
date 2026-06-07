# Shared Service-Health Feed Fixtures

These fixtures are generated from the typed shared service-health feed contract:

- `canonical_feed.json` is the stable shared feed
- `support_export_projection.json` is the metadata-safe support/export view
- `validation_report.json` is the protected validation report

Regenerate with:

```sh
cargo run -q -p aureline-service-health --bin aureline_service_health_destination_truth -- canonical-feed > fixtures/help/m4/stabilize-service-health-feed-objects-outage-scope/canonical_feed.json
cargo run -q -p aureline-service-health --bin aureline_service_health_destination_truth -- canonical-feed-support-export > fixtures/help/m4/stabilize-service-health-feed-objects-outage-scope/support_export_projection.json
cargo run -q -p aureline-service-health --bin aureline_service_health_destination_truth -- canonical-feed-validation > fixtures/help/m4/stabilize-service-health-feed-objects-outage-scope/validation_report.json
```

The protected tests compare `canonical_feed.json` against
`aureline_service_health::canonical_service_health_feed()` directly, so fixture
updates should normally be generated from the binary or a small regeneration
helper rather than hand-edited.
