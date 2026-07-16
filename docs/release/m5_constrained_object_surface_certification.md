# M5 constrained-object surface certification (M05-1264)

This contract is the **closing B150 surface-certification capstone** over the frozen M5
constrained-file-state matrix (`m5_constrained_file_state_matrix`). Where the freeze matrix defines the six
governed constrained-current-object classes — **read-only, generated, policy-locked, managed, projection, and
captured-snapshot** — the M05-1257–1263 implementation lanes resolve their constrained-state descriptor,
change-diff, badge-group / reason-strip consumer, canonical-source relation, write-target review,
write-review-sheet fallback path, cross-actor mutation gate, drill corpus, and support / export evidence
truth; this capstone **certifies** that the shared constrained-object truth holds on every claimed M5 **editor,
review, save, AI, repair, and export surface** — state badges, blocked-write reason strips, canonical-source
rows, exact write targets, reviewed safe-next-steps, and actor-parity blocking — and auto-narrows any profile
that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_constrained_object_surface_certification/`
- **Schema:** `schemas/release/m5-constrained-object-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-constrained-object-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/release/m5-constrained-object-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **consumer profile** an editor / save operator, a review / diff owner, an
AI / automation flow, or a support / export consumer reads a constrained current object through, not on the
underlying object class it renders:

1. **Fully-classified constrained-object lane** — a constrained current object whose state badge, blocked-write
   reason, canonical-source relation, exact write target, and reviewed safe-next-step all converge on one
   export-safe, internally consistent record identical across every consumer. The **only** profile that may
   certify a `certified_constrained_object_truth` claim.
2. **Reviewable constrained-state record structure** — a self-sufficient, inspectable constrained-state
   descriptor; certifies at most `reviewable_constrained_state_record`.
3. **Disclosed generated-divergence-partial profile** — a generated / derived artifact whose divergence from
   its generator can only be partially disclosed; auto-narrows to `generated_divergence_disclosed_projection`.
4. **Unverified canonical-source profile** — a projection whose canonical source or backing object can no longer
   be resolved; auto-narrows to `canonical_source_unverified_projection`.
5. **Unverified write-target-review profile** — a managed object whose reviewed write target and
   preserved-versus-lost sync note can no longer be reconstructed; auto-narrows to
   `write_target_review_unverified_projection`.
6. **Unverified actor-parity profile** — a captured snapshot whose shared constrained-write block across the
   direct-edit, AI, automation, import, and repair actors can no longer be verified; auto-narrows to
   `actor_parity_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and constrained-object-truth behavior — and resolves
to a derived verdict:

- **green** — every axis certified, every invariant held, the claimed constrained-object tier delivered;
- **yellow** — a truth axis is not current, so the constrained-object claim auto-narrows to the weakest
  supported ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh certified claim, a hard invariant breaks,
  CLI/export parity drops, a non-lane profile claims a certified constrained-object record, or the narrowing is
  inconsistent.

The six seeded rows cover all six frozen object classes (one class per row), so the certification runs across
the full matrix rather than a single class.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `certified_constrained_object_truth` / `reviewable_constrained_state_record` claim while one of its truth
   axes is not current over-claims and blocks.
2. **Only a fully-classified constrained-object lane may certify a certified constrained-object record.**
   Every other profile is at most a reviewable constrained-state record structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the state
   badge, blocked-write reason, canonical source, exact write target, write disposition, and safe-next-step as
   text / JSON / Markdown.
4. **Every B150 hard invariant holds per row.** No profile may let one constrained-state class hide another when
   both materially affect behavior; let a generated, managed, projection, or archived object silently fall back
   to a lossy direct write; give an AI, automation, import, or repair flow a hidden bypass around the
   constrained-state rules; leave the canonical source, exact write target, preserved-versus-lost sync, or
   recovery / regenerate path unstated; or present a constrained object as directly writable or hide the
   recovery / regenerate path.
5. **One canonical proof bundle.** Every row cites exactly one canonical constrained-file-state matrix proof
   bundle (`artifacts/support/m5-constrained-object-state/support_export.json`) — the frozen constrained-file-state
   matrix proof — so support, docs / help, release, and public-proof surfaces consume a single constrained-object
   certification source rather than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_CONSTRAINED_OBJECT_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_constrained_object_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
