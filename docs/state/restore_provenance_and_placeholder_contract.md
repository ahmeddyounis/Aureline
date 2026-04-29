# Restore-provenance, compatibility-restore downgrade, and missing-dependency placeholder contract

This document freezes the cross-surface vocabulary every startup,
session-restore, support-export, migration-handoff, repair, and
docs/help flow uses when it answers three questions about a partial
restore:

- **what reopened live, what reopened only as placeholder or context,
  and what was intentionally skipped** — recorded as a single
  restore-provenance record;
- **which compatibility-restore downgrade class the restore landed on**
  — Exact, Compatible, Layout only, Recovered drafts, or Evidence
  only — and which compare/export/rollback handles back the claim;
- **which missing-dependency placeholder card stands in for each pane
  whose live dependency was unavailable** — naming the typed missing
  dependency, preserving the original pane role and stable pane id,
  and naming the closed recovery actions the user is offered.

The record is the **shared inspectable body** that every restore
explanation surface emits before partial-restore behavior multiplies
across startup, support, and docs/help. It is not a restore engine, a
sync engine, or a UI rendering plan; it is the contract those surfaces
MUST conform to so a reviewer, support engineer, or migration tool can
explain a partial restore mechanically instead of negotiating parallel
field names.

The machine-readable schema lives at:

- [`/schemas/state/restore_provenance_record.schema.json`](../../schemas/state/restore_provenance_record.schema.json)

Worked fixtures live under:

- [`/fixtures/state/restore_placeholder_cases/`](../../fixtures/state/restore_placeholder_cases/)

This contract composes with:

