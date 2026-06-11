# M5 Companion Lane Certification Fixtures

These fixtures are generated deterministically from the first-consumer
certification builder (`canonical_m5_companion_certification`) in
`aureline-companion` and validate against
`schemas/companion/certify-companion-incident-sync-residency-encryption-and-offboarding-lanes-on-every-marketed-m5-profile.schema.json`.
Each one applies `apply_downgrade_automation` to the canonical certification, so it
demonstrates that an adverse live signal narrows the claim honestly rather than
shipping greener than the evidence.

## proof_stale_certification.json

Every lane's proof has gone stale. Each headline claim and every certified profile
row narrows one qualification step and one rollout step, and every row — certified
or not — is forced to a labeled `stale` freshness state. `degraded_labels` records
`proof_stale` and `freshness_downgraded_to_stale`. Demonstrates that a stale proof
narrows the certification and downgrades freshness honestly instead of showing
stale state as live.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_m5_companion_certification -- proof_stale
```

## sync_evidence_invalid_certification.json

The managed-sync lane's evidence failed validation. The lane is held (`held`) and
every certified profile row is withheld (`withheld`, `certified_on_profile: false`)
rather than shipped greener than its proof. `degraded_labels` records
`evidence_invalid`. Other lanes are untouched. Demonstrates that invalid evidence
holds and withholds a single lane without narrowing the rest.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_m5_companion_certification -- sync_evidence_invalid
```

## residency_unverified_certification.json

The residency/encryption lane's customer-managed-key or end-to-end-encryption claim
could not be verified. The lane narrows one step (Beta → Preview) on the two
managed profiles where it is certified (`team_managed`, `enterprise_managed`); the
profiles with no managed plane were never marketed and stay unclaimed.
`degraded_labels` records `residency_or_encryption_unverified`. Demonstrates that
an unverifiable residency or encryption claim narrows only the profiles where the
managed lane is actually certified.

Regenerate with:

```text
cargo run -p aureline-companion --example dump_m5_companion_certification -- residency_unverified
```
