# M5 retired-state surface certification (M05-1246)

This contract is the **closing B148 surface-certification capstone** over the frozen M5
retired-state matrix (`m5_retired_state_matrix`). Where the freeze matrix defines the seven governed
retirement object classes — **supported line, stable capability, bundle, command / deep link, schema-bearing
surface, registry-visible package, and managed / new-tenant feature** — the 1239–1245 implementation lanes
resolve their retirement-manifest, manifest-change-diff, impact-report, blocker-gate, countdown, safety-gate,
review-packet, closure-gate, tombstone, claim-block-gate, last-supported-snapshot, archive-export-gate,
closure-ledger, and propagation-blocker-gate truth; this capstone **certifies** that the shared retired-state
truth holds on every claimed M5 **supported line and stable-facing surface** — complete retirement manifests,
exact-build last-supported snapshots, tombstones and archival routes, closed support notes, recorded migration
outcomes, and multi-profile propagation — and auto-narrows any profile that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_retired_state_surface_certification/`
- **Schema:** `schemas/release/m5-retired-state-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-retired-state-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/release/m5-retired-state-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **profile** a release engineer, release operator, program-governance owner,
or support engineer reads a retirement-manifest, last-supported-snapshot, tombstone, closure-ledger,
successor-route, or propagation surface through, not on the underlying object class it renders:

1. **Live retired-state closure lane** — a live, fully closed retiring class whose retirement manifest,
   exact-build last-supported snapshot, tombstone / archival route, closed support note, recorded migration
   outcome, and deployment-profile propagation all converge on one export-safe retired-state record. The
   **only** profile that may certify a `certified_retired_closure` claim.
2. **Reviewable retirement-record structure** — a self-sufficient, inspectable retirement-manifest /
   last-supported-snapshot / tombstone projection; certifies at most `reviewable_retirement_record`.
3. **Disclosed archive-partial profile** — a last-supported snapshot / archive whose coverage and exact-build
   join can only be partially disclosed; auto-narrows to `archive_disclosed_projection`.
4. **Unverified propagation profile** — a retiring class whose mirror / offline / self-hosted / managed
   deployment-profile propagation has aged out or a profile still lags; auto-narrows to
   `propagation_unverified_projection`.
5. **Unverified closure-ledger profile** — a retiring class whose support-note closure / migration-outcome
   retention is missing or whose closure ledger has become unreconstructable; auto-narrows to
   `closure_ledger_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and retired-state-truth behavior — and
resolves to a derived verdict:

- **green** — every axis certified, every invariant held, the claimed closure tier delivered;
- **yellow** — a truth axis is not current, so the closure claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh certified claim, a hard invariant breaks,
  CLI/export parity drops, a non-live profile claims a certified retired closure, or the narrowing is
  inconsistent.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a `certified_retired_closure`
   / `reviewable_retirement_record` claim while one of its truth axes is not current over-claims and blocks.
2. **Only a live, fully closed retired-state closure lane may certify a certified retired closure.** Every other
   profile is at most a reviewable retirement-record structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the
   last-supported version / channel, cutoff date, successor path, disable path, export / rollback route,
   archival note, migration outcome, support-note closure state, and registry reference as text / JSON /
   Markdown.
4. **Every B148 hard invariant holds per row.** No profile may let a retired surface disappear without a
   tombstone, archival route, or successor pointer; keep a retired class selectable in a new-install /
   new-tenant / marketplace / upgrade flow; destroy last-supported docs / schemas / evidence before support-note
   closure; leave retirement state unjoined to exact build, line identity, deployment profile, and migration
   outcome; or retire a surface through silent disappearance, stale selection UI, or orphaned support / docs
   truth.
5. **One canonical proof bundle.** Every row cites exactly one canonical retired-state matrix proof
   bundle (`artifacts/release/m5-retirements/support_export.json`) — the frozen retired-state matrix proof —
   so release, help, docs, support, public-proof, marketplace, and partner/procurement surfaces consume a
   single retired-state certification source rather than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_RETIRED_STATE_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_retired_state_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
