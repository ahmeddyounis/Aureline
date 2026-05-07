# Responsive fallback fixtures

Curated cases for the responsive fallback and identity-cue preservation contract:

- [`/docs/ux/shell_responsive_fallback_contract.md`](../../../docs/ux/shell_responsive_fallback_contract.md)
- machine-readable rules: [`/artifacts/ux/zone_priority_rules.yaml`](../../../artifacts/ux/zone_priority_rules.yaml)

These fixtures are **scenario-first**: they pin the constraint state (width,
split pressure, density, zoom/text scaling, restore, multi-monitor moves) and
the expected cue fallbacks so later shell implementations do not invent private
collapse behavior per surface.

All identities are opaque refs. Fixtures carry **no raw absolute paths**, raw
URLs, raw credentials, raw prompt bodies, or raw logs.

## Cases

| Fixture | Stress axis | Main proof |
| --- | --- | --- |
| [`narrow_desktop_long_path_breadcrumb_overflow.json`](./narrow_desktop_long_path_breadcrumb_overflow.json) | narrow width + long path | Breadcrumb root + current item survive; intermediate segments overflow; status overflow stays labeled and keyboard reachable. |
| [`stacked_splits_tab_priority_and_compare_fallback.json`](./stacked_splits_tab_priority_and_compare_fallback.json) | stacked splits | Per-group active/dirty/pinned tabs stay reachable; compare degrades to tabbed compare rather than unusable panes; focus continuity preserved. |
| [`multi_monitor_move_identity_stable_compact_shell.json`](./multi_monitor_move_identity_stable_compact_shell.json) | multi-monitor move | Window reflow engages compact-shell fallback without losing trust/target/route cues; overflow triggers remain labeled and discoverable. |
| [`high_density_compact_overflow_parity.json`](./high_density_compact_overflow_parity.json) | density | Compact density does not mint new collapse rules; status + tabs + breadcrumbs remain keyboard-complete and stable under crowding. |
| [`zoom_400_overflow_keyboard_and_sr_parity.json`](./zoom_400_overflow_keyboard_and_sr_parity.json) | zoom/text scaling | 400% zoom collapses into typed overflow/sheet surfaces; keyboard path and screen-reader announcements remain coherent. |
| [`restored_window_placeholder_preserves_identity.json`](./restored_window_placeholder_preserves_identity.json) | restore | Restored windows keep shell shape; missing surfaces become placeholders in-place; cues remain truthful and re-entry explicit. |
| [`terminal_header_long_title_collapse_preserves_boundary.json`](./terminal_header_long_title_collapse_preserves_boundary.json) | long title | Terminal boundary/host + exit/degraded state survive even when title truncates; full identity recovered via overflow/sheet with keyboard route. |

