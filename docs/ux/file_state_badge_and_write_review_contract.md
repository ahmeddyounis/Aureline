# File-State Badge and Write-Review Contract

This contract freezes the shared badge, reason-strip, and write-review
vocabulary Aureline uses when a file-like surface is read-only,
generated, policy locked, managed or mirrored, projected, or captured as
evidence. The goal is to keep editor, diff, preview, review, notebook,
and evidence surfaces from inventing incompatible authority language.

Companion artifacts:

- [`/schemas/ux/file_state_badge_group.schema.json`](../../schemas/ux/file_state_badge_group.schema.json)
  defines the machine-readable badge group and reason-strip record.
- [`/schemas/ux/write_review_sheet.schema.json`](../../schemas/ux/write_review_sheet.schema.json)
  defines the write-review sheet record used when a write-like action is
  blocked, redirected, or requires review.
- [`/fixtures/ux/file_state_surface_cases/`](../../fixtures/ux/file_state_surface_cases/)
  contains worked badge and sheet cases for canonical local source,
  generated artifacts, policy locks, managed mirrors, projected notebook
  result views, and captured evidence snapshots.

This contract composes with:

- [`/docs/ux/editor_document_state_contract.md`](./editor_document_state_contract.md)
  for editor-specific document-state placement and identity preservation.
- [`/docs/architecture/generated_artifact_safe_edit_policy.md`](../architecture/generated_artifact_safe_edit_policy.md)
  for generated, mirrored, imported, preview, and notebook/result edit
  posture.
- [`/docs/ux/view_freshness_contract.md`](./view_freshness_contract.md)
  for live, snapshot, partial, stale, approximate, and captured-scope
  disclosure.
- [`/docs/ux/dialog_sheet_contract.md`](./dialog_sheet_contract.md)
  for sheet selection, consequence blocks, explicit dismissal, focus
  return, and durable handoff rules.

If this document conflicts with a source product document in `.t2/docs/`,
that source wins and this contract, schemas, and fixtures must update in
the same change.

## Boundary

File-state badges and write-review sheets are surface contracts. They do
not own buffer bytes, file permissions, policy decisions, generator
lineage, live target state, notebook kernels, evidence packets, or save
algorithms. They project those owners into one stable vocabulary.

Frozen here:

- six reusable badge groups: `read_only`, `generated`,
  `policy_locked`, `managed_mirrored`, `projection`, and
  `captured_snapshot`;
- the reason-strip fields every constrained surface must show when a
  user tries to understand or change the object;
- the write-review sheet action vocabulary for duplicate, detach,
  regenerate, create editable overlay, open canonical source, request
  approval, and export evidence;
- the rule that dirty state, write authority, source-of-truth relation,
  and freshness stay separate facts; and
- fixture coverage proving canonical objects, projections, mirrors, and
  snapshots do not collapse into one generic read-only label.

Out of scope: final badge rendering, icon artwork, provider-specific
save workflows, all generated-file classes, and full implementation of
every review action.

## Badge Groups

Every protected surface that shows one of these states emits one
`file_state_badge_group_record`. The badge group can be compact in the
UI, but the record always carries source-of-truth relation, write
authority, dirty state, freshness, reason-strip text, and next safe
action as distinct fields.

| Badge group | Meaning | Source-of-truth relation | Write authority | Freshness | Default next safe action |
|---|---|---|---|---|---|
| `read_only` | The current object can be inspected, copied, or compared, but not directly written in this surface. | May be the canonical object or a related writable source. | `read_only_permission`, `approval_required`, or a more specific constrained authority. | Current, snapshot, stale, or unknown depending on owner. | Open canonical source, duplicate, request approval, or export evidence. |
| `generated` | The object is derived from source plus a generator, resolver, kernel, or toolchain. | Generator output from canonical source. | Direct write is blocked unless a separate override contract admits it. | Current only for the generation basis; stale when inputs or generator changed. | Open canonical source, compare, or regenerate. |
| `policy_locked` | A policy, entitlement, managed setting, or legal/admin hold narrows mutation. | Policy rule controls current object or action. | Direct write is blocked until policy allows, an approval is granted, or a scoped exception exists. | Policy epoch / bundle freshness is explicit. | Inspect policy, request approval, or export evidence. |
| `managed_mirrored` | The object is managed by an upstream, mirror, promotion, sync, or admin-controlled source. | Managed upstream or mirror origin remains canonical. | Refresh, promote, detach, or request admin approval; ad hoc local writes are not the official repair path. | Mirror basis and last refresh are explicit. | Open upstream, refresh/promotion review, detach, request approval, or export evidence. |
| `projection` | The visible object is a rendered, queried, live, cached, or materialized view. | Live target, query basis, kernel, or render source is canonical. | Direct writes to the view do not mutate the source; edits require an overlay or canonical source route. | Live, snapshot, partial, stale, or approximate according to view-freshness rules. | Requery/rerun, create editable overlay, open canonical source, or export evidence. |
| `captured_snapshot` | The object is an evidence, support, history, crash, review, or offline capture. | Captured basis is canonical only for the capture moment; current truth may differ. | Snapshot is evidence-only; mutation happens through source, duplicate, overlay, or separate restore review. | Captured timestamp and live-current distinction are explicit. | Open canonical source, compare, duplicate, or export evidence. |

