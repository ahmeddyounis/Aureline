# Support Center beta shell-surface fixture corpus

Protected fixture corpus for the shell-side Support Center beta surface
seeded by `crates/aureline-shell/src/support_center/`. Every fixture is
a `support_center_beta_surface_record` projection that mirrors the
boundary schema at `schemas/support/support_center_beta.schema.json`.

## Scenarios

- **local_only_baseline.yaml** — Reference local-only individual install.
  No degraded state is active; the surface still quotes the
  `none_degraded` truth row so a reviewer can read the state directly
  rather than infer it. Every launch action declares `local_only`
  service dependency and `no_account_required`.

- **post_crash_loop_degraded.yaml** — Reference safe-mode active
  surface entered after a startup crash loop. `safe_mode_active` is
  quoted as a degraded-truth row with the explicit exit command. The
  claim truth row marks the safe-mode lane as
  `degraded_local_only`; every launch action remains reachable
  without account creation.

- **managed_workspace_export_only.yaml** — Reference managed
  deployment context. Local-only recovery lanes still pin
  `no_account_required`; only the optional handoff and escalation
  packet draft destinations declare a managed-admin gate on apply.
  The export-route rows still resolve to local-first preview paths,
  never an upload-first first action.

## Acceptance contract

Every scenario in this corpus MUST:

1. List at least one launch-action row each for `enter_safe_mode`,
   `open_project_doctor`, `start_extension_bisect`,
   `open_repair_preview`, and an export action (either
   `export_support_bundle` or `preview_support_bundle`).
2. Pin `no_account_required_for_local_only` and
   `no_hidden_service_required_for_local_only` to `true`.
3. Quote `user_authored_files` in the preserved-state set of every
   launch-action row.
4. Quote at least one claim-truth row bound to a stable scorecard
   target and an evidence packet ref.
5. Quote at least one degraded-truth row (use `none_degraded` when
   nothing is currently degraded, so the surface always names the
   state explicitly).
6. Set `local_first_path_named` to `true` and
   `upload_required_for_first_action` to `false` on every
   export-route row.

These rules are enforced by `SupportCenterBetaEvaluator` in
`crates/aureline-shell/src/support_center/mod.rs` and replayed against
every fixture by `support_center_beta_fixtures.rs`.
