# M5 admission and first-useful-work routing — reviewer artifact

Human-readable companion to the governed packet at
`artifacts/workspace/m5/m5-admission-and-routing.json`. The full contract lives in
`docs/workspace/m5/m5-admission-and-routing.md`; the typed model lives in the
`aureline-workspace` crate (`m5_admission_and_routing`).

This artifact freezes one admission **checkpoint** per M5 wedge. Each checkpoint classifies the
workspace, labels where that classification came from, separates setup into blocking-now,
recommended-soon, and optional-later, and routes the user to first useful work — without a hidden
trust change, a forced wizard, or false archetype certainty.

## Checkpoint roll-up (as of 2026-06-11)

| Checkpoint | Wedge | Class | Source | Confidence | Route | Local-safe | Setup (block/soon/later) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `notebook-certified` | notebook | certified | certified_manifest | certified | guided_setup_offered | yes | 0 / 1 / 1 |
| `data-api-probable` | data/API | probable | heuristic_probe | probable | open_minimal | yes | 0 / 1 / 1 |
| `profiler-missing-prereq` | profiler | missing_prerequisite | missing_toolchain_probe | unknown | local_safe_fallback | yes | 1 / 0 / 0 |
| `framework-pack-certified` | framework-pack | certified | framework_pack_signal | certified | guided_setup_offered | yes | 0 / 1 / 1 |
| `docs-unknown` | docs | unknown | no_signal | unknown | set_up_later | yes | 0 / 0 / 1 |
| `companion-mixed` | companion | mixed | mixed_signals | mixed | open_minimal | yes | 0 / 1 / 1 |
| `sync-handoff-probable` | sync-handoff | probable | framework_pack_signal | probable | set_up_later | yes | 0 / 1 / 1 |
| `local-folder-restricted` | local folder | restricted | policy_restriction | unknown | restricted_browse | **no** | 0 / 1 / 0 |

## What the checkpoints prove

- **Six distinct admission states.** `certified`, `probable`, `mixed`, `unknown`, `restricted`,
  and `missing_prerequisite` each appear and stay distinct. Only the two certified checkpoints
  set `presented_as_certified_support`; probable and mixed never read as certified support.
- **No claim out-ranks its evidence.** Every checkpoint's admission class is permitted under its
  archetype confidence, and every bundle recommendation is permitted under that confidence — a
  probable detection can never present certified support or a certified-archetype bundle match.
- **Setup is separated, not funneled.** Blocking-now (1), recommended-soon (6), and optional-later
  (6) items are all exercised. Open-minimal and set-up-later routes carry equal weight with guided
  setup.
- **Local-safe work survives deferral.** Seven of eight checkpoints keep minimal local-safe work;
  the missing-prerequisite profiler keeps it even behind a blocking runtime gate. Only the
  policy-`restricted` local folder is browse-only — the single state that removes local-safe work.
- **No hidden setup.** All four guardrail flags (`forces_wizard`, `auto_installs_packs`,
  `rewrites_layout_without_review`, `widens_trust_without_review`) are `false` on every
  checkpoint, and all 13 setup items carry `auto_runs: false`. Four items are review-gated.
- **Reconstructable provenance.** Every checkpoint carries a `routing_provenance_ref`, an
  `archetype_evidence_ref`, and a `bundle_recommendation_ref` so diagnostics and support export
  can explain how the workspace was admitted and routed.

## Summary counts

- 8 checkpoints across 8 wedges: 2 certified, 2 probable, 1 mixed, 1 unknown, 1 restricted,
  1 missing-prerequisite.
- 2 present as certified support; 7 keep local-safe work; 1 has a blocking-now item.
- 13 setup items: 1 blocking-now, 6 recommended-soon, 6 optional-later; 4 require review.
