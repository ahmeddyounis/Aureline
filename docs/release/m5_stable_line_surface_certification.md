# M5 stable-line surface certification (M05-1228)

This contract is the **closing B146 surface-certification capstone** over the frozen M5 stable-line-protection
matrix (`m5_stable_line_protection_matrix`). Where the freeze matrix defines the five governed active
stable / stable-candidate lines — **fresh stable line, evidence-refresh line, correction/backport line,
bundle-currentness line, and lts-candidate line** — the 1221–1227 implementation lanes resolve their
protection-plan, correction-queue, refresh-policy, claim-downgrade, deferral-backlog, correction-conversion,
bundle-refresh-audit, shipping-line-drift, supported-line defect-ledger, backport-decision-timer,
post-launch correction-report, train-comparison, LTS-readiness-decision, and line-creation-gate truth; this
capstone **certifies** that the shared stable-line operating truth holds on every claimed M5 **supported
line** — stable-line protection, evidence refresh, backlog conversion, bundle currentness,
correction/backport servicing, and LTS readiness — and auto-narrows any profile that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_stable_line_surface_certification/`
- **Schema:** `schemas/release/m5-stable-line-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-stable-line-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/release/m5-stable-line-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **profile** a release engineer, release operator, program-governance owner,
or support engineer reads a support-window, refresh-state, correction-posture, bundle-currentness, LTS-readiness,
evidence-snapshot, or rollback-stop surface through, not on the underlying line it renders:

1. **Live supported-line operating lane** — a live, first-party supported line whose current refresh and
   correction evidence, bundle-currentness audit, and explicit LTS-readiness decision converge on one preserved
   evidence snapshot and named correction-owner roster. The **only** profile that may certify a
   `certified_operating_line` claim.
2. **Reviewable stable-line structure** — a self-sufficient, inspectable support-window / refresh-state /
   defect-ledger projection; certifies at most `reviewable_stable_line_surface`.
3. **Disclosed correction-ownership profile** — a correction/backport line whose correction ownership and
   backport decision can only be partially disclosed; auto-narrows to
   `correction_ownership_disclosed_projection`.
4. **Unverified bundle-currentness profile** — a bundle-currentness line whose launch-bundle freshness and
   reversibility audit has aged out; auto-narrows to `bundle_currentness_unverified_projection`.
5. **Unverified LTS-readiness profile** — an lts-candidate line whose LTS-readiness decision packet is missing
   its current rollback/support evidence or whose evidence snapshot has aged out; auto-narrows to
   `lts_readiness_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and stable-line-component-truth behavior — and
resolves to a derived verdict:

- **green** — every axis certified, every invariant held, the claimed operating tier delivered;
- **yellow** — a truth axis is not current, so the operating claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh certified claim, a hard invariant breaks,
  CLI/export parity drops, a non-live profile claims a certified operating line, or the narrowing is
  inconsistent.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a `certified_operating_line`
   / `reviewable_stable_line_surface` claim while one of its truth axes is not current over-claims and blocks.
2. **Only a live first-party supported-line operating lane may certify a certified operating line.** Every other
   profile is at most a reviewable stable-line structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the
   support-window truth, refresh state, correction ownership, backport decision, bundle-currentness audit,
   LTS-readiness decision, preserved evidence snapshot, named correction-owner roster, rollback-stop rule, and
   registry reference as text / JSON / Markdown.
4. **Every B146 hard invariant holds per row.** No profile may widen support language without current refresh and
   correction evidence, drift a shipping line on stale evidence or frozen launch bundles, rely on tribal backport
   memory instead of a documented correction packet, claim LTS eligibility without current rollback and support
   evidence, or leave a supported-line defect unowned or unresolved past its SLA.
5. **One canonical proof bundle.** Every row cites exactly one canonical stable-line proof bundle
   (`artifacts/release/m5-stable-line-correction-reports/support_export.json`) — the frozen stable-line-protection
   matrix proof — so release, help, support, and public-proof surfaces consume a single stable-line certification
   source rather than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_STABLE_LINE_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_stable_line_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
