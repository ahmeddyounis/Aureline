# First-run import diff and rollback contract

This document freezes the first-run import contract Aureline uses when
it detects existing editor profiles, builds a dry-run import plan,
renders a preview diff, applies selected changes, and rolls back a
failed or unwanted apply. The contract exists so the first switching
experience is governed by the same preview/apply/revert, policy,
portability, and support vocabulary used elsewhere in the repository.

The contract is intentionally structural. It does not implement importer
adapters or UI. It defines the packets those adapters and surfaces MUST
emit before any importer is allowed to mutate durable profile,
workspace, extension, trust, permission, entitlement, or egress state.

Companion artifacts:

- [`/schemas/migration/import_plan.schema.json`](../../schemas/migration/import_plan.schema.json)
  — boundary schema for `first_run_import_plan_record`, the read-only
  detection and dry-run planning packet.
- [`/schemas/migration/import_diff_preview.schema.json`](../../schemas/migration/import_diff_preview.schema.json)
  — boundary schema for `first_run_import_diff_preview_record`, the
  reviewer-facing preview packet.
- [`/schemas/migration/import_rollback_checkpoint.schema.json`](../../schemas/migration/import_rollback_checkpoint.schema.json)
  — boundary schema for `first_run_import_rollback_checkpoint_record`,
  the import-specific rollback checkpoint projection.
- [`/fixtures/migration/import_preview_cases/`](../../fixtures/migration/import_preview_cases/)
  — worked cases for full preview, partial import, skipped paths,
  rollback after apply, and imported-profile history linkage.
- [`/docs/migration/migration_center_object_model.md`](./migration_center_object_model.md)
  and [`/schemas/migration/importer_outcome.schema.json`](../../schemas/migration/importer_outcome.schema.json)
  — durable migration-session and importer-outcome vocabulary reused by
  this contract.
- [`/docs/verification/migration_and_profile_packet.md`](../verification/migration_and_profile_packet.md)
  — shared import-diff row, fidelity-label, profile-portability,
  temporary-profile, sync-conflict, and rollback-checkpoint vocabulary.
- [`/docs/state/migration_and_restore_playbook.md`](../state/migration_and_restore_playbook.md)
  — state-plane, fidelity-label, downgrade, failure-state, and
  preserved-prior-artifact rules.
- [`/docs/migration/source_ecosystem_coverage_matrix.md`](./source_ecosystem_coverage_matrix.md)
  — governed source ecosystem coverage matrix.
- [`/docs/migration/compatibility_scorecard_contract.md`](./compatibility_scorecard_contract.md),
  [`/schemas/migration/compatibility_scorecard.schema.json`](../../schemas/migration/compatibility_scorecard.schema.json),
  and [`/artifacts/migration/top_imported_workflow_rows.yaml`](../../artifacts/migration/top_imported_workflow_rows.yaml)
  — reusable imported-extension, imported-workflow, and workflow-bundle
  scorecard rows for blockers, partial paths, community paths, and
  native alternatives surfaced by preview packets.
- [`/docs/architecture/preview_runtime_contract.md`](../architecture/preview_runtime_contract.md)
  — preview/apply discipline reused for planned mutation disclosure.

If this document disagrees with the PRD, technical architecture,
technical design, UI/UX spec, or the frozen migration-center and
restore-provenance contracts, those sources win and this document plus
its schemas MUST be updated in the same change.

## Scope

This contract freezes:

- read-only importer detection for existing tool installations and
  profiles;
- dry-run planning for selected import domains and source-labeled target
  rows;
- preview packet fields for counts, conflicts, unsupported keys,
  incompatible extensions or workflows, native alternatives, trust and
  permission implications, and rollback requirements;
- apply semantics that are idempotent, checkpointed, and never silently
  overwrite user-owned durable truth;
- rollback semantics that preserve the checkpoint, validation, and
  support/export linkage after clean apply, partial apply, failure, or
  restore; and
- imported-profile history linkage into onboarding, migration center,
  and support surfaces.

Out of scope:

