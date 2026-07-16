# M5 supported-line surface certification (M05-1237)

This contract is the **closing B147 surface-certification capstone** over the frozen M5
supported-line-transparency matrix (`m5_supported_line_transparency_matrix`). Where the freeze matrix defines
the five governed supported-line proof objects — **public-proof ledger, transparency report, migration
scoreboard, ORR-history event, and correction-train archive** — the 1230–1236 implementation lanes resolve their
public-proof-ledger, claim-history-diff, transparency-report, snapshot-diff, migration-scoreboard,
scoreboard-delta, ORR-history, follow-up-closure, correction-train-archive, closure-gate, truth-feed,
audience-packet, retention-policy, and stale-escalation truth; this capstone **certifies** that the shared
durable-proof truth holds on every claimed M5 **supported line** — current public-proof ledgers, export-safe
transparency reports, versioned migration scoreboards, retained ORR history, and archived correction trains —
and auto-narrows any profile that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_supported_line_surface_certification/`
- **Schema:** `schemas/release/m5-supported-line-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-supported-line-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/release/m5-supported-line-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **profile** a release engineer, release operator, program-governance owner,
or support engineer reads a public-proof, transparency-report, migration-scoreboard, ORR-history,
correction-archive, or freshness-window surface through, not on the underlying proof object it renders:

1. **Live supported-line operating lane** — a live, first-party supported line whose current public-proof
   ledger, export-safe transparency report, versioned migration scoreboard, retained ORR history, and archived
   correction train all join to exact build / release-line identity within their freshness window. The **only**
   profile that may certify a `certified_operating_line` claim.
2. **Reviewable transparency structure** — a self-sufficient, inspectable public-proof / transparency-snapshot /
   archived-history projection; certifies at most `reviewable_transparency_surface`.
3. **Disclosed correction-archive profile** — a correction-train archive whose coverage and exact-build join can
   only be partially disclosed; auto-narrows to `correction_archive_disclosed_projection`.
4. **Unverified migration-scoreboard profile** — a migration scoreboard whose importer / bridge scoring and
   migration-pain deltas have aged out; auto-narrows to `migration_scoreboard_unverified_projection`.
5. **Unverified ORR-history profile** — an ORR-history line whose retained ORR / go-no-go / cohort-transition
   decisions are missing or whose archived history has become unreconstructable; auto-narrows to
   `orr_history_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and supported-line-proof-truth behavior — and
resolves to a derived verdict:

- **green** — every axis certified, every invariant held, the claimed operating tier delivered;
- **yellow** — a truth axis is not current, so the operating claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh certified claim, a hard invariant breaks,
  CLI/export parity drops, a non-live profile claims a certified operating line, or the narrowing is
  inconsistent.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a `certified_operating_line`
   / `reviewable_transparency_surface` claim while one of its truth axes is not current over-claims and blocks.
2. **Only a live first-party supported-line operating lane may certify a certified operating line.** Every other
   profile is at most a reviewable transparency structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the
   public-proof-ledger truth, transparency snapshot, migration-scoreboard currency, ORR-history retention,
   correction-archive retention, freshness window, export class, supported-line association, and registry
   reference as text / JSON / Markdown.
4. **Every B147 hard invariant holds per row.** No profile may widen a claim because a report once existed
   without current freshness, stay green on stale external proof or opaque upstream health, leak internal-only
   incident or security detail into a public-safe feed, leave public-proof / migration / history unjoined to
   build and release-line identity, or leave migration pain / ORR / correction history unretained.
5. **One canonical proof bundle.** Every row cites exactly one canonical supported-line-transparency proof
   bundle (`artifacts/release/m5-supported-line-transparency/support_export.json`) — the frozen
   supported-line-transparency matrix proof — so release, help, docs, support, public-proof, and
   partner/procurement surfaces consume a single supported-line certification source rather than hand-authored
   prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_SUPPORTED_LINE_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_supported_line_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
