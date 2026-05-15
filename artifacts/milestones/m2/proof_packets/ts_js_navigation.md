# External alpha TS/JS navigation proof packet

```yaml
packet_id: review_packet:alpha.ts_js_navigation.2026-05-15
artifact_index_ref: artifacts/milestones/m2/artifact_index.yaml
scoreboard_row_refs:
  - scoreboard_row:alpha_scope.ts_js_navigation
owner_dri: "@ahmeddyounis"
reviewer_refs:
  - "@alpha-review"
freshness_date: 2026-05-15
captured_at: 2026-05-15T07:49:35Z
stale_after: P14D
source_revision: git:dbbcdc964679ad645f96b46f39584acb54db80e4
trigger_revision: ts_js_acceptance_rows@2026-05-15
exact_build_identity_ref: artifacts/build/build_identity.json
channel_context: preview
deployment_context:
  - individual_local
claim_change_state: no_claim_widening
same_change_truth_refs:
  docs_ref: docs/milestones/m2_alpha_scope.md
  migration_ref: docs/migration/source_ecosystem_coverage_matrix.md
  help_truth_ref: docs/docs/help_about_service_health_routes.md
  known_limits_ref: artifacts/feedback/external_alpha_known_limits.md
  support_export_ref: docs/support/support_bundle_contract.md
```

This packet registers the current proof root for the external alpha TS/JS
navigation floor. It closes the blocked packet state for first useful result,
quick-open, symbol/route navigation truth, and rename-preview reviewability
without widening the alpha-limited TS/JS claim.

## Canonical Artifacts

- Scoreboard row: `scoreboard_row:alpha_scope.ts_js_navigation`
- Acceptance matrix: `artifacts/compat/ts_js_acceptance_rows.yaml`
- Reference workspace seed: `fixtures/workspaces/reference/ts_web_app_archetype_seed.json`
- Launch bundle: `artifacts/bundles/tsjs_launch_bundle_alpha.yaml`
- Language pack: `artifacts/language_packs/tsjs_web_alpha.yaml`
- Latest capture: `artifacts/milestones/m2/captures/ts_js_navigation_validation_capture.json`
- Exact build identity: `artifacts/build/build_identity.json`

## Required Outcome

`accept_current`

Evidence is fresh, scoped to the individual-local preview channel, and cited by
the owning scoreboard row. The row stays alpha-limited: this packet does not
claim full replacement-grade TS/JS, framework expert depth, managed workspace
parity, browser preview parity, package-manager mutation parity, or AI patch
parity.

## Evidence Rows

| Evidence | Value |
|---|---|
| Owning packet | `artifacts/milestones/m2/proof_packets/ts_js_navigation.md` |
| Latest capture | `artifacts/milestones/m2/captures/ts_js_navigation_validation_capture.json` |
| Validator commands | See `validator_commands` in the capture |
| Exact build identity | `artifacts/build/build_identity.json` |
| Freshness rule | `docs_claim_truth_proof`, `stale_after: P14D` |

## Protected Proof Path

Run the substrate checks cited by the capture:

```sh
cargo test -p aureline-search --test hot_set_scheduler_cases --test remap_packets_alpha
cargo test -p aureline-language --test tsjs_nav_alpha --test tsjs_web_pack_alpha
cargo test -p aureline-shell --test search_alpha_validation
python3 ci/check_alpha_scope.py --repo-root .
```

## Coverage

The capture cites every row in `artifacts/compat/ts_js_acceptance_rows.yaml`.
Navigation-owned coverage is current for:

- `ts_js_acceptance_row:monorepo_first_useful_result`
- `ts_js_acceptance_row:rename_across_project_references`
- `ts_js_acceptance_row:framework_route_to_component_workflow`
- `ts_js_acceptance_row:mixed_js_ts_path_aliases`

The remaining TS/JS rows are cited as wedge-boundary rows and are not promoted
by this packet. They remain owned by the task/test/debug, Git/review,
deployment, migration, supportability, package, preview, or AI evidence lanes
named by the alpha matrix and known-limits packet.

## Substrate Consumed

- `crates/aureline-search/src/hot_set/mod.rs` keeps quick open useful before
  full indexing with hot-set readiness and partial-truth causes.
- `crates/aureline-search/src/remap/mod.rs` preserves deep-link and navigation
  continuity through explicit remap packets.
- `crates/aureline-graph/src/readiness/mod.rs` projects graph fact cues with
  readiness, truth-lane, and action-posture labels.
- `crates/aureline-language/src/packs/tsjs_web.rs` enables the TS/JS web pack
  from one bounded alpha artifact.
- `crates/aureline-language/src/tsjs/` emits hover, definition, reference, and
  rename-preview records with degraded-disclosure and generated/protected
  candidate counts.
- `crates/aureline-shell/src/palette/`, `crates/aureline-shell/src/search/`,
  and `crates/aureline-shell/src/start_center/` project first-useful work,
  quick-open/search, ranking reasons, and palette discoverability without
  inventing a second truth model.

## Same-Change-Set Checklist

- Owning proof packet added.
- Validator capture added.
- Scoreboard row moved to `green` and `conditional_go`.
- Scoreboard row now cites the acceptance matrix, reference workspace seed,
  owning packet, and latest capture.
- Docs, migration notes, Help/About truth, known limits, and support-export
  truth are explicitly unchanged because this is `no_claim_widening`.
- `artifacts/milestones/m2/known_limits_alpha.yaml` is unchanged.
