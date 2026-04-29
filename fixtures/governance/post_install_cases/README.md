# Post-install disclosure fixture cases

These fixtures validate the contract in
[`/docs/governance/post_install_notice_and_provenance_contract.md`](../../../docs/governance/post_install_notice_and_provenance_contract.md)
and the schema at
[`/schemas/governance/post_install_disclosure.schema.json`](../../../schemas/governance/post_install_disclosure.schema.json).

Every YAML file is a single `post_install_disclosure_record`. The
records are pre-implementation governance artifacts: they carry opaque
refs, controlled vocabulary values, privacy-safe notes, access-point
rules, and export projections, but no raw artifact bytes, raw
signatures, raw SBOM bodies, raw notice text, raw registry URLs, raw
license files, raw advisory payloads, private mirror endpoints, or
customer identifiers.

| Fixture | Source class | Subject | Scenario |
|---|---|---|---|
| [`official_signed_build.yaml`](./official_signed_build.yaml) | `official` | `product_build` | Signed official desktop build with verified signature, attestation, SBOM formats, complete notices, and current revocation state |
| [`mirrored_artifact_stale_revocation.yaml`](./mirrored_artifact_stale_revocation.yaml) | `mirrored` | `mirrored_transport_artifact` | Offline update bundle whose origin verifies but whose revocation snapshot is stale and visibly refreshable |
| [`side_loaded_extension.yaml`](./side_loaded_extension.yaml) | `side_loaded` | `extension_package` | Side-loaded extension after review with missing license, notice, SBOM, and attestation data rendered as visible states |
| [`generated_export_redistribution_hint.yaml`](./generated_export_redistribution_hint.yaml) | `official` | `generated_user_artifact` | Generated export produced by an official build with lineage refs and a redistribution review hint |
