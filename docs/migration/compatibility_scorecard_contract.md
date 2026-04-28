# Imported Extension and Workflow Compatibility Scorecard Contract

This contract freezes one machine-readable scorecard format for imported
extensions, imported workflows, and workflow-bundle handoff rows. It exists
so migration claims can be current, challengeable, and reusable across docs,
the migration center, claim manifests, compatibility reports, support
exports, and workflow-bundle surfaces.

Companion artifacts:

- [`/schemas/migration/compatibility_scorecard.schema.json`](../../schemas/migration/compatibility_scorecard.schema.json)
  defines `compatibility_scorecard_row_record` and
  `compatibility_scorecard_packet_record`.
- [`/artifacts/migration/top_imported_workflow_rows.yaml`](../../artifacts/migration/top_imported_workflow_rows.yaml)
  is the seed scorecard packet for common imported extensions, workflow
  blockers, and bundle handoff rows.
- [`/fixtures/migration/compatibility_scorecards/`](../../fixtures/migration/compatibility_scorecards/)
  contains JSON fixtures for the blocker and native-alternative cases.
- [`/docs/migration/source_ecosystem_coverage_matrix.md`](./source_ecosystem_coverage_matrix.md)
  and [`/artifacts/migration/source_ecosystem_rows.yaml`](../../artifacts/migration/source_ecosystem_rows.yaml)
  remain the source-lane catalog the scorecards cite.
- [`/docs/migration/first_run_import_diff_and_rollback_contract.md`](./first_run_import_diff_and_rollback_contract.md)
  remains the preview/apply/rollback contract that emits importer rows
  linking to scorecard rows.
- [`/docs/ecosystem/extension_lockfile_and_recommendation_contract.md`](../ecosystem/extension_lockfile_and_recommendation_contract.md)
  consumes scorecard refs when imported extension recommendations resolve.

## Scope

Each scorecard row names one compatibility decision for a source object:

- an imported extension or plugin;
- an imported workflow, such as a run/debug configuration or modal editing
  profile; or
- a workflow-bundle handoff row that tells the migration center which
  governed native bundle or archetype path should be recommended.

The scorecard does not implement extension compatibility. It freezes the
machine contract that prevents a bridge, workaround, native replacement, or
blocked state from being rewritten differently by each publication surface.

Out of scope:

- executing foreign extension runtimes;
- broad extension compatibility guarantees;
- marketplace ranking implementation;
- final product copy for every scorecard row.

## Row Contract

Every scorecard row must carry:

| Field | Purpose |
|---|---|
| `scorecard_id` | Stable `migration_scorecard:*` id quoted by migration center, docs, support, and reports. |
| `row_subject_class` | `imported_extension`, `imported_workflow`, or `workflow_bundle`. |
| `source_ecosystem_id` | Governed source ecosystem id, or `native_aureline` / `mixed_imported_sources` for bundle handoff rows. |
| `source_object` | Source object ref, label, class, optional version, and source profile refs. |
| `target` | Target archetype, workflow, workflow bundle, native command/package, bridge, or `none`. |
| `native_path` | Whether a native path is available, partial, replaced, blocked, or not applicable. |
| `compatibility_bridge_path` | Whether a bridge path exists and what caveat or evidence state applies. |
| `blocked_state` | Explicit block class and whether the import/apply path is denied. |
| `status_semantics` | Declared status, effective status, public-claim posture, downgrade triggers, and scoring summary. |
| `caveat` | Required caveat that must travel with any public or support-facing row projection. |
| `owner_allocation` | Row, evidence, publication, and support owners. |
| `publication_links` | Docs/help refs, compatibility-row refs, claim-manifest refs, migration report refs, support export refs, and issue-template refs. |
| `evidence_refs` | Current evidence or seed evidence backing the row. |
| `freshness` | Captured time, stale window, next refresh target, source revision, and refresh triggers. |

Rows use opaque refs and short redaction-aware summaries. Raw source
profile bodies, extension storage, absolute paths, credentials, marketplace
payloads, and workspace contents do not belong in scorecards.

