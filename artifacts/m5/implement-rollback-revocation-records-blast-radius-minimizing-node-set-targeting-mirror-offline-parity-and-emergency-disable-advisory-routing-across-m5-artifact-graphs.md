# M5 evidence pointer — artifact-graph recovery ledgers: rollback/revocation records, blast-radius-minimizing node-set targeting, mirror/offline parity, and emergency-disable/advisory routing

Evidence pointer for the rollback/revocation register that materializes one
inspectable recovery ledger per M5 artifact family — joining the affected
artifact-graph node set (every node carrying an immutable digest and an
installable-after-action flag) with the scoped rollback, revocation, yank, repin,
and emergency-disable records that target the smallest affected node set while
preserving unaffected nodes as installable, keep the artifact graph consistent,
bind a last-known-good target, and route security advisories. The register proves
the hosted, mirrored, and offline channels each receive the same recovery records
and advisories, captures emergency-disable bundles in the same auditable record
model as an ordinary rollback, exposes an audit/advisory replay of every recovery
action and its per-channel delivery, and narrows any family that over-revokes a
preservable node or withholds emergency truth from the mirrored or offline channel.
This row is a release-publication/advisory/revocation proof that sits beside the M5
exact-build publication matrix and the promotion-ledger register and is governed by
the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Recovery-ledger register: `artifacts/release/m5/implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs.json`
- Operator/auditor contract: `docs/m5/implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs.md`
- Fixture corpus: `fixtures/release/m5/implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs/`
- Owning crate module: `crates/aureline-release/src/implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs/`
- Regenerator: `cargo run -p aureline-release --bin aureline_release_implement_rollback_revocation_records_blast -- emit-artifact artifacts/release/m5/implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs.json`

## Executable proof

Inline unit coverage lives in
`crates/aureline-release/src/implement_rollback_revocation_records_blast_radius_minimizing_node_set_targeting_mirror_offline_parity_and_emergency_disable_advisory_routing_across_m5_artifact_graphs/tests.rs`.
It loads the embedded register, proves it validates cleanly, proves the embedded
JSON never drifts from the in-code builder, proves every M5 artifact-family kind is
covered, proves every contained family delivers the same recovery truth to the
hosted, mirrored, and offline channels at parity, proves every recovery record
targets the smallest affected node set and preserves every unaffected node as
installable without over-revoking, proves emergency-disable records ride the same
record model and that a contained emergency-disable is reconciled and advisory-routed,
proves the audit/advisory export replays each action's kind, blast radius, advisory,
and per-channel delivery, proves at least one family narrows below the cutline, proves
every narrowing reason has a stop rule, and exercises the validation guards for a
contained family with an active gap, an over-revoked preservable node, emergency truth
withheld from the offline channel, a broken artifact graph, a missing owner sign-off,
and a mirror delivery gap.

## Narrowing rule

Any marketed or support-class row that depends on this register narrows
automatically when the backing evidence is missing, stale, or downgraded: a family
whose recovery record leaves its blast radius unscoped, that does not preserve an
installable node, that leaves the artifact graph broken, whose restore action cites
no last-known-good target, whose mirrored or offline channel has no delivery path,
whose channel recovery truth is stale, whose withdrawal record routes no advisory,
that holds an unreconciled emergency-disable, that rides stale blocking evidence, that
breaches or loses its proof packet, that relies on an expired waiver, or that loses
owner sign-off drops **below** the stable launch cutline and narrows its published
label, naming every reason, instead of inheriting an adjacent contained family.
