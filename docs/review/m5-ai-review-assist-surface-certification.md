# M5 AI-review-assist surface certification (M05-1273)

This contract is the **closing B151 surface-certification capstone** over the frozen M5 AI-review-assist
matrix (`m5_ai_review_assist_matrix`). Where the freeze matrix defines the four governed AI-review object
classes — **AI review finding row, review scope selector, publish-to-review sheet, and resolution memory
row** — the M05-1266–1270 implementation lanes resolve their finding / scope-source, review-scope-selector /
rerun-freshness, publish-to-review-sheet / publish-scope-decision, resolution-memory / finding-lifecycle, and
publish-continuity / compare-reconcile registry truth; this capstone **certifies** that the shared AI-review
truth holds on every claimed M5 **review, AI, provider, pending-review, and support / export surface** —
finding labels, finding class / severity badges, analyzed diff scope, publish destinations, local-versus-provider
state, and outdated / suppressed lifecycle history — and auto-narrows any profile that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_ai_review_assist_surface_certification/`
- **Schema:** `schemas/review/m5-ai-review-assist-surface-certification.schema.json`
- **Review proof:** `artifacts/review/m5-ai-review-assist-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/review/m5-ai-review-assist-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **consumer profile** a review / diff owner, an AI / automation flow, a
provider publish consumer, or a support / export consumer reads an AI review finding through, not on the
underlying object class it renders:

1. **Fully-classified AI-review lane** — an AI review finding whose finding class, analyzed diff scope, publish
   destination, local-versus-provider state, and lifecycle state all converge on one export-safe,
   provider-current, internally consistent record identical across every consumer. The **only** profile that
   may certify a `certified_ai_review_truth` claim.
2. **Reviewable AI-review record structure** — a self-sufficient, inspectable resolution-memory record;
   certifies at most `reviewable_ai_review_record`.
3. **Disclosed provider-freshness-partial profile** — a finding whose provider-freshness signal is stale;
   auto-narrows to `provider_freshness_disclosed_projection`.
4. **Unverified diff-scope profile** — a finding whose analyzed diff scope has drifted from the diff it was
   produced against; auto-narrows to `diff_scope_unverified_projection`.
5. **Unverified publish-target profile** — a finding whose provider publish target and write scope are
   unavailable; auto-narrows to `publish_target_unverified_projection`.
6. **Unverified finding-lifecycle profile** — a finding whose open / dismissed / suppressed / published /
   outdated lifecycle memory can no longer be verified; auto-narrows to
   `finding_lifecycle_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and ai-review-truth behavior — and resolves to a
derived verdict:

- **green** — every axis certified, every invariant held, the claimed AI-review tier delivered;
- **yellow** — a truth axis is not current, so the AI-review claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh certified claim, a hard invariant breaks,
  CLI/export parity drops, a non-lane profile claims a certified AI-review record, or the narrowing is
  inconsistent.

The six seeded rows cover all four frozen object classes (finding row and resolution memory row each appear on
a green and a yellow row), so the certification runs across the full matrix rather than a single class.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `certified_ai_review_truth` / `reviewable_ai_review_record` claim while one of its truth axes is not current
   over-claims and blocks.
2. **Only a fully-classified AI-review lane may certify a certified AI-review record.** Every other profile is
   at most a reviewable AI-review record structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the finding
   label, finding class, analyzed scope, publish destination, local-versus-provider state, and lifecycle state
   as text / JSON / Markdown.
4. **Every B151 hard invariant holds per row.** No profile may let AI review results publish or merge
   implicitly; hide whether output stays local or becomes a provider comment, a suggested patch, or a
   provider-specific check annotation; keep stale findings looking current after diff or instruction drift;
   lose local drafts or evidence when provider write scope is missing or publish fails; or present an AI review
   finding without its analyzed scope, publish destination, or lifecycle state.
5. **One canonical proof bundle.** Every row cites exactly one canonical AI-review-assist matrix proof bundle
   (`artifacts/review/m5-ai-review-publish-packets/support_export.json`) — the frozen AI-review-assist matrix
   proof — so support, docs / help, release, and public-proof surfaces consume a single AI-review certification
   source rather than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in review artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_AI_REVIEW_ASSIST_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_ai_review_assist_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
