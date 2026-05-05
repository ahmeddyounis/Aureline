# Advisory identity and affected-install assessment case fixtures

These fixtures anchor the shared advisory identity and local
affected-install assessment contracts in:

- [`/docs/security/advisory_identity_and_install_assessment_contract.md`](../../../docs/security/advisory_identity_and_install_assessment_contract.md)
- [`/schemas/security/advisory_identity.schema.json`](../../../schemas/security/advisory_identity.schema.json)
- [`/schemas/security/affected_install_assessment.schema.json`](../../../schemas/security/affected_install_assessment.schema.json)

Each YAML file is one boundary record (`advisory_identity_record` or
`affected_install_assessment_record`). The records are copy-safe by
construction: they carry opaque refs, controlled vocabularies, and
reviewable summaries, but do not include raw exploit payloads, raw
reporter identities, raw hostnames, raw paths, private registry URLs,
raw signatures, or raw secret material.

See `manifest.yaml` for the case index.

