# M5 launch-control surface certification (M05-1219)

This contract is the **closing B145 surface-certification capstone** over the frozen M5 launch-control
matrix (`m5_launch_control_matrix`). Where the freeze matrix defines the five governed launch-bearing cohorts —
**core-team canary, design-partner preview, extension-author, public preview, and certified archetype** — the
1213–1218 implementation lanes resolve their cohort-descriptor, cohort-evidence-packet, ring-progression,
rollback-stop, regression-asset, incident-close, freeze-exception, go/no-go, ORR-review, rehearsal-drill,
widening-decision, and ring-history truth, this capstone **certifies** that the shared launch-control truth
holds on every claimed M5 **launch-bearing widening profile** — cohort graduation, ring soak,
incident-regression assets, intake/freeze-exception gating, rehearsal freshness, and explicit go/no-go
records — and auto-narrows any profile that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_launch_control_surface_certification/`
- **Schema:** `schemas/release/m5-launch-control-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-launch-control-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/release/m5-launch-control-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **profile** a release engineer, shiproom operator, program-governance owner,
or support engineer reads a cohort-membership, readiness-event, rehearsal-currency, freeze-exception, go/no-go,
evidence-snapshot, or rollback-stop surface through, not on the underlying cohort it renders:

1. **Live certified widening lane** — a live, first-party certified-archetype lane whose current cohort and
   rehearsal evidence, signed ORR, and explicit stable/LTS go/no-go decision converge on one preserved evidence
   snapshot and named on-call/signoff roster. The **only** profile that may certify a `certified_widening_lane`
   claim.
2. **Reviewable launch-control structure** — a self-sufficient, inspectable cohort descriptor / readiness state /
   ring-history projection; certifies at most `reviewable_launch_control_surface`.
3. **Disclosed freeze-exception profile** — an extension-author lane whose freeze-exception scope can only be
   partially disclosed; auto-narrows to `freeze_exception_disclosed_projection`.
4. **Unverified rehearsal-currency profile** — a public-preview lane whose publish/rollback, mixed-version,
   advisory/revocation, and support-handoff rehearsal drills have aged out; auto-narrows to
   `rehearsal_currency_unverified_projection`.
5. **Unverified regression-asset profile** — a design-partner-preview lane whose closed Sev-1/Sev-2 incident is
   missing its linked regression asset or whose go/no-go evidence snapshot has aged out; auto-narrows to
   `go_no_go_evidence_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and launch-control-component-truth behavior — and
resolves to a derived verdict:

- **green** — every axis certified, every invariant held, the claimed widening tier delivered;
- **yellow** — a truth axis is not current, so the widening claim auto-narrows to the weakest supported
  ceiling with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh certified claim, a hard invariant breaks,
  CLI/export parity drops, a non-live profile claims a certified widening lane, or the narrowing is
  inconsistent.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a `certified_widening_lane`
   / `reviewable_launch_control_surface` claim while one of its truth axes is not current over-claims and blocks.
2. **Only a live first-party certified-archetype lane may certify a certified widening lane.** Every other
   profile is at most a reviewable launch-control structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the cohort
   membership, readiness event, rehearsal currency, freeze-exception authority, go/no-go decision, preserved
   evidence snapshot, named on-call/signoff roster, rollback-stop rule, and registry reference as
   text / JSON / Markdown.
4. **Every B145 hard invariant holds per row.** No profile may widen a stable claim without current cohort and
   rehearsal evidence, let a freeze exception become undocumented scope widening, close a Sev-1/Sev-2 incident
   without a regression asset, imply green when go/no-go records or ORR packets are stale, or maintain partner
   or public support language that outruns current cohort proof.
5. **One canonical proof bundle.** Every row cites exactly one canonical launch-control proof bundle
   (`artifacts/release/m5-orr-rehearsal-proof/support_export.json`) — the frozen launch-control matrix proof —
   so shiproom, release, docs, support, and public-proof surfaces consume a single launch-control certification
   source rather than hand-authored prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_LAUNCH_CONTROL_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_launch_control_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