- implementing source-specific importer adapters;
- implementing first-run UI, CLI, or migration-center rendering;
- executing foreign plugin runtimes or arbitrary source scripts;
- defining final product copy for every row; and
- broadening migration coverage beyond the governed source rows.

## Lifecycle

First-run import moves through the following reviewable states:

1. `detecting_sources` — importer probes enumerate known installation
   and profile locations in read-only mode.
2. `sources_detected` — all discovered, missing, blocked, and
   unsupported sources are represented as source-labeled rows.
3. `planning_dry_run` — selected domains are resolved into an
   `first_run_import_plan_record`; no durable writes are authorized.
4. `preview_ready` — a `first_run_import_diff_preview_record` renders
   the planned effects and blockers.
5. `checkpoint_ready` — a
   `first_run_import_rollback_checkpoint_record` exists for every
   durable state class the apply may touch.
6. `applying` — apply consumes the reviewed preview and checkpoint refs
   with an idempotency token.
7. `applied`, `partially_applied`, `blocked`, or `failed` — the session
   writes durable importer outcomes and support/export refs.
8. `rolled_back` — rollback restores through the checkpoint and emits
   post-restore validation refs.

Rules:

1. A first-run import MAY stop after `preview_ready`. That is a complete
   dry-run outcome, not a failed migration.
2. A session MUST NOT enter `applying` until a rollback checkpoint is
   available for every durable target scope in the preview.
3. The same reviewed preview packet, checkpoint ref, policy epoch, and
   idempotency token MUST be used by desktop, CLI, support replay, and
   automation entry points.
4. If any packet required for apply is stale, incomplete, or from a
   different policy epoch, apply is denied and a fresh dry run is
   required.

## Importer detection and profile discovery

Detection is a read-only inventory operation. It may inspect known
installation metadata, profile manifests, portable bundles, and
workspace files that the current process already has read authority for.
It MUST NOT activate extensions, execute startup hooks, run source
scripts, contact marketplaces, refresh credentials, alter trust state,
or create target-side profile/workspace files.

Every detected or expected source row carries:

- `source_kind` from the governed migration source set;
- redaction-safe `display_name` and optional `source_version`;
- `discovery_method`;
- `path_availability_class`, including unavailable or denied path
  states;
- profile refs and labels when a profile is discoverable;
- domain availability rows for settings, keybindings, snippets, tasks,
  launches, themes, extensions/providers, workspace metadata, and
  layout; and
- evidence refs sufficient for support without carrying raw absolute
  paths or source file bodies.

Rules:

1. Missing, permission-denied, unsupported-version, corrupted, and
   policy-blocked paths remain visible as rows. They may not disappear
   because no import can be produced.
2. A profile discovered from a source tool is not a target profile. The
   plan MUST keep `source_profile_ref` and target descriptors separate.
3. The importer MUST label the source of every target candidate. A
   settings merge that loses source object refs is non-conforming.
4. `generic_import` is allowed only as an explicit generic lane. It may
   not imply marketed coverage for an ungoverned source ecosystem.
5. Detectors that cannot establish truth emit `insufficient_evidence` or
   `unsupported` rows rather than silently omitting the source.

## Dry-run import plan

The `first_run_import_plan_record` is the immutable plan input to the
preview. It records what the importer would attempt if the user later
chooses apply, but it grants no write authority.

Minimum plan fields:

| Field | Meaning |
|---|---|
| `source_installations` | Detected, unavailable, denied, or unsupported source installation rows |
| `discovered_profiles` | Source profile rows with domain availability and unreadable-path disclosure |
| `target_descriptor` | Destination profile/workspace candidate and apply scope |
| `selected_domains` | Domains included by the user or policy |
| `plan_items` | Source-labeled proposed rows, including skipped and blocked rows |
| `policy_floor` | Policy, trust, egress, permission, and entitlement ceilings used during dry run |
| `write_authority_posture` | Always `none_dry_run_only` before apply |
| `dry_run_stability` | Whether the plan is current, partial, stale, or blocked |

