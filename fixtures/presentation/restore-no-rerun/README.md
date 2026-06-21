# Presentation restore (no-rerun) fixtures

These fixtures are the literal projection of the seeded presentation-restore
corpus in
[`aureline-shell::presentation::presentation_restore`](../../../crates/aureline-shell/src/presentation/presentation_restore/corpus.rs).
They prove that entering presentation mode checkpoints the prior layout and that
exit, cancel, crash recovery, and interrupted resume all replay that checkpoint
at a **classified, visible fidelity** — exact, compatible, layout-only,
evidence-only, or no-restore — and that any target a restore cannot bring back
degrades to an honest placeholder / disconnected state rather than being silently
re-run, re-authorized, or hidden behind a generic success message.

The restore-fidelity vocabulary mirrors the durable-shell restore classes in
[`aureline-recovery::session_restore`](../../../crates/aureline-recovery/src/session_restore/records.rs);
each presentation class and degrade trigger maps onto a canonical durable class /
downgrade trigger, so presentation restore reads exactly like window-session
restore.

## Files

- `restore-report-corpus.json` — the in-product / inspector truth: one case per
  scenario, each carrying a restore report with its restore class, the restored
  layout / focus / panel / accessibility refs, one state per restored waypoint,
  and the surfaced degrade triggers. Each report conforms to
  [`schemas/presentation/restore-report.schema.json`](../../../schemas/presentation/restore-report.schema.json).
- `restore-report-support-export.json` — the support-safe projection: one row per
  report carrying restore class (and its durable-shell mapping), trigger,
  lifecycle, waypoint counts, degrade triggers, and the guardrail booleans only.
  Checkpoint refs, target refs, and placeholder bodies are excluded.

## Cases

- `restore-case:exit-exact` — a solo rehearsal exits cleanly; the layout, focus,
  panels, and accessibility posture come back **exactly** and every waypoint is
  restored read-only.
- `restore-case:crash-compatible` — crash recovery rehydrates the session but the
  window topology no longer maps one-to-one, so it comes back through a
  **compatible** translation; every waypoint stays live.
- `restore-case:resume-layout-only-degraded` — an interrupted resume restores the
  layout, but one waypoint's surface dependency is gone (honest **placeholder**)
  and another's sharing grant was revoked (honest **disconnected**) →
  **layout-only**.
- `restore-case:cancel-disconnected-remote-and-expired` — a cancel restores the
  layout, but a remote target is unreachable and a privileged grant has expired;
  both waypoints degrade to **disconnected** and the expired authority stays
  expired → **layout-only**.
- `restore-case:crash-evidence-only` — crash recovery brings the layout back but
  the live shared walkthrough cannot be rehydrated, so only an **evidence-only**
  record remains; no waypoint is re-run.
- `restore-case:resume-no-restore` — an interrupted resume finds no checkpoint was
  ever captured; nothing is restored, the user keeps their current layout, and the
  resume is honestly reported as **no-restore** rather than a fake success.

## Regenerating

These files are generated, not hand-edited. After changing the restore shape or
the seed corpus, regenerate them so the in-tree test
`checked_in_fixtures_match_the_seed_projection` keeps passing:

```sh
cargo run -q -p aureline-shell --example dump_presentation_restore -- corpus \
  > fixtures/presentation/restore-no-rerun/restore-report-corpus.json
cargo run -q -p aureline-shell --example dump_presentation_restore -- support-export \
  > fixtures/presentation/restore-no-rerun/restore-report-support-export.json
```
