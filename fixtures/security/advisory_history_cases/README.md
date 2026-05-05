# Advisory history and resolution case fixtures

These fixtures anchor the advisory history timeline and resolution
contract in:

- [`/docs/security/advisory_history_and_resolution_contract.md`](../../../docs/security/advisory_history_and_resolution_contract.md)
- [`/schemas/security/advisory_timeline_entry.schema.json`](../../../schemas/security/advisory_timeline_entry.schema.json)

Each YAML fixture is one `advisory_history_timeline_record` describing a
renderable timeline snapshot plus the entries it retains. The fixtures
are copy-safe by construction: they carry stable IDs, typed vocabularies,
and reviewable summaries, but do not include raw exploit payloads, raw
reporter identities, raw hostnames, raw paths, private registry URLs, raw
signatures, or secret material.

See `manifest.yaml` for the case index.

