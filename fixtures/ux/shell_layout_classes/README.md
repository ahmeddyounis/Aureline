# Shell layout-class fixtures

Seed corpus for the contract frozen in
[`/docs/ux/shell_zone_and_density_contract.md`](../../../docs/ux/shell_zone_and_density_contract.md)
and the machine-readable artifact at
[`/artifacts/ux/shell_metrics.yaml`](../../../artifacts/ux/shell_metrics.yaml).

Each file is a single JSON document describing one reviewable
shell-layout rehearsal — compact, standard, expanded, split-
heavy, multi-window, or off-screen-restore — that the renderer,
layout, restore, and QA lanes can cite by id. A fixture is a
**seed**: it pins the adaptive class, density mode, visible
zones, engaged fallback surfaces, preserved identity cues,
preserved required-visible-field set, and the restore /
lifecycle posture for later shell implementations to plug into
without minting private fallback stories.

Every fixture:

- Resolves every axis to vocabulary re-exported from
  [`/artifacts/ux/shell_metrics.yaml`](../../../artifacts/ux/shell_metrics.yaml),
  [`/docs/ux/shell_interaction_safety_contract.md`](../../../docs/ux/shell_interaction_safety_contract.md),
  [`/docs/workspace/layout_serialization_contract.md`](../../../docs/workspace/layout_serialization_contract.md),
  and [`/artifacts/ux/desktop_shell_boundary_matrix.yaml`](../../../artifacts/ux/desktop_shell_boundary_matrix.yaml).
- Pins `adaptive_class` (and `per_window_adaptive_classes` for
  multi-window scenarios) to values from the
  `adaptive_class_vocabulary` in `shell_metrics.yaml`.
- Cites the multi-window scenario id it exercises (from
  [`/artifacts/qa/window_display_matrix.yaml`](../../../artifacts/qa/window_display_matrix.yaml)
  and
  [`/docs/qa/multi_window_verification.md`](../../../docs/qa/multi_window_verification.md))
  where applicable.
- Asserts `chrome_hid_required_field = false`. A fixture that
  pins this field to `true` is non-conforming — the active
  consequence class would deny the interaction under the
  shell-interaction-safety contract's
  `chrome_hid_required_field` rule.
- Carries no raw absolute paths, raw URLs, raw credential
  material, raw prompt text, or raw logs. Every identity is
  an opaque ref; every timestamp is ISO-8601 or monotonic.
- Lists `forbidden_outcomes_verified_absent` — the typed
  outcomes the scenario proves are not happening.

## Cases

| Fixture | Adaptive class(es) | Density | Main proof |
| --- | --- | --- | --- |
| [`compact_desktop_inspector_sheet.json`](./compact_desktop_inspector_sheet.json) | `compact_desktop` | `standard` | Right inspector sheets on demand, tabs / breadcrumbs / status indicators collapse to typed fallback surfaces (overflow menu, compact shell menu, summary chip) while root / current-item / trust / execution-target cues remain visible. |
| [`standard_desktop_full_chrome.json`](./standard_desktop_full_chrome.json) | `standard_desktop` | `standard` | Full chrome baseline at 1440 px with no collapse; restart_reopen_live_surface_rebind restore proves `skeleton` / `hydrate` run automatically while a terminal pane remains `evidence_only` until the user reruns explicitly. |
| [`expanded_desktop_full_chrome.json`](./expanded_desktop_full_chrome.json) | `expanded_desktop` | `standard` | Two side surfaces coexist on 1920 px; fullscreen_snapped_restore_intent scenario verifies dominant work-mode intent survives OS state rewrites; duplicated truth surfaces and oversized empty chrome remain forbidden. |
| [`split_heavy_desktop_tabbed_compare_fallback.json`](./split_heavy_desktop_tabbed_compare_fallback.json) | `split_heavy_desktop` | `standard` | Opening a third editor group that would violate the 420 px main-workspace minimum explicitly falls back to tabbed compare; per-group identity and focus are preserved across the dock-to-tabbed transition. |
| [`multi_window_desktop_split_heavy.json`](./multi_window_desktop_split_heavy.json) | `multi_window_desktop`, `expanded_desktop`, `compact_desktop`, `split_heavy_desktop` | `standard` | Two top-level windows back the same workspace authority with different per-window adaptive classes; a destructive-apply dialog stays attached to its owning window; closing a window with dirty buffers blocks with a continuation path. |
| [`offscreen_restore_recentre.json`](./offscreen_restore_recentre.json) | `multi_window_desktop` | `standard` | Last-known bounds on a disconnected external display become unreachable; shell remaps to safe bounds and records `safe_bounds_remap` on restore provenance; an owner dialog recenters with its window; a terminal pane restores as `evidence_only` and `rebind` revalidation is required for remote / credential / trust state. Exercises display_detach_dock_safe_bounds, offscreen_dialog_owner_recenter, mixed_dpi_cross_monitor_reflow, and suspend_resume_remote_rebind scenarios. |

## Schema references

- Shell metrics, zones, density, adaptive classes, identity
  cues, multi-window rules, restore rules:
  [`/artifacts/ux/shell_metrics.yaml`](../../../artifacts/ux/shell_metrics.yaml).
- Responsive-fallback modes and required-visible-field set:
  [`/docs/ux/shell_interaction_safety_contract.md`](../../../docs/ux/shell_interaction_safety_contract.md)
  and
  [`/schemas/ux/interaction_safety.schema.json`](../../../schemas/ux/interaction_safety.schema.json).
- Restore phases, placeholder payloads, workspace-authority
  separation:
  [`/docs/workspace/layout_serialization_contract.md`](../../../docs/workspace/layout_serialization_contract.md)
  and
  [`/schemas/workspace/pane_tree.schema.json`](../../../schemas/workspace/pane_tree.schema.json).
- Multi-window scenario id rows and claimed-profile notes:
  [`/artifacts/qa/window_display_matrix.yaml`](../../../artifacts/qa/window_display_matrix.yaml)
  and
  [`/docs/qa/multi_window_verification.md`](../../../docs/qa/multi_window_verification.md).

## Build identity

Every fixture carries `running_build_identity_ref:
build-identity-seed-shell-metrics` reserved for later exact-
build-identity wiring. A later lane resolves the ref against
the fixture / build-identity record without renaming the
field.

## What's out of scope

- Final visual specification of compact / standard /
  comfortable density chrome.
- Per-OS titlebar / fullscreen / snap / Spaces behavior.
- The production layout-restore engine.

These lines move only by opening a new decision row, not by
editing a fixture.
