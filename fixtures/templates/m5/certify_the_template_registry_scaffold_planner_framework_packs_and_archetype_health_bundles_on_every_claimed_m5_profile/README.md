# M5 Template, Scaffold, Framework-Pack, and Archetype-Health Certification Fixtures

These fixtures are valid, export-safe certification packets that exercise the
downgrade automation the canonical support export keeps green. Each keeps every
claimed profile present, trust-review and consumer-projection invariants
satisfied, proof freshness valid, and the compatibility report in agreement with
the profile verdicts — the difference is which profile narrowed or blocked and
why.

## framework_pack_evidence_blocked.json

The framework-pack-header profile is blocked because its evidence packet failed
validation. `apply_downgrade_automation` moved the profile to `blocked`, the
compatibility report shows one blocked profile, and `all_profiles_publishable`
is `false` — proving stale or underqualified evidence narrows the claim instead
of shipping greener than the proof.

## archetype_health_proof_stale_narrowed.json

The archetype-health-bundle profile is narrowed from `certified` to
`narrowed_certified` because its proof went stale relative to the freshness SLO;
the profile's `proof_fresh` flag flips to `false` while every other profile keeps
its verdict. Demonstrates proof-staleness narrowing a claim rather than hiding
the profile.
