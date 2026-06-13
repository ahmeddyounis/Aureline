# Host Lane And Reattach Support Contract

This contract defines the support-facing packet for runtime host-lane identity,
fault-domain state, restart budgets, crash-loop or quarantine posture, and
reattach review.

## Runtime Records

- `host_lane_record` names the plain-language host family, lane role, health,
  fault-domain id, restart-budget ref, affected capabilities, preserved
  checkpoints, and stale result refs.
- `topology_inspector_record` maps visible results back to host lanes and
  carries inline host badge groups for logs, runs, debug views, notebook
  outputs, previews, profiler/replay views, data/API sessions, AI tool
  actions, provider-backed summaries, and pipeline viewers.
- `fault_domain_restart_card_record` exposes strike count, restart-budget
  state, affected capabilities, preserved checkpoints, and next-safe actions.
- `reattach_review_sheet_record` compares previous and current host identity,
  preserved state, lost state, replay risk, rerun requirements, policy drift,
  and auth drift.
- `crash_loop_quarantine_banner_record` prevents disconnected, quarantined,
  disabled, or crash-looping lanes from rendering as healthy.
- `lane_filtered_event_viewer_record` preserves event ids, timestamps, lane
  refs, restart markers, and run or diagnostic provenance refs.

## Support Packet

`fault_domain_view_packet` is metadata-only. It includes one row per host lane,
one restart card per lane, active reattach reviews, crash-loop or quarantine
banners, and the lane-filtered event viewer used to reconstruct the visible
timeline. Raw command lines, paths, environment bodies, payloads, and secrets do
not cross this boundary.

## Required Surface Truth

Every claimed runtime-heavy surface that can affect trust must show a host badge
group or a linked topology detail action:

- logs
- runs
- debug views
- notebook outputs
- previews
- profiler/replay views
- data/API sessions
- AI tool actions
- provider-backed runtime summaries
- pipeline viewers

Non-mutating analysis restarts can continue with stale labels and progressive
refresh. Mutating lanes, debug disconnects, notebook-kernel restarts, and remote
identity drift require review, explicit rerun, or reapproval before the lane is
treated as current again.

Visible labels must remain explicit in export packets as well as the shell:

- `stale`
- `rebuilding`
- `provider unavailable`
- `reconnecting`
- `local fallback`
- `captured snapshot`
