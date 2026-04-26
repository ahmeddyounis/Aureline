# Persistence Inspector, Portable-State Export, and Restore-Provenance Card Contract

This document freezes the UX contract for inspecting remembered state,
reviewing portable-state exports, and explaining restore provenance.
It composes with the state-map and layout contracts instead of
renaming their fields:

- [`/docs/state/profile_and_state_map.md`](../state/profile_and_state_map.md)
- [`/docs/state/migration_and_restore_playbook.md`](../state/migration_and_restore_playbook.md)
- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)
- [`/schemas/state/portable_state_package.schema.json`](../../schemas/state/portable_state_package.schema.json)
- [`/fixtures/state/restore_provenance_cards/`](../../fixtures/state/restore_provenance_cards/)

The purpose is to keep persistence inspectable. A remembered session,
portable export, or restore result must never collapse workspace
authority, window topology, local-only machine state, and placeholder
evidence into one opaque blob.

## 1. Scope

This contract freezes three presentation surfaces:

- `remembered_state_inspector` - a reviewable inventory of what
  Aureline remembers for a workspace, window, profile, or support
  context.
- `portable_state_export_sheet` - the preflight sheet that shows what
  will enter a portable-state package and what will stay local-only.
- `restore_provenance_card` - the card that explains what reopened
  live, what reopened as placeholder or context, and what was
  intentionally excluded.

Out of scope:

- sync, backup, restore, migration, and package-upload services;
- concrete storage bytes for the eventual persistence database;
- final product copy for every label; and
- privileged secret or credential handling beyond the exclusion rules
  already frozen in the state-map and workspace-memory contracts.

## 2. Shared Invariants

Every surface in this contract follows the same rules:

1. A row names its artifact class, state plane, schema version, last
   write time, redaction posture, portability posture, and restore
   fidelity before offering actions.
2. A row that references a pane uses the layout contract's stable pane
   id. Placeholder insertion must not mint a new pane id.
3. A portable export lists local-only and machine-local exclusions
   before confirmation. Hidden omission is non-conforming.
4. A clear action is scoped to the selected remembered-state artifact
   refs. It must not delete source files, workspace manifests, unrelated
   caches, credential-store entries, or workspace content by implication.
5. A restore-provenance card distinguishes live restore, placeholder
   restore, evidence-only restore, blocked review, and intentional
   exclusion in separate outcome rows.

## 3. Artifact Classes

The schema-backed artifact-class set covers the persistence objects
these surfaces may display.

| Artifact class | Primary source | Default posture | Surface obligation |
|---|---|---|---|
| `workspace_authority_checkpoint` | workspace authority checkpoint / recovery journal | local or workspace-owned | Show checkpoint source, last write, and whether clearing requires preview or rollback. |
| `window_topology_snapshot` | pane-tree snapshot | local-only by default | Show stable pane ids, restore phase, and placeholder posture. |
| `portable_profile_body` | portable profile artifact | portable | Show included state classes and excluded secret/live-authority classes. |
| `portable_state_package_manifest` | portable-state package | portable or support export | Show selected artifact classes, redaction, size estimate, checksum, signature, and exclusions. |
| `restore_provenance_record` | shared restore-provenance record | inspectable metadata | Show source, producer build, schema version, redaction class, resulting fidelity, compare/export handles. |
| `missing_surface_placeholder_card` | degraded layout placeholder | local context or evidence | Show missing dependency, original pane id, evidence retained, and safe recovery actions. |
| `dirty_buffer_recovery_journal` | user-owned recovery state | local-only | Show compare/discard/open-journal posture; never treat as disposable cache. |
| `local_history_checkpoint` | local history | local-only | Show clear preview requirements and retained compare handles. |
| `terminal_restore_metadata` | terminal/session metadata | local-only/evidence | Show transcript or cwd posture and no-rerun guarantee. |
| `display_affinity_hints` | machine/display cache | machine-local | Show best-effort label and safe-bounds remap note when used. |
| `workspace_manifest`, `workset_manifest`, `tasks_and_launch_configs` | workspace-shared manifests | workspace-shared | Route mutation through diff/review; never overwrite from portable state silently. |
| `execution_context_snapshot` | execution context metadata | local-only/evidence | Show target boundary, redaction class, and live-authority exclusion. |

Surfaces may hide a class only when the user did not request that
scope. A surface may not merge rows to avoid showing that one class is
portable while another is local-only.

## 4. Remembered-State Inspector

The inspector renders a sortable, filterable inventory. Each row
includes:

- `artifact_class`
- `artifact_ref`
- `state_plane`
- `state_class_refs[]`
- `stable_pane_id` when pane-bound
- `last_written_at`
- `schema_version`
- `restore_fidelity`
- `portability_class`
- `local_only_reason` when not portable
- `redaction_class`
- `size_estimate`
- `checksum_state`
- `signature_state`
- `source_refs`
- row actions

Required actions:

