# M5 critical-journey checkpoints: visible milestone surfaces for warm startup, large-repo open, AI multi-file apply, remote attach-and-run, and collaboration join-follow

Generated from the seeded packet in
[`crate::m5_critical_journey_checkpoints`](../../crates/aureline-shell/src/m5_critical_journey_checkpoints/mod.rs).
Regenerate with:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_critical_journey_checkpoints -- markdown > \
  artifacts/lifecycle/m5-critical-journey-checkpoints.md
```

- Packet id: `m5-critical-journey-checkpoints:stable:0001`
- Source schema ref: `schemas/lifecycle/m5-critical-journey-checkpoints.schema.json`
- Certifies matrix packet: `m5-lifecycle-matrix:stable:0001`
- Exact build: `build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2`
- Release channel: `stable`
- Required checkpoint dimensions: `checkpoint_visibility`, `partial_truth_labeling`, `place_continuity`, `capture_parity`
- Protected journeys certified: 5
- Green (full visibility): 2
- Yellow (auto-narrowed): 3
- Red (blocked): 0
- All rows publishable: `true`
- Blocking findings: 0
- Status: **clean**
- Generated at: `2026-06-30T00:00:00Z`

## Certification rows

| Journey | Status | Checkpoints | Visibility | Partial truth | Place | Capture | Headless | Waiver |
| ------- | ------ | ----------- | ---------- | ------------- | ----- | ------- | -------- | ------ |
| Warm startup | `green` | preparing → warming → restoring → ready | `named_milestones_replace_spinner` | `partial_state_labeled_and_attributed` | `place_and_next_action_preserved` | `checkpoints_captured_in_export_and_screenshot` | `true` | — |
| Large-repo open | `yellow` | preparing → warming → building → ready | `named_milestones_replace_spinner` | `disclosed_coarse_partial_label` | `place_and_next_action_preserved` | `checkpoints_captured_in_export_and_screenshot` | `true` | — |
| AI multi-file apply | `green` | preparing → authorizing → building → verifying → ready | `named_milestones_replace_spinner` | `partial_state_labeled_and_attributed` | `place_and_next_action_preserved` | `checkpoints_captured_in_export_and_screenshot` | `true` | — |
| Remote attach-and-run | `yellow` | authorizing → connecting → warming → ready | `disclosed_compacted_milestones` | `partial_state_labeled_and_attributed` | `place_and_next_action_preserved` | `checkpoints_captured_in_export_and_screenshot` | `true` | — |
| Collaboration join-follow | `yellow` | queued → authorizing → warming → verifying → ready | `named_milestones_replace_spinner` | `partial_state_labeled_and_attributed` | `disclosed_reduced_next_action` | `checkpoints_captured_in_export_and_screenshot` | `true` | `waiver:collaboration-reduced-next-action:0001` |

## Auto-narrowed rows

- `large_repo_open` (`yellow`) — While a large repository opens, the journey shows a disclosed coarse partial-truth label — the partial tree and warm search fallback are labeled at the container grain rather than per-file while indexing progresses — while still naming each milestone and attributing the partial state to indexing, so the large-repo-open journey is narrowed and disclosed rather than leaving the partial state unlabeled.
- `remote_attach_run` (`yellow`) — On a compact remote status strip the attach-and-run journey presents its auth/policy, environment-probe, sync-warming, and task-stream milestones in a disclosed compacted form while still naming each milestone individually, so the remote journey is narrowed and disclosed rather than collapsing its milestones into an anonymous spinner.
- `collaboration_join_follow` (`yellow`) — When a collaboration join-follow session loses its shared connection mid-follow, the journey keeps the user's place in the checkpoint sequence and a disclosed, waivered reduced next-safe-action — the rejoin affordance is offered immediately while control transfer is deferred until reconnect — so the collaboration journey is narrowed and disclosed rather than dropping the user onto a generic shell.

## Exact journey causes

- `large_repo_open` — `upstream_dependency_narrowed` (disclosed: `true`) — The journey shows a disclosed coarse partial-truth label — naming a stage group rather than the exact sub-step — while still labeling and attributing the partial state, so the partial truth is narrowed and disclosed rather than unlabeled.
- `remote_attach_run` — `upstream_dependency_narrowed` (disclosed: `true`) — The journey presents its milestone checkpoints in a disclosed compacted form on a compact surface while still naming each milestone individually, so the checkpoint sequence is narrowed and disclosed rather than collapsing into an anonymous spinner.
- `collaboration_join_follow` — `upstream_dependency_narrowed` (disclosed: `true`) — The journey keeps a disclosed, waivered reduced next-safe-action — for example deferring one recovery path until a dependency resolves — while still keeping the user's place and a safe action, so the affordance is narrowed and disclosed rather than lost.

## Active waivers

- `waiver:collaboration-reduced-next-action:0001` (`collaboration_join_follow`, owner: Collaboration owner, expires `2026-09-30T00:00:00Z`) — When a collaboration join-follow session loses its shared connection mid-follow, the journey keeps the user's place in the checkpoint sequence and a disclosed, still-safe reduced next-safe-action: the rejoin affordance is offered immediately while the control-transfer request is deferred until the session reconnects, rather than dropping the user onto a generic shell. The reduced next-safe-action is disclosed, never silent, and the full affordance set is restored the moment the collaboration lane rejoins.

## Findings

Findings: none.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_m5_critical_journey_checkpoints -- validate
cargo test -p aureline-shell --test m5_critical_journey_checkpoints_fixtures
```
