# Import Diff And Rollback Packet

This packet is the release-evidence object for beta switching import
rows. It joins the existing migration corpus and wizard fixtures to
the first-useful-work scorecard so import failures, lossy mappings,
and rollback posture remain inspectable after first run.

- Packet id: `migration_packet:import_diff_and_rollback.beta_switching`
- As of: `2026-05-15`
- Scorecard: `artifacts/milestones/m3/first_useful_work_scorecard.json`
- Packet manifest: `fixtures/onboarding/m3/first_useful_work/manifest.yaml`
- Wizard mapping report: `fixtures/migration/m3/migration_wizard/mapping_report.json`
- Wizard rollback checkpoint: `fixtures/migration/m3/migration_wizard/rollback_checkpoint.json`
- Support export: `fixtures/migration/m3/migration_wizard/support_export.json`
- Claim-manifest evidence id: `claim_evidence:switching.first_useful_work_scorecard`

## Packet Rules

Every import row must preserve these fields as structured data:

| Field | Required behavior |
|---|---|
| Before config | Source object refs, domain, source label, producer version when known, and raw-body exclusion posture. |
| After config | Aureline target refs, mapped setting/command/profile ids, and explicit non-imported target refs. |
| Mapping result | One of `Exact`, `Translated`, `Partial`, `Shimmed`, or `Unsupported`; no generic success bucket. |
| Skipped mappings | Source object, reason class, policy or unsupported cause, and docs/help ref. |
| Rollback checkpoint | Checkpoint ref created before durable apply for every touched domain. |
| Retained diagnostics | Mapping report ref, unsupported gap refs, compare actions, undo actions, support export refs. |
| Claim downgrade hook | Scorecard row and downgrade trigger that narrows claim copy when the packet is stale or regressed. |

## Rollback Checkpoints

| Checkpoint ref | Scope | Created before apply | Protects every touched domain | Retention posture | Restore guidance |
|---|---|---:|---:|---|---|
| `rollback-checkpoint:import-review-3bcae9aef7bd1cab` | imported settings, keybindings, snippets, tasks, extension recommendations, profile metadata | yes | yes | retained with migration report | restore through `cmd:workspace.restore_from_checkpoint`; export support bundle if restore fails |

## Import: VS Code / Code-OSS
<a id="import-vs-code--code-oss"></a>

Scorecard row:
`scorecard_row:first_useful_work.import.vs_code_code_oss`

First-useful-work packet:
`fixtures/onboarding/m3/first_useful_work/manifest.yaml#fuw_packet:entry.import_vscode_diff_rollback`

| Domain | Before config | After config | Mapping result | Skipped or retained gap | Rollback / diagnostic refs |
|---|---|---|---|---|---|
| Settings | `.vscode/settings.json` | Aureline user/workspace setting records | `Exact` | none | `mapping-report:import-review-3bcae9aef7bd1cab` |
| Shortcuts | `workbench.action.showCommands` in `.vscode/keybindings.json` | `aureline:command.palette.open` | `Translated` | command-id stability caveat retained | `fixtures/migration/m3/migration_wizard/mapping_report.json` |
| Keymaps | multi-key chord | shortcut delta digest | `Partial` | platform-reserved chord remains manual-reviewable | `fixtures/migration/m3/migration_wizard/compare_actions.json` |
| Extension/provider | `dbaeumer.vscode-eslint` | native ESLint lint package recommendation | `Shimmed` | source extension authority and storage not imported | `fixtures/migration/compatibility_scorecards/native_alternative_recommendation.json` |
| Webview runtime | webview-heavy extension state | no safe target | `Unsupported` | apply denied for source object | `fixtures/migration/m3/migration_wizard/unsupported_gaps.json` |

Rollback guidance:

- Restore from `rollback-checkpoint:import-review-3bcae9aef7bd1cab`.
- Reopen compare actions before applying a narrower retry.
- Keep unsupported webview state as retained diagnostics only; do not
  apply it through extension runtime authority.

Downgrade triggers:

- `mapping_report_missing_required_class`
- `rollback_checkpoint_missing`
- `unsupported_gap_hidden`
- `first_useful_work_packet_stale`

## JetBrains IDEs
<a id="jetbrains-ides"></a>

Scorecard row:
`scorecard_row:first_useful_work.import.jetbrains_family`

First-useful-work packet:
`fixtures/onboarding/m3/first_useful_work/manifest.yaml#fuw_packet:import.jetbrains_profile`

