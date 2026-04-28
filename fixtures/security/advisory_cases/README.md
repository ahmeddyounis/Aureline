# Advisory surface case fixtures

These fixtures validate the advisory-surface contract in
[`/docs/security/advisory_surface_contract.md`](../../../docs/security/advisory_surface_contract.md)
and
[`/schemas/security/advisory_card.schema.json`](../../../schemas/security/advisory_card.schema.json).

Each JSON file is an `advisory_surface_record`. The records are
pre-implementation governance artifacts: they carry opaque refs,
controlled vocabulary values, privacy-safe notes, and exact build /
install-profile linkage, but no raw exploit payloads, raw signatures,
raw reporter identities, raw hostnames, raw paths, or private registry
URLs.

| Fixture | Surface kind | Scenario |
|---|---|---|
| [`staged_disclosure_advisory.json`](./staged_disclosure_advisory.json) | `advisory_card` | Private staged disclosure with public aliases pending publication |
| [`active_emergency_disable.json`](./active_emergency_disable.json) | `emergency_banner` | Active emergency disable with local-only, managed, offline-mirror, and manual-import delivery rows |
| [`mirror_only_advisory.json`](./mirror_only_advisory.json) | `advisory_card` | Mirror-only advisory where snapshot freshness and receipt refs remain visible without live update reachability |
| [`superseded_advisory_chain.json`](./superseded_advisory_chain.json) | `advisory_card` | Superseded, mitigated advisory whose ids, mitigation, emergency refs, and resolved history chain remain reachable |
