# M5 evidence pointer — per-family release-candidate, blocker/evidence, and artifact-bundle graph

Evidence pointer for the per-family release graph that materializes one durable
release candidate per new M5 artifact family — with blockers, evidence-freshness
rows, known issues, rollback target, per-family scope, and a scoped
artifact-bundle card that joins binaries, sidecars, symbols, docs packs, schemas,
SDK artifacts, support packets, and compatibility rows by immutable digest and
exact-build identity. This row is a release-publication proof that sits beside the
M5 exact-build publication matrix and is governed by the canonical M5 evidence
index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Release graph: `artifacts/release/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family.json`
- Reviewer contract: `docs/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family.md`
- Fixture corpus: `fixtures/release/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family/`
- Owning crate module: `crates/aureline-release/src/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family/`
- Regenerator: `cargo run -p aureline-release --bin aureline_release_implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family -- emit-artifact artifacts/release/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family.json`

## Executable proof

Inline unit coverage lives in
`crates/aureline-release/src/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family/tests.rs`.
It loads the embedded graph, proves it validates cleanly, proves the embedded
JSON never drifts from the in-code builder, proves every M5 artifact family is
covered, proves every candidate lists every one of the eight bundle member kinds
(never omitted), proves per-family scope is not flattened, proves at least one
family narrows below the cutline, proves missing and stale required evidence
surface as blockers in the support-export projection, and exercises the
validation guards for omitted bundle members, undigested provided members,
backed candidates carrying an active gap, candidates that fail to narrow, and
missing owner sign-off.

## Narrowing rule

Any marketed or support-class row that depends on this graph narrows
automatically when the backing evidence is missing, stale, or downgraded: a
family that loses a required bundle member, ships a partial member, has stale or
missing required evidence, carries an open blocker, drops its rollback target or
exact-build identity, breaches or loses its proof packet, relies on an expired
waiver, or loses owner sign-off drops **below** the stable launch cutline and
narrows its published label, naming every gap reason, instead of inheriting an
adjacent backed family.
