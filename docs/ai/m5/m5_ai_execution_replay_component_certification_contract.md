# M5 AI-Execution/Replay Component Surface Certification (M05-883)

This is the closing capstone of the B103 AI-execution/replay component lane. Where the
freeze matrix
(`freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix`)
froze the eight reusable AI-execution/replay components, the M05-877..881 primitive lanes
narrowed each one, and the M05-882 consumer lane proved they are reusable across the
claimed AI consumers, this lane **certifies** that the shared component truth holds on
every claimed M5 AI surface — and auto-narrows any surface that cannot sustain it.

- **Module:** `crates/aureline-ai/src/certify_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_truth_on_every_claimed_m5_ai_surface`
- **Schema:** `schemas/ai/m5-ai-execution-replay-component-certification.schema.json`
- **Support export (canonical):** `artifacts/ai/m5/m5-ai-execution-replay-component-certification/support_export.json`
- **Matrix CSV / report:** same directory (`matrix.csv`, `report.md`)
- **Fixtures:** `fixtures/ai/m5/m5-ai-execution-replay-component-certification/`

## What is certified

The packet is keyed on the claimed **surface** a user reviews, reruns, pauses, resumes,
exports, or hands off AI work through — **not** on component family or primitive lane. The
eight certified surfaces are:

`inline_assistant`, `assistant_panel`, `patch_review`, `test_generation`,
`branch_worktree_queue`, `help_console`, `cli_headless`, `support_export`.

Each surface is scored across six truth axes:

1. `visual` — mode, action state, route/provider/model, tool boundary, auth posture,
   approval gate, and replay completeness shown on the primary surface.
2. `keyboard` — the same truth and its controls are reachable without a pointer.
3. `screen_reader` — the same truth is announced non-visually, never color/glyph-only.
4. `cli_export` — **always-on**; the surface state is reconstructable as text / JSON /
   Markdown from the same run identity. This axis must stay certified on every row.
5. `degraded_state` — a cached/buffered read, an unreachable provider, or a stale proof
   honestly downgrades a `live_governed_execution` / `complete_replay` claim.
6. `execution_boundary_and_replay_provenance` — route/provider/model, tool boundary, auth
   posture, approval gate, checkpoint lineage, replay completeness, drift reason, and
   manual-takeover path stay explicit, never inheriting a healthier surface's provenance.

## Support-claim ladder

Every surface asserts a `claimed_claim` and is certified down to a `certified_claim` on
this six-tier ladder (strongest first): `live_governed_execution` (5) → `complete_replay`
(4) → `route_adjacent_replay` (3) → `cached_evidence` (2) → `unverified_agent_state` (1) →
`policy_blocked_execution` (0). Certification may only ever *narrow* a claim (lower its
rank), never raise it.

## The invariant

**A degraded truth axis must produce a visible claim narrowing.** The derived status is
recomputed, never asserted:

- **green** — every axis certified and the claimed tier is delivered.
- **yellow** — an axis is not current, the claim narrows to the weakest supported tier,
  and the reduction binds to that axis with a non-generic label and a frozen downgrade
  trigger.
- **red** — a degraded axis is hidden behind a full claim (undisclosed drift), the
  always-on `cli_export` axis drops, a claim is strengthened, or the narrowing is
  inconsistent. A red surface blocks the release.

Compatibility notes are captured for the paths the spec names — provider/model
unavailability, connector policy blocks, replay incompleteness, stale approvals, and
manual-takeover paths — and each yellow surface auto-narrows visibly rather than
inheriting a stronger label from a healthier AI lane.

## Canonical bundle

Every row cites exactly one canonical proof bundle — the frozen AI-execution component
matrix release proof
(`artifacts/ai/m5/freeze_the_m5_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_matrix/support_export.json`) —
plus the M05-882 consumer-adoption support export as supporting evidence. The packet is
metadata-only; raw prompts, provider tokens, connector credentials, and replay bodies
never cross this boundary.

## Seeded certification

Four surfaces deliver their claim (green): `inline_assistant`, `assistant_panel`,
`patch_review`, `support_export`. Four auto-narrow a not-current axis (yellow):

- `test_generation` — incomplete rerun replay → `unverified_agent_state`
  (`replay_completeness_overstated`).
- `branch_worktree_queue` — interrupted agent, incomplete checkpoint lineage →
  `unverified_agent_state` (`checkpoint_lineage_broken`).
- `help_console` — policy-blocked connector → `policy_blocked_execution`
  (`auth_posture_masked`).
- `cli_headless` — unreachable provider serving cached evidence → `cached_evidence`
  (`route_or_provider_masked`, degraded-state axis).

No surface hides drift (0 red). All eight frozen component families are certified on at
least one surface.

## Regenerating the artifacts

```
GEN_AI_CERT_ARTIFACTS=1 cargo test -p aureline-ai \
  certify_ai_action_state_banner_connector_detail_row_local_model_pack_card_approval_sheet_tool_call_timeline_row_run_history_row_replay_review_and_agent_status_component_truth_on_every_claimed_m5_ai_surface::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the checked-in support export
drifts from the seeded builder.