## Status Semantics

The scorecard status set is closed:

| Status | Meaning | Publication posture |
|---|---|---|
| `supported` | A native or bridge path is current, evidence-backed, documented, and linked to the relevant compatibility row. | Claim-bearing only while evidence and docs/help links are current. |
| `partial` | A subset works or translates, but one or more capabilities remain manual-review, unsupported, or caveated. | Limited wording; affected capabilities must stay visible. |
| `community_path` | A community-maintained, external, or self-service path exists without first-party support evidence. | Community-only wording; no stable support claim. |
| `blocked` | No safe native or bridge path exists, or the path would widen trust, permission, egress, runtime, or policy authority. | Blocked wording; import/apply must not proceed through this row. |
| `deprecated` | The source path still has a documented support window but is being retired. | Deprecated wording plus successor or removal window. |
| `replaced` | A native Aureline path should be recommended instead of preserving source compatibility. | Replacement-grade wording; do not imply source runtime parity. |

Required downgrade rules:

1. A `supported` row with stale or missing required evidence downgrades to
   `partial` when a narrow, caveated path remains; otherwise it downgrades
   to `blocked`.
2. A row missing docs/help linkage cannot be public claim-bearing even if
   technical evidence exists.
3. A stale bridge profile, changed permission vocabulary, or changed
   workflow-bundle revision forces `retest_pending` or a narrower status
   before docs, marketplace, migration center, or release copy may widen.
4. A row whose evidence is narrower than the target claim must project the
   narrower scope. Public surfaces may narrow further for space, but may
   not widen above `status_semantics.effective_status`.
5. Unsupported runtime states such as webview-heavy extension APIs,
   arbitrary Lua plugin execution, arbitrary Elisp execution, or hidden
   Node-native dependency execution remain `blocked` until a governed
   bridge or native replacement row exists.

## Publication Rules

Scorecards are the current row source for imported extension and workflow
claims. Publication surfaces must follow these rules:

1. Docs, migration center, claim manifests, compatibility reports, support
   exports, and release notes quote the same `scorecard_id`, caveat, and
   `effective_status`.
2. A surface may not describe a row as supported, compatible, equivalent,
   migrated, or native unless the scorecard has `public_claim_posture:
   claim_bearing` or `replacement_grade` for that exact target path.
3. A migration preview or importer outcome row that references an imported
   extension or workflow blocker should include the matching
   `scorecard_id` in its docs/help, export, or compatibility refs.
4. Claim-manifest rows and release compatibility rows may aggregate
   scorecards, but aggregation may not hide `partial`, `blocked`, stale,
   deprecated, or replaced sub-rows.
5. Any status change that widens or narrows public wording must update the
   scorecard, docs/help refs, claim manifest projection, and compatibility
   report in the same change set or carry a late-proof exception.

## Refresh Rules

Scorecards are current only inside their declared freshness window. A
refresh is required when any of these change:

- source extension/plugin version or source profile fixture;
- native package, native command, or target workflow bundle revision;
- compatibility bridge profile, host ABI, permission vocabulary, or policy
  admission rule;
- compatibility report, certified-archetype report, claim-manifest row, or
  public-truth parity projection;
- docs/help, known-limit, migration-note, or support-export link;
- evidence freshness window expires.

If the row owner cannot refresh before `next_refresh_due`, the row must be
published as stale, partial, community-only, blocked, deprecated, or
replaced rather than left as a current supported claim.

## Seed Cases

The seed packet covers:

- unsupported webview-heavy extension import;
- partial JetBrains run/debug configuration translation;
- blocked Vim/Neovim Lua plugin runtime;
- blocked Emacs Elisp package runtime;
- native alternative recommendation for an imported linting extension;
- community path for a Sublime/TextMate syntax bundle;
- workflow-bundle handoff for a TypeScript web-app switching flow.

These seed rows are intentionally conservative. They make blocker and
replacement states reusable before broad compatibility implementation
exists.