| Domain | Before config | After config | Mapping result | Skipped or retained gap | Rollback / diagnostic refs |
|---|---|---|---|---|---|
| Formatter/profile | common formatter profile | Aureline formatter hints | `Exact` | none | `fixtures/migration/source_profile_examples/jetbrains_family_profile.yaml` |
| Keymap | common IDE preset | `aureline:keymaps.jetbrains_preset` | `Translated` | plugin-specific actions excluded | `docs/migration/keymap_presets.md` |
| Run/debug | application run config | execution-context candidate | `Partial` | reviewer must accept runnable target | `fixtures/migration/compatibility_scorecards/partial_run_debug_translation.json` |
| Workspace roots | module/content roots | workspace-manifest roots | `Shimmed` | IDE index semantics not claimed | `artifacts/migration/m3/migration_scoreboard.md` |
| Plugin runtime | arbitrary source plugin | no safe target | `Unsupported` | runtime apply denied | `fixtures/migration/m3/incumbent_flows/jetbrains_family.json` |

Rollback guidance:

- Restore imported settings, keymap preset, workspace roots, and
  profile metadata through the wizard checkpoint.
- Do not roll back by deleting source profile files; the checkpoint
  only owns Aureline-side durable state.

Downgrade triggers:

- `manual_review_row_hidden`
- `post_import_validation_state_changed`
- `rollback_checkpoint_missing`
- `first_useful_work_packet_stale`

## Vim / Neovim
<a id="vim--neovim"></a>

Scorecard row:
`scorecard_row:first_useful_work.import.vim_neovim`

First-useful-work packet:
`fixtures/onboarding/m3/first_useful_work/manifest.yaml#fuw_packet:import.vim_neovim_profile`

| Domain | Before config | After config | Mapping result | Skipped or retained gap | Rollback / diagnostic refs |
|---|---|---|---|---|---|
| Modal editing | curated modal profile | Aureline modal-editing profile | `Exact` | none | `fixtures/migration/source_profile_examples/vim_neovim_profile.yaml` |
| Leader keys | leader-key mappings | leader overlay command ids | `Translated` | Lua/Vimscript actions excluded | `artifacts/migration/m3/migration_scoreboard.md` |
| Snippets | selected snippet directories | snippet/template records | `Partial` | unsupported snippet engine behavior retained | `docs/migration/source_ecosystem_coverage_matrix.md` |
| Clipboard/search | option defaults | modal-profile shim | `Shimmed` | register history and macro history excluded | `fixtures/migration/m3/incumbent_flows/vim_neovim.json` |
| Plugin runtime | arbitrary Lua runtime | no safe target | `Unsupported` | runtime apply denied | `fixtures/migration/compatibility_scorecards/blocked_lua_plugin_runtime.json` |

Rollback guidance:

- Roll back Aureline modal profile, leader overlay, snippet rows, and
  shim records together so the profile does not retain a half-imported
  modal state.

Downgrade triggers:

- `unsupported_runtime_applied`
- `leader_overlay_schema_changed`
- `rollback_checkpoint_missing`
- `first_useful_work_packet_stale`

## Emacs
<a id="emacs"></a>

Scorecard row:
`scorecard_row:first_useful_work.import.emacs`

First-useful-work packet:
`fixtures/onboarding/m3/first_useful_work/manifest.yaml#fuw_packet:import.emacs_profile`

| Domain | Before config | After config | Mapping result | Skipped or retained gap | Rollback / diagnostic refs |
|---|---|---|---|---|---|
| Global keymap | global keymap and command aliases | Aureline command aliases | `Translated` | source-only commands retained as skipped rows | `fixtures/migration/source_profile_examples/emacs_profile.yaml` |
| Project defaults | source project defaults | manual-review workspace defaults | `Partial` | reviewer must accept project defaults | `fixtures/migration/restore_shortcut_cases/emacs_elisp_blocked_widening_held.yaml` |
| Theme | token mapping | design-token shim | `Shimmed` | non-tokenized theme behavior excluded | `artifacts/migration/m3/migration_scoreboard.md` |
| Package runtime | Elisp package runtime | no safe target | `Unsupported` | runtime apply denied | `fixtures/migration/compatibility_scorecards/blocked_elisp_package_runtime.json` |
| Profile notes | source profile metadata | retained diagnostic notes | `Exact` | raw startup files excluded | `fixtures/migration/m3/incumbent_flows/emacs.json` |

Rollback guidance:

- Restore imported keymap, theme-token shim, project-default review
  state, and retained diagnostics through the checkpoint.
- Unsupported Elisp package state remains evidence only.

Downgrade triggers:

- `unsupported_runtime_applied`
- `post_import_validation_state_changed`
- `rollback_checkpoint_missing`
- `first_useful_work_packet_stale`

## Failure Packet Requirements

When an import fails or becomes lossy after preview, support/export
surfaces must retain:

- the reviewed preview ref;
- the source and target descriptors;
- every skipped or unsupported source object;
- the rollback checkpoint ref and checkpoint state;
- the typed diagnostic rows that explain policy, unsupported runtime,
  missing dependency, or manual-review causes;
- the scorecard row whose status must narrow if the failure is current.

Failure packets may omit raw source bodies, raw absolute paths, raw
credentials, raw command histories, raw extension storage, and raw URLs.
