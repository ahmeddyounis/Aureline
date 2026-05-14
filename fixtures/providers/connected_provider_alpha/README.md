# Connected-provider registry alpha fixtures

This directory contains the protected fixture packet for the
connected-provider registry alpha. The packet is consumed by the
`aureline-provider` crate and covers:

- code-host, issue, and CI/check provider descriptors;
- local draft, publish now, open in provider, publish-later queue, and
  inspect-only surface states;
- publish-later queue rows with dependency order, stale-target risk,
  reauth/rescope posture, and support-export-safe summaries;
- pipeline run, log, artifact, and annotation overlays plus auditable
  rerun, cancel, and retry controls.

Verify it with:

```bash
cargo test -p aureline-provider
cargo run -p aureline-provider --bin aureline_provider_alpha -- --validate-only
```
