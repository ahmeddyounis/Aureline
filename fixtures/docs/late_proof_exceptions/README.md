# Late-proof exception packet examples

Worked fixtures for the late-proof exception packet family frozen in
[`/docs/docsops/same_changeset_policy.md`](../../../docs/docsops/same_changeset_policy.md).
Every fixture here conforms to
[`/schemas/docs/late_proof_exception_packet.schema.json`](../../../schemas/docs/late_proof_exception_packet.schema.json).

The fixtures exist so docs, release, support, migration, compatibility,
and public-truth work can reuse one late-proof packet family when proof
for a release-bearing claim cannot land in the same change train as the
wording it backs.

## Intended usage

- **Schema conformance.** Every fixture MUST validate against
  [`/schemas/docs/late_proof_exception_packet.schema.json`](../../../schemas/docs/late_proof_exception_packet.schema.json).
- **Composition with late-copy.** Fixtures exercise the composition rule
  from the same-change-set policy: late-copy governs what the text says,
  late-proof governs what proof backs it and when the proof will land.
  Where the two compose, the fixture names the paired late-copy packet.
- **Detector linkage.** Fixtures whose origin is a stale-example
  detector cite the detector id from
  [`/artifacts/docs/stale_example_rules.yaml`](../../../artifacts/docs/stale_example_rules.yaml).
- **Shiproom projection.** Every fixture's packet id is safe to surface
  on the shiproom dashboard alongside waivers and freeze exceptions.

## Fixtures

- [`benchmark_proof_deferred_narrowed_claim.json`](./benchmark_proof_deferred_narrowed_claim.json)
  — benchmark corpus re-run slips a release train. The claim narrows
  from replacement-grade wording to limited wording for the train's
  lifetime; docs pane, release notes, and public-proof packet render the
  narrowed copy while the benchmark owner ships the rerun. The packet
  lands as `land_proof_and_drop_exception` once the rerun is signed.
- [`docs_pack_refresh_deferred_stale_examples.json`](./docs_pack_refresh_deferred_stale_examples.json)
  — a stale-example detector fired after the docs pack refresh window
  closed. The guided step is suppressed, the generated example is
  labeled stale on the primary surface, and a late-copy packet carries
  the corrected wording on Help/About. The packet carries matching
  impacted docs-pack refs, support-runbook update, and a successor
  reviewed-pack target.
- [`compatibility_proof_deferred_known_limit_route.json`](./compatibility_proof_deferred_known_limit_route.json)
  — compatibility report re-run could not land in the train. The
  compat row narrows, the claim posture moves to `limited`, and the
  migration-notes and support-export surfaces render the known-limit
  route while the compat owner re-runs the report. The reversal class
  is `withdraw_claim_and_route_to_known_limit`.

## Related artifacts

- [`/docs/docsops/same_changeset_policy.md`](../../../docs/docsops/same_changeset_policy.md)
  — same-change-set workflow, stale-example rubric, and late-proof
  exception path this family closes on.
- [`/artifacts/docs/stale_example_rules.yaml`](../../../artifacts/docs/stale_example_rules.yaml)
  — detector registry whose composite
  `release_bearing_needs_late_proof_exception` detector routes into
  this packet family.
- [`/docs/docs/reviewed_pack_and_late_copy_policy.md`](../../../docs/docs/reviewed_pack_and_late_copy_policy.md)
  — reviewed-pack / late-copy wording policy this family composes with.
- [`/fixtures/docs/late_copy_examples/`](../late_copy_examples/)
  — late-copy packet fixtures the examples here cite when both packets
  are open against the same train.
