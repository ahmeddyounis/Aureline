# Docs Maintenance And Stale Example Governance

This stable packet is the source of truth for docs render configs, suggestions, validation results, stale-example findings, suppression records, and maintenance packets across Help/About, onboarding cards, migration guidance, release notes, docs feedback export, docs packs, CLI/headless output, and support/community handoff.

Rendered Markdown is never canonical source or release proof. Each render config carries the preview mode, CommonMark baseline, extension baseline, active-content posture, source/version refs, security profile, mirror/offline posture, browser handoff packet when applicable, and evidence refs.

Every stable docs lane must emit a maintenance packet for README, changelog, onboarding, module docs, and support article classes. Packets carry source/version context, validation freshness, pending suggestions, stale findings, share/export posture, and publish-boundary notes. Stale-example suppressions are governed records with actor, reason, expiry, and evidence refs; page-local comments and silent exemptions do not satisfy this contract.

Canonical artifacts:

- Schema: `schemas/docs/docs-maintenance-and-stale-example-governance.schema.json`
- Packet: `artifacts/docs/m4/docs-maintenance-and-stale-example-governance.json`
- Fixtures: `fixtures/docs/m4/docs-maintenance-and-stale-example-governance/`

Regenerate the packet with:

```sh
cargo run -q -p aureline-docs --bin aureline_docs_maintenance_governance -- packet > artifacts/docs/m4/docs-maintenance-and-stale-example-governance.json
```
