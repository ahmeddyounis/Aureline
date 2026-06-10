# External-Tool Connector Manifest Fixtures

## blocked_connector_narrows.json

A connector-manifest catalogue captured after a connector signature failed
verification.

The managed review-comment connector stays at Stable: it is admitted, its
reversible external comment previews and is per-invocation approved and audited to
the evidence timeline, and its checkpoint-reversible rollback is verified.

The quarantined deploy connector demonstrates narrowing: its enterprise-gateway
signature could not be verified, so its state is `quarantined_signature` and it
claims `held` rather than any public lane. Its irreversible external publish shows
a diff, is denied by policy while quarantined, and is audited to the evidence
timeline; every downgrade rule narrows to `unavailable`, so a provider outage
keeps it out of every claimed lane.

This demonstrates that a blocked connector drops its public claim instead of
hiding behind a Stable, Beta, or Preview label, that a mutating side effect still
previews, gates, and audits even while blocked, and that the downgrade rules
narrow rather than hide the connector.
