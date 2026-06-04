# SARIF and scanner parity evidence packet

This packet is the stable evidence source for imported scanner findings, SARIF-compatible imports, delta packets, local-confirmation parity, and export-safe review/support/release projections.

## Canonical assets

| Asset | Path | Purpose |
|---|---|---|
| Runtime model | `crates/aureline-runtime/src/scanner_import/mod.rs` | Materializes SARIF and structured scanner imports into sessions, finding deltas, review packets, support exports, release packets, and CI/local parity views. |
| Schema | `schemas/quality/sarif-and-scanner-parity.schema.json` | Defines the stable fixture and CI/local parity view boundary. |
| Fixture corpus | `fixtures/quality/m4/sarif-and-scanner-parity/` | Proves signed mirror-only scanner import, raw-payload refs, unsupported-field retention, local-confirmation parity, and safe release projection. |
| Consumer test | `crates/aureline-runtime/tests/scanner_import_beta.rs` | Verifies import sessions, delta counts, support redaction, release packets, mirror/offline metadata, and source-separated parity rows. |

## Stable truth

- Scanner import sessions preserve tool identity, tool version, rule-pack version, artifact roots, revision binding, raw-payload refs, signer evidence, deployment route, source class, freshness, and unsupported-field refs.
- Finding deltas only produce `new`, `resolved`, `persisting`, `suppressed`, `waived`, and `unmapped` rows through the compatibility model. Rule-pack, profile/tool, stale, redacted, and anchor-mapping failures narrow the parity claim instead of fabricating clean deltas.
- The CI/local parity view separates `local_analysis`, `remote_target_analysis`, `managed_pipeline`, and `imported_provider` source rows. It compares them with explicit parity states and keeps imported rows read-only.
- Support and release projections preserve finding IDs, rule IDs, categories, source kinds, rule metadata refs, suppression/baseline refs, raw-payload backlinks, signer metadata, and redaction class without embedding raw scanner bodies or raw source.
- Mirror-only and air-gapped imports carry deployment-route truth and block public fallback. Signed payloads and baselines cite signer, signature, and trust-root refs.

## Guardrails

- Imported findings are not live local analyzer truth.
- Untrusted imported metadata cannot unlock silent fix-all; local confirmation or a separate governed quality action is required before mutation claims.
- Compatibility failures remain visible as parity gaps or unmapped states.
- Raw scanner evidence is retained by opaque refs and excluded from normalized export bodies.