`clean`, `writable`, and `current` remain default facts. They are not
badge groups. A canonical local source file with a read-only permission
problem is `read_only` with `source_of_truth_relation =
current_object_is_canonical`; a generated stale file is `generated`
with separate `freshness = stale`; a policy-locked dirty draft is
`policy_locked` plus an explicit dirty-state field.

## Required Fact Separation

Surfaces MUST NOT collapse these axes:

- **Dirty state** says whether this tab, sheet, overlay, or projected
  view owns unsaved or staged local changes.
- **Write authority** says whether the current surface may directly
  mutate the object, needs approval, can only create a copy or overlay,
  or must route to an upstream owner.
- **Source-of-truth relation** says whether the visible object is the
  canonical object, generated from source, controlled by policy,
  mirrored from upstream, projected from a live target, or captured from
  a past basis.
- **Freshness** says whether the facts are live, current, snapshot,
  stale, partial, unknown, or not applicable.

A surface may render a short visual badge, but support exports,
accessibility labels, screenshots, and review sheets must preserve all
four axes.

## Reason Strip

The reason strip is the compact plain-language row shown near the badge
group or at the top of a write-review sheet. It has four required
plain-language fields plus a canonical-source relation row:

| Field | Required content |
|---|---|
| `cause_plain_language` | Why the current object cannot be treated as ordinary writable source. |
| `scope_plain_language` | Which object, view, range, mirror, policy, source, or capture the state applies to. |
| `boundary_plain_language` | What boundary is being protected: filesystem permission, generator lineage, policy, managed upstream, live target, or evidence capture. |
| `safe_next_step_plain_language` | The next action that keeps identity and recovery honest. |
| `canonical_source_relation_row` | Source, generator, policy rule, managed upstream, live target, or evidence capture ref when known, plus freshness and open/action ref. |

The reason strip must be visible without relying on hover-only detail
whenever the state blocks save, edit, apply, regenerate, publish, share,
or evidence export.

## Write-Review Sheet

When a user tries to write through a constrained object, the surface
opens or references one `write_review_sheet_record`. The sheet explains
the current object, the blocked or redirected intent, safe actions, side
effects, checkpoint/rollback posture, close behavior, and focus return.

The action vocabulary is closed:

| Action class | Use for | Required side-effect disclosure | Required rollback/checkpoint posture |
|---|---|---|---|
| `duplicate` | Create a writable copy while preserving the original constrained object. | New local duplicate or support copy, destination class, and whether the source remains linked. | Reversible by deleting duplicate, or a pre-action checkpoint if the duplicate is inserted into workspace state. |
| `detach` | Break a managed, mirrored, generated, or overlay relationship before local editing. | Manager/link removal, future sync or regenerate behavior, and loss of managed updates. | Checkpoint or reattach/restore route before detachment. |
| `regenerate` | Replace a generated, projected, or derived artifact from canonical inputs. | Artifact replacement scope, generator/runtime ref, skipped members, and stale-basis handling. | Pre-action checkpoint or rollback by regeneration with pinned inputs. |
| `create_editable_overlay` | Let the user annotate or stage edits over a projection/snapshot without mutating the source. | Overlay creation scope, ownership, retention/export class, and merge/apply limits. | Overlay can be discarded, exported, or compared without changing the original. |
| `open_canonical_source` | Navigate to the object that actually owns writes. | No source mutation; focus/identity route only. | No checkpoint required. |
| `request_approval` | Ask policy, admin, managed owner, or provider authority to grant a write path. | Approval ticket request, target scope, expiry/revocation posture, and deny path. | No local source mutation until approval succeeds. |
| `export_evidence` | Preserve the constrained state for support, audit, review, or handoff. | Evidence packet/export created, redaction class, and captured-vs-live refs. | Evidence export does not mutate source; deletion/revocation follows export policy. |

Every action row must name a command ref, target relation, side effects,
checkpoint posture, result state, and confirmation requirement. Product
native action labels must name the verb and target, not generic
confirmation text.

## Surface Obligations

Editor, diff, preview, review, notebook, and evidence surfaces MUST
reuse the same record fields when they expose constrained file-like
objects:

- Editors and diff views preserve badge group, dirty-state, source
  relation, write authority, freshness, and canonical action refs.
- Preview and notebook surfaces must identify projections and outputs as
  views, not writable source.
- Review and evidence surfaces must preserve captured-vs-live
  distinctions and export posture.
- Support exports and screenshots keep badge class names, reason-strip
  fields, action classes, and authority refs without raw file bodies,
  secrets, provider payloads, raw policy bodies, or raw absolute paths.

## Acceptance Checklist

A reviewer can accept a surface or fixture when:

1. Read-only, generated, policy-locked, managed/mirrored, projection, and
   captured-snapshot states use the shared badge vocabulary.
2. Dirty state, write authority, source-of-truth relation, and freshness
   are separately inspectable.
3. Reason strips explain cause, scope, boundary, safe next step, and
   canonical-source relation in plain language.
4. Write-review sheets offer only explicit safe actions and disclose side
   effects plus checkpoint/rollback posture.
5. Fixtures make it clear whether Aureline is editing the canonical
   object, a generated artifact, a managed mirror, a projection, or a
   captured snapshot.
