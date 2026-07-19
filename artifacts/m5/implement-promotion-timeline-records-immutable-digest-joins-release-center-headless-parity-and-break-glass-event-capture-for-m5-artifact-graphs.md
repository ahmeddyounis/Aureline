# M5 evidence pointer — artifact-graph promotion ledgers: timeline records, immutable-digest joins, release-center/headless parity, and break-glass capture

Evidence pointer for the promotion-ledger register that materializes one
inspectable promotion ledger per M5 artifact family — joining the affected
artifact-graph node set (every node carrying an immutable digest) with an ordered
promotion timeline that records, for every step, the source stage, destination
stage, approving actors, evidence bundle refs, immutable digest refs, reversible
window, and rollback target, and that captures break-glass freezes, emergency
publications, and out-of-band corrections in the same step model as ordinary
promotions. The register proves the release-center UI and the headless plan
reconstruct the same promotion history, exposes an audit/postmortem replay of who
promoted what, when, on which evidence, and with which reversible window, and
narrows any family that lets an emergency bypass timeline capture or digest
binding or lets a mutable "latest" pointer stand in for immutable graph history.
This row is a release-publication/provenance proof that sits beside the M5
exact-build publication matrix and is governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Promotion-ledger register: `artifacts/release/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs.json`
- Operator/auditor contract: `docs/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs.md`
- Fixture corpus: `fixtures/release/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs/`
- Owning crate module: `crates/aureline-release/src/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs/`
- Regenerator: `cargo run -p aureline-release --bin aureline_release_implement_promotion_timeline_records_immutable -- emit-artifact artifacts/release/m5/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs.json`

## Executable proof

Inline unit coverage lives in
`crates/aureline-release/src/implement_promotion_timeline_records_immutable_digest_joins_release_center_headless_parity_and_break_glass_event_capture_for_m5_artifact_graphs/tests.rs`.
It loads the embedded register, proves it validates cleanly, proves the embedded
JSON never drifts from the in-code builder, proves every M5 artifact-family kind is
covered, proves every reconstructable family reconstructs the same history across
release-center and headless flows with an available audit replay whose ordered step
ids equal the timeline, proves every promotion step — ordinary and break-glass —
binds an immutable digest resolving to the affected node set, proves break-glass
events ride the same timeline model and that a held family's break-glass steps are
reconciled and never bypass capture, proves the audit export replays who promoted
what, when, on which evidence, and with which reversible window, proves at least one
family narrows below the cutline, proves every narrowing reason has a stop rule, and
exercises the validation guards for a held family carrying an active gap, a mutable
latest pointer, a broken reconstruction, an emergency step that strips its digest
binding, and a missing owner sign-off.

## Narrowing rule

Any marketed or support-class row that depends on this register narrows
automatically when the backing evidence is missing, stale, or downgraded: a family
whose promotion bypassed timeline capture, whose step binds no immutable digest,
whose affected node set does not back a cited digest, that drives publication from a
mutable "latest" pointer, whose release-center and headless reconstructions diverge,
that exposes no audit/postmortem replay, that carries an unreconciled break-glass
action, whose step discloses neither a reversible window nor a rollback target, that
rides stale blocking evidence, that breaches or loses its proof packet, that relies
on an expired waiver, or that loses owner sign-off drops **below** the stable launch
cutline and narrows its published label, naming every reason, instead of inheriting
an adjacent reconstructable family.
