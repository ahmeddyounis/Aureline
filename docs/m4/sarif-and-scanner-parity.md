# SARIF and scanner parity

Aureline treats imported scanner findings as attributable evidence objects, not as current local analyzer output. SARIF and structured scanner imports flow through one scanner-import session model that records tool identity, rule-pack version, artifact roots, revision binding, raw-payload refs, signer/source class, freshness, baseline compatibility, and unsupported-field refs.

## Consumer contract

Problems, review, CLI/headless, support, and release consumers should ingest the runtime session and parity view directly:

- `ScannerImportSessionAlpha` preserves the import envelope and the normalized finding rows.
- `ScannerDeltaPacketAlpha` preserves baseline, suppression, waiver, resolved, persisting, new, and unmapped state.
- `DiagnosticReviewPacketAlpha` preserves review-quality rows and local-confirmation actions.
- `ScannerCiLocalParityView` keeps local analysis, remote-target analysis, managed pipeline findings, and imported provider results distinct while still comparable.
- `ImportedScannerSupportExport` and `ScannerImportReleasePacket` preserve redaction-safe evidence, release-visible debt, signer metadata, raw-payload backlinks, and compatibility gaps.

## Mutation policy

Imported metadata is inspect-only. It can identify a finding, support a review decision, or request local confirmation, but it cannot unlock silent fix-all. A source mutation needs either a locally confirmed compatible analyzer result or a separate governed quality-action proposal with current rule metadata and preview/apply/revert posture.

## Mirror and offline imports

Mirror-only and air-gapped imports remain first-class states. Packets carry the deployment route, signer ref, signature ref, trust-root ref, freshness, raw-payload ref, and public-fallback posture so release and support exports can reconstruct what was verified without live provider access.
