# Experiments / Flags / Labs Governance Beta Fixtures

These fixtures exercise the beta experiments / flags / Labs governance UI
projection. They are copy-safe examples for the shell governance card,
diagnostics panel, settings root pane, support-export packets, and the
reviewer-facing companion doc.

The canonical source remains
`artifacts/governance/experiments_inventory_alpha.yaml`. The projection
adds: per-row alignment fields (owner, cohort/ring, expiry/review date,
kill-switch path summary), visible-marker tokens that prevent
stable-claiming surfaces from silently depending on hidden Labs / Preview
/ Beta state, and a closed lifecycle vocabulary shared by UI, CLI, and
support-export rows.

## Files

- `page.json` — full beta governance page emitted by
  `cargo run -q -p aureline-shell --bin aureline_shell_experiments_governance -- page`.
- `cli_projection.json` — CLI / headless projection
  (`cargo run ... -- cli`).
- `support_export.json` — support-export wrapper
  (`cargo run ... -- support-export`).
- `render_summary.json` — shell-facing rendering summary
  (`cargo run ... -- summary`).
- `drill_hidden_experiment_on_stable_surface.json` — failure drill where
  a Labs row is rendered on a stable-claiming surface without a visible
  marker; the validator flags
  `stable_host_renders_hidden_experiment` and `visible_marker_missing`.
- `drill_missing_alignment_field.json` — failure drill where the owner
  is removed from a row; the validator flags
  `alignment_field_missing` for the `owner` field.

Protected states covered:

- visible lifecycle rows for Labs, Preview, Beta, Stable, Deprecated,
  DisabledByPolicy, and Retired;
- kill-switch precedence with preserved local data and a non-empty
  fallback path;
- visible-marker tokens for every non-Stable row on a
  `claims_stable_posture` host surface;
- per-row alignment of owner, cohort/ring, expiry/review date, and
  kill-switch summary.
