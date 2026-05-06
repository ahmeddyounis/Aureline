# State migration and restore playbook

This document freezes the shared playbook every profile import or
restore, sync apply, session restore, layout restore, checkpoint
restore, support export, and repair escalation uses when it explains
why remembered state restored exactly, downgraded, or stopped for
review.

The playbook sits above the artifact-specific contracts. It does not
replace them; it gives profile, layout, and future migration tooling one
closed set of state-plane labels, fidelity labels, downgrade reasons,
failure states, and preserved-artifact rules so support, compare/export,
and restore surfaces do not mint parallel vocabularies.

Companion contracts:

- [`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
- [`/docs/workspace/entry_restore_object_model.md`](../workspace/entry_restore_object_model.md)

Coverage inventory and shared evidence:

- [`/docs/state/state_migration_coverage_matrix.md`](./state_migration_coverage_matrix.md)
  — published inventory of state-bearing artifact families and their
  migration / recovery obligations.
- [`/artifacts/state/migration_playbook_index.yaml`](../../artifacts/state/migration_playbook_index.yaml)
  — canonical per-family index that keeps coverage complete across the
  state-object inventory and adjacent packet families.
- [`/schemas/state/recovery_evidence_packet.schema.json`](../../schemas/state/recovery_evidence_packet.schema.json)
  — shared recovery-evidence packet shape restore / downgrade / migration
  surfaces can cite instead of inventing per-surface outputs.

The machine-readable schema lives at:

- [`/schemas/state/restore_provenance.schema.json`](../../schemas/state/restore_provenance.schema.json)

The portable-state package manifest that wraps cross-machine
import postures (`exact`, `compatible`, `downgraded`, `inspect_only`)
around the same fidelity vocabulary is frozen at:

- [`/docs/state/portable_state_package_contract.md`](./portable_state_package_contract.md)
- [`/schemas/state/portable_state_manifest.schema.json`](../../schemas/state/portable_state_manifest.schema.json)

The durable-state compatibility window, backup-before-migrate matrix,
and restore-after-downgrade packet that bind each artifact family's
backward window, forward-read expectations, backup rule, and rollback
or downgrade behavior are frozen at:

- [`/docs/state/durable_state_compatibility_contract.md`](./durable_state_compatibility_contract.md)
- [`/schemas/state/compatibility_window_row.schema.json`](../../schemas/state/compatibility_window_row.schema.json)
- [`/schemas/state/restore_after_downgrade_packet.schema.json`](../../schemas/state/restore_after_downgrade_packet.schema.json)

Worked fixtures live under:

- [`/fixtures/state/migration_cases/`](../../fixtures/state/migration_cases/)
- [`/fixtures/state/portable_state_packages/`](../../fixtures/state/portable_state_packages/)
- [`/fixtures/state/durable_state_cases/`](../../fixtures/state/durable_state_cases/)

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
or the specialized profile or layout contracts, those documents win and
this playbook plus the shared schema MUST be updated in the same change.
Where a restore, sync, migration, support-export, or repair surface
invents a different downgrade label, exclusion label, or preserved-
artifact story, this playbook wins and the surface is non-conforming.

## Scope

- Freeze the four state planes every restore-provenance record uses to
  separate portable settings, local context, workspace-shared manifests,
  and non-portable live authority.
- Freeze one shared fidelity-label set for state migration and restore:
  `exact`, `compatible`, `layout_only`, and `manual_review`.
- Freeze the downgrade and failure-state rules for schema translation,
  schema meaning changes, missing extensions, missing remote sessions,
  unsupported display topology, excluded secrets or live handles, and
  manual-repair escalation.
- Freeze the rule that prior artifacts stay preserved by reference for
  compare/export whenever schema meaning changed or the restore stopped
  short of a faithful apply.
- Freeze one shared restore-provenance record that carries source,
  producer build, schema version, redaction class, resulting fidelity,
  state-plane separation, typed downgrade reasons, typed failure states,
  and preserved-artifact refs.

## Out of scope

- The full migration executor, sync engine, session database, or GUI
  repair flow.
- The concrete byte encoding for profile archives, layout snapshots, or
  support bundles.
- Final product copy for badges or banners. This document freezes the
  machine vocabulary the copy resolves against.

## 1. State planes

Every migrated or restored component MUST resolve to exactly one state
plane before the product claims a fidelity label.

| State plane | Typical contents | Default portability | Restore rule |
|---|---|---|---|
| `portable_settings` | settings, keybindings, snippets, themes, extension inventory | portable | may restore across machines and channels, but translation and review remain explicit |
| `local_context` | layout topology, display hints, local session context, evidence snapshots | local-only | may restore on the same machine or degrade to layout/evidence without claiming portable truth |
| `workspace_shared_manifest` | workspace manifests, worksets, tasks, launch configs | workspace-shared | review and diff apply are required before overwriting divergent workspace truth |
| `non_portable_live_authority` | secrets, credential handles, approval tickets, PTY handles, debug sessions, live kernels, remote-session bindings | excluded from portable restore | never auto-restores as live authority; only reference, evidence-only, or explicit reconnect paths are allowed |

Rules:

1. A restore-provenance record MUST classify every user-visible
   component it explains into exactly one `state_plane`.
2. `portable_settings` may claim `exact` or `compatible` restore.
   `local_context` may claim `layout_only` or evidence-only posture
   without implying portable truth.
3. `workspace_shared_manifest` is durable workspace truth. A migration
   MUST stop at diff/review rather than silently overwrite a divergent
   manifest.
4. `non_portable_live_authority` never crosses a machine or support-
   export boundary as restored live authority. If a source artifact
   carried only references or evidence for that plane, the record MUST
   say so explicitly.

## 2. Shared fidelity labels

The machine enum is lower-case snake case. Display copy may render the
Title Case labels shown below, but the closed machine set is fixed.

| Display label | Machine enum | Meaning | Minimum requirements |
|---|---|---|---|
| `Exact` | `exact` | every requested component round-tripped without translation or review | no downgrade reasons; no rollback checkpoint; no equivalence map |
| `Compatible` | `compatible` | at least one requested component translated through a declared compatibility path without blocking review | `equivalence_map_ref` and `rollback_checkpoint_ref` present |
| `LayoutOnly` | `layout_only` | only local context restored meaningfully; portable or workspace-authored truth stayed authoritative elsewhere | no claim that portable settings or live authority were restored |
| `ManualReview` | `manual_review` | one or more requested components require explicit review or repair before apply | blocking failure state present plus `equivalence_map_ref`, `rollback_checkpoint_ref`, and compare/export affordances |

Rules:

1. The label set is closed. A surface that invents `partial`, `best
   effort`, or another parallel label is non-conforming.
2. `exact` is forbidden once any requested component needed
   translation, user acknowledgement, a rollback checkpoint, placeholder
   fallback, or manual repair.
3. `compatible` is allowed only when every requested blocking issue was
   resolved inside the declared compatibility path and the user can
   inspect what translated.
4. `layout_only` is allowed only when the product preserved local
   context honestly and did not claim that portable settings,
   workspace-shared manifests, or live authority were reapplied.
5. `manual_review` is required when the product needs a human to accept
   a manifest conflict, substitute a missing dependency, or perform a
   repair step outside the declared compatibility path.

## 3. Downgrade and failure-state playbook

The downgrade reason explains why the fidelity label narrowed. The
failure state explains what the user or support engineer can do next.
Not every failure state is a downgrade: excluded secrets or live handles
may be informational when the source never claimed to carry them.

| Trigger | Affected planes | Minimum label | Required handling | Forbidden shortcut |
|---|---|---|---|---|
| schema translation with stable meaning | `portable_settings`, `workspace_shared_manifest` | `compatible` | record the equivalence map and create a rollback checkpoint before apply | claiming `exact` |
| schema meaning changed | any plane | `compatible` or `manual_review` | preserve the prior artifact by ref for compare/export before mutating the translated result | in-place rewrite with no preserved prior artifact |
| missing extension dependency | usually `local_context` | `layout_only` or `manual_review` | keep the pane slot or binding visible, record the missing dependency, and offer install/open-without/export actions | silently dropping the missing surface |
| missing remote session or authority | `non_portable_live_authority`, sometimes `local_context` | `layout_only` or `manual_review` | degrade to placeholder or evidence-only state, preserve provenance, and offer reconnect or reauthenticate | rerunning commands or silently reusing stale authority |
| unsupported display topology | `local_context` | `layout_only` | preserve pane order and focus truth, then reflow to safe bounds with an explicit note | restoring off-screen or unreachable geometry |
| excluded secret material or live handles | `non_portable_live_authority` | informational by default | record the exclusion and redaction class so support can tell it was intentional | implying the live authority was restored |
| workspace manifest conflict or policy narrowing | `workspace_shared_manifest` | `manual_review` | stop at diff/review, preserve both sides for compare/export, and expose manual repair | silently overwriting workspace truth |
| manual repair outside the declared compatibility path | any plane | `manual_review` | emit a repair-focused compare/export packet and keep the rollback checkpoint reachable | masking the repair step behind a success badge |

## 4. Prior-artifact preservation rules

Preserving the prior artifact is mandatory whenever a restore would
otherwise hide what changed in meaning.

Rules:

1. When schema meaning changes, the product MUST preserve the pre-apply
   artifact by opaque ref before writing the translated result.
2. A preserved artifact MUST carry enough metadata for compare/export:
   source family, preservation reason, redaction class, and compare or
   export handles.
3. The preserved artifact MAY be redacted, but it MUST remain useful for
   support and repair. "Preserved" does not permit replacing the artifact
   with an unlabeled summary blob.
4. A downgrade from `compatible` to `manual_review` MUST keep both the
   translated candidate and the preserved prior artifact reachable.
5. A support export that omits raw bodies MUST still carry the preserved
   artifact refs and the reason the body was redacted or excluded.

## 5. Shared restore-provenance record

The shared record captures the minimum fields every migration or restore
surface needs before it renders a fidelity badge or a repair action.

| Field | Meaning |
|---|---|
| `source_class`, `source_artifact_ref`, `artifact_family` | what the product restored from |
| `producer_build`, `source_schema_version` | who produced the source artifact and which schema meaning it used |
| `redaction_class` | what redaction posture applied to the provenance payload itself |
| `fidelity_label`, `restore_level` | the shared claim (`exact`, `compatible`, `layout_only`, `manual_review`) and the surface decision at apply time |
| `state_segments` | one row per explained component, each bound to one state plane, source posture, restore posture, and optional intentional-exclusion reason |
| `downgrade_reasons` | typed reasons that narrowed the resulting fidelity |
| `failure_states` | typed next-step rows for user or support follow-up, including informational exclusions |
| `preserved_prior_artifacts` | compare/export refs for the prior artifact(s) retained before mutation |
| `equivalence_map_ref`, `rollback_checkpoint_ref`, `compare_ref`, `export_ref` | stable handles used by review, repair, and support flows |

Rules:

1. `source_class`, `producer_build`, `source_schema_version`,
   `redaction_class`, and `fidelity_label` are mandatory on every
   record.
2. `state_segments` MUST explain intentional exclusions when the source
   omitted `non_portable_live_authority` by design.
3. `compatible` and `manual_review` MUST carry
   `equivalence_map_ref` and `rollback_checkpoint_ref`.
4. `manual_review` MUST carry at least one blocking failure state and at
   least one preserved prior artifact.
5. `layout_only` MAY omit translation handles, but it MUST still say
   which local-context segments restored and which live-authority
   segments degraded to placeholder or evidence-only posture.

## 6. Seed cases

The seed fixtures cover the closed fidelity-label set and the required
failure states.

| Fixture | Result | Primary point |
|---|---|---|
| [`profile_restore_exact.json`](../../fixtures/state/migration_cases/profile_restore_exact.json) | `exact` | portable settings restored exactly while secrets and live handles stayed intentionally excluded |
| [`profile_restore_compatible_schema_shift.json`](../../fixtures/state/migration_cases/profile_restore_compatible_schema_shift.json) | `compatible` | schema translation preserved a prior artifact for compare/export because meaning changed |
| [`layout_restore_layout_only_missing_dependencies.json`](../../fixtures/state/migration_cases/layout_restore_layout_only_missing_dependencies.json) | `layout_only` | layout truth survived missing extension, missing remote session, unsupported display topology, and excluded live handles |
| [`support_bundle_manual_review_workspace_conflict.json`](../../fixtures/state/migration_cases/support_bundle_manual_review_workspace_conflict.json) | `manual_review` | support recovery stopped at manifest conflict and manual repair with preserved compare/export refs |
