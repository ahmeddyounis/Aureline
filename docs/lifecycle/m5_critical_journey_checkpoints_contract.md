# M5 critical-journey checkpoints contract

This lane is the **critical-journey checkpoint capstone** on top of the frozen
[M5 lifecycle-state and journey-checkpoint matrix](m5_lifecycle_matrix_contract.md). The matrix
freezes, for every long-lived M5 object family, an explicit state machine and an inventory of
protected journeys that must show named milestone checkpoints instead of an anonymous spinner. This
lane materializes **visible checkpoint surfaces for the five highest-value M5 journeys** the spec
protects by name, and certifies that each one exposes visible milestones instead of one anonymous
monolithic spinner, keeps any partial or degraded behavior labeled and attributable, keeps the
user's place and a next-safe-action, and preserves its checkpoint truths through export, screenshot,
and support-packet capture — and that the same state-truth vocabulary survives a headless or
companion-adjacent execution.

The lane exists so that M5 can honestly ship its growing mix of notebook, data/API, AI, remote,
preview, operator, docs, and release surfaces without its highest-value journeys hiding a half-ready
or maybe-applied state behind one anonymous spinner, and without support and screenshots being
unable to reproduce the checkpoint truth the user saw live.

## Protected journeys

The certification covers exactly the five protected journeys the spec names, and refuses to ship if
any is missing:

- `warm_startup` — Warm startup: skeleton shell → command system ready → session restore note →
  first interactive editor. Drives the `workspace` object family; binds the matrix
  `workspace_restore` journey.
- `large_repo_open` — Large-repo open: partial tree → warm search fallback → indexing progress →
  first jump confidence note. Drives the `workspace` object family; has no frozen matrix journey and
  anchors on the workspace object family directly.
- `ai_multi_file_apply` — AI multi-file apply: context resolving → approval requirement → reviewable
  patch → verification result → rollback handle. Drives the `ai_action` object family; binds the
  matrix `ai_action_run` journey.
- `remote_attach_run` — Remote attach-and-run: auth/policy stage → environment probe → sync warming
  → structured task stream. Drives the `remote_session` object family; binds the matrix
  `remote_reconnect` journey.
- `collaboration_join_follow` — Collaboration join-follow: publish/join → role assignment → follow
  state → control transfer visibility → archived outcome. Drives the `collaboration_session` object
  family; binds the matrix `collaboration_join` journey.

Every attribute a row certifies over — the driving object family, its explicit state machine (the
admitted controlled states, always including `ready`), the named recovery affordance the
next-safe-action anchors on, the controlled last-failure reason classes, the declared consumer
surfaces, and the applicable downgrade triggers — is pulled straight from the frozen matrix's seeded
packet, so this lane mints no parallel lifecycle vocabulary and cannot certify a journey the matrix
does not anchor. Only the ordered milestone checkpoint sequence each journey shows is authored here,
drawn from the frozen journey-checkpoint vocabulary (`queued`, `authorizing`, `preparing`,
`connecting`, `restoring`, `building`, `warming`, `verifying`, `finalizing`, `ready`,
`partial_ready`, `recoverable_failure`).

## Certified checkpoint dimensions

Each row is certified across the four checkpoint dimensions the acceptance criteria require every
protected journey to hold (`checkpoint_visibility`, `partial_truth_labeling`, `place_continuity`,
`capture_parity`):

- **checkpoint visibility** — `named_milestones_replace_spinner` (green: the journey shows its
  ordered, named milestone checkpoints in place of a single opaque progress indicator), a disclosed
  `disclosed_compacted_milestones` where the milestones are presented in a compacted form on a
  compact surface while each is still named (yellow), or `anonymous_spinner_shown` (red: the journey
  fell back to one anonymous monolithic spinner, hiding its milestone boundaries).
- **partial-truth labeling** — `partial_state_labeled_and_attributed` (green: a partial-ready or
  degraded milestone shows a controlled label naming what is and is not ready and attributes the
  partial truth to a named cause), a disclosed `disclosed_coarse_partial_label` where the partial
  state is labeled at a coarse grain — e.g. a stage group rather than the exact sub-step — while
  still labeled and attributed (yellow), or `partial_state_unlabeled_or_unattributed` (red: a
  partial or degraded milestone went unlabeled or unattributed).
