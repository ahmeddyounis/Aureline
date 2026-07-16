# M5 historical-evidence surface certification (M05-1255)

This contract is the **closing B149 surface-certification capstone** over the frozen M5
historical-reference matrix (`m5_historical_reference_matrix`). Where the freeze matrix defines the five
governed non-live-evidence object classes — **retirement snapshot, support / export evidence, archived runbook
packet, imported / offline route evidence, and review / incident snapshot** — the 1248–1254 implementation
lanes resolve their historical-snapshot-descriptor, descriptor-change-diff, archived-snapshot-viewer,
historical-versus-live compare, live-target-handoff, expiry / removal-state, imported / offline
lineage-propagation, and drill-corpus truth; this capstone **certifies** that the shared non-live-evidence
truth holds on every claimed M5 **support, retirement, incident, review, and export surface** — snapshot
labels, capture time, provenance, mutation-blocked posture, imported / offline warnings, expired / removed
metadata fallback, and validated open-live-target handoffs — and auto-narrows any profile that cannot sustain
it.

- **Module:** `crates/aureline-ui/src/m5_historical_evidence_surface_certification/`
- **Schema:** `schemas/release/m5-historical-evidence-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-historical-evidence-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/release/m5-historical-evidence-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **profile** a support engineer, release operator, program-governance owner,
or review / incident owner reads a snapshot descriptor, archived packet, imported / offline evidence, or
live-target-handoff surface through, not on the underlying object class it renders:

1. **Current non-live-evidence lane** — a current, fully-attributed archived-snapshot / captured-evidence class
   whose snapshot label, capture time, provenance lineage, mutation-blocked posture, and validated live-target
   handoff (or metadata-only exit) all converge on one export-safe non-live-evidence record. The **only**
   profile that may certify a `certified_non_live_evidence` claim.
2. **Reviewable snapshot-record structure** — a self-sufficient, inspectable snapshot descriptor / captured
   support / export evidence projection; certifies at most `reviewable_snapshot_record`.
3. **Disclosed imported / offline-partial profile** — imported / offline route evidence whose coverage and
   live-route join can only be partially disclosed; auto-narrows to `imported_offline_disclosed_projection`.
4. **Unverified live-target profile** — an archived packet whose live-target existence, scope, route, trust, or
   authority can no longer be validated; auto-narrows to `live_target_unverified_projection`.
5. **Unverified expiry / removal-ledger profile** — a review / incident snapshot whose retention / removal
   metadata is missing or whose expiry / removal ledger has become unreconstructable; auto-narrows to
   `expiry_removal_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and non-live-evidence-truth behavior — and
resolves to a derived verdict:

- **green** — every axis certified, every invariant held, the claimed non-live-evidence tier delivered;
- **yellow** — a truth axis is not current, so the non-live-evidence claim auto-narrows to the weakest
  supported ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh certified claim, a hard invariant breaks,
  CLI/export parity drops, a non-current-lane profile claims a certified non-live-evidence record, or the
  narrowing is inconsistent.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a
   `certified_non_live_evidence` / `reviewable_snapshot_record` claim while one of its truth axes is not
   current over-claims and blocks.
2. **Only a current, fully-attributed non-live-evidence lane may certify a certified non-live-evidence record.**
   Every other profile is at most a reviewable snapshot-record structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the snapshot
   label, capture time, provenance, live-target availability, imported / offline status, mutation-blocked
   posture, and expiry / removal state as text / JSON / Markdown.
4. **Every B149 hard invariant holds per row.** No profile may let archived or imported / offline evidence look
   live, writable, or current by omission; reopen a live target from a snapshot without validating target
   identity, trust, route, and authority; dead-link an expired / removed artifact when it can still show
   metadata, provenance, or safe cleanup state; leave non-live evidence unjoined to capture time, provenance,
   retention / removal state, or any current live-target mismatch; or present a snapshot or imported / offline
   packet as a current live object or reopen through an ambiguous route.
5. **One canonical proof bundle.** Every row cites exactly one canonical historical-reference matrix proof
   bundle (`artifacts/support/m5-historical-evidence/support_export.json`) — the frozen historical-reference
   matrix proof — so support, docs / help, release, and public-proof surfaces consume a single
   non-live-evidence certification source rather than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_HISTORICAL_EVIDENCE_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_historical_evidence_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
