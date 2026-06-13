# M5 Provider-Backed Team-Workflow Certification Fixtures

These fixtures are valid, export-safe certification packets that exercise the
downgrade automation the canonical support export keeps green. Each keeps
every claimed row present, the provider-family compatibility stories intact,
and the compatibility report aligned with the row verdicts; the difference is
which row narrowed or blocked and why.

## deferred_publish_evidence_blocked.json

The deferred-publish continuity row is blocked because its evidence packet
failed validation. The compatibility report records one blocked row and
`all_rows_publishable = false`, proving the certification refuses to keep a
broad publish-later claim green when that row loses current proof.

## provider_linked_mutation_proof_stale_narrowed.json

The provider-linked mutation row is narrowed on stale proof. The row
remains publishable, but its `proof_fresh` flag flips to `false` and the
compatibility report records two narrowed rows instead of hiding the drift.
