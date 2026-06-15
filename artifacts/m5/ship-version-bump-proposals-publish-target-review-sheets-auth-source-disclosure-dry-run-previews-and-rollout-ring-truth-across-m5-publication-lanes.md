# M5 evidence pointer — publication review sheets: version-bump proposals, publish-target disclosure, dry-run previews, and rollout-ring truth

Evidence pointer for the publication review-sheet register that materializes one
inspectable review sheet per M5 publication lane — pairing the shared version-bump
proposal (prior/target version, affected artifacts, compatibility notes, migration
flags, public-surface impact) with the shared publish-target descriptor (target
class, visibility, mutability, auth-source class, dry-run preview, rollout ring,
mirror destination, rollback target), and proving the human review and the headless
plan share the same descriptor and diff payload, that the auth source and target
scope are disclosed before any mutation, and that no publish flow inherits ambient
credentials. This row is a release-publication proof that sits beside the M5
exact-build publication matrix and is governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Review-sheet register: `artifacts/release/m5/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes.json`
- Reviewer contract: `docs/m5/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes.md`
- Fixture corpus: `fixtures/release/m5/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes/`
- Owning crate module: `crates/aureline-release/src/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes/`
- Regenerator: `cargo run -p aureline-release --bin aureline_release_ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes -- emit-artifact artifacts/release/m5/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes.json`

## Executable proof

Inline unit coverage lives in
`crates/aureline-release/src/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes/tests.rs`.
It loads the embedded register, proves it validates cleanly, proves the embedded
JSON never drifts from the in-code builder, proves every M5 publication-lane kind
is covered, proves every cleared lane shares its publish-target descriptor and diff
payload across human review and headless plan, proves every cleared lane discloses
a non-ambient auth source and target scope before mutation, proves at least one
lane narrows below the cutline, proves every narrowing reason has a stop rule,
proves the narrowed lane surfaces its concrete disclosure gap in the support-export
projection, and exercises the validation guards for a cleared lane carrying an
active gap, an ambient-credential lane without its reason, a broken descriptor
parity, and a missing owner sign-off.

## Narrowing rule

Any marketed or support-class row that depends on this register narrows
automatically when the backing evidence is missing, stale, or downgraded: a lane
that hides its version-bump impact, fails to disclose its auth source or target
scope before mutation, would inherit ambient credentials, has an unavailable or
stale dry-run preview, diverges on its publish-target descriptor or diff payload
between human review and headless plan, does not disclose its rollout ring, records
no rollback target, breaches or loses its proof packet, relies on an expired
waiver, or loses owner sign-off drops **below** the stable launch cutline and
narrows its published label, naming every reason, instead of inheriting an adjacent
cleared lane.
