# Incident workspace alpha fixtures

These fixtures exercise the incident workspace/runbook packet alpha
contract implemented by `crates/aureline-incident`.

The protected case keeps hosted/provider spans unavailable while local
logs, crash refs, task history, runbook metadata, and support bundle ids
remain attached by reference. Missing spans are first-class rows instead
of empty arrays or implied success.

## Coverage

| Fixture | Focus |
|---|---|
| [`provider_unavailable_missing_span.yaml`](./provider_unavailable_missing_span.yaml) | Provider trace lane unavailable, local continuity preserved, support runbook packet consumed by reference |

