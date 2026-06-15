# M5 evidence pointer — runtime inspector cards

Reviewer contract for the canonical M5 runtime inspector cards that give an author or
operator the live runtime surface for each claimed M5 ecosystem artifact family:
activation time, current host, granted capabilities, recent logs, recent failures,
hot-reload posture, rendered trust tier, last-known-good state, and the
quarantine/disable/re-enable actions. This row is a depth-lane proof governed by the
canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).

## Canonical artifacts

- Truth packet: `artifacts/ecosystem/m5/m5-runtime-inspector.json`
- Boundary schema: `schemas/ecosystem/m5-runtime-inspector.schema.json`
- Reviewer contract: `docs/m5/implement-runtime-inspectors-with-activation-time-host-granted-capabilities-logs-failures-and-quarantine-or-disable-actions.md`
- Human-readable rendering: `artifacts/m5/implement-runtime-inspectors-with-activation-time-host-granted-capabilities-logs-failures-and-quarantine-or-disable-actions.md`
- Overview companion: `docs/ecosystem/m5/m5-runtime-inspector.md`
- Fixture corpus: `fixtures/ecosystem/m5/m5-runtime-inspector/`
- Owning crate module: `crates/aureline-ecosystem/src/m5_runtime_inspector/`

## Reuses the shared M5 vocabulary

The runtime inspector is the live counterpart to the install and sideload review lanes.
It reuses the closed artifact-family, source-class, runtime-class, host/ABI,
signing-state, trust-posture, anti-abuse, and hot-reload vocabulary already frozen by the
install-governance matrix and the publish-preview gate
(`artifacts/ecosystem/m5/m5-author-and-publish-preview.json`), plus the activation-budget
capability and resource vocabulary (`artifacts/ecosystem/m5/m5-activation-budget.json`),
rather than minting a parallel set — so a review sheet, an activation report, and a
runtime inspector describe the same artifact with the same words.

## What the card proves

- **Live runtime truth, not extension-manager metadata.** Each card carries the measured
  activation time and memory, the current host, the granted capabilities with
  declared-versus-exercised state, recent redacted logs, and recent failures — not just
  listing metadata.
- **No local or untrusted package inherits a trusted badge.** The rendered trust tier is
  capped by the signing state, so an unsigned local-dev, side-loaded, or revoked
  artifact renders `unsigned_local_only` even when built on a machine that holds a
  trusted key. A genuinely `signed_verified` package still renders its real badge, so
  trust reflects provenance rather than blanketing every card to local-only.
- **A widening hot reload forces a fresh review.** A hot reload that widens the runtime
  class, expands permissions, or adds an external executable — or an undeclared exercised
  capability — recomputes to `fresh_review_required`, and the restart and reload actions
  are held until a fresh review clears it, so widening cannot apply through a silent hot
  reload.
- **The inspector stays useful when failing or quarantined.** A `load_failed` or
  `source_missing` card keeps its last-known-good revision, runtime, host, and badge
  visible, and that last-good badge can never exceed the family's current cap. A disabled
  or quarantined card still exposes its logs, crash history, and granted capabilities and
  offers a review-routed re-enable.
- **Crash history and capabilities are never hidden.** Logs are always available, and
  over-grants (`declared_unused`) and policy violations (`undeclared_exercised`) stay on
  the card.
- **Records are export-safe.** Every field is a typed state, a redacted label, or an
  opaque ref — no absolute paths, raw log bodies, supervisor traces, signing secrets, or
  payloads.

## Executable proof

`crates/aureline-ecosystem/src/m5_runtime_inspector/tests.rs` loads the embedded packet,
asserts it validates with zero violations, proves every load state and disposition is
exercised, asserts the non-inheritance, fresh-review-on-widening,
useful-when-failing/last-known-good, and nothing-hidden guardrails, and checks the export
projection. `M5RuntimeInspector::validate()` is the CI-facing gate that flags any
overstated rendered badge, inherited trust, dropped last-known-good state,
above-cap last-good badge, silently enabled widening action, hidden review trigger,
inconsistent signature, or summary drift.