| Action | Availability rule | Required guardrail |
|---|---|---|
| `inspect` | available for every non-secret row | Redact according to `redaction_class`; secret and credential bodies remain metadata-only or denied. |
| `export` | available only when the row's portability and destination allow it | Opens the portable-state export sheet with the row selected and exclusions visible. |
| `compare` | available when a restore, package, or preserved-prior artifact ref exists | Opens a diff or structured compare against the referenced artifact without mutating state. |
| `clear` | available only when the row's authority class allows local clearing | Requires preview for user-owned recovery state and rollback for user-authored durable truth; denied for admin-owned, credential-store, live-authority, and workspace-content rows. |

Clear actions are metadata- or state-artifact cleanup, not project
cleanup. Clearing a `window_topology_snapshot` may remove a remembered
layout. It must not close files on disk, delete source artifacts,
remove workspace manifests, clear unrelated index caches, or erase
dirty-buffer journals unless those rows are separately selected and
previewed.

## 5. Portable-State Export Sheet

The export sheet is a preflight review for a
`portable_state_export_sheet_record`. It is opened from the inspector,
profile export, support export, or restore-compare flow.

Required sheet fields:

- package purpose and destination class;
- producer build and created-at time;
- selected artifact classes with per-row state plane and portability;
- redaction class and redaction summary;
- machine-local exclusions with explicit reasons;
- size estimate with precision class;
- checksum state and digest ref when available;
- signature state and signer ref when available;
- package preview ref;
- `confirm_export` and `cancel_export` actions.

Rules:

1. The sheet must show machine-local exclusions in the same viewport as
   the selected artifact list before `confirm_export` is enabled.
2. `non_portable_live_authority` rows may appear as opaque refs,
   metadata, evidence, or placeholder context only. They may not be
   exported as live handles.
3. A checksum marked unavailable must explain whether the package is
   still building, the exporter is policy-limited, or the destination
   does not support package integrity checks.
4. A signature state of `unsigned` is allowed for local plain exports,
   but the sheet must not imply verified provenance.
5. `cancel_export` leaves the underlying remembered state untouched.

## 6. Restore-Provenance Card

A restore-provenance card summarizes a schema-backed restore outcome.
Each card includes:

- source class and source artifact ref;
- created-at time;
- producer build;
- source schema version;
- redaction class;
- resulting fidelity;
- restore level;
- outcome rows for live, placeholder, evidence-only, blocked, and
  intentionally excluded segments;
- missing dependencies;
- schema-migration note;
- `open_details` and `compare` actions.

Outcome row classes:

| Outcome | Meaning |
|---|---|
| `reopened_live` | The surface or setting reopened with current authority and no hidden replay. |
| `reopened_as_placeholder` | The layout slot or context row stayed visible, but a missing dependency prevents live hydration. |
| `reopened_as_context` | Metadata, transcript, snapshot, or evidence reopened without live authority. |
| `blocked_pending_review` | The restore stopped before mutation and requires review. |
| `intentionally_excluded` | The source or policy intentionally omitted the segment, usually because it was secret, live authority, or machine-local. |

The card must answer the three user questions in one place:

- what is live now;
- what is only context or a placeholder; and
- what was intentionally excluded.

`open_details` opens the full provenance record. `compare` opens the
preserved prior artifact, package manifest, or restore diff when
available. When compare is unavailable, the disabled reason must be
typed, not free-form.

## 7. Cross-Surface Mapping

The contract ties four persistence surfaces together:

| Surface | Schema-backed record | Required linkage |
|---|---|---|
| Workspace-authority checkpoint | `workspace_authority_checkpoint` artifact row | Inspector row points at checkpoint ref; export sheet can include metadata/evidence only when allowed. |
| Window-topology snapshot | `window_topology_snapshot` artifact row | Uses stable pane ids from the pane-tree schema and records placeholder cards separately. |
| Portable-state package | `portable_state_package_record` | Package manifest lists selected artifact classes, exclusions, checksum/signature state, and restore-provenance refs. |
| Missing-surface placeholder card | `missing_surface_placeholder_card` artifact row | Restore-provenance card names missing dependency, preserved pane id, evidence retained, and safe actions. |

Reviewers should be able to start from any inspector row, export-sheet
row, package manifest row, or restore-provenance card and resolve the
same artifact ref, state plane, portability class, and fidelity label.

## 8. Fixture Coverage

The worked restore-provenance cards live under
[`/fixtures/state/restore_provenance_cards/`](../../fixtures/state/restore_provenance_cards/).
They validate against
[`/schemas/state/portable_state_package.schema.json`](../../schemas/state/portable_state_package.schema.json)
and cover:

- a layout-only restore with missing extension and remote dependencies;
- a compatible portable-profile restore with schema translation; and
- a manual-review restore where workspace truth blocks automatic apply.

Every new fixture in that directory must cite the contract section it
exercises and must avoid raw paths, raw logs, raw commands, raw source
content, raw credentials, and live authority handles.
