# M5 review-pack surface certification (M05-1283)

This contract is the **closing B152 surface-certification capstone** over the frozen M5 review-pack evaluator
matrix (`m5_review_pack_evaluator_matrix`). Where the freeze matrix defines the six governed review-pack object
classes — **review-pack record, ownership signal, required-evidence / required-check row, local-CI parity
strip, AI review policy hook, and review-template packet** — the M05-1275–1280 implementation lanes resolve
their review-pack record / result, ownership-signal / owner-conflict, required-evidence-check / local-CI-parity,
AI-policy-hook / AI-policy-result, review-template-packet / publish-attribution, and invalidation / rerun-compare
registry truth; this capstone **certifies** that the shared review-pack truth holds on every claimed M5
**review, AI, provider, browser-handoff, and support / export surface** — pack labels, pack version / digest,
owner provenance, evaluator result class, local-versus-provider parity, pack freshness, and template
attribution — and auto-narrows any profile that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_review_pack_surface_certification/`
- **Schema:** `schemas/review/m5-review-pack-surface-certification.schema.json`
- **Review proof:** `artifacts/review/m5-review-pack-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/review/m5-review-pack-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **consumer profile** a review / diff owner, an AI / automation flow, a
provider handoff consumer, or a support / export consumer reads a review pack through, not on the underlying
object class it renders:

1. **Fully-certified review-pack lane** — a review pack whose pack version / digest, owner provenance,
   evaluator result class, local-versus-provider parity, pack freshness, and template attribution all converge
   on one export-safe, provider-current, internally consistent record identical across every consumer. The
   **only** profile that may certify a `certified_review_pack_truth` claim.
2. **Reviewable review-pack record structure** — a self-sufficient, inspectable pack-bound record; certifies at
   most `reviewable_review_pack_record`.
3. **Stale-pack-version-digest profile** — a record whose pack version / digest can no longer be confirmed
   fresh; auto-narrows to `pack_version_digest_unverified_projection`.
4. **Unverified-owner-provenance profile** — a scope slice whose advisory-versus-enforced owner provenance can
   no longer be verified; auto-narrows to `owner_provenance_unverified_projection`.
5. **Unevaluated-required-check profile** — a required check that is ci-only, not-evaluated-here, or
   provider-unavailable; auto-narrows to `evidence_check_unverified_projection`.
6. **Local-only-parity profile** — a check whose result is a local parity estimate diverging from
   provider-authoritative state; auto-narrows to `local_parity_unverified_projection`.
7. **Undisclosed-AI-pack-binding profile** — an AI review that ran under a pack version / digest not disclosed
   against the active pack; auto-narrows to `ai_pack_binding_unverified_projection`.
8. **Stale-template-attribution profile** — a template whose attribution can no longer be verified against the
   pack it came from; auto-narrows to `template_attribution_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and review-pack-truth behavior — and resolves to a
derived verdict:

- **green** — every axis certified, every invariant held, the claimed review-pack tier delivered;
- **yellow** — a truth axis is not current, so the review-pack claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh certified claim, a hard invariant breaks,
  CLI/export parity drops, a non-lane profile claims a certified review-pack record, or the narrowing is
  inconsistent.

The eight seeded rows cover all six frozen object classes (review-pack record and review-template packet each
appear on a green and a yellow row), so the certification runs across the full matrix rather than a single
class.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `certified_review_pack_truth` / `reviewable_review_pack_record` claim while one of its truth axes is not
   current over-claims and blocks.
2. **Only a fully-certified review-pack lane may certify a certified review-pack record.** Every other profile
   is at most a reviewable review-pack record structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the pack
   label, pack version / digest, owner provenance, evaluator result class, local-versus-provider parity, pack
   freshness, and template attribution as text / JSON / Markdown.
4. **Every B152 hard invariant holds per row.** No profile may let a local parity estimate masquerade as
   provider-authoritative mergeability or approval truth; hide a ci-only, not-evaluated-here, or
   provider-unavailable state behind a green summary; flatten advisory-owner and enforced-owner into one owner
   pill; let AI review run under a different pack version without disclosure; or lose the review-pack version /
   digest or template attribution when exporting, publishing, or reopening review evidence.
5. **One canonical proof bundle.** Every row cites exactly one canonical review-pack evaluator matrix proof
   bundle (`artifacts/review/m5-review-pack-results/support_export.json`) — the frozen review-pack evaluator
   matrix proof — so support, docs / help, release, and public-proof surfaces consume a single review-pack
   certification source rather than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in review artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_REVIEW_PACK_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_review_pack_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
