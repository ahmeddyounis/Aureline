# Multi-window, display-topology, and suspend/resume verification seed

This document is the narrative companion to
[`/artifacts/qa/window_display_matrix.yaml`](../../artifacts/qa/window_display_matrix.yaml).
It seeds one canonical QA matrix for the claimed desktop rows that must
preserve layout intent, focus visibility, restore provenance, and
restore-no-rerun truth across monitor changes, mixed DPI,
suspend/resume, and reopen.

If this document and the YAML disagree, the YAML wins for tooling and
this document must update in the same change.

Companion artifacts:

- [`/artifacts/qa/window_display_matrix.yaml`](../../artifacts/qa/window_display_matrix.yaml)
  — machine-readable scenario rows, drill ids, claimed-profile notes,
  and cadence guidance.
- [`/fixtures/platform/window_display_cases/`](../../fixtures/platform/window_display_cases/)
  — reviewable scenario fixtures the matrix rows point at.
- [`/docs/platform/desktop_platform_conformance_matrix.md`](../platform/desktop_platform_conformance_matrix.md)
  and
  [`/artifacts/platform/claimed_desktop_profiles.yaml`](../../artifacts/platform/claimed_desktop_profiles.yaml)
  — the claimed desktop rows this seed qualifies.
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
  — the window-topology, placeholder, and no-rerun vocabulary this seed
  reuses rather than redefining.

Normative sources projected here:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` state-ownership
  and window/restore rules plus `FIT-WIN-001`.
- `.t2/docs/Aureline_Technical_Design_Document.md` desktop support and
  workspace-window architecture sections.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` restore-provenance,
  topology-chaos, and crash/restore drill sections.
- [`/docs/platform/desktop_platform_conformance_matrix.md`](../platform/desktop_platform_conformance_matrix.md)
  windowing/DPI and wake/display-reconnect rows.

## What this seed freezes

- one stable scenario-id family for split layouts, detached windows,
  display detach/dock, mixed-DPI reflow, fullscreen or snapped restore,
  off-screen recovery, suspend/resume, and restart/reopen;
- one required drill list for focus routing, dialog ownership,
  placeholder hydration, restore provenance, and no-hidden-rerun
  behavior;
- one claimed-profile note set so monitor-topology, mixed-DPI, and
  laptop/mobile power expectations stay tied to named profile ids;
- one cadence posture: targeted validation on affected rows for window
  or restore changes, and full claimed-profile validation before release
  candidates widen desktop claims.

This seed does not implement an automated chaos harness. It freezes the
reviewable matrix and fixture corpus future harnesses, RC packets, and
support exports will cite.

## Shared rules

| Rule | Required truth |
|---|---|
| Layout intent over pixels | Preserve pane order, window role, active tab, focus visibility, and reachable recovery actions. Stale monitor coordinates are best-effort metadata only. |
| Visible focus over stale geometry | After detach, DPI change, recenter, or reopen, the next useful focus target stays visible and keyboard reachable. |
| Dialog ownership stays window-local | Sheets, prompts, and dialogs remain attached to the originating window and return focus there on dismiss. |
| Restorable context is not live authority | Titles, transcripts, cwd hints, badges, and pane placement may restore while terminal/debug/notebook/remote authority still requires reconnect, rerun, or reauthorization. |
| Restore provenance stays visible | Topology adjustments, missing dependencies, and continuity level remain inspectable after reopen or wake. |
| No hidden rerun | Restore may reopen layout and evidence, but it must not rerun commands or silently reacquire broader authority in the background. |

## Scenario matrix