- [`/docs/state/migration_and_restore_playbook.md`](./migration_and_restore_playbook.md)
- [`/docs/state/restore_artifact_family_contract.md`](./restore_artifact_family_contract.md)
- [`/docs/state/portable_state_package_contract.md`](./portable_state_package_contract.md)
- [`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
- [`/docs/state/workspace_memory_contract.md`](./workspace_memory_contract.md)
- [`/docs/ux/persistence_inspector_contract.md`](../ux/persistence_inspector_contract.md)
- [`/docs/ux/recent_work_and_restore_card_contract.md`](../ux/recent_work_and_restore_card_contract.md)
- [`/docs/ux/crash_loop_and_restore_fidelity_contract.md`](../ux/crash_loop_and_restore_fidelity_contract.md)

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, or one of the upstream state contracts above, those
documents win and this contract plus the record schema MUST be updated
in the same change. Where a downstream restore, support-export,
diagnostics, or docs/help surface mints a parallel
fidelity, missing-dependency, or placeholder vocabulary, this contract
wins and the surface is non-conforming.

## Why freeze this now

Partial-restore drift starts when each surface invents its own way of
saying "some of it came back". Without one frozen record:

- a startup banner says `Partial restore` while a support tool quotes
  `Best-effort restore` and a docs page describes a `Recovered`
  outcome — and none of the three labels are in the schema;
- an extension goes missing and the restore silently drops the pane
  out of the layout, leaving a hole the user cannot recover from;
- a remote target is unreachable and the restore reuses a stale route
  grant rather than admitting the dependency is gone;
- a permission was revoked and the user is shown a generic error
  instead of a typed reauthenticate action;
- a stale service dependency is pinged automatically, masking the
  outage instead of preserving the layout role;
- a schema-meaning change is hidden behind a Compatible badge with no
  rollback or preserved-prior artifact;
- the restore-provenance record carries different field names on
  every surface and cannot be diffed mechanically.

The record forecloses these patterns by treating restore-provenance,
compatibility-restore downgrade, and missing-dependency placeholder
behavior as three distinct contracts inside one frozen body. Once the
boundary is named, every restore stays explicit, comparable, and
exportable.

## Scope

- Freeze one `state_restore_provenance_and_placeholder_record` shape
  carrying source, created-at, producer build, source schema version,
  redaction class, resulting fidelity, restore level, missing-
  dependency classes, missing-dependency placeholder cards, schema-
  migration notes, preserved-prior-artifact refs (with retained
  rollback notes), intentional exclusions, and compare/export hooks.
- Freeze the closed five-class compatibility-restore downgrade
  vocabulary — `exact`, `compatible`, `layout_only`,
  `recovered_drafts`, `evidence_only` — and the compare/export/
  rollback hooks each class MUST carry.
- Freeze the closed missing-dependency taxonomy covering absent
  extensions, absent remote targets, revoked permissions, stale
  service dependencies, missing workspace authority, and missing
  schema-equivalence maps.
- Freeze the missing-dependency placeholder card shape: typed
  missing-dependency class, preserved original pane id, preserved
  pane role and surface class, preserved title hint, evidence
  posture, and a closed set of recovery actions.
- Freeze the rule that intentional exclusions (live authority,
  secret material, raw payloads, machine-unique handles) are
  recorded in their own row set and never imply a missing-dependency
  placeholder card.

## Out of scope

- The restore engine, persistence runtime, sync apply pipeline, or
  UI rendering of the restore card. The vocabulary freeze lands here;
  production surfaces compose over it later.
- Final product copy. Display copy may render `Exact`, `Compatible`,
  `Layout only`, `Recovered drafts`, and `Evidence only`; the closed
  machine set is fixed.
- The recursive pane-tree body or the workspace-authority checkpoint
  body. This record references them by opaque id.

## 1. Record boundary

Every restore-provenance record under this contract MUST resolve every
field to exactly one of the four boundaries below. Flattening them
into one payload is non-conforming.

| Boundary | What it carries | Where it lives |
|---|---|---|
| **Provenance core** | source family/class/ref, created-at, producer build, source schema version, redaction class, resulting fidelity, restore level, top-level rollback/equivalence/compare/export refs | top-level fields of `state_restore_provenance_and_placeholder_record` |
| **Compatibility-restore downgrade** | `resulting_fidelity` and `restore_level` enums, plus the conditional rules tying each fidelity to its required handles and preserved-prior-artifact rows | `resulting_fidelity`, `restore_level`, top-level handles, `preserved_prior_artifacts[]` |
| **Missing-dependency placeholder cards** | per-pane rows preserving the original pane id, role, surface class, evidence posture, recovery actions | `missing_dependency_classes[]`, `missing_dependency_placeholder_cards[]` |
| **Intentional exclusions** | typed exclusion rows for state the source never claimed to carry (live authority, secrets, raw payloads, etc.) | `intentional_exclusions[]` |

Rules (frozen):

1. A single record MUST cover exactly one source artifact. A
   multi-artifact restore emits one record per source.
2. Missing-dependency placeholder cards and intentional exclusions are
   distinct row sets. A row that names a `missing_dependency_class`
   MUST NOT also name an `intentional_exclusion_class`, and vice
   versa.
3. The aggregate `missing_dependency_classes[]` set MUST be the set
   of classes covered by `missing_dependency_placeholder_cards[]`.
   Free-form prose in `notes` is non-conforming as a substitute.
4. Compare and export refs are top-level handles for the resulting
   restore. Per-pane compare/export is reached through the preserved
   prior artifact list, not by minting parallel handles inside a
   placeholder card.

## 2. Provenance core fields

The provenance core is the shared header every record carries.

Required fields (frozen):

- `restore_provenance_id` — opaque stable id for the record.
- `source` — `artifact_family`, `source_class`, and
  `source_artifact_ref`. Names what the restore consumed.
- `created_at` — producer-local monotonic timestamp for record
  creation.
- `producer_build` — producer name, version, channel, platform class,
  and pseudonymous instance handle. Never a raw hostname.
- `source_schema_version` — opaque schema-version string the producer
  used.
- `redaction_class` — closed redaction-class enum reused from the
  shared portability vocabulary.
- `resulting_fidelity` — exactly one value from §3.
- `restore_level` — re-export of the same five-value vocabulary.
- `compare_ref`, `export_ref` — top-level handles for the resulting
  restore. Required on every fidelity above `exact`.
- `rollback_checkpoint_ref`, `equivalence_map_ref` — present per the
  rules in §3 and §5.
- `emitted_at` — producer-local monotonic timestamp for emission.

Rules (frozen):

1. `producer_build` and `source_schema_version` are mandatory on
   every record so a downgrade can be replayed and a fixture can
   reproduce the same fidelity decision.
2. `redaction_class` describes the redaction posture of the record
   payload itself. Per-row redaction is described inside preserved-
   prior-artifact rows.
3. `notes` is reviewer prose only and never a place to hide a missing
   dependency, an intentional exclusion, or a fidelity claim.

## 3. Compatibility-restore downgrade vocabulary

The closed five-class machine set is fixed. Display copy may render
the title-case labels shown below.

| Display label | Machine enum | Meaning | Required handles | Rollback note |
|---|---|---|---|---|
| `Exact` | `exact` | every component round-tripped without translation, placeholder, or review | none beyond compare/export are required; rollback and equivalence-map MUST be null | preserved prior artifact MAY be empty; no rollback row required |
| `Compatible` | `compatible` | one or more components translated through a declared compatibility path without blocking review | rollback checkpoint, equivalence map, compare, and export are all required | preserved prior artifact rows SHOULD record `downgraded_for_compare` for the translated components |
| `Layout only` | `layout_only` | window-local topology and stable pane ids survived; live or workspace authority did not restore as live | compare and export required; rollback and equivalence-map MAY be null | placeholder cards carry the per-pane recovery actions |
| `Recovered drafts` | `recovered_drafts` | dirty-buffer or local-history bodies were rehydrated as drafts the user must compare or accept | rollback, compare, export are required; preserved prior artifacts MUST list the rehydrated drafts | preserved prior artifacts carry `rollback_retained` notes naming the prior draft state |
| `Evidence only` | `evidence_only` | no live restore was attempted; only transcripts, snapshots, refs, and provenance survive | rollback, compare, export are required; placeholder cards MUST cover every live surface that did not reopen | preserved prior artifacts MAY appear with `support_export` notes |

Rules (frozen):

1. The label set is closed. A surface that invents `partial`,
   `best_effort`, `recovered`, or another parallel label is
   non-conforming.
2. `exact` is forbidden once any component required translation,
   placeholder fallback, review, or rollback. The conditional
   schema rules enforce empty `missing_dependency_classes[]`,
   empty `missing_dependency_placeholder_cards[]`, and null
   rollback/equivalence-map refs in this case.
3. `compatible` and `evidence_only` MUST carry rollback,
   equivalence-map (compatible), compare, and export refs.
4. `recovered_drafts` MUST carry at least one preserved prior
   artifact row so the rollback note is reachable.
5. `layout_only` MAY omit translation handles, but it MUST still
   carry compare and export refs.
6. The `restore_level` enum mirrors the fidelity class one-to-one.
   A surface that emits a `restore_level` outside the closed five-
   value set is non-conforming.
7. The vocabulary is reusable verbatim by startup banners, support
   exporters, docs/help cross-links, and migration handoffs.
   Surfaces re-render the title-case label; the machine value never
   varies.

## 4. Missing-dependency placeholder cards

A missing-dependency placeholder card is the only row type that may
stand in for a pane whose live dependency was unavailable. Each card
preserves the original pane role instead of silently collapsing the
surface out of the window topology.

### 4.1 Missing-dependency taxonomy

The closed `missing_dependency_class` set is fixed.

| Class | When it applies | Typical recovery actions |
|---|---|---|
| `absent_extension` | An extension or feature pack required to hydrate the pane is not installed. | `locate_extension`, `install_extension`, `open_without`, `export_evidence`, `remove_pane` |
| `absent_remote_target` | The remote target (host, kernel, runtime, workspace endpoint) is unreachable. | `reconnect_remote`, `reauthenticate`, `export_evidence`, `remove_pane` |
| `revoked_permission` | A capability ticket, delegated approval, or scoped grant the surface depended on is no longer valid. | `reauthenticate`, `open_restricted`, `export_evidence` |
| `stale_service_dependency` | A managed service or external dependency is offline, deprecated, or returns a no-longer-valid endpoint. | `retry_hydrate`, `open_repair_instructions`, `export_evidence`, `remove_pane` |
| `missing_workspace_authority` | The workspace-authority checkpoint the pane referenced is unavailable on this machine. | `compare_with_preserved_artifact`, `escalate_to_manual_repair`, `export_evidence` |
| `missing_schema_equivalence_map` | A schema migration is required but the equivalence map is absent or refused. | `compare_with_preserved_artifact`, `open_repair_instructions`, `escalate_to_manual_repair` |

Rules (frozen):

1. Every aggregate `missing_dependency_classes[]` value MUST be the
   class of at least one card row. Free-form prose substitutions are
   non-conforming.
2. New classes are additive only and require updating the schema and
   this contract in the same change. Repurposing an existing class is
   breaking and requires a governance decision row.

### 4.2 Placeholder card fields

Required fields (frozen):

- `placeholder_card_id` — opaque stable id for the row.
- `missing_dependency_class` — exactly one class from §4.1.
- `missing_dependency_ref` — opaque handle for the missing dependency
  body, or null when the producer cannot name a stable handle.
- `preserved_pane_id` — the original stable pane id. Minting a new
  pane id to substitute for a missing pane is non-conforming.
- `preserved_pane_role` — the original pane role (`editor`,
  `terminal`, `notebook`, `preview`, etc.). The placeholder card
  never uses the role `placeholder` itself; that posture is recorded
  on the window-topology snapshot's pane inventory.
- `preserved_surface_class` — the original surface class.
- `evidence_retained` — boolean. True when a transcript, snapshot,
  or metadata-only summary remains available behind the placeholder.
- `recovery_actions[]` — at least one closed recovery action.

Optional fields:

- `preserved_title_hint` — best-effort title hint preserved from the
  original pane.
- `evidence_ref` — opaque handle for the retained evidence body.
- `last_known_provenance_label` — short redaction-aware label naming
  the last-known live provenance ("rust analyzer extension", "remote
  python kernel on staging notebook host").
- `note` — short reviewer note.

Rules (frozen):

1. The card preserves layout truth. The original pane id, role, and
   surface class are mandatory. A surface that substitutes a generic
   "missing surface" pane is non-conforming.
2. The closed `recovery_actions[]` set is shared with the window-
   topology snapshot's `placeholder_action` vocabulary. A free-form
   action label is non-conforming.
3. `evidence_retained = false` is allowed only when the source
   intentionally never carried evidence for the surface; it is not
   permitted as a way to hide an unrecorded missing artifact.
4. A missing-dependency placeholder card never doubles as a typed
   intentional exclusion. Live-authority exclusions are §6 rows.

## 5. Schema-migration notes

A schema-migration note is mandatory on every record so a downstream
reviewer can tell whether meaning travelled across versions.

The closed `note_class` set is:

- `no_migration_required` — source and target schema versions match.
- `schema_translation_applied` — translation succeeded with stable
  meaning. Equivalence map and rollback checkpoint are required.
- `schema_meaning_changed` — meaning shifted during translation.
  Equivalence map, rollback checkpoint, and preserved-prior-artifact
  ref are required, and at least one preserved prior artifact row
  MUST appear at the top level.
- `blocked_pending_review` — translation stopped at review.
- `producer_schema_downgrade_refused` — producer refused to emit a
  downgraded schema; the destination MUST treat the artifact as
  inspect-only.

Rules (frozen):

1. The migration note class is closed. Free-form schema-shift prose is
   non-conforming.
2. `schema_translation_applied` and `schema_meaning_changed` require
   the equivalence-map and rollback-checkpoint refs inside the note.
3. `schema_meaning_changed` additionally requires at least one
   preserved prior artifact row so compare/export remain reachable.
4. `blocked_pending_review` and `producer_schema_downgrade_refused`
   pair with one or more `missing_schema_equivalence_map` placeholder
   rows when the blocked surface had a pane.

## 6. Intentional exclusions

Intentional exclusions are state the source never claimed to carry.
They are distinct from missing dependencies and never imply a
placeholder card.

The closed `intentional_exclusion_class` set is:

- `non_portable_live_authority` (PTY handles, kernels, debug
  sessions, live remote bindings)
- `secret_or_credential_material`
- `delegated_approval_or_ticket`
- `machine_unique_handle`
- `raw_provider_payload`
- `raw_path_or_url`
- `raw_command_line`
- `raw_log_or_trace`
- `raw_source_content`
- `user_declined`
- `policy_excluded`

Rules (frozen):

1. A row in `intentional_exclusions[]` records what the source
   intentionally never claimed to carry. The matching pane (if any)
   does not get a missing-dependency placeholder card; instead, the
   window-topology snapshot's evidence-only or topology-adjustment
   rows describe the visible effect.
2. The closed set is shared with the workspace-memory contract's
   excluded-class vocabulary so support and docs do not invent
   parallel labels.
3. A surface that infers continuity ("the live session continued")
   from an absent intentional-exclusion row is non-conforming.

## 7. Preserved prior artifacts and rollback notes

Preserved prior artifacts are the compare/export/rollback handles
retained whenever the restore would otherwise hide what changed.

Required fields (frozen):

- `artifact_ref`, `artifact_family`, `preservation_reason`,
  `redaction_class`.
- `compare_ref`, `export_ref` — opaque handles or null.
- `rollback_note` — short retained rollback note describing why the
  prior artifact remains reachable.

The closed `preservation_reason` set:

- `schema_meaning_changed`
- `downgraded_for_compare`
- `support_export`
- `manual_repair_escalation`
- `rollback_retained`

Rules (frozen):

1. `recovered_drafts` and `schema_meaning_changed` MUST emit at
   least one preserved-prior-artifact row.
2. The `rollback_note` is reviewer prose for the support and docs
   surface; it does not replace the typed `preservation_reason`.
3. A preserved prior artifact MAY be redacted, but it MUST remain
   useful for compare and export. Replacing the artifact with an
   unlabeled summary blob is non-conforming.

## 8. Cross-surface mapping

The record is reusable verbatim by every surface that explains a
restore.

| Surface | Linkage |
|---|---|
| Startup banners and restore badges | Render the closed `resulting_fidelity` label and gate the safe-action set on `missing_dependency_placeholder_cards[]`. |
| Persistence inspector and restore-provenance card | Map record fields onto the inspector's outcome rows; placeholder cards line up one-to-one with `missing_surface_placeholder_card` artifact rows. |
| Support recovery exports | Quote the same record verbatim; preserved prior artifacts back the compare/export refs. |
| Migration handoff and managed sync | Reuse the closed downgrade vocabulary and missing-dependency taxonomy in handoff manifests. |
| Docs and help flows | Reuse the title-case labels and recovery-action set; never invent parallel copy. |

Reviewers should be able to start from any of those surfaces and
resolve the same `restore_provenance_id`, `resulting_fidelity`,
`missing_dependency_classes[]`, and per-pane placeholder rows.

## 9. Conformance checklist

A restore-provenance record conforms when it can answer:

- Which `source` did the restore consume, and which producer build
  and source schema version emitted it?
- Which `resulting_fidelity` and `restore_level` does the record
  claim, and do its rollback/equivalence-map/compare/export refs
  match the conditional rules in §3?
- Which `missing_dependency_classes[]` were detected, and does each
  class appear as the class of at least one
  `missing_dependency_placeholder_cards[]` row?
- For each placeholder card, is the original `preserved_pane_id`,
  `preserved_pane_role`, and `preserved_surface_class` recorded, and
  is the `recovery_actions[]` set drawn from the closed enum?
- Does the `schema_migration_note.note_class` match the equivalence-
  map, rollback, and preserved-prior-artifact obligations in §5?
- Are intentional exclusions captured separately from missing
  dependencies?
- Do preserved prior artifacts carry a typed `preservation_reason`
  and a retained `rollback_note`?

If any answer requires new vocabulary, this contract and its schema
are extended first.

## 10. Changing this vocabulary

- **Additive-minor** changes (new missing-dependency class, new
  recovery action, new intentional-exclusion class, new schema-
  migration note class, new preservation reason) land here and in
  the schema in the same change. The change MUST cite the motivating
  fixture under
  [`/fixtures/state/restore_placeholder_cases/`](../../fixtures/state/restore_placeholder_cases/).
- **Repurposing** an existing fidelity label, dependency class,
  recovery action, exclusion class, or migration-note class is
  breaking and requires a governance decision row.
- The conditional rules in §3 stay aligned with the schema's `if/
  then` blocks. A surface that loosens a required handle without
  updating both this contract and the schema is non-conforming.