Rules:

1. A plan item may predict `imported`, `mapped`, `skipped`,
   `manual_review`, `bridge_required`, or `unsupported`, but it may not
   claim success before apply.
2. Plan rows that touch workspace truth, extension selections, tasks,
   launch/debug, trust, egress, entitlements, or credentials MUST include
   a safety implication row even when the result is blocked.
3. The plan records source counts and unavailable counts separately.
   A source path that cannot be read is not counted as successfully
   imported or silently excluded.
4. Plan identity is part of idempotency. Re-applying the same plan
   against the same target and policy epoch MUST be a no-op or an
   identical validated result.

## Diff preview packet

The `first_run_import_diff_preview_record` is the contract a reviewer,
CLI caller, support engineer, or later migration-center surface reads to
know exactly what would change before apply.

Minimum preview fields:

| Field | Meaning |
|---|---|
| `import_plan_ref` | Dry-run plan the preview was generated from |
| `migration_session_ref` | Durable migration session ref |
| `source_summary` | Counted detected, unavailable, blocked, and selected source profiles |
| `item_counts` | Counts for all six importer outcomes plus conflicts and blocked rows |
| `rows` | Source-labeled preview rows with target refs, outcome state, reason, mapping basis, and fidelity projection |
| `conflicts` | Target collisions, reserved shortcuts, divergent workspace truth, or policy-locked values |
| `unsupported_keys` | Source keys or concepts with no Aureline-native target |
| `incompatible_items` | Extensions, workflows, or source features blocked by compatibility or policy, with scorecard refs when one exists |
| `native_alternatives` | Suggested Aureline-native command, setting, extension, workflow, or docs refs |
| `trust_permission_implications` | Trust, extension permission, AI/network egress, entitlement, credential, subprocess, or filesystem-write deltas |
| `rollback_requirements` | Checkpoint scope, checkpoint ref/posture, retention, and restore-record linkage |
| `apply_gate` | Whether apply is allowed, denied, or requires review |
| `support_and_history_links` | Onboarding, docs/help, export, issue, and support refs |

Rules:

1. `item_counts` always carries all six importer outcome counters:
   `imported`, `mapped`, `skipped`, `manual_review`,
   `bridge_required`, and `unsupported`.
2. Conflicts, unsupported keys, incompatible items, and native
   alternatives are first-class arrays. They may be empty but may not be
   folded into prose.
3. A row with `bridge_required` or `unsupported` MUST carry docs/help and
   support/export refs through the preview packet. When the row matches an
   imported-extension, imported-workflow, or workflow-bundle scorecard, it
   MUST also carry the scorecard ref so later docs and support surfaces do
   not reinterpret the blocker.
4. A row that would widen trust, extension permissions, managed
   entitlement, AI egress, network egress, credential access, subprocess
   authority, or destructive automation defaults MUST be blocked from
   import. The preview may suggest a separate native review path, but
   first-run import itself may not broaden that authority.
5. The preview MUST identify the rollback checkpoint requirement even
   when apply is currently denied, so reviewers know what would be
   needed before the import can proceed.

## Apply semantics

Apply is the only state transition that may mutate durable target state.
It consumes:

- the `first_run_import_plan_record`;
- the `first_run_import_diff_preview_record`;
- a checkpoint record;
- the migration session ref;
- the current policy/trust epoch;
- the selected target descriptor; and
- an idempotency token.

Rules:

1. Apply MUST fail closed when the preview is stale, the target changed
   materially, a required checkpoint is missing, or policy/trust epochs
   no longer match.
2. Apply is field-aware. It may write only the source-labeled rows the
   user selected and the preview admitted.
3. Apply MUST NOT silently overwrite user-owned durable truth. Existing
   target values with a different owner, later revision, policy lock, or
   workspace authority either remain unchanged or move to
   `manual_review` with preserved-prior-artifact refs.
4. Apply MUST NOT widen workspace trust, extension permissions, AI
   egress, network egress, managed entitlements, credential access,
   subprocess authority, or destructive automation defaults.
