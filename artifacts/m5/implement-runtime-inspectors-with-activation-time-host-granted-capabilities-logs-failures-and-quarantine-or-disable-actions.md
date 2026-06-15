# M5 runtime inspector cards — human-readable rendering

Human-readable rendering of the canonical M5 runtime inspector cards. This row is a
depth-lane proof governed by the canonical M5 evidence index
(`docs/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.md`).
The machine-readable truth is at `artifacts/ecosystem/m5/m5-runtime-inspector.json`.

## Per-family runtime inspector card

| Family | Load state | Activation | Current host | Signing → rendered tier | Capabilities | Failures | Disposition |
| --- | --- | --- | --- | --- | --- | --- | --- |
| first_party_framework_pack | loaded_healthy | cold 210ms | managed_host | signed_verified → **enterprise_approved** | 1 used, 1 unused | — | running_healthy |
| docs_pack | loaded_healthy | warm 8ms | no_code_execution | signed_unverified → registry_bound | 1 used | — | running_healthy |
| local_model_pack | loaded_degraded | warm 920ms (mem elevated) | external_process | unsigned_sideload → **unsigned_local_only** | 3 used | host_disconnect ×2 | running_degraded |
| signed_recipe_pack | loaded_healthy | cold 175ms | managed_host | signed_verified → verified_publisher | 2 used | — | **fresh_review_required** |
| template_artifact | source_missing | — | browser_runtime | signed_unverified → registry_bound | 1 unused | — | **showing_last_known_good** |
| bridge_backed_package | load_failed | — | managed_host | signed_verified → registry_bound | 1 used | crash_on_activation ×1 | **showing_last_known_good** |
| side_loaded_package | operator_disabled | — | remote_target | unsigned_sideload → **unsigned_local_only** | 1 used, 1 undeclared | undeclared_capability_use ×1 | **operator_disabled** |
| mirrored_registry_variant | quarantine_held | — (mem over_budget) | managed_host | unsigned_sideload → **unsigned_local_only** | 1 used | crash_loop ×5 | **quarantined** |

## Non-inheritance, fresh-review holds, and last-known-good

- **framework pack** — genuinely signed and verified, so it renders its real
  `enterprise_approved` badge; the cap reflects provenance rather than blanketing every
  card to local-only.
- **model pack** — unsigned side-load, so it renders `unsigned_local_only` even while
  running; degraded by recent host disconnects, with its last clean activation kept
  visible.
- **recipe pack** — runs healthy, but a pending hot reload widens permissions; it
  recomputes to `fresh_review_required`, and its restart and reload actions are held
  until a fresh review clears it, so the widening cannot apply through a silent hot
  reload.
- **template artifact** — its source path disappeared; the inspector keeps the last good
  render, runtime, host, and `registry_bound` badge visible and offers a reload once the
  source returns.
- **bridge-backed pack** — its adapter crashed on activation, so the current load failed;
  the inspector shows the last successful bridge activation and offers a restart.
- **side-loaded package** — disabled by an operator after exercising an undeclared
  secret-read capability; the crash and capability history stay visible, the
  `undeclared_capability_exercised` trigger is recorded, and re-enabling routes through
  review.
- **mirrored variant** — quarantined under anti-abuse review after a crash loop and an
  over-budget memory profile; logs and crash history stay visible and restart is held.

## Summary

- 8 families, one runtime inspector card each.
- Load states: every one of `loaded_healthy`, `loaded_degraded`, `load_failed`,
  `source_missing`, `quarantine_held`, and `operator_disabled` is exercised.
- Dispositions: 2 `running_healthy`, 1 `running_degraded`, 2 `showing_last_known_good`,
  1 `fresh_review_required`, 1 `operator_disabled`, 1 `quarantined`.
- 3 cards render `unsigned_local_only`; the verified framework pack still renders
  `enterprise_approved`, so trust reflects provenance.
- 5 cards keep a last-known-good state visible; 4 carry recent failures, all visible
  including on the disabled and quarantined cards.
