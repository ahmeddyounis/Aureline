# Mixed-Version Compatibility and Skew Governance

Canonical JSON: `artifacts/compat/stable/mixed-version-compatibility-and-skew-governance.json`

Schema: `schemas/compat/mixed-version-compatibility-and-skew-governance.schema.json`

Fixtures: `fixtures/compat/stable/mixed-version-compatibility-and-skew-governance/`

The JSON matrix is the machine-readable truth source for supported skew windows,
negotiated fields, upgrade order, rollback order, downgrade posture, and
fail-closed unsupported behavior. Release-center and support-export consumers
must cite row ids from that matrix instead of cloning compatibility text.

Current publication verdict: hold, because
`mixed_version_boundary:provider_linked_packet` has stale skew-window and
rollback-order proof packets and is narrowed to Beta.
