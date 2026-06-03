# Stabilize the release center, promotion evidence, canary→pilot→broad ring control, and rollback-stop rules

## Purpose

This document is the human-readable companion to the ring promotion control
artifact that governs how every promotion subject — binary artifact graph or
non-binary AI/provider/model/prompt/tool pack — moves through the
internal→canary→pilot→broad ring ladder, what evidence is required before it
widens, and what happens when proof ages out or a regression appears mid-soak.

## Companion artifacts

- [`/artifacts/release/stabilize_the_release_center_promotion_evidence_canary_pilot.json`](../../artifacts/release/stabilize_the_release_center_promotion_evidence_canary_pilot.json)
- [`/schemas/release/stabilize_the_release_center_promotion_evidence_canary_pilot.schema.json`](../../schemas/release/stabilize_the_release_center_promotion_evidence_canary_pilot.schema.json)
- [`/artifacts/release/m4/stabilize_the_release_center_promotion_evidence_canary_pilot_proof_packet.md`](../../artifacts/release/m4/stabilize_the_release_center_promotion_evidence_canary_pilot_proof_packet.md)
- [`/fixtures/release/m4/stabilize_the_release_center_promotion_evidence_canary_pilot/`](../../fixtures/release/m4/stabilize_the_release_center_promotion_evidence_canary_pilot/)

## Model

- **Promotion subject** — anything that can be promoted through rings. Subjects
  are either binary (`binary_artifact_graph`) or non-binary
  (`ai_provider_pack`, `ai_model_pack`, `ai_prompt_pack`, `ai_tool_pack`).
- **Ring** — `internal`, `canary`, `pilot`, `broad`. Lanes widen in order; a
  subject does not skip a ring unless shiproom explicitly narrows scope first.
- **Soak window** — per-transition requirement that defines minimum duration,
  required evidence refs, and required fitness checks. A subject may not widen
  until its soak window is complete.
- **Rollback-stop trigger** — per-subject, per-ring-scope condition that may
  fire mid-soak or in the current ring. When fired, it blocks widening and may
  trigger automatic rollback.
- **Kill-switch posture** — `disabled`, `armed`, or `enabled`. An armed kill
  switch engages when a rollback-stop trigger fires; an enabled kill switch
  actively blocks widening.
- **Effective label** — the lifecycle label the subject actually publishes after
  narrowing. It is never wider than the claim label from the stable claim
  manifest.

## Promotion states

| State | Meaning |
|---|---|
| `qualified` | Soak complete, evidence current, owner signed off. |
| `soaking` | In soak window; widening blocked until completion. |
| `provisional_on_waiver` | Held on an active waiver; may still block promotion if waiver expires. |
| `blocked` | Active gap prevents widening; stays in current ring. |
| `narrowed_stale` | Proof packet breached freshness SLO; effective label narrowed. |
| `narrowed_missing` | No proof packet captured; effective label narrowed. |
| `narrowed_regressed` | Regression detected; effective label narrowed. |
| `narrowed_waiver_expired` | Waiver expired; effective label narrowed. |
| `rolled_back` | Explicitly rolled back to rollback target ring. |

## Ring-specific rules

### Canary

- Enter or widen only when owner coverage is current, crash/incident visibility
  is present, an explicit rollback path is named, and issue capture is
  exact-build aware.
- Minimum soak: at least 24 hours of protected-path smoke tests.
- Default rollback-stop: crash, data-loss, or trust regression triggers
  auto-rollback to `internal`.

### Pilot

- Enter or widen only when the current compatibility row is fresh, the support
  or export path is viable, known limits are published, and a named incident
  path exists.
- Minimum soak: at least 72 hours across claimed profiles.
- Default rollback-stop: partner-blocking trust, recovery, or compatibility
  failure.

### Broad

- Enter or widen only when all matrix rows are current, mixed-version and
  rollback drills are current, docs/help language matches current claim rows,
  and no unreviewed red-risk item remains.
- Minimum soak: at least 168 hours (one week) across the full surface set.
- Default rollback-stop: any regression detected in pilot that was not cleared
  before broad widening.

## Downgrade behavior

When a subject's proof packet ages out, its soak window remains incomplete, its
waiver expires, a regression trigger fires, or its kill switch engages:

1. The `promotion_state` moves to the corresponding narrowed or blocked state.
2. The `effective_label` drops below the claim label.
3. The active gap reason is recorded.
4. Shiproom and release tooling fail promotion for the affected gate.
5. Public-facing surfaces (docs, Help/About, release notes) render the
   narrowed label instead of the claim label.

## Non-binary promotion subjects

AI/provider/model/prompt/tool packs are promoted independently of the binary
artifact graph. Each pack has its own ring state, rollback lever, kill-switch
posture, and linked verification packet. Rolling back or hard-disabling an
AI/provider pack is visible as an independent promotion event and does not
require inference from binary release notes alone.

## Shiproom export

The `support_export_projection()` function produces a redaction-safe view that
reconstructs the exact binary plus non-binary surface set promoted to any
cohort, including:

- current ring and target ring
- promotion state and effective label
- soak completion status for the target transition
- kill-switch posture and rollback target ring
- active gap reasons

## Verification

```bash
cargo test -p aureline-release
```

The test suite verifies that:
- the embedded artifact parses and validates with zero violations
- every subject kind is present
- every gap reason has a rule
- summary counts match the rows
- publication decision matches the computed decision
- no row renders wider than its claim ceiling
