# M5 Runbook Source Register

- Register: `m5-runbook-source-register:stable:0001`
- Label: `M5 runbook source register`
- Evaluated as-of: `2026-07-06T00:00:00Z`
- Sources: 5
- Executable: 4 · Reference-only: 1
- Exposed on: docs browser, incident workspace, operator dashboards, support exports

## Governed runbook sources

| Source | Provenance | Authority | Freshness | Executable | Signer |
|--------|------------|-----------|-----------|------------|--------|
| `src:repo-pipeline-restart` | `repo_local` | Authoritative | `fresh` | yes | release-signing-key:runbooks via signed_first_party (verified) |
| `src:mirror-observability-pack` | `mirrored_docs_pack` | Mirrored | `fresh` | yes | mirror-digest:sha256:obs-pack via mirror_digest (verified) |
| `src:catalog-failover` | `managed_catalog` | Managed | `fresh` | yes | catalog-manifest:dr-catalog via catalog_manifest (verified) |
| `src:browser-vendor-scaling` | `browser_reference` | Reference only | `fresh` | no | browser-capture:vendor-scaling via browser_capture (unverified) |
| `src:browser-promoted-dr` | `browser_reference` | Authoritative | `fresh` | yes | browser-capture:vendor-dr via browser_capture (unverified) |
