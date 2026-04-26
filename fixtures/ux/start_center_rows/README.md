# Start Center, workspace-switcher, and open-flow disclosure fixtures

Seed corpus for the contract frozen in
[`/docs/ux/start_center_contract.md`](../../../docs/ux/start_center_contract.md)
and the schema at
[`/schemas/ux/start_center_surface.schema.json`](../../../schemas/ux/start_center_surface.schema.json).

Each file is a single JSON document validating against one of the
six record kinds in the schema
(`start_center_surface_record`,
`start_center_primary_action_record`,
`recent_work_row_disclosure_record`,
`restore_card_record`,
`disclosure_banner_record`,
`workspace_switcher_view_record`).

Every fixture:

- Resolves every axis to vocabulary re-exported from the
  entry-restore object model §1–§4, the entry-restore truth
  audit §6 (`startup_state` tokens), and the navigation and
  escalation contract §3 (`navigation_route_id`).
- Pins the `start_center_surface_family`, zone, posture,
  freshness / absence, and privacy-reduction values this
  contract owns.
- Carries no raw absolute paths, raw URLs, raw credential
  material, or raw secrets. Every id is an opaque ref; every
  timestamp is a monotonic placeholder.
- Names the contract sections it exercises under
  `__fixture__.contract_sections`.

## Cases

| Fixture | Record kind | Scenario axis | Contract anchor |
| --- | --- | --- | --- |
| [`start_center_first_run_no_account.json`](./start_center_first_run_no_account.json) | `start_center_surface_record` | First-run, five distinct primary actions, `optional_local_path_available` on every row, no account nag above primary work-resume. | §4, §11 (first_run), §12.1 |
| [`start_center_offline_managed.json`](./start_center_offline_managed.json) | `disclosure_banner_record` | Offline banner for a remote-preferred reopen; blocking disclosure in `disclosure_band` above `primary_work_resume` with typed resolution hooks. | §3.2, §3.9, §8, §11 (offline_startup), §12.2 |
| [`start_center_recent_work_missing_target.json`](./start_center_recent_work_missing_target.json) | `recent_work_row_disclosure_record` | Recent-work row whose local folder moved; `freshness_class = unknown_since`, `absence_class = unreachable`, typed row actions. | §3.4, §6, §12.3 |
| [`start_center_recent_work_privacy_reduced.json`](./start_center_recent_work_privacy_reduced.json) | `recent_work_row_disclosure_record` | Shared-device launch with `hide_paths` redaction; `privacy_redaction_applied` names the fields redacted; `deferred_review_pending` account posture. | §3.3, §3.4, §3.5, §6, §12.4 |
| [`start_center_restore_card_compatible.json`](./start_center_restore_card_compatible.json) | `restore_card_record` | `compatible_restore` card with separable `summary_counts` (3 exact, 2 dirty-buffer, 0 checkpoint-rollback, 0 evidence-only) and per-pane session-execution posture. | §3.8, §7, §12.5 |
| [`start_center_restore_card_evidence_only.json`](./start_center_restore_card_evidence_only.json) | `restore_card_record` | `evidence_only` card with 5 evidence-only items and 1 recoverable dirty buffer — counts never flatten. | §7.1, §7.2, §11 (restore_failed), §12.6 |
| [`workspace_switcher_palette_view.json`](./workspace_switcher_palette_view.json) | `workspace_switcher_view_record` | Mid-session palette-hosted switcher re-advertising all five first-launch verbs plus `add_root`; keyboard-only reachable. | §3.1, §9, §12.7 |
| [`start_center_unsupported_envelope.json`](./start_center_unsupported_envelope.json) | `start_center_surface_record` | Managed-fleet envelope narrows primary actions to Open folder + Clone repository with `hide_all_except_open_and_clone`; blocking policy disclosure; `disclosed_narrowing_reason = unsupported_envelope`. | §3.5, §11 (unsupported_startup), §12.8 |

## Schema references

- Start-center surface schema:
  [`/schemas/ux/start_center_surface.schema.json`](../../../schemas/ux/start_center_surface.schema.json).
- Entry / restore vocabulary (re-exported here by reference):
  [`/schemas/workspace/entry_and_restore_result.schema.json`](../../../schemas/workspace/entry_and_restore_result.schema.json).
- Navigation and escalation contract (source of `navigation_route_id`):
  [`/docs/ux/navigation_and_escalation_contract.md`](../../../docs/ux/navigation_and_escalation_contract.md).

## Companion corpora

- `startup_state` tokens and per-state placeholder truth rows:
  [`/fixtures/ux/entry_restore_states/`](../entry_restore_states/).
- Entry / restore record shapes the disclosure rows wrap:
  [`/fixtures/workspace/entry_restore_examples/`](../../workspace/entry_restore_examples/).
- Concrete recent-work row, restore-card, and switcher-row anatomy:
  [`/fixtures/ux/recent_work_rows/`](../recent_work_rows/).
