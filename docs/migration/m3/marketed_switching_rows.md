# Marketed Switching Rows

This page is the docs/help map for beta switching evidence. It points
every marketed switching row at the same first-useful-work packet,
scorecard row, import/rollback packet, restore provenance packet, and
claim-manifest hook used by support/export surfaces.

Authoritative artifacts:

- Scorecard:
  [`artifacts/milestones/m3/first_useful_work_scorecard.json`](../../../artifacts/milestones/m3/first_useful_work_scorecard.json)
- First-useful-work packet manifest:
  [`fixtures/onboarding/m3/first_useful_work/manifest.yaml`](../../../fixtures/onboarding/m3/first_useful_work/manifest.yaml)
- Import diff and rollback packet:
  [`artifacts/migration/m3/import_diff_and_rollback_packet.md`](../../../artifacts/migration/m3/import_diff_and_rollback_packet.md)
- Restore provenance packet:
  [`artifacts/migration/m3/restore_provenance_packet.md`](../../../artifacts/migration/m3/restore_provenance_packet.md)
- Claim manifest:
  [`artifacts/release/m3/claim_manifest.json`](../../../artifacts/release/m3/claim_manifest.json)
- Corpus lineage registry:
  [`fixtures/registry/corpus_registry.yaml`](../../../fixtures/registry/corpus_registry.yaml)
- Corpus freshness report:
  [`artifacts/registry/corpus_freshness_report.json`](../../../artifacts/registry/corpus_freshness_report.json)

## Required Row Truth

Each row below must keep these fields aligned:

| Field | Required source |
|---|---|
| Score | `first_useful_work_scorecard.json` |
| Packet | `fixtures/onboarding/m3/first_useful_work/manifest.yaml` |
| Account posture | Scorecard row `account_posture`; account-free local rows are not blended with managed/provider-linked rows. |
| Mapping result | Packet `mapping_class_outcomes` and import scoreboard rows. |
| Import rollback | `import_diff_and_rollback_packet.md` for lossy or failed import rows. |
| Restore provenance | `restore_provenance_packet.md` for restore/import/workspace-switch rows. |
| Claim downgrade | `claim_evidence:switching.first_useful_work_scorecard` in the generated claim manifest. |
| Corpus freshness | `corpus_claim_binding.migration_suite` in `corpus_freshness_report.json`. |

## Marketed Entry Rows

| Switching row | Scorecard row | Packet | Current posture |
|---|---|---|---|
| `switching_row:entry.local_open` | `scorecard_row:first_useful_work.entry.local_open` | `fuw_packet:entry.local_open_account_free` | account-free local, current |
| `switching_row:entry.clone` | `scorecard_row:first_useful_work.entry.clone` | `fuw_packet:entry.clone_review_first` | account-free local, current limited |
| `switching_row:entry.import` | `scorecard_row:first_useful_work.import.vs_code_code_oss` | `fuw_packet:entry.import_vscode_diff_rollback` | optional provider declined, current |
| `switching_row:entry.restore` | `scorecard_row:first_useful_work.entry.restore` | `fuw_packet:entry.restore_checkpoint_provenance` | account-free local, current limited |
| `switching_row:entry.missing_root_recovery` | `scorecard_row:first_useful_work.entry.recent_work_missing_root` | `fuw_packet:entry.recent_work_missing_root` | account-free local, current limited |
| `switching_row:entry.workspace_switch` | `scorecard_row:first_useful_work.entry.workspace_switch` | `fuw_packet:entry.workspace_switch_portable_workset` | account-free local, current |
| `switching_row:entry.archetype_routing` | `scorecard_row:first_useful_work.archetype.ts_web_app_or_service` and `scorecard_row:first_useful_work.archetype.python_service_or_data_app` | `fuw_packet:entry.archetype_routing_mixed_workspace` | account-free local, retest pending |

## Marketed Archetype Rows

| Archetype row | Scorecard row | Packet | Current posture |
|---|---|---|---|
| `beta_archetype:ts_web_app_or_service` | `scorecard_row:first_useful_work.archetype.ts_web_app_or_service` | `fuw_packet:entry.clone_review_first`, `fuw_packet:entry.archetype_routing_mixed_workspace` | account-free local, retest pending |
| `beta_archetype:python_service_or_data_app` | `scorecard_row:first_useful_work.archetype.python_service_or_data_app` | `fuw_packet:archetype.python_service_or_data_app` | account-free local, retest pending |
| `beta_archetype:java_or_kotlin_service` | `scorecard_row:first_useful_work.archetype.java_or_kotlin_service` | `fuw_packet:archetype.java_or_kotlin_service` | account-free local, retest pending |
| `beta_archetype:rust_workspace` | `scorecard_row:first_useful_work.archetype.rust_workspace` | `fuw_packet:archetype.rust_workspace` | account-free local, retest pending |
| `beta_archetype:go_service_or_monorepo_slice` | `scorecard_row:first_useful_work.archetype.go_service_or_monorepo_slice` | `fuw_packet:archetype.go_service_or_monorepo_slice` | account-free local, retest pending |
| `beta_archetype:c_or_cpp_native_project` | `scorecard_row:first_useful_work.archetype.c_or_cpp_native_project` | `fuw_packet:archetype.c_or_cpp_native_project` | account-free local, retest pending |

Retest pending means the row has a current packet and score, but the
claim must stay narrowed until the referenced archetype report and
workflow harness results move out of retest pending.

## Top Incumbent Import Rows

| Source row | Scorecard row | Packet | Import rollback | Restore provenance |
|---|---|---|---|---|
| `migration_source:vs_code_code_oss` | `scorecard_row:first_useful_work.import.vs_code_code_oss` | `fuw_packet:entry.import_vscode_diff_rollback` | `artifacts/migration/m3/import_diff_and_rollback_packet.md#import-vs-code--code-oss` | `artifacts/migration/m3/restore_provenance_packet.md#imported-profile-restore-provenance` |
| `migration_source:jetbrains_family` | `scorecard_row:first_useful_work.import.jetbrains_family` | `fuw_packet:import.jetbrains_profile` | `artifacts/migration/m3/import_diff_and_rollback_packet.md#jetbrains-ides` | `artifacts/migration/m3/restore_provenance_packet.md#imported-profile-restore-provenance` |
| `migration_source:vim_neovim` | `scorecard_row:first_useful_work.import.vim_neovim` | `fuw_packet:import.vim_neovim_profile` | `artifacts/migration/m3/import_diff_and_rollback_packet.md#vim--neovim` | `artifacts/migration/m3/restore_provenance_packet.md#imported-profile-restore-provenance` |
| `migration_source:emacs` | `scorecard_row:first_useful_work.import.emacs` | `fuw_packet:import.emacs_profile` | `artifacts/migration/m3/import_diff_and_rollback_packet.md#emacs` | `artifacts/migration/m3/restore_provenance_packet.md#imported-profile-restore-provenance` |

The incumbent import rows deliberately show the full
`Exact` / `Translated` / `Partial` / `Shimmed` / `Unsupported` spread.
Unsupported runtime rows remain retained diagnostics and must not be
quietly applied as live authority.

## Downgrade Path

The claim manifest consumes the scorecard through
`claim_evidence:switching.first_useful_work_scorecard`. When any
scorecard row becomes stale or regressed, downstream surfaces must
narrow the affected row before publishing:

- docs and migration notes use the current scorecard status;
- Help/About and support export cite the same packet and downgrade
  trigger;
- release packets preserve the import rollback and restore provenance
  refs for the affected row;
- no channel may blend account-free local rows with managed or
  provider-linked rows to keep an aggregate green.
