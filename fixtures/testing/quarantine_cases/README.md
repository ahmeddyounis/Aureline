# Test quarantine and mute worked cases

These fixtures anchor the contract in
[`/docs/testing/test_quarantine_and_mute_contract.md`](../../../docs/testing/test_quarantine_and_mute_contract.md)
and validate against:

- [`/schemas/testing/quarantine_record.schema.json`](../../../schemas/testing/quarantine_record.schema.json)

The fixture set covers:

| Fixture | Key coverage |
|---|---|
| [`time_bounded_quarantine.yaml`](./time_bounded_quarantine.yaml) | active owner-backed quarantine with expiry, evidence, review cadence, release debt, waiver linkage, and mandatory release visibility |
| [`ownerless_expired_mute.yaml`](./ownerless_expired_mute.yaml) | degraded expired mute with missing owner, handoff required, scorecard/claim/promotion visibility, and stable-promotion block |
| [`recovered_stable_again.yaml`](./recovered_stable_again.yaml) | stable-again recovery row citing prior treatment, unblock evidence, stable evidence window, and release-packet closure |
| [`policy_muted_enterprise_restriction.yaml`](./policy_muted_enterprise_restriction.yaml) | enterprise policy mute with policy owner, policy epoch binding, affected scope, review trigger, and waiver-linked release debt |

Fixtures MUST NOT encode raw command lines, raw stdout or stderr byte
streams, raw environment bodies, raw absolute paths, raw URLs, raw
secret values, raw test names, raw assertion bodies, raw source
excerpts, raw artifact bytes, raw provider payloads, raw policy
payloads, or raw stack traces. They use opaque refs, counts, class
labels, bounded summaries, and timestamps.

Removing one of the four scenario classes is a breaking contract
coverage reduction.
