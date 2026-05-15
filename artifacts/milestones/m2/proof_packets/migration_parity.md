# External alpha migration parity proof packet

```yaml
packet_id: review_packet:alpha.migration_parity.2026-05-15
artifact_index_ref: artifacts/milestones/m2/artifact_index.yaml
scoreboard_row_refs:
  - scoreboard_row:alpha_scope.migration_parity
owner_dri: "@ahmeddyounis"
reviewer_refs:
  - "@alpha-review"
freshness_date: 2026-05-15
as_of: 2026-05-15
captured_at: 2026-05-15T08:44:25Z
stale_after: P14D
source_revision: git:0a66b910be0eaa57e0569335dd19faec0684e368
trigger_revision: alpha_migration_parity_contract_set@2026-05-15
channel_context: preview
deployment_context:
  - individual_local
claim_change_state: no_claim_widening
same_change_truth_refs:
  docs_ref: docs/migration/import_diagnostics_packet.md
  migration_ref: artifacts/migration/m2_parity_scoreboard.yaml
  known_limits_ref: artifacts/feedback/external_alpha_known_limits.md
  support_export_ref: docs/support/support_bundle_contract.md
```

This packet registers the current proof root for external alpha migration
parity routing. It promotes the alpha scope row to green by proving that the
scoreboard, import-gap taxonomy, retained diagnostics packet, fixture manifest,
and validator agree on the switching-path truth they expose.

## Canonical Artifacts

- Alpha scope row: `scoreboard_row:alpha_scope.migration_parity`
- Parity scoreboard: `artifacts/migration/m2_parity_scoreboard.yaml`
- Import-gap taxonomy: `artifacts/migration/import_gap_taxonomy.yaml`
- Retained diagnostics packet: `docs/migration/import_diagnostics_packet.md`
- Protected fixture manifest: `fixtures/migration/parity_alpha_cases/manifest.yaml`
- Known limits: `artifacts/feedback/external_alpha_known_limits.md`
- Validator: `ci/check_migration_parity_alpha.py`
- Latest capture: `artifacts/milestones/m2/captures/migration_parity_validation_capture.json`

## Required Outcome

`accept_current`

Evidence is fresh, scoped to the individual-local preview channel, and cited by
the owning scoreboard row. This packet does not claim replacement-grade
importer behavior, full extension runtime parity, full source-tool profile
round-trip, or managed workspace migration parity.

## Evidence Rows

| Evidence | Value |
|---|---|
| Owning packet | `artifacts/milestones/m2/proof_packets/migration_parity.md` |
| Latest capture | `artifacts/milestones/m2/captures/migration_parity_validation_capture.json` |
| Validator commands | See `validator_commands` in the capture |
| Freshness rule | `docs_claim_truth_proof`, `stale_after: P14D` |

## Protected Proof Path

Run the protected checks:

```sh
python3 ci/check_migration_parity_alpha.py --repo-root .
python3 ci/check_migration_parity_alpha.py --repo-root . --render-retained-diagnostics
python3 ci/check_migration_parity_alpha.py --repo-root . --report artifacts/milestones/m2/captures/migration_parity_validation_capture.json
python3 ci/check_alpha_scope.py --repo-root .
```

## Import Mapping Coverage

The capture cites the five required import-review classes:

| Class | Current evidence |
|---|---|
| `Exact` | `migration_parity_row:tsjs.vscode.settings_keymap.native` and `fixtures/migration/parity_alpha_cases/manifest.yaml#case:native_vscode_settings_keymap` |
| `Translated` | `migration_parity_row:tsjs.vscode.settings_keymap.native`, `migration_parity_row:tsjs.vscode.extension_provider.bridged`, and translated equivalence refs in the parity scoreboard |
| `Partial` | `migration_parity_row:tsjs.vscode.keymap_chord.lossy`, `migration_parity_row:python.jetbrains.run_debug.manual_follow_up`, and `migration_parity_row:python.vscode.launch_config.lossy` |
| `Shimmed` | `migration_parity_row:tsjs.vscode.extension_provider.bridged` and `import_gap:extension_provider_bridge_not_native` |
| `Unsupported` | `migration_parity_row:tsjs.vscode.webview_extension.unsupported` and the unsupported runtime taxonomy rows |

## Import-Gap Taxonomy Coverage

The capture now includes one `taxonomy_gap_coverage` entry for every row in
`artifacts/migration/import_gap_taxonomy.yaml`:

- `import_gap:extension_provider_bridge_not_native`
- `import_gap:keymap_chord_capacity_remap`
- `import_gap:tasks_run_config_execution_context_review`
- `import_gap:extension_runtime_webview_unsupported`
- `import_gap:extension_runtime_lua_unsupported`
- `import_gap:extension_runtime_elisp_unsupported`
- `import_gap:theme_token_slot_fallback`

Each entry carries inferred Exact / Translated / Partial / Shimmed /
Unsupported class coverage where applicable, parity states, outcome states,
scoreboard refs, fixture refs when the protected manifest owns one, known-limit
refs, docs/help refs, support-export refs, and issue-template refs.

## Substrate Consumed

- `crates/aureline-shell/src/import/diff_review.rs` defines the exact
  `ImportMappingClassification` vocabulary and retained migration report
  reopening contract.
- `crates/aureline-shell/src/import/mod.rs` emits first-pass import review
  records before mutation.
- `crates/aureline-shell/src/start_center/admission_review.rs` routes Start
  Center import choices through reviewed admission packets.
- `crates/aureline-workspace/src/admission` keeps import admission, staging,
  and rollback checkpoint posture explicit.
- `crates/aureline-telemetry/src/onboarding` records migration outcome counts
  without collecting source content.
- `crates/aureline-shell/src/onboarding` and
  `crates/aureline-shell/src/learning_tour_alpha` consume the same onboarding,
  migration, and help truth surfaces rather than inventing separate parity
  wording.

## Same-Change-Set Checklist

- Owning proof packet refreshed.
- Validator capture refreshed with import mapping class coverage and full
  taxonomy gap coverage.
- Scoreboard row moved to `green` and remains `conditional_go`.
- Docs, migration notes, Help/About truth, known limits, and support-export
  truth are explicitly unchanged because this is `no_claim_widening`.