5. Extension and workflow recommendations are recommendations until they
   pass their own install, permission, policy, and compatibility review.
6. A repeated apply with the same idempotency token and same effective
   target state MUST return the same outcome refs without duplicating
   rows or checkpoints.
7. Partial apply is allowed only when the packet keeps blocked and
   unapplied rows visible, preserves prior artifacts for mutated durable
   truth, and retains the rollback checkpoint.

## Rollback semantics

Rollback restores the target scopes named by the checkpoint. It is not a
general reset and it MUST NOT remove independent user changes that
landed after the import unless the user explicitly selects a conflict
resolution path that names those changes.

Minimum checkpoint fields:

| Field | Meaning |
|---|---|
| `checkpoint_ref` | Stable checkpoint id |
| `import_plan_ref` and `import_diff_preview_ref` | Reviewed plan and preview the checkpoint protects |
| `migration_session_ref` | Session that created or consumed the checkpoint |
| `restore_record_ref` | Companion `migration_restore_record` |
| `checkpoint_scope` | Profile, workspace, extension, setting, or composite scope |
| `protected_state_refs` | Snapshot or prior-artifact refs captured before apply |
| `availability_state` and `cleanup_state` | Whether rollback is still available and retained |
| `rollback_checkpoint_outcome_class` | Projection onto the frozen rollback outcome vocabulary |
| `post_restore_validation_refs` | Validators proving rollback did what it claims |

Rules:

1. Checkpoint creation happens before apply and records exactly which
   state classes are protected.
2. Rollback MUST preserve migration session, compatibility, preview,
   plan, docs/help, support, and export linkage.
3. Rollback is idempotent. A second rollback against an already-restored
   checkpoint returns the existing restored state and validation refs.
4. If the checkpoint is expired, policy-hidden, or cleanup-blocked, the
   checkpoint row remains visible with the typed outcome class and next
   action. It may not vanish from support/export surfaces.
5. Post-restore validation is required before a rollback surface claims
   `checkpoint_restored_to_prior_state`.

## Imported-profile history linkage

First-run import history is product state, not wizard-local memory. Any
applied, partially applied, skipped, blocked, or rolled-back import keeps
an imported-profile history ref that can appear in onboarding, profile
library, migration center, docs/help, support export, and issue-template
flows.

Required links:

- source descriptor and source profile refs;
- target profile/workspace refs;
- import plan, preview, outcome packet, restore/checkpoint, and
  validation refs;
- docs/help refs explaining unsupported, bridge-required, and native
  alternative rows;
- support/export refs when the import was partial, blocked, rolled back,
  or required manual review; and
- onboarding portability refs so tour progress and imported-profile
  history live in portable user/profile state rather than workspace
  source trees or hidden local-only stores.

Rules:

1. A rolled-back import remains in history as `rolled_back`; it is not
   deleted as though it never happened.
2. A skipped or preview-only import remains exportable when it explains
   why a source profile was not imported.
3. History rows preserve source labels. A later support packet must be
   able to tell which source tool/profile produced each imported,
   blocked, or skipped row.

## Fixture coverage

The fixture corpus under
[`/fixtures/migration/import_preview_cases/`](../../fixtures/migration/import_preview_cases/)
covers:

| Fixture | Primary behavior |
|---|---|
| `full_preview_vscode_common.yaml` | Full dry-run preview with counts, conflicts, unsupported keys, native alternatives, trust/permission implications, and checkpoint requirements before apply |
| `partial_import_missing_extension.yaml` | Partial import where settings and keybindings can apply, an extension is bridge-required, and a workflow remains manual-review |
| `skip_path_user_declined.yaml` | User-selected skip path that keeps unavailable paths and declined domains visible without producing writes |
| `rollback_after_apply.yaml` | Apply followed by rollback to the pre-apply checkpoint with post-restore validation refs |
| `imported_profile_history_linkage.yaml` | Imported-profile history row linking onboarding, support, docs/help, preview, outcome, and rollback refs |
