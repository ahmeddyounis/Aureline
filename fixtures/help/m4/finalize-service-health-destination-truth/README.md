# Service-Health Destination Truth Fixtures

These fixtures are literal projections of
`aureline_service_health_destination_truth`:

- `canonical_descriptor.json` is the shared descriptor consumed by About, Help,
  service health, CLI/headless output, diagnostics, support export, release
  notes, migration notices, issue/report templates, and community handoff.
- `support_export_projection.json` is the local-first support projection.
- `validation_report.json` is the typed validation report for the descriptor.

Regenerate with:

```sh
cargo run -q -p aureline-service-health --bin aureline_service_health_destination_truth -- descriptor > fixtures/help/m4/finalize-service-health-destination-truth/canonical_descriptor.json
cargo run -q -p aureline-service-health --bin aureline_service_health_destination_truth -- support-export > fixtures/help/m4/finalize-service-health-destination-truth/support_export_projection.json
cargo run -q -p aureline-service-health --bin aureline_service_health_destination_truth -- validation > fixtures/help/m4/finalize-service-health-destination-truth/validation_report.json
```