- **place continuity** — `place_and_next_action_preserved` (green: at every checkpoint, including a
  recoverable failure, the journey keeps the user oriented at their place and offers a named
  next-safe-action), a disclosed `disclosed_reduced_next_action` where a reduced next-safe-action is
  kept — e.g. deferring one recovery path until a dependency resolves — while still keeping the
  user's place and a safe action (yellow, **requires an active waiver**), or `place_or_recovery_lost`
  (red: the journey lost the user's place or its named recovery affordance, dropping them onto a
  generic shell with no next-safe-action).
- **capture parity** — `checkpoints_captured_in_export_and_screenshot` (green: the same named
  milestones, partial-truth labels, and next-safe-actions the user sees live are captured in a
  screenshot, a support packet, and an export), a disclosed `disclosed_partial_capture` where a
  reduced subset of checkpoint detail is captured while the milestone boundaries and terminal are
  still captured (yellow), or `checkpoints_absent_from_capture` (red: the journey's checkpoints did
  not survive export/screenshot/support capture).

A `headless_parity_preserved` flag records that the same state-truth vocabulary survives a headless
or companion-adjacent execution; losing it is a hard blocker. A malformed checkpoint sequence (fewer
than two milestones, a repeated milestone, or no terminal) is likewise a hard blocker — it cannot
prove the journey shows named milestones rather than an anonymous spinner.

## Auto-narrowing and completeness

Each row's green/yellow/red status is **derived**, never asserted. Any hard blocker — an anonymous
spinner, an unlabeled partial state, a lost place or recovery affordance, checkpoints absent from
capture, a headless/companion-adjacent vocabulary loss, a malformed checkpoint sequence, or a row
that did not certify every consumer surface the matrix declares for the journey's driving object
family — forces `red`; any disclosed narrowing forces `yellow`; otherwise `green`. A disclosed
reduced next-safe-action must carry an active waiver to stay publishable, and every non-green row
must disclose a reason. The consumer-surface and checkpoint-sequence completeness checks are the
lints that keep a certification from regressing into a partial view that would let a protected flow
hide a half-ready or maybe-applied state behind one generic spinner on the surfaces it did not
certify.

The seeded certification is **2 green** (warm startup, AI multi-file apply) and **3 yellow**
(large-repo open disclosing a coarse partial-truth label, remote attach-and-run disclosing compacted
milestones, and collaboration join-follow with a waivered reduced next-safe-action), with **0 red**.
Five protected blocked fixtures prove the red path for each acceptance-criteria failure mode: warm
startup falling back to an anonymous spinner, large-repo open leaving its partial state unlabeled,
collaboration join-follow losing the user's place, AI multi-file apply dropping its checkpoints from
capture, and remote attach-and-run losing headless parity.

## Artifacts

- Schema: `schemas/lifecycle/m5-critical-journey-checkpoints.schema.json`
- Report: `artifacts/lifecycle/m5-critical-journey-checkpoints.md`
- Proof packet: `artifacts/release/m5-critical-journey-checkpoints-proof/packet.json`
- Proof dashboard: `artifacts/release/m5-critical-journey-checkpoints-proof/dashboard.json`
- Proof support export: `artifacts/release/m5-critical-journey-checkpoints-proof/support_export.json`
- Proof CSV: `artifacts/release/m5-critical-journey-checkpoints-proof/matrix.csv`
- Fixtures: `fixtures/state/m5-critical-journey-checkpoints/packet.json` (and `dashboard.json`,
  `support_export.json`, `compact.txt`)

The Rust validator `validate_m5_critical_journey_checkpoints_packet` in
`crates/aureline-shell/src/m5_critical_journey_checkpoints/` is the authoritative gate; the schema
above documents the shape. The headless emitter
`aureline_shell_m5_critical_journey_checkpoints` is the only mint-from-truth path.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_critical_journey_checkpoints -- validate
cargo test -p aureline-shell --test m5_critical_journey_checkpoints_fixtures
cargo test -p aureline-shell --lib m5_critical_journey_checkpoints
```