| Scenario id | Covers | Continuity expectation | Main proof |
|---|---|---|---|
| `split_layout_detached_auxiliary_focus` | split layout + detached window | `no_restore` | re-docking preserves split identity, detached-window role, and visible focus without warping into a background pane |
| `display_detach_dock_safe_bounds` | display detach/dock + off-screen recovery | `no_restore` with topology-adjustment note | windows snap into reachable bounds and keep the working pane visible |
| `mixed_dpi_cross_monitor_reflow` | mixed-DPI and scale-bucket changes | `no_restore` with topology-adjustment note | readable scale, reachable sheets, and inspector ownership survive per-monitor DPI changes |
| `fullscreen_snapped_restore_intent` | fullscreen/spaces/snap restore | `compatible_restore` | dominant work mode intent survives even if the OS rewrites stale window-manager state |
| `offscreen_dialog_owner_recenter` | off-screen dialog/sheet recovery | `no_restore` with topology-adjustment note | orphaned prompts recenter on the owning window and focus returns there |
| `suspend_resume_remote_rebind` | suspend/resume + display reconnect | `compatible_restore` | local context survives wake, but remote/debug/callback authority rebinding stays explicit |
| `restart_reopen_live_surface_rebind` | restart/reopen of live surfaces | `compatible_restore` | transcripts, lineage, and layout reopen without hidden reruns or silent control-channel reuse |
| `restart_reopen_missing_dependency_placeholder` | restart/reopen with missing dependencies or revoked authority | `layout_only` | missing panes stay as placeholders or evidence-only cards without collapsing the surrounding layout |

## Required drills

| Drill id | Must prove | Typical scenario rows |
|---|---|---|
| `focus_routing_visible_return` | focus never lands in a hidden or unreachable pane; the next useful keyboard target remains visible after reflow or reopen | every row |
| `dialog_ownership_window_local` | dialogs and sheets stay attached to the owning window across moves, scale changes, and reconnects | detached-window, topology-change, mixed-DPI, fullscreen/snap, off-screen rows |
| `placeholder_hydration_preserves_topology` | only the unavailable pane degrades; pane ids, tab order, and split structure remain intact | suspend/resume and restart/reopen missing-surface rows |
| `restore_provenance_visible_after_reopen` | continuity level, topology remaps, and missing-target states stay visible after wake or reopen | display-topology, mixed-DPI, fullscreen/snap, suspend/resume, restart/reopen rows |
| `no_hidden_rerun_live_surfaces` | terminals, tasks, debuggers, notebooks, previews, AI, and remote panes disclose truthful postures instead of replaying hidden work | suspend/resume and restart/reopen live-surface rows |

## Claimed profile notes

| Profile id | Monitor topology | Mixed DPI | Laptop/mobile power |
|---|---|---|---|
| `macos_15_plus_universal` | Spaces/fullscreen plus display detach/redock are claim-bearing; safe-bounds remap beats stale coordinates. | Retina/non-Retina moves keep active panes and sheets visible. | Sleep/wake revalidates callbacks, remote state, and display topology before privileged resume. |
| `windows_11_23h2_plus_x86_64` | Docking, undocking, snapped layouts, and wake/display reconnect are claim-bearing. | Per-monitor DPI restore is normative; stale pixel rectangles are not. | Resume may keep layout and transcript context, but remote/debug/callback authority must rebind explicitly. |
| `linux_ubuntu_24_04_gnome_wayland_x86_64` | Portal-aware monitor changes keep visible focus and safe recovery actions explicit. | Fractional scaling claims preserve readable scale and reachable recovery. | On battery-backed hardware, wake keeps local-safe continuation visible and requires explicit reconnect for remote work. |
| `linux_ubuntu_24_04_gnome_x11_x86_64` | X11 rows must still recover stranded windows and dialogs explicitly. | Mixed-DPI behavior may narrow compared with Wayland, but reachability remains mandatory. | Suspend/resume still revalidates remote and privileged state before live resume. |
| `linux_fedora_current_gnome_wayland_x86_64` | Safe-bounds remap, visible focus, and no silent stranded windows apply on disconnect/reconnect. | Fractional scaling must keep the active pane and owner dialog visible after monitor moves. | Portable Fedora hardware may continue local-safe work, but remote authority must rebind explicitly. |
| `linux_debian_stable_gnome_x11_x86_64` | The narrowest Linux claim row still requires explicit off-screen recovery and dialog ownership. | Layout intent and visible focus remain mandatory even if compositor exactness narrows to layout-only recovery. | Battery-backed runs preserve local context while keeping reconnect/rerun requirements visible for live panes. |

## Planning and evidence use

The planning backlog already names suspend/resume, mixed-DPI, and
multi-monitor continuity as claimed-dogfood evidence. This matrix is
the stable row set that those future packets attach to. Later RC
checklists, support exports, or automation outputs should cite
`scenario_id` and `drill_id` values from the YAML rather than writing a
fresh topology checklist in each lane.
